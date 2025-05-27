use crate::util::const_traits::*;

use super::defs::*;

//                         TABLE OF STRESS TYPE CONVERSIONS
// ┌———————┬——————┬——————┬——————┬——————┬——————┬——————┬——————╥——————┬——————┬——————┐
// │From\To│ Any  │ Noun │ AdjF │ AdjS │ Pro  │ VerbF│ VerbP║ ANY  │ ADJ  │ VERB │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Any   │ ———— │  []  │  []  │  []  │  []  │  []  │  []  ║  ██  │  []  │  []  │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Noun  │  ██  │ ———— │      │      │      │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ AdjF  │  ██  │      │ ———— │      │      │      │      ║  ██  │  ██  │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ AdjS  │  ██  │      │      │ ———— │      │      │      ║  ██  │  []  │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Pro   │  ██  │      │      │      │ ———— │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VerbF │  ██  │      │      │      │      │ ———— │      ║  ██  │      │  ██  │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VerbP │  ██  │      │      │      │      │      │ ———— ║  ██  │      │  []  │
// ╞═══════╪══════╪══════╪══════╪══════╪══════╪══════╪══════╬══════╪══════╪══════╡
// │ ANY   │      │      │      │      │      │      │      ║ ———— │  []  │  []  │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ ADJ   │      │      │      │      │      │      │      ║  ██  │ ———— │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VERB  │      │      │      │      │      │      │      ║  ██  │      │ ———— │
// └———————┴——————┴——————┴——————┴——————┴——————┴——————┴——————╨——————┴——————┴——————┘
//                                                     ██ — From   [] — TryFrom

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

impl_const_From!(<AdjectiveFullStress> for AdjectiveStress {
    fn from(value: AdjectiveFullStress) -> Self {
        Self::new(value, AnyStress::_from(value)._try_into()._unwrap())
    }
});
impl_const_TryFrom!(<AdjectiveShortStress> for AdjectiveStress {
    type Error = AdjectiveFullStressError;
    fn try_from(value: AdjectiveShortStress) -> Result<Self, Self::Error> {
        let full = AnyStress::_from(value).unprime()._try_into();
        // FIXME(const-hack): Replace with `?`.
        Ok(Self::new(const_try!(full), value))
    }
});
impl_const_TryFrom!(<AnyStress> for AdjectiveStress {
    type Error = AdjectiveFullStressError;
    fn try_from(value: AnyStress) -> Result<Self, Self::Error> {
        // FIXME(const-hack): Replace with `.map_or(Err(Self::Error {}), |x| x.try_into())`.
        match AdjectiveShortStress::_try_from(value) {
            Ok(x) => x._try_into(),
            Err(_) => Err(Self::Error {}),
        }
    }
});

impl_const_From!(<VerbPresentStress> for VerbStress {
    fn from(value: VerbPresentStress) -> Self {
        Self::new(value, VerbPastStress::A)
    }
});
impl_const_TryFrom!(<VerbPastStress> for VerbStress {
    type Error = VerbPresentStressError;
    fn try_from(value: VerbPastStress) -> Result<Self, Self::Error> {
        // FIXME(const-hack): Replace with `?`.
        Ok(Self::new(const_try!(AnyStress::_from(value)._try_into()), value))
    }
});
impl_const_TryFrom!(<AnyStress> for VerbStress {
    type Error = VerbPresentStressError;
    fn try_from(value: AnyStress) -> Result<Self, Self::Error> {
        // FIXME(const-hack): Replace with `.map_or(Err(Self::Error {}), |x| x.try_into())`.
        match VerbPastStress::_try_from(value) {
            Ok(x) => x._try_into(),
            Err(_) => Err(Self::Error {}),
        }
    }
});

impl_const_TryFrom!(<AnyDualStress> for AdjectiveStress {
    type Error = AdjectiveStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if let Some(alt) = value.alt {
            Ok(Self::new(
                // FIXME(const-hack): Replace with `?`.
                const_try!(value.main._try_into(), Self::Error::Full),
                // FIXME(const-hack): Replace with `?`.
                const_try!(alt._try_into(), Self::Error::Short),
            ))
        } else {
            // FIXME(const-hack): Replace with `?`.
            let alt = const_try!(value.main._try_into(), Self::Error::Short);
            Ok(Self::new(
                // FIXME(const-hack): Replace with `?`.
                const_try!(AnyStress::_from(alt).unprime()._try_into(), Self::Error::Full),
                alt
            ))
        }
    }
});
impl_const_TryFrom!(<AnyDualStress> for VerbStress {
    type Error = VerbStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        Ok(Self::new(
            // FIXME(const-hack): Replace with `?`.
            const_try!(value.main._try_into(), Self::Error::Present),
            // FIXME(const-hack): Replace with `.map_or(Ok(VerbPastStress::A), |x| x.try_into())?`.
            match value.alt {
                Some(x) => const_try!(x._try_into(), Self::Error::Past),
                None => VerbPastStress::A,
            }
        ))
    }
});
