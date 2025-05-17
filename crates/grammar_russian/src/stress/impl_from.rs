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

impl From<AdjectiveStress> for AnyDualStress {
    fn from(value: AdjectiveStress) -> Self {
        Self::new(value.full.into(), Some(value.short.into()))
    }
}
impl From<VerbStress> for AnyDualStress {
    fn from(value: VerbStress) -> Self {
        Self::new(value.present.into(), Some(value.past.into()))
    }
}

impl From<AdjectiveFullStress> for AdjectiveStress {
    fn from(value: AdjectiveFullStress) -> Self {
        Self::new(value, AnyStress::from(value).try_into().unwrap())
    }
}
impl TryFrom<AdjectiveShortStress> for AdjectiveStress {
    type Error = AdjectiveFullStressError;
    fn try_from(value: AdjectiveShortStress) -> Result<Self, Self::Error> {
        let full = AnyStress::from(value).unprime().try_into();
        Ok(Self::new(full?, value))
    }
}
impl TryFrom<AnyStress> for AdjectiveStress {
    type Error = AdjectiveFullStressError;
    fn try_from(value: AnyStress) -> Result<Self, Self::Error> {
        AdjectiveShortStress::try_from(value).map_or(Err(Self::Error {}), |x| x.try_into())
    }
}

impl From<VerbPresentStress> for VerbStress {
    fn from(value: VerbPresentStress) -> Self {
        Self::new(value, VerbPastStress::A)
    }
}
impl TryFrom<VerbPastStress> for VerbStress {
    type Error = VerbPresentStressError;
    fn try_from(value: VerbPastStress) -> Result<Self, Self::Error> {
        Ok(Self::new(AnyStress::from(value).try_into()?, value))
    }
}
impl TryFrom<AnyStress> for VerbStress {
    type Error = VerbPresentStressError;
    fn try_from(value: AnyStress) -> Result<Self, Self::Error> {
        VerbPastStress::try_from(value).map_or(Err(Self::Error {}), |x| x.try_into())
    }
}

impl From<AdjectiveFullStressError> for AdjectiveStressError {
    fn from(value: AdjectiveFullStressError) -> Self {
        Self::Full(value)
    }
}
impl From<AdjectiveShortStressError> for AdjectiveStressError {
    fn from(value: AdjectiveShortStressError) -> Self {
        Self::Short(value)
    }
}
impl TryFrom<AnyDualStress> for AdjectiveStress {
    type Error = AdjectiveStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if let Some(alt) = value.alt {
            Ok(Self::new(value.main.try_into()?, alt.try_into()?))
        } else {
            let alt = value.main.try_into()?;
            Ok(Self::new(AnyStress::from(alt).try_into()?, alt))
        }
    }
}

impl From<VerbPresentStressError> for VerbStressError {
    fn from(value: VerbPresentStressError) -> Self {
        Self::Present(value)
    }
}
impl From<VerbPastStressError> for VerbStressError {
    fn from(value: VerbPastStressError) -> Self {
        Self::Past(value)
    }
}
impl TryFrom<AnyDualStress> for VerbStress {
    type Error = VerbStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.main.try_into()?,
            value.alt.map_or(Ok(VerbPastStress::A), |x| x.try_into())?,
        ))
    }
}
