use crate::expat_external_h::XML_Char;
use crate::lib::xmlparse::{ExpatBufRef, ExpatBufRefMut, XmlConvert};
use crate::lib::xmltok::{ENCODING, XML_Convert_Result};

use bumpalo::Bump;
use bumpalo::collections::vec::Vec as BumpVec;
use crate::fallible_rc::FallibleRc;
use fallible_collections::FallibleBox;
use libc::c_int;
use owning_ref::{OwningRef, RcRef};

use std::cell::{Cell, RefCell};
use std::convert::TryInto;
use std::rc::Rc;

pub(crate) type StringPoolPtr = Rc<Option<InnerStringPool>>;
pub(crate) type StringPoolSlice = OwningRef<StringPoolPtr, [XML_Char]>;
pub(crate) type StringPoolCellSlice = OwningRef<StringPoolPtr, [Cell<XML_Char>]>;

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

rental! {
    mod rental_pool {
        use super::*;

        #[rental(debug)]
        pub(crate) struct InnerStringPool {
            // The rental crate requires that all fields but the last one
            // implement `StableDeref`, which means we need to box it
            bump: Box<Bump>,
            current_bump_vec: RefCell<RentedBumpVec<'bump>>,
        }
    }
}

use rental_pool::InnerStringPool;

/// A StringPool has the purpose of allocating distinct strings and then
/// handing them off to be referenced either temporarily or for the entire length
/// of the pool.
pub(crate) struct StringPool(StringPoolPtr);

impl StringPool {
    pub(crate) fn try_new() -> Result<Self, ()> {
        let bump = Bump::try_with_capacity(INIT_BLOCK_SIZE).map_err(|_| ())?;
        let boxed_bump = Box::try_new(bump).map_err(|_| ())?;

        let pool = InnerStringPool::new(
            boxed_bump,
            |bump| RefCell::new(RentedBumpVec(BumpVec::new_in(&bump))),
        );
        let rc_pool = Rc::try_new(Some(pool)).map_err(|_| ())?;

        Ok(StringPool(rc_pool))
    }

    /// # Safety
    ///
    /// The inner type is only ever None in middle of the clear()
    /// method. Therefore it is safe to use anywhere else.
    fn inner(&self) -> &InnerStringPool {
        (*self.0).as_ref().unwrap_or_else(|| unsafe {
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

    /// Gets the current vec, converts it into an immutable slice,
    /// and resets bookkeeping so that it will create a new vec next time.
    pub(crate) fn finish(&self) -> StringPoolSlice {
        RcRef::new(Rc::clone(&self.0)).map(|inner| {
            inner.as_ref().unwrap().ref_rent_all(|pool| {
                let mut vec = RentedBumpVec(BumpVec::new_in(&pool.bump));
                pool.current_bump_vec.replace(vec).0.into_bump_slice()
            })
        })
    }

    /// Gets the current vec, converts it into a slice of cells (with interior mutability),
    /// and resets bookkeeping so that it will create a new vec next time.
    pub(crate) fn finish_cells(&self) -> StringPoolCellSlice {
        RcRef::new(Rc::clone(&self.0)).map(|inner| {
            inner.as_ref().unwrap().ref_rent_all(|pool| {
                let mut vec = RentedBumpVec(BumpVec::new_in(&pool.bump));
                let sl = pool.current_bump_vec.replace(vec).0.into_bump_slice_mut();
                Cell::from_mut(sl).as_slice_of_cells()
            })
        })
    }

    /// Resets the current bump vec to the beginning
    pub(crate) fn discard(&self) {
        self.inner().rent(|v| v.borrow_mut().0.clear())
    }

    /// Obtains the length of the current BumpVec.
    pub(crate) fn len(&self) -> usize {
        self.inner().rent(|vec| vec.borrow().0.len())
    }

    /// Call callback with an immutable buffer of the current BumpVec. This must
    /// be a callback to ensure that we don't (safely) borrow the slice for
    /// longer than it stays vaild.
    pub(crate) fn with_current_slice<F, R>(&self, mut callback: F) -> R
        where F: FnMut(&[XML_Char]) -> R
    {
        self.inner().rent(|v| callback(v.borrow().0.as_slice()))
    }

    /// Call callback with a mutable buffer of the current BumpVec. This must
    /// be a callback to ensure that we don't (safely) borrow the slice for
    /// longer than it stays vaild.
    pub(crate) fn with_current_mut_slice<F, R>(&self, mut callback: F) -> R
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
        let pool_ref = Rc::get_mut(&mut self.0).unwrap();

        let mut bump = pool_ref.take().unwrap().into_head();
        bump.reset();

        *pool_ref = Some(InnerStringPool::new(
            bump,
            |bump| RefCell::new(RentedBumpVec(BumpVec::new_in(&bump))),
        ));
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
    ) -> Option<StringPoolSlice> {
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

        Some(self.finish())
    }

    pub(crate) unsafe fn copy_c_string_n(
        &self,
        mut s: *const XML_Char,
        mut n: c_int,
    ) -> Option<StringPoolSlice> {
        let successful = self.inner().rent(|vec| {
            let mut vec = vec.borrow_mut();
            let mut n = n.try_into().unwrap();

            if vec.0.try_reserve_exact(n).is_err() {
                return false;
            };

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

        Some(self.finish())
    }
}

#[derive(Debug)]
pub(crate) struct RentedBumpVec<'bump>(BumpVec<'bump, XML_Char>);

impl<'bump> RentedBumpVec<'bump> {
    fn is_full(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    fn append<'a>(
        &mut self,
        enc: &ENCODING,
        mut read_buf: ExpatBufRef<'a>,
    ) -> bool {
        loop {
            // REXPAT: always reserve at least 4 bytes,
            // so at least one character gets converted every iteration
            if self.0.try_reserve(read_buf.len().max(4)).is_err() {
                return false;
            }

            let start_len = self.0.len();
            let cap = self.0.capacity();

            self.0.resize(cap, 0);

            let mut write_buf = ExpatBufRefMut::from(&mut self.0[start_len..]);
            let write_buf_len = write_buf.len();
            let convert_res = XmlConvert!(enc, &mut read_buf, &mut write_buf);
            // The write buf shrinks by how much was written to it
            let written_size = write_buf_len - write_buf.len();

            self.0.truncate(start_len + written_size);

            if convert_res == XML_Convert_Result::COMPLETED || convert_res == XML_Convert_Result::INPUT_INCOMPLETE {
                return true;
            }
        }
    }

    fn append_char(&mut self, c: XML_Char) -> bool {
        if self.0.try_reserve(1).is_err() {
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
    pool.with_current_slice(|s| assert_eq!(s, [A]));

    assert!(pool.append_char(B));
    pool.with_current_slice(|s| assert_eq!(s, [A, B]));

    // New BumpVec
    pool.finish();

    assert!(pool.append_char(C));
    pool.with_current_slice(|s| assert_eq!(s, [C]));
}

#[test]
fn test_append_string() {
    use consts::*;

    let mut pool = StringPool::try_new().unwrap();
    let mut string = [A, B, C, NULL];

    unsafe {
        assert!(pool.append_c_string(string.as_mut_ptr()));
    }

    pool.with_current_slice(|s| assert_eq!(s, [A, B, C]));
}

#[test]
fn test_copy_string() {
    use consts::*;

    let mut pool = StringPool::try_new().unwrap();

    assert!(pool.append_char(A));
    pool.with_current_slice(|s| assert_eq!(s, [A]));

    let new_string = unsafe {
        pool.copy_c_string(S.as_ptr())
    };

    assert_eq!(*new_string.unwrap(), [A, C, D, D, C, NULL]);
    assert!(pool.append_char(B));
    pool.with_current_slice(|s| assert_eq!(s, [B]));

    let new_string2 = unsafe {
        pool.copy_c_string_n(S.as_ptr(), 4)
    };

    assert_eq!(*new_string2.unwrap(), [B, C, D, D, C]);
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

    let string = pool.finish();

    assert_eq!(&*string, &[C, D, D, NULL]);
    assert!(pool.append_char(A));
    pool.with_current_slice(|s| assert_eq!(s, [A]));

    // No overlap between buffers:
    assert_eq!(&*string, &[C, D, D, NULL]);

    assert!(pool.append_char(A));

    pool.with_current_slice(|s| assert_eq!(s, [A, A]));

    // Force reallocation:
    pool.inner().rent(|v| v.borrow_mut().0.resize(2, 0));

    assert!(pool.store_c_string(enc, read_buf));

    let s = pool.finish();

    assert_eq!(*s, [A, A, C, D, D, NULL]);
}
