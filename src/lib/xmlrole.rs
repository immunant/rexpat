// =============== BEGIN xmlrole_h ================

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

use libc::{c_char, c_int, c_uint, c_ulong};
pub type C2RustUnnamed_0 = c_int;

pub const XML_ROLE_ERROR: C2RustUnnamed_0 = -1;
pub const XML_ROLE_NONE: C2RustUnnamed_0 = 0;
pub const XML_ROLE_XML_DECL: C2RustUnnamed_0 = 1;
pub const XML_ROLE_INSTANCE_START: C2RustUnnamed_0 = 2;
pub const XML_ROLE_DOCTYPE_NONE: C2RustUnnamed_0 = 3;
pub const XML_ROLE_DOCTYPE_NAME: C2RustUnnamed_0 = 4;
pub const XML_ROLE_DOCTYPE_SYSTEM_ID: C2RustUnnamed_0 = 5;
pub const XML_ROLE_DOCTYPE_PUBLIC_ID: C2RustUnnamed_0 = 6;
pub const XML_ROLE_DOCTYPE_INTERNAL_SUBSET: C2RustUnnamed_0 = 7;
pub const XML_ROLE_DOCTYPE_CLOSE: C2RustUnnamed_0 = 8;
pub const XML_ROLE_GENERAL_ENTITY_NAME: C2RustUnnamed_0 = 9;
pub const XML_ROLE_PARAM_ENTITY_NAME: C2RustUnnamed_0 = 10;
pub const XML_ROLE_ENTITY_NONE: C2RustUnnamed_0 = 11;
pub const XML_ROLE_ENTITY_VALUE: C2RustUnnamed_0 = 12;
pub const XML_ROLE_ENTITY_SYSTEM_ID: C2RustUnnamed_0 = 13;
pub const XML_ROLE_ENTITY_PUBLIC_ID: C2RustUnnamed_0 = 14;
pub const XML_ROLE_ENTITY_COMPLETE: C2RustUnnamed_0 = 15;
pub const XML_ROLE_ENTITY_NOTATION_NAME: C2RustUnnamed_0 = 16;
pub const XML_ROLE_NOTATION_NONE: C2RustUnnamed_0 = 17;
pub const XML_ROLE_NOTATION_NAME: C2RustUnnamed_0 = 18;
pub const XML_ROLE_NOTATION_SYSTEM_ID: C2RustUnnamed_0 = 19;
pub const XML_ROLE_NOTATION_NO_SYSTEM_ID: C2RustUnnamed_0 = 20;
pub const XML_ROLE_NOTATION_PUBLIC_ID: C2RustUnnamed_0 = 21;
pub const XML_ROLE_ATTRIBUTE_NAME: C2RustUnnamed_0 = 22;
pub const XML_ROLE_ATTRIBUTE_TYPE_CDATA: C2RustUnnamed_0 = 23;
pub const XML_ROLE_ATTRIBUTE_TYPE_ID: C2RustUnnamed_0 = 24;
pub const XML_ROLE_ATTRIBUTE_TYPE_IDREF: C2RustUnnamed_0 = 25;
pub const XML_ROLE_ATTRIBUTE_TYPE_IDREFS: C2RustUnnamed_0 = 26;
pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITY: C2RustUnnamed_0 = 27;
pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITIES: C2RustUnnamed_0 = 28;
pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN: C2RustUnnamed_0 = 29;
pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS: C2RustUnnamed_0 = 30;
pub const XML_ROLE_ATTRIBUTE_ENUM_VALUE: C2RustUnnamed_0 = 31;
pub const XML_ROLE_ATTRIBUTE_NOTATION_VALUE: C2RustUnnamed_0 = 32;
pub const XML_ROLE_ATTLIST_NONE: C2RustUnnamed_0 = 33;
pub const XML_ROLE_ATTLIST_ELEMENT_NAME: C2RustUnnamed_0 = 34;
pub const XML_ROLE_IMPLIED_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 35;
pub const XML_ROLE_REQUIRED_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 36;
pub const XML_ROLE_DEFAULT_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 37;
pub const XML_ROLE_FIXED_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 38;
pub const XML_ROLE_ELEMENT_NONE: C2RustUnnamed_0 = 39;
pub const XML_ROLE_ELEMENT_NAME: C2RustUnnamed_0 = 40;
pub const XML_ROLE_CONTENT_ANY: C2RustUnnamed_0 = 41;
pub const XML_ROLE_CONTENT_EMPTY: C2RustUnnamed_0 = 42;
pub const XML_ROLE_CONTENT_PCDATA: C2RustUnnamed_0 = 43;
pub const XML_ROLE_GROUP_OPEN: C2RustUnnamed_0 = 44;
pub const XML_ROLE_GROUP_CLOSE: C2RustUnnamed_0 = 45;
pub const XML_ROLE_GROUP_CLOSE_REP: C2RustUnnamed_0 = 46;
pub const XML_ROLE_GROUP_CLOSE_OPT: C2RustUnnamed_0 = 47;
pub const XML_ROLE_GROUP_CLOSE_PLUS: C2RustUnnamed_0 = 48;
pub const XML_ROLE_GROUP_CHOICE: C2RustUnnamed_0 = 49;
pub const XML_ROLE_GROUP_SEQUENCE: C2RustUnnamed_0 = 50;
pub const XML_ROLE_CONTENT_ELEMENT: C2RustUnnamed_0 = 51;
pub const XML_ROLE_CONTENT_ELEMENT_REP: C2RustUnnamed_0 = 52;
pub const XML_ROLE_CONTENT_ELEMENT_OPT: C2RustUnnamed_0 = 53;
pub const XML_ROLE_CONTENT_ELEMENT_PLUS: C2RustUnnamed_0 = 54;
pub const XML_ROLE_PI: C2RustUnnamed_0 = 55;
pub const XML_ROLE_COMMENT: C2RustUnnamed_0 = 56;
pub const XML_ROLE_TEXT_DECL: C2RustUnnamed_0 = 57;
pub const XML_ROLE_IGNORE_SECT: C2RustUnnamed_0 = 58;
pub const XML_ROLE_INNER_PARAM_ENTITY_REF: C2RustUnnamed_0 = 59;
/* XML_DTD */
/* XML_DTD */

pub const XML_ROLE_PARAM_ENTITY_REF: C2RustUnnamed_0 = 60;

pub type PROLOG_STATE = prolog_state;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prolog_state {
    pub handler: Option<
        unsafe extern "C" fn(
            _: *mut prolog_state,
            _: c_int,
            _: *const c_char,
            _: *const c_char,
            _: *const super::xmltok::ENCODING,
        ) -> c_int,
    >,
    pub level: c_uint,
    pub role_none: c_int,
    pub includeLevel: c_uint,
    pub documentEntity: c_int,
    pub inEntityValue: c_int,
}

pub use crate::ascii_h::{
    ASCII_A, ASCII_B, ASCII_C, ASCII_D, ASCII_E, ASCII_F, ASCII_G, ASCII_I, ASCII_K, ASCII_L,
    ASCII_M, ASCII_N, ASCII_O, ASCII_P, ASCII_Q, ASCII_R, ASCII_S, ASCII_T, ASCII_U, ASCII_X,
    ASCII_Y,
};
pub use crate::expat_external_h::XML_Size;
pub use crate::src::lib::xmltok::{
    position, XML_Convert_Result, ATTRIBUTE, ENCODING, POSITION, XML_CONVERT_COMPLETED,
    XML_CONVERT_INPUT_INCOMPLETE, XML_CONVERT_OUTPUT_EXHAUSTED, XML_TOK_BOM, XML_TOK_CLOSE_BRACKET,
    XML_TOK_CLOSE_PAREN, XML_TOK_CLOSE_PAREN_ASTERISK, XML_TOK_CLOSE_PAREN_PLUS,
    XML_TOK_CLOSE_PAREN_QUESTION, XML_TOK_COMMA, XML_TOK_COMMENT, XML_TOK_COND_SECT_CLOSE,
    XML_TOK_COND_SECT_OPEN, XML_TOK_DECL_CLOSE, XML_TOK_DECL_OPEN, XML_TOK_LITERAL, XML_TOK_NAME,
    XML_TOK_NAME_ASTERISK, XML_TOK_NAME_PLUS, XML_TOK_NAME_QUESTION, XML_TOK_NMTOKEN, XML_TOK_NONE,
    XML_TOK_OPEN_BRACKET, XML_TOK_OPEN_PAREN, XML_TOK_OR, XML_TOK_PARAM_ENTITY_REF,
    XML_TOK_PERCENT, XML_TOK_PI, XML_TOK_POUND_NAME, XML_TOK_PREFIXED_NAME, XML_TOK_XML_DECL,
};
pub use crate::xmltok_h::{XML_TOK_INSTANCE_START, XML_TOK_PROLOG_S};
use ::libc;
/* not XML_DTD */
/* not XML_DTD */

pub type PROLOG_HANDLER = unsafe extern "C" fn(
    _: *mut PROLOG_STATE,
    _: c_int,
    _: *const c_char,
    _: *const c_char,
    _: *const super::xmltok::ENCODING,
) -> c_int;
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
/* Doesn't check:

 that ,| are not mixed in a model group
 content of literals

*/

static mut KW_ANY: [c_char; 4] = [ASCII_A, ASCII_N, ASCII_Y, '\u{0}' as c_char];

static mut KW_ATTLIST: [c_char; 8] = [
    ASCII_A,
    ASCII_T,
    ASCII_T,
    ASCII_L,
    ASCII_I,
    ASCII_S,
    ASCII_T,
    '\u{0}' as c_char,
];

static mut KW_CDATA: [c_char; 6] = [
    ASCII_C,
    ASCII_D,
    ASCII_A,
    ASCII_T,
    ASCII_A,
    '\u{0}' as c_char,
];

static mut KW_DOCTYPE: [c_char; 8] = [
    ASCII_D,
    ASCII_O,
    ASCII_C,
    ASCII_T,
    ASCII_Y,
    ASCII_P,
    ASCII_E,
    '\u{0}' as c_char,
];

static mut KW_ELEMENT: [c_char; 8] = [
    ASCII_E,
    ASCII_L,
    ASCII_E,
    ASCII_M,
    ASCII_E,
    ASCII_N,
    ASCII_T,
    '\u{0}' as c_char,
];

static mut KW_EMPTY: [c_char; 6] = [
    ASCII_E,
    ASCII_M,
    ASCII_P,
    ASCII_T,
    ASCII_Y,
    '\u{0}' as c_char,
];

static mut KW_ENTITIES: [c_char; 9] = [
    ASCII_E,
    ASCII_N,
    ASCII_T,
    ASCII_I,
    ASCII_T,
    ASCII_I,
    ASCII_E,
    ASCII_S,
    '\u{0}' as c_char,
];

static mut KW_ENTITY: [c_char; 7] = [
    ASCII_E,
    ASCII_N,
    ASCII_T,
    ASCII_I,
    ASCII_T,
    ASCII_Y,
    '\u{0}' as c_char,
];

static mut KW_FIXED: [c_char; 6] = [
    ASCII_F,
    ASCII_I,
    ASCII_X,
    ASCII_E,
    ASCII_D,
    '\u{0}' as c_char,
];

static mut KW_ID: [c_char; 3] = [ASCII_I, ASCII_D, '\u{0}' as c_char];

static mut KW_IDREF: [c_char; 6] = [
    ASCII_I,
    ASCII_D,
    ASCII_R,
    ASCII_E,
    ASCII_F,
    '\u{0}' as c_char,
];

static mut KW_IDREFS: [c_char; 7] = [
    ASCII_I,
    ASCII_D,
    ASCII_R,
    ASCII_E,
    ASCII_F,
    ASCII_S,
    '\u{0}' as c_char,
];

static mut KW_IGNORE: [c_char; 7] = [
    ASCII_I,
    ASCII_G,
    ASCII_N,
    ASCII_O,
    ASCII_R,
    ASCII_E,
    '\u{0}' as c_char,
];

static mut KW_IMPLIED: [c_char; 8] = [
    ASCII_I,
    ASCII_M,
    ASCII_P,
    ASCII_L,
    ASCII_I,
    ASCII_E,
    ASCII_D,
    '\u{0}' as c_char,
];

static mut KW_INCLUDE: [c_char; 8] = [
    ASCII_I,
    ASCII_N,
    ASCII_C,
    ASCII_L,
    ASCII_U,
    ASCII_D,
    ASCII_E,
    '\u{0}' as c_char,
];

static mut KW_NDATA: [c_char; 6] = [
    ASCII_N,
    ASCII_D,
    ASCII_A,
    ASCII_T,
    ASCII_A,
    '\u{0}' as c_char,
];

static mut KW_NMTOKEN: [c_char; 8] = [
    ASCII_N,
    ASCII_M,
    ASCII_T,
    ASCII_O,
    ASCII_K,
    ASCII_E,
    ASCII_N,
    '\u{0}' as c_char,
];

static mut KW_NMTOKENS: [c_char; 9] = [
    ASCII_N,
    ASCII_M,
    ASCII_T,
    ASCII_O,
    ASCII_K,
    ASCII_E,
    ASCII_N,
    ASCII_S,
    '\u{0}' as c_char,
];

static mut KW_NOTATION: [c_char; 9] = [
    ASCII_N,
    ASCII_O,
    ASCII_T,
    ASCII_A,
    ASCII_T,
    ASCII_I,
    ASCII_O,
    ASCII_N,
    '\u{0}' as c_char,
];

static mut KW_PCDATA: [c_char; 7] = [
    ASCII_P,
    ASCII_C,
    ASCII_D,
    ASCII_A,
    ASCII_T,
    ASCII_A,
    '\u{0}' as c_char,
];

static mut KW_PUBLIC: [c_char; 7] = [
    ASCII_P,
    ASCII_U,
    ASCII_B,
    ASCII_L,
    ASCII_I,
    ASCII_C,
    '\u{0}' as c_char,
];

static mut KW_REQUIRED: [c_char; 9] = [
    ASCII_R,
    ASCII_E,
    ASCII_Q,
    ASCII_U,
    ASCII_I,
    ASCII_R,
    ASCII_E,
    ASCII_D,
    '\u{0}' as c_char,
];

static mut KW_SYSTEM: [c_char; 7] = [
    ASCII_S,
    ASCII_Y,
    ASCII_S,
    ASCII_T,
    ASCII_E,
    ASCII_M,
    '\u{0}' as c_char,
];

unsafe extern "C" fn prolog0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => {
            (*state).handler = Some(prolog1 as PROLOG_HANDLER); /* LCOV_EXCL_LINE */
            return XML_ROLE_NONE;
        }
        super::xmltok::XML_TOK_XML_DECL => {
            (*state).handler = Some(prolog1 as PROLOG_HANDLER);
            return XML_ROLE_XML_DECL;
        }
        super::xmltok::XML_TOK_PI => {
            (*state).handler = Some(prolog1 as PROLOG_HANDLER);
            return XML_ROLE_PI;
        }
        super::xmltok::XML_TOK_COMMENT => {
            (*state).handler = Some(prolog1 as PROLOG_HANDLER);
            return XML_ROLE_COMMENT;
        }
        super::xmltok::XML_TOK_BOM => return XML_ROLE_NONE,
        super::xmltok::XML_TOK_DECL_OPEN => {
            if !((*enc).nameMatchesAscii(
                ptr.offset((2i32 * (*enc).minBytesPerChar()) as isize),
                end,
                KW_DOCTYPE.as_ptr(),
            ) == 0)
            {
                (*state).handler = Some(doctype0 as PROLOG_HANDLER);
                return XML_ROLE_DOCTYPE_NONE;
            }
        }
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(error as PROLOG_HANDLER);
            return XML_ROLE_INSTANCE_START;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn prolog1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        super::xmltok::XML_TOK_PI => return XML_ROLE_PI,
        super::xmltok::XML_TOK_COMMENT => return XML_ROLE_COMMENT,
        super::xmltok::XML_TOK_BOM => {
            /* This case can never arise.  To reach this role function, the
             * parse must have passed through prolog0 and therefore have had
             * some form of input, even if only a space.  At that point, a
             * byte order mark is no longer a valid character (though
             * technically it should be interpreted as a non-breaking space),
             * so will be rejected by the tokenizing stages.
             */
            return XML_ROLE_NONE;
        }
        super::xmltok::XML_TOK_DECL_OPEN => {
            if !((*enc).nameMatchesAscii(
                ptr.offset((2i32 * (*enc).minBytesPerChar()) as isize),
                end,
                KW_DOCTYPE.as_ptr(),
            ) == 0)
            {
                (*state).handler = Some(doctype0 as PROLOG_HANDLER);
                return XML_ROLE_DOCTYPE_NONE;
            }
        }
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(error as PROLOG_HANDLER);
            return XML_ROLE_INSTANCE_START;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn prolog2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        super::xmltok::XML_TOK_PI => return XML_ROLE_PI,
        super::xmltok::XML_TOK_COMMENT => return XML_ROLE_COMMENT,
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(error as PROLOG_HANDLER);
            return XML_ROLE_INSTANCE_START;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn doctype0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(doctype1 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn doctype1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        super::xmltok::XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(internalSubset as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET;
        }
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(prolog2 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_CLOSE;
        }
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(ptr, end, KW_SYSTEM.as_ptr()) != 0 {
                (*state).handler = Some(doctype3 as PROLOG_HANDLER);
                return XML_ROLE_DOCTYPE_NONE;
            }
            if (*enc).nameMatchesAscii(ptr, end, KW_PUBLIC.as_ptr()) != 0 {
                (*state).handler = Some(doctype2 as PROLOG_HANDLER);
                return XML_ROLE_DOCTYPE_NONE;
            }
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn doctype2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(doctype3 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_PUBLIC_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn doctype3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(doctype4 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_SYSTEM_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn doctype4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        super::xmltok::XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(internalSubset as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET;
        }
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(prolog2 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_CLOSE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn doctype5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(prolog2 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_CLOSE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn internalSubset(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        super::xmltok::XML_TOK_DECL_OPEN => {
            if (*enc).nameMatchesAscii(
                ptr.offset((2i32 * (*enc).minBytesPerChar()) as isize),
                end,
                KW_ENTITY.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(entity0 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE;
            }
            if (*enc).nameMatchesAscii(
                ptr.offset((2i32 * (*enc).minBytesPerChar()) as isize),
                end,
                KW_ATTLIST.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(attlist0 as PROLOG_HANDLER);
                return XML_ROLE_ATTLIST_NONE;
            }
            if (*enc).nameMatchesAscii(
                ptr.offset((2i32 * (*enc).minBytesPerChar()) as isize),
                end,
                KW_ELEMENT.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(element0 as PROLOG_HANDLER);
                return XML_ROLE_ELEMENT_NONE;
            }
            if (*enc).nameMatchesAscii(
                ptr.offset((2i32 * (*enc).minBytesPerChar()) as isize),
                end,
                KW_NOTATION.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(notation0 as PROLOG_HANDLER);
                return XML_ROLE_NOTATION_NONE;
            }
        }
        super::xmltok::XML_TOK_PI => return XML_ROLE_PI,
        super::xmltok::XML_TOK_COMMENT => return XML_ROLE_COMMENT,
        super::xmltok::XML_TOK_PARAM_ENTITY_REF => return XML_ROLE_PARAM_ENTITY_REF,
        super::xmltok::XML_TOK_CLOSE_BRACKET => {
            (*state).handler = Some(doctype5 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_NONE;
        }
        super::xmltok::XML_TOK_NONE => return XML_ROLE_NONE,
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn externalSubset0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    (*state).handler = Some(externalSubset1 as PROLOG_HANDLER);
    if tok == super::xmltok::XML_TOK_XML_DECL {
        return XML_ROLE_TEXT_DECL;
    }
    return externalSubset1(state, tok, ptr, end, enc);
}

unsafe extern "C" fn externalSubset1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        super::xmltok::XML_TOK_COND_SECT_OPEN => {
            (*state).handler = Some(condSect0 as PROLOG_HANDLER);
            return XML_ROLE_NONE;
        }
        super::xmltok::XML_TOK_COND_SECT_CLOSE => {
            if !((*state).includeLevel == 0u32) {
                (*state).includeLevel = (*state).includeLevel.wrapping_sub(1u32);
                return XML_ROLE_NONE;
            }
        }
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        super::xmltok::XML_TOK_CLOSE_BRACKET => {}
        super::xmltok::XML_TOK_NONE => {
            if !((*state).includeLevel != 0) {
                return XML_ROLE_NONE;
            }
        }
        _ => return internalSubset(state, tok, ptr, end, enc),
    }
    return common(state, tok);
}
/* XML_DTD */

unsafe extern "C" fn entity0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_PERCENT => {
            (*state).handler = Some(entity1 as PROLOG_HANDLER);
            return XML_ROLE_ENTITY_NONE;
        }
        super::xmltok::XML_TOK_NAME => {
            (*state).handler = Some(entity2 as PROLOG_HANDLER);
            return XML_ROLE_GENERAL_ENTITY_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn entity1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_NAME => {
            (*state).handler = Some(entity7 as PROLOG_HANDLER);
            return XML_ROLE_PARAM_ENTITY_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn entity2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(ptr, end, KW_SYSTEM.as_ptr()) != 0 {
                (*state).handler = Some(entity4 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE;
            }
            if (*enc).nameMatchesAscii(ptr, end, KW_PUBLIC.as_ptr()) != 0 {
                (*state).handler = Some(entity3 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE;
            }
        }
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ENTITY_NONE;
            return XML_ROLE_ENTITY_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn entity3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(entity4 as PROLOG_HANDLER);
            return XML_ROLE_ENTITY_PUBLIC_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn entity4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(entity5 as PROLOG_HANDLER);
            return XML_ROLE_ENTITY_SYSTEM_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn entity5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE_ENTITY_COMPLETE;
        }
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(ptr, end, KW_NDATA.as_ptr()) != 0 {
                (*state).handler = Some(entity6 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE;
            }
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn entity6(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_NAME => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ENTITY_NONE;
            return XML_ROLE_ENTITY_NOTATION_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn entity7(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(ptr, end, KW_SYSTEM.as_ptr()) != 0 {
                (*state).handler = Some(entity9 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE;
            }
            if (*enc).nameMatchesAscii(ptr, end, KW_PUBLIC.as_ptr()) != 0 {
                (*state).handler = Some(entity8 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE;
            }
        }
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ENTITY_NONE;
            return XML_ROLE_ENTITY_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn entity8(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(entity9 as PROLOG_HANDLER);
            return XML_ROLE_ENTITY_PUBLIC_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn entity9(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(entity10 as PROLOG_HANDLER);
            return XML_ROLE_ENTITY_SYSTEM_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn entity10(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE_ENTITY_COMPLETE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn notation0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        super::xmltok::XML_TOK_NAME => {
            (*state).handler = Some(notation1 as PROLOG_HANDLER);
            return XML_ROLE_NOTATION_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn notation1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(ptr, end, KW_SYSTEM.as_ptr()) != 0 {
                (*state).handler = Some(notation3 as PROLOG_HANDLER);
                return XML_ROLE_NOTATION_NONE;
            }
            if (*enc).nameMatchesAscii(ptr, end, KW_PUBLIC.as_ptr()) != 0 {
                (*state).handler = Some(notation2 as PROLOG_HANDLER);
                return XML_ROLE_NOTATION_NONE;
            }
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn notation2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(notation4 as PROLOG_HANDLER);
            return XML_ROLE_NOTATION_PUBLIC_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn notation3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_NOTATION_NONE;
            return XML_ROLE_NOTATION_SYSTEM_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn notation4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_NOTATION_NONE;
            return XML_ROLE_NOTATION_SYSTEM_ID;
        }
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE_NOTATION_NO_SYSTEM_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn attlist0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(attlist1 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_ELEMENT_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn attlist1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE_ATTLIST_NONE;
        }
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(attlist2 as PROLOG_HANDLER);
            return XML_ROLE_ATTRIBUTE_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn attlist2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        super::xmltok::XML_TOK_NAME => {
            static mut types: [*const c_char; 8] = unsafe {
                [
                    KW_CDATA.as_ptr(),
                    KW_ID.as_ptr(),
                    KW_IDREF.as_ptr(),
                    KW_IDREFS.as_ptr(),
                    KW_ENTITY.as_ptr(),
                    KW_ENTITIES.as_ptr(),
                    KW_NMTOKEN.as_ptr(),
                    KW_NMTOKENS.as_ptr(),
                ]
            };
            let mut i: c_int = 0;
            i = 0;
            while i
                < (::std::mem::size_of::<[*const c_char; 8]>() as c_ulong)
                    .wrapping_div(::std::mem::size_of::<*const c_char>() as c_ulong)
                    as c_int
            {
                if (*enc).nameMatchesAscii(ptr, end, types[i as usize]) != 0 {
                    (*state).handler = Some(attlist8 as PROLOG_HANDLER);
                    return XML_ROLE_ATTRIBUTE_TYPE_CDATA + i;
                }
                i += 1
            }
            if (*enc).nameMatchesAscii(ptr, end, KW_NOTATION.as_ptr()) != 0 {
                (*state).handler = Some(attlist5 as PROLOG_HANDLER);
                return XML_ROLE_ATTLIST_NONE;
            }
        }
        super::xmltok::XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(attlist3 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn attlist3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        super::xmltok::XML_TOK_NMTOKEN
        | super::xmltok::XML_TOK_NAME
        | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(attlist4 as PROLOG_HANDLER);
            return XML_ROLE_ATTRIBUTE_ENUM_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn attlist4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        super::xmltok::XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(attlist8 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE;
        }
        super::xmltok::XML_TOK_OR => {
            (*state).handler = Some(attlist3 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn attlist5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        super::xmltok::XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(attlist6 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn attlist6(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        super::xmltok::XML_TOK_NAME => {
            (*state).handler = Some(attlist7 as PROLOG_HANDLER);
            return XML_ROLE_ATTRIBUTE_NOTATION_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn attlist7(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        super::xmltok::XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(attlist8 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE;
        }
        super::xmltok::XML_TOK_OR => {
            (*state).handler = Some(attlist6 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}
/* default value */

unsafe extern "C" fn attlist8(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        super::xmltok::XML_TOK_POUND_NAME => {
            if (*enc).nameMatchesAscii(
                ptr.offset((*enc).minBytesPerChar() as isize),
                end,
                KW_IMPLIED.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(attlist1 as PROLOG_HANDLER);
                return XML_ROLE_IMPLIED_ATTRIBUTE_VALUE;
            }
            if (*enc).nameMatchesAscii(
                ptr.offset((*enc).minBytesPerChar() as isize),
                end,
                KW_REQUIRED.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(attlist1 as PROLOG_HANDLER);
                return XML_ROLE_REQUIRED_ATTRIBUTE_VALUE;
            }
            if (*enc).nameMatchesAscii(
                ptr.offset((*enc).minBytesPerChar() as isize),
                end,
                KW_FIXED.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(attlist9 as PROLOG_HANDLER);
                return XML_ROLE_ATTLIST_NONE;
            }
        }
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(attlist1 as PROLOG_HANDLER);
            return XML_ROLE_DEFAULT_ATTRIBUTE_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn attlist9(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(attlist1 as PROLOG_HANDLER);
            return XML_ROLE_FIXED_ATTRIBUTE_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn element0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(element1 as PROLOG_HANDLER);
            return XML_ROLE_ELEMENT_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn element1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(ptr, end, KW_EMPTY.as_ptr()) != 0 {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE;
                return XML_ROLE_CONTENT_EMPTY;
            }
            if (*enc).nameMatchesAscii(ptr, end, KW_ANY.as_ptr()) != 0 {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE;
                return XML_ROLE_CONTENT_ANY;
            }
        }
        super::xmltok::XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(element2 as PROLOG_HANDLER);
            (*state).level = 1u32;
            return XML_ROLE_GROUP_OPEN;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn element2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        super::xmltok::XML_TOK_POUND_NAME => {
            if (*enc).nameMatchesAscii(
                ptr.offset((*enc).minBytesPerChar() as isize),
                end,
                KW_PCDATA.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(element3 as PROLOG_HANDLER);
                return XML_ROLE_CONTENT_PCDATA;
            }
        }
        super::xmltok::XML_TOK_OPEN_PAREN => {
            (*state).level = 2u32;
            (*state).handler = Some(element6 as PROLOG_HANDLER);
            return XML_ROLE_GROUP_OPEN;
        }
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT;
        }
        super::xmltok::XML_TOK_NAME_QUESTION => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_OPT;
        }
        super::xmltok::XML_TOK_NAME_ASTERISK => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_REP;
        }
        super::xmltok::XML_TOK_NAME_PLUS => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_PLUS;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn element3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        super::xmltok::XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ELEMENT_NONE;
            return XML_ROLE_GROUP_CLOSE;
        }
        super::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ELEMENT_NONE;
            return XML_ROLE_GROUP_CLOSE_REP;
        }
        super::xmltok::XML_TOK_OR => {
            (*state).handler = Some(element4 as PROLOG_HANDLER);
            return XML_ROLE_ELEMENT_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn element4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(element5 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn element5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        super::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ELEMENT_NONE;
            return XML_ROLE_GROUP_CLOSE_REP;
        }
        super::xmltok::XML_TOK_OR => {
            (*state).handler = Some(element4 as PROLOG_HANDLER);
            return XML_ROLE_ELEMENT_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn element6(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        super::xmltok::XML_TOK_OPEN_PAREN => {
            (*state).level = (*state).level.wrapping_add(1u32);
            return XML_ROLE_GROUP_OPEN;
        }
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT;
        }
        super::xmltok::XML_TOK_NAME_QUESTION => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_OPT;
        }
        super::xmltok::XML_TOK_NAME_ASTERISK => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_REP;
        }
        super::xmltok::XML_TOK_NAME_PLUS => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_PLUS;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn element7(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        super::xmltok::XML_TOK_CLOSE_PAREN => {
            (*state).level = (*state).level.wrapping_sub(1u32);
            if (*state).level == 0u32 {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE
            }
            return XML_ROLE_GROUP_CLOSE;
        }
        super::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).level = (*state).level.wrapping_sub(1u32);
            if (*state).level == 0u32 {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE
            }
            return XML_ROLE_GROUP_CLOSE_REP;
        }
        super::xmltok::XML_TOK_CLOSE_PAREN_QUESTION => {
            (*state).level = (*state).level.wrapping_sub(1u32);
            if (*state).level == 0u32 {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE
            }
            return XML_ROLE_GROUP_CLOSE_OPT;
        }
        super::xmltok::XML_TOK_CLOSE_PAREN_PLUS => {
            (*state).level = (*state).level.wrapping_sub(1u32);
            if (*state).level == 0u32 {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE
            }
            return XML_ROLE_GROUP_CLOSE_PLUS;
        }
        super::xmltok::XML_TOK_COMMA => {
            (*state).handler = Some(element6 as PROLOG_HANDLER);
            return XML_ROLE_GROUP_SEQUENCE;
        }
        super::xmltok::XML_TOK_OR => {
            (*state).handler = Some(element6 as PROLOG_HANDLER);
            return XML_ROLE_GROUP_CHOICE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn condSect0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(ptr, end, KW_INCLUDE.as_ptr()) != 0 {
                (*state).handler = Some(condSect1 as PROLOG_HANDLER);
                return XML_ROLE_NONE;
            }
            if (*enc).nameMatchesAscii(ptr, end, KW_IGNORE.as_ptr()) != 0 {
                (*state).handler = Some(condSect2 as PROLOG_HANDLER);
                return XML_ROLE_NONE;
            }
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn condSect1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        super::xmltok::XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(externalSubset1 as PROLOG_HANDLER);
            (*state).includeLevel = (*state).includeLevel.wrapping_add(1u32);
            return XML_ROLE_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe extern "C" fn condSect2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        super::xmltok::XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(externalSubset1 as PROLOG_HANDLER);
            return XML_ROLE_IGNORE_SECT;
        }
        _ => {}
    }
    return common(state, tok);
}
/* XML_DTD */

unsafe extern "C" fn declClose(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return (*state).role_none,
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return (*state).role_none;
        }
        _ => {}
    }
    return common(state, tok);
}
/* This function will only be invoked if the internal logic of the
 * parser has broken down.  It is used in two cases:
 *
 * 1: When the XML prolog has been finished.  At this point the
 * processor (the parser level above these role handlers) should
 * switch from prologProcessor to contentProcessor and reinitialise
 * the handler function.
 *
 * 2: When an error has been detected (via common() below).  At this
 * point again the processor should be switched to errorProcessor,
 * which will never call a handler.
 *
 * The result of this is that error() can only be called if the
 * processor switch failed to happen, which is an internal error and
 * therefore we shouldn't be able to provoke it simply by using the
 * library.  It is a necessary backstop, however, so we merely exclude
 * it from the coverage statistics.
 *
 * LCOV_EXCL_START
 */

unsafe extern "C" fn error(
    mut _state: *mut PROLOG_STATE,
    mut _tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const super::xmltok::ENCODING,
) -> c_int {
    return XML_ROLE_NONE;
}
/* LCOV_EXCL_STOP */

unsafe extern "C" fn common(mut state: *mut PROLOG_STATE, mut tok: c_int) -> c_int {
    if (*state).documentEntity == 0 && tok == super::xmltok::XML_TOK_PARAM_ENTITY_REF {
        return XML_ROLE_INNER_PARAM_ENTITY_REF;
    }
    (*state).handler = Some(error as PROLOG_HANDLER);
    return XML_ROLE_ERROR;
}
#[no_mangle]

pub unsafe extern "C" fn XmlPrologStateInit(mut state: *mut PROLOG_STATE) {
    (*state).handler = Some(prolog0 as PROLOG_HANDLER);
    (*state).documentEntity = 1;
    (*state).includeLevel = 0u32;
    (*state).inEntityValue = 0;
    /* XML_DTD */
}
#[no_mangle]

pub unsafe extern "C" fn XmlPrologStateInitExternalEntity(mut state: *mut PROLOG_STATE) {
    (*state).handler = Some(externalSubset0 as PROLOG_HANDLER);
    (*state).documentEntity = 0;
    (*state).includeLevel = 0u32;
}
/* XML_DTD */
