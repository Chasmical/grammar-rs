use super::{Animacy, Case, CaseEx, Gender, GenderAnimacy, GenderEx, GenderExAnimacy, Number};
use crate::util::*;

// Traits providing CaseEx and Case values
#[const_trait]
pub trait HasCaseEx {
    fn case_ex(&self) -> CaseEx;
}
#[const_trait]
pub trait HasCase {
    fn case(&self) -> Case;
}

// Traits providing GenderEx and Gender values
#[const_trait]
pub trait HasGenderEx {
    fn gender_ex(&self) -> GenderEx;
}
#[const_trait]
pub trait HasGender {
    fn gender(&self) -> Gender;
}

// Traits providing Animacy and Number values
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
#[const_trait]
pub trait HasNumber {
    fn number(&self) -> Number;

    fn is_singular(&self) -> bool {
        matches!(self.number(), Number::Singular)
    }
    fn is_plural(&self) -> bool {
        matches!(self.number(), Number::Plural)
    }
}

// All values provide themselves
impl const HasCaseEx for CaseEx {
    fn case_ex(&self) -> CaseEx {
        *self
    }
}
impl const HasCase for Case {
    fn case(&self) -> Case {
        *self
    }
}
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
impl const HasNumber for Number {
    fn number(&self) -> Number {
        *self
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

// Any type implementing HasCase implements HasCaseEx as well
impl<T: ~const HasCase> const HasCaseEx for T {
    fn case_ex(&self) -> CaseEx {
        self.case()._into()
    }
}
// Any type implementing HasGender implements HasGenderEx as well
impl<T: ~const HasGender> const HasGenderEx for T {
    fn gender_ex(&self) -> GenderEx {
        self.gender()._into()
    }
}
