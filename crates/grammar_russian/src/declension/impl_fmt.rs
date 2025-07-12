use crate::{categories::*, declension::*, stress::*, util::*};

// Longest forms:
// Noun     : жо 8°*f″①②③, ё   (26 bytes, 14 chars)
// Pronoun  : мс 8°*f①②③, ё    (23 bytes, 13 chars)
// Adjective: п 8°*f″/f″①②③, ё (29 bytes, 16 chars)
pub const DECLENSION_MAX_LEN: usize = 29;
pub const DECLENSION_MAX_CHARS: usize = 16;

impl Declension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &str {
        let mut dst = UnsafeBuf::new(dst);

        let (stem_type, stress, flags): (AnyStemType, AnyDualStress, DeclensionFlags);

        match self {
            Self::Noun(decl) => {
                if let Some(gender_animacy) = decl.override_gender {
                    dst.push_str(gender_animacy.abbr_zaliznyak());
                    dst.push(' ');
                }
                stem_type = decl.stem_type._into();
                stress = decl.stress._into();
                flags = decl.flags;
            },
            Self::Pronoun(decl) => {
                dst.push_str("мс ");
                stem_type = decl.stem_type._into();
                stress = decl.stress._into();
                flags = decl.flags;
            },
            Self::Adjective(decl) => {
                dst.push_str("п ");
                stem_type = decl.stem_type._into();
                stress = decl.stress.abbr();
                flags = decl.flags;
            },
        };

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
    InvalidStemType,
    InvalidStress(ParseStressError),
    InvalidFlags,
    IncompatibleStemType,
    IncompatibleStress,
    IncompatibleFlags,
    Invalid,
}

const fn parse_declension_base(
    parser: &mut UnsafeParser,
) -> Result<(AnyStemType, AnyDualStress, DeclensionFlags), ParseDeclensionError> {
    use ParseDeclensionError as Error;

    let stem_type = match parser.read_one() {
        Some(ch @ b'1'..=b'8') => AnyStemType::from_ascii_digit(*ch)._unwrap(),
        _ => return Err(Error::InvalidStemType),
    };

    let mut flags = DeclensionFlags::empty();

    if parser.skip('°') {
        flags = flags.union(DeclensionFlags::CIRCLE);
    }
    if parser.skip('*') {
        flags = flags.union(DeclensionFlags::STAR);
    }

    let stress = const_try!(AnyDualStress::partial_parse(parser), Error::InvalidStress);

    const CircledOne_Bytes: [u8; 3] = utf8_bytes!('①');
    const CircledTwo_Bytes: [u8; 3] = utf8_bytes!('②');
    const CircledThree_Bytes: [u8; 3] = utf8_bytes!('③');

    loop {
        match parser.peek::<3>() {
            Some(&CircledOne_Bytes | &[b'(', b'1', b')']) => {
                if flags.intersects(DeclensionFlags::CIRCLED_ONE) {
                    return Err(Error::InvalidFlags);
                }
                flags = flags.union(DeclensionFlags::CIRCLED_ONE);
            },
            Some(&CircledTwo_Bytes | &[b'(', b'2', b')']) => {
                if flags.intersects(DeclensionFlags::CIRCLED_TWO) {
                    return Err(Error::InvalidFlags);
                }
                flags = flags.union(DeclensionFlags::CIRCLED_TWO);
            },
            Some(&CircledThree_Bytes | &[b'(', b'3', b')']) => {
                if flags.intersects(DeclensionFlags::CIRCLED_THREE) {
                    return Err(Error::InvalidFlags);
                }
                flags = flags.union(DeclensionFlags::CIRCLED_THREE);
            },
            _ => break,
        };
        parser.forward(3);
    }

    Ok((stem_type, stress, flags))
}

impl const PartialParse for NounDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        use crate::letters;

        let override_gender = match parser.peek_letters::<2>() {
            // Note: it's okay to match _ in cases of inanimate genders, since they're always
            // followed by at least 2 UTF-8 bytes: the mandatory ' ', and the stem type digit.
            Some([letters::м, letters::о]) => Some(GenderAnimacy::MasculineAnimate),
            Some([letters::м, _]) => Some(GenderAnimacy::MasculineInanimate),
            Some([letters::с, letters::о]) => Some(GenderAnimacy::NeuterAnimate),
            Some([letters::с, _]) => Some(GenderAnimacy::NeuterInanimate),
            Some([letters::ж, letters::о]) => Some(GenderAnimacy::FeminineAnimate),
            Some([letters::ж, _]) => Some(GenderAnimacy::FeminineInanimate),
            _ => None,
        };
        if let Some(gender_animacy) = override_gender {
            parser.forward(if gender_animacy.is_animate() { 4 } else { 2 });
            if !parser.skip(' ') {
                return Err(Self::Err::Invalid);
            }
        }

        let (stem_type, stress, flags) = const_try!(parse_declension_base(parser));

        Ok(NounDeclension {
            stem_type: const_try!(stem_type._try_into(), Self::Err::IncompatibleStemType {}),
            stress: const_try!(stress._try_into(), Self::Err::IncompatibleStress {}),
            flags,
            override_gender,
        })
    }
}
impl const PartialParse for PronounDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        let (stem_type, stress, flags) = const_try!(parse_declension_base(parser));
        // TODO: what about the 'мс ' prefix? Parse it here, or only in Declension?

        Ok(PronounDeclension {
            stem_type: const_try!(stem_type._try_into(), Self::Err::IncompatibleStemType {}),
            stress: const_try!(stress._try_into(), Self::Err::IncompatibleStress {}),
            flags,
        })
    }
}
impl const PartialParse for AdjectiveDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        let (stem_type, stress, flags) = const_try!(parse_declension_base(parser));
        // TODO: what about the 'п ' prefix? Parse it here, or only in Declension?

        Ok(AdjectiveDeclension {
            stem_type: const_try!(stem_type._try_into(), Self::Err::IncompatibleStemType {}),
            stress: const_try!(stress._try_into(), Self::Err::IncompatibleStress {}),
            flags,
        })
    }
}

// TODO: implement parsing Declension enum

impl std::str::FromStr for NounDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or(s, Self::Err::Invalid)
    }
}
impl std::str::FromStr for PronounDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or(s, Self::Err::Invalid)
    }
}
impl std::str::FromStr for AdjectiveDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or(s, Self::Err::Invalid)
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
