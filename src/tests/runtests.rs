/* Run the Expat test suite
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
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(
    const_raw_ptr_to_usize_cast,
    const_transmute,
    label_break_value,
    main,
    ptr_wrapping_offset_from,
    register_tool
)]

#[cfg(all(feature = "unicode", not(feature = "unicode_wchar_t")))]
compile_error!("Tests are not compatible with feature \"unicode\" without 16-bit char support (\"unicode_wchar_t\")");

use crate::stdlib::{stderr, strncmp};
use ::rexpat::ascii_h::{ASCII_0, ASCII_9, ASCII_PERIOD};
use ::rexpat::expat_h::{
    XML_Encoding, XML_Expat_Version, XML_Feature, XML_ParserStruct,
    XML_ParsingStatus, XML_ERROR_ABORTED, XML_ERROR_ASYNC_ENTITY, XML_ERROR_BAD_CHAR_REF,
    XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING, XML_ERROR_DUPLICATE_ATTRIBUTE,
    XML_ERROR_EXTERNAL_ENTITY_HANDLING, XML_ERROR_FINISHED, XML_ERROR_INCORRECT_ENCODING,
    XML_ERROR_INVALID_TOKEN, XML_ERROR_MISPLACED_XML_PI, XML_ERROR_NONE, XML_ERROR_NOT_STANDALONE,
    XML_ERROR_NOT_SUSPENDED, XML_ERROR_NO_ELEMENTS, XML_ERROR_NO_MEMORY, XML_ERROR_PARTIAL_CHAR,
    XML_ERROR_PUBLICID, XML_ERROR_RECURSIVE_ENTITY_REF, XML_ERROR_RESERVED_NAMESPACE_URI,
    XML_ERROR_RESERVED_PREFIX_XML, XML_ERROR_RESERVED_PREFIX_XMLNS, XML_ERROR_SUSPENDED,
    XML_ERROR_SUSPEND_PE, XML_ERROR_SYNTAX, XML_ERROR_TAG_MISMATCH, XML_ERROR_TEXT_DECL,
    XML_ERROR_UNBOUND_PREFIX, XML_ERROR_UNCLOSED_CDATA_SECTION, XML_ERROR_UNCLOSED_TOKEN,
    XML_ERROR_UNDECLARING_PREFIX, XML_ERROR_UNDEFINED_ENTITY, XML_ERROR_UNKNOWN_ENCODING,
    XML_ERROR_XML_DECL, XML_FALSE, XML_FEATURE_CONTEXT_BYTES, XML_FEATURE_END, XML_FINISHED,
    XML_INITIALIZED, XML_PARAM_ENTITY_PARSING_ALWAYS, XML_PARAM_ENTITY_PARSING_NEVER,
    XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE, XML_STATUS_ERROR, XML_STATUS_ERROR_0,
    XML_STATUS_OK_0, XML_STATUS_SUSPENDED_0, XML_SUSPENDED, XML_TRUE,
};
use ::rexpat::siphash_h::{siphash, sipkey};
use ::rexpat::lib::xmlparse::{
    XML_DefaultCurrent, XML_ErrorString, XML_ExpatVersion, XML_ExpatVersionInfo,
    XML_ExternalEntityParserCreate, XML_FreeContentModel, XML_GetBase, XML_GetBuffer,
    XML_GetCurrentByteCount, XML_GetCurrentByteIndex, XML_GetCurrentColumnNumber,
    XML_GetCurrentLineNumber, XML_GetErrorCode, XML_GetFeatureList, XML_GetIdAttributeIndex,
    XML_GetInputContext, XML_GetParsingStatus, XML_GetSpecifiedAttributeCount, XML_MemFree,
    XML_MemMalloc, XML_MemRealloc, XML_Parse, XML_ParseBuffer, XML_ParserCreate,
    XML_ParserCreateNS, XML_ParserCreate_MM, XML_ParserFree, XML_ParserReset, XML_ResumeParser,
    XML_SetAttlistDeclHandler, XML_SetBase, XML_SetCdataSectionHandler,
    XML_SetCharacterDataHandler, XML_SetCommentHandler, XML_SetDefaultHandler,
    XML_SetDefaultHandlerExpand, XML_SetDoctypeDeclHandler, XML_SetElementDeclHandler,
    XML_SetElementHandler, XML_SetEncoding, XML_SetEndCdataSectionHandler,
    XML_SetEndDoctypeDeclHandler, XML_SetEndElementHandler, XML_SetEndNamespaceDeclHandler,
    XML_SetEntityDeclHandler, XML_SetExternalEntityRefHandler, XML_SetExternalEntityRefHandlerArg,
    XML_SetHashSalt, XML_SetNamespaceDeclHandler, XML_SetNotStandaloneHandler,
    XML_SetNotationDeclHandler, XML_SetParamEntityParsing, XML_SetProcessingInstructionHandler,
    XML_SetReturnNSTriplet, XML_SetSkippedEntityHandler, XML_SetStartCdataSectionHandler,
    XML_SetStartDoctypeDeclHandler, XML_SetStartElementHandler, XML_SetStartNamespaceDeclHandler,
    XML_SetUnknownEncodingHandler, XML_SetUnparsedEntityDeclHandler, XML_SetUserData,
    XML_SetXmlDeclHandler, XML_StopParser, XML_UseForeignDTD, XML_UseParserAsHandlerArg, INT_MAX,
};
use ::rexpat::lib::xmltok::_INTERNAL_trim_to_complete_utf8_characters;
use ::rexpat::stdbool_h::{false_0, true_0};
use ::rexpat::stdlib::{__assert_fail, fprintf, malloc, memcmp, memcpy, realloc, strlen};
pub use ::rexpat::*;
use ::libc::{free, printf, sprintf, strcmp, EXIT_FAILURE, EXIT_SUCCESS};
use ::rexpat::lib::xmlparse::ExpatBufRef;

use std::alloc::{GlobalAlloc, Layout, System};
use std::mem::transmute;

use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};

pub mod chardata;
pub mod memcheck;
pub mod minicheck;
pub mod structdata;

pub mod minicheck_h {
    use libc::c_int;
    pub const CK_SILENT: c_int = 0;

    pub const CK_NORMAL: c_int = 1;
    /* Workaround for Microsoft's compiler and Tru64 Unix systems where the
    C compiler has a working __func__, but the C++ compiler only has a
    working __FUNCTION__.  This could be fixed in configure.in, but it's
    not worth it right now. */

    pub type tcase_setup_function = Option<unsafe extern "C" fn() -> ()>;

    pub type tcase_teardown_function = Option<unsafe extern "C" fn() -> ()>;

    pub type tcase_test_function = Option<unsafe extern "C" fn() -> ()>;
}

pub mod siphash_h {

    /* struct sipkey */

    use crate::stddef_h::size_t;
    use crate::stdlib::uint64_t;
    use ::rexpat::siphash_h::{siphash, sipkey};
    use libc::{c_char, c_int, c_uchar, c_ulong, c_void};
    pub unsafe extern "C" fn sip_tokey(
        mut key: *mut sipkey,
        mut src: *const c_void,
    ) -> *mut sipkey {
        (*key).k[0] = (*(src as *const c_uchar).offset(0) as uint64_t) << 0
            | (*(src as *const c_uchar).offset(1) as uint64_t) << 8
            | (*(src as *const c_uchar).offset(2) as uint64_t) << 16
            | (*(src as *const c_uchar).offset(3) as uint64_t) << 24
            | (*(src as *const c_uchar).offset(4) as uint64_t) << 32
            | (*(src as *const c_uchar).offset(5) as uint64_t) << 40
            | (*(src as *const c_uchar).offset(6) as uint64_t) << 48
            | (*(src as *const c_uchar).offset(7) as uint64_t) << 56;
        (*key).k[1] = (*(src as *const c_uchar).offset(8).offset(0) as uint64_t) << 0
            | (*(src as *const c_uchar).offset(8).offset(1) as uint64_t) << 8
            | (*(src as *const c_uchar).offset(8).offset(2) as uint64_t) << 16
            | (*(src as *const c_uchar).offset(8).offset(3) as uint64_t) << 24
            | (*(src as *const c_uchar).offset(8).offset(4) as uint64_t) << 32
            | (*(src as *const c_uchar).offset(8).offset(5) as uint64_t) << 40
            | (*(src as *const c_uchar).offset(8).offset(6) as uint64_t) << 48
            | (*(src as *const c_uchar).offset(8).offset(7) as uint64_t) << 56;
        return key;
    }
    /* sip_tokey() */
    /* SIPHASH_TOBIN */

    pub unsafe extern "C" fn sip_round(mut H: *mut siphash, rounds: c_int) {
        let mut i: c_int = 0;
        i = 0;
        while i < rounds {
            (*H).v0 = ((*H).v0).wrapping_add((*H).v1);
            (*H).v1 = (*H).v1 << 13 | (*H).v1 >> 64 - 13;
            (*H).v1 ^= (*H).v0;
            (*H).v0 = (*H).v0 << 32 | (*H).v0 >> 64 - 32;
            (*H).v2 = ((*H).v2).wrapping_add((*H).v3);
            (*H).v3 = (*H).v3 << 16 | (*H).v3 >> 64 - 16;
            (*H).v3 ^= (*H).v2;
            (*H).v0 = ((*H).v0).wrapping_add((*H).v3);
            (*H).v3 = (*H).v3 << 21 | (*H).v3 >> 64 - 21;
            (*H).v3 ^= (*H).v0;
            (*H).v2 = ((*H).v2).wrapping_add((*H).v1);
            (*H).v1 = (*H).v1 << 17 | (*H).v1 >> 64 - 17;
            (*H).v1 ^= (*H).v2;
            (*H).v2 = (*H).v2 << 32 | (*H).v2 >> 64 - 32;
            i += 1
        }
    }
    /* sip_round() */

    pub unsafe extern "C" fn sip24_init(
        mut H: *mut siphash,
        mut key: *const sipkey,
    ) -> *mut siphash {
        (*H).v0 = ((0x736f6d65u64) << 32 | 0x70736575) ^ (*key).k[0];
        (*H).v1 = ((0x646f7261u64) << 32 | 0x6e646f6d) ^ (*key).k[1];
        (*H).v2 = ((0x6c796765u64) << 32 | 0x6e657261) ^ (*key).k[0];
        (*H).v3 = ((0x74656462u64) << 32 | 0x79746573) ^ (*key).k[1];
        (*H).p = (*H).buf.as_mut_ptr();
        (*H).c = 0u64;
        return H;
    }
    /* sip24_init() */

    pub unsafe extern "C" fn sip24_update(
        mut H: *mut siphash,
        mut src: *const c_void,
        mut len: size_t,
    ) -> *mut siphash {
        let mut p: *const c_uchar = src as *const c_uchar;
        let mut pe: *const c_uchar = p.offset(len as isize);
        let mut m: uint64_t = 0;
        loop {
            while p < pe
                && (*H).p
                    < &mut *(*H).buf.as_mut_ptr().offset(
                        (::std::mem::size_of::<[c_uchar; 8]>() as c_ulong)
                            .wrapping_div(::std::mem::size_of::<c_uchar>() as c_ulong)
                            as isize,
                    ) as *mut c_uchar
            {
                let fresh0 = p;
                p = p.offset(1);
                let fresh1 = (*H).p;
                (*H).p = (*H).p.offset(1);
                *fresh1 = *fresh0
            }
            if (*H).p
                < &mut *(*H).buf.as_mut_ptr().offset(
                    (::std::mem::size_of::<[c_uchar; 8]>() as c_ulong)
                        .wrapping_div(::std::mem::size_of::<c_uchar>() as c_ulong)
                        as isize,
                ) as *mut c_uchar
            {
                break;
            }
            m = ((*H).buf[0] as uint64_t) << 0
                | ((*H).buf[1] as uint64_t) << 8
                | ((*H).buf[2] as uint64_t) << 16
                | ((*H).buf[3] as uint64_t) << 24
                | ((*H).buf[4] as uint64_t) << 32
                | ((*H).buf[5] as uint64_t) << 40
                | ((*H).buf[6] as uint64_t) << 48
                | ((*H).buf[7] as uint64_t) << 56;
            (*H).v3 ^= m;
            sip_round(H, 2);
            (*H).v0 ^= m;
            (*H).p = (*H).buf.as_mut_ptr();
            (*H).c = ((*H).c).wrapping_add(8u64);
            if !(p < pe) {
                break;
            }
        }
        return H;
    }
    /* sip24_update() */

    pub unsafe extern "C" fn sip24_final(mut H: *mut siphash) -> uint64_t {
        let left: c_char = (*H).p.wrapping_offset_from((*H).buf.as_mut_ptr()) as c_char;
        let mut b: uint64_t = (*H).c.wrapping_add(left as c_ulong) << 56;
        let mut current_block_6: u64;
        match left as c_int {
            7 => {
                b |= ((*H).buf[6] as uint64_t) << 48;
                current_block_6 = 13145881433301908673;
            }
            6 => {
                current_block_6 = 13145881433301908673;
            }
            5 => {
                current_block_6 = 588075840077989673;
            }
            4 => {
                current_block_6 = 9865490829578899964;
            }
            3 => {
                current_block_6 = 610692000009365401;
            }
            2 => {
                current_block_6 = 5204012310284484386;
            }
            1 => {
                current_block_6 = 34749046854646975;
            }
            0 | _ => {
                current_block_6 = 17965632435239708295;
            }
        }
        match current_block_6 {
            13145881433301908673 =>
            /* fall through */
            {
                b |= ((*H).buf[5] as uint64_t) << 40;
                current_block_6 = 588075840077989673;
            }
            _ => {}
        }
        match current_block_6 {
            588075840077989673 =>
            /* fall through */
            {
                b |= ((*H).buf[4] as uint64_t) << 32;
                current_block_6 = 9865490829578899964;
            }
            _ => {}
        }
        match current_block_6 {
            9865490829578899964 =>
            /* fall through */
            {
                b |= ((*H).buf[3] as uint64_t) << 24;
                current_block_6 = 610692000009365401;
            }
            _ => {}
        }
        match current_block_6 {
            610692000009365401 =>
            /* fall through */
            {
                b |= ((*H).buf[2] as uint64_t) << 16;
                current_block_6 = 5204012310284484386;
            }
            _ => {}
        }
        match current_block_6 {
            5204012310284484386 =>
            /* fall through */
            {
                b |= ((*H).buf[1] as uint64_t) << 8;
                current_block_6 = 34749046854646975;
            }
            _ => {}
        }
        match current_block_6 {
            34749046854646975 =>
            /* fall through */
            {
                b |= ((*H).buf[0] as uint64_t) << 0
            }
            _ => {}
        }
        (*H).v3 ^= b;
        sip_round(H, 2);
        (*H).v0 ^= b;
        (*H).v2 ^= 0xffu64;
        sip_round(H, 4);
        return (*H).v0 ^ (*H).v1 ^ (*H).v2 ^ (*H).v3;
    }
    /* sip24_final() */

    pub unsafe extern "C" fn siphash24(
        mut src: *const c_void,
        mut len: size_t,
        mut key: *const sipkey,
    ) -> uint64_t {
        let mut state: siphash = {
            let mut init = siphash {
                v0: 0u64,
                v1: 0u64,
                v2: 0u64,
                v3: 0u64,
                buf: [0u8, 0, 0, 0, 0, 0, 0, 0],
                p: 0 as *mut c_uchar,
                c: 0u64,
            };
            init
        };
        return sip24_final(sip24_update(sip24_init(&mut state, key), src, len));
    }
    /* siphash24() */
    /*
     * SipHash-2-4 output with
     * k = 00 01 02 ...
     * and
     * in = (empty string)
     * in = 00 (1 byte)
     * in = 00 01 (2 bytes)
     * in = 00 01 02 (3 bytes)
     * ...
     * in = 00 01 02 ... 3e (63 bytes)
     */

    pub unsafe extern "C" fn sip24_valid() -> c_int {
        /* clang-format off */
        pub static mut vectors: [[c_uchar; 8]; 64] = [
            [0x31, 0xe, 0xe, 0xdd, 0x47, 0xdb, 0x6f, 0x72],
            [0xfd, 0x67, 0xdc, 0x93, 0xc5, 0x39, 0xf8, 0x74],
            [0x5a, 0x4f, 0xa9, 0xd9, 0x9, 0x80, 0x6c, 0xd],
            [0x2d, 0x7e, 0xfb, 0xd7, 0x96, 0x66, 0x67, 0x85],
            [0xb7, 0x87, 0x71, 0x27, 0xe0, 0x94, 0x27, 0xcf],
            [0x8d, 0xa6, 0x99, 0xcd, 0x64, 0x55, 0x76, 0x18],
            [0xce, 0xe3, 0xfe, 0x58, 0x6e, 0x46, 0xc9, 0xcb],
            [0x37, 0xd1, 0x1, 0x8b, 0xf5, 0, 0x2, 0xab],
            [0x62, 0x24, 0x93, 0x9a, 0x79, 0xf5, 0xf5, 0x93],
            [0xb0, 0xe4, 0xa9, 0xb, 0xdf, 0x82, 0, 0x9e],
            [0xf3, 0xb9, 0xdd, 0x94, 0xc5, 0xbb, 0x5d, 0x7a],
            [0xa7, 0xad, 0x6b, 0x22, 0x46, 0x2f, 0xb3, 0xf4],
            [0xfb, 0xe5, 0xe, 0x86, 0xbc, 0x8f, 0x1e, 0x75],
            [0x90, 0x3d, 0x84, 0xc0, 0x27, 0x56, 0xea, 0x14],
            [0xee, 0xf2, 0x7a, 0x8e, 0x90, 0xca, 0x23, 0xf7],
            [0xe5, 0x45, 0xbe, 0x49, 0x61, 0xca, 0x29, 0xa1],
            [0xdb, 0x9b, 0xc2, 0x57, 0x7f, 0xcc, 0x2a, 0x3f],
            [0x94, 0x47, 0xbe, 0x2c, 0xf5, 0xe9, 0x9a, 0x69],
            [0x9c, 0xd3, 0x8d, 0x96, 0xf0, 0xb3, 0xc1, 0x4b],
            [0xbd, 0x61, 0x79, 0xa7, 0x1d, 0xc9, 0x6d, 0xbb],
            [0x98, 0xee, 0xa2, 0x1a, 0xf2, 0x5c, 0xd6, 0xbe],
            [0xc7, 0x67, 0x3b, 0x2e, 0xb0, 0xcb, 0xf2, 0xd0],
            [0x88, 0x3e, 0xa3, 0xe3, 0x95, 0x67, 0x53, 0x93],
            [0xc8, 0xce, 0x5c, 0xcd, 0x8c, 0x3, 0xc, 0xa8],
            [0x94, 0xaf, 0x49, 0xf6, 0xc6, 0x50, 0xad, 0xb8],
            [0xea, 0xb8, 0x85, 0x8a, 0xde, 0x92, 0xe1, 0xbc],
            [0xf3, 0x15, 0xbb, 0x5b, 0xb8, 0x35, 0xd8, 0x17],
            [0xad, 0xcf, 0x6b, 0x7, 0x63, 0x61, 0x2e, 0x2f],
            [0xa5, 0xc9, 0x1d, 0xa7, 0xac, 0xaa, 0x4d, 0xde],
            [0x71, 0x65, 0x95, 0x87, 0x66, 0x50, 0xa2, 0xa6],
            [0x28, 0xef, 0x49, 0x5c, 0x53, 0xa3, 0x87, 0xad],
            [0x42, 0xc3, 0x41, 0xd8, 0xfa, 0x92, 0xd8, 0x32],
            [0xce, 0x7c, 0xf2, 0x72, 0x2f, 0x51, 0x27, 0x71],
            [0xe3, 0x78, 0x59, 0xf9, 0x46, 0x23, 0xf3, 0xa7],
            [0x38, 0x12, 0x5, 0xbb, 0x1a, 0xb0, 0xe0, 0x12],
            [0xae, 0x97, 0xa1, 0xf, 0xd4, 0x34, 0xe0, 0x15],
            [0xb4, 0xa3, 0x15, 0x8, 0xbe, 0xff, 0x4d, 0x31],
            [0x81, 0x39, 0x62, 0x29, 0xf0, 0x90, 0x79, 0x2],
            [0x4d, 0xc, 0xf4, 0x9e, 0xe5, 0xd4, 0xdc, 0xca],
            [0x5c, 0x73, 0x33, 0x6a, 0x76, 0xd8, 0xbf, 0x9a],
            [0xd0, 0xa7, 0x4, 0x53, 0x6b, 0xa9, 0x3e, 0xe],
            [0x92, 0x59, 0x58, 0xfc, 0xd6, 0x42, 0xc, 0xad],
            [0xa9, 0x15, 0xc2, 0x9b, 0xc8, 0x6, 0x73, 0x18],
            [0x95, 0x2b, 0x79, 0xf3, 0xbc, 0xa, 0xa6, 0xd4],
            [0xf2, 0x1d, 0xf2, 0xe4, 0x1d, 0x45, 0x35, 0xf9],
            [0x87, 0x57, 0x75, 0x19, 0x4, 0x8f, 0x53, 0xa9],
            [0x10, 0xa5, 0x6c, 0xf5, 0xdf, 0xcd, 0x9a, 0xdb],
            [0xeb, 0x75, 0x9, 0x5c, 0xcd, 0x98, 0x6c, 0xd0],
            [0x51, 0xa9, 0xcb, 0x9e, 0xcb, 0xa3, 0x12, 0xe6],
            [0x96, 0xaf, 0xad, 0xfc, 0x2c, 0xe6, 0x66, 0xc7],
            [0x72, 0xfe, 0x52, 0x97, 0x5a, 0x43, 0x64, 0xee],
            [0x5a, 0x16, 0x45, 0xb2, 0x76, 0xd5, 0x92, 0xa1],
            [0xb2, 0x74, 0xcb, 0x8e, 0xbf, 0x87, 0x87, 0xa],
            [0x6f, 0x9b, 0xb4, 0x20, 0x3d, 0xe7, 0xb3, 0x81],
            [0xea, 0xec, 0xb2, 0xa3, 0xb, 0x22, 0xa8, 0x7f],
            [0x99, 0x24, 0xa4, 0x3c, 0xc1, 0x31, 0x57, 0x24],
            [0xbd, 0x83, 0x8d, 0x3a, 0xaf, 0xbf, 0x8d, 0xb7],
            [0xb, 0x1a, 0x2a, 0x32, 0x65, 0xd5, 0x1a, 0xea],
            [0x13, 0x50, 0x79, 0xa3, 0x23, 0x1c, 0xe6, 0x60],
            [0x93, 0x2b, 0x28, 0x46, 0xe4, 0xd7, 0x6, 0x66],
            [0xe1, 0x91, 0x5f, 0x5c, 0xb1, 0xec, 0xa4, 0x6c],
            [0xf3, 0x25, 0x96, 0x5c, 0xa1, 0x6d, 0x62, 0x9f],
            [0x57, 0x5f, 0xf2, 0x8e, 0x60, 0x38, 0x1b, 0xe5],
            [0x72, 0x45, 0x6, 0xeb, 0x4c, 0x32, 0x8a, 0x95],
        ];
        /* clang-format on */
        let mut in_0: [c_uchar; 64] = [0; 64];
        let mut k: sipkey = sipkey { k: [0; 2] };
        let mut i: size_t = 0;
        sip_tokey(
            &mut k,
            b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x00".as_ptr()
                as *const c_void,
        );
        i = 0;
        while i < ::std::mem::size_of::<[c_uchar; 64]>() as c_ulong {
            in_0[i as usize] = i as c_uchar;
            if siphash24(in_0.as_mut_ptr() as *const c_void, i, &mut k)
                != (vectors[i as usize][0] as uint64_t) << 0
                    | (vectors[i as usize][1] as uint64_t) << 8
                    | (vectors[i as usize][2] as uint64_t) << 16
                    | (vectors[i as usize][3] as uint64_t) << 24
                    | (vectors[i as usize][4] as uint64_t) << 32
                    | (vectors[i as usize][5] as uint64_t) << 40
                    | (vectors[i as usize][6] as uint64_t) << 48
                    | (vectors[i as usize][7] as uint64_t) << 56
            {
                return 0i32;
            }
            i = i.wrapping_add(1)
        }
        return 1;
    }

    /* SIPHASH_H */
    /* SIPHASH_MAIN */
    /* sip24_valid() */
}

pub use crate::expat_external_h::{XML_Char, XML_Index, XML_LChar, XML_Size};
pub use crate::expat_h::{
    XML_AttlistDeclHandler, XML_Bool, XML_CharacterDataHandler, XML_CommentHandler, XML_Content,
    XML_Content_Quant, XML_Content_Type, XML_DefaultHandler, XML_ElementDeclHandler,
    XML_EndCdataSectionHandler, XML_EndDoctypeDeclHandler, XML_EndElementHandler,
    XML_EndNamespaceDeclHandler, XML_EntityDeclHandler, XML_Error, XML_ExternalEntityRefHandler,
    XML_FeatureEnum, XML_NotStandaloneHandler, XML_NotationDeclHandler, XML_ParamEntityParsing,
    XML_Parser, XML_Parsing, XML_ProcessingInstructionHandler, XML_SkippedEntityHandler,
    XML_StartCdataSectionHandler, XML_StartDoctypeDeclHandler, XML_StartElementHandler,
    XML_StartNamespaceDeclHandler, XML_Status, XML_UnknownEncodingHandler,
    XML_UnparsedEntityDeclHandler, XML_XmlDeclHandler,
};
pub use crate::minicheck_h::{
    tcase_setup_function, tcase_teardown_function, tcase_test_function, CK_NORMAL, CK_SILENT,
};
pub use crate::siphash_h::{
    sip24_final, sip24_init, sip24_update, sip24_valid, sip_round, sip_tokey, siphash24,
};
pub use crate::stddef_h::{ptrdiff_t, size_t};
pub use crate::stdlib::{
    _IO_lock_t, __off64_t, __off_t, __uint64_t, intptr_t,
    uint64_t, FILE,
};

/* Test attribute counts, indexing, etc */

pub type AttrInfo = attrInfo;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct attrInfo {
    pub name: *const XML_Char,
    pub value: *const XML_Char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtOption {
    pub system_id: *const XML_Char,
    pub parse_text: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtTest {
    pub parse_text: *const c_char,
    pub encoding: *const XML_Char,
    pub storage: *mut crate::chardata::CharData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TestCase {
    pub expectedMovementInChars: ptrdiff_t,
    pub input: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_faults {
    pub parse_text: *const c_char,
    pub fail_text: *const c_char,
    pub encoding: *const XML_Char,
    pub error: XML_Error,
}
/* Test that bad encodings are faulted */

pub type ExtFaults = ext_faults;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttTest {
    pub definition: *const c_char,
    pub element_name: *const XML_Char,
    pub attr_name: *const XML_Char,
    pub attr_type: *const XML_Char,
    pub default_value: *const XML_Char,
    pub is_required: XML_Bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CaseData {
    pub text: *const c_char,
    pub expectedError: XML_Error,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CaseData_0 {
    pub text_bytes: size_t,
    pub text: *const c_char,
    pub expected_error: XML_Error,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elementInfo {
    pub name: *const XML_Char,
    pub attr_count: c_int,
    pub id_name: *const XML_Char,
    pub attributes: *mut AttrInfo,
}

pub type ElementInfo = elementInfo;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ByteTestData {
    pub start_element_len: c_int,
    pub cdata_len: c_int,
    pub total_string_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_hdlr_data {
    pub parse_text: *const c_char,
    pub handler: XML_ExternalEntityRefHandler,
}
/* Test a different form of unknown external entity */

pub type ExtHdlrData = ext_hdlr_data;
/* Test an external entity parser set to use latin-1 detects UTF-16
 * BOMs correctly.
 */

pub type ee_parse_flags = c_uint;

pub const EE_PARSE_FULL_BUFFER: ee_parse_flags = 1;

pub const EE_PARSE_NONE: ee_parse_flags = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtTest2 {
    pub parse_text: *const c_char,
    pub parse_len: c_int,
    pub encoding: *const XML_Char,
    pub storage: *mut crate::chardata::CharData,
    pub flags: ee_parse_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtFaults2 {
    pub parse_text: *const c_char,
    pub parse_len: c_int,
    pub fail_text: *const c_char,
    pub encoding: *const XML_Char,
    pub error: XML_Error,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct default_check {
    pub expected: *const XML_Char,
    pub expectedLen: c_int,
    pub seen: XML_Bool,
}
/* Test for issue #11, wrongly suppressed default handler */

pub type DefaultCheck = default_check;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataIssue240 {
    pub parser: XML_Parser,
    pub deep: c_int,
}
/* ptrdiff_t */
/* intptr_t uint64_t */
/* XML_UNICODE */
/* XML_UNICODE_WCHAR_T */

struct Allocator;

enum AllocatorMode {
    System,
    Duff,
    Tracking,
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
static SYSTEM_ALLOCATOR: System = System;
static mut ALLOCATOR_MODE: AllocatorMode = AllocatorMode::System;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = match ALLOCATOR_MODE {
            AllocatorMode::System => {
                return SYSTEM_ALLOCATOR.alloc(layout);
            }
            AllocatorMode::Duff => {
                duff_allocator(layout.size() as size_t) as *mut u8
            }
            AllocatorMode::Tracking => {
                crate::memcheck::tracking_malloc(layout.size() as size_t) as *mut u8
            }
        };
        // Check Rust's alignment requirements,
        // and abort the allocation if they can't be met
        // Alternatively, we could panic instead
        if ptr.align_offset(layout.align()) == 0 {
            ptr
        } else {
            self.dealloc(ptr, layout);
            std::ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match ALLOCATOR_MODE {
            AllocatorMode::System => {
                SYSTEM_ALLOCATOR.dealloc(ptr, layout);
            }
            AllocatorMode::Duff => {
                free(ptr as *mut _);
            }
            AllocatorMode::Tracking => {
                crate::memcheck::tracking_free(ptr as *mut _);
            }
        }
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize
    ) -> *mut u8 {
        let ptr = match ALLOCATOR_MODE {
            AllocatorMode::System => {
                return SYSTEM_ALLOCATOR.realloc(ptr, layout, new_size);
            }
            AllocatorMode::Duff => {
                duff_reallocator(ptr as *mut _,
                                 new_size as size_t) as *mut u8
            }
            AllocatorMode::Tracking => {
                crate::memcheck::tracking_realloc(ptr as *mut _,
                                                  new_size as size_t) as *mut u8
            }
        };
        if ptr.align_offset(layout.align()) == 0 {
            ptr
        } else {
            self.dealloc(ptr, layout);
            std::ptr::null_mut()
        }
    }
}

static mut g_parser: XML_Parser = ::rexpat::stddef_h::NULL as XML_Parser;

unsafe extern "C" fn basic_setup() {
    ALLOCATOR_MODE = AllocatorMode::System;
    g_parser = XML_ParserCreate(::rexpat::stddef_h::NULL as *const XML_Char);
    if g_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            115i32,
            b"Parser not created.\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn basic_teardown() {
    if !g_parser.is_null() {
        XML_ParserFree(g_parser);
        // Switch to system allocator to prevent the tests' own allocations
        // from going to the Duff or tracking allocators
        ALLOCATOR_MODE = AllocatorMode::System;
        g_parser = ::rexpat::stddef_h::NULL as XML_Parser
    };
}
/* Generate a failure using the parser state to create an error message;
   this should be used when the parser reports an error we weren't
   expecting.
*/

unsafe extern "C" fn _xml_failure(
    mut parser: XML_Parser,
    mut file: *const c_char,
    mut line: c_int,
) {
    let mut buffer: [c_char; 1024] = [0; 1024]; /* to help out-of-bounds detection */
    let mut err: XML_Error = XML_GetErrorCode(parser);
    sprintf(
        buffer.as_mut_ptr(),
        b"    %d: %s (line %lu, offset %lu)\n    reported from %s, line %d\n\x00".as_ptr()
            as *const c_char,
        err,
        XML_ErrorString(err),
        XML_GetCurrentLineNumber(parser),
        XML_GetCurrentColumnNumber(parser),
        file,
        line,
    );
    crate::minicheck::_fail_unless(0, file, line, buffer.as_mut_ptr());
}

unsafe extern "C" fn _XML_Parse_SINGLE_BYTES(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut len: c_int,
    mut isFinal: XML_Bool,
) -> XML_Status {
    let mut res: XML_Status = XML_STATUS_ERROR_0 as XML_Status;
    let mut offset: c_int = 0;
    if len == 0 {
        return XML_Parse(parser, s, len, isFinal as c_int);
    }
    while offset < len {
        let innerIsFinal: c_int = (offset == len - 1 && isFinal != 0) as c_int;
        let c: c_char = *s.offset(offset as isize);
        res = XML_Parse(
            parser,
            &c,
            ::std::mem::size_of::<c_char>() as c_int,
            innerIsFinal,
        );
        if res != XML_STATUS_OK_0 as c_uint {
            return res;
        }
        offset += 1
    }
    return res;
}

unsafe extern "C" fn _expect_failure(
    mut text: *const c_char,
    mut errorCode: XML_Error,
    mut errorMessage: *const c_char,
    mut file: *const c_char,
    mut lineno: c_int,
) {
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_OK_0 as c_uint
    {
        /* Hackish use of _fail_unless() macro, but let's us report
        the right filename and line number. */
        crate::minicheck::_fail_unless(0i32, file, lineno, errorMessage);
    }
    if XML_GetErrorCode(g_parser) != errorCode {
        _xml_failure(g_parser, file, lineno);
    };
}
/* Dummy handlers for when we need to set a handler to tickle a bug,
   but it doesn't need to do anything.
*/

static mut dummy_handler_flags: c_ulong = 0;

pub const DUMMY_START_DOCTYPE_HANDLER_FLAG: c_ulong = (1) << 0;

pub const DUMMY_END_DOCTYPE_HANDLER_FLAG: c_ulong = (1) << 1;

pub const DUMMY_ENTITY_DECL_HANDLER_FLAG: c_ulong = (1) << 2;

pub const DUMMY_NOTATION_DECL_HANDLER_FLAG: c_ulong = (1) << 3;

pub const DUMMY_ELEMENT_DECL_HANDLER_FLAG: c_ulong = (1) << 4;

pub const DUMMY_ATTLIST_DECL_HANDLER_FLAG: c_ulong = (1) << 5;

pub const DUMMY_COMMENT_HANDLER_FLAG: c_ulong = (1) << 6;

pub const DUMMY_PI_HANDLER_FLAG: c_ulong = (1) << 7;

pub const DUMMY_START_ELEMENT_HANDLER_FLAG: c_ulong = (1) << 8;

pub const DUMMY_START_CDATA_HANDLER_FLAG: c_ulong = (1) << 9;

pub const DUMMY_END_CDATA_HANDLER_FLAG: c_ulong = (1) << 10;

pub const DUMMY_UNPARSED_ENTITY_DECL_HANDLER_FLAG: c_ulong = (1) << 11;

pub const DUMMY_START_NS_DECL_HANDLER_FLAG: c_ulong = (1) << 12;

pub const DUMMY_END_NS_DECL_HANDLER_FLAG: c_ulong = (1) << 13;

pub const DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG: c_ulong = (1) << 14;

pub const DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG: c_ulong = (1) << 15;

pub const DUMMY_SKIP_HANDLER_FLAG: c_ulong = (1) << 16;

unsafe extern "C" fn dummy_xdecl_handler(
    mut _userData: *mut c_void,
    mut _version: *const XML_Char,
    mut _encoding: *const XML_Char,
    mut _standalone: c_int,
) {
}

unsafe extern "C" fn dummy_start_doctype_handler(
    mut _userData: *mut c_void,
    mut _doctypeName: *const XML_Char,
    mut _sysid: *const XML_Char,
    mut _pubid: *const XML_Char,
    mut _has_internal_subset: c_int,
) {
    dummy_handler_flags |= DUMMY_START_DOCTYPE_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_end_doctype_handler(mut _userData: *mut c_void) {
    dummy_handler_flags |= DUMMY_END_DOCTYPE_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_entity_decl_handler(
    mut _userData: *mut c_void,
    mut _entityName: *const XML_Char,
    mut _is_parameter_entity: c_int,
    mut _value: *const XML_Char,
    mut _value_length: c_int,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
    mut _notationName: *const XML_Char,
) {
    dummy_handler_flags |= DUMMY_ENTITY_DECL_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_notation_decl_handler(
    mut _userData: *mut c_void,
    mut _notationName: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) {
    dummy_handler_flags |= DUMMY_NOTATION_DECL_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_element_decl_handler(
    mut _userData: *mut c_void,
    mut _name: *const XML_Char,
    mut model: *mut XML_Content,
) {
    /* The content model must be freed by the handler.  Unfortunately
     * we cannot pass the parser as the userData because this is used
     * with other handlers that require other userData.
     */
    XML_FreeContentModel(g_parser, model);
    dummy_handler_flags |= DUMMY_ELEMENT_DECL_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_attlist_decl_handler(
    mut _userData: *mut c_void,
    mut _elname: *const XML_Char,
    mut _attname: *const XML_Char,
    mut _att_type: *const XML_Char,
    mut _dflt: *const XML_Char,
    mut _isrequired: c_int,
) {
    dummy_handler_flags |= DUMMY_ATTLIST_DECL_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_comment_handler(mut _userData: *mut c_void, mut _data: *const XML_Char) {
    dummy_handler_flags |= DUMMY_COMMENT_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_pi_handler(
    mut _userData: *mut c_void,
    mut _target: *const XML_Char,
    mut _data: *const XML_Char,
) {
    dummy_handler_flags |= DUMMY_PI_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_start_element(
    mut _userData: *mut c_void,
    mut _name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
    dummy_handler_flags |= DUMMY_START_ELEMENT_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_end_element(mut _userData: *mut c_void, mut _name: *const XML_Char) {}

unsafe extern "C" fn dummy_start_cdata_handler(mut _userData: *mut c_void) {
    dummy_handler_flags |= DUMMY_START_CDATA_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_end_cdata_handler(mut _userData: *mut c_void) {
    dummy_handler_flags |= DUMMY_END_CDATA_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_cdata_handler(
    mut _userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
}

unsafe extern "C" fn dummy_start_namespace_decl_handler(
    mut _userData: *mut c_void,
    mut _prefix: *const XML_Char,
    mut _uri: *const XML_Char,
) {
    dummy_handler_flags |= DUMMY_START_NS_DECL_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_end_namespace_decl_handler(
    mut _userData: *mut c_void,
    mut _prefix: *const XML_Char,
) {
    dummy_handler_flags |= DUMMY_END_NS_DECL_HANDLER_FLAG;
}
/* This handler is obsolete, but while the code exists we should
 * ensure that dealing with the handler is covered by tests.
 */

unsafe extern "C" fn dummy_unparsed_entity_decl_handler(
    mut _userData: *mut c_void,
    mut _entityName: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
    mut _notationName: *const XML_Char,
) {
    dummy_handler_flags |= DUMMY_UNPARSED_ENTITY_DECL_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_default_handler(
    mut _userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
}

unsafe extern "C" fn dummy_start_doctype_decl_handler(
    mut _userData: *mut c_void,
    mut _doctypeName: *const XML_Char,
    mut _sysid: *const XML_Char,
    mut _pubid: *const XML_Char,
    mut _has_internal_subset: c_int,
) {
    dummy_handler_flags |= DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_end_doctype_decl_handler(mut _userData: *mut c_void) {
    dummy_handler_flags |= DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG;
}

unsafe extern "C" fn dummy_skip_handler(
    mut _userData: *mut c_void,
    mut _entityName: *const XML_Char,
    mut _is_parameter_entity: c_int,
) {
    dummy_handler_flags |= DUMMY_SKIP_HANDLER_FLAG;
}

unsafe extern "C" fn external_entity_optioner(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut options: *mut ExtOption = *(parser as *mut *mut c_void) as *mut ExtOption;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    while !(*options).parse_text.is_null() {
        if strcmp(systemId, (*options).system_id) == 0 {
            let mut rc: XML_Status = XML_STATUS_ERROR;
            ext_parser = XML_ExternalEntityParserCreate(
                parser,
                context,
                ::rexpat::stddef_h::NULL as *const XML_Char,
            );
            if ext_parser.is_null() {
                return XML_STATUS_ERROR_0;
            }
            rc = _XML_Parse_SINGLE_BYTES(
                ext_parser,
                (*options).parse_text,
                strlen((*options).parse_text) as c_int,
                XML_TRUE,
            );
            XML_ParserFree(ext_parser);
            return rc as c_int;
        }
        options = options.offset(1)
    }
    crate::minicheck::_fail_unless(
        0,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        432,
        b"No suitable option found\x00".as_ptr() as *const c_char,
    );
    return XML_STATUS_ERROR_0;
}
/*
 * Parameter entity evaluation support.
 */

pub const ENTITY_MATCH_FAIL: c_int = -(1);

pub const ENTITY_MATCH_NOT_FOUND: c_int = 0;

pub const ENTITY_MATCH_SUCCESS: c_int = 1;

static mut entity_name_to_match: *const XML_Char = ::rexpat::stddef_h::NULL as *const XML_Char;

static mut entity_value_to_match: *const XML_Char = ::rexpat::stddef_h::NULL as *const XML_Char;

static mut entity_match_flag: c_int = ENTITY_MATCH_NOT_FOUND;

unsafe extern "C" fn param_entity_match_handler(
    mut _userData: *mut c_void,
    mut entityName: *const XML_Char,
    mut is_parameter_entity: c_int,
    mut value: *const XML_Char,
    mut value_length: c_int,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
    mut _notationName: *const XML_Char,
) {
    if is_parameter_entity == 0 || entity_name_to_match.is_null() || entity_value_to_match.is_null()
    {
        return;
    }
    if strcmp(entityName, entity_name_to_match) == 0 {
        /* The cast here is safe because we control the horizontal and
         * the vertical, and we therefore know our strings are never
         * going to overflow an int.
         */
        if value_length != strlen(entity_value_to_match) as c_int
            || strncmp(value, entity_value_to_match, value_length as c_ulong) != 0
        {
            entity_match_flag = ENTITY_MATCH_FAIL
        } else {
            entity_match_flag = ENTITY_MATCH_SUCCESS
        }
    };
    /* Else leave the match flag alone */
}
/*
 * Character & encoding tests.
 */

unsafe extern "C" fn test_nul_byte() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 14], &[c_char; 14]>(b"test_nul_byte\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        480,
    );
    let mut text: [c_char; 13] =
        *::std::mem::transmute::<&[u8; 13], &mut [c_char; 13]>(b"<doc>\x00</doc>\x00");
    /* test that a NUL byte (in US-ASCII data) is an error */
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_mut_ptr(),
        (::std::mem::size_of::<[c_char; 13]>() as c_ulong).wrapping_sub(1u64) as c_int,
        XML_TRUE,
    ) == XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            486i32,
            b"Parser did not report error on NUL-byte.\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_INVALID_TOKEN {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            488i32,
        );
    };
}

unsafe extern "C" fn test_u0000_char() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 16], &[c_char; 16]>(b"test_u0000_char\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        492,
    );
    /* test that a NUL byte (in US-ASCII data) is an error */
    _expect_failure(
        b"<doc>&#0;</doc>\x00".as_ptr() as *const c_char,
        XML_ERROR_BAD_CHAR_REF,
        b"Parser did not report error on NUL-byte.\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        495,
    );
}

unsafe extern "C" fn test_siphash_self() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_siphash_self\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        499,
    );
    if sip24_valid() == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            501i32,
            b"SipHash self-test failed\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_siphash_spec() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_siphash_spec\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        505,
    );
    /* https://131002.net/siphash/siphash.pdf (page 19, "Test values") */
    let message: [c_char; 16] = *::std::mem::transmute::<&[u8; 16], &[c_char; 16]>(
        b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x00",
    );
    let len: size_t = (::std::mem::size_of::<[c_char; 16]>() as c_ulong).wrapping_sub(1u64);
    let expected: uint64_t = (0xa129ca61) << 32 | 0x49be45e5;
    let mut state: siphash = siphash {
        v0: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        buf: [0; 8],
        p: 0 as *mut c_uchar,
        c: 0,
    };
    let mut key: sipkey = sipkey { k: [0; 2] };
    sip_tokey(
        &mut key,
        b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x00".as_ptr() as *const c_void,
    );
    sip24_init(&mut state, &mut key);
    /* Cover spread across calls */
    sip24_update(&mut state, message.as_ptr() as *const c_void, 4);
    sip24_update(
        &mut state,
        message.as_ptr().offset(4) as *const c_void,
        len.wrapping_sub(4u64),
    );
    /* Cover null length */
    sip24_update(&mut state, message.as_ptr() as *const c_void, 0);
    if sip24_final(&mut state) != expected {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            526i32,
            b"sip24_final failed spec test\n\x00".as_ptr() as *const c_char,
        );
    }
    /* Cover wrapper */
    if siphash24(message.as_ptr() as *const c_void, len, &mut key) != expected {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            530i32,
            b"siphash24 failed spec test\n\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_bom_utf8() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 14], &[c_char; 14]>(b"test_bom_utf8\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        534,
    );
    /* This test is really just making sure we don't core on a UTF-8 BOM. */
    let mut text: *const c_char = b"\xef\xbb\xbf<e/>\x00".as_ptr() as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            540i32,
        );
    };
}

unsafe extern "C" fn test_bom_utf16_be() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_bom_utf16_be\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        544,
    );
    let mut text: [c_char; 11] =
        *::std::mem::transmute::<&[u8; 11], &mut [c_char; 11]>(b"\xfe\xff\x00<\x00e\x00/\x00>\x00");
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_mut_ptr(),
        (::std::mem::size_of::<[c_char; 11]>() as c_ulong).wrapping_sub(1u64) as c_int,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            549i32,
        );
    };
}

unsafe extern "C" fn test_bom_utf16_le() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_bom_utf16_le\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        553,
    );
    let mut text: [c_char; 11] =
        *::std::mem::transmute::<&[u8; 11], &mut [c_char; 11]>(b"\xff\xfe<\x00e\x00/\x00>\x00\x00");
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_mut_ptr(),
        (::std::mem::size_of::<[c_char; 11]>() as c_ulong).wrapping_sub(1u64) as c_int,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            558i32,
        );
    };
}
/* Parse whole buffer at once to exercise a different code path */

unsafe extern "C" fn test_nobom_utf16_le() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 20], &[c_char; 20]>(b"test_nobom_utf16_le\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        563,
    );
    let mut text: [c_char; 11] =
        *::std::mem::transmute::<&[u8; 11], &mut [c_char; 11]>(b" \x00<\x00e\x00/\x00>\x00\x00");
    if XML_Parse(
        g_parser,
        text.as_mut_ptr(),
        (::std::mem::size_of::<[c_char; 11]>() as c_ulong).wrapping_sub(1u64) as c_int,
        XML_TRUE as c_int,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            567i32,
        );
    };
}

unsafe extern "C" fn accumulate_characters(
    mut userData: *mut c_void,
    mut s: *const XML_Char,
    mut len: c_int,
) {
    crate::chardata::CharData_AppendXMLChars(userData as *mut crate::chardata::CharData, s, len);
}

unsafe extern "C" fn accumulate_attribute(
    mut userData: *mut c_void,
    mut _name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut storage: *mut crate::chardata::CharData = userData as *mut crate::chardata::CharData;
    /* Check there are attributes to deal with */
    if atts.is_null() {
        return;
    }
    while (*storage).count < 0 && !(*atts.offset(0)).is_null() {
        /* "accumulate" the value of the first attribute we see */
        crate::chardata::CharData_AppendXMLChars(storage, *atts.offset(1), -(1));
        atts = atts.offset(2)
    }
}

unsafe extern "C" fn _run_character_check(
    mut text: *const c_char,
    mut expected: *const XML_Char,
    mut file: *const c_char,
    mut line: c_int,
) {
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(g_parser, file, line);
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn _run_attribute_check(
    mut text: *const c_char,
    mut expected: *const XML_Char,
    mut file: *const c_char,
    mut line: c_int,
) {
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(g_parser, file, line);
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn ext_accumulate_characters(
    mut userData: *mut c_void,
    mut s: *const XML_Char,
    mut len: c_int,
) {
    let mut test_data: *mut ExtTest = userData as *mut ExtTest;
    accumulate_characters((*test_data).storage as *mut c_void, s, len);
}

unsafe extern "C" fn _run_ext_character_check(
    mut text: *const c_char,
    mut test_data: *mut ExtTest,
    mut expected: *const XML_Char,
    mut file: *const c_char,
    mut line: c_int,
) {
    let storage: *mut crate::chardata::CharData =
        malloc(::std::mem::size_of::<crate::chardata::CharData>() as c_ulong)
            as *mut crate::chardata::CharData;
    crate::chardata::CharData_Init(storage);
    (*test_data).storage = storage;
    XML_SetUserData(g_parser, test_data as *mut c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext_accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(g_parser, file, line);
    }
    crate::chardata::CharData_CheckXMLChars(storage, expected);
    free(storage as *mut c_void);
}
/* Regression test for SF bug #491986. */

unsafe extern "C" fn test_danish_latin1() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 19], &[c_char; 19]>(b"test_danish_latin1\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        659,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'iso-8859-1\'?>\n<e>J\xf8rgen \xe6\xf8\xe5\xc6\xd8\xc5</e>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"J\xc3\xb8rgen \xc3\xa6\xc3\xb8\xc3\xa5\xc3\x86\xc3\x98\xc3\x85\x00".as_ptr()
            as *const c_char;
    _run_character_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        669,
    );
}
/* Regression test for SF bug #514281. */

unsafe extern "C" fn test_french_charref_hexidecimal() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_french_charref_hexidecimal\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        674,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'iso-8859-1\'?>\n<doc>&#xE9;&#xE8;&#xE0;&#xE7;&#xEA;&#xC8;</doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"\xc3\xa9\xc3\xa8\xc3\xa0\xc3\xa7\xc3\xaa\xc3\x88\x00".as_ptr() as *const c_char;
    _run_character_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        683,
    );
}

unsafe extern "C" fn test_french_charref_decimal() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 28], &[c_char; 28]>(b"test_french_charref_decimal\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        687,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'iso-8859-1\'?>\n<doc>&#233;&#232;&#224;&#231;&#234;&#200;</doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"\xc3\xa9\xc3\xa8\xc3\xa0\xc3\xa7\xc3\xaa\xc3\x88\x00".as_ptr() as *const c_char;
    _run_character_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        696,
    );
}

unsafe extern "C" fn test_french_latin1() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 19], &[c_char; 19]>(b"test_french_latin1\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        700,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'iso-8859-1\'?>\n<doc>\xe9\xe8\xe0\xe7\xea\xc8</doc>\x00"
            .as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"\xc3\xa9\xc3\xa8\xc3\xa0\xc3\xa7\xc3\xaa\xc3\x88\x00".as_ptr() as *const c_char;
    _run_character_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        709,
    );
}

unsafe extern "C" fn test_french_utf8() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 17], &[c_char; 17]>(b"test_french_utf8\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        713,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n<doc>\xc3\xa9</doc>\x00".as_ptr()
            as *const c_char;
    let mut expected: *const XML_Char = b"\xc3\xa9\x00".as_ptr() as *const c_char;
    _run_character_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        721,
    );
}
/* Regression test for SF bug #600479.
   XXX There should be a test that exercises all legal XML Unicode
   characters as PCDATA and attribute value content, and XML Name
   characters as part of element and attribute names.
*/

unsafe extern "C" fn test_utf8_false_rejection() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_utf8_false_rejection\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        730,
    );
    let mut text: *const c_char = b"<doc>\xef\xba\xbf</doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char = b"\xef\xba\xbf\x00".as_ptr() as *const c_char;
    _run_character_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        737,
    );
}
/* Regression test for SF bug #477667.
   This test assures that any 8-bit character followed by a 7-bit
   character will not be mistakenly interpreted as a valid UTF-8
   sequence.
*/

unsafe extern "C" fn test_illegal_utf8() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_illegal_utf8\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        746,
    );
    let mut text: [c_char; 100] = [0; 100];
    let mut i: c_int = 0;
    i = 128;
    while i <= 255 {
        sprintf(
            text.as_mut_ptr(),
            b"<e>%ccd</e>\x00".as_ptr() as *const c_char,
            i,
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text.as_mut_ptr(),
            strlen(text.as_mut_ptr()) as c_int,
            XML_TRUE,
        ) == XML_STATUS_OK_0 as c_uint
        {
            sprintf(
                text.as_mut_ptr(),
                b"expected token error for \'%c\' (ordinal %d) in UTF-8 text\x00".as_ptr()
                    as *const c_char,
                i,
                i,
            );
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                756i32,
                text.as_mut_ptr(),
            );
        } else if XML_GetErrorCode(g_parser) != XML_ERROR_INVALID_TOKEN {
            _xml_failure(
                g_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                758i32,
            );
        }
        /* Reset the parser since we use the same parser repeatedly. */
        XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        i += 1
    }
}
/* Examples, not masks: */

pub const UTF8_LEAD_1: [c_char; 2] =
    unsafe { *::std::mem::transmute::<&[u8; 2], &[c_char; 2]>(b"\x7f\x00") };
/* 0b01111111 */

pub const UTF8_LEAD_2: [c_char; 2] =
    unsafe { *::std::mem::transmute::<&[u8; 2], &[c_char; 2]>(b"\xdf\x00") };
/* 0b11011111 */

pub const UTF8_LEAD_3: [c_char; 2] =
    unsafe { *::std::mem::transmute::<&[u8; 2], &[c_char; 2]>(b"\xef\x00") };
/* 0b11101111 */

pub const UTF8_LEAD_4: [c_char; 2] =
    unsafe { *::std::mem::transmute::<&[u8; 2], &[c_char; 2]>(b"\xf7\x00") };
/* 0b11110111 */
/* 0b10111111 */

unsafe extern "C" fn test_utf8_auto_align() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_utf8_auto_align\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        772,
    );
    let mut cases: [TestCase; 11] = [
        {
            let mut init = TestCase {
                expectedMovementInChars: 0i64,
                input: b"\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = TestCase {
                expectedMovementInChars: 0i64,
                input: UTF8_LEAD_1.as_ptr(),
            };
            init
        },
        {
            let mut init = TestCase {
                expectedMovementInChars: -1i64,
                input: UTF8_LEAD_2.as_ptr(),
            };
            init
        },
        {
            let mut init = TestCase {
                expectedMovementInChars: 0i64,
                input: b"\xdf\xbf\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = TestCase {
                expectedMovementInChars: -1i64,
                input: UTF8_LEAD_3.as_ptr(),
            };
            init
        },
        {
            let mut init = TestCase {
                expectedMovementInChars: -2i64,
                input: b"\xef\xbf\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = TestCase {
                expectedMovementInChars: 0i64,
                input: b"\xef\xbf\xbf\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = TestCase {
                expectedMovementInChars: -1i64,
                input: UTF8_LEAD_4.as_ptr(),
            };
            init
        },
        {
            let mut init = TestCase {
                expectedMovementInChars: -2i64,
                input: b"\xf7\xbf\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = TestCase {
                expectedMovementInChars: -3i64,
                input: b"\xf7\xbf\xbf\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = TestCase {
                expectedMovementInChars: 0i64,
                input: b"\xf7\xbf\xbf\xbf\x00".as_ptr() as *const c_char,
            };
            init
        },
    ];
    let mut i: size_t = 0;
    let mut success: bool = true_0 != 0;
    while i
        < (::std::mem::size_of::<[TestCase; 11]>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<TestCase>() as c_ulong)
    {
        let mut fromLim: *const c_char = cases[i as usize]
            .input
            .offset(strlen(cases[i as usize].input) as isize);
        let fromLimInitially: *const c_char = fromLim;
        let mut actualMovementInChars: ptrdiff_t = 0;
        let mut buf = ExpatBufRef::new(cases[i as usize].input, fromLim);
        _INTERNAL_trim_to_complete_utf8_characters(&mut buf);
        fromLim = buf.end();
        actualMovementInChars = fromLim.wrapping_offset_from(fromLimInitially) as c_long;
        if actualMovementInChars != cases[i as usize].expectedMovementInChars {
            let mut j: size_t = 0;
            success = false_0 != 0;
            printf(b"[-] UTF-8 case %2u: Expected movement by %2d chars, actually moved by %2d chars: \"\x00".as_ptr() as *const c_char,
                   i.wrapping_add(1u64) as
                       c_uint,
                   cases[i as usize].expectedMovementInChars as c_int,
                   actualMovementInChars as c_int);
            while j < strlen(cases[i as usize].input) {
                printf(
                    b"\\x%02x\x00".as_ptr() as *const c_char,
                    *cases[i as usize].input.offset(j as isize) as c_uchar as c_int,
                );
                j = j.wrapping_add(1)
            }
            printf(b"\"\n\x00".as_ptr() as *const c_char);
        }
        i = i.wrapping_add(1)
    }
    if !success {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            821i32,
            b"UTF-8 auto-alignment is not bullet-proof\n\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_utf16() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 11], &[c_char; 11]>(b"test_utf16\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        826,
    );
    /* <?xml version="1.0" encoding="UTF-16"?>
     *  <doc a='123'>some {A} text</doc>
     *
     * where {A} is U+FF21, FULLWIDTH LATIN CAPITAL LETTER A
     */
    let mut text: [c_char; 141] =
        *::std::mem::transmute::<&[u8; 141],
                                 &mut [c_char; 141]>(b"\x00<\x00?\x00x\x00m\x00l\x00 \x00v\x00e\x00r\x00s\x00i\x00o\x00n\x00=\x00\'\x001\x00.\x000\x00\'\x00 \x00e\x00n\x00c\x00o\x00d\x00i\x00n\x00g\x00=\x00\'\x00U\x00T\x00F\x00-\x001\x006\x00\'\x00?\x00>\x00\n\x00<\x00d\x00o\x00c\x00 \x00a\x00=\x00\'\x001\x002\x003\x00\'\x00>\x00s\x00o\x00m\x00e\x00 \xff!\x00 \x00t\x00e\x00x\x00t\x00<\x00/\x00d\x00o\x00c\x00>\x00"); /* epilog */
    let mut expected: *const XML_Char = b"some \xef\xbc\xa1 text\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_mut_ptr(),
        (::std::mem::size_of::<[c_char; 141]>() as c_ulong).wrapping_sub(1u64) as c_int,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            852i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_utf16_le_epilog_newline() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_utf16_le_epilog_newline\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        857,
    );
    let mut first_chunk_bytes: c_uint = 17;
    let mut text: [c_char; 19] = *::std::mem::transmute::<&[u8; 19], &mut [c_char; 19]>(
        b"\xff\xfe<\x00e\x00/\x00>\x00\r\x00\n\x00\r\x00\n\x00\x00",
    );
    if first_chunk_bytes as c_ulong
        >= (::std::mem::size_of::<[c_char; 19]>() as c_ulong).wrapping_sub(1u64)
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            864i32,
            b"bad value of first_chunk_bytes\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_mut_ptr(),
        first_chunk_bytes as c_int,
        XML_FALSE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            867i32,
        );
    } else {
        let mut rc: XML_Status = XML_STATUS_ERROR;
        rc = _XML_Parse_SINGLE_BYTES(
            g_parser,
            text.as_mut_ptr().offset(first_chunk_bytes as isize),
            (::std::mem::size_of::<[c_char; 19]>() as c_ulong)
                .wrapping_sub(first_chunk_bytes as c_ulong)
                .wrapping_sub(1u64) as c_int,
            XML_TRUE,
        );
        if rc == XML_STATUS_ERROR_0 as c_uint {
            _xml_failure(
                g_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                874i32,
            );
        }
    };
}
/* Test that an outright lie in the encoding is faulted */

unsafe extern "C" fn test_not_utf16() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 15], &[c_char; 15]>(b"test_not_utf16\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        880,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'utf-16\'?><doc>Hi</doc>\x00".as_ptr() as *const c_char;
    /* Use a handler to provoke the appropriate code paths */
    XML_SetXmlDeclHandler(
        g_parser,
        Some(
            dummy_xdecl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_INCORRECT_ENCODING,
        b"UTF-16 declared in UTF-8 not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        887,
    );
}
/* Test that an unknown encoding is rejected */

unsafe extern "C" fn test_bad_encoding() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_bad_encoding\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        892,
    );
    let mut text: *const c_char = b"<doc>Hi</doc>\x00".as_ptr() as *const c_char;
    if XML_SetEncoding(g_parser, b"unknown-encoding\x00".as_ptr() as *const c_char) as u64 == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            896i32,
            b"XML_SetEncoding failed\x00".as_ptr() as *const c_char,
        );
    }
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Unknown encoding not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        898,
    );
}
/* Regression test for SF bug #481609, #774028. */

unsafe extern "C" fn test_latin1_umlauts() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 20], &[c_char; 20]>(b"test_latin1_umlauts\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        903,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'iso-8859-1\'?>\n<e a=\'\xe4 \xf6 \xfc &#228; &#246; &#252; &#x00E4; &#x0F6; &#xFC; >\'\n  >\xe4 \xf6 \xfc &#228; &#246; &#252; &#x00E4; &#x0F6; &#xFC; ></e>\x00".as_ptr() as *const c_char;
    /* Expected results in UTF-8 */
    let mut expected: *const XML_Char =
        b"\xc3\xa4 \xc3\xb6 \xc3\xbc \xc3\xa4 \xc3\xb6 \xc3\xbc \xc3\xa4 \xc3\xb6 \xc3\xbc >\x00"
            .as_ptr() as *const c_char;
    _run_character_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        918,
    );
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    _run_attribute_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        920,
    );
    /* Repeat with a default handler */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    _run_character_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        924,
    );
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    _run_attribute_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        927,
    );
}
/* Test that an element name with a 4-byte UTF-8 character is rejected */

unsafe extern "C" fn test_long_utf8_character() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_long_utf8_character\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        932,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n<do\xf0\x90\x80\x80/>\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"4-byte UTF-8 character in element name not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        938,
    );
}
/* Test that a long latin-1 attribute (too long to convert in one go)
 * is correctly converted
 */

unsafe extern "C" fn test_long_latin1_attribute() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_long_latin1_attribute\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        945,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'iso-8859-1\'?>\n<doc att=\'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO\xe4\'>\n</doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        
        b"ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO\xc3\xa4\x00".as_ptr() as *const c_char;
    _run_attribute_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        995,
    );
}
/* Test that a long ASCII attribute (too long to convert in one go)
 * is correctly converted
 */

unsafe extern "C" fn test_long_ascii_attribute() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_long_ascii_attribute\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1002,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<doc att=\'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP01234\'>\n</doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        
        b"ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP01234\x00".as_ptr() as *const c_char;
    /* clang-format on */
    _run_attribute_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1046,
    );
}
/* Regression test #1 for SF bug #653180. */

unsafe extern "C" fn test_line_number_after_parse() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_line_number_after_parse\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1051,
    );
    let mut text: *const c_char = b"<tag>\n\n\n</tag>\x00".as_ptr() as *const c_char;
    let mut lineno: XML_Size = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1059i32,
        );
    }
    lineno = XML_GetCurrentLineNumber(g_parser);
    if lineno != 4 {
        let mut buffer: [c_char; 100] = [0; 100];
        sprintf(
            buffer.as_mut_ptr(),
            b"expected 4 lines, saw %lu\x00".as_ptr() as *const c_char,
            lineno,
        );
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1064i32,
            buffer.as_mut_ptr(),
        );
    };
}
/* Regression test #2 for SF bug #653180. */

unsafe extern "C" fn test_column_number_after_parse() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_column_number_after_parse\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1070,
    );
    let mut text: *const c_char = b"<tag></tag>\x00".as_ptr() as *const c_char;
    let mut colno: XML_Size = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1076i32,
        );
    }
    colno = XML_GetCurrentColumnNumber(g_parser);
    if colno != 11 {
        let mut buffer: [c_char; 100] = [0; 100];
        sprintf(
            buffer.as_mut_ptr(),
            b"expected 11 columns, saw %lu\x00".as_ptr() as *const c_char,
            colno,
        );
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1081i32,
            buffer.as_mut_ptr(),
        );
    };
}

pub const STRUCT_START_TAG: c_int = 0;

pub const STRUCT_END_TAG: c_int = 1;

unsafe extern "C" fn start_element_event_handler2(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut _attr: *mut *const XML_Char,
) {
    let mut storage: *mut crate::structdata::StructData =
        userData as *mut crate::structdata::StructData;
    crate::structdata::StructData_AddItem(
        storage,
        name,
        XML_GetCurrentColumnNumber(g_parser) as c_int,
        XML_GetCurrentLineNumber(g_parser) as c_int,
        STRUCT_START_TAG,
    );
}

unsafe extern "C" fn end_element_event_handler2(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
) {
    let mut storage: *mut crate::structdata::StructData =
        userData as *mut crate::structdata::StructData;
    crate::structdata::StructData_AddItem(
        storage,
        name,
        XML_GetCurrentColumnNumber(g_parser) as c_int,
        XML_GetCurrentLineNumber(g_parser) as c_int,
        STRUCT_END_TAG,
    );
}
/* Regression test #3 for SF bug #653180. */

unsafe extern "C" fn test_line_and_column_numbers_inside_handlers() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 45], &[c_char; 45]>(
            b"test_line_and_column_numbers_inside_handlers\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1105,
    );
    let mut text: *const c_char =
        b"<a>\n  <b>\r\n    <c/>\r  </b>\n  <d>\n    <f/>\n  </d>\n</a>\x00".as_ptr()
            as *const c_char;
    let expected: [crate::structdata::StructDataEntry; 10] = [
        {
            let mut init = crate::structdata::StructDataEntry {
                str_0: b"a\x00".as_ptr() as *const c_char,
                data0: 0,
                data1: 1,
                data2: STRUCT_START_TAG,
            };
            init
        },
        {
            let mut init = crate::structdata::StructDataEntry {
                str_0: b"b\x00".as_ptr() as *const c_char,
                data0: 2,
                data1: 2,
                data2: STRUCT_START_TAG,
            };
            init
        },
        {
            let mut init = crate::structdata::StructDataEntry {
                str_0: b"c\x00".as_ptr() as *const c_char,
                data0: 4,
                data1: 3,
                data2: STRUCT_START_TAG,
            };
            init
        },
        {
            let mut init = crate::structdata::StructDataEntry {
                str_0: b"c\x00".as_ptr() as *const c_char,
                data0: 8,
                data1: 3,
                data2: STRUCT_END_TAG,
            };
            init
        },
        {
            let mut init = crate::structdata::StructDataEntry {
                str_0: b"b\x00".as_ptr() as *const c_char,
                data0: 2,
                data1: 4,
                data2: STRUCT_END_TAG,
            };
            init
        },
        {
            let mut init = crate::structdata::StructDataEntry {
                str_0: b"d\x00".as_ptr() as *const c_char,
                data0: 2,
                data1: 5,
                data2: STRUCT_START_TAG,
            };
            init
        },
        {
            let mut init = crate::structdata::StructDataEntry {
                str_0: b"f\x00".as_ptr() as *const c_char,
                data0: 4,
                data1: 6,
                data2: STRUCT_START_TAG,
            };
            init
        },
        {
            let mut init = crate::structdata::StructDataEntry {
                str_0: b"f\x00".as_ptr() as *const c_char,
                data0: 8,
                data1: 6,
                data2: STRUCT_END_TAG,
            };
            init
        },
        {
            let mut init = crate::structdata::StructDataEntry {
                str_0: b"d\x00".as_ptr() as *const c_char,
                data0: 2,
                data1: 7,
                data2: STRUCT_END_TAG,
            };
            init
        },
        {
            let mut init = crate::structdata::StructDataEntry {
                str_0: b"a\x00".as_ptr() as *const c_char,
                data0: 0,
                data1: 8,
                data2: STRUCT_END_TAG,
            };
            init
        },
    ];
    let expected_count: c_int = (::std::mem::size_of::<[crate::structdata::StructDataEntry; 10]>()
        as c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::structdata::StructDataEntry>() as c_ulong)
        as c_int;
    let mut storage: crate::structdata::StructData = crate::structdata::StructData {
        count: 0,
        max_count: 0,
        entries: 0 as *mut crate::structdata::StructDataEntry,
    };
    crate::structdata::StructData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::structdata::StructData as *mut c_void,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_event_handler2
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndElementHandler(
        g_parser,
        Some(
            end_element_event_handler2
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1129i32,
        );
    }
    crate::structdata::StructData_CheckItems(
        &mut storage as *mut _,
        expected.as_ptr(),
        expected_count,
    );
    crate::structdata::StructData_Dispose(&mut storage as *mut _);
}
/* Regression test #4 for SF bug #653180. */

unsafe extern "C" fn test_line_number_after_error() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_line_number_after_error\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1137,
    ); /* missing </b> */
    let mut text: *const c_char = b"<a>\n  <b>\n  </a>\x00".as_ptr() as *const c_char;
    let mut lineno: XML_Size = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_FALSE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1144i32,
            b"Expected a parse error\x00".as_ptr() as *const c_char,
        );
    }
    lineno = XML_GetCurrentLineNumber(g_parser);
    if lineno != 3 {
        let mut buffer: [c_char; 100] = [0; 100];
        sprintf(
            buffer.as_mut_ptr(),
            b"expected 3 lines, saw %lu\x00".as_ptr() as *const c_char,
            lineno,
        );
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1150i32,
            buffer.as_mut_ptr(),
        );
    };
}
/* Regression test #5 for SF bug #653180. */

unsafe extern "C" fn test_column_number_after_error() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_column_number_after_error\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1156,
    ); /* missing </b> */
    let mut text: *const c_char = b"<a>\n  <b>\n  </a>\x00".as_ptr() as *const c_char;
    let mut colno: XML_Size = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_FALSE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1163i32,
            b"Expected a parse error\x00".as_ptr() as *const c_char,
        );
    }
    colno = XML_GetCurrentColumnNumber(g_parser);
    if colno != 4 {
        let mut buffer: [c_char; 100] = [0; 100];
        sprintf(
            buffer.as_mut_ptr(),
            b"expected 4 columns, saw %lu\x00".as_ptr() as *const c_char,
            colno,
        );
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1169i32,
            buffer.as_mut_ptr(),
        );
    };
}
/* Regression test for SF bug #478332. */

unsafe extern "C" fn test_really_long_lines() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_really_long_lines\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1175,
    );
    /* This parses an input line longer than INIT_DATA_BUF_SIZE
       characters long (defined to be 1024 in xmlparse.c).  We take a
       really cheesy approach to building the input buffer, because
       this avoids writing bugs in buffer-filling code.
    */
    let mut text: *const c_char =
        
        b"<e>ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+</e>\x00".as_ptr() as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1205i32,
        );
    };
}
/* Test cdata processing across a buffer boundary */

unsafe extern "C" fn test_really_long_encoded_lines() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_really_long_encoded_lines\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1210,
    );
    /* As above, except that we want to provoke an output buffer
     * overflow with a non-trivial encoding.  For this we need to pass
     * the whole cdata in one go, not byte-by-byte.
     */
    let mut buffer: *mut c_void = 0 as *mut c_void;
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'iso-8859-1\'?><e>ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+</e>\x00".as_ptr() as *const c_char;
    let mut parse_len: c_int = strlen(text) as c_int;
    /* Need a cdata handler to provoke the code path we want to test */
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            dummy_cdata_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    buffer = XML_GetBuffer(g_parser, parse_len);
    if buffer.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1245i32,
            b"Could not allocate parse buffer\x00".as_ptr() as *const c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1246u32,
            (*::std::mem::transmute::<&[u8; 42], &[c_char; 42]>(
                b"void test_really_long_encoded_lines(void)\x00",
            ))
            .as_ptr(),
        );
    }
    memcpy(buffer, text as *const c_void, parse_len as c_ulong);
    if XML_ParseBuffer(g_parser, parse_len, XML_TRUE as c_int) == XML_STATUS_ERROR_0 as c_uint {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1249i32,
        );
    };
}
/*
 * Element event tests.
 */

unsafe extern "C" fn start_element_event_handler(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
    crate::chardata::CharData_AppendXMLChars(
        userData as *mut crate::chardata::CharData,
        name,
        -(1),
    );
}

unsafe extern "C" fn end_element_event_handler(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
) {
    let mut storage: *mut crate::chardata::CharData = userData as *mut crate::chardata::CharData;
    crate::chardata::CharData_AppendXMLChars(storage, b"/\x00".as_ptr() as *const c_char, 1);
    crate::chardata::CharData_AppendXMLChars(storage, name, -(1));
}

unsafe extern "C" fn test_end_element_events() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_end_element_events\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1271,
    );
    let mut text: *const c_char = b"<a><b><c/></b><d><f/></d></a>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char = b"/c/b/f/d/a\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetEndElementHandler(
        g_parser,
        Some(
            end_element_event_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1281i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/*
 * Attribute tests.
 */
/* Helpers used by the following test; this checks any "attr" and "refs"
   attributes to make sure whitespace has been normalized.

   Return true if whitespace has been normalized in a string, using
   the rules for attribute value normalization.  The 'is_cdata' flag
   is needed since CDATA attributes don't need to have multiple
   whitespace characters collapsed to a single space, while other
   attribute data types do.  (Section 3.3.3 of the recommendation.)
*/

unsafe extern "C" fn is_whitespace_normalized(
    mut s: *const XML_Char,
    mut is_cdata: c_int,
) -> c_int {
    let mut blanks: c_int = 0;
    let mut at_start: c_int = 1;
    while *s != 0 {
        if *s as c_int == ' ' as i32 {
            blanks += 1
        } else if *s as c_int == '\t' as i32
            || *s as c_int == '\n' as i32
            || *s as c_int == '\r' as i32
        {
            return 0i32;
        } else {
            if at_start != 0 {
                at_start = 0;
                if blanks != 0 && is_cdata == 0 {
                    /* illegal leading blanks */
                    return 0i32;
                }
            } else if blanks > 1 && is_cdata == 0 {
                return 0i32;
            }
            blanks = 0
        }
        s = s.offset(1)
    }
    if blanks != 0 && is_cdata == 0 {
        return 0i32;
    }
    return 1;
}
/* Check the attribute whitespace checker: */

unsafe extern "C" fn testhelper_is_whitespace_normalized() {
    if is_whitespace_normalized(b"abc\x00".as_ptr() as *const c_char, 0) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc\"), 0)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1328u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"abc\x00".as_ptr() as *const c_char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc\"), 1)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1329u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"abc def ghi\x00".as_ptr() as *const c_char, 0) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc def ghi\"), 0)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1330u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"abc def ghi\x00".as_ptr() as *const c_char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc def ghi\"), 1)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1331u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b" abc def ghi\x00".as_ptr() as *const c_char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\" abc def ghi\"), 0)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1332u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b" abc def ghi\x00".as_ptr() as *const c_char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\" abc def ghi\"), 1)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1333u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"abc  def ghi\x00".as_ptr() as *const c_char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"abc  def ghi\"), 0)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1334u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"abc  def ghi\x00".as_ptr() as *const c_char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc  def ghi\"), 1)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1335u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"abc def ghi \x00".as_ptr() as *const c_char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"abc def ghi \"), 0)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1336u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"abc def ghi \x00".as_ptr() as *const c_char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc def ghi \"), 1)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1337u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b" \x00".as_ptr() as *const c_char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\" \"), 0)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1338u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b" \x00".as_ptr() as *const c_char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\" \"), 1)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1339u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"\t\x00".as_ptr() as *const c_char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\t\"), 0)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1340u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"\t\x00".as_ptr() as *const c_char, 1) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\t\"), 1)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1341u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"\n\x00".as_ptr() as *const c_char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\n\"), 0)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1342u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"\n\x00".as_ptr() as *const c_char, 1) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\n\"), 1)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1343u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"\r\x00".as_ptr() as *const c_char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\r\"), 0)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1344u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"\r\x00".as_ptr() as *const c_char, 1) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\r\"), 1)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1345u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if is_whitespace_normalized(b"abc\t def\x00".as_ptr() as *const c_char, 1) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"abc\\t def\"), 1)\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1346u32,
            (*::std::mem::transmute::<&[u8; 47], &[c_char; 47]>(
                b"void testhelper_is_whitespace_normalized(void)\x00",
            ))
            .as_ptr(),
        );
    };
}

unsafe extern "C" fn check_attr_contains_normalized_whitespace(
    mut _userData: *mut c_void,
    mut _name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut i: c_int = 0;
    i = 0;
    while !(*atts.offset(i as isize)).is_null() {
        let mut attrname: *const XML_Char = *atts.offset(i as isize);
        let mut value: *const XML_Char = *atts.offset((i + 1) as isize);
        if strcmp(b"attr\x00".as_ptr() as *const c_char, attrname) == 0
            || strcmp(b"ents\x00".as_ptr() as *const c_char, attrname) == 0
            || strcmp(b"refs\x00".as_ptr() as *const c_char, attrname) == 0
        {
            if is_whitespace_normalized(value, 0) == 0 {
                let mut buffer: [c_char; 256] = [0; 256];
                sprintf(
                    buffer.as_mut_ptr(),
                    b"attribute value not normalized: %s=\'%s\'\x00".as_ptr() as *const c_char,
                    attrname,
                    value,
                );
                crate::minicheck::_fail_unless(
                    0i32,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                        .as_ptr() as *const c_char,
                    1367i32,
                    buffer.as_mut_ptr(),
                );
            }
        }
        i += 2
    }
}

unsafe extern "C" fn test_attr_whitespace_normalization() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_attr_whitespace_normalization\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1373,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ATTLIST doc\n            attr NMTOKENS #REQUIRED\n            ents ENTITIES #REQUIRED\n            refs IDREFS   #REQUIRED>\n]>\n<doc attr=\'    a  b c\t\td\te\t\' refs=\' id-1   \t  id-2\t\t\'  \n     ents=\' ent-1   \t\r\n            ent-2  \' >\n  <e id=\'id-1\'/>\n  <e id=\'id-2\'/>\n</doc>\x00".as_ptr() as *const c_char;
    XML_SetStartElementHandler(
        g_parser,
        Some(
            check_attr_contains_normalized_whitespace
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1392i32,
        );
    };
}
/*
 * XML declaration tests.
 */

unsafe extern "C" fn test_xmldecl_misplaced() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_xmldecl_misplaced\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1400,
    );
    _expect_failure(
        b"\n<?xml version=\'1.0\'?>\n<a/>\x00".as_ptr() as *const c_char,
        XML_ERROR_MISPLACED_XML_PI,
        b"failed to report misplaced XML declaration\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1405,
    );
}

unsafe extern "C" fn test_xmldecl_invalid() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_xmldecl_invalid\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1409,
    );
    _expect_failure(
        b"<?xml version=\'1.0\' \xc3\xa7?>\n<doc/>\x00".as_ptr() as *const c_char,
        XML_ERROR_XML_DECL,
        b"Failed to report invalid XML declaration\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1411,
    );
}

unsafe extern "C" fn test_xmldecl_missing_attr() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_xmldecl_missing_attr\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1415,
    );
    _expect_failure(
        b"<?xml =\'1.0\'?>\n<doc/>\n\x00".as_ptr() as *const c_char,
        XML_ERROR_XML_DECL,
        b"Failed to report missing XML declaration attribute\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1417,
    );
}

unsafe extern "C" fn test_xmldecl_missing_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_xmldecl_missing_value\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1421,
    );
    _expect_failure(
        b"<?xml version=\'1.0\' encoding=\'us-ascii\' standalone?>\n<doc/>\x00".as_ptr()
            as *const c_char,
        XML_ERROR_XML_DECL,
        b"Failed to report missing attribute value\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1425,
    );
}
/* Regression test for SF bug #584832. */

unsafe extern "C" fn UnknownEncodingHandler(
    mut _data: *mut c_void,
    mut encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> c_int {
    if strcmp(
        encoding,
        b"unsupported-encoding\x00".as_ptr() as *const c_char,
    ) == 0
    {
        let mut i: c_int = 0;
        i = 0;
        while i < 256 {
            (*info).map[i as usize] = i;
            i += 1
        }
        (*info).data = ::rexpat::stddef_h::NULL as *mut c_void;
        (*info).convert = ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int>,
        >(::rexpat::stddef_h::NULL as libc::intptr_t);
        (*info).release = ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut c_void) -> ()>,
        >(::rexpat::stddef_h::NULL as libc::intptr_t);
        return XML_STATUS_OK_0;
    }
    return XML_STATUS_ERROR_0;
}

unsafe extern "C" fn test_unknown_encoding_internal_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 38], &[c_char; 38]>(
            b"test_unknown_encoding_internal_entity\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1446,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'unsupported-encoding\'?>\n<!DOCTYPE test [<!ENTITY foo \'bar\'>]>\n<test a=\'&foo;\'/>\x00".as_ptr() as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            UnknownEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1454i32,
        );
    };
}
/* Test unrecognised encoding handler */

unsafe extern "C" fn dummy_release(mut _data: *mut c_void) {}

unsafe extern "C" fn UnrecognisedEncodingHandler(
    mut _data: *mut c_void,
    mut _encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> c_int {
    (*info).data = ::rexpat::stddef_h::NULL as *mut c_void;
    (*info).convert = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int>,
    >(::rexpat::stddef_h::NULL as libc::intptr_t);
    (*info).release = Some(dummy_release as unsafe extern "C" fn(_: *mut c_void) -> ());
    return XML_STATUS_ERROR_0;
}

unsafe extern "C" fn test_unrecognised_encoding_internal_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 43], &[c_char; 43]>(
            b"test_unrecognised_encoding_internal_entity\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1475,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'unsupported-encoding\'?>\n<!DOCTYPE test [<!ENTITY foo \'bar\'>]>\n<test a=\'&foo;\'/>\x00".as_ptr() as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            UnrecognisedEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1483i32,
            b"Unrecognised encoding not rejected\x00".as_ptr() as *const c_char,
        );
    };
}
/* Regression test for SF bug #620106. */

unsafe extern "C" fn external_entity_loader(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut test_data: *mut ExtTest = *(parser as *mut *mut c_void) as *mut ExtTest;
    let mut extparser: XML_Parser = 0 as *mut XML_ParserStruct;
    extparser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if extparser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1500i32,
            b"Could not create external entity parser.\x00".as_ptr() as *const c_char,
        );
    }
    if !(*test_data).encoding.is_null() {
        if XML_SetEncoding(extparser, (*test_data).encoding) as u64 == 0 {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                1503i32,
                b"XML_SetEncoding() ignored for external entity\x00".as_ptr() as *const c_char,
            );
        }
    }
    if _XML_Parse_SINGLE_BYTES(
        extparser,
        (*test_data).parse_text,
        strlen((*test_data).parse_text) as c_int,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            extparser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1508,
        );
        return XML_STATUS_ERROR_0;
    }
    XML_ParserFree(extparser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_ext_entity_set_encoding() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_ext_entity_set_encoding\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1515,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<?xml encoding=\'iso-8859-3\'?>\xc3\xa9\x00".as_ptr() as *const c_char,
            encoding: b"utf-8\x00".as_ptr() as *const c_char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    let mut expected: *const XML_Char = b"\xc3\xa9\x00".as_ptr() as *const c_char;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    _run_ext_character_check(
        text,
        &mut test_data,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1532,
    );
}
/* Test external entities with no handler */

unsafe extern "C" fn test_ext_entity_no_handler() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_ext_entity_no_handler\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1537,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    _run_character_check(
        text,
        b"\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1544,
    );
}
/* Test UTF-8 BOM is accepted */

unsafe extern "C" fn test_ext_entity_set_bom() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_ext_entity_set_bom\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1549,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"\xef\xbb\xbf<?xml encoding=\'iso-8859-3\'?>\xc3\xa9\x00".as_ptr()
                as *const c_char,
            encoding: b"utf-8\x00".as_ptr() as *const c_char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    let mut expected: *const XML_Char = b"\xc3\xa9\x00".as_ptr() as *const c_char;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    _run_ext_character_check(
        text,
        &mut test_data,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1565,
    );
}

unsafe extern "C" fn external_entity_faulter(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut fault: *mut ExtFaults = *(parser as *mut *mut c_void) as *mut ExtFaults;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1589i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if !(*fault).encoding.is_null() {
        if XML_SetEncoding(ext_parser, (*fault).encoding) as u64 == 0 {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                1592i32,
                b"XML_SetEncoding failed\x00".as_ptr() as *const c_char,
            );
        }
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        (*fault).parse_text,
        strlen((*fault).parse_text) as c_int,
        XML_TRUE,
    ) != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1597i32,
            (*fault).fail_text,
        );
    }
    if XML_GetErrorCode(ext_parser) != (*fault).error {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1599i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_ERROR_0;
}

unsafe extern "C" fn test_ext_entity_bad_encoding() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_ext_entity_bad_encoding\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1605,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut fault: ExtFaults = {
        let mut init = ext_faults {
            parse_text: b"<?xml encoding=\'iso-8859-3\'?>u\x00".as_ptr() as *const c_char,
            fail_text: b"Unsupported encoding not faulted\x00".as_ptr() as *const c_char,
            encoding: b"unknown\x00".as_ptr() as *const c_char,
            error: XML_ERROR_UNKNOWN_ENCODING,
        };
        init
    };
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut fault as *mut ExtFaults as *mut c_void);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad encoding should not have been accepted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1617,
    );
}
/* Try handing an invalid encoding to an external entity parser */

unsafe extern "C" fn test_ext_entity_bad_encoding_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_ext_entity_bad_encoding_2\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1622,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut fault: ExtFaults = {
        let mut init = ext_faults {
            parse_text: b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char,
            fail_text: b"Unknown encoding not faulted\x00".as_ptr() as *const c_char,
            encoding: b"unknown-encoding\x00".as_ptr() as *const c_char,
            error: XML_ERROR_UNKNOWN_ENCODING,
        };
        init
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut fault as *mut ExtFaults as *mut c_void);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad encoding not faulted in external entity handler\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1634,
    );
}
/* Test that no error is reported for unknown entities if we don't
   read an external subset.  This was fixed in Expat 1.95.5.
*/

unsafe extern "C" fn test_wfc_undeclared_entity_unread_external_subset() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 50], &[c_char; 50]>(
            b"test_wfc_undeclared_entity_unread_external_subset\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1641,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1647i32,
        );
    };
}
/* Test that an error is reported for unknown entities if we don't
   have an external subset.
*/

unsafe extern "C" fn test_wfc_undeclared_entity_no_external_subset() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 46], &[c_char; 46]>(
            b"test_wfc_undeclared_entity_no_external_subset\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1654,
    );
    _expect_failure(
        b"<doc>&entity;</doc>\x00".as_ptr() as *const c_char,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity w/out a DTD.\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1656,
    );
}
/* Test that an error is reported for unknown entities if we don't
   read an external subset, but have been declared standalone.
*/

unsafe extern "C" fn test_wfc_undeclared_entity_standalone() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 38], &[c_char; 38]>(
            b"test_wfc_undeclared_entity_standalone\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1663,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\' standalone=\'yes\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity (standalone).\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1670,
    );
}
/* Test that an error is reported for unknown entities if we have read
   an external subset, and standalone is true.
*/

unsafe extern "C" fn test_wfc_undeclared_entity_with_external_subset_standalone() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 59], &[c_char; 59]>(
            b"test_wfc_undeclared_entity_with_external_subset_standalone\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1677,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\' standalone=\'yes\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest as *mut c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity (external DTD).\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1688,
    );
}
/* Test that external entity handling is not done if the parsing flag
 * is set to UNLESS_STANDALONE
 */

unsafe extern "C" fn test_entity_with_external_subset_unless_standalone() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 51], &[c_char; 51]>(
            b"test_entity_with_external_subset_unless_standalone\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1695,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\' standalone=\'yes\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<!ENTITY entity \'bar\'>\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE);
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest as *mut c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1707,
    );
}
/* Test that no error is reported for unknown entities if we have read
   an external subset, and standalone is false.
*/

unsafe extern "C" fn test_wfc_undeclared_entity_with_external_subset() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 48], &[c_char; 48]>(
            b"test_wfc_undeclared_entity_with_external_subset\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1714,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    _run_ext_character_check(
        text,
        &mut test_data,
        b"\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1722,
    );
}
/* Test that an error is reported if our NotStandalone handler fails */

unsafe extern "C" fn reject_not_standalone_handler(mut _userData: *mut c_void) -> c_int {
    return XML_STATUS_ERROR_0;
}

unsafe extern "C" fn test_not_standalone_handler_reject() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_not_standalone_handler_reject\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1733,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest as *mut c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetNotStandaloneHandler(
        g_parser,
        Some(reject_not_standalone_handler as unsafe extern "C" fn(_: *mut c_void) -> c_int),
    );
    _expect_failure(
        text,
        XML_ERROR_NOT_STANDALONE,
        b"NotStandalone handler failed to reject\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1744,
    );
    /* Try again but without external entity handling */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetNotStandaloneHandler(
        g_parser,
        Some(reject_not_standalone_handler as unsafe extern "C" fn(_: *mut c_void) -> c_int),
    );
    _expect_failure(
        text,
        XML_ERROR_NOT_STANDALONE,
        b"NotStandalone handler failed to reject\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1750,
    );
}
/* Test that no error is reported if our NotStandalone handler succeeds */

unsafe extern "C" fn accept_not_standalone_handler(mut _userData: *mut c_void) -> c_int {
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_not_standalone_handler_accept() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_not_standalone_handler_accept\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1761,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetNotStandaloneHandler(
        g_parser,
        Some(accept_not_standalone_handler as unsafe extern "C" fn(_: *mut c_void) -> c_int),
    );
    _run_ext_character_check(
        text,
        &mut test_data,
        b"\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1770,
    );
    /* Repeat wtihout the external entity handler */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetNotStandaloneHandler(
        g_parser,
        Some(accept_not_standalone_handler as unsafe extern "C" fn(_: *mut c_void) -> c_int),
    );
    _run_character_check(
        text,
        b"\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1775,
    );
}

unsafe extern "C" fn test_wfc_no_recursive_entity_refs() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 34], &[c_char; 34]>(
            b"test_wfc_no_recursive_entity_refs\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1779,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY entity \'&#38;entity;\'>\n]>\n<doc>&entity;</doc>\x00"
            .as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_RECURSIVE_ENTITY_REF,
        b"Parser did not report recursive entity reference.\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1786,
    );
}
/* Test incomplete external entities are faulted */

unsafe extern "C" fn test_ext_entity_invalid_parse() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_ext_entity_invalid_parse\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1791,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let faults: [ExtFaults; 4] = [
        {
            let mut init = ext_faults {
                parse_text: b"<\x00".as_ptr() as *const c_char,
                fail_text: b"Incomplete element declaration not faulted\x00".as_ptr()
                    as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<\xe2\x82\x00".as_ptr() as *const c_char,
                fail_text: b"Incomplete character not faulted\x00".as_ptr() as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_PARTIAL_CHAR,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<tag>\xe2\x82\x00".as_ptr() as *const c_char,
                fail_text: b"Incomplete character in CDATA not faulted\x00".as_ptr()
                    as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_PARTIAL_CHAR,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
                fail_text: ::rexpat::stddef_h::NULL as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_NONE,
            };
            init
        },
    ];
    let mut fault: *const ExtFaults = faults.as_ptr();
    while !(*fault).parse_text.is_null() {
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_faulter
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        XML_SetUserData(g_parser, fault as *mut c_void);
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Parser did not report external entity error\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1811,
        );
        XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        fault = fault.offset(1)
    }
}
/* Regression test for SF bug #483514. */

unsafe extern "C" fn test_dtd_default_handling() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_dtd_default_handling\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1818,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ENTITY e SYSTEM \'http://example.org/e\'>\n<!NOTATION n SYSTEM \'http://example.org/n\'>\n<!ELEMENT doc EMPTY>\n<!ATTLIST doc a CDATA #IMPLIED>\n<?pi in dtd?>\n<!--comment in dtd-->\n]><doc/>\x00".as_ptr() as *const c_char;
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetStartDoctypeDeclHandler(
        g_parser,
        Some(
            dummy_start_doctype_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    XML_SetEndDoctypeDeclHandler(
        g_parser,
        Some(dummy_end_doctype_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
    );
    XML_SetEntityDeclHandler(
        g_parser,
        Some(
            dummy_entity_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: c_int,
                    _: *const XML_Char,
                    _: c_int,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetNotationDeclHandler(
        g_parser,
        Some(
            dummy_notation_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetElementDeclHandler(
        g_parser,
        transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetAttlistDeclHandler(
        g_parser,
        Some(
            dummy_attlist_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            dummy_pi_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetCommentHandler(
        g_parser,
        Some(
            dummy_comment_handler as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    XML_SetStartCdataSectionHandler(
        g_parser,
        Some(dummy_start_cdata_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
    );
    XML_SetEndCdataSectionHandler(
        g_parser,
        Some(dummy_end_cdata_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
    );
    _run_character_check(
        text,
        b"\n\n\n\n\n\n\n<doc/>\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1839,
    );
}

unsafe extern "C" fn verify_attlist_decl_handler(
    mut userData: *mut c_void,
    mut element_name: *const XML_Char,
    mut attr_name: *const XML_Char,
    mut attr_type: *const XML_Char,
    mut default_value: *const XML_Char,
    mut is_required: c_int,
) {
    let mut at: *mut AttTest = userData as *mut AttTest;
    if strcmp(element_name, (*at).element_name) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1861i32,
            b"Unexpected element name in attribute declaration\x00".as_ptr() as *const c_char,
        );
    }
    if strcmp(attr_name, (*at).attr_name) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1863i32,
            b"Unexpected attribute name in attribute declaration\x00".as_ptr() as *const c_char,
        );
    }
    if strcmp(attr_type, (*at).attr_type) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1865i32,
            b"Unexpected attribute type in attribute declaration\x00".as_ptr() as *const c_char,
        );
    }
    if default_value.is_null() && !(*at).default_value.is_null()
        || !default_value.is_null() && (*at).default_value.is_null()
        || !default_value.is_null() && strcmp(default_value, (*at).default_value) != 0
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1869i32,
            b"Unexpected default value in attribute declaration\x00".as_ptr() as *const c_char,
        );
    }
    if is_required != (*at).is_required as c_int {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1871i32,
            b"Requirement mismatch in attribute declaration\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_dtd_attr_handling() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_dtd_attr_handling\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1874,
    );
    let mut prolog: *const c_char =
        b"<!DOCTYPE doc [\n<!ELEMENT doc EMPTY>\n\x00".as_ptr() as *const c_char;
    let mut attr_data: [AttTest; 5] = [
        {
            let mut init = AttTest {
                definition:
                    b"<!ATTLIST doc a ( one | two | three ) #REQUIRED>\n]><doc a=\'two\'/>\x00"
                        .as_ptr() as *const c_char,
                element_name: b"doc\x00".as_ptr() as *const c_char,
                attr_name: b"a\x00".as_ptr() as *const c_char,
                attr_type: b"(one|two|three)\x00".as_ptr() as *const c_char,
                default_value: ::rexpat::stddef_h::NULL as *const XML_Char,
                is_required: XML_TRUE,
            };
            init
        },
        {
            let mut init =
                 AttTest{definition:
                             
                             b"<!NOTATION foo SYSTEM \'http://example.org/foo\'>\n<!ATTLIST doc a NOTATION (foo) #IMPLIED>\n]><doc/>\x00".as_ptr() as *const c_char,
                         element_name:
                             
                             b"doc\x00".as_ptr() as *const c_char,
                         attr_name:
                             
                             b"a\x00".as_ptr() as *const c_char,
                         attr_type:
                             
                             b"NOTATION(foo)\x00".as_ptr() as
                                 *const c_char,
                         default_value: ::rexpat::stddef_h::NULL as *const XML_Char,
                         is_required: XML_FALSE,};
            init
        },
        {
            let mut init = AttTest {
                definition: b"<!ATTLIST doc a NOTATION (foo) \'bar\'>\n]><doc/>\x00".as_ptr()
                    as *const c_char,
                element_name: b"doc\x00".as_ptr() as *const c_char,
                attr_name: b"a\x00".as_ptr() as *const c_char,
                attr_type: b"NOTATION(foo)\x00".as_ptr() as *const c_char,
                default_value: b"bar\x00".as_ptr() as *const c_char,
                is_required: XML_FALSE,
            };
            init
        },
        {
            let mut init = AttTest {
                definition: b"<!ATTLIST doc a CDATA \'\xdb\xb2\'>\n]><doc/>\x00".as_ptr()
                    as *const c_char,
                element_name: b"doc\x00".as_ptr() as *const c_char,
                attr_name: b"a\x00".as_ptr() as *const c_char,
                attr_type: b"CDATA\x00".as_ptr() as *const c_char,
                default_value: b"\xdb\xb2\x00".as_ptr() as *const c_char,
                is_required: XML_FALSE,
            };
            init
        },
        {
            let mut init = AttTest {
                definition: ::rexpat::stddef_h::NULL as *const c_char,
                element_name: ::rexpat::stddef_h::NULL as *const XML_Char,
                attr_name: ::rexpat::stddef_h::NULL as *const XML_Char,
                attr_type: ::rexpat::stddef_h::NULL as *const XML_Char,
                default_value: ::rexpat::stddef_h::NULL as *const XML_Char,
                is_required: XML_FALSE,
            };
            init
        },
    ];
    let mut test: *mut AttTest = 0 as *mut AttTest;
    test = attr_data.as_mut_ptr();
    while !(*test).definition.is_null() {
        XML_SetAttlistDeclHandler(
            g_parser,
            Some(
                verify_attlist_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, test as *mut c_void);
        if _XML_Parse_SINGLE_BYTES(g_parser, prolog, strlen(prolog) as c_int, XML_FALSE)
            == XML_STATUS_ERROR_0 as c_uint
        {
            _xml_failure(
                g_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                1912i32,
            );
        }
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            (*test).definition,
            strlen((*test).definition) as c_int,
            XML_TRUE,
        ) == XML_STATUS_ERROR_0 as c_uint
        {
            _xml_failure(
                g_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                1916i32,
            );
        }
        XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        test = test.offset(1)
    }
}
/* See related SF bug #673791.
   When namespace processing is enabled, setting the namespace URI for
   a prefix is not allowed; this test ensures that it *is* allowed
   when namespace processing is not enabled.
   (See Namespaces in XML, section 2.)
*/

unsafe extern "C" fn test_empty_ns_without_namespaces() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_empty_ns_without_namespaces\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1928,
    );
    let mut text: *const c_char =
        b"<doc xmlns:prefix=\'http://example.org/\'>\n  <e xmlns:prefix=\'\'/>\n</doc>\x00".as_ptr()
            as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1935i32,
        );
    };
}
/* Regression test for SF bug #824420.
   Checks that an xmlns:prefix attribute set in an attribute's default
   value isn't misinterpreted.
*/

unsafe extern "C" fn test_ns_in_attribute_default_without_namespaces() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 48], &[c_char; 48]>(
            b"test_ns_in_attribute_default_without_namespaces\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1943,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE e:element [\n  <!ATTLIST e:element\n    xmlns:e CDATA \'http://example.org/\'>\n      ]>\n<e:element/>\x00".as_ptr() as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            1952i32,
        );
    };
}

static mut long_character_data_text: *const c_char =
    
    b"<?xml version=\'1.0\' encoding=\'iso-8859-1\'?><s>012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789</s>\x00".as_ptr() as *const c_char;

static mut resumable: XML_Bool = XML_FALSE;

unsafe extern "C" fn clearing_aborting_character_handler(
    mut _userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
    XML_StopParser(g_parser, resumable);
    XML_SetCharacterDataHandler(
        g_parser,
        ::std::mem::transmute::<libc::intptr_t, XML_CharacterDataHandler>(
            ::rexpat::stddef_h::NULL as libc::intptr_t,
        ),
    );
}
/* Regression test for SF bug #1515266: missing check of stopped
parser in doContext() 'for' loop. */

unsafe extern "C" fn test_stop_parser_between_char_data_calls() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 41], &[c_char; 41]>(
            b"test_stop_parser_between_char_data_calls\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        1994,
    );
    /* The sample data must be big enough that there are two calls to
       the character data handler from within the inner "for" loop of
       the XML_TOK_DATA_CHARS case in doContent(), and the character
       handler must stop the parser and clear the character data
       handler.
    */
    let mut text: *const c_char = long_character_data_text;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    resumable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2007i32,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_ABORTED {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2009i32,
        );
    };
}
/* Regression test for SF bug #1515266: missing check of stopped
parser in doContext() 'for' loop. */

unsafe extern "C" fn test_suspend_parser_between_char_data_calls() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 44], &[c_char; 44]>(
            b"test_suspend_parser_between_char_data_calls\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2015,
    );
    /* The sample data must be big enough that there are two calls to
       the character data handler from within the inner "for" loop of
       the XML_TOK_DATA_CHARS case in doContent(), and the character
       handler must stop the parser and clear the character data
       handler.
    */
    let mut text: *const c_char = long_character_data_text;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    resumable = XML_TRUE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_SUSPENDED_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2028i32,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NONE {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2030i32,
        );
    }
    /* Try parsing directly */
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2034i32,
            b"Attempt to continue parse while suspended not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_SUSPENDED {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2036i32,
            b"Suspended parse not faulted with correct error\x00".as_ptr() as *const c_char,
        );
    };
}

static mut abortable: XML_Bool = XML_FALSE;

unsafe extern "C" fn parser_stop_character_handler(
    mut _userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
    XML_StopParser(g_parser, resumable);
    XML_SetCharacterDataHandler(
        g_parser,
        ::std::mem::transmute::<libc::intptr_t, XML_CharacterDataHandler>(
            ::rexpat::stddef_h::NULL as libc::intptr_t,
        ),
    );
    if resumable == 0 {
        /* Check that aborting an aborted parser is faulted */
        if XML_StopParser(g_parser, XML_FALSE) != XML_STATUS_ERROR_0 as c_uint {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2052i32,
                b"Aborting aborted parser not faulted\x00".as_ptr() as *const c_char,
            );
        }
        if XML_GetErrorCode(g_parser) != XML_ERROR_FINISHED {
            _xml_failure(
                g_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2054i32,
            );
        }
    } else if abortable != 0 {
        /* Check that aborting a suspended parser works */
        if XML_StopParser(g_parser, XML_FALSE) == XML_STATUS_ERROR_0 as c_uint {
            _xml_failure(
                g_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2058i32,
            );
        }
    } else {
        /* Check that suspending a suspended parser works */
        if XML_StopParser(g_parser, XML_TRUE) != XML_STATUS_ERROR_0 as c_uint {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2062i32,
                b"Suspending suspended parser not faulted\x00".as_ptr() as *const c_char,
            );
        }
        if XML_GetErrorCode(g_parser) != XML_ERROR_SUSPENDED {
            _xml_failure(
                g_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2064i32,
            );
        }
    };
}
/* Test repeated calls to XML_StopParser are handled correctly */

unsafe extern "C" fn test_repeated_stop_parser_between_char_data_calls() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 50], &[c_char; 50]>(
            b"test_repeated_stop_parser_between_char_data_calls\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2069,
    );
    let mut text: *const c_char = long_character_data_text;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            parser_stop_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    resumable = XML_FALSE;
    abortable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2077i32,
            b"Failed to double-stop parser\x00".as_ptr() as *const c_char,
        );
    }
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            parser_stop_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    resumable = XML_TRUE;
    abortable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_SUSPENDED_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2085i32,
            b"Failed to double-suspend parser\x00".as_ptr() as *const c_char,
        );
    }
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            parser_stop_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    resumable = XML_TRUE;
    abortable = XML_TRUE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2093i32,
            b"Failed to suspend-abort parser\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_good_cdata_ascii() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_good_cdata_ascii\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2097,
    );
    let mut text: *const c_char =
        b"<a><![CDATA[<greeting>Hello, world!</greeting>]]></a>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"<greeting>Hello, world!</greeting>\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    /* Add start and end handlers for coverage */
    XML_SetStartCdataSectionHandler(
        g_parser,
        Some(dummy_start_cdata_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
    );
    XML_SetEndCdataSectionHandler(
        g_parser,
        Some(dummy_end_cdata_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2111i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
    /* Try again, this time with a default handler */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2123i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_good_cdata_utf16() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_good_cdata_utf16\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2128,
    );
    /* Test data is:
     *   <?xml version='1.0' encoding='utf-16'?>
     *   <a><![CDATA[hello]]></a>
     */
    let text: [c_char; 129] =
        *::std::mem::transmute::<&[u8; 129],
                                 &[c_char; 129]>(b"\x00<\x00?\x00x\x00m\x00l\x00 \x00v\x00e\x00r\x00s\x00i\x00o\x00n\x00=\x00\'\x001\x00.\x000\x00\'\x00 \x00e\x00n\x00c\x00o\x00d\x00i\x00n\x00g\x00=\x00\'\x00u\x00t\x00f\x00-\x001\x006\x00\'\x00?\x00>\x00\n\x00<\x00a\x00>\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\x00h\x00e\x00l\x00l\x00o\x00]\x00]\x00>\x00<\x00/\x00a\x00>\x00");
    let mut expected: *const XML_Char = b"hello\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 129]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2150i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_good_cdata_utf16_le() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_good_cdata_utf16_le\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2155,
    );
    /* Test data is:
     *   <?xml version='1.0' encoding='utf-16'?>
     *   <a><![CDATA[hello]]></a>
     */
    let text: [c_char; 129] =
        *::std::mem::transmute::<&[u8; 129],
                                 &[c_char; 129]>(b"<\x00?\x00x\x00m\x00l\x00 \x00v\x00e\x00r\x00s\x00i\x00o\x00n\x00=\x00\'\x001\x00.\x000\x00\'\x00 \x00e\x00n\x00c\x00o\x00d\x00i\x00n\x00g\x00=\x00\'\x00u\x00t\x00f\x00-\x001\x006\x00\'\x00?\x00>\x00\n\x00<\x00a\x00>\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\x00h\x00e\x00l\x00l\x00o\x00]\x00]\x00>\x00<\x00/\x00a\x00>\x00\x00");
    let mut expected: *const XML_Char = b"hello\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 129]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2177i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test UTF16 conversion of a long cdata string */
/* 16 characters: handy macro to reduce visual clutter */

unsafe extern "C" fn test_long_cdata_utf16() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_long_cdata_utf16\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2187,
    );
    /* Test data is:
     * <?xlm version='1.0' encoding='utf-16'?>
     * <a><![CDATA[
     * ABCDEFGHIJKLMNOP
     * ]]></a>
     */
    let text: [c_char; 2197] =
        *::std::mem::transmute::<&[u8; 2197],
                                 &[c_char; 2197]>(b"\x00<\x00?\x00x\x00m\x00l\x00 \x00v\x00e\x00r\x00s\x00i\x00o\x00n\x00=\x00\'\x001\x00.\x000\x00\'\x00 \x00e\x00n\x00c\x00o\x00d\x00i\x00n\x00g\x00=\x00\'\x00u\x00t\x00f\x00-\x001\x006\x00\'\x00?\x00>\x00<\x00a\x00>\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00A\x00B\x00C\x00D\x00E\x00F\x00G\x00H\x00I\x00J\x00K\x00L\x00M\x00N\x00O\x00P\x00]\x00]\x00>\x00<\x00/\x00a\x00>\x00");
    let mut expected: *const XML_Char =
        
        b"ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\x00".as_ptr() as *const c_char;
    /* clang-format on */
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    let mut buffer: *mut c_void = 0 as *mut c_void;
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    buffer = XML_GetBuffer(
        g_parser,
        (::std::mem::size_of::<[c_char; 2197]>() as c_ulong).wrapping_sub(1u64) as c_int,
    );
    if buffer.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2248i32,
            b"Could not allocate parse buffer\x00".as_ptr() as *const c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2249u32,
            (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
                b"void test_long_cdata_utf16(void)\x00",
            ))
            .as_ptr(),
        );
    }
    memcpy(
        buffer,
        text.as_ptr() as *const c_void,
        (::std::mem::size_of::<[c_char; 2197]>() as c_ulong).wrapping_sub(1u64),
    );
    if XML_ParseBuffer(
        g_parser,
        (::std::mem::size_of::<[c_char; 2197]>() as c_ulong).wrapping_sub(1u64) as c_int,
        XML_TRUE as c_int,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2252i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test handling of multiple unit UTF-16 characters */

unsafe extern "C" fn test_multichar_cdata_utf16() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_multichar_cdata_utf16\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2258,
    );
    /* Test data is:
     *   <?xml version='1.0' encoding='utf-16'?>
     *   <a><![CDATA[{MINIM}{CROTCHET}]]></a>
     *
     * where {MINIM} is U+1d15e (a minim or half-note)
     *   UTF-16: 0xd834 0xdd5e
     *   UTF-8:  0xf0 0x9d 0x85 0x9e
     * and {CROTCHET} is U+1d15f (a crotchet or quarter-note)
     *   UTF-16: 0xd834 0xdd5f
     *   UTF-8:  0xf0 0x9d 0x85 0x9f
     */
    let text: [c_char; 127] =
        *::std::mem::transmute::<&[u8; 127],
                                 &[c_char; 127]>(b"\x00<\x00?\x00x\x00m\x00l\x00 \x00v\x00e\x00r\x00s\x00i\x00o\x00n\x00=\x00\'\x001\x00.\x000\x00\'\x00 \x00e\x00n\x00c\x00o\x00d\x00i\x00n\x00g\x00=\x00\'\x00u\x00t\x00f\x00-\x001\x006\x00\'\x00?\x00>\x00\n\x00<\x00a\x00>\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\xd84\xdd^\xd84\xdd_\x00]\x00]\x00>\x00<\x00/\x00a\x00>\x00");
    let mut expected: *const XML_Char =
        b"\xf0\x9d\x85\x9e\xf0\x9d\x85\x9f\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 127]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2292i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test that an element name with a UTF-16 surrogate pair is rejected */

unsafe extern "C" fn test_utf16_bad_surrogate_pair() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_utf16_bad_surrogate_pair\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2298,
    );
    /* Test data is:
     *   <?xml version='1.0' encoding='utf-16'?>
     *   <a><![CDATA[{BADLINB}]]></a>
     *
     * where {BADLINB} is U+10000 (the first Linear B character)
     * with the UTF-16 surrogate pair in the wrong order, i.e.
     *   0xdc00 0xd800
     */
    let text: [c_char; 123] =
        *::std::mem::transmute::<&[u8; 123],
                                 &[c_char; 123]>(b"\x00<\x00?\x00x\x00m\x00l\x00 \x00v\x00e\x00r\x00s\x00i\x00o\x00n\x00=\x00\'\x001\x00.\x000\x00\'\x00 \x00e\x00n\x00c\x00o\x00d\x00i\x00n\x00g\x00=\x00\'\x00u\x00t\x00f\x00-\x001\x006\x00\'\x00?\x00>\x00\n\x00<\x00a\x00>\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\xdc\x00\xd8\x00\x00]\x00]\x00>\x00<\x00/\x00a\x00>\x00");
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 123]>() as c_int - 1,
        XML_TRUE,
    ) != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2319i32,
            b"Reversed UTF-16 surrogate pair not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_INVALID_TOKEN {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2321i32,
        );
    };
}

unsafe extern "C" fn test_bad_cdata() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 15], &[c_char; 15]>(b"test_bad_cdata\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2325,
    );
    let mut cases: [CaseData; 21] = [
        {
            let mut init = CaseData {
                text: b"<a><\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><!\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![C\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CD\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDA\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDAT\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDATA\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDATA[\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDATA[]\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDATA[]]\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><!<a/>\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_INVALID_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![<a/>\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![C<a/>\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CD<a/>\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_INVALID_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDA<a/>\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_INVALID_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDAT<a/>\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_INVALID_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDATA<a/>\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_INVALID_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDATA[<a/>\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDATA[]<a/>\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
        {
            let mut init = CaseData {
                text: b"<a><![CDATA[]]<a/>\x00".as_ptr() as *const c_char,
                expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
    ];
    let mut i: size_t = 0;
    while i
        < (::std::mem::size_of::<[CaseData; 21]>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<CaseData>() as c_ulong)
    {
        let actualStatus: XML_Status = _XML_Parse_SINGLE_BYTES(
            g_parser,
            cases[i as usize].text,
            strlen(cases[i as usize].text) as c_int,
            XML_TRUE,
        );
        let actualError: XML_Error = XML_GetErrorCode(g_parser);
        if actualStatus == XML_STATUS_ERROR {
        } else {
            __assert_fail(
                b"actualStatus == XML_STATUS_ERROR\x00".as_ptr() as *const c_char,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2363u32,
                (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(
                    b"void test_bad_cdata(void)\x00",
                ))
                .as_ptr(),
            );
        }
        if actualError != cases[i as usize].expectedError {
            let mut message: [c_char; 100] = [0; 100];
            sprintf(
                message.as_mut_ptr(),
                b"Expected error %d but got error %d for case %u: \"%s\"\n\x00".as_ptr()
                    as *const c_char,
                cases[i as usize].expectedError,
                actualError,
                (i as c_uint).wrapping_add(1u32),
                cases[i as usize].text,
            );
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2371i32,
                message.as_mut_ptr(),
            );
        }
        XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        i = i.wrapping_add(1)
    }
}
/* Test failures in UTF-16 CDATA */

unsafe extern "C" fn test_bad_cdata_utf16() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_bad_cdata_utf16\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2380,
    );
    let prolog: [c_char; 87] =
        *::std::mem::transmute::<&[u8; 87],
                                 &[c_char; 87]>(b"\x00<\x00?\x00x\x00m\x00l\x00 \x00v\x00e\x00r\x00s\x00i\x00o\x00n\x00=\x00\'\x001\x00.\x000\x00\'\x00 \x00e\x00n\x00c\x00o\x00d\x00i\x00n\x00g\x00=\x00\'\x00u\x00t\x00f\x00-\x001\x006\x00\'\x00?\x00>\x00\n\x00<\x00a\x00>\x00");
    let mut cases: [CaseData_0; 24] = [
        {
            let mut init = CaseData_0 {
                text_bytes: 1u64,
                text: b"\x00\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 2u64,
                text: b"\x00<\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 3u64,
                text: b"\x00<\x00\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 4u64,
                text: b"\x00<\x00!\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 5u64,
                text: b"\x00<\x00!\x00\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 6u64,
                text: b"\x00<\x00!\x00[\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 7u64,
                text: b"\x00<\x00!\x00[\x00\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 8u64,
                text: b"\x00<\x00!\x00[\x00C\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 9u64,
                text: b"\x00<\x00!\x00[\x00C\x00\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 10u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 11u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 12u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 13u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 14u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 15u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 16u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 17u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00\x00".as_ptr() as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 18u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\x00".as_ptr()
                    as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 19u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\x00\x00".as_ptr()
                    as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 20u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\x00Z\x00".as_ptr()
                    as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 21u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\x00Z\xd8\x00".as_ptr()
                    as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 22u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\x00Z\xd84\x00".as_ptr()
                    as *const c_char,
                expected_error: XML_ERROR_PARTIAL_CHAR,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 23u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\x00Z\xd84\xdd\x00".as_ptr()
                    as *const c_char,
                expected_error: XML_ERROR_PARTIAL_CHAR,
            };
            init
        },
        {
            let mut init = CaseData_0 {
                text_bytes: 24u64,
                text: b"\x00<\x00!\x00[\x00C\x00D\x00A\x00T\x00A\x00[\x00Z\xd84\xdd^\x00".as_ptr()
                    as *const c_char,
                expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
            };
            init
        },
    ];
    let mut i: size_t = 0;
    i = 0;
    while i
        < (::std::mem::size_of::<[CaseData_0; 24]>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<CaseData_0>() as c_ulong)
    {
        let mut actual_status: XML_Status = XML_STATUS_ERROR;
        let mut actual_error: XML_Error = XML_ERROR_NONE;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            prolog.as_ptr(),
            ::std::mem::size_of::<[c_char; 87]>() as c_int - 1,
            XML_FALSE,
        ) == XML_STATUS_ERROR_0 as c_uint
        {
            _xml_failure(
                g_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2432i32,
            );
        }
        actual_status = _XML_Parse_SINGLE_BYTES(
            g_parser,
            cases[i as usize].text,
            cases[i as usize].text_bytes as c_int,
            XML_TRUE,
        );
        if actual_status == XML_STATUS_ERROR {
        } else {
            __assert_fail(
                b"actual_status == XML_STATUS_ERROR\x00".as_ptr() as *const c_char,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2435u32,
                (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
                    b"void test_bad_cdata_utf16(void)\x00",
                ))
                .as_ptr(),
            );
        }
        actual_error = XML_GetErrorCode(g_parser);
        if actual_error != cases[i as usize].expected_error {
            let mut message: [c_char; 1024] = [0; 1024];
            sprintf(
                message.as_mut_ptr(),
                b"Expected error %d (%s), got %d (%s) for case %lu\n\x00".as_ptr() as *const c_char,
                cases[i as usize].expected_error,
                XML_ErrorString(cases[i as usize].expected_error),
                actual_error,
                XML_ErrorString(actual_error),
                i.wrapping_add(1u64),
            );
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2446i32,
                message.as_mut_ptr(),
            );
        }
        XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        i = i.wrapping_add(1)
    }
}

static mut long_cdata_text: *const c_char =
    
    b"<s><![CDATA[012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789]]></s>\x00".as_ptr() as *const c_char;
/* Test stopping the parser in cdata handler */

unsafe extern "C" fn test_stop_parser_between_cdata_calls() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 37], &[c_char; 37]>(
            b"test_stop_parser_between_cdata_calls\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2478,
    );
    let mut text: *const c_char = long_cdata_text;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    resumable = XML_FALSE;
    _expect_failure(
        text,
        XML_ERROR_ABORTED,
        b"Parse not aborted in CDATA handler\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2483,
    );
}
/* Test suspending the parser in cdata handler */

unsafe extern "C" fn test_suspend_parser_between_cdata_calls() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 40], &[c_char; 40]>(
            b"test_suspend_parser_between_cdata_calls\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2488,
    );
    let mut text: *const c_char = long_cdata_text;
    let mut result: XML_Status = XML_STATUS_ERROR;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    resumable = XML_TRUE;
    result = _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE);
    if result != XML_STATUS_SUSPENDED_0 as c_uint {
        if result == XML_STATUS_ERROR_0 as c_uint {
            _xml_failure(
                g_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2497i32,
            );
        }
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2498i32,
            b"Parse not suspended in CDATA handler\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NONE {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2501i32,
        );
    };
}
/* Test memory allocation functions */

unsafe extern "C" fn test_memory_allocation() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_memory_allocation\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2506,
    );
    let mut buffer: *mut c_char = XML_MemMalloc(g_parser, 256) as *mut c_char;
    let mut p: *mut c_char = 0 as *mut c_char;
    if buffer.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2511i32,
            b"Allocation failed\x00".as_ptr() as *const c_char,
        );
    } else {
        /* Try writing to memory; some OSes try to cheat! */
        *buffer.offset(0) = 'T' as c_char;
        *buffer.offset(1) = 'E' as c_char;
        *buffer.offset(2) = 'S' as c_char;
        *buffer.offset(3) = 'T' as c_char;
        *buffer.offset(4) = '\u{0}' as c_char;
        if strcmp(buffer, b"TEST\x00".as_ptr() as *const c_char) != 0 {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2520i32,
                b"Memory not writable\x00".as_ptr() as *const c_char,
            );
        } else {
            p = XML_MemRealloc(g_parser, buffer as *mut c_void, 512) as *mut c_char;
            if p.is_null() {
                crate::minicheck::_fail_unless(
                    0i32,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                        .as_ptr() as *const c_char,
                    2524i32,
                    b"Reallocation failed\x00".as_ptr() as *const c_char,
                );
            } else {
                /* Write again, just to be sure */
                buffer = p;
                *buffer.offset(0) = 'V' as c_char;
                if strcmp(buffer, b"VEST\x00".as_ptr() as *const c_char) != 0 {
                    crate::minicheck::_fail_unless(0i32,
                                 
                                 b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr() as *const c_char,
                                 2530i32,
                                 
                                 b"Reallocated memory not writable\x00".as_ptr() as *const c_char);
                }
            }
        }
        XML_MemFree(g_parser, buffer as *mut c_void);
    };
}

unsafe extern "C" fn record_default_handler(
    mut userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
    crate::chardata::CharData_AppendXMLChars(
        userData as *mut crate::chardata::CharData,
        b"D\x00".as_ptr() as *const c_char,
        1,
    );
}

unsafe extern "C" fn record_cdata_handler(
    mut userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
    crate::chardata::CharData_AppendXMLChars(
        userData as *mut crate::chardata::CharData,
        b"C\x00".as_ptr() as *const c_char,
        1,
    );
    XML_DefaultCurrent(g_parser);
}

unsafe extern "C" fn record_cdata_nodefault_handler(
    mut userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
    crate::chardata::CharData_AppendXMLChars(
        userData as *mut crate::chardata::CharData,
        b"c\x00".as_ptr() as *const c_char,
        1,
    );
}

unsafe extern "C" fn record_skip_handler(
    mut userData: *mut c_void,
    mut _entityName: *const XML_Char,
    mut is_parameter_entity: c_int,
) {
    crate::chardata::CharData_AppendXMLChars(
        userData as *mut crate::chardata::CharData,
        if is_parameter_entity != 0 {
            b"E\x00".as_ptr() as *const c_char
        } else {
            b"e\x00".as_ptr() as *const c_char
        },
        1,
    );
}
/* Test XML_DefaultCurrent() passes handling on correctly */

unsafe extern "C" fn test_default_current() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_default_current\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2570,
    );
    let mut text: *const c_char = b"<doc>hell]</doc>\x00".as_ptr() as *const c_char;
    let mut entity_text: *const c_char =
        b"<!DOCTYPE doc [\n<!ENTITY entity \'&#37;\'>\n]>\n<doc>&entity;</doc>\x00".as_ptr()
            as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    XML_SetDefaultHandler(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2584i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"DCDCDCDCDCDD\x00".as_ptr() as *const c_char,
    );
    /* Again, without the defaulting */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_nodefault_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2595i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"DcccccD\x00".as_ptr() as *const c_char,
    );
    /* Now with an internal entity to complicate matters */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        entity_text,
        strlen(entity_text) as c_int,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2607i32,
        );
    }
    /* The default handler suppresses the entity */
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"DDDDDDDDDDDDDDDDDDD\x00".as_ptr() as *const c_char,
    );
    /* Again, with a skip handler */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetSkippedEntityHandler(
        g_parser,
        Some(
            record_skip_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        entity_text,
        strlen(entity_text) as c_int,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2621i32,
        );
    }
    /* The default handler suppresses the entity */
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"DDDDDDDDDDDDDDDDDeD\x00".as_ptr() as *const c_char,
    );
    /* This time, allow the entity through */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetDefaultHandlerExpand(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        entity_text,
        strlen(entity_text) as c_int,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2634i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"DDDDDDDDDDDDDDDDDCDD\x00".as_ptr() as *const c_char,
    );
    /* Finally, without passing the cdata to the default handler */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetDefaultHandlerExpand(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_nodefault_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        entity_text,
        strlen(entity_text) as c_int,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2646i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"DDDDDDDDDDDDDDDDDcD\x00".as_ptr() as *const c_char,
    );
}
/* Test DTD element parsing code paths */

unsafe extern "C" fn test_dtd_elements() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_dtd_elements\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2652,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ELEMENT doc (chapter)>\n<!ELEMENT chapter (#PCDATA)>\n]>\n<doc><chapter>Wombats are go</chapter></doc>\x00".as_ptr() as *const c_char;
    XML_SetElementDeclHandler(
        g_parser,
        transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Content,
                ) -> (),
        )),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2662i32,
        );
    };
}
/* Test foreign DTD handling */

unsafe extern "C" fn test_set_foreign_dtd() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_set_foreign_dtd\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2667,
    );
    let mut text1: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n\x00".as_ptr() as *const c_char;
    let mut text2: *const c_char = b"<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    /* Check hash salt is passed through too */
    XML_SetHashSalt(g_parser, 0x12345678);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest as *mut c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    /* Add a default handler to exercise more code paths */
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_NONE {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2680i32,
            b"Could not set foreign DTD\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(g_parser, text1, strlen(text1) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2683i32,
        );
    }
    /* Ensure that trying to set the DTD after parsing has started
     * is faulted, even if it's the same setting.
     */
    if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2690i32,
            b"Failed to reject late foreign DTD setting\x00".as_ptr() as *const c_char,
        );
    }
    /* Ditto for the hash salt */
    if XML_SetHashSalt(g_parser, 0x23456789) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2693i32,
            b"Failed to reject late hash salt change\x00".as_ptr() as *const c_char,
        );
    }
    /* Now finish the parse */
    if _XML_Parse_SINGLE_BYTES(g_parser, text2, strlen(text2) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2698i32,
        );
    };
}
/* Test foreign DTD handling with a failing NotStandalone handler */

unsafe extern "C" fn test_foreign_dtd_not_standalone() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_foreign_dtd_not_standalone\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2703,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<doc>&entity;</doc>\x00".as_ptr()
            as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest as *mut c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetNotStandaloneHandler(
        g_parser,
        Some(reject_not_standalone_handler as unsafe extern "C" fn(_: *mut c_void) -> c_int),
    );
    if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_NONE {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2713i32,
            b"Could not set foreign DTD\x00".as_ptr() as *const c_char,
        );
    }
    _expect_failure(
        text,
        XML_ERROR_NOT_STANDALONE,
        b"NotStandalonehandler failed to reject\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2715,
    );
}
/* Test invalid character in a foreign DTD is faulted */

unsafe extern "C" fn test_invalid_foreign_dtd() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_invalid_foreign_dtd\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2720,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<doc>&entity;</doc>\x00".as_ptr()
            as *const c_char;
    let mut test_data: ExtFaults = {
        let mut init = ext_faults {
            parse_text: b"$\x00".as_ptr() as *const c_char,
            fail_text: b"Dollar not faulted\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            error: XML_ERROR_INVALID_TOKEN,
        };
        init
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &mut test_data as *mut ExtFaults as *mut c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_UseForeignDTD(g_parser, XML_TRUE);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad DTD should not have been accepted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2731,
    );
}
/* Test foreign DTD use with a doctype */

unsafe extern "C" fn test_foreign_dtd_with_doctype() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_foreign_dtd_with_doctype\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2736,
    );
    let mut text1: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!DOCTYPE doc [<!ENTITY entity \'hello world\'>]>\n\x00".as_ptr() as *const c_char;
    let mut text2: *const c_char = b"<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    /* Check hash salt is passed through too */
    XML_SetHashSalt(g_parser, 0x12345678);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest as *mut c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    /* Add a default handler to exercise more code paths */
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_NONE {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2750i32,
            b"Could not set foreign DTD\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(g_parser, text1, strlen(text1) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2753i32,
        );
    }
    /* Ensure that trying to set the DTD after parsing has started
     * is faulted, even if it's the same setting.
     */
    if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2760i32,
            b"Failed to reject late foreign DTD setting\x00".as_ptr() as *const c_char,
        );
    }
    /* Ditto for the hash salt */
    if XML_SetHashSalt(g_parser, 0x23456789) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2763i32,
            b"Failed to reject late hash salt change\x00".as_ptr() as *const c_char,
        );
    }
    /* Now finish the parse */
    if _XML_Parse_SINGLE_BYTES(g_parser, text2, strlen(text2) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2768i32,
        );
    };
}
/* Test XML_UseForeignDTD with no external subset present */

unsafe extern "C" fn external_entity_null_loader(
    mut _parser: XML_Parser,
    mut _context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_foreign_dtd_without_external_subset() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 41], &[c_char; 41]>(
            b"test_foreign_dtd_without_external_subset\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2785,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [<!ENTITY foo \'bar\'>]>\n<doc>&foo;</doc>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, ::rexpat::stddef_h::NULL as *mut c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_null_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_UseForeignDTD(g_parser, XML_TRUE);
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2795i32,
        );
    };
}

unsafe extern "C" fn test_empty_foreign_dtd() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_empty_foreign_dtd\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2799,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<doc>&entity;</doc>\x00".as_ptr()
            as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_null_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_UseForeignDTD(g_parser, XML_TRUE);
    _expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Undefined entity not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2807,
    );
}
/* Test XML Base is set and unset appropriately */

unsafe extern "C" fn test_set_base() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 14], &[c_char; 14]>(b"test_set_base\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2812,
    );
    let mut old_base: *const XML_Char = 0 as *const XML_Char;
    let mut new_base: *const XML_Char = b"/local/file/name.xml\x00".as_ptr() as *const c_char;
    old_base = XML_GetBase(g_parser);
    if XML_SetBase(g_parser, new_base) != XML_STATUS_OK_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2818i32,
            b"Unable to set base\x00".as_ptr() as *const c_char,
        );
    }
    if strcmp(XML_GetBase(g_parser), new_base) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2820i32,
            b"Base setting not correct\x00".as_ptr() as *const c_char,
        );
    }
    if XML_SetBase(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char)
        != XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2822i32,
            b"Unable to NULL base\x00".as_ptr() as *const c_char,
        );
    }
    if !XML_GetBase(g_parser).is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2824i32,
            b"Base setting not nulled\x00".as_ptr() as *const c_char,
        );
    }
    XML_SetBase(g_parser, old_base);
}

unsafe extern "C" fn counting_start_element_handler(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut info: *mut ElementInfo = userData as *mut ElementInfo;
    let mut attr: *mut AttrInfo = 0 as *mut AttrInfo;
    let mut count: c_int = 0;
    let mut id: c_int = 0;
    let mut i: c_int = 0;
    while !(*info).name.is_null() {
        if strcmp(name, (*info).name) == 0 {
            break;
        }
        info = info.offset(1)
    }
    if (*info).name.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2855i32,
            b"Element not recognised\x00".as_ptr() as *const c_char,
        );
    }
    /* The attribute count is twice what you might expect.  It is a
     * count of items in atts, an array which contains alternating
     * attribute names and attribute values.  For the naive user this
     * is possibly a little unexpected, but it is what the
     * documentation in expat.h tells us to expect.
     */
    count = XML_GetSpecifiedAttributeCount(g_parser);
    if (*info).attr_count * 2 != count {
        crate::minicheck::_fail_unless(
            0,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2864,
            b"Not got expected attribute count\x00".as_ptr() as *const c_char,
        );
        return;
    }
    id = XML_GetIdAttributeIndex(g_parser);
    if id == -(1) && !(*info).id_name.is_null() {
        crate::minicheck::_fail_unless(
            0,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2869,
            b"ID not present\x00".as_ptr() as *const c_char,
        );
        return;
    }
    if id != -(1) && strcmp(*atts.offset(id as isize), (*info).id_name) != 0 {
        crate::minicheck::_fail_unless(
            0,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2873,
            b"ID does not have the correct name\x00".as_ptr() as *const c_char,
        );
        return;
    }
    i = 0;
    while i < (*info).attr_count {
        attr = (*info).attributes;
        while !(*attr).name.is_null() {
            if strcmp(*atts.offset(0), (*attr).name) == 0 {
                break;
            }
            attr = attr.offset(1)
        }
        if (*attr).name.is_null() {
            crate::minicheck::_fail_unless(
                0,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2884,
                b"Attribute not recognised\x00".as_ptr() as *const c_char,
            );
            return;
        }
        if strcmp(*atts.offset(1), (*attr).value) != 0 {
            crate::minicheck::_fail_unless(
                0,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                2888,
                b"Attribute has wrong value\x00".as_ptr() as *const c_char,
            );
            return;
        }
        /* Remember, two entries in atts per attribute (see above) */
        atts = atts.offset(2);
        i += 1
    }
}

unsafe extern "C" fn test_attributes() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 16], &[c_char; 16]>(b"test_attributes\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2896,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ELEMENT doc (tag)>\n<!ATTLIST doc id ID #REQUIRED>\n]><doc a=\'1\' id=\'one\' b=\'2\'><tag c=\'3\'/></doc>\x00".as_ptr() as *const c_char;
    let mut doc_info: [AttrInfo; 4] = [
        {
            let mut init = attrInfo {
                name: b"a\x00".as_ptr() as *const c_char,
                value: b"1\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = attrInfo {
                name: b"b\x00".as_ptr() as *const c_char,
                value: b"2\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = attrInfo {
                name: b"id\x00".as_ptr() as *const c_char,
                value: b"one\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = attrInfo {
                name: ::rexpat::stddef_h::NULL as *const XML_Char,
                value: ::rexpat::stddef_h::NULL as *const XML_Char,
            };
            init
        },
    ];
    let mut tag_info: [AttrInfo; 2] = [
        {
            let mut init = attrInfo {
                name: b"c\x00".as_ptr() as *const c_char,
                value: b"3\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = attrInfo {
                name: ::rexpat::stddef_h::NULL as *const XML_Char,
                value: ::rexpat::stddef_h::NULL as *const XML_Char,
            };
            init
        },
    ];
    let mut info: [ElementInfo; 3] = [
        {
            let mut init = elementInfo {
                name: b"doc\x00".as_ptr() as *const c_char,
                attr_count: 3,
                id_name: b"id\x00".as_ptr() as *const c_char,
                attributes: ::rexpat::stddef_h::NULL as *mut AttrInfo,
            };
            init
        },
        {
            let mut init = elementInfo {
                name: b"tag\x00".as_ptr() as *const c_char,
                attr_count: 1,
                id_name: ::rexpat::stddef_h::NULL as *const XML_Char,
                attributes: ::rexpat::stddef_h::NULL as *mut AttrInfo,
            };
            init
        },
        {
            let mut init = elementInfo {
                name: ::rexpat::stddef_h::NULL as *const XML_Char,
                attr_count: 0,
                id_name: ::rexpat::stddef_h::NULL as *const XML_Char,
                attributes: ::rexpat::stddef_h::NULL as *mut AttrInfo,
            };
            init
        },
    ];
    info[0].attributes = doc_info.as_mut_ptr();
    info[1].attributes = tag_info.as_mut_ptr();
    XML_SetStartElementHandler(
        g_parser,
        Some(
            counting_start_element_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, info.as_mut_ptr() as *mut c_void);
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2919i32,
        );
    };
}
/* Test reset works correctly in the middle of processing an internal
 * entity.  Exercises some obscure code in XML_ParserReset().
 */

unsafe extern "C" fn test_reset_in_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_reset_in_entity\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2926,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ENTITY wombat \'wom\'>\n<!ENTITY entity \'hi &wom; there\'>\n]>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut status: XML_ParsingStatus = XML_ParsingStatus {
        parsing: XML_INITIALIZED,
        finalBuffer: 0,
    };
    resumable = XML_TRUE;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2938i32,
        );
    }
    XML_GetParsingStatus(g_parser, &mut status as *mut _);
    if status.parsing != XML_SUSPENDED {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2941i32,
            b"Parsing status not SUSPENDED\x00".as_ptr() as *const c_char,
        );
    }
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_GetParsingStatus(g_parser, &mut status as *mut _);
    if status.parsing != XML_INITIALIZED {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2945i32,
            b"Parsing status doesn\'t reset to INITIALIZED\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test that resume correctly passes through parse errors */

unsafe extern "C" fn test_resume_invalid_parse() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_resume_invalid_parse\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2950,
    ); /* Missing closing wedge */
    let mut text: *const c_char = b"<doc>Hello</doc\x00".as_ptr() as *const c_char;
    resumable = XML_TRUE;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2957i32,
        );
    }
    if XML_ResumeParser(g_parser) == XML_STATUS_OK_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2959i32,
            b"Resumed invalid parse not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_UNCLOSED_TOKEN {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2961i32,
            b"Invalid parse not correctly faulted\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test that re-suspended parses are correctly passed through */

unsafe extern "C" fn test_resume_resuspended() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_resume_resuspended\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2966,
    );
    let mut text: *const c_char = b"<doc>Hello<meep/>world</doc>\x00".as_ptr() as *const c_char;
    resumable = XML_TRUE;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2973i32,
        );
    }
    resumable = XML_TRUE;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if XML_ResumeParser(g_parser) != XML_STATUS_SUSPENDED_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2977i32,
            b"Resumption not suspended\x00".as_ptr() as *const c_char,
        );
    }
    /* This one should succeed and finish up */
    if XML_ResumeParser(g_parser) != XML_STATUS_OK_0 as c_uint {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2980i32,
        );
    };
}
/* Test that CDATA shows up correctly through a default handler */

unsafe extern "C" fn test_cdata_default() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 19], &[c_char; 19]>(b"test_cdata_default\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        2985,
    );
    let mut text: *const c_char =
        b"<doc><![CDATA[Hello\nworld]]></doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"<doc><![CDATA[Hello\nworld]]></doc>\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            2996i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test resetting a subordinate parser does exactly nothing */

unsafe extern "C" fn external_entity_resetter(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char = b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut status: XML_ParsingStatus = XML_ParsingStatus {
        parsing: XML_INITIALIZED,
        finalBuffer: 0,
    };
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3015i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    XML_GetParsingStatus(ext_parser, &mut status as *mut _);
    if status.parsing != XML_INITIALIZED {
        crate::minicheck::_fail_unless(
            0,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3018,
            b"Parsing status is not INITIALIZED\x00".as_ptr() as *const c_char,
        );
        return XML_STATUS_ERROR_0;
    }
    if _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3023,
        );
        return XML_STATUS_ERROR_0;
    }
    XML_GetParsingStatus(ext_parser, &mut status as *mut _);
    if status.parsing != XML_FINISHED {
        crate::minicheck::_fail_unless(
            0,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3028,
            b"Parsing status is not FINISHED\x00".as_ptr() as *const c_char,
        );
        return XML_STATUS_ERROR_0;
    }
    /* Check we can't parse here */
    if XML_Parse(ext_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3034i32,
            b"Parsing when finished not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(ext_parser) != XML_ERROR_FINISHED {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3036i32,
            b"Parsing when finished faulted with wrong code\x00".as_ptr() as *const c_char,
        );
    }
    XML_ParserReset(ext_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_GetParsingStatus(ext_parser, &mut status as *mut _);
    if status.parsing != XML_FINISHED {
        crate::minicheck::_fail_unless(
            0,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3040,
            b"Parsing status not still FINISHED\x00".as_ptr() as *const c_char,
        );
        return XML_STATUS_ERROR_0;
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_subordinate_reset() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_subordinate_reset\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3047,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_resetter
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3056i32,
        );
    };
}
/* Test suspending a subordinate parser */

unsafe extern "C" fn entity_suspending_decl_handler(
    mut userData: *mut c_void,
    mut _name: *const XML_Char,
    mut model: *mut XML_Content,
) {
    let mut ext_parser: XML_Parser = userData as XML_Parser;
    if XML_StopParser(ext_parser, XML_TRUE) != XML_STATUS_ERROR_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3069i32,
            b"Attempting to suspend a subordinate parser not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(ext_parser) != XML_ERROR_SUSPEND_PE {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3071i32,
            b"Suspending subordinate parser get wrong code\x00".as_ptr() as *const c_char,
        );
    }
    XML_SetElementDeclHandler(
        ext_parser,
        ::std::mem::transmute::<libc::intptr_t, XML_ElementDeclHandler>(
            ::rexpat::stddef_h::NULL as libc::intptr_t,
        ),
    );
    XML_FreeContentModel(g_parser, model);
}

unsafe extern "C" fn external_entity_suspender(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char = b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3088i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    XML_SetElementDeclHandler(
        ext_parser,
        transmute(Some(
            entity_suspending_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetUserData(ext_parser, ext_parser as *mut c_void);
    if _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3093,
        );
        return XML_STATUS_ERROR_0;
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_subordinate_suspend() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_subordinate_suspend\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3100,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_suspender
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3109i32,
        );
    };
}
/* Test suspending a subordinate parser from an XML declaration */
/* Increases code coverage of the tests */

unsafe extern "C" fn entity_suspending_xdecl_handler(
    mut userData: *mut c_void,
    mut _version: *const XML_Char,
    mut _encoding: *const XML_Char,
    mut _standalone: c_int,
) {
    let mut ext_parser: XML_Parser = userData as XML_Parser;
    XML_StopParser(ext_parser, resumable);
    XML_SetXmlDeclHandler(
        ext_parser,
        ::std::mem::transmute::<libc::intptr_t, XML_XmlDeclHandler>(
            ::rexpat::stddef_h::NULL as libc::intptr_t,
        ),
    );
}

unsafe extern "C" fn external_entity_suspend_xmldecl(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut status: XML_ParsingStatus = XML_ParsingStatus {
        parsing: XML_INITIALIZED,
        finalBuffer: 0,
    };
    let mut rc: XML_Status = XML_STATUS_ERROR;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3141i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    XML_SetXmlDeclHandler(
        ext_parser,
        Some(
            entity_suspending_xdecl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(ext_parser, ext_parser as *mut c_void);
    rc = _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE);
    XML_GetParsingStatus(ext_parser, &mut status as *mut _);
    if resumable != 0 {
        if rc == XML_STATUS_ERROR_0 as c_uint {
            _xml_failure(
                ext_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                3148i32,
            );
        }
        if status.parsing != XML_SUSPENDED {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                3150i32,
                b"Ext Parsing status not SUSPENDED\x00".as_ptr() as *const c_char,
            );
        }
    } else {
        if rc != XML_STATUS_ERROR_0 as c_uint {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                3153i32,
                b"Ext parsing not aborted\x00".as_ptr() as *const c_char,
            );
        }
        if XML_GetErrorCode(ext_parser) != XML_ERROR_ABORTED {
            _xml_failure(
                ext_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                3155i32,
            );
        }
        if status.parsing != XML_FINISHED {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                3157i32,
                b"Ext Parsing status not FINISHED\x00".as_ptr() as *const c_char,
            );
        }
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_subordinate_xdecl_suspend() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_subordinate_xdecl_suspend\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3164,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY entity SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_suspend_xmldecl
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    resumable = XML_TRUE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3176i32,
        );
    };
}

unsafe extern "C" fn test_subordinate_xdecl_abort() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_subordinate_xdecl_abort\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3180,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY entity SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_suspend_xmldecl
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    resumable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3192i32,
        );
    };
}
/* Test external entity fault handling with suspension */

unsafe extern "C" fn external_entity_suspending_faulter(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut fault: *mut ExtFaults = *(parser as *mut *mut c_void) as *mut ExtFaults;
    let mut buffer: *mut c_void = 0 as *mut c_void;
    let mut parse_len: c_int = strlen((*fault).parse_text) as c_int;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3212i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    XML_SetXmlDeclHandler(
        ext_parser,
        Some(
            entity_suspending_xdecl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(ext_parser, ext_parser as *mut c_void);
    resumable = XML_TRUE;
    buffer = XML_GetBuffer(ext_parser, parse_len);
    if buffer.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3218i32,
            b"Could not allocate parse buffer\x00".as_ptr() as *const c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(b"buffer != NULL\x00".as_ptr() as
                          *const c_char,
                      
                      b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr() as *const c_char,
                      3219u32,
                      (*::std::mem::transmute::<&[u8; 123],
                                                &[c_char; 123]>(b"int external_entity_suspending_faulter(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\x00")).as_ptr());
    }
    memcpy(
        buffer,
        (*fault).parse_text as *const c_void,
        parse_len as c_ulong,
    );
    if XML_ParseBuffer(ext_parser, parse_len, XML_FALSE as c_int)
        != XML_STATUS_SUSPENDED_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3222i32,
            b"XML declaration did not suspend\x00".as_ptr() as *const c_char,
        );
    }
    if XML_ResumeParser(ext_parser) != XML_STATUS_OK_0 as c_uint {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3224i32,
        );
    }
    if XML_ParseBuffer(ext_parser, 0, XML_TRUE as c_int) != XML_STATUS_ERROR_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3226i32,
            (*fault).fail_text,
        );
    }
    if XML_GetErrorCode(ext_parser) != (*fault).error {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3228i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_ERROR_0;
}

unsafe extern "C" fn test_ext_entity_invalid_suspended_parse() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 40], &[c_char; 40]>(
            b"test_ext_entity_invalid_suspended_parse\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3234,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut faults: [ExtFaults; 3] = [
        {
            let mut init = ext_faults {
                parse_text: b"<?xml version=\'1.0\' encoding=\'us-ascii\'?><\x00".as_ptr()
                    as *const c_char,
                fail_text: b"Incomplete element declaration not faulted\x00".as_ptr()
                    as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\xe2\x82\x00".as_ptr()
                    as *const c_char,
                fail_text: b"Incomplete character not faulted\x00".as_ptr() as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_PARTIAL_CHAR,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
                fail_text: ::rexpat::stddef_h::NULL as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_NONE,
            };
            init
        },
    ];
    let mut fault: *mut ExtFaults = 0 as *mut ExtFaults;
    fault = &mut *faults.as_mut_ptr().offset(0) as *mut ExtFaults;
    while !(*fault).parse_text.is_null() {
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_suspending_faulter
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        XML_SetUserData(g_parser, fault as *mut c_void);
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Parser did not report external entity error\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3255,
        );
        XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        fault = fault.offset(1)
    }
}
/* Test setting an explicit encoding */

unsafe extern "C" fn test_explicit_encoding() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_explicit_encoding\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3262,
    );
    let mut text1: *const c_char = b"<doc>Hello \x00".as_ptr() as *const c_char;
    let mut text2: *const c_char = b" World</doc>\x00".as_ptr() as *const c_char;
    /* Just check that we can set the encoding to NULL before starting */
    if XML_SetEncoding(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char)
        != XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3268i32,
            b"Failed to initialise encoding to NULL\x00".as_ptr() as *const c_char,
        );
    }
    /* Say we are UTF-8 */
    if XML_SetEncoding(g_parser, b"utf-8\x00".as_ptr() as *const c_char)
        != XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3271i32,
            b"Failed to set explicit encoding\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(g_parser, text1, strlen(text1) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3274i32,
        );
    }
    /* Try to switch encodings mid-parse */
    if XML_SetEncoding(g_parser, b"us-ascii\x00".as_ptr() as *const c_char)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3277i32,
            b"Allowed encoding change\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(g_parser, text2, strlen(text2) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3280i32,
        );
    }
    /* Try now the parse is over */
    if XML_SetEncoding(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char)
        != XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3283i32,
            b"Failed to unset encoding\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test handling of trailing CR (rather than newline) */

unsafe extern "C" fn cr_cdata_handler(
    mut userData: *mut c_void,
    mut s: *const XML_Char,
    mut len: c_int,
) {
    let mut pfound: *mut c_int = userData as *mut c_int;
    /* Internal processing turns the CR into a newline for the
     * character data handler, but not for the default handler
     */
    if len == 1 && (*s as c_int == '\n' as i32 || *s as c_int == '\r' as i32) {
        *pfound = 1
    };
}

unsafe extern "C" fn test_trailing_cr() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 17], &[c_char; 17]>(b"test_trailing_cr\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3299,
    );
    let mut text: *const c_char = b"<doc>\r\x00".as_ptr() as *const c_char;
    let mut found_cr: c_int = 0;
    /* Try with a character handler, for code coverage */
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            cr_cdata_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(g_parser, &mut found_cr as *mut c_int as *mut c_void);
    found_cr = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3309i32,
            b"Failed to fault unclosed doc\x00".as_ptr() as *const c_char,
        );
    }
    if found_cr == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3311i32,
            b"Did not catch the carriage return\x00".as_ptr() as *const c_char,
        );
    }
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    /* Now with a default handler instead */
    XML_SetDefaultHandler(
        g_parser,
        Some(
            cr_cdata_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(g_parser, &mut found_cr as *mut c_int as *mut c_void);
    found_cr = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3320i32,
            b"Failed to fault unclosed doc\x00".as_ptr() as *const c_char,
        );
    }
    if found_cr == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3322i32,
            b"Did not catch default carriage return\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test trailing CR in an external entity parse */

unsafe extern "C" fn external_entity_cr_catcher(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char = b"\r\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3339i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            cr_cdata_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3343i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn external_entity_bad_cr_catcher(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char = b"<tag>\r\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3360i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            cr_cdata_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3364i32,
            b"Async entity error not caught\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(ext_parser) != XML_ERROR_ASYNC_ENTITY {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3366i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_ext_entity_trailing_cr() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 28], &[c_char; 28]>(b"test_ext_entity_trailing_cr\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3371,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut found_cr: c_int = 0;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_cr_catcher
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut found_cr as *mut c_int as *mut c_void);
    found_cr = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_OK_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3384i32,
        );
    }
    if found_cr == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3386i32,
            b"No carriage return found\x00".as_ptr() as *const c_char,
        );
    }
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    /* Try again with a different trailing CR */
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_bad_cr_catcher
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut found_cr as *mut c_int as *mut c_void);
    found_cr = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_OK_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3396i32,
        );
    }
    if found_cr == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3398i32,
            b"No carriage return found\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test handling of trailing square bracket */

unsafe extern "C" fn rsqb_handler(
    mut userData: *mut c_void,
    mut s: *const XML_Char,
    mut len: c_int,
) {
    let mut pfound: *mut c_int = userData as *mut c_int;
    if len == 1 && *s as c_int == ']' as i32 {
        *pfound = 1
    };
}

unsafe extern "C" fn test_trailing_rsqb() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 19], &[c_char; 19]>(b"test_trailing_rsqb\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3411,
    );
    let mut text8: *const c_char = b"<doc>]\x00".as_ptr() as *const c_char;
    let text16: [c_char; 15] = *::std::mem::transmute::<&[u8; 15], &[c_char; 15]>(
        b"\xff\xfe<\x00d\x00o\x00c\x00>\x00]\x00\x00",
    );
    let mut found_rsqb: c_int = 0;
    let mut text8_len: c_int = strlen(text8) as c_int;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            rsqb_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(g_parser, &mut found_rsqb as *mut c_int as *mut c_void);
    found_rsqb = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text8, text8_len, XML_TRUE) == XML_STATUS_OK_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3422i32,
            b"Failed to fault unclosed doc\x00".as_ptr() as *const c_char,
        );
    }
    if found_rsqb == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3424i32,
            b"Did not catch the right square bracket\x00".as_ptr() as *const c_char,
        );
    }
    /* Try again with a different encoding */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            rsqb_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(g_parser, &mut found_rsqb as *mut c_int as *mut c_void);
    found_rsqb = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text16.as_ptr(),
        ::std::mem::size_of::<[c_char; 15]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3434i32,
            b"Failed to fault unclosed doc\x00".as_ptr() as *const c_char,
        );
    }
    if found_rsqb == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3436i32,
            b"Did not catch the right square bracket\x00".as_ptr() as *const c_char,
        );
    }
    /* And finally with a default handler */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            rsqb_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(g_parser, &mut found_rsqb as *mut c_int as *mut c_void);
    found_rsqb = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text16.as_ptr(),
        ::std::mem::size_of::<[c_char; 15]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3446i32,
            b"Failed to fault unclosed doc\x00".as_ptr() as *const c_char,
        );
    }
    if found_rsqb == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3448i32,
            b"Did not catch the right square bracket\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test trailing right square bracket in an external entity parse */

unsafe extern "C" fn external_entity_rsqb_catcher(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char = b"<tag>]\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3465i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            rsqb_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3469i32,
            b"Async entity error not caught\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(ext_parser) != XML_ERROR_ASYNC_ENTITY {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3471i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_ext_entity_trailing_rsqb() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_ext_entity_trailing_rsqb\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3476,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut found_rsqb: c_int = 0;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_rsqb_catcher
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut found_rsqb as *mut c_int as *mut c_void);
    found_rsqb = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_OK_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3489i32,
        );
    }
    if found_rsqb == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3491i32,
            b"No right square bracket found\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test CDATA handling in an external entity */

unsafe extern "C" fn external_entity_good_cdata_ascii(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char =
        b"<a><![CDATA[<greeting>Hello, world!</greeting>]]></a>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"<greeting>Hello, world!</greeting>\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    crate::chardata::CharData_Init(&mut storage as *mut _);
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3511i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    XML_SetUserData(
        ext_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3517i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_ext_entity_good_cdata() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_ext_entity_good_cdata\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3524,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_good_cdata_ascii
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_OK_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3534i32,
        );
    };
}
/* Test user parameter settings */
/* Variable holding the expected handler userData */

static mut handler_data: *mut c_void = ::rexpat::stddef_h::NULL as *mut c_void;
/* Count of the number of times the comment handler has been invoked */

static mut comment_count: c_int = 0;
/* Count of the number of skipped entities */

static mut skip_count: c_int = 0;
/* Count of the number of times the XML declaration handler is invoked */

static mut xdecl_count: c_int = 0;

unsafe extern "C" fn xml_decl_handler(
    mut userData: *mut c_void,
    mut _version: *const XML_Char,
    mut _encoding: *const XML_Char,
    mut standalone: c_int,
) {
    if userData != handler_data {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3554i32,
            b"User data (xml decl) not correctly set\x00".as_ptr() as *const c_char,
        );
    }
    if standalone != -(1) {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3556i32,
            b"Standalone not flagged as not present in XML decl\x00".as_ptr() as *const c_char,
        );
    }
    xdecl_count += 1;
}

unsafe extern "C" fn param_check_skip_handler(
    mut userData: *mut c_void,
    mut _entityName: *const XML_Char,
    mut _is_parameter_entity: c_int,
) {
    if userData != handler_data {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3566i32,
            b"User data (skip) not correctly set\x00".as_ptr() as *const c_char,
        );
    }
    skip_count += 1;
}

unsafe extern "C" fn data_check_comment_handler(
    mut userData: *mut c_void,
    mut _data: *const XML_Char,
) {
    /* Check that the userData passed through is what we expect */
    if userData != handler_data {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3575i32,
            b"User data (parser) not correctly set\x00".as_ptr() as *const c_char,
        );
    }
    /* Check that the user data in the parser is appropriate */
    if *(userData as *mut *mut c_void) != 1i32 as *mut c_void {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3578i32,
            b"User data in parser not correctly set\x00".as_ptr() as *const c_char,
        );
    }
    comment_count += 1;
}

unsafe extern "C" fn external_entity_param_checker(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char =
        b"<!-- Subordinate parser -->\n<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3595i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    handler_data = ext_parser as *mut c_void;
    if _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3599,
        );
        return XML_STATUS_ERROR_0;
    }
    handler_data = parser as *mut c_void;
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_user_parameters() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_user_parameters\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3607,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!-- Primary parse -->\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;\x00".as_ptr() as *const c_char;
    let mut epilog: *const c_char =
        b"<!-- Back to primary parser -->\n</doc>\x00".as_ptr() as *const c_char;
    comment_count = 0;
    skip_count = 0;
    xdecl_count = 0;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetXmlDeclHandler(
        g_parser,
        Some(
            xml_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_param_checker
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetCommentHandler(
        g_parser,
        Some(
            data_check_comment_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    XML_SetSkippedEntityHandler(
        g_parser,
        Some(
            param_check_skip_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_UseParserAsHandlerArg(g_parser);
    XML_SetUserData(g_parser, 1i32 as *mut c_void);
    handler_data = g_parser as *mut c_void;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3628i32,
        );
    }
    if comment_count != 2 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3630i32,
            b"Comment handler not invoked enough times\x00".as_ptr() as *const c_char,
        );
    }
    /* Ensure we can't change policy mid-parse */
    if XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_NEVER) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3633i32,
            b"Changed param entity parsing policy while parsing\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(g_parser, epilog, strlen(epilog) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3636i32,
        );
    }
    if comment_count != 3 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3638i32,
            b"Comment handler not invoked enough times\x00".as_ptr() as *const c_char,
        );
    }
    if skip_count != 1 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3640i32,
            b"Skip handler not invoked enough times\x00".as_ptr() as *const c_char,
        );
    }
    if xdecl_count != 1 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3642i32,
            b"XML declaration handler not invoked\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test that an explicit external entity handler argument replaces
 * the parser as the first argument.
 *
 * We do not call the first parameter to the external entity handler
 * 'parser' for once, since the first time the handler is called it
 * will actually be a text string.  We need to be able to access the
 * global 'parser' variable to create our external entity parser from,
 * since there are code paths we need to ensure get executed.
 */

unsafe extern "C" fn external_entity_ref_param_checker(
    mut parameter: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char = b"<!ELEMENT doc (#PCDATA)*>\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    if parameter as *mut c_void != handler_data {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3667i32,
            b"External entity ref handler parameter not correct\x00".as_ptr() as *const c_char,
        );
    }
    /* Here we use the global 'parser' variable */
    ext_parser = XML_ExternalEntityParserCreate(
        g_parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3672i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3675i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_ext_entity_ref_parameter() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_ext_entity_ref_parameter\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3681,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_ref_param_checker
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    /* Set a handler arg that is not NULL and not parser (which is
     * what NULL would cause to be passed.
     */
    XML_SetExternalEntityRefHandlerArg(g_parser, text as *mut c_void);
    handler_data = text as *mut c_void;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3695i32,
        );
    }
    /* Now try again with unset args */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_ref_param_checker
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetExternalEntityRefHandlerArg(g_parser, ::rexpat::stddef_h::NULL as *mut c_void);
    handler_data = g_parser as *mut c_void;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3705i32,
        );
    };
}
/* Test the parsing of an empty string */

unsafe extern "C" fn test_empty_parse() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 17], &[c_char; 17]>(b"test_empty_parse\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3710,
    );
    let mut text: *const c_char = b"<doc></doc>\x00".as_ptr() as *const c_char;
    let mut partial: *const c_char = b"<doc>\x00".as_ptr() as *const c_char;
    if XML_Parse(
        g_parser,
        ::rexpat::stddef_h::NULL as *const c_char,
        0,
        XML_FALSE as c_int,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3715i32,
            b"Parsing empty string faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_Parse(
        g_parser,
        ::rexpat::stddef_h::NULL as *const c_char,
        0,
        XML_TRUE as c_int,
    ) != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3717i32,
            b"Parsing final empty string not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NO_ELEMENTS {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3719i32,
            b"Parsing final empty string faulted for wrong reason\x00".as_ptr() as *const c_char,
        );
    }
    /* Now try with valid text before the empty end */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3725i32,
        );
    }
    if XML_Parse(
        g_parser,
        ::rexpat::stddef_h::NULL as *const c_char,
        0,
        XML_TRUE as c_int,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3727i32,
            b"Parsing final empty string faulted\x00".as_ptr() as *const c_char,
        );
    }
    /* Now try with invalid text before the empty end */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    if _XML_Parse_SINGLE_BYTES(g_parser, partial, strlen(partial) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3734i32,
        );
    }
    if XML_Parse(
        g_parser,
        ::rexpat::stddef_h::NULL as *const c_char,
        0,
        XML_TRUE as c_int,
    ) != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3736i32,
            b"Parsing final incomplete empty string not faulted\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test odd corners of the XML_GetBuffer interface */

unsafe extern "C" fn get_feature(
    mut feature_id: XML_FeatureEnum,
    mut presult: *mut c_long,
) -> XML_Status {
    let mut feature: *const XML_Feature = XML_GetFeatureList();
    if feature.is_null() {
        return XML_STATUS_ERROR_0 as XML_Status;
    }
    while (*feature).feature != XML_FEATURE_END {
        if (*feature).feature == feature_id {
            *presult = (*feature).value;
            return XML_STATUS_OK_0 as XML_Status;
        }
        feature = feature.offset(1)
    }
    return XML_STATUS_ERROR_0 as XML_Status;
}
/* Having an element name longer than 1024 characters exercises some
 * of the pool allocation code in the parser that otherwise does not
 * get executed.  The count at the end of the line is the number of
 * characters (bytes) in the element name by that point.x
 */

static mut get_buffer_test_text: *const c_char =
    
    b"<documentwitharidiculouslylongelementnametoteaseaparticularcorneroftheallocationinXML_GetBuffersothatwecanimprovethecoverageyetagain012345678901123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789>\n<ef0\x00".as_ptr() as *const c_char;
/* 0x420 */
/* Test odd corners of the XML_GetBuffer interface */

unsafe extern "C" fn test_get_buffer_1() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_get_buffer_1\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3786,
    );
    let mut text: *const c_char = get_buffer_test_text;
    let mut buffer: *mut c_void = 0 as *mut c_void;
    let mut context_bytes: c_long = 0;
    /* Attempt to allocate a negative length buffer */
    if !XML_GetBuffer(g_parser, -(12)).is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3793i32,
            b"Negative length buffer not failed\x00".as_ptr() as *const c_char,
        );
    }
    /* Now get a small buffer and extend it past valid length */
    buffer = XML_GetBuffer(g_parser, 1536);
    if buffer.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3798i32,
            b"1.5K buffer failed\x00".as_ptr() as *const c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3799u32,
            (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(
                b"void test_get_buffer_1(void)\x00",
            ))
            .as_ptr(),
        );
    }
    memcpy(buffer, text as *const c_void, strlen(text));
    if XML_ParseBuffer(g_parser, strlen(text) as c_int, XML_FALSE as c_int)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3803i32,
        );
    }
    if !XML_GetBuffer(g_parser, INT_MAX).is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3805i32,
            b"INT_MAX buffer not failed\x00".as_ptr() as *const c_char,
        );
    }
    /* Now try extending it a more reasonable but still too large
     * amount.  The allocator in XML_GetBuffer() doubles the buffer
     * size until it exceeds the requested amount or INT_MAX.  If it
     * exceeds INT_MAX, it rejects the request, so we want a request
     * between INT_MAX and INT_MAX/2.  A gap of 1K seems comfortable,
     * with an extra byte just to ensure that the request is off any
     * boundary.  The request will be inflated internally by
     * XML_CONTEXT_BYTES (if defined), so we subtract that from our
     * request.
     */
    if get_feature(XML_FEATURE_CONTEXT_BYTES, &mut context_bytes) != XML_STATUS_OK_0 as c_uint {
        context_bytes = 0
    }
    if !XML_GetBuffer(
        g_parser,
        (INT_MAX as c_long - (context_bytes + 1025)) as c_int,
    )
    .is_null()
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3820i32,
            b"INT_MAX- buffer not failed\x00".as_ptr() as *const c_char,
        );
    }
    /* Now try extending it a carefully crafted amount */
    if XML_GetBuffer(g_parser, 1000).is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3824i32,
            b"1000 buffer failed\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test more corners of the XML_GetBuffer interface */

unsafe extern "C" fn test_get_buffer_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_get_buffer_2\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3829,
    );
    let mut text: *const c_char = get_buffer_test_text;
    let mut buffer: *mut c_void = 0 as *mut c_void;
    /* Now get a decent buffer */
    buffer = XML_GetBuffer(g_parser, 1536);
    if buffer.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3836i32,
            b"1.5K buffer failed\x00".as_ptr() as *const c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3837u32,
            (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(
                b"void test_get_buffer_2(void)\x00",
            ))
            .as_ptr(),
        );
    }
    memcpy(buffer, text as *const c_void, strlen(text));
    if XML_ParseBuffer(g_parser, strlen(text) as c_int, XML_FALSE as c_int)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3841i32,
        );
    }
    /* Extend it, to catch a different code path */
    if XML_GetBuffer(g_parser, 1024).is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3845i32,
            b"1024 buffer failed\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test position information macros */

unsafe extern "C" fn test_byte_info_at_end() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_byte_info_at_end\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3850,
    );
    let mut text: *const c_char = b"<doc></doc>\x00".as_ptr() as *const c_char;
    if XML_GetCurrentByteIndex(g_parser) != -1 || XML_GetCurrentByteCount(g_parser) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3855i32,
            b"Byte index/count incorrect at start of parse\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3858i32,
        );
    }
    /* At end, the count will be zero and the index the end of string */
    if XML_GetCurrentByteCount(g_parser) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3861i32,
            b"Terminal byte count incorrect\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetCurrentByteIndex(g_parser) != strlen(text) as XML_Index {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3863i32,
            b"Terminal byte index incorrect\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test position information from errors */

pub const PRE_ERROR_STR: [c_char; 8] =
    unsafe { *::std::mem::transmute::<&[u8; 8], &[c_char; 8]>(b"<doc></\x00") };

unsafe extern "C" fn test_byte_info_at_error() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_byte_info_at_error\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3870,
    );
    let mut text: *const c_char = b"<doc></wombat></doc>\x00".as_ptr() as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_OK_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3875i32,
            b"Syntax error not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetCurrentByteCount(g_parser) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3877i32,
            b"Error byte count incorrect\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetCurrentByteIndex(g_parser) as c_ulong != strlen(PRE_ERROR_STR.as_ptr()) {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3879i32,
            b"Error byte index incorrect\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn byte_character_handler(
    mut userData: *mut c_void,
    mut _s: *const XML_Char,
    mut len: c_int,
) {
    let mut offset: c_int = 0;
    let mut size: c_int = 0;
    let mut buffer: *const c_char = 0 as *const c_char;
    let mut data: *mut ByteTestData = userData as *mut ByteTestData;
    buffer = XML_GetInputContext(g_parser, &mut offset, &mut size);
    if buffer.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3902i32,
            b"Failed to get context buffer\x00".as_ptr() as *const c_char,
        );
    }
    if offset != (*data).start_element_len {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3904i32,
            b"Context offset in unexpected position\x00".as_ptr() as *const c_char,
        );
    }
    if len != (*data).cdata_len {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3906i32,
            b"CDATA length reported incorrectly\x00".as_ptr() as *const c_char,
        );
    }
    if size != (*data).total_string_len {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3908i32,
            b"Context size is not full buffer\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetCurrentByteIndex(g_parser) != offset as c_long {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3910i32,
            b"Character byte index incorrect\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetCurrentByteCount(g_parser) != len {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3912i32,
            b"Character byte count incorrect\x00".as_ptr() as *const c_char,
        );
    };
}

pub const START_ELEMENT: [c_char; 4] =
    unsafe { *::std::mem::transmute::<&[u8; 4], &[c_char; 4]>(b"<e>\x00") };

pub const CDATA_TEXT: [c_char; 6] =
    unsafe { *::std::mem::transmute::<&[u8; 6], &[c_char; 6]>(b"Hello\x00") };

unsafe extern "C" fn test_byte_info_at_cdata() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_byte_info_at_cdata\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3923,
    );
    let mut text: *const c_char = b"<e>Hello</e>\x00".as_ptr() as *const c_char;
    let mut offset: c_int = 0;
    let mut size: c_int = 0;
    let mut data: ByteTestData = ByteTestData {
        start_element_len: 0,
        cdata_len: 0,
        total_string_len: 0,
    };
    /* Check initial context is empty */
    if !XML_GetInputContext(g_parser, &mut offset, &mut size).is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3930i32,
            b"Unexpected context at start of parse\x00".as_ptr() as *const c_char,
        );
    }
    data.start_element_len = strlen(START_ELEMENT.as_ptr()) as c_int;
    data.cdata_len = strlen(CDATA_TEXT.as_ptr()) as c_int;
    data.total_string_len = strlen(text) as c_int;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            byte_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(g_parser, &mut data as *mut ByteTestData as *mut c_void);
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_OK_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3938i32,
        );
    };
}
/* Test predefined entities are correctly recognised */

unsafe extern "C" fn test_predefined_entities() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_predefined_entities\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3946,
    );
    let mut text: *const c_char =
        b"<doc>&lt;&gt;&amp;&quot;&apos;</doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"<doc>&lt;&gt;&amp;&quot;&apos;</doc>\x00".as_ptr() as *const c_char;
    let mut result: *const XML_Char = b"<>&\"\'\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    /* run_character_check uses XML_SetCharacterDataHandler(), which
     * unfortunately heads off a code path that we need to exercise.
     */
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3960i32,
        );
    }
    /* The default handler doesn't translate the entities */
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
    /* Now try again and check the translation */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    _run_character_check(
        text,
        result,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        3966,
    );
}
/* Regression test that an invalid tag in an external parameter
 * reference in an external DTD is correctly faulted.
 *
 * Only a few specific tags are legal in DTDs ignoring comments and
 * processing instructions, all of which begin with an exclamation
 * mark.  "<el/>" is not one of them, so the parser should raise an
 * error on encountering it.
 */

unsafe extern "C" fn external_entity_param(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text1: *const c_char =
        
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM \'004-2.ent\'>\n<!ENTITY % e2 \'%e1;\'>\n%e1;\n\x00".as_ptr() as *const c_char;
    let mut text2: *const c_char = b"<!ELEMENT el EMPTY>\n<el/>\n\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    if systemId.is_null() {
        return XML_STATUS_OK_0;
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            3997i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if strcmp(systemId, b"004-1.ent\x00".as_ptr() as *const c_char) == 0 {
        if _XML_Parse_SINGLE_BYTES(ext_parser, text1, strlen(text1) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4002i32,
                b"Inner DTD with invalid tag not rejected\x00".as_ptr() as *const c_char,
            );
        }
        if XML_GetErrorCode(ext_parser) != XML_ERROR_EXTERNAL_ENTITY_HANDLING {
            _xml_failure(
                ext_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4004i32,
            );
        }
    } else if strcmp(systemId, b"004-2.ent\x00".as_ptr() as *const c_char) == 0 {
        if _XML_Parse_SINGLE_BYTES(ext_parser, text2, strlen(text2) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4008i32,
                b"Invalid tag in external param not rejected\x00".as_ptr() as *const c_char,
            );
        }
        if XML_GetErrorCode(ext_parser) != XML_ERROR_SYNTAX {
            _xml_failure(
                ext_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4010i32,
            );
        }
    } else {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4012i32,
            b"Unknown system ID\x00".as_ptr() as *const c_char,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_ERROR_0;
}

unsafe extern "C" fn test_invalid_tag_in_dtd() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_invalid_tag_in_dtd\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4019,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'004-1.ent\'>\n<doc></doc>\n\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_param
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Invalid tag IN DTD external param not rejected\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4026,
    );
}
/* Test entities not quite the predefined ones are not mis-recognised */

unsafe extern "C" fn test_not_predefined_entities() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_not_predefined_entities\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4031,
    );
    let mut text: [*const c_char; 5] = [
        b"<doc>&pt;</doc>\x00".as_ptr() as *const c_char,
        b"<doc>&amo;</doc>\x00".as_ptr() as *const c_char,
        b"<doc>&quid;</doc>\x00".as_ptr() as *const c_char,
        b"<doc>&apod;</doc>\x00".as_ptr() as *const c_char,
        ::rexpat::stddef_h::NULL as *const c_char,
    ];
    let mut i: c_int = 0;
    while !text[i as usize].is_null() {
        _expect_failure(
            text[i as usize],
            XML_ERROR_UNDEFINED_ENTITY,
            b"Undefined entity not rejected\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4038,
        );
        XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        i += 1
    }
}
/* Test conditional inclusion (IGNORE) */

unsafe extern "C" fn external_entity_load_ignore(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char =
        b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4058i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4061i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_ignore_section() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 20], &[c_char; 20]>(b"test_ignore_section\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4067,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'foo\'>\n<doc><e>&entity;</e></doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\n&entity;\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_load_ignore
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetStartDoctypeDeclHandler(
        g_parser,
        Some(
            dummy_start_doctype_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    XML_SetEndDoctypeDeclHandler(
        g_parser,
        Some(dummy_end_doctype_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
    );
    XML_SetElementDeclHandler(
        g_parser,
        transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            dummy_start_element
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndElementHandler(
        g_parser,
        Some(dummy_end_element as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4086i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn external_entity_load_ignore_utf16(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let text: [c_char; 73] =
        *::std::mem::transmute::<&[u8; 73],
                                 &[c_char; 73]>(b"<\x00!\x00[\x00I\x00G\x00N\x00O\x00R\x00E\x00[\x00<\x00!\x00E\x00L\x00E\x00M\x00E\x00N\x00T\x00 \x00e\x00 \x00(\x00#\x00P\x00C\x00D\x00A\x00T\x00A\x00)\x00*\x00>\x00]\x00]\x00>\x00\x00");
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4108i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 73]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4111i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_ignore_section_utf16() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_ignore_section_utf16\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4117,
    );
    let text: [c_char; 85] =
        *::std::mem::transmute::<&[u8; 85],
                                 &[c_char; 85]>(b"<\x00!\x00D\x00O\x00C\x00T\x00Y\x00P\x00E\x00 \x00d\x00 \x00S\x00Y\x00S\x00T\x00E\x00M\x00 \x00\'\x00s\x00\'\x00>\x00\n\x00<\x00d\x00>\x00<\x00e\x00>\x00&\x00e\x00n\x00;\x00<\x00/\x00e\x00>\x00<\x00/\x00d\x00>\x00\x00");
    let mut expected: *const XML_Char =
        b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\n&en;\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_load_ignore_utf16
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetStartDoctypeDeclHandler(
        g_parser,
        Some(
            dummy_start_doctype_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    XML_SetEndDoctypeDeclHandler(
        g_parser,
        Some(dummy_end_doctype_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
    );
    XML_SetElementDeclHandler(
        g_parser,
        transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            dummy_start_element
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndElementHandler(
        g_parser,
        Some(dummy_end_element as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 85]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4139i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn external_entity_load_ignore_utf16_be(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let text: [c_char; 73] =
        *::std::mem::transmute::<&[u8; 73],
                                 &[c_char; 73]>(b"\x00<\x00!\x00[\x00I\x00G\x00N\x00O\x00R\x00E\x00[\x00<\x00!\x00E\x00L\x00E\x00M\x00E\x00N\x00T\x00 \x00e\x00 \x00(\x00#\x00P\x00C\x00D\x00A\x00T\x00A\x00)\x00*\x00>\x00]\x00]\x00>\x00");
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4161i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 73]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4164i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_ignore_section_utf16_be() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_ignore_section_utf16_be\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4170,
    );
    let text: [c_char; 85] =
        *::std::mem::transmute::<&[u8; 85],
                                 &[c_char; 85]>(b"\x00<\x00!\x00D\x00O\x00C\x00T\x00Y\x00P\x00E\x00 \x00d\x00 \x00S\x00Y\x00S\x00T\x00E\x00M\x00 \x00\'\x00s\x00\'\x00>\x00\n\x00<\x00d\x00>\x00<\x00e\x00>\x00&\x00e\x00n\x00;\x00<\x00/\x00e\x00>\x00<\x00/\x00d\x00>\x00");
    let mut expected: *const XML_Char =
        b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\n&en;\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_load_ignore_utf16_be
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetStartDoctypeDeclHandler(
        g_parser,
        Some(
            dummy_start_doctype_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    XML_SetEndDoctypeDeclHandler(
        g_parser,
        Some(dummy_end_doctype_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
    );
    XML_SetElementDeclHandler(
        g_parser,
        transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            dummy_start_element
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndElementHandler(
        g_parser,
        Some(dummy_end_element as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 85]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4193i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test mis-formatted conditional exclusion */

unsafe extern "C" fn test_bad_ignore_section() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_bad_ignore_section\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4199,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'foo\'>\n<doc><e>&entity;</e></doc>\x00".as_ptr() as *const c_char;
    let mut faults: [ExtFaults; 4] = [
        {
            let mut init = ext_faults {
                parse_text: b"<![IGNORE[<!ELEM\x00".as_ptr() as *const c_char,
                fail_text: b"Broken-off declaration not faulted\x00".as_ptr() as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_SYNTAX,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<![IGNORE[\x01]]>\x00".as_ptr() as *const c_char,
                fail_text: b"Invalid XML character not faulted\x00".as_ptr() as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_INVALID_TOKEN,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<![IGNORE[\xe2\x82\x00".as_ptr() as *const c_char,
                fail_text: b"Partial XML character not faulted\x00".as_ptr() as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_PARTIAL_CHAR,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
                fail_text: ::rexpat::stddef_h::NULL as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_NONE,
            };
            init
        },
    ];
    let mut fault: *mut ExtFaults = 0 as *mut ExtFaults;
    fault = &mut *faults.as_mut_ptr().offset(0) as *mut ExtFaults;
    while !(*fault).parse_text.is_null() {
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_faulter
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        XML_SetUserData(g_parser, fault as *mut c_void);
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Incomplete IGNORE section not failed\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4218,
        );
        XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        fault = fault.offset(1)
    }
}
/* Test recursive parsing */

unsafe extern "C" fn external_entity_valuer(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text1: *const c_char =
        
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM \'004-2.ent\'>\n<!ENTITY % e2 \'%e1;\'>\n%e1;\n\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    if systemId.is_null() {
        return XML_STATUS_OK_0;
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4241i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if strcmp(systemId, b"004-1.ent\x00".as_ptr() as *const c_char) == 0 {
        if _XML_Parse_SINGLE_BYTES(ext_parser, text1, strlen(text1) as c_int, XML_TRUE)
            == XML_STATUS_ERROR_0 as c_uint
        {
            _xml_failure(
                ext_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4245i32,
            );
        }
    } else if strcmp(systemId, b"004-2.ent\x00".as_ptr() as *const c_char) == 0 {
        let mut fault: *mut ExtFaults = *(parser as *mut *mut c_void) as *mut ExtFaults;
        let mut status: XML_Status = XML_STATUS_ERROR;
        let mut error: XML_Error = XML_ERROR_NONE;
        status = _XML_Parse_SINGLE_BYTES(
            ext_parser,
            (*fault).parse_text,
            strlen((*fault).parse_text) as c_int,
            XML_TRUE,
        );
        if (*fault).error == XML_ERROR_NONE {
            if status == XML_STATUS_ERROR_0 as c_uint {
                _xml_failure(
                    ext_parser,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                        .as_ptr() as *const c_char,
                    4255i32,
                );
            }
        } else {
            if status != XML_STATUS_ERROR_0 as c_uint {
                crate::minicheck::_fail_unless(
                    0i32,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                        .as_ptr() as *const c_char,
                    4258i32,
                    (*fault).fail_text,
                );
            }
            error = XML_GetErrorCode(ext_parser);
            if error != (*fault).error
                && ((*fault).error != XML_ERROR_XML_DECL || error != XML_ERROR_TEXT_DECL)
            {
                _xml_failure(
                    ext_parser,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                        .as_ptr() as *const c_char,
                    4263i32,
                );
            }
        }
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_external_entity_values() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 28], &[c_char; 28]>(b"test_external_entity_values\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4271,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'004-1.ent\'>\n<doc></doc>\n\x00".as_ptr() as *const c_char;
    let mut data_004_2: [ExtFaults; 12] = [
        {
            let mut init = ext_faults {
                parse_text: b"<!ATTLIST doc a1 CDATA \'value\'>\x00".as_ptr() as *const c_char,
                fail_text: ::rexpat::stddef_h::NULL as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_NONE,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<!ATTLIST $doc a1 CDATA \'value\'>\x00".as_ptr() as *const c_char,
                fail_text: b"Invalid token not faulted\x00".as_ptr() as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_INVALID_TOKEN,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"\'wombat\x00".as_ptr() as *const c_char,
                fail_text: b"Unterminated string not faulted\x00".as_ptr() as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"\xe2\x82\x00".as_ptr() as *const c_char,
                fail_text: b"Partial UTF-8 character not faulted\x00".as_ptr() as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_PARTIAL_CHAR,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n\x00".as_ptr()
                    as *const c_char,
                fail_text: ::rexpat::stddef_h::NULL as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_NONE,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<?xml?>\x00".as_ptr() as *const c_char,
                fail_text: b"Malformed XML declaration not faulted\x00".as_ptr() as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_XML_DECL,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"\xef\xbb\xbf<!ATTLIST doc a1 CDATA \'value\'>\x00".as_ptr()
                    as *const c_char,
                fail_text: ::rexpat::stddef_h::NULL as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_NONE,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n$\x00".as_ptr()
                    as *const c_char,
                fail_text: b"Invalid token after text declaration not faulted\x00".as_ptr()
                    as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_INVALID_TOKEN,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n\'wombat\x00".as_ptr()
                    as *const c_char,
                fail_text: b"Unterminated string after text decl not faulted\x00".as_ptr()
                    as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_UNCLOSED_TOKEN,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n\xe2\x82\x00".as_ptr()
                    as *const c_char,
                fail_text: b"Partial UTF-8 character after text decl not faulted\x00".as_ptr()
                    as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_PARTIAL_CHAR,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: b"%e1;\x00".as_ptr() as *const c_char,
                fail_text: b"Recursive parameter entity not faulted\x00".as_ptr() as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_RECURSIVE_ENTITY_REF,
            };
            init
        },
        {
            let mut init = ext_faults {
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
                fail_text: ::rexpat::stddef_h::NULL as *const c_char,
                encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                error: XML_ERROR_NONE,
            };
            init
        },
    ];
    let mut i: c_int = 0;
    i = 0;
    while !data_004_2[i as usize].parse_text.is_null() {
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_valuer
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        XML_SetUserData(
            g_parser,
            &mut *data_004_2.as_mut_ptr().offset(i as isize) as *mut ExtFaults as *mut c_void,
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            == XML_STATUS_ERROR_0 as c_uint
        {
            _xml_failure(
                g_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4308i32,
            );
        }
        XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        i += 1
    }
}
/* Test the recursive parse interacts with a not standalone handler */

unsafe extern "C" fn external_entity_not_standalone(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text1: *const c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM \'bar\'>\n%e1;\n\x00".as_ptr()
            as *const c_char;
    let mut text2: *const c_char =
        b"<!ATTLIST doc a1 CDATA \'value\'>\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    if systemId.is_null() {
        return XML_STATUS_OK_0;
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4331i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if strcmp(systemId, b"foo\x00".as_ptr() as *const c_char) == 0 {
        XML_SetNotStandaloneHandler(
            ext_parser,
            Some(reject_not_standalone_handler as unsafe extern "C" fn(_: *mut c_void) -> c_int),
        );
        if _XML_Parse_SINGLE_BYTES(ext_parser, text1, strlen(text1) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4336i32,
                b"Expected not standalone rejection\x00".as_ptr() as *const c_char,
            );
        }
        if XML_GetErrorCode(ext_parser) != XML_ERROR_NOT_STANDALONE {
            _xml_failure(
                ext_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4338i32,
            );
        }
        XML_SetNotStandaloneHandler(
            ext_parser,
            ::std::mem::transmute::<libc::intptr_t, XML_NotStandaloneHandler>(
                ::rexpat::stddef_h::NULL as libc::intptr_t,
            ),
        );
        XML_ParserFree(ext_parser);
        return XML_STATUS_ERROR_0;
    } else {
        if strcmp(systemId, b"bar\x00".as_ptr() as *const c_char) == 0 {
            if _XML_Parse_SINGLE_BYTES(ext_parser, text2, strlen(text2) as c_int, XML_TRUE)
                == XML_STATUS_ERROR_0 as c_uint
            {
                _xml_failure(
                    ext_parser,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                        .as_ptr() as *const c_char,
                    4345i32,
                );
            }
        }
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_ext_entity_not_standalone() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_ext_entity_not_standalone\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4352,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'foo\'>\n<doc></doc>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_not_standalone
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Standalone rejection not caught\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4359,
    );
}

unsafe extern "C" fn external_entity_value_aborter(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text1: *const c_char =
        
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM \'004-2.ent\'>\n<!ENTITY % e2 \'%e1;\'>\n%e1;\n\x00".as_ptr() as *const c_char;
    let mut text2: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    if systemId.is_null() {
        return XML_STATUS_OK_0;
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4380i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if strcmp(systemId, b"004-1.ent\x00".as_ptr() as *const c_char) == 0 {
        if _XML_Parse_SINGLE_BYTES(ext_parser, text1, strlen(text1) as c_int, XML_TRUE)
            == XML_STATUS_ERROR_0 as c_uint
        {
            _xml_failure(
                ext_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4384i32,
            );
        }
    }
    if strcmp(systemId, b"004-2.ent\x00".as_ptr() as *const c_char) == 0 {
        XML_SetXmlDeclHandler(
            ext_parser,
            Some(
                entity_suspending_xdecl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(ext_parser, ext_parser as *mut c_void);
        if _XML_Parse_SINGLE_BYTES(ext_parser, text2, strlen(text2) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4391i32,
                b"Aborted parse not faulted\x00".as_ptr() as *const c_char,
            );
        }
        if XML_GetErrorCode(ext_parser) != XML_ERROR_ABORTED {
            _xml_failure(
                ext_parser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                4393i32,
            );
        }
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_ext_entity_value_abort() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 28], &[c_char; 28]>(b"test_ext_entity_value_abort\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4400,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'004-1.ent\'>\n<doc></doc>\n\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_value_aborter
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    resumable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4409i32,
        );
    };
}

unsafe extern "C" fn test_bad_public_doctype() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_bad_public_doctype\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4413,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n<!DOCTYPE doc PUBLIC \'{BadName}\' \'test\'>\n<doc></doc>\x00".as_ptr() as *const c_char;
    /* Setting a handler provokes a particular code path */
    XML_SetDoctypeDeclHandler(
        g_parser,
        Some(
            dummy_start_doctype_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
        Some(dummy_end_doctype_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
    );
    _expect_failure(
        text,
        XML_ERROR_PUBLICID,
        b"Bad Public ID not failed\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4421,
    );
}
/* Test based on ibm/valid/P32/ibm32v04.xml */

unsafe extern "C" fn test_attribute_enum_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_attribute_enum_value\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4426,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' standalone=\'no\'?>\n<!DOCTYPE animal SYSTEM \'test.dtd\'>\n<animal>This is a \n    <a/>  \n\nyellow tiger</animal>\x00".as_ptr() as *const c_char;
    let mut dtd_data: ExtTest = {
        let mut init =
                ExtTest{parse_text:
                            
                            b"<!ELEMENT animal (#PCDATA|a)*>\n<!ELEMENT a EMPTY>\n<!ATTLIST animal xml:space (default|preserve) \'preserve\'>\x00".as_ptr() as *const c_char,
                        encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                        storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,};
        init
    };
    let mut expected: *const XML_Char =
        b"This is a \n      \n\nyellow tiger\x00".as_ptr() as *const c_char;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut dtd_data as *mut ExtTest as *mut c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    /* An attribute list handler provokes a different code path */
    XML_SetAttlistDeclHandler(
        g_parser,
        Some(
            dummy_attlist_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    _run_ext_character_check(
        text,
        &mut dtd_data,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4442,
    );
}
/* Slightly bizarrely, the library seems to silently ignore entity
 * definitions for predefined entities, even when they are wrong.  The
 * language of the XML 1.0 spec is somewhat unhelpful as to what ought
 * to happen, so this is currently treated as acceptable.
 */

unsafe extern "C" fn test_predefined_entity_redefinition() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_predefined_entity_redefinition\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4451,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n<!ENTITY apos \'foo\'>\n]>\n<doc>&apos;</doc>\x00".as_ptr()
            as *const c_char;
    _run_character_check(
        text,
        b"\'\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4456,
    );
}
/* Test that the parser stops processing the DTD after an unresolved
 * parameter entity is encountered.
 */

unsafe extern "C" fn test_dtd_stop_processing() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_dtd_stop_processing\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4463,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n%foo;\n<!ENTITY bar \'bas\'>\n]><doc/>\x00".as_ptr() as *const c_char;
    XML_SetEntityDeclHandler(
        g_parser,
        Some(
            dummy_entity_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: c_int,
                    _: *const XML_Char,
                    _: c_int,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    dummy_handler_flags = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4473i32,
        );
    }
    if dummy_handler_flags != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4475i32,
            b"DTD processing still going after undefined PE\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test public notations with no system ID */

unsafe extern "C" fn test_public_notation_no_sysid() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_public_notation_no_sysid\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4480,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n<!NOTATION note PUBLIC \'foo\'>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\x00"
            .as_ptr() as *const c_char;
    dummy_handler_flags = 0;
    XML_SetNotationDeclHandler(
        g_parser,
        Some(
            dummy_notation_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4490i32,
        );
    }
    if dummy_handler_flags != DUMMY_NOTATION_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4492i32,
            b"Notation declaration handler not called\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn record_element_start_handler(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
    crate::chardata::CharData_AppendXMLChars(
        userData as *mut crate::chardata::CharData,
        name,
        strlen(name) as c_int,
    );
}

unsafe extern "C" fn test_nested_groups() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 19], &[c_char; 19]>(b"test_nested_groups\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4503,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ELEMENT doc (e,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?))))))))))))))))))))))))))))))))>\n<!ELEMENT e EMPTY>]>\n<doc><e/></doc>\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetElementDeclHandler(
        g_parser,
        transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            record_element_start_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    dummy_handler_flags = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4523i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"doce\x00".as_ptr() as *const c_char,
    );
    if dummy_handler_flags != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4526i32,
            b"Element handler not fired\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_group_choice() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_group_choice\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4530,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ELEMENT doc (a|b|c)+>\n<!ELEMENT a EMPTY>\n<!ELEMENT b (#PCDATA)>\n<!ELEMENT c ANY>\n]>\n<doc>\n<a/>\n<b attr=\'foo\'>This is a foo</b>\n<c></c>\n</doc>\n\x00".as_ptr() as *const c_char;
    XML_SetElementDeclHandler(
        g_parser,
        transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Content,
                ) -> (),
        )),
    );
    dummy_handler_flags = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4547i32,
        );
    }
    if dummy_handler_flags != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4549i32,
            b"Element handler flag not raised\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn external_entity_public(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> c_int {
    let mut text1: *const c_char = *(parser as *mut *mut c_void) as *const c_char;
    let mut text2: *const c_char =
        b"<!ATTLIST doc a CDATA \'value\'>\x00".as_ptr() as *const c_char;
    let mut text: *const c_char = ::rexpat::stddef_h::NULL as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut parse_res: c_int = 0;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        return XML_STATUS_ERROR_0;
    }
    if !systemId.is_null()
        && strcmp(
            systemId,
            b"http://example.org/\x00".as_ptr() as *const c_char,
        ) == 0
    {
        text = text1
    } else if !publicId.is_null() && strcmp(publicId, b"foo\x00".as_ptr() as *const c_char) == 0 {
        text = text2
    } else {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4572i32,
            b"Unexpected parameters to external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if !text.is_null() {
    } else {
        __assert_fail(b"text != NULL\x00".as_ptr() as *const c_char,
                      
                      b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr() as *const c_char,
                      4573u32,
                      (*::std::mem::transmute::<&[u8; 111],
                                                &[c_char; 111]>(b"int external_entity_public(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\x00")).as_ptr());
    }
    parse_res = _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE) as c_int;
    XML_ParserFree(ext_parser);
    return parse_res;
}

unsafe extern "C" fn test_standalone_parameter_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_standalone_parameter_entity\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4580,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' standalone=\'yes\'?>\n<!DOCTYPE doc SYSTEM \'http://example.org/\' [\n<!ENTITY % entity \'<!ELEMENT doc (#PCDATA)>\'>\n%entity;\n]>\n<doc></doc>\x00".as_ptr() as *const c_char;
    let mut dtd_data: [c_char; 22] =
        *::std::mem::transmute::<&[u8; 22], &mut [c_char; 22]>(b"<!ENTITY % e1 \'foo\'>\n\x00");
    XML_SetUserData(g_parser, dtd_data.as_mut_ptr() as *mut c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_public
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4594i32,
        );
    };
}
/* Test skipping of parameter entity in an external DTD */
/* Derived from ibm/invalid/P69/ibm69i01.xml */

unsafe extern "C" fn test_skipped_parameter_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_skipped_parameter_entity\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4600,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\'?>\n<!DOCTYPE root SYSTEM \'http://example.org/dtd.ent\' [\n<!ELEMENT root (#PCDATA|a)* >\n]>\n<root></root>\x00".as_ptr() as *const c_char;
    let mut dtd_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"%pe2;\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut dtd_data as *mut ExtTest as *mut c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetSkippedEntityHandler(
        g_parser,
        Some(
            dummy_skip_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    dummy_handler_flags = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4615i32,
        );
    }
    if dummy_handler_flags != DUMMY_SKIP_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4617i32,
            b"Skip handler not executed\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test recursive parameter entity definition rejected in external DTD */

unsafe extern "C" fn test_recursive_external_parameter_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 41], &[c_char; 41]>(
            b"test_recursive_external_parameter_entity\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4622,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\'?>\n<!DOCTYPE root SYSTEM \'http://example.org/dtd.ent\' [\n<!ELEMENT root (#PCDATA|a)* >\n]>\n<root></root>\x00".as_ptr() as *const c_char;
    let mut dtd_data: ExtFaults = {
        let mut init = ext_faults {
            parse_text: b"<!ENTITY % pe2 \'&#37;pe2;\'>\n%pe2;\x00".as_ptr() as *const c_char,
            fail_text: b"Recursive external parameter entity not faulted\x00".as_ptr()
                as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            error: XML_ERROR_RECURSIVE_ENTITY_REF,
        };
        init
    };
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut dtd_data as *mut ExtFaults as *mut c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Recursive external parameter not spotted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4636,
    );
}
/* Test undefined parameter entity in external entity handler */

unsafe extern "C" fn external_entity_devaluer(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char = b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM \'bar\'>\n%e1;\n\x00"
        .as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut clear_handler: intptr_t = *(parser as *mut *mut c_void) as intptr_t;
    if systemId.is_null() || strcmp(systemId, b"bar\x00".as_ptr() as *const c_char) == 0 {
        return XML_STATUS_OK_0;
    }
    if strcmp(systemId, b"foo\x00".as_ptr() as *const c_char) != 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4656i32,
            b"Unexpected system ID\x00".as_ptr() as *const c_char,
        );
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4659i32,
            b"Could note create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if clear_handler != 0 {
        XML_SetExternalEntityRefHandler(
            ext_parser,
            ::std::mem::transmute::<libc::intptr_t, XML_ExternalEntityRefHandler>(
                ::rexpat::stddef_h::NULL as libc::intptr_t,
            ),
        );
    }
    if _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4664i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_undefined_ext_entity_in_external_dtd() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 42], &[c_char; 42]>(
            b"test_undefined_ext_entity_in_external_dtd\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4670,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'foo\'>\n<doc></doc>\n\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_devaluer
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, 0 as *mut c_void);
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4679i32,
        );
    }
    /* Now repeat without the external entity ref handler invoking
     * another copy of itself.
     */
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_devaluer
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, XML_TRUE as *mut c_void);
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4690i32,
        );
    };
}

unsafe extern "C" fn aborting_xdecl_handler(
    mut _userData: *mut c_void,
    mut _version: *const XML_Char,
    mut _encoding: *const XML_Char,
    mut _standalone: c_int,
) {
    XML_StopParser(g_parser, resumable);
    XML_SetXmlDeclHandler(
        g_parser,
        ::std::mem::transmute::<libc::intptr_t, XML_XmlDeclHandler>(
            ::rexpat::stddef_h::NULL as libc::intptr_t,
        ),
    );
}
/* Test suspending the parse on receiving an XML declaration works */

unsafe extern "C" fn test_suspend_xdecl() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 19], &[c_char; 19]>(b"test_suspend_xdecl\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4706,
    );
    let mut text: *const c_char = long_character_data_text;
    XML_SetXmlDeclHandler(
        g_parser,
        Some(
            aborting_xdecl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: c_int,
                ) -> (),
        ),
    );
    resumable = XML_TRUE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_SUSPENDED_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4713i32,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NONE {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4715i32,
        );
    }
    /* Attempt to start a new parse while suspended */
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4719i32,
            b"Attempt to parse while suspended not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_SUSPENDED {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4721i32,
            b"Suspended parse not faulted with correct error\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test aborting the parse in an epilog works */

unsafe extern "C" fn selective_aborting_default_handler(
    mut userData: *mut c_void,
    mut s: *const XML_Char,
    mut len: c_int,
) {
    let mut match_0: *const XML_Char = userData as *const XML_Char;
    if match_0.is_null()
        || strlen(match_0) == len as c_uint as c_ulong && strncmp(match_0, s, len as c_ulong) == 0
    {
        XML_StopParser(g_parser, resumable);
        XML_SetDefaultHandler(
            g_parser,
            ::std::mem::transmute::<libc::intptr_t, XML_DefaultHandler>(
                ::rexpat::stddef_h::NULL as libc::intptr_t,
            ),
        );
    };
}

unsafe extern "C" fn test_abort_epilog() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_abort_epilog\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4737,
    );
    let mut text: *const c_char = b"<doc></doc>\n\r\n\x00".as_ptr() as *const c_char;
    let mut match_0: [XML_Char; 2] =
        *::std::mem::transmute::<&[u8; 2], &mut [XML_Char; 2]>(b"\r\x00");
    XML_SetDefaultHandler(
        g_parser,
        Some(
            selective_aborting_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(g_parser, match_0.as_mut_ptr() as *mut c_void);
    resumable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4746i32,
            b"Abort not triggered\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_ABORTED {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4748i32,
        );
    };
}
/* Test a different code path for abort in the epilog */

unsafe extern "C" fn test_abort_epilog_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 20], &[c_char; 20]>(b"test_abort_epilog_2\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4753,
    );
    let mut text: *const c_char = b"<doc></doc>\n\x00".as_ptr() as *const c_char;
    let mut match_0: [XML_Char; 2] =
        *::std::mem::transmute::<&[u8; 2], &mut [XML_Char; 2]>(b"\n\x00");
    XML_SetDefaultHandler(
        g_parser,
        Some(
            selective_aborting_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(g_parser, match_0.as_mut_ptr() as *mut c_void);
    resumable = XML_FALSE;
    _expect_failure(
        text,
        XML_ERROR_ABORTED,
        b"Abort not triggered\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4760,
    );
}
/* Test suspension from the epilog */

unsafe extern "C" fn test_suspend_epilog() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 20], &[c_char; 20]>(b"test_suspend_epilog\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4765,
    );
    let mut text: *const c_char = b"<doc></doc>\n\x00".as_ptr() as *const c_char;
    let mut match_0: [XML_Char; 2] =
        *::std::mem::transmute::<&[u8; 2], &mut [XML_Char; 2]>(b"\n\x00");
    XML_SetDefaultHandler(
        g_parser,
        Some(
            selective_aborting_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(g_parser, match_0.as_mut_ptr() as *mut c_void);
    resumable = XML_TRUE;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_SUSPENDED_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4774i32,
        );
    };
}

unsafe extern "C" fn suspending_end_handler(mut userData: *mut c_void, mut _s: *const XML_Char) {
    XML_StopParser(userData as XML_Parser, 1);
}

unsafe extern "C" fn test_suspend_in_sole_empty_tag() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_suspend_in_sole_empty_tag\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4784,
    );
    let mut text: *const c_char = b"<doc/>\x00".as_ptr() as *const c_char;
    let mut rc: XML_Status = XML_STATUS_ERROR;
    XML_SetEndElementHandler(
        g_parser,
        Some(
            suspending_end_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    XML_SetUserData(g_parser, g_parser as *mut c_void);
    rc = _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE);
    if rc == XML_STATUS_ERROR_0 as c_uint {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4792i32,
        );
    } else if rc != XML_STATUS_SUSPENDED_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4794i32,
            b"Suspend not triggered\x00".as_ptr() as *const c_char,
        );
    }
    rc = XML_ResumeParser(g_parser);
    if rc == XML_STATUS_ERROR_0 as c_uint {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4797i32,
        );
    } else if rc != XML_STATUS_OK_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4799i32,
            b"Resume failed\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_unfinished_epilog() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_unfinished_epilog\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4803,
    );
    let mut text: *const c_char = b"<doc></doc><\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_UNCLOSED_TOKEN,
        b"Incomplete epilog entry not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4807,
    );
}

unsafe extern "C" fn test_partial_char_in_epilog() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 28], &[c_char; 28]>(b"test_partial_char_in_epilog\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4811,
    );
    let mut text: *const c_char = b"<doc></doc>\xe2\x82\x00".as_ptr() as *const c_char;
    /* First check that no fault is raised if the parse is not finished */
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4817i32,
        );
    }
    /* Now check that it is faulted once we finish */
    if XML_ParseBuffer(g_parser, 0, XML_TRUE as c_int) != XML_STATUS_ERROR_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4820i32,
            b"Partial character in epilog not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_PARTIAL_CHAR {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4822i32,
        );
    };
}

unsafe extern "C" fn test_hash_collision() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 20], &[c_char; 20]>(b"test_hash_collision\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4826,
    );
    /* For full coverage of the lookup routine, we need to ensure a
     * hash collision even though we can only tell that we have one
     * through breakpoint debugging or coverage statistics.  The
     * following will cause a hash collision on machines with a 64-bit
     * long type; others will have to experiment.  The full coverage
     * tests invoked from qa.sh usually provide a hash collision, but
     * not always.  This is an attempt to provide insurance.
     */
    let mut text: *const c_char =
        
        b"<doc>\n<a1/><a2/><a3/><a4/><a5/><a6/><a7/><a8/>\n<b1></b1><b2 attr=\'foo\'>This is a foo</b2><b3></b3><b4></b4>\n<b5></b5><b6></b6><b7></b7><b8></b8>\n<c1/><c2/><c3/><c4/><c5/><c6/><c7/><c8/>\n<d1/><d2/><d3/><d4/><d5/><d6/><d7/>\n<d8>This triggers the table growth and collides with b2</d8>\n</doc>\n\x00".as_ptr() as *const c_char;
    XML_SetHashSalt(g_parser, (0xffffffff) << 32 | 0xff99fc90);
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4849i32,
        );
    };
}
/* Test resuming a parse suspended in entity substitution */

unsafe extern "C" fn start_element_suspender(
    mut _userData: *mut c_void,
    mut name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
    if strcmp(name, b"suspend\x00".as_ptr() as *const c_char) == 0 {
        XML_StopParser(g_parser, XML_TRUE);
    }
    if strcmp(name, b"abort\x00".as_ptr() as *const c_char) == 0 {
        XML_StopParser(g_parser, XML_FALSE);
    };
}

unsafe extern "C" fn test_suspend_resume_internal_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_suspend_resume_internal_entity\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4866,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ENTITY foo \'<suspend>Hi<suspend>Ho</suspend></suspend>\'>\n]>\n<doc>&foo;</doc>\n\x00".as_ptr() as *const c_char;
    let mut expected1: *const XML_Char = b"Hi\x00".as_ptr() as *const c_char;
    let mut expected2: *const XML_Char = b"HiHo\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_suspender
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_SUSPENDED_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4882i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"\x00".as_ptr() as *const c_char,
    );
    if XML_ResumeParser(g_parser) != XML_STATUS_SUSPENDED_0 as c_uint {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4885i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected1);
    if XML_ResumeParser(g_parser) != XML_STATUS_OK_0 as c_uint {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4888i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected2);
}
/* Test syntax error is caught at parse resumption */

unsafe extern "C" fn test_resume_entity_with_syntax_error() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 37], &[c_char; 37]>(
            b"test_resume_entity_with_syntax_error\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4894,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n<!ENTITY foo \'<suspend>Hi</wombat>\'>\n]>\n<doc>&foo;</doc>\n\x00"
            .as_ptr() as *const c_char;
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_suspender
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_SUSPENDED_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4903i32,
        );
    }
    if XML_ResumeParser(g_parser) != XML_STATUS_ERROR_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4905i32,
            b"Syntax error in entity not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_TAG_MISMATCH {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4907i32,
        );
    };
}
/* Test suspending and resuming in a parameter entity substitution */

unsafe extern "C" fn element_decl_suspender(
    mut _userData: *mut c_void,
    mut _name: *const XML_Char,
    mut model: *mut XML_Content,
) {
    XML_StopParser(g_parser, XML_TRUE);
    XML_FreeContentModel(g_parser, model);
}

unsafe extern "C" fn test_suspend_resume_parameter_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 37], &[c_char; 37]>(
            b"test_suspend_resume_parameter_entity\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4921,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ENTITY % foo \'<!ELEMENT doc (#PCDATA)*>\'>\n%foo;\n]>\n<doc>Hello, world</doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char = b"Hello, world\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetElementDeclHandler(
        g_parser,
        transmute(Some(
            element_decl_suspender
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_SUSPENDED_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4937i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"\x00".as_ptr() as *const c_char,
    );
    if XML_ResumeParser(g_parser) != XML_STATUS_OK_0 as c_uint {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4940i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test attempting to use parser after an error is faulted */

unsafe extern "C" fn test_restart_on_error() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_restart_on_error\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4946,
    );
    let mut text: *const c_char = b"<$doc><doc></doc>\x00".as_ptr() as *const c_char;
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4951i32,
            b"Invalid tag name not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_INVALID_TOKEN {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4953i32,
        );
    }
    if XML_Parse(
        g_parser,
        ::rexpat::stddef_h::NULL as *const c_char,
        0,
        XML_TRUE as c_int,
    ) != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4955i32,
            b"Restarting invalid parse not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_INVALID_TOKEN {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4957i32,
        );
    };
}
/* Test that angle brackets in an attribute default value are faulted */

unsafe extern "C" fn test_reject_lt_in_attribute_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 34], &[c_char; 34]>(
            b"test_reject_lt_in_attribute_value\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4962,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [<!ATTLIST doc a CDATA \'<bar>\'>]>\n<doc></doc>\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad attribute default not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4967,
    );
}

unsafe extern "C" fn test_reject_unfinished_param_in_att_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 42], &[c_char; 42]>(
            b"test_reject_unfinished_param_in_att_value\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4971,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [<!ATTLIST doc a CDATA \'&foo\'>]>\n<doc></doc>\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad attribute default not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4976,
    );
}

unsafe extern "C" fn test_trailing_cr_in_att_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_trailing_cr_in_att_value\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4980,
    );
    let mut text: *const c_char = b"<doc a=\'value\r\'/>\x00".as_ptr() as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            4985i32,
        );
    };
}
/* Try parsing a general entity within a parameter entity in a
 * standalone internal DTD.  Covers a corner case in the parser.
 */

unsafe extern "C" fn test_standalone_internal_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_standalone_internal_entity\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        4992,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' standalone=\'yes\' ?>\n<!DOCTYPE doc [\n  <!ELEMENT doc (#PCDATA)>\n  <!ENTITY % pe \'<!ATTLIST doc att2 CDATA \"&ge;\">\'>\n  <!ENTITY ge \'AttDefaultValue\'>\n  %pe;\n]>\n<doc att2=\'any\'/>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5005i32,
        );
    };
}
/* Test that a reference to an unknown external entity is skipped */

unsafe extern "C" fn test_skipped_external_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_skipped_external_entity\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5010,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'http://example.org/\'>\n<doc></doc>\n\x00".as_ptr()
            as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<!ELEMENT doc EMPTY>\n<!ENTITY % e2 \'%e1;\'>\n\x00".as_ptr()
                as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest as *mut c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5022i32,
        );
    };
}

unsafe extern "C" fn external_entity_oneshot_loader(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut test_data: *mut ExtHdlrData = *(parser as *mut *mut c_void) as *mut ExtHdlrData;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5044i32,
            b"Could not create external entity parser.\x00".as_ptr() as *const c_char,
        );
    }
    /* Use the requested entity parser for further externals */
    XML_SetExternalEntityRefHandler(ext_parser, (*test_data).handler);
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        (*test_data).parse_text,
        strlen((*test_data).parse_text) as c_int,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            ext_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5050i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_skipped_null_loaded_ext_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_skipped_null_loaded_ext_entity\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5057,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'http://example.org/one.ent\'>\n<doc />\x00".as_ptr()
            as *const c_char;
    let mut test_data: ExtHdlrData = {
        let mut init =
                ext_hdlr_data{parse_text:
                                  
                                  b"<!ENTITY % pe1 SYSTEM \'http://example.org/two.ent\'>\n<!ENTITY % pe2 \'%pe1;\'>\n%pe2;\n\x00".as_ptr() as *const c_char,
                              handler:
                                  Some(external_entity_null_loader as
                                           unsafe extern "C" fn(_: XML_Parser,
                                                                _:
                                                                    *const XML_Char,
                                                                _:
                                                                    *const XML_Char,
                                                                _:
                                                                    *const XML_Char,
                                                                _:
                                                                    *const XML_Char)
                                               -> c_int),};
        init
    };
    XML_SetUserData(g_parser, &mut test_data as *mut ExtHdlrData as *mut c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_oneshot_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5071i32,
        );
    };
}

unsafe extern "C" fn test_skipped_unloaded_ext_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_skipped_unloaded_ext_entity\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5075,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'http://example.org/one.ent\'>\n<doc />\x00".as_ptr()
            as *const c_char;
    let mut test_data: ExtHdlrData = {
        let mut init =
                ext_hdlr_data{parse_text:
                                  
                                  b"<!ENTITY % pe1 SYSTEM \'http://example.org/two.ent\'>\n<!ENTITY % pe2 \'%pe1;\'>\n%pe2;\n\x00".as_ptr() as *const c_char,
                              handler:
                                  ::std::mem::transmute::<libc::intptr_t,
                                                          XML_ExternalEntityRefHandler>(::rexpat::stddef_h::NULL
                                                                                            as
                                                                                            libc::intptr_t),};
        init
    };
    XML_SetUserData(g_parser, &mut test_data as *mut ExtHdlrData as *mut c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_oneshot_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5089i32,
        );
    };
}
/* Test that a parameter entity value ending with a carriage return
 * has it translated internally into a newline.
 */

unsafe extern "C" fn test_param_entity_with_trailing_cr() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_param_entity_with_trailing_cr\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5096,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'http://example.org/\'>\n<doc/>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest = {
        let mut init = ExtTest {
            parse_text: b"<!ENTITY % pe \'<!ATTLIST doc att CDATA \"default\">\r\'>\n%pe;\n\x00"
                .as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
        };
        init
    };
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest as *mut c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetEntityDeclHandler(
        g_parser,
        Some(
            param_entity_match_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: c_int,
                    _: *const XML_Char,
                    _: c_int,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    entity_name_to_match = b"pe\x00".as_ptr() as *const c_char;
    entity_value_to_match = b"<!ATTLIST doc att CDATA \"default\">\n\x00".as_ptr() as *const c_char;
    entity_match_flag = ENTITY_MATCH_NOT_FOUND;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5115i32,
        );
    }
    if entity_match_flag == ENTITY_MATCH_FAIL {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5117i32,
            b"Parameter entity CR->NEWLINE conversion failed\x00".as_ptr() as *const c_char,
        );
    } else if entity_match_flag == ENTITY_MATCH_NOT_FOUND {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5119i32,
            b"Parameter entity not parsed\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_invalid_character_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_invalid_character_entity\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5125,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY entity \'&#x110000;\'>\n]>\n<doc>&entity;</doc>\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_BAD_CHAR_REF,
        b"Out of range character reference not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5132,
    );
}

unsafe extern "C" fn test_invalid_character_entity_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_invalid_character_entity_2\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5136,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY entity \'&#xg0;\'>\n]>\n<doc>&entity;</doc>\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Out of range character reference not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5143,
    );
}

unsafe extern "C" fn test_invalid_character_entity_3() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_invalid_character_entity_3\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5147,
    );
    let text: [c_char; 125] =
        *::std::mem::transmute::<&[u8; 125],
                                 &[c_char; 125]>(b"\x00<\x00!\x00D\x00O\x00C\x00T\x00Y\x00P\x00E\x00 \x00d\x00o\x00c\x00 \x00[\x00\n\x00<\x00!\x00E\x00N\x00T\x00I\x00T\x00Y\x00 \x00e\x00n\x00t\x00i\x00t\x00y\x00 \x00\'\x00&\x0e\x04\x0e\x08\x00;\x00\'\x00>\x00\n\x00]\x00>\x00\n\x00<\x00d\x00o\x00c\x00>\x00&\x00e\x00n\x00t\x00i\x00t\x00y\x00;\x00<\x00/\x00d\x00o\x00c\x00>\x00");
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 125]>() as c_int - 1,
        XML_TRUE,
    ) != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5163i32,
            b"Invalid start of entity name not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_UNDEFINED_ENTITY {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5165i32,
        );
    };
}

unsafe extern "C" fn test_invalid_character_entity_4() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_invalid_character_entity_4\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5169,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY entity \'&#1114112;\'>\n]>\n<doc>&entity;</doc>\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_BAD_CHAR_REF,
        b"Out of range character reference not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5176,
    );
}
/* Test that processing instructions are picked up by a default handler */

unsafe extern "C" fn test_pi_handled_in_default() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_pi_handled_in_default\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5181,
    );
    let mut text: *const c_char =
        b"<?test processing instruction?>\n<doc/>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"<?test processing instruction?>\n<doc/>\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5191i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test that comments are picked up by a default handler */

unsafe extern "C" fn test_comment_handled_in_default() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_comment_handled_in_default\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5197,
    );
    let mut text: *const c_char =
        b"<!-- This is a comment -->\n<doc/>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"<!-- This is a comment -->\n<doc/>\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5207i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test PIs that look almost but not quite like XML declarations */

unsafe extern "C" fn accumulate_pi_characters(
    mut userData: *mut c_void,
    mut target: *const XML_Char,
    mut data: *const XML_Char,
) {
    let mut storage: *mut crate::chardata::CharData = userData as *mut crate::chardata::CharData;
    crate::chardata::CharData_AppendXMLChars(storage, target, -(1));
    crate::chardata::CharData_AppendXMLChars(storage, b": \x00".as_ptr() as *const c_char, 2);
    crate::chardata::CharData_AppendXMLChars(storage, data, -(1));
    crate::chardata::CharData_AppendXMLChars(storage, b"\n\x00".as_ptr() as *const c_char, 1);
}

unsafe extern "C" fn test_pi_yml() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 12], &[c_char; 12]>(b"test_pi_yml\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5224,
    );
    let mut text: *const c_char =
        b"<?yml something like data?><doc/>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char = b"yml: something like data\n\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            accumulate_pi_characters
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5234i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_pi_xnl() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 12], &[c_char; 12]>(b"test_pi_xnl\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5239,
    );
    let mut text: *const c_char = b"<?xnl nothing like data?><doc/>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char = b"xnl: nothing like data\n\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            accumulate_pi_characters
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5249i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_pi_xmm() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 12], &[c_char; 12]>(b"test_pi_xmm\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5254,
    );
    let mut text: *const c_char =
        b"<?xmm everything like data?><doc/>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"xmm: everything like data\n\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            accumulate_pi_characters
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5264i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_utf16_pi() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 14], &[c_char; 14]>(b"test_utf16_pi\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5269,
    );
    let text: [c_char; 21] = *::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(
        b"<\x00?\x00\x04\x0e\x08\x0e?\x00>\x00<\x00q\x00/\x00>\x00\x00",
    );
    let mut expected: *const XML_Char =
        b"\xe0\xb8\x84\xe0\xb8\x88: \n\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            accumulate_pi_characters
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 21]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5290i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_utf16_be_pi() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 17], &[c_char; 17]>(b"test_utf16_be_pi\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5295,
    );
    let text: [c_char; 21] = *::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(
        b"\x00<\x00?\x0e\x04\x0e\x08\x00?\x00>\x00<\x00q\x00/\x00>\x00",
    );
    let mut expected: *const XML_Char =
        b"\xe0\xb8\x84\xe0\xb8\x88: \n\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            accumulate_pi_characters
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 21]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5316i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test that comments can be picked up and translated */

unsafe extern "C" fn accumulate_comment(mut userData: *mut c_void, mut data: *const XML_Char) {
    let mut storage: *mut crate::chardata::CharData = userData as *mut crate::chardata::CharData;
    crate::chardata::CharData_AppendXMLChars(storage, data, -(1));
}

unsafe extern "C" fn test_utf16_be_comment() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_utf16_be_comment\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5329,
    );
    let text: [c_char; 51] =
        *::std::mem::transmute::<&[u8; 51],
                                 &[c_char; 51]>(b"\x00<\x00!\x00-\x00-\x00 \x00C\x00o\x00m\x00m\x00e\x00n\x00t\x00 \x00A\x00 \x00-\x00-\x00>\x00\n\x00<\x00d\x00o\x00c\x00/\x00>\x00");
    let mut expected: *const XML_Char = b" Comment A \x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetCommentHandler(
        g_parser,
        Some(accumulate_comment as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 51]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5343i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_utf16_le_comment() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_utf16_le_comment\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5348,
    );
    let text: [c_char; 51] =
        *::std::mem::transmute::<&[u8; 51],
                                 &[c_char; 51]>(b"<\x00!\x00-\x00-\x00 \x00C\x00o\x00m\x00m\x00e\x00n\x00t\x00 \x00B\x00 \x00-\x00-\x00>\x00\n\x00<\x00d\x00o\x00c\x00/\x00>\x00\x00");
    let mut expected: *const XML_Char = b" Comment B \x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetCommentHandler(
        g_parser,
        Some(accumulate_comment as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 51]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5362i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test that the unknown encoding handler with map entries that expect
 * conversion but no conversion function is faulted
 */

unsafe extern "C" fn failing_converter(mut _data: *mut c_void, mut _s: *const c_char) -> c_int {
    /* Always claim to have failed */
    return -(1);
}

unsafe extern "C" fn prefix_converter(mut _data: *mut c_void, mut s: *const c_char) -> c_int {
    /* If the first byte is 0xff, raise an error */
    if *s.offset(0) as c_int == -1 {
        return -(1i32);
    }
    /* Just add the low bits of the first byte to the second */
    return *s.offset(1) as c_int + (*s.offset(0) as c_int & 0x7f) & 0x1ff; /* Assume a 2-byte sequence */
}

unsafe extern "C" fn MiscEncodingHandler(
    mut data: *mut c_void,
    mut encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> c_int {
    let mut i: c_int = 0;
    let mut high_map: c_int = -(2);
    if strcmp(encoding, b"invalid-9\x00".as_ptr() as *const c_char) == 0
        || strcmp(encoding, b"ascii-like\x00".as_ptr() as *const c_char) == 0
        || strcmp(encoding, b"invalid-len\x00".as_ptr() as *const c_char) == 0
        || strcmp(encoding, b"invalid-a\x00".as_ptr() as *const c_char) == 0
        || strcmp(encoding, b"invalid-surrogate\x00".as_ptr() as *const c_char) == 0
        || strcmp(encoding, b"invalid-high\x00".as_ptr() as *const c_char) == 0
    {
        high_map = -(1)
    }
    i = 0;
    while i < 128 {
        (*info).map[i as usize] = i;
        i += 1
    }
    while i < 256 {
        (*info).map[i as usize] = high_map;
        i += 1
    }
    /* If required, put an invalid value in the ASCII entries */
    if strcmp(encoding, b"invalid-9\x00".as_ptr() as *const c_char) == 0 {
        (*info).map[9] = 5
    }
    /* If required, have a top-bit set character starts a 5-byte sequence */
    if strcmp(encoding, b"invalid-len\x00".as_ptr() as *const c_char) == 0 {
        (*info).map[0x81] = -(5)
    }
    /* If required, make a top-bit set character a valid ASCII character */
    if strcmp(encoding, b"invalid-a\x00".as_ptr() as *const c_char) == 0 {
        (*info).map[0x82] = 'a' as i32
    }
    /* If required, give a top-bit set character a forbidden value,
     * what would otherwise be the first of a surrogate pair.
     */
    if strcmp(encoding, b"invalid-surrogate\x00".as_ptr() as *const c_char) == 0 {
        (*info).map[0x83] = 0xd801
    }
    /* If required, give a top-bit set character too high a value */
    if strcmp(encoding, b"invalid-high\x00".as_ptr() as *const c_char) == 0 {
        (*info).map[0x84] = 0x10101
    }
    (*info).data = data;
    (*info).release = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: *mut c_void) -> ()>,
    >(::rexpat::stddef_h::NULL as libc::intptr_t);
    if strcmp(encoding, b"failing-conv\x00".as_ptr() as *const c_char) == 0 {
        (*info).convert = Some(
            failing_converter as unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int,
        )
    } else if strcmp(encoding, b"prefix-conv\x00".as_ptr() as *const c_char) == 0 {
        (*info).convert = Some(
            prefix_converter as unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int,
        )
    } else {
        (*info).convert = ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int>,
        >(::rexpat::stddef_h::NULL as libc::intptr_t)
    }
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_missing_encoding_conversion_fn() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_missing_encoding_conversion_fn\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5435,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'no-conv\'?>\n<doc>\x81</doc>\x00".as_ptr()
            as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    /* MiscEncodingHandler sets up an encoding with every top-bit-set
     * character introducing a two-byte sequence.  For this, it
     * requires a convert function.  The above function call doesn't
     * pass one through, so when BadEncodingHandler actually gets
     * called it should supply an invalid encoding.
     */
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Encoding with missing convert() not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5447,
    );
}

unsafe extern "C" fn test_failing_encoding_conversion_fn() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_failing_encoding_conversion_fn\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5451,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'failing-conv\'?>\n<doc>\x81</doc>\x00".as_ptr()
            as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    /* BadEncodingHandler sets up an encoding with every top-bit-set
     * character introducing a two-byte sequence.  For this, it
     * requires a convert function.  The above function call passes
     * one that insists all possible sequences are invalid anyway.
     */
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Encoding with failing convert() not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5462,
    );
}
/* Test unknown encoding conversions */

unsafe extern "C" fn test_unknown_encoding_success() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_unknown_encoding_success\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5467,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'prefix-conv\'?>\n<\x81d\x80oc>Hello, world</\x81d\x80oc>\x00".as_ptr() as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _run_character_check(
        text,
        b"Hello, world\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5473,
    );
}
/* Test bad name character in unknown encoding */

unsafe extern "C" fn test_unknown_encoding_bad_name() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_unknown_encoding_bad_name\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5478,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'prefix-conv\'?>\n<\xffdoc>Hello, world</\xffdoc>\x00"
            .as_ptr() as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad name start in unknown encoding not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5484,
    );
}
/* Test bad mid-name character in unknown encoding */

unsafe extern "C" fn test_unknown_encoding_bad_name_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_unknown_encoding_bad_name_2\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5489,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'prefix-conv\'?>\n<d\xffoc>Hello, world</d\xffoc>\x00"
            .as_ptr() as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad name in unknown encoding not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5495,
    );
}
/* Test element name that is long enough to fill the conversion buffer
 * in an unknown encoding, finishing with an encoded character.
 */

unsafe extern "C" fn test_unknown_encoding_long_name_1() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 34], &[c_char; 34]>(
            b"test_unknown_encoding_long_name_1\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5502,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'prefix-conv\'?>\n<abcdefghabcdefghabcdefghijkl\x80m\x80n\x80o\x80p>Hi</abcdefghabcdefghabcdefghijkl\x80m\x80n\x80o\x80p>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"abcdefghabcdefghabcdefghijklmnop\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            record_element_start_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5516i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test element name that is long enough to fill the conversion buffer
 * in an unknown encoding, finishing with an simple character.
 */

unsafe extern "C" fn test_unknown_encoding_long_name_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 34], &[c_char; 34]>(
            b"test_unknown_encoding_long_name_2\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5524,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'prefix-conv\'?>\n<abcdefghabcdefghabcdefghijklmnop>Hi</abcdefghabcdefghabcdefghijklmnop>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"abcdefghabcdefghabcdefghijklmnop\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            record_element_start_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5538i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_invalid_unknown_encoding() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_invalid_unknown_encoding\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5543,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'invalid-9\'?>\n<doc>Hello world</doc>\x00".as_ptr()
            as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5549,
    );
}

unsafe extern "C" fn test_unknown_ascii_encoding_ok() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_unknown_ascii_encoding_ok\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5553,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'ascii-like\'?>\n<doc>Hello, world</doc>\x00".as_ptr()
            as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _run_character_check(
        text,
        b"Hello, world\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5558,
    );
}

unsafe extern "C" fn test_unknown_ascii_encoding_fail() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_unknown_ascii_encoding_fail\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5562,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'ascii-like\'?>\n<doc>Hello, \x80 world</doc>\x00"
            .as_ptr() as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5568,
    );
}

unsafe extern "C" fn test_unknown_encoding_invalid_length() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 37], &[c_char; 37]>(
            b"test_unknown_encoding_invalid_length\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5572,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'invalid-len\'?>\n<doc>Hello, world</doc>\x00".as_ptr()
            as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5578,
    );
}

unsafe extern "C" fn test_unknown_encoding_invalid_topbit() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 37], &[c_char; 37]>(
            b"test_unknown_encoding_invalid_topbit\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5582,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'invalid-a\'?>\n<doc>Hello, world</doc>\x00".as_ptr()
            as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5588,
    );
}

unsafe extern "C" fn test_unknown_encoding_invalid_surrogate() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 40], &[c_char; 40]>(
            b"test_unknown_encoding_invalid_surrogate\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5592,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'invalid-surrogate\'?>\n<doc>Hello, \x82 world</doc>\x00"
            .as_ptr() as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid unknown encoding not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5598,
    );
}

unsafe extern "C" fn test_unknown_encoding_invalid_high() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_unknown_encoding_invalid_high\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5602,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'invalid-high\'?>\n<doc>Hello, world</doc>\x00".as_ptr()
            as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5608,
    );
}

unsafe extern "C" fn test_unknown_encoding_invalid_attr_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 41], &[c_char; 41]>(
            b"test_unknown_encoding_invalid_attr_value\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5612,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'prefix-conv\'?>\n<doc attr=\'\xff0\'/>\x00".as_ptr()
            as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid attribute valid not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5618,
    );
}

unsafe extern "C" fn external_entity_loader2(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut test_data: *mut ExtTest2 = *(parser as *mut *mut c_void) as *mut ExtTest2;
    let mut extparser: XML_Parser = 0 as *mut XML_ParserStruct;
    extparser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if extparser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5647i32,
            b"Coulr not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if !(*test_data).encoding.is_null() {
        if XML_SetEncoding(extparser, (*test_data).encoding) as u64 == 0 {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                5650i32,
                b"XML_SetEncoding() ignored for external entity\x00".as_ptr() as *const c_char,
            );
        }
    }
    if (*test_data).flags & EE_PARSE_FULL_BUFFER != 0 {
        if XML_Parse(
            extparser,
            (*test_data).parse_text,
            (*test_data).parse_len,
            XML_TRUE as c_int,
        ) == XML_STATUS_ERROR_0 as c_uint
        {
            _xml_failure(
                extparser,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                5656i32,
            );
        }
    } else if _XML_Parse_SINGLE_BYTES(
        extparser,
        (*test_data).parse_text,
        (*test_data).parse_len,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            extparser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5661i32,
        );
    }
    XML_ParserFree(extparser);
    return XML_STATUS_OK_0;
}
/* Test that UTF-16 BOM does not select UTF-16 given explicit encoding */

unsafe extern "C" fn ext2_accumulate_characters(
    mut userData: *mut c_void,
    mut s: *const XML_Char,
    mut len: c_int,
) {
    let mut test_data: *mut ExtTest2 = userData as *mut ExtTest2;
    accumulate_characters((*test_data).storage as *mut c_void, s, len);
}

unsafe extern "C" fn test_ext_entity_latin1_utf16le_bom() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_ext_entity_latin1_utf16le_bom\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5675,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest2 = {
        let mut init = ExtTest2 {
            parse_text: b"\xff\xfeL \x00".as_ptr() as *const c_char,
            parse_len: 4,
            encoding: b"iso-8859-1\x00".as_ptr() as *const c_char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
            flags: EE_PARSE_NONE,
        };
        init
    };
    /* In UTF-8, y-diaeresis is 0xc3 0xbf, lowercase thorn is 0xc3 0xbe */
    let mut expected: *const XML_Char = b"\xc3\xbf\xc3\xbeL \x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    test_data.storage = &mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest2 as *mut c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5701i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_ext_entity_latin1_utf16be_bom() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_ext_entity_latin1_utf16be_bom\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5706,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest2 = {
        let mut init = ExtTest2 {
            parse_text: b"\xfe\xff L\x00".as_ptr() as *const c_char,
            parse_len: 4,
            encoding: b"iso-8859-1\x00".as_ptr() as *const c_char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
            flags: EE_PARSE_NONE,
        };
        init
    };
    /* In UTF-8, y-diaeresis is 0xc3 0xbf, lowercase thorn is 0xc3 0xbe */
    let mut expected: *const XML_Char = b"\xc3\xbe\xc3\xbf L\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    test_data.storage = &mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest2 as *mut c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5732i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Parsing the full buffer rather than a byte at a time makes a
 * difference to the encoding scanning code, so repeat the above tests
 * without breaking them down by byte.
 */

unsafe extern "C" fn test_ext_entity_latin1_utf16le_bom2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_ext_entity_latin1_utf16le_bom2\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5741,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest2 = {
        let mut init = ExtTest2 {
            parse_text: b"\xff\xfeL \x00".as_ptr() as *const c_char,
            parse_len: 4,
            encoding: b"iso-8859-1\x00".as_ptr() as *const c_char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
            flags: EE_PARSE_FULL_BUFFER,
        };
        init
    };
    /* In UTF-8, y-diaeresis is 0xc3 0xbf, lowercase thorn is 0xc3 0xbe */
    let mut expected: *const XML_Char = b"\xc3\xbf\xc3\xbeL \x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    test_data.storage = &mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest2 as *mut c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5767i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_ext_entity_latin1_utf16be_bom2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_ext_entity_latin1_utf16be_bom2\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5772,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest2 = {
        let mut init = ExtTest2 {
            parse_text: b"\xfe\xff L\x00".as_ptr() as *const c_char,
            parse_len: 4,
            encoding: b"iso-8859-1\x00".as_ptr() as *const c_char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
            flags: EE_PARSE_FULL_BUFFER,
        };
        init
    };
    /* In UTF-8, y-diaeresis is 0xc3 0xbf, lowercase thorn is 0xc3 0xbe */
    let mut expected: *const XML_Char = b"\xc3\xbe\xc3\xbf L\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    test_data.storage = &mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest2 as *mut c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if XML_Parse(g_parser, text, strlen(text) as c_int, XML_TRUE as c_int)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5798i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test little-endian UTF-16 given an explicit big-endian encoding */

unsafe extern "C" fn test_ext_entity_utf16_be() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_ext_entity_utf16_be\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5804,
    ); /* U+3E00 */
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest2 = {
        let mut init = ExtTest2 {
            parse_text: b"<\x00e\x00/\x00>\x00\x00".as_ptr() as *const c_char,
            parse_len: 8,
            encoding: b"utf-16be\x00".as_ptr() as *const c_char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
            flags: EE_PARSE_NONE,
        };
        init
    };
    let mut expected: *const XML_Char =
        b"\xe3\xb0\x80\xe6\x94\x80\xe2\xbc\x80\xe3\xb8\x80\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    test_data.storage = &mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest2 as *mut c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5828i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test big-endian UTF-16 given an explicit little-endian encoding */

unsafe extern "C" fn test_ext_entity_utf16_le() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_ext_entity_utf16_le\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5834,
    ); /* U+3E00 */
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest2 = {
        let mut init = ExtTest2 {
            parse_text: b"\x00<\x00e\x00/\x00>\x00".as_ptr() as *const c_char,
            parse_len: 8,
            encoding: b"utf-16le\x00".as_ptr() as *const c_char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
            flags: EE_PARSE_NONE,
        };
        init
    };
    let mut expected: *const XML_Char =
        b"\xe3\xb0\x80\xe6\x94\x80\xe2\xbc\x80\xe3\xb8\x80\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    test_data.storage = &mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest2 as *mut c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5858i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn external_entity_faulter2(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut test_data: *mut ExtFaults2 = *(parser as *mut *mut c_void) as *mut ExtFaults2;
    let mut extparser: XML_Parser = 0 as *mut XML_ParserStruct;
    extparser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if extparser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5890i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    if !(*test_data).encoding.is_null() {
        if XML_SetEncoding(extparser, (*test_data).encoding) as u64 == 0 {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                5893i32,
                b"XML_SetEncoding() ignored for external entity\x00".as_ptr() as *const c_char,
            );
        }
    }
    if XML_Parse(
        extparser,
        (*test_data).parse_text,
        (*test_data).parse_len,
        XML_TRUE as c_int,
    ) != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5898i32,
            (*test_data).fail_text,
        );
    }
    if XML_GetErrorCode(extparser) != (*test_data).error {
        _xml_failure(
            extparser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5900i32,
        );
    }
    XML_ParserFree(extparser);
    return XML_STATUS_ERROR_0;
}

unsafe extern "C" fn test_ext_entity_utf16_unknown() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_ext_entity_utf16_unknown\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5906,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtFaults2 = {
        let mut init = ExtFaults2 {
            parse_text: b"a\x00b\x00c\x00\x00".as_ptr() as *const c_char,
            parse_len: 6,
            fail_text: b"Invalid character in entity not faulted\x00".as_ptr() as *const c_char,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            error: XML_ERROR_INVALID_TOKEN,
        };
        init
    };
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter2
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut test_data as *mut ExtFaults2 as *mut c_void);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Invalid character should not have been accepted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5918,
    );
}
/* Test not-quite-UTF-8 BOM (0xEF 0xBB 0xBF) */

unsafe extern "C" fn test_ext_entity_utf8_non_bom() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_ext_entity_utf8_non_bom\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5923,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: ExtTest2 = {
        let mut init = ExtTest2 {
            parse_text: b"\xef\xbb\x80\x00".as_ptr() as *const c_char,
            parse_len: 3,
            encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
            storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
            flags: EE_PARSE_NONE,
        };
        init
    };
    let mut expected: *const XML_Char = b"\xef\xbb\x80\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    test_data.storage = &mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest2 as *mut c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5945i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test that UTF-8 in a CDATA section is correctly passed through */

unsafe extern "C" fn test_utf8_in_cdata_section() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_utf8_in_cdata_section\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5951,
    );
    let mut text: *const c_char =
        b"<doc><![CDATA[one \xc3\xa9 two]]></doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char = b"one \xc3\xa9 two\x00".as_ptr() as *const c_char;
    _run_character_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5959,
    );
}
/* Test that little-endian UTF-16 in a CDATA section is handled */

unsafe extern "C" fn test_utf8_in_cdata_section_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_utf8_in_cdata_section_2\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5964,
    );
    let mut text: *const c_char =
        b"<doc><![CDATA[\xc3\xa9]\xc3\xa9two]]></doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char = b"\xc3\xa9]\xc3\xa9two\x00".as_ptr() as *const c_char;
    _run_character_check(
        text,
        expected,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5972,
    );
}
/* Test trailing spaces in elements are accepted */

unsafe extern "C" fn record_element_end_handler(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
) {
    let mut storage: *mut crate::chardata::CharData = userData as *mut crate::chardata::CharData;
    crate::chardata::CharData_AppendXMLChars(storage, b"/\x00".as_ptr() as *const c_char, 1);
    crate::chardata::CharData_AppendXMLChars(storage, name, -(1));
}

unsafe extern "C" fn test_trailing_spaces_in_elements() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_trailing_spaces_in_elements\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        5985,
    );
    let mut text: *const c_char = b"<doc   >Hi</doc >\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char = b"doc/doc\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetElementHandler(
        g_parser,
        Some(
            record_element_start_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
        Some(
            record_element_end_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            5996i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_utf16_attribute() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_utf16_attribute\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6001,
    );
    let text: [c_char; 23] = *::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(
        b"<\x00d\x00 \x00\x04\x0e\x08\x0e=\x00\'\x00a\x00\'\x00/\x00>\x00\x00",
    );
    let mut expected: *const XML_Char = b"a\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 23]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6016i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_utf16_second_attr() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_utf16_second_attr\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6021,
    );
    /* <d a='1' {KHO KHWAI}{CHO CHAN}='2'/>
     * where {KHO KHWAI} = U+0E04 = 0xe0 0xb8 0x84 in UTF-8
     * and   {CHO CHAN}  = U+0E08 = 0xe0 0xb8 0x88 in UTF-8
     */
    let text: [c_char; 35] =
        *::std::mem::transmute::<&[u8; 35],
                                 &[c_char; 35]>(b"<\x00d\x00 \x00a\x00=\x00\'\x001\x00\'\x00 \x00\x04\x0e\x08\x0e=\x00\'\x002\x00\'\x00/\x00>\x00\x00");
    let mut expected: *const XML_Char = b"1\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 35]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6036i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_attr_after_solidus() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_attr_after_solidus\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6041,
    );
    let mut text: *const c_char = b"<doc attr1=\'a\' / attr2=\'b\'>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Misplaced / not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6044,
    );
}

unsafe extern "C" fn accumulate_entity_decl(
    mut userData: *mut c_void,
    mut entityName: *const XML_Char,
    mut _is_parameter_entity: c_int,
    mut value: *const XML_Char,
    mut value_length: c_int,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
    mut _notationName: *const XML_Char,
) {
    let mut storage: *mut crate::chardata::CharData = userData as *mut crate::chardata::CharData;
    crate::chardata::CharData_AppendXMLChars(storage, entityName, -(1));
    crate::chardata::CharData_AppendXMLChars(storage, b"=\x00".as_ptr() as *const c_char, 1);
    crate::chardata::CharData_AppendXMLChars(storage, value, value_length);
    crate::chardata::CharData_AppendXMLChars(storage, b"\n\x00".as_ptr() as *const c_char, 1);
}

unsafe extern "C" fn test_utf16_pe() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 14], &[c_char; 14]>(b"test_utf16_pe\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6067,
    );
    /* <!DOCTYPE doc [
     * <!ENTITY % {KHO KHWAI}{CHO CHAN} '<!ELEMENT doc (#PCDATA)>'>
     * %{KHO KHWAI}{CHO CHAN};
     * ]>
     * <doc></doc>
     *
     * where {KHO KHWAI} = U+0E04 = 0xe0 0xb8 0x84 in UTF-8
     * and   {CHO CHAN}  = U+0E08 = 0xe0 0xb8 0x88 in UTF-8
     */
    let text: [c_char; 155] =
        *::std::mem::transmute::<&[u8; 155],
                                 &[c_char; 155]>(b"\x00<\x00!\x00D\x00O\x00C\x00T\x00Y\x00P\x00E\x00 \x00d\x00o\x00c\x00 \x00[\x00\n\x00<\x00!\x00E\x00N\x00T\x00I\x00T\x00Y\x00 \x00%\x00 \x0e\x04\x0e\x08\x00 \x00\'\x00<\x00!\x00E\x00L\x00E\x00M\x00E\x00N\x00T\x00 \x00d\x00o\x00c\x00 \x00(\x00#\x00P\x00C\x00D\x00A\x00T\x00A\x00)\x00>\x00\'\x00>\x00\n\x00%\x0e\x04\x0e\x08\x00;\x00\n\x00]\x00>\x00\n\x00<\x00d\x00o\x00c\x00>\x00<\x00/\x00d\x00o\x00c\x00>\x00");
    let mut expected: *const XML_Char =
        b"\xe0\xb8\x84\xe0\xb8\x88=<!ELEMENT doc (#PCDATA)>\n\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetEntityDeclHandler(
        g_parser,
        Some(
            accumulate_entity_decl
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: c_int,
                    _: *const XML_Char,
                    _: c_int,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 155]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6097i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test that duff attribute description keywords are rejected */

unsafe extern "C" fn test_bad_attr_desc_keyword() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_bad_attr_desc_keyword\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6103,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!ATTLIST doc attr CDATA #!IMPLIED>\n]>\n<doc />\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad keyword !IMPLIED not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6110,
    );
}
/* Test that an invalid attribute description keyword consisting of
 * UTF-16 characters with their top bytes non-zero are correctly
 * faulted
 */

unsafe extern "C" fn test_bad_attr_desc_keyword_utf16() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_bad_attr_desc_keyword_utf16\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6118,
    );
    /* <!DOCTYPE d [
     * <!ATTLIST d a CDATA #{KHO KHWAI}{CHO CHAN}>
     * ]><d/>
     *
     * where {KHO KHWAI} = U+0E04 = 0xe0 0xb8 0x84 in UTF-8
     * and   {CHO CHAN}  = U+0E08 = 0xe0 0xb8 0x88 in UTF-8
     */
    let text: [c_char; 91] =
        *::std::mem::transmute::<&[u8; 91],
                                 &[c_char; 91]>(b"\x00<\x00!\x00D\x00O\x00C\x00T\x00Y\x00P\x00E\x00 \x00d\x00 \x00[\x00\n\x00<\x00!\x00A\x00T\x00T\x00L\x00I\x00S\x00T\x00 \x00d\x00 \x00a\x00 \x00C\x00D\x00A\x00T\x00A\x00 \x00#\x0e\x04\x0e\x08\x00>\x00\n\x00]\x00>\x00<\x00d\x00/\x00>\x00");
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 91]>() as c_int - 1,
        XML_TRUE,
    ) != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6134i32,
            b"Invalid UTF16 attribute keyword not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_SYNTAX {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6136i32,
        );
    };
}
/* Test that invalid syntax in a <!DOCTYPE> is rejected.  Do this
 * using prefix-encoding (see above) to trigger specific code paths
 */

unsafe extern "C" fn test_bad_doctype() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 17], &[c_char; 17]>(b"test_bad_doctype\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6143,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'prefix-conv\'?>\n<!DOCTYPE doc [ \x80D ]><doc/>\x00"
            .as_ptr() as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Invalid bytes in DOCTYPE not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6149,
    );
}

unsafe extern "C" fn test_bad_doctype_utf16() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_bad_doctype_utf16\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6153,
    );
    let text: [c_char; 53] =
        *::std::mem::transmute::<&[u8; 53],
                                 &[c_char; 53]>(b"\x00<\x00!\x00D\x00O\x00C\x00T\x00Y\x00P\x00E\x00 \x00d\x00o\x00c\x00 \x00[\x00 \x06\xf2\x00 \x00]\x00>\x00<\x00d\x00o\x00c\x00/\x00>\x00");
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 53]>() as c_int - 1,
        XML_TRUE,
    ) != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6166i32,
            b"Invalid bytes in DOCTYPE not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_SYNTAX {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6168i32,
        );
    };
}

unsafe extern "C" fn test_bad_doctype_plus() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_bad_doctype_plus\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6172,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE 1+ [ <!ENTITY foo \'bar\'> ]>\n<1+>&foo;</1+>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"\'+\' in document name not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6177,
    );
}

unsafe extern "C" fn test_bad_doctype_star() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_bad_doctype_star\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6181,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE 1* [ <!ENTITY foo \'bar\'> ]>\n<1*>&foo;</1*>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"\'*\' in document name not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6186,
    );
}

unsafe extern "C" fn test_bad_doctype_query() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_bad_doctype_query\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6190,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE 1? [ <!ENTITY foo \'bar\'> ]>\n<1?>&foo;</1?>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"\'?\' in document name not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6195,
    );
}

unsafe extern "C" fn test_unknown_encoding_bad_ignore() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_unknown_encoding_bad_ignore\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6199,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'prefix-conv\'?><!DOCTYPE doc SYSTEM \'foo\'><doc><e>&entity;</e></doc>\x00".as_ptr() as *const c_char;
    let mut fault: ExtFaults = {
        let mut init = ext_faults {
            parse_text: b"<![IGNORE[<!ELEMENT \xffG (#PCDATA)*>]]>\x00".as_ptr() as *const c_char,
            fail_text: b"Invalid character not faulted\x00".as_ptr() as *const c_char,
            encoding: b"prefix-conv\x00".as_ptr() as *const c_char,
            error: XML_ERROR_INVALID_TOKEN,
        };
        init
    };
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut fault as *mut ExtFaults as *mut c_void);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad IGNORE section with unknown encoding not failed\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6212,
    );
}

unsafe extern "C" fn test_entity_in_utf16_be_attr() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_entity_in_utf16_be_attr\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6216,
    );
    let text: [c_char; 55] =
        *::std::mem::transmute::<&[u8; 55],
                                 &[c_char; 55]>(b"\x00<\x00e\x00 \x00a\x00=\x00\'\x00&\x00#\x002\x002\x008\x00;\x00 \x00&\x00#\x00x\x000\x000\x00E\x004\x00;\x00\'\x00>\x00<\x00/\x00e\x00>\x00");
    let mut expected: *const XML_Char = b"\xc3\xa4 \xc3\xa4\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 55]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6233i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_entity_in_utf16_le_attr() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_entity_in_utf16_le_attr\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6238,
    );
    let text: [c_char; 55] =
        *::std::mem::transmute::<&[u8; 55],
                                 &[c_char; 55]>(b"<\x00e\x00 \x00a\x00=\x00\'\x00&\x00#\x002\x002\x008\x00;\x00 \x00&\x00#\x00x\x000\x000\x00E\x004\x00;\x00\'\x00>\x00<\x00/\x00e\x00>\x00\x00");
    let mut expected: *const XML_Char = b"\xc3\xa4 \xc3\xa4\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 55]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6255i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_entity_public_utf16_be() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 28], &[c_char; 28]>(b"test_entity_public_utf16_be\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6260,
    );
    let text: [c_char; 137] =
        *::std::mem::transmute::<&[u8; 137],
                                 &[c_char; 137]>(b"\x00<\x00!\x00D\x00O\x00C\x00T\x00Y\x00P\x00E\x00 \x00d\x00 \x00[\x00\n\x00<\x00!\x00E\x00N\x00T\x00I\x00T\x00Y\x00 \x00%\x00 \x00e\x00 \x00P\x00U\x00B\x00L\x00I\x00C\x00 \x00\'\x00f\x00o\x00o\x00\'\x00 \x00\'\x00b\x00a\x00r\x00.\x00e\x00n\x00t\x00\'\x00>\x00\n\x00%\x00e\x00;\x00\n\x00]\x00>\x00\n\x00<\x00d\x00>\x00&\x00j\x00;\x00<\x00/\x00d\x00>\x00");
    let mut test_data: ExtTest2 = {
        let mut init =
                ExtTest2{parse_text:
                             
                             b"\x00<\x00!\x00E\x00N\x00T\x00I\x00T\x00Y\x00 \x00j\x00 \x00\'\x00b\x00a\x00z\x00\'\x00>\x00".as_ptr() as *const c_char,
                         parse_len: 34,
                         encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                         storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
                         flags: EE_PARSE_NONE,};
        init
    };
    let mut expected: *const XML_Char = b"baz\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    test_data.storage = &mut storage;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest2 as *mut c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 137]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6287i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_entity_public_utf16_le() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 28], &[c_char; 28]>(b"test_entity_public_utf16_le\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6292,
    );
    let text: [c_char; 137] =
        *::std::mem::transmute::<&[u8; 137],
                                 &[c_char; 137]>(b"<\x00!\x00D\x00O\x00C\x00T\x00Y\x00P\x00E\x00 \x00d\x00 \x00[\x00\n\x00<\x00!\x00E\x00N\x00T\x00I\x00T\x00Y\x00 \x00%\x00 \x00e\x00 \x00P\x00U\x00B\x00L\x00I\x00C\x00 \x00\'\x00f\x00o\x00o\x00\'\x00 \x00\'\x00b\x00a\x00r\x00.\x00e\x00n\x00t\x00\'\x00>\x00\n\x00%\x00e\x00;\x00\n\x00]\x00>\x00\n\x00<\x00d\x00>\x00&\x00j\x00;\x00<\x00/\x00d\x00>\x00\x00");
    let mut test_data: ExtTest2 = {
        let mut init =
                ExtTest2{parse_text:
                             
                             b"<\x00!\x00E\x00N\x00T\x00I\x00T\x00Y\x00 \x00j\x00 \x00\'\x00b\x00a\x00z\x00\'\x00>\x00\x00".as_ptr() as *const c_char,
                         parse_len: 34,
                         encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                         storage: ::rexpat::stddef_h::NULL as *mut crate::chardata::CharData,
                         flags: EE_PARSE_NONE,};
        init
    };
    let mut expected: *const XML_Char = b"baz\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    test_data.storage = &mut storage;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, &mut test_data as *mut ExtTest2 as *mut c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 137]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6319i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}
/* Test that a doctype with neither an internal nor external subset is
 * faulted
 */

unsafe extern "C" fn test_short_doctype() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 19], &[c_char; 19]>(b"test_short_doctype\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6327,
    );
    let mut text: *const c_char = b"<!DOCTYPE doc></doc>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"DOCTYPE without subset not rejected\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6330,
    );
}

unsafe extern "C" fn test_short_doctype_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_short_doctype_2\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6334,
    );
    let mut text: *const c_char = b"<!DOCTYPE doc PUBLIC></doc>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"DOCTYPE without Public ID not rejected\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6337,
    );
}

unsafe extern "C" fn test_short_doctype_3() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_short_doctype_3\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6341,
    );
    let mut text: *const c_char = b"<!DOCTYPE doc SYSTEM></doc>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"DOCTYPE without System ID not rejected\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6344,
    );
}

unsafe extern "C" fn test_long_doctype() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_long_doctype\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6348,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc PUBLIC \'foo\' \'bar\' \'baz\'></doc>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"DOCTYPE with extra ID not rejected\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6350,
    );
}

unsafe extern "C" fn test_bad_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 16], &[c_char; 16]>(b"test_bad_entity\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6354,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY foo PUBLIC>\n]>\n<doc/>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"ENTITY without Public ID is not rejected\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6360,
    );
}
/* Test unquoted value is faulted */

unsafe extern "C" fn test_bad_entity_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_bad_entity_2\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6365,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY % foo bar>\n]>\n<doc/>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"ENTITY without Public ID is not rejected\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6371,
    );
}

unsafe extern "C" fn test_bad_entity_3() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_bad_entity_3\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6375,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY % foo PUBLIC>\n]>\n<doc/>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Parameter ENTITY without Public ID is not rejected\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6381,
    );
}

unsafe extern "C" fn test_bad_entity_4() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_bad_entity_4\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6385,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY % foo SYSTEM>\n]>\n<doc/>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Parameter ENTITY without Public ID is not rejected\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6391,
    );
}

unsafe extern "C" fn test_bad_notation() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_bad_notation\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6395,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!NOTATION n SYSTEM>\n]>\n<doc/>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Notation without System ID is not rejected\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6401,
    );
}

unsafe extern "C" fn checking_default_handler(
    mut userData: *mut c_void,
    mut s: *const XML_Char,
    mut len: c_int,
) {
    let mut data: *mut DefaultCheck = userData as *mut DefaultCheck;
    let mut i: c_int = 0;
    i = 0;
    while !(*data.offset(i as isize)).expected.is_null() {
        if (*data.offset(i as isize)).expectedLen == len
            && memcmp(
                (*data.offset(i as isize)).expected as *const c_void,
                s as *const c_void,
                (len as c_ulong).wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
            ) == 0
        {
            (*data.offset(i as isize)).seen = XML_TRUE;
            break;
        } else {
            i += 1
        }
    }
}

unsafe extern "C" fn test_default_doctype_handler() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_default_doctype_handler\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6426,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc PUBLIC \'pubname\' \'test.dtd\' [\n  <!ENTITY foo \'bar\'>\n]>\n<doc>&foo;</doc>\x00".as_ptr() as *const c_char;
    let mut test_data: [DefaultCheck; 3] = [
        {
            let mut init = default_check {
                expected: b"\'pubname\'\x00".as_ptr() as *const c_char,
                expectedLen: 9,
                seen: XML_FALSE,
            };
            init
        },
        {
            let mut init = default_check {
                expected: b"\'test.dtd\'\x00".as_ptr() as *const c_char,
                expectedLen: 10,
                seen: XML_FALSE,
            };
            init
        },
        {
            let mut init = default_check {
                expected: ::rexpat::stddef_h::NULL as *const XML_Char,
                expectedLen: 0,
                seen: XML_FALSE,
            };
            init
        },
    ];
    let mut i: c_int = 0;
    XML_SetUserData(
        g_parser,
        &mut test_data as *mut [DefaultCheck; 3] as *mut c_void,
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            checking_default_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    XML_SetEntityDeclHandler(
        g_parser,
        Some(
            dummy_entity_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: c_int,
                    _: *const XML_Char,
                    _: c_int,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6441i32,
        );
    }
    i = 0;
    while !test_data[i as usize].expected.is_null() {
        if test_data[i as usize].seen == 0 {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                6444i32,
                b"Default handler not run for public !DOCTYPE\x00".as_ptr() as *const c_char,
            );
        }
        i += 1
    }
}

unsafe extern "C" fn test_empty_element_abort() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_empty_element_abort\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6448,
    );
    let mut text: *const c_char = b"<abort/>\x00".as_ptr() as *const c_char;
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_suspender
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6454i32,
            b"Expected to error on abort\x00".as_ptr() as *const c_char,
        );
    };
}
/*
 * Namespaces tests.
 */

unsafe extern "C" fn namespace_setup() {
    g_parser = XML_ParserCreateNS(
        ::rexpat::stddef_h::NULL as *const XML_Char,
        ' ' as XML_Char,
    );
    if g_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6466i32,
            b"Parser not created.\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn namespace_teardown() {
    basic_teardown();
}
/* Check that an element name and attribute name match the expected values.
   The expected values are passed as an array reference of string pointers
   provided as the userData argument; the first is the expected
   element name, and the second is the expected attribute name.
*/

static mut triplet_start_flag: XML_Bool = XML_FALSE;

static mut triplet_end_flag: XML_Bool = XML_FALSE;

unsafe extern "C" fn triplet_start_checker(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut elemstr: *mut *mut XML_Char = userData as *mut *mut XML_Char;
    let mut buffer: [c_char; 1024] = [0; 1024];
    if strcmp(*elemstr.offset(0), name) != 0 {
        sprintf(
            buffer.as_mut_ptr(),
            b"unexpected start string: \'%s\'\x00".as_ptr() as *const c_char,
            name,
        );
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6489i32,
            buffer.as_mut_ptr(),
        );
    }
    if strcmp(*elemstr.offset(1), *atts.offset(0)) != 0 {
        sprintf(
            buffer.as_mut_ptr(),
            b"unexpected attribute string: \'%s\'\x00".as_ptr() as *const c_char,
            *atts.offset(0isize),
        );
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6493i32,
            buffer.as_mut_ptr(),
        );
    }
    triplet_start_flag = XML_TRUE;
}
/* Check that the element name passed to the end-element handler matches
   the expected value.  The expected value is passed as the first element
   in an array of strings passed as the userData argument.
*/

unsafe extern "C" fn triplet_end_checker(mut userData: *mut c_void, mut name: *const XML_Char) {
    let mut elemstr: *mut *mut XML_Char = userData as *mut *mut XML_Char;
    if strcmp(*elemstr.offset(0), name) != 0 {
        let mut buffer: [c_char; 1024] = [0; 1024];
        sprintf(
            buffer.as_mut_ptr(),
            b"unexpected end string: \'%s\'\x00".as_ptr() as *const c_char,
            name,
        );
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6508i32,
            buffer.as_mut_ptr(),
        );
    }
    triplet_end_flag = XML_TRUE;
}

unsafe extern "C" fn test_return_ns_triplet() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_return_ns_triplet\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6513,
    );
    let mut text: *const c_char =
        
        b"<foo:e xmlns:foo=\'http://example.org/\' bar:a=\'12\'\n       xmlns:bar=\'http://example.org/\'>\x00".as_ptr() as *const c_char;
    let mut epilog: *const c_char = b"</foo:e>\x00".as_ptr() as *const c_char;
    let mut elemstr: [*const XML_Char; 2] = [
        b"http://example.org/ e foo\x00".as_ptr() as *const c_char,
        b"http://example.org/ a bar\x00".as_ptr() as *const c_char,
    ];
    XML_SetReturnNSTriplet(g_parser, XML_TRUE);
    XML_SetUserData(g_parser, elemstr.as_mut_ptr() as *mut c_void);
    XML_SetElementHandler(
        g_parser,
        Some(
            triplet_start_checker
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
        Some(triplet_end_checker as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()),
    );
    XML_SetNamespaceDeclHandler(
        g_parser,
        Some(
            dummy_start_namespace_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
        Some(
            dummy_end_namespace_decl_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    triplet_start_flag = XML_FALSE;
    triplet_end_flag = XML_FALSE;
    dummy_handler_flags = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6529i32,
        );
    }
    if triplet_start_flag == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6531i32,
            b"triplet_start_checker not invoked\x00".as_ptr() as *const c_char,
        );
    }
    /* Check that unsetting "return triplets" fails while still parsing */
    XML_SetReturnNSTriplet(g_parser, XML_FALSE);
    if _XML_Parse_SINGLE_BYTES(g_parser, epilog, strlen(epilog) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6536i32,
        );
    }
    if triplet_end_flag == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6538i32,
            b"triplet_end_checker not invoked\x00".as_ptr() as *const c_char,
        );
    }
    if dummy_handler_flags != DUMMY_START_NS_DECL_HANDLER_FLAG | DUMMY_END_NS_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6541i32,
            b"Namespace handlers not called\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn overwrite_start_checker(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut storage: *mut crate::chardata::CharData = userData as *mut crate::chardata::CharData;
    crate::chardata::CharData_AppendXMLChars(storage, b"start \x00".as_ptr() as *const c_char, 6);
    crate::chardata::CharData_AppendXMLChars(storage, name, -(1));
    while !(*atts).is_null() {
        crate::chardata::CharData_AppendXMLChars(
            storage,
            b"\nattribute \x00".as_ptr() as *const c_char,
            11,
        );
        crate::chardata::CharData_AppendXMLChars(storage, *atts, -(1));
        atts = atts.offset(2)
    }
    crate::chardata::CharData_AppendXMLChars(storage, b"\n\x00".as_ptr() as *const c_char, 1);
}

unsafe extern "C" fn overwrite_end_checker(mut userData: *mut c_void, mut name: *const XML_Char) {
    let mut storage: *mut crate::chardata::CharData = userData as *mut crate::chardata::CharData;
    crate::chardata::CharData_AppendXMLChars(storage, b"end \x00".as_ptr() as *const c_char, 4);
    crate::chardata::CharData_AppendXMLChars(storage, name, -(1));
    crate::chardata::CharData_AppendXMLChars(storage, b"\n\x00".as_ptr() as *const c_char, 1);
}

unsafe extern "C" fn run_ns_tagname_overwrite_test(
    mut text: *const c_char,
    mut result: *const XML_Char,
) {
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetElementHandler(
        g_parser,
        Some(
            overwrite_start_checker
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
        Some(
            overwrite_end_checker as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6576i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, result);
}
/* Regression test for SF bug #566334. */

unsafe extern "C" fn test_ns_tagname_overwrite() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_ns_tagname_overwrite\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6581,
    );
    let mut text: *const c_char =
        
        b"<n:e xmlns:n=\'http://example.org/\'>\n  <n:f n:attr=\'foo\'/>\n  <n:g n:attr2=\'bar\'/>\n</n:e>\x00".as_ptr() as *const c_char;
    let mut result: *const XML_Char =
        
        b"start http://example.org/ e\nstart http://example.org/ f\nattribute http://example.org/ attr\nend http://example.org/ f\nstart http://example.org/ g\nattribute http://example.org/ attr2\nend http://example.org/ g\nend http://example.org/ e\n\x00".as_ptr() as *const c_char;
    run_ns_tagname_overwrite_test(text, result);
}
/* Regression test for SF bug #566334. */

unsafe extern "C" fn test_ns_tagname_overwrite_triplet() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 34], &[c_char; 34]>(
            b"test_ns_tagname_overwrite_triplet\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6599,
    );
    let mut text: *const c_char =
        
        b"<n:e xmlns:n=\'http://example.org/\'>\n  <n:f n:attr=\'foo\'/>\n  <n:g n:attr2=\'bar\'/>\n</n:e>\x00".as_ptr() as *const c_char;
    let mut result: *const XML_Char =
        
        b"start http://example.org/ e n\nstart http://example.org/ f n\nattribute http://example.org/ attr n\nend http://example.org/ f n\nstart http://example.org/ g n\nattribute http://example.org/ attr2 n\nend http://example.org/ g n\nend http://example.org/ e n\n\x00".as_ptr() as *const c_char;
    XML_SetReturnNSTriplet(g_parser, XML_TRUE);
    run_ns_tagname_overwrite_test(text, result);
}
/* Regression test for SF bug #620343. */

unsafe extern "C" fn start_element_fail(
    mut _userData: *mut c_void,
    mut _name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
    /* We should never get here. */
    crate::minicheck::_fail_unless(
        0,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6626,
        b"should never reach start_element_fail()\x00".as_ptr() as *const c_char,
    );
}

unsafe extern "C" fn start_ns_clearing_start_element(
    mut userData: *mut c_void,
    mut _prefix: *const XML_Char,
    mut _uri: *const XML_Char,
) {
    XML_SetStartElementHandler(
        userData as XML_Parser,
        ::std::mem::transmute::<libc::intptr_t, XML_StartElementHandler>(
            ::rexpat::stddef_h::NULL as libc::intptr_t,
        ),
    );
}

unsafe extern "C" fn test_start_ns_clears_start_element() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_start_ns_clears_start_element\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6637,
    );
    /* This needs to use separate start/end tags; using the empty tag
       syntax doesn't cause the problematic path through Expat to be
       taken.
    */
    let mut text: *const c_char =
        b"<e xmlns=\'http://example.org/\'></e>\x00".as_ptr() as *const c_char;
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_fail
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetStartNamespaceDeclHandler(
        g_parser,
        Some(
            start_ns_clearing_start_element
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndNamespaceDeclHandler(
        g_parser,
        Some(
            dummy_end_namespace_decl_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    XML_UseParserAsHandlerArg(g_parser);
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6650i32,
        );
    };
}
/* Regression test for SF bug #616863. */

unsafe extern "C" fn external_entity_handler(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut callno: intptr_t = 1 + *(parser as *mut *mut c_void) as intptr_t;
    let mut text: *const c_char = 0 as *const c_char;
    let mut p2: XML_Parser = 0 as *mut XML_ParserStruct;
    if callno == 1 {
        text = b"<!ELEMENT doc (e+)>\n<!ATTLIST doc xmlns CDATA #IMPLIED>\n<!ELEMENT e EMPTY>\n\x00"
            .as_ptr() as *const c_char
    } else {
        text = b"<?xml version=\'1.0\' encoding=\'us-ascii\'?><e/>\x00".as_ptr() as *const c_char
    }
    XML_SetUserData(parser, callno as *mut c_void);
    p2 = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if _XML_Parse_SINGLE_BYTES(p2, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            p2,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6678,
        );
        return XML_STATUS_ERROR_0;
    }
    XML_ParserFree(p2);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_default_ns_from_ext_subset_and_ext_ge() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 43], &[c_char; 43]>(
            b"test_default_ns_from_ext_subset_and_ext_ge\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6685,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\'?>\n<!DOCTYPE doc SYSTEM \'http://example.org/doc.dtd\' [\n  <!ENTITY en SYSTEM \'http://example.org/entity.ent\'>\n]>\n<doc xmlns=\'http://example.org/ns1\'>\n&en;\n</doc>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_handler
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    /* We actually need to set this handler to tickle this bug. */
    XML_SetStartElementHandler(
        g_parser,
        Some(
            dummy_start_element
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, ::rexpat::stddef_h::NULL as *mut c_void);
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6701i32,
        );
    };
}
/* Regression test #1 for SF bug #673791. */

unsafe extern "C" fn test_ns_prefix_with_empty_uri_1() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_ns_prefix_with_empty_uri_1\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6706,
    );
    let mut text: *const c_char =
        b"<doc xmlns:prefix=\'http://example.org/\'>\n  <e xmlns:prefix=\'\'/>\n</doc>\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_UNDECLARING_PREFIX,
        b"Did not report re-setting namespace URI with prefix to \'\'.\x00".as_ptr()
            as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6713,
    );
}
/* Regression test #2 for SF bug #673791. */

unsafe extern "C" fn test_ns_prefix_with_empty_uri_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_ns_prefix_with_empty_uri_2\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6718,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\'?>\n<docelem xmlns:pre=\'\'/>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_UNDECLARING_PREFIX,
        b"Did not report setting namespace URI with prefix to \'\'.\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6723,
    );
}
/* Regression test #3 for SF bug #673791. */

unsafe extern "C" fn test_ns_prefix_with_empty_uri_3() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_ns_prefix_with_empty_uri_3\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6728,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ELEMENT doc EMPTY>\n  <!ATTLIST doc\n    xmlns:prefix CDATA \'\'>\n]>\n<doc/>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_UNDECLARING_PREFIX,
        b"Didn\'t report attr default setting NS w/ prefix to \'\'.\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6737,
    );
}
/* Regression test #4 for SF bug #673791. */

unsafe extern "C" fn test_ns_prefix_with_empty_uri_4() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_ns_prefix_with_empty_uri_4\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6742,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ELEMENT prefix:doc EMPTY>\n  <!ATTLIST prefix:doc\n    xmlns:prefix CDATA \'http://example.org/\'>\n]>\n<prefix:doc/>\x00".as_ptr() as *const c_char;
    /* Packaged info expected by the end element handler;
    the weird structuring lets us re-use the triplet_end_checker()
    function also used for another test. */
    let mut elemstr: [*const XML_Char; 1] =
        [b"http://example.org/ doc prefix\x00".as_ptr() as *const c_char];
    XML_SetReturnNSTriplet(g_parser, XML_TRUE);
    XML_SetUserData(g_parser, elemstr.as_mut_ptr() as *mut c_void);
    XML_SetEndElementHandler(
        g_parser,
        Some(triplet_end_checker as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6758i32,
        );
    };
}
/* Test with non-xmlns prefix */

unsafe extern "C" fn test_ns_unbound_prefix() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_ns_unbound_prefix\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6763,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ELEMENT prefix:doc EMPTY>\n  <!ATTLIST prefix:doc\n    notxmlns:prefix CDATA \'http://example.org/\'>\n]>\n<prefix:doc/>\x00".as_ptr() as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6773i32,
            b"Unbound prefix incorrectly passed\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_UNBOUND_PREFIX {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6775i32,
        );
    };
}

unsafe extern "C" fn test_ns_default_with_empty_uri() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_ns_default_with_empty_uri\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6779,
    );
    let mut text: *const c_char =
        b"<doc xmlns=\'http://example.org/\'>\n  <e xmlns=\'\'/>\n</doc>\x00".as_ptr()
            as *const c_char;
    /* Add some handlers to exercise extra code paths */
    XML_SetStartNamespaceDeclHandler(
        g_parser,
        Some(
            dummy_start_namespace_decl_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndNamespaceDeclHandler(
        g_parser,
        Some(
            dummy_end_namespace_decl_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6789i32,
        );
    };
}
/* Regression test for SF bug #692964: two prefixes for one namespace. */

unsafe extern "C" fn test_ns_duplicate_attrs_diff_prefixes() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 38], &[c_char; 38]>(
            b"test_ns_duplicate_attrs_diff_prefixes\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6794,
    );
    let mut text: *const c_char =
        
        b"<doc xmlns:a=\'http://example.org/a\'\n     xmlns:b=\'http://example.org/a\'\n     a:a=\'v\' b:a=\'v\' />\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_DUPLICATE_ATTRIBUTE,
        b"did not report multiple attributes with same URI+name\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6799,
    );
}

unsafe extern "C" fn test_ns_duplicate_hashes() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_ns_duplicate_hashes\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6803,
    );
    /* The hash of an attribute is calculated as the hash of its URI
     * concatenated with a space followed by its name (after the
     * colon).  We wish to generate attributes with the same hash
     * value modulo the attribute table size so that we can check that
     * the attribute hash table works correctly.  The attribute hash
     * table size will be the smallest power of two greater than the
     * number of attributes, but at least eight.  There is
     * unfortunately no programmatic way of getting the hash or the
     * table size at user level, but the test code coverage percentage
     * will drop if the hashes cease to point to the same row.
     *
     * The cunning plan is to have few enough attributes to have a
     * reliable table size of 8, and have the single letter attribute
     * names be 8 characters apart, producing a hash which will be the
     * same modulo 8.
     */
    let mut text: *const c_char =
        b"<doc xmlns:a=\'http://example.org/a\'\n     a:a=\'v\' a:i=\'w\' />\x00".as_ptr()
            as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6824i32,
        );
    };
}
/* Regression test for SF bug #695401: unbound prefix. */

unsafe extern "C" fn test_ns_unbound_prefix_on_attribute() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_ns_unbound_prefix_on_attribute\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6829,
    );
    let mut text: *const c_char = b"<doc a:attr=\'\'/>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_UNBOUND_PREFIX,
        b"did not report unbound prefix on attribute\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6832,
    );
}
/* Regression test for SF bug #695401: unbound prefix. */

unsafe extern "C" fn test_ns_unbound_prefix_on_element() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 34], &[c_char; 34]>(
            b"test_ns_unbound_prefix_on_element\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6837,
    );
    let mut text: *const c_char = b"<a:doc/>\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_UNBOUND_PREFIX,
        b"did not report unbound prefix on element\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6840,
    );
}
/* Test that the parsing status is correctly reset by XML_ParserReset().
 * We usE test_return_ns_triplet() for our example parse to improve
 * coverage of tidying up code executed.
 */

unsafe extern "C" fn test_ns_parser_reset() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_ns_parser_reset\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6848,
    );
    let mut status: XML_ParsingStatus = XML_ParsingStatus {
        parsing: XML_INITIALIZED,
        finalBuffer: 0,
    };
    XML_GetParsingStatus(g_parser, &mut status as *mut _);
    if status.parsing != XML_INITIALIZED {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6853i32,
            b"parsing status doesn\'t start INITIALIZED\x00".as_ptr() as *const c_char,
        );
    }
    test_return_ns_triplet();
    XML_GetParsingStatus(g_parser, &mut status as *mut _);
    if status.parsing != XML_FINISHED {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6857i32,
            b"parsing status doesn\'t end FINISHED\x00".as_ptr() as *const c_char,
        );
    }
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    XML_GetParsingStatus(g_parser, &mut status as *mut _);
    if status.parsing != XML_INITIALIZED {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6861i32,
            b"parsing status doesn\'t reset to INITIALIZED\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test that long element names with namespaces are handled correctly */

unsafe extern "C" fn test_ns_long_element() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_ns_long_element\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6866,
    );
    let mut text: *const c_char =
        
        b"<foo:thisisalongenoughelementnametotriggerareallocation\n xmlns:foo=\'http://example.org/\' bar:a=\'12\'\n xmlns:bar=\'http://example.org/\'></foo:thisisalongenoughelementnametotriggerareallocation>\x00".as_ptr() as *const c_char;
    let mut elemstr: [*const XML_Char; 2] = [
        b"http://example.org/ thisisalongenoughelementnametotriggerareallocation foo\x00".as_ptr()
            as *const c_char,
        b"http://example.org/ a bar\x00".as_ptr() as *const c_char,
    ];
    XML_SetReturnNSTriplet(g_parser, XML_TRUE);
    XML_SetUserData(g_parser, elemstr.as_mut_ptr() as *mut c_void);
    XML_SetElementHandler(
        g_parser,
        Some(
            triplet_start_checker
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
        Some(triplet_end_checker as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6882i32,
        );
    };
}
/* Test mixed population of prefixed and unprefixed attributes */

unsafe extern "C" fn test_ns_mixed_prefix_atts() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_ns_mixed_prefix_atts\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6887,
    );
    let mut text: *const c_char =
        b"<e a=\'12\' bar:b=\'13\'\n xmlns:bar=\'http://example.org/\'></e>\x00".as_ptr()
            as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6894i32,
        );
    };
}
/* Test having a long namespaced element name inside a short one.
 * This exercises some internal buffer reallocation that is shared
 * across elements with the same namespace URI.
 */

unsafe extern "C" fn test_ns_extend_uri_buffer() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_ns_extend_uri_buffer\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6902,
    );
    let mut text: *const c_char =
        
        b"<foo:e xmlns:foo=\'http://example.org/\'> <foo:thisisalongenoughnametotriggerallocationaction   foo:a=\'12\' /></foo:e>\x00".as_ptr() as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6909i32,
        );
    };
}
/* Test that xmlns is correctly rejected as an attribute in the xmlns
 * namespace, but not in other namespaces
 */

unsafe extern "C" fn test_ns_reserved_attributes() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 28], &[c_char; 28]>(b"test_ns_reserved_attributes\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6916,
    );
    let mut text1: *const c_char =
        b"<foo:e xmlns:foo=\'http://example.org/\' xmlns:xmlns=\'12\' />\x00".as_ptr()
            as *const c_char;
    let mut text2: *const c_char =
        b"<foo:e xmlns:foo=\'http://example.org/\' foo:xmlns=\'12\' />\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text1,
        XML_ERROR_RESERVED_PREFIX_XMLNS,
        b"xmlns not rejected as an attribute\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6922,
    );
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    if _XML_Parse_SINGLE_BYTES(g_parser, text2, strlen(text2) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            6926i32,
        );
    };
}
/* Test more reserved attributes */

unsafe extern "C" fn test_ns_reserved_attributes_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_ns_reserved_attributes_2\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6931,
    );
    let mut text1: *const c_char =
        b"<foo:e xmlns:foo=\'http://example.org/\'  xmlns:xml=\'http://example.org/\' />\x00"
            .as_ptr() as *const c_char;
    let mut text2: *const c_char =
        b"<foo:e xmlns:foo=\'http://www.w3.org/XML/1998/namespace\' />\x00".as_ptr()
            as *const c_char;
    let mut text3: *const c_char =
        b"<foo:e xmlns:foo=\'http://www.w3.org/2000/xmlns/\' />\x00".as_ptr() as *const c_char;
    _expect_failure(
        text1,
        XML_ERROR_RESERVED_PREFIX_XML,
        b"xml not rejected as an attribute\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6939,
    );
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    _expect_failure(
        text2,
        XML_ERROR_RESERVED_NAMESPACE_URI,
        b"Use of w3.org URL not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6942,
    );
    XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
    _expect_failure(
        text3,
        XML_ERROR_RESERVED_NAMESPACE_URI,
        b"Use of w3.org xmlns URL not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6945,
    );
}
/* Test string pool handling of namespace names of 2048 characters */
/* Exercises a particular string pool growth path */

unsafe extern "C" fn test_ns_extremely_long_prefix() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_ns_extremely_long_prefix\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        6951,
    );
    /* C99 compilers are only required to support 4095-character
     * strings, so the following needs to be split in two to be safe
     * for all compilers.
     */
    let mut text1: *const c_char =
        
        b"<doc ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:a=\'12\'\x00".as_ptr() as *const c_char;
    let mut text2: *const c_char =
        
        b" xmlns:ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP=\'foo\'\n></doc>\x00".as_ptr() as *const c_char;
    if _XML_Parse_SINGLE_BYTES(g_parser, text1, strlen(text1) as c_int, XML_FALSE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7032i32,
        );
    }
    if _XML_Parse_SINGLE_BYTES(g_parser, text2, strlen(text2) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7035i32,
        );
    };
}
/* Test unknown encoding handlers in namespace setup */

unsafe extern "C" fn test_ns_unknown_encoding_success() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_ns_unknown_encoding_success\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7040,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'prefix-conv\'?>\n<foo:e xmlns:foo=\'http://example.org/\'>Hi</foo:e>\x00".as_ptr() as *const c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    _run_character_check(
        text,
        b"Hi\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7045,
    );
}
/* Test that too many colons are rejected */

unsafe extern "C" fn test_ns_double_colon() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_ns_double_colon\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7050,
    );
    let mut text: *const c_char = b"<foo:e xmlns:foo=\'http://example.org/\' foo:a:b=\'bar\' />\x00"
        .as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Double colon in attribute name not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7054,
    );
}

unsafe extern "C" fn test_ns_double_colon_element() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_ns_double_colon_element\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7058,
    );
    let mut text: *const c_char =
        b"<foo:bar:e xmlns:foo=\'http://example.org/\' />\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Double colon in element name not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7062,
    );
}
/* Test that non-name characters after a colon are rejected */

unsafe extern "C" fn test_ns_bad_attr_leafname() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_ns_bad_attr_leafname\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7067,
    );
    let mut text: *const c_char = b"<foo:e xmlns:foo=\'http://example.org/\' foo:?ar=\'baz\' />\x00"
        .as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character in leafname not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7071,
    );
}

unsafe extern "C" fn test_ns_bad_element_leafname() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_ns_bad_element_leafname\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7075,
    );
    let mut text: *const c_char =
        b"<foo:?oc xmlns:foo=\'http://example.org/\' />\x00".as_ptr() as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character in element leafname not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7079,
    );
}
/* Test high-byte-set UTF-16 characters are valid in a leafname */

unsafe extern "C" fn test_ns_utf16_leafname() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_ns_utf16_leafname\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7084,
    );
    let text: [c_char; 59] =
        *::std::mem::transmute::<&[u8; 59],
                                 &[c_char; 59]>(b"<\x00n\x00:\x00e\x00 \x00x\x00m\x00l\x00n\x00s\x00:\x00n\x00=\x00\'\x00U\x00R\x00I\x00\'\x00 \x00n\x00:\x00\x04\x0e=\x00\'\x00a\x00\'\x00 \x00/\x00>\x00\x00");
    let mut expected: *const XML_Char = b"a\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 59]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7099i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_ns_utf16_element_leafname() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_ns_utf16_element_leafname\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7104,
    );
    let text: [c_char; 41] =
        *::std::mem::transmute::<&[u8; 41],
                                 &[c_char; 41]>(b"\x00<\x00n\x00:\x0e\x04\x00 \x00x\x00m\x00l\x00n\x00s\x00:\x00n\x00=\x00\'\x00U\x00R\x00I\x00\'\x00/\x00>\x00");
    let mut expected: *const XML_Char = b"URI \xe0\xb8\x84\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_event_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 41]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7122i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_ns_utf16_doctype() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_ns_utf16_doctype\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7127,
    );
    let text: [c_char; 155] =
        *::std::mem::transmute::<&[u8; 155],
                                 &[c_char; 155]>(b"\x00<\x00!\x00D\x00O\x00C\x00T\x00Y\x00P\x00E\x00 \x00f\x00o\x00o\x00:\x0e\x04\x00 \x00[\x00 \x00<\x00!\x00E\x00N\x00T\x00I\x00T\x00Y\x00 \x00b\x00a\x00r\x00 \x00\'\x00b\x00a\x00z\x00\'\x00>\x00 \x00]\x00>\x00\n\x00<\x00f\x00o\x00o\x00:\x0e\x04\x00 \x00x\x00m\x00l\x00n\x00s\x00:\x00f\x00o\x00o\x00=\x00\'\x00U\x00R\x00I\x00\'\x00>\x00&\x00b\x00a\x00r\x00;\x00<\x00/\x00f\x00o\x00o\x00:\x0e\x04\x00>\x00");
    let mut expected: *const XML_Char = b"URI \xe0\xb8\x84\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_event_handler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUnknownEncodingHandler(
        g_parser,
        transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut XML_Encoding,
                ) -> c_int,
        )),
        ::rexpat::stddef_h::NULL as *mut c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 155]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7153i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn test_ns_invalid_doctype() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_ns_invalid_doctype\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7158,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE foo:!bad [ <!ENTITY bar \'baz\' ]>\n<foo:!bad>&bar;</foo:!bad>\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character in document local name not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7163,
    );
}

unsafe extern "C" fn test_ns_double_colon_doctype() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_ns_double_colon_doctype\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7167,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE foo:a:doc [ <!ENTITY bar \'baz\' ]>\n<foo:a:doc>&bar;</foo:a:doc>\x00".as_ptr()
            as *const c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Double colon in document name not faulted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7172,
    );
}
/* Control variable; the number of times duff_allocator() will successfully
 * allocate */

pub const ALLOC_ALWAYS_SUCCEED: c_int = -(1);

pub const REALLOC_ALWAYS_SUCCEED: c_int = -(1);

static mut allocation_count: intptr_t = ALLOC_ALWAYS_SUCCEED as intptr_t;

static mut reallocation_count: intptr_t = REALLOC_ALWAYS_SUCCEED as intptr_t;
/* Crocked allocator for allocation failure tests */

unsafe extern "C" fn duff_allocator(mut size: size_t) -> *mut c_void {
    if allocation_count == 0 {
        return ::rexpat::stddef_h::NULL as *mut c_void;
    }
    if allocation_count != ALLOC_ALWAYS_SUCCEED as c_long {
        allocation_count -= 1
    }
    return malloc(size);
}
/* Crocked reallocator for allocation failure tests */

unsafe extern "C" fn duff_reallocator(mut ptr: *mut c_void, mut size: size_t) -> *mut c_void {
    if reallocation_count == 0 {
        return ::rexpat::stddef_h::NULL as *mut c_void;
    }
    if reallocation_count != REALLOC_ALWAYS_SUCCEED as c_long {
        reallocation_count -= 1
    }
    return realloc(ptr, size);
}
/* Test that a failure to allocate the parser structure fails gracefully */

unsafe extern "C" fn test_misc_alloc_create_parser() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_misc_alloc_create_parser\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7205,
    );
    ALLOCATOR_MODE = AllocatorMode::Duff;
    let mut i: c_uint = 0;
    let max_alloc_count: c_uint = 10;
    /* Something this simple shouldn't need more than 10 allocations */
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        g_parser = XML_ParserCreate_MM(
            ::rexpat::stddef_h::NULL as *const XML_Char,
            None,
            ::rexpat::stddef_h::NULL as *const XML_Char,
        );
        if !g_parser.is_null() {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7218i32,
            b"Parser unexpectedly ignored failing allocator\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7220i32,
            b"Parser not created with max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test memory allocation failures for a parser with an encoding */

unsafe extern "C" fn test_misc_alloc_create_parser_with_encoding() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 44], &[c_char; 44]>(
            b"test_misc_alloc_create_parser_with_encoding\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7225,
    );
    ALLOCATOR_MODE = AllocatorMode::Duff;
    let mut i: c_uint = 0;
    let max_alloc_count: c_uint = 10;
    /* Try several levels of allocation */
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        g_parser = XML_ParserCreate_MM(
            b"us-ascii\x00".as_ptr() as *const c_char,
            None,
            ::rexpat::stddef_h::NULL as *const XML_Char,
        );
        if !g_parser.is_null() {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7238i32,
            b"Parser ignored failing allocator\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7240i32,
            b"Parser not created with max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test that freeing a NULL parser doesn't cause an explosion.
 * (Not actually tested anywhere else)
 */

unsafe extern "C" fn test_misc_null_parser() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_misc_null_parser\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7247,
    );
    XML_ParserFree(::rexpat::stddef_h::NULL as XML_Parser);
}
/* Test that XML_ErrorString rejects out-of-range codes */

unsafe extern "C" fn test_misc_error_string() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_misc_error_string\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7253,
    );
    if !XML_ErrorString(4294967295).is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7255i32,
            b"Negative error code not rejected\x00".as_ptr() as *const c_char,
        );
    }
    if !XML_ErrorString(100).is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7257i32,
            b"Large error code not rejected\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test the version information is consistent */
/* Since we are working in XML_LChars (potentially 16-bits), we
 * can't use the standard C library functions for character
 * manipulation and have to roll our own.
 */

unsafe extern "C" fn parse_version(
    mut version_text: *const XML_LChar,
    mut version_struct: *mut XML_Expat_Version,
) -> XML_Bool {
    if version_text.is_null() {
        return XML_FALSE;
    }
    while *version_text as c_int != 0 {
        if *version_text >= ASCII_0 && *version_text <= ASCII_9 {
            break;
        }
        version_text = version_text.offset(1)
    }
    if *version_text as c_int == 0 {
        return XML_FALSE;
    }
    /* version_struct->major = strtoul(version_text, 10, &version_text) */
    (*version_struct).major = 0;
    while *version_text >= ASCII_0 && *version_text <= ASCII_9 {
        let fresh2 = version_text;
        version_text = version_text.offset(1);
        (*version_struct).major =
            10 * (*version_struct).major + (*fresh2 as c_int - ASCII_0 as c_int)
    }
    let fresh3 = version_text;
    version_text = version_text.offset(1);
    if *fresh3 != ASCII_PERIOD {
        return XML_FALSE;
    }
    /* Now for the minor version number */
    (*version_struct).minor = 0;
    while *version_text >= ASCII_0 && *version_text <= ASCII_9 {
        let fresh4 = version_text;
        version_text = version_text.offset(1);
        (*version_struct).minor =
            10 * (*version_struct).minor + (*fresh4 as c_int - ASCII_0 as c_int)
    }
    let fresh5 = version_text;
    version_text = version_text.offset(1);
    if *fresh5 != ASCII_PERIOD {
        return XML_FALSE;
    }
    /* Finally the micro version number */
    (*version_struct).micro = 0;
    while *version_text >= ASCII_0 && *version_text <= ASCII_9 {
        let fresh6 = version_text;
        version_text = version_text.offset(1);
        (*version_struct).micro =
            10 * (*version_struct).micro + (*fresh6 as c_int - ASCII_0 as c_int)
    }
    if *version_text != 0 {
        return XML_FALSE;
    }
    return XML_TRUE;
}

unsafe extern "C" fn versions_equal(
    mut first: *const XML_Expat_Version,
    mut second: *const XML_Expat_Version,
) -> c_int {
    return ((*first).major == (*second).major
        && (*first).minor == (*second).minor
        && (*first).micro == (*second).micro) as c_int;
}

unsafe extern "C" fn test_misc_version() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_misc_version\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7317,
    );
    let mut read_version: XML_Expat_Version = XML_ExpatVersionInfo();
    /* ! defined(XML_UNICODE) || defined(XML_UNICODE_WCHAR_T) */
    let mut parsed_version: XML_Expat_Version = {
        let mut init = XML_Expat_Version {
            major: 0,
            minor: 0,
            micro: 0,
        };
        init
    };
    let mut version_text: *const XML_LChar = XML_ExpatVersion();
    if version_text.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7324i32,
            b"Could not obtain version text\x00".as_ptr() as *const c_char,
        );
    }
    if !version_text.is_null() {
    } else {
        __assert_fail(
            b"version_text != NULL\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7325u32,
            (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(
                b"void test_misc_version(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if parse_version(version_text, &mut parsed_version) == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7327i32,
            b"Unable to parse version text\x00".as_ptr() as *const c_char,
        );
    }
    if versions_equal(&mut read_version, &mut parsed_version) == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7329i32,
            b"Version mismatch\x00".as_ptr() as *const c_char,
        );
    }
    if strcmp(version_text, b"expat_2.2.9\x00".as_ptr() as *const c_char) != 0 {
        /* Silence compiler warning with the following assignment */
        /* needs bump on releases */
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7333i32,
            b"XML_*_VERSION in expat.h out of sync?\n\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test feature information */

unsafe extern "C" fn test_misc_features() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 19], &[c_char; 19]>(b"test_misc_features\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7345,
    );
    let mut features: *const XML_Feature = XML_GetFeatureList();
    /* Prevent problems with double-freeing parsers */
    g_parser = ::rexpat::stddef_h::NULL as XML_Parser;
    if features.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7351i32,
            b"Failed to get feature information\x00".as_ptr() as *const c_char,
        );
    } else {
        /* Loop through the features checking what we can */
        while (*features).feature != XML_FEATURE_END {
            match (*features).feature {
                6 => {
                    if (*features).value as c_ulong != ::std::mem::size_of::<XML_Char>() as c_ulong
                    {
                        crate::minicheck::_fail_unless(0i32,
                                     
                                     b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr() as *const c_char,
                                     7358i32,
                                     
                                     b"Incorrect size of XML_Char\x00".as_ptr() as *const c_char);
                    }
                }
                7 => {
                    if (*features).value as c_ulong != ::std::mem::size_of::<XML_LChar>() as c_ulong
                    {
                        crate::minicheck::_fail_unless(0i32,
                                     
                                     b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr() as *const c_char,
                                     7362i32,
                                     
                                     b"Incorrect size of XML_LChar\x00".as_ptr() as *const c_char);
                    }
                }
                _ => {}
            }
            features = features.offset(1)
        }
    };
}
/* Regression test for GitHub Issue #17: memory leak parsing attribute
 * values with mixed bound and unbound namespaces.
 */

unsafe extern "C" fn test_misc_attribute_leak() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_misc_attribute_leak\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7376,
    );
    let mut text: *const c_char =
        b"<D xmlns:L=\"D\" l:a=\'\' L:a=\'\'/>\x00".as_ptr() as *const c_char;
    ALLOCATOR_MODE = AllocatorMode::Tracking;
    g_parser = XML_ParserCreate_MM(
        b"UTF-8\x00".as_ptr() as *const c_char,
        None,
        b"\n\x00".as_ptr() as *const c_char,
    );
    _expect_failure(
        text,
        XML_ERROR_UNBOUND_PREFIX,
        b"Unbound prefixes not found\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7382,
    );
    XML_ParserFree(g_parser);
    /* Prevent the teardown trying to double free */
    g_parser = ::rexpat::stddef_h::NULL as XML_Parser;
    if crate::memcheck::tracking_report() == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7388i32,
            b"Memory leak found\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test parser created for UTF-16LE is successful */

unsafe extern "C" fn test_misc_utf16le() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"test_misc_utf16le\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7393,
    );
    let text: [c_char; 61] =
        *::std::mem::transmute::<&[u8; 61],
                                 &[c_char; 61]>(b"<\x00?\x00x\x00m\x00l\x00 \x00v\x00e\x00r\x00s\x00i\x00o\x00n\x00=\x00\'\x001\x00.\x000\x00\'\x00?\x00>\x00<\x00q\x00>\x00H\x00i\x00<\x00/\x00q\x00>\x00\x00");
    let mut expected: *const XML_Char = b"Hi\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    g_parser = XML_ParserCreate(b"UTF-16LE\x00".as_ptr() as *const c_char);
    if g_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7404i32,
            b"Parser not created\x00".as_ptr() as *const c_char,
        );
    }
    crate::chardata::CharData_Init(&mut storage as *mut _);
    XML_SetUserData(
        g_parser,
        &mut storage as *mut crate::chardata::CharData as *mut c_void,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text.as_ptr(),
        ::std::mem::size_of::<[c_char; 61]>() as c_int - 1,
        XML_TRUE,
    ) == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7411i32,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
}

unsafe extern "C" fn start_element_issue_240(
    mut userData: *mut c_void,
    mut _name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
    let mut mydata: *mut DataIssue240 = userData as *mut DataIssue240;
    (*mydata).deep += 1;
}

unsafe extern "C" fn end_element_issue_240(mut userData: *mut c_void, mut _name: *const XML_Char) {
    let mut mydata: *mut DataIssue240 = userData as *mut DataIssue240;
    (*mydata).deep -= 1;
    if (*mydata).deep == 0 {
        XML_StopParser((*mydata).parser, 0u8);
    };
}

unsafe extern "C" fn test_misc_stop_during_end_handler_issue_240_1() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 46], &[c_char; 46]>(
            b"test_misc_stop_during_end_handler_issue_240_1\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7441,
    );
    let mut parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut mydata: *mut DataIssue240 = 0 as *mut DataIssue240;
    let mut result: XML_Status = XML_STATUS_ERROR;
    let doc1: *const c_char = b"<doc><e1/><e><foo/></e></doc>\x00".as_ptr() as *const c_char;
    parser = XML_ParserCreate(::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetElementHandler(
        parser,
        Some(
            start_element_issue_240
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
        Some(
            end_element_issue_240 as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    mydata = malloc(::std::mem::size_of::<DataIssue240>() as c_ulong) as *mut DataIssue240;
    (*mydata).parser = parser;
    (*mydata).deep = 0;
    XML_SetUserData(parser, mydata as *mut c_void);
    result = XML_Parse(parser, doc1, strlen(doc1) as c_int, 1);
    XML_ParserFree(parser);
    free(mydata as *mut c_void);
    if result != XML_STATUS_ERROR_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7458i32,
            b"Stopping the parser did not work as expected\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_misc_stop_during_end_handler_issue_240_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 46], &[c_char; 46]>(
            b"test_misc_stop_during_end_handler_issue_240_2\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7462,
    );
    let mut parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut mydata: *mut DataIssue240 = 0 as *mut DataIssue240;
    let mut result: XML_Status = XML_STATUS_ERROR;
    let doc2: *const c_char = b"<doc><elem/></doc>\x00".as_ptr() as *const c_char;
    parser = XML_ParserCreate(::rexpat::stddef_h::NULL as *const XML_Char);
    XML_SetElementHandler(
        parser,
        Some(
            start_element_issue_240
                as unsafe extern "C" fn(
                    _: *mut c_void,
                    _: *const XML_Char,
                    _: *mut *const XML_Char,
                ) -> (),
        ),
        Some(
            end_element_issue_240 as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
        ),
    );
    mydata = malloc(::std::mem::size_of::<DataIssue240>() as c_ulong) as *mut DataIssue240;
    (*mydata).parser = parser;
    (*mydata).deep = 0;
    XML_SetUserData(parser, mydata as *mut c_void);
    result = XML_Parse(parser, doc2, strlen(doc2) as c_int, 1);
    XML_ParserFree(parser);
    free(mydata as *mut c_void);
    if result != XML_STATUS_ERROR_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7479i32,
            b"Stopping the parser did not work as expected\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_misc_deny_internal_entity_closing_doctype_issue_317() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 57], &[c_char; 57]>(
            b"test_misc_deny_internal_entity_closing_doctype_issue_317\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7484,
    );
    let inputOne: *const c_char =
        b"<!DOCTYPE d [\n<!ENTITY % e \']><d/>\'>\n\n%e;\x00".as_ptr() as *const c_char;
    let inputTwo: *const c_char =
        b"<!DOCTYPE d [\n<!ENTITY % e1 \']><d/>\'><!ENTITY % e2 \'&e1;\'>\n\n%e2;\x00".as_ptr()
            as *const c_char;
    let inputThree: *const c_char =
        b"<!DOCTYPE d [\n<!ENTITY % e \']><d\'>\n\n%e;\x00".as_ptr() as *const c_char;
    let inputIssue317: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ENTITY % foo \']>\n<doc>Hell<oc (#PCDATA)*>\'>\n%foo;\n]>\n<doc>Hello, world</dVc>\x00".as_ptr() as *const c_char;
    let inputs: [*const c_char; 4] = [inputOne, inputTwo, inputThree, inputIssue317];
    let mut inputIndex: size_t = 0;
    while inputIndex
        < (::std::mem::size_of::<[*const c_char; 4]>() as c_ulong)
            .wrapping_div(::std::mem::size_of::<*const c_char>() as c_ulong)
    {
        let mut parser: XML_Parser = 0 as *mut XML_ParserStruct;
        let mut parseResult: XML_Status = XML_STATUS_ERROR;
        let mut setParamEntityResult: c_int = 0;
        let mut lineNumber: XML_Size = 0;
        let mut columnNumber: XML_Size = 0;
        let input: *const c_char = inputs[inputIndex as usize];
        parser = XML_ParserCreate(::rexpat::stddef_h::NULL as *const XML_Char);
        setParamEntityResult = XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        if setParamEntityResult != 1 {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                7519i32,
                b"Failed to set XML_PARAM_ENTITY_PARSING_ALWAYS.\x00".as_ptr() as *const c_char,
            );
        }
        parseResult = XML_Parse(parser, input, strlen(input) as c_int, 0);
        if parseResult != XML_STATUS_ERROR_0 as c_uint {
            parseResult = XML_Parse(parser, b"\x00".as_ptr() as *const c_char, 0, 1);
            if parseResult != XML_STATUS_ERROR_0 as c_uint {
                crate::minicheck::_fail_unless(
                    0i32,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                        .as_ptr() as *const c_char,
                    7525i32,
                    b"Parsing was expected to fail but succeeded.\x00".as_ptr() as *const c_char,
                );
            }
        }
        if XML_GetErrorCode(parser) != XML_ERROR_INVALID_TOKEN {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                7530i32,
                b"Error code does not match XML_ERROR_INVALID_TOKEN\x00".as_ptr() as *const c_char,
            );
        }
        lineNumber = XML_GetCurrentLineNumber(parser);
        if lineNumber != 4 {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                7534i32,
                b"XML_GetCurrentLineNumber does not work as expected.\x00".as_ptr()
                    as *const c_char,
            );
        }
        columnNumber = XML_GetCurrentColumnNumber(parser);
        if columnNumber != 0 {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                7538i32,
                b"XML_GetCurrentColumnNumber does not work as expected.\x00".as_ptr()
                    as *const c_char,
            );
        }
        XML_ParserFree(parser);
        inputIndex = inputIndex.wrapping_add(1)
    }
}

unsafe extern "C" fn alloc_setup() {
    ALLOCATOR_MODE = AllocatorMode::Duff;
    /* Ensure the parser creation will go through */
    allocation_count = ALLOC_ALWAYS_SUCCEED as intptr_t;
    reallocation_count = REALLOC_ALWAYS_SUCCEED as intptr_t;
    g_parser = XML_ParserCreate_MM(
        ::rexpat::stddef_h::NULL as *const XML_Char,
        None,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if g_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7555i32,
            b"Parser not created\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn alloc_teardown() {
    basic_teardown();
}
/* Test the effects of allocation failures on xml declaration processing */

unsafe extern "C" fn test_alloc_parse_xdecl() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_alloc_parse_xdecl\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7564,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n<doc>Hello, world</doc>\x00".as_ptr()
            as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 15;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetXmlDeclHandler(
            g_parser,
            Some(
                dummy_xdecl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* Resetting the parser is insufficient, because some memory
         * allocations are cached within the parser.  Instead we use
         * the teardown and setup routines to ensure that we have the
         * right sort of parser back in our hands.
         */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7585i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7587i32,
            b"Parse failed with max allocations\x00".as_ptr() as *const c_char,
        );
    };
}
/* As above, but with an encoding big enough to cause storing the
 * version information to expand the string pool being used.
 */

unsafe extern "C" fn long_encoding_handler(
    mut _userData: *mut c_void,
    mut _encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> c_int {
    let mut i: c_int = 0;
    i = 0;
    while i < 256 {
        (*info).map[i as usize] = i;
        i += 1
    }
    (*info).data = ::rexpat::stddef_h::NULL as *mut c_void;
    (*info).convert = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int>,
    >(::rexpat::stddef_h::NULL as libc::intptr_t);
    (*info).release = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: *mut c_void) -> ()>,
    >(::rexpat::stddef_h::NULL as libc::intptr_t);
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_alloc_parse_xdecl_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_alloc_parse_xdecl_2\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7609,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'ThisIsAStupidlyLongEncodingNameIntendedToTriggerPoolGrowth123456ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMN\'?><doc>Hello, world</doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 20;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetXmlDeclHandler(
            g_parser,
            Some(
                dummy_xdecl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
        );
        XML_SetUnknownEncodingHandler(
            g_parser,
            transmute(Some(
                long_encoding_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut XML_Encoding,
                    ) -> c_int,
            )),
            ::rexpat::stddef_h::NULL as *mut c_void,
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7646i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7648i32,
            b"Parse failed with max allocations\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test the effects of allocation failures on a straightforward parse */

unsafe extern "C" fn test_alloc_parse_pi() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 20], &[c_char; 20]>(b"test_alloc_parse_pi\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7653,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n<?pi unknown?>\n<doc>Hello, world</doc>\x00"
            .as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 15;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                dummy_pi_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7673i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7675i32,
            b"Parse failed with max allocations\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_parse_pi_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_alloc_parse_pi_2\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7679,
    );
    let mut text: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n<doc>Hello, world<?pi unknown?>\n</doc>\x00"
            .as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 15;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                dummy_pi_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7699i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7701i32,
            b"Parse failed with max allocations\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_parse_pi_3() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_alloc_parse_pi_3\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7705,
    );
    let mut text: *const c_char =
        
        b"<?This processing instruction should be long enough to ensure thatit triggers the growth of an internal string pool when the      allocator fails at a cruicial moment FGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPQ?><doc/>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 20;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                dummy_pi_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7740i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7742i32,
            b"Parse failed with max allocations\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_parse_comment() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_alloc_parse_comment\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7746,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n<!-- Test parsing this comment --><doc>Hi</doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 15;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetCommentHandler(
            g_parser,
            Some(
                dummy_comment_handler
                    as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7764i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7766i32,
            b"Parse failed with max allocations\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_parse_comment_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_alloc_parse_comment_2\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7770,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n<doc>Hello, world<!-- Parse this comment too --></doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 15;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetCommentHandler(
            g_parser,
            Some(
                dummy_comment_handler
                    as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7790i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7792i32,
            b"Parse failed with max allocations\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn external_entity_duff_loader(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut new_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut i: c_uint = 0;
    let max_alloc_count: c_uint = 10;
    /* Try a few different allocation levels */
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        new_parser = XML_ExternalEntityParserCreate(
            parser,
            context,
            ::rexpat::stddef_h::NULL as *const XML_Char,
        );
        if !new_parser.is_null() {
            XML_ParserFree(new_parser);
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7817i32,
            b"External parser creation ignored failing allocator\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7819i32,
            b"Extern parser not created with max allocation count\x00".as_ptr() as *const c_char,
        );
    }
    /* Make sure other random allocation doesn't now fail */
    allocation_count = ALLOC_ALWAYS_SUCCEED as intptr_t;
    /* Make sure the failure code path is executed too */
    return XML_STATUS_ERROR_0;
}
/* Test that external parser creation running out of memory is
 * correctly reported.  Based on the external entity test cases.
 */

unsafe extern "C" fn test_alloc_create_external_parser() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 34], &[c_char; 34]>(
            b"test_alloc_create_external_parser\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7831,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut foo_text: [c_char; 26] =
        *::std::mem::transmute::<&[u8; 26], &mut [c_char; 26]>(b"<!ELEMENT doc (#PCDATA)*>\x00");
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, foo_text.as_mut_ptr() as *mut c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_duff_loader
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7842i32,
            b"External parser allocator returned success incorrectly\x00".as_ptr() as *const c_char,
        );
    };
}
/* More external parser memory allocation testing */

unsafe extern "C" fn test_alloc_run_external_parser() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_alloc_run_external_parser\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7848,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<!DOCTYPE doc SYSTEM \'foo\'>\n<doc>&entity;</doc>\x00".as_ptr() as *const c_char;
    let mut foo_text: [c_char; 26] =
        *::std::mem::transmute::<&[u8; 26], &mut [c_char; 26]>(b"<!ELEMENT doc (#PCDATA)*>\x00");
    let mut i: c_uint = 0;
    let max_alloc_count: c_uint = 15;
    i = 0;
    while i < max_alloc_count {
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetUserData(g_parser, foo_text.as_mut_ptr() as *mut c_void);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_null_loader
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i = i.wrapping_add(1)
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7869i32,
            b"Parsing ignored failing allocator\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7871i32,
            b"Parsing failed with allocation count 10\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn external_entity_dbl_handler(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut callno: intptr_t = *(parser as *mut *mut c_void) as intptr_t;
    let mut text: *const c_char = 0 as *const c_char;
    let mut new_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 20;
    if callno == 0 {
        /* First time through, check how many calls to malloc occur */
        text = b"<!ELEMENT doc (e+)>\n<!ATTLIST doc xmlns CDATA #IMPLIED>\n<!ELEMENT e EMPTY>\n\x00"
            .as_ptr() as *const c_char;
        allocation_count = 10000;
        new_parser = XML_ExternalEntityParserCreate(
            parser,
            context,
            ::rexpat::stddef_h::NULL as *const XML_Char,
        );
        if new_parser.is_null() {
            crate::minicheck::_fail_unless(
                0,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                7896,
                b"Unable to allocate first external parser\x00".as_ptr() as *const c_char,
            );
            return XML_STATUS_ERROR_0;
        }
        /* Stash the number of calls in the user data */
        XML_SetUserData(parser, (10000i64 - allocation_count) as *mut c_void);
    } else {
        text = b"<?xml version=\'1.0\' encoding=\'us-ascii\'?><e/>\x00".as_ptr() as *const c_char;
        /* Try at varying levels to exercise more code paths */
        i = 0;
        while i < max_alloc_count {
            allocation_count = callno + i as c_long;
            new_parser = XML_ExternalEntityParserCreate(
                parser,
                context,
                ::rexpat::stddef_h::NULL as *const XML_Char,
            );
            if !new_parser.is_null() {
                break;
            }
            i += 1
        }
        if i == 0 {
            crate::minicheck::_fail_unless(
                0,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                7912,
                b"Second external parser unexpectedly created\x00".as_ptr() as *const c_char,
            );
            XML_ParserFree(new_parser);
            return XML_STATUS_ERROR_0;
        } else {
            if i == max_alloc_count {
                crate::minicheck::_fail_unless(
                    0,
                    b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                        .as_ptr() as *const c_char,
                    7916,
                    b"Second external parser not created\x00".as_ptr() as *const c_char,
                );
                return XML_STATUS_ERROR_0;
            }
        }
    }
    allocation_count = ALLOC_ALWAYS_SUCCEED as intptr_t;
    if _XML_Parse_SINGLE_BYTES(new_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            new_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7924,
        );
        return XML_STATUS_ERROR_0;
    }
    XML_ParserFree(new_parser);
    return XML_STATUS_OK_0;
}
/* Test that running out of memory in dtdCopy is correctly reported.
 * Based on test_default_ns_from_ext_subset_and_ext_ge()
 */

unsafe extern "C" fn test_alloc_dtd_copy_default_atts() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_alloc_dtd_copy_default_atts\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7934,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\'?>\n<!DOCTYPE doc SYSTEM \'http://example.org/doc.dtd\' [\n  <!ENTITY en SYSTEM \'http://example.org/entity.ent\'>\n]>\n<doc xmlns=\'http://example.org/ns1\'>\n&en;\n</doc>\x00".as_ptr() as *const c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_dbl_handler
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    XML_SetUserData(g_parser, ::rexpat::stddef_h::NULL as *mut c_void);
    if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            7948i32,
        );
    };
}

unsafe extern "C" fn external_entity_dbl_handler_2(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut callno: intptr_t = *(parser as *mut *mut c_void) as intptr_t;
    let mut text: *const c_char = 0 as *const c_char;
    let mut new_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut rv: XML_Status = XML_STATUS_ERROR;
    if callno == 0 {
        /* Try different allocation levels for whole exercise */
        text = b"<!ELEMENT doc (e+)>\n<!ATTLIST doc xmlns CDATA #IMPLIED>\n<!ELEMENT e EMPTY>\n\x00"
            .as_ptr() as *const c_char;
        XML_SetUserData(parser, 1i64 as *mut c_void);
        new_parser = XML_ExternalEntityParserCreate(
            parser,
            context,
            ::rexpat::stddef_h::NULL as *const XML_Char,
        );
        if new_parser.is_null() {
            return XML_STATUS_ERROR_0;
        }
        rv = _XML_Parse_SINGLE_BYTES(new_parser, text, strlen(text) as c_int, XML_TRUE)
    } else {
        /* Just run through once */
        text = b"<?xml version=\'1.0\' encoding=\'us-ascii\'?><e/>\x00".as_ptr() as *const c_char;
        new_parser = XML_ExternalEntityParserCreate(
            parser,
            context,
            ::rexpat::stddef_h::NULL as *const XML_Char,
        );
        if new_parser.is_null() {
            return XML_STATUS_ERROR_0;
        }
        rv = _XML_Parse_SINGLE_BYTES(new_parser, text, strlen(text) as c_int, XML_TRUE)
    }
    XML_ParserFree(new_parser);
    if rv == XML_STATUS_ERROR_0 as c_uint {
        return XML_STATUS_ERROR_0;
    }
    return XML_STATUS_OK_0;
}
/* Test more external entity allocation failure paths */

unsafe extern "C" fn test_alloc_external_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_alloc_external_entity\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        7990,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\'?>\n<!DOCTYPE doc SYSTEM \'http://example.org/doc.dtd\' [\n  <!ENTITY en SYSTEM \'http://example.org/entity.ent\'>\n]>\n<doc xmlns=\'http://example.org/ns1\'>\n&en;\n</doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let alloc_test_max_repeats: c_int = 50;
    i = 0;
    while i < alloc_test_max_repeats {
        allocation_count = -1;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_dbl_handler_2
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        XML_SetUserData(g_parser, ::rexpat::stddef_h::NULL as *mut c_void);
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            == XML_STATUS_OK_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    allocation_count = -1;
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8016i32,
            b"External entity parsed despite duff allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == alloc_test_max_repeats {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8018i32,
            b"External entity not parsed at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test more allocation failure paths */

unsafe extern "C" fn external_entity_alloc_set_encoding(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    /* As for external_entity_loader() */
    let mut text: *const c_char =
        b"<?xml encoding=\'iso-8859-3\'?>\xc3\xa9\x00".as_ptr() as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut status: XML_Status = XML_STATUS_ERROR;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        return XML_STATUS_ERROR_0;
    }
    if XML_SetEncoding(ext_parser, b"utf-8\x00".as_ptr() as *const c_char) as u64 == 0 {
        XML_ParserFree(ext_parser);
        return XML_STATUS_ERROR_0;
    }
    status = _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE);
    XML_ParserFree(ext_parser);
    if status == XML_STATUS_ERROR_0 as c_uint {
        return XML_STATUS_ERROR_0;
    }
    return XML_STATUS_OK_0;
}

unsafe extern "C" fn test_alloc_ext_entity_set_encoding() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_alloc_ext_entity_set_encoding\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8052,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_allocation_count: c_int = 30;
    i = 0;
    while i < max_allocation_count {
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_alloc_set_encoding
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            == XML_STATUS_OK_0 as c_uint
        {
            break;
        }
        allocation_count = -1;
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8073i32,
            b"Encoding check succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_allocation_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8075i32,
            b"Encoding failed at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn unknown_released_encoding_handler(
    mut _data: *mut c_void,
    mut encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> c_int {
    if strcmp(
        encoding,
        b"unsupported-encoding\x00".as_ptr() as *const c_char,
    ) == 0
    {
        let mut i: c_int = 0;
        i = 0;
        while i < 256 {
            (*info).map[i as usize] = i;
            i += 1
        }
        (*info).data = ::rexpat::stddef_h::NULL as *mut c_void;
        (*info).convert = ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int>,
        >(::rexpat::stddef_h::NULL as libc::intptr_t);
        (*info).release = Some(dummy_release as unsafe extern "C" fn(_: *mut c_void) -> ());
        return XML_STATUS_OK_0;
    }
    return XML_STATUS_ERROR_0;
}
/* Test the effects of allocation failure in internal entities.
 * Based on test_unknown_encoding_internal_entity
 */

unsafe extern "C" fn test_alloc_internal_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_alloc_internal_entity\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8099,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'unsupported-encoding\'?>\n<!DOCTYPE test [<!ENTITY foo \'bar\'>]>\n<test a=\'&foo;\'/>\x00".as_ptr() as *const c_char;
    let mut i: c_uint = 0;
    let max_alloc_count: c_uint = 20;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetUnknownEncodingHandler(
            g_parser,
            transmute(Some(
                unknown_released_encoding_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut XML_Encoding,
                    ) -> c_int,
            )),
            ::rexpat::stddef_h::NULL as *mut c_void,
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i = i.wrapping_add(1)
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8118i32,
            b"Internal entity worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8120i32,
            b"Internal entity failed at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test the robustness against allocation failure of element handling
 * Based on test_dtd_default_handling().
 */

unsafe extern "C" fn test_alloc_dtd_default_handling() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_alloc_dtd_default_handling\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8127,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ENTITY e SYSTEM \'http://example.org/e\'>\n<!NOTATION n SYSTEM \'http://example.org/n\'>\n<!ENTITY e1 SYSTEM \'http://example.org/e\' NDATA n>\n<!ELEMENT doc (#PCDATA)>\n<!ATTLIST doc a CDATA #IMPLIED>\n<?pi in dtd?>\n<!--comment in dtd-->\n]>\n<doc><![CDATA[text in doc]]></doc>\x00".as_ptr() as *const c_char;
    let mut expected: *const XML_Char =
        b"\n\n\n\n\n\n\n\n\n<doc>text in doc</doc>\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 25;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        dummy_handler_flags = 0;
        XML_SetDefaultHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
            ),
        );
        XML_SetDoctypeDeclHandler(
            g_parser,
            Some(
                dummy_start_doctype_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
            Some(dummy_end_doctype_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
        );
        XML_SetEntityDeclHandler(
            g_parser,
            Some(
                dummy_entity_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: c_int,
                        _: *const XML_Char,
                        _: c_int,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetNotationDeclHandler(
            g_parser,
            Some(
                dummy_notation_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetElementDeclHandler(
            g_parser,
            transmute(Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut XML_Content,
                    ) -> (),
            )),
        );
        XML_SetAttlistDeclHandler(
            g_parser,
            Some(
                dummy_attlist_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
        );
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                dummy_pi_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetCommentHandler(
            g_parser,
            Some(
                dummy_comment_handler
                    as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
            ),
        );
        XML_SetCdataSectionHandler(
            g_parser,
            Some(dummy_start_cdata_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
            Some(dummy_end_cdata_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
        );
        XML_SetUnparsedEntityDeclHandler(
            g_parser,
            Some(
                dummy_unparsed_entity_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        crate::chardata::CharData_Init(&mut storage as *mut _);
        XML_SetUserData(
            g_parser,
            &mut storage as *mut crate::chardata::CharData as *mut c_void,
        );
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8170i32,
            b"Default DTD parsed despite allocation failures\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8172i32,
            b"Default DTD not parsed with maximum alloc count\x00".as_ptr() as *const c_char,
        );
    }
    crate::chardata::CharData_CheckXMLChars(&mut storage as *mut _, expected);
    if dummy_handler_flags
        != DUMMY_START_DOCTYPE_HANDLER_FLAG
            | DUMMY_END_DOCTYPE_HANDLER_FLAG
            | DUMMY_ENTITY_DECL_HANDLER_FLAG
            | DUMMY_NOTATION_DECL_HANDLER_FLAG
            | DUMMY_ELEMENT_DECL_HANDLER_FLAG
            | DUMMY_ATTLIST_DECL_HANDLER_FLAG
            | DUMMY_COMMENT_HANDLER_FLAG
            | DUMMY_PI_HANDLER_FLAG
            | DUMMY_START_CDATA_HANDLER_FLAG
            | DUMMY_END_CDATA_HANDLER_FLAG
            | DUMMY_UNPARSED_ENTITY_DECL_HANDLER_FLAG
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8181i32,
            b"Not all handlers were called\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test robustness of XML_SetEncoding() with a failing allocator */

unsafe extern "C" fn test_alloc_explicit_encoding() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_alloc_explicit_encoding\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8186,
    );
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 5;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if XML_SetEncoding(g_parser, b"us-ascii\x00".as_ptr() as *const c_char)
            == XML_STATUS_OK_0 as c_uint
        {
            break;
        }
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8196i32,
            b"Encoding set despite failing allocator\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8198i32,
            b"Encoding not set at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test robustness of XML_SetBase against a failing allocator */

unsafe extern "C" fn test_alloc_set_base() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 20], &[c_char; 20]>(b"test_alloc_set_base\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8203,
    );
    let mut new_base: *const XML_Char = b"/local/file/name.xml\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 5;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if XML_SetBase(g_parser, new_base) == XML_STATUS_OK_0 as c_uint {
            break;
        }
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8214i32,
            b"Base set despite failing allocator\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8216i32,
            b"Base not set with max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test buffer extension in the face of a duff reallocator */

unsafe extern "C" fn test_alloc_realloc_buffer() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_alloc_realloc_buffer\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8221,
    );
    let mut text: *const c_char = get_buffer_test_text;
    let mut buffer: *mut c_void = 0 as *mut c_void;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    /* Get a smallish buffer */
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        buffer = XML_GetBuffer(g_parser, 1536);
        if buffer.is_null() {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                8232i32,
                b"1.5K buffer reallocation failed\x00".as_ptr() as *const c_char,
            );
        }
        if !buffer.is_null() {
        } else {
            __assert_fail(
                b"buffer != NULL\x00".as_ptr() as *const c_char,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                8233u32,
                (*::std::mem::transmute::<&[u8; 37], &[c_char; 37]>(
                    b"void test_alloc_realloc_buffer(void)\x00",
                ))
                .as_ptr(),
            );
        }
        memcpy(buffer, text as *const c_void, strlen(text));
        if XML_ParseBuffer(g_parser, strlen(text) as c_int, XML_FALSE as c_int)
            == XML_STATUS_OK_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    reallocation_count = -1;
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8244i32,
            b"Parse succeeded with no reallocation\x00".as_ptr() as *const c_char,
        );
    } else if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8246i32,
            b"Parse failed with max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Same test for external entity parsers */

unsafe extern "C" fn external_entity_reallocator(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char = get_buffer_test_text;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut buffer: *mut c_void = 0 as *mut c_void;
    let mut status: XML_Status = XML_STATUS_ERROR;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8265i32,
            b"Could not create external entity parser\x00".as_ptr() as *const c_char,
        );
    }
    reallocation_count = *(parser as *mut *mut c_void) as intptr_t;
    buffer = XML_GetBuffer(ext_parser, 1536);
    if buffer.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8270i32,
            b"Buffer allocation failed\x00".as_ptr() as *const c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(b"buffer != NULL\x00".as_ptr() as
                          *const c_char,
                      
                      b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr() as *const c_char,
                      8271u32,
                      (*::std::mem::transmute::<&[u8; 116],
                                                &[c_char; 116]>(b"int external_entity_reallocator(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\x00")).as_ptr());
    }
    memcpy(buffer, text as *const c_void, strlen(text));
    status = XML_ParseBuffer(ext_parser, strlen(text) as c_int, XML_FALSE as c_int);
    reallocation_count = -1;
    XML_ParserFree(ext_parser);
    return if status == XML_STATUS_OK_0 as c_uint {
        XML_STATUS_OK_0
    } else {
        XML_STATUS_ERROR_0
    };
}

unsafe extern "C" fn test_alloc_ext_entity_realloc_buffer() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 37], &[c_char; 37]>(
            b"test_alloc_ext_entity_realloc_buffer\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8279,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM \'http://example.org/dummy.ent\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_reallocator
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        XML_SetUserData(g_parser, i as *mut c_void);
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            == XML_STATUS_OK_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8298i32,
            b"Succeeded with no reallocations\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8300i32,
            b"Failed with max reallocations\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test elements with many attributes are handled correctly */

unsafe extern "C" fn test_alloc_realloc_many_attributes() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_alloc_realloc_many_attributes\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8305,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ATTLIST doc za CDATA \'default\'>\n<!ATTLIST doc zb CDATA \'def2\'>\n<!ATTLIST doc zc CDATA \'def3\'>\n]>\n<doc a=\'1\'     b=\'2\'     c=\'3\'     d=\'4\'     e=\'5\'     f=\'6\'     g=\'7\'     h=\'8\'     i=\'9\'     j=\'10\'     k=\'11\'     l=\'12\'     m=\'13\'     n=\'14\'     p=\'15\'     q=\'16\'     r=\'17\'     s=\'18\'></doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8343i32,
            b"Parse succeeded despite no reallocations\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8345i32,
            b"Parse failed at max reallocations\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test handling of a public entity with failing allocator */

unsafe extern "C" fn test_alloc_public_entity_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 31], &[c_char; 31]>(b"test_alloc_public_entity_value\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8350,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'http://example.org/\'>\n<doc></doc>\n\x00".as_ptr()
            as *const c_char;
    let mut dtd_text: [c_char; 1109] =
        *::std::mem::transmute::<&[u8; 1109],
                                 &mut [c_char; 1109]>(b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 PUBLIC \'foo\' \'bar.ent\'>\n<!ENTITY % ThisIsAStupidlyLongParameterNameIntendedToTriggerPoolGrowth12345ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP \'%e1;\'>\n%e1;\n\x00");
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 50;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        dummy_handler_flags = 0;
        XML_SetUserData(g_parser, dtd_text.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_public
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        /* Provoke a particular code path */
        XML_SetEntityDeclHandler(
            g_parser,
            Some(
                dummy_entity_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: c_int,
                        _: *const XML_Char,
                        _: c_int,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8395i32,
            b"Parsing worked despite failing allocation\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8397i32,
            b"Parsing failed at max allocation count\x00".as_ptr() as *const c_char,
        );
    }
    if dummy_handler_flags != DUMMY_ENTITY_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8399i32,
            b"Entity declaration handler not called\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_realloc_subst_public_entity_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 45], &[c_char; 45]>(
            b"test_alloc_realloc_subst_public_entity_value\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8403,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'http://example.org/\'>\n<doc></doc>\n\x00".as_ptr()
            as *const c_char;
    let mut dtd_text: [c_char; 2108] =
        *::std::mem::transmute::<&[u8; 2108],
                                 &mut [c_char; 2108]>(b"<!ELEMENT doc EMPTY>\n<!ENTITY % ThisIsAStupidlyLongParameterNameIntendedToTriggerPoolGrowth12345ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP PUBLIC \'foo\' \'bar.ent\'>\n%ThisIsAStupidlyLongParameterNameIntendedToTriggerPoolGrowth12345ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP;\x00");
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        XML_SetUserData(g_parser, dtd_text.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_public
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8459i32,
            b"Parsing worked despite failing reallocation\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8461i32,
            b"Parsing failed at max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_parse_public_doctype() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_alloc_parse_public_doctype\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8465,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n<!DOCTYPE doc PUBLIC \'http://example.com/a/long/enough/name/to/trigger/pool/growth/zz/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/\' \'test\'>\n<doc></doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 25;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        dummy_handler_flags = 0;
        XML_SetDoctypeDeclHandler(
            g_parser,
            Some(
                dummy_start_doctype_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
            Some(dummy_end_doctype_decl_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8504i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8506i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    }
    if dummy_handler_flags
        != DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG | DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8510i32,
            b"Doctype handler functions not called\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_parse_public_doctype_long_name() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 42], &[c_char; 42]>(
            b"test_alloc_parse_public_doctype_long_name\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8514,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' encoding=\'utf-8\'?>\n<!DOCTYPE doc PUBLIC \'http://example.com/foo\' \'ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOP\'>\n<doc></doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 25;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetDoctypeDeclHandler(
            g_parser,
            Some(
                dummy_start_doctype_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
            Some(dummy_end_doctype_decl_handler as unsafe extern "C" fn(_: *mut c_void) -> ()),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8552i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8554i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn external_entity_alloc(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut _base: *const XML_Char,
    mut _systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut text: *const c_char = *(parser as *mut *mut c_void) as *const c_char;
    let mut ext_parser: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut parse_res: c_int = 0;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::rexpat::stddef_h::NULL as *const XML_Char,
    );
    if ext_parser.is_null() {
        return XML_STATUS_ERROR_0;
    }
    parse_res = _XML_Parse_SINGLE_BYTES(ext_parser, text, strlen(text) as c_int, XML_TRUE) as c_int;
    XML_ParserFree(ext_parser);
    return parse_res;
}
/* Test foreign DTD handling */

unsafe extern "C" fn test_alloc_set_foreign_dtd() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_alloc_set_foreign_dtd\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8579,
    );
    let mut text1: *const c_char =
        b"<?xml version=\'1.0\' encoding=\'us-ascii\'?>\n<doc>&entity;</doc>\x00".as_ptr()
            as *const c_char;
    let mut text2: [c_char; 26] =
        *::std::mem::transmute::<&[u8; 26], &mut [c_char; 26]>(b"<!ELEMENT doc (#PCDATA)*>\x00");
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 25;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetUserData(g_parser, &mut text2 as *mut [c_char; 26] as *mut c_void);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_alloc
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_NONE {
            crate::minicheck::_fail_unless(
                0i32,
                b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00"
                    .as_ptr() as *const c_char,
                8592i32,
                b"Could not set foreign DTD\x00".as_ptr() as *const c_char,
            );
        }
        if _XML_Parse_SINGLE_BYTES(g_parser, text1, strlen(text1) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8601i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8603i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test based on ibm/valid/P32/ibm32v04.xml */

unsafe extern "C" fn test_alloc_attribute_enum_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_alloc_attribute_enum_value\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8608,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' standalone=\'no\'?>\n<!DOCTYPE animal SYSTEM \'test.dtd\'>\n<animal>This is a \n    <a/>  \n\nyellow tiger</animal>\x00".as_ptr() as *const c_char;
    let mut dtd_text: [c_char; 108] =
        *::std::mem::transmute::<&[u8; 108],
                                 &mut [c_char; 108]>(b"<!ELEMENT animal (#PCDATA|a)*>\n<!ELEMENT a EMPTY>\n<!ATTLIST animal xml:space (default|preserve) \'preserve\'>\x00");
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 30;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_alloc
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        XML_SetUserData(g_parser, dtd_text.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        /* An attribute list handler provokes a different code path */
        XML_SetAttlistDeclHandler(
            g_parser,
            Some(
                dummy_attlist_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8633i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8635i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test attribute enums sufficient to overflow the string pool */

unsafe extern "C" fn test_alloc_realloc_attribute_enum_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 40], &[c_char; 40]>(
            b"test_alloc_realloc_attribute_enum_value\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8640,
    );
    let mut text: *const c_char =
        
        b"<?xml version=\'1.0\' standalone=\'no\'?>\n<!DOCTYPE animal SYSTEM \'test.dtd\'>\n<animal>This is a yellow tiger</animal>\x00".as_ptr() as *const c_char;
    /* We wish to define a collection of attribute enums that will
     * cause the string pool storing them to have to expand.  This
     * means more than 1024 bytes, including the parentheses and
     * separator bars.
     */
    let mut dtd_text: [c_char; 1097] =
        *::std::mem::transmute::<&[u8; 1097],
                                 &mut [c_char; 1097]>(b"<!ELEMENT animal (#PCDATA)*>\n<!ATTLIST animal thing (default|ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|BBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|CBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|DBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|EBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|FBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|GBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|HBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|IBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|JBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|KBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|LBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|MBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|NBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|OBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|PBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO) \'default\'>\x00");
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_alloc
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        XML_SetUserData(g_parser, dtd_text.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        /* An attribute list handler provokes a different code path */
        XML_SetAttlistDeclHandler(
            g_parser,
            Some(
                dummy_attlist_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8689i32,
            b"Parse succeeded despite failing reallocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8691i32,
            b"Parse failed at maximum reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test attribute enums in a #IMPLIED attribute forcing pool growth */

unsafe extern "C" fn test_alloc_realloc_implied_attribute() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 37], &[c_char; 37]>(
            b"test_alloc_realloc_implied_attribute\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8696,
    );
    /* Forcing this particular code path is a balancing act.  The
     * addition of the closing parenthesis and terminal NUL must be
     * what pushes the string of enums over the 1024-byte limit,
     * otherwise a different code path will pick up the realloc.
     */
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ELEMENT doc EMPTY>\n<!ATTLIST doc a (ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|BBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|CBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|DBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|EBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|FBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|GBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|HBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|IBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|JBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|KBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|LBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|MBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|NBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|OBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|PBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMN) #IMPLIED>\n]><doc/>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        XML_SetAttlistDeclHandler(
            g_parser,
            Some(
                dummy_attlist_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8739i32,
            b"Parse succeeded despite failing reallocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8741i32,
            b"Parse failed at maximum reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test attribute enums in a defaulted attribute forcing pool growth */

unsafe extern "C" fn test_alloc_realloc_default_attribute() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 37], &[c_char; 37]>(
            b"test_alloc_realloc_default_attribute\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8746,
    );
    /* Forcing this particular code path is a balancing act.  The
     * addition of the closing parenthesis and terminal NUL must be
     * what pushes the string of enums over the 1024-byte limit,
     * otherwise a different code path will pick up the realloc.
     */
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ELEMENT doc EMPTY>\n<!ATTLIST doc a (ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|BBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|CBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|DBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|EBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|FBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|GBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|HBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|IBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|JBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|KBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|LBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|MBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|NBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|OBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|PBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMN) \'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO\'>\n]><doc/>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        XML_SetAttlistDeclHandler(
            g_parser,
            Some(
                dummy_attlist_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8789i32,
            b"Parse succeeded despite failing reallocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8791i32,
            b"Parse failed at maximum reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test long notation name with dodgy allocator */

unsafe extern "C" fn test_alloc_notation() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 20], &[c_char; 20]>(b"test_alloc_notation\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8796,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!NOTATION ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP SYSTEM \'http://example.org/n\'>\n<!ENTITY e SYSTEM \'http://example.org/e\' NDATA ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 20;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        dummy_handler_flags = 0;
        XML_SetNotationDeclHandler(
            g_parser,
            Some(
                dummy_notation_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetEntityDeclHandler(
            g_parser,
            Some(
                dummy_entity_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: c_int,
                        _: *const XML_Char,
                        _: c_int,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8854i32,
            b"Parse succeeded despite allocation failures\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8856i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    }
    if dummy_handler_flags != DUMMY_ENTITY_DECL_HANDLER_FLAG | DUMMY_NOTATION_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8859i32,
            b"Entity declaration handler not called\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test public notation with dodgy allocator */

unsafe extern "C" fn test_alloc_public_notation() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_alloc_public_notation\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8864,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!NOTATION note PUBLIC \'http://example.com/a/long/enough/name/to/trigger/pool/growth/zz/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/\' \'foo\'>\n<!ENTITY e SYSTEM \'http://example.com/e\' NDATA note>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 20;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        dummy_handler_flags = 0;
        XML_SetNotationDeclHandler(
            g_parser,
            Some(
                dummy_notation_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8904i32,
            b"Parse succeeded despite allocation failures\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8906i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    }
    if dummy_handler_flags != DUMMY_NOTATION_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8908i32,
            b"Notation handler not called\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test public notation with dodgy allocator */

unsafe extern "C" fn test_alloc_system_notation() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_alloc_system_notation\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8913,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!NOTATION note SYSTEM \'http://example.com/a/long/enough/name/to/trigger/pool/growth/zz/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/\'>\n<!ENTITY e SYSTEM \'http://example.com/e\' NDATA note>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 20;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        dummy_handler_flags = 0;
        XML_SetNotationDeclHandler(
            g_parser,
            Some(
                dummy_notation_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8953i32,
            b"Parse succeeded despite allocation failures\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8955i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    }
    if dummy_handler_flags != DUMMY_NOTATION_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8957i32,
            b"Notation handler not called\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_nested_groups() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_alloc_nested_groups\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        8961,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ELEMENT doc (e,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?))))))))))))))))))))))))))))))))>\n<!ELEMENT e EMPTY>]>\n<doc><e/></doc>\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 20;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        crate::chardata::CharData_Init(&mut storage as *mut _);
        XML_SetElementDeclHandler(
            g_parser,
            transmute(Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut XML_Content,
                    ) -> (),
            )),
        );
        XML_SetStartElementHandler(
            g_parser,
            Some(
                record_element_start_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(
            g_parser,
            &mut storage as *mut crate::chardata::CharData as *mut c_void,
        );
        dummy_handler_flags = 0;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8992i32,
            b"Parse succeeded despite failing reallocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8994i32,
            b"Parse failed at maximum reallocation count\x00".as_ptr() as *const c_char,
        );
    }
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"doce\x00".as_ptr() as *const c_char,
    );
    if dummy_handler_flags != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            8997i32,
            b"Element handler not fired\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_realloc_nested_groups() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_alloc_realloc_nested_groups\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9001,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ELEMENT doc (e,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?))))))))))))))))))))))))))))))))>\n<!ELEMENT e EMPTY>]>\n<doc><e/></doc>\x00".as_ptr() as *const c_char;
    let mut storage: crate::chardata::CharData = crate::chardata::CharData {
        count: 0,
        data: [0; 2048],
    };
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        crate::chardata::CharData_Init(&mut storage as *mut _);
        XML_SetElementDeclHandler(
            g_parser,
            transmute(Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut XML_Content,
                    ) -> (),
            )),
        );
        XML_SetStartElementHandler(
            g_parser,
            Some(
                record_element_start_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetUserData(
            g_parser,
            &mut storage as *mut crate::chardata::CharData as *mut c_void,
        );
        dummy_handler_flags = 0;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9032i32,
            b"Parse succeeded despite failing reallocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9034i32,
            b"Parse failed at maximum reallocation count\x00".as_ptr() as *const c_char,
        );
    }
    crate::chardata::CharData_CheckXMLChars(
        &mut storage as *mut _,
        b"doce\x00".as_ptr() as *const c_char,
    );
    if dummy_handler_flags != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9037i32,
            b"Element handler not fired\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_large_group() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_alloc_large_group\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9041,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ELEMENT doc (a1|a2|a3|a4|a5|a6|a7|a8|b1|b2|b3|b4|b5|b6|b7|b8|c1|c2|c3|c4|c5|c6|c7|c8|d1|d2|d3|d4|d5|d6|d7|d8|e1)+>\n]>\n<doc>\n<a1/>\n</doc>\n\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 50;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetElementDeclHandler(
            g_parser,
            transmute(Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut XML_Content,
                    ) -> (),
            )),
        );
        dummy_handler_flags = 0;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9069i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9071i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    }
    if dummy_handler_flags != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9073i32,
            b"Element handler flag not raised\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_realloc_group_choice() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_alloc_realloc_group_choice\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9077,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n<!ELEMENT doc (a1|a2|a3|a4|a5|a6|a7|a8|b1|b2|b3|b4|b5|b6|b7|b8|c1|c2|c3|c4|c5|c6|c7|c8|d1|d2|d3|d4|d5|d6|d7|d8|e1)+>\n]>\n<doc>\n<a1/>\n<b2 attr=\'foo\'>This is a foo</b2>\n<c3></c3>\n</doc>\n\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        XML_SetElementDeclHandler(
            g_parser,
            transmute(Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut XML_Content,
                    ) -> (),
            )),
        );
        dummy_handler_flags = 0;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9107i32,
            b"Parse succeeded despite failing reallocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9109i32,
            b"Parse failed at maximum reallocation count\x00".as_ptr() as *const c_char,
        );
    }
    if dummy_handler_flags != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9111i32,
            b"Element handler flag not raised\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_pi_in_epilog() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 24], &[c_char; 24]>(b"test_alloc_pi_in_epilog\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9115,
    );
    let mut text: *const c_char = b"<doc></doc>\n<?pi in epilog?>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 15;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetProcessingInstructionHandler(
            g_parser,
            Some(
                dummy_pi_handler
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> (),
            ),
        );
        dummy_handler_flags = 0;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9133i32,
            b"Parse completed despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9135i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    }
    if dummy_handler_flags != DUMMY_PI_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9137i32,
            b"Processing instruction handler not invoked\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_comment_in_epilog() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_alloc_comment_in_epilog\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9141,
    );
    let mut text: *const c_char =
        b"<doc></doc>\n<!-- comment in epilog -->\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 15;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetCommentHandler(
            g_parser,
            Some(
                dummy_comment_handler
                    as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
            ),
        );
        dummy_handler_flags = 0;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9159i32,
            b"Parse completed despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9161i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    }
    if dummy_handler_flags != DUMMY_COMMENT_HANDLER_FLAG {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9163i32,
            b"Processing instruction handler not invoked\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_realloc_long_attribute_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 40], &[c_char; 40]>(
            b"test_alloc_realloc_long_attribute_value\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9167,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [<!ENTITY foo \'This entity will be substituted as an attribute value, and is   calculated to be exactly long enough that the terminating NUL   that the library adds internally will trigger the string pool togrow. GHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\'>]>\n<doc a=\'&foo;\'></doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9202i32,
            b"Parse succeeded despite failing reallocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9204i32,
            b"Parse failed at maximum reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_attribute_whitespace() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_alloc_attribute_whitespace\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9208,
    );
    let mut text: *const c_char = b"<doc a=\' \'></doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 15;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9223i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9225i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_attribute_predefined_entity() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 39], &[c_char; 39]>(
            b"test_alloc_attribute_predefined_entity\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9229,
    );
    let mut text: *const c_char = b"<doc a=\'&amp;\'></doc>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 15;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9244i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9246i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test that a character reference at the end of a suitably long
 * default value for an attribute can trigger pool growth, and recovers
 * if the allocator fails on it.
 */

unsafe extern "C" fn test_alloc_long_attr_default_with_char_ref() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 43], &[c_char; 43]>(
            b"test_alloc_long_attr_default_with_char_ref\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9254,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [<!ATTLIST doc a CDATA \'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHI&#x31;\'>]>\n<doc/>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 20;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9289i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9291i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test that a long character reference substitution triggers a pool
 * expansion correctly for an attribute value.
 */

unsafe extern "C" fn test_alloc_long_attr_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_alloc_long_attr_value\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9298,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE test [<!ENTITY foo \'\nABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\'>]>\n<test a=\'&foo;\'/>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 25;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9333i32,
            b"Parse succeeded despite failing allocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9335i32,
            b"Parse failed at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test that an error in a nested parameter entity substitution is
 * handled correctly.  It seems unlikely that the code path being
 * exercised can be reached purely by carefully crafted XML, but an
 * allocation error in the right place will definitely do it.
 */

unsafe extern "C" fn test_alloc_nested_entities() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"test_alloc_nested_entities\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9344,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'http://example.org/one.ent\'>\n<doc />\x00".as_ptr()
            as *const c_char;
    let mut test_data: ExtFaults = {
        let mut init =
                ext_faults{parse_text:
                               
                               b"<!ENTITY % pe1 \'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\'>\n<!ENTITY % pe2 \'%pe1;\'>\n%pe2;\x00".as_ptr() as *const c_char,
                           fail_text:
                               
                               b"Memory Fail not faulted\x00".as_ptr() as
                                   *const c_char,
                           encoding: ::rexpat::stddef_h::NULL as *const XML_Char,
                           error: XML_ERROR_NO_MEMORY,};
        init
    };
    /* Causes an allocation error in a nested storeEntityValue() */
    allocation_count = 12;
    XML_SetUserData(g_parser, &mut test_data as *mut ExtFaults as *mut c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    _: XML_Parser,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                    _: *const XML_Char,
                ) -> c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Entity allocation failure not noted\x00".as_ptr() as *const c_char,
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9377,
    );
}

unsafe extern "C" fn test_alloc_realloc_param_entity_newline() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 40], &[c_char; 40]>(
            b"test_alloc_realloc_param_entity_newline\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9381,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'http://example.org/\'>\n<doc/>\x00".as_ptr() as *const c_char;
    let mut dtd_text: [c_char; 1048] =
        *::std::mem::transmute::<&[u8; 1048],
                                 &mut [c_char; 1048]>(b"<!ENTITY % pe \'<!ATTLIST doc att CDATA \"This default value is carefully crafted so that the carriage    return right at the end of the entity string causes an internal string pool to have to grow.  This allows us to test the alloc  failure path from that point. OPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDE\">\n\'>%pe;\n\x00");
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 5;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        XML_SetUserData(g_parser, dtd_text.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_alloc
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9421i32,
            b"Parse succeeded despite failing reallocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9423i32,
            b"Parse failed at maximum reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_realloc_ce_extends_pe() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_alloc_realloc_ce_extends_pe\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9427,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc SYSTEM \'http://example.org/\'>\n<doc/>\x00".as_ptr() as *const c_char;
    let mut dtd_text: [c_char; 1056] =
        *::std::mem::transmute::<&[u8; 1056],
                                 &mut [c_char; 1056]>(b"<!ENTITY % pe \'<!ATTLIST doc att CDATA \"This default value is carefully crafted so that the character   entity at the end causes an internal string pool to have to     grow.  This allows us to test the allocation failure path from  that point onwards. EFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFG&#x51;\">\n\'>%pe;\n\x00");
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 5;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        XML_SetUserData(g_parser, dtd_text.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_alloc
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9467i32,
            b"Parse succeeded despite failing reallocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9469i32,
            b"Parse failed at maximum reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_realloc_attributes() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_alloc_realloc_attributes\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9473,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ATTLIST doc\n    a1  (a|b|c)   \'a\'\n    a2  (foo|bar) #IMPLIED\n    a3  NMTOKEN   #IMPLIED\n    a4  NMTOKENS  #IMPLIED\n    a5  ID        #IMPLIED\n    a6  IDREF     #IMPLIED\n    a7  IDREFS    #IMPLIED\n    a8  ENTITY    #IMPLIED\n    a9  ENTITIES  #IMPLIED\n    a10 CDATA     #IMPLIED\n  >]>\n<doc>wombat</doc>\n\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 5;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9502i32,
            b"Parse succeeded despite failing reallocator\x00".as_ptr() as *const c_char,
        );
    }
    if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9504i32,
            b"Parse failed at maximum reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_long_doc_name() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_alloc_long_doc_name\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9508,
    );
    let mut text: *const c_char =
        
        b"<LongRootElementNameThatWillCauseTheNextAllocationToExpandTheStringPoolForTheDTDQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ a=\'1\'/>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 20;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9541i32,
            b"Parsing worked despite failing reallocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9543i32,
            b"Parsing failed even at max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_long_base() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 21], &[c_char; 21]>(b"test_alloc_long_base\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9547,
    );
    let mut text: *const c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY e SYSTEM \'foo\'>\n]>\n<doc>&e;</doc>\x00".as_ptr()
            as *const c_char;
    let mut entity_text: [c_char; 12] =
        *::std::mem::transmute::<&[u8; 12], &mut [c_char; 12]>(b"Hello world\x00");
    let mut base: *const XML_Char =
        
        b"LongBaseURI/that/will/overflow/an/internal/buffer/and/cause/it/to/have/to/grow/PQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/\x00".as_ptr() as *const c_char;
    /* clang-format on */
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 25;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetUserData(g_parser, entity_text.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_alloc
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if XML_SetBase(g_parser, base) == XML_STATUS_ERROR_0 as c_uint {
            XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        } else {
            if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
                != XML_STATUS_ERROR_0 as c_uint
            {
                break;
            }
            /* See comment in test_alloc_parse_xdecl() */
            alloc_teardown();
            alloc_setup();
        }
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9593i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9595i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_long_public_id() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_alloc_long_public_id\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9599,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY e PUBLIC \'LongPublicIDThatShouldResultInAnInternalStringPoolGrowingAtASpecificMomentKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB\' \'bar\'>\n]>\n<doc>&e;</doc>\x00".as_ptr() as *const c_char;
    let mut entity_text: [c_char; 12] =
        *::std::mem::transmute::<&[u8; 12], &mut [c_char; 12]>(b"Hello world\x00");
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 40;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetUserData(g_parser, entity_text.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_alloc
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9640i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9642i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_long_entity_value() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 29], &[c_char; 29]>(b"test_alloc_long_entity_value\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9646,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ENTITY e1 \'Long entity value that should provoke a string pool to grow while setting up to parse the external entity below. xyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB\'>\n  <!ENTITY e2 SYSTEM \'bar\'>\n]>\n<doc>&e2;</doc>\x00".as_ptr() as *const c_char;
    let mut entity_text: [c_char; 12] =
        *::std::mem::transmute::<&[u8; 12], &mut [c_char; 12]>(b"Hello world\x00");
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 40;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetUserData(g_parser, entity_text.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_alloc
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9688i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9690i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_alloc_long_notation() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_alloc_long_notation\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9694,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!NOTATION note SYSTEM \'ALongNotationNameThatShouldProvokeStringPoolGrowthWhileCallingAnExternalEntityParserUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB\'>\n  <!ENTITY e1 SYSTEM \'foo\' NDATA ALongNotationNameThatShouldProvokeStringPoolGrowthWhileCallingAnExternalEntityParserUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB>\n  <!ENTITY e2 SYSTEM \'bar\'>\n]>\n<doc>&e2;</doc>\x00".as_ptr() as *const c_char;
    let mut options: [ExtOption; 3] = [
        {
            let mut init = ExtOption {
                system_id: b"foo\x00".as_ptr() as *const c_char,
                parse_text: b"Entity Foo\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: b"bar\x00".as_ptr() as *const c_char,
                parse_text: b"Entity Bar\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: ::rexpat::stddef_h::NULL as *const XML_Char,
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
            };
            init
        },
    ];
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 40;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetUserData(g_parser, options.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_optioner
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_alloc_parse_xdecl() */
        alloc_teardown();
        alloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9757i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9759i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn nsalloc_setup() {
    ALLOCATOR_MODE = AllocatorMode::Duff;
    let mut ns_sep: [XML_Char; 2] = [' ' as XML_Char, '\u{0}' as XML_Char];
    /* Ensure the parser creation will go through */
    allocation_count = ALLOC_ALWAYS_SUCCEED as intptr_t;
    reallocation_count = REALLOC_ALWAYS_SUCCEED as intptr_t;
    g_parser = XML_ParserCreate_MM(
        ::rexpat::stddef_h::NULL as *const XML_Char,
        None,
        ns_sep.as_mut_ptr(),
    );
    if g_parser.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9773i32,
            b"Parser not created\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn nsalloc_teardown() {
    basic_teardown();
}
/* Test the effects of allocation failure in simple namespace parsing.
 * Based on test_ns_default_with_empty_uri()
 */

unsafe extern "C" fn test_nsalloc_xmlns() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 19], &[c_char; 19]>(b"test_nsalloc_xmlns\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9784,
    );
    let mut text: *const c_char =
        b"<doc xmlns=\'http://example.org/\'>\n  <e xmlns=\'\'/>\n</doc>\x00".as_ptr()
            as *const c_char;
    let mut i: c_uint = 0;
    let max_alloc_count: c_uint = 30;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        /* Exercise more code paths with a default handler */
        XML_SetDefaultHandler(
            g_parser,
            Some(
                dummy_default_handler
                    as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* Resetting the parser is insufficient, because some memory
         * allocations are cached within the parser.  Instead we use
         * the teardown and setup routines to ensure that we have the
         * right sort of parser back in our hands.
         */
        nsalloc_teardown();
        nsalloc_setup();
        i = i.wrapping_add(1)
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9807i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9809i32,
            b"Parsing failed even at maximum allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test XML_ParseBuffer interface with namespace and a dicky allocator */

unsafe extern "C" fn test_nsalloc_parse_buffer() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_nsalloc_parse_buffer\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9814,
    );
    let mut text: *const c_char = b"<doc>Hello</doc>\x00".as_ptr() as *const c_char;
    let mut buffer: *mut c_void = 0 as *mut c_void;
    /* Try a parse before the start of the world */
    /* (Exercises new code path) */
    allocation_count = 0;
    if XML_ParseBuffer(g_parser, 0, XML_FALSE as c_int) != XML_STATUS_ERROR_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9822i32,
            b"Pre-init XML_ParseBuffer not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NO_MEMORY {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9824i32,
            b"Pre-init XML_ParseBuffer faulted for wrong reason\x00".as_ptr() as *const c_char,
        );
    }
    /* Now with actual memory allocation */
    allocation_count = ALLOC_ALWAYS_SUCCEED as intptr_t;
    if XML_ParseBuffer(g_parser, 0, XML_FALSE as c_int) != XML_STATUS_OK_0 as c_uint {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9829i32,
        );
    }
    /* Check that resuming an unsuspended parser is faulted */
    if XML_ResumeParser(g_parser) != XML_STATUS_ERROR_0 as c_uint {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9833i32,
            b"Resuming unsuspended parser not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NOT_SUSPENDED {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9835i32,
        );
    }
    /* Get the parser into suspended state */
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
        ),
    );
    resumable = XML_TRUE;
    buffer = XML_GetBuffer(g_parser, strlen(text) as c_int);
    if buffer.is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9842i32,
            b"Could not acquire parse buffer\x00".as_ptr() as *const c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9843u32,
            (*::std::mem::transmute::<&[u8; 37], &[c_char; 37]>(
                b"void test_nsalloc_parse_buffer(void)\x00",
            ))
            .as_ptr(),
        );
    }
    memcpy(buffer, text as *const c_void, strlen(text));
    if XML_ParseBuffer(g_parser, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_SUSPENDED_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9847i32,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NONE {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9849i32,
        );
    }
    if XML_ParseBuffer(g_parser, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9852i32,
            b"Suspended XML_ParseBuffer not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_SUSPENDED {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9854i32,
        );
    }
    if !XML_GetBuffer(g_parser, strlen(text) as c_int).is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9856i32,
            b"Suspended XML_GetBuffer not faulted\x00".as_ptr() as *const c_char,
        );
    }
    /* Get it going again and complete the world */
    XML_SetCharacterDataHandler(
        g_parser,
        ::std::mem::transmute::<libc::intptr_t, XML_CharacterDataHandler>(
            ::rexpat::stddef_h::NULL as libc::intptr_t,
        ),
    );
    if XML_ResumeParser(g_parser) != XML_STATUS_OK_0 as c_uint {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9861i32,
        );
    }
    if XML_ParseBuffer(g_parser, strlen(text) as c_int, XML_TRUE as c_int)
        != XML_STATUS_ERROR_0 as c_uint
    {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9864i32,
            b"Post-finishing XML_ParseBuffer not faulted\x00".as_ptr() as *const c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_FINISHED {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9866i32,
        );
    }
    if !XML_GetBuffer(g_parser, strlen(text) as c_int).is_null() {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9868i32,
            b"Post-finishing XML_GetBuffer not faulted\x00".as_ptr() as *const c_char,
        );
    };
}
/* Check handling of long prefix names (pool growth) */

unsafe extern "C" fn test_nsalloc_long_prefix() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"test_nsalloc_long_prefix\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9873,
    );
    let mut text: *const c_char =
        
        b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ=\'http://example.org/\'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 40;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9942i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            9944i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Check handling of long uri names (pool growth) */

unsafe extern "C" fn test_nsalloc_long_uri() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"test_nsalloc_long_uri\x00")).as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        9949,
    );
    let mut text: *const c_char =
        
        b"<foo:e xmlns:foo=\'http://example.org/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/\' bar:a=\'12\'\nxmlns:bar=\'http://example.org/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/\'></foo:e>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 40;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10002i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10004i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test handling of long attribute names with prefixes */

unsafe extern "C" fn test_nsalloc_long_attr() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"test_nsalloc_long_attr\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10009,
    );
    let mut text: *const c_char =
        
        b"<foo:e xmlns:foo=\'http://example.org/\' bar:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ=\'12\'\nxmlns:bar=\'http://example.org/\'></foo:e>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 40;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10045i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10047i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test handling of an attribute name with a long namespace prefix */

unsafe extern "C" fn test_nsalloc_long_attr_prefix() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_nsalloc_long_attr_prefix\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10052,
    );
    let mut text: *const c_char =
        
        b"<foo:e xmlns:foo=\'http://example.org/\' ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:a=\'12\'\nxmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ=\'http://example.org/\'></foo:e>\x00".as_ptr() as *const c_char;
    let mut elemstr: [*const XML_Char; 2] =
        [b"http://example.org/ e foo\x00".as_ptr() as *const c_char,
         
         b"http://example.org/ a ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ\x00".as_ptr() as *const c_char];
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 40;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetReturnNSTriplet(g_parser, XML_TRUE);
        XML_SetUserData(g_parser, elemstr.as_mut_ptr() as *mut c_void);
        XML_SetElementHandler(
            g_parser,
            Some(
                triplet_start_checker
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut *const XML_Char,
                    ) -> (),
            ),
            Some(
                triplet_end_checker
                    as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10131i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10133i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test attribute handling in the face of a dodgy reallocator */

unsafe extern "C" fn test_nsalloc_realloc_attributes() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 32], &[c_char; 32]>(
            b"test_nsalloc_realloc_attributes\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10138,
    );
    let mut text: *const c_char =
        
        b"<foo:e xmlns:foo=\'http://example.org/\' bar:a=\'12\'\n       xmlns:bar=\'http://example.org/\'></foo:e>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        // FIXME: `HashSet` doesn't use `realloc`
        allocation_count = 3 * i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10155i32,
            b"Parsing worked despite failing reallocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10157i32,
            b"Parsing failed at max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test long element names with namespaces under a failing allocator */

unsafe extern "C" fn test_nsalloc_long_element() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_nsalloc_long_element\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10162,
    );
    let mut text: *const c_char =
        
        b"<foo:thisisalongenoughelementnametotriggerareallocation\n xmlns:foo=\'http://example.org/\' bar:a=\'12\'\n xmlns:bar=\'http://example.org/\'></foo:thisisalongenoughelementnametotriggerareallocation>\x00".as_ptr() as *const c_char;
    let mut elemstr: [*const XML_Char; 2] = [
        b"http://example.org/ thisisalongenoughelementnametotriggerareallocation foo\x00".as_ptr()
            as *const c_char,
        b"http://example.org/ a bar\x00".as_ptr() as *const c_char,
    ];
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 30;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetReturnNSTriplet(g_parser, XML_TRUE);
        XML_SetUserData(g_parser, elemstr.as_mut_ptr() as *mut c_void);
        XML_SetElementHandler(
            g_parser,
            Some(
                triplet_start_checker
                    as unsafe extern "C" fn(
                        _: *mut c_void,
                        _: *const XML_Char,
                        _: *mut *const XML_Char,
                    ) -> (),
            ),
            Some(
                triplet_end_checker
                    as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10188i32,
            b"Parsing worked despite failing reallocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10190i32,
            b"Parsing failed at max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test the effects of reallocation failure when reassigning a
 * binding.
 *
 * XML_ParserReset does not free the BINDING structures used by a
 * parser, but instead adds them to an internal free list to be reused
 * as necessary.  Likewise the URI buffers allocated for the binding
 * aren't freed, but kept attached to their existing binding.  If the
 * new binding has a longer URI, it will need reallocation.  This test
 * provokes that reallocation, and tests the control path if it fails.
 */

unsafe extern "C" fn test_nsalloc_realloc_binding_uri() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_nsalloc_realloc_binding_uri\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10204,
    );
    let mut first: *const c_char =
        b"<doc xmlns=\'http://example.org/\'>\n  <e xmlns=\'\' />\n</doc>\x00".as_ptr()
            as *const c_char;
    let mut second: *const c_char =
        
        b"<doc xmlns=\'http://example.org/long/enough/URI/to/reallocate/\'>\n  <e xmlns=\'\' />\n</doc>\x00".as_ptr() as *const c_char;
    let mut i: c_uint = 0;
    let max_realloc_count: c_uint = 10;
    /* First, do a full parse that will leave bindings around */
    if _XML_Parse_SINGLE_BYTES(g_parser, first, strlen(first) as c_int, XML_TRUE)
        == XML_STATUS_ERROR_0 as c_uint
    {
        _xml_failure(
            g_parser,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10218i32,
        );
    }
    /* Now repeat with a longer URI and a duff reallocator */
    i = 0;
    while i < max_realloc_count {
        XML_ParserReset(g_parser, ::rexpat::stddef_h::NULL as *const XML_Char);
        reallocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, second, strlen(second) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10229i32,
            b"Parsing worked despite failing reallocation\x00".as_ptr() as *const c_char,
        );
    } else if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10231i32,
            b"Parsing failed at max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Check handling of long prefix names (pool growth) */

unsafe extern "C" fn test_nsalloc_realloc_long_prefix() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_nsalloc_realloc_long_prefix\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10236,
    );
    let mut text: *const c_char =
        
        b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ=\'http://example.org/\'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 12;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10305i32,
            b"Parsing worked despite failing reallocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10307i32,
            b"Parsing failed even at max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Check handling of even long prefix names (different code path) */

unsafe extern "C" fn test_nsalloc_realloc_longer_prefix() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 35], &[c_char; 35]>(
            b"test_nsalloc_realloc_longer_prefix\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10312,
    );
    let mut text: *const c_char =
        
        b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ=\'http://example.org/\'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ:foo>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 12;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10381i32,
            b"Parsing worked despite failing reallocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10383i32,
            b"Parsing failed even at max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_nsalloc_long_namespace() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 28], &[c_char; 28]>(b"test_nsalloc_long_namespace\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10387,
    );
    let mut text1: *const c_char =
        
        b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:e xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ=\'http://example.org/\'>\n\x00".as_ptr() as *const c_char;
    let mut text2: *const c_char =
        
        b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:f ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:attr=\'foo\'/>\n</ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:e>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 40;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text1, strlen(text1) as c_int, XML_FALSE)
            != XML_STATUS_ERROR_0 as c_uint
            && _XML_Parse_SINGLE_BYTES(g_parser, text2, strlen(text2) as c_int, XML_TRUE)
                != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10495i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10497i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Using a slightly shorter namespace name provokes allocations in
 * slightly different places in the code.
 */

unsafe extern "C" fn test_nsalloc_less_long_namespace() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_nsalloc_less_long_namespace\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10504,
    );
    let mut text: *const c_char =
        
        b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:e xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678=\'http://example.org/\'>\n<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:f ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:att=\'foo\'/>\n</ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:e>\x00".as_ptr() as *const c_char;
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 40;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10568i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10570i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_nsalloc_long_context() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"test_nsalloc_long_context\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10574,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc SYSTEM \'foo\' [\n  <!ATTLIST doc baz ID #REQUIRED>\n  <!ENTITY en SYSTEM \'bar\'>\n]>\n<doc xmlns=\'http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKL\' baz=\'2\'>\n&en;</doc>\x00".as_ptr() as *const c_char;
    let mut options: [ExtOption; 3] = [
        {
            let mut init = ExtOption {
                system_id: b"foo\x00".as_ptr() as *const c_char,
                parse_text: b"<!ELEMENT e EMPTY>\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: b"bar\x00".as_ptr() as *const c_char,
                parse_text: b"<e/>\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: ::rexpat::stddef_h::NULL as *const XML_Char,
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
            };
            init
        },
    ];
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 70;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetUserData(g_parser, options.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_optioner
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10620i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10622i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* This function is void; it will throw a fail() on error, so if it
 * returns normally it must have succeeded.
 */

unsafe extern "C" fn context_realloc_test(mut text: *const c_char) {
    let mut options: [ExtOption; 3] = [
        {
            let mut init = ExtOption {
                system_id: b"foo\x00".as_ptr() as *const c_char,
                parse_text: b"<!ELEMENT e EMPTY>\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: b"bar\x00".as_ptr() as *const c_char,
                parse_text: b"<e/>\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: ::rexpat::stddef_h::NULL as *const XML_Char,
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
            };
            init
        },
    ];
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 6;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        XML_SetUserData(g_parser, options.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_optioner
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10649i32,
            b"Parsing worked despite failing reallocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10651i32,
            b"Parsing failed even at max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_nsalloc_realloc_long_context() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 34], &[c_char; 34]>(
            b"test_nsalloc_realloc_long_context\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10654,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc SYSTEM \'foo\' [\n  <!ENTITY en SYSTEM \'bar\'>\n]>\n<doc xmlns=\'http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKL\'>\n&en;</doc>\x00".as_ptr() as *const c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_2() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_nsalloc_realloc_long_context_2\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10685,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc SYSTEM \'foo\' [\n  <!ENTITY en SYSTEM \'bar\'>\n]>\n<doc xmlns=\'http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJK\'>\n&en;</doc>\x00".as_ptr() as *const c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_3() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_nsalloc_realloc_long_context_3\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10716,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc SYSTEM \'foo\' [\n  <!ENTITY en SYSTEM \'bar\'>\n]>\n<doc xmlns=\'http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGH\'>\n&en;</doc>\x00".as_ptr() as *const c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_4() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_nsalloc_realloc_long_context_4\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10747,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc SYSTEM \'foo\' [\n  <!ENTITY en SYSTEM \'bar\'>\n]>\n<doc xmlns=\'http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO\'>\n&en;</doc>\x00".as_ptr() as *const c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_5() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_nsalloc_realloc_long_context_5\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10778,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc SYSTEM \'foo\' [\n  <!ENTITY en SYSTEM \'bar\'>\n]>\n<doc xmlns=\'http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABC\'>\n&en;</doc>\x00".as_ptr() as *const c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_6() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_nsalloc_realloc_long_context_6\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10809,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc SYSTEM \'foo\' [\n  <!ENTITY en SYSTEM \'bar\'>\n]>\n<doc xmlns=\'http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOP\'>\n&en;</doc>\x00".as_ptr() as *const c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_7() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 36], &[c_char; 36]>(
            b"test_nsalloc_realloc_long_context_7\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10839,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc SYSTEM \'foo\' [\n  <!ENTITY en SYSTEM \'bar\'>\n]>\n<doc xmlns=\'http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLM\'>\n&en;</doc>\x00".as_ptr() as *const c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_ge_name() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 34], &[c_char; 34]>(
            b"test_nsalloc_realloc_long_ge_name\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10870,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc SYSTEM \'foo\' [\n  <!ENTITY ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP SYSTEM \'bar\'>\n]>\n<doc xmlns=\'http://example.org/baz\'>\n&ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP;</doc>\x00".as_ptr() as *const c_char;
    let mut options: [ExtOption; 3] = [
        {
            let mut init = ExtOption {
                system_id: b"foo\x00".as_ptr() as *const c_char,
                parse_text: b"<!ELEMENT el EMPTY>\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: b"bar\x00".as_ptr() as *const c_char,
                parse_text: b"<el/>\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: ::rexpat::stddef_h::NULL as *const XML_Char,
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
            };
            init
        },
    ];
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 10;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        XML_SetUserData(g_parser, options.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_optioner
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10932i32,
            b"Parsing worked despite failing reallocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            10934i32,
            b"Parsing failed even at max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test that when a namespace is passed through the context mechanism
 * to an external entity parser, the parsers handle reallocation
 * failures correctly.  The prefix is exactly the right length to
 * provoke particular uncommon code paths.
 */

unsafe extern "C" fn test_nsalloc_realloc_long_context_in_dtd() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 41], &[c_char; 41]>(
            b"test_nsalloc_realloc_long_context_in_dtd\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        10943,
    );
    let mut text1: *const c_char =
        
        b"<!DOCTYPE ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:doc [\n  <!ENTITY First SYSTEM \'foo/First\'>\n]>\n<ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:doc xmlns:ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP=\'foo/Second\'>&First;\x00".as_ptr() as *const c_char;
    let mut text2: *const c_char =
        
        b"</ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:doc>\x00".as_ptr() as *const c_char;
    let mut options: [ExtOption; 2] = [
        {
            let mut init = ExtOption {
                system_id: b"foo/First\x00".as_ptr() as *const c_char,
                parse_text: b"Hello world\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: ::rexpat::stddef_h::NULL as *const XML_Char,
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
            };
            init
        },
    ];
    let mut i: c_int = 0;
    let max_realloc_count: c_int = 20;
    i = 0;
    while i < max_realloc_count {
        reallocation_count = i as intptr_t;
        XML_SetUserData(g_parser, options.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_optioner
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text1, strlen(text1) as c_int, XML_FALSE)
            != XML_STATUS_ERROR_0 as c_uint
            && _XML_Parse_SINGLE_BYTES(g_parser, text2, strlen(text2) as c_int, XML_TRUE)
                != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            11040i32,
            b"Parsing worked despite failing reallocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_realloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            11042i32,
            b"Parsing failed even at max reallocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_nsalloc_long_default_in_ext() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 33], &[c_char; 33]>(
            b"test_nsalloc_long_default_in_ext\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        11046,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc [\n  <!ATTLIST e a1 CDATA \'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\'>\n  <!ENTITY x SYSTEM \'foo\'>\n]>\n<doc>&x;</doc>\x00".as_ptr() as *const c_char;
    let mut options: [ExtOption; 2] = [
        {
            let mut init = ExtOption {
                system_id: b"foo\x00".as_ptr() as *const c_char,
                parse_text: b"<e/>\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: ::rexpat::stddef_h::NULL as *const XML_Char,
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
            };
            init
        },
    ];
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 50;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetUserData(g_parser, options.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_optioner
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            11089i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            11091i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn test_nsalloc_long_systemid_in_ext() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 34], &[c_char; 34]>(
            b"test_nsalloc_long_systemid_in_ext\x00",
        ))
        .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        11095,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE doc SYSTEM \'foo\' [\n  <!ENTITY en SYSTEM \'ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/\'>\n]>\n<doc>&en;</doc>\x00".as_ptr() as *const c_char;
    let mut options: [ExtOption; 3] = [
        {
            let mut init = ExtOption {
                system_id: b"foo\x00".as_ptr() as *const c_char,
                parse_text: b"<!ELEMENT e EMPTY>\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init =
                 ExtOption{system_id:
                               
                               b"ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/\x00".as_ptr() as *const c_char,
                           parse_text:
                               
                               b"<e/>\x00".as_ptr() as
                                   *const c_char,};
            init
        },
        {
            let mut init = ExtOption {
                system_id: ::rexpat::stddef_h::NULL as *const XML_Char,
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
            };
            init
        },
    ];
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 55;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetUserData(g_parser, options.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_optioner
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            11158i32,
            b"Parsing worked despite failing allocations\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            11160i32,
            b"Parsing failed even at max allocation count\x00".as_ptr() as *const c_char,
        );
    };
}
/* Test the effects of allocation failure on parsing an element in a
 * namespace.  Based on test_nsalloc_long_context.
 */

unsafe extern "C" fn test_nsalloc_prefixed_element() {
    crate::minicheck::_check_set_test_info(
        (*::std::mem::transmute::<&[u8; 30], &[c_char; 30]>(b"test_nsalloc_prefixed_element\x00"))
            .as_ptr(),
        b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
            as *const c_char,
        11167,
    );
    let mut text: *const c_char =
        
        b"<!DOCTYPE pfx:element SYSTEM \'foo\' [\n  <!ATTLIST pfx:element baz ID #REQUIRED>\n  <!ENTITY en SYSTEM \'bar\'>\n]>\n<pfx:element xmlns:pfx=\'http://example.org/\' baz=\'2\'>\n&en;</pfx:element>\x00".as_ptr() as *const c_char;
    let mut options: [ExtOption; 3] = [
        {
            let mut init = ExtOption {
                system_id: b"foo\x00".as_ptr() as *const c_char,
                parse_text: b"<!ELEMENT e EMPTY>\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: b"bar\x00".as_ptr() as *const c_char,
                parse_text: b"<e/>\x00".as_ptr() as *const c_char,
            };
            init
        },
        {
            let mut init = ExtOption {
                system_id: ::rexpat::stddef_h::NULL as *const XML_Char,
                parse_text: ::rexpat::stddef_h::NULL as *const c_char,
            };
            init
        },
    ];
    let mut i: c_int = 0;
    let max_alloc_count: c_int = 70;
    i = 0;
    while i < max_alloc_count {
        allocation_count = i as intptr_t;
        XML_SetUserData(g_parser, options.as_mut_ptr() as *mut c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_optioner
                    as unsafe extern "C" fn(
                        _: XML_Parser,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                        _: *const XML_Char,
                    ) -> c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(g_parser, text, strlen(text) as c_int, XML_TRUE)
            != XML_STATUS_ERROR_0 as c_uint
        {
            break;
        }
        /* See comment in test_nsalloc_xmlns() */
        nsalloc_teardown();
        nsalloc_setup();
        i += 1
    }
    if i == 0 {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            11194i32,
            b"Success despite failing allocator\x00".as_ptr() as *const c_char,
        );
    } else if i == max_alloc_count {
        crate::minicheck::_fail_unless(
            0i32,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/tests/runtests.c\x00".as_ptr()
                as *const c_char,
            11196i32,
            b"Failed even at full allocation count\x00".as_ptr() as *const c_char,
        );
    };
}

unsafe extern "C" fn make_suite() -> *mut crate::minicheck::Suite {
    let mut s: *mut crate::minicheck::Suite =
        crate::minicheck::suite_create(b"basic\x00".as_ptr() as *const c_char);
    let mut tc_basic: *mut crate::minicheck::TCase =
        crate::minicheck::tcase_create(b"basic tests\x00".as_ptr() as *const c_char);
    let mut tc_namespace: *mut crate::minicheck::TCase =
        crate::minicheck::tcase_create(b"XML namespaces\x00".as_ptr() as *const c_char);
    let mut tc_misc: *mut crate::minicheck::TCase =
        crate::minicheck::tcase_create(b"miscellaneous tests\x00".as_ptr() as *const c_char);
    let mut tc_alloc: *mut crate::minicheck::TCase =
        crate::minicheck::tcase_create(b"allocation tests\x00".as_ptr() as *const c_char);
    let mut tc_nsalloc: *mut crate::minicheck::TCase =
        crate::minicheck::tcase_create(b"namespace allocation tests\x00".as_ptr() as *const c_char);
    crate::minicheck::suite_add_tcase(s, tc_basic);
    crate::minicheck::tcase_add_checked_fixture(
        tc_basic,
        Some(basic_setup as unsafe extern "C" fn() -> ()),
        Some(basic_teardown as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_nul_byte as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_u0000_char as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_siphash_self as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_siphash_spec as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bom_utf8 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bom_utf16_be as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bom_utf16_le as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_nobom_utf16_le as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_illegal_utf8 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf8_auto_align as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(tc_basic, Some(test_utf16 as unsafe extern "C" fn() -> ()));
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf16_le_epilog_newline as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_not_utf16 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_encoding as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_latin1_umlauts as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_long_utf8_character as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_long_latin1_attribute as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_long_ascii_attribute as unsafe extern "C" fn() -> ()),
    );
    /* Regression test for SF bug #491986. */
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_danish_latin1 as unsafe extern "C" fn() -> ()),
    );
    /* Regression test for SF bug #514281. */
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_french_charref_hexidecimal as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_french_charref_decimal as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_french_latin1 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_french_utf8 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf8_false_rejection as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_line_number_after_parse as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_column_number_after_parse as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_line_and_column_numbers_inside_handlers as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_line_number_after_error as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_column_number_after_error as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_really_long_lines as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_really_long_encoded_lines as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_end_element_events as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_attr_whitespace_normalization as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_xmldecl_misplaced as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_xmldecl_invalid as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_xmldecl_missing_attr as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_xmldecl_missing_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_internal_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unrecognised_encoding_internal_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_wfc_undeclared_entity_unread_external_subset as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_wfc_undeclared_entity_no_external_subset as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_wfc_undeclared_entity_standalone as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_wfc_undeclared_entity_with_external_subset as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_not_standalone_handler_reject as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_not_standalone_handler_accept as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(
            test_wfc_undeclared_entity_with_external_subset_standalone
                as unsafe extern "C" fn() -> (),
        ),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_entity_with_external_subset_unless_standalone as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_wfc_no_recursive_entity_refs as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_set_encoding as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_no_handler as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_set_bom as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_bad_encoding as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_bad_encoding_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_invalid_parse as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_invalid_suspended_parse as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_dtd_default_handling as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_dtd_attr_handling as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_empty_ns_without_namespaces as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ns_in_attribute_default_without_namespaces as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_stop_parser_between_char_data_calls as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_suspend_parser_between_char_data_calls as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_repeated_stop_parser_between_char_data_calls as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_good_cdata_ascii as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_good_cdata_utf16 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_good_cdata_utf16_le as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_long_cdata_utf16 as unsafe extern "C" fn() -> ()),
    );
    /* FIXME workaround -DXML_MIN_SIZE + ASan (issue #332) */
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_multichar_cdata_utf16 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf16_bad_surrogate_pair as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_cdata as unsafe extern "C" fn() -> ()),
    );
    /* FIXME workaround -DXML_MIN_SIZE + ASan (issue #332) */
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_cdata_utf16 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_stop_parser_between_cdata_calls as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_suspend_parser_between_cdata_calls as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_memory_allocation as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_default_current as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_dtd_elements as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_set_foreign_dtd as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_foreign_dtd_not_standalone as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_invalid_foreign_dtd as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_foreign_dtd_with_doctype as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_foreign_dtd_without_external_subset as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_empty_foreign_dtd as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_set_base as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_attributes as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_reset_in_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_resume_invalid_parse as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_resume_resuspended as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_cdata_default as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_subordinate_reset as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_subordinate_suspend as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_subordinate_xdecl_suspend as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_subordinate_xdecl_abort as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_explicit_encoding as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_trailing_cr as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_trailing_cr as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_trailing_rsqb as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_trailing_rsqb as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_good_cdata as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_user_parameters as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_ref_parameter as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_empty_parse as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_get_buffer_1 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_get_buffer_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_byte_info_at_end as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_byte_info_at_error as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_byte_info_at_cdata as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_predefined_entities as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_invalid_tag_in_dtd as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_not_predefined_entities as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ignore_section as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ignore_section_utf16 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ignore_section_utf16_be as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_ignore_section as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_external_entity_values as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_not_standalone as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_value_abort as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_public_doctype as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_attribute_enum_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_predefined_entity_redefinition as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_dtd_stop_processing as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_public_notation_no_sysid as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_nested_groups as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_group_choice as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_standalone_parameter_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_skipped_parameter_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_recursive_external_parameter_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_undefined_ext_entity_in_external_dtd as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_suspend_xdecl as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_abort_epilog as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_abort_epilog_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_suspend_epilog as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_suspend_in_sole_empty_tag as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unfinished_epilog as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_partial_char_in_epilog as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_hash_collision as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_suspend_resume_internal_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_resume_entity_with_syntax_error as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_suspend_resume_parameter_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_restart_on_error as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_reject_lt_in_attribute_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_reject_unfinished_param_in_att_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_trailing_cr_in_att_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_standalone_internal_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_skipped_external_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_skipped_null_loaded_ext_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_skipped_unloaded_ext_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_param_entity_with_trailing_cr as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_invalid_character_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_invalid_character_entity_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_invalid_character_entity_3 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_invalid_character_entity_4 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_pi_handled_in_default as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_comment_handled_in_default as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(tc_basic, Some(test_pi_yml as unsafe extern "C" fn() -> ()));
    crate::minicheck::tcase_add_test(tc_basic, Some(test_pi_xnl as unsafe extern "C" fn() -> ()));
    crate::minicheck::tcase_add_test(tc_basic, Some(test_pi_xmm as unsafe extern "C" fn() -> ()));
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf16_pi as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf16_be_pi as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf16_be_comment as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf16_le_comment as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_missing_encoding_conversion_fn as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_failing_encoding_conversion_fn as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_success as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_bad_name as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_bad_name_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_long_name_1 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_long_name_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_invalid_unknown_encoding as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_ascii_encoding_ok as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_ascii_encoding_fail as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_invalid_length as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_invalid_topbit as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_invalid_surrogate as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_invalid_high as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_invalid_attr_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_latin1_utf16le_bom as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_latin1_utf16be_bom as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_latin1_utf16le_bom2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_latin1_utf16be_bom2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_utf16_be as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_utf16_le as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_utf16_unknown as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_ext_entity_utf8_non_bom as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf8_in_cdata_section as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf8_in_cdata_section_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_trailing_spaces_in_elements as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf16_attribute as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf16_second_attr as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_attr_after_solidus as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_utf16_pe as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_attr_desc_keyword as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_attr_desc_keyword_utf16 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_doctype as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_doctype_utf16 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_doctype_plus as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_doctype_star as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_doctype_query as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_bad_ignore as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_entity_in_utf16_be_attr as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_entity_in_utf16_le_attr as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_entity_public_utf16_be as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_entity_public_utf16_le as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_short_doctype as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_short_doctype_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_short_doctype_3 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_long_doctype as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_entity_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_entity_3 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_entity_4 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_bad_notation as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_default_doctype_handler as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_basic,
        Some(test_empty_element_abort as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::suite_add_tcase(s, tc_namespace);
    crate::minicheck::tcase_add_checked_fixture(
        tc_namespace,
        Some(namespace_setup as unsafe extern "C" fn() -> ()),
        Some(namespace_teardown as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_return_ns_triplet as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_tagname_overwrite as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_tagname_overwrite_triplet as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_start_ns_clears_start_element as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_default_ns_from_ext_subset_and_ext_ge as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_prefix_with_empty_uri_1 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_prefix_with_empty_uri_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_prefix_with_empty_uri_3 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_prefix_with_empty_uri_4 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_unbound_prefix as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_default_with_empty_uri as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_duplicate_attrs_diff_prefixes as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_duplicate_hashes as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_unbound_prefix_on_attribute as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_unbound_prefix_on_element as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_parser_reset as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_long_element as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_mixed_prefix_atts as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_extend_uri_buffer as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_reserved_attributes as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_reserved_attributes_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_extremely_long_prefix as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_unknown_encoding_success as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_double_colon as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_double_colon_element as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_bad_attr_leafname as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_bad_element_leafname as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_utf16_leafname as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_utf16_element_leafname as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_utf16_doctype as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_invalid_doctype as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_namespace,
        Some(test_ns_double_colon_doctype as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::suite_add_tcase(s, tc_misc);
    crate::minicheck::tcase_add_checked_fixture(
        tc_misc,
        ::std::mem::transmute::<libc::intptr_t, tcase_setup_function>(
            ::rexpat::stddef_h::NULL as libc::intptr_t,
        ),
        Some(basic_teardown as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(test_misc_alloc_create_parser as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(test_misc_alloc_create_parser_with_encoding as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(test_misc_null_parser as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(test_misc_error_string as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(test_misc_version as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(test_misc_features as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(test_misc_attribute_leak as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(test_misc_utf16le as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(test_misc_stop_during_end_handler_issue_240_1 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(test_misc_stop_during_end_handler_issue_240_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_misc,
        Some(
            test_misc_deny_internal_entity_closing_doctype_issue_317
                as unsafe extern "C" fn() -> (),
        ),
    );
    crate::minicheck::suite_add_tcase(s, tc_alloc);
    crate::minicheck::tcase_add_checked_fixture(
        tc_alloc,
        Some(alloc_setup as unsafe extern "C" fn() -> ()),
        Some(alloc_teardown as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_parse_xdecl as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_parse_xdecl_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_parse_pi as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_parse_pi_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_parse_pi_3 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_parse_comment as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_parse_comment_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_create_external_parser as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_run_external_parser as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_dtd_copy_default_atts as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_external_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_ext_entity_set_encoding as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_internal_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_dtd_default_handling as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_explicit_encoding as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_set_base as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_buffer as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_ext_entity_realloc_buffer as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_many_attributes as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_public_entity_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_subst_public_entity_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_parse_public_doctype as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_parse_public_doctype_long_name as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_set_foreign_dtd as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_attribute_enum_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_attribute_enum_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_implied_attribute as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_default_attribute as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_notation as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_public_notation as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_system_notation as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_nested_groups as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_nested_groups as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_large_group as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_group_choice as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_pi_in_epilog as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_comment_in_epilog as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_long_attribute_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_attribute_whitespace as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_attribute_predefined_entity as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_long_attr_default_with_char_ref as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_long_attr_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_nested_entities as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_param_entity_newline as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_ce_extends_pe as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_realloc_attributes as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_long_doc_name as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_long_base as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_long_public_id as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_long_entity_value as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_alloc,
        Some(test_alloc_long_notation as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::suite_add_tcase(s, tc_nsalloc);
    crate::minicheck::tcase_add_checked_fixture(
        tc_nsalloc,
        Some(nsalloc_setup as unsafe extern "C" fn() -> ()),
        Some(nsalloc_teardown as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_xmlns as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_parse_buffer as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_long_prefix as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_long_uri as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_long_attr as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_long_attr_prefix as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_attributes as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_long_element as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_binding_uri as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_long_prefix as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_longer_prefix as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_long_namespace as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_less_long_namespace as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_long_context as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_long_context as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_long_context_2 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_long_context_3 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_long_context_4 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_long_context_5 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_long_context_6 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_long_context_7 as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_long_ge_name as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_realloc_long_context_in_dtd as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_long_default_in_ext as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_long_systemid_in_ext as unsafe extern "C" fn() -> ()),
    );
    crate::minicheck::tcase_add_test(
        tc_nsalloc,
        Some(test_nsalloc_prefixed_element as unsafe extern "C" fn() -> ()),
    );
    return s;
}

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    let argc: c_int = (args.len() - 1) as c_int; 
    let mut argv: *mut *mut c_char = args.as_mut_ptr();

    unsafe {
        let mut i: c_int = 1;
        let mut verbosity: c_int = CK_NORMAL;
        let mut s: *mut crate::minicheck::Suite = make_suite();
        let mut sr: *mut crate::minicheck::SRunner = crate::minicheck::srunner_create(s);
        /* run the tests for internal helper functions */
        testhelper_is_whitespace_normalized();
        while i < argc {
            let mut opt: *mut c_char = *argv.offset(i as isize);
            if strcmp(opt, b"-v\x00".as_ptr() as *const c_char) == 0
                || strcmp(opt, b"--verbose\x00".as_ptr() as *const c_char) == 0
            {
                verbosity = crate::minicheck::CK_VERBOSE
            } else if strcmp(opt, b"-q\x00".as_ptr() as *const c_char) == 0
                || strcmp(opt, b"--quiet\x00".as_ptr() as *const c_char) == 0
            {
                verbosity = CK_SILENT
            } else {
                fprintf(
                    stderr,
                    b"runtests: unknown option \'%s\'\n\x00".as_ptr() as *const c_char,
                    opt,
                );
                ::std::process::exit(2);
            }
            i += 1
        }
        if verbosity != CK_SILENT {
            printf(
                b"Expat version: %s\n\x00".as_ptr() as *const c_char,
                XML_ExpatVersion(),
            );
        }
        crate::minicheck::srunner_run_all(sr, verbosity);
        let nf = crate::minicheck::srunner_ntests_failed(sr);
        crate::minicheck::srunner_free(sr);
        ::std::process::exit(if nf == 0 { EXIT_SUCCESS } else { EXIT_FAILURE });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn runtests() {
        crate::main();
    }
}
