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

use super::xmlparse::ExpatBufRef;
use libc::{c_char, c_int, c_uint};

#[repr(i32)]
#[derive(Clone, Copy, PartialEq)]
pub enum XML_ROLE {
    ERROR = -1,
    NONE = 0,
    XML_DECL = 1,
    INSTANCE_START = 2,
    DOCTYPE_NONE = 3,
    DOCTYPE_NAME = 4,
    DOCTYPE_SYSTEM_ID = 5,
    DOCTYPE_PUBLIC_ID = 6,
    DOCTYPE_INTERNAL_SUBSET = 7,
    DOCTYPE_CLOSE = 8,
    GENERAL_ENTITY_NAME = 9,
    PARAM_ENTITY_NAME = 10,
    ENTITY_NONE = 11,
    ENTITY_VALUE = 12,
    ENTITY_SYSTEM_ID = 13,
    ENTITY_PUBLIC_ID = 14,
    ENTITY_COMPLETE = 15,
    ENTITY_NOTATION_NAME = 16,
    NOTATION_NONE = 17,
    NOTATION_NAME = 18,
    NOTATION_SYSTEM_ID = 19,
    NOTATION_NO_SYSTEM_ID = 20,
    NOTATION_PUBLIC_ID = 21,
    ATTRIBUTE_NAME = 22,
    ATTRIBUTE_TYPE_CDATA = 23,
    ATTRIBUTE_TYPE_ID = 24,
    ATTRIBUTE_TYPE_IDREF = 25,
    ATTRIBUTE_TYPE_IDREFS = 26,
    ATTRIBUTE_TYPE_ENTITY = 27,
    ATTRIBUTE_TYPE_ENTITIES = 28,
    ATTRIBUTE_TYPE_NMTOKEN = 29,
    ATTRIBUTE_TYPE_NMTOKENS = 30,
    ATTRIBUTE_ENUM_VALUE = 31,
    ATTRIBUTE_NOTATION_VALUE = 32,
    ATTLIST_NONE = 33,
    ATTLIST_ELEMENT_NAME = 34,
    IMPLIED_ATTRIBUTE_VALUE = 35,
    REQUIRED_ATTRIBUTE_VALUE = 36,
    DEFAULT_ATTRIBUTE_VALUE = 37,
    FIXED_ATTRIBUTE_VALUE = 38,
    ELEMENT_NONE = 39,
    ELEMENT_NAME = 40,
    CONTENT_ANY = 41,
    CONTENT_EMPTY = 42,
    CONTENT_PCDATA = 43,
    GROUP_OPEN = 44,
    GROUP_CLOSE = 45,
    GROUP_CLOSE_REP = 46,
    GROUP_CLOSE_OPT = 47,
    GROUP_CLOSE_PLUS = 48,
    GROUP_CHOICE = 49,
    GROUP_SEQUENCE = 50,
    CONTENT_ELEMENT = 51,
    CONTENT_ELEMENT_REP = 52,
    CONTENT_ELEMENT_OPT = 53,
    CONTENT_ELEMENT_PLUS = 54,
    PI = 55,
    COMMENT = 56,
    TEXT_DECL = 57,
    IGNORE_SECT = 58,
    INNER_PARAM_ENTITY_REF = 59,
/* XML_DTD */
/* XML_DTD */

    PARAM_ENTITY_REF = 60,
}

impl Default for XML_ROLE {
    fn default() -> Self {
        XML_ROLE::NONE
    }
}

pub type PROLOG_STATE = prolog_state;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct prolog_state {
    pub handler: Option<
        fn(_: &mut prolog_state, _: XML_TOK, _: ExpatBufRef, _: &super::xmltok::ENCODING) -> XML_ROLE,
    >,
    pub level: c_uint,
    pub role_none: XML_ROLE,
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
pub use crate::lib::xmltok::{
    position, XML_Convert_Result, ATTRIBUTE, ENCODING, POSITION, XML_Convert_Result::COMPLETED,
    XML_Convert_Result::INPUT_INCOMPLETE, XML_Convert_Result::OUTPUT_EXHAUSTED, XML_TOK
};
use libc;
/* not XML_DTD */
/* not XML_DTD */

pub type PROLOG_HANDLER =
    fn(_: &mut PROLOG_STATE, _: XML_TOK, _: ExpatBufRef, _: &super::xmltok::ENCODING) -> XML_ROLE;
/* ndef _WIN32 */
/* Doesn't check:

 that ,| are not mixed in a model group
 content of literals

*/

static KW_ANY: [c_char; 3] = [ASCII_A, ASCII_N, ASCII_Y];

static KW_ATTLIST: [c_char; 7] = [
    ASCII_A,
    ASCII_T,
    ASCII_T,
    ASCII_L,
    ASCII_I,
    ASCII_S,
    ASCII_T,
];

static KW_CDATA: [c_char; 5] = [
    ASCII_C,
    ASCII_D,
    ASCII_A,
    ASCII_T,
    ASCII_A,
];

static KW_DOCTYPE: [c_char; 7] = [
    ASCII_D,
    ASCII_O,
    ASCII_C,
    ASCII_T,
    ASCII_Y,
    ASCII_P,
    ASCII_E,
];

static KW_ELEMENT: [c_char; 7] = [
    ASCII_E,
    ASCII_L,
    ASCII_E,
    ASCII_M,
    ASCII_E,
    ASCII_N,
    ASCII_T,
];

static KW_EMPTY: [c_char; 5] = [
    ASCII_E,
    ASCII_M,
    ASCII_P,
    ASCII_T,
    ASCII_Y,
];

static KW_ENTITIES: [c_char; 8] = [
    ASCII_E,
    ASCII_N,
    ASCII_T,
    ASCII_I,
    ASCII_T,
    ASCII_I,
    ASCII_E,
    ASCII_S,
];

static KW_ENTITY: [c_char; 6] = [
    ASCII_E,
    ASCII_N,
    ASCII_T,
    ASCII_I,
    ASCII_T,
    ASCII_Y,
];

static KW_FIXED: [c_char; 5] = [
    ASCII_F,
    ASCII_I,
    ASCII_X,
    ASCII_E,
    ASCII_D,
];

static KW_ID: [c_char; 2] = [ASCII_I, ASCII_D];

static KW_IDREF: [c_char; 5] = [
    ASCII_I,
    ASCII_D,
    ASCII_R,
    ASCII_E,
    ASCII_F,
];

static KW_IDREFS: [c_char; 6] = [
    ASCII_I,
    ASCII_D,
    ASCII_R,
    ASCII_E,
    ASCII_F,
    ASCII_S,
];

static KW_IGNORE: [c_char; 6] = [
    ASCII_I,
    ASCII_G,
    ASCII_N,
    ASCII_O,
    ASCII_R,
    ASCII_E,
];

static KW_IMPLIED: [c_char; 7] = [
    ASCII_I,
    ASCII_M,
    ASCII_P,
    ASCII_L,
    ASCII_I,
    ASCII_E,
    ASCII_D,
];

static KW_INCLUDE: [c_char; 7] = [
    ASCII_I,
    ASCII_N,
    ASCII_C,
    ASCII_L,
    ASCII_U,
    ASCII_D,
    ASCII_E,
];

static KW_NDATA: [c_char; 5] = [
    ASCII_N,
    ASCII_D,
    ASCII_A,
    ASCII_T,
    ASCII_A,
];

static KW_NMTOKEN: [c_char; 7] = [
    ASCII_N,
    ASCII_M,
    ASCII_T,
    ASCII_O,
    ASCII_K,
    ASCII_E,
    ASCII_N,
];

static KW_NMTOKENS: [c_char; 8] = [
    ASCII_N,
    ASCII_M,
    ASCII_T,
    ASCII_O,
    ASCII_K,
    ASCII_E,
    ASCII_N,
    ASCII_S,
];

static KW_NOTATION: [c_char; 8] = [
    ASCII_N,
    ASCII_O,
    ASCII_T,
    ASCII_A,
    ASCII_T,
    ASCII_I,
    ASCII_O,
    ASCII_N,
];

static KW_PCDATA: [c_char; 6] = [
    ASCII_P,
    ASCII_C,
    ASCII_D,
    ASCII_A,
    ASCII_T,
    ASCII_A,
];

static KW_PUBLIC: [c_char; 6] = [
    ASCII_P,
    ASCII_U,
    ASCII_B,
    ASCII_L,
    ASCII_I,
    ASCII_C,
];

static KW_REQUIRED: [c_char; 8] = [
    ASCII_R,
    ASCII_E,
    ASCII_Q,
    ASCII_U,
    ASCII_I,
    ASCII_R,
    ASCII_E,
    ASCII_D,
];

static KW_SYSTEM: [c_char; 6] = [
    ASCII_S,
    ASCII_Y,
    ASCII_S,
    ASCII_T,
    ASCII_E,
    ASCII_M,
];

fn prolog0(
    mut state: &mut PROLOG_STATE,
    mut tok:XML_TOK,
    mut buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => {
            state.handler = Some(prolog1 as PROLOG_HANDLER); /* LCOV_EXCL_LINE */
            return XML_ROLE::NONE;
        }
        super::xmltok::XML_TOK::XML_DECL => {
            state.handler = Some(prolog1 as PROLOG_HANDLER);
            return XML_ROLE::XML_DECL;
        }
        super::xmltok::XML_TOK::PI => {
            state.handler = Some(prolog1 as PROLOG_HANDLER);
            return XML_ROLE::PI;
        }
        super::xmltok::XML_TOK::COMMENT => {
            state.handler = Some(prolog1 as PROLOG_HANDLER);
            return XML_ROLE::COMMENT;
        }
        super::xmltok::XML_TOK::BOM => return XML_ROLE::NONE,
        super::xmltok::XML_TOK::DECL_OPEN => {
            if enc.nameMatchesAscii(
                buf.inc_start((2 * enc.minBytesPerChar()) as isize),
                &KW_DOCTYPE)
            {
                state.handler = Some(doctype0 as PROLOG_HANDLER);
                return XML_ROLE::DOCTYPE_NONE;
            }
        }
        XML_TOK::INSTANCE_START => {
            state.handler = Some(error as PROLOG_HANDLER);
            return XML_ROLE::INSTANCE_START;
        }
        _ => {}
    }
    common(state, tok)
}

fn prolog1(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NONE,
        super::xmltok::XML_TOK::PI => return XML_ROLE::PI,
        super::xmltok::XML_TOK::COMMENT => return XML_ROLE::COMMENT,
        super::xmltok::XML_TOK::BOM => {
            /* This case can never arise.  To reach this role function, the
             * parse must have passed through prolog0 and therefore have had
             * some form of input, even if only a space.  At that point, a
             * byte order mark is no longer a valid character (though
             * technically it should be interpreted as a non-breaking space),
             * so will be rejected by the tokenizing stages.
             */
            return XML_ROLE::NONE;
        }
        super::xmltok::XML_TOK::DECL_OPEN => {
            if enc.nameMatchesAscii(
                buf.inc_start((2 * enc.minBytesPerChar()) as isize),
                &KW_DOCTYPE)
            {
                state.handler = Some(doctype0 as PROLOG_HANDLER);
                return XML_ROLE::DOCTYPE_NONE;
            }
        }
        XML_TOK::INSTANCE_START => {
            state.handler = Some(error as PROLOG_HANDLER);
            return XML_ROLE::INSTANCE_START;
        }
        _ => {}
    }
    common(state, tok)
}

fn prolog2(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NONE,
        XML_TOK::PI => return XML_ROLE::PI,
        XML_TOK::COMMENT => return XML_ROLE::COMMENT,
        XML_TOK::INSTANCE_START => {
            state.handler = Some(error as PROLOG_HANDLER);
            return XML_ROLE::INSTANCE_START;
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype0(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::DOCTYPE_NONE,
        super::xmltok::XML_TOK::NAME | super::xmltok::XML_TOK::PREFIXED_NAME => {
            state.handler = Some(doctype1 as PROLOG_HANDLER);
            return XML_ROLE::DOCTYPE_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype1(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::DOCTYPE_NONE,
        super::xmltok::XML_TOK::OPEN_BRACKET => {
            state.handler = Some(internalSubset as PROLOG_HANDLER);
            return XML_ROLE::DOCTYPE_INTERNAL_SUBSET;
        }
        super::xmltok::XML_TOK::DECL_CLOSE => {
            state.handler = Some(prolog2 as PROLOG_HANDLER);
            return XML_ROLE::DOCTYPE_CLOSE;
        }
        super::xmltok::XML_TOK::NAME => {
            if enc.nameMatchesAscii(buf, &KW_SYSTEM) {
                state.handler = Some(doctype3 as PROLOG_HANDLER);
                return XML_ROLE::DOCTYPE_NONE;
            }
            if enc.nameMatchesAscii(buf, &KW_PUBLIC) {
                state.handler = Some(doctype2 as PROLOG_HANDLER);
                return XML_ROLE::DOCTYPE_NONE;
            }
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype2(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::DOCTYPE_NONE,
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(doctype3 as PROLOG_HANDLER);
            return XML_ROLE::DOCTYPE_PUBLIC_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype3(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::DOCTYPE_NONE,
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(doctype4 as PROLOG_HANDLER);
            return XML_ROLE::DOCTYPE_SYSTEM_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype4(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::DOCTYPE_NONE,
        super::xmltok::XML_TOK::OPEN_BRACKET => {
            state.handler = Some(internalSubset as PROLOG_HANDLER);
            return XML_ROLE::DOCTYPE_INTERNAL_SUBSET;
        }
        super::xmltok::XML_TOK::DECL_CLOSE => {
            state.handler = Some(prolog2 as PROLOG_HANDLER);
            return XML_ROLE::DOCTYPE_CLOSE;
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype5(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::DOCTYPE_NONE,
        super::xmltok::XML_TOK::DECL_CLOSE => {
            state.handler = Some(prolog2 as PROLOG_HANDLER);
            return XML_ROLE::DOCTYPE_CLOSE;
        }
        _ => {}
    }
    common(state, tok)
}

fn internalSubset(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NONE,
        super::xmltok::XML_TOK::DECL_OPEN => {
            if enc.nameMatchesAscii(
                buf.inc_start((2 * enc.minBytesPerChar()) as isize),
                &KW_ENTITY)
            {
                state.handler = Some(entity0 as PROLOG_HANDLER);
                return XML_ROLE::ENTITY_NONE;
            }
            if enc.nameMatchesAscii(
                buf.inc_start((2 * enc.minBytesPerChar()) as isize),
                &KW_ATTLIST)
            {
                state.handler = Some(attlist0 as PROLOG_HANDLER);
                return XML_ROLE::ATTLIST_NONE;
            }
            if enc.nameMatchesAscii(
                buf.inc_start((2 * enc.minBytesPerChar()) as isize),
                &KW_ELEMENT,
            )
            {
                state.handler = Some(element0 as PROLOG_HANDLER);
                return XML_ROLE::ELEMENT_NONE;
            }
            if enc.nameMatchesAscii(
                buf.inc_start((2 * enc.minBytesPerChar()) as isize),
                &KW_NOTATION)
            {
                state.handler = Some(notation0 as PROLOG_HANDLER);
                return XML_ROLE::NOTATION_NONE;
            }
        }
        super::xmltok::XML_TOK::PI => return XML_ROLE::PI,
        super::xmltok::XML_TOK::COMMENT => return XML_ROLE::COMMENT,
        super::xmltok::XML_TOK::PARAM_ENTITY_REF => return XML_ROLE::PARAM_ENTITY_REF,
        super::xmltok::XML_TOK::CLOSE_BRACKET => {
            state.handler = Some(doctype5 as PROLOG_HANDLER);
            return XML_ROLE::DOCTYPE_NONE;
        }
        super::xmltok::XML_TOK::NONE => return XML_ROLE::NONE,
        _ => {}
    }
    common(state, tok)
}

fn externalSubset0(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    state.handler = Some(externalSubset1 as PROLOG_HANDLER);
    if tok == super::xmltok::XML_TOK::XML_DECL {
        return XML_ROLE::TEXT_DECL;
    }
    externalSubset1(state, tok, buf, enc)
}

fn externalSubset1(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        super::xmltok::XML_TOK::COND_SECT_OPEN => {
            state.handler = Some(condSect0 as PROLOG_HANDLER);
            return XML_ROLE::NONE;
        }
        super::xmltok::XML_TOK::COND_SECT_CLOSE => {
            if !(state.includeLevel == 0) {
                state.includeLevel = state.includeLevel.wrapping_sub(1);
                return XML_ROLE::NONE;
            }
        }
        XML_TOK::PROLOG_S => return XML_ROLE::NONE,
        super::xmltok::XML_TOK::CLOSE_BRACKET => {}
        super::xmltok::XML_TOK::NONE => {
            if !(state.includeLevel != 0) {
                return XML_ROLE::NONE;
            }
        }
        _ => return internalSubset(state, tok, buf, enc),
    }
    common(state, tok)
}
/* XML_DTD */

fn entity0(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::PERCENT => {
            state.handler = Some(entity1 as PROLOG_HANDLER);
            return XML_ROLE::ENTITY_NONE;
        }
        super::xmltok::XML_TOK::NAME => {
            state.handler = Some(entity2 as PROLOG_HANDLER);
            return XML_ROLE::GENERAL_ENTITY_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity1(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::NAME => {
            state.handler = Some(entity7 as PROLOG_HANDLER);
            return XML_ROLE::PARAM_ENTITY_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity2(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::NAME => {
            if enc.nameMatchesAscii(buf, &KW_SYSTEM) {
                state.handler = Some(entity4 as PROLOG_HANDLER);
                return XML_ROLE::ENTITY_NONE;
            }
            if enc.nameMatchesAscii(buf, &KW_PUBLIC) {
                state.handler = Some(entity3 as PROLOG_HANDLER);
                return XML_ROLE::ENTITY_NONE;
            }
        }
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(declClose as PROLOG_HANDLER);
            state.role_none = XML_ROLE::ENTITY_NONE;
            return XML_ROLE::ENTITY_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity3(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(entity4 as PROLOG_HANDLER);
            return XML_ROLE::ENTITY_PUBLIC_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity4(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(entity5 as PROLOG_HANDLER);
            return XML_ROLE::ENTITY_SYSTEM_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity5(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::DECL_CLOSE => {
            state.handler = if state.documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE::ENTITY_COMPLETE;
        }
        super::xmltok::XML_TOK::NAME => {
            if enc.nameMatchesAscii(buf, &KW_NDATA) {
                state.handler = Some(entity6 as PROLOG_HANDLER);
                return XML_ROLE::ENTITY_NONE;
            }
        }
        _ => {}
    }
    common(state, tok)
}

fn entity6(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::NAME => {
            state.handler = Some(declClose as PROLOG_HANDLER);
            state.role_none = XML_ROLE::ENTITY_NONE;
            return XML_ROLE::ENTITY_NOTATION_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity7(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::NAME => {
            if enc.nameMatchesAscii(buf, &KW_SYSTEM) {
                state.handler = Some(entity9 as PROLOG_HANDLER);
                return XML_ROLE::ENTITY_NONE;
            }
            if enc.nameMatchesAscii(buf, &KW_PUBLIC) {
                state.handler = Some(entity8 as PROLOG_HANDLER);
                return XML_ROLE::ENTITY_NONE;
            }
        }
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(declClose as PROLOG_HANDLER);
            state.role_none = XML_ROLE::ENTITY_NONE;
            return XML_ROLE::ENTITY_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity8(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(entity9 as PROLOG_HANDLER);
            return XML_ROLE::ENTITY_PUBLIC_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity9(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(entity10 as PROLOG_HANDLER);
            return XML_ROLE::ENTITY_SYSTEM_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity10(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ENTITY_NONE,
        super::xmltok::XML_TOK::DECL_CLOSE => {
            state.handler = if state.documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE::ENTITY_COMPLETE;
        }
        _ => {}
    }
    common(state, tok)
}

fn notation0(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NOTATION_NONE,
        super::xmltok::XML_TOK::NAME => {
            state.handler = Some(notation1 as PROLOG_HANDLER);
            return XML_ROLE::NOTATION_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn notation1(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NOTATION_NONE,
        super::xmltok::XML_TOK::NAME => {
            if enc.nameMatchesAscii(buf, &KW_SYSTEM) {
                state.handler = Some(notation3 as PROLOG_HANDLER);
                return XML_ROLE::NOTATION_NONE;
            }
            if enc.nameMatchesAscii(buf, &KW_PUBLIC) {
                state.handler = Some(notation2 as PROLOG_HANDLER);
                return XML_ROLE::NOTATION_NONE;
            }
        }
        _ => {}
    }
    common(state, tok)
}

fn notation2(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NOTATION_NONE,
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(notation4 as PROLOG_HANDLER);
            return XML_ROLE::NOTATION_PUBLIC_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn notation3(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NOTATION_NONE,
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(declClose as PROLOG_HANDLER);
            state.role_none = XML_ROLE::NOTATION_NONE;
            return XML_ROLE::NOTATION_SYSTEM_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn notation4(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NOTATION_NONE,
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(declClose as PROLOG_HANDLER);
            state.role_none = XML_ROLE::NOTATION_NONE;
            return XML_ROLE::NOTATION_SYSTEM_ID;
        }
        super::xmltok::XML_TOK::DECL_CLOSE => {
            state.handler = if state.documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE::NOTATION_NO_SYSTEM_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist0(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ATTLIST_NONE,
        super::xmltok::XML_TOK::NAME | super::xmltok::XML_TOK::PREFIXED_NAME => {
            state.handler = Some(attlist1 as PROLOG_HANDLER);
            return XML_ROLE::ATTLIST_ELEMENT_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist1(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ATTLIST_NONE,
        super::xmltok::XML_TOK::DECL_CLOSE => {
            state.handler = if state.documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return XML_ROLE::ATTLIST_NONE;
        }
        super::xmltok::XML_TOK::NAME | super::xmltok::XML_TOK::PREFIXED_NAME => {
            state.handler = Some(attlist2 as PROLOG_HANDLER);
            return XML_ROLE::ATTRIBUTE_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist2(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ATTLIST_NONE,
        super::xmltok::XML_TOK::NAME => {
            static TYPES: [&[c_char]; 8] = [
                &KW_CDATA,
                &KW_ID,
                &KW_IDREF,
                &KW_IDREFS,
                &KW_ENTITY,
                &KW_ENTITIES,
                &KW_NMTOKEN,
                &KW_NMTOKENS,
            ];
            for i in 0..TYPES.len() {
                if enc.nameMatchesAscii(buf, &TYPES[i]) {
                    state.handler = Some(attlist8 as PROLOG_HANDLER);
                    return match i {
                        0 => XML_ROLE::ATTRIBUTE_TYPE_CDATA,
                        1 => XML_ROLE::ATTRIBUTE_TYPE_ID,
                        2 => XML_ROLE::ATTRIBUTE_TYPE_IDREF,
                        3 => XML_ROLE::ATTRIBUTE_TYPE_IDREFS,
                        4 => XML_ROLE::ATTRIBUTE_TYPE_ENTITY,
                        5 => XML_ROLE::ATTRIBUTE_TYPE_ENTITIES,
                        6 => XML_ROLE::ATTRIBUTE_TYPE_NMTOKEN,
                        7 => XML_ROLE::ATTRIBUTE_TYPE_NMTOKENS,
                        _ => panic!(),
                    };
                }
            }
            if enc.nameMatchesAscii(buf, &KW_NOTATION) {
                state.handler = Some(attlist5 as PROLOG_HANDLER);
                return XML_ROLE::ATTLIST_NONE;
            }
        }
        super::xmltok::XML_TOK::OPEN_PAREN => {
            state.handler = Some(attlist3 as PROLOG_HANDLER);
            return XML_ROLE::ATTLIST_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist3(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ATTLIST_NONE,
        super::xmltok::XML_TOK::NMTOKEN
        | super::xmltok::XML_TOK::NAME
        | super::xmltok::XML_TOK::PREFIXED_NAME => {
            state.handler = Some(attlist4 as PROLOG_HANDLER);
            return XML_ROLE::ATTRIBUTE_ENUM_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist4(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ATTLIST_NONE,
        super::xmltok::XML_TOK::CLOSE_PAREN => {
            state.handler = Some(attlist8 as PROLOG_HANDLER);
            return XML_ROLE::ATTLIST_NONE;
        }
        super::xmltok::XML_TOK::OR => {
            state.handler = Some(attlist3 as PROLOG_HANDLER);
            return XML_ROLE::ATTLIST_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist5(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ATTLIST_NONE,
        super::xmltok::XML_TOK::OPEN_PAREN => {
            state.handler = Some(attlist6 as PROLOG_HANDLER);
            return XML_ROLE::ATTLIST_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist6(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ATTLIST_NONE,
        super::xmltok::XML_TOK::NAME => {
            state.handler = Some(attlist7 as PROLOG_HANDLER);
            return XML_ROLE::ATTRIBUTE_NOTATION_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist7(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ATTLIST_NONE,
        super::xmltok::XML_TOK::CLOSE_PAREN => {
            state.handler = Some(attlist8 as PROLOG_HANDLER);
            return XML_ROLE::ATTLIST_NONE;
        }
        super::xmltok::XML_TOK::OR => {
            state.handler = Some(attlist6 as PROLOG_HANDLER);
            return XML_ROLE::ATTLIST_NONE;
        }
        _ => {}
    }
    common(state, tok)
}
/* default value */

fn attlist8(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ATTLIST_NONE,
        super::xmltok::XML_TOK::POUND_NAME => {
            if enc.nameMatchesAscii(
                buf.inc_start((enc.minBytesPerChar()) as isize),
                &KW_IMPLIED,
            )
            {
                state.handler = Some(attlist1 as PROLOG_HANDLER);
                return XML_ROLE::IMPLIED_ATTRIBUTE_VALUE;
            }
            if enc.nameMatchesAscii(
                buf.inc_start((enc.minBytesPerChar()) as isize),
                &KW_REQUIRED,
            )
            {
                state.handler = Some(attlist1 as PROLOG_HANDLER);
                return XML_ROLE::REQUIRED_ATTRIBUTE_VALUE;
            }
            if enc.nameMatchesAscii(
                buf.inc_start((enc.minBytesPerChar()) as isize),
                &KW_FIXED,
            )
            {
                state.handler = Some(attlist9 as PROLOG_HANDLER);
                return XML_ROLE::ATTLIST_NONE;
            }
        }
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(attlist1 as PROLOG_HANDLER);
            return XML_ROLE::DEFAULT_ATTRIBUTE_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist9(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ATTLIST_NONE,
        super::xmltok::XML_TOK::LITERAL => {
            state.handler = Some(attlist1 as PROLOG_HANDLER);
            return XML_ROLE::FIXED_ATTRIBUTE_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn element0(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ELEMENT_NONE,
        super::xmltok::XML_TOK::NAME | super::xmltok::XML_TOK::PREFIXED_NAME => {
            state.handler = Some(element1 as PROLOG_HANDLER);
            return XML_ROLE::ELEMENT_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn element1(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ELEMENT_NONE,
        super::xmltok::XML_TOK::NAME => {
            if enc.nameMatchesAscii(buf, &KW_EMPTY) {
                state.handler = Some(declClose as PROLOG_HANDLER);
                state.role_none = XML_ROLE::ELEMENT_NONE;
                return XML_ROLE::CONTENT_EMPTY;
            }
            if enc.nameMatchesAscii(buf, &KW_ANY) {
                state.handler = Some(declClose as PROLOG_HANDLER);
                state.role_none = XML_ROLE::ELEMENT_NONE;
                return XML_ROLE::CONTENT_ANY;
            }
        }
        super::xmltok::XML_TOK::OPEN_PAREN => {
            state.handler = Some(element2 as PROLOG_HANDLER);
            state.level = 1;
            return XML_ROLE::GROUP_OPEN;
        }
        _ => {}
    }
    common(state, tok)
}

fn element2(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ELEMENT_NONE,
        super::xmltok::XML_TOK::POUND_NAME => {
            if enc.nameMatchesAscii(
                buf.inc_start((enc.minBytesPerChar()) as isize),
                &KW_PCDATA,
            )
            {
                state.handler = Some(element3 as PROLOG_HANDLER);
                return XML_ROLE::CONTENT_PCDATA;
            }
        }
        super::xmltok::XML_TOK::OPEN_PAREN => {
            state.level = 2;
            state.handler = Some(element6 as PROLOG_HANDLER);
            return XML_ROLE::GROUP_OPEN;
        }
        super::xmltok::XML_TOK::NAME | super::xmltok::XML_TOK::PREFIXED_NAME => {
            state.handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE::CONTENT_ELEMENT;
        }
        super::xmltok::XML_TOK::NAME_QUESTION => {
            state.handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE::CONTENT_ELEMENT_OPT;
        }
        super::xmltok::XML_TOK::NAME_ASTERISK => {
            state.handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE::CONTENT_ELEMENT_REP;
        }
        super::xmltok::XML_TOK::NAME_PLUS => {
            state.handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE::CONTENT_ELEMENT_PLUS;
        }
        _ => {}
    }
    common(state, tok)
}

fn element3(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ELEMENT_NONE,
        super::xmltok::XML_TOK::CLOSE_PAREN => {
            state.handler = Some(declClose as PROLOG_HANDLER);
            state.role_none = XML_ROLE::ELEMENT_NONE;
            return XML_ROLE::GROUP_CLOSE;
        }
        super::xmltok::XML_TOK::CLOSE_PAREN_ASTERISK => {
            state.handler = Some(declClose as PROLOG_HANDLER);
            state.role_none = XML_ROLE::ELEMENT_NONE;
            return XML_ROLE::GROUP_CLOSE_REP;
        }
        super::xmltok::XML_TOK::OR => {
            state.handler = Some(element4 as PROLOG_HANDLER);
            return XML_ROLE::ELEMENT_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn element4(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ELEMENT_NONE,
        super::xmltok::XML_TOK::NAME | super::xmltok::XML_TOK::PREFIXED_NAME => {
            state.handler = Some(element5 as PROLOG_HANDLER);
            return XML_ROLE::CONTENT_ELEMENT;
        }
        _ => {}
    }
    common(state, tok)
}

fn element5(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ELEMENT_NONE,
        super::xmltok::XML_TOK::CLOSE_PAREN_ASTERISK => {
            state.handler = Some(declClose as PROLOG_HANDLER);
            state.role_none = XML_ROLE::ELEMENT_NONE;
            return XML_ROLE::GROUP_CLOSE_REP;
        }
        super::xmltok::XML_TOK::OR => {
            state.handler = Some(element4 as PROLOG_HANDLER);
            return XML_ROLE::ELEMENT_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn element6(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ELEMENT_NONE,
        super::xmltok::XML_TOK::OPEN_PAREN => {
            state.level = state.level.wrapping_add(1);
            return XML_ROLE::GROUP_OPEN;
        }
        super::xmltok::XML_TOK::NAME | super::xmltok::XML_TOK::PREFIXED_NAME => {
            state.handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE::CONTENT_ELEMENT;
        }
        super::xmltok::XML_TOK::NAME_QUESTION => {
            state.handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE::CONTENT_ELEMENT_OPT;
        }
        super::xmltok::XML_TOK::NAME_ASTERISK => {
            state.handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE::CONTENT_ELEMENT_REP;
        }
        super::xmltok::XML_TOK::NAME_PLUS => {
            state.handler = Some(element7 as PROLOG_HANDLER);
            return XML_ROLE::CONTENT_ELEMENT_PLUS;
        }
        _ => {}
    }
    common(state, tok)
}

fn element7(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::ELEMENT_NONE,
        super::xmltok::XML_TOK::CLOSE_PAREN => {
            state.level = state.level.wrapping_sub(1);
            if state.level == 0 {
                state.handler = Some(declClose as PROLOG_HANDLER);
                state.role_none = XML_ROLE::ELEMENT_NONE
            }
            return XML_ROLE::GROUP_CLOSE;
        }
        super::xmltok::XML_TOK::CLOSE_PAREN_ASTERISK => {
            state.level = state.level.wrapping_sub(1);
            if state.level == 0 {
                state.handler = Some(declClose as PROLOG_HANDLER);
                state.role_none = XML_ROLE::ELEMENT_NONE
            }
            return XML_ROLE::GROUP_CLOSE_REP;
        }
        super::xmltok::XML_TOK::CLOSE_PAREN_QUESTION => {
            state.level = state.level.wrapping_sub(1);
            if state.level == 0 {
                state.handler = Some(declClose as PROLOG_HANDLER);
                state.role_none = XML_ROLE::ELEMENT_NONE
            }
            return XML_ROLE::GROUP_CLOSE_OPT;
        }
        super::xmltok::XML_TOK::CLOSE_PAREN_PLUS => {
            state.level = state.level.wrapping_sub(1);
            if state.level == 0 {
                state.handler = Some(declClose as PROLOG_HANDLER);
                state.role_none = XML_ROLE::ELEMENT_NONE
            }
            return XML_ROLE::GROUP_CLOSE_PLUS;
        }
        super::xmltok::XML_TOK::COMMA => {
            state.handler = Some(element6 as PROLOG_HANDLER);
            return XML_ROLE::GROUP_SEQUENCE;
        }
        super::xmltok::XML_TOK::OR => {
            state.handler = Some(element6 as PROLOG_HANDLER);
            return XML_ROLE::GROUP_CHOICE;
        }
        _ => {}
    }
    common(state, tok)
}

fn condSect0(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    buf: ExpatBufRef,
    mut enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NONE,
        super::xmltok::XML_TOK::NAME => {
            if enc.nameMatchesAscii(buf, &KW_INCLUDE) {
                state.handler = Some(condSect1 as PROLOG_HANDLER);
                return XML_ROLE::NONE;
            }
            if enc.nameMatchesAscii(buf, &KW_IGNORE) {
                state.handler = Some(condSect2 as PROLOG_HANDLER);
                return XML_ROLE::NONE;
            }
        }
        _ => {}
    }
    common(state, tok)
}

fn condSect1(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NONE,
        super::xmltok::XML_TOK::OPEN_BRACKET => {
            state.handler = Some(externalSubset1 as PROLOG_HANDLER);
            state.includeLevel = state.includeLevel.wrapping_add(1);
            return XML_ROLE::NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn condSect2(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return XML_ROLE::NONE,
        super::xmltok::XML_TOK::OPEN_BRACKET => {
            state.handler = Some(externalSubset1 as PROLOG_HANDLER);
            return XML_ROLE::IGNORE_SECT;
        }
        _ => {}
    }
    common(state, tok)
}
/* XML_DTD */

fn declClose(
    mut state: &mut PROLOG_STATE,
    mut tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    match tok {
        XML_TOK::PROLOG_S => return state.role_none,
        super::xmltok::XML_TOK::DECL_CLOSE => {
            state.handler = if state.documentEntity != 0 {
                Some(internalSubset as PROLOG_HANDLER)
            } else {
                Some(externalSubset1 as PROLOG_HANDLER)
            };
            return state.role_none;
        }
        _ => {}
    }
    common(state, tok)
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

fn error(
    mut _state: &mut PROLOG_STATE,
    mut _tok: XML_TOK,
    mut _buf: ExpatBufRef,
    mut _enc: &super::xmltok::ENCODING,
) -> XML_ROLE {
    XML_ROLE::NONE
}
/* LCOV_EXCL_STOP */

fn common(mut state: &mut PROLOG_STATE, mut tok: XML_TOK) -> XML_ROLE {
    if state.documentEntity == 0 && tok == super::xmltok::XML_TOK::PARAM_ENTITY_REF {
        return XML_ROLE::INNER_PARAM_ENTITY_REF;
    }
    state.handler = Some(error as PROLOG_HANDLER);
    XML_ROLE::ERROR
}
#[no_mangle]

pub fn XmlPrologStateInit(mut state: &mut PROLOG_STATE) {
    state.handler = Some(prolog0 as PROLOG_HANDLER);
    state.documentEntity = 1;
    state.includeLevel = 0;
    state.inEntityValue = 0;
    /* XML_DTD */
}
#[no_mangle]

pub fn XmlPrologStateInitExternalEntity(mut state: &mut PROLOG_STATE) {
    state.handler = Some(externalSubset0 as PROLOG_HANDLER);
    state.documentEntity = 0;
    state.includeLevel = 0;
}
/* XML_DTD */
