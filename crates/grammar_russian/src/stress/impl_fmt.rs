use crate::util::const_traits::*;

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
    Invalid,
    InvalidPrime,
    InvalidType,
}

impl AnyStress {
    pub const fn from_str(text: &str) -> Result<Self, ParseStressError> {
        match Self::from_str_partial(text) {
            Ok((stress, len)) => {
                // Return Ok only if the entire string was parsed
                if len as usize == text.len() { Ok(stress) } else { Err(ParseStressError::Invalid) }
            },
            Err(err) => Err(err),
        }
    }
    pub const fn from_str_partial(text: &str) -> Result<(Self, u8), ParseStressError> {
        let text = text.as_bytes();

        // First, parse the latin letter
        let letter = match text.first() {
            Some(b'a') => Self::A,
            Some(b'b') => Self::B,
            Some(b'c') => Self::C,
            Some(b'd') => Self::D,
            Some(b'e') => Self::E,
            Some(b'f') => Self::F,
            _ => return Err(ParseStressError::Invalid),
        };

        // Then parse prime indicators
        let (primes, parsed_len) = match text {
            [_, 0xE2, 0x80, 0xB2, ..] => (1, 4), // ′ (UTF-8 single prime)
            [_, 0xE2, 0x80, 0xB3, ..] => (2, 4), // ″ (UTF-8 double prime)
            [_, b'\'', b'\'', ..] => (2, 3),     // '' (double apostrophe)
            [_, b'\'', ..] => (1, 2),            // ' (apostrophe)
            [_, b'"', ..] => (2, 2),             // " (quotation)
            _ => (0u8, 1u8),                     // no primes
        };

        // Apply appropriate primes, and return
        Ok((
            match primes {
                0 => letter,
                // FIXME(const-hack): Replace with `.ok_or(Err(…))`.
                1 => match letter.add_single_prime() {
                    Some(stress) => stress,
                    None => return Err(ParseStressError::InvalidPrime),
                },
                // FIXME(const-hack): Replace with `.ok_or(Err(…))`.
                2 => match letter.add_double_prime() {
                    Some(stress) => stress,
                    None => return Err(ParseStressError::InvalidPrime),
                },
                _ => unreachable!(),
            },
            parsed_len,
        ))
    }
}
impl AnyDualStress {
    pub const fn from_str(text: &str) -> Result<Self, ParseStressError> {
        match Self::from_str_partial(text) {
            Ok((stress, len)) => {
                // Return Ok only if the entire string was parsed
                if len as usize == text.len() { Ok(stress) } else { Err(ParseStressError::Invalid) }
            },
            Err(err) => Err(err),
        }
    }
    pub const fn from_str_partial(text: &str) -> Result<(Self, u8), ParseStressError> {
        let (main, mut len);
        let mut alt = None;

        // Parse the main stress
        // FIXME(const-hack): Replace with `?`.
        (main, len) = const_try!(AnyStress::from_str_partial(text));

        // If followed by a '/', parse the alt stress
        // FIXME(const-hack): Replace with `.get()`.
        if matches!(_get(text, len as usize), Some(b'/')) {
            len += 1;
            let (_, text) = text.split_at(len as usize);

            // Parse the alt stress
            // FIXME(const-hack): Replace with `?`.
            let (stress, stress_len) = const_try!(AnyStress::from_str_partial(text));
            alt = Some(stress);
            len += stress_len;
        }

        // Construct the dual stress and return
        return Ok((AnyDualStress::new(main, alt), len));

        const fn _get(text: &str, pos: usize) -> Option<u8> {
            if pos < text.len() { Some(unsafe { *text.as_ptr().add(pos) }) } else { None }
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
    ($($t:ty),* $(,)?) => ($(
        impl std::str::FromStr for $t {
            type Err = ParseStressError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                AnyStress::from_str(s)?.try_into().or(Err(Self::Err::InvalidType))
            }
        }
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                // TODO: there should be a special constructor, that converts `a/a` to `a`
                AnyStress::from(*self).fmt(f)
            }
        }
    )*);
}
derive_stress_impls! {
    NounStress, PronounStress, AdjectiveFullStress, AdjectiveShortStress, VerbPresentStress, VerbPastStress,
}

impl std::str::FromStr for AdjectiveStress {
    type Err = ParseStressError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AnyDualStress::from_str(s)?.try_into().or(Err(Self::Err::InvalidType))
    }
}
impl std::str::FromStr for VerbStress {
    type Err = ParseStressError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AnyDualStress::from_str(s)?.try_into().or(Err(Self::Err::InvalidType))
    }
}
impl std::fmt::Display for AdjectiveStress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr().fmt(f)
    }
}
impl std::fmt::Display for VerbStress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.abbr().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::stress::*;

    #[test]
    fn fmt_any() {
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

        assert::<AdjectiveStress>(stress![a / a], "a");
        assert::<AdjectiveStress>(stress![b / b], "b");
        assert::<AdjectiveStress>(stress![a / a1], "a′");
        assert::<AdjectiveStress>(stress![b / b1], "b′");
        assert::<AdjectiveStress>(stress![b / a], "b/a");
        assert::<AdjectiveStress>(stress![a / c1], "a/c′");
        assert::<AdjectiveStress>(stress![b / c2], "b/c″");

        assert::<VerbStress>(stress![a / a], "a");
        assert::<VerbStress>(stress![b / a], "b");
        assert::<VerbStress>(stress![c / a], "c");
        assert::<VerbStress>(stress![a / c], "a/c");
        assert::<VerbStress>(stress![b / b], "b/b");
        assert::<VerbStress>(stress![c / c2], "c/c″");
        assert::<VerbStress>(stress![c1 / c], "c′/c");
    }

    #[test]
    fn parse_any() {
        type Error = ParseStressError;

        assert_eq!("a".parse::<AnyStress>(), Ok(stress![a]));
        assert_eq!("f".parse::<AnyStress>(), Ok(stress![f]));
        assert_eq!("e'".parse::<AnyStress>(), Ok(stress![e1]));
        assert_eq!("c\"".parse::<AnyStress>(), Ok(stress![c2]));
        assert_eq!("a′".parse::<AnyStress>(), Ok(stress![a1]));
        assert_eq!("c''".parse::<AnyStress>(), Ok(stress![c2]));
        assert_eq!("f″".parse::<AnyStress>(), Ok(stress![f2]));

        assert_eq!("".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("/".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("a/".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("/b".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("a/b".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("z".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("A".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("ab".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("$a".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("a$".parse::<AnyStress>(), Err(Error::Invalid));

        assert_eq!("a".parse::<AnyDualStress>(), Ok(stress![a]));
        assert_eq!("f".parse::<AnyDualStress>(), Ok(stress![f]));
        assert_eq!("e'".parse::<AnyDualStress>(), Ok(stress![e1]));
        assert_eq!("c\"".parse::<AnyDualStress>(), Ok(stress![c2]));
        assert_eq!("a/b".parse::<AnyDualStress>(), Ok(stress![a / b]));
        assert_eq!("d'/b′".parse::<AnyDualStress>(), Ok(stress![d1 / b1]));
        assert_eq!("e′/c\"".parse::<AnyDualStress>(), Ok(stress![e1 / c2]));
        assert_eq!("f″/e'".parse::<AnyDualStress>(), Ok(stress![f2 / e1]));
        assert_eq!("e′/c''".parse::<AnyDualStress>(), Ok(stress![e1 / c2]));

        assert_eq!("".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("/".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("a/".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("/b".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("z".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("A".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("ab".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("$a/b".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("a/b$".parse::<AnyStress>(), Err(Error::Invalid));
    }

    #[test]
    fn parse_typed() {
        type Error = ParseStressError;

        assert_eq!("a".parse::<NounStress>(), Ok(stress![a]));
        assert_eq!("f".parse::<NounStress>(), Ok(stress![f]));
        assert_eq!("a′".parse::<NounStress>(), Err(Error::InvalidType));
        assert_eq!("b′".parse::<NounStress>(), Ok(stress![b1]));
        assert_eq!("c″".parse::<NounStress>(), Err(Error::InvalidType));
        assert_eq!("f″".parse::<NounStress>(), Ok(stress![f2]));

        assert_eq!("a".parse::<PronounStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<PronounStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<PronounStress>(), Err(Error::InvalidType));
        assert_eq!("f".parse::<PronounStress>(), Ok(stress![f]));
        assert_eq!("a′".parse::<PronounStress>(), Err(Error::InvalidType));

        assert_eq!("a".parse::<AdjectiveFullStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<AdjectiveFullStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<AdjectiveFullStress>(), Err(Error::InvalidType));
        assert_eq!("a′".parse::<AdjectiveFullStress>(), Err(Error::InvalidType));

        assert_eq!("a".parse::<AdjectiveShortStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<AdjectiveShortStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<AdjectiveShortStress>(), Ok(stress![c]));
        assert_eq!("d".parse::<AdjectiveShortStress>(), Err(Error::InvalidType));
        assert_eq!("a′".parse::<AdjectiveShortStress>(), Ok(stress![a1]));
        assert_eq!("c′".parse::<AdjectiveShortStress>(), Ok(stress![c1]));
        assert_eq!("e′".parse::<AdjectiveShortStress>(), Err(Error::InvalidType));
        assert_eq!("c″".parse::<AdjectiveShortStress>(), Ok(stress![c2]));
        assert_eq!("f″".parse::<AdjectiveShortStress>(), Err(Error::InvalidType));

        assert_eq!("a".parse::<VerbPresentStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<VerbPresentStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<VerbPresentStress>(), Ok(stress![c]));
        assert_eq!("d".parse::<VerbPresentStress>(), Err(Error::InvalidType));
        assert_eq!("c′".parse::<VerbPresentStress>(), Ok(stress![c1]));
        assert_eq!("d′".parse::<VerbPresentStress>(), Err(Error::InvalidType));
        assert_eq!("f″".parse::<VerbPresentStress>(), Err(Error::InvalidType));

        assert_eq!("a".parse::<VerbPastStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<VerbPastStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<VerbPastStress>(), Ok(stress![c]));
        assert_eq!("d".parse::<VerbPastStress>(), Err(Error::InvalidType));
        assert_eq!("b′".parse::<VerbPastStress>(), Err(Error::InvalidType));
        assert_eq!("c′".parse::<VerbPastStress>(), Ok(stress![c1]));
        assert_eq!("d′".parse::<VerbPastStress>(), Err(Error::InvalidType));
        assert_eq!("c″".parse::<VerbPastStress>(), Ok(stress![c2]));
        assert_eq!("f″".parse::<VerbPastStress>(), Err(Error::InvalidType));
    }

    #[test]
    fn parse_dual() {
        type Error = ParseStressError;

        assert_eq!("a".parse::<AdjectiveStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<AdjectiveStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<AdjectiveStress>(), Err(Error::InvalidType));
        assert_eq!("a′".parse::<AdjectiveStress>(), Ok(stress![a1]));
        assert_eq!("b′".parse::<AdjectiveStress>(), Ok(stress![b1]));
        assert_eq!("c′".parse::<AdjectiveStress>(), Err(Error::InvalidType));
        assert_eq!("d′".parse::<AdjectiveStress>(), Err(Error::InvalidType));
        assert_eq!("f″".parse::<AdjectiveStress>(), Err(Error::InvalidType));

        assert_eq!("a/a".parse::<AdjectiveStress>(), Ok(stress![a / a]));
        assert_eq!("a/c".parse::<AdjectiveStress>(), Ok(stress![a / c]));
        assert_eq!("b/b".parse::<AdjectiveStress>(), Ok(stress![b / b]));
        assert_eq!("a/a′".parse::<AdjectiveStress>(), Ok(stress![a / a1]));
        assert_eq!("b/b′".parse::<AdjectiveStress>(), Ok(stress![b / b1]));
        assert_eq!("b/c′".parse::<AdjectiveStress>(), Ok(stress![b / c1]));
        assert_eq!("c/c′".parse::<AdjectiveStress>(), Err(Error::InvalidType));

        assert_eq!("a".parse::<VerbStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<VerbStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<VerbStress>(), Ok(stress![c]));
        assert_eq!("d".parse::<VerbStress>(), Err(Error::InvalidType));
        assert_eq!("a′".parse::<VerbStress>(), Err(Error::InvalidType));
        assert_eq!("b′".parse::<VerbStress>(), Err(Error::InvalidType));
        assert_eq!("c′".parse::<VerbStress>(), Ok(stress![c1]));
        assert_eq!("c″".parse::<VerbStress>(), Err(Error::InvalidType));
        assert_eq!("f″".parse::<VerbStress>(), Err(Error::InvalidType));

        assert_eq!("a/a".parse::<VerbStress>(), Ok(stress![a / a]));
        assert_eq!("b/a".parse::<VerbStress>(), Ok(stress![b / a]));
        assert_eq!("c/a".parse::<VerbStress>(), Ok(stress![c / a]));
        assert_eq!("b/b".parse::<VerbStress>(), Ok(stress![b / b]));
        assert_eq!("a/b".parse::<VerbStress>(), Ok(stress![a / b]));
        assert_eq!("c/c".parse::<VerbStress>(), Ok(stress![c / c]));
        assert_eq!("d/a".parse::<VerbStress>(), Err(Error::InvalidType));
        assert_eq!("a′/a".parse::<VerbStress>(), Err(Error::InvalidType));
        assert_eq!("b′/a".parse::<VerbStress>(), Err(Error::InvalidType));
        assert_eq!("c′/a".parse::<VerbStress>(), Ok(stress![c1 / a]));
        assert_eq!("c″/a".parse::<VerbStress>(), Err(Error::InvalidType));
        assert_eq!("f″/a".parse::<VerbStress>(), Err(Error::InvalidType));
    }
}
