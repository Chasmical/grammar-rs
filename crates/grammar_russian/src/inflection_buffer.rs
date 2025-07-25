use crate::Letter;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct InflectionBuffer {
    dst: Vec<u8>,
    pub stem_len: usize,
}

impl InflectionBuffer {
    // TODO: document that stem must contain only cyrillic alphabetic characters
    pub fn from_stem_unchecked(stem: &str) -> Self {
        let mut dst = Vec::with_capacity(stem.len() + 16);
        dst.extend_from_slice(stem.as_bytes());
        Self { dst, stem_len: stem.len() }
    }

    pub const fn stem(&self) -> &[Letter] {
        // FIXME(const-hack): Remove `as_slice()` when Deref for Vec is constified.
        Letter::from_bytes(self.dst.as_slice().get(..self.stem_len).unwrap())
    }
    pub const fn stem_mut(&mut self) -> &mut [Letter] {
        // FIXME(const-hack): Remove `as_mut_slice()` when Deref for Vec is constified.
        Letter::from_bytes_mut(self.dst.as_mut_slice().get_mut(..self.stem_len).unwrap())
    }
    pub const fn ending(&self) -> &[Letter] {
        // FIXME(const-hack): Remove `as_slice()` when Deref for Vec is constified.
        Letter::from_bytes(self.dst.as_slice().get(self.stem_len..).unwrap())
    }
    pub const fn ending_mut(&mut self) -> &mut [Letter] {
        // FIXME(const-hack): Remove `as_mut_slice()` when Deref for Vec is constified.
        Letter::from_bytes_mut(self.dst.as_mut_slice().get_mut(self.stem_len..).unwrap())
    }

    pub fn append_to_ending(&mut self, append: &str) {
        self.dst.extend_from_slice(append.as_bytes());
    }
    pub fn replace_ending(&mut self, new_ending: &str) {
        self.dst.splice(self.stem_len.., new_ending.bytes());
    }

    pub fn append_to_stem(&mut self, append: &str) {
        self.insert_at(self.stem_len, append);
        self.stem_len += append.len();
    }
    pub fn shrink_stem_by(&mut self, shrink: usize) {
        self.dst.splice((self.stem_len - shrink)..self.stem_len, []);
        self.stem_len -= shrink;
    }
    pub fn remove_from_stem<R: std::ops::RangeBounds<usize>>(&mut self, range: R) {
        let shrink = self.dst.splice(range, []).len();
        self.stem_len -= shrink;
    }
    pub fn insert_between_last_two_stem_letters(&mut self, ch: Letter) {
        self.insert_at(self.stem_len - 2, ch.as_str());
        self.stem_len += 2;
    }
    fn insert_at(&mut self, index: usize, replace: &str) {
        self.dst.splice(index..index, replace.bytes());
    }

    pub const fn as_str(&self) -> &str {
        // FIXME(const-hack): Remove `as_slice()` when Deref for Vec is constified.
        unsafe { str::from_utf8_unchecked(self.dst.as_slice()) }
    }
}
