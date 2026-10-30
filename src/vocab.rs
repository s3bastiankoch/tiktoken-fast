//! The token table the hot path reads. Pieces of up to eight bytes are looked
//! up by their bytes packed into one integer; longer pieces hash into a second
//! table over one byte arena. Both keep one 16-byte entry per slot.

pub const NO_TOKEN: i32 = -1;

/// Bytes of non-ASCII pieces merged before the backtracking tables of a
/// shipped vocabulary are built (they take about 45 ms to build).
const BT_FROM: usize = 1 << 20;

/// Twelve bytes: the key split in two so the entry aligns to four, and the
/// token with the length in its top four bits. The table then takes 1.5 MB
/// for cl100k_base, which keeps more of it in L2.
#[derive(Clone, Copy)]
struct ShortEntry {
    k_lo: u32,
    k_hi: u32,
    vn: u32, // (n << 28) | token; n == 0 marks an empty slot
}

impl ShortEntry {
    const EMPTY: ShortEntry = ShortEntry { k_lo: 0, k_hi: 0, vn: 0 };
    #[inline(always)]
    fn new(key: u64, n: usize, tok: u32) -> ShortEntry {
        ShortEntry { k_lo: key as u32, k_hi: (key >> 32) as u32, vn: ((n as u32) << 28) | tok }
    }
    #[inline(always)]
    fn key(&self) -> u64 {
        (self.k_hi as u64) << 32 | self.k_lo as u64
    }
}

#[derive(Clone, Copy)]
struct LongEntry {
    hash: u32,
    off: u32,
    n: u32,
    val: i32,
}

pub struct Vocab {
    short: Vec<ShortEntry>,
    smask: usize,
    sshift: u32,
    long: Vec<LongEntry>,
    lmask: usize,
    arena: Vec<u8>,
    pub byte_tok: [i32; 256],
    /// the token of every two-byte string, by (first << 8 | second), or NO_TOKEN
    pub byte_pair: Vec<i32>,
    /// one bit per byte pair (first << 8 | second): set when the pair occurs
    /// inside some token. Where it does not, no merge can join the bytes on
    /// either side, so the two sides encode apart.
    pub pair_inside: Vec<u64>,
    /// the byte-pair merge of each character U+1F000..U+1FFFF (most emoji),
    /// packed as count | tokens << 3 + 18k, or 0 when it does not pack; filled
    /// by CoreBPE::new once the merge is available
    pub emoji: Vec<u64>,
    /// tables for byte-pair encoding by backtracking, when the vocabulary allows it
    /// tables for byte-pair encoding by backtracking, when the vocabulary
    /// allows it; for a shipped vocabulary built on first use
    bt: std::sync::OnceLock<Option<Box<crate::backtrack::Backtrack>>>,
    /// the tokens by rank (bytes, start offsets), kept until bt is built
    bt_src: std::sync::Mutex<Option<(Vec<u8>, Vec<u32>)>>,
    /// every token is known to encode to itself
    pub self_encoding: bool,
    /// bytes of non-ASCII pieces merged without the backtracking tables
    wide_seen: std::sync::atomic::AtomicUsize,
}

/// The up to eight bytes of s packed little-endian into one integer. Lengths
/// that are not a power of two take two overlapping loads instead of a loop.
#[inline]
pub fn pack8(s: &[u8]) -> u64 {
    let n = s.len();
    let u16_at = |i: usize| u16::from_le_bytes([s[i], s[i + 1]]) as u64;
    let u32_at = |i: usize| u32::from_le_bytes([s[i], s[i + 1], s[i + 2], s[i + 3]]) as u64;
    match n {
        8 => u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]),
        5..=7 => u32_at(0) | u32_at(n - 4) << (8 * (n - 4)),
        4 => u32_at(0),
        3 => u16_at(0) | u16_at(1) << 8,
        2 => u16_at(0),
        1 => s[0] as u64,
        _ => 0,
    }
}

#[inline]
fn short_hash(key: u64, n: usize) -> u64 {
    (key ^ ((n as u64) << 56)).wrapping_mul(0x9E3779B97F4A7C15)
}

#[inline]
pub fn long_hash(s: &[u8]) -> u32 {
    let mut h: u64 = 0x9E3779B97F4A7C15 ^ s.len() as u64;
    let mut chunks = s.chunks_exact(8);
    for c in &mut chunks {
        let w = u64::from_le_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]);
        h = (h ^ w).wrapping_mul(0xBF58476D1CE4E5B9);
        h ^= h >> 31;
    }
    let rest = chunks.remainder();
    if !rest.is_empty() {
        h = (h ^ pack8(rest)).wrapping_mul(0x94D049BB133111EB);
        h ^= h >> 29;
    }
    (h ^ (h >> 32)) as u32
}

fn table_size(n: usize) -> usize {
    let mut s = 16;
    while s < 2 * n {
        s <<= 1;
    }
    s
}

impl Vocab {
    pub fn new(tokens: &[(&[u8], u32)], verify: bool) -> Vocab {
        let (mut n_short, mut n_long, mut arena_len) = (0usize, 0usize, 0usize);
        for &(k, _) in tokens {
            if k.len() <= 8 {
                n_short += 1;
            } else {
                n_long += 1;
                arena_len += k.len();
            }
        }
        let ss = table_size(n_short * 2 / 3); // three-quarter load; one line per probe
        let mut sshift = 0;
        let mut b = 1;
        while (1usize << b) < ss {
            sshift = 64 - b - 1;
            b += 1;
        }
        let mut v = Vocab {
            short: vec![ShortEntry::EMPTY; ss],
            smask: ss - 1,
            sshift: sshift as u32,
            long: vec![LongEntry { hash: 0, off: 0, n: 0, val: NO_TOKEN }; table_size(n_long)],
            lmask: table_size(n_long) - 1,
            arena: Vec::with_capacity(arena_len),
            byte_tok: [NO_TOKEN; 256],
            byte_pair: vec![NO_TOKEN; 1 << 16],
            bt: std::sync::OnceLock::new(),
            bt_src: std::sync::Mutex::new(None),
            self_encoding: false,
            wide_seen: std::sync::atomic::AtomicUsize::new(0),
            pair_inside: vec![0; 1 << 10],
            emoji: Vec::new(),
        };
        for &(k, tok) in tokens {
            if k.len() <= 8 {
                let key = pack8(k);
                let mut i = (short_hash(key, k.len()) >> v.sshift) as usize;
                while v.short[i].vn != 0 {
                    i = (i + 1) & v.smask;
                }
                v.short[i] = ShortEntry::new(key, k.len(), tok);
            } else {
                let h = long_hash(k);
                let mut i = h as usize & v.lmask;
                while v.long[i].val != NO_TOKEN {
                    i = (i + 1) & v.lmask;
                }
                v.long[i] = LongEntry { hash: h, off: v.arena.len() as u32, n: k.len() as u32, val: tok as i32 };
                v.arena.extend_from_slice(k);
            }
            if k.len() == 1 {
                v.byte_tok[k[0] as usize] = tok as i32;
            }
            if k.len() == 2 {
                v.byte_pair[(k[0] as usize) << 8 | k[1] as usize] = tok as i32;
            }
            for w in k.windows(2) {
                let i = (w[0] as usize) << 8 | w[1] as usize;
                v.pair_inside[i >> 6] |= 1 << (i & 63);
            }
        }
        if verify {
            let bt = crate::backtrack::Backtrack::new(&v, tokens, true).map(Box::new);
            v.self_encoding = bt.is_some();
            let _ = v.bt.set(bt);
        } else {
            // a shipped vocabulary known to pass the check: the tables wait
            // for the first piece that needs them
            let mut bytes = Vec::with_capacity(tokens.iter().map(|(k, _)| k.len()).sum());
            let mut starts = Vec::with_capacity(tokens.len() + 1);
            starts.push(0u32);
            for (i, &(k, t)) in tokens.iter().enumerate() {
                debug_assert_eq!(t as usize, i, "shipped tokens are dense and sorted");
                bytes.extend_from_slice(k);
                starts.push(bytes.len() as u32);
            }
            v.bt_src = std::sync::Mutex::new(Some((bytes, starts)));
            v.self_encoding = true;
        }
        v
    }

    /// The backtracking tables, built on first use for a shipped vocabulary.
    #[inline]
    pub fn bt(&self) -> Option<&crate::backtrack::Backtrack> {
        self.bt.get_or_init(|| self.build_bt()).as_deref()
    }

    /// The backtracking tables for a non-ASCII piece of n bytes, or None
    /// while they are not worth building yet: a shipped vocabulary builds
    /// them once BT_FROM bytes of such pieces have been merged the plain way,
    /// so a process that encodes little non-ASCII text never pays for them.
    #[inline]
    pub fn bt_for(&self, n: usize) -> Option<&crate::backtrack::Backtrack> {
        match self.bt.get() {
            Some(bt) => bt.as_deref(),
            None => {
                let seen = self.wide_seen.fetch_add(n, std::sync::atomic::Ordering::Relaxed) + n;
                if seen >= BT_FROM {
                    self.bt()
                } else {
                    None
                }
            }
        }
    }

    #[cold]
    #[inline(never)]
    fn build_bt(&self) -> Option<Box<crate::backtrack::Backtrack>> {
        let (bytes, starts) = self.bt_src.lock().unwrap().take()?;
        let tokens: Vec<(&[u8], u32)> = (0..starts.len() - 1).map(|i| (&bytes[starts[i] as usize..starts[i + 1] as usize], i as u32)).collect();
        crate::backtrack::Backtrack::new(self, &tokens, false).map(Box::new)
    }

    /// The token of the piece text[i..end], or NO_TOKEN. When eight bytes are
    /// readable at i the key is one unaligned load and a mask.
    #[inline(always)]
    pub fn lookup_at(&self, text: &[u8], i: usize, end: usize) -> i32 {
        let n = end - i;
        if n <= 8 {
            if let Some(w) = text.get(i..i + 8) {
                let key = u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]]) & (u64::MAX >> (64 - 8 * n as u32));
                return self.lookup_short(key, n);
            }
        }
        self.lookup(&text[i..end])
    }

    /// Pulls the table line for a short key towards the cache, for a lookup
    /// that follows a few pieces later.
    #[inline(always)]
    pub fn prefetch_short(&self, key: u64, n: usize) {
        let i = (short_hash(key, n) >> self.sshift) as usize;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use std::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};
            _mm_prefetch(self.short.as_ptr().add(i) as *const i8, _MM_HINT_T0);
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let _ = i;
        }
    }

    #[inline(always)]
    pub fn lookup_short(&self, key: u64, n: usize) -> i32 {
        let h = short_hash(key, n);
        let mut i = (h >> self.sshift) as usize;
        let want = (n as u32) << 28;
        loop {
            let e = &self.short[i];
            if e.key() == key && e.vn & 0xF000_0000 == want {
                return (e.vn & 0x0FFF_FFFF) as i32;
            }
            if e.vn == 0 {
                return NO_TOKEN;
            }
            i = (i + 1) & self.smask;
        }
    }

    /// Whether no token holds the byte a followed by the byte b, so a piece
    /// can be cut between them.
    #[inline(always)]
    pub fn can_cut(&self, a: u8, b: u8) -> bool {
        let i = (a as usize) << 8 | b as usize;
        self.pair_inside[i >> 6] & (1 << (i & 63)) == 0
    }

    /// The token of piece s, or NO_TOKEN.
    #[inline]
    pub fn lookup(&self, s: &[u8]) -> i32 {
        if s.len() <= 8 {
            if s.is_empty() {
                return NO_TOKEN;
            }
            return self.lookup_short(pack8(s), s.len());
        }
        let h = long_hash(s);
        let mut i = h as usize & self.lmask;
        loop {
            let e = &self.long[i];
            if e.val == NO_TOKEN {
                return NO_TOKEN;
            }
            if e.hash == h && e.n as usize == s.len() && &self.arena[e.off as usize..(e.off + e.n) as usize] == s {
                return e.val;
            }
            i = (i + 1) & self.lmask;
        }
    }
}
