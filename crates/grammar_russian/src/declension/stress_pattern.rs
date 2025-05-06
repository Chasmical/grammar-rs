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
    const MIN_1_PRIME_U8: u8 = Stress::Ap as u8;
    const MIN_2_PRIME_U8: u8 = Stress::Cpp as u8;

    // Simple stress value checks
    pub const fn is_zero(self) -> bool {
        matches!(self, Stress::Zero)
    }
    pub const fn or(self, default: Self) -> Self {
        if self.is_zero() { default } else { self }
    }
    pub const fn has_any_primes(self) -> bool {
        matches!(self as u8, Self::MIN_1_PRIME_U8..)
    }
    pub const fn has_single_prime(self) -> bool {
        matches!(self as u8, Self::MIN_1_PRIME_U8..Self::MIN_2_PRIME_U8)
    }
    pub const fn has_double_prime(self) -> bool {
        matches!(self as u8, Self::MIN_2_PRIME_U8..)
    }

    // Removing primes from a stress value
    pub const fn unprime(self) -> Stress {
        if self.has_any_primes() {
            return if self.has_double_prime() {
                match self {
                    Stress::Cpp => Stress::C,
                    _ => Stress::F,
                }
            } else {
                unsafe { std::mem::transmute(self as u8 - (Self::MIN_1_PRIME_U8 - 1)) }
            };
        }
        self
    }
}

// Constructing DualStress
impl DualStress {
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

// Normalizing dual stress values
impl DualStress {
    pub const fn normalize_adj(self) -> Self {
        if self.alt.is_zero() {
            return Self {
                alt: self.main,
                main: self.main.unprime(),
            };
        }
        self
    }
    pub const fn normalize_verb(self) -> Self {
        if self.alt.is_zero() {
            return Self {
                alt: Stress::A,
                ..self
            };
        }
        self
    }
}

// Formatting Stress and DualStress
impl Stress {
    pub const fn fmt_to(self, dst: &mut [u8; 4]) -> &mut str {
        // If stress has primes, it will occupy the entire 4 byte buffer
        if self.has_any_primes() {
            // Contains the first two bytes of ′ and ″
            *dst = [0, 0xE2, 0x80, 0];

            // Write the letter: a, b, c, d, e, f
            dst[0] = (b'a' - 1) + self.unprime() as u8;
            // Write the last byte of ′ or ″
            dst[3] = if self.has_double_prime() { 0xB3 } else { 0xB2 };

            // Return string slice from the entire buffer
            return unsafe { str::from_utf8_unchecked_mut(dst) };
        }

        // If zero, don't write anything
        if self.is_zero() {
            // Return string slice of length 0
            let slice = unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr(), 0) };
            return unsafe { str::from_utf8_unchecked_mut(slice) };
        }

        // Write just the letter character
        dst[0] = (b'a' - 1) + self as u8;

        // Return string slice of length 1
        let slice = unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr(), 1) };
        return unsafe { str::from_utf8_unchecked_mut(slice) };
    }
}
impl DualStress {
    pub const fn fmt_to(self, dst: &mut [u8; 9]) -> &str {
        let mut offset: usize = 0;

        if !self.main.is_zero() {
            // Cast slice to one of 4-byte length
            let buffer = Self::const_slice_4_array(dst, offset);

            offset += self.main.fmt_to(buffer).len();
        }

        if !self.alt.is_zero() {
            // Append '/' as a separator, even if main is not present
            dst[offset] = '/' as u8;
            offset += 1;

            // Cast slice to one of 4-byte length
            let buffer = Self::const_slice_4_array(dst, offset);

            offset += self.alt.fmt_to(buffer).len();
        }

        // Return string slice with current offset as length
        let slice = unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr(), offset) };
        return unsafe { str::from_utf8_unchecked_mut(slice) };
    }
    #[inline]
    const fn const_slice_4_array(dst: &mut [u8; 9], offset: usize) -> &mut [u8; 4] {
        unsafe { &mut *(dst.as_mut_ptr().add(offset) as *mut [u8; 4]) }
    }
}

impl std::fmt::Display for Stress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer: [u8; 4] = [0; 4];
        f.write_str(self.fmt_to(&mut buffer))
    }
}
impl std::fmt::Display for DualStress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer: [u8; 9] = [0; 9];
        f.write_str(self.fmt_to(&mut buffer))
    }
}
