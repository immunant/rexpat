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
    XML_EndNamespaceDeclHandler, XML_EntityDeclHandler, XML_Error, XML_ErrorCode,
    XML_Expat_Version, XML_ExternalEntityRefHandler, XML_Feature, XML_FeatureEnum,
    XML_Memory_Handling_Suite, XML_NotStandaloneHandler, XML_NotationDeclHandler,
    XML_ParamEntityParsing, XML_Parser, XML_Parsing, XML_ParsingStatus,
    XML_ProcessingInstructionHandler, XML_SkippedEntityHandler, XML_StartCdataSectionHandler,
    XML_StartDoctypeDeclHandler, XML_StartElementHandler, XML_StartNamespaceDeclHandler,
    XML_Status, XML_UnknownEncodingHandler, XML_UnparsedEntityDeclHandler, XML_XmlDeclHandler,
    XML_cp,
};
use crate::fallible_rc::FallibleRc;
pub use crate::lib::xmlrole::{
    prolog_state, XmlPrologStateInit, XmlPrologStateInitExternalEntity, PROLOG_STATE, XML_ROLE,
};
pub use crate::lib::xmltok::*;
use crate::string_pool::{StringPool, StringPoolCellSlice, StringPoolSlice};
pub use libc::INT_MAX;
use libc::{
    c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void, memcmp, memcpy, memset,
    size_t,
};
use num_traits::{FromPrimitive, ToPrimitive};

use fallible_collections::FallibleBox;
use owning_ref::{OwningRef, RcRef, Erased};

use std::alloc::{self, Layout};
use std::cell::{Cell, RefCell};
use std::cmp;
use std::collections::{hash_map, HashMap, HashSet, TryReserveError};
use std::convert::{TryInto, TryFrom};
use std::mem;
use std::ops::{self, DerefMut};
use std::ptr;
use std::rc::{Rc, Weak};
use std::slice;

type Ptr<T> = Option<Rc<T>>;

pub const XML_CONTEXT_BYTES: c_int = 1024;

#[derive(Copy, Clone, Debug)]
pub struct ExpatBufRef<'a, T = c_char>(&'a [T]);
impl<'a, T> ExpatBufRef<'a, T> {
    pub fn new<'new>(start: *const T, end: *const T) -> ExpatBufRef<'new, T> {
        unsafe {
            ExpatBufRef(std::slice::from_raw_parts(
                start,
                end.wrapping_offset_from(start) as usize,
            ))
        }
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
            panic!(
                "Attempted to move the start of an ExpatBufRef to an invalid pointer: {:?}",
                new_start
            );
        }
        ExpatBufRef::new(new_start, self.end())
    }

    pub fn with_end(&self, new_end: *const T) -> ExpatBufRef<'a, T> {
        if new_end < self.as_ptr() || new_end > self.end() {
            panic!(
                "Attempted to move the end of an ExpatBufRef to an invalid pointer: {:?}",
                new_end
            );
        }
        ExpatBufRef::new(self.0.as_ptr(), new_end)
    }

    pub fn with_len(&self, len: usize) -> ExpatBufRef<'a, T> {
        if len > self.len() {
            panic!("Attempted to expand an ExpatBufRef to lenth: {:?}", len);
        }
        ExpatBufRef(&self.0[..len])
    }

    pub fn inc_start(&self, offset: usize) -> ExpatBufRef<'a, T> {
        if offset > self.len() {
            panic!(
                "Attempted to increment the start of an ExpatBufRef by too much: {:?}",
                offset
            );
        }
        ExpatBufRef(&self.0[offset..])
    }

    pub fn dec_end(&self, offset: usize) -> ExpatBufRef<'a, T> {
        if offset > self.len() {
            panic!(
                "Attempted to decrement the end of an ExpatBufRef by too much: {:?}",
                offset
            );
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
        unsafe {
            ExpatBufRefMut(std::slice::from_raw_parts_mut(
                start,
                end.wrapping_offset_from(start) as usize,
            ))
        }
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
            panic!(
                "Attempted to move the start of an ExpatBufRefMut to an invalid pointer: {:?}",
                new_start
            );
        }
        let offset = new_start.wrapping_offset_from(self.0.as_ptr());
        let new_start = unsafe { self.0.as_mut_ptr().offset(offset) };
        *self = ExpatBufRefMut::new(new_start, self.end());
    }

    pub fn with_end(&mut self, new_end: *mut T) {
        if new_end < self.as_mut_ptr() || new_end > self.end() {
            panic!(
                "Attempted to move the end of an ExpatBufRefMut to an invalid pointer: {:?}",
                new_end
            );
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
            panic!(
                "Attempted to increment the start of an ExpatBufRef by too much: {:?}",
                offset
            );
        }
        let new_start = unsafe { self.0.as_mut_ptr().offset(offset.try_into().unwrap()) };
        *self = ExpatBufRefMut::new(new_start, self.end());
    }

    pub fn dec_end(&mut self, offset: usize) {
        if offset > self.len() {
            panic!(
                "Attempted to decrement the end of an ExpatBufRefMut by too much: {:?}",
                offset
            );
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

#[derive(Clone, Debug)]
pub struct ExpatSliceRc<T = c_char>(OwningRef<Rc<dyn Erased>, [T]>);
impl<T> ExpatSliceRc<T> {
    pub fn end(&self) -> *const T {
        self.0[self.0.len()..].as_ptr()
    }

    // TODO: get rid of this
    pub fn with_start(self, new_start: *const T) -> ExpatSliceRc<T> {
        if new_start < self.as_ptr() || new_start > self.end() {
            panic!(
                "Attempted to move the start of an ExpatSliceRc to an invalid pointer: {:?}",
                new_start
            );
        }
        let offset = new_start.wrapping_offset_from(self.as_ptr()) as usize;
        self.inc_start(offset)
    }

    // TODO: get rid of this
    pub fn with_end(self, new_end: *const T) -> ExpatSliceRc<T> {
        if new_end < self.as_ptr() || new_end > self.end() {
            panic!(
                "Attempted to move the end of an ExpatSliceRc to an invalid pointer: {:?}",
                new_end
            );
        }
        let new_len = new_end.wrapping_offset_from(self.as_ptr()) as usize;
        self.with_len(new_len)
    }

    pub fn with_len(self, len: usize) -> ExpatSliceRc<T> {
        if len > self.len() {
            panic!("Attempted to expand an ExpatSliceRc to lenth: {:?}", len);
        }
        ExpatSliceRc(self.0.map(|sl| &sl[..len]))
    }

    pub fn inc_start(self, offset: usize) -> ExpatSliceRc<T> {
        if offset > self.len() {
            panic!(
                "Attempted to increment the start of an ExpatSliceRc by too much: {:?}",
                offset
            );
        }
        ExpatSliceRc(self.0.map(|sl| &sl[offset..]))
    }

    pub fn dec_end(self, offset: usize) -> ExpatSliceRc<T> {
        if offset > self.len() {
            panic!(
                "Attempted to decrement the end of an ExpatSliceRc by too much: {:?}",
                offset
            );
        }
        let self_len = self.len();
        ExpatSliceRc(self.0.map(|sl| &sl[..self_len - offset]))
    }

    pub fn map<F: FnOnce(&[T]) -> &[T]>(self, f: F) -> Self {
        ExpatSliceRc(self.0.map(f))
    }

    fn as_buf_ref<'a>(&'a self) -> ExpatBufRef<'a, T> {
        ExpatBufRef::from(&self.0[..])
    }
}

impl<T> ops::Deref for ExpatSliceRc<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &*self.0
    }
}

impl From<StringPoolSlice> for ExpatSliceRc<XML_Char> {
    fn from(sl: StringPoolSlice) -> ExpatSliceRc<XML_Char> {
        ExpatSliceRc(sl.erase_owner())
    }
}

impl From<OwningRef<Rc<Vec<c_char>>, [c_char]>> for ExpatSliceRc<c_char> {
    fn from(or: OwningRef<Rc<Vec<c_char>>, [c_char]>) -> ExpatSliceRc<c_char> {
        ExpatSliceRc(or.erase_owner())
    }
}

/// Create a null-terminated XML_Char array from ASCII_ literals
macro_rules! XML_STR {
    [$($char:ident),* $(,)*] => {
        [$( $char as XML_Char, )* 0,]
    };
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
    fn attlistDecl(
        &self,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: c_int,
    ) -> bool;
    fn characterData(&self, _: &[XML_Char]) -> bool;
    fn comment(&self, b: *const XML_Char) -> bool;
    fn default(&self, _: *const XML_Char, _: c_int) -> bool;
    fn elementDecl(&self, _: *const XML_Char, _: *mut XML_Content) -> bool;
    fn endCDataSection(&self) -> bool;
    fn endDoctypeDecl(&self) -> bool;
    fn endElement(&self, _: *const XML_Char) -> bool;
    fn endNamespaceDecl(&self, _: *const XML_Char) -> bool;
    fn entityDecl(
        &self,
        _: *const XML_Char,
        _: c_int,
        _: *const XML_Char,
        _: c_int,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
    ) -> bool;
    fn externalEntityRef(
        &self,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
    ) -> Result<c_int, ()>;
    fn notationDecl(
        &self,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
    ) -> bool;
    fn notStandalone(&self) -> Result<c_int, ()>;
    fn processingInstruction(&self, b: *const XML_Char, c: *const XML_Char) -> bool;
    fn skippedEntity(&self, _: *const XML_Char, _: c_int) -> bool;
    fn startCDataSection(&self) -> bool;
    fn startDoctypeDecl(
        &self,
        a: *const XML_Char,
        b: *const XML_Char,
        c: *const XML_Char,
        d: c_int,
    ) -> bool;
    fn startElement(&self, _: *const XML_Char, _: &mut [Attribute]) -> bool;
    fn startNamespaceDecl(&self, _: *const XML_Char, b: *const XML_Char) -> bool;
    fn unknownEncoding(&self, _: *const XML_Char, _: *mut XML_Encoding) -> Result<c_int, ()>;
    fn unparsedEntityDecl(
        &self,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
    ) -> bool;
    fn xmlDecl(&self, _: *const XML_Char, _: *const XML_Char, _: c_int) -> bool;
}

#[repr(C)]
#[derive(Clone)]
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
    m_unknownEncoding: Option<Box<UnknownEncoding>>,
    m_unknownEncodingData: *mut c_void,
    m_unknownEncodingRelease: Option<unsafe extern "C" fn(_: *mut c_void)>,
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
    fn startElement(&self, a: *const XML_Char, b: &mut [Attribute]) -> bool {
        self.m_startElementHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b.as_mut_ptr() as *mut *const XML_Char);

                true
            })
            .unwrap_or(false)
    }

    fn endElement(&self, a: *const XML_Char) -> bool {
        self.m_endElementHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a);

                true
            })
            .unwrap_or(false)
    }

    fn characterData(&self, buf: &[XML_Char]) -> bool {
        self.m_characterDataHandler
            .map(|handler| unsafe {
                handler(
                    self.m_handlerArg,
                    buf.as_ptr(),
                    buf.len().try_into().unwrap(),
                );

                true
            })
            .unwrap_or(false)
    }

    fn externalEntityRef(
        &self,
        a: *const XML_Char,
        b: *const XML_Char,
        c: *const XML_Char,
        d: *const XML_Char,
    ) -> Result<c_int, ()> {
        self.m_externalEntityRefHandler
            .map(|handler| unsafe { Ok(handler(self.m_externalEntityRefHandlerArg, a, b, c, d)) })
            .unwrap_or(Err(()))
    }

    fn processingInstruction(&self, a: *const XML_Char, b: *const XML_Char) -> bool {
        self.m_processingInstructionHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b);

                true
            })
            .unwrap_or(false)
    }

    fn comment(&self, a: *const XML_Char) -> bool {
        self.m_commentHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a);

                true
            })
            .unwrap_or(false)
    }

    fn startCDataSection(&self) -> bool {
        self.m_startCdataSectionHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg);

                true
            })
            .unwrap_or(false)
    }

    fn endCDataSection(&self) -> bool {
        self.m_endCdataSectionHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg);

                true
            })
            .unwrap_or(false)
    }

    fn startDoctypeDecl(
        &self,
        a: *const XML_Char,
        b: *const XML_Char,
        c: *const XML_Char,
        d: c_int,
    ) -> bool {
        self.m_startDoctypeDeclHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b, c, d);

                true
            })
            .unwrap_or(false)
    }

    fn endDoctypeDecl(&self) -> bool {
        self.m_endDoctypeDeclHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg);

                true
            })
            .unwrap_or(false)
    }

    fn startNamespaceDecl(&self, a: *const XML_Char, b: *const XML_Char) -> bool {
        self.m_startNamespaceDeclHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b);

                true
            })
            .unwrap_or(false)
    }

    fn endNamespaceDecl(&self, a: *const XML_Char) -> bool {
        self.m_endNamespaceDeclHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a);

                true
            })
            .unwrap_or(false)
    }

    fn elementDecl(&self, a: *const XML_Char, b: *mut XML_Content) -> bool {
        self.m_elementDeclHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b);

                true
            })
            .unwrap_or(false)
    }

    fn attlistDecl(
        &self,
        a: *const XML_Char,
        b: *const XML_Char,
        c: *const XML_Char,
        d: *const XML_Char,
        e: c_int,
    ) -> bool {
        self.m_attlistDeclHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b, c, d, e);

                true
            })
            .unwrap_or(false)
    }

    fn entityDecl(
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
        self.m_entityDeclHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b, c, d, e, f, g, h);

                true
            })
            .unwrap_or(false)
    }

    fn xmlDecl(&self, a: *const XML_Char, b: *const XML_Char, c: c_int) -> bool {
        self.m_xmlDeclHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b, c);

                true
            })
            .unwrap_or(false)
    }

    fn unknownEncoding(&self, a: *const XML_Char, b: *mut XML_Encoding) -> Result<c_int, ()> {
        self.m_unknownEncodingHandler
            .map(|handler| unsafe { Ok(handler(self.m_unknownEncodingHandlerData, a, b)) })
            .unwrap_or(Err(()))
    }

    fn skippedEntity(&self, a: *const XML_Char, b: c_int) -> bool {
        self.m_skippedEntityHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b);

                true
            })
            .unwrap_or(false)
    }

    fn unparsedEntityDecl(
        &self,
        a: *const XML_Char,
        b: *const XML_Char,
        c: *const XML_Char,
        d: *const XML_Char,
        e: *const XML_Char,
    ) -> bool {
        self.m_unparsedEntityDeclHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b, c, d, e);

                true
            })
            .unwrap_or(false)
    }

    fn notationDecl(
        &self,
        a: *const XML_Char,
        b: *const XML_Char,
        c: *const XML_Char,
        d: *const XML_Char,
    ) -> bool {
        self.m_notationDeclHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, a, b, c, d);

                true
            })
            .unwrap_or(false)
    }

    fn notStandalone(&self) -> Result<c_int, ()> {
        self.m_notStandaloneHandler
            .map(|handler| unsafe { Ok(handler(self.m_handlerArg)) })
            .unwrap_or(Err(()))
    }

    fn default(&self, s: *const XML_Char, next: c_int) -> bool {
        self.m_defaultHandler
            .map(|handler| unsafe {
                handler(self.m_handlerArg, s, next);

                true
            })
            .unwrap_or(false)
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

    fn from_default(da: &DefaultAttribute) -> Self {
        Attribute {
            name: da.id.name().as_ptr(),
            value: da.value,
        }
    }
}

#[repr(C)]
pub struct XML_ParserStruct {
    /* The first member must be m_userData so that the XML_GetUserData
    macro works. */
    pub m_userData: *mut c_void,
    m_buffer: Rc<Vec<c_char>>,
    // index in m_buffer of first character to be parsed
    m_bufferStart: usize,
    // index in m_buffer after last character to be parsed
    m_bufferEnd: usize,
    // Absolute index after last character that has been parsed (in the overall
    // input stream)
    m_parseEndByteIndex: usize,
    // Index in m_buffer after last character that has been parsed
    m_parseEndIdx: usize,
    // Temporary scratch buffer
    m_dataBuf: Box<[XML_Char; INIT_DATA_BUF_SIZE as usize]>,

    // Handlers should be trait, with native C callback instance
    m_handlers: CXmlHandlers,
    pub m_encoding: *const ENCODING,
    pub m_initEncoding: Option<InitEncoding>,
    pub m_internalEncoding: &'static super::xmltok::ENCODING,
    pub m_protocolEncodingName: Option<Vec<XML_Char>>,
    pub m_ns: XML_Bool,
    pub m_ns_triplets: XML_Bool,
    pub m_prologState: super::xmlrole::PROLOG_STATE,
    pub m_processor: Option<Processor>,
    pub m_errorCode: XML_Error,
    pub m_eventPtr: Cell<*const c_char>,
    pub m_eventEndPtr: Cell<*const c_char>,
    pub m_positionIdx: Option<usize>,
    pub m_openInternalEntities: Option<Box<OpenInternalEntity>>,
    pub m_freeInternalEntities: Option<Box<OpenInternalEntity>>,
    pub m_defaultExpandInternalEntities: XML_Bool,
    pub m_tagLevel: c_int,
    pub m_declEntity: Ptr<Entity>,
    m_doctypeName: Option<StringPoolSlice>,
    pub m_doctypeSysid: *const XML_Char,
    pub m_doctypePubid: *const XML_Char,
    pub m_declAttributeType: *const XML_Char,
    pub m_declNotationName: *const XML_Char,
    pub m_declNotationPublicId: *const XML_Char,
    pub m_declElementType: Ptr<ElementType>,
    pub m_declAttributeId: Ptr<AttributeId>,
    pub m_declAttributeIsCdata: XML_Bool,
    pub m_declAttributeIsId: XML_Bool,
    pub m_dtd: Rc<DTD>,
    pub m_curBase: *const XML_Char,
    pub m_tagStack: Option<Box<Tag>>,
    pub m_freeTagList: Option<Box<Tag>>,
    m_inheritedBindings: RefCell<Ptr<Binding>>,
    m_freeBindingList: RefCell<Ptr<Binding>>,
    pub m_nSpecifiedAtts: c_int,
    pub m_idAttIndex: c_int,
    pub m_atts: Vec<Attribute>,
    typed_atts: Vec<TypedAttributeName>,
    m_nsAtts: HashSet<StringPoolSlice>,
    pub m_position: super::xmltok::Position,
    m_tempPool: StringPool,
    m_temp2Pool: StringPool,
    pub m_groupConnector: Vec<c_char>,
    pub m_namespaceSeparator: XML_Char,
    is_child_parser: bool,
    pub m_parsingStatus: XML_ParsingStatus,
    pub m_isParamEntity: XML_Bool,
    pub m_useForeignDTD: XML_Bool,
    pub m_paramEntityParsing: XML_ParamEntityParsing,

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

    // TODO(SJC): add a better err type
    fn buffer_index(&self, p: *const c_char) -> Result<usize, ()> {
        if p < self.m_buffer.as_ptr()
            || p >= self.m_buffer.as_ptr().wrapping_add(self.m_buffer.len())
        {
            Err(())
        } else {
            Ok(p.wrapping_offset_from(self.m_buffer.as_ptr()) as usize)
        }
    }
}

#[repr(C)]
pub struct Binding {
    pub prefix: Weak<Prefix>,
    pub nextTagBinding: Cell<Ptr<Binding>>,
    pub prevPrefixBinding: Cell<Ptr<Binding>>,
    pub attId: Cell<Ptr<AttributeId>>,
    pub uri: RefCell<Vec<XML_Char>>,
    pub uriLen: Cell<usize>,
}

#[repr(C)]
pub struct AttributeId {
    name: Option<StringPoolSlice>,
    r#type: RefCell<Option<StringPoolCellSlice>>,
    // TODO(rexpat): Cell or RefCell?
    pub prefix: RefCell<Option<Weak<Prefix>>>,
    pub maybeTokenized: Cell<XML_Bool>,
    pub xmlns: Cell<XML_Bool>,
}

impl AttributeId {
    fn new(name: StringPoolSlice) -> AttributeId {
        AttributeId {
            name: Some(name),
            r#type: RefCell::new(None),
            prefix: RefCell::new(None),
            maybeTokenized: Cell::new(false),
            xmlns: Cell::new(false),
        }
    }

    fn typed_name(&self) -> TypedAttributeName {
        TypedAttributeName {
            name: self.name.as_ref().unwrap().clone(),
            r#type: self.r#type.borrow().as_ref().unwrap().clone(),
        }
    }

    fn get_type(&self) -> AttributeType {
        self.r#type.borrow().as_ref().unwrap()[0].get().into()
    }

    fn set_type(&self, r#type: AttributeType) {
        self.r#type.borrow().as_ref().unwrap()[0].set(r#type.into());
    }

    fn name(&self) -> &StringPoolSlice {
        self.name.as_ref().unwrap()
    }
}

struct TypedAttributeName {
    name: StringPoolSlice,
    r#type: StringPoolCellSlice,
}

impl TypedAttributeName {
    #[inline]
    fn get_type(&self) -> AttributeType {
        assert!(self.r#type.len() == 1);
        self.r#type[0].get().into()
    }

    #[inline]
    fn set_type(&self, at: AttributeType) {
        assert!(self.r#type.len() == 1);
        self.r#type[0].set(at.into())
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
            _ => true,
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
            _ => panic!("invalid attribute type byte: {}", c),
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

pub struct Prefix {
    name: Option<StringPoolSlice>,
    binding: Cell<Ptr<Binding>>,
}

impl Prefix {
    fn new(name: StringPoolSlice) -> Prefix {
        Prefix {
            name: Some(name),
            binding: Cell::new(None),
        }
    }

    fn reset(&self) {
        // FIXME(rexpat): reset `name` too?
        self.binding.set(None);
    }
}

impl Default for Prefix {
    fn default() -> Self {
        Prefix {
            name: None,
            binding: Cell::new(None),
        }
    }
}

/// Helper function that copies a pointer inside a `Cell<Ptr<T>>`
#[inline]
fn get_cell_ptr<T>(cell: &Cell<Ptr<T>>) -> Ptr<T> {
    let rc = cell.take();
    cell.set(rc.as_ref().map(Rc::clone));
    rc
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
pub struct Tag {
    /// parent of this element
    pub parent: Option<Box<Tag>>,

    /// tagName in the original encoding
    pub rawName: *const c_char,
    pub rawNameLength: c_int,

    /// tagName in the API encoding
    pub name: TagName,

    /// buffers for name components
    pub buf: Rc<Vec<XML_Char>>,
    pub rawNameBuf: Rc<Vec<c_char>>,

    pub bindings: Ptr<Binding>,
}

impl Tag {
    fn try_new() -> Result<Tag, ()> {
        let mut buf = vec![];
        buf.try_reserve(INIT_TAG_BUF_SIZE).map_err(|_| ())?;

        Ok(Tag {
            parent: None,
            rawName: ptr::null(),
            rawNameLength: 0,
            name: TagName::default(),
            buf: Rc::try_new(buf).map_err(|_| ())?,
            rawNameBuf: Rc::try_new(vec![]).map_err(|_| ())?,
            bindings: None,
        })
    }
}

#[repr(C)]
pub struct TagName {
    pub str_0: TagNameString,
    pub localPart: TagNameLocalPart,
    pub prefix: *const XML_Char,
    pub strLen: c_int,
    pub uriLen: usize,
    pub prefixLen: usize,
}

impl Default for TagName {
    fn default() -> TagName {
        TagName {
            str_0: TagNameString::None,
            localPart: TagNameLocalPart::None,
            prefix: ptr::null(),
            strLen: 0,
            uriLen: 0,
            prefixLen: 0,
        }
    }
}

pub enum TagNameString {
    None,
    // TODO(rexpat): get rid of this one
    Ptr(*const XML_Char),
    BindingUri(Rc<Binding>),
}

impl TagNameString {
    fn as_ptr(&self) -> *const XML_Char {
        match *self {
            TagNameString::None => ptr::null(),
            TagNameString::Ptr(ptr) => ptr,
            TagNameString::BindingUri(ref b) => b.uri.borrow().as_ptr(),
        }
    }

    fn with_slice<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&[XML_Char]) -> T,
    {
        match *self {
            TagNameString::None => f(&[]),
            TagNameString::Ptr(ptr) => unsafe {
                let len = strlen(ptr) as usize;
                let sl = std::slice::from_raw_parts(ptr, len + 1);
                f(sl)
            },
            TagNameString::BindingUri(ref b) => {
                let r = b.uri.borrow();
                f(&r[..])
            }
        }
    }
}

pub enum TagNameLocalPart {
    None,
    Ptr(*const XML_Char),
    BindingUri(Rc<Binding>, usize),
}

impl TagNameLocalPart {
    fn as_ptr(&self) -> *const XML_Char {
        match *self {
            TagNameLocalPart::None => ptr::null(),
            TagNameLocalPart::Ptr(ptr) => ptr,
            TagNameLocalPart::BindingUri(ref b, off) => b.uri.borrow()[off..].as_ptr(),
        }
    }

    fn from_tag_name(str_0: &TagNameString, skip_to_colon: bool) -> TagNameLocalPart {
        match str_0 {
            TagNameString::None => TagNameLocalPart::None,
            TagNameString::Ptr(mut ptr) => unsafe {
                if skip_to_colon {
                    while *ptr != ASCII_COLON {
                        ptr = ptr.add(1);
                    }
                    ptr = ptr.add(1);
                }
                TagNameLocalPart::Ptr(ptr)
            },
            TagNameString::BindingUri(b) => {
                let off = if skip_to_colon {
                    b.uri
                        .borrow()
                        .iter()
                        .position(|&c| c == ASCII_COLON)
                        .map(|p| p + 1)
                        .unwrap_or(0)
                } else {
                    0
                };
                TagNameLocalPart::BindingUri(Rc::clone(b), off)
            }
        }
    }
}

/// Round up n to be a multiple of sz, where sz is a power of 2.
#[inline]
fn round_up(n: usize, sz: usize) -> usize {
    (n + (sz - 1)) & !(sz - 1)
}

#[inline]
fn safe_ptr_diff<T>(p: *const T, q: *const T) -> isize {
    if p.is_null() || q.is_null() {
        0
    } else {
        p.wrapping_offset_from(q)
    }
}

enum HashInsertResult<T> {
    New(T),
    Found(T),
    Err,
}

impl<T> HashInsertResult<T> {
    fn is_new(&self) -> bool {
        match self {
            HashInsertResult::New(_) => true,
            _ => false,
        }
    }

    fn is_found(&self) -> bool {
        match self {
            HashInsertResult::Found(_) => true,
            _ => false,
        }
    }

    fn is_err(&self) -> bool {
        match self {
            HashInsertResult::Err => true,
            _ => false,
        }
    }
}

impl<'a, T> HashInsertResult<&'a Rc<T>> {
    fn cloned(&self) -> HashInsertResult<Rc<T>> {
        match self {
            HashInsertResult::New(r) => HashInsertResult::New(Rc::clone(r)),
            HashInsertResult::Found(r) => HashInsertResult::Found(Rc::clone(r)),
            HashInsertResult::Err => HashInsertResult::Err,
        }
    }
}

impl<T> From<HashInsertResult<T>> for Option<T> {
    fn from(x: HashInsertResult<T>) -> Self {
        match x {
            HashInsertResult::New(x) | HashInsertResult::Found(x) => Some(x),
            HashInsertResult::Err => None,
        }
    }
}

macro_rules! hash_insert {
    ($map:expr, $key:expr, $alloc:expr, $et:ident) => {{
        let __map = $map;
        let __key = $key;
        let __res = __map.get(&__key);
        if let Some(__res) = __res {
            HashInsertResult::Found(__res)
        } else if __map.try_reserve(1).is_err() {
            HashInsertResult::Err
        } else {
            let v = $et::new(OwningRef::clone(&__key));
            if let Ok(b) = $alloc(v) {
                use hash_map::Entry::*;
                match __map.entry(__key) {
                    Occupied(_) => panic!("found Occupied hash key"),
                    Vacant(e) => HashInsertResult::New(&*e.insert(b)),
                }
            } else {
                HashInsertResult::Err
            }
        }
    }};
}

macro_rules! hash_insert_pool {
    ($map:expr, $pool:expr, $et:ident) => {{
        let __res = $pool.with_current_slice(|__key| $map.get(__key));
        if let Some(__res) = __res {
            $pool.discard();
            HashInsertResult::Found(__res)
        } else if $map.try_reserve(1).is_err() {
            HashInsertResult::Err
        } else {
            let __key = $pool.finish();
            let v = $et::new(OwningRef::clone(&__key));
            if let Ok(b) = Rc::try_new(v) {
                use hash_map::Entry::*;
                match $map.entry(__key) {
                    Occupied(_) => panic!("found Occupied hash key"),
                    Vacant(e) => HashInsertResult::New(&*e.insert(b)),
                }
            } else {
                HashInsertResult::Err
            }
        }
    }};
}

#[repr(C)]
pub struct DTD {
    // The string pools for this DTD
    pool: StringPool,
    entityValuePool: StringPool,

    // All the hash tables merged into one structure;
    // this is where most of the interior mutability lies
    // that can't be stored in `Cell`s below
    tables: RefCell<DTDTables>,

    scaffold: Rc<RefCell<DTDScaffold>>,
    keepProcessing: Cell<XML_Bool>,
    hasParamEntityRefs: Cell<XML_Bool>,
    standalone: Cell<XML_Bool>,
    paramEntityRead: Cell<XML_Bool>,
    in_eldecl: Cell<XML_Bool>,
    contentStringLen: Cell<c_uint>,
}

#[derive(Default)]
pub struct DTDTables {
    defaultPrefix: Ptr<Prefix>,
    externalSubsetSlice: Option<StringPoolSlice>,
    generalEntities: HashMap<StringPoolSlice, Rc<Entity>>,
    elementTypes: HashMap<StringPoolSlice, Rc<ElementType>>,
    attributeIds: HashMap<StringPoolSlice, Rc<AttributeId>>,
    prefixes: HashMap<StringPoolSlice, Rc<Prefix>>,
    paramEntities: HashMap<StringPoolSlice, Rc<Entity>>,
}

impl DTD {
    fn try_new() -> Result<DTD, TryReserveError> {
        // Eagerly allocate the `defaultPrefix`
        let defaultPrefix = match Rc::try_new(Default::default()) {
            Ok(p) => Some(p),
            Err(e) => return Err(e),
        };

        Ok(DTD {
            pool: StringPool::try_new().map_err(|_| TryReserveError::CapacityOverflow)?,
            entityValuePool: StringPool::try_new()
                .map_err(|_| TryReserveError::CapacityOverflow)?,
            tables: RefCell::new(DTDTables {
                defaultPrefix,
                ..Default::default()
            }),
            keepProcessing: Cell::new(true),
            hasParamEntityRefs: Cell::new(false),
            standalone: Cell::new(false),
            paramEntityRead: Cell::new(false),
            in_eldecl: Cell::new(false),
            scaffold: Rc::try_new(Default::default())?,
            contentStringLen: Cell::new(0),
        })
    }

    fn reset(&mut self) {
        let mut tables = self.tables.get_mut();
        tables.defaultPrefix.as_ref().unwrap().reset();
        tables.externalSubsetSlice = None;
        tables.generalEntities.clear();
        tables.paramEntities.clear();
        tables.elementTypes.clear();
        tables.attributeIds.clear();
        tables.prefixes.clear();

        let mut scf = self.scaffold.borrow_mut();
        scf.scaffold.clear();
        scf.index.clear();

        self.paramEntityRead.set(false);
        self.in_eldecl.set(false);
        self.contentStringLen.set(0);
        self.keepProcessing.set(true);
        self.hasParamEntityRefs.set(false);
        self.standalone.set(false);

        self.pool.clear();
        self.entityValuePool.clear();
    }

    // Get a copy of externalSubsetName interned in `self.pool`
    fn intern_external_subset(&self) -> Result<StringPoolSlice, TryReserveError> {
        const externalSubsetName: [XML_Char; 2] = XML_STR![ASCII_HASH];
        if let Some(ess) = self.tables.borrow().externalSubsetSlice.as_ref() {
            return Ok(OwningRef::clone(ess));
        }

        for &c in &externalSubsetName {
            if !self.pool.append_char(c) {
                return Err(TryReserveError::CapacityOverflow);
            }
        }
        let sl = self.pool.finish();
        Ok(OwningRef::clone(
            self.tables
                .borrow_mut()
                .externalSubsetSlice
                .get_or_insert(sl),
        ))
    }

    fn try_clone(&self) -> Result<DTD, TryReserveError> {
        let mut newDtd = DTD {
            pool: StringPool::try_new().map_err(|_| TryReserveError::CapacityOverflow)?,
            entityValuePool: StringPool::try_new()
                .map_err(|_| TryReserveError::CapacityOverflow)?,
            tables: Default::default(),
            keepProcessing: self.keepProcessing.clone(),
            hasParamEntityRefs: self.hasParamEntityRefs.clone(),
            standalone: self.standalone.clone(),
            paramEntityRead: self.paramEntityRead.clone(),
            in_eldecl: self.in_eldecl.clone(),
            /* Don't want deep copying for scaffolding */
            scaffold: Rc::clone(&self.scaffold),
            contentStringLen: self.contentStringLen.clone(),
        };

        unsafe {
            let mut new_tables = newDtd.tables.get_mut();
            let old_tables = self.tables.borrow();

            new_tables.defaultPrefix = match Rc::try_new(Default::default()) {
                Ok(p) => Some(p),
                Err(e) => return Err(e),
            };

            /* Copy the prefix table. */
            for oldP in old_tables.prefixes.values() {
                // TODO(rexpat): pass the StringPoolSlice directly
                if !newDtd
                    .pool
                    .copy_c_string(oldP.name.as_ref().unwrap().as_ptr())
                {
                    return Err(TryReserveError::CapacityOverflow);
                }
                if hash_insert_pool!(new_tables.prefixes, &newDtd.pool, Prefix).is_err() {
                    return Err(TryReserveError::CapacityOverflow);
                }
            }
            for oldA in old_tables.attributeIds.values()
            /* Copy the attribute id table. */
            {
                if !newDtd.pool.copy_c_string(oldA.name().as_ptr()) {
                    return Err(TryReserveError::CapacityOverflow);
                }
                let newA =
                    match hash_insert_pool!(new_tables.attributeIds, &newDtd.pool, AttributeId) {
                        HashInsertResult::New(newA) => newA,
                        HashInsertResult::Found(_) => panic!("AttributeId already found"),
                        HashInsertResult::Err => return Err(TryReserveError::CapacityOverflow),
                    };

                if !newDtd.pool.append_char(AttributeType::Unset.into()) {
                    return Err(TryReserveError::CapacityOverflow);
                }
                let r#type = newDtd.pool.finish_cells();
                assert!(r#type.len() == 1);
                *newA.r#type.borrow_mut() = Some(r#type);

                newA.maybeTokenized.set(oldA.maybeTokenized.get());
                if let Some(old_prefix) = oldA.prefix.borrow().as_ref().and_then(Weak::upgrade) {
                    newA.xmlns.set(oldA.xmlns.get());
                    if Rc::ptr_eq(&old_prefix, old_tables.defaultPrefix.as_ref().unwrap()) {
                        *newA.prefix.borrow_mut() =
                            new_tables.defaultPrefix.as_ref().map(Rc::downgrade);
                    } else {
                        *newA.prefix.borrow_mut() = new_tables
                            .prefixes
                            .get(old_prefix.name.as_ref().unwrap())
                            .map(Rc::downgrade);
                    }
                }
            }
            /* Copy the element type table. */
            for oldE in old_tables.elementTypes.values() {
                // TODO(rexpat): pass in the name as a StringPoolSlice
                if !newDtd.pool.copy_c_string(oldE.name.as_ptr()) {
                    return Err(TryReserveError::CapacityOverflow);
                };
                let newE =
                    match hash_insert_pool!(new_tables.elementTypes, &newDtd.pool, ElementType) {
                        HashInsertResult::New(newE) => newE,
                        HashInsertResult::Found(_) => panic!("ElementType already found"),
                        HashInsertResult::Err => return Err(TryReserveError::CapacityOverflow),
                    };
                if let Some(id_att) = get_cell_ptr(&oldE.idAtt) {
                    let id_att = new_tables.attributeIds.get(id_att.name()).map(Rc::clone);
                    newE.idAtt.set(id_att);
                }
                if let Some(ref old_prefix) = get_cell_ptr(&oldE.prefix) {
                    newE.prefix.set(
                        new_tables
                            .prefixes
                            .get(old_prefix.name.as_ref().unwrap())
                            .map(Rc::clone),
                    );
                }

                let mut new_default_atts = newE.defaultAtts.borrow_mut();
                let old_default_atts = oldE.defaultAtts.borrow();
                if new_default_atts
                    .try_reserve(old_default_atts.len())
                    .is_err()
                {
                    return Err(TryReserveError::CapacityOverflow);
                }
                for oldAtt in old_default_atts.iter() {
                    let id = new_tables.attributeIds.get(oldAtt.id.name()).unwrap();
                    let isCdata = oldAtt.isCdata;
                    let value = if !oldAtt.value.is_null() {
                        if !newDtd.pool.copy_c_string(oldAtt.value) {
                            return Err(TryReserveError::CapacityOverflow);
                        };
                        newDtd.pool.finish().as_ptr()
                    } else {
                        ptr::null()
                    };
                    let newAtt = DefaultAttribute {
                        id: Rc::clone(id),
                        isCdata,
                        value,
                    };
                    new_default_atts.push(newAtt);
                }
            }
            /* Copy the entity tables. */
            if copyEntityTable(
                &mut new_tables.generalEntities,
                &mut newDtd.pool,
                &old_tables.generalEntities,
            ) == 0
            {
                return Err(TryReserveError::CapacityOverflow);
            }
            if copyEntityTable(
                &mut new_tables.paramEntities,
                &mut newDtd.pool,
                &old_tables.paramEntities,
            ) == 0
            {
                return Err(TryReserveError::CapacityOverflow);
            }
        }

        Ok(newDtd)
    }
}

impl Clone for DTD {
    fn clone(&self) -> DTD {
        panic!("tried to clone a DTD");
    }
}

#[repr(C)]
#[derive(Default)]
pub struct DTDScaffold {
    scaffold: Vec<ContentScaffold>,
    index: Vec<usize>,
}

impl DTDScaffold {
    fn next_part(&mut self) -> Option<usize> {
        if self.scaffold.try_reserve(1).is_err() {
            return None;
        }

        let next = self.scaffold.len();
        self.scaffold.push(Default::default());

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

impl Clone for DTDScaffold {
    fn clone(&self) -> DTDScaffold {
        panic!("tried to clone a DTD scaffold");
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct ContentScaffold {
    pub type_0: XML_Content_Type,
    pub quant: XML_Content_Quant,
    name: Option<StringPoolSlice>,
    pub firstchild: usize,
    pub lastchild: usize,
    pub childcnt: usize,
    pub nextsib: usize,
}

impl Default for ContentScaffold {
    fn default() -> ContentScaffold {
        ContentScaffold {
            type_0: XML_Content_Type::EMPTY,
            quant: XML_Content_Quant::NONE,
            name: None,
            firstchild: 0,
            lastchild: 0,
            childcnt: 0,
            nextsib: 0,
        }
    }
}

/// Data structure that gets passed to elementDecl wrapped in a `Box`.
struct ContentModel {
    root: XML_Content,
    nodes: Vec<XML_Content>,
    strings: Vec<XML_Char>,
}

impl ContentModel {
    fn new(root: XML_Content) -> Self {
        Self {
            root,
            nodes: vec![],
            strings: vec![],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Named {
    pub name: KEY,
}

pub type KEY = *const XML_Char;
/* The XML_Char before the name is used to determine whether
an attribute has been specified. */

#[repr(C)]
pub struct ElementType {
    name: StringPoolSlice,
    pub prefix: Cell<Ptr<Prefix>>,
    pub idAtt: Cell<Ptr<AttributeId>>,
    pub defaultAtts: RefCell<Vec<DefaultAttribute>>,
}

impl ElementType {
    fn new(name: StringPoolSlice) -> Self {
        ElementType {
            name,
            prefix: Cell::new(None),
            idAtt: Cell::new(None),
            defaultAtts: RefCell::new(Vec::new()),
        }
    }
}

#[repr(C)]
pub struct DefaultAttribute {
    pub id: Rc<AttributeId>,
    pub isCdata: XML_Bool,
    pub value: *const XML_Char,
}

#[repr(C)]
pub struct Entity {
    // TODO: get rid of the RefCell
    name: RefCell<Option<StringPoolSlice>>,
    textPtr: RefCell<Option<StringPoolSlice>>,
    pub processed: Cell<c_int>,
    pub systemId: Cell<*const XML_Char>,
    pub base: Cell<*const XML_Char>,
    pub publicId: Cell<*const XML_Char>,
    pub notation: Cell<*const XML_Char>,
    pub open: Cell<XML_Bool>,
    pub is_param: Cell<XML_Bool>,
    pub is_internal: Cell<XML_Bool>,
}

impl Entity {
    fn new(name: StringPoolSlice) -> Self {
        Entity {
            name: RefCell::new(Some(name)),
            textPtr: RefCell::new(None),
            processed: Cell::new(0),
            systemId: Cell::new(ptr::null()),
            base: Cell::new(ptr::null()),
            publicId: Cell::new(ptr::null()),
            notation: Cell::new(ptr::null()),
            open: Cell::new(false),
            is_param: Cell::new(false),
            is_internal: Cell::new(false),
        }
    }
}

#[repr(C)]
pub struct OpenInternalEntity {
    pub internalEventPtr: Cell<*const c_char>,
    pub internalEventEndPtr: Cell<*const c_char>,
    pub next: Option<Box<OpenInternalEntity>>,
    pub entity: Ptr<Entity>,
    pub startTagLevel: c_int,
    pub betweenDecl: XML_Bool,
}

impl Default for OpenInternalEntity {
    fn default() -> OpenInternalEntity {
        OpenInternalEntity {
            internalEventPtr: Cell::new(ptr::null()),
            internalEventEndPtr: Cell::new(ptr::null()),
            next: None,
            entity: None,
            startTagLevel: 0,
            betweenDecl: false,
        }
    }
}

pub type Processor =
    unsafe extern "C" fn(_: XML_Parser, _: ExpatSliceRc, _: &mut *const c_char) -> XML_Error;

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

pub const INIT_TAG_BUF_SIZE: usize = 32; /* must be a multiple of sizeof(XML_Char) */

pub const INIT_DATA_BUF_SIZE: c_int = 1024;

pub const INIT_ATTS_SIZE: c_int = 16;

pub const INIT_ATTS_VERSION: c_uint = 0xffffffff;

pub const INIT_BUFFER_SIZE: c_int = 1024;

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
        let layout = Layout::from_size_align($size as usize, 1).expect("failed to create Layout");
        alloc::realloc($ptr as *mut u8, layout, $size as usize) as *mut c_void
    }};
    ($ptr:expr => [$ty:ty; $n:expr]) => {{
        // FIXME: we should pass the old `Layout` to `realloc`,
        // but we don't have it
        let layout = Layout::array::<$ty>($n as usize).expect("failed to create array Layout");
        alloc::realloc($ptr as *mut u8, layout, layout.size()) as *mut $ty
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
    XML_ParserCreate_MM(encodingName, None, ptr::null())
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
    XML_ParserCreate_MM(encodingName, None, tmp.as_mut_ptr())
}

const implicitContext: [XML_Char; 41] = XML_STR![
    ASCII_x,
    ASCII_m,
    ASCII_l,
    ASCII_EQUALS,
    ASCII_h,
    ASCII_t,
    ASCII_t,
    ASCII_p,
    ASCII_COLON,
    ASCII_SLASH,
    ASCII_SLASH,
    ASCII_w,
    ASCII_w,
    ASCII_w,
    ASCII_PERIOD,
    ASCII_w,
    ASCII_3,
    ASCII_PERIOD,
    ASCII_o,
    ASCII_r,
    ASCII_g,
    ASCII_SLASH,
    ASCII_X,
    ASCII_M,
    ASCII_L,
    ASCII_SLASH,
    ASCII_1,
    ASCII_9,
    ASCII_9,
    ASCII_8,
    ASCII_SLASH,
    ASCII_n,
    ASCII_a,
    ASCII_m,
    ASCII_e,
    ASCII_s,
    ASCII_p,
    ASCII_a,
    ASCII_c,
    ASCII_e,
];

/* To avoid warnings about unused functions: */

impl XML_ParserStruct {
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
    let dtd = match DTD::try_new().and_then(Rc::try_new) {
        Ok(dtd) => dtd,
        Err(_) => return ptr::null_mut(),
    };
    let encodingName = slice_from_c_string(encodingName);
    XML_ParserStruct::try_new(encodingName, nameSep, dtd)
        .map_or(ptr::null_mut(), Box::into_raw)
}

    // fn try_new(use_namespaces: bool, dtd: Rc<DTD>) -> Result<Self, ()> {
    // fn new(use_namespaces: bool) -> Result<Self, TryReserveError> {
impl XML_ParserStruct {
    unsafe fn try_new(
        mut encodingName: Option<&[XML_Char]>,
        mut nameSep: *const XML_Char,
        mut dtd: Rc<DTD>,
    ) -> Result<Box<Self>, ()> {
        let use_namespaces = !nameSep.is_null();

        let m_dataBuf = Box::try_new([0; INIT_DATA_BUF_SIZE as usize])
            .map_err(|_| ())?;

        let m_buffer = Rc::try_new(Vec::new())
            .map_err(|_| ())?;

        let parser = Self {
            m_userData: ptr::null_mut(),
            m_buffer,
            // index of first character to be parsed
            m_bufferStart: 0,
            // index after last character to be parsed
            m_bufferEnd: 0,
            m_parseEndByteIndex: 0,
            m_parseEndIdx: 0,
            m_dataBuf,

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

            m_protocolEncodingName: None,
            m_ns: false,
            m_ns_triplets: false,
            m_prologState: super::xmlrole::PROLOG_STATE::default(),
            m_processor: None,
            m_errorCode: XML_Error::NONE,
            m_eventPtr: Cell::new(ptr::null()),
            m_eventEndPtr: Cell::new(ptr::null()),
            m_positionIdx: None,
            m_openInternalEntities: None,
            m_freeInternalEntities: None,
            m_defaultExpandInternalEntities: false,
            m_tagLevel: 0,
            m_declEntity: None,
            m_doctypeName: None,
            m_doctypeSysid: ptr::null(),
            m_doctypePubid: ptr::null(),
            m_declAttributeType: ptr::null(),
            m_declNotationName: ptr::null(),
            m_declNotationPublicId: ptr::null(),
            m_declElementType: None,
            m_declAttributeId: None,
            m_declAttributeIsCdata: false,
            m_declAttributeIsId: false,
            m_dtd: dtd,
            m_curBase: ptr::null(),
            m_tagStack: None,
            m_freeTagList: None,
            m_inheritedBindings: RefCell::new(None),
            m_freeBindingList: RefCell::new(None),
            m_nSpecifiedAtts: 0,
            m_idAttIndex: 0,
            m_atts: Vec::new(),
            typed_atts: Vec::new(),
            m_nsAtts: HashSet::new(),
            m_position: super::xmltok::Position::default(),
            m_tempPool: StringPool::try_new()?,
            m_temp2Pool: StringPool::try_new()?,
            m_groupConnector: Vec::new(),
            is_child_parser: false,
            m_namespaceSeparator: ASCII_EXCL as XML_Char,
            // m_parentParser: ptr::null_mut(),
            m_parsingStatus: XML_ParsingStatus::default(),
            m_isParamEntity: false,
            m_useForeignDTD: false,
            m_paramEntityParsing: XML_ParamEntityParsing::NEVER,

            #[cfg(feature = "mozilla")]
            m_mismatch: ptr::null(),
        };

        let mut parser = Box::try_new(parser)
            .map_err(|_| ())?;

        // TODO: Move initialization into XML_ParserStruct::new
        if parser.m_atts.try_reserve(INIT_ATTS_SIZE as usize).is_err() {
            return Err(());
        }
        if parser
            .typed_atts
            .try_reserve(INIT_ATTS_SIZE as usize)
            .is_err()
        {
            return Err(());
        }
        *parser.m_freeBindingList.borrow_mut() = None;
        parser.m_freeTagList = None;
        parser.m_freeInternalEntities = None;
        parser.m_groupConnector.clear();
        parser.m_initEncoding = None;
        parser.m_handlers.m_unknownEncoding = None;
        parser.m_namespaceSeparator = ASCII_EXCL as XML_Char;
        parser.m_ns = false;
        parser.m_ns_triplets = false;
        parser.m_protocolEncodingName = None;
        parser.init(encodingName);
        if !encodingName.is_none() && parser.m_protocolEncodingName.is_none() {
            return Err(());
        }
        if !nameSep.is_null() {
            parser.m_ns = true;
            parser.m_namespaceSeparator = *nameSep
        }

        Ok(parser)
    }

    unsafe fn init(&mut self, encodingName: Option<&[XML_Char]>) {
        self.m_processor = Some(prologInitProcessor as Processor);
        super::xmlrole::XmlPrologStateInit(&mut self.m_prologState);
        self.m_protocolEncodingName = copy_c_string(encodingName);
        self.m_curBase = ptr::null();
        self.m_initEncoding = InitEncoding::new(&mut self.m_encoding, ptr::null());
        self.m_encoding = self.m_initEncoding.as_ref().unwrap();
        self.m_userData = ptr::null_mut();
        self.m_handlers = Default::default();
        self.m_handlers.m_externalEntityRefHandlerArg = self as XML_Parser;
        self.m_bufferStart = 0;
        self.m_bufferEnd = 0;
        self.m_parseEndByteIndex = 0;
        self.m_parseEndIdx = 0;
        self.m_declElementType = None;
        self.m_declAttributeId = None;
        self.m_declEntity = None;
        self.m_doctypeName = None;
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
        self.m_eventPtr.set(ptr::null());
        self.m_eventEndPtr.set(ptr::null());
        self.m_positionIdx = Some(0);
        self.m_openInternalEntities = None;
        self.m_defaultExpandInternalEntities = true;
        self.m_tagLevel = 0;
        self.m_tagStack = None;
        *self.m_inheritedBindings.borrow_mut() = None;
        self.m_nSpecifiedAtts = 0;
        self.m_handlers.m_unknownEncoding = None;
        self.m_handlers.m_unknownEncodingRelease = None;
        self.m_handlers.m_unknownEncodingData = ptr::null_mut();
        self.is_child_parser = false;
        self.m_parsingStatus.parsing = XML_Parsing::INITIALIZED;
        self.m_isParamEntity = false;
        self.m_useForeignDTD = false;
        self.m_paramEntityParsing = XML_ParamEntityParsing::NEVER;
    }

    /* moves list of bindings to m_freeBindingList */
    fn moveToFreeBindingList(&mut self, mut bindings: Ptr<Binding>) {
        let mut fbl = self.m_freeBindingList.borrow_mut();
        while let Some(b) = bindings {
            // Release the Rc's inside attId
            b.attId.set(None);
            bindings = b.nextTagBinding.replace(fbl.take());
            *fbl = Some(b);
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
    pub unsafe fn reset(&mut self, encodingName: Option<&[XML_Char]>) -> XML_Bool {
        if self.is_child_parser {
            return false;
        }

        // Clear any references to the string pools before clearing the latter
        self.m_doctypeName = None;
        self.m_declEntity = None;
        self.m_declElementType = None;
        self.m_declAttributeId = None;
        self.typed_atts.clear();
        self.m_nsAtts.clear();

        /* move m_tagStack to m_freeTagList */
        let mut tStk = self.m_tagStack.take();
        while let Some(mut tag) = tStk {
            self.moveToFreeBindingList(tag.bindings.take());

            let new_parent = self.m_freeTagList.take();
            tStk = std::mem::replace(&mut tag.parent, new_parent);
            self.m_freeTagList = Some(tag);
        }

        /* move m_openInternalEntities to m_freeInternalEntities */
        let mut openEntities = self.m_openInternalEntities.take();
        while let Some(mut openEntity) = openEntities {
            // oEs = oE.next; oE.next = fIE; fIE = oE (below);
            openEntities =
                std::mem::replace(&mut openEntity.next, self.m_freeInternalEntities.take());
            self.m_freeInternalEntities = Some(openEntity);
        }

        // Drop the Rc's for all the free entity names
        let mut freeEntities = self.m_freeInternalEntities.as_mut();
        while let Some(freeEntity) = freeEntities {
            if let Some(ref mut entity) = freeEntity.entity {
                *entity.name.borrow_mut() = None;
                *entity.textPtr.borrow_mut() = None;
            }
            freeEntities = freeEntity.next.as_mut();
        }

        let ib = self.m_inheritedBindings.borrow_mut().take();
        self.moveToFreeBindingList(ib);
        let _ = self.m_handlers.m_unknownEncoding.take();
        if self.m_handlers.m_unknownEncodingRelease.is_some() {
            self.m_handlers
                .m_unknownEncodingRelease
                .expect("non-null function pointer")(
                self.m_handlers.m_unknownEncodingData
            );
        }

        self.m_tempPool.clear();
        self.m_temp2Pool.clear();
        self.m_protocolEncodingName = None;
        self.init(encodingName);
        Rc::get_mut(&mut self.m_dtd).unwrap().reset();
        true
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserReset(
    parser: XML_Parser,
    encodingName: *const XML_Char,
) -> XML_Bool {
    if parser.is_null() {
        return false;
    }
    let encodingName = slice_from_c_string(encodingName);
    (*parser).reset(encodingName)
}
/* Returns the last value set by XML_SetUserData or NULL. */
/* This is equivalent to supplying an encoding argument to
   XML_ParserCreate. On success XML_SetEncoding returns non-zero,
   zero otherwise.
   Note: Calling XML_SetEncoding after XML_Parse or XML_ParseBuffer
     has no effect and returns XML_Status::ERROR.
*/

impl XML_ParserStruct {
    pub fn setEncoding(&mut self, encodingName: Option<&[XML_Char]>) -> XML_Status {
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
        if encodingName.is_none() {
            /* No new encoding name */
            self.m_protocolEncodingName = None;
        } else {
            /* Copy the new encoding name into allocated memory */
            self.m_protocolEncodingName = copy_c_string(encodingName);
            if self.m_protocolEncodingName.is_none() {
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

    let encodingName = slice_from_c_string(encodingName);
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
    mut oldParser: Option<&XML_ParserStruct>,
    mut context: *const XML_Char,
    mut encodingName: *const XML_Char,
) -> XML_Parser {
    /* Validate the oldParser parameter before we pull everything out of it */
    let mut oldParser: &XML_ParserStruct = match oldParser {
        Some(parser) => parser,
        None => return ptr::null_mut(),
    };
    let newDtd = if context.is_null() {
        Rc::clone(&oldParser.m_dtd)
    } else {
        match oldParser.m_dtd.try_clone().and_then(Rc::try_new) {
            Ok(newDtd) => newDtd,
            Err(_) => return ptr::null_mut(),
        }
    };
    /* XML_DTD */
    /* Note that the magical uses of the pre-processor to make field
       access look more like C++ require that `parser' be overwritten
       here.  This makes this function more painful to follow than it
       would be otherwise.
    */
    let encodingName = slice_from_c_string(encodingName);
    let mut parser = if oldParser.m_ns {
        let mut tmp: [XML_Char; 2] = [0; 2];
        *tmp.as_mut_ptr() = oldParser.m_namespaceSeparator;
        XML_ParserStruct::try_new(encodingName, tmp.as_mut_ptr(), newDtd)
    } else {
        XML_ParserStruct::try_new(encodingName, ptr::null(), newDtd)
    };
    let mut parser = match parser {
        Ok(parser) => parser,
        Err(_) => return ptr::null_mut(),
    };
    (*parser)
        .m_handlers
        .setStartElement(oldParser.m_handlers.m_startElementHandler);
    (*parser)
        .m_handlers
        .setEndElement(oldParser.m_handlers.m_endElementHandler);
    (*parser)
        .m_handlers
        .setCharacterData(oldParser.m_handlers.m_characterDataHandler);
    (*parser)
        .m_handlers
        .setProcessingInstruction(oldParser.m_handlers.m_processingInstructionHandler);
    (*parser)
        .m_handlers
        .setComment(oldParser.m_handlers.m_commentHandler);
    (*parser)
        .m_handlers
        .setStartCDataSection(oldParser.m_handlers.m_startCdataSectionHandler);
    (*parser)
        .m_handlers
        .setEndCDataSection(oldParser.m_handlers.m_endCdataSectionHandler);
    (*parser)
        .m_handlers
        .setDefault(oldParser.m_handlers.m_defaultHandler);
    (*parser)
        .m_handlers
        .setUnparsedEntityDecl(oldParser.m_handlers.m_unparsedEntityDeclHandler);
    (*parser)
        .m_handlers
        .setNotationDecl(oldParser.m_handlers.m_notationDeclHandler);
    (*parser)
        .m_handlers
        .setStartNamespaceDecl(oldParser.m_handlers.m_startNamespaceDeclHandler);
    (*parser)
        .m_handlers
        .setEndNamespaceDecl(oldParser.m_handlers.m_endNamespaceDeclHandler);
    (*parser)
        .m_handlers
        .setNotStandalone(oldParser.m_handlers.m_notStandaloneHandler);
    (*parser)
        .m_handlers
        .setExternalEntityRef(oldParser.m_handlers.m_externalEntityRefHandler);
    (*parser)
        .m_handlers
        .setSkippedEntity(oldParser.m_handlers.m_skippedEntityHandler);
    (*parser)
        .m_handlers
        .setUnknownEncoding(oldParser.m_handlers.m_unknownEncodingHandler);
    (*parser)
        .m_handlers
        .setElementDecl(oldParser.m_handlers.m_elementDeclHandler);
    (*parser)
        .m_handlers
        .setAttlistDecl(oldParser.m_handlers.m_attlistDeclHandler);
    (*parser)
        .m_handlers
        .setEntityDecl(oldParser.m_handlers.m_entityDeclHandler);
    (*parser)
        .m_handlers
        .setXmlDecl(oldParser.m_handlers.m_xmlDeclHandler);
    (*parser).m_declElementType = oldParser.m_declElementType.as_ref().map(Rc::clone);
    (*parser).m_userData = oldParser.m_userData;
    if oldParser.m_userData == oldParser.m_handlers.m_handlerArg {
        (*parser).m_handlers.m_handlerArg = (*parser).m_userData;
    } else {
        (*parser).m_handlers.m_handlerArg = parser.as_mut() as *mut _ as *mut _;
    }
    if oldParser.m_handlers.m_externalEntityRefHandlerArg != oldParser as *const _ as *mut _ {
        (*parser).m_handlers.m_externalEntityRefHandlerArg =
            oldParser.m_handlers.m_externalEntityRefHandlerArg;
    }
    (*parser).m_defaultExpandInternalEntities = oldParser.m_defaultExpandInternalEntities;
    (*parser).m_ns_triplets = oldParser.m_ns_triplets;
    (*parser).is_child_parser = true;
    (*parser).m_paramEntityParsing = oldParser.m_paramEntityParsing;
    (*parser).m_prologState.inEntityValue = oldParser.m_prologState.inEntityValue;
    if !context.is_null() {
        /* XML_DTD */
        if !(*parser).setContext(context) {
            return ptr::null_mut();
        }
        (*parser).m_processor = Some(externalEntityInitProcessor as Processor);
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
        (*parser).m_processor = Some(externalParEntInitProcessor as Processor);
    }
    /* XML_DTD */
    Box::into_raw(parser)
}

impl Drop for XML_ParserStruct {
    /* Frees memory used by the parser. */
    fn drop(&mut self) {
        unsafe {
            /* external parameter entity parsers share the DTD structure
            parser->m_dtd with the root parser, so we must not destroy it
            */
            if self.m_handlers.m_unknownEncodingRelease.is_some() {
                self.m_handlers
                    .m_unknownEncodingRelease
                    .expect("non-null function pointer")(
                    self.m_handlers.m_unknownEncodingData
                );
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
        if !(*parser).m_dtd.pool.copy_c_string(p) {
            return XML_Status::ERROR;
        }
        (*parser).m_curBase = (*parser).m_dtd.pool.finish().as_ptr();
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
pub unsafe extern "C" fn XML_SetHashSalt(_: XML_Parser, _: c_ulong) -> c_int {
    // FIXME
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

impl XML_ParserStruct {
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
                if !self.is_child_parser && !self.startParsing() {
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
            self.m_positionIdx = Some(self.m_bufferStart);
            self.m_parseEndIdx = self.m_bufferEnd;
            /* If data are left over from last buffer, and we now know that these
            data are the final chunk of input, then we have to check them again
            to detect errors based on that fact.
             */
            let mut start_ptr = self.m_buffer.as_ptr().add(self.m_bufferStart);
            self.m_errorCode = self.m_processor.expect("non-null function pointer")(
                self,
                RcRef::new(Rc::clone(&self.m_buffer))
                    .map(|sl| &sl[self.m_bufferStart..self.m_bufferEnd])
                    .into(),
                &mut start_ptr,
            );
            self.m_bufferStart = start_ptr.wrapping_offset_from(self.m_buffer.as_ptr()) as usize;
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
                            self.m_buffer[self.m_positionIdx.unwrap()..self.m_bufferStart].into(),
                            &mut self.m_position,
                        );
                        self.m_positionIdx = Some(self.m_bufferStart);
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
            self.m_eventEndPtr.set(self.m_eventPtr.get());
            self.m_processor = Some(errorProcessor as Processor);
            XML_Status::ERROR
        } else {
            /* not defined XML_CONTEXT_BYTES */
            if let Some(buff) = self.getBuffer(len) {
                buff[..len as usize].copy_from_slice(std::slice::from_raw_parts(s, len as usize));
                self.parseBuffer(len, isFinal)
            } else {
                XML_Status::ERROR
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

impl XML_ParserStruct {
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
                if !self.is_child_parser && !self.startParsing() {
                    self.m_errorCode = XML_Error::NO_MEMORY;
                    return XML_Status::ERROR;
                }
            }
            _ => {}
        }
        /* fall through */
        self.m_parsingStatus.parsing = XML_Parsing::PARSING;
        // convert in-out parameter `start` from index to pointer
        // TODO(SJC): is signed overflow an issue here?
        start = self.m_buffer.as_ptr().add(self.m_bufferStart);
        self.m_positionIdx = Some(self.m_bufferStart);
        self.m_bufferEnd += len as usize;
        self.m_parseEndIdx = self.m_bufferEnd;
        self.m_parseEndByteIndex += len as usize;
        self.m_parsingStatus.finalBuffer = isFinal != 0;
        self.m_errorCode = self.m_processor.expect("non-null function pointer")(
            self,
            RcRef::new(Rc::clone(&self.m_buffer))
                .map(|sl| &sl[self.m_bufferStart..self.m_bufferEnd])
                .into(),
            &mut start,
        );
        // convert in-out parameter `start` from pointer back to index
        self.m_bufferStart = start.wrapping_offset_from(self.m_buffer.as_ptr()) as usize;
        if self.m_errorCode != XML_Error::NONE {
            self.m_eventEndPtr.set(self.m_eventPtr.get());
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
            self.m_buffer[self.m_positionIdx.unwrap()..self.m_bufferStart].into(),
            &mut self.m_position,
        );
        self.m_positionIdx = Some(self.m_bufferStart);
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


impl  XML_ParserStruct {
    pub unsafe fn getBuffer(&mut self, len: c_int) -> Option<&mut [c_char]> {
        if len < 0 {
            self.m_errorCode = XML_Error::NO_MEMORY;
            return None;
        }
        match self.m_parsingStatus.parsing {
            XML_Parsing::SUSPENDED => {
                self.m_errorCode = XML_Error::SUSPENDED;
                return None;
            }
            XML_Parsing::FINISHED => {
                self.m_errorCode = XML_Error::FINISHED;
                return None;
            }
            _ => {}
        }
        let mut buffer = Rc::get_mut(&mut self.m_buffer).unwrap();
        if len as usize > buffer.len() - self.m_bufferEnd {
            let maybe_needed_size = len.checked_add((self.m_bufferEnd - self.m_bufferStart).try_into().unwrap());
            let mut neededSize = match maybe_needed_size {
                None => {
                    self.m_errorCode = XML_Error::NO_MEMORY;
                    return None;
                }
                Some(s) => s as usize,
            };
            let keep = cmp::min(
                XML_CONTEXT_BYTES as usize,
                self.m_bufferStart,
            );
            neededSize += keep;
            if neededSize <= buffer.len() {
                if keep < self.m_bufferStart {
                    let offset = self.m_bufferStart - keep as usize;
                    /* The buffer pointers cannot be NULL here; we have at least some bytes
                     * in the buffer */
                    buffer.copy_within(offset..self.m_bufferEnd, 0);
                    self.m_bufferEnd -= offset;
                    self.m_bufferStart -= offset;
                }
            } else {
                let mut bufferSize: c_int = match buffer.len() - self.m_bufferStart {
                    0 => INIT_BUFFER_SIZE,
                    size => size.try_into().unwrap(),
                };
                while (bufferSize as usize) < neededSize {
                    bufferSize = match 2i32.checked_mul(bufferSize) {
                        Some(s) => s,
                        None => {
                            self.m_errorCode = XML_Error::NO_MEMORY;
                            return None;
                        }
                    }
                }

                let bufferSize = bufferSize.try_into().unwrap();
                let additional = bufferSize - buffer.capacity();
                if buffer.try_reserve_exact(additional).is_err() {
                    self.m_errorCode = XML_Error::NO_MEMORY;
                    return None;
                }
                buffer.resize(bufferSize, 0);
                if self.m_bufferStart < self.m_bufferEnd {
                    buffer.copy_within(self.m_bufferStart-keep..self.m_bufferEnd, 0);
                    self.m_bufferEnd = self.m_bufferEnd - self.m_bufferStart + keep;
                    self.m_bufferStart = keep;
                } else {
                    /* This must be a brand new buffer with no data in it yet */
                    self.m_bufferStart = 0;
                    self.m_bufferEnd = 0;
                }
            }
            self.m_eventEndPtr.set(ptr::null());
            self.m_eventPtr.set(ptr::null());
            self.m_positionIdx = None;
        }
        Some(&mut buffer[self.m_bufferEnd..])
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetBuffer(mut parser: XML_Parser, mut len: c_int) -> *mut c_void {
    if parser.is_null() {
        return ptr::null_mut();
    }

    if let Some(buf) = (*parser).getBuffer(len) {
        buf.as_mut_ptr() as *mut c_void
    } else {
        ptr::null_mut()
    }
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

impl XML_ParserStruct {
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
impl XML_ParserStruct {
    pub unsafe fn resumeParser(&mut self) -> XML_Status {
        let mut result: XML_Status = XML_Status::OK;
        if self.m_parsingStatus.parsing != XML_Parsing::SUSPENDED {
            self.m_errorCode = XML_Error::NOT_SUSPENDED;
            return XML_Status::ERROR;
        }
        self.m_parsingStatus.parsing = XML_Parsing::PARSING;
        self.m_errorCode = self.m_processor.expect("non-null function pointer")(
            self,
            RcRef::new(Rc::clone(&self.m_buffer))
                .map(|sl| &sl[self.m_bufferStart..self.m_parseEndIdx])
                .into(),
            &mut (&self.m_buffer[self.m_bufferStart] as *const _),
        );
        if self.m_errorCode != XML_Error::NONE {
            self.m_eventEndPtr.set(self.m_eventPtr.get());
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
            self.m_buffer[self.m_positionIdx.unwrap()..self.m_bufferStart].into(),
            &mut self.m_position,
        );
        self.m_positionIdx = Some(self.m_bufferStart);

        #[cfg(feature = "mozilla")]
        {
            self.m_eventPtr.set(&self.m_buffer[self.m_bufferStart]);
            self.m_eventEndPtr.set(&self.m_buffer[self.m_bufferStart]);
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
    } else {
        XML_Error::INVALID_ARGUMENT
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteIndex(mut parser: XML_Parser) -> XML_Index {
    if parser.is_null() {
        return -1;
    }
    if !(*parser).m_eventPtr.get().is_null() {
        return ((*parser).m_parseEndByteIndex - ((*parser).m_parseEndIdx - (*parser).buffer_index((*parser).m_eventPtr.get()).unwrap())) as XML_Index;
    }
    if cfg!(feature = "mozilla") {
        return (*parser).m_parseEndByteIndex as XML_Index;
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
    if !(*parser).m_eventEndPtr.get().is_null() && !(*parser).m_eventPtr.get().is_null() {
        return (*parser)
            .m_eventEndPtr
            .get()
            .wrapping_offset_from((*parser).m_eventPtr.get()) as c_int;
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
    if !(*parser).m_eventPtr.get().is_null() {
        if !offset.is_null() {
            *offset = (*parser)
                .m_eventPtr
                .get()
                .wrapping_offset_from((*parser).m_buffer.as_ptr()) as c_int
        }
        if !size.is_null() {
            *size = (*parser).m_bufferEnd.try_into().unwrap();
        }
        return (*parser).m_buffer.as_ptr();
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
    let positionPtr = (*parser).m_buffer.as_ptr().wrapping_add((*parser).m_positionIdx.unwrap());
    if !(*parser).m_eventPtr.get().is_null() && (*parser).m_eventPtr.get() >= positionPtr {
        (*(*parser).m_encoding).updatePosition(
            ExpatBufRef::new(positionPtr, (*parser).m_eventPtr.get()),
            &mut (*parser).m_position,
        );
        (*parser).m_positionIdx = Some(
            (*parser).m_eventPtr.get().wrapping_offset_from((*parser).m_buffer.as_ptr()) as usize
        );
    }
    (*parser).m_position.lineNumber.wrapping_add(1)
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentColumnNumber(mut parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0;
    }
    let positionPtr = (*parser).m_buffer.as_ptr().wrapping_add((*parser).m_positionIdx.unwrap());
    if !(*parser).m_eventPtr.get().is_null() && (*parser).m_eventPtr.get() >= positionPtr {
        (*(*parser).m_encoding).updatePosition(
            ExpatBufRef::new(positionPtr, (*parser).m_eventPtr.get()),
            &mut (*parser).m_position,
        );
        (*parser).m_positionIdx = Some(
            (*parser).m_eventPtr.get().wrapping_offset_from((*parser).m_buffer.as_ptr()) as usize
        );
    }
    (*parser).m_position.columnNumber
}
/* Frees the content model passed to the element declaration handler */
#[no_mangle]
pub unsafe extern "C" fn XML_FreeContentModel(mut parser: XML_Parser, mut model: *mut XML_Content) {
    if !parser.is_null() {
        let _ = Box::from_raw(model as *mut ContentModel);
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
        if let Some(openEntity) = (*parser).m_openInternalEntities.as_mut() {
            reportDefault(
                parser,
                EncodingType::Internal,
                ExpatBufRef::new(
                    openEntity.internalEventPtr.get(),
                    openEntity.internalEventEndPtr.get(),
                ),
            );
        } else {
            reportDefault(
                parser,
                EncodingType::Normal,
                ExpatBufRef::new((*parser).m_eventPtr.get(), (*parser).m_eventEndPtr.get()),
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
            XML_Error::MISPLACED_XML_PI => {
                wch!("XML or text declaration not at start of entity\x00")
            }
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
    !(*parser).m_openInternalEntities.is_none()
}

/* Initially tag->rawName always points into the parse buffer;
   for those TAG instances opened while the current parse buffer was
   processed, and not yet closed, we need to store tag->rawName in a more
   permanent location, since the parse buffer is about to be discarded.
*/
impl XML_ParserStruct {
    unsafe fn storeRawNames(&mut self) -> XML_Bool {
        let mut tStk = &mut self.m_tagStack;
        while let Some(tag) = tStk {
            /* Stop if already stored.  Since m_tagStack is a stack, we can stop
            at the first entry that has already been copied; everything
            below it in the stack is already been accounted for in a
            previous call to this function.
            */
            if !tag.rawNameBuf.is_empty() {
                break;
            }

            let mut raw_name_buf = Rc::get_mut(&mut tag.rawNameBuf).unwrap();
            let raw_name_length = tag.rawNameLength.try_into().unwrap();
            if raw_name_buf.try_reserve(raw_name_length).is_err() {
                return false;
            }
            raw_name_buf.extend_from_slice(slice::from_raw_parts(tag.rawName, raw_name_length));
            tag.rawName = raw_name_buf.as_ptr();
            tStk = &mut tag.parent;
        }
        true
    }
}

unsafe extern "C" fn contentProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatSliceRc,
    mut endPtr: &mut *const c_char,
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
    mut buf: ExpatSliceRc,
    mut endPtr: &mut *const c_char,
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
    mut buf: ExpatSliceRc,
    mut endPtr: &mut *const c_char,
) -> XML_Error {
    let mut next_idx = 0;
    let mut tok = (*(*parser).m_encoding).xmlTok(XML_STATE::CONTENT, buf.as_buf_ref(), &mut next_idx);
    let mut next = buf.clone().inc_start(next_idx).as_ptr();
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
            (*parser).m_eventPtr.set(buf.as_ptr());
            return XML_Error::UNCLOSED_TOKEN;
        }
        XML_TOK::PARTIAL_CHAR => {
            if !(*parser).m_parsingStatus.finalBuffer {
                *endPtr = buf.as_ptr();
                return XML_Error::NONE;
            }
            (*parser).m_eventPtr.set(buf.as_ptr());
            return XML_Error::PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(externalEntityInitProcessor3 as Processor);
    externalEntityInitProcessor3(parser, buf, endPtr)
}

unsafe extern "C" fn externalEntityInitProcessor3(
    mut parser: XML_Parser,
    mut buf: ExpatSliceRc,
    mut endPtr: &mut *const c_char,
) -> XML_Error {
    let mut next_idx = 0;
    (*parser).m_eventPtr.set(buf.as_ptr());
    let tok = (*(*parser).m_encoding).xmlTok(XML_STATE::CONTENT, buf.as_buf_ref(), &mut next_idx);
    let mut next = buf.clone().inc_start(next_idx).as_ptr();
    (*parser).m_eventEndPtr.set(next);
    match tok {
        XML_TOK::XML_DECL => {
            let mut result: XML_Error = XML_Error::NONE;
            result = (*parser).processXmlDecl(1, buf.clone().with_len(next_idx));
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
    mut buf: ExpatSliceRc,
    mut endPtr: &mut *const c_char,
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


impl XML_ParserStruct {
    #[inline]
    fn eventPP(&self, enc_type: EncodingType) -> &Cell<*const c_char> {
        if enc_type.is_internal() {
            let openEntity = self.m_openInternalEntities.as_deref().unwrap();
            &openEntity.internalEventPtr
        } else {
            &self.m_eventPtr
        }
    }

    #[inline]
    fn eventEndPP(&self, enc_type: EncodingType) -> &Cell<*const c_char> {
        if enc_type.is_internal() {
            let openEntity = self.m_openInternalEntities.as_deref().unwrap();
            &openEntity.internalEventEndPtr
        } else {
            &self.m_eventEndPtr
        }
    }

    unsafe fn doContent(
        &mut self,
        startTagLevel: c_int,
        enc_type: EncodingType,
        mut buf: ExpatSliceRc,
        nextPtr: &mut *const c_char,
        haveMore: XML_Bool,
    ) -> XML_Error {
        /* XmlContentTok doesn't always set the last arg */
        let enc = self.encoding(enc_type);
        self.eventPP(enc_type).set(buf.as_ptr());
        loop {
            let mut next_idx = 0;
            let mut tok = (*enc).xmlTok(XML_STATE::CONTENT, buf.as_buf_ref(), &mut next_idx);
            let mut next = buf.clone().inc_start(next_idx).as_ptr();
            self.eventEndPP(enc_type).set(next);
            match tok {
                XML_TOK::TRAILING_CR => {
                    if haveMore {
                        *nextPtr = buf.as_ptr();
                        return XML_Error::NONE;
                    }
                    self.eventEndPP(enc_type)
                        .set(buf.as_ptr().offset(buf.len().try_into().unwrap()));
                    if self.m_handlers.hasCharacterData() {
                        let mut c: XML_Char = 0xa;
                        self.m_handlers.characterData(&[c]);
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.as_buf_ref());
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
                    self.eventPP(enc_type).set(next);
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
                        buf.as_buf_ref()
                            .inc_start((*enc).minBytesPerChar())
                            .with_end(next)
                            .dec_end((*enc).minBytesPerChar()),
                    ) as XML_Char;
                    if ch != 0 {
                        let hasCharacterData = self.m_handlers.characterData(&[ch]);

                        if !hasCharacterData && self.m_handlers.hasDefault() {
                            reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                        }
                    } else {
                        let successful = self.m_dtd.pool.store_c_string(
                            enc,
                            buf.as_buf_ref()
                                .inc_start((*enc).minBytesPerChar())
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar()),
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        };
                        let entity = self.m_dtd.pool.with_current_slice(|key| {
                            let mut dtd_tables = self.m_dtd.tables.borrow_mut();
                            dtd_tables.generalEntities.get(key).map(Rc::clone)
                        });

                        /* First, determine if a check for an existing declaration is needed;
                           if yes, check that the entity exists, and that it is internal,
                           otherwise call the skipped entity or default handler.
                        */
                        let mut skipHandlers = false;
                        // REXPAT: Moved `clear_current` from outside the following branches
                        // into them to avoid a use-after-free when the 2nd branch uses the
                        // `name` pointer again.
                        if !self.m_dtd.hasParamEntityRefs.get() || self.m_dtd.standalone.get() {
                            self.m_dtd.pool.discard();

                            if entity.is_none() {
                                return XML_Error::UNDEFINED_ENTITY;
                            } else {
                                if !entity.as_deref().unwrap().is_internal.get() {
                                    return XML_Error::ENTITY_DECLARED_IN_PE;
                                }
                            }
                        } else if entity.is_none() {
                            let skippedHandlerRan = self.m_dtd.pool.with_current_slice(|name| {
                                self.m_handlers.skippedEntity(name.as_ptr(), 0)
                            });

                            self.m_dtd.pool.discard();

                            if !skippedHandlerRan && self.m_handlers.hasDefault() {
                                if !cfg!(feature = "mozilla") {
                                    reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                                }
                            }
                            if cfg!(feature = "mozilla") {
                                return XML_Error::UNDEFINED_ENTITY;
                            } else {
                                skipHandlers = true;
                            }
                        } else {
                            self.m_dtd.pool.discard();
                        }

                        if !skipHandlers {
                            let entity = entity.as_ref().unwrap();
                            if entity.open.get() {
                                return XML_Error::RECURSIVE_ENTITY_REF;
                            }
                            if !entity.notation.get().is_null() {
                                return XML_Error::BINARY_ENTITY_REF;
                            }
                            if !entity.textPtr.borrow().is_none() {
                                let mut result: XML_Error = XML_Error::NONE;
                                if !self.m_defaultExpandInternalEntities {
                                    let skippedHandlerRan = self.m_handlers.skippedEntity(
                                        entity.name.borrow().as_ref().unwrap().as_ptr(),
                                        0,
                                    );

                                    if !skippedHandlerRan && self.m_handlers.hasDefault() {
                                        reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                                    }
                                } else {
                                    result = self.processInternalEntity(entity, false);
                                    if result != XML_Error::NONE {
                                        return result;
                                    }
                                }
                            } else if self.m_handlers.hasExternalEntityRef() {
                                entity.open.set(true);
                                // Avoiding borrowing issues by separating out
                                // the fields here:
                                let context = self.getContext();
                                entity.open.set(false);
                                if !context {
                                    return XML_Error::NO_MEMORY;
                                }
                                if self.m_tempPool.with_current_slice(|name| {
                                    self.m_handlers.externalEntityRef(
                                        name.as_ptr(),
                                        entity.base.get(),
                                        entity.systemId.get(),
                                        entity.publicId.get(),
                                    )
                                }) == Ok(0)
                                {
                                    return XML_Error::EXTERNAL_ENTITY_HANDLING;
                                }
                                self.m_tempPool.discard();
                            } else if self.m_handlers.hasDefault() {
                                reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                            }
                        }
                    }
                }
                XML_TOK::START_TAG_NO_ATTS | XML_TOK::START_TAG_WITH_ATTS => {
                    /* fall through */
                    let mut result_0: XML_Error = XML_Error::NONE;
                    let mut to_buf: ExpatBufRefMut<XML_Char>;
                    let mut tag = match self.m_freeTagList.take() {
                        Some(mut tag) => {
                            self.m_freeTagList = tag.parent.take();

                            // Clear the raw name buffer to mark that it's not in use
                            Rc::get_mut(&mut tag.rawNameBuf).unwrap().clear();

                            tag
                        }
                        None => {
                            let mut tag = match Tag::try_new().and_then(|x| Box::try_new(x).map_err(|_| ())) {
                                Ok(tag) => tag,
                                Err(_) => return XML_Error::NO_MEMORY,
                            };

                            tag
                        }
                    };
                    tag.bindings = None;
                    tag.name.localPart = TagNameLocalPart::None;
                    tag.name.prefix = ptr::null();
                    let mut fromBuf = buf.as_buf_ref().inc_start((*enc).minBytesPerChar());
                    tag.rawName = fromBuf.as_ptr();
                    tag.rawNameLength = (*enc).nameLength(fromBuf);
                    fromBuf = fromBuf.with_len(tag.rawNameLength as usize);
                    self.m_tagLevel += 1;

                    let mut tag_buf = Rc::get_mut(&mut tag.buf).unwrap();
                    let mut tag_buf_len = tag_buf.capacity();
                    tag_buf.resize(tag_buf_len, 0);
                    let mut convLen = 0;
                    loop {
                        let mut bufSize: c_int = 0;
                        let (convert_res, from_count, to_count)  =
                            XmlConvert!(enc, fromBuf, &mut tag_buf[convLen..tag_buf_len - 1]);
                        fromBuf = fromBuf.inc_start(from_count);
                        convLen += to_count;
                        if fromBuf.is_empty()
                            || convert_res == super::xmltok::XML_Convert_Result::INPUT_INCOMPLETE
                        {
                            tag.name.strLen = convLen.try_into().unwrap();
                            break;
                        } else {
                            if tag_buf.try_reserve(tag_buf.capacity()).is_err() {
                                return XML_Error::NO_MEMORY;
                            }
                            tag_buf_len = tag_buf.capacity();
                            tag_buf.resize(tag_buf_len, 0);
                        }
                    }
                    tag_buf[convLen] = '\u{0}' as XML_Char;
                    tag_buf.truncate(convLen + 1);
                    tag.name.str_0 = TagNameString::Ptr(tag_buf.as_ptr());
                    result_0 = self.storeAtts(enc_type, buf.as_buf_ref(), &mut tag.name, &mut tag.bindings);
                    if result_0 as u64 != 0 {
                        return result_0;
                    }

                    let handlers = &self.m_handlers;
                    let started = handlers.startElement(tag.name.str_0.as_ptr(), &mut self.m_atts);

                    if !started && handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                    }

                    self.m_nsAtts.clear();
                    self.m_tempPool.clear();

                    // FIXME: do we need to update m_tagStack when returning an error?
                    tag.parent = self.m_tagStack.take();
                    self.m_tagStack = Some(tag);
                }
                XML_TOK::EMPTY_ELEMENT_NO_ATTS | XML_TOK::EMPTY_ELEMENT_WITH_ATTS => {
                    /* fall through */
                    let mut rawName = buf.as_buf_ref().inc_start((*enc).minBytesPerChar());
                    let mut result_1: XML_Error = XML_Error::NONE;
                    let mut bindings = None;
                    let mut noElmHandlers = true;

                    let successful = self.m_tempPool.store_c_string(
                        enc,
                        rawName.with_len((*enc).nameLength(rawName) as usize),
                    );
                    if !successful {
                        return XML_Error::NO_MEMORY;
                    }
                    let name_0_str_0 = self.m_tempPool.finish().as_ptr();

                    let mut name_0 = TagName {
                        str_0: TagNameString::Ptr(name_0_str_0),
                        localPart: TagNameLocalPart::None,
                        prefix: ptr::null(),
                        strLen: 0,
                        uriLen: 0,
                        prefixLen: 0,
                    };
                    result_1 = self.storeAtts(enc_type, buf.as_buf_ref(), &mut name_0, &mut bindings);
                    if result_1 != XML_Error::NONE {
                        self.freeBindings(bindings);
                        return result_1;
                    }
                    self.m_tempPool.finish();
                    let handlers = &self.m_handlers;
                    let started = handlers.startElement(name_0.str_0.as_ptr(), &mut self.m_atts);
                    if started {
                        noElmHandlers = false
                    }
                    if self.m_handlers.hasEndElement() {
                        if started {
                            self.eventPP(enc_type).set(self.eventEndPP(enc_type).get());
                        }

                        self.m_handlers.endElement(name_0.str_0.as_ptr());

                        noElmHandlers = false
                    }
                    if noElmHandlers && self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                    }

                    self.m_nsAtts.clear();
                    self.m_tempPool.clear();
                    self.freeBindings(bindings);
                    if self.m_tagLevel == 0 && self.m_parsingStatus.parsing != XML_Parsing::FINISHED
                    {
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

                        let rawName_0 = buf.as_buf_ref().inc_start((*enc).minBytesPerChar() * 2);
                        let len = (*enc).nameLength(rawName_0);
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
                                let mut localPart = tag.name.localPart.as_ptr();
                                let mut prefix: *const XML_Char = ptr::null();
                                let mut uri: *mut XML_Char = ptr::null_mut();
                                if self.m_ns && !localPart.is_null() {
                                    /* localPart and prefix may have been overwritten in
                                       tag->name.str, since this points to the binding->uri
                                       buffer which gets re-used; so we have to add them again
                                    */
                                    uri = (tag.name.str_0.as_ptr() as *mut XML_Char)
                                        .add(tag.name.uriLen.get());
                                    /* don't need to check for space - already done in storeAtts() */
                                    while *localPart != 0 {
                                        *uri = *localPart;
                                        localPart = localPart.offset(1);
                                        uri = uri.offset(1);
                                    }
                                    prefix = tag.name.prefix as *mut XML_Char;
                                    if self.m_ns_triplets && !prefix.is_null() {
                                        *uri = self.m_namespaceSeparator;
                                        uri = uri.offset(1);
                                        while *prefix != 0 {
                                            *uri = *prefix;
                                            prefix = prefix.offset(1);
                                            uri = uri.offset(1);
                                        }
                                    }
                                    *uri = '\u{0}' as XML_Char
                                }
                                self.m_mismatch = tag.name.str_0.as_ptr();
                            }
                            self.eventPP(enc_type).set(rawName_0.as_ptr());
                            return XML_Error::TAG_MISMATCH;
                        }
                        self.m_tagLevel -= 1;
                        if self.m_handlers.hasEndElement() {
                            let mut localPart = tag.name.localPart.as_ptr();
                            let mut prefix: *const XML_Char = ptr::null();
                            let mut uri: *mut XML_Char = ptr::null_mut();
                            if self.m_ns && !localPart.is_null() {
                                /* localPart and prefix may have been overwritten in
                                   tag->name.str, since this points to the binding->uri
                                   buffer which gets re-used; so we have to add them again
                                */
                                uri =
                                    (tag.name.str_0.as_ptr() as *mut XML_Char).add(tag.name.uriLen);
                                /* don't need to check for space - already done in storeAtts() */
                                while *localPart != 0 {
                                    *uri = *localPart;
                                    localPart = localPart.offset(1);
                                    uri = uri.offset(1);
                                }
                                prefix = tag.name.prefix as *mut XML_Char;
                                if self.m_ns_triplets && !prefix.is_null() {
                                    *uri = self.m_namespaceSeparator;
                                    uri = uri.offset(1);
                                    while *prefix != 0 {
                                        *uri = *prefix;
                                        prefix = prefix.offset(1);
                                        uri = uri.offset(1);
                                    }
                                }
                                *uri = '\u{0}' as XML_Char
                            }

                            self.m_handlers.endElement(tag.name.str_0.as_ptr());
                        } else if self.m_handlers.hasDefault() {
                            reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                        }

                        {
                            let mut fbl = self.m_freeBindingList.borrow_mut();
                            while let Some(b) = tag.bindings.take() {
                                let prefix = b.prefix.upgrade().unwrap();
                                self.m_handlers.endNamespaceDecl(
                                    prefix.name.as_ref().map_or_else(ptr::null, |x| x.as_ptr()),
                                );
                                prefix.binding.set(b.prevPrefixBinding.take());
                                // Release the Rc's inside attId
                                b.attId.set(None);
                                tag.bindings = b.nextTagBinding.replace(fbl.take());
                                *fbl = Some(b);
                            }
                        }

                        // Clear `str_0` and `localPart` to reduce
                        // their pointee's reference counts
                        tag.name.str_0 = TagNameString::None;
                        tag.name.localPart = TagNameLocalPart::None;

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
                    let mut n: c_int = (*enc).charRefNumber(buf.as_buf_ref());
                    if n < 0 {
                        return XML_Error::BAD_CHAR_REF;
                    }
                    if self.m_handlers.hasCharacterData() {
                        let mut out_buf: [XML_Char; XML_ENCODE_MAX] = [0; XML_ENCODE_MAX];
                        let n = XmlEncode(n, &mut out_buf) as usize;
                        self.m_handlers.characterData(&out_buf[..n]);
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                    }
                }
                XML_TOK::XML_DECL => return XML_Error::MISPLACED_XML_PI,
                XML_TOK::DATA_NEWLINE => {
                    if self.m_handlers.hasCharacterData() {
                        let mut c_0: XML_Char = 0xa;
                        self.m_handlers.characterData(&[c_0]);
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
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
                        reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                    }
                    let mut new_buf = Some(buf.clone().with_start(next));
                    result_2 = doCdataSection(self, enc_type, &mut new_buf, nextPtr, haveMore);
                    next = new_buf.map_or_else(ptr::null, |x| x.as_ptr());
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
                            let mut bufRef = buf.as_buf_ref();
                            let (_, buf_count, data_count) =
                                XmlConvert!(enc, bufRef, &mut self.m_dataBuf[..]);
                            buf = buf.clone().inc_start(buf_count);
                            self.m_handlers
                                .characterData(&ExpatBufRef::from(&self.m_dataBuf[..data_count]));
                        } else {
                            self.m_handlers
                                .characterData(&ExpatBufRef::<XML_Char>::from(buf.as_buf_ref()));
                        }
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.as_buf_ref());
                    }
                    /* We are at the end of the final buffer, should we check for
                       XML_Parsing::SUSPENDED, XML_Parsing::FINISHED?
                    */
                    if startTagLevel == 0 {
                        self.eventPP(enc_type).set(buf.end());
                        return XML_Error::NO_ELEMENTS;
                    }
                    if self.m_tagLevel != startTagLevel {
                        self.eventPP(enc_type).set(buf.end());
                        return XML_Error::ASYNC_ENTITY;
                    }
                    *nextPtr = buf.end();
                    return XML_Error::NONE;
                }
                XML_TOK::DATA_CHARS => {
                    if self.m_handlers.hasCharacterData() {
                        if MUST_CONVERT!(enc, buf.as_ptr()) {
                            loop {
                                let mut from_buf = buf.as_buf_ref().with_end(next);
                                let (convert_res_0, buf_count, data_count) =
                                    XmlConvert!(enc, from_buf, &mut self.m_dataBuf[..]);
                                buf = buf.clone().inc_start(buf_count);

                                let data_buf = ExpatBufRef::from(&self.m_dataBuf[..data_count]);
                                self.eventEndPP(enc_type).set(buf.as_ptr());
                                self.m_handlers.characterData(&data_buf);
                                if convert_res_0 == super::xmltok::XML_Convert_Result::COMPLETED
                                    || convert_res_0
                                        == super::xmltok::XML_Convert_Result::INPUT_INCOMPLETE
                                {
                                    break;
                                }
                                self.eventPP(enc_type).set(buf.as_ptr());
                            }
                        } else {
                            let data_buf: ExpatBufRef<XML_Char> = buf.as_buf_ref().with_end(next).into();
                            self.m_handlers.characterData(&data_buf);
                        }
                    } else if self.m_handlers.hasDefault() {
                        reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                    }
                }
                XML_TOK::PI => {
                    if reportProcessingInstruction(self, enc_type, buf.as_buf_ref().with_end(next)) == 0 {
                        return XML_Error::NO_MEMORY;
                    }
                }
                XML_TOK::COMMENT => {
                    if reportComment(self, enc_type, buf.as_buf_ref().with_end(next)) == 0 {
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
                        reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                    }
                }
            }
            buf = buf.with_start(next);
            self.eventPP(enc_type).set(buf.as_ptr());
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

    fn freeBindings(&mut self, mut bindings: Ptr<Binding>) {
        let mut fbl = self.m_freeBindingList.borrow_mut();
        while let Some(b) = bindings {
            /* m_startNamespaceDeclHandler will have been called for this
             * binding in addBindings(), so call the end handler now.
             */
            let prefix = b.prefix.upgrade().unwrap();
            self.m_handlers
                .endNamespaceDecl(prefix.name.as_ref().map_or_else(ptr::null, |x| x.as_ptr()));
            prefix.binding.set(b.prevPrefixBinding.take());
            // Release the Rc's inside attId
            b.attId.set(None);
            bindings = b.nextTagBinding.replace(fbl.take());
            *fbl = Some(b);
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
        tagNamePtr: &mut TagName,
        bindingsPtr: &mut Ptr<Binding>,
    ) -> XML_Error {
        let mut nPrefixes: c_int = 0;
        let mut nXMLNSDeclarations: c_int = 0;
        let enc = self.encoding(enc_type);
        /* lookup the element type name */
        let elementType = {
            let mut dtd_tables = self.m_dtd.tables.borrow_mut();
            let elementType = tagNamePtr
                .str_0
                .with_slice(|tag_name| dtd_tables.elementTypes.get(tag_name).map(Rc::clone));
            if let Some(elementType) = elementType {
                elementType
            } else {
                if !self.m_dtd.pool.copy_c_string(tagNamePtr.str_0.as_ptr()) {
                    return XML_Error::NO_MEMORY;
                }
                let elementType: Rc<ElementType> =
                    match hash_insert_pool!(dtd_tables.elementTypes, &self.m_dtd.pool, ElementType)
                        .cloned()
                        .into()
                    {
                        Some(elementType) => elementType,
                        None => return XML_Error::NO_MEMORY,
                    };

                // Drop the `RefCell` borrows manually before `setElementTypePrefix`,
                // so `self` is released
                drop(dtd_tables);

                if self.m_ns as c_int != 0 && self.setElementTypePrefix(&elementType) == 0 {
                    return XML_Error::NO_MEMORY;
                }

                elementType
            }
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
            // REXPAT: we assume that the name of the attribute will end
            // somewhere before the value, so we use the start of the latter
            // as the upper bound for nameLength
            let att_name_buf = ExpatBufRef::new(currAtt.name, currAtt.valuePtr);
            let mut attId = match self.getAttributeId(
                enc_type,
                att_name_buf.with_len((*enc).nameLength(att_name_buf) as usize),
            ) {
                Some(attId) => attId,
                None => return XML_Error::NO_MEMORY,
            };
            /* Detect duplicate attributes by their QNames. This does not work when
            namespace processing is turned on and different prefixes for the same
            namespace are used. For this case we have a check further down.
            */
            if attId.get_type().is_set() {
                if !enc_type.is_internal() {
                    self.m_eventPtr.set(currAtt.name);
                }
                return XML_Error::DUPLICATE_ATTRIBUTE;
            }
            attId.set_type(AttributeType::Normal);
            if !currAtt.normalized {
                let mut result: XML_Error = XML_Error::NONE;
                let mut isCdata = true;
                /* figure out whether declared as other than CDATA */
                if attId.maybeTokenized.get() {
                    isCdata = elementType
                        .defaultAtts
                        .borrow()
                        .iter()
                        .find(|da| Rc::ptr_eq(&attId, &da.id))
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
                self.m_atts.push(Attribute::new(
                    attId.name().as_ptr(),
                    self.m_tempPool.finish().as_ptr(),
                ));
            } else {
                /* the value did not need normalizing */
                if !self
                    .m_tempPool
                    .store_c_string(enc, ExpatBufRef::new(currAtt.valuePtr, currAtt.valueEnd))
                {
                    return XML_Error::NO_MEMORY;
                }
                self.m_atts.push(Attribute::new(
                    attId.name().as_ptr(),
                    self.m_tempPool.finish().as_ptr(),
                ));
            }
            /* handle prefixed attribute names */
            if let Some(prefix) = attId.prefix.borrow().as_ref().and_then(Weak::upgrade) {
                if attId.xmlns.get() {
                    /* deal with namespace declarations here */
                    let mut result_0: XML_Error = addBinding(
                        self,
                        &prefix,
                        Some(Rc::clone(&attId)),
                        self.m_atts.last().unwrap().value,
                        bindingsPtr,
                    );
                    if result_0 as u64 != 0 {
                        return result_0;
                    }
                    if cfg!(feature = "mozilla") {
                        nXMLNSDeclarations += 1;
                        attId.set_type(AttributeType::Namespace);
                        self.typed_atts.push(attId.typed_name());
                    } else {
                        // Mozilla code replaces `--attIndex` with `attIndex++`,
                        // the former being equivalent to popping the last
                        // attribute from m_atts
                        self.m_atts.pop();
                    }
                } else {
                    /* deal with other prefixed names later */
                    nPrefixes += 1;
                    attId.set_type(AttributeType::Prefixed);
                    self.typed_atts.push(attId.typed_name());
                }
            } else {
                self.typed_atts.push(attId.typed_name());
            }

            XML_Error::NONE
        });
        if res != XML_Error::NONE {
            return res;
        }
        assert!(self.m_atts.len() == self.typed_atts.len());

        /* set-up for XML_GetSpecifiedAttributeCount and XML_GetIdAttributeIndex */
        self.m_nSpecifiedAtts = 2 * self.m_atts.len() as c_int;
        self.m_idAttIndex = -1;
        if let Some(attId) = get_cell_ptr(&elementType.idAtt).as_deref() {
            if attId.get_type().is_set() {
                for i in 0..self.typed_atts.len() {
                    if self.typed_atts[i].name.as_ptr() == attId.name().as_ptr() {
                        self.m_idAttIndex = 2 * i as c_int;
                        break;
                    }
                }
            }
        }

        /* do attribute defaulting */
        if self
            .m_atts
            .try_reserve(elementType.defaultAtts.borrow().len())
            .is_err()
        {
            return XML_Error::NO_MEMORY;
        }
        if self
            .typed_atts
            .try_reserve(elementType.defaultAtts.borrow().len())
            .is_err()
        {
            return XML_Error::NO_MEMORY;
        }
        for da in elementType.defaultAtts.borrow().iter() {
            if !da.id.get_type().is_set() && !da.value.is_null() {
                if let Some(prefix) = da.id.prefix.borrow().as_ref().and_then(Weak::upgrade) {
                    if da.id.xmlns.get() {
                        let mut result_1: XML_Error = addBinding(
                            self,
                            &prefix,
                            Some(Rc::clone(&da.id)),
                            da.value,
                            bindingsPtr,
                        );
                        if result_1 as u64 != 0 {
                            return result_1;
                        }
                        #[cfg(feature = "mozilla")]
                        {
                            da.id.set_type(AttributeType::Namespace);
                            nXMLNSDeclarations += 1;
                            self.m_atts.push(Attribute::from_default(da));
                            self.typed_atts.push(da.id.typed_name());
                        }
                    } else {
                        da.id.set_type(AttributeType::Prefixed);
                        nPrefixes += 1;
                        self.m_atts.push(Attribute::from_default(da));
                        self.typed_atts.push(da.id.typed_name());
                    }
                } else {
                    da.id.set_type(AttributeType::Normal);
                    self.m_atts.push(Attribute::from_default(da));
                    self.typed_atts.push(da.id.typed_name());
                }
            }
        }
        assert!(self.m_atts.len() == self.typed_atts.len());

        /* expand prefixed attribute names, check for duplicates,
        and clear flags that say whether attributes were specified */
        let mut i = 0; /* hash table index */
        if nPrefixes != 0 || nXMLNSDeclarations != 0 {
            // MOZILLA CHANGE
            let mut dtd_tables = self.m_dtd.tables.borrow_mut();
            if nPrefixes != 0 {
                // MOZILLA CHANGE
                self.m_nsAtts.clear();
                /* size of hash table must be at least 2 * (# of prefixed attributes) */
                if self
                    .m_nsAtts
                    .try_reserve((nPrefixes as usize) << 1)
                    .is_err()
                {
                    return XML_Error::NO_MEMORY;
                }
            } // MOZILLA CHANGE
              /* expand prefixed names and check for duplicates */
            while i < self.m_atts.len() {
                let mut s: *const XML_Char = self.m_atts[i].name;
                if self.typed_atts[i].get_type() == AttributeType::Prefixed {
                    // TODO: this could be a match instead
                    /* clear flag */
                    /* not prefixed */
                    /* prefixed */
                    self.typed_atts[i].set_type(AttributeType::Unset); /* clear flag */
                    let key = slice::from_raw_parts(s, strlen(s) + 1);
                    let id = dtd_tables.attributeIds.get(key);
                    if id.is_none() || id.unwrap().prefix.borrow().is_none() {
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
                    let prefix = id.prefix.borrow().as_ref().and_then(Weak::upgrade).unwrap();
                    let b = match get_cell_ptr(&prefix.binding) {
                        Some(b) => b,
                        None => return XML_Error::UNBOUND_PREFIX,
                    };
                    let uri = b.uri.borrow();
                    for &c in &uri[..b.uriLen.get()] {
                        if !self.m_tempPool.append_char(c) {
                            return XML_Error::NO_MEMORY;
                        }
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
                    let ret = self.m_tempPool.with_current_slice(|name| {
                        if self.m_nsAtts.contains(name) {
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
                        if let Some(ref name) = b.prefix.upgrade().unwrap().name {
                            for &c in name.iter() {
                                if !self.m_tempPool.append_char(c) {
                                    return XML_Error::NO_MEMORY;
                                }
                                if c == 0 {
                                    break;
                                }
                            }
                        }
                    }
                    if ret != XML_Error::NONE {
                        return ret;
                    }
                    /* store expanded name in attribute list */
                    let name_slice = self.m_tempPool.finish();
                    self.m_atts[i].name = name_slice.as_ptr();
                    self.m_nsAtts.insert(name_slice);
                    nPrefixes -= 1;
                    if nPrefixes == 0 && nXMLNSDeclarations == 0 {
                        i += 1;
                        break;
                    }
                } else if cfg!(feature = "mozilla")
                    && self.typed_atts[i].get_type() == AttributeType::Namespace
                {
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
                        '\u{0}' as XML_Char,
                    ];
                    const xmlnsPrefix: [XML_Char; 6] = [
                        ASCII_x as XML_Char,
                        ASCII_m as XML_Char,
                        ASCII_l as XML_Char,
                        ASCII_n as XML_Char,
                        ASCII_s as XML_Char,
                        '\u{0}' as XML_Char,
                    ];

                    self.typed_atts[i].set_type(AttributeType::Unset); /* clear flag */
                    if !self.m_tempPool.append_c_string(xmlnsNamespace.as_ptr())
                        || !self.m_tempPool.append_char(self.m_namespaceSeparator)
                    {
                        return XML_Error::NO_MEMORY;
                    }

                    s = s.offset(xmlnsPrefix.len() as isize - 1);
                    if *s == ':' as XML_Char {
                        s = s.offset(1);
                        loop {
                            /* copies null terminator */
                            if !self.m_tempPool.append_char(*s) {
                                return XML_Error::NO_MEMORY;
                            }
                            if *s == '\u{0}' as XML_Char {
                                s = s.offset(1);
                                break;
                            }
                            s = s.offset(1);
                        }

                        if self.m_ns_triplets {
                            /* append namespace separator and prefix */
                            self.m_tempPool.replace_last_char(self.m_namespaceSeparator);
                            if !self.m_tempPool.append_c_string(xmlnsPrefix.as_ptr())
                                || !self.m_tempPool.append_char('\u{0}' as XML_Char)
                            {
                                return XML_Error::NO_MEMORY;
                            }
                        }
                    } else {
                        /* xlmns attribute without a prefix. */
                        if !self.m_tempPool.append_c_string(xmlnsPrefix.as_ptr())
                            || !self.m_tempPool.append_char('\u{0}' as XML_Char)
                        {
                            return XML_Error::NO_MEMORY;
                        }
                    }

                    /* store expanded name in attribute list */
                    s = self.m_tempPool.finish().as_ptr();
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
            i += 1;
        }

        // REXPAT: append a NULL pointer as the stop marker
        if self.m_atts.try_reserve(1).is_err() {
            return XML_Error::NO_MEMORY;
        }
        self.m_atts.push(Attribute::new_null());

        let mut binding = bindingsPtr.as_ref().map(Rc::clone);
        while let Some(b) = binding {
            get_cell_ptr(&b.attId)
                .unwrap()
                .set_type(AttributeType::Unset);
            binding = get_cell_ptr(&b.nextTagBinding);
        }
        if !self.m_ns {
            return XML_Error::NONE;
        }
        /* expand the element type name */
        let (binding, skip_to_colon) = if let Some(ref prefix) = get_cell_ptr(&elementType.prefix) {
            let binding = match get_cell_ptr(&prefix.binding) {
                Some(b) => b,
                None => return XML_Error::UNBOUND_PREFIX,
            };
            (binding, true)
        } else {
            let dtd_tables = self.m_dtd.tables.borrow_mut();
            if let Some(b) = dtd_tables
                .defaultPrefix
                .as_deref()
                .and_then(|p| get_cell_ptr(&p.binding))
            {
                (b, false)
            } else {
                return XML_Error::NONE;
            }
        };
        let binding_prefix = binding.prefix.upgrade().unwrap();
        let mut prefixLen = 0;
        if self.m_ns_triplets {
            if let Some(ref name) = binding_prefix.name {
                while name[prefixLen] != 0 {
                    prefixLen += 1;
                }
                /* prefixLen includes null terminator */
                prefixLen += 1;
            }
        }
        tagNamePtr.localPart = TagNameLocalPart::from_tag_name(&tagNamePtr.str_0, skip_to_colon);
        tagNamePtr.uriLen = (*binding).uriLen.get();
        tagNamePtr.prefix = binding_prefix
            .name
            .as_ref()
            .map_or_else(ptr::null, |x| x.as_ptr());
        tagNamePtr.prefixLen = prefixLen;

        let localPart = tagNamePtr.localPart.as_ptr();
        let mut i = 0;
        while *localPart.add(i) != 0 {
            i += 1;
        }
        /* i includes null terminator */
        i += 1;

        let mut uri = binding.uri.borrow_mut();
        uri.truncate(binding.uriLen.get());
        if uri.try_reserve(i + prefixLen).is_err() {
            return XML_Error::NO_MEMORY;
        }
        uri.extend(slice::from_raw_parts(tagNamePtr.localPart.as_ptr(), i));

        /* we always have a namespace separator between localPart and prefix */
        if prefixLen != 0 {
            /* replace null terminator */
            *uri.last_mut().unwrap() = self.m_namespaceSeparator;

            let prefix_slice = &binding_prefix.name.as_ref().unwrap()[..prefixLen];
            uri.extend(prefix_slice);
        }
        drop(uri);
        tagNamePtr.str_0 = TagNameString::BindingUri(binding);
        XML_Error::NONE
    }
}

/* addBinding() overwrites the value of prefix->binding without checking.
   Therefore one must keep track of the old value outside of addBinding().
*/

unsafe extern "C" fn addBinding(
    mut parser: &XML_ParserStruct,
    mut prefix: &Rc<Prefix>,
    mut attId: Ptr<AttributeId>,
    mut uri: *const XML_Char,
    mut bindingsPtr: &mut Ptr<Binding>,
) -> XML_Error {
    const xmlNamespace: [XML_Char; 37] = XML_STR![
        ASCII_h,
        ASCII_t,
        ASCII_t,
        ASCII_p,
        ASCII_COLON,
        ASCII_SLASH,
        ASCII_SLASH,
        ASCII_w,
        ASCII_w,
        ASCII_w,
        ASCII_PERIOD,
        ASCII_w,
        ASCII_3,
        ASCII_PERIOD,
        ASCII_o,
        ASCII_r,
        ASCII_g,
        ASCII_SLASH,
        ASCII_X,
        ASCII_M,
        ASCII_L,
        ASCII_SLASH,
        ASCII_1,
        ASCII_9,
        ASCII_9,
        ASCII_8,
        ASCII_SLASH,
        ASCII_n,
        ASCII_a,
        ASCII_m,
        ASCII_e,
        ASCII_s,
        ASCII_p,
        ASCII_a,
        ASCII_c,
        ASCII_e,
    ];
    const xmlnsNamespace: [XML_Char; 30] = XML_STR![
        ASCII_h,
        ASCII_t,
        ASCII_t,
        ASCII_p,
        ASCII_COLON,
        ASCII_SLASH,
        ASCII_SLASH,
        ASCII_w,
        ASCII_w,
        ASCII_w,
        ASCII_PERIOD,
        ASCII_w,
        ASCII_3,
        ASCII_PERIOD,
        ASCII_o,
        ASCII_r,
        ASCII_g,
        ASCII_SLASH,
        ASCII_2,
        ASCII_0,
        ASCII_0,
        ASCII_0,
        ASCII_SLASH,
        ASCII_x,
        ASCII_m,
        ASCII_l,
        ASCII_n,
        ASCII_s,
        ASCII_SLASH,
    ];
    let mut mustBeXML = false;
    let mut isXML = true;
    let mut isXMLNS = true;
    /* empty URI is only valid for default namespace per XML NS 1.0 (not 1.1) */
    if *uri as c_int == '\u{0}' as i32 && !prefix.name.is_none() {
        return XML_Error::UNDECLARING_PREFIX;
    }
    if let Some(ref name) = prefix.name {
        if name[0] == ASCII_x as XML_Char
            && name[1] == ASCII_m as XML_Char
            && name[2] == ASCII_l as XML_Char
        {
            /* Not allowed to bind xmlns */
            if name[3] == ASCII_n as XML_Char
                && name[4] == ASCII_s as XML_Char
                && name[5] == '\u{0}' as XML_Char
            {
                return XML_Error::RESERVED_PREFIX_XMLNS;
            }
            if name[3] == '\u{0}' as XML_Char {
                mustBeXML = true;
            }
        }
    }
    let mut len = 0;
    while *uri.add(len) != 0 {
        if isXML
            && (len >= xmlNamespace.len() || *uri.add(len) as c_int != xmlNamespace[len] as c_int)
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
    if parser.m_namespaceSeparator != 0 {
        len += 1;
    }
    let att_id_is_none = attId.is_none();
    let old_tag_binding = bindingsPtr.take();
    let old_prefix_binding = prefix.binding.take();
    let mut fbl = parser.m_freeBindingList.borrow_mut();
    let b = if let Some(mut b) = fbl.take() {
        let mut b_mut = Rc::get_mut(&mut b).unwrap();
        b_mut.prefix = Rc::downgrade(prefix);

        b.attId.set(attId);
        b.prevPrefixBinding.set(old_prefix_binding);
        *fbl = b.nextTagBinding.replace(old_tag_binding);

        b.uriLen.set(len);

        b
    } else {
        let b = Binding {
            prefix: Rc::downgrade(prefix),
            attId: Cell::new(attId),
            nextTagBinding: Cell::new(old_tag_binding),
            prevPrefixBinding: Cell::new(old_prefix_binding),
            uri: RefCell::new(Vec::new()),
            uriLen: Cell::new(len),
        };

        match Rc::try_new(b) {
            Ok(b) => b,
            Err(_) => return XML_Error::NO_MEMORY,
        }
    };

    let mut uri_vec = b.uri.borrow_mut();
    uri_vec.clear();
    if uri_vec.try_reserve(len).is_err() {
        return XML_Error::NO_MEMORY;
    }
    let uri_slice = slice::from_raw_parts(uri, len);
    uri_vec.extend(uri_slice);
    if parser.m_namespaceSeparator != 0 {
        uri_vec[len - 1] = parser.m_namespaceSeparator;
    }

    /* NULL binding when default namespace undeclared */
    let uri_ptr = if *uri == '\u{0}' as XML_Char
        && Rc::ptr_eq(
            prefix,
            parser.m_dtd.tables.borrow().defaultPrefix.as_ref().unwrap(),
        ) {
        ptr::null()
    } else {
        prefix.binding.set(Some(Rc::clone(&b)));
        uri_vec.as_ptr()
    };
    drop(uri_vec);
    *bindingsPtr = Some(b);

    /* if attId == NULL then we are not starting a namespace scope */
    if !att_id_is_none && parser.m_handlers.hasStartNamespaceDecl() {
        parser.m_handlers.startNamespaceDecl(
            prefix.name.as_ref().map_or_else(ptr::null, |x| x.as_ptr()),
            uri_ptr,
        );
    }
    XML_Error::NONE
}
/* The idea here is to avoid using stack for each CDATA section when
   the whole file is parsed with one call.
*/

unsafe extern "C" fn cdataSectionProcessor(
    mut parser: XML_Parser,
    buf: ExpatSliceRc,
    mut endPtr: &mut *const c_char,
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
        if (*parser).is_child_parser {
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
    start_buf: &mut Option<ExpatSliceRc>,
    mut nextPtr: &mut *const c_char,
    mut haveMore: XML_Bool,
) -> XML_Error {
    let mut buf = start_buf.as_ref().unwrap().clone();
    (*parser).eventPP(enc_type).set(buf.as_ptr());
    *start_buf = None;
    let enc = (*parser).encoding(enc_type);
    loop {
        let mut next_idx = 0;
        let mut tok = (*enc).xmlTok(XML_STATE::CDATA_SECTION, buf.as_buf_ref(), &mut next_idx);
        let mut next = buf.clone().inc_start(next_idx).as_ptr();
        (*parser).eventEndPP(enc_type).set(next);
        match tok {
            XML_TOK::CDATA_SECT_CLOSE => {
                let endHandlerRan = (*parser).m_handlers.endCDataSection();

                if endHandlerRan {
                } else if 0 != 0 && (*parser).m_handlers.hasCharacterData() {
                    (*parser).m_handlers.characterData(&[]);
                } else if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, enc_type, buf.as_buf_ref().with_end(next));
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
                    reportDefault(parser, enc_type, buf.as_buf_ref().with_end(next));
                }
            }
            XML_TOK::DATA_CHARS => {
                let mut handlers = &(*parser).m_handlers;
                if handlers.hasCharacterData() {
                    if MUST_CONVERT!(enc, buf.as_ptr()) {
                        loop {
                            let mut from_buf = buf.as_buf_ref().with_end(next);
                            let (convert_res, buf_count, data_count) =
                                XmlConvert!(enc, from_buf, &mut (*parser).m_dataBuf[..]);
                            buf = buf.clone().inc_start(buf_count);

                            (*parser).eventEndPP(enc_type).set(next);
                            handlers.characterData(&ExpatBufRef::from(&(*parser).m_dataBuf[..data_count]));
                            if convert_res == super::xmltok::XML_Convert_Result::COMPLETED
                                || convert_res
                                    == super::xmltok::XML_Convert_Result::INPUT_INCOMPLETE
                            {
                                break;
                            }
                            (*parser).eventPP(enc_type).set(buf.as_ptr());
                        }
                    } else {
                        handlers.characterData(&ExpatBufRef::<XML_Char>::from(buf.as_buf_ref().with_end(next)));
                    }
                } else if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, enc_type, buf.as_buf_ref().with_end(next));
                }
            }
            XML_TOK::INVALID => {
                (*parser).eventPP(enc_type).set(next);
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
                (*parser).eventPP(enc_type).set(next);
                return XML_Error::UNEXPECTED_STATE;
            }
        }
        buf = buf.with_start(next);
        (*parser).eventPP(enc_type).set(buf.as_ptr());
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
    buf: ExpatSliceRc,
    mut endPtr: &mut *const c_char,
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
    start_buf: &mut Option<ExpatSliceRc>,
    mut nextPtr: &mut *const c_char,
    mut haveMore: XML_Bool,
) -> XML_Error {
    let mut next: *const c_char = ptr::null();
    let mut next_idx = 0;
    let mut buf = start_buf.as_ref().unwrap().clone();
    (*parser).eventPP(enc_type).set(buf.as_ptr());
    *start_buf = None;
    let enc = (*parser).encoding(enc_type);
    let tok = (*enc).xmlTok(XML_STATE::IGNORE_SECTION, buf.as_buf_ref(), &mut next_idx);
    next = buf.clone().inc_start(next_idx).as_ptr();
    (*parser).eventEndPP(enc_type).set(next);
    match tok {
        XML_TOK::IGNORE_SECT => {
            if (*parser).m_handlers.hasDefault() {
                reportDefault(parser, enc_type, buf.as_buf_ref().with_end(next));
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
            (*parser).eventPP(enc_type).set(next); /* XML_ERROR_UNCLOSED_IGNORE_SECTION */
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
            (*parser).eventPP(enc_type).set(next);
            return XML_Error::UNEXPECTED_STATE;
        }
    };
    /* not reached */
}
/* XML_DTD */

impl XML_ParserStruct {
    unsafe fn initializeEncoding(&mut self) -> XML_Error {
        let mut s: *const c_char = ptr::null();
        if cfg!(feature = "unicode") {
            let mut encodingBuf: [libc::c_char; 128] = [0; 128];
            /* See comments abount `protoclEncodingName` in parserInit() */
            match self.m_protocolEncodingName {
                None => {
                    s = ptr::null();
                }
                Some(ref protocol_encoding_name) => {
                    let mut i = 0;
                    i = 0;
                    while protocol_encoding_name[i] != 0 {
                        if i as libc::c_ulong
                            == (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            || protocol_encoding_name[i] as libc::c_int & !(0x7f as libc::c_int)
                                != 0 as libc::c_int
                        {
                            encodingBuf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                            break;
                        } else {
                            encodingBuf[i as usize] = protocol_encoding_name[i] as libc::c_char;
                            i += 1;
                        }
                    }
                    encodingBuf[i as usize] = '\u{0}' as i32 as libc::c_char;
                    s = encodingBuf.as_mut_ptr();
                }
            }
        } else {
            s = self
                .m_protocolEncodingName
                .as_ref()
                .map_or_else(ptr::null, |x| x.as_ptr());
        }
        let enc = if self.m_ns {
            InitEncoding::new_ns(&mut self.m_encoding, s)
        } else {
            InitEncoding::new(&mut self.m_encoding, s)
        };
        if enc.is_some() {
            self.m_initEncoding = enc;
            self.m_encoding = self.m_initEncoding.as_ref().unwrap();
            return XML_Error::NONE;
        }

        let protocol_encoding_name = self
            .m_protocolEncodingName
            .as_ref()
            .map_or_else(ptr::null, |x| x.as_ptr());
        self.m_handlers.handleUnknownEncoding(
            protocol_encoding_name,
            self.m_ns,
            &mut self.m_encoding,
        )
    }

    unsafe fn processXmlDecl(
        &mut self,
        mut isGeneralTextEntity: c_int,
        buf: ExpatSliceRc,
    ) -> XML_Error {
        let mut encodingName = None;
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
            buf.as_buf_ref(),
            &self.m_eventPtr,
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
            self.m_dtd.standalone.set(true);
            if self.m_paramEntityParsing == XML_ParamEntityParsing::UNLESS_STANDALONE {
                self.m_paramEntityParsing = XML_ParamEntityParsing::NEVER
            }
            /* XML_DTD */
        }
        if self.m_handlers.hasXmlDecl() {
            if let Some(encodingName) = encodingName {
                let successful = self.m_temp2Pool.store_c_string(
                    &*self.m_encoding,
                    encodingName.with_len((*self.m_encoding).nameLength(encodingName) as usize),
                );
                if !successful {
                    return XML_Error::NO_MEMORY;
                }

                storedEncName = self.m_temp2Pool.current_start();
            }
            if let Some(version_buf) = version_buf {
                let successful = self
                    .m_temp2Pool
                    .store_c_string(&*self.m_encoding, version_buf);
                if !successful {
                    return XML_Error::NO_MEMORY;
                }

                storedversion = self.m_temp2Pool.current_start();
            }
            self.m_handlers
                .xmlDecl(storedversion, storedEncName, standalone);
        } else if self.m_handlers.hasDefault() {
            reportDefault(self, EncodingType::Normal, buf.as_buf_ref());
        }
        if self.m_protocolEncodingName.is_none() {
            if let Some(newEncoding) = newEncoding {
                /* Check that the specified encoding does not conflict with what
                 * the parser has already deduced.  Do we have the same number
                 * of bytes in the smallest representation of a character?  If
                 * this is UTF-16, is it the same endianness?
                 */
                if (*newEncoding).minBytesPerChar() != (*self.m_encoding).minBytesPerChar()
                    || (*newEncoding).minBytesPerChar() == 2
                        && !ptr::eq(newEncoding, self.m_encoding)
                {
                    self.m_eventPtr.set(encodingName.map_or_else(ptr::null, |x| x.as_ptr()));
                    return XML_Error::INCORRECT_ENCODING;
                }
                self.m_encoding = newEncoding;
            } else if let Some(encodingName) = encodingName {
                let mut result: XML_Error = XML_Error::NONE;
                if storedEncName.is_null() {
                    let successful = self.m_temp2Pool.store_c_string(
                        &*self.m_encoding,
                        encodingName.with_len((*self.m_encoding).nameLength(encodingName) as usize),
                    );

                    if !successful {
                        return XML_Error::NO_MEMORY;
                    }

                    storedEncName = self.m_temp2Pool.current_start();
                };
                result = self.m_handlers.handleUnknownEncoding(
                    storedEncName,
                    self.m_ns,
                    &mut self.m_encoding,
                );
                self.m_temp2Pool.clear();
                if result == XML_Error::UNKNOWN_ENCODING {
                    self.m_eventPtr.set(encodingName.as_ptr());
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

impl CXmlHandlers {
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
                let initialized =
                    unknown_enc.initialize(info.map.as_mut_ptr(), info.convert, info.data, m_ns);
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
    mut buf: ExpatSliceRc,
    mut nextPtr: &mut *const c_char,
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
    mut buf: ExpatSliceRc,
    mut nextPtr: &mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = (*parser).initializeEncoding();
    if result != XML_Error::NONE {
        return result;
    }
    /* we know now that XML_Parse(Buffer) has been called,
    so we consider the external parameter entity read */
    (*parser).m_dtd.paramEntityRead.set(true);
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
    init_buf: ExpatSliceRc,
    mut nextPtr: &mut *const c_char,
) -> XML_Error {
    let mut buf = init_buf.clone();
    let mut next: *const c_char = buf.as_ptr();
    let mut next_idx;
    (*parser).m_eventPtr.set(buf.as_ptr());
    loop {
        next_idx = 0;
        let tok = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf.as_buf_ref(), &mut next_idx);
        next = buf.clone().inc_start(next_idx).as_ptr();
        (*parser).m_eventEndPtr.set(next);
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
            return storeEntityValue(parser, EncodingType::Normal, init_buf.as_buf_ref());
        } else {
            if tok == XML_TOK::XML_DECL {
                let mut result: XML_Error = XML_Error::NONE;
                result = (*parser).processXmlDecl(0, buf.clone().with_len(next_idx));
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
        (*parser).m_eventPtr.set(buf.as_ptr());
    }
}

unsafe extern "C" fn externalParEntProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatSliceRc,
    mut nextPtr: &mut *const c_char,
) -> XML_Error {
    let mut next_idx = 0;
    let mut tok = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf.as_buf_ref(), &mut next_idx);
    let mut next = buf.clone().inc_start(next_idx).as_ptr();
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
        next_idx = 0;
        tok = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf.as_buf_ref(), &mut next_idx);
        next = buf.clone().inc_start(next_idx).as_ptr();
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
    mut buf: ExpatSliceRc,
    mut nextPtr: &mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = buf.as_ptr();
    let mut next_idx;
    let mut enc: &ENCODING = &*(*parser).m_encoding;
    let mut tok = XML_TOK::INVALID;
    loop {
        next_idx = 0;
        tok = (*enc).xmlTok(XML_STATE::PROLOG, buf.as_buf_ref(), &mut next_idx);
        next = buf.clone().inc_start(next_idx).as_ptr();
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
            return storeEntityValue(parser, EncodingType::Normal, buf.as_buf_ref());
        }
        buf = buf.with_start(next);
    }
}
/* XML_DTD */

unsafe extern "C" fn prologProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatSliceRc,
    mut nextPtr: &mut *const c_char,
) -> XML_Error {
    let mut next_idx = 0;
    let mut tok = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf.as_buf_ref(), &mut next_idx);
    let next = buf.clone().inc_start(next_idx).as_ptr();
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

impl XML_ParserStruct {
    unsafe fn doPrologHandleEntityRef(
        &mut self,
        mut entity: &Rc<Entity>,
        role: XML_ROLE,
        handleDefault: &mut bool,
    ) -> Option<XML_Error> {
        if entity.open.get() {
            return Some(XML_Error::RECURSIVE_ENTITY_REF);
        }
        if !entity.textPtr.borrow().is_none() {
            let mut result_4 = XML_Error::NONE;
            let mut betweenDecl = if role == XML_ROLE::PARAM_ENTITY_REF {
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
            self.m_dtd.paramEntityRead.set(false);
            entity.open.set(true);
            let entity_name = if cfg!(feature = "mozilla") {
                entity.name.borrow().as_ref().unwrap().as_ptr()
            } else {
                ptr::null()
            };
            if self.m_handlers.externalEntityRef(
                entity_name,
                entity.base.get(),
                entity.systemId.get(),
                entity.publicId.get(),
            ) == Ok(0)
            {
                entity.open.set(false);
                return Some(XML_Error::EXTERNAL_ENTITY_HANDLING);
            }
            entity.open.set(false);
            *handleDefault = false;
            if !self.m_dtd.paramEntityRead.get() {
                self.m_dtd.keepProcessing.set(self.m_dtd.standalone.get());
            } else {
                /* XML_DTD */
                if !self.m_dtd.standalone.get() && self.m_handlers.notStandalone() == Ok(0) {
                    return Some(XML_Error::NOT_STANDALONE);
                }
            }
        } else {
            self.m_dtd.keepProcessing.set(self.m_dtd.standalone.get());
        }
        None
    }

    unsafe fn doProlog(
        &mut self,
        mut enc_type: EncodingType,
        mut buf: ExpatSliceRc,
        mut tok: XML_TOK,
        mut next: *const c_char,
        mut nextPtr: &mut *const c_char,
        mut haveMore: XML_Bool,
        mut allowClosingDoctype: XML_Bool,
    ) -> XML_Error {
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
            ASCII_N,
            ASCII_O,
            ASCII_T,
            ASCII_A,
            ASCII_T,
            ASCII_I,
            ASCII_O,
            ASCII_N,
            ASCII_LPAREN
        ];
        const enumValueSep: [XML_Char; 2] = XML_STR![ASCII_PIPE];
        const enumValueStart: [XML_Char; 2] = XML_STR![ASCII_LPAREN];
        /* save one level of indirection */
        let mut quant: XML_Content_Quant = XML_Content_Quant::NONE;
        let mut enc = self.encoding(enc_type);
        loop {
            let mut handleDefault = true;
            self.eventPP(enc_type).set(buf.as_ptr());
            self.eventEndPP(enc_type).set(next);
            if tok.is_error() {
                if haveMore && tok != XML_TOK::INVALID {
                    *nextPtr = buf.as_ptr();
                    return XML_Error::NONE;
                }
                match tok {
                    XML_TOK::INVALID => {
                        self.eventPP(enc_type).set(next);
                        return XML_Error::INVALID_TOKEN;
                    }
                    XML_TOK::PARTIAL => return XML_Error::UNCLOSED_TOKEN,
                    XML_TOK::PARTIAL_CHAR => return XML_Error::PARTIAL_CHAR,

                    XML_TOK::PROLOG_S_NEG => tok = XML_TOK::PROLOG_S,
                    XML_TOK::NONE => {
                        /* for internal PE NOT referenced between declarations */
                        if enc_type.is_internal()
                            && !self.m_openInternalEntities.as_deref().unwrap().betweenDecl
                        {
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
                &mut self.m_prologState,
                tok,
                buf.as_buf_ref().with_end(next),
                enc,
            );
            match role {
                XML_ROLE::XML_DECL => {
                    let mut result: XML_Error = self.processXmlDecl(0, buf.clone().with_end(next));
                    if result != XML_Error::NONE {
                        return result;
                    }
                    enc = self.encoding(EncodingType::Normal);
                    handleDefault = false;
                }
                XML_ROLE::DOCTYPE_NAME => {
                    if self.m_handlers.hasStartDoctypeDecl() {
                        if !self.m_tempPool.store_c_string(enc, buf.as_buf_ref().with_end(next)) {
                            self.m_doctypeName = None;
                            return XML_Error::NO_MEMORY;
                        }
                        self.m_doctypeName = Some(self.m_tempPool.finish());
                        self.m_doctypePubid = ptr::null();
                        handleDefault = false
                    }
                    self.m_doctypeSysid = ptr::null();
                }
                XML_ROLE::DOCTYPE_INTERNAL_SUBSET => {
                    let startHandlerRan = self.m_handlers.startDoctypeDecl(
                        self.m_doctypeName
                            .as_ref()
                            .map_or_else(ptr::null, |x| x.as_ptr()),
                        self.m_doctypeSysid,
                        self.m_doctypePubid,
                        1,
                    );

                    if startHandlerRan {
                        self.m_doctypeName = None;
                        self.m_tempPool.clear();
                        handleDefault = false
                    }
                }
                XML_ROLE::TEXT_DECL => {
                    let mut result_0: XML_Error = self.processXmlDecl(1, buf.clone().with_end(next));
                    if result_0 != XML_Error::NONE {
                        return result_0;
                    }
                    enc = self.encoding(EncodingType::Normal);
                    handleDefault = false;
                }
                XML_ROLE::DOCTYPE_PUBLIC_ID => {
                    let ess = match self.m_dtd.intern_external_subset() {
                        Ok(ess) => ess,
                        Err(_) => return XML_Error::NO_MEMORY,
                    };

                    let mut dtd_tables = self.m_dtd.tables.borrow_mut();
                    /* XML_DTD */
                    self.m_useForeignDTD = false;
                    self.m_declEntity =
                        hash_insert!(&mut dtd_tables.paramEntities, ess, Rc::try_new, Entity)
                            .cloned()
                            .into();
                    if self.m_declEntity.is_none() {
                        return XML_Error::NO_MEMORY;
                    }
                    /* XML_DTD */
                    self.m_dtd.hasParamEntityRefs.set(true);
                    if self.m_handlers.hasStartDoctypeDecl() {
                        if (*enc).isPublicId(buf.as_buf_ref().with_end(next), self.eventPP(enc_type)) == 0 {
                            return XML_Error::PUBLICID;
                        }
                        let successful = self.m_tempPool.store_c_string(
                            enc,
                            buf.as_buf_ref()
                                .inc_start((*enc).minBytesPerChar())
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar()),
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        }
                        let pub_id = self.m_tempPool.finish_cells();
                        normalizePublicId(&pub_id);
                        self.m_doctypePubid = pub_id.as_ptr() as *const _;
                        handleDefault = false;
                    } else {
                        if (*enc).isPublicId(buf.as_buf_ref().with_end(next), self.eventPP(enc_type)) == 0 {
                            return XML_Error::PUBLICID;
                        }
                    }
                }
                XML_ROLE::ENTITY_PUBLIC_ID => {
                    if (*enc).isPublicId(buf.as_buf_ref().with_end(next), self.eventPP(enc_type)) == 0 {
                        return XML_Error::PUBLICID;
                    }
                }
                XML_ROLE::DOCTYPE_CLOSE => {
                    if allowClosingDoctype != true {
                        /* Must not close doctype from within expanded parameter entities */
                        return XML_Error::INVALID_TOKEN;
                    }
                    if !self.m_doctypeName.is_none() {
                        self.m_handlers.startDoctypeDecl(
                            self.m_doctypeName
                                .as_ref()
                                .map_or_else(ptr::null, |x| x.as_ptr()),
                            self.m_doctypeSysid,
                            self.m_doctypePubid,
                            0,
                        );
                        self.m_doctypeName = None;
                        self.m_tempPool.clear();
                        handleDefault = false
                    }
                    /* parser->m_doctypeSysid will be non-NULL in the case of a previous
                    XML_ROLE::DOCTYPE_SYSTEM_ID, even if parser->m_startDoctypeDeclHandler
                    was not set, indicating an external subset
                     */
                    if !self.m_doctypeSysid.is_null() || self.m_useForeignDTD {
                        let mut hadParamEntityRefs = self.m_dtd.hasParamEntityRefs.get();
                        self.m_dtd.hasParamEntityRefs.set(true);
                        if self.m_paramEntityParsing != XML_ParamEntityParsing::NEVER
                            && self.m_handlers.hasExternalEntityRef()
                        {
                            let (base, systemId, publicId) = {
                                let ess = match self.m_dtd.intern_external_subset() {
                                    Ok(ess) => ess,
                                    Err(_) => return XML_Error::NO_MEMORY,
                                };

                                let mut dtd_tables = self.m_dtd.tables.borrow_mut();
                                let entity: &Rc<Entity> = match hash_insert!(
                                    &mut dtd_tables.paramEntities,
                                    ess,
                                    Rc::try_new,
                                    Entity
                                )
                                .into()
                                {
                                    Some(entity) => entity,
                                    /* end of DTD - no need to update dtd->keepProcessing */
                                    /* The external subset name "#" will have already been
                                     * inserted into the hash table at the start of the
                                     * external entity parsing, so no allocation will happen
                                     * and lookup() cannot fail.
                                     */
                                    None => return XML_Error::NO_MEMORY, /* LCOV_EXCL_LINE */
                                };

                                if self.m_useForeignDTD {
                                    entity.base.set(self.m_curBase);
                                }
                                self.m_dtd.paramEntityRead.set(false);

                                (
                                    entity.base.get(),
                                    entity.systemId.get(),
                                    entity.publicId.get(),
                                )
                            };
                            if self.m_handlers.externalEntityRef(
                                ptr::null(),
                                base,
                                systemId,
                                publicId,
                            ) == Ok(0)
                            {
                                return XML_Error::EXTERNAL_ENTITY_HANDLING;
                            }
                            if self.m_dtd.paramEntityRead.get() {
                                if !self.m_dtd.standalone.get()
                                    && self.m_handlers.notStandalone() == Ok(0)
                                {
                                    return XML_Error::NOT_STANDALONE;
                                }
                            } else if self.m_doctypeSysid.is_null() {
                                self.m_dtd.hasParamEntityRefs.set(hadParamEntityRefs);
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
                        let mut hadParamEntityRefs = self.m_dtd.hasParamEntityRefs.get();
                        self.m_dtd.hasParamEntityRefs.set(true);
                        if self.m_paramEntityParsing != XML_ParamEntityParsing::NEVER
                            && self.m_handlers.hasExternalEntityRef()
                        {
                            let ess = match self.m_dtd.intern_external_subset() {
                                Ok(ess) => ess,
                                Err(_) => return XML_Error::NO_MEMORY,
                            };

                            let mut dtd_tables = self.m_dtd.tables.borrow_mut();
                            let mut entity: &Rc<Entity> = match hash_insert!(
                                &mut dtd_tables.paramEntities,
                                ess,
                                Rc::try_new,
                                Entity
                            )
                            .into()
                            {
                                Some(entity) => entity,
                                None => return XML_Error::NO_MEMORY,
                            };
                            entity.base.set(self.m_curBase);
                            self.m_dtd.paramEntityRead.set(false);
                            if self.m_handlers.externalEntityRef(
                                ptr::null(),
                                entity.base.get(),
                                entity.systemId.get(),
                                entity.publicId.get(),
                            ) == Ok(0)
                            {
                                return XML_Error::EXTERNAL_ENTITY_HANDLING;
                            }
                            if self.m_dtd.paramEntityRead.get() {
                                if !self.m_dtd.standalone.get()
                                    && self.m_handlers.notStandalone() == Ok(0)
                                {
                                    return XML_Error::NOT_STANDALONE;
                                }
                            } else {
                                /* end of DTD - no need to update dtd->keepProcessing */
                                /* if we didn't read the foreign DTD then this means that there
                                is no external subset and we must reset dtd->hasParamEntityRefs
                                 */
                                self.m_dtd.hasParamEntityRefs.set(hadParamEntityRefs);
                            }
                        }
                    }
                    /* XML_DTD */
                    self.m_processor = Some(contentProcessor as Processor);
                    return contentProcessor(self, buf, nextPtr);
                }
                XML_ROLE::ATTLIST_ELEMENT_NAME => {
                    self.m_declElementType = self.getElementType(enc_type, buf.as_buf_ref().with_end(next));
                    if self.m_declElementType.is_none() {
                        return XML_Error::NO_MEMORY;
                    }
                }
                XML_ROLE::ATTRIBUTE_NAME => {
                    self.m_declAttributeId = self.getAttributeId(enc_type, buf.as_buf_ref().with_end(next));
                    if self.m_declAttributeId.is_none() {
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
                    if self.m_dtd.keepProcessing.get() && self.m_handlers.hasAttlistDecl() {
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
                        if !self.m_tempPool.append(enc, buf.as_buf_ref().with_end(next)) {
                            return XML_Error::NO_MEMORY;
                        }
                        // This gets finalized later, not exactly the safest
                        // idiom though.
                        //REXPAT: disable this for safety
                        //self.m_declAttributeType = self.m_tempPool.current_start();
                        handleDefault = false
                    }
                }
                XML_ROLE::IMPLIED_ATTRIBUTE_VALUE | XML_ROLE::REQUIRED_ATTRIBUTE_VALUE => {
                    if self.m_dtd.keepProcessing.get() {
                        if defineAttribute(
                            self.m_declElementType.as_ref().unwrap(),
                            self.m_declAttributeId.as_ref().unwrap(),
                            self.m_declAttributeIsCdata,
                            self.m_declAttributeIsId,
                            ptr::null(),
                        ) == 0
                        {
                            return XML_Error::NO_MEMORY;
                        }
                        if self.m_handlers.hasAttlistDecl() && !self.m_declAttributeType.is_null() {
                            if *self.m_declAttributeType == ASCII_LPAREN as XML_Char
                                || *self.m_declAttributeType == ASCII_N as XML_Char
                                    && *self.m_declAttributeType.offset(1) == ASCII_O as XML_Char
                            {
                                /* Enumerated or Notation type */
                                if !self.m_tempPool.append_char(ASCII_RPAREN as XML_Char)
                                    || !self.m_tempPool.append_char('\u{0}' as XML_Char)
                                {
                                    return XML_Error::NO_MEMORY;
                                }
                                self.m_declAttributeType = self.m_tempPool.finish().as_ptr();
                            }
                            self.eventEndPP(enc_type).set(buf.as_ptr());
                            self.m_handlers.attlistDecl(
                                self.m_declElementType.as_ref().unwrap().name.as_ptr(),
                                self.m_declAttributeId.as_ref().unwrap().name().as_ptr(),
                                self.m_declAttributeType,
                                ptr::null(),
                                (role == super::xmlrole::XML_ROLE::REQUIRED_ATTRIBUTE_VALUE)
                                    as c_int,
                            );
                            self.m_tempPool.clear();
                            handleDefault = false
                        }
                    }
                }
                XML_ROLE::DEFAULT_ATTRIBUTE_VALUE | XML_ROLE::FIXED_ATTRIBUTE_VALUE => {
                    if self.m_dtd.keepProcessing.get() {
                        let mut attVal: *const XML_Char = ptr::null();
                        let mut result_1: XML_Error = storeAttributeValue(
                            self,
                            enc_type,
                            self.m_declAttributeIsCdata,
                            buf.as_buf_ref()
                                .inc_start((*enc).minBytesPerChar())
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar()),
                            &self.m_dtd.pool,
                        );
                        if result_1 as u64 != 0 {
                            return result_1;
                        }
                        attVal = self.m_dtd.pool.finish().as_ptr();
                        /* ID attributes aren't allowed to have a default */
                        if defineAttribute(
                            self.m_declElementType.as_ref().unwrap(),
                            self.m_declAttributeId.as_ref().unwrap(),
                            self.m_declAttributeIsCdata,
                            false,
                            attVal,
                        ) == 0
                        {
                            return XML_Error::NO_MEMORY;
                        }
                        if self.m_handlers.hasAttlistDecl() && !self.m_declAttributeType.is_null() {
                            if *self.m_declAttributeType == ASCII_LPAREN as XML_Char
                                || *self.m_declAttributeType == ASCII_N as XML_Char
                                    && *self.m_declAttributeType.offset(1) == ASCII_O as XML_Char
                            {
                                /* Enumerated or Notation type */
                                if !self.m_tempPool.append_char(ASCII_RPAREN as XML_Char)
                                    || !self.m_tempPool.append_char('\u{0}' as XML_Char)
                                {
                                    return XML_Error::NO_MEMORY;
                                }
                                self.m_declAttributeType = self.m_tempPool.finish().as_ptr();
                            }
                            self.eventEndPP(enc_type).set(buf.as_ptr());
                            self.m_handlers.attlistDecl(
                                self.m_declElementType.as_ref().unwrap().name.as_ptr(),
                                self.m_declAttributeId.as_ref().unwrap().name().as_ptr(),
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
                    if self.m_dtd.keepProcessing.get() {
                        let mut result_2: XML_Error = storeEntityValue(
                            self,
                            enc_type,
                            buf.as_buf_ref()
                                .inc_start((*enc).minBytesPerChar())
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar()),
                        );

                        if !self.m_declEntity.is_none() {
                            let mut declEntity = self.m_declEntity.as_deref().unwrap();
                            let text_slice = self.m_dtd.entityValuePool.finish();
                            *declEntity.textPtr.borrow_mut() = Some(text_slice.clone());
                            if self.m_handlers.hasEntityDecl() {
                                self.eventEndPP(enc_type).set(buf.as_ptr());
                                self.m_handlers.entityDecl(
                                    declEntity.name.borrow().as_ref().unwrap().as_ptr(),
                                    declEntity.is_param.get() as c_int,
                                    text_slice.as_ptr(),
                                    text_slice.len().try_into().unwrap(),
                                    self.m_curBase,
                                    ptr::null(),
                                    ptr::null(),
                                    ptr::null(),
                                );
                                handleDefault = false;
                            }
                        } else {
                            self.m_dtd.entityValuePool.discard();
                        }
                        if result_2 != XML_Error::NONE {
                            return result_2;
                        }
                    }
                }
                XML_ROLE::DOCTYPE_SYSTEM_ID => {
                    self.m_useForeignDTD = false;
                    /* XML_DTD */
                    self.m_dtd.hasParamEntityRefs.set(true);
                    if self.m_handlers.hasStartDoctypeDecl() {
                        let successful = self.m_tempPool.store_c_string(
                            enc,
                            buf.as_buf_ref()
                                .inc_start((*enc).minBytesPerChar())
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar()),
                        );
                        if !successful {
                            self.m_doctypeSysid = ptr::null();
                            return XML_Error::NO_MEMORY;
                        }
                        self.m_doctypeSysid = self.m_tempPool.finish().as_ptr();

                        handleDefault = false
                    } else {
                        /* use externalSubsetName to make parser->m_doctypeSysid non-NULL
                        for the case where no parser->m_startDoctypeDeclHandler is set */
                        self.m_doctypeSysid = match self.m_dtd.intern_external_subset() {
                            Ok(ess) => ess.as_ptr(),
                            Err(_) => return XML_Error::NO_MEMORY,
                        };
                    }
                    /* XML_DTD */
                    if !self.m_dtd.standalone.get()
                        && self.m_paramEntityParsing as u64 == 0
                        && self.m_handlers.notStandalone() == Ok(0)
                    {
                        return XML_Error::NOT_STANDALONE;
                    }
                    /* XML_DTD */
                    if self.m_declEntity.is_none() {
                        let ess = match self.m_dtd.intern_external_subset() {
                            Ok(ess) => ess,
                            Err(_) => return XML_Error::NO_MEMORY,
                        };

                        let mut dtd_tables = self.m_dtd.tables.borrow_mut();
                        self.m_declEntity =
                            hash_insert!(&mut dtd_tables.paramEntities, ess, Rc::try_new, Entity)
                                .cloned()
                                .into();
                        if let Some(declEntity) = self.m_declEntity.as_deref() {
                            declEntity.publicId.set(ptr::null());
                        } else {
                            return XML_Error::NO_MEMORY;
                        }
                    }
                }
                XML_ROLE::ENTITY_SYSTEM_ID => {}
                XML_ROLE::ENTITY_COMPLETE => {
                    if self.m_dtd.keepProcessing.get()
                        && !self.m_declEntity.is_none()
                        && self.m_handlers.hasEntityDecl()
                    {
                        self.eventEndPP(enc_type).set(buf.as_ptr());
                        let mut declEntity = self.m_declEntity.as_deref().unwrap();
                        self.m_handlers.entityDecl(
                            declEntity.name.borrow().as_ref().unwrap().as_ptr(),
                            declEntity.is_param.get() as c_int,
                            ptr::null(),
                            0,
                            declEntity.base.get(),
                            declEntity.systemId.get(),
                            declEntity.publicId.get(),
                            ptr::null(),
                        );
                        handleDefault = false
                    }
                }
                XML_ROLE::ENTITY_NOTATION_NAME => {
                    if self.m_dtd.keepProcessing.get() && !self.m_declEntity.is_none() {
                        let mut declEntity = self.m_declEntity.as_deref().unwrap();
                        if !self.m_dtd.pool.store_c_string(enc, buf.as_buf_ref().with_end(next)) {
                            declEntity.notation.set(ptr::null());
                            return XML_Error::NO_MEMORY;
                        }
                        declEntity.notation.set(self.m_dtd.pool.finish().as_ptr());
                        if self.m_handlers.hasUnparsedEntityDecl() {
                            self.eventEndPP(enc_type).set(buf.as_ptr());
                            self.m_handlers.unparsedEntityDecl(
                                declEntity.name.borrow().as_ref().unwrap().as_ptr(),
                                declEntity.base.get(),
                                declEntity.systemId.get(),
                                declEntity.publicId.get(),
                                declEntity.notation.get(),
                            );
                            handleDefault = false
                        } else if self.m_handlers.hasEntityDecl() {
                            self.eventEndPP(enc_type).set(buf.as_ptr());
                            self.m_handlers.entityDecl(
                                declEntity.name.borrow().as_ref().unwrap().as_ptr(),
                                0,
                                ptr::null(),
                                0,
                                declEntity.base.get(),
                                declEntity.systemId.get(),
                                declEntity.publicId.get(),
                                declEntity.notation.get(),
                            );
                            handleDefault = false
                        }
                    }
                }
                XML_ROLE::GENERAL_ENTITY_NAME => {
                    let mut dtd_tables = self.m_dtd.tables.borrow_mut();
                    if (*enc).predefinedEntityName(buf.as_buf_ref().with_end(next)) != 0 {
                        self.m_declEntity = None;
                    } else if self.m_dtd.keepProcessing.get() {
                        if !self.m_dtd.pool.store_c_string(enc, buf.as_buf_ref().with_end(next)) {
                            return XML_Error::NO_MEMORY;
                        }
                        let declEntity =
                            hash_insert_pool!(dtd_tables.generalEntities, &self.m_dtd.pool, Entity)
                                .cloned();
                        match declEntity {
                            HashInsertResult::Found(_) => {
                                self.m_declEntity = None;
                            }
                            HashInsertResult::New(declEntity) => {
                                declEntity.publicId.set(ptr::null());
                                declEntity.is_param.set(false);
                                /* if we have a parent parser or are reading an internal parameter
                                entity, then the entity declaration is not considered "internal"
                                 */
                                declEntity.is_internal.set(
                                    !(self.is_child_parser
                                        || !self.m_openInternalEntities.is_none())
                                        as XML_Bool,
                                );
                                if self.m_handlers.hasEntityDecl() {
                                    handleDefault = false
                                }
                                self.m_declEntity = Some(declEntity);
                            }
                            HashInsertResult::Err => return XML_Error::NO_MEMORY,
                        };
                    } else {
                        self.m_dtd.pool.discard();
                        self.m_declEntity = None;
                    }
                }
                XML_ROLE::PARAM_ENTITY_NAME => {
                    let mut dtd_tables = self.m_dtd.tables.borrow_mut();
                    if self.m_dtd.keepProcessing.get() {
                        if !self.m_dtd.pool.store_c_string(enc, buf.as_buf_ref().with_end(next)) {
                            return XML_Error::NO_MEMORY;
                        }
                        let declEntity =
                            hash_insert_pool!(dtd_tables.paramEntities, &self.m_dtd.pool, Entity)
                                .cloned();
                        match declEntity {
                            HashInsertResult::Found(_) => {
                                self.m_declEntity = None;
                            }
                            HashInsertResult::New(declEntity) => {
                                declEntity.publicId.set(ptr::null());
                                declEntity.is_param.set(true);
                                /* if we have a parent parser or are reading an internal parameter
                                entity, then the entity declaration is not considered "internal"
                                 */
                                declEntity.is_internal.set(
                                    !(self.is_child_parser
                                        || !self.m_openInternalEntities.is_none()),
                                );
                                if self.m_handlers.hasEntityDecl() {
                                    handleDefault = false
                                }
                                self.m_declEntity = Some(declEntity);
                            }
                            HashInsertResult::Err => return XML_Error::NO_MEMORY,
                        };
                    } else {
                        self.m_dtd.pool.discard();
                        self.m_declEntity = None;
                    }
                }
                XML_ROLE::NOTATION_NAME => {
                    self.m_declNotationPublicId = ptr::null();
                    self.m_declNotationName = ptr::null();
                    if self.m_handlers.hasNotationDecl() {
                        if !self.m_tempPool.store_c_string(enc, buf.as_buf_ref().with_end(next)) {
                            self.m_declNotationName = ptr::null();
                            return XML_Error::NO_MEMORY;
                        }
                        self.m_declNotationName = self.m_tempPool.finish().as_ptr();
                        handleDefault = false
                    }
                }
                XML_ROLE::NOTATION_PUBLIC_ID => {
                    if (*enc).isPublicId(buf.as_buf_ref().with_end(next), self.eventPP(enc_type)) == 0 {
                        return XML_Error::PUBLICID;
                    }
                    if !self.m_declNotationName.is_null() {
                        /* means m_notationDeclHandler != NULL */
                        let successful = self.m_tempPool.store_c_string(
                            enc,
                            buf.as_buf_ref()
                                .inc_start((*enc).minBytesPerChar())
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar()),
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        }
                        let tem_0 = self.m_tempPool.finish_cells();
                        normalizePublicId(&tem_0);
                        self.m_declNotationPublicId = tem_0.as_ptr() as *const _;
                        handleDefault = false
                    }
                }
                XML_ROLE::NOTATION_SYSTEM_ID => {
                    if !self.m_declNotationName.is_null() && self.m_handlers.hasNotationDecl() {
                        let successful = self.m_tempPool.store_c_string(
                            enc,
                            buf.as_buf_ref()
                                .inc_start((*enc).minBytesPerChar())
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar()),
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        }
                        self.eventEndPP(enc_type).set(buf.as_ptr());
                        self.m_tempPool.with_current_slice(|systemId| {
                            self.m_handlers.notationDecl(
                                self.m_declNotationName,
                                self.m_curBase,
                                systemId.as_ptr(),
                                self.m_declNotationPublicId,
                            )
                        });
                        handleDefault = false
                    }
                    self.m_tempPool.clear();
                }
                XML_ROLE::NOTATION_NO_SYSTEM_ID => {
                    if !self.m_declNotationPublicId.is_null() && self.m_handlers.hasNotationDecl() {
                        self.eventEndPP(enc_type).set(buf.as_ptr());
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
                        reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
                    }
                    handleDefault = false;
                    let mut ignore_buf = Some(buf.clone().with_start(next));
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
                    if self.m_prologState.level as usize >= self.m_groupConnector.len() {
                        let new_len = self.m_prologState.level as usize + 1;
                        let extra = new_len - self.m_groupConnector.len();
                        if self.m_groupConnector.try_reserve(extra).is_err() {
                            return XML_Error::NO_MEMORY;
                        }
                        self.m_groupConnector.resize(new_len, 0);
                    }
                    self.m_groupConnector[self.m_prologState.level as usize] = 0;
                    if self.m_dtd.in_eldecl.get() {
                        let mut scf = self.m_dtd.scaffold.borrow_mut();
                        if scf.index.try_reserve(1).is_err() {
                            return XML_Error::NO_MEMORY;
                        }
                        match scf.next_part() {
                            Some(myindex) => {
                                scf.index.push(myindex);
                                scf.scaffold[myindex].type_0 = XML_Content_Type::SEQ;
                            }
                            None => return XML_Error::NO_MEMORY,
                        };
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = false
                        }
                    }
                }
                XML_ROLE::GROUP_SEQUENCE => {
                    if self.m_groupConnector[self.m_prologState.level as usize] == ASCII_PIPE {
                        return XML_Error::SYNTAX;
                    }
                    self.m_groupConnector[self.m_prologState.level as usize] = ASCII_COMMA;
                    if self.m_dtd.in_eldecl.get() && self.m_handlers.hasElementDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::GROUP_CHOICE => {
                    if self.m_groupConnector[self.m_prologState.level as usize] == ASCII_COMMA {
                        return XML_Error::SYNTAX;
                    }
                    if self.m_dtd.in_eldecl.get()
                        && self.m_groupConnector[self.m_prologState.level as usize] == 0
                    {
                        let mut scf = self.m_dtd.scaffold.borrow_mut();
                        let idx = scf.index.last().copied().unwrap();
                        if scf.scaffold[idx].type_0 != XML_Content_Type::MIXED {
                            scf.scaffold[idx].type_0 = XML_Content_Type::CHOICE;
                            if self.m_handlers.hasElementDecl() {
                                handleDefault = false
                            }
                        }
                    }
                    self.m_groupConnector[self.m_prologState.level as usize] = ASCII_PIPE;
                }
                XML_ROLE::PARAM_ENTITY_REF | XML_ROLE::INNER_PARAM_ENTITY_REF => {
                    self.m_dtd.hasParamEntityRefs.set(true);
                    if self.m_paramEntityParsing as u64 == 0 {
                        self.m_dtd.keepProcessing.set(self.m_dtd.standalone.get());
                        /* XML_DTD */
                        if !self.m_dtd.standalone.get() && self.m_handlers.notStandalone() == Ok(0)
                        {
                            return XML_Error::NOT_STANDALONE;
                        }
                    } else {
                        let successful = self.m_dtd.pool.store_c_string(
                            enc,
                            buf.as_buf_ref()
                                .inc_start((*enc).minBytesPerChar())
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar()),
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        }
                        let mut entity = self.m_dtd.pool.with_current_slice(|key| {
                            let mut dtd_tables = self.m_dtd.tables.borrow_mut();
                            dtd_tables.paramEntities.get(key).map(Rc::clone)
                        });

                        /* first, determine if a check for an existing declaration is needed;
                        if yes, check that the entity exists, and that it is internal,
                        otherwise call the skipped entity handler
                         */
                        // REXPAT: Moved `clear_current` from outside the following branches
                        // into them to avoid a use-after-free when the 2nd branch uses the
                        // `name` pointer again.
                        if self.m_prologState.documentEntity != 0
                            && (if self.m_dtd.standalone.get() {
                                self.m_openInternalEntities.is_none()
                            } else {
                                !self.m_dtd.hasParamEntityRefs.get()
                            })
                        {
                            self.m_dtd.pool.discard();
                            let entity = match entity {
                                None => return XML_Error::UNDEFINED_ENTITY,
                                Some(entity) => {
                                    if !entity.is_internal.get() {
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
                                    entity
                                }
                            };
                            if let Some(r) =
                                self.doPrologHandleEntityRef(&entity, role, &mut handleDefault)
                            {
                                return r;
                            }
                            /* XML_DTD */
                            if !self.m_dtd.standalone.get()
                                && self.m_handlers.notStandalone() == Ok(0)
                            {
                                return XML_Error::NOT_STANDALONE;
                            }
                        } else if entity.is_none() {
                            self.m_dtd.keepProcessing.set(self.m_dtd.standalone.get());
                            /* cannot report skipped entities in declarations */
                            if role == super::xmlrole::XML_ROLE::PARAM_ENTITY_REF
                                && self.m_handlers.hasSkippedEntity()
                            {
                                self.m_dtd.pool.with_current_slice(|name| {
                                    self.m_handlers.skippedEntity(name.as_ptr(), 1);
                                });
                                handleDefault = false;
                            }
                            self.m_dtd.pool.discard();
                        } else {
                            self.m_dtd.pool.discard();
                            if let Some(r) = self.doPrologHandleEntityRef(
                                entity.as_ref().unwrap(),
                                role,
                                &mut handleDefault,
                            ) {
                                return r;
                            }
                            /* XML_DTD */
                            if !self.m_dtd.standalone.get()
                                && self.m_handlers.notStandalone() == Ok(0)
                            {
                                return XML_Error::NOT_STANDALONE;
                            }
                        }
                    }
                }
                XML_ROLE::ELEMENT_NAME => {
                    /* Element declaration stuff */
                    if self.m_handlers.hasElementDecl() {
                        self.m_declElementType = self.getElementType(enc_type, buf.as_buf_ref().with_end(next));
                        if self.m_declElementType.is_none() {
                            return XML_Error::NO_MEMORY;
                        }
                        // FIXME: turn into new Rc instead???
                        let mut scf = self.m_dtd.scaffold.borrow_mut();
                        scf.scaffold.clear();
                        scf.index.clear();
                        self.m_dtd.in_eldecl.set(true);
                        handleDefault = false
                    }
                }
                XML_ROLE::CONTENT_ANY | XML_ROLE::CONTENT_EMPTY => {
                    if self.m_dtd.in_eldecl.get() {
                        if self.m_handlers.hasElementDecl() {
                            let content = XML_Content {
                                quant: XML_Content_Quant::NONE,
                                name: ptr::null_mut(),
                                numchildren: 0,
                                children: ptr::null_mut(),
                                type_0: if role == super::xmlrole::XML_ROLE::CONTENT_ANY {
                                    XML_Content_Type::ANY
                                } else {
                                    XML_Content_Type::EMPTY
                                },
                            };
                            let content = ContentModel::new(content);
                            let mut content = match Box::try_new(content) {
                                Ok(content) => content,
                                Err(_) => return XML_Error::NO_MEMORY,
                            };

                            self.eventEndPP(enc_type).set(buf.as_ptr());
                            self.m_handlers.elementDecl(
                                self.m_declElementType.as_ref().unwrap().name.as_ptr(),
                                Box::into_raw(content) as *mut _,
                            );
                            handleDefault = false
                        }
                        self.m_dtd.in_eldecl.set(false);
                    }
                }
                XML_ROLE::CONTENT_PCDATA => {
                    if self.m_dtd.in_eldecl.get() {
                        let mut scf = self.m_dtd.scaffold.borrow_mut();
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
                    if reportProcessingInstruction(self, enc_type, buf.as_buf_ref().with_end(next)) == 0 {
                        return XML_Error::NO_MEMORY;
                    }
                    handleDefault = false;
                }
                XML_ROLE::COMMENT => {
                    if reportComment(self, enc_type, buf.as_buf_ref().with_end(next)) == 0 {
                        return XML_Error::NO_MEMORY;
                    }
                    handleDefault = false;
                }
                XML_ROLE::NONE => match tok {
                    XML_TOK::BOM => handleDefault = false,
                    _ => {}
                },
                XML_ROLE::DOCTYPE_NONE => {
                    if self.m_handlers.hasStartDoctypeDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::ENTITY_NONE => {
                    if self.m_dtd.keepProcessing.get() && self.m_handlers.hasEntityDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::NOTATION_NONE => {
                    if self.m_handlers.hasNotationDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::ATTLIST_NONE => {
                    if self.m_dtd.keepProcessing.get() && self.m_handlers.hasAttlistDecl() {
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
                XML_ROLE::ATTRIBUTE_NAME
                | XML_ROLE::ATTLIST_ELEMENT_NAME
                | XML_ROLE::ATTRIBUTE_TYPE_CDATA
                | XML_ROLE::ATTRIBUTE_TYPE_ID
                | XML_ROLE::ATTRIBUTE_TYPE_IDREF
                | XML_ROLE::ATTRIBUTE_TYPE_IDREFS
                | XML_ROLE::ATTRIBUTE_TYPE_ENTITY
                | XML_ROLE::ATTRIBUTE_TYPE_ENTITIES
                | XML_ROLE::ATTRIBUTE_TYPE_NMTOKEN
                | XML_ROLE::ATTRIBUTE_TYPE_NMTOKENS => {
                    if self.m_dtd.keepProcessing.get() && self.m_handlers.hasAttlistDecl() {
                        handleDefault = false
                    }
                }
                XML_ROLE::CONTENT_ELEMENT
                | XML_ROLE::CONTENT_ELEMENT_OPT
                | XML_ROLE::CONTENT_ELEMENT_REP
                | XML_ROLE::CONTENT_ELEMENT_PLUS => {
                    if self.m_dtd.in_eldecl.get() {
                        let mut nxt: *const c_char = if quant == XML_Content_Quant::NONE {
                            next
                        } else {
                            next.offset(-((*enc).minBytesPerChar() as isize))
                        };

                        let mut el = match self.getElementType(enc_type, buf.as_buf_ref().with_end(nxt)) {
                            Some(el) => el,
                            None => return XML_Error::NO_MEMORY,
                        };

                        let mut scf = self.m_dtd.scaffold.borrow_mut();
                        let myindex = match scf.next_part() {
                            Some(myindex) => myindex,
                            None => return XML_Error::NO_MEMORY,
                        };
                        scf.scaffold[myindex].type_0 = XML_Content_Type::NAME;
                        scf.scaffold[myindex].quant = quant;
                        scf.scaffold[myindex].name = Some(el.name.clone());

                        let mut name_2 = &el.name[..];
                        let mut nameLen = 0;
                        loop {
                            nameLen = nameLen + 1;
                            if name_2[nameLen - 1] == 0 {
                                break;
                            }
                        }
                        self.m_dtd.contentStringLen.set(
                            self.m_dtd
                                .contentStringLen
                                .get()
                                .wrapping_add(nameLen as c_uint),
                        );
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = false
                        }
                    }
                }
                XML_ROLE::GROUP_CLOSE
                | XML_ROLE::GROUP_CLOSE_OPT
                | XML_ROLE::GROUP_CLOSE_REP
                | XML_ROLE::GROUP_CLOSE_PLUS => {
                    if self.m_dtd.in_eldecl.get() {
                        if self.m_handlers.hasElementDecl() {
                            handleDefault = false
                        }
                        let empty_scaffold = {
                            let mut scf = self.m_dtd.scaffold.borrow_mut();
                            let idx = scf.index.pop().unwrap();
                            scf.scaffold[idx].quant = quant;
                            scf.index.is_empty()
                        };
                        if empty_scaffold {
                            if !handleDefault {
                                let mut model = match self.build_model() {
                                    Ok(model) => model,
                                    Err(_) => return XML_Error::NO_MEMORY,
                                };
                                self.eventEndPP(enc_type).set(buf.as_ptr());
                                self.m_handlers.elementDecl(
                                    self.m_declElementType.as_ref().unwrap().name.as_ptr(),
                                    Box::into_raw(model) as *mut _,
                                );
                            }
                            self.m_dtd.in_eldecl.set(false);
                            self.m_dtd.contentStringLen.set(0);
                        }
                    }
                }
                XML_ROLE::DOCTYPE_PUBLIC_ID | XML_ROLE::ENTITY_PUBLIC_ID => {
                    if self.m_dtd.keepProcessing.get() && !self.m_declEntity.is_none() {
                        let successful = self.m_dtd.pool.store_c_string(
                            enc,
                            buf.as_buf_ref()
                                .inc_start((*enc).minBytesPerChar())
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar()),
                        );
                        if !successful {
                            return XML_Error::NO_MEMORY;
                        }
                        let mut tem = self.m_dtd.pool.finish_cells();
                        normalizePublicId(&tem);
                        self.m_declEntity
                            .as_mut()
                            .unwrap()
                            .publicId
                            .set(tem.as_ptr() as *const _);
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
                XML_ROLE::DOCTYPE_SYSTEM_ID | XML_ROLE::ENTITY_SYSTEM_ID => {
                    if self.m_dtd.keepProcessing.get() && !self.m_declEntity.is_none() {
                        let successful = self.m_dtd.pool.store_c_string(
                            enc,
                            buf.as_buf_ref()
                                .inc_start((*enc).minBytesPerChar())
                                .with_end(next)
                                .dec_end((*enc).minBytesPerChar()),
                        );
                        let mut declEntity = self.m_declEntity.as_deref().unwrap();
                        if !successful {
                            declEntity.systemId.set(ptr::null());
                            return XML_Error::NO_MEMORY;
                        }
                        declEntity.systemId.set(self.m_dtd.pool.finish().as_ptr());
                        declEntity.base.set(self.m_curBase);
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
                reportDefault(self, enc_type, buf.as_buf_ref().with_end(next));
            }
            match self.m_parsingStatus.parsing {
                XML_Parsing::SUSPENDED => {
                    *nextPtr = next;
                    return XML_Error::NONE;
                }
                XML_Parsing::FINISHED => return XML_Error::ABORTED,
                _ => {
                    let mut next_idx = 0;
                    buf = buf.with_start(next);
                    tok = (*enc).xmlTok(XML_STATE::PROLOG, buf.as_buf_ref(), &mut next_idx);
                    next = buf.clone().inc_start(next_idx).as_ptr();
                }
            }
        }
        /* not reached */
    }
}
/* XML_DTD */

unsafe extern "C" fn epilogProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatSliceRc,
    mut nextPtr: &mut *const c_char,
) -> XML_Error {
    (*parser).m_processor = Some(epilogProcessor as Processor);
    (*parser).m_eventPtr.set(buf.as_ptr());
    loop {
        let mut next_idx = 0;
        let mut tok = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf.as_buf_ref(), &mut next_idx);
        let mut next = buf.clone().inc_start(next_idx).as_ptr();
        (*parser).m_eventEndPtr.set(next);
        match tok {
            XML_TOK::PROLOG_S_NEG => {
                /* report partial linebreak - it might be the last token */
                if (*parser).m_handlers.hasDefault() {
                    reportDefault(parser, EncodingType::Normal, buf.as_buf_ref().with_end(next));
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
                    reportDefault(parser, EncodingType::Normal, buf.as_buf_ref().with_end(next));
                }
            }
            XML_TOK::PI => {
                if reportProcessingInstruction(parser, EncodingType::Normal, buf.as_buf_ref().with_end(next))
                    == 0
                {
                    return XML_Error::NO_MEMORY;
                }
            }
            XML_TOK::COMMENT => {
                if reportComment(parser, EncodingType::Normal, buf.as_buf_ref().with_end(next)) == 0 {
                    return XML_Error::NO_MEMORY;
                }
            }
            XML_TOK::INVALID => {
                (*parser).m_eventPtr.set(next);
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
        (*parser).m_eventPtr.set(buf.as_ptr());
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

impl XML_ParserStruct {
    unsafe fn processInternalEntity(
        &mut self,
        mut entity: &Rc<Entity>,
        mut betweenDecl: XML_Bool,
    ) -> XML_Error {
        let mut next: *const c_char = ptr::null();
        let mut next_idx;
        let mut result: XML_Error = XML_Error::NONE;
        let mut openEntity = if let Some(mut openEntity) = self.m_freeInternalEntities.take() {
            self.m_freeInternalEntities = openEntity.next.take();
            openEntity
        } else {
            match Box::try_new(Default::default()) {
                Ok(openEntity) => openEntity,
                Err(_) => return XML_Error::NO_MEMORY,
            }
        };
        let openEntityPtr = openEntity.deref_mut() as *mut _;
        entity.open.set(true);
        entity.processed.set(0);
        openEntity.entity = Some(Rc::clone(entity));
        openEntity.startTagLevel = self.m_tagLevel;
        openEntity.betweenDecl = betweenDecl;
        openEntity.internalEventPtr.set(ptr::null());
        openEntity.internalEventEndPtr.set(ptr::null());
        openEntity.next = self.m_openInternalEntities.take();
        self.m_openInternalEntities = Some(openEntity);
        let text_slice_ref = entity.textPtr.borrow();
        let text_slice = text_slice_ref.as_ref().unwrap();
        let text_buf = ExpatBufRef::from(&text_slice[..]);
        /* Set a safe default value in case 'next' does not get set */
        next = text_buf.as_ptr();
        if entity.is_param.get() {
            next_idx = 0;
            let mut tok = (*self.m_internalEncoding).xmlTok(XML_STATE::PROLOG, text_buf, &mut next_idx);
            next = text_buf.inc_start(next_idx).as_ptr();
            result = self.doProlog(
                EncodingType::Internal,
                RcRef::clone(text_slice).into(),
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
                RcRef::clone(text_slice).into(),
                &mut next,
                false,
            )
        }
        if result == XML_Error::NONE {
            if text_buf.end() != next && self.m_parsingStatus.parsing == XML_Parsing::SUSPENDED {
                entity
                    .processed
                    .set(next.wrapping_offset_from(text_buf.as_ptr()) as i32);
                self.m_processor = Some(internalEntityProcessor as Processor)
            } else {
                entity.open.set(false);
                let openEntity = if cfg!(feature = "mozilla") {
                    // REXPAT FIXME: the original expat code identifies the open entity
                    // by comparing pointers; for `Rc` we'd use `Rc::ptr_eq` but
                    // `Box` doesn't have that
                    let opt_box_ptr = |opt_box: &mut Option<Box<OpenInternalEntity>>| {
                        opt_box
                            .as_mut()
                            .map(|b| b.deref_mut() as *mut OpenInternalEntity)
                            .unwrap_or_else(ptr::null_mut)
                    };
                    if opt_box_ptr(&mut self.m_openInternalEntities) == openEntityPtr {
                        let mut openEntity = self.m_openInternalEntities.take();
                        if let Some(openEntity) = openEntity.as_mut() {
                            self.m_openInternalEntities = openEntity.next.take();
                        }
                        openEntity
                    } else {
                        /* openEntity should be closed, but it contains an inner entity that is
                        still open. Remove openEntity from the openInternalEntities linked
                        list by looking for the inner entity in the list that links to
                        openEntity and fixing up its 'next' member
                         */
                        let mut openEntity = None;
                        let mut innerOpenEntities = self.m_openInternalEntities.as_mut();
                        while let Some(innerOpenEntity) = innerOpenEntities {
                            if opt_box_ptr(&mut innerOpenEntity.next) == openEntityPtr {
                                openEntity = innerOpenEntity.next.take();
                                if let Some(openEntity) = openEntity.as_mut() {
                                    innerOpenEntity.next = openEntity.next.take();
                                }
                                break;
                            }
                            innerOpenEntities = innerOpenEntity.next.as_mut();
                        }
                        openEntity
                    }
                } else {
                    let mut openEntity = self.m_openInternalEntities.take();
                    if let Some(openEntity) = openEntity.as_mut() {
                        self.m_openInternalEntities = openEntity.next.take();
                    }
                    openEntity
                };
                if let Some(mut openEntity) = openEntity {
                    /* put openEntity back in list of free instances */
                    openEntity.next = self.m_freeInternalEntities.take();
                    self.m_freeInternalEntities = Some(openEntity);
                }
            }
        }
        result
    }
}

unsafe extern "C" fn internalEntityProcessor(
    mut parser: XML_Parser,
    mut buf: ExpatSliceRc,
    mut nextPtr: &mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = ptr::null();
    let mut next_idx;
    let mut result: XML_Error = XML_Error::NONE;
    let mut openEntity = match (*parser).m_openInternalEntities.as_mut() {
        Some(openEntity) => openEntity,
        None => return XML_Error::UNEXPECTED_STATE,
    };
    let mut entity = openEntity.entity.as_deref().unwrap();
    let text_slice_ref = entity.textPtr.borrow();
    let text_slice = text_slice_ref.as_ref().unwrap().clone();
    let text_slice = ExpatSliceRc::from(text_slice)
        .inc_start(entity.processed.get().try_into().unwrap());
    let text_slice_end = text_slice.end();
    /* Set a safe default value in case 'next' does not get set */
    next = text_slice.as_ptr();
    if entity.is_param.get() {
        next_idx = 0;
        let mut tok =
            (*(*parser).m_internalEncoding).xmlTok(XML_STATE::PROLOG, text_slice.as_buf_ref(), &mut next_idx);
        next = text_slice.clone().inc_start(next_idx).as_ptr();
        result = (*parser).doProlog(
            EncodingType::Internal,
            text_slice,
            tok,
            next,
            &mut next,
            false,
            true,
        )
    } else {
        /* XML_DTD */
        result = (*parser).doContent(
            openEntity.startTagLevel,
            EncodingType::Internal,
            text_slice,
            &mut next,
            false,
        )
    }
    if result != XML_Error::NONE {
        return result;
    } else {
        if text_slice_end != next && (*parser).m_parsingStatus.parsing == XML_Parsing::SUSPENDED {
            entity
                .processed
                .set(next.wrapping_offset_from(entity.textPtr.borrow().as_deref().unwrap().as_ptr()) as i32);
            return result;
        } else {
            entity.open.set(false);
            /* put openEntity back in list of free instances */
            let next = std::mem::replace(
                &mut openEntity.next,
                (*parser).m_freeInternalEntities.take(),
            );
            (*parser).m_freeInternalEntities =
                std::mem::replace(&mut (*parser).m_openInternalEntities, next);
        }
    }
    if entity.is_param.get() {
        (*parser).m_processor = Some(prologProcessor as Processor);
        next_idx = 0;
        let tok_0 = (*(*parser).m_encoding).xmlTok(XML_STATE::PROLOG, buf.as_buf_ref(), &mut next_idx);
        next = buf.clone().inc_start(next_idx).as_ptr();
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
            if (*parser).is_child_parser {
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
    mut _buf: ExpatSliceRc,
    mut _nextPtr: &mut *const c_char,
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
    let enc = (*parser).encoding(enc_type);
    loop {
        let mut next_idx = 0;
        let mut tok = (*enc).xmlLiteralTok(XML_ATTRIBUTE_VALUE_LITERAL, buf, &mut next_idx);
        let mut next = buf.clone().inc_start(next_idx).as_ptr();
        match tok {
            XML_TOK::NONE => {
                return XML_Error::NONE;
                /* LCOV_EXCL_STOP */
            }
            XML_TOK::INVALID => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr.set(next);
                }
                return XML_Error::INVALID_TOKEN;
            }
            XML_TOK::PARTIAL => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr.set(buf.as_ptr());
                }
                return XML_Error::INVALID_TOKEN;
            }
            XML_TOK::CHAR_REF => {
                let mut out_buf: [XML_Char; XML_ENCODE_MAX] = [0; XML_ENCODE_MAX];
                let mut i: c_int = 0;
                let mut n: c_int = (*enc).charRefNumber(buf);
                if n < 0 {
                    if !enc_type.is_internal() {
                        (*parser).m_eventPtr.set(buf.as_ptr());
                    }
                    return XML_Error::BAD_CHAR_REF;
                }
                // FIXME: simplify this if-then-else stmt
                if !isCdata
                    && n == 0x20
                    && (pool.is_empty() || pool.get_last_char() as c_int == 0x20)
                {
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
            XML_TOK::TRAILING_CR | XML_TOK::ATTRIBUTE_VALUE_S | XML_TOK::DATA_NEWLINE => {
                if tok == XML_TOK::TRAILING_CR {
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
                    buf.inc_start((*enc).minBytesPerChar())
                        .with_end(next)
                        .dec_end((*enc).minBytesPerChar()),
                ) as XML_Char;
                if ch != 0 {
                    if !pool.append_char(ch) {
                        return XML_Error::NO_MEMORY;
                    }
                } else {
                    let successful = (*parser).m_temp2Pool.store_c_string(
                        enc,
                        buf.inc_start((*enc).minBytesPerChar())
                            .with_end(next)
                            .dec_end((*enc).minBytesPerChar()),
                    );
                    if !successful {
                        return XML_Error::NO_MEMORY;
                    }

                    let entity = (*parser).m_temp2Pool.with_current_slice(|key| {
                        let mut dtd_tables = (*parser).m_dtd.tables.borrow_mut();
                        dtd_tables.generalEntities.get(key).map(Rc::clone)
                    });

                    (*parser).m_temp2Pool.discard();

                    /* First, determine if a check for an existing declaration is needed;
                       if yes, check that the entity exists, and that it is internal.
                    */
                    if ptr::eq(pool, &(*parser).m_dtd.pool) {
                        /* are we called from prolog? */
                        checkEntityDecl = (*parser).m_prologState.documentEntity != 0
                            && (if (*parser).m_dtd.standalone.get() {
                                (*parser).m_openInternalEntities.is_none()
                            } else {
                                !(*parser).m_dtd.hasParamEntityRefs.get()
                            })
                    } else {
                        /* if (pool == &parser->m_tempPool): we are called from content */
                        checkEntityDecl = !(*parser).m_dtd.hasParamEntityRefs.get()
                            || (*parser).m_dtd.standalone.get()
                    }
                    if checkEntityDecl {
                        if let Some(entity) = entity.as_deref() {
                            if !entity.is_internal.get() {
                                return XML_Error::ENTITY_DECLARED_IN_PE;
                            }
                        } else {
                            return XML_Error::UNDEFINED_ENTITY;
                        }
                    } else if entity.is_none() {
                        if cfg!(feature = "mozilla") {
                            return XML_Error::UNDEFINED_ENTITY;
                        }
                    }

                    if let Some(entity) = entity {
                        if entity.open.get() {
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
                                (*parser).m_eventPtr.set(buf.as_ptr());
                                /* LCOV_EXCL_LINE */
                            }
                            return XML_Error::RECURSIVE_ENTITY_REF;
                        }
                        if !entity.notation.get().is_null() {
                            if !enc_type.is_internal() {
                                (*parser).m_eventPtr.set(buf.as_ptr());
                            }
                            return XML_Error::BINARY_ENTITY_REF;
                        }
                        if entity.textPtr.borrow().is_none() {
                            if !enc_type.is_internal() {
                                (*parser).m_eventPtr.set(buf.as_ptr());
                            }
                            return XML_Error::ATTRIBUTE_EXTERNAL_ENTITY_REF;
                        } else {
                            let mut result: XML_Error = XML_Error::NONE;
                            let text_slice = entity.textPtr.borrow();
                            entity.open.set(true);
                            result = appendAttributeValue(
                                parser,
                                EncodingType::Internal,
                                isCdata,
                                text_slice.as_deref().unwrap().into(),
                                pool,
                            );
                            entity.open.set(false);
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
                    (*parser).m_eventPtr.set(buf.as_ptr());
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
    let mut result: XML_Error = XML_Error::NONE;
    let mut oldInEntityValue: c_int = (*parser).m_prologState.inEntityValue;
    let enc = (*parser).encoding(enc_type);
    (*parser).m_prologState.inEntityValue = 1;
    /* XML_DTD */
    /* never return Null for the value argument in EntityDeclHandler,
    since this would indicate an external entity; therefore we
    have to make sure that entityValuePool.start is not null */
    's_41: loop {
        let mut next_idx = 0;
        let mut tok = (*enc).xmlLiteralTok(XML_ENTITY_VALUE_LITERAL, entityTextBuf, &mut next_idx);
        let mut next = entityTextBuf[next_idx..].as_ptr();
        match tok {
            XML_TOK::PARAM_ENTITY_REF => {
                if (*parser).m_isParamEntity || enc_type.is_internal() {
                    let successful = (*parser).m_tempPool.store_c_string(
                        enc,
                        entityTextBuf
                            .inc_start((*enc).minBytesPerChar())
                            .with_end(next)
                            .dec_end((*enc).minBytesPerChar()),
                    );
                    if !successful {
                        result = XML_Error::NO_MEMORY;
                        break;
                    } else {
                        let mut entity = match (*parser).m_tempPool.with_current_slice(|key| {
                            let mut dtd_tables = (*parser).m_dtd.tables.borrow_mut();
                            dtd_tables.paramEntities.get(key).map(Rc::clone)
                        }) {
                            Some(entity) => entity,
                            None => {
                                /* not a well-formedness error - see XML 1.0: WFC Entity Declared */
                                /* cannot report skipped entity here - see comments on
                                   parser->m_skippedEntityHandler
                                if (parser->m_skippedEntityHandler)
                                  parser->m_skippedEntityHandler(parser->m_handlerArg, name, 0);
                                */
                                (*parser)
                                    .m_dtd
                                    .keepProcessing
                                    .set((*parser).m_dtd.standalone.get());
                                break;
                            }
                        };
                        (*parser).m_tempPool.discard();
                        if entity.open.get() {
                            if !enc_type.is_internal() {
                                (*parser).m_eventPtr.set(entityTextBuf.as_ptr());
                            }
                            result = XML_Error::RECURSIVE_ENTITY_REF;
                            break;
                        } else if !entity.systemId.get().is_null() {
                            if (*parser).m_handlers.hasExternalEntityRef() {
                                (*parser).m_dtd.paramEntityRead.set(false);
                                entity.open.set(true);
                                if (*parser).m_handlers.externalEntityRef(
                                    ptr::null(),
                                    entity.base.get(),
                                    entity.systemId.get(),
                                    entity.publicId.get(),
                                ) == Ok(0)
                                {
                                    entity.open.set(false);
                                    result = XML_Error::EXTERNAL_ENTITY_HANDLING;
                                    break;
                                } else {
                                    entity.open.set(false);
                                    if !(*parser).m_dtd.paramEntityRead.get() {
                                        (*parser)
                                            .m_dtd
                                            .keepProcessing
                                            .set((*parser).m_dtd.standalone.get());
                                    }
                                }
                            } else {
                                (*parser)
                                    .m_dtd
                                    .keepProcessing
                                    .set((*parser).m_dtd.standalone.get());
                            }
                        } else {
                            let text_slice = entity.textPtr.borrow();
                            entity.open.set(true);
                            result = storeEntityValue(
                                parser,
                                EncodingType::Internal,
                                text_slice.as_deref().unwrap().into(),
                            );
                            entity.open.set(false);
                            if result as u64 != 0 {
                                break;
                            }
                        }
                    }
                } else {
                    /* XML_DTD */
                    /* In the internal subset, PE references are not legal
                    within markup declarations, e.g entity values in this case. */
                    (*parser).m_eventPtr.set(entityTextBuf.as_ptr());
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
                if !(*parser)
                    .m_dtd
                    .entityValuePool
                    .append(enc, entityTextBuf.with_end(next))
                {
                    result = XML_Error::NO_MEMORY;
                    break;
                }
            }
            XML_TOK::TRAILING_CR | XML_TOK::DATA_NEWLINE => {
                if tok == XML_TOK::TRAILING_CR {
                    next = entityTextBuf
                        .as_ptr()
                        .offset((*enc).minBytesPerChar() as isize);
                }

                if !(*parser).m_dtd.entityValuePool.append_char(0xa) {
                    result = XML_Error::NO_MEMORY;
                    break;
                }
            }
            XML_TOK::CHAR_REF => {
                let mut out_buf: [XML_Char; XML_ENCODE_MAX] = [0; XML_ENCODE_MAX];
                let mut n: c_int = (*enc).charRefNumber(entityTextBuf);
                if n < 0 {
                    if !enc_type.is_internal() {
                        (*parser).m_eventPtr.set(entityTextBuf.as_ptr());
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
                    let n = n.try_into().unwrap();
                    for &c in &out_buf[..n] {
                        if !(*parser).m_dtd.entityValuePool.append_char(c) {
                            result = XML_Error::NO_MEMORY;
                            break 's_41;
                        }
                    }
                }
            }
            XML_TOK::PARTIAL => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr.set(entityTextBuf.as_ptr());
                }
                result = XML_Error::INVALID_TOKEN;
                break;
            }
            XML_TOK::INVALID => {
                if !enc_type.is_internal() {
                    (*parser).m_eventPtr.set(next);
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
                    (*parser).m_eventPtr.set(entityTextBuf.as_ptr());
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

fn normalizeLines(s: &mut [XML_Char]) {
    let mut i = 0;
    loop {
        if s[i] == '\u{0}' as XML_Char {
            return;
        }
        if s[i] as c_int == 0xd {
            break;
        }
        i += 1;
    }
    let mut j = i;
    loop {
        if s[i] as c_int == 0xd {
            s[j] = 0xa;
            i += 1;
            j += 1;
            if s[i] as c_int == 0xa {
                i += 1;
            }
        } else {
            s[j] = s[i];
            i += 1;
            j += 1;
        }
        if s[i] == 0 {
            break;
        }
    }
    s[j] = '\u{0}' as XML_Char;
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
    buf = buf.inc_start((*enc).minBytesPerChar() * 2);
    let buf_name_len = (*enc).nameLength(buf).try_into().unwrap();
    let tem = buf.inc_start(buf_name_len);
    let successful = (*parser)
        .m_tempPool
        .store_c_string(enc, buf.with_len(buf_name_len));

    if !successful {
        return 0;
    }
    let target = (*parser).m_tempPool.finish_cells().as_ptr();
    let successful = (*parser).m_tempPool.store_c_string(
        enc,
        // TODO(SJC): fix this ugliness
        (*enc).skipS(tem).with_end(
            tem.end().offset(-(((*enc).minBytesPerChar() * 2) as isize))
        ),
    );
    if !successful {
        return 0;
    }
    (*parser).m_tempPool.with_current_mut_slice(|data| {
        normalizeLines(data);
        (*parser)
            .m_handlers
            .processingInstruction(target as *mut _, data.as_ptr());
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
        buf.inc_start((*enc).minBytesPerChar() * 4)
            .dec_end(((*enc).minBytesPerChar() * 3) as usize),
    );
    if !successful {
        return 0;
    };
    (*parser).m_tempPool.with_current_mut_slice(|data| {
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
        loop {
            let (convert_res, buf_count, data_count) =
                XmlConvert!(enc, buf, &mut (*parser).m_dataBuf[..]);
            buf = buf.inc_start(buf_count);
            (*parser).eventEndPP(enc_type).set(buf.as_ptr());

            let defaultRan = (*parser).m_handlers.default(
                (*parser).m_dataBuf.as_ptr(),
                data_count.try_into().unwrap(),
            );

            // Previously unwrapped an Option
            if !defaultRan {
                panic!("Expected default handler to be set");
            }

            (*parser).eventPP(enc_type).set(buf.as_ptr());
            if !(convert_res != super::xmltok::XML_Convert_Result::COMPLETED
                && convert_res != super::xmltok::XML_Convert_Result::INPUT_INCOMPLETE)
            {
                break;
            }
        }
    } else {
        let buf: ExpatBufRef<XML_Char> = buf.into();
        let defaultRan = (*parser)
            .m_handlers
            .default(buf.as_ptr(), buf.len().try_into().unwrap());

        // Previously unwrapped an Option
        if !defaultRan {
            panic!("Expected default handler to be set");
        }
    };
}

unsafe extern "C" fn defineAttribute(
    mut type_0: &Rc<ElementType>,
    mut attId: &Rc<AttributeId>,
    mut isCdata: XML_Bool,
    mut isId: XML_Bool,
    mut value: *const XML_Char,
) -> c_int {
    let mut default_atts = type_0.defaultAtts.borrow_mut();
    if !value.is_null() || isId {
        /* The handling of default attributes gets messed up if we have
        a default which duplicates a non-default. */
        /* save one level of indirection */

        if default_atts.iter().any(|da| Rc::ptr_eq(attId, &da.id)) {
            return 1;
        }
        if isId && get_cell_ptr(&type_0.idAtt).is_none() && !attId.xmlns.get() {
            type_0.idAtt.set(Some(Rc::clone(attId)));
        }
    }

    if default_atts.try_reserve(1).is_ok() {
        if !isCdata {
            attId.maybeTokenized.set(true);
        }

        let att = DefaultAttribute {
            id: Rc::clone(attId),
            isCdata,
            value,
        };
        default_atts.push(att);
        1
    } else {
        0
    }
}

impl XML_ParserStruct {
    fn setElementTypePrefix(&mut self, mut elementType: &ElementType) -> c_int {
        let mut dtd_tables = self.m_dtd.tables.borrow_mut();
        let mut name = &elementType.name[..];
        while name[0] != 0 {
            if name[0] == ASCII_COLON as XML_Char {
                let mut s = &elementType.name[..];
                while s.as_ptr() != name.as_ptr() {
                    if !self.m_dtd.pool.append_char(s[0]) {
                        return 0;
                    }
                    s = &s[1..];
                }
                if !self.m_dtd.pool.append_char('\u{0}' as XML_Char) {
                    return 0;
                }
                // This is unsafe, start needs be very temporary
                let prefix =
                    hash_insert_pool!(dtd_tables.prefixes, &self.m_dtd.pool, Prefix).cloned();
                if prefix.is_err() {
                    return 0;
                }
                elementType.prefix.set(prefix.into());
                break;
            } else {
                name = &name[1..];
            }
        }
        1
    }

    fn getAttributeId(
        &mut self,
        mut enc_type: EncodingType,
        mut buf: ExpatBufRef,
    ) -> Ptr<AttributeId> {
        let mut dtd_tables = self.m_dtd.tables.borrow_mut();
        let enc = self.encoding(enc_type);
        if !self.m_dtd.pool.store_c_string(enc, buf) {
            return None;
        }
        let id = hash_insert_pool!(dtd_tables.attributeIds, &self.m_dtd.pool, AttributeId).cloned();
        match id {
            HashInsertResult::Err => None,
            HashInsertResult::Found(id) => Some(id),
            HashInsertResult::New(id) => {
                if !self.m_dtd.pool.append_char(AttributeType::Unset.into()) {
                    return None;
                }
                let r#type = self.m_dtd.pool.finish_cells();
                assert!(r#type.len() == 1);
                *id.r#type.borrow_mut() = Some(r#type);

                let name = id.name();
                if self.m_ns {
                    if name[0] == ASCII_x as XML_Char
                        && name[1] == ASCII_m as XML_Char
                        && name[2] == ASCII_l as XML_Char
                        && name[3] == ASCII_n as XML_Char
                        && name[4] == ASCII_s as XML_Char
                        && (name[5] == '\u{0}' as XML_Char || name[5] == ASCII_COLON as XML_Char)
                    {
                        if name[5] == '\u{0}' as XML_Char {
                            *id.prefix.borrow_mut() =
                                dtd_tables.defaultPrefix.as_ref().map(Rc::downgrade);
                        } else {
                            let prefix = hash_insert!(
                                &mut dtd_tables.prefixes,
                                name.clone().map(|s| &s[6..]),
                                Rc::try_new,
                                Prefix
                            );
                            *id.prefix.borrow_mut() = Option::from(prefix).map(Rc::downgrade);
                        }
                        id.xmlns.set(true);
                    } else {
                        let mut i = 0;
                        while name[i] != 0 {
                            /* attributes without prefix are *not* in the default namespace */
                            if name[i] == ASCII_COLON as XML_Char {
                                for j in 0..i {
                                    if !self.m_dtd.pool.append_char(name[j]) {
                                        return None;
                                    }
                                }
                                if !self.m_dtd.pool.append_char('\u{0}' as XML_Char) {
                                    return None;
                                }
                                let prefix = hash_insert_pool!(
                                    dtd_tables.prefixes,
                                    &self.m_dtd.pool,
                                    Prefix
                                );
                                match prefix {
                                    HashInsertResult::New(prefix)
                                    | HashInsertResult::Found(prefix) => {
                                        *id.prefix.borrow_mut() = Some(Rc::downgrade(prefix));
                                    }
                                    HashInsertResult::Err => return None,
                                }
                                break;
                            } else {
                                i += 1
                            }
                        }
                    }
                }
                Some(id)
            }
        }
    }
}

const CONTEXT_SEP: XML_Char = ASCII_FF as XML_Char;

impl XML_ParserStruct {
    fn getContext(&mut self) -> bool {
        let mut dtd_tables = self.m_dtd.tables.borrow_mut();
        let mut needSep = false;
        if let Some(dpb) = get_cell_ptr(&dtd_tables.defaultPrefix.as_deref().unwrap().binding) {
            if !self.m_tempPool.append_char(ASCII_EQUALS as XML_Char) {
                return false;
            }
            let mut len = dpb.uriLen.get();
            if self.m_namespaceSeparator != 0 {
                len -= 1;
            }
            let uri = dpb.uri.borrow();
            for &c in &uri[..len] {
                if !self.m_tempPool.append_char(c) {
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
            }
            needSep = true;
        }
        for (pk, prefix) in dtd_tables.prefixes.iter_mut()
        /* This test appears to be (justifiable) paranoia.  There does
         * not seem to be a way of injecting a prefix without a binding
         * that doesn't get errored long before this function is called.
         * The test should remain for safety's sake, so we instead
         * exclude the following line from the coverage statistics.
         */
        {
            let mut len_0 = 0;
            if let Some(binding) = get_cell_ptr(&prefix.binding) {
                if needSep && !self.m_tempPool.append_char(CONTEXT_SEP) {
                    return false;
                }
                for &c in pk.iter() {
                    if c == 0 {
                        break;
                    }
                    if !self.m_tempPool.append_char(c) {
                        return false;
                    }
                }
                if !self.m_tempPool.append_char(ASCII_EQUALS as XML_Char) {
                    return false;
                }
                len_0 = binding.uriLen.get();
                if self.m_namespaceSeparator != 0 {
                    len_0 -= 1;
                }
                let uri_vec = binding.uri.borrow();
                for &c in &uri_vec[..len_0] {
                    if !self.m_tempPool.append_char(c) {
                        return false;
                    }
                }
                needSep = true
            }
        }
        for (ek, e) in dtd_tables.generalEntities.iter() {
            if !e.open.get() {
                continue;
            }
            if needSep && !self.m_tempPool.append_char(CONTEXT_SEP) {
                return false;
            }
            // TODO: Could the following be replaced by m_tempPool.append_c_string((*e).name)?
            for &ck_0 in &ek[..] {
                if ck_0 == 0 {
                    break;
                }
                if !self.m_tempPool.append_char(ck_0) {
                    return false;
                }
            }
            needSep = true
        }
        if !self.m_tempPool.append_char('\u{0}' as XML_Char) {
            return false;
        }

        true
    }
}

impl XML_ParserStruct {
    unsafe fn setContext(&mut self, mut context: *const XML_Char) -> XML_Bool {
        let mut s: *const XML_Char = context;
        while *context != '\u{0}' as XML_Char {
            // FIXME(rexpat): would be nice if we didn't need to borrow once per iteration,
            // but we need to drop it early before `addBinding`
            let mut dtd_tables = self.m_dtd.tables.borrow_mut();
            if *s == CONTEXT_SEP || *s == '\u{0}' as XML_Char {
                if !self.m_tempPool.append_char('\u{0}' as XML_Char) {
                    return false;
                }
                self.m_tempPool.with_current_slice(|key| {
                    if let Some(e) = dtd_tables.generalEntities.get(key) {
                        e.open.set(true);
                    }
                    if *s != '\u{0}' as XML_Char {
                        s = s.offset(1);
                    }
                    context = s;
                });
                self.m_tempPool.discard();
            } else if *s == ASCII_EQUALS as XML_Char {
                let prefix = if self.m_tempPool.is_empty() {
                    Rc::clone(dtd_tables.defaultPrefix.as_ref().unwrap())
                } else {
                    if !self.m_tempPool.append_char('\u{0}' as XML_Char) {
                        return false;
                    }
                    let prefix = match self.m_tempPool.with_current_slice(|prefix_name| {
                        if let Some(prefix) = dtd_tables.prefixes.get(prefix_name) {
                            return HashInsertResult::Found(Rc::clone(prefix));
                        }

                        // REXPAT change: we need to copy the prefix name
                        // into the DTD pool, since the HashMap keeps a permanent
                        // reference to the name which we can't modify after
                        // the call to `hash_insert!` (unlike the original C code)
                        if !self.m_dtd.pool.copy_c_string(prefix_name.as_ptr()) {
                            return HashInsertResult::Err;
                        }
                        hash_insert_pool!(dtd_tables.prefixes, &self.m_dtd.pool, Prefix).cloned()
                    }) {
                        HashInsertResult::New(prefix) | HashInsertResult::Found(prefix) => prefix,
                        HashInsertResult::Err => return false,
                    };
                    self.m_tempPool.discard();
                    prefix
                };
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
                drop(dtd_tables);

                let err = self.m_tempPool.with_current_slice(|uri| {
                    addBinding(
                        self,
                        &prefix,
                        None,
                        uri.as_ptr(),
                        &mut *self.m_inheritedBindings.borrow_mut(),
                    )
                });
                if err != XML_Error::NONE {
                    return false;
                }
                self.m_tempPool.discard();
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

fn normalizePublicId(publicId: &StringPoolCellSlice) {
    let mut p = &publicId[..];
    let mut s = &publicId[..];
    let mut last_p: Option<&Cell<XML_Char>> = None;
    while s[0].get() != 0 {
        match s[0].get() {
            32 | 13 | 10 => {
                if last_p.map_or(false, |x| x.get() != 0x20) {
                    p[0].set(0x20);

                    last_p = Some(&p[0]);
                    p = &p[1..];
                }
            }
            _ => {
                p[0].set(s[0].get());

                last_p = Some(&p[0]);
                p = &p[1..];
            }
        }
        s = &s[1..];
    }
    if let Some(last_p) = last_p {
        if last_p.get() == 0x20 {
            last_p.set('\u{0}' as XML_Char);
            return;
        }
    }
    p[0].set('\u{0}' as XML_Char);
}

unsafe extern "C" fn copyEntityTable(
    mut newTable: &mut HashMap<StringPoolSlice, Rc<Entity>>,
    mut newPool: &StringPool,
    mut oldTable: &HashMap<StringPoolSlice, Rc<Entity>>,
) -> c_int {
    let mut cachedOldBase: *const XML_Char = ptr::null();
    let mut cachedNewBase: *const XML_Char = ptr::null();
    for oldE in oldTable.values() {
        // TODO: direct safe copy instead of as_ptr()
        if !newPool.copy_c_string((*oldE).name.borrow().as_ref().unwrap().as_ptr()) {
            return 0;
        }
        let newE = match hash_insert_pool!(newTable, &newPool, Entity) {
            HashInsertResult::New(newE) => newE,
            HashInsertResult::Found(_) => panic!("Entity already found"),
            HashInsertResult::Err => return 0,
        };
        if !oldE.systemId.get().is_null() {
            if !newPool.copy_c_string(oldE.systemId.get()) {
                return 0;
            }
            let tem = newPool.finish();
            newE.systemId.set(tem.as_ptr());
            if !oldE.base.get().is_null() {
                if oldE.base.get() == cachedOldBase {
                    newE.base.set(cachedNewBase);
                } else {
                    cachedOldBase = oldE.base.get();
                    if !newPool.copy_c_string(cachedOldBase) {
                        return 0;
                    }
                    let tem = newPool.finish();
                    newE.base.set(tem.as_ptr());
                    cachedNewBase = tem.as_ptr();
                }
            }
            if !oldE.publicId.get().is_null() {
                if !newPool.copy_c_string(oldE.publicId.get()) {
                    return 0;
                }
                let tem = newPool.finish();
                newE.publicId.set(tem.as_ptr());
            }
        } else {
            let text_slice_ref = oldE.textPtr.borrow();
            let text_slice = text_slice_ref.as_deref().unwrap();
            if !newPool.copy_c_string_n(
                text_slice.as_ptr(),
                text_slice.len().try_into().unwrap(),
            ) {
                return 0;
            }
            *newE.textPtr.borrow_mut() = Some(newPool.finish());
        }
        if !oldE.notation.get().is_null() {
            if !newPool.copy_c_string(oldE.notation.get()) {
                return 0;
            }
            let tem = newPool.finish();
            newE.notation.set(tem.as_ptr());
        }
        newE.is_param.set(oldE.is_param.get());
        newE.is_internal.set(oldE.is_internal.get());
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

unsafe fn strlen(mut s: *const XML_Char) -> size_t {
    let mut len: size_t = 0;
    while *s != 0 {
        s = s.offset(1);
        len = len.wrapping_add(1);
    }
    len
}

impl XML_ParserStruct {
    fn build_node<'a, 'b>(
        &self,
        src_node: usize,
        dest: &mut XML_Content,
        mut contpos: &'a mut [XML_Content],
        mut strpos: &'b mut [XML_Char],
    ) -> (&'a mut [XML_Content], &'b mut [XML_Char]) {
        let scf = RefCell::borrow(&self.m_dtd.scaffold);
        dest.type_0 = scf.scaffold[src_node].type_0;
        dest.quant = scf.scaffold[src_node].quant;
        if dest.type_0 == XML_Content_Type::NAME {
            dest.name = (*strpos).as_ptr() as *mut XML_Char;
            dest.numchildren = 0;
            dest.children = ptr::null_mut();

            let mut src = &scf.scaffold[src_node].name.as_ref().unwrap()[..];
            loop {
                let (first, rest) = strpos.split_first_mut().unwrap();
                *first = src[0];
                if *first == 0 {
                    return (contpos, rest);
                }
                src = &src[1..];
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

    fn build_model(&mut self) -> Result<Box<ContentModel>, ()> {
        let mut ret = ContentModel::new(Default::default());

        let scaffold_len = self.m_dtd.scaffold.borrow().scaffold.len();
        if scaffold_len > 0 {
            // The root is outside the Vec, so we need one less node
            let nodes_len = scaffold_len - 1;
            ret.nodes.try_reserve_exact(nodes_len).map_err(|_| ())?;
            ret.nodes.resize_with(nodes_len, Default::default);
        }

        let strings_len: usize = self.m_dtd.contentStringLen.get()
            .try_into()
            .unwrap();
        if strings_len > 0 {
            ret.strings.try_reserve_exact(strings_len).map_err(|_| ())?;
            ret.strings.resize(strings_len, 0);
        }

        self.build_node(
            0,
            &mut ret.root,
            &mut ret.nodes[..],
            &mut ret.strings[..],
        );

        Box::try_new(ret).map_err(|_| ())
    }

    fn getElementType(
        &mut self,
        mut enc_type: EncodingType,
        mut buf: ExpatBufRef,
    ) -> Ptr<ElementType> {
        let mut dtd_tables = self.m_dtd.tables.borrow_mut();
        let enc = self.encoding(enc_type);
        if !self.m_dtd.pool.store_c_string(enc, buf) {
            return None;
        }
        let ret =
            hash_insert_pool!(dtd_tables.elementTypes, &self.m_dtd.pool, ElementType).cloned();
        match ret {
            HashInsertResult::Found(ret) => Some(ret),
            HashInsertResult::New(ret) => {
                drop(dtd_tables);
                if self.setElementTypePrefix(&ret) == 0 {
                    return None;
                }
                Some(ret)
            }
            HashInsertResult::Err => None,
        }
    }
}

unsafe fn slice_from_c_string<'a>(s: *const XML_Char) -> Option<&'a [XML_Char]> {
    if s.is_null() {
        return None;
    }

    let len = strlen(s);
    let sl = slice::from_raw_parts(s, len + 1);
    Some(sl)
}

fn copy_c_string(mut s: Option<&[XML_Char]>) -> Option<Vec<XML_Char>> {
    s.and_then(|s| {
        let mut result = Vec::new();
        if result.try_reserve_exact(s.len()).is_err() {
            return None;
        }
        result.extend_from_slice(s);
        Some(result)
    })
}
