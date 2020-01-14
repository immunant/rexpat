// =============== BEGIN structdata_h ================

use crate::minicheck::_fail_unless;
use crate::stdlib::{__assert_fail, malloc, memcpy, realloc, strlen};
use ::libc::{free, sprintf, strcmp};
use libc::{c_char, c_int, c_ulong, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructDataEntry {
    pub str_0: *const XML_Char,
    pub data0: c_int,
    pub data1: c_int,
    pub data2: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructData {
    pub count: c_int,
    pub max_count: c_int,
    pub entries: *mut StructDataEntry,
}

pub use crate::expat_external_h::XML_Char;
pub use crate::stddef_h::{size_t, NULL};
use ::libc;
/*
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

pub const STRUCT_EXTENSION_COUNT: c_int = 8i32;

unsafe extern "C" fn xmlstrdup(mut s: *const XML_Char) -> *mut XML_Char {
    let mut byte_count: size_t = strlen(s)
        .wrapping_add(1u64)
        .wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong);
    let mut dup: *mut XML_Char = malloc(byte_count) as *mut XML_Char;
    if !dup.is_null() {
    } else {
        __assert_fail(
            b"dup != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const c_char,
            63u32,
            (*::std::mem::transmute::<&[u8; 38], &[c_char; 38]>(
                b"XML_Char *xmlstrdup(const XML_Char *)\x00",
            ))
            .as_ptr(),
        );
    }
    memcpy(dup as *mut c_void, s as *const c_void, byte_count);
    return dup;
}
#[no_mangle]

pub unsafe extern "C" fn StructData_Init(mut storage: *mut StructData) {
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const c_char,
            70u32,
            (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
                b"void StructData_Init(StructData *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*storage).count = 0i32;
    (*storage).max_count = 0i32;
    (*storage).entries = NULL as *mut StructDataEntry;
}
/* Interface to some helper routines used to accumulate and check
   structured content.
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
/* Number of entries used */
/* Number of StructDataEntry items in `entries` */
#[no_mangle]

pub unsafe extern "C" fn StructData_AddItem(
    mut storage: *mut StructData,
    mut s: *const XML_Char,
    mut data0: c_int,
    mut data1: c_int,
    mut data2: c_int,
) {
    let mut entry: *mut StructDataEntry = 0 as *mut StructDataEntry;
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const c_char,
            81u32,
            (*::std::mem::transmute::<&[u8; 71], &[c_char; 71]>(
                b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if !s.is_null() {
    } else {
        __assert_fail(
            b"s != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const c_char,
            82u32,
            (*::std::mem::transmute::<&[u8; 71], &[c_char; 71]>(
                b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*storage).count == (*storage).max_count {
        let mut new: *mut StructDataEntry = 0 as *mut StructDataEntry;
        (*storage).max_count += STRUCT_EXTENSION_COUNT;
        new = realloc(
            (*storage).entries as *mut c_void,
            ((*storage).max_count as c_ulong)
                .wrapping_mul(::std::mem::size_of::<StructDataEntry>() as c_ulong),
        ) as *mut StructDataEntry;
        if !new.is_null() {
        } else {
            __assert_fail(
                b"new != NULL\x00" as *const u8 as *const c_char,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                    as *const u8 as *const c_char,
                89u32,
                (*::std::mem::transmute::<&[u8; 71], &[c_char; 71]>(
                    b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\x00",
                ))
                .as_ptr(),
            );
        }
        (*storage).entries = new
    }
    entry = &mut *(*storage).entries.offset((*storage).count as isize) as *mut StructDataEntry;
    (*entry).str_0 = xmlstrdup(s);
    (*entry).data0 = data0;
    (*entry).data1 = data1;
    (*entry).data2 = data2;
    (*storage).count += 1;
}
/* 'fail()' aborts the function via a longjmp, so there is no point
 * in returning a value from this function.
 */
#[no_mangle]

pub unsafe extern "C" fn StructData_CheckItems(
    mut storage: *mut StructData,
    mut expected: *const StructDataEntry,
    mut count: c_int,
) {
    let mut buffer: [c_char; 1024] = [0; 1024];
    let mut i: c_int = 0;
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const c_char,
            110u32,
            (*::std::mem::transmute::<&[u8; 71], &[c_char; 71]>(
                b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if !expected.is_null() {
    } else {
        __assert_fail(
            b"expected != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const c_char,
            111u32,
            (*::std::mem::transmute::<&[u8; 71], &[c_char; 71]>(
                b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if count != (*storage).count {
        sprintf(
            buffer.as_mut_ptr(),
            b"wrong number of entries: got %d, expected %d\x00" as *const u8 as *const c_char,
            (*storage).count,
            count,
        );
        StructData_Dispose(storage);
        _fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const c_char,
            116i32,
            buffer.as_mut_ptr(),
        );
    } else {
        i = 0i32;
        while i < count {
            let mut got: *const StructDataEntry =
                &mut *(*storage).entries.offset(i as isize) as *mut StructDataEntry;
            let mut want: *const StructDataEntry =
                &*expected.offset(i as isize) as *const StructDataEntry;
            if !got.is_null() {
            } else {
                __assert_fail(b"got != NULL\x00" as *const u8 as
                                  *const c_char,
                              b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                                  as *const u8 as *const c_char,
                              122u32,
                              (*::std::mem::transmute::<&[u8; 71],
                                                        &[c_char; 71]>(b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\x00")).as_ptr());
            }
            if !want.is_null() {
            } else {
                __assert_fail(b"want != NULL\x00" as *const u8 as
                                  *const c_char,
                              b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                                  as *const u8 as *const c_char,
                              123u32,
                              (*::std::mem::transmute::<&[u8; 71],
                                                        &[c_char; 71]>(b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\x00")).as_ptr());
            }
            if strcmp((*got).str_0, (*want).str_0) != 0i32 {
                StructData_Dispose(storage);
                _fail_unless(
                    0i32,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                        as *const u8 as *const c_char,
                    127i32,
                    b"structure got bad string\x00" as *const u8 as *const c_char,
                );
            } else if (*got).data0 != (*want).data0
                || (*got).data1 != (*want).data1
                || (*got).data2 != (*want).data2
            {
                sprintf(
                    buffer.as_mut_ptr(),
                    b"struct \'%s\' expected (%d,%d,%d), got (%d,%d,%d)\x00" as *const u8
                        as *const c_char,
                    (*got).str_0,
                    (*want).data0,
                    (*want).data1,
                    (*want).data2,
                    (*got).data0,
                    (*got).data1,
                    (*got).data2,
                );
                StructData_Dispose(storage);
                _fail_unless(
                    0i32,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                        as *const u8 as *const c_char,
                    137i32,
                    buffer.as_mut_ptr(),
                );
            }
            i += 1
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn StructData_Dispose(mut storage: *mut StructData) {
    let mut i: c_int = 0;
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const c_char,
            148u32,
            (*::std::mem::transmute::<&[u8; 38], &[c_char; 38]>(
                b"void StructData_Dispose(StructData *)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32;
    while i < (*storage).count {
        free((*(*storage).entries.offset(i as isize)).str_0 as *mut c_void);
        i += 1
    }
    free((*storage).entries as *mut c_void);
    (*storage).count = 0i32;
    (*storage).entries = NULL as *mut StructDataEntry;
}
