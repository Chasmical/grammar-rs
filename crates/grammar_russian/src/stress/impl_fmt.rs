use super::defs::*;

impl AnyStress {
    pub const fn fmt_to(self, dst: &mut [u8; 4]) -> &mut str {
        // Write the letter: a, b, c, d, e, f
        dst[0] = match self.unprime() {
            Self::A => b'a',
            Self::B => b'b',
            Self::C => b'c',
            Self::D => b'd',
            Self::E => b'e',
            Self::F => b'f',
            _ => unreachable!(),
        };

        // If the stress has primes, it will occupy the entire 4 byte buffer
        if self.has_any_primes() {
            // Write the UTF-8 bytes of ′ or ″
            let ch = match self.has_double_prime() {
                false => '′',
                true => '″',
            };
            ch.encode_utf8(dst.last_chunk_mut::<3>().unwrap());

            // Return string slice from the entire buffer
            return unsafe { str::from_utf8_unchecked_mut(dst) };
        }

        // Return string slice of length 1, containing only the letter
        let slice = unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr(), 1) };
        return unsafe { str::from_utf8_unchecked_mut(slice) };
    }
}
impl AnyDualStress {
    pub const fn fmt_to(self, dst: &mut [u8; 9]) -> &mut str {
        let mut offset = 0;

        // Format main into a 4-byte sub-buffer
        offset += self.main.fmt_to(const_slice(dst, offset)).len();

        if let Some(alt) = self.alt {
            // Append '/' as a separator
            dst[offset] = b'/';
            offset += 1;

            // Format alt into a 4-byte sub-buffer
            offset += alt.fmt_to(const_slice(dst, offset)).len();
        }

        // Return string slice with current offset as length
        let slice = unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr(), offset) };
        return unsafe { str::from_utf8_unchecked_mut(slice) };

        const fn const_slice(dst: &mut [u8; 9], offset: usize) -> &mut [u8; 4] {
            unsafe { &mut *(dst.as_mut_ptr().add(offset).cast::<[u8; 4]>()) }
        }
    }
}

impl std::fmt::Display for AnyStress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_to(&mut [0; 4]).fmt(f)
    }
}
impl std::fmt::Display for AnyDualStress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_to(&mut [0; 9]).fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseStressError {
    Empty,
    InvalidPrime,
    InvalidFormat,
}

impl AnyStress {
    pub const fn from_str(text: &str) -> Result<Self, ParseStressError> {
        let text = text.as_bytes();

        // First, parse the letter
        let letter = match text.first() {
            None => return Err(ParseStressError::Empty),
            Some(b'a') => Self::A,
            Some(b'b') => Self::B,
            Some(b'c') => Self::C,
            Some(b'd') => Self::D,
            Some(b'e') => Self::E,
            Some(b'f') => Self::F,
            _ => return Err(ParseStressError::InvalidFormat),
        };

        match text {
            // Simple one-letter stress
            [_] => Ok(letter),

            // Single-prime: apostrophe, or UTF-8 char
            [_, b'\''] | [_, 0xE2, 0x80, 0xB2] => {
                // FIXME(const-hack): Replace with `.ok_or(Err(…))`.
                match letter.add_single_prime() {
                    Some(stress) => Ok(stress),
                    None => Err(ParseStressError::InvalidPrime),
                }
            },
            // Double-prime: quotation, two apostrophes, or UTF-8 char
            [_, b'"'] | [_, b'\'', b'\''] | [_, 0xE2, 0x80, 0xB3] => {
                // FIXME(const-hack): Replace with `.ok_or(Err(…))`.
                match letter.add_double_prime() {
                    Some(stress) => Ok(stress),
                    None => Err(ParseStressError::InvalidPrime),
                }
            },
            // All other formats are invalid
            _ => Err(ParseStressError::InvalidFormat),
        }
    }
}
impl AnyDualStress {
    pub const fn from_str(text: &str) -> Result<Self, ParseStressError> {
        if text.len() > 9 {
            return Err(ParseStressError::InvalidFormat);
        }

        // FIXME(const-hack): Replace with `text.split_once('/')`.
        if let Some(index) = find_slash(text) {
            let (main, alt) = text.split_at(index);

            return Ok(Self::new(
                // FIXME(const-hack): Replace with `?`.
                match AnyStress::from_str(main) {
                    Ok(x) => x,
                    Err(e) => return Err(e),
                },
                // FIXME(const-hack): Replace with `?`.
                match AnyStress::from_str(alt) {
                    Ok(x) => Some(x),
                    Err(e) => return Err(e),
                },
            ));
        } else {
            return Ok(Self::new(
                // FIXME(const-hack): Replace with `?`.
                match AnyStress::from_str(text) {
                    Ok(x) => x,
                    Err(e) => return Err(e),
                },
                None,
            ));
        }

        #[inline]
        const fn find_slash(text: &str) -> Option<usize> {
            let text = text.as_bytes();
            let mut i = 0;
            while i < text.len() {
                if text[i] == b'/' {
                    return Some(i);
                }
                i += 1;
            }
            None
        }
    }
}

impl std::str::FromStr for AnyStress {
    type Err = ParseStressError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl std::str::FromStr for AnyDualStress {
    type Err = ParseStressError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
