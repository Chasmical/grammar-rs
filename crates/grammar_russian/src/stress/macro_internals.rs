use crate::stress::{
    AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress,
    NounStress, PronounStress, VerbPastStress, VerbPresentStress, VerbStress,
};

#[allow(non_camel_case_types)]
pub mod aliases {
    macro_rules! define_empty_structs {
        ($($name:ident),*) => ($( pub struct $name; )*);
    }
    define_empty_structs! { Unset, a, b, c, d, e, f, a1, b1, c1, d1, e1, f1, c2, f2 }
}

pub const trait StressConst<T> {
    const STRESS: T;
}

macro_rules! define_aliases {
    ($stress:ty: $($name:ident $alias:ident),*) => ($(
        impl const StressConst<$stress> for aliases::$alias {
            const STRESS: $stress = <$stress>::$name;
        }
    )*);
}

define_aliases!(AnyStress: A a, B b, C c, D d, E e, F f, Ap a1, Bp b1, Cp c1, Dp d1, Ep e1, Fp f1, Cpp c2, Fpp f2);
define_aliases!(NounStress: A a, B b, C c, D d, E e, F f, Bp b1, Dp d1, Fp f1, Fpp f2);
define_aliases!(PronounStress: A a, B b, F f);
define_aliases!(AdjectiveFullStress: A a, B b);
define_aliases!(AdjectiveShortStress: A a, B b, C c, Ap a1, Bp b1, Cp c1, Cpp c2);
define_aliases!(VerbPresentStress: A a, B b, C c, Cp c1);
define_aliases!(VerbPastStress: A a, B b, C c, Cp c1, Cpp c2);

pub struct Builder<Main, Alt>(Main, Alt);

pub const trait Build<T> {
    const RESULT: T;
}

macro_rules! build_fn {
    ($($main:ty,)*) => ($(
        impl<MAIN: StressConst<$main>> const Build<$main> for Builder<MAIN, aliases::Unset> {
            const RESULT: $main = MAIN::STRESS;
        }
    )*);
    (($main:ty) $res:ty, $expr:expr) => (
        impl<MAIN: StressConst<$main>> const Build<$res> for Builder<MAIN, aliases::Unset> {
            const RESULT: $res = $expr;
        }
    );
    (($main:ty, $alt:ty) $res:ty, $expr:expr) => (
        impl<MAIN: StressConst<$main>, ALT: StressConst<$alt>> const Build<$res> for Builder<MAIN, ALT> {
            const RESULT: $res = $expr;
        }
    );
}

build_fn!(
    AnyStress,
    NounStress,
    PronounStress,
    AdjectiveFullStress,
    AdjectiveShortStress,
    VerbPresentStress,
    VerbPastStress,
);

build_fn!(
    (AdjectiveFullStress, AdjectiveShortStress) AdjectiveStress,
    AdjectiveStress::new(MAIN::STRESS, ALT::STRESS)
);
build_fn!(
    (VerbPresentStress, VerbPastStress) VerbStress,
    VerbStress::new(MAIN::STRESS, ALT::STRESS)
);
build_fn!(
    (VerbPresentStress) VerbStress,
    VerbStress::new(MAIN::STRESS, VerbPastStress::A)
);
build_fn!(
    (AnyStress) AnyDualStress,
    AnyDualStress::new(MAIN::STRESS, None)
);
build_fn!(
    (AnyStress, AnyStress) AnyDualStress,
    AnyDualStress::new(MAIN::STRESS, Some(ALT::STRESS))
);

pub trait IsStressAOrB {}
impl IsStressAOrB for aliases::a {}
impl IsStressAOrB for aliases::b {}
impl IsStressAOrB for aliases::a1 {}
impl IsStressAOrB for aliases::b1 {}

impl<X: StressConst<AdjectiveShortStress> + IsStressAOrB> const Build<AdjectiveStress>
    for Builder<X, aliases::Unset>
{
    const RESULT: AdjectiveStress = AdjectiveStress::new(
        match X::STRESS {
            AdjectiveShortStress::A | AdjectiveShortStress::Ap => AdjectiveFullStress::A,
            _ => AdjectiveFullStress::B,
        },
        X::STRESS,
    );
}

/// Provides a quick and easy way of initializing stress values.
///
/// Due to macros not supporting standalone `'`/`"` characters, single and double primes are
/// replaced by `1` (single prime) and `2` (double prime). So, instead of `a'` you'd write `a1`,
/// and instead of `f"` - `f2`.
///
/// # Examples
/// ```
/// # use grammar_russian::{stress, stress::*};
/// #
/// let x: NounStress = stress![f];
/// assert_eq!(x, NounStress::F);
///
/// let x: AdjectiveStress = stress![a / b];
/// assert_eq!(x, AdjectiveStress::A_B);
///
/// let x: VerbStress = stress![c1];
/// assert_eq!(x, VerbStress::Cp_A);
/// ```
///
/// When providing values to functions, variable's type can be omitted:
/// ```
/// # use grammar_russian::{stress, stress::*};
/// #
/// fn inflect_noun(word: &str, stress: NounStress) {}
/// fn inflect_adj(word: &str, stress: AdjectiveStress) {}
///
/// inflect_noun("word", stress![e]);
/// inflect_adj("word", stress![b / c2]);
/// ```
///
/// Invalid stress values cannot be constructed, and give a compilation error:
/// ```compile_fail
/// # use grammar_russian::{stress, stress::*};
/// #
/// let x: NounStress = stress![a1];
/// let x: NounStress = stress![b/b];
/// let x: AdjectiveStress = stress![c];
/// ```
#[macro_export]
macro_rules! stress {
    ($x:ident) => {{
        use $crate::stress::macro_internals::{Build, Builder, aliases};
        Builder::<aliases::$x, aliases::Unset>::RESULT
    }};
    ($x:ident/$y:ident) => {{
        use $crate::stress::macro_internals::{Build, Builder, aliases};
        Builder::<aliases::$x, aliases::$y>::RESULT
    }};
}
pub use stress;

#[cfg(test)]
mod tests {
    use crate::stress::*;

    fn assert_eq<T: std::fmt::Debug + PartialEq>(left: T, right: T) {
        assert_eq!(left, right);
    }
    #[allow(unused)]
    fn assert_invalid<T: TryFrom<AnyStress>>(values: &[T]) {
        panic!();
    }

    #[test]
    fn expand_noun() {
        type Stress = NounStress;

        assert_eq(stress![a], Stress::A);
        assert_eq(stress![b], Stress::B);
        assert_eq(stress![c], Stress::C);
        assert_eq(stress![d], Stress::D);
        assert_eq(stress![e], Stress::E);
        assert_eq(stress![f], Stress::F);
        assert_eq(stress![b1], Stress::Bp);
        assert_eq(stress![d1], Stress::Dp);
        assert_eq(stress![f1], Stress::Fp);
        assert_eq(stress![f2], Stress::Fpp);

        // assert_invalid::<Stress>(&[
        //     stress![a1],
        //     stress![c1],
        //     stress![e1],
        //     stress![c2],
        //     stress![a / b],
        //     stress![f1 / c2],
        // ]);
    }
    #[test]
    fn expand_pro() {
        type Stress = PronounStress;

        assert_eq(stress![a], Stress::A);
        assert_eq(stress![b], PronounStress::B);
        assert_eq(stress![f], PronounStress::F);

        // assert_invalid::<Stress>(&[
        //     stress![c],
        //     stress![a1],
        //     stress![c2],
        //     stress![a / b],
        //     stress![f1 / c2],
        // ]);
    }
    #[test]
    fn expand_adj_full() {
        type Stress = AdjectiveFullStress;

        assert_eq(stress![a], Stress::A);
        assert_eq(stress![b], Stress::B);

        // assert_invalid::<Stress>(&[
        //     stress![c],
        //     stress![a1],
        //     stress![c2],
        //     stress![a / b],
        //     stress![f1 / c2],
        // ]);
    }
    #[test]
    fn expand_adj_short() {
        type Stress = AdjectiveShortStress;

        assert_eq(stress![a], Stress::A);
        assert_eq(stress![b], Stress::B);
        assert_eq(stress![c], Stress::C);
        assert_eq(stress![a1], Stress::Ap);
        assert_eq(stress![b1], Stress::Bp);
        assert_eq(stress![c1], Stress::Cp);
        assert_eq(stress![c2], Stress::Cpp);

        // assert_invalid::<Stress>(&[
        //     stress![d],
        //     stress![d1],
        //     stress![f2],
        //     stress![a / b],
        //     stress![f1 / c2],
        // ]);
    }
    #[test]
    fn expand_adj_dual() {
        type Stress = AdjectiveStress;

        assert_eq(stress![a], Stress::A);
        assert_eq(stress![b], Stress::B);
        assert_eq(stress![a / b], Stress::A_B);
        assert_eq(stress![b / c], Stress::B_C);
        assert_eq(stress![a1], Stress::Ap);
        assert_eq(stress![b1], Stress::Bp);
        assert_eq(stress![a / a1], Stress::Ap);
        assert_eq(stress![b / b1], Stress::Bp);
        assert_eq(stress![b / c1], Stress::B_Cp);
        assert_eq(stress![a / c2], Stress::A_Cpp);

        // assert_invalid::<Stress>(&[
        //     stress![c],
        //     stress![c1],
        //     stress![c2],
        //     stress![c / b],
        //     stress![f1 / c2],
        // ]);
    }
    #[test]
    fn expand_verb_present() {
        type Stress = VerbPresentStress;

        assert_eq(stress![a], Stress::A);
        assert_eq(stress![b], Stress::B);
        assert_eq(stress![c], Stress::C);
        assert_eq(stress![c1], Stress::Cp);

        // assert_invalid::<Stress>(&[
        //     stress![d],
        //     stress![a1],
        //     stress![c2],
        //     stress![a / b],
        //     stress![f1 / c2],
        // ]);
    }
    #[test]
    fn expand_verb_past() {
        type Stress = VerbPastStress;

        assert_eq(stress![a], Stress::A);
        assert_eq(stress![b], Stress::B);
        assert_eq(stress![c], Stress::C);
        assert_eq(stress![c1], Stress::Cp);
        assert_eq(stress![c2], Stress::Cpp);

        // assert_invalid::<Stress>(&[
        //     stress![d],
        //     stress![a1],
        //     stress![f2],
        //     stress![a / b],
        //     stress![f1 / c2],
        // ]);
    }
    #[test]
    fn expand_verb_dual() {
        type Stress = VerbStress;

        assert_eq(stress![a], Stress::A);
        assert_eq(stress![b], Stress::B);
        assert_eq(stress![c], Stress::C);
        assert_eq(stress![c1], Stress::Cp);
        assert_eq(stress![c / b], Stress::C_B);
        assert_eq(stress![a / c1], Stress::A_Cp);
        assert_eq(stress![c1 / c2], Stress::Cp_Cpp);

        // assert_invalid::<Stress>(&[
        //     stress![d],
        //     stress![a1],
        //     stress![f2],
        //     stress![d / b],
        //     stress![f1 / c2],
        // ]);
    }
}
