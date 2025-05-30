pub(crate) struct UnsafeBuf<'a> {
    start: &'a u8,
    end: *mut u8,
}

impl<'a> UnsafeBuf<'a> {
    pub const fn new<const N: usize>(dst: &'a mut [u8; N]) -> Self {
        let start = unsafe { std::mem::transmute(dst.first().unwrap()) };
        let end = dst.first_mut().unwrap() as *mut u8;
        Self { start, end }
    }

    pub const fn push_bytes(&mut self, bytes: &[u8]) {
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), self.end, bytes.len()) };
        self.end = self.end.wrapping_add(bytes.len());
    }
    pub const fn push_str(&mut self, s: &str) {
        self.push_bytes(s.as_bytes());
    }
    pub const fn push(&mut self, ch: char) {
        let buffer: &mut [u8; 4] = unsafe { std::mem::transmute(self.end) };
        self.end = self.end.wrapping_add(ch.encode_utf8(buffer).len());
    }
    pub const fn push_byte(&mut self, byte: u8) {
        unsafe { *self.end = byte };
        self.end = self.end.wrapping_add(1);
    }

    pub fn push_fmt<const N: usize, F: FnOnce(&'a mut [u8; N]) -> &'a str>(&mut self, format: F) {
        let buffer: &mut [u8; N] = unsafe { std::mem::transmute(self.end) };
        self.end = self.end.wrapping_add(format(buffer).len());
    }

    pub const fn sub_buf<const N: usize>(&mut self) -> &mut [u8; N] {
        unsafe { std::mem::transmute(self.end) }
    }
    pub const fn advance_by(&mut self, distance: usize) {
        self.end = self.end.wrapping_add(distance);
    }

    pub const fn finish(self) -> &'a mut str {
        unsafe {
            str::from_utf8_unchecked_mut(std::slice::from_raw_parts_mut(
                std::mem::transmute(self.start),
                self.end.offset_from_unsigned(self.start),
            ))
        }
    }
}
