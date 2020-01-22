#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main,
           ptr_wrapping_offset_from, register_tool)]
use ::libexpat_rs::*;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "156:1"]
    pub type __clock_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:33"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/stat.h:33"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    #[c2rust::src_loc = "38:10"]
    pub const _STAT_VER_LINUX: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const _STAT_VER: libc::c_int = _STAT_VER_LINUX;
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.0/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "89:11"]
    pub const NULL_0: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/types/clock_t.h:34"]
pub mod clock_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type clock_t = __clock_t;
    use super::types_h::__clock_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:35"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:35"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/sjcrane/projects/c2rust/dogfooding/expat-2.2.9/lib/expat_external.h:37"]
pub mod expat_external_h {
    #[c2rust::src_loc = "142:1"]
    pub type XML_Char = libc::c_char;
    #[c2rust::src_loc = "143:1"]
    pub type XML_LChar = libc::c_char;
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
    /* External API definitions */
    /* Expat tries very hard to make the API boundary very specifically
   defined.  There are two macros defined to control this boundary;
   each of these can be defined before including this header to
   achieve some different behavior, but doing so it not recommended or
   tested frequently.

   XMLCALL    - The calling convention to use for all calls across the
                "library boundary."  This will default to cdecl, and
                try really hard to tell the compiler that's what we
                want.

   XMLIMPORT  - Whatever magic is needed to note that a function is
                to be imported from a dynamically loaded library
                (.dll, .so, or .sl, depending on your platform).

   The XMLCALL macro was added in Expat 1.95.7.  The only one which is
   expected to be directly useful in client code is XMLCALL.

   Note that on at least some Unix versions, the Expat library must be
   compiled with the cdecl calling convention as the default since
   system headers may assume the cdecl convention.
*/
    /* For any platform which uses this definition and supports more than
   one calling convention, we need to extend this definition to
   declare the convention used on that platform, if it's possible to
   do so.

   If this is the case for your platform, please file a bug report
   with information on how to identify your platform via the C
   pre-processor and how to specify the same calling convention as the
   platform's malloc() implementation.
*/
    /* not defined XMLCALL */
    /* using Expat from an application */
    /* not defined XML_STATIC */
    /* If we didn't define it above, define it away: */
    /* Information is UTF-16 encoded. */
    /* Information is UTF-8 encoded. */
    /* XML_UNICODE */
    /* Use large integers for file/stream positions. */
    #[c2rust::src_loc = "151:1"]
    pub type XML_Size = libc::c_ulong;
    /* not Expat_External_INCLUDED */
    /* XML_LARGE_SIZE */
}
#[c2rust::header_src =
  "/home/sjcrane/projects/c2rust/dogfooding/expat-2.2.9/lib/expat.h:37"]
pub mod expat_h {
    #[c2rust::src_loc = "44:1"]
    pub type XML_Parser = *mut XML_ParserStruct;
    #[c2rust::src_loc = "46:1"]
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
    #[c2rust::src_loc = "63:1"]
    pub type XML_Status = libc::c_uint;
    #[c2rust::src_loc = "68:3"]
    pub const XML_STATUS_SUSPENDED: XML_Status = 2;
    #[c2rust::src_loc = "66:3"]
    pub const XML_STATUS_OK: XML_Status = 1;
    #[c2rust::src_loc = "64:3"]
    pub const XML_STATUS_ERROR: XML_Status = 0;
    #[c2rust::src_loc = "72:1"]
    pub type XML_Error = libc::c_uint;
    /* Added in 2.2.1. */
    #[c2rust::src_loc = "118:3"]
    pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
    #[c2rust::src_loc = "116:3"]
    pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
    #[c2rust::src_loc = "115:3"]
    pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
    /* Added in 2.0. */
    #[c2rust::src_loc = "114:3"]
    pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
    #[c2rust::src_loc = "112:3"]
    pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
    #[c2rust::src_loc = "111:3"]
    pub const XML_ERROR_FINISHED: XML_Error = 36;
    #[c2rust::src_loc = "110:3"]
    pub const XML_ERROR_ABORTED: XML_Error = 35;
    #[c2rust::src_loc = "109:3"]
    pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
    #[c2rust::src_loc = "108:3"]
    pub const XML_ERROR_SUSPENDED: XML_Error = 33;
    #[c2rust::src_loc = "107:3"]
    pub const XML_ERROR_PUBLICID: XML_Error = 32;
    #[c2rust::src_loc = "106:3"]
    pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
    #[c2rust::src_loc = "105:3"]
    pub const XML_ERROR_XML_DECL: XML_Error = 30;
    #[c2rust::src_loc = "104:3"]
    pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
    /* Added in 1.95.8. */
    #[c2rust::src_loc = "103:3"]
    pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
    /* Added in 1.95.7. */
    #[c2rust::src_loc = "101:3"]
    pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
    #[c2rust::src_loc = "99:3"]
    pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
    #[c2rust::src_loc = "98:3"]
    pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
    #[c2rust::src_loc = "97:3"]
    pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
    #[c2rust::src_loc = "96:3"]
    pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
    #[c2rust::src_loc = "95:3"]
    pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
    #[c2rust::src_loc = "94:3"]
    pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
    #[c2rust::src_loc = "93:3"]
    pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
    #[c2rust::src_loc = "92:3"]
    pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
    #[c2rust::src_loc = "91:3"]
    pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
    #[c2rust::src_loc = "90:3"]
    pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
    #[c2rust::src_loc = "89:3"]
    pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
    #[c2rust::src_loc = "88:3"]
    pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
    #[c2rust::src_loc = "87:3"]
    pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
    #[c2rust::src_loc = "86:3"]
    pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
    #[c2rust::src_loc = "85:3"]
    pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
    #[c2rust::src_loc = "84:3"]
    pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
    #[c2rust::src_loc = "83:3"]
    pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
    #[c2rust::src_loc = "82:3"]
    pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
    #[c2rust::src_loc = "81:3"]
    pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
    #[c2rust::src_loc = "80:3"]
    pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
    #[c2rust::src_loc = "79:3"]
    pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
    #[c2rust::src_loc = "78:3"]
    pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
    #[c2rust::src_loc = "77:3"]
    pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
    #[c2rust::src_loc = "76:3"]
    pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
    #[c2rust::src_loc = "75:3"]
    pub const XML_ERROR_SYNTAX: XML_Error = 2;
    #[c2rust::src_loc = "74:3"]
    pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
    #[c2rust::src_loc = "73:3"]
    pub const XML_ERROR_NONE: XML_Error = 0;
    use super::expat_external_h::{XML_Char, XML_Size, XML_LChar};
    extern "C" {
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
        #[c2rust::src_loc = "43:8"]
        pub type XML_ParserStruct;
        /* Constructs a new parser; encoding is the encoding specified by the
   external protocol or NULL if there is none specified.
*/
        #[no_mangle]
        #[c2rust::src_loc = "216:1"]
        pub fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser;
        /* Constructs a new parser and namespace processor.  Element type
   names and attribute names that belong to a namespace will be
   expanded; unprefixed attribute names are never expanded; unprefixed
   element type names are expanded only if there is a default
   namespace. The expanded name is the concatenation of the namespace
   URI, the namespace separator character, and the local part of the
   name.  If the namespace separator is '\0' then the namespace URI
   and the local part will be concatenated without any separator.
   It is a programming error to use the separator '\0' with namespace
   triplets (see XML_SetReturnNSTriplet).
*/
        #[no_mangle]
        #[c2rust::src_loc = "230:1"]
        pub fn XML_ParserCreateNS(encoding: *const XML_Char,
                                  namespaceSeparator: XML_Char) -> XML_Parser;
        /* Prepare a parser object to be re-used.  This is particularly
   valuable when memory allocation overhead is disproportionately high,
   such as when a large number of small documnents need to be parsed.
   All handlers are cleared from the parser, except for the
   unknownEncodingHandler. The parser's external state is re-initialized
   except for the values of ns and ns_triplets.

   Added in Expat 1.95.3.
*/
        #[no_mangle]
        #[c2rust::src_loc = "256:1"]
        pub fn XML_ParserReset(parser: XML_Parser, encoding: *const XML_Char)
         -> XML_Bool;
        /* Parses some input. Returns XML_STATUS_ERROR if a fatal error is
   detected.  The last call to XML_Parse must have isFinal true; len
   may be zero for this call (or any other).

   Though the return values for these functions has always been
   described as a Boolean value, the implementation, at least for the
   1.95.x series, has always returned exactly one of the XML_Status
   values.
*/
        #[no_mangle]
        #[c2rust::src_loc = "757:1"]
        pub fn XML_Parse(parser: XML_Parser, s: *const libc::c_char,
                         len: libc::c_int, isFinal: libc::c_int)
         -> XML_Status;
        /* If XML_Parse or XML_ParseBuffer have returned XML_STATUS_ERROR, then
   XML_GetErrorCode returns information about the error.
*/
        #[no_mangle]
        #[c2rust::src_loc = "896:1"]
        pub fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
        /* These functions return information about the current parse
   location.  They may be called from any callback called to report
   some parse event; in this case the location is the location of the
   first of the sequence of characters that generated the event.  When
   called from callbacks generated by declarations in the document
   prologue, the location identified isn't as neatly defined, but will
   be within the relevant markup.  When called outside of the callback
   functions, the position indicated will be just past the last parse
   event (regardless of whether there was an associated callback).

   They may also be called after returning from a call to XML_Parse
   or XML_ParseBuffer.  If the return value is XML_STATUS_ERROR then
   the location is the location of the character at which the error
   was detected; otherwise the location is the location of the last
   parse event, as described above.

   Note: XML_GetCurrentLineNumber and XML_GetCurrentColumnNumber
   return 0 to indicate an error.
   Note: XML_GetCurrentByteIndex returns -1 to indicate an error.
*/
        #[no_mangle]
        #[c2rust::src_loc = "919:1"]
        pub fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size;
        #[no_mangle]
        #[c2rust::src_loc = "920:1"]
        pub fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> XML_Size;
        /* Frees memory used by the parser. */
        #[no_mangle]
        #[c2rust::src_loc = "965:1"]
        pub fn XML_ParserFree(parser: XML_Parser);
        /* Returns a string describing the error. */
        #[no_mangle]
        #[c2rust::src_loc = "969:1"]
        pub fn XML_ErrorString(code: XML_Error) -> *const XML_LChar;
    }
    /* not Expat_INCLUDED */
    /* Expat follows the semantic versioning convention.
   See http://semver.org.
*/
}
#[c2rust::header_src = "/usr/include/sys/stat.h:33"]
pub mod sys_stat_h {
    #[inline]
    #[c2rust::src_loc = "452:1"]
    pub unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                                  mut __statbuf: *mut stat) -> libc::c_int {
        return __xstat(_STAT_VER, __path, __statbuf);
    }
    use super::stat_h::{stat, _STAT_VER_LINUX, _STAT_VER};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "397:1"]
        pub fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
                       __stat_buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:34"]
pub mod stdlib_h {
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char)
     -> libc::c_int {
        return strtol(__nptr,
                      NULL as *mut libc::c_void as *mut *mut libc::c_char,
                      10 as libc::c_int) as libc::c_int;
    }
    use super::stddef_h::NULL;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "176:17"]
        pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
                      _: libc::c_int) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:35"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "646:15"]
        pub fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
                     _: *mut FILE) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/time.h:36"]
pub mod time_h {
    use super::clock_t_h::clock_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn clock() -> clock_t;
    }
}
#[c2rust::header_src = "/usr/include/bits/time.h:36"]
pub mod bits_time_h {
    #[c2rust::src_loc = "34:9"]
    pub const CLOCKS_PER_SEC: libc::c_int = 1000000 as libc::c_int;
}
pub use self::types_h::{__dev_t, __uid_t, __gid_t, __ino_t, __mode_t,
                        __nlink_t, __off_t, __off64_t, __clock_t, __time_t,
                        __blksize_t, __blkcnt_t, __syscall_slong_t};
pub use self::struct_timespec_h::timespec;
pub use self::stat_h::{stat, _STAT_VER_LINUX, _STAT_VER};
pub use self::stddef_h::{size_t, NULL, NULL_0};
pub use self::clock_t_h::clock_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::expat_external_h::{XML_Char, XML_LChar, XML_Size};
pub use self::expat_h::{XML_Parser, XML_Bool, XML_Status,
                        XML_STATUS_SUSPENDED, XML_STATUS_OK, XML_STATUS_ERROR,
                        XML_Error, XML_ERROR_INVALID_ARGUMENT,
                        XML_ERROR_RESERVED_NAMESPACE_URI,
                        XML_ERROR_RESERVED_PREFIX_XMLNS,
                        XML_ERROR_RESERVED_PREFIX_XML, XML_ERROR_SUSPEND_PE,
                        XML_ERROR_FINISHED, XML_ERROR_ABORTED,
                        XML_ERROR_NOT_SUSPENDED, XML_ERROR_SUSPENDED,
                        XML_ERROR_PUBLICID, XML_ERROR_TEXT_DECL,
                        XML_ERROR_XML_DECL, XML_ERROR_INCOMPLETE_PE,
                        XML_ERROR_UNDECLARING_PREFIX,
                        XML_ERROR_UNBOUND_PREFIX,
                        XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING,
                        XML_ERROR_FEATURE_REQUIRES_XML_DTD,
                        XML_ERROR_ENTITY_DECLARED_IN_PE,
                        XML_ERROR_UNEXPECTED_STATE, XML_ERROR_NOT_STANDALONE,
                        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
                        XML_ERROR_UNCLOSED_CDATA_SECTION,
                        XML_ERROR_INCORRECT_ENCODING,
                        XML_ERROR_UNKNOWN_ENCODING,
                        XML_ERROR_MISPLACED_XML_PI,
                        XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF,
                        XML_ERROR_BINARY_ENTITY_REF, XML_ERROR_BAD_CHAR_REF,
                        XML_ERROR_ASYNC_ENTITY,
                        XML_ERROR_RECURSIVE_ENTITY_REF,
                        XML_ERROR_UNDEFINED_ENTITY,
                        XML_ERROR_PARAM_ENTITY_REF,
                        XML_ERROR_JUNK_AFTER_DOC_ELEMENT,
                        XML_ERROR_DUPLICATE_ATTRIBUTE, XML_ERROR_TAG_MISMATCH,
                        XML_ERROR_PARTIAL_CHAR, XML_ERROR_UNCLOSED_TOKEN,
                        XML_ERROR_INVALID_TOKEN, XML_ERROR_NO_ELEMENTS,
                        XML_ERROR_SYNTAX, XML_ERROR_NO_MEMORY, XML_ERROR_NONE,
                        XML_ParserStruct, XML_ParserCreate,
                        XML_ParserCreateNS, XML_ParserReset, XML_Parse,
                        XML_GetErrorCode, XML_GetCurrentLineNumber,
                        XML_GetCurrentColumnNumber, XML_ParserFree,
                        XML_ErrorString};
pub use self::sys_stat_h::{stat, __xstat};
pub use self::stdlib_h::{atoi, strtol, malloc, free, exit};
use self::stdio_h::{stderr, fclose, fopen, fprintf, printf, fread};
use self::time_h::clock;
pub use self::bits_time_h::CLOCKS_PER_SEC;
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
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn usage(mut prog: *const libc::c_char,
                           mut rc: libc::c_int) {
    fprintf(stderr,
            b"usage: %s [-n] filename bufferSize nr_of_loops\n\x00" as
                *const u8 as *const libc::c_char, prog);
    exit(rc);
}
#[c2rust::src_loc = "57:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut XMLBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut XMLBufEnd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut XMLBufPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut fileAttr: stat =
        stat{st_dev: 0,
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
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut nrOfLoops: libc::c_int = 0;
    let mut bufferSize: libc::c_int = 0;
    let mut fileSize: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut isFinal: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut ns: libc::c_int = 0 as libc::c_int;
    let mut tstart: clock_t = 0;
    let mut tend: clock_t = 0;
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
        fprintf(stderr,
                b"could not access file \'%s\'\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset((j + 1 as libc::c_int) as isize));
        return 2 as libc::c_int
    }
    fd =
        fopen(*argv.offset((j + 1 as libc::c_int) as isize),
              b"r\x00" as *const u8 as *const libc::c_char);
    if fd.is_null() {
        fprintf(stderr,
                b"could not open file \'%s\'\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset((j + 1 as libc::c_int) as isize));
        exit(2 as libc::c_int);
    }
    bufferSize = atoi(*argv.offset((j + 2 as libc::c_int) as isize));
    nrOfLoops = atoi(*argv.offset((j + 3 as libc::c_int) as isize));
    if bufferSize <= 0 as libc::c_int || nrOfLoops <= 0 as libc::c_int {
        fprintf(stderr,
                b"buffer size and nr of loops must be greater than zero.\n\x00"
                    as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    XMLBuf = malloc(fileAttr.st_size as libc::c_ulong) as *mut libc::c_char;
    fileSize =
        fread(XMLBuf as *mut libc::c_void,
              ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
              fileAttr.st_size as libc::c_ulong, fd) as libc::c_int;
    fclose(fd);
    if ns != 0 {
        parser =
            XML_ParserCreateNS(NULL_0 as *const XML_Char,
                               '!' as i32 as XML_Char)
    } else { parser = XML_ParserCreate(NULL_0 as *const XML_Char) }
    i = 0 as libc::c_int;
    XMLBufEnd = XMLBuf.offset(fileSize as isize);
    while i < nrOfLoops {
        XMLBufPtr = XMLBuf;
        isFinal = 0 as libc::c_int;
        tstart = clock();
        loop  {
            let mut parseBufferSize: libc::c_int =
                XMLBufEnd.wrapping_offset_from(XMLBufPtr) as libc::c_long as
                    libc::c_int;
            if parseBufferSize <= bufferSize {
                isFinal = 1 as libc::c_int
            } else { parseBufferSize = bufferSize }
            if XML_Parse(parser, XMLBufPtr, parseBufferSize, isFinal) as u64
                   == 0 {
                fprintf(stderr,
                        b"error \'%s\' at line %lu character %lu\n\x00" as
                            *const u8 as *const libc::c_char,
                        XML_ErrorString(XML_GetErrorCode(parser)),
                        XML_GetCurrentLineNumber(parser),
                        XML_GetCurrentColumnNumber(parser));
                free(XMLBuf as *mut libc::c_void);
                XML_ParserFree(parser);
                exit(4 as libc::c_int);
            }
            XMLBufPtr = XMLBufPtr.offset(bufferSize as isize);
            if !(isFinal == 0) { break ; }
        }
        tend = clock();
        cpuTime +=
            (tend - tstart) as libc::c_double /
                CLOCKS_PER_SEC as libc::c_double;
        XML_ParserReset(parser, NULL_0 as *const XML_Char);
        i += 1
    }
    XML_ParserFree(parser);
    free(XMLBuf as *mut libc::c_void);
    printf(b"%d loops, with buffer size %d. Average time per loop: %f\n\x00"
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
