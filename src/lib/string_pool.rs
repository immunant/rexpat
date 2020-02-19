use crate::expat_external_h::XML_Char;
use crate::lib::xmlparse::{ICHAR, INIT_BLOCK_SIZE, ExpatBufRef, ExpatBufRefMut, XmlConvert};
use crate::lib::xmltok::{ENCODING, XML_CONVERT_INPUT_INCOMPLETE, XML_CONVERT_COMPLETED};
use crate::stddef_h::size_t;

use bumpalo::Bump;
use bumpalo::collections::vec::Vec as BumpVec;
use libc::{INT_MAX, c_int, c_uint, c_ulong};

use std::cell::Cell;
use std::convert::TryInto;
use std::mem::ManuallyDrop;
use std::ptr;

fn poolBytesToAllocateFor(mut blockSize: c_int) -> size_t {
    /* Unprotected math would be:
     ** return offsetof(BLOCK, s) + blockSize * sizeof(XML_Char);
     **
     ** Detect overflow, avoiding _signed_ overflow undefined behavior
     ** For a + b * c we check b * c in isolation first, so that addition of a
     ** on top has no chance of making us accept a small non-negative number
     */
    let stretch: size_t = ::std::mem::size_of::<XML_Char>() as c_ulong; /* can be 4 bytes */
    if blockSize <= 0 {
        return 0u64;
    }
    if blockSize > (INT_MAX as c_ulong).wrapping_div(stretch) as c_int {
        return 0u64;
    }
    let stretchedBlockSize: c_int = blockSize * stretch as c_int;
    let bytesToAllocate: c_int =
        (12u64).wrapping_add(stretchedBlockSize as c_uint as c_ulong) as c_int;
    if bytesToAllocate < 0 {
        return 0u64;
    }
    bytesToAllocate as size_t
}

#[derive(Copy, Clone, PartialEq)]
struct RawBumpVec {
    start: *mut XML_Char, // self.start
    len: usize, // self.ptr
    cap: usize,
}

impl RawBumpVec {
    fn new() -> Self {
        RawBumpVec {
            start: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
}

// #[derive(PartialEq)]
pub(crate) struct StringPool {
    bump: Bump,
    currentBumpVec: Cell<RawBumpVec>,
}

impl StringPool {
    pub(crate) fn new() -> Result<Self, ()> {
        Ok(StringPool {
            bump: Bump::try_new()?,
            currentBumpVec: Cell::new(RawBumpVec::new()),
        })
    }

    pub(crate) fn is_empty(&self) -> bool {
        let RawBumpVec { len, .. } = self.currentBumpVec.get();

        len == 0
    }

    pub(crate) fn is_full(&self) -> bool {
        let RawBumpVec { cap, len, .. } = self.currentBumpVec.get();

        cap == len
    }

    /// Gets the current vec, converts it into a slice, and resets
    /// bookkeeping so that it will create a new vec next time get_bump_vec
    /// is called.
    fn consume_current_vec(&self) -> &mut [XML_Char] {
        let slice = ManuallyDrop::into_inner(self.get_bump_vec()).into_bump_slice_mut();

        self.currentBumpVec.set(RawBumpVec::new());

        slice
    }

    /// Resets the current bump vec to the beginning
    pub(crate) fn clear_current(&self) {
        let RawBumpVec { start, cap, .. } = self.currentBumpVec.get();

        self.currentBumpVec.set(RawBumpVec { start, cap, len: 0 });
    }

    /// Moves the start pointer to the current length so that a new bump vec region begins
    /// REVIEW: Maybe shouldn't be necessary or just force a new vec creation..?
    pub(crate) fn finish_current(&self) {
        // let RawBumpVec { mut start, cap, len } = self.currentBumpVec.get();

        // unsafe {
        //     start = start.add(len);
        // }

        // self.currentBumpVec.set(RawBumpVec { start, cap: cap - len, len: 0 });
        self.resetCurrentBumpVec()
    }

    pub(crate) fn len(&self) -> usize {
        self.currentBumpVec.get().len
    }

    // TODO: Use finish_current instead?
    // #[cfg(test)]
    fn resetCurrentBumpVec(&self) {
        self.currentBumpVec.set(RawBumpVec::new())
    }

    fn get_bump_vec(&self) -> ManuallyDrop<BumpVec<XML_Char>> {
        // If we don't already have an existing bump vec, create a new one
        // otherwise regenerate previous one
        if self.currentBumpVec.get().start.is_null() {
            ManuallyDrop::new(BumpVec::new_in(&self.bump))
        } else {
            let RawBumpVec { start, len, cap } = self.currentBumpVec.get();

            unsafe {
                ManuallyDrop::new(BumpVec::from_raw_parts_in(start, len, cap, &self.bump))
            }
        }
    }

    pub(crate) fn current_slice(&self) -> &[XML_Char] {
        ManuallyDrop::into_inner(self.get_bump_vec()).into_bump_slice()
    }

    /// Updates bookeeping so that the current bump vec can be regenerated
    fn update_raw(&self, buf: &mut BumpVec<XML_Char>) {
        debug_assert!(buf.len() <= buf.capacity());

        self.currentBumpVec.set(RawBumpVec {
            start: buf.as_mut_ptr(),
            len: buf.len(),
            cap: buf.capacity(),
        });
    }

    pub(crate) fn appendChar(&self, c: XML_Char) -> bool {
        if self.is_full() && !self.grow() {
            false
        } else {
            let mut buf = self.get_bump_vec();

            buf.push(c);

            self.update_raw(&mut buf);

            true
        }
    }

    /// Note that this will panic if empty and that this is not an insert
    /// operation as it does not shift bytes afterwards.
    pub(crate) fn prepend_char(&self, c: XML_Char) {
        let mut buf = self.get_bump_vec();
        let len = buf.len();

        buf[len - 1] = c;
    }

    /// Decrements the length, panicing if len is 0
    pub(crate) fn backtrack(&self) {
        let RawBumpVec { start, cap, len } = self.currentBumpVec.get();

        assert!(len > 0);

        self.currentBumpVec.set(RawBumpVec { start, cap, len: len - 1 });
    }

    /// Gets the last character, panicing if len is 0
    pub(crate) fn get_last_char(&self) -> XML_Char {
        let mut buf = self.get_bump_vec();

        buf[buf.len() - 1]
    }

    pub(crate) unsafe fn appendString(&self, mut s: *const XML_Char) -> bool {
        while *s != 0 {
            if !self.appendChar(*s) {
                return false;
            }
            s = s.offset(1)
        }
        !self.currentBumpVec.get().start.is_null() // TODO: Just return true?
    }

    pub(crate) fn clear(&mut self) {
        self.bump.reset()
    }

    /// Replaced by drop?
    pub(crate) unsafe fn destroy(&mut self) {

    }

    pub(crate) unsafe fn storeString(
        &self,
        enc: &ENCODING,
        buf: ExpatBufRef,
    ) -> Option<&mut [XML_Char]> {
        if !self.append(enc, buf) {
            return None;
        }
        if self.is_full() && !self.grow() {
            return None;
        }

        self.appendChar('\0' as XML_Char);

        Some(self.consume_current_vec())
    }

    unsafe fn set_len(&self, len: usize) {
        let mut buf = self.get_bump_vec();

        debug_assert!(len <= buf.capacity());

        buf.set_len(len);

        self.update_raw(&mut buf);
    }

    pub(crate) unsafe fn append(
        &self,
        enc: &ENCODING,
        mut readBuf: ExpatBufRef,
    ) -> bool {
        let RawBumpVec { mut start, .. } = self.currentBumpVec.get();

        // REVIEW: Can this be replaced with self.is_full() &&?
        if start.is_null() && !self.grow() {
            return false;
        }

        let RawBumpVec { mut start, cap, len } = self.currentBumpVec.get();
        let mut end2;
        let mut start2;

        // TODO: Test looping w/ new cap/len.
        loop {
            debug_assert!(len <= cap);

            start2 = start.add(len);
            end2 = start.add(cap) as *mut ICHAR;

            // Zero cap bytes
            let mut tmp = start2;

            while tmp != end2 {
                std::ptr::write(tmp, 0);
                tmp = tmp.add(1);
            }

            // Vec[init len, uninit cap - len]
            // FIXME: Writing to slice of uninit memory (UB most likely)
            // XmlConvert should probably take a &mut ExpatBufRefMut<T = MaybeUninit<XML_Char>> instead
            let mut writeBuf = ExpatBufRefMut::new(start2, end2);

            let convert_res = XmlConvert!(
                enc,
                &mut readBuf,
                &mut writeBuf,
            );

            // TODO: How to not need wrapping_offset_from here? Cast to u/isize first?
            let len = writeBuf.as_ptr().wrapping_offset_from(start).try_into().unwrap();

            self.set_len(len);

            if convert_res == XML_CONVERT_COMPLETED || convert_res == XML_CONVERT_INPUT_INCOMPLETE {
                break;
            }

            if !self.grow() {
                return false;
            }
        }

        !self.currentBumpVec.get().start.is_null() // TODO: Just return true?
    }

    pub(crate) unsafe fn copyString(
        &self,
        mut s: *const XML_Char,
    ) -> Option<&mut [XML_Char]> {
        // self.appendString(s);?
        loop {
            if !self.appendChar(*s) {
                return None;
            }
            let fresh78 = s;
            s = s.offset(1);
            if *fresh78 == 0 {
                break;
            }
        }

        Some(self.consume_current_vec())
    }

    pub(crate) unsafe fn copyStringN(
        &self,
        mut s: *const XML_Char,
        mut n: c_int,
    ) -> Option<&[XML_Char]> {
        // REVIEW: Is this correct:
        // if self.ptr.is_null() && !self.grow() {
        if self.is_empty() && !self.grow() {
            /* The following line is unreachable given the current usage of
            * poolCopyStringN().  Currently it is called from exactly one
            * place to copy the text of a simple general entity.  By that
            * point, the name of the entity is already stored in the pool, so
            * pool->ptr cannot be NULL.
            *
            * If poolCopyStringN() is used elsewhere as it well might be,
            * this line may well become executable again.  Regardless, this
            * sort of check shouldn't be removed lightly, so we just exclude
            * it from the coverage statistics.
            */
            return None;
        }
        while n > 0 {
            if !self.appendChar(*s) {
                return None;
            }
            n -= 1;
            s = s.offset(1)
        }

        Some(&*self.consume_current_vec())
    }

    /// There's currently no try_push in Bumpalo, so can't determine if
    /// it's possible to allocate or not
    pub(crate) fn grow(&self) -> bool {
        let mut buf = self.get_bump_vec();
        let mut blockSize_0: c_int = buf.len() as c_int;
        let mut bytesToAllocate_0: size_t = 0;
        // if blockSize_0 < 0 {
        //     /* This condition traps a situation where either more than
        //      * INT_MAX bytes have already been allocated (which is prevented
        //      * by various pieces of program logic, not least this one, never
        //      * mind the unlikelihood of actually having that much memory) or
        //      * the pool control fields have been corrupted (which could
        //      * conceivably happen in an extremely buggy user handler
        //      * function).  Either way it isn't readily testable, so we
        //      * exclude it from the coverage statistics.
        //      */
        //     return false;
        //     /* LCOV_EXCL_LINE */
        // }
        if blockSize_0 < INIT_BLOCK_SIZE {
            blockSize_0 = INIT_BLOCK_SIZE
        } else {
            /* Detect overflow, avoiding _signed_ overflow undefined behavior */
            if ((blockSize_0 as c_uint).wrapping_mul(2u32) as c_int) < 0 {
                return false;
            } /* save one level of indirection */
            blockSize_0 *= 2
        } /* save one level of indirection */
        bytesToAllocate_0 = poolBytesToAllocateFor(blockSize_0); /* save one level of indirection */

        if bytesToAllocate_0 == 0 {
            return false;
        }

        // REVIEW: Should this be additional bytes or bytes including
        // what's already allocated?
        // buf.reserve_exact(bytesToAllocate_0 as usize);

        if buf.try_reserve_exact(bytesToAllocate_0 as usize).is_err() {
            return false;
        };

        self.update_raw(&mut buf);

        true
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
    // use std::mem::size_of;

    let mut pool = StringPool::new().unwrap();

    assert!(pool.appendChar(A));
    assert_eq!(pool.get_bump_vec().as_slice(), [A]);
    // allocated_bytes seems to always return 0 for some reason
    // assert_eq!(pool.bump.allocated_bytes(), size_of::<XML_Char>());

    assert!(pool.appendChar(B));
    assert_eq!(pool.get_bump_vec().as_slice(), [A, B]);
    // assert_eq!(pool.bump.allocated_bytes(), size_of::<XML_Char>() * 2);

    // New BumpVec
    pool.resetCurrentBumpVec();

    assert!(pool.appendChar(C));
    assert_eq!(pool.get_bump_vec().as_slice(), [C]);

    // Check whole Bump buff
    // assert_eq!(pool.bump.allocated_bytes(), size_of::<XML_Char>() * 3);
}

#[test]
fn test_append_string() {
    use consts::*;

    let mut pool = StringPool::new().unwrap();
    let mut string = [A, B, C, NULL];

    unsafe {
        assert!(pool.appendString(string.as_mut_ptr()));
    }

    assert_eq!(pool.get_bump_vec().as_slice(), [A, B, C]);
}

#[test]
fn test_copy_string() {
    use consts::*;

    let mut pool = StringPool::new().unwrap();

    assert!(pool.appendChar(A));
    assert_eq!(pool.get_bump_vec().as_slice(), [A]);

    let new_string = unsafe {
        pool.copyString(S.as_ptr())
    };

    assert_eq!(new_string.unwrap(), [A, C, D, D, C, NULL]);
    assert!(pool.appendChar(B));
    assert_eq!(pool.get_bump_vec().as_slice(), [B]);

    let new_string2 = unsafe {
        pool.copyStringN(S.as_ptr(), 4)
    };

    assert_eq!(new_string2.unwrap(), [B, C, D, D, C]);
}

#[test]
fn test_store_string() {
    use consts::*;
    use crate::lib::xmlparse::XmlGetInternalEncoding;

    let mut pool = StringPool::new().unwrap();
    let enc = XmlGetInternalEncoding();
    let read_buf = unsafe {
        ExpatBufRef::new(S.as_ptr(), S.as_ptr().add(3))
    };
    let string = unsafe {
        pool.storeString(enc, read_buf).unwrap()
    };

    assert_eq!(pool.bump.allocated_bytes(), 1036);
    assert_eq!(&*string, &[C, D, D, NULL]);
    assert!(pool.appendChar(A));
    assert_eq!(pool.current_slice(), [A]);
    assert_eq!(pool.bump.allocated_bytes(), 2072);

    // No overlap between buffers:
    assert_eq!(&*string, &[C, D, D, NULL]);
}
