use crate::{categories::*, declension::*, stress::*, util::const_traits::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Declension {
    Noun(NounDeclension),
    Pronoun(PronounDeclension),
    Adjective(AdjectiveDeclension),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclensionKind {
    Noun,
    Pronoun,
    Adjective,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NounDeclension {
    pub stem_type: NounStemType,
    pub flags: DeclensionFlags,
    pub stress: NounStress,
    pub override_gender: Option<GenderAnimacy>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PronounDeclension {
    pub stem_type: PronounStemType,
    pub flags: DeclensionFlags,
    pub stress: PronounStress,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdjectiveDeclension {
    pub stem_type: AdjectiveStemType,
    pub flags: DeclensionFlags,
    pub stress: AdjectiveStress,
    pub is_reflexive: bool,
}

impl Declension {
    pub const fn kind(self) -> DeclensionKind {
        match self {
            Self::Noun(_) => DeclensionKind::Noun,
            Self::Pronoun(_) => DeclensionKind::Pronoun,
            Self::Adjective(_) => DeclensionKind::Adjective,
        }
    }
    pub const fn as_noun(self) -> Option<NounDeclension> {
        if let Self::Noun(x) = self { Some(x) } else { None }
    }
    pub const fn as_pronoun(self) -> Option<PronounDeclension> {
        if let Self::Pronoun(x) = self { Some(x) } else { None }
    }
    pub const fn as_adjective(self) -> Option<AdjectiveDeclension> {
        if let Self::Adjective(x) = self { Some(x) } else { None }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DeclInfo {
    pub case: Case,
    pub number: Number,
    pub gender: Gender,
    pub animacy: Animacy,
}

impl const HasCase for DeclInfo {
    fn case(&self) -> Case {
        self.case
    }
}
impl const HasNumber for DeclInfo {
    fn number(&self) -> Number {
        self.number
    }
}
impl const HasGender for DeclInfo {
    fn gender(&self) -> Gender {
        self.gender
    }
}
impl const HasAnimacy for DeclInfo {
    fn animacy(&self) -> Animacy {
        self.animacy
    }
}

impl Declension {
    pub const fn is_noun(self) -> bool {
        matches!(self, Self::Noun(_))
    }
    pub const fn is_adjective(self) -> bool {
        matches!(self, Self::Adjective(_))
    }
    pub const fn is_pronoun(self) -> bool {
        matches!(self, Self::Pronoun(_))
    }

    pub const fn stem_type(self) -> AnyStemType {
        match self {
            Self::Noun(x) => x.stem_type._into(),
            Self::Pronoun(x) => x.stem_type._into(),
            Self::Adjective(x) => x.stem_type._into(),
        }
    }
    pub const fn flags(self) -> DeclensionFlags {
        match self {
            Self::Noun(x) => x.flags,
            Self::Pronoun(x) => x.flags,
            Self::Adjective(x) => x.flags,
        }
    }
    pub const fn stress(self) -> AnyDualStress {
        match self {
            Self::Noun(x) => x.stress._into(),
            Self::Pronoun(x) => x.stress._into(),
            Self::Adjective(x) => x.stress._into(),
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DeclensionFlags: u8 {
        const STAR = 1 << 0;
        const CIRCLE = 1 << 1;
        const CIRCLED_ONE = 1 << 2;
        const CIRCLED_TWO = 1 << 3;
        const CIRCLED_THREE = 1 << 4;
        const ALTERNATING_YO = 1 << 5;
    }
}
impl DeclensionFlags {
    pub const fn has_star(self) -> bool {
        self.intersects(Self::STAR)
    }
    pub const fn has_circle(self) -> bool {
        self.intersects(Self::CIRCLE)
    }
    pub const fn has_circled_one(self) -> bool {
        self.intersects(Self::CIRCLED_ONE)
    }
    pub const fn has_circled_two(self) -> bool {
        self.intersects(Self::CIRCLED_TWO)
    }
    pub const fn has_circled_three(self) -> bool {
        self.intersects(Self::CIRCLED_THREE)
    }
    pub const fn has_alternating_yo(self) -> bool {
        self.intersects(Self::ALTERNATING_YO)
    }

    const ALL_LEADING_FLAGS: Self = Self::STAR.union(Self::CIRCLE);
    const ALL_TRAILING_FLAGS: Self = Self::ALL_CIRCLED_DIGITS.union(Self::ALTERNATING_YO);
    const ALL_CIRCLED_DIGITS: Self =
        Self::CIRCLED_ONE.union(Self::CIRCLED_TWO).union(Self::CIRCLED_THREE);

    pub const fn has_any_leading_flags(self) -> bool {
        self.intersects(Self::ALL_LEADING_FLAGS)
    }
    pub const fn has_any_trailing_flags(self) -> bool {
        self.intersects(Self::ALL_TRAILING_FLAGS)
    }
    pub const fn has_any_circled_digits(self) -> bool {
        self.intersects(Self::ALL_CIRCLED_DIGITS)
    }
}

impl_const_From!(<NounDeclension> for Declension {
    fn from(value: NounDeclension) -> Self {
        Self::Noun(value)
    }
});
impl_const_From!(<PronounDeclension> for Declension {
    fn from(value: PronounDeclension) -> Self {
        Self::Pronoun(value)
    }
});
impl_const_From!(<AdjectiveDeclension> for Declension {
    fn from(value: AdjectiveDeclension) -> Self {
        Self::Adjective(value)
    }
});

impl_const_TryFrom!(<Declension> for NounDeclension {
    type Error = ();
    fn try_from(value: Declension) -> Result<Self, Self::Error> {
        if let Declension::Noun(x) = value { Ok(x) } else { Err(()) }
    }
});
impl_const_TryFrom!(<Declension> for PronounDeclension {
    type Error = ();
    fn try_from(value: Declension) -> Result<Self, Self::Error> {
        if let Declension::Pronoun(x) = value { Ok(x) } else { Err(()) }
    }
});
impl_const_TryFrom!(<Declension> for AdjectiveDeclension {
    type Error = ();
    fn try_from(value: Declension) -> Result<Self, Self::Error> {
        if let Declension::Adjective(x) = value { Ok(x) } else { Err(()) }
    }
});
