//! The byte-pair merge, with the caches that make it cheap: a pair table that
//! remembers whether two adjacent tokens merge, a piece cache for pieces that
//! have been merged before, and borrowed scratch arrays so a piece allocates
//! nothing. Pieces longer than HEAP_FROM bytes merge through a heap.

use crate::vocab::{long_hash, pack8, Vocab, NO_TOKEN};

pub const MERGE_MAX: usize = 256;
const HEAP_FROM: usize = 40;

const NO_RANK: i32 = i32::MAX;

const CACHE_WAYS: usize = 4; // sets of four, so a few thousand distinct pieces do not evict each other
const CACHE_SETS: usize = (1 << 13) / CACHE_WAYS;
const SMALL_CACHE_SETS: usize = 64;
const CACHE_PIECE: usize = 48;
const CACHE_TOK: usize = 24;
const PAIR_SIZE: usize = 1 << 15; // byte pairs never reach it: Vocab::byte_pair answers them
const SMALL_PAIR_SIZE: usize = 1 << 12;

#[derive(Clone, Copy)]
struct CacheEntry {
    hash: u64,
    k0: u64, // the first sixteen bytes, packed, so a short piece compares as two integers
    k1: u64,
    n: u8,
    ntok: u8,
    tail: [u8; CACHE_PIECE - 16], // bytes 16.. of a longer piece
    tok: [u32; CACHE_TOK],
}

#[derive(Clone, Copy)]
struct PairEntry {
    a: i32,
    b: i32,
    r: i32,
}

/// A piece too long for the fixed-size piece cache, with its tokens.
struct LongPiece {
    hash: u64,
    bytes: Box<[u8]>,
    toks: Box<[u32]>,
}

const LONG_PIECE_BITS: u32 = 9;
const SMALL_LONG_PIECE_BITS: u32 = 5;
const LONG_PIECE_MAX: usize = 512;

pub struct Cache {
    /// direct-mapped memo of merged pieces longer than the piece cache holds,
    /// up to LONG_PIECE_MAX bytes: separator lines, indentation, long words
    long_pieces: Vec<Option<LongPiece>>,
    long_shift: u32,
    /// verdicts of the CJK boundary test by character pair: (cp1 << 16 | cp2)
    /// in the low 32 bits, bit 32 set when blocked, bit 33 set when valid
    pub cjk_pairs: Vec<u64>,
    /// 64 minus the log2 of cjk_pairs' length, once it is set up
    pub cjk_shift: u32,
    /// the size step these tables were made at
    level: u8,
    e: Vec<CacheEntry>,
    victim: Vec<u8>, // per set, the way the next insert replaces
    pairs: Vec<PairEntry>,
    starts: Vec<i32>,
    ranks: Vec<i32>,
    toks: Vec<i32>,
    next: Vec<i32>,
    prev: Vec<i32>,
    heap: Vec<u64>,
    bt_bits: Vec<u64>,
}

impl Cache {
    /// Full-size caches.
    pub fn new() -> Cache {
        Self::sized(CACHE_SETS, PAIR_SIZE, LONG_PIECE_BITS, MERGE_MAX + 1)
    }

    /// Caches small enough to set up in a few microseconds, for a caller that
    /// may encode only a short text; `grow` replaces them with full ones.
    pub fn small() -> Cache {
        Self::at_level(0)
    }

    /// Caches of the given size step: 0 none, 1 tiny, 2 small, 3 and up full.
    /// Step 0 allocates nothing and merges every piece afresh.
    pub fn at_level(level: u8) -> Cache {
        let mut c = match level {
            0 => Cache {
                long_pieces: Vec::new(),
                long_shift: 64,
                cjk_pairs: Vec::new(),
                cjk_shift: 64,
                level: 0,
                e: Vec::new(),
                victim: Vec::new(),
                pairs: Vec::new(),
                starts: Vec::new(),
                ranks: Vec::new(),
                toks: Vec::new(),
                next: Vec::new(),
                prev: Vec::new(),
                heap: Vec::new(),
                bt_bits: Vec::new(),
            },
            1 => Self::sized(SMALL_CACHE_SETS / 4, SMALL_PAIR_SIZE / 4, SMALL_LONG_PIECE_BITS - 1, MERGE_MAX + 1),
            2 => Self::sized(SMALL_CACHE_SETS, SMALL_PAIR_SIZE, SMALL_LONG_PIECE_BITS, MERGE_MAX + 1),
            _ => Self::new(),
        };
        c.level = level;
        c
    }

    /// Swaps the hashed tables for those of a larger size step, keeping the
    /// character tables.
    pub fn grow_to(&mut self, level: u8) {
        let mut c = Self::at_level(level);
        let had_cjk = !self.cjk_pairs.is_empty();
        std::mem::swap(&mut c.bt_bits, &mut self.bt_bits);
        *self = c;
        if had_cjk {
            self.alloc_chars();
        }
    }

    /// Sets up the tables of three-byte characters on first use, so a text
    /// without such characters never pays for them.
    #[inline(always)]
    pub fn ensure_chars(&mut self) {
        if self.cjk_pairs.is_empty() {
            self.alloc_chars();
        }
    }

    #[cold]
    fn alloc_chars(&mut self) {
        let bits = [8, 10, 14, 16][self.level.min(3) as usize];
        self.cjk_pairs = zeroed(1 << bits);
        self.cjk_shift = 64 - bits;
    }

    /// The merge scratch arrays hold at least n + 1 slots afterwards.
    #[cold]
    fn grow_scratch(&mut self, n: usize) {
        let len = (n + 1).next_power_of_two().clamp(16, MERGE_MAX + 1);
        for a in [&mut self.starts, &mut self.ranks, &mut self.toks, &mut self.next, &mut self.prev] {
            *a = vec![0; len];
        }
    }

    fn sized(sets: usize, pairs: usize, long_bits: u32, scratch: usize) -> Cache {
        Cache {
            long_pieces: (0..1usize << long_bits).map(|_| None).collect(),
            long_shift: 64 - long_bits,
            cjk_pairs: Vec::new(),
            cjk_shift: 64,
            level: 3,
            e: zeroed(sets * CACHE_WAYS),
            victim: zeroed(sets),
            pairs: zeroed(pairs),
            starts: vec![0; scratch],
            ranks: vec![0; scratch],
            toks: vec![0; scratch],
            next: vec![0; scratch],
            prev: vec![0; scratch],
            heap: Vec::with_capacity(3 * scratch),
            bt_bits: Vec::new(),
        }
    }

    /// The byte-pair merge of a part of a piece, cached like `merge`. Same as
    /// `merge` today, since `merge` never consults the vocabulary for the
    /// whole input; kept as its own name so the two meanings stay apart.
    #[inline]
    pub fn merge_part(&mut self, v: &Vocab, out: &mut Vec<u32>, piece: &[u8]) {
        self.merge(v, out, piece)
    }

    /// Appends the tokens of piece, from the cache when it has them.
    #[inline]
    pub fn merge(&mut self, v: &Vocab, out: &mut Vec<u32>, piece: &[u8]) {
        self.merge_cached(v, out, piece, true)
    }

    /// `merge`, where a miss goes through the cuts when `cut` is set and
    /// straight to the merge otherwise.
    #[inline]
    fn merge_cached(&mut self, v: &Vocab, out: &mut Vec<u32>, piece: &[u8], cut: bool) {
        if self.victim.is_empty() {
            if cut { self.merge_into(v, out, piece) } else { self.merge_core(v, out, piece) }
            return;
        }
        if piece.len() > CACHE_PIECE {
            if piece.len() > LONG_PIECE_MAX {
                self.merge_into(v, out, piece);
                return;
            }
            let h = (long_hash(piece) as u64) << 32 | piece.len() as u64;
            let slot = (h.wrapping_mul(0x9E3779B97F4A7C15) >> self.long_shift) as usize;
            if let Some(e) = &self.long_pieces[slot] {
                if e.hash == h && &*e.bytes == piece {
                    out.extend_from_slice(&e.toks);
                    return;
                }
            }
            let before = out.len();
            if cut { self.merge_into(v, out, piece) } else { self.merge_core(v, out, piece) }
            self.long_pieces[slot] = Some(LongPiece { hash: h, bytes: piece.into(), toks: out[before..].into() });
            return;
        }
        let (k0, k1) = if piece.len() <= 8 {
            (pack8(piece), 0)
        } else if piece.len() <= 16 {
            (pack8(&piece[..8]), pack8(&piece[8..]))
        } else {
            (pack8(&piece[..8]), pack8(&piece[8..16]))
        };
        let mut h = (k0 ^ k1.wrapping_mul(0x9E3779B97F4A7C15) ^ ((piece.len() as u64) << 56)).wrapping_mul(0xBF58476D1CE4E5B9);
        if piece.len() > 16 {
            h ^= long_hash(&piece[16..]) as u64;
        }
        h ^= h >> 32;
        h = h.wrapping_mul(0x94D049BB133111EB);
        h ^= h >> 29;
        let set = (h & (self.victim.len() as u64 - 1)) as usize;
        let base = set * CACHE_WAYS;
        for w in 0..CACHE_WAYS {
            let e = &self.e[base + w];
            if e.hash == h && e.k0 == k0 && e.k1 == k1 && e.n as usize == piece.len() && (piece.len() <= 16 || e.tail[..piece.len() - 16] == piece[16..]) {
                out.extend_from_slice(&e.tok[..e.ntok as usize]);
                return;
            }
        }
        let before = out.len();
        if cut { self.merge_into(v, out, piece) } else { self.merge_core(v, out, piece) }
        let ntok = out.len() - before;
        if ntok <= CACHE_TOK {
            let w = self.victim[set] as usize;
            self.victim[set] = ((w + 1) % CACHE_WAYS) as u8;
            let e = &mut self.e[base + w];
            e.hash = h;
            e.k0 = k0;
            e.k1 = k1;
            e.n = piece.len() as u8;
            e.ntok = ntok as u8;
            if piece.len() > 16 {
                e.tail[..piece.len() - 16].copy_from_slice(&piece[16..]);
            }
            e.tok[..ntok].copy_from_slice(&out[before..]);
        }
    }

    #[inline]
    fn pair_rank(pairs: &mut [PairEntry], v: &Vocab, a: i32, b: i32, bytes: &[u8]) -> i32 {
        if pairs.is_empty() {
            let t = v.lookup(bytes);
            return if t == NO_TOKEN { NO_RANK } else { t };
        }
        let h = (((a as u32 as u64) << 32) | b as u32 as u64).wrapping_mul(0x9E3779B97F4A7C15);
        let mask = pairs.len() - 1;
        let e = &mut pairs[((h >> 32) as usize) & mask];
        if e.a == a && e.b == b && e.r != 0 {
            return e.r;
        }
        let t = v.lookup(bytes);
        let r = if t == NO_TOKEN { NO_RANK } else { t };
        *e = PairEntry { a, b, r };
        r
    }

    /// Appends the byte-pair merge of piece. A piece with characters beyond
    /// ASCII is first cut where no token can span the boundary (see
    /// `Vocab::can_cut`); its parts then merge, and are cached, on their own.
    pub fn merge_into(&mut self, v: &Vocab, out: &mut Vec<u32>, piece: &[u8]) {
        if piece.len() > 4 && piece.iter().any(|&b| b >= 0x80) {
            let mut start = 0;
            for p in 1..piece.len() {
                if piece[p] & 0xC0 != 0x80 && v.can_cut(piece[p - 1], piece[p]) {
                    self.merge_part_cut(v, out, &piece[start..p]);
                    start = p;
                }
            }
            if start > 0 {
                self.merge_part_cut(v, out, &piece[start..]);
                return;
            }
        }
        self.merge_core(v, out, piece)
    }

    /// A part between two cuts: a lone emoji from its table, anything else
    /// through the cache.
    #[inline]
    fn merge_part_cut(&mut self, v: &Vocab, out: &mut Vec<u32>, part: &[u8]) {
        if part.len() == 4 && part[0] == 0xF0 && part[1] == 0x9F && !v.emoji.is_empty() {
            let i = ((part[2] as usize & 0x3F) << 6) | (part[3] as usize & 0x3F);
            let e = v.emoji[i];
            if e != 0 {
                for k in 0..(e & 3) as usize {
                    out.push(((e >> (3 + 18 * k)) & ((1 << 18) - 1)) as u32);
                }
                return;
            }
        }
        self.merge_cached(v, out, part, false)
    }

    /// The byte-pair merge itself, without cuts or caching.
    fn merge_core(&mut self, v: &Vocab, out: &mut Vec<u32>, piece: &[u8]) {
        let n = piece.len();
        // ASCII pieces are short and merge faster the plain way, and so do
        // pieces with four-byte characters, whose tokens are mostly fragments
        // of those characters; the backtracking tables are only touched (and
        // for a shipped vocabulary built) for the rest
        let (mut wide, mut four) = (false, false);
        for &b in piece {
            wide |= b >= 0x80;
            four |= b >= 0xF0;
        }
        if wide && !four {
            if let Some(bt) = v.bt_for(n) {
                bt.encode(v, piece, out, &mut self.bt_bits);
                return;
            }
        }
        if n > MERGE_MAX {
            // rare: fall back to a fresh scratch set of the right size
            let mut big = Cache {
                long_pieces: Vec::new(),
                long_shift: 64,
                cjk_pairs: Vec::new(),
                cjk_shift: 64,
                level: 3,
                e: Vec::new(),
                victim: Vec::new(),
                pairs: std::mem::take(&mut self.pairs),
                starts: vec![0; n + 1],
                ranks: vec![0; n + 1],
                toks: vec![0; n + 1],
                next: vec![0; n + 1],
                prev: vec![0; n + 1],
                heap: Vec::with_capacity(3 * n),
                bt_bits: Vec::new(),
            };
            big.merge_heap(v, out, piece);
            self.pairs = std::mem::take(&mut big.pairs);
            return;
            // note: `victim` and `e` of `big` are empty and never read
        }
        if self.ranks.len() <= n {
            self.grow_scratch(n);
        }
        if n > HEAP_FROM {
            self.merge_heap(v, out, piece);
            return;
        }
        // part i covers piece[i..next[i]]; ranks[i] is the rank of joining it
        // with the part after it, NO_RANK for the last part and for slots
        // merged away. The lowest rank is found by a flat scan the compiler
        // vectorises; the links only find a part's neighbours.
        let Cache { ranks, toks, next, prev, .. } = self;
        let ranks = &mut ranks[..n];
        let toks = &mut toks[..n];
        let next = &mut next[..n];
        let prev = &mut prev[..n];
        for i in 0..n {
            toks[i] = v.byte_tok[piece[i] as usize];
            next[i] = i as i32 + 1;
            prev[i] = i as i32 - 1;
        }
        for i in 0..n - 1 {
            let t = v.byte_pair[(piece[i] as usize) << 8 | piece[i + 1] as usize];
            ranks[i] = if t == NO_TOKEN { NO_RANK } else { t };
        }
        ranks[n - 1] = NO_RANK;
        let pairs = &mut self.pairs;
        loop {
            let min_rank = ranks.iter().copied().fold(NO_RANK, i32::min);
            if min_rank == NO_RANK {
                break;
            }
            let at = ranks.iter().position(|&r| r == min_rank).unwrap();
            let j = next[at] as usize;
            toks[at] = min_rank;
            ranks[j] = NO_RANK;
            let k = next[j] as usize;
            next[at] = k as i32;
            if k < n {
                prev[k] = at as i32;
                let e = (next[k] as usize).min(n);
                ranks[at] = Self::pair_rank(pairs, v, toks[at], toks[k], &piece[at..e]);
            } else {
                ranks[at] = NO_RANK;
            }
            let before = prev[at];
            if before >= 0 {
                let b = before as usize;
                ranks[b] = Self::pair_rank(pairs, v, toks[b], toks[at], &piece[b..k.min(n)]);
            }
        }
        let mut i = 0usize;
        while i < n {
            out.push(toks[i] as u32);
            i = next[i] as usize;
        }
    }

    /// merge_into for long pieces: parts form a linked list and the heap holds
    /// (rank, start) of every pair that was ever a candidate, lowest rank then
    /// leftmost first, the order the scan would pick. A stale entry is skipped.
    fn merge_heap(&mut self, v: &Vocab, out: &mut Vec<u32>, piece: &[u8]) {
        let n = piece.len();
        let Cache { pairs, starts, ranks, toks, next, prev, heap, .. } = self;
        let starts = &mut starts[..n];
        let ranks = &mut ranks[..n];
        let toks = &mut toks[..n];
        let next = &mut next[..n];
        let prev = &mut prev[..n];
        heap.clear();
        for i in 0..n {
            starts[i] = i as i32;
            toks[i] = v.byte_tok[piece[i] as usize];
            next[i] = i as i32 + 1;
            prev[i] = i as i32 - 1;
            ranks[i] = NO_RANK;
        }
        next[n - 1] = -1;
        for i in 0..n - 1 {
            let t = v.byte_pair[(piece[i] as usize) << 8 | piece[i + 1] as usize];
            let r = if t == NO_TOKEN { NO_RANK } else { t };
            if r != NO_RANK {
                ranks[i] = r;
                heap_push(heap, ((r as u32 as u64) << 32) | i as u64);
            }
        }
        let end = |next: &[i32], starts: &[i32], i: usize| -> usize {
            let j = next[i];
            if j >= 0 { starts[j as usize] as usize } else { n }
        };
        while let Some(e) = heap_pop(heap) {
            let r = (e >> 32) as i32;
            let i = e as u32 as usize;
            if ranks[i] != r || next[i] < 0 {
                continue;
            }
            let j = next[i] as usize;
            next[i] = next[j];
            if next[j] >= 0 {
                prev[next[j] as usize] = i as i32;
            }
            toks[i] = r;
            ranks[j] = NO_RANK;
            let k = next[i];
            if k >= 0 {
                let k = k as usize;
                let e2 = end(next, starts, k);
                ranks[i] = Self::pair_rank(pairs, v, toks[i], toks[k], &piece[starts[i] as usize..e2]);
                if ranks[i] != NO_RANK {
                    heap_push(heap, ((ranks[i] as u32 as u64) << 32) | i as u64);
                }
            } else {
                ranks[i] = NO_RANK;
            }
            let p = prev[i];
            if p >= 0 {
                let p = p as usize;
                let e2 = end(next, starts, i);
                ranks[p] = Self::pair_rank(pairs, v, toks[p], toks[i], &piece[starts[p] as usize..e2]);
                if ranks[p] != NO_RANK {
                    heap_push(heap, ((ranks[p] as u32 as u64) << 32) | p as u64);
                }
            }
        }
        let mut i = 0i32;
        while i >= 0 {
            out.push(toks[i as usize] as u32);
            i = next[i as usize];
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

/// A vector of n zero values, from zeroed memory: a large one maps fresh
/// pages that are only touched when used, so it costs little to set up.
pub fn zeroed<T: Zeroable>(n: usize) -> Vec<T> {
    if n == 0 {
        return Vec::new();
    }
    let layout = std::alloc::Layout::array::<T>(n).expect("table size");
    // SAFETY: the layout is non-empty; T is valid as all-zero bytes (Zeroable),
    // and the vector takes the allocation with the layout it was made with
    unsafe {
        let p = std::alloc::alloc_zeroed(layout) as *mut T;
        if p.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        Vec::from_raw_parts(p, n, n)
    }
}

/// # Safety
/// Implementors are valid when every byte is zero.
pub unsafe trait Zeroable: Copy {}
unsafe impl Zeroable for u8 {}
unsafe impl Zeroable for u32 {}
unsafe impl Zeroable for u64 {}
unsafe impl Zeroable for CacheEntry {}
unsafe impl Zeroable for PairEntry {}

fn heap_push(h: &mut Vec<u64>, x: u64) {
    h.push(x);
    let mut i = h.len() - 1;
    while i > 0 {
        let p = (i - 1) / 2;
        if h[p] <= h[i] {
            break;
        }
        h.swap(p, i);
        i = p;
    }
}

fn heap_pop(h: &mut Vec<u64>) -> Option<u64> {
    if h.is_empty() {
        return None;
    }
    let top = h[0];
    let last = h.len() - 1;
    h.swap(0, last);
    h.pop();
    let mut i = 0;
    loop {
        let (l, r) = (2 * i + 1, 2 * i + 2);
        let mut m = i;
        if l < h.len() && h[l] < h[m] {
            m = l;
        }
        if r < h.len() && h[r] < h[m] {
            m = r;
        }
        if m == i {
            break;
        }
        h.swap(m, i);
        i = m;
    }
    Some(top)
}
