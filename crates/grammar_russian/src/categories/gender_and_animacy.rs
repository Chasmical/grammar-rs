use thiserror::Error;

use crate::{
    categories::*,
    util::{const_traits::*, enum_conversion},
};

/// A main or secondary Russian grammatical gender: [`Masculine`][GenderEx::Masculine],
/// [`Neuter`][GenderEx::Neuter], [`Feminine`][GenderEx::Feminine] or [`Common`][GenderEx::Common].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderEx {
    #[default]
    Masculine = 0,
    Neuter = 1,
    Feminine = 2,
    Common = 3,
}
/// One of the main 3 Russian grammatical genders: [`Masculine`][Gender::Masculine],
/// [`Neuter`][Gender::Neuter], [`Feminine`][Gender::Feminine].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    #[default]
    Masculine = 0,
    Neuter = 1,
    Feminine = 2,
}

enum_conversion! {
    impl From<Gender, Error = GenderError> for GenderEx {
        Masculine, Neuter, Feminine,
    }
}
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("gender must be one of the main 3: masculine, neuter or feminine")]
pub struct GenderError;

/// A Russian grammatical animacy: inanimate or animate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Animacy {
    #[default]
    Inanimate = 0,
    Animate = 1,
}

// Traits providing GenderEx, Gender and Animacy values
#[const_trait]
pub trait HasGenderEx {
    fn gender_ex(&self) -> GenderEx;
}
#[const_trait]
pub trait HasGender {
    fn gender(&self) -> Gender;
}
#[const_trait]
pub trait HasAnimacy {
    fn animacy(&self) -> Animacy;

    fn is_animate(&self) -> bool {
        matches!(self.animacy(), Animacy::Animate)
    }
    fn is_inanimate(&self) -> bool {
        matches!(self.animacy(), Animacy::Inanimate)
    }
    fn acc_case(&self) -> Case {
        match self.animacy() {
            Animacy::Inanimate => Case::Nominative,
            Animacy::Animate => Case::Genitive,
        }
    }
}

// GenderEx, Gender and Animacy provides themselves
impl const HasGenderEx for GenderEx {
    fn gender_ex(&self) -> GenderEx {
        *self
    }
}
impl const HasGender for Gender {
    fn gender(&self) -> Gender {
        *self
    }
}
impl const HasAnimacy for Animacy {
    fn animacy(&self) -> Animacy {
        *self
    }
}
// Any type implementing HasGender implements HasGenderEx as well
impl<T: ~const HasGender> const HasGenderEx for T {
    fn gender_ex(&self) -> GenderEx {
        self.gender()._into()
    }
}

impl GenderEx {
    pub const fn normalize(self) -> Gender {
        match self {
            Self::Masculine => Gender::Masculine,
            Self::Neuter => Gender::Neuter,
            Self::Feminine | Self::Common => Gender::Feminine,
        }
    }
}
