use crate::util::{define_error, define_subenum};

define_error! {
    pub struct AnyStressError("words can only have stresses a-f, a′-f′, c″ and f″");
}
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

define_error! {
    pub struct NounStressError("nouns can only have stresses a, b, c, d, e, f, b′, d′, f′ and f″");
}
define_subenum! {
    pub enum NounStress from AnyStress [NounStressError] {
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
}

define_error! {
    pub struct AdjectiveFullStressError("adjectives (full form) can only have stresses a and b");
}
define_subenum! {
    pub enum AdjectiveFullStress from AnyStress [AdjectiveFullStressError] {
        /// Stress schema `a`. Stress is always on the stem.
        A,
        /// Stress schema `b`. Stress is always on the ending.
        B,
    }
}

define_error! {
    pub struct AdjectiveShortStressError("adjectives (short form) can only have stresses a, b, c, a′, b′, c′ and c″");
}
define_subenum! {
    pub enum AdjectiveShortStress from AnyStress [AdjectiveShortStressError] {
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
}

define_error! {
    pub struct PronounStressError("pronouns can only have stresses a, b and f");
}
define_subenum! {
    pub enum PronounStress from AnyStress [PronounStressError] {
        /// Stress schema `a`. Stress is always on the stem.
        A,
        /// Stress schema `b`. Stress is always on the ending.
        B,
        /// Stress schema `f`. Plural nominative - stress on stem, all other - stress on ending.
        F,
    }
}

define_error! {
    pub struct VerbPresentStressError("verbs (present tense) only have stresses a, b, c and c′");
}
define_subenum! {
    pub enum VerbPresentStress from AnyStress [VerbPresentStressError] {
        /// Stress schema `a`. Stress is always on the stem.
        A,
        /// Stress schema `b`. Stress is always on the ending.
        B,
        /// Stress schema `c`. First person, and imperative - stress on ending, all other - stress on stem.
        C,
        /// Stress schema `c′` (`c` with single prime). First person, imperative, and plural - stress on ending, all other - stress on stem.
        Cp,
    }
}

define_error! {
    pub struct VerbPastStressError("verbs (past tense) only have stresses a, b, c, c′ and c″");
}
define_subenum! {
    pub enum VerbPastStress from AnyStress [VerbPastStressError] {
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
}

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

impl AnyDualStress {
    pub const fn new(main: AnyStress, alt: Option<AnyStress>) -> Self {
        Self { main, alt }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjectiveStressError {
    Full(AdjectiveFullStressError),
    Short(AdjectiveShortStressError),
}
impl AdjectiveStress {
    pub const fn new(full: AdjectiveFullStress, short: AdjectiveShortStress) -> Self {
        Self { full, short }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbStressError {
    Present(VerbPresentStressError),
    Past(VerbPastStressError),
}
impl VerbStress {
    pub const fn new(present: VerbPresentStress, past: VerbPastStress) -> Self {
        Self { present, past }
    }
}

//                         TABLE OF STRESS TYPE CONVERSIONS
// ┌———————┬——————┬——————┬——————┬——————┬——————┬——————┬——————╥——————┬——————┬——————┐
// │From\To│ Any  │ Noun │ AdjF │ AdjS │ Pro  │ VerbF│ VerbP║ ANY  │ ADJ  │ VERB │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Any   │ ———— │  []  │  []  │  []  │  []  │  []  │  []  ║  ██  │  []  │  []  │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Noun  │  ██  │ ———— │      │      │      │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ AdjF  │  ██  │      │ ———— │      │      │      │      ║  ██  │  ██  │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ AdjS  │  ██  │      │      │ ———— │      │      │      ║  ██  │  []  │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Pro   │  ██  │      │      │      │ ———— │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VerbF │  ██  │      │      │      │      │ ———— │      ║  ██  │      │  ██  │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VerbP │  ██  │      │      │      │      │      │ ———— ║  ██  │      │  []  │
// ╞═══════╪══════╪══════╪══════╪══════╪══════╪══════╪══════╬══════╪══════╪══════╡
// │ ANY   │  []  │      │      │      │      │      │      ║ ———— │  []  │  []  │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ ADJ   │      │      │      │      │      │      │      ║  ██  │ ———— │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VERB  │      │      │      │      │      │      │      ║  ██  │      │ ———— │
// └———————┴——————┴——————┴——————┴——————┴——————┴——————┴——————╨——————┴——————┴——————┘
//                                                     ██ — From   [] — TryFrom

impl<T: Into<AnyStress>> From<T> for AnyDualStress {
    fn from(value: T) -> Self {
        Self::new(value.into(), None)
    }
}
impl TryFrom<AnyDualStress> for AnyStress {
    type Error = ();
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() {
            Ok(value.main)
        } else {
            Err(())
        }
    }
}

impl From<AdjectiveStress> for AnyDualStress {
    fn from(value: AdjectiveStress) -> Self {
        Self::new(value.full.into(), Some(value.short.into()))
    }
}
impl From<VerbStress> for AnyDualStress {
    fn from(value: VerbStress) -> Self {
        Self::new(value.present.into(), Some(value.past.into()))
    }
}

impl From<AdjectiveFullStress> for AdjectiveStress {
    fn from(value: AdjectiveFullStress) -> Self {
        Self::new(value, match value {
            AdjectiveFullStress::A => AdjectiveShortStress::A,
            AdjectiveFullStress::B => AdjectiveShortStress::B,
        })
    }
}
impl TryFrom<AdjectiveShortStress> for AdjectiveStress {
    type Error = AdjectiveFullStressError;
    fn try_from(value: AdjectiveShortStress) -> Result<Self, Self::Error> {
        Ok(Self::new(AnyStress::from(value).try_into()?, value))
    }
}
impl TryFrom<AnyStress> for AdjectiveStress {
    type Error = AdjectiveFullStressError;
    fn try_from(value: AnyStress) -> Result<Self, Self::Error> {
        AdjectiveShortStress::try_from(value).map_or(Err(Self::Error {}), |x| x.try_into())
    }
}

impl From<VerbPresentStress> for VerbStress {
    fn from(value: VerbPresentStress) -> Self {
        Self::new(value, VerbPastStress::A)
    }
}
impl TryFrom<VerbPastStress> for VerbStress {
    type Error = VerbPresentStressError;
    fn try_from(value: VerbPastStress) -> Result<Self, Self::Error> {
        Ok(Self::new(AnyStress::from(value).try_into()?, value))
    }
}
impl TryFrom<AnyStress> for VerbStress {
    type Error = VerbPresentStressError;
    fn try_from(value: AnyStress) -> Result<Self, Self::Error> {
        VerbPastStress::try_from(value).map_or(Err(Self::Error {}), |x| x.try_into())
    }
}

impl From<AdjectiveFullStressError> for AdjectiveStressError {
    fn from(value: AdjectiveFullStressError) -> Self {
        Self::Full(value)
    }
}
impl From<AdjectiveShortStressError> for AdjectiveStressError {
    fn from(value: AdjectiveShortStressError) -> Self {
        Self::Short(value)
    }
}
impl TryFrom<AnyDualStress> for AdjectiveStress {
    type Error = AdjectiveStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if let Some(alt) = value.alt {
            Ok(Self::new(value.main.try_into()?, alt.try_into()?))
        } else {
            let alt = value.main.try_into()?;
            Ok(Self::new(AnyStress::from(alt).try_into()?, alt))
        }
    }
}

impl From<VerbPresentStressError> for VerbStressError {
    fn from(value: VerbPresentStressError) -> Self {
        Self::Present(value)
    }
}
impl From<VerbPastStressError> for VerbStressError {
    fn from(value: VerbPastStressError) -> Self {
        Self::Past(value)
    }
}
impl TryFrom<AnyDualStress> for VerbStress {
    type Error = VerbStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.main.try_into()?,
            value.alt.map_or(Ok(VerbPastStress::A), |x| x.try_into())?,
        ))
    }
}

#[macro_export]
macro_rules! stress {
    ($x:ident) => ({
        stress![@res $x].try_into().unwrap()
    });
    ($x:ident/$y:ident) => ({
        AnyDualStress::new(stress![@res $x], Some(stress![@res $y])).try_into().unwrap()
    });
    (@res $x:ident) => (
        #[allow(non_upper_case_globals)]
        {
            const a: AnyStress = AnyStress::A;
            const b: AnyStress = AnyStress::B;
            const c: AnyStress = AnyStress::C;
            const d: AnyStress = AnyStress::D;
            const e: AnyStress = AnyStress::E;
            const f: AnyStress = AnyStress::F;
            const a1: AnyStress = AnyStress::Ap;
            const b1: AnyStress = AnyStress::Bp;
            const c1: AnyStress = AnyStress::Cp;
            const d1: AnyStress = AnyStress::Dp;
            const e1: AnyStress = AnyStress::Ep;
            const f1: AnyStress = AnyStress::Fp;
            const c2: AnyStress = AnyStress::Cpp;
            const f2: AnyStress = AnyStress::Fpp;
            $x
        }
    );
}

#[allow(unused_variables, dead_code)]
fn test_conversions() {
    // TODO: make into a proper test

    let i: NounStress = stress![a];
    let j: AdjectiveStress = stress![a1];
    let k: VerbStress = stress![b / c2];
    let l: AdjectiveStress = stress![f2 / f2]; // TODO: should panic

    let x = NounStress::A;
    let y: AnyDualStress = x.into(); // should be a/

    let a = AdjectiveFullStress::A;
    let b: AdjectiveStress = a.into(); // should be a/a

    let u = AnyStress::Bp;
    let v: AdjectiveStress = u.try_into().unwrap(); // should be b/b'
    let u = AdjectiveShortStress::Bp;
    let v: AdjectiveStress = u.try_into().unwrap(); // should be b/b'

    let z = AnyDualStress::new(AnyStress::B, None);
    let w: AdjectiveStress = z.try_into().unwrap(); // should be b/b
}
