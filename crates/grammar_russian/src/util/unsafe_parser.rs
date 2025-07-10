pub(crate) struct UnsafeParser<'a> {
    start: &'a u8,
    end: &'a u8,
}

impl<'a> UnsafeParser<'a> {
    pub const fn new(s: &'a str) -> Self {
        let start = unsafe { &*s.as_ptr() };
        let end = unsafe { &*(start as *const u8).add(s.len()) };
        UnsafeParser { start, end }
    }

    pub const fn remaining_len(&self) -> usize {
        unsafe { (self.end as *const u8).offset_from_unsigned(self.start as *const u8) }
    }
    pub const fn remaining(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.start, self.remaining_len()) }
    }

    pub const fn forward(&mut self, dist: usize) {
        self.start = unsafe { &*(self.start as *const u8).add(dist) };
    }
    pub const fn finished(&self) -> bool {
        self.remaining_len() == 0
    }

    pub const fn peek<const N: usize>(&self) -> Option<&'a [u8; N]> {
        self.remaining().first_chunk::<N>()
    }
    pub const fn peek_one(&self) -> Option<&'a u8> {
        if !self.finished() { Some(self.start) } else { None }
    }

    pub const fn read<const N: usize>(&mut self) -> Option<&'a [u8; N]> {
        if let Some(chunk) = self.remaining().first_chunk::<N>() {
            self.forward(N);
            return Some(chunk);
        }
        None
    }
    pub const fn read_one(&mut self) -> Option<&'a u8> {
        if !self.finished() {
            let read = self.start;
            self.forward(1);
            return Some(read);
        }
        None
    }

    pub const fn skip_bytes(&mut self, bytes: &[u8]) -> bool {
        if self.remaining_len() >= bytes.len() {
            let peeked = unsafe { std::slice::from_raw_parts(self.start, bytes.len()) };
            if eq_slices(peeked, bytes) {
                self.forward(bytes.len());
                return true;
            }
        }
        false
    }
    pub const fn skip_str(&mut self, s: &str) -> bool {
        self.skip_bytes(s.as_bytes())
    }
    pub const fn skip(&mut self, ch: char) -> bool {
        self.skip_str(ch.encode_utf8(&mut [0; 4]))
    }
}

const fn eq_slices(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut idx = 0;
    while idx < left.len() {
        if left[idx] != right[idx] {
            return false;
        }
        idx += 1;
    }
    true
}

#[const_trait]
pub trait PartialParse: Sized {
    type Err;
    fn partial_parse(parser: &mut UnsafeParser) -> Result<Self, Self::Err>;

    fn from_str_or(s: &str, default_err: Self::Err) -> Result<Self, Self::Err>
    where
        Self::Err: Copy,
        Result<Self, Self::Err>: Copy,
    {
        let mut parser = UnsafeParser::new(s);

        match Self::partial_parse(&mut parser) {
            // Ok only when the entire string was parsed
            Ok(result) if parser.finished() => Ok(result),
            Err(err) => Err(err),
            _ => Err(default_err),
        }
    }
}
