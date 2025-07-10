use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnyStress {
    /// Stress schema `a`. The stress is always on the stem. Used by all inflectable words.
    A = 1,
    /// Stress schema `b`. The stress is always on the ending. Used by all inflectable words.
    B,
    /// Stress schema `c`.
    /// - Nouns: singular - stress on stem, plural - stress on ending.
    /// - Adjectives (short form only): feminine - stress on ending, all other - stress on stem.
    /// - Verbs (present tense): first person, and imperative - stress on ending, all other - stress on stem.
    /// - Verbs (past tense): feminine - stress on ending, all other - stress on stem.
    C,
    /// Stress schema `d`.
    /// - Nouns: singular - stress on ending, plural - stress on stem.
    D,
    /// Stress schema `e`.
    /// - Nouns: singular, and plural nominative - stress on stem, plural of other cases - stress on ending.
    E,
    /// Stress schema `f`.
    /// - Nouns and pronouns: plural nominative - stress on stem, all other - stress on ending.
    F,
    /// Stress schema `a′` (`a` with single prime).
    /// - Adjectives (short form only): feminine - both??? (resolved as on stem), all other - stress on stem.
    Ap,
    /// Stress schema `b′` (`b` with single prime).
    /// - Nouns: singular instrumental - stress on stem, all other - stress on ending.
    /// - Adjectives (short form only): plural - both??? (resolved as on ending), all other - stress on ending.
    Bp,
    /// Stress schema `c′` (`c` with single prime).
    /// - Adjectives (short form only): feminine - stress on ending, neuter - stress on stem, plural - TODO: both???.
    /// - Verbs (present tense): first person, imperative, and plural - stress on ending, all other - stress on stem.
    /// - Verbs (past tense): feminine - stress on ending, neuter - TODO: both???, all other - stress on stem.
    Cp,
    /// Stress schema `d′` (`d` with single prime).
    /// - Nouns: singular accusative, and plural - stress on stem, singular of other cases - stress on ending.
    Dp,
    /// Stress schema `e′` (`e` with single prime).
    /// TODO: Unused???
    Ep,
    /// Stress schema `f′` (`f` with single prime).
    /// - Nouns: singular accusative, and plural nominative - stress on stem, all other - stress on ending.
    Fp,
    /// Stress schema `c″` (`c` with double prime).
    /// - Adjectives (short form only): feminine - stress on ending, all other - both??? (resolved as on ending).
    /// - Verbs (past tense reflexive only): masculine - stress on stem, feminine - stress on ending, neuter and plural - TODO: both???.
    Cpp,
    /// Stress schema `f″` (`f` with double prime).
    /// - Nouns: singular instrumental, and plural nominative - stress on stem, all other - stress on ending.
    Fpp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NounStress {
    /// Stress schema `a`. Stress is always on the stem.
    A,
    /// Stress schema `b`. Stress is always on the ending.
    B,
    /// Stress schema `c`. Singular - stress on stem, plural - stress on ending.
    C,
    /// Stress schema `d`. Singular - stress on ending, plural - stress on stem.
    D,
    /// Stress schema `e`. Singular, and plural nominative - stress on stem, plural of other cases - stress on ending.
    E,
    /// Stress schema `f`. Plural nominative - stress on stem, all other - stress on ending.
    F,
    /// Stress schema `b′` (`b` with single prime). Singular instrumental - stress on stem, all other - stress on ending.
    Bp,
    /// Stress schema `d′` (`d` with single prime). Singular accusative, and plural - stress on stem, singular of other cases - stress on ending.
    Dp,
    /// Stress schema `f′` (`f` with single prime). Singular accusative, and plural nominative - stress on stem, all other - stress on ending.
    Fp,
    /// Stress schema `f″` (`f` with double prime). Singular instrumental, and plural nominative - stress on stem, all other - stress on ending.
    Fpp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PronounStress {
    /// Stress schema `a`. Stress is always on the stem.
    A,
    /// Stress schema `b`. Stress is always on the ending.
    B,
    /// Stress schema `f`. Plural nominative - stress on stem, all other - stress on ending.
    F,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjectiveFullStress {
    /// Stress schema `a`. Stress is always on the stem.
    A,
    /// Stress schema `b`. Stress is always on the ending.
    B,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjectiveShortStress {
    /// Stress schema `a`. Stress is always on the stem.
    A,
    /// Stress schema `b`. Stress is always on the ending.
    B,
    /// Stress schema `c`. Feminine - stress on ending, all other - stress on stem.
    C,
    /// Stress schema `a′` (`a` with single prime). Feminine - both??? (resolved as on stem), all other - stress on stem.
    Ap,
    /// Stress schema `b′` (`b` with single prime). Plural - both??? (resolved as on ending), all other - stress on ending.
    Bp,
    /// Stress schema `c′` (`c` with single prime). Feminine - stress on ending, neuter - stress on stem, plural - TODO: both???.
    Cp,
    /// Stress schema `c″` (`c` with double prime). Feminine - stress on ending, all other - both??? (resolved as on ending).
    Cpp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbPresentStress {
    /// Stress schema `a`. Stress is always on the stem.
    A,
    /// Stress schema `b`. Stress is always on the ending.
    B,
    /// Stress schema `c`. First person, and imperative - stress on ending, all other - stress on stem.
    C,
    /// Stress schema `c′` (`c` with single prime). First person, imperative, and plural - stress on ending, all other - stress on stem.
    Cp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbPastStress {
    /// Stress schema `a`. Stress is always on the stem.
    A,
    /// Stress schema `b`. Stress is always on the ending.
    B,
    /// Stress schema `c`. Feminine - stress on ending, all other - stress on stem.
    C,
    /// Stress schema `c′` (`c` with single prime). Feminine - stress on ending, neuter - TODO: both???, all other - stress on stem.
    Cp,
    /// Stress schema `c″` (`c` with double prime). Reflexive only. Masculine - stress on stem, feminine - stress on ending, neuter and plural - TODO: both???.
    Cpp,
}

#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("words can only have stresses a-f, a′-f′, c″ and f″")]
pub struct AnyStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("nouns can only have stresses a, b, c, d, e, f, b′, d′, f′ and f″")]
pub struct NounStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("pronouns can only have stresses a, b and f")]
pub struct PronounStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("adjectives (full form) can only have stresses a and b")]
pub struct AdjectiveFullStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("adjectives (short form) can only have stresses a, b, c, a′, b′, c′ and c″")]
pub struct AdjectiveShortStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("verbs (present tense) can only have stresses a, b, c and c′")]
pub struct VerbPresentStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("verbs (past tense) can only have stresses a, b, c, c′ and c″")]
pub struct VerbPastStressError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnyDualStress {
    pub main: AnyStress,
    pub alt: Option<AnyStress>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdjectiveStress {
    pub full: AdjectiveFullStress,
    pub short: AdjectiveShortStress,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerbStress {
    pub present: VerbPresentStress,
    pub past: VerbPastStress,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum AdjectiveStressError {
    #[error("{0}")]
    Full(#[from] AdjectiveFullStressError),
    #[error("{0}")]
    Short(#[from] AdjectiveShortStressError),
}
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum VerbStressError {
    #[error("{0}")]
    Present(#[from] VerbPresentStressError),
    #[error("{0}")]
    Past(#[from] VerbPastStressError),
}

impl AnyDualStress {
    pub const fn new(main: AnyStress, alt: Option<AnyStress>) -> Self {
        Self { main, alt }
    }
}
impl AdjectiveStress {
    pub const fn new(full: AdjectiveFullStress, short: AdjectiveShortStress) -> Self {
        Self { full, short }
    }
}
impl VerbStress {
    pub const fn new(present: VerbPresentStress, past: VerbPastStress) -> Self {
        Self { present, past }
    }
}

#[allow(non_upper_case_globals)]
impl AdjectiveStress {
    pub const A: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::A);
    pub const A_A: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::A);
    pub const A_B: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::B);
    pub const A_C: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::C);
    pub const A_Ap: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Ap);
    pub const A_Bp: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Bp);
    pub const A_Cp: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Cp);
    pub const A_Cpp: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Cpp);

    pub const B: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::B);
    pub const B_A: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::A);
    pub const B_B: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::B);
    pub const B_C: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::C);
    pub const B_Ap: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Ap);
    pub const B_Bp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Bp);
    pub const B_Cp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Cp);
    pub const B_Cpp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Cpp);

    pub const Ap: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Ap);
    pub const Bp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Bp);
}
#[allow(non_upper_case_globals)]
impl VerbStress {
    pub const A: Self = Self::new(VerbPresentStress::A, VerbPastStress::A);
    pub const A_A: Self = Self::new(VerbPresentStress::A, VerbPastStress::A);
    pub const A_B: Self = Self::new(VerbPresentStress::A, VerbPastStress::B);
    pub const A_C: Self = Self::new(VerbPresentStress::A, VerbPastStress::C);
    pub const A_Cp: Self = Self::new(VerbPresentStress::A, VerbPastStress::Cp);
    pub const A_Cpp: Self = Self::new(VerbPresentStress::A, VerbPastStress::Cpp);

    pub const B: Self = Self::new(VerbPresentStress::B, VerbPastStress::A);
    pub const B_A: Self = Self::new(VerbPresentStress::B, VerbPastStress::A);
    pub const B_B: Self = Self::new(VerbPresentStress::B, VerbPastStress::B);
    pub const B_C: Self = Self::new(VerbPresentStress::B, VerbPastStress::C);
    pub const B_Cp: Self = Self::new(VerbPresentStress::B, VerbPastStress::Cp);
    pub const B_Cpp: Self = Self::new(VerbPresentStress::B, VerbPastStress::Cpp);

    pub const C: Self = Self::new(VerbPresentStress::C, VerbPastStress::A);
    pub const C_A: Self = Self::new(VerbPresentStress::C, VerbPastStress::A);
    pub const C_B: Self = Self::new(VerbPresentStress::C, VerbPastStress::B);
    pub const C_C: Self = Self::new(VerbPresentStress::C, VerbPastStress::C);
    pub const C_Cp: Self = Self::new(VerbPresentStress::C, VerbPastStress::Cp);
    pub const C_Cpp: Self = Self::new(VerbPresentStress::C, VerbPastStress::Cpp);

    pub const Cp: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::A);
    pub const Cp_A: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::A);
    pub const Cp_B: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::B);
    pub const Cp_C: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::C);
    pub const Cp_Cp: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::Cp);
    pub const Cp_Cpp: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::Cpp);
}
