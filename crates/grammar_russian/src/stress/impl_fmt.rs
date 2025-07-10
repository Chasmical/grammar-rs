use super::defs::*;
use crate::util::*;

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
            let ch = if self.has_double_prime() { '″' } else { '′' };
            ch.encode_utf8(dst.last_chunk_mut::<3>().unwrap());

            // Return string slice from the entire buffer
            unsafe { str::from_utf8_unchecked_mut(dst) }
        } else {
            // Return string slice of length 1, containing only the letter
            let slice = unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr(), 1) };
            unsafe { str::from_utf8_unchecked_mut(slice) }
        }
    }
}
impl AnyDualStress {
    pub const fn fmt_to(self, dst: &mut [u8; 9]) -> &mut str {
        let mut dst = UnsafeBuf::new(dst);

        // Format main into a 4-byte sub-buffer
        let main_len = self.main.fmt_to(dst.chunk()).len();
        dst.forward(main_len);

        if let Some(alt) = self.alt {
            // Append '/' as a separator
            dst.push('/');

            // Format alt into a 4-byte sub-buffer
            let alt_len = alt.fmt_to(dst.chunk()).len();
            dst.forward(alt_len);
        }

        dst.finish()
    }
}

impl std::fmt::Display for AnyStress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; 4]).fmt(f)
    }
}
impl std::fmt::Display for AnyDualStress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; 9]).fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseStressError {
    Invalid,
    InvalidPrime,
    InvalidType,
}

impl const PartialParse for AnyStress {
    type Err = ParseStressError;

    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        // First, parse the latin letter
        let letter = match parser.read_one() {
            Some(b'a') => Self::A,
            Some(b'b') => Self::B,
            Some(b'c') => Self::C,
            Some(b'd') => Self::D,
            Some(b'e') => Self::E,
            Some(b'f') => Self::F,
            _ => return Err(ParseStressError::Invalid),
        };

        // Then parse prime indicators
        let (primes, primes_len) = match parser.remaining() {
            [0xE2, 0x80, 0xB2, ..] => (1, 3), // ′ (UTF-8 single prime)
            [0xE2, 0x80, 0xB3, ..] => (2, 3), // ″ (UTF-8 double prime)
            [b'\'', b'\'', ..] => (2, 2),     // '' (double apostrophe)
            [b'\'', ..] => (1, 1),            // ' (apostrophe)
            [b'"', ..] => (2, 1),             // " (quotation)
            _ => (0u8, 0u8),                  // no primes
        };
        parser.forward(primes_len as usize);

        Ok(match primes {
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
        })
    }
}
impl const PartialParse for AnyDualStress {
    type Err = ParseStressError;

    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        // Parse the main stress
        let main = const_try!(AnyStress::partial_parse(parser));
        let mut alt = None;

        // If followed by a '/', parse the alt stress
        if let Some(b'/') = parser.peek_one() {
            alt = Some(const_try!(AnyStress::partial_parse(parser)));
        }

        // Construct the dual stress and return
        Ok(AnyDualStress::new(main, alt))
    }
}

impl std::str::FromStr for AnyStress {
    type Err = ParseStressError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or(s, Self::Err::Invalid)
    }
}
impl std::str::FromStr for AnyDualStress {
    type Err = ParseStressError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or(s, Self::Err::Invalid)
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
