use crate::{
    categories::GenderAnimacy, declension::*, letters, stress::*, util::{const_traits::*, UnsafeBuf}, Letter
};

// Longest forms:
// Noun     : жо 8°*f″①②③, ё   (14 chars, 26 bytes)
// Pronoun  : мс 8°*f①②③, ё    (13 chars, 23 bytes)
// Adjective: п 8°*f″/f″①②③, ё (16 chars, 29 bytes)
pub const DECLENSION_MAX_LEN: usize = 29;

impl Declension {
    pub const fn fmt_to<'a>(self, dst: &'a mut [u8; DECLENSION_MAX_LEN]) -> &'a str {
        let mut dst = UnsafeBuf::new(dst);

        let (stem_type, stress, flags): (AnyStemType, AnyDualStress, DeclensionFlags);

        match self {
            Declension::Noun(decl) => {
                if let Some(gender_animacy) = decl.override_gender {
                    let gen_an_len = gender_animacy.fmt_to(dst.sub_buf()).len();
                    dst.advance_by(gen_an_len);
                    dst.push(' ');
                }
                stem_type = decl.stem_type._into();
                stress = decl.stress._into();
                flags = decl.flags;
            },
            Declension::Pronoun(decl) => {
                dst.push_str("мс ");
                stem_type = decl.stem_type._into();
                stress = decl.stress._into();
                flags = decl.flags;
            },
            Declension::Adjective(decl) => {
                dst.push_str("п ");
                stem_type = decl.stem_type._into();
                stress = decl.stress.abbr();
                flags = decl.flags;
            },
        };

        dst.push_byte(b'0' + stem_type as u8);

        if flags.has_any_leading_flags() {
            if flags.has_circle() {
                dst.push('°');
            }
            if flags.has_star() {
                dst.push('*');
            }
        }

        let stress_len = stress.fmt_to(dst.sub_buf()).len();
        dst.advance_by(stress_len);

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

        dst.finish()
    }
}
impl NounDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        Declension::Noun(self).fmt_to(dst)
    }
}
impl PronounDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        Declension::Pronoun(self).fmt_to(dst)
    }
}
impl AdjectiveDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        Declension::Adjective(self).fmt_to(dst)
    }
}

impl std::fmt::Display for Declension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; DECLENSION_MAX_LEN]).fmt(f)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseDeclensionError {
    Invalid,
    InvalidStemType,
    InvalidStress(ParseStressError),
    InvalidFlags,
}

// TODO: declension parsing

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
