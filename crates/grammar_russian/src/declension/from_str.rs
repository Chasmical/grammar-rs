use crate::{
    declension::{
        AdjectiveDeclension, AnyStemType, Declension, DeclensionFlags, DeclensionKind,
        NounDeclension, PronounDeclension,
    },
    letters,
    stress::{AnyDualStress, ParseStressError},
    util::{PartialParse, UnsafeParser, const_traits::*},
};

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

type Error = ParseDeclensionError;

const fn parse_declension_any(
    parser: &mut UnsafeParser,
) -> Result<(AnyStemType, DeclensionFlags, AnyDualStress), ParseDeclensionError> {
    let stem_type = match parser.read_one() {
        Some(ch @ b'1'..=b'8') => AnyStemType::from_ascii_digit(*ch).unwrap(),
        _ => return Err(Error::InvalidStemType),
    };

    let mut flags = DeclensionFlags::empty();

    DeclensionFlags::partial_parse_leading(&mut flags, parser);

    let stress = const_try!(AnyDualStress::partial_parse(parser), Error::InvalidStress);

    DeclensionFlags::partial_parse_trailing(&mut flags, parser)?;

    Ok((stem_type, flags, stress))
}

impl const PartialParse for NounDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, ParseDeclensionError> {
        let (stem_type, flags, stress) = parse_declension_any(parser)?;

        Ok(NounDeclension {
            stem_type: stem_type.into(),
            stress: const_try!(stress.try_into(), Error::IncompatibleStress {}),
            flags,
        })
    }
}
impl const PartialParse for PronounDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, ParseDeclensionError> {
        let (stem_type, flags, stress) = parse_declension_any(parser)?;

        Ok(PronounDeclension {
            stem_type: const_try!(stem_type.try_into(), Error::IncompatibleStemType {}),
            stress: const_try!(stress.try_into(), Error::IncompatibleStress {}),
            flags,
        })
    }
}
impl const PartialParse for AdjectiveDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, ParseDeclensionError> {
        let (stem_type, flags, stress) = parse_declension_any(parser)?;

        Ok(AdjectiveDeclension {
            stem_type: const_try!(stem_type.try_into(), Error::IncompatibleStemType {}),
            stress: const_try!(stress.try_into(), Error::IncompatibleStress {}),
            flags,
        })
    }
}
impl const PartialParse for Declension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        let (kind, len) = match parser.peek_letters::<2>() {
            Some([letters::м, letters::с]) => (DeclensionKind::Pronoun, 4),
            Some([letters::п, _]) => (DeclensionKind::Adjective, 2),
            _ => (DeclensionKind::Noun, 0),
        };
        if len > 0 {
            parser.forward(len);
            if !parser.skip(' ') {
                return Err(Error::Invalid);
            }
        }

        let (stem_type, flags, stress) = parse_declension_any(parser)?;

        Ok(match kind {
            DeclensionKind::Noun => Declension::Noun(NounDeclension {
                stem_type: stem_type.into(),
                stress: const_try!(stress.try_into(), Error::IncompatibleStress {}),
                flags,
            }),
            DeclensionKind::Pronoun => Declension::Pronoun(PronounDeclension {
                stem_type: const_try!(stem_type.try_into(), Error::IncompatibleStemType {}),
                stress: const_try!(stress.try_into(), Error::IncompatibleStress {}),
                flags,
            }),
            DeclensionKind::Adjective => Declension::Adjective(AdjectiveDeclension {
                stem_type: const_try!(stem_type.try_into(), Error::IncompatibleStemType {}),
                stress: const_try!(stress.try_into(), Error::IncompatibleStress {}),
                flags,
            }),
        })
    }
}

impl std::str::FromStr for NounDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or(s, Error::Invalid)
    }
}
impl std::str::FromStr for PronounDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or(s, Error::Invalid)
    }
}
impl std::str::FromStr for AdjectiveDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or(s, Error::Invalid)
    }
}
impl std::str::FromStr for Declension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or(s, Error::Invalid)
    }
}
