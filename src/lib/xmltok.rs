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

/* The following token may be returned by XmlContentTok */

use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void, size_t};
use crate::expat_h::{XML_Error};
use super::xmlparse::{ExpatBufRef, ExpatBufRefMut};
use std::convert::TryInto;
use std::marker::PhantomData;
use std::ptr;
use crate::xmltok_impl_h::ByteType;
use num_derive::{ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};

#[cfg(feature = "mozilla")]
pub mod moz_extensions;


#[repr(i32)]
#[derive(ToPrimitive, PartialEq, Copy, Clone, Debug)]
pub enum XML_TOK {    
    TRAILING_RSQB = -5, /* ] or ]] at the end of the scan, might be
                                             start of illegal ]]> sequence */

/* The following tokens may be returned by both XmlPrologTok and
   XmlContentTok.
*/
    NONE = -4, /* The string to be scanned is empty */
    TRAILING_CR = -3, /* A CR at the end of the scan,
                                                * might be part of CRLF sequence */
    PARTIAL_CHAR = -2, /* only part of a multibyte sequence */
    PARTIAL = -1, /* only part of a token */
    INVALID = 0,

/* The following tokens are returned by XmlContentTok, some are also
   returned by XmlAttributeValueTok, XmlEntityTok, XmlCdataSectionTok.
*/

    START_TAG_WITH_ATTS = 1,
    START_TAG_NO_ATTS = 2,
    EMPTY_ELEMENT_WITH_ATTS = 3, /* empty element tag <e/> */
    EMPTY_ELEMENT_NO_ATTS = 4,
    END_TAG = 5,
    DATA_CHARS = 6,
    DATA_NEWLINE = 7,
    CDATA_SECT_OPEN = 8,
    ENTITY_REF = 9,
    CHAR_REF = 10, /* numeric character reference */

/* The following tokens may be returned by both XmlPrologTok and
   XmlContentTok.
*/
    PI = 11, /* processing instruction */
    XML_DECL = 12, /* XML decl or text decl */
    COMMENT = 13,
    BOM = 14, /* Byte order mark */

/* The following tokens are returned only by XmlPrologTok */
    PROLOG_S = 15,
    PROLOG_S_NEG = -15,      /* NOTE: added in c2rust port */
    DECL_OPEN = 16, /* <!foo */
    DECL_CLOSE = 17, /* > */
    NAME = 18,
    NAME_NEG = -18,
    NMTOKEN = 19,
    NMTOKEN_NEG = -19,       /* NOTE: added in c2rust port */
    POUND_NAME = 20, /* #name */
    POUND_NAME_NEG = -20,    /* NOTE: added in c2rust port */
    OR = 21, /* | */
    PERCENT = 22,
    OPEN_PAREN = 23,
    CLOSE_PAREN = 24,
    CLOSE_PAREN_NEG = -24,   /* NOTE: added in c2rust port */
    OPEN_BRACKET = 25,
    CLOSE_BRACKET = 26,
    CLOSE_BRACKET_NEG = -26, /* NOTE: added in c2rust port */
    LITERAL = 27,
    LITERAL_NEG = -27,       /* NOTE: added in c2rust port */
    PARAM_ENTITY_REF = 28,
    INSTANCE_START = 29,

/* The following occur only in element type declarations */
    NAME_QUESTION = 30, /* name? */
    NAME_ASTERISK = 31, /* name* */
    NAME_PLUS = 32, /* name+ */
    COND_SECT_OPEN = 33, /* <![ */
    COND_SECT_CLOSE = 34, /* ]]> */
    CLOSE_PAREN_QUESTION = 35, /* )? */
    CLOSE_PAREN_ASTERISK = 36, /* )* */
    CLOSE_PAREN_PLUS = 37, /* )+ */
    COMMA = 38,

/* The following token is returned only by XmlAttributeValueTok */
    ATTRIBUTE_VALUE_S = 39,

/* The following token is returned only by XmlCdataSectionTok */
    CDATA_SECT_CLOSE = 40,

/* With namespace processing this is returned by XmlPrologTok for a
   name with a colon.
*/
    PREFIXED_NAME = 41,
    PREFIXED_NAME_NEG = -41,
    IGNORE_SECT = 42,
}

impl XML_TOK {
    pub fn is_error(&self) -> bool {
        self.to_i32().unwrap() <= 0
    }

    pub fn negate(&self) -> XML_TOK {
        match self {
            XML_TOK::CLOSE_PAREN_NEG => XML_TOK::CLOSE_PAREN,
            XML_TOK::POUND_NAME_NEG => XML_TOK::POUND_NAME,
            XML_TOK::LITERAL_NEG => XML_TOK::LITERAL,
            XML_TOK::NAME_NEG => XML_TOK::NAME,
            XML_TOK::NAME => XML_TOK::NAME_NEG,
            XML_TOK::PREFIXED_NAME => XML_TOK::PREFIXED_NAME_NEG,
            XML_TOK::NMTOKEN => XML_TOK::NMTOKEN_NEG,
            /* FIXME: not 100% sure we need the cases below */
            XML_TOK::CLOSE_BRACKET_NEG => XML_TOK::CLOSE_BRACKET,
            XML_TOK::CLOSE_BRACKET => XML_TOK::CLOSE_BRACKET_NEG,
            XML_TOK::PREFIXED_NAME_NEG => XML_TOK::PREFIXED_NAME,
            XML_TOK::NMTOKEN_NEG => XML_TOK::NMTOKEN,

            _unimplemented => unimplemented!(
                "negation not implemented for {}", 
                self.to_i32().unwrap()),
        }
    }
}

#[repr(i32)]
#[derive(PartialEq)]
pub enum XML_STATE {
    PROLOG = 0,
    CONTENT = 1, 
    CDATA_SECTION = 2,
    IGNORE_SECTION = 3
}

pub const XML_ATTRIBUTE_VALUE_LITERAL: c_int = 0;
pub const XML_ENTITY_VALUE_LITERAL: c_int = 1;

/* The size of the buffer passed to XmlUtf8Encode must be at least this. */
pub const XML_UTF8_ENCODE_MAX: c_int = 4;
/* The size of the buffer passed to XmlUtf16Encode must be at least this. */
pub const XML_UTF16_ENCODE_MAX: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Position {
    /* first line and first column are 0 not 1 */
    pub lineNumber: XML_Size,
    pub columnNumber: XML_Size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Attribute {
    pub name: *const c_char,
    pub valuePtr: *const c_char,
    pub valueEnd: *const c_char,
    pub normalized: bool,
}

pub type ENCODING = dyn XmlEncoding;

#[repr(u32)]
#[derive(PartialEq)]
pub enum XML_Convert_Result {
    COMPLETED = 0,
    INPUT_INCOMPLETE = 1,
    /* and therefore potentially input remaining as well */
    OUTPUT_EXHAUSTED = 2,
}

#[macro_export]
macro_rules! XmlUtf8Convert {
    ($enc:path, $from:expr, $to:expr $(,)?) => {
        (*$enc).utf8Convert($from, $to)
    };
}

#[macro_export]
macro_rules! XmlUtf16Convert {
    ($enc:path, $from:expr, $to:expr $(,)?) => {
        (*$enc).utf16Convert($from, $to)
    };
}

pub type Converter = Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int>;

// =============== END xmltok_h ================

// Replaces ENCODING
pub trait XmlEncoding {
    // scanners[4]
    fn prologTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK;
    fn contentTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK;
    fn cdataSectionTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK;
    fn ignoreSectionTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK;

    // literalScanners[2]
    fn attributeValueTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK;
    fn entityValueTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK;

    fn nameMatchesAscii(
        &self,
        buf: ExpatBufRef,
        ptr2: &[libc::c_char],
    ) -> bool;

    unsafe fn nameLength(&self, ptr: *const libc::c_char) -> libc::c_int;

    unsafe fn skipS(&self, ptr: *const libc::c_char) -> *const libc::c_char;

    fn getAtts(
        &self,
        buf: ExpatBufRef,
        f: &mut dyn FnMut(Attribute) -> XML_Error,
    ) -> XML_Error;

    fn charRefNumber(&self, buf: ExpatBufRef) -> libc::c_int;

    fn predefinedEntityName(
        &self,
        buf: ExpatBufRef,
    ) -> libc::c_int;

    fn updatePosition(
        &self,
        buf: ExpatBufRef,
        pos: &mut Position,
    );

    fn isPublicId(
        &self,
        buf: ExpatBufRef,
        badPtr: &mut *const libc::c_char,
    ) -> libc::c_int;

    fn utf8Convert<'b, 'a: 'b>(
        &self,
        from_buf: &'b mut ExpatBufRef<'a>,
        to_buf: &'b mut ExpatBufRefMut<'a>,
    ) -> XML_Convert_Result;

    fn utf16Convert(
        &self,
        from_buf: &mut ExpatBufRef,
        to_buf: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result;

    fn minBytesPerChar(&self) -> c_int;
    fn isUtf8(&self) -> bool;
    fn isUtf16(&self) -> bool;

    // xmlTok and xmlLiteralTok were macros
    #[inline]
    fn xmlTok(
        &self,
        state: XML_STATE,
        buf: ExpatBufRef,
        nextTokPtr: &mut *const c_char,
    ) -> XML_TOK {
        match state {
            XML_STATE::PROLOG => self.prologTok(buf, nextTokPtr),
            XML_STATE::CONTENT => self.contentTok(buf, nextTokPtr),
            XML_STATE::CDATA_SECTION => self.cdataSectionTok(buf, nextTokPtr),
            XML_STATE::IGNORE_SECTION => self.ignoreSectionTok(buf, nextTokPtr),
        }
    }

    #[inline]
    fn xmlLiteralTok(
        &self,
        literal_type: c_int,
        buf: ExpatBufRef,
        nextTokPtr: &mut *const c_char,
    ) -> XML_TOK {
        match literal_type {
            XML_ATTRIBUTE_VALUE_LITERAL => self.attributeValueTok(buf, nextTokPtr),
            XML_ENTITY_VALUE_LITERAL => self.entityValueTok(buf, nextTokPtr),
            _ => panic!("Unexpected literal type {}", literal_type),
        }
    }
}

pub trait XmlEncodingImpl {
    fn MINBPC(&self) -> isize;
    fn isUtf8(&self) -> bool;
    fn isUtf16(&self) -> bool;

    fn byte_type(&self, p: *const c_char) -> ByteType;
    fn byte_to_ascii(&self, p: *const c_char) -> c_char;
    fn is_name_char(&self, p: *const c_char, n: isize) -> bool;
    fn is_nmstrt_char(&self, p: *const c_char, n: isize) -> bool;
    fn is_invalid_char(&self, p: *const c_char, n: isize) -> bool;
    fn is_name_char_minbpc(&self, p: *const c_char) -> bool;
    fn is_nmstrt_char_minbpc(&self, p: *const c_char) -> bool;
    fn char_matches(&self, p: *const c_char, c: c_char) -> bool;

    fn utf8Convert<'a: 'b, 'b>(
        &self,
        from_buf: &'b mut ExpatBufRef<'a>,
        to_buf: &'b mut ExpatBufRefMut<'a>,
    ) -> XML_Convert_Result;

    fn utf16Convert(
        &self,
        from_buf: &mut ExpatBufRef,
        to_buf: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result;
}

struct NormalEncoding<T: NormalEncodingTable> {
    t: std::marker::PhantomData<T>,
}

trait NormalEncodingTable {
    const types: [ByteType; 256];
}

macro_rules! UCS2_GET_NAMING {
    ($pages:path, $hi:expr, $lo:expr) => {
        (namingBitmap[(($pages[$hi as u8 as usize] as usize) << 3) + (($lo as u8 as usize) >> 5)] & (1 << (($lo as u8 as usize)&0x1F)))
    };
}

struct Utf8EncodingImpl<T: NormalEncodingTable>(std::marker::PhantomData<T>);

type Utf8Encoding = Utf8EncodingImpl<Utf8EncodingTable>;
type Utf8EncodingNS = Utf8EncodingImpl<Utf8EncodingTableNS>;
type InternalUtf8Encoding = Utf8EncodingImpl<InternalUtf8EncodingTable>;
type InternalUtf8EncodingNS = Utf8EncodingImpl<InternalUtf8EncodingTableNS>;

impl<T: NormalEncodingTable> XmlEncodingImpl for Utf8EncodingImpl<T> {
    fn isUtf8(&self) -> bool { true }
    fn isUtf16(&self) -> bool { false }

    fn MINBPC(&self) -> isize { 1 }

    #[inline]
    fn byte_type(&self, p: *const c_char) -> ByteType {
        let idx = unsafe { *(p as *const u8) } as usize;
        T::types[idx]
    }

    #[inline]
    fn byte_to_ascii(&self, p: *const c_char) -> c_char {
        unsafe { *p }
    }

    #[inline]
    fn is_name_char(&self, p: *const c_char, n: isize) -> bool {
        unsafe {
            match n {
                2 => {
                    (namingBitmap[(((namePages
                        [(*(p as *const c_uchar).offset(0) as c_int >> 2 & 7) as usize]
                        as c_int)
                        << 3)
                        + ((*(p as *const c_uchar).offset(0) as c_int & 3) << 1)
                        + (*(p as *const c_uchar).offset(1) as c_int >> 5 & 1))
                        as usize]
                        & (1) << (*(p as *const c_uchar).offset(1) as c_int & 0x1f))
                        != 0
                }
                3 => {
                    (namingBitmap[(((namePages[(((*(p as *const c_uchar).offset(0) as c_int & 0xf)
                        << 4)
                        + (*(p as *const c_uchar).offset(1) as c_int >> 2 & 0xf))
                        as usize] as c_int)
                        << 3)
                        + ((*(p as *const c_uchar).offset(1) as c_int & 3) << 1)
                        + (*(p as *const c_uchar).offset(2) as c_int >> 5 & 1))
                        as usize]
                        & (1) << (*(p as *const c_uchar).offset(2) as c_int & 0x1f))
                        != 0
                }
                4 => false,
                _ => panic!("Unexpected byte length"),
            }
        }
    }
    #[inline]
    fn is_nmstrt_char(&self, p: *const c_char, n: isize) -> bool {
        unsafe {
            match n {
                2 => {
                    (namingBitmap[(((nmstrtPages
                        [(*(p as *const c_uchar).offset(0) as c_int >> 2 & 7) as usize]
                        as c_int)
                        << 3)
                        + ((*(p as *const c_uchar).offset(0) as c_int & 3) << 1)
                        + (*(p as *const c_uchar).offset(1) as c_int >> 5 & 1))
                        as usize]
                        & (1) << (*(p as *const c_uchar).offset(1) as c_int & 0x1f))
                        != 0
                }
                3 => {
                    (namingBitmap[(((nmstrtPages[(((*(p as *const c_uchar).offset(0) as c_int
                        & 0xf)
                        << 4)
                        + (*(p as *const c_uchar).offset(1) as c_int >> 2 & 0xf))
                        as usize] as c_int)
                        << 3)
                        + ((*(p as *const c_uchar).offset(1) as c_int & 3) << 1)
                        + (*(p as *const c_uchar).offset(2) as c_int >> 5 & 1))
                        as usize]
                        & (1) << (*(p as *const c_uchar).offset(2) as c_int & 0x1f))
                        != 0
                }
                4 => false,
                _ => panic!("Unexpected byte length"),
            }
        }
    }

    #[inline]
    fn is_invalid_char(&self, p: *const c_char, n: isize) -> bool {
        unsafe {
            match n {
                2 => {
                    (*(p as *const c_uchar) as c_int) < 0xc2
                        || *(p as *const c_uchar).offset(1) as c_int & 0x80 == 0
                        || *(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0
                }
                3 => {
                    *(p as *const c_uchar).offset(2) as c_int & 0x80 == 0
                        || (if *(p as *const c_uchar) as c_int == 0xef
                            && *(p as *const c_uchar).offset(1) as c_int == 0xbf
                        {
                            (*(p as *const c_uchar).offset(2) as c_int > 0xbd) as c_int
                        } else {
                            (*(p as *const c_uchar).offset(2) as c_int & 0xc0 == 0xc0) as c_int
                        }) != 0
                        || (if *(p as *const c_uchar) as c_int == 0xe0 {
                            ((*(p as *const c_uchar).offset(1) as c_int) < 0xa0
                                || *(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0)
                                as c_int
                        } else {
                            (*(p as *const c_uchar).offset(1) as c_int & 0x80 == 0
                                || (if *(p as *const c_uchar) as c_int == 0xed {
                                    (*(p as *const c_uchar).offset(1) as c_int > 0x9f) as c_int
                                } else {
                                    (*(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0)
                                        as c_int
                                }) != 0) as c_int
                        }) != 0
                }
                4 => {
                    *(p as *const c_uchar).offset(3) as c_int & 0x80 == 0
                        || *(p as *const c_uchar).offset(3) as c_int & 0xc0 == 0xc0
                        || *(p as *const c_uchar).offset(2) as c_int & 0x80 == 0
                        || *(p as *const c_uchar).offset(2) as c_int & 0xc0 == 0xc0
                        || (if *(p as *const c_uchar) as c_int == 0xf0 {
                            ((*(p as *const c_uchar).offset(1) as c_int) < 0x90
                                || *(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0)
                                as c_int
                        } else {
                            (*(p as *const c_uchar).offset(1) as c_int & 0x80 == 0
                                || (if *(p as *const c_uchar) as c_int == 0xf4 {
                                    (*(p as *const c_uchar).offset(1) as c_int > 0x8f) as c_int
                                } else {
                                    (*(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0)
                                        as c_int
                                }) != 0) as c_int
                        }) != 0
                }
                _ => panic!("Unexpected byte length"),
            }
        }
    }

    #[inline]
    fn is_name_char_minbpc(&self, _p: *const c_char) -> bool {
        false
    }

    #[inline]
    fn is_nmstrt_char_minbpc(&self, _p: *const c_char) -> bool {
        false
    }

    #[inline]
    fn char_matches(&self, p: *const c_char, c: c_char) -> bool {
        unsafe { *p == c }
    }

    #[inline]
    fn utf8Convert<'a: 'b, 'b>(
        &self,
        fromP: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut,
    ) -> XML_Convert_Result {
        let mut from = fromP.clone();
        let mut input_incomplete: bool = false_0 != 0;
        let mut output_exhausted: bool = false_0 != 0;
        /* Avoid copying partial characters (due to limited space). */
        let bytesAvailable = from.len();
        let bytesStorable = to.len();
        if bytesAvailable > bytesStorable {
            from = from.with_len(bytesStorable);
            output_exhausted = true_0 != 0
        }
        /* Avoid copying partial characters (from incomplete input). */
        let len_before = from.len();
        _INTERNAL_trim_to_complete_utf8_characters(&mut from);
        if from.len() < len_before {
            input_incomplete = true_0 != 0
        }
        to[..from.len()].copy_from_slice(&from);
        to.inc_start(from.len());
        *fromP = fromP.inc_start(from.len().try_into().unwrap());
        if output_exhausted {
            /* needs to go first */
            XML_Convert_Result::OUTPUT_EXHAUSTED
        } else if input_incomplete {
            XML_Convert_Result::INPUT_INCOMPLETE
        } else {
            XML_Convert_Result::COMPLETED
        }
    }

    #[inline]
    fn utf16Convert(
        &self,
        from: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        let mut res: XML_Convert_Result = XML_Convert_Result::COMPLETED;
        // let mut to: *mut c_ushort = *toP;
        // let mut from: *const c_char = *fromP;
        while !from.is_empty() && !to.is_empty() {
            match T::types[from[0] as c_uchar as usize] as c_int {
                5 => {
                    if (from.len() as c_long) < 2 {
                        res = XML_Convert_Result::INPUT_INCOMPLETE;
                        break;
                    } else {
                        to[0] = ((from[0] as c_int & 0x1f) << 6
                                 | from[1] as c_int & 0x3f)
                            as c_ushort;
                        to.inc_start(1);
                        from.inc_start(2);
                    }
                }
                6 => {
                    if (from.len() as c_long) < 3 {
                        res = XML_Convert_Result::INPUT_INCOMPLETE;
                        break;
                    } else {
                        to[0] = ((from[0] as c_int & 0xf) << 12
                                 | (from[1] as c_int & 0x3f) << 6
                                 | from[2] as c_int & 0x3f)
                            as c_ushort;
                        to.inc_start(1);
                        from.inc_start(3);
                    }
                }
                7 => {
                    let mut n: c_ulong = 0;
                    if (to.len() as c_long) < 2 {
                        res = XML_Convert_Result::OUTPUT_EXHAUSTED;
                        break;
                    } else if (from.len() as c_long) < 4 {
                        res = XML_Convert_Result::INPUT_INCOMPLETE;
                        break;
                    } else {
                        n = ((from[0] as c_int & 0x7) << 18
                             | (from[1] as c_int & 0x3f) << 12
                             | (from[2] as c_int & 0x3f) << 6
                             | from[3] as c_int & 0x3f) as c_ulong;
                        n = n.wrapping_sub(0x10000);
                        to[0] = (n >> 10 | 0xd800) as c_ushort;
                        to[1] = (n & 0x3ff | 0xdc00) as c_ushort;
                        to.inc_start(2);
                        *from = from.inc_start(4)
                    }
                }
                _ => {
                    to[0] = from[0] as u16;
                    *from = from.inc_start(1);
                    to.inc_start(1);
                }
            }
        }
        if !from.is_empty() {
            res = XML_Convert_Result::OUTPUT_EXHAUSTED
        }
        res
    }
}

struct Latin1EncodingImpl<T: NormalEncodingTable>(std::marker::PhantomData<T>);

type Latin1Encoding = Latin1EncodingImpl<Latin1EncodingTable>;
type Latin1EncodingNS = Latin1EncodingImpl<Latin1EncodingTableNS>;

impl<T: NormalEncodingTable> XmlEncodingImpl for Latin1EncodingImpl<T> {
    fn isUtf8(&self) -> bool { false }
    fn isUtf16(&self) -> bool { false }

    fn MINBPC(&self) -> isize { 1 }

    #[inline]
    fn byte_type(&self, p: *const c_char) -> ByteType {
        let idx = unsafe { *(p as *const u8) } as usize;
        T::types[idx]
    }

    #[inline]
    fn byte_to_ascii(&self, p: *const c_char) -> c_char {
        unsafe { *p }
    }

    #[inline]
    fn is_name_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }
    #[inline]
    fn is_nmstrt_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }

    #[inline]
    fn is_invalid_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }

    #[inline]
    fn is_name_char_minbpc(&self, _p: *const c_char) -> bool {
        false
    }

    #[inline]
    fn is_nmstrt_char_minbpc(&self, _p: *const c_char) -> bool {
        false
    }

    #[inline]
    fn char_matches(&self, p: *const c_char, c: c_char) -> bool {
        unsafe { *p == c }
    }

    #[inline]
    fn utf8Convert<'a: 'b, 'b>(
        &self,
        from: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut,
    ) -> XML_Convert_Result {
        loop {
            let mut c: c_uchar = 0;
            if from.is_empty() {
                return XML_Convert_Result::COMPLETED;
            }
            c = from[0] as c_uchar;
            if c as c_int & 0x80 != 0 {
                if (to.len() as c_long) < 2 {
                    return XML_Convert_Result::OUTPUT_EXHAUSTED;
                }
                to[0] = (c as c_int >> 6 | UTF8_cval2 as c_int) as c_char;
                to.inc_start(1);
                to[0] = (c as c_int & 0x3f | 0x80) as c_char;
                to.inc_start(1);
                *from = from.inc_start(1);
            } else {
                if to.is_empty() {
                    return XML_Convert_Result::OUTPUT_EXHAUSTED;
                }
                to[0] = from[0];
                to.inc_start(1);
                *from = from.inc_start(1);
            }
        }
    }

    #[inline]
    fn utf16Convert(
        &self,
        from_buf: &mut ExpatBufRef,
        to_buf: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        latin1_toUtf16(from_buf, to_buf)
    }
}

struct AsciiEncodingImpl<T: NormalEncodingTable>(std::marker::PhantomData<T>);

type AsciiEncoding = AsciiEncodingImpl<AsciiEncodingTable>;
type AsciiEncodingNS = AsciiEncodingImpl<AsciiEncodingTableNS>;

impl<T: NormalEncodingTable> XmlEncodingImpl for AsciiEncodingImpl<T> {
    fn isUtf8(&self) -> bool { true }
    fn isUtf16(&self) -> bool { false }

    fn MINBPC(&self) -> isize { 1 }

    #[inline]
    fn byte_type(&self, p: *const c_char) -> ByteType {
        let idx = unsafe { *(p as *const u8) } as usize;
        T::types[idx]
    }

    #[inline]
    fn byte_to_ascii(&self, p: *const c_char) -> c_char {
        unsafe { *p }
    }

    #[inline]
    fn is_name_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }
    #[inline]
    fn is_nmstrt_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }

    #[inline]
    fn is_invalid_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }

    #[inline]
    fn is_name_char_minbpc(&self, _p: *const c_char) -> bool {
        false
    }

    #[inline]
    fn is_nmstrt_char_minbpc(&self, _p: *const c_char) -> bool {
        false
    }

    #[inline]
    fn char_matches(&self, p: *const c_char, c: c_char) -> bool {
        unsafe { *p == c }
    }

    #[inline]
    fn utf8Convert<'a: 'b, 'b>(
        &self,
        from: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut,
    ) -> XML_Convert_Result {
        while !from.is_empty() && !to.is_empty() {
            to[0] = from[0];
            to.inc_start(1);
            *from = from.inc_start(1);
        }
        if to.is_empty() && !from.is_empty() {
            XML_Convert_Result::OUTPUT_EXHAUSTED
        } else {
            XML_Convert_Result::COMPLETED
        }
    }

    #[inline]
    fn utf16Convert(
        &self,
        from_buf: &mut ExpatBufRef,
        to_buf: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        latin1_toUtf16(from_buf, to_buf)
    }
}

struct Little2EncodingImpl<T: NormalEncodingTable>(std::marker::PhantomData<T>);

type Little2Encoding = Little2EncodingImpl<Latin1EncodingTable>;
type Little2EncodingNS = Little2EncodingImpl<Latin1EncodingTableNS>;
type InternalLittle2Encoding = Little2EncodingImpl<InternalLatin1EncodingTable>;
type InternalLittle2EncodingNS = Little2EncodingImpl<InternalLatin1EncodingTableNS>;

impl<T: NormalEncodingTable> XmlEncodingImpl for Little2EncodingImpl<T> {
    fn isUtf8(&self) -> bool { false }

    #[cfg(target_endian = "little")]
    fn isUtf16(&self) -> bool { true }

    #[cfg(not(target_endian = "little"))]
    fn isUtf16(&self) -> bool { false }

    fn MINBPC(&self) -> isize { 2 }

    #[inline]
    fn byte_type(&self, p: *const c_char) -> ByteType {
        let bytes = unsafe { (*p, *p.offset(1)) };
        if bytes.1 == 0 {
            T::types[bytes.0 as u8 as usize]
        } else {
            unicode_byte_type(bytes.1, bytes.0)
        }
    }

    #[inline]
    fn byte_to_ascii(&self, p: *const c_char) -> c_char {
        let bytes = unsafe { (*p, *p.offset(1)) };
        if bytes.1 == 0 {
            bytes.0
        } else {
            -1
        }
    }

    #[inline]
    fn is_name_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }
    #[inline]
    fn is_nmstrt_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }

    #[inline]
    fn is_invalid_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }

    #[inline]
    fn is_name_char_minbpc(&self, p: *const c_char) -> bool {
        unsafe { UCS2_GET_NAMING!(namePages, *p.offset(1), *p) != 0 }
    }

    #[inline]
    fn is_nmstrt_char_minbpc(&self, p: *const c_char) -> bool {
        unsafe { UCS2_GET_NAMING!(nmstrtPages, *p.offset(1), *p) != 0 }
    }

    #[inline]
    fn char_matches(&self, p: *const c_char, c: c_char) -> bool {
        unsafe { *p.offset(1) == 0 && *p == c }
    }

    #[inline]
    fn utf8Convert<'a: 'b, 'b>(
        &self,
        fromP: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut,
    ) -> XML_Convert_Result {
        let mut from = *fromP;
        from = from.with_len(((from.len() as c_long >> 1) << 1) as usize);
        while !from.is_empty() {
            let mut plane: c_int = 0;
            let mut lo2: c_uchar = 0;
            let mut lo: c_uchar = from[0] as c_uchar;
            let mut hi: c_uchar = from[1] as c_uchar;
            match hi as c_int {
                0..=7 => {
                    if hi as c_int == 0 && (lo as c_int) < 0x80 {
                        if to.is_empty() {
                            *fromP = from;
                            return XML_Convert_Result::OUTPUT_EXHAUSTED;
                        }
                        to[0] = lo as c_char;
                        to.inc_start(1);
                    } else {
                        if to.len() < 2 {
                            *fromP = from;
                            return XML_Convert_Result::OUTPUT_EXHAUSTED;
                        }
                        to[0] = (lo as c_int >> 6 | (hi as c_int) << 2 | UTF8_cval2 as c_int) as c_char;
                        to.inc_start(1);
                        to[0] = (lo as c_int & 0x3f | 0x80) as c_char;
                        to.inc_start(1);
                    }
                }
                216..=219 => {
                    if (to.len() as c_long) < 4 {
                        *fromP = from;
                        return XML_Convert_Result::OUTPUT_EXHAUSTED;
                    }
                    if (from.len() as c_long) < 4 {
                        *fromP = from;
                        return XML_Convert_Result::INPUT_INCOMPLETE;
                    }
                    plane = ((hi as c_int & 0x3) << 2 | lo as c_int >> 6 & 0x3) + 1;
                    to[0] = (plane >> 2 | UTF8_cval4 as c_int) as c_char;
                    to.inc_start(1);
                    to[0] = (lo as c_int >> 2 & 0xf | (plane & 0x3) << 4 | 0x80) as c_char;
                    to.inc_start(1);
                    from = from.inc_start(2);
                    lo2 = from[0] as c_uchar;
                    to[0] = ((lo as c_int & 0x3) << 4
                             | (from[1] as c_uchar as c_int & 0x3) << 2
                             | lo2 as c_int >> 6
                             | 0x80) as c_char;
                    to.inc_start(1);
                    to[0] = (lo2 as c_int & 0x3f | 0x80) as c_char;
                    to.inc_start(1);
                }
                _ => {
                    if (to.len() as c_long) < 3 {
                        *fromP = from;
                        return XML_Convert_Result::OUTPUT_EXHAUSTED;
                    }
                    to[0] = (hi as c_int >> 4 | UTF8_cval3 as c_int) as c_char;
                    to.inc_start(1);
                    to[0] = ((hi as c_int & 0xf) << 2 | lo as c_int >> 6 | 0x80) as c_char;
                    to.inc_start(1);
                    to[0] = (lo as c_int & 0x3f | 0x80) as c_char;
                    to.inc_start(1);
                }
            }
            from = from.inc_start(2)
        }
        *fromP = from;
        if !from.is_empty() {
            XML_Convert_Result::INPUT_INCOMPLETE
        } else {
            XML_Convert_Result::COMPLETED
        }
    }

    #[inline]
    fn utf16Convert(
        &self,
        from: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        let mut res: XML_Convert_Result = XML_Convert_Result::COMPLETED;
        *from = from.with_len(((from.len() as c_long >> 1) << 1) as usize);
        if from.len() as c_long > ((to.len() as c_long) << 1)
            && from[from.len()-1] as c_uchar as c_int & 0xf8 == 0xd8
        {
            *from = from.dec_end(2);
            res = XML_Convert_Result::INPUT_INCOMPLETE
        }
        while !from.is_empty() && !to.is_empty() {
            to[0] = ((from[1] as c_uchar as c_int) << 8
                     | from[0] as c_uchar as c_int) as c_ushort;
            to.inc_start(1);
            *from = from.inc_start(2);
        }
        if to.is_empty() && !from.is_empty() {
            XML_Convert_Result::OUTPUT_EXHAUSTED
        } else {
            res
        }
    }
}

struct Big2EncodingImpl<T: NormalEncodingTable>(std::marker::PhantomData<T>);

type Big2Encoding = Big2EncodingImpl<Latin1EncodingTable>;
type Big2EncodingNS = Big2EncodingImpl<Latin1EncodingTableNS>;
type InternalBig2Encoding = Big2EncodingImpl<InternalLatin1EncodingTable>;
type InternalBig2EncodingNS = Big2EncodingImpl<InternalLatin1EncodingTableNS>;

impl<T: NormalEncodingTable> XmlEncodingImpl for Big2EncodingImpl<T> {
    fn MINBPC(&self) -> isize { 2 }
    fn isUtf8(&self) -> bool { false }

    #[cfg(target_endian = "big")]
    fn isUtf16(&self) -> bool { true }

    #[cfg(not(target_endian = "big"))]
    fn isUtf16(&self) -> bool { false }

    #[inline]
    fn byte_type(&self, p: *const c_char) -> ByteType {
        let bytes = unsafe { (*p, *p.offset(1)) };
        if bytes.0 == 0 {
            T::types[bytes.1 as usize]
        } else {
            unicode_byte_type(bytes.0, bytes.1)
        }
    }

    #[inline]
    fn byte_to_ascii(&self, p: *const c_char) -> c_char {
        let bytes = unsafe { (*p, *p.offset(1)) };
        if bytes.0 == 0 {
            bytes.1
        } else {
            -1
        }
    }

    #[inline]
    fn is_name_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }
    #[inline]
    fn is_nmstrt_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }

    #[inline]
    fn is_invalid_char(&self, _p: *const c_char, _n: isize) -> bool {
        false
    }

    #[inline]
    fn is_name_char_minbpc(&self, p: *const c_char) -> bool {
        unsafe { UCS2_GET_NAMING!(namePages, *p, *p.offset(1)) != 0 }
    }

    #[inline]
    fn is_nmstrt_char_minbpc(&self, p: *const c_char) -> bool {
        unsafe { UCS2_GET_NAMING!(nmstrtPages, *p, *p.offset(1)) != 0 }
    }

    #[inline]
    fn char_matches(&self, p: *const c_char, c: c_char) -> bool {
        unsafe { *p == 0 && *p.offset(1) == c }
    }

    #[inline]
    fn utf8Convert<'a: 'b, 'b>(
        &self,
        fromP: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut,
    ) -> XML_Convert_Result {
        let mut from = *fromP;
        from = from.with_len(((from.len() as c_long >> 1) << 1) as usize);
        while !from.is_empty() {
            let mut plane: c_int = 0;
            let mut lo2: c_uchar = 0;
            let mut lo: c_uchar = from[1] as c_uchar;
            let mut hi: c_uchar = from[0] as c_uchar;
            match hi as c_int {
                0..=7 => {
                    if hi as c_int == 0 && (lo as c_int) < 0x80 {
                        if to.is_empty() {
                            *fromP = from;
                            return XML_Convert_Result::OUTPUT_EXHAUSTED;
                        }
                        to[0] = lo as c_char;
                        to.inc_start(1);
                    } else {
                        if to.len() < 2 {
                            *fromP = from;
                            return XML_Convert_Result::OUTPUT_EXHAUSTED;
                        }
                        to[0] = (lo as c_int >> 6 | (hi as c_int) << 2 | UTF8_cval2 as c_int) as c_char;
                        to.inc_start(1);
                        to[0] = (lo as c_int & 0x3f | 0x80) as c_char;
                        to.inc_start(1);
                    }
                }
                216..=219 => {
                    if (to.len() as c_long) < 4 {
                        *fromP = from;
                        return XML_Convert_Result::OUTPUT_EXHAUSTED;
                    }
                    if (from.len() as c_long) < 4 {
                        *fromP = from;
                        return XML_Convert_Result::INPUT_INCOMPLETE;
                    }
                    plane = ((hi as c_int & 0x3) << 2 | lo as c_int >> 6 & 0x3) + 1;
                    to[0] = (plane >> 2 | UTF8_cval4 as c_int) as c_char;
                    to.inc_start(1);
                    to[0] = (lo as c_int >> 2 & 0xf | (plane & 0x3) << 4 | 0x80) as c_char;
                    to.inc_start(1);
                    from = from.inc_start(2);
                    lo2 = from[1] as c_uchar;
                    to[0] = ((lo as c_int & 0x3) << 4
                             | (from[0] as c_uchar as c_int & 0x3) << 2
                             | lo2 as c_int >> 6
                             | 0x80) as c_char;
                    to.inc_start(1);
                    to[0] = (lo2 as c_int & 0x3f | 0x80) as c_char;
                    to.inc_start(1);
                }
                _ => {
                    if (to.len() as c_long) < 3 {
                        *fromP = from;
                        return XML_Convert_Result::OUTPUT_EXHAUSTED;
                    }
                    to[0] = (hi as c_int >> 4 | UTF8_cval3 as c_int) as c_char;
                    to.inc_start(1);
                    to[0] = ((hi as c_int & 0xf) << 2 | lo as c_int >> 6 | 0x80) as c_char;
                    to.inc_start(1);
                    to[0] = (lo as c_int & 0x3f | 0x80) as c_char;
                    to.inc_start(1);
                }
            }
            from = from.inc_start(2)
        }
        *fromP = from;
        if !from.is_empty() {
            XML_Convert_Result::INPUT_INCOMPLETE
        } else {
            XML_Convert_Result::COMPLETED
        }
    }

    #[inline]
    fn utf16Convert(
        &self,
        from: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        let mut res: XML_Convert_Result = XML_Convert_Result::COMPLETED;
        *from = from.with_len(((from.len() as c_long >> 1) << 1) as usize);
        if from.len() as c_long > ((to.len() as c_long) << 1)
            && from[from.len()-2] as c_uchar as c_int & 0xf8 == 0xd8
        {
            *from = from.dec_end(2);
            res = XML_Convert_Result::INPUT_INCOMPLETE
        }
        while !from.is_empty() && !to.is_empty() {
            to[0] = ((from[0] as c_uchar as c_int) << 8
                     | from[1] as c_uchar as c_int) as c_ushort;
            to.inc_start(1);
            *from = from.inc_start(2);
        }
        if to.is_empty() && !from.is_empty() {
            XML_Convert_Result::OUTPUT_EXHAUSTED
        } else {
            res
        }
    }
}

struct Utf8EncodingTable;

impl NormalEncodingTable for Utf8EncodingTable {
    const types: [ByteType; 256] = [
        // asciitab.h
        /* 0x00 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x04 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x08 */ ByteType::NONXML, ByteType::S, ByteType::LF, ByteType::NONXML,
        /* 0x0C */ ByteType::NONXML, ByteType::CR, ByteType::NONXML, ByteType::NONXML,
        /* 0x10 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x14 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x18 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x1C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x20 */ ByteType::S, ByteType::EXCL, ByteType::QUOT, ByteType::NUM,
        /* 0x24 */ ByteType::OTHER, ByteType::PERCNT, ByteType::AMP, ByteType::APOS,
        /* 0x28 */ ByteType::LPAR, ByteType::RPAR, ByteType::AST, ByteType::PLUS,
        /* 0x2C */ ByteType::COMMA, ByteType::MINUS, ByteType::NAME, ByteType::SOL,
        /* 0x30 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x34 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x38 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::NMSTRT, ByteType::SEMI,
        /* 0x3C */ ByteType::LT, ByteType::EQUALS, ByteType::GT, ByteType::QUEST,
        /* 0x40 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x44 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x48 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x4C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x50 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x54 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x58 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::LSQB,
        /* 0x5C */ ByteType::OTHER, ByteType::RSQB, ByteType::OTHER, ByteType::NMSTRT,
        /* 0x60 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x64 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x68 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x6C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x70 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x74 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x78 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0x7C */ ByteType::VERBAR, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,

        // utf8tab.h
        /* 0x80 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x84 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x88 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x8C */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x90 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x94 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x98 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x9C */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA0 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA4 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA8 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xAC */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB0 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB4 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB8 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xBC */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xC0 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xC4 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xC8 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xCC */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD0 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD4 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD8 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xDC */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xE0 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xE4 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xE8 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xEC */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xF0 */ ByteType::LEAD4, ByteType::LEAD4, ByteType::LEAD4, ByteType::LEAD4,
        /* 0xF4 */ ByteType::LEAD4, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xF8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xFC */ ByteType::NONXML, ByteType::NONXML, ByteType::MALFORM, ByteType::MALFORM,
    ];
}

struct Utf8EncodingTableNS;

impl NormalEncodingTable for Utf8EncodingTableNS {
    const types: [ByteType; 256] = [
        // asciitab.h
        /* 0x00 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x04 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x08 */ ByteType::NONXML, ByteType::S, ByteType::LF, ByteType::NONXML,
        /* 0x0C */ ByteType::NONXML, ByteType::CR, ByteType::NONXML, ByteType::NONXML,
        /* 0x10 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x14 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x18 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x1C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x20 */ ByteType::S, ByteType::EXCL, ByteType::QUOT, ByteType::NUM,
        /* 0x24 */ ByteType::OTHER, ByteType::PERCNT, ByteType::AMP, ByteType::APOS,
        /* 0x28 */ ByteType::LPAR, ByteType::RPAR, ByteType::AST, ByteType::PLUS,
        /* 0x2C */ ByteType::COMMA, ByteType::MINUS, ByteType::NAME, ByteType::SOL,
        /* 0x30 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x34 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x38 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::COLON, ByteType::SEMI,
        /* 0x3C */ ByteType::LT, ByteType::EQUALS, ByteType::GT, ByteType::QUEST,
        /* 0x40 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x44 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x48 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x4C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x50 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x54 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x58 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::LSQB,
        /* 0x5C */ ByteType::OTHER, ByteType::RSQB, ByteType::OTHER, ByteType::NMSTRT,
        /* 0x60 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x64 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x68 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x6C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x70 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x74 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x78 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0x7C */ ByteType::VERBAR, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,

        // utf8tab.h
        /* 0x80 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x84 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x88 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x8C */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x90 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x94 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x98 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x9C */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA0 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA4 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA8 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xAC */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB0 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB4 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB8 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xBC */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xC0 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xC4 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xC8 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xCC */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD0 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD4 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD8 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xDC */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xE0 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xE4 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xE8 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xEC */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xF0 */ ByteType::LEAD4, ByteType::LEAD4, ByteType::LEAD4, ByteType::LEAD4,
        /* 0xF4 */ ByteType::LEAD4, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xF8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xFC */ ByteType::NONXML, ByteType::NONXML, ByteType::MALFORM, ByteType::MALFORM,
    ];
}

struct InternalUtf8EncodingTable;

impl NormalEncodingTable for InternalUtf8EncodingTable {
    const types: [ByteType; 256] = [
        // iasciitab.h
        /* Like asciitab.h, except that 0xD has code ByteType::S rather than ByteType::CR */
        /* 0x00 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x04 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x08 */ ByteType::NONXML, ByteType::S, ByteType::LF, ByteType::NONXML,
        /* 0x0C */ ByteType::NONXML, ByteType::S, ByteType::NONXML, ByteType::NONXML,
        /* 0x10 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x14 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x18 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x1C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x20 */ ByteType::S, ByteType::EXCL, ByteType::QUOT, ByteType::NUM,
        /* 0x24 */ ByteType::OTHER, ByteType::PERCNT, ByteType::AMP, ByteType::APOS,
        /* 0x28 */ ByteType::LPAR, ByteType::RPAR, ByteType::AST, ByteType::PLUS,
        /* 0x2C */ ByteType::COMMA, ByteType::MINUS, ByteType::NAME, ByteType::SOL,
        /* 0x30 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x34 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x38 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::NMSTRT, ByteType::SEMI,
        /* 0x3C */ ByteType::LT, ByteType::EQUALS, ByteType::GT, ByteType::QUEST,
        /* 0x40 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x44 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x48 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x4C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x50 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x54 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x58 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::LSQB,
        /* 0x5C */ ByteType::OTHER, ByteType::RSQB, ByteType::OTHER, ByteType::NMSTRT,
        /* 0x60 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x64 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x68 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x6C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x70 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x74 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x78 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0x7C */ ByteType::VERBAR, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,

        // utf8tab.h
        /* 0x80 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x84 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x88 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x8C */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x90 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x94 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x98 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x9C */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA0 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA4 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA8 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xAC */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB0 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB4 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB8 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xBC */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xC0 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xC4 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xC8 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xCC */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD0 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD4 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD8 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xDC */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xE0 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xE4 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xE8 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xEC */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xF0 */ ByteType::LEAD4, ByteType::LEAD4, ByteType::LEAD4, ByteType::LEAD4,
        /* 0xF4 */ ByteType::LEAD4, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xF8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xFC */ ByteType::NONXML, ByteType::NONXML, ByteType::MALFORM, ByteType::MALFORM,
    ];
}

struct InternalUtf8EncodingTableNS;

impl NormalEncodingTable for InternalUtf8EncodingTableNS {
    const types: [ByteType; 256] = [
        // iasciitab.h
        /* Like asciitab.h, except that 0xD has code ByteType::S rather than ByteType::CR */
        /* 0x00 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x04 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x08 */ ByteType::NONXML, ByteType::S, ByteType::LF, ByteType::NONXML,
        /* 0x0C */ ByteType::NONXML, ByteType::S, ByteType::NONXML, ByteType::NONXML,
        /* 0x10 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x14 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x18 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x1C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x20 */ ByteType::S, ByteType::EXCL, ByteType::QUOT, ByteType::NUM,
        /* 0x24 */ ByteType::OTHER, ByteType::PERCNT, ByteType::AMP, ByteType::APOS,
        /* 0x28 */ ByteType::LPAR, ByteType::RPAR, ByteType::AST, ByteType::PLUS,
        /* 0x2C */ ByteType::COMMA, ByteType::MINUS, ByteType::NAME, ByteType::SOL,
        /* 0x30 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x34 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x38 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::COLON, ByteType::SEMI,
        /* 0x3C */ ByteType::LT, ByteType::EQUALS, ByteType::GT, ByteType::QUEST,
        /* 0x40 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x44 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x48 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x4C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x50 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x54 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x58 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::LSQB,
        /* 0x5C */ ByteType::OTHER, ByteType::RSQB, ByteType::OTHER, ByteType::NMSTRT,
        /* 0x60 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x64 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x68 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x6C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x70 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x74 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x78 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0x7C */ ByteType::VERBAR, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,

        // utf8tab.h
        /* 0x80 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x84 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x88 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x8C */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x90 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x94 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x98 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0x9C */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA0 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA4 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xA8 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xAC */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB0 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB4 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xB8 */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xBC */ ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL, ByteType::TRAIL,
        /* 0xC0 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xC4 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xC8 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xCC */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD0 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD4 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xD8 */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xDC */ ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2, ByteType::LEAD2,
        /* 0xE0 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xE4 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xE8 */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xEC */ ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3, ByteType::LEAD3,
        /* 0xF0 */ ByteType::LEAD4, ByteType::LEAD4, ByteType::LEAD4, ByteType::LEAD4,
        /* 0xF4 */ ByteType::LEAD4, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xF8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xFC */ ByteType::NONXML, ByteType::NONXML, ByteType::MALFORM, ByteType::MALFORM,
    ];
}

struct Latin1EncodingTable;

impl NormalEncodingTable for Latin1EncodingTable {
    const types: [ByteType; 256] = [
        // asciitab.h
        /* 0x00 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x04 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x08 */ ByteType::NONXML, ByteType::S, ByteType::LF, ByteType::NONXML,
        /* 0x0C */ ByteType::NONXML, ByteType::CR, ByteType::NONXML, ByteType::NONXML,
        /* 0x10 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x14 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x18 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x1C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x20 */ ByteType::S, ByteType::EXCL, ByteType::QUOT, ByteType::NUM,
        /* 0x24 */ ByteType::OTHER, ByteType::PERCNT, ByteType::AMP, ByteType::APOS,
        /* 0x28 */ ByteType::LPAR, ByteType::RPAR, ByteType::AST, ByteType::PLUS,
        /* 0x2C */ ByteType::COMMA, ByteType::MINUS, ByteType::NAME, ByteType::SOL,
        /* 0x30 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x34 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x38 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::NMSTRT, ByteType::SEMI,
        /* 0x3C */ ByteType::LT, ByteType::EQUALS, ByteType::GT, ByteType::QUEST,
        /* 0x40 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x44 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x48 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x4C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x50 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x54 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x58 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::LSQB,
        /* 0x5C */ ByteType::OTHER, ByteType::RSQB, ByteType::OTHER, ByteType::NMSTRT,
        /* 0x60 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x64 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x68 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x6C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x70 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x74 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x78 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0x7C */ ByteType::VERBAR, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,

        // latin1tab.h
        /* 0x80 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x84 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x88 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x8C */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x90 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x94 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x98 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x9C */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA0 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA4 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA8 */ ByteType::OTHER, ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xAC */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xB0 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xB4 */ ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER, ByteType::NAME,
        /* 0xB8 */ ByteType::OTHER, ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xBC */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xC0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xC4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xC8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xCC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xD0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xD4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xD8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xDC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xEC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xF0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xF4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xF8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xFC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
    ];
}

struct Latin1EncodingTableNS;

impl NormalEncodingTable for Latin1EncodingTableNS {
    const types: [ByteType; 256] = [
        // asciitab.h
        /* 0x00 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x04 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x08 */ ByteType::NONXML, ByteType::S, ByteType::LF, ByteType::NONXML,
        /* 0x0C */ ByteType::NONXML, ByteType::CR, ByteType::NONXML, ByteType::NONXML,
        /* 0x10 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x14 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x18 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x1C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x20 */ ByteType::S, ByteType::EXCL, ByteType::QUOT, ByteType::NUM,
        /* 0x24 */ ByteType::OTHER, ByteType::PERCNT, ByteType::AMP, ByteType::APOS,
        /* 0x28 */ ByteType::LPAR, ByteType::RPAR, ByteType::AST, ByteType::PLUS,
        /* 0x2C */ ByteType::COMMA, ByteType::MINUS, ByteType::NAME, ByteType::SOL,
        /* 0x30 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x34 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x38 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::COLON, ByteType::SEMI,
        /* 0x3C */ ByteType::LT, ByteType::EQUALS, ByteType::GT, ByteType::QUEST,
        /* 0x40 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x44 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x48 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x4C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x50 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x54 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x58 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::LSQB,
        /* 0x5C */ ByteType::OTHER, ByteType::RSQB, ByteType::OTHER, ByteType::NMSTRT,
        /* 0x60 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x64 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x68 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x6C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x70 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x74 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x78 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0x7C */ ByteType::VERBAR, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,

        // latin1tab.h
        /* 0x80 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x84 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x88 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x8C */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x90 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x94 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x98 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x9C */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA0 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA4 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA8 */ ByteType::OTHER, ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xAC */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xB0 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xB4 */ ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER, ByteType::NAME,
        /* 0xB8 */ ByteType::OTHER, ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xBC */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xC0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xC4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xC8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xCC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xD0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xD4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xD8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xDC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xEC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xF0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xF4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xF8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xFC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
    ];
}

struct InternalLatin1EncodingTable;

impl NormalEncodingTable for InternalLatin1EncodingTable {
    const types: [ByteType; 256] = [
        // iasciitab.h
        /* Like asciitab.h, except that 0xD has code ByteType::S rather than ByteType::CR */
        /* 0x00 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x04 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x08 */ ByteType::NONXML, ByteType::S, ByteType::LF, ByteType::NONXML,
        /* 0x0C */ ByteType::NONXML, ByteType::S, ByteType::NONXML, ByteType::NONXML,
        /* 0x10 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x14 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x18 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x1C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x20 */ ByteType::S, ByteType::EXCL, ByteType::QUOT, ByteType::NUM,
        /* 0x24 */ ByteType::OTHER, ByteType::PERCNT, ByteType::AMP, ByteType::APOS,
        /* 0x28 */ ByteType::LPAR, ByteType::RPAR, ByteType::AST, ByteType::PLUS,
        /* 0x2C */ ByteType::COMMA, ByteType::MINUS, ByteType::NAME, ByteType::SOL,
        /* 0x30 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x34 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x38 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::NMSTRT, ByteType::SEMI,
        /* 0x3C */ ByteType::LT, ByteType::EQUALS, ByteType::GT, ByteType::QUEST,
        /* 0x40 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x44 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x48 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x4C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x50 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x54 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x58 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::LSQB,
        /* 0x5C */ ByteType::OTHER, ByteType::RSQB, ByteType::OTHER, ByteType::NMSTRT,
        /* 0x60 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x64 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x68 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x6C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x70 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x74 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x78 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0x7C */ ByteType::VERBAR, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,

        // latin1tab.h
        /* 0x80 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x84 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x88 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x8C */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x90 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x94 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x98 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x9C */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA0 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA4 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA8 */ ByteType::OTHER, ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xAC */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xB0 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xB4 */ ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER, ByteType::NAME,
        /* 0xB8 */ ByteType::OTHER, ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xBC */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xC0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xC4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xC8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xCC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xD0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xD4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xD8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xDC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xEC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xF0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xF4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xF8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xFC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
    ];
}

struct InternalLatin1EncodingTableNS;

impl NormalEncodingTable for InternalLatin1EncodingTableNS {
    const types: [ByteType; 256] = [
        // iasciitab.h
        /* Like asciitab.h, except that 0xD has code ByteType::S rather than ByteType::CR */
        /* 0x00 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x04 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x08 */ ByteType::NONXML, ByteType::S, ByteType::LF, ByteType::NONXML,
        /* 0x0C */ ByteType::NONXML, ByteType::S, ByteType::NONXML, ByteType::NONXML,
        /* 0x10 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x14 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x18 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x1C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x20 */ ByteType::S, ByteType::EXCL, ByteType::QUOT, ByteType::NUM,
        /* 0x24 */ ByteType::OTHER, ByteType::PERCNT, ByteType::AMP, ByteType::APOS,
        /* 0x28 */ ByteType::LPAR, ByteType::RPAR, ByteType::AST, ByteType::PLUS,
        /* 0x2C */ ByteType::COMMA, ByteType::MINUS, ByteType::NAME, ByteType::SOL,
        /* 0x30 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x34 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x38 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::COLON, ByteType::SEMI,
        /* 0x3C */ ByteType::LT, ByteType::EQUALS, ByteType::GT, ByteType::QUEST,
        /* 0x40 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x44 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x48 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x4C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x50 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x54 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x58 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::LSQB,
        /* 0x5C */ ByteType::OTHER, ByteType::RSQB, ByteType::OTHER, ByteType::NMSTRT,
        /* 0x60 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x64 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x68 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x6C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x70 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x74 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x78 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0x7C */ ByteType::VERBAR, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,

        // latin1tab.h
        /* 0x80 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x84 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x88 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x8C */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x90 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x94 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x98 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0x9C */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA0 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA4 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xA8 */ ByteType::OTHER, ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xAC */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xB0 */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xB4 */ ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER, ByteType::NAME,
        /* 0xB8 */ ByteType::OTHER, ByteType::OTHER, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xBC */ ByteType::OTHER, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,
        /* 0xC0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xC4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xC8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xCC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xD0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xD4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xD8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xDC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xE8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xEC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xF0 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xF4 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0xF8 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0xFC */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
    ];
}

struct AsciiEncodingTable;

impl NormalEncodingTable for AsciiEncodingTable {
    const types: [ByteType; 256] = [
        // asciitab.h
        /* 0x00 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x04 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x08 */ ByteType::NONXML, ByteType::S, ByteType::LF, ByteType::NONXML,
        /* 0x0C */ ByteType::NONXML, ByteType::CR, ByteType::NONXML, ByteType::NONXML,
        /* 0x10 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x14 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x18 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x1C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x20 */ ByteType::S, ByteType::EXCL, ByteType::QUOT, ByteType::NUM,
        /* 0x24 */ ByteType::OTHER, ByteType::PERCNT, ByteType::AMP, ByteType::APOS,
        /* 0x28 */ ByteType::LPAR, ByteType::RPAR, ByteType::AST, ByteType::PLUS,
        /* 0x2C */ ByteType::COMMA, ByteType::MINUS, ByteType::NAME, ByteType::SOL,
        /* 0x30 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x34 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x38 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::NMSTRT, ByteType::SEMI,
        /* 0x3C */ ByteType::LT, ByteType::EQUALS, ByteType::GT, ByteType::QUEST,
        /* 0x40 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x44 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x48 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x4C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x50 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x54 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x58 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::LSQB,
        /* 0x5C */ ByteType::OTHER, ByteType::RSQB, ByteType::OTHER, ByteType::NMSTRT,
        /* 0x60 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x64 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x68 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x6C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x70 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x74 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x78 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0x7C */ ByteType::VERBAR, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,

        /* 0x80 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x84 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x88 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x8C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x90 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x94 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x98 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x9C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xA0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xA4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xA8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xAC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xB0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xB4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xB8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xBC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xC0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xC4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xC8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xCC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xD0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xD4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xD8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xDC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xE0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xE4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xE8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xEC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xF0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xF4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xF8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xFC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
    ];
}

struct AsciiEncodingTableNS;

impl NormalEncodingTable for AsciiEncodingTableNS {
    const types: [ByteType; 256] = [
        // asciitab.h
        /* 0x00 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x04 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x08 */ ByteType::NONXML, ByteType::S, ByteType::LF, ByteType::NONXML,
        /* 0x0C */ ByteType::NONXML, ByteType::CR, ByteType::NONXML, ByteType::NONXML,
        /* 0x10 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x14 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x18 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x1C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x20 */ ByteType::S, ByteType::EXCL, ByteType::QUOT, ByteType::NUM,
        /* 0x24 */ ByteType::OTHER, ByteType::PERCNT, ByteType::AMP, ByteType::APOS,
        /* 0x28 */ ByteType::LPAR, ByteType::RPAR, ByteType::AST, ByteType::PLUS,
        /* 0x2C */ ByteType::COMMA, ByteType::MINUS, ByteType::NAME, ByteType::SOL,
        /* 0x30 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x34 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT, ByteType::DIGIT,
        /* 0x38 */ ByteType::DIGIT, ByteType::DIGIT, ByteType::COLON, ByteType::SEMI,
        /* 0x3C */ ByteType::LT, ByteType::EQUALS, ByteType::GT, ByteType::QUEST,
        /* 0x40 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x44 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x48 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x4C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x50 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x54 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x58 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::LSQB,
        /* 0x5C */ ByteType::OTHER, ByteType::RSQB, ByteType::OTHER, ByteType::NMSTRT,
        /* 0x60 */ ByteType::OTHER, ByteType::HEX, ByteType::HEX, ByteType::HEX,
        /* 0x64 */ ByteType::HEX, ByteType::HEX, ByteType::HEX, ByteType::NMSTRT,
        /* 0x68 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x6C */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x70 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x74 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT,
        /* 0x78 */ ByteType::NMSTRT, ByteType::NMSTRT, ByteType::NMSTRT, ByteType::OTHER,
        /* 0x7C */ ByteType::VERBAR, ByteType::OTHER, ByteType::OTHER, ByteType::OTHER,

        /* 0x80 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x84 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x88 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x8C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x90 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x94 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x98 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0x9C */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xA0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xA4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xA8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xAC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xB0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xB4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xB8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xBC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xC0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xC4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xC8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xCC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xD0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xD4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xD8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xDC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xE0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xE4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xE8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xEC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xF0 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xF4 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xF8 */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
        /* 0xFC */ ByteType::NONXML, ByteType::NONXML, ByteType::NONXML, ByteType::NONXML,
    ];
}

// Shared between Latin1Encoding and AsciiEncoding
fn latin1_toUtf16(
    mut from: &mut ExpatBufRef,
    to: &mut ExpatBufRefMut<u16>,
) -> XML_Convert_Result {
    while !from.is_empty() && to.is_empty() {
        to[0] = from[0] as u16;
        to.inc_start(1);
        *from = from.inc_start(1);
    }
    if to.is_empty() && !from.is_empty() {
        XML_Convert_Result::OUTPUT_EXHAUSTED
    } else {
        XML_Convert_Result::COMPLETED
    }
}

#[derive(Clone)]
pub struct InitEncoding {
    encoding_index: c_int,

    encoding_table: &'static [&'static ENCODING],

    encPtr: *mut *const ENCODING,
}

impl InitEncoding {
    fn new_impl(
        encPtr: *mut *const ENCODING,
        mut name: *const c_char,
        encoding_table: &'static [&'static ENCODING],
    ) -> Option<InitEncoding> {
        let mut i: c_int = unsafe { getEncodingIndex(name) };
        if i == UNKNOWN_ENC {
            return None;
        }
        Some(Self {
            encoding_index: i,
            encoding_table,
            encPtr,
        })
    }

    pub fn new_ns(
        encPtr: *mut *const ENCODING,
        mut name: *const c_char,
    ) -> Option<InitEncoding> {
        Self::new_impl(encPtr, name, &encodingsNS)
    }

    pub fn new(
        encPtr: *mut *const ENCODING,
        mut name: *const c_char,
    ) -> Option<InitEncoding> {
        Self::new_impl(encPtr, name, &encodings)
    }

    /* This is what detects the encoding.  encodingTable maps from
    encoding indices to encodings; INIT_ENC_INDEX(enc) is the index of
    the external (protocol) specified encoding; state is
    XML_STATE::CONTENT if we're parsing an external text entity, and
    XML_STATE::PROLOG otherwise.
     */
    fn initScan(
        &self,
        mut state: XML_STATE,
        buf: ExpatBufRef,
        mut nextTokPtr: &mut *const c_char,
    ) -> XML_TOK {
        if buf.is_empty() {
            return XML_TOK::NONE;
        }
        if buf.len() == 1 {
            /* only a single byte available for auto-detection */
            /* FIXME */
            /* so we're parsing an external text entity... */
            /* if UTF-16 was externally specified, then we need at least 2 bytes */
            match self.encoding_index as c_int {
                3 | 5 | 4 => return XML_TOK::PARTIAL,
                _ => {}
            }
            match buf[0] as c_uchar as c_int {
                254 | 255 | 239 => {
                    /* possibly first byte of UTF-8 BOM */
                    if self.encoding_index as c_int != ISO_8859_1_ENC || state != XML_STATE::CONTENT {
                        return XML_TOK::PARTIAL
                    }
                }
                0 | 60 => { return XML_TOK::PARTIAL }
                _ => { }
            }
        } else {
            match (buf[0] as c_uchar as c_int) << 8 | buf[1] as c_uchar as c_int {
                65279 => {
                    if !(self.encoding_index as c_int == ISO_8859_1_ENC && state == XML_STATE::CONTENT)
                    {
                        *nextTokPtr = buf[2..].as_ptr();
                        unsafe { *self.encPtr = &*self.encoding_table[UTF_16BE_ENC as usize]; }
                        return XML_TOK::BOM;
                    }
                }
                15360 => unsafe {
                    /* 00 3C is handled in the default case */
                    if !((self.encoding_index as c_int == UTF_16BE_ENC
                          || self.encoding_index as c_int == UTF_16_ENC)
                         && state == XML_STATE::CONTENT)
                    {
                        *self.encPtr = &*self.encoding_table[UTF_16LE_ENC as usize];
                        return (**self.encPtr).xmlTok(state, buf, nextTokPtr);
                    }
                }
                65534 => {
                    if !(self.encoding_index as c_int == ISO_8859_1_ENC && state == XML_STATE::CONTENT)
                    {
                        *nextTokPtr = buf[2..].as_ptr();
                        unsafe { *self.encPtr = &*self.encoding_table[UTF_16LE_ENC as usize]; }
                        return XML_TOK::BOM;
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
                    if state != XML_STATE::CONTENT || {
                        match self.encoding_index {
                            ISO_8859_1_ENC | 
                            UTF_16BE_ENC |
                            UTF_16LE_ENC |  
                            UTF_16_ENC => false,
                            _ => true 
                        }
                    } {
                        if buf.len() == 2 {
                            return XML_TOK::PARTIAL;
                        }
                        if buf[2] as c_uchar as c_int == 0xbf {
                            *nextTokPtr = buf[3..].as_ptr();
                            unsafe { *self.encPtr = &*self.encoding_table[UTF_8_ENC as usize]; }
                            return XML_TOK::BOM;
                        }
                    }
                }
                _ => unsafe {
                    if buf[0] as c_int == '\u{0}' as i32 {
                        /* 0 isn't a legal data character. Furthermore a document
                        entity can only start with ASCII characters.  So the only
                        way this can fail to be big-endian UTF-16 if it it's an
                        external parsed general entity that's labelled as
                        UTF-16LE.
                         */
                        if !(state == XML_STATE::CONTENT
                             && self.encoding_index as c_int == UTF_16LE_ENC)
                        {
                            *self.encPtr = &*self.encoding_table[UTF_16BE_ENC as usize];
                            return (**self.encPtr).xmlTok(state, buf, nextTokPtr);
                        }
                    } else if buf[1] as c_int == '\u{0}' as i32 {
                        /* We could recover here in the case:
                        - parsing an external entity
                        - second byte is 0
                        - no externally specified encoding
                        - no encoding declaration
                        by assuming UTF-16LE.  But we don't, because this would mean when
                        presented just with a single byte, we couldn't reliably determine
                        whether we needed further bytes.
                         */
                        if !(state == XML_STATE::CONTENT) {
                            *self.encPtr = &*self.encoding_table[UTF_16LE_ENC as usize];
                            return (**self.encPtr).xmlTok(state, buf, nextTokPtr);
                        }
                    }
                }
            }
        }
        unsafe {
            *self.encPtr = &*self.encoding_table[self.encoding_index as c_int as usize];
            (**self.encPtr).xmlTok(state, buf, nextTokPtr)
        }
    }

}

impl XmlEncoding for InitEncoding {
    // scanners[4]
    fn prologTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        self.initScan(
            XML_STATE::PROLOG,
            buf,
            nextTokPtr,
        )
    }
    fn contentTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        self.initScan(
            XML_STATE::CONTENT,
            buf,
            nextTokPtr,
        )
    }
    fn cdataSectionTok(
        &self,
        _buf: ExpatBufRef,
        _nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        XML_TOK::INVALID
    }
    fn ignoreSectionTok(
        &self,
        _buf: ExpatBufRef,
        _nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        XML_TOK::INVALID
    }

    // literalScanners[2]
    fn attributeValueTok(
        &self,
        _buf: ExpatBufRef,
        _nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        XML_TOK::INVALID
    }
    fn entityValueTok(
        &self,
        _buf: ExpatBufRef,
        _nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        XML_TOK::INVALID
    }

    fn nameMatchesAscii(
        &self,
        _buf: ExpatBufRef,
        _ptr2: &[libc::c_char],
    ) -> bool {
        false
    }

    unsafe fn nameLength(&self, _ptr: *const libc::c_char) -> libc::c_int {
        0
    }

    unsafe fn skipS(&self, _ptr: *const libc::c_char) -> *const libc::c_char {
        std::ptr::null()
    }

    fn getAtts(
        &self,
        _buf: ExpatBufRef,
        _f: &mut dyn FnMut(Attribute) -> XML_Error,
    ) -> XML_Error {
        XML_Error::NONE
    }

    fn charRefNumber(&self, _buf: ExpatBufRef) -> libc::c_int {
        0
    }

    fn predefinedEntityName(
        &self,
        _buf: ExpatBufRef,
    ) -> libc::c_int {
        0
    }

    fn updatePosition(
        &self,
        buf: ExpatBufRef,
        pos: &mut Position,
    ) {
        utf8_encoding.updatePosition(buf, pos);
    }

    fn isPublicId(
        &self,
        _buf: ExpatBufRef,
        _badPtr: &mut *const libc::c_char,
    ) -> libc::c_int {
        0
    }

    fn utf8Convert<'b, 'a: 'b>(
        &self,
        _from_buf: &mut ExpatBufRef<'a>,
        _to_buf: &'b mut ExpatBufRefMut<'a>,
    ) -> XML_Convert_Result {
        XML_Convert_Result::COMPLETED
    }

    fn utf16Convert(
        &self,
        _from_buf: &mut ExpatBufRef,
        _to_buf: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        XML_Convert_Result::COMPLETED
    }

    fn minBytesPerChar(&self) -> c_int {
        0
    }
    fn isUtf8(&self) -> bool {
        false
    }
    fn isUtf16(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct UnknownEncoding {
    types: [ByteType; 256],
    convert: Converter,
    userData: *mut c_void,
    utf16: [c_ushort; 256],
    utf8: [[c_char; 4]; 256],
}

impl UnknownEncoding {
    pub fn new() -> Self {
        Self {
            types: Latin1EncodingTable::types,
            convert: None,
            userData: std::ptr::null_mut(),
            utf16: [0; 256],
            utf8: [[0; 4]; 256],
        }
    }

    pub fn initialize(
        &mut self,
        table: *mut c_int,
        convert: Converter,
        userData: *mut c_void,
        is_ns: bool,
    ) -> bool {
        for i in 0..128 {
            if Latin1EncodingTable::types[i] != ByteType::OTHER
                && Latin1EncodingTable::types[i] != ByteType::NONXML
                && unsafe { *table.offset(i as isize) } != i as c_int
            {
                return false;
            }
        }
        for i in 0..256 {
            let mut c: c_int = unsafe { *table.offset(i as isize) };
            if c == -1 {
                self.types[i] = ByteType::MALFORM;
                /* This shouldn't really get used. */
                self.utf16[i] = 0xffff;
                self.utf8[i][0] = 1;
                self.utf8[i][1] = 0
            } else if c < 0 {
                if c < -(4) {
                    return false;
                }
                /* Multi-byte sequences need a converter function */
                if convert.is_none() {
                    return false;
                }
                let p: c_int = ByteType::LEAD2.to_i32().unwrap() - (c + 2);
                self.types[i] = ByteType::from_i32(p).unwrap();
                self.utf8[i][0] = 0;
                self.utf16[i] = 0
            } else if c < 0x80 {
                if Latin1EncodingTable::types[c as usize] != ByteType::OTHER
                    && Latin1EncodingTable::types[c as usize] != ByteType::NONXML
                    && c != i as c_int
                {
                    return false;
                }
                self.types[i] = Latin1EncodingTable::types[c as usize];
                self.utf8[i][0] = 1;
                self.utf8[i][1] = c as c_char;
                self.utf16[i] = if c == 0 { 0xffff } else { c } as c_ushort
            } else if checkCharRefNumber(c) < 0 {
                self.types[i] = ByteType::NONXML;
                /* This shouldn't really get used. */
                self.utf16[i] = 0xffff;
                self.utf8[i][0] = 1;
                self.utf8[i][1] = 0
            } else {
                if c > 0xffff {
                    return false;
                }
                if namingBitmap
                    [(((nmstrtPages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
                    & (1) << (c & 0xff & 0x1f)
                    != 0
                {
                    self.types[i] = ByteType::NMSTRT;
                } else if namingBitmap
                    [(((namePages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
                    & (1) << (c & 0xff & 0x1f)
                    != 0
                {
                    self.types[i] = ByteType::NAME;
                } else {
                    self.types[i] = ByteType::OTHER;
                }
                self.utf8[i][0] = XmlUtf8Encode(c, &mut self.utf8[i][1..]) as c_char;
                self.utf16[i] = c as c_ushort
            }
        }
        self.userData = userData;
        self.convert = convert;

        if is_ns {
            self.types[ASCII_COLON as usize] = ByteType::COLON;
        }

        true
    }
}

impl XmlEncodingImpl for UnknownEncoding {
    fn isUtf8(&self) -> bool { false }
    fn isUtf16(&self) -> bool { false }

    fn MINBPC(&self) -> isize { 1 }

    #[inline]
    fn byte_type(&self, p: *const c_char) -> ByteType {
        let idx = unsafe { *(p as *const u8) } as usize;
        self.types[idx]
    }

    #[inline]
    fn byte_to_ascii(&self, p: *const c_char) -> c_char {
        unsafe { *p }
    }

    #[inline]
    fn is_name_char(&self, p: *const c_char, _n: isize) -> bool {
        if let Some(convert) = self.convert {
            let mut c: c_int = unsafe { convert(self.userData, p) };
            if c & !(0xffff) != 0 {
                return false;
            }
            UCS2_GET_NAMING!(namePages, c >> 8, c & 0xff) != 0
        } else {
            false
        }
    }
    #[inline]
    fn is_nmstrt_char(&self, p: *const c_char, _n: isize) -> bool {
        if let Some(convert) = self.convert {
            let mut c: c_int = unsafe { convert(self.userData, p) };
            if c & !(0xffff) != 0 {
                return false;
            }
            UCS2_GET_NAMING!(nmstrtPages, c >> 8, c & 0xff) != 0
        } else {
            false
        }
    }

    #[inline]
    fn is_invalid_char(&self, p: *const c_char, _n: isize) -> bool {
        if let Some(convert) = self.convert {
            let mut c: c_int = unsafe { convert(self.userData, p) };
            (c & !(0xffff)) != 0 || checkCharRefNumber(c) < 0
        } else {
            false
        }
    }

    #[inline]
    fn is_name_char_minbpc(&self, _p: *const c_char) -> bool {
        false
    }

    #[inline]
    fn is_nmstrt_char_minbpc(&self, _p: *const c_char) -> bool {
        false
    }

    #[inline]
    fn char_matches(&self, p: *const c_char, c: c_char) -> bool {
        unsafe { *p == c }
    }

    #[inline]
    fn utf8Convert<'a: 'b, 'b>(
        &self,
        from_buf: &'b mut ExpatBufRef<'a>,
        to: &'b mut ExpatBufRefMut<'a>,
    ) -> XML_Convert_Result {
        let mut buf: [c_char; 4] = [0; 4];
        loop {
            let mut n: c_int = 0;
            if from_buf.is_empty() {
                return XML_Convert_Result::COMPLETED;
            }
            let mut utf8: ExpatBufRef = self.utf8[from_buf[0] as c_uchar as usize][..].into();
            n = utf8[0] as c_int;
            utf8 = utf8.inc_start(1);
            if n == 0 {
                let mut c: c_int = unsafe {
                    self.convert.expect("non-null function pointer")(self.userData, from_buf.as_ptr())
                };
                n = XmlUtf8Encode(c, &mut buf);
                if n as c_long > to.len() as c_long {
                    return XML_Convert_Result::OUTPUT_EXHAUSTED;
                }
                utf8 = buf[..].into();
                *from_buf = from_buf.inc_start(
                    (self.types[from_buf[0] as c_uchar as usize] as c_int
                     - (ByteType::LEAD2 as c_int - 2)) as isize);
            } else {
                if n as c_long > to.len() as c_long {
                    return XML_Convert_Result::OUTPUT_EXHAUSTED;
                }
                *from_buf = from_buf.inc_start(1);
            }
            to[..n as usize].copy_from_slice(&utf8[..n as usize]);
            to.inc_start(n as usize);
        }
    }

    #[inline]
    fn utf16Convert(
        &self,
        from_buf: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        while !from_buf.is_empty() && !to.is_empty() {
            let mut c: c_ushort = self.utf16[from_buf[0] as c_uchar as usize];
            if c as c_int == 0 {
                c = unsafe {
                    self.convert.expect("non-null function pointer")(self.userData, from_buf.as_ptr())
                    as c_ushort
                };
                *from_buf = (*from_buf).inc_start(
                    (self.types[from_buf[0] as c_uchar as usize] as c_int
                     - (ByteType::LEAD2 as c_int - 2)) as isize);
            } else {
                *from_buf = (*from_buf).inc_start(1)
            }
            to[0] = c;
            to.inc_start(1);
        }
        if to.is_empty() && !from_buf.is_empty() {
            XML_Convert_Result::OUTPUT_EXHAUSTED
        } else {
            XML_Convert_Result::COMPLETED
        }
    }
}



const latin1_encoding: Latin1Encoding = Latin1EncodingImpl(PhantomData);
const latin1_encoding_ns: Latin1EncodingNS = Latin1EncodingImpl(PhantomData);
const utf8_encoding: Utf8Encoding = Utf8EncodingImpl(PhantomData);
const utf8_encoding_ns: Utf8EncodingNS = Utf8EncodingImpl(PhantomData);
const internal_utf8_encoding: InternalUtf8Encoding = Utf8EncodingImpl(PhantomData);
const internal_utf8_encoding_ns: InternalUtf8EncodingNS = Utf8EncodingImpl(PhantomData);
const ascii_encoding: AsciiEncoding = AsciiEncodingImpl(PhantomData);
const ascii_encoding_ns: AsciiEncodingNS = AsciiEncodingImpl(PhantomData);
const little2_encoding: Little2Encoding = Little2EncodingImpl(PhantomData);
const little2_encoding_ns: Little2EncodingNS = Little2EncodingImpl(PhantomData);
#[cfg(target_endian = "little")]
const internal_little2_encoding: InternalLittle2Encoding = Little2EncodingImpl(PhantomData);
#[cfg(target_endian = "little")]
const internal_little2_encoding_ns: InternalLittle2EncodingNS = Little2EncodingImpl(PhantomData);
const big2_encoding: Big2Encoding = Big2EncodingImpl(PhantomData);
const big2_encoding_ns: Big2EncodingNS = Big2EncodingImpl(PhantomData);
#[cfg(target_endian = "big")]
const internal_big2_encoding: InternalBig2Encoding = Big2EncodingImpl(PhantomData);
#[cfg(target_endian = "big")]
const internal_big2_encoding_ns: InternalBig2EncodingNS = Big2EncodingImpl(PhantomData);

pub fn XmlGetUtf8InternalEncodingNS() -> &'static ENCODING {
    &internal_utf8_encoding_ns
}

pub fn XmlGetUtf8InternalEncoding() -> &'static ENCODING {
    &internal_utf8_encoding
}

#[cfg(target_endian = "little")]
pub fn XmlGetUtf16InternalEncoding() -> &'static ENCODING {
    &internal_little2_encoding
}

#[cfg(target_endian = "big")]
pub fn XmlGetUtf16InternalEncoding() -> &'static ENCODING {
    &internal_big2_encoding
}

#[cfg(target_endian = "little")]
pub fn XmlGetUtf16InternalEncodingNS() -> &'static ENCODING {
    &internal_little2_encoding_ns
}

#[cfg(target_endian = "big")]
pub fn XmlGetUtf16InternalEncodingNS() -> &'static ENCODING {
    &internal_big2_encoding_ns
}

pub const encodingsNS: [&'static ENCODING; 7] = [
    &latin1_encoding_ns,
    &ascii_encoding_ns,
    &utf8_encoding_ns,
    &big2_encoding_ns,
    &big2_encoding_ns,
    &little2_encoding_ns,
    &utf8_encoding_ns,
];

pub const encodings: [&'static ENCODING; 7] = [
    &latin1_encoding,
    &ascii_encoding,
    &utf8_encoding,
    &big2_encoding,
    &big2_encoding,
    &little2_encoding,
    &utf8_encoding,
];

pub fn findEncoding(
    mut enc: &ENCODING,
    mut buf: ExpatBufRef,
) -> Option<*const ENCODING> {
    let mut out_buf: [c_char; 128] = [0; 128];
    let mut p = ExpatBufRefMut::new(
        out_buf.as_mut_ptr(),
        &mut out_buf[127],
    );
    let mut i: c_int = 0;
    (*enc).utf8Convert(&mut buf, &mut p);
    if !buf.is_empty() {
        return None;
    }
    p[0] = 0;
    unsafe {
        if streqci(out_buf.as_ptr(), KW_UTF_16.as_ptr()) != 0 && (*enc).minBytesPerChar() == 2 {
            return Some(enc.clone());
        }
        i = getEncodingIndex(out_buf.as_ptr());
        if i == UNKNOWN_ENC {
            None
        } else {
            Some(encodings[i as usize])
        }
    }
}

pub fn findEncodingNS(
    mut enc: &ENCODING,
    mut buf: ExpatBufRef,
) -> Option<*const ENCODING> {
    let mut out_buf: [c_char; 128] = [0; 128];
    let mut p = ExpatBufRefMut::new(
        out_buf.as_mut_ptr(),
        &mut out_buf[127],
    );
    let mut i: c_int = 0;
    (*enc).utf8Convert(&mut buf, &mut p);
    if !buf.is_empty() {
        return None;
    }
    p[0] = 0;
    unsafe {
        if streqci(out_buf.as_ptr(), KW_UTF_16.as_ptr()) != 0 && (*enc).minBytesPerChar() == 2 {
            return Some(enc.clone());
        }
        i = getEncodingIndex(out_buf.as_ptr());
        if i == UNKNOWN_ENC {
            None
        } else {
            Some(encodingsNS[i as usize])
        }
    }
}

pub fn XmlParseXmlDeclNS<'a>(
    mut isGeneralTextEntity: c_int,
    mut enc: &ENCODING,
    mut buf: ExpatBufRef<'a>,
    mut badPtr: &mut *const c_char,
    mut versionBuf: &mut Option<ExpatBufRef<'a>>,
    mut encodingName: &mut *const c_char,
    mut encoding: &mut Option<*const ENCODING>,
    mut standalone: &mut c_int,
) -> c_int {
    return doParseXmlDecl(
        Some(findEncodingNS),
        isGeneralTextEntity,
        enc,
        buf,
        badPtr,
        versionBuf,
        encodingName,
        encoding,
        standalone,
    );
}

pub fn XmlParseXmlDecl<'a>(
    mut isGeneralTextEntity: c_int,
    mut enc: &ENCODING,
    mut buf: ExpatBufRef<'a>,
    mut badPtr: &mut *const c_char,
    mut versionBuf: &mut Option<ExpatBufRef<'a>>,
    mut encodingName: &mut *const c_char,
    mut encoding: &mut Option<*const ENCODING>,
    mut standalone: &mut c_int,
) -> c_int {
    return doParseXmlDecl(
        Some(findEncoding),
        isGeneralTextEntity,
        enc,
        buf,
        badPtr,
        versionBuf,
        encodingName,
        encoding,
        standalone,
    );
}

pub use crate::ascii_h::*;
pub use crate::expat_external_h::XML_Size;
pub use crate::lib::nametab::{namePages, namingBitmap, nmstrtPages};
pub use crate::stdbool_h::{false_0, true_0};
pub use crate::xmltok_impl_h::*;

pub type normal_encoding = Box<dyn XmlEncoding>;

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

pub type unknown_encoding = UnknownEncoding;
/* minimum bytes per character */
/* c is an ASCII character */

pub type C2RustUnnamed_7 = c_uint;
/* If this enumeration is changed, getEncodingIndex and encodings
must also be changed. */

pub type C2RustUnnamed_8 = c_int;

pub const US_ASCII_ENC: C2RustUnnamed_8 = 1;

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
pub fn _INTERNAL_trim_to_complete_utf8_characters(
    from_buf: &mut ExpatBufRef,
) {
    let mut i = from_buf.len();
    let mut walked: size_t = 0;
    while i > 0 && i <= from_buf.len() {
        let prev: c_uchar = from_buf[i-1] as c_uchar;
        if prev as c_uint & 0xf8 == 0xf0 {
            /* 4-byte character, lead by 0b11110xxx byte */
            if walked.wrapping_add(1) >= 4 {
                i += 4 - 1;
                break;
            } else {
                walked = 0
            }
        } else if prev as c_uint & 0xf0 == 0xe0 {
            /* 3-byte character, lead by 0b1110xxxx byte */
            if walked.wrapping_add(1) >= 3 {
                i += 3 - 1;
                break;
            } else {
                walked = 0
            }
        } else if prev as c_uint & 0xe0 == 0xc0 {
            /* 2-byte character, lead by 0b110xxxxx byte */
            if walked.wrapping_add(1) >= 2 {
                i += 2 - 1;
                break;
            } else {
                walked = 0
            }
        } else if prev as c_uint & 0x80 == 0 {
            break;
        }
        i -= 1;
        walked = walked.wrapping_add(1)
    }
    *from_buf = from_buf.with_len(i);
}

// Shared between Big2 and Little2 encodings
fn unicode_byte_type(mut hi: c_char, mut lo: c_char) -> ByteType {
    match hi as c_uchar as c_int {
        216 | 217 | 218 | 219 => {
            /* 0xD800–0xDBFF first 16-bit code unit or high surrogate (W1) */
            return ByteType::LEAD4;
        }
        220 | 221 | 222 | 223 => {
            /* 0xDC00–0xDFFF second 16-bit code unit or low surrogate (W2) */
            return ByteType::TRAIL;
        }
        255 => {
            match lo as c_uchar as c_int {
                255 | 254 => {
                    /* noncharacter-FFFF */
                    /* noncharacter-FFFE */
                    return ByteType::NONXML;
                }
                _ => {}
            }
        }
        _ => {}
    }
    ByteType::NONASCII
}

unsafe fn streqci(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
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
            return 0;
        }
        if c1 == 0 {
            break;
        }
    }
    1
}

fn initUpdatePosition(
    mut _enc: &ENCODING,
    buf: ExpatBufRef,
    mut pos: &mut Position,
) {
    utf8_encoding.updatePosition(buf, pos);
}

fn toAscii(
    mut enc: &ENCODING,
    mut buf: ExpatBufRef,
) -> c_int {
    let mut out_buf: [c_char; 1] = [0; 1];
    let mut p = ExpatBufRefMut::new(
        &mut out_buf[0],
        out_buf[1..].as_mut_ptr(),
    );
    (*enc).utf8Convert(&mut buf, &mut p);
    if p.as_ptr() == out_buf.as_ptr() {
        return -1;
    } else {
        return out_buf[0] as c_int;
    };
}

fn isSpace(mut c: c_int) -> c_int {
    match c {
        32 | 13 | 10 | 9 => return 1,
        _ => {}
    }
    0
}
/* Return 1 if there's just optional white space or there's an S
   followed by name=val.
*/

fn parsePseudoAttribute<'a>(
    mut enc: &ENCODING,
    mut buf: ExpatBufRef<'a>,
    mut name: &mut Option<ExpatBufRef<'a>>,
    mut val: &mut Option<ExpatBufRef<'a>>,
    mut nextTokPtr: &mut *const c_char,
) -> c_int {
    let mut c: c_int = 0;
    let mut open: c_char = 0;
    if buf.is_empty() {
        *name = None;
        return 1;
    }
    if isSpace(toAscii(enc, buf)) == 0 {
        *nextTokPtr = buf.as_ptr();
        return 0;
    }
    loop {
        buf = buf.inc_start(((*enc).minBytesPerChar()) as isize);
        if !(isSpace(toAscii(enc, buf)) != 0) {
            break;
        }
    }
    if buf.is_empty() {
        *name = None;
        return 1;
    }
    *name = Some(buf.clone());
    loop {
        c = toAscii(enc, buf);
        if c == -1 {
            *nextTokPtr = buf.as_ptr();
            return 0;
        }
        if c == ASCII_EQUALS as c_int {
            *name = name.map(|name| name.with_end(buf.as_ptr()));
            break;
        } else if isSpace(c) != 0 {
            *name = name.map(|name| name.with_end(buf.as_ptr()));
            loop {
                buf = buf.inc_start(((*enc).minBytesPerChar()) as isize);
                c = toAscii(enc, buf);
                if !(isSpace(c) != 0) {
                    break;
                }
            }
            if c != ASCII_EQUALS as c_int {
                *nextTokPtr = buf.as_ptr();
                return 0;
            }
            break;
        } else {
            buf = buf.inc_start(((*enc).minBytesPerChar()) as isize);
        }
    }
    if buf.as_ptr() == name.unwrap().as_ptr() {
        *nextTokPtr = buf.as_ptr();
        return 0;
    }
    buf = buf.inc_start(((*enc).minBytesPerChar()) as isize);
    c = toAscii(enc, buf);
    while isSpace(c) != 0 {
        buf = buf.inc_start(((*enc).minBytesPerChar()) as isize);
        c = toAscii(enc, buf)
    }
    if c != ASCII_QUOT as c_int && c != ASCII_APOS as c_int {
        *nextTokPtr = buf.as_ptr();
        return 0;
    }
    open = c as c_char;
    buf = buf.inc_start(((*enc).minBytesPerChar()) as isize);
    *val = Some(buf.clone());
    loop {
        c = toAscii(enc, buf);
        if c == open as c_int {
            break;
        }
        if !((ASCII_a as c_int) <= c && c <= (ASCII_z as c_int))
            && !((ASCII_A as c_int) <= c && c <= (ASCII_Z as c_int))
            && !((ASCII_0 as c_int) <= c && c <= (ASCII_9 as c_int))
            && c != ASCII_PERIOD as c_int
            && c != ASCII_MINUS as c_int
            && c != ASCII_UNDERSCORE as c_int
        {
            *nextTokPtr = buf.as_ptr();
            return 0;
        }
        buf = buf.inc_start(((*enc).minBytesPerChar()) as isize);
    }
    *nextTokPtr = buf.inc_start((*enc).minBytesPerChar() as isize).as_ptr();
    1
}

const KW_version: [c_char; 7] =
    [ASCII_v, ASCII_e, ASCII_r, ASCII_s, ASCII_i, ASCII_o, ASCII_n];

const KW_encoding: [c_char; 8] =
    [ASCII_e, ASCII_n, ASCII_c, ASCII_o, ASCII_d, ASCII_i, ASCII_n, ASCII_g];

const KW_standalone: [c_char; 10] = [
    ASCII_s, ASCII_t, ASCII_a, ASCII_n, ASCII_d, ASCII_a, ASCII_l, ASCII_o, ASCII_n, ASCII_e
];

const KW_yes: [c_char; 3] = [ASCII_y, ASCII_e, ASCII_s];

const KW_no: [c_char; 2] = [ASCII_n, ASCII_o];

#[cfg(feature = "mozilla")]
const KW_XML_1_0: [c_char; 3] = [ASCII_1, ASCII_PERIOD, ASCII_0];

fn doParseXmlDecl<'a>(
    mut encodingFinder: Option<
        unsafe fn(
            _: &ENCODING,
            _: ExpatBufRef,
        ) -> Option<*const ENCODING>,
    >,
    mut isGeneralTextEntity: c_int,
    mut enc: &ENCODING,
    mut buf: ExpatBufRef<'a>,
    mut badPtr: &mut *const c_char,
    mut versionBuf: &mut Option<ExpatBufRef<'a>>,
    mut encodingName: &mut *const c_char,
    mut encoding: &mut Option<*const ENCODING>,
    mut standalone: &mut c_int,
) -> c_int {
    let mut val_buf = None;
    let mut name = None;
    buf = buf
        .inc_start((5 * (*enc).minBytesPerChar()) as isize)
        .dec_end((2 * (*enc).minBytesPerChar()) as usize);
    let mut pseudo_ptr = buf.as_ptr();
    if parsePseudoAttribute(enc, buf, &mut name, &mut val_buf, &mut pseudo_ptr) == 0
        || name.is_none()
    {
        *badPtr = pseudo_ptr;
        return 0;
    }
    buf = buf.with_start(pseudo_ptr);
    if !(*enc).nameMatchesAscii(name.unwrap(), &KW_version) {
        if isGeneralTextEntity == 0 {
            *badPtr = name.map_or(ptr::null(), |x| x.as_ptr());
            return 0;
        }
    } else {
        *versionBuf = val_buf;
        #[cfg(feature = "mozilla")]
        {
            if !(*enc).nameMatchesAscii(val_buf
                                       .unwrap()
                                       .with_end(pseudo_ptr)
                                       .dec_end((*enc).minBytesPerChar() as usize),
                                       &KW_XML_1_0)
            {
                *badPtr = val_buf.map_or(ptr::null(), |x| x.as_ptr());
                return 0;
            }
        }
        let mut pseudo_ptr = buf.as_ptr();
        if parsePseudoAttribute(enc, buf, &mut name, &mut val_buf, &mut pseudo_ptr) == 0 {
            *badPtr = pseudo_ptr;
            return 0;
        }
        buf = buf.with_start(pseudo_ptr);
        if name.is_none() {
            if isGeneralTextEntity != 0 {
                /* a TextDecl must have an EncodingDecl */
                *badPtr = buf.as_ptr();
                return 0;
            }
            return 1;
        }
    }
    if (*enc).nameMatchesAscii(name.unwrap(), &KW_encoding) {
        let mut c: c_int = toAscii(enc, val_buf.unwrap());
        if !((ASCII_a as c_int) <= c && c <= (ASCII_z as c_int))
            && !((ASCII_A as c_int) <= c && c <= (ASCII_Z as c_int))
        {
            *badPtr = val_buf.map_or(ptr::null(), |x| x.as_ptr());
            return 0;
        }
        *encodingName = val_buf.unwrap().as_ptr();
        *encoding = unsafe { encodingFinder.expect("non-null function pointer")(
            enc,
            val_buf
                .unwrap()
                .with_end(buf.as_ptr())
                .dec_end(((*enc).minBytesPerChar()) as usize),
        ) };
        let mut pseudo_ptr = buf.as_ptr();
        if parsePseudoAttribute(enc, buf, &mut name, &mut val_buf, &mut pseudo_ptr) == 0 {
            *badPtr = buf.as_ptr();
            return 0;
        }
        buf = buf.with_start(pseudo_ptr);
        if name.is_none() {
            return 1;
        }
    }
    if !(*enc).nameMatchesAscii(name.unwrap(), &KW_standalone)
        || isGeneralTextEntity != 0
    {
        *badPtr = name.map_or(ptr::null(), |x| x.as_ptr());
        return 0;
    }
    if (*enc).nameMatchesAscii(
        val_buf
            .unwrap()
            .with_end(buf.as_ptr())
            .dec_end(((*enc).minBytesPerChar()) as usize),
        &KW_yes,
    )
    {
        *standalone = 1;
    } else if (*enc).nameMatchesAscii(
        val_buf
            .unwrap()
            .with_end(buf.as_ptr())
            .dec_end(((*enc).minBytesPerChar()) as usize),
        &KW_no,
    )
    {
        *standalone = 0;
    } else {
        *badPtr = val_buf.map_or(ptr::null(), |x| x.as_ptr());
        return 0;
    }
    // TODO(SJC): make toAscii take a buf
    while isSpace(toAscii(enc, buf)) != 0 {
        buf = buf.inc_start(((*enc).minBytesPerChar()) as isize);
    }
    if !buf.is_empty() {
        *badPtr = buf.as_ptr();
        return 0;
    }
    1
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

pub fn checkCharRefNumber(mut result: c_int) -> c_int {
    match result >> 8 {
        216..=223 => return -1,
        0 => {
            if Latin1EncodingTable::types[result as usize] as c_int == ByteType::NONXML as c_int {
                return -1;
            }
        }
        255 => {
            if result == 0xfffe || result == 0xffff {
                return -1;
            }
        }
        _ => {}
    } /* LCOV_EXCL_LINE: this case is always eliminated beforehand */
    result
}

pub fn XmlUtf8Encode(mut c: c_int, buf: &mut [c_char]) -> c_int {
    if c < 0 {
        return 0;
    }
    if c < min2 as c_int {
        buf[0] = (c | UTF8_cval1 as c_int) as c_char;
        return 1;
    }
    if c < min3 as c_int {
        buf[0] = (c >> 6 | UTF8_cval2 as c_int) as c_char;
        buf[1] = (c & 0x3f | 0x80) as c_char;
        return 2;
    }
    if c < min4 as c_int {
        buf[0] = (c >> 12 | UTF8_cval3 as c_int) as c_char;
        buf[1] = (c >> 6 & 0x3f | 0x80) as c_char;
        buf[2] = (c & 0x3f | 0x80) as c_char;
        return 3;
    }
    if c < 0x110000 {
        buf[0] = (c >> 18 | UTF8_cval4 as c_int) as c_char;
        buf[1] = (c >> 12 & 0x3f | 0x80) as c_char;
        buf[2] = (c >> 6 & 0x3f | 0x80) as c_char;
        buf[3] = (c & 0x3f | 0x80) as c_char;
        return 4;
    }
    0
    /* LCOV_EXCL_LINE: this case too is eliminated before calling */
}

pub fn XmlUtf16Encode(mut charNum: c_int, buf: &mut [c_ushort]) -> c_int {
    if charNum < 0 {
        return 0;
    }
    if charNum < 0x10000 {
        buf[0] = charNum as c_ushort;
        return 1;
    }
    if charNum < 0x110000 {
        charNum -= 0x10000;
        buf[0] = ((charNum >> 10) + 0xd800) as c_ushort;
        buf[1] = ((charNum & 0x3ff) + 0xdc00) as c_ushort;
        return 2;
    }
    0
}

pub fn XmlSizeOfUnknownEncoding() -> c_int {
    ::std::mem::size_of::<unknown_encoding>() as c_int
}

const KW_ISO_8859_1: [c_char; 11] = [
    ASCII_I, ASCII_S, ASCII_O, ASCII_MINUS, ASCII_8, ASCII_8, ASCII_5, ASCII_9, ASCII_MINUS,
    ASCII_1, 0,
];

const KW_US_ASCII: [c_char; 9] =
    [ASCII_U, ASCII_S, ASCII_MINUS, ASCII_A, ASCII_S, ASCII_C, ASCII_I, ASCII_I, 0];

const KW_UTF_8: [c_char; 6] =
    [ASCII_U, ASCII_T, ASCII_F, ASCII_MINUS, ASCII_8, 0];

const KW_UTF_16: [c_char; 7] =
    [ASCII_U, ASCII_T, ASCII_F, ASCII_MINUS, ASCII_1, ASCII_6, 0];

const KW_UTF_16BE: [c_char; 9] =
    [ASCII_U, ASCII_T, ASCII_F, ASCII_MINUS, ASCII_1, ASCII_6, ASCII_B, ASCII_E, 0];

const KW_UTF_16LE: [c_char; 9] =
    [ASCII_U, ASCII_T, ASCII_F, ASCII_MINUS, ASCII_1, ASCII_6, ASCII_L, ASCII_E, 0];

unsafe fn getEncodingIndex(mut name: *const c_char) -> c_int {
    const encodingNames: [*const c_char; 6] = [
        KW_ISO_8859_1.as_ptr(),
        KW_US_ASCII.as_ptr(),
        KW_UTF_8.as_ptr(),
        KW_UTF_16.as_ptr(),
        KW_UTF_16BE.as_ptr(),
        KW_UTF_16LE.as_ptr(),
    ];
    let mut i: c_int = 0;
    if name.is_null() {
        return NO_ENC;
    }
    i = 0;
    while i
        < (::std::mem::size_of::<[*const c_char; 6]>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<*const c_char>() as c_ulong) as c_int
    {
        if streqci(name, encodingNames[i as usize]) != 0 {
            return i;
        }
        i += 1
    }
    UNKNOWN_ENC
}
// XML_NS
