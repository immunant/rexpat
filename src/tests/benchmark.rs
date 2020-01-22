#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main,
           ptr_wrapping_offset_from, register_tool)]
pub mod expat_h {
    pub type XML_Parser =  *mut ::libexpat_rs::expat_h::XML_ParserStruct;
    
    
    pub type XML_Bool = libc::c_uchar;
    /* The XML_Status enum gives the possible return values for several
   API functions.  The preprocessor #defines are included so this
   stanza can be added to code that still needs to support older
   versions of Expat 1.95.x:

   #ifndef XML_STATUS_OK
   #define XML_STATUS_OK    1
   #define XML_STATUS_ERROR 0
   #endif

   Otherwise, the #define hackery is quite ugly and would have been
   dropped.
*/
    
    pub type XML_Status = libc::c_uint;
    
    
    pub type XML_Error = libc::c_uint;
}pub mod expat_external_h {
    pub type XML_Char =  libc::c_char;
    
    
    pub type XML_LChar = libc::c_char;
    pub type XML_Size =  libc::c_ulong;
}pub mod stddef_h {
    pub type size_t =  libc::c_ulong;
    
    
    pub const NULL_0: libc::c_int = 0 as libc::c_int;
}pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub static mut stderr:  *mut crate::stdlib::FILE;
        pub type _IO_marker;
        
        
        pub type _IO_codecvt;
        
        
        pub type _IO_wide_data;
        #[no_mangle]
        pub fn __xstat(__ver:  libc::c_int, __filename:  *const libc::c_char,
                       __stat_buf:  *mut ::libexpat_rs::stdlib::stat)
         ->  libc::c_int;
        #[no_mangle]
        pub fn clock() ->  crate::stdlib::clock_t;
    }
    pub type FILE =  ::libexpat_rs::stdlib::_IO_FILE;
    pub const CLOCKS_PER_SEC:  libc::c_int =  1000000 as libc::c_int;
    pub type clock_t =  crate::stdlib::__clock_t;
    pub const _STAT_VER_LINUX:  libc::c_int =  1 as libc::c_int;
    
    
    pub const _STAT_VER: libc::c_int = crate::stdlib::_STAT_VER_LINUX;
    pub type _IO_lock_t =  ();
    pub type __dev_t =  libc::c_ulong;
    
    
    pub type __uid_t = libc::c_uint;
    
    
    pub type __gid_t = libc::c_uint;
    
    
    pub type __ino_t = libc::c_ulong;
    
    
    pub type __mode_t = libc::c_uint;
    
    
    pub type __nlink_t = libc::c_ulong;
    
    
    pub type __off_t = libc::c_long;
    
    
    pub type __off64_t = libc::c_long;
    
    
    pub type __clock_t = libc::c_long;
    
    
    pub type __time_t = libc::c_long;
    
    
    pub type __blksize_t = libc::c_long;
    
    
    pub type __blkcnt_t = libc::c_long;
    
    
    pub type __syscall_slong_t = libc::c_long;
}use ::libexpat_rs::*;










pub mod sys_stat_h {
    #[inline]
    
    pub unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                                  mut __statbuf: *mut ::libexpat_rs::stdlib::stat) -> libc::c_int {
        return crate::stdlib::__xstat(crate::stdlib::_STAT_VER, __path, __statbuf);
    }
    
    use crate::stdlib::_STAT_VER;
}

pub mod stdlib_h {
    #[inline]
    
    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char)
     -> libc::c_int {
        return ::libc::strtol(__nptr,
                      ::libexpat_rs::stddef_h::NULL as *mut libc::c_void as *mut *mut libc::c_char,
                      10 as libc::c_int) as libc::c_int;
    }
    use ::libexpat_rs::stddef_h::NULL;
    
}






pub use crate::stdlib::__dev_t;pub use crate::stdlib::__uid_t;pub use crate::stdlib::__gid_t;pub use crate::stdlib::__ino_t;pub use crate::stdlib::__mode_t;pub use crate::stdlib::__nlink_t;pub use crate::stdlib::__off_t;pub use crate::stdlib::__off64_t;pub use crate::stdlib::__clock_t;pub use crate::stdlib::__time_t;pub use crate::stdlib::__blksize_t;pub use crate::stdlib::__blkcnt_t;pub use crate::stdlib::__syscall_slong_t;pub use crate::stdlib::_STAT_VER_LINUX;pub use crate::stdlib::_STAT_VER;pub use crate::stddef_h::size_t;pub use crate::stddef_h::NULL_0;
pub use crate::stdlib::clock_t;
pub use crate::stdlib::_IO_lock_t;pub use crate::stdlib::_IO_wide_data;pub use crate::stdlib::_IO_codecvt;pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::FILE;




pub use crate::expat_external_h::XML_Char;pub use crate::expat_external_h::XML_LChar;pub use crate::expat_external_h::XML_Size;pub use crate::expat_h::XML_Parser;pub use crate::expat_h::XML_Bool;pub use crate::expat_h::XML_Status;pub use crate::expat_h::XML_Error;pub use crate::sys_stat_h::stat;pub use crate::stdlib::__xstat;pub use crate::stdlib_h::atoi;use crate::stdlib::stderr;
use crate::stdlib::clock;
pub use crate::stdlib::CLOCKS_PER_SEC;
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

unsafe extern "C" fn usage(mut prog: *const libc::c_char,
                           mut rc: libc::c_int) {
    ::libexpat_rs::stdlib::fprintf(crate::stdlib::stderr as *mut ::libexpat_rs::stdlib::_IO_FILE,
            b"usage: %s [-n] filename bufferSize nr_of_loops\n\x00" as
                *const u8 as *const libc::c_char, prog);
    ::libc::exit(rc);
}

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut parser: crate::expat_h::XML_Parser = 0 as *mut ::libexpat_rs::expat_h::XML_ParserStruct;
    let mut XMLBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut XMLBufEnd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut XMLBufPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut fileAttr: ::libexpat_rs::stdlib::stat =
        ::libexpat_rs::stdlib::stat{st_dev:  0,
                            st_ino:  0,
                            st_nlink:  0,
                            st_mode:  0,
                            st_uid:  0,
                            st_gid:  0,
                            __pad0:  0,
                            st_rdev:  0,
                            st_size:  0,
                            st_blksize:  0,
                            st_blocks:  0,
                            st_atim:  ::libc::timespec{tv_sec:  0, tv_nsec:  0,},
                            st_mtim:  ::libc::timespec{tv_sec:  0, tv_nsec:  0,},
                            st_ctim:  ::libc::timespec{tv_sec:  0, tv_nsec:  0,},
                            __glibc_reserved:  [0; 3],};
    let mut nrOfLoops: libc::c_int = 0;
    let mut bufferSize: libc::c_int = 0;
    let mut fileSize: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut isFinal: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut ns: libc::c_int = 0 as libc::c_int;
    let mut tstart: crate::stdlib::clock_t = 0;
    let mut tend: crate::stdlib::clock_t = 0;
    let mut cpuTime: libc::c_double = 0.0f64;
    if argc > 1 as libc::c_int {
        if *(*argv.offset(1 as libc::c_int as
                              isize)).offset(0 as libc::c_int as isize) as
               libc::c_int == '-' as i32 {
            if *(*argv.offset(1 as libc::c_int as
                                  isize)).offset(1 as libc::c_int as isize) as
                   libc::c_int == 'n' as i32 &&
                   *(*argv.offset(1 as libc::c_int as
                                      isize)).offset(2 as libc::c_int as
                                                         isize) as libc::c_int
                       == '\u{0}' as i32 {
                ns = 1 as libc::c_int;
                j = 1 as libc::c_int
            } else {
                usage(*argv.offset(0 as libc::c_int as isize),
                      1 as libc::c_int);
            }
        }
    }
    if argc != j + 4 as libc::c_int {
        usage(*argv.offset(0 as libc::c_int as isize), 1 as libc::c_int);
    }
    if stat(*argv.offset((j + 1 as libc::c_int) as isize), &mut fileAttr) !=
           0 as libc::c_int {
        ::libexpat_rs::stdlib::fprintf(crate::stdlib::stderr as *mut ::libexpat_rs::stdlib::_IO_FILE,
                b"could not access file \'%s\'\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset((j + 1 as libc::c_int) as isize));
        return 2 as libc::c_int
    }
    fd =
        
        ::libexpat_rs::stdlib::fopen(*argv.offset((j + 1 as libc::c_int) as isize),
              b"r\x00" as *const u8 as *const libc::c_char) as
    *mut ::libexpat_rs::stdlib::_IO_FILE;
    if fd.is_null() {
        ::libexpat_rs::stdlib::fprintf(crate::stdlib::stderr as *mut ::libexpat_rs::stdlib::_IO_FILE,
                b"could not open file \'%s\'\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset((j + 1 as libc::c_int) as isize));
        ::libc::exit(2 as libc::c_int);
    }
    bufferSize = atoi(*argv.offset((j + 2 as libc::c_int) as isize));
    nrOfLoops = atoi(*argv.offset((j + 3 as libc::c_int) as isize));
    if bufferSize <= 0 as libc::c_int || nrOfLoops <= 0 as libc::c_int {
        ::libexpat_rs::stdlib::fprintf(crate::stdlib::stderr as *mut ::libexpat_rs::stdlib::_IO_FILE,
                b"buffer size and nr of loops must be greater than zero.\n\x00"
                    as *const u8 as *const libc::c_char);
        ::libc::exit(3 as libc::c_int);
    }
    XMLBuf = ::libexpat_rs::stdlib::malloc(fileAttr.st_size as libc::c_ulong) as *mut libc::c_char;
    fileSize =
        ::libexpat_rs::stdlib::fread(XMLBuf as *mut libc::c_void,
              ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
              fileAttr.st_size as libc::c_ulong,  fd as *mut ::libexpat_rs::stdlib::_IO_FILE) as libc::c_int;
    ::libexpat_rs::stdlib::fclose(fd as *mut ::libexpat_rs::stdlib::_IO_FILE);
    if ns != 0 {
        parser =
            ::libexpat_rs::lib::xmlparse::XML_ParserCreateNS(crate::stddef_h::NULL_0 as *const crate::expat_external_h::XML_Char,
                               '!' as i32 as crate::expat_external_h::XML_Char)
    } else { parser = ::libexpat_rs::lib::xmlparse::XML_ParserCreate(crate::stddef_h::NULL_0 as *const crate::expat_external_h::XML_Char) }
    i = 0 as libc::c_int;
    XMLBufEnd = XMLBuf.offset(fileSize as isize);
    while i < nrOfLoops {
        XMLBufPtr = XMLBuf;
        isFinal = 0 as libc::c_int;
        tstart = crate::stdlib::clock();
        loop  {
            let mut parseBufferSize: libc::c_int =
                XMLBufEnd.wrapping_offset_from(XMLBufPtr) as libc::c_long as
                    libc::c_int;
            if parseBufferSize <= bufferSize {
                isFinal = 1 as libc::c_int
            } else { parseBufferSize = bufferSize }
            if ::libexpat_rs::lib::xmlparse::XML_Parse(parser, XMLBufPtr, parseBufferSize, isFinal) as u64
                   == 0 {
                ::libexpat_rs::stdlib::fprintf(crate::stdlib::stderr as *mut ::libexpat_rs::stdlib::_IO_FILE,
                        b"error \'%s\' at line %lu character %lu\n\x00" as
                            *const u8 as *const libc::c_char,
                        ::libexpat_rs::lib::xmlparse::XML_ErrorString(::libexpat_rs::lib::xmlparse::XML_GetErrorCode(parser)),
                        ::libexpat_rs::lib::xmlparse::XML_GetCurrentLineNumber(parser),
                        ::libexpat_rs::lib::xmlparse::XML_GetCurrentColumnNumber(parser));
                ::libc::free(XMLBuf as *mut libc::c_void);
                ::libexpat_rs::lib::xmlparse::XML_ParserFree(parser);
                ::libc::exit(4 as libc::c_int);
            }
            XMLBufPtr = XMLBufPtr.offset(bufferSize as isize);
            if !(isFinal == 0) { break ; }
        }
        tend = crate::stdlib::clock();
        cpuTime +=
            (tend - tstart) as libc::c_double /
                crate::stdlib::CLOCKS_PER_SEC as libc::c_double;
        ::libexpat_rs::lib::xmlparse::XML_ParserReset(parser, crate::stddef_h::NULL_0 as *const crate::expat_external_h::XML_Char);
        i += 1
    }
    ::libexpat_rs::lib::xmlparse::XML_ParserFree(parser);
    ::libc::free(XMLBuf as *mut libc::c_void);
    ::libc::printf(b"%d loops, with buffer size %d. Average time per loop: %f\n\x00"
               as *const u8 as *const libc::c_char, nrOfLoops, bufferSize,
           cpuTime / nrOfLoops as libc::c_double);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
