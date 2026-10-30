//! A faster encoder for OpenAI's tiktoken vocabularies. Same token ids as the
//! reference core for the shipped encodings; see the README for the numbers.

#[doc(hidden)]
pub mod cjk;
#[doc(hidden)]
mod backtrack;
pub mod merge;
#[doc(hidden)]
pub mod scan;
#[doc(hidden)]
pub mod segment;
#[doc(hidden)]
pub mod unicode;
#[doc(hidden)]
pub mod vocab;

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use merge::Cache;
use scan::Kind;
use segment::{cut_bits, key16_at, key32_at, BigMemo, SegCache, BIG_MAX, LONG_MAX, SEG_MAX};
use vocab::{Vocab, NO_TOKEN};

pub type Rank = u32;

/// Runs of three-byte characters up to this many bytes, a Korean word say,
/// take one cached merge of the whole piece instead of the cut logic.
const SHORT_RUN: usize = 12;

#[derive(Debug)]
pub enum Error {
    UnknownPattern(String),
    BadVocabulary(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownPattern(p) => write!(f, "no scanner for pattern {p:?}"),
            Error::BadVocabulary(m) => write!(f, "bad vocabulary: {m}"),
        }
    }
}
impl std::error::Error for Error {}

pub struct CoreBPE {
    vocab: Vocab,
    cjk: cjk::CjkCuts,
    /// one word per code point of the three-byte range: bits 0-1 the token
    /// count of the character's byte-pair merge (0 when it does not pack),
    /// bit 2 set when the character is also a vocabulary token other than its
    /// merge, three 18-bit token ids from bit 3, the span class in bits 57-59,
    /// the crossing flags in bits 60-63
    char_info: Vec<u64>,
    kind: Kind,
    decoder: Vec<(u32, u32)>, // token -> (offset, len) into bytes
    bytes: Vec<u8>,
    specials: Vec<(String, Rank)>,
    special_decoder: HashMap<Rank, String>,
    pool: Mutex<Vec<Box<Caches>>>,
}

/// What one caller borrows from the pool: the merge caches and the segment memo.
pub struct Caches {
    pieces: Cache,
    segments: SegCache,
    big: BigMemo,
    bits: Vec<u64>,
    /// the size step of the tables: 0 none, 1 tiny, 2 small, 3 full
    level: u8,
    /// bytes still to encode before the tables grow a step
    until_next: usize,
}

/// Input encoded at each size step before the tables grow to the next.
const STEP_BYTES: [usize; 3] = [512, 4 << 10, 60 << 10];

impl Caches {
    fn new() -> Box<Caches> {
        Box::new(Caches { pieces: Cache::small(), segments: SegCache::small(), big: BigMemo::at_level(0), bits: Vec::new(), level: 0, until_next: STEP_BYTES[0] })
    }
}

impl CoreBPE {
    /// A core for `encoder` and `pattern`. The pattern must be one of the three
    /// the shipped encodings use; anything else is an error, not a slow path.
    pub fn new(encoder: HashMap<Vec<u8>, Rank>, special_tokens: HashMap<String, Rank>, pattern: &str) -> Result<CoreBPE, Error> {
        let mut tokens: Vec<(&[u8], Rank)> = encoder.iter().map(|(k, &t)| (k.as_slice(), t)).collect();
        tokens.sort_unstable_by_key(|&(_, t)| t);
        Self::build(&tokens, special_tokens, pattern, true)
    }

    /// `new` for a shipped vocabulary, read straight from its file. `verify`
    /// is false for those whose tokens are known to encode to themselves
    /// (r50k_base, cl100k_base, o200k_base; checked by
    /// `shipped_vocabularies_verify`), so the check is not run again.
    fn shipped(data: &str, special_tokens: HashMap<String, Rank>, pattern: &str, verify: bool) -> Result<CoreBPE, Error> {
        let (arena, spans) = parse_tiktoken_flat(data);
        let tokens: Vec<(&[u8], Rank)> = spans.iter().map(|&(a, b, t)| (&arena[a as usize..b as usize], t)).collect();
        Self::build(&tokens, special_tokens, pattern, verify)
    }

    /// The core from (bytes, rank) pairs, sorted by rank.
    fn build(tokens: &[(&[u8], Rank)], special_tokens: HashMap<String, Rank>, pattern: &str, verify: bool) -> Result<CoreBPE, Error> {
        let kind = scan::kind_for(pattern).ok_or_else(|| Error::UnknownPattern(pattern.to_string()))?;
        let max = tokens.iter().map(|&(_, t)| t).max().unwrap_or(0) as usize;
        let mut decoder = vec![(0u32, u32::MAX); max + 1];
        let mut bytes = Vec::with_capacity(tokens.iter().map(|(k, _)| k.len()).sum::<usize>() + 8);
        let mut single = [false; 256];
        for &(k, t) in tokens {
            if decoder[t as usize].1 != u32::MAX {
                return Err(Error::BadVocabulary(format!("token {t} has two byte strings")));
            }
            decoder[t as usize] = (bytes.len() as u32, k.len() as u32);
            bytes.extend_from_slice(k);
            if k.len() == 1 {
                single[k[0] as usize] = true;
            }
        }
        if let Some(b) = (0..256).find(|&b| !single[b]) {
            return Err(Error::BadVocabulary(format!("byte {b:#04x} has no token")));
        }
        let mut vocab = Vocab::new(tokens, verify);
        let cjk = cjk::CjkCuts::new(tokens);
        let span_cls = cjk::span_classes(kind);
        let mut char_info = vec![0u64; 0x10000];
        {
            let mut scratch = Cache::new();
            let mut tmp: Vec<Rank> = Vec::with_capacity(8);
            for cp in 0x800u32..0x10000 {
                let Some(ch) = char::from_u32(cp) else { continue };
                let mut b = [0u8; 4];
                let c = ch.encode_utf8(&mut b).as_bytes();
                tmp.clear();
                if vocab.self_encoding {
                    // every token encodes to itself here, and a three-byte
                    // string that is no token merges at most one byte pair:
                    // the lower-ranked of the two, or the left one on a tie
                    let t = vocab.lookup(c);
                    if t != NO_TOKEN {
                        tmp.push(t as Rank);
                    } else {
                        let r01 = vocab.byte_pair[(c[0] as usize) << 8 | c[1] as usize];
                        let r12 = vocab.byte_pair[(c[1] as usize) << 8 | c[2] as usize];
                        let tok = |b: u8| vocab.byte_tok[b as usize] as Rank;
                        if r01 != NO_TOKEN && (r12 == NO_TOKEN || r01 <= r12) {
                            tmp.extend_from_slice(&[r01 as Rank, tok(c[2])]);
                        } else if r12 != NO_TOKEN {
                            tmp.extend_from_slice(&[tok(c[0]), r12 as Rank]);
                        } else {
                            tmp.extend_from_slice(&[tok(c[0]), tok(c[1]), tok(c[2])]);
                        }
                    }
                    if cfg!(debug_assertions) {
                        let mut slow = Vec::new();
                        scratch.merge_into(&vocab, &mut slow, c);
                        debug_assert_eq!(slow, tmp, "U+{cp:04X}");
                    }
                } else {
                    scratch.merge_into(&vocab, &mut tmp, c);
                }
                let mut e = 0u64;
                if tmp.len() <= 3 && tmp.iter().all(|&t| t < 1 << 18) {
                    e = tmp.len() as u64;
                    for (k, &t) in tmp.iter().enumerate() {
                        e |= (t as u64) << (3 + 18 * k);
                    }
                }
                let t = vocab.lookup(c);
                if t != NO_TOKEN && (tmp.len() != 1 || tmp[0] != t as Rank) {
                    e |= 4;
                }
                e |= (span_cls[cp as usize] as u64 & 7) << 57;
                e |= (cjk.flag_of(cp as usize) as u64 & 15) << 60;
                char_info[cp as usize] = e;
            }
        }
        {
            // the merge of each character U+1F000..U+1FFFF, for lone emoji between cuts
            let mut scratch = Cache::new();
            let mut tmp: Vec<Rank> = Vec::with_capacity(8);
            let mut table = vec![0u64; 1 << 12];
            for (i, slot) in table.iter_mut().enumerate() {
                let Some(ch) = char::from_u32(0x1F000 + i as u32) else { continue };
                let mut b = [0u8; 4];
                tmp.clear();
                scratch.merge_into(&vocab, &mut tmp, ch.encode_utf8(&mut b).as_bytes());
                if !tmp.is_empty() && tmp.len() <= 3 && tmp.iter().all(|&t| t < 1 << 18) {
                    let mut e = tmp.len() as u64;
                    for (k, &t) in tmp.iter().enumerate() {
                        e |= (t as u64) << (3 + 18 * k);
                    }
                    *slot = e;
                }
            }
            vocab.emoji = table;
        }
        let mut specials: Vec<(String, Rank)> = special_tokens.iter().map(|(k, &v)| (k.clone(), v)).collect();
        specials.sort();
        let special_decoder = special_tokens.iter().map(|(k, &v)| (v, k.clone())).collect();
        // eight bytes of padding, so decode can copy any token of up to eight
        // bytes with one unaligned eight-byte load
        bytes.extend_from_slice(&[0u8; 8]);
        Ok(CoreBPE { vocab, cjk, char_info, kind, decoder, bytes, specials, special_decoder, pool: Mutex::new(Vec::new()) })
    }

    fn take_cache(&self) -> Box<Caches> {
        self.pool.lock().unwrap().pop().unwrap_or_else(Caches::new)
    }

    /// Whether pieces encode by backtracking. For benchmarks and tests.
    #[doc(hidden)]
    pub fn backtracking(&self) -> Option<usize> {
        self.vocab.bt().map(|b| b.heap_bytes())
    }

    /// Drops every pooled cache, so the next call starts cold. For benchmarks and tests.
    #[doc(hidden)]
    pub fn reset_caches(&self) {
        self.pool.lock().unwrap().clear();
    }

    fn put_cache(&self, c: Box<Caches>) {
        let mut p = self.pool.lock().unwrap();
        if p.len() < 64 {
            p.push(c);
        }
    }

    /// Encodes the pieces of s, appending to out. Spans of three-byte
    /// characters go to the fused span encoder, which hands back any piece it
    /// cannot finish on its own.
    fn encode_pieces(&self, cache: &mut Cache, out: &mut Vec<Rank>, s: &[u8]) {
        let fused = self.kind != Kind::P50k;
        let mut i = 0;
        while i < s.len() {
            if fused && s[i] & 0xF0 == 0xE0 {
                let k = self.encode_span(cache, out, s, i);
                if k > i {
                    i = k;
                    continue;
                }
            }
            let end = scan::scan(self.kind, s, i);
            if end - i == 3 && s[i] & 0xF0 == 0xE0 {
                self.encode_char3(cache, out, &s[i..end]);
            } else if end - i > SHORT_RUN && s[i] & 0xF0 == 0xE0 && cjk::all_three_byte(&s[i..end]) {
                // a run of three-byte characters: if the run is itself a token,
                // that token blocks every internal cut and the run encoder looks
                // the whole run up
                self.encode_cjk_run(cache, out, &s[i..end]);
            } else if end - i > SHORT_RUN + 1 && s[i] < 0x80 && s[i + 1] & 0xF0 == 0xE0 && cjk::all_three_byte(&s[i + 1..end]) {
                // the same, after a one-byte prefix such as a space
                self.encode_cjk_run_from(cache, out, &s[i..end], 1);
            } else {
                let t = self.vocab.lookup_at(s, i, end);
                if t != NO_TOKEN {
                    out.push(t as Rank);
                } else {
                    cache.merge(&self.vocab, out, &s[i..end]);
                }
            }
            i = end;
        }
    }

    /// Emits a one-character part with info word e.
    #[inline(always)]
    fn emit_char_part(&self, cache: &mut Cache, out: &mut Vec<Rank>, c: &[u8], e: u64, whole: bool) {
        if (whole && e & 4 != 0) || e & 3 == 0 {
            if whole {
                let t = self.vocab.lookup(c);
                if t != NO_TOKEN {
                    out.push(t as Rank);
                    return;
                }
            }
            cache.merge_into(&self.vocab, out, c);
            out.reserve(8);
        } else {
            // SAFETY: the span encoder keeps three free slots
            unsafe { Self::put_char3(out, e) };
        }
    }

    /// Emits the part s[a..b) of two or more characters: its byte-pair merge,
    /// or the vocabulary match when the part is the whole piece.
    #[inline(never)]
    fn emit_multi_part(&self, cache: &mut Cache, out: &mut Vec<Rank>, s: &[u8], a: usize, b: usize, whole: bool) {
        let part = &s[a..b];
        if whole {
            let t = if part.len() <= 8 {
                self.vocab.lookup_at(s, a, b)
            } else if self.cjk.may_be_token(part) {
                self.vocab.lookup(part)
            } else {
                NO_TOKEN
            };
            if t != NO_TOKEN {
                out.push(t as Rank);
                out.reserve(8);
                return;
            }
        }
        cache.merge_part(&self.vocab, out, part);
        out.reserve(8);
    }

    /// Whether the boundary at p, between characters with info words e1 and
    /// e2, is a safe cut: the flags decide most boundaries, then the caller's
    /// pair verdict cache, then the exact test.
    #[inline(always)]
    fn safe_cut_info(&self, cache: &mut Cache, s: &[u8], p: usize, cp1: usize, cp2: usize, e1: u64, e2: u64) -> bool {
        if !self.cjk.needs_check_flags((e1 >> 60) as u8, (e2 >> 60) as u8, cp1, cp2) {
            return true;
        }
        let key = ((cp1 as u64) << 16) | cp2 as u64;
        let slot = ((key.wrapping_mul(0x9E3779B97F4A7C15)) >> cache.cjk_shift) as usize;
        let e = cache.cjk_pairs[slot];
        if e & (1 << 33) != 0 && e as u32 as u64 == key {
            return e & (1 << 32) == 0;
        }
        let b = self.cjk.blocked_cp(cp1, cp2, &s[p - 3..p], &s[p..p + 3]);
        cache.cjk_pairs[slot] = key | (1 << 33) | ((b as u64) << 32);
        !b
    }

    /// Encodes the pieces of the span of three-byte characters that starts at
    /// i, a piece start, in one pass, and returns where it stopped: the end
    /// of what it encoded, or the start of the first piece it could not
    /// settle from the span alone (one starting at whitespace or a cased
    /// letter, or one whose end depends on what follows the span in a way
    /// handled only by the scanner). Output for such a piece is undone. The
    /// piece rules are the pattern's alternatives restricted to three-byte
    /// characters: a letter run with an optional punctuation prefix, up to
    /// three numbers, or a punctuation run. Inside a piece, parts are cut
    /// where no token can span the boundary.
    fn encode_span(&self, cache: &mut Cache, out: &mut Vec<Rank>, s: &[u8], i: usize) -> usize {
        cache.ensure_chars();
        use cjk::{C_LETTER, C_MARK, C_NUM, C_OTHER, C_PUNCT};
        const LET: u8 = 0;
        const NUM: u8 = 1;
        const PUN: u8 = 2;
        let mut j = i;
        while j < s.len() && s[j] & 0xF0 == 0xE0 {
            j += 3;
        }
        let o200k = self.kind == Kind::O200k;
        // SAFETY: callers pass q < j, a whole three-byte character
        let cp = |q: usize| -> usize { cjk::cp3(unsafe { s.get_unchecked(q..q + 3) }) };
        // SAFETY: code points of three-byte characters are below 0x10000
        let info = |c: usize| -> u64 { unsafe { *self.char_info.get_unchecked(c) } };
        let cls = |e: u64| -> u8 { ((e >> 57) & 7) as u8 };
        let in_letters = |c: u8| c == C_LETTER || (o200k && c == C_MARK);
        let in_punct = |c: u8| c == C_PUNCT || (o200k && c == C_MARK);
        // every character adds at most three tokens; keep eight spare
        out.reserve(j - i + 8);
        let mut q = i;
        while q < j {
            let cp_q = cp(q);
            let e_q = info(cp_q);
            let c = cls(e_q);
            let kind;
            let mut prefix = false;
            if in_letters(c) {
                kind = LET;
            } else if c == C_NUM {
                kind = NUM;
            } else if in_punct(c) {
                if q + 3 < j {
                    let c2 = cls(info(cp(q + 3)));
                    if in_letters(c2) {
                        kind = LET;
                        prefix = true;
                    } else if o200k && c2 == C_OTHER {
                        return q;
                    } else {
                        kind = PUN;
                    }
                } else {
                    kind = PUN;
                }
            } else {
                return q;
            }
            let mark = out.len();
            let mut part = q;
            let mut cut_inside = false;
            let mut prev = cp_q;
            let mut e_prev = e_q;
            let mut p = q + 3;
            let mut count = 1;
            let end;
            loop {
                if p == j {
                    match s.get(j) {
                        None => {}
                        Some(&b) if b >= 0x80 => {
                            out.truncate(mark);
                            return q;
                        }
                        Some(&b) => {
                            let fall = match kind {
                                LET => b.is_ascii_alphabetic() || (o200k && b == b'\''),
                                NUM => b.is_ascii_digit() && count < 3,
                                _ => (p == q + 3 && b.is_ascii_alphabetic()) || (!b.is_ascii_alphanumeric() && !b.is_ascii_whitespace()),
                            };
                            if fall {
                                out.truncate(mark);
                                return q;
                            }
                            let tail = |x: u8| x == b'\r' || x == b'\n' || (o200k && x == b'/');
                            if kind == PUN && tail(b) {
                                out.truncate(mark);
                                let mut e = j;
                                while e < s.len() && tail(s[e]) {
                                    e += 1;
                                }
                                let t = self.vocab.lookup_at(s, q, e);
                                if t != NO_TOKEN {
                                    out.push(t as Rank);
                                } else {
                                    cache.merge(&self.vocab, out, &s[q..e]);
                                }
                                return e;
                            }
                        }
                    }
                    end = j;
                    break;
                }
                let cp_p = cp(p);
                let e_p = info(cp_p);
                let c2 = cls(e_p);
                let continues = match kind {
                    LET => in_letters(c2) || (prefix && p == q + 3),
                    NUM => c2 == C_NUM && count < 3,
                    _ => in_punct(c2),
                };
                if !continues {
                    if o200k && kind == LET && c2 == C_OTHER {
                        out.truncate(mark);
                        return q;
                    }
                    end = p;
                    break;
                }
                if self.safe_cut_info(cache, s, p, prev, cp_p, e_prev, e_p) {
                    if p - part == 3 {
                        self.emit_char_part(cache, out, &s[part..p], e_prev, false);
                    } else {
                        self.emit_multi_part(cache, out, s, part, p, false);
                    }
                    cut_inside = true;
                    part = p;
                }
                prev = cp_p;
                e_prev = e_p;
                p += 3;
                count += 1;
            }
            if end - part == 3 {
                self.emit_char_part(cache, out, &s[part..end], e_prev, !cut_inside);
            } else {
                self.emit_multi_part(cache, out, s, part, end, !cut_inside);
            }
            q = end;
        }
        q
    }

    /// The byte-pair merge of one three-byte character, packed as in
    /// `char_info` (its low 57 bits); a token count of 0 means it does not pack.
    #[inline(always)]
    fn char3_entry(&self, c: &[u8]) -> u64 {
        // SAFETY: cp3 of a three-byte character is below 0x10000 = char_info.len()
        unsafe { *self.char_info.get_unchecked(cjk::cp3(c)) & ((1 << 57) - 1) }
    }

    /// Writes the tokens of a packed character entry at the end of out,
    /// which must have room for three more.
    #[inline(always)]
    unsafe fn put_char3(out: &mut Vec<Rank>, e: u64) {
        let dst = out.as_mut_ptr().add(out.len());
        *dst = ((e >> 3) & 0x3FFFF) as u32;
        *dst.add(1) = ((e >> 21) & 0x3FFFF) as u32;
        *dst.add(2) = ((e >> 39) & 0x3FFFF) as u32;
        out.set_len(out.len() + (e & 3) as usize);
    }

    /// Appends the tokens of one three-byte character that is a whole piece.
    #[inline(always)]
    fn encode_char3(&self, cache: &mut Cache, out: &mut Vec<Rank>, c: &[u8]) {
        cache.ensure_chars();
        let e = self.char3_entry(c);
        if e & 4 != 0 || e & 3 == 0 {
            // a vocabulary match that differs from the merge, or an entry too
            // large to pack: the slow, exact way
            let t = self.vocab.lookup(c);
            if t != NO_TOKEN {
                out.push(t as Rank);
            } else {
                cache.merge_into(&self.vocab, out, c);
            }
        } else {
            out.reserve(3);
            // SAFETY: three slots reserved
            unsafe { Self::put_char3(out, e) };
        }
    }

    /// Encodes a run of three-byte characters that missed the vocabulary,
    /// cut wherever no token can span the boundary. Single characters, the
    /// usual part, come from a per-code-point table.
    #[inline(never)]
    fn encode_cjk_run(&self, cache: &mut Cache, out: &mut Vec<Rank>, s: &[u8]) {
        cache.ensure_chars();
        self.encode_cjk_run_from(cache, out, s, 0)
    }

    /// encode_cjk_run for a piece whose first `off` bytes (0 or 1, an ASCII
    /// prefix) come before the run of three-byte characters. The prefix is
    /// never cut from the first character: not cutting is always exact.
    fn encode_cjk_run_from(&self, cache: &mut Cache, out: &mut Vec<Rank>, s: &[u8], off: usize) {
        cache.ensure_chars();
        let n = s.len();
        // at most three tokens per character, and n / 3 characters
        out.reserve(n + 8);
        let mut start = 0;
        let mut cp_prev = cjk::cp3(&s[off..off + 3]);
        let mut p = off + 3;
        loop {
            let (safe, cp_next) = if p == n {
                (true, 0)
            } else {
                let cp = cjk::cp3(&s[p..p + 3]);
                let safe = if !self.cjk.needs_check(cp_prev, cp) {
                    true
                } else {
                    let key = ((cp_prev as u64) << 16) | cp as u64;
                    let slot = ((key.wrapping_mul(0x9E3779B97F4A7C15)) >> cache.cjk_shift) as usize;
                    let e = cache.cjk_pairs[slot];
                    if e & (1 << 33) != 0 && e as u32 as u64 == key {
                        e & (1 << 32) == 0
                    } else {
                        let b = self.cjk.blocked_cp(cp_prev, cp, &s[p - 3..p], &s[p..p + 3]);
                        cache.cjk_pairs[slot] = key | (1 << 33) | ((b as u64) << 32);
                        !b
                    }
                };
                (safe, cp)
            };
            if safe {
                if start == 0 && p == n {
                    // no cut: the run is a whole piece, a vocabulary match wins
                    let t = if self.cjk.may_be_token(s) || n <= 8 { self.vocab.lookup(s) } else { NO_TOKEN };
                    if t != NO_TOKEN {
                        out.push(t as Rank);
                    } else {
                        cache.merge(&self.vocab, out, s);
                    }
                } else if p - start == 3 {
                    let e = self.char3_entry(&s[start..p]);
                    if e & 3 != 0 {
                        // SAFETY: n + 8 slots were reserved and each character adds at most three
                        unsafe { Self::put_char3(out, e) };
                    } else {
                        cache.merge_into(&self.vocab, out, &s[start..p]);
                        out.reserve(n + 8);
                    }
                } else {
                    // a part of a larger piece: its byte-pair merge
                    cache.merge_part(&self.vocab, out, &s[start..p]);
                    out.reserve(n + 8);
                }
                start = p;
            }
            if p == n {
                break;
            }
            cp_prev = cp_next;
            p += 3;
        }
    }

    /// Whether text may be cut before byte p without changing any piece: the
    /// character before is a letter and the one at p is punctuation that no
    /// pattern attaches to a preceding word (so not an apostrophe and not a
    /// combining mark), or p is a single space between two ASCII letters.
    fn safe_cut(text: &str, p: usize) -> bool {
        let s = text.as_bytes();
        if p == 0 || p >= s.len() || !text.is_char_boundary(p) {
            return false;
        }
        let prev = text[..p].chars().next_back().unwrap();
        let cur = text[p..].chars().next().unwrap();
        if cur == ' ' {
            return prev.is_ascii_alphabetic() && s.get(p + 1).map_or(false, |c| c.is_ascii_alphabetic());
        }
        let prev_letter = if prev.is_ascii() { prev.is_ascii_alphabetic() } else { unicode::classes().letter.has(prev as u32) };
        if !prev_letter {
            return false;
        }
        if cur.is_ascii() {
            return cur != '\'' && !cur.is_ascii_alphanumeric() && !cur.is_ascii_whitespace();
        }
        matches!(cur, '\u{3002}' | '\u{ff0c}' | '\u{3001}' | '\u{ff01}' | '\u{ff1f}' | '\u{ff1b}' | '\u{ff1a}' | '\u{ff08}' | '\u{ff09}' | '\u{300c}' | '\u{300d}' | '\u{300e}' | '\u{300f}' | '\u{300a}' | '\u{300b}' | '\u{2026}' | '\u{201c}' | '\u{201d}' | '\u{2018}' | '\u{2019}')
            && !matches!(cur, '\u{2019}')
    }

    /// encode_ordinary with the text cut into chunks that encode on rayon's
    /// thread pool. Cuts are made only where `safe_cut` holds, so the ids
    /// equal those of the single-threaded call. `chunk` is the target chunk
    /// size in bytes; the call falls back to one thread below two chunks.
    pub fn encode_ordinary_parallel(&self, text: &str, chunk: usize) -> Vec<Rank> {
        use rayon::prelude::*;
        let chunk = chunk.max(4096);
        if text.len() < 2 * chunk {
            return self.encode_ordinary(text);
        }
        let mut cuts = vec![0usize];
        let mut target = chunk;
        while target < text.len() {
            let mut p = target.max(*cuts.last().unwrap() + 1);
            while p < text.len() && !Self::safe_cut(text, p) {
                p += 1;
            }
            if p >= text.len() {
                break;
            }
            cuts.push(p);
            target = p + chunk;
        }
        cuts.push(text.len());
        let parts: Vec<Vec<Rank>> = cuts
            .par_windows(2)
            .map(|w| {
                let s = &text.as_bytes()[w[0]..w[1]];
                let mut out = Vec::with_capacity(s.len() / 4 + 16);
                let mut cache = self.take_cache();
                self.encode_segments(&mut cache, &mut out, s);
                self.put_cache(cache);
                out
            })
            .collect();
        let mut out = Vec::with_capacity(parts.iter().map(|p| p.len()).sum());
        for p in parts {
            out.extend_from_slice(&p);
        }
        out
    }

    /// Encodes text with no special-token handling: special tokens in the text
    /// are encoded as ordinary text.
    pub fn encode_ordinary(&self, text: &str) -> Vec<Rank> {
        let mut out = Vec::with_capacity(text.len() / 4 + 16);
        let mut cache = self.take_cache();
        self.encode_segments(&mut cache, &mut out, text.as_bytes());
        self.put_cache(cache);
        out
    }

    /// Encodes s segment by segment through the segment memo; a segment the
    /// memo does not hold goes through the piece path and is remembered.
    fn encode_segments(&self, caches: &mut Caches, out: &mut Vec<Rank>, s: &[u8]) {
        let n = s.len();
        let Caches { pieces, segments, big, bits, level, until_next, .. } = caches;
        // the tables grow a step at the first segment that starts at or past grow_at
        let mut grow_at = if *level >= 3 { usize::MAX } else { *until_next };
        cut_bits(s, bits);
        let mut i = 0;
        let mut wi = 0usize;
        let mut w = if bits.is_empty() { 0 } else { bits[0] };
        while i < n {
            if i >= grow_at {
                *level += 1;
                pieces.grow_to(*level);
                *segments = SegCache::at_level(*level);
                *big = BigMemo::at_level(*level);
                grow_at = if *level >= 3 { usize::MAX } else { i + STEP_BYTES[*level as usize] };
            }
            let end = loop {
                if w != 0 {
                    let q = wi * 64 + w.trailing_zeros() as usize;
                    w &= w - 1;
                    break q.min(n);
                }
                wi += 1;
                if wi >= bits.len() {
                    break n;
                }
                w = bits[wi];
            };
            if *level == 0 {
                self.encode_pieces(pieces, out, &s[i..end]);
            } else if end - i <= SEG_MAX {
                let (k0, k1) = key16_at(s, i, end);
                out.reserve(8);
                // SAFETY: eight free slots were just reserved; write_at writes five
                let got = unsafe { segments.write_at(k0, k1, out.as_mut_ptr().add(out.len())) };
                if got != 0 {
                    unsafe { out.set_len(out.len() + got) };
                } else {
                    let before = out.len();
                    self.encode_pieces(pieces, out, &s[i..end]);
                    let ntok = out.len() - before;
                    if ntok <= 5 {
                        let mut a = [0u32; 5];
                        a[..ntok].copy_from_slice(&out[before..]);
                        segments.put(k0, k1, &a[..ntok]);
                    }
                }
            } else if end - i <= LONG_MAX {
                let k = key32_at(s, i, end);
                if !segments.append_long(&k, out) {
                    let before = out.len();
                    self.encode_pieces(pieces, out, &s[i..end]);
                    let ntok = out.len() - before;
                    if ntok <= 8 {
                        let mut a = [0u32; 8];
                        a[..ntok].copy_from_slice(&out[before..]);
                        segments.put_long(&k, &a[..ntok]);
                    }
                }
            } else if end - i <= BIG_MAX && !big.is_empty() {
                let seg = &s[i..end];
                let h = BigMemo::hash(seg);
                if !big.append(h, seg, out) {
                    let before = out.len();
                    self.encode_pieces(pieces, out, seg);
                    big.note(h, seg, &out[before..]);
                }
            } else {
                self.encode_pieces(pieces, out, &s[i..end]);
            }
            i = end;
        }
        if *level < 3 {
            *until_next = grow_at.saturating_sub(n).max(1);
        }
    }

    /// Encodes text; the special tokens in `allowed_special` become their ids
    /// and every other special token in the text is encoded as ordinary text,
    /// as the reference core does.
    pub fn encode(&self, text: &str, allowed_special: &HashSet<&str>) -> Vec<Rank> {
        self.encode_allowing(text, allowed_special)
    }

    /// Encodes text with every special token allowed.
    pub fn encode_with_special_tokens(&self, text: &str) -> Vec<Rank> {
        let all: HashSet<&str> = self.specials.iter().map(|(s, _)| s.as_str()).collect();
        self.encode_allowing(text, &all)
    }

    fn encode_allowing(&self, text: &str, allowed: &HashSet<&str>) -> Vec<Rank> {
        let mut out = Vec::with_capacity(text.len() / 4 + 16);
        let mut cache = self.take_cache();
        let specials: Vec<(&str, Rank)> = self.specials.iter().filter(|(s, _)| allowed.contains(s.as_str())).map(|(s, t)| (s.as_str(), *t)).collect();
        let mut start = 0;
        loop {
            // leftmost allowed special at or after start, longest on a tie
            let mut best: Option<(usize, usize, Rank)> = None;
            for &(sp, t) in &specials {
                if let Some(j) = text[start..].find(sp) {
                    let j = j + start;
                    match best {
                        Some((bj, bl, _)) if bj < j || (bj == j && bl >= sp.len()) => {}
                        _ => best = Some((j, sp.len(), t)),
                    }
                }
            }
            let end = best.map(|b| b.0).unwrap_or(text.len());
            self.encode_segments(&mut cache, &mut out, text[start..end].as_bytes());
            match best {
                Some((j, l, t)) => {
                    out.push(t);
                    start = j + l;
                }
                None => break,
            }
        }
        self.put_cache(cache);
        out
    }

    #[doc(hidden)]
    pub fn cjk_cuts(&self) -> &cjk::CjkCuts {
        &self.cjk
    }

    /// The bytes of a token sequence; not necessarily valid UTF-8.
    pub fn decode_bytes(&self, tokens: &[Rank]) -> Result<Vec<u8>, Error> {
        // first the exact size; a special or unknown token takes the slow path
        let mut total = 0usize;
        for &t in tokens {
            match self.decoder.get(t as usize) {
                Some(&(_, len)) if len != u32::MAX => total += len as usize,
                _ => return self.decode_bytes_slow(tokens),
            }
        }
        let mut out: Vec<u8> = Vec::with_capacity(total + 8);
        let base = self.bytes.as_ptr();
        let dst = out.as_mut_ptr();
        let mut at = 0usize;
        for &t in tokens {
            let (off, len) = self.decoder[t as usize];
            let (off, len) = (off as usize, len as usize);
            // SAFETY: every token's bytes lie in self.bytes, which has eight
            // bytes of padding after the last token, and out has room for
            // total + 8 bytes, total being the sum of all lengths
            unsafe {
                if len <= 8 {
                    let w = std::ptr::read_unaligned(base.add(off) as *const u64);
                    std::ptr::write_unaligned(dst.add(at) as *mut u64, w);
                } else {
                    std::ptr::copy_nonoverlapping(base.add(off), dst.add(at), len);
                }
            }
            at += len;
        }
        // SAFETY: the first `total` bytes were written above
        unsafe { out.set_len(total) };
        Ok(out)
    }

    fn decode_bytes_slow(&self, tokens: &[Rank]) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(tokens.len() * 3);
        for &t in tokens {
            if let Some(&(off, len)) = self.decoder.get(t as usize) {
                if len != u32::MAX {
                    out.extend_from_slice(&self.bytes[off as usize..(off + len) as usize]);
                    continue;
                }
            }
            match self.special_decoder.get(&t) {
                Some(s) => out.extend_from_slice(s.as_bytes()),
                None => return Err(Error::BadVocabulary(format!("unknown token {t}"))),
            }
        }
        Ok(out)
    }

    pub fn special_tokens(&self) -> HashSet<&str> {
        self.specials.iter().map(|(s, _)| s.as_str()).collect()
    }
}

// --- the shipped encodings ---------------------------------------------------

/// A .tiktoken file as one byte arena and (start, end, rank) per token,
/// sorted by rank, without a map or an allocation per token.
fn parse_tiktoken_flat(data: &str) -> (Vec<u8>, Vec<(u32, u32, Rank)>) {
    use base64::Engine;
    let mut arena = Vec::with_capacity(data.len() * 3 / 4);
    let mut spans = Vec::with_capacity(data.len() / 12);
    for line in data.lines() {
        let Some((raw, rank)) = line.split_once(' ') else { continue };
        let a = arena.len() as u32;
        base64::engine::general_purpose::STANDARD.decode_vec(raw, &mut arena).expect("base64 token");
        spans.push((a, arena.len() as u32, rank.parse().expect("rank")));
    }
    spans.sort_unstable_by_key(|&(_, _, r)| r);
    (arena, spans)
}

pub const ENDOFTEXT: &str = "<|endoftext|>";
pub const FIM_PREFIX: &str = "<|fim_prefix|>";
pub const FIM_MIDDLE: &str = "<|fim_middle|>";
pub const FIM_SUFFIX: &str = "<|fim_suffix|>";
pub const ENDOFPROMPT: &str = "<|endofprompt|>";

const PAT_CL100K: &str = r"'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?+\p{L}++|\p{N}{1,3}+| ?[^\s\p{L}\p{N}]++[\r\n]*+|\s++$|\s*[\r\n]|\s+(?!\S)|\s";
const PAT_P50K: &str = r"'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s";
const PAT_O200K: &str = concat!(
    r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*[\p{Ll}\p{Lm}\p{Lo}\p{M}]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
    "|",
    r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+[\p{Ll}\p{Lm}\p{Lo}\p{M}]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
    "|",
    r"\p{N}{1,3}",
    "|",
    r" ?[^\s\p{L}\p{N}]+[\r\n/]*",
    "|",
    r"\s*[\r\n]+",
    "|",
    r"\s+(?!\S)",
    "|",
    r"\s+"
);

fn specials(list: &[(&str, Rank)]) -> HashMap<String, Rank> {
    list.iter().map(|&(s, t)| (s.to_string(), t)).collect()
}

pub fn cl100k_base() -> Result<CoreBPE, Error> {
    CoreBPE::shipped(
        include_str!("../assets/cl100k_base.tiktoken"),
        specials(&[(ENDOFTEXT, 100257), (FIM_PREFIX, 100258), (FIM_MIDDLE, 100259), (FIM_SUFFIX, 100260), (ENDOFPROMPT, 100276)]),
        PAT_CL100K,
        false,
    )
}

pub fn o200k_base() -> Result<CoreBPE, Error> {
    CoreBPE::shipped(
        include_str!("../assets/o200k_base.tiktoken"),
        specials(&[(ENDOFTEXT, 199999), (ENDOFPROMPT, 200018)]),
        PAT_O200K,
        false,
    )
}

pub fn p50k_base() -> Result<CoreBPE, Error> {
    CoreBPE::shipped(include_str!("../assets/p50k_base.tiktoken"), specials(&[(ENDOFTEXT, 50256)]), PAT_P50K, true)
}

pub fn p50k_edit() -> Result<CoreBPE, Error> {
    CoreBPE::shipped(
        include_str!("../assets/p50k_base.tiktoken"),
        specials(&[(ENDOFTEXT, 50256), (FIM_PREFIX, 50281), (FIM_MIDDLE, 50282), (FIM_SUFFIX, 50283)]),
        PAT_P50K,
        true,
    )
}

pub fn r50k_base() -> Result<CoreBPE, Error> {
    CoreBPE::shipped(include_str!("../assets/r50k_base.tiktoken"), specials(&[(ENDOFTEXT, 50256)]), PAT_P50K, false)
}

#[cfg(test)]
mod shipped_tests {
    use super::*;

    /// The shipped vocabularies skip the check that every token encodes to
    /// itself; this runs it.
    #[test]
    fn shipped_vocabularies_verify() {
        for data in [
            include_str!("../assets/r50k_base.tiktoken"),
            include_str!("../assets/p50k_base.tiktoken"),
            include_str!("../assets/cl100k_base.tiktoken"),
            include_str!("../assets/o200k_base.tiktoken"),
        ] {
            let (arena, spans) = parse_tiktoken_flat(data);
            let ranks: Vec<(&[u8], Rank)> = spans.iter().map(|&(a, b, t)| (&arena[a as usize..b as usize], t)).collect();
            let v = Vocab::new(&ranks, true);
            // p50k_base fails the check, so p50k_base() and p50k_edit() keep
            // checking; the other three skip it and must pass it
            let p50k = data.len() == include_str!("../assets/p50k_base.tiktoken").len();
            assert_eq!(v.bt().is_some(), !p50k, "vocabulary of {} tokens", ranks.len());
        }
    }
}
