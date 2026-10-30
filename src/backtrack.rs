//! Byte-pair encoding by backtracking, after the `bpe` crate of GitHub's
//! rust-gems (MIT): take the longest token at the position, keep it when it
//! forms a valid pair with the token before it, otherwise try the next
//! shorter prefix token, and step back when none fits. A pair is valid when
//! byte-pair encoding the two tokens' bytes gives back the same two tokens;
//! that is decided by unwinding the merges that built them, from a split
//! table and a table of the merges themselves.
//!
//! The longest-token search uses the vocabulary's own hash table: a small
//! table keyed by a token's first three bytes holds the lengths that tokens
//! with those bytes have, and only those lengths are looked up.

use crate::vocab::{long_hash, pack8, Vocab, NO_TOKEN};

const NONE: u32 = u32::MAX;
const EMPTY: u64 = u64::MAX;
const ID_BITS: u32 = 21;
const ID_MASK: u64 = (1 << ID_BITS) - 1;

pub struct Backtrack {
    /// token bytes by id, as offsets into bytes
    starts: Vec<u32>,
    bytes: Vec<u8>,
    /// token lengths by id, small enough to stay in cache
    lens: Vec<u8>,
    /// a blocked Bloom filter over the pairs, so most pairs that are no merge
    /// are told apart without touching the pair table
    bloom: Vec<u64>,
    bloom_shift: u32,
    /// a blocked Bloom filter over the token strings of three bytes or more,
    /// so most lengths the length masks allow are ruled out without a lookup
    tok_bloom: Vec<u64>,
    tok_bloom_shift: u32,
    /// the longest token that is a proper prefix of each token, or NONE
    next_prefix: Vec<u32>,
    /// the two tokens each token was merged from; a token that is no merge
    /// splits into itself
    split: Vec<(u32, u32)>,
    /// (a, b) -> the token the merge of a and b makes, one word per slot:
    /// a in bits 42..63, b in 21..42, the token in 0..21; EMPTY when unused
    pairs: Vec<u64>,
    pair_shift: u32,
    /// lengths (bit L - 3 for L in 3..=5) of tokens by hash of the first three bytes
    masks: Vec<u64>,
    mask_shift: u32,
    /// lengths (bit L - 6 for L in 6..=68, bit 63 for 69 and more) of tokens
    /// by hash of the first six bytes
    masks6: Vec<u64>,
    mask6_shift: u32,
    max_len: usize,
}

#[inline(always)]
fn mix(x: u64, shift: u32) -> usize {
    (x.wrapping_mul(0x9E3779B97F4A7C15) >> shift) as usize
}

#[inline(always)]
fn key3(s: &[u8]) -> u64 {
    s[0] as u64 | (s[1] as u64) << 8 | (s[2] as u64) << 16
}

#[inline(always)]
fn key6(s: &[u8]) -> u64 {
    u32::from_le_bytes([s[0], s[1], s[2], s[3]]) as u64 | (s[4] as u64) << 32 | (s[5] as u64) << 40
}

fn pow2_at_least(n: usize) -> usize {
    let mut s = 16;
    while s < n {
        s <<= 1;
    }
    s
}

impl Backtrack {
    /// Builds the tables for the vocabulary, or returns None when its ids are
    /// not dense or some token does not encode to itself, which the method
    /// needs. `verify` checks the latter; the shipped vocabularies skip it,
    /// since a unit test checks them once.
    pub fn new(v: &Vocab, tokens: &[(&[u8], u32)], verify: bool) -> Option<Backtrack> {
        let n = tokens.len();
        if n >= ID_MASK as usize {
            return None;
        }
        let mut by_id: Vec<Option<&[u8]>> = vec![None; n];
        for &(k, t) in tokens {
            if t as usize >= n || by_id[t as usize].is_some() || k.is_empty() {
                return None;
            }
            by_id[t as usize] = Some(k);
        }
        let mut starts = Vec::with_capacity(n + 1);
        let mut bytes = Vec::new();
        starts.push(0);
        let mut n3 = 0;
        let mut n6 = 0;
        let mut max_len = 0;
        for t in &by_id {
            let t = (*t)?;
            max_len = max_len.max(t.len());
            if t.len() >= 3 {
                n3 += 1;
            }
            if t.len() >= 6 {
                n6 += 1;
            }
            if t.len() > 255 {
                return None;
            }
            bytes.extend_from_slice(t);
            starts.push(bytes.len() as u32);
        }
        let msize = pow2_at_least(2 * n3);
        let m6size = pow2_at_least(2 * n6);
        let lens = by_id.iter().map(|t| t.unwrap().len() as u8).collect();
        let bsize = pow2_at_least(n / 4);
        let mut bt = Backtrack {
            starts,
            bytes,
            lens,
            bloom: vec![0; bsize],
            bloom_shift: 64 - bsize.trailing_zeros(),
            tok_bloom: vec![0; pow2_at_least(n / 4)],
            tok_bloom_shift: 64 - pow2_at_least(n / 4).trailing_zeros(),
            next_prefix: Vec::new(),
            split: Vec::with_capacity(n),
            pairs: Vec::new(),
            pair_shift: 64,
            masks: vec![0; msize],
            masks6: vec![0; m6size],
            mask6_shift: 64 - m6size.trailing_zeros(),
            max_len: 0,
            mask_shift: 64 - msize.trailing_zeros(),
        };
        bt.max_len = max_len;
        for t in &by_id {
            let t = t.unwrap();
            if t.len() >= 3 {
                let (w, b) = bt.tok_bits(t);
                bt.tok_bloom[w] |= b;
            }
            if (3..6).contains(&t.len()) {
                let i = mix(key3(t), bt.mask_shift);
                bt.masks[i] |= 1 << (t.len() - 3);
            }
            if t.len() >= 6 {
                let i = mix(key6(t), bt.mask6_shift);
                bt.masks6[i] |= 1 << (t.len() - 6).min(63);
            }
        }
        bt.next_prefix = (0..n).map(|id| {
            let t = bt.token(id as u32);
            bt.longest(v, &t[..t.len() - 1]).unwrap_or(NONE)
        }).collect();
        let psize = pow2_at_least(2 * n);
        bt.pairs = vec![EMPTY; psize];
        bt.pair_shift = 64 - psize.trailing_zeros();
        // reverse engineer the merges, lowest id first, as the bpe crate does
        for id in 0..n as u32 {
            let t = bt.token(id);
            let mut a = bt.next_prefix[id as usize];
            let mut found = false;
            while a != NONE {
                let rest = &t[bt.len(a)..];
                let b = v.lookup(rest);
                if b != NO_TOKEN && a < id && (b as u32) < id && bt.valid_pair(a, b as u32) {
                    bt.insert_pair(a, b as u32, id);
                    bt.split.push((a, b as u32));
                    found = true;
                    break;
                }
                a = bt.next_prefix[a as usize];
            }
            if !found {
                bt.split.push((id, id));
            }
        }
        // the method assumes every token encodes to itself
        if !verify {
            return Some(bt);
        }
        let mut out = Vec::new();
        let mut bits = Vec::new();
        for id in 0..n as u32 {
            out.clear();
            let t = bt.token(id).to_vec();
            bt.encode(v, &t, &mut out, &mut bits);
            if out.len() != 1 || out[0] != id {
                return None;
            }
        }
        Some(bt)
    }

    #[inline(always)]
    fn token(&self, id: u32) -> &[u8] {
        &self.bytes[self.starts[id as usize] as usize..self.starts[id as usize + 1] as usize]
    }

    #[inline(always)]
    fn len(&self, id: u32) -> usize {
        self.lens[id as usize] as usize
    }

    /// Word and bits of the token filter for the bytes s, at least three long.
    #[inline(always)]
    fn tok_bits(&self, s: &[u8]) -> (usize, u64) {
        let k = if s.len() <= 8 { pack8(s) } else { long_hash(s) as u64 ^ (pack8(&s[..8]) << 1) };
        let h = (k ^ (s.len() as u64) << 58).wrapping_mul(0x9E3779B97F4A7C15);
        let h = h ^ (h >> 29);
        ((h >> self.tok_bloom_shift) as usize, 1u64 << (h & 63) | 1u64 << ((h >> 6) & 63))
    }

    /// The token of s, at least three bytes long, or NO_TOKEN; the filter
    /// answers most strings that are no token.
    #[inline(always)]
    fn token_of(&self, v: &Vocab, s: &[u8]) -> i32 {
        let (w, b) = self.tok_bits(s);
        if self.tok_bloom[w] & b != b {
            return NO_TOKEN;
        }
        v.lookup(s)
    }

    #[inline(always)]
    fn bloom_bits(&self, key: u64) -> (usize, u64) {
        let h = key.wrapping_mul(0xD6E8FEB86659FD93);
        let word = (h >> self.bloom_shift) as usize;
        (word, 1u64 << (h & 63) | 1u64 << ((h >> 6) & 63))
    }

    fn insert_pair(&mut self, a: u32, b: u32, id: u32) {
        let key = (a as u64) << ID_BITS | b as u64;
        let mask = self.pairs.len() - 1;
        let mut i = mix(key, self.pair_shift);
        while self.pairs[i] != EMPTY {
            i = (i + 1) & mask;
        }
        self.pairs[i] = key << ID_BITS | id as u64;
        let (w, b) = self.bloom_bits(key);
        self.bloom[w] |= b;
    }

    #[inline(always)]
    fn pair(&self, a: u32, b: u32) -> u32 {
        let key = (a as u64) << ID_BITS | b as u64;
        let (w, bits) = self.bloom_bits(key);
        if self.bloom[w] & bits != bits {
            return NONE;
        }
        let mask = self.pairs.len() - 1;
        let mut i = mix(key, self.pair_shift);
        loop {
            let e = self.pairs[i];
            if e >> ID_BITS == key {
                return (e & ID_MASK) as u32;
            }
            if e == EMPTY {
                return NONE;
            }
            i = (i + 1) & mask;
        }
    }

    /// Whether byte-pair encoding the bytes of a followed by those of b gives a, b.
    fn valid_pair(&self, mut a: u32, mut b: u32) -> bool {
        let mut limit = u32::MAX;
        loop {
            let c = self.pair(a, b);
            if c != NONE && c < limit {
                return false;
            }
            if a > b {
                limit = a;
                a = self.split[a as usize].1;
                if a == limit {
                    limit = b + 1;
                    b = self.split[b as usize].0;
                    if b + 1 == limit {
                        return true;
                    }
                }
            } else {
                limit = b + 1;
                b = self.split[b as usize].0;
                if b + 1 == limit {
                    limit = a;
                    a = self.split[a as usize].1;
                    if a == limit {
                        return true;
                    }
                }
            }
        }
    }

    /// The longest token that is a prefix of s, or None when s is empty.
    #[inline]
    fn longest(&self, v: &Vocab, s: &[u8]) -> Option<u32> {
        if s.is_empty() {
            return None;
        }
        if s.len() >= 6 {
            let mut m = self.masks6[mix(key6(s), self.mask6_shift)];
            if m >> 63 != 0 {
                // a token of 69 bytes or more may start here: try those lengths one by one
                for l in (69..=s.len().min(self.max_len)).rev() {
                    let t = v.lookup(&s[..l]);
                    if t != NO_TOKEN {
                        return Some(t as u32);
                    }
                }
                m &= !(1 << 63);
            }
            let cap = s.len() - 5; // lengths 6..=s.len() are bits 0..cap
            if cap < 63 {
                m &= (1u64 << cap) - 1;
            }
            while m != 0 {
                let bit = 63 - m.leading_zeros() as usize;
                let t = self.token_of(v, &s[..bit + 6]);
                if t != NO_TOKEN {
                    return Some(t as u32);
                }
                m &= !(1u64 << bit);
            }
        }
        if s.len() >= 3 {
            let mut m = self.masks[mix(key3(s), self.mask_shift)];
            let cap = (s.len() - 2).min(3); // lengths 3..=5 are bits 0..3
            m &= (1u64 << cap) - 1;
            while m != 0 {
                let bit = 63 - m.leading_zeros() as usize;
                let t = self.token_of(v, &s[..bit + 3]);
                if t != NO_TOKEN {
                    return Some(t as u32);
                }
                m &= !(1u64 << bit);
            }
        }
        if s.len() >= 2 {
            let t = v.byte_pair[(s[0] as usize) << 8 | s[1] as usize];
            if t != NO_TOKEN {
                return Some(t as u32);
            }
        }
        Some(v.byte_tok[s[0] as usize] as u32)
    }

    /// Appends the byte-pair encoding of s to out. bits is scratch space.
    pub fn encode(&self, v: &Vocab, s: &[u8], out: &mut Vec<u32>, bits: &mut Vec<u64>) {
        let n = s.len();
        let start = out.len();
        bits.clear();
        bits.resize(n / 64 + 1, u64::MAX);
        let mut pos = 0usize;
        let mut next = self.longest(v, s);
        while let Some(mut token) = next {
            let last = if out.len() > start { Some(out[out.len() - 1]) } else { None };
            loop {
                let end = pos + self.len(token);
                let open = bits[end >> 6] & (1 << (end & 63)) != 0;
                if open && last.map_or(true, |l| self.valid_pair(l, token)) {
                    out.push(token);
                    pos = end;
                    next = self.longest(v, &s[end..]);
                    break;
                }
                let shorter = self.next_prefix[token as usize];
                if shorter != NONE {
                    token = shorter;
                } else {
                    bits[pos >> 6] &= !(1 << (pos & 63));
                    out.pop();
                    pos -= last.map_or(0, |t| self.len(t));
                    next = last;
                    break;
                }
            }
        }
    }

    /// Bytes the tables take.
    pub fn heap_bytes(&self) -> usize {
        self.starts.len() * 4 + self.bytes.len() + self.lens.len() + self.bloom.len() * 8 + self.tok_bloom.len() * 8 + self.next_prefix.len() * 4 + self.split.len() * 8 + self.pairs.len() * 8 + self.masks.len() * 8 + self.masks6.len() * 8
    }
}
