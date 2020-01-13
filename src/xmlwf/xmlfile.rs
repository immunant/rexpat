// =============== BEGIN xmlfile_h ================
pub const XML_MAP_FILE: libc::c_int = 0o1 as libc::c_int;

pub const XML_EXTERNAL_ENTITIES: libc::c_int = 0o2 as libc::c_int;
use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::expat_external_h::XML_Char;
pub use crate::expat_external_h::XML_LChar;
pub use crate::expat_external_h::XML_Size;
pub use crate::expat_h::XML_Error;
pub use crate::expat_h::XML_ExternalEntityRefHandler;
pub use crate::expat_h::XML_GetErrorColumnNumber;
pub use crate::expat_h::XML_GetErrorLineNumber;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_ParserStruct;
pub use crate::expat_h::XML_Status;
pub use crate::expat_h::XML_ERROR_ABORTED;
pub use crate::expat_h::XML_ERROR_ASYNC_ENTITY;
pub use crate::expat_h::XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_BAD_CHAR_REF;
pub use crate::expat_h::XML_ERROR_BINARY_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
pub use crate::expat_h::XML_ERROR_DUPLICATE_ATTRIBUTE;
pub use crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
pub use crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
pub use crate::expat_h::XML_ERROR_FEATURE_REQUIRES_XML_DTD;
pub use crate::expat_h::XML_ERROR_FINISHED;
pub use crate::expat_h::XML_ERROR_INCOMPLETE_PE;
pub use crate::expat_h::XML_ERROR_INCORRECT_ENCODING;
pub use crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
pub use crate::expat_h::XML_ERROR_INVALID_TOKEN;
pub use crate::expat_h::XML_ERROR_JUNK_AFTER_DOC_ELEMENT;
pub use crate::expat_h::XML_ERROR_MISPLACED_XML_PI;
pub use crate::expat_h::XML_ERROR_NONE;
pub use crate::expat_h::XML_ERROR_NOT_STANDALONE;
pub use crate::expat_h::XML_ERROR_NOT_SUSPENDED;
pub use crate::expat_h::XML_ERROR_NO_ELEMENTS;
pub use crate::expat_h::XML_ERROR_NO_MEMORY;
pub use crate::expat_h::XML_ERROR_PARAM_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_PARTIAL_CHAR;
pub use crate::expat_h::XML_ERROR_PUBLICID;
pub use crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_RESERVED_NAMESPACE_URI;
pub use crate::expat_h::XML_ERROR_RESERVED_PREFIX_XML;
pub use crate::expat_h::XML_ERROR_RESERVED_PREFIX_XMLNS;
pub use crate::expat_h::XML_ERROR_SUSPENDED;
pub use crate::expat_h::XML_ERROR_SUSPEND_PE;
pub use crate::expat_h::XML_ERROR_SYNTAX;
pub use crate::expat_h::XML_ERROR_TAG_MISMATCH;
pub use crate::expat_h::XML_ERROR_TEXT_DECL;
pub use crate::expat_h::XML_ERROR_UNBOUND_PREFIX;
pub use crate::expat_h::XML_ERROR_UNCLOSED_CDATA_SECTION;
pub use crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
pub use crate::expat_h::XML_ERROR_UNDECLARING_PREFIX;
pub use crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
pub use crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
pub use crate::expat_h::XML_ERROR_UNKNOWN_ENCODING;
pub use crate::expat_h::XML_ERROR_XML_DECL;
pub use crate::expat_h::XML_STATUS_ERROR;
pub use crate::expat_h::XML_STATUS_ERROR_0;
pub use crate::expat_h::XML_STATUS_OK;
pub use crate::expat_h::XML_STATUS_SUSPENDED;
pub use crate::src::lib::xmlparse::XML_ErrorString;
pub use crate::src::lib::xmlparse::XML_ExternalEntityParserCreate;
pub use crate::src::lib::xmlparse::XML_GetBuffer;
pub use crate::src::lib::xmlparse::XML_GetCurrentColumnNumber;
pub use crate::src::lib::xmlparse::XML_GetCurrentLineNumber;
pub use crate::src::lib::xmlparse::XML_GetErrorCode;
pub use crate::src::lib::xmlparse::XML_Parse;
pub use crate::src::lib::xmlparse::XML_ParseBuffer;
pub use crate::src::lib::xmlparse::XML_ParserFree;
pub use crate::src::lib::xmlparse::XML_SetBase;
pub use crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler;
pub use crate::stdlib::fprintf;
use crate::stdlib::malloc;
pub use crate::stdlib::ssize_t;
pub use crate::stdlib::stderr;
pub use crate::stdlib::stdout;
use crate::stdlib::strlen;
use ::libc::exit;
use ::libc::free;
use ::libc::open;
pub use ::libc::perror;
use ::libc::strcpy;
use ::libc::strrchr;
pub use ::libc::O_RDONLY;

use crate::src::xmlwf::readfilemap::filemap;
use crate::stdlib::read;
pub use crate::xmltchar_h::ftprintf;
pub use crate::xmltchar_h::tcscpy;
pub use crate::xmltchar_h::tcslen;
pub use crate::xmltchar_h::tcsrchr;
pub use crate::xmltchar_h::topen;
pub use crate::xmltchar_h::tperror;
use ::libc::close;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PROCESS_ARGS {
    pub parser: crate::expat_h::XML_Parser,
    pub retPtr: *mut libc::c_int,
}
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
/* ndef _WIN32 */

pub const O_BINARY: libc::c_int = 0 as libc::c_int;

pub const READ_SIZE: libc::c_int = 1024 as libc::c_int * 8 as libc::c_int;

unsafe extern "C" fn reportError(
    mut parser: crate::expat_h::XML_Parser,
    mut filename: *const crate::expat_external_h::XML_Char,
) {
    let mut code: crate::expat_h::XML_Error = crate::src::lib::xmlparse::XML_GetErrorCode(parser);
    let mut message: *const crate::expat_external_h::XML_Char =
        crate::src::lib::xmlparse::XML_ErrorString(code);
    if !message.is_null() {
        crate::stdlib::fprintf(
            crate::stdlib::stdout,
            b"%s:%lu:%lu: %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
            crate::src::lib::xmlparse::XML_GetCurrentLineNumber(parser),
            crate::src::lib::xmlparse::XML_GetCurrentColumnNumber(parser),
            message,
        );
    } else {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: (unknown message %d)\n\x00" as *const u8 as *const libc::c_char,
            filename,
            code as libc::c_uint,
        );
    };
}
/* This implementation will give problems on files larger than INT_MAX. */

unsafe extern "C" fn processFile(
    mut data: *const libc::c_void,
    mut size: crate::stddef_h::size_t,
    mut filename: *const crate::expat_external_h::XML_Char,
    mut args: *mut libc::c_void,
) {
    let mut parser: crate::expat_h::XML_Parser = (*(args as *mut PROCESS_ARGS)).parser;
    let mut retPtr: *mut libc::c_int = (*(args as *mut PROCESS_ARGS)).retPtr;
    if crate::src::lib::xmlparse::XML_Parse(
        parser,
        data as *const libc::c_char,
        size as libc::c_int,
        1 as libc::c_int,
    ) as libc::c_uint
        == crate::expat_h::XML_STATUS_ERROR_0 as libc::c_uint
    {
        reportError(parser, filename);
        *retPtr = 0 as libc::c_int
    } else {
        *retPtr = 1 as libc::c_int
    };
}
/* _WIN32 */

unsafe extern "C" fn resolveSystemId(
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut toFree: *mut *mut crate::expat_external_h::XML_Char,
) -> *const crate::expat_external_h::XML_Char {
    let mut s: *mut crate::expat_external_h::XML_Char = 0 as *mut crate::expat_external_h::XML_Char;
    *toFree = 0 as *mut crate::expat_external_h::XML_Char;
    if base.is_null() || *systemId as libc::c_int == '/' as i32 {
        return systemId;
    }
    *toFree = crate::stdlib::malloc(
        crate::stdlib::strlen(base)
            .wrapping_add(crate::stdlib::strlen(systemId))
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong
            ),
    ) as *mut crate::expat_external_h::XML_Char;
    if (*toFree).is_null() {
        return systemId;
    }
    ::libc::strcpy(*toFree, base);
    s = *toFree;
    if !::libc::strrchr(s, '/' as i32).is_null() {
        s = ::libc::strrchr(s, '/' as i32).offset(1 as libc::c_int as isize)
    }
    ::libc::strcpy(s, systemId);
    return *toFree;
}

unsafe extern "C" fn externalEntityRefFilemap(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut s: *mut crate::expat_external_h::XML_Char = 0 as *mut crate::expat_external_h::XML_Char;
    let mut filename: *const crate::expat_external_h::XML_Char =
        0 as *const crate::expat_external_h::XML_Char;
    let mut entParser: crate::expat_h::XML_Parser =
        crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
            parser,
            context,
            0 as *const crate::expat_external_h::XML_Char,
        );
    let mut filemapRes: libc::c_int = 0;
    let mut args: PROCESS_ARGS = PROCESS_ARGS {
        parser: 0 as *mut crate::expat_h::XML_ParserStruct,
        retPtr: 0 as *mut libc::c_int,
    };
    args.retPtr = &mut result;
    args.parser = entParser;
    filename = resolveSystemId(base, systemId, &mut s);
    crate::src::lib::xmlparse::XML_SetBase(entParser, filename);
    filemapRes = crate::src::xmlwf::readfilemap::filemap(
        filename,
        Some(
            processFile
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: crate::stddef_h::size_t,
                    _: *const crate::expat_external_h::XML_Char,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        &mut args as *mut PROCESS_ARGS as *mut libc::c_void,
    );
    match filemapRes {
        0 => result = 0 as libc::c_int,
        2 => {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: file too large for memory-mapping, switching to streaming\n\x00" as *const u8
                    as *const libc::c_char,
                filename,
            );
            result = processStream(filename, entParser)
        }
        _ => {}
    }
    ::libc::free(s as *mut libc::c_void);
    crate::src::lib::xmlparse::XML_ParserFree(entParser);
    return result;
}

unsafe extern "C" fn processStream(
    mut filename: *const crate::expat_external_h::XML_Char,
    mut parser: crate::expat_h::XML_Parser,
) -> libc::c_int {
    /* passing NULL for filename means read input from stdin */
    let mut fd: libc::c_int = 0 as libc::c_int; /* 0 is the fileno for stdin */
    if !filename.is_null() {
        fd = ::libc::open(filename, O_BINARY | ::libc::O_RDONLY);
        if fd < 0 as libc::c_int {
            ::libc::perror(filename);
            return 0 as libc::c_int;
        }
    }
    loop {
        let mut nread: libc::c_int = 0;
        let mut buf: *mut libc::c_char =
            crate::src::lib::xmlparse::XML_GetBuffer(parser, READ_SIZE) as *mut libc::c_char;
        if buf.is_null() {
            if !filename.is_null() {
                ::libc::close(fd);
            }
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: out of memory\n\x00" as *const u8 as *const libc::c_char,
                if !filename.is_null() {
                    filename
                } else {
                    b"xmlwf\x00" as *const u8 as *const libc::c_char
                },
            );
            return 0 as libc::c_int;
        }
        nread = crate::stdlib::read(
            fd,
            buf as *mut libc::c_void,
            READ_SIZE as crate::stddef_h::size_t,
        ) as libc::c_int;
        if nread < 0 as libc::c_int {
            ::libc::perror(if !filename.is_null() {
                filename
            } else {
                b"STDIN\x00" as *const u8 as *const libc::c_char
            });
            if !filename.is_null() {
                ::libc::close(fd);
            }
            return 0 as libc::c_int;
        }
        if crate::src::lib::xmlparse::XML_ParseBuffer(
            parser,
            nread,
            (nread == 0 as libc::c_int) as libc::c_int,
        ) as libc::c_uint
            == crate::expat_h::XML_STATUS_ERROR_0 as libc::c_uint
        {
            reportError(
                parser,
                if !filename.is_null() {
                    filename
                } else {
                    b"STDIN\x00" as *const u8 as *const libc::c_char
                },
            );
            if !filename.is_null() {
                ::libc::close(fd);
            }
            return 0 as libc::c_int;
        }
        if !(nread == 0 as libc::c_int) {
            continue;
        }
        if !filename.is_null() {
            ::libc::close(fd);
        }
        break;
    }
    return 1 as libc::c_int;
}

unsafe extern "C" fn externalEntityRefStream(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> libc::c_int {
    let mut s: *mut crate::expat_external_h::XML_Char = 0 as *mut crate::expat_external_h::XML_Char;
    let mut filename: *const crate::expat_external_h::XML_Char =
        0 as *const crate::expat_external_h::XML_Char;
    let mut ret: libc::c_int = 0;
    let mut entParser: crate::expat_h::XML_Parser =
        crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
            parser,
            context,
            0 as *const crate::expat_external_h::XML_Char,
        );
    filename = resolveSystemId(base, systemId, &mut s);
    crate::src::lib::xmlparse::XML_SetBase(entParser, filename);
    ret = processStream(filename, entParser);
    ::libc::free(s as *mut libc::c_void);
    crate::src::lib::xmlparse::XML_ParserFree(entParser);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn XML_ProcessFile(
    mut parser: crate::expat_h::XML_Parser,
    mut filename: *const crate::expat_external_h::XML_Char,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if crate::src::lib::xmlparse::XML_SetBase(parser, filename) as u64 == 0 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: out of memory\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        ::libc::exit(1 as libc::c_int);
    }
    if flags & crate::src::xmlwf::xmlfile::XML_EXTERNAL_ENTITIES as libc::c_uint != 0 {
        crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler(
            parser,
            if flags & crate::src::xmlwf::xmlfile::XML_MAP_FILE as libc::c_uint != 0 {
                Some(
                    externalEntityRefFilemap
                        as unsafe extern "C" fn(
                            _: crate::expat_h::XML_Parser,
                            _: *const crate::expat_external_h::XML_Char,
                            _: *const crate::expat_external_h::XML_Char,
                            _: *const crate::expat_external_h::XML_Char,
                            _: *const crate::expat_external_h::XML_Char,
                        ) -> libc::c_int,
                )
            } else {
                Some(
                    externalEntityRefStream
                        as unsafe extern "C" fn(
                            _: crate::expat_h::XML_Parser,
                            _: *const crate::expat_external_h::XML_Char,
                            _: *const crate::expat_external_h::XML_Char,
                            _: *const crate::expat_external_h::XML_Char,
                            _: *const crate::expat_external_h::XML_Char,
                        ) -> libc::c_int,
                )
            },
        );
    }
    if flags & crate::src::xmlwf::xmlfile::XML_MAP_FILE as libc::c_uint != 0 {
        let mut filemapRes: libc::c_int = 0;
        let mut args: PROCESS_ARGS = PROCESS_ARGS {
            parser: 0 as *mut crate::expat_h::XML_ParserStruct,
            retPtr: 0 as *mut libc::c_int,
        };
        args.retPtr = &mut result;
        args.parser = parser;
        filemapRes = crate::src::xmlwf::readfilemap::filemap(
            filename,
            Some(
                processFile
                    as unsafe extern "C" fn(
                        _: *const libc::c_void,
                        _: crate::stddef_h::size_t,
                        _: *const crate::expat_external_h::XML_Char,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            &mut args as *mut PROCESS_ARGS as *mut libc::c_void,
        );
        match filemapRes {
            0 => result = 0 as libc::c_int,
            2 => {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s: file too large for memory-mapping, switching to streaming\n\x00"
                        as *const u8 as *const libc::c_char,
                    filename,
                );
                result = processStream(filename, parser)
            }
            _ => {}
        }
    } else {
        result = processStream(filename, parser)
    }
    return result;
}
