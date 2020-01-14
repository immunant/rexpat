// =============== BEGIN xmltok_h ================
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

/* The following token may be returned by XmlContentTok */

use crate::stdlib::memcpy;
use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void, intptr_t};
pub const XML_TOK_TRAILING_RSQB: c_int = -5; /* ] or ]] at the end of the scan; might be
                                             start of illegal ]]> sequence */

/* The following tokens may be returned by both XmlPrologTok and
   XmlContentTok.
*/
pub const XML_TOK_NONE: c_int = -4; /* The string to be scanned is empty */
pub const XML_TOK_TRAILING_CR: c_int = -3; /* A CR at the end of the scan;
                                            * might be part of CRLF sequence */
pub const XML_TOK_PARTIAL_CHAR: c_int = -2; /* only part of a multibyte sequence */
pub const XML_TOK_PARTIAL: c_int = -1; /* only part of a token */
pub const XML_TOK_INVALID: c_int = 0;

/* The following tokens are returned by XmlContentTok; some are also
   returned by XmlAttributeValueTok, XmlEntityTok, XmlCdataSectionTok.
*/

pub const XML_TOK_START_TAG_WITH_ATTS: c_int = 1;
pub const XML_TOK_START_TAG_NO_ATTS: c_int = 2;
pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: c_int = 3; /* empty element tag <e/> */
pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS: c_int = 4;
pub const XML_TOK_END_TAG: c_int = 5;
pub const XML_TOK_DATA_CHARS: c_int = 6;
pub const XML_TOK_DATA_NEWLINE: c_int = 7;
pub const XML_TOK_CDATA_SECT_OPEN: c_int = 8;
pub const XML_TOK_ENTITY_REF: c_int = 9;
pub const XML_TOK_CHAR_REF: c_int = 10; /* numeric character reference */

/* The following tokens may be returned by both XmlPrologTok and
   XmlContentTok.
*/
pub const XML_TOK_PI: c_int = 11; /* processing instruction */
pub const XML_TOK_XML_DECL: c_int = 12; /* XML decl or text decl */
pub const XML_TOK_COMMENT: c_int = 13;
pub const XML_TOK_BOM: c_int = 14; /* Byte order mark */

/* The following tokens are returned only by XmlPrologTok */
pub const XML_TOK_PROLOG_S: c_int = 15i32;
pub const XML_TOK_DECL_OPEN: c_int = 16; /* <!foo */
pub const XML_TOK_DECL_CLOSE: c_int = 17; /* > */
pub const XML_TOK_NAME: c_int = 18;
pub const XML_TOK_NMTOKEN: c_int = 19;
pub const XML_TOK_POUND_NAME: c_int = 20; /* #name */
pub const XML_TOK_OR: c_int = 21; /* | */
pub const XML_TOK_PERCENT: c_int = 22;
pub const XML_TOK_OPEN_PAREN: c_int = 23;
pub const XML_TOK_CLOSE_PAREN: c_int = 24;
pub const XML_TOK_OPEN_BRACKET: c_int = 25;
pub const XML_TOK_CLOSE_BRACKET: c_int = 26;
pub const XML_TOK_LITERAL: c_int = 27;
pub const XML_TOK_PARAM_ENTITY_REF: c_int = 28;
pub const XML_TOK_INSTANCE_START: c_int = 29;

/* The following occur only in element type declarations */
pub const XML_TOK_NAME_QUESTION: c_int = 30; /* name? */
pub const XML_TOK_NAME_ASTERISK: c_int = 31; /* name* */
pub const XML_TOK_NAME_PLUS: c_int = 32; /* name+ */
pub const XML_TOK_COND_SECT_OPEN: c_int = 33; /* <![ */
pub const XML_TOK_COND_SECT_CLOSE: c_int = 34; /* ]]> */
pub const XML_TOK_CLOSE_PAREN_QUESTION: c_int = 35; /* )? */
pub const XML_TOK_CLOSE_PAREN_ASTERISK: c_int = 36; /* )* */
pub const XML_TOK_CLOSE_PAREN_PLUS: c_int = 37; /* )+ */
pub const XML_TOK_COMMA: c_int = 38;

/* The following token is returned only by XmlAttributeValueTok */
pub const XML_TOK_ATTRIBUTE_VALUE_S: c_int = 39;

/* The following token is returned only by XmlCdataSectionTok */
pub const XML_TOK_CDATA_SECT_CLOSE: c_int = 40;

/* With namespace processing this is returned by XmlPrologTok for a
   name with a colon.
*/
pub const XML_TOK_PREFIXED_NAME: c_int = 41;
pub const XML_TOK_IGNORE_SECT: c_int = 42;
pub const XML_PROLOG_STATE: c_int = 0;
pub const XML_CONTENT_STATE: c_int = 1;

/* The size of the buffer passed to XmlUtf8Encode must be at least this. */
pub const XML_UTF8_ENCODE_MAX: c_int = 4;
/* The size of the buffer passed to XmlUtf16Encode must be at least this. */
pub const XML_UTF16_ENCODE_MAX: c_int = 2;

pub type POSITION = position;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct position {
    /* first line and first column are 0 not 1 */
    pub lineNumber: XML_Size,
    pub columnNumber: XML_Size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ATTRIBUTE {
    pub name: *const c_char,
    pub valuePtr: *const c_char,
    pub valueEnd: *const c_char,
    pub normalized: c_char,
}

pub type ENCODING = encoding;

pub type SCANNER = Option<
    unsafe extern "C" fn(
        _: *const ENCODING,
        _: *const c_char,
        _: *const c_char,
        _: *mut *const c_char,
    ) -> c_int,
>;

pub type XML_Convert_Result = c_uint;
pub const XML_CONVERT_COMPLETED: XML_Convert_Result = 0;
pub const XML_CONVERT_INPUT_INCOMPLETE: XML_Convert_Result = 1;
pub const XML_CONVERT_OUTPUT_EXHAUSTED: XML_Convert_Result = 2; /* and therefore potentially input remaining as well */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encoding {
    pub scanners: [SCANNER; 4],
    pub literalScanners: [SCANNER; 2],
    pub nameMatchesAscii: Option<
        unsafe extern "C" fn(
            _: *const ENCODING,
            _: *const c_char,
            _: *const c_char,
            _: *const c_char,
        ) -> c_int,
    >,
    pub nameLength: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
    pub skipS: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> *const c_char>,
    pub getAtts: Option<
        unsafe extern "C" fn(
            _: *const ENCODING,
            _: *const c_char,
            _: c_int,
            _: *mut ATTRIBUTE,
        ) -> c_int,
    >,
    pub charRefNumber: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
    pub predefinedEntityName: Option<
        unsafe extern "C" fn(_: *const ENCODING, _: *const c_char, _: *const c_char) -> c_int,
    >,
    pub updatePosition: Option<
        unsafe extern "C" fn(
            _: *const ENCODING,
            _: *const c_char,
            _: *const c_char,
            _: *mut POSITION,
        ) -> (),
    >,
    pub isPublicId: Option<
        unsafe extern "C" fn(
            _: *const ENCODING,
            _: *const c_char,
            _: *const c_char,
            _: *mut *const c_char,
        ) -> c_int,
    >,
    pub utf8Convert: Option<
        unsafe extern "C" fn(
            _: *const ENCODING,
            _: *mut *const c_char,
            _: *const c_char,
            _: *mut *mut c_char,
            _: *const c_char,
        ) -> XML_Convert_Result,
    >,
    pub utf16Convert: Option<
        unsafe extern "C" fn(
            _: *const ENCODING,
            _: *mut *const c_char,
            _: *const c_char,
            _: *mut *mut c_ushort,
            _: *const c_ushort,
        ) -> XML_Convert_Result,
    >,
    pub minBytesPerChar: c_int,
    pub isUtf8: c_char,
    pub isUtf16: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct INIT_ENCODING {
    pub initEnc: ENCODING,
    pub encPtr: *mut *const ENCODING,
}

pub type CONVERTER = Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int>;

// =============== END xmltok_h ================

pub mod xmltok_impl_c {

    /* vertical bar = "|" */
    /* This file is included!
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
    /* This file is included!
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
    /* This file is included!
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
    /* fall through */
    /* fall through */
    /* fall through */
    /* fall through */
    /* fall through */
    /* fall through */
    /* ptr points to character following "<!-" */
    /* ptr points to character following "<!-" */
    /* ptr points to character following "<!-" */

    use super::{ATTRIBUTE, ENCODING, POSITION, XML_TOK_INSTANCE_START, XML_TOK_PROLOG_S};
    use crate::ascii_h::{
        ASCII_a, ASCII_c, ASCII_e, ASCII_g, ASCII_l, ASCII_m, ASCII_x, ASCII_0, ASCII_1, ASCII_2,
        ASCII_3, ASCII_8, ASCII_9, ASCII_B, ASCII_E, ASCII_F, ASCII_L, ASCII_M, ASCII_X,
    };
    use crate::ascii_h::{
        ASCII_b, ASCII_d, ASCII_f, ASCII_q, ASCII_4, ASCII_5, ASCII_6, ASCII_7, ASCII_A, ASCII_AMP,
        ASCII_APOS, ASCII_C, ASCII_D, ASCII_GT, ASCII_LSQB, ASCII_LT, ASCII_QUOT, ASCII_SPACE,
        ASCII_T,
    };
    use crate::expat_external_h::XML_Size;
    use crate::stddef_h::size_t;
    use crate::xmltok_h::{
        XML_TOK_ATTRIBUTE_VALUE_S, XML_TOK_CDATA_SECT_CLOSE, XML_TOK_CDATA_SECT_OPEN,
        XML_TOK_CHAR_REF, XML_TOK_CLOSE_BRACKET, XML_TOK_CLOSE_PAREN, XML_TOK_CLOSE_PAREN_ASTERISK,
        XML_TOK_CLOSE_PAREN_PLUS, XML_TOK_CLOSE_PAREN_QUESTION, XML_TOK_COMMA, XML_TOK_COMMENT,
        XML_TOK_COND_SECT_CLOSE, XML_TOK_COND_SECT_OPEN, XML_TOK_DATA_CHARS, XML_TOK_DATA_NEWLINE,
        XML_TOK_DECL_CLOSE, XML_TOK_DECL_OPEN, XML_TOK_EMPTY_ELEMENT_NO_ATTS,
        XML_TOK_EMPTY_ELEMENT_WITH_ATTS, XML_TOK_END_TAG, XML_TOK_ENTITY_REF, XML_TOK_IGNORE_SECT,
        XML_TOK_INVALID, XML_TOK_LITERAL, XML_TOK_NAME, XML_TOK_NAME_ASTERISK, XML_TOK_NAME_PLUS,
        XML_TOK_NAME_QUESTION, XML_TOK_NMTOKEN, XML_TOK_NONE, XML_TOK_OPEN_BRACKET,
        XML_TOK_OPEN_PAREN, XML_TOK_OR, XML_TOK_PARAM_ENTITY_REF, XML_TOK_PARTIAL,
        XML_TOK_PARTIAL_CHAR, XML_TOK_PERCENT, XML_TOK_PI, XML_TOK_POUND_NAME,
        XML_TOK_PREFIXED_NAME, XML_TOK_START_TAG_NO_ATTS, XML_TOK_START_TAG_WITH_ATTS,
        XML_TOK_TRAILING_CR, XML_TOK_TRAILING_RSQB, XML_TOK_XML_DECL,
    };
    use crate::xmltok_impl_c::{
        inName, inName_0, inName_1, inValue, inValue_0, inValue_1, other, other_0, other_1,
    };
    use crate::xmltok_impl_h::{C2RustUnnamed_2, BT_APOS, BT_EQUALS, BT_LF, BT_QUOT};
    use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong};
    pub unsafe extern "C" fn big2_scanComment(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            if !(*ptr.offset(0isize) as c_int == 0i32
                && *ptr.offset(1isize) as c_int == 0x2di32)
            {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            ptr = ptr.offset(2isize);
            while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
                match if *ptr.offset(0isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    5 => {
                        if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                            return XML_TOK_PARTIAL_CHAR;
                        }
                        ptr = ptr.offset(2isize)
                    }
                    6 => {
                        if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                            return XML_TOK_PARTIAL_CHAR;
                        }
                        ptr = ptr.offset(3isize)
                    }
                    7 => {
                        if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                            return XML_TOK_PARTIAL_CHAR;
                        }
                        ptr = ptr.offset(4isize)
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    27 => {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(0isize) as c_int == 0i32
                            && *ptr.offset(1isize) as c_int == 0x2di32
                        {
                            ptr = ptr.offset(2isize);
                            if !(end.wrapping_offset_from(ptr) as c_long
                                >= (1i32 * 2i32) as c_long)
                            {
                                return XML_TOK_PARTIAL;
                            }
                            if !(*ptr.offset(0isize) as c_int == 0i32
                                && *ptr.offset(1isize) as c_int == 0x3ei32)
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            *nextTokPtr = ptr.offset(2isize);
                            return XML_TOK_COMMENT;
                        }
                    }
                    _ => ptr = ptr.offset(2isize),
                }
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_scanComment(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            if !(*ptr.offset(1isize) as c_int == 0i32
                && *ptr.offset(0isize) as c_int == 0x2di32)
            {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            ptr = ptr.offset(2isize);
            while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
                match if *ptr.offset(1isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    5 => {
                        if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                            return XML_TOK_PARTIAL_CHAR;
                        }
                        ptr = ptr.offset(2isize)
                    }
                    6 => {
                        if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                            return XML_TOK_PARTIAL_CHAR;
                        }
                        ptr = ptr.offset(3isize)
                    }
                    7 => {
                        if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                            return XML_TOK_PARTIAL_CHAR;
                        }
                        ptr = ptr.offset(4isize)
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    27 => {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(1isize) as c_int == 0i32
                            && *ptr.offset(0isize) as c_int == 0x2di32
                        {
                            ptr = ptr.offset(2isize);
                            if !(end.wrapping_offset_from(ptr) as c_long
                                >= (1i32 * 2i32) as c_long)
                            {
                                return XML_TOK_PARTIAL;
                            }
                            if !(*ptr.offset(1isize) as c_int == 0i32
                                && *ptr.offset(0isize) as c_int == 0x3ei32)
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            *nextTokPtr = ptr.offset(2isize);
                            return XML_TOK_COMMENT;
                        }
                    }
                    _ => ptr = ptr.offset(2isize),
                }
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn normal_scanComment(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            if !(*ptr as c_int == 0x2di32) {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            ptr = ptr.offset(1isize);
            while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
                match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                    5 => {
                        if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                            return XML_TOK_PARTIAL_CHAR;
                        }
                        if (*(enc as *const normal_encoding))
                            .isInvalid2
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                        {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                        ptr = ptr.offset(2isize)
                    }
                    6 => {
                        if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                            return XML_TOK_PARTIAL_CHAR;
                        }
                        if (*(enc as *const normal_encoding))
                            .isInvalid3
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                        {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                        ptr = ptr.offset(3isize)
                    }
                    7 => {
                        if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                            return XML_TOK_PARTIAL_CHAR;
                        }
                        if (*(enc as *const normal_encoding))
                            .isInvalid4
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                        {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                        ptr = ptr.offset(4isize)
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    27 => {
                        ptr = ptr.offset(1isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 1i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr as c_int == 0x2di32 {
                            ptr = ptr.offset(1isize);
                            if !(end.wrapping_offset_from(ptr) as c_long
                                >= (1i32 * 1i32) as c_long)
                            {
                                return XML_TOK_PARTIAL;
                            }
                            if !(*ptr as c_int == 0x3ei32) {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            *nextTokPtr = ptr.offset(1isize);
                            return XML_TOK_COMMENT;
                        }
                    }
                    _ => ptr = ptr.offset(1isize),
                }
            }
        }
        return XML_TOK_PARTIAL;
    }
    /* ptr points to character following "<!" */
    /* ptr points to character following "<!" */
    /* ptr points to character following "<!" */

    pub unsafe extern "C" fn normal_scanDecl(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
            27 => return normal_scanComment(enc, ptr.offset(1isize), end, nextTokPtr),
            20 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_COND_SECT_OPEN;
            }
            22 | 24 => ptr = ptr.offset(1isize),
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            's_151: {
                match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                    30 => {
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (2i32 * 1i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        /* don't allow <!ENTITY% foo "whatever"> */
                        /* don't allow <!ENTITY% foo "whatever"> */
                        /* don't allow <!ENTITY% foo "whatever"> */
                        match (*(enc as *mut normal_encoding)).type_0
                            [*ptr.offset(1isize) as c_uchar as usize]
                            as c_int
                        {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(1isize);
                        break 's_151;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                /* fall through */
                /* fall through */
                /* fall through */
                *nextTokPtr = ptr;
                return XML_TOK_DECL_OPEN;
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_scanDecl(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        match if *ptr.offset(1isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            27 => {
                return little2_scanComment(enc, ptr.offset(2isize), end, nextTokPtr)
            }
            20 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_COND_SECT_OPEN;
            }
            22 | 24 => ptr = ptr.offset(2isize),
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            's_151: {
                match if *ptr.offset(1isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    30 => {
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (2i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        match if *ptr.offset(2isize).offset(1isize)
                            as c_int
                            == 0i32
                        {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(2isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(2isize).offset(1isize),
                                *ptr.offset(2isize).offset(0isize),
                            )
                        } {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(2isize);
                        break 's_151;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_DECL_OPEN;
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn big2_scanDecl(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        match if *ptr.offset(0isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0
                [*ptr.offset(1isize) as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            27 => return big2_scanComment(enc, ptr.offset(2isize), end, nextTokPtr),
            20 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_COND_SECT_OPEN;
            }
            22 | 24 => ptr = ptr.offset(2isize),
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            's_151: {
                match if *ptr.offset(0isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    30 => {
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (2i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        match if *ptr.offset(2isize).offset(0isize)
                            as c_int
                            == 0i32
                        {
                            (*(enc as *mut normal_encoding)).type_0[*ptr
                                .offset(2isize)
                                .offset(1isize)
                                as c_uchar
                                as usize] as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(2isize).offset(0isize),
                                *ptr.offset(2isize).offset(1isize),
                            )
                        } {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(2isize);
                        break 's_151;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_DECL_OPEN;
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_checkPiTarget(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut tokPtr: *mut c_int,
    ) -> c_int {
        let mut upper: c_int = 0i32;
        *tokPtr = XML_TOK_PI;
        if end.wrapping_offset_from(ptr) as c_long != (2i32 * 3i32) as c_long {
            return 1i32;
        }
        match if *ptr.offset(1isize) as c_int == 0i32 {
            *ptr.offset(0isize)
        } else {
            -1
        } {
            ASCII_x => {}
            ASCII_X => upper = 1i32,
            _ => return 1i32,
        }
        ptr = ptr.offset(2isize);
        match if *ptr.offset(1isize) as c_int == 0i32 {
            *ptr.offset(0isize)
        } else {
            -1
        } {
            ASCII_m => {}
            ASCII_M => upper = 1i32,
            _ => return 1i32,
        }
        ptr = ptr.offset(2isize);
        match if *ptr.offset(1isize) as c_int == 0i32 {
            *ptr.offset(0isize)
        } else {
            -1
        } {
            ASCII_l => {}
            ASCII_L => upper = 1i32,
            _ => return 1i32,
        }
        if upper != 0 {
            return 0i32;
        }
        *tokPtr = XML_TOK_XML_DECL;
        return 1i32;
    }

    pub unsafe extern "C" fn big2_checkPiTarget(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut tokPtr: *mut c_int,
    ) -> c_int {
        let mut upper: c_int = 0i32;
        *tokPtr = XML_TOK_PI;
        if end.wrapping_offset_from(ptr) as c_long != (2i32 * 3i32) as c_long {
            return 1i32;
        }
        match if *ptr.offset(0isize) as c_int == 0i32 {
            *ptr.offset(1isize)
        } else {
            -1
        } {
            ASCII_x => {}
            ASCII_X => upper = 1i32,
            _ => return 1i32,
        }
        ptr = ptr.offset(2isize);
        match if *ptr.offset(0isize) as c_int == 0i32 {
            *ptr.offset(1isize)
        } else {
            -1
        } {
            ASCII_m => {}
            ASCII_M => upper = 1i32,
            _ => return 1i32,
        }
        ptr = ptr.offset(2isize);
        match if *ptr.offset(0isize) as c_int == 0i32 {
            *ptr.offset(1isize)
        } else {
            -1
        } {
            ASCII_l => {}
            ASCII_L => upper = 1i32,
            _ => return 1i32,
        }
        if upper != 0 {
            return 0i32;
        }
        *tokPtr = XML_TOK_XML_DECL;
        return 1i32;
    }

    pub unsafe extern "C" fn normal_checkPiTarget(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut tokPtr: *mut c_int,
    ) -> c_int {
        let mut upper: c_int = 0i32;
        *tokPtr = XML_TOK_PI;
        if end.wrapping_offset_from(ptr) as c_long != (1i32 * 3i32) as c_long {
            return 1i32;
        }
        match *ptr {
            ASCII_x => {}
            ASCII_X => upper = 1i32,
            _ => return 1i32,
        }
        ptr = ptr.offset(1isize);
        match *ptr {
            ASCII_m => {}
            ASCII_M => upper = 1i32,
            _ => return 1i32,
        }
        ptr = ptr.offset(1isize);
        match *ptr {
            ASCII_l => {}
            ASCII_L => upper = 1i32,
            _ => return 1i32,
        }
        if upper != 0 {
            return 0i32;
        }
        *tokPtr = XML_TOK_XML_DECL;
        return 1i32;
    }
    /* ptr points to character following "<?" */
    /* ptr points to character following "<?" */
    /* ptr points to character following "<?" */

    pub unsafe extern "C" fn little2_scanPi(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut tok: c_int = 0;
        let mut target: *const c_char = ptr;
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(1isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_32 = 6398034955962673631;
            }
            22 | 24 => {
                current_block_32 = 6398034955962673631;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_32 = 11913429853522160501;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_32 = 11913429853522160501;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_32 = 11913429853522160501;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_32 {
            6398034955962673631 => ptr = ptr.offset(2isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_118: u64;
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_118 = 12311525036306139934;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_118 = 12311525036306139934;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_118 = 18218798608644444571;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_118 = 18218798608644444571;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_118 = 18218798608644444571;
                }
                21 | 9 | 10 => {
                    if little2_checkPiTarget(enc, target, ptr, &mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    while end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long
                    {
                        match if *ptr.offset(1isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        } {
                            5 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(2isize)
                            }
                            6 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(3isize)
                            }
                            7 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(4isize)
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            15 => {
                                ptr = ptr.offset(2isize);
                                if !(end.wrapping_offset_from(ptr) as c_long
                                    >= (1i32 * 2i32) as c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                if *ptr.offset(1isize) as c_int == 0i32
                                    && *ptr.offset(0isize) as c_int == 0x3ei32
                                {
                                    *nextTokPtr = ptr.offset(2isize);
                                    return tok;
                                }
                            }
                            _ => ptr = ptr.offset(2isize),
                        }
                    }
                    return XML_TOK_PARTIAL;
                }
                15 => {
                    if little2_checkPiTarget(enc, target, ptr, &mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(1isize) as c_int == 0i32
                        && *ptr.offset(0isize) as c_int == 0x3ei32
                    {
                        *nextTokPtr = ptr.offset(2isize);
                        return tok;
                    }
                    current_block_118 = 1368869732813945810;
                }
                _ => {
                    current_block_118 = 1368869732813945810;
                }
            }
            match current_block_118 {
                1368869732813945810 =>
                /* fall through */
                /* fall through */
                /* fall through */
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                12311525036306139934 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn big2_scanPi(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut tok: c_int = 0;
        let mut target: *const c_char = ptr;
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(0isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0
                [*ptr.offset(1isize) as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_32 = 13803923597135322110;
            }
            22 | 24 => {
                current_block_32 = 13803923597135322110;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_32 = 11913429853522160501;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_32 = 11913429853522160501;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_32 = 11913429853522160501;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_32 {
            13803923597135322110 => ptr = ptr.offset(2isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_118: u64;
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(0isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_118 = 8820551849686684873;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_118 = 8820551849686684873;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_118 = 18218798608644444571;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_118 = 18218798608644444571;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_118 = 18218798608644444571;
                }
                21 | 9 | 10 => {
                    if big2_checkPiTarget(enc, target, ptr, &mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    while end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long
                    {
                        match if *ptr.offset(0isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        } {
                            5 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(2isize)
                            }
                            6 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(3isize)
                            }
                            7 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(4isize)
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            15 => {
                                ptr = ptr.offset(2isize);
                                if !(end.wrapping_offset_from(ptr) as c_long
                                    >= (1i32 * 2i32) as c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                if *ptr.offset(0isize) as c_int == 0i32
                                    && *ptr.offset(1isize) as c_int == 0x3ei32
                                {
                                    *nextTokPtr = ptr.offset(2isize);
                                    return tok;
                                }
                            }
                            _ => ptr = ptr.offset(2isize),
                        }
                    }
                    return XML_TOK_PARTIAL;
                }
                15 => {
                    if big2_checkPiTarget(enc, target, ptr, &mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(0isize) as c_int == 0i32
                        && *ptr.offset(1isize) as c_int == 0x3ei32
                    {
                        *nextTokPtr = ptr.offset(2isize);
                        return tok;
                    }
                    current_block_118 = 5922746384104648762;
                }
                _ => {
                    current_block_118 = 5922746384104648762;
                }
            }
            match current_block_118 {
                5922746384104648762 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                8820551849686684873 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn normal_scanPi(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut tok: c_int = 0;
        let mut target: *const c_char = ptr;
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_32: u64;
        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
            29 => {
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_32 = 909593556805851584;
            }
            22 | 24 => {
                current_block_32 = 909593556805851584;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt2
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_32 = 11913429853522160501;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt3
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_32 = 11913429853522160501;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt4
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_32 = 11913429853522160501;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_32 {
            909593556805851584 => ptr = ptr.offset(1isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut current_block_118: u64;
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                29 => {
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_118 = 5152980791761092907;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_118 = 5152980791761092907;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_118 = 18218798608644444571;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_118 = 18218798608644444571;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_118 = 18218798608644444571;
                }
                21 | 9 | 10 => {
                    if normal_checkPiTarget(enc, target, ptr, &mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(1isize);
                    while end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long
                    {
                        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                            as c_int
                        {
                            5 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(2isize)
                            }
                            6 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(3isize)
                            }
                            7 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(4isize)
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            15 => {
                                ptr = ptr.offset(1isize);
                                if !(end.wrapping_offset_from(ptr) as c_long
                                    >= (1i32 * 1i32) as c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                if *ptr as c_int == 0x3ei32 {
                                    *nextTokPtr = ptr.offset(1isize);
                                    return tok;
                                }
                            }
                            _ => ptr = ptr.offset(1isize),
                        }
                    }
                    return XML_TOK_PARTIAL;
                }
                15 => {
                    if normal_checkPiTarget(enc, target, ptr, &mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(1isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr as c_int == 0x3ei32 {
                        *nextTokPtr = ptr.offset(1isize);
                        return tok;
                    }
                    current_block_118 = 12516980621136662590;
                }
                _ => {
                    current_block_118 = 12516980621136662590;
                }
            }
            match current_block_118 {
                12516980621136662590 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                5152980791761092907 => ptr = ptr.offset(1isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn big2_scanCdataSection(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        pub static mut CDATA_LSQB: [c_char; 6] = [
            
            ASCII_C,
            
            ASCII_D,
            
            ASCII_A,
            
            ASCII_T,
            
            ASCII_A,
            
            ASCII_LSQB,
        ];
        let mut i: c_int = 0;
        /* CDATA[ */
        /* CDATA[ */
        /* CDATA[ */
        if !(end.wrapping_offset_from(ptr) as c_long >= (6i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        i = 0i32;
        while i < 6i32 {
            if !(*ptr.offset(0isize) as c_int == 0i32
                && *ptr.offset(1isize) as c_int == CDATA_LSQB[i as usize] as c_int)
            {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            i += 1;
            ptr = ptr.offset(2isize)
        }
        *nextTokPtr = ptr;
        return XML_TOK_CDATA_SECT_OPEN;
    }

    pub unsafe extern "C" fn normal_scanCdataSection(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        pub static mut CDATA_LSQB: [c_char; 6] = [
            
            ASCII_C,
            
            ASCII_D,
            
            ASCII_A,
            
            ASCII_T,
            
            ASCII_A,
            
            ASCII_LSQB,
        ];
        let mut i: c_int = 0;
        if !(end.wrapping_offset_from(ptr) as c_long >= (6i32 * 1i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        i = 0i32;
        while i < 6i32 {
            if !(*ptr as c_int == CDATA_LSQB[i as usize] as c_int) {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            i += 1;
            ptr = ptr.offset(1isize)
        }
        *nextTokPtr = ptr;
        return XML_TOK_CDATA_SECT_OPEN;
    }

    pub unsafe extern "C" fn little2_scanCdataSection(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        pub static mut CDATA_LSQB: [c_char; 6] = [
            
            ASCII_C,
            
            ASCII_D,
            
            ASCII_A,
            
            ASCII_T,
            
            ASCII_A,
            
            ASCII_LSQB,
        ];
        let mut i: c_int = 0;
        if !(end.wrapping_offset_from(ptr) as c_long >= (6i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        i = 0i32;
        while i < 6i32 {
            if !(*ptr.offset(1isize) as c_int == 0i32
                && *ptr.offset(0isize) as c_int == CDATA_LSQB[i as usize] as c_int)
            {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            i += 1;
            ptr = ptr.offset(2isize)
        }
        *nextTokPtr = ptr;
        return XML_TOK_CDATA_SECT_OPEN;
    }

    pub unsafe extern "C" fn big2_cdataSectionTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (2i32 - 1i32) as c_ulong != 0 {
                n &= !(2i32 - 1i32) as c_ulong;
                if n == 0u64 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        match if *ptr.offset(0isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0
                [*ptr.offset(1isize) as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                if *ptr.offset(0isize) as c_int == 0i32
                    && *ptr.offset(1isize) as c_int == 0x5di32
                {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if !(*ptr.offset(0isize) as c_int == 0i32
                        && *ptr.offset(1isize) as c_int == 0x3ei32)
                    {
                        ptr = ptr.offset(-(2isize))
                    } else {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CDATA_SECT_CLOSE;
                    }
                }
            }
            9 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                if (if *ptr.offset(0isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                }) == BT_LF as c_int
                {
                    ptr = ptr.offset(2isize)
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE;
            }
            10 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DATA_NEWLINE;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(2isize)
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(3isize)
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(4isize)
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            _ => ptr = ptr.offset(2isize),
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(2isize)
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(3isize)
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(4isize)
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => ptr = ptr.offset(2isize),
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }

    pub unsafe extern "C" fn little2_cdataSectionTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (2i32 - 1i32) as c_ulong != 0 {
                n &= !(2i32 - 1i32) as c_ulong;
                if n == 0u64 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        match if *ptr.offset(1isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                if *ptr.offset(1isize) as c_int == 0i32
                    && *ptr.offset(0isize) as c_int == 0x5di32
                {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if !(*ptr.offset(1isize) as c_int == 0i32
                        && *ptr.offset(0isize) as c_int == 0x3ei32)
                    {
                        ptr = ptr.offset(-(2isize))
                    } else {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CDATA_SECT_CLOSE;
                    }
                }
            }
            9 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                if (if *ptr.offset(1isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                }) == BT_LF as c_int
                {
                    ptr = ptr.offset(2isize)
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE;
            }
            10 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DATA_NEWLINE;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(2isize)
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(3isize)
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(4isize)
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            _ => ptr = ptr.offset(2isize),
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(2isize)
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(3isize)
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(4isize)
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => ptr = ptr.offset(2isize),
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }

    pub unsafe extern "C" fn normal_cdataSectionTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 1i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (1i32 - 1i32) as c_ulong != 0 {
                n &= !(1i32 - 1i32) as c_ulong;
                if n == 0u64 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
            4 => {
                ptr = ptr.offset(1isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                if *ptr as c_int == 0x5di32 {
                    ptr = ptr.offset(1isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if !(*ptr as c_int == 0x3ei32) {
                        ptr = ptr.offset(-(1isize))
                    } else {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CDATA_SECT_CLOSE;
                    }
                }
            }
            9 => {
                ptr = ptr.offset(1isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                if (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                    == BT_LF as c_int
                {
                    ptr = ptr.offset(1isize)
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE;
            }
            10 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_DATA_NEWLINE;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize)
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize)
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize)
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            _ => ptr = ptr.offset(1isize),
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid2
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(2isize)
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid3
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(3isize)
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid4
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(4isize)
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => ptr = ptr.offset(1isize),
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
    /* ptr points to character following "</" */
    /* ptr points to character following "</" */
    /* ptr points to character following "</" */

    pub unsafe extern "C" fn normal_scanEndTag(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_32: u64;
        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
            29 => {
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_32 = 18181195574538971450;
            }
            22 | 24 => {
                current_block_32 = 18181195574538971450;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt2
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_32 = 5494826135382683477;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt3
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_32 = 5494826135382683477;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt4
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_32 = 5494826135382683477;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_32 {
            18181195574538971450 => ptr = ptr.offset(1isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut current_block_73: u64;
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                29 => {
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_73 = 17691760011438675887;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_73 = 17691760011438675887;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_73 = 10809827304263610514;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_73 = 10809827304263610514;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_73 = 10809827304263610514;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(1isize);
                    while end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long
                    {
                        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                            as c_int
                        {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(1isize);
                                return XML_TOK_END_TAG;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        ptr = ptr.offset(1isize)
                    }
                    return XML_TOK_PARTIAL;
                }
                23 => {
                    /* no need to check qname syntax here,
                    since end-tag must match exactly */
                    /* no need to check qname syntax here,
                    since end-tag must match exactly */
                    /* no need to check qname syntax here,
                    since end-tag must match exactly */
                    ptr = ptr.offset(1isize);
                    current_block_73 = 10809827304263610514;
                }
                11 => {
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_END_TAG;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_73 {
                17691760011438675887 => ptr = ptr.offset(1isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn big2_scanEndTag(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(0isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0
                [*ptr.offset(1isize) as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_32 = 10497796554217858559;
            }
            22 | 24 => {
                current_block_32 = 10497796554217858559;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_32 = 5494826135382683477;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_32 = 5494826135382683477;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_32 = 5494826135382683477;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_32 {
            10497796554217858559 => ptr = ptr.offset(2isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_73: u64;
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(0isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_73 = 15122140455506841825;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_73 = 15122140455506841825;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_73 = 10809827304263610514;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_73 = 10809827304263610514;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_73 = 10809827304263610514;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2isize);
                    while end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long
                    {
                        match if *ptr.offset(0isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        } {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_END_TAG;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        ptr = ptr.offset(2isize)
                    }
                    return XML_TOK_PARTIAL;
                }
                23 => {
                    ptr = ptr.offset(2isize);
                    current_block_73 = 10809827304263610514;
                }
                11 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_END_TAG;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_73 {
                15122140455506841825 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_scanEndTag(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(1isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_32 = 188456398757501463;
            }
            22 | 24 => {
                current_block_32 = 188456398757501463;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_32 = 5494826135382683477;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_32 = 5494826135382683477;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_32 = 5494826135382683477;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_32 {
            188456398757501463 => ptr = ptr.offset(2isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_73: u64;
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_73 = 8542218964871577364;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_73 = 8542218964871577364;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_73 = 10809827304263610514;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_73 = 10809827304263610514;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_73 = 10809827304263610514;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2isize);
                    while end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long
                    {
                        match if *ptr.offset(1isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        } {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_END_TAG;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        ptr = ptr.offset(2isize)
                    }
                    return XML_TOK_PARTIAL;
                }
                23 => {
                    ptr = ptr.offset(2isize);
                    current_block_73 = 10809827304263610514;
                }
                11 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_END_TAG;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_73 {
                8542218964871577364 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
    /* ptr points to character following "&#X" */
    /* ptr points to character following "&#X" */
    /* ptr points to character following "&#X" */

    pub unsafe extern "C" fn little2_scanHexCharRef(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr = ptr.offset(2isize);
            while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
                match if *ptr.offset(1isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                ptr = ptr.offset(2isize)
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn big2_scanHexCharRef(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr = ptr.offset(2isize);
            while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
                match if *ptr.offset(0isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                ptr = ptr.offset(2isize)
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn normal_scanHexCharRef(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr = ptr.offset(1isize);
            while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
                match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                ptr = ptr.offset(1isize)
            }
        }
        return XML_TOK_PARTIAL;
    }
    /* ptr points to character following "&#" */
    /* ptr points to character following "&#" */
    /* ptr points to character following "&#" */

    pub unsafe extern "C" fn little2_scanCharRef(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            if *ptr.offset(1isize) as c_int == 0i32
                && *ptr.offset(0isize) as c_int == 0x78i32
            {
                return little2_scanHexCharRef(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr = ptr.offset(2isize);
            while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
                match if *ptr.offset(1isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                ptr = ptr.offset(2isize)
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn big2_scanCharRef(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            if *ptr.offset(0isize) as c_int == 0i32
                && *ptr.offset(1isize) as c_int == 0x78i32
            {
                return big2_scanHexCharRef(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr = ptr.offset(2isize);
            while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
                match if *ptr.offset(0isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                ptr = ptr.offset(2isize)
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn normal_scanCharRef(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            if *ptr as c_int == 0x78i32 {
                return normal_scanHexCharRef(
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr = ptr.offset(1isize);
            while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
                match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                ptr = ptr.offset(1isize)
            }
        }
        return XML_TOK_PARTIAL;
    }
    /* ptr points to character following "&" */
    /* ptr points to character following "&" */
    /* ptr points to character following "&" */

    pub unsafe extern "C" fn normal_scanRef(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_33: u64;
        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
            29 => {
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_33 = 15143551936184752825;
            }
            22 | 24 => {
                current_block_33 = 15143551936184752825;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt2
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_33 = 18377268871191777778;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt3
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_33 = 18377268871191777778;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt4
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_33 = 18377268871191777778;
            }
            19 => return normal_scanCharRef(enc, ptr.offset(1isize), end, nextTokPtr),
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_33 {
            15143551936184752825 => ptr = ptr.offset(1isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut current_block_64: u64;
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                29 => {
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_64 = 12579958577185958249;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_64 = 12579958577185958249;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_64 = 1134115459065347084;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_64 = 1134115459065347084;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_64 = 1134115459065347084;
                }
                18 => {
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_ENTITY_REF;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_64 {
                12579958577185958249 => ptr = ptr.offset(1isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_scanRef(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_33: u64;
        match if *ptr.offset(1isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_33 = 8695478541019159461;
            }
            22 | 24 => {
                current_block_33 = 8695478541019159461;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_33 = 18377268871191777778;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_33 = 18377268871191777778;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_33 = 18377268871191777778;
            }
            19 => {
                return little2_scanCharRef(enc, ptr.offset(2isize), end, nextTokPtr)
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_33 {
            8695478541019159461 => ptr = ptr.offset(2isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_64: u64;
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_64 = 16314533736538551187;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_64 = 16314533736538551187;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_64 = 1134115459065347084;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_64 = 1134115459065347084;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_64 = 1134115459065347084;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_ENTITY_REF;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_64 {
                16314533736538551187 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn big2_scanRef(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_33: u64;
        match if *ptr.offset(0isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0
                [*ptr.offset(1isize) as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_33 = 11934390993149424930;
            }
            22 | 24 => {
                current_block_33 = 11934390993149424930;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_33 = 18377268871191777778;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_33 = 18377268871191777778;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_33 = 18377268871191777778;
            }
            19 => return big2_scanCharRef(enc, ptr.offset(2isize), end, nextTokPtr),
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_33 {
            11934390993149424930 => ptr = ptr.offset(2isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_64: u64;
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(0isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_64 = 10291975244637513717;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_64 = 10291975244637513717;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_64 = 1134115459065347084;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_64 = 1134115459065347084;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_64 = 1134115459065347084;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_ENTITY_REF;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_64 {
                10291975244637513717 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
    /* ptr points to character following first character of attribute name */
    /* ptr points to character following first character of attribute name */
    /* ptr points to character following first character of attribute name */

    pub unsafe extern "C" fn big2_scanAtts(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut hadColon: c_int = 0i32;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_186: u64;
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(0isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_186 = 17518983136156868271;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_186 = 17518983136156868271;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_186 = 11099343707781121639;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_186 = 11099343707781121639;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_186 = 11099343707781121639;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut current_block_64: u64;
                    match if *ptr.offset(0isize) as c_int == 0i32 {
                        (*(enc as *mut normal_encoding)).type_0
                            [*ptr.offset(1isize) as c_uchar as usize]
                            as c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0isize),
                            *ptr.offset(1isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages
                                [*ptr.offset(0isize) as c_uchar as usize]
                                as c_int)
                                << 3i32)
                                + (*ptr.offset(1isize) as c_uchar as c_int
                                    >> 5i32))
                                as usize]
                                & (1u32)
                                    << (*ptr.offset(1isize) as c_uchar as c_int
                                        & 0x1fi32)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            current_block_64 = 1844840963447537551;
                        }
                        22 | 24 => {
                            current_block_64 = 1844840963447537551;
                        }
                        5 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2isize);
                            current_block_64 = 317151059986244064;
                        }
                        6 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3isize);
                            current_block_64 = 317151059986244064;
                        }
                        7 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4isize);
                            current_block_64 = 317151059986244064;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match current_block_64 {
                        1844840963447537551 => ptr = ptr.offset(2isize),
                        _ => {}
                    }
                    current_block_186 = 11099343707781121639;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: c_int = 0;
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t = if *ptr.offset(0isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        };
                        if t == BT_EQUALS as c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    current_block_186 = 17167606947040001567;
                }
                14 => {
                    current_block_186 = 17167606947040001567;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_186 {
                17167606947040001567 =>
                /* fall through */
                /* fall through */
                /* fall through */
                {
                    let mut open: c_int = 0;
                    hadColon = 0i32;
                    loop {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        open = if *ptr.offset(0isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        };
                        if open == BT_QUOT as c_int || open == BT_APOS as c_int {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    ptr = ptr.offset(2isize);
                    loop
                    /* in attribute value */
                    /* in attribute value */
                    /* in attribute value */
                    {
                        let mut t_0: c_int = 0;
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t_0 = if *ptr.offset(0isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        };
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(2isize)
                            }
                            6 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(3isize)
                            }
                            7 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(4isize)
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            3 => {
                                let mut tok: c_int = big2_scanRef(
                                    enc,
                                    ptr.offset(2isize),
                                    end,
                                    &mut ptr,
                                );
                                if tok <= 0i32 {
                                    if tok == XML_TOK_INVALID {
                                        *nextTokPtr = ptr
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => ptr = ptr.offset(2isize),
                        }
                    }
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    match if *ptr.offset(0isize) as c_int == 0i32 {
                        (*(enc as *mut normal_encoding)).type_0
                            [*ptr.offset(1isize) as c_uchar as usize]
                            as c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0isize),
                            *ptr.offset(1isize),
                        )
                    } {
                        21 | 9 | 10 => {
                            loop
                            /* ptr points to closing quote */
                            /* ptr points to closing quote */
                            /* ptr points to closing quote */
                            {
                                ptr = ptr.offset(2isize);
                                if !(end.wrapping_offset_from(ptr) as c_long
                                    >= (1i32 * 2i32) as c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                match if *ptr.offset(0isize) as c_int == 0i32 {
                                    (*(enc as *mut normal_encoding)).type_0
                                        [*ptr.offset(1isize) as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(
                                        *ptr.offset(0isize),
                                        *ptr.offset(1isize),
                                    )
                                } {
                                    29 => {
                                        if namingBitmap[(((nmstrtPages
                                            [*ptr.offset(0isize) as c_uchar as usize]
                                            as c_int)
                                            << 3i32)
                                            + (*ptr.offset(1isize) as c_uchar
                                                as c_int
                                                >> 5i32))
                                            as usize]
                                            & (1u32)
                                                << (*ptr.offset(1isize) as c_uchar
                                                    as c_int
                                                    & 0x1fi32)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        current_block_186 = 5783548739092765259;
                                        break;
                                    }
                                    22 | 24 => {
                                        current_block_186 = 5783548739092765259;
                                        break;
                                    }
                                    5 => {
                                        if (end.wrapping_offset_from(ptr) as c_long)
                                            < 2i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0i32 == 0 {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(2isize);
                                        current_block_186 = 11099343707781121639;
                                        break;
                                    }
                                    6 => {
                                        if (end.wrapping_offset_from(ptr) as c_long)
                                            < 3i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0i32 == 0 {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(3isize);
                                        current_block_186 = 11099343707781121639;
                                        break;
                                    }
                                    7 => {
                                        if (end.wrapping_offset_from(ptr) as c_long)
                                            < 4i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0i32 == 0 {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(4isize);
                                        current_block_186 = 11099343707781121639;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        current_block_186 = 8371071221061937948;
                                        break;
                                    }
                                    17 => {
                                        current_block_186 = 1275279814574597502;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                }
                            }
                            match current_block_186 {
                                8371071221061937948 => {}
                                1275279814574597502 => {}
                                11099343707781121639 => {}
                                _ => {
                                    ptr = ptr.offset(2isize);
                                    current_block_186 = 11099343707781121639;
                                }
                            }
                        }
                        17 => {
                            current_block_186 = 1275279814574597502;
                        }
                        11 => {
                            current_block_186 = 8371071221061937948;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match current_block_186 {
                        11099343707781121639 => {}
                        _ => match current_block_186 {
                            1275279814574597502 => {
                                ptr = ptr.offset(2isize);
                                if !(end.wrapping_offset_from(ptr) as c_long
                                    >= (1i32 * 2i32) as c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                if !(*ptr.offset(0isize) as c_int == 0i32
                                    && *ptr.offset(1isize) as c_int == 0x3ei32)
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_START_TAG_WITH_ATTS;
                            }
                        },
                    }
                }
                17518983136156868271 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_scanAtts(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut hadColon: c_int = 0i32;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_186: u64;
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_186 = 13609164463425058869;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_186 = 13609164463425058869;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_186 = 11099343707781121639;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_186 = 11099343707781121639;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_186 = 11099343707781121639;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut current_block_64: u64;
                    match if *ptr.offset(1isize) as c_int == 0i32 {
                        (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1isize),
                            *ptr.offset(0isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int)
                                << 3i32)
                                + (*ptr.offset(0isize) as c_uchar as c_int
                                    >> 5i32))
                                as usize]
                                & (1u32)
                                    << (*ptr.offset(0isize) as c_uchar as c_int
                                        & 0x1fi32)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            current_block_64 = 16848571710846909653;
                        }
                        22 | 24 => {
                            current_block_64 = 16848571710846909653;
                        }
                        5 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2isize);
                            current_block_64 = 317151059986244064;
                        }
                        6 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3isize);
                            current_block_64 = 317151059986244064;
                        }
                        7 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4isize);
                            current_block_64 = 317151059986244064;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match current_block_64 {
                        16848571710846909653 => ptr = ptr.offset(2isize),
                        _ => {}
                    }
                    current_block_186 = 11099343707781121639;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: c_int = 0;
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t = if *ptr.offset(1isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        };
                        if t == BT_EQUALS as c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    current_block_186 = 17167606947040001567;
                }
                14 => {
                    current_block_186 = 17167606947040001567;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_186 {
                17167606947040001567 => {
                    let mut open: c_int = 0;
                    hadColon = 0i32;
                    loop {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        open = if *ptr.offset(1isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        };
                        if open == BT_QUOT as c_int || open == BT_APOS as c_int {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    ptr = ptr.offset(2isize);
                    loop {
                        let mut t_0: c_int = 0;
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t_0 = if *ptr.offset(1isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        };
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(2isize)
                            }
                            6 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(3isize)
                            }
                            7 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(4isize)
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            3 => {
                                let mut tok: c_int = little2_scanRef(
                                    enc,
                                    ptr.offset(2isize),
                                    end,
                                    &mut ptr,
                                );
                                if tok <= 0i32 {
                                    if tok == XML_TOK_INVALID {
                                        *nextTokPtr = ptr
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => ptr = ptr.offset(2isize),
                        }
                    }
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    match if *ptr.offset(1isize) as c_int == 0i32 {
                        (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1isize),
                            *ptr.offset(0isize),
                        )
                    } {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(2isize);
                                if !(end.wrapping_offset_from(ptr) as c_long
                                    >= (1i32 * 2i32) as c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                match if *ptr.offset(1isize) as c_int == 0i32 {
                                    (*(enc as *mut normal_encoding)).type_0
                                        [*ptr as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(
                                        *ptr.offset(1isize),
                                        *ptr.offset(0isize),
                                    )
                                } {
                                    29 => {
                                        if namingBitmap[(((nmstrtPages
                                            [*ptr.offset(1isize) as c_uchar as usize]
                                            as c_int)
                                            << 3i32)
                                            + (*ptr.offset(0isize) as c_uchar
                                                as c_int
                                                >> 5i32))
                                            as usize]
                                            & (1u32)
                                                << (*ptr.offset(0isize) as c_uchar
                                                    as c_int
                                                    & 0x1fi32)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        current_block_186 = 2518365774060181242;
                                        break;
                                    }
                                    22 | 24 => {
                                        current_block_186 = 2518365774060181242;
                                        break;
                                    }
                                    5 => {
                                        if (end.wrapping_offset_from(ptr) as c_long)
                                            < 2i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0i32 == 0 {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(2isize);
                                        current_block_186 = 11099343707781121639;
                                        break;
                                    }
                                    6 => {
                                        if (end.wrapping_offset_from(ptr) as c_long)
                                            < 3i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0i32 == 0 {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(3isize);
                                        current_block_186 = 11099343707781121639;
                                        break;
                                    }
                                    7 => {
                                        if (end.wrapping_offset_from(ptr) as c_long)
                                            < 4i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0i32 == 0 {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(4isize);
                                        current_block_186 = 11099343707781121639;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        current_block_186 = 6641879004945384064;
                                        break;
                                    }
                                    17 => {
                                        current_block_186 = 6487149160532398276;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                }
                            }
                            match current_block_186 {
                                6641879004945384064 => {}
                                6487149160532398276 => {}
                                11099343707781121639 => {}
                                _ => {
                                    ptr = ptr.offset(2isize);
                                    current_block_186 = 11099343707781121639;
                                }
                            }
                        }
                        17 => {
                            current_block_186 = 6487149160532398276;
                        }
                        11 => {
                            current_block_186 = 6641879004945384064;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match current_block_186 {
                        11099343707781121639 => {}
                        _ => match current_block_186 {
                            6487149160532398276 => {
                                ptr = ptr.offset(2isize);
                                if !(end.wrapping_offset_from(ptr) as c_long
                                    >= (1i32 * 2i32) as c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                if !(*ptr.offset(1isize) as c_int == 0i32
                                    && *ptr.offset(0isize) as c_int == 0x3ei32)
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_START_TAG_WITH_ATTS;
                            }
                        },
                    }
                }
                13609164463425058869 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn normal_scanAtts(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut hadColon: c_int = 0i32;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut current_block_186: u64;
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                29 => {
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_186 = 1328744425620679301;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_186 = 1328744425620679301;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_186 = 11099343707781121639;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_186 = 11099343707781121639;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_186 = 11099343707781121639;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(1isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut current_block_64: u64;
                    match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                    {
                        29 => {
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            current_block_64 = 13714184482889841412;
                        }
                        22 | 24 => {
                            current_block_64 = 13714184482889841412;
                        }
                        5 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isNmstrt2
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2isize);
                            current_block_64 = 317151059986244064;
                        }
                        6 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isNmstrt3
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3isize);
                            current_block_64 = 317151059986244064;
                        }
                        7 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isNmstrt4
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4isize);
                            current_block_64 = 317151059986244064;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match current_block_64 {
                        13714184482889841412 => ptr = ptr.offset(1isize),
                        _ => {}
                    }
                    current_block_186 = 11099343707781121639;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: c_int = 0;
                        ptr = ptr.offset(1isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 1i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t = (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                            as c_int;
                        if t == BT_EQUALS as c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    current_block_186 = 17167606947040001567;
                }
                14 => {
                    current_block_186 = 17167606947040001567;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_186 {
                17167606947040001567 => {
                    let mut open: c_int = 0;
                    hadColon = 0i32;
                    loop {
                        ptr = ptr.offset(1isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 1i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        open = (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                            as c_int;
                        if open == BT_QUOT as c_int || open == BT_APOS as c_int {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    ptr = ptr.offset(1isize);
                    loop {
                        let mut t_0: c_int = 0;
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 1i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t_0 = (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                            as c_int;
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(2isize)
                            }
                            6 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(3isize)
                            }
                            7 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(4isize)
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            3 => {
                                let mut tok: c_int = normal_scanRef(
                                    enc,
                                    ptr.offset(1isize),
                                    end,
                                    &mut ptr,
                                );
                                if tok <= 0i32 {
                                    if tok == XML_TOK_INVALID {
                                        *nextTokPtr = ptr
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => ptr = ptr.offset(1isize),
                        }
                    }
                    ptr = ptr.offset(1isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                    {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(1isize);
                                if !(end.wrapping_offset_from(ptr) as c_long
                                    >= (1i32 * 1i32) as c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                match (*(enc as *mut normal_encoding)).type_0
                                    [*ptr as c_uchar as usize]
                                    as c_int
                                {
                                    29 => {
                                        if 0i32 == 0 {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        current_block_186 = 10484988340598120725;
                                        break;
                                    }
                                    22 | 24 => {
                                        current_block_186 = 10484988340598120725;
                                        break;
                                    }
                                    5 => {
                                        if (end.wrapping_offset_from(ptr) as c_long)
                                            < 2i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if (*(enc as *const normal_encoding))
                                            .isNmstrt2
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(2isize);
                                        current_block_186 = 11099343707781121639;
                                        break;
                                    }
                                    6 => {
                                        if (end.wrapping_offset_from(ptr) as c_long)
                                            < 3i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if (*(enc as *const normal_encoding))
                                            .isNmstrt3
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(3isize);
                                        current_block_186 = 11099343707781121639;
                                        break;
                                    }
                                    7 => {
                                        if (end.wrapping_offset_from(ptr) as c_long)
                                            < 4i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if (*(enc as *const normal_encoding))
                                            .isNmstrt4
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(4isize);
                                        current_block_186 = 11099343707781121639;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        current_block_186 = 359750854795906639;
                                        break;
                                    }
                                    17 => {
                                        current_block_186 = 13746452913673866366;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                }
                            }
                            match current_block_186 {
                                359750854795906639 => {}
                                13746452913673866366 => {}
                                11099343707781121639 => {}
                                _ => {
                                    ptr = ptr.offset(1isize);
                                    current_block_186 = 11099343707781121639;
                                }
                            }
                        }
                        17 => {
                            current_block_186 = 13746452913673866366;
                        }
                        11 => {
                            current_block_186 = 359750854795906639;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match current_block_186 {
                        11099343707781121639 => {}
                        _ => match current_block_186 {
                            13746452913673866366 => {
                                ptr = ptr.offset(1isize);
                                if !(end.wrapping_offset_from(ptr) as c_long
                                    >= (1i32 * 1i32) as c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                if !(*ptr as c_int == 0x3ei32) {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                *nextTokPtr = ptr.offset(1isize);
                                return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(1isize);
                                return XML_TOK_START_TAG_WITH_ATTS;
                            }
                        },
                    }
                }
                1328744425620679301 => ptr = ptr.offset(1isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
    /* ptr points to character following "<" */
    /* ptr points to character following "<" */
    /* ptr points to character following "<" */

    pub unsafe extern "C" fn normal_scanLt(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut hadColon: c_int = 0;
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_45: u64;
        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
            29 => {
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_45 = 2093567751087104139;
            }
            22 | 24 => {
                current_block_45 = 2093567751087104139;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt2
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_45 = 1847472278776910194;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt3
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_45 = 1847472278776910194;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt4
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_45 = 1847472278776910194;
            }
            16 => {
                ptr = ptr.offset(1isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                    27 => {
                        return normal_scanComment(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        )
                    }
                    20 => {
                        return normal_scanCdataSection(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        )
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            15 => return normal_scanPi(enc, ptr.offset(1isize), end, nextTokPtr),
            17 => return normal_scanEndTag(enc, ptr.offset(1isize), end, nextTokPtr),
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_45 {
            2093567751087104139 => ptr = ptr.offset(1isize),
            _ => {}
        }
        hadColon = 0i32;
        /* we have a start-tag */
        /* we have a start-tag */
        /* we have a start-tag */
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut current_block_161: u64;
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                29 => {
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_161 = 9017017852254768106;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_161 = 9017017852254768106;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_161 = 12655303178690906525;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_161 = 12655303178690906525;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_161 = 12655303178690906525;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(1isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut current_block_112: u64;
                    match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                    {
                        29 => {
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            current_block_112 = 15659092796775618263;
                        }
                        22 | 24 => {
                            current_block_112 = 15659092796775618263;
                        }
                        5 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isNmstrt2
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2isize);
                            current_block_112 = 2463987395154258233;
                        }
                        6 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isNmstrt3
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3isize);
                            current_block_112 = 2463987395154258233;
                        }
                        7 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isNmstrt4
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4isize);
                            current_block_112 = 2463987395154258233;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match current_block_112 {
                        15659092796775618263 => ptr = ptr.offset(1isize),
                        _ => {}
                    }
                    current_block_161 = 12655303178690906525;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(1isize);
                    loop {
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 1i32) as c_long)
                        {
                            current_block_161 = 13000670339742628194;
                            break;
                        }
                        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                            as c_int
                        {
                            29 => {
                                if 0i32 == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                current_block_161 = 5850705615406568950;
                            }
                            22 | 24 => {
                                current_block_161 = 5850705615406568950;
                            }
                            5 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isNmstrt2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(2isize);
                                current_block_161 = 7999014830792590863;
                            }
                            6 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isNmstrt3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(3isize);
                                current_block_161 = 7999014830792590863;
                            }
                            7 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isNmstrt4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(4isize);
                                current_block_161 = 7999014830792590863;
                            }
                            11 => {
                                current_block_161 = 11968577662814546452;
                                break;
                            }
                            17 => {
                                current_block_161 = 885266785391146906;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(1isize);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        match current_block_161 {
                            5850705615406568950 => ptr = ptr.offset(1isize),
                            _ => {}
                        }
                        return normal_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match current_block_161 {
                        11968577662814546452 => {}
                        885266785391146906 => {}
                        _ => return XML_TOK_PARTIAL,
                    }
                }
                11 => {
                    current_block_161 = 11968577662814546452;
                }
                17 => {
                    current_block_161 = 885266785391146906;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_161 {
                885266785391146906 => {
                    ptr = ptr.offset(1isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if !(*ptr as c_int == 0x3ei32) {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                }
                11968577662814546452 => {
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_START_TAG_NO_ATTS;
                }
                9017017852254768106 => ptr = ptr.offset(1isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn big2_scanLt(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut hadColon: c_int = 0;
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_45: u64;
        match if *ptr.offset(0isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0
                [*ptr.offset(1isize) as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_45 = 3982799367787197894;
            }
            22 | 24 => {
                current_block_45 = 3982799367787197894;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_45 = 1847472278776910194;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_45 = 1847472278776910194;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_45 = 1847472278776910194;
            }
            16 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match if *ptr.offset(0isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    27 => {
                        return big2_scanComment(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        )
                    }
                    20 => {
                        return big2_scanCdataSection(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        )
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            15 => return big2_scanPi(enc, ptr.offset(2isize), end, nextTokPtr),
            17 => return big2_scanEndTag(enc, ptr.offset(2isize), end, nextTokPtr),
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_45 {
            3982799367787197894 => ptr = ptr.offset(2isize),
            _ => {}
        }
        hadColon = 0i32;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_161: u64;
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(0isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_161 = 14827193857955628413;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_161 = 14827193857955628413;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_161 = 12655303178690906525;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_161 = 12655303178690906525;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_161 = 12655303178690906525;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut current_block_112: u64;
                    match if *ptr.offset(0isize) as c_int == 0i32 {
                        (*(enc as *mut normal_encoding)).type_0
                            [*ptr.offset(1isize) as c_uchar as usize]
                            as c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0isize),
                            *ptr.offset(1isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages
                                [*ptr.offset(0isize) as c_uchar as usize]
                                as c_int)
                                << 3i32)
                                + (*ptr.offset(1isize) as c_uchar as c_int
                                    >> 5i32))
                                as usize]
                                & (1u32)
                                    << (*ptr.offset(1isize) as c_uchar as c_int
                                        & 0x1fi32)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            current_block_112 = 7694897095983579606;
                        }
                        22 | 24 => {
                            current_block_112 = 7694897095983579606;
                        }
                        5 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2isize);
                            current_block_112 = 2463987395154258233;
                        }
                        6 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3isize);
                            current_block_112 = 2463987395154258233;
                        }
                        7 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4isize);
                            current_block_112 = 2463987395154258233;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match current_block_112 {
                        7694897095983579606 => ptr = ptr.offset(2isize),
                        _ => {}
                    }
                    current_block_161 = 12655303178690906525;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2isize);
                    loop {
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            current_block_161 = 13000670339742628194;
                            break;
                        }
                        match if *ptr.offset(0isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        } {
                            29 => {
                                if namingBitmap[(((nmstrtPages
                                    [*ptr.offset(0isize) as c_uchar as usize]
                                    as c_int)
                                    << 3i32)
                                    + (*ptr.offset(1isize) as c_uchar as c_int
                                        >> 5i32))
                                    as usize]
                                    & (1u32)
                                        << (*ptr.offset(1isize) as c_uchar as c_int
                                            & 0x1fi32)
                                    == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                current_block_161 = 16787315395666308353;
                            }
                            22 | 24 => {
                                current_block_161 = 16787315395666308353;
                            }
                            5 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0i32 == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(2isize);
                                current_block_161 = 7999014830792590863;
                            }
                            6 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0i32 == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(3isize);
                                current_block_161 = 7999014830792590863;
                            }
                            7 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0i32 == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(4isize);
                                current_block_161 = 7999014830792590863;
                            }
                            11 => {
                                current_block_161 = 8186870683368195072;
                                break;
                            }
                            17 => {
                                current_block_161 = 1225701803299345799;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(2isize);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        match current_block_161 {
                            16787315395666308353 => ptr = ptr.offset(2isize),
                            _ => {}
                        }
                        return big2_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match current_block_161 {
                        8186870683368195072 => {}
                        1225701803299345799 => {}
                        _ => return XML_TOK_PARTIAL,
                    }
                }
                11 => {
                    current_block_161 = 8186870683368195072;
                }
                17 => {
                    current_block_161 = 1225701803299345799;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_161 {
                1225701803299345799 => {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if !(*ptr.offset(0isize) as c_int == 0i32
                        && *ptr.offset(1isize) as c_int == 0x3ei32)
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                }
                8186870683368195072 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_START_TAG_NO_ATTS;
                }
                14827193857955628413 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_scanLt(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut hadColon: c_int = 0;
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_45: u64;
        match if *ptr.offset(1isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_45 = 3604408518828465902;
            }
            22 | 24 => {
                current_block_45 = 3604408518828465902;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_45 = 1847472278776910194;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_45 = 1847472278776910194;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_45 = 1847472278776910194;
            }
            16 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match if *ptr.offset(1isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    27 => {
                        return little2_scanComment(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        )
                    }
                    20 => {
                        return little2_scanCdataSection(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        )
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            15 => return little2_scanPi(enc, ptr.offset(2isize), end, nextTokPtr),
            17 => return little2_scanEndTag(enc, ptr.offset(2isize), end, nextTokPtr),
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_45 {
            3604408518828465902 => ptr = ptr.offset(2isize),
            _ => {}
        }
        hadColon = 0i32;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_161: u64;
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_161 = 11090466150472633662;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_161 = 11090466150472633662;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_161 = 12655303178690906525;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_161 = 12655303178690906525;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_161 = 12655303178690906525;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut current_block_112: u64;
                    match if *ptr.offset(1isize) as c_int == 0i32 {
                        (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1isize),
                            *ptr.offset(0isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int)
                                << 3i32)
                                + (*ptr.offset(0isize) as c_uchar as c_int
                                    >> 5i32))
                                as usize]
                                & (1u32)
                                    << (*ptr.offset(0isize) as c_uchar as c_int
                                        & 0x1fi32)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            current_block_112 = 5114428014282072970;
                        }
                        22 | 24 => {
                            current_block_112 = 5114428014282072970;
                        }
                        5 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2isize);
                            current_block_112 = 2463987395154258233;
                        }
                        6 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3isize);
                            current_block_112 = 2463987395154258233;
                        }
                        7 => {
                            if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0i32 == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4isize);
                            current_block_112 = 2463987395154258233;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match current_block_112 {
                        5114428014282072970 => ptr = ptr.offset(2isize),
                        _ => {}
                    }
                    current_block_161 = 12655303178690906525;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2isize);
                    loop {
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            current_block_161 = 13000670339742628194;
                            break;
                        }
                        match if *ptr.offset(1isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        } {
                            29 => {
                                if namingBitmap[(((nmstrtPages
                                    [*ptr.offset(1isize) as c_uchar as usize]
                                    as c_int)
                                    << 3i32)
                                    + (*ptr.offset(0isize) as c_uchar as c_int
                                        >> 5i32))
                                    as usize]
                                    & (1u32)
                                        << (*ptr.offset(0isize) as c_uchar as c_int
                                            & 0x1fi32)
                                    == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                current_block_161 = 8174279831488790583;
                            }
                            22 | 24 => {
                                current_block_161 = 8174279831488790583;
                            }
                            5 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0i32 == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(2isize);
                                current_block_161 = 7999014830792590863;
                            }
                            6 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0i32 == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(3isize);
                                current_block_161 = 7999014830792590863;
                            }
                            7 => {
                                if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0i32 == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(4isize);
                                current_block_161 = 7999014830792590863;
                            }
                            11 => {
                                current_block_161 = 16654161972338254730;
                                break;
                            }
                            17 => {
                                current_block_161 = 13380707534857435492;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(2isize);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        match current_block_161 {
                            8174279831488790583 => ptr = ptr.offset(2isize),
                            _ => {}
                        }
                        return little2_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match current_block_161 {
                        16654161972338254730 => {}
                        13380707534857435492 => {}
                        _ => return XML_TOK_PARTIAL,
                    }
                }
                11 => {
                    current_block_161 = 16654161972338254730;
                }
                17 => {
                    current_block_161 = 13380707534857435492;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_161 {
                13380707534857435492 => {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if !(*ptr.offset(1isize) as c_int == 0i32
                        && *ptr.offset(0isize) as c_int == 0x3ei32)
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                }
                16654161972338254730 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_START_TAG_NO_ATTS;
                }
                11090466150472633662 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_contentTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (2i32 - 1i32) as c_ulong != 0 {
                n &= !(2i32 - 1i32) as c_ulong;
                if n == 0u64 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        match if *ptr.offset(1isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            2 => return little2_scanLt(enc, ptr.offset(2isize), end, nextTokPtr),
            3 => return little2_scanRef(enc, ptr.offset(2isize), end, nextTokPtr),
            9 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_TRAILING_CR;
                }
                if (if *ptr.offset(1isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                }) == BT_LF as c_int
                {
                    ptr = ptr.offset(2isize)
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE;
            }
            10 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DATA_NEWLINE;
            }
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_TRAILING_RSQB;
                }
                if *ptr.offset(1isize) as c_int == 0i32
                    && *ptr.offset(0isize) as c_int == 0x5di32
                {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_TRAILING_RSQB;
                    }
                    if !(*ptr.offset(1isize) as c_int == 0i32
                        && *ptr.offset(0isize) as c_int == 0x3ei32)
                    {
                        ptr = ptr.offset(-(2isize))
                    } else {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(2isize)
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(3isize)
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(4isize)
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            _ => ptr = ptr.offset(2isize),
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_76: u64;
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_76 = 10213293998891106930;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_76 = 10213293998891106930;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_76 = 10213293998891106930;
                }
                4 => {
                    if end.wrapping_offset_from(ptr) as c_long
                        >= (2i32 * 2i32) as c_long
                    {
                        if !(*ptr.offset(2isize).offset(1isize) as c_int
                            == 0i32
                            && *ptr.offset(2isize).offset(0isize)
                                as c_int
                                == 0x5di32)
                        {
                            ptr = ptr.offset(2isize);
                            current_block_76 = 10213293998891106930;
                        } else if end.wrapping_offset_from(ptr) as c_long
                            >= (3i32 * 2i32) as c_long
                        {
                            if !(*ptr
                                .offset((2i32 * 2i32) as isize)
                                .offset(1isize)
                                as c_int
                                == 0i32
                                && *ptr
                                    .offset((2i32 * 2i32) as isize)
                                    .offset(0isize)
                                    as c_int
                                    == 0x3ei32)
                            {
                                ptr = ptr.offset(2isize)
                            } else {
                                *nextTokPtr = ptr.offset((2i32 * 2i32) as isize);
                                return XML_TOK_INVALID;
                            }
                            current_block_76 = 10213293998891106930;
                        } else {
                            current_block_76 = 8987447449223982590;
                        }
                    } else {
                        current_block_76 = 8987447449223982590;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    current_block_76 = 8987447449223982590;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                    current_block_76 = 10213293998891106930;
                }
            }
            match current_block_76 {
                10213293998891106930 => {}
                _ =>
                /* fall through */
                /* fall through */
                /* fall through */
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }

    pub unsafe extern "C" fn normal_contentTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 1i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (1i32 - 1i32) as c_ulong != 0 {
                n &= !(1i32 - 1i32) as c_ulong;
                if n == 0u64 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
            2 => return normal_scanLt(enc, ptr.offset(1isize), end, nextTokPtr),
            3 => return normal_scanRef(enc, ptr.offset(1isize), end, nextTokPtr),
            9 => {
                ptr = ptr.offset(1isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long)
                {
                    return XML_TOK_TRAILING_CR;
                }
                if (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                    == BT_LF as c_int
                {
                    ptr = ptr.offset(1isize)
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE;
            }
            10 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_DATA_NEWLINE;
            }
            4 => {
                ptr = ptr.offset(1isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long)
                {
                    return XML_TOK_TRAILING_RSQB;
                }
                if *ptr as c_int == 0x5di32 {
                    ptr = ptr.offset(1isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long)
                    {
                        return XML_TOK_TRAILING_RSQB;
                    }
                    if !(*ptr as c_int == 0x3ei32) {
                        ptr = ptr.offset(-(1isize))
                    } else {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize)
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize)
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize)
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            _ => ptr = ptr.offset(1isize),
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut current_block_76: u64;
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid2
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_76 = 10213293998891106930;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid3
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_76 = 10213293998891106930;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid4
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_76 = 10213293998891106930;
                }
                4 => {
                    if end.wrapping_offset_from(ptr) as c_long
                        >= (2i32 * 1i32) as c_long
                    {
                        if !(*ptr.offset(1isize) as c_int == 0x5di32) {
                            ptr = ptr.offset(1isize);
                            current_block_76 = 10213293998891106930;
                        } else if end.wrapping_offset_from(ptr) as c_long
                            >= (3i32 * 1i32) as c_long
                        {
                            if !(*ptr.offset((2i32 * 1i32) as isize) as c_int
                                == 0x3ei32)
                            {
                                ptr = ptr.offset(1isize)
                            } else {
                                *nextTokPtr = ptr.offset((2i32 * 1i32) as isize);
                                return XML_TOK_INVALID;
                            }
                            current_block_76 = 10213293998891106930;
                        } else {
                            current_block_76 = 15298127564675707271;
                        }
                    } else {
                        current_block_76 = 15298127564675707271;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    current_block_76 = 15298127564675707271;
                }
                _ => {
                    ptr = ptr.offset(1isize);
                    current_block_76 = 10213293998891106930;
                }
            }
            match current_block_76 {
                10213293998891106930 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }

    pub unsafe extern "C" fn big2_contentTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (2i32 - 1i32) as c_ulong != 0 {
                n &= !(2i32 - 1i32) as c_ulong;
                if n == 0u64 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        match if *ptr.offset(0isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0
                [*ptr.offset(1isize) as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            2 => return big2_scanLt(enc, ptr.offset(2isize), end, nextTokPtr),
            3 => return big2_scanRef(enc, ptr.offset(2isize), end, nextTokPtr),
            9 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_TRAILING_CR;
                }
                if (if *ptr.offset(0isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                }) == BT_LF as c_int
                {
                    ptr = ptr.offset(2isize)
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE;
            }
            10 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DATA_NEWLINE;
            }
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_TRAILING_RSQB;
                }
                if *ptr.offset(0isize) as c_int == 0i32
                    && *ptr.offset(1isize) as c_int == 0x5di32
                {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_TRAILING_RSQB;
                    }
                    if !(*ptr.offset(0isize) as c_int == 0i32
                        && *ptr.offset(1isize) as c_int == 0x3ei32)
                    {
                        ptr = ptr.offset(-(2isize))
                    } else {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(2isize)
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(3isize)
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(4isize)
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            _ => ptr = ptr.offset(2isize),
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_76: u64;
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_76 = 10213293998891106930;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_76 = 10213293998891106930;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64
                        || 0i32 != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_76 = 10213293998891106930;
                }
                4 => {
                    if end.wrapping_offset_from(ptr) as c_long
                        >= (2i32 * 2i32) as c_long
                    {
                        if !(*ptr.offset(2isize).offset(0isize) as c_int
                            == 0i32
                            && *ptr.offset(2isize).offset(1isize)
                                as c_int
                                == 0x5di32)
                        {
                            ptr = ptr.offset(2isize);
                            current_block_76 = 10213293998891106930;
                        } else if end.wrapping_offset_from(ptr) as c_long
                            >= (3i32 * 2i32) as c_long
                        {
                            if !(*ptr
                                .offset((2i32 * 2i32) as isize)
                                .offset(0isize)
                                as c_int
                                == 0i32
                                && *ptr
                                    .offset((2i32 * 2i32) as isize)
                                    .offset(1isize)
                                    as c_int
                                    == 0x3ei32)
                            {
                                ptr = ptr.offset(2isize)
                            } else {
                                *nextTokPtr = ptr.offset((2i32 * 2i32) as isize);
                                return XML_TOK_INVALID;
                            }
                            current_block_76 = 10213293998891106930;
                        } else {
                            current_block_76 = 14775589748497275135;
                        }
                    } else {
                        current_block_76 = 14775589748497275135;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    current_block_76 = 14775589748497275135;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                    current_block_76 = 10213293998891106930;
                }
            }
            match current_block_76 {
                10213293998891106930 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
    /* ptr points to character following "%" */
    /* ptr points to character following "%" */
    /* ptr points to character following "%" */

    pub unsafe extern "C" fn little2_scanPercent(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_34: u64;
        match if *ptr.offset(1isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_34 = 5609339353924833948;
            }
            22 | 24 => {
                current_block_34 = 5609339353924833948;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_34 = 9520865839495247062;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_34 = 9520865839495247062;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_34 = 9520865839495247062;
            }
            21 | 10 | 9 | 30 => {
                *nextTokPtr = ptr;
                return XML_TOK_PERCENT;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_34 {
            5609339353924833948 => ptr = ptr.offset(2isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_65: u64;
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_65 = 14924637866976491194;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_65 = 14924637866976491194;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_65 = 12758904613967585247;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_65 = 12758904613967585247;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_65 = 12758904613967585247;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_PARAM_ENTITY_REF;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_65 {
                14924637866976491194 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn big2_scanPercent(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_34: u64;
        match if *ptr.offset(0isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0
                [*ptr.offset(1isize) as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_34 = 2535787018921809524;
            }
            22 | 24 => {
                current_block_34 = 2535787018921809524;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_34 = 9520865839495247062;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_34 = 9520865839495247062;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_34 = 9520865839495247062;
            }
            21 | 10 | 9 | 30 => {
                *nextTokPtr = ptr;
                return XML_TOK_PERCENT;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_34 {
            2535787018921809524 => ptr = ptr.offset(2isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_65: u64;
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(0isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_65 = 16518566462615084259;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_65 = 16518566462615084259;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_65 = 12758904613967585247;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_65 = 12758904613967585247;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_65 = 12758904613967585247;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_PARAM_ENTITY_REF;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_65 {
                16518566462615084259 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn normal_scanPercent(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_34: u64;
        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
            29 => {
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_34 = 16508288390984586136;
            }
            22 | 24 => {
                current_block_34 = 16508288390984586136;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt2
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_34 = 9520865839495247062;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt3
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_34 = 9520865839495247062;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt4
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_34 = 9520865839495247062;
            }
            21 | 10 | 9 | 30 => {
                *nextTokPtr = ptr;
                return XML_TOK_PERCENT;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_34 {
            16508288390984586136 => ptr = ptr.offset(1isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut current_block_65: u64;
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                29 => {
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_65 = 3744295902770449260;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_65 = 3744295902770449260;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_65 = 12758904613967585247;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_65 = 12758904613967585247;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_65 = 12758904613967585247;
                }
                18 => {
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_PARAM_ENTITY_REF;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_65 {
                3744295902770449260 => ptr = ptr.offset(1isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_scanPoundName(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(1isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_32 = 2269812350301213550;
            }
            22 | 24 => {
                current_block_32 = 2269812350301213550;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_32 = 5494826135382683477;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_32 = 5494826135382683477;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_32 = 5494826135382683477;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_32 {
            2269812350301213550 => ptr = ptr.offset(2isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_63: u64;
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_63 = 15716831554174598335;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_63 = 15716831554174598335;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_63 = 8869332144787829186;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_63 = 8869332144787829186;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_63 = 8869332144787829186;
                }
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_POUND_NAME;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_63 {
                15716831554174598335 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return -XML_TOK_POUND_NAME;
    }

    pub unsafe extern "C" fn normal_scanPoundName(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_32: u64;
        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
            29 => {
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_32 = 14531478163722833811;
            }
            22 | 24 => {
                current_block_32 = 14531478163722833811;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt2
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_32 = 5494826135382683477;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt3
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_32 = 5494826135382683477;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt4
                    .expect("non-null function pointer")(enc, ptr)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_32 = 5494826135382683477;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_32 {
            14531478163722833811 => ptr = ptr.offset(1isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut current_block_63: u64;
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                29 => {
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_63 = 1251073717335040501;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_63 = 1251073717335040501;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_63 = 8869332144787829186;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_63 = 8869332144787829186;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_63 = 8869332144787829186;
                }
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_POUND_NAME;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_63 {
                1251073717335040501 => ptr = ptr.offset(1isize),
                _ => {}
            }
        }
        return -XML_TOK_POUND_NAME;
    }

    pub unsafe extern "C" fn big2_scanPoundName(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
            return XML_TOK_PARTIAL;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(0isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0
                [*ptr.offset(1isize) as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as c_uchar as c_int & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_32 = 633956857006047984;
            }
            22 | 24 => {
                current_block_32 = 633956857006047984;
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2isize);
                current_block_32 = 5494826135382683477;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3isize);
                current_block_32 = 5494826135382683477;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0i32 == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4isize);
                current_block_32 = 5494826135382683477;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_32 {
            633956857006047984 => ptr = ptr.offset(2isize),
            _ => {}
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_63: u64;
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(0isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_63 = 3145220949513738010;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_63 = 3145220949513738010;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_63 = 8869332144787829186;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_63 = 8869332144787829186;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_63 = 8869332144787829186;
                }
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_POUND_NAME;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_63 {
                3145220949513738010 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return -XML_TOK_POUND_NAME;
    }

    pub unsafe extern "C" fn little2_scanLit(
        mut open: c_int,
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut t: c_int = if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            };
            match t {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(2isize)
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(3isize)
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(4isize)
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                12 | 13 => {
                    ptr = ptr.offset(2isize);
                    if !(t != open) {
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return -XML_TOK_LITERAL;
                        }
                        *nextTokPtr = ptr;
                        match if *ptr.offset(1isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        } {
                            21 | 9 | 10 | 11 | 30 | 20 => return XML_TOK_LITERAL,
                            _ => return XML_TOK_INVALID,
                        }
                    }
                }
                _ => ptr = ptr.offset(2isize),
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn big2_scanLit(
        mut open: c_int,
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut t: c_int = if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            };
            match t {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(2isize)
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(3isize)
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(4isize)
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                12 | 13 => {
                    ptr = ptr.offset(2isize);
                    if !(t != open) {
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return -XML_TOK_LITERAL;
                        }
                        *nextTokPtr = ptr;
                        match if *ptr.offset(0isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        } {
                            21 | 9 | 10 | 11 | 30 | 20 => return XML_TOK_LITERAL,
                            _ => return XML_TOK_INVALID,
                        }
                    }
                }
                _ => ptr = ptr.offset(2isize),
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn normal_scanLit(
        mut open: c_int,
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut t: c_int =
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int;
            match t {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize)
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize)
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize)
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                12 | 13 => {
                    ptr = ptr.offset(1isize);
                    if !(t != open) {
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 1i32) as c_long)
                        {
                            return -XML_TOK_LITERAL;
                        }
                        *nextTokPtr = ptr;
                        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                            as c_int
                        {
                            21 | 9 | 10 | 11 | 30 | 20 => return XML_TOK_LITERAL,
                            _ => return XML_TOK_INVALID,
                        }
                    }
                }
                _ => ptr = ptr.offset(1isize),
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_prologTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut tok: c_int = 0;
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (2i32 - 1i32) as c_ulong != 0 {
                n &= !(2i32 - 1i32) as c_ulong;
                if n == 0u64 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        let mut current_block_112: u64;
        match if *ptr.offset(1isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            12 => {
                return little2_scanLit(
                    BT_QUOT as c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                )
            }
            13 => {
                return little2_scanLit(
                    BT_APOS as c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                )
            }
            2 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match if *ptr.offset(1isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    16 => {
                        return little2_scanDecl(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        )
                    }
                    15 => {
                        return little2_scanPi(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        )
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(2isize));
                        return XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            9 => {
                if ptr.offset(2isize) == end {
                    *nextTokPtr = end;
                    /* indicate that this might be part of a CR/LF pair */
                    /* indicate that this might be part of a CR/LF pair */
                    /* indicate that this might be part of a CR/LF pair */
                    return -XML_TOK_PROLOG_S;
                }
                current_block_112 = 18365440899698678160;
            }
            21 | 10 => {
                current_block_112 = 18365440899698678160;
            }
            30 => {
                return little2_scanPercent(enc, ptr.offset(2isize), end, nextTokPtr)
            }
            35 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_COMMA;
            }
            20 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OPEN_BRACKET;
            }
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return -XML_TOK_CLOSE_BRACKET;
                }
                if *ptr.offset(1isize) as c_int == 0i32
                    && *ptr.offset(0isize) as c_int == 0x5di32
                {
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (2i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(2isize).offset(1isize) as c_int
                        == 0i32
                        && *ptr.offset(2isize).offset(0isize) as c_int
                            == 0x3ei32
                    {
                        *nextTokPtr = ptr.offset((2i32 * 2i32) as isize);
                        return XML_TOK_COND_SECT_CLOSE;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET;
            }
            31 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OPEN_PAREN;
            }
            32 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return -XML_TOK_CLOSE_PAREN;
                }
                match if *ptr.offset(1isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    33 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_PLUS;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            36 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OR;
            }
            11 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DECL_CLOSE;
            }
            19 => {
                return little2_scanPoundName(enc, ptr.offset(2isize), end, nextTokPtr)
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            22 | 24 => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(2isize);
                current_block_112 = 2222055338596505704;
            }
            25 | 26 | 27 | 23 => {
                tok = XML_TOK_NMTOKEN;
                ptr = ptr.offset(2isize);
                current_block_112 = 2222055338596505704;
            }
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as c_uchar as c_int & 0x1fi32)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NAME;
                    current_block_112 = 2222055338596505704;
                } else if namingBitmap[(((namePages
                    [*ptr.offset(1isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as c_uchar as c_int & 0x1fi32)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NMTOKEN;
                    current_block_112 = 2222055338596505704;
                } else {
                    current_block_112 = 15425764709914638707;
                }
            }
            _ => {
                current_block_112 = 15425764709914638707;
            }
        }
        match current_block_112 {
            2222055338596505704 => {}
            18365440899698678160 =>
            /* fall through */
            /* fall through */
            /* fall through */
            {
                loop {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        break;
                    }
                    let mut current_block_32: u64;
                    match if *ptr.offset(1isize) as c_int == 0i32 {
                        (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1isize),
                            *ptr.offset(0isize),
                        )
                    } {
                        21 | 10 => {
                            current_block_32 = 14072441030219150333;
                        }
                        9 => {
                            /* don't split CR/LF pair */
                            /* don't split CR/LF pair */
                            /* don't split CR/LF pair */
                            if ptr.offset(2isize) != end {
                                current_block_32 = 14072441030219150333;
                            } else {
                                current_block_32 = 9063685204218595982;
                            }
                        }
                        _ => {
                            current_block_32 = 9063685204218595982;
                        }
                    }
                    match current_block_32 {
                        14072441030219150333 => {}
                        _ =>
                        /* fall through */
                        /* fall through */
                        /* fall through */
                        {
                            *nextTokPtr = ptr;
                            return XML_TOK_PROLOG_S;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_PROLOG_S;
            }
            _ =>
            /* fall through */
            /* fall through */
            /* fall through */
            {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_198: u64;
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_198 = 16114569038092921525;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_198 = 16114569038092921525;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_198 = 11165907417739823532;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_198 = 11165907417739823532;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_198 = 11165907417739823532;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(2isize);
                    match tok {
                        XML_TOK_NAME => {
                            if !(end.wrapping_offset_from(ptr) as c_long
                                >= (1i32 * 2i32) as c_long)
                            {
                                return XML_TOK_PARTIAL;
                            }
                            tok = XML_TOK_PREFIXED_NAME;
                            let mut current_block_175: u64;
                            match if *ptr.offset(1isize) as c_int == 0i32 {
                                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(1isize),
                                    *ptr.offset(0isize),
                                )
                            } {
                                29 => {
                                    if namingBitmap[(((namePages
                                        [*ptr.offset(1isize) as c_uchar as usize]
                                        as c_int)
                                        << 3i32)
                                        + (*ptr.offset(0isize) as c_uchar as c_int
                                            >> 5i32))
                                        as usize]
                                        & (1u32)
                                            << (*ptr.offset(0isize) as c_uchar
                                                as c_int
                                                & 0x1fi32)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    current_block_175 = 995715539629334807;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    current_block_175 = 995715539629334807;
                                }
                                5 => {
                                    if (end.wrapping_offset_from(ptr) as c_long)
                                        < 2i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0i32 == 0 {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(2isize);
                                    current_block_175 = 4755552050407867010;
                                }
                                6 => {
                                    if (end.wrapping_offset_from(ptr) as c_long)
                                        < 3i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0i32 == 0 {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(3isize);
                                    current_block_175 = 4755552050407867010;
                                }
                                7 => {
                                    if (end.wrapping_offset_from(ptr) as c_long)
                                        < 4i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0i32 == 0 {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(4isize);
                                    current_block_175 = 4755552050407867010;
                                }
                                _ => {
                                    tok = XML_TOK_NMTOKEN;
                                    current_block_175 = 4755552050407867010;
                                }
                            }
                            match current_block_175 {
                                995715539629334807 => ptr = ptr.offset(2isize),
                                _ => {}
                            }
                        }
                        XML_TOK_PREFIXED_NAME => tok = XML_TOK_NMTOKEN,
                        _ => {}
                    }
                    current_block_198 = 11165907417739823532;
                }
                34 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_PLUS;
                }
                33 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_ASTERISK;
                }
                15 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_QUESTION;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_198 {
                16114569038092921525 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return -tok;
    }

    pub unsafe extern "C" fn normal_prologTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut tok: c_int = 0;
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 1i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (1i32 - 1i32) as c_ulong != 0 {
                n &= !(1i32 - 1i32) as c_ulong;
                if n == 0u64 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        let mut current_block_112: u64;
        match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
            12 => {
                return normal_scanLit(
                    BT_QUOT as c_int,
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                )
            }
            13 => {
                return normal_scanLit(
                    BT_APOS as c_int,
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                )
            }
            2 => {
                ptr = ptr.offset(1isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                    16 => {
                        return normal_scanDecl(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        )
                    }
                    15 => {
                        return normal_scanPi(enc, ptr.offset(1isize), end, nextTokPtr)
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(1isize));
                        return XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            9 => {
                if ptr.offset(1isize) == end {
                    *nextTokPtr = end;
                    return -XML_TOK_PROLOG_S;
                }
                current_block_112 = 12267575154421147267;
            }
            21 | 10 => {
                current_block_112 = 12267575154421147267;
            }
            30 => return normal_scanPercent(enc, ptr.offset(1isize), end, nextTokPtr),
            35 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_COMMA;
            }
            20 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_OPEN_BRACKET;
            }
            4 => {
                ptr = ptr.offset(1isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long)
                {
                    return -XML_TOK_CLOSE_BRACKET;
                }
                if *ptr as c_int == 0x5di32 {
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (2i32 * 1i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(1isize) as c_int == 0x3ei32 {
                        *nextTokPtr = ptr.offset((2i32 * 1i32) as isize);
                        return XML_TOK_COND_SECT_CLOSE;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET;
            }
            31 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_OPEN_PAREN;
            }
            32 => {
                ptr = ptr.offset(1isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long)
                {
                    return -XML_TOK_CLOSE_PAREN;
                }
                match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                    33 => {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CLOSE_PAREN_PLUS;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            36 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_OR;
            }
            11 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_DECL_CLOSE;
            }
            19 => {
                return normal_scanPoundName(enc, ptr.offset(1isize), end, nextTokPtr)
            }
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NAME
                } else if (*(enc as *const normal_encoding))
                    .isName2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NMTOKEN
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_112 = 2222055338596505704;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(3isize);
                    tok = XML_TOK_NAME
                } else if (*(enc as *const normal_encoding))
                    .isName3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(3isize);
                    tok = XML_TOK_NMTOKEN
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_112 = 2222055338596505704;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(4isize);
                    tok = XML_TOK_NAME
                } else if (*(enc as *const normal_encoding))
                    .isName4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(4isize);
                    tok = XML_TOK_NMTOKEN
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                current_block_112 = 2222055338596505704;
            }
            22 | 24 => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(1isize);
                current_block_112 = 2222055338596505704;
            }
            25 | 26 | 27 | 23 => {
                tok = XML_TOK_NMTOKEN;
                ptr = ptr.offset(1isize);
                current_block_112 = 2222055338596505704;
            }
            29 | _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match current_block_112 {
            2222055338596505704 => {}
            _ => {
                loop {
                    ptr = ptr.offset(1isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long)
                    {
                        break;
                    }
                    let mut current_block_32: u64;
                    match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
                    {
                        21 | 10 => {
                            current_block_32 = 14072441030219150333;
                        }
                        9 => {
                            if ptr.offset(1isize) != end {
                                current_block_32 = 14072441030219150333;
                            } else {
                                current_block_32 = 1184991590081026370;
                            }
                        }
                        _ => {
                            current_block_32 = 1184991590081026370;
                        }
                    }
                    match current_block_32 {
                        14072441030219150333 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_PROLOG_S;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_PROLOG_S;
            }
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut current_block_198: u64;
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                29 => {
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_198 = 17381770869190098083;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_198 = 17381770869190098083;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_198 = 11165907417739823532;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_198 = 11165907417739823532;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isName4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_198 = 11165907417739823532;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(1isize);
                    match tok {
                        XML_TOK_NAME => {
                            if !(end.wrapping_offset_from(ptr) as c_long
                                >= (1i32 * 1i32) as c_long)
                            {
                                return XML_TOK_PARTIAL;
                            }
                            tok = XML_TOK_PREFIXED_NAME;
                            let mut current_block_175: u64;
                            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                            {
                                29 => {
                                    if 0i32 == 0 {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    current_block_175 = 8121321173029633297;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    current_block_175 = 8121321173029633297;
                                }
                                5 => {
                                    if (end.wrapping_offset_from(ptr) as c_long)
                                        < 2i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if (*(enc as *const normal_encoding))
                                        .isName2
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(2isize);
                                    current_block_175 = 4755552050407867010;
                                }
                                6 => {
                                    if (end.wrapping_offset_from(ptr) as c_long)
                                        < 3i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if (*(enc as *const normal_encoding))
                                        .isName3
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(3isize);
                                    current_block_175 = 4755552050407867010;
                                }
                                7 => {
                                    if (end.wrapping_offset_from(ptr) as c_long)
                                        < 4i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if (*(enc as *const normal_encoding))
                                        .isName4
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(4isize);
                                    current_block_175 = 4755552050407867010;
                                }
                                _ => {
                                    tok = XML_TOK_NMTOKEN;
                                    current_block_175 = 4755552050407867010;
                                }
                            }
                            match current_block_175 {
                                8121321173029633297 => ptr = ptr.offset(1isize),
                                _ => {}
                            }
                        }
                        XML_TOK_PREFIXED_NAME => tok = XML_TOK_NMTOKEN,
                        _ => {}
                    }
                    current_block_198 = 11165907417739823532;
                }
                34 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_NAME_PLUS;
                }
                33 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_NAME_ASTERISK;
                }
                15 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_NAME_QUESTION;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_198 {
                17381770869190098083 => ptr = ptr.offset(1isize),
                _ => {}
            }
        }
        return -tok;
    }

    pub unsafe extern "C" fn big2_prologTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut tok: c_int = 0;
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (2i32 - 1i32) as c_ulong != 0 {
                n &= !(2i32 - 1i32) as c_ulong;
                if n == 0u64 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        let mut current_block_112: u64;
        match if *ptr.offset(0isize) as c_int == 0i32 {
            (*(enc as *mut normal_encoding)).type_0
                [*ptr.offset(1isize) as c_uchar as usize] as c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            12 => {
                return big2_scanLit(
                    BT_QUOT as c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                )
            }
            13 => {
                return big2_scanLit(
                    BT_APOS as c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                )
            }
            2 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match if *ptr.offset(0isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    16 => {
                        return big2_scanDecl(enc, ptr.offset(2isize), end, nextTokPtr)
                    }
                    15 => {
                        return big2_scanPi(enc, ptr.offset(2isize), end, nextTokPtr)
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(2isize));
                        return XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            9 => {
                if ptr.offset(2isize) == end {
                    *nextTokPtr = end;
                    return -XML_TOK_PROLOG_S;
                }
                current_block_112 = 6797528435482537081;
            }
            21 | 10 => {
                current_block_112 = 6797528435482537081;
            }
            30 => return big2_scanPercent(enc, ptr.offset(2isize), end, nextTokPtr),
            35 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_COMMA;
            }
            20 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OPEN_BRACKET;
            }
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return -XML_TOK_CLOSE_BRACKET;
                }
                if *ptr.offset(0isize) as c_int == 0i32
                    && *ptr.offset(1isize) as c_int == 0x5di32
                {
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (2i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(2isize).offset(0isize) as c_int
                        == 0i32
                        && *ptr.offset(2isize).offset(1isize) as c_int
                            == 0x3ei32
                    {
                        *nextTokPtr = ptr.offset((2i32 * 2i32) as isize);
                        return XML_TOK_COND_SECT_CLOSE;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET;
            }
            31 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OPEN_PAREN;
            }
            32 => {
                ptr = ptr.offset(2isize);
                if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long)
                {
                    return -XML_TOK_CLOSE_PAREN;
                }
                match if *ptr.offset(0isize) as c_int == 0i32 {
                    (*(enc as *mut normal_encoding)).type_0
                        [*ptr.offset(1isize) as c_uchar as usize]
                        as c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    33 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_PLUS;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            36 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OR;
            }
            11 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DECL_CLOSE;
            }
            19 => return big2_scanPoundName(enc, ptr.offset(2isize), end, nextTokPtr),
            5 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            6 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            7 => {
                if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            22 | 24 => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(2isize);
                current_block_112 = 2222055338596505704;
            }
            25 | 26 | 27 | 23 => {
                tok = XML_TOK_NMTOKEN;
                ptr = ptr.offset(2isize);
                current_block_112 = 2222055338596505704;
            }
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as c_uchar as c_int & 0x1fi32)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NAME;
                    current_block_112 = 2222055338596505704;
                } else if namingBitmap[(((namePages
                    [*ptr.offset(0isize) as c_uchar as usize]
                    as c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as c_uchar as c_int & 0x1fi32)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NMTOKEN;
                    current_block_112 = 2222055338596505704;
                } else {
                    current_block_112 = 9687627612024382126;
                }
            }
            _ => {
                current_block_112 = 9687627612024382126;
            }
        }
        match current_block_112 {
            2222055338596505704 => {}
            6797528435482537081 => {
                loop {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        break;
                    }
                    let mut current_block_32: u64;
                    match if *ptr.offset(0isize) as c_int == 0i32 {
                        (*(enc as *mut normal_encoding)).type_0
                            [*ptr.offset(1isize) as c_uchar as usize]
                            as c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0isize),
                            *ptr.offset(1isize),
                        )
                    } {
                        21 | 10 => {
                            current_block_32 = 14072441030219150333;
                        }
                        9 => {
                            if ptr.offset(2isize) != end {
                                current_block_32 = 14072441030219150333;
                            } else {
                                current_block_32 = 13880765133914978832;
                            }
                        }
                        _ => {
                            current_block_32 = 13880765133914978832;
                        }
                    }
                    match current_block_32 {
                        14072441030219150333 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_PROLOG_S;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_PROLOG_S;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_198: u64;
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages
                        [*ptr.offset(0isize) as c_uchar as usize]
                        as c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as c_uchar as c_int >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize) as c_uchar as c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    current_block_198 = 14255987127689470057;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_198 = 14255987127689470057;
                }
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize);
                    current_block_198 = 11165907417739823532;
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize);
                    current_block_198 = 11165907417739823532;
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0i32 == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize);
                    current_block_198 = 11165907417739823532;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(2isize);
                    match tok {
                        XML_TOK_NAME => {
                            if !(end.wrapping_offset_from(ptr) as c_long
                                >= (1i32 * 2i32) as c_long)
                            {
                                return XML_TOK_PARTIAL;
                            }
                            tok = XML_TOK_PREFIXED_NAME;
                            let mut current_block_175: u64;
                            match if *ptr.offset(0isize) as c_int == 0i32 {
                                (*(enc as *mut normal_encoding)).type_0
                                    [*ptr.offset(1isize) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(0isize),
                                    *ptr.offset(1isize),
                                )
                            } {
                                29 => {
                                    if namingBitmap[(((namePages
                                        [*ptr.offset(0isize) as c_uchar as usize]
                                        as c_int)
                                        << 3i32)
                                        + (*ptr.offset(1isize) as c_uchar as c_int
                                            >> 5i32))
                                        as usize]
                                        & (1u32)
                                            << (*ptr.offset(1isize) as c_uchar
                                                as c_int
                                                & 0x1fi32)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    current_block_175 = 17746292690483154078;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    current_block_175 = 17746292690483154078;
                                }
                                5 => {
                                    if (end.wrapping_offset_from(ptr) as c_long)
                                        < 2i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0i32 == 0 {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(2isize);
                                    current_block_175 = 4755552050407867010;
                                }
                                6 => {
                                    if (end.wrapping_offset_from(ptr) as c_long)
                                        < 3i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0i32 == 0 {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(3isize);
                                    current_block_175 = 4755552050407867010;
                                }
                                7 => {
                                    if (end.wrapping_offset_from(ptr) as c_long)
                                        < 4i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0i32 == 0 {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(4isize);
                                    current_block_175 = 4755552050407867010;
                                }
                                _ => {
                                    tok = XML_TOK_NMTOKEN;
                                    current_block_175 = 4755552050407867010;
                                }
                            }
                            match current_block_175 {
                                17746292690483154078 => ptr = ptr.offset(2isize),
                                _ => {}
                            }
                        }
                        XML_TOK_PREFIXED_NAME => tok = XML_TOK_NMTOKEN,
                        _ => {}
                    }
                    current_block_198 = 11165907417739823532;
                }
                34 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_PLUS;
                }
                33 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_ASTERISK;
                }
                15 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_QUESTION;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match current_block_198 {
                14255987127689470057 => ptr = ptr.offset(2isize),
                _ => {}
            }
        }
        return -tok;
    }

    pub unsafe extern "C" fn normal_attributeValueTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut start: *const c_char = 0 as *const c_char;
        if ptr >= end {
            return XML_TOK_NONE;
        } else {
            if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long) {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                return XML_TOK_PARTIAL;
                /* LCOV_EXCL_LINE */
                /* LCOV_EXCL_LINE */
                /* LCOV_EXCL_LINE */
            }
        }
        start = ptr;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                3 => {
                    if ptr == start {
                        return normal_scanRef(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                2 => {
                    /* this is for inside entity references */
                    /* this is for inside entity references */
                    /* this is for inside entity references */
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(1isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 1i32) as c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                            as c_int
                            == BT_LF as c_int
                        {
                            ptr = ptr.offset(1isize)
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => ptr = ptr.offset(1isize),
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }

    pub unsafe extern "C" fn big2_attributeValueTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut start: *const c_char = 0 as *const c_char;
        if ptr >= end {
            return XML_TOK_NONE;
        } else {
            if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
                return XML_TOK_PARTIAL;
            }
        }
        start = ptr;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                3 => {
                    if ptr == start {
                        return big2_scanRef(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (if *ptr.offset(0isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        }) == BT_LF as c_int
                        {
                            ptr = ptr.offset(2isize)
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => ptr = ptr.offset(2isize),
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }

    pub unsafe extern "C" fn little2_attributeValueTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut start: *const c_char = 0 as *const c_char;
        if ptr >= end {
            return XML_TOK_NONE;
        } else {
            if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
                return XML_TOK_PARTIAL;
            }
        }
        start = ptr;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                3 => {
                    if ptr == start {
                        return little2_scanRef(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (if *ptr.offset(1isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        }) == BT_LF as c_int
                        {
                            ptr = ptr.offset(2isize)
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => ptr = ptr.offset(2isize),
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }

    pub unsafe extern "C" fn big2_entityValueTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut start: *const c_char = 0 as *const c_char;
        if ptr >= end {
            return XML_TOK_NONE;
        } else {
            if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                return XML_TOK_PARTIAL;
                /* LCOV_EXCL_LINE */
                /* LCOV_EXCL_LINE */
                /* LCOV_EXCL_LINE */
            }
        }
        start = ptr;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                3 => {
                    if ptr == start {
                        return big2_scanRef(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                30 => {
                    if ptr == start {
                        let mut tok: c_int =
                            big2_scanPercent(enc, ptr.offset(2isize), end, nextTokPtr);
                        return if tok == XML_TOK_PERCENT {
                            XML_TOK_INVALID
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (if *ptr.offset(0isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        }) == BT_LF as c_int
                        {
                            ptr = ptr.offset(2isize)
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => ptr = ptr.offset(2isize),
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }

    pub unsafe extern "C" fn little2_entityValueTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut start: *const c_char = 0 as *const c_char;
        if ptr >= end {
            return XML_TOK_NONE;
        } else {
            if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long) {
                return XML_TOK_PARTIAL;
            }
        }
        start = ptr;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                3 => {
                    if ptr == start {
                        return little2_scanRef(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                30 => {
                    if ptr == start {
                        let mut tok: c_int = little2_scanPercent(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                        return if tok == XML_TOK_PERCENT {
                            XML_TOK_INVALID
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (if *ptr.offset(1isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        }) == BT_LF as c_int
                        {
                            ptr = ptr.offset(2isize)
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => ptr = ptr.offset(2isize),
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }

    pub unsafe extern "C" fn normal_entityValueTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut start: *const c_char = 0 as *const c_char;
        if ptr >= end {
            return XML_TOK_NONE;
        } else {
            if !(end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long) {
                return XML_TOK_PARTIAL;
            }
        }
        start = ptr;
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                3 => {
                    if ptr == start {
                        return normal_scanRef(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                30 => {
                    if ptr == start {
                        let mut tok: c_int = normal_scanPercent(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        );
                        return if tok == XML_TOK_PERCENT {
                            XML_TOK_INVALID
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(1isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 1i32) as c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                            as c_int
                            == BT_LF as c_int
                        {
                            ptr = ptr.offset(1isize)
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => ptr = ptr.offset(1isize),
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }

    pub unsafe extern "C" fn big2_ignoreSectionTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut level: c_int = 0i32;
        if 2i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (2i32 - 1i32) as c_ulong != 0 {
                n &= !(2i32 - 1i32) as c_ulong;
                end = ptr.offset(n as isize)
            }
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(2isize)
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(3isize)
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(4isize)
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                2 => {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(0isize) as c_int == 0i32
                        && *ptr.offset(1isize) as c_int == 0x21i32
                    {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(0isize) as c_int == 0i32
                            && *ptr.offset(1isize) as c_int == 0x5bi32
                        {
                            level += 1;
                            ptr = ptr.offset(2isize)
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(0isize) as c_int == 0i32
                        && *ptr.offset(1isize) as c_int == 0x5di32
                    {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(0isize) as c_int == 0i32
                            && *ptr.offset(1isize) as c_int == 0x3ei32
                        {
                            ptr = ptr.offset(2isize);
                            if level == 0i32 {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT;
                            }
                            level -= 1
                        }
                    }
                }
                _ => ptr = ptr.offset(2isize),
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn little2_ignoreSectionTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut level: c_int = 0i32;
        if 2i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (2i32 - 1i32) as c_ulong != 0 {
                n &= !(2i32 - 1i32) as c_ulong;
                end = ptr.offset(n as isize)
            }
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(2isize)
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(3isize)
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(4isize)
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                2 => {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(1isize) as c_int == 0i32
                        && *ptr.offset(0isize) as c_int == 0x21i32
                    {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(1isize) as c_int == 0i32
                            && *ptr.offset(0isize) as c_int == 0x5bi32
                        {
                            level += 1;
                            ptr = ptr.offset(2isize)
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(2isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(1isize) as c_int == 0i32
                        && *ptr.offset(0isize) as c_int == 0x5di32
                    {
                        ptr = ptr.offset(2isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 2i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(1isize) as c_int == 0i32
                            && *ptr.offset(0isize) as c_int == 0x3ei32
                        {
                            ptr = ptr.offset(2isize);
                            if level == 0i32 {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT;
                            }
                            level -= 1
                        }
                    }
                }
                _ => ptr = ptr.offset(2isize),
            }
        }
        return XML_TOK_PARTIAL;
    }

    pub unsafe extern "C" fn normal_ignoreSectionTok(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        let mut level: c_int = 0i32;
        if 1i32 > 1i32 {
            let mut n: size_t =  end.wrapping_offset_from(ptr) as size_t;
            if n & (1i32 - 1i32) as c_ulong != 0 {
                n &= !(1i32 - 1i32) as c_ulong;
                end = ptr.offset(n as isize)
            }
        }
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                5 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2isize)
                }
                6 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3isize)
                }
                7 => {
                    if (end.wrapping_offset_from(ptr) as c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4isize)
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                2 => {
                    ptr = ptr.offset(1isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr as c_int == 0x21i32 {
                        ptr = ptr.offset(1isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 1i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr as c_int == 0x5bi32 {
                            level += 1;
                            ptr = ptr.offset(1isize)
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(1isize);
                    if !(end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr as c_int == 0x5di32 {
                        ptr = ptr.offset(1isize);
                        if !(end.wrapping_offset_from(ptr) as c_long
                            >= (1i32 * 1i32) as c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr as c_int == 0x3ei32 {
                            ptr = ptr.offset(1isize);
                            if level == 0i32 {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT;
                            }
                            level -= 1
                        }
                    }
                }
                _ => ptr = ptr.offset(1isize),
            }
        }
        return XML_TOK_PARTIAL;
    }
    /* XML_DTD */
    /* XML_DTD */
    /* XML_DTD */

    pub unsafe extern "C" fn big2_isPublicId(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut badPtr: *mut *const c_char,
    ) -> c_int {
        ptr = ptr.offset(2isize);
        end = end.offset(-(2isize));
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_8: u64;
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    current_block_8 = 13242334135786603907;
                }
                21 => {
                    if *ptr.offset(0isize) as c_int == 0i32
                        && *ptr.offset(1isize) as c_int == 0x9i32
                    {
                        *badPtr = ptr;
                        return 0i32;
                    }
                    current_block_8 = 13242334135786603907;
                }
                26 | 22 => {
                    if (if *ptr.offset(0isize) as c_int == 0i32 {
                        *ptr.offset(1isize) as c_int
                    } else {
                        -1
                    }) & !(0x7fi32)
                        == 0
                    {
                        current_block_8 = 13242334135786603907;
                    } else {
                        current_block_8 = 1012613755837077285;
                    }
                }
                _ => {
                    current_block_8 = 1012613755837077285;
                }
            }
            match current_block_8 {
                1012613755837077285 =>
                /* fall through */
                /* fall through */
                /* fall through */
                {
                    match if *ptr.offset(0isize) as c_int == 0i32 {
                        *ptr.offset(1isize) as c_int
                    } else {
                        -1
                    } {
                        36 => {}
                        64 => {}
                        _ => {
                            *badPtr = ptr;
                            return 0i32;
                        }
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(2isize)
        }
        return 1i32;
    }

    pub unsafe extern "C" fn little2_isPublicId(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut badPtr: *mut *const c_char,
    ) -> c_int {
        ptr = ptr.offset(2isize);
        end = end.offset(-(2isize));
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            let mut current_block_8: u64;
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    current_block_8 = 13242334135786603907;
                }
                21 => {
                    if *ptr.offset(1isize) as c_int == 0i32
                        && *ptr.offset(0isize) as c_int == 0x9i32
                    {
                        *badPtr = ptr;
                        return 0i32;
                    }
                    current_block_8 = 13242334135786603907;
                }
                26 | 22 => {
                    if (if *ptr.offset(1isize) as c_int == 0i32 {
                        *ptr.offset(0isize) as c_int
                    } else {
                        -1
                    }) & !(0x7fi32)
                        == 0
                    {
                        current_block_8 = 13242334135786603907;
                    } else {
                        current_block_8 = 8842977063429728986;
                    }
                }
                _ => {
                    current_block_8 = 8842977063429728986;
                }
            }
            match current_block_8 {
                8842977063429728986 => {
                    match if *ptr.offset(1isize) as c_int == 0i32 {
                        *ptr.offset(0isize) as c_int
                    } else {
                        -1
                    } {
                        36 | 64 => {}
                        _ => {
                            *badPtr = ptr;
                            return 0i32;
                        }
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(2isize)
        }
        return 1i32;
    }

    pub unsafe extern "C" fn normal_isPublicId(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut badPtr: *mut *const c_char,
    ) -> c_int {
        ptr = ptr.offset(1isize);
        end = end.offset(-(1isize));
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            let mut current_block_8: u64;
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    current_block_8 = 13242334135786603907;
                }
                21 => {
                    if *ptr as c_int == 0x9i32 {
                        *badPtr = ptr;
                        return 0i32;
                    }
                    current_block_8 = 13242334135786603907;
                }
                26 | 22 => {
                    if *ptr as c_int & !(0x7fi32) == 0 {
                        current_block_8 = 13242334135786603907;
                    } else {
                        current_block_8 = 4503056668119438195;
                    }
                }
                _ => {
                    current_block_8 = 4503056668119438195;
                }
            }
            match current_block_8 {
                4503056668119438195 => match *ptr as c_int {
                    36 | 64 => {}
                    _ => {
                        *badPtr = ptr;
                        return 0i32;
                    }
                },
                _ => {}
            }
            ptr = ptr.offset(1isize)
        }
        return 1i32;
    }
    /* This must only be called for a well-formed start-tag or empty
       element tag.  Returns the number of attributes.  Pointers to the
       first attsMax attributes are stored in atts.
    */
    /* This must only be called for a well-formed start-tag or empty
       element tag.  Returns the number of attributes.  Pointers to the
       first attsMax attributes are stored in atts.
    */
    /* This must only be called for a well-formed start-tag or empty
       element tag.  Returns the number of attributes.  Pointers to the
       first attsMax attributes are stored in atts.
    */

    pub unsafe extern "C" fn normal_getAtts(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut attsMax: c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> c_int {
        let mut state: C2RustUnnamed_2 = inName;
        let mut nAtts: c_int = 0i32;
        let mut open: c_int = 0i32;
        /* defined when state == inValue;
        initialization just to shut up compilers */
        /* defined when state == inValue;
        initialization just to shut up compilers */
        /* defined when state == inValue;
        initialization just to shut up compilers */
        ptr = ptr.offset(1isize);
        loop {
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                5 => {
                    if  state ==  other {
                        if nAtts < attsMax {
                            let ref mut fresh0 = (*atts.offset(nAtts as isize)).name;
                            *fresh0 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName
                    }
                    ptr = ptr.offset((2i32 - 1i32) as isize)
                }
                6 => {
                    if  state ==  other {
                        if nAtts < attsMax {
                            let ref mut fresh1 = (*atts.offset(nAtts as isize)).name;
                            *fresh1 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName
                    }
                    ptr = ptr.offset((3i32 - 1i32) as isize)
                }
                7 => {
                    if  state ==  other {
                        if nAtts < attsMax {
                            let ref mut fresh2 = (*atts.offset(nAtts as isize)).name;
                            *fresh2 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName
                    }
                    ptr = ptr.offset((4i32 - 1i32) as isize)
                }
                29 | 22 | 24 => {
                    if  state ==  other {
                        if nAtts < attsMax {
                            let ref mut fresh3 = (*atts.offset(nAtts as isize)).name;
                            *fresh3 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName
                    }
                }
                12 => {
                    if  state !=  inValue {
                        if nAtts < attsMax {
                            let ref mut fresh4 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh4 = ptr.offset(1isize)
                        }
                        state = inValue;
                        open = BT_QUOT as c_int
                    } else if open == BT_QUOT as c_int {
                        state = other;
                        if nAtts < attsMax {
                            let ref mut fresh5 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh5 = ptr
                        }
                        nAtts += 1
                    }
                }
                13 => {
                    if  state !=  inValue {
                        if nAtts < attsMax {
                            let ref mut fresh6 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh6 = ptr.offset(1isize)
                        }
                        state = inValue;
                        open = BT_APOS as c_int
                    } else if open == BT_APOS as c_int {
                        state = other;
                        if nAtts < attsMax {
                            let ref mut fresh7 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh7 = ptr
                        }
                        nAtts += 1
                    }
                }
                3 => {
                    if nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8
                    }
                }
                21 => {
                    if  state ==  inName {
                        state = other
                    } else if  state ==  inValue
                        && nAtts < attsMax
                        && (*atts.offset(nAtts as isize)).normalized as c_int != 0
                        && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                            || *ptr != ASCII_SPACE
                            || *ptr.offset(1isize) == ASCII_SPACE
                            || (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                                == open)
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8
                    }
                }
                9 | 10 => {
                    /* This case ensures that the first attribute name is counted
                    Apart from that we could just change state on the quote. */
                    /* This case ensures that the first attribute name is counted
                    Apart from that we could just change state on the quote. */
                    /* This case ensures that the first attribute name is counted
                    Apart from that we could just change state on the quote. */
                    if  state ==  inName {
                        state = other
                    } else if  state ==  inValue && nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8
                    }
                }
                11 | 17 => {
                    if  state !=  inValue {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(1isize)
        }
        /* not reached */
        /* not reached */
        /* not reached */
    }

    pub unsafe extern "C" fn little2_getAtts(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut attsMax: c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> c_int {
        let mut state: C2RustUnnamed_2 = inName_0;
        let mut nAtts: c_int = 0i32;
        let mut open: c_int = 0i32;
        ptr = ptr.offset(2isize);
        loop {
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => {
                    if  state ==  other_0 {
                        if nAtts < attsMax {
                            let ref mut fresh8 = (*atts.offset(nAtts as isize)).name;
                            *fresh8 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName_0
                    }
                    ptr = ptr.offset((2i32 - 2i32) as isize)
                }
                6 => {
                    if  state ==  other_0 {
                        if nAtts < attsMax {
                            let ref mut fresh9 = (*atts.offset(nAtts as isize)).name;
                            *fresh9 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName_0
                    }
                    ptr = ptr.offset((3i32 - 2i32) as isize)
                }
                7 => {
                    if  state ==  other_0 {
                        if nAtts < attsMax {
                            let ref mut fresh10 = (*atts.offset(nAtts as isize)).name;
                            *fresh10 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName_0
                    }
                    ptr = ptr.offset((4i32 - 2i32) as isize)
                }
                29 | 22 | 24 => {
                    if  state ==  other_0 {
                        if nAtts < attsMax {
                            let ref mut fresh11 = (*atts.offset(nAtts as isize)).name;
                            *fresh11 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName_0
                    }
                }
                12 => {
                    if  state !=  inValue_0 {
                        if nAtts < attsMax {
                            let ref mut fresh12 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh12 = ptr.offset(2isize)
                        }
                        state = inValue_0;
                        open = BT_QUOT as c_int
                    } else if open == BT_QUOT as c_int {
                        state = other_0;
                        if nAtts < attsMax {
                            let ref mut fresh13 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh13 = ptr
                        }
                        nAtts += 1
                    }
                }
                13 => {
                    if  state !=  inValue_0 {
                        if nAtts < attsMax {
                            let ref mut fresh14 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh14 = ptr.offset(2isize)
                        }
                        state = inValue_0;
                        open = BT_APOS as c_int
                    } else if open == BT_APOS as c_int {
                        state = other_0;
                        if nAtts < attsMax {
                            let ref mut fresh15 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh15 = ptr
                        }
                        nAtts += 1
                    }
                }
                3 => {
                    if nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8
                    }
                }
                21 => {
                    if  state ==  inName_0 {
                        state = other_0
                    } else if  state ==  inValue_0
                        && nAtts < attsMax
                        && (*atts.offset(nAtts as isize)).normalized as c_int != 0
                        && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                            || (if *ptr.offset(1isize) as c_int == 0i32 {
                                *ptr.offset(0isize)
                            } else {
                                -1
                            }) != ASCII_SPACE
                            || (if *ptr.offset(2isize).offset(1isize)
                                as c_int
                                == 0i32
                            {
                                *ptr.offset(2isize).offset(0isize)
                            } else {
                                -1
                            }) == ASCII_SPACE
                            || (if *ptr.offset(2isize).offset(1isize)
                                as c_int
                                == 0i32
                            {
                                (*(enc as *mut normal_encoding)).type_0
                                    [*ptr.offset(2isize) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(2isize).offset(1isize),
                                    *ptr.offset(2isize).offset(0isize),
                                )
                            }) == open)
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8
                    }
                }
                9 | 10 => {
                    if  state ==  inName_0 {
                        state = other_0
                    } else if  state ==  inValue_0 && nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8
                    }
                }
                11 | 17 => {
                    if  state !=  inValue_0 {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(2isize)
        }
    }

    pub unsafe extern "C" fn big2_getAtts(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut attsMax: c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> c_int {
        let mut state: C2RustUnnamed_2 = inName_1;
        let mut nAtts: c_int = 0i32;
        let mut open: c_int = 0i32;
        ptr = ptr.offset(2isize);
        loop {
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => {
                    if  state ==  other_1 {
                        if nAtts < attsMax {
                            let ref mut fresh16 = (*atts.offset(nAtts as isize)).name;
                            *fresh16 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName_1
                    }
                    ptr = ptr.offset((2i32 - 2i32) as isize)
                }
                6 => {
                    if  state ==  other_1 {
                        if nAtts < attsMax {
                            let ref mut fresh17 = (*atts.offset(nAtts as isize)).name;
                            *fresh17 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName_1
                    }
                    ptr = ptr.offset((3i32 - 2i32) as isize)
                }
                7 => {
                    if  state ==  other_1 {
                        if nAtts < attsMax {
                            let ref mut fresh18 = (*atts.offset(nAtts as isize)).name;
                            *fresh18 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName_1
                    }
                    ptr = ptr.offset((4i32 - 2i32) as isize)
                }
                29 | 22 | 24 => {
                    if  state ==  other_1 {
                        if nAtts < attsMax {
                            let ref mut fresh19 = (*atts.offset(nAtts as isize)).name;
                            *fresh19 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8
                        }
                        state = inName_1
                    }
                }
                12 => {
                    if  state !=  inValue_1 {
                        if nAtts < attsMax {
                            let ref mut fresh20 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh20 = ptr.offset(2isize)
                        }
                        state = inValue_1;
                        open = BT_QUOT as c_int
                    } else if open == BT_QUOT as c_int {
                        state = other_1;
                        if nAtts < attsMax {
                            let ref mut fresh21 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh21 = ptr
                        }
                        nAtts += 1
                    }
                }
                13 => {
                    if  state !=  inValue_1 {
                        if nAtts < attsMax {
                            let ref mut fresh22 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh22 = ptr.offset(2isize)
                        }
                        state = inValue_1;
                        open = BT_APOS as c_int
                    } else if open == BT_APOS as c_int {
                        state = other_1;
                        if nAtts < attsMax {
                            let ref mut fresh23 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh23 = ptr
                        }
                        nAtts += 1
                    }
                }
                3 => {
                    if nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8
                    }
                }
                21 => {
                    if  state ==  inName_1 {
                        state = other_1
                    } else if  state ==  inValue_1
                        && nAtts < attsMax
                        && (*atts.offset(nAtts as isize)).normalized as c_int != 0
                        && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                            || (if *ptr.offset(0isize) as c_int == 0i32 {
                                *ptr.offset(1isize)
                            } else {
                                -1
                            }) != ASCII_SPACE
                            || (if *ptr.offset(2isize).offset(0isize)
                                as c_int
                                == 0i32
                            {
                                *ptr.offset(2isize).offset(1isize)
                            } else {
                                -1
                            }) == ASCII_SPACE
                            || (if *ptr.offset(2isize).offset(0isize)
                                as c_int
                                == 0i32
                            {
                                (*(enc as *mut normal_encoding)).type_0[*ptr
                                    .offset(2isize)
                                    .offset(1isize)
                                    as c_uchar
                                    as usize] as c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(2isize).offset(0isize),
                                    *ptr.offset(2isize).offset(1isize),
                                )
                            }) == open)
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8
                    }
                }
                9 | 10 => {
                    if  state ==  inName_1 {
                        state = other_1
                    } else if  state ==  inValue_1 && nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8
                    }
                }
                11 | 17 => {
                    if  state !=  inValue_1 {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(2isize)
        }
    }

    pub unsafe extern "C" fn little2_charRefNumber(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
    ) -> c_int {
        let mut result: c_int = 0i32;
        /* skip &# */
        /* skip &# */
        /* skip &# */
        ptr = ptr.offset((2i32 * 2i32) as isize);
        if *ptr.offset(1isize) as c_int == 0i32
            && *ptr.offset(0isize) as c_int == 0x78i32
        {
            ptr = ptr.offset(2isize);
            while !(*ptr.offset(1isize) as c_int == 0i32
                && *ptr.offset(0isize) as c_int == 0x3bi32)
            {
                let mut c = if *ptr.offset(1isize) as c_int == 0i32 {
                    *ptr.offset(0isize)
                } else {
                    -1
                };
                match c {
                    ASCII_0 | ASCII_1 | ASCII_2 | ASCII_3 | ASCII_4 | ASCII_5 | ASCII_6
                    | ASCII_7 | ASCII_8 | ASCII_9 => {
                        result <<= 4i32;
                        result |= (c - ASCII_0) as c_int
                    }
                    ASCII_A | ASCII_B | ASCII_C | ASCII_D | ASCII_E | ASCII_F => {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_A) as c_int
                    }
                    ASCII_a | ASCII_b | ASCII_c | ASCII_d | ASCII_e | ASCII_f => {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_a) as c_int
                    }
                    _ => {}
                }
                if result >= 0x110000i32 {
                    return -1;
                }
                ptr = ptr.offset(2isize)
            }
        } else {
            while !(*ptr.offset(1isize) as c_int == 0i32
                && *ptr.offset(0isize) as c_int == 0x3bi32)
            {
                let mut c_0: c_int = if *ptr.offset(1isize) as c_int == 0i32 {
                    *ptr.offset(0isize) as c_int
                } else {
                    -1
                };
                result *= 10i32;
                result += c_0 - ASCII_0 as c_int;
                if result >= 0x110000i32 {
                    return -1;
                }
                ptr = ptr.offset(2isize)
            }
        }
        return checkCharRefNumber(result);
    }

    pub unsafe extern "C" fn normal_charRefNumber(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
    ) -> c_int {
        let mut result: c_int = 0i32;
        ptr = ptr.offset((2i32 * 1i32) as isize);
        if *ptr as c_int == 0x78i32 {
            ptr = ptr.offset(1isize);
            while !(*ptr as c_int == 0x3bi32) {
                let mut c = *ptr;
                match c {
                    ASCII_0 | ASCII_1 | ASCII_2 | ASCII_3 | ASCII_4 | ASCII_5 | ASCII_6
                    | ASCII_7 | ASCII_8 | ASCII_9 => {
                        result <<= 4i32;
                        result |= (c - ASCII_0) as c_int
                    }
                    ASCII_A | ASCII_B | ASCII_C | ASCII_D | ASCII_E | ASCII_F => {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_A) as c_int
                    }
                    ASCII_a | ASCII_b | ASCII_c | ASCII_d | ASCII_e | ASCII_f => {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_a) as c_int
                    }
                    _ => {}
                }
                if result >= 0x110000i32 {
                    return -1;
                }
                ptr = ptr.offset(1isize)
            }
        } else {
            while !(*ptr as c_int == 0x3bi32) {
                let mut c_0: c_int = *ptr as c_int;
                result *= 10i32;
                result += c_0 - ASCII_0 as c_int;
                if result >= 0x110000i32 {
                    return -1;
                }
                ptr = ptr.offset(1isize)
            }
        }
        return checkCharRefNumber(result);
    }

    pub unsafe extern "C" fn big2_charRefNumber(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
    ) -> c_int {
        let mut result: c_int = 0i32;
        ptr = ptr.offset((2i32 * 2i32) as isize);
        if *ptr.offset(0isize) as c_int == 0i32
            && *ptr.offset(1isize) as c_int == 0x78i32
        {
            ptr = ptr.offset(2isize);
            while !(*ptr.offset(0isize) as c_int == 0i32
                && *ptr.offset(1isize) as c_int == 0x3bi32)
            {
                let mut c = if *ptr.offset(0isize) as c_int == 0i32 {
                    *ptr.offset(1isize)
                } else {
                    -1
                };
                match c {
                    ASCII_0 | ASCII_1 | ASCII_2 | ASCII_3 | ASCII_4 | ASCII_5 | ASCII_6
                    | ASCII_7 | ASCII_8 | ASCII_9 => {
                        result <<= 4i32;
                        result |= (c - ASCII_0) as c_int
                    }
                    ASCII_A | ASCII_B | ASCII_C | ASCII_D | ASCII_E | ASCII_F => {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_A) as c_int
                    }
                    ASCII_a | ASCII_b | ASCII_c | ASCII_d | ASCII_e | ASCII_f => {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_a) as c_int
                    }
                    _ => {}
                }
                if result >= 0x110000i32 {
                    return -1;
                }
                ptr = ptr.offset(2isize)
            }
        } else {
            while !(*ptr.offset(0isize) as c_int == 0i32
                && *ptr.offset(1isize) as c_int == 0x3bi32)
            {
                let mut c_0 = if *ptr.offset(0isize) as c_int == 0i32 {
                    *ptr.offset(1isize)
                } else {
                    -1
                };
                result *= 10i32;
                result += (c_0 - ASCII_0) as c_int;
                if result >= 0x110000i32 {
                    return -1;
                }
                ptr = ptr.offset(2isize)
            }
        }
        return checkCharRefNumber(result);
    }

    pub unsafe extern "C" fn little2_predefinedEntityName(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
    ) -> c_int {
        match end.wrapping_offset_from(ptr) as c_long / 2i64 {
            2 => {
                if *ptr.offset(2isize).offset(1isize) as c_int
                    == 0i32
                    && *ptr.offset(2isize).offset(0isize) as c_int
                        == 0x74i32
                {
                    match if *ptr.offset(1isize) as c_int == 0i32 {
                        *ptr.offset(0isize)
                    } else {
                        -1
                    } {
                        ASCII_l => return ASCII_LT as c_int,
                        ASCII_g => return ASCII_GT as c_int,
                        _ => {}
                    }
                }
            }
            3 => {
                if *ptr.offset(1isize) as c_int == 0i32
                    && *ptr.offset(0isize) as c_int == 0x61i32
                {
                    ptr = ptr.offset(2isize);
                    if *ptr.offset(1isize) as c_int == 0i32
                        && *ptr.offset(0isize) as c_int == 0x6di32
                    {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(1isize) as c_int == 0i32
                            && *ptr.offset(0isize) as c_int == 0x70i32
                        {
                            return ASCII_AMP as c_int;
                        }
                    }
                }
            }
            4 => {
                match if *ptr.offset(1isize) as c_int == 0i32 {
                    *ptr.offset(0isize)
                } else {
                    -1
                } {
                    ASCII_q => {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(1isize) as c_int == 0i32
                            && *ptr.offset(0isize) as c_int == 0x75i32
                        {
                            ptr = ptr.offset(2isize);
                            if *ptr.offset(1isize) as c_int == 0i32
                                && *ptr.offset(0isize) as c_int == 0x6fi32
                            {
                                ptr = ptr.offset(2isize);
                                if *ptr.offset(1isize) as c_int == 0i32
                                    && *ptr.offset(0isize) as c_int == 0x74i32
                                {
                                    return ASCII_QUOT as c_int;
                                }
                            }
                        }
                    }
                    ASCII_a => {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(1isize) as c_int == 0i32
                            && *ptr.offset(0isize) as c_int == 0x70i32
                        {
                            ptr = ptr.offset(2isize);
                            if *ptr.offset(1isize) as c_int == 0i32
                                && *ptr.offset(0isize) as c_int == 0x6fi32
                            {
                                ptr = ptr.offset(2isize);
                                if *ptr.offset(1isize) as c_int == 0i32
                                    && *ptr.offset(0isize) as c_int == 0x73i32
                                {
                                    return ASCII_APOS as c_int;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return 0i32;
    }

    pub unsafe extern "C" fn normal_predefinedEntityName(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
    ) -> c_int {
        match end.wrapping_offset_from(ptr) as c_long / 1i64 {
            2 => {
                if *ptr.offset(1isize) as c_int == 0x74i32 {
                    match *ptr {
                        ASCII_l => return ASCII_LT as c_int,
                        ASCII_g => return ASCII_GT as c_int,
                        _ => {}
                    }
                }
            }
            3 => {
                if *ptr as c_int == 0x61i32 {
                    ptr = ptr.offset(1isize);
                    if *ptr as c_int == 0x6di32 {
                        ptr = ptr.offset(1isize);
                        if *ptr as c_int == 0x70i32 {
                            return ASCII_AMP as c_int;
                        }
                    }
                }
            }
            4 => match *ptr {
                ASCII_q => {
                    ptr = ptr.offset(1isize);
                    if *ptr as c_int == 0x75i32 {
                        ptr = ptr.offset(1isize);
                        if *ptr as c_int == 0x6fi32 {
                            ptr = ptr.offset(1isize);
                            if *ptr as c_int == 0x74i32 {
                                return ASCII_QUOT as c_int;
                            }
                        }
                    }
                }
                ASCII_a => {
                    ptr = ptr.offset(1isize);
                    if *ptr as c_int == 0x70i32 {
                        ptr = ptr.offset(1isize);
                        if *ptr as c_int == 0x6fi32 {
                            ptr = ptr.offset(1isize);
                            if *ptr as c_int == 0x73i32 {
                                return ASCII_APOS as c_int;
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        return 0i32;
    }

    pub unsafe extern "C" fn big2_predefinedEntityName(
        mut _enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
    ) -> c_int {
        match end.wrapping_offset_from(ptr) as c_long / 2i64 {
            2 => {
                if *ptr.offset(2isize).offset(0isize) as c_int
                    == 0i32
                    && *ptr.offset(2isize).offset(1isize) as c_int
                        == 0x74i32
                {
                    match if *ptr.offset(0isize) as c_int == 0i32 {
                        *ptr.offset(1isize)
                    } else {
                        -1
                    } {
                        ASCII_l => return ASCII_LT as c_int,
                        ASCII_g => return ASCII_GT as c_int,
                        _ => {}
                    }
                }
            }
            3 => {
                if *ptr.offset(0isize) as c_int == 0i32
                    && *ptr.offset(1isize) as c_int == 0x61i32
                {
                    ptr = ptr.offset(2isize);
                    if *ptr.offset(0isize) as c_int == 0i32
                        && *ptr.offset(1isize) as c_int == 0x6di32
                    {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(0isize) as c_int == 0i32
                            && *ptr.offset(1isize) as c_int == 0x70i32
                        {
                            return ASCII_AMP as c_int;
                        }
                    }
                }
            }
            4 => {
                match if *ptr.offset(0isize) as c_int == 0i32 {
                    *ptr.offset(1isize)
                } else {
                    -1
                } {
                    ASCII_q => {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(0isize) as c_int == 0i32
                            && *ptr.offset(1isize) as c_int == 0x75i32
                        {
                            ptr = ptr.offset(2isize);
                            if *ptr.offset(0isize) as c_int == 0i32
                                && *ptr.offset(1isize) as c_int == 0x6fi32
                            {
                                ptr = ptr.offset(2isize);
                                if *ptr.offset(0isize) as c_int == 0i32
                                    && *ptr.offset(1isize) as c_int == 0x74i32
                                {
                                    return ASCII_QUOT as c_int;
                                }
                            }
                        }
                    }
                    ASCII_a => {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(0isize) as c_int == 0i32
                            && *ptr.offset(1isize) as c_int == 0x70i32
                        {
                            ptr = ptr.offset(2isize);
                            if *ptr.offset(0isize) as c_int == 0i32
                                && *ptr.offset(1isize) as c_int == 0x6fi32
                            {
                                ptr = ptr.offset(2isize);
                                if *ptr.offset(0isize) as c_int == 0i32
                                    && *ptr.offset(1isize) as c_int == 0x73i32
                                {
                                    return ASCII_APOS as c_int;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return 0i32;
    }

    pub unsafe extern "C" fn normal_nameMatchesAscii(
        mut _enc: *const ENCODING,
        mut ptr1: *const c_char,
        mut end1: *const c_char,
        mut ptr2: *const c_char,
    ) -> c_int {
        while *ptr2 != 0 {
            if (end1.wrapping_offset_from(ptr1) as c_long) < 1i64 {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the
                 * paranoia check is still valuable, however.
                 */
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the
                 * paranoia check is still valuable, however.
                 */
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the
                 * paranoia check is still valuable, however.
                 */
                return 0i32;
                /* LCOV_EXCL_LINE */
                /* LCOV_EXCL_LINE */
                /* LCOV_EXCL_LINE */
            }
            if !(*ptr1 as c_int == *ptr2 as c_int) {
                return 0i32;
            }
            ptr1 = ptr1.offset(1isize);
            ptr2 = ptr2.offset(1)
        }
        return (ptr1 == end1) as c_int;
    }

    pub unsafe extern "C" fn big2_nameMatchesAscii(
        mut _enc: *const ENCODING,
        mut ptr1: *const c_char,
        mut end1: *const c_char,
        mut ptr2: *const c_char,
    ) -> c_int {
        while *ptr2 != 0 {
            if (end1.wrapping_offset_from(ptr1) as c_long) < 2i64 {
                return 0i32;
            }
            if !(*ptr1.offset(0isize) as c_int == 0i32
                && *ptr1.offset(1isize) as c_int == *ptr2 as c_int)
            {
                return 0i32;
            }
            ptr1 = ptr1.offset(2isize);
            ptr2 = ptr2.offset(1)
        }
        return (ptr1 == end1) as c_int;
    }

    pub unsafe extern "C" fn little2_nameMatchesAscii(
        mut _enc: *const ENCODING,
        mut ptr1: *const c_char,
        mut end1: *const c_char,
        mut ptr2: *const c_char,
    ) -> c_int {
        while *ptr2 != 0 {
            if (end1.wrapping_offset_from(ptr1) as c_long) < 2i64 {
                return 0i32;
            }
            if !(*ptr1.offset(1isize) as c_int == 0i32
                && *ptr1.offset(0isize) as c_int == *ptr2 as c_int)
            {
                return 0i32;
            }
            ptr1 = ptr1.offset(2isize);
            ptr2 = ptr2.offset(1)
        }
        return (ptr1 == end1) as c_int;
    }

    pub unsafe extern "C" fn normal_nameLength(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
    ) -> c_int {
        let mut start: *const c_char = ptr;
        loop {
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                29 | 22 | 23 | 24 | 25 | 26 | 27 => ptr = ptr.offset(1isize),
                _ => return  ptr.wrapping_offset_from(start) as c_int,
            }
        }
    }

    pub unsafe extern "C" fn big2_nameLength(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
    ) -> c_int {
        let mut start: *const c_char = ptr;
        loop {
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                29 | 22 | 23 | 24 | 25 | 26 | 27 => ptr = ptr.offset(2isize),
                _ => return  ptr.wrapping_offset_from(start) as c_int,
            }
        }
    }

    pub unsafe extern "C" fn little2_nameLength(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
    ) -> c_int {
        let mut start: *const c_char = ptr;
        loop {
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                29 | 22 | 23 | 24 | 25 | 26 | 27 => ptr = ptr.offset(2isize),
                _ => return  ptr.wrapping_offset_from(start) as c_int,
            }
        }
    }

    pub unsafe extern "C" fn little2_skipS(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
    ) -> *const c_char {
        loop {
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                10 | 9 | 21 => ptr = ptr.offset(2isize),
                _ => return ptr,
            }
        }
    }

    pub unsafe extern "C" fn big2_skipS(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
    ) -> *const c_char {
        loop {
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                10 | 9 | 21 => ptr = ptr.offset(2isize),
                _ => return ptr,
            }
        }
    }

    pub unsafe extern "C" fn normal_skipS(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
    ) -> *const c_char {
        loop {
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                10 | 9 | 21 => ptr = ptr.offset(1isize),
                _ => return ptr,
            }
        }
    }

    pub unsafe extern "C" fn normal_updatePosition(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut pos: *mut POSITION,
    ) {
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 1i32) as c_long {
            match (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                10 => {
                    (*pos).columnNumber = -(1i32) as XML_Size;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(1isize)
                }
                9 => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(1isize);
                    if end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 1i32) as c_long
                        && (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                            as c_int
                            == BT_LF as c_int
                    {
                        ptr = ptr.offset(1isize)
                    }
                    (*pos).columnNumber = -(1i32) as XML_Size
                }
                _ => ptr = ptr.offset(1isize),
            }
            (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1)
        }
    }

    pub unsafe extern "C" fn big2_updatePosition(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut pos: *mut POSITION,
    ) {
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(0isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0
                    [*ptr.offset(1isize) as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                10 => {
                    (*pos).columnNumber = -(1i32) as XML_Size;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2isize)
                }
                9 => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2isize);
                    if end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long
                        && (if *ptr.offset(0isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0
                                [*ptr.offset(1isize) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        }) == BT_LF as c_int
                    {
                        ptr = ptr.offset(2isize)
                    }
                    (*pos).columnNumber = -(1i32) as XML_Size
                }
                _ => ptr = ptr.offset(2isize),
            }
            (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1)
        }
    }

    pub unsafe extern "C" fn little2_updatePosition(
        mut enc: *const ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut pos: *mut POSITION,
    ) {
        while end.wrapping_offset_from(ptr) as c_long >= (1i32 * 2i32) as c_long {
            match if *ptr.offset(1isize) as c_int == 0i32 {
                (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize] as c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => ptr = ptr.offset(2isize),
                6 => ptr = ptr.offset(3isize),
                7 => ptr = ptr.offset(4isize),
                10 => {
                    (*pos).columnNumber = -(1i32) as XML_Size;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2isize)
                }
                9 => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2isize);
                    if end.wrapping_offset_from(ptr) as c_long
                        >= (1i32 * 2i32) as c_long
                        && (if *ptr.offset(1isize) as c_int == 0i32 {
                            (*(enc as *mut normal_encoding)).type_0[*ptr as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        }) == BT_LF as c_int
                    {
                        ptr = ptr.offset(2isize)
                    }
                    (*pos).columnNumber = -(1i32) as XML_Size
                }
                _ => ptr = ptr.offset(2isize),
            }
            (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1)
        }
    }

    use crate::src::lib::xmltok::nametab_h::{namePages, namingBitmap, nmstrtPages};
    use crate::src::lib::xmltok::{checkCharRefNumber, normal_encoding, unicode_byte_type};

    /* XML_TOK_IMPL_C */
    /* XML_TOK_IMPL_C */
    /* XML_TOK_IMPL_C */
}

pub mod nametab_h {
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

    use libc::{c_int, c_uchar, c_uint};
    pub static mut namingBitmap: [c_uint; 320] = [
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0u32,
        0x4000000u32,
        0x87fffffeu32,
        0x7fffffeu32,
        0u32,
        0u32,
        0xff7fffffu32,
        0xff7fffffu32,
        0xffffffffu32,
        0x7ff3ffffu32,
        0xfffffdfeu32,
        0x7fffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffe00fu32,
        0xfc31ffffu32,
        0xffffffu32,
        0u32,
        0xffff0000u32,
        0xffffffffu32,
        0xffffffffu32,
        0xf80001ffu32,
        0x3u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xffffd740u32,
        0xfffffffbu32,
        0x547f7fffu32,
        0xffffdu32,
        0xffffdffeu32,
        0xffffffffu32,
        0xdffeffffu32,
        0xffffffffu32,
        0xffff0003u32,
        0xffffffffu32,
        0xffff199fu32,
        0x33fcfffu32,
        0u32,
        0xfffe0000u32,
        0x27fffffu32,
        0xfffffffeu32,
        0x7fu32,
        0u32,
        0xffff0000u32,
        0x707ffu32,
        0u32,
        0x7fffffeu32,
        0x7feu32,
        0xfffe0000u32,
        0xffffffffu32,
        0x7cffffffu32,
        0x2f7fffu32,
        0x60u32,
        0xffffffe0u32,
        0x23ffffffu32,
        0xff000000u32,
        0x3u32,
        0xfff99fe0u32,
        0x3c5fdffu32,
        0xb0000000u32,
        0x30003u32,
        0xfff987e0u32,
        0x36dfdffu32,
        0x5e000000u32,
        0x1c0000u32,
        0xfffbafe0u32,
        0x23edfdffu32,
        0u32,
        0x1u32,
        0xfff99fe0u32,
        0x23cdfdffu32,
        0xb0000000u32,
        0x3u32,
        0xd63dc7e0u32,
        0x3bfc718u32,
        0u32,
        0u32,
        0xfffddfe0u32,
        0x3effdffu32,
        0u32,
        0x3u32,
        0xfffddfe0u32,
        0x3effdffu32,
        0x40000000u32,
        0x3u32,
        0xfffddfe0u32,
        0x3fffdffu32,
        0u32,
        0x3u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xfffffffeu32,
        0xd7fffu32,
        0x3fu32,
        0u32,
        0xfef02596u32,
        0x200d6caeu32,
        0x1fu32,
        0u32,
        0u32,
        0u32,
        0xfffffeffu32,
        0x3ffu32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xffffffffu32,
        0xffff003fu32,
        0x7fffffu32,
        0x7daedu32,
        0x50000000u32,
        0x82315001u32,
        0x2c62abu32,
        0x40000000u32,
        0xf580c900u32,
        0x7u32,
        0x2010800u32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xfffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0x3ffffffu32,
        0x3f3fffffu32,
        0xffffffffu32,
        0xaaff3f3fu32,
        0x3fffffffu32,
        0xffffffffu32,
        0x5fdfffffu32,
        0xfcf1fdcu32,
        0x1fdc1fffu32,
        0u32,
        0x4c40u32,
        0u32,
        0u32,
        0x7u32,
        0u32,
        0u32,
        0u32,
        0x80u32,
        0x3feu32,
        0xfffffffeu32,
        0xffffffffu32,
        0x1fffffu32,
        0xfffffffeu32,
        0xffffffffu32,
        0x7ffffffu32,
        0xffffffe0u32,
        0x1fffu32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0x3fu32,
        0u32,
        0u32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xfu32,
        0u32,
        0u32,
        0u32,
        0x7ff6000u32,
        0x87fffffeu32,
        0x7fffffeu32,
        0u32,
        0x800000u32,
        0xff7fffffu32,
        0xff7fffffu32,
        0xffffffu32,
        0u32,
        0xffff0000u32,
        0xffffffffu32,
        0xffffffffu32,
        0xf80001ffu32,
        0x30003u32,
        0u32,
        0xffffffffu32,
        0xffffffffu32,
        0x3fu32,
        0x3u32,
        0xffffd7c0u32,
        0xfffffffbu32,
        0x547f7fffu32,
        0xffffdu32,
        0xffffdffeu32,
        0xffffffffu32,
        0xdffeffffu32,
        0xffffffffu32,
        0xffff007bu32,
        0xffffffffu32,
        0xffff199fu32,
        0x33fcfffu32,
        0u32,
        0xfffe0000u32,
        0x27fffffu32,
        0xfffffffeu32,
        0xfffe007fu32,
        0xbbfffffbu32,
        0xffff0016u32,
        0x707ffu32,
        0u32,
        0x7fffffeu32,
        0x7ffffu32,
        0xffff03ffu32,
        0xffffffffu32,
        0x7cffffffu32,
        0xffef7fffu32,
        0x3ff3dffu32,
        0xffffffeeu32,
        0xf3ffffffu32,
        0xff1e3fffu32,
        0xffcfu32,
        0xfff99feeu32,
        0xd3c5fdffu32,
        0xb080399fu32,
        0x3ffcfu32,
        0xfff987e4u32,
        0xd36dfdffu32,
        0x5e003987u32,
        0x1fffc0u32,
        0xfffbafeeu32,
        0xf3edfdffu32,
        0x3bbfu32,
        0xffc1u32,
        0xfff99feeu32,
        0xf3cdfdffu32,
        0xb0c0398fu32,
        0xffc3u32,
        0xd63dc7ecu32,
        0xc3bfc718u32,
        0x803dc7u32,
        0xff80u32,
        0xfffddfeeu32,
        0xc3effdffu32,
        0x603ddfu32,
        0xffc3u32,
        0xfffddfecu32,
        0xc3effdffu32,
        0x40603ddfu32,
        0xffc3u32,
        0xfffddfecu32,
        0xc3fffdffu32,
        0x803dcfu32,
        0xffc3u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xfffffffeu32,
        0x7ff7fffu32,
        0x3ff7fffu32,
        0u32,
        0xfef02596u32,
        0x3bff6caeu32,
        0x3ff3f5fu32,
        0u32,
        0x3000000u32,
        0xc2a003ffu32,
        0xfffffeffu32,
        0xfffe03ffu32,
        0xfebf0fdfu32,
        0x2fe3fffu32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0x1fff0000u32,
        0x2u32,
        0xa0u32,
        0x3efffeu32,
        0xfffffffeu32,
        0xffffffffu32,
        0x661fffffu32,
        0xfffffffeu32,
        0xffffffffu32,
        0x77ffffffu32,
    ];

    pub static mut nmstrtPages: [c_uchar; 256] = [
        0x2u8,
        0x3u8,
        0x4u8,
        0x5u8,
        0x6u8,
        0x7u8,
        0x8u8,
        0u8,
        0u8,
        0x9u8,
        0xau8,
        0xbu8,
        0xcu8,
        0xdu8,
        0xeu8,
        0xfu8,
        0x10u8,
        0x11u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x12u8,
        0x13u8,
        0u8,
        0x14u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x15u8,
        0x16u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x17u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x18u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
    ];

    pub static mut namePages: [c_uchar; 256] = [
        0x19u8,
        0x3u8,
        0x1au8,
        0x1bu8,
        0x1cu8,
        0x1du8,
        0x1eu8,
        0u8,
        0u8,
        0x1fu8,
        0x20u8,
        0x21u8,
        0x22u8,
        0x23u8,
        0x24u8,
        0x25u8,
        0x10u8,
        0x11u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x12u8,
        0x13u8,
        0x26u8,
        0x14u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x27u8,
        0x16u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x17u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x18u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
    ];
}

pub mod xmltok_ns_c {
    /* This file is included!
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
    /* This file is included!
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

    use libc::{c_char, c_int};
    #[no_mangle]

    pub unsafe extern "C" fn XmlGetUtf8InternalEncodingNS() -> *const super::ENCODING {
        return &internal_utf8_encoding_ns.enc;
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlGetUtf8InternalEncoding() -> *const super::ENCODING {
        return &internal_utf8_encoding.enc;
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlGetUtf16InternalEncoding() -> *const super::ENCODING {
        return &internal_little2_encoding.enc;
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlGetUtf16InternalEncodingNS() -> *const super::ENCODING {
        return &internal_little2_encoding_ns.enc;
    }
    // Initialized in run_static_initializers

    pub static mut encodingsNS: [*const super::ENCODING; 7] = [0 as *const super::ENCODING; 7];
    // Initialized in run_static_initializers

    pub static mut encodings: [*const super::ENCODING; 7] = [0 as *const super::ENCODING; 7];

    pub unsafe extern "C" fn initScanProlog(
        mut enc: *const super::ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        return initScan(
            encodings.as_ptr(),
            enc as *const super::INIT_ENCODING,
            super::XML_PROLOG_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub unsafe extern "C" fn initScanPrologNS(
        mut enc: *const super::ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        return initScan(
            encodingsNS.as_ptr(),
            enc as *const super::INIT_ENCODING,
            super::XML_PROLOG_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub unsafe extern "C" fn initScanContentNS(
        mut enc: *const super::ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        return initScan(
            encodingsNS.as_ptr(),
            enc as *const super::INIT_ENCODING,
            super::XML_CONTENT_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub unsafe extern "C" fn initScanContent(
        mut enc: *const super::ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        return initScan(
            encodings.as_ptr(),
            enc as *const super::INIT_ENCODING,
            super::XML_CONTENT_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlInitEncodingNS(
        mut p: *mut super::INIT_ENCODING,
        mut encPtr: *mut *const super::ENCODING,
        mut name: *const c_char,
    ) -> c_int {
        let mut i: c_int = getEncodingIndex(name);
        if i ==  UNKNOWN_ENC {
            return 0i32;
        }
        (*p).initEnc.isUtf16 = i as c_char;
        (*p).initEnc.scanners[super::XML_PROLOG_STATE as usize] = Some(
            initScanPrologNS
                as unsafe extern "C" fn(
                    _: *const super::ENCODING,
                    _: *const c_char,
                    _: *const c_char,
                    _: *mut *const c_char,
                ) -> c_int,
        );
        (*p).initEnc.scanners[super::XML_CONTENT_STATE as usize] = Some(
            initScanContentNS
                as unsafe extern "C" fn(
                    _: *const super::ENCODING,
                    _: *const c_char,
                    _: *const c_char,
                    _: *mut *const c_char,
                ) -> c_int,
        );
        (*p).initEnc.updatePosition = Some(
            initUpdatePosition
                as unsafe extern "C" fn(
                    _: *const super::ENCODING,
                    _: *const c_char,
                    _: *const c_char,
                    _: *mut super::POSITION,
                ) -> (),
        );
        (*p).encPtr = encPtr;
        *encPtr = &mut (*p).initEnc;
        return 1i32;
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlInitEncoding(
        mut p: *mut super::INIT_ENCODING,
        mut encPtr: *mut *const super::ENCODING,
        mut name: *const c_char,
    ) -> c_int {
        let mut i: c_int = getEncodingIndex(name);
        if i ==  UNKNOWN_ENC {
            return 0i32;
        }
        (*p).initEnc.isUtf16 = i as c_char;
        (*p).initEnc.scanners[super::XML_PROLOG_STATE as usize] = Some(
            initScanProlog
                as unsafe extern "C" fn(
                    _: *const super::ENCODING,
                    _: *const c_char,
                    _: *const c_char,
                    _: *mut *const c_char,
                ) -> c_int,
        );
        (*p).initEnc.scanners[super::XML_CONTENT_STATE as usize] = Some(
            initScanContent
                as unsafe extern "C" fn(
                    _: *const super::ENCODING,
                    _: *const c_char,
                    _: *const c_char,
                    _: *mut *const c_char,
                ) -> c_int,
        );
        (*p).initEnc.updatePosition = Some(
            initUpdatePosition
                as unsafe extern "C" fn(
                    _: *const super::ENCODING,
                    _: *const c_char,
                    _: *const c_char,
                    _: *mut super::POSITION,
                ) -> (),
        );
        (*p).encPtr = encPtr;
        *encPtr = &mut (*p).initEnc;
        return 1i32;
    }

    pub unsafe extern "C" fn findEncoding(
        mut enc: *const super::ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
    ) -> *const super::ENCODING {
        let mut buf: [c_char; 128] = [0; 128];
        let mut p: *mut c_char = buf.as_mut_ptr();
        let mut i: c_int = 0;
        (*enc).utf8Convert.expect("non-null function pointer")(
            enc,
            &mut ptr,
            end,
            &mut p,
            p.offset(128isize)
                .offset(-(1isize)),
        );
        if ptr != end {
            return 0 as *const super::ENCODING;
        }
        *p = 0i8;
        if streqci(buf.as_mut_ptr(), KW_UTF_16.as_ptr()) != 0
            && (*enc).minBytesPerChar == 2i32
        {
            return enc;
        }
        i = getEncodingIndex(buf.as_mut_ptr());
        if i ==  UNKNOWN_ENC {
            return 0 as *const super::ENCODING;
        }
        return encodings[i as usize];
    }

    pub unsafe extern "C" fn findEncodingNS(
        mut enc: *const super::ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
    ) -> *const super::ENCODING {
        let mut buf: [c_char; 128] = [0; 128];
        let mut p: *mut c_char = buf.as_mut_ptr();
        let mut i: c_int = 0;
        (*enc).utf8Convert.expect("non-null function pointer")(
            enc,
            &mut ptr,
            end,
            &mut p,
            p.offset(128isize)
                .offset(-(1isize)),
        );
        if ptr != end {
            return 0 as *const super::ENCODING;
        }
        *p = 0i8;
        if streqci(buf.as_mut_ptr(), KW_UTF_16.as_ptr()) != 0
            && (*enc).minBytesPerChar == 2i32
        {
            return enc;
        }
        i = getEncodingIndex(buf.as_mut_ptr());
        if i ==  UNKNOWN_ENC {
            return 0 as *const super::ENCODING;
        }
        return encodingsNS[i as usize];
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlParseXmlDeclNS(
        mut isGeneralTextEntity: c_int,
        mut enc: *const super::ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut badPtr: *mut *const c_char,
        mut versionPtr: *mut *const c_char,
        mut versionEndPtr: *mut *const c_char,
        mut encodingName: *mut *const c_char,
        mut encoding: *mut *const super::ENCODING,
        mut standalone: *mut c_int,
    ) -> c_int {
        return doParseXmlDecl(
            Some(
                findEncodingNS
                    as unsafe extern "C" fn(
                        _: *const super::ENCODING,
                        _: *const c_char,
                        _: *const c_char,
                    ) -> *const super::ENCODING,
            ),
            isGeneralTextEntity,
            enc,
            ptr,
            end,
            badPtr,
            versionPtr,
            versionEndPtr,
            encodingName,
            encoding,
            standalone,
        );
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlParseXmlDecl(
        mut isGeneralTextEntity: c_int,
        mut enc: *const super::ENCODING,
        mut ptr: *const c_char,
        mut end: *const c_char,
        mut badPtr: *mut *const c_char,
        mut versionPtr: *mut *const c_char,
        mut versionEndPtr: *mut *const c_char,
        mut encodingName: *mut *const c_char,
        mut encoding: *mut *const super::ENCODING,
        mut standalone: *mut c_int,
    ) -> c_int {
        return doParseXmlDecl(
            Some(
                findEncoding
                    as unsafe extern "C" fn(
                        _: *const super::ENCODING,
                        _: *const c_char,
                        _: *const c_char,
                    ) -> *const super::ENCODING,
            ),
            isGeneralTextEntity,
            enc,
            ptr,
            end,
            badPtr,
            versionPtr,
            versionEndPtr,
            encodingName,
            encoding,
            standalone,
        );
    }

    use crate::src::lib::xmltok::{
        doParseXmlDecl, getEncodingIndex, initScan, initUpdatePosition, internal_little2_encoding,
        internal_little2_encoding_ns, internal_utf8_encoding, internal_utf8_encoding_ns, streqci,
        ENCODING, INIT_ENCODING, KW_UTF_16, POSITION, UNKNOWN_ENC, XML_CONTENT_STATE,
        XML_PROLOG_STATE,
    };
    /* XML_TOK_NS_C */
    /* XML_TOK_NS_C */
}

pub use crate::ascii_h::{
    ASCII_a, ASCII_b, ASCII_c, ASCII_d, ASCII_e, ASCII_f, ASCII_g, ASCII_i, ASCII_l, ASCII_m,
    ASCII_n, ASCII_o, ASCII_q, ASCII_r, ASCII_s, ASCII_t, ASCII_v, ASCII_x, ASCII_y, ASCII_z,
    ASCII_0, ASCII_1, ASCII_2, ASCII_3, ASCII_4, ASCII_5, ASCII_6, ASCII_7, ASCII_8, ASCII_9,
    ASCII_A, ASCII_AMP, ASCII_APOS, ASCII_B, ASCII_C, ASCII_COLON, ASCII_D, ASCII_E, ASCII_EQUALS,
    ASCII_F, ASCII_GT, ASCII_I, ASCII_L, ASCII_LSQB, ASCII_LT, ASCII_M, ASCII_MINUS, ASCII_O,
    ASCII_PERIOD, ASCII_QUOT, ASCII_S, ASCII_SPACE, ASCII_T, ASCII_U, ASCII_UNDERSCORE, ASCII_X,
    ASCII_Z,
};
pub use crate::expat_external_h::XML_Size;
pub use crate::src::lib::xmltok::nametab_h::{namePages, namingBitmap, nmstrtPages};
pub use crate::src::lib::xmltok::xmltok_impl_c::{
    big2_attributeValueTok, big2_cdataSectionTok, big2_charRefNumber, big2_checkPiTarget,
    big2_contentTok, big2_entityValueTok, big2_getAtts, big2_ignoreSectionTok, big2_isPublicId,
    big2_nameLength, big2_nameMatchesAscii, big2_predefinedEntityName, big2_prologTok,
    big2_scanAtts, big2_scanCdataSection, big2_scanCharRef, big2_scanComment, big2_scanDecl,
    big2_scanEndTag, big2_scanHexCharRef, big2_scanLit, big2_scanLt, big2_scanPercent, big2_scanPi,
    big2_scanPoundName, big2_scanRef, big2_skipS, big2_updatePosition, little2_attributeValueTok,
    little2_cdataSectionTok, little2_charRefNumber, little2_checkPiTarget, little2_contentTok,
    little2_entityValueTok, little2_getAtts, little2_ignoreSectionTok, little2_isPublicId,
    little2_nameLength, little2_nameMatchesAscii, little2_predefinedEntityName, little2_prologTok,
    little2_scanAtts, little2_scanCdataSection, little2_scanCharRef, little2_scanComment,
    little2_scanDecl, little2_scanEndTag, little2_scanHexCharRef, little2_scanLit, little2_scanLt,
    little2_scanPercent, little2_scanPi, little2_scanPoundName, little2_scanRef, little2_skipS,
    little2_updatePosition, normal_attributeValueTok, normal_cdataSectionTok, normal_charRefNumber,
    normal_checkPiTarget, normal_contentTok, normal_entityValueTok, normal_getAtts,
    normal_ignoreSectionTok, normal_isPublicId, normal_nameLength, normal_nameMatchesAscii,
    normal_predefinedEntityName, normal_prologTok, normal_scanAtts, normal_scanCdataSection,
    normal_scanCharRef, normal_scanComment, normal_scanDecl, normal_scanEndTag,
    normal_scanHexCharRef, normal_scanLit, normal_scanLt, normal_scanPercent, normal_scanPi,
    normal_scanPoundName, normal_scanRef, normal_skipS, normal_updatePosition,
};
pub use crate::src::lib::xmltok::xmltok_ns_c::{
    encodings, encodingsNS, findEncoding, findEncodingNS, initScanContent, initScanContentNS,
    initScanProlog, initScanPrologNS, XmlGetUtf16InternalEncoding, XmlGetUtf16InternalEncodingNS,
    XmlGetUtf8InternalEncoding, XmlGetUtf8InternalEncodingNS, XmlInitEncoding, XmlInitEncodingNS,
    XmlParseXmlDecl, XmlParseXmlDeclNS,
};
pub use crate::stdbool_h::{false_0, true_0};
pub use crate::stddef_h::{ptrdiff_t, size_t, NULL};
pub use crate::xmltok_impl_c::{
    inName, inName_0, inName_1, inValue, inValue_0, inValue_1, other, other_0, other_1,
};
pub use crate::xmltok_impl_h::{
    C2RustUnnamed_2, BT_AMP, BT_APOS, BT_AST, BT_COLON_0, BT_COMMA, BT_CR, BT_DIGIT, BT_EQUALS,
    BT_EXCL, BT_GT, BT_HEX, BT_LEAD2, BT_LEAD3, BT_LEAD4, BT_LF, BT_LPAR, BT_LSQB, BT_LT,
    BT_MALFORM, BT_MINUS, BT_NAME, BT_NMSTRT, BT_NONASCII, BT_NONXML, BT_NUM, BT_OTHER, BT_PERCNT,
    BT_PLUS, BT_QUEST, BT_QUOT, BT_RPAR, BT_RSQB, BT_S, BT_SEMI, BT_SOL, BT_TRAIL, BT_VERBAR,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct normal_encoding {
    pub enc: ENCODING,
    pub type_0: [c_uchar; 256],
    pub isName2: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
    pub isName3: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
    pub isName4: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
    pub isNmstrt2: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
    pub isNmstrt3: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
    pub isNmstrt4: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
    pub isInvalid2: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
    pub isInvalid3: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
    pub isInvalid4: Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
}

pub const UTF8_cval2: C2RustUnnamed_7 = 192;

pub const UTF8_cval4: C2RustUnnamed_7 = 240;

pub const UTF8_cval3: C2RustUnnamed_7 = 224;

pub const UNKNOWN_ENC: C2RustUnnamed_8 = -1;
/* must match encodingNames up to here */

pub const NO_ENC: C2RustUnnamed_8 = 6;

pub const UTF_16LE_ENC: C2RustUnnamed_8 = 5;

pub const UTF_16BE_ENC: C2RustUnnamed_8 = 4;

pub const UTF_8_ENC: C2RustUnnamed_8 = 2;

pub const UTF_16_ENC: C2RustUnnamed_8 = 3;

pub const ISO_8859_1_ENC: C2RustUnnamed_8 = 0;

pub const min4: C2RustUnnamed_6 = 65536;

pub const min3: C2RustUnnamed_6 = 2048;
/* UTF8_cvalN is value of masked first byte of N byte sequence */

pub const UTF8_cval1: C2RustUnnamed_7 = 0;
/* minN is minimum legal resulting value for N byte sequence */

pub const min2: C2RustUnnamed_6 = 128;

pub type C2RustUnnamed_6 = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unknown_encoding {
    pub normal: normal_encoding,
    pub convert: CONVERTER,
    pub userData: *mut c_void,
    pub utf16: [c_ushort; 256],
    pub utf8: [[c_char; 4]; 256],
}
/* minimum bytes per character */
/* c is an ASCII character */

pub type C2RustUnnamed_7 = c_uint;
/* If this enumeration is changed, getEncodingIndex and encodings
must also be changed. */

pub type C2RustUnnamed_8 = c_int;

pub const US_ASCII_ENC: C2RustUnnamed_8 = 1;
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
/* memcpy */
/* ndef _WIN32 */
/* A 2 byte UTF-8 representation splits the characters 11 bits between
   the bottom 5 and 6 bits of the bytes.  We need 8 bits to index into
   pages, 3 bits to add to that index and 5 bits to generate the mask.
*/
/* A 3 byte UTF-8 representation splits the characters 16 bits between
   the bottom 4, 6 and 6 bits of the bytes.  We need 8 bits to index
   into pages, 3 bits to add to that index and 5 bits to generate the
   mask.
*/
/* Detection of invalid UTF-8 sequences is based on Table 3.1B
   of Unicode 3.2: http://www.unicode.org/unicode/reports/tr28/
   with the additional restriction of not allowing the Unicode
   code points 0xFFFF and 0xFFFE (sequences EF,BF,BF and EF,BF,BE).
   Implementation details:
     (A & 0x80) == 0     means A < 0x80
   and
     (A & 0xC0) == 0xC0  means A > 0xBF
*/

unsafe extern "C" fn isNever(mut _enc: *const ENCODING, mut _p: *const c_char) -> c_int {
    return 0i32;
}

unsafe extern "C" fn utf8_isName2(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (namingBitmap[(((namePages[(*(p as *const c_uchar).offset(0isize) as c_int
        >> 2i32
        & 7i32) as usize] as c_int)
        << 3i32)
        + ((*(p as *const c_uchar).offset(0isize) as c_int & 3i32)
            << 1i32)
        + (*(p as *const c_uchar).offset(1isize) as c_int >> 5i32 & 1i32))
        as usize]
        & (1u32)
            << (*(p as *const c_uchar).offset(1isize) as c_int & 0x1fi32))
        as c_int;
}

unsafe extern "C" fn utf8_isName3(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (namingBitmap[(((namePages[(((*(p as *const c_uchar).offset(0isize)
        as c_int
        & 0xfi32)
        << 4i32)
        + (*(p as *const c_uchar).offset(1isize) as c_int >> 2i32
            & 0xfi32)) as usize] as c_int)
        << 3i32)
        + ((*(p as *const c_uchar).offset(1isize) as c_int & 3i32)
            << 1i32)
        + (*(p as *const c_uchar).offset(2isize) as c_int >> 5i32 & 1i32))
        as usize]
        & (1u32)
            << (*(p as *const c_uchar).offset(2isize) as c_int & 0x1fi32))
        as c_int;
}

pub const utf8_isName4: unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int =
    isNever;

unsafe extern "C" fn utf8_isNmstrt2(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (namingBitmap[(((nmstrtPages[(*(p as *const c_uchar).offset(0isize)
        as c_int
        >> 2i32
        & 7i32) as usize] as c_int)
        << 3i32)
        + ((*(p as *const c_uchar).offset(0isize) as c_int & 3i32)
            << 1i32)
        + (*(p as *const c_uchar).offset(1isize) as c_int >> 5i32 & 1i32))
        as usize]
        & (1u32)
            << (*(p as *const c_uchar).offset(1isize) as c_int & 0x1fi32))
        as c_int;
}

unsafe extern "C" fn utf8_isNmstrt3(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (namingBitmap[(((nmstrtPages[(((*(p as *const c_uchar).offset(0isize)
        as c_int
        & 0xfi32)
        << 4i32)
        + (*(p as *const c_uchar).offset(1isize) as c_int >> 2i32
            & 0xfi32)) as usize] as c_int)
        << 3i32)
        + ((*(p as *const c_uchar).offset(1isize) as c_int & 3i32)
            << 1i32)
        + (*(p as *const c_uchar).offset(2isize) as c_int >> 5i32 & 1i32))
        as usize]
        & (1u32)
            << (*(p as *const c_uchar).offset(2isize) as c_int & 0x1fi32))
        as c_int;
}

pub const utf8_isNmstrt4: unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int =
    isNever;

unsafe extern "C" fn utf8_isInvalid2(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return ((*(p as *const c_uchar) as c_int) < 0xc2i32
        || *(p as *const c_uchar).offset(1isize) as c_int & 0x80i32
            == 0i32
        || *(p as *const c_uchar).offset(1isize) as c_int & 0xc0i32
            == 0xc0i32) as c_int;
}

unsafe extern "C" fn utf8_isInvalid3(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (*(p as *const c_uchar).offset(2isize) as c_int & 0x80i32
        == 0i32
        || (if *(p as *const c_uchar) as c_int == 0xefi32
            && *(p as *const c_uchar).offset(1isize) as c_int == 0xbfi32
        {
            (*(p as *const c_uchar).offset(2isize) as c_int > 0xbdi32) as c_int
        } else {
            (*(p as *const c_uchar).offset(2isize) as c_int & 0xc0i32
                == 0xc0i32) as c_int
        }) != 0
        || (if *(p as *const c_uchar) as c_int == 0xe0i32 {
            ((*(p as *const c_uchar).offset(1isize) as c_int) < 0xa0i32
                || *(p as *const c_uchar).offset(1isize) as c_int & 0xc0i32
                    == 0xc0i32) as c_int
        } else {
            (*(p as *const c_uchar).offset(1isize) as c_int & 0x80i32
                == 0i32
                || (if *(p as *const c_uchar) as c_int == 0xedi32 {
                    (*(p as *const c_uchar).offset(1isize) as c_int > 0x9fi32)
                        as c_int
                } else {
                    (*(p as *const c_uchar).offset(1isize) as c_int & 0xc0i32
                        == 0xc0i32) as c_int
                }) != 0) as c_int
        }) != 0) as c_int;
}

unsafe extern "C" fn utf8_isInvalid4(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (*(p as *const c_uchar).offset(3isize) as c_int & 0x80i32
        == 0i32
        || *(p as *const c_uchar).offset(3isize) as c_int & 0xc0i32
            == 0xc0i32
        || *(p as *const c_uchar).offset(2isize) as c_int & 0x80i32
            == 0i32
        || *(p as *const c_uchar).offset(2isize) as c_int & 0xc0i32
            == 0xc0i32
        || (if *(p as *const c_uchar) as c_int == 0xf0i32 {
            ((*(p as *const c_uchar).offset(1isize) as c_int) < 0x90i32
                || *(p as *const c_uchar).offset(1isize) as c_int & 0xc0i32
                    == 0xc0i32) as c_int
        } else {
            (*(p as *const c_uchar).offset(1isize) as c_int & 0x80i32
                == 0i32
                || (if *(p as *const c_uchar) as c_int == 0xf4i32 {
                    (*(p as *const c_uchar).offset(1isize) as c_int > 0x8fi32)
                        as c_int
                } else {
                    (*(p as *const c_uchar).offset(1isize) as c_int & 0xc0i32
                        == 0xc0i32) as c_int
                }) != 0) as c_int
        }) != 0) as c_int;
}
/* internal.h

   Internal definitions used by Expat.  This is not needed to compile
   client code.

   The following calling convention macros are defined for frequently
   called functions:

   FASTCALL    - Used for those internal functions that have a simple
                 body and a low number of arguments and local variables.

   PTRCALL     - Used for functions called though function pointers.

   PTRFASTCALL - Like PTRCALL, but for low number of arguments.

   inline      - Used for selected internal functions for which inlining
                 may improve performance on some platforms.

   Note: Use of these macros is based on judgement, not hard rules,
         and therefore subject to change.
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
/* Using __fastcall seems to have an unexpected negative effect under
   MS VC++, especially for function pointers, so we won't use it for
   now on that platform. It may be reconsidered for a future release
   if it can be made more effective.
   Likely reason: __fastcall on Windows is like stdcall, therefore
   the compiler cannot perform stack optimizations for call clusters.
*/
/* Make sure all of these are defined if they aren't already. */
/* __GNUC__ */
/* XML_MIN_SIZE */
#[no_mangle]

pub unsafe extern "C" fn _INTERNAL_trim_to_complete_utf8_characters(
    mut from: *const c_char,
    mut fromLimRef: *mut *const c_char,
) {
    let mut fromLim: *const c_char = *fromLimRef;
    let mut walked: size_t = 0u64;
    while fromLim > from {
        let prev: c_uchar = *fromLim.offset(-1isize) as c_uchar;
        if prev as c_uint & 0xf8u32 == 0xf0u32 {
            /* 4-byte character, lead by 0b11110xxx byte */
            if walked.wrapping_add(1u64) >= 4u64 {
                fromLim = fromLim.offset((4i32 - 1i32) as isize);
                break;
            } else {
                walked = 0u64
            }
        } else if prev as c_uint & 0xf0u32 == 0xe0u32 {
            /* 3-byte character, lead by 0b1110xxxx byte */
            if walked.wrapping_add(1u64) >= 3u64 {
                fromLim = fromLim.offset((3i32 - 1i32) as isize);
                break;
            } else {
                walked = 0u64
            }
        } else if prev as c_uint & 0xe0u32 == 0xc0u32 {
            /* 2-byte character, lead by 0b110xxxxx byte */
            if walked.wrapping_add(1u64) >= 2u64 {
                fromLim = fromLim.offset((2i32 - 1i32) as isize);
                break;
            } else {
                walked = 0u64
            }
        } else if prev as c_uint & 0x80u32 == 0u32 {
            break;
        }
        fromLim = fromLim.offset(-1);
        walked = walked.wrapping_add(1)
    }
    *fromLimRef = fromLim;
}

unsafe extern "C" fn utf8_toUtf8(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    let mut input_incomplete: bool = false_0 != 0;
    let mut output_exhausted: bool = false_0 != 0;
    /* Avoid copying partial characters (due to limited space). */
    let bytesAvailable: ptrdiff_t = fromLim.wrapping_offset_from(*fromP) as c_long;
    let bytesStorable: ptrdiff_t = toLim.wrapping_offset_from(*toP) as c_long;
    if bytesAvailable > bytesStorable {
        fromLim = (*fromP).offset(bytesStorable as isize);
        output_exhausted = true_0 != 0
    }
    /* Avoid copying partial characters (from incomplete input). */
    let fromLimBefore: *const c_char = fromLim;
    _INTERNAL_trim_to_complete_utf8_characters(*fromP, &mut fromLim);
    if fromLim < fromLimBefore {
        input_incomplete = true_0 != 0
    }
    let bytesToCopy: ptrdiff_t = fromLim.wrapping_offset_from(*fromP) as c_long;
    memcpy(
        *toP as *mut c_void,
        *fromP as *const c_void,
        bytesToCopy as c_ulong,
    );
    *fromP = (*fromP).offset(bytesToCopy as isize);
    *toP = (*toP).offset(bytesToCopy as isize);
    if output_exhausted {
        /* needs to go first */
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else if input_incomplete {
        return XML_CONVERT_INPUT_INCOMPLETE;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}

unsafe extern "C" fn utf8_toUtf16(
    mut enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_ushort,
    mut toLim: *const c_ushort,
) -> XML_Convert_Result {
    let mut current_block: u64;
    let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
    let mut to: *mut c_ushort = *toP;
    let mut from: *const c_char = *fromP;
    loop {
        if !(from < fromLim && to < toLim as *mut c_ushort) {
            current_block = 1608152415753874203;
            break;
        }
        match (*(enc as *mut normal_encoding)).type_0[*from as c_uchar as usize] as c_int {
            5 => {
                if (fromLim.wrapping_offset_from(from) as c_long) < 2i64 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    current_block = 10086016483950629671;
                    break;
                } else {
                    let fresh24 = to;
                    to = to.offset(1);
                    *fresh24 = ((*from.offset(0isize) as c_int & 0x1fi32)
                        << 6i32
                        | *from.offset(1isize) as c_int & 0x3fi32)
                        as c_ushort;
                    from = from.offset(2isize)
                }
            }
            6 => {
                if (fromLim.wrapping_offset_from(from) as c_long) < 3i64 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    current_block = 10086016483950629671;
                    break;
                } else {
                    let fresh25 = to;
                    to = to.offset(1);
                    *fresh25 = ((*from.offset(0isize) as c_int & 0xfi32)
                        << 12i32
                        | (*from.offset(1isize) as c_int & 0x3fi32)
                            << 6i32
                        | *from.offset(2isize) as c_int & 0x3fi32)
                        as c_ushort;
                    from = from.offset(3isize)
                }
            }
            7 => {
                let mut n: c_ulong = 0;
                if (toLim.wrapping_offset_from(to) as c_long) < 2i64 {
                    res = XML_CONVERT_OUTPUT_EXHAUSTED;
                    current_block = 10086016483950629671;
                    break;
                } else if (fromLim.wrapping_offset_from(from) as c_long) < 4i64 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    current_block = 10086016483950629671;
                    break;
                } else {
                    n = ((*from.offset(0isize) as c_int & 0x7i32) << 18i32
                        | (*from.offset(1isize) as c_int & 0x3fi32)
                            << 12i32
                        | (*from.offset(2isize) as c_int & 0x3fi32)
                            << 6i32
                        | *from.offset(3isize) as c_int & 0x3fi32)
                        as c_ulong;
                    n = n.wrapping_sub(0x10000u64);
                    *to.offset(0isize) =
                        (n >> 10i32 | 0xd800u64) as c_ushort;
                    *to.offset(1isize) =
                        (n & 0x3ffu64 | 0xdc00u64) as c_ushort;
                    to = to.offset(2isize);
                    from = from.offset(4isize)
                }
            }
            _ => {
                let fresh26 = from;
                from = from.offset(1);
                let fresh27 = to;
                to = to.offset(1);
                *fresh27 = *fresh26 as c_ushort
            }
        }
    }
    match current_block {
        1608152415753874203 => {
            if from < fromLim {
                res = XML_CONVERT_OUTPUT_EXHAUSTED
            }
        }
        _ => {}
    }
    *fromP = from;
    *toP = to;
    return res;
}

static mut utf8_encoding_ns: normal_encoding = {
    let mut init = normal_encoding {
        enc: {
            let mut init = encoding {
                scanners: [
                    Some(
                        normal_prologTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_contentTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_cdataSectionTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_ignoreSectionTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                ],
                literalScanners: [
                    Some(
                        normal_attributeValueTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_entityValueTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                ],
                nameMatchesAscii: Some(
                    normal_nameMatchesAscii
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *const c_char,
                        ) -> c_int,
                ),
                nameLength: Some(
                    normal_nameLength
                        as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                ),
                skipS: Some(
                    normal_skipS
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                        ) -> *const c_char,
                ),
                getAtts: Some(
                    normal_getAtts
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: c_int,
                            _: *mut ATTRIBUTE,
                        ) -> c_int,
                ),
                charRefNumber: Some(
                    normal_charRefNumber
                        as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                ),
                predefinedEntityName: Some(
                    normal_predefinedEntityName
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                        ) -> c_int,
                ),
                updatePosition: Some(
                    normal_updatePosition
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *mut POSITION,
                        ) -> (),
                ),
                isPublicId: Some(
                    normal_isPublicId
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *mut *const c_char,
                        ) -> c_int,
                ),
                utf8Convert: Some(
                    utf8_toUtf8
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *mut *const c_char,
                            _: *const c_char,
                            _: *mut *mut c_char,
                            _: *const c_char,
                        ) -> XML_Convert_Result,
                ),
                utf16Convert: Some(
                    utf8_toUtf16
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *mut *const c_char,
                            _: *const c_char,
                            _: *mut *mut c_ushort,
                            _: *const c_ushort,
                        ) -> XML_Convert_Result,
                ),
                minBytesPerChar: 1i32,
                isUtf8: 1i8,
                isUtf16: 0i8,
            };
            init
        },
        type_0: [
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_S as c_uchar,
            
            BT_LF as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_CR as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_S as c_uchar,
            
            BT_EXCL as c_uchar,
            
            BT_QUOT as c_uchar,
            
            BT_NUM as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_PERCNT as c_uchar,
            
            BT_AMP as c_uchar,
            
            BT_APOS as c_uchar,
            
            BT_LPAR as c_uchar,
            
            BT_RPAR as c_uchar,
            
            BT_AST as c_uchar,
            
            BT_PLUS as c_uchar,
            
            BT_COMMA as c_uchar,
            
            BT_MINUS as c_uchar,
            
            BT_NAME as c_uchar,
            
            BT_SOL as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_COLON_0 as c_uchar,
            
            BT_SEMI as c_uchar,
            
            BT_LT as c_uchar,
            
            BT_EQUALS as c_uchar,
            
            BT_GT as c_uchar,
            
            BT_QUEST as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_LSQB as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_RSQB as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_VERBAR as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_MALFORM as c_uchar,
            
            BT_MALFORM as c_uchar,
        ],
        isName2: Some(
            utf8_isName2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isName3: Some(
            utf8_isName3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isName4: Some(utf8_isName4),
        isNmstrt2: Some(
            utf8_isNmstrt2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isNmstrt3: Some(
            utf8_isNmstrt3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isNmstrt4: Some(utf8_isNmstrt4),
        isInvalid2: Some(
            utf8_isInvalid2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isInvalid3: Some(
            utf8_isInvalid3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isInvalid4: Some(
            utf8_isInvalid4 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
    };
    init
};

static mut utf8_encoding: normal_encoding = {
    let mut init = normal_encoding {
        enc: {
            let mut init = encoding {
                scanners: [
                    Some(
                        normal_prologTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_contentTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_cdataSectionTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_ignoreSectionTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                ],
                literalScanners: [
                    Some(
                        normal_attributeValueTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_entityValueTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                ],
                nameMatchesAscii: Some(
                    normal_nameMatchesAscii
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *const c_char,
                        ) -> c_int,
                ),
                nameLength: Some(
                    normal_nameLength
                        as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                ),
                skipS: Some(
                    normal_skipS
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                        ) -> *const c_char,
                ),
                getAtts: Some(
                    normal_getAtts
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: c_int,
                            _: *mut ATTRIBUTE,
                        ) -> c_int,
                ),
                charRefNumber: Some(
                    normal_charRefNumber
                        as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                ),
                predefinedEntityName: Some(
                    normal_predefinedEntityName
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                        ) -> c_int,
                ),
                updatePosition: Some(
                    normal_updatePosition
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *mut POSITION,
                        ) -> (),
                ),
                isPublicId: Some(
                    normal_isPublicId
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *mut *const c_char,
                        ) -> c_int,
                ),
                utf8Convert: Some(
                    utf8_toUtf8
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *mut *const c_char,
                            _: *const c_char,
                            _: *mut *mut c_char,
                            _: *const c_char,
                        ) -> XML_Convert_Result,
                ),
                utf16Convert: Some(
                    utf8_toUtf16
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *mut *const c_char,
                            _: *const c_char,
                            _: *mut *mut c_ushort,
                            _: *const c_ushort,
                        ) -> XML_Convert_Result,
                ),
                minBytesPerChar: 1i32,
                isUtf8: 1i8,
                isUtf16: 0i8,
            };
            init
        },
        type_0: [
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_S as c_uchar,
            
            BT_LF as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_CR as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_S as c_uchar,
            
            BT_EXCL as c_uchar,
            
            BT_QUOT as c_uchar,
            
            BT_NUM as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_PERCNT as c_uchar,
            
            BT_AMP as c_uchar,
            
            BT_APOS as c_uchar,
            
            BT_LPAR as c_uchar,
            
            BT_RPAR as c_uchar,
            
            BT_AST as c_uchar,
            
            BT_PLUS as c_uchar,
            
            BT_COMMA as c_uchar,
            
            BT_MINUS as c_uchar,
            
            BT_NAME as c_uchar,
            
            BT_SOL as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            BT_COLON as c_uchar,
            
            BT_SEMI as c_uchar,
            
            BT_LT as c_uchar,
            
            BT_EQUALS as c_uchar,
            
            BT_GT as c_uchar,
            
            BT_QUEST as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_LSQB as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_RSQB as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_VERBAR as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_MALFORM as c_uchar,
            
            BT_MALFORM as c_uchar,
        ],
        isName2: Some(
            utf8_isName2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isName3: Some(
            utf8_isName3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isName4: Some(utf8_isName4),
        isNmstrt2: Some(
            utf8_isNmstrt2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isNmstrt3: Some(
            utf8_isNmstrt3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isNmstrt4: Some(utf8_isNmstrt4),
        isInvalid2: Some(
            utf8_isInvalid2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isInvalid3: Some(
            utf8_isInvalid3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isInvalid4: Some(
            utf8_isInvalid4 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
    };
    init
};

pub const BT_COLON: c_int = BT_NMSTRT as c_int;

static mut internal_utf8_encoding_ns: normal_encoding = {
    let mut init = normal_encoding {
        enc: {
            let mut init = encoding {
                scanners: [
                    Some(
                        normal_prologTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_contentTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_cdataSectionTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_ignoreSectionTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                ],
                literalScanners: [
                    Some(
                        normal_attributeValueTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_entityValueTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                ],
                nameMatchesAscii: Some(
                    normal_nameMatchesAscii
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *const c_char,
                        ) -> c_int,
                ),
                nameLength: Some(
                    normal_nameLength
                        as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                ),
                skipS: Some(
                    normal_skipS
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                        ) -> *const c_char,
                ),
                getAtts: Some(
                    normal_getAtts
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: c_int,
                            _: *mut ATTRIBUTE,
                        ) -> c_int,
                ),
                charRefNumber: Some(
                    normal_charRefNumber
                        as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                ),
                predefinedEntityName: Some(
                    normal_predefinedEntityName
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                        ) -> c_int,
                ),
                updatePosition: Some(
                    normal_updatePosition
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *mut POSITION,
                        ) -> (),
                ),
                isPublicId: Some(
                    normal_isPublicId
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *mut *const c_char,
                        ) -> c_int,
                ),
                utf8Convert: Some(
                    utf8_toUtf8
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *mut *const c_char,
                            _: *const c_char,
                            _: *mut *mut c_char,
                            _: *const c_char,
                        ) -> XML_Convert_Result,
                ),
                utf16Convert: Some(
                    utf8_toUtf16
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *mut *const c_char,
                            _: *const c_char,
                            _: *mut *mut c_ushort,
                            _: *const c_ushort,
                        ) -> XML_Convert_Result,
                ),
                minBytesPerChar: 1i32,
                isUtf8: 1i8,
                isUtf16: 0i8,
            };
            init
        },
        type_0: [
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_S as c_uchar,
            
            BT_LF as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_S as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_S as c_uchar,
            
            BT_EXCL as c_uchar,
            
            BT_QUOT as c_uchar,
            
            BT_NUM as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_PERCNT as c_uchar,
            
            BT_AMP as c_uchar,
            
            BT_APOS as c_uchar,
            
            BT_LPAR as c_uchar,
            
            BT_RPAR as c_uchar,
            
            BT_AST as c_uchar,
            
            BT_PLUS as c_uchar,
            
            BT_COMMA as c_uchar,
            
            BT_MINUS as c_uchar,
            
            BT_NAME as c_uchar,
            
            BT_SOL as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_COLON_0 as c_uchar,
            
            BT_SEMI as c_uchar,
            
            BT_LT as c_uchar,
            
            BT_EQUALS as c_uchar,
            
            BT_GT as c_uchar,
            
            BT_QUEST as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_LSQB as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_RSQB as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_VERBAR as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_MALFORM as c_uchar,
            
            BT_MALFORM as c_uchar,
        ],
        isName2: Some(
            utf8_isName2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isName3: Some(
            utf8_isName3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isName4: Some(utf8_isName4),
        isNmstrt2: Some(
            utf8_isNmstrt2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isNmstrt3: Some(
            utf8_isNmstrt3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isNmstrt4: Some(utf8_isNmstrt4),
        isInvalid2: Some(
            utf8_isInvalid2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isInvalid3: Some(
            utf8_isInvalid3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isInvalid4: Some(
            utf8_isInvalid4 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
    };
    init
};

static mut internal_utf8_encoding: normal_encoding = {
    let mut init = normal_encoding {
        enc: {
            let mut init = encoding {
                scanners: [
                    Some(
                        normal_prologTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_contentTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_cdataSectionTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_ignoreSectionTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                ],
                literalScanners: [
                    Some(
                        normal_attributeValueTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    Some(
                        normal_entityValueTok
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                ],
                nameMatchesAscii: Some(
                    normal_nameMatchesAscii
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *const c_char,
                        ) -> c_int,
                ),
                nameLength: Some(
                    normal_nameLength
                        as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                ),
                skipS: Some(
                    normal_skipS
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                        ) -> *const c_char,
                ),
                getAtts: Some(
                    normal_getAtts
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: c_int,
                            _: *mut ATTRIBUTE,
                        ) -> c_int,
                ),
                charRefNumber: Some(
                    normal_charRefNumber
                        as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                ),
                predefinedEntityName: Some(
                    normal_predefinedEntityName
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                        ) -> c_int,
                ),
                updatePosition: Some(
                    normal_updatePosition
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *mut POSITION,
                        ) -> (),
                ),
                isPublicId: Some(
                    normal_isPublicId
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *const c_char,
                            _: *const c_char,
                            _: *mut *const c_char,
                        ) -> c_int,
                ),
                utf8Convert: Some(
                    utf8_toUtf8
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *mut *const c_char,
                            _: *const c_char,
                            _: *mut *mut c_char,
                            _: *const c_char,
                        ) -> XML_Convert_Result,
                ),
                utf16Convert: Some(
                    utf8_toUtf16
                        as unsafe extern "C" fn(
                            _: *const ENCODING,
                            _: *mut *const c_char,
                            _: *const c_char,
                            _: *mut *mut c_ushort,
                            _: *const c_ushort,
                        ) -> XML_Convert_Result,
                ),
                minBytesPerChar: 1i32,
                isUtf8: 1i8,
                isUtf16: 0i8,
            };
            init
        },
        type_0: [
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_S as c_uchar,
            
            BT_LF as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_S as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_S as c_uchar,
            
            BT_EXCL as c_uchar,
            
            BT_QUOT as c_uchar,
            
            BT_NUM as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_PERCNT as c_uchar,
            
            BT_AMP as c_uchar,
            
            BT_APOS as c_uchar,
            
            BT_LPAR as c_uchar,
            
            BT_RPAR as c_uchar,
            
            BT_AST as c_uchar,
            
            BT_PLUS as c_uchar,
            
            BT_COMMA as c_uchar,
            
            BT_MINUS as c_uchar,
            
            BT_NAME as c_uchar,
            
            BT_SOL as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            
            BT_DIGIT as c_uchar,
            BT_COLON_5 as c_uchar,
            
            BT_SEMI as c_uchar,
            
            BT_LT as c_uchar,
            
            BT_EQUALS as c_uchar,
            
            BT_GT as c_uchar,
            
            BT_QUEST as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_LSQB as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_RSQB as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_HEX as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_NMSTRT as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_VERBAR as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_OTHER as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_TRAIL as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD2 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD3 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_LEAD4 as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_NONXML as c_uchar,
            
            BT_MALFORM as c_uchar,
            
            BT_MALFORM as c_uchar,
        ],
        isName2: Some(
            utf8_isName2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isName3: Some(
            utf8_isName3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isName4: Some(utf8_isName4),
        isNmstrt2: Some(
            utf8_isNmstrt2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isNmstrt3: Some(
            utf8_isNmstrt3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isNmstrt4: Some(utf8_isNmstrt4),
        isInvalid2: Some(
            utf8_isInvalid2 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isInvalid3: Some(
            utf8_isInvalid3 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
        isInvalid4: Some(
            utf8_isInvalid4 as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        ),
    };
    init
};

pub const BT_COLON_5: c_int = BT_NMSTRT as c_int;

unsafe extern "C" fn latin1_toUtf8(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    loop {
        let mut c: c_uchar = 0;
        if *fromP == fromLim {
            return XML_CONVERT_COMPLETED;
        }
        c = **fromP as c_uchar;
        if c as c_int & 0x80i32 != 0 {
            if (toLim.wrapping_offset_from(*toP) as c_long) < 2i64 {
                return XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            let fresh28 = *toP;
            *toP = (*toP).offset(1);
            *fresh28 = (c as c_int >> 6i32 | UTF8_cval2 as c_int) as c_char;
            let fresh29 = *toP;
            *toP = (*toP).offset(1);
            *fresh29 = (c as c_int & 0x3fi32 | 0x80i32) as c_char;
            *fromP = (*fromP).offset(1)
        } else {
            if *toP == toLim as *mut c_char {
                return XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            let fresh30 = *fromP;
            *fromP = (*fromP).offset(1);
            let fresh31 = *toP;
            *toP = (*toP).offset(1);
            *fresh31 = *fresh30
        }
    }
}

unsafe extern "C" fn latin1_toUtf16(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_ushort,
    mut toLim: *const c_ushort,
) -> XML_Convert_Result {
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let fresh32 = *fromP;
        *fromP = (*fromP).offset(1);
        let fresh33 = *toP;
        *toP = (*toP).offset(1);
        *fresh33 = *fresh32 as c_uchar as c_ushort
    }
    if *toP == toLim as *mut c_ushort && *fromP < fromLim {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}

static mut latin1_encoding_ns: normal_encoding = unsafe {
    {
        let mut init = normal_encoding {
            enc: {
                let mut init = encoding {
                    scanners: [
                        Some(
                            normal_prologTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_contentTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_cdataSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_ignoreSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    literalScanners: [
                        Some(
                            normal_attributeValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_entityValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    nameMatchesAscii: Some(
                        normal_nameMatchesAscii
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    nameLength: Some(
                        normal_nameLength
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    skipS: Some(
                        normal_skipS
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                            ) -> *const c_char,
                    ),
                    getAtts: Some(
                        normal_getAtts
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: c_int,
                                _: *mut ATTRIBUTE,
                            ) -> c_int,
                    ),
                    charRefNumber: Some(
                        normal_charRefNumber
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    predefinedEntityName: Some(
                        normal_predefinedEntityName
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    updatePosition: Some(
                        normal_updatePosition
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut POSITION,
                            ) -> (),
                    ),
                    isPublicId: Some(
                        normal_isPublicId
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    utf8Convert: Some(
                        latin1_toUtf8
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_char,
                                _: *const c_char,
                            )
                                -> XML_Convert_Result,
                    ),
                    utf16Convert: Some(
                        latin1_toUtf16
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_ushort,
                                _: *const c_ushort,
                            )
                                -> XML_Convert_Result,
                    ),
                    minBytesPerChar: 1i32,
                    isUtf8: 0i8,
                    isUtf16: 0i8,
                };
                init
            },
            type_0: [
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_LF as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_CR as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_EXCL as c_uchar,
                
                BT_QUOT as c_uchar,
                
                BT_NUM as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_PERCNT as c_uchar,
                
                BT_AMP as c_uchar,
                
                BT_APOS as c_uchar,
                
                BT_LPAR as c_uchar,
                
                BT_RPAR as c_uchar,
                
                BT_AST as c_uchar,
                
                BT_PLUS as c_uchar,
                
                BT_COMMA as c_uchar,
                
                BT_MINUS as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_SOL as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_COLON_0 as c_uchar,
                
                BT_SEMI as c_uchar,
                
                BT_LT as c_uchar,
                
                BT_EQUALS as c_uchar,
                
                BT_GT as c_uchar,
                
                BT_QUEST as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_LSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_RSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_VERBAR as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
            ],
            isName2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
        };
        init
    }
};

static mut latin1_encoding: normal_encoding = unsafe {
    {
        let mut init = normal_encoding {
            enc: {
                let mut init = encoding {
                    scanners: [
                        Some(
                            normal_prologTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_contentTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_cdataSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_ignoreSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    literalScanners: [
                        Some(
                            normal_attributeValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_entityValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    nameMatchesAscii: Some(
                        normal_nameMatchesAscii
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    nameLength: Some(
                        normal_nameLength
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    skipS: Some(
                        normal_skipS
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                            ) -> *const c_char,
                    ),
                    getAtts: Some(
                        normal_getAtts
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: c_int,
                                _: *mut ATTRIBUTE,
                            ) -> c_int,
                    ),
                    charRefNumber: Some(
                        normal_charRefNumber
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    predefinedEntityName: Some(
                        normal_predefinedEntityName
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    updatePosition: Some(
                        normal_updatePosition
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut POSITION,
                            ) -> (),
                    ),
                    isPublicId: Some(
                        normal_isPublicId
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    utf8Convert: Some(
                        latin1_toUtf8
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_char,
                                _: *const c_char,
                            )
                                -> XML_Convert_Result,
                    ),
                    utf16Convert: Some(
                        latin1_toUtf16
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_ushort,
                                _: *const c_ushort,
                            )
                                -> XML_Convert_Result,
                    ),
                    minBytesPerChar: 1i32,
                    isUtf8: 0i8,
                    isUtf16: 0i8,
                };
                init
            },
            type_0: [
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_LF as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_CR as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_EXCL as c_uchar,
                
                BT_QUOT as c_uchar,
                
                BT_NUM as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_PERCNT as c_uchar,
                
                BT_AMP as c_uchar,
                
                BT_APOS as c_uchar,
                
                BT_LPAR as c_uchar,
                
                BT_RPAR as c_uchar,
                
                BT_AST as c_uchar,
                
                BT_PLUS as c_uchar,
                
                BT_COMMA as c_uchar,
                
                BT_MINUS as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_SOL as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                BT_COLON_1 as c_uchar,
                
                BT_SEMI as c_uchar,
                
                BT_LT as c_uchar,
                
                BT_EQUALS as c_uchar,
                
                BT_GT as c_uchar,
                
                BT_QUEST as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_LSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_RSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_VERBAR as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
            ],
            isName2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
        };
        init
    }
};

pub const BT_COLON_1: c_int = BT_NMSTRT as c_int;

unsafe extern "C" fn ascii_toUtf8(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    while *fromP < fromLim && *toP < toLim as *mut c_char {
        let fresh34 = *fromP;
        *fromP = (*fromP).offset(1);
        let fresh35 = *toP;
        *toP = (*toP).offset(1);
        *fresh35 = *fresh34
    }
    if *toP == toLim as *mut c_char && *fromP < fromLim {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}

static mut ascii_encoding_ns: normal_encoding = unsafe {
    {
        let mut init = normal_encoding {
            enc: {
                let mut init = encoding {
                    scanners: [
                        Some(
                            normal_prologTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_contentTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_cdataSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_ignoreSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    literalScanners: [
                        Some(
                            normal_attributeValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_entityValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    nameMatchesAscii: Some(
                        normal_nameMatchesAscii
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    nameLength: Some(
                        normal_nameLength
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    skipS: Some(
                        normal_skipS
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                            ) -> *const c_char,
                    ),
                    getAtts: Some(
                        normal_getAtts
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: c_int,
                                _: *mut ATTRIBUTE,
                            ) -> c_int,
                    ),
                    charRefNumber: Some(
                        normal_charRefNumber
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    predefinedEntityName: Some(
                        normal_predefinedEntityName
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    updatePosition: Some(
                        normal_updatePosition
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut POSITION,
                            ) -> (),
                    ),
                    isPublicId: Some(
                        normal_isPublicId
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    utf8Convert: Some(
                        ascii_toUtf8
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_char,
                                _: *const c_char,
                            )
                                -> XML_Convert_Result,
                    ),
                    utf16Convert: Some(
                        latin1_toUtf16
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_ushort,
                                _: *const c_ushort,
                            )
                                -> XML_Convert_Result,
                    ),
                    minBytesPerChar: 1i32,
                    isUtf8: 1i8,
                    isUtf16: 0i8,
                };
                init
            },
            type_0: [
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_LF as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_CR as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_EXCL as c_uchar,
                
                BT_QUOT as c_uchar,
                
                BT_NUM as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_PERCNT as c_uchar,
                
                BT_AMP as c_uchar,
                
                BT_APOS as c_uchar,
                
                BT_LPAR as c_uchar,
                
                BT_RPAR as c_uchar,
                
                BT_AST as c_uchar,
                
                BT_PLUS as c_uchar,
                
                BT_COMMA as c_uchar,
                
                BT_MINUS as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_SOL as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_COLON_0 as c_uchar,
                
                BT_SEMI as c_uchar,
                
                BT_LT as c_uchar,
                
                BT_EQUALS as c_uchar,
                
                BT_GT as c_uchar,
                
                BT_QUEST as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_LSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_RSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_VERBAR as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            isName2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
        };
        init
    }
};

static mut ascii_encoding: normal_encoding = unsafe {
    {
        let mut init = normal_encoding {
            enc: {
                let mut init = encoding {
                    scanners: [
                        Some(
                            normal_prologTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_contentTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_cdataSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_ignoreSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    literalScanners: [
                        Some(
                            normal_attributeValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            normal_entityValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    nameMatchesAscii: Some(
                        normal_nameMatchesAscii
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    nameLength: Some(
                        normal_nameLength
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    skipS: Some(
                        normal_skipS
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                            ) -> *const c_char,
                    ),
                    getAtts: Some(
                        normal_getAtts
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: c_int,
                                _: *mut ATTRIBUTE,
                            ) -> c_int,
                    ),
                    charRefNumber: Some(
                        normal_charRefNumber
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    predefinedEntityName: Some(
                        normal_predefinedEntityName
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    updatePosition: Some(
                        normal_updatePosition
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut POSITION,
                            ) -> (),
                    ),
                    isPublicId: Some(
                        normal_isPublicId
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    utf8Convert: Some(
                        ascii_toUtf8
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_char,
                                _: *const c_char,
                            )
                                -> XML_Convert_Result,
                    ),
                    utf16Convert: Some(
                        latin1_toUtf16
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_ushort,
                                _: *const c_ushort,
                            )
                                -> XML_Convert_Result,
                    ),
                    minBytesPerChar: 1i32,
                    isUtf8: 1i8,
                    isUtf16: 0i8,
                };
                init
            },
            type_0: [
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_LF as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_CR as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_EXCL as c_uchar,
                
                BT_QUOT as c_uchar,
                
                BT_NUM as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_PERCNT as c_uchar,
                
                BT_AMP as c_uchar,
                
                BT_APOS as c_uchar,
                
                BT_LPAR as c_uchar,
                
                BT_RPAR as c_uchar,
                
                BT_AST as c_uchar,
                
                BT_PLUS as c_uchar,
                
                BT_COMMA as c_uchar,
                
                BT_MINUS as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_SOL as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                BT_COLON_4 as c_uchar,
                
                BT_SEMI as c_uchar,
                
                BT_LT as c_uchar,
                
                BT_EQUALS as c_uchar,
                
                BT_GT as c_uchar,
                
                BT_QUEST as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_LSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_RSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_VERBAR as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            isName2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
        };
        init
    }
};

pub const BT_COLON_4: c_int = BT_NMSTRT as c_int;

unsafe extern "C" fn unicode_byte_type(mut hi: c_char, mut lo: c_char) -> c_int {
    match hi as c_uchar as c_int {
        216 | 217 | 218 | 219 => {
            /* 0xD800–0xDBFF first 16-bit code unit or high surrogate (W1) */
            return BT_LEAD4 as c_int;
        }
        220 | 221 | 222 | 223 => {
            /* 0xDC00–0xDFFF second 16-bit code unit or low surrogate (W2) */
            return BT_TRAIL as c_int;
        }
        255 => {
            match lo as c_uchar as c_int {
                255 | 254 => {
                    /* noncharacter-FFFF */
                    /* noncharacter-FFFE */
                    return BT_NONXML as c_int;
                }
                _ => {}
            }
        }
        _ => {}
    }
    return BT_NONASCII as c_int;
}
/* shrink to even */
/* fall through */
/* 16 bits divided 4, 6, 6 amongst 3 bytes */
/* shrink to even */
/* Avoid copying first half only of surrogate */

unsafe extern "C" fn little2_toUtf8(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    let mut from: *const c_char = *fromP;
    fromLim = from.offset(
        ((fromLim.wrapping_offset_from(from) as c_long >> 1i32) << 1i32) as isize,
    );
    while from < fromLim {
        let mut plane: c_int = 0;
        let mut lo2: c_uchar = 0;
        let mut lo: c_uchar = *from.offset(0isize) as c_uchar;
        let mut hi: c_uchar = *from.offset(1isize) as c_uchar;
        let mut current_block_34: u64;
        match hi as c_int {
            0 => {
                if (lo as c_int) < 0x80i32 {
                    if *toP == toLim as *mut c_char {
                        *fromP = from;
                        return XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    let fresh36 = *toP;
                    *toP = (*toP).offset(1);
                    *fresh36 = lo as c_char;
                    current_block_34 = 18435049525520518667;
                } else {
                    current_block_34 = 11412679543673842523;
                }
            }
            1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                current_block_34 = 11412679543673842523;
            }
            216 | 217 | 218 | 219 => {
                if (toLim.wrapping_offset_from(*toP) as c_long) < 4i64 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                if (fromLim.wrapping_offset_from(from) as c_long) < 4i64 {
                    *fromP = from;
                    return XML_CONVERT_INPUT_INCOMPLETE;
                }
                plane = ((hi as c_int & 0x3i32) << 2i32
                    | lo as c_int >> 6i32 & 0x3i32)
                    + 1i32;
                let fresh42 = *toP;
                *toP = (*toP).offset(1);
                *fresh42 = (plane >> 2i32 | UTF8_cval4 as c_int) as c_char;
                let fresh43 = *toP;
                *toP = (*toP).offset(1);
                *fresh43 = (lo as c_int >> 2i32 & 0xfi32
                    | (plane & 0x3i32) << 4i32
                    | 0x80i32) as c_char;
                from = from.offset(2isize);
                lo2 = *from.offset(0isize) as c_uchar;
                let fresh44 = *toP;
                *toP = (*toP).offset(1);
                *fresh44 = ((lo as c_int & 0x3i32) << 4i32
                    | (*from.offset(1isize) as c_uchar as c_int & 0x3i32)
                        << 2i32
                    | lo2 as c_int >> 6i32
                    | 0x80i32) as c_char;
                let fresh45 = *toP;
                *toP = (*toP).offset(1);
                *fresh45 = (lo2 as c_int & 0x3fi32 | 0x80i32) as c_char;
                current_block_34 = 18435049525520518667;
            }
            _ => {
                if (toLim.wrapping_offset_from(*toP) as c_long) < 3i64 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let fresh39 = *toP;
                *toP = (*toP).offset(1);
                *fresh39 = (hi as c_int >> 4i32 | UTF8_cval3 as c_int) as c_char;
                let fresh40 = *toP;
                *toP = (*toP).offset(1);
                *fresh40 = ((hi as c_int & 0xfi32) << 2i32
                    | lo as c_int >> 6i32
                    | 0x80i32) as c_char;
                let fresh41 = *toP;
                *toP = (*toP).offset(1);
                *fresh41 = (lo as c_int & 0x3fi32 | 0x80i32) as c_char;
                current_block_34 = 18435049525520518667;
            }
        }
        match current_block_34 {
            11412679543673842523 => {
                if (toLim.wrapping_offset_from(*toP) as c_long) < 2i64 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let fresh37 = *toP;
                *toP = (*toP).offset(1);
                *fresh37 = (lo as c_int >> 6i32
                    | (hi as c_int) << 2i32
                    | UTF8_cval2 as c_int) as c_char;
                let fresh38 = *toP;
                *toP = (*toP).offset(1);
                *fresh38 = (lo as c_int & 0x3fi32 | 0x80i32) as c_char
            }
            _ => {}
        }
        from = from.offset(2isize)
    }
    *fromP = from;
    if from < fromLim {
        return XML_CONVERT_INPUT_INCOMPLETE;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}

unsafe extern "C" fn little2_toUtf16(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_ushort,
    mut toLim: *const c_ushort,
) -> XML_Convert_Result {
    let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
    fromLim = (*fromP).offset(
        ((fromLim.wrapping_offset_from(*fromP) as c_long >> 1i32) << 1i32) as isize,
    );
    if fromLim.wrapping_offset_from(*fromP) as c_long
        > (toLim.wrapping_offset_from(*toP) as c_long) << 1i32
        && *fromLim
            .offset(-(2isize))
            .offset(1isize) as c_uchar as c_int
            & 0xf8i32
            == 0xd8i32
    {
        fromLim = fromLim.offset(-(2isize));
        res = XML_CONVERT_INPUT_INCOMPLETE
    }
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let fresh46 = *toP;
        *toP = (*toP).offset(1);
        *fresh46 = ((*(*fromP).offset(1isize) as c_uchar as c_int) << 8i32
            | *(*fromP).offset(0isize) as c_uchar as c_int)
            as c_ushort;
        *fromP = (*fromP).offset(2isize)
    }
    if *toP == toLim as *mut c_ushort && *fromP < fromLim {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return res;
    };
}

unsafe extern "C" fn big2_toUtf8(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    let mut from: *const c_char = *fromP;
    fromLim = from.offset(
        ((fromLim.wrapping_offset_from(from) as c_long >> 1i32) << 1i32) as isize,
    );
    while from < fromLim {
        let mut plane: c_int = 0;
        let mut lo2: c_uchar = 0;
        let mut lo: c_uchar = *from.offset(1isize) as c_uchar;
        let mut hi: c_uchar = *from.offset(0isize) as c_uchar;
        let mut current_block_34: u64;
        match hi as c_int {
            0 => {
                if (lo as c_int) < 0x80i32 {
                    if *toP == toLim as *mut c_char {
                        *fromP = from;
                        return XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    let fresh47 = *toP;
                    *toP = (*toP).offset(1);
                    *fresh47 = lo as c_char;
                    current_block_34 = 18435049525520518667;
                } else {
                    current_block_34 = 6790550795307076813;
                }
            }
            1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                current_block_34 = 6790550795307076813;
            }
            216 | 217 | 218 | 219 => {
                if (toLim.wrapping_offset_from(*toP) as c_long) < 4i64 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                if (fromLim.wrapping_offset_from(from) as c_long) < 4i64 {
                    *fromP = from;
                    return XML_CONVERT_INPUT_INCOMPLETE;
                }
                plane = ((hi as c_int & 0x3i32) << 2i32
                    | lo as c_int >> 6i32 & 0x3i32)
                    + 1i32;
                let fresh53 = *toP;
                *toP = (*toP).offset(1);
                *fresh53 = (plane >> 2i32 | UTF8_cval4 as c_int) as c_char;
                let fresh54 = *toP;
                *toP = (*toP).offset(1);
                *fresh54 = (lo as c_int >> 2i32 & 0xfi32
                    | (plane & 0x3i32) << 4i32
                    | 0x80i32) as c_char;
                from = from.offset(2isize);
                lo2 = *from.offset(1isize) as c_uchar;
                let fresh55 = *toP;
                *toP = (*toP).offset(1);
                *fresh55 = ((lo as c_int & 0x3i32) << 4i32
                    | (*from.offset(0isize) as c_uchar as c_int & 0x3i32)
                        << 2i32
                    | lo2 as c_int >> 6i32
                    | 0x80i32) as c_char;
                let fresh56 = *toP;
                *toP = (*toP).offset(1);
                *fresh56 = (lo2 as c_int & 0x3fi32 | 0x80i32) as c_char;
                current_block_34 = 18435049525520518667;
            }
            _ => {
                if (toLim.wrapping_offset_from(*toP) as c_long) < 3i64 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let fresh50 = *toP;
                *toP = (*toP).offset(1);
                *fresh50 = (hi as c_int >> 4i32 | UTF8_cval3 as c_int) as c_char;
                let fresh51 = *toP;
                *toP = (*toP).offset(1);
                *fresh51 = ((hi as c_int & 0xfi32) << 2i32
                    | lo as c_int >> 6i32
                    | 0x80i32) as c_char;
                let fresh52 = *toP;
                *toP = (*toP).offset(1);
                *fresh52 = (lo as c_int & 0x3fi32 | 0x80i32) as c_char;
                current_block_34 = 18435049525520518667;
            }
        }
        match current_block_34 {
            6790550795307076813 => {
                if (toLim.wrapping_offset_from(*toP) as c_long) < 2i64 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let fresh48 = *toP;
                *toP = (*toP).offset(1);
                *fresh48 = (lo as c_int >> 6i32
                    | (hi as c_int) << 2i32
                    | UTF8_cval2 as c_int) as c_char;
                let fresh49 = *toP;
                *toP = (*toP).offset(1);
                *fresh49 = (lo as c_int & 0x3fi32 | 0x80i32) as c_char
            }
            _ => {}
        }
        from = from.offset(2isize)
    }
    *fromP = from;
    if from < fromLim {
        return XML_CONVERT_INPUT_INCOMPLETE;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}

unsafe extern "C" fn big2_toUtf16(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_ushort,
    mut toLim: *const c_ushort,
) -> XML_Convert_Result {
    let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
    fromLim = (*fromP).offset(
        ((fromLim.wrapping_offset_from(*fromP) as c_long >> 1i32) << 1i32) as isize,
    );
    if fromLim.wrapping_offset_from(*fromP) as c_long
        > (toLim.wrapping_offset_from(*toP) as c_long) << 1i32
        && *fromLim
            .offset(-(2isize))
            .offset(0isize) as c_uchar as c_int
            & 0xf8i32
            == 0xd8i32
    {
        fromLim = fromLim.offset(-(2isize));
        res = XML_CONVERT_INPUT_INCOMPLETE
    }
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let fresh57 = *toP;
        *toP = (*toP).offset(1);
        *fresh57 = ((*(*fromP).offset(0isize) as c_uchar as c_int) << 8i32
            | *(*fromP).offset(1isize) as c_uchar as c_int)
            as c_ushort;
        *fromP = (*fromP).offset(2isize)
    }
    if *toP == toLim as *mut c_ushort && *fromP < fromLim {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return res;
    };
}
/* not XML_MIN_SIZE */
/* CHAR_MATCHES is guaranteed to have MINBPC bytes available. */
/* not XML_MIN_SIZE */

static mut little2_encoding_ns: normal_encoding = unsafe {
    {
        let mut init = normal_encoding {
            enc: {
                let mut init = encoding {
                    scanners: [
                        Some(
                            little2_prologTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_contentTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_cdataSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_ignoreSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    literalScanners: [
                        Some(
                            little2_attributeValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_entityValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    nameMatchesAscii: Some(
                        little2_nameMatchesAscii
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    nameLength: Some(
                        little2_nameLength
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    skipS: Some(
                        little2_skipS
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                            ) -> *const c_char,
                    ),
                    getAtts: Some(
                        little2_getAtts
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: c_int,
                                _: *mut ATTRIBUTE,
                            ) -> c_int,
                    ),
                    charRefNumber: Some(
                        little2_charRefNumber
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    predefinedEntityName: Some(
                        little2_predefinedEntityName
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    updatePosition: Some(
                        little2_updatePosition
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut POSITION,
                            ) -> (),
                    ),
                    isPublicId: Some(
                        little2_isPublicId
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    utf8Convert: Some(
                        little2_toUtf8
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_char,
                                _: *const c_char,
                            )
                                -> XML_Convert_Result,
                    ),
                    utf16Convert: Some(
                        little2_toUtf16
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_ushort,
                                _: *const c_ushort,
                            )
                                -> XML_Convert_Result,
                    ),
                    minBytesPerChar: 2i32,
                    isUtf8: 0i8,
                    isUtf16: 1i8,
                };
                init
            },
            type_0: [
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_LF as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_CR as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_EXCL as c_uchar,
                
                BT_QUOT as c_uchar,
                
                BT_NUM as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_PERCNT as c_uchar,
                
                BT_AMP as c_uchar,
                
                BT_APOS as c_uchar,
                
                BT_LPAR as c_uchar,
                
                BT_RPAR as c_uchar,
                
                BT_AST as c_uchar,
                
                BT_PLUS as c_uchar,
                
                BT_COMMA as c_uchar,
                
                BT_MINUS as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_SOL as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_COLON_0 as c_uchar,
                
                BT_SEMI as c_uchar,
                
                BT_LT as c_uchar,
                
                BT_EQUALS as c_uchar,
                
                BT_GT as c_uchar,
                
                BT_QUEST as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_LSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_RSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_VERBAR as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
            ],
            isName2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
        };
        init
    }
};

static mut little2_encoding: normal_encoding = unsafe {
    {
        let mut init = normal_encoding {
            enc: {
                let mut init = encoding {
                    scanners: [
                        Some(
                            little2_prologTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_contentTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_cdataSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_ignoreSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    literalScanners: [
                        Some(
                            little2_attributeValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_entityValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    nameMatchesAscii: Some(
                        little2_nameMatchesAscii
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    nameLength: Some(
                        little2_nameLength
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    skipS: Some(
                        little2_skipS
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                            ) -> *const c_char,
                    ),
                    getAtts: Some(
                        little2_getAtts
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: c_int,
                                _: *mut ATTRIBUTE,
                            ) -> c_int,
                    ),
                    charRefNumber: Some(
                        little2_charRefNumber
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    predefinedEntityName: Some(
                        little2_predefinedEntityName
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    updatePosition: Some(
                        little2_updatePosition
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut POSITION,
                            ) -> (),
                    ),
                    isPublicId: Some(
                        little2_isPublicId
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    utf8Convert: Some(
                        little2_toUtf8
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_char,
                                _: *const c_char,
                            )
                                -> XML_Convert_Result,
                    ),
                    utf16Convert: Some(
                        little2_toUtf16
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_ushort,
                                _: *const c_ushort,
                            )
                                -> XML_Convert_Result,
                    ),
                    minBytesPerChar: 2i32,
                    isUtf8: 0i8,
                    isUtf16: 1i8,
                };
                init
            },
            type_0: [
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_LF as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_CR as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_EXCL as c_uchar,
                
                BT_QUOT as c_uchar,
                
                BT_NUM as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_PERCNT as c_uchar,
                
                BT_AMP as c_uchar,
                
                BT_APOS as c_uchar,
                
                BT_LPAR as c_uchar,
                
                BT_RPAR as c_uchar,
                
                BT_AST as c_uchar,
                
                BT_PLUS as c_uchar,
                
                BT_COMMA as c_uchar,
                
                BT_MINUS as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_SOL as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                BT_COLON_2 as c_uchar,
                
                BT_SEMI as c_uchar,
                
                BT_LT as c_uchar,
                
                BT_EQUALS as c_uchar,
                
                BT_GT as c_uchar,
                
                BT_QUEST as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_LSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_RSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_VERBAR as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
            ],
            isName2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
        };
        init
    }
};

pub const BT_COLON_2: c_int = BT_NMSTRT as c_int;

static mut internal_little2_encoding_ns: normal_encoding = unsafe {
    {
        let mut init = normal_encoding {
            enc: {
                let mut init = encoding {
                    scanners: [
                        Some(
                            little2_prologTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_contentTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_cdataSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_ignoreSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    literalScanners: [
                        Some(
                            little2_attributeValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_entityValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    nameMatchesAscii: Some(
                        little2_nameMatchesAscii
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    nameLength: Some(
                        little2_nameLength
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    skipS: Some(
                        little2_skipS
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                            ) -> *const c_char,
                    ),
                    getAtts: Some(
                        little2_getAtts
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: c_int,
                                _: *mut ATTRIBUTE,
                            ) -> c_int,
                    ),
                    charRefNumber: Some(
                        little2_charRefNumber
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    predefinedEntityName: Some(
                        little2_predefinedEntityName
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    updatePosition: Some(
                        little2_updatePosition
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut POSITION,
                            ) -> (),
                    ),
                    isPublicId: Some(
                        little2_isPublicId
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    utf8Convert: Some(
                        little2_toUtf8
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_char,
                                _: *const c_char,
                            )
                                -> XML_Convert_Result,
                    ),
                    utf16Convert: Some(
                        little2_toUtf16
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_ushort,
                                _: *const c_ushort,
                            )
                                -> XML_Convert_Result,
                    ),
                    minBytesPerChar: 2i32,
                    isUtf8: 0i8,
                    isUtf16: 1i8,
                };
                init
            },
            type_0: [
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_LF as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_EXCL as c_uchar,
                
                BT_QUOT as c_uchar,
                
                BT_NUM as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_PERCNT as c_uchar,
                
                BT_AMP as c_uchar,
                
                BT_APOS as c_uchar,
                
                BT_LPAR as c_uchar,
                
                BT_RPAR as c_uchar,
                
                BT_AST as c_uchar,
                
                BT_PLUS as c_uchar,
                
                BT_COMMA as c_uchar,
                
                BT_MINUS as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_SOL as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_COLON_0 as c_uchar,
                
                BT_SEMI as c_uchar,
                
                BT_LT as c_uchar,
                
                BT_EQUALS as c_uchar,
                
                BT_GT as c_uchar,
                
                BT_QUEST as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_LSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_RSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_VERBAR as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
            ],
            isName2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
        };
        init
    }
};

static mut internal_little2_encoding: normal_encoding = unsafe {
    {
        let mut init = normal_encoding {
            enc: {
                let mut init = encoding {
                    scanners: [
                        Some(
                            little2_prologTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_contentTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_cdataSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_ignoreSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    literalScanners: [
                        Some(
                            little2_attributeValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            little2_entityValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    nameMatchesAscii: Some(
                        little2_nameMatchesAscii
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    nameLength: Some(
                        little2_nameLength
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    skipS: Some(
                        little2_skipS
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                            ) -> *const c_char,
                    ),
                    getAtts: Some(
                        little2_getAtts
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: c_int,
                                _: *mut ATTRIBUTE,
                            ) -> c_int,
                    ),
                    charRefNumber: Some(
                        little2_charRefNumber
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    predefinedEntityName: Some(
                        little2_predefinedEntityName
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    updatePosition: Some(
                        little2_updatePosition
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut POSITION,
                            ) -> (),
                    ),
                    isPublicId: Some(
                        little2_isPublicId
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    utf8Convert: Some(
                        little2_toUtf8
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_char,
                                _: *const c_char,
                            )
                                -> XML_Convert_Result,
                    ),
                    utf16Convert: Some(
                        little2_toUtf16
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_ushort,
                                _: *const c_ushort,
                            )
                                -> XML_Convert_Result,
                    ),
                    minBytesPerChar: 2i32,
                    isUtf8: 0i8,
                    isUtf16: 1i8,
                };
                init
            },
            type_0: [
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_LF as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_EXCL as c_uchar,
                
                BT_QUOT as c_uchar,
                
                BT_NUM as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_PERCNT as c_uchar,
                
                BT_AMP as c_uchar,
                
                BT_APOS as c_uchar,
                
                BT_LPAR as c_uchar,
                
                BT_RPAR as c_uchar,
                
                BT_AST as c_uchar,
                
                BT_PLUS as c_uchar,
                
                BT_COMMA as c_uchar,
                
                BT_MINUS as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_SOL as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                BT_COLON_6 as c_uchar,
                
                BT_SEMI as c_uchar,
                
                BT_LT as c_uchar,
                
                BT_EQUALS as c_uchar,
                
                BT_GT as c_uchar,
                
                BT_QUEST as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_LSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_RSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_VERBAR as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
            ],
            isName2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
        };
        init
    }
};

pub const BT_COLON_6: c_int = BT_NMSTRT as c_int;
/* not XML_MIN_SIZE */
/* CHAR_MATCHES is guaranteed to have MINBPC bytes available. */
/* not XML_MIN_SIZE */

static mut big2_encoding_ns: normal_encoding = unsafe {
    {
        let mut init = normal_encoding {
            enc: {
                let mut init = encoding {
                    scanners: [
                        Some(
                            big2_prologTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            big2_contentTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            big2_cdataSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            big2_ignoreSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    literalScanners: [
                        Some(
                            big2_attributeValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            big2_entityValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    nameMatchesAscii: Some(
                        big2_nameMatchesAscii
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    nameLength: Some(
                        big2_nameLength
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    skipS: Some(
                        big2_skipS
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                            ) -> *const c_char,
                    ),
                    getAtts: Some(
                        big2_getAtts
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: c_int,
                                _: *mut ATTRIBUTE,
                            ) -> c_int,
                    ),
                    charRefNumber: Some(
                        big2_charRefNumber
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    predefinedEntityName: Some(
                        big2_predefinedEntityName
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    updatePosition: Some(
                        big2_updatePosition
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut POSITION,
                            ) -> (),
                    ),
                    isPublicId: Some(
                        big2_isPublicId
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    utf8Convert: Some(
                        big2_toUtf8
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_char,
                                _: *const c_char,
                            )
                                -> XML_Convert_Result,
                    ),
                    utf16Convert: Some(
                        big2_toUtf16
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_ushort,
                                _: *const c_ushort,
                            )
                                -> XML_Convert_Result,
                    ),
                    minBytesPerChar: 2i32,
                    isUtf8: 0i8,
                    isUtf16: 0i8,
                }; /* LCOV_EXCL_LINE */
                init
            },
            type_0: [
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_LF as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_CR as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_EXCL as c_uchar,
                
                BT_QUOT as c_uchar,
                
                BT_NUM as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_PERCNT as c_uchar,
                
                BT_AMP as c_uchar,
                
                BT_APOS as c_uchar,
                
                BT_LPAR as c_uchar,
                
                BT_RPAR as c_uchar,
                
                BT_AST as c_uchar,
                
                BT_PLUS as c_uchar,
                
                BT_COMMA as c_uchar,
                
                BT_MINUS as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_SOL as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_COLON_0 as c_uchar,
                
                BT_SEMI as c_uchar,
                
                BT_LT as c_uchar,
                
                BT_EQUALS as c_uchar,
                
                BT_GT as c_uchar,
                
                BT_QUEST as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_LSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_RSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_VERBAR as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
            ],
            isName2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
        };
        init
    }
};

static mut big2_encoding: normal_encoding = unsafe {
    {
        let mut init = normal_encoding {
            enc: {
                let mut init = encoding {
                    scanners: [
                        Some(
                            big2_prologTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            big2_contentTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            big2_cdataSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            big2_ignoreSectionTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    literalScanners: [
                        Some(
                            big2_attributeValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                        Some(
                            big2_entityValueTok
                                as unsafe extern "C" fn(
                                    _: *const ENCODING,
                                    _: *const c_char,
                                    _: *const c_char,
                                    _: *mut *const c_char,
                                ) -> c_int,
                        ),
                    ],
                    nameMatchesAscii: Some(
                        big2_nameMatchesAscii
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    nameLength: Some(
                        big2_nameLength
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    skipS: Some(
                        big2_skipS
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                            ) -> *const c_char,
                    ),
                    getAtts: Some(
                        big2_getAtts
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: c_int,
                                _: *mut ATTRIBUTE,
                            ) -> c_int,
                    ),
                    charRefNumber: Some(
                        big2_charRefNumber
                            as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
                    ),
                    predefinedEntityName: Some(
                        big2_predefinedEntityName
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                            ) -> c_int,
                    ),
                    updatePosition: Some(
                        big2_updatePosition
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut POSITION,
                            ) -> (),
                    ),
                    isPublicId: Some(
                        big2_isPublicId
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *const c_char,
                                _: *const c_char,
                                _: *mut *const c_char,
                            ) -> c_int,
                    ),
                    utf8Convert: Some(
                        big2_toUtf8
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_char,
                                _: *const c_char,
                            )
                                -> XML_Convert_Result,
                    ),
                    utf16Convert: Some(
                        big2_toUtf16
                            as unsafe extern "C" fn(
                                _: *const ENCODING,
                                _: *mut *const c_char,
                                _: *const c_char,
                                _: *mut *mut c_ushort,
                                _: *const c_ushort,
                            )
                                -> XML_Convert_Result,
                    ),
                    minBytesPerChar: 2i32,
                    isUtf8: 0i8,
                    isUtf16: 0i8,
                };
                init
            },
            type_0: [
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_LF as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_CR as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_NONXML as c_uchar,
                
                BT_S as c_uchar,
                
                BT_EXCL as c_uchar,
                
                BT_QUOT as c_uchar,
                
                BT_NUM as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_PERCNT as c_uchar,
                
                BT_AMP as c_uchar,
                
                BT_APOS as c_uchar,
                
                BT_LPAR as c_uchar,
                
                BT_RPAR as c_uchar,
                
                BT_AST as c_uchar,
                
                BT_PLUS as c_uchar,
                
                BT_COMMA as c_uchar,
                
                BT_MINUS as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_SOL as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                
                BT_DIGIT as c_uchar,
                BT_COLON_3 as c_uchar,
                
                BT_SEMI as c_uchar,
                
                BT_LT as c_uchar,
                
                BT_EQUALS as c_uchar,
                
                BT_GT as c_uchar,
                
                BT_QUEST as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_LSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_RSQB as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_HEX as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_VERBAR as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NAME as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_OTHER as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
                
                BT_NMSTRT as c_uchar,
            ],
            isName2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isName4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isNmstrt4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid2: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid3: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
            isInvalid4: ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int>,
            >(NULL as intptr_t),
        };
        init
    }
};

pub const BT_COLON_3: c_int = BT_NMSTRT as c_int;

unsafe extern "C" fn streqci(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    loop {
        let fresh58 = s1;
        s1 = s1.offset(1);
        let mut c1: c_char = *fresh58;
        let fresh59 = s2;
        s2 = s2.offset(1);
        let mut c2: c_char = *fresh59;
        if ASCII_a <= c1 && c1 <= ASCII_z {
            c1 = (c1 as c_int + (ASCII_A - ASCII_a) as c_int) as c_char
        }
        if ASCII_a <= c2 && c2 <= ASCII_z {
            /* The following line will never get executed.  streqci() is
             * only called from two places, both of which guarantee to put
             * upper-case strings into s2.
             */
            c2 = (c2 as c_int + (ASCII_A - ASCII_a) as c_int) as c_char
        }
        if c1 as c_int != c2 as c_int {
            return 0i32;
        }
        if c1 == 0 {
            break;
        }
    }
    return 1i32;
}

unsafe extern "C" fn initUpdatePosition(
    mut _enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut pos: *mut POSITION,
) {
    normal_updatePosition(&utf8_encoding.enc, ptr, end, pos);
}

unsafe extern "C" fn toAscii(
    mut enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
) -> c_int {
    let mut buf: [c_char; 1] = [0; 1];
    let mut p: *mut c_char = buf.as_mut_ptr();
    (*enc).utf8Convert.expect("non-null function pointer")(
        enc,
        &mut ptr,
        end,
        &mut p,
        p.offset(1isize),
    );
    if p == buf.as_mut_ptr() {
        return -1;
    } else {
        return buf[0usize] as c_int;
    };
}

unsafe extern "C" fn isSpace(mut c: c_int) -> c_int {
    match c {
        32 | 13 | 10 | 9 => return 1i32,
        _ => {}
    }
    return 0i32;
}
/* Return 1 if there's just optional white space or there's an S
   followed by name=val.
*/

unsafe extern "C" fn parsePseudoAttribute(
    mut enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut namePtr: *mut *const c_char,
    mut nameEndPtr: *mut *const c_char,
    mut valPtr: *mut *const c_char,
    mut nextTokPtr: *mut *const c_char,
) -> c_int {
    let mut c: c_int = 0;
    let mut open: c_char = 0;
    if ptr == end {
        *namePtr = NULL as *const c_char;
        return 1i32;
    }
    if isSpace(toAscii(enc, ptr, end)) == 0 {
        *nextTokPtr = ptr;
        return 0i32;
    }
    loop {
        ptr = ptr.offset((*enc).minBytesPerChar as isize);
        if !(isSpace(toAscii(enc, ptr, end)) != 0) {
            break;
        }
    }
    if ptr == end {
        *namePtr = NULL as *const c_char;
        return 1i32;
    }
    *namePtr = ptr;
    loop {
        c = toAscii(enc, ptr, end);
        if c == -1 {
            *nextTokPtr = ptr;
            return 0i32;
        }
        if c == ASCII_EQUALS as c_int {
            *nameEndPtr = ptr;
            break;
        } else if isSpace(c) != 0 {
            *nameEndPtr = ptr;
            loop {
                ptr = ptr.offset((*enc).minBytesPerChar as isize);
                c = toAscii(enc, ptr, end);
                if !(isSpace(c) != 0) {
                    break;
                }
            }
            if c != ASCII_EQUALS as c_int {
                *nextTokPtr = ptr;
                return 0i32;
            }
            break;
        } else {
            ptr = ptr.offset((*enc).minBytesPerChar as isize)
        }
    }
    if ptr == *namePtr {
        *nextTokPtr = ptr;
        return 0i32;
    }
    ptr = ptr.offset((*enc).minBytesPerChar as isize);
    c = toAscii(enc, ptr, end);
    while isSpace(c) != 0 {
        ptr = ptr.offset((*enc).minBytesPerChar as isize);
        c = toAscii(enc, ptr, end)
    }
    if c != ASCII_QUOT as c_int && c != ASCII_APOS as c_int {
        *nextTokPtr = ptr;
        return 0i32;
    }
    open = c as c_char;
    ptr = ptr.offset((*enc).minBytesPerChar as isize);
    *valPtr = ptr;
    loop {
        c = toAscii(enc, ptr, end);
        if c == open as c_int {
            break;
        }
        if !(ASCII_a as c_int <= c && c <= ASCII_z as c_int)
            && !(ASCII_A as c_int <= c && c <= ASCII_Z as c_int)
            && !(ASCII_0 as c_int <= c && c <= ASCII_9 as c_int)
            && c != ASCII_PERIOD as c_int
            && c != ASCII_MINUS as c_int
            && c != ASCII_UNDERSCORE as c_int
        {
            *nextTokPtr = ptr;
            return 0i32;
        }
        ptr = ptr.offset((*enc).minBytesPerChar as isize)
    }
    *nextTokPtr = ptr.offset((*enc).minBytesPerChar as isize);
    return 1i32;
}

static mut KW_version: [c_char; 8] = [
    
    ASCII_v,
    
    ASCII_e,
    
    ASCII_r,
    
    ASCII_s,
    
    ASCII_i,
    
    ASCII_o,
    
    ASCII_n,
    
    '\u{0}' as c_char,
];

static mut KW_encoding: [c_char; 9] = [
    
    ASCII_e,
    
    ASCII_n,
    
    ASCII_c,
    
    ASCII_o,
    
    ASCII_d,
    
    ASCII_i,
    
    ASCII_n,
    
    ASCII_g,
    
    '\u{0}' as c_char,
];

static mut KW_standalone: [c_char; 11] = [
    
    ASCII_s,
    
    ASCII_t,
    
    ASCII_a,
    
    ASCII_n,
    
    ASCII_d,
    
    ASCII_a,
    
    ASCII_l,
    
    ASCII_o,
    
    ASCII_n,
    
    ASCII_e,
    
    '\u{0}' as c_char,
];

static mut KW_yes: [c_char; 4] = [
    
    ASCII_y,
    
    ASCII_e,
    
    ASCII_s,
    
    '\u{0}' as c_char,
];

static mut KW_no: [c_char; 3] = [
    
    ASCII_n,
    
    ASCII_o,
    
    '\u{0}' as c_char,
];

unsafe extern "C" fn doParseXmlDecl(
    mut encodingFinder: Option<
        unsafe extern "C" fn(
            _: *const ENCODING,
            _: *const c_char,
            _: *const c_char,
        ) -> *const ENCODING,
    >,
    mut isGeneralTextEntity: c_int,
    mut enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut badPtr: *mut *const c_char,
    mut versionPtr: *mut *const c_char,
    mut versionEndPtr: *mut *const c_char,
    mut encodingName: *mut *const c_char,
    mut encoding: *mut *const ENCODING,
    mut standalone: *mut c_int,
) -> c_int {
    let mut val: *const c_char = NULL as *const c_char;
    let mut name: *const c_char = NULL as *const c_char;
    let mut nameEnd: *const c_char = NULL as *const c_char;
    ptr = ptr.offset((5i32 * (*enc).minBytesPerChar) as isize);
    end = end.offset(-((2i32 * (*enc).minBytesPerChar) as isize));
    if parsePseudoAttribute(enc, ptr, end, &mut name, &mut nameEnd, &mut val, &mut ptr) == 0
        || name.is_null()
    {
        *badPtr = ptr;
        return 0i32;
    }
    if (*enc).nameMatchesAscii.expect("non-null function pointer")(
        enc,
        name,
        nameEnd,
        KW_version.as_ptr(),
    ) == 0
    {
        if isGeneralTextEntity == 0 {
            *badPtr = name;
            return 0i32;
        }
    } else {
        if !versionPtr.is_null() {
            *versionPtr = val
        }
        if !versionEndPtr.is_null() {
            *versionEndPtr = ptr
        }
        if parsePseudoAttribute(enc, ptr, end, &mut name, &mut nameEnd, &mut val, &mut ptr) == 0 {
            *badPtr = ptr;
            return 0i32;
        }
        if name.is_null() {
            if isGeneralTextEntity != 0 {
                /* a TextDecl must have an EncodingDecl */
                *badPtr = ptr;
                return 0i32;
            }
            return 1i32;
        }
    }
    if (*enc).nameMatchesAscii.expect("non-null function pointer")(
        enc,
        name,
        nameEnd,
        KW_encoding.as_ptr(),
    ) != 0
    {
        let mut c: c_int = toAscii(enc, val, end);
        if !(ASCII_a as c_int <= c && c <= ASCII_z as c_int)
            && !(ASCII_A as c_int <= c && c <= ASCII_Z as c_int)
        {
            *badPtr = val;
            return 0i32;
        }
        if !encodingName.is_null() {
            *encodingName = val
        }
        if !encoding.is_null() {
            *encoding = encodingFinder.expect("non-null function pointer")(
                enc,
                val,
                ptr.offset(-((*enc).minBytesPerChar as isize)),
            )
        }
        if parsePseudoAttribute(enc, ptr, end, &mut name, &mut nameEnd, &mut val, &mut ptr) == 0 {
            *badPtr = ptr;
            return 0i32;
        }
        if name.is_null() {
            return 1i32;
        }
    }
    if (*enc).nameMatchesAscii.expect("non-null function pointer")(
        enc,
        name,
        nameEnd,
        KW_standalone.as_ptr(),
    ) == 0
        || isGeneralTextEntity != 0
    {
        *badPtr = name;
        return 0i32;
    }
    if (*enc).nameMatchesAscii.expect("non-null function pointer")(
        enc,
        val,
        ptr.offset(-((*enc).minBytesPerChar as isize)),
        KW_yes.as_ptr(),
    ) != 0
    {
        if !standalone.is_null() {
            *standalone = 1i32
        }
    } else if (*enc).nameMatchesAscii.expect("non-null function pointer")(
        enc,
        val,
        ptr.offset(-((*enc).minBytesPerChar as isize)),
        KW_no.as_ptr(),
    ) != 0
    {
        if !standalone.is_null() {
            *standalone = 0i32
        }
    } else {
        *badPtr = val;
        return 0i32;
    }
    while isSpace(toAscii(enc, ptr, end)) != 0 {
        ptr = ptr.offset((*enc).minBytesPerChar as isize)
    }
    if ptr != end {
        *badPtr = ptr;
        return 0i32;
    }
    return 1i32;
}
/* as nothing */
/* isName2 */
/* isName3 */
/* isName4 */
/* isNmstrt2 */
/* isNmstrt3 */
/* isNmstrt4 */
/* isInvalid2 */
/* isInvalid3 */
/* isInvalid4 */

unsafe extern "C" fn checkCharRefNumber(mut result: c_int) -> c_int {
    match result >> 8i32 {
        216 | 217 | 218 | 219 | 220 | 221 | 222 | 223 => return -1,
        0 => {
            if latin1_encoding.type_0[result as usize] as c_int == BT_NONXML as c_int {
                return -1;
            }
        }
        255 => {
            if result == 0xfffei32 || result == 0xffffi32 {
                return -1;
            }
        }
        _ => {}
    } /* LCOV_EXCL_LINE: this case is always eliminated beforehand */
    return result;
}
#[no_mangle]

pub unsafe extern "C" fn XmlUtf8Encode(mut c: c_int, mut buf: *mut c_char) -> c_int {
    if c < 0i32 {
        return 0i32;
    }
    if c < min2 as c_int {
        *buf.offset(0isize) = (c | UTF8_cval1 as c_int) as c_char;
        return 1i32;
    }
    if c < min3 as c_int {
        *buf.offset(0isize) = (c >> 6i32 | UTF8_cval2 as c_int) as c_char;
        *buf.offset(1isize) = (c & 0x3fi32 | 0x80i32) as c_char;
        return 2i32;
    }
    if c < min4 as c_int {
        *buf.offset(0isize) = (c >> 12i32 | UTF8_cval3 as c_int) as c_char;
        *buf.offset(1isize) =
            (c >> 6i32 & 0x3fi32 | 0x80i32) as c_char;
        *buf.offset(2isize) = (c & 0x3fi32 | 0x80i32) as c_char;
        return 3i32;
    }
    if c < 0x110000i32 {
        *buf.offset(0isize) = (c >> 18i32 | UTF8_cval4 as c_int) as c_char;
        *buf.offset(1isize) =
            (c >> 12i32 & 0x3fi32 | 0x80i32) as c_char;
        *buf.offset(2isize) =
            (c >> 6i32 & 0x3fi32 | 0x80i32) as c_char;
        *buf.offset(3isize) = (c & 0x3fi32 | 0x80i32) as c_char;
        return 4i32;
    }
    return 0i32;
    /* LCOV_EXCL_LINE: this case too is eliminated before calling */
}
#[no_mangle]

pub unsafe extern "C" fn XmlUtf16Encode(mut charNum: c_int, mut buf: *mut c_ushort) -> c_int {
    if charNum < 0i32 {
        return 0i32;
    }
    if charNum < 0x10000i32 {
        *buf.offset(0isize) = charNum as c_ushort;
        return 1i32;
    }
    if charNum < 0x110000i32 {
        charNum -= 0x10000i32;
        *buf.offset(0isize) = ((charNum >> 10i32) + 0xd800i32) as c_ushort;
        *buf.offset(1isize) =
            ((charNum & 0x3ffi32) + 0xdc00i32) as c_ushort;
        return 2i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn XmlSizeOfUnknownEncoding() -> c_int {
    return  ::std::mem::size_of::<unknown_encoding>() as c_int;
}

unsafe extern "C" fn unknown_isName(mut enc: *const ENCODING, mut p: *const c_char) -> c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: c_int = (*uenc).convert.expect("non-null function pointer")((*uenc).userData, p);
    if c & !(0xffffi32) != 0 {
        return 0i32;
    }
    return (namingBitmap[(((namePages[(c >> 8i32) as usize] as c_int) << 3i32)
        + ((c & 0xffi32) >> 5i32)) as usize]
        & (1u32) << (c & 0xffi32 & 0x1fi32)) as c_int;
}

unsafe extern "C" fn unknown_isNmstrt(mut enc: *const ENCODING, mut p: *const c_char) -> c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: c_int = (*uenc).convert.expect("non-null function pointer")((*uenc).userData, p);
    if c & !(0xffffi32) != 0 {
        return 0i32;
    }
    return (namingBitmap[(((nmstrtPages[(c >> 8i32) as usize] as c_int) << 3i32)
        + ((c & 0xffi32) >> 5i32)) as usize]
        & (1u32) << (c & 0xffi32 & 0x1fi32)) as c_int;
}

unsafe extern "C" fn unknown_isInvalid(mut enc: *const ENCODING, mut p: *const c_char) -> c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: c_int = (*uenc).convert.expect("non-null function pointer")((*uenc).userData, p);
    return (c & !(0xffffi32) != 0 || checkCharRefNumber(c) < 0i32) as c_int;
}

unsafe extern "C" fn unknown_toUtf8(
    mut enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut buf: [c_char; 4] = [0; 4];
    loop {
        let mut utf8: *const c_char = 0 as *const c_char;
        let mut n: c_int = 0;
        if *fromP == fromLim {
            return XML_CONVERT_COMPLETED;
        }
        utf8 = (*uenc).utf8[**fromP as c_uchar as usize].as_ptr();
        let fresh60 = utf8;
        utf8 = utf8.offset(1);
        n = *fresh60 as c_int;
        if n == 0i32 {
            let mut c: c_int =
                (*uenc).convert.expect("non-null function pointer")((*uenc).userData, *fromP);
            n = XmlUtf8Encode(c, buf.as_mut_ptr());
            if n as c_long > toLim.wrapping_offset_from(*toP) as c_long {
                return XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            utf8 = buf.as_mut_ptr();
            *fromP = (*fromP).offset(
                ((*(enc as *const normal_encoding)).type_0[**fromP as c_uchar as usize] as c_int
                    - (BT_LEAD2 as c_int - 2i32)) as isize,
            )
        } else {
            if n as c_long > toLim.wrapping_offset_from(*toP) as c_long {
                return XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            *fromP = (*fromP).offset(1)
        }
        memcpy(*toP as *mut c_void, utf8 as *const c_void, n as c_ulong);
        *toP = (*toP).offset(n as isize)
    }
}

unsafe extern "C" fn unknown_toUtf16(
    mut enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_ushort,
    mut toLim: *const c_ushort,
) -> XML_Convert_Result {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let mut c: c_ushort = (*uenc).utf16[**fromP as c_uchar as usize];
        if c as c_int == 0i32 {
            c = (*uenc).convert.expect("non-null function pointer")((*uenc).userData, *fromP)
                as c_ushort;
            *fromP = (*fromP).offset(
                ((*(enc as *const normal_encoding)).type_0[**fromP as c_uchar as usize] as c_int
                    - (BT_LEAD2 as c_int - 2i32)) as isize,
            )
        } else {
            *fromP = (*fromP).offset(1)
        }
        let fresh61 = *toP;
        *toP = (*toP).offset(1);
        *fresh61 = c
    }
    if *toP == toLim as *mut c_ushort && *fromP < fromLim {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}
#[no_mangle]

pub unsafe extern "C" fn XmlInitUnknownEncoding(
    mut mem: *mut c_void,
    mut table: *mut c_int,
    mut convert: CONVERTER,
    mut userData: *mut c_void,
) -> *mut ENCODING {
    let mut i: c_int = 0;
    let mut e: *mut unknown_encoding = mem as *mut unknown_encoding;
    memcpy(
        mem,
        &latin1_encoding as *const normal_encoding as *const c_void,
        ::std::mem::size_of::<normal_encoding>() as c_ulong,
    );
    i = 0i32;
    while i < 128i32 {
        if latin1_encoding.type_0[i as usize] as c_int != BT_OTHER as c_int
            && latin1_encoding.type_0[i as usize] as c_int != BT_NONXML as c_int
            && *table.offset(i as isize) != i
        {
            return 0 as *mut ENCODING;
        }
        i += 1
    }
    i = 0i32;
    while i < 256i32 {
        let mut c: c_int = *table.offset(i as isize);
        if c == -1 {
            (*e).normal.type_0[i as usize] =  BT_MALFORM as c_uchar;
            /* This shouldn't really get used. */
            (*e).utf16[i as usize] = 0xffffu16;
            (*e).utf8[i as usize][0usize] = 1i8;
            (*e).utf8[i as usize][1usize] = 0i8
        } else if c < 0i32 {
            if c < -(4i32) {
                return 0 as *mut ENCODING;
            }
            /* Multi-byte sequences need a converter function */
            if convert.is_none() {
                return 0 as *mut ENCODING;
            }
            (*e).normal.type_0[i as usize] = (BT_LEAD2 as c_int - (c + 2i32)) as c_uchar;
            (*e).utf8[i as usize][0usize] = 0i8;
            (*e).utf16[i as usize] = 0u16
        } else if c < 0x80i32 {
            if latin1_encoding.type_0[c as usize] as c_int != BT_OTHER as c_int
                && latin1_encoding.type_0[c as usize] as c_int != BT_NONXML as c_int
                && c != i
            {
                return 0 as *mut ENCODING;
            }
            (*e).normal.type_0[i as usize] = latin1_encoding.type_0[c as usize];
            (*e).utf8[i as usize][0usize] = 1i8;
            (*e).utf8[i as usize][1usize] = c as c_char;
            (*e).utf16[i as usize] = if c == 0i32 { 0xffffi32 } else { c } as c_ushort
        } else if checkCharRefNumber(c) < 0i32 {
            (*e).normal.type_0[i as usize] =  BT_NONXML as c_uchar;
            /* This shouldn't really get used. */
            (*e).utf16[i as usize] = 0xffffu16;
            (*e).utf8[i as usize][0usize] = 1i8;
            (*e).utf8[i as usize][1usize] = 0i8
        } else {
            if c > 0xffffi32 {
                return 0 as *mut ENCODING;
            }
            if namingBitmap[(((nmstrtPages[(c >> 8i32) as usize] as c_int) << 3i32)
                + ((c & 0xffi32) >> 5i32)) as usize]
                & (1u32) << (c & 0xffi32 & 0x1fi32)
                != 0
            {
                (*e).normal.type_0[i as usize] =  BT_NMSTRT as c_uchar
            } else if namingBitmap[(((namePages[(c >> 8i32) as usize] as c_int)
                << 3i32)
                + ((c & 0xffi32) >> 5i32)) as usize]
                & (1u32) << (c & 0xffi32 & 0x1fi32)
                != 0
            {
                (*e).normal.type_0[i as usize] =  BT_NAME as c_uchar
            } else {
                (*e).normal.type_0[i as usize] =  BT_OTHER as c_uchar
            }
            (*e).utf8[i as usize][0usize] = XmlUtf8Encode(
                c,
                (*e).utf8[i as usize]
                    .as_mut_ptr()
                    .offset(1isize),
            ) as c_char;
            (*e).utf16[i as usize] = c as c_ushort
        }
        i += 1
    }
    (*e).userData = userData;
    (*e).convert = convert;
    if convert.is_some() {
        (*e).normal.isName2 = Some(
            unknown_isName as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        );
        (*e).normal.isName3 = Some(
            unknown_isName as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        );
        (*e).normal.isName4 = Some(
            unknown_isName as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        );
        (*e).normal.isNmstrt2 = Some(
            unknown_isNmstrt as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        );
        (*e).normal.isNmstrt3 = Some(
            unknown_isNmstrt as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        );
        (*e).normal.isNmstrt4 = Some(
            unknown_isNmstrt as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        );
        (*e).normal.isInvalid2 = Some(
            unknown_isInvalid
                as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        );
        (*e).normal.isInvalid3 = Some(
            unknown_isInvalid
                as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        );
        (*e).normal.isInvalid4 = Some(
            unknown_isInvalid
                as unsafe extern "C" fn(_: *const ENCODING, _: *const c_char) -> c_int,
        )
    }
    (*e).normal.enc.utf8Convert = Some(
        unknown_toUtf8
            as unsafe extern "C" fn(
                _: *const ENCODING,
                _: *mut *const c_char,
                _: *const c_char,
                _: *mut *mut c_char,
                _: *const c_char,
            ) -> XML_Convert_Result,
    );
    (*e).normal.enc.utf16Convert = Some(
        unknown_toUtf16
            as unsafe extern "C" fn(
                _: *const ENCODING,
                _: *mut *const c_char,
                _: *const c_char,
                _: *mut *mut c_ushort,
                _: *const c_ushort,
            ) -> XML_Convert_Result,
    );
    return &mut (*e).normal.enc;
}

static mut KW_ISO_8859_1: [c_char; 11] = [
    
    ASCII_I,
    
    ASCII_S,
    
    ASCII_O,
    
    ASCII_MINUS,
    
    ASCII_8,
    
    ASCII_8,
    
    ASCII_5,
    
    ASCII_9,
    
    ASCII_MINUS,
    
    ASCII_1,
    
    '\u{0}' as c_char,
];

static mut KW_US_ASCII: [c_char; 9] = [
    
    ASCII_U,
    
    ASCII_S,
    
    ASCII_MINUS,
    
    ASCII_A,
    
    ASCII_S,
    
    ASCII_C,
    
    ASCII_I,
    
    ASCII_I,
    
    '\u{0}' as c_char,
];

static mut KW_UTF_8: [c_char; 6] = [
    
    ASCII_U,
    
    ASCII_T,
    
    ASCII_F,
    
    ASCII_MINUS,
    
    ASCII_8,
    
    '\u{0}' as c_char,
];

static mut KW_UTF_16: [c_char; 7] = [
    
    ASCII_U,
    
    ASCII_T,
    
    ASCII_F,
    
    ASCII_MINUS,
    
    ASCII_1,
    
    ASCII_6,
    
    '\u{0}' as c_char,
];

static mut KW_UTF_16BE: [c_char; 9] = [
    
    ASCII_U,
    
    ASCII_T,
    
    ASCII_F,
    
    ASCII_MINUS,
    
    ASCII_1,
    
    ASCII_6,
    
    ASCII_B,
    
    ASCII_E,
    
    '\u{0}' as c_char,
];

static mut KW_UTF_16LE: [c_char; 9] = [
    
    ASCII_U,
    
    ASCII_T,
    
    ASCII_F,
    
    ASCII_MINUS,
    
    ASCII_1,
    
    ASCII_6,
    
    ASCII_L,
    
    ASCII_E,
    
    '\u{0}' as c_char,
];

unsafe extern "C" fn getEncodingIndex(mut name: *const c_char) -> c_int {
    static mut encodingNames: [*const c_char; 6] = unsafe {
        [
            KW_ISO_8859_1.as_ptr(),
            KW_US_ASCII.as_ptr(),
            KW_UTF_8.as_ptr(),
            KW_UTF_16.as_ptr(),
            KW_UTF_16BE.as_ptr(),
            KW_UTF_16LE.as_ptr(),
        ]
    };
    let mut i: c_int = 0;
    if name.is_null() {
        return  NO_ENC;
    }
    i = 0i32;
    while i
        < (::std::mem::size_of::<[*const c_char; 6]>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<*const c_char>() as c_ulong) as c_int
    {
        if streqci(name, encodingNames[i as usize]) != 0 {
            return i;
        }
        i += 1
    }
    return  UNKNOWN_ENC;
}
/* For binary compatibility, we store the index of the encoding
   specified at initialization in the isUtf16 member.
*/
/* This is what detects the encoding.  encodingTable maps from
   encoding indices to encodings; INIT_ENC_INDEX(enc) is the index of
   the external (protocol) specified encoding; state is
   XML_CONTENT_STATE if we're parsing an external text entity, and
   XML_PROLOG_STATE otherwise.
*/

unsafe extern "C" fn initScan(
    mut encodingTable: *const *const ENCODING,
    mut enc: *const INIT_ENCODING,
    mut state: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut nextTokPtr: *mut *const c_char,
) -> c_int {
    let mut encPtr: *mut *const ENCODING = 0 as *mut *const ENCODING;
    if ptr >= end {
        return crate::xmltok_h::XML_TOK_NONE;
    }
    encPtr = (*enc).encPtr;
    if ptr.offset(1isize) == end {
        /* only a single byte available for auto-detection */
        /* FIXME */
        /* so we're parsing an external text entity... */
        /* if UTF-16 was externally specified, then we need at least 2 bytes */
        match (*enc).initEnc.isUtf16 as c_int {
            3 | 5 | 4 => return crate::xmltok_h::XML_TOK_PARTIAL,
            _ => {}
        }
        let mut current_block_5: u64;
        match *ptr as c_uchar as c_int {
            254 | 255 | 239 => {
                /* possibly first byte of UTF-8 BOM */
                if (*enc).initEnc.isUtf16 as c_int ==  ISO_8859_1_ENC
                    && state == XML_CONTENT_STATE
                {
                    current_block_5 = 17965632435239708295;
                } else {
                    current_block_5 = 16867440708908940295;
                }
            }
            0 | 60 => {
                current_block_5 = 16867440708908940295;
            }
            _ => {
                current_block_5 = 17965632435239708295;
            }
        }
        match current_block_5 {
            17965632435239708295 => {}
            _ =>
            /* fall through */
            {
                return crate::xmltok_h::XML_TOK_PARTIAL
            }
        }
    } else {
        let mut current_block_26: u64;
        match (*ptr.offset(0isize) as c_uchar as c_int) << 8i32
            | *ptr.offset(1isize) as c_uchar as c_int
        {
            65279 => {
                if !((*enc).initEnc.isUtf16 as c_int ==  ISO_8859_1_ENC
                    && state == XML_CONTENT_STATE)
                {
                    *nextTokPtr = ptr.offset(2isize);
                    *encPtr = *encodingTable.offset(UTF_16BE_ENC as isize);
                    return crate::xmltok_h::XML_TOK_BOM;
                }
            }
            15360 => {
                /* 00 3C is handled in the default case */
                if !(((*enc).initEnc.isUtf16 as c_int ==  UTF_16BE_ENC
                    || (*enc).initEnc.isUtf16 as c_int ==  UTF_16_ENC)
                    && state == XML_CONTENT_STATE)
                {
                    *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                    return (**encPtr).scanners[state as usize].expect("non-null function pointer")(
                        *encPtr, ptr, end, nextTokPtr,
                    );
                }
            }
            65534 => {
                if !((*enc).initEnc.isUtf16 as c_int ==  ISO_8859_1_ENC
                    && state == XML_CONTENT_STATE)
                {
                    *nextTokPtr = ptr.offset(2isize);
                    *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                    return crate::xmltok_h::XML_TOK_BOM;
                }
            }
            61371 => {
                /* Maybe a UTF-8 BOM (EF BB BF) */
                /* If there's an explicitly specified (external) encoding
                   of ISO-8859-1 or some flavour of UTF-16
                   and this is an external text entity,
                   don't look for the BOM,
                   because it might be a legal data.
                */
                if state == XML_CONTENT_STATE {
                    let mut e: c_int = (*enc).initEnc.isUtf16 as c_int;
                    if e ==  ISO_8859_1_ENC
                        || e ==  UTF_16BE_ENC
                        || e ==  UTF_16LE_ENC
                        || e ==  UTF_16_ENC
                    {
                        current_block_26 = 10758786907990354186;
                    } else {
                        current_block_26 = 15925075030174552612;
                    }
                } else {
                    current_block_26 = 15925075030174552612;
                }
                match current_block_26 {
                    10758786907990354186 => {}
                    _ => {
                        if ptr.offset(2isize) == end {
                            return crate::xmltok_h::XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(2isize) as c_uchar as c_int == 0xbfi32 {
                            *nextTokPtr = ptr.offset(3isize);
                            *encPtr = *encodingTable.offset(UTF_8_ENC as isize);
                            return crate::xmltok_h::XML_TOK_BOM;
                        }
                    }
                }
            }
            _ => {
                if *ptr.offset(0isize) as c_int == '\u{0}' as i32 {
                    /* 0 isn't a legal data character. Furthermore a document
                       entity can only start with ASCII characters.  So the only
                       way this can fail to be big-endian UTF-16 if it it's an
                       external parsed general entity that's labelled as
                       UTF-16LE.
                    */
                    if !(state == XML_CONTENT_STATE
                        && (*enc).initEnc.isUtf16 as c_int ==  UTF_16LE_ENC)
                    {
                        *encPtr = *encodingTable.offset(UTF_16BE_ENC as isize);
                        return (**encPtr).scanners[state as usize]
                            .expect("non-null function pointer")(
                            *encPtr, ptr, end, nextTokPtr
                        );
                    }
                } else if *ptr.offset(1isize) as c_int == '\u{0}' as i32 {
                    /* We could recover here in the case:
                        - parsing an external entity
                        - second byte is 0
                        - no externally specified encoding
                        - no encoding declaration
                       by assuming UTF-16LE.  But we don't, because this would mean when
                       presented just with a single byte, we couldn't reliably determine
                       whether we needed further bytes.
                    */
                    if !(state == XML_CONTENT_STATE) {
                        *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                        return (**encPtr).scanners[state as usize]
                            .expect("non-null function pointer")(
                            *encPtr, ptr, end, nextTokPtr
                        );
                    }
                }
            }
        }
    }
    *encPtr = *encodingTable.offset((*enc).initEnc.isUtf16 as c_int as isize);
    return (**encPtr).scanners[state as usize].expect("non-null function pointer")(
        *encPtr, ptr, end, nextTokPtr,
    );
}
#[no_mangle]

pub unsafe extern "C" fn XmlInitUnknownEncodingNS(
    mut mem: *mut c_void,
    mut table: *mut c_int,
    mut convert: CONVERTER,
    mut userData: *mut c_void,
) -> *mut ENCODING {
    let mut enc: *mut ENCODING = XmlInitUnknownEncoding(mem, table, convert, userData);
    if !enc.is_null() {
        (*(enc as *mut normal_encoding)).type_0[ASCII_COLON as usize] =
            
            BT_COLON_0 as c_uchar
    }
    return enc;
}
unsafe extern "C" fn run_static_initializers() {
    encodingsNS = [
        &latin1_encoding_ns.enc,
        &ascii_encoding_ns.enc,
        &utf8_encoding_ns.enc,
        &big2_encoding_ns.enc,
        &big2_encoding_ns.enc,
        &little2_encoding_ns.enc,
        &utf8_encoding_ns.enc,
    ];
    encodings = [
        &latin1_encoding.enc,
        &ascii_encoding.enc,
        &utf8_encoding.enc,
        &big2_encoding.enc,
        &big2_encoding.enc,
        &little2_encoding.enc,
        &utf8_encoding.enc,
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
/* XML_NS */
