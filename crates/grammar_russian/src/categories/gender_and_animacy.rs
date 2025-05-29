use thiserror::Error;

use crate::{
    Letter, LetterSliceExt,
    categories::*,
    letters,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderExAnimacy {
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderAnimacy {
    #[default]
    MasculineInanimate = 0,
    MasculineAnimate = 1,
    NeuterInanimate = 2,
    NeuterAnimate = 3,
    FeminineInanimate = 4,
    FeminineAnimate = 5,
}

enum_conversion! {
    impl From<GenderAnimacy, Error = GenderError> for GenderExAnimacy {
        MasculineInanimate, MasculineAnimate,
        NeuterInanimate, NeuterAnimate,
        FeminineInanimate, FeminineAnimate,
    }
}

impl GenderExAnimacy {
    pub const fn new(gender: GenderEx, animacy: Animacy) -> Self {
        let val = ((gender as u8) << 1) | animacy as u8;
        if val == 6 { Self::FeminineInanimate } else { unsafe { std::mem::transmute(val) } }
    }
}
impl_const_From!(<(GenderEx, Animacy)> for GenderExAnimacy {
    fn from(value: (GenderEx, Animacy)) -> Self {
        Self::new(value.0, value.1)
    }
});
impl const HasGenderEx for GenderExAnimacy {
    fn gender_ex(&self) -> GenderEx {
        unsafe { std::mem::transmute(*self as u8 >> 1) }
    }
}
impl const HasAnimacy for GenderExAnimacy {
    fn animacy(&self) -> Animacy {
        unsafe { std::mem::transmute(*self as u8 & 1) }
    }
}

impl GenderAnimacy {
    pub const fn new(gender: Gender, animacy: Animacy) -> Self {
        unsafe { std::mem::transmute(((gender as u8) << 1) | animacy as u8) }
    }
}
impl_const_From!(<(Gender, Animacy)> for GenderAnimacy {
    fn from(value: (Gender, Animacy)) -> Self {
        Self::new(value.0, value.1)
    }
});
impl const HasGender for GenderAnimacy {
    fn gender(&self) -> Gender {
        unsafe { std::mem::transmute(*self as u8 >> 1) }
    }
}
impl const HasAnimacy for GenderAnimacy {
    fn animacy(&self) -> Animacy {
        unsafe { std::mem::transmute(*self as u8 & 1) }
    }
}

impl GenderExAnimacy {
    pub const fn normalize(self) -> GenderAnimacy {
        match self {
            GenderExAnimacy::CommonAnimate => GenderAnimacy::FeminineAnimate,
            _ => unsafe { std::mem::transmute(self) },
        }
    }
}

impl GenderExAnimacy {
    pub const fn fmt_to(self, dst: &mut [u8; 9]) -> &str {
        let result: Result<GenderAnimacy, _> = self._try_into();
        match result {
            Ok(x) => x.fmt_to(dst.first_chunk_mut::<4>().unwrap()),
            Err(_) => {
                dst.copy_from_slice("мо-жо".as_bytes());
                unsafe { str::from_utf8_unchecked(dst) }
            },
        }
    }
}
impl GenderAnimacy {
    pub const fn fmt_to(self, dst: &mut [u8; 4]) -> &str {
        let dst: &mut [Letter; 2] = unsafe { std::mem::transmute(dst) };

        dst[0] = match self.gender() {
            Gender::Masculine => letters::м,
            Gender::Neuter => letters::с,
            Gender::Feminine => letters::ж,
        };
        dst[1] = letters::о;

        unsafe {
            str::from_utf8_unchecked(std::slice::from_raw_parts(
                dst.as_bytes().as_ptr(),
                if self.is_animate() { 4 } else { 2 },
            ))
        }
    }
}

impl std::fmt::Display for GenderExAnimacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; 9]).fmt(f)
    }
}
impl std::fmt::Display for GenderAnimacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; 4]).fmt(f)
    }
}
