use thiserror::Error;

use super::{HasAnimacy, Number};

use crate::util::{const_traits::*, enum_conversion};

/// A main or secondary Russian grammatical case.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CaseEx {
    #[default]
    Nominative = 0,
    Genitive = 1,
    Dative = 2,
    Accusative = 3,
    Instrumental = 4,
    Prepositional = 5,

    Partitive = 6,
    Translative = 7,
    Locative = 8,
}
/// One of the main 6 Russian grammatical cases.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    #[default]
    Nominative = 0,
    Genitive = 1,
    Dative = 2,
    Accusative = 3,
    Instrumental = 4,
    Prepositional = 5,
}

enum_conversion! {
    impl From<Case, Error = CaseError> for CaseEx {
        Nominative, Genitive, Dative, Accusative, Instrumental, Prepositional,
    }
}
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error(
    "case must be one of the main 6: nominative, genitive, dative, accusative, instrumental or prepositional"
)]
pub struct CaseError;

// Traits providing CaseEx and Case values
#[const_trait]
pub trait HasCaseEx {
    fn case_ex(&self) -> CaseEx;
}
#[const_trait]
pub trait HasCase {
    fn case(&self) -> Case;
}

// CaseEx and Case provide themselves
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
// Any type implementing HasCase implements HasCaseEx as well
impl<T: ~const HasCase> const HasCaseEx for T {
    fn case_ex(&self) -> CaseEx {
        self.case()._into()
    }
}

impl CaseEx {
    // TODO: resolve conflicting into impl???
    pub const fn normalize_with(self, number: Number) -> (Case, Number) {
        match self {
            CaseEx::Partitive => (Case::Genitive, number),
            CaseEx::Translative => (Case::Nominative, Number::Plural),
            CaseEx::Locative => (Case::Prepositional, number),
            _ => (unsafe { std::mem::transmute(self) }, number),
        }
    }
}

impl Case {
    pub const fn acc_is_nom(self, animacy: impl ~const HasAnimacy + Copy) -> Option<bool> {
        match self {
            Self::Nominative => Some(true),
            Self::Genitive => Some(false),
            Self::Accusative => Some(animacy.is_inanimate()),
            _ => None,
        }
    }
    pub const fn is_nom_or_acc_inan(self, animacy: impl ~const HasAnimacy + Copy) -> bool {
        matches!(self.acc_is_nom(animacy), Some(true))
    }
    pub const fn is_gen_or_acc_an(self, animacy: impl ~const HasAnimacy + Copy) -> bool {
        matches!(self.acc_is_nom(animacy), Some(false))
    }
}
