#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Letter {
    pub(crate) utf8: [u8; 2],
}

pub mod letters {
    use super::Letter;

    macro_rules! define_letters {
        ($($ident:ident)*) => ($(
            pub const $ident: Letter = Letter::from_str_unchecked(stringify!($ident));
        )*);
    }
    define_letters! { а б в г д е ё ж з и й к л м н о п р с т у ф х ц ч ш щ ъ ы ь э ю я }
}
use letters::*;

impl Letter {
    pub const fn from(ch: char) -> Self {
        let mut utf8: [u8; 2] = [0; 2];
        ch.encode_utf8(&mut utf8);
        Letter { utf8 }
    }
    pub const fn from_str_unchecked(str: &str) -> Self {
        let mut utf8: [u8; 2] = [0; 2];
        utf8.copy_from_slice(str.as_bytes());
        Letter { utf8 }
    }

    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.utf8) }
    }
    pub const fn as_char(&self) -> char {
        unsafe {
            char::from_u32_unchecked(
                (((self.utf8[0] & 0x1F) as u32) << 6) | (self.utf8[1] & 0x3F) as u32,
            )
        }
    }

    pub const fn is_vowel(self) -> bool {
        matches!(self, а | е | и | о | у | ы | э | ю | я | ё)
    }
    pub const fn is_hissing(self) -> bool {
        matches!(self, ж | ч | ш | щ)
    }
    pub const fn is_sibilant(self) -> bool {
        matches!(self, ж | ц | ч | ш | щ)
    }
    pub const fn is_non_sibilant_consonant(self) -> bool {
        matches!(self, б | в | г | д | з | й | к | л | м | н | п | р | с | т | ф | х)
    }
    #[rustfmt::skip]
    pub const fn is_consonant(self) -> bool {
        matches!(self, б | в | г | д | ж | з | й | к | л | м | н | п | р | с | т | ф | х | ц | ч | ш | щ)
    }

    pub const fn from_bytes(slice: &[u8]) -> &[Letter] {
        unsafe {
            let ptr: *const Letter = std::mem::transmute(slice.as_ptr());
            std::slice::from_raw_parts(ptr, slice.len() >> 1)
        }
    }
    pub const fn from_bytes_mut(slice: &mut [u8]) -> &mut [Letter] {
        unsafe {
            let ptr: *mut Letter = std::mem::transmute(slice.as_mut_ptr());
            std::slice::from_raw_parts_mut(ptr, slice.len() >> 1)
        }
    }
}

#[const_trait]
pub trait LetterSliceExt {
    fn as_bytes(&self) -> &[u8];
    fn as_str(&self) -> &str;
}
impl const LetterSliceExt for [Letter] {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr: *const u8 = std::mem::transmute(self.as_ptr());
            std::slice::from_raw_parts(ptr, self.len() << 1)
        }
    }
    fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        assert_eq!(а.as_char(), 'а');
        assert_eq!(п.as_char(), 'п');
        assert_eq!(р.as_char(), 'р');
        assert_eq!(я.as_char(), 'я');
        assert_eq!(ё.as_char(), 'ё');

        assert_eq!(а.as_str(), "а");
        assert_eq!(п.as_str(), "п");
        assert_eq!(р.as_str(), "р");
        assert_eq!(я.as_str(), "я");
        assert_eq!(ё.as_str(), "ё");

        let bytes: &[u8] = "апряё".as_bytes();
        let letters: &[Letter] = Letter::from_bytes(bytes);
        assert_eq!(letters, [а, п, р, я, ё]);
    }
}
