use crate::util::{_Into, define_error, enum_conversion};

use super::HasAnimacy;

/// One of the main or secondary Russian grammatical cases.
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

define_error! {
    pub struct CaseError("TODO");
}
enum_conversion! {
    impl From<Case, Error = CaseError> for CaseEx {
        Nominative, Genitive, Dative, Accusative, Instrumental, Prepositional,
    }
}

/// A Russian grammatical number.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    #[default]
    Singular = 0,
    Plural = 1,
}

/// [`CaseEx`] and [`Number`] as one value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CaseExAndNumber {
    #[default]
    NominativeSingular = 0,
    NominativePlural = 1,
    GenitiveSingular = 2,
    GenitivePlural = 3,
    DativeSingular = 4,
    DativePlural = 5,
    AccusativeSingular = 6,
    AccusativePlural = 7,
    InstrumentalSingular = 8,
    InstrumentalPlural = 9,
    PrepositionalSingular = 10,
    PrepositionalPlural = 11,

    PartitiveSingular = 12,
    PartitivePlural = 13,
    TranslativeSingular = 14,
    TranslativePlural = 15,
    LocativeSingular = 16,
    LocativePlural = 17,
}
/// [`Case`] and [`Number`] as one value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CaseAndNumber {
    #[default]
    NominativeSingular = 0,
    NominativePlural = 1,
    GenitiveSingular = 2,
    GenitivePlural = 3,
    DativeSingular = 4,
    DativePlural = 5,
    AccusativeSingular = 6,
    AccusativePlural = 7,
    InstrumentalSingular = 8,
    InstrumentalPlural = 9,
    PrepositionalSingular = 10,
    PrepositionalPlural = 11,
}

enum_conversion! {
    impl From<CaseAndNumber, Error = CaseError> for CaseExAndNumber {
        NominativeSingular, NominativePlural,
        GenitiveSingular, GenitivePlural,
        DativeSingular, DativePlural,
        AccusativeSingular, AccusativePlural,
        InstrumentalSingular, InstrumentalPlural,
        PrepositionalSingular, PrepositionalPlural,
    }
}

// Traits providing Case, CaseEx and Number values
#[const_trait]
pub trait HasCase {
    fn case(&self) -> Case;
}
#[const_trait]
pub trait HasCaseEx {
    fn case_ex(&self) -> CaseEx;
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

// Case, CaseEx and Number provide themselves
impl const HasCase for Case {
    fn case(&self) -> Case {
        *self
    }
}
impl const HasCaseEx for CaseEx {
    fn case_ex(&self) -> CaseEx {
        *self
    }
}
impl const HasNumber for Number {
    fn number(&self) -> Number {
        *self
    }
}
// Any type implementing HasCase implements HasCaseEx as well
impl<T: ~const HasCase> const HasCaseEx for T {
    fn case_ex(&self) -> CaseEx {
        // FIXME(const-hack): Replace with into().
        self.case()._into()
    }
}

// Constructing and deconstructing CaseAndNumber
impl CaseAndNumber {
    pub const fn new(case: Case, number: Number) -> Self {
        unsafe { std::mem::transmute(((case as u8) << 1) | number as u8) }
    }
}
impl From<(Case, Number)> for CaseAndNumber {
    fn from(value: (Case, Number)) -> Self {
        Self::new(value.0, value.1)
    }
}
impl Case {
    pub const fn with(self, number: Number) -> CaseAndNumber {
        CaseAndNumber::new(self, number)
    }
}
impl const HasCase for CaseAndNumber {
    fn case(&self) -> Case {
        unsafe { std::mem::transmute(*self as u8 >> 1) }
    }
}
impl const HasNumber for CaseAndNumber {
    fn number(&self) -> Number {
        unsafe { std::mem::transmute(*self as u8 & 1) }
    }
}

// Constructing and deconstructing CaseExAndNumber
impl CaseExAndNumber {
    pub const fn new(case_ex: CaseEx, number: Number) -> Self {
        unsafe { std::mem::transmute(((case_ex as u8) << 1) | number as u8) }
    }
}
impl From<(CaseEx, Number)> for CaseExAndNumber {
    fn from(value: (CaseEx, Number)) -> Self {
        Self::new(value.0, value.1)
    }
}
impl CaseEx {
    pub const fn with(self, number: Number) -> CaseExAndNumber {
        CaseExAndNumber::new(self, number)
    }
}
impl const HasCaseEx for CaseExAndNumber {
    fn case_ex(&self) -> CaseEx {
        unsafe { std::mem::transmute(*self as u8 >> 1) }
    }
}
impl const HasNumber for CaseExAndNumber {
    fn number(&self) -> Number {
        unsafe { std::mem::transmute(*self as u8 & 1) }
    }
}

impl CaseExAndNumber {
    // TODO: resolve conflicting into impl???
    pub const fn normalize(self) -> CaseAndNumber {
        match self.case_ex() {
            CaseEx::Partitive => CaseAndNumber::new(Case::Genitive, self.number()),
            CaseEx::Translative => CaseAndNumber::NominativePlural,
            CaseEx::Locative => CaseAndNumber::new(Case::Prepositional, self.number()),
            _ => unsafe { std::mem::transmute(self) },
        }
    }
}

impl Case {
    pub const fn is_nom_or_acc_inan(self, animacy: impl ~const HasAnimacy + Copy) -> bool {
        match self {
            Self::Nominative => true,
            Self::Accusative => animacy.is_inanimate(),
            _ => false,
        }
    }
}
