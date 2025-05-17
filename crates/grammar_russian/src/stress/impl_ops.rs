use crate::categories::*;

use super::defs::*;

impl AnyStress {
    pub const fn has_any_primes(self) -> bool {
        !matches!(
            self,
            Self::A | Self::B | Self::C | Self::D | Self::E | Self::F
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

    pub const fn unprime(self) -> AnyStress {
        match self {
            Self::A | Self::Ap => Self::A,
            Self::B | Self::Bp => Self::B,
            Self::C | Self::Cp | Self::Cpp => Self::C,
            Self::D | Self::Dp => Self::D,
            Self::E | Self::Ep => Self::E,
            Self::F | Self::Fp | Self::Fpp => Self::F,
        }
    }

    pub const fn add_single_prime(self) -> Option<AnyStress> {
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
    pub const fn add_double_prime(self) -> Option<AnyStress> {
        Some(match self {
            Self::C => Self::Cpp,
            Self::F => Self::Fpp,
            _ => return None,
        })
    }
}

impl NounStress {
    pub const fn is_stem_stressed(
        self,
        info: impl ~const HasCase + ~const HasNumber + Copy,
        animacy: impl ~const HasAnimacy + Copy,
    ) -> bool {
        // Note: `is_nom_with` is called only when number is plural, that is, when the
        // accusative case maps to either nominative or genitive depending on animacy.

        match self {
            Self::A => true,
            Self::B => false,
            Self::C => info.is_singular(),
            Self::D => info.is_plural(),
            Self::E => info.is_singular() || info.is_nom_or_acc_inan(animacy),
            Self::F => info.is_plural() && info.is_nom_or_acc_inan(animacy),
            Self::Bp => info.is_singular() && matches!(info.case(), Case::Instrumental),
            Self::Dp => info.is_plural() || matches!(info.case(), Case::Accusative),
            Self::Fp => match info.number() {
                Number::Singular => matches!(info.case(), Case::Accusative),
                Number::Plural => info.is_nom_or_acc_inan(animacy),
            },
            Self::Fpp => match info.number() {
                Number::Singular => matches!(info.case(), Case::Instrumental),
                Number::Plural => info.is_nom_or_acc_inan(animacy),
            },
        }
    }
    pub const fn is_ending_stressed(
        self,
        info: impl ~const HasCase + ~const HasNumber + Copy,
        animacy: impl ~const HasAnimacy + Copy,
    ) -> bool {
        !self.is_stem_stressed(info, animacy)
    }
}

impl AdjectiveFullStress {
    pub const fn is_stem_stressed(self) -> bool {
        matches!(self, Self::A)
    }
    pub const fn is_ending_stressed(self) -> bool {
        !self.is_stem_stressed()
    }
}
impl AdjectiveShortStress {
    pub const fn is_stem_stressed(self, info: GenderOrPlural) -> bool {
        use {Gender as G, GenderOrPlural as GP};

        match self {
            Self::A => true,
            Self::B => false,
            Self::C => matches!(info, GP::Singular(G::Masculine | G::Neuter) | GP::Plural),
            Self::Ap => true,
            Self::Bp => matches!(info, GP::Singular(G::Masculine)),
            Self::Cp => matches!(info, GP::Singular(G::Masculine | G::Neuter)),
            Self::Cpp => matches!(info, GP::Singular(G::Masculine | G::Neuter)),
        }
    }
    pub const fn is_ending_stressed(self, info: GenderOrPlural) -> bool {
        !self.is_stem_stressed(info)
    }
}

impl PronounStress {
    pub const fn is_stem_stressed(
        self,
        info: impl ~const HasCase + ~const HasNumber + Copy,
        animacy: impl ~const HasAnimacy + Copy,
    ) -> bool {
        match self {
            PronounStress::A => true,
            PronounStress::B => false,
            PronounStress::F => info.is_plural() && info.is_nom_or_acc_inan(animacy),
        }
    }
    pub const fn is_ending_stressed(
        self,
        info: impl ~const HasCase + ~const HasNumber + Copy,
        animacy: impl ~const HasAnimacy + Copy,
    ) -> bool {
        !self.is_stem_stressed(info, animacy)
    }
}

// TODO: VerbStress methods
