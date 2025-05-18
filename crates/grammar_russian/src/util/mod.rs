#[const_trait]
pub trait _From<T>: Sized {
    fn _from(value: T) -> Self;
}
#[const_trait]
pub trait _TryFrom<T>: Sized {
    type Error;
    fn _try_from(value: T) -> Result<Self, Self::Error>;
}
#[const_trait]
pub trait _Into<T>: Sized {
    fn _into(self) -> T;
}
#[const_trait]
pub trait _TryInto<T>: Sized {
    type Error;
    fn _try_into(self) -> Result<T, Self::Error>;
}

impl<T, U: ~const _From<T>> const _Into<U> for T {
    fn _into(self) -> U {
        U::_from(self)
    }
}
impl<T, U: ~const _TryFrom<T>> const _TryInto<U> for T {
    type Error = U::Error;
    fn _try_into(self) -> Result<U, Self::Error> {
        U::_try_from(self)
    }
}

macro_rules! enum_conversion {
    (
        impl From<$a:ty, Error = $err:ty> for $b:ty {
            $($variant:ident,)*
        }
    ) => (
        impl const $crate::util::_From<$a> for $b {
            fn _from(value: $a) -> Self {
                match value {
                    $(<$a>::$variant => Self::$variant,)*
                }
            }
        }
        impl From<$a> for $b {
            fn from(value: $a) -> Self {
                match value {
                    $(<$a>::$variant => Self::$variant,)*
                }
            }
        }
        impl const $crate::util::_TryFrom<$b> for $a {
            type Error = $err;
            #[allow(unreachable_patterns)]
            fn _try_from(value: $b) -> Result<Self, Self::Error> {
                Ok(match value {
                    $( <$b>::$variant => Self::$variant, )*
                    _ => return Err(Self::Error {}),
                })
            }
        }
        impl TryFrom<$b> for $a {
            type Error = $err;
            #[allow(unreachable_patterns)]
            fn try_from(value: $b) -> Result<Self, Self::Error> {
                Ok(match value {
                    $( <$b>::$variant => Self::$variant, )*
                    _ => return Err(Self::Error {}),
                })
            }
        }
    );
}

pub(crate) use enum_conversion;
