//! Exact cuts inside runs of three-byte characters. Byte-pair merging never
//! joins across a position that no vocabulary token spans, so a piece cut at
//! such positions encodes to the concatenation of its parts. For runs of
//! Chinese, Japanese or Korean this splits clauses that never repeat into
//! single characters and short words that repeat constantly.
//!
//! A token can span the boundary between characters c1 and c2 only by
//! containing a suffix of c1 and a prefix of c2, one to three bytes each. Every
//! such crossing in the vocabulary is recorded with up to three bytes of
//! context on each side; a boundary is safe when none of them fits. The test
//! is conservative: fewer bytes of context than a token really needs only make
//! a crossing match more boundaries, never fewer.


pub struct CjkCuts {
    /// crossings with three bytes on both sides, i.e. whole character pairs
    full: U64Set,
    /// crossings with fewer bytes on a side, keyed by lengths and bytes
    part: U64Set,
    /// per code point of the three-byte range: bit 0 when the character
    /// matches the left side of a partial crossing flagged by its left side,
    /// bit 1 the right side of one flagged by its right side, bit 2 the left
    /// of a whole-pair crossing, bit 3 the right. A boundary can only be
    /// blocked when bit 0 of the left character, bit 1 of the right one, or
    /// bit 2 of the left and bit 3 of the right are set.
    flags: Vec<u8>,
    /// the longest token that is a run of whole three-byte characters; a
    /// longer run cannot be a single token
    pub max_run_token: usize,
    /// a Bloom filter over the character pairs of whole-pair crossings, one
    /// hash, 2^17 bits
    pair_bloom: Vec<u64>,
    /// hashes of the tokens of nine or more bytes that are runs of whole
    /// three-byte characters: a run not in here cannot be a single token
    run_tokens: U64Set,
}

/// The code point of the three-byte character c.
#[inline(always)]
pub fn cp3(c: &[u8]) -> usize {
    (((c[0] & 0x0F) as usize) << 12) | (((c[1] & 0x3F) as usize) << 6) | (c[2] & 0x3F) as usize
}

#[inline(always)]
fn pack(b: &[u8]) -> u64 {
    let mut k = 0u64;
    for (i, &x) in b.iter().enumerate() {
        k |= (x as u64) << (8 * i);
    }
    k
}

fn unpack3(k: u64) -> [u8; 3] {
    [k as u8, (k >> 8) as u8, (k >> 16) as u8]
}

#[inline(always)]
fn bloom_bit(cp1: usize, cp2: usize) -> usize {
    ((((cp1 as u64) << 16 | cp2 as u64).wrapping_mul(0x9E3779B97F4A7C15)) >> (64 - 17)) as usize
}

#[inline(always)]
fn part_key(left: &[u8], right: &[u8]) -> u64 {
    pack(left) | (pack(right) << 24) | ((left.len() as u64) << 48) | ((right.len() as u64) << 52)
}

#[inline(always)]
fn is_cont(b: u8) -> bool {
    (0x80..0xC0).contains(&b)
}

#[inline(always)]
fn is_lead3(b: u8) -> bool {
    (0xE0..0xF0).contains(&b)
}

impl CjkCuts {
    pub fn new(tokens: &[(&[u8], u32)]) -> CjkCuts {
        // byte strings of up to three bytes, as (length << 24 | bytes)
        fn key(s: &[u8]) -> u32 {
            s.iter().enumerate().fold((s.len() as u32) << 24, |k, (i, &b)| k | (b as u32) << (8 * i))
        }
        let mut full = Vec::new();
        let mut part = Vec::new();
        let mut lefts: std::collections::HashSet<u32> = Default::default();
        let mut rights: std::collections::HashSet<u32> = Default::default();
        let mut full_l: std::collections::HashSet<u32> = Default::default();
        let mut full_r: std::collections::HashSet<u32> = Default::default();
        for &(t, _) in tokens {
            for q in 1..t.len() {
                if !(is_lead3(t[q]) && is_cont(t[q - 1])) {
                    continue;
                }
                let left = &t[q.saturating_sub(3)..q];
                let right = &t[q..(q + 3).min(t.len())];
                if left.len() == 3 && right.len() == 3 {
                    full.push(pack(left) | (pack(right) << 24));
                    full_l.insert(key(left));
                    full_r.insert(key(right));
                } else {
                    part.push(part_key(left, right));
                    // flag the side that names more bytes: it matches fewer characters
                    if left.len() >= right.len() {
                        lefts.insert(key(left));
                    } else {
                        rights.insert(key(right));
                    }
                }
            }
        }
        let mut flags = vec![0u8; 0x10000];
        for cp in 0x800u32..0x10000 {
            let Some(ch) = char::from_u32(cp) else { continue };
            let mut b = [0u8; 4];
            let c = ch.encode_utf8(&mut b).as_bytes();
            let mut f = 0u8;
            if (1..=3).any(|k| lefts.contains(&key(&c[3 - k..]))) {
                f |= 1;
            }
            if (1..=3).any(|k| rights.contains(&key(&c[..k]))) {
                f |= 2;
            }
            if full_l.contains(&key(c)) {
                f |= 4;
            }
            if full_r.contains(&key(c)) {
                f |= 8;
            }
            flags[cp as usize] = f;
        }
        let max_run_token = tokens.iter().map(|&(t, _)| t).filter(|t| t.len() >= 3 && all_three_byte_any(t)).map(|t| t.len()).max().unwrap_or(0);
        let run_hashes: Vec<u64> = tokens.iter().map(|&(t, _)| t).filter(|t| t.len() >= 9 && all_three_byte_any(t)).map(run_hash).collect();
        let mut pair_bloom = vec![0u64; (1 << 17) / 64];
        for &k in &full {
            let c1 = unpack3(k);
            let c2 = unpack3(k >> 24);
            let b = bloom_bit(cp3(&c1), cp3(&c2));
            pair_bloom[b / 64] |= 1 << (b % 64);
        }
        CjkCuts { full: U64Set::new(&full), part: U64Set::new(&part), flags, max_run_token, pair_bloom, run_tokens: U64Set::new(&run_hashes) }
    }

    /// Whether the boundary between characters cp1 and cp2 needs the full
    /// test: the flags alone decide most boundaries.
    #[inline(always)]
    pub fn needs_check(&self, cp1: usize, cp2: usize) -> bool {
        // SAFETY: code points of three-byte characters are below 0x10000
        let (f1, f2) = unsafe { (*self.flags.get_unchecked(cp1), *self.flags.get_unchecked(cp2)) };
        if (f1 & 1) | (f2 & 2) != 0 {
            return true;
        }
        if f1 & 4 == 0 || f2 & 8 == 0 {
            return false;
        }
        let b = bloom_bit(cp1, cp2);
        // SAFETY: b < 2^17 and the filter holds 2^17 bits
        unsafe { *self.pair_bloom.get_unchecked(b / 64) & (1 << (b % 64)) != 0 }
    }

    /// needs_check() from the flags already in two character entries.
    #[inline(always)]
    pub fn needs_check_flags(&self, f1: u8, f2: u8, cp1: usize, cp2: usize) -> bool {
        if (f1 & 1) | (f2 & 2) != 0 {
            return true;
        }
        if f1 & 4 == 0 || f2 & 8 == 0 {
            return false;
        }
        let b = bloom_bit(cp1, cp2);
        // SAFETY: b < 2^17 and the filter holds 2^17 bits
        unsafe { *self.pair_bloom.get_unchecked(b / 64) & (1 << (b % 64)) != 0 }
    }

    #[inline(always)]
    pub fn flag_of(&self, cp: usize) -> u8 {
        // SAFETY: callers pass code points of three-byte characters, below 0x10000
        unsafe { *self.flags.get_unchecked(cp) }
    }

    /// blocked(), with both code points already decoded. The common case, a
    /// left character that starts no crossing at all, is one byte load.
    #[inline(always)]
    pub fn blocked_cp(&self, cp1: usize, cp2: usize, c1: &[u8], c2: &[u8]) -> bool {
        // SAFETY: code points of three-byte characters are below 0x10000
        let f1 = unsafe { *self.flags.get_unchecked(cp1) };
        let f2 = unsafe { *self.flags.get_unchecked(cp2) };
        if f1 & 4 != 0 && f2 & 8 != 0 && self.full.contains(pack(c1) | (pack(c2) << 24)) {
            return true;
        }
        if f1 & 1 == 0 && f2 & 2 == 0 {
            return false;
        }
        for k1 in 1..=3 {
            for k2 in 1..=3 {
                if k1 == 3 && k2 == 3 {
                    continue;
                }
                if self.part.contains(part_key(&c1[3 - k1..], &c2[..k2])) {
                    return true;
                }
            }
        }
        false
    }

    /// Whether some token may span the boundary between the three-byte
    /// characters c1 and c2.
    #[inline(always)]
    pub fn blocked(&self, c1: &[u8], c2: &[u8]) -> bool {
        self.blocked_cp(cp3(c1), cp3(c2), c1, c2)
    }
}

#[inline(always)]
fn run_hash(t: &[u8]) -> u64 {
    crate::vocab::long_hash(t) as u64 | ((t.len() as u64) << 32)
}

impl CjkCuts {
    /// Whether the run s, nine or more bytes, may be a single token.
    #[inline(always)]
    pub fn may_be_token(&self, s: &[u8]) -> bool {
        s.len() <= self.max_run_token && self.run_tokens.contains(run_hash(s))
    }
}

/// Whether s is a run of whole three-byte characters, any number of them.
fn all_three_byte_any(s: &[u8]) -> bool {
    s.len() % 3 == 0 && s.chunks_exact(3).all(|c| is_lead3(c[0]) && is_cont(c[1]) && is_cont(c[2]))
}

/// Whether s, valid UTF-8, is a run of whole three-byte characters, at least
/// two of them. Valid UTF-8 means only the lead bytes need a look.
#[inline]
pub fn all_three_byte(s: &[u8]) -> bool {
    if s.len() < 6 || s.len() % 3 != 0 {
        return false;
    }
    let mut i = 0;
    while i < s.len() {
        if s[i] & 0xF0 != 0xE0 {
            return false;
        }
        i += 3;
    }
    true
}

/// A small open-addressing set of nonzero u64 keys.
struct U64Set {
    slots: Vec<u64>,
    shift: u32,
}

impl U64Set {
    fn new(keys: &[u64]) -> U64Set {
        let mut size = 16usize;
        while size < 2 * keys.len() {
            size <<= 1;
        }
        let shift = 64 - size.trailing_zeros();
        let mut slots = vec![0u64; size];
        for &k in keys {
            let k = k | (1 << 63); // never zero
            let mut i = (k.wrapping_mul(0x9E3779B97F4A7C15) >> shift) as usize;
            while slots[i] != 0 && slots[i] != k {
                i = (i + 1) & (size - 1);
            }
            slots[i] = k;
        }
        U64Set { slots, shift }
    }

    #[inline(always)]
    fn contains(&self, k: u64) -> bool {
        let k = k | (1 << 63);
        let mask = self.slots.len() - 1;
        let mut i = (k.wrapping_mul(0x9E3779B97F4A7C15) >> self.shift) as usize;
        loop {
            let v = self.slots[i];
            if v == k {
                return true;
            }
            if v == 0 {
                return false;
            }
            i = (i + 1) & mask;
        }
    }
}

/// Character classes for the fused span encoder, per code point of the
/// three-byte range, for one pre-tokenisation pattern.
pub const C_LETTER: u8 = 1; // continues a letter run (cl100k: \p{L}; o200k: in both case classes and a letter)
pub const C_MARK: u8 = 2; // o200k only: in both case classes but not \p{L} (marks): letter run and punctuation run
pub const C_NUM: u8 = 3; // \p{N}
pub const C_PUNCT: u8 = 4; // [^\s\p{L}\p{N}] and not C_MARK
pub const C_OTHER: u8 = 5; // anything the span encoder leaves to the scanner (whitespace, cased letters in o200k)

pub fn span_classes(kind: crate::scan::Kind) -> Vec<u8> {
    use crate::scan::Kind;
    let c = crate::unicode::classes();
    let mut t = vec![C_OTHER; 0x10000];
    for cp in 0x800u32..0x10000 {
        if char::from_u32(cp).is_none() {
            continue;
        }
        let letter = c.letter.has(cp);
        let number = c.number.has(cp);
        let space = c.space.has(cp);
        t[cp as usize] = match kind {
            Kind::Cl100k => {
                if letter {
                    C_LETTER
                } else if number {
                    C_NUM
                } else if space {
                    C_OTHER
                } else {
                    C_PUNCT
                }
            }
            Kind::O200k => {
                let (u, w) = (c.upperish.has(cp), c.lowerish.has(cp));
                if u && w {
                    if letter { C_LETTER } else { C_MARK }
                } else if u || w {
                    C_OTHER
                } else if number {
                    C_NUM
                } else if space {
                    C_OTHER
                } else {
                    C_PUNCT
                }
            }
            Kind::P50k => C_OTHER,
        };
    }
    t
}
