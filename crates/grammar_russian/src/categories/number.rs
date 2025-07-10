/// A Russian grammatical number.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    #[default]
    Singular = 0,
    Plural = 1,
}

// Trait providing Number value
#[const_trait]
pub trait HasNumber {
    fn number(&self) -> Number;

    fn is_singular(&self) -> bool {
        matches!(self.number(), Number::Singular)
    }
    fn is_plural(&self) -> bool {
        matches!(self.number(), Number::Plural)
    }
}

// Number provides itself
impl const HasNumber for Number {
    fn number(&self) -> Number {
        *self
    }
}

// Number abbreviation constants
impl Number {
    pub const SG: Self = Self::Singular;
    pub const PL: Self = Self::Plural;
}

impl Number {
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

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr_upper().fmt(f)
    }
}
