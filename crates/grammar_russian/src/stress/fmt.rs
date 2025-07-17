use crate::{
    stress::{
        AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress,
        NounStress, PronounStress, VerbPastStress, VerbPresentStress, VerbStress,
    },
    util::UnsafeBuf,
};

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

macro_rules! derive_stress_impls {
    ($($t:ty),* $(,)?) => ($(
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                AnyStress::from(*self).fmt(f)
            }
        }
    )*);
}
derive_stress_impls! {
    NounStress, PronounStress, AdjectiveFullStress, AdjectiveShortStress, VerbPresentStress, VerbPastStress,
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
    use super::*;
    use crate::stress;

    #[test]
    fn fmt_any() {
        fn assert_fmt<T: std::fmt::Display>(value: T, expected: &str) {
            assert_eq!(value.to_string(), expected);
        }

        assert_fmt(AnyStress::A, "a");
        assert_fmt(AnyStress::B, "b");
        assert_fmt(AnyStress::C, "c");
        assert_fmt(AnyStress::D, "d");
        assert_fmt(AnyStress::E, "e");
        assert_fmt(AnyStress::F, "f");
        assert_fmt(AnyStress::Ap, "a′");
        assert_fmt(AnyStress::Bp, "b′");
        assert_fmt(AnyStress::Cp, "c′");
        assert_fmt(AnyStress::Dp, "d′");
        assert_fmt(AnyStress::Ep, "e′");
        assert_fmt(AnyStress::Fp, "f′");
        assert_fmt(AnyStress::Cpp, "c″");
        assert_fmt(AnyStress::Fpp, "f″");

        assert_fmt::<AnyDualStress>(stress![a], "a");
        assert_fmt::<AnyDualStress>(stress![f], "f");
        assert_fmt::<AnyDualStress>(stress![b1], "b′");
        assert_fmt::<AnyDualStress>(stress![e1], "e′");
        assert_fmt::<AnyDualStress>(stress![c2], "c″");
        assert_fmt::<AnyDualStress>(stress![f2], "f″");
        assert_fmt::<AnyDualStress>(stress![a / a], "a/a");
        assert_fmt::<AnyDualStress>(stress![a / f1], "a/f′");
        assert_fmt::<AnyDualStress>(stress![c1 / e], "c′/e");
        assert_fmt::<AnyDualStress>(stress![f2 / c2], "f″/c″");

        assert_fmt::<AdjectiveStress>(stress![a / a], "a");
        assert_fmt::<AdjectiveStress>(stress![b / b], "b");
        assert_fmt::<AdjectiveStress>(stress![a / a1], "a′");
        assert_fmt::<AdjectiveStress>(stress![b / b1], "b′");
        assert_fmt::<AdjectiveStress>(stress![b / a], "b/a");
        assert_fmt::<AdjectiveStress>(stress![a / c1], "a/c′");
        assert_fmt::<AdjectiveStress>(stress![b / c2], "b/c″");

        assert_fmt::<VerbStress>(stress![a / a], "a");
        assert_fmt::<VerbStress>(stress![b / a], "b");
        assert_fmt::<VerbStress>(stress![c / a], "c");
        assert_fmt::<VerbStress>(stress![a / c], "a/c");
        assert_fmt::<VerbStress>(stress![b / b], "b/b");
        assert_fmt::<VerbStress>(stress![c / c2], "c/c″");
        assert_fmt::<VerbStress>(stress![c1 / c], "c′/c");
    }
}
