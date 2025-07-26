use super::{
    Animacy, Case, CaseEx, Gender, GenderAnimacy, GenderEx, GenderExAnimacy, Number,
    traits::{HasAnimacy, HasGender, HasGenderEx},
};

impl CaseEx {
    pub const fn normalize_with(self, number: Number) -> (Case, Number) {
        match self {
            CaseEx::Partitive => (Case::Genitive, number),
            CaseEx::Translative => (Case::Nominative, Number::Plural),
            CaseEx::Locative => (Case::Prepositional, number),
            _ => (unsafe { std::mem::transmute::<CaseEx, Case>(self) }, number),
        }
    }
}
impl Case {
    pub const fn acc_is_nom<A>(self, animacy: A) -> Option<bool>
    where A: [const] HasAnimacy + [const] std::marker::Destruct {
        match self {
            Self::Nominative => Some(true),
            Self::Genitive => Some(false),
            Self::Accusative => Some(animacy.is_inanimate()),
            _ => None,
        }
    }
    pub const fn is_nom_or_acc_inan<A>(self, animacy: A) -> bool
    where A: [const] HasAnimacy + [const] std::marker::Destruct {
        self.acc_is_nom(animacy) == Some(true)
    }
    pub const fn is_gen_or_acc_an<A>(self, animacy: A) -> bool
    where A: [const] HasAnimacy + [const] std::marker::Destruct {
        self.acc_is_nom(animacy) == Some(false)
    }
}

impl GenderEx {
    pub const fn normalize(self) -> Gender {
        // FIXME(const-hack): Replace `try_into()` with `unwrap_or()` when it's constified.
        if let Ok(x) = self.try_into() { x } else { Gender::Feminine }
    }
}

impl Animacy {
    pub const fn acc_case(self) -> Case {
        match self {
            Animacy::Inanimate => Case::Nominative,
            Animacy::Animate => Case::Genitive,
        }
    }
}

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

// Compose/decompose Gender[Ex]Animacy values
impl const From<(GenderEx, Animacy)> for GenderExAnimacy {
    fn from(value: (GenderEx, Animacy)) -> Self {
        Self::new(value.0, value.1)
    }
}
impl const From<(Gender, Animacy)> for GenderAnimacy {
    fn from(value: (Gender, Animacy)) -> Self {
        Self::new(value.0, value.1)
    }
}
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
