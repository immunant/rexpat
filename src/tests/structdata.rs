// =============== BEGIN structdata_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructDataEntry {
    pub str_0: *const crate::expat_external_h::XML_Char,
    pub data0: libc::c_int,
    pub data1: libc::c_int,
    pub data2: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructData {
    pub count: libc::c_int,
    pub max_count: libc::c_int,
    pub entries: *mut crate::src::tests::structdata::StructDataEntry,
}

pub use crate::expat_external_h::XML_Char;
use crate::src::tests::minicheck::_fail_unless;
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::{__assert_fail, malloc, memcpy, realloc, strlen};
use ::libc::{self, free, sprintf, strcmp};
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

pub const STRUCT_EXTENSION_COUNT: libc::c_int = 8 as libc::c_int;

unsafe extern "C" fn xmlstrdup(
    mut s: *const crate::expat_external_h::XML_Char,
) -> *mut crate::expat_external_h::XML_Char {
    let mut byte_count: crate::stddef_h::size_t = crate::stdlib::strlen(s)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong);
    let mut dup: *mut crate::expat_external_h::XML_Char =
        crate::stdlib::malloc(byte_count) as *mut crate::expat_external_h::XML_Char;
    if !dup.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"dup != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"XML_Char *xmlstrdup(const XML_Char *)\x00",
            ))
            .as_ptr(),
        );
    }
    crate::stdlib::memcpy(
        dup as *mut libc::c_void,
        s as *const libc::c_void,
        byte_count,
    );
    return dup;
}
#[no_mangle]

pub unsafe extern "C" fn StructData_Init(
    mut storage: *mut crate::src::tests::structdata::StructData,
) {
    if !storage.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"storage != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"void StructData_Init(StructData *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*storage).count = 0 as libc::c_int;
    (*storage).max_count = 0 as libc::c_int;
    (*storage).entries =
        crate::stddef_h::NULL as *mut crate::src::tests::structdata::StructDataEntry;
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
    mut storage: *mut crate::src::tests::structdata::StructData,
    mut s: *const crate::expat_external_h::XML_Char,
    mut data0: libc::c_int,
    mut data1: libc::c_int,
    mut data2: libc::c_int,
) {
    let mut entry: *mut crate::src::tests::structdata::StructDataEntry =
        0 as *mut crate::src::tests::structdata::StructDataEntry;
    if !storage.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"storage != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
                b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if !s.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"s != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
                b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*storage).count == (*storage).max_count {
        let mut new: *mut crate::src::tests::structdata::StructDataEntry =
            0 as *mut crate::src::tests::structdata::StructDataEntry;
        (*storage).max_count += STRUCT_EXTENSION_COUNT;
        new = crate::stdlib::realloc(
            (*storage).entries as *mut libc::c_void,
            ((*storage).max_count as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::src::tests::structdata::StructDataEntry,
            >() as libc::c_ulong),
        ) as *mut crate::src::tests::structdata::StructDataEntry;
        if !new.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"new != NULL\x00" as *const u8 as *const libc::c_char,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                    as *const u8 as *const libc::c_char,
                89 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
                    b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\x00",
                ))
                .as_ptr(),
            );
        }
        (*storage).entries = new
    }
    entry = &mut *(*storage).entries.offset((*storage).count as isize)
        as *mut crate::src::tests::structdata::StructDataEntry;
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
    mut storage: *mut crate::src::tests::structdata::StructData,
    mut expected: *const crate::src::tests::structdata::StructDataEntry,
    mut count: libc::c_int,
) {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    if !storage.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"storage != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
                b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if !expected.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"expected != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const libc::c_char,
            111 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
                b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if count != (*storage).count {
        ::libc::sprintf(
            buffer.as_mut_ptr(),
            b"wrong number of entries: got %d, expected %d\x00" as *const u8 as *const libc::c_char,
            (*storage).count,
            count,
        );
        StructData_Dispose(storage);
        crate::src::tests::minicheck::_fail_unless(
            0 as libc::c_int,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            buffer.as_mut_ptr(),
        );
    } else {
        i = 0 as libc::c_int;
        while i < count {
            let mut got: *const crate::src::tests::structdata::StructDataEntry =
                &mut *(*storage).entries.offset(i as isize)
                    as *mut crate::src::tests::structdata::StructDataEntry;
            let mut want: *const crate::src::tests::structdata::StructDataEntry = &*expected
                .offset(i as isize)
                as *const crate::src::tests::structdata::StructDataEntry;
            if !got.is_null() {
            } else {
                crate::stdlib::__assert_fail(b"got != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                                  as *const u8 as *const libc::c_char,
                              122 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 71],
                                                        &[libc::c_char; 71]>(b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\x00")).as_ptr());
            }
            if !want.is_null() {
            } else {
                crate::stdlib::__assert_fail(b"want != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                                  as *const u8 as *const libc::c_char,
                              123 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 71],
                                                        &[libc::c_char; 71]>(b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\x00")).as_ptr());
            }
            if ::libc::strcmp((*got).str_0, (*want).str_0) != 0 as libc::c_int {
                StructData_Dispose(storage);
                crate::src::tests::minicheck::_fail_unless(
                    0 as libc::c_int,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                        as *const u8 as *const libc::c_char,
                    127 as libc::c_int,
                    b"structure got bad string\x00" as *const u8 as *const libc::c_char,
                );
            } else if (*got).data0 != (*want).data0
                || (*got).data1 != (*want).data1
                || (*got).data2 != (*want).data2
            {
                ::libc::sprintf(
                    buffer.as_mut_ptr(),
                    b"struct \'%s\' expected (%d,%d,%d), got (%d,%d,%d)\x00" as *const u8
                        as *const libc::c_char,
                    (*got).str_0,
                    (*want).data0,
                    (*want).data1,
                    (*want).data2,
                    (*got).data0,
                    (*got).data1,
                    (*got).data2,
                );
                StructData_Dispose(storage);
                crate::src::tests::minicheck::_fail_unless(
                    0 as libc::c_int,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                        as *const u8 as *const libc::c_char,
                    137 as libc::c_int,
                    buffer.as_mut_ptr(),
                );
            }
            i += 1
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn StructData_Dispose(
    mut storage: *mut crate::src::tests::structdata::StructData,
) {
    let mut i: libc::c_int = 0;
    if !storage.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"storage != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/structdata.c\x00"
                as *const u8 as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void StructData_Dispose(StructData *)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < (*storage).count {
        ::libc::free((*(*storage).entries.offset(i as isize)).str_0 as *mut libc::c_void);
        i += 1
    }
    ::libc::free((*storage).entries as *mut libc::c_void);
    (*storage).count = 0 as libc::c_int;
    (*storage).entries =
        crate::stddef_h::NULL as *mut crate::src::tests::structdata::StructDataEntry;
}
