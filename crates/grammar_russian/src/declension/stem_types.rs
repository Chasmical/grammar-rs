use crate::util::enum_conversion;
use thiserror::Error;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PronounStemType {
    Type1 = 1,
    Type2 = 2,
    Type4 = 4,
    Type6 = 6,
}
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

#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("words can only have stem types 1 through 8")]
pub struct AnyStemTypeError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("nouns can only have stem types 1 through 8")]
pub struct NounStemTypeError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("pronouns can only have stem types 1, 2, 4 and 6")]
pub struct PronounStemTypeError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("adjectives can only have stem types 1 through 7")]
pub struct AdjectiveStemTypeError;

enum_conversion!(NounStemType => <= AnyStemType {
    Type1, Type2, Type3, Type4, Type5, Type6, Type7, Type8,
});
enum_conversion!(PronounStemType => AnyStemType [<= PronounStemTypeError] {
    Type1, Type2, Type4, Type6,
});
enum_conversion!(AdjectiveStemType => AnyStemType [<= AdjectiveStemTypeError] {
    Type1, Type2, Type3, Type4, Type5, Type6, Type7,
});

macro_rules! define_from_impl {
    ($(
        $StemType:ty => $repr:ty [$error:ty]: {
            $($variant:ident => $value:literal,)*
        }
    )*) => ($(
        impl $StemType {
            pub const fn from_digit(num: $repr) -> Result<Self, $error> {
                type Error = $error;
                Ok(match num {
                    $($value => <$StemType>::$variant,)*
                    _ => return Err(Error {}),
                })
            }
            pub const fn from_ascii_digit(num: $repr) -> Result<Self, $error> {
                Self::from_digit(num - b'0')
            }
            pub const fn to_digit(self) -> $repr {
                match self {
                    $(<$StemType>::$variant => $value,)*
                }
            }
            pub const fn to_ascii_digit(self) -> $repr {
                b'0' + self.to_digit()
            }
        }
    )*);
}

define_from_impl! {
    AnyStemType => u8 [AnyStemTypeError]: {
        Type1 => 1, Type2 => 2, Type3 => 3, Type4 => 4,
        Type5 => 5, Type6 => 6, Type7 => 7, Type8 => 8,
    }
    NounStemType => u8 [NounStemTypeError]: {
        Type1 => 1, Type2 => 2, Type3 => 3, Type4 => 4,
        Type5 => 5, Type6 => 6, Type7 => 7, Type8 => 8,
    }
    PronounStemType => u8 [PronounStemTypeError]: {
        Type1 => 1, Type2 => 2, Type4 => 4, Type6 => 6,
    }
    AdjectiveStemType => u8 [AdjectiveStemTypeError]: {
        Type1 => 1, Type2 => 2, Type3 => 3, Type4 => 4,
        Type5 => 5, Type6 => 6, Type7 => 7,
    }
}
