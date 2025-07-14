use crate::util::*;
use thiserror::Error;

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

enum_conversion!(Gender => GenderEx [<= GenderError] {
    Masculine, Neuter, Feminine,
});
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("gender must be one of the main 3: masculine, neuter or feminine")]
pub struct GenderError;

// Traits providing GenderEx and Gender values
#[const_trait]
pub trait HasGenderEx {
    fn gender_ex(&self) -> GenderEx;
}
#[const_trait]
pub trait HasGender {
    fn gender(&self) -> Gender;
}

// GenderEx and Gender provides themselves
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
// Any type implementing HasGender implements HasGenderEx as well
impl<T: ~const HasGender> const HasGenderEx for T {
    fn gender_ex(&self) -> GenderEx {
        self.gender()._into()
    }
}

impl GenderEx {
    pub const fn normalize(self) -> Gender {
        if let Ok(x) = self._try_into() { x } else { Gender::Feminine }
    }
}

// Gender abbreviation constants
impl GenderEx {
    pub const MASC: Self = Self::Masculine;
    pub const NEUT: Self = Self::Neuter;
    pub const FEM: Self = Self::Feminine;
}
impl Gender {
    pub const MASC: Self = Self::Masculine;
    pub const NEUT: Self = Self::Neuter;
    pub const FEM: Self = Self::Feminine;
}

impl GenderEx {
    pub const fn abbr_upper(self) -> &'static str {
        match self {
            Self::Masculine => "MASC",
            Self::Neuter => "NEUT",
            Self::Feminine => "FEM",
            Self::Common => "MASC/FEM",
        }
    }
    pub const fn abbr_lower(self) -> &'static str {
        match self {
            Self::Masculine => "masc",
            Self::Neuter => "neut",
            Self::Feminine => "fem",
            Self::Common => "masc/fem",
        }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        // Note: small caps 'ꜰ' (U+A730) may not render correctly in some fonts.
        match self {
            Self::Masculine => "ᴍᴀꜱᴄ",
            Self::Neuter => "ɴᴇᴜᴛ",
            Self::Feminine => "ꜰᴇᴍ",
            Self::Common => "ᴍᴀꜱᴄ/ꜰᴇᴍ",
        }
    }
}
impl Gender {
    pub const fn abbr_upper(self) -> &'static str {
        GenderEx::_from(self).abbr_upper()
    }
    pub const fn abbr_lower(self) -> &'static str {
        GenderEx::_from(self).abbr_lower()
    }
    pub const fn abbr_smcp(self) -> &'static str {
        GenderEx::_from(self).abbr_smcp()
    }
}

impl std::fmt::Display for GenderEx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr_upper().fmt(f)
    }
}
impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr_upper().fmt(f)
    }
}
