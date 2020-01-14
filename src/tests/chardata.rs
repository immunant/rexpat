// =============== BEGIN chardata_h ================
use crate::minicheck::_fail_unless;
use crate::stdlib::__assert_fail;
use crate::stdlib::memcmp;
use crate::stdlib::memcpy;
use ::libc::sprintf;
use libc::c_char;
use libc::c_int;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CharData {
    pub count: c_int,
    pub data: [XML_Char; 2048],
}

pub use crate::expat_external_h::XML_Char;

use ::libc::{self};
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

unsafe extern "C" fn xmlstrlen(mut s: *const XML_Char) -> c_int {
    let mut len: c_int = 0 as c_int;
    if !s.is_null() {
    } else {
        __assert_fail(
            b"s != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/chardata.c\x00"
                as *const u8 as *const c_char,
            47 as c_int as c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
                b"int xmlstrlen(const XML_Char *)\x00",
            ))
            .as_ptr(),
        );
    }
    while *s.offset(len as isize) as c_int != 0 as c_int {
        len += 1
    }
    return len;
}
#[no_mangle]

pub unsafe extern "C" fn CharData_Init(mut storage: *mut CharData) {
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/chardata.c\x00"
                as *const u8 as *const c_char,
            55 as c_int as c_uint,
            (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(
                b"void CharData_Init(CharData *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*storage).count = -(1 as c_int);
}
#[no_mangle]

pub unsafe extern "C" fn CharData_AppendXMLChars(
    mut storage: *mut CharData,
    mut s: *const XML_Char,
    mut len: c_int,
) {
    let mut maxchars: c_int = 0;
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/chardata.c\x00"
                as *const u8 as *const c_char,
            63 as c_int as c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[c_char; 64]>(
                b"void CharData_AppendXMLChars(CharData *, const XML_Char *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if !s.is_null() {
    } else {
        __assert_fail(
            b"s != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/chardata.c\x00"
                as *const u8 as *const c_char,
            64 as c_int as c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[c_char; 64]>(
                b"void CharData_AppendXMLChars(CharData *, const XML_Char *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    maxchars = (::std::mem::size_of::<[XML_Char; 2048]>() as c_ulong)
        .wrapping_div(::std::mem::size_of::<XML_Char>() as c_ulong) as c_int;
    if (*storage).count < 0 as c_int {
        (*storage).count = 0 as c_int
    }
    if len < 0 as c_int {
        len = xmlstrlen(s)
    }
    if len + (*storage).count > maxchars {
        len = maxchars - (*storage).count
    }
    if len + (*storage).count < ::std::mem::size_of::<[XML_Char; 2048]>() as c_ulong as c_int {
        memcpy(
            (*storage)
                .data
                .as_mut_ptr()
                .offset((*storage).count as isize) as *mut c_void,
            s as *const c_void,
            (len as c_ulong).wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
        );
        (*storage).count += len
    };
}
/* Interface to some helper routines used to accumulate and check text
   and attribute content.
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
/* # of chars, < 0 if not set */
#[no_mangle]

pub unsafe extern "C" fn CharData_CheckXMLChars(
    mut storage: *mut CharData,
    mut expected: *const XML_Char,
) -> c_int {
    let mut buffer: [c_char; 1024] = [0; 1024];
    let mut len: c_int = xmlstrlen(expected);
    let mut count: c_int = 0;
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\x00" as *const u8 as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/chardata.c\x00"
                as *const u8 as *const c_char,
            85 as c_int as c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[c_char; 57]>(
                b"int CharData_CheckXMLChars(CharData *, const XML_Char *)\x00",
            ))
            .as_ptr(),
        );
    }
    count = if (*storage).count < 0 as c_int {
        0 as c_int
    } else {
        (*storage).count
    };
    if len != count {
        sprintf(
            buffer.as_mut_ptr(),
            b"wrong number of data characters: got %d, expected %d\x00" as *const u8
                as *const c_char,
            count,
            len,
        );
        _fail_unless(
            0 as c_int,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/chardata.c\x00"
                as *const u8 as *const c_char,
            90 as c_int,
            buffer.as_mut_ptr(),
        );
        return 0 as c_int;
    }
    if memcmp(
        expected as *const c_void,
        (*storage).data.as_mut_ptr() as *const c_void,
        (len as c_ulong).wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
    ) != 0 as c_int
    {
        _fail_unless(
            0 as c_int,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/chardata.c\x00"
                as *const u8 as *const c_char,
            94 as c_int,
            b"got bad data bytes\x00" as *const u8 as *const c_char,
        );
        return 0 as c_int;
    }
    return 1 as c_int;
}
