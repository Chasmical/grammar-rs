#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Letter {
    utf8: [u8; 2],
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
        return unsafe {
            char::from_u32_unchecked(
                (((self.utf8[0] & 0x1F) as u32) << 6) | (self.utf8[1] & 0x3F) as u32,
            )
        };
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
            let ptr = std::mem::transmute(slice.as_ptr());
            std::slice::from_raw_parts(ptr, slice.len() >> 1)
        }
    }
    pub const fn from_bytes_mut(slice: &mut [u8]) -> &mut [Letter] {
        unsafe {
            let ptr = std::mem::transmute(slice.as_mut_ptr());
            std::slice::from_raw_parts_mut(ptr, slice.len() >> 1)
        }
    }
}

#[const_trait]
pub trait LetterSliceExt {
    fn as_str(&self) -> &str;
    fn as_bytes(&self) -> &[u8];
    fn as_mut_str(&mut self) -> &mut str;
    fn as_bytes_mut(&mut self) -> &mut [u8];
}

impl const LetterSliceExt for [Letter] {
    fn as_str(&self) -> &str {
        unsafe {
            let ptr = std::mem::transmute(self.as_ptr());
            let slice = std::slice::from_raw_parts(ptr, self.len() << 1);
            str::from_utf8_unchecked(slice)
        }
    }
    fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
    fn as_mut_str(&mut self) -> &mut str {
        unsafe {
            let ptr = std::mem::transmute(self.as_mut_ptr());
            let slice = std::slice::from_raw_parts_mut(ptr, self.len() << 1);
            str::from_utf8_unchecked_mut(slice)
        }
    }
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { self.as_mut_str().as_bytes_mut() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        assert_eq!(а.as_char(), 'а'); // 0xD0 0xB0
        assert_eq!(п.as_char(), 'п'); // 0xD0 0xBF
        assert_eq!(р.as_char(), 'р'); // 0xD1 0x80
        assert_eq!(я.as_char(), 'я'); // 0xD1 0x8F
        assert_eq!(ё.as_char(), 'ё'); // 0xD1 0x91

        assert_eq!(а.as_str(), "а");
        assert_eq!(п.as_str(), "п");
        assert_eq!(р.as_str(), "р");
        assert_eq!(я.as_str(), "я");
        assert_eq!(ё.as_str(), "ё");

        let bytes: &[u8] = &[0xD0, 0xB0, 0xD0, 0xBF, 0xD1, 0x80, 0xD1, 0x8F, 0xD1, 0x91];
        let letters: &[Letter] = Letter::from_bytes(bytes);
        assert_eq!(letters, [а, п, р, я, ё]);
    }
}
