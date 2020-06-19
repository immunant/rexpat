use crate::expat_external_h::XML_Char;
use crate::lib::xmlparse::{ExpatBufRef, ExpatBufRefMut, XmlConvert};
use crate::lib::xmltok::{ENCODING, XML_Convert_Result};

use bumpalo::Bump;
use fallible_collections::FallibleVec;
use libc::c_int;

use std::alloc::Layout;
use std::cell::RefCell;
use std::convert::TryInto;

pub const INIT_BLOCK_SIZE: usize = init_block_size_const();

#[cfg(feature = "mozilla")]
const fn init_block_size_const() -> usize {
    // FIXME: should be `offset_of(BLOCK, s)`, but that's not supported yet,
    // so we over-estimate its offset
    1024 - (std::mem::size_of::<BLOCK>() / std::mem::size_of::<XML_Char>())
}

#[cfg(not(feature = "mozilla"))]
const fn init_block_size_const() -> usize {
    1024
}

/// A StringPool has the purpose of allocating distinct strings and then
/// handing them off to be referenced either temporarily or for the entire length
/// of the pool.
pub(crate) struct StringPool {
    vec: RefCell<Vec<XML_Char>>,
    finished: RefCell<Bump>,
}

impl StringPool {
    pub(crate) fn try_new() -> Result<Self, ()> {
        let vec = Vec::try_with_capacity(INIT_BLOCK_SIZE).map_err(|_| ())?;
        let finished = Bump::try_with_capacity(INIT_BLOCK_SIZE).map_err(|_| ())?;
        Ok(StringPool {
            vec: RefCell::new(vec),
            finished: RefCell::new(finished),
        })
    }

    /// Determines whether or not the current BumpVec is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.vec.borrow().is_empty()
    }

    /// Gets the current vec, converts it into an immutable slice,
    /// and resets bookkeeping so that it will create a new vec next time.
    pub(crate) fn finish_string(&self) -> *const XML_Char {
        let mut vec = self.vec.borrow_mut();
        let mut finished = self.finished.borrow_mut();

        let slice_layout = Layout::for_value(&vec[..]);
        let dst = match finished.try_alloc_layout(slice_layout) {
            Ok(dst) => dst.as_ptr() as *mut XML_Char,
            Err(_) => return std::ptr::null(),
        };

        unsafe {
            std::ptr::copy_nonoverlapping(vec.as_ptr(), dst, vec.len());
        }
        vec.clear();
        dst
    }

    /// Resets the current bump vec to the beginning
    pub(crate) fn clear_current(&self) {
        self.vec.borrow_mut().clear();
    }

    /// Obtains the length of the current BumpVec.
    pub(crate) fn len(&self) -> usize {
        self.vec.borrow().len()
    }

    /// Call callback with an immutable buffer of the current BumpVec. This must
    /// be a callback to ensure that we don't (safely) borrow the slice for
    /// longer than it stays vaild.
    pub(crate) fn current_slice<F, R>(&self, mut callback: F) -> R
        where F: FnMut(&[XML_Char]) -> R
    {
        let vec = self.vec.borrow();
        callback(&vec[..])
    }

    /// Call callback with a mutable buffer of the current BumpVec. This must
    /// be a callback to ensure that we don't (safely) borrow the slice for
    /// longer than it stays vaild.
    pub(crate) fn current_mut_slice<F, R>(&self, mut callback: F) -> R
        where F: FnMut(&mut [XML_Char]) -> R
    {
        let mut vec = self.vec.borrow_mut();
        callback(&mut vec[..])
    }

    /// Unsafe temporary version of `current_slice()`. This needs to be removed
    /// when callers are made safe.
    pub(crate) unsafe fn current_start(&self) -> *const XML_Char {
        let vec = self.vec.borrow();
        vec[..].as_ptr()
    }

    /// Appends a char to the current BumpVec.
    pub(crate) fn append_char(&self, c: XML_Char) -> bool {
        let mut vec = self.vec.borrow_mut();
        Self::append_char_inner(&mut vec, c)
    }

    /// Overwrites the last char in the current BumpVec.
    /// Note that this will panic if empty. This is not an insert
    /// operation as it does not shift bytes afterwards.
    pub(crate) fn replace_last_char(&self, c: XML_Char) {
        let mut vec = self.vec.borrow_mut();
        *vec.last_mut()
            .expect("Called replace_last_char() when string was empty") = c;
    }

    /// Decrements the length, panicing if len is 0
    pub(crate) fn backtrack(&self) {
        let mut vec = self.vec.borrow_mut();
        vec.pop().expect("Called backtrack() on empty StringPool");
    }

    /// Gets the last character, panicing if len is 0
    pub(crate) fn get_last_char(&self) -> XML_Char {
        let vec = self.vec.borrow();
        *vec.last().expect("Called get_last_char() when string was empty")
    }

    /// Appends an entire C String to the current BumpVec.
    pub(crate) unsafe fn append_c_string(&self, mut s: *const XML_Char) -> bool {
        let mut vec = self.vec.borrow_mut();
        while *s != 0 {
            if !Self::append_char_inner(&mut vec, *s) {
                return false;
            }
            s = s.offset(1)
        }
        true
    }

    /// Resets the current Vec and deallocates its contents.
    pub(crate) fn clear(&mut self) {
        self.vec.get_mut().clear();
        self.finished.get_mut().reset();
    }

    pub(crate) fn store_c_string(
        &self,
        enc: &ENCODING,
        buf: ExpatBufRef,
    ) -> bool {
        let mut vec = self.vec.borrow_mut();
        if !Self::convert(&mut vec, enc, buf) {
            return false;
        }

        if !Self::append_char_inner(&mut vec, '\0' as XML_Char) {
            return false;
        }

        true
    }

    pub(crate) fn append(
        &self,
        enc: &ENCODING,
        read_buf: ExpatBufRef,
    ) -> bool {
        let mut vec = self.vec.borrow_mut();
        Self::convert(&mut vec, enc, read_buf)
    }

    pub(crate) unsafe fn copy_c_string(
        &self,
        mut s: *const XML_Char,
    ) -> bool {
        // self.append_c_string(s);?
        let mut vec = self.vec.borrow_mut();
        loop {
            if !Self::append_char_inner(&mut vec, *s) {
                return false;
            }
            if *s == 0 {
                break;
            }
            s = s.offset(1);
        }
        true
    }

    pub(crate) unsafe fn copy_c_string_n(
        &self,
        mut s: *const XML_Char,
        mut n: c_int,
    ) -> bool {
        let mut vec = self.vec.borrow_mut();
        let mut n = n.try_into().unwrap();

        if vec.try_reserve_exact(n).is_err() {
            return false;
        };

        while n > 0 {
            if !Self::append_char_inner(&mut vec, *s) {
                return false;
            }
            n -= 1;
            s = s.offset(1)
        }
        true
    }

    fn convert<'a>(
        vec: &mut Vec<XML_Char>,
        enc: &ENCODING,
        mut read_buf: ExpatBufRef<'a>,
    ) -> bool {
        loop {
            // REXPAT: always reserve at least 4 bytes,
            // so at least one character gets converted every iteration
            if vec.try_reserve(read_buf.len().max(4)).is_err() {
                return false;
            }

            let start_len = vec.len();
            let cap = vec.capacity();

            vec.resize(cap, 0);

            let mut write_buf = ExpatBufRefMut::from(&mut vec[start_len..]);
            let write_buf_len = write_buf.len();
            let convert_res = XmlConvert!(enc, &mut read_buf, &mut write_buf);
            // The write buf shrinks by how much was written to it
            let written_size = write_buf_len - write_buf.len();

            vec.truncate(start_len + written_size);

            if convert_res == XML_Convert_Result::COMPLETED || convert_res == XML_Convert_Result::INPUT_INCOMPLETE {
                return true;
            }
        }
    }

    fn append_char_inner(vec: &mut Vec<XML_Char>, c: XML_Char) -> bool {
        if vec.try_reserve(1).is_err() {
            false
        } else {
            vec.push(c);

            true
        }
    }
}

#[cfg(test)]
mod consts {
    use super::XML_Char;

    pub const A: XML_Char = 'a' as XML_Char;
    pub const B: XML_Char = 'b' as XML_Char;
    pub const C: XML_Char = 'c' as XML_Char;
    pub const D: XML_Char = 'd' as XML_Char;
    pub const NULL: XML_Char = '\0' as XML_Char;
    pub static S: [XML_Char; 5] = [C, D, D, C, NULL];
}

#[test]
fn test_append_char() {
    use consts::*;

    let mut pool = StringPool::new();

    assert!(pool.append_char(A));
    pool.current_slice(|s| assert_eq!(s, [A]));

    assert!(pool.append_char(B));
    pool.current_slice(|s| assert_eq!(s, [A, B]));

    // New BumpVec
    pool.finish_string();

    assert!(pool.append_char(C));
    pool.current_slice(|s| assert_eq!(s, [C]));
}

#[test]
fn test_append_string() {
    use consts::*;

    let mut pool = StringPool::new();
    let mut string = [A, B, C, NULL];

    unsafe {
        assert!(pool.append_c_string(string.as_mut_ptr()));
    }

    pool.current_slice(|s| assert_eq!(s, [A, B, C]));
}

#[test]
fn test_copy_string() {
    use consts::*;

    let mut pool = StringPool::new();

    assert!(pool.append_char(A));
    pool.current_slice(|s| assert_eq!(s, [A]));

    let new_string = unsafe {
        pool.copy_c_string(S.as_ptr())
    };

    assert_eq!(new_string.unwrap()[..], [A, C, D, D, C, NULL]);
    assert!(pool.append_char(B));
    pool.current_slice(|s| assert_eq!(s, [B]));

    let new_string2 = unsafe {
        pool.copy_c_string_n(S.as_ptr(), 4)
    };

    assert_eq!(new_string2.unwrap()[..], [B, C, D, D, C]);
}

#[test]
fn test_store_c_string() {
    use consts::*;
    use crate::lib::xmlparse::XmlGetInternalEncoding;

    let mut pool = StringPool::new();
    let enc = XmlGetInternalEncoding();
    let read_buf = unsafe {
        ExpatBufRef::new(S.as_ptr(), S.as_ptr().add(3))
    };
    assert!(pool.store_c_string(enc, read_buf));
    pool.current_slice(|s| assert_eq!(s, [C, D, D, NULL]));
    pool.finish_string();

    assert!(pool.append_char(A));
    pool.current_slice(|s| assert_eq!(s, [A]));
    assert!(pool.append_char(A));

    pool.current_slice(|s| assert_eq!(s, [A, A]));

    assert!(pool.store_c_string(enc, read_buf));

    pool.finish_string();
    pool.current_slice(|s| assert_eq!(s, [A, A, C, D, D, NULL]));
}
