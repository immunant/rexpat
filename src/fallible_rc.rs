use std::alloc::{AllocRef, Global, Layout};
use std::cell::Cell;
use std::collections::TryReserveError;
use std::rc::Rc;

use fallible_collections::FallibleBox;

pub trait FallibleRc<T> {
    fn try_new(value: T) -> Result<Rc<T>, TryReserveError>;
}

struct RcBox<T: ?Sized> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
}

impl<T> FallibleRc<T> for Rc<T> {
    fn try_new(value: T) -> Result<Self, TryReserveError> {
        let inner = RcBox {
            strong: Cell::new(1),
            weak: Cell::new(1),
            value,
        };
        // FIXME: the Error types should match, but for some reason
        // it now returns the one from the hashbrown crate
        let b = Box::try_new(inner).map_err(|_| TryReserveError::CapacityOverflow)?;
        unsafe {
            // WARNING: really unsafe!!!
            Ok(std::mem::transmute(b))
        }
    }
}

// TODO: we should have a default implementation for T: Clone,
// and an override for T: Copy
pub trait FallibleRcSlice<T: Copy> {
    fn try_from_slice(slice: &[T]) -> Result<Rc<[T]>, TryReserveError>;
}

// FIXME: this is a fallible reimplementation of `From<&[T]> for Rc<[T]>`,
// until either `fallible_collections` or libstd get this feature
impl<T: Copy> FallibleRcSlice<T> for Rc<[T]> {
    fn try_from_slice(slice: &[T]) -> Result<Self, TryReserveError> {
        let slice_layout = Layout::for_value(slice);
        let box_layout = Layout::new::<RcBox<()>>()
            .extend(slice_layout)
            .map_err(|_| TryReserveError::CapacityOverflow)?
            .0
            .pad_to_align();

        unsafe {
            let ptr = Global.alloc(box_layout)
                .map_err(|_| TryReserveError::CapacityOverflow)?;
            let slice_ptr = std::ptr::slice_from_raw_parts_mut(
                ptr.0.as_ptr() as *mut T,
                slice.len(),
            ) as *mut RcBox<[T]>;

            std::ptr::write(&mut (*slice_ptr).strong, Cell::new(1));
            std::ptr::write(&mut (*slice_ptr).weak, Cell::new(1));
            std::ptr::copy_nonoverlapping(
                slice.as_ptr(),
                &mut (*slice_ptr).value as *mut [T] as *mut T,
                slice.len(),
            );

            Ok(std::mem::transmute(slice_ptr))
        }
    }
}
