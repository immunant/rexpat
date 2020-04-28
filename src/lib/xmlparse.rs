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
pub use crate::expat_external_h::{XML_Char, XML_Index, XML_LChar, XML_Size};
pub use crate::expat_h::{
    XML_AttlistDeclHandler, XML_Bool, XML_CharacterDataHandler, XML_CommentHandler, XML_Content,
    XML_Content_Quant, XML_Content_Type, XML_DefaultHandler, XML_ElementDeclHandler, XML_Encoding,
    XML_EndCdataSectionHandler, XML_EndDoctypeDeclHandler, XML_EndElementHandler,
    XML_EndNamespaceDeclHandler, XML_EntityDeclHandler, XML_Error, XML_ErrorCode, XML_Expat_Version,
    XML_ExternalEntityRefHandler, XML_Feature, XML_FeatureEnum, XML_Memory_Handling_Suite,
    XML_NotStandaloneHandler, XML_NotationDeclHandler, XML_ParamEntityParsing, XML_Parser,
    XML_Parsing, XML_ParsingStatus, XML_ProcessingInstructionHandler, XML_SkippedEntityHandler,
    XML_StartCdataSectionHandler, XML_StartDoctypeDeclHandler, XML_StartElementHandler,
    XML_StartNamespaceDeclHandler, XML_Status, XML_UnknownEncodingHandler,
    XML_UnparsedEntityDeclHandler, XML_XmlDeclHandler, XML_cp,
};
pub use crate::lib::xmlrole::{
    prolog_state, XmlPrologStateInit, XmlPrologStateInitExternalEntity,
    PROLOG_STATE, XML_ROLE
};
pub use crate::lib::xmltok::{
    XmlParseXmlDecl, XmlParseXmlDeclNS, UnknownEncoding,
};
pub use crate::lib::xmltok::*;
use crate::lib::string_pool::StringPool;
pub use ::libc::INT_MAX;
use libc::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void, size_t, ptrdiff_t, memcpy, memcmp, memmove, memset};
use num_traits::{ToPrimitive,FromPrimitive};

use fallible_collections::FallibleBox;

use std::alloc::{self, Layout};
use std::borrow::Cow;
use std::cell::RefCell;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::mem;
use std::ops;
use std::ptr;

pub const XML_CONTEXT_BYTES: c_int = 1024;

#[derive(Copy, Clone, Debug)]
pub struct ExpatBufRef<'a, T = c_char>(&'a [T]);
impl<'a, T> ExpatBufRef<'a, T> {
    pub fn new<'new>(start: *const T, end: *const T) -> ExpatBufRef<'new, T> {
        unsafe { ExpatBufRef(std::slice::from_raw_parts(start, end.wrapping_offset_from(start) as usize)) }
    }
    pub fn new_len<'new>(start: *const T, len: usize) -> ExpatBufRef<'new, T> {
        unsafe { ExpatBufRef(std::slice::from_raw_parts(start, len)) }
    }
    pub fn empty<'new>() -> ExpatBufRef<'new, T> {
        ExpatBufRef(&[])
    }

    pub fn end(&self) -> *const T {
        unsafe { self.as_ptr().offset(self.len().try_into().unwrap()) }
    }

    pub fn with_start(&self, new_start: *const T) -> ExpatBufRef<'a, T> {
        if new_start < self.as_ptr() || new_start > self.end() {
            panic!("Attempted to move the start of an ExpatBufRef to an invalid pointer: {:?}", new_start);
        }
        ExpatBufRef::new(new_start, self.end())
    }

    pub fn with_end(&self, new_end: *const T) -> ExpatBufRef<'a, T> {
        if new_end < self.as_ptr() || new_end > self.end() {
            panic!("Attempted to move the end of an ExpatBufRef to an invalid pointer: {:?}", new_end);
        }
        ExpatBufRef::new(self.0.as_ptr(), new_end)
    }

    pub fn with_len(&self, len: usize) -> ExpatBufRef<'a, T> {
        if len > self.len() {
            panic!("Attempted to expand an ExpatBufRef to lenth: {:?}", len);
        }
        ExpatBufRef(&self.0[..len])
    }

    pub fn inc_start(&self, offset: isize) -> ExpatBufRef<'a, T> {
        if offset < 0 {
            panic!("Attempted to decrement the start of an ExpatBufRef");
        }
        if offset as usize > self.len() {
            panic!("Attempted to increment the start of an ExpatBufRef by too much: {:?}", offset);
        }
        ExpatBufRef(&self.0[(offset as usize)..])
    }

    pub fn dec_end(&self, offset: usize) -> ExpatBufRef<'a, T> {
        if offset > self.len() {
            panic!("Attempted to decrement the end of an ExpatBufRef by too much: {:?}", offset);
        }
        ExpatBufRef(&self.0[..(self.len() - offset)])
    }
}

impl<'a, T> ops::Deref for ExpatBufRef<'a, T> {
    type Target = &'a [T];

    fn deref(&self) -> &&'a [T] {
        &self.0
    }
}

impl<'a, T> From<&'a [T]> for ExpatBufRef<'a, T> {
    fn from(s: &'a [T]) -> ExpatBufRef<'a, T> {
        ExpatBufRef(s)
    }
}

impl<'a> From<ExpatBufRef<'a, c_char>> for ExpatBufRef<'a, c_ushort> {
    fn from(s: ExpatBufRef<'a, c_char>) -> ExpatBufRef<'a, c_ushort> {
        ExpatBufRef::new(s.as_ptr() as *const c_ushort, s.end() as *const c_ushort)
    }
}

#[derive(Debug)]
pub struct ExpatBufRefMut<'a, T = c_char>(&'a mut [T]);
impl<'a, T> ExpatBufRefMut<'a, T> {
    pub fn new<'new>(start: *mut T, end: *mut T) -> ExpatBufRefMut<'new, T> {
        unsafe { ExpatBufRefMut(std::slice::from_raw_parts_mut(start, end.wrapping_offset_from(start) as usize)) }
    }
    pub fn new_len<'new>(start: *mut T, len: usize) -> ExpatBufRefMut<'new, T> {
        unsafe { ExpatBufRefMut(std::slice::from_raw_parts_mut(start, len)) }
    }
    pub fn empty<'new>() -> ExpatBufRefMut<'new, T> {
        ExpatBufRefMut(&mut [])
    }

    pub fn end(&mut self) -> *mut T {
        unsafe { self.as_mut_ptr().offset(self.len().try_into().unwrap()) }
    }

    pub fn with_start<'b: 'a>(&'b mut self, new_start: *mut T) {
        if new_start < self.as_mut_ptr() || new_start > self.end() {
            panic!("Attempted to move the start of an ExpatBufRefMut to an invalid pointer: {:?}", new_start);
        }
        let offset = new_start.wrapping_offset_from(self.0.as_ptr());
        let new_start = unsafe { self.0.as_mut_ptr().offset(offset) };
        *self = ExpatBufRefMut::new(new_start, self.end());
    }

    pub fn with_end(&mut self, new_end: *mut T) {
        if new_end < self.as_mut_ptr() || new_end > self.end() {
            panic!("Attempted to move the end of an ExpatBufRefMut to an invalid pointer: {:?}", new_end);
        }
        *self = ExpatBufRefMut::new(self.0.as_mut_ptr(), new_end);
    }

    pub fn with_len(&mut self, len: usize) {
        if len > self.len() {
            panic!("Attempted to expand an ExpatBufRefMut to lenth: {:?}", len);
        }
        *self = ExpatBufRefMut::new_len(self.0.as_mut_ptr(), len);
    }

    pub fn inc_start(&mut self, offset: usize) {
        if offset > self.len() {
            panic!("Attempted to increment the start of an ExpatBufRef by too much: {:?}", offset);
        }
        let new_start = unsafe { self.0.as_mut_ptr().offset(offset.try_into().unwrap()) };
        *self = ExpatBufRefMut::new(new_start, self.end());
    }

    pub fn dec_end(&mut self, offset: usize) {
        if offset > self.len() {
            panic!("Attempted to decrement the end of an ExpatBufRefMut by too much: {:?}", offset);
        }
        let new_len = self.len() - offset;
        *self = ExpatBufRefMut::new_len(self.0.as_mut_ptr(), new_len);
    }
}

impl<'a, T> ops::Deref for ExpatBufRefMut<'a, T> {
    type Target = &'a mut [T];

    fn deref(&self) -> &&'a mut [T] {
        &self.0
    }
}

impl<'a, T> ops::DerefMut for ExpatBufRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut &'a mut [T] {
        &mut self.0
    }
}

impl<'a, T> From<&'a mut [T]> for ExpatBufRefMut<'a, T> {
    fn from(s: &'a mut [T]) -> ExpatBufRefMut<'a, T> {
        ExpatBufRefMut(s)
    }
}

/// Create a null-terminated XML_Char array from ASCII_ literals
macro_rules! XML_STR {
    [$($char:ident),* $(,)*] => {
        [$( $char as XML_Char, )* 0,]
    };
}

impl STRING_POOL {
    #[inline]
    unsafe fn append_char(&mut self, c: XML_Char) -> bool {
        if self.ptr == self.end as *mut XML_Char && !self.grow() {
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
    unsafe fn characterData(&self, _: &[XML_Char]) -> bool;
    unsafe fn comment(&self, b: *const XML_Char) -> bool;
    unsafe fn default(&self, _: *const XML_Char, _: c_int) -> bool;
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
    unsafe fn startElement(&self, _: *const XML_Char, _: &mut [Attribute]) -> bool;
    unsafe fn startNamespaceDecl(&self, _: *const XML_Char, b: *const XML_Char) -> bool;
    unsafe fn unknownEncoding(&self, _: *const XML_Char, _: *mut XML_Encoding) -> Result<c_int, ()>;
    unsafe fn unparsedEntityDecl(&self, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char, _: *const XML_Char) -> bool;
    unsafe fn xmlDecl(&self, _: *const XML_Char, _: *const XML_Char, _: c_int) -> bool;
}

#[repr(C)]
#[derive(Clone)]
struct CXmlHandlers<'scf> {
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
    m_externalEntityRefHandlerArg: XML_Parser<'scf>,
    m_handlerArg: *mut c_void,
    m_notationDeclHandler: XML_NotationDeclHandler,
    m_notStandaloneHandler: XML_NotStandaloneHandler,
    m_processingInstructionHandler: XML_ProcessingInstructionHandler,
    m_skippedEntityHandler: XML_SkippedEntityHandler,
    m_startCdataSectionHandler: XML_StartCdataSectionHandler,
    m_startDoctypeDeclHandler: XML_StartDoctypeDeclHandler,
    m_startElementHandler: XML_StartElementHandler,
    m_startNamespaceDeclHandler: XML_StartNamespaceDeclHandler,
    m_unknownEncoding: Option<Box<UnknownEncoding>>,
    m_unknownEncodingData: *mut c_void,
    m_unknownEncodingRelease: Option<unsafe extern "C" fn(_: *mut c_void)>,
    m_unknownEncodingHandler: XML_UnknownEncodingHandler,
    m_unknownEncodingHandlerData: *mut c_void,
    m_unparsedEntityDeclHandler: XML_UnparsedEntityDeclHandler,
    m_xmlDeclHandler: XML_XmlDeclHandler,
}

impl<'scf> Default for CXmlHandlers<'scf> {
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
            m_unknownEncoding: None,
            m_unknownEncodingData: std::ptr::null_mut(),
            m_unknownEncodingRelease: None,
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

impl EncodingType {
    fn is_internal(&self) -> bool {
        match self {
            EncodingType::Internal => true,
            _ => false,
        }
    }
}

impl<'scf> CXmlHandlers<'scf> {
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

impl<'scf> XmlHandlers for CXmlHandlers<'scf> {
    unsafe fn startElement(&self, a: *const XML_Char, b: &mut [Attribute]) -> bool {
        self.m_startElementHandler.map(|handler| {
            handler(self.m_handlerArg, a, b.as_mut_ptr() as *mut *const XML_Char);

            true
        }).unwrap_or(false)
    }

    unsafe fn endElement(&self, a: *const XML_Char) -> bool {
        self.m_endElementHandler.map(|handler| {
            handler(self.m_handlerArg, a);

            true
        }).unwrap_or(false)
    }

    unsafe fn characterData(&self, buf: &[XML_Char]) -> bool {
        self.m_characterDataHandler.map(|handler| {
            handler(self.m_handlerArg, buf.as_ptr(), buf.len().try_into().unwrap());

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

    unsafe fn default(&self, s: *const XML_Char, next: c_int) -> bool {
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
pub struct Attribute {
    name: *const XML_Char,
    value: *const XML_Char,
}

impl Attribute {
    fn new(name: *const XML_Char, value: *const XML_Char) -> Self {
        Attribute { name, value }
    }

    fn new_null() -> Self {
        Attribute {
            name: ptr::null(),
            value: ptr::null(),
        }
    }

    unsafe fn from_default(da: &DefaultAttribute) -> Self {
        Attribute {
            name: (*da.id).name.name(),
            value: da.value,
        }
    }
}

#[repr(C)]
pub struct XML_ParserStruct<'scf> {
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
    m_handlers: CXmlHandlers<'scf>,
    pub m_encoding: *const ENCODING,
    pub m_initEncoding: Option<InitEncoding>,
    pub m_internalEncoding: &'static super::xmltok::ENCODING,
    pub m_protocolEncodingName: *const XML_Char,
    pub m_ns: XML_Bool,
    pub m_ns_triplets: XML_Bool,
    pub m_prologState: super::xmlrole::PROLOG_STATE,
    pub m_processor: Option<Processor>,
    pub m_errorCode: XML_Error,
    pub m_eventPtr: *const c_char,
    pub m_eventEndPtr: *const c_char,
    pub m_positionPtr: *const c_char,
    pub m_openInternalEntities: *mut OpenInternalEntity,
    pub m_freeInternalEntities: *mut OpenInternalEntity,
    pub m_defaultExpandInternalEntities: XML_Bool,
    pub m_tagLevel: c_int,
    pub m_declEntity: *mut Entity,
    pub m_doctypeName: *const XML_Char,
    pub m_doctypeSysid: *const XML_Char,
    pub m_doctypePubid: *const XML_Char,
    pub m_declAttributeType: *const XML_Char,
    pub m_declNotationName: *const XML_Char,
    pub m_declNotationPublicId: *const XML_Char,
    pub m_declElementType: *mut ElementType,
    pub m_declAttributeId: *mut AttributeId,
    pub m_declAttributeIsCdata: XML_Bool,
    pub m_declAttributeIsId: XML_Bool,
    pub m_dtd: *mut DTD<'scf>,
    pub m_curBase: *const XML_Char,
    pub m_tagStack: Option<Box<Tag>>,
    pub m_freeTagList: Option<Box<Tag>>,
    pub m_inheritedBindings: *mut Binding,
    pub m_freeBindingList: *mut Binding,
    pub m_nSpecifiedAtts: c_int,
    pub m_idAttIndex: c_int,
    pub m_atts: Vec<Attribute>,
    typed_atts: Vec<TypedAttributeName>,
    pub m_nsAtts: HashSet<HashKey>,
    pub m_position: super::xmltok::Position,
    m_tempPool: StringPool,
    m_temp2Pool: StringPool,
    pub m_groupConnector: *mut c_char,
    pub m_groupSize: c_uint,
    pub m_namespaceSeparator: XML_Char,
    pub m_parentParser: XML_Parser<'scf>,
    pub m_parsingStatus: XML_ParsingStatus,
    pub m_isParamEntity: XML_Bool,
    pub m_useForeignDTD: XML_Bool,
    pub m_paramEntityParsing: XML_ParamEntityParsing,

    #[cfg(feature = "mozilla")]
    pub m_mismatch: *const XML_Char,
}

impl<'scf> XML_ParserStruct<'scf> {
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
    pub blocks: *mut Block,
    pub freeBlocks: *mut Block,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Block {
    pub next: *mut Block,
    pub size: c_int,
    pub s: [XML_Char; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Binding {
    pub prefix: *mut Prefix,
    pub nextTagBinding: *mut Binding,
    pub prevPrefixBinding: *mut Binding,
    pub attId: *mut AttributeId,
    pub uri: *mut XML_Char,
    pub uriLen: c_int,
    pub uriAlloc: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttributeId {
    pub name: TypedAttributeName,
    pub prefix: *mut Prefix,
    pub maybeTokenized: XML_Bool,
    pub xmlns: XML_Bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TypedAttributeName(*mut XML_Char);

impl TypedAttributeName {
    #[inline]
    fn get_type(&self) -> AttributeType {
        unsafe { (*self.0).into() }
    }

    #[inline]
    fn set_type(&mut self, at: AttributeType) {
        unsafe { (*self.0) = at.into(); }
    }

    #[inline]
    fn name(&self) -> *const XML_Char {
        unsafe { self.0.offset(1) }
    }
}

impl From<TypedAttributeName> for HashKey {
    #[inline]
    fn from(typed_name: TypedAttributeName) -> Self {
        HashKey::from(typed_name.name())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum AttributeType {
    Unset,
    Normal,
    Prefixed,
    Namespace,
}

impl AttributeType {
    #[inline]
    fn is_set(&self) -> bool {
        match self {
            AttributeType::Unset => false,
            _ => true
        }
    }
}

impl From<XML_Char> for AttributeType {
    #[inline]
    fn from(c: XML_Char) -> Self {
        use AttributeType::*;
        match c {
            0 => Unset,
            1 => Normal,
            2 => Prefixed,
            3 => Namespace,
            _ => panic!("invalid attribute type byte: {}", c)
        }
    }
}

impl From<AttributeType> for XML_Char {
    #[inline]
    fn from(at: AttributeType) -> Self {
        use AttributeType::*;
        match at {
            Unset => 0,
            Normal => 1,
            Prefixed => 2,
            Namespace => 3,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Prefix {
    pub name: *const XML_Char,
    pub binding: *mut Binding,
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
#[repr(C)]
#[derive(Clone)]
pub struct Tag {
    pub parent: Option<Box<Tag>>,
    pub rawName: *const c_char,
    pub rawNameLength: c_int,
    pub name: TagName,
    pub buf: *mut c_char,
    pub bufEnd: *mut c_char,
    pub bindings: *mut Binding,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TagName {
    pub str_0: *const XML_Char,
    pub localPart: *const XML_Char,
    pub prefix: *const XML_Char,
    pub strLen: c_int,
    pub uriLen: c_int,
    pub prefixLen: c_int,
}


/// Round up n to be a multiple of sz, where sz is a power of 2.
#[inline]
fn round_up(n: usize, sz: usize) -> usize {
    (n + (sz-1)) & !(sz-1)
}

#[inline]
fn safe_ptr_diff<T>(p: *const T, q: *const T) -> isize {
    if p.is_null() || q.is_null() {
        0
    } else {
        p.wrapping_offset_from(q)
    }
}

// FIXME: add a proper lifetime
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HashKey(&'static [XML_Char]);

impl From<KEY> for HashKey {
    fn from(key: KEY) -> Self {
        unsafe {
            HashKey(std::slice::from_raw_parts(key, keylen(key) as usize))
        }
    }
}

macro_rules! hash_insert {
    ($map:expr, $key:expr, $et:ident, $new_fn:expr) => {{
        let __key = $key;
        let __hk = HashKey::from(__key);
        $map.get_mut(&__hk)
            .map(|x| x.as_mut() as *mut $et)
            .unwrap_or_else(|| {
                if $map.try_reserve(1).is_err() {
                    return ptr::null_mut();
                }

                let v = $et { name: __key, ..$new_fn() };
                if let Ok(b) = Box::try_new(v) {
                    $map.entry(__hk).or_insert(b).as_mut() as *mut $et
                } else {
                    ptr::null_mut()
                }
            })
    }};
    ($map:expr, $key:expr, $et:ident) => {
        hash_insert!($map, $key, $et, std::mem::zeroed)
    };
}

macro_rules! hash_lookup {
    ($map:expr, $key:expr) => {
        $map.get_mut(&HashKey::from($key))
            .map_or_else(std::ptr::null_mut, |x| x.as_mut())
    }
}

#[repr(C)]
pub struct DTD<'scf> {
    // TODO: get rid of the `Box`es to eliminate the extra indirection;
    // for now, we can keep them since they're equivalent to the C code's
    // structure anyway
    pub generalEntities: HashMap<HashKey, Box<Entity>>,
    pub elementTypes: HashMap<HashKey, Box<ElementType>>,
    pub attributeIds: HashMap<HashKey, Box<AttributeId>>,
    pub prefixes: HashMap<HashKey, Box<Prefix>>,
    pool: StringPool,
    entityValuePool: StringPool,
    pub keepProcessing: XML_Bool,
    pub hasParamEntityRefs: XML_Bool,
    pub standalone: XML_Bool,
    pub paramEntityRead: XML_Bool,
    // `test_alloc_nested_entities` counts the allocations,
    // so we need to use `Box` here to pass that test
    pub paramEntities: HashMap<HashKey, Box<Entity>>,
    pub defaultPrefix: Prefix,
    pub in_eldecl: XML_Bool,
    pub scaffold: Cow<'scf, RefCell<Scaffold>>,
    pub contentStringLen: c_uint,
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct Scaffold {
    scaffold: Vec<ContentScaffold>,
    index: Vec<usize>,
}

impl Scaffold {
    fn next_part(&mut self) -> Option<usize> {
        if self.scaffold.try_reserve(1).is_err() {
            return None;
        }

        let next = self.scaffold.len();
        self.scaffold.push(unsafe { std::mem::zeroed() });

        if !self.index.is_empty() {
            let idx = self.index.last().unwrap();
            if self.scaffold[*idx].lastchild != 0 {
                let lc = self.scaffold[*idx].lastchild;
                self.scaffold[lc].nextsib = next;
            }
            if self.scaffold[*idx].childcnt == 0 {
                self.scaffold[*idx].firstchild = next;
            }
            self.scaffold[*idx].lastchild = next;
            self.scaffold[*idx].childcnt += 1;
        }
        Some(next)
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ContentScaffold {
    pub type_0: XML_Content_Type,
    pub quant: XML_Content_Quant,
    pub name: *const XML_Char,
    pub firstchild: usize,
    pub lastchild: usize,
    pub childcnt: usize,
    pub nextsib: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Named {
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

#[repr(C)]
#[derive(Clone)]
pub struct ElementType {
    pub name: *const XML_Char,
    pub prefix: *mut Prefix,
    pub idAtt: *const AttributeId,
    pub defaultAtts: Vec<DefaultAttribute>,
}

impl ElementType {
    fn new() -> Self {
        ElementType {
            name: ptr::null(),
            prefix: ptr::null_mut(),
            idAtt: ptr::null(),
            defaultAtts: Vec::new(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DefaultAttribute {
    pub id: *mut AttributeId,
    pub isCdata: XML_Bool,
    pub value: *const XML_Char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entity {
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpenInternalEntity {
    pub internalEventPtr: *const c_char,
    pub internalEventEndPtr: *const c_char,
    pub next: *mut OpenInternalEntity,
    pub entity: *mut Entity,
    pub startTagLevel: c_int,
    pub betweenDecl: XML_Bool,
}

pub type Processor = unsafe extern "C" fn(
    _: XML_Parser,
    _: ExpatBufRef,
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

pub(crate) use unicode_defines::*;

/* WFC: PE Between Declarations */

pub const INIT_TAG_BUF_SIZE: c_int = 32; /* must be a multiple of sizeof(XML_Char) */

pub const INIT_DATA_BUF_SIZE: c_int = 1024;

pub const INIT_ATTS_SIZE: c_int = 16;

pub const INIT_ATTS_VERSION: c_uint = 0xffffffff;

pub const INIT_BLOCK_SIZE: c_int = init_block_size_const();

#[cfg(feature = "mozilla")]
const fn init_block_size_const() -> c_int {
    // FIXME: should be `offset_of(Block, s)`, but that's not supported yet,
    // so we over-estimate its offset
    1024 - (std::mem::size_of::<Block>() / std::mem::size_of::<XML_Char>()) as c_int
}

#[cfg(not(feature = "mozilla"))]
const fn init_block_size_const() -> c_int {
    1024
}

pub const INIT_BUFFER_SIZE: c_int = 1024;

pub const EXPAND_SPARE: usize = 24;

pub const INIT_SCAFFOLD_ELEMENTS: c_int = 32;

macro_rules! MALLOC {
    ($size:expr $(,)?) => {{
        let layout = Layout::from_size_align($size as usize, 1)
            .expect("failed to create Layout");
        alloc::alloc(layout) as *mut c_void
    }};
    // FIXME: we need the @ to disambiguate from the previous form
    (@$ty:ty) => {{
        let layout = Layout::new::<$ty>();
        alloc::alloc(layout) as *mut $ty
    }};
    [$ty:ty; $n:expr] => {{
        let layout = Layout::array::<$ty>($n as usize)
            .expect("failed to create array Layout");
        alloc::alloc(layout) as *mut $ty
    }};
}
macro_rules! REALLOC {
    ($ptr:expr, $size:expr $(,)?) => {{
        let layout = Layout::from_size_align($size as usize, 1)
            .expect("failed to create Layout");
        alloc::realloc($ptr as *mut u8, layout, $size as usize) as *mut c_void
    }};
    ($ptr:expr => [$ty:ty; $n:expr]) => {{
        // FIXME: we should pass the old `Layout` to `realloc`,
        // but we don't have it
        let layout = Layout::array::<$ty>($n as usize)
            .expect("failed to create array Layout");
        alloc::realloc($ptr as *mut u8, layout, layout.size()) as *mut $ty
    }};
}
macro_rules! FREE {
    ($ptr:expr $(,)?) => {
        // FIXME: get the actual layout somehow
        alloc::dealloc($ptr as *mut u8, Layout::new::<u8>())
    };
}

// TODO: move this closer to the definition of `tag`,
// it's only here because it needs the definition of `FREE`
impl Drop for Tag {
    fn drop(&mut self) {
        unsafe {
            FREE!(self.buf);
            destroyBindings(self.bindings);
        }
    }
}

/* Constructs a new parser; encoding is the encoding specified by the
   external protocol or NULL if there is none specified.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate<'scf>(
    mut encodingName: *const XML_Char
) -> XML_Parser<'scf> {
    XML_ParserCreate_MM(
        encodingName,
        None,
        ptr::null(),
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
pub unsafe extern "C" fn XML_ParserCreateNS<'scf>(
    mut encodingName: *const XML_Char,
    mut nsSep: XML_Char,
) -> XML_Parser<'scf> {
    let mut tmp: [XML_Char; 2] = [0; 2];
    tmp[0] = nsSep;
    XML_ParserCreate_MM(
        encodingName,
        None,
        tmp.as_mut_ptr(),
    )
}

const implicitContext: [XML_Char; 41] = XML_STR![
    ASCII_x, ASCII_m, ASCII_l, ASCII_EQUALS, ASCII_h, ASCII_t, ASCII_t, ASCII_p, ASCII_COLON,
    ASCII_SLASH, ASCII_SLASH, ASCII_w, ASCII_w, ASCII_w, ASCII_PERIOD, ASCII_w, ASCII_3,
    ASCII_PERIOD, ASCII_o, ASCII_r, ASCII_g, ASCII_SLASH, ASCII_X, ASCII_M, ASCII_L, ASCII_SLASH,
    ASCII_1, ASCII_9, ASCII_9, ASCII_8, ASCII_SLASH, ASCII_n, ASCII_a, ASCII_m, ASCII_e, ASCII_s,
    ASCII_p, ASCII_a, ASCII_c, ASCII_e,
];

/* To avoid warnings about unused functions: */

impl<'scf> XML_ParserStruct<'scf> {
    /* only valid for root parser */
    unsafe fn startParsing(&mut self) -> XML_Bool {
        /* hash functions must be initialized before setContext() is called */
        if self.m_ns {
            /* implicit context only set for root parser, since child
               parsers (i.e. external entity parsers) will inherit it
            */
            return self.setContext(implicitContext.as_ptr());
        }
        true
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
    if memsuite.is_some() {
        unimplemented!("custom memory allocators are not supported");
    }
    XML_ParserStruct::create(encodingName, nameSep, ptr::null_mut())
}

impl<'scf> XML_ParserStruct<'scf> {
    fn try_new(use_namespaces: bool) -> Result<Self, ()> {
        Ok(Self {
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
            m_ns: false,
            m_ns_triplets: false,
            m_prologState: super::xmlrole::PROLOG_STATE::default(),
            m_processor: None,
            m_errorCode: XML_Error::NONE,
            m_eventPtr: ptr::null(),
            m_eventEndPtr: ptr::null(),
            m_positionPtr: ptr::null(),
            m_openInternalEntities: ptr::null_mut(),
            m_freeInternalEntities: ptr::null_mut(),
            m_defaultExpandInternalEntities: false,
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
            m_declAttributeIsCdata: false,
            m_declAttributeIsId: false,
            m_dtd: ptr::null_mut(),
            m_curBase: ptr::null(),
            m_tagStack: None,
            m_freeTagList: None,
            m_inheritedBindings: ptr::null_mut(),
            m_freeBindingList: ptr::null_mut(),
            m_nSpecifiedAtts: 0,
            m_idAttIndex: 0,
            m_atts: Vec::new(),
            typed_atts: Vec::new(),
            m_nsAtts: HashSet::new(),
            m_position: super::xmltok::Position::default(),
            m_tempPool: StringPool::try_new()?,
            m_temp2Pool: StringPool::try_new()?,
            m_groupConnector: ptr::null_mut(),
            m_groupSize: 0,
            m_namespaceSeparator: 0,
            m_parentParser: ptr::null_mut(),
            m_parsingStatus: XML_ParsingStatus::default(),
            m_isParamEntity: false,
            m_useForeignDTD: false,
            m_paramEntityParsing: XML_ParamEntityParsing::NEVER,

            #[cfg(feature = "mozilla")]
            m_mismatch: ptr::null(),
        })
    }

    unsafe fn create(
        mut encodingName: *const XML_Char,
        mut nameSep: *const XML_Char,
        mut dtd: *mut DTD,
    ) -> XML_Parser {
        let use_namespaces = !nameSep.is_null();
        let mut parser = match XML_ParserStruct::try_new(use_namespaces) {
            Ok(parser) => parser,
            Err(()) => return ptr::null_mut(),
        };

        let mut parser = match Box::try_new(parser) {
            Ok(p) => p,
            Err(_) => return ptr::null_mut(),
        };

        // TODO: Move initialization into XML_ParserStruct::try_new
        parser.m_buffer = ptr::null_mut();
        parser.m_bufferLim = ptr::null();
        if parser.m_atts.try_reserve(INIT_ATTS_SIZE as usize).is_err() {
            return ptr::null_mut();
        }
        if parser.typed_atts.try_reserve(INIT_ATTS_SIZE as usize).is_err() {
            return ptr::null_mut();
        }
        parser.m_dataBuf = MALLOC![XML_Char; INIT_DATA_BUF_SIZE];
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
        parser.m_freeBindingList = ptr::null_mut();
        parser.m_freeTagList = None;
        parser.m_freeInternalEntities = ptr::null_mut();
        parser.m_groupSize = 0;
        parser.m_groupConnector = ptr::null_mut();
        parser.m_initEncoding = None;
        parser.m_handlers.m_unknownEncoding = None;
        parser.m_namespaceSeparator = ASCII_EXCL as XML_Char;
        parser.m_ns = false;
        parser.m_ns_triplets = false;
        parser.m_protocolEncodingName = ptr::null();
        parser.init(encodingName);
        if !encodingName.is_null() && parser.m_protocolEncodingName.is_null() {
            return ptr::null_mut();
        }
        if !nameSep.is_null() {
            parser.m_ns = true;
            parser.m_namespaceSeparator = *nameSep
        }

        #[cfg(feature = "mozilla")]
        {
            parser.m_mismatch = ptr::null();
        }
        Box::into_raw(parser)
    }

    unsafe fn init(&mut self, mut encodingName: *const XML_Char) {
        self.m_processor = Some(prologInitProcessor as Processor);
        super::xmlrole::XmlPrologStateInit(&mut self.m_prologState);
        if !encodingName.is_null() {
            self.m_protocolEncodingName = copy_c_string(encodingName);
        }
        self.m_curBase = ptr::null();
        self.m_initEncoding = InitEncoding::new(&mut self.m_encoding, ptr::null());
        self.m_encoding = &*self.m_initEncoding.as_ref().unwrap();
        self.m_userData = ptr::null_mut();
        self.m_handlers = Default::default();
        self.m_handlers.m_externalEntityRefHandlerArg = self as XML_Parser;
        self.m_bufferPtr = self.m_buffer;
        self.m_bufferEnd = self.m_buffer;
        self.m_parseEndByteIndex = 0;
        self.m_parseEndPtr = ptr::null();
        self.m_declElementType = ptr::null_mut();
        self.m_declAttributeId = ptr::null_mut();
        self.m_declEntity = ptr::null_mut();
        self.m_doctypeName = ptr::null();
        self.m_doctypeSysid = ptr::null();
        self.m_doctypePubid = ptr::null();
        self.m_declAttributeType = ptr::null();
        self.m_declNotationName = ptr::null();
        self.m_declNotationPublicId = ptr::null();
        self.m_declAttributeIsCdata = false;
        self.m_declAttributeIsId = false;
        memset(
            &mut self.m_position as *mut super::xmltok::Position as *mut c_void,
            0,
            ::std::mem::size_of::<super::xmltok::Position>(),
        );
        self.m_errorCode = XML_Error::NONE;
        self.m_eventPtr = ptr::null();
        self.m_eventEndPtr = ptr::null();
        self.m_positionPtr = ptr::null();
        self.m_openInternalEntities = ptr::null_mut();
        self.m_defaultExpandInternalEntities = true;
        self.m_tagLevel = 0;
        self.m_tagStack = None;
        self.m_inheritedBindings = ptr::null_mut();
        self.m_nSpecifiedAtts = 0;
        self.m_handlers.m_unknownEncoding = None;
        self.m_handlers.m_unknownEncodingRelease = None;
        self.m_handlers.m_unknownEncodingData = ptr::null_mut();
        self.m_parentParser = ptr::null_mut();
        self.m_parsingStatus.parsing = XML_Parsing::INITIALIZED;
        self.m_isParamEntity = false;
        self.m_useForeignDTD = false;
        self.m_paramEntityParsing = XML_ParamEntityParsing::NEVER;
    }

    /* moves list of bindings to m_freeBindingList */
    unsafe fn moveToFreeBindingList(&mut self, mut bindings: *mut Binding) {
        while !bindings.is_null() {
            let mut b: *mut Binding = bindings;
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
impl<'scf> XML_ParserStruct<'scf> {
    pub unsafe fn reset(&mut self, encodingName: *const XML_Char) -> XML_Bool {
        let mut openEntityList: *mut OpenInternalEntity = ptr::null_mut();
        if !self.m_parentParser.is_null() {
            return false;
        }
        /* move m_tagStack to m_freeTagList */
        let mut tStk = self.m_tagStack.take();
        while let Some(mut tag) = tStk {
            self.moveToFreeBindingList(tag.bindings);
            tag.bindings = ptr::null_mut();

            let new_parent = self.m_freeTagList.take();
            tStk = std::mem::replace(&mut tag.parent, new_parent);
            self.m_freeTagList = Some(tag);
        }
        /* move m_openInternalEntities to m_freeInternalEntities */
        openEntityList = self.m_openInternalEntities;
        while !openEntityList.is_null() {
            let mut openEntity: *mut OpenInternalEntity = openEntityList;
            openEntityList = (*openEntity).next;
            (*openEntity).next = self.m_freeInternalEntities;
            self.m_freeInternalEntities = openEntity
        }
        self.moveToFreeBindingList(self.m_inheritedBindings);
        let _ = self.m_handlers.m_unknownEncoding.take();
        if self.m_handlers.m_unknownEncodingRelease.is_some() {
            self.m_handlers.m_unknownEncodingRelease
                .expect("non-null function pointer")(self.m_handlers.m_unknownEncodingData);
        }
        self.m_tempPool.clear();
        self.m_temp2Pool.clear();
        FREE!(self.m_protocolEncodingName);
        self.m_protocolEncodingName = ptr::null();
        self.init(encodingName);
        dtdReset(self.m_dtd);
        true
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserReset(parser: XML_Parser, encodingName: *const XML_Char) -> XML_Bool {
    if parser.is_null() {
        return false;
    }
    (*parser).reset(encodingName)
}
/* Returns the last value set by XML_SetUserData or NULL. */
/* This is equivalent to supplying an encoding argument to
   XML_ParserCreate. On success XML_SetEncoding returns non-zero,
   zero otherwise.
   Note: Calling XML_SetEncoding after XML_Parse or XML_ParseBuffer
     has no effect and returns XML_Status::ERROR.
*/

impl<'scf> XML_ParserStruct<'scf> {
    pub unsafe fn setEncoding(&mut self, encodingName: *const XML_Char) -> XML_Status {
        /* Block after XML_Parse()/XML_ParseBuffer() has been called.
        XXX There's no way for the caller to determine which of the
        XXX possible error cases caused the XML_Status::ERROR return.
        */
        if self.m_parsingStatus.parsing == XML_Parsing::PARSING
            || self.m_parsingStatus.parsing == XML_Parsing::SUSPENDED
        {
            return XML_Status::ERROR;
        }

        /* Get rid of any previous encoding name */
        FREE!(self.m_protocolEncodingName);
        if encodingName.is_null() {
            /* No new encoding name */
            self.m_protocolEncodingName = ptr::null()
        } else {
            /* Copy the new encoding name into allocated memory */
            self.m_protocolEncodingName = copy_c_string(encodingName);
            if self.m_protocolEncodingName.is_null() {
                return XML_Status::ERROR;
            }
        }
        XML_Status::OK
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEncoding(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Status {
    if parser.is_null() {
        return XML_Status::ERROR;
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
    let mut newDtd: *mut DTD = ptr::null_mut();
    let mut oldDtd: *mut DTD = ptr::null_mut();
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
    let mut oldDeclElementType: *mut ElementType = ptr::null_mut();
    let mut oldUserData: *mut c_void = ptr::null_mut();
    let mut oldHandlerArg: *mut c_void = ptr::null_mut();
    let mut oldDefaultExpandInternalEntities = false;
    let mut oldExternalEntityRefHandlerArg: XML_Parser = ptr::null_mut();
    let mut oldParamEntityParsing: XML_ParamEntityParsing = XML_ParamEntityParsing::NEVER;
    let mut oldInEntityValue: c_int = 0;
    let mut oldns_triplets = false;
    /* Note that the new parser shares the same hash secret as the old
       parser, so that dtdCopy and copyEntityTable can lookup values
       from hash tables associated with either parser without us having
       to worry which hash secrets each table has.
    */
    /* Validate the oldParser parameter before we pull everything out of it */
    if oldParser.is_null() {
        return ptr::null_mut();
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
    if context.is_null() {
        newDtd = oldDtd
    }
    /* XML_DTD */
    /* Note that the magical uses of the pre-processor to make field
       access look more like C++ require that `parser' be overwritten
       here.  This makes this function more painful to follow than it
       would be otherwise.
    */
    if (*parser).m_ns {
        let mut tmp: [XML_Char; 2] = [0; 2];
        *tmp.as_mut_ptr() = (*parser).m_namespaceSeparator;
        parser = XML_ParserStruct::create(encodingName, tmp.as_mut_ptr(), newDtd)
    } else {
        parser = XML_ParserStruct::create(
            encodingName,
            ptr::null(),
            newDtd,
        )
    }
    if parser.is_null() {
        return ptr::null_mut();
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
    (*parser).m_parentParser = oldParser;
    (*parser).m_paramEntityParsing = oldParamEntityParsing;
    (*parser).m_prologState.inEntityValue = oldInEntityValue;
    if !context.is_null() {
        /* XML_DTD */
        if dtdCopy((*parser).m_dtd, oldDtd) == 0
            || !(*parser).setContext(context)
        {
            XML_ParserFree(parser);
            return ptr::null_mut();
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
        (*parser).m_isParamEntity = true;
        super::xmlrole::XmlPrologStateInitExternalEntity(&mut (*parser).m_prologState);
        (*parser).m_processor = Some(externalParEntInitProcessor as Processor)
    }
    /* XML_DTD */
    parser
}

unsafe fn destroyBindings(mut bindings: *mut Binding) {
    loop {
        let mut b: *mut Binding = bindings;
        if b.is_null() {
            break;
        }
        bindings = (*b).nextTagBinding;
        FREE!((*b).uri);
        FREE!(b);
    }
}

impl<'scf> Drop for XML_ParserStruct<'scf> {
    /* Frees memory used by the parser. */
    fn drop(&mut self) {
        let mut entityList: *mut OpenInternalEntity = ptr::null_mut();
        unsafe {
            /* free m_openInternalEntities and m_freeInternalEntities */
            entityList = self.m_openInternalEntities;
            loop {
                let mut openEntity: *mut OpenInternalEntity = ptr::null_mut();
                if entityList.is_null() {
                    if self.m_freeInternalEntities.is_null() {
                        break;
                    }
                    entityList = self.m_freeInternalEntities;
                    self.m_freeInternalEntities = ptr::null_mut()
                }
                openEntity = entityList;
                entityList = (*entityList).next;
                FREE!(openEntity);
            }
            destroyBindings(self.m_freeBindingList);
            destroyBindings(self.m_inheritedBindings);
            FREE!(self.m_protocolEncodingName);
            /* external parameter entity parsers share the DTD structure
            parser->m_dtd with the root parser, so we must not destroy it
            */
            if !self.m_isParamEntity && !self.m_dtd.is_null() {
                /* XML_DTD */
                dtdDestroy(self.m_dtd);
            }
            FREE!(self.m_groupConnector);
            FREE!(self.m_buffer);
            FREE!(self.m_dataBuf);
            if self.m_handlers.m_unknownEncodingRelease.is_some() {
                self.m_handlers.m_unknownEncodingRelease
                    .expect("non-null function pointer")(self.m_handlers.m_unknownEncodingData);
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
/* If useDTD == true is passed to this function, then the parser
   will assume that there is an external subset, even if none is
   specified in the document. In such a case the parser will call the
   externalEntityRefHandler with a value of NULL for the systemId
   argument (the publicId and context arguments will be NULL as well).
   Note: For the purpose of checking WFC: Entity Declared, passing
     useDTD == true will make the parser behave as if the document
     had a DTD with an external subset.
   Note: If this function is called, then this must be done before
     the first call to XML_Parse or XML_ParseBuffer, since it will
     have no effect after that.  Returns
     XML_Error::CANT_CHANGE_FEATURE_ONCE_PARSING.
   Note: If the document does not have a DOCTYPE declaration at all,
     then startDoctypeDeclHandler and endDoctypeDeclHandler will not
     be called, despite an external subset being parsed.
   Note: If XML_DTD is not defined when Expat is compiled, returns
     XML_Error::FEATURE_REQUIRES_XML_DTD.
   Note: If parser == NULL, returns XML_Error::INVALID_ARGUMENT.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_UseForeignDTD(
    mut parser: XML_Parser,
    mut useDTD: XML_Bool,
) -> XML_Error {
    if parser.is_null() {
        return XML_Error::INVALID_ARGUMENT;
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing == XML_Parsing::PARSING
        || (*parser).m_parsingStatus.parsing == XML_Parsing::SUSPENDED
    {
        return XML_Error::CANT_CHANGE_FEATURE_ONCE_PARSING;
    }
    (*parser).m_useForeignDTD = useDTD;
    XML_Error::NONE
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
    if (*parser).m_parsingStatus.parsing == XML_Parsing::PARSING
        || (*parser).m_parsingStatus.parsing == XML_Parsing::SUSPENDED
    {
        return;
    }
    (*parser).m_ns_triplets = do_nst;
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
   argument will be copied.  Returns XML_Status::ERROR if out of memory,
   XML_Status::OK otherwise.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_SetBase(mut parser: XML_Parser, mut p: *const XML_Char) -> XML_Status {
    if parser.is_null() {
        return XML_Status::ERROR;
    }
    if !p.is_null() {
        let p = match (*(*parser).m_dtd).pool.copy_c_string(p) {
            Some(p) => p,
            None => return XML_Status::ERROR,
        };
        (*parser).m_curBase = p.as_ptr();
    } else {
        (*parser).m_curBase = ptr::null();
    }
    XML_Status::OK
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetBase(mut parser: XML_Parser) -> *const XML_Char {
    if parser.is_null() {
        return ptr::null();
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
        return -1;
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
        return -1;
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
    (*parser).m_defaultExpandInternalEntities = false;
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
    (*parser).m_defaultExpandInternalEntities = true;
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
        return 0;
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing == XML_Parsing::PARSING
        || (*parser).m_parsingStatus.parsing == XML_Parsing::SUSPENDED
    {
        return 0;
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
        return 0;
    }
    if !(*parser).m_parentParser.is_null() {
        return XML_SetHashSalt((*parser).m_parentParser, hash_salt);
    }
    /* block after XML_Parse()/XML_ParseBuffer() has been called */
    if (*parser).m_parsingStatus.parsing == XML_Parsing::PARSING
        || (*parser).m_parsingStatus.parsing == XML_Parsing::SUSPENDED
    {
        return 0;
    }
    // FIXME
    //return 1;
    0
}
/* Parses some input. Returns XML_Status::ERROR if a fatal error is
   detected.  The last call to XML_Parse must have isFinal true; len
   may be zero for this call (or any other).

   Though the return values for these functions has always been
   described as a Boolean value, the implementation, at least for the
   1.95.x series, has always returned exactly one of the XML_Status
   values.
*/

impl<'scf> XML_ParserStruct<'scf> {
    pub unsafe fn parse(&mut self, s: *const c_char, len: c_int, isFinal: c_int) -> XML_Status {
        if len < 0 || s.is_null() && len != 0 {
            return XML_Status::ERROR;
        }
        match self.m_parsingStatus.parsing {
            XML_Parsing::SUSPENDED => {
                self.m_errorCode = XML_Error::SUSPENDED;
                return XML_Status::ERROR;
            }
            XML_Parsing::FINISHED => {
                self.m_errorCode = XML_Error::FINISHED;
                return XML_Status::ERROR;
            }
            XML_Parsing::INITIALIZED => {
                if self.m_parentParser.is_null() && !self.startParsing() {
                    self.m_errorCode = XML_Error::NO_MEMORY;
                    return XML_Status::ERROR;
                }
            }
            _ => {}
        }
        /* fall through */
        self.m_parsingStatus.parsing = XML_Parsing::PARSING;
        if len == 0 {
            self.m_parsingStatus.finalBuffer = isFinal != 0;
            if isFinal == 0 {
                return XML_Status::OK;
            }
            self.m_positionPtr = self.m_bufferPtr;
            self.m_parseEndPtr = self.m_bufferEnd;
            /* If data are left over from last buffer, and we now know that these
            data are the final chunk of input, then we have to check them again
            to detect errors based on that fact.
            */
            self.m_errorCode = self.m_processor.expect("non-null function pointer")(
                self,
                ExpatBufRef::new(
                    self.m_bufferPtr,
                    self.m_parseEndPtr,
                ),
                &mut self.m_bufferPtr,
            );
            if self.m_errorCode == XML_Error::NONE {
                match self.m_parsingStatus.parsing {
                    XML_Parsing::SUSPENDED => {
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
                            ExpatBufRef::new(
                                self.m_positionPtr,
                                self.m_bufferPtr,
                            ),
                            &mut self.m_position,
                        );
                        self.m_positionPtr = self.m_bufferPtr;
                        return XML_Status::SUSPENDED;
                    }
                    XML_Parsing::INITIALIZED | XML_Parsing::PARSING => {
                        /* LCOV_EXCL_STOP */
                        self.m_parsingStatus.parsing = XML_Parsing::FINISHED
                    }
                    _ => {}
                }
                /* fall through */
                return XML_Status::OK;
            }
            self.m_eventEndPtr = self.m_eventPtr;
            self.m_processor = Some(errorProcessor as Processor);
            XML_Status::ERROR
        } else {
            /* not defined XML_CONTEXT_BYTES */
            let mut buff: *mut c_void = self.getBuffer(len);
            if buff.is_null() {
                XML_Status::ERROR as XML_Status
            } else {
                memcpy(buff, s as *const c_void, len as usize);
                self.parseBuffer(len, isFinal)
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
            (*parser).m_errorCode = XML_Error::INVALID_ARGUMENT
        }
        return XML_Status::ERROR;
    }
    (*parser).parse(s, len, isFinal)
}

impl<'scf> XML_ParserStruct<'scf> {
    pub unsafe fn parseBuffer(&mut self, len: c_int, isFinal: c_int) -> XML_Status {
        let mut start: *const c_char = ptr::null();
        let mut result: XML_Status = XML_Status::OK;
        match self.m_parsingStatus.parsing {
            XML_Parsing::SUSPENDED => {
                self.m_errorCode = XML_Error::SUSPENDED;
                return XML_Status::ERROR as XML_Status;
            }
            XML_Parsing::FINISHED => {
                self.m_errorCode = XML_Error::FINISHED;
                return XML_Status::ERROR as XML_Status;
            }
            XML_Parsing::INITIALIZED => {
                if self.m_parentParser.is_null() && !self.startParsing() {
                    self.m_errorCode = XML_Error::NO_MEMORY;
                    return XML_Status::ERROR;
                }
            }
            _ => {}
        }
        /* fall through */
        self.m_parsingStatus.parsing = XML_Parsing::PARSING;
        start = self.m_bufferPtr;
        self.m_positionPtr = start;
        self.m_bufferEnd = self.m_bufferEnd.offset(len as isize);
        self.m_parseEndPtr = self.m_bufferEnd;
        self.m_parseEndByteIndex += len as c_long;
        self.m_parsingStatus.finalBuffer = isFinal != 0;
        self.m_errorCode = self.m_processor.expect("non-null function pointer")(
            self,
            ExpatBufRef::new(
                start,
                self.m_parseEndPtr,
            ),
            &mut self.m_bufferPtr,
        );

        if self.m_errorCode != XML_Error::NONE {
            self.m_eventEndPtr = self.m_eventPtr;
            self.m_processor = Some(errorProcessor as Processor);
            return XML_Status::ERROR as XML_Status;
        } else {
            match self.m_parsingStatus.parsing {
                XML_Parsing::SUSPENDED => {
                    result = XML_Status::SUSPENDED;
                    /* should not happen */
                }
                XML_Parsing::INITIALIZED | XML_Parsing::PARSING => {
                    if isFinal != 0 {
                        self.m_parsingStatus.parsing = XML_Parsing::FINISHED;
                        return result;
                    }
                }
                _ => {}
            }
        }
        (*self.m_encoding).updatePosition(
            ExpatBufRef::new(
                self.m_positionPtr,
                self.m_bufferPtr,
            ),
            &mut self.m_position,
        );
        self.m_positionPtr = self.m_bufferPtr;

        result
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParseBuffer(
    mut parser: XML_Parser,
    mut len: c_int,
    mut isFinal: c_int,
) -> XML_Status {
    if parser.is_null() {
        return XML_Status::ERROR;
    }

    (*parser).parseBuffer(len, isFinal)
}

impl<'scf> XML_ParserStruct<'scf> {
    pub unsafe fn getBuffer(&mut self, len: c_int) -> *mut c_void {
        if len < 0 {
            self.m_errorCode = XML_Error::NO_MEMORY;
            return ptr::null_mut();
        }
        match self.m_parsingStatus.parsing {
            XML_Parsing::SUSPENDED => {
                self.m_errorCode = XML_Error::SUSPENDED;
                return ptr::null_mut();
            }
            XML_Parsing::FINISHED => {
                self.m_errorCode = XML_Error::FINISHED;
                return ptr::null_mut();
            }
            _ => {}
        }
        if len as isize > safe_ptr_diff(self.m_bufferLim, self.m_bufferEnd) {
            let maybe_needed_size = len.checked_add(
                safe_ptr_diff(self.m_bufferEnd, self.m_bufferPtr) as c_int
            );
            let mut neededSize = match maybe_needed_size {
                None => {
                    self.m_errorCode = XML_Error::NO_MEMORY;
                    return ptr::null_mut();
                }
                Some(s) => s,
            };
            let keep = cmp::min(
                XML_CONTEXT_BYTES,
                safe_ptr_diff(self.m_bufferPtr, self.m_buffer) as c_int,
            );
            neededSize += keep;
            if (neededSize as isize) <= safe_ptr_diff(self.m_bufferLim, self.m_buffer) {
                if (keep as isize) < safe_ptr_diff(self.m_bufferPtr, self.m_buffer) {
                    let offset = safe_ptr_diff(self.m_bufferPtr, self.m_buffer) - keep as isize;
                    /* The buffer pointers cannot be NULL here; we have at least some bytes
                     * in the buffer */
                    memmove(
                        self.m_buffer as *mut c_void,
                        &mut *self.m_buffer.offset(offset as isize) as *mut c_char
                            as *const c_void,
                        (self
                            .m_bufferEnd
                            .wrapping_offset_from(self.m_bufferPtr)
                            + keep as isize).try_into().unwrap(),
                    );
                    self.m_bufferEnd = self.m_bufferEnd.offset(-offset);
                    self.m_bufferPtr = self.m_bufferPtr.offset(-offset);
                }
            } else {
                let mut bufferSize: c_int = match safe_ptr_diff(self.m_bufferLim, self.m_bufferPtr) {
                    0 => INIT_BUFFER_SIZE,
                    size => size.try_into().unwrap(),
                };
                while bufferSize < neededSize {
                    bufferSize = match 2i32.checked_mul(bufferSize) {
                        Some(s) => s,
                        None => {
                            self.m_errorCode = XML_Error::NO_MEMORY;
                            return ptr::null_mut();
                        }
                    }
                }
                let newBuf = MALLOC![c_char; bufferSize];
                if newBuf.is_null() {
                    self.m_errorCode = XML_Error::NO_MEMORY;
                    return ptr::null_mut();
                }
                self.m_bufferLim = newBuf.offset(bufferSize as isize);
                if !self.m_bufferPtr.is_null() {
                    memcpy(
                        newBuf as *mut c_void,
                        &*self.m_bufferPtr.offset(-keep as isize) as *const c_char
                            as *const c_void,
                        safe_ptr_diff(self.m_bufferEnd, self.m_bufferPtr) as usize + keep as usize,
                    );
                    FREE!(self.m_buffer);
                    self.m_buffer = newBuf;
                    self.m_bufferEnd = self
                        .m_buffer
                        .offset(safe_ptr_diff(self.m_bufferEnd, self.m_bufferPtr))
                        .offset(keep as isize);
                    self.m_bufferPtr = self.m_buffer.offset(keep as isize)
                } else {
                    /* This must be a brand new buffer with no data in it yet */
                    self.m_bufferEnd = newBuf;
                    self.m_buffer = newBuf;
                    self.m_bufferPtr = self.m_buffer
                }
            }
            self.m_eventEndPtr = ptr::null();
            self.m_eventPtr = ptr::null();
            self.m_positionPtr = ptr::null();
        }
        self.m_bufferEnd as *mut c_void
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetBuffer(mut parser: XML_Parser, mut len: c_int) -> *mut c_void {
    if parser.is_null() {
        return ptr::null_mut();
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
   Returns XML_Status::OK when successful, XML_Status::ERROR otherwise.
   Possible error codes:
   - XML_Error::SUSPENDED: when suspending an already suspended parser.
   - XML_Error::FINISHED: when the parser has already finished.
   - XML_Error::SUSPEND_PE: when suspending while parsing an external PE.

   When resumable != 0 (true) then parsing is suspended, that is,
   XML_Parse() and XML_ParseBuffer() return XML_STATUS_SUSPENDED.
   Otherwise, parsing is aborted, that is, XML_Parse() and XML_ParseBuffer()
   return XML_Status::ERROR with error code XML_Error::ABORTED.

   *Note*:
   This will be applied to the current parser instance only, that is, if
   there is a parent parser then it will continue parsing when the
   externalEntityRefHandler() returns. It is up to the implementation of
   the externalEntityRefHandler() to call XML_StopParser() on the parent
   parser (recursively), if one wants to stop parsing altogether.

   When suspended, parsing can be resumed by calling XML_ResumeParser().
*/

impl<'scf> XML_ParserStruct<'scf> {
    pub unsafe fn stopParser(&mut self, resumable: XML_Bool) -> XML_Status {
        match self.m_parsingStatus.parsing {
            XML_Parsing::SUSPENDED => {
                if resumable {
                    self.m_errorCode = XML_Error::SUSPENDED;
                    return XML_Status::ERROR;
                }
                self.m_parsingStatus.parsing = XML_Parsing::FINISHED
            }
            XML_Parsing::FINISHED => {
                self.m_errorCode = XML_Error::FINISHED;
                return XML_Status::ERROR;
            }
            _ => {
                if resumable {
                    if self.m_isParamEntity {
                        self.m_errorCode = XML_Error::SUSPEND_PE;
                        return XML_Status::ERROR;
                    }
                    self.m_parsingStatus.parsing = XML_Parsing::SUSPENDED
                } else {
                    self.m_parsingStatus.parsing = XML_Parsing::FINISHED
                }
            }
        }
        XML_Status::OK
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_StopParser(parser: XML_Parser, resumable: XML_Bool) -> XML_Status {
    if parser.is_null() {
        return XML_Status::ERROR;
    }
    (*parser).stopParser(resumable)
}
/* Resumes parsing after it has been suspended with XML_StopParser().
   Must not be called from within a handler call-back. Returns same
   status codes as XML_Parse() or XML_ParseBuffer().
   Additional error code XML_Error::NOT_SUSPENDED possible.

   *Note*:
   This must be called on the most deeply nested child parser instance
   first, and on its parent parser only after the child parser has finished,
   to be applied recursively until the document entity's parser is restarted.
   That is, the parent parser will not resume by itself and it is up to the
   application to call XML_ResumeParser() on it at the appropriate moment.
*/
impl<'scf> XML_ParserStruct<'scf> {
    pub unsafe fn resumeParser(&mut self) -> XML_Status {
        let mut result: XML_Status = XML_Status::OK;
        if self.m_parsingStatus.parsing != XML_Parsing::SUSPENDED {
            self.m_errorCode = XML_Error::NOT_SUSPENDED;
            return XML_Status::ERROR;
        }
        self.m_parsingStatus.parsing = XML_Parsing::PARSING;
        self.m_errorCode = self.m_processor.expect("non-null function pointer")(
            self,
            ExpatBufRef::new(
                self.m_bufferPtr,
                self.m_parseEndPtr,
            ),
            &mut self.m_bufferPtr,
        );
        if self.m_errorCode != XML_Error::NONE {
            self.m_eventEndPtr = self.m_eventPtr;
            self.m_processor = Some(errorProcessor as Processor);
            return XML_Status::ERROR;
        } else {
            match self.m_parsingStatus.parsing {
                XML_Parsing::SUSPENDED => result = XML_Status::SUSPENDED,
                XML_Parsing::INITIALIZED | XML_Parsing::PARSING => {
                    if self.m_parsingStatus.finalBuffer {
                        self.m_parsingStatus.parsing = XML_Parsing::FINISHED;
                        return result;
                    }
                }
                _ => {}
            }
        }
        (*self.m_encoding).updatePosition(
            ExpatBufRef::new(
                self.m_positionPtr,
                self.m_bufferPtr,
            ),
            &mut self.m_position,
        );
        self.m_positionPtr = self.m_bufferPtr;

        #[cfg(feature = "mozilla")]
        {
            self.m_eventPtr = self.m_bufferPtr;
            self.m_eventEndPtr = self.m_bufferPtr;
        }

        result
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ResumeParser(mut parser: XML_Parser) -> XML_Status {
    if parser.is_null() {
        return XML_Status::ERROR as XML_Status;
    }

    (*parser).resumeParser()
}
/* Returns status of parser with respect to being initialized, parsing,
   finished, or suspended and processing the final buffer.
   XXX XML_Parse() and XML_ParseBuffer() should return XML_ParsingStatus,
   XXX with XML_Parsing::FINISHED_OK or XML_Parsing::FINISHED_ERROR replacing XML_Parsing::FINISHED
*/
#[no_mangle]
pub unsafe extern "C" fn XML_GetParsingStatus(
    mut parser: XML_Parser,
    mut status: *mut XML_ParsingStatus,
) {
    if parser.is_null() {
        return;
    }
    assert!(!status.is_null());
    *status = (*parser).m_parsingStatus;
}
/* If XML_Parse or XML_ParseBuffer have returned XML_Status::ERROR, then
   XML_GetErrorCode returns information about the error.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_GetErrorCode(mut parser: XML_Parser) -> XML_ErrorCode {
    XML_GetError(parser).code()
}
pub unsafe fn XML_GetError(mut parser: XML_Parser) -> XML_Error {
    if let Some(parser) = parser.as_ref() {
        parser.m_errorCode
    } else { XML_Error::INVALID_ARGUMENT }
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteIndex(mut parser: XML_Parser) -> XML_Index {
    if parser.is_null() {
        return -1;
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
    -1
}
/* Return the number of bytes in the current event.
   Returns 0 if the event is in an internal entity.
*/
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteCount(mut parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return 0;
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
        return ptr::null();
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
   or XML_ParseBuffer.  If the return value is XML_Status::ERROR then
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
        return 0;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        (*(*parser).m_encoding).updatePosition(
            ExpatBufRef::new(
                (*parser).m_positionPtr,
                (*parser).m_eventPtr,
            ),
            &mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr
    }
    (*parser).m_position.lineNumber.wrapping_add(1)
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentColumnNumber(mut parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        (*(*parser).m_encoding).updatePosition(
            ExpatBufRef::new(
                (*parser).m_positionPtr,
                (*parser).m_eventPtr,
            ),
            &mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr
    }
    (*parser).m_position.columnNumber
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
        return ptr::null_mut();
    }
    MALLOC!(size)
}
#[no_mangle]
pub unsafe extern "C" fn XML_MemRealloc(
    mut parser: XML_Parser,
    mut ptr: *mut c_void,
    mut size: size_t,
) -> *mut c_void {
    if parser.is_null() {
        return ptr::null_mut();
    }
    REALLOC!(ptr, size)
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
                ExpatBufRef::new(
                    (*(*parser).m_openInternalEntities).internalEventPtr,
                    (*(*parser).m_openInternalEntities).internalEventEndPtr,
                ),
            );
        } else {
            reportDefault(
                parser,
                EncodingType::Normal,
                ExpatBufRef::new(
                    (*parser).m_eventPtr,
                    (*parser).m_eventEndPtr,
                ),
            );
        }
    };
}

impl XML_Error {
    fn description(&self) -> *const XML_LChar {
        match self {
            XML_Error::NONE => ptr::null(),
            XML_Error::NO_MEMORY => wch!("out of memory\x00"),
            XML_Error::SYNTAX => wch!("syntax error\x00"),
            XML_Error::NO_ELEMENTS => wch!("no element found\x00"),
            XML_Error::INVALID_TOKEN => wch!("not well-formed (invalid token)\x00"),
            XML_Error::UNCLOSED_TOKEN => wch!("unclosed token\x00"),
            XML_Error::PARTIAL_CHAR => wch!("partial character\x00"),
            XML_Error::TAG_MISMATCH => wch!("mismatched tag\x00"),
            XML_Error::DUPLICATE_ATTRIBUTE => wch!("duplicate attribute\x00"),
            XML_Error::JUNK_AFTER_DOC_ELEMENT => wch!("junk after document element\x00"),
            XML_Error::PARAM_ENTITY_REF => wch!("illegal parameter entity reference\x00"),
            XML_Error::UNDEFINED_ENTITY => wch!("undefined entity\x00"),
            XML_Error::RECURSIVE_ENTITY_REF => wch!("recursive entity reference\x00"),
            XML_Error::ASYNC_ENTITY => wch!("asynchronous entity\x00"),
            XML_Error::BAD_CHAR_REF => wch!("reference to invalid character number\x00"),
            XML_Error::BINARY_ENTITY_REF => wch!("reference to binary entity\x00"),
            XML_Error::ATTRIBUTE_EXTERNAL_ENTITY_REF => {
                wch!("reference to external entity in attribute\x00")
            }
            XML_Error::MISPLACED_XML_PI => wch!("XML or text declaration not at start of entity\x00"),
            XML_Error::UNKNOWN_ENCODING => wch!("unknown encoding\x00"),
            XML_Error::INCORRECT_ENCODING => {
                wch!("encoding specified in XML declaration is incorrect\x00")
            }
            XML_Error::UNCLOSED_CDATA_SECTION => wch!("unclosed CDATA section\x00"),
            XML_Error::EXTERNAL_ENTITY_HANDLING => {
                wch!("error in processing external entity reference\x00")
            }
            XML_Error::NOT_STANDALONE => wch!("document is not standalone\x00"),
            XML_Error::UNEXPECTED_STATE => {
                wch!("unexpected parser state - please send a bug report\x00")
            }
            XML_Error::ENTITY_DECLARED_IN_PE => wch!("entity declared in parameter entity\x00"),
            XML_Error::FEATURE_REQUIRES_XML_DTD => {
                wch!("requested feature requires XML_DTD support in Expat\x00")
            }
            XML_Error::CANT_CHANGE_FEATURE_ONCE_PARSING => {
                wch!("cannot change setting once parsing has begun\x00")
            }
            /* Added in 1.95.7. */
            XML_Error::UNBOUND_PREFIX => wch!("unbound prefix\x00"),
            /* Added in 1.95.8. */
            XML_Error::UNDECLARING_PREFIX => wch!("must not undeclare prefix\x00"),
            XML_Error::INCOMPLETE_PE => wch!("incomplete markup in parameter entity\x00"),
            XML_Error::XML_DECL => wch!("XML declaration not well-formed\x00"),
            XML_Error::TEXT_DECL => wch!("text declaration not well-formed\x00"),
            XML_Error::PUBLICID => wch!("illegal character(s) in public id\x00"),
            XML_Error::SUSPENDED => wch!("parser suspended\x00"),
            XML_Error::NOT_SUSPENDED => wch!("parser not suspended\x00"),
            XML_Error::ABORTED => wch!("parsing aborted\x00"),
            XML_Error::FINISHED => wch!("parsing finished\x00"),
            XML_Error::SUSPEND_PE => wch!("cannot suspend in external parameter entity\x00"),
            XML_Error::RESERVED_PREFIX_XML => {
                /* Added in 2.0.0. */
                wch!("reserved prefix (xml) must not be undeclared or bound to another namespace name\x00")
            }
            XML_Error::RESERVED_PREFIX_XMLNS => {
                wch!("reserved prefix (xmlns) must not be declared or undeclared\x00")
            }
            XML_Error::RESERVED_NAMESPACE_URI => {
                wch!("prefix must not be bound to one of the reserved namespace names\x00")
            }
            /* Added in 2.2.5. */
            XML_Error::INVALID_ARGUMENT => {
                /* Constant added in 2.2.1, already */
                wch!("invalid argument\x00")
            }
        }
    }
}

/* Returns a string describing the error. */
#[no_mangle]
pub unsafe extern "C" fn XML_ErrorString(mut code: u32) -> *const XML_LChar {
    if let Some(code) = XML_Error::from_u32(code) {
            code.description()
    } else {
        ptr::null()
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
            feature: XML_FeatureEnum::SIZEOF_XML_CHAR,
            name: wch!("sizeof(XML_Char)\x00"),
            value: ::std::mem::size_of::<XML_Char>() as c_long,
        },
        XML_Feature {
            feature: XML_FeatureEnum::SIZEOF_XML_LCHAR,
            name: wch!("sizeof(XML_LChar)\x00"),
            value: ::std::mem::size_of::<XML_LChar>() as c_long,
        },
        #[cfg(feature = "unicode")]
        XML_Feature {
            feature: XML_FeatureEnum::UNICODE,
            name: wch!("XML_UNICODE\x00"),
            value: 0,
        },
        #[cfg(feature = "unicode_wchar_t")]
        XML_Feature {
            feature: XML_FeatureEnum::UNICODE_WCHAR_T,
            name: wch!("XML_UNICODE_WHCAR_T\x00"),
            value: 0,
        },
        XML_Feature {
            feature: XML_FeatureEnum::DTD,
            name: wch!("XML_DTD\x00"),
            value: 0,
        },
        XML_Feature {
            feature: XML_FeatureEnum::CONTEXT_BYTES,
            name: wch!("XML_CONTEXT_BYTES\x00"),
            value: XML_CONTEXT_BYTES as c_long,
        },
        XML_Feature {
            feature: XML_FeatureEnum::NS,
            name: wch!("XML_NS\x00"),
            value: 0,
        },
        XML_Feature {
            feature: XML_FeatureEnum::END,
            name: ptr::null(),
            value: 0,
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
    !(*parser).m_openInternalEntities.is_null()
}

/* Initially tag->rawName always points into the parse buffer;
   for those TAG instances opened while the current parse buffer was
   processed, and not yet closed, we need to store tag->rawName in a more
   permanent location, since the parse buffer is about to be discarded.
*/
impl<'scf> XML_ParserStruct<'scf> {
    unsafe fn storeRawNames(&mut self) -> XML_Bool {
        let mut tStk = &mut self.m_tagStack;
        while let Some(tag) = tStk {
            let mut bufSize: c_int = 0;
            let mut nameLen: c_int = (::std::mem::size_of::<XML_Char>() as c_ulong)
                .wrapping_mul((tag.name.strLen + 1) as c_ulong)
                as c_int;
            let mut rawNameBuf: *mut c_char = tag.buf.offset(nameLen as isize);
            /* Stop if already stored.  Since m_tagStack is a stack, we can stop
            at the first entry that has already been copied; everything
            below it in the stack is already been accounted for in a
            previous call to this function.
            */
            if tag.rawName == rawNameBuf as *const c_char {
                break;
            }
            /* For re-use purposes we need to ensure that the
            size of tag->buf is a multiple of sizeof(XML_Char).
            */
            bufSize = (nameLen as c_ulong).wrapping_add(
                round_up(tag.rawNameLength as usize, mem::size_of::<XML_Char>()) as c_ulong,
            ) as c_int;
            if bufSize as c_long > tag.bufEnd.wrapping_offset_from(tag.buf) as c_long {
                let mut temp = REALLOC!(tag.buf => [c_char; bufSize]);
                if temp.is_null() {
                    return false;
                }
                /* if tag->name.str points to tag->buf (only when namespace
                processing is off) then we have to update it
                */
                if tag.name.str_0 == tag.buf as *const XML_Char {
                    tag.name.str_0 = temp as *const XML_Char
                }
                /* if tag->name.localPart is set (when namespace processing is on)
                then update it as well, since it will always point into tag->buf
                */
                if !tag.name.localPart.is_null() {
                    tag.name.localPart = (temp).offset(
                        tag
                            .name
                            .localPart
                            .wrapping_offset_from(tag.buf as *const XML_Char),
                    ) as *const XML_Char
                } /* XmlContentTok doesn't always set the last arg */
                tag.buf = temp;
                tag.bufEnd = temp.offset(bufSize as isize);
                rawNameBuf = temp.offset(nameLen as isize)
            }
            memcpy(
                rawNameBuf as *mut c_void,
                tag.rawName as *const c_void,
                tag.rawNameLength as usize,
            );
            tag.rawName = rawNameBuf;
            tStk = &mut tag.parent;
        }
        true
    }
}

unsafe extern "C" fn contentProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).doContent(
        0,
        EncodingType::Normal,
        buf,
        endPtr,
        !(*parser).m_parsingStatus.finalBuffer,
    );
    if result == XML_Error::NONE {
        if !(*parser).storeRawNames() {
            return XML_Error::NO_MEMORY;
        }
    }
    result
}

unsafe extern "C" fn externalEntityInitProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).initializeEncoding();
    if result != XML_Error::NONE {
        return result;
    }
    (*parser).m_processor = Some(externalEntityInitProcessor2 as Processor);
    externalEntityInitProcessor2(parser, buf, endPtr)
}

unsafe extern "C" fn externalEntityInitProcessor2(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = buf.as_ptr();
    let mut tok = (*(*parser).m_encoding).xmlTok(XML_STATE::CONTENT, buf, &mut next);
    match tok {
        XML_TOK::BOM => {
            /* If we are at the end of the buffer, this would cause the next stage,
               i.e. externalEntityInitProcessor3, to pass control directly to
               doContent (by detecting XML_TOK::NONE) without processing any xml text
               declaration - causing the error XML_Error::MISPLACED_XML_PI in doContent.
            */
            if next == buf.end() && !(*parser).m_parsingStatus.finalBuffer {
                *endPtr = next; /* XmlContentTok doesn't always set the last arg */
                return XML_Error::NONE;
            }
            buf = buf.with_start(next);
        }
        XML_TOK::PARTIAL => {
            if !(*parser).m_parsingStatus.finalBuffer {
                *endPtr = buf.as_ptr();
                return XML_Error::NONE;
            }
            (*parser).m_eventPtr = buf.as_ptr();
            return XML_Error::UNCLOSED_TOKEN;
        }
        XML_TOK::PARTIAL_CHAR => {
            if !(*parser).m_parsingStatus.finalBuffer {
                *endPtr = buf.as_ptr();
                return XML_Error::NONE;
            }
            (*parser).m_eventPtr = buf.as_ptr();
            return XML_Error::PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(externalEntityInitProcessor3 as Processor);
    externalEntityInitProcessor3(parser, buf, endPtr)
}

unsafe extern "C" fn externalEntityInitProcessor3(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = buf.as_ptr();
    (*parser).m_eventPtr = buf.as_ptr();
    let tok = (*(*parser).m_encoding).xmlTok(XML_STATE::CONTENT, buf, &mut next);
    (*parser).m_eventEndPtr = next;
    match tok {
        XML_TOK::XML_DECL => {
            let mut result: XML_Error = XML_Error::NONE;
            result = (*parser).processXmlDecl(1, buf.with_end(next));
            if result != XML_Error::NONE {
                return result;
            }
            match (*parser).m_parsingStatus.parsing {
                XML_Parsing::SUSPENDED => {
                    *endPtr = next;
                    return XML_Error::NONE;
                }
                XML_Parsing::FINISHED => return XML_Error::ABORTED,
                _ => buf = buf.with_start(next),
            }
        }
        XML_TOK::PARTIAL => {
            if !(*parser).m_parsingStatus.finalBuffer {
                *endPtr = buf.as_ptr();
                return XML_Error::NONE;
            }
            return XML_Error::UNCLOSED_TOKEN;
        }
        XML_TOK::PARTIAL_CHAR => {
            if !(*parser).m_parsingStatus.finalBuffer {
                *endPtr = buf.as_ptr();
                return XML_Error::NONE;
            }
            return XML_Error::PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(externalEntityContentProcessor as Processor);
    (*parser).m_tagLevel = 1;
    externalEntityContentProcessor(parser, buf, endPtr)
}

unsafe extern "C" fn externalEntityContentProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).doContent(
        1,
        EncodingType::Normal,
        buf,
        endPtr,
        !(*parser).m_parsingStatus.finalBuffer,
    );
    if result == XML_Error::NONE {
        if !(*parser).storeRawNames() {
            return XML_Error::NO_MEMORY;
        }
    }
    result
}

impl<'scf> XML_ParserStruct<'scf> {
    unsafe fn doContent(
        &mut self,
        startTagLevel: c_int,
        enc_type: EncodingType,
        mut buf: ExpatBufRef,
        nextPtr: *mut *const c_char,
        haveMore: XML_Bool,
    ) -> XML_Error {
        /* save one level of indirection */
        let dtd: *mut DTD = self.m_dtd; /* XmlContentTok doesn't always set the last arg */
        let mut eventPP: *mut *const c_char = ptr::null_mut();
        let mut eventEndPP: *mut *const c_char = ptr::null_mut();
        if enc_type.is_internal() {
            eventPP = &mut (*self.m_openInternalEntities).internalEventPtr;
            eventEndPP = &mut (*self.m_openInternalEntities).internalEventEndPtr
        } else {
            eventPP = &mut self.m_eventPtr;
            eventEndPP = &mut self.m_eventEndPtr
        }
        let enc = self.encoding(enc_type);
        *eventPP = buf.as_ptr();
        loop {
            let mut next: *const c_char = buf.as_ptr();
            let mut tok = (*enc).xmlTok(XML_STATE::CONTENT, buf, &mut next);
            *eventEndPP = next;
            match tok {
                XML_TOK::TRAILING_CR => {
                    if haveMore {
                        *nextPtr = buf.as_ptr();
                        return XML_Error::NONE;
                    }
                    *eventEndPP = buf.as_ptr().offset(buf.len().try_into().unwrap());
                    if self.m_handlers.hasCharacterData() {
                        let mut c: XML_Char = 0xa;
                        self.m_handlers.characterData(&[c]);
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf);
                    }
                    /* LCOV_EXCL_STOP */
                    /* We are at the end of the final buffer, should we check for
                       XML_Parsing::SUSPENDED, XML_Parsing::FINISHED?
                    */
                    if startTagLevel == 0 {
                        return XML_Error::NO_ELEMENTS;
                    }
                    if self.m_tagLevel != startTagLevel {
                        return XML_Error::ASYNC_ENTITY;
                    }
                    *nextPtr = buf.end();
                    return XML_Error::NONE;
                }
                XML_TOK::NONE => {
                    if haveMore {
                        *nextPtr = buf.as_ptr();
                        return XML_Error::NONE;
                    }
                    if startTagLevel > 0 {
                        if self.m_tagLevel != startTagLevel {
                            return XML_Error::ASYNC_ENTITY;
                        }
                        *nextPtr = buf.as_ptr();
                        return XML_Error::NONE;
                    }
                    return XML_Error::NO_ELEMENTS;
                }
                XML_TOK::INVALID => {
                    *eventPP = next;
                    return XML_Error::INVALID_TOKEN;
                }
                XML_TOK::PARTIAL => {
                    if haveMore {
                        *nextPtr = buf.as_ptr();
                        return XML_Error::NONE;
                    }
                    return XML_Error::UNCLOSED_TOKEN;
                }
                XML_TOK::PARTIAL_CHAR => {
                    if haveMore {
                        *nextPtr = buf.as_ptr();
                        return XML_Error::NONE;
                    }
                    return XML_Error::PARTIAL_CHAR;
                }
                XML_TOK::ENTITY_REF => {
                    let mut ch: XML_Char = (*enc).predefinedEntityName(
                        buf
                            .inc_start((*enc).minBytesPerChar() as isize)
                            .with_end(next)
                            .dec_end((*enc).minBytesPerChar() as usize)
                    ) as XML_Char;
                    if ch != 0 {
                        let hasCharacterData = self.m_handlers.characterData(&[ch]);

                        if !hasCharacterData && self.m_handlers.hasDefault() {
                            reportDefault(self, enc_type, buf.with_end(next));
                        }
                    } else {
                        let successful = (*dtd).pool.store_c_string(
                            enc,
                            buf
                                .inc_start((*enc).minBytesPerChar() as isize)
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar() as usize)
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        };
                        let entity = (*dtd).pool.current_slice(|name| {
                            hash_lookup!((*dtd).generalEntities, name.as_ptr())
                        });

                        /* First, determine if a check for an existing declaration is needed;
                           if yes, check that the entity exists, and that it is internal,
                           otherwise call the skipped entity or default handler.
                        */
                        let mut skipHandlers = false;
                        if !(*dtd).hasParamEntityRefs || (*dtd).standalone {
                            (*dtd).pool.clear_current();

                            if entity.is_null() {
                                return XML_Error::UNDEFINED_ENTITY;
                            } else {
                                if !(*entity).is_internal {
                                    return XML_Error::ENTITY_DECLARED_IN_PE;
                                }
                            }
                        } else if entity.is_null() {
                            let skippedHandlerRan = (*dtd).pool.current_slice(|name| {
                                self.m_handlers.skippedEntity(name.as_ptr(), 0)
                            });

                            (*dtd).pool.clear_current();

                            if !skippedHandlerRan && self.m_handlers.hasDefault() {
                                if !cfg!(feature = "mozilla") {
                                    reportDefault(self, enc_type, buf.with_end(next));
                                }
                            }
                            if cfg!(feature = "mozilla") {
                                return XML_Error::UNDEFINED_ENTITY;
                            } else {
                                skipHandlers = true;
                            }
                        } else {
                            (*dtd).pool.clear_current();
                        }

                        if !skipHandlers {
                            if (*entity).open {
                                return XML_Error::RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity).notation.is_null() {
                                return XML_Error::BINARY_ENTITY_REF;
                            }
                            if !(*entity).textPtr.is_null() {
                                let mut result: XML_Error = XML_Error::NONE;
                                if !self.m_defaultExpandInternalEntities {
                                    let skippedHandlerRan = self.m_handlers.skippedEntity((*entity).name, 0);

                                    if !skippedHandlerRan && self.m_handlers.hasDefault() {
                                        reportDefault(self, enc_type, buf.with_end(next));
                                    }
                                } else {
                                    result = self.processInternalEntity(entity, false);
                                    if result != XML_Error::NONE {
                                        return result;
                                    }
                                }
                            } else if self.m_handlers.hasExternalEntityRef() {
                                (*entity).open = true;
                                // Avoiding borrowing issues by separating out
                                // the fields here:
                                let context = self.getContext();
                                (*entity).open = false;
                                if !context {
                                    return XML_Error::NO_MEMORY;
                                }
                                if self.m_tempPool.current_slice(|name| {
                                    self.m_handlers.externalEntityRef(
                                        name.as_ptr(),
                                        (*entity).base,
                                        (*entity).systemId,
                                        (*entity).publicId,
                                    )
                                }) == Ok(0)
                                {
                                    return XML_Error::EXTERNAL_ENTITY_HANDLING;
                                }
                                self.m_tempPool.clear_current();
                            } else if self.m_handlers.hasDefault() {
                                reportDefault(self, enc_type, buf.with_end(next));
                            }
                        }
                    }
                }
                XML_TOK::START_TAG_NO_ATTS
                | XML_TOK::START_TAG_WITH_ATTS => {
                    /* fall through */
                    let mut result_0: XML_Error = XML_Error::NONE;
                    let mut to_buf: ExpatBufRefMut<XML_Char>;
                    let mut tag = match self.m_freeTagList.take() {
                        Some(mut tag) => {
                            self.m_freeTagList = tag.parent.take();
                            tag
                        }
                        None => {
                            let mut tag = match Box::try_new(std::mem::zeroed::<Tag>()) {
                                Ok(tag) => tag,
                                Err(_) => return XML_Error::NO_MEMORY
                            };

                            tag.buf = MALLOC![c_char; INIT_TAG_BUF_SIZE];
                            if tag.buf.is_null() {
                                return XML_Error::NO_MEMORY;
                            }
                            tag.bufEnd = tag.buf.offset(INIT_TAG_BUF_SIZE as isize);

                            tag
                        }
                    };
                    tag.bindings = ptr::null_mut();
                    tag.name.localPart = ptr::null();
                    tag.name.prefix = ptr::null();
                    let mut fromBuf: ExpatBufRef = buf.inc_start((*enc).minBytesPerChar() as isize);
                    tag.rawName = fromBuf.as_ptr();
                    tag.rawNameLength = (*enc).nameLength(tag.rawName);
                    fromBuf = fromBuf.with_len(tag.rawNameLength as usize);
                    self.m_tagLevel += 1;
                    // let mut rawNameEnd: *const c_char =
                    //     tag.rawName.offset(tag.rawNameLength as isize);
                    to_buf = ExpatBufRefMut::new(
                        tag.buf as *mut ICHAR,
                        (tag.bufEnd as *mut ICHAR).offset(-1),
                    );
                    loop {
                        let mut bufSize: c_int = 0;
                        let mut convLen: c_int = 0;
                        let convert_res: super::xmltok::XML_Convert_Result = XmlConvert!(
                            enc,
                            &mut fromBuf,
                            &mut to_buf,
                        );
                        convLen = to_buf.as_ptr().wrapping_offset_from(tag.buf as *mut XML_Char).try_into().unwrap();
                        if fromBuf.is_empty() || convert_res == super::xmltok::XML_Convert_Result::INPUT_INCOMPLETE
                        {
                            tag.name.strLen = convLen;
                            break;
                        } else {
                            bufSize = (tag.bufEnd.wrapping_offset_from(tag.buf) as c_int) << 1;
                            let mut temp = REALLOC!(tag.buf => [c_char; bufSize]);
                            if temp.is_null() {
                                return XML_Error::NO_MEMORY;
                            }
                            tag.buf = temp;
                            tag.bufEnd = temp.offset(bufSize as isize);
                            to_buf = ExpatBufRefMut::new(
                                (temp as *mut XML_Char).offset(convLen as isize),
                                (tag.bufEnd as *mut XML_Char).offset(-1),
                            );
                        }
                    }
                    tag.name.str_0 = tag.buf as *const XML_Char;
                    *to_buf.as_mut_ptr() = '\u{0}' as XML_Char;
                    result_0 = self.storeAtts(enc_type, buf, &mut tag.name, &mut tag.bindings);
                    if result_0 as u64 != 0 {
                        return result_0;
                    }

                    let handlers = &self.m_handlers;
                    let started = handlers.startElement(tag.name.str_0, &mut self.m_atts);

                    if !started && handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.with_end(next));
                    }
                    self.m_tempPool.clear();

                    // FIXME: do we need to update m_tagStack when returning an error?
                    tag.parent = self.m_tagStack.take();
                    self.m_tagStack = Some(tag);
                }
                XML_TOK::EMPTY_ELEMENT_NO_ATTS
                | XML_TOK::EMPTY_ELEMENT_WITH_ATTS => {
                    /* fall through */
                    let mut rawName: ExpatBufRef = buf.inc_start((*enc).minBytesPerChar() as isize);
                    let mut result_1: XML_Error = XML_Error::NONE;
                    let mut bindings: *mut Binding = ptr::null_mut();
                    let mut noElmHandlers = true;
                    let mut name_0 = TagName {
                        str_0: ptr::null(),
                        localPart: ptr::null(),
                        prefix: ptr::null(),
                        strLen: 0,
                        uriLen: 0,
                        prefixLen: 0,
                    };
                    let successful = self.m_tempPool.store_c_string(
                        enc,
                        rawName.with_len((*enc).nameLength(rawName.as_ptr()) as usize),
                    );
                    if !successful {
                        return XML_Error::NO_MEMORY;
                    }
                    name_0.str_0 = self.m_tempPool.finish_string().as_ptr();
                    result_1 = self.storeAtts(enc_type, buf, &mut name_0, &mut bindings);
                    if result_1 != XML_Error::NONE {
                        self.freeBindings(bindings);
                        return result_1;
                    }
                    self.m_tempPool.finish_string();
                    let handlers = &self.m_handlers;
                    let started = handlers.startElement(name_0.str_0, &mut self.m_atts);
                    if started {
                        noElmHandlers = false
                    }
                    if self.m_handlers.hasEndElement() {
                        if started {
                            *eventPP = *eventEndPP
                        }

                        self.m_handlers.endElement(name_0.str_0);

                        noElmHandlers = false
                    }
                    if noElmHandlers && self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.with_end(next));
                    }
                    self.m_tempPool.clear();
                    self.freeBindings(bindings);
                    if self.m_tagLevel == 0 && self.m_parsingStatus.parsing != XML_Parsing::FINISHED {
                        if self.m_parsingStatus.parsing == XML_Parsing::SUSPENDED {
                            self.m_processor = Some(epilogProcessor as Processor)
                        } else {
                            return epilogProcessor(self, buf.with_start(next), nextPtr);
                        }
                    }
                }
                XML_TOK::END_TAG => {
                    if self.m_tagLevel == startTagLevel {
                        return XML_Error::ASYNC_ENTITY;
                    } else {
                        // Pop the tag off the tagStack
                        let mut tag = self.m_tagStack.take().unwrap();
                        self.m_tagStack = tag.parent.take();

                        let rawName_0 = buf.inc_start(((*enc).minBytesPerChar() * 2) as isize);
                        let len = (*enc).nameLength(rawName_0.as_ptr());
                        if len != tag.rawNameLength
                            || memcmp(
                                tag.rawName as *const c_void,
                                rawName_0.as_ptr() as *const c_void,
                                len as usize,
                            ) != 0
                        {
                            #[cfg(feature = "mozilla")]
                            {
                                /* This code is copied from the |if (endElementHandler)| block below */
                                let mut localPart: *const XML_Char = ptr::null();
                                let mut prefix: *const XML_Char = ptr::null();
                                let mut uri: *mut XML_Char = ptr::null_mut();
                                localPart = tag.name.localPart;
                                if self.m_ns && !localPart.is_null() {
                                    /* localPart and prefix may have been overwritten in
                                       tag->name.str, since this points to the binding->uri
                                       buffer which gets re-used; so we have to add them again
                                    */
                                    uri = (tag.name.str_0 as *mut XML_Char)
                                        .offset(tag.name.uriLen as isize);
                                    /* don't need to check for space - already done in storeAtts() */
                                    while *localPart != 0 {
                                        let fresh2 = localPart;
                                        localPart = localPart.offset(1);
                                        let fresh3 = uri;
                                        uri = uri.offset(1);
                                        *fresh3 = *fresh2
                                    }
                                    prefix = tag.name.prefix as *mut XML_Char;
                                    if self.m_ns_triplets && !prefix.is_null() {
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
                                self.m_mismatch = tag.name.str_0;
                            }
                            *eventPP = rawName_0.as_ptr();
                            return XML_Error::TAG_MISMATCH;
                        }
                        self.m_tagLevel -= 1;
                        if self.m_handlers.hasEndElement() {
                            let mut localPart: *const XML_Char = ptr::null();
                            let mut prefix: *const XML_Char = ptr::null();
                            let mut uri: *mut XML_Char = ptr::null_mut();
                            localPart = tag.name.localPart;
                            if self.m_ns && !localPart.is_null() {
                                /* localPart and prefix may have been overwritten in
                                   tag->name.str, since this points to the binding->uri
                                   buffer which gets re-used; so we have to add them again
                                */
                                uri = (tag.name.str_0 as *mut XML_Char)
                                    .offset(tag.name.uriLen as isize);
                                /* don't need to check for space - already done in storeAtts() */
                                while *localPart != 0 {
                                    let fresh2 = localPart;
                                    localPart = localPart.offset(1);
                                    let fresh3 = uri;
                                    uri = uri.offset(1);
                                    *fresh3 = *fresh2
                                }
                                prefix = tag.name.prefix as *mut XML_Char;
                                if self.m_ns_triplets && !prefix.is_null() {
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

                            self.m_handlers.endElement(tag.name.str_0);
                        } else if self.m_handlers.hasDefault() {
                            reportDefault(self, enc_type, buf.with_end(next));
                        }
                        while !tag.bindings.is_null() {
                            let mut b: *mut Binding = tag.bindings;
                            self.m_handlers.endNamespaceDecl((*(*b).prefix).name);
                            tag.bindings = (*tag.bindings).nextTagBinding;
                            (*b).nextTagBinding = self.m_freeBindingList;
                            self.m_freeBindingList = b;
                            (*(*b).prefix).binding = (*b).prevPrefixBinding
                        }

                        // Move the tag to the free list
                        tag.parent = self.m_freeTagList.take();
                        self.m_freeTagList = Some(tag);

                        if self.m_tagLevel == 0
                            && self.m_parsingStatus.parsing != XML_Parsing::FINISHED
                        {
                            if self.m_parsingStatus.parsing == XML_Parsing::SUSPENDED {
                                self.m_processor = Some(epilogProcessor as Processor)
                            } else {
                                return epilogProcessor(self, buf.with_start(next), nextPtr);
                            }
                        }
                    }
                }
                XML_TOK::CHAR_REF => {
                    let mut n: c_int = (*enc).charRefNumber(buf);
                    if n < 0 {
                        return XML_Error::BAD_CHAR_REF;
                    }
                    if self.m_handlers.hasCharacterData() {
                        let mut out_buf: [XML_Char; XML_ENCODE_MAX] = [0; XML_ENCODE_MAX];
                        let n = XmlEncode(n, &mut out_buf) as usize;
                        self.m_handlers.characterData(&out_buf[..n]);
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.with_end(next));
                    }
                }
                XML_TOK::XML_DECL => return XML_Error::MISPLACED_XML_PI,
                XML_TOK::DATA_NEWLINE => {
                    if self.m_handlers.hasCharacterData() {
                        let mut c_0: XML_Char = 0xa;
                        self.m_handlers.characterData(&[c_0]);
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.with_end(next));
                    }
                }
                XML_TOK::CDATA_SECT_OPEN => {
                    let mut result_2: XML_Error = XML_Error::NONE;

                    let startHandlerRan = self.m_handlers.startCDataSection();

                    if startHandlerRan {
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
                    } else if 0 != 0 && self.m_handlers.hasCharacterData() {
                        self.m_handlers.characterData(&[]);
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.with_end(next));
                    }
                    let mut new_buf = Some(buf.with_start(next));
                    result_2 = doCdataSection(self, enc_type, &mut new_buf, nextPtr, haveMore);
                    next = new_buf.map_or(ptr::null(), |x| x.as_ptr());
                    if result_2 != XML_Error::NONE {
                        return result_2;
                    } else if next.is_null() {
                        self.m_processor = Some(cdataSectionProcessor as Processor);
                        return result_2;
                    }
                }
                XML_TOK::TRAILING_RSQB => {
                    if haveMore {
                        *nextPtr = buf.as_ptr();
                        return XML_Error::NONE;
                    }
                    if self.m_handlers.hasCharacterData() {
                        if MUST_CONVERT!(enc, buf.as_ptr()) {
                            let mut dataPtr = ExpatBufRefMut::new(
                                self.m_dataBuf as *mut ICHAR,
                                self.m_dataBufEnd as *mut ICHAR,
                            );
                            XmlConvert!(enc, &mut buf, &mut dataPtr);
                            self.m_handlers.characterData(
                                &ExpatBufRef::new(
                                    self.m_dataBuf,
                                    dataPtr.as_ptr(),
                                ),
                            );
                        } else {
                            self.m_handlers.characterData(&ExpatBufRef::<XML_Char>::from(buf));
                        }
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf);
                    }
                    /* We are at the end of the final buffer, should we check for
                       XML_Parsing::SUSPENDED, XML_Parsing::FINISHED?
                    */
                    if startTagLevel == 0 {
                        *eventPP = buf.end();
                        return XML_Error::NO_ELEMENTS;
                    }
                    if self.m_tagLevel != startTagLevel {
                        *eventPP = buf.end();
                        return XML_Error::ASYNC_ENTITY;
                    }
                    *nextPtr = buf.end();
                    return XML_Error::NONE;
                }
                XML_TOK::DATA_CHARS => {
                    let mut handlers = &self.m_handlers;
                    if handlers.hasCharacterData() {
                        if MUST_CONVERT!(enc, buf.as_ptr()) {
                            loop {
                                let mut from_buf = buf.with_end(next);
                                let mut to_buf = ExpatBufRefMut::new(
                                    self.m_dataBuf as *mut ICHAR,
                                    self.m_dataBufEnd as *mut ICHAR,
                                );
                                let convert_res_0: super::xmltok::XML_Convert_Result = XmlConvert!(
                                    enc,
                                    &mut from_buf,
                                    &mut to_buf,
                                );
                                buf = buf.with_start(from_buf.as_ptr());
                                *eventEndPP = buf.as_ptr();
                                let data_buf = ExpatBufRef::new(self.m_dataBuf, to_buf.as_ptr());
                                handlers.characterData(&data_buf);
                                if convert_res_0 == super::xmltok::XML_Convert_Result::COMPLETED
                                    || convert_res_0 == super::xmltok::XML_Convert_Result::INPUT_INCOMPLETE
                                {
                                    break;
                                }
                                *eventPP = buf.as_ptr()
                            }
                        } else {
                            let data_buf: ExpatBufRef<XML_Char> = buf.with_end(next).into();
                            handlers.characterData(&data_buf);
                        }
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.with_end(next));
                    }
                }
                XML_TOK::PI => {
                    if reportProcessingInstruction(self, enc_type, buf.with_end(next)) == 0 {
                        return XML_Error::NO_MEMORY;
                    }
                }
                XML_TOK::COMMENT => {
                    if reportComment(self, enc_type, buf.with_end(next)) == 0 {
                        return XML_Error::NO_MEMORY;
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
                        reportDefault(self, enc_type, buf.with_end(next));
                    }
                }
            }
            buf = buf.with_start(next);
            *eventPP = buf.as_ptr();
            match self.m_parsingStatus.parsing {
                XML_Parsing::SUSPENDED => {
                    *nextPtr = next;
                    return XML_Error::NONE;
                }
                XML_Parsing::FINISHED => return XML_Error::ABORTED,
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

    unsafe fn freeBindings(&mut self, mut bindings: *mut Binding) {
        while !bindings.is_null() {
            let mut b: *mut Binding = bindings;
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
        attStr: ExpatBufRef,
        tagNamePtr: *mut TagName,
        bindingsPtr: *mut *mut Binding,
    ) -> XML_Error {
        let dtd: *mut DTD = self.m_dtd; /* save one level of indirection */
        let mut prefixLen: c_int = 0;
        let mut uri: *mut XML_Char = ptr::null_mut();
        let mut nPrefixes: c_int = 0;
        let mut nXMLNSDeclarations: c_int = 0;
        let mut binding: *mut Binding = ptr::null_mut();
        let mut localPart: *const XML_Char = ptr::null();
        let enc = self.encoding(enc_type);
        /* lookup the element type name */
        let elementType = if let Some(elementType) = (*dtd).elementTypes.get_mut(&HashKey::from((*tagNamePtr).str_0)) {
            elementType.as_mut()
        } else {
            let mut name = match (*dtd).pool.copy_c_string((*tagNamePtr).str_0) {
                Some(name) => name,
                None => return XML_Error::NO_MEMORY,
            };
            let elementType = hash_insert!(
                &mut (*dtd).elementTypes,
                name.as_ptr(),
                ElementType,
                ElementType::new
            );
            if elementType.is_null() {
                return XML_Error::NO_MEMORY;
            }
            if self.m_ns && self.setElementTypePrefix(elementType) == 0 {
                return XML_Error::NO_MEMORY;
            }
            elementType
        };
        self.m_atts.clear();
        self.typed_atts.clear();
        /* get the attributes from the tokenizer */
        let res = (*enc).getAtts(attStr, &mut |currAtt: super::xmltok::Attribute| {
            if self.m_atts.try_reserve(1).is_err() {
                return XML_Error::NO_MEMORY;
            }
            if self.typed_atts.try_reserve(1).is_err() {
                return XML_Error::NO_MEMORY;
            }

            /* add the name and value to the attribute list */
            let mut attId: *mut AttributeId = self.getAttributeId(
                enc_type,
                ExpatBufRef::new(
                    currAtt.name,
                    currAtt
                        .name
                        .offset((*enc).nameLength(currAtt.name) as isize),
                ),
            );
            if attId.is_null() {
                return XML_Error::NO_MEMORY;
            }
            /* Detect duplicate attributes by their QNames. This does not work when
            namespace processing is turned on and different prefixes for the same
            namespace are used. For this case we have a check further down.
            */
            if (*attId).name.get_type().is_set() {
                if !enc_type.is_internal() {
                    self.m_eventPtr = currAtt.name
                }
                return XML_Error::DUPLICATE_ATTRIBUTE;
            }
            (*attId).name.set_type(AttributeType::Normal);
            if !currAtt.normalized {
                let mut result: XML_Error = XML_Error::NONE;
                let mut isCdata = true;
                /* figure out whether declared as other than CDATA */
                if (*attId).maybeTokenized {
                    isCdata = (*elementType)
                        .defaultAtts
                        .iter()
                        .find(|da| attId == da.id)
                        .map(|da| da.isCdata)
                        .unwrap_or(isCdata);
                }
                /* normalize the attribute value */
                result = storeAttributeValue(
                    self,
                    enc_type,
                    isCdata,
                    ExpatBufRef::new(currAtt.valuePtr, currAtt.valueEnd),
                    &self.m_tempPool,
                );
                if result as u64 != 0 {
                    return result;
                }
                self.m_atts.push(Attribute::new((*attId).name.name(), self.m_tempPool.finish_string().as_ptr()));
            } else {
                /* the value did not need normalizing */
                if !self.m_tempPool.store_c_string(enc, ExpatBufRef::new(currAtt.valuePtr, currAtt.valueEnd)) {
                    return XML_Error::NO_MEMORY;
                }
                self.m_atts.push(Attribute::new((*attId).name.name(), self.m_tempPool.finish_string().as_ptr()));
            }
            /* handle prefixed attribute names */
            if !(*attId).prefix.is_null() {
                if (*attId).xmlns {
                    /* deal with namespace declarations here */
                    let mut result_0: XML_Error = addBinding(
                        self,
                        (*attId).prefix,
                        attId,
                        self.m_atts.last().unwrap().value,
                        bindingsPtr,
                    );
                    if result_0 as u64 != 0 {
                        return result_0;
                    }
                    if cfg!(feature = "mozilla") {
                        nXMLNSDeclarations += 1;
                        (*attId).name.set_type(AttributeType::Namespace);
                        self.typed_atts.push((*attId).name);
                    } else {
                        // Mozilla code replaces `--attIndex` with `attIndex++`,
                        // the former being equivalent to popping the last
                        // attribute from m_atts
                        self.m_atts.pop();
                    }
                } else {
                    /* deal with other prefixed names later */
                    nPrefixes += 1;
                    (*attId).name.set_type(AttributeType::Prefixed);
                    self.typed_atts.push((*attId).name);
                }
            } else {
                self.typed_atts.push((*attId).name);
            }

            XML_Error::NONE
        });
        if res != XML_Error::NONE {
            return res;
        }
        assert!(self.m_atts.len() == self.typed_atts.len());

        /* set-up for XML_GetSpecifiedAttributeCount and XML_GetIdAttributeIndex */
        self.m_nSpecifiedAtts = 2 * self.m_atts.len() as c_int;
        if !(*elementType).idAtt.is_null() && (*(*elementType).idAtt).name.get_type().is_set() {
            for i in 0..self.typed_atts.len() {
                if self.typed_atts[i] == (*(*elementType).idAtt).name {
                    self.m_idAttIndex = 2 * i as c_int;
                    break;
                }
            }
        } else {
            self.m_idAttIndex = -1
        }

        /* do attribute defaulting */
        if self.m_atts.try_reserve((*elementType).defaultAtts.len()).is_err() {
            return XML_Error::NO_MEMORY;
        }
        if self.typed_atts.try_reserve((*elementType).defaultAtts.len()).is_err() {
            return XML_Error::NO_MEMORY;
        }
        for da in &(*elementType).defaultAtts {
            if !(*da.id).name.get_type().is_set() && !da.value.is_null() {
                if !(*da.id).prefix.is_null() {
                    if (*da.id).xmlns {
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
                            (*da.id).name.set_type(AttributeType::Namespace);
                            nXMLNSDeclarations += 1;
                            self.m_atts.push(Attribute::from_default(da));
                            self.typed_atts.push((*(*da).id).name);
                        }
                    } else {
                        (*da.id).name.set_type(AttributeType::Prefixed);
                        nPrefixes += 1;
                        self.m_atts.push(Attribute::from_default(da));
                        self.typed_atts.push((*(*da).id).name);
                    }
                } else {
                    (*da.id).name.set_type(AttributeType::Normal);
                    self.m_atts.push(Attribute::from_default(da));
                    self.typed_atts.push((*(*da).id).name);
                }
            }
        }
        assert!(self.m_atts.len() == self.typed_atts.len());

        /* expand prefixed attribute names, check for duplicates,
        and clear flags that say whether attributes were specified */
        let mut i = 0; /* hash table index */
        if nPrefixes != 0 || nXMLNSDeclarations != 0 { // MOZILLA CHANGE
            let mut j_0: c_int = 0;
            if nPrefixes != 0 { // MOZILLA CHANGE
                self.m_nsAtts.clear();
                /* size of hash table must be at least 2 * (# of prefixed attributes) */
                if self.m_nsAtts.try_reserve((nPrefixes as usize) << 1).is_err() {
                    return XML_Error::NO_MEMORY;
                }
            } // MOZILLA CHANGE
            /* expand prefixed names and check for duplicates */
            while i < self.m_atts.len() {
                let mut s: *const XML_Char = self.m_atts[i].name;
                if self.typed_atts[i].get_type() == AttributeType::Prefixed { // TODO: this could be a match instead
                    let mut b: *const Binding = ptr::null();
                    /* clear flag */
                    /* not prefixed */
                    /* prefixed */
                    self.typed_atts[i].set_type(AttributeType::Unset); /* clear flag */
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
                        return XML_Error::NO_MEMORY;
                        /* LCOV_EXCL_LINE */
                    }
                    let id = id.unwrap();
                    b = (*(*id).prefix).binding;
                    if b.is_null() {
                        return XML_Error::UNBOUND_PREFIX;
                    }
                    j_0 = 0;
                    while j_0 < (*b).uriLen {
                        let c: XML_Char = *(*b).uri.offset(j_0 as isize);
                        if !self.m_tempPool.append_char(c) {
                            return XML_Error::NO_MEMORY;
                        }
                        j_0 += 1
                    }
                    loop {
                        let fresh22 = s;
                        s = s.offset(1);
                        if !(*fresh22 != ASCII_COLON as XML_Char) {
                            break;
                        }
                    }
                    loop {
                        /* copies null terminator */
                        if !self.m_tempPool.append_char(*s) {
                            return XML_Error::NO_MEMORY;
                        }
                        let fresh24 = s;
                        s = s.offset(1);
                        if !(*fresh24 != 0) {
                            break;
                        }
                    }
                    /* Check hash table for duplicate of expanded name (uriName).
                    Derived from code in lookup(parser, HASH_TABLE *table, ...).
                     */
                    let ret = self.m_tempPool.current_slice(|name| {
                        let hk = HashKey::from(name.as_ptr() as KEY);
                        if self.m_nsAtts.contains(&hk) {
                            return XML_Error::DUPLICATE_ATTRIBUTE;
                        }
                        XML_Error::NONE
                    });
                    if ret != XML_Error::NONE {
                        return ret;
                    }
                    if self.m_ns_triplets {
                        /* append namespace separator and prefix */
                        self.m_tempPool.replace_last_char(self.m_namespaceSeparator);
                        s = (*(*b).prefix).name;
                        loop {
                            if !self.m_tempPool.append_char(*s) {
                                return XML_Error::NO_MEMORY;
                            }
                            let fresh26 = s;
                            s = s.offset(1);
                            if !(*fresh26 != 0) {
                                break;
                            }
                        }
                    }
                    if ret != XML_Error::NONE { return ret; }
                    /* store expanded name in attribute list */
                    self.m_atts[i].name = self.m_tempPool.finish_string().as_ptr();
                    let hk = HashKey::from(self.m_atts[i].name as KEY);
                    self.m_nsAtts.insert(hk);
                    nPrefixes -= 1;
                    if nPrefixes == 0 && nXMLNSDeclarations == 0 {
                        i += 1;
                        break;
                    }
                } else if cfg!(feature = "mozilla") && self.typed_atts[i].get_type() == AttributeType::Namespace {
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

                    self.typed_atts[i].set_type(AttributeType::Unset); /* clear flag */
                    if !self.m_tempPool.append_c_string(xmlnsNamespace.as_ptr()) ||
                        !self.m_tempPool.append_char(self.m_namespaceSeparator)
                    {
                        return XML_Error::NO_MEMORY;
                    }

                    s = s.offset(xmlnsPrefix.len() as isize - 1);
                    if *s == ':' as XML_Char {
                        s = s.offset(1);
                        loop { /* copies null terminator */
                            if !self.m_tempPool.append_char(*s) {
                                return XML_Error::NO_MEMORY;
                            }
                            if *s == '\u{0}' as XML_Char {
                                s = s.offset(1);
                                break;
                            }
                            s = s.offset(1);
                        }

                        if self.m_ns_triplets { /* append namespace separator and prefix */
                            self.m_tempPool.replace_last_char(self.m_namespaceSeparator);
                            if !self.m_tempPool.append_c_string(xmlnsPrefix.as_ptr()) ||
                                !self.m_tempPool.append_char('\u{0}' as XML_Char)
                            {
                                return XML_Error::NO_MEMORY;
                            }
                        }
                    } else {
                        /* xlmns attribute without a prefix. */
                        if !self.m_tempPool.append_c_string(xmlnsPrefix.as_ptr()) ||
                            !self.m_tempPool.append_char('\u{0}' as XML_Char)
                        {
                            return XML_Error::NO_MEMORY;
                        }
                    }

                    /* store expanded name in attribute list */
                    s = self.m_tempPool.finish_string().as_ptr();
                    self.m_atts[i].name = s;

                    nXMLNSDeclarations -= 1;
                    if nXMLNSDeclarations == 0 && nPrefixes == 0 {
                        i += 1;
                        break;
                    }
                } else {
                    self.typed_atts[i].set_type(AttributeType::Unset);
                }
                i += 1
            }
        }
        /* clear flags for the remaining attributes */
        while i < self.m_atts.len() {
            self.typed_atts[i].set_type(AttributeType::Unset);
            i += 1
        }

        // REXPAT: append a NULL pointer as the stop marker
        if self.m_atts.try_reserve(1).is_err() {
            return XML_Error::NO_MEMORY;
        }
        self.m_atts.push(Attribute::new_null());

        binding = *bindingsPtr;
        while !binding.is_null() {
            (*(*binding).attId).name.set_type(AttributeType::Unset);
            binding = (*binding).nextTagBinding
        }
        if !self.m_ns {
            return XML_Error::NONE;
        }
        /* expand the element type name */
        if !(*elementType).prefix.is_null() {
            binding = (*(*elementType).prefix).binding;
            if binding.is_null() {
                return XML_Error::UNBOUND_PREFIX;
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
            return XML_Error::NONE;
        }
        prefixLen = 0;
        if self.m_ns_triplets && !(*(*binding).prefix).name.is_null() {
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
        let mut i = 0;
        loop {
            let fresh31 = i;
            i = i + 1;
            if !(*localPart.offset(fresh31 as isize) != 0) {
                break;
            }
        }
        let n = i + (*binding).uriLen + prefixLen;
        if n > (*binding).uriAlloc {
            uri = MALLOC![XML_Char; n as usize + EXPAND_SPARE];
            if uri.is_null() {
                return XML_Error::NO_MEMORY;
            }
            (*binding).uriAlloc = n + EXPAND_SPARE as c_int;
            memcpy(
                uri as *mut c_void,
                (*binding).uri as *const c_void,
                ((*binding).uriLen as usize).wrapping_mul(::std::mem::size_of::<XML_Char>()),
            );
            let mut tStk = &mut self.m_tagStack;
            while let Some(tag) = tStk {
                if tag.name.str_0 == (*binding).uri as *const XML_Char {
                    tag.name.str_0 = uri
                }
                tStk = &mut tag.parent;
            }
            FREE!((*binding).uri);
            (*binding).uri = uri
        }
        /* if m_namespaceSeparator != '\0' then uri includes it already */
        uri = (*binding).uri.offset((*binding).uriLen as isize);
        memcpy(
            uri as *mut c_void,
            localPart as *const c_void,
            (i as usize).wrapping_mul(::std::mem::size_of::<XML_Char>()),
        );
        /* we always have a namespace separator between localPart and prefix */
        if prefixLen != 0 {
            uri = uri.offset((i - 1) as isize); /* replace null terminator */
            *uri = self.m_namespaceSeparator;
            memcpy(
                uri.offset(1) as *mut c_void,
                (*(*binding).prefix).name as *const c_void,
                (prefixLen as usize).wrapping_mul(::std::mem::size_of::<XML_Char>()),
            );
        }
        (*tagNamePtr).str_0 = (*binding).uri;
        XML_Error::NONE
    }
}

/* addBinding() overwrites the value of prefix->binding without checking.
   Therefore one must keep track of the old value outside of addBinding().
*/

unsafe extern "C" fn addBinding(
    mut parser: XML_Parser,
    mut prefix: *mut Prefix,
    mut attId: *mut AttributeId,
    mut uri: *const XML_Char,
    mut bindingsPtr: *mut *mut Binding,
) -> XML_Error {
    const xmlNamespace: [XML_Char; 37] = XML_STR![
        ASCII_h, ASCII_t, ASCII_t, ASCII_p, ASCII_COLON, ASCII_SLASH, ASCII_SLASH, ASCII_w,
        ASCII_w, ASCII_w, ASCII_PERIOD, ASCII_w, ASCII_3, ASCII_PERIOD, ASCII_o, ASCII_r, ASCII_g,
        ASCII_SLASH, ASCII_X, ASCII_M, ASCII_L, ASCII_SLASH, ASCII_1, ASCII_9, ASCII_9, ASCII_8,
        ASCII_SLASH, ASCII_n, ASCII_a, ASCII_m, ASCII_e, ASCII_s, ASCII_p, ASCII_a, ASCII_c,
        ASCII_e,
    ];
    const xmlnsNamespace: [XML_Char; 30] = XML_STR![
        ASCII_h, ASCII_t, ASCII_t, ASCII_p, ASCII_COLON, ASCII_SLASH, ASCII_SLASH, ASCII_w,
        ASCII_w, ASCII_w, ASCII_PERIOD, ASCII_w, ASCII_3, ASCII_PERIOD, ASCII_o, ASCII_r, ASCII_g,
        ASCII_SLASH, ASCII_2, ASCII_0, ASCII_0, ASCII_0, ASCII_SLASH, ASCII_x, ASCII_m, ASCII_l,
        ASCII_n, ASCII_s, ASCII_SLASH,
    ];
    let mut mustBeXML = false;
    let mut isXML = true;
    let mut isXMLNS = true;
    let mut b: *mut Binding = ptr::null_mut();
    /* empty URI is only valid for default namespace per XML NS 1.0 (not 1.1) */
    if *uri as c_int == '\u{0}' as i32 && !(*prefix).name.is_null() {
        return XML_Error::UNDECLARING_PREFIX;
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
            return XML_Error::RESERVED_PREFIX_XMLNS;
        }
        if *(*prefix).name.offset(3) == '\u{0}' as XML_Char {
            mustBeXML = true
        }
    }
    let mut len = 0;
    while *uri.add(len) != 0 {
        if isXML && (len >= xmlNamespace.len()
                || *uri.add(len) as c_int != xmlNamespace[len] as c_int)
        {
            isXML = false
        }
        if !mustBeXML
            && isXMLNS
            && (len >= xmlnsNamespace.len()
                || *uri.add(len) as c_int != xmlnsNamespace[len] as c_int)
        {
            isXMLNS = false
        }
        len += 1;
    }
    isXML = isXML && len == (xmlNamespace.len() - 1);
    isXMLNS = isXMLNS && len == (xmlnsNamespace.len() - 1);
    if mustBeXML != isXML {
        return if mustBeXML {
            XML_Error::RESERVED_PREFIX_XML
        } else {
            XML_Error::RESERVED_NAMESPACE_URI
        } as XML_Error;
    }
    if isXMLNS {
        return XML_Error::RESERVED_NAMESPACE_URI;
    }
    if (*parser).m_namespaceSeparator != 0 {
        len += 1;
    }
    if !(*parser).m_freeBindingList.is_null() {
        b = (*parser).m_freeBindingList;
        if len > (*b).uriAlloc as usize {
            let mut temp: *mut XML_Char = REALLOC!((*b).uri => [XML_Char; len + EXPAND_SPARE]);
            if temp.is_null() {
                return XML_Error::NO_MEMORY;
            }
            (*b).uri = temp;
            (*b).uriAlloc = (len + EXPAND_SPARE).try_into().unwrap();
        }
        (*parser).m_freeBindingList = (*b).nextTagBinding
    } else {
        b = MALLOC!(@Binding);
        if b.is_null() {
            return XML_Error::NO_MEMORY;
        }
        (*b).uri = MALLOC![XML_Char; len + EXPAND_SPARE];
        if (*b).uri.is_null() {
            FREE!(b);
            return XML_Error::NO_MEMORY;
        }
        (*b).uriAlloc = (len + EXPAND_SPARE).try_into().unwrap();
    }
    (*b).uriLen = len.try_into().unwrap();
    memcpy(
        (*b).uri as *mut c_void,
        uri as *const c_void,
        (len as usize).wrapping_mul(::std::mem::size_of::<XML_Char>()),
    );
    if (*parser).m_namespaceSeparator != 0 {
        *(*b).uri.offset((len - 1) as isize) = (*parser).m_namespaceSeparator
    }
    (*b).prefix = prefix;
    (*b).attId = attId;
    (*b).prevPrefixBinding = (*prefix).binding;
    /* NULL binding when default namespace undeclared */
    if *uri == '\u{0}' as XML_Char && prefix == &mut (*(*parser).m_dtd).defaultPrefix as *mut Prefix
    {
        (*prefix).binding = ptr::null_mut()
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
                ptr::null()
            },
        );
    }
    XML_Error::NONE
}
/* The idea here is to avoid using stack for each CDATA section when
   the whole file is parsed with one call.
*/

unsafe extern "C" fn cdataSectionProcessor(
    mut parser: XML_Parser,
    buf: ExpatBufRef,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut opt_buf = Some(buf);
    let mut result: XML_Error = doCdataSection(
        parser,
        EncodingType::Normal,
        &mut opt_buf,
        endPtr,
        !(*parser).m_parsingStatus.finalBuffer,
    );
    if result != XML_Error::NONE {
        return result;
    }
    if let Some(buf) = opt_buf {
        if !(*parser).m_parentParser.is_null() {
            /* we are parsing an external entity */
            (*parser).m_processor = Some(externalEntityContentProcessor as Processor);
            return externalEntityContentProcessor(parser, buf, endPtr);
        } else {
            (*parser).m_processor = Some(contentProcessor as Processor);
            return contentProcessor(parser, buf, endPtr);
        }
    }
    result
}
/* startPtr gets set to non-null if the section is closed, and to null if
   the section is not yet closed.
*/

unsafe extern "C" fn doCdataSection(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    start_buf: &mut Option<ExpatBufRef>,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
) -> XML_Error {
    let mut buf = start_buf.unwrap().clone();
    let mut eventPP: *mut *const c_char = ptr::null_mut();
    let mut eventEndPP: *mut *const c_char = ptr::null_mut();
    if !enc_type.is_internal() {
        eventPP = &mut (*parser).m_eventPtr;
        eventEndPP = &mut (*parser).m_eventEndPtr
    } else {
        eventPP = &mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &mut (*(*parser).m_openInternalEntities).internalEventEndPtr
    }
    *eventPP = buf.as_ptr();
    *start_buf = None;
    let enc = (*parser).encoding(enc_type);
    loop {
        let mut next: *const c_char = ptr::null();
        let mut tok = (*enc).xmlTok(XML_STATE::CDATA_SECTION, buf, &mut next);
        *eventEndPP = next;
        match tok {
            XML_TOK::CDATA_SECT_CLOSE => {
                let endHandlerRan = (*parser).m_handlers.endCDataSection();

                if endHandlerRan {
                } else if 0 != 0 && (*parser).m_handlers.hasCharacterData() {
                    (*parser).m_handlers.characterData(&[]);
                } else if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, enc_type, buf.with_end(next));
                }
                *start_buf = Some(buf.with_start(next));
                *nextPtr = next;
                if (*parser).m_parsingStatus.parsing == XML_Parsing::FINISHED {
                    return XML_Error::ABORTED;
                } else {
                    return XML_Error::NONE;
                }
                /* BEGIN disabled code */
                /* see comment under XML_TOK::CDATA_SECT_OPEN */
                /* END disabled code */
                /* LCOV_EXCL_STOP */
            }
            XML_TOK::DATA_NEWLINE => {
                if (*parser).m_handlers.hasCharacterData() {
                    let mut c: XML_Char = 0xa;
                    (*parser).m_handlers.characterData(&[c]);
                } else if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, enc_type, buf.with_end(next));
                }
            }
            XML_TOK::DATA_CHARS => {
                let mut handlers = &(*parser).m_handlers;
                if handlers.hasCharacterData() {
                    if MUST_CONVERT!(enc, buf.as_ptr()) {
                        loop {
                            let mut from_buf = buf.with_end(next);
                            let mut to_buf = ExpatBufRefMut::new(
                                (*parser).m_dataBuf as *mut ICHAR,
                                (*parser).m_dataBufEnd as *mut ICHAR,
                            );
                            let convert_res: super::xmltok::XML_Convert_Result = XmlConvert!(
                                enc,
                                &mut from_buf,
                                &mut to_buf,
                            );
                            buf = buf.with_start(from_buf.as_ptr());
                            *eventEndPP = next;
                            handlers.characterData(
                                &ExpatBufRef::new(
                                    (*parser).m_dataBuf,
                                    to_buf.as_ptr(),
                                ),
                            );
                            if convert_res == super::xmltok::XML_Convert_Result::COMPLETED
                                || convert_res == super::xmltok::XML_Convert_Result::INPUT_INCOMPLETE
                            {
                                break;
                            }
                            *eventPP = buf.as_ptr()
                        }
                    } else {
                        handlers.characterData(&ExpatBufRef::<XML_Char>::from(buf.with_end(next)));
                    }
                } else if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, enc_type, buf.with_end(next));
                }
            }
            XML_TOK::INVALID => {
                *eventPP = next;
                return XML_Error::INVALID_TOKEN;
            }
            XML_TOK::PARTIAL_CHAR => {
                if haveMore {
                    *nextPtr = buf.as_ptr();
                    return XML_Error::NONE;
                }
                return XML_Error::PARTIAL_CHAR;
            }
            XML_TOK::PARTIAL | XML_TOK::NONE => {
                if haveMore {
                    *nextPtr = buf.as_ptr();
                    return XML_Error::NONE;
                }
                return XML_Error::UNCLOSED_CDATA_SECTION;
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
                return XML_Error::UNEXPECTED_STATE;
            }
        }
        buf = buf.with_start(next);
        *eventPP = buf.as_ptr();
        match (*parser).m_parsingStatus.parsing {
            XML_Parsing::SUSPENDED => {
                *nextPtr = next;
                return XML_Error::NONE;
            }
            XML_Parsing::FINISHED => return XML_Error::ABORTED,
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
    buf: ExpatBufRef,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut opt_buf = Some(buf);
    let mut result: XML_Error = doIgnoreSection(
        parser,
        EncodingType::Normal,
        &mut opt_buf,
        endPtr,
        !(*parser).m_parsingStatus.finalBuffer,
    );
    if result != XML_Error::NONE {
        return result;
    }
    if let Some(buf) = opt_buf {
        (*parser).m_processor = Some(prologProcessor as Processor);
        return prologProcessor(parser, buf, endPtr);
    }
    result
}
/* startPtr gets set to non-null is the section is closed, and to null
   if the section is not yet closed.
*/

unsafe extern "C" fn doIgnoreSection(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    start_buf: &mut Option<ExpatBufRef>,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
) -> XML_Error {
    let mut next: *const c_char = ptr::null();
    let mut buf = start_buf.unwrap().clone();
    let mut eventPP: *mut *const c_char = ptr::null_mut();
    let mut eventEndPP: *mut *const c_char = ptr::null_mut();
    if !enc_type.is_internal() {
        eventPP = &mut (*parser).m_eventPtr;
        *eventPP = buf.as_ptr();
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
    *eventPP = buf.as_ptr();
    *start_buf = None;
    let enc = (*parser).encoding(enc_type);
    let tok = (*enc).xmlTok(XML_STATE::IGNORE_SECTION, buf, &mut next);
    *eventEndPP = next;
    match tok {
        XML_TOK::IGNORE_SECT => {
            if (*parser).m_handlers.hasDefault() {
                reportDefault(parser, enc_type, buf.with_end(next));
            }
            *start_buf = Some(buf.with_start(next));
            *nextPtr = next;
            if (*parser).m_parsingStatus.parsing == XML_Parsing::FINISHED {
                return XML_Error::ABORTED;
            } else {
                return XML_Error::NONE;
            }
            /* LCOV_EXCL_STOP */
        }
        XML_TOK::INVALID => {
            *eventPP = next; /* XML_ERROR_UNCLOSED_IGNORE_SECTION */
            return XML_Error::INVALID_TOKEN;
        }
        XML_TOK::PARTIAL_CHAR => {
            if haveMore {
                *nextPtr = buf.as_ptr();
                return XML_Error::NONE;
            }
            return XML_Error::PARTIAL_CHAR;
        }
        XML_TOK::PARTIAL | XML_TOK::NONE => {
            if haveMore {
                *nextPtr = buf.as_ptr();
                return XML_Error::NONE;
            }
            return XML_Error::SYNTAX;
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
            return XML_Error::UNEXPECTED_STATE;
        }
    };
    /* not reached */
}
/* XML_DTD */

impl<'scf> XML_ParserStruct<'scf> {
    unsafe fn initializeEncoding(&mut self) -> XML_Error {
        let mut s: *const c_char = ptr::null();
        if cfg!(feature = "unicode") {
            let mut encodingBuf: [libc::c_char; 128] = [0; 128];
            /* See comments abount `protoclEncodingName` in parserInit() */
            if self.m_protocolEncodingName.is_null() {
                s = ptr::null();
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
        let enc = if self.m_ns {
            InitEncoding::new_ns(&mut self.m_encoding, s)
        } else {
            InitEncoding::new(&mut self.m_encoding, s)
        };
        if enc.is_some() {
            self.m_initEncoding = enc;
            self.m_encoding = &*self.m_initEncoding.as_ref().unwrap();
            return XML_Error::NONE;
        }

        self.m_handlers.handleUnknownEncoding(self.m_protocolEncodingName, self.m_ns, &mut self.m_encoding)
    }

    unsafe fn processXmlDecl(
        &mut self,
        mut isGeneralTextEntity: c_int,
        buf: ExpatBufRef,
    ) -> XML_Error {
        let mut encodingName: *const c_char = ptr::null();
        let mut storedEncName = ptr::null();
        let mut newEncoding: Option<*const ENCODING> = None;
        let mut version_buf = None;
        let mut storedversion = ptr::null();
        let mut standalone: c_int = -(1);
        if if self.m_ns {
            super::xmltok::XmlParseXmlDeclNS
        } else {
            super::xmltok::XmlParseXmlDecl
        }(
            isGeneralTextEntity,
            &*self.m_encoding,
            buf,
            &mut self.m_eventPtr,
            &mut version_buf,
            &mut encodingName,
            &mut newEncoding,
            &mut standalone,
        ) == 0
        {
            if isGeneralTextEntity != 0 {
                return XML_Error::TEXT_DECL;
            } else {
                return XML_Error::XML_DECL;
            }
        }
        if isGeneralTextEntity == 0 && standalone == 1 {
            (*self.m_dtd).standalone = true;
            if self.m_paramEntityParsing == XML_ParamEntityParsing::UNLESS_STANDALONE {
                self.m_paramEntityParsing = XML_ParamEntityParsing::NEVER
            }
            /* XML_DTD */
        }
        if self.m_handlers.hasXmlDecl() {
            if !encodingName.is_null() {
                let successful = self.m_temp2Pool.store_c_string(
                    &*self.m_encoding,
                    ExpatBufRef::new(
                        encodingName,
                        encodingName.offset((*self.m_encoding).nameLength(encodingName) as isize),
                    ),
                );
                if !successful {
                    return XML_Error::NO_MEMORY;
                }

                storedEncName = self.m_temp2Pool.current_start();
            }
            if let Some(version_buf) = version_buf {
                let successful = self.m_temp2Pool.store_c_string(
                    &*self.m_encoding,
                    version_buf,
                );
                if !successful {
                    return XML_Error::NO_MEMORY;
                }

                storedversion = self.m_temp2Pool.current_start();
            }
            self.m_handlers.xmlDecl(
                storedversion,
                storedEncName,
                standalone,
            );
        } else if self.m_handlers.hasDefault() {
            reportDefault(self, EncodingType::Normal, buf);
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
                    return XML_Error::INCORRECT_ENCODING;
                }
                self.m_encoding = newEncoding
            } else if !encodingName.is_null() {
                let mut result: XML_Error = XML_Error::NONE;
                if storedEncName.is_null() {
                    let successful = self.m_temp2Pool.store_c_string(
                        &*self.m_encoding,
                        ExpatBufRef::new(
                            encodingName,
                            encodingName.offset((*self.m_encoding).nameLength(encodingName) as isize),
                        ),
                    );

                    if !successful {
                        return XML_Error::NO_MEMORY;
                    }

                    storedEncName = self.m_temp2Pool.current_start();
                };
                result = self.m_handlers.handleUnknownEncoding(storedEncName, self.m_ns, &mut self.m_encoding);
                self.m_temp2Pool.clear();
                if result == XML_Error::UNKNOWN_ENCODING {
                    self.m_eventPtr = encodingName
                }
                return result;
            }
        }
        if !storedEncName.is_null() || !storedversion.is_null() {
            self.m_temp2Pool.clear();
        }
        XML_Error::NONE
    }
}

impl<'scf> CXmlHandlers<'scf> {
    unsafe fn handleUnknownEncoding(
        &mut self,
        mut encodingName: *const XML_Char,
        m_ns: bool,
        m_encoding: &mut *const ENCODING,
    ) -> XML_Error {
        if self.hasUnknownEncoding() {
            let mut info: XML_Encoding = XML_Encoding {
                map: [0; 256],
                data: ptr::null_mut(),
                convert: None,
                release: None,
            };
            let mut i: c_int = 0;
            i = 0;
            while i < 256 {
                info.map[i as usize] = -(1);
                i += 1
            }
            info.convert = None;
            info.data = ptr::null_mut();
            info.release = None;

            // Unwrapping because the handler was already checked to exist
            if self.unknownEncoding(encodingName, &mut info).unwrap() != 0 {
                let mut unknown_enc = UnknownEncoding::new();
                let initialized = unknown_enc.initialize(
                    info.map.as_mut_ptr(),
                    info.convert,
                    info.data,
                    m_ns,
                );
                if initialized {
                    match Box::try_new(unknown_enc) {
                        Err(_) => {
                            if info.release.is_some() {
                                info.release.expect("non-null function pointer")(info.data);
                            }
                            return XML_Error::NO_MEMORY;
                        }
                        Ok(unknown_enc) => {
                            *m_encoding = &*unknown_enc;
                            self.m_unknownEncoding = Some(unknown_enc);
                            self.m_unknownEncodingData = info.data;
                            self.m_unknownEncodingRelease = info.release;
                            return XML_Error::NONE;
                        }
                    }
                }
            }
            if info.release.is_some() {
                info.release.expect("non-null function pointer")(info.data);
            }
        }
        XML_Error::UNKNOWN_ENCODING
    }
}

unsafe extern "C" fn prologInitProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).initializeEncoding();
    if result != XML_Error::NONE {
        return result;
    }
    (*parser).m_processor = Some(prologProcessor as Processor);
    prologProcessor(parser, buf, nextPtr)
}

unsafe extern "C" fn externalParEntInitProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).initializeEncoding();
    if result != XML_Error::NONE {
        return result;
    }
    /* we know now that XML_Parse(Buffer) has been called,
    so we consider the external parameter entity read */
    (*(*parser).m_dtd).paramEntityRead = true;
    if (*parser).m_prologState.inEntityValue != 0 {
        (*parser).m_processor = Some(entityValueInitProcessor as Processor);
        entityValueInitProcessor(parser, buf, nextPtr)
    } else {
        (*parser).m_processor = Some(externalParEntProcessor as Processor);
        externalParEntProcessor(parser, buf, nextPtr)
    }
}

unsafe extern "C" fn entityValueInitProcessor(
    mut parser: XML_Parser,
    init_buf: ExpatBufRef,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut buf = init_buf.clone();
    let mut next: *const c_char = buf.as_ptr();
    (*parser).m_eventPtr = buf.as_ptr();
    loop {
        let tok = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf, &mut next);
        (*parser).m_eventEndPtr = next;
        if tok.to_i32().unwrap() <= 0 {
            if !(*parser).m_parsingStatus.finalBuffer && tok != XML_TOK::INVALID {
                *nextPtr = buf.as_ptr();
                return XML_Error::NONE;
            }
            match tok {
                XML_TOK::INVALID => return XML_Error::INVALID_TOKEN,
                XML_TOK::PARTIAL => return XML_Error::UNCLOSED_TOKEN,
                XML_TOK::PARTIAL_CHAR => return XML_Error::PARTIAL_CHAR,
                XML_TOK::NONE | _ => {}
            }
            /* found end of entity value - can store it now */
            return storeEntityValue(parser, EncodingType::Normal, init_buf);
        } else {
            if tok == XML_TOK::XML_DECL {
                let mut result: XML_Error = XML_Error::NONE;
                result = (*parser).processXmlDecl(0, buf.with_end(next));
                if result != XML_Error::NONE {
                    return result;
                }
                /* At this point, m_parsingStatus.parsing cannot be XML_Parsing::SUSPENDED.  For
                 * that to happen, a parameter entity parsing handler must have attempted
                 * to suspend the parser, which fails and raises an error.  The parser can
                 * be aborted, but can't be suspended.
                 */
                if (*parser).m_parsingStatus.parsing == XML_Parsing::FINISHED {
                    return XML_Error::ABORTED;
                }
                *nextPtr = next;
                /* stop scanning for text declaration - we found one */
                (*parser).m_processor = Some(entityValueProcessor as Processor);
                return entityValueProcessor(parser, buf.with_start(next), nextPtr);
            } else {
                /* If we are at the end of the buffer, this would cause XmlPrologTok to
                   return XML_TOK::NONE on the next call, which would then cause the
                   function to exit with *nextPtr set to s - that is what we want for other
                   tokens, but not for the BOM - we would rather like to skip it;
                   then, when this routine is entered the next time, XmlPrologTok will
                   return XML_TOK::INVALID, since the BOM is still in the buffer
                */
                if tok == XML_TOK::BOM
                    && next == buf.end()
                    && !(*parser).m_parsingStatus.finalBuffer
                {
                    *nextPtr = next;
                    return XML_Error::NONE;
                } else {
                    /* If we get this token, we have the start of what might be a
                       normal tag, but not a declaration (i.e. it doesn't begin with
                       "<!").  In a DTD context, that isn't legal.
                    */
                    if tok == XML_TOK::INSTANCE_START {
                        *nextPtr = next;
                        return XML_Error::SYNTAX;
                    }
                }
            }
        }
        buf = buf.with_start(next);
        (*parser).m_eventPtr = buf.as_ptr();
    }
}

unsafe extern "C" fn externalParEntProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = buf.as_ptr();
    let mut tok = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf, &mut next);
    if tok.to_i32().unwrap() <= 0 {
        if !(*parser).m_parsingStatus.finalBuffer && tok != XML_TOK::INVALID {
            *nextPtr = buf.as_ptr();
            return XML_Error::NONE;
        }
        match tok {
            XML_TOK::INVALID => return XML_Error::INVALID_TOKEN,
            XML_TOK::PARTIAL => return XML_Error::UNCLOSED_TOKEN,
            XML_TOK::PARTIAL_CHAR => return XML_Error::PARTIAL_CHAR,
            XML_TOK::NONE | _ => {}
        }
    } else if tok == XML_TOK::BOM {
        buf = buf.with_start(next);
        tok = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf, &mut next)
    }
    (*parser).m_processor = Some(prologProcessor as Processor);
    return (*parser).doProlog(
        EncodingType::Normal,
        buf,
        tok,
        next,
        nextPtr,
        !(*parser).m_parsingStatus.finalBuffer,
        true,
    );
}

unsafe extern "C" fn entityValueProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = buf.as_ptr();
    let mut enc: &ENCODING = &*(*parser).m_encoding;
    let mut tok = XML_TOK::INVALID;
    loop {
        tok = (*enc).xmlTok(XML_STATE::PROLOG, buf, &mut next);
        if tok.to_i32().unwrap() <= 0 {
            if !(*parser).m_parsingStatus.finalBuffer && tok != XML_TOK::INVALID {
                *nextPtr = buf.as_ptr();
                return XML_Error::NONE;
            }
            match tok {
                XML_TOK::INVALID => return XML_Error::INVALID_TOKEN,
                XML_TOK::PARTIAL => return XML_Error::UNCLOSED_TOKEN,
                XML_TOK::PARTIAL_CHAR => return XML_Error::PARTIAL_CHAR,
                XML_TOK::NONE | _ => {}
            }
            /* This would cause the next stage, i.e. doProlog to be passed XML_TOK::BOM.
               However, when parsing an external subset, doProlog will not accept a BOM
               as valid, and report a syntax error, so we have to skip the BOM
            */
            /* found end of entity value - can store it now */
            return storeEntityValue(parser, EncodingType::Normal, buf);
        }
        buf = buf.with_start(next);
    }
}
/* XML_DTD */

unsafe extern "C" fn prologProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = buf.as_ptr();
    let mut tok = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf, &mut next);
    return (*parser).doProlog(
        EncodingType::Normal,
        buf,
        tok,
        next,
        nextPtr,
        !(*parser).m_parsingStatus.finalBuffer,
        true,
    );
}

impl<'scf> XML_ParserStruct<'scf> {
    unsafe fn doPrologHandleEntityRef(
        &mut self,
        mut entity: *mut Entity,
        role: XML_ROLE,
        handleDefault: &mut bool,
        dtd: *mut DTD,
    ) -> Option<XML_Error> {
        if (*entity).open {
            return Some(XML_Error::RECURSIVE_ENTITY_REF);
        }
        if !(*entity).textPtr.is_null() {
            let mut result_4 = XML_Error::NONE;
            let mut betweenDecl =
                if role == XML_ROLE::PARAM_ENTITY_REF {
                    true
                } else {
                    false
                };
            result_4 = self.processInternalEntity(entity, betweenDecl);
            if result_4 != XML_Error::NONE {
                return Some(result_4);
            }
            *handleDefault = false;
        } else if self.m_handlers.hasExternalEntityRef() {
            (*dtd).paramEntityRead = false;
            (*entity).open = true;
            let entity_name = if cfg!(feature = "mozilla") {
                (*entity).name
            } else {
                ptr::null()
            };
            if self.m_handlers.externalEntityRef(
                entity_name,
                (*entity).base,
                (*entity).systemId,
                (*entity).publicId,
            ) == Ok(0)
            {
                (*entity).open = false;
                return Some(XML_Error::EXTERNAL_ENTITY_HANDLING);
            }
            (*entity).open = false;
            *handleDefault = false;
            if !(*dtd).paramEntityRead {
                (*dtd).keepProcessing = (*dtd).standalone;
            } else {
                /* XML_DTD */
                if !(*dtd).standalone && self.m_handlers.notStandalone() == Ok(0) {
                    return Some(XML_Error::NOT_STANDALONE);
                }
            }
        } else {
            (*dtd).keepProcessing = (*dtd).standalone;
        }
        None
    }

    unsafe fn doProlog(
        &mut self,
        mut enc_type: EncodingType,
        mut buf: ExpatBufRef,
        mut tok: XML_TOK,
        mut next: *const c_char,
        mut nextPtr: *mut *const c_char,
        mut haveMore: XML_Bool,
        mut allowClosingDoctype: XML_Bool,
    ) -> XML_Error {
        const externalSubsetName: [XML_Char; 2] = XML_STR![ASCII_HASH];
        const atypeCDATA: [XML_Char; 6] = XML_STR![ASCII_C, ASCII_D, ASCII_A, ASCII_T, ASCII_A];
        const atypeID: [XML_Char; 3] = XML_STR![ASCII_I, ASCII_D];
        const atypeIDREF: [XML_Char; 6] = XML_STR![ASCII_I, ASCII_D, ASCII_R, ASCII_E, ASCII_F];
        const atypeIDREFS: [XML_Char; 7] =
            XML_STR![ASCII_I, ASCII_D, ASCII_R, ASCII_E, ASCII_F, ASCII_S];
        const atypeENTITY: [XML_Char; 7] =
            XML_STR![ASCII_E, ASCII_N, ASCII_T, ASCII_I, ASCII_T, ASCII_Y];
        const atypeENTITIES: [XML_Char; 9] =
            XML_STR![ASCII_E, ASCII_N, ASCII_T, ASCII_I, ASCII_T, ASCII_I, ASCII_E, ASCII_S];
        const atypeNMTOKEN: [XML_Char; 8] =
            XML_STR![ASCII_N, ASCII_M, ASCII_T, ASCII_O, ASCII_K, ASCII_E, ASCII_N];
        const atypeNMTOKENS: [XML_Char; 9] =
            XML_STR![ASCII_N, ASCII_M, ASCII_T, ASCII_O, ASCII_K, ASCII_E, ASCII_N, ASCII_S];
        const notationPrefix: [XML_Char; 10] = XML_STR![
            ASCII_N, ASCII_O, ASCII_T, ASCII_A, ASCII_T, ASCII_I, ASCII_O, ASCII_N, ASCII_LPAREN
        ];
        const enumValueSep: [XML_Char; 2] = XML_STR![ASCII_PIPE];
        const enumValueStart: [XML_Char; 2] = XML_STR![ASCII_LPAREN];
        /* save one level of indirection */
        let dtd: *mut DTD = self.m_dtd;
        let mut eventPP: *mut *const c_char = ptr::null_mut();
        let mut eventEndPP: *mut *const c_char = ptr::null_mut();
        let mut quant: XML_Content_Quant = XML_Content_Quant::NONE;
        if enc_type.is_internal() {
            eventPP = &mut (*self.m_openInternalEntities).internalEventPtr;
            eventEndPP = &mut (*self.m_openInternalEntities).internalEventEndPtr
        } else {
            eventPP = &mut self.m_eventPtr;
            eventEndPP = &mut self.m_eventEndPtr
        }
        let mut enc = self.encoding(enc_type);
        loop {
            let mut handleDefault = true;
            *eventPP = buf.as_ptr();
            *eventEndPP = next;
            if tok.is_error() {
                if haveMore && tok != XML_TOK::INVALID {
                    *nextPtr = buf.as_ptr();
                    return XML_Error::NONE;
                }
                match tok {
                    XML_TOK::INVALID => {
                        *eventPP = next;
                        return XML_Error::INVALID_TOKEN;
                    }
                    XML_TOK::PARTIAL => return XML_Error::UNCLOSED_TOKEN,
                    XML_TOK::PARTIAL_CHAR => return XML_Error::PARTIAL_CHAR,

                    XML_TOK::PROLOG_S_NEG => tok = XML_TOK::PROLOG_S,
                    XML_TOK::NONE => {
                        /* for internal PE NOT referenced between declarations */
                        if enc_type.is_internal() && !(*self.m_openInternalEntities).betweenDecl {
                            *nextPtr = buf.as_ptr();
                            return XML_Error::NONE;
                        }
                        /* WFC: PE Between Declarations - must check that PE contains
                        complete markup, not only for external PEs, but also for
                        internal PEs if the reference occurs between declarations.
                         */
                        if self.m_isParamEntity || enc_type.is_internal() {
                            if self
                                .m_prologState
                                .handler
                                .expect("non-null function pointer")(
                                    &mut self.m_prologState,
                                    XML_TOK::NONE,
                                    // TODO(SJC): is this right??
                                    ExpatBufRef::empty(),
                                    enc,
                                ) == super::xmlrole::XML_ROLE::ERROR
                            {
                                return XML_Error::INCOMPLETE_PE;
                            }
                            *nextPtr = buf.as_ptr();
                            return XML_Error::NONE;
                        }
                        /* XML_DTD */
                        return XML_Error::NO_ELEMENTS;
                    }
                    _ => {
                        // tok = -tok; /* end of big switch */
                        tok = tok.negate();
                        next = buf.end();
                    }
                }
            } /* always initialize to NULL */
            let role = self
                .m_prologState
                .handler
                .expect("non-null function pointer")(
                    &mut self.m_prologState, tok, buf.with_end(next), enc
                );
            match role {
                XML_ROLE::XML_DECL => {
                    let mut result: XML_Error = self.processXmlDecl(0, buf.with_end(next));
                    if result != XML_Error::NONE {
                        return result;
                    }
                    enc = self.encoding(EncodingType::Normal);
                    handleDefault = false;
                }
                XML_ROLE::DOCTYPE_NAME => {
                    if self.m_handlers.hasStartDoctypeDecl() {
                        if !self.m_tempPool.store_c_string(enc, buf.with_end(next)) {
                            self.m_doctypeName = ptr::null();
                            return XML_Error::NO_MEMORY;
                        }
                        self.m_doctypeName = self.m_tempPool.finish_string().as_ptr();
                        self.m_doctypePubid = ptr::null();
                        handleDefault = false
                    }
                    self.m_doctypeSysid = ptr::null();
                }
                XML_ROLE::DOCTYPE_INTERNAL_SUBSET => {
                    let startHandlerRan = self.m_handlers.startDoctypeDecl(
                        self.m_doctypeName,
                        self.m_doctypeSysid,
                        self.m_doctypePubid,
                        1,
                    );

                    if startHandlerRan {
                        self.m_doctypeName = ptr::null();
                        self.m_tempPool.clear();
                        handleDefault = false
                    }
                }
                XML_ROLE::TEXT_DECL => {
                    let mut result_0: XML_Error = self.processXmlDecl(1, buf.with_end(next));
                    if result_0 != XML_Error::NONE {
                        return result_0;
                    }
                    enc = self.encoding(EncodingType::Normal);
                    handleDefault = false;
                }
                XML_ROLE::DOCTYPE_PUBLIC_ID => {
                    /* XML_DTD */
                    self.m_useForeignDTD = false;
                    self.m_declEntity = hash_insert!(
                        &mut (*dtd).paramEntities,
                        externalSubsetName.as_ptr(),
                        Entity
                    );
                    if self.m_declEntity.is_null() {
                        return XML_Error::NO_MEMORY;
                    }
                    /* XML_DTD */
                    (*dtd).hasParamEntityRefs = true;
                    if self.m_handlers.hasStartDoctypeDecl() {
                        if (*enc).isPublicId(buf.with_end(next), &mut *eventPP) == 0 {
                            return XML_Error::PUBLICID;
                        }
                        let successful = self.m_tempPool.store_c_string(
                            enc,
                            buf
                                .inc_start((*enc).minBytesPerChar() as isize)
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar() as usize)
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        }
                        let pub_id = self.m_tempPool.finish_string_cells();
                        normalizePublicId(pub_id.as_ptr() as *const _ as *mut _);
                        self.m_doctypePubid = pub_id.as_ptr() as *const _;
                        handleDefault = false;
                    } else {
                        if (*enc).isPublicId(buf.with_end(next), &mut *eventPP) == 0 {
                            return XML_Error::PUBLICID;
                        }
                    }
                }
                XML_ROLE::ENTITY_PUBLIC_ID => {
                    if (*enc).isPublicId(buf.with_end(next), &mut *eventPP) == 0 {
                        return XML_Error::PUBLICID;
                    }
                }
                XML_ROLE::DOCTYPE_CLOSE => {
                    if allowClosingDoctype != true {
                        /* Must not close doctype from within expanded parameter entities */
                        return XML_Error::INVALID_TOKEN;
                    }
                    if !self.m_doctypeName.is_null() {
                        self.m_handlers.startDoctypeDecl(
                            self.m_doctypeName,
                            self.m_doctypeSysid,
                            self.m_doctypePubid,
                            0,
                        );
                        self.m_tempPool.clear();
                        handleDefault = false
                    }
                    /* parser->m_doctypeSysid will be non-NULL in the case of a previous
                    XML_ROLE::DOCTYPE_SYSTEM_ID, even if parser->m_startDoctypeDeclHandler
                    was not set, indicating an external subset
                     */
                    if !self.m_doctypeSysid.is_null() || self.m_useForeignDTD {
                        let mut hadParamEntityRefs = (*dtd).hasParamEntityRefs;
                        (*dtd).hasParamEntityRefs = true;
                        if self.m_paramEntityParsing != XML_ParamEntityParsing::NEVER
                            && self.m_handlers.hasExternalEntityRef()
                        {
                            let mut entity = hash_insert!(
                                &mut (*dtd).paramEntities,
                                externalSubsetName.as_ptr(),
                                Entity
                            );
                            if entity.is_null() {
                                /* end of DTD - no need to update dtd->keepProcessing */
                                /* The external subset name "#" will have already been
                                 * inserted into the hash table at the start of the
                                 * external entity parsing, so no allocation will happen
                                 * and lookup() cannot fail.
                                 */
                                return XML_Error::NO_MEMORY;
                                /* LCOV_EXCL_LINE */
                            }
                            if self.m_useForeignDTD {
                                (*entity).base = self.m_curBase
                            }
                            (*dtd).paramEntityRead = false;
                            if self.m_handlers.externalEntityRef(
                                ptr::null(),
                                (*entity).base,
                                (*entity).systemId,
                                (*entity).publicId,
                            ) == Ok(0)
                            {
                                return XML_Error::EXTERNAL_ENTITY_HANDLING;
                            }
                            if (*dtd).paramEntityRead {
                                if !(*dtd).standalone && self.m_handlers.notStandalone() == Ok(0) {
                                    return XML_Error::NOT_STANDALONE;
                                }
                            } else if self.m_doctypeSysid.is_null() {
                                (*dtd).hasParamEntityRefs = hadParamEntityRefs
                            }
                        }
                        self.m_useForeignDTD = false
                    }
                    /* if we didn't read the foreign DTD then this means that there
                    is no external subset and we must reset dtd->hasParamEntityRefs
                     */
                    /* XML_DTD */
                    if self.m_handlers.endDoctypeDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::INSTANCE_START => {
                    /* if there is no DOCTYPE declaration then now is the
                    last chance to read the foreign DTD
                     */
                    if self.m_useForeignDTD {
                        let mut hadParamEntityRefs = (*dtd).hasParamEntityRefs;
                        (*dtd).hasParamEntityRefs = true;
                        if self.m_paramEntityParsing != XML_ParamEntityParsing::NEVER && self.m_handlers.hasExternalEntityRef() {
                            let mut entity = hash_insert!(
                                &mut (*dtd).paramEntities,
                                externalSubsetName.as_ptr(),
                                Entity
                            );
                            if entity.is_null() {
                                return XML_Error::NO_MEMORY;
                            }
                            (*entity).base = self.m_curBase;
                            (*dtd).paramEntityRead = false;
                            if self.m_handlers.externalEntityRef(
                                ptr::null(),
                                (*entity).base,
                                (*entity).systemId,
                                (*entity).publicId,
                            ) == Ok(0)
                            {
                                return XML_Error::EXTERNAL_ENTITY_HANDLING;
                            }
                            if (*dtd).paramEntityRead  {
                                if !(*dtd).standalone && self.m_handlers.notStandalone() == Ok(0) {
                                    return XML_Error::NOT_STANDALONE;
                                }
                            } else {
                                /* end of DTD - no need to update dtd->keepProcessing */
                                /* if we didn't read the foreign DTD then this means that there
                                is no external subset and we must reset dtd->hasParamEntityRefs
                                 */
                                (*dtd).hasParamEntityRefs = hadParamEntityRefs
                            }
                        }
                    }
                    /* XML_DTD */
                    self.m_processor = Some(contentProcessor as Processor);
                    return contentProcessor(self, buf, nextPtr);
                }
                XML_ROLE::ATTLIST_ELEMENT_NAME => {
                    self.m_declElementType = self.getElementType(enc_type, buf.with_end(next));
                    if self.m_declElementType.is_null() {
                        return XML_Error::NO_MEMORY;
                    }
                }
                XML_ROLE::ATTRIBUTE_NAME => {
                    self.m_declAttributeId = self.getAttributeId(enc_type, buf.with_end(next));
                    if self.m_declAttributeId.is_null() {
                        return XML_Error::NO_MEMORY;
                    }
                    self.m_declAttributeIsCdata = false;
                    self.m_declAttributeType = ptr::null();
                    self.m_declAttributeIsId = false;
                }
                XML_ROLE::ATTRIBUTE_TYPE_CDATA => {
                    self.m_declAttributeIsCdata = true;
                    self.m_declAttributeType = atypeCDATA.as_ptr();
                }
                XML_ROLE::ATTRIBUTE_TYPE_ID => {
                    self.m_declAttributeIsId = true;
                    self.m_declAttributeType = atypeID.as_ptr();
                }
                XML_ROLE::ATTRIBUTE_TYPE_IDREF => {
                    self.m_declAttributeType = atypeIDREF.as_ptr();
                }
                XML_ROLE::ATTRIBUTE_TYPE_IDREFS => {
                    self.m_declAttributeType = atypeIDREFS.as_ptr();
                }
                XML_ROLE::ATTRIBUTE_TYPE_ENTITY => {
                    self.m_declAttributeType = atypeENTITY.as_ptr();
                }
                XML_ROLE::ATTRIBUTE_TYPE_ENTITIES => {
                    self.m_declAttributeType = atypeENTITIES.as_ptr();
                }
                XML_ROLE::ATTRIBUTE_TYPE_NMTOKEN => {
                    self.m_declAttributeType = atypeNMTOKEN.as_ptr();
                }
                XML_ROLE::ATTRIBUTE_TYPE_NMTOKENS => {
                    self.m_declAttributeType = atypeNMTOKENS.as_ptr();
                }
                XML_ROLE::ATTRIBUTE_ENUM_VALUE | XML_ROLE::ATTRIBUTE_NOTATION_VALUE => {
                    if (*dtd).keepProcessing && self.m_handlers.hasAttlistDecl() {
                        let mut prefix: *const XML_Char = ptr::null();
                        if !self.m_declAttributeType.is_null() {
                            prefix = enumValueSep.as_ptr()
                        } else {
                            prefix = if role == super::xmlrole::XML_ROLE::ATTRIBUTE_NOTATION_VALUE {
                                notationPrefix.as_ptr()
                            } else {
                                enumValueStart.as_ptr()
                            }
                        }
                        if !self.m_tempPool.append_c_string(prefix) {
                            return XML_Error::NO_MEMORY;
                        }
                        if !self.m_tempPool.append(enc, buf.with_end(next)) {
                            return XML_Error::NO_MEMORY;
                        }
                        // This gets finalized later, not exactly the safest
                        // idiom though.
                        self.m_declAttributeType = self.m_tempPool.current_start();
                        handleDefault = false
                    }
                }
                XML_ROLE::IMPLIED_ATTRIBUTE_VALUE | XML_ROLE::REQUIRED_ATTRIBUTE_VALUE => {
                    if (*dtd).keepProcessing {
                        if defineAttribute(
                            self.m_declElementType,
                            self.m_declAttributeId,
                            self.m_declAttributeIsCdata,
                            self.m_declAttributeIsId,
                            ptr::null(),
                        ) == 0
                        {
                            return XML_Error::NO_MEMORY;
                        }
                        if self.m_handlers.hasAttlistDecl()
                            && !self.m_declAttributeType.is_null()
                        {
                            if *self.m_declAttributeType == ASCII_LPAREN as XML_Char
                                || *self.m_declAttributeType == ASCII_N as XML_Char
                                && *self.m_declAttributeType.offset(1) == ASCII_O as XML_Char
                            {
                                /* Enumerated or Notation type */
                                if !self.m_tempPool.append_char(ASCII_RPAREN as XML_Char)
                                || !self.m_tempPool.append_char('\u{0}' as XML_Char) {
                                    return XML_Error::NO_MEMORY;
                                }
                                self.m_declAttributeType = self.m_tempPool.finish_string().as_ptr();
                            }
                            *eventEndPP = buf.as_ptr();
                            self.m_handlers.attlistDecl(
                                (*self.m_declElementType).name,
                                (*self.m_declAttributeId).name.name(),
                                self.m_declAttributeType,
                                ptr::null(),
                                (role == super::xmlrole::XML_ROLE::REQUIRED_ATTRIBUTE_VALUE) as c_int,
                            );
                            self.m_tempPool.clear();
                            handleDefault = false
                        }
                    }
                }
                XML_ROLE::DEFAULT_ATTRIBUTE_VALUE | XML_ROLE::FIXED_ATTRIBUTE_VALUE => {
                    if (*dtd).keepProcessing {
                        let mut attVal: *const XML_Char = ptr::null();
                        let mut result_1: XML_Error = storeAttributeValue(
                            self,
                            enc_type,
                            self.m_declAttributeIsCdata,
                            buf
                                .inc_start((*enc).minBytesPerChar() as isize)
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar() as usize),
                            &(*dtd).pool,
                        );
                        if result_1 as u64 != 0 {
                            return result_1;
                        }
                        attVal = (*dtd).pool.finish_string().as_ptr();
                        /* ID attributes aren't allowed to have a default */
                        if defineAttribute(
                            self.m_declElementType,
                            self.m_declAttributeId,
                            self.m_declAttributeIsCdata,
                            false,
                            attVal,
                        ) == 0
                        {
                            return XML_Error::NO_MEMORY;
                        }
                        if self.m_handlers.hasAttlistDecl()
                            && !self.m_declAttributeType.is_null()
                        {
                            if *self.m_declAttributeType == ASCII_LPAREN as XML_Char
                                || *self.m_declAttributeType == ASCII_N as XML_Char
                                && *self.m_declAttributeType.offset(1) == ASCII_O as XML_Char
                            {
                                /* Enumerated or Notation type */
                                if !self.m_tempPool.append_char(ASCII_RPAREN as XML_Char)
                                || !self.m_tempPool.append_char('\u{0}' as XML_Char) {
                                    return XML_Error::NO_MEMORY;
                                }
                                self.m_declAttributeType = self.m_tempPool.finish_string().as_ptr();
                            }
                            *eventEndPP = buf.as_ptr();
                            self.m_handlers.attlistDecl(
                                (*self.m_declElementType).name,
                                (*self.m_declAttributeId).name.name(),
                                self.m_declAttributeType,
                                attVal,
                                (role == super::xmlrole::XML_ROLE::FIXED_ATTRIBUTE_VALUE) as c_int,
                            );
                            self.m_tempPool.clear();
                            handleDefault = false
                        }
                    }
                }
                XML_ROLE::ENTITY_VALUE => {
                    if (*dtd).keepProcessing {
                        let mut result_2: XML_Error = storeEntityValue(
                            self,
                            enc_type,
                            buf
                                .inc_start((*enc).minBytesPerChar() as isize)
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar() as usize)
                        );
                        if !self.m_declEntity.is_null() {
                            (*self.m_declEntity).textLen = (*dtd).entityValuePool.len() as c_int;
                            (*self.m_declEntity).textPtr = (*dtd).entityValuePool.finish_string().as_ptr();
                            if self.m_handlers.hasEntityDecl() {
                                *eventEndPP = buf.as_ptr();
                                self.m_handlers.entityDecl(
                                    (*self.m_declEntity).name,
                                    (*self.m_declEntity).is_param as c_int,
                                    (*self.m_declEntity).textPtr,
                                    (*self.m_declEntity).textLen,
                                    self.m_curBase,
                                    ptr::null(),
                                    ptr::null(),
                                    ptr::null(),
                                );
                                handleDefault = false
                            }
                        } else {
                            (*dtd).entityValuePool.clear_current();
                        }
                        if result_2 != XML_Error::NONE {
                            return result_2;
                        }
                    }
                }
                XML_ROLE::DOCTYPE_SYSTEM_ID => {
                    self.m_useForeignDTD = false;
                    /* XML_DTD */
                    (*dtd).hasParamEntityRefs = true;
                    if self.m_handlers.hasStartDoctypeDecl() {
                        let successful = self.m_tempPool.store_c_string(
                            enc,
                            buf
                                .inc_start((*enc).minBytesPerChar() as isize)
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar() as usize)
                        );
                        if !successful {
                            self.m_doctypeSysid = ptr::null();
                            return XML_Error::NO_MEMORY;
                        }
                        self.m_doctypeSysid = self.m_tempPool.finish_string().as_ptr();

                        handleDefault = false
                    } else {
                        /* use externalSubsetName to make parser->m_doctypeSysid non-NULL
                        for the case where no parser->m_startDoctypeDeclHandler is set */
                        self.m_doctypeSysid = externalSubsetName.as_ptr()
                    }
                    /* XML_DTD */
                    if !(*dtd).standalone
                        && self.m_paramEntityParsing as u64 == 0
                        && self.m_handlers.notStandalone() == Ok(0)
                    {
                        return XML_Error::NOT_STANDALONE;
                    }
                    /* XML_DTD */
                    if self.m_declEntity.is_null() {
                        self.m_declEntity = hash_insert!(
                            &mut (*dtd).paramEntities,
                            externalSubsetName.as_ptr(),
                            Entity
                        );
                        if self.m_declEntity.is_null() {
                            return XML_Error::NO_MEMORY;
                        }
                        (*self.m_declEntity).publicId = ptr::null()
                    }
                
                }
                XML_ROLE::ENTITY_SYSTEM_ID => { }
                XML_ROLE::ENTITY_COMPLETE => {
                    if (*dtd).keepProcessing
                        && !self.m_declEntity.is_null()
                        && self.m_handlers.hasEntityDecl()
                    {
                        *eventEndPP = buf.as_ptr();
                        self.m_handlers.entityDecl(
                            (*self.m_declEntity).name,
                            (*self.m_declEntity).is_param as c_int,
                            ptr::null(),
                            0,
                            (*self.m_declEntity).base,
                            (*self.m_declEntity).systemId,
                            (*self.m_declEntity).publicId,
                            ptr::null(),
                        );
                        handleDefault = false
                    }
                }
                XML_ROLE::ENTITY_NOTATION_NAME => {
                    if (*dtd).keepProcessing && !self.m_declEntity.is_null() {
                        if !(*dtd).pool.store_c_string(enc, buf.with_end(next)) {
                            (*self.m_declEntity).notation = ptr::null();
                            return XML_Error::NO_MEMORY;
                        }
                        (*self.m_declEntity).notation = (*dtd).pool.finish_string().as_ptr();
                        if self.m_handlers.hasUnparsedEntityDecl() {
                            *eventEndPP = buf.as_ptr();
                            self.m_handlers.unparsedEntityDecl(
                                (*self.m_declEntity).name,
                                (*self.m_declEntity).base,
                                (*self.m_declEntity).systemId,
                                (*self.m_declEntity).publicId,
                                (*self.m_declEntity).notation,
                            );
                            handleDefault = false
                        } else if self.m_handlers.hasEntityDecl() {
                            *eventEndPP = buf.as_ptr();
                            self.m_handlers.entityDecl(
                                (*self.m_declEntity).name,
                                0,
                                ptr::null(),
                                0,
                                (*self.m_declEntity).base,
                                (*self.m_declEntity).systemId,
                                (*self.m_declEntity).publicId,
                                (*self.m_declEntity).notation,
                            );
                            handleDefault = false
                        }
                    }
                }
                XML_ROLE::GENERAL_ENTITY_NAME => {
                    if (*enc).predefinedEntityName(buf.with_end(next)) != 0 {
                        self.m_declEntity = ptr::null_mut();
                    } else if (*dtd).keepProcessing {
                        if !(*dtd).pool.store_c_string(enc, buf.with_end(next)) {
                            return XML_Error::NO_MEMORY;
                        }
                        self.m_declEntity = (*dtd).pool.current_slice(|name| hash_insert!(
                            &mut (*dtd).generalEntities,
                            name.as_ptr(),
                            Entity
                        ));
                        if self.m_declEntity.is_null() {
                            // FIXME: this never happens in Rust, it just panics
                            return XML_Error::NO_MEMORY;
                        }
                        if (*self.m_declEntity).name != (*dtd).pool.current_start() {
                            (*dtd).pool.clear_current();
                            self.m_declEntity = ptr::null_mut();
                        } else {
                            (*dtd).pool.finish_string();
                            (*self.m_declEntity).publicId = ptr::null();
                            (*self.m_declEntity).is_param = false;
                            /* if we have a parent parser or are reading an internal parameter
                            entity, then the entity declaration is not considered "internal"
                             */
                            (*self.m_declEntity).is_internal =
                                !(!self.m_parentParser.is_null()
                                  || !self.m_openInternalEntities.is_null())
                                as XML_Bool;
                            if self.m_handlers.hasEntityDecl() {
                                handleDefault = false
                            }
                        }
                    } else {
                        (*dtd).pool.clear_current();
                        self.m_declEntity = ptr::null_mut();
                    }
                }
                XML_ROLE::PARAM_ENTITY_NAME => {
                    if (*dtd).keepProcessing {
                        if !(*dtd).pool.store_c_string(enc, buf.with_end(next)) {
                            return XML_Error::NO_MEMORY;
                        }
                        self.m_declEntity = (*dtd).pool.current_slice(|name_0| hash_insert!(
                            &mut (*dtd).paramEntities,
                            name_0.as_ptr(),
                            Entity
                        ));
                        if self.m_declEntity.is_null() {
                            return XML_Error::NO_MEMORY;
                        }
                        if (*self.m_declEntity).name != (*dtd).pool.current_start() {
                            (*dtd).pool.clear_current();
                            self.m_declEntity = ptr::null_mut();
                        } else {
                            (*dtd).pool.finish_string();
                            (*self.m_declEntity).publicId = ptr::null();
                            (*self.m_declEntity).is_param = true;
                            /* if we have a parent parser or are reading an internal parameter
                            entity, then the entity declaration is not considered "internal"
                             */
                            (*self.m_declEntity).is_internal =
                                !(!self.m_parentParser.is_null()
                                  || !self.m_openInternalEntities.is_null());
                            if self.m_handlers.hasEntityDecl() {
                                handleDefault = false
                            }
                        }
                    } else {
                        (*dtd).pool.clear_current();
                        self.m_declEntity = ptr::null_mut()
                    }
                }
                XML_ROLE::NOTATION_NAME => {
                    self.m_declNotationPublicId = ptr::null();
                    self.m_declNotationName = ptr::null();
                    if self.m_handlers.hasNotationDecl() {
                        if !self.m_tempPool.store_c_string(enc, buf.with_end(next)) {
                            self.m_declNotationName = ptr::null();
                            return XML_Error::NO_MEMORY;
                        }
                        self.m_declNotationName = self.m_tempPool.finish_string().as_ptr();
                        handleDefault = false
                    }
                }
                XML_ROLE::NOTATION_PUBLIC_ID => {
                    if (*enc).isPublicId(buf.with_end(next), &mut *eventPP) == 0 {
                        return XML_Error::PUBLICID;
                    }
                    if !self.m_declNotationName.is_null() {
                        /* means m_notationDeclHandler != NULL */
                        let successful = self.m_tempPool.store_c_string(
                            enc,
                            buf
                                .inc_start((*enc).minBytesPerChar() as isize)
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar() as usize)
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        }
                        let tem_0 = self.m_tempPool.finish_string_cells();
                        normalizePublicId(tem_0.as_ptr() as *const _ as *mut _);
                        self.m_declNotationPublicId = tem_0.as_ptr() as *const _;
                        handleDefault = false
                    }
                }
                XML_ROLE::NOTATION_SYSTEM_ID => {
                    if !self.m_declNotationName.is_null()
                        && self.m_handlers.hasNotationDecl()
                    {
                        let successful = self.m_tempPool.store_c_string(
                            enc,
                            buf
                                .inc_start((*enc).minBytesPerChar() as isize)
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar() as usize)
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        }
                        *eventEndPP = buf.as_ptr();
                        self.m_tempPool.current_slice(|systemId| self.m_handlers.notationDecl(
                            self.m_declNotationName,
                            self.m_curBase,
                            systemId.as_ptr(),
                            self.m_declNotationPublicId,
                        ));
                        handleDefault = false
                    }
                    self.m_tempPool.clear();
                }
                XML_ROLE::NOTATION_NO_SYSTEM_ID => {
                    if !self.m_declNotationPublicId.is_null()
                        && self.m_handlers.hasNotationDecl()
                    {
                        *eventEndPP = buf.as_ptr();
                        self.m_handlers.notationDecl(
                            self.m_declNotationName,
                            self.m_curBase,
                            ptr::null(),
                            self.m_declNotationPublicId,
                        );
                        handleDefault = false
                    }
                    self.m_tempPool.clear();
                }
                XML_ROLE::ERROR => {
                    match tok {
                        XML_TOK::PARAM_ENTITY_REF => {
                            /* PE references in internal subset are
                            not allowed within declarations. */
                            return XML_Error::PARAM_ENTITY_REF;
                        }
                        XML_TOK::XML_DECL => return XML_Error::MISPLACED_XML_PI,
                        _ => return XML_Error::SYNTAX,
                    }
                }
                XML_ROLE::IGNORE_SECT => {
                    let mut result_3: XML_Error = XML_Error::NONE;
                    if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.with_end(next));
                    }
                    handleDefault = false;
                    let mut ignore_buf = Some(buf.with_start(next));
                    result_3 = doIgnoreSection(self, enc_type, &mut ignore_buf, nextPtr, haveMore);
                    if result_3 != XML_Error::NONE {
                        return result_3;
                    } else {
                        if ignore_buf.is_none() {
                            self.m_processor = Some(ignoreSectionProcessor as Processor);
                            return result_3;
                        }
                    }
                    next = ignore_buf.unwrap().as_ptr();
                }
                XML_ROLE::GROUP_OPEN => {
                    /* XML_DTD */
                    if self.m_prologState.level >= self.m_groupSize {
                        if self.m_groupSize != 0 {
                            self.m_groupSize = self.m_groupSize.wrapping_mul(2);
                            let new_connector = REALLOC!(
                                self.m_groupConnector => [c_char; self.m_groupSize]);
                            if new_connector.is_null() {
                                self.m_groupSize = self.m_groupSize.wrapping_div(2);
                                return XML_Error::NO_MEMORY;
                            }
                            self.m_groupConnector = new_connector;
                        } else {
                            self.m_groupSize = 32;
                            self.m_groupConnector = MALLOC![c_char; self.m_groupSize];
                            if self.m_groupConnector.is_null() {
                                self.m_groupSize = 0;
                                return XML_Error::NO_MEMORY;
                            }
                        }
                    }
                    *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize) = 0;
                    if (*dtd).in_eldecl {
                        let mut scf = (*dtd).scaffold.borrow_mut();
                        if scf.index.try_reserve(1).is_err() {
                            return XML_Error::NO_MEMORY;
                        }
                        match scf.next_part() {
                            Some(myindex) => {
                                scf.index.push(myindex);
                                scf.scaffold[myindex].type_0 = XML_Content_Type::SEQ;
                            }
                            None => return XML_Error::NO_MEMORY
                        };
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = false
                        }
                    }
                }
                XML_ROLE::GROUP_SEQUENCE => {
                    if *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize)
                        == ASCII_PIPE
                    {
                        return XML_Error::SYNTAX;
                    }
                    *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize) = ASCII_COMMA;
                    if (*dtd).in_eldecl && self.m_handlers.hasElementDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::GROUP_CHOICE => {
                    if *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize)
                        == ASCII_COMMA
                    {
                        return XML_Error::SYNTAX;
                    }
                    if (*dtd).in_eldecl
                        && *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize)
                        == 0
                    {
                        let mut scf = (*dtd).scaffold.borrow_mut();
                        let idx = scf.index.last().copied().unwrap();
                        if scf.scaffold[idx].type_0 != XML_Content_Type::MIXED
                        {
                            scf.scaffold[idx].type_0 = XML_Content_Type::CHOICE;
                            if self.m_handlers.hasElementDecl() {
                                handleDefault = false
                            }
                        }
                    }
                    *self
                        .m_groupConnector
                        .offset(self.m_prologState.level as isize) = ASCII_PIPE;
                }
                XML_ROLE::PARAM_ENTITY_REF | XML_ROLE::INNER_PARAM_ENTITY_REF => {
                    (*dtd).hasParamEntityRefs = true;
                    if self.m_paramEntityParsing as u64 == 0 {
                        (*dtd).keepProcessing = (*dtd).standalone;
                        /* XML_DTD */
                        if !(*dtd).standalone && self.m_handlers.notStandalone() == Ok(0) {
                            return XML_Error::NOT_STANDALONE;
                        }
                    } else {
                        let successful = (*dtd).pool.store_c_string(
                            enc,
                            buf
                                .inc_start((*enc).minBytesPerChar() as isize)
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar() as usize)
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        }
                        let mut entity = (*dtd).pool.current_slice(|name| hash_lookup!((*dtd).paramEntities, name.as_ptr()));
                        /* first, determine if a check for an existing declaration is needed;
                        if yes, check that the entity exists, and that it is internal,
                        otherwise call the skipped entity handler
                         */
                        // REXPAT: Moved `clear_current` from outside the following branches
                        // into them to avoid a use-after-free when the 2nd branch uses the
                        // `name` pointer again.
                        if self.m_prologState.documentEntity != 0
                            && (if (*dtd).standalone {
                                self.m_openInternalEntities.is_null()
                            } else {
                                !(*dtd).hasParamEntityRefs
                            })
                        {
                            (*dtd).pool.clear_current();
                            if entity.is_null() {
                                return XML_Error::UNDEFINED_ENTITY;
                            } else {
                                if !(*entity).is_internal {
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
                                    return XML_Error::ENTITY_DECLARED_IN_PE;
                                    /* LCOV_EXCL_LINE */
                                }
                            }
                            if let Some(r) = self.doPrologHandleEntityRef(entity, role, &mut handleDefault, dtd) {
                                return r;
                            }
                            /* XML_DTD */
                            if !(*dtd).standalone && self.m_handlers.notStandalone() == Ok(0) {
                                return XML_Error::NOT_STANDALONE;
                            }
                        } else if entity.is_null() {
                            (*dtd).keepProcessing = (*dtd).standalone;
                            /* cannot report skipped entities in declarations */
                            if role == super::xmlrole::XML_ROLE::PARAM_ENTITY_REF
                                && self.m_handlers.hasSkippedEntity()
                            {
                                (*dtd).pool.current_slice(|name| {
                                    self.m_handlers.skippedEntity(name.as_ptr(), 1);
                                });
                                handleDefault = false;
                            }
                            (*dtd).pool.clear_current();
                        } else {
                            (*dtd).pool.clear_current();
                            if let Some(r) = self.doPrologHandleEntityRef(entity, role, &mut handleDefault,     dtd) {
                                return r;
                            }
                            /* XML_DTD */
                            if !(*dtd).standalone && self.m_handlers.notStandalone() == Ok(0) {
                                return XML_Error::NOT_STANDALONE;
                            }
                        }
                    }
                }
                XML_ROLE::ELEMENT_NAME => {
                    /* Element declaration stuff */
                    if self.m_handlers.hasElementDecl() {
                        self.m_declElementType = self.getElementType(enc_type, buf.with_end(next));
                        if self.m_declElementType.is_null() {
                            return XML_Error::NO_MEMORY;
                        }
                        // FIXME: turn into new Cow::Owned instead???
                        let mut scf = (*dtd).scaffold.borrow_mut();
                        scf.scaffold.clear();
                        scf.index.clear();
                        (*dtd).in_eldecl = true;
                        handleDefault = false
                    }
                }
                XML_ROLE::CONTENT_ANY | XML_ROLE::CONTENT_EMPTY => {
                    if (*dtd).in_eldecl {
                        if self.m_handlers.hasElementDecl() {
                            let mut content: *mut XML_Content = MALLOC!(@XML_Content);
                            if content.is_null() {
                                return XML_Error::NO_MEMORY;
                            }
                            (*content).quant = XML_Content_Quant::NONE;
                            (*content).name = ptr::null_mut();
                            (*content).numchildren = 0;
                            (*content).children = ptr::null_mut();
                            (*content).type_0 = if role == super::xmlrole::XML_ROLE::CONTENT_ANY {
                                XML_Content_Type::ANY
                            } else {
                                XML_Content_Type::EMPTY
                            };
                            *eventEndPP = buf.as_ptr();
                            self.m_handlers.elementDecl((*self.m_declElementType).name, content);
                            handleDefault = false
                        }
                        (*dtd).in_eldecl = false
                    }
                }
                XML_ROLE::CONTENT_PCDATA => {
                    if (*dtd).in_eldecl {
                        let mut scf = (*dtd).scaffold.borrow_mut();
                        let idx = scf.index.last().copied().unwrap();
                        scf.scaffold[idx].type_0 = XML_Content_Type::MIXED;
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = false
                        }
                    }
                }
                XML_ROLE::CONTENT_ELEMENT => {
                    quant = XML_Content_Quant::NONE;
                }
                XML_ROLE::CONTENT_ELEMENT_OPT => {
                    quant = XML_Content_Quant::OPT;
                }
                XML_ROLE::CONTENT_ELEMENT_REP => {
                    quant = XML_Content_Quant::REP;
                }
                XML_ROLE::CONTENT_ELEMENT_PLUS => {
                    quant = XML_Content_Quant::PLUS;
                }
                XML_ROLE::GROUP_CLOSE => {
                    quant = XML_Content_Quant::NONE;
                }
                XML_ROLE::GROUP_CLOSE_OPT => {
                    quant = XML_Content_Quant::OPT;
                }
                XML_ROLE::GROUP_CLOSE_REP => {
                    quant = XML_Content_Quant::REP;
                }
                XML_ROLE::GROUP_CLOSE_PLUS => {
                    quant = XML_Content_Quant::PLUS;
                }
                XML_ROLE::PI => {
                    /* End element declaration stuff */
                    if reportProcessingInstruction(self, enc_type, buf.with_end(next)) == 0 {
                        return XML_Error::NO_MEMORY;
                    }
                    handleDefault = false;
                }
                XML_ROLE::COMMENT => {
                    if reportComment(self, enc_type, buf.with_end(next)) == 0 {
                        return XML_Error::NO_MEMORY;
                    }
                    handleDefault = false;
                }
                XML_ROLE::NONE => {
                    match tok {
                        XML_TOK::BOM => handleDefault = false,
                        _ => {}
                    }
                }
                XML_ROLE::DOCTYPE_NONE => {
                    if self.m_handlers.hasStartDoctypeDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::ENTITY_NONE => {
                    if (*dtd).keepProcessing && self.m_handlers.hasEntityDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::NOTATION_NONE => {
                    if self.m_handlers.hasNotationDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::ATTLIST_NONE => {
                    if (*dtd).keepProcessing && self.m_handlers.hasAttlistDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::ELEMENT_NONE => {
                    if self.m_handlers.hasElementDecl() {
                        handleDefault = false
                    }
                }
            }
            match role {
                XML_ROLE::ATTRIBUTE_NAME |
                XML_ROLE::ATTLIST_ELEMENT_NAME |
                XML_ROLE::ATTRIBUTE_TYPE_CDATA |
                XML_ROLE::ATTRIBUTE_TYPE_ID |
                XML_ROLE::ATTRIBUTE_TYPE_IDREF |
                XML_ROLE::ATTRIBUTE_TYPE_IDREFS |
                XML_ROLE::ATTRIBUTE_TYPE_ENTITY |
                XML_ROLE::ATTRIBUTE_TYPE_ENTITIES |
                XML_ROLE::ATTRIBUTE_TYPE_NMTOKEN |
                XML_ROLE::ATTRIBUTE_TYPE_NMTOKENS => {
                    if (*dtd).keepProcessing && self.m_handlers.hasAttlistDecl() {
                        handleDefault = false
                    }
                },
                XML_ROLE::CONTENT_ELEMENT | 
                XML_ROLE::CONTENT_ELEMENT_OPT | 
                XML_ROLE::CONTENT_ELEMENT_REP | 
                XML_ROLE::CONTENT_ELEMENT_PLUS => {
                    if (*dtd).in_eldecl {
                        let mut scf = (*dtd).scaffold.borrow_mut();
                        let myindex = match scf.next_part() {
                            Some(myindex) => myindex,
                            None => return XML_Error::NO_MEMORY
                        };
                        scf.scaffold[myindex].type_0 = XML_Content_Type::NAME;
                        scf.scaffold[myindex].quant = quant;

                        let mut nxt: *const c_char = if quant == XML_Content_Quant::NONE {
                            next
                        } else {
                            next.offset(-((*enc).minBytesPerChar() as isize))
                        };
                        let mut el = self.getElementType(enc_type, buf.with_end(nxt));
                        if el.is_null() {
                            return XML_Error::NO_MEMORY;
                        }
                        scf.scaffold[myindex].name = (*el).name;

                        let mut name_2 = (*el).name;
                        let mut nameLen = 0;
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
                            handleDefault = false
                        }
                    }                    
                }
                XML_ROLE::GROUP_CLOSE |
                XML_ROLE::GROUP_CLOSE_OPT |
                XML_ROLE::GROUP_CLOSE_REP |
                XML_ROLE::GROUP_CLOSE_PLUS => {
                    if (*dtd).in_eldecl {
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = false
                        }
                        let empty_scaffold = {
                            let mut scf = (*dtd).scaffold.borrow_mut();
                            let idx = scf.index.pop().unwrap();
                            scf.scaffold[idx].quant = quant;
                            scf.index.is_empty()
                        };
                        if empty_scaffold {
                            if !handleDefault {
                                let mut model: *mut XML_Content = self.build_model();
                                if model.is_null() {
                                    return XML_Error::NO_MEMORY;
                                }
                                *eventEndPP = buf.as_ptr();
                                self.m_handlers.elementDecl((*self.m_declElementType).name, model);
                            }
                            (*dtd).in_eldecl = false;
                            (*dtd).contentStringLen = 0
                        }
                    }
                },
                XML_ROLE::DOCTYPE_PUBLIC_ID |
                XML_ROLE::ENTITY_PUBLIC_ID => {
                    if (*dtd).keepProcessing && !self.m_declEntity.is_null() {
                        let successful = (*dtd).pool.store_c_string(
                            enc,
                            buf
                                .inc_start((*enc).minBytesPerChar() as isize)
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar() as usize)
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        }
                        let mut tem = (*dtd).pool.finish_string_cells();
                        normalizePublicId(tem.as_ptr() as *const _ as *mut _);
                        (*self.m_declEntity).publicId = tem.as_ptr() as *const _;
                        /* Don't suppress the default handler if we fell through from
                         * the XML_ROLE::DOCTYPE_PUBLIC_ID case.
                         */
                        if self.m_handlers.hasEntityDecl()
                            && role == super::xmlrole::XML_ROLE::ENTITY_PUBLIC_ID
                        {
                            handleDefault = false
                        }
                    }
                }
                XML_ROLE::DOCTYPE_SYSTEM_ID | 
                XML_ROLE::ENTITY_SYSTEM_ID => {
                    if (*dtd).keepProcessing && !self.m_declEntity.is_null() {
                        let successful = (*dtd).pool.store_c_string(
                            enc,
                            buf
                                .inc_start((*enc).minBytesPerChar() as isize)
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar() as usize),
                        );
                        if !successful {
                            (*self.m_declEntity).systemId = ptr::null();
                            return XML_Error::NO_MEMORY;
                        }
                        (*self.m_declEntity).systemId = (*dtd).pool.finish_string().as_ptr();
                        (*self.m_declEntity).base = self.m_curBase;
                        /* Don't suppress the default handler if we fell through from
                         * the XML_ROLE::DOCTYPE_SYSTEM_ID case.
                         */
                        if self.m_handlers.hasEntityDecl()
                            && role == super::xmlrole::XML_ROLE::ENTITY_SYSTEM_ID
                        {
                            handleDefault = false
                        }
                    }
                }
                _ => {}
            }
            /* not XML_DTD */
            /* XML_DTD */
            if handleDefault && self.m_handlers.hasDefault() {
                reportDefault(self, enc_type, buf.with_end(next));
            }
            match self.m_parsingStatus.parsing {
                XML_Parsing::SUSPENDED => {
                    *nextPtr = next;
                    return XML_Error::NONE;
                }
                XML_Parsing::FINISHED => return XML_Error::ABORTED,
                _ => {
                    buf = buf.with_start(next);
                    tok = (*enc).xmlTok(XML_STATE::PROLOG, buf, &mut next)
                }
            }
        }
        /* not reached */
    }
}
/* XML_DTD */

unsafe extern "C" fn epilogProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    (*parser).m_processor = Some(epilogProcessor as Processor);
    (*parser).m_eventPtr = buf.as_ptr();
    loop {
        let mut next: *const c_char = ptr::null();
        let mut tok = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf, &mut next);
        (*parser).m_eventEndPtr = next;
        match tok {
            XML_TOK::PROLOG_S_NEG => {
                /* report partial linebreak - it might be the last token */
                if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, EncodingType::Normal, buf.with_end(next));
                    if (*parser).m_parsingStatus.parsing == XML_Parsing::FINISHED {
                        return XML_Error::ABORTED;
                    }
                }
                *nextPtr = next;
                return XML_Error::NONE;
            }
            XML_TOK::NONE => {
                *nextPtr = buf.as_ptr();
                return XML_Error::NONE;
            }
            XML_TOK::PROLOG_S => {
                if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, EncodingType::Normal, buf.with_end(next));
                }
            }
            XML_TOK::PI => {
                if reportProcessingInstruction(parser, EncodingType::Normal, buf.with_end(next)) == 0 {
                    return XML_Error::NO_MEMORY;
                }
            }
            XML_TOK::COMMENT => {
                if reportComment(parser, EncodingType::Normal, buf.with_end(next)) == 0 {
                    return XML_Error::NO_MEMORY;
                }
            }
            XML_TOK::INVALID => {
                (*parser).m_eventPtr = next;
                return XML_Error::INVALID_TOKEN;
            }
            XML_TOK::PARTIAL => {
                if !(*parser).m_parsingStatus.finalBuffer {
                    *nextPtr = buf.as_ptr();
                    return XML_Error::NONE;
                }
                return XML_Error::UNCLOSED_TOKEN;
            }
            XML_TOK::PARTIAL_CHAR => {
                if !(*parser).m_parsingStatus.finalBuffer {
                    *nextPtr = buf.as_ptr();
                    return XML_Error::NONE;
                }
                return XML_Error::PARTIAL_CHAR;
            }
            _ => return XML_Error::JUNK_AFTER_DOC_ELEMENT,
        }
        buf = buf.with_start(next);
        (*parser).m_eventPtr = buf.as_ptr();
        match (*parser).m_parsingStatus.parsing {
            XML_Parsing::SUSPENDED => {
                *nextPtr = next;
                return XML_Error::NONE;
            }
            XML_Parsing::FINISHED => return XML_Error::ABORTED,
            _ => {}
        }
    }
}

impl<'scf> XML_ParserStruct<'scf> {
    unsafe fn processInternalEntity(
        &mut self,
        mut entity: *mut Entity,
        mut betweenDecl: XML_Bool,
    ) -> XML_Error {
        let mut next: *const c_char = ptr::null();
        let mut result: XML_Error = XML_Error::NONE;
        let mut openEntity: *mut OpenInternalEntity = ptr::null_mut();
        if !self.m_freeInternalEntities.is_null() {
            openEntity = self.m_freeInternalEntities;
            self.m_freeInternalEntities = (*openEntity).next
        } else {
            openEntity = MALLOC!(@OpenInternalEntity);
            if openEntity.is_null() {
                return XML_Error::NO_MEMORY;
            }
        }
        (*entity).open = true;
        (*entity).processed = 0;
        (*openEntity).next = self.m_openInternalEntities;
        self.m_openInternalEntities = openEntity;
        (*openEntity).entity = entity;
        (*openEntity).startTagLevel = self.m_tagLevel;
        (*openEntity).betweenDecl = betweenDecl;
        (*openEntity).internalEventPtr = ptr::null();
        (*openEntity).internalEventEndPtr = ptr::null();
        let text_buf = ExpatBufRef::new(
            (*entity).textPtr as *mut c_char,
            (*entity).textPtr.add((*entity).textLen as usize) as *mut c_char,
        );
        /* Set a safe default value in case 'next' does not get set */
        next = text_buf.as_ptr();
        if (*entity).is_param {
            let mut tok =
                (*self.m_internalEncoding).xmlTok(XML_STATE::PROLOG, text_buf, &mut next);
            result = self.doProlog(
                EncodingType::Internal,
                text_buf,
                tok,
                next,
                &mut next,
                false,
                false,
            )
        } else {
            /* XML_DTD */
            result = self.doContent(
                self.m_tagLevel,
                EncodingType::Internal,
                text_buf,
                &mut next,
                false,
            )
        }
        if result == XML_Error::NONE {
            if text_buf.end() != next && self.m_parsingStatus.parsing == XML_Parsing::SUSPENDED {
                (*entity).processed = next.wrapping_offset_from(text_buf.as_ptr()) as i32;
                self.m_processor = Some(internalEntityProcessor as Processor)
            } else {
                (*entity).open = false;
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
        result
    }
}

unsafe extern "C" fn internalEntityProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatBufRef,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut entity: *mut Entity = ptr::null_mut();
    let mut next: *const c_char = ptr::null();
    let mut result: XML_Error = XML_Error::NONE;
    let mut openEntity: *mut OpenInternalEntity = (*parser).m_openInternalEntities;
    if openEntity.is_null() {
        return XML_Error::UNEXPECTED_STATE;
    }
    entity = (*openEntity).entity;
    let text_buf = ExpatBufRef::new(
        ((*entity).textPtr as *mut c_char).offset((*entity).processed as isize),
        (*entity).textPtr.offset((*entity).textLen as isize) as *mut c_char,
    );
    /* Set a safe default value in case 'next' does not get set */
    next = text_buf.as_ptr();
    if (*entity).is_param {
        let mut tok =
            (*(*parser).m_internalEncoding).xmlTok(XML_STATE::PROLOG, text_buf, &mut next);
        result = (*parser).doProlog(
            EncodingType::Internal,
            text_buf,
            tok,
            next,
            &mut next,
            false,
            true,
        )
    } else {
        /* XML_DTD */
        result = (*parser).doContent(
            (*openEntity).startTagLevel,
            EncodingType::Internal,
            text_buf,
            &mut next,
            false,
        )
    }
    if result != XML_Error::NONE {
        return result;
    } else {
        if text_buf.end() != next && (*parser).m_parsingStatus.parsing == XML_Parsing::SUSPENDED {
            (*entity).processed =
                next.wrapping_offset_from((*entity).textPtr as *mut c_char) as c_int;
            return result;
        } else {
            (*entity).open = false;
            (*parser).m_openInternalEntities = (*openEntity).next;
            /* put openEntity back in list of free instances */
            (*openEntity).next = (*parser).m_freeInternalEntities;
            (*parser).m_freeInternalEntities = openEntity
        }
    }
    if (*entity).is_param {
        (*parser).m_processor = Some(prologProcessor as Processor);
        let tok_0 = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf, &mut next);
        (*parser).doProlog(
            EncodingType::Normal,
            buf,
            tok_0,
            next,
            nextPtr,
            !(*parser).m_parsingStatus.finalBuffer,
            true,
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
            buf,
            nextPtr,
            !(*parser).m_parsingStatus.finalBuffer,
        )
    } /* save one level of indirection */
}

unsafe extern "C" fn errorProcessor(
    mut parser: XML_Parser,
    mut _buf: ExpatBufRef,
    mut _nextPtr: *mut *const c_char,
) -> XML_Error {
    (*parser).m_errorCode
}

unsafe extern "C" fn storeAttributeValue(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut isCdata: XML_Bool,
    mut buf: ExpatBufRef,
    mut pool: &StringPool,
) -> XML_Error {
    let mut result: XML_Error = appendAttributeValue(parser, enc_type, isCdata, buf, pool);
    if result as u64 != 0 {
        return result;
    }
    if !isCdata && !pool.is_empty() && pool.get_last_char() as c_int == 0x20 {
        pool.backtrack();
    }
    if !pool.append_char('\u{0}' as XML_Char) {
        return XML_Error::NO_MEMORY;
    }
    XML_Error::NONE
}

unsafe extern "C" fn appendAttributeValue(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut isCdata: XML_Bool,
    mut buf: ExpatBufRef,
    mut pool: &StringPool,
) -> XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    let enc = (*parser).encoding(enc_type);
    loop {
        let mut next: *const c_char = ptr::null();
        let mut tok = (*enc).xmlLiteralTok(XML_ATTRIBUTE_VALUE_LITERAL, buf, &mut next);
        match tok {
            XML_TOK::NONE => {
                return XML_Error::NONE;
                /* LCOV_EXCL_STOP */
            }
            XML_TOK::INVALID => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr = next
                }
                return XML_Error::INVALID_TOKEN;
            }
            XML_TOK::PARTIAL => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr = buf.as_ptr();
                }
                return XML_Error::INVALID_TOKEN;
            }
            XML_TOK::CHAR_REF => {
                let mut out_buf: [XML_Char; XML_ENCODE_MAX] = [0; XML_ENCODE_MAX];
                let mut i: c_int = 0;
                let mut n: c_int = (*enc).charRefNumber(ExpatBufRef(&buf));
                if n < 0 {
                    if !enc_type.is_internal() {
                        (*parser).m_eventPtr = buf.as_ptr();
                    }
                    return XML_Error::BAD_CHAR_REF;
                }
                // FIXME: simplify this if-then-else stmt
                if !isCdata && n == 0x20 && (pool.is_empty() || pool.get_last_char() as c_int == 0x20) {
                    // empty!
                } else {
                    n = XmlEncode(n, &mut out_buf);
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
                        if !pool.append_char(out_buf[i as usize]) {
                            return XML_Error::NO_MEMORY;
                        }
                        i += 1
                    }
                }
            }
            XML_TOK::DATA_CHARS => {
                if !pool.append(enc, buf.with_end(next)) {
                    return XML_Error::NO_MEMORY;
                }
            }
            XML_TOK::TRAILING_CR | 
            XML_TOK::ATTRIBUTE_VALUE_S | 
            XML_TOK::DATA_NEWLINE => {
                if tok ==  XML_TOK::TRAILING_CR {
                    next = buf.as_ptr().offset((*enc).minBytesPerChar() as isize);
                }
                
                if !(!isCdata && (pool.is_empty() || pool.get_last_char() as c_int == 0x20)) {
                    if !pool.append_char(0x20) {
                        return XML_Error::NO_MEMORY;
                    }
                }
            }
            XML_TOK::ENTITY_REF => {
                let mut checkEntityDecl = false;
                let mut ch: XML_Char = (*enc).predefinedEntityName(
                    buf
                        .inc_start((*enc).minBytesPerChar() as isize)
                        .with_end(next)
                        .dec_end((*enc).minBytesPerChar() as usize)
                ) as XML_Char;
                if ch != 0 {
                    if !(*pool).append_char(ch) {
                        return XML_Error::NO_MEMORY;
                    }
                } else {
                    let successful = (*parser).m_temp2Pool.store_c_string(
                        enc,
                        buf
                            .inc_start((*enc).minBytesPerChar() as isize)
                            .with_end(next)
                            .dec_end((*enc).minBytesPerChar() as usize),
                    );
                    if !successful {
                        return XML_Error::NO_MEMORY;
                    }

                    let entity = (*parser).m_temp2Pool.current_slice(|name| {
                        hash_lookup!((*dtd).generalEntities, name.as_ptr())
                    });

                    (*parser).m_temp2Pool.clear_current();

                    /* First, determine if a check for an existing declaration is needed;
                       if yes, check that the entity exists, and that it is internal.
                    */
                    if ptr::eq(pool, &mut (*dtd).pool) {
                        /* are we called from prolog? */
                        checkEntityDecl = (*parser).m_prologState.documentEntity != 0
                            && (if (*dtd).standalone {
                                (*parser).m_openInternalEntities.is_null()
                            } else {
                                !(*dtd).hasParamEntityRefs
                            })
                    } else {
                        /* if (pool == &parser->m_tempPool): we are called from content */
                        checkEntityDecl = !(*dtd).hasParamEntityRefs || (*dtd).standalone
                    }
                    if checkEntityDecl {
                        if entity.is_null() {
                            return XML_Error::UNDEFINED_ENTITY;
                        } else {
                            if !(*entity).is_internal {
                                return XML_Error::ENTITY_DECLARED_IN_PE;
                            }
                        }
                    } else if entity.is_null() {
                        if cfg!(feature = "mozilla") {
                            return XML_Error::UNDEFINED_ENTITY;
                        }
                    }

                    if checkEntityDecl || !entity.is_null() {
                        if (*entity).open {
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
                                (*parser).m_eventPtr = buf.as_ptr()
                                /* LCOV_EXCL_LINE */
                            }
                            return XML_Error::RECURSIVE_ENTITY_REF;
                        }
                        if !(*entity).notation.is_null() {
                            if !enc_type.is_internal() {
                                (*parser).m_eventPtr = buf.as_ptr()
                            }
                            return XML_Error::BINARY_ENTITY_REF;
                        }
                        if (*entity).textPtr.is_null() {
                            if !enc_type.is_internal() {
                                (*parser).m_eventPtr = buf.as_ptr()
                            }
                            return XML_Error::ATTRIBUTE_EXTERNAL_ENTITY_REF;
                        } else {
                            let mut result: XML_Error = XML_Error::NONE;
                            let mut textEnd: *const XML_Char =
                                (*entity).textPtr.offset((*entity).textLen as isize);
                            (*entity).open = true;
                            result = appendAttributeValue(
                                parser,
                                EncodingType::Internal,
                                isCdata,
                                ExpatBufRef::new(
                                    (*entity).textPtr as *const c_char,
                                    textEnd as *const c_char,
                                ),
                                pool,
                            );
                            (*entity).open = false;
                            if result != XML_Error::NONE {
                                return result;
                            }
                        }
                    }
                }
            }
            _ => {
                /* The only token returned by XmlAttributeValueTok() that does
                 * not have an explicit case here is XML_TOK::PARTIAL_CHAR.
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
                    (*parser).m_eventPtr = buf.as_ptr()
                }
                return XML_Error::UNEXPECTED_STATE;
            }
        }
        buf = buf.with_start(next);
    }
    /* not reached */
}

unsafe extern "C" fn storeEntityValue(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut entityTextBuf: ExpatBufRef,
) -> XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    let pool = &(*dtd).entityValuePool;
    let mut result: XML_Error = XML_Error::NONE;
    let mut oldInEntityValue: c_int = (*parser).m_prologState.inEntityValue;
    let enc = (*parser).encoding(enc_type);
    (*parser).m_prologState.inEntityValue = 1;
    /* XML_DTD */
    /* never return Null for the value argument in EntityDeclHandler,
    since this would indicate an external entity; therefore we
    have to make sure that entityValuePool.start is not null */
    's_41: loop {
        let mut next: *const c_char = ptr::null();
        let mut tok = (*enc).xmlLiteralTok(
            XML_ENTITY_VALUE_LITERAL,
            entityTextBuf,
            &mut next,
        );
        match tok {
            XML_TOK::PARAM_ENTITY_REF => {
                if (*parser).m_isParamEntity || enc_type.is_internal() {
                    let mut entity: *mut Entity = ptr::null_mut();
                    let successful = (*parser).m_tempPool.store_c_string(
                        enc,
                        entityTextBuf
                            .inc_start((*enc).minBytesPerChar() as isize)
                            .with_end(next)
                            .dec_end((*enc).minBytesPerChar() as usize),
                    );
                    if !successful {
                        result = XML_Error::NO_MEMORY;
                        break;
                    } else {
                        entity = (*parser).m_tempPool.current_slice(|name| {
                            hash_lookup!((*dtd).paramEntities, name.as_ptr())
                        });
                        (*parser).m_tempPool.clear_current();
                        if entity.is_null() {
                            /* not a well-formedness error - see XML 1.0: WFC Entity Declared */
                            /* cannot report skipped entity here - see comments on
                               parser->m_skippedEntityHandler
                            if (parser->m_skippedEntityHandler)
                              parser->m_skippedEntityHandler(parser->m_handlerArg, name, 0);
                            */
                            (*dtd).keepProcessing = (*dtd).standalone;
                            break;
                        } else if (*entity).open {
                            if !enc_type.is_internal() {
                                (*parser).m_eventPtr = entityTextBuf.as_ptr();
                            }
                            result = XML_Error::RECURSIVE_ENTITY_REF;
                            break;
                        } else if !(*entity).systemId.is_null() {
                            if (*parser).m_handlers.hasExternalEntityRef() {
                                (*dtd).paramEntityRead = false;
                                (*entity).open = true;
                                if (*parser).m_handlers.externalEntityRef(
                                    ptr::null(),
                                    (*entity).base,
                                    (*entity).systemId,
                                    (*entity).publicId,
                                ) == Ok(0)
                                {
                                    (*entity).open = false;
                                    result = XML_Error::EXTERNAL_ENTITY_HANDLING;
                                    break;
                                } else {
                                    (*entity).open = false;
                                    if !(*dtd).paramEntityRead {
                                        (*dtd).keepProcessing = (*dtd).standalone
                                    }
                                }
                            } else {
                                (*dtd).keepProcessing = (*dtd).standalone
                            }
                        } else {
                            (*entity).open = true;
                            result = storeEntityValue(
                                parser,
                                EncodingType::Internal,
                                ExpatBufRef::new(
                                    (*entity).textPtr as *mut c_char,
                                    (*entity).textPtr.offset((*entity).textLen as isize) as *mut c_char,
                                ),
                            );
                            (*entity).open = false;
                            if result as u64 != 0 {
                                break;
                            }
                        }
                    }
                } else {
                    /* XML_DTD */
                    /* In the internal subset, PE references are not legal
                    within markup declarations, e.g entity values in this case. */
                    (*parser).m_eventPtr = entityTextBuf.as_ptr();
                    result = XML_Error::PARAM_ENTITY_REF;
                    break;
                }
                /* LCOV_EXCL_STOP */
            }
            XML_TOK::NONE => {
                result = XML_Error::NONE;
                break;
            }
            XML_TOK::ENTITY_REF | XML_TOK::DATA_CHARS => {
                if !pool.append(enc, entityTextBuf.with_end(next)) {
                    result = XML_Error::NO_MEMORY;
                    break;
                }
            }
            XML_TOK::TRAILING_CR | XML_TOK::DATA_NEWLINE => {
                if tok == XML_TOK::TRAILING_CR {
                    next = entityTextBuf.as_ptr().offset((*enc).minBytesPerChar() as isize);
                }
                
                if !pool.append_char(0xa) {
                    result = XML_Error::NO_MEMORY;
                    break;
                }
            }
            XML_TOK::CHAR_REF => {
                let mut out_buf: [XML_Char; XML_ENCODE_MAX] = [0; XML_ENCODE_MAX];
                let mut i: c_int = 0;
                let mut n: c_int = (*enc).charRefNumber(entityTextBuf);
                if n < 0 {
                    if !enc_type.is_internal() {
                        (*parser).m_eventPtr = entityTextBuf.as_ptr();
                    }
                    result = XML_Error::BAD_CHAR_REF;
                    break;
                } else {
                    n = XmlEncode(n, &mut out_buf);
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
                        if !pool.append_char(out_buf[i as usize]) {
                            result = XML_Error::NO_MEMORY;
                            break 's_41;
                        } else {
                            i += 1
                        }
                    }
                }
            }
            XML_TOK::PARTIAL => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr = entityTextBuf.as_ptr();
                }
                result = XML_Error::INVALID_TOKEN;
                break;
            }
            XML_TOK::INVALID => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr = next
                }
                result = XML_Error::INVALID_TOKEN;
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
                    (*parser).m_eventPtr = entityTextBuf.as_ptr();
                }
                result = XML_Error::UNEXPECTED_STATE;
                break;
            }
        }
        entityTextBuf = entityTextBuf.with_start(next);
    }
    (*parser).m_prologState.inEntityValue = oldInEntityValue;
    /* XML_DTD */
    result
}

unsafe fn normalizeLines(mut s: &mut [XML_Char]) {
    // TODO: Safeify this
    let mut s = s.as_mut_ptr();
    let mut p: *mut XML_Char = ptr::null_mut();
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
    mut buf: ExpatBufRef,
) -> c_int {
    if !(*parser).m_handlers.hasProcessingInstruction() {
        if (*parser).m_handlers.hasDefault() {
            reportDefault(parser, enc_type, buf);
        }
        return 1;
    }
    let enc = (*parser).encoding(enc_type);
    buf = buf.inc_start(((*enc).minBytesPerChar() * 2) as isize);
    let tem = buf.inc_start((*enc).nameLength(buf.as_ptr()) as isize);
    let successful = (*parser).m_tempPool.store_c_string(enc, buf.with_len((*enc).nameLength(buf.as_ptr()) as usize));

    if !successful {
        return 0;
    }
    let target = (*parser).m_tempPool.finish_string_cells();
    let successful = (*parser).m_tempPool.store_c_string(
        enc,
        // TODO(SJC): fix this ugliness
        ExpatBufRef::new(
            (*enc).skipS(tem.as_ptr()),
            tem.end().offset(-(((*enc).minBytesPerChar() * 2) as isize)),
        ),
    );
    if !successful {
        return 0;
    }
    (*parser).m_tempPool.current_mut_slice(|data| {
        normalizeLines(data);
        (*parser).m_handlers.processingInstruction(target.as_ptr() as *mut _, data.as_ptr());
    });
    (*parser).m_tempPool.clear();
    1
}

unsafe extern "C" fn reportComment(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut buf: ExpatBufRef,
) -> c_int {
    if !(*parser).m_handlers.hasComment() {
        if (*parser).m_handlers.hasDefault() {
            reportDefault(parser, enc_type, buf);
        }
        return 1;
    }
    let enc = (*parser).encoding(enc_type);
    let successful = (*parser).m_tempPool.store_c_string(
        enc,
        buf.inc_start(((*enc).minBytesPerChar() * 4) as isize)
            .dec_end(((*enc).minBytesPerChar() * 3) as usize),
    );
    if !successful {
        return 0;
    };
    (*parser).m_tempPool.current_mut_slice(|data| {
        normalizeLines(data);
        (*parser).m_handlers.comment(data.as_ptr());
    });
    (*parser).m_tempPool.clear();
    1
}

unsafe extern "C" fn reportDefault(
    mut parser: XML_Parser,
    mut enc_type: EncodingType,
    mut buf: ExpatBufRef,
) {
    let enc = (*parser).encoding(enc_type);
    if MUST_CONVERT!(enc, buf.as_ptr()) {
        let mut convert_res: super::xmltok::XML_Convert_Result =
            super::xmltok::XML_Convert_Result::COMPLETED;
        let mut eventPP: *mut *const c_char = ptr::null_mut();
        let mut eventEndPP: *mut *const c_char = ptr::null_mut();
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
            let mut data_buf = ExpatBufRefMut::new(
                (*parser).m_dataBuf as *mut ICHAR,
                (*parser).m_dataBufEnd as *mut ICHAR,
            );
            convert_res = XmlConvert!(enc, &mut buf, &mut data_buf);
            *eventEndPP = buf.as_ptr();

            let defaultRan = (*parser).m_handlers.default(
                (*parser).m_dataBuf,
                data_buf.as_ptr().wrapping_offset_from((*parser).m_dataBuf).try_into().unwrap(),
            );

            // Previously unwrapped an Option
            if !defaultRan {
                panic!("Expected default handler to be set");
            }

            *eventPP = buf.as_ptr();
            if !(convert_res != super::xmltok::XML_Convert_Result::COMPLETED
                && convert_res != super::xmltok::XML_Convert_Result::INPUT_INCOMPLETE)
            {
                break;
            }
        }
    } else {
        let buf: ExpatBufRef<XML_Char> = buf.into();
        let defaultRan = (*parser).m_handlers.default(buf.as_ptr(), buf.len().try_into().unwrap());

        // Previously unwrapped an Option
        if !defaultRan {
            panic!("Expected default handler to be set");
        }
    };
}

unsafe extern "C" fn defineAttribute(
    mut type_0: *mut ElementType,
    mut attId: *mut AttributeId,
    mut isCdata: XML_Bool,
    mut isId: XML_Bool,
    mut value: *const XML_Char,
) -> c_int {
    if !value.is_null() || isId  {
        /* The handling of default attributes gets messed up if we have
        a default which duplicates a non-default. */
        /* save one level of indirection */

        if (*type_0).defaultAtts.iter().any(|da| attId == da.id) {
            return 1;
        }
        if isId && (*type_0).idAtt.is_null() && !(*attId).xmlns {
            (*type_0).idAtt = attId
        }
    }

    if (*type_0).defaultAtts.try_reserve(1).is_ok() {
        if !isCdata {
            (*attId).maybeTokenized = true
        }

        let att = DefaultAttribute { id: attId, isCdata, value };
        (*type_0).defaultAtts.push(att);
        1
    } else {
        0
    }
}

impl<'scf> XML_ParserStruct<'scf> {
    unsafe fn setElementTypePrefix(
        &mut self,
        mut elementType: *mut ElementType,
    ) -> c_int {
        let dtd: *mut DTD = self.m_dtd;
        let mut name = (*elementType).name;
        while *name != 0 {
            if *name == ASCII_COLON as XML_Char {
                let mut s: *const XML_Char = ptr::null();
                s = (*elementType).name;
                while s != name {
                    if !(*dtd).pool.append_char(*s) {
                        return 0;
                    }
                    s = s.offset(1)
                }
                if !(*dtd).pool.append_char('\u{0}' as XML_Char) {
                    return 0;
                }
                // This is unsafe, start needs be very temporary
                let start = (*dtd).pool.current_start();
                let prefix = hash_insert!(
                    &mut (*dtd).prefixes,
                    start,
                    Prefix
                );
                if prefix.is_null() {
                    return 0;
                }
                if (*prefix).name == start {
                    (*dtd).pool.finish_string();
                } else {
                    (*dtd).pool.clear_current();
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
        mut buf: ExpatBufRef,
    ) -> *mut AttributeId {
        let dtd: *mut DTD = self.m_dtd;
        if !(*dtd).pool.append_char(AttributeType::Unset.into()) {
            return ptr::null_mut();
        }
        let enc = self.encoding(enc_type);
        if !(*dtd).pool.store_c_string(enc, buf) {
            return ptr::null_mut();
        }
        // let mut name = &mut *name;
        /* skip quotation mark - its storage will be re-used (like in name[-1]) */
        let id = (*dtd).pool.current_mut_slice(|name| {
            let typed_name = TypedAttributeName(name.as_mut_ptr() as *mut XML_Char);
            hash_insert!(
                &mut (*dtd).attributeIds,
                typed_name,
                AttributeId
            )
        });
        if id.is_null() {
            return ptr::null_mut();
        }
        if (*id).name.name() != (*dtd).pool.current_start().add(1) as *mut XML_Char {
            (*dtd).pool.clear_current();
        } else {
            let name = (*dtd).pool.finish_string().split_at(1).1;
            if self.m_ns {
                if name[0] == ASCII_x as XML_Char
                && name[1] == ASCII_m as XML_Char
                && name[2] == ASCII_l as XML_Char
                && name[3] == ASCII_n as XML_Char
                && name[4] == ASCII_s as XML_Char
                && (name[5] == '\u{0}' as XML_Char
                || name[5] == ASCII_COLON as XML_Char)
                {
                    if name[5] == '\u{0}' as XML_Char {
                        (*id).prefix = &mut (*dtd).defaultPrefix
                    } else {
                        (*id).prefix = hash_insert!(
                            &mut (*dtd).prefixes,
                            &name[6] as *const _,
                            Prefix
                        );
                    }
                    (*id).xmlns = true
                } else {
                    let mut i: c_int = 0;
                    i = 0;
                    while name[usize::try_from(i).unwrap()] != 0 {
                        /* attributes without prefix are *not* in the default namespace */
                        if name[usize::try_from(i).unwrap()] == ASCII_COLON as XML_Char {
                            let mut j: c_int = 0; /* save one level of indirection */
                            j = 0;
                            while j < i {
                                if !(*dtd).pool.append_char(name[usize::try_from(j).unwrap()]) {
                                    return ptr::null_mut();
                                }
                                j += 1
                            }
                            if !(*dtd).pool.append_char('\u{0}' as XML_Char) {
                                return ptr::null_mut();
                            }
                            let tempName = (*dtd).pool.current_start();
                            (*id).prefix = hash_insert!(
                                &mut (*dtd).prefixes,
                                tempName,
                                Prefix
                            );
                            if (*id).prefix.is_null() {
                                return ptr::null_mut();
                            }
                            if (*(*id).prefix).name == tempName {
                                (*dtd).pool.finish_string();
                            } else {
                                (*dtd).pool.clear_current();
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

impl<'scf> XML_ParserStruct<'scf> {
    unsafe fn getContext(&mut self) -> bool {
        let dtd: *mut DTD = self.m_dtd;
        let mut needSep = false;
        if !(*dtd).defaultPrefix.binding.is_null() {
            let mut i: c_int = 0;
            let mut len: c_int = 0;
            if !self.m_tempPool.append_char(ASCII_EQUALS as XML_Char) {
                return false;
            }
            len = (*(*dtd).defaultPrefix.binding).uriLen;
            if self.m_namespaceSeparator != 0 {
                len -= 1
            }
            i = 0;
            while i < len {
                if !self.m_tempPool.append_char(*(*(*dtd).defaultPrefix.binding).uri.offset(i as isize)) {
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
                    return false;
                    /* LCOV_EXCL_LINE */
                }
                i += 1
            }
            needSep = true
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
            let mut s: *const XML_Char = ptr::null();
            if !(*prefix).binding.is_null() {
                if needSep && !self.m_tempPool.append_char(CONTEXT_SEP) {
                    return false;
                }
                s = (*prefix).name;
                while *s != 0 {
                    if !self.m_tempPool.append_char(*s) {
                        return false;
                    }
                    s = s.offset(1)
                }
                if !self.m_tempPool.append_char(ASCII_EQUALS as XML_Char) {
                    return false;
                }
                len_0 = (*(*prefix).binding).uriLen;
                if self.m_namespaceSeparator != 0 {
                    len_0 -= 1
                }
                i_0 = 0;
                while i_0 < len_0 {
                    if !self.m_tempPool.append_char(*(*(*prefix).binding).uri.offset(i_0 as isize)) {
                        return false;
                    }
                    i_0 += 1
                }
                needSep = true
            }
        }
        for e in (*dtd).generalEntities.values() {
            if !(*e).open {
                continue;
            }
            if needSep && !self.m_tempPool.append_char(CONTEXT_SEP) {
                return false;
            }
            // TODO: Could the following be replaced by m_tempPool.append_c_string((*e).name)?
            let mut s_0 = (*e).name;
            while *s_0 != 0 {
                if !self.m_tempPool.append_char(*s_0) {
                    return false;
                }
                s_0 = s_0.offset(1)
            }
            needSep = true
        }
        if !self.m_tempPool.append_char('\u{0}' as XML_Char) {
            return false;
        }

        true
    }
}

impl<'scf> XML_ParserStruct<'scf> {
    unsafe fn setContext(&mut self, mut context: *const XML_Char) -> XML_Bool {
        let dtd: *mut DTD = self.m_dtd;
        let mut s: *const XML_Char = context;
        while *context != '\u{0}' as XML_Char {
            if *s == CONTEXT_SEP || *s == '\u{0}' as XML_Char {
                if !self.m_tempPool.append_char('\u{0}' as XML_Char) {
                    return false;
                }
                self.m_tempPool.current_slice(|entity_name| {
                    if let Some(e) = (*dtd).generalEntities.get_mut(&HashKey::from(entity_name.as_ptr() as KEY)) {
                        e.open = true
                    }
                    if *s != '\u{0}' as XML_Char {
                        s = s.offset(1)
                    }
                    context = s;
                });
                self.m_tempPool.clear_current();
            } else if *s == ASCII_EQUALS as XML_Char {
                let mut prefix: *mut Prefix;
                if self.m_tempPool.is_empty() {
                    prefix = &mut (*dtd).defaultPrefix;
                } else {
                    if !self.m_tempPool.append_char('\u{0}' as XML_Char) {
                        return false;
                    }
                    prefix = self.m_tempPool.current_slice(|prefix_name| {
                        let mut prefix = hash_lookup!(
                            &mut (*dtd).prefixes,
                            prefix_name.as_ptr()
                        );
                        if prefix.is_null() {
                            // librexpat change: we need to copy the prefix name
                            // into the DTD pool, since the HashMap keeps a permanent
                            // reference to the name which we can't modify after
                            // the call to `hash_insert!` (unlike the original C code)
                            let prefix_name = match (*dtd).pool.copy_c_string(prefix_name.as_ptr()) {
                                Some(name) => name,
                                None => return prefix,
                            };
                            prefix = hash_insert!(
                                &mut (*dtd).prefixes,
                                prefix_name.as_ptr(),
                                Prefix
                            );
                        }
                        prefix
                    });
                    if prefix.is_null() { return false; }
                    self.m_tempPool.clear_current();
                }
                context = s.offset(1);
                while *context != CONTEXT_SEP && *context != '\u{0}' as XML_Char {
                    if !self.m_tempPool.append_char(*context) {
                        return false;
                    }
                    context = context.offset(1)
                }
                if !self.m_tempPool.append_char('\u{0}' as XML_Char) {
                    return false;
                }
                if addBinding(
                    self,
                    prefix,
                    ptr::null_mut(),
                    self.m_tempPool.current_start(),
                    &mut self.m_inheritedBindings,
                ) != XML_Error::NONE {
                    return false;
                }
                self.m_tempPool.clear_current();
                if *context != '\u{0}' as XML_Char {
                    context = context.offset(1)
                }
                s = context
            } else {
                if !self.m_tempPool.append_char(*s) {
                    return false;
                }
                s = s.offset(1)
            }
        }
        true
    }
}

// TODO: Good candidate for safeification
unsafe extern "C" fn normalizePublicId(mut publicId: *mut XML_Char) {
    let mut p: *mut XML_Char = publicId;
    let mut s: *mut XML_Char = ptr::null_mut();
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


unsafe extern "C" fn dtdCreate<'scf>() -> *mut DTD<'scf> {
    // Fail if pools fail to allocate
    let pool1 = match StringPool::try_new() {
        Ok(pool) => pool,
        Err(()) => return ptr::null_mut(),
    };
    let pool2 = match StringPool::try_new() {
        Ok(pool) => pool,
        Err(()) => return ptr::null_mut(),
    };

    let mut p: *mut DTD = MALLOC!(@DTD);
    if p.is_null() {
        return p;
    }
    // FIXME: we're writing over uninitialized memory, use `MaybeUninit`???
    std::ptr::write(&mut (*p).pool, pool1);
    std::ptr::write(&mut (*p).entityValuePool, pool2);
    std::ptr::write(&mut (*p).generalEntities, Default::default());
    std::ptr::write(&mut (*p).elementTypes, Default::default());
    std::ptr::write(&mut (*p).attributeIds, Default::default());
    std::ptr::write(&mut (*p).prefixes, Default::default());
    (*p).paramEntityRead = false;
    std::ptr::write(&mut (*p).paramEntities, Default::default());
    /* XML_DTD */
    (*p).defaultPrefix.name = ptr::null();
    (*p).defaultPrefix.binding = ptr::null_mut();
    (*p).in_eldecl = false;
    std::ptr::write(&mut (*p).scaffold, Default::default());
    (*p).contentStringLen = 0;
    (*p).keepProcessing = true;
    (*p).hasParamEntityRefs = false;
    (*p).standalone = false;
    p
}
/* do not call if m_parentParser != NULL */

unsafe extern "C" fn dtdReset(mut p: *mut DTD) {
    (*p).generalEntities.clear();
    (*p).paramEntityRead = false;
    (*p).paramEntities.clear();
    /* XML_DTD */
    (*p).elementTypes.clear();
    (*p).attributeIds.clear();
    (*p).prefixes.clear();
    (*p).pool.clear();
    (*p).entityValuePool.clear();
    (*p).defaultPrefix.name = ptr::null();
    (*p).defaultPrefix.binding = ptr::null_mut();
    (*p).in_eldecl = false;
    (*p).scaffold = Default::default();
    (*p).contentStringLen = 0;
    (*p).keepProcessing = true;
    (*p).hasParamEntityRefs = false;
    (*p).standalone = false;
}

unsafe extern "C" fn dtdDestroy(
    mut p: *mut DTD,
) {
    std::ptr::drop_in_place(&mut (*p).pool);
    std::ptr::drop_in_place(&mut (*p).entityValuePool);
    std::ptr::drop_in_place(&mut (*p).generalEntities);
    std::ptr::drop_in_place(&mut (*p).paramEntities);
    /* XML_DTD */
    std::ptr::drop_in_place(&mut (*p).elementTypes);
    std::ptr::drop_in_place(&mut (*p).attributeIds);
    std::ptr::drop_in_place(&mut (*p).prefixes);
    std::ptr::drop_in_place(&mut (*p).scaffold);
    FREE!(p);
}
/* Do a deep copy of the DTD. Return 0 for out of memory, non-zero otherwise.
   The new DTD has already been initialized.
*/

unsafe extern "C" fn dtdCopy<'scf>(
    mut newDtd: *mut DTD<'scf>,
    mut oldDtd: *const DTD<'scf>,
) -> c_int {
    /* Copy the prefix table. */
    for oldP in (*oldDtd).prefixes.values() {
        let mut name = match (*newDtd).pool.copy_c_string((*oldP).name) {
            Some(name) => name,
            None => return 0,
        };
        if hash_insert!(
            &mut (*newDtd).prefixes,
            name.as_ptr(),
            Prefix
        )
        .is_null()
        {
            return 0;
        }
    }
    for oldA in (*oldDtd).attributeIds.values()
    /* Copy the attribute id table. */
    {
        /* Remember to allocate the scratch byte before the name. */
        if !(*newDtd).pool.append_char(AttributeType::Unset.into()) {
            return 0;
        }
        // FIXME: should be copy_c_string_cells
        let mut name_0 = match (*newDtd).pool.copy_c_string((*oldA).name.name()) {
            Some(name) => name,
            None => return 0,
        };
        let typed_name = TypedAttributeName(name_0.as_ptr() as *mut XML_Char);
        let newA = hash_insert!(
            &mut (*newDtd).attributeIds,
            typed_name,
            AttributeId
        );
        if newA.is_null() {
            return 0;
        }
        (*newA).maybeTokenized = (*oldA).maybeTokenized;
        if !(*oldA).prefix.is_null() {
            (*newA).xmlns = (*oldA).xmlns;
            if (*oldA).prefix == &(*oldDtd).defaultPrefix as *const Prefix as *mut Prefix {
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
        let mut name_1 = match (*newDtd).pool.copy_c_string((*oldE).name) {
            Some(name) => name,
            None => return 0,
        };
        let newE = hash_insert!(
            &mut (*newDtd).elementTypes,
            name_1.as_ptr(),
            ElementType,
            ElementType::new
        );
        if newE.is_null() {
            return 0;
        }
        if !(*oldE).idAtt.is_null() {
            (*newE).idAtt = hash_lookup!(
                (*newDtd).attributeIds,
                (*(*oldE).idAtt).name
            );
        }
        if !(*oldE).prefix.is_null() {
            (*newE).prefix = hash_lookup!(
                (*newDtd).prefixes,
                (*(*oldE).prefix).name
            );
        }

        if (*newE).defaultAtts.try_reserve((*oldE).defaultAtts.len()).is_err() {
            return 0;
        }
        for oldAtt in &(*oldE).defaultAtts {
            let id = hash_lookup!((*newDtd).attributeIds, (*oldAtt.id).name);
            let isCdata = oldAtt.isCdata;
            let value = if !oldAtt.value.is_null() {
                match (*newDtd).pool.copy_c_string(oldAtt.value) {
                    Some(s) => s.as_ptr(),
                    None => return 0,
                }
            } else {
                ptr::null()
            };
            let newAtt = DefaultAttribute { id, isCdata, value };
            (*newE).defaultAtts.push(newAtt);
        }
    }
    /* Copy the entity tables. */
    if copyEntityTable(
        &mut (*newDtd).generalEntities,
        &(*newDtd).pool,
        &(*oldDtd).generalEntities,
    ) == 0
    {
        return 0;
    }
    if copyEntityTable(
        &mut (*newDtd).paramEntities,
        &(*newDtd).pool,
        &(*oldDtd).paramEntities,
    ) == 0
    {
        return 0;
    }
    (*newDtd).paramEntityRead = (*oldDtd).paramEntityRead;
    /* XML_DTD */
    (*newDtd).keepProcessing = (*oldDtd).keepProcessing;
    (*newDtd).hasParamEntityRefs = (*oldDtd).hasParamEntityRefs;
    (*newDtd).standalone = (*oldDtd).standalone;
    /* Don't want deep copying for scaffolding */
    (*newDtd).in_eldecl = (*oldDtd).in_eldecl;
    (*newDtd).scaffold = Cow::Borrowed(&(*oldDtd).scaffold);
    (*newDtd).contentStringLen = (*oldDtd).contentStringLen;
    1
}
/* End dtdCopy */

unsafe extern "C" fn copyEntityTable(
    mut newTable: &mut HashMap<HashKey, Box<Entity>>,
    mut newPool: &StringPool,
    mut oldTable: &HashMap<HashKey, Box<Entity>>,
) -> c_int {
    let mut cachedOldBase: *const XML_Char = ptr::null();
    let mut cachedNewBase: *const XML_Char = ptr::null();
    for oldE in oldTable.values() {
        let mut name = match (*newPool).copy_c_string((*oldE).name) {
            Some(name) => name,
            None => return 0,
        };
        let newE = hash_insert!(
            &mut newTable,
            name.as_ptr(),
            Entity
        );
        if newE.is_null() {
            return 0;
        }
        if !(*oldE).systemId.is_null() {
            let mut tem = match (*newPool).copy_c_string((*oldE).systemId) {
                Some(tem) => tem,
                None => return 0,
            };
            (*newE).systemId = tem.as_ptr();
            if !(*oldE).base.is_null() {
                if (*oldE).base == cachedOldBase {
                    (*newE).base = cachedNewBase
                } else {
                    cachedOldBase = (*oldE).base;
                    tem = match (*newPool).copy_c_string(cachedOldBase) {
                        Some(tem) => tem,
                        None => return 0,
                    };
                    (*newE).base = tem.as_ptr();
                    cachedNewBase = (*newE).base
                }
            }
            if !(*oldE).publicId.is_null() {
                tem = match (*newPool).copy_c_string((*oldE).publicId) {
                    Some(tem) => tem,
                    None => return 0,
                };
                (*newE).publicId = tem.as_ptr();
            }
        } else {
            let mut tem_0 = match (*newPool).copy_c_string_n((*oldE).textPtr, (*oldE).textLen) {
                Some(tem) => tem,
                None => return 0,
            };
            (*newE).textPtr = tem_0.as_ptr();
            (*newE).textLen = (*oldE).textLen
        }
        if !(*oldE).notation.is_null() {
            let mut tem_1 = match (*newPool).copy_c_string((*oldE).notation) {
                Some(tem) => tem,
                None => return 0,
            };
            (*newE).notation = tem_1.as_ptr();
        }
        (*newE).is_param = (*oldE).is_param;
        (*newE).is_internal = (*oldE).is_internal
    }
    1
}

pub const INIT_POWER: c_int = 6;

unsafe extern "C" fn keyeq(mut s1: KEY, mut s2: KEY) -> XML_Bool {
    while *s1 as c_int == *s2 as c_int {
        if *s1 as c_int == 0 {
            return true;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1)
    }
    false
}

unsafe extern "C" fn keylen(mut s: KEY) -> size_t {
    let mut len: size_t = 0;
    while *s != 0 {
        s = s.offset(1);
        len = len.wrapping_add(1)
    }
    len
}


impl STRING_POOL {
    unsafe fn init(
        &mut self,
    ) {
        self.blocks = ptr::null_mut();
        self.freeBlocks = ptr::null_mut();
        self.start = ptr::null_mut();
        self.ptr = ptr::null_mut();
        self.end = ptr::null();
    }

    unsafe fn clear(&mut self) {
        if self.freeBlocks.is_null() {
            self.freeBlocks = self.blocks
        } else {
            let mut p: *mut Block = self.blocks;
            while !p.is_null() {
                let mut tem: *mut Block = (*p).next;
                (*p).next = self.freeBlocks;
                self.freeBlocks = p;
                p = tem
            }
        }
        self.blocks = ptr::null_mut();
        self.start = ptr::null_mut();
        self.ptr = ptr::null_mut();
        self.end = ptr::null();
    }

    unsafe fn destroy(&mut self) {
        let mut p: *mut Block = self.blocks;
        while !p.is_null() {
            let mut tem: *mut Block = (*p).next;
            FREE!(p);
            p = tem
        }
        p = self.freeBlocks;
        while !p.is_null() {
            let mut tem_0: *mut Block = (*p).next;
            FREE!(p);
            p = tem_0
        }
    }

    unsafe fn append(
        &mut self,
        enc: &ENCODING,
        mut buf: ExpatBufRef,
    ) -> bool {
        if self.ptr.is_null() && !self.grow() {
            return false;
        }
        loop {
            let mut pool_buf = ExpatBufRefMut::new(
                self.ptr as *mut ICHAR,
                self.end as *mut ICHAR,
            );
            let convert_res: super::xmltok::XML_Convert_Result = XmlConvert!(
                enc,
                &mut buf,
                &mut pool_buf
            );
            self.ptr = pool_buf.as_mut_ptr();
            if convert_res == super::xmltok::XML_Convert_Result::COMPLETED
                || convert_res == super::xmltok::XML_Convert_Result::INPUT_INCOMPLETE
            {
                break;
            }
            if !self.grow() {
                return false;
            }
        }
        !self.start.is_null()
    }

    unsafe fn copy_c_string(
        &mut self,
        mut s: *const XML_Char,
    ) -> *const XML_Char {
        loop {
            if if self.ptr == self.end as *mut XML_Char && !self.grow() {
                0
            } else {
                let fresh77 = self.ptr;
                self.ptr = self.ptr.offset(1);
                *fresh77 = *s;
                1
            } == 0
            {
                return ptr::null();
            }
            let fresh78 = s;
            s = s.offset(1);
            if !(*fresh78 != 0) {
                break;
            }
        }
        s = self.start;
        self.start = self.ptr;
        s
    }

    unsafe fn copy_c_string_n(
        &mut self,
        mut s: *const XML_Char,
        mut n: c_int,
    ) -> *const XML_Char {
        if self.ptr.is_null() && !self.grow() {
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
            return ptr::null();
            /* LCOV_EXCL_LINE */
        }
        while n > 0 {
            if if self.ptr == self.end as *mut XML_Char && !self.grow() {
                0
            } else {
                let fresh79 = self.ptr;
                self.ptr = self.ptr.offset(1);
                *fresh79 = *s;
                1
            } == 0
            {
                return ptr::null();
            }
            n -= 1;
            s = s.offset(1)
        }
        s = self.start;
        self.start = self.ptr;
        s
    }

    unsafe fn append_c_string(&mut self, mut s: *const XML_Char) -> bool {
        while *s != 0 {
            if !self.append_char(*s) {
                return false;
            }
            s = s.offset(1)
        }
        !self.start.is_null()
    }

    unsafe fn store_c_string(&mut self, enc: &ENCODING, buf: ExpatBufRef) -> *mut XML_Char {
        if !self.append(enc, buf) {
            return ptr::null_mut();
        }
        if self.ptr == self.end as *mut XML_Char && !self.grow() {
            return ptr::null_mut();
        }
        let fresh81 = self.ptr;
        self.ptr = self.ptr.offset(1);
        *fresh81 = 0;
        self.start
    }
}

unsafe extern "C" fn poolBytesToAllocateFor(mut blockSize: c_int) -> size_t {
    /* Unprotected math would be:
     ** return offsetof(Block, s) + blockSize * sizeof(XML_Char);
     **
     ** Detect overflow, avoiding _signed_ overflow undefined behavior
     ** For a + b * c we check b * c in isolation first, so that addition of a
     ** on top has no chance of making us accept a small non-negative number
     */
    let stretch = ::std::mem::size_of::<XML_Char>() as u64; /* can be 4 bytes */
    if blockSize <= 0 {
        return 0;
    }
    if blockSize > (INT_MAX as c_ulong).wrapping_div(stretch) as c_int {
        return 0;
    }
    let stretchedBlockSize: c_int = blockSize * stretch as c_int;
    let bytesToAllocate: c_int =
        (12 as c_ulong).wrapping_add(stretchedBlockSize as c_uint as c_ulong) as c_int;
    if bytesToAllocate < 0 {
        return 0;
    }
    bytesToAllocate as size_t
}

impl STRING_POOL {
    unsafe fn grow(&mut self) -> XML_Bool {
        if !self.freeBlocks.is_null() {
            if self.start.is_null() {
                self.blocks = self.freeBlocks;
                self.freeBlocks = (*self.freeBlocks).next;
                (*self.blocks).next = ptr::null_mut();
                self.start = (*self.blocks).s.as_mut_ptr();
                self.end = self.start.offset((*self.blocks).size as isize);
                self.ptr = self.start;
                return true;
            }
            if (self.end.wrapping_offset_from(self.start) as c_long)
                < (*self.freeBlocks).size as c_long
            {
                let mut tem: *mut Block = (*self.freeBlocks).next;
                (*self.freeBlocks).next = self.blocks;
                self.blocks = self.freeBlocks;
                self.freeBlocks = tem;
                memcpy(
                    (*self.blocks).s.as_mut_ptr() as *mut c_void,
                    self.start as *const c_void,
                    (self.end.wrapping_offset_from(self.start) as usize)
                        .wrapping_mul(::std::mem::size_of::<XML_Char>()),
                );
                self.ptr = (*self.blocks)
                    .s
                    .as_mut_ptr()
                    .offset(self.ptr.wrapping_offset_from(self.start));
                self.start = (*self.blocks).s.as_mut_ptr();
                self.end = self.start.offset((*self.blocks).size as isize);
                return true;
            }
        }
        if !self.blocks.is_null() && self.start == (*self.blocks).s.as_mut_ptr() {
            let mut temp: *mut Block = ptr::null_mut();
            let mut blockSize: c_int =
                (self.end.wrapping_offset_from(self.start) as c_uint).wrapping_mul(2) as c_int;
            let mut bytesToAllocate: size_t = 0;
            /* NOTE: Needs to be calculated prior to calling `realloc`
            to avoid dangling pointers: */
            let offsetInsideBlock: ptrdiff_t =
                self.ptr.wrapping_offset_from(self.start);
            if blockSize < 0 {
                /* This condition traps a situation where either more than
                 * INT_MAX/2 bytes have already been allocated.  This isn't
                 * readily testable, since it is unlikely that an average
                 * machine will have that much memory, so we exclude it from the
                 * coverage statistics.
                 */
                return false;
                /* LCOV_EXCL_LINE */
            }
            bytesToAllocate = poolBytesToAllocateFor(blockSize);
            if bytesToAllocate == 0 {
                return false;
            }
            temp = REALLOC!(self.blocks, bytesToAllocate) as *mut Block;
            if temp.is_null() {
                return false;
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
            let mut tem_0: *mut Block = ptr::null_mut();
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
                return false;
                /* LCOV_EXCL_LINE */
            }
            if blockSize_0 < INIT_BLOCK_SIZE {
                blockSize_0 = INIT_BLOCK_SIZE
            } else {
                /* Detect overflow, avoiding _signed_ overflow undefined behavior */
                if ((blockSize_0 as c_uint).wrapping_mul(2) as c_int) < 0 {
                    return false;
                } /* save one level of indirection */
                blockSize_0 *= 2
            } /* save one level of indirection */
            bytesToAllocate_0 = poolBytesToAllocateFor(blockSize_0); /* save one level of indirection */
            if bytesToAllocate_0 == 0 {
                return false;
            } /* save one level of indirection */
            tem_0 = MALLOC!(bytesToAllocate_0) as *mut Block;
            if tem_0.is_null() {
                return false;
            }
            (*tem_0).size = blockSize_0;
            (*tem_0).next = self.blocks;
            self.blocks = tem_0;
            if self.ptr != self.start {
                memcpy(
                    (*tem_0).s.as_mut_ptr() as *mut c_void,
                    self.start as *const c_void,
                    (self.ptr.wrapping_offset_from(self.start) as usize)
                        .wrapping_mul(::std::mem::size_of::<XML_Char>()),
                );
            }
            self.ptr = (*tem_0)
                .s
                .as_mut_ptr()
                .offset(self.ptr.wrapping_offset_from(self.start));
            self.start = (*tem_0).s.as_mut_ptr();
            self.end = (*tem_0).s.as_mut_ptr().offset(blockSize_0 as isize)
        }
        true
    }
}

impl<'scf> XML_ParserStruct<'scf> {
    fn build_node<'a, 'b>(
        &mut self,
        src_node: usize,
        dest: &mut XML_Content,
        mut contpos: &'a mut [XML_Content],
        mut strpos: &'b mut [XML_Char],
    ) -> (&'a mut [XML_Content], &'b mut [XML_Char]) {
        let dtd: *mut DTD = self.m_dtd;
        let scf = RefCell::borrow(unsafe { &(*dtd).scaffold });
        dest.type_0 = scf.scaffold[src_node].type_0;
        dest.quant = scf.scaffold[src_node].quant;
        if dest.type_0 == XML_Content_Type::NAME {
            dest.name = (*strpos).as_ptr() as *mut XML_Char;
            dest.numchildren = 0;
            dest.children = ptr::null_mut();

            let mut src = scf.scaffold[src_node].name;
            loop {
                let (first, rest) = strpos.split_first_mut().unwrap();
                *first = unsafe { *src };
                if *first == 0 {
                    return (contpos, rest);
                }
                unsafe { src = src.offset(1) };
                strpos = rest;
            }
        } else {
            let (children, rest) = contpos.split_at_mut(scf.scaffold[src_node].childcnt);
            contpos = rest;

            dest.name = ptr::null_mut();
            dest.numchildren = children.len().try_into().unwrap();
            dest.children = children.as_ptr() as *mut XML_Content;

            let mut cn = scf.scaffold[src_node].firstchild;
            for child in children {
                let (ncp, nsp) = self.build_node(cn, child, contpos, strpos);
                cn = scf.scaffold[cn].nextsib;
                contpos = ncp;
                strpos = nsp;
            }
        }
        (contpos, strpos)
    }

    unsafe fn build_model(&mut self) -> *mut XML_Content {
        let dtd: *mut DTD = self.m_dtd;
        let mut ret: *mut XML_Content = ptr::null_mut();
        let scaffold_len = (*dtd).scaffold.borrow().scaffold.len();
        let mut allocsize: c_int = (scaffold_len as c_ulong)
            .wrapping_mul(::std::mem::size_of::<XML_Content>() as c_ulong)
            .wrapping_add(
                ((*dtd).contentStringLen as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
            ) as c_int;
        ret = MALLOC!(allocsize as size_t) as *mut XML_Content;
        if ret.is_null() {
            return ptr::null_mut();
        }
        let str = std::slice::from_raw_parts_mut(
            ret.add(scaffold_len) as *mut XML_Char,
            (*dtd).contentStringLen as usize);
        let cpos = std::slice::from_raw_parts_mut(
            ret.offset(1) as *mut XML_Content,
            scaffold_len as usize - 1);
        self.build_node(0, &mut *ret, cpos, str);
        ret
    }

    unsafe fn getElementType(
        &mut self,
        mut enc_type: EncodingType,
        mut buf: ExpatBufRef,
    ) -> *mut ElementType {
        let dtd: *mut DTD = self.m_dtd;
        let enc = self.encoding(enc_type);
        if !(*dtd).pool.store_c_string(enc, buf) {
            return ptr::null_mut();
        }
        let ret = (*dtd).pool.current_slice(|name| hash_insert!(
            &mut (*dtd).elementTypes,
            name.as_ptr(),
            ElementType,
            ElementType::new
        ));
        if ret.is_null() {
            return ptr::null_mut();
        }
        if (*ret).name != (*dtd).pool.current_start() {
            (*dtd).pool.clear_current();
        } else {
            (*dtd).pool.finish_string();
            if self.setElementTypePrefix(ret) == 0 {
                return ptr::null_mut();
            }
        }
        ret
    }
}

unsafe extern "C" fn copy_c_string(
    mut s: *const XML_Char,
) -> *mut XML_Char {
    let mut charsRequired = 0isize;
    /* First determine how long the string is */
    while *s.offset(charsRequired) != 0 {
        charsRequired += 1
    }
    /* Include the terminator */
    charsRequired += 1;
    /* Now allocate space for the copy */
    let result = MALLOC![XML_Char; charsRequired];
    if result.is_null() {
        return ptr::null_mut();
    }
    /* Copy the original into place */
    let n: size_t = (charsRequired as size_t)
        .checked_mul(::std::mem::size_of::<XML_Char>())
        .unwrap();
    memcpy(result as *mut c_void, s as *const c_void, n);
    result
}
