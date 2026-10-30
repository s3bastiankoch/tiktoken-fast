//! Memoised encoding of text segments. The text is cut wherever every pattern
//! resolves the same way with or without the other side (a single space
//! between two ASCII letters, or a letter followed by ASCII punctuation that
//! no pattern attaches to a word), so a segment's tokens depend on its bytes
//! alone. Running text draws most of its segments from a few thousand words,
//! and a table of those answers a segment in one probe instead of a scan, a
//! vocabulary lookup and a merge per piece.

use crate::merge::{zeroed, Zeroable};
use crate::unicode::{ASCII, CL_DIGIT, CL_LETTER, CL_SPACE};

pub const SEG_MAX: usize = 15; // longer segments take the piece path; byte 15 of the key holds the length
const SEG_TOK: usize = 5; // segments with more tokens are not memoised
const SEG_SETS_BITS: u32 = 14;
const SMALL_SEG_SETS_BITS: u32 = 8;
const SEG_WAYS: usize = 3;
pub const LONG_MAX: usize = 31; // segments of 16..=31 bytes go to the long memo
const LONG_TOK: usize = 8;
const LONG_BITS: usize = 12;
const LONG_SIZE: usize = 1 << LONG_BITS;
const LONG_WAYS: usize = 4;
const LONG_SETS: usize = LONG_SIZE / LONG_WAYS;
const SMALL_LONG_SETS: usize = 64;

/// Three ways in one cache line: both key words and the first token of each,
/// the token count in the first token's top three bits. A one-token segment,
/// two thirds of them in running text, is answered from this line alone. A
/// key of (0, 0) is never a real segment (the length sits in the top byte of
/// k1), so zeroed ways are empty.
#[derive(Clone, Copy)]
#[repr(C, align(64))]
pub struct KeySet {
    k0: [u64; SEG_WAYS],
    k1: [u64; SEG_WAYS],
    t0: [u32; SEG_WAYS],
}

/// Tokens two to five of each way.
#[derive(Clone, Copy)]
#[repr(C, align(64))]
pub struct TokSet {
    t: [[u32; SEG_TOK - 1]; SEG_WAYS],
}

pub struct SegCache {
    keys: Vec<KeySet>,
    toks: Vec<TokSet>,
    victim: Vec<u8>,
    long: Vec<LongEntry>,
    long_victim: Vec<u8>,
    shift: u32, // 64 minus the log2 of the number of sets
}

/// A memo entry for a segment of 16 to 31 bytes: four key words, the length in
/// the top byte of the last, up to eight tokens.
#[derive(Clone, Copy)]
pub struct LongEntry {
    k: [u64; 4],
    ntok: u8,
    tok: [u32; LONG_TOK],
}

impl SegCache {
    /// A full-size memo.
    pub fn new() -> SegCache {
        Self::sized(SEG_SETS_BITS, LONG_SETS)
    }

    /// A memo small enough to set up in a few microseconds; `grow` replaces it.
    pub fn small() -> SegCache {
        Self::at_level(0)
    }

    /// A memo of the given size step: 0 none (the caller skips it), 1 tiny,
    /// 2 small, 3 and up full.
    pub fn at_level(level: u8) -> SegCache {
        match level {
            0 => SegCache { keys: Vec::new(), toks: Vec::new(), victim: Vec::new(), long: Vec::new(), long_victim: Vec::new(), shift: 63 },
            1 => Self::sized(SMALL_SEG_SETS_BITS - 2, SMALL_LONG_SETS / 4),
            2 => Self::sized(SMALL_SEG_SETS_BITS, SMALL_LONG_SETS),
            _ => Self::new(),
        }
    }

    fn sized(bits: u32, long_sets: usize) -> SegCache {
        SegCache {
            keys: zeroed(1 << bits),
            toks: zeroed(1 << bits),
            victim: zeroed(1 << bits),
            long: zeroed(long_sets * LONG_WAYS),
            long_victim: zeroed(long_sets),
            shift: 64 - bits,
        }
    }

    /// The memoised tokens of the segment with key (k0, k1), for the statistics example.
    pub fn get(&self, k0: u64, k1: u64) -> Option<Vec<u32>> {
        let mut v = Vec::new();
        if self.append(k0, k1, &mut v) {
            Some(v)
        } else {
            None
        }
    }

    /// Appends the memoised tokens of the segment with key (k0, k1) to out and
    /// returns true, or returns false when the memo does not hold it.
    pub fn append(&self, k0: u64, k1: u64, out: &mut Vec<u32>) -> bool {
        let mut buf = [0u32; SEG_TOK];
        // SAFETY: buf has SEG_TOK slots
        let n = unsafe { self.write_at(k0, k1, buf.as_mut_ptr()) };
        out.extend_from_slice(&buf[..n]);
        n != 0
    }

    /// Writes the memoised tokens of the segment with key (k0, k1) at dst and
    /// returns how many, or 0 when the memo does not hold it.
    ///
    /// # Safety
    /// dst must be valid for SEG_TOK writes.
    #[inline(always)]
    pub unsafe fn write_at(&self, k0: u64, k1: u64, dst: *mut u32) -> usize {
        let set = set_of(k0, k1, self.shift);
        let ks = &self.keys[set];
        let m = ((ks.k0[0] == k0 && ks.k1[0] == k1) as u32)
            | (((ks.k0[1] == k0 && ks.k1[1] == k1) as u32) << 1)
            | (((ks.k0[2] == k0 && ks.k1[2] == k1) as u32) << 2);
        if m == 0 {
            return 0;
        }
        let w = m.trailing_zeros() as usize;
        let t0 = ks.t0[w];
        let ntok = (t0 >> 29) as usize;
        *dst = t0 & 0x1FFF_FFFF;
        if ntok > 1 {
            std::ptr::copy_nonoverlapping(self.toks[set].t[w].as_ptr(), dst.add(1), SEG_TOK - 1);
        }
        ntok
    }

    #[inline]
    pub fn put(&mut self, k0: u64, k1: u64, toks: &[u32]) {
        if toks.len() > SEG_TOK || toks.is_empty() || toks[0] >= 1 << 29 {
            return;
        }
        let set = set_of(k0, k1, self.shift);
        let w = self.victim[set] as usize;
        self.victim[set] = ((w + 1) % SEG_WAYS) as u8;
        let ks = &mut self.keys[set];
        ks.k0[w] = k0;
        ks.k1[w] = k1;
        ks.t0[w] = toks[0] | ((toks.len() as u32) << 29);
        let t = &mut self.toks[set].t[w];
        t[..toks.len() - 1].copy_from_slice(&toks[1..]);
    }
}

impl SegCache {
    /// Appends the memoised tokens of a long segment with key k, or returns false.
    #[inline(always)]
    pub fn append_long(&self, k: &[u64; 4], out: &mut Vec<u32>) -> bool {
        let base = long_set_of(k, self.long_victim.len()) * LONG_WAYS;
        for w in 0..LONG_WAYS {
            let e = &self.long[base + w];
            if e.k == *k && e.ntok != 0 {
                out.extend_from_slice(&e.tok[..e.ntok as usize]);
                return true;
            }
        }
        false
    }

    pub fn put_long(&mut self, k: &[u64; 4], toks: &[u32]) {
        if toks.len() > LONG_TOK || toks.is_empty() {
            return;
        }
        let set = long_set_of(k, self.long_victim.len());
        let w = self.long_victim[set] as usize;
        self.long_victim[set] = ((w + 1) % LONG_WAYS) as u8;
        let e = &mut self.long[set * LONG_WAYS + w];
        e.k = *k;
        e.ntok = toks.len() as u8;
        e.tok[..toks.len()].copy_from_slice(toks);
    }
}

#[inline(always)]
fn long_set_of(k: &[u64; 4], sets: usize) -> usize {
    let h = (k[0] ^ k[1].rotate_left(17) ^ k[2].rotate_left(34) ^ k[3].rotate_left(51)).wrapping_mul(0x9E3779B97F4A7C15);
    (h >> (64 - 10)) as usize & (sets - 1)
}

/// Packs the 16 to 31 bytes text[i..end] into four words, the length in the top byte.
#[inline(always)]
pub fn key32_at(text: &[u8], i: usize, end: usize) -> [u64; 4] {
    let n = end - i;
    let mut k = [0u64; 4];
    for (j, word) in k.iter_mut().enumerate() {
        let p = i + 8 * j;
        if p >= end {
            break;
        }
        let m = (end - p).min(8);
        *word = match text.get(p..p + 8) {
            Some(w) => {
                let v = u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]]);
                if m >= 8 { v } else { v & (u64::MAX >> (64 - 8 * m as u32)) }
            }
            None => {
                let mut v = 0u64;
                for (q, &b) in text[p..p + m].iter().enumerate() {
                    v |= (b as u64) << (8 * q);
                }
                v
            }
        };
    }
    k[3] |= (n as u64) << 56;
    k
}

impl Default for SegCache {
    fn default() -> Self {
        Self::new()
    }
}

#[inline(always)]
fn set_of(k0: u64, k1: u64, shift: u32) -> usize {
    let h = (k0 ^ k1.rotate_left(29)).wrapping_mul(0x9E3779B97F4A7C15);
    (h >> shift) as usize
}

/// Packs the up to sixteen bytes text[i..end] into two words. Where eight
/// bytes are readable the key is one load and a mask.
#[inline(always)]
pub fn key16_at(text: &[u8], i: usize, end: usize) -> (u64, u64) {
    #[inline(always)]
    fn load(text: &[u8], p: usize, n: usize) -> u64 {
        if n == 0 {
            return 0;
        }
        let w = match text.get(p..p + 8) {
            Some(w) => u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]]),
            None => {
                let mut k = 0u64;
                for (j, &b) in text[p..p + n].iter().enumerate() {
                    k |= (b as u64) << (8 * j);
                }
                return k;
            }
        };
        if n >= 8 { w } else { w & (u64::MAX >> (64 - 8 * n as u32)) }
    }
    let n = end - i;
    (load(text, i, n.min(8)), load(text, i + 8, n.saturating_sub(8)) | ((n as u64) << 56))
}

/// Sets one bit per byte of s in `bits`, at every cut: a single ASCII space
/// with an ASCII letter on both sides, or ASCII punctuation (not the
/// apostrophe, not whitespace, not alphanumeric) right after an ASCII letter.
/// One streaming pass, eight bytes per step, with the letter flag of the
/// byte before and after carried across words.
pub fn cut_bits(s: &[u8], bits: &mut Vec<u64>) {
    let words = (s.len() + 63) / 64;
    bits.clear();
    bits.resize(words, 0);
    let n = s.len();
    let mut prev_letter = 0u64; // 0x80 if the byte before this word is a letter
    let mut p = 0usize;
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") && n > 64 {
            // SAFETY: the feature was detected at run time
            p = unsafe { cut_bits_avx2(s, bits) };
            prev_letter = if p > 0 && s[p - 1].is_ascii_alphabetic() { 0x80 } else { 0 };
        }
    }
    // byte p..p+8 as a word, zero past the end
    let load = |p: usize| -> u64 {
        match s.get(p..p + 8) {
            Some(w) => u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]]),
            None => {
                let mut k = 0u64;
                for (j, &b) in s[p.min(n)..].iter().enumerate() {
                    k |= (b as u64) << (8 * j);
                }
                k
            }
        }
    };
    let mut w = load(p);
    let mut letter = letters8(w);
    while p < n {
        let w_next = load(p + 8);
        let letter_next = letters8(w_next);
        let prev = (letter << 8) | prev_letter;
        let next = (letter >> 8) | (letter_next << 56);
        let space = spaces8(w);
        let punct = HIGH & !(w & HIGH) & !letter & !digits8(w) & !spaces8_all(w) & !apos8(w);
        let cut = (space & prev & next) | (punct & prev);
        if cut != 0 {
            // one bit per byte: compress the 0x80 flags to eight bits
            let m = cut.wrapping_mul(0x0002040810204081) >> 56;
            bits[p / 64] |= m << (p % 64);
        }
        prev_letter = (letter >> 56) & 0x80;
        w = w_next;
        letter = letter_next;
        p += 8;
    }
    // bytes past the end never cut; bit 0 is never a cut
    if let Some(last) = bits.last_mut() {
        let valid = n % 64;
        if valid != 0 {
            *last &= (1u64 << valid) - 1;
        }
    }
    if let Some(first) = bits.first_mut() {
        *first &= !1;
    }
}

/// The next cut strictly after i, or the end of s, from the bitmap.
#[inline(always)]
pub fn next_cut_bits(bits: &[u64], n: usize, i: usize) -> usize {
    let p = i + 1;
    if p >= n {
        return n;
    }
    let mut wi = p / 64;
    let mut w = bits[wi] & (u64::MAX << (p % 64));
    loop {
        if w != 0 {
            let q = wi * 64 + w.trailing_zeros() as usize;
            return q.min(n);
        }
        wi += 1;
        if wi >= bits.len() {
            return n;
        }
        w = bits[wi];
    }
    #[allow(unreachable_code)]
    {
        p = n;
        p
    }
}

/// The next cut strictly after i, or the end of s, byte by byte.
pub fn next_cut(s: &[u8], i: usize) -> usize {
    let n = s.len();
    let mut p = i + 1;
    while p < n {
        let c = s[p];
        if c < 0x80 {
            let cl = ASCII[c as usize];
            if c == b' ' {
                if p + 1 < n && s[p - 1].is_ascii_alphabetic() && s[p + 1].is_ascii_alphabetic() {
                    return p;
                }
            } else if cl & (CL_LETTER | CL_DIGIT | CL_SPACE) == 0 && c != b'\'' && s[p - 1].is_ascii_alphabetic() {
                return p;
            }
        }
        p += 1;
    }
    n
}

/// The AVX2 form of the streaming pass: 32 bytes per step, class flags by
/// byte compares, one bit per byte from movemask. Handles whole blocks while
/// a byte after the block exists and returns the position it stopped at; the
/// word loop finishes from there.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn cut_bits_avx2(s: &[u8], bits: &mut [u64]) -> usize {
    use std::arch::x86_64::*;
    let n = s.len();
    #[inline(always)]
    unsafe fn mask_between(v: __m256i, lo_excl: i8, hi_excl: i8) -> u32 {
        // bytes strictly between lo_excl and hi_excl, signed, so bytes >= 0x80 never qualify
        let ge = _mm256_cmpgt_epi8(v, _mm256_set1_epi8(lo_excl));
        let le = _mm256_cmpgt_epi8(_mm256_set1_epi8(hi_excl), v);
        _mm256_movemask_epi8(_mm256_and_si256(ge, le)) as u32
    }
    #[inline(always)]
    unsafe fn letters(v: __m256i) -> u32 {
        mask_between(_mm256_or_si256(v, _mm256_set1_epi8(0x20)), 0x60, 0x7b)
    }
    let mut p = 0usize;
    let mut prev_letter = 0u32;
    let mut v = _mm256_loadu_si256(s.as_ptr() as *const __m256i);
    let mut letter = letters(v);
    while p + 32 < n {
        // the next block, or the single byte after this one, gives the lookahead bit
        let full_next = p + 64 <= n;
        let (next_v, next_letter, next_bit) = if full_next {
            let nv = _mm256_loadu_si256(s.as_ptr().add(p + 32) as *const __m256i);
            let nl = letters(nv);
            (nv, nl, nl & 1)
        } else {
            (v, 0, s[p + 32].is_ascii_alphabetic() as u32)
        };
        let digit = mask_between(v, 0x2f, 0x3a);
        let space = _mm256_movemask_epi8(_mm256_cmpeq_epi8(v, _mm256_set1_epi8(0x20))) as u32;
        let ctl = mask_between(v, 0x08, 0x0e);
        let apos = _mm256_movemask_epi8(_mm256_cmpeq_epi8(v, _mm256_set1_epi8(0x27))) as u32;
        let ascii = !(_mm256_movemask_epi8(v) as u32);
        let punct = ascii & !letter & !digit & !space & !ctl & !apos;
        let prev = (letter << 1) | prev_letter;
        let next = (letter >> 1) | (next_bit << 31);
        let cut = (space & prev & next) | (punct & prev);
        if cut != 0 {
            bits[p / 64] |= (cut as u64) << (p % 64);
        }
        prev_letter = letter >> 31;
        p += 32;
        if !full_next {
            break; // fewer than 32 bytes left after p: the word loop finishes
        }
        v = next_v;
        letter = next_letter;
    }
    if let Some(first) = bits.first_mut() {
        *first &= !1;
    }
    p
}

const ONES: u64 = 0x0101010101010101;
const HIGH: u64 = 0x8080808080808080;

/// 0x80 in every byte of w that is an ASCII letter.
#[inline(always)]
fn letters8(w: u64) -> u64 {
    let high = w & HIGH;
    let v = (w | (0x20 * ONES)) & !HIGH; // folded to lowercase, seven bits
    let ge = (v + ((0x80 - 0x61) * ONES)) & HIGH; // v >= 'a'
    let gt = (v + ((0x7f - 0x7a) * ONES)) & HIGH; // v > 'z'
    ge & !gt & !high
}

/// 0x80 in every byte of w that is a space.
#[inline(always)]
fn spaces8(w: u64) -> u64 {
    let x = w ^ (0x20 * ONES);
    !((x & !HIGH) + (0x7f * ONES)) & !x & HIGH
}

/// 0x80 in every byte of w that is ASCII whitespace: space or 0x09..=0x0d.
#[inline(always)]
fn spaces8_all(w: u64) -> u64 {
    let v = w & !HIGH;
    let ge = (v + ((0x80 - 0x09) * ONES)) & HIGH;
    let gt = (v + ((0x7f - 0x0d) * ONES)) & HIGH;
    (ge & !gt & !(w & HIGH)) | spaces8(w)
}

/// 0x80 in every byte of w that is an apostrophe.
#[inline(always)]
fn apos8(w: u64) -> u64 {
    let x = w ^ (0x27 * ONES);
    !((x & !HIGH) + (0x7f * ONES)) & !x & HIGH
}

/// 0x80 in every byte of w that is an ASCII digit.
#[inline(always)]
fn digits8(w: u64) -> u64 {
    let v = w & !HIGH;
    let ge = (v + ((0x80 - 0x30) * ONES)) & HIGH;
    let gt = (v + ((0x7f - 0x39) * ONES)) & HIGH;
    ge & !gt & !(w & HIGH)
}

/// The byte-at-a-time definition, for the tests.
#[cfg(test)]
pub fn next_cut_slow(s: &[u8], i: usize) -> usize {
    let n = s.len();
    let mut p = i + 1;
    while p < n {
        let c = s[p];
        if c < 0x80 {
            let cl = ASCII[c as usize];
            if c == b' ' {
                if p + 1 < n && s[p - 1].is_ascii_alphabetic() && s[p + 1].is_ascii_alphabetic() {
                    return p;
                }
            } else if cl & (CL_LETTER | CL_DIGIT | CL_SPACE) == 0 && c != b'\'' && s[p - 1].is_ascii_alphabetic() {
                return p;
            }
        }
        p += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuts_match_slow_definition() {
        let mut x: u64 = 7;
        let alphabet: &[u8] = b"abcXYZ  \t\n',.;:-/0123\x7f\x80\xc3\xa9";
        for round in 0..3000 {
            let len = if round % 3 == 0 { (x % 700) as usize + 65 } else { (x % 64) as usize + 1 };
            let mut s = Vec::with_capacity(len);
            for _ in 0..len {
                x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                s.push(alphabet[(x >> 33) as usize % alphabet.len()]);
            }
            let mut bits = Vec::new();
            cut_bits(&s, &mut bits);
            let mut i = 0;
            while i < s.len() {
                let a = next_cut_bits(&bits, s.len(), i);
                let b = next_cut_slow(&s, i);
                assert_eq!(a, b, "at {i} in {:?}", String::from_utf8_lossy(&s));
                assert_eq!(next_cut(&s, i), b);
                i = a;
            }
        }
    }
}

// SAFETY: plain integers and arrays of them
unsafe impl Zeroable for KeySet {}
unsafe impl Zeroable for TokSet {}
unsafe impl Zeroable for LongEntry {}

/// Longest segment the long-segment memo holds.
pub const BIG_MAX: usize = 16 << 10;

/// A memo for segments of 32 to BIG_MAX bytes, mostly runs of text without
/// ASCII spaces (Thai, Chinese and Japanese paragraphs, long identifiers). A
/// segment is stored the second time it is seen: a first sighting only sets
/// a bit, so text that never repeats pays for one hash per segment. What it
/// stores stays within a byte budget.
pub struct BigMemo {
    slots: Vec<Option<BigEntry>>,
    seen: Vec<u64>,
    stored: usize,
    budget: usize,
}

struct BigEntry {
    hash: u64,
    bytes: Box<[u8]>,
    toks: Box<[u32]>,
}

impl BigMemo {
    /// A memo of the given size step; none below step 2.
    pub fn at_level(level: u8) -> BigMemo {
        let (slots, seen_words, budget) = match level {
            0 | 1 => (0, 0, 0),
            2 => (1 << 9, 1 << 9, 256 << 10),
            _ => (1 << 11, 1 << 11, 2 << 20),
        };
        BigMemo { slots: (0..slots).map(|_| None).collect(), seen: vec![0; seen_words], stored: 0, budget }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    #[inline]
    pub fn hash(s: &[u8]) -> u64 {
        let mut h = 0x9E3779B97F4A7C15u64 ^ s.len() as u64;
        let mut c = s.chunks_exact(8);
        for w in &mut c {
            h = (h ^ u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]])).wrapping_mul(0xBF58476D1CE4E5B9);
            h ^= h >> 29;
        }
        for &b in c.remainder() {
            h = (h ^ b as u64).wrapping_mul(0x94D049BB133111EB);
        }
        h ^ (h >> 32)
    }

    /// Appends the tokens of segment s with hash h and returns true, or
    /// returns false when the memo does not hold it.
    #[inline]
    pub fn append(&self, h: u64, s: &[u8], out: &mut Vec<u32>) -> bool {
        match &self.slots[h as usize & (self.slots.len() - 1)] {
            Some(e) if e.hash == h && &*e.bytes == s => {
                out.extend_from_slice(&e.toks);
                true
            }
            _ => false,
        }
    }

    /// Records that segment s encodes to toks: stores it when it was seen
    /// before, otherwise marks it as seen.
    pub fn note(&mut self, h: u64, s: &[u8], toks: &[u32]) {
        let bit = (h >> 40) as usize & (self.seen.len() * 64 - 1);
        let (w, m) = (bit / 64, 1u64 << (bit % 64));
        if self.seen[w] & m == 0 {
            self.seen[w] |= m;
            return;
        }
        let i = h as usize & (self.slots.len() - 1);
        let size = |e: &BigEntry| e.bytes.len() + 4 * e.toks.len();
        let freed = self.slots[i].as_ref().map_or(0, size);
        let need = s.len() + 4 * toks.len();
        if self.stored - freed + need > self.budget {
            return;
        }
        self.stored = self.stored - freed + need;
        self.slots[i] = Some(BigEntry { hash: h, bytes: s.into(), toks: toks.into() });
    }
}
