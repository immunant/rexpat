use crate::expat_external_h::XML_Char;
use crate::lib::xmlparse::{ExpatBufRef, ExpatBufRefMut, XmlConvert};
use crate::lib::xmltok::{ENCODING, XML_Convert_Result};

use bumpalo::Bump;
use bumpalo::collections::vec::Vec as BumpVec;
use fallible_collections::FallibleBox;
use libc::{INT_MAX, c_int, c_uint, c_ulong, size_t};

use std::cell::RefCell;
use std::convert::TryInto;
use std::mem::swap;

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
    let stretch = ::std::mem::size_of::<XML_Char>() as u64; /* can be 4 bytes */
    if blockSize <= 0 {
        return 0;
    }
    if blockSize > (INT_MAX as c_ulong).wrapping_div(stretch) as c_int {
        return 0;
    }
    let stretchedBlockSize: c_int = blockSize * stretch as c_int;
    let bytesToAllocate: c_int =
        (12 as c_ulong).wrapping_add(stretchedBlockSize as c_uint as c_ulong) as c_int;
    if bytesToAllocate < 0 {
        return 0;
    }
    bytesToAllocate as size_t
}

rental! {
    mod rental_pool {
        use super::*;

        #[rental(debug)]
        pub(crate) struct InnerStringPool {
            bump: Box<Bump>,
            current_bump_vec: RefCell<RentedBumpVec<'bump>>,
        }
    }
}

use rental_pool::InnerStringPool;

/// A StringPool has the purpose of allocating distinct strings and then
/// handing them off to be referenced either temporarily or for the entire length
/// of the pool.
pub(crate) struct StringPool(Option<InnerStringPool>);

impl StringPool {
    pub(crate) fn try_new() -> Result<Self, ()> {
        let bump = Bump::try_new().map_err(|_| ())?;
        let boxed_bump = Box::try_new(bump).map_err(|_| ())?;

        Ok(StringPool(Some(InnerStringPool::new(
            boxed_bump,
            |bump| RefCell::new(RentedBumpVec(BumpVec::new_in(&bump))),
        ))))
    }

    /// # Safety
    ///
    /// The inner type is only ever None in middle of the clear()
    /// method. Therefore it is safe to use anywhere else.
    fn inner(&self) -> &InnerStringPool {
        self.0.as_ref().unwrap_or_else(|| unsafe {
            std::hint::unreachable_unchecked()
        })
    }

    /// Determines whether or not the current BumpVec is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.inner().rent(|vec| vec.borrow().0.is_empty())
    }

    /// Determines whether or not the current BumpVec is full.
    pub(crate) fn is_full(&self) -> bool {
        self.inner().rent(|vec| vec.borrow().is_full())
    }

    /// Gets the current vec, converts it into a slice, and resets
    /// bookkeeping so that it will create a new vec next time.
    pub(crate) fn finish_string(&self) -> &mut [XML_Char] {
        /// # Safety
        ///
        /// Rental provides `ref_rent()`, which would do what we want here,
        /// except that we need to return a mutable reference with the string
        /// pool's lifetime, not an immutable one. We can't use
        /// `ref_rent_mut()`, which return a mutable reference, because we don't
        /// have a mutable reference to the rental itself.
        ///
        /// This borrows the inner pool with a new, anonymous lifetime, allowing
        /// us to return a mutable slice with the same lifetime as self. The
        /// lifetime of this reference is valid because the Bump will be valid
        /// for the lifetime of self, and mutability is allowed because mutating
        /// a finalized string can never affect any other string in the pool.
        let pool = unsafe { self.inner().all_erased() };
        let mut vec = RentedBumpVec(BumpVec::new_in(&pool.bump));
        pool.current_bump_vec.replace(vec).0.into_bump_slice_mut()
    }

    /// Resets the current bump vec to the beginning
    pub(crate) fn clear_current(&self) {
        self.inner().rent(|v| v.borrow_mut().0.clear())
    }

    /// Obtains the length of the current BumpVec.
    pub(crate) fn len(&self) -> usize {
        self.inner().rent(|vec| vec.borrow().0.len())
    }

    /// Call callback with an immutable buffer of the current BumpVec. This must
    /// be a callback to ensure that we don't (safely) borrow the slice for
    /// longer than it stays vaild.
    pub(crate) fn current_slice<F, R>(&self, mut callback: F) -> R
        where F: FnMut(&[XML_Char]) -> R
    {
        self.inner().rent(|v| callback(v.borrow().0.as_slice()))
    }

    /// Call callback with a mutable buffer of the current BumpVec. This must
    /// be a callback to ensure that we don't (safely) borrow the slice for
    /// longer than it stays vaild.
    pub(crate) fn current_mut_slice<F, R>(&self, mut callback: F) -> R
        where F: FnMut(&mut [XML_Char]) -> R
    {
        self.inner().rent(|v| callback(v.borrow_mut().0.as_mut_slice()))
    }

    /// Unsafe temporary version of `current_slice()`. This needs to be removed
    /// when callers are made safe.
    pub(crate) unsafe fn current_start(&self) -> *const XML_Char {
        self.inner().rent(|v| v.borrow().0.as_ptr())
    }

    /// Appends a char to the current BumpVec.
    pub(crate) fn append_char(&self, c: XML_Char) -> bool {
        self.inner().rent(|vec| vec.borrow_mut().append_char(c))
    }

    /// Overwrites the last char in the current BumpVec.
    /// Note that this will panic if empty. This is not an insert
    /// operation as it does not shift bytes afterwards.
    pub(crate) fn replace_last_char(&self, c: XML_Char) {
        self.inner().rent(|buf| {
            *buf.borrow_mut()
                .0
                .last_mut()
                .expect("Called replace_last_char() when string was empty") = c;
        })
    }

    /// Decrements the length, panicing if len is 0
    pub(crate) fn backtrack(&self) {
        self.inner().rent(|vec| vec.borrow_mut().0.pop().expect("Called backtrack() on empty BumpVec"));
    }

    /// Gets the last character, panicing if len is 0
    pub(crate) fn get_last_char(&self) -> XML_Char {
        self.inner().rent(|buf| *buf.borrow().0.last().expect("Called get_last_char() when string was empty"))
    }

    /// Appends an entire C String to the current BumpVec.
    pub(crate) unsafe fn append_c_string(&self, mut s: *const XML_Char) -> bool {
        self.inner().rent(|vec| {
            let mut vec = vec.borrow_mut();

            while *s != 0 {
                if !vec.append_char(*s) {
                    return false;
                }
                s = s.offset(1)
            }
            true
        })
    }

    /// Resets the current Bump and deallocates its contents.
    /// The `inner` method must never be called here as it assumes
    /// self.0 is never `None`
    pub(crate) fn clear(&mut self) {
        let mut inner_pool = self.0.take();

        let mut bump = inner_pool.unwrap().into_head();

        bump.reset();

        inner_pool = Some(InnerStringPool::new(
            bump,
            |bump| RefCell::new(RentedBumpVec(BumpVec::new_in(&bump))),
        ));

        swap(&mut self.0, &mut inner_pool);
    }

    pub(crate) fn store_c_string(
        &self,
        enc: &ENCODING,
        buf: ExpatBufRef,
    ) -> bool {
        self.inner().rent(|vec| {
            let mut vec = vec.borrow_mut();

            if !vec.append(enc, buf) {
                return false;
            }
            if !vec.append_char('\0' as XML_Char) {
                return false;
            }

            true
        })
    }

    pub(crate) fn append(
        &self,
        enc: &ENCODING,
        read_buf: ExpatBufRef,
    ) -> bool {
        self.inner().rent(|vec| vec.borrow_mut().append(enc, read_buf))
    }

    pub(crate) unsafe fn copy_c_string(
        &self,
        mut s: *const XML_Char,
    ) -> Option<&mut [XML_Char]> {
        // self.append_c_string(s);?
        let successful = self.inner().rent(|vec| {
            let mut vec = vec.borrow_mut();

            loop {
                if !vec.append_char(*s) {
                    return false;
                }
                if *s == 0 {
                    break;
                }
                s = s.offset(1);
            }

            true
        });

        if !successful {
            return None;
        }

        Some(self.finish_string())
    }

    pub(crate) unsafe fn copy_c_string_n(
        &self,
        mut s: *const XML_Char,
        mut n: c_int,
    ) -> Option<&[XML_Char]> {
        let successful = self.inner().rent(|vec| {
            let mut vec = vec.borrow_mut();

            while n > 0 {
                if !vec.append_char(*s) {
                    return false;
                }
                n -= 1;
                s = s.offset(1)
            }

            true
        });

        if !successful {
            return None;
        }

        Some(&*self.finish_string())
    }

    pub(crate) fn grow(&self) -> bool {
        self.inner().rent(|vec| vec.borrow_mut().grow())
    }
}

#[derive(Debug)]
pub(crate) struct RentedBumpVec<'bump>(BumpVec<'bump, XML_Char>);

impl<'bump> RentedBumpVec<'bump> {
    fn is_full(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    fn grow(&mut self) -> bool {
        let mut blockSize = self.0.capacity();
        let mut bytesToAllocate: size_t = 0;
        // if blockSize < 0 {
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
        blockSize = if blockSize < INIT_BLOCK_SIZE {
            INIT_BLOCK_SIZE
        } else {
            /* Detect overflow, avoiding _signed_ overflow undefined behavior */
            match blockSize.checked_mul(2) {
                Some(size) => size,
                None => return false,
            }
        };
        bytesToAllocate = poolBytesToAllocateFor(blockSize.try_into().unwrap());

        if bytesToAllocate == 0 {
            return false;
        }

        if self.0.try_reserve_exact(bytesToAllocate as usize).is_err() {
            return false;
        };

        true
    }

    fn append(
        &mut self,
        enc: &ENCODING,
        mut read_buf: ExpatBufRef,
    ) -> bool {
        let start = self.0.as_ptr();

        // REVIEW: Can this be replaced with self.is_empty() &&?
        if start.is_null() && !self.grow() {
            return false;
        }

        let mut cap = self.0.capacity();
        let mut len = self.0.len();

        loop {
            debug_assert!(len <= cap);

            // Continue to allocate if we don't have enough space
            while (cap - len) < read_buf.len() {
                if !self.grow() {
                    return false;
                }

                cap = self.0.capacity();
            }

            let start_len = self.0.len();
            let cap = self.0.capacity();

            self.0.resize(cap, 0);

            let mut write_buf = unsafe { ExpatBufRefMut::new_len(self.0.as_mut_ptr().add(len), cap - start_len) };
            let write_buf_len = write_buf.len();
            let convert_res = unsafe {
                XmlConvert!(enc, &mut read_buf, &mut write_buf)
            };
            // The write buf shrinks by how much was written to it
            let written_size = write_buf_len - write_buf.len();

            self.0.truncate(start_len + written_size);

            if convert_res == XML_Convert_Result::COMPLETED || convert_res == XML_Convert_Result::INPUT_INCOMPLETE {
                break;
            }
        }

        true
    }

    fn append_char(&mut self, c: XML_Char) -> bool {
        if self.is_full() && !self.grow() {
            false
        } else {
            self.0.push(c);

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

    let mut pool = StringPool::try_new().unwrap();

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

    let mut pool = StringPool::try_new().unwrap();
    let mut string = [A, B, C, NULL];

    unsafe {
        assert!(pool.append_c_string(string.as_mut_ptr()));
    }

    pool.current_slice(|s| assert_eq!(s, [A, B, C]));
}

#[test]
fn test_copy_string() {
    use consts::*;

    let mut pool = StringPool::try_new().unwrap();

    assert!(pool.append_char(A));
    pool.current_slice(|s| assert_eq!(s, [A]));

    let new_string = unsafe {
        pool.copy_c_string(S.as_ptr())
    };

    assert_eq!(new_string.unwrap(), [A, C, D, D, C, NULL]);
    assert!(pool.append_char(B));
    pool.current_slice(|s| assert_eq!(s, [B]));

    let new_string2 = unsafe {
        pool.copy_c_string_n(S.as_ptr(), 4)
    };

    assert_eq!(new_string2.unwrap(), [B, C, D, D, C]);
}

#[test]
fn test_store_c_string() {
    use consts::*;
    use crate::lib::xmlparse::XmlGetInternalEncoding;

    let mut pool = StringPool::try_new().unwrap();
    let enc = XmlGetInternalEncoding();
    let read_buf = unsafe {
        ExpatBufRef::new(S.as_ptr(), S.as_ptr().add(3))
    };
    assert!(pool.store_c_string(enc, read_buf));

    let string = pool.finish_string();

    assert_eq!(pool.inner().head().allocated_bytes(), 1036);
    assert_eq!(&*string, &[C, D, D, NULL]);
    assert!(pool.append_char(A));
    pool.current_slice(|s| assert_eq!(s, [A]));
    assert_eq!(pool.inner().head().allocated_bytes(), 2072);

    // No overlap between buffers:
    assert_eq!(&*string, &[C, D, D, NULL]);

    assert!(pool.append_char(A));

    pool.current_slice(|s| assert_eq!(s, [A, A]));

    // Force reallocation:
    pool.inner().rent(|v| v.borrow_mut().0.resize(2, 0));

    assert!(pool.store_c_string(enc, read_buf));

    let s = pool.finish_string();

    assert_eq!(s, [A, A, C, D, D, NULL]);
}
