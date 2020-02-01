use crate::expat_external_h::{XML_Char, XML_LChar, XML_Size};
pub use crate::lib::xmlparse::XML_ParserStruct;
use crate::lib::xmlparse::{XML_GetCurrentColumnNumber, XML_GetCurrentLineNumber};
use crate::stddef_h::size_t;
use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_void};
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
pub type XML_Parser = *mut XML_ParserStruct;
pub type XML_Bool = c_uchar;
pub const XML_TRUE: XML_Bool = 1;
pub const XML_FALSE: XML_Bool = 0;
/* The XML_Status enum gives the possible return values for several
   API functions.  The preprocessor #defines are included so this
   stanza can be added to code that still needs to support older
   versions of Expat 1.95.x:

   #ifndef XML_STATUS_OK
   #define XML_STATUS_OK    1
   #define XML_STATUS_ERROR 0
   #endif

   Otherwise, the #define hackery is quite ugly and would have been
   dropped.
*/
pub type XML_Status = c_uint;
pub const XML_STATUS_ERROR: XML_Status = 0;
pub const XML_STATUS_ERROR_0: c_int = XML_STATUS_ERROR as c_int;
pub const XML_STATUS_OK: XML_Status = 1;
pub const XML_STATUS_OK_0: c_int = XML_STATUS_OK as c_int;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;
pub const XML_STATUS_SUSPENDED_0: c_int = XML_STATUS_SUSPENDED as c_int;
pub type XML_Error = c_uint;
pub const XML_ERROR_NONE: XML_Error = 0;
pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
pub const XML_ERROR_SYNTAX: XML_Error = 2;
pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
/* Added in 1.95.7. */
pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
/* Added in 1.95.8. */
pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
pub const XML_ERROR_XML_DECL: XML_Error = 30;
pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
pub const XML_ERROR_PUBLICID: XML_Error = 32;
pub const XML_ERROR_SUSPENDED: XML_Error = 33;
pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
pub const XML_ERROR_ABORTED: XML_Error = 35;
pub const XML_ERROR_FINISHED: XML_Error = 36;
pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
/* Added in 2.0. */
pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
/* Added in 2.2.1. */
pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
pub type XML_Content_Type = c_uint;
pub const XML_CTYPE_EMPTY: XML_Content_Type = 1;
pub const XML_CTYPE_ANY: XML_Content_Type = 2;
pub const XML_CTYPE_MIXED: XML_Content_Type = 3;
pub const XML_CTYPE_NAME: XML_Content_Type = 4;
pub const XML_CTYPE_CHOICE: XML_Content_Type = 5;
pub const XML_CTYPE_SEQ: XML_Content_Type = 6;
pub type XML_Content_Quant = c_uint;
pub const XML_CQUANT_NONE: XML_Content_Quant = 0;
pub const XML_CQUANT_OPT: XML_Content_Quant = 1;
pub const XML_CQUANT_REP: XML_Content_Quant = 2;
pub const XML_CQUANT_PLUS: XML_Content_Quant = 3;
/* If type == XML_CTYPE_EMPTY or XML_CTYPE_ANY, then quant will be
   XML_CQUANT_NONE, and the other fields will be zero or NULL.
   If type == XML_CTYPE_MIXED, then quant will be NONE or REP and
   numchildren will contain number of elements that may be mixed in
   and children point to an array of XML_Content cells that will be
   all of XML_CTYPE_NAME type with no quantification.

   If type == XML_CTYPE_NAME, then the name points to the name, and
   the numchildren field will be zero and children will be NULL. The
   quant fields indicates any quantifiers placed on the name.

   CHOICE and SEQ will have name NULL, the number of children in
   numchildren and children will point, recursively, to an array
   of XML_Content cells.

   The EMPTY, ANY, and MIXED types will only occur at top level.
*/
pub type XML_Content = XML_cp;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XML_cp {
    pub type_0: XML_Content_Type,
    pub quant: XML_Content_Quant,
    pub name: *mut XML_Char,
    pub numchildren: c_uint,
    pub children: *mut XML_Content,
}
/* This is called for an element declaration. See above for
   description of the model argument. It's the caller's responsibility
   to free model when finished with it.
*/
pub type XML_ElementDeclHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: *mut XML_Content) -> ()>;
/* The Attlist declaration handler is called for *each* attribute. So
   a single Attlist declaration with multiple attributes declared will
   generate multiple calls to this handler. The "default" parameter
   may be NULL in the case of the "#IMPLIED" or "#REQUIRED"
   keyword. The "isrequired" parameter will be true and the default
   value will be NULL in the case of "#REQUIRED". If "isrequired" is
   true and default is non-NULL, then this is a "#FIXED" default.
*/
pub type XML_AttlistDeclHandler = Option<
    unsafe extern "C" fn(
        _: *mut c_void,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: c_int,
    ) -> (),
>;
/* The XML declaration handler is called for *both* XML declarations
   and text declarations. The way to distinguish is that the version
   parameter will be NULL for text declarations. The encoding
   parameter may be NULL for XML declarations. The standalone
   parameter will be -1, 0, or 1 indicating respectively that there
   was no standalone parameter in the declaration, that it was given
   as no, or that it was given as yes.
*/
pub type XML_XmlDeclHandler = Option<
    unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: *const XML_Char, _: c_int) -> (),
>;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct XML_Memory_Handling_Suite {
    pub malloc_fcn: Option<unsafe extern "C" fn(_: size_t) -> *mut c_void>,
    pub realloc_fcn: Option<unsafe extern "C" fn(_: *mut c_void, _: size_t) -> *mut c_void>,
    pub free_fcn: Option<unsafe extern "C" fn(_: *mut c_void) -> ()>,
}
/* atts is array of name/value pairs, terminated by 0;
   names and values are 0 terminated.
*/
pub type XML_StartElementHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: *mut *const XML_Char) -> ()>;
pub type XML_EndElementHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()>;
/* s is not 0 terminated. */
pub type XML_CharacterDataHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> ()>;
/* target and data are 0 terminated */
pub type XML_ProcessingInstructionHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: *const XML_Char) -> ()>;
/* data is 0 terminated */
pub type XML_CommentHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()>;
pub type XML_StartCdataSectionHandler = Option<unsafe extern "C" fn(_: *mut c_void) -> ()>;
pub type XML_EndCdataSectionHandler = Option<unsafe extern "C" fn(_: *mut c_void) -> ()>;
/* This is called for any characters in the XML document for which
   there is no applicable handler.  This includes both characters that
   are part of markup which is of a kind that is not reported
   (comments, markup declarations), or characters that are part of a
   construct which could be reported but for which no handler has been
   supplied. The characters are passed exactly as they were in the XML
   document except that they will be encoded in UTF-8 or UTF-16.
   Line boundaries are not normalized. Note that a byte order mark
   character is not passed to the default handler. There are no
   guarantees about how characters are divided between calls to the
   default handler: for example, a comment might be split between
   multiple calls.
*/
pub type XML_DefaultHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> ()>;
/* This is called for the start of the DOCTYPE declaration, before
   any DTD or internal subset is parsed.
*/
pub type XML_StartDoctypeDeclHandler = Option<
    unsafe extern "C" fn(
        _: *mut c_void,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: c_int,
    ) -> (),
>;
/* This is called for the start of the DOCTYPE declaration when the
   closing > is encountered, but after processing any external
   subset.
*/
pub type XML_EndDoctypeDeclHandler = Option<unsafe extern "C" fn(_: *mut c_void) -> ()>;
/* This is called for entity declarations. The is_parameter_entity
   argument will be non-zero if the entity is a parameter entity, zero
   otherwise.

   For internal entities (<!ENTITY foo "bar">), value will
   be non-NULL and systemId, publicID, and notationName will be NULL.
   The value string is NOT nul-terminated; the length is provided in
   the value_length argument. Since it is legal to have zero-length
   values, do not use this argument to test for internal entities.

   For external entities, value will be NULL and systemId will be
   non-NULL. The publicId argument will be NULL unless a public
   identifier was provided. The notationName argument will have a
   non-NULL value only for unparsed entity declarations.

   Note that is_parameter_entity can't be changed to XML_Bool, since
   that would break binary compatibility.
*/
pub type XML_EntityDeclHandler = Option<
    unsafe extern "C" fn(
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
>;
/* OBSOLETE -- OBSOLETE -- OBSOLETE
   This handler has been superseded by the EntityDeclHandler above.
   It is provided here for backward compatibility.

   This is called for a declaration of an unparsed (NDATA) entity.
   The base argument is whatever was set by XML_SetBase. The
   entityName, systemId and notationName arguments will never be
   NULL. The other arguments may be.
*/
pub type XML_UnparsedEntityDeclHandler = Option<
    unsafe extern "C" fn(
        _: *mut c_void,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
    ) -> (),
>;
/* This is called for a declaration of notation.  The base argument is
   whatever was set by XML_SetBase. The notationName will never be
   NULL.  The other arguments can be.
*/
pub type XML_NotationDeclHandler = Option<
    unsafe extern "C" fn(
        _: *mut c_void,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
    ) -> (),
>;
/* When namespace processing is enabled, these are called once for
   each namespace declaration. The call to the start and end element
   handlers occur between the calls to the start and end namespace
   declaration handlers. For an xmlns attribute, prefix will be
   NULL.  For an xmlns="" attribute, uri will be NULL.
*/
pub type XML_StartNamespaceDeclHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: *const XML_Char) -> ()>;
pub type XML_EndNamespaceDeclHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> ()>;
/* This is called if the document is not standalone, that is, it has an
   external subset or a reference to a parameter entity, but does not
   have standalone="yes". If this handler returns XML_STATUS_ERROR,
   then processing will not continue, and the parser will return a
   XML_ERROR_NOT_STANDALONE error.
   If parameter entity parsing is enabled, then in addition to the
   conditions above this handler will only be called if the referenced
   entity was actually read.
*/
pub type XML_NotStandaloneHandler = Option<unsafe extern "C" fn(_: *mut c_void) -> c_int>;
/* This is called for a reference to an external parsed general
   entity.  The referenced entity is not automatically parsed.  The
   application can parse it immediately or later using
   XML_ExternalEntityParserCreate.

   The parser argument is the parser parsing the entity containing the
   reference; it can be passed as the parser argument to
   XML_ExternalEntityParserCreate.  The systemId argument is the
   system identifier as specified in the entity declaration; it will
   not be NULL.

   The base argument is the system identifier that should be used as
   the base for resolving systemId if systemId was relative; this is
   set by XML_SetBase; it may be NULL.

   The publicId argument is the public identifier as specified in the
   entity declaration, or NULL if none was specified; the whitespace
   in the public identifier will have been normalized as required by
   the XML spec.

   The context argument specifies the parsing context in the format
   expected by the context argument to XML_ExternalEntityParserCreate;
   context is valid only until the handler returns, so if the
   referenced entity is to be parsed later, it must be copied.
   context is NULL only when the entity is a parameter entity.

   The handler should return XML_STATUS_ERROR if processing should not
   continue because of a fatal error in the handling of the external
   entity.  In this case the calling parser will return an
   XML_ERROR_EXTERNAL_ENTITY_HANDLING error.

   Note that unlike other handlers the first argument is the parser,
   not userData.
*/
pub type XML_ExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        _: XML_Parser,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
        _: *const XML_Char,
    ) -> c_int,
>;
/* This is called in two situations:
   1) An entity reference is encountered for which no declaration
  has been read *and* this is not an error.
   2) An internal entity reference is read, but not expanded, because
  XML_SetDefaultHandler has been called.
   Note: skipped parameter entities in declarations and skipped general
     entities in attribute values cannot be reported, because
     the event would be out of sync with the reporting of the
     declarations or attribute values
*/
pub type XML_SkippedEntityHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> ()>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XML_Encoding {
    pub map: [c_int; 256],
    pub data: *mut c_void,
    pub convert: Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int>,
    pub release: Option<unsafe extern "C" fn(_: *mut c_void) -> ()>,
}
/* This is called for an encoding that is unknown to the parser.

   The encodingHandlerData argument is that which was passed as the
   second argument to XML_SetUnknownEncodingHandler.

   The name argument gives the name of the encoding as specified in
   the encoding declaration.

   If the callback can provide information about the encoding, it must
   fill in the XML_Encoding structure, and return XML_STATUS_OK.
   Otherwise it must return XML_STATUS_ERROR.

   If info does not describe a suitable encoding, then the parser will
   return an XML_UNKNOWN_ENCODING error.
*/
pub type XML_UnknownEncodingHandler =
    Option<unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: *mut XML_Encoding) -> c_int>;
pub type XML_Parsing = c_uint;
pub const XML_INITIALIZED: XML_Parsing = 0;
pub const XML_PARSING: XML_Parsing = 1;
pub const XML_FINISHED: XML_Parsing = 2;
pub const XML_SUSPENDED: XML_Parsing = 3;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct XML_ParsingStatus {
    pub parsing: XML_Parsing,
    pub finalBuffer: XML_Bool,
}
pub type XML_ParamEntityParsing = c_uint;
pub const XML_PARAM_ENTITY_PARSING_NEVER: XML_ParamEntityParsing = 0;
pub const XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE: XML_ParamEntityParsing = 1;
pub const XML_PARAM_ENTITY_PARSING_ALWAYS: XML_ParamEntityParsing = 2;
pub const XML_GetErrorLineNumber: unsafe extern "C" fn(_: XML_Parser) -> XML_Size =
    XML_GetCurrentLineNumber;
pub const XML_GetErrorColumnNumber: unsafe extern "C" fn(_: XML_Parser) -> XML_Size =
    XML_GetCurrentColumnNumber;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XML_Expat_Version {
    pub major: c_int,
    pub minor: c_int,
    pub micro: c_int,
}

/* Added in Expat 1.95.5. */
pub type XML_FeatureEnum = c_uint;
pub const XML_FEATURE_END: XML_FeatureEnum = 0;
pub const XML_FEATURE_UNICODE: XML_FeatureEnum = 1;
pub const XML_FEATURE_UNICODE_WCHAR_T: XML_FeatureEnum = 2;
pub const XML_FEATURE_DTD: XML_FeatureEnum = 3;
pub const XML_FEATURE_CONTEXT_BYTES: XML_FeatureEnum = 4;
pub const XML_FEATURE_MIN_SIZE: XML_FeatureEnum = 5;
pub const XML_FEATURE_SIZEOF_XML_CHAR: XML_FeatureEnum = 6;
pub const XML_FEATURE_SIZEOF_XML_LCHAR: XML_FeatureEnum = 7;
pub const XML_FEATURE_NS: XML_FeatureEnum = 8;
pub const XML_FEATURE_LARGE_SIZE: XML_FeatureEnum = 9;
pub const XML_FEATURE_ATTR_INFO: XML_FeatureEnum = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XML_Feature {
    pub feature: XML_FeatureEnum,
    pub name: *const XML_LChar,
    pub value: c_long,
}
/* Additional features must be added to the end of this enum. */

