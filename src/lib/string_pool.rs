use crate::expat_external_h::XML_Char;
use crate::expat_h::{XML_Memory_Handling_Suite};
use crate::lib::xmlparse::{ICHAR, INIT_BLOCK_SIZE, XmlConvert};
use crate::lib::xmltok::{ENCODING, XML_CONVERT_INPUT_INCOMPLETE, XML_CONVERT_COMPLETED};
use crate::stddef_h::size_t;

use bumpalo::Bump;
use bumpalo::collections::vec::Vec as BumpVec;
use libc::{INT_MAX, c_char, c_int, c_uint, c_ulong};

use std::cell::Cell;
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

#[derive(Copy, Clone)]
struct RawBumpVec {
    start: *mut XML_Char,
    len: usize,
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

struct StringPool {
    bump: Bump,
    currentBumpVec: Cell<RawBumpVec>,
}

impl StringPool {
    pub(crate) fn new() -> Self {
        StringPool {
            bump: Bump::new(),
            currentBumpVec: Cell::new(RawBumpVec::new()),
        }
    }

    #[cfg(test)]
    fn resetCurrentBumpVec(&mut self) {
        self.currentBumpVec.set(RawBumpVec::new())
    }

    fn getBumpVec(&self) -> BumpVec<XML_Char> {
        // If we don't already have an existing bump vec, create a new one
        // otherwise regenerate previous one
        if self.currentBumpVec.get().start.is_null() {
            BumpVec::new_in(&self.bump)
        } else {
            let RawBumpVec { start, len, cap } = self.currentBumpVec.get();

            unsafe {
                BumpVec::from_raw_parts_in(start, len, cap, &self.bump)
            }
        }
    }

    fn updateRaw(&self, buf: &mut BumpVec<XML_Char>) {
        self.currentBumpVec.set(RawBumpVec {
            start: buf.as_mut_ptr(),
            len: buf.len(),
            cap: buf.capacity(),
        });
    }

    pub(crate) fn appendChar(&mut self, c: XML_Char) -> bool {
        let mut buf = self.getBumpVec();

        if buf.len() == buf.capacity() && !self.grow() {
            false
        } else {
            buf.push(c);

            self.updateRaw(&mut buf);

            true
        }
    }

    pub(crate) unsafe fn appendString(&mut self, mut s: *const XML_Char) -> bool {
        while *s != 0 {
            if !self.appendChar(*s) {
                return false;
            }
            s = s.offset(1)
        }
        !self.currentBumpVec.get().start.is_null()
    }

    pub(crate) fn init(&mut self, _ms: &XML_Memory_Handling_Suite) {
        *self = Self::new();
    }

    pub(crate) fn clear(&mut self) {
        *self = Self::new();
    }

    /// Replaced by drop?
    pub(crate) unsafe fn destroy(&mut self) {

    }

    // TODO: Wrap Vec so it cannot grow
    unsafe fn storeString(
        &mut self,
        enc: &ENCODING,
        ptr: *const c_char,
        end: *const c_char,
    ) -> Option<BumpVec<XML_Char>> {
        dbg!("Pre-append");
        if !self.append(enc, ptr, end) {
            dbg!("Post-append Ret");
            return None;
        }
        dbg!("Post-append");

        let mut buf = self.getBumpVec();
        let len = buf.len();
        let cap = buf.capacity();

        drop(buf);

        if len == cap && !self.grow() {
            return None;
        }

        self.appendChar('\0' as XML_Char);

        let vec = self.getBumpVec();

        self.currentBumpVec.set(RawBumpVec::new());

        Some(vec)
    }

    pub(crate) unsafe fn append(
        &mut self,
        enc: &ENCODING,
        mut ptr: *const c_char,
        end: *const c_char,
    ) -> bool {
        let RawBumpVec { mut start, .. } = self.currentBumpVec.get();

        if start.is_null() && !self.grow() {
            return false;
        }

        let RawBumpVec { mut start, cap, .. } = self.currentBumpVec.get();

        let mut ptr = &mut ptr as *mut _;
        let mut end2;
        let mut start2;
        let mut remaining_cap = 0;

        loop {
            end2 = start.add(cap) as *mut ICHAR;
            start2 = &mut start as *mut *mut _ as *mut *mut ICHAR;

            let convert_res = XmlConvert!(
                enc,
                ptr,
                end,
                start2,
                end2,
            );

            remaining_cap = (end2).wrapping_offset_from(*start2);

            debug_assert!(remaining_cap >= 0);

            // RMME: 2 == 0 || 2 == 1
            if convert_res == XML_CONVERT_COMPLETED || convert_res == XML_CONVERT_INPUT_INCOMPLETE {
                break;
            }

            if !self.grow() {
                return false;
            }
        }

        let mut buf = self.getBumpVec();
        let newly_written = cap - remaining_cap as usize;

        buf.set_len(buf.len() + newly_written);

        self.updateRaw(&mut buf);

        !self.currentBumpVec.get().start.is_null()
    }

    unsafe fn copyString(
        &mut self,
        mut s: *const XML_Char,
    ) -> Option<&[XML_Char]> {
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
        // self.appendString(s);

        let slice = self.getBumpVec().into_bump_slice();

        self.currentBumpVec.set(RawBumpVec::new());

        Some(slice)
    }

    unsafe fn copyStringN(
        &mut self,
        mut s: *const XML_Char,
        mut n: c_int,
    ) -> Option<&[XML_Char]> {
        let vec = self.getBumpVec();
        let is_empty = vec.is_empty();

        drop(vec);

        // REVIEW: Is this correct:
        // if self.ptr.is_null() && !self.grow() {
        if is_empty && !self.grow() {
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

        let slice = self.getBumpVec().into_bump_slice();

        self.currentBumpVec.set(RawBumpVec::new());

        Some(slice)
    }

    /// There's currently no try_push in Bumpalo, so can't determine if
    /// it's possible to allocate or not
    pub(crate) fn grow(&self) -> bool {
        let mut buf = self.getBumpVec();
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
        buf.reserve_exact(bytesToAllocate_0 as usize);

        dbg!((buf.as_ptr(), buf.len(), buf.capacity()));

        self.updateRaw(&mut buf);

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

    let mut pool = StringPool::new();

    assert!(pool.appendChar(A));
    assert_eq!(pool.getBumpVec().as_slice(), [A]);
    // allocated_bytes seems to always return 0 for some reason
    // assert_eq!(pool.bump.allocated_bytes(), size_of::<XML_Char>());

    assert!(pool.appendChar(B));
    assert_eq!(pool.getBumpVec().as_slice(), [A, B]);
    // assert_eq!(pool.bump.allocated_bytes(), size_of::<XML_Char>() * 2);

    // New BumpVec
    pool.resetCurrentBumpVec();

    assert!(pool.appendChar(C));
    assert_eq!(pool.getBumpVec().as_slice(), [C]);

    // Check whole Bump buff
    // assert_eq!(pool.bump.allocated_bytes(), size_of::<XML_Char>() * 3);
}

#[test]
fn test_append_string() {
    use consts::*;

    let mut pool = StringPool::new();
    let mut string = [A, B, C, NULL];

    unsafe {
        assert!(pool.appendString(string.as_mut_ptr()));
    }

    assert_eq!(pool.getBumpVec().as_slice(), [A, B, C]);
}

#[test]
fn test_copy_string() {
    use consts::*;

    let mut pool = StringPool::new();

    assert!(pool.appendChar(A));
    assert_eq!(pool.getBumpVec().as_slice(), [A]);

    let new_string = unsafe {
        pool.copyString(S.as_ptr())
    };

    assert_eq!(new_string.unwrap(), [A, C, D, D, C, NULL]);
    assert!(pool.appendChar(B));
    assert_eq!(pool.getBumpVec().as_slice(), [B]);

    let new_string2 = unsafe {
        pool.copyStringN(S.as_ptr(), 4)
    };

    assert_eq!(new_string2.unwrap(), [B, C, D, D, C]);
}

#[test]
fn test_store_string() {
    use consts::*;
    use crate::lib::xmlparse::XmlGetInternalEncoding;

    let mut pool = StringPool::new();
    let enc = XmlGetInternalEncoding();

    let string = unsafe {
        pool.storeString(enc, S.as_ptr(), S.as_ptr().add(3))
    };

    assert_eq!(&*string.unwrap(), &[C, D, D, NULL]);
    assert!(pool.appendChar(A));
    assert_eq!(pool.getBumpVec().as_slice(), [A]);
}
