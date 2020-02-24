use crate::expat_external_h::XML_Char;
use crate::lib::xmlparse::{ICHAR, ExpatBufRef, ExpatBufRefMut, XmlConvert};
use crate::lib::xmltok::{ENCODING, XML_CONVERT_INPUT_INCOMPLETE, XML_CONVERT_COMPLETED};
use crate::stddef_h::size_t;

use bumpalo::Bump;
use bumpalo::collections::vec::Vec as BumpVec;
use fallible_collections::FallibleBox;
use libc::{INT_MAX, c_int, c_uint, c_ulong};

use std::cell::RefCell;
use std::convert::TryInto;
use std::mem::swap;
use std::ptr;

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

rental! {
    mod rental_pool {
        use super::*;

        #[rental(debug)]
        pub(crate) struct InnerStringPool {
            bump: Box<Bump>,
            current_bump_vec: RefCell<BumpVec<'bump, XML_Char>>,
        }
    }
}

use rental_pool::InnerStringPool;

pub(crate) struct StringPool(Option<InnerStringPool>);

impl StringPool {
    pub(crate) fn try_new() -> Result<Self, ()> {
        let bump = Bump::try_new().map_err(|_| ())?;
        let boxed_bump = Box::try_new(bump).map_err(|_| ())?;

        Ok(StringPool(Some(InnerStringPool::new(
            boxed_bump,
            |bump| RefCell::new(BumpVec::new_in(&bump)),
        ))))
    }

    fn inner(&self) -> &InnerStringPool {
        self.0.as_ref().unwrap()
    }

    /// Determines whether or not the current BumpVec is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.inner().rent(|vec| vec.borrow().is_empty())
    }

    /// Determines whether or not the current BumpVec is full.
    pub(crate) fn is_full(&self) -> bool {
        self.inner().rent(|vec| vec.borrow().len() == vec.borrow().capacity())
    }

    /// Gets the current vec, converts it into a slice, and resets
    /// bookkeeping so that it will create a new vec next time.
    pub(crate) fn consume_current_vec(&self) -> &mut [XML_Char] {
        let pool = unsafe { self.inner().all_erased() };
        let mut vec = BumpVec::new_in(&pool.bump);
        pool.current_bump_vec.replace(vec).into_bump_slice_mut()
    }

    /// Resets the current bump vec to the beginning
    pub(crate) fn clear_current(&self) {
        self.set_len(0)
    }

    /// Moves the start pointer to the current length so that a new bump vec region begins
    /// REVIEW: Maybe shouldn't be necessary or just force a new vec creation..?
    pub(crate) fn finish_current(&self) {
        self.consume_current_vec();
    }

    // REVIEW: Not sure if necessary
    pub(crate) fn finish_string(&self) -> &mut [XML_Char] {
        self.consume_current_vec()
    }

    /// Obtains the length of the current BumpVec.
    pub(crate) fn len(&self) -> usize {
        self.inner().rent(|vec| vec.borrow().len())
    }

    /// Call callback with an immutable buffer of the current BumpVec. This must
    /// be a callback to ensure that we don't (safely) borrow the slice for
    /// longer than it stays vaild.
    pub(crate) fn current_slice<F, R>(&self, mut callback: F) -> R
        where F: FnMut(&[XML_Char]) -> R
    {
        self.inner().rent(|v| callback(v.borrow().as_slice()))
    }

    /// Unsafe temporary version of `current_slice()`. This needs to be removed
    /// when callers are made safe.
    pub(crate) unsafe fn current_start(&self) -> *const XML_Char {
        self.inner().rent(|v| v.borrow().as_ptr())
    }

    /// Appends a char to the current BumpVec.
    pub(crate) fn appendChar(&self, c: XML_Char) -> bool {
        if self.is_full() && !self.grow() {
            false
        } else {
            self.inner().rent(|buf| buf.borrow_mut().push(c));

            true
        }
    }

    /// Overwrites the last char in the current BumpVec.
    /// Note that this will panic if empty and that this is not an insert
    /// operation as it does not shift bytes afterwards.
    pub(crate) fn prepend_char(&self, c: XML_Char) {
        self.inner().rent(|buf| {
            let len = buf.borrow().len();

            buf.borrow_mut()[len - 1] = c;
        })
    }

    /// Decrements the length, panicing if len is 0 in debug
    pub(crate) fn backtrack(&self) {
        self.inner().rent(|vec| vec.borrow_mut().pop().expect("Called backtrack() on empty BumpVec"));
    }

    /// Gets the last character, panicing if len is 0
    pub(crate) fn get_last_char(&self) -> XML_Char {
        self.inner().rent(|buf| buf.borrow_mut()[buf.borrow().len() - 1])
    }

    /// Appends an entire C String to the current BumpVec.
    pub(crate) unsafe fn appendString(&self, mut s: *const XML_Char) -> bool {
        while *s != 0 {
            if !self.appendChar(*s) {
                return false;
            }
            s = s.offset(1)
        }
        true
    }

    /// Resets the current Bump and deallocates its contents.
    pub(crate) fn clear(&mut self) {
        let mut inner_pool = None;

        swap(&mut self.0, &mut inner_pool);

        let mut bump = inner_pool.unwrap().into_head();

        bump.reset();

        inner_pool = Some(InnerStringPool::new(
            bump,
            |bump| RefCell::new(BumpVec::new_in(&bump)),
        ));

        swap(&mut self.0, &mut inner_pool);
    }

    pub(crate) fn storeString(
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

    /// Sets a new length on the current bump vec. This is always safe because:
    /// 1) We assert the length is never greater than the capacity.
    /// 2) We always zero the capacity buffer in grow(), so we can
    ///    never point to uninit data.
    fn set_len(&self, len: usize) {
        self.inner().rent(|vec| {
            assert!(len <= vec.borrow().capacity());

            unsafe {
                vec.borrow_mut().set_len(len);
            }
        })
    }

    pub(crate) fn append(
        &self,
        enc: &ENCODING,
        mut readBuf: ExpatBufRef,
    ) -> bool {
        let start = self.inner().rent(|vec| vec.borrow().as_ptr());

        // REVIEW: Can this be replaced with self.is_empty() &&?
        if start.is_null() && !self.grow() {
            return false;
        }

        let (mut start, mut cap, mut len) = self.inner().rent(|vec| {
            let start = vec.borrow_mut().as_mut_ptr();
            let cap = vec.borrow().capacity();
            let len = vec.borrow().len();
            (start, cap, len)
        });
        let mut cap_begin;
        let mut cap_end;

        loop {
            debug_assert!(len <= cap);

            // Continue to allocate if we don't have enough space
            while (cap - len) < readBuf.len() {
                if !self.grow() {
                    return false;
                }

                let (new_start, new_cap) = self.inner().rent(|vec| {
                    let new_start = vec.borrow_mut().as_mut_ptr();
                    let new_cap = vec.borrow().capacity();
                    (new_start, new_cap)
                });
                start = new_start;
                cap = new_cap;
            }

            // Should always be safe and inbounds to offset into the buffer
            unsafe {
                cap_begin = start.add(len);
                cap_end = start.add(cap) as *mut ICHAR;
            }

            // NOTE: We avoid UB when writing to the capacity via a mutable slice by zeroing the
            // capacity ahead of time in the grow() method
            let mut writeBuf = ExpatBufRefMut::new(cap_begin, cap_end);

            let convert_res = unsafe {
                XmlConvert!(
                    enc,
                    &mut readBuf,
                    &mut writeBuf,
                )
            };

            // TODO: How to not need wrapping_offset_from here? Cast to u/isize first?
            let new_len = writeBuf.as_ptr().wrapping_offset_from(start).try_into().unwrap();

            self.set_len(new_len);

            if convert_res == XML_CONVERT_COMPLETED || convert_res == XML_CONVERT_INPUT_INCOMPLETE {
                break;
            }

            // if !self.grow() {
            //     return false;
            // }

            len = new_len;
        }

        true
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
        self.inner().rent(|buf| {
            let mut blockSize_0 = buf.borrow().capacity();
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
            blockSize_0 = if blockSize_0 < INIT_BLOCK_SIZE {
                INIT_BLOCK_SIZE
            } else {
                /* Detect overflow, avoiding _signed_ overflow undefined behavior */
                match blockSize_0.checked_mul(2) {
                    Some(size) => size,
                    None => return false,
                }
            };
            bytesToAllocate_0 = poolBytesToAllocateFor(blockSize_0.try_into().unwrap());

            if bytesToAllocate_0 == 0 {
                return false;
            }

            if buf.borrow_mut().try_reserve_exact(bytesToAllocate_0 as usize).is_err() {
                return false;
            };

            // Zero capacity bytes
            let start = buf.borrow_mut().as_mut_ptr();

            // Safety: This is safe because we are writing to data we actually own,
            // and don't go out of bounds of the BumpVec's capactiy
            unsafe {
                let mut tmp = start.add(buf.borrow().len());
                let end = start.add(buf.borrow().capacity());

                while tmp != end {
                    ptr::write(tmp, 0);
                    tmp = tmp.add(1);
                }
            }

            true
        })
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

    let mut pool = StringPool::try_new().unwrap();

    assert!(pool.appendChar(A));
    pool.current_slice(|s| assert_eq!(s, [A]));

    assert!(pool.appendChar(B));
    pool.current_slice(|s| assert_eq!(s, [A, B]));

    // New BumpVec
    pool.finish_current();

    assert!(pool.appendChar(C));
    pool.current_slice(|s| assert_eq!(s, [C]));
}

#[test]
fn test_append_string() {
    use consts::*;

    let mut pool = StringPool::try_new().unwrap();
    let mut string = [A, B, C, NULL];

    unsafe {
        assert!(pool.appendString(string.as_mut_ptr()));
    }

    pool.current_slice(|s| assert_eq!(s, [A, B, C]));
}

#[test]
fn test_copy_string() {
    use consts::*;

    let mut pool = StringPool::try_new().unwrap();

    assert!(pool.appendChar(A));
    pool.current_slice(|s| assert_eq!(s, [A]));

    let new_string = unsafe {
        pool.copyString(S.as_ptr())
    };

    assert_eq!(new_string.unwrap(), [A, C, D, D, C, NULL]);
    assert!(pool.appendChar(B));
    pool.current_slice(|s| assert_eq!(s, [B]));

    let new_string2 = unsafe {
        pool.copyStringN(S.as_ptr(), 4)
    };

    assert_eq!(new_string2.unwrap(), [B, C, D, D, C]);
}

#[test]
fn test_store_string() {
    use consts::*;
    use crate::lib::xmlparse::XmlGetInternalEncoding;

    let mut pool = StringPool::try_new().unwrap();
    let enc = XmlGetInternalEncoding();
    let read_buf = unsafe {
        ExpatBufRef::new(S.as_ptr(), S.as_ptr().add(3))
    };
    let string = unsafe {
        pool.storeString(enc, read_buf).unwrap()
    };

    assert_eq!(pool.inner().head().allocated_bytes(), 1036);
    assert_eq!(&*string, &[C, D, D, NULL]);
    assert!(pool.appendChar(A));
    pool.current_slice(|s| assert_eq!(s, [A]));
    assert_eq!(pool.inner().head().allocated_bytes(), 2072);

    // No overlap between buffers:
    assert_eq!(&*string, &[C, D, D, NULL]);
}
