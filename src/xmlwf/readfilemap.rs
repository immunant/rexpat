pub use crate::filemap_h::XML_MAX_CHUNK_LEN;
pub use crate::internal::__INT_MAX__;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __blkcnt_t, __blksize_t, __dev_t, __gid_t,
    __ino_t, __mode_t, __nlink_t, __off64_t, __off_t, __ssize_t, __syscall_slong_t, __time_t,
    __uid_t, ssize_t, stat, FILE, _IO_FILE, __S_IFMT,
};
use crate::stdlib::{fprintf, fstat, malloc, read, stderr};
pub use crate::xmltchar_h::{ftprintf, topen, tperror};
use ::libc::{self, close, free, open, perror};
pub use ::libc::{timespec, INT_MAX, O_RDONLY};
use libc::{c_char, c_int, c_long, c_uint, c_void};
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
/* Functions close(2) and read(2) */
/* Function "read": */
/* POSIX */
/* http://pubs.opengroup.org/onlinepubs/009695399/functions/read.html */

pub const _EXPAT_read: unsafe extern "C" fn(_: c_int, _: *mut c_void, _: size_t) -> ssize_t = read;
/* not S_ISREG */

pub const O_BINARY: c_int = 0;
#[no_mangle]

pub unsafe extern "C" fn filemap(
    mut name: *const c_char,
    mut processor: Option<
        unsafe extern "C" fn(_: *const c_void, _: size_t, _: *const c_char, _: *mut c_void) -> (),
    >,
    mut arg: *mut c_void,
) -> c_int {
    let mut nbytes: size_t = 0;
    let mut fd: c_int = 0;
    let mut n: ssize_t = 0;
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut p: *mut c_void = 0 as *mut c_void;
    fd = open(name, O_RDONLY | O_BINARY);
    if fd < 0 {
        perror(name);
        return 0i32;
    }
    if fstat(fd, &mut sb) < 0 {
        perror(name);
        close(fd);
        return 0i32;
    }
    if !(sb.st_mode & __S_IFMT as c_uint == 0o100000) {
        fprintf(
            stderr,
            b"%s: not a regular file\n\x00" as *const u8 as *const c_char,
            name,
        );
        close(fd);
        return 0i32;
    }
    if sb.st_size > XML_MAX_CHUNK_LEN as c_long {
        close(fd);
        return 2i32;
        /* Cannot be passed to XML_Parse in one go */
    }
    nbytes = sb.st_size as size_t;
    /* malloc will return NULL with nbytes == 0, handle files with size 0 */
    if nbytes == 0 {
        static mut c: c_char = '\u{0}' as c_char;
        processor.expect("non-null function pointer")(
            &c as *const c_char as *const c_void,
            0u64,
            name,
            arg,
        );
        close(fd);
        return 1i32;
    }
    p = malloc(nbytes);
    if p.is_null() {
        fprintf(
            stderr,
            b"%s: out of memory\n\x00" as *const u8 as *const c_char,
            name,
        );
        close(fd);
        return 0i32;
    }
    n = read(fd, p, nbytes);
    if n < 0 {
        perror(name);
        free(p);
        close(fd);
        return 0i32;
    }
    if n != nbytes as ssize_t {
        fprintf(
            stderr,
            b"%s: read unexpected number of bytes\n\x00" as *const u8 as *const c_char,
            name,
        );
        free(p);
        close(fd);
        return 0i32;
    }
    processor.expect("non-null function pointer")(p, nbytes, name, arg);
    free(p);
    close(fd);
    return 1;
}
