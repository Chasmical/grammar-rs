pub(crate) struct UnsafeBuf<'a> {
    start: &'a mut u8,
    end: &'a mut u8,
}

impl<'a> UnsafeBuf<'a> {
    pub const fn new<const N: usize>(dst: &'a mut [u8; N]) -> Self {
        let start = dst.first_mut().unwrap();
        let end = unsafe { &mut *(start as *mut u8) };
        Self { start, end }
    }

    pub const fn push_bytes(&mut self, bytes: &[u8]) {
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), self.end, bytes.len()) };
        self.forward(bytes.len());
    }
    pub const fn push_byte(&mut self, byte: u8) {
        *self.end = byte;
        self.forward(1);
    }
    pub const fn push_str(&mut self, s: &str) {
        self.push_bytes(s.as_bytes());
    }
    pub const fn push(&mut self, ch: char) {
        let buffer: &mut [u8; 4] = self.chunk();
        self.forward(ch.encode_utf8(buffer).len());
    }

    pub const fn chunk<const N: usize>(&mut self) -> &'a mut [u8; N] {
        unsafe { &mut *(self.end as *mut u8 as *mut [u8; N]) }
    }
    pub const fn forward(&mut self, distance: usize) {
        self.end = unsafe { &mut *(self.end as *mut u8).add(distance) };
    }

    pub const fn finish(self) -> &'a mut str {
        unsafe {
            let start = self.start as *mut u8;
            let len = (self.end as *mut u8).offset_from_unsigned(start);
            str::from_utf8_unchecked_mut(std::slice::from_raw_parts_mut(start, len))
        }
    }
}
