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

use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use crate::expat_h::{XML_Error};
use super::xmlparse::{ExpatBufRef, ExpatBufRefMut};
use std::convert::TryInto;
use std::ptr;
use crate::xmltok_impl_h::ByteType;
use num_traits::{ToPrimitive, FromPrimitive};

#[cfg(feature = "mozilla")]
pub mod moz_extensions;

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
pub const XML_TOK_PROLOG_S: c_int = 15;
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
pub const XML_CDATA_SECTION_STATE: c_int = 2;
pub const XML_IGNORE_SECTION_STATE: c_int = 3;

pub const XML_ATTRIBUTE_VALUE_LITERAL: c_int = 0;
pub const XML_ENTITY_VALUE_LITERAL: c_int = 1;

/* The size of the buffer passed to XmlUtf8Encode must be at least this. */
pub const XML_UTF8_ENCODE_MAX: c_int = 4;
/* The size of the buffer passed to XmlUtf16Encode must be at least this. */
pub const XML_UTF16_ENCODE_MAX: c_int = 2;

pub type POSITION = position;

#[repr(C)]
#[derive(Copy, Clone, Default)]
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
    pub normalized: bool,
}

pub type ENCODING = dyn XmlEncoding;

pub type XML_Convert_Result = c_uint;
pub const XML_CONVERT_COMPLETED: XML_Convert_Result = 0;
pub const XML_CONVERT_INPUT_INCOMPLETE: XML_Convert_Result = 1;
pub const XML_CONVERT_OUTPUT_EXHAUSTED: XML_Convert_Result = 2; /* and therefore potentially input remaining as well */

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

pub type CONVERTER = Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int>;

// =============== END xmltok_h ================

// Replaces ENCODING
pub trait XmlEncoding {
    // scanners[4]
    unsafe fn prologTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int;
    unsafe fn contentTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int;
    unsafe fn cdataSectionTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int;
    unsafe fn ignoreSectionTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int;

    // literalScanners[2]
    unsafe fn attributeValueTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int;
    unsafe fn entityValueTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int;

    fn nameMatchesAscii(
        &self,
        buf: ExpatBufRef,
        ptr2: &[libc::c_char],
    ) -> libc::c_int;

    unsafe fn nameLength(&self, ptr: *const libc::c_char) -> libc::c_int;

    unsafe fn skipS(&self, ptr: *const libc::c_char) -> *const libc::c_char;

    unsafe fn getAtts(
        &self,
        buf: ExpatBufRef,
        f: &mut dyn FnMut(ATTRIBUTE) -> XML_Error,
    ) -> XML_Error;

    unsafe fn charRefNumber(&self, buf: ExpatBufRef) -> libc::c_int;

    unsafe fn predefinedEntityName(
        &self,
        buf: ExpatBufRef,
    ) -> libc::c_int;

    unsafe fn updatePosition(
        &self,
        buf: ExpatBufRef,
        pos: *mut POSITION,
    );

    unsafe fn isPublicId(
        &self,
        buf: ExpatBufRef,
        badPtr: *mut *const libc::c_char,
    ) -> libc::c_int;

    unsafe fn utf8Convert<'b, 'a: 'b>(
        &self,
        from_buf: &'b mut ExpatBufRef<'a>,
        to_buf: &'b mut ExpatBufRefMut<'a>,
    ) -> XML_Convert_Result;

    unsafe fn utf16Convert(
        &self,
        from_buf: &mut ExpatBufRef,
        to_buf: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result;

    fn minBytesPerChar(&self) -> c_int;
    fn isUtf8(&self) -> bool;
    fn isUtf16(&self) -> bool;

    // xmlTok and xmlLiteralTok were macros
    #[inline]
    unsafe fn xmlTok(
        &self,
        state: c_int,
        buf: ExpatBufRef,
        nextTokPtr: *mut *const c_char,
    ) -> c_int {
        match state {
            XML_PROLOG_STATE => self.prologTok(buf, nextTokPtr),
            XML_CONTENT_STATE => self.contentTok(buf, nextTokPtr),
            XML_CDATA_SECTION_STATE => self.cdataSectionTok(buf, nextTokPtr),
            XML_IGNORE_SECTION_STATE => self.ignoreSectionTok(buf, nextTokPtr),
            _ => panic!("Unexpected state {}", state),
        }
    }

    #[inline]
    unsafe fn xmlLiteralTok(
        &self,
        literal_type: c_int,
        buf: ExpatBufRef,
        nextTokPtr: *mut *const c_char,
    ) -> c_int {
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

    unsafe fn utf8Convert<'a: 'b, 'b>(
        &self,
        from_buf: &'b mut ExpatBufRef<'a>,
        to_buf: &'b mut ExpatBufRefMut<'a>,
    ) -> XML_Convert_Result;

    unsafe fn utf16Convert(
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

impl<T: NormalEncodingTable> Utf8EncodingImpl<T> {
    fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

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
                    ((*(p as *const c_uchar) as c_int) < 0xc2
                        || *(p as *const c_uchar).offset(1) as c_int & 0x80 == 0
                        || *(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0)
                }
                3 => {
                    (*(p as *const c_uchar).offset(2) as c_int & 0x80 == 0
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
                        }) != 0)
                }
                4 => {
                    (*(p as *const c_uchar).offset(3) as c_int & 0x80 == 0
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
                        }) != 0)
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
    unsafe fn utf8Convert<'a: 'b, 'b>(
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
            XML_CONVERT_OUTPUT_EXHAUSTED
        } else if input_incomplete {
            XML_CONVERT_INPUT_INCOMPLETE
        } else {
            XML_CONVERT_COMPLETED
        }
    }

    #[inline]
    unsafe fn utf16Convert(
        &self,
        from: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        let mut current_block: u64;
        let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
        // let mut to: *mut c_ushort = *toP;
        // let mut from: *const c_char = *fromP;
        loop {
            if from.is_empty() || to.is_empty() {
                current_block = 1608152415753874203;
                break;
            }
            match T::types[from[0] as c_uchar as usize] as c_int {
                5 => {
                    if (from.len() as c_long) < 2 {
                        res = XML_CONVERT_INPUT_INCOMPLETE;
                        current_block = 10086016483950629671;
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
                        res = XML_CONVERT_INPUT_INCOMPLETE;
                        current_block = 10086016483950629671;
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
                        res = XML_CONVERT_OUTPUT_EXHAUSTED;
                        current_block = 10086016483950629671;
                        break;
                    } else if (from.len() as c_long) < 4 {
                        res = XML_CONVERT_INPUT_INCOMPLETE;
                        current_block = 10086016483950629671;
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
        match current_block {
            1608152415753874203 => {
                if !from.is_empty() {
                    res = XML_CONVERT_OUTPUT_EXHAUSTED
                }
            }
            _ => {}
        }
        res
    }
}

struct Latin1EncodingImpl<T: NormalEncodingTable>(std::marker::PhantomData<T>);

impl<T: NormalEncodingTable> Latin1EncodingImpl<T> {
    fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

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
    unsafe fn utf8Convert<'a: 'b, 'b>(
        &self,
        from: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut,
    ) -> XML_Convert_Result {
        loop {
            let mut c: c_uchar = 0;
            if from.is_empty() {
                return XML_CONVERT_COMPLETED;
            }
            c = from[0] as c_uchar;
            if c as c_int & 0x80 != 0 {
                if (to.len() as c_long) < 2 {
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                to[0] = (c as c_int >> 6 | UTF8_cval2 as c_int) as c_char;
                to.inc_start(1);
                to[0] = (c as c_int & 0x3f | 0x80) as c_char;
                to.inc_start(1);
                *from = from.inc_start(1);
            } else {
                if to.is_empty() {
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                to[0] = from[0];
                to.inc_start(1);
                *from = from.inc_start(1);
            }
        }
    }

    #[inline]
    unsafe fn utf16Convert(
        &self,
        from_buf: &mut ExpatBufRef,
        to_buf: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        latin1_toUtf16(from_buf, to_buf)
    }
}

struct AsciiEncodingImpl<T: NormalEncodingTable>(std::marker::PhantomData<T>);

impl<T: NormalEncodingTable> AsciiEncodingImpl<T> {
    fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

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
    unsafe fn utf8Convert<'a: 'b, 'b>(
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
            XML_CONVERT_OUTPUT_EXHAUSTED
        } else {
            XML_CONVERT_COMPLETED
        }
    }

    #[inline]
    unsafe fn utf16Convert(
        &self,
        from_buf: &mut ExpatBufRef,
        to_buf: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        latin1_toUtf16(from_buf, to_buf)
    }
}

struct Little2EncodingImpl<T: NormalEncodingTable>(std::marker::PhantomData<T>);

impl<T: NormalEncodingTable> Little2EncodingImpl<T> {
    fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

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
            unsafe { unicode_byte_type(bytes.1, bytes.0) }
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
    unsafe fn utf8Convert<'a: 'b, 'b>(
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
            let mut current_block_34: u64;
            match hi as c_int {
                0 => {
                    if (lo as c_int) < 0x80 {
                        if to.is_empty() {
                            *fromP = from;
                            return XML_CONVERT_OUTPUT_EXHAUSTED;
                        }
                        to[0] = lo as c_char;
                        to.inc_start(1);
                        current_block_34 = 18435049525520518667;
                    } else {
                        current_block_34 = 11412679543673842523;
                    }
                }
                1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                    current_block_34 = 11412679543673842523;
                }
                216 | 217 | 218 | 219 => {
                    if (to.len() as c_long) < 4 {
                        *fromP = from;
                        return XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    if (from.len() as c_long) < 4 {
                        *fromP = from;
                        return XML_CONVERT_INPUT_INCOMPLETE;
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
                    current_block_34 = 18435049525520518667;
                }
                _ => {
                    if (to.len() as c_long) < 3 {
                        *fromP = from;
                        return XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    to[0] = (hi as c_int >> 4 | UTF8_cval3 as c_int) as c_char;
                    to.inc_start(1);
                    to[0] = ((hi as c_int & 0xf) << 2 | lo as c_int >> 6 | 0x80) as c_char;
                    to.inc_start(1);
                    to[0] = (lo as c_int & 0x3f | 0x80) as c_char;
                    to.inc_start(1);
                    current_block_34 = 18435049525520518667;
                }
            }
            match current_block_34 {
                11412679543673842523 => {
                    if (to.len() as c_long) < 2 {
                        *fromP = from;
                        return XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    to[0] = (lo as c_int >> 6 | (hi as c_int) << 2 | UTF8_cval2 as c_int) as c_char;
                    to.inc_start(1);
                    to[0] = (lo as c_int & 0x3f | 0x80) as c_char;
                    to.inc_start(1);
                }
                _ => {}
            }
            from = from.inc_start(2)
        }
        *fromP = from;
        if !from.is_empty() {
            XML_CONVERT_INPUT_INCOMPLETE
        } else {
            XML_CONVERT_COMPLETED
        }
    }

    #[inline]
    unsafe fn utf16Convert(
        &self,
        from: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
        *from = from.with_len(((from.len() as c_long >> 1) << 1) as usize);
        if from.len() as c_long > ((to.len() as c_long) << 1)
            && from[from.len()-1] as c_uchar as c_int & 0xf8 == 0xd8
        {
            *from = from.dec_end(2);
            res = XML_CONVERT_INPUT_INCOMPLETE
        }
        while !from.is_empty() && !to.is_empty() {
            to[0] = ((from[1] as c_uchar as c_int) << 8
                     | from[0] as c_uchar as c_int) as c_ushort;
            to.inc_start(1);
            *from = from.inc_start(2);
        }
        if to.is_empty() && !from.is_empty() {
            XML_CONVERT_OUTPUT_EXHAUSTED
        } else {
            res
        }
    }
}

struct Big2EncodingImpl<T: NormalEncodingTable>(std::marker::PhantomData<T>);

impl<T: NormalEncodingTable> Big2EncodingImpl<T> {
    fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

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
            unsafe { unicode_byte_type(bytes.0, bytes.1) }
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
    unsafe fn utf8Convert<'a: 'b, 'b>(
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
            let mut current_block_34: u64;
            match hi as c_int {
                0 => {
                    if (lo as c_int) < 0x80 {
                        if to.is_empty() {
                            *fromP = from;
                            return XML_CONVERT_OUTPUT_EXHAUSTED;
                        }
                        to[0] = lo as c_char;
                        to.inc_start(1);
                        current_block_34 = 18435049525520518667;
                    } else {
                        current_block_34 = 11412679543673842523;
                    }
                }
                1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                    current_block_34 = 11412679543673842523;
                }
                216 | 217 | 218 | 219 => {
                    if (to.len() as c_long) < 4 {
                        *fromP = from;
                        return XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    if (from.len() as c_long) < 4 {
                        *fromP = from;
                        return XML_CONVERT_INPUT_INCOMPLETE;
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
                    current_block_34 = 18435049525520518667;
                }
                _ => {
                    if (to.len() as c_long) < 3 {
                        *fromP = from;
                        return XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    to[0] = (hi as c_int >> 4 | UTF8_cval3 as c_int) as c_char;
                    to.inc_start(1);
                    to[0] = ((hi as c_int & 0xf) << 2 | lo as c_int >> 6 | 0x80) as c_char;
                    to.inc_start(1);
                    to[0] = (lo as c_int & 0x3f | 0x80) as c_char;
                    to.inc_start(1);
                    current_block_34 = 18435049525520518667;
                }
            }
            match current_block_34 {
                11412679543673842523 => {
                    if (to.len() as c_long) < 2 {
                        *fromP = from;
                        return XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    to[0] = (lo as c_int >> 6 | (hi as c_int) << 2 | UTF8_cval2 as c_int) as c_char;
                    to.inc_start(1);
                    to[0] = (lo as c_int & 0x3f | 0x80) as c_char;
                    to.inc_start(1);
                }
                _ => {}
            }
            from = from.inc_start(2)
        }
        *fromP = from;
        if !from.is_empty() {
            XML_CONVERT_INPUT_INCOMPLETE
        } else {
            XML_CONVERT_COMPLETED
        }
    }

    #[inline]
    unsafe fn utf16Convert(
        &self,
        from: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
        *from = from.with_len(((from.len() as c_long >> 1) << 1) as usize);
        if from.len() as c_long > ((to.len() as c_long) << 1)
            && from[from.len()-2] as c_uchar as c_int & 0xf8 == 0xd8
        {
            *from = from.dec_end(2);
            res = XML_CONVERT_INPUT_INCOMPLETE
        }
        while !from.is_empty() && !to.is_empty() {
            to[0] = ((from[0] as c_uchar as c_int) << 8
                     | from[1] as c_uchar as c_int) as c_ushort;
            to.inc_start(1);
            *from = from.inc_start(2);
        }
        if to.is_empty() && !from.is_empty() {
            XML_CONVERT_OUTPUT_EXHAUSTED
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
unsafe fn latin1_toUtf16(
    mut from: &mut ExpatBufRef,
    to: &mut ExpatBufRefMut<u16>,
) -> XML_Convert_Result {
    while !from.is_empty() && to.is_empty() {
        to[0] = from[0] as u16;
        to.inc_start(1);
        *from = from.inc_start(1);
    }
    if to.is_empty() && !from.is_empty() {
        XML_CONVERT_OUTPUT_EXHAUSTED
    } else {
        XML_CONVERT_COMPLETED
    }
}

#[derive(Clone)]
pub struct InitEncoding {
    encoding_index: c_int,

    encoding_table: &'static [&'static ENCODING],

    encPtr: *mut *const ENCODING,
}

impl InitEncoding {
    unsafe fn new_impl(
        encPtr: *mut *const ENCODING,
        mut name: *const c_char,
        encoding_table: &'static [&'static ENCODING],
    ) -> Option<InitEncoding> {
        let mut i: c_int = getEncodingIndex(name);
        if i == UNKNOWN_ENC {
            return None;
        }
        Some(Self {
            encoding_index: i,
            encoding_table,
            encPtr,
        })
    }

    pub unsafe fn new_ns(
        encPtr: *mut *const ENCODING,
        mut name: *const c_char,
    ) -> Option<InitEncoding> {
        Self::new_impl(encPtr, name, encodingsNS.as_ref().unwrap())
    }

    pub unsafe fn new(
        encPtr: *mut *const ENCODING,
        mut name: *const c_char,
    ) -> Option<InitEncoding> {
        Self::new_impl(encPtr, name, encodings.as_ref().unwrap())
    }

    /* This is what detects the encoding.  encodingTable maps from
    encoding indices to encodings; INIT_ENC_INDEX(enc) is the index of
    the external (protocol) specified encoding; state is
    XML_CONTENT_STATE if we're parsing an external text entity, and
    XML_PROLOG_STATE otherwise.
     */
    unsafe fn initScan(
        &self,
        mut state: c_int,
        buf: ExpatBufRef,
        mut nextTokPtr: *mut *const c_char,
    ) -> c_int {
        if buf.is_empty() {
            return crate::xmltok_h::XML_TOK_NONE;
        }
        if buf.len() == 1 {
            /* only a single byte available for auto-detection */
            /* FIXME */
            /* so we're parsing an external text entity... */
            /* if UTF-16 was externally specified, then we need at least 2 bytes */
            match self.encoding_index as c_int {
                3 | 5 | 4 => return crate::xmltok_h::XML_TOK_PARTIAL,
                _ => {}
            }
            let mut current_block_5: u64;
            match buf[0] as c_uchar as c_int {
                254 | 255 | 239 => {
                    /* possibly first byte of UTF-8 BOM */
                    if self.encoding_index as c_int == ISO_8859_1_ENC && state == XML_CONTENT_STATE {
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
            match (buf[0] as c_uchar as c_int) << 8 | buf[1] as c_uchar as c_int {
                65279 => {
                    if !(self.encoding_index as c_int == ISO_8859_1_ENC && state == XML_CONTENT_STATE)
                    {
                        *nextTokPtr = buf.as_ptr().offset(2);
                        *self.encPtr = &*self.encoding_table[UTF_16BE_ENC as usize];
                        return crate::xmltok_h::XML_TOK_BOM;
                    }
                }
                15360 => {
                    /* 00 3C is handled in the default case */
                    if !((self.encoding_index as c_int == UTF_16BE_ENC
                          || self.encoding_index as c_int == UTF_16_ENC)
                         && state == XML_CONTENT_STATE)
                    {
                        *self.encPtr = &*self.encoding_table[UTF_16LE_ENC as usize];
                        return (**self.encPtr).xmlTok(state, buf, nextTokPtr);
                    }
                }
                65534 => {
                    if !(self.encoding_index as c_int == ISO_8859_1_ENC && state == XML_CONTENT_STATE)
                    {
                        *nextTokPtr = buf.as_ptr().offset(2);
                        *self.encPtr = &*self.encoding_table[UTF_16LE_ENC as usize];
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
                        let mut e: c_int = self.encoding_index as c_int;
                        if e == ISO_8859_1_ENC
                            || e == UTF_16BE_ENC
                            || e == UTF_16LE_ENC
                            || e == UTF_16_ENC
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
                            if buf.len() == 2 {
                                return crate::xmltok_h::XML_TOK_PARTIAL;
                            }
                            if buf[2] as c_uchar as c_int == 0xbf {
                                *nextTokPtr = buf.as_ptr().offset(3);
                                *self.encPtr = &*self.encoding_table[UTF_8_ENC as usize];
                                return crate::xmltok_h::XML_TOK_BOM;
                            }
                        }
                    }
                }
                _ => {
                    if buf[0] as c_int == '\u{0}' as i32 {
                        /* 0 isn't a legal data character. Furthermore a document
                        entity can only start with ASCII characters.  So the only
                        way this can fail to be big-endian UTF-16 if it it's an
                        external parsed general entity that's labelled as
                        UTF-16LE.
                         */
                        if !(state == XML_CONTENT_STATE
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
                        if !(state == XML_CONTENT_STATE) {
                            *self.encPtr = &*self.encoding_table[UTF_16LE_ENC as usize];
                            return (**self.encPtr).xmlTok(state, buf, nextTokPtr);
                        }
                    }
                }
            }
        }
        *self.encPtr = &*self.encoding_table[self.encoding_index as c_int as usize];
        (**self.encPtr).xmlTok(state, buf, nextTokPtr)
    }

}

impl XmlEncoding for InitEncoding {
    // scanners[4]
    unsafe fn prologTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        self.initScan(
            XML_PROLOG_STATE,
            buf,
            nextTokPtr,
        )
    }
    unsafe fn contentTok(
        &self,
        buf: ExpatBufRef,
        nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        self.initScan(
            XML_CONTENT_STATE,
            buf,
            nextTokPtr,
        )
    }
    unsafe fn cdataSectionTok(
        &self,
        _buf: ExpatBufRef,
        _nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        0
    }
    unsafe fn ignoreSectionTok(
        &self,
        _buf: ExpatBufRef,
        _nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        0
    }

    // literalScanners[2]
    unsafe fn attributeValueTok(
        &self,
        _buf: ExpatBufRef,
        _nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        0
    }
    unsafe fn entityValueTok(
        &self,
        _buf: ExpatBufRef,
        _nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        0
    }

    fn nameMatchesAscii(
        &self,
        _buf: ExpatBufRef,
        _ptr2: &[libc::c_char],
    ) -> libc::c_int {
        0
    }

    unsafe fn nameLength(&self, _ptr: *const libc::c_char) -> libc::c_int {
        0
    }

    unsafe fn skipS(&self, _ptr: *const libc::c_char) -> *const libc::c_char {
        std::ptr::null()
    }

    unsafe fn getAtts(
        &self,
        _buf: ExpatBufRef,
        _f: &mut dyn FnMut(ATTRIBUTE) -> XML_Error,
    ) -> XML_Error {
        XML_Error::NONE
    }

    unsafe fn charRefNumber(&self, _buf: ExpatBufRef) -> libc::c_int {
        0
    }

    unsafe fn predefinedEntityName(
        &self,
        _buf: ExpatBufRef,
    ) -> libc::c_int {
        0
    }

    unsafe fn updatePosition(
        &self,
        buf: ExpatBufRef,
        pos: *mut POSITION,
    ) {
        utf8_encoding
            .as_ref()
            .unwrap()
            .updatePosition(buf, pos);
    }

    unsafe fn isPublicId(
        &self,
        _buf: ExpatBufRef,
        _badPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        0
    }

    unsafe fn utf8Convert<'b, 'a: 'b>(
        &self,
        _from_buf: &mut ExpatBufRef<'a>,
        _to_buf: &'b mut ExpatBufRefMut<'a>,
    ) -> XML_Convert_Result {
        0
    }

    unsafe fn utf16Convert(
        &self,
        _from_buf: &mut ExpatBufRef,
        _to_buf: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        0
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
    convert: CONVERTER,
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
        convert: CONVERTER,
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
                self.utf8[i][0] = MOZ_XmlUtf8Encode(c, &mut self.utf8[i][1..]) as c_char;
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
    unsafe fn utf8Convert<'a: 'b, 'b>(
        &self,
        from_buf: &'b mut ExpatBufRef<'a>,
        to: &'b mut ExpatBufRefMut<'a>,
    ) -> XML_Convert_Result {
        let mut buf: [c_char; 4] = [0; 4];
        loop {
            let mut n: c_int = 0;
            if from_buf.is_empty() {
                return XML_CONVERT_COMPLETED;
            }
            let mut utf8: ExpatBufRef = self.utf8[from_buf[0] as c_uchar as usize][..].into();
            n = utf8[0] as c_int;
            utf8 = utf8.inc_start(1);
            if n == 0 {
                let mut c: c_int =
                    self.convert.expect("non-null function pointer")(self.userData, from_buf.as_ptr());
                n = MOZ_XmlUtf8Encode(c, &mut buf);
                if n as c_long > to.len() as c_long {
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                utf8 = buf[..].into();
                *from_buf = from_buf.inc_start(
                    (self.types[from_buf[0] as c_uchar as usize] as c_int
                     - (ByteType::LEAD2 as c_int - 2)) as isize);
            } else {
                if n as c_long > to.len() as c_long {
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                *from_buf = from_buf.inc_start(1);
            }
            to[..n as usize].copy_from_slice(&utf8[..n as usize]);
            to.inc_start(n as usize);
        }
    }

    #[inline]
    unsafe fn utf16Convert(
        &self,
        from_buf: &mut ExpatBufRef,
        to: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        while !from_buf.is_empty() && !to.is_empty() {
            let mut c: c_ushort = self.utf16[from_buf[0] as c_uchar as usize];
            if c as c_int == 0 {
                c = self.convert.expect("non-null function pointer")(self.userData, from_buf.as_ptr())
                    as c_ushort;
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
            XML_CONVERT_OUTPUT_EXHAUSTED
        } else {
            XML_CONVERT_COMPLETED
        }
    }
}



static mut latin1_encoding: Option<Box<dyn XmlEncoding>> = None;
static mut latin1_encoding_ns: Option<Box<dyn XmlEncoding>> = None;
static mut utf8_encoding: Option<Box<dyn XmlEncoding>> = None;
static mut utf8_encoding_ns: Option<Box<dyn XmlEncoding>> = None;
static mut internal_utf8_encoding: Option<Box<InternalUtf8Encoding>> = None;
static mut internal_utf8_encoding_ns: Option<Box<InternalUtf8EncodingNS>> = None;
static mut ascii_encoding: Option<Box<dyn XmlEncoding>> = None;
static mut ascii_encoding_ns: Option<Box<dyn XmlEncoding>> = None;
static mut little2_encoding: Option<Box<dyn XmlEncoding>> = None;
static mut little2_encoding_ns: Option<Box<dyn XmlEncoding>> = None;
#[cfg(target_endian = "little")]
static mut internal_little2_encoding: Option<Box<InternalLittle2Encoding>> = None;
#[cfg(target_endian = "little")]
static mut internal_little2_encoding_ns: Option<Box<InternalLittle2EncodingNS>> = None;
static mut big2_encoding: Option<Box<dyn XmlEncoding>> = None;
static mut big2_encoding_ns: Option<Box<dyn XmlEncoding>> = None;
#[cfg(target_endian = "big")]
static mut internal_big2_encoding: Option<Box<InternalBig2Encoding>> = None;
#[cfg(target_endian = "big")]
static mut internal_big2_encoding_ns: Option<Box<InternalBig2EncodingNS>> = None;

pub fn MOZ_XmlGetUtf8InternalEncodingNS() -> &'static ENCODING {
    unsafe { &**internal_utf8_encoding_ns.as_ref().unwrap() }
}

pub fn MOZ_XmlGetUtf8InternalEncoding() -> &'static ENCODING {
    unsafe { &**internal_utf8_encoding.as_ref().unwrap() }
}

#[cfg(target_endian = "little")]
pub fn MOZ_XmlGetUtf16InternalEncoding() -> &'static ENCODING {
    unsafe { &**internal_little2_encoding.as_ref().unwrap() }
}

#[cfg(target_endian = "big")]
pub fn MOZ_XmlGetUtf16InternalEncoding() -> &'static ENCODING {
    unsafe { &**internal_big2_encoding.as_ref().unwrap() }
}

#[cfg(target_endian = "little")]
pub fn MOZ_XmlGetUtf16InternalEncodingNS() -> &'static ENCODING {
    unsafe { &**internal_little2_encoding_ns.as_ref().unwrap() }
}

#[cfg(target_endian = "big")]
pub fn MOZ_XmlGetUtf16InternalEncodingNS() -> &'static ENCODING {
    unsafe { &**internal_big2_encoding_ns.as_ref().unwrap() }
}

// Initialized in run_static_initializers
pub static mut encodingsNS: Option<[&'static ENCODING; 7]> = None;

// Initialized in run_static_initializers
pub static mut encodings: Option<[&'static ENCODING; 7]> = None;

pub unsafe fn findEncoding(
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
    *p.as_mut_ptr() = 0;
    if streqci(out_buf.as_ptr(), KW_UTF_16.as_ptr()) != 0 && (*enc).minBytesPerChar() == 2 {
        return Some(enc.clone());
    }
    i = getEncodingIndex(out_buf.as_ptr());
    if i == UNKNOWN_ENC {
        None
    } else {
        Some(encodings.unwrap()[i as usize])
    }
}

pub unsafe fn findEncodingNS(
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
    *p.as_mut_ptr() = 0;
    if streqci(out_buf.as_ptr(), KW_UTF_16.as_ptr()) != 0 && (*enc).minBytesPerChar() == 2 {
        return Some(enc.clone());
    }
    i = getEncodingIndex(out_buf.as_ptr());
    if i == UNKNOWN_ENC {
        None
    } else {
        Some(encodingsNS.unwrap()[i as usize])
    }
}

pub unsafe fn MOZ_XmlParseXmlDeclNS<'a>(
    mut isGeneralTextEntity: c_int,
    mut enc: &ENCODING,
    mut buf: ExpatBufRef<'a>,
    mut badPtr: *mut *const c_char,
    mut versionBuf: &mut Option<ExpatBufRef<'a>>,
    mut encodingName: *mut *const c_char,
    mut encoding: &mut Option<*const ENCODING>,
    mut standalone: *mut c_int,
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

pub unsafe fn MOZ_XmlParseXmlDecl<'a>(
    mut isGeneralTextEntity: c_int,
    mut enc: &ENCODING,
    mut buf: ExpatBufRef<'a>,
    mut badPtr: *mut *const c_char,
    mut versionBuf: &mut Option<ExpatBufRef<'a>>,
    mut encodingName: *mut *const c_char,
    mut encoding: &mut Option<*const ENCODING>,
    mut standalone: *mut c_int,
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
pub use crate::stddef_h::{ptrdiff_t, size_t, NULL};
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
pub unsafe fn _INTERNAL_trim_to_complete_utf8_characters(
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
unsafe fn unicode_byte_type(mut hi: c_char, mut lo: c_char) -> ByteType {
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

unsafe fn initUpdatePosition(
    mut _enc: &ENCODING,
    buf: ExpatBufRef,
    mut pos: *mut POSITION,
) {
    utf8_encoding
        .as_ref()
        .unwrap()
        .updatePosition(buf, pos);
}

unsafe fn toAscii(
    mut enc: &ENCODING,
    mut buf: ExpatBufRef,
) -> c_int {
    let mut out_buf: [c_char; 1] = [0; 1];
    let mut p = ExpatBufRefMut::new(
        &mut out_buf[0],
        out_buf.as_mut_ptr().offset(1),
    );
    (*enc).utf8Convert(&mut buf, &mut p);
    if p.as_ptr() == out_buf.as_ptr() {
        return -1;
    } else {
        return out_buf[0] as c_int;
    };
}

unsafe fn isSpace(mut c: c_int) -> c_int {
    match c {
        32 | 13 | 10 | 9 => return 1,
        _ => {}
    }
    0
}
/* Return 1 if there's just optional white space or there's an S
   followed by name=val.
*/

unsafe fn parsePseudoAttribute<'a>(
    mut enc: &ENCODING,
    mut buf: ExpatBufRef<'a>,
    mut name: &mut Option<ExpatBufRef<'a>>,
    mut val: *mut Option<ExpatBufRef<'a>>,
    mut nextTokPtr: *mut *const c_char,
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

unsafe fn doParseXmlDecl<'a>(
    mut encodingFinder: Option<
        unsafe fn(
            _: &ENCODING,
            _: ExpatBufRef,
        ) -> Option<*const ENCODING>,
    >,
    mut isGeneralTextEntity: c_int,
    mut enc: &ENCODING,
    mut buf: ExpatBufRef<'a>,
    mut badPtr: *mut *const c_char,
    mut versionBuf: &mut Option<ExpatBufRef<'a>>,
    mut encodingName: *mut *const c_char,
    mut encoding: *mut Option<*const ENCODING>,
    mut standalone: *mut c_int,
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
    if (*enc).nameMatchesAscii(name.unwrap(), &KW_version) == 0 {
        if isGeneralTextEntity == 0 {
            *badPtr = name.map_or(ptr::null(), |x| x.as_ptr());
            return 0;
        }
    } else {
        *versionBuf = val_buf;
        #[cfg(feature = "mozilla")]
        {
            if (*enc).nameMatchesAscii(val_buf
                                       .unwrap()
                                       .with_end(pseudo_ptr)
                                       .dec_end((*enc).minBytesPerChar() as usize),
                                       &KW_XML_1_0) == 0
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
    if (*enc).nameMatchesAscii(name.unwrap(), &KW_encoding) != 0 {
        let mut c: c_int = toAscii(enc, val_buf.unwrap());
        if !((ASCII_a as c_int) <= c && c <= (ASCII_z as c_int))
            && !((ASCII_A as c_int) <= c && c <= (ASCII_Z as c_int))
        {
            *badPtr = val_buf.map_or(ptr::null(), |x| x.as_ptr());
            return 0;
        }
        if !encodingName.is_null() {
            *encodingName = val_buf.unwrap().as_ptr();
        }
        if !encoding.is_null() {
            *encoding = encodingFinder.expect("non-null function pointer")(
                enc,
                val_buf
                    .unwrap()
                    .with_end(buf.as_ptr())
                    .dec_end(((*enc).minBytesPerChar()) as usize),
            )
        }
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
    if (*enc).nameMatchesAscii(name.unwrap(), &KW_standalone) == 0
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
    ) != 0
    {
        if !standalone.is_null() {
            *standalone = 1
        }
    } else if (*enc).nameMatchesAscii(
        val_buf
            .unwrap()
            .with_end(buf.as_ptr())
            .dec_end(((*enc).minBytesPerChar()) as usize),
        &KW_no,
    ) != 0
    {
        if !standalone.is_null() {
            *standalone = 0
        }
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
        216 | 217 | 218 | 219 | 220 | 221 | 222 | 223 => return -1,
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

pub fn MOZ_XmlUtf8Encode(mut c: c_int, buf: &mut [c_char]) -> c_int {
    if c < 0 {
        return 0;
    }
    if c < min2 as c_int {
        buf[0] = (c | UTF8_cval1 as c_int) as c_char;
        return 1;
    }
    if c < min3 as c_int {
        buf[0] = (c >> 6 | UTF8_cval2 as c_int) as c_char;
        buf[1] = (c & 0x3fi32 | 0x80) as c_char;
        return 2;
    }
    if c < min4 as c_int {
        buf[0] = (c >> 12 | UTF8_cval3 as c_int) as c_char;
        buf[1] = (c >> 6 & 0x3fi32 | 0x80) as c_char;
        buf[2] = (c & 0x3fi32 | 0x80) as c_char;
        return 3;
    }
    if c < 0x110000 {
        buf[0] = (c >> 18 | UTF8_cval4 as c_int) as c_char;
        buf[1] = (c >> 12 & 0x3fi32 | 0x80) as c_char;
        buf[2] = (c >> 6 & 0x3fi32 | 0x80) as c_char;
        buf[3] = (c & 0x3fi32 | 0x80) as c_char;
        return 4;
    }
    0
    /* LCOV_EXCL_LINE: this case too is eliminated before calling */
}

pub fn MOZ_XmlUtf16Encode(mut charNum: c_int, buf: &mut [c_ushort]) -> c_int {
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
        buf[1] = ((charNum & 0x3ffi32) + 0xdc00) as c_ushort;
        return 2;
    }
    0
}

pub unsafe fn MOZ_XmlSizeOfUnknownEncoding() -> c_int {
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

unsafe fn run_static_initializers() {
    latin1_encoding = Some(Box::new(Latin1Encoding::new()));
    latin1_encoding_ns = Some(Box::new(Latin1EncodingNS::new()));
    utf8_encoding = Some(Box::new(Utf8Encoding::new()));
    utf8_encoding_ns = Some(Box::new(Utf8EncodingNS::new()));
    internal_utf8_encoding = Some(Box::new(InternalUtf8Encoding::new()));
    internal_utf8_encoding_ns = Some(Box::new(InternalUtf8EncodingNS::new()));
    ascii_encoding = Some(Box::new(AsciiEncoding::new()));
    ascii_encoding_ns = Some(Box::new(AsciiEncodingNS::new()));
    little2_encoding = Some(Box::new(Little2Encoding::new()));
    little2_encoding_ns = Some(Box::new(Little2EncodingNS::new()));
    #[cfg(target_endian = "little")]
    {
        internal_little2_encoding = Some(Box::new(InternalLittle2Encoding::new()));
        internal_little2_encoding_ns = Some(Box::new(InternalLittle2EncodingNS::new()));
    }
    big2_encoding = Some(Box::new(Big2Encoding::new()));
    big2_encoding_ns = Some(Box::new(Big2EncodingNS::new()));
    #[cfg(target_endian = "big")]
    {
        internal_big2_encoding = Some(Box::new(InternalBig2Encoding::new()));
        internal_big2_encoding_ns = Some(Box::new(InternalBig2EncodingNS::new()));
    }
    encodingsNS = Some([
        &**latin1_encoding_ns.as_ref().unwrap(),
        &**ascii_encoding_ns.as_ref().unwrap(),
        &**utf8_encoding_ns.as_ref().unwrap(),
        &**big2_encoding_ns.as_ref().unwrap(),
        &**big2_encoding_ns.as_ref().unwrap(),
        &**little2_encoding_ns.as_ref().unwrap(),
        &**utf8_encoding_ns.as_ref().unwrap(),
    ]);
    encodings = Some([
        &**latin1_encoding.as_ref().unwrap(),
        &**ascii_encoding.as_ref().unwrap(),
        &**utf8_encoding.as_ref().unwrap(),
        &**big2_encoding.as_ref().unwrap(),
        &**big2_encoding.as_ref().unwrap(),
        &**little2_encoding.as_ref().unwrap(),
        &**utf8_encoding.as_ref().unwrap(),
    ]);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
// XML_NS
