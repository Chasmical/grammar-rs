// FIXME(const-hack): Remove these when Result::unwrap is constified.

pub(crate) const trait _Result<T, E> {
    fn _unwrap(self) -> T;
}
impl<T, E: std::fmt::Debug> const _Result<T, E> for Result<T, E>
where Result<T, E>: [const] std::marker::Destruct
{
    fn _unwrap(self) -> T {
        if let Ok(x) = self { x } else { panic!("called `_Result::_unwrap()` on an `Err` value") }
    }
}

// FIXME(const-hack): Remove these when ? and From<Err> are constified.

pub(crate) const trait _Tryable<T, E> {
    fn _as_result(self) -> Result<T, E>;
}
impl<T: [const] std::marker::Destruct, E> const _Tryable<T, E> for Result<T, E> {
    fn _as_result(self) -> Result<T, E> {
        self
    }
}
impl<T: [const] std::marker::Destruct> const _Tryable<T, ()> for Option<T> {
    fn _as_result(self) -> Result<T, ()> {
        self.ok_or(())
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

pub(crate) use const_try;
