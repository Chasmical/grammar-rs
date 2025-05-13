/// Represents a Russian grammatical gender.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    #[default]
    Masculine = 0,
    Neuter = 1,
    Feminine = 2,
    Common = 3,
}

/// Represents a Russian grammatical animacy.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Animacy {
    #[default]
    Inanimate = 0,
    Animate = 1,
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
    // CommonInanimate is not a thing, but 6 is still reserved for it,
    // primarily so that CommonAnimate has the last animacy bit set.
    CommonAnimate = 7,
}

// Traits providing Gender and Animacy values
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
}

// Gender and Animacy provides themselves
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

// Combining Gender with Animacy
impl Gender {
    pub const fn with(self, animacy: Animacy) -> GenderAndAnimacy {
        GenderAndAnimacy::new(self, animacy)
    }
}

// Constructing and deconstructing GenderAndAnimacy
impl GenderAndAnimacy {
    pub const fn try_new(gender: Gender, animacy: Animacy) -> Option<GenderAndAnimacy> {
        match (gender, animacy) {
            (Gender::Common, Animacy::Inanimate) => None,
            _ => Some(unsafe { std::mem::transmute(((gender as u8) << 1) | animacy as u8) }),
        }
    }
    pub const fn new(gender: Gender, animacy: Animacy) -> Self {
        Self::try_new(gender, animacy).unwrap()
    }
}
impl From<(Gender, Animacy)> for GenderAndAnimacy {
    fn from(value: (Gender, Animacy)) -> Self {
        Self::new(value.0, value.1)
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
