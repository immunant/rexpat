/* f519f27c7c3b79fee55aeb8b1e53b7384b079d9118bf3a62eb3a60986a6742f2 (2.2.9+)
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

pub use crate::ascii_h::{
    ASCII_a, ASCII_c, ASCII_e, ASCII_g, ASCII_h, ASCII_l, ASCII_m, ASCII_n, ASCII_o, ASCII_p,
    ASCII_r, ASCII_s, ASCII_t, ASCII_w, ASCII_x, ASCII_0, ASCII_1, ASCII_2, ASCII_3, ASCII_8,
    ASCII_9, ASCII_A, ASCII_C, ASCII_COLON, ASCII_COMMA, ASCII_D, ASCII_E, ASCII_EQUALS,
    ASCII_EXCL, ASCII_F, ASCII_FF, ASCII_HASH, ASCII_I, ASCII_K, ASCII_L, ASCII_LPAREN, ASCII_M,
    ASCII_N, ASCII_O, ASCII_PERIOD, ASCII_PIPE, ASCII_R, ASCII_RPAREN, ASCII_S, ASCII_SLASH,
    ASCII_T, ASCII_X, ASCII_Y,
};
pub use crate::expat_config_h::XML_CONTEXT_BYTES;
pub use crate::expat_external_h::{XML_Char, XML_Index, XML_LChar, XML_Size};
pub use crate::expat_h::{
    XML_AttlistDeclHandler, XML_Bool, XML_CharacterDataHandler, XML_CommentHandler, XML_Content,
    XML_Content_Quant, XML_Content_Type, XML_DefaultHandler, XML_ElementDeclHandler, XML_Encoding,
    XML_EndCdataSectionHandler, XML_EndDoctypeDeclHandler, XML_EndElementHandler,
    XML_EndNamespaceDeclHandler, XML_EntityDeclHandler, XML_Error, XML_Expat_Version,
    XML_ExternalEntityRefHandler, XML_Feature, XML_FeatureEnum, XML_Memory_Handling_Suite,
    XML_NotStandaloneHandler, XML_NotationDeclHandler, XML_ParamEntityParsing, XML_Parser,
    XML_Parsing, XML_ParsingStatus, XML_ProcessingInstructionHandler, XML_SkippedEntityHandler,
    XML_StartCdataSectionHandler, XML_StartDoctypeDeclHandler, XML_StartElementHandler,
    XML_StartNamespaceDeclHandler, XML_Status, XML_UnknownEncodingHandler,
    XML_UnparsedEntityDeclHandler, XML_XmlDeclHandler, XML_cp, XML_CQUANT_NONE, XML_CQUANT_OPT,
    XML_CQUANT_PLUS, XML_CQUANT_REP, XML_CTYPE_ANY, XML_CTYPE_CHOICE, XML_CTYPE_EMPTY,
    XML_CTYPE_MIXED, XML_CTYPE_NAME, XML_CTYPE_SEQ, XML_ERROR_ABORTED, XML_ERROR_ASYNC_ENTITY,
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
    XML_ERROR_UNKNOWN_ENCODING, XML_ERROR_XML_DECL, XML_FALSE, XML_FEATURE_ATTR_INFO,
    XML_FEATURE_CONTEXT_BYTES, XML_FEATURE_DTD, XML_FEATURE_END, XML_FEATURE_LARGE_SIZE,
    XML_FEATURE_MIN_SIZE, XML_FEATURE_NS, XML_FEATURE_SIZEOF_XML_CHAR,
    XML_FEATURE_SIZEOF_XML_LCHAR, XML_FEATURE_UNICODE, XML_FEATURE_UNICODE_WCHAR_T, XML_FINISHED,
    XML_INITIALIZED,
    XML_PARAM_ENTITY_PARSING_ALWAYS, XML_PARAM_ENTITY_PARSING_NEVER,
    XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE, XML_PARSING, XML_STATUS_ERROR, XML_STATUS_ERROR_0,
    XML_STATUS_OK, XML_STATUS_OK_0, XML_STATUS_SUSPENDED, XML_STATUS_SUSPENDED_0, XML_SUSPENDED,
    XML_TRUE,
};
pub use crate::siphash_h::{
    sip24_final, sip24_init, sip24_update, sip24_valid, sip_round, sip_tokey, siphash, siphash24,
    sipkey,
};
pub use crate::lib::xmlrole::{
    prolog_state, C2RustUnnamed_0, XmlPrologStateInit, XmlPrologStateInitExternalEntity,
    PROLOG_STATE, XML_ROLE_ATTLIST_ELEMENT_NAME, XML_ROLE_ATTLIST_NONE,
    XML_ROLE_ATTRIBUTE_ENUM_VALUE, XML_ROLE_ATTRIBUTE_NAME, XML_ROLE_ATTRIBUTE_NOTATION_VALUE,
    XML_ROLE_ATTRIBUTE_TYPE_CDATA, XML_ROLE_ATTRIBUTE_TYPE_ENTITIES,
    XML_ROLE_ATTRIBUTE_TYPE_ENTITY, XML_ROLE_ATTRIBUTE_TYPE_ID, XML_ROLE_ATTRIBUTE_TYPE_IDREF,
    XML_ROLE_ATTRIBUTE_TYPE_IDREFS, XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN,
    XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS, XML_ROLE_COMMENT, XML_ROLE_CONTENT_ANY,
    XML_ROLE_CONTENT_ELEMENT, XML_ROLE_CONTENT_ELEMENT_OPT, XML_ROLE_CONTENT_ELEMENT_PLUS,
    XML_ROLE_CONTENT_ELEMENT_REP, XML_ROLE_CONTENT_EMPTY, XML_ROLE_CONTENT_PCDATA,
    XML_ROLE_DEFAULT_ATTRIBUTE_VALUE, XML_ROLE_DOCTYPE_CLOSE, XML_ROLE_DOCTYPE_INTERNAL_SUBSET,
    XML_ROLE_DOCTYPE_NAME, XML_ROLE_DOCTYPE_NONE, XML_ROLE_DOCTYPE_PUBLIC_ID,
    XML_ROLE_DOCTYPE_SYSTEM_ID, XML_ROLE_ELEMENT_NAME, XML_ROLE_ELEMENT_NONE,
    XML_ROLE_ENTITY_COMPLETE, XML_ROLE_ENTITY_NONE, XML_ROLE_ENTITY_NOTATION_NAME,
    XML_ROLE_ENTITY_PUBLIC_ID, XML_ROLE_ENTITY_SYSTEM_ID, XML_ROLE_ENTITY_VALUE, XML_ROLE_ERROR,
    XML_ROLE_FIXED_ATTRIBUTE_VALUE, XML_ROLE_GENERAL_ENTITY_NAME, XML_ROLE_GROUP_CHOICE,
    XML_ROLE_GROUP_CLOSE, XML_ROLE_GROUP_CLOSE_OPT, XML_ROLE_GROUP_CLOSE_PLUS,
    XML_ROLE_GROUP_CLOSE_REP, XML_ROLE_GROUP_OPEN, XML_ROLE_GROUP_SEQUENCE, XML_ROLE_IGNORE_SECT,
    XML_ROLE_IMPLIED_ATTRIBUTE_VALUE, XML_ROLE_INNER_PARAM_ENTITY_REF, XML_ROLE_INSTANCE_START,
    XML_ROLE_NONE, XML_ROLE_NOTATION_NAME, XML_ROLE_NOTATION_NONE, XML_ROLE_NOTATION_NO_SYSTEM_ID,
    XML_ROLE_NOTATION_PUBLIC_ID, XML_ROLE_NOTATION_SYSTEM_ID, XML_ROLE_PARAM_ENTITY_NAME,
    XML_ROLE_PARAM_ENTITY_REF, XML_ROLE_PI, XML_ROLE_REQUIRED_ATTRIBUTE_VALUE, XML_ROLE_TEXT_DECL,
    XML_ROLE_XML_DECL,
};
pub use crate::lib::xmltok::{
    XmlParseXmlDecl, XmlParseXmlDeclNS, UnknownEncoding,
};
pub use crate::lib::xmltok::*;
pub use crate::stddef_h::{ptrdiff_t, size_t, NULL};
pub use crate::stdlib::{
    _IO_lock_t, __off64_t, __off_t, __pid_t, __ssize_t,
    __suseconds_t, __time_t, __timezone_ptr_t, __uint64_t, fprintf, getrandom, gettimeofday,
    ssize_t, stderr, timezone, uint64_t, FILE, GRND_NONBLOCK, _IO_FILE,
};
use crate::stdlib::{__assert_fail, memcmp, memcpy, memmove, memset, read};
use ::libc::{self, __errno_location, close, getenv, getpid, open, strcmp};
pub use ::libc::{timeval, EINTR, INT_MAX, O_RDONLY};
use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void, intptr_t};
#[cfg(feature = "getrandom_syscall")]
use libc::{SYS_getrandom, syscall};

use fallible_collections::FallibleBox;

use std::alloc::{self, Layout};
use std::collections::HashMap;
use std::ptr;

impl STRING_POOL {
    #[inline]
    unsafe fn appendChar(&mut self, c: XML_Char) -> bool {
        if self.ptr == self.end as *mut XML_Char && self.grow() == 0 {
            false
        } else {
            *self.ptr = c;
            self.ptr = self.ptr.offset(1);
            true
        }
    }
}

trait XmlHandlers {
    fn hasAttlistDecl(&self) -> bool;
    fn hasCharacterData(&self) -> bool;
    fn hasComment(&self) -> bool;
    fn hasDefault(&self) -> bool;
    fn hasElementDecl(&self) -> bool;
    fn hasEndCDataSection(&self) -> bool;
    fn hasEndDoctypeDecl(&self) -> bool;
    fn hasEndElement(&self) -> bool;
    fn hasEndNamespaceDecl(&self) -> bool;
    fn hasEntityDecl(&self) -> bool;
    fn hasExternalEntityRef(&self) -> bool;
    fn hasNotationDecl(&self) -> bool;
    fn hasNotStandalone(&self) -> bool;
    fn hasProcessingInstruction(&self) -> bool;
    fn hasSkippedEntity(&self) -> bool;
    fn hasStartCDataSection(&self) -> bool;
    fn hasStartDoctypeDecl(&self) -> bool;
    fn hasStartNamespaceDecl(&self) -> bool;
    fn hasUnknownEncoding(&self) -> bool;
    fn hasUnparsedEntityDecl(&self) -> bool;
    fn hasXmlDecl(&self) -> bool;
    unsafe fn attlistDecl(&self, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char, _: c_int) -> bool;
    unsafe fn characterData(&self, _: *const XML_Char, _: c_int) -> bool;
    unsafe fn comment(&self, b: *const XML_Char) -> bool;
    unsafe fn default(&self, _: *const c_char, _: c_int) -> bool;
    unsafe fn elementDecl(&self, _: *const XML_Char, _: *mut XML_Content) -> bool;
    unsafe fn endCDataSection(&self) -> bool;
    unsafe fn endDoctypeDecl(&self) -> bool;
    unsafe fn endElement(&self, _: *const XML_Char) -> bool;
    unsafe fn endNamespaceDecl(&self, _: *const XML_Char) -> bool;
    unsafe fn entityDecl(&self, _: *const XML_Char, _: c_int, _: *const XML_Char, _: c_int, _: *const XML_Char,
                                _: *const XML_Char, _: *const XML_Char, _: *const XML_Char) -> bool;
    unsafe fn externalEntityRef(&self, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char) -> Result<c_int, ()>;
    unsafe fn notationDecl(&self, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char) -> bool;
    unsafe fn notStandalone(&self) -> Result<c_int, ()>;
    unsafe fn processingInstruction(&self, b: *const XML_Char, c: *const XML_Char) -> bool;
    unsafe fn skippedEntity(&self, _: *const XML_Char, _: c_int) -> bool;
    unsafe fn startCDataSection(&self) -> bool;
    unsafe fn startDoctypeDecl(&self, a: *const XML_Char, b: *const XML_Char, c: *const XML_Char, d: c_int) -> bool;
    unsafe fn startElement(&self, _: *const XML_Char, _: *mut ATTRIBUTE) -> bool;
    unsafe fn startNamespaceDecl(&self, _: *const XML_Char, b: *const XML_Char) -> bool;
    unsafe fn unknownEncoding(&self, _: *const XML_Char, _: *mut XML_Encoding) -> Result<c_int, ()>;
    unsafe fn unparsedEntityDecl(&self, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char) -> bool;
    unsafe fn xmlDecl(&self, _: *const XML_Char, _: *const XML_Char, _: c_int) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CXmlHandlers {
    m_attlistDeclHandler: XML_AttlistDeclHandler,
    m_characterDataHandler: XML_CharacterDataHandler,
    m_commentHandler: XML_CommentHandler,
    m_defaultHandler: XML_DefaultHandler,
    m_elementDeclHandler: XML_ElementDeclHandler,
    m_endCdataSectionHandler: XML_EndCdataSectionHandler,
    m_endDoctypeDeclHandler: XML_EndDoctypeDeclHandler,
    m_endElementHandler: XML_EndElementHandler,
    m_endNamespaceDeclHandler: XML_EndNamespaceDeclHandler,
    m_entityDeclHandler: XML_EntityDeclHandler,
    m_externalEntityRefHandler: XML_ExternalEntityRefHandler,
    m_externalEntityRefHandlerArg: XML_Parser,
    m_handlerArg: *mut c_void,
    m_notationDeclHandler: XML_NotationDeclHandler,
    m_notStandaloneHandler: XML_NotStandaloneHandler,
    m_processingInstructionHandler: XML_ProcessingInstructionHandler,
    m_skippedEntityHandler: XML_SkippedEntityHandler,
    m_startCdataSectionHandler: XML_StartCdataSectionHandler,
    m_startDoctypeDeclHandler: XML_StartDoctypeDeclHandler,
    m_startElementHandler: XML_StartElementHandler,
    m_startNamespaceDeclHandler: XML_StartNamespaceDeclHandler,
    m_unknownEncodingHandler: XML_UnknownEncodingHandler,
    m_unknownEncodingHandlerData: *mut c_void,
    m_unparsedEntityDeclHandler: XML_UnparsedEntityDeclHandler,
    m_xmlDeclHandler: XML_XmlDeclHandler,
}

impl Default for CXmlHandlers {
    fn default() -> Self {
        CXmlHandlers {
            m_attlistDeclHandler: None,
            m_characterDataHandler: None,
            m_commentHandler: None,
            m_defaultHandler: None,
            m_elementDeclHandler: None,
            m_endCdataSectionHandler: None,
            m_endDoctypeDeclHandler: None,
            m_endElementHandler: None,
            m_endNamespaceDeclHandler: None,
            m_entityDeclHandler: None,
            m_externalEntityRefHandler: None,
            m_externalEntityRefHandlerArg: std::ptr::null_mut(),
            m_handlerArg: std::ptr::null_mut(),
            m_notationDeclHandler: None,
            m_notStandaloneHandler: None,
            m_processingInstructionHandler: None,
            m_skippedEntityHandler: None,
            m_startCdataSectionHandler: None,
            m_startDoctypeDeclHandler: None,
            m_startElementHandler: None,
            m_startNamespaceDeclHandler: None,
            m_unknownEncodingHandler: None,
            m_unknownEncodingHandlerData: std::ptr::null_mut(),
            m_unparsedEntityDeclHandler: None,
            m_xmlDeclHandler: None,
        }
    }
}

#[derive(Copy, Clone)]
pub enum EncodingType {
    Normal,
    Internal,
}

pub enum OldEncoding {
    Global(&'static dyn XmlEncoding),
    Init(Box<InitEncoding>),
    Unknown(Box<UnknownEncoding>),
    Internal(&'static dyn XmlEncoding),
}

impl EncodingType {
    fn is_internal(&self) -> bool {
        match self {
            EncodingType::Internal => true,
            _ => false,
        }
    }
}

impl CXmlHandlers {
    fn setStartElement(&mut self, handler: XML_StartElementHandler) {
        self.m_startElementHandler = handler;
    }

    fn setEndElement(&mut self, handler: XML_EndElementHandler) {
        self.m_endElementHandler = handler;
    }

    fn setCharacterData(&mut self, handler: XML_CharacterDataHandler) {
        self.m_characterDataHandler = handler;
    }

    fn setProcessingInstruction(&mut self, handler: XML_ProcessingInstructionHandler) {
        self.m_processingInstructionHandler = handler;
    }

    fn setComment(&mut self, handler: XML_CommentHandler) {
        self.m_commentHandler = handler;
    }

    fn setDefault(&mut self, handler: XML_DefaultHandler) {
        self.m_defaultHandler = handler;
    }

    fn setStartCDataSection(&mut self, handler: XML_StartCdataSectionHandler) {
        self.m_startCdataSectionHandler = handler;
    }

    fn setEndCDataSection(&mut self, handler: XML_EndCdataSectionHandler) {
        self.m_endCdataSectionHandler = handler;
    }

    fn setStartDoctypeDecl(&mut self, handler: XML_StartDoctypeDeclHandler) {
        self.m_startDoctypeDeclHandler = handler;
    }

    fn setEndDoctypeDecl(&mut self, handler: XML_EndDoctypeDeclHandler) {
        self.m_endDoctypeDeclHandler = handler;
    }

    fn setStartNamespaceDecl(&mut self, handler: XML_StartNamespaceDeclHandler) {
        self.m_startNamespaceDeclHandler = handler;
    }

    fn setEndNamespaceDecl(&mut self, handler: XML_EndNamespaceDeclHandler) {
        self.m_endNamespaceDeclHandler = handler;
    }

    fn setElementDecl(&mut self, handler: XML_ElementDeclHandler) {
        self.m_elementDeclHandler = handler;
    }

    fn setAttlistDecl(&mut self, handler: XML_AttlistDeclHandler) {
        self.m_attlistDeclHandler = handler;
    }

    fn setEntityDecl(&mut self, handler: XML_EntityDeclHandler) {
        self.m_entityDeclHandler = handler;
    }

    fn setXmlDecl(&mut self, handler: XML_XmlDeclHandler) {
        self.m_xmlDeclHandler = handler;
    }

    fn setSkippedEntity(&mut self, handler: XML_SkippedEntityHandler) {
        self.m_skippedEntityHandler = handler;
    }

    fn setUnknownEncoding(&mut self, handler: XML_UnknownEncodingHandler) {
        self.m_unknownEncodingHandler = handler;
    }

    fn setUnparsedEntityDecl(&mut self, handler: XML_UnparsedEntityDeclHandler) {
        self.m_unparsedEntityDeclHandler = handler;
    }

    fn setNotationDecl(&mut self, handler: XML_NotationDeclHandler) {
        self.m_notationDeclHandler = handler;
    }

    fn setNotStandalone(&mut self, handler: XML_NotStandaloneHandler) {
        self.m_notStandaloneHandler = handler;
    }

    fn setExternalEntityRef(&mut self, handler: XML_ExternalEntityRefHandler) {
        self.m_externalEntityRefHandler = handler;
    }
}

impl XmlHandlers for CXmlHandlers {
    unsafe fn startElement(&self, a: *const XML_Char, b: *mut ATTRIBUTE) -> bool {
        self.m_startElementHandler.map(|handler| {
            handler(self.m_handlerArg, a, b as *mut *const XML_Char);

            true
        }).unwrap_or(false)
    }

    unsafe fn endElement(&self, a: *const XML_Char) -> bool {
        self.m_endElementHandler.map(|handler| {
            handler(self.m_handlerArg, a);

            true
        }).unwrap_or(false)
    }

    unsafe fn characterData(&self, a: *const XML_Char, b: c_int) -> bool {
        self.m_characterDataHandler.map(|handler| {
            handler(self.m_handlerArg, a, b);

            true
        }).unwrap_or(false)
    }

    unsafe fn externalEntityRef(&self, a: *const XML_Char, b: *const XML_Char, c: *const XML_Char, d: *const XML_Char) -> Result<c_int, ()> {
        self.m_externalEntityRefHandler
            .map(|handler| Ok(handler(self.m_externalEntityRefHandlerArg, a, b, c, d)))
            .unwrap_or(Err(()))
    }

    unsafe fn processingInstruction(&self, a: *const XML_Char, b: *const XML_Char) -> bool {
        self.m_processingInstructionHandler.map(|handler| {
            handler(self.m_handlerArg, a, b);

            true
        }).unwrap_or(false)
    }

    unsafe fn comment(&self, a: *const XML_Char) -> bool {
        self.m_commentHandler.map(|handler| {
            handler(self.m_handlerArg, a);

            true
        }).unwrap_or(false)
    }

    unsafe fn startCDataSection(&self) -> bool {
        self.m_startCdataSectionHandler.map(|handler| {
            handler(self.m_handlerArg);

            true
        }).unwrap_or(false)
    }

    unsafe fn endCDataSection(&self) -> bool {
        self.m_endCdataSectionHandler.map(|handler| {
            handler(self.m_handlerArg);

            true
        }).unwrap_or(false)
    }

    unsafe fn startDoctypeDecl(&self, a: *const XML_Char, b: *const XML_Char, c: *const XML_Char, d: c_int) -> bool {
        self.m_startDoctypeDeclHandler.map(|handler| {
            handler(self.m_handlerArg, a, b, c, d);

            true
        }).unwrap_or(false)
    }

    unsafe fn endDoctypeDecl(&self) -> bool {
        self.m_endDoctypeDeclHandler.map(|handler| {
            handler(self.m_handlerArg);

            true
        }).unwrap_or(false)
    }

    unsafe fn startNamespaceDecl(&self, a: *const XML_Char, b: *const XML_Char) -> bool {
        self.m_startNamespaceDeclHandler.map(|handler| {
            handler(self.m_handlerArg, a, b);

            true
        }).unwrap_or(false)
    }

    unsafe fn endNamespaceDecl(&self, a: *const XML_Char) -> bool {
        self.m_endNamespaceDeclHandler.map(|handler| {
            handler(self.m_handlerArg, a);

            true
        }).unwrap_or(false)
    }

    unsafe fn elementDecl(&self, a: *const XML_Char, b: *mut XML_Content) -> bool {
        self.m_elementDeclHandler.map(|handler| {
            handler(self.m_handlerArg, a, b);

            true
        }).unwrap_or(false)
    }

    unsafe fn attlistDecl(&self, a: *const XML_Char, b: *const XML_Char, c: *const XML_Char, d: *const XML_Char, e: c_int) -> bool {
        self.m_attlistDeclHandler.map(|handler| {
            handler(self.m_handlerArg, a, b, c, d, e);

            true
        }).unwrap_or(false)
    }

    unsafe fn entityDecl(
        &self,
        a: *const XML_Char,
        b: c_int,
        c: *const XML_Char,
        d: c_int,
        e: *const XML_Char,
        f: *const XML_Char,
        g: *const XML_Char,
        h: *const XML_Char,
    ) -> bool {
        self.m_entityDeclHandler.map(|handler| {
            handler(self.m_handlerArg, a, b, c, d, e, f, g, h);

            true
        }).unwrap_or(false)
    }

    unsafe fn xmlDecl(&self, a: *const XML_Char, b: *const XML_Char, c: c_int) -> bool {
        self.m_xmlDeclHandler.map(|handler| {
            handler(self.m_handlerArg, a, b, c);

            true
        }).unwrap_or(false)
    }

    unsafe fn unknownEncoding(&self, a: *const XML_Char, b: *mut XML_Encoding) -> Result<c_int, ()> {
        self.m_unknownEncodingHandler
            .map(|handler| Ok(handler(self.m_unknownEncodingHandlerData, a, b)))
            .unwrap_or(Err(()))
    }

    unsafe fn skippedEntity(&self, a: *const XML_Char, b: c_int) -> bool {
        self.m_skippedEntityHandler.map(|handler| {
            handler(self.m_handlerArg, a, b);

            true
        }).unwrap_or(false)
    }

    unsafe fn unparsedEntityDecl(
        &self,
        a: *const XML_Char,
        b: *const XML_Char,
        c: *const XML_Char,
        d: *const XML_Char,
        e: *const XML_Char,
    ) -> bool {
        self.m_unparsedEntityDeclHandler.map(|handler| {
            handler(self.m_handlerArg, a, b, c, d, e);

            true
        }).unwrap_or(false)
    }

    unsafe fn notationDecl(
        &self,
        a: *const XML_Char,
        b: *const XML_Char,
        c: *const XML_Char,
        d: *const XML_Char,
    ) -> bool {
        self.m_notationDeclHandler.map(|handler| {
            handler(self.m_handlerArg, a, b, c, d);

            true
        }).unwrap_or(false)
    }

    unsafe fn notStandalone(&self) -> Result<c_int, ()> {
        self.m_notStandaloneHandler
            .map(|handler| Ok(handler(self.m_handlerArg)))
            .unwrap_or(Err(()))
    }

    unsafe fn default(&self, s: *const c_char, next: c_int) -> bool {
        self.m_defaultHandler.map(|handler| {
            handler(self.m_handlerArg, s, next);

            true
        }).unwrap_or(false)
    }

    fn hasEndElement(&self) -> bool {
        self.m_endElementHandler.is_some()
    }

    fn hasDefault(&self) -> bool {
        self.m_defaultHandler.is_some()
    }

    fn hasCharacterData(&self) -> bool {
        self.m_characterDataHandler.is_some()
    }

    fn hasProcessingInstruction(&self) -> bool {
        self.m_processingInstructionHandler.is_some()
    }

    fn hasComment(&self) -> bool {
        self.m_commentHandler.is_some()
    }

    fn hasStartCDataSection(&self) -> bool {
        self.m_startCdataSectionHandler.is_some()
    }

    fn hasEndCDataSection(&self) -> bool {
        self.m_endCdataSectionHandler.is_some()
    }

    fn hasStartDoctypeDecl(&self) -> bool {
        self.m_startDoctypeDeclHandler.is_some()
    }

    fn hasEndDoctypeDecl(&self) -> bool {
        self.m_endDoctypeDeclHandler.is_some()
    }

    fn hasStartNamespaceDecl(&self) -> bool {
        self.m_startNamespaceDeclHandler.is_some()
    }

    fn hasEndNamespaceDecl(&self) -> bool {
        self.m_endNamespaceDeclHandler.is_some()
    }

    fn hasElementDecl(&self) -> bool {
        self.m_elementDeclHandler.is_some()
    }

    fn hasAttlistDecl(&self) -> bool {
        self.m_attlistDeclHandler.is_some()
    }

    fn hasEntityDecl(&self) -> bool {
        self.m_entityDeclHandler.is_some()
    }

    fn hasXmlDecl(&self) -> bool {
        self.m_xmlDeclHandler.is_some()
    }

    fn hasSkippedEntity(&self) -> bool {
        self.m_skippedEntityHandler.is_some()
    }

    fn hasUnknownEncoding(&self) -> bool {
        self.m_unknownEncodingHandler.is_some()
    }

    fn hasUnparsedEntityDecl(&self) -> bool {
        self.m_unparsedEntityDeclHandler.is_some()
    }

    fn hasNotationDecl(&self) -> bool {
        self.m_notationDeclHandler.is_some()
    }

    fn hasNotStandalone(&self) -> bool {
        self.m_notStandaloneHandler.is_some()
    }

    fn hasExternalEntityRef(&self) -> bool {
        self.m_externalEntityRefHandler.is_some()
    }
}

#[repr(C)]
pub struct XML_ParserStruct {
    /* The first member must be m_userData so that the XML_GetUserData
    macro works. */
    pub m_userData: *mut c_void,
    pub m_buffer: *mut c_char,
    /* first character to be parsed */
    pub m_bufferPtr: *const c_char,
    /* past last character to be parsed */
    pub m_bufferEnd: *mut c_char,
    /* allocated end of m_buffer */
    pub m_bufferLim: *const c_char,
    pub m_parseEndByteIndex: XML_Index,
    pub m_parseEndPtr: *const c_char,
    pub m_dataBuf: *mut XML_Char, // Box<[XML_Char; INIT_DATA_BUF_SIZE]>
    pub m_dataBufEnd: *mut XML_Char,

    // Handlers should be trait, with native C callback instance
    m_handlers: CXmlHandlers,
    pub m_encoding: *const ENCODING,
    pub m_initEncoding: Option<InitEncoding>,
    pub m_internalEncoding: &'static super::xmltok::ENCODING,
    pub m_protocolEncodingName: *const XML_Char,
    pub m_ns: XML_Bool,
    pub m_ns_triplets: XML_Bool,
    pub m_unknownEncoding: Option<Box<UnknownEncoding>>,
    pub m_unknownEncodingData: *mut c_void,
    pub m_unknownEncodingRelease: Option<unsafe extern "C" fn(_: *mut c_void) -> ()>,
    pub m_prologState: super::xmlrole::PROLOG_STATE,
    pub m_processor: Option<Processor>,
    pub m_errorCode: XML_Error,
    pub m_eventPtr: *const c_char,
    pub m_eventEndPtr: *const c_char,
    pub m_positionPtr: *const c_char,
    pub m_openInternalEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeInternalEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_defaultExpandInternalEntities: XML_Bool,
    pub m_tagLevel: c_int,
    pub m_declEntity: *mut ENTITY,
    pub m_doctypeName: *const XML_Char,
    pub m_doctypeSysid: *const XML_Char,
    pub m_doctypePubid: *const XML_Char,
    pub m_declAttributeType: *const XML_Char,
    pub m_declNotationName: *const XML_Char,
    pub m_declNotationPublicId: *const XML_Char,
    pub m_declElementType: *mut ELEMENT_TYPE,
    pub m_declAttributeId: *mut ATTRIBUTE_ID,
    pub m_declAttributeIsCdata: XML_Bool,
    pub m_declAttributeIsId: XML_Bool,
    pub m_dtd: *mut DTD,
    pub m_curBase: *const XML_Char,
    pub m_tagStack: *mut TAG,
    pub m_freeTagList: *mut TAG,
    pub m_inheritedBindings: *mut BINDING,
    pub m_freeBindingList: *mut BINDING,
    pub m_attsSize: c_int,
    pub m_nSpecifiedAtts: c_int,
    pub m_idAttIndex: c_int,
    pub m_atts: *mut super::xmltok::ATTRIBUTE,
    pub m_nsAtts: *mut NS_ATT,
    pub m_nsAttsVersion: c_ulong,
    pub m_nsAttsPower: c_uchar,
    pub m_position: super::xmltok::POSITION,
    pub m_tempPool: STRING_POOL,
    pub m_temp2Pool: STRING_POOL,
    pub m_groupConnector: *mut c_char,
    pub m_groupSize: c_uint,
    pub m_namespaceSeparator: XML_Char,
    pub m_parentParser: XML_Parser,
    pub m_parsingStatus: XML_ParsingStatus,
    pub m_isParamEntity: XML_Bool,
    pub m_useForeignDTD: XML_Bool,
    pub m_paramEntityParsing: XML_ParamEntityParsing,
    pub m_hash_secret_salt: c_ulong,

    #[cfg(feature = "mozilla")]
    pub m_mismatch: *const XML_Char,
}

impl XML_ParserStruct {
    fn encoding<'a, 'b>(&'a self, enc_type: EncodingType) -> &'b dyn XmlEncoding {
        match enc_type {
            EncodingType::Normal => unsafe { &*self.m_encoding },
            EncodingType::Internal => self.m_internalEncoding,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct STRING_POOL {
    pub blocks: *mut BLOCK,
    pub freeBlocks: *mut BLOCK,
    pub end: *const XML_Char,
    pub ptr: *mut XML_Char,
    pub start: *mut XML_Char,
}

impl STRING_POOL {
    fn new() -> Self {
        Self {
            blocks: ptr::null_mut(),
            freeBlocks: ptr::null_mut(),
            end: ptr::null(),
            ptr: ptr::null_mut(),
            start: ptr::null_mut(),
        }
    }
}

pub type BLOCK = block;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block {
    pub next: *mut block,
    pub size: c_int,
    pub s: [XML_Char; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NS_ATT {
    pub version: c_ulong,
    pub hash: c_ulong,
    pub uriName: *const XML_Char,
}

pub type BINDING = binding;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binding {
    pub prefix: *mut prefix,
    pub nextTagBinding: *mut binding,
    pub prevPrefixBinding: *mut binding,
    pub attId: *const attribute_id,
    pub uri: *mut XML_Char,
    pub uriLen: c_int,
    pub uriAlloc: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct attribute_id {
    pub name: *mut XML_Char,
    pub prefix: *mut PREFIX,
    pub maybeTokenized: XML_Bool,
    pub xmlns: XML_Bool,
}

pub type PREFIX = prefix;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prefix {
    pub name: *const XML_Char,
    pub binding: *mut BINDING,
}

/* TAG represents an open element.
   The name of the element is stored in both the document and API
   encodings.  The memory buffer 'buf' is a separately-allocated
   memory area which stores the name.  During the XML_Parse()/
   XMLParseBuffer() when the element is open, the memory for the 'raw'
   version of the name (in the document encoding) is shared with the
   document buffer.  If the element is open across calls to
   XML_Parse()/XML_ParseBuffer(), the buffer is re-allocated to
   contain the 'raw' name as well.

   A parser re-uses these structures, maintaining a list of allocated
   TAG objects in a free list.
*/
pub type TAG = tag;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag {
    pub parent: *mut tag,
    pub rawName: *const c_char,
    pub rawNameLength: c_int,
    pub name: TAG_NAME,
    pub buf: *mut c_char,
    pub bufEnd: *mut c_char,
    pub bindings: *mut BINDING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TAG_NAME {
    pub str_0: *const XML_Char,
    pub localPart: *const XML_Char,
    pub prefix: *const XML_Char,
    pub strLen: c_int,
    pub uriLen: c_int,
    pub prefixLen: c_int,
}

// FIXME: add a proper lifetime
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HashKey(&'static [XML_Char]);

impl HashKey {
    unsafe fn from(key: KEY) -> Self {
        HashKey(std::slice::from_raw_parts(key, keylen(key) as usize))
    }
}

macro_rules! hash_insert {
    ($map:expr, $key:expr, $et:ident) => {{
        let __key = $key;
        let __hk = HashKey::from(__key);
        if !$map.contains_key(&__hk) {
            if $map.try_reserve(1).is_ok() {
                let v = $et { name: __key, ..std::mem::zeroed() };
                if let Ok(b) = Box::try_new(v) {
                    $map.insert(__hk.clone(), b);
                }
            }
        }
        $map.get_mut(&__hk)
            .map_or_else(std::ptr::null_mut, |x| x.as_mut())
    }};
}

macro_rules! hash_lookup {
    ($map:expr, $key:expr) => {
        $map.get_mut(&HashKey::from($key))
            .map_or_else(std::ptr::null_mut, |x| x.as_mut())
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct DTD {
    pub generalEntities: HashMap<HashKey, Box<ENTITY>>,
    pub elementTypes: HashMap<HashKey, Box<ELEMENT_TYPE>>,
    pub attributeIds: HashMap<HashKey, Box<ATTRIBUTE_ID>>,
    pub prefixes: HashMap<HashKey, Box<PREFIX>>,
    pub pool: STRING_POOL,
    pub entityValuePool: STRING_POOL,
    pub keepProcessing: XML_Bool,
    pub hasParamEntityRefs: XML_Bool,
    pub standalone: XML_Bool,
    pub paramEntityRead: XML_Bool,
    // `test_alloc_nested_entities` counts the allocations,
    // so we need to use `Box` here to pass that test
    pub paramEntities: HashMap<HashKey, Box<ENTITY>>,
    pub defaultPrefix: PREFIX,
    pub in_eldecl: XML_Bool,
    pub scaffold: *mut CONTENT_SCAFFOLD,
    pub contentStringLen: c_uint,
    pub scaffSize: c_uint,
    pub scaffCount: c_uint,
    pub scaffLevel: c_int,
    pub scaffIndex: *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CONTENT_SCAFFOLD {
    pub type_0: XML_Content_Type,
    pub quant: XML_Content_Quant,
    pub name: *const XML_Char,
    pub firstchild: c_int,
    pub lastchild: c_int,
    pub childcnt: c_int,
    pub nextsib: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NAMED {
    pub name: KEY,
}
/* parent of this element */
/* tagName in the original encoding */
/* tagName in the API encoding */
/* buffer for name components */
/* end of the buffer */
/* Round up n to be a multiple of sz, where sz is a power of 2. */
/* Do safe (NULL-aware) pointer arithmetic */

pub type KEY = *const XML_Char;
/* The XML_Char before the name is used to determine whether
an attribute has been specified. */

pub type ATTRIBUTE_ID = attribute_id;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ELEMENT_TYPE {
    pub name: *const XML_Char,
    pub prefix: *mut PREFIX,
    pub idAtt: *const ATTRIBUTE_ID,
    pub nDefaultAtts: c_int,
    pub allocDefaultAtts: c_int,
    pub defaultAtts: *mut DEFAULT_ATTRIBUTE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DEFAULT_ATTRIBUTE {
    pub id: *const ATTRIBUTE_ID,
    pub isCdata: XML_Bool,
    pub value: *const XML_Char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ENTITY {
    pub name: *const XML_Char,
    pub textPtr: *const XML_Char,
    pub textLen: c_int,
    pub processed: c_int,
    pub systemId: *const XML_Char,
    pub base: *const XML_Char,
    pub publicId: *const XML_Char,
    pub notation: *const XML_Char,
    pub open: XML_Bool,
    pub is_param: XML_Bool,
    pub is_internal: XML_Bool,
}

pub type OPEN_INTERNAL_ENTITY = open_internal_entity;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct open_internal_entity {
    pub internalEventPtr: *const c_char,
    pub internalEventEndPtr: *const c_char,
    pub next: *mut open_internal_entity,
    pub entity: *mut ENTITY,
    pub startTagLevel: c_int,
    pub betweenDecl: XML_Bool,
}

pub type Processor = unsafe extern "C" fn(
    _: XML_Parser,
    _: *const c_char,
    _: *const c_char,
    _: *mut *const c_char,
) -> XML_Error;

#[cfg(feature = "unicode")]
#[macro_use]
mod unicode_defines {
    pub const XML_ENCODE_MAX: usize = crate::lib::xmltok::XML_UTF16_ENCODE_MAX as usize;
    pub use crate::lib::xmltok::XmlGetUtf16InternalEncoding as XmlGetInternalEncoding;
    pub use crate::lib::xmltok::XmlGetUtf16InternalEncodingNS as XmlGetInternalEncodingNS;
    pub use crate::lib::xmltok::XmlUtf16Encode as XmlEncode;
    pub use crate::XmlUtf16Convert as XmlConvert;

    macro_rules! MUST_CONVERT {
        ($enc:path, $s:expr $(,)?) => {
            !(*$enc).isUtf16()
                // TODO(SJC): should this be MINBPC?
                || $s.align_offset(2) != 0
        };
    }

    pub type ICHAR = libc::c_ushort;

    #[cfg(feature = "unicode_wchar_t")]
    macro_rules! wch {
        ($s:literal) => {
            wchar::wch!(i32, $s).as_ptr()
        };
    }

    #[cfg(not(feature = "unicode_wchar_t"))]
    macro_rules! wch {
        ($s:literal) => {
            $s.as_ptr() as *const crate::expat_external_h::XML_LChar
        };
    }
}

#[cfg(not(feature = "unicode"))]
#[macro_use]
mod unicode_defines {
    pub const XML_ENCODE_MAX: usize = crate::lib::xmltok::XML_UTF8_ENCODE_MAX as usize;
    pub use crate::lib::xmltok::XmlGetUtf8InternalEncoding as XmlGetInternalEncoding;
    pub use crate::lib::xmltok::XmlGetUtf8InternalEncodingNS as XmlGetInternalEncodingNS;
    pub use crate::lib::xmltok::XmlUtf8Encode as XmlEncode;
    pub use crate::XmlUtf8Convert as XmlConvert;

    macro_rules! MUST_CONVERT {
        ($enc:path, $s:expr $(,)?) => {
            !(*$enc).isUtf8()
        };
    }

    pub type ICHAR = libc::c_char;

    macro_rules! wch {
        ($s:literal) => {
            $s.as_ptr() as *const crate::expat_external_h::XML_LChar
        };
    }
}

use unicode_defines::*;

/* WFC: PE Between Declarations */

pub const INIT_TAG_BUF_SIZE: c_int = 32; /* must be a multiple of sizeof(XML_Char) */

pub const INIT_DATA_BUF_SIZE: c_int = 1024;

pub const INIT_ATTS_SIZE: c_int = 16;

pub const INIT_ATTS_VERSION: c_uint = 0xffffffff;

pub const INIT_BLOCK_SIZE: c_int = init_block_size_const();

#[cfg(feature = "mozilla")]
const fn init_block_size_const() -> c_int {
    // FIXME: should be `offset_of(BLOCK, s)`, but that's not supported yet,
    // so we over-estimate its offset
    1024 - (std::mem::size_of::<BLOCK>() / std::mem::size_of::<XML_Char>()) as c_int
}

#[cfg(not(feature = "mozilla"))]
const fn init_block_size_const() -> c_int {
    1024
}

pub const INIT_BUFFER_SIZE: c_int = 1024;

pub const EXPAND_SPARE: c_int = 24;

pub const INIT_SCAFFOLD_ELEMENTS: c_int = 32;

macro_rules! MALLOC {
    ($size:expr $(,)?) => {{
        let layout = Layout::from_size_align($size as usize, 1)
            .expect("failed to create Layout");
        alloc::alloc(layout) as *mut c_void
    }};
    [$ty:ty; $n:expr] => {{
        let layout = Layout::array::<$ty>($n as usize)
            .expect("failed to create array Layout");
        alloc::alloc(layout) as *mut $ty
    }}
}
macro_rules! REALLOC {
    ($ptr:expr, $size:expr $(,)?) => {{
        let layout = Layout::from_size_align($size as usize, 1)
            .expect("failed to create Layout");
        alloc::realloc($ptr as *mut u8, layout, $size as usize) as *mut c_void
    }};
}
macro_rules! FREE {
    ($ptr:expr $(,)?) => {
        // FIXME: get the actual layout somehow
        alloc::dealloc($ptr as *mut u8, Layout::new::<u8>())
    };
}

/* Constructs a new parser; encoding is the encoding specified by the
   external protocol or NULL if there is none specified.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate(mut encodingName: *const XML_Char) -> XML_Parser {
    XML_ParserCreate_MM(
        encodingName,
        None,
        NULL as *const XML_Char,
    )
}
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
pub unsafe extern "C" fn XML_ParserCreateNS(
    mut encodingName: *const XML_Char,
    mut nsSep: XML_Char,
) -> XML_Parser {
    let mut tmp: [XML_Char; 2] = [0; 2];
    tmp[0] = nsSep;
    XML_ParserCreate_MM(
        encodingName,
        None,
        tmp.as_mut_ptr(),
    )
}

const implicitContext: [XML_Char; 41] = [
    ASCII_x as XML_Char,
    ASCII_m as XML_Char,
    ASCII_l as XML_Char,
    ASCII_EQUALS as XML_Char,
    ASCII_h as XML_Char,
    ASCII_t as XML_Char,
    ASCII_t as XML_Char,
    ASCII_p as XML_Char,
    ASCII_COLON as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_w as XML_Char,
    ASCII_w as XML_Char,
    ASCII_w as XML_Char,
    ASCII_PERIOD as XML_Char,
    ASCII_w as XML_Char,
    ASCII_3 as XML_Char,
    ASCII_PERIOD as XML_Char,
    ASCII_o as XML_Char,
    ASCII_r as XML_Char,
    ASCII_g as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_X as XML_Char,
    ASCII_M as XML_Char,
    ASCII_L as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_1 as XML_Char,
    ASCII_9 as XML_Char,
    ASCII_9 as XML_Char,
    ASCII_8 as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_n as XML_Char,
    ASCII_a as XML_Char,
    ASCII_m as XML_Char,
    ASCII_e as XML_Char,
    ASCII_s as XML_Char,
    ASCII_p as XML_Char,
    ASCII_a as XML_Char,
    ASCII_c as XML_Char,
    ASCII_e as XML_Char,
    '\u{0}' as XML_Char,
];

/* To avoid warnings about unused functions: */

/* Obtain entropy on Linux 3.17+ */
unsafe extern "C" fn writeRandomBytes_getrandom_nonblock(
    mut target: *mut c_void,
    mut count: size_t,
) -> c_int {
    let mut success: c_int = 0; /* full count bytes written? */
    let mut bytesWrittenTotal: size_t = 0;
    let getrandomFlags: c_uint = GRND_NONBLOCK as c_uint;
    loop {
        let currentTarget: *mut c_void =
            (target as *mut c_char).offset(bytesWrittenTotal as isize) as *mut c_void;
        let bytesToWrite: size_t = count.wrapping_sub(bytesWrittenTotal);

        #[cfg(not(feature = "getrandom_syscall"))]
        let bytesWrittenMore: c_int =
            getrandom(currentTarget, bytesToWrite, getrandomFlags) as c_int;
        #[cfg(feature = "getrandom_syscall")]
        let bytesWrittenMore: c_int =
            syscall(SYS_getrandom, currentTarget, bytesToWrite, getrandomFlags) as c_int;

        if bytesWrittenMore > 0 {
            bytesWrittenTotal = (bytesWrittenTotal).wrapping_add(bytesWrittenMore as c_ulong);
            if bytesWrittenTotal >= count {
                success = 1
            }
        }
        if !(success == 0 && *__errno_location() == EINTR) {
            break;
        }
    }
    return success;
}
/* defined(HAVE_GETRANDOM) || defined(HAVE_SYSCALL_GETRANDOM) */

/* Extract entropy from /dev/urandom */
unsafe extern "C" fn writeRandomBytes_dev_urandom(
    mut target: *mut c_void,
    mut count: size_t,
) -> c_int {
    let mut success: c_int = 0; /* full count bytes written? */
    let mut bytesWrittenTotal: size_t = 0;
    let fd: c_int = open(b"/dev/urandom\x00".as_ptr() as *const c_char, O_RDONLY);
    if fd < 0 {
        return 0i32;
    }
    loop {
        let currentTarget: *mut c_void =
            (target as *mut c_char).offset(bytesWrittenTotal as isize) as *mut c_void;
        let bytesToWrite: size_t = count.wrapping_sub(bytesWrittenTotal);
        let bytesWrittenMore: ssize_t = read(fd, currentTarget, bytesToWrite);
        if bytesWrittenMore > 0 {
            bytesWrittenTotal = (bytesWrittenTotal).wrapping_add(bytesWrittenMore as c_ulong);
            if bytesWrittenTotal >= count {
                success = 1
            }
        }
        if !(success == 0 && *__errno_location() == EINTR) {
            break;
        }
    }
    close(fd);
    return success;
}
/* ! defined(_WIN32) && defined(XML_DEV_URANDOM) */
/* ! defined(HAVE_ARC4RANDOM_BUF) && ! defined(HAVE_ARC4RANDOM) */
/* defined(HAVE_ARC4RANDOM) && ! defined(HAVE_ARC4RANDOM_BUF) */
/* _WIN32 */

unsafe extern "C" fn gather_time_entropy() -> c_ulong {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut gettimeofday_res: c_int = 0;
    gettimeofday_res = gettimeofday(&mut tv, NULL as *mut timezone);
    if gettimeofday_res == 0 {
    } else {
        __assert_fail(
            b"gettimeofday_res == 0\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/lib/xmlparse.c\x00".as_ptr()
                as *const c_char,
            782u32,
            (*::std::mem::transmute::<&[u8; 40], &[c_char; 40]>(
                b"unsigned long gather_time_entropy(void)\x00",
            ))
            .as_ptr(),
        );
    }
    /* defined(NDEBUG) */
    /* Microseconds time is <20 bits entropy */
    return tv.tv_usec as c_ulong;
}
/* ! defined(HAVE_ARC4RANDOM_BUF) && ! defined(HAVE_ARC4RANDOM) */

unsafe extern "C" fn ENTROPY_DEBUG(mut label: *const c_char, mut entropy: c_ulong) -> c_ulong {
    if cfg!(feature = "mozilla") {
        return entropy;
    }

    let EXPAT_ENTROPY_DEBUG: *const c_char =
        getenv(b"EXPAT_ENTROPY_DEBUG\x00".as_ptr() as *const c_char);
    if !EXPAT_ENTROPY_DEBUG.is_null()
        && strcmp(EXPAT_ENTROPY_DEBUG, b"1\x00".as_ptr() as *const c_char) == 0
    {
        fprintf(
            stderr,
            b"Entropy: %s --> 0x%0*lx (%lu bytes)\n\x00".as_ptr() as *const c_char,
            label,
            ::std::mem::size_of::<c_ulong>() as c_int * 2i32,
            entropy,
            ::std::mem::size_of::<c_ulong>() as c_ulong,
        );
    }
    entropy
}

unsafe fn generate_hash_secret_salt() -> c_ulong {
    let mut entropy: c_ulong = 0;
    /* "Failproof" high quality providers: */
    /* Try high quality providers first .. */
    if writeRandomBytes_getrandom_nonblock(
        &mut entropy as *mut c_ulong as *mut c_void,
        ::std::mem::size_of::<c_ulong>() as c_ulong,
    ) != 0
    {
        return ENTROPY_DEBUG(b"getrandom\x00".as_ptr() as *const c_char, entropy);
    }
    if writeRandomBytes_dev_urandom(
        &mut entropy as *mut c_ulong as *mut c_void,
        ::std::mem::size_of::<c_ulong>() as c_ulong,
    ) != 0
    {
        return ENTROPY_DEBUG(b"/dev/urandom\x00".as_ptr() as *const c_char, entropy);
    }
    /* ! defined(_WIN32) && defined(XML_DEV_URANDOM) */
    /* .. and self-made low quality for backup: */
    /* Process ID is 0 bits entropy if attacker has local access */
    entropy = gather_time_entropy() ^ getpid() as c_ulong;
    /* Factors are 2^31-1 and 2^61-1 (Mersenne primes M31 and M61) */
    if ::std::mem::size_of::<c_ulong>() as c_ulong == 4 {
        ENTROPY_DEBUG(
            b"fallback(4)\x00".as_ptr() as *const c_char,
            entropy.wrapping_mul(2147483647u64),
        )
    } else {
        ENTROPY_DEBUG(
            b"fallback(8)\x00".as_ptr() as *const c_char,
            entropy.wrapping_mul(2305843009213693951u64),
        )
    }
}

impl XML_ParserStruct {
    unsafe fn get_hash_secret_salt(&mut self) -> c_ulong {
        if !self.m_parentParser.is_null() {
            return (*self.m_parentParser).get_hash_secret_salt();
        }
        self.m_hash_secret_salt
    }

    /* only valid for root parser */
    unsafe fn startParsing(&mut self) -> XML_Bool {
        /* hash functions must be initialized before setContext() is called */
        if self.m_hash_secret_salt == 0u64 {
            self.m_hash_secret_salt = generate_hash_secret_salt()
        }
        if self.m_ns != 0 {
            /* implicit context only set for root parser, since child
               parsers (i.e. external entity parsers) will inherit it
            */
            return self.setContext(implicitContext.as_ptr());
        }
        XML_TRUE
    }
}

/* Constructs a new parser using the memory management suite referred to
   by memsuite. If memsuite is NULL, then use the standard library memory
   suite. If namespaceSeparator is non-NULL it creates a parser with
   namespace processing as described above. The character pointed at
   will serve as the namespace separator.

   All further memory operations used for the created parser will come from
   the given suite.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate_MM(
    mut encodingName: *const XML_Char,
    mut memsuite: Option<&XML_Memory_Handling_Suite>,
    mut nameSep: *const XML_Char,
) -> XML_Parser {
    // FIXME
    assert!(memsuite.is_none());
    XML_ParserStruct::create(encodingName, nameSep, NULL as *mut DTD)
}

impl XML_ParserStruct {
    fn new(use_namespaces: bool) -> Self {
        Self {
            m_userData: ptr::null_mut(),
            m_buffer: ptr::null_mut(),
            /* first character to be parsed */
            m_bufferPtr: ptr::null(),
            /* past last character to be parsed */
            m_bufferEnd: ptr::null_mut(),
            /* allocated end of m_buffer */
            m_bufferLim: ptr::null(),
            m_parseEndByteIndex: 0,
            m_parseEndPtr: ptr::null(),
            m_dataBuf: ptr::null_mut(), // Box<[XML_Char; INIT_DATA_BUF_SIZE]>
            m_dataBufEnd: ptr::null_mut(),

            m_handlers: Default::default(),

            // This is not what the C version does, it leaves this pointer
            // uninitialized. Unfortunately we don't have that luxury with trait
            // object pointers.
            m_encoding: XmlGetInternalEncoding(),
            m_initEncoding: None,

            // This is not what the C version does, it leaves this pointer
            // uninitialized. Unfortunately we don't have that luxury with trait
            // object pointers.
            m_internalEncoding: {
                if use_namespaces {
                    XmlGetInternalEncodingNS()
                } else {
                    XmlGetInternalEncoding()
                }
            },

            m_protocolEncodingName: ptr::null(),
            m_ns: 0,
            m_ns_triplets: 0,
            m_unknownEncoding: None,
            m_unknownEncodingData: ptr::null_mut(),
            m_unknownEncodingRelease: None,
            m_prologState: super::xmlrole::PROLOG_STATE::default(),
            m_processor: None,
            m_errorCode: 0,
            m_eventPtr: ptr::null(),
            m_eventEndPtr: ptr::null(),
            m_positionPtr: ptr::null(),
            m_openInternalEntities: ptr::null_mut(),
            m_freeInternalEntities: ptr::null_mut(),
            m_defaultExpandInternalEntities: 0,
            m_tagLevel: 0,
            m_declEntity: ptr::null_mut(),
            m_doctypeName: ptr::null(),
            m_doctypeSysid: ptr::null(),
            m_doctypePubid: ptr::null(),
            m_declAttributeType: ptr::null(),
            m_declNotationName: ptr::null(),
            m_declNotationPublicId: ptr::null(),
            m_declElementType: ptr::null_mut(),
            m_declAttributeId: ptr::null_mut(),
            m_declAttributeIsCdata: 0,
            m_declAttributeIsId: 0,
            m_dtd: ptr::null_mut(),
            m_curBase: ptr::null(),
            m_tagStack: ptr::null_mut(),
            m_freeTagList: ptr::null_mut(),
            m_inheritedBindings: ptr::null_mut(),
            m_freeBindingList: ptr::null_mut(),
            m_attsSize: 0,
            m_nSpecifiedAtts: 0,
            m_idAttIndex: 0,
            m_atts: ptr::null_mut(),
            m_nsAtts: ptr::null_mut(),
            m_nsAttsVersion: 0,
            m_nsAttsPower: 0,
            m_position: super::xmltok::POSITION::default(),
            m_tempPool: STRING_POOL::new(),
            m_temp2Pool: STRING_POOL::new(),
            m_groupConnector: ptr::null_mut(),
            m_groupSize: 0,
            m_namespaceSeparator: 0,
            m_parentParser: ptr::null_mut(),
            m_parsingStatus: XML_ParsingStatus::default(),
            m_isParamEntity: 0,
            m_useForeignDTD: 0,
            m_paramEntityParsing: 0,
            m_hash_secret_salt: 0,

            #[cfg(feature = "mozilla")]
            m_mismatch: ptr::null(),
        }
    }

    unsafe fn create(
        mut encodingName: *const XML_Char,
        mut nameSep: *const XML_Char,
        mut dtd: *mut DTD,
    ) -> XML_Parser {
        let use_namespaces = !nameSep.is_null();
        let mut parser = XML_ParserStruct::new(use_namespaces);

        let mut parser = match Box::try_new(parser) {
            Ok(p) => p,
            Err(_) => return ptr::null_mut(),
        };

        // TODO: Move initialization into XML_ParserStruct::new
        parser.m_buffer = NULL as *mut c_char;
        parser.m_bufferLim = NULL as *const c_char;
        parser.m_attsSize = INIT_ATTS_SIZE;
        parser.m_atts = MALLOC![super::xmltok::ATTRIBUTE; parser.m_attsSize];
        if parser.m_atts.is_null() {
            return ptr::null_mut();
        }
        parser.m_dataBuf = MALLOC![XML_Char; 1024];
        if parser.m_dataBuf.is_null() {
            return ptr::null_mut();
        }
        parser.m_dataBufEnd = parser.m_dataBuf.offset(INIT_DATA_BUF_SIZE as isize);
        if !dtd.is_null() {
            parser.m_dtd = dtd
        } else {
            parser.m_dtd = dtdCreate();
            if parser.m_dtd.is_null() {
                return ptr::null_mut();
            }
        }
        parser.m_freeBindingList = NULL as *mut BINDING;
        parser.m_freeTagList = NULL as *mut TAG;
        parser.m_freeInternalEntities = NULL as *mut OPEN_INTERNAL_ENTITY;
        parser.m_groupSize = 0;
        parser.m_groupConnector = NULL as *mut c_char;
        parser.m_initEncoding = None;
        parser.m_unknownEncoding = None;
        parser.m_namespaceSeparator = ASCII_EXCL as XML_Char;
        parser.m_ns = XML_FALSE;
        parser.m_ns_triplets = XML_FALSE;
        parser.m_nsAtts = NULL as *mut NS_ATT;
        parser.m_nsAttsVersion = 0;
        parser.m_nsAttsPower = 0;
        parser.m_protocolEncodingName = NULL as *const XML_Char;

        if let XML_ParserStruct {ref mut m_tempPool, ref mut m_temp2Pool, ..} = &mut *parser {
            m_tempPool.init();
            m_temp2Pool.init();
        };

        parser.init(encodingName);
        if !encodingName.is_null() && parser.m_protocolEncodingName.is_null() {
            return ptr::null_mut();
        }
        if !nameSep.is_null() {
            parser.m_ns = XML_TRUE;
            parser.m_namespaceSeparator = *nameSep
        }

        #[cfg(feature = "mozilla")]
        {
            parser.m_mismatch = NULL as *const XML_Char;
        }
        Box::into_raw(parser)
    }

    unsafe fn init(&mut self, mut encodingName: *const XML_Char) {
        self.m_processor = Some(prologInitProcessor as Processor);
        super::xmlrole::XmlPrologStateInit(&mut self.m_prologState as *mut _);
        if !encodingName.is_null() {
            self.m_protocolEncodingName = copyString(encodingName)
        }
        self.m_curBase = NULL as *const XML_Char;
        self.m_initEncoding = InitEncoding::new(&mut self.m_encoding, ptr::null());
        self.m_encoding = &*self.m_initEncoding.as_ref().unwrap();
        self.m_userData = NULL as *mut c_void;
        self.m_handlers = Default::default();
        self.m_handlers.m_externalEntityRefHandlerArg = self as XML_Parser;
        self.m_bufferPtr = self.m_buffer;
        self.m_bufferEnd = self.m_buffer;
        self.m_parseEndByteIndex = 0i64;
        self.m_parseEndPtr = NULL as *const c_char;
        self.m_declElementType = NULL as *mut ELEMENT_TYPE;
        self.m_declAttributeId = NULL as *mut ATTRIBUTE_ID;
        self.m_declEntity = NULL as *mut ENTITY;
        self.m_doctypeName = NULL as *const XML_Char;
        self.m_doctypeSysid = NULL as *const XML_Char;
        self.m_doctypePubid = NULL as *const XML_Char;
        self.m_declAttributeType = NULL as *const XML_Char;
        self.m_declNotationName = NULL as *const XML_Char;
        self.m_declNotationPublicId = NULL as *const XML_Char;
        self.m_declAttributeIsCdata = XML_FALSE;
        self.m_declAttributeIsId = XML_FALSE;
        memset(
            &mut self.m_position as *mut super::xmltok::POSITION as *mut c_void,
            0,
            ::std::mem::size_of::<super::xmltok::POSITION>() as c_ulong,
        );
        self.m_errorCode = XML_ERROR_NONE;
        self.m_eventPtr = NULL as *const c_char;
        self.m_eventEndPtr = NULL as *const c_char;
        self.m_positionPtr = NULL as *const c_char;
        self.m_openInternalEntities = NULL as *mut OPEN_INTERNAL_ENTITY;
        self.m_defaultExpandInternalEntities = XML_TRUE;
        self.m_tagLevel = 0;
        self.m_tagStack = NULL as *mut TAG;
        self.m_inheritedBindings = NULL as *mut BINDING;
        self.m_nSpecifiedAtts = 0;
        self.m_unknownEncoding = None;
        self.m_unknownEncodingRelease = ::std::mem::transmute::<
            intptr_t,
            Option<unsafe extern "C" fn(_: *mut c_void)>,
        >(NULL as intptr_t);
        self.m_unknownEncodingData = NULL as *mut c_void;
        self.m_parentParser = NULL as XML_Parser;
        self.m_parsingStatus.parsing = XML_INITIALIZED;
        self.m_isParamEntity = XML_FALSE;
        self.m_useForeignDTD = XML_FALSE;
        self.m_paramEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
        self.m_hash_secret_salt = 0u64;
    }

    /* moves list of bindings to m_freeBindingList */
    unsafe fn moveToFreeBindingList(&mut self, mut bindings: *mut BINDING) {
        while !bindings.is_null() {
            let mut b: *mut BINDING = bindings;
            bindings = (*bindings).nextTagBinding;
            (*b).nextTagBinding = self.m_freeBindingList;
            self.m_freeBindingList = b
        }
    }
}

/* Prepare a parser object to be re-used.  This is particularly
   valuable when memory allocation overhead is disproportionately high,
   such as when a large number of small documnents need to be parsed.
   All handlers are cleared from the parser, except for the
   unknownEncodingHandler. The parser's external state is re-initialized
   except for the values of ns and ns_triplets.

   Added in Expat 1.95.3.
*/
impl XML_ParserStruct {
    pub unsafe fn reset(&mut self, encodingName: *const XML_Char) -> XML_Bool {
        let mut tStk: *mut TAG = 0 as *mut TAG;
        let mut openEntityList: *mut OPEN_INTERNAL_ENTITY = 0 as *mut OPEN_INTERNAL_ENTITY;
        if !self.m_parentParser.is_null() {
            return XML_FALSE;
        }
        /* move m_tagStack to m_freeTagList */
        tStk = self.m_tagStack;
        while !tStk.is_null() {
            let mut tag: *mut TAG = tStk;
            tStk = (*tStk).parent;
            (*tag).parent = self.m_freeTagList;
            self.moveToFreeBindingList((*tag).bindings);
            (*tag).bindings = NULL as *mut BINDING;
            self.m_freeTagList = tag
        }
        /* move m_openInternalEntities to m_freeInternalEntities */
        openEntityList = self.m_openInternalEntities;
        while !openEntityList.is_null() {
            let mut openEntity: *mut OPEN_INTERNAL_ENTITY = openEntityList;
            openEntityList = (*openEntity).next;
            (*openEntity).next = self.m_freeInternalEntities;
            self.m_freeInternalEntities = openEntity
        }
        self.moveToFreeBindingList(self.m_inheritedBindings);
        let _ = self.m_unknownEncoding.take();
        if self.m_unknownEncodingRelease.is_some() {
            self.m_unknownEncodingRelease
                .expect("non-null function pointer")(self.m_unknownEncodingData);
        }
        self.m_tempPool.clear();
        self.m_temp2Pool.clear();
        FREE!(self.m_protocolEncodingName);
        self.m_protocolEncodingName = NULL as *const XML_Char;
        self.init(encodingName);
        dtdReset(self.m_dtd);
        XML_TRUE
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserReset(parser: XML_Parser, encodingName: *const XML_Char) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    (*parser).reset(encodingName)
}
/* Returns the last value set by XML_SetUserData or NULL. */
/* This is equivalent to supplying an encoding argument to
   XML_ParserCreate. On success XML_SetEncoding returns non-zero,
   zero otherwise.
   Note: Calling XML_SetEncoding after XML_Parse or XML_ParseBuffer
     has no effect and returns XML_STATUS_ERROR.
*/

impl XML_ParserStruct {
    pub unsafe fn setEncoding(&mut self, encodingName: *const XML_Char) -> XML_Status {
        /* Block after XML_Parse()/XML_ParseBuffer() has been called.
        XXX There's no way for the caller to determine which of the
        XXX possible error cases caused the XML_STATUS_ERROR return.
        */
        if self.m_parsingStatus.parsing == XML_PARSING
            || self.m_parsingStatus.parsing == XML_SUSPENDED
        {
            return XML_STATUS_ERROR_0 as XML_Status;
        }

        /* Get rid of any previous encoding name */
        FREE!(self.m_protocolEncodingName);
        if encodingName.is_null() {
            /* No new encoding name */
            self.m_protocolEncodingName = NULL as *const XML_Char
        } else {
            /* Copy the new encoding name into allocated memory */
            self.m_protocolEncodingName = copyString(encodingName);
            if self.m_protocolEncodingName.is_null() {
                return XML_STATUS_ERROR_0 as XML_Status;
            }
        }
        XML_STATUS_OK_0 as XML_Status
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEncoding(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR_0 as XML_Status;
    }

    (*parser).setEncoding(encodingName)
}
/* Creates an XML_Parser object that can parse an external general
   entity; context is a '\0'-terminated string specifying the parse
   context; encoding is a '\0'-terminated string giving the name of
   the externally specified encoding, or NULL if there is no
   externally specified encoding.  The context string consists of a
   sequence of tokens separated by formfeeds (\f); a token consisting
   of a name specifies that the general entity of the name is open; a
   token of the form prefix=uri specifies the namespace for a
   particular prefix; a token of the form =uri specifies the default
   namespace.  This can be called at any point after the first call to
   an ExternalEntityRefHandler so longer as the parser has not yet
   been freed.  The new parser is completely independent and may
   safely be used in a separate thread.  The handlers and userData are
   initialized from the parser argument.  Returns NULL if out of memory.
   Otherwise returns a new XML_Parser object.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_ExternalEntityParserCreate(
    mut oldParser: XML_Parser,
    mut context: *const XML_Char,
    mut encodingName: *const XML_Char,
) -> XML_Parser {
    let mut parser: XML_Parser = oldParser;
    let mut newDtd: *mut DTD = NULL as *mut DTD;
    let mut oldDtd: *mut DTD = 0 as *mut DTD;
    let mut oldStartElementHandler: XML_StartElementHandler = None;
    let mut oldEndElementHandler: XML_EndElementHandler = None;
    let mut oldCharacterDataHandler: XML_CharacterDataHandler = None;
    let mut oldProcessingInstructionHandler: XML_ProcessingInstructionHandler = None;
    let mut oldCommentHandler: XML_CommentHandler = None;
    let mut oldStartCdataSectionHandler: XML_StartCdataSectionHandler = None;
    let mut oldEndCdataSectionHandler: XML_EndCdataSectionHandler = None;
    let mut oldDefaultHandler: XML_DefaultHandler = None;
    let mut oldUnparsedEntityDeclHandler: XML_UnparsedEntityDeclHandler = None;
    let mut oldNotationDeclHandler: XML_NotationDeclHandler = None;
    let mut oldStartNamespaceDeclHandler: XML_StartNamespaceDeclHandler = None;
    let mut oldEndNamespaceDeclHandler: XML_EndNamespaceDeclHandler = None;
    let mut oldNotStandaloneHandler: XML_NotStandaloneHandler = None;
    let mut oldExternalEntityRefHandler: XML_ExternalEntityRefHandler = None;
    let mut oldSkippedEntityHandler: XML_SkippedEntityHandler = None;
    let mut oldUnknownEncodingHandler: XML_UnknownEncodingHandler = None;
    let mut oldElementDeclHandler: XML_ElementDeclHandler = None;
    let mut oldAttlistDeclHandler: XML_AttlistDeclHandler = None;
    let mut oldEntityDeclHandler: XML_EntityDeclHandler = None;
    let mut oldXmlDeclHandler: XML_XmlDeclHandler = None;
    let mut oldDeclElementType: *mut ELEMENT_TYPE = 0 as *mut ELEMENT_TYPE;
    let mut oldUserData: *mut c_void = 0 as *mut c_void;
    let mut oldHandlerArg: *mut c_void = 0 as *mut c_void;
    let mut oldDefaultExpandInternalEntities: XML_Bool = 0;
    let mut oldExternalEntityRefHandlerArg: XML_Parser = 0 as *mut XML_ParserStruct;
    let mut oldParamEntityParsing: XML_ParamEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
    let mut oldInEntityValue: c_int = 0;
    let mut oldns_triplets: XML_Bool = 0;
    /* Note that the new parser shares the same hash secret as the old
       parser, so that dtdCopy and copyEntityTable can lookup values
       from hash tables associated with either parser without us having
       to worry which hash secrets each table has.
    */
    let mut oldhash_secret_salt: c_ulong = 0;
    /* Validate the oldParser parameter before we pull everything out of it */
    if oldParser.is_null() {
        return NULL as XML_Parser;
    }
    /* Stash the original parser contents on the stack */
    oldDtd = (*parser).m_dtd;
    // TODO: Maybe just copy/clone the handler struct?
    oldStartElementHandler = (*parser).m_handlers.m_startElementHandler;
    oldEndElementHandler = (*parser).m_handlers.m_endElementHandler;
    oldCharacterDataHandler = (*parser).m_handlers.m_characterDataHandler;
    oldProcessingInstructionHandler = (*parser).m_handlers.m_processingInstructionHandler;
    oldCommentHandler = (*parser).m_handlers.m_commentHandler;
    oldStartCdataSectionHandler = (*parser).m_handlers.m_startCdataSectionHandler;
    oldEndCdataSectionHandler = (*parser).m_handlers.m_endCdataSectionHandler;
    oldDefaultHandler = (*parser).m_handlers.m_defaultHandler;
    oldUnparsedEntityDeclHandler = (*parser).m_handlers.m_unparsedEntityDeclHandler;
    oldNotationDeclHandler = (*parser).m_handlers.m_notationDeclHandler;
    oldStartNamespaceDeclHandler = (*parser).m_handlers.m_startNamespaceDeclHandler;
    oldEndNamespaceDeclHandler = (*parser).m_handlers.m_endNamespaceDeclHandler;
    oldNotStandaloneHandler = (*parser).m_handlers.m_notStandaloneHandler;
    oldExternalEntityRefHandler = (*parser).m_handlers.m_externalEntityRefHandler;
    oldSkippedEntityHandler = (*parser).m_handlers.m_skippedEntityHandler;
    oldUnknownEncodingHandler = (*parser).m_handlers.m_unknownEncodingHandler;
    oldElementDeclHandler = (*parser).m_handlers.m_elementDeclHandler;
    oldAttlistDeclHandler = (*parser).m_handlers.m_attlistDeclHandler;
    oldEntityDeclHandler = (*parser).m_handlers.m_entityDeclHandler;
    oldXmlDeclHandler = (*parser).m_handlers.m_xmlDeclHandler;
    oldDeclElementType = (*parser).m_declElementType;
    oldUserData = (*parser).m_userData;
    oldHandlerArg = (*parser).m_handlers.m_handlerArg;
    oldDefaultExpandInternalEntities = (*parser).m_defaultExpandInternalEntities;
    oldExternalEntityRefHandlerArg = (*parser).m_handlers.m_externalEntityRefHandlerArg;
    oldParamEntityParsing = (*parser).m_paramEntityParsing;
    oldInEntityValue = (*parser).m_prologState.inEntityValue;
    oldns_triplets = (*parser).m_ns_triplets;
    /* Note that the new parser shares the same hash secret as the old
       parser, so that dtdCopy and copyEntityTable can lookup values
       from hash tables associated with either parser without us having
       to worry which hash secrets each table has.
    */
    oldhash_secret_salt = (*parser).m_hash_secret_salt;
    if context.is_null() {
        newDtd = oldDtd
    }
    /* XML_DTD */
    /* Note that the magical uses of the pre-processor to make field
       access look more like C++ require that `parser' be overwritten
       here.  This makes this function more painful to follow than it
       would be otherwise.
    */
    if (*parser).m_ns != 0 {
        let mut tmp: [XML_Char; 2] = [0; 2];
        *tmp.as_mut_ptr() = (*parser).m_namespaceSeparator;
        parser = XML_ParserStruct::create(encodingName, tmp.as_mut_ptr(), newDtd)
    } else {
        parser = XML_ParserStruct::create(
            encodingName,
            NULL as *const XML_Char,
            newDtd,
        )
    }
    if parser.is_null() {
        return NULL as XML_Parser;
    }
    (*parser).m_handlers.setStartElement(oldStartElementHandler);
    (*parser).m_handlers.setEndElement(oldEndElementHandler);
    (*parser).m_handlers.setCharacterData(oldCharacterDataHandler);
    (*parser).m_handlers.setProcessingInstruction(oldProcessingInstructionHandler);
    (*parser).m_handlers.setComment(oldCommentHandler);
    (*parser).m_handlers.setStartCDataSection(oldStartCdataSectionHandler);
    (*parser).m_handlers.setEndCDataSection(oldEndCdataSectionHandler);
    (*parser).m_handlers.setDefault(oldDefaultHandler);
    (*parser).m_handlers.setUnparsedEntityDecl(oldUnparsedEntityDeclHandler);
    (*parser).m_handlers.setNotationDecl(oldNotationDeclHandler);
    (*parser).m_handlers.setStartNamespaceDecl(oldStartNamespaceDeclHandler);
    (*parser).m_handlers.setEndNamespaceDecl(oldEndNamespaceDeclHandler);
    (*parser).m_handlers.setNotStandalone(oldNotStandaloneHandler);
    (*parser).m_handlers.setExternalEntityRef(oldExternalEntityRefHandler);
    (*parser).m_handlers.setSkippedEntity(oldSkippedEntityHandler);
    (*parser).m_handlers.setUnknownEncoding(oldUnknownEncodingHandler);
    (*parser).m_handlers.setElementDecl(oldElementDeclHandler);
    (*parser).m_handlers.setAttlistDecl(oldAttlistDeclHandler);
    (*parser).m_handlers.setEntityDecl(oldEntityDeclHandler);
    (*parser).m_handlers.setXmlDecl(oldXmlDeclHandler);
    (*parser).m_declElementType = oldDeclElementType;
    (*parser).m_userData = oldUserData;
    if oldUserData == oldHandlerArg {
        (*parser).m_handlers.m_handlerArg = (*parser).m_userData
    } else {
        (*parser).m_handlers.m_handlerArg = parser as *mut c_void
    }
    if oldExternalEntityRefHandlerArg != oldParser {
        (*parser).m_handlers.m_externalEntityRefHandlerArg = oldExternalEntityRefHandlerArg
    }
    (*parser).m_defaultExpandInternalEntities = oldDefaultExpandInternalEntities;
    (*parser).m_ns_triplets = oldns_triplets;
    (*parser).m_hash_secret_salt = oldhash_secret_salt;
    (*parser).m_parentParser = oldParser;
    (*parser).m_paramEntityParsing = oldParamEntityParsing;
    (*parser).m_prologState.inEntityValue = oldInEntityValue;
    if !context.is_null() {
        /* XML_DTD */
        if dtdCopy((*parser).m_dtd, oldDtd) == 0
            || (*parser).setContext(context) == 0
        {
            XML_ParserFree(parser);
            return NULL as XML_Parser;
        }
        (*parser).m_processor = Some(externalEntityInitProcessor as Processor)
    } else {
        /* The DTD instance referenced by parser->m_dtd is shared between the
           document's root parser and external PE parsers, therefore one does not
           need to call setContext. In addition, one also *must* not call
           setContext, because this would overwrite existing prefix->binding
           pointers in parser->m_dtd with ones that get destroyed with the external
           PE parser. This would leave those prefixes with dangling pointers.
        */
        (*parser).m_isParamEntity = XML_TRUE;
        super::xmlrole::XmlPrologStateInitExternalEntity(&mut (*parser).m_prologState as *mut _);
        (*parser).m_processor = Some(externalParEntInitProcessor as Processor)
    }
    /* XML_DTD */
    parser
}

unsafe fn destroyBindings(mut bindings: *mut BINDING) {
    loop {
        let mut b: *mut BINDING = bindings;
        if b.is_null() {
            break;
        }
        bindings = (*b).nextTagBinding;
        FREE!((*b).uri);
        FREE!(b);
    }
}

impl Drop for XML_ParserStruct {
    /* Frees memory used by the parser. */
    fn drop(&mut self) {
        let mut tagList: *mut TAG = 0 as *mut TAG;
        let mut entityList: *mut OPEN_INTERNAL_ENTITY = 0 as *mut OPEN_INTERNAL_ENTITY;
        /* free m_tagStack and m_freeTagList */
        tagList = self.m_tagStack;
        unsafe {
            loop {
                let mut p: *mut TAG = 0 as *mut TAG;
                if tagList.is_null() {
                    if self.m_freeTagList.is_null() {
                        break;
                    }
                    tagList = self.m_freeTagList;
                    self.m_freeTagList = NULL as *mut TAG
                }
                p = tagList;
                tagList = (*tagList).parent;
                FREE!((*p).buf);
                destroyBindings((*p).bindings);
            }
            /* free m_openInternalEntities and m_freeInternalEntities */
            entityList = self.m_openInternalEntities;
            loop {
                let mut openEntity: *mut OPEN_INTERNAL_ENTITY = 0 as *mut OPEN_INTERNAL_ENTITY;
                if entityList.is_null() {
                    if self.m_freeInternalEntities.is_null() {
                        break;
                    }
                    entityList = self.m_freeInternalEntities;
                    self.m_freeInternalEntities = NULL as *mut OPEN_INTERNAL_ENTITY
                }
                openEntity = entityList;
                entityList = (*entityList).next;
                FREE!(openEntity);
            }
            destroyBindings(self.m_freeBindingList);
            destroyBindings(self.m_inheritedBindings);
            self.m_tempPool.destroy();
            self.m_temp2Pool.destroy();
            FREE!(self.m_protocolEncodingName);
            /* external parameter entity parsers share the DTD structure
            parser->m_dtd with the root parser, so we must not destroy it
            */
            if self.m_isParamEntity == 0 && !self.m_dtd.is_null() {
                /* XML_DTD */
                dtdDestroy(
                    self.m_dtd,
                    self.m_parentParser.is_null() as XML_Bool,
                );
            }
            FREE!(self.m_atts);
            FREE!(self.m_groupConnector);
            FREE!(self.m_buffer);
            FREE!(self.m_dataBuf);
            FREE!(self.m_nsAtts);
            if self.m_unknownEncodingRelease.is_some() {
                self.m_unknownEncodingRelease
                    .expect("non-null function pointer")(self.m_unknownEncodingData);
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserFree(parser: XML_Parser) {
    if parser.is_null() {
        return;
    }
    let _ = Box::from_raw(parser);
}
/* If this function is called, then the parser will be passed as the
   first argument to callbacks instead of userData.  The userData will
   still be accessible using XML_GetUserData.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_UseParserAsHandlerArg(mut parser: XML_Parser) {
    if !parser.is_null() {
        (*parser).m_handlers.m_handlerArg = parser as *mut c_void
    };
}
/* If useDTD == XML_TRUE is passed to this function, then the parser
   will assume that there is an external subset, even if none is
   specified in the document. In such a case the parser will call the
   externalEntityRefHandler with a value of NULL for the systemId
   argument (the publicId and context arguments will be NULL as well).
   Note: For the purpose of checking WFC: Entity Declared, passing
     useDTD == XML_TRUE will make the parser behave as if the document
     had a DTD with an external subset.
   Note: If this function is called, then this must be done before
     the first call to XML_Parse or XML_ParseBuffer, since it will
     have no effect after that.  Returns
     XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING.
   Note: If the document does not have a DOCTYPE declaration at all,
     then startDoctypeDeclHandler and endDoctypeDeclHandler will not
     be called, despite an external subset being parsed.
   Note: If XML_DTD is not defined when Expat is compiled, returns
     XML_ERROR_FEATURE_REQUIRES_XML_DTD.
   Note: If parser == NULL, returns XML_ERROR_INVALID_ARGUMENT.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_UseForeignDTD(
    mut parser: XML_Parser,
    mut useDTD: XML_Bool,
) -> XML_Error {
    if parser.is_null() {
        return XML_ERROR_INVALID_ARGUMENT;
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing == XML_PARSING
        || (*parser).m_parsingStatus.parsing == XML_SUSPENDED
    {
        return XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
    }
    (*parser).m_useForeignDTD = useDTD;
    XML_ERROR_NONE
}
/* If do_nst is non-zero, and namespace processing is in effect, and
   a name has a prefix (i.e. an explicit namespace qualifier) then
   that name is returned as a triplet in a single string separated by
   the separator character specified when the parser was created: URI
   + sep + local_name + sep + prefix.

   If do_nst is zero, then namespace information is returned in the
   default manner (URI + sep + local_name) whether or not the name
   has a prefix.

   Note: Calling XML_SetReturnNSTriplet after XML_Parse or
     XML_ParseBuffer has no effect.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_SetReturnNSTriplet(mut parser: XML_Parser, mut do_nst: XML_Bool) {
    if parser.is_null() {
        return;
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing == XML_PARSING
        || (*parser).m_parsingStatus.parsing == XML_SUSPENDED
    {
        return;
    }
    (*parser).m_ns_triplets = if do_nst != 0 { XML_TRUE } else { XML_FALSE };
}
/* This value is passed as the userData argument to callbacks. */
#[no_mangle]
pub unsafe extern "C" fn XML_SetUserData(mut parser: XML_Parser, mut p: *mut c_void) {
    if parser.is_null() {
        return;
    }
    if (*parser).m_handlers.m_handlerArg == (*parser).m_userData {
        (*parser).m_userData = p;
        (*parser).m_handlers.m_handlerArg = (*parser).m_userData
    } else {
        (*parser).m_userData = p
    };
}
/* Sets the base to be used for resolving relative URIs in system
   identifiers in declarations.  Resolving relative identifiers is
   left to the application: this value will be passed through as the
   base argument to the XML_ExternalEntityRefHandler,
   XML_NotationDeclHandler and XML_UnparsedEntityDeclHandler. The base
   argument will be copied.  Returns XML_STATUS_ERROR if out of memory,
   XML_STATUS_OK otherwise.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_SetBase(mut parser: XML_Parser, mut p: *const XML_Char) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR_0 as XML_Status;
    }
    if !p.is_null() {
        p = (*(*parser).m_dtd).pool.copyString(p);
        if p.is_null() {
            return XML_STATUS_ERROR_0 as XML_Status;
        }
        (*parser).m_curBase = p
    } else {
        (*parser).m_curBase = NULL as *const XML_Char
    }
    XML_STATUS_OK_0 as XML_Status
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetBase(mut parser: XML_Parser) -> *const XML_Char {
    if parser.is_null() {
        return NULL as *const XML_Char;
    }
    (*parser).m_curBase
}
/* Returns the number of the attribute/value pairs passed in last call
   to the XML_StartElementHandler that were specified in the start-tag
   rather than defaulted. Each attribute/value pair counts as 2; thus
   this correspondds to an index into the atts array passed to the
   XML_StartElementHandler.  Returns -1 if parser == NULL.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_GetSpecifiedAttributeCount(mut parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return -(1i32);
    }
    (*parser).m_nSpecifiedAtts
}
/* Returns the index of the ID attribute passed in the last call to
   XML_StartElementHandler, or -1 if there is no ID attribute or
   parser == NULL.  Each attribute/value pair counts as 2; thus this
   correspondds to an index into the atts array passed to the
   XML_StartElementHandler.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_GetIdAttributeIndex(mut parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return -(1i32);
    }
    (*parser).m_idAttIndex
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetElementHandler(
    mut parser: XML_Parser,
    mut start: XML_StartElementHandler,
    mut end: XML_EndElementHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_handlers.setStartElement(start);
    (*parser).m_handlers.setEndElement(end);
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetStartElementHandler(
    mut parser: XML_Parser,
    mut start: XML_StartElementHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setStartElement(start)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEndElementHandler(
    mut parser: XML_Parser,
    mut end: XML_EndElementHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setEndElement(end)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetCharacterDataHandler(
    mut parser: XML_Parser,
    mut handler: XML_CharacterDataHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setCharacterData(handler)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetProcessingInstructionHandler(
    mut parser: XML_Parser,
    mut handler: XML_ProcessingInstructionHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setProcessingInstruction(handler)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetCommentHandler(
    mut parser: XML_Parser,
    mut handler: XML_CommentHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setComment(handler)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetCdataSectionHandler(
    mut parser: XML_Parser,
    mut start: XML_StartCdataSectionHandler,
    mut end: XML_EndCdataSectionHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_handlers.setStartCDataSection(start);
    (*parser).m_handlers.setEndCDataSection(end);
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetStartCdataSectionHandler(
    mut parser: XML_Parser,
    mut start: XML_StartCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setStartCDataSection(start);
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEndCdataSectionHandler(
    mut parser: XML_Parser,
    mut end: XML_EndCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setEndCDataSection(end)
    };
}
/* This sets the default handler and also inhibits expansion of
   internal entities. These entity references will be passed to the
   default handler, or to the skipped entity handler, if one is set.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandler(
    mut parser: XML_Parser,
    mut handler: XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_handlers.setDefault(handler);
    (*parser).m_defaultExpandInternalEntities = XML_FALSE;
}
/* This sets the default handler but does not inhibit expansion of
   internal entities.  The entity reference will not be passed to the
   default handler.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandlerExpand(
    mut parser: XML_Parser,
    mut handler: XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_handlers.setDefault(handler);
    (*parser).m_defaultExpandInternalEntities = XML_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartDoctypeDeclHandler,
    mut end: XML_EndDoctypeDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_handlers.setStartDoctypeDecl(start);
    (*parser).m_handlers.setEndDoctypeDecl(end);
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetStartDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setStartDoctypeDecl(start)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEndDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut end: XML_EndDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setEndDoctypeDecl(end)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetUnparsedEntityDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_UnparsedEntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setUnparsedEntityDecl(handler)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetNotationDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_NotationDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setNotationDecl(handler)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartNamespaceDeclHandler,
    mut end: XML_EndNamespaceDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_handlers.setStartNamespaceDecl(start);
    (*parser).m_handlers.setEndNamespaceDecl(end);
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetStartNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setStartNamespaceDecl(start)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEndNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut end: XML_EndNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setEndNamespaceDecl(end)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetNotStandaloneHandler(
    mut parser: XML_Parser,
    mut handler: XML_NotStandaloneHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setNotStandalone(handler)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetExternalEntityRefHandler(
    mut parser: XML_Parser,
    mut handler: XML_ExternalEntityRefHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setExternalEntityRef(handler)
    };
}
/* If a non-NULL value for arg is specified here, then it will be
   passed as the first argument to the external entity ref handler
   instead of the parser object.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_SetExternalEntityRefHandlerArg(
    mut parser: XML_Parser,
    mut arg: *mut c_void,
) {
    if parser.is_null() {
        return;
    }
    if !arg.is_null() {
        (*parser).m_handlers.m_externalEntityRefHandlerArg = arg as XML_Parser
    } else {
        (*parser).m_handlers.m_externalEntityRefHandlerArg = parser
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetSkippedEntityHandler(
    mut parser: XML_Parser,
    mut handler: XML_SkippedEntityHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setSkippedEntity(handler)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetUnknownEncodingHandler(
    mut parser: XML_Parser,
    mut handler: XML_UnknownEncodingHandler,
    mut data: *mut c_void,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_handlers.setUnknownEncoding(handler);
    (*parser).m_handlers.m_unknownEncodingHandlerData = data;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetElementDeclHandler(
    mut parser: XML_Parser,
    mut eldecl: XML_ElementDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setElementDecl(eldecl)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetAttlistDeclHandler(
    mut parser: XML_Parser,
    mut attdecl: XML_AttlistDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setAttlistDecl(attdecl)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEntityDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_EntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setEntityDecl(handler)
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetXmlDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_XmlDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_handlers.setXmlDecl(handler)
    };
}
/* Controls parsing of parameter entities (including the external DTD
   subset). If parsing of parameter entities is enabled, then
   references to external parameter entities (including the external
   DTD subset) will be passed to the handler set with
   XML_SetExternalEntityRefHandler.  The context passed will be 0.

   Unlike external general entities, external parameter entities can
   only be parsed synchronously.  If the external parameter entity is
   to be parsed, it must be parsed during the call to the external
   entity ref handler: the complete sequence of
   XML_ExternalEntityParserCreate, XML_Parse/XML_ParseBuffer and
   XML_ParserFree calls must be made during this call.  After
   XML_ExternalEntityParserCreate has been called to create the parser
   for the external parameter entity (context must be 0 for this
   call), it is illegal to make any calls on the old parser until
   XML_ParserFree has been called on the newly created parser.
   If the library has been compiled without support for parameter
   entity parsing (ie without XML_DTD being defined), then
   XML_SetParamEntityParsing will return 0 if parsing of parameter
   entities is requested; otherwise it will return non-zero.
   Note: If XML_SetParamEntityParsing is called after XML_Parse or
      XML_ParseBuffer, then it has no effect and will always return 0.
   Note: If parser == NULL, the function will do nothing and return 0.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_SetParamEntityParsing(
    mut parser: XML_Parser,
    mut peParsing: XML_ParamEntityParsing,
) -> c_int {
    if parser.is_null() {
        return 0i32;
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing == XML_PARSING
        || (*parser).m_parsingStatus.parsing == XML_SUSPENDED
    {
        return 0i32;
    }
    (*parser).m_paramEntityParsing = peParsing;
    1
}
/* Sets the hash salt to use for internal hash calculations.
   Helps in preventing DoS attacks based on predicting hash
   function behavior. This must be called before parsing is started.
   Returns 1 if successful, 0 when called after parsing has started.
   Note: If parser == NULL, the function will do nothing and return 0.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_SetHashSalt(mut parser: XML_Parser, mut hash_salt: c_ulong) -> c_int {
    if parser.is_null() {
        return 0i32;
    }
    if !(*parser).m_parentParser.is_null() {
        return XML_SetHashSalt((*parser).m_parentParser, hash_salt);
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing == XML_PARSING
        || (*parser).m_parsingStatus.parsing == XML_SUSPENDED
    {
        return 0i32;
    }
    (*parser).m_hash_secret_salt = hash_salt;
    1
}
/* Parses some input. Returns XML_STATUS_ERROR if a fatal error is
   detected.  The last call to XML_Parse must have isFinal true; len
   may be zero for this call (or any other).

   Though the return values for these functions has always been
   described as a Boolean value, the implementation, at least for the
   1.95.x series, has always returned exactly one of the XML_Status
   values.
*/

impl XML_ParserStruct {
    pub unsafe fn parse(&mut self, s: *const c_char, len: c_int, isFinal: c_int) -> XML_Status {
        if len < 0 || s.is_null() && len != 0 {
            return XML_STATUS_ERROR_0 as XML_Status;
        }
        match self.m_parsingStatus.parsing {
            3 => {
                self.m_errorCode = XML_ERROR_SUSPENDED;
                return XML_STATUS_ERROR_0 as XML_Status;
            }
            2 => {
                self.m_errorCode = XML_ERROR_FINISHED;
                return XML_STATUS_ERROR_0 as XML_Status;
            }
            0 => {
                if self.m_parentParser.is_null() && self.startParsing() == 0 {
                    self.m_errorCode = XML_ERROR_NO_MEMORY;
                    return XML_STATUS_ERROR_0 as XML_Status;
                }
            }
            _ => {}
        }
        /* fall through */
        self.m_parsingStatus.parsing = XML_PARSING;
        if len == 0 {
            self.m_parsingStatus.finalBuffer = isFinal as XML_Bool;
            if isFinal == 0 {
                return XML_STATUS_OK_0 as XML_Status;
            }
            self.m_positionPtr = self.m_bufferPtr;
            self.m_parseEndPtr = self.m_bufferEnd;
            /* If data are left over from last buffer, and we now know that these
            data are the final chunk of input, then we have to check them again
            to detect errors based on that fact.
            */
            self.m_errorCode = self.m_processor.expect("non-null function pointer")(
                self,
                self.m_bufferPtr,
                self.m_parseEndPtr,
                &mut self.m_bufferPtr,
            );
            if self.m_errorCode == XML_ERROR_NONE {
                match self.m_parsingStatus.parsing {
                    3 => {
                        /* It is hard to be certain, but it seems that this case
                        * cannot occur.  This code is cleaning up a previous parse
                        * with no new data (since len == 0).  Changing the parsing
                        * state requires getting to execute a handler function, and
                        * there doesn't seem to be an opportunity for that while in
                        * this circumstance.
                        *
                        * Given the uncertainty, we retain the code but exclude it
                        * from coverage tests.
                        *
                        * LCOV_EXCL_START
                        */
                        (*self.m_encoding).updatePosition(
                            self.m_positionPtr,
                            self.m_bufferPtr,
                            &mut self.m_position,
                        );
                        self.m_positionPtr = self.m_bufferPtr;
                        return XML_STATUS_SUSPENDED_0 as XML_Status;
                    }
                    0 | 1 => {
                        /* LCOV_EXCL_STOP */
                        self.m_parsingStatus.parsing = XML_FINISHED
                    }
                    _ => {}
                }
                /* fall through */
                return XML_STATUS_OK_0 as XML_Status;
            }
            self.m_eventEndPtr = self.m_eventPtr;
            self.m_processor = Some(errorProcessor as Processor);
            XML_STATUS_ERROR_0 as XML_Status
        } else {
            /* not defined XML_CONTEXT_BYTES */
            let mut buff: *mut c_void = self.getBuffer(len);
            if buff.is_null() {
                XML_STATUS_ERROR_0 as XML_Status
            } else {
                memcpy(buff, s as *const c_void, len as c_ulong);
                XML_ParseBuffer(self, len, isFinal)
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_Parse(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut len: c_int,
    mut isFinal: c_int,
) -> XML_Status {
    if parser.is_null() || len < 0 || s.is_null() && len != 0 {
        if !parser.is_null() {
            (*parser).m_errorCode = XML_ERROR_INVALID_ARGUMENT
        }
        return XML_STATUS_ERROR_0 as XML_Status;
    }
    (*parser).parse(s, len, isFinal)
}

impl XML_ParserStruct {
    pub unsafe fn parseBuffer(&mut self, len: c_int, isFinal: c_int) -> XML_Status {
        let mut start: *const c_char = 0 as *const c_char;
        let mut result: XML_Status = XML_STATUS_OK_0 as XML_Status;
        match self.m_parsingStatus.parsing {
            3 => {
                self.m_errorCode = XML_ERROR_SUSPENDED;
                return XML_STATUS_ERROR_0 as XML_Status;
            }
            2 => {
                self.m_errorCode = XML_ERROR_FINISHED;
                return XML_STATUS_ERROR_0 as XML_Status;
            }
            0 => {
                if self.m_parentParser.is_null() && self.startParsing() == 0 {
                    self.m_errorCode = XML_ERROR_NO_MEMORY;
                    return XML_STATUS_ERROR_0 as XML_Status;
                }
            }
            _ => {}
        }
        /* fall through */
        self.m_parsingStatus.parsing = XML_PARSING;
        start = self.m_bufferPtr;
        self.m_positionPtr = start;
        self.m_bufferEnd = self.m_bufferEnd.offset(len as isize);
        self.m_parseEndPtr = self.m_bufferEnd;
        self.m_parseEndByteIndex += len as c_long;
        self.m_parsingStatus.finalBuffer = isFinal as XML_Bool;
        self.m_errorCode = self.m_processor.expect("non-null function pointer")(
            self,
            start,
            self.m_parseEndPtr,
            &mut self.m_bufferPtr,
        );
        if self.m_errorCode != XML_ERROR_NONE {
            self.m_eventEndPtr = self.m_eventPtr;
            self.m_processor = Some(errorProcessor as Processor);
            return XML_STATUS_ERROR_0 as XML_Status;
        } else {
            match self.m_parsingStatus.parsing {
                3 => {
                    result = XML_STATUS_SUSPENDED_0 as XML_Status
                    /* should not happen */
                }
                0 | 1 => {
                    if isFinal != 0 {
                        self.m_parsingStatus.parsing = XML_FINISHED;
                        return result;
                    }
                }
                _ => {}
            }
        }
        (*self.m_encoding).updatePosition(
            self.m_positionPtr,
            self.m_bufferPtr,
            &mut self.m_position,
        );
        self.m_positionPtr = self.m_bufferPtr;
        return result;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParseBuffer(
    mut parser: XML_Parser,
    mut len: c_int,
    mut isFinal: c_int,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR_0 as XML_Status;
    }

    (*parser).parseBuffer(len, isFinal)
}

impl XML_ParserStruct {
    pub unsafe fn getBuffer(&mut self, len: c_int) -> *mut c_void {
        if len < 0 {
            self.m_errorCode = XML_ERROR_NO_MEMORY;
            return NULL as *mut c_void;
        }
        match self.m_parsingStatus.parsing {
            3 => {
                self.m_errorCode = XML_ERROR_SUSPENDED;
                return NULL as *mut c_void;
            }
            2 => {
                self.m_errorCode = XML_ERROR_FINISHED;
                return NULL as *mut c_void;
            }
            _ => {}
        }
        if len as c_long
            > (if !self.m_bufferLim.is_null() && !self.m_bufferEnd.is_null() {
                self.m_bufferLim
                    .wrapping_offset_from(self.m_bufferEnd) as c_long
            } else {
                0
            })
        {
            let mut keep: c_int = 0;
            /* defined XML_CONTEXT_BYTES */
            /* Do not invoke signed arithmetic overflow: */
            let mut neededSize: c_int = (len as c_uint).wrapping_add(
                (if !self.m_bufferEnd.is_null() && !self.m_bufferPtr.is_null() {
                    self.m_bufferEnd
                        .wrapping_offset_from(self.m_bufferPtr) as c_long
                } else {
                    0
                }) as c_uint,
            ) as c_int;
            if neededSize < 0 {
                self.m_errorCode = XML_ERROR_NO_MEMORY;
                return NULL as *mut c_void;
            }
            keep = if !self.m_bufferPtr.is_null() && !self.m_buffer.is_null() {
                self.m_bufferPtr
                    .wrapping_offset_from(self.m_buffer) as c_long
            } else {
                0
            } as c_int;
            if keep > XML_CONTEXT_BYTES {
                keep = XML_CONTEXT_BYTES
            }
            neededSize += keep;
            /* defined XML_CONTEXT_BYTES */
            if neededSize as c_long
                <= (if !self.m_bufferLim.is_null() && !self.m_buffer.is_null() {
                    self.m_bufferLim
                        .wrapping_offset_from(self.m_buffer) as c_long
                } else {
                    0
                })
            {
                if (keep as c_long)
                    < (if !self.m_bufferPtr.is_null() && !self.m_buffer.is_null() {
                        self.m_bufferPtr
                            .wrapping_offset_from(self.m_buffer) as c_long
                    } else {
                        0
                    })
                {
                    let mut offset: c_int = (if !self.m_bufferPtr.is_null()
                        && !self.m_buffer.is_null()
                    {
                        self.m_bufferPtr
                            .wrapping_offset_from(self.m_buffer) as c_long
                    } else {
                        0
                    }) as c_int
                        - keep;
                    /* The buffer pointers cannot be NULL here; we have at least some bytes
                     * in the buffer */
                    memmove(
                        self.m_buffer as *mut c_void,
                        &mut *self.m_buffer.offset(offset as isize) as *mut c_char
                            as *const c_void,
                        (self
                            .m_bufferEnd
                            .wrapping_offset_from(self.m_bufferPtr) as c_long
                            + keep as c_long) as c_ulong,
                    );
                    self.m_bufferEnd = self.m_bufferEnd.offset(-(offset as isize));
                    self.m_bufferPtr = self.m_bufferPtr.offset(-(offset as isize))
                }
            /* not defined XML_CONTEXT_BYTES */
            } else {
                let mut newBuf: *mut c_char = 0 as *mut c_char;
                let mut bufferSize: c_int =
                    if !self.m_bufferLim.is_null() && !self.m_bufferPtr.is_null() {
                        self.m_bufferLim
                            .wrapping_offset_from(self.m_bufferPtr) as c_long
                    } else {
                        0
                    } as c_int;
                if bufferSize == 0 {
                    bufferSize = INIT_BUFFER_SIZE
                }
                loop {
                    /* not defined XML_CONTEXT_BYTES */
                    /* Do not invoke signed arithmetic overflow: */
                    bufferSize = (2u32).wrapping_mul(bufferSize as c_uint) as c_int;
                    if !(bufferSize < neededSize && bufferSize > 0) {
                        break;
                    }
                }
                if bufferSize <= 0 {
                    self.m_errorCode = XML_ERROR_NO_MEMORY;
                    return NULL as *mut c_void;
                }
                newBuf = MALLOC!(bufferSize as size_t) as *mut c_char;
                if newBuf.is_null() {
                    self.m_errorCode = XML_ERROR_NO_MEMORY;
                    return NULL as *mut c_void;
                }
                self.m_bufferLim = newBuf.offset(bufferSize as isize);
                if !self.m_bufferPtr.is_null() {
                    memcpy(
                        newBuf as *mut c_void,
                        &*self.m_bufferPtr.offset(-keep as isize) as *const c_char
                            as *const c_void,
                        ((if !self.m_bufferEnd.is_null() && !self.m_bufferPtr.is_null() {
                            self.m_bufferEnd
                                .wrapping_offset_from(self.m_bufferPtr)
                                as c_long
                        } else {
                            0
                        }) + keep as c_long) as c_ulong,
                    );
                    FREE!(self.m_buffer);
                    self.m_buffer = newBuf;
                    self.m_bufferEnd = self
                        .m_buffer
                        .offset(
                            (if !self.m_bufferEnd.is_null() && !self.m_bufferPtr.is_null() {
                                self.m_bufferEnd
                                    .wrapping_offset_from(self.m_bufferPtr)
                                    as c_long
                            } else {
                                0
                            }) as isize,
                        )
                        .offset(keep as isize);
                    self.m_bufferPtr = self.m_buffer.offset(keep as isize)
                } else {
                    /* This must be a brand new buffer with no data in it yet */
                    self.m_bufferEnd = newBuf;
                    self.m_buffer = newBuf;
                    self.m_bufferPtr = self.m_buffer
                }
            }
            self.m_eventEndPtr = NULL as *const c_char;
            self.m_eventPtr = self.m_eventEndPtr;
            self.m_positionPtr = NULL as *const c_char
        }
        self.m_bufferEnd as *mut c_void
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetBuffer(mut parser: XML_Parser, mut len: c_int) -> *mut c_void {
    if parser.is_null() {
        return NULL as *mut c_void;
    }

    (*parser).getBuffer(len)
}
/* Stops parsing, causing XML_Parse() or XML_ParseBuffer() to return.
   Must be called from within a call-back handler, except when aborting
   (resumable = 0) an already suspended parser. Some call-backs may
   still follow because they would otherwise get lost. Examples:
   - endElementHandler() for empty elements when stopped in
     startElementHandler(),
   - endNameSpaceDeclHandler() when stopped in endElementHandler(),
   and possibly others.

   Can be called from most handlers, including DTD related call-backs,
   except when parsing an external parameter entity and resumable != 0.
   Returns XML_STATUS_OK when successful, XML_STATUS_ERROR otherwise.
   Possible error codes:
   - XML_ERROR_SUSPENDED: when suspending an already suspended parser.
   - XML_ERROR_FINISHED: when the parser has already finished.
   - XML_ERROR_SUSPEND_PE: when suspending while parsing an external PE.

   When resumable != 0 (true) then parsing is suspended, that is,
   XML_Parse() and XML_ParseBuffer() return XML_STATUS_SUSPENDED.
   Otherwise, parsing is aborted, that is, XML_Parse() and XML_ParseBuffer()
   return XML_STATUS_ERROR with error code XML_ERROR_ABORTED.

   *Note*:
   This will be applied to the current parser instance only, that is, if
   there is a parent parser then it will continue parsing when the
   externalEntityRefHandler() returns. It is up to the implementation of
   the externalEntityRefHandler() to call XML_StopParser() on the parent
   parser (recursively), if one wants to stop parsing altogether.

   When suspended, parsing can be resumed by calling XML_ResumeParser().
*/

impl XML_ParserStruct {
    pub unsafe fn stopParser(&mut self, resumable: XML_Bool) -> XML_Status {
        match self.m_parsingStatus.parsing {
            3 => {
                if resumable != 0 {
                    self.m_errorCode = XML_ERROR_SUSPENDED;
                    return XML_STATUS_ERROR_0 as XML_Status;
                }
                self.m_parsingStatus.parsing = XML_FINISHED
            }
            2 => {
                self.m_errorCode = XML_ERROR_FINISHED;
                return XML_STATUS_ERROR_0 as XML_Status;
            }
            _ => {
                if resumable != 0 {
                    if self.m_isParamEntity != 0 {
                        self.m_errorCode = XML_ERROR_SUSPEND_PE;
                        return XML_STATUS_ERROR_0 as XML_Status;
                    }
                    self.m_parsingStatus.parsing = XML_SUSPENDED
                } else {
                    self.m_parsingStatus.parsing = XML_FINISHED
                }
            }
        }
        XML_STATUS_OK_0 as XML_Status
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_StopParser(parser: XML_Parser, resumable: XML_Bool) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR_0 as XML_Status;
    }
    (*parser).stopParser(resumable)
}
/* Resumes parsing after it has been suspended with XML_StopParser().
   Must not be called from within a handler call-back. Returns same
   status codes as XML_Parse() or XML_ParseBuffer().
   Additional error code XML_ERROR_NOT_SUSPENDED possible.

   *Note*:
   This must be called on the most deeply nested child parser instance
   first, and on its parent parser only after the child parser has finished,
   to be applied recursively until the document entity's parser is restarted.
   That is, the parent parser will not resume by itself and it is up to the
   application to call XML_ResumeParser() on it at the appropriate moment.
*/
impl XML_ParserStruct {
    pub unsafe fn resumeParser(&mut self) -> XML_Status {
        let mut result: XML_Status = XML_STATUS_OK_0 as XML_Status;
        if self.m_parsingStatus.parsing != XML_SUSPENDED {
            self.m_errorCode = XML_ERROR_NOT_SUSPENDED;
            return XML_STATUS_ERROR_0 as XML_Status;
        }
        self.m_parsingStatus.parsing = XML_PARSING;
        self.m_errorCode = self.m_processor.expect("non-null function pointer")(
            self,
            self.m_bufferPtr,
            self.m_parseEndPtr,
            &mut self.m_bufferPtr,
        );
        if self.m_errorCode != XML_ERROR_NONE {
            self.m_eventEndPtr = self.m_eventPtr;
            self.m_processor = Some(errorProcessor as Processor);
            return XML_STATUS_ERROR_0 as XML_Status;
        } else {
            match self.m_parsingStatus.parsing {
                3 => result = XML_STATUS_SUSPENDED_0 as XML_Status,
                0 | 1 => {
                    if self.m_parsingStatus.finalBuffer != 0 {
                        self.m_parsingStatus.parsing = XML_FINISHED;
                        return result;
                    }
                }
                _ => {}
            }
        }
        (*self.m_encoding).updatePosition(
            self.m_positionPtr,
            self.m_bufferPtr,
            &mut self.m_position,
        );
        self.m_positionPtr = self.m_bufferPtr;

        #[cfg(feature = "mozilla")]
        {
            self.m_eventPtr = self.m_bufferPtr;
            self.m_eventEndPtr = self.m_bufferPtr;
        }
        return result;
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ResumeParser(mut parser: XML_Parser) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR_0 as XML_Status;
    }

    (*parser).resumeParser()
}
/* Returns status of parser with respect to being initialized, parsing,
   finished, or suspended and processing the final buffer.
   XXX XML_Parse() and XML_ParseBuffer() should return XML_ParsingStatus,
   XXX with XML_FINISHED_OK or XML_FINISHED_ERROR replacing XML_FINISHED
*/
#[no_mangle]
pub unsafe extern "C" fn XML_GetParsingStatus(
    mut parser: XML_Parser,
    mut status: *mut XML_ParsingStatus,
) {
    if parser.is_null() {
        return;
    }
    if !status.is_null() {
    } else {
        __assert_fail(
            b"status != NULL\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/lib/xmlparse.c\x00".as_ptr()
                as *const c_char,
            2113u32,
            (*::std::mem::transmute::<&[u8; 59], &[c_char; 59]>(
                b"void XML_GetParsingStatus(XML_Parser, XML_ParsingStatus *)\x00",
            ))
            .as_ptr(),
        );
    }
    *status = (*parser).m_parsingStatus;
}
/* If XML_Parse or XML_ParseBuffer have returned XML_STATUS_ERROR, then
   XML_GetErrorCode returns information about the error.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_GetErrorCode(mut parser: XML_Parser) -> XML_Error {
    if parser.is_null() {
        return XML_ERROR_INVALID_ARGUMENT;
    }
    return (*parser).m_errorCode;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteIndex(mut parser: XML_Parser) -> XML_Index {
    if parser.is_null() {
        return -1i64;
    }
    if !(*parser).m_eventPtr.is_null() {
        return (*parser).m_parseEndByteIndex
            - (*parser)
                .m_parseEndPtr
                .wrapping_offset_from((*parser).m_eventPtr) as c_long;
    }
    if cfg!(feature = "mozilla") {
        return (*parser).m_parseEndByteIndex;
    }
    -1i64
}
/* Return the number of bytes in the current event.
   Returns 0 if the event is in an internal entity.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteCount(mut parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return 0i32;
    }
    if !(*parser).m_eventEndPtr.is_null() && !(*parser).m_eventPtr.is_null() {
        return (*parser)
            .m_eventEndPtr
            .wrapping_offset_from((*parser).m_eventPtr) as c_int;
    }
    0
}
/* If XML_CONTEXT_BYTES is defined, returns the input buffer, sets
   the integer pointed to by offset to the offset within this buffer
   of the current parse position, and sets the integer pointed to by size
   to the size of this buffer (the number of input bytes). Otherwise
   returns a NULL pointer. Also returns a NULL pointer if a parse isn't
   active.

   NOTE: The character pointer returned should not be used outside
   the handler that makes the call.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_GetInputContext(
    mut parser: XML_Parser,
    mut offset: *mut c_int,
    mut size: *mut c_int,
) -> *const c_char {
    if parser.is_null() {
        return NULL as *const c_char;
    }
    if !(*parser).m_eventPtr.is_null() && !(*parser).m_buffer.is_null() {
        if !offset.is_null() {
            *offset = (*parser)
                .m_eventPtr
                .wrapping_offset_from((*parser).m_buffer) as c_int
        }
        if !size.is_null() {
            *size = (*parser)
                .m_bufferEnd
                .wrapping_offset_from((*parser).m_buffer) as c_int
        }
        return (*parser).m_buffer;
    }
    /* defined XML_CONTEXT_BYTES */
    ptr::null()
}
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
pub unsafe extern "C" fn XML_GetCurrentLineNumber(mut parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0u64;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        (*(*parser).m_encoding).updatePosition(
            (*parser).m_positionPtr,
            (*parser).m_eventPtr,
            &mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr
    }
    return (*parser).m_position.lineNumber.wrapping_add(1u64);
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentColumnNumber(mut parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0u64;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        (*(*parser).m_encoding).updatePosition(
            (*parser).m_positionPtr,
            (*parser).m_eventPtr,
            &mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr
    }
    return (*parser).m_position.columnNumber;
}
/* For backwards compatibility with previous versions. */
/* Frees the content model passed to the element declaration handler */
#[no_mangle]
pub unsafe extern "C" fn XML_FreeContentModel(mut parser: XML_Parser, mut model: *mut XML_Content) {
    if !parser.is_null() {
        FREE!(model);
    };
}
/* Exposing the memory handling functions used in Expat */
#[no_mangle]
pub unsafe extern "C" fn XML_MemMalloc(mut parser: XML_Parser, mut size: size_t) -> *mut c_void {
    if parser.is_null() {
        return NULL as *mut c_void;
    }
    return MALLOC!(size);
}
#[no_mangle]
pub unsafe extern "C" fn XML_MemRealloc(
    mut parser: XML_Parser,
    mut ptr: *mut c_void,
    mut size: size_t,
) -> *mut c_void {
    if parser.is_null() {
        return NULL as *mut c_void;
    }
    return REALLOC!(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn XML_MemFree(mut parser: XML_Parser, mut ptr: *mut c_void) {
    if !parser.is_null() {
        FREE!(ptr);
    };
}
/* This can be called within a handler for a start element, end
   element, processing instruction or character data.  It causes the
   corresponding markup to be passed to the default handler.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_DefaultCurrent(mut parser: XML_Parser) {
    if parser.is_null() {
        return;
    }
    if (*parser).m_handlers.hasDefault() {
        if !(*parser).m_openInternalEntities.is_null() {
            reportDefault(
                parser,
                EncodingType::Internal,
                (*(*parser).m_openInternalEntities).internalEventPtr,
                (*(*parser).m_openInternalEntities).internalEventEndPtr,
            );
        } else {
            reportDefault(
                parser,
                EncodingType::Normal,
                (*parser).m_eventPtr,
                (*parser).m_eventEndPtr,
            );
        }
    };
}
/* Returns a string describing the error. */
#[no_mangle]
pub unsafe extern "C" fn XML_ErrorString(mut code: XML_Error) -> *const XML_LChar {
    match code {
        XML_ERROR_NONE => NULL as *const XML_LChar,
        XML_ERROR_NO_MEMORY => wch!("out of memory\x00"),
        XML_ERROR_SYNTAX => wch!("syntax error\x00"),
        XML_ERROR_NO_ELEMENTS => wch!("no element found\x00"),
        XML_ERROR_INVALID_TOKEN => wch!("not well-formed (invalid token)\x00"),
        XML_ERROR_UNCLOSED_TOKEN => wch!("unclosed token\x00"),
        XML_ERROR_PARTIAL_CHAR => wch!("partial character\x00"),
        XML_ERROR_TAG_MISMATCH => wch!("mismatched tag\x00"),
        XML_ERROR_DUPLICATE_ATTRIBUTE => wch!("duplicate attribute\x00"),
        XML_ERROR_JUNK_AFTER_DOC_ELEMENT => wch!("junk after document element\x00"),
        XML_ERROR_PARAM_ENTITY_REF => wch!("illegal parameter entity reference\x00"),
        XML_ERROR_UNDEFINED_ENTITY => wch!("undefined entity\x00"),
        XML_ERROR_RECURSIVE_ENTITY_REF => wch!("recursive entity reference\x00"),
        XML_ERROR_ASYNC_ENTITY => wch!("asynchronous entity\x00"),
        XML_ERROR_BAD_CHAR_REF => wch!("reference to invalid character number\x00"),
        XML_ERROR_BINARY_ENTITY_REF => wch!("reference to binary entity\x00"),
        XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF => {
            wch!("reference to external entity in attribute\x00")
        }
        XML_ERROR_MISPLACED_XML_PI => wch!("XML or text declaration not at start of entity\x00"),
        XML_ERROR_UNKNOWN_ENCODING => wch!("unknown encoding\x00"),
        XML_ERROR_INCORRECT_ENCODING => {
            wch!("encoding specified in XML declaration is incorrect\x00")
        }
        XML_ERROR_UNCLOSED_CDATA_SECTION => wch!("unclosed CDATA section\x00"),
        XML_ERROR_EXTERNAL_ENTITY_HANDLING => {
            wch!("error in processing external entity reference\x00")
        }
        XML_ERROR_NOT_STANDALONE => wch!("document is not standalone\x00"),
        XML_ERROR_UNEXPECTED_STATE => {
            wch!("unexpected parser state - please send a bug report\x00")
        }
        XML_ERROR_ENTITY_DECLARED_IN_PE => wch!("entity declared in parameter entity\x00"),
        XML_ERROR_FEATURE_REQUIRES_XML_DTD => {
            wch!("requested feature requires XML_DTD support in Expat\x00")
        }
        XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING => {
            wch!("cannot change setting once parsing has begun\x00")
        }
        /* Added in 1.95.7. */
        XML_ERROR_UNBOUND_PREFIX => wch!("unbound prefix\x00"),
        /* Added in 1.95.8. */
        XML_ERROR_UNDECLARING_PREFIX => wch!("must not undeclare prefix\x00"),
        XML_ERROR_INCOMPLETE_PE => wch!("incomplete markup in parameter entity\x00"),
        XML_ERROR_XML_DECL => wch!("XML declaration not well-formed\x00"),
        XML_ERROR_TEXT_DECL => wch!("text declaration not well-formed\x00"),
        XML_ERROR_PUBLICID => wch!("illegal character(s) in public id\x00"),
        XML_ERROR_SUSPENDED => wch!("parser suspended\x00"),
        XML_ERROR_NOT_SUSPENDED => wch!("parser not suspended\x00"),
        XML_ERROR_ABORTED => wch!("parsing aborted\x00"),
        XML_ERROR_FINISHED => wch!("parsing finished\x00"),
        XML_ERROR_SUSPEND_PE => wch!("cannot suspend in external parameter entity\x00"),
        XML_ERROR_RESERVED_PREFIX_XML => {
            /* Added in 2.0.0. */
            wch!("reserved prefix (xml) must not be undeclared or bound to another namespace name\x00")
        }
        XML_ERROR_RESERVED_PREFIX_XMLNS => {
            wch!("reserved prefix (xmlns) must not be declared or undeclared\x00")
        }
        XML_ERROR_RESERVED_NAMESPACE_URI => {
            wch!("prefix must not be bound to one of the reserved namespace names\x00")
        }
        /* Added in 2.2.5. */
        XML_ERROR_INVALID_ARGUMENT => {
            /* Constant added in 2.2.1, already */
            wch!("invalid argument\x00")
        }
        _ => NULL as *const XML_LChar,
    }
}
/* Return a string containing the version number of this expat */
#[no_mangle]
pub unsafe extern "C" fn XML_ExpatVersion() -> *const XML_LChar {
    /* V1 is used to string-ize the version number. However, it would
    string-ize the actual version macro *names* unless we get them
    substituted before being passed to V1. CPP is defined to expand
    a macro, then rescan for more expansions. Thus, we use V2 to expand
    the version macros, then CPP will expand the resulting V1() macro
    with the correct numerals. */
    /* ### I'm assuming cpp is portable in this respect... */
    wch!("expat_2.2.9\x00")
}
/* Return an XML_Expat_Version structure containing numeric version
   number information for this version of expat. Expat follows the
   semantic versioning convention. See http://semver.org.
*/
pub const XML_MAJOR_VERSION: c_int = 2;
pub const XML_MINOR_VERSION: c_int = 2;
pub const XML_MICRO_VERSION: c_int = 9;
#[no_mangle]
pub unsafe extern "C" fn XML_ExpatVersionInfo() -> XML_Expat_Version {
    XML_Expat_Version {
        major: XML_MAJOR_VERSION,
        minor: XML_MINOR_VERSION,
        micro: XML_MICRO_VERSION,
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetFeatureList() -> *const XML_Feature {
    const features: &[XML_Feature] = &[
        XML_Feature {
            feature: XML_FEATURE_SIZEOF_XML_CHAR,
            name: wch!("sizeof(XML_Char)\x00"),
            value: ::std::mem::size_of::<XML_Char>() as c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_SIZEOF_XML_LCHAR,
            name: wch!("sizeof(XML_LChar)\x00"),
            value: ::std::mem::size_of::<XML_LChar>() as c_long,
        },
        #[cfg(feature = "unicode")]
        XML_Feature {
            feature: XML_FEATURE_UNICODE,
            name: wch!("XML_UNICODE\x00"),
            value: 0i64,
        },
        #[cfg(feature = "unicode_wchar_t")]
        XML_Feature {
            feature: XML_FEATURE_UNICODE_WCHAR_T,
            name: wch!("XML_UNICODE_WHCAR_T\x00"),
            value: 0i64,
        },
        XML_Feature {
            feature: XML_FEATURE_DTD,
            name: wch!("XML_DTD\x00"),
            value: 0i64,
        },
        XML_Feature {
            feature: XML_FEATURE_CONTEXT_BYTES,
            name: wch!("XML_CONTEXT_BYTES\x00"),
            value: XML_CONTEXT_BYTES as c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_NS,
            name: wch!("XML_NS\x00"),
            value: 0i64,
        },
        XML_Feature {
            feature: XML_FEATURE_END,
            name: NULL as *const XML_LChar,
            value: 0i64,
        },
    ];
    features.as_ptr()
}

#[cfg(feature = "mozilla")]
#[no_mangle]
pub unsafe extern "C" fn MOZ_XML_GetMismatchedTag(parser: XML_Parser) -> *const XML_Char {
    (*parser).m_mismatch
}

#[cfg(feature = "mozilla")]
#[no_mangle]
pub unsafe extern "C" fn MOZ_XML_ProcessingEntityValue(parser: XML_Parser) -> XML_Bool {
    !(*parser).m_openInternalEntities.is_null() as XML_Bool
}

/* Initially tag->rawName always points into the parse buffer;
   for those TAG instances opened while the current parse buffer was
   processed, and not yet closed, we need to store tag->rawName in a more
   permanent location, since the parse buffer is about to be discarded.
*/
impl XML_ParserStruct {
    unsafe fn storeRawNames(&mut self) -> XML_Bool {
        let mut tag: *mut TAG = self.m_tagStack;
        while !tag.is_null() {
            let mut bufSize: c_int = 0;
            let mut nameLen: c_int = (::std::mem::size_of::<XML_Char>() as c_ulong)
                .wrapping_mul(((*tag).name.strLen + 1) as c_ulong)
                as c_int;
            let mut rawNameBuf: *mut c_char = (*tag).buf.offset(nameLen as isize);
            /* Stop if already stored.  Since m_tagStack is a stack, we can stop
            at the first entry that has already been copied; everything
            below it in the stack is already been accounted for in a
            previous call to this function.
            */
            if (*tag).rawName == rawNameBuf as *const c_char {
                break;
            }
            /* For re-use purposes we need to ensure that the
            size of tag->buf is a multiple of sizeof(XML_Char).
            */
            bufSize = (nameLen as c_ulong).wrapping_add(
                ((*tag).rawNameLength as c_ulong)
                    .wrapping_add((::std::mem::size_of::<XML_Char>() as c_ulong).wrapping_sub(1u64))
                    & !(::std::mem::size_of::<XML_Char>() as c_ulong).wrapping_sub(1u64),
            ) as c_int;
            if bufSize as c_long > (*tag).bufEnd.wrapping_offset_from((*tag).buf) as c_long {
                let mut temp: *mut c_char =
                    REALLOC!((*tag).buf as *mut c_void, bufSize as size_t) as *mut c_char;
                if temp.is_null() {
                    return XML_FALSE;
                }
                /* if tag->name.str points to tag->buf (only when namespace
                processing is off) then we have to update it
                */
                if (*tag).name.str_0 == (*tag).buf as *const XML_Char {
                    (*tag).name.str_0 = temp as *const XML_Char
                }
                /* if tag->name.localPart is set (when namespace processing is on)
                then update it as well, since it will always point into tag->buf
                */
                if !(*tag).name.localPart.is_null() {
                    (*tag).name.localPart = (temp).offset(
                        (*tag)
                            .name
                            .localPart
                            .wrapping_offset_from((*tag).buf as *const XML_Char),
                    ) as *const XML_Char
                } /* XmlContentTok doesn't always set the last arg */
                (*tag).buf = temp;
                (*tag).bufEnd = temp.offset(bufSize as isize);
                rawNameBuf = temp.offset(nameLen as isize)
            }
            memcpy(
                rawNameBuf as *mut c_void,
                (*tag).rawName as *const c_void,
                (*tag).rawNameLength as c_ulong,
            );
            (*tag).rawName = rawNameBuf;
            tag = (*tag).parent
        }
        XML_TRUE
    }
}

unsafe extern "C" fn contentProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).doContent(
        0,
        EncodingType::Normal,
        start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
    );
    if result == XML_ERROR_NONE {
        if (*parser).storeRawNames() == 0 {
            return XML_ERROR_NO_MEMORY;
        }
    }
    return result;
}

unsafe extern "C" fn externalEntityInitProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).initializeEncoding();
    if result != XML_ERROR_NONE {
        return result;
    }
    (*parser).m_processor = Some(externalEntityInitProcessor2 as Processor);
    externalEntityInitProcessor2(parser, start, end, endPtr)
}

unsafe extern "C" fn externalEntityInitProcessor2(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = start;
    let mut tok: c_int = (*(*parser).m_encoding).xmlTok(XML_CONTENT_STATE, start, end, &mut next);
    match tok {
        super::xmltok::XML_TOK_BOM => {
            /* If we are at the end of the buffer, this would cause the next stage,
               i.e. externalEntityInitProcessor3, to pass control directly to
               doContent (by detecting XML_TOK_NONE) without processing any xml text
               declaration - causing the error XML_ERROR_MISPLACED_XML_PI in doContent.
            */
            if next == end && (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = next; /* XmlContentTok doesn't always set the last arg */
                return XML_ERROR_NONE;
            }
            start = next
        }
        super::xmltok::XML_TOK_PARTIAL => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            (*parser).m_eventPtr = start;
            return XML_ERROR_UNCLOSED_TOKEN;
        }
        super::xmltok::XML_TOK_PARTIAL_CHAR => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            (*parser).m_eventPtr = start;
            return XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(externalEntityInitProcessor3 as Processor);
    return externalEntityInitProcessor3(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityInitProcessor3(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut tok: c_int = 0;
    let mut next: *const c_char = start;
    (*parser).m_eventPtr = start;
    tok = (*(*parser).m_encoding).xmlTok(XML_CONTENT_STATE, start, end, &mut next);
    (*parser).m_eventEndPtr = next;
    match tok {
        super::xmltok::XML_TOK_XML_DECL => {
            let mut result: XML_Error = XML_ERROR_NONE;
            result = (*parser).processXmlDecl(1, start, next);
            if result != XML_ERROR_NONE {
                return result;
            }
            match (*parser).m_parsingStatus.parsing {
                3 => {
                    *endPtr = next;
                    return XML_ERROR_NONE;
                }
                2 => return XML_ERROR_ABORTED,
                _ => start = next,
            }
        }
        super::xmltok::XML_TOK_PARTIAL => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_UNCLOSED_TOKEN;
        }
        super::xmltok::XML_TOK_PARTIAL_CHAR => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(externalEntityContentProcessor as Processor);
    (*parser).m_tagLevel = 1;
    return externalEntityContentProcessor(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityContentProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).doContent(
        1,
        EncodingType::Normal,
        start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
    );
    if result == XML_ERROR_NONE {
        if (*parser).storeRawNames() == 0 {
            return XML_ERROR_NO_MEMORY;
        }
    }
    return result;
}

impl XML_ParserStruct {
    unsafe fn doContent(
        &mut self,
        startTagLevel: c_int,
        enc_type: EncodingType,
        mut s: *const c_char,
        end: *const c_char,
        nextPtr: *mut *const c_char,
        haveMore: XML_Bool,
    ) -> XML_Error {
        /* save one level of indirection */
        let dtd: *mut DTD = self.m_dtd; /* XmlContentTok doesn't always set the last arg */
        let mut eventPP: *mut *const c_char = 0 as *mut *const c_char;
        let mut eventEndPP: *mut *const c_char = 0 as *mut *const c_char;
        if enc_type.is_internal() {
            eventPP = &mut (*self.m_openInternalEntities).internalEventPtr;
            eventEndPP = &mut (*self.m_openInternalEntities).internalEventEndPtr
        } else {
            eventPP = &mut self.m_eventPtr;
            eventEndPP = &mut self.m_eventEndPtr
        }
        let enc = self.encoding(enc_type);
        *eventPP = s;
        loop {
            let mut next: *const c_char = s;
            let mut tok: c_int = (*enc).xmlTok(XML_CONTENT_STATE, s, end, &mut next);
            *eventEndPP = next;
            let mut current_block_275: u64;
            match tok {
                super::xmltok::XML_TOK_TRAILING_CR => {
                    if haveMore != 0 {
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    *eventEndPP = end;
                    if self.m_handlers.hasCharacterData() {
                        let mut c: XML_Char = 0xa;
                        self.m_handlers.characterData(&mut c, 1);
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, s, end);
                    }
                    /* LCOV_EXCL_STOP */
                    /* We are at the end of the final buffer, should we check for
                       XML_SUSPENDED, XML_FINISHED?
                    */
                    if startTagLevel == 0 {
                        return XML_ERROR_NO_ELEMENTS;
                    }
                    if self.m_tagLevel != startTagLevel {
                        return XML_ERROR_ASYNC_ENTITY;
                    }
                    *nextPtr = end;
                    return XML_ERROR_NONE;
                }
                super::xmltok::XML_TOK_NONE => {
                    if haveMore != 0 {
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    if startTagLevel > 0 {
                        if self.m_tagLevel != startTagLevel {
                            return XML_ERROR_ASYNC_ENTITY;
                        }
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    return XML_ERROR_NO_ELEMENTS;
                }
                super::xmltok::XML_TOK_INVALID => {
                    *eventPP = next;
                    return XML_ERROR_INVALID_TOKEN;
                }
                super::xmltok::XML_TOK_PARTIAL => {
                    if haveMore != 0 {
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    return XML_ERROR_UNCLOSED_TOKEN;
                }
                super::xmltok::XML_TOK_PARTIAL_CHAR => {
                    if haveMore != 0 {
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    return XML_ERROR_PARTIAL_CHAR;
                }
                super::xmltok::XML_TOK_ENTITY_REF => {
                    let mut name: *const XML_Char = 0 as *const XML_Char;
                    let mut ch: XML_Char = (*enc).predefinedEntityName(
                        s.offset((*enc).minBytesPerChar() as isize),
                        next.offset(-((*enc).minBytesPerChar() as isize)),
                    ) as XML_Char;
                    if ch != 0 {
                        let hasCharacterData = self.m_handlers.characterData(
                            &mut ch,
                            1i32,
                        );

                        if !hasCharacterData && self.m_handlers.hasDefault() {
                            reportDefault(self, enc_type, s, next);
                        }
                    } else {
                        name = (*dtd).pool.storeString(
                            enc,
                            s.offset((*enc).minBytesPerChar() as isize),
                            next.offset(-((*enc).minBytesPerChar() as isize)),
                        );
                        if name.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        let entity = hash_lookup!((*dtd).generalEntities, name);
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        /* First, determine if a check for an existing declaration is needed;
                           if yes, check that the entity exists, and that it is internal,
                           otherwise call the skipped entity or default handler.
                        */
                        if (*dtd).hasParamEntityRefs == 0 || (*dtd).standalone as c_int != 0 {
                            if entity.is_null() {
                                return XML_ERROR_UNDEFINED_ENTITY;
                            } else {
                                if (*entity).is_internal == 0 {
                                    return XML_ERROR_ENTITY_DECLARED_IN_PE;
                                }
                            }
                            current_block_275 = 10067844863897285902;
                        } else if entity.is_null() {
                            let skippedHandlerRan = self.m_handlers.skippedEntity(name, 0);

                            if !skippedHandlerRan && self.m_handlers.hasDefault() {
                                if !cfg!(feature = "mozilla") {
                                    reportDefault(self, enc_type, s, next);
                                }
                            }
                            if cfg!(feature = "mozilla") {
                                return XML_ERROR_UNDEFINED_ENTITY;
                            }
                            current_block_275 = 17939951368883298147;
                        } else {
                            current_block_275 = 10067844863897285902;
                        }
                        match current_block_275 {
                            17939951368883298147 => {}
                            _ => {
                                if (*entity).open != 0 {
                                    return XML_ERROR_RECURSIVE_ENTITY_REF;
                                }
                                if !(*entity).notation.is_null() {
                                    return XML_ERROR_BINARY_ENTITY_REF;
                                }
                                if !(*entity).textPtr.is_null() {
                                    let mut result: XML_Error = XML_ERROR_NONE;
                                    if self.m_defaultExpandInternalEntities == 0 {
                                        let skippedHandlerRan = self.m_handlers.skippedEntity((*entity).name, 0);

                                        if !skippedHandlerRan && self.m_handlers.hasDefault() {
                                            reportDefault(self, enc_type, s, next);
                                        }
                                    } else {
                                        result = self.processInternalEntity(entity, XML_FALSE);
                                        if result != XML_ERROR_NONE {
                                            return result;
                                        }
                                    }
                                } else if self.m_handlers.hasExternalEntityRef() {
                                    let mut context: *const XML_Char = 0 as *const XML_Char;
                                    (*entity).open = XML_TRUE;
                                    context = self.getContext();
                                    (*entity).open = XML_FALSE;
                                    if context.is_null() {
                                        return XML_ERROR_NO_MEMORY;
                                    }
                                    if self.m_handlers.externalEntityRef(
                                        context,
                                        (*entity).base,
                                        (*entity).systemId,
                                        (*entity).publicId,
                                    ) == Ok(0)
                                    {
                                        return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                    }
                                    self.m_tempPool.ptr = self.m_tempPool.start
                                } else if self.m_handlers.hasDefault() {
                                    reportDefault(self, enc_type, s, next);
                                }
                            }
                        }
                    }
                }
                super::xmltok::XML_TOK_START_TAG_NO_ATTS
                | super::xmltok::XML_TOK_START_TAG_WITH_ATTS => {
                    /* fall through */
                    let mut tag: *mut TAG = 0 as *mut TAG;
                    let mut result_0: XML_Error = XML_ERROR_NONE;
                    let mut toPtr: *mut XML_Char = 0 as *mut XML_Char;
                    if !self.m_freeTagList.is_null() {
                        tag = self.m_freeTagList;
                        self.m_freeTagList = (*self.m_freeTagList).parent
                    } else {
                        tag = MALLOC!(::std::mem::size_of::<TAG>() as c_ulong) as *mut TAG;
                        if tag.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*tag).buf = MALLOC!(32u64) as *mut c_char;
                        if (*tag).buf.is_null() {
                            FREE!(tag);
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*tag).bufEnd = (*tag).buf.offset(INIT_TAG_BUF_SIZE as isize)
                    }
                    (*tag).bindings = NULL as *mut BINDING;
                    (*tag).parent = self.m_tagStack;
                    self.m_tagStack = tag;
                    (*tag).name.localPart = NULL as *const XML_Char;
                    (*tag).name.prefix = NULL as *const XML_Char;
                    (*tag).rawName = s.offset((*enc).minBytesPerChar() as isize);
                    (*tag).rawNameLength = (*enc).nameLength((*tag).rawName);
                    self.m_tagLevel += 1;
                    let mut rawNameEnd: *const c_char =
                        (*tag).rawName.offset((*tag).rawNameLength as isize);
                    let mut fromPtr: *const c_char = (*tag).rawName;
                    toPtr = (*tag).buf as *mut XML_Char;
                    loop {
                        let mut bufSize: c_int = 0;
                        let mut convLen: c_int = 0;
                        let convert_res: super::xmltok::XML_Convert_Result = XmlConvert!(
                            enc,
                            &mut fromPtr,
                            rawNameEnd,
                            &mut toPtr as *mut *mut _ as *mut *mut ICHAR,
                            ((*tag).bufEnd as *const ICHAR).offset(-1),
                        );
                        convLen = toPtr.wrapping_offset_from((*tag).buf as *const XML_Char) as c_int;
                        if fromPtr >= rawNameEnd
                            || convert_res == super::xmltok::XML_CONVERT_INPUT_INCOMPLETE
                        {
                            (*tag).name.strLen = convLen;
                            break;
                        } else {
                            bufSize = ((*tag).bufEnd.wrapping_offset_from((*tag).buf) as c_int) << 1;
                            let mut temp: *mut c_char =
                                REALLOC!((*tag).buf as *mut c_void, bufSize as size_t)
                                    as *mut c_char;
                            if temp.is_null() {
                                return XML_ERROR_NO_MEMORY;
                            }
                            (*tag).buf = temp;
                            (*tag).bufEnd = temp.offset(bufSize as isize);
                            toPtr = (temp).offset(convLen as isize) as *mut XML_Char
                        }
                    }
                    (*tag).name.str_0 = (*tag).buf as *const XML_Char;
                    *toPtr = '\u{0}' as XML_Char;
                    result_0 = self.storeAtts(enc_type, s, &mut (*tag).name, &mut (*tag).bindings);
                    if result_0 as u64 != 0 {
                        return result_0;
                    }

                    let handlers = self.m_handlers;
                    let started = handlers.startElement((*tag).name.str_0, self.m_atts);

                    if !started && handlers.hasDefault() {
                        reportDefault(self, enc_type, s, next);
                    }
                    self.m_tempPool.clear();
                }
                super::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS
                | super::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS => {
                    /* fall through */
                    let mut rawName: *const c_char = s.offset((*enc).minBytesPerChar() as isize);
                    let mut result_1: XML_Error = XML_ERROR_NONE;
                    let mut bindings: *mut BINDING = NULL as *mut BINDING;
                    let mut noElmHandlers: XML_Bool = XML_TRUE;
                    let mut name_0: TAG_NAME = TAG_NAME {
                        str_0: 0 as *const XML_Char,
                        localPart: 0 as *const XML_Char,
                        prefix: 0 as *const XML_Char,
                        strLen: 0,
                        uriLen: 0,
                        prefixLen: 0,
                    };
                    name_0.str_0 = self.m_tempPool.storeString(
                        enc,
                        rawName,
                        rawName.offset((*enc).nameLength(rawName) as isize),
                    );
                    if name_0.str_0.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    self.m_tempPool.start = self.m_tempPool.ptr;
                    result_1 = self.storeAtts(enc_type, s, &mut name_0, &mut bindings);
                    if result_1 != XML_ERROR_NONE {
                        self.freeBindings(bindings);
                        return result_1;
                    }
                    self.m_tempPool.start = self.m_tempPool.ptr;
                    let handlers = self.m_handlers;
                    let started = handlers.startElement(name_0.str_0, self.m_atts);
                    if started {
                        noElmHandlers = XML_FALSE
                    }
                    if self.m_handlers.hasEndElement() {
                        if started {
                            *eventPP = *eventEndPP
                        }

                        self.m_handlers.endElement(name_0.str_0);

                        noElmHandlers = XML_FALSE
                    }
                    if noElmHandlers as c_int != 0 && self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, s, next);
                    }
                    self.m_tempPool.clear();
                    self.freeBindings(bindings);
                    if self.m_tagLevel == 0 && self.m_parsingStatus.parsing != XML_FINISHED {
                        if self.m_parsingStatus.parsing == XML_SUSPENDED {
                            self.m_processor = Some(epilogProcessor as Processor)
                        } else {
                            return epilogProcessor(self, next, end, nextPtr);
                        }
                    }
                }
                super::xmltok::XML_TOK_END_TAG => {
                    if self.m_tagLevel == startTagLevel {
                        return XML_ERROR_ASYNC_ENTITY;
                    } else {
                        let mut len: c_int = 0;
                        let mut rawName_0: *const c_char = 0 as *const c_char;
                        let mut tag_0: *mut TAG = self.m_tagStack;
                        self.m_tagStack = (*tag_0).parent;
                        (*tag_0).parent = self.m_freeTagList;
                        self.m_freeTagList = tag_0;
                        rawName_0 = s.offset(((*enc).minBytesPerChar() * 2i32) as isize);
                        len = (*enc).nameLength(rawName_0);
                        if len != (*tag_0).rawNameLength
                            || memcmp(
                                (*tag_0).rawName as *const c_void,
                                rawName_0 as *const c_void,
                                len as c_ulong,
                            ) != 0
                        {
                            #[cfg(feature = "mozilla")]
                            {
                                /* This code is copied from the |if (endElementHandler)| block below */
                                let mut localPart: *const XML_Char = 0 as *const XML_Char;
                                let mut prefix: *const XML_Char = 0 as *const XML_Char;
                                let mut uri: *mut XML_Char = 0 as *mut XML_Char;
                                localPart = (*tag_0).name.localPart;
                                if self.m_ns as c_int != 0 && !localPart.is_null() {
                                    /* localPart and prefix may have been overwritten in
                                       tag->name.str, since this points to the binding->uri
                                       buffer which gets re-used; so we have to add them again
                                    */
                                    uri = ((*tag_0).name.str_0 as *mut XML_Char)
                                        .offset((*tag_0).name.uriLen as isize);
                                    /* don't need to check for space - already done in storeAtts() */
                                    while *localPart != 0 {
                                        let fresh2 = localPart;
                                        localPart = localPart.offset(1);
                                        let fresh3 = uri;
                                        uri = uri.offset(1);
                                        *fresh3 = *fresh2
                                    }
                                    prefix = (*tag_0).name.prefix as *mut XML_Char;
                                    if self.m_ns_triplets as c_int != 0 && !prefix.is_null() {
                                        let fresh4 = uri;
                                        uri = uri.offset(1);
                                        *fresh4 = self.m_namespaceSeparator;
                                        while *prefix != 0 {
                                            let fresh5 = prefix;
                                            prefix = prefix.offset(1);
                                            let fresh6 = uri;
                                            uri = uri.offset(1);
                                            *fresh6 = *fresh5
                                        }
                                    }
                                    *uri = '\u{0}' as XML_Char
                                }
                                self.m_mismatch = (*tag_0).name.str_0;
                            }
                            *eventPP = rawName_0;
                            return XML_ERROR_TAG_MISMATCH;
                        }
                        self.m_tagLevel -= 1;
                        if self.m_handlers.hasEndElement() {
                            let mut localPart: *const XML_Char = 0 as *const XML_Char;
                            let mut prefix: *const XML_Char = 0 as *const XML_Char;
                            let mut uri: *mut XML_Char = 0 as *mut XML_Char;
                            localPart = (*tag_0).name.localPart;
                            if self.m_ns as c_int != 0 && !localPart.is_null() {
                                /* localPart and prefix may have been overwritten in
                                   tag->name.str, since this points to the binding->uri
                                   buffer which gets re-used; so we have to add them again
                                */
                                uri = ((*tag_0).name.str_0 as *mut XML_Char)
                                    .offset((*tag_0).name.uriLen as isize);
                                /* don't need to check for space - already done in storeAtts() */
                                while *localPart != 0 {
                                    let fresh2 = localPart;
                                    localPart = localPart.offset(1);
                                    let fresh3 = uri;
                                    uri = uri.offset(1);
                                    *fresh3 = *fresh2
                                }
                                prefix = (*tag_0).name.prefix as *mut XML_Char;
                                if self.m_ns_triplets as c_int != 0 && !prefix.is_null() {
                                    let fresh4 = uri;
                                    uri = uri.offset(1);
                                    *fresh4 = self.m_namespaceSeparator;
                                    while *prefix != 0 {
                                        let fresh5 = prefix;
                                        prefix = prefix.offset(1);
                                        let fresh6 = uri;
                                        uri = uri.offset(1);
                                        *fresh6 = *fresh5
                                    }
                                }
                                *uri = '\u{0}' as XML_Char
                            }

                            self.m_handlers.endElement((*tag_0).name.str_0);
                        } else if self.m_handlers.hasDefault() {
                            reportDefault(self, enc_type, s, next);
                        }
                        while !(*tag_0).bindings.is_null() {
                            let mut b: *mut BINDING = (*tag_0).bindings;
                            self.m_handlers.endNamespaceDecl((*(*b).prefix).name);
                            (*tag_0).bindings = (*(*tag_0).bindings).nextTagBinding;
                            (*b).nextTagBinding = self.m_freeBindingList;
                            self.m_freeBindingList = b;
                            (*(*b).prefix).binding = (*b).prevPrefixBinding
                        }
                        if self.m_tagLevel == 0
                            && self.m_parsingStatus.parsing != XML_FINISHED
                        {
                            if self.m_parsingStatus.parsing == XML_SUSPENDED {
                                self.m_processor = Some(epilogProcessor as Processor)
                            } else {
                                return epilogProcessor(self, next, end, nextPtr);
                            }
                        }
                    }
                }
                super::xmltok::XML_TOK_CHAR_REF => {
                    let mut n: c_int = (*enc).charRefNumber(s);
                    if n < 0 {
                        return XML_ERROR_BAD_CHAR_REF;
                    }
                    if self.m_handlers.hasCharacterData() {
                        let mut buf: [XML_Char; XML_ENCODE_MAX] = [0; XML_ENCODE_MAX];
                        self.m_handlers.characterData(
                            buf.as_mut_ptr(),
                            XmlEncode(n, buf.as_mut_ptr() as *mut ICHAR),
                        );
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, s, next);
                    }
                }
                super::xmltok::XML_TOK_XML_DECL => return XML_ERROR_MISPLACED_XML_PI,
                super::xmltok::XML_TOK_DATA_NEWLINE => {
                    if self.m_handlers.hasCharacterData() {
                        let mut c_0: XML_Char = 0xa;
                        self.m_handlers.characterData(
                            &mut c_0,
                            1,
                        );
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, s, next);
                    }
                }
                super::xmltok::XML_TOK_CDATA_SECT_OPEN => {
                    let mut result_2: XML_Error = XML_ERROR_NONE;

                    let startHandlerRan = self.m_handlers.startCDataSection();

                    if startHandlerRan {
                    } else if 0 != 0 && self.m_handlers.hasCharacterData() {
                        self.m_handlers.characterData(self.m_dataBuf, 0);
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, s, next);
                    }
                    result_2 = doCdataSection(self, enc_type, &mut next, end, nextPtr, haveMore);
                    if result_2 != XML_ERROR_NONE {
                        return result_2;
                    } else if next.is_null() {
                        self.m_processor = Some(cdataSectionProcessor as Processor);
                        return result_2;
                    }
                }
                super::xmltok::XML_TOK_TRAILING_RSQB => {
                    if haveMore != 0 {
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    if self.m_handlers.hasCharacterData() {
                        if MUST_CONVERT!(enc, s) {
                            let mut dataPtr = self.m_dataBuf as *mut ICHAR;
                            XmlConvert!(enc, &mut s, end, &mut dataPtr,
                                        self.m_dataBufEnd as *mut ICHAR);
                            self.m_handlers.characterData(
                                self.m_dataBuf,
                                dataPtr.wrapping_offset_from(self.m_dataBuf as *mut ICHAR) as c_int,
                            );
                        } else {
                            self.m_handlers.characterData(
                                s as *mut XML_Char,
                                (end as *mut XML_Char).wrapping_offset_from(s as *mut XML_Char)
                                    as c_int,
                            );
                        }
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, s, end);
                    }
                    /* BEGIN disabled code */
                    /* Suppose you doing a transformation on a document that involves
                       changing only the character data.  You set up a defaultHandler
                       and a characterDataHandler.  The defaultHandler simply copies
                       characters through.  The characterDataHandler does the
                       transformation and writes the characters out escaping them as
                       necessary.  This case will fail to work if we leave out the
                       following two lines (because & and < inside CDATA sections will
                       be incorrectly escaped).

                       However, now we have a start/endCdataSectionHandler, so it seems
                       easier to let the user deal with this.
                    */
                    /* END disabled code */
                    /* We are at the end of the final buffer, should we check for
                       XML_SUSPENDED, XML_FINISHED?
                    */
                    if startTagLevel == 0 {
                        *eventPP = end;
                        return XML_ERROR_NO_ELEMENTS;
                    }
                    if self.m_tagLevel != startTagLevel {
                        *eventPP = end;
                        return XML_ERROR_ASYNC_ENTITY;
                    }
                    *nextPtr = end;
                    return XML_ERROR_NONE;
                }
                super::xmltok::XML_TOK_DATA_CHARS => {
                    let mut handlers = self.m_handlers;
                    if handlers.hasCharacterData() {
                        if MUST_CONVERT!(enc, s) {
                            loop {
                                let mut dataPtr_0 = self.m_dataBuf as *mut ICHAR;
                                let convert_res_0: super::xmltok::XML_Convert_Result = XmlConvert!(
                                    enc,
                                    &mut s,
                                    next,
                                    &mut dataPtr_0,
                                    self.m_dataBufEnd as *mut ICHAR,
                                );
                                *eventEndPP = s;
                                handlers.characterData(
                                    self.m_dataBuf,
                                    dataPtr_0.wrapping_offset_from(self.m_dataBuf as *mut ICHAR) as c_int,
                                );
                                if convert_res_0 == super::xmltok::XML_CONVERT_COMPLETED
                                    || convert_res_0 == super::xmltok::XML_CONVERT_INPUT_INCOMPLETE
                                {
                                    break;
                                }
                                *eventPP = s
                            }
                        } else {
                            handlers.characterData(
                                s as *mut XML_Char,
                                (next as *mut XML_Char).wrapping_offset_from(s as *mut XML_Char)
                                    as c_int,
                            );
                        }
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, s, next);
                    }
                }
                super::xmltok::XML_TOK_PI => {
                    if reportProcessingInstruction(self, enc_type, s, next) == 0 {
                        return XML_ERROR_NO_MEMORY;
                    }
                }
                super::xmltok::XML_TOK_COMMENT => {
                    if reportComment(self, enc_type, s, next) == 0 {
                        return XML_ERROR_NO_MEMORY;
                    }
                }
                _ => {
                    /* All of the tokens produced by XmlContentTok() have their own
                     * explicit cases, so this default is not strictly necessary.
                     * However it is a useful safety net, so we retain the code and
                     * simply exclude it from the coverage tests.
                     *
                     * LCOV_EXCL_START
                     */
                    if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, s, next);
                    }
                }
            }
            s = next;
            *eventPP = s;
            match self.m_parsingStatus.parsing {
                3 => {
                    *nextPtr = next;
                    return XML_ERROR_NONE;
                }
                2 => return XML_ERROR_ABORTED,
                _ => {}
            }
        }
        /* not reached */
    }

/* XML_DTD */
/* This function does not call free() on the allocated memory, merely
 * moving it to the parser's m_freeBindingList where it can be freed or
 * reused as appropriate.
 */

    unsafe fn freeBindings(&mut self, mut bindings: *mut BINDING) {
        while !bindings.is_null() {
            let mut b: *mut BINDING = bindings;
            /* m_startNamespaceDeclHandler will have been called for this
            * binding in addBindings(), so call the end handler now.
            */
            self.m_handlers.endNamespaceDecl((*(*b).prefix).name);
            bindings = (*bindings).nextTagBinding;
            (*b).nextTagBinding = self.m_freeBindingList;
            self.m_freeBindingList = b;
            (*(*b).prefix).binding = (*b).prevPrefixBinding
        }
    }
/* Precondition: all arguments must be non-NULL;
   Purpose:
   - normalize attributes
   - check attributes for well-formedness
   - generate namespace aware attribute names (URI, prefix)
   - build list of attributes for startElementHandler
   - default attributes
   - process namespace declarations (check and report them)
   - generate namespace aware element name (URI, prefix)
*/

    unsafe fn storeAtts(
        &mut self,
        enc_type: EncodingType,
        attStr: *const c_char,
        tagNamePtr: *mut TAG_NAME,
        bindingsPtr: *mut *mut BINDING,
    ) -> XML_Error {
        let dtd: *mut DTD = self.m_dtd; /* save one level of indirection */
        let mut nDefaultAtts: c_int = 0;
        let mut appAtts: *mut *const XML_Char = 0 as *mut *const XML_Char;
        let mut attIndex: c_int = 0;
        let mut prefixLen: c_int = 0;
        let mut i: c_int = 0;
        let mut n: c_int = 0;
        let mut uri: *mut XML_Char = 0 as *mut XML_Char;
        let mut nPrefixes: c_int = 0;
        let mut nXMLNSDeclarations: c_int = 0;
        let mut binding: *mut BINDING = 0 as *mut BINDING;
        let mut localPart: *const XML_Char = 0 as *const XML_Char;
        let enc = self.encoding(enc_type);
        /* lookup the element type name */
        let elementType = if let Some(elementType) = (*dtd).elementTypes.get_mut(&HashKey::from((*tagNamePtr).str_0)) {
            elementType.as_mut()
        } else {
            let mut name: *const XML_Char = (*dtd).pool.copyString((*tagNamePtr).str_0);
            if name.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            let elementType = hash_insert!(
                &mut (*dtd).elementTypes,
                name,
                ELEMENT_TYPE
            );
            if elementType.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            if self.m_ns as c_int != 0 && self.setElementTypePrefix(elementType) == 0 {
                return XML_ERROR_NO_MEMORY;
            }
            elementType
        };
        nDefaultAtts = (*elementType).nDefaultAtts;
        /* get the attributes from the tokenizer */
        n = (*enc).getAtts(attStr, self.m_attsSize, self.m_atts);
        if n + nDefaultAtts > self.m_attsSize {
            let mut oldAttsSize: c_int = self.m_attsSize;
            let mut temp: *mut super::xmltok::ATTRIBUTE = 0 as *mut super::xmltok::ATTRIBUTE;
            self.m_attsSize = n + nDefaultAtts + INIT_ATTS_SIZE;
            temp = REALLOC!(
                self.m_atts as *mut c_void,
                (self.m_attsSize as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<super::xmltok::ATTRIBUTE>() as c_ulong)
            ) as *mut super::xmltok::ATTRIBUTE;
            if temp.is_null() {
                self.m_attsSize = oldAttsSize;
                return XML_ERROR_NO_MEMORY;
            }
            self.m_atts = temp;
            if n > oldAttsSize {
                (*enc).getAtts(attStr, n, self.m_atts);
            }
        }
        appAtts = self.m_atts as *mut *const XML_Char;
        i = 0;
        while i < n {
            let mut currAtt: *mut super::xmltok::ATTRIBUTE =
                &mut *self.m_atts.offset(i as isize) as *mut super::xmltok::ATTRIBUTE;
            /* add the name and value to the attribute list */
            let mut attId: *mut ATTRIBUTE_ID = self.getAttributeId(
                enc_type,
                (*currAtt).name,
                (*currAtt)
                    .name
                    .offset((*enc).nameLength((*currAtt).name) as isize),
            );
            if attId.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            /* Detect duplicate attributes by their QNames. This does not work when
            namespace processing is turned on and different prefixes for the same
            namespace are used. For this case we have a check further down.
            */
            if *(*attId).name.offset(-1) != 0 {
                if !enc_type.is_internal() {
                    self.m_eventPtr = (*self.m_atts.offset(i as isize)).name
                }
                return XML_ERROR_DUPLICATE_ATTRIBUTE;
            }
            *(*attId).name.offset(-1) = 1;
            let fresh7 = attIndex;
            attIndex = attIndex + 1;
            let ref mut fresh8 = *appAtts.offset(fresh7 as isize);
            *fresh8 = (*attId).name;
            if (*self.m_atts.offset(i as isize)).normalized == 0 {
                let mut result: XML_Error = XML_ERROR_NONE;
                let mut isCdata: XML_Bool = XML_TRUE;
                /* figure out whether declared as other than CDATA */
                if (*attId).maybeTokenized != 0 {
                    let mut j: c_int = 0;
                    j = 0;
                    while j < nDefaultAtts {
                        if attId
                            == (*(*elementType).defaultAtts.offset(j as isize)).id as *mut ATTRIBUTE_ID
                        {
                            isCdata = (*(*elementType).defaultAtts.offset(j as isize)).isCdata;
                            break;
                        } else {
                            j += 1
                        }
                    }
                }
                /* normalize the attribute value */
                result = storeAttributeValue(
                    self,
                    enc_type,
                    isCdata,
                    (*self.m_atts.offset(i as isize)).valuePtr,
                    (*self.m_atts.offset(i as isize)).valueEnd,
                    &mut self.m_tempPool,
                );
                if result as u64 != 0 {
                    return result;
                }
                let ref mut fresh9 = *appAtts.offset(attIndex as isize);
                *fresh9 = self.m_tempPool.start;
                self.m_tempPool.start = self.m_tempPool.ptr
            } else {
                /* the value did not need normalizing */
                let ref mut fresh10 = *appAtts.offset(attIndex as isize);
                *fresh10 = self.m_tempPool.storeString(
                    enc,
                    (*self.m_atts.offset(i as isize)).valuePtr,
                    (*self.m_atts.offset(i as isize)).valueEnd,
                );
                if (*appAtts.offset(attIndex as isize)).is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                self.m_tempPool.start = self.m_tempPool.ptr
            }
            /* handle prefixed attribute names */
            if !(*attId).prefix.is_null() {
                if (*attId).xmlns != 0 {
                    /* deal with namespace declarations here */
                    let mut result_0: XML_Error = addBinding(
                        self,
                        (*attId).prefix,
                        attId,
                        *appAtts.offset(attIndex as isize),
                        bindingsPtr,
                    );
                    if result_0 as u64 != 0 {
                        return result_0;
                    }
                    attIndex -= 1;
                    #[cfg(feature = "mozilla")]
                    {
                        // Mozilla code replaces `--attIndex` with `attIndex++`,
                        // which is a shift by 2 positions
                        attIndex += 2;
                        nXMLNSDeclarations += 1;
                        *(*attId).name.offset(-1) = 3;
                    }
                } else {
                    /* deal with other prefixed names later */
                    attIndex += 1;
                    nPrefixes += 1;
                    *(*attId).name.offset(-1) = 2
                }
            } else {
                attIndex += 1
            }
            i += 1
        }
        /* set-up for XML_GetSpecifiedAttributeCount and XML_GetIdAttributeIndex */
        self.m_nSpecifiedAtts = attIndex;
        if !(*elementType).idAtt.is_null() && *(*(*elementType).idAtt).name.offset(-1) as c_int != 0 {
            i = 0;
            while i < attIndex {
                if *appAtts.offset(i as isize) == (*(*elementType).idAtt).name as *const XML_Char {
                    self.m_idAttIndex = i;
                    break;
                } else {
                    i += 2
                }
            }
        } else {
            self.m_idAttIndex = -(1)
        }
        /* do attribute defaulting */
        i = 0;
        while i < nDefaultAtts {
            let mut da: *const DEFAULT_ATTRIBUTE = (*elementType).defaultAtts.offset(i as isize);
            if *(*(*da).id).name.offset(-1) == 0 && !(*da).value.is_null() {
                if !(*(*da).id).prefix.is_null() {
                    if (*(*da).id).xmlns != 0 {
                        let mut result_1: XML_Error = addBinding(
                            self,
                            (*(*da).id).prefix,
                            (*da).id,
                            (*da).value,
                            bindingsPtr,
                        );
                        if result_1 as u64 != 0 {
                            return result_1;
                        }
                        #[cfg(feature = "mozilla")]
                        {
                            *(*(*da).id).name.offset(-1) = 3;
                            nXMLNSDeclarations += 1;
                            *appAtts.offset(attIndex as isize) = (*(*da).id).name;
                            attIndex += 1;
                            *appAtts.offset(attIndex as isize) = (*da).value;
                            attIndex += 1;
                        }
                    } else {
                        *(*(*da).id).name.offset(-1) = 2;
                        nPrefixes += 1;
                        let fresh11 = attIndex;
                        attIndex = attIndex + 1;
                        let ref mut fresh12 = *appAtts.offset(fresh11 as isize);
                        *fresh12 = (*(*da).id).name;
                        let fresh13 = attIndex;
                        attIndex = attIndex + 1;
                        let ref mut fresh14 = *appAtts.offset(fresh13 as isize);
                        *fresh14 = (*da).value
                    }
                } else {
                    *(*(*da).id).name.offset(-1) = 1;
                    let fresh15 = attIndex;
                    attIndex = attIndex + 1;
                    let ref mut fresh16 = *appAtts.offset(fresh15 as isize);
                    *fresh16 = (*(*da).id).name;
                    let fresh17 = attIndex;
                    attIndex = attIndex + 1;
                    let ref mut fresh18 = *appAtts.offset(fresh17 as isize);
                    *fresh18 = (*da).value
                }
            }
            i += 1
        }
        let ref mut fresh19 = *appAtts.offset(attIndex as isize);
        *fresh19 = 0 as *const XML_Char;
        /* expand prefixed attribute names, check for duplicates,
        and clear flags that say whether attributes were specified */
        i = 0; /* hash table index */
        if nPrefixes != 0 || nXMLNSDeclarations != 0 { // MOZILLA CHANGE
            let mut j_0: c_int = 0;
            let mut version: c_ulong = self.m_nsAttsVersion;
            let mut nsAttsSize: c_int = (1) << self.m_nsAttsPower as c_int;
            let mut oldNsAttsPower: c_uchar = self.m_nsAttsPower;
            if nPrefixes != 0 { // MOZILLA CHANGE
                /* size of hash table must be at least 2 * (# of prefixed attributes) */
                if nPrefixes << 1 >> self.m_nsAttsPower as c_int != 0 {
                    /* true for m_nsAttsPower = 0 */
                    let mut temp_0: *mut NS_ATT = 0 as *mut NS_ATT;
                    loop
                    /* hash table size must also be a power of 2 and >= 8 */
                    {
                        let fresh20 = self.m_nsAttsPower;
                        self.m_nsAttsPower = self.m_nsAttsPower.wrapping_add(1);
                        if !(nPrefixes >> fresh20 as c_int != 0) {
                            break;
                        }
                    }
                    if (self.m_nsAttsPower as c_int) < 3 {
                        self.m_nsAttsPower = 3u8
                    }
                    nsAttsSize = (1) << self.m_nsAttsPower as c_int;
                    temp_0 = REALLOC!(
                        self.m_nsAtts as *mut c_void,
                        (nsAttsSize as c_ulong).wrapping_mul(::std::mem::size_of::<NS_ATT>() as c_ulong)
                    ) as *mut NS_ATT;
                    if temp_0.is_null() {
                        /* Restore actual size of memory in m_nsAtts */
                        self.m_nsAttsPower = oldNsAttsPower;
                        return XML_ERROR_NO_MEMORY;
                    }
                    self.m_nsAtts = temp_0;
                    version = 0
                }
                /* using a version flag saves us from initializing m_nsAtts every time */
                if version == 0 {
                    /* initialize version flags when version wraps around */
                    version = INIT_ATTS_VERSION as c_ulong;
                    j_0 = nsAttsSize;
                    while j_0 != 0 {
                        j_0 -= 1;
                        (*self.m_nsAtts.offset(j_0 as isize)).version = version
                    }
                }
                version = version.wrapping_sub(1);
                self.m_nsAttsVersion = version;
            } // MOZILLA CHANGE
            /* expand prefixed names and check for duplicates */
            while i < attIndex {
                let mut s: *const XML_Char = *appAtts.offset(i as isize);
                if *s.offset(-1) as c_int == 2 {
                    let mut b: *const BINDING = 0 as *const BINDING;
                    let mut uriHash: c_ulong = 0;
                    let mut sip_state: siphash = siphash {
                        v0: 0,
                        v1: 0,
                        v2: 0,
                        v3: 0,
                        buf: [0; 8],
                        p: 0 as *mut c_uchar,
                        c: 0,
                    };
                    let mut sip_key: sipkey = sipkey { k: [0; 2] };
                    self.copy_salt_to_sipkey(&mut sip_key);
                    sip24_init(&mut sip_state, &mut sip_key);
                    /* clear flag */
                    /* not prefixed */
                    /* prefixed */
                    *(s as *mut XML_Char).offset(-1) = 0; /* clear flag */
                    let id = (*dtd).attributeIds.get(&HashKey::from(s));
                    if id.is_none() || id.unwrap().prefix.is_null() {
                        /* This code is walking through the appAtts array, dealing
                        * with (in this case) a prefixed attribute name.  To be in
                        * the array, the attribute must have already been bound, so
                        * has to have passed through the hash table lookup once
                        * already.  That implies that an entry for it already
                        * exists, so the lookup above will return a pointer to
                        * already allocated memory.  There is no opportunaity for
                        * the allocator to fail, so the condition above cannot be
                        * fulfilled.
                        *
                        * Since it is difficult to be certain that the above
                        * analysis is complete, we retain the test and merely
                        * remove the code from coverage tests.
                        */
                        return XML_ERROR_NO_MEMORY;
                        /* LCOV_EXCL_LINE */
                    }
                    let id = id.unwrap();
                    b = (*(*id).prefix).binding;
                    if b.is_null() {
                        return XML_ERROR_UNBOUND_PREFIX;
                    }
                    j_0 = 0;
                    while j_0 < (*b).uriLen {
                        let c: XML_Char = *(*b).uri.offset(j_0 as isize);
                        if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                            && self.m_tempPool.grow() == 0
                        {
                            0
                        } else {
                            let fresh21 = self.m_tempPool.ptr;
                            self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                            *fresh21 = c;
                            1
                        } == 0
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        j_0 += 1
                    }
                    sip24_update(
                        &mut sip_state,
                        (*b).uri as *const c_void,
                        ((*b).uriLen as c_ulong)
                            .wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
                    );
                    loop {
                        let fresh22 = s;
                        s = s.offset(1);
                        if !(*fresh22 != ASCII_COLON as XML_Char) {
                            break;
                        }
                    }
                    sip24_update(
                        &mut sip_state,
                        s as *const c_void,
                        keylen(s).wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
                    );
                    loop {
                        /* copies null terminator */
                        if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                            && self.m_tempPool.grow() == 0
                        {
                            0
                        } else {
                            let fresh23 = self.m_tempPool.ptr;
                            self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                            *fresh23 = *s;
                            1
                        } == 0
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        let fresh24 = s;
                        s = s.offset(1);
                        if !(*fresh24 != 0) {
                            break;
                        }
                    }
                    uriHash = sip24_final(&mut sip_state);
                    /* Check hash table for duplicate of expanded name (uriName).
                    Derived from code in lookup(parser, HASH_TABLE *table, ...).
                    */
                    let mut step: c_uchar = 0; /* index into hash table */
                    let mut mask: c_ulong = (nsAttsSize - 1) as c_ulong;
                    j_0 = (uriHash & mask) as c_int;
                    while (*self.m_nsAtts.offset(j_0 as isize)).version == version {
                        /* for speed we compare stored hash values first */
                        if uriHash == (*self.m_nsAtts.offset(j_0 as isize)).hash {
                            let mut s1: *const XML_Char = self.m_tempPool.start;
                            let mut s2: *const XML_Char =
                                (*self.m_nsAtts.offset(j_0 as isize)).uriName;
                            /* s1 is null terminated, but not s2 */
                            while *s1 as c_int == *s2 as c_int && *s1 as c_int != 0 {
                                s1 = s1.offset(1);
                                s2 = s2.offset(1)
                            }
                            if *s1 as c_int == 0 {
                                return XML_ERROR_DUPLICATE_ATTRIBUTE;
                            }
                        }
                        if step == 0 {
                            step = ((uriHash & !mask) >> self.m_nsAttsPower as c_int - 1
                                & mask >> 2
                                | 1) as c_uchar
                        }
                        if j_0 < step as c_int {
                            j_0 += nsAttsSize - step as c_int
                        } else {
                            j_0 -= step as c_int
                        };
                    }
                    if self.m_ns_triplets != 0 {
                        /* append namespace separator and prefix */
                        *self.m_tempPool.ptr.offset(-1) = self.m_namespaceSeparator;
                        s = (*(*b).prefix).name;
                        loop {
                            if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                                && self.m_tempPool.grow() == 0
                            {
                                0
                            } else {
                                let fresh25 = self.m_tempPool.ptr;
                                self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                                *fresh25 = *s;
                                1
                            } == 0
                            {
                                return XML_ERROR_NO_MEMORY;
                            }
                            let fresh26 = s;
                            s = s.offset(1);
                            if !(*fresh26 != 0) {
                                break;
                            }
                        }
                    }
                    /* store expanded name in attribute list */
                    s = self.m_tempPool.start;
                    self.m_tempPool.start = self.m_tempPool.ptr;
                    let ref mut fresh27 = *appAtts.offset(i as isize);
                    *fresh27 = s;
                    /* fill empty slot with new version, uriName and hash value */
                    (*self.m_nsAtts.offset(j_0 as isize)).version = version;
                    (*self.m_nsAtts.offset(j_0 as isize)).hash = uriHash;
                    let ref mut fresh28 = (*self.m_nsAtts.offset(j_0 as isize)).uriName;
                    *fresh28 = s;
                    nPrefixes -= 1;
                    if nPrefixes == 0 && nXMLNSDeclarations == 0 {
                        i += 2;
                        break;
                    }
                } else if cfg!(feature = "mozilla") && *s.offset(-1) as c_int == 3 {
                    const xmlnsNamespace: [XML_Char; 30] = [
                        ASCII_h as XML_Char,
                        ASCII_t as XML_Char,
                        ASCII_t as XML_Char,
                        ASCII_p as XML_Char,
                        ASCII_COLON as XML_Char,
                        ASCII_SLASH as XML_Char,
                        ASCII_SLASH as XML_Char,
                        ASCII_w as XML_Char,
                        ASCII_w as XML_Char,
                        ASCII_w as XML_Char,
                        ASCII_PERIOD as XML_Char,
                        ASCII_w as XML_Char,
                        ASCII_3 as XML_Char,
                        ASCII_PERIOD as XML_Char,
                        ASCII_o as XML_Char,
                        ASCII_r as XML_Char,
                        ASCII_g as XML_Char,
                        ASCII_SLASH as XML_Char,
                        ASCII_2 as XML_Char,
                        ASCII_0 as XML_Char,
                        ASCII_0 as XML_Char,
                        ASCII_0 as XML_Char,
                        ASCII_SLASH as XML_Char,
                        ASCII_x as XML_Char,
                        ASCII_m as XML_Char,
                        ASCII_l as XML_Char,
                        ASCII_n as XML_Char,
                        ASCII_s as XML_Char,
                        ASCII_SLASH as XML_Char,
                        '\u{0}' as XML_Char
                    ];
                    const xmlnsPrefix: [XML_Char; 6] = [
                        ASCII_x as XML_Char, ASCII_m as XML_Char, ASCII_l as XML_Char,
                        ASCII_n as XML_Char, ASCII_s as XML_Char, '\u{0}' as XML_Char
                    ];

                    *(s as *mut XML_Char).offset(-1) = 0; /* clear flag */
                    if self.m_tempPool.appendString(xmlnsNamespace.as_ptr()) ||
                        !self.m_tempPool.appendChar(self.m_namespaceSeparator)
                    {
                        return XML_ERROR_NO_MEMORY;
                    }

                    s = s.offset(xmlnsPrefix.len() as isize - 1);
                    if *s == ':' as XML_Char {
                        s = s.offset(1);
                        loop { /* copies null terminator */
                            if !self.m_tempPool.appendChar(*s) {
                                return XML_ERROR_NO_MEMORY;
                            }
                            if *s == '\u{0}' as XML_Char {
                                s = s.offset(1);
                                break;
                            }
                            s = s.offset(1);
                        }

                        if self.m_ns_triplets != 0 { /* append namespace separator and prefix */
                            *self.m_tempPool.ptr.offset(-1) = self.m_namespaceSeparator;
                            if self.m_tempPool.appendString(xmlnsPrefix.as_ptr()) ||
                                !self.m_tempPool.appendChar('\u{0}' as XML_Char)
                            {
                                return XML_ERROR_NO_MEMORY;
                            }
                        }
                    } else {
                        /* xlmns attribute without a prefix. */
                        if self.m_tempPool.appendString(xmlnsPrefix.as_ptr()) ||
                            !self.m_tempPool.appendChar('\u{0}' as XML_Char)
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                    }

                    /* store expanded name in attribute list */
                    s = self.m_tempPool.start;
                    self.m_tempPool.start = self.m_tempPool.ptr;
                    let ref mut fresh27 = *appAtts.offset(i as isize);
                    *fresh27 = s;

                    nXMLNSDeclarations -= 1;
                    if nXMLNSDeclarations == 0 && nPrefixes == 0 {
                        i += 2;
                        break;
                    }
                } else {
                    *(s as *mut XML_Char).offset(-1) = 0
                }
                i += 2
            }
        }
        /* clear flags for the remaining attributes */
        while i < attIndex {
            *(*appAtts.offset(i as isize) as *mut XML_Char).offset(-1) = 0;
            i += 2
        }
        binding = *bindingsPtr;
        while !binding.is_null() {
            *(*(*binding).attId).name.offset(-1) = 0;
            binding = (*binding).nextTagBinding
        }
        if self.m_ns == 0 {
            return XML_ERROR_NONE;
        }
        /* expand the element type name */
        if !(*elementType).prefix.is_null() {
            binding = (*(*elementType).prefix).binding;
            if binding.is_null() {
                return XML_ERROR_UNBOUND_PREFIX;
            }
            localPart = (*tagNamePtr).str_0;
            loop {
                let fresh29 = localPart;
                localPart = localPart.offset(1);
                if !(*fresh29 != ASCII_COLON as XML_Char) {
                    break;
                }
            }
        } else if !(*dtd).defaultPrefix.binding.is_null() {
            binding = (*dtd).defaultPrefix.binding;
            localPart = (*tagNamePtr).str_0
        } else {
            return XML_ERROR_NONE;
        }
        prefixLen = 0;
        if self.m_ns_triplets as c_int != 0 && !(*(*binding).prefix).name.is_null() {
            loop {
                let fresh30 = prefixLen;
                prefixLen = prefixLen + 1;
                if !(*(*(*binding).prefix).name.offset(fresh30 as isize) != 0) {
                    break;
                }
            }
            /* prefixLen includes null terminator */
        } /* i includes null terminator */
        (*tagNamePtr).localPart = localPart;
        (*tagNamePtr).uriLen = (*binding).uriLen;
        (*tagNamePtr).prefix = (*(*binding).prefix).name;
        (*tagNamePtr).prefixLen = prefixLen;
        i = 0;
        loop {
            let fresh31 = i;
            i = i + 1;
            if !(*localPart.offset(fresh31 as isize) != 0) {
                break;
            }
        }
        n = i + (*binding).uriLen + prefixLen;
        if n > (*binding).uriAlloc {
            let mut p: *mut TAG = 0 as *mut TAG;
            uri = MALLOC![XML_Char; n + 24];
            if uri.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            (*binding).uriAlloc = n + EXPAND_SPARE;
            memcpy(
                uri as *mut c_void,
                (*binding).uri as *const c_void,
                ((*binding).uriLen as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
            );
            p = self.m_tagStack;
            while !p.is_null() {
                if (*p).name.str_0 == (*binding).uri as *const XML_Char {
                    (*p).name.str_0 = uri
                }
                p = (*p).parent
            }
            FREE!((*binding).uri);
            (*binding).uri = uri
        }
        /* if m_namespaceSeparator != '\0' then uri includes it already */
        uri = (*binding).uri.offset((*binding).uriLen as isize);
        memcpy(
            uri as *mut c_void,
            localPart as *const c_void,
            (i as c_ulong).wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
        );
        /* we always have a namespace separator between localPart and prefix */
        if prefixLen != 0 {
            uri = uri.offset((i - 1) as isize); /* replace null terminator */
            *uri = self.m_namespaceSeparator;
            memcpy(
                uri.offset(1isize) as *mut c_void,
                (*(*binding).prefix).name as *const c_void,
                (prefixLen as c_ulong).wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
            );
        }
        (*tagNamePtr).str_0 = (*binding).uri;
        XML_ERROR_NONE
    }
}

// Initialized in run_static_initializers
static mut xmlLen: c_int = 0;
// Initialized in run_static_initializers
static mut xmlnsLen: c_int = 0;
/* addBinding() overwrites the value of prefix->binding without checking.
   Therefore one must keep track of the old value outside of addBinding().
*/

unsafe extern "C" fn addBinding(
    mut parser: XML_Parser,
    mut prefix: *mut PREFIX,
    mut attId: *const ATTRIBUTE_ID,
    mut uri: *const XML_Char,
    mut bindingsPtr: *mut *mut BINDING,
) -> XML_Error {
    static mut xmlNamespace: [XML_Char; 37] = [
        ASCII_h as XML_Char,
        ASCII_t as XML_Char,
        ASCII_t as XML_Char,
        ASCII_p as XML_Char,
        ASCII_COLON as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_w as XML_Char,
        ASCII_3 as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_o as XML_Char,
        ASCII_r as XML_Char,
        ASCII_g as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_X as XML_Char,
        ASCII_M as XML_Char,
        ASCII_L as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_1 as XML_Char,
        ASCII_9 as XML_Char,
        ASCII_9 as XML_Char,
        ASCII_8 as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_n as XML_Char,
        ASCII_a as XML_Char,
        ASCII_m as XML_Char,
        ASCII_e as XML_Char,
        ASCII_s as XML_Char,
        ASCII_p as XML_Char,
        ASCII_a as XML_Char,
        ASCII_c as XML_Char,
        ASCII_e as XML_Char,
        '\u{0}' as XML_Char,
    ];
    static mut xmlnsNamespace: [XML_Char; 30] = [
        ASCII_h as XML_Char,
        ASCII_t as XML_Char,
        ASCII_t as XML_Char,
        ASCII_p as XML_Char,
        ASCII_COLON as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_w as XML_Char,
        ASCII_3 as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_o as XML_Char,
        ASCII_r as XML_Char,
        ASCII_g as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_2 as XML_Char,
        ASCII_0 as XML_Char,
        ASCII_0 as XML_Char,
        ASCII_0 as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_x as XML_Char,
        ASCII_m as XML_Char,
        ASCII_l as XML_Char,
        ASCII_n as XML_Char,
        ASCII_s as XML_Char,
        ASCII_SLASH as XML_Char,
        '\u{0}' as XML_Char,
    ];
    let mut mustBeXML: XML_Bool = XML_FALSE;
    let mut isXML: XML_Bool = XML_TRUE;
    let mut isXMLNS: XML_Bool = XML_TRUE;
    let mut b: *mut BINDING = 0 as *mut BINDING;
    let mut len: c_int = 0;
    /* empty URI is only valid for default namespace per XML NS 1.0 (not 1.1) */
    if *uri as c_int == '\u{0}' as i32 && !(*prefix).name.is_null() {
        return XML_ERROR_UNDECLARING_PREFIX;
    }
    if !(*prefix).name.is_null()
        && *(*prefix).name.offset(0) == ASCII_x as XML_Char
        && *(*prefix).name.offset(1) == ASCII_m as XML_Char
        && *(*prefix).name.offset(2) == ASCII_l as XML_Char
    {
        /* Not allowed to bind xmlns */
        if *(*prefix).name.offset(3) == ASCII_n as XML_Char
            && *(*prefix).name.offset(4) == ASCII_s as XML_Char
            && *(*prefix).name.offset(5) == '\u{0}' as XML_Char
        {
            return XML_ERROR_RESERVED_PREFIX_XMLNS;
        }
        if *(*prefix).name.offset(3) == '\u{0}' as XML_Char {
            mustBeXML = XML_TRUE
        }
    }
    len = 0;
    while *uri.offset(len as isize) != 0 {
        if isXML as c_int != 0
            && (len > xmlLen
                || *uri.offset(len as isize) as c_int != xmlNamespace[len as usize] as c_int)
        {
            isXML = XML_FALSE
        }
        if mustBeXML == 0
            && isXMLNS as c_int != 0
            && (len > xmlnsLen
                || *uri.offset(len as isize) as c_int != xmlnsNamespace[len as usize] as c_int)
        {
            isXMLNS = XML_FALSE
        }
        len += 1
    }
    isXML = (isXML as c_int != 0 && len == xmlLen) as XML_Bool;
    isXMLNS = (isXMLNS as c_int != 0 && len == xmlnsLen) as XML_Bool;
    if mustBeXML as c_int != isXML as c_int {
        return if mustBeXML as c_int != 0 {
            XML_ERROR_RESERVED_PREFIX_XML as c_int
        } else {
            XML_ERROR_RESERVED_NAMESPACE_URI as c_int
        } as XML_Error;
    }
    if isXMLNS != 0 {
        return XML_ERROR_RESERVED_NAMESPACE_URI;
    }
    if (*parser).m_namespaceSeparator != 0 {
        len += 1
    }
    if !(*parser).m_freeBindingList.is_null() {
        b = (*parser).m_freeBindingList;
        if len > (*b).uriAlloc {
            let mut temp: *mut XML_Char = REALLOC!(
                (*b).uri as *mut c_void,
                (::std::mem::size_of::<XML_Char>() as c_ulong).wrapping_mul((len + 24) as c_ulong)
            ) as *mut XML_Char;
            if temp.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            (*b).uri = temp;
            (*b).uriAlloc = len + EXPAND_SPARE
        }
        (*parser).m_freeBindingList = (*b).nextTagBinding
    } else {
        b = MALLOC!(::std::mem::size_of::<BINDING>() as c_ulong) as *mut BINDING;
        if b.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        (*b).uri = MALLOC![XML_Char; len + 24];
        if (*b).uri.is_null() {
            FREE!(b);
            return XML_ERROR_NO_MEMORY;
        }
        (*b).uriAlloc = len + EXPAND_SPARE
    }
    (*b).uriLen = len;
    memcpy(
        (*b).uri as *mut c_void,
        uri as *const c_void,
        (len as c_ulong).wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
    );
    if (*parser).m_namespaceSeparator != 0 {
        *(*b).uri.offset((len - 1) as isize) = (*parser).m_namespaceSeparator
    }
    (*b).prefix = prefix;
    (*b).attId = attId;
    (*b).prevPrefixBinding = (*prefix).binding;
    /* NULL binding when default namespace undeclared */
    if *uri == '\u{0}' as XML_Char && prefix == &mut (*(*parser).m_dtd).defaultPrefix as *mut PREFIX
    {
        (*prefix).binding = NULL as *mut BINDING
    } else {
        (*prefix).binding = b
    }
    (*b).nextTagBinding = *bindingsPtr;
    *bindingsPtr = b;
    /* if attId == NULL then we are not starting a namespace scope */
    if !attId.is_null() && (*parser).m_handlers.hasStartNamespaceDecl() {
        (*parser).m_handlers.startNamespaceDecl(
            (*prefix).name,
            if !(*prefix).binding.is_null() {
                uri
            } else {
                0 as *const XML_Char
            },
        );
    }
    return XML_ERROR_NONE;
}
/* The idea here is to avoid using stack for each CDATA section when
   the whole file is parsed with one call.
*/

unsafe extern "C" fn cdataSectionProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = doCdataSection(
        parser,
        EncodingType::Normal,
        &mut start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
    );
    if result != XML_ERROR_NONE {
        return result;
    }
    if !start.is_null() {
        if !(*parser).m_parentParser.is_null() {
            /* we are parsing an external entity */
            (*parser).m_processor = Some(externalEntityContentProcessor as Processor);
            return externalEntityContentProcessor(parser, start, end, endPtr);
        } else {
            (*parser).m_processor = Some(contentProcessor as Processor);
            return contentProcessor(parser, start, end, endPtr);
        }
    }
    return result;
}
/* startPtr gets set to non-null if the section is closed, and to null if
   the section is not yet closed.
*/

unsafe extern "C" fn doCdataSection(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut startPtr: *mut *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
) -> XML_Error {
    let mut s: *const c_char = *startPtr;
    let mut eventPP: *mut *const c_char = 0 as *mut *const c_char;
    let mut eventEndPP: *mut *const c_char = 0 as *mut *const c_char;
    if !enc_type.is_internal() {
        eventPP = &mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &mut (*parser).m_eventEndPtr
    } else {
        eventPP = &mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &mut (*(*parser).m_openInternalEntities).internalEventEndPtr
    }
    *eventPP = s;
    *startPtr = NULL as *const c_char;
    let enc = (*parser).encoding(enc_type);
    loop {
        let mut next: *const c_char = 0 as *const c_char;
        let mut tok: c_int = (*enc).xmlTok(XML_CDATA_SECTION_STATE, s, end, &mut next);
        *eventEndPP = next;
        match tok {
            super::xmltok::XML_TOK_CDATA_SECT_CLOSE => {
                let endHandlerRan = (*parser).m_handlers.endCDataSection();

                if endHandlerRan {
                } else if 0 != 0 && (*parser).m_handlers.hasCharacterData() {
                    (*parser).m_handlers.characterData((*parser).m_dataBuf, 0);
                } else if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, enc_type, s, next);
                }
                *startPtr = next;
                *nextPtr = next;
                if (*parser).m_parsingStatus.parsing == XML_FINISHED {
                    return XML_ERROR_ABORTED;
                } else {
                    return XML_ERROR_NONE;
                }
                /* BEGIN disabled code */
                /* see comment under XML_TOK_CDATA_SECT_OPEN */
                /* END disabled code */
                /* LCOV_EXCL_STOP */
            }
            super::xmltok::XML_TOK_DATA_NEWLINE => {
                if (*parser).m_handlers.hasCharacterData() {
                    let mut c: XML_Char = 0xa;
                    (*parser).m_handlers.characterData(&mut c, 1);
                } else if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, enc_type, s, next);
                }
            }
            super::xmltok::XML_TOK_DATA_CHARS => {
                let mut handlers = (*parser).m_handlers;
                if handlers.hasCharacterData() {
                    if MUST_CONVERT!(enc, s) {
                        loop {
                            let mut dataPtr = (*parser).m_dataBuf as *mut ICHAR;
                            let convert_res: super::xmltok::XML_Convert_Result = XmlConvert!(
                                enc,
                                &mut s,
                                next,
                                &mut dataPtr,
                                (*parser).m_dataBufEnd as *mut ICHAR,
                            );
                            *eventEndPP = next;
                            handlers.characterData(
                                (*parser).m_dataBuf,
                                dataPtr.wrapping_offset_from((*parser).m_dataBuf as *mut ICHAR) as c_int,
                            );
                            if convert_res == super::xmltok::XML_CONVERT_COMPLETED
                                || convert_res == super::xmltok::XML_CONVERT_INPUT_INCOMPLETE
                            {
                                break;
                            }
                            *eventPP = s
                        }
                    } else {
                        handlers.characterData(
                            s as *mut XML_Char,
                            (next as *mut XML_Char).wrapping_offset_from(s as *mut XML_Char)
                                as c_int,
                        );
                    }
                } else if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, enc_type, s, next);
                }
            }
            super::xmltok::XML_TOK_INVALID => {
                *eventPP = next;
                return XML_ERROR_INVALID_TOKEN;
            }
            super::xmltok::XML_TOK_PARTIAL_CHAR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_PARTIAL_CHAR;
            }
            super::xmltok::XML_TOK_PARTIAL | super::xmltok::XML_TOK_NONE => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_UNCLOSED_CDATA_SECTION;
            }
            _ => {
                /* Every token returned by XmlCdataSectionTok() has its own
                 * explicit case, so this default case will never be executed.
                 * We retain it as a safety net and exclude it from the coverage
                 * statistics.
                 *
                 * LCOV_EXCL_START
                 */
                *eventPP = next;
                return XML_ERROR_UNEXPECTED_STATE;
            }
        }
        s = next;
        *eventPP = s;
        match (*parser).m_parsingStatus.parsing {
            3 => {
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            2 => return XML_ERROR_ABORTED,
            _ => {}
        }
    }
    /* not reached */
}
/* The idea here is to avoid using stack for each IGNORE section when
   the whole file is parsed with one call.
*/

unsafe extern "C" fn ignoreSectionProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = doIgnoreSection(
        parser,
        EncodingType::Normal,
        &mut start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
    );
    if result != XML_ERROR_NONE {
        return result;
    }
    if !start.is_null() {
        (*parser).m_processor = Some(prologProcessor as Processor);
        return prologProcessor(parser, start, end, endPtr);
    }
    return result;
}
/* startPtr gets set to non-null is the section is closed, and to null
   if the section is not yet closed.
*/

unsafe extern "C" fn doIgnoreSection(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut startPtr: *mut *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
) -> XML_Error {
    let mut next: *const c_char = 0 as *const c_char;
    let mut tok: c_int = 0;
    let mut s: *const c_char = *startPtr;
    let mut eventPP: *mut *const c_char = 0 as *mut *const c_char;
    let mut eventEndPP: *mut *const c_char = 0 as *mut *const c_char;
    if !enc_type.is_internal() {
        eventPP = &mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &mut (*parser).m_eventEndPtr
    } else {
        /* It's not entirely clear, but it seems the following two lines
         * of code cannot be executed.  The only occasions on which 'enc'
         * is not 'encoding' are when this function is called
         * from the internal entity processing, and IGNORE sections are an
         * error in internal entities.
         *
         * Since it really isn't clear that this is true, we keep the code
         * and just remove it from our coverage tests.
         *
         * LCOV_EXCL_START
         */
        eventPP = &mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &mut (*(*parser).m_openInternalEntities).internalEventEndPtr
        /* LCOV_EXCL_STOP */
    }
    *eventPP = s;
    *startPtr = NULL as *const c_char;
    let enc = (*parser).encoding(enc_type);
    tok = (*enc).xmlTok(XML_IGNORE_SECTION_STATE, s, end, &mut next);
    *eventEndPP = next;
    match tok {
        super::xmltok::XML_TOK_IGNORE_SECT => {
            if (*parser).m_handlers.hasDefault() {
                reportDefault(parser, enc_type, s, next);
            }
            *startPtr = next;
            *nextPtr = next;
            if (*parser).m_parsingStatus.parsing == XML_FINISHED {
                return XML_ERROR_ABORTED;
            } else {
                return XML_ERROR_NONE;
            }
            /* LCOV_EXCL_STOP */
        }
        super::xmltok::XML_TOK_INVALID => {
            *eventPP = next; /* XML_ERROR_UNCLOSED_IGNORE_SECTION */
            return XML_ERROR_INVALID_TOKEN;
        }
        super::xmltok::XML_TOK_PARTIAL_CHAR => {
            if haveMore != 0 {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_PARTIAL_CHAR;
        }
        super::xmltok::XML_TOK_PARTIAL | super::xmltok::XML_TOK_NONE => {
            if haveMore != 0 {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_SYNTAX;
        }
        _ => {
            /* All of the tokens that XmlIgnoreSectionTok() returns have
             * explicit cases to handle them, so this default case is never
             * executed.  We keep it as a safety net anyway, and remove it
             * from our test coverage statistics.
             *
             * LCOV_EXCL_START
             */
            *eventPP = next;
            return XML_ERROR_UNEXPECTED_STATE;
        }
    };
    /* not reached */
}
/* XML_DTD */

impl XML_ParserStruct {
    unsafe fn initializeEncoding(&mut self) -> XML_Error {
        let mut s: *const c_char = 0 as *const c_char;
        if cfg!(feature = "unicode") {
            let mut encodingBuf: [libc::c_char; 128] = [0; 128];
            /* See comments abount `protoclEncodingName` in parserInit() */
            if self.m_protocolEncodingName.is_null() {
                s = crate::stddef_h::NULL as *const libc::c_char;
            } else {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while *self.m_protocolEncodingName.offset(i as isize) != 0 {
                    if i as libc::c_ulong
                        == (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        || *self.m_protocolEncodingName.offset(i as isize) as libc::c_int
                            & !(0x7f as libc::c_int)
                            != 0 as libc::c_int
                    {
                        encodingBuf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                        break;
                    } else {
                        encodingBuf[i as usize] =
                            *self.m_protocolEncodingName.offset(i as isize) as libc::c_char;
                        i += 1
                    }
                }
                encodingBuf[i as usize] = '\u{0}' as i32 as libc::c_char;
                s = encodingBuf.as_mut_ptr();
            }
        } else {
            s = self.m_protocolEncodingName as *const c_char;
        }
        let enc = if self.m_ns as c_int != 0 {
            InitEncoding::new_ns(&mut self.m_encoding, s)
        } else {
            InitEncoding::new(&mut self.m_encoding, s)
        };
        if enc.is_some() {
            self.m_initEncoding = enc;
            self.m_encoding = &*self.m_initEncoding.as_ref().unwrap();
            return XML_ERROR_NONE;
        }

        self.handleUnknownEncoding(self.m_protocolEncodingName)
    }
}

impl XML_ParserStruct {
    unsafe fn processXmlDecl(
        &mut self,
        mut isGeneralTextEntity: c_int,
        mut s: *const c_char,
        mut next: *const c_char,
    ) -> XML_Error {
        let mut encodingName: *const c_char = NULL as *const c_char;
        let mut storedEncName: *const XML_Char = NULL as *const XML_Char;
        let mut newEncoding: Option<*const ENCODING> = None;
        let mut version: *const c_char = NULL as *const c_char;
        let mut versionend: *const c_char = 0 as *const c_char;
        let mut storedversion: *const XML_Char = NULL as *const XML_Char;
        let mut standalone: c_int = -(1);
        if if self.m_ns as c_int != 0 {
            super::xmltok::XmlParseXmlDeclNS
        } else {
            super::xmltok::XmlParseXmlDecl
        }(
            isGeneralTextEntity,
            &*self.m_encoding,
            s,
            next,
            &mut self.m_eventPtr,
            &mut version,
            &mut versionend,
            &mut encodingName,
            &mut newEncoding,
            &mut standalone,
        ) == 0
        {
            if isGeneralTextEntity != 0 {
                return XML_ERROR_TEXT_DECL;
            } else {
                return XML_ERROR_XML_DECL;
            }
        }
        if isGeneralTextEntity == 0 && standalone == 1 {
            (*self.m_dtd).standalone = XML_TRUE;
            if self.m_paramEntityParsing == XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE {
                self.m_paramEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER
            }
            /* XML_DTD */
        }
        if self.m_handlers.hasXmlDecl() {
            if !encodingName.is_null() {
                storedEncName = self.m_temp2Pool.storeString(
                    &*self.m_encoding,
                    encodingName,
                    encodingName.offset((*self.m_encoding).nameLength(encodingName) as isize),
                );
                if storedEncName.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                self.m_temp2Pool.start = self.m_temp2Pool.ptr
            }
            if !version.is_null() {
                storedversion = self.m_temp2Pool.storeString(
                    &*self.m_encoding,
                    version,
                    versionend.offset(-((*self.m_encoding).minBytesPerChar() as isize)),
                );
                if storedversion.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            self.m_handlers.xmlDecl(
                storedversion,
                storedEncName,
                standalone,
            );
        } else if self.m_handlers.hasDefault() {
            reportDefault(self, EncodingType::Normal, s, next);
        }
        if self.m_protocolEncodingName.is_null() {
            if let Some(newEncoding) = newEncoding {
                /* Check that the specified encoding does not conflict with what
                 * the parser has already deduced.  Do we have the same number
                 * of bytes in the smallest representation of a character?  If
                 * this is UTF-16, is it the same endianness?
                 */
                if (*newEncoding).minBytesPerChar() != (*self.m_encoding).minBytesPerChar()
                    || (*newEncoding).minBytesPerChar() == 2 && !ptr::eq(newEncoding, self.m_encoding)
                {
                    self.m_eventPtr = encodingName;
                    return XML_ERROR_INCORRECT_ENCODING;
                }
                self.m_encoding = newEncoding
            } else if !encodingName.is_null() {
                let mut result: XML_Error = XML_ERROR_NONE;
                if storedEncName.is_null() {
                    storedEncName = self.m_temp2Pool.storeString(
                        &*self.m_encoding,
                        encodingName,
                        encodingName.offset((*self.m_encoding).nameLength(encodingName) as isize),
                    );
                    if storedEncName.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                }
                result = self.handleUnknownEncoding(storedEncName);
                self.m_temp2Pool.clear();
                if result == XML_ERROR_UNKNOWN_ENCODING {
                    self.m_eventPtr = encodingName
                }
                return result;
            }
        }
        if !storedEncName.is_null() || !storedversion.is_null() {
            self.m_temp2Pool.clear();
        }
        XML_ERROR_NONE
    }

    unsafe fn handleUnknownEncoding(
        &mut self,
        mut encodingName: *const XML_Char,
    ) -> XML_Error {
        if self.m_handlers.hasUnknownEncoding() {
            let mut info: XML_Encoding = XML_Encoding {
                map: [0; 256],
                data: 0 as *mut c_void,
                convert: None,
                release: None,
            };
            let mut i: c_int = 0;
            i = 0;
            while i < 256 {
                info.map[i as usize] = -(1);
                i += 1
            }
            info.convert = ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int>,
            >(NULL as intptr_t);
            info.data = NULL as *mut c_void;
            info.release = ::std::mem::transmute::<
                intptr_t,
                Option<unsafe extern "C" fn(_: *mut c_void)>,
            >(NULL as intptr_t);

            // Unwrapping because the handler was already checked to exist
            if self.m_handlers.unknownEncoding(encodingName, &mut info).unwrap() != 0 {
                let mut unknown_enc = UnknownEncoding::new();
                let initialized = unknown_enc.initialize(
                    info.map.as_mut_ptr(),
                    info.convert,
                    info.data,
                    self.m_ns != 0
                );
                if initialized {
                    match Box::try_new(unknown_enc) {
                        Err(_) => {
                            if info.release.is_some() {
                                info.release.expect("non-null function pointer")(info.data);
                            }
                            return XML_ERROR_NO_MEMORY;
                        }
                        Ok(unknown_enc) => {
                            self.m_encoding = &*unknown_enc;
                            self.m_unknownEncoding = Some(unknown_enc);
                            self.m_unknownEncodingData = info.data;
                            self.m_unknownEncodingRelease = info.release;
                            return XML_ERROR_NONE;
                        }
                    }
                }
            }
            if info.release.is_some() {
                info.release.expect("non-null function pointer")(info.data);
            }
        }
        XML_ERROR_UNKNOWN_ENCODING
    }
}

unsafe extern "C" fn prologInitProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).initializeEncoding();
    if result != XML_ERROR_NONE {
        return result;
    }
    (*parser).m_processor = Some(prologProcessor as Processor);
    prologProcessor(parser, s, end, nextPtr)
}

unsafe extern "C" fn externalParEntInitProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).initializeEncoding();
    if result != XML_ERROR_NONE {
        return result;
    }
    /* we know now that XML_Parse(Buffer) has been called,
    so we consider the external parameter entity read */
    (*(*parser).m_dtd).paramEntityRead = XML_TRUE;
    if (*parser).m_prologState.inEntityValue != 0 {
        (*parser).m_processor = Some(entityValueInitProcessor as Processor);
        entityValueInitProcessor(parser, s, end, nextPtr)
    } else {
        (*parser).m_processor = Some(externalParEntProcessor as Processor);
        externalParEntProcessor(parser, s, end, nextPtr)
    }
}

unsafe extern "C" fn entityValueInitProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut tok: c_int = 0;
    let mut start: *const c_char = s;
    let mut next: *const c_char = start;
    (*parser).m_eventPtr = start;
    loop {
        tok = (*(*parser).m_encoding).xmlTok(XML_PROLOG_STATE, start, end, &mut next);
        (*parser).m_eventEndPtr = next;
        if tok <= 0 {
            if (*parser).m_parsingStatus.finalBuffer == 0 && tok != super::xmltok::XML_TOK_INVALID {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            match tok {
                super::xmltok::XML_TOK_INVALID => return XML_ERROR_INVALID_TOKEN,
                super::xmltok::XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
                super::xmltok::XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
                super::xmltok::XML_TOK_NONE | _ => {}
            }
            /* found end of entity value - can store it now */
            return storeEntityValue(parser, EncodingType::Normal, s, end);
        } else {
            if tok == super::xmltok::XML_TOK_XML_DECL {
                let mut result: XML_Error = XML_ERROR_NONE;
                result = (*parser).processXmlDecl(0, start, next);
                if result != XML_ERROR_NONE {
                    return result;
                }
                /* At this point, m_parsingStatus.parsing cannot be XML_SUSPENDED.  For
                 * that to happen, a parameter entity parsing handler must have attempted
                 * to suspend the parser, which fails and raises an error.  The parser can
                 * be aborted, but can't be suspended.
                 */
                if (*parser).m_parsingStatus.parsing == XML_FINISHED {
                    return XML_ERROR_ABORTED;
                }
                *nextPtr = next;
                /* stop scanning for text declaration - we found one */
                (*parser).m_processor = Some(entityValueProcessor as Processor);
                return entityValueProcessor(parser, next, end, nextPtr);
            } else {
                /* If we are at the end of the buffer, this would cause XmlPrologTok to
                   return XML_TOK_NONE on the next call, which would then cause the
                   function to exit with *nextPtr set to s - that is what we want for other
                   tokens, but not for the BOM - we would rather like to skip it;
                   then, when this routine is entered the next time, XmlPrologTok will
                   return XML_TOK_INVALID, since the BOM is still in the buffer
                */
                if tok == super::xmltok::XML_TOK_BOM
                    && next == end
                    && (*parser).m_parsingStatus.finalBuffer == 0
                {
                    *nextPtr = next;
                    return XML_ERROR_NONE;
                } else {
                    /* If we get this token, we have the start of what might be a
                       normal tag, but not a declaration (i.e. it doesn't begin with
                       "<!").  In a DTD context, that isn't legal.
                    */
                    if tok == super::xmltok::XML_TOK_INSTANCE_START {
                        *nextPtr = next;
                        return XML_ERROR_SYNTAX;
                    }
                }
            }
        }
        start = next;
        (*parser).m_eventPtr = start
    }
}

unsafe extern "C" fn externalParEntProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = s;
    let mut tok: c_int = 0;
    tok = (*(*parser).m_encoding).xmlTok(XML_PROLOG_STATE, s, end, &mut next);
    if tok <= 0 {
        if (*parser).m_parsingStatus.finalBuffer == 0 && tok != super::xmltok::XML_TOK_INVALID {
            *nextPtr = s;
            return XML_ERROR_NONE;
        }
        match tok {
            super::xmltok::XML_TOK_INVALID => return XML_ERROR_INVALID_TOKEN,
            super::xmltok::XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
            super::xmltok::XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
            super::xmltok::XML_TOK_NONE | _ => {}
        }
    } else if tok == super::xmltok::XML_TOK_BOM {
        s = next;
        tok = (*(*parser).m_encoding).xmlTok(XML_PROLOG_STATE, s, end, &mut next)
    }
    (*parser).m_processor = Some(prologProcessor as Processor);
    return (*parser).doProlog(
        EncodingType::Normal,
        s,
        end,
        tok,
        next,
        nextPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
        XML_TRUE,
    );
}

unsafe extern "C" fn entityValueProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut start: *const c_char = s;
    let mut next: *const c_char = s;
    let mut enc: &ENCODING = &*(*parser).m_encoding;
    let mut tok: c_int = 0;
    loop {
        tok = (*enc).xmlTok(XML_PROLOG_STATE, start, end, &mut next);
        if tok <= 0 {
            if (*parser).m_parsingStatus.finalBuffer == 0 && tok != super::xmltok::XML_TOK_INVALID {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            match tok {
                super::xmltok::XML_TOK_INVALID => return XML_ERROR_INVALID_TOKEN,
                super::xmltok::XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
                super::xmltok::XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
                super::xmltok::XML_TOK_NONE | _ => {}
            }
            /* This would cause the next stage, i.e. doProlog to be passed XML_TOK_BOM.
               However, when parsing an external subset, doProlog will not accept a BOM
               as valid, and report a syntax error, so we have to skip the BOM
            */
            /* found end of entity value - can store it now */
            return storeEntityValue(parser, EncodingType::Normal, s, end);
        }
        start = next
    }
}
/* XML_DTD */

unsafe extern "C" fn prologProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = s;
    let mut tok: c_int = (*(*parser).m_encoding).xmlTok(XML_PROLOG_STATE, s, end, &mut next);
    return (*parser).doProlog(
        EncodingType::Normal,
        s,
        end,
        tok,
        next,
        nextPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
        XML_TRUE,
    );
}

impl XML_ParserStruct {
    unsafe fn doProlog<'a>(
        &mut self,
        mut enc_type: EncodingType,
        mut s: *const c_char,
        mut end: *const c_char,
        mut tok: c_int,
        mut next: *const c_char,
        mut nextPtr: *mut *const c_char,
        mut haveMore: XML_Bool,
        mut allowClosingDoctype: XML_Bool,
    ) -> XML_Error {
        let mut current_block: u64;
        static mut externalSubsetName: [XML_Char; 2] = [ASCII_HASH as XML_Char, '\u{0}' as XML_Char];
        /* XML_DTD */
        static mut atypeCDATA: [XML_Char; 6] = [
            ASCII_C as XML_Char,
            ASCII_D as XML_Char,
            ASCII_A as XML_Char,
            ASCII_T as XML_Char,
            ASCII_A as XML_Char,
            '\u{0}' as XML_Char,
        ];
        static mut atypeID: [XML_Char; 3] = [
            ASCII_I as XML_Char,
            ASCII_D as XML_Char,
            '\u{0}' as XML_Char,
        ];
        static mut atypeIDREF: [XML_Char; 6] = [
            ASCII_I as XML_Char,
            ASCII_D as XML_Char,
            ASCII_R as XML_Char,
            ASCII_E as XML_Char,
            ASCII_F as XML_Char,
            '\u{0}' as XML_Char,
        ];
        static mut atypeIDREFS: [XML_Char; 7] = [
            ASCII_I as XML_Char,
            ASCII_D as XML_Char,
            ASCII_R as XML_Char,
            ASCII_E as XML_Char,
            ASCII_F as XML_Char,
            ASCII_S as XML_Char,
            '\u{0}' as XML_Char,
        ];
        static mut atypeENTITY: [XML_Char; 7] = [
            ASCII_E as XML_Char,
            ASCII_N as XML_Char,
            ASCII_T as XML_Char,
            ASCII_I as XML_Char,
            ASCII_T as XML_Char,
            ASCII_Y as XML_Char,
            '\u{0}' as XML_Char,
        ];
        static mut atypeENTITIES: [XML_Char; 9] = [
            ASCII_E as XML_Char,
            ASCII_N as XML_Char,
            ASCII_T as XML_Char,
            ASCII_I as XML_Char,
            ASCII_T as XML_Char,
            ASCII_I as XML_Char,
            ASCII_E as XML_Char,
            ASCII_S as XML_Char,
            '\u{0}' as XML_Char,
        ];
        static mut atypeNMTOKEN: [XML_Char; 8] = [
            ASCII_N as XML_Char,
            ASCII_M as XML_Char,
            ASCII_T as XML_Char,
            ASCII_O as XML_Char,
            ASCII_K as XML_Char,
            ASCII_E as XML_Char,
            ASCII_N as XML_Char,
            '\u{0}' as XML_Char,
        ];
        static mut atypeNMTOKENS: [XML_Char; 9] = [
            ASCII_N as XML_Char,
            ASCII_M as XML_Char,
            ASCII_T as XML_Char,
            ASCII_O as XML_Char,
            ASCII_K as XML_Char,
            ASCII_E as XML_Char,
            ASCII_N as XML_Char,
            ASCII_S as XML_Char,
            '\u{0}' as XML_Char,
        ];
        static mut notationPrefix: [XML_Char; 10] = [
            ASCII_N as XML_Char,
            ASCII_O as XML_Char,
            ASCII_T as XML_Char,
            ASCII_A as XML_Char,
            ASCII_T as XML_Char,
            ASCII_I as XML_Char,
            ASCII_O as XML_Char,
            ASCII_N as XML_Char,
            ASCII_LPAREN as XML_Char,
            '\u{0}' as XML_Char,
        ];
        static mut enumValueSep: [XML_Char; 2] = [ASCII_PIPE as XML_Char, '\u{0}' as XML_Char];
        static mut enumValueStart: [XML_Char; 2] = [ASCII_LPAREN as XML_Char, '\u{0}' as XML_Char];
        /* save one level of indirection */
        let dtd: *mut DTD = self.m_dtd;
        let mut eventPP: *mut *const c_char = 0 as *mut *const c_char;
        let mut eventEndPP: *mut *const c_char = 0 as *mut *const c_char;
        let mut quant: XML_Content_Quant = XML_CQUANT_NONE;
        if enc_type.is_internal() {
            eventPP = &mut (*self.m_openInternalEntities).internalEventPtr;
            eventEndPP = &mut (*self.m_openInternalEntities).internalEventEndPtr
        } else {
            eventPP = &mut self.m_eventPtr;
            eventEndPP = &mut self.m_eventEndPtr
        }
        let mut enc = self.encoding(enc_type);
        loop {
            let mut role: c_int = 0;
            let mut handleDefault: XML_Bool = XML_TRUE;
            *eventPP = s;
            *eventEndPP = next;
            if tok <= 0 {
                if haveMore as c_int != 0 && tok != super::xmltok::XML_TOK_INVALID {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                match tok {
                    super::xmltok::XML_TOK_INVALID => {
                        *eventPP = next;
                        return XML_ERROR_INVALID_TOKEN;
                    }
                    super::xmltok::XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
                    super::xmltok::XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,

                    -15 => tok = -tok,
                    super::xmltok::XML_TOK_NONE => {
                        /* for internal PE NOT referenced between declarations */
                        if enc_type.is_internal() && (*self.m_openInternalEntities).betweenDecl == 0 {
                            *nextPtr = s;
                            return XML_ERROR_NONE;
                        }
                        /* WFC: PE Between Declarations - must check that PE contains
                           complete markup, not only for external PEs, but also for
                           internal PEs if the reference occurs between declarations.
                        */
                        if self.m_isParamEntity as c_int != 0 || enc_type.is_internal() {
                            if self
                                .m_prologState
                                .handler
                                .expect("non-null function pointer")(
                                &mut self.m_prologState,
                                -(4),
                                end,
                                end,
                                &*enc,
                            ) == super::xmlrole::XML_ROLE_ERROR
                            {
                                return XML_ERROR_INCOMPLETE_PE;
                            }
                            *nextPtr = s;
                            return XML_ERROR_NONE;
                        }
                        /* XML_DTD */
                        return XML_ERROR_NO_ELEMENTS;
                    }
                    _ => {
                        tok = -tok; /* end of big switch */
                        next = end
                    }
                }
            } /* always initialize to NULL */
            role = self
                .m_prologState
                .handler
                .expect("non-null function pointer")(
                &mut self.m_prologState, tok, s, next, &*enc
            );
            match role {
                1 => {
                    let mut result: XML_Error = self.processXmlDecl(0, s, next);
                    if result != XML_ERROR_NONE {
                        return result;
                    }
                    enc = self.encoding(EncodingType::Normal);
                    handleDefault = XML_FALSE;
                    current_block = 1553878188884632965;
                }
                4 => {
                    if self.m_handlers.hasStartDoctypeDecl() {
                        self.m_doctypeName =
                        self.m_tempPool.storeString(enc, s, next);
                        if self.m_doctypeName.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        self.m_tempPool.start = self.m_tempPool.ptr;
                        self.m_doctypePubid = NULL as *const XML_Char;
                        handleDefault = XML_FALSE
                    }
                    self.m_doctypeSysid = NULL as *const XML_Char;
                    current_block = 1553878188884632965;
                }
                7 => {
                    let startHandlerRan = self.m_handlers.startDoctypeDecl(
                        self.m_doctypeName,
                        self.m_doctypeSysid,
                        self.m_doctypePubid,
                        1,
                    );

                    if startHandlerRan {
                        self.m_doctypeName = NULL as *const XML_Char;
                        self.m_tempPool.clear();
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                57 => {
                    let mut result_0: XML_Error = self.processXmlDecl(1, s, next);
                    if result_0 != XML_ERROR_NONE {
                        return result_0;
                    }
                    enc = self.encoding(EncodingType::Normal);
                    handleDefault = XML_FALSE;
                    current_block = 1553878188884632965;
                }
                6 => {
                    /* XML_DTD */
                    self.m_useForeignDTD = XML_FALSE;
                    self.m_declEntity = hash_insert!(
                        &mut (*dtd).paramEntities,
                        externalSubsetName.as_ptr(),
                        ENTITY
                    );
                    if self.m_declEntity.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    /* XML_DTD */
                    (*dtd).hasParamEntityRefs = XML_TRUE;
                    if self.m_handlers.hasStartDoctypeDecl() {
                        let mut pubId: *mut XML_Char = 0 as *mut XML_Char;
                        if (*enc).isPublicId(s, next, eventPP) == 0 {
                            return XML_ERROR_PUBLICID;
                        }
                        pubId = self.m_tempPool.storeString(
                            enc,
                            s.offset((*enc).minBytesPerChar() as isize),
                            next.offset(-((*enc).minBytesPerChar() as isize)),
                        );
                        if pubId.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        normalizePublicId(pubId);
                        self.m_tempPool.start = self.m_tempPool.ptr;
                        self.m_doctypePubid = pubId;
                        handleDefault = XML_FALSE;
                        current_block = 9007411418488376351;
                    } else {
                        current_block = 926243229934402080;
                    }
                }
                14 => {
                    current_block = 926243229934402080;
                }
                8 => {
                    if allowClosingDoctype != XML_TRUE {
                        /* Must not close doctype from within expanded parameter entities */
                        return XML_ERROR_INVALID_TOKEN;
                    }
                    if !self.m_doctypeName.is_null() {
                        self.m_handlers.startDoctypeDecl(
                            self.m_doctypeName,
                            self.m_doctypeSysid,
                            self.m_doctypePubid,
                            0,
                        );
                        self.m_tempPool.clear();
                        handleDefault = XML_FALSE
                    }
                    /* parser->m_doctypeSysid will be non-NULL in the case of a previous
                       XML_ROLE_DOCTYPE_SYSTEM_ID, even if parser->m_startDoctypeDeclHandler
                       was not set, indicating an external subset
                    */
                    if !self.m_doctypeSysid.is_null() || self.m_useForeignDTD as c_int != 0 {
                        let mut hadParamEntityRefs: XML_Bool = (*dtd).hasParamEntityRefs;
                        (*dtd).hasParamEntityRefs = XML_TRUE;
                        if self.m_paramEntityParsing != 0
                            && self.m_handlers.hasExternalEntityRef()
                        {
                            let mut entity = hash_insert!(
                                &mut (*dtd).paramEntities,
                                externalSubsetName.as_ptr(),
                                ENTITY
                            );
                            if entity.is_null() {
                                /* end of DTD - no need to update dtd->keepProcessing */
                                /* The external subset name "#" will have already been
                                 * inserted into the hash table at the start of the
                                 * external entity parsing, so no allocation will happen
                                 * and lookup() cannot fail.
                                 */
                                return XML_ERROR_NO_MEMORY;
                                /* LCOV_EXCL_LINE */
                            }
                            if self.m_useForeignDTD != 0 {
                                (*entity).base = self.m_curBase
                            }
                            (*dtd).paramEntityRead = XML_FALSE;
                            if self.m_handlers.externalEntityRef(
                                0 as *const XML_Char,
                                (*entity).base,
                                (*entity).systemId,
                                (*entity).publicId,
                            ) == Ok(0)
                            {
                                return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                            }
                            if (*dtd).paramEntityRead != 0 {
                                if (*dtd).standalone == 0 && self.m_handlers.notStandalone() == Ok(0) {
                                    return XML_ERROR_NOT_STANDALONE;
                                }
                            } else if self.m_doctypeSysid.is_null() {
                                (*dtd).hasParamEntityRefs = hadParamEntityRefs
                            }
                        }
                        self.m_useForeignDTD = XML_FALSE
                    }
                    /* if we didn't read the foreign DTD then this means that there
                       is no external subset and we must reset dtd->hasParamEntityRefs
                    */
                    /* XML_DTD */
                    let endHandlerRan = self.m_handlers.endDoctypeDecl();

                    if endHandlerRan {
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                2 => {
                    /* if there is no DOCTYPE declaration then now is the
                       last chance to read the foreign DTD
                    */
                    if self.m_useForeignDTD != 0 {
                        let mut hadParamEntityRefs_0: XML_Bool = (*dtd).hasParamEntityRefs;
                        (*dtd).hasParamEntityRefs = XML_TRUE;
                        if self.m_paramEntityParsing != 0 && self.m_handlers.hasExternalEntityRef() {
                            let mut entity_0 = hash_insert!(
                                &mut (*dtd).paramEntities,
                                externalSubsetName.as_ptr(),
                                ENTITY
                            );
                            if entity_0.is_null() {
                                return XML_ERROR_NO_MEMORY;
                            }
                            (*entity_0).base = self.m_curBase;
                            (*dtd).paramEntityRead = XML_FALSE;
                            if self.m_handlers.externalEntityRef(
                                0 as *const XML_Char,
                                (*entity_0).base,
                                (*entity_0).systemId,
                                (*entity_0).publicId,
                            ) == Ok(0)
                            {
                                return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                            }
                            if (*dtd).paramEntityRead != 0 {
                                if (*dtd).standalone == 0 && self.m_handlers.notStandalone() == Ok(0) {
                                    return XML_ERROR_NOT_STANDALONE;
                                }
                            } else {
                                /* end of DTD - no need to update dtd->keepProcessing */
                                /* if we didn't read the foreign DTD then this means that there
                                   is no external subset and we must reset dtd->hasParamEntityRefs
                                */
                                (*dtd).hasParamEntityRefs = hadParamEntityRefs_0
                            }
                        }
                    }
                    /* XML_DTD */
                    self.m_processor = Some(contentProcessor as Processor);
                    return contentProcessor(self, s, end, nextPtr);
                }
                34 => {
                    self.m_declElementType = self.getElementType(enc_type, s, next);
                    if self.m_declElementType.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    current_block = 6455255476181645667;
                }
                22 => {
                    self.m_declAttributeId = self.getAttributeId(enc_type, s, next);
                    if self.m_declAttributeId.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    self.m_declAttributeIsCdata = XML_FALSE;
                    self.m_declAttributeType = NULL as *const XML_Char;
                    self.m_declAttributeIsId = XML_FALSE;
                    current_block = 6455255476181645667;
                }
                23 => {
                    self.m_declAttributeIsCdata = XML_TRUE;
                    self.m_declAttributeType = atypeCDATA.as_ptr();
                    current_block = 6455255476181645667;
                }
                24 => {
                    self.m_declAttributeIsId = XML_TRUE;
                    self.m_declAttributeType = atypeID.as_ptr();
                    current_block = 6455255476181645667;
                }
                25 => {
                    self.m_declAttributeType = atypeIDREF.as_ptr();
                    current_block = 6455255476181645667;
                }
                26 => {
                    self.m_declAttributeType = atypeIDREFS.as_ptr();
                    current_block = 6455255476181645667;
                }
                27 => {
                    self.m_declAttributeType = atypeENTITY.as_ptr();
                    current_block = 6455255476181645667;
                }
                28 => {
                    self.m_declAttributeType = atypeENTITIES.as_ptr();
                    current_block = 6455255476181645667;
                }
                29 => {
                    self.m_declAttributeType = atypeNMTOKEN.as_ptr();
                    current_block = 6455255476181645667;
                }
                30 => {
                    self.m_declAttributeType = atypeNMTOKENS.as_ptr();
                    current_block = 6455255476181645667;
                }
                31 | 32 => {
                    if (*dtd).keepProcessing as c_int != 0 && self.m_handlers.hasAttlistDecl() {
                        let mut prefix: *const XML_Char = 0 as *const XML_Char;
                        if !self.m_declAttributeType.is_null() {
                            prefix = enumValueSep.as_ptr()
                        } else {
                            prefix = if role == super::xmlrole::XML_ROLE_ATTRIBUTE_NOTATION_VALUE {
                                notationPrefix.as_ptr()
                            } else {
                                enumValueStart.as_ptr()
                            }
                        }
                        if self.m_tempPool.appendString(prefix) {
                            return XML_ERROR_NO_MEMORY;
                        }
                        if self.m_tempPool.append(enc, s, next).is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        self.m_declAttributeType = self.m_tempPool.start;
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                35 | 36 => {
                    if (*dtd).keepProcessing != 0 {
                        if defineAttribute(
                            self.m_declElementType,
                            self.m_declAttributeId,
                            self.m_declAttributeIsCdata,
                            self.m_declAttributeIsId,
                            0 as *const XML_Char,
                        ) == 0
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        if self.m_handlers.hasAttlistDecl()
                            && !self.m_declAttributeType.is_null()
                        {
                            if *self.m_declAttributeType == ASCII_LPAREN as XML_Char
                                || *self.m_declAttributeType == ASCII_N as XML_Char
                                    && *self.m_declAttributeType.offset(1) == ASCII_O as XML_Char
                            {
                                /* Enumerated or Notation type */
                                if (if self.m_tempPool.ptr
                                    == self.m_tempPool.end as *mut XML_Char
                                    && self.m_tempPool.grow() == 0
                                {
                                    0
                                } else {
                                    let fresh32 = self.m_tempPool.ptr;
                                    self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                                    *fresh32 = ASCII_RPAREN as XML_Char;
                                    1
                                }) == 0
                                    || (if self.m_tempPool.ptr
                                        == self.m_tempPool.end as *mut XML_Char
                                        && self.m_tempPool.grow() == 0
                                    {
                                        0
                                    } else {
                                        let fresh33 = self.m_tempPool.ptr;
                                        self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                                        *fresh33 = '\u{0}' as XML_Char;
                                        1
                                    }) == 0
                                {
                                    return XML_ERROR_NO_MEMORY;
                                }
                                self.m_declAttributeType = self.m_tempPool.start;
                                self.m_tempPool.start = self.m_tempPool.ptr
                            }
                            *eventEndPP = s;
                            self.m_handlers.attlistDecl(
                                (*self.m_declElementType).name,
                                (*self.m_declAttributeId).name,
                                self.m_declAttributeType,
                                0 as *const XML_Char,
                                (role == super::xmlrole::XML_ROLE_REQUIRED_ATTRIBUTE_VALUE) as c_int,
                            );
                            self.m_tempPool.clear();
                            handleDefault = XML_FALSE
                        }
                    }
                    current_block = 1553878188884632965;
                }
                37 | 38 => {
                    if (*dtd).keepProcessing != 0 {
                        let mut attVal: *const XML_Char = 0 as *const XML_Char;
                        let mut result_1: XML_Error = storeAttributeValue(
                            self,
                            enc_type,
                            self.m_declAttributeIsCdata,
                            s.offset((*enc).minBytesPerChar() as isize),
                            next.offset(-((*enc).minBytesPerChar() as isize)),
                            &mut (*dtd).pool,
                        );
                        if result_1 as u64 != 0 {
                            return result_1;
                        }
                        attVal = (*dtd).pool.start;
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        /* ID attributes aren't allowed to have a default */
                        if defineAttribute(
                            self.m_declElementType,
                            self.m_declAttributeId,
                            self.m_declAttributeIsCdata,
                            XML_FALSE,
                            attVal,
                        ) == 0
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        if self.m_handlers.hasAttlistDecl()
                            && !self.m_declAttributeType.is_null()
                        {
                            if *self.m_declAttributeType == ASCII_LPAREN as XML_Char
                                || *self.m_declAttributeType == ASCII_N as XML_Char
                                    && *self.m_declAttributeType.offset(1) == ASCII_O as XML_Char
                            {
                                /* Enumerated or Notation type */
                                if (if self.m_tempPool.ptr
                                    == self.m_tempPool.end as *mut XML_Char
                                    && self.m_tempPool.grow() == 0
                                {
                                    0
                                } else {
                                    let fresh34 = self.m_tempPool.ptr;
                                    self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                                    *fresh34 = ASCII_RPAREN as XML_Char;
                                    1
                                }) == 0
                                    || (if self.m_tempPool.ptr
                                        == self.m_tempPool.end as *mut XML_Char
                                        && self.m_tempPool.grow() == 0
                                    {
                                        0
                                    } else {
                                        let fresh35 = self.m_tempPool.ptr;
                                        self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                                        *fresh35 = '\u{0}' as XML_Char;
                                        1
                                    }) == 0
                                {
                                    return XML_ERROR_NO_MEMORY;
                                }
                                self.m_declAttributeType = self.m_tempPool.start;
                                self.m_tempPool.start = self.m_tempPool.ptr
                            }
                            *eventEndPP = s;
                            self.m_handlers.attlistDecl(
                                (*self.m_declElementType).name,
                                (*self.m_declAttributeId).name,
                                self.m_declAttributeType,
                                attVal,
                                (role == super::xmlrole::XML_ROLE_FIXED_ATTRIBUTE_VALUE) as c_int,
                            );
                            self.m_tempPool.clear();
                            handleDefault = XML_FALSE
                        }
                    }
                    current_block = 1553878188884632965;
                }
                12 => {
                    if (*dtd).keepProcessing != 0 {
                        let mut result_2: XML_Error = storeEntityValue(
                            self,
                            enc_type,
                            s.offset((*enc).minBytesPerChar() as isize),
                            next.offset(-((*enc).minBytesPerChar() as isize)),
                        );
                        if !self.m_declEntity.is_null() {
                            (*self.m_declEntity).textPtr = (*dtd).entityValuePool.start;
                            (*self.m_declEntity).textLen = (*dtd)
                                .entityValuePool
                                .ptr
                                .wrapping_offset_from((*dtd).entityValuePool.start)
                                as c_int;
                            (*dtd).entityValuePool.start = (*dtd).entityValuePool.ptr;
                            if self.m_handlers.hasEntityDecl() {
                                *eventEndPP = s;
                                self.m_handlers.entityDecl(
                                    (*self.m_declEntity).name,
                                    (*self.m_declEntity).is_param as c_int,
                                    (*self.m_declEntity).textPtr,
                                    (*self.m_declEntity).textLen,
                                    self.m_curBase,
                                    0 as *const XML_Char,
                                    0 as *const XML_Char,
                                    0 as *const XML_Char,
                                );
                                handleDefault = XML_FALSE
                            }
                        } else {
                            (*dtd).entityValuePool.ptr = (*dtd).entityValuePool.start
                        }
                        if result_2 != XML_ERROR_NONE {
                            return result_2;
                        }
                    }
                    current_block = 1553878188884632965;
                }
                5 => {
                    self.m_useForeignDTD = XML_FALSE;
                    /* XML_DTD */
                    (*dtd).hasParamEntityRefs = XML_TRUE;
                    if self.m_handlers.hasStartDoctypeDecl() {
                        self.m_doctypeSysid = self.m_tempPool.storeString(
                            enc,
                            s.offset((*enc).minBytesPerChar() as isize),
                            next.offset(-((*enc).minBytesPerChar() as isize)),
                        );
                        if self.m_doctypeSysid.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        self.m_tempPool.start = self.m_tempPool.ptr;
                        handleDefault = XML_FALSE
                    } else {
                        /* use externalSubsetName to make parser->m_doctypeSysid non-NULL
                        for the case where no parser->m_startDoctypeDeclHandler is set */
                        self.m_doctypeSysid = externalSubsetName.as_ptr()
                    }
                    /* XML_DTD */
                    if (*dtd).standalone == 0
                        && self.m_paramEntityParsing as u64 == 0
                        && self.m_handlers.notStandalone() == Ok(0)
                    {
                        return XML_ERROR_NOT_STANDALONE;
                    }
                    /* XML_DTD */
                    if self.m_declEntity.is_null() {
                        self.m_declEntity = hash_insert!(
                            &mut (*dtd).paramEntities,
                            externalSubsetName.as_ptr(),
                            ENTITY
                        );
                        if self.m_declEntity.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*self.m_declEntity).publicId = NULL as *const XML_Char
                    }
                    current_block = 15307276507984219638;
                }
                13 => {
                    current_block = 15307276507984219638;
                }
                15 => {
                    if (*dtd).keepProcessing as c_int != 0
                        && !self.m_declEntity.is_null()
                        && self.m_handlers.hasEntityDecl()
                    {
                        *eventEndPP = s;
                        self.m_handlers.entityDecl(
                            (*self.m_declEntity).name,
                            (*self.m_declEntity).is_param as c_int,
                            0 as *const XML_Char,
                            0,
                            (*self.m_declEntity).base,
                            (*self.m_declEntity).systemId,
                            (*self.m_declEntity).publicId,
                            0 as *const XML_Char,
                        );
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                16 => {
                    if (*dtd).keepProcessing as c_int != 0 && !self.m_declEntity.is_null() {
                        (*self.m_declEntity).notation =
                        (*dtd).pool.storeString(enc, s, next);
                        if (*self.m_declEntity).notation.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        if self.m_handlers.hasUnparsedEntityDecl() {
                            *eventEndPP = s;
                            self.m_handlers.unparsedEntityDecl(
                                (*self.m_declEntity).name,
                                (*self.m_declEntity).base,
                                (*self.m_declEntity).systemId,
                                (*self.m_declEntity).publicId,
                                (*self.m_declEntity).notation,
                            );
                            handleDefault = XML_FALSE
                        } else if self.m_handlers.hasEntityDecl() {
                            *eventEndPP = s;
                            self.m_handlers.entityDecl(
                                (*self.m_declEntity).name,
                                0,
                                0 as *const XML_Char,
                                0,
                                (*self.m_declEntity).base,
                                (*self.m_declEntity).systemId,
                                (*self.m_declEntity).publicId,
                                (*self.m_declEntity).notation,
                            );
                            handleDefault = XML_FALSE
                        }
                    }
                    current_block = 1553878188884632965;
                }
                9 => {
                    if (*enc).predefinedEntityName(s, next) != 0 {
                        self.m_declEntity = NULL as *mut ENTITY
                    } else if (*dtd).keepProcessing != 0 {
                        let mut name: *const XML_Char = (*dtd).pool.storeString(enc, s, next);
                        if name.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        self.m_declEntity = hash_insert!(
                            &mut (*dtd).generalEntities,
                            name,
                            ENTITY
                        );
                        if self.m_declEntity.is_null() {
                            // FIXME: this never happens in Rust, it just panics
                            return XML_ERROR_NO_MEMORY;
                        }
                        if (*self.m_declEntity).name != name {
                            (*dtd).pool.ptr = (*dtd).pool.start;
                            self.m_declEntity = NULL as *mut ENTITY
                        } else {
                            (*dtd).pool.start = (*dtd).pool.ptr;
                            (*self.m_declEntity).publicId = NULL as *const XML_Char;
                            (*self.m_declEntity).is_param = XML_FALSE;
                            /* if we have a parent parser or are reading an internal parameter
                               entity, then the entity declaration is not considered "internal"
                            */
                            (*self.m_declEntity).is_internal =
                                !(!self.m_parentParser.is_null()
                                    || !self.m_openInternalEntities.is_null())
                                    as XML_Bool;
                            if self.m_handlers.hasEntityDecl() {
                                handleDefault = XML_FALSE
                            }
                        }
                    } else {
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        self.m_declEntity = NULL as *mut ENTITY
                    }
                    current_block = 1553878188884632965;
                }
                10 => {
                    if (*dtd).keepProcessing != 0 {
                        let mut name_0: *const XML_Char =
                        (*dtd).pool.storeString(enc, s, next);
                        if name_0.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        self.m_declEntity = hash_insert!(
                            &mut (*dtd).paramEntities,
                            name_0,
                            ENTITY
                        );
                        if self.m_declEntity.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        if (*self.m_declEntity).name != name_0 {
                            (*dtd).pool.ptr = (*dtd).pool.start;
                            self.m_declEntity = NULL as *mut ENTITY
                        } else {
                            (*dtd).pool.start = (*dtd).pool.ptr;
                            (*self.m_declEntity).publicId = NULL as *const XML_Char;
                            (*self.m_declEntity).is_param = XML_TRUE;
                            /* if we have a parent parser or are reading an internal parameter
                               entity, then the entity declaration is not considered "internal"
                            */
                            (*self.m_declEntity).is_internal =
                                !(!self.m_parentParser.is_null()
                                    || !self.m_openInternalEntities.is_null())
                                    as XML_Bool;
                            if self.m_handlers.hasEntityDecl() {
                                handleDefault = XML_FALSE
                            }
                        }
                    } else {
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        self.m_declEntity = NULL as *mut ENTITY
                    }
                    current_block = 1553878188884632965;
                }
                18 => {
                    self.m_declNotationPublicId = NULL as *const XML_Char;
                    self.m_declNotationName = NULL as *const XML_Char;
                    if self.m_handlers.hasNotationDecl() {
                        self.m_declNotationName =
                        self.m_tempPool.storeString(enc, s, next);
                        if self.m_declNotationName.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        self.m_tempPool.start = self.m_tempPool.ptr;
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                21 => {
                    if (*enc).isPublicId(s, next, eventPP) == 0 {
                        return XML_ERROR_PUBLICID;
                    }
                    if !self.m_declNotationName.is_null() {
                        /* means m_notationDeclHandler != NULL */
                        let mut tem_0: *mut XML_Char = self.m_tempPool.storeString(
                            enc,
                            s.offset((*enc).minBytesPerChar() as isize),
                            next.offset(-((*enc).minBytesPerChar() as isize)),
                        );
                        if tem_0.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        normalizePublicId(tem_0);
                        self.m_declNotationPublicId = tem_0;
                        self.m_tempPool.start = self.m_tempPool.ptr;
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                19 => {
                    if !self.m_declNotationName.is_null()
                        && self.m_handlers.hasNotationDecl()
                    {
                        let mut systemId: *const XML_Char = self.m_tempPool.storeString(
                            enc,
                            s.offset((*enc).minBytesPerChar() as isize),
                            next.offset(-((*enc).minBytesPerChar() as isize)),
                        );
                        if systemId.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        *eventEndPP = s;
                        self.m_handlers.notationDecl(
                            self.m_declNotationName,
                            self.m_curBase,
                            systemId,
                            self.m_declNotationPublicId,
                        );
                        handleDefault = XML_FALSE
                    }
                    self.m_tempPool.clear();
                    current_block = 1553878188884632965;
                }
                20 => {
                    if !self.m_declNotationPublicId.is_null()
                        && self.m_handlers.hasNotationDecl()
                    {
                        *eventEndPP = s;
                        self.m_handlers.notationDecl(
                            self.m_declNotationName,
                            self.m_curBase,
                            0 as *const XML_Char,
                            self.m_declNotationPublicId,
                        );
                        handleDefault = XML_FALSE
                    }
                    self.m_tempPool.clear();
                    current_block = 1553878188884632965;
                }
                -1 => {
                    match tok {
                        super::xmltok::XML_TOK_PARAM_ENTITY_REF => {
                            /* PE references in internal subset are
                            not allowed within declarations. */
                            return XML_ERROR_PARAM_ENTITY_REF;
                        }
                        super::xmltok::XML_TOK_XML_DECL => return XML_ERROR_MISPLACED_XML_PI,
                        _ => return XML_ERROR_SYNTAX,
                    }
                }
                58 => {
                    let mut result_3: XML_Error = XML_ERROR_NONE;
                    if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, s, next);
                    }
                    handleDefault = XML_FALSE;
                    result_3 = doIgnoreSection(self, enc_type, &mut next, end, nextPtr, haveMore);
                    if result_3 != XML_ERROR_NONE {
                        return result_3;
                    } else {
                        if next.is_null() {
                            self.m_processor = Some(ignoreSectionProcessor as Processor);
                            return result_3;
                        }
                    }
                    current_block = 1553878188884632965;
                }
                44 => {
                    /* XML_DTD */
                    if self.m_prologState.level >= self.m_groupSize {
                        if self.m_groupSize != 0 {
                            self.m_groupSize = self.m_groupSize.wrapping_mul(2u32);
                            let new_connector: *mut c_char = REALLOC!(
                                self.m_groupConnector as *mut c_void,
                                self.m_groupSize as size_t
                            ) as *mut c_char;
                            if new_connector.is_null() {
                                self.m_groupSize = self.m_groupSize.wrapping_div(2u32);
                                return XML_ERROR_NO_MEMORY;
                            }
                            self.m_groupConnector = new_connector;
                            if !(*dtd).scaffIndex.is_null() {
                                let new_scaff_index: *mut c_int = REALLOC!(
                                    (*dtd).scaffIndex as *mut c_void,
                                    (self.m_groupSize as c_ulong)
                                        .wrapping_mul(::std::mem::size_of::<c_int>() as c_ulong)
                                )
                                    as *mut c_int;
                                if new_scaff_index.is_null() {
                                    return XML_ERROR_NO_MEMORY;
                                }
                                (*dtd).scaffIndex = new_scaff_index
                            }
                        } else {
                            self.m_groupSize = 32u32;
                            self.m_groupConnector =
                                MALLOC!(self.m_groupSize as size_t) as *mut c_char;
                            if self.m_groupConnector.is_null() {
                                self.m_groupSize = 0u32;
                                return XML_ERROR_NO_MEMORY;
                            }
                        }
                    }
                    *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize) = 0i8;
                    if (*dtd).in_eldecl != 0 {
                        let mut myindex: c_int = self.nextScaffoldPart();
                        if myindex < 0 {
                            return XML_ERROR_NO_MEMORY;
                        }
                        if !(*dtd).scaffIndex.is_null() {
                        } else {
                            __assert_fail(b"dtd->scaffIndex != NULL\x00".as_ptr() as *const c_char,

                                          b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/lib/xmlparse.c\x00".as_ptr() as *const c_char,
                                          4790u32,
                                          (*::std::mem::transmute::<&[u8; 136],
                                                                    &[c_char; 136]>(b"enum XML_Error doProlog(XML_Parser, const ENCODING *, const char *, const char *, int, const char *, const char **, XML_Bool, XML_Bool)\x00")).as_ptr());
                        }
                        *(*dtd).scaffIndex.offset((*dtd).scaffLevel as isize) = myindex;
                        (*dtd).scaffLevel += 1;
                        (*(*dtd).scaffold.offset(myindex as isize)).type_0 = XML_CTYPE_SEQ;
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = XML_FALSE
                        }
                    }
                    current_block = 1553878188884632965;
                }
                50 => {
                    if *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize)
                        == ASCII_PIPE
                    {
                        return XML_ERROR_SYNTAX;
                    }
                    *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize) = ASCII_COMMA;
                    if (*dtd).in_eldecl as c_int != 0 && self.m_handlers.hasElementDecl() {
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                49 => {
                    if *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize)
                        == ASCII_COMMA
                    {
                        return XML_ERROR_SYNTAX;
                    }
                    if (*dtd).in_eldecl as c_int != 0
                        && *self
                            .m_groupConnector
                            .offset(self.m_prologState.level as isize)
                            == 0
                        && (*(*dtd).scaffold.offset(
                            *(*dtd).scaffIndex.offset(((*dtd).scaffLevel - 1) as isize) as isize,
                        ))
                        .type_0
                            != XML_CTYPE_MIXED
                    {
                        (*(*dtd).scaffold.offset(
                            *(*dtd).scaffIndex.offset(((*dtd).scaffLevel - 1) as isize) as isize,
                        ))
                        .type_0 = XML_CTYPE_CHOICE;
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = XML_FALSE
                        }
                    }
                    *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize) = ASCII_PIPE;
                    current_block = 1553878188884632965;
                }
                60 | 59 => {
                    (*dtd).hasParamEntityRefs = XML_TRUE;
                    if self.m_paramEntityParsing as u64 == 0 {
                        (*dtd).keepProcessing = (*dtd).standalone;
                        current_block = 10770532911212200937;
                    } else {
                        let mut name_1: *const XML_Char = 0 as *const XML_Char;
                        let mut entity_1: *mut ENTITY = 0 as *mut ENTITY;
                        name_1 = (*dtd).pool.storeString(
                            enc,
                            s.offset((*enc).minBytesPerChar() as isize),
                            next.offset(-((*enc).minBytesPerChar() as isize)),
                        );
                        if name_1.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        entity_1 = hash_lookup!((*dtd).paramEntities, name_1);
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        /* first, determine if a check for an existing declaration is needed;
                           if yes, check that the entity exists, and that it is internal,
                           otherwise call the skipped entity handler
                        */
                        if self.m_prologState.documentEntity != 0
                            && (if (*dtd).standalone as c_int != 0 {
                                self.m_openInternalEntities.is_null() as c_int
                            } else {
                                ((*dtd).hasParamEntityRefs == 0) as c_int
                            }) != 0
                        {
                            if entity_1.is_null() {
                                return XML_ERROR_UNDEFINED_ENTITY;
                            } else {
                                if (*entity_1).is_internal == 0 {
                                    /* It's hard to exhaustively search the code to be sure,
                                     * but there doesn't seem to be a way of executing the
                                     * following line.  There are two cases:
                                     *
                                     * If 'standalone' is false, the DTD must have no
                                     * parameter entities or we wouldn't have passed the outer
                                     * 'if' statement.  That measn the only entity in the hash
                                     * table is the external subset name "#" which cannot be
                                     * given as a parameter entity name in XML syntax, so the
                                     * lookup must have returned NULL and we don't even reach
                                     * the test for an internal entity.
                                     *
                                     * If 'standalone' is true, it does not seem to be
                                     * possible to create entities taking this code path that
                                     * are not internal entities, so fail the test above.
                                     *
                                     * Because this analysis is very uncertain, the code is
                                     * being left in place and merely removed from the
                                     * coverage test statistics.
                                     */
                                    return XML_ERROR_ENTITY_DECLARED_IN_PE;
                                    /* LCOV_EXCL_LINE */
                                }
                            }
                            current_block = 13351260019855268589;
                        } else if entity_1.is_null() {
                            (*dtd).keepProcessing = (*dtd).standalone;
                            /* cannot report skipped entities in declarations */
                            if role == super::xmlrole::XML_ROLE_PARAM_ENTITY_REF
                                && self.m_handlers.hasSkippedEntity()
                            {
                                self.m_handlers.skippedEntity(name_1, 1);
                                handleDefault = XML_FALSE
                            }
                            current_block = 1553878188884632965;
                        } else {
                            current_block = 13351260019855268589;
                        }
                        match current_block {
                            1553878188884632965 => {}
                            _ => {
                                if (*entity_1).open != 0 {
                                    return XML_ERROR_RECURSIVE_ENTITY_REF;
                                }
                                if !(*entity_1).textPtr.is_null() {
                                    let mut result_4: XML_Error = XML_ERROR_NONE;
                                    let mut betweenDecl: XML_Bool =
                                        if role == super::xmlrole::XML_ROLE_PARAM_ENTITY_REF {
                                            XML_TRUE
                                        } else {
                                            XML_FALSE
                                        };
                                    result_4 = self.processInternalEntity(entity_1, betweenDecl);
                                    if result_4 != XML_ERROR_NONE {
                                        return result_4;
                                    }
                                    handleDefault = XML_FALSE;
                                    current_block = 1553878188884632965;
                                } else if self.m_handlers.hasExternalEntityRef() {
                                    (*dtd).paramEntityRead = XML_FALSE;
                                    (*entity_1).open = XML_TRUE;
                                    let entity_1_name = if cfg!(feature = "mozilla") {
                                        (*entity_1).name
                                    } else {
                                        0 as *const XML_Char
                                    };
                                    if self.m_handlers.externalEntityRef(
                                        entity_1_name,
                                        (*entity_1).base,
                                        (*entity_1).systemId,
                                        (*entity_1).publicId,
                                    ) == Ok(0)
                                    {
                                        (*entity_1).open = XML_FALSE;
                                        return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                    }
                                    (*entity_1).open = XML_FALSE;
                                    handleDefault = XML_FALSE;
                                    if (*dtd).paramEntityRead == 0 {
                                        (*dtd).keepProcessing = (*dtd).standalone;
                                        current_block = 1553878188884632965;
                                    } else {
                                        current_block = 10770532911212200937;
                                    }
                                } else {
                                    (*dtd).keepProcessing = (*dtd).standalone;
                                    current_block = 1553878188884632965;
                                }
                            }
                        }
                    }
                    match current_block {
                        1553878188884632965 => {}
                        _ => {
                            /* XML_DTD */
                            if (*dtd).standalone == 0 && self.m_handlers.notStandalone() == Ok(0) {
                                return XML_ERROR_NOT_STANDALONE;
                            }
                            current_block = 1553878188884632965;
                        }
                    }
                }
                40 => {
                    /* Element declaration stuff */
                    if self.m_handlers.hasElementDecl() {
                        self.m_declElementType = self.getElementType(enc_type, s, next);
                        if self.m_declElementType.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*dtd).scaffLevel = 0;
                        (*dtd).scaffCount = 0;
                        (*dtd).in_eldecl = XML_TRUE;
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                41 | 42 => {
                    if (*dtd).in_eldecl != 0 {
                        if self.m_handlers.hasElementDecl() {
                            let mut content: *mut XML_Content =
                                MALLOC!(::std::mem::size_of::<XML_Content>() as c_ulong)
                                    as *mut XML_Content;
                            if content.is_null() {
                                return XML_ERROR_NO_MEMORY;
                            }
                            (*content).quant = XML_CQUANT_NONE;
                            (*content).name = NULL as *mut XML_Char;
                            (*content).numchildren = 0;
                            (*content).children = NULL as *mut XML_Content;
                            (*content).type_0 = if role == super::xmlrole::XML_ROLE_CONTENT_ANY {
                                XML_CTYPE_ANY as c_int
                            } else {
                                XML_CTYPE_EMPTY as c_int
                            } as XML_Content_Type;
                            *eventEndPP = s;
                            self.m_handlers.elementDecl((*self.m_declElementType).name, content);
                            handleDefault = XML_FALSE
                        }
                        (*dtd).in_eldecl = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                43 => {
                    if (*dtd).in_eldecl != 0 {
                        (*(*dtd).scaffold.offset(
                            *(*dtd).scaffIndex.offset(((*dtd).scaffLevel - 1) as isize) as isize,
                        ))
                        .type_0 = XML_CTYPE_MIXED;
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = XML_FALSE
                        }
                    }
                    current_block = 1553878188884632965;
                }
                51 => {
                    quant = XML_CQUANT_NONE;
                    current_block = 4542134034984465527;
                }
                53 => {
                    quant = XML_CQUANT_OPT;
                    current_block = 4542134034984465527;
                }
                52 => {
                    quant = XML_CQUANT_REP;
                    current_block = 4542134034984465527;
                }
                54 => {
                    quant = XML_CQUANT_PLUS;
                    current_block = 4542134034984465527;
                }
                45 => {
                    quant = XML_CQUANT_NONE;
                    current_block = 7739131043814808354;
                }
                47 => {
                    quant = XML_CQUANT_OPT;
                    current_block = 7739131043814808354;
                }
                46 => {
                    quant = XML_CQUANT_REP;
                    current_block = 7739131043814808354;
                }
                48 => {
                    quant = XML_CQUANT_PLUS;
                    current_block = 7739131043814808354;
                }
                55 => {
                    /* End element declaration stuff */
                    if reportProcessingInstruction(self, enc_type, s, next) == 0 {
                        return XML_ERROR_NO_MEMORY;
                    }
                    handleDefault = XML_FALSE;
                    current_block = 1553878188884632965;
                }
                56 => {
                    if reportComment(self, enc_type, s, next) == 0 {
                        return XML_ERROR_NO_MEMORY;
                    }
                    handleDefault = XML_FALSE;
                    current_block = 1553878188884632965;
                }
                0 => {
                    match tok {
                        super::xmltok::XML_TOK_BOM => handleDefault = XML_FALSE,
                        _ => {}
                    }
                    current_block = 1553878188884632965;
                }
                3 => {
                    if self.m_handlers.hasStartDoctypeDecl() {
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                11 => {
                    if (*dtd).keepProcessing as c_int != 0 && self.m_handlers.hasEntityDecl() {
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                17 => {
                    if self.m_handlers.hasNotationDecl() {
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                33 => {
                    if (*dtd).keepProcessing as c_int != 0 && self.m_handlers.hasAttlistDecl() {
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                39 => {
                    if self.m_handlers.hasElementDecl() {
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                _ => {
                    current_block = 1553878188884632965;
                }
            }
            match current_block {
                926243229934402080 =>
                /* fall through */
                {
                    if (*enc).isPublicId(s, next, eventPP) == 0 {
                        return XML_ERROR_PUBLICID;
                    }
                    current_block = 9007411418488376351;
                }
                15307276507984219638 =>
                /* XML_DTD */
                /* fall through */
                {
                    if (*dtd).keepProcessing as c_int != 0 && !self.m_declEntity.is_null() {
                        (*self.m_declEntity).systemId = (*dtd).pool.storeString(
                            enc,
                            s.offset((*enc).minBytesPerChar() as isize),
                            next.offset(-((*enc).minBytesPerChar() as isize)),
                        );
                        if (*self.m_declEntity).systemId.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*self.m_declEntity).base = self.m_curBase;
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        /* Don't suppress the default handler if we fell through from
                         * the XML_ROLE_DOCTYPE_SYSTEM_ID case.
                         */
                        if self.m_handlers.hasEntityDecl()
                            && role == super::xmlrole::XML_ROLE_ENTITY_SYSTEM_ID
                        {
                            handleDefault = XML_FALSE
                        }
                    }
                    current_block = 1553878188884632965;
                }
                6455255476181645667 => {
                    if (*dtd).keepProcessing as c_int != 0 && self.m_handlers.hasAttlistDecl() {
                        handleDefault = XML_FALSE
                    }
                    current_block = 1553878188884632965;
                }
                4542134034984465527 => {
                    if (*dtd).in_eldecl != 0 {
                        let mut el: *mut ELEMENT_TYPE = 0 as *mut ELEMENT_TYPE;
                        let mut name_2: *const XML_Char = 0 as *const XML_Char;
                        let mut nameLen: c_int = 0;
                        let mut nxt: *const c_char = if quant == XML_CQUANT_NONE {
                            next
                        } else {
                            next.offset(-((*enc).minBytesPerChar() as isize))
                        };
                        let mut myindex_0: c_int = self.nextScaffoldPart();
                        if myindex_0 < 0 {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*(*dtd).scaffold.offset(myindex_0 as isize)).type_0 = XML_CTYPE_NAME;
                        (*(*dtd).scaffold.offset(myindex_0 as isize)).quant = quant;
                        el = self.getElementType(enc_type, s, nxt);
                        if el.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        name_2 = (*el).name;
                        let ref mut fresh36 = (*(*dtd).scaffold.offset(myindex_0 as isize)).name;
                        *fresh36 = name_2;
                        nameLen = 0;
                        loop {
                            let fresh37 = nameLen;
                            nameLen = nameLen + 1;
                            if !(*name_2.offset(fresh37 as isize) != 0) {
                                break;
                            }
                        }
                        (*dtd).contentStringLen =
                            (*dtd).contentStringLen.wrapping_add(nameLen as c_uint);
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = XML_FALSE
                        }
                    }
                    current_block = 1553878188884632965;
                }
                7739131043814808354 => {
                    if (*dtd).in_eldecl != 0 {
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = XML_FALSE
                        }
                        (*dtd).scaffLevel -= 1;
                        (*(*dtd)
                            .scaffold
                            .offset(*(*dtd).scaffIndex.offset((*dtd).scaffLevel as isize) as isize))
                        .quant = quant;
                        if (*dtd).scaffLevel == 0 {
                            if handleDefault == 0 {
                                let mut model: *mut XML_Content = self.build_model();
                                if model.is_null() {
                                    return XML_ERROR_NO_MEMORY;
                                }
                                *eventEndPP = s;
                                self.m_handlers.elementDecl((*self.m_declElementType).name, model);
                            }
                            (*dtd).in_eldecl = XML_FALSE;
                            (*dtd).contentStringLen = 0
                        }
                    }
                    current_block = 1553878188884632965;
                }
                _ => {}
            }
            match current_block {
                9007411418488376351 => {
                    if (*dtd).keepProcessing as c_int != 0 && !self.m_declEntity.is_null() {
                        let mut tem: *mut XML_Char = (*dtd).pool.storeString(
                            enc,
                            s.offset((*enc).minBytesPerChar() as isize),
                            next.offset(-((*enc).minBytesPerChar() as isize)),
                        );
                        if tem.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        normalizePublicId(tem);
                        (*self.m_declEntity).publicId = tem;
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        /* Don't suppress the default handler if we fell through from
                         * the XML_ROLE_DOCTYPE_PUBLIC_ID case.
                         */
                        if self.m_handlers.hasEntityDecl()
                            && role == super::xmlrole::XML_ROLE_ENTITY_PUBLIC_ID
                        {
                            handleDefault = XML_FALSE
                        }
                    }
                }
                _ => {}
            }
            /* not XML_DTD */
            /* XML_DTD */
            if handleDefault as c_int != 0 && self.m_handlers.hasDefault() {
                reportDefault(self, enc_type, s, next);
            }
            match self.m_parsingStatus.parsing {
                3 => {
                    *nextPtr = next;
                    return XML_ERROR_NONE;
                }
                2 => return XML_ERROR_ABORTED,
                _ => {
                    s = next;
                    tok = (*enc).xmlTok(XML_PROLOG_STATE, s, end, &mut next)
                }
            }
        }
        /* not reached */
    }
}
/* XML_DTD */

unsafe extern "C" fn epilogProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    (*parser).m_processor = Some(epilogProcessor as Processor);
    (*parser).m_eventPtr = s;
    loop {
        let mut next: *const c_char = NULL as *const c_char;
        let mut tok: c_int = (*(*parser).m_encoding).xmlTok(XML_PROLOG_STATE, s, end, &mut next);
        (*parser).m_eventEndPtr = next;
        match tok {
            -15 => {
                /* report partial linebreak - it might be the last token */
                if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, EncodingType::Normal, s, next);
                    if (*parser).m_parsingStatus.parsing == XML_FINISHED {
                        return XML_ERROR_ABORTED;
                    }
                }
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            super::xmltok::XML_TOK_NONE => {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            super::xmltok::XML_TOK_PROLOG_S => {
                if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, EncodingType::Normal, s, next);
                }
            }
            super::xmltok::XML_TOK_PI => {
                if reportProcessingInstruction(parser, EncodingType::Normal, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            super::xmltok::XML_TOK_COMMENT => {
                if reportComment(parser, EncodingType::Normal, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            super::xmltok::XML_TOK_INVALID => {
                (*parser).m_eventPtr = next;
                return XML_ERROR_INVALID_TOKEN;
            }
            super::xmltok::XML_TOK_PARTIAL => {
                if (*parser).m_parsingStatus.finalBuffer == 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_UNCLOSED_TOKEN;
            }
            super::xmltok::XML_TOK_PARTIAL_CHAR => {
                if (*parser).m_parsingStatus.finalBuffer == 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_PARTIAL_CHAR;
            }
            _ => return XML_ERROR_JUNK_AFTER_DOC_ELEMENT,
        }
        s = next;
        (*parser).m_eventPtr = s;
        match (*parser).m_parsingStatus.parsing {
            3 => {
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            2 => return XML_ERROR_ABORTED,
            _ => {}
        }
    }
}

impl XML_ParserStruct {
    unsafe fn processInternalEntity(
        &mut self,
        mut entity: *mut ENTITY,
        mut betweenDecl: XML_Bool,
    ) -> XML_Error {
        let mut textStart: *const c_char = 0 as *const c_char;
        let mut textEnd: *const c_char = 0 as *const c_char;
        let mut next: *const c_char = 0 as *const c_char;
        let mut result: XML_Error = XML_ERROR_NONE;
        let mut openEntity: *mut OPEN_INTERNAL_ENTITY = 0 as *mut OPEN_INTERNAL_ENTITY;
        if !self.m_freeInternalEntities.is_null() {
            openEntity = self.m_freeInternalEntities;
            self.m_freeInternalEntities = (*openEntity).next
        } else {
            openEntity = MALLOC!(
                ::std::mem::size_of::<OPEN_INTERNAL_ENTITY>() as c_ulong
            ) as *mut OPEN_INTERNAL_ENTITY;
            if openEntity.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
        }
        (*entity).open = XML_TRUE;
        (*entity).processed = 0;
        (*openEntity).next = self.m_openInternalEntities;
        self.m_openInternalEntities = openEntity;
        (*openEntity).entity = entity;
        (*openEntity).startTagLevel = self.m_tagLevel;
        (*openEntity).betweenDecl = betweenDecl;
        (*openEntity).internalEventPtr = NULL as *const c_char;
        (*openEntity).internalEventEndPtr = NULL as *const c_char;
        textStart = (*entity).textPtr as *mut c_char;
        textEnd = (*entity).textPtr.offset((*entity).textLen as isize) as *mut c_char;
        /* Set a safe default value in case 'next' does not get set */
        next = textStart;
        if (*entity).is_param != 0 {
            let mut tok: c_int =
                (*self.m_internalEncoding).xmlTok(XML_PROLOG_STATE, textStart, textEnd, &mut next);
            result = self.doProlog(
                EncodingType::Internal,
                textStart,
                textEnd,
                tok,
                next,
                &mut next,
                XML_FALSE,
                XML_FALSE,
            )
        } else {
            /* XML_DTD */
            result = self.doContent(
                self.m_tagLevel,
                EncodingType::Internal,
                textStart,
                textEnd,
                &mut next,
                XML_FALSE,
            )
        }
        if result == XML_ERROR_NONE {
            if textEnd != next && self.m_parsingStatus.parsing == XML_SUSPENDED {
                (*entity).processed = next.wrapping_offset_from(textStart) as c_int;
                self.m_processor = Some(internalEntityProcessor as Processor)
            } else {
                (*entity).open = XML_FALSE;
                if cfg!(feature = "mozilla") {
                    if self.m_openInternalEntities == openEntity {
                        self.m_openInternalEntities = (*openEntity).next;
                    } else {
                        /* openEntity should be closed, but it contains an inner entity that is
                           still open. Remove openEntity from the openInternalEntities linked
                           list by looking for the inner entity in the list that links to
                           openEntity and fixing up its 'next' member
                        */
                        let mut innerOpenEntity = self.m_openInternalEntities;
                        loop {
                            if (*innerOpenEntity).next == openEntity {
                                (*innerOpenEntity).next = (*openEntity).next;
                                break;
                            }
                            innerOpenEntity = (*innerOpenEntity).next;
                            if innerOpenEntity.is_null() {
                                break;
                            }
                        }
                    }
                } else {
                    self.m_openInternalEntities = (*openEntity).next;
                }
                /* put openEntity back in list of free instances */
                (*openEntity).next = self.m_freeInternalEntities;
                self.m_freeInternalEntities = openEntity
            }
        }
        return result;
    }
}

unsafe extern "C" fn internalEntityProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut entity: *mut ENTITY = 0 as *mut ENTITY;
    let mut textStart: *const c_char = 0 as *const c_char;
    let mut textEnd: *const c_char = 0 as *const c_char;
    let mut next: *const c_char = 0 as *const c_char;
    let mut result: XML_Error = XML_ERROR_NONE;
    let mut openEntity: *mut OPEN_INTERNAL_ENTITY = (*parser).m_openInternalEntities;
    if openEntity.is_null() {
        return XML_ERROR_UNEXPECTED_STATE;
    }
    entity = (*openEntity).entity;
    textStart = ((*entity).textPtr as *mut c_char).offset((*entity).processed as isize);
    textEnd = (*entity).textPtr.offset((*entity).textLen as isize) as *mut c_char;
    /* Set a safe default value in case 'next' does not get set */
    next = textStart;
    if (*entity).is_param != 0 {
        let mut tok: c_int =
            (*(*parser).m_internalEncoding).xmlTok(XML_PROLOG_STATE, textStart, textEnd, &mut next);
        result = (*parser).doProlog(
            EncodingType::Internal,
            textStart,
            textEnd,
            tok,
            next,
            &mut next,
            XML_FALSE,
            XML_TRUE,
        )
    } else {
        /* XML_DTD */
        result = (*parser).doContent(
            (*openEntity).startTagLevel,
            EncodingType::Internal,
            textStart,
            textEnd,
            &mut next,
            XML_FALSE,
        )
    }
    if result != XML_ERROR_NONE {
        return result;
    } else {
        if textEnd != next && (*parser).m_parsingStatus.parsing == XML_SUSPENDED {
            (*entity).processed =
                next.wrapping_offset_from((*entity).textPtr as *mut c_char) as c_int;
            return result;
        } else {
            (*entity).open = XML_FALSE;
            (*parser).m_openInternalEntities = (*openEntity).next;
            /* put openEntity back in list of free instances */
            (*openEntity).next = (*parser).m_freeInternalEntities;
            (*parser).m_freeInternalEntities = openEntity
        }
    }
    if (*entity).is_param != 0 {
        let mut tok_0: c_int = 0;
        (*parser).m_processor = Some(prologProcessor as Processor);
        tok_0 = (*(*parser).m_encoding).xmlTok(XML_PROLOG_STATE, s, end, &mut next);
        (*parser).doProlog(
            EncodingType::Normal,
            s,
            end,
            tok_0,
            next,
            nextPtr,
            ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
            XML_TRUE,
        )
    } else {
        /* XML_DTD */
        (*parser).m_processor = Some(contentProcessor as Processor);
        /* see externalEntityContentProcessor vs contentProcessor */
        (*parser).doContent(
            if !(*parser).m_parentParser.is_null() {
                1i32
            } else {
                0i32
            },
            EncodingType::Normal,
            s,
            end,
            nextPtr,
            ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
        )
    } /* save one level of indirection */
}

unsafe extern "C" fn errorProcessor(
    mut parser: XML_Parser,
    mut _s: *const c_char,
    mut _end: *const c_char,
    mut _nextPtr: *mut *const c_char,
) -> XML_Error {
    return (*parser).m_errorCode;
}

unsafe extern "C" fn storeAttributeValue(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut isCdata: XML_Bool,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut pool: *mut STRING_POOL,
) -> XML_Error {
    let mut result: XML_Error = appendAttributeValue(parser, enc_type, isCdata, ptr, end, pool);
    if result as u64 != 0 {
        return result;
    }
    if isCdata == 0
        && (*pool).ptr.wrapping_offset_from((*pool).start) as c_long != 0
        && *(*pool).ptr.offset(-1) as c_int == 0x20
    {
        (*pool).ptr = (*pool).ptr.offset(-1)
    }
    if if (*pool).ptr == (*pool).end as *mut XML_Char && (*pool).grow() == 0 {
        0
    } else {
        let fresh38 = (*pool).ptr;
        (*pool).ptr = (*pool).ptr.offset(1);
        *fresh38 = '\u{0}' as XML_Char;
        1
    } == 0
    {
        return XML_ERROR_NO_MEMORY;
    }
    return XML_ERROR_NONE;
}

unsafe extern "C" fn appendAttributeValue(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut isCdata: XML_Bool,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut pool: *mut STRING_POOL,
) -> XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    let enc = (*parser).encoding(enc_type);
    loop {
        let mut next: *const c_char = 0 as *const c_char;
        let mut tok: c_int = (*enc).xmlLiteralTok(XML_ATTRIBUTE_VALUE_LITERAL, ptr, end, &mut next);
        let mut current_block_62: u64;
        match tok {
            super::xmltok::XML_TOK_NONE => {
                return XML_ERROR_NONE;
                /* LCOV_EXCL_STOP */
            }
            super::xmltok::XML_TOK_INVALID => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr = next
                }
                return XML_ERROR_INVALID_TOKEN;
            }
            super::xmltok::XML_TOK_PARTIAL => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr = ptr
                }
                return XML_ERROR_INVALID_TOKEN;
            }
            super::xmltok::XML_TOK_CHAR_REF => {
                let mut buf: [XML_Char; XML_ENCODE_MAX] = [0; XML_ENCODE_MAX];
                let mut i: c_int = 0;
                let mut n: c_int = (*enc).charRefNumber(ptr);
                if n < 0 {
                    if !enc_type.is_internal() {
                        (*parser).m_eventPtr = ptr
                    }
                    return XML_ERROR_BAD_CHAR_REF;
                }
                if isCdata == 0
                    && n == 0x20
                    && ((*pool).ptr.wrapping_offset_from((*pool).start) as c_long == 0
                        || *(*pool).ptr.offset(-1) as c_int == 0x20)
                {
                    current_block_62 = 11796148217846552555;
                } else {
                    n = XmlEncode(n, buf.as_mut_ptr() as *mut ICHAR);
                    /* The XmlEncode() functions can never return 0 here.  That
                     * error return happens if the code point passed in is either
                     * negative or greater than or equal to 0x110000.  The
                     * XmlCharRefNumber() functions will all return a number
                     * strictly less than 0x110000 or a negative value if an error
                     * occurred.  The negative value is intercepted above, so
                     * XmlEncode() is never passed a value it might return an
                     * error for.
                     */
                    i = 0;
                    while i < n {
                        if if (*pool).ptr == (*pool).end as *mut XML_Char && (*pool).grow() == 0 {
                            0
                        } else {
                            let fresh39 = (*pool).ptr;
                            (*pool).ptr = (*pool).ptr.offset(1);
                            *fresh39 = buf[i as usize];
                            1
                        } == 0
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        i += 1
                    }
                    current_block_62 = 11796148217846552555;
                }
            }
            super::xmltok::XML_TOK_DATA_CHARS => {
                if (*pool).append(enc, ptr, next).is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                current_block_62 = 11796148217846552555;
            }
            super::xmltok::XML_TOK_TRAILING_CR => {
                next = ptr.offset((*enc).minBytesPerChar() as isize);
                current_block_62 = 9696599617798541816;
            }
            super::xmltok::XML_TOK_ATTRIBUTE_VALUE_S | super::xmltok::XML_TOK_DATA_NEWLINE => {
                current_block_62 = 9696599617798541816;
            }
            super::xmltok::XML_TOK_ENTITY_REF => {
                let mut name: *const XML_Char = 0 as *const XML_Char;
                let mut checkEntityDecl: c_char = 0;
                let mut ch: XML_Char = (*enc).predefinedEntityName(
                    ptr.offset((*enc).minBytesPerChar() as isize),
                    next.offset(-((*enc).minBytesPerChar() as isize)),
                ) as XML_Char;
                if ch != 0 {
                    if if (*pool).ptr == (*pool).end as *mut XML_Char && (*pool).grow() == 0 {
                        0
                    } else {
                        let fresh41 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *fresh41 = ch;
                        1
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                } else {
                    name = (*parser).m_temp2Pool.storeString(
                        enc,
                        ptr.offset((*enc).minBytesPerChar() as isize),
                        next.offset(-((*enc).minBytesPerChar() as isize)),
                    );
                    if name.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    let entity = hash_lookup!((*dtd).generalEntities, name);
                    (*parser).m_temp2Pool.ptr = (*parser).m_temp2Pool.start;
                    /* First, determine if a check for an existing declaration is needed;
                       if yes, check that the entity exists, and that it is internal.
                    */
                    if pool == &mut (*dtd).pool as *mut STRING_POOL {
                        /* are we called from prolog? */
                        checkEntityDecl = ((*parser).m_prologState.documentEntity != 0
                            && (if (*dtd).standalone as c_int != 0 {
                                (*parser).m_openInternalEntities.is_null() as c_int
                            } else {
                                ((*dtd).hasParamEntityRefs == 0) as c_int
                            }) != 0) as c_char
                    } else {
                        /* if (pool == &parser->m_tempPool): we are called from content */
                        checkEntityDecl = ((*dtd).hasParamEntityRefs == 0
                            || (*dtd).standalone as c_int != 0)
                            as c_char
                    }
                    if checkEntityDecl != 0 {
                        if entity.is_null() {
                            return XML_ERROR_UNDEFINED_ENTITY;
                        } else {
                            if (*entity).is_internal == 0 {
                                return XML_ERROR_ENTITY_DECLARED_IN_PE;
                            }
                        }
                        current_block_62 = 11777552016271000781;
                    } else if entity.is_null() {
                        if cfg!(feature = "mozilla") {
                            return XML_ERROR_UNDEFINED_ENTITY;
                        }
                        current_block_62 = 11796148217846552555;
                    } else {
                        current_block_62 = 11777552016271000781;
                    }
                    match current_block_62 {
                        11796148217846552555 => {}
                        _ => {
                            if (*entity).open != 0 {
                                if !enc_type.is_internal() {
                                    /* It does not appear that this line can be executed.
                                     *
                                     * The "if (entity->open)" check catches recursive entity
                                     * definitions.  In order to be called with an open
                                     * entity, it must have gone through this code before and
                                     * been through the recursive call to
                                     * appendAttributeValue() some lines below.  That call
                                     * sets the local encoding ("enc") to the parser's
                                     * internal encoding (internal_utf8 or internal_utf16),
                                     * which can never be the same as the principle encoding.
                                     * It doesn't appear there is another code path that gets
                                     * here with entity->open being TRUE.
                                     *
                                     * Since it is not certain that this logic is watertight,
                                     * we keep the line and merely exclude it from coverage
                                     * tests.
                                     */
                                    (*parser).m_eventPtr = ptr
                                    /* LCOV_EXCL_LINE */
                                }
                                return XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity).notation.is_null() {
                                if !enc_type.is_internal() {
                                    (*parser).m_eventPtr = ptr
                                }
                                return XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if (*entity).textPtr.is_null() {
                                if !enc_type.is_internal() {
                                    (*parser).m_eventPtr = ptr
                                }
                                return XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
                            } else {
                                let mut result: XML_Error = XML_ERROR_NONE;
                                let mut textEnd: *const XML_Char =
                                    (*entity).textPtr.offset((*entity).textLen as isize);
                                (*entity).open = XML_TRUE;
                                result = appendAttributeValue(
                                    parser,
                                    EncodingType::Internal,
                                    isCdata,
                                    (*entity).textPtr as *mut c_char,
                                    textEnd as *mut c_char,
                                    pool,
                                );
                                (*entity).open = XML_FALSE;
                                if result as u64 != 0 {
                                    return result;
                                }
                            }
                        }
                    }
                }
                current_block_62 = 11796148217846552555;
            }
            _ => {
                /* The only token returned by XmlAttributeValueTok() that does
                 * not have an explicit case here is XML_TOK_PARTIAL_CHAR.
                 * Getting that would require an entity name to contain an
                 * incomplete XML character (e.g. \xE2\x82); however previous
                 * tokenisers will have already recognised and rejected such
                 * names before XmlAttributeValueTok() gets a look-in.  This
                 * default case should be retained as a safety net, but the code
                 * excluded from coverage tests.
                 *
                 * LCOV_EXCL_START
                 */
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr = ptr
                }
                return XML_ERROR_UNEXPECTED_STATE;
            }
        }
        match current_block_62 {
            9696599617798541816 =>
            /* fall through */
            {
                if !(isCdata == 0
                    && ((*pool).ptr.wrapping_offset_from((*pool).start) as c_long == 0
                        || *(*pool).ptr.offset(-1) as c_int == 0x20))
                {
                    if if (*pool).ptr == (*pool).end as *mut XML_Char && (*pool).grow() == 0 {
                        0
                    } else {
                        let fresh40 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *fresh40 = 0x20;
                        1
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                }
            }
            _ => {}
        }
        ptr = next
    }
    /* not reached */
}

unsafe extern "C" fn storeEntityValue(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut entityTextPtr: *const c_char,
    mut entityTextEnd: *const c_char,
) -> XML_Error {
    let mut current_block: u64; /* save one level of indirection */
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut pool: *mut STRING_POOL = &mut (*dtd).entityValuePool;
    let mut result: XML_Error = XML_ERROR_NONE;
    let mut oldInEntityValue: c_int = (*parser).m_prologState.inEntityValue;
    let enc = (*parser).encoding(enc_type);
    (*parser).m_prologState.inEntityValue = 1;
    /* XML_DTD */
    /* never return Null for the value argument in EntityDeclHandler,
    since this would indicate an external entity; therefore we
    have to make sure that entityValuePool.start is not null */
    if (*pool).blocks.is_null() {
        if (*pool).grow() == 0 {
            return XML_ERROR_NO_MEMORY;
        }
    }
    's_41: loop {
        let mut next: *const c_char = 0 as *const c_char;
        let mut tok: c_int = (*enc).xmlLiteralTok(
            XML_ENTITY_VALUE_LITERAL,
            entityTextPtr,
            entityTextEnd,
            &mut next,
        );
        match tok {
            super::xmltok::XML_TOK_PARAM_ENTITY_REF => {
                if (*parser).m_isParamEntity as c_int != 0 || enc_type.is_internal() {
                    let mut name: *const XML_Char = 0 as *const XML_Char;
                    let mut entity: *mut ENTITY = 0 as *mut ENTITY;
                    name = (*parser).m_tempPool.storeString(
                        enc,
                        entityTextPtr.offset((*enc).minBytesPerChar() as isize),
                        next.offset(-((*enc).minBytesPerChar() as isize)),
                    );
                    if name.is_null() {
                        result = XML_ERROR_NO_MEMORY;
                        break;
                    } else {
                        entity = hash_lookup!((*dtd).paramEntities, name);
                        (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
                        if entity.is_null() {
                            /* not a well-formedness error - see XML 1.0: WFC Entity Declared */
                            /* cannot report skipped entity here - see comments on
                               parser->m_skippedEntityHandler
                            if (parser->m_skippedEntityHandler)
                              parser->m_skippedEntityHandler(parser->m_handlerArg, name, 0);
                            */
                            (*dtd).keepProcessing = (*dtd).standalone;
                            break;
                        } else if (*entity).open != 0 {
                            if !enc_type.is_internal() {
                                (*parser).m_eventPtr = entityTextPtr
                            }
                            result = XML_ERROR_RECURSIVE_ENTITY_REF;
                            break;
                        } else if !(*entity).systemId.is_null() {
                            if (*parser).m_handlers.hasExternalEntityRef() {
                                (*dtd).paramEntityRead = XML_FALSE;
                                (*entity).open = XML_TRUE;
                                if (*parser).m_handlers.externalEntityRef(
                                    0 as *const XML_Char,
                                    (*entity).base,
                                    (*entity).systemId,
                                    (*entity).publicId,
                                ) == Ok(0)
                                {
                                    (*entity).open = XML_FALSE;
                                    result = XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                    break;
                                } else {
                                    (*entity).open = XML_FALSE;
                                    if (*dtd).paramEntityRead == 0 {
                                        (*dtd).keepProcessing = (*dtd).standalone
                                    }
                                }
                            } else {
                                (*dtd).keepProcessing = (*dtd).standalone
                            }
                        } else {
                            (*entity).open = XML_TRUE;
                            result = storeEntityValue(
                                parser,
                                EncodingType::Internal,
                                (*entity).textPtr as *mut c_char,
                                (*entity).textPtr.offset((*entity).textLen as isize) as *mut c_char,
                            );
                            (*entity).open = XML_FALSE;
                            if result as u64 != 0 {
                                break;
                            }
                        }
                    }
                } else {
                    /* XML_DTD */
                    /* In the internal subset, PE references are not legal
                    within markup declarations, e.g entity values in this case. */
                    (*parser).m_eventPtr = entityTextPtr;
                    result = XML_ERROR_PARAM_ENTITY_REF;
                    break;
                }
                current_block = 10007731352114176167;
                /* LCOV_EXCL_STOP */
            }
            super::xmltok::XML_TOK_NONE => {
                result = XML_ERROR_NONE;
                break;
            }
            super::xmltok::XML_TOK_ENTITY_REF | super::xmltok::XML_TOK_DATA_CHARS => {
                if (*pool).append(enc, entityTextPtr, next).is_null() {
                    result = XML_ERROR_NO_MEMORY;
                    break;
                } else {
                    current_block = 10007731352114176167;
                }
            }
            super::xmltok::XML_TOK_TRAILING_CR => {
                next = entityTextPtr.offset((*enc).minBytesPerChar() as isize);
                current_block = 13862322071133341448;
            }
            super::xmltok::XML_TOK_DATA_NEWLINE => {
                current_block = 13862322071133341448;
            }
            super::xmltok::XML_TOK_CHAR_REF => {
                let mut buf: [XML_Char; XML_ENCODE_MAX] = [0; XML_ENCODE_MAX];
                let mut i: c_int = 0;
                let mut n: c_int = (*enc).charRefNumber(entityTextPtr);
                if n < 0 {
                    if !enc_type.is_internal() {
                        (*parser).m_eventPtr = entityTextPtr
                    }
                    result = XML_ERROR_BAD_CHAR_REF;
                    break;
                } else {
                    n = XmlEncode(n, buf.as_mut_ptr() as *mut ICHAR);
                    /* The XmlEncode() functions can never return 0 here.  That
                     * error return happens if the code point passed in is either
                     * negative or greater than or equal to 0x110000.  The
                     * XmlCharRefNumber() functions will all return a number
                     * strictly less than 0x110000 or a negative value if an error
                     * occurred.  The negative value is intercepted above, so
                     * XmlEncode() is never passed a value it might return an
                     * error for.
                     */
                    i = 0;
                    while i < n {
                        if (*pool).end == (*pool).ptr as *const XML_Char && (*pool).grow() == 0 {
                            result = XML_ERROR_NO_MEMORY;
                            break 's_41;
                        } else {
                            let fresh43 = (*pool).ptr;
                            (*pool).ptr = (*pool).ptr.offset(1);
                            *fresh43 = buf[i as usize];
                            i += 1
                        }
                    }
                }
                current_block = 10007731352114176167;
            }
            super::xmltok::XML_TOK_PARTIAL => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr = entityTextPtr
                }
                result = XML_ERROR_INVALID_TOKEN;
                break;
            }
            super::xmltok::XML_TOK_INVALID => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr = next
                }
                result = XML_ERROR_INVALID_TOKEN;
                break;
            }
            _ => {
                /* This default case should be unnecessary -- all the tokens
                 * that XmlEntityValueTok() can return have their own explicit
                 * cases -- but should be retained for safety.  We do however
                 * exclude it from the coverage statistics.
                 *
                 * LCOV_EXCL_START
                 */
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr = entityTextPtr
                }
                result = XML_ERROR_UNEXPECTED_STATE;
                break;
            }
        }
        match current_block {
            13862322071133341448 =>
            /* fall through */
            {
                if (*pool).end == (*pool).ptr as *const XML_Char && (*pool).grow() == 0 {
                    result = XML_ERROR_NO_MEMORY;
                    break;
                } else {
                    let fresh42 = (*pool).ptr;
                    (*pool).ptr = (*pool).ptr.offset(1);
                    *fresh42 = 0xa
                }
            }
            _ => {}
        }
        entityTextPtr = next
    }
    (*parser).m_prologState.inEntityValue = oldInEntityValue;
    /* XML_DTD */
    return result;
}

unsafe extern "C" fn normalizeLines(mut s: *mut XML_Char) {
    let mut p: *mut XML_Char = 0 as *mut XML_Char;
    loop {
        if *s == '\u{0}' as XML_Char {
            return;
        }
        if *s as c_int == 0xd {
            break;
        }
        s = s.offset(1)
    }
    p = s;
    loop {
        if *s as c_int == 0xd {
            let fresh44 = p;
            p = p.offset(1);
            *fresh44 = 0xa;
            s = s.offset(1);
            if *s as c_int == 0xa {
                s = s.offset(1)
            }
        } else {
            let fresh45 = s;
            s = s.offset(1);
            let fresh46 = p;
            p = p.offset(1);
            *fresh46 = *fresh45
        }
        if !(*s != 0) {
            break;
        }
    }
    *p = '\u{0}' as XML_Char;
}

unsafe extern "C" fn reportProcessingInstruction(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut start: *const c_char,
    mut end: *const c_char,
) -> c_int {
    let mut target: *const XML_Char = 0 as *const XML_Char;
    let mut data: *mut XML_Char = 0 as *mut XML_Char;
    let mut tem: *const c_char = 0 as *const c_char;
    if !(*parser).m_handlers.hasProcessingInstruction() {
        if (*parser).m_handlers.hasDefault() {
            reportDefault(parser, enc_type, start, end);
        }
        return 1i32;
    }
    let enc = (*parser).encoding(enc_type);
    start = start.offset(((*enc).minBytesPerChar() * 2i32) as isize);
    tem = start.offset((*enc).nameLength(start) as isize);
    target = (*parser).m_tempPool.storeString(enc, start, tem);
    if target.is_null() {
        return 0i32;
    }
    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
    data = (*parser).m_tempPool.storeString(
        enc,
        (*enc).skipS(tem),
        end.offset(-(((*enc).minBytesPerChar() * 2i32) as isize)),
    );
    if data.is_null() {
        return 0i32;
    }
    normalizeLines(data);
    (*parser).m_handlers.processingInstruction(target, data);
    (*parser).m_tempPool.clear();
    1
}

unsafe extern "C" fn reportComment(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut start: *const c_char,
    mut end: *const c_char,
) -> c_int {
    let mut data: *mut XML_Char = 0 as *mut XML_Char;
    if !(*parser).m_handlers.hasComment() {
        if (*parser).m_handlers.hasDefault() {
            reportDefault(parser, enc_type, start, end);
        }
        return 1i32;
    }
    let enc = (*parser).encoding(enc_type);
    data = (*parser).m_tempPool.storeString(
        enc,
        start.offset(((*enc).minBytesPerChar() * 4i32) as isize),
        end.offset(-(((*enc).minBytesPerChar() * 3i32) as isize)),
    );
    if data.is_null() {
        return 0i32;
    }
    normalizeLines(data);
    (*parser).m_handlers.comment(data);
    (*parser).m_tempPool.clear();
    1
}

unsafe extern "C" fn reportDefault(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut s: *const c_char,
    mut end: *const c_char,
) {
    let enc = (*parser).encoding(enc_type);
    if MUST_CONVERT!(enc, s) {
        let mut convert_res: super::xmltok::XML_Convert_Result =
            super::xmltok::XML_CONVERT_COMPLETED;
        let mut eventPP: *mut *const c_char = 0 as *mut *const c_char;
        let mut eventEndPP: *mut *const c_char = 0 as *mut *const c_char;
        if enc_type.is_internal() {
            /* To get here, two things must be true; the parser must be
             * using a character encoding that is not the same as the
             * encoding passed in, and the encoding passed in must need
             * conversion to the internal format (UTF-8 unless XML_UNICODE
             * is defined).  The only occasions on which the encoding passed
             * in is not the same as the parser's encoding are when it is
             * the internal encoding (e.g. a previously defined parameter
             * entity, already converted to internal format).  This by
             * definition doesn't need conversion, so the whole branch never
             * gets executed.
             *
             * For safety's sake we don't delete these lines and merely
             * exclude them from coverage statistics.
             *
             * LCOV_EXCL_START
             */
            eventPP = &mut (*(*parser).m_openInternalEntities).internalEventPtr;
            eventEndPP = &mut (*(*parser).m_openInternalEntities).internalEventEndPtr
            /* LCOV_EXCL_STOP */
        } else {
            eventPP = &mut (*parser).m_eventPtr;
            eventEndPP = &mut (*parser).m_eventEndPtr
        }
        loop {
            let mut dataPtr = (*parser).m_dataBuf as *mut ICHAR;
            convert_res = XmlConvert!(enc, &mut s, end, &mut dataPtr,
                                      (*parser).m_dataBufEnd as *mut ICHAR);
            *eventEndPP = s;

            let defaultRan = (*parser).m_handlers.default(
                (*parser).m_dataBuf,
                dataPtr.wrapping_offset_from((*parser).m_dataBuf as *mut ICHAR) as c_int,
            );

            // Previously unwrapped an Option
            if !defaultRan {
                panic!("Expected default handler to be set");
            }

            *eventPP = s;
            if !(convert_res != super::xmltok::XML_CONVERT_COMPLETED
                && convert_res != super::xmltok::XML_CONVERT_INPUT_INCOMPLETE)
            {
                break;
            }
        }
    } else {
        let defaultRan = (*parser).m_handlers.default(
            s as *mut XML_Char,
            (end as *mut XML_Char).wrapping_offset_from(s as *mut XML_Char) as c_int,
        );

        // Previously unwrapped an Option
        if !defaultRan {
            panic!("Expected default handler to be set");
        }
    };
}

unsafe extern "C" fn defineAttribute(
    mut type_0: *mut ELEMENT_TYPE,
    mut attId: *mut ATTRIBUTE_ID,
    mut isCdata: XML_Bool,
    mut isId: XML_Bool,
    mut value: *const XML_Char,
) -> c_int {
    let mut att: *mut DEFAULT_ATTRIBUTE = 0 as *mut DEFAULT_ATTRIBUTE;
    if !value.is_null() || isId as c_int != 0 {
        /* The handling of default attributes gets messed up if we have
        a default which duplicates a non-default. */
        let mut i: c_int = 0; /* save one level of indirection */
        i = 0; /* save one level of indirection */
        while i < (*type_0).nDefaultAtts {
            if attId == (*(*type_0).defaultAtts.offset(i as isize)).id as *mut ATTRIBUTE_ID {
                return 1i32;
            }
            i += 1
        }
        if isId as c_int != 0 && (*type_0).idAtt.is_null() && (*attId).xmlns == 0 {
            (*type_0).idAtt = attId
        }
    }
    if (*type_0).nDefaultAtts == (*type_0).allocDefaultAtts {
        if (*type_0).allocDefaultAtts == 0 {
            (*type_0).allocDefaultAtts = 8;
            (*type_0).defaultAtts = MALLOC![DEFAULT_ATTRIBUTE; (*type_0).allocDefaultAtts];
            if (*type_0).defaultAtts.is_null() {
                (*type_0).allocDefaultAtts = 0;
                return 0i32;
            }
        } else {
            let mut temp: *mut DEFAULT_ATTRIBUTE = 0 as *mut DEFAULT_ATTRIBUTE;
            let mut count: c_int = (*type_0).allocDefaultAtts * 2;
            temp = REALLOC!(
                (*type_0).defaultAtts as *mut c_void,
                (count as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<DEFAULT_ATTRIBUTE>() as c_ulong)
            ) as *mut DEFAULT_ATTRIBUTE;
            if temp.is_null() {
                return 0i32;
            }
            (*type_0).allocDefaultAtts = count;
            (*type_0).defaultAtts = temp
        }
    }
    att = (*type_0)
        .defaultAtts
        .offset((*type_0).nDefaultAtts as isize);
    (*att).id = attId;
    (*att).value = value;
    (*att).isCdata = isCdata;
    if isCdata == 0 {
        (*attId).maybeTokenized = XML_TRUE
    }
    (*type_0).nDefaultAtts += 1;
    return 1;
}

impl XML_ParserStruct {
    unsafe fn setElementTypePrefix(
        &mut self,
        mut elementType: *mut ELEMENT_TYPE,
    ) -> c_int {
        let dtd: *mut DTD = self.m_dtd;
        let mut name: *const XML_Char = 0 as *const XML_Char;
        name = (*elementType).name;
        while *name != 0 {
            if *name == ASCII_COLON as XML_Char {
                let mut s: *const XML_Char = 0 as *const XML_Char;
                s = (*elementType).name;
                while s != name {
                    if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                        && (*dtd).pool.grow() == 0
                    {
                        0
                    } else {
                        let fresh47 = (*dtd).pool.ptr;
                        (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                        *fresh47 = *s;
                        1
                    } == 0
                    {
                        return 0i32;
                    }
                    s = s.offset(1)
                }
                if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                    && (*dtd).pool.grow() == 0
                {
                    0
                } else {
                    let fresh48 = (*dtd).pool.ptr;
                    (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                    *fresh48 = '\u{0}' as XML_Char;
                    1
                } == 0
                {
                    return 0i32;
                }
                let prefix = hash_insert!(
                    &mut (*dtd).prefixes,
                    (*dtd).pool.start as KEY,
                    PREFIX
                );
                if prefix.is_null() {
                    return 0i32;
                }
                if (*prefix).name == (*dtd).pool.start as *const XML_Char {
                    (*dtd).pool.start = (*dtd).pool.ptr
                } else {
                    (*dtd).pool.ptr = (*dtd).pool.start
                }
                (*elementType).prefix = prefix;
                break;
            } else {
                name = name.offset(1)
            }
        }
        1
    }

    unsafe fn getAttributeId(
        &mut self,
        mut enc_type: EncodingType,
        mut start: *const c_char,
        mut end: *const c_char,
    ) -> *mut ATTRIBUTE_ID {
        let dtd: *mut DTD = self.m_dtd;
        let mut name: *const XML_Char = 0 as *const XML_Char;
        if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char && (*dtd).pool.grow() == 0 {
            0
        } else {
            let fresh49 = (*dtd).pool.ptr;
            (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
            *fresh49 = '\u{0}' as XML_Char;
            1
        } == 0
        {
            return NULL as *mut ATTRIBUTE_ID;
        }
        let enc = self.encoding(enc_type);
        name = (*dtd).pool.storeString(enc, start, end);
        if name.is_null() {
            return NULL as *mut ATTRIBUTE_ID;
        }
        /* skip quotation mark - its storage will be re-used (like in name[-1]) */
        name = name.offset(1);
        let id = hash_insert!(
            &mut (*dtd).attributeIds,
            name as *mut XML_Char,
            ATTRIBUTE_ID
        );
        if id.is_null() {
            return NULL as *mut ATTRIBUTE_ID;
        }
        if (*id).name != name as *mut XML_Char {
            (*dtd).pool.ptr = (*dtd).pool.start
        } else {
            (*dtd).pool.start = (*dtd).pool.ptr;
            if !(self.m_ns == 0) {
                if *name.offset(0) == ASCII_x as XML_Char
                    && *name.offset(1) == ASCII_m as XML_Char
                    && *name.offset(2) == ASCII_l as XML_Char
                    && *name.offset(3) == ASCII_n as XML_Char
                    && *name.offset(4) == ASCII_s as XML_Char
                    && (*name.offset(5) == '\u{0}' as XML_Char
                        || *name.offset(5) == ASCII_COLON as XML_Char)
                {
                    if *name.offset(5) == '\u{0}' as XML_Char {
                        (*id).prefix = &mut (*dtd).defaultPrefix
                    } else {
                        (*id).prefix = hash_insert!(
                            &mut (*dtd).prefixes,
                            name.offset(6),
                            PREFIX
                        );
                    }
                    (*id).xmlns = XML_TRUE
                } else {
                    let mut i: c_int = 0;
                    i = 0;
                    while *name.offset(i as isize) != 0 {
                        /* attributes without prefix are *not* in the default namespace */
                        if *name.offset(i as isize) == ASCII_COLON as XML_Char {
                            let mut j: c_int = 0; /* save one level of indirection */
                            j = 0;
                            while j < i {
                                if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                                    && (*dtd).pool.grow() == 0
                                {
                                    0
                                } else {
                                    let fresh50 = (*dtd).pool.ptr;
                                    (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                                    *fresh50 = *name.offset(j as isize);
                                    1
                                } == 0
                                {
                                    return NULL as *mut ATTRIBUTE_ID;
                                }
                                j += 1
                            }
                            if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                                && (*dtd).pool.grow() == 0
                            {
                                0
                            } else {
                                let fresh51 = (*dtd).pool.ptr;
                                (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                                *fresh51 = '\u{0}' as XML_Char;
                                1
                            } == 0
                            {
                                return NULL as *mut ATTRIBUTE_ID;
                            }
                            (*id).prefix = hash_insert!(
                                &mut (*dtd).prefixes,
                                (*dtd).pool.start as KEY,
                                PREFIX
                            );
                            if (*id).prefix.is_null() {
                                return NULL as *mut ATTRIBUTE_ID;
                            }
                            if (*(*id).prefix).name == (*dtd).pool.start as *const XML_Char {
                                (*dtd).pool.start = (*dtd).pool.ptr
                            } else {
                                (*dtd).pool.ptr = (*dtd).pool.start
                            }
                            break;
                        } else {
                            i += 1
                        }
                    }
                }
            }
        }
        id
    }
}


const CONTEXT_SEP: XML_Char = ASCII_FF as XML_Char;

impl XML_ParserStruct {
    unsafe fn getContext(&mut self) -> *const XML_Char {
        let dtd: *mut DTD = self.m_dtd;
        let mut needSep: XML_Bool = XML_FALSE;
        if !(*dtd).defaultPrefix.binding.is_null() {
            let mut i: c_int = 0;
            let mut len: c_int = 0;
            if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                && self.m_tempPool.grow() == 0
            {
                0
            } else {
                let fresh52 = self.m_tempPool.ptr;
                self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                *fresh52 = ASCII_EQUALS as XML_Char;
                1
            } == 0
            {
                return NULL as *const XML_Char;
            }
            len = (*(*dtd).defaultPrefix.binding).uriLen;
            if self.m_namespaceSeparator != 0 {
                len -= 1
            }
            i = 0;
            while i < len {
                if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                    && self.m_tempPool.grow() == 0
                {
                    0
                } else {
                    let fresh53 = self.m_tempPool.ptr;
                    self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                    *fresh53 = *(*(*dtd).defaultPrefix.binding).uri.offset(i as isize);
                    1
                } == 0
                {
                    /* Because of memory caching, I don't believe this line can be
                     * executed.
                     *
                     * This is part of a loop copying the default prefix binding
                     * URI into the parser's temporary string pool.  Previously,
                     * that URI was copied into the same string pool, with a
                     * terminating NUL character, as part of setContext().  When
                     * the pool was cleared, that leaves a block definitely big
                     * enough to hold the URI on the free block list of the pool.
                     * The URI copy in getContext() therefore cannot run out of
                     * memory.
                     *
                     * If the pool is used between the setContext() and
                     * getContext() calls, the worst it can do is leave a bigger
                     * block on the front of the free list.  Given that this is
                     * all somewhat inobvious and program logic can be changed, we
                     * don't delete the line but we do exclude it from the test
                     * coverage statistics.
                     */
                    return NULL as *const XML_Char;
                    /* LCOV_EXCL_LINE */
                }
                i += 1
            }
            needSep = XML_TRUE
        }
        for prefix in (*dtd).prefixes.values_mut()
        /* This test appears to be (justifiable) paranoia.  There does
         * not seem to be a way of injecting a prefix without a binding
         * that doesn't get errored long before this function is called.
         * The test should remain for safety's sake, so we instead
         * exclude the following line from the coverage statistics.
         */
        {
            let mut i_0: c_int = 0; /* save one level of indirection */
            let mut len_0: c_int = 0;
            let mut s: *const XML_Char = 0 as *const XML_Char;
            if !(*prefix).binding.is_null() {
                if needSep as c_int != 0
                    && (if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                        && self.m_tempPool.grow() == 0
                    {
                        0
                    } else {
                        let fresh54 = self.m_tempPool.ptr;
                        self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                        *fresh54 = CONTEXT_SEP;
                        1
                    }) == 0
                {
                    return NULL as *const XML_Char;
                }
                s = (*prefix).name;
                while *s != 0 {
                    if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                        && self.m_tempPool.grow() == 0
                    {
                        0
                    } else {
                        let fresh55 = self.m_tempPool.ptr;
                        self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                        *fresh55 = *s;
                        1
                    } == 0
                    {
                        return NULL as *const XML_Char;
                    }
                    s = s.offset(1)
                }
                if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                    && self.m_tempPool.grow() == 0
                {
                    0
                } else {
                    let fresh56 = self.m_tempPool.ptr;
                    self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                    *fresh56 = ASCII_EQUALS as XML_Char;
                    1
                } == 0
                {
                    return NULL as *const XML_Char;
                }
                len_0 = (*(*prefix).binding).uriLen;
                if self.m_namespaceSeparator != 0 {
                    len_0 -= 1
                }
                i_0 = 0;
                while i_0 < len_0 {
                    if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                        && self.m_tempPool.grow() == 0
                    {
                        0
                    } else {
                        let fresh57 = self.m_tempPool.ptr;
                        self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                        *fresh57 = *(*(*prefix).binding).uri.offset(i_0 as isize);
                        1
                    } == 0
                    {
                        return NULL as *const XML_Char;
                    }
                    i_0 += 1
                }
                needSep = XML_TRUE
            }
        }
        for e in (*dtd).generalEntities.values() {
            let mut s_0: *const XML_Char = 0 as *const XML_Char;
            if (*e).open == 0 {
                continue;
            }
            if needSep as c_int != 0
                && (if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                    && self.m_tempPool.grow() == 0
                {
                    0
                } else {
                    let fresh58 = self.m_tempPool.ptr;
                    self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                    *fresh58 = CONTEXT_SEP;
                    1
                }) == 0
            {
                return NULL as *const XML_Char;
            }
            s_0 = (*e).name;
            while *s_0 != 0 {
                if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                    && self.m_tempPool.grow() == 0
                {
                    0
                } else {
                    let fresh59 = self.m_tempPool.ptr;
                    self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                    *fresh59 = *s_0;
                    1
                } == 0
                {
                    return 0 as *const XML_Char;
                }
                s_0 = s_0.offset(1)
            }
            needSep = XML_TRUE
        }
        if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
            && self.m_tempPool.grow() == 0
        {
            0
        } else {
            let fresh60 = self.m_tempPool.ptr;
            self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
            *fresh60 = '\u{0}' as XML_Char;
            1
        } == 0
        {
            return NULL as *const XML_Char;
        }
        self.m_tempPool.start
    }

    unsafe fn setContext(&mut self, mut context: *const XML_Char) -> XML_Bool {
        let dtd: *mut DTD = self.m_dtd;
        let mut s: *const XML_Char = context;
        while *context != '\u{0}' as XML_Char {
            if *s == CONTEXT_SEP || *s == '\u{0}' as XML_Char {
                if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                    && self.m_tempPool.grow() == 0
                {
                    0
                } else {
                    let fresh61 = self.m_tempPool.ptr;
                    self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                    *fresh61 = '\u{0}' as XML_Char;
                    1
                } == 0
                {
                    return XML_FALSE;
                }
                if let Some(e) = (*dtd).generalEntities.get_mut(&HashKey::from(self.m_tempPool.start as KEY)) {
                    e.open = XML_TRUE
                }
                if *s != '\u{0}' as XML_Char {
                    s = s.offset(1)
                }
                context = s;
                self.m_tempPool.ptr = self.m_tempPool.start
            } else if *s == ASCII_EQUALS as XML_Char {
                let mut prefix: *mut PREFIX;
                if self
                    .m_tempPool
                    .ptr
                    .wrapping_offset_from(self.m_tempPool.start) as c_long
                    == 0
                {
                    prefix = &mut (*dtd).defaultPrefix
                } else {
                    if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                        && self.m_tempPool.grow() == 0
                    {
                        0
                    } else {
                        let fresh62 = self.m_tempPool.ptr;
                        self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                        *fresh62 = '\u{0}' as XML_Char;
                        1
                    } == 0
                    {
                        return XML_FALSE;
                    }
                    prefix = hash_lookup!(
                        &mut (*dtd).prefixes,
                        self.m_tempPool.start as KEY
                    );
                    if prefix.is_null() {
                        // librexpat change: we need to copy the prefix name
                        // into the DTD pool, since the HashMap keeps a permanent
                        // reference to the name which we can't modify after
                        // the call to `hash_insert!` (unlike the original C code)
                        let prefix_name = (*dtd).pool.copyString(self.m_tempPool.start);
                        if prefix_name.is_null() {
                            return XML_FALSE;
                        }
                        prefix = hash_insert!(
                            &mut (*dtd).prefixes,
                            prefix_name as KEY,
                            PREFIX
                        );
                        if prefix.is_null() {
                            return XML_FALSE;
                        }
                    }
                    self.m_tempPool.ptr = self.m_tempPool.start
                }
                context = s.offset(1);
                while *context != CONTEXT_SEP && *context != '\u{0}' as XML_Char {
                    if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                        && self.m_tempPool.grow() == 0
                    {
                        0
                    } else {
                        let fresh63 = self.m_tempPool.ptr;
                        self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                        *fresh63 = *context;
                        1
                    } == 0
                    {
                        return XML_FALSE;
                    }
                    context = context.offset(1)
                }
                if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                    && self.m_tempPool.grow() == 0
                {
                    0
                } else {
                    let fresh64 = self.m_tempPool.ptr;
                    self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                    *fresh64 = '\u{0}' as XML_Char;
                    1
                } == 0
                {
                    return XML_FALSE;
                }
                if addBinding(
                    self,
                    prefix,
                    NULL as *const ATTRIBUTE_ID,
                    self.m_tempPool.start,
                    &mut self.m_inheritedBindings,
                ) != XML_ERROR_NONE
                {
                    return XML_FALSE;
                }
                self.m_tempPool.ptr = self.m_tempPool.start;
                if *context != '\u{0}' as XML_Char {
                    context = context.offset(1)
                }
                s = context
            } else {
                if if self.m_tempPool.ptr == self.m_tempPool.end as *mut XML_Char
                    && self.m_tempPool.grow() == 0
                {
                    0
                } else {
                    let fresh65 = self.m_tempPool.ptr;
                    self.m_tempPool.ptr = self.m_tempPool.ptr.offset(1);
                    *fresh65 = *s;
                    1
                } == 0
                {
                    return XML_FALSE;
                }
                s = s.offset(1)
            }
        }
        XML_TRUE
    }
}

unsafe extern "C" fn normalizePublicId(mut publicId: *mut XML_Char) {
    let mut p: *mut XML_Char = publicId;
    let mut s: *mut XML_Char = 0 as *mut XML_Char;
    s = publicId;
    while *s != 0 {
        match *s as c_int {
            32 | 13 | 10 => {
                if p != publicId && *p.offset(-1) as c_int != 0x20 {
                    let fresh66 = p;
                    p = p.offset(1);
                    *fresh66 = 0x20
                }
            }
            _ => {
                let fresh67 = p;
                p = p.offset(1);
                *fresh67 = *s
            }
        }
        s = s.offset(1)
    }
    if p != publicId && *p.offset(-1) as c_int == 0x20 {
        p = p.offset(-1)
    }
    *p = '\u{0}' as XML_Char;
}

unsafe extern "C" fn dtdCreate() -> *mut DTD {
    let mut p: *mut DTD = alloc::alloc(Layout::new::<DTD>()) as *mut DTD;
    if p.is_null() {
        return p;
    }
    (*p).pool.init();
    (*p).entityValuePool.init();
    // FIXME: we're writing over uninitialized memory, use `MaybeUninit`???
    std::ptr::write(&mut (*p).generalEntities, Default::default());
    std::ptr::write(&mut (*p).elementTypes, Default::default());
    std::ptr::write(&mut (*p).attributeIds, Default::default());
    std::ptr::write(&mut (*p).prefixes, Default::default());
    (*p).paramEntityRead = XML_FALSE;
    std::ptr::write(&mut (*p).paramEntities, Default::default());
    /* XML_DTD */
    (*p).defaultPrefix.name = NULL as *const XML_Char;
    (*p).defaultPrefix.binding = NULL as *mut BINDING;
    (*p).in_eldecl = XML_FALSE;
    (*p).scaffIndex = NULL as *mut c_int;
    (*p).scaffold = NULL as *mut CONTENT_SCAFFOLD;
    (*p).scaffLevel = 0;
    (*p).scaffSize = 0;
    (*p).scaffCount = 0;
    (*p).contentStringLen = 0;
    (*p).keepProcessing = XML_TRUE;
    (*p).hasParamEntityRefs = XML_FALSE;
    (*p).standalone = XML_FALSE;
    return p;
}
/* do not call if m_parentParser != NULL */

unsafe extern "C" fn dtdReset(mut p: *mut DTD) {
    for e in (*p).elementTypes.values_mut() {
        if (*e).allocDefaultAtts != 0 {
            FREE!((*e).defaultAtts);
        }
    }
    (*p).generalEntities.clear();
    (*p).paramEntityRead = XML_FALSE;
    (*p).paramEntities.clear();
    /* XML_DTD */
    (*p).elementTypes.clear();
    (*p).attributeIds.clear();
    (*p).prefixes.clear();
    (*p).pool.clear();
    (*p).entityValuePool.clear();
    (*p).defaultPrefix.name = NULL as *const XML_Char;
    (*p).defaultPrefix.binding = NULL as *mut BINDING;
    (*p).in_eldecl = XML_FALSE;
    FREE!((*p).scaffIndex);
    (*p).scaffIndex = NULL as *mut c_int;
    FREE!((*p).scaffold);
    (*p).scaffold = NULL as *mut CONTENT_SCAFFOLD;
    (*p).scaffLevel = 0;
    (*p).scaffSize = 0u32;
    (*p).scaffCount = 0u32;
    (*p).contentStringLen = 0u32;
    (*p).keepProcessing = XML_TRUE;
    (*p).hasParamEntityRefs = XML_FALSE;
    (*p).standalone = XML_FALSE;
}

unsafe extern "C" fn dtdDestroy(
    mut p: *mut DTD,
    mut isDocEntity: XML_Bool,
) {
    for e in (*p).elementTypes.values_mut() {
        if (*e).allocDefaultAtts != 0 {
            FREE!((*e).defaultAtts);
        }
    }
    std::ptr::drop_in_place(&mut (*p).generalEntities);
    std::ptr::drop_in_place(&mut (*p).paramEntities);
    /* XML_DTD */
    std::ptr::drop_in_place(&mut (*p).elementTypes);
    std::ptr::drop_in_place(&mut (*p).attributeIds);
    std::ptr::drop_in_place(&mut (*p).prefixes);
    (*p).pool.destroy();
    (*p).entityValuePool.destroy();
    if isDocEntity != 0 {
        FREE!((*p).scaffIndex);
        FREE!((*p).scaffold);
    }
    alloc::dealloc(p as *mut u8, Layout::new::<DTD>());
}
/* Do a deep copy of the DTD. Return 0 for out of memory, non-zero otherwise.
   The new DTD has already been initialized.
*/

unsafe extern "C" fn dtdCopy(
    mut newDtd: *mut DTD,
    mut oldDtd: *const DTD,
) -> c_int {
    /* Copy the prefix table. */
    for oldP in (*oldDtd).prefixes.values() {
        let mut name: *const XML_Char = 0 as *const XML_Char;
        name = (*newDtd).pool.copyString((*oldP).name);
        if name.is_null() {
            return 0i32;
        }
        if hash_insert!(
            &mut (*newDtd).prefixes,
            name,
            PREFIX
        )
        .is_null()
        {
            return 0i32;
        }
    }
    for oldA in (*oldDtd).attributeIds.values()
    /* Copy the attribute id table. */
    {
        let mut name_0: *const XML_Char = 0 as *const XML_Char;
        /* Remember to allocate the scratch byte before the name. */
        if if (*newDtd).pool.ptr == (*newDtd).pool.end as *mut XML_Char
            && (*newDtd).pool.grow() == 0
        {
            0
        } else {
            let fresh68 = (*newDtd).pool.ptr;
            (*newDtd).pool.ptr = (*newDtd).pool.ptr.offset(1);
            *fresh68 = '\u{0}' as XML_Char;
            1
        } == 0
        {
            return 0i32;
        }
        name_0 = (*newDtd).pool.copyString((*oldA).name);
        if name_0.is_null() {
            return 0i32;
        }
        name_0 = name_0.offset(1);
        let newA = hash_insert!(
            &mut (*newDtd).attributeIds,
            name_0 as *mut XML_Char,
            ATTRIBUTE_ID
        );
        if newA.is_null() {
            return 0i32;
        }
        (*newA).maybeTokenized = (*oldA).maybeTokenized;
        if !(*oldA).prefix.is_null() {
            (*newA).xmlns = (*oldA).xmlns;
            if (*oldA).prefix == &(*oldDtd).defaultPrefix as *const PREFIX as *mut PREFIX {
                (*newA).prefix = &mut (*newDtd).defaultPrefix
            } else {
                (*newA).prefix = hash_lookup!(
                    (*newDtd).prefixes,
                    (*(*oldA).prefix).name
                );
            }
        }
    }
    /* Copy the element type table. */
    for oldE in (*oldDtd).elementTypes.values() {
        let mut i: c_int = 0;
        let mut name_1: *const XML_Char = 0 as *const XML_Char;
        name_1 = (*newDtd).pool.copyString((*oldE).name);
        if name_1.is_null() {
            return 0i32;
        }
        let newE = hash_insert!(
            &mut (*newDtd).elementTypes,
            name_1,
            ELEMENT_TYPE
        );
        if newE.is_null() {
            return 0i32;
        }
        if (*oldE).nDefaultAtts != 0 {
            (*newE).defaultAtts = MALLOC![DEFAULT_ATTRIBUTE; (*oldE).nDefaultAtts];
            if (*newE).defaultAtts.is_null() {
                return 0i32;
            }
        }
        if !(*oldE).idAtt.is_null() {
            (*newE).idAtt = hash_lookup!(
                (*newDtd).attributeIds,
                (*(*oldE).idAtt).name as KEY
            );
        }
        (*newE).nDefaultAtts = (*oldE).nDefaultAtts;
        (*newE).allocDefaultAtts = (*newE).nDefaultAtts;
        if !(*oldE).prefix.is_null() {
            (*newE).prefix = hash_lookup!(
                (*newDtd).prefixes,
                (*(*oldE).prefix).name
            );
        }
        i = 0;
        while i < (*newE).nDefaultAtts {
            let ref mut fresh69 = (*(*newE).defaultAtts.offset(i as isize)).id;
            *fresh69 = hash_lookup!(
                (*newDtd).attributeIds,
                (*(*(*oldE).defaultAtts.offset(i as isize)).id).name as KEY
            );
            (*(*newE).defaultAtts.offset(i as isize)).isCdata =
                (*(*oldE).defaultAtts.offset(i as isize)).isCdata;
            if !(*(*oldE).defaultAtts.offset(i as isize)).value.is_null() {
                let ref mut fresh70 = (*(*newE).defaultAtts.offset(i as isize)).value;
                *fresh70 = (*newDtd).pool.copyString(
                    (*(*oldE).defaultAtts.offset(i as isize)).value,
                );
                if (*(*newE).defaultAtts.offset(i as isize)).value.is_null() {
                    return 0i32;
                }
            } else {
                let ref mut fresh71 = (*(*newE).defaultAtts.offset(i as isize)).value;
                *fresh71 = NULL as *const XML_Char
            }
            i += 1
        }
    }
    /* Copy the entity tables. */
    if copyEntityTable(
        &mut (*newDtd).generalEntities,
        &mut (*newDtd).pool,
        &(*oldDtd).generalEntities,
    ) == 0
    {
        return 0i32;
    }
    if copyEntityTable(
        &mut (*newDtd).paramEntities,
        &mut (*newDtd).pool,
        &(*oldDtd).paramEntities,
    ) == 0
    {
        return 0i32;
    }
    (*newDtd).paramEntityRead = (*oldDtd).paramEntityRead;
    /* XML_DTD */
    (*newDtd).keepProcessing = (*oldDtd).keepProcessing;
    (*newDtd).hasParamEntityRefs = (*oldDtd).hasParamEntityRefs;
    (*newDtd).standalone = (*oldDtd).standalone;
    /* Don't want deep copying for scaffolding */
    (*newDtd).in_eldecl = (*oldDtd).in_eldecl;
    (*newDtd).scaffold = (*oldDtd).scaffold;
    (*newDtd).contentStringLen = (*oldDtd).contentStringLen;
    (*newDtd).scaffSize = (*oldDtd).scaffSize;
    (*newDtd).scaffLevel = (*oldDtd).scaffLevel;
    (*newDtd).scaffIndex = (*oldDtd).scaffIndex;
    return 1;
}
/* End dtdCopy */

unsafe extern "C" fn copyEntityTable(
    mut newTable: &mut HashMap<HashKey, Box<ENTITY>>,
    mut newPool: *mut STRING_POOL,
    mut oldTable: &HashMap<HashKey, Box<ENTITY>>,
) -> c_int {
    let mut cachedOldBase: *const XML_Char = NULL as *const XML_Char;
    let mut cachedNewBase: *const XML_Char = NULL as *const XML_Char;
    for oldE in oldTable.values() {
        let mut name: *const XML_Char = 0 as *const XML_Char;
        name = (*newPool).copyString((*oldE).name);
        if name.is_null() {
            return 0i32;
        }
        let newE = hash_insert!(
            &mut newTable,
            name,
            ENTITY
        );
        if newE.is_null() {
            return 0i32;
        }
        if !(*oldE).systemId.is_null() {
            let mut tem: *const XML_Char = (*newPool).copyString((*oldE).systemId);
            if tem.is_null() {
                return 0i32;
            }
            (*newE).systemId = tem;
            if !(*oldE).base.is_null() {
                if (*oldE).base == cachedOldBase {
                    (*newE).base = cachedNewBase
                } else {
                    cachedOldBase = (*oldE).base;
                    tem = (*newPool).copyString(cachedOldBase);
                    if tem.is_null() {
                        return 0i32;
                    }
                    (*newE).base = tem;
                    cachedNewBase = (*newE).base
                }
            }
            if !(*oldE).publicId.is_null() {
                tem = (*newPool).copyString((*oldE).publicId);
                if tem.is_null() {
                    return 0i32;
                }
                (*newE).publicId = tem
            }
        } else {
            let mut tem_0: *const XML_Char =
                (*newPool).copyStringN((*oldE).textPtr, (*oldE).textLen);
            if tem_0.is_null() {
                return 0i32;
            }
            (*newE).textPtr = tem_0;
            (*newE).textLen = (*oldE).textLen
        }
        if !(*oldE).notation.is_null() {
            let mut tem_1: *const XML_Char = (*newPool).copyString((*oldE).notation);
            if tem_1.is_null() {
                return 0i32;
            }
            (*newE).notation = tem_1
        }
        (*newE).is_param = (*oldE).is_param;
        (*newE).is_internal = (*oldE).is_internal
    }
    return 1;
}

pub const INIT_POWER: c_int = 6;

unsafe extern "C" fn keyeq(mut s1: KEY, mut s2: KEY) -> XML_Bool {
    while *s1 as c_int == *s2 as c_int {
        if *s1 as c_int == 0 {
            return XML_TRUE;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1)
    }
    return XML_FALSE;
}

unsafe extern "C" fn keylen(mut s: KEY) -> size_t {
    let mut len: size_t = 0;
    while *s != 0 {
        s = s.offset(1);
        len = len.wrapping_add(1)
    }
    len
}

impl XML_ParserStruct {
    unsafe fn copy_salt_to_sipkey(&mut self, mut key: *mut sipkey) {
        (*key).k[0] = 0u64;
        (*key).k[1] = self.get_hash_secret_salt();
    }

    unsafe fn hash(&mut self, mut s: KEY) -> c_ulong {
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
        self.copy_salt_to_sipkey(&mut key);
        sip24_init(&mut state, &mut key);
        sip24_update(
            &mut state,
            s as *const c_void,
            keylen(s).wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
        );
        return sip24_final(&mut state);
    }
}


impl STRING_POOL {
    unsafe fn init(
        &mut self,
    ) {
        self.blocks = NULL as *mut BLOCK;
        self.freeBlocks = NULL as *mut BLOCK;
        self.start = NULL as *mut XML_Char;
        self.ptr = NULL as *mut XML_Char;
        self.end = NULL as *const XML_Char;
    }

    unsafe fn clear(&mut self) {
        if self.freeBlocks.is_null() {
            self.freeBlocks = self.blocks
        } else {
            let mut p: *mut BLOCK = self.blocks;
            while !p.is_null() {
                let mut tem: *mut BLOCK = (*p).next;
                (*p).next = self.freeBlocks;
                self.freeBlocks = p;
                p = tem
            }
        }
        self.blocks = NULL as *mut BLOCK;
        self.start = NULL as *mut XML_Char;
        self.ptr = NULL as *mut XML_Char;
        self.end = NULL as *const XML_Char;
    }

    unsafe fn destroy(&mut self) {
        let mut p: *mut BLOCK = self.blocks;
        while !p.is_null() {
            let mut tem: *mut BLOCK = (*p).next;
            FREE!(p);
            p = tem
        }
        p = self.freeBlocks;
        while !p.is_null() {
            let mut tem_0: *mut BLOCK = (*p).next;
            FREE!(p);
            p = tem_0
        }
    }

    unsafe fn append(
        &mut self,
        enc: &ENCODING,
        mut ptr: *const c_char,
        end: *const c_char,
    ) -> *mut XML_Char {
        if self.ptr.is_null() && self.grow() == 0 {
            return NULL as *mut XML_Char;
        }
        loop {
            let convert_res: super::xmltok::XML_Convert_Result = XmlConvert!(
                enc,
                &mut ptr,
                end,
                &mut self.ptr as *mut *mut _ as *mut *mut ICHAR,
                self.end as *mut ICHAR,
            );
            if convert_res == super::xmltok::XML_CONVERT_COMPLETED
                || convert_res == super::xmltok::XML_CONVERT_INPUT_INCOMPLETE
            {
                break;
            }
            if self.grow() == 0 {
                return NULL as *mut XML_Char;
            }
        }
        self.start
    }

    unsafe fn copyString(
        &mut self,
        mut s: *const XML_Char,
    ) -> *const XML_Char {
        loop {
            if if self.ptr == self.end as *mut XML_Char && self.grow() == 0 {
                0
            } else {
                let fresh77 = self.ptr;
                self.ptr = self.ptr.offset(1);
                *fresh77 = *s;
                1
            } == 0
            {
                return NULL as *const XML_Char;
            }
            let fresh78 = s;
            s = s.offset(1);
            if !(*fresh78 != 0) {
                break;
            }
        }
        s = self.start;
        self.start = self.ptr;
        return s;
    }

    unsafe fn copyStringN(
        &mut self,
        mut s: *const XML_Char,
        mut n: c_int,
    ) -> *const XML_Char {
        if self.ptr.is_null() && self.grow() == 0 {
            /* The following line is unreachable given the current usage of
            * poolCopyStringN().  Currently it is called from exactly one
            * place to copy the text of a simple general entity.  By that
            * point, the name of the entity is already stored in the pool, so
            * pool->ptr cannot be NULL.
            *
            * If poolCopyStringN() is used elsewhere as it well might be,
            * this line may well become executable again.  Regardless, this
            * sort of check shouldn't be removed lightly, so we just exclude
            * it from the coverage statistics.
            */
            return NULL as *const XML_Char;
            /* LCOV_EXCL_LINE */
        }
        while n > 0 {
            if if self.ptr == self.end as *mut XML_Char && self.grow() == 0 {
                0
            } else {
                let fresh79 = self.ptr;
                self.ptr = self.ptr.offset(1);
                *fresh79 = *s;
                1
            } == 0
            {
                return NULL as *const XML_Char;
            }
            n -= 1;
            s = s.offset(1)
        }
        s = self.start;
        self.start = self.ptr;
        return s;
    }

    unsafe fn appendString(&mut self, mut s: *const XML_Char) -> bool {
        while *s != 0 {
            if !self.appendChar(*s) {
                return true;
            }
            s = s.offset(1)
        }
        self.start.is_null()
    }

    unsafe fn storeString(&mut self, enc: &ENCODING, ptr: *const c_char, end: *const c_char) -> *mut XML_Char {
        if self.append(enc, ptr, end).is_null() {
            return NULL as *mut XML_Char;
        }
        if self.ptr == self.end as *mut XML_Char && self.grow() == 0 {
            return NULL as *mut XML_Char;
        }
        let fresh81 = self.ptr;
        self.ptr = self.ptr.offset(1);
        *fresh81 = 0;
        self.start
    }
}

unsafe extern "C" fn poolBytesToAllocateFor(mut blockSize: c_int) -> size_t {
    /* Unprotected math would be:
     ** return offsetof(BLOCK, s) + blockSize * sizeof(XML_Char);
     **
     ** Detect overflow, avoiding _signed_ overflow undefined behavior
     ** For a + b * c we check b * c in isolation first, so that addition of a
     ** on top has no chance of making us accept a small non-negative number
     */
    let stretch: size_t = ::std::mem::size_of::<XML_Char>() as c_ulong; /* can be 4 bytes */
    if blockSize <= 0 {
        return 0u64;
    }
    if blockSize > (INT_MAX as c_ulong).wrapping_div(stretch) as c_int {
        return 0u64;
    }
    let stretchedBlockSize: c_int = blockSize * stretch as c_int;
    let bytesToAllocate: c_int =
        (12u64).wrapping_add(stretchedBlockSize as c_uint as c_ulong) as c_int;
    if bytesToAllocate < 0 {
        return 0u64;
    }
    bytesToAllocate as size_t
}

impl STRING_POOL {
    unsafe fn grow(&mut self) -> XML_Bool {
        if !self.freeBlocks.is_null() {
            if self.start.is_null() {
                self.blocks = self.freeBlocks;
                self.freeBlocks = (*self.freeBlocks).next;
                (*self.blocks).next = NULL as *mut block;
                self.start = (*self.blocks).s.as_mut_ptr();
                self.end = self.start.offset((*self.blocks).size as isize);
                self.ptr = self.start;
                return XML_TRUE;
            }
            if (self.end.wrapping_offset_from(self.start) as c_long)
                < (*self.freeBlocks).size as c_long
            {
                let mut tem: *mut BLOCK = (*self.freeBlocks).next;
                (*self.freeBlocks).next = self.blocks;
                self.blocks = self.freeBlocks;
                self.freeBlocks = tem;
                memcpy(
                    (*self.blocks).s.as_mut_ptr() as *mut c_void,
                    self.start as *const c_void,
                    (self.end.wrapping_offset_from(self.start) as c_ulong)
                        .wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
                );
                self.ptr = (*self.blocks)
                    .s
                    .as_mut_ptr()
                    .offset(self.ptr.wrapping_offset_from(self.start));
                self.start = (*self.blocks).s.as_mut_ptr();
                self.end = self.start.offset((*self.blocks).size as isize);
                return XML_TRUE;
            }
        }
        if !self.blocks.is_null() && self.start == (*self.blocks).s.as_mut_ptr() {
            let mut temp: *mut BLOCK = 0 as *mut BLOCK;
            let mut blockSize: c_int =
                (self.end.wrapping_offset_from(self.start) as c_uint).wrapping_mul(2u32) as c_int;
            let mut bytesToAllocate: size_t = 0;
            /* NOTE: Needs to be calculated prior to calling `realloc`
            to avoid dangling pointers: */
            let offsetInsideBlock: ptrdiff_t =
                self.ptr.wrapping_offset_from(self.start) as c_long;
            if blockSize < 0 {
                /* This condition traps a situation where either more than
                 * INT_MAX/2 bytes have already been allocated.  This isn't
                 * readily testable, since it is unlikely that an average
                 * machine will have that much memory, so we exclude it from the
                 * coverage statistics.
                 */
                return XML_FALSE;
                /* LCOV_EXCL_LINE */
            }
            bytesToAllocate = poolBytesToAllocateFor(blockSize);
            if bytesToAllocate == 0 {
                return XML_FALSE;
            }
            temp = REALLOC!(self.blocks, bytesToAllocate as usize) as *mut BLOCK;
            if temp.is_null() {
                return XML_FALSE;
            }
            self.blocks = temp;
            (*self.blocks).size = blockSize;
            self.ptr = (*self.blocks)
                .s
                .as_mut_ptr()
                .offset(offsetInsideBlock as isize);
            self.start = (*self.blocks).s.as_mut_ptr();
            self.end = self.start.offset(blockSize as isize)
        } else {
            let mut tem_0: *mut BLOCK = 0 as *mut BLOCK;
            let mut blockSize_0: c_int = self.end.wrapping_offset_from(self.start) as c_int;
            let mut bytesToAllocate_0: size_t = 0;
            if blockSize_0 < 0 {
                /* This condition traps a situation where either more than
                 * INT_MAX bytes have already been allocated (which is prevented
                 * by various pieces of program logic, not least this one, never
                 * mind the unlikelihood of actually having that much memory) or
                 * the pool control fields have been corrupted (which could
                 * conceivably happen in an extremely buggy user handler
                 * function).  Either way it isn't readily testable, so we
                 * exclude it from the coverage statistics.
                 */
                return XML_FALSE;
                /* LCOV_EXCL_LINE */
            }
            if blockSize_0 < INIT_BLOCK_SIZE {
                blockSize_0 = INIT_BLOCK_SIZE
            } else {
                /* Detect overflow, avoiding _signed_ overflow undefined behavior */
                if ((blockSize_0 as c_uint).wrapping_mul(2u32) as c_int) < 0 {
                    return XML_FALSE;
                } /* save one level of indirection */
                blockSize_0 *= 2
            } /* save one level of indirection */
            bytesToAllocate_0 = poolBytesToAllocateFor(blockSize_0); /* save one level of indirection */
            if bytesToAllocate_0 == 0 {
                return XML_FALSE;
            } /* save one level of indirection */
            tem_0 = MALLOC!(bytesToAllocate_0) as *mut BLOCK;
            if tem_0.is_null() {
                return XML_FALSE;
            }
            (*tem_0).size = blockSize_0;
            (*tem_0).next = self.blocks;
            self.blocks = tem_0;
            if self.ptr != self.start {
                memcpy(
                    (*tem_0).s.as_mut_ptr() as *mut c_void,
                    self.start as *const c_void,
                    (self.ptr.wrapping_offset_from(self.start) as c_ulong)
                        .wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
                );
            }
            self.ptr = (*tem_0)
                .s
                .as_mut_ptr()
                .offset(self.ptr.wrapping_offset_from(self.start));
            self.start = (*tem_0).s.as_mut_ptr();
            self.end = (*tem_0).s.as_mut_ptr().offset(blockSize_0 as isize)
        }
        XML_TRUE
    }
}

impl XML_ParserStruct {
    unsafe fn nextScaffoldPart(&mut self) -> c_int {
        let dtd: *mut DTD = self.m_dtd;
        let mut me: *mut CONTENT_SCAFFOLD = 0 as *mut CONTENT_SCAFFOLD;
        let mut next: c_int = 0;
        if (*dtd).scaffIndex.is_null() {
            (*dtd).scaffIndex = MALLOC![c_int; self.m_groupSize];
            if (*dtd).scaffIndex.is_null() {
                return -(1i32);
            }
            *(*dtd).scaffIndex.offset(0) = 0
        }
        if (*dtd).scaffCount >= (*dtd).scaffSize {
            let mut temp: *mut CONTENT_SCAFFOLD = 0 as *mut CONTENT_SCAFFOLD;
            if !(*dtd).scaffold.is_null() {
                temp = REALLOC!(
                    (*dtd).scaffold as *mut c_void,
                    ((*dtd).scaffSize.wrapping_mul(2u32) as c_ulong)
                        .wrapping_mul(::std::mem::size_of::<CONTENT_SCAFFOLD>() as c_ulong)
                ) as *mut CONTENT_SCAFFOLD;
                if temp.is_null() {
                    return -(1i32);
                }
                (*dtd).scaffSize = (*dtd).scaffSize.wrapping_mul(2u32)
            } else {
                temp = MALLOC![CONTENT_SCAFFOLD; 32];
                if temp.is_null() {
                    return -(1i32);
                }
                (*dtd).scaffSize = INIT_SCAFFOLD_ELEMENTS as c_uint
            }
            (*dtd).scaffold = temp
        }
        let fresh82 = (*dtd).scaffCount;
        (*dtd).scaffCount = (*dtd).scaffCount.wrapping_add(1);
        next = fresh82 as c_int;
        me = &mut *(*dtd).scaffold.offset(next as isize) as *mut CONTENT_SCAFFOLD;
        if (*dtd).scaffLevel != 0 {
            let mut parent: *mut CONTENT_SCAFFOLD = &mut *(*dtd)
                .scaffold
                .offset(*(*dtd).scaffIndex.offset(((*dtd).scaffLevel - 1) as isize) as isize)
                as *mut CONTENT_SCAFFOLD;
            if (*parent).lastchild != 0 {
                (*(*dtd).scaffold.offset((*parent).lastchild as isize)).nextsib = next
            }
            if (*parent).childcnt == 0 {
                (*parent).firstchild = next
            }
            (*parent).lastchild = next;
            (*parent).childcnt += 1
        }
        (*me).nextsib = 0;
        (*me).childcnt = (*me).nextsib;
        (*me).lastchild = (*me).childcnt;
        (*me).firstchild = (*me).lastchild;
        next
    }

    unsafe fn build_node(
        &mut self,
        mut src_node: c_int,
        mut dest: *mut XML_Content,
        mut contpos: *mut *mut XML_Content,
        mut strpos: *mut *mut XML_Char,
    ) {
        let dtd: *mut DTD = self.m_dtd;
        (*dest).type_0 = (*(*dtd).scaffold.offset(src_node as isize)).type_0;
        (*dest).quant = (*(*dtd).scaffold.offset(src_node as isize)).quant;
        if (*dest).type_0 == XML_CTYPE_NAME {
            let mut src: *const XML_Char = 0 as *const XML_Char;
            (*dest).name = *strpos;
            src = (*(*dtd).scaffold.offset(src_node as isize)).name;
            loop {
                let fresh83 = *strpos;
                *strpos = (*strpos).offset(1);
                *fresh83 = *src;
                if *src == 0 {
                    break;
                }
                src = src.offset(1)
            }
            (*dest).numchildren = 0u32;
            (*dest).children = NULL as *mut XML_Content
        } else {
            let mut i: c_uint = 0;
            let mut cn: c_int = 0;
            (*dest).numchildren = (*(*dtd).scaffold.offset(src_node as isize)).childcnt as c_uint;
            (*dest).children = *contpos;
            *contpos = (*contpos).offset((*dest).numchildren as isize);
            i = 0;
            cn = (*(*dtd).scaffold.offset(src_node as isize)).firstchild;
            while i < (*dest).numchildren {
                self.build_node(
                    cn,
                    &mut *(*dest).children.offset(i as isize),
                    contpos,
                    strpos,
                );
                i = i.wrapping_add(1);
                cn = (*(*dtd).scaffold.offset(cn as isize)).nextsib
            }
            (*dest).name = NULL as *mut XML_Char
        };
    }

    unsafe fn build_model(&mut self) -> *mut XML_Content {
        let dtd: *mut DTD = self.m_dtd;
        let mut ret: *mut XML_Content = 0 as *mut XML_Content;
        let mut cpos: *mut XML_Content = 0 as *mut XML_Content;
        let mut str: *mut XML_Char = 0 as *mut XML_Char;
        let mut allocsize: c_int = ((*dtd).scaffCount as c_ulong)
            .wrapping_mul(::std::mem::size_of::<XML_Content>() as c_ulong)
            .wrapping_add(
                ((*dtd).contentStringLen as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
            ) as c_int;
        ret = MALLOC!(allocsize as size_t) as *mut XML_Content;
        if ret.is_null() {
            return NULL as *mut XML_Content;
        }
        str = &mut *ret.offset((*dtd).scaffCount as isize) as *mut XML_Content as *mut XML_Char;
        cpos = &mut *ret.offset(1) as *mut XML_Content;
        self.build_node(0, ret, &mut cpos, &mut str);
        ret
    }

    unsafe fn getElementType(
        &mut self,
        mut enc_type: EncodingType,
        mut ptr: *const c_char,
        mut end: *const c_char,
    ) -> *mut ELEMENT_TYPE {
        let dtd: *mut DTD = self.m_dtd;
        let enc = self.encoding(enc_type);
        let mut name: *const XML_Char = (*dtd).pool.storeString(enc, ptr, end);
        if name.is_null() {
            return NULL as *mut ELEMENT_TYPE;
        }
        let ret = hash_insert!(
            &mut (*dtd).elementTypes,
            name,
            ELEMENT_TYPE
        );
        if ret.is_null() {
            return NULL as *mut ELEMENT_TYPE;
        }
        if (*ret).name != name {
            (*dtd).pool.ptr = (*dtd).pool.start
        } else {
            (*dtd).pool.start = (*dtd).pool.ptr;
            if self.setElementTypePrefix(ret) == 0 {
                return NULL as *mut ELEMENT_TYPE;
            }
        }
        return ret;
    }
}

unsafe extern "C" fn copyString(
    mut s: *const XML_Char,
) -> *mut XML_Char {
    let mut charsRequired: c_int = 0;
    let mut result: *mut XML_Char = 0 as *mut XML_Char;
    /* First determine how long the string is */
    while *s.offset(charsRequired as isize) as c_int != 0 {
        charsRequired += 1
    }
    /* Include the terminator */
    charsRequired += 1;
    /* Now allocate space for the copy */
    result = MALLOC![XML_Char; charsRequired];
    if result.is_null() {
        return NULL as *mut XML_Char;
    }
    /* Copy the original into place */
    memcpy(
        result as *mut c_void,
        s as *const c_void,
        (charsRequired as c_ulong).wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
    );
    return result;
}
unsafe extern "C" fn run_static_initializers() {
    xmlLen = (::std::mem::size_of::<[XML_Char; 37]>() as c_int as c_ulong)
        .wrapping_div(::std::mem::size_of::<XML_Char>() as c_ulong)
        .wrapping_sub(1u64) as c_int;
    xmlnsLen = (::std::mem::size_of::<[XML_Char; 30]>() as c_int as c_ulong)
        .wrapping_div(::std::mem::size_of::<XML_Char>() as c_ulong)
        .wrapping_sub(1u64) as c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
