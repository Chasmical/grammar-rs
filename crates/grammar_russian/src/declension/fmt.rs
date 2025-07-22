use crate::{
    declension::{
        AdjectiveDeclension, AnyStemType, DeclensionFlags, NounDeclension, PronounDeclension,
    },
    stress::AnyDualStress,
    util::{UnsafeBuf, const_traits::*},
};

// Longest form: 8°*f″/f″①②③, ё (26 bytes, 14 chars)
pub const DECLENSION_MAX_LEN: usize = 26;
pub const DECLENSION_MAX_CHARS: usize = 14;

const fn fmt_declension_any(
    dst: &mut [u8; DECLENSION_MAX_LEN],
    stem_type: AnyStemType,
    flags: DeclensionFlags,
    stress: AnyDualStress,
) -> &str {
    let mut dst = UnsafeBuf::new(dst);

    dst.push_byte(stem_type.to_ascii_digit());

    flags.fmt_leading_to_buf(&mut dst);

    let stress_len = stress.fmt_to(dst.chunk()).len();
    dst.forward(stress_len);

    flags.fmt_trailing_to_buf(&mut dst);

    dst.finish()
}

impl NounDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        fmt_declension_any(dst, self.stem_type._into(), self.flags, self.stress._into())
    }
}
impl PronounDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        fmt_declension_any(dst, self.stem_type._into(), self.flags, self.stress._into())
    }
}
impl AdjectiveDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        fmt_declension_any(dst, self.stem_type._into(), self.flags, self.stress.abbr())
    }
}

impl std::fmt::Display for NounDeclension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; DECLENSION_MAX_LEN]).fmt(f)
    }
}
impl std::fmt::Display for PronounDeclension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; DECLENSION_MAX_LEN]).fmt(f)
    }
}
impl std::fmt::Display for AdjectiveDeclension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; DECLENSION_MAX_LEN]).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{declension::*, stress::*};

    #[test]
    fn fmt() {
        assert_eq!(
            NounDeclension {
                stem_type: NounStemType::Type4,
                flags: DeclensionFlags::empty(),
                stress: NounStress::B,
            }
            .to_string(),
            "4b",
        );
        assert_eq!(
            NounDeclension {
                stem_type: NounStemType::Type7,
                flags: DeclensionFlags::STAR | DeclensionFlags::CIRCLED_ONE,
                stress: NounStress::Bp,
            }
            .to_string(),
            "7*b′①",
        );
        assert_eq!(
            NounDeclension {
                stem_type: NounStemType::Type8,
                flags: DeclensionFlags::all(),
                stress: NounStress::Fpp,
            }
            .to_string(),
            "8°*f″①②③, ё",
        );

        assert_eq!(
            PronounDeclension {
                stem_type: PronounStemType::Type1,
                flags: DeclensionFlags::STAR,
                stress: PronounStress::A,
            }
            .to_string(),
            "1*a",
        );
        assert_eq!(
            PronounDeclension {
                stem_type: PronounStemType::Type6,
                flags: DeclensionFlags::all(),
                stress: PronounStress::F,
            }
            .to_string(),
            "6°*f①②③, ё",
        );

        assert_eq!(
            AdjectiveDeclension {
                stem_type: AdjectiveStemType::Type1,
                flags: DeclensionFlags::empty(),
                stress: AdjectiveStress::B,
            }
            .to_string(),
            "1b",
        );
        assert_eq!(
            AdjectiveDeclension {
                stem_type: AdjectiveStemType::Type4,
                flags: DeclensionFlags::STAR
                    | DeclensionFlags::CIRCLED_ONE
                    | DeclensionFlags::CIRCLED_TWO,
                stress: AdjectiveStress::Ap,
            }
            .to_string(),
            "4*a′①②",
        );
        assert_eq!(
            AdjectiveDeclension {
                stem_type: AdjectiveStemType::Type7,
                flags: DeclensionFlags::all(),
                stress: AdjectiveStress::A_Cpp,
            }
            .to_string(),
            "7°*a/c″①②③, ё",
        );
    }
}
