use super::defs::*;
use crate::util::*;

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
impl<T: Into<AnyStress>> From<T> for AnyDualStress {
    fn from(value: T) -> Self {
        Self::new(value.into(), None)
    }
}
impl<T: ~const _Into<AnyStress>> const _From<T> for AnyDualStress {
    fn _from(value: T) -> Self {
        Self::new(value._into(), None)
    }
}
// Convert AdjectiveStress and VerbStress into AnyDualStress
impl_const_From!(<AdjectiveStress> for AnyDualStress {
    fn from(value: AdjectiveStress) -> Self {
        Self::new(value.full._into(), Some(value.short._into()))
    }
});
impl_const_From!(<VerbStress> for AnyDualStress {
    fn from(value: VerbStress) -> Self {
        Self::new(value.present._into(), Some(value.past._into()))
    }
});

// Try to convert main-only AnyDualStress into simple stresses
impl_const_TryFrom!(<AnyDualStress> for AnyStress {
    type Error = AnyStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Ok(value.main) } else { Err(Self::Error {}) }
    }
});
impl_const_TryFrom!(<AnyDualStress> for NounStress {
    type Error = NounStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::_try_from(value.main) } else { Err(Self::Error {}) }
    }
});
impl_const_TryFrom!(<AnyDualStress> for PronounStress {
    type Error = PronounStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::_try_from(value.main) } else { Err(Self::Error {}) }
    }
});
impl_const_TryFrom!(<AnyDualStress> for AdjectiveFullStress {
    type Error = AdjectiveFullStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::_try_from(value.main) } else { Err(Self::Error {}) }
    }
});
impl_const_TryFrom!(<AnyDualStress> for AdjectiveShortStress {
    type Error = AdjectiveShortStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::_try_from(value.main) } else { Err(Self::Error {}) }
    }
});
impl_const_TryFrom!(<AnyDualStress> for VerbPresentStress {
    type Error = VerbPresentStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::_try_from(value.main) } else { Err(Self::Error {}) }
    }
});
impl_const_TryFrom!(<AnyDualStress> for VerbPastStress {
    type Error = VerbPastStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Self::_try_from(value.main) } else { Err(Self::Error {}) }
    }
});

// Try to convert AnyDualStress to AdjectiveStress and VerbStress
impl_const_TryFrom!(<AnyDualStress> for AdjectiveStress {
    type Error = AdjectiveStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        let (main, alt) = value.normalize_adj();

        Ok(Self::new(
            const_try!(main._try_into(), Self::Error::Full),
            const_try!(alt._try_into(), Self::Error::Short),
        ))
    }
});
impl_const_TryFrom!(<AnyDualStress> for VerbStress {
    type Error = VerbStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        let (main, alt) = value.normalize_verb();

        Ok(Self::new(
            const_try!(main._try_into(), Self::Error::Present),
            const_try!(alt._try_into(), Self::Error::Past),
        ))
    }
});
