use crate::{
    declension::{AdjectiveStemType, AnyStemType, DeclensionFlags, NounStemType, PronounStemType},
    stress::{AdjectiveStress, AnyDualStress, NounStress, PronounStress},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NounDeclension {
    pub stem_type: NounStemType,
    pub flags: DeclensionFlags,
    pub stress: NounStress,
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
            Self::Noun(x) => x.stem_type.into(),
            Self::Pronoun(x) => x.stem_type.into(),
            Self::Adjective(x) => x.stem_type.into(),
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
            Self::Noun(x) => x.stress.into(),
            Self::Pronoun(x) => x.stress.into(),
            Self::Adjective(x) => x.stress.into(),
        }
    }
}

impl const From<NounDeclension> for Declension {
    fn from(value: NounDeclension) -> Self {
        Self::Noun(value)
    }
}
impl const From<PronounDeclension> for Declension {
    fn from(value: PronounDeclension) -> Self {
        Self::Pronoun(value)
    }
}
impl const From<AdjectiveDeclension> for Declension {
    fn from(value: AdjectiveDeclension) -> Self {
        Self::Adjective(value)
    }
}

impl const TryFrom<Declension> for NounDeclension {
    type Error = ();
    fn try_from(value: Declension) -> Result<Self, Self::Error> {
        value.as_noun().ok_or(())
    }
}
impl const TryFrom<Declension> for PronounDeclension {
    type Error = ();
    fn try_from(value: Declension) -> Result<Self, Self::Error> {
        value.as_pronoun().ok_or(())
    }
}
impl const TryFrom<Declension> for AdjectiveDeclension {
    type Error = ();
    fn try_from(value: Declension) -> Result<Self, Self::Error> {
        value.as_adjective().ok_or(())
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaybeZeroDeclension(Option<Declension>);

impl MaybeZeroDeclension {
    pub const ZERO: Self = Self(None);

    pub const fn new(value: Option<Declension>) -> Self {
        Self(value)
    }
    pub const fn as_option(self) -> Option<Declension> {
        self.0
    }

    pub const fn is_zero(self) -> bool {
        self.0.is_none()
    }
    pub const fn is_noun(self) -> bool {
        self.0.is_some_and(Declension::is_noun)
    }
    pub const fn is_pronoun(self) -> bool {
        self.0.is_some_and(Declension::is_pronoun)
    }
    pub const fn is_adjective(self) -> bool {
        self.0.is_some_and(Declension::is_adjective)
    }
    pub const fn as_noun(self) -> Option<NounDeclension> {
        self.0.and_then(Declension::as_noun)
    }
    pub const fn as_pronoun(self) -> Option<PronounDeclension> {
        self.0.and_then(Declension::as_pronoun)
    }
    pub const fn as_adjective(self) -> Option<AdjectiveDeclension> {
        self.0.and_then(Declension::as_adjective)
    }

    pub const fn kind(self) -> Option<DeclensionKind> {
        self.0.map(Declension::kind)
    }
    pub const fn stem_type(self) -> Option<AnyStemType> {
        self.0.map(Declension::stem_type)
    }
    pub const fn flags(self) -> DeclensionFlags {
        self.0.map_or(DeclensionFlags::empty(), Declension::flags)
    }
    pub const fn stress(self) -> Option<AnyDualStress> {
        self.0.map(Declension::stress)
    }
}

impl const From<NounDeclension> for MaybeZeroDeclension {
    fn from(value: NounDeclension) -> Self {
        Self(Some(Declension::Noun(value)))
    }
}
impl const From<PronounDeclension> for MaybeZeroDeclension {
    fn from(value: PronounDeclension) -> Self {
        Self(Some(Declension::Pronoun(value)))
    }
}
impl const From<AdjectiveDeclension> for MaybeZeroDeclension {
    fn from(value: AdjectiveDeclension) -> Self {
        Self(Some(Declension::Adjective(value)))
    }
}
impl const From<Declension> for MaybeZeroDeclension {
    fn from(value: Declension) -> Self {
        Self(Some(value))
    }
}

impl const TryFrom<MaybeZeroDeclension> for NounDeclension {
    type Error = ();
    fn try_from(value: MaybeZeroDeclension) -> Result<Self, Self::Error> {
        value.as_noun().ok_or(())
    }
}
impl const TryFrom<MaybeZeroDeclension> for PronounDeclension {
    type Error = ();
    fn try_from(value: MaybeZeroDeclension) -> Result<Self, Self::Error> {
        value.as_pronoun().ok_or(())
    }
}
impl const TryFrom<MaybeZeroDeclension> for AdjectiveDeclension {
    type Error = ();
    fn try_from(value: MaybeZeroDeclension) -> Result<Self, Self::Error> {
        value.as_adjective().ok_or(())
    }
}
impl const TryFrom<MaybeZeroDeclension> for Declension {
    type Error = ();
    fn try_from(value: MaybeZeroDeclension) -> Result<Self, Self::Error> {
        value.0.ok_or(())
    }
}

impl const From<Option<NounDeclension>> for MaybeZeroDeclension {
    fn from(value: Option<NounDeclension>) -> Self {
        Self(value.map(Declension::Noun))
    }
}
impl const From<Option<PronounDeclension>> for MaybeZeroDeclension {
    fn from(value: Option<PronounDeclension>) -> Self {
        Self(value.map(Declension::Pronoun))
    }
}
impl const From<Option<AdjectiveDeclension>> for MaybeZeroDeclension {
    fn from(value: Option<AdjectiveDeclension>) -> Self {
        Self(value.map(Declension::Adjective))
    }
}

impl const From<Option<Declension>> for MaybeZeroDeclension {
    fn from(value: Option<Declension>) -> Self {
        Self(value)
    }
}
impl const From<MaybeZeroDeclension> for Option<Declension> {
    fn from(value: MaybeZeroDeclension) -> Self {
        value.0
    }
}
