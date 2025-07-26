use super::{
    Animacy, Case, CaseEx, Gender, GenderAnimacy, GenderEx, GenderExAnimacy, Number,
    traits::{HasAnimacy, HasGender, HasGenderEx, HasNumber},
};

// Case[Ex] abbreviations
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

    pub const fn abbr_upper(self) -> &'static str {
        match self {
            Self::Nominative => "NOM",
            Self::Genitive => "GEN",
            Self::Dative => "DAT",
            Self::Accusative => "ACC",
            Self::Instrumental => "INS",
            Self::Prepositional => "PRP",
            Self::Partitive => "PRT",
            Self::Translative => "TRANSL",
            Self::Locative => "LOC",
        }
    }
    pub const fn abbr_lower(self) -> &'static str {
        match self {
            Self::Nominative => "nom",
            Self::Genitive => "gen",
            Self::Dative => "dat",
            Self::Accusative => "acc",
            Self::Instrumental => "ins",
            Self::Prepositional => "prp",
            Self::Partitive => "prt",
            Self::Translative => "transl",
            Self::Locative => "loc",
        }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        // Note: small caps 'ꜱ' (U+A731) may not render correctly in some fonts,
        //       so a regular 's' can be used instead for better consistency.
        match self {
            Self::Nominative => "ɴᴏᴍ",
            Self::Genitive => "ɢᴇɴ",
            Self::Dative => "ᴅᴀᴛ",
            Self::Accusative => "ᴀᴄᴄ",
            Self::Instrumental => "ɪɴꜱ",
            Self::Prepositional => "ᴘʀᴘ",
            Self::Partitive => "ᴘʀᴛ",
            Self::Translative => "ᴛʀᴀɴꜱʟ",
            Self::Locative => "ʟᴏᴄ",
        }
    }
}
impl Case {
    pub const NOM: Self = Self::Nominative;
    pub const GEN: Self = Self::Genitive;
    pub const DAT: Self = Self::Dative;
    pub const ACC: Self = Self::Accusative;
    pub const INS: Self = Self::Instrumental;
    pub const PRP: Self = Self::Prepositional;

    pub const fn abbr_upper(self) -> &'static str {
        CaseEx::from(self).abbr_upper()
    }
    pub const fn abbr_lower(self) -> &'static str {
        CaseEx::from(self).abbr_lower()
    }
    pub const fn abbr_smcp(self) -> &'static str {
        CaseEx::from(self).abbr_smcp()
    }
}

// Gender[Ex] abbreviations
impl GenderEx {
    pub const MASC: Self = Self::Masculine;
    pub const NEUT: Self = Self::Neuter;
    pub const FEM: Self = Self::Feminine;
    pub const COMMON: Self = Self::Common;

    pub const fn abbr_upper(self) -> &'static str {
        match self {
            Self::Masculine => "MASC",
            Self::Neuter => "NEUT",
            Self::Feminine => "FEM",
            Self::Common => "MASC/FEM",
        }
    }
    pub const fn abbr_lower(self) -> &'static str {
        match self {
            Self::Masculine => "masc",
            Self::Neuter => "neut",
            Self::Feminine => "fem",
            Self::Common => "masc/fem",
        }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        // Note: small caps 'ꜰ' (U+A730) may not render correctly in some fonts.
        match self {
            Self::Masculine => "ᴍᴀꜱᴄ",
            Self::Neuter => "ɴᴇᴜᴛ",
            Self::Feminine => "ꜰᴇᴍ",
            Self::Common => "ᴍᴀꜱᴄ/ꜰᴇᴍ",
        }
    }
}
impl Gender {
    pub const MASC: Self = Self::Masculine;
    pub const NEUT: Self = Self::Neuter;
    pub const FEM: Self = Self::Feminine;

    pub const fn abbr_upper(self) -> &'static str {
        GenderEx::from(self).abbr_upper()
    }
    pub const fn abbr_lower(self) -> &'static str {
        GenderEx::from(self).abbr_lower()
    }
    pub const fn abbr_smcp(self) -> &'static str {
        GenderEx::from(self).abbr_smcp()
    }
}

// Animacy abbreviations
impl Animacy {
    pub const INAN: Self = Self::Inanimate;
    pub const AN: Self = Self::Animate;

    pub const fn abbr_upper(self) -> &'static str {
        if self.is_inanimate() { "INAN" } else { "AN" }
    }
    pub const fn abbr_lower(self) -> &'static str {
        if self.is_inanimate() { "inan" } else { "an" }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        if self.is_inanimate() { "ɪɴᴀɴ" } else { "ᴀɴ" }
    }
}

// Number abbreviations
impl Number {
    pub const SG: Self = Self::Singular;
    pub const PL: Self = Self::Plural;

    pub const fn abbr_upper(self) -> &'static str {
        if self.is_singular() { "SG" } else { "PL" }
    }
    pub const fn abbr_lower(self) -> &'static str {
        if self.is_singular() { "sg" } else { "pl" }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        if self.is_singular() { "ꜱɢ" } else { "ᴘʟ" }
    }
}

// Gender[Ex]Animacy abbreviation constants
impl GenderExAnimacy {
    pub const MASC_INAN: Self = Self::MasculineInanimate;
    pub const MASC_AN: Self = Self::MasculineAnimate;
    pub const NEUT_INAN: Self = Self::NeuterInanimate;
    pub const NEUT_AN: Self = Self::NeuterAnimate;
    pub const FEM_INAN: Self = Self::FeminineInanimate;
    pub const FEM_AN: Self = Self::FeminineAnimate;

    pub const fn abbr_zaliznyak(self) -> &'static str {
        match self {
            Self::MasculineInanimate => "м",
            Self::MasculineAnimate => "мо",
            Self::NeuterInanimate => "с",
            Self::NeuterAnimate => "со",
            Self::FeminineInanimate => "ж",
            Self::FeminineAnimate => "жо",
            Self::CommonAnimate => "мо-жо",
        }
    }
}
impl GenderAnimacy {
    pub const MASC_INAN: Self = Self::MasculineInanimate;
    pub const MASC_AN: Self = Self::MasculineAnimate;
    pub const NEUT_INAN: Self = Self::NeuterInanimate;
    pub const NEUT_AN: Self = Self::NeuterAnimate;
    pub const FEM_INAN: Self = Self::FeminineInanimate;
    pub const FEM_AN: Self = Self::FeminineAnimate;

    pub const fn abbr_zaliznyak(self) -> &'static str {
        GenderExAnimacy::from(self).abbr_zaliznyak()
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
impl std::fmt::Display for GenderEx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr_upper().fmt(f)
    }
}
impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr_upper().fmt(f)
    }
}
impl std::fmt::Display for Animacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr_upper().fmt(f)
    }
}
impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr_upper().fmt(f)
    }
}

impl std::fmt::Display for GenderExAnimacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.gender_ex(), self.animacy())
    }
}
impl std::fmt::Display for GenderAnimacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.gender(), self.animacy())
    }
}
