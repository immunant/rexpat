use crate::readfilemap::filemap;
use crate::stdlib::{malloc, read, strlen};
use ::libc::{close, exit, free, open, strcpy, strrchr};
use libc::{c_char, c_int, c_uint, c_ulong, c_void};
pub const XML_MAP_FILE: c_int = 0o1i32;

pub const XML_EXTERNAL_ENTITIES: c_int = 0o2i32;

pub use crate::expat_external_h::{XML_Char, XML_LChar, XML_Size};
pub use crate::expat_h::{
    XML_Error, XML_ExternalEntityRefHandler, XML_GetErrorColumnNumber, XML_GetErrorLineNumber,
    XML_Parser, XML_ParserStruct, XML_Status, XML_ERROR_ABORTED, XML_ERROR_ASYNC_ENTITY,
    XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF, XML_ERROR_BAD_CHAR_REF, XML_ERROR_BINARY_ENTITY_REF,
    XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING, XML_ERROR_DUPLICATE_ATTRIBUTE,
    XML_ERROR_ENTITY_DECLARED_IN_PE, XML_ERROR_EXTERNAL_ENTITY_HANDLING,
    XML_ERROR_FEATURE_REQUIRES_XML_DTD, XML_ERROR_FINISHED, XML_ERROR_INCOMPLETE_PE,
    XML_ERROR_INCORRECT_ENCODING, XML_ERROR_INVALID_ARGUMENT, XML_ERROR_INVALID_TOKEN,
    XML_ERROR_JUNK_AFTER_DOC_ELEMENT, XML_ERROR_MISPLACED_XML_PI, XML_ERROR_NONE,
    XML_ERROR_NOT_STANDALONE, XML_ERROR_NOT_SUSPENDED, XML_ERROR_NO_ELEMENTS, XML_ERROR_NO_MEMORY,
    XML_ERROR_PARAM_ENTITY_REF, XML_ERROR_PARTIAL_CHAR, XML_ERROR_PUBLICID,
    XML_ERROR_RECURSIVE_ENTITY_REF, XML_ERROR_RESERVED_NAMESPACE_URI,
    XML_ERROR_RESERVED_PREFIX_XML, XML_ERROR_RESERVED_PREFIX_XMLNS, XML_ERROR_SUSPENDED,
    XML_ERROR_SUSPEND_PE, XML_ERROR_SYNTAX, XML_ERROR_TAG_MISMATCH, XML_ERROR_TEXT_DECL,
    XML_ERROR_UNBOUND_PREFIX, XML_ERROR_UNCLOSED_CDATA_SECTION, XML_ERROR_UNCLOSED_TOKEN,
    XML_ERROR_UNDECLARING_PREFIX, XML_ERROR_UNDEFINED_ENTITY, XML_ERROR_UNEXPECTED_STATE,
    XML_ERROR_UNKNOWN_ENCODING, XML_ERROR_XML_DECL, XML_STATUS_ERROR, XML_STATUS_ERROR_0,
    XML_STATUS_OK, XML_STATUS_SUSPENDED,
};
pub use crate::src::lib::xmlparse::{
    XML_ErrorString, XML_ExternalEntityParserCreate, XML_GetBuffer, XML_GetCurrentColumnNumber,
    XML_GetCurrentLineNumber, XML_GetErrorCode, XML_Parse, XML_ParseBuffer, XML_ParserFree,
    XML_SetBase, XML_SetExternalEntityRefHandler,
};
pub use crate::stddef_h::{size_t, NULL};
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, __ssize_t, fprintf,
    ssize_t, stderr, stdout, FILE, _IO_FILE,
};
pub use crate::xmltchar_h::{ftprintf, tcscpy, tcslen, tcsrchr, topen, tperror};
use ::libc;
pub use ::libc::{perror, O_RDONLY};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PROCESS_ARGS {
    pub parser: XML_Parser,
    pub retPtr: *mut c_int,
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

pub const O_BINARY: c_int = 0i32;

pub const READ_SIZE: c_int = 1024i32 * 8i32;

unsafe extern "C" fn reportError(mut parser: XML_Parser, mut filename: *const XML_Char) {
    let mut code: XML_Error = XML_GetErrorCode(parser);
    let mut message: *const XML_Char = XML_ErrorString(code);
    if !message.is_null() {
        fprintf(
            stdout,
            b"%s:%lu:%lu: %s\n\x00" as *const u8 as *const c_char,
            filename,
            XML_GetCurrentLineNumber(parser),
            XML_GetCurrentColumnNumber(parser),
            message,
        );
    } else {
        fprintf(
            stderr,
            b"%s: (unknown message %d)\n\x00" as *const u8 as *const c_char,
            filename,
            
            code,
        );
    };
}
/* This implementation will give problems on files larger than INT_MAX. */

unsafe extern "C" fn processFile(
    mut data: *const c_void,
    mut size: size_t,
    mut filename: *const XML_Char,
    mut args: *mut c_void,
) {
    let mut parser: XML_Parser = (*(args as *mut PROCESS_ARGS)).parser;
    let mut retPtr: *mut c_int = (*(args as *mut PROCESS_ARGS)).retPtr;
    if  XML_Parse(parser, data as *const c_char, size as c_int, 1i32)
        == XML_STATUS_ERROR_0 as c_uint
    {
        reportError(parser, filename);
        *retPtr = 0i32
    } else {
        *retPtr = 1i32
    };
}
/* _WIN32 */

unsafe extern "C" fn resolveSystemId(
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut toFree: *mut *mut XML_Char,
) -> *const XML_Char {
    let mut s: *mut XML_Char = 0 as *mut XML_Char;
    *toFree = 0 as *mut XML_Char;
    if base.is_null() || *systemId as c_int == '/' as i32 {
        return systemId;
    }
    *toFree = malloc(
        strlen(base)
            .wrapping_add(strlen(systemId))
            .wrapping_add(2u64)
            .wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
    ) as *mut XML_Char;
    if (*toFree).is_null() {
        return systemId;
    }
    strcpy(*toFree, base);
    s = *toFree;
    if !strrchr(s, '/' as i32).is_null() {
        s = strrchr(s, '/' as i32).offset(1isize)
    }
    strcpy(s, systemId);
    return *toFree;
}

unsafe extern "C" fn externalEntityRefFilemap(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut result: c_int = 0;
    let mut s: *mut XML_Char = 0 as *mut XML_Char;
    let mut filename: *const XML_Char = 0 as *const XML_Char;
    let mut entParser: XML_Parser =
        XML_ExternalEntityParserCreate(parser, context, 0 as *const XML_Char);
    let mut filemapRes: c_int = 0;
    let mut args: PROCESS_ARGS = PROCESS_ARGS {
        parser: 0 as *mut XML_ParserStruct,
        retPtr: 0 as *mut c_int,
    };
    args.retPtr = &mut result;
    args.parser = entParser;
    filename = resolveSystemId(base, systemId, &mut s);
    XML_SetBase(entParser, filename);
    filemapRes = filemap(
        filename,
        Some(
            processFile
                as unsafe extern "C" fn(
                    _: *const c_void,
                    _: size_t,
                    _: *const XML_Char,
                    _: *mut c_void,
                ) -> (),
        ),
        &mut args as *mut PROCESS_ARGS as *mut c_void,
    );
    match filemapRes {
        0 => result = 0i32,
        2 => {
            fprintf(
                stderr,
                b"%s: file too large for memory-mapping, switching to streaming\n\x00" as *const u8
                    as *const c_char,
                filename,
            );
            result = processStream(filename, entParser)
        }
        _ => {}
    }
    free(s as *mut c_void);
    XML_ParserFree(entParser);
    return result;
}

unsafe extern "C" fn processStream(mut filename: *const XML_Char, mut parser: XML_Parser) -> c_int {
    /* passing NULL for filename means read intput from stdin */
    let mut fd: c_int = 0i32; /* 0 is the fileno for stdin */
    if !filename.is_null() {
        fd = open(filename, O_BINARY | O_RDONLY);
        if fd < 0i32 {
            perror(filename);
            return 0i32;
        }
    }
    loop {
        let mut nread: c_int = 0;
        let mut buf: *mut c_char = XML_GetBuffer(parser, READ_SIZE) as *mut c_char;
        if buf.is_null() {
            if !filename.is_null() {
                close(fd);
            }
            fprintf(
                stderr,
                b"%s: out of memory\n\x00" as *const u8 as *const c_char,
                if !filename.is_null() {
                    filename
                } else {
                    b"xmlwf\x00" as *const u8 as *const c_char
                },
            );
            return 0i32;
        }
        nread = read(fd, buf as *mut c_void, READ_SIZE as size_t) as c_int;
        if nread < 0i32 {
            perror(if !filename.is_null() {
                filename
            } else {
                b"STDIN\x00" as *const u8 as *const c_char
            });
            if !filename.is_null() {
                close(fd);
            }
            return 0i32;
        }
        if  XML_ParseBuffer(parser, nread, (nread == 0i32) as c_int)
            == XML_STATUS_ERROR_0 as c_uint
        {
            reportError(
                parser,
                if !filename.is_null() {
                    filename
                } else {
                    b"STDIN\x00" as *const u8 as *const c_char
                },
            );
            if !filename.is_null() {
                close(fd);
            }
            return 0i32;
        }
        if !(nread == 0i32) {
            continue;
        }
        if !filename.is_null() {
            close(fd);
        }
        break;
    }
    return 1i32;
}

unsafe extern "C" fn externalEntityRefStream(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut s: *mut XML_Char = 0 as *mut XML_Char;
    let mut filename: *const XML_Char = 0 as *const XML_Char;
    let mut ret: c_int = 0;
    let mut entParser: XML_Parser =
        XML_ExternalEntityParserCreate(parser, context, 0 as *const XML_Char);
    filename = resolveSystemId(base, systemId, &mut s);
    XML_SetBase(entParser, filename);
    ret = processStream(filename, entParser);
    free(s as *mut c_void);
    XML_ParserFree(entParser);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn XML_ProcessFile(
    mut parser: XML_Parser,
    mut filename: *const XML_Char,
    mut flags: c_uint,
) -> c_int {
    let mut result: c_int = 0;
    if XML_SetBase(parser, filename) as u64 == 0 {
        fprintf(
            stderr,
            b"%s: out of memory\x00" as *const u8 as *const c_char,
            filename,
        );
        exit(1i32);
    }
    if flags & XML_EXTERNAL_ENTITIES as c_uint != 0 {
        XML_SetExternalEntityRefHandler(
            parser,
            if flags & XML_MAP_FILE as c_uint != 0 {
                Some(
                    externalEntityRefFilemap
                        as unsafe extern "C" fn(
                            _: XML_Parser,
                            _: *const XML_Char,
                            _: *const XML_Char,
                            _: *const XML_Char,
                            _: *const XML_Char,
                        ) -> c_int,
                )
            } else {
                Some(
                    externalEntityRefStream
                        as unsafe extern "C" fn(
                            _: XML_Parser,
                            _: *const XML_Char,
                            _: *const XML_Char,
                            _: *const XML_Char,
                            _: *const XML_Char,
                        ) -> c_int,
                )
            },
        );
    }
    if flags & XML_MAP_FILE as c_uint != 0 {
        let mut filemapRes: c_int = 0;
        let mut args: PROCESS_ARGS = PROCESS_ARGS {
            parser: 0 as *mut XML_ParserStruct,
            retPtr: 0 as *mut c_int,
        };
        args.retPtr = &mut result;
        args.parser = parser;
        filemapRes = filemap(
            filename,
            Some(
                processFile
                    as unsafe extern "C" fn(
                        _: *const c_void,
                        _: size_t,
                        _: *const XML_Char,
                        _: *mut c_void,
                    ) -> (),
            ),
            &mut args as *mut PROCESS_ARGS as *mut c_void,
        );
        match filemapRes {
            0 => result = 0i32,
            2 => {
                fprintf(
                    stderr,
                    b"%s: file too large for memory-mapping, switching to streaming\n\x00"
                        as *const u8 as *const c_char,
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
