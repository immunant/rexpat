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
#![feature(const_raw_ptr_to_usize_cast, main, register_tool)]

use ::rexpat::expat_h::{XML_Status};
use ::rexpat::lib::xmlparse::{
    XML_ErrorString, XML_GetCurrentLineNumber, XML_GetErrorCode, XML_Parse, XML_ParserCreate,
    XML_ParserFree, XML_SetElementHandler,
};
use ::rexpat::stddef_h::NULL;
use ::libc::{exit, printf, fprintf};

use libc::{c_char, c_int, c_void, fread};
pub mod stdlib {

    use libc::{c_int, c_long, FILE};
    extern "C" {
        #[cfg(all(unix, not(target_os = "macos")))]
        #[no_mangle]
        pub static mut stderr: *mut FILE;

        #[cfg(all(unix, not(target_os = "macos")))]
        #[no_mangle]
        pub static mut stdin: *mut FILE;

        #[cfg(target_os = "macos")]
        #[no_mangle]
        #[link_name = "__stderrp"]
        pub static mut stderr: *mut FILE;

        #[cfg(target_os = "macos")]
        #[no_mangle]
        #[link_name = "__stdinp"]
        pub static mut stdin: *mut FILE;

        #[no_mangle]
        pub fn feof(__stream: *mut FILE) -> c_int;

        #[no_mangle]
        pub fn ferror(__stream: *mut FILE) -> c_int;
    }
    pub type _IO_lock_t = ();
    pub type __off_t = c_long;

    pub type __off64_t = c_long;
}

pub use ::rexpat::expat_external_h::{XML_Char, XML_LChar, XML_Size};
pub use ::rexpat::expat_h::{
    XML_EndElementHandler, XML_Error, XML_Parser, XML_StartElementHandler,
};
pub use ::rexpat::stdlib::{
    _IO_lock_t, __off64_t, __off_t,
};

/* Read an XML document from standard input and print an element
   outline on standard output.
   Must be used with Expat compiled for UTF-8 output.
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

pub const BUFFSIZE: c_int = 8192;
#[no_mangle]

pub static mut Buff: [c_char; 8192] = [0; 8192];
#[no_mangle]

pub static mut Depth: c_int = 0;

unsafe extern "C" fn start(
    mut _data: *mut c_void,
    mut el: *const XML_Char,
    mut attr: *mut *const XML_Char,
) {
    let mut i: c_int = 0;
    i = 0;
    while i < Depth {
        printf(b"  \x00".as_ptr() as *const c_char);
        i += 1
    }
    printf(b"%s\x00".as_ptr() as *const c_char, el);
    i = 0;
    while !(*attr.offset(i as isize)).is_null() {
        printf(
            b" %s=\'%s\'\x00".as_ptr() as *const c_char,
            *attr.offset(i as isize),
            *attr.offset((i + 1i32) as isize),
        );
        i += 2
    }
    printf(b"\n\x00".as_ptr() as *const c_char);
    Depth += 1;
}

unsafe extern "C" fn end(mut _data: *mut c_void, mut _el: *const XML_Char) {
    Depth -= 1;
}

unsafe fn main_0(mut _argc: c_int, mut _argv: *mut *mut c_char) -> c_int {
    let mut p: XML_Parser = XML_ParserCreate(NULL as *const XML_Char);
    if p.is_null() {
        fprintf(
            crate::stdlib::stderr,
            b"Couldn\'t allocate memory for parser\n\x00".as_ptr() as *const c_char,
        );
        exit(-(1i32));
    }
    XML_SetElementHandler(
        p,
        Some(
            start
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
        Some(end as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()),
    );
    loop {
        let mut done = false;
        let mut len: c_int = 0;
        len = fread(
            Buff.as_mut_ptr() as *mut c_void,
            1,
            BUFFSIZE as usize,
            crate::stdlib::stdin,
        ) as c_int;
        if crate::stdlib::ferror(crate::stdlib::stdin) != 0 {
            fprintf(
                crate::stdlib::stderr,
                b"Read error\n\x00".as_ptr() as *const c_char,
            );
            exit(-(1i32));
        }
        done = crate::stdlib::feof(crate::stdlib::stdin) != 0;
        if XML_Parse(p, Buff.as_mut_ptr(), len, done as c_int) == XML_Status::ERROR {
            fprintf(
                crate::stdlib::stderr,
                b"Parse error at line %lu:\n%s\n\x00".as_ptr() as *const c_char,
                XML_GetCurrentLineNumber(p),
                XML_ErrorString(XML_GetErrorCode(p)),
            );
            exit(-(1i32));
        }
        if done {
            break;
        }
    }
    XML_ParserFree(p);
    return 0;
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
    unsafe { ::std::process::exit(main_0((args.len() - 1) as libc::c_int, args.as_mut_ptr())) }
}
