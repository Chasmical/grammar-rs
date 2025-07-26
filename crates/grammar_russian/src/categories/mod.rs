mod abbrs;
mod convert;
mod ops;
mod traits;

pub use convert::*;
pub use traits::*;

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

/// A main or secondary Russian grammatical gender: [`Masculine`][GenderEx::Masculine],
/// [`Neuter`][GenderEx::Neuter], [`Feminine`][GenderEx::Feminine] or [`Common`][GenderEx::Common].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderEx {
    #[default]
    Masculine = 0,
    Neuter = 1,
    Feminine = 2,
    Common = 3,
}
/// One of the main 3 Russian grammatical genders: [`Masculine`][Gender::Masculine],
/// [`Neuter`][Gender::Neuter], [`Feminine`][Gender::Feminine].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    #[default]
    Masculine = 0,
    Neuter = 1,
    Feminine = 2,
}

/// A Russian grammatical animacy: [`Inanimate`][Animacy::Inanimate] or [`Animate`][Animacy::Animate].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Animacy {
    #[default]
    Inanimate = 0,
    Animate = 1,
}
/// A Russian grammatical number: [`Singular`][Number::Singular] or [`Plural`][Number::Plural].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    #[default]
    Singular = 0,
    Plural = 1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderExAnimacy {
    #[default]
    MasculineInanimate = 0,
    MasculineAnimate = 1,
    NeuterInanimate = 2,
    NeuterAnimate = 3,
    FeminineInanimate = 4,
    FeminineAnimate = 5,
    // common inanimate isn't a thing, but 6 is reserved for it,
    // just so that CommonAnimate has the animacy bit set to 1.
    CommonAnimate = 7,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GenderAnimacy {
    #[default]
    MasculineInanimate = 0,
    MasculineAnimate = 1,
    NeuterInanimate = 2,
    NeuterAnimate = 3,
    FeminineInanimate = 4,
    FeminineAnimate = 5,
}

impl CaseEx {
    pub const VALUES: [CaseEx; 9] = [
        Self::NOM,
        Self::GEN,
        Self::DAT,
        Self::ACC,
        Self::INS,
        Self::PRP,
        Self::PRT,
        Self::TRANSL,
        Self::LOC,
    ];
}
impl Case {
    pub const VALUES: [Case; 6] =
        [Self::NOM, Self::GEN, Self::DAT, Self::ACC, Self::INS, Self::PRP];
}

impl GenderEx {
    pub const VALUES: [GenderEx; 4] = [Self::MASC, Self::NEUT, Self::FEM, Self::COMMON];
}
impl Gender {
    pub const VALUES: [Gender; 3] = [Self::Masculine, Self::Neuter, Self::Feminine];
}

impl Animacy {
    pub const VALUES: [Animacy; 2] = [Self::Inanimate, Self::Animate];
}
impl Number {
    pub const VALUES: [Number; 2] = [Self::Singular, Self::Plural];
}

impl GenderExAnimacy {
    pub const VALUES: [GenderExAnimacy; 7] = [
        Self::MasculineInanimate,
        Self::MasculineAnimate,
        Self::NeuterInanimate,
        Self::NeuterAnimate,
        Self::FeminineInanimate,
        Self::FeminineAnimate,
        Self::CommonAnimate,
    ];
}
impl GenderAnimacy {
    pub const VALUES: [GenderAnimacy; 6] = [
        Self::MasculineInanimate,
        Self::MasculineAnimate,
        Self::NeuterInanimate,
        Self::NeuterAnimate,
        Self::FeminineInanimate,
        Self::FeminineAnimate,
    ];
}
