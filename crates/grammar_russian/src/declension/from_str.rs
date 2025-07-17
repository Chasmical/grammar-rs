use crate::{
    declension::{
        AdjectiveDeclension, AnyStemType, DeclensionFlags, NounDeclension, PronounDeclension,
    },
    stress::{AnyDualStress, ParseStressError},
    util::{PartialParse, UnsafeParser, const_traits::*, utf8_bytes},
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

    if parser.skip_str(", ё") {
        flags = flags.union(DeclensionFlags::ALTERNATING_YO);
    }

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
