use crate::{
    stress::{
        AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress,
        NounStress, PronounStress, VerbPastStress, VerbPresentStress, VerbStress,
    },
    util::{const_traits::const_try, enum_conversion},
};
use thiserror::Error;

#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("words can only have stresses a-f, a′-f′, c″ and f″")]
pub struct AnyStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("nouns can only have stresses a, b, c, d, e, f, b′, d′, f′ and f″")]
pub struct NounStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("pronouns can only have stresses a, b and f")]
pub struct PronounStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("adjectives (full form) can only have stresses a and b")]
pub struct AdjectiveFullStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("adjectives (short form) can only have stresses a, b, c, a′, b′, c′ and c″")]
pub struct AdjectiveShortStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("verbs (present tense) can only have stresses a, b, c and c′")]
pub struct VerbPresentStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("verbs (past tense) can only have stresses a, b, c, c′ and c″")]
pub struct VerbPastStressError;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum AdjectiveStressError {
    #[error("{0}")]
    Full(#[from] AdjectiveFullStressError),
    #[error("{0}")]
    Short(#[from] AdjectiveShortStressError),
}
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum VerbStressError {
    #[error("{0}")]
    Present(#[from] VerbPresentStressError),
    #[error("{0}")]
    Past(#[from] VerbPastStressError),
}

//                         TABLE OF STRESS TYPE CONVERSIONS
// ┌———————┬——————┬——————┬——————┬——————┬——————┬——————┬——————╥——————┬——————┬——————┐
// │From\To│ Any  │ Noun │ Pro  │ AdjF │ AdjS │ VerbF│ VerbP║ ANY  │ ADJ  │ VERB │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Any   │ ———— │  []  │  []  │  []  │  []  │  []  │  []  ║  ██  │  []  │  []  │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Noun  │  ██  │ ———— │      │      │      │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Pro   │  ██  │      │ ———— │      │      │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ AdjF  │  ██  │      │      │ ———— │      │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ AdjS  │  ██  │      │      │      │ ———— │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VerbF │  ██  │      │      │      │      │ ———— │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VerbP │  ██  │      │      │      │      │      │ ———— ║  ██  │      │      │
// ╞═══════╪══════╪══════╪══════╪══════╪══════╪══════╪══════╬══════╪══════╪══════╡
// │ ANY   │  []  │  []  │  []  │  []  │  []  │  []  │  []  ║ ———— │  []  │  []  │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ ADJ   │      │      │      │      │      │      │      ║  ██  │ ———— │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VERB  │      │      │      │      │      │      │      ║  ██  │      │ ———— │
// └———————┴——————┴——————┴——————┴——————┴——————┴——————┴——————╨——————┴——————┴——————┘
//                                                     ██ — From   [] — TryFrom

// Convert simple stresses to AnyStress, and vice versa
enum_conversion!(NounStress => AnyStress [<= NounStressError] {
    A, B, C, D, E, F, Bp, Dp, Fp, Fpp,
});
enum_conversion!(PronounStress => AnyStress [<= PronounStressError] {
    A, B, F,
});
enum_conversion!(AdjectiveFullStress => AnyStress [<= AdjectiveFullStressError] {
    A, B,
});
enum_conversion!(AdjectiveShortStress => AnyStress [<= AdjectiveShortStressError] {
    A, B, C, Ap, Bp, Cp, Cpp,
});
enum_conversion!(VerbPresentStress => AnyStress [<= VerbPresentStressError] {
    A, B, C, Cp,
});
enum_conversion!(VerbPastStress => AnyStress [<= VerbPastStressError] {
    A, B, C, Cp, Cpp,
});

// Convert any simple stresses into AnyDualStress
impl<T: [const] Into<AnyStress>> const From<T> for AnyDualStress {
    fn from(value: T) -> Self {
        Self::new(value.into(), None)
    }
}
// Convert AdjectiveStress and VerbStress into AnyDualStress
impl const From<AdjectiveStress> for AnyDualStress {
    fn from(value: AdjectiveStress) -> Self {
        Self::new(value.full.into(), Some(value.short.into()))
    }
}
impl const From<VerbStress> for AnyDualStress {
    fn from(value: VerbStress) -> Self {
        Self::new(value.present.into(), Some(value.past.into()))
    }
}

// Try to convert main-only AnyDualStress into simple stresses
impl const TryFrom<AnyDualStress> for AnyStress {
    type Error = AnyStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Ok(value.main) } else { Err(Self::Error {}) }
    }
}
impl const TryFrom<AnyDualStress> for NounStress {
    type Error = NounStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::try_from(value.main) } else { Err(Self::Error {}) }
    }
}
impl const TryFrom<AnyDualStress> for PronounStress {
    type Error = PronounStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::try_from(value.main) } else { Err(Self::Error {}) }
    }
}
impl const TryFrom<AnyDualStress> for AdjectiveFullStress {
    type Error = AdjectiveFullStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::try_from(value.main) } else { Err(Self::Error {}) }
    }
}
impl const TryFrom<AnyDualStress> for AdjectiveShortStress {
    type Error = AdjectiveShortStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::try_from(value.main) } else { Err(Self::Error {}) }
    }
}
impl const TryFrom<AnyDualStress> for VerbPresentStress {
    type Error = VerbPresentStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::try_from(value.main) } else { Err(Self::Error {}) }
    }
}
impl const TryFrom<AnyDualStress> for VerbPastStress {
    type Error = VerbPastStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::try_from(value.main) } else { Err(Self::Error {}) }
    }
}

// Try to convert AnyDualStress to AdjectiveStress and VerbStress
impl const TryFrom<AnyDualStress> for AdjectiveStress {
    type Error = AdjectiveStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        let (main, alt) = value.normalize_adj();

        Ok(Self::new(
            const_try!(main.try_into(), Self::Error::Full),
            const_try!(alt.try_into(), Self::Error::Short),
        ))
    }
}
impl const TryFrom<AnyDualStress> for VerbStress {
    type Error = VerbStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        let (main, alt) = value.normalize_verb();

        Ok(Self::new(
            const_try!(main.try_into(), Self::Error::Present),
            const_try!(alt.try_into(), Self::Error::Past),
        ))
    }
}
