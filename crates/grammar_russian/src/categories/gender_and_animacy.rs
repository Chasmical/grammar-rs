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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderExAndAnimacy {
    #[default]
    MasculineInanimate = 0,
    MasculineAnimate = 1,
    NeuterInanimate = 2,
    NeuterAnimate = 3,
    FeminineInanimate = 4,
    FeminineAnimate = 5,
    // CommonInanimate is not a thing, but 6 is still reserved for it,
    // primarily so that CommonAnimate has the last animacy bit set.
    CommonAnimate = 7,
}
/// A composite of both [`Gender`] and [`Animacy`] as one value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderAndAnimacy {
    #[default]
    MasculineInanimate = 0,
    MasculineAnimate = 1,
    NeuterInanimate = 2,
    NeuterAnimate = 3,
    FeminineInanimate = 4,
    FeminineAnimate = 5,
}

enum_conversion! {
    impl From<GenderAndAnimacy, Error = GenderError> for GenderExAndAnimacy {
        MasculineInanimate, MasculineAnimate,
        NeuterInanimate, NeuterAnimate,
        FeminineInanimate, FeminineAnimate,
    }
}

// Constructing and deconstructing GenderExAndAnimacy
impl GenderExAndAnimacy {
    pub const fn new(gender: GenderEx, animacy: Animacy) -> Self {
        unsafe { std::mem::transmute(((gender as u8) << 1) | animacy as u8) }
    }
}
impl GenderEx {
    pub const fn with_an(self, animacy: Animacy) -> GenderExAndAnimacy {
        GenderExAndAnimacy::new(self, animacy)
    }
}
impl const HasGender for GenderExAndAnimacy {
    fn gender(&self) -> Gender {
        unsafe { std::mem::transmute(*self as u8 >> 1) }
    }
}
impl const HasAnimacy for GenderExAndAnimacy {
    fn animacy(&self) -> Animacy {
        unsafe { std::mem::transmute(*self as u8 & 1) }
    }
}

// Constructing and deconstructing GenderAndAnimacy
impl GenderAndAnimacy {
    pub const fn new(gender: Gender, animacy: Animacy) -> Self {
        unsafe { std::mem::transmute(((gender as u8) << 1) | animacy as u8) }
    }
}
impl Gender {
    pub const fn with_an(self, animacy: Animacy) -> GenderAndAnimacy {
        GenderAndAnimacy::new(self, animacy)
    }
}
impl const HasGender for GenderAndAnimacy {
    fn gender(&self) -> Gender {
        unsafe { std::mem::transmute(*self as u8 >> 1) }
    }
}
impl const HasAnimacy for GenderAndAnimacy {
    fn animacy(&self) -> Animacy {
        unsafe { std::mem::transmute(*self as u8 & 1) }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderOrPlural {
    #[default]
    Masculine = 0,
    Neuter = 1,
    Feminine = 2,
    Plural = 3,
}

enum_conversion! {
    impl From<Gender, Error = GenderError> for GenderOrPlural {
        Masculine, Neuter, Feminine,
    }
}

impl GenderOrPlural {
    pub const fn new(gender: Gender, number: Number) -> Self {
        match number {
            Number::Singular => gender._into(),
            Number::Plural => Self::Plural,
        }
    }
}
impl Gender {
    pub const fn with_num(self, number: Number) -> GenderOrPlural {
        GenderOrPlural::new(self, number)
    }
}
impl GenderOrPlural {
    pub const fn gender(self) -> Option<Gender> {
        Some(match self {
            Self::Masculine => Gender::Masculine,
            Self::Neuter => Gender::Neuter,
            Self::Feminine => Gender::Feminine,
            Self::Plural => return None,
        })
    }
}
impl const HasNumber for GenderOrPlural {
    fn number(&self) -> Number {
        match self {
            Self::Plural => Number::Plural,
            _ => Number::Singular,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderOrPluralAndAnimacy {
    #[default]
    MasculineInanimate = 0,
    MasculineAnimate = 1,
    NeuterInanimate = 2,
    NeuterAnimate = 3,
    FeminineInanimate = 4,
    FeminineAnimate = 5,
    PluralInanimate = 6,
    PluralAnimate = 7,
}

impl GenderOrPluralAndAnimacy {
    pub const fn new(gender_or_plural: GenderOrPlural, animacy: Animacy) -> Self {
        unsafe { std::mem::transmute(((gender_or_plural as u8) << 1) | animacy as u8) }
    }
}
impl GenderOrPlural {
    pub const fn with_an(self, animacy: Animacy) -> GenderOrPluralAndAnimacy {
        GenderOrPluralAndAnimacy::new(self, animacy)
    }
}
impl GenderAndAnimacy {
    pub const fn with_num(self, number: Number) -> GenderOrPluralAndAnimacy {
        GenderOrPluralAndAnimacy::new(GenderOrPlural::new(self.gender(), number), self.animacy())
    }
}
impl GenderOrPluralAndAnimacy {
    pub const fn gender_or_plural(self) -> GenderOrPlural {
        unsafe { std::mem::transmute(self as u8 >> 1) }
    }
}
impl const HasAnimacy for GenderOrPluralAndAnimacy {
    fn animacy(&self) -> Animacy {
        unsafe { std::mem::transmute(*self as u8 & 1) }
    }
}
impl const HasNumber for GenderOrPluralAndAnimacy {
    fn number(&self) -> Number {
        match self {
            Self::PluralInanimate | Self::PluralAnimate => Number::Plural,
            _ => Number::Singular,
        }
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
impl GenderExAndAnimacy {
    pub const fn normalize(self) -> GenderAndAnimacy {
        match self {
            Self::MasculineInanimate => GenderAndAnimacy::MasculineInanimate,
            Self::MasculineAnimate => GenderAndAnimacy::MasculineAnimate,
            Self::NeuterInanimate => GenderAndAnimacy::NeuterInanimate,
            Self::NeuterAnimate => GenderAndAnimacy::NeuterAnimate,
            Self::FeminineInanimate => GenderAndAnimacy::FeminineInanimate,
            Self::FeminineAnimate | Self::CommonAnimate => GenderAndAnimacy::FeminineAnimate,
        }
    }
}
