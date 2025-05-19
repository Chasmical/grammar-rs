use thiserror::Error;

use crate::util::{const_traits::*, enum_conversion};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnyStemType {
    Type1 = 1,
    Type2 = 2,
    Type3 = 3,
    Type4 = 4,
    Type5 = 5,
    Type6 = 6,
    Type7 = 7,
    Type8 = 8,
}
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("words can only have stem types 1 through 8")]
pub struct AnyStemTypeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NounStemType {
    Type1 = 1,
    Type2 = 2,
    Type3 = 3,
    Type4 = 4,
    Type5 = 5,
    Type6 = 6,
    Type7 = 7,
    Type8 = 8,
}
enum_conversion! {
    impl From<NounStemType, Error = NounStemTypeError> for AnyStemType {
        Type1, Type2, Type3, Type4, Type5, Type6, Type7, Type8,
    }
}
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("nouns can only have stem types 1 through 8")]
pub struct NounStemTypeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdjectiveStemType {
    Type1 = 1,
    Type2 = 2,
    Type3 = 3,
    Type4 = 4,
    Type5 = 5,
    Type6 = 6,
    Type7 = 7,
}
enum_conversion! {
    impl From<AdjectiveStemType, Error = AdjectiveStemTypeError> for AnyStemType {
        Type1, Type2, Type3, Type4, Type5, Type6, Type7,
    }
}
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("adjectives can only have stem types 1 through 7")]
pub struct AdjectiveStemTypeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PronounStemType {
    Type1 = 1,
    Type2 = 2,
    Type4 = 4,
    Type6 = 6,
}
enum_conversion! {
    impl From<PronounStemType, Error = PronounStemTypeError> for AnyStemType {
        Type1, Type2, Type4, Type6,
    }
}
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("pronouns can only have stem types 1, 2, 4 and 6")]
pub struct PronounStemTypeError;

// TODO: From<Any> for Noun (full mapping)
// TODO: TryFrom for noun -> adjective, etc.

impl_const_TryFrom!(<u8> for AnyStemType {
    type Error = AnyStemTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Type1,
            2 => Self::Type2,
            3 => Self::Type3,
            4 => Self::Type4,
            5 => Self::Type5,
            6 => Self::Type6,
            7 => Self::Type7,
            8 => Self::Type8,
            _ => return Err(Self::Error {}),
        })
    }
});
impl_const_TryFrom!(<u8> for NounStemType {
    type Error = NounStemTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // FIXME(const-hack): Replace with `.map_or(Err(Self::Error {}), |x| x.try_into())`.
        match AnyStemType::_try_from(value) {
            Ok(x) => x._try_into(),
            Err(_) => Err(Self::Error {}),
        }
    }
});
impl_const_TryFrom!(<u8> for AdjectiveStemType {
    type Error = AdjectiveStemTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // FIXME(const-hack): Replace with `.map_or(Err(Self::Error {}), |x| x.try_into())`.
        match AnyStemType::_try_from(value) {
            Ok(x) => x._try_into(),
            Err(_) => Err(Self::Error {}),
        }
    }
});
impl_const_TryFrom!(<u8> for PronounStemType {
    type Error = PronounStemTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // FIXME(const-hack): Replace with `.map_or(Err(Self::Error {}), |x| x.try_into())`.
        match AnyStemType::_try_from(value) {
            Ok(x) => x._try_into(),
            Err(_) => Err(Self::Error {}),
        }
    }
});

macro_rules! stem_type_display_fromstr_impl {
    ($($t:ty, $err:ty;)*) => ($(
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                ((b'0' + *self as u8) as char).fmt(f)
            }
        }
        impl std::str::FromStr for $t {
            type Err = $err;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.as_bytes() {
                    [ch @ _] => Self::try_from(*ch - b'0'),
                    _ => Err(Self::Err {}),
                }
            }
        }
    )*);
}

stem_type_display_fromstr_impl! {
    AnyStemType, AnyStemTypeError;
    NounStemType, NounStemTypeError;
    AdjectiveStemType, AdjectiveStemTypeError;
    PronounStemType, PronounStemTypeError;
}
