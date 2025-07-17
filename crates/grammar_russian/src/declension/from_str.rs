use crate::{
    categories::{GenderAnimacy, HasAnimacy},
    declension::{
        AdjectiveDeclension, AnyStemType, DeclensionFlags, NounDeclension, PronounDeclension,
    },
    letters,
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

const fn parse_declension_any(
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

    // TODO: parse `, ё`

    Ok((stem_type, stress, flags))
}

impl const PartialParse for NounDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
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

        let (stem_type, stress, flags) = const_try!(parse_declension_any(parser));

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
        let (stem_type, stress, flags) = const_try!(parse_declension_any(parser));
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
        let (stem_type, stress, flags) = const_try!(parse_declension_any(parser));
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
