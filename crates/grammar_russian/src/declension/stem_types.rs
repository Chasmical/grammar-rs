use crate::util::{const_traits::*, enum_conversion};
use thiserror::Error;

macro_rules! impl_stem_type {
    (
        $(#[$outer:meta])*
        $vis:vis enum $T:ident {
            $($(#[$inner:meta])* $variant:ident = $value:expr),* $(,)?
        }
        $(#[$outer_e:meta])*
        $vis_e:vis struct $E:ident($error:expr);
    ) => (
        $(#[$outer])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        $vis enum $T {
            $($(#[$inner])* $variant,)+
        }
        $(#[$outer_e])*
        #[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
        #[error($error)]
        $vis_e struct $E;

        impl $T {
            pub const fn from_digit(num: u8) -> Option<Self> {
                Some(match num {
                    $($value => <$T>::$variant,)*
                    _ => return None,
                })
            }
            pub const fn from_ascii_digit(num: u8) -> Option<Self> {
                Self::from_digit(num - b'0')
            }
            pub const fn to_digit(&self) -> u8 {
                match self {
                    $(<$T>::$variant => $value,)*
                }
            }
            pub const fn to_ascii_digit(&self) -> u8 {
                b'0' + self.to_digit()
            }
        }
        impl std::fmt::Display for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.to_ascii_digit().fmt(f)
            }
        }
        impl std::str::FromStr for $T {
            type Err = $E;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let [ch] = s.as_bytes() {
                    Self::from_ascii_digit(*ch)._ok_or(Self::Err {})
                } else {
                    Err(Self::Err {})
                }
            }
        }
    );
}

impl_stem_type! {
    pub enum AnyStemType {
        Type1 = 1, Type2 = 2, Type3 = 3, Type4 = 4,
        Type5 = 5, Type6 = 6, Type7 = 7, Type8 = 8,
    }
    pub struct AnyStemTypeError("words can only have stem types 1 through 8");
}
impl_stem_type! {
    pub enum NounStemType {
        Type1 = 1, Type2 = 2, Type3 = 3, Type4 = 4,
        Type5 = 5, Type6 = 6, Type7 = 7, Type8 = 8,
    }
    pub struct NounStemTypeError("nouns can only have stem types 1 through 8");
}
impl_stem_type! {
    pub enum PronounStemType {
        Type1 = 1, Type2 = 2, Type4 = 4, Type6 = 6,
    }
    pub struct PronounStemTypeError("pronouns can only have stem types 1, 2, 4 and 6");
}
impl_stem_type! {
    pub enum AdjectiveStemType {
        Type1 = 1, Type2 = 2, Type3 = 3, Type4 = 4,
        Type5 = 5, Type6 = 6, Type7 = 7,
    }
    pub struct AdjectiveStemTypeError("adjectives can only have stem types 1 through 7");
}

enum_conversion!(NounStemType => <= AnyStemType {
    Type1, Type2, Type3, Type4, Type5, Type6, Type7, Type8,
});
enum_conversion!(PronounStemType => AnyStemType [<= PronounStemTypeError] {
    Type1, Type2, Type4, Type6,
});
enum_conversion!(AdjectiveStemType => AnyStemType [<= AdjectiveStemTypeError] {
    Type1, Type2, Type3, Type4, Type5, Type6, Type7,
});
