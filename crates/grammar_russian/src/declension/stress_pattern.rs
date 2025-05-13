/// Represents a Russian stress schema.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Stress {
    /// Zero, or unspecified, stress.
    #[default]
    Zero = 0,
    /// Stress schema `a`. The stress is always on the stem. Used by all inflectable words.
    A = 1,
    /// Stress schema `b`. The stress is always on the ending. Used by all inflectable words.
    B = 2,
    /// Stress schema `c`.
    /// - Nouns: singular - stress on stem, plural - stress on ending.
    /// - Adjectives (short form only): feminine - stress on ending, all other - stress on stem.
    /// - Verbs (non-past tense): first person, and imperative - stress on ending, all other - stress on stem.
    /// - Verbs (past tense): feminine - stress on ending, all other - stress on stem.
    C = 3,
    /// Stress schema `d`.
    /// - Nouns: singular - stress on ending, plural - stress on stem.
    D = 4,
    /// Stress schema `e`.
    /// - Nouns: singular, and plural nominative - stress on stem, plural of other cases - stress on ending.
    E = 5,
    /// Stress schema `f`.
    /// - Nouns and pronouns: plural nominative - stress on stem, all other - stress on ending.
    F = 6,
    /// Stress schema `a′` (`a` with single prime).
    /// - Adjectives (short form only): feminine - both??? (resolved as on stem), all other - stress on stem.
    Ap = 7,
    /// Stress schema `b′` (`b` with single prime).
    /// - Nouns: singular instrumental - stress on stem, all other - stress on ending.
    /// - Adjectives (short form only): plural - both??? (resolved as on ending), all other - stress on ending.
    Bp = 8,
    /// Stress schema `c′` (`c` with single prime).
    /// - Adjectives (short form only): feminine - stress on ending, neuter - stress on stem, plural - TODO: both???.
    /// - Verbs (non-past tense): first person, imperative, and plural - stress on ending, all other - stress on stem.
    /// - Verbs (past tense): feminine - stress on ending, neuter - TODO: both???, all other - stress on stem.
    Cp = 9,
    /// Stress schema `d′` (`d` with single prime).
    /// - Nouns: singular accusative, and plural - stress on stem, singular of other cases - stress on ending.
    Dp = 10,
    /// Stress schema `e′` (`e` with single prime).
    /// TODO: Unused???
    Ep = 11,
    /// Stress schema `f′` (`f` with single prime).
    /// - Nouns: singular accusative, and plural nominative - stress on stem, all other - stress on ending.
    Fp = 12,
    /// Stress schema `c″` (`c` with double prime).
    /// - Adjectives (short form only): feminine - stress on ending, all other - both??? (resolved as on ending).
    /// - Verbs (past tense reflexive only): masculine - stress on stem, feminine - stress on ending, neuter and plural - TODO: both???.
    Cpp = 13,
    /// Stress schema `f″` (`f` with double prime).
    /// - Nouns: singular instrumental, and plural nominative - stress on stem, all other - stress on ending.
    Fpp = 14,
}

/// [`DualStress`] consists of two [`Stress`] values, for main and alternative forms of the word.
///
/// An adjective's alternative form is its short form, and a verb's is its past tense form.
/// Other parts of speech don't have alternative forms.
///
/// # Examples
/// ```
/// use grammar_russian::declension::{DualStress, HasStress, Stress};
///
/// let dual = DualStress::new(Stress::A, Stress::B);
/// assert_eq!(dual.main_stress(), Stress::A);
/// assert_eq!(dual.alt_stress(), Stress::B);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DualStress {
    main: Stress,
    alt: Stress,
}

/// Trait for providing main and alternative form [`Stress`]es.
///
/// An adjective's alternative form is its short form, and a verb's is its past tense form.
/// Other parts of speech don't have alternative forms.
///
/// # Examples
/// ```
/// use grammar_russian::declension::{DualStress, HasStress, Stress};
///
/// assert_eq!(Stress::C.main_stress(), Stress::C);
/// assert_eq!(Stress::C.alt_stress(), Stress::Zero);
///
/// let dual = DualStress::new(Stress::A, Stress::B);
/// assert_eq!(dual.main_stress(), Stress::A);
/// assert_eq!(dual.alt_stress(), Stress::B);
/// ```
#[const_trait]
pub trait HasStress {
    /// Returns the main form stress associated with the current value.
    ///
    /// # Examples
    /// ```
    /// use grammar_russian::declension::{DualStress, HasStress, Stress};
    ///
    /// assert_eq!(Stress::C.main_stress(), Stress::C);
    /// assert_eq!(DualStress::new(Stress::A, Stress::B).main_stress(), Stress::A);
    /// ```
    fn main_stress(self) -> Stress;
    /// Returns the alternative form stress associated with the current value.
    ///
    /// # Examples
    /// ```
    /// use grammar_russian::declension::{DualStress, HasStress, Stress};
    ///
    /// assert_eq!(Stress::C.alt_stress(), Stress::Zero);
    /// assert_eq!(DualStress::new(Stress::A, Stress::B).alt_stress(), Stress::B);
    /// ```
    fn alt_stress(self) -> Stress;
}

// Stress and DualStress provide their values
impl const HasStress for Stress {
    fn main_stress(self) -> Stress {
        self
    }
    fn alt_stress(self) -> Stress {
        Stress::Zero
    }
}
impl const HasStress for DualStress {
    fn main_stress(self) -> Stress {
        self.main
    }
    fn alt_stress(self) -> Stress {
        self.alt
    }
}

impl Stress {
    /// Returns `true` if the stress is a [`Stress::Zero`] value.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::Stress;
    /// #
    /// assert_eq!(Stress::Zero.is_zero(), true);
    /// assert_eq!(Stress::A.is_zero(), false);
    /// ```
    pub const fn is_zero(self) -> bool {
        matches!(self, Stress::Zero)
    }
    /// Returns the stress or the specified default value, if it's zero.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::Stress;
    /// #
    /// assert_eq!(Stress::Zero.or(Stress::C), Stress::C);
    /// assert_eq!(Stress::A.or(Stress::C), Stress::A);
    /// assert_eq!(Stress::Fpp.or(Stress::C), Stress::Fpp);
    /// ```
    pub const fn or(self, default: Self) -> Stress {
        if self.is_zero() { default } else { self }
    }
    /// Returns `true` if the stress has any primes.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::Stress;
    /// #
    /// assert_eq!(Stress::A.has_any_primes(), false);
    /// assert_eq!(Stress::Bp.has_any_primes(), true);
    /// assert_eq!(Stress::Fpp.has_any_primes(), true);
    /// ```
    pub const fn has_any_primes(self) -> bool {
        !matches!(
            self,
            Self::Zero | Self::A | Self::B | Self::C | Self::D | Self::E | Self::F
        )
    }
    /// Returns `true` if the stress has a single prime.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::Stress;
    /// #
    /// assert_eq!(Stress::A.has_single_prime(), false);
    /// assert_eq!(Stress::Bp.has_single_prime(), true);
    /// assert_eq!(Stress::Fpp.has_single_prime(), false);
    /// ```
    pub const fn has_single_prime(self) -> bool {
        matches!(
            self,
            Self::Ap | Self::Bp | Self::Cp | Self::Dp | Self::Ep | Self::Fp
        )
    }
    /// Returns `true` if the stress has a double prime.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::Stress;
    /// #
    /// assert_eq!(Stress::A.has_double_prime(), false);
    /// assert_eq!(Stress::Bp.has_double_prime(), false);
    /// assert_eq!(Stress::Fpp.has_double_prime(), true);
    /// ```
    pub const fn has_double_prime(self) -> bool {
        matches!(self, Self::Cpp | Self::Fpp)
    }

    /// Removes the primes from a stress value.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::Stress;
    /// #
    /// assert_eq!(Stress::A.unprime(), Stress::A);
    /// assert_eq!(Stress::Bp.unprime(), Stress::B);
    /// assert_eq!(Stress::Fpp.unprime(), Stress::F);
    /// ```
    pub const fn unprime(self) -> Stress {
        match self {
            Self::Zero => Self::Zero,
            Self::A | Self::Ap => Self::A,
            Self::B | Self::Bp => Self::B,
            Self::C | Self::Cp | Self::Cpp => Self::C,
            Self::D | Self::Dp => Self::D,
            Self::E | Self::Ep => Self::E,
            Self::F | Self::Fp | Self::Fpp => Self::F,
        }
    }

    /// Attempts to add a single prime to a simple stress value.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::Stress;
    /// #
    /// assert_eq!(Stress::Zero.add_single_prime(), None);
    /// assert_eq!(Stress::A.add_single_prime(), Some(Stress::Ap));
    /// assert_eq!(Stress::Bp.add_single_prime(), None);
    /// assert_eq!(Stress::Fpp.add_single_prime(), None);
    /// ```
    pub const fn add_single_prime(self) -> Option<Stress> {
        Some(match self {
            Self::A => Self::Ap,
            Self::B => Self::Bp,
            Self::C => Self::Cp,
            Self::D => Self::Dp,
            Self::E => Self::Ep,
            Self::F => Self::Fp,
            _ => return None,
        })
    }
    /// Attempts to add a double prime to a simple stress value.
    ///
    /// Note that [`Stress::C`] and [`Stress::F`] are the only simple stresses that can be
    /// converted into their double-primed counterparts [`Stress::Cpp`] and [`Stress::Fpp`].
    /// Other simple stresses don't have double-primed counterparts.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::Stress;
    /// #
    /// assert_eq!(Stress::Zero.add_double_prime(), None);
    /// assert_eq!(Stress::A.add_double_prime(), None);
    /// assert_eq!(Stress::C.add_double_prime(), Some(Stress::Cpp));
    /// assert_eq!(Stress::D.add_double_prime(), None);
    /// assert_eq!(Stress::F.add_double_prime(), Some(Stress::Fpp));
    /// assert_eq!(Stress::Bp.add_double_prime(), None);
    /// assert_eq!(Stress::Fpp.add_double_prime(), None);
    /// ```
    pub const fn add_double_prime(self) -> Option<Stress> {
        Some(match self {
            Self::C => Self::Cpp,
            Self::F => Self::Fpp,
            _ => return None,
        })
    }
}

impl DualStress {
    /// Zero dual stress, consisting of [`Stress::Zero`] and [`Stress::Zero`].
    pub const ZERO: DualStress = Self::new(Stress::Zero, Stress::Zero);

    /// Creates a new [`DualStress`] with the given main and alternative form stresses.
    ///
    /// # Examples
    /// ```
    /// use grammar_russian::declension::{DualStress, HasStress, Stress};
    ///
    /// let dual = DualStress::new(Stress::A, Stress::B);
    /// assert_eq!(dual.main_stress(), Stress::A);
    /// assert_eq!(dual.alt_stress(), Stress::B);
    /// ```
    pub const fn new(main: Stress, alt: Stress) -> Self {
        DualStress { main, alt }
    }
}
impl From<(Stress, Stress)> for DualStress {
    fn from(value: (Stress, Stress)) -> Self {
        DualStress::new(value.0, value.1)
    }
}
impl From<Stress> for DualStress {
    fn from(value: Stress) -> Self {
        DualStress::new(value, Stress::Zero)
    }
}

impl DualStress {
    /// Converts dual stress from shortened dictionary form (as specified for _adjectives_)
    /// to the full normalized form that's easier to process (e.g. `c'` to `c/c'`).
    ///
    /// If alternative stress is zero, then it is set to main, and main is unprimed.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::DualStress;
    /// #
    /// assert_normalize_adj("a/b", "a/b");
    /// assert_normalize_adj("a", "a/a");
    /// assert_normalize_adj("b", "b/b");
    /// assert_normalize_adj("c'", "c/c'");
    /// assert_normalize_adj("f\"", "f/f\"");
    ///
    /// fn assert_normalize_adj(left: &str, right: &str) {
    ///   let left: DualStress = left.parse().unwrap();
    ///   let right: DualStress = right.parse().unwrap();
    ///   assert_eq!(left.normalize_adj(), right);
    /// }
    /// ```
    pub const fn normalize_adj(self) -> Self {
        if self.alt.is_zero() {
            return Self::new(self.main.unprime(), self.main);
        }
        self
    }
    /// Converts dual stress from shortened dictionary form (as specified for _verbs_)
    /// to the full normalized form that's easier to process (e.g. `c'` to `c'/a`).
    ///
    /// If alternative stress is zero, then it is set to `a`.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::DualStress;
    /// #
    /// assert_normalize_verb("a/b", "a/b");
    /// assert_normalize_verb("a", "a/a");
    /// assert_normalize_verb("b", "b/a");
    /// assert_normalize_verb("c'", "c'/a");
    /// assert_normalize_verb("f\"", "f\"/a");
    ///
    /// fn assert_normalize_verb(left: &str, right: &str) {
    ///   let left: DualStress = left.parse().unwrap();
    ///   let right: DualStress = right.parse().unwrap();
    ///   assert_eq!(left.normalize_verb(), right);
    /// }
    /// ```
    pub const fn normalize_verb(self) -> Self {
        if self.alt.is_zero() {
            return Self::new(self.main, Stress::A);
        }
        self
    }

    /// Converts dual stress from normalized form to shortened dictionary form
    /// (as specified for _adjectives_, e.g. `c/c'` to `c'`).
    ///
    /// If main stress is equal to unprimed alternative stress, alternative is set to 0.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::DualStress;
    /// #
    /// assert_shorten_adj("a/b", "a/b");
    /// assert_shorten_adj("a/a", "a");
    /// assert_shorten_adj("b/b", "b");
    /// assert_shorten_adj("c/c'", "c'");
    /// assert_shorten_adj("f/f\"", "f\"");
    ///
    /// fn assert_shorten_adj(left: &str, right: &str) {
    ///   let left: DualStress = left.parse().unwrap();
    ///   let right: DualStress = right.parse().unwrap();
    ///   assert_eq!(left.shorten_adj(), right);
    /// }
    /// ```
    pub const fn shorten_adj(self) -> Self {
        if self.main as u8 == self.alt.unprime() as u8 {
            return Self::new(self.alt, Stress::Zero);
        }
        self
    }
    /// Converts dual stress from normalized form to shortened dictionary form
    /// (as specified for _verbs_, e.g. `c'/a` to `c'`).
    ///
    /// If alternative stress is `a`, then it is set to 0.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::DualStress;
    /// #
    /// assert_shorten_verb("a/b", "a/b");
    /// assert_shorten_verb("a/a", "a");
    /// assert_shorten_verb("b/a", "b");
    /// assert_shorten_verb("c'/a", "c'");
    /// assert_shorten_verb("f\"/a", "f\"");
    ///
    /// fn assert_shorten_verb(left: &str, right: &str) {
    ///   let left: DualStress = left.parse().unwrap();
    ///   let right: DualStress = right.parse().unwrap();
    ///   assert_eq!(left.shorten_verb(), right);
    /// }
    /// ```
    pub const fn shorten_verb(self) -> Self {
        if self.alt as u8 == Stress::A as u8 {
            return Self::new(self.main, Stress::Zero);
        }
        self
    }
}

impl Stress {
    /// Formats the stress into the provided byte buffer, returning
    /// a subslice of the buffer containing the formatted stress.
    ///
    /// Requires the buffer to be 4 bytes long via `[u8; 4]`.
    /// If the length of your buffer is different, use [`slice::first_chunk_mut`]:
    ///
    /// ```
    /// # use grammar_russian::declension::Stress;
    /// #
    /// let buffer = &mut [0; 32][..];
    /// let dst = buffer.first_chunk_mut::<4>().unwrap();
    /// assert_eq!(Stress::Fpp.fmt_to(dst), "f″");
    /// ```
    pub const fn fmt_to(self, dst: &mut [u8; 4]) -> &mut str {
        // If zero, don't write anything
        if self.is_zero() {
            // Return string slice of length 0
            let slice = unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr(), 0) };
            return unsafe { str::from_utf8_unchecked_mut(slice) };
        }

        // Write the letter: a, b, c, d, e, f
        dst[0] = match self.unprime() {
            Stress::A => b'a',
            Stress::B => b'b',
            Stress::C => b'c',
            Stress::D => b'd',
            Stress::E => b'e',
            Stress::F => b'f',
            _ => unreachable!(),
        };

        // If stress has primes, it will occupy the entire 4 byte buffer
        if self.has_any_primes() {
            // // Write the UTF-8 bytes of ′ or ″
            let ch = match self.has_double_prime() {
                true => '″',
                false => '′',
            };
            ch.encode_utf8(dst.last_chunk_mut::<3>().unwrap());

            // Return string slice from the entire buffer
            return unsafe { str::from_utf8_unchecked_mut(dst) };
        }

        // Return string slice of length 1
        let slice = unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr(), 1) };
        return unsafe { str::from_utf8_unchecked_mut(slice) };
    }
}
impl DualStress {
    /// Formats the stress into the provided byte buffer, returning
    /// a subslice of the buffer containing the formatted stress.
    ///
    /// Requires the buffer to be 9 bytes long via `[u8; 9]`.
    /// If the length of your buffer is different, use [`slice::first_chunk_mut`]:
    ///
    /// ```
    /// # use grammar_russian::declension::{DualStress, Stress};
    /// #
    /// let buffer = &mut [0; 32][..];
    /// let dst = buffer.first_chunk_mut::<9>().unwrap();
    /// assert_eq!(DualStress::new(Stress::D, Stress::Fpp).fmt_to(dst), "d/f″");
    /// ```
    pub const fn fmt_to(self, dst: &mut [u8; 9]) -> &str {
        let mut offset: usize = 0;

        if !self.main.is_zero() {
            // Format main into a 4-byte sub-buffer
            offset += self.main.fmt_to(const_slice(dst, offset)).len();
        }

        if !self.alt.is_zero() {
            // Append '/' as a separator, even if main is not present
            dst[offset] = b'/';
            offset += 1;

            // Format alt into a 4-byte sub-buffer
            offset += self.alt.fmt_to(const_slice(dst, offset)).len();
        }

        // Return string slice with current offset as length
        let slice = unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr(), offset) };
        return unsafe { str::from_utf8_unchecked_mut(slice) };

        const fn const_slice(dst: &mut [u8; 9], offset: usize) -> &mut [u8; 4] {
            unsafe { &mut *(dst.as_mut_ptr().add(offset).cast::<[u8; 4]>()) }
        }
    }
}

impl std::fmt::Display for Stress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.fmt_to(&mut [0; 4]))
    }
}
impl std::fmt::Display for DualStress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.fmt_to(&mut [0; 9]))
    }
}

/// Error returned when a [`Stress`] or [`DualStress`] cannot be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseStressError {
    /// Invalid double-primed letter, such as a″, b″, d″, or e″.
    InvalidPrime,
    /// The given string slice was of a generally invalid format.
    InvalidFormat,
}
impl std::fmt::Display for ParseStressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ParseStressError::InvalidPrime => "",
            ParseStressError::InvalidFormat => "",
        })
    }
}
impl std::error::Error for ParseStressError {}

impl Stress {
    /// Parses a [`Stress`] value from a string slice.
    ///
    /// The string is expected to be a lowercase latin letter (`a-f`), optionally
    /// followed by a prime modifier (Unicode: `′`, `″`; ASCII: `'`, `"`, `''`).
    ///
    /// Only `c` and `f` can have a double-prime modifier.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::{ParseStressError, Stress};
    /// #
    /// assert_eq!(Stress::from_str(""), Ok(Stress::Zero));
    /// assert_eq!(Stress::from_str("a"), Ok(Stress::A));
    /// assert_eq!(Stress::from_str("b′"), Ok(Stress::Bp));
    /// assert_eq!(Stress::from_str("c″"), Ok(Stress::Cpp));
    ///
    /// assert_eq!(Stress::from_str("e″"), Err(ParseStressError::InvalidPrime));
    /// assert_eq!(Stress::from_str("z"), Err(ParseStressError::InvalidFormat));
    /// ```
    pub const fn from_str(text: &str) -> Result<Self, ParseStressError> {
        let text = text.as_bytes();

        let letter = match text.first() {
            None => return Ok(Stress::Zero),
            Some(b'a') => Stress::A,
            Some(b'b') => Stress::B,
            Some(b'c') => Stress::C,
            Some(b'd') => Stress::D,
            Some(b'e') => Stress::E,
            Some(b'f') => Stress::F,
            _ => return Err(ParseStressError::InvalidFormat),
        };

        match text {
            // Simple one-letter stress
            [_] => Ok(letter),
            // Single-prime: apostrophe, or UTF-8 char
            [_, b'\''] | [_, 0xE2, 0x80, 0xB2] => match letter.add_single_prime() {
                Some(stress) => Ok(stress),
                None => Err(ParseStressError::InvalidPrime),
            },
            // Double-prime: quotation, two apostrophes, or UTF-8 char
            [_, b'"'] | [_, b'\'', b'\''] | [_, 0xE2, 0x80, 0xB3] => {
                match letter.add_double_prime() {
                    Some(stress) => Ok(stress),
                    None => Err(ParseStressError::InvalidPrime),
                }
            },
            // All other cases are invalid
            _ => Err(ParseStressError::InvalidFormat),
        }
    }
}
impl DualStress {
    /// Parses a [`DualStress`] value from a string slice.
    ///
    /// The string is expected to be either one or two [`Stress`] values separated by `/`
    /// (`main/alt` or `main`). The alternative stress defaults to 0.
    ///
    /// Usually the parsed value is then normalized with [`DualStress::normalize_adj`]
    /// or [`DualStress::normalize_verb`] to handle dictionary shorthands.
    ///
    /// # Examples
    /// ```
    /// # use grammar_russian::declension::{DualStress, ParseStressError, Stress};
    /// #
    /// assert_eq!(DualStress::from_str(""), Ok(DualStress::new(Stress::Zero, Stress::Zero)));
    /// assert_eq!(DualStress::from_str("/"), Ok(DualStress::new(Stress::Zero, Stress::Zero)));
    /// assert_eq!(DualStress::from_str("a"), Ok(DualStress::new(Stress::A, Stress::Zero)));
    /// assert_eq!(DualStress::from_str("a'/b"), Ok(DualStress::new(Stress::Ap, Stress::B)));
    /// assert_eq!(DualStress::from_str("c/f\""), Ok(DualStress::new(Stress::C, Stress::Fpp)));
    ///
    /// assert_eq!(DualStress::from_str("c/e″"), Err(ParseStressError::InvalidPrime));
    /// assert_eq!(DualStress::from_str("x/y"), Err(ParseStressError::InvalidFormat));
    /// ```
    pub fn from_str(text: &str) -> Result<Self, ParseStressError> {
        match text.split_once('/') {
            Some((left, right)) => Ok(Self::new(Stress::from_str(left)?, Stress::from_str(right)?)),
            None => Ok(Self::new(Stress::from_str(text)?, Stress::Zero)),
        }
    }
}

impl std::str::FromStr for Stress {
    type Err = ParseStressError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::from_str(text)
    }
}
impl std::str::FromStr for DualStress {
    type Err = ParseStressError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::from_str(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Stress::*;

    #[test]
    fn stress_fmt() {
        assert_eq!(Zero.to_string(), "");
        // Simple
        assert_eq!(A.to_string(), "a");
        assert_eq!(B.to_string(), "b");
        assert_eq!(C.to_string(), "c");
        assert_eq!(D.to_string(), "d");
        assert_eq!(E.to_string(), "e");
        assert_eq!(F.to_string(), "f");
        // Single-primed
        assert_eq!(Ap.to_string(), "a′");
        assert_eq!(Bp.to_string(), "b′");
        assert_eq!(Cp.to_string(), "c′");
        assert_eq!(Dp.to_string(), "d′");
        assert_eq!(Ep.to_string(), "e′");
        assert_eq!(Fp.to_string(), "f′");
        // Double-primed
        assert_eq!(Cpp.to_string(), "c″");
        assert_eq!(Fpp.to_string(), "f″");
    }

    #[test]
    fn dual_stress_fmt() {
        // Both zeroes
        assert_eq!(DualStress::new(Zero, Zero).to_string(), "");
        // Main stress only
        assert_eq!(DualStress::new(A, Zero).to_string(), "a");
        assert_eq!(DualStress::new(F, Zero).to_string(), "f");
        assert_eq!(DualStress::new(Ap, Zero).to_string(), "a′");
        assert_eq!(DualStress::new(Fp, Zero).to_string(), "f′");
        assert_eq!(DualStress::new(Cpp, Zero).to_string(), "c″");
        assert_eq!(DualStress::new(Fpp, Zero).to_string(), "f″");
        // Alt stress only
        assert_eq!(DualStress::new(Zero, A).to_string(), "/a");
        assert_eq!(DualStress::new(Zero, F).to_string(), "/f");
        assert_eq!(DualStress::new(Zero, Ap).to_string(), "/a′");
        assert_eq!(DualStress::new(Zero, Fp).to_string(), "/f′");
        assert_eq!(DualStress::new(Zero, Cpp).to_string(), "/c″");
        assert_eq!(DualStress::new(Zero, Fpp).to_string(), "/f″");
        // Both stresses
        assert_eq!(DualStress::new(A, B).to_string(), "a/b");
        assert_eq!(DualStress::new(E, Ep).to_string(), "e/e′");
        assert_eq!(DualStress::new(Ep, E).to_string(), "e′/e");
        assert_eq!(DualStress::new(Fp, Fpp).to_string(), "f′/f″");
        assert_eq!(DualStress::new(Fpp, Fp).to_string(), "f″/f′");
    }

    #[test]
    fn stress_parse() {
        // Zero/empty
        assert_eq!(Stress::from_str(""), Ok(Zero));
        // Simple
        assert_eq!(Stress::from_str("a"), Ok(A));
        assert_eq!(Stress::from_str("b"), Ok(B));
        assert_eq!(Stress::from_str("c"), Ok(C));
        assert_eq!(Stress::from_str("d"), Ok(D));
        assert_eq!(Stress::from_str("e"), Ok(E));
        assert_eq!(Stress::from_str("f"), Ok(F));
        // Single-primed
        assert_eq!(Stress::from_str("a'"), Ok(Ap));
        assert_eq!(Stress::from_str("f′"), Ok(Fp));
        // Double-primed
        assert_eq!(Stress::from_str("c''"), Ok(Cpp));
        assert_eq!(Stress::from_str("c\""), Ok(Cpp));
        assert_eq!(Stress::from_str("f″"), Ok(Fpp));
    }
    #[test]
    fn dual_stress_parse() {
        // Zero/empty
        assert_eq!(DualStress::from_str(""), Ok(DualStress::ZERO));
        assert_eq!(DualStress::from_str("/"), Ok(DualStress::ZERO));
        // Main stress only
        assert_eq!(DualStress::from_str("a"), Ok(DualStress::new(A, Zero)));
        assert_eq!(DualStress::from_str("f/"), Ok(DualStress::new(F, Zero)));
        assert_eq!(DualStress::from_str("a′"), Ok(DualStress::new(Ap, Zero)));
        assert_eq!(DualStress::from_str("f'/"), Ok(DualStress::new(Fp, Zero)));
        assert_eq!(DualStress::from_str("c″"), Ok(DualStress::new(Cpp, Zero)));
        assert_eq!(DualStress::from_str("f\"/"), Ok(DualStress::new(Fpp, Zero)));
        // Alt stress only
        assert_eq!(DualStress::from_str("/a"), Ok(DualStress::new(Zero, A)));
        assert_eq!(DualStress::from_str("/f"), Ok(DualStress::new(Zero, F)));
        assert_eq!(DualStress::from_str("/a′"), Ok(DualStress::new(Zero, Ap)));
        assert_eq!(DualStress::from_str("/f'"), Ok(DualStress::new(Zero, Fp)));
        assert_eq!(DualStress::from_str("/c″"), Ok(DualStress::new(Zero, Cpp)));
        assert_eq!(DualStress::from_str("/f\""), Ok(DualStress::new(Zero, Fpp)));
        // Both stresses
        assert_eq!(DualStress::from_str("a/b"), Ok(DualStress::new(A, B)));
        assert_eq!(DualStress::from_str("e/e′"), Ok(DualStress::new(E, Ep)));
        assert_eq!(DualStress::from_str("e'/e"), Ok(DualStress::new(Ep, E)));
        assert_eq!(DualStress::from_str("f'/f″"), Ok(DualStress::new(Fp, Fpp)));
        assert_eq!(DualStress::from_str("f''/f′"), Ok(DualStress::new(Fpp, Fp)));
    }
}
