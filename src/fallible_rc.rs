use std::cell::Cell;
use std::collections::TryReserveError;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

use fallible_collections::FallibleBox;

struct RcInner<T> {
    rc: Cell<usize>,
    value: T,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    marker: PhantomData<RcInner<T>>,
}

impl<T: ?Sized> !Send for Rc<T> {}
impl<T: ?Sized> !Sync for Rc<T> {}

impl<T> Rc<T> {
    pub fn try_new(value: T) -> Result<Self, TryReserveError> {
        let inner = RcInner {
            rc: Cell::new(1),
            value,
        };
        // FIXME: the Error types should match, but for some reason
        // it now returns the one from the hashbrown crate
        let b = Box::try_new(inner).map_err(|_| TryReserveError::CapacityOverflow)?;
        match NonNull::new(Box::into_raw(b)) {
            Some(inner) => Ok(Rc { inner, marker: PhantomData }),
            // FIXME: error should be different
            None => Err(TryReserveError::CapacityOverflow)
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        let inner = unsafe { self.inner.as_ref() };
        if inner.rc.get() == 1 {
            unsafe {
                Some(&mut self.inner.as_mut().value)
            }
        } else {
            None
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let old_rc = inner.rc.get();
        assert!(old_rc > 0 && old_rc < usize::max_value());
        inner.rc.set(old_rc + 1);
        Rc {
            inner: self.inner,
            marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let old_rc = inner.rc.get();
        assert!(old_rc > 0);
        inner.rc.set(old_rc - 1);
        if old_rc == 1 {
            unsafe {
                drop(Box::from_raw(self.inner.as_ptr()));
            }
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let inner = unsafe { self.inner.as_ref() };
        assert!(inner.rc.get() > 0);
        &inner.value
    }
}
