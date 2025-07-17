use crate::{
    declension::{
        AdjectiveDeclension, AnyStemType, Declension, DeclensionFlags, MaybeZeroDeclension,
        NounDeclension, PronounDeclension,
    },
    stress::AnyDualStress,
    util::{UnsafeBuf, const_traits::*},
};

// Longest forms:
// Noun     : жо 8°*f″①②③, ё   (26 bytes, 14 chars)
// Pronoun  : мс 8°*f①②③, ё    (23 bytes, 13 chars)
// Adjective: п 8°*f″/f″①②③, ё (29 bytes, 16 chars)
pub const DECLENSION_MAX_LEN: usize = 29;
pub const DECLENSION_MAX_CHARS: usize = 16;

const fn fmt_declension_body(
    stem_type: AnyStemType,
    flags: DeclensionFlags,
    stress: AnyDualStress,
    dst: &mut UnsafeBuf,
) {
    dst.push_byte(stem_type.to_ascii_digit());

    if flags.has_any_leading_flags() {
        if flags.has_circle() {
            dst.push('°');
        }
        if flags.has_star() {
            dst.push('*');
        }
    }

    let stress_len = stress.fmt_to(dst.chunk()).len();
    dst.forward(stress_len);

    if flags.has_any_trailing_flags() {
        if flags.has_circled_one() {
            dst.push('①');
        }
        if flags.has_circled_two() {
            dst.push('②');
        }
        if flags.has_circled_three() {
            dst.push('③');
        }
        if flags.has_alternating_yo() {
            dst.push_str(", ё");
        }
    }
}

impl NounDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        let mut dst = UnsafeBuf::new(dst);
        if let Some(gender_animacy) = self.override_gender {
            dst.push_str(gender_animacy.abbr_zaliznyak());
            dst.push(' ');
        }
        fmt_declension_body(self.stem_type._into(), self.flags, self.stress._into(), &mut dst);
        dst.finish()
    }
}
impl PronounDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        let mut dst = UnsafeBuf::new(dst);
        fmt_declension_body(self.stem_type._into(), self.flags, self.stress._into(), &mut dst);
        dst.finish()
    }
}
impl AdjectiveDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        let mut dst = UnsafeBuf::new(dst);
        fmt_declension_body(self.stem_type._into(), self.flags, self.stress._into(), &mut dst);
        dst.finish()
    }
}
impl Declension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        let mut dst = UnsafeBuf::new(dst);

        let info = match self {
            Self::Noun(decl) => {
                if let Some(gender_animacy) = decl.override_gender {
                    dst.push_str(gender_animacy.abbr_zaliznyak());
                    dst.push(' ');
                }
                (decl.stem_type._into(), decl.flags, decl.stress._into())
            },
            Self::Pronoun(decl) => {
                dst.push_str("мс ");
                (decl.stem_type._into(), decl.flags, decl.stress._into())
            },
            Self::Adjective(decl) => {
                dst.push_str("п ");
                (decl.stem_type._into(), decl.flags, decl.stress._into())
            },
        };

        fmt_declension_body(info.0, info.1, info.2, &mut dst);

        dst.finish()
    }
}
impl MaybeZeroDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        if let Some(decl) = self.as_option() {
            decl.fmt_to(dst)
        } else {
            dst[0] = b'0';
            unsafe { str::from_utf8_unchecked(dst.first_chunk::<1>().unwrap()) }
        }
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
impl std::fmt::Display for Declension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; DECLENSION_MAX_LEN]).fmt(f)
    }
}
impl std::fmt::Display for MaybeZeroDeclension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; DECLENSION_MAX_LEN]).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{categories::*, declension::*, stress::*};

    #[test]
    fn fmt() {
        assert_eq!(
            NounDeclension {
                stem_type: NounStemType::Type4,
                flags: DeclensionFlags::empty(),
                stress: NounStress::B,
                override_gender: None,
            }
            .to_string(),
            "4b",
        );
        assert_eq!(
            NounDeclension {
                stem_type: NounStemType::Type7,
                flags: DeclensionFlags::STAR | DeclensionFlags::CIRCLED_ONE,
                stress: NounStress::Bp,
                override_gender: Some(GenderAnimacy::FeminineAnimate),
            }
            .to_string(),
            "жо 7*b′①",
        );
        assert_eq!(
            NounDeclension {
                stem_type: NounStemType::Type8,
                flags: DeclensionFlags::all(),
                stress: NounStress::Fpp,
                override_gender: Some(GenderAnimacy::NeuterAnimate),
            }
            .to_string(),
            "со 8°*f″①②③, ё",
        );

        assert_eq!(
            PronounDeclension {
                stem_type: PronounStemType::Type1,
                flags: DeclensionFlags::STAR,
                stress: PronounStress::A,
            }
            .to_string(),
            "мс 1*a",
        );
        assert_eq!(
            PronounDeclension {
                stem_type: PronounStemType::Type6,
                flags: DeclensionFlags::all(),
                stress: PronounStress::F,
            }
            .to_string(),
            "мс 6°*f①②③, ё",
        );

        assert_eq!(
            AdjectiveDeclension {
                stem_type: AdjectiveStemType::Type1,
                flags: DeclensionFlags::empty(),
                stress: AdjectiveStress::B,
            }
            .to_string(),
            "п 1b",
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
            "п 4*a′①②",
        );
        assert_eq!(
            AdjectiveDeclension {
                stem_type: AdjectiveStemType::Type7,
                flags: DeclensionFlags::all(),
                stress: AdjectiveStress::A_Cpp,
            }
            .to_string(),
            "п 7°*a/c″①②③, ё",
        );
    }
}
