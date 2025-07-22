use crate::{
    declension::ParseDeclensionError,
    util::{UnsafeBuf, UnsafeParser, utf8_bytes},
};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DeclensionFlags: u8 {
        const STAR = 1 << 0;
        const CIRCLE = 1 << 1;
        const CIRCLED_ONE = 1 << 2;
        const CIRCLED_TWO = 1 << 3;
        const CIRCLED_THREE = 1 << 4;
        const ALTERNATING_YO = 1 << 5;
    }
}

impl DeclensionFlags {
    pub const fn has_star(self) -> bool {
        self.intersects(Self::STAR)
    }
    pub const fn has_circle(self) -> bool {
        self.intersects(Self::CIRCLE)
    }
    pub const fn has_circled_one(self) -> bool {
        self.intersects(Self::CIRCLED_ONE)
    }
    pub const fn has_circled_two(self) -> bool {
        self.intersects(Self::CIRCLED_TWO)
    }
    pub const fn has_circled_three(self) -> bool {
        self.intersects(Self::CIRCLED_THREE)
    }
    pub const fn has_alternating_yo(self) -> bool {
        self.intersects(Self::ALTERNATING_YO)
    }

    const ALL_LEADING_FLAGS: Self = Self::STAR.union(Self::CIRCLE);
    const ALL_TRAILING_FLAGS: Self = Self::ALL_CIRCLED_DIGITS.union(Self::ALTERNATING_YO);
    const ALL_CIRCLED_DIGITS: Self =
        Self::CIRCLED_ONE.union(Self::CIRCLED_TWO).union(Self::CIRCLED_THREE);

    pub const fn has_any_leading_flags(self) -> bool {
        self.intersects(Self::ALL_LEADING_FLAGS)
    }
    pub const fn has_any_trailing_flags(self) -> bool {
        self.intersects(Self::ALL_TRAILING_FLAGS)
    }
    pub const fn has_any_circled_digits(self) -> bool {
        self.intersects(Self::ALL_CIRCLED_DIGITS)
    }
}

// Longest form: °*①②③, ё (16 bytes, 8 chars)
pub const DECLENSION_FLAGS_MAX_LEN: usize = 16;
pub const DECLENSION_FLAGS_MAX_CHARS: usize = 8;

impl DeclensionFlags {
    #[inline]
    pub(crate) const fn fmt_leading_to_buf(self, dst: &mut UnsafeBuf) {
        if self.has_circle() {
            dst.push('°');
        }
        if self.has_star() {
            dst.push('*');
        }
    }
    #[inline]
    pub(crate) const fn fmt_trailing_to_buf(self, dst: &mut UnsafeBuf) {
        if self.has_any_trailing_flags() {
            if self.has_circled_one() {
                dst.push('①');
            }
            if self.has_circled_two() {
                dst.push('②');
            }
            if self.has_circled_three() {
                dst.push('③');
            }
            if self.has_alternating_yo() {
                dst.push_str(", ё");
            }
        }
    }
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_FLAGS_MAX_LEN]) -> &str {
        let mut dst = UnsafeBuf::new(dst);
        self.fmt_leading_to_buf(&mut dst);
        self.fmt_trailing_to_buf(&mut dst);
        dst.finish()
    }
}

impl std::fmt::Display for DeclensionFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; DECLENSION_FLAGS_MAX_LEN]).fmt(f)
    }
}

impl DeclensionFlags {
    #[inline]
    pub(crate) const fn partial_parse_leading(flags: &mut Self, parser: &mut UnsafeParser) {
        if parser.skip('°') {
            *flags = flags.union(Self::CIRCLE);
        }
        if parser.skip('*') {
            *flags = flags.union(Self::STAR);
        }
    }
    #[inline]
    pub(crate) const fn partial_parse_trailing(
        flags: &mut Self,
        parser: &mut UnsafeParser,
    ) -> Result<(), ParseDeclensionError> {
        const CircledOne_Bytes: [u8; 3] = utf8_bytes!('①');
        const CircledTwo_Bytes: [u8; 3] = utf8_bytes!('②');
        const CircledThree_Bytes: [u8; 3] = utf8_bytes!('③');

        loop {
            match parser.peek::<3>() {
                Some(&CircledOne_Bytes | b"(1)") => {
                    if flags.intersects(DeclensionFlags::CIRCLED_ONE) {
                        return Err(ParseDeclensionError::InvalidFlags);
                    }
                    *flags = flags.union(DeclensionFlags::CIRCLED_ONE);
                },
                Some(&CircledTwo_Bytes | b"(2)") => {
                    if flags.intersects(DeclensionFlags::CIRCLED_TWO) {
                        return Err(ParseDeclensionError::InvalidFlags);
                    }
                    *flags = flags.union(DeclensionFlags::CIRCLED_TWO);
                },
                Some(&CircledThree_Bytes | b"(3)") => {
                    if flags.intersects(DeclensionFlags::CIRCLED_THREE) {
                        return Err(ParseDeclensionError::InvalidFlags);
                    }
                    *flags = flags.union(DeclensionFlags::CIRCLED_THREE);
                },
                _ => break,
            };
            parser.forward(3);
        }

        if parser.skip_str(", ё") {
            *flags = flags.union(DeclensionFlags::ALTERNATING_YO);
        }

        Ok(())
    }
}
