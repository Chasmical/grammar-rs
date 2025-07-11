use std::intrinsics::const_eval_select;
extern crate memchr;

// FIXME(const-hack): Remove this and replace calls when `haystack.find(needle)` is introduced and constified.
pub const fn slice_find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    const_eval_select((haystack, needle), slice_find_const, memchr::memmem::find)
}
const fn slice_find_const(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let mut idx = 0;
    while idx <= haystack.len() - needle.len() {
        let window =
            unsafe { std::slice::from_raw_parts(haystack.as_ptr().add(idx), needle.len()) };
        if slice_eq_const(window, needle) {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

// FIXME(const-hack): Remove this and replace calls with simple `==` when `Eq` is constified.
pub const fn slice_eq(left: &[u8], right: &[u8]) -> bool {
    const_eval_select((left, right), slice_eq_const, PartialEq::eq)
}
const fn slice_eq_const(left: &[u8], right: &[u8]) -> bool {
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
