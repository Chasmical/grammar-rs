use super::{animacy::*, gender::*};
use crate::util::{const_traits::*, enum_conversion};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderExAnimacy {
    #[default]
    MasculineInanimate = 0,
    MasculineAnimate = 1,
    NeuterInanimate = 2,
    NeuterAnimate = 3,
    FeminineInanimate = 4,
    FeminineAnimate = 5,
    // common inanimate isn't a thing, but 6 is reserved for it,
    // just so that CommonAnimate has the animacy bit set to 1.
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

enum_conversion!(GenderAnimacy => GenderExAnimacy [<= GenderError] {
    MasculineInanimate, MasculineAnimate,
    NeuterInanimate, NeuterAnimate,
    FeminineInanimate, FeminineAnimate,
});

impl GenderExAnimacy {
    pub const fn new(gender_ex: GenderEx, animacy: Animacy) -> Self {
        let result = ((gender_ex as u8) << 1) | animacy as u8;
        unsafe { std::mem::transmute(if result == 6 { 7 } else { result }) }
    }
}
impl GenderAnimacy {
    pub const fn new(gender: Gender, animacy: Animacy) -> Self {
        unsafe { std::mem::transmute(((gender as u8) << 1) | animacy as u8) }
    }
}

// Gender[Ex]Animacy provide Gender[Ex] and Animacy values
impl const HasGenderEx for GenderExAnimacy {
    fn gender_ex(&self) -> GenderEx {
        unsafe { std::mem::transmute((*self as u8) >> 1) }
    }
}
impl const HasGender for GenderAnimacy {
    fn gender(&self) -> Gender {
        unsafe { std::mem::transmute((*self as u8) >> 1) }
    }
}
impl const HasAnimacy for GenderExAnimacy {
    fn animacy(&self) -> Animacy {
        unsafe { std::mem::transmute((*self as u8) & 1) }
    }
}
impl const HasAnimacy for GenderAnimacy {
    fn animacy(&self) -> Animacy {
        unsafe { std::mem::transmute((*self as u8) & 1) }
    }
}

// Compose/decompose Gender[Ex]Animacy values
impl_const_From!(<(GenderEx, Animacy)> for GenderExAnimacy {
    fn from(value: (GenderEx, Animacy)) -> Self {
        Self::new(value.0, value.1)
    }
});
impl_const_From!(<(Gender, Animacy)> for GenderAnimacy {
    fn from(value: (Gender, Animacy)) -> Self {
        Self::new(value.0, value.1)
    }
});
impl GenderEx {
    pub const fn with_an(self, animacy: Animacy) -> GenderExAnimacy {
        GenderExAnimacy::new(self, animacy)
    }
}
impl Gender {
    pub const fn with_an(self, animacy: Animacy) -> GenderAnimacy {
        GenderAnimacy::new(self, animacy)
    }
}
impl GenderExAnimacy {
    pub const fn parts(self) -> (GenderEx, Animacy) {
        (self.gender_ex(), self.animacy())
    }
}
impl GenderAnimacy {
    pub const fn parts(self) -> (Gender, Animacy) {
        (self.gender(), self.animacy())
    }
}

// Gender[Ex]Animacy abbreviation constants
impl GenderExAnimacy {
    pub const MASC_INAN: Self = Self::MasculineInanimate;
    pub const MASC_AN: Self = Self::MasculineAnimate;
    pub const NEUT_INAN: Self = Self::NeuterInanimate;
    pub const NEUT_AN: Self = Self::NeuterAnimate;
    pub const FEM_INAN: Self = Self::FeminineInanimate;
    pub const FEM_AN: Self = Self::FeminineAnimate;
}
impl GenderAnimacy {
    pub const MASC_INAN: Self = Self::MasculineInanimate;
    pub const MASC_AN: Self = Self::MasculineAnimate;
    pub const NEUT_INAN: Self = Self::NeuterInanimate;
    pub const NEUT_AN: Self = Self::NeuterAnimate;
    pub const FEM_INAN: Self = Self::FeminineInanimate;
    pub const FEM_AN: Self = Self::FeminineAnimate;
}

impl GenderExAnimacy {
    pub const fn abbr_zaliznyak(self) -> &'static str {
        match self {
            Self::MasculineInanimate => "м",
            Self::MasculineAnimate => "мо",
            Self::NeuterInanimate => "с",
            Self::NeuterAnimate => "со",
            Self::FeminineInanimate => "ж",
            Self::FeminineAnimate => "жо",
            Self::CommonAnimate => "мо-жо",
        }
    }
}
impl GenderAnimacy {
    pub const fn abbr_zaliznyak(self) -> &'static str {
        GenderExAnimacy::_from(self).abbr_zaliznyak()
    }
}

impl std::fmt::Display for GenderExAnimacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.gender_ex(), self.animacy())
    }
}
impl std::fmt::Display for GenderAnimacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.gender(), self.animacy())
    }
}
