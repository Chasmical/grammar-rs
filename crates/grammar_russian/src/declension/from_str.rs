use crate::{
    declension::{
        AdjectiveDeclension, AnyStemType, DeclensionFlags, NounDeclension, PronounDeclension,
    },
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
type Result<T> = std::result::Result<T, Error>;

const fn parse_declension_any(
    parser: &mut UnsafeParser,
) -> Result<(AnyStemType, DeclensionFlags, AnyDualStress)> {
    let stem_type = match parser.read_one() {
        Some(ch @ b'1'..=b'8') => AnyStemType::from_ascii_digit(*ch)._unwrap(),
        _ => return Err(Error::InvalidStemType),
    };

    let mut flags = DeclensionFlags::empty();

    DeclensionFlags::partial_parse_leading(&mut flags, parser);

    let stress = const_try!(AnyDualStress::partial_parse(parser), Error::InvalidStress);

    const_try!(DeclensionFlags::partial_parse_trailing(&mut flags, parser));

    Ok((stem_type, flags, stress))
}

impl const PartialParse for NounDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self> {
        let (stem_type, flags, stress) = const_try!(parse_declension_any(parser));

        Ok(NounDeclension {
            stem_type: const_try!(stem_type._try_into(), Error::IncompatibleStemType {}),
            stress: const_try!(stress._try_into(), Error::IncompatibleStress {}),
            flags,
        })
    }
}
impl const PartialParse for PronounDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self> {
        let (stem_type, flags, stress) = const_try!(parse_declension_any(parser));

        Ok(PronounDeclension {
            stem_type: const_try!(stem_type._try_into(), Error::IncompatibleStemType {}),
            stress: const_try!(stress._try_into(), Error::IncompatibleStress {}),
            flags,
        })
    }
}
impl const PartialParse for AdjectiveDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self> {
        let (stem_type, flags, stress) = const_try!(parse_declension_any(parser));

        Ok(AdjectiveDeclension {
            stem_type: const_try!(stem_type._try_into(), Error::IncompatibleStemType {}),
            stress: const_try!(stress._try_into(), Error::IncompatibleStress {}),
            flags,
        })
    }
}

impl std::str::FromStr for NounDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_str_or(s, Error::Invalid)
    }
}
impl std::str::FromStr for PronounDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_str_or(s, Error::Invalid)
    }
}
impl std::str::FromStr for AdjectiveDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_str_or(s, Error::Invalid)
    }
}
