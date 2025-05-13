use super::HasAnimacy;

/// Represents one of the main 6 Russian grammatical cases.
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

/// Represents a Russian grammatical case (including secondary ones).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

/// Represents a Russian grammatical number.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    #[default]
    Singular = 0,
    Plural = 1,
}

/// A composite of both [`Case`] and [`Number`] as one value.
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
/// A composite of both [`CaseEx`] and [`Number`] as one value.
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
        self.case().as_ex()
    }
}

// Casting Case to CaseEx
impl Case {
    pub const fn as_ex(self) -> CaseEx {
        unsafe { std::mem::transmute(self) }
    }
}
impl From<Case> for CaseEx {
    fn from(value: Case) -> Self {
        value.as_ex()
    }
}

// Combining Case and CaseEx with Number
impl Case {
    pub const fn with(self, number: Number) -> CaseAndNumber {
        CaseAndNumber::new(self, number)
    }
}
impl CaseEx {
    pub const fn with(self, number: Number) -> CaseExAndNumber {
        CaseExAndNumber::new(self, number)
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

// Casting CaseAndNumber to CaseExAndNumber
impl CaseAndNumber {
    pub const fn as_ex(self) -> CaseExAndNumber {
        unsafe { std::mem::transmute(self) }
    }
}
impl From<CaseAndNumber> for CaseExAndNumber {
    fn from(value: CaseAndNumber) -> Self {
        value.as_ex()
    }
}
// Normalizing CaseExAndNumber to CaseAndNumber
impl CaseExAndNumber {
    pub const fn normalize(self) -> CaseAndNumber {
        match self.case_ex() {
            CaseEx::Partitive => CaseAndNumber::new(Case::Genitive, self.number()),
            CaseEx::Translative => CaseAndNumber::NominativePlural,
            CaseEx::Locative => CaseAndNumber::new(Case::Prepositional, self.number()),
            _ => unsafe { std::mem::transmute(self) },
        }
    }
}
impl From<CaseExAndNumber> for CaseAndNumber {
    fn from(value: CaseExAndNumber) -> Self {
        value.normalize()
    }
}

impl Case {
    pub fn is_nom_normalized(self, info: impl HasAnimacy) -> bool {
        match self {
            Case::Nominative => true,
            Case::Accusative => info.is_inanimate(),
            _ => false,
        }
    }
    pub fn is_gen_normalized(self, info: impl HasAnimacy) -> bool {
        match self {
            Case::Genitive => true,
            Case::Accusative => info.is_animate(),
            _ => false,
        }
    }
}
