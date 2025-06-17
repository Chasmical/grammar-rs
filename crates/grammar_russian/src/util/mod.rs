pub(crate) mod unsafe_buf;
pub(crate) use unsafe_buf::*;
pub(crate) mod unsafe_parser;
pub(crate) use unsafe_parser::*;

pub(crate) mod const_traits;
pub(crate) use const_traits::*;

macro_rules! enum_conversion {
    (
        impl From<$a:ty, Error = $err:ty> for $b:ty {
            $($variant:ident,)*
        }
    ) => (
        $crate::util::impl_const_From!(<$a> for $b {
            fn from(value: $a) -> Self {
                match value {
                    $(<$a>::$variant => Self::$variant,)*
                }
            }
        });
        $crate::util::impl_const_TryFrom!(<$b> for $a {
            type Error = $err;
            #[allow(unreachable_patterns)]
            fn try_from(value: $b) -> Result<Self, Self::Error> {
                Ok(match value {
                    $( <$b>::$variant => Self::$variant, )*
                    _ => return Err(Self::Error {}),
                })
            }
        });
    );
}

pub(crate) use enum_conversion;
