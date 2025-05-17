/// Creates a simple zero-sized error struct.
/// ```ignore
/// define_error! {
///     /// An error for when something goes wrong.
///     pub struct MyError("something went wrong");
/// }
/// ```
/// Automatically derives [`Debug`], [`Default`], [`Clone`], [`Copy`], [`PartialEq`], [`Eq`].
///
/// Implements [`std::fmt::Display`] with given error message, and [`std::error::Error`].
macro_rules! define_error {
    (
        $(#[$meta:meta])*
        $vis:vis struct $err:ident($msg:literal);
    ) => (
        $(#[$meta])*
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        $vis struct $err;

        impl std::fmt::Display for $err {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                $msg.fmt(f)
            }
        }
        impl std::error::Error for $err {}
    );
}

/// `subsetof` keyword: Creates an enum, that _is a subset of_ another enum.
/// ```ignore
/// pub enum Super { A = 1, B = 2, C = 7, D = 8 }
/// define_subenum! {
///     pub enum Sub subsetof Super [SubError] { A, C }
///     // A and C will inherit values from Super (1 and 7)
/// }
/// ```
/// `from` keyword: Creates an enum, _with only variant names from_ another enum.
/// ```ignore
/// pub enum Super { A = 1, B = 2, C = 7, D = 8 }
/// define_subenum! {
///     pub enum Sub from Super [SubError] { A, C }
///     // A and C are numbered independently (0 and 1)
/// }
/// ```
/// Automatically derives [`Debug`], [`Clone`], [`Copy`], [`PartialEq`], [`Eq`].
///
/// Implements [`TryFrom<Super>`] for `Sub` and [`From<Sub>`] for `Super`.
macro_rules! define_subenum {
    (
        $(#[$outer:meta])*
        $vis:vis enum $t:ident from $src:ty [$err:ty] {
            $( $(#[$inner:meta])* $variant:ident, )*
        }
    ) => (
        $(#[$outer])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis enum $t {
            $( $(#[$inner])* $variant, )*
        }

        define_subenum! { @define_from $t, $src, $err, $($variant)* }
    );
    (
        $(#[$outer:meta])*
        $vis:vis enum $t:ident subsetof $src:ty [$err:ty] {
            $( $(#[$inner:meta])* $variant:ident, )*
        }
    ) => (
        $(#[$outer])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis enum $t {
            $( $(#[$inner])* $variant = <$src>::$variant as _, )*
        }

        define_subenum! { @define_from $t, $src, $err, $($variant)* }
    );

    (@define_from $t:ident, $src:ty, $err:ty, $($variant:ident)*) => (
        impl TryFrom<$src> for $t {
            type Error = $err;
            #[allow(unreachable_patterns)]
            fn try_from(value: $src) -> Result<Self, Self::Error> {
                Ok(match value {
                    $( <$src>::$variant => $t::$variant, )*
                    _ => return Err(Self::Error {}),
                })
            }
        }
        impl From<$t> for $src {
            fn from(value: $t) -> Self {
                match value {
                    $( $t::$variant => Self::$variant, )*
                }
            }
        }
    );
}

pub(crate) use {define_error, define_subenum};
