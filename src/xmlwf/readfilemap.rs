use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__blkcnt_t;
pub use crate::stdlib::__blksize_t;
pub use crate::stdlib::__dev_t;
pub use crate::stdlib::__gid_t;
pub use crate::stdlib::__ino_t;
pub use crate::stdlib::__mode_t;
pub use crate::stdlib::__nlink_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__syscall_slong_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uid_t;
pub use crate::stdlib::ssize_t;
pub use ::libc::timespec;

pub use crate::internal::__INT_MAX__;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
use crate::stdlib::fstat;
pub use crate::stdlib::stat;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
pub use crate::stdlib::__S_IFMT;
use ::libc::open;
pub use ::libc::O_RDONLY;

pub use crate::filemap_h::XML_MAX_CHUNK_LEN;
use crate::stdlib::fprintf;
use crate::stdlib::malloc;
use crate::stdlib::read;
use crate::stdlib::stderr;
pub use crate::xmltchar_h::ftprintf;
pub use crate::xmltchar_h::topen;
pub use crate::xmltchar_h::tperror;
use ::libc::close;
use ::libc::free;
use ::libc::perror;
pub use ::libc::INT_MAX;
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

pub const _EXPAT_read: unsafe extern "C" fn(
    _: libc::c_int,
    _: *mut libc::c_void,
    _: crate::stddef_h::size_t,
) -> crate::stdlib::ssize_t = crate::stdlib::read;
/* not S_ISREG */

pub const O_BINARY: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub unsafe extern "C" fn filemap(
    mut name: *const libc::c_char,
    mut processor: Option<
        unsafe extern "C" fn(
            _: *const libc::c_void,
            _: crate::stddef_h::size_t,
            _: *const libc::c_char,
            _: *mut libc::c_void,
        ) -> (),
    >,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut nbytes: crate::stddef_h::size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut n: crate::stdlib::ssize_t = 0;
    let mut sb: crate::stdlib::stat = crate::stdlib::stat {
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
        st_atim: ::libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: ::libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: ::libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    fd = ::libc::open(name, ::libc::O_RDONLY | O_BINARY);
    if fd < 0 as libc::c_int {
        ::libc::perror(name);
        return 0 as libc::c_int;
    }
    if crate::stdlib::fstat(fd, &mut sb) < 0 as libc::c_int {
        ::libc::perror(name);
        ::libc::close(fd);
        return 0 as libc::c_int;
    }
    if !(sb.st_mode & crate::stdlib::__S_IFMT as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: not a regular file\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
        ::libc::close(fd);
        return 0 as libc::c_int;
    }
    if sb.st_size > crate::filemap_h::XML_MAX_CHUNK_LEN as libc::c_long {
        ::libc::close(fd);
        return 2 as libc::c_int;
        /* Cannot be passed to XML_Parse in one go */
    }
    nbytes = sb.st_size as crate::stddef_h::size_t;
    /* malloc will return NULL with nbytes == 0, handle files with size 0 */
    if nbytes == 0 as libc::c_int as libc::c_ulong {
        static mut c: libc::c_char = '\u{0}' as i32 as libc::c_char;
        processor.expect("non-null function pointer")(
            &c as *const libc::c_char as *const libc::c_void,
            0 as libc::c_int as crate::stddef_h::size_t,
            name,
            arg,
        );
        ::libc::close(fd);
        return 1 as libc::c_int;
    }
    p = crate::stdlib::malloc(nbytes);
    if p.is_null() {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: out of memory\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
        ::libc::close(fd);
        return 0 as libc::c_int;
    }
    n = crate::stdlib::read(fd, p, nbytes);
    if n < 0 as libc::c_int as libc::c_long {
        ::libc::perror(name);
        ::libc::free(p);
        ::libc::close(fd);
        return 0 as libc::c_int;
    }
    if n != nbytes as crate::stdlib::ssize_t {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: read unexpected number of bytes\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
        ::libc::free(p);
        ::libc::close(fd);
        return 0 as libc::c_int;
    }
    processor.expect("non-null function pointer")(p, nbytes, name, arg);
    ::libc::free(p);
    ::libc::close(fd);
    return 1 as libc::c_int;
}
