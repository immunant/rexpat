/*
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
#![feature(
    const_raw_ptr_to_usize_cast,
    main,
    ptr_wrapping_offset_from,
    register_tool
)]

use std::mem;
use std::convert::TryInto;
use libc::{exit, free, printf};
use rexpat::expat_h::XML_ParserStruct;
use rexpat::lib::xmlparse::{
    XML_ErrorString, XML_GetCurrentColumnNumber, XML_GetCurrentLineNumber, XML_GetErrorCode,
    XML_Parse, XML_ParserCreate, XML_ParserCreateNS, XML_ParserFree, XML_ParserReset,
};
use std::ffi::CString;
use std::ptr::null_mut;
use libc::{c_char, c_double, c_int, c_long, c_void, FILE, fclose, fopen, fread, fprintf, malloc};

pub use rexpat::expat_external_h::{XML_Char, XML_LChar, XML_Size};
pub use rexpat::expat_h::{XML_Bool, XML_Error, XML_Parser, XML_Status};
pub use rexpat::stddef_h::{NULL};
use rexpat::stdlib::{clock, CLOCKS_PER_SEC, stderr};
pub use libc::{atoi, stat, size_t, clock_t};

unsafe extern "C" fn usage(mut prog: *const c_char, mut rc: c_int) {
    fprintf(
        stderr as *mut FILE,
        b"usage: %s [-n] filename bufferSize nr_of_loops\n\x00" as *const u8 as *const c_char,
        prog,
    );
    exit(rc);
}

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut XMLBuf: *mut c_char = 0 as *mut c_char;
    let mut XMLBufEnd: *mut c_char = 0 as *mut c_char;
    let mut XMLBufPtr: *mut c_char = 0 as *mut c_char;
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut fileAttr: stat = mem::zeroed();
    let mut nrOfLoops: c_int = 0;
    let mut bufferSize: c_int = 0;
    let mut fileSize: c_int = 0;
    let mut i: c_int = 0;
    let mut isFinal: c_int = 0;
    let mut j: c_int = 0 as c_int;
    let mut ns: c_int = 0 as c_int;
    let mut tstart: clock_t = 0;
    let mut tend: clock_t = 0;
    let mut cpuTime: c_double = 0.0f64;
    if argc > 1 as c_int {
        if *(*argv.offset(1 as c_int as isize)).offset(0 as c_int as isize) as c_int == '-' as i32 {
            if *(*argv.offset(1 as c_int as isize)).offset(1 as c_int as isize) as c_int
                == 'n' as i32
                && *(*argv.offset(1 as c_int as isize)).offset(2 as c_int as isize) as c_int
                    == '\u{0}' as i32
            {
                ns = 1 as c_int;
                j = 1 as c_int
            } else {
                usage(*argv.offset(0 as c_int as isize), 1 as c_int);
            }
        }
    }
    if argc != j + 4 as c_int {
        usage(*argv.offset(0 as c_int as isize), 1 as c_int);
    }
    if libc::stat(*argv.offset((j + 1 as c_int) as isize), &mut fileAttr) != 0 as c_int {
        fprintf(
            stderr as *mut FILE,
            b"could not access file \'%s\'\n\x00" as *const u8 as *const c_char,
            *argv.offset((j + 1 as c_int) as isize),
        );
        return 2 as c_int;
    }
    fd = fopen(
        *argv.offset((j + 1 as c_int) as isize),
        b"r\x00" as *const u8 as *const c_char,
    ) as *mut FILE;
    if fd.is_null() {
        fprintf(
            stderr as *mut FILE,
            b"could not open file \'%s\'\n\x00" as *const u8 as *const c_char,
            *argv.offset((j + 1 as c_int) as isize),
        );
        exit(2 as c_int);
    }
    bufferSize = atoi(*argv.offset((j + 2 as c_int) as isize));
    nrOfLoops = atoi(*argv.offset((j + 3 as c_int) as isize));
    if bufferSize <= 0 as c_int || nrOfLoops <= 0 as c_int {
        fprintf(
            stderr as *mut FILE,
            b"buffer size and nr of loops must be greater than zero.\n\x00" as *const u8
                as *const c_char,
        );
        exit(3 as c_int);
    }
    XMLBuf = malloc(fileAttr.st_size as usize) as *mut c_char;
    fileSize = fread(
        XMLBuf as *mut c_void,
        ::std::mem::size_of::<c_char>(),
        fileAttr.st_size.try_into().unwrap(),
        fd as *mut FILE,
    ) as c_int;
    fclose(fd as *mut FILE);
    if ns != 0 {
        parser = XML_ParserCreateNS(NULL as *const XML_Char, '!' as i32 as XML_Char)
    } else {
        parser = XML_ParserCreate(NULL as *const XML_Char)
    }
    i = 0 as c_int;
    XMLBufEnd = XMLBuf.offset(fileSize as isize);
    while i < nrOfLoops {
        XMLBufPtr = XMLBuf;
        isFinal = 0 as c_int;
        tstart = clock();
        loop {
            let mut parseBufferSize: c_int =
                XMLBufEnd.wrapping_offset_from(XMLBufPtr) as c_long as c_int;
            if parseBufferSize <= bufferSize {
                isFinal = 1 as c_int
            } else {
                parseBufferSize = bufferSize
            }
            if XML_Parse(parser, XMLBufPtr, parseBufferSize, isFinal) as u64 == 0 {
                fprintf(
                    stderr as *mut FILE,
                    b"error \'%s\' at line %lu character %lu\n\x00" as *const u8 as *const c_char,
                    XML_ErrorString(XML_GetErrorCode(parser)),
                    XML_GetCurrentLineNumber(parser),
                    XML_GetCurrentColumnNumber(parser),
                );
                free(XMLBuf as *mut c_void);
                XML_ParserFree(parser);
                exit(4 as c_int);
            }
            XMLBufPtr = XMLBufPtr.offset(bufferSize as isize);
            if !(isFinal == 0) {
                break;
            }
        }
        tend = clock();
        cpuTime += (tend - tstart) as c_double / CLOCKS_PER_SEC as c_double;
        XML_ParserReset(parser, NULL as *const XML_Char);
        i += 1
    }
    XML_ParserFree(parser);
    free(XMLBuf as *mut c_void);
    printf(
        b"%d loops, with buffer size %d. Average time per loop: %f\n\x00" as *const u8
            as *const c_char,
        nrOfLoops,
        bufferSize,
        cpuTime / nrOfLoops as c_double,
    );
    return 0 as c_int;
}
#[main]
pub fn orig_main() {
    let mut args: Vec<*mut c_char> = Vec::new();
    for arg in std::env::args() {
        // skip extra arguments added by `cargo bench`
        if arg == "--bench" { break; }
        args.push(
            CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as c_int,
            args.as_mut_ptr() as *mut *mut c_char,
        ) as i32)
    }
}

#[macro_use]
extern crate bencher;
use bencher::Bencher;

fn benchmark(b: &mut Bencher) {
    b.iter(|| crate::orig_main());
}

benchmark_group!(benches, benchmark);
benchmark_main!(benches); // emits a main function
