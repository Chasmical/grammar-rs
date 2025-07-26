use crate::{
    stress::{
        AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress,
        NounStress, PronounStress, VerbPastStress, VerbPresentStress, VerbStress,
    },
    util::{PartialParse, UnsafeParser, const_traits::*},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseStressError {
    InvalidLetter,
    InvalidPrime,
    Incompatible,
    Invalid,
}

impl const PartialParse for AnyStress {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        // First, parse the latin letter
        let letter = match parser.read_one() {
            Some(b'a') => Self::A,
            Some(b'b') => Self::B,
            Some(b'c') => Self::C,
            Some(b'd') => Self::D,
            Some(b'e') => Self::E,
            Some(b'f') => Self::F,
            _ => return Err(ParseStressError::InvalidLetter),
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
            1 => const_try!(letter.add_single_prime(), ParseStressError::InvalidPrime {}),
            2 => const_try!(letter.add_double_prime(), ParseStressError::InvalidPrime {}),
            _ => unreachable!(),
        })
    }
}
impl const PartialParse for AnyDualStress {
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        // Parse the main stress
        let main = AnyStress::partial_parse(parser)?;
        let mut alt = None;

        // If followed by a '/', parse the alt stress
        if parser.skip('/') {
            alt = Some(AnyStress::partial_parse(parser)?);
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
                AnyStress::from_str(s)?.try_into().or(Err(Self::Err::Incompatible))
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
        AnyDualStress::from_str(s)?.try_into().or(Err(Self::Err::Incompatible))
    }
}
impl std::str::FromStr for VerbStress {
    type Err = ParseStressError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AnyDualStress::from_str(s)?.try_into().or(Err(Self::Err::Incompatible))
    }
}

#[cfg(test)]
mod tests {
    use super::{ParseStressError as Error, *};
    use crate::stress;

    #[test]
    fn parse_any() {
        assert_eq!("a".parse::<AnyStress>(), Ok(stress![a]));
        assert_eq!("f".parse::<AnyStress>(), Ok(stress![f]));
        assert_eq!("e'".parse::<AnyStress>(), Ok(stress![e1]));
        assert_eq!("c\"".parse::<AnyStress>(), Ok(stress![c2]));
        assert_eq!("a′".parse::<AnyStress>(), Ok(stress![a1]));
        assert_eq!("c''".parse::<AnyStress>(), Ok(stress![c2]));
        assert_eq!("f″".parse::<AnyStress>(), Ok(stress![f2]));

        assert_eq!("".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("/".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("a/".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("/b".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("a/b".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("z".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("A".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("ab".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("$a".parse::<AnyStress>(), Err(Error::InvalidLetter));
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

        assert_eq!("".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("/".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("a/".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("/b".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("z".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("A".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("ab".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("$a/b".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("a/b$".parse::<AnyStress>(), Err(Error::Invalid));
    }

    #[test]
    fn parse_typed() {
        assert_eq!("a".parse::<NounStress>(), Ok(stress![a]));
        assert_eq!("f".parse::<NounStress>(), Ok(stress![f]));
        assert_eq!("a′".parse::<NounStress>(), Err(Error::Incompatible));
        assert_eq!("b′".parse::<NounStress>(), Ok(stress![b1]));
        assert_eq!("c″".parse::<NounStress>(), Err(Error::Incompatible));
        assert_eq!("f″".parse::<NounStress>(), Ok(stress![f2]));

        assert_eq!("a".parse::<PronounStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<PronounStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<PronounStress>(), Err(Error::Incompatible));
        assert_eq!("f".parse::<PronounStress>(), Ok(stress![f]));
        assert_eq!("a′".parse::<PronounStress>(), Err(Error::Incompatible));

        assert_eq!("a".parse::<AdjectiveFullStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<AdjectiveFullStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<AdjectiveFullStress>(), Err(Error::Incompatible));
        assert_eq!("a′".parse::<AdjectiveFullStress>(), Err(Error::Incompatible));

        assert_eq!("a".parse::<AdjectiveShortStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<AdjectiveShortStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<AdjectiveShortStress>(), Ok(stress![c]));
        assert_eq!("d".parse::<AdjectiveShortStress>(), Err(Error::Incompatible));
        assert_eq!("a′".parse::<AdjectiveShortStress>(), Ok(stress![a1]));
        assert_eq!("c′".parse::<AdjectiveShortStress>(), Ok(stress![c1]));
        assert_eq!("e′".parse::<AdjectiveShortStress>(), Err(Error::Incompatible));
        assert_eq!("c″".parse::<AdjectiveShortStress>(), Ok(stress![c2]));
        assert_eq!("f″".parse::<AdjectiveShortStress>(), Err(Error::Incompatible));

        assert_eq!("a".parse::<VerbPresentStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<VerbPresentStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<VerbPresentStress>(), Ok(stress![c]));
        assert_eq!("d".parse::<VerbPresentStress>(), Err(Error::Incompatible));
        assert_eq!("c′".parse::<VerbPresentStress>(), Ok(stress![c1]));
        assert_eq!("d′".parse::<VerbPresentStress>(), Err(Error::Incompatible));
        assert_eq!("f″".parse::<VerbPresentStress>(), Err(Error::Incompatible));

        assert_eq!("a".parse::<VerbPastStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<VerbPastStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<VerbPastStress>(), Ok(stress![c]));
        assert_eq!("d".parse::<VerbPastStress>(), Err(Error::Incompatible));
        assert_eq!("b′".parse::<VerbPastStress>(), Err(Error::Incompatible));
        assert_eq!("c′".parse::<VerbPastStress>(), Ok(stress![c1]));
        assert_eq!("d′".parse::<VerbPastStress>(), Err(Error::Incompatible));
        assert_eq!("c″".parse::<VerbPastStress>(), Ok(stress![c2]));
        assert_eq!("f″".parse::<VerbPastStress>(), Err(Error::Incompatible));
    }

    #[test]
    fn parse_dual() {
        assert_eq!("a".parse::<AdjectiveStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<AdjectiveStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<AdjectiveStress>(), Err(Error::Incompatible));
        assert_eq!("a′".parse::<AdjectiveStress>(), Ok(stress![a1]));
        assert_eq!("b′".parse::<AdjectiveStress>(), Ok(stress![b1]));
        assert_eq!("c′".parse::<AdjectiveStress>(), Err(Error::Incompatible));
        assert_eq!("d′".parse::<AdjectiveStress>(), Err(Error::Incompatible));
        assert_eq!("f″".parse::<AdjectiveStress>(), Err(Error::Incompatible));

        assert_eq!("a/a".parse::<AdjectiveStress>(), Ok(stress![a / a]));
        assert_eq!("a/c".parse::<AdjectiveStress>(), Ok(stress![a / c]));
        assert_eq!("b/b".parse::<AdjectiveStress>(), Ok(stress![b / b]));
        assert_eq!("a/a′".parse::<AdjectiveStress>(), Ok(stress![a / a1]));
        assert_eq!("b/b′".parse::<AdjectiveStress>(), Ok(stress![b / b1]));
        assert_eq!("b/c′".parse::<AdjectiveStress>(), Ok(stress![b / c1]));
        assert_eq!("c/c′".parse::<AdjectiveStress>(), Err(Error::Incompatible));

        assert_eq!("a".parse::<VerbStress>(), Ok(stress![a]));
        assert_eq!("b".parse::<VerbStress>(), Ok(stress![b]));
        assert_eq!("c".parse::<VerbStress>(), Ok(stress![c]));
        assert_eq!("d".parse::<VerbStress>(), Err(Error::Incompatible));
        assert_eq!("a′".parse::<VerbStress>(), Err(Error::Incompatible));
        assert_eq!("b′".parse::<VerbStress>(), Err(Error::Incompatible));
        assert_eq!("c′".parse::<VerbStress>(), Ok(stress![c1]));
        assert_eq!("c″".parse::<VerbStress>(), Err(Error::Incompatible));
        assert_eq!("f″".parse::<VerbStress>(), Err(Error::Incompatible));

        assert_eq!("a/a".parse::<VerbStress>(), Ok(stress![a / a]));
        assert_eq!("b/a".parse::<VerbStress>(), Ok(stress![b / a]));
        assert_eq!("c/a".parse::<VerbStress>(), Ok(stress![c / a]));
        assert_eq!("b/b".parse::<VerbStress>(), Ok(stress![b / b]));
        assert_eq!("a/b".parse::<VerbStress>(), Ok(stress![a / b]));
        assert_eq!("c/c".parse::<VerbStress>(), Ok(stress![c / c]));
        assert_eq!("d/a".parse::<VerbStress>(), Err(Error::Incompatible));
        assert_eq!("a′/a".parse::<VerbStress>(), Err(Error::Incompatible));
        assert_eq!("b′/a".parse::<VerbStress>(), Err(Error::Incompatible));
        assert_eq!("c′/a".parse::<VerbStress>(), Ok(stress![c1 / a]));
        assert_eq!("c″/a".parse::<VerbStress>(), Err(Error::Incompatible));
        assert_eq!("f″/a".parse::<VerbStress>(), Err(Error::Incompatible));
    }
}
