use super::{HasAnimacy, Number};
use crate::util::{const_traits::*, enum_conversion};
use thiserror::Error;

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

enum_conversion!(Case => CaseEx [<= CaseError] {
    Nominative, Genitive, Dative, Accusative, Instrumental, Prepositional,
});
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
            _ => (unsafe { std::mem::transmute::<CaseEx, Case>(self) }, number),
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

// Case abbreviation constants
impl CaseEx {
    pub const NOM: Self = Self::Nominative;
    pub const GEN: Self = Self::Genitive;
    pub const DAT: Self = Self::Dative;
    pub const ACC: Self = Self::Accusative;
    pub const INS: Self = Self::Instrumental;
    pub const PRP: Self = Self::Prepositional;
    pub const PRT: Self = Self::Partitive;
    pub const TRANSL: Self = Self::Translative;
    pub const LOC: Self = Self::Locative;
}
impl Case {
    pub const NOM: Self = Self::Nominative;
    pub const GEN: Self = Self::Genitive;
    pub const DAT: Self = Self::Dative;
    pub const ACC: Self = Self::Accusative;
    pub const INS: Self = Self::Instrumental;
    pub const PRP: Self = Self::Prepositional;
}

impl CaseEx {
    pub const fn abbr_upper(self) -> &'static str {
        match self {
            Self::NOM => "NOM",
            Self::GEN => "GEN",
            Self::DAT => "DAT",
            Self::ACC => "ACC",
            Self::INS => "INS",
            Self::PRP => "PRP",
            Self::PRT => "PRT",
            Self::TRANSL => "TRANSL",
            Self::LOC => "LOC",
        }
    }
    pub const fn abbr_lower(self) -> &'static str {
        match self {
            Self::NOM => "nom",
            Self::GEN => "gen",
            Self::DAT => "dat",
            Self::ACC => "acc",
            Self::INS => "ins",
            Self::PRP => "prp",
            Self::PRT => "prt",
            Self::TRANSL => "transl",
            Self::LOC => "loc",
        }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        // Note: small caps 'ꜱ' (U+A731) may not render correctly in some fonts,
        //       so a regular 's' can be used instead for better consistency.
        match self {
            Self::NOM => "ɴᴏᴍ",
            Self::GEN => "ɢᴇɴ",
            Self::DAT => "ᴅᴀᴛ",
            Self::ACC => "ᴀᴄᴄ",
            Self::INS => "ɪɴꜱ",
            Self::PRP => "ᴘʀᴘ",
            Self::PRT => "ᴘʀᴛ",
            Self::TRANSL => "ᴛʀᴀɴꜱʟ",
            Self::LOC => "ʟᴏᴄ",
        }
    }
}
impl Case {
    pub const fn abbr_upper(self) -> &'static str {
        CaseEx::_from(self).abbr_upper()
    }
    pub const fn abbr_lower(self) -> &'static str {
        CaseEx::_from(self).abbr_lower()
    }
    pub const fn abbr_smcp(self) -> &'static str {
        CaseEx::_from(self).abbr_smcp()
    }
}

impl std::fmt::Display for CaseEx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr_upper().fmt(f)
    }
}
impl std::fmt::Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr_upper().fmt(f)
    }
}
