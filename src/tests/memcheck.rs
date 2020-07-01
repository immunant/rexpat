/* Debug allocators for the Expat test suite
                            __  __            _
                         ___\ \/ /_ __   __ _| |_
                        / _ \\  /| '_ \ / _` | __|
                       |  __//  \| |_) | (_| | |_
                        \___/_/\_\ .__/ \__,_|\__|
                                 |_| XML parser

   Copyright (c) 1997-2000 Thai Open Source Software Center Ltd
   Copyright (c) 2000-2017 Expat development team
   Portions copyright (c) 2020 Immunant, Inc.
   Licensed under the MIT license:

   Permission is  hereby granted,  free of charge,  to any  person obtaining
   a  copy  of  this  software   and  associated  documentation  files  (the
   "Software"),  to  deal in  the  Software  without restriction,  including
   without  limitation the  rights  to use,  copy,  modify, merge,  publish,
   distribute, sublicense, and/or sell copies of the Software, and to permit
   persons  to whom  the Software  is  furnished to  do so,  subject to  the
   following conditions:

   The above copyright  notice and this permission notice  shall be included
   in all copies or substantial portions of the Software.

   THE  SOFTWARE  IS  PROVIDED  "AS  IS",  WITHOUT  WARRANTY  OF  ANY  KIND,
   EXPRESS  OR IMPLIED,  INCLUDING  BUT  NOT LIMITED  TO  THE WARRANTIES  OF
   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN
   NO EVENT SHALL THE AUTHORS OR  COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
   DAMAGES OR  OTHER LIABILITY, WHETHER  IN AN  ACTION OF CONTRACT,  TORT OR
   OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
   USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use ::libc::{self, free, printf, malloc, realloc};
use libc::{c_char, c_int, c_void, size_t};
/* Structures to keep track of what has been allocated.  Speed isn't a
 * big issue for the tests this is required for, so we will use a
 * doubly-linked list to make deletion easier.
 */

pub type AllocationEntry = allocation_entry;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocation_entry {
    pub next: *mut allocation_entry,
    pub prev: *mut allocation_entry,
    pub allocation: *mut c_void,
    pub num_bytes: size_t,
}

static mut alloc_head: *mut AllocationEntry = std::ptr::null_mut();

static mut alloc_tail: *mut AllocationEntry = std::ptr::null_mut();
/* Allocation declarations */
/* Allocate some memory and keep track of it. */
#[no_mangle]

pub unsafe extern "C" fn tracking_malloc(mut size: size_t) -> *mut c_void {
    let mut entry: *mut AllocationEntry =
        malloc(::std::mem::size_of::<AllocationEntry>()) as *mut AllocationEntry;
    if entry.is_null() {
        printf(b"Allocator failure\n\x00".as_ptr() as *const c_char);
        return std::ptr::null_mut();
    }
    (*entry).num_bytes = size;
    (*entry).allocation = malloc(size);
    if (*entry).allocation.is_null() {
        free(entry as *mut c_void);
        return std::ptr::null_mut();
    }
    (*entry).next = std::ptr::null_mut();
    /* Add to the list of allocations */
    if alloc_head.is_null() {
        (*entry).prev = std::ptr::null_mut();
        alloc_tail = entry;
        alloc_head = alloc_tail
    } else {
        (*entry).prev = alloc_tail;
        (*alloc_tail).next = entry;
        alloc_tail = entry
    }
    return (*entry).allocation;
}

unsafe extern "C" fn find_allocation(mut ptr: *mut c_void) -> *mut AllocationEntry {
    let mut entry: *mut AllocationEntry = 0 as *mut AllocationEntry;
    entry = alloc_head;
    while !entry.is_null() {
        if (*entry).allocation == ptr {
            return entry;
        }
        entry = (*entry).next
    }
    return std::ptr::null_mut();
}
/* Free some memory and remove the tracking for it */
#[no_mangle]

pub unsafe extern "C" fn tracking_free(mut ptr: *mut c_void) {
    let mut entry: *mut AllocationEntry = 0 as *mut AllocationEntry;
    if ptr.is_null() {
        /* There won't be an entry for this */
        return;
    }
    entry = find_allocation(ptr);
    if !entry.is_null() {
        /* This is the relevant allocation.  Unlink it */
        if !(*entry).prev.is_null() {
            (*(*entry).prev).next = (*entry).next
        } else {
            alloc_head = (*entry).next
        }
        if !(*entry).next.is_null() {
            (*(*entry).next).prev = (*entry).prev
        } else {
            alloc_tail = (*entry).prev
        }
        free(entry as *mut c_void);
    } else {
        printf(
            b"Attempting to free unallocated memory at %p\n\x00".as_ptr() as *const c_char,
            ptr,
        );
    }
    free(ptr);
}
/* Reallocate some memory and keep track of it */
#[no_mangle]

pub unsafe extern "C" fn tracking_realloc(mut ptr: *mut c_void, mut size: size_t) -> *mut c_void {
    let mut entry: *mut AllocationEntry = 0 as *mut AllocationEntry;
    if ptr.is_null() {
        /* By definition, this is equivalent to malloc(size) */
        return tracking_malloc(size);
    }
    if size == 0 {
        /* By definition, this is equivalent to free(ptr) */
        tracking_free(ptr);
        return std::ptr::null_mut();
    }
    /* Find the allocation entry for this memory */
    entry = find_allocation(ptr);
    if entry.is_null() {
        printf(
            b"Attempting to realloc unallocated memory at %p\n\x00".as_ptr() as *const c_char,
            ptr,
        );
        entry = malloc(::std::mem::size_of::<AllocationEntry>()) as *mut AllocationEntry;
        if entry.is_null() {
            printf(b"Reallocator failure\n\x00".as_ptr() as *const c_char);
            return std::ptr::null_mut();
        }
        (*entry).allocation = realloc(ptr, size);
        if (*entry).allocation.is_null() {
            free(entry as *mut c_void);
            return std::ptr::null_mut();
        }
        /* Add to the list of allocations */
        (*entry).next = std::ptr::null_mut();
        if alloc_head.is_null() {
            (*entry).prev = std::ptr::null_mut();
            alloc_tail = entry;
            alloc_head = alloc_tail
        } else {
            (*entry).prev = alloc_tail;
            (*alloc_tail).next = entry;
            alloc_tail = entry
        }
    } else {
        (*entry).allocation = realloc(ptr, size);
        if (*entry).allocation.is_null() {
            /* Realloc semantics say the original is still allocated */
            (*entry).allocation = ptr;
            return std::ptr::null_mut();
        }
    }
    (*entry).num_bytes = size;
    return (*entry).allocation;
}
/* End-of-test check to see if unfreed allocations remain. Returns
 * TRUE (1) if there is nothing, otherwise prints a report of the
 * remaining allocations and returns FALSE (0).
 */
#[no_mangle]

pub unsafe extern "C" fn tracking_report() -> c_int {
    let mut entry: *mut AllocationEntry = 0 as *mut AllocationEntry;
    if alloc_head.is_null() {
        return 1i32;
    }
    /* Otherwise we have allocations that haven't been freed */
    entry = alloc_head;
    while !entry.is_null() {
        printf(
            b"Allocated %lu bytes at %p\n\x00".as_ptr() as *const c_char,
            (*entry).num_bytes,
            (*entry).allocation,
        );
        entry = (*entry).next
    }
    return 0;
}
