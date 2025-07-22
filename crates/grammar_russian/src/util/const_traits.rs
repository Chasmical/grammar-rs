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
            #[allow(clippy::manual_map)]
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
            #[allow(clippy::manual_map)]
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
where Result<T, E>: ~const std::marker::Destruct
{
    fn _unwrap(self) -> T {
        if let Ok(x) = self { x } else { panic!("called `_Result::_unwrap()` on an `Err` value") }
    }
}

// FIXME(const-hack): Remove these when Option::unwrap/ok_or are constified.

#[const_trait]
pub(crate) trait _Option<T> {
    fn _unwrap(self) -> T;
    fn _ok_or<E: ~const std::marker::Destruct>(self, err: E) -> Result<T, E>;
}
impl<T> const _Option<T> for Option<T>
where Option<T>: ~const std::marker::Destruct
{
    fn _unwrap(self) -> T {
        if let Some(x) = self { x } else { panic!("called `_Option::_unwrap()` on a `None` value") }
    }
    fn _ok_or<E: ~const std::marker::Destruct>(self, err: E) -> Result<T, E> {
        if let Some(x) = self { Ok(x) } else { Err(err) }
    }
}

// FIXME(const-hack): Remove these when ? and From<Err> are constified.

#[const_trait]
pub(crate) trait _Tryable<T, E> {
    fn _as_result(self) -> Result<T, E>;
}
impl<T: ~const std::marker::Destruct, E> const _Tryable<T, E> for Result<T, E> {
    fn _as_result(self) -> Result<T, E> {
        self
    }
}
impl<T: ~const std::marker::Destruct> const _Tryable<T, ()> for Option<T> {
    fn _as_result(self) -> Result<T, ()> {
        if let Some(x) = self { Ok(x) } else { Err(()) }
    }
}

macro_rules! const_try {
    ($expr:expr) => (const_try!($expr, x => x));
    ($expr:expr, $err_fn:path) => (const_try!($expr, x => $err_fn(x)));
    ($expr:expr, $err_expr:expr) => (const_try!($expr, _x => $err_expr));
    ($expr:expr, $err:ident => $err_expr:expr) => ({
        match $crate::util::_Tryable::_as_result($expr) {
            Ok(x) => x,
            Err($err) => return Err($err_expr),
        }
    });
}

pub(crate) use {const_try, impl_const_From, impl_const_TryFrom};
