//! Hand-written pre-tokenisers for the three patterns the shipped encodings
//! use. Each returns the end of the piece that starts at `i`, reproducing the
//! regex engine's leftmost-first, greedy-with-backtracking choice; every
//! alternative consumes at least one character and every character starts
//! some alternative, so the pieces partition the text and no search is needed.
//! `s` is valid UTF-8.

use crate::unicode::{classes, ASCII, CL_DIGIT, CL_LETTER, CL_LOWER, CL_NL, CL_SPACE, CL_UPPER};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Cl100k,
    O200k,
    P50k,
}

/// The patterns, as tiktoken-rs and as openai/tiktoken write them.
pub fn kind_for(pattern: &str) -> Option<Kind> {
    const CL100K_A: &str = r"'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?+\p{L}++|\p{N}{1,3}+| ?[^\s\p{L}\p{N}]++[\r\n]*+|\s++$|\s*[\r\n]|\s+(?!\S)|\s";
    const CL100K_B: &str = r"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\r\n\p{L}\p{N}]?\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+";
    const O200K: &str = concat!(
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
    const P50K_A: &str = r"'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s";
    const P50K_B: &str = r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+";
    match pattern {
        CL100K_A | CL100K_B => Some(Kind::Cl100k),
        O200K => Some(Kind::O200k),
        P50K_A | P50K_B => Some(Kind::P50k),
        _ => None,
    }
}

/// Decodes the character at i (valid UTF-8) as a code point and its length.
#[inline(always)]
fn decode_at(s: &[u8], i: usize) -> (u32, usize) {
    let b0 = s[i] as u32;
    if b0 < 0x80 {
        return (b0, 1);
    }
    if b0 < 0xE0 {
        return (((b0 & 0x1F) << 6) | (s[i + 1] as u32 & 0x3F), 2);
    }
    if b0 < 0xF0 {
        return (((b0 & 0x0F) << 12) | ((s[i + 1] as u32 & 0x3F) << 6) | (s[i + 2] as u32 & 0x3F), 3);
    }
    (((b0 & 0x07) << 18) | ((s[i + 1] as u32 & 0x3F) << 12) | ((s[i + 2] as u32 & 0x3F) << 6) | (s[i + 3] as u32 & 0x3F), 4)
}

#[inline(always)]
fn is_letter(c: u32) -> bool {
    if c < 0x80 { ASCII[c as usize] & CL_LETTER != 0 } else { classes().letter.has(c) }
}
#[inline(always)]
fn is_number(c: u32) -> bool {
    if c < 0x80 { ASCII[c as usize] & CL_DIGIT != 0 } else { classes().number.has(c) }
}
#[inline(always)]
fn is_space(c: u32) -> bool {
    if c < 0x80 { ASCII[c as usize] & CL_SPACE != 0 } else { classes().space.has(c) }
}
/// [^\r\n\p{L}\p{N}]
#[inline(always)]
fn is_prefix(c: u32) -> bool {
    if c < 0x80 { ASCII[c as usize] & (CL_NL | CL_LETTER | CL_DIGIT) == 0 } else { !is_letter(c) && !is_number(c) }
}
/// [^\s\p{L}\p{N}]
#[inline(always)]
fn is_punct(c: u32) -> bool {
    if c < 0x80 { ASCII[c as usize] & (CL_SPACE | CL_LETTER | CL_DIGIT) == 0 } else { !is_space(c) && !is_letter(c) && !is_number(c) }
}
#[inline(always)]
fn is_lowerish(c: u32) -> bool {
    if c < 0x80 { ASCII[c as usize] & CL_LOWER != 0 } else { classes().lowerish.has(c) }
}

#[inline(always)]
pub fn scan(kind: Kind, s: &[u8], i: usize) -> usize {
    match kind {
        Kind::Cl100k => scan_cl100k(s, i),
        Kind::O200k => scan_o200k(s, i),
        Kind::P50k => scan_p50k(s, i),
    }
}

#[inline(always)]
fn scan_cl100k(s: &[u8], i: usize) -> usize {
    // ASCII first byte: the common pieces decided from the class table alone
    let c = s[i];
    if c < 0x80 {
        let cl = ASCII[c as usize];
        if cl & CL_LETTER != 0 {
            return scan_letters(s, i + 1);
        }
        if c == b' ' && i + 1 < s.len() {
            let c2 = s[i + 1];
            if c2 < 0x80 && ASCII[c2 as usize] & CL_LETTER != 0 {
                return scan_letters(s, i + 2);
            }
        }
    }
    let (r, size) = decode_at(s, i);
    // (?i:'s|'t|'re|'ve|'m|'ll|'d)
    if r == b'\'' as u32 {
        let n = contraction_len(s, i + 1, true);
        if n > 0 {
            return i + 1 + n;
        }
    }
    // [^\r\n\p{L}\p{N}]?\p{L}+
    if is_letter(r) {
        return scan_letters(s, i + size);
    }
    if is_prefix(r) && i + size < s.len() {
        let (r2, n2) = decode_at(s, i + size);
        if is_letter(r2) {
            return scan_letters(s, i + size + n2);
        }
    }
    // \p{N}{1,3}
    if is_number(r) {
        return scan_numbers_up_to(s, i + size, 2);
    }
    // ` ?[^\s\p{L}\p{N}]+[\r\n]*`
    if let Some(end) = scan_punct(s, i, r, false) {
        return end;
    }
    // \s*[\r\n]+ | \s+(?!\S) | \s+
    scan_space_alts(s, i, true)
}

#[inline(always)]
fn scan_o200k(s: &[u8], i: usize) -> usize {
    // ASCII first byte, lowercase word or space plus lowercase word: no class
    // overlap can arise, so the first alternative resolves with one run
    // ASCII first byte. A word that opens with an ASCII capital can still
    // run on into characters of both letter classes, so it takes the
    // backtracking rule of o200k_upper_word. A punctuation byte is a
    // prefix character: followed by an ASCII letter it opens a word, otherwise
    // the word alternatives fail and the punctuation run is the piece. Only a
    // following non-ASCII byte is left to the general path.
    let c = s[i];
    if c < 0x80 {
        let cl = ASCII[c as usize];
        if cl & CL_LOWER != 0 {
            return opt_contraction(s, scan_lowerish(s, i + 1));
        }
        if cl & CL_UPPER != 0 {
            return o200k_upper_word(s, i + 1);
        }
        if cl & CL_DIGIT != 0 {
            return scan_numbers_up_to(s, i + 1, 2);
        }
        // at the end of the input c2 reads as 0, whose class is empty; a real
        // NUL also reads as 0, so the end is told apart by at_end alone
        let at_end = i + 1 >= s.len();
        let next_ascii = if at_end { Some(0) } else { Some(s[i + 1]).filter(|&b| b < 0x80) };
        if let Some(c2) = next_ascii {
            let cl2 = ASCII[c2 as usize];
            if cl2 & CL_LOWER != 0 && cl & CL_NL == 0 {
                return opt_contraction(s, scan_lowerish(s, i + 2));
            }
            if cl2 & CL_UPPER != 0 && cl & CL_NL == 0 {
                return o200k_upper_word(s, i + 2);
            }
            if cl & (CL_SPACE | CL_NL) == 0 {
                // punctuation not followed by a letter
                return scan_punct(s, i, c as u32, true).unwrap_or(i + 1);
            }
            if c == b' ' && cl2 & (CL_SPACE | CL_LETTER | CL_DIGIT) == 0 && !at_end {
                return scan_punct(s, i, c as u32, true).unwrap_or(i + 1);
            }
            if cl & CL_SPACE != 0 && (cl2 & CL_SPACE != 0 || at_end || cl2 & (CL_LETTER | CL_DIGIT) != 0 || c == b'\n' || c == b'\r') {
                return scan_space_alts(s, i, true);
            }
        }
    }
    let (r, size) = decode_at(s, i);
    let prefixed = is_prefix(r);
    // [^\r\n\p{L}\p{N}]?[upperish]*[lowerish]+(?i:...)?
    if prefixed {
        if let Some(end) = o200k_lower_tail(s, i + size) {
            return end;
        }
    }
    if let Some(end) = o200k_lower_tail(s, i) {
        return end;
    }
    // [^\r\n\p{L}\p{N}]?[upperish]+[lowerish]*(?i:...)?
    if prefixed {
        if let Some(end) = o200k_upper_head(s, i + size) {
            return end;
        }
    }
    if let Some(end) = o200k_upper_head(s, i) {
        return end;
    }
    if is_number(r) {
        return scan_numbers_up_to(s, i + size, 2);
    }
    if let Some(end) = scan_punct(s, i, r, true) {
        return end;
    }
    scan_space_alts(s, i, true)
}

#[inline(always)]
fn scan_p50k(s: &[u8], i: usize) -> usize {
    let (r, size) = decode_at(s, i);
    if r == b'\'' as u32 {
        let n = contraction_len(s, i + 1, false);
        if n > 0 {
            return i + 1 + n;
        }
    }
    if is_letter(r) {
        return scan_letters(s, i + size);
    }
    if r == b' ' as u32 && i + 1 < s.len() {
        let (r2, n2) = decode_at(s, i + 1);
        if is_letter(r2) {
            return scan_letters(s, i + 1 + n2);
        }
        if is_number(r2) {
            return scan_numbers(s, i + 1 + n2);
        }
        if is_punct(r2) {
            return scan_punct_run(s, i + 1 + n2);
        }
    }
    if is_number(r) {
        return scan_numbers(s, i + size);
    }
    if is_punct(r) {
        return scan_punct_run(s, i + size);
    }
    scan_space_alts(s, i, false)
}

/// [upperish]*[lowerish]+ plus the optional contraction at p. The classes
/// overlap on Lm, Lo and M, so the greedy first loop may have to give ground:
/// the engine lands on the last position at or before the end of the upper-ish
/// run that the lower-ish class also accepts.
#[inline]
fn o200k_lower_tail(s: &[u8], p: usize) -> Option<usize> {
    let (a_end, last_lower) = scan_upperish(s, p);
    let mut q: Option<usize> = None;
    if a_end < s.len() {
        let (r, _) = decode_at(s, a_end);
        if is_lowerish(r) {
            q = Some(a_end);
        }
    }
    if q.is_none() {
        q = last_lower;
    }
    let q = q?;
    Some(opt_contraction(s, scan_lowerish(s, q)))
}

/// A word whose upper-ish run has begun with an ASCII capital, continued at p.
/// The run can go on into characters both classes accept (Lm, Lo, M); with
/// no lower-ish character after it, the first alternative gives ground back
/// to the last of those, and only without one does the second alternative
/// take the whole run.
#[inline(always)]
fn o200k_upper_word(s: &[u8], p: usize) -> usize {
    let (a_end, last_lower) = scan_upperish(s, p);
    if a_end < s.len() {
        let (r, _) = decode_at(s, a_end);
        if is_lowerish(r) {
            return opt_contraction(s, scan_lowerish(s, a_end));
        }
    }
    match last_lower {
        Some(q) => opt_contraction(s, scan_lowerish(s, q)),
        None => opt_contraction(s, a_end),
    }
}

/// [upperish]+[lowerish]* plus the optional contraction at p.
#[inline]
fn o200k_upper_head(s: &[u8], p: usize) -> Option<usize> {
    let (a_end, _) = scan_upperish(s, p);
    if a_end == p {
        return None;
    }
    Some(opt_contraction(s, scan_lowerish(s, a_end)))
}

/// ` ?[^\s\p{L}\p{N}]+[\r\n]*`, or with '/' in the tail class for o200k.
#[inline]
fn scan_punct(s: &[u8], i: usize, r: u32, slash: bool) -> Option<usize> {
    let mut p = i;
    if r == b' ' as u32 {
        if i + 1 >= s.len() {
            return None;
        }
        let (r2, _) = decode_at(s, i + 1);
        if !is_punct(r2) {
            return None;
        }
        p = i + 1;
    } else if !is_punct(r) {
        return None;
    }
    let mut end = scan_punct_run(s, p);
    if slash {
        while end < s.len() && (s[end] == b'\n' || s[end] == b'\r' || s[end] == b'/') {
            end += 1;
        }
    } else {
        while end < s.len() && (s[end] == b'\n' || s[end] == b'\r') {
            end += 1;
        }
    }
    Some(end)
}

/// `\s*[\r\n]+` when the pattern has it, then `\s+(?!\S)`, then `\s+`.
#[inline]
fn scan_space_alts(s: &[u8], i: usize, newlines: bool) -> usize {
    let (end, last_start, last_nl) = scan_space_run(s, i);
    if newlines {
        if let Some(nl) = last_nl {
            return nl + 1;
        }
    }
    if end == s.len() || last_start <= i {
        return end;
    }
    last_start
}

#[inline]
fn opt_contraction(s: &[u8], pos: usize) -> usize {
    if pos < s.len() && s[pos] == b'\'' {
        let n = contraction_len(s, pos + 1, true);
        if n > 0 {
            return pos + 1 + n;
        }
    }
    pos
}

#[inline]
fn contraction_len(s: &[u8], j: usize, fold: bool) -> usize {
    if j >= s.len() {
        return 0;
    }
    let lower = |c: u8| if fold { c.to_ascii_lowercase() } else { c };
    match lower(s[j]) {
        b's' | b't' | b'm' | b'd' => 1,
        b'r' | b'v' => {
            if j + 1 < s.len() && lower(s[j + 1]) == b'e' { 2 } else { 0 }
        }
        b'l' => {
            if j + 1 < s.len() && lower(s[j + 1]) == b'l' { 2 } else { 0 }
        }
        _ => 0,
    }
}

/// For eight bytes w, the index of the first byte that is not an ASCII
/// letter, or 8. Case is folded, then each byte is range-checked against
/// 'a'..='z' with carry tricks on the whole word.
#[inline(always)]
fn first_non_letter8(w: u64) -> usize {
    const ONES: u64 = 0x0101010101010101;
    const HIGH: u64 = 0x8080808080808080;
    let high = w & HIGH; // non-ASCII bytes
    let v = (w | (0x20 * ONES)) & !HIGH; // folded, seven bits
    let ge = (v + ((0x80 - 0x61) * ONES)) & HIGH; // set where v >= 'a'
    let gt = (v + ((0x7f - 0x7a) * ONES)) & HIGH; // set where v > 'z'
    let non = high | (!ge & HIGH) | gt;
    (non.trailing_zeros() / 8) as usize
}


#[inline]
fn scan_letters(s: &[u8], mut i: usize) -> usize {
    let letters = &classes().letter;
    while let Some(w) = s.get(i..i + 8) {
        let w = u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]]);
        let k = first_non_letter8(w);
        i += k;
        if k < 8 {
            if s[i] < 0x80 {
                return i;
            }
            break; // a non-ASCII byte: the slow loop decides
        }
    }
    while i < s.len() {
        let c = s[i];
        if c < 0x80 {
            if ASCII[c as usize] & CL_LETTER == 0 {
                return i;
            }
            i += 1;
            continue;
        }
        if c & 0xF0 == 0xE0 && i + 2 < s.len() {
            // three-byte character: decode inline, test the BMP bitmap
            let r = (((c & 0x0F) as u32) << 12) | (((s[i + 1] & 0x3F) as u32) << 6) | (s[i + 2] & 0x3F) as u32;
            if !letters.has(r) {
                return i;
            }
            i += 3;
            continue;
        }
        let (r, n) = decode_at(s, i);
        if !letters.has(r) {
            return i;
        }
        i += n;
    }
    i
}

#[inline]
fn scan_numbers(s: &[u8], mut i: usize) -> usize {
    while i < s.len() {
        let c = s[i];
        if c < 0x80 {
            if ASCII[c as usize] & CL_DIGIT == 0 {
                return i;
            }
            i += 1;
            continue;
        }
        let (r, n) = decode_at(s, i);
        if !classes().number.has(r) {
            return i;
        }
        i += n;
    }
    i
}

#[inline]
fn scan_numbers_up_to(s: &[u8], mut i: usize, mut more: usize) -> usize {
    while more > 0 && i < s.len() {
        let c = s[i];
        if c < 0x80 {
            if ASCII[c as usize] & CL_DIGIT == 0 {
                return i;
            }
            i += 1;
        } else {
            let (r, n) = decode_at(s, i);
            if !classes().number.has(r) {
                return i;
            }
            i += n;
        }
        more -= 1;
    }
    i
}

#[inline]
fn scan_punct_run(s: &[u8], mut i: usize) -> usize {
    while i < s.len() {
        let c = s[i];
        if c < 0x80 {
            if ASCII[c as usize] & (CL_SPACE | CL_LETTER | CL_DIGIT) != 0 {
                return i;
            }
            i += 1;
            continue;
        }
        let (r, n) = decode_at(s, i);
        if !is_punct(r) {
            return i;
        }
        i += n;
    }
    i
}

/// Walks [upperish]* from i; returns the end and the offset of the last
/// character in the run that the lower-ish class also accepts.
#[inline]
fn scan_upperish(s: &[u8], mut i: usize) -> (usize, Option<usize>) {
    let mut last_lower = None;
    while i < s.len() {
        let c = s[i];
        if c < 0x80 {
            if ASCII[c as usize] & CL_UPPER == 0 {
                return (i, last_lower);
            }
            i += 1;
            continue;
        }
        let (r, n) = decode_at(s, i);
        if !classes().upperish.has(r) {
            return (i, last_lower);
        }
        if classes().lowerish.has(r) {
            last_lower = Some(i);
        }
        i += n;
    }
    (i, last_lower)
}

#[inline]
fn scan_lowerish(s: &[u8], mut i: usize) -> usize {
    while i < s.len() {
        let c = s[i];
        if c < 0x80 {
            if ASCII[c as usize] & CL_LOWER == 0 {
                return i;
            }
            i += 1;
            continue;
        }
        let (r, n) = decode_at(s, i);
        if !classes().lowerish.has(r) {
            return i;
        }
        i += n;
    }
    i
}

/// Walks \s* from i: returns the end, the start of the run's last character,
/// and the offset of the last \r or \n in it.
#[inline]
fn scan_space_run(s: &[u8], i: usize) -> (usize, usize, Option<usize>) {
    let (mut end, mut last_start, mut last_nl) = (i, i, None);
    while end < s.len() {
        let c = s[end];
        if c < 0x80 {
            if ASCII[c as usize] & CL_SPACE == 0 {
                return (end, last_start, last_nl);
            }
            if ASCII[c as usize] & CL_NL != 0 {
                last_nl = Some(end);
            }
            last_start = end;
            end += 1;
            continue;
        }
        let (r, n) = decode_at(s, end);
        if !classes().space.has(r) {
            return (end, last_start, last_nl);
        }
        last_start = end;
        end += n;
    }
    (end, last_start, last_nl)
}
