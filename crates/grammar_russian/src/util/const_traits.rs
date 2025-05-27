// FIXME(const-hack): Remove these when [Try]From/Into are constified.

#[const_trait]
pub(crate) trait _From<T>: Sized {
    fn _from(value: T) -> Self;
}
#[const_trait]
pub(crate) trait _TryFrom<T>: Sized {
    type Error;
    fn _try_from(value: T) -> Result<Self, Self::Error>;
}
#[const_trait]
pub(crate) trait _Into<T>: Sized {
    fn _into(self) -> T;
}
#[const_trait]
pub(crate) trait _TryInto<T>: Sized {
    type Error;
    fn _try_into(self) -> Result<T, Self::Error>;
}

impl<T> const _From<T> for T {
    fn _from(value: T) -> T {
        value
    }
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
impl<T, U: ~const _Into<T>> const _TryFrom<U> for T {
    type Error = std::convert::Infallible;
    fn _try_from(value: U) -> Result<Self, Self::Error> {
        Ok(U::_into(value))
    }
}

macro_rules! impl_const_From {
    (
        <$a:ty> for $b:ty {
            $(#[$outer:meta])*
            fn from($param:ident: $a2:ty) -> Self $body:block
        }
    ) => (
        impl From<$a> for $b {
            $(#[$outer])*
            fn from($param: $a2) -> Self $body
        }
        impl const $crate::util::_From<$a> for $b {
            $(#[$outer])*
            fn _from($param: $a2) -> Self $body
        }
    );
}
macro_rules! impl_const_TryFrom {
    (
        <$a:ty> for $b:ty {
            type Error = $err:ty;
            $(#[$outer:meta])*
            fn try_from($param:ident: $a2:ty) -> Result<Self, Self::Error> $body:block
        }
    ) => (
        impl TryFrom<$a> for $b {
            type Error = $err;
            $(#[$outer])*
            fn try_from($param: $a2) -> Result<Self, Self::Error> $body
        }
        impl const $crate::util::_TryFrom<$a> for $b {
            type Error = $err;
            $(#[$outer])*
            fn _try_from($param: $a2) -> Result<Self, Self::Error> $body
        }
    );
}

// FIXME(const-hack): Remove these when Result::unwrap is constified.

#[const_trait]
pub(crate) trait _Result<T, E> {
    fn _unwrap(self) -> T;
}
impl<T, E: std::fmt::Debug> const _Result<T, E> for Result<T, E>
where Result<T, E>: Copy
{
    fn _unwrap(self) -> T {
        match self {
            Ok(x) => x,
            Err(_) => panic!("called `_Result::_unwrap()` on an `Err` value"),
        }
    }
}

// FIXME(const-hack): Remove these when ? (and From<Err>) is constified.

macro_rules! const_try {
    ($expr:expr) => (const_try!($expr, x => x));
    ($expr:expr, $fn:path) => (const_try!($expr, x => $fn(x)));
    ($expr:expr, $err:ident => $err_expr:expr) => ({
        match $expr {
            Ok(x) => x,
            Err($err) => return Err($err_expr),
        }
    });
}

pub(crate) use {const_try, impl_const_From, impl_const_TryFrom};
