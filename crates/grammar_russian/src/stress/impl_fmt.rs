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
    InvalidType,
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
        if let Some((main, alt)) = split_by_slash(text) {
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
        const fn split_by_slash(text: &str) -> Option<(&str, &str)> {
            let bytes = text.as_bytes();
            // Slash can't be the first or last char, so we can just search the range 1..(N-1).
            // Also, it doesn't matter if it splits a character in half, since AnyStress parses
            // the string by bytes, accepting only valid UTF-8 anyway.
            let mut i = 1;
            while i < bytes.len() - 1 {
                if bytes[i] == b'/' {
                    return Some(split_by_unchecked(text, i));
                }
                i += 1;
            }
            None
        }
        #[inline]
        const fn split_by_unchecked(text: &str, i: usize) -> (&str, &str) {
            use std::slice;
            let len = text.len();
            let ptr = text.as_ptr();
            unsafe {
                (
                    str::from_utf8_unchecked(slice::from_raw_parts(ptr, i)),
                    // Unlike str::split_at_unchecked, the right part starts 1 byte later (after /)
                    str::from_utf8_unchecked(slice::from_raw_parts(ptr.add(i + 1), len - (i + 1))),
                )
            }
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

macro_rules! derive_stress_impls {
    ($($src:ty: [$($t:ty),*]),*) => ($($(
        impl std::str::FromStr for $t {
            type Err = <$src as std::str::FromStr>::Err;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                AnyStress::from_str(s)?.try_into().or(Err(Self::Err::InvalidType))
            }
        }
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                // TODO: there should be a special constructor, that converts `a/a` to `a`
                <$src>::from(*self).fmt(f)
            }
        }
    )*)*);
}

derive_stress_impls! {
    AnyStress: [NounStress, PronounStress, AdjectiveFullStress, AdjectiveShortStress, VerbPresentStress, VerbPastStress],
    AnyDualStress: [AdjectiveStress, VerbStress]
}

#[cfg(test)]
mod tests {
    use crate::stress::*;

    #[test]
    fn fmt() {
        fn assert<T: std::fmt::Display>(value: T, expected: &str) {
            assert_eq!(value.to_string(), expected);
        }

        assert(AnyStress::A, "a");
        assert(AnyStress::B, "b");
        assert(AnyStress::C, "c");
        assert(AnyStress::D, "d");
        assert(AnyStress::E, "e");
        assert(AnyStress::F, "f");
        assert(AnyStress::Ap, "a′");
        assert(AnyStress::Bp, "b′");
        assert(AnyStress::Cp, "c′");
        assert(AnyStress::Dp, "d′");
        assert(AnyStress::Ep, "e′");
        assert(AnyStress::Fp, "f′");
        assert(AnyStress::Cpp, "c″");
        assert(AnyStress::Fpp, "f″");

        assert::<AnyDualStress>(stress![a], "a");
        assert::<AnyDualStress>(stress![f], "f");
        assert::<AnyDualStress>(stress![b1], "b′");
        assert::<AnyDualStress>(stress![e1], "e′");
        assert::<AnyDualStress>(stress![c2], "c″");
        assert::<AnyDualStress>(stress![f2], "f″");
        assert::<AnyDualStress>(stress![a / a], "a/a");
        assert::<AnyDualStress>(stress![a / f1], "a/f′");
        assert::<AnyDualStress>(stress![c1 / e], "c′/e");
        assert::<AnyDualStress>(stress![f2 / c2], "f″/c″");
    }

    #[test]
    fn parse() {
        type Error = ParseStressError;

        assert_eq!("a".parse::<AnyStress>(), Ok(stress![a]));
        assert_eq!("f".parse::<AnyStress>(), Ok(stress![f]));
        assert_eq!("e'".parse::<AnyStress>(), Ok(stress![e1]));
        assert_eq!("c\"".parse::<AnyStress>(), Ok(stress![c2]));

        assert_eq!("".parse::<AnyStress>(), Err(Error::Empty));
        assert_eq!("/".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("a/".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("/b".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("a/b".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("z".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("$a".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("a$".parse::<AnyStress>(), Err(Error::InvalidFormat));

        assert_eq!("a".parse::<AnyDualStress>(), Ok(stress![a]));
        assert_eq!("f".parse::<AnyDualStress>(), Ok(stress![f]));
        assert_eq!("e'".parse::<AnyDualStress>(), Ok(stress![e1]));
        assert_eq!("c\"".parse::<AnyDualStress>(), Ok(stress![c2]));
        assert_eq!("a/b".parse::<AnyDualStress>(), Ok(stress![a / b]));
        assert_eq!("d'/b'".parse::<AnyDualStress>(), Ok(stress![d1 / b1]));
        assert_eq!("e'/c\"".parse::<AnyDualStress>(), Ok(stress![e1 / c2]));

        assert_eq!("".parse::<AnyStress>(), Err(Error::Empty));
        assert_eq!("/".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("a/".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("/b".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("z".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("$a/b".parse::<AnyStress>(), Err(Error::InvalidFormat));
        assert_eq!("a/b$".parse::<AnyStress>(), Err(Error::InvalidFormat));
    }
}
