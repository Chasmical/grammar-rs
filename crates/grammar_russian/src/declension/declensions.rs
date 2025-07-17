use crate::{
    categories::GenderAnimacy,
    declension::{AdjectiveStemType, AnyStemType, DeclensionFlags, NounStemType, PronounStemType},
    stress::{AdjectiveStress, AnyDualStress, NounStress, PronounStress},
    util::const_traits::*,
};

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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NounDeclension {
    pub stem_type: NounStemType,
    pub flags: DeclensionFlags,
    pub stress: NounStress,
    pub override_gender: Option<GenderAnimacy>,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PronounDeclension {
    pub stem_type: PronounStemType,
    pub flags: DeclensionFlags,
    pub stress: PronounStress,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdjectiveDeclension {
    pub stem_type: AdjectiveStemType,
    pub flags: DeclensionFlags,
    pub stress: AdjectiveStress,
}

impl Declension {
    pub const fn is_noun(self) -> bool {
        matches!(self, Self::Noun(_))
    }
    pub const fn is_pronoun(self) -> bool {
        matches!(self, Self::Pronoun(_))
    }
    pub const fn is_adjective(self) -> bool {
        matches!(self, Self::Adjective(_))
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

    pub const fn kind(self) -> DeclensionKind {
        match self {
            Self::Noun(_) => DeclensionKind::Noun,
            Self::Pronoun(_) => DeclensionKind::Pronoun,
            Self::Adjective(_) => DeclensionKind::Adjective,
        }
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
