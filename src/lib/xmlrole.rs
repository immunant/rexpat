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
use libc::c_char;
use libc::c_int;
use libc::c_uint;
use libc::c_ulong;
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
    encoding, position, XML_Convert_Result, ATTRIBUTE, ENCODING, POSITION, SCANNER,
    XML_CONVERT_COMPLETED, XML_CONVERT_INPUT_INCOMPLETE, XML_CONVERT_OUTPUT_EXHAUSTED, XML_TOK_BOM,
    XML_TOK_CLOSE_BRACKET, XML_TOK_CLOSE_PAREN, XML_TOK_CLOSE_PAREN_ASTERISK,
    XML_TOK_CLOSE_PAREN_PLUS, XML_TOK_CLOSE_PAREN_QUESTION, XML_TOK_COMMA, XML_TOK_COMMENT,
    XML_TOK_COND_SECT_CLOSE, XML_TOK_COND_SECT_OPEN, XML_TOK_DECL_CLOSE, XML_TOK_DECL_OPEN,
    XML_TOK_LITERAL, XML_TOK_NAME, XML_TOK_NAME_ASTERISK, XML_TOK_NAME_PLUS, XML_TOK_NAME_QUESTION,
    XML_TOK_NMTOKEN, XML_TOK_NONE, XML_TOK_OPEN_BRACKET, XML_TOK_OPEN_PAREN, XML_TOK_OR,
    XML_TOK_PARAM_ENTITY_REF, XML_TOK_PERCENT, XML_TOK_PI, XML_TOK_POUND_NAME,
    XML_TOK_PREFIXED_NAME, XML_TOK_XML_DECL,
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

static mut KW_ANY: [c_char; 4] = [
    ASCII_A as c_char,
    ASCII_N as c_char,
    ASCII_Y as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_ATTLIST: [c_char; 8] = [
    ASCII_A as c_char,
    ASCII_T as c_char,
    ASCII_T as c_char,
    ASCII_L as c_char,
    ASCII_I as c_char,
    ASCII_S as c_char,
    ASCII_T as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_CDATA: [c_char; 6] = [
    ASCII_C as c_char,
    ASCII_D as c_char,
    ASCII_A as c_char,
    ASCII_T as c_char,
    ASCII_A as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_DOCTYPE: [c_char; 8] = [
    ASCII_D as c_char,
    ASCII_O as c_char,
    ASCII_C as c_char,
    ASCII_T as c_char,
    ASCII_Y as c_char,
    ASCII_P as c_char,
    ASCII_E as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_ELEMENT: [c_char; 8] = [
    ASCII_E as c_char,
    ASCII_L as c_char,
    ASCII_E as c_char,
    ASCII_M as c_char,
    ASCII_E as c_char,
    ASCII_N as c_char,
    ASCII_T as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_EMPTY: [c_char; 6] = [
    ASCII_E as c_char,
    ASCII_M as c_char,
    ASCII_P as c_char,
    ASCII_T as c_char,
    ASCII_Y as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_ENTITIES: [c_char; 9] = [
    ASCII_E as c_char,
    ASCII_N as c_char,
    ASCII_T as c_char,
    ASCII_I as c_char,
    ASCII_T as c_char,
    ASCII_I as c_char,
    ASCII_E as c_char,
    ASCII_S as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_ENTITY: [c_char; 7] = [
    ASCII_E as c_char,
    ASCII_N as c_char,
    ASCII_T as c_char,
    ASCII_I as c_char,
    ASCII_T as c_char,
    ASCII_Y as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_FIXED: [c_char; 6] = [
    ASCII_F as c_char,
    ASCII_I as c_char,
    ASCII_X as c_char,
    ASCII_E as c_char,
    ASCII_D as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_ID: [c_char; 3] = [
    ASCII_I as c_char,
    ASCII_D as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_IDREF: [c_char; 6] = [
    ASCII_I as c_char,
    ASCII_D as c_char,
    ASCII_R as c_char,
    ASCII_E as c_char,
    ASCII_F as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_IDREFS: [c_char; 7] = [
    ASCII_I as c_char,
    ASCII_D as c_char,
    ASCII_R as c_char,
    ASCII_E as c_char,
    ASCII_F as c_char,
    ASCII_S as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_IGNORE: [c_char; 7] = [
    ASCII_I as c_char,
    ASCII_G as c_char,
    ASCII_N as c_char,
    ASCII_O as c_char,
    ASCII_R as c_char,
    ASCII_E as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_IMPLIED: [c_char; 8] = [
    ASCII_I as c_char,
    ASCII_M as c_char,
    ASCII_P as c_char,
    ASCII_L as c_char,
    ASCII_I as c_char,
    ASCII_E as c_char,
    ASCII_D as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_INCLUDE: [c_char; 8] = [
    ASCII_I as c_char,
    ASCII_N as c_char,
    ASCII_C as c_char,
    ASCII_L as c_char,
    ASCII_U as c_char,
    ASCII_D as c_char,
    ASCII_E as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_NDATA: [c_char; 6] = [
    ASCII_N as c_char,
    ASCII_D as c_char,
    ASCII_A as c_char,
    ASCII_T as c_char,
    ASCII_A as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_NMTOKEN: [c_char; 8] = [
    ASCII_N as c_char,
    ASCII_M as c_char,
    ASCII_T as c_char,
    ASCII_O as c_char,
    ASCII_K as c_char,
    ASCII_E as c_char,
    ASCII_N as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_NMTOKENS: [c_char; 9] = [
    ASCII_N as c_char,
    ASCII_M as c_char,
    ASCII_T as c_char,
    ASCII_O as c_char,
    ASCII_K as c_char,
    ASCII_E as c_char,
    ASCII_N as c_char,
    ASCII_S as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_NOTATION: [c_char; 9] = [
    ASCII_N as c_char,
    ASCII_O as c_char,
    ASCII_T as c_char,
    ASCII_A as c_char,
    ASCII_T as c_char,
    ASCII_I as c_char,
    ASCII_O as c_char,
    ASCII_N as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_PCDATA: [c_char; 7] = [
    ASCII_P as c_char,
    ASCII_C as c_char,
    ASCII_D as c_char,
    ASCII_A as c_char,
    ASCII_T as c_char,
    ASCII_A as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_PUBLIC: [c_char; 7] = [
    ASCII_P as c_char,
    ASCII_U as c_char,
    ASCII_B as c_char,
    ASCII_L as c_char,
    ASCII_I as c_char,
    ASCII_C as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_REQUIRED: [c_char; 9] = [
    ASCII_R as c_char,
    ASCII_E as c_char,
    ASCII_Q as c_char,
    ASCII_U as c_char,
    ASCII_I as c_char,
    ASCII_R as c_char,
    ASCII_E as c_char,
    ASCII_D as c_char,
    '\u{0}' as i32 as c_char,
];

static mut KW_SYSTEM: [c_char; 7] = [
    ASCII_S as c_char,
    ASCII_Y as c_char,
    ASCII_S as c_char,
    ASCII_T as c_char,
    ASCII_E as c_char,
    ASCII_M as c_char,
    '\u{0}' as i32 as c_char,
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
            return XML_ROLE_NONE as c_int;
        }
        super::xmltok::XML_TOK_XML_DECL => {
            (*state).handler = Some(prolog1 as PROLOG_HANDLER);
            return XML_ROLE_XML_DECL as c_int;
        }
        super::xmltok::XML_TOK_PI => {
            (*state).handler = Some(prolog1 as PROLOG_HANDLER);
            return XML_ROLE_PI as c_int;
        }
        super::xmltok::XML_TOK_COMMENT => {
            (*state).handler = Some(prolog1 as PROLOG_HANDLER);
            return XML_ROLE_COMMENT as c_int;
        }
        super::xmltok::XML_TOK_BOM => return XML_ROLE_NONE as c_int,
        super::xmltok::XML_TOK_DECL_OPEN => {
            if !((*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as c_int * (*enc).minBytesPerChar) as isize),
                end,
                KW_DOCTYPE.as_ptr(),
            ) == 0)
            {
                (*state).handler = Some(doctype0 as PROLOG_HANDLER);
                return XML_ROLE_DOCTYPE_NONE as c_int;
            }
        }
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(error as PROLOG_HANDLER);
            return XML_ROLE_INSTANCE_START as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as c_int,
        super::xmltok::XML_TOK_PI => return XML_ROLE_PI as c_int,
        super::xmltok::XML_TOK_COMMENT => return XML_ROLE_COMMENT as c_int,
        super::xmltok::XML_TOK_BOM => {
            /* This case can never arise.  To reach this role function, the
             * parse must have passed through prolog0 and therefore have had
             * some form of input, even if only a space.  At that point, a
             * byte order mark is no longer a valid character (though
             * technically it should be interpreted as a non-breaking space),
             * so will be rejected by the tokenizing stages.
             */
            return XML_ROLE_NONE as c_int;
        }
        super::xmltok::XML_TOK_DECL_OPEN => {
            if !((*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as c_int * (*enc).minBytesPerChar) as isize),
                end,
                KW_DOCTYPE.as_ptr(),
            ) == 0)
            {
                (*state).handler = Some(doctype0 as PROLOG_HANDLER);
                return XML_ROLE_DOCTYPE_NONE as c_int;
            }
        }
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(error as PROLOG_HANDLER);
            return XML_ROLE_INSTANCE_START as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as c_int,
        super::xmltok::XML_TOK_PI => return XML_ROLE_PI as c_int,
        super::xmltok::XML_TOK_COMMENT => return XML_ROLE_COMMENT as c_int,
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(error as PROLOG_HANDLER);
            return XML_ROLE_INSTANCE_START as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as c_int,
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(doctype1 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_NAME as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as c_int,
        super::xmltok::XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(internalSubset as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET as c_int;
        }
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(prolog2 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_CLOSE as c_int;
        }
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_SYSTEM.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(doctype3 as PROLOG_HANDLER);
                return XML_ROLE_DOCTYPE_NONE as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_PUBLIC.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(doctype2 as PROLOG_HANDLER);
                return XML_ROLE_DOCTYPE_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as c_int,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(doctype3 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_PUBLIC_ID as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as c_int,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(doctype4 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_SYSTEM_ID as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as c_int,
        super::xmltok::XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(internalSubset as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET as c_int;
        }
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(prolog2 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_CLOSE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as c_int,
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(prolog2 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_CLOSE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as c_int,
        super::xmltok::XML_TOK_DECL_OPEN => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as c_int * (*enc).minBytesPerChar) as isize),
                end,
                KW_ENTITY.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(entity0 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as c_int * (*enc).minBytesPerChar) as isize),
                end,
                KW_ATTLIST.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(attlist0 as PROLOG_HANDLER);
                return XML_ROLE_ATTLIST_NONE as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as c_int * (*enc).minBytesPerChar) as isize),
                end,
                KW_ELEMENT.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(element0 as PROLOG_HANDLER);
                return XML_ROLE_ELEMENT_NONE as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as c_int * (*enc).minBytesPerChar) as isize),
                end,
                KW_NOTATION.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(notation0 as PROLOG_HANDLER);
                return XML_ROLE_NOTATION_NONE as c_int;
            }
        }
        super::xmltok::XML_TOK_PI => return XML_ROLE_PI as c_int,
        super::xmltok::XML_TOK_COMMENT => return XML_ROLE_COMMENT as c_int,
        super::xmltok::XML_TOK_PARAM_ENTITY_REF => return XML_ROLE_PARAM_ENTITY_REF as c_int,
        super::xmltok::XML_TOK_CLOSE_BRACKET => {
            (*state).handler = Some(doctype5 as PROLOG_HANDLER);
            return XML_ROLE_DOCTYPE_NONE as c_int;
        }
        super::xmltok::XML_TOK_NONE => return XML_ROLE_NONE as c_int,
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
        return XML_ROLE_TEXT_DECL as c_int;
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
            return XML_ROLE_NONE as c_int;
        }
        super::xmltok::XML_TOK_COND_SECT_CLOSE => {
            if !((*state).includeLevel == 0 as c_int as c_uint) {
                (*state).includeLevel = (*state).includeLevel.wrapping_sub(1 as c_int as c_uint);
                return XML_ROLE_NONE as c_int;
            }
        }
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as c_int,
        super::xmltok::XML_TOK_CLOSE_BRACKET => {}
        super::xmltok::XML_TOK_NONE => {
            if !((*state).includeLevel != 0) {
                return XML_ROLE_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_PERCENT => {
            (*state).handler = Some(entity1 as PROLOG_HANDLER);
            return XML_ROLE_ENTITY_NONE as c_int;
        }
        super::xmltok::XML_TOK_NAME => {
            (*state).handler = Some(entity2 as PROLOG_HANDLER);
            return XML_ROLE_GENERAL_ENTITY_NAME as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_NAME => {
            (*state).handler = Some(entity7 as PROLOG_HANDLER);
            return XML_ROLE_PARAM_ENTITY_NAME as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_SYSTEM.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(entity4 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_PUBLIC.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(entity3 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE as c_int;
            }
        }
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ENTITY_NONE as c_int;
            return XML_ROLE_ENTITY_VALUE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(entity4 as PROLOG_HANDLER);
            return XML_ROLE_ENTITY_PUBLIC_ID as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(entity5 as PROLOG_HANDLER);
            return XML_ROLE_ENTITY_SYSTEM_ID as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE_ENTITY_COMPLETE as c_int;
        }
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_NDATA.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(entity6 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_NAME => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ENTITY_NONE as c_int;
            return XML_ROLE_ENTITY_NOTATION_NAME as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_SYSTEM.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(entity9 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_PUBLIC.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(entity8 as PROLOG_HANDLER);
                return XML_ROLE_ENTITY_NONE as c_int;
            }
        }
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ENTITY_NONE as c_int;
            return XML_ROLE_ENTITY_VALUE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(entity9 as PROLOG_HANDLER);
            return XML_ROLE_ENTITY_PUBLIC_ID as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(entity10 as PROLOG_HANDLER);
            return XML_ROLE_ENTITY_SYSTEM_ID as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as c_int,
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE_ENTITY_COMPLETE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as c_int,
        super::xmltok::XML_TOK_NAME => {
            (*state).handler = Some(notation1 as PROLOG_HANDLER);
            return XML_ROLE_NOTATION_NAME as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as c_int,
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_SYSTEM.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(notation3 as PROLOG_HANDLER);
                return XML_ROLE_NOTATION_NONE as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_PUBLIC.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(notation2 as PROLOG_HANDLER);
                return XML_ROLE_NOTATION_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as c_int,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(notation4 as PROLOG_HANDLER);
            return XML_ROLE_NOTATION_PUBLIC_ID as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as c_int,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_NOTATION_NONE as c_int;
            return XML_ROLE_NOTATION_SYSTEM_ID as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as c_int,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_NOTATION_NONE as c_int;
            return XML_ROLE_NOTATION_SYSTEM_ID as c_int;
        }
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE_NOTATION_NO_SYSTEM_ID as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as c_int,
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(attlist1 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_ELEMENT_NAME as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as c_int,
        super::xmltok::XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE_ATTLIST_NONE as c_int;
        }
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(attlist2 as PROLOG_HANDLER);
            return XML_ROLE_ATTRIBUTE_NAME as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as c_int,
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
            i = 0 as c_int;
            while i
                < (::std::mem::size_of::<[*const c_char; 8]>() as c_ulong)
                    .wrapping_div(::std::mem::size_of::<*const c_char>() as c_ulong)
                    as c_int
            {
                if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                    enc,
                    ptr,
                    end,
                    types[i as usize],
                ) != 0
                {
                    (*state).handler = Some(attlist8 as PROLOG_HANDLER);
                    return XML_ROLE_ATTRIBUTE_TYPE_CDATA as c_int + i;
                }
                i += 1
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_NOTATION.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(attlist5 as PROLOG_HANDLER);
                return XML_ROLE_ATTLIST_NONE as c_int;
            }
        }
        super::xmltok::XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(attlist3 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as c_int,
        super::xmltok::XML_TOK_NMTOKEN
        | super::xmltok::XML_TOK_NAME
        | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(attlist4 as PROLOG_HANDLER);
            return XML_ROLE_ATTRIBUTE_ENUM_VALUE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as c_int,
        super::xmltok::XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(attlist8 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE as c_int;
        }
        super::xmltok::XML_TOK_OR => {
            (*state).handler = Some(attlist3 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as c_int,
        super::xmltok::XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(attlist6 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as c_int,
        super::xmltok::XML_TOK_NAME => {
            (*state).handler = Some(attlist7 as PROLOG_HANDLER);
            return XML_ROLE_ATTRIBUTE_NOTATION_VALUE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as c_int,
        super::xmltok::XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(attlist8 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE as c_int;
        }
        super::xmltok::XML_TOK_OR => {
            (*state).handler = Some(attlist6 as PROLOG_HANDLER);
            return XML_ROLE_ATTLIST_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as c_int,
        super::xmltok::XML_TOK_POUND_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                KW_IMPLIED.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(attlist1 as PROLOG_HANDLER);
                return XML_ROLE_IMPLIED_ATTRIBUTE_VALUE as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                KW_REQUIRED.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(attlist1 as PROLOG_HANDLER);
                return XML_ROLE_REQUIRED_ATTRIBUTE_VALUE as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                KW_FIXED.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(attlist9 as PROLOG_HANDLER);
                return XML_ROLE_ATTLIST_NONE as c_int;
            }
        }
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(attlist1 as PROLOG_HANDLER);
            return XML_ROLE_DEFAULT_ATTRIBUTE_VALUE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as c_int,
        super::xmltok::XML_TOK_LITERAL => {
            (*state).handler = Some(attlist1 as PROLOG_HANDLER);
            return XML_ROLE_FIXED_ATTRIBUTE_VALUE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as c_int,
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(element1 as PROLOG_HANDLER);
            return XML_ROLE_ELEMENT_NAME as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as c_int,
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_EMPTY.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE as c_int;
                return XML_ROLE_CONTENT_EMPTY as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_ANY.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE as c_int;
                return XML_ROLE_CONTENT_ANY as c_int;
            }
        }
        super::xmltok::XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(element2 as PROLOG_HANDLER);
            (*state).level = 1 as c_int as c_uint;
            return XML_ROLE_GROUP_OPEN as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as c_int,
        super::xmltok::XML_TOK_POUND_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                KW_PCDATA.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(element3 as PROLOG_HANDLER);
                return XML_ROLE_CONTENT_PCDATA as c_int;
            }
        }
        super::xmltok::XML_TOK_OPEN_PAREN => {
            (*state).level = 2 as c_int as c_uint;
            (*state).handler = Some(element6 as PROLOG_HANDLER);
            return XML_ROLE_GROUP_OPEN as c_int;
        }
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT as c_int;
        }
        super::xmltok::XML_TOK_NAME_QUESTION => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_OPT as c_int;
        }
        super::xmltok::XML_TOK_NAME_ASTERISK => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_REP as c_int;
        }
        super::xmltok::XML_TOK_NAME_PLUS => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_PLUS as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as c_int,
        super::xmltok::XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ELEMENT_NONE as c_int;
            return XML_ROLE_GROUP_CLOSE as c_int;
        }
        super::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ELEMENT_NONE as c_int;
            return XML_ROLE_GROUP_CLOSE_REP as c_int;
        }
        super::xmltok::XML_TOK_OR => {
            (*state).handler = Some(element4 as PROLOG_HANDLER);
            return XML_ROLE_ELEMENT_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as c_int,
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(element5 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as c_int,
        super::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).handler = Some(declClose as PROLOG_HANDLER);
            (*state).role_none = XML_ROLE_ELEMENT_NONE as c_int;
            return XML_ROLE_GROUP_CLOSE_REP as c_int;
        }
        super::xmltok::XML_TOK_OR => {
            (*state).handler = Some(element4 as PROLOG_HANDLER);
            return XML_ROLE_ELEMENT_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as c_int,
        super::xmltok::XML_TOK_OPEN_PAREN => {
            (*state).level = (*state).level.wrapping_add(1 as c_int as c_uint);
            return XML_ROLE_GROUP_OPEN as c_int;
        }
        super::xmltok::XML_TOK_NAME | super::xmltok::XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT as c_int;
        }
        super::xmltok::XML_TOK_NAME_QUESTION => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_OPT as c_int;
        }
        super::xmltok::XML_TOK_NAME_ASTERISK => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_REP as c_int;
        }
        super::xmltok::XML_TOK_NAME_PLUS => {
            (*state).handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE_CONTENT_ELEMENT_PLUS as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as c_int,
        super::xmltok::XML_TOK_CLOSE_PAREN => {
            (*state).level = (*state).level.wrapping_sub(1 as c_int as c_uint);
            if (*state).level == 0 as c_int as c_uint {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE as c_int
            }
            return XML_ROLE_GROUP_CLOSE as c_int;
        }
        super::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).level = (*state).level.wrapping_sub(1 as c_int as c_uint);
            if (*state).level == 0 as c_int as c_uint {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE as c_int
            }
            return XML_ROLE_GROUP_CLOSE_REP as c_int;
        }
        super::xmltok::XML_TOK_CLOSE_PAREN_QUESTION => {
            (*state).level = (*state).level.wrapping_sub(1 as c_int as c_uint);
            if (*state).level == 0 as c_int as c_uint {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE as c_int
            }
            return XML_ROLE_GROUP_CLOSE_OPT as c_int;
        }
        super::xmltok::XML_TOK_CLOSE_PAREN_PLUS => {
            (*state).level = (*state).level.wrapping_sub(1 as c_int as c_uint);
            if (*state).level == 0 as c_int as c_uint {
                (*state).handler = Some(declClose as PROLOG_HANDLER);
                (*state).role_none = XML_ROLE_ELEMENT_NONE as c_int
            }
            return XML_ROLE_GROUP_CLOSE_PLUS as c_int;
        }
        super::xmltok::XML_TOK_COMMA => {
            (*state).handler = Some(element6 as PROLOG_HANDLER);
            return XML_ROLE_GROUP_SEQUENCE as c_int;
        }
        super::xmltok::XML_TOK_OR => {
            (*state).handler = Some(element6 as PROLOG_HANDLER);
            return XML_ROLE_GROUP_CHOICE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as c_int,
        super::xmltok::XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_INCLUDE.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(condSect1 as PROLOG_HANDLER);
                return XML_ROLE_NONE as c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                KW_IGNORE.as_ptr(),
            ) != 0
            {
                (*state).handler = Some(condSect2 as PROLOG_HANDLER);
                return XML_ROLE_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as c_int,
        super::xmltok::XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(externalSubset1 as PROLOG_HANDLER);
            (*state).includeLevel = (*state).includeLevel.wrapping_add(1 as c_int as c_uint);
            return XML_ROLE_NONE as c_int;
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
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as c_int,
        super::xmltok::XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(externalSubset1 as PROLOG_HANDLER);
            return XML_ROLE_IGNORE_SECT as c_int;
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
    return XML_ROLE_NONE as c_int;
}
/* LCOV_EXCL_STOP */

unsafe extern "C" fn common(mut state: *mut PROLOG_STATE, mut tok: c_int) -> c_int {
    if (*state).documentEntity == 0 && tok == super::xmltok::XML_TOK_PARAM_ENTITY_REF {
        return XML_ROLE_INNER_PARAM_ENTITY_REF as c_int;
    }
    (*state).handler = Some(error as PROLOG_HANDLER);
    return XML_ROLE_ERROR as c_int;
}
#[no_mangle]

pub unsafe extern "C" fn XmlPrologStateInit(mut state: *mut PROLOG_STATE) {
    (*state).handler = Some(prolog0 as PROLOG_HANDLER);
    (*state).documentEntity = 1 as c_int;
    (*state).includeLevel = 0 as c_int as c_uint;
    (*state).inEntityValue = 0 as c_int;
    /* XML_DTD */
}
#[no_mangle]

pub unsafe extern "C" fn XmlPrologStateInitExternalEntity(mut state: *mut PROLOG_STATE) {
    (*state).handler = Some(externalSubset0 as PROLOG_HANDLER);
    (*state).documentEntity = 0 as c_int;
    (*state).includeLevel = 0 as c_int as c_uint;
}
/* XML_DTD */
