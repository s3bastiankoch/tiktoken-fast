//! Character classes, exact to the regex crate's Unicode tables, which is what
//! the reference core matches its pattern with. The Basic Multilingual Plane is
//! a bitmap per class, one bit per code point; astral code points go through
//! the range lists.

use std::sync::OnceLock;

pub struct Class {
    bmp: Vec<u64>,          // 0x10000 / 64 words
    astral: Vec<(u32, u32)>, // inclusive ranges above 0xFFFF, sorted
}

impl Class {
    fn from_pattern(pat: &str) -> Class {
        use regex_syntax::hir::{Class as HClass, HirKind};
        let hir = regex_syntax::ParserBuilder::new().build().parse(pat).expect("class pattern");
        let ranges: Vec<(u32, u32)> = match hir.kind() {
            HirKind::Class(HClass::Unicode(c)) => c.ranges().iter().map(|r| (r.start() as u32, r.end() as u32)).collect(),
            _ => panic!("not a class: {pat}"),
        };
        let mut bmp = vec![0u64; 0x10000 / 64];
        let mut astral = Vec::new();
        for &(lo, hi) in &ranges {
            let mut c = lo;
            while c <= hi && c < 0x10000 {
                bmp[(c >> 6) as usize] |= 1 << (c & 63);
                c += 1;
            }
            if hi >= 0x10000 {
                astral.push((lo.max(0x10000), hi));
            }
        }
        Class { bmp, astral }
    }

    #[inline]
    pub fn has(&self, c: u32) -> bool {
        if c < 0x10000 {
            self.bmp[(c >> 6) as usize] & (1 << (c & 63)) != 0
        } else {
            self.astral.binary_search_by(|&(lo, hi)| {
                if c < lo { std::cmp::Ordering::Greater } else if c > hi { std::cmp::Ordering::Less } else { std::cmp::Ordering::Equal }
            }).is_ok()
        }
    }
}

pub struct Classes {
    pub letter: Class,   // \p{L}
    pub number: Class,   // \p{N}
    pub upperish: Class, // [\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]
    pub lowerish: Class, // [\p{Ll}\p{Lm}\p{Lo}\p{M}]
    pub space: Class,    // \s
}

pub fn classes() -> &'static Classes {
    static C: OnceLock<Classes> = OnceLock::new();
    C.get_or_init(|| Classes {
        letter: Class::from_pattern(r"\p{L}"),
        number: Class::from_pattern(r"\p{N}"),
        upperish: Class::from_pattern(r"[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]"),
        lowerish: Class::from_pattern(r"[\p{Ll}\p{Lm}\p{Lo}\p{M}]"),
        space: Class::from_pattern(r"\s"),
    })
}

// ASCII classes in one table.
pub const CL_LETTER: u8 = 1;
pub const CL_DIGIT: u8 = 2;
pub const CL_SPACE: u8 = 4;
pub const CL_NL: u8 = 8;
pub const CL_UPPER: u8 = 16;
pub const CL_LOWER: u8 = 32;

pub static ASCII: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut c = 0usize;
    while c < 128 {
        let b = c as u8;
        t[c] = if b.is_ascii_lowercase() {
            CL_LETTER | CL_LOWER
        } else if b.is_ascii_uppercase() {
            CL_LETTER | CL_UPPER
        } else if b.is_ascii_digit() {
            CL_DIGIT
        } else if b == b' ' || (b >= 0x09 && b <= 0x0d) {
            if b == b'\n' || b == b'\r' { CL_SPACE | CL_NL } else { CL_SPACE }
        } else {
            0
        };
        c += 1;
    }
    t
};
