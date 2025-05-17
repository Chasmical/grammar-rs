use super::types::*;

#[allow(non_camel_case_types)]
pub mod stress_names {
    macro_rules! define_empty_structs {
        ($($name:ident),*) => ($( pub struct $name; )*);
    }
    define_empty_structs! { Unset, a, b, c, d, e, f, a1, b1, c1, d1, e1, f1, c2, f2 }
}

#[const_trait]
pub trait StressConst<T> {
    const STRESS: T;
}

macro_rules! define_aliases {
    ($stress:ty: $($name:ident $alias:ident),*) => ($(
        impl const $crate::stress::macro_internals::StressConst<$stress> for stress_names::$alias {
            const STRESS: $stress = <$stress>::$name;
        }
    )*);
}

define_aliases!(NounStress: A a, B b, C c, D d, E e, F f, Bp b1, Dp d1, Fp f1, Fpp f2);
define_aliases!(PronounStress: A a, B b, F f);
define_aliases!(AdjectiveFullStress: A a, B b);
define_aliases!(AdjectiveShortStress: A a, B b, C c, Ap a1, Bp b1, Cp c1, Cpp c2);
define_aliases!(VerbPresentStress: A a, B b, C c, Cp c1);
define_aliases!(VerbPastStress: A a, B b, C c, Cp c1, Cpp c2);

pub struct Builder<Main, Alt>(Main, Alt);

#[const_trait]
pub trait Build<T> {
    fn build() -> T;
}

macro_rules! simple_build_fn {
    ($($ty:ty),*) => ($(
        impl<T: StressConst<$ty>> const Build<$ty> for Builder<T, stress_names::Unset> {
            fn build() -> $ty {
                T::STRESS
            }
        }
    )*);
}

simple_build_fn! {
    NounStress, PronounStress,
    AdjectiveFullStress, AdjectiveShortStress,
    VerbPresentStress, VerbPastStress
}

trait IsApOrBp {}
impl IsApOrBp for stress_names::a1 {}
impl IsApOrBp for stress_names::b1 {}

impl<X: StressConst<AdjectiveFullStress>, Y: StressConst<AdjectiveShortStress>> const
    Build<AdjectiveStress> for Builder<X, Y>
{
    fn build() -> AdjectiveStress {
        AdjectiveStress::new(X::STRESS, Y::STRESS)
    }
}
impl<X: StressConst<AdjectiveShortStress> + IsApOrBp> const Build<AdjectiveStress>
    for Builder<X, stress_names::Unset>
{
    fn build() -> AdjectiveStress {
        AdjectiveStress::new(
            match X::STRESS {
                AdjectiveShortStress::Ap => AdjectiveFullStress::A,
                _ => AdjectiveFullStress::B,
            },
            X::STRESS,
        )
    }
}

impl<X: StressConst<VerbPresentStress>, Y: StressConst<VerbPastStress>> const Build<VerbStress>
    for Builder<X, Y>
{
    fn build() -> VerbStress {
        VerbStress::new(X::STRESS, Y::STRESS)
    }
}
impl<X: StressConst<VerbPresentStress>> const Build<VerbStress>
    for Builder<X, stress_names::Unset>
{
    fn build() -> VerbStress {
        VerbStress::new(X::STRESS, VerbPastStress::A)
    }
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
        use $crate::stress::macro_internals::{Build, Builder, stress_names};
        Builder::<stress_names::$x, stress_names::Unset>::build()
    }};
    ($x:ident/$y:ident) => {{
        use $crate::stress::macro_internals::{Build, Builder, stress_names};
        Builder::<stress_names::$x, stress_names::$y>::build()
    }};
}

#[cfg(test)]
mod tests {
    use crate::stress::*;

    #[test]
    pub fn test() {
        let x: NounStress = stress![a];
        assert_eq!(x, NounStress::A);
        let x: NounStress = stress![c];
        assert_eq!(x, NounStress::C);

        let x: PronounStress = stress![a];
        assert_eq!(x, PronounStress::A);
        let x: PronounStress = stress![b];
        assert_eq!(x, PronounStress::B);

        let x: AdjectiveFullStress = stress![b];
        assert_eq!(x, AdjectiveFullStress::B);
        let x: AdjectiveShortStress = stress![c2];
        assert_eq!(x, AdjectiveShortStress::Cpp);

        let x: AdjectiveStress = stress![a / b];
        assert_eq!(x, AdjectiveStress::A_B);
        let x: AdjectiveStress = stress![b / b];
        assert_eq!(x, AdjectiveStress::B);
        let x: AdjectiveStress = stress![b1];
        assert_eq!(x, AdjectiveStress::Bp);
    }
}
