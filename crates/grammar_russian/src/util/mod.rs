pub(crate) mod unsafe_buf;
pub(crate) use unsafe_buf::*;
pub(crate) mod unsafe_parser;
pub(crate) use unsafe_parser::*;

pub(crate) mod const_traits;
pub(crate) use const_traits::*;
pub(crate) mod const_utils;
pub(crate) use const_utils::*;

macro_rules! enum_conversion {
    (
        $from:ty => <= $to:ty { $($variant:ident,)* }
    ) => (
        $crate::util::impl_const_From!(<$from> for $to {
            fn from(value: $from) -> Self {
                match value { $(<$from>::$variant => <$to>::$variant,)* }
            }
        });
        $crate::util::impl_const_From!(<$to> for $from {
            fn from(value: $to) -> Self {
                match value { $( <$to>::$variant => <$from>::$variant, )* }
            }
        });
    );
    (
        $from:ty => $to:ty [<= $err:ty] { $($variant:ident,)* }
    ) => (
        $crate::util::impl_const_From!(<$from> for $to {
            fn from(value: $from) -> Self {
                match value { $(<$from>::$variant => <$to>::$variant,)* }
            }
        });
        $crate::util::impl_const_TryFrom!(<$to> for $from {
            type Error = $err;
            fn try_from(value: $to) -> Result<Self, Self::Error> {
                Ok(match value {
                    $( <$to>::$variant => <$from>::$variant, )*
                    _ => return Err(Self::Error {}),
                })
            }
        });
    );
}

macro_rules! utf8_bytes {
    ($ch:literal) => {{
        let mut buf = [0; $ch.len_utf8()];
        $ch.encode_utf8(&mut buf);
        buf
    }};
}

pub(crate) use {enum_conversion, utf8_bytes};
