/// Represents a Russian stress schema.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Stress {
    #[default]
    Zero = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    Ap = 7,
    Bp = 8,
    Cp = 9,
    Dp = 10,
    Ep = 11,
    Fp = 12,
    Cpp = 13,
    Fpp = 14,
}

/// Represents a Russian dual stress schema, for main and alt forms of the word.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DualStress {
    main: Stress,
    alt: Stress,
}

// Trait providing Stress values
#[const_trait]
pub trait HasStress {
    fn main_stress(self) -> Stress;
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
    // Simple stress value checks
    pub const fn is_zero(self) -> bool {
        matches!(self, Stress::Zero)
    }
    pub const fn or(self, default: Self) -> Stress {
        if self.is_zero() { default } else { self }
    }
    pub const fn has_any_primes(self) -> bool {
        !matches!(
            self,
            Self::Zero | Self::A | Self::B | Self::C | Self::D | Self::E | Self::F
        )
    }
    pub const fn has_single_prime(self) -> bool {
        matches!(
            self,
            Self::Ap | Self::Bp | Self::Cp | Self::Dp | Self::Ep | Self::Fp
        )
    }
    pub const fn has_double_prime(self) -> bool {
        matches!(self, Self::Cpp | Self::Fpp)
    }

    // Removing primes from a stress value
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

    // Adding primes to a letter stress
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
    pub const fn add_double_prime(self) -> Option<Stress> {
        Some(match self {
            Self::C => Self::Cpp,
            Self::F => Self::Fpp,
            _ => return None,
        })
    }
}

// Constructing DualStress
impl DualStress {
    pub const ZERO: DualStress = Self::new(Stress::Zero, Stress::Zero);

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

// Normalizing DualStress values
impl DualStress {
    pub const fn normalize_adj(self) -> Self {
        if self.alt.is_zero() {
            return Self::new(self.main.unprime(), self.main);
        }
        self
    }
    pub const fn normalize_verb(self) -> Self {
        if self.alt.is_zero() {
            return Self::new(self.main, Stress::A);
        }
        self
    }
}

// Formatting Stress and DualStress
impl Stress {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseStressError {
    Empty,
    InvalidPrime,
    InvalidFormat,
}

impl Stress {
    pub const fn from_str_or_empty(text: &str) -> Result<Self, ParseStressError> {
        if text.is_empty() {
            Ok(Stress::Zero)
        } else {
            Self::from_str(text)
        }
    }
    pub const fn from_str(text: &str) -> Result<Self, ParseStressError> {
        let text = text.as_bytes();

        let letter = match text.first() {
            None => return Err(ParseStressError::Empty),
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
    pub fn from_str_or_empty(text: &str) -> Result<Self, ParseStressError> {
        match text.split_once('/') {
            Some((left, right)) => Ok(Self::new(
                Stress::from_str_or_empty(left)?,
                Stress::from_str_or_empty(right)?,
            )),
            None => Ok(Self::new(Stress::from_str_or_empty(text)?, Stress::Zero)),
        }
    }
    pub fn from_str(text: &str) -> Result<Self, ParseStressError> {
        match text.split_once('/') {
            Some((left, right)) => Ok(Self::new(
                Stress::from_str_or_empty(left)?,
                Stress::from_str_or_empty(right)?,
            )),
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
        assert_eq!(Stress::from_str(""), Err(ParseStressError::Empty));
        assert_eq!(Stress::from_str_or_empty(""), Ok(Zero));
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
        assert_eq!(DualStress::from_str(""), Err(ParseStressError::Empty));
        assert_eq!(DualStress::from_str_or_empty(""), Ok(DualStress::ZERO));
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
