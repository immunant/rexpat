pub use crate::stddef_h::{size_t, NULL};

use ::libc::{self};
/* Debug allocators for the Expat test suite
                            __  __            _
                         ___\ \/ /_ __   __ _| |_
                        / _ \\  /| '_ \ / _` | __|
                       |  __//  \| |_) | (_| | |_
                        \___/_/\_\ .__/ \__,_|\__|
                                 |_| XML parser

   Copyright (c) 1997-2000 Thai Open Source Software Center Ltd
   Copyright (c) 2000-2017 Expat development team
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
    pub allocation: *mut libc::c_void,
    pub num_bytes: crate::stddef_h::size_t,
}

static mut alloc_head: *mut AllocationEntry = crate::stddef_h::NULL as *mut AllocationEntry;

static mut alloc_tail: *mut AllocationEntry = crate::stddef_h::NULL as *mut AllocationEntry;
/* Interface to allocation functions that will track what has or has
   not been freed.
                            __  __            _
                         ___\ \/ /_ __   __ _| |_
                        / _ \\  /| '_ \ / _` | __|
                       |  __//  \| |_) | (_| | |_
                        \___/_/\_\ .__/ \__,_|\__|
                                 |_| XML parser

   Copyright (c) 1997-2000 Thai Open Source Software Center Ltd
   Copyright (c) 2000-2017 Expat development team
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
/* Allocation declarations */
/* Allocate some memory and keep track of it. */
#[no_mangle]

pub unsafe extern "C" fn tracking_malloc(mut size: crate::stddef_h::size_t) -> *mut libc::c_void {
    let mut entry: *mut AllocationEntry =
        crate::stdlib::malloc(::std::mem::size_of::<AllocationEntry>() as libc::c_ulong)
            as *mut AllocationEntry;
    if entry.is_null() {
        ::libc::printf(b"Allocator failure\n\x00" as *const u8 as *const libc::c_char);
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    (*entry).num_bytes = size;
    (*entry).allocation = crate::stdlib::malloc(size);
    if (*entry).allocation.is_null() {
        ::libc::free(entry as *mut libc::c_void);
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    (*entry).next = crate::stddef_h::NULL as *mut allocation_entry;
    /* Add to the list of allocations */
    if alloc_head.is_null() {
        (*entry).prev = crate::stddef_h::NULL as *mut allocation_entry;
        alloc_tail = entry;
        alloc_head = alloc_tail
    } else {
        (*entry).prev = alloc_tail;
        (*alloc_tail).next = entry;
        alloc_tail = entry
    }
    return (*entry).allocation;
}

unsafe extern "C" fn find_allocation(mut ptr: *mut libc::c_void) -> *mut AllocationEntry {
    let mut entry: *mut AllocationEntry = 0 as *mut AllocationEntry;
    entry = alloc_head;
    while !entry.is_null() {
        if (*entry).allocation == ptr {
            return entry;
        }
        entry = (*entry).next
    }
    return crate::stddef_h::NULL as *mut AllocationEntry;
}
/* Free some memory and remove the tracking for it */
#[no_mangle]

pub unsafe extern "C" fn tracking_free(mut ptr: *mut libc::c_void) {
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
            alloc_tail = (*entry).next
        }
        ::libc::free(entry as *mut libc::c_void);
    } else {
        ::libc::printf(
            b"Attempting to free unallocated memory at %p\n\x00" as *const u8
                as *const libc::c_char,
            ptr,
        );
    }
    ::libc::free(ptr);
}
/* Reallocate some memory and keep track of it */
#[no_mangle]

pub unsafe extern "C" fn tracking_realloc(
    mut ptr: *mut libc::c_void,
    mut size: crate::stddef_h::size_t,
) -> *mut libc::c_void {
    let mut entry: *mut AllocationEntry = 0 as *mut AllocationEntry;
    if ptr.is_null() {
        /* By definition, this is equivalent to malloc(size) */
        return tracking_malloc(size);
    }
    if size == 0 as libc::c_int as libc::c_ulong {
        /* By definition, this is equivalent to free(ptr) */
        tracking_free(ptr);
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    /* Find the allocation entry for this memory */
    entry = find_allocation(ptr);
    if entry.is_null() {
        ::libc::printf(
            b"Attempting to realloc unallocated memory at %p\n\x00" as *const u8
                as *const libc::c_char,
            ptr,
        );
        entry = crate::stdlib::malloc(::std::mem::size_of::<AllocationEntry>() as libc::c_ulong)
            as *mut AllocationEntry;
        if entry.is_null() {
            ::libc::printf(b"Reallocator failure\n\x00" as *const u8 as *const libc::c_char);
            return crate::stddef_h::NULL as *mut libc::c_void;
        }
        (*entry).allocation = crate::stdlib::realloc(ptr, size);
        if (*entry).allocation.is_null() {
            ::libc::free(entry as *mut libc::c_void);
            return crate::stddef_h::NULL as *mut libc::c_void;
        }
        /* Add to the list of allocations */
        (*entry).next = crate::stddef_h::NULL as *mut allocation_entry;
        if alloc_head.is_null() {
            (*entry).prev = crate::stddef_h::NULL as *mut allocation_entry;
            alloc_tail = entry;
            alloc_head = alloc_tail
        } else {
            (*entry).prev = alloc_tail;
            (*alloc_tail).next = entry;
            alloc_tail = entry
        }
    } else {
        (*entry).allocation = crate::stdlib::realloc(ptr, size);
        if (*entry).allocation.is_null() {
            /* Realloc semantics say the original is still allocated */
            (*entry).allocation = ptr;
            return crate::stddef_h::NULL as *mut libc::c_void;
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

pub unsafe extern "C" fn tracking_report() -> libc::c_int {
    let mut entry: *mut AllocationEntry = 0 as *mut AllocationEntry;
    if alloc_head.is_null() {
        return 1 as libc::c_int;
    }
    /* Otherwise we have allocations that haven't been freed */
    entry = alloc_head;
    while !entry.is_null() {
        ::libc::printf(
            b"Allocated %lu bytes at %p\n\x00" as *const u8 as *const libc::c_char,
            (*entry).num_bytes,
            (*entry).allocation,
        );
        entry = (*entry).next
    }
    return 0 as libc::c_int;
}
