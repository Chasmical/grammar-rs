use crate::{
    categories::GenderAnimacy,
    declension::{
        AdjectiveDeclension, AnyStemType, Declension, DeclensionFlags, MaybeZeroDeclension,
        NounDeclension, PronounDeclension,
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

type Error = ParseDeclensionError;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KindWithGenderAnimacy {
    Noun(Option<GenderAnimacy>),
    Pronoun,
    Adjective,
}

const fn parse_declension_kind(parser: &mut UnsafeParser) -> Result<KindWithGenderAnimacy> {
    use {GenderAnimacy as GA, KindWithGenderAnimacy as Kind};

    #[rustfmt::skip]
    let (len, kind) = match parser.peek_letters::<2>() {
        // Note: it's okay to match _ in cases of one-letter prefixes, since they're always
        // followed by at least 2 UTF-8 bytes: the mandatory ' ', and the stem type digit.
        Some([letters::м, letters::с]) => (4, Kind::Pronoun),
        Some([letters::п, _         ]) => (2, Kind::Adjective),
        Some([letters::м, letters::о]) => (4, Kind::Noun(Some(GA::MasculineAnimate))),
        Some([letters::м, _         ]) => (2, Kind::Noun(Some(GA::MasculineInanimate))),
        Some([letters::с, letters::о]) => (4, Kind::Noun(Some(GA::NeuterAnimate))),
        Some([letters::с, _         ]) => (2, Kind::Noun(Some(GA::NeuterInanimate))),
        Some([letters::ж, letters::о]) => (4, Kind::Noun(Some(GA::FeminineAnimate))),
        Some([letters::ж, _         ]) => (2, Kind::Noun(Some(GA::FeminineInanimate))),
        _                              => (0, Kind::Noun(None)),
    };

    if len != 0 {
        parser.forward(len);

        if !parser.skip(' ') {
            return Err(Error::Invalid);
        }
    }

    Ok(kind)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParseResult {
    stem_type: AnyStemType,
    flags: DeclensionFlags,
    stress: AnyDualStress,
}

const fn parse_declension_body(parser: &mut UnsafeParser) -> Result<ParseResult> {
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

    Ok(ParseResult { stem_type, flags, stress })
}

impl ParseResult {
    const fn to_noun(self, override_gender: Option<GenderAnimacy>) -> Result<NounDeclension> {
        Ok(NounDeclension {
            stem_type: const_try!(self.stem_type._try_into(), Error::IncompatibleStemType {}),
            stress: const_try!(self.stress._try_into(), Error::IncompatibleStress {}),
            flags: self.flags,
            override_gender,
        })
    }
    const fn to_pronoun(self) -> Result<PronounDeclension> {
        Ok(PronounDeclension {
            stem_type: const_try!(self.stem_type._try_into(), Error::IncompatibleStemType {}),
            stress: const_try!(self.stress._try_into(), Error::IncompatibleStress {}),
            flags: self.flags,
        })
    }
    const fn to_adjective(self) -> Result<AdjectiveDeclension> {
        Ok(AdjectiveDeclension {
            stem_type: const_try!(self.stem_type._try_into(), Error::IncompatibleStemType {}),
            stress: const_try!(self.stress._try_into(), Error::IncompatibleStress {}),
            flags: self.flags,
        })
    }
}

impl const PartialParse for NounDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self> {
        let kind = const_try!(parse_declension_kind(parser));

        if let KindWithGenderAnimacy::Noun(override_gender) = kind {
            const_try!(parse_declension_body(parser)).to_noun(override_gender)
        } else {
            Err(Error::Invalid)
        }
    }
}
impl const PartialParse for PronounDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self> {
        const_try!(parse_declension_body(parser)).to_pronoun()
    }
}
impl const PartialParse for AdjectiveDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self> {
        const_try!(parse_declension_body(parser)).to_adjective()
    }
}
impl const PartialParse for Declension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self> {
        use KindWithGenderAnimacy as Kind;

        let kind = const_try!(parse_declension_kind(parser));
        let res = const_try!(parse_declension_body(parser));

        Ok(match kind {
            Kind::Noun(override_gender) => Self::Noun(const_try!(res.to_noun(override_gender))),
            Kind::Pronoun => Self::Pronoun(const_try!(res.to_pronoun())),
            Kind::Adjective => Self::Adjective(const_try!(res.to_adjective())),
        })
    }
}
impl const PartialParse for MaybeZeroDeclension {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self> {
        Ok(if parser.skip('0') {
            Self::ZERO
        } else {
            const_try!(Declension::partial_parse(parser))._into()
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
impl std::str::FromStr for Declension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_str_or(s, Error::Invalid)
    }
}
impl std::str::FromStr for MaybeZeroDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_str_or(s, Error::Invalid)
    }
}
