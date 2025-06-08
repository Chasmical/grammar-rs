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
