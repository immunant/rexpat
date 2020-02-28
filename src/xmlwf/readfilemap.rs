pub use crate::filemap_h::XML_MAX_CHUNK_LEN;
use crate::stdlib::{stderr};
pub use crate::xmltchar_h::{ftprintf, topen, tperror};
use ::libc::{self, close, free, open, read, fstat, fprintf, perror, stat, malloc, S_IFREG, S_IFMT, size_t, ssize_t};
pub use ::libc::{timespec, INT_MAX, O_RDONLY};
use libc::{c_char, c_int, c_long, c_void};
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
    let mut sb: stat = std::mem::MaybeUninit::uninit().assume_init();
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
    if !(sb.st_mode & S_IFMT == S_IFREG) {
        fprintf(
            stderr,
            b"%s: not a regular file\n\x00".as_ptr() as *const c_char,
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
            0,
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
            b"%s: out of memory\n\x00".as_ptr() as *const c_char,
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
            b"%s: read unexpected number of bytes\n\x00".as_ptr() as *const c_char,
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
