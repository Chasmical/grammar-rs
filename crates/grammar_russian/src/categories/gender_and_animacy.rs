use thiserror::Error;

use crate::{Letter, categories::*, letters, util::*};

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

impl GenderEx {
    pub const fn normalize(self) -> Gender {
        if let Ok(x) = self._try_into() { x } else { Gender::Feminine }
    }
}
impl GenderExAnimacy {
    pub const fn normalize(self) -> GenderAnimacy {
        if let Ok(x) = self._try_into() { x } else { GenderAnimacy::FeminineAnimate }
    }
}

impl Gender {
    pub const fn fmt_to(self, dst: &mut [u8; 2]) -> &mut str {
        dst.copy_from_slice(match self.gender() {
            Gender::Masculine => letters::м.as_str().as_bytes(),
            Gender::Neuter => letters::с.as_str().as_bytes(),
            Gender::Feminine => letters::ж.as_str().as_bytes(),
        });
        unsafe { str::from_utf8_unchecked_mut(dst) }
    }
}
impl GenderAnimacy {
    pub const fn fmt_to(self, dst: &mut [u8; 4]) -> &str {
        self.gender().fmt_to(dst.first_chunk_mut::<2>().unwrap());
        dst.last_chunk_mut::<2>().unwrap().copy_from_slice("о".as_bytes());

        unsafe {
            str::from_utf8_unchecked(std::slice::from_raw_parts(
                dst.as_ptr(),
                if self.is_animate() { 4 } else { 2 },
            ))
        }
    }
}

impl GenderEx {
    pub const fn fmt_to(self, dst: &mut [u8; 5]) -> &mut str {
        let res: Result<Gender, _> = self._try_into();
        match res {
            Ok(x) => x.fmt_to(dst.first_chunk_mut::<2>().unwrap()),
            Err(_) => {
                dst.copy_from_slice("м-ж".as_bytes());
                unsafe { str::from_utf8_unchecked_mut(dst) }
            },
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

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; 2]).fmt(f)
    }
}
impl std::fmt::Display for GenderEx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; 5]).fmt(f)
    }
}
impl std::fmt::Display for GenderAnimacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; 4]).fmt(f)
    }
}
impl std::fmt::Display for GenderExAnimacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; 9]).fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseGenderError {
    Invalid,
}

impl Gender {
    pub const fn from_str(text: &str) -> Result<Self, ParseGenderError> {
        match Self::from_str_partial(text) {
            Ok((gender, len)) => {
                // Return Ok only if the entire string was parsed
                if len as usize == text.len() { Ok(gender) } else { Err(ParseGenderError::Invalid) }
            },
            Err(err) => Err(err),
        }
    }
    pub const fn from_str_partial(text: &str) -> Result<(Self, u8), ParseGenderError> {
        let text = Letter::from_bytes(text.as_bytes());
        match text {
            [letters::м, ..] => Ok((Self::Masculine, 2)),
            [letters::с, ..] => Ok((Self::Neuter, 2)),
            [letters::ж, ..] => Ok((Self::Feminine, 2)),
            _ => Err(ParseGenderError::Invalid),
        }
    }
}
impl GenderEx {
    pub const fn from_str(text: &str) -> Result<Self, ParseGenderError> {
        match Self::from_str_partial(text) {
            Ok((gender, len)) => {
                // Return Ok only if the entire string was parsed
                if len as usize == text.len() { Ok(gender) } else { Err(ParseGenderError::Invalid) }
            },
            Err(err) => Err(err),
        }
    }
    pub const fn from_str_partial(text: &str) -> Result<(Self, u8), ParseGenderError> {
        match text.as_bytes() {
            /* 'м-ж' */ [0xd0, 0xbc, b'-', 0xd0, 0xb6, ..] => Ok((Self::Common, 5)),
            /* 'м'   */ [0xd0, 0xbc, ..] => Ok((Self::Masculine, 2)),
            /* 'с'   */ [0xd1, 0x81, ..] => Ok((Self::Neuter, 2)),
            /* 'ж'   */ [0xd0, 0xb6, ..] => Ok((Self::Feminine, 2)),
            _ => Err(ParseGenderError::Invalid),
        }
    }
}

impl std::str::FromStr for Gender {
    type Err = ParseGenderError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl std::str::FromStr for GenderEx {
    type Err = ParseGenderError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl GenderAnimacy {
    pub const fn from_str(text: &str) -> Result<Self, ParseGenderError> {
        match Self::from_str_partial(text) {
            Ok((gender, len)) => {
                // Return Ok only if the entire string was parsed
                if len as usize == text.len() { Ok(gender) } else { Err(ParseGenderError::Invalid) }
            },
            Err(err) => Err(err),
        }
    }
    pub const fn from_str_partial(text: &str) -> Result<(Self, u8), ParseGenderError> {
        match text.as_bytes() {
            /* 'мо'    */ [0xd0, 0xbc, 0xd0, 0xbe] => Ok((Self::MasculineAnimate, 4)),
            /* 'м'     */ [0xd0, 0xbc] => Ok((Self::MasculineInanimate, 2)),
            /* 'со'    */ [0xd1, 0x81, 0xd0, 0xbe] => Ok((Self::NeuterAnimate, 4)),
            /* 'с'     */ [0xd1, 0x81] => Ok((Self::NeuterInanimate, 2)),
            /* 'жо'    */ [0xd0, 0xb6, 0xd0, 0xbe] => Ok((Self::FeminineAnimate, 4)),
            /* 'ж'     */ [0xd0, 0xb6] => Ok((Self::FeminineInanimate, 2)),
            _ => Err(ParseGenderError::Invalid),
        }
    }
}
impl GenderExAnimacy {
    pub const fn from_str(text: &str) -> Result<Self, ParseGenderError> {
        match Self::from_str_partial(text) {
            Ok((gender, len)) => {
                // Return Ok only if the entire string was parsed
                if len as usize == text.len() { Ok(gender) } else { Err(ParseGenderError::Invalid) }
            },
            Err(err) => Err(err),
        }
    }
    pub const fn from_str_partial(text: &str) -> Result<(Self, u8), ParseGenderError> {
        match text.as_bytes() {
            /* 'мо-жо' */
            [0xd0, 0xbc, 0xd0, 0xbe, b'-', 0xd0, 0xb6, 0xd0, 0xbe] => Ok((Self::CommonAnimate, 9)),
            /* 'мо'    */ [0xd0, 0xbc, 0xd0, 0xbe] => Ok((Self::MasculineAnimate, 4)),
            /* 'м'     */ [0xd0, 0xbc] => Ok((Self::MasculineInanimate, 2)),
            /* 'со'    */ [0xd1, 0x81, 0xd0, 0xbe] => Ok((Self::NeuterAnimate, 4)),
            /* 'с'     */ [0xd1, 0x81] => Ok((Self::NeuterInanimate, 2)),
            /* 'жо'    */ [0xd0, 0xb6, 0xd0, 0xbe] => Ok((Self::FeminineAnimate, 4)),
            /* 'ж'     */ [0xd0, 0xb6] => Ok((Self::FeminineInanimate, 2)),
            _ => Err(ParseGenderError::Invalid),
        }
    }
}

impl std::str::FromStr for GenderAnimacy {
    type Err = ParseGenderError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl std::str::FromStr for GenderExAnimacy {
    type Err = ParseGenderError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt() {
        assert_eq!(Gender::Masculine.to_string(), "м");
        assert_eq!(Gender::Neuter.to_string(), "с");
        assert_eq!(Gender::Feminine.to_string(), "ж");

        assert_eq!(GenderEx::Masculine.to_string(), "м");
        assert_eq!(GenderEx::Neuter.to_string(), "с");
        assert_eq!(GenderEx::Feminine.to_string(), "ж");
        assert_eq!(GenderEx::Common.to_string(), "м-ж");

        assert_eq!(GenderAnimacy::MasculineInanimate.to_string(), "м");
        assert_eq!(GenderAnimacy::MasculineAnimate.to_string(), "мо");
        assert_eq!(GenderAnimacy::NeuterInanimate.to_string(), "с");
        assert_eq!(GenderAnimacy::NeuterAnimate.to_string(), "со");
        assert_eq!(GenderAnimacy::FeminineInanimate.to_string(), "ж");
        assert_eq!(GenderAnimacy::FeminineAnimate.to_string(), "жо");

        assert_eq!(GenderExAnimacy::MasculineInanimate.to_string(), "м");
        assert_eq!(GenderExAnimacy::MasculineAnimate.to_string(), "мо");
        assert_eq!(GenderExAnimacy::NeuterInanimate.to_string(), "с");
        assert_eq!(GenderExAnimacy::NeuterAnimate.to_string(), "со");
        assert_eq!(GenderExAnimacy::FeminineInanimate.to_string(), "ж");
        assert_eq!(GenderExAnimacy::FeminineAnimate.to_string(), "жо");
        assert_eq!(GenderExAnimacy::CommonAnimate.to_string(), "мо-жо");
    }

    #[test]
    fn parse() {
        assert_eq!("м".parse(), Ok(Gender::Masculine));
        assert_eq!("с".parse(), Ok(Gender::Neuter));
        assert_eq!("ж".parse(), Ok(Gender::Feminine));

        assert_eq!("м".parse(), Ok(GenderEx::Masculine));
        assert_eq!("с".parse(), Ok(GenderEx::Neuter));
        assert_eq!("ж".parse(), Ok(GenderEx::Feminine));
        assert_eq!("м-ж".parse(), Ok(GenderEx::Common));

        assert_eq!("м".parse(), Ok(GenderAnimacy::MasculineInanimate));
        assert_eq!("мо".parse(), Ok(GenderAnimacy::MasculineAnimate));
        assert_eq!("с".parse(), Ok(GenderAnimacy::NeuterInanimate));
        assert_eq!("со".parse(), Ok(GenderAnimacy::NeuterAnimate));
        assert_eq!("ж".parse(), Ok(GenderAnimacy::FeminineInanimate));
        assert_eq!("жо".parse(), Ok(GenderAnimacy::FeminineAnimate));

        assert_eq!("м".parse(), Ok(GenderExAnimacy::MasculineInanimate));
        assert_eq!("мо".parse(), Ok(GenderExAnimacy::MasculineAnimate));
        assert_eq!("с".parse(), Ok(GenderExAnimacy::NeuterInanimate));
        assert_eq!("со".parse(), Ok(GenderExAnimacy::NeuterAnimate));
        assert_eq!("ж".parse(), Ok(GenderExAnimacy::FeminineInanimate));
        assert_eq!("жо".parse(), Ok(GenderExAnimacy::FeminineAnimate));
        assert_eq!("мо-жо".parse(), Ok(GenderExAnimacy::CommonAnimate));
    }
}
