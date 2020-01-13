#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]
pub mod expat_h {
    pub type XML_Parser = *mut ::c2rust_out::expat_h::XML_ParserStruct;

    pub type XML_Status = libc::c_uint;

    pub type XML_Error = libc::c_uint;

    pub type XML_StartElementHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
            _: *mut *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_EndElementHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;
}
pub mod expat_external_h {
    pub type XML_Char = libc::c_char;

    pub type XML_LChar = libc::c_char;

    pub type XML_Size = libc::c_ulong;
}
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
}
pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub static mut stdin: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub static mut stderr: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn fread(
            _: *mut libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut crate::stdlib::FILE,
        ) -> libc::c_ulong;
        pub type _IO_marker;

        pub type _IO_codecvt;

        pub type _IO_wide_data;
    }
    pub type FILE = ::c2rust_out::stdlib::_IO_FILE;
    pub type _IO_lock_t = ();
    pub type __off_t = libc::c_long;

    pub type __off64_t = libc::c_long;
}

pub use crate::expat_external_h::{XML_Char, XML_LChar, XML_Size};
pub use crate::expat_h::{
    XML_EndElementHandler, XML_Error, XML_Parser, XML_StartElementHandler, XML_Status,
};
pub use crate::stddef_h::size_t;
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, FILE,
};
use crate::stdlib::{fread, stderr, stdin};
use ::c2rust_out::*;
/* This is simple demonstration of how to use expat. This program
   reads an XML document from standard input and writes a line with
   the name of each element to standard output indenting child
   elements by one tab stop more than their parent element.
   It must be used with Expat compiled for UTF-8 output.
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

unsafe extern "C" fn startElement(
    mut userData: *mut libc::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    let mut i: libc::c_int = 0;
    let mut depthPtr: *mut libc::c_int = userData as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < *depthPtr {
        ::libc::putchar('\t' as i32);
        i += 1
    }
    ::libc::printf(b"%s\n\x00" as *const u8 as *const libc::c_char, name);
    *depthPtr += 1 as libc::c_int;
}

unsafe extern "C" fn endElement(
    mut userData: *mut libc::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
) {
    let mut depthPtr: *mut libc::c_int = userData as *mut libc::c_int;
    *depthPtr -= 1 as libc::c_int;
}

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut parser: crate::expat_h::XML_Parser = ::c2rust_out::src::lib::xmlparse::XML_ParserCreate(
        ::c2rust_out::stddef_h::NULL as *const crate::expat_external_h::XML_Char,
    );
    let mut done: libc::c_int = 0;
    let mut depth: libc::c_int = 0 as libc::c_int;
    ::c2rust_out::src::lib::xmlparse::XML_SetUserData(
        parser,
        &mut depth as *mut libc::c_int as *mut libc::c_void,
    );
    ::c2rust_out::src::lib::xmlparse::XML_SetElementHandler(
        parser,
        Some(
            startElement
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *const crate::expat_external_h::XML_Char,
                    _: *mut *const crate::expat_external_h::XML_Char,
                ) -> (),
        ),
        Some(
            endElement
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *const crate::expat_external_h::XML_Char,
                ) -> (),
        ),
    );
    loop {
        let mut len: crate::stddef_h::size_t = crate::stdlib::fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            crate::stdlib::stdin,
        );
        done =
            (len < ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong) as libc::c_int;
        if ::c2rust_out::src::lib::xmlparse::XML_Parse(
            parser,
            buf.as_mut_ptr(),
            len as libc::c_int,
            done,
        ) as libc::c_uint
            == ::c2rust_out::expat_h::XML_STATUS_ERROR_0 as libc::c_uint
        {
            ::c2rust_out::stdlib::fprintf(
                crate::stdlib::stderr as *mut ::c2rust_out::stdlib::_IO_FILE,
                b"%s at line %lu\n\x00" as *const u8 as *const libc::c_char,
                ::c2rust_out::src::lib::xmlparse::XML_ErrorString(
                    ::c2rust_out::src::lib::xmlparse::XML_GetErrorCode(parser),
                ),
                ::c2rust_out::src::lib::xmlparse::XML_GetCurrentLineNumber(parser),
            );
            ::c2rust_out::src::lib::xmlparse::XML_ParserFree(parser);
            return 1 as libc::c_int;
        }
        if !(done == 0) {
            break;
        }
    }
    ::c2rust_out::src::lib::xmlparse::XML_ParserFree(parser);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
