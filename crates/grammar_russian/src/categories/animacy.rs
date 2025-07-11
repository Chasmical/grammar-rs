use super::Case;

/// A Russian grammatical animacy: inanimate or animate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Animacy {
    #[default]
    Inanimate = 0,
    Animate = 1,
}

// Trait providing Animacy value
#[const_trait]
pub trait HasAnimacy {
    fn animacy(&self) -> Animacy;

    fn is_animate(&self) -> bool {
        matches!(self.animacy(), Animacy::Animate)
    }
    fn is_inanimate(&self) -> bool {
        matches!(self.animacy(), Animacy::Inanimate)
    }
}

// Animacy provides itself
impl const HasAnimacy for Animacy {
    fn animacy(&self) -> Animacy {
        *self
    }
}

impl Animacy {
    pub const INAN: Self = Self::Inanimate;
    pub const AN: Self = Self::Animate;

    pub const fn acc_case(self) -> Case {
        match self {
            Animacy::Inanimate => Case::Nominative,
            Animacy::Animate => Case::Genitive,
        }
    }
}

impl Animacy {
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

impl std::fmt::Display for Animacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr_upper().fmt(f)
    }
}
