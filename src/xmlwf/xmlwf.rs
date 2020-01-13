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
    extern_types,
    label_break_value,
    main,
    ptr_wrapping_offset_from,
    register_tool
)]
pub mod expat_h {
    pub type XML_Parser = *mut ::c2rust_out::expat_h::XML_ParserStruct;
    /* atts is array of name/value pairs, terminated by 0;
       names and values are 0 terminated.
    */

    pub type XML_StartElementHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
            _: *mut *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_EndElementHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;
    /* s is not 0 terminated. */

    pub type XML_CharacterDataHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
            _: libc::c_int,
        ) -> (),
    >;
    /* target and data are 0 terminated */

    pub type XML_ProcessingInstructionHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
            _: *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;
    /* data is 0 terminated */

    pub type XML_CommentHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_StartCdataSectionHandler =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;

    pub type XML_EndCdataSectionHandler = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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

    pub type XML_DefaultHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
            _: libc::c_int,
        ) -> (),
    >;
    /* This is called for the start of the DOCTYPE declaration, before
       any DTD or internal subset is parsed.
    */

    pub type XML_StartDoctypeDeclHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
            _: *const crate::expat_external_h::XML_Char,
            _: *const crate::expat_external_h::XML_Char,
            _: libc::c_int,
        ) -> (),
    >;
    /* This is called for the start of the DOCTYPE declaration when the
       closing > is encountered, but after processing any external
       subset.
    */

    pub type XML_EndDoctypeDeclHandler = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
            _: libc::c_int,
            _: *const crate::expat_external_h::XML_Char,
            _: libc::c_int,
            _: *const crate::expat_external_h::XML_Char,
            _: *const crate::expat_external_h::XML_Char,
            _: *const crate::expat_external_h::XML_Char,
            _: *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;
    /* This is called for a declaration of notation.  The base argument is
       whatever was set by XML_SetBase. The notationName will never be
       NULL.  The other arguments can be.
    */

    pub type XML_NotationDeclHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
            _: *const crate::expat_external_h::XML_Char,
            _: *const crate::expat_external_h::XML_Char,
            _: *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;
    /* When namespace processing is enabled, these are called once for
       each namespace declaration. The call to the start and end element
       handlers occur between the calls to the start and end namespace
       declaration handlers. For an xmlns attribute, prefix will be
       NULL.  For an xmlns="" attribute, uri will be NULL.
    */

    pub type XML_StartNamespaceDeclHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
            _: *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_EndNamespaceDeclHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;
    /* This is called if the document is not standalone, that is, it has an
       external subset or a reference to a parameter entity, but does not
       have standalone="yes". If this handler returns XML_STATUS_ERROR,
       then processing will not continue, and the parser will return a
       XML_ERROR_NOT_STANDALONE error.
       If parameter entity parsing is enabled, then in addition to the
       conditions above this handler will only be called if the referenced
       entity was actually read.
    */

    pub type XML_NotStandaloneHandler =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
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

    pub type XML_UnknownEncodingHandler = Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const crate::expat_external_h::XML_Char,
            _: *mut ::c2rust_out::expat_h::XML_Encoding,
        ) -> libc::c_int,
    >;

    pub type XML_ParamEntityParsing = libc::c_uint;
    /* Added in Expat 1.95.5. */

    pub type XML_FeatureEnum = libc::c_uint;
}
pub mod expat_external_h {
    pub type XML_Char = libc::c_char;
    pub type XML_LChar = libc::c_char;
    /* XML_UNICODE */
    /* Use large integers for file/stream positions. */

    pub type XML_Index = libc::c_long;

    pub type XML_Size = libc::c_ulong;
}
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
}
pub mod xmltchar_h {
    pub const tfopen: unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> *mut crate::stdlib::FILE = crate::stdlib::fopen;

    pub const fputts: unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut crate::stdlib::FILE,
    ) -> libc::c_int = crate::stdlib::fputs;

    pub const puttc: unsafe extern "C" fn(
        _: libc::c_int,
        _: *mut crate::stdlib::FILE,
    ) -> libc::c_int = crate::stdlib::putc;

    pub const tcscmp: unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> libc::c_int = ::libc::strcmp;

    pub const tcscat: unsafe extern "C" fn(
        _: *mut libc::c_char,
        _: *const libc::c_char,
    ) -> *mut libc::c_char = ::libc::strcat;

    pub const tcschr: unsafe extern "C" fn(
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char = ::libc::strchr;

    pub const tremove: unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int = ::libc::remove;
}
pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub static mut stdout: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub static mut stderr: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn fclose(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn setvbuf(
            __stream: *mut crate::stdlib::FILE,
            __buf: *mut libc::c_char,
            __modes: libc::c_int,
            __n: crate::stddef_h::size_t,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn putc(__c: libc::c_int, __stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut crate::stdlib::FILE) -> libc::c_int;
        #[no_mangle]
        pub fn qsort(
            __base: *mut libc::c_void,
            __nmemb: crate::stddef_h::size_t,
            __size: crate::stddef_h::size_t,
            __compar: crate::stdlib::__compar_fn_t,
        );
        pub type _IO_marker;

        pub type _IO_codecvt;

        pub type _IO_wide_data;
    }
    pub type FILE = ::c2rust_out::stdlib::_IO_FILE;
    pub const __ASSERT_FUNCTION: [libc::c_char; 46] = unsafe {
        *::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
            b"void attributeValue(FILE *, const XML_Char *)\x00",
        )
    };
    pub type __compar_fn_t =
        Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
    pub type _IO_lock_t = ();
    pub type __off_t = libc::c_long;

    pub type __off64_t = libc::c_long;
}

pub use crate::expat_external_h::{XML_Char, XML_Index, XML_LChar, XML_Size};
pub use crate::expat_h::{
    XML_CharacterDataHandler, XML_CommentHandler, XML_DefaultHandler, XML_EndCdataSectionHandler,
    XML_EndDoctypeDeclHandler, XML_EndElementHandler, XML_EndNamespaceDeclHandler,
    XML_EntityDeclHandler, XML_FeatureEnum, XML_NotStandaloneHandler, XML_NotationDeclHandler,
    XML_ParamEntityParsing, XML_Parser, XML_ProcessingInstructionHandler,
    XML_StartCdataSectionHandler, XML_StartDoctypeDeclHandler, XML_StartElementHandler,
    XML_StartNamespaceDeclHandler, XML_UnknownEncodingHandler,
};
pub use crate::stddef_h::size_t;
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __compar_fn_t, __off64_t, __off_t, fclose,
    fopen, fputs, putc, qsort, setvbuf, stderr, stdout, FILE, __ASSERT_FUNCTION,
};
pub use crate::xmltchar_h::{fputts, puttc, tcscat, tcschr, tcscmp, tfopen, tremove};


#[repr(C)]
#[derive(Copy, Clone)]
pub struct NotationList {
    pub next: *mut NotationList,
    pub notationName: *const crate::expat_external_h::XML_Char,
    pub systemId: *const crate::expat_external_h::XML_Char,
    pub publicId: *const crate::expat_external_h::XML_Char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xmlwfUserData {
    pub fp: *mut crate::stdlib::FILE,
    pub notationListHead: *mut NotationList,
    pub currentDoctypeName: *const crate::expat_external_h::XML_Char,
}

pub type XmlwfUserData = xmlwfUserData;
/* This ensures proper sorting. */

unsafe extern "C" fn characterData(
    mut userData: *mut libc::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: libc::c_int,
) {
    let mut fp: *mut crate::stdlib::FILE = (*(userData as *mut XmlwfUserData)).fp;
    while len > 0 as libc::c_int {
        match *s as libc::c_int {
            38 => {
                crate::stdlib::fputs(b"&amp;\x00" as *const u8 as *const libc::c_char, fp);
            }
            60 => {
                crate::stdlib::fputs(b"&lt;\x00" as *const u8 as *const libc::c_char, fp);
            }
            62 => {
                crate::stdlib::fputs(b"&gt;\x00" as *const u8 as *const libc::c_char, fp);
            }
            34 => {
                crate::stdlib::fputs(b"&quot;\x00" as *const u8 as *const libc::c_char, fp);
            }
            9 | 10 | 13 => {
                ::c2rust_out::stdlib::fprintf(
                    fp as *mut ::c2rust_out::stdlib::_IO_FILE,
                    b"&#%d;\x00" as *const u8 as *const libc::c_char,
                    *s as libc::c_int,
                );
            }
            _ => {
                crate::stdlib::putc(*s as libc::c_int, fp);
            }
        }
        len -= 1;
        s = s.offset(1)
    }
}

unsafe extern "C" fn attributeValue(
    mut fp: *mut crate::stdlib::FILE,
    mut s: *const crate::expat_external_h::XML_Char,
) {
    crate::stdlib::putc('=' as i32, fp);
    crate::stdlib::putc('\"' as i32, fp);
    if !s.is_null() {
    } else {
        ::c2rust_out::stdlib::__assert_fail(
            b"s\x00" as *const u8 as *const libc::c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/xmlwf/xmlwf.c\x00" as *const u8
                as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            crate::stdlib::__ASSERT_FUNCTION.as_ptr(),
        );
    }
    loop {
        match *s as libc::c_int {
            0 | 1 => {
                crate::stdlib::putc('\"' as i32, fp);
                return;
            }
            38 => {
                crate::stdlib::fputs(b"&amp;\x00" as *const u8 as *const libc::c_char, fp);
            }
            60 => {
                crate::stdlib::fputs(b"&lt;\x00" as *const u8 as *const libc::c_char, fp);
            }
            34 => {
                crate::stdlib::fputs(b"&quot;\x00" as *const u8 as *const libc::c_char, fp);
            }
            62 => {
                crate::stdlib::fputs(b"&gt;\x00" as *const u8 as *const libc::c_char, fp);
            }
            9 | 10 | 13 => {
                ::c2rust_out::stdlib::fprintf(
                    fp as *mut ::c2rust_out::stdlib::_IO_FILE,
                    b"&#%d;\x00" as *const u8 as *const libc::c_char,
                    *s as libc::c_int,
                );
            }
            _ => {
                crate::stdlib::putc(*s as libc::c_int, fp);
            }
        }
        s = s.offset(1)
    }
}
/* Lexicographically comparing UTF-8 encoded attribute values,
is equivalent to lexicographically comparing based on the character number. */

unsafe extern "C" fn attcmp(
    mut att1: *const libc::c_void,
    mut att2: *const libc::c_void,
) -> libc::c_int {
    return ::libc::strcmp(
        *(att1 as *mut *const crate::expat_external_h::XML_Char),
        *(att2 as *mut *const crate::expat_external_h::XML_Char),
    );
}

unsafe extern "C" fn startElement(
    mut userData: *mut libc::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    let mut nAtts: libc::c_int = 0;
    let mut p: *mut *const crate::expat_external_h::XML_Char =
        0 as *mut *const crate::expat_external_h::XML_Char;
    let mut fp: *mut crate::stdlib::FILE = (*(userData as *mut XmlwfUserData)).fp;
    crate::stdlib::putc('<' as i32, fp);
    crate::stdlib::fputs(name, fp);
    p = atts;
    while !(*p).is_null() {
        p = p.offset(1)
    }
    nAtts = (p.wrapping_offset_from(atts) as libc::c_long >> 1 as libc::c_int) as libc::c_int;
    if nAtts > 1 as libc::c_int {
        crate::stdlib::qsort(
            atts as *mut libc::c_void,
            nAtts as crate::stddef_h::size_t,
            (::std::mem::size_of::<*mut crate::expat_external_h::XML_Char>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            Some(
                attcmp
                    as unsafe extern "C" fn(
                        _: *const libc::c_void,
                        _: *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
    while !(*atts).is_null() {
        crate::stdlib::putc(' ' as i32, fp);
        let fresh0 = atts;
        atts = atts.offset(1);
        crate::stdlib::fputs(*fresh0, fp);
        attributeValue(fp, *atts);
        atts = atts.offset(1)
    }
    crate::stdlib::putc('>' as i32, fp);
}

unsafe extern "C" fn endElement(
    mut userData: *mut libc::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
) {
    let mut fp: *mut crate::stdlib::FILE = (*(userData as *mut XmlwfUserData)).fp;
    crate::stdlib::putc('<' as i32, fp);
    crate::stdlib::putc('/' as i32, fp);
    crate::stdlib::fputs(name, fp);
    crate::stdlib::putc('>' as i32, fp);
}

unsafe extern "C" fn nsattcmp(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut att1: *const crate::expat_external_h::XML_Char =
        *(p1 as *mut *const crate::expat_external_h::XML_Char);
    let mut att2: *const crate::expat_external_h::XML_Char =
        *(p2 as *mut *const crate::expat_external_h::XML_Char);
    let mut sep1: libc::c_int =
        (::libc::strrchr(att1, '\u{1}' as i32) != 0 as *mut libc::c_char) as libc::c_int;
    let mut sep2: libc::c_int =
        (::libc::strrchr(att1, '\u{1}' as i32) != 0 as *mut libc::c_char) as libc::c_int;
    if sep1 != sep2 {
        return sep1 - sep2;
    }
    return ::libc::strcmp(att1, att2);
}

unsafe extern "C" fn startElementNS(
    mut userData: *mut libc::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    let mut nAtts: libc::c_int = 0;
    let mut nsi: libc::c_int = 0;
    let mut p: *mut *const crate::expat_external_h::XML_Char =
        0 as *mut *const crate::expat_external_h::XML_Char;
    let mut fp: *mut crate::stdlib::FILE = (*(userData as *mut XmlwfUserData)).fp;
    let mut sep: *const crate::expat_external_h::XML_Char =
        0 as *const crate::expat_external_h::XML_Char;
    crate::stdlib::putc('<' as i32, fp);
    sep = ::libc::strrchr(name, '\u{1}' as i32);
    if !sep.is_null() {
        crate::stdlib::fputs(b"n1:\x00" as *const u8 as *const libc::c_char, fp);
        crate::stdlib::fputs(sep.offset(1 as libc::c_int as isize), fp);
        crate::stdlib::fputs(b" xmlns:n1\x00" as *const u8 as *const libc::c_char, fp);
        attributeValue(fp, name);
        nsi = 2 as libc::c_int
    } else {
        crate::stdlib::fputs(name, fp);
        nsi = 1 as libc::c_int
    }
    p = atts;
    while !(*p).is_null() {
        p = p.offset(1)
    }
    nAtts = (p.wrapping_offset_from(atts) as libc::c_long >> 1 as libc::c_int) as libc::c_int;
    if nAtts > 1 as libc::c_int {
        crate::stdlib::qsort(
            atts as *mut libc::c_void,
            nAtts as crate::stddef_h::size_t,
            (::std::mem::size_of::<*mut crate::expat_external_h::XML_Char>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            Some(
                nsattcmp
                    as unsafe extern "C" fn(
                        _: *const libc::c_void,
                        _: *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
    while !(*atts).is_null() {
        let fresh1 = atts;
        atts = atts.offset(1);
        name = *fresh1;
        sep = ::libc::strrchr(name, '\u{1}' as i32);
        crate::stdlib::putc(' ' as i32, fp);
        if !sep.is_null() {
            ::c2rust_out::stdlib::fprintf(
                fp as *mut ::c2rust_out::stdlib::_IO_FILE,
                b"n%d:\x00" as *const u8 as *const libc::c_char,
                nsi,
            );
            crate::stdlib::fputs(sep.offset(1 as libc::c_int as isize), fp);
        } else {
            crate::stdlib::fputs(name, fp);
        }
        attributeValue(fp, *atts);
        if !sep.is_null() {
            let fresh2 = nsi;
            nsi = nsi + 1;
            ::c2rust_out::stdlib::fprintf(
                fp as *mut ::c2rust_out::stdlib::_IO_FILE,
                b" xmlns:n%d\x00" as *const u8 as *const libc::c_char,
                fresh2,
            );
            attributeValue(fp, name);
        }
        atts = atts.offset(1)
    }
    crate::stdlib::putc('>' as i32, fp);
}

unsafe extern "C" fn endElementNS(
    mut userData: *mut libc::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
) {
    let mut fp: *mut crate::stdlib::FILE = (*(userData as *mut XmlwfUserData)).fp;
    let mut sep: *const crate::expat_external_h::XML_Char =
        0 as *const crate::expat_external_h::XML_Char;
    crate::stdlib::putc('<' as i32, fp);
    crate::stdlib::putc('/' as i32, fp);
    sep = ::libc::strrchr(name, '\u{1}' as i32);
    if !sep.is_null() {
        crate::stdlib::fputs(b"n1:\x00" as *const u8 as *const libc::c_char, fp);
        crate::stdlib::fputs(sep.offset(1 as libc::c_int as isize), fp);
    } else {
        crate::stdlib::fputs(name, fp);
    }
    crate::stdlib::putc('>' as i32, fp);
}

unsafe extern "C" fn processingInstruction(
    mut userData: *mut libc::c_void,
    mut target: *const crate::expat_external_h::XML_Char,
    mut data: *const crate::expat_external_h::XML_Char,
) {
    let mut fp: *mut crate::stdlib::FILE = (*(userData as *mut XmlwfUserData)).fp;
    crate::stdlib::putc('<' as i32, fp);
    crate::stdlib::putc('?' as i32, fp);
    crate::stdlib::fputs(target, fp);
    crate::stdlib::putc(' ' as i32, fp);
    crate::stdlib::fputs(data, fp);
    crate::stdlib::putc('?' as i32, fp);
    crate::stdlib::putc('>' as i32, fp);
}

unsafe extern "C" fn xcsdup(
    mut s: *const crate::expat_external_h::XML_Char,
) -> *mut crate::expat_external_h::XML_Char {
    let mut result: *mut crate::expat_external_h::XML_Char =
        0 as *mut crate::expat_external_h::XML_Char;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut numBytes: libc::c_int = 0;
    loop
    /* Get the length of the string, including terminator */
    {
        let fresh3 = count;
        count = count + 1;
        if !(*s.offset(fresh3 as isize) as libc::c_int != 0 as libc::c_int) {
            break;
        }
    }
    numBytes = (count as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong)
        as libc::c_int;
    result = ::c2rust_out::stdlib::malloc(numBytes as libc::c_ulong)
        as *mut crate::expat_external_h::XML_Char;
    if result.is_null() {
        return ::c2rust_out::stddef_h::NULL as *mut crate::expat_external_h::XML_Char;
    }
    ::c2rust_out::stdlib::memcpy(
        result as *mut libc::c_void,
        s as *const libc::c_void,
        numBytes as libc::c_ulong,
    );
    return result;
}

unsafe extern "C" fn startDoctypeDecl(
    mut userData: *mut libc::c_void,
    mut doctypeName: *const crate::expat_external_h::XML_Char,
    mut _sysid: *const crate::expat_external_h::XML_Char,
    mut _publid: *const crate::expat_external_h::XML_Char,
    mut _has_internal_subset: libc::c_int,
) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    (*data).currentDoctypeName = xcsdup(doctypeName);
}

unsafe extern "C" fn freeNotations(mut data: *mut XmlwfUserData) {
    let mut notationListHead: *mut NotationList = (*data).notationListHead;
    while !notationListHead.is_null() {
        let mut next: *mut NotationList = (*notationListHead).next;
        ::libc::free((*notationListHead).notationName as *mut libc::c_void);
        ::libc::free((*notationListHead).systemId as *mut libc::c_void);
        ::libc::free((*notationListHead).publicId as *mut libc::c_void);
        ::libc::free(notationListHead as *mut libc::c_void);
        notationListHead = next
    }
    (*data).notationListHead = ::c2rust_out::stddef_h::NULL as *mut NotationList;
}

unsafe extern "C" fn xcscmp(
    mut xs: *const crate::expat_external_h::XML_Char,
    mut xt: *const crate::expat_external_h::XML_Char,
) -> libc::c_int {
    while *xs as libc::c_int != 0 as libc::c_int && *xt as libc::c_int != 0 as libc::c_int {
        if (*xs as libc::c_int) < *xt as libc::c_int {
            return -(1 as libc::c_int);
        }
        if *xs as libc::c_int > *xt as libc::c_int {
            return 1 as libc::c_int;
        }
        xs = xs.offset(1);
        xt = xt.offset(1)
    }
    if (*xs as libc::c_int) < *xt as libc::c_int {
        return -(1 as libc::c_int);
    }
    if *xs as libc::c_int > *xt as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

unsafe extern "C" fn notationCmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let n1: *const NotationList = *(a as *mut *mut NotationList);
    let n2: *const NotationList = *(b as *mut *mut NotationList);
    return xcscmp((*n1).notationName, (*n2).notationName);
}

unsafe extern "C" fn endDoctypeDecl(mut userData: *mut libc::c_void) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    let mut notations: *mut *mut NotationList = 0 as *mut *mut NotationList;
    let mut notationCount: libc::c_int = 0 as libc::c_int;
    let mut p: *mut NotationList = 0 as *mut NotationList;
    let mut i: libc::c_int = 0;
    /* How many notations do we have? */
    p = (*data).notationListHead;
    while !p.is_null() {
        notationCount += 1;
        p = (*p).next
    }
    if notationCount == 0 as libc::c_int {
        /* Nothing to report */
        ::libc::free((*data).currentDoctypeName as *mut libc::c_void);
        (*data).currentDoctypeName =
            ::c2rust_out::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
        return;
    }
    notations = ::c2rust_out::stdlib::malloc(
        (notationCount as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut NotationList>() as libc::c_ulong),
    ) as *mut *mut NotationList;
    if notations.is_null() {
        ::c2rust_out::stdlib::fprintf(
            crate::stdlib::stderr as *mut ::c2rust_out::stdlib::_IO_FILE,
            b"Unable to sort notations\x00" as *const u8 as *const libc::c_char,
        );
        freeNotations(data);
        return;
    }
    p = (*data).notationListHead;
    i = 0 as libc::c_int;
    while i < notationCount {
        let ref mut fresh4 = *notations.offset(i as isize);
        *fresh4 = p;
        p = (*p).next;
        i += 1
    }
    crate::stdlib::qsort(
        notations as *mut libc::c_void,
        notationCount as crate::stddef_h::size_t,
        ::std::mem::size_of::<*mut NotationList>() as libc::c_ulong,
        Some(
            notationCmp
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    /* Output the DOCTYPE header */
    crate::stdlib::fputs(
        b"<!DOCTYPE \x00" as *const u8 as *const libc::c_char,
        (*data).fp,
    );
    crate::stdlib::fputs((*data).currentDoctypeName, (*data).fp);
    crate::stdlib::fputs(b" [\n\x00" as *const u8 as *const libc::c_char, (*data).fp);
    /* Now the NOTATIONs */
    i = 0 as libc::c_int;
    while i < notationCount {
        crate::stdlib::fputs(
            b"<!NOTATION \x00" as *const u8 as *const libc::c_char,
            (*data).fp,
        );
        crate::stdlib::fputs((**notations.offset(i as isize)).notationName, (*data).fp);
        if !(**notations.offset(i as isize)).publicId.is_null() {
            crate::stdlib::fputs(
                b" PUBLIC \'\x00" as *const u8 as *const libc::c_char,
                (*data).fp,
            );
            crate::stdlib::fputs((**notations.offset(i as isize)).publicId, (*data).fp);
            crate::stdlib::putc('\'' as i32, (*data).fp);
            if !(**notations.offset(i as isize)).systemId.is_null() {
                crate::stdlib::putc(' ' as i32, (*data).fp);
                crate::stdlib::putc('\'' as i32, (*data).fp);
                crate::stdlib::fputs((**notations.offset(i as isize)).systemId, (*data).fp);
                crate::stdlib::putc('\'' as i32, (*data).fp);
            }
        } else if !(**notations.offset(i as isize)).systemId.is_null() {
            crate::stdlib::fputs(
                b" SYSTEM \'\x00" as *const u8 as *const libc::c_char,
                (*data).fp,
            );
            crate::stdlib::fputs((**notations.offset(i as isize)).systemId, (*data).fp);
            crate::stdlib::putc('\'' as i32, (*data).fp);
        }
        crate::stdlib::putc('>' as i32, (*data).fp);
        crate::stdlib::putc('\n' as i32, (*data).fp);
        i += 1
    }
    /* Finally end the DOCTYPE */
    crate::stdlib::fputs(b"]>\n\x00" as *const u8 as *const libc::c_char, (*data).fp);
    ::libc::free(notations as *mut libc::c_void);
    freeNotations(data);
    ::libc::free((*data).currentDoctypeName as *mut libc::c_void);
    (*data).currentDoctypeName =
        ::c2rust_out::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
}

unsafe extern "C" fn notationDecl(
    mut userData: *mut libc::c_void,
    mut notationName: *const crate::expat_external_h::XML_Char,
    mut _base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    let mut entry: *mut NotationList = ::c2rust_out::stdlib::malloc(::std::mem::size_of::<
        NotationList,
    >() as libc::c_ulong) as *mut NotationList;
    let mut errorMessage: *const libc::c_char =
        b"Unable to store NOTATION for output\n\x00" as *const u8 as *const libc::c_char;
    if entry.is_null() {
        crate::stdlib::fputs(errorMessage, crate::stdlib::stderr);
        return;
        /* Nothing we can really do about this */
    } /* Safe if it's NULL */
    (*entry).notationName = xcsdup(notationName);
    if (*entry).notationName.is_null() {
        crate::stdlib::fputs(errorMessage, crate::stdlib::stderr);
        ::libc::free(entry as *mut libc::c_void);
        return;
    }
    if !systemId.is_null() {
        (*entry).systemId = xcsdup(systemId);
        if (*entry).systemId.is_null() {
            crate::stdlib::fputs(errorMessage, crate::stdlib::stderr);
            ::libc::free((*entry).notationName as *mut libc::c_void);
            ::libc::free(entry as *mut libc::c_void);
            return;
        }
    } else {
        (*entry).systemId = ::c2rust_out::stddef_h::NULL as *const crate::expat_external_h::XML_Char
    }
    if !publicId.is_null() {
        (*entry).publicId = xcsdup(publicId);
        if (*entry).publicId.is_null() {
            crate::stdlib::fputs(errorMessage, crate::stdlib::stderr);
            ::libc::free((*entry).systemId as *mut libc::c_void);
            ::libc::free((*entry).notationName as *mut libc::c_void);
            ::libc::free(entry as *mut libc::c_void);
            return;
        }
    } else {
        (*entry).publicId = ::c2rust_out::stddef_h::NULL as *const crate::expat_external_h::XML_Char
    }
    (*entry).next = (*data).notationListHead;
    (*data).notationListHead = entry;
}
/* not W3C14N */

unsafe extern "C" fn defaultCharacterData(
    mut userData: *mut libc::c_void,
    mut _s: *const crate::expat_external_h::XML_Char,
    mut _len: libc::c_int,
) {
    ::c2rust_out::src::lib::xmlparse::XML_DefaultCurrent(userData as crate::expat_h::XML_Parser);
}

unsafe extern "C" fn defaultStartElement(
    mut userData: *mut libc::c_void,
    mut _name: *const crate::expat_external_h::XML_Char,
    mut _atts: *mut *const crate::expat_external_h::XML_Char,
) {
    ::c2rust_out::src::lib::xmlparse::XML_DefaultCurrent(userData as crate::expat_h::XML_Parser);
}

unsafe extern "C" fn defaultEndElement(
    mut userData: *mut libc::c_void,
    mut _name: *const crate::expat_external_h::XML_Char,
) {
    ::c2rust_out::src::lib::xmlparse::XML_DefaultCurrent(userData as crate::expat_h::XML_Parser);
}

unsafe extern "C" fn defaultProcessingInstruction(
    mut userData: *mut libc::c_void,
    mut _target: *const crate::expat_external_h::XML_Char,
    mut _data: *const crate::expat_external_h::XML_Char,
) {
    ::c2rust_out::src::lib::xmlparse::XML_DefaultCurrent(userData as crate::expat_h::XML_Parser);
}

unsafe extern "C" fn nopCharacterData(
    mut _userData: *mut libc::c_void,
    mut _s: *const crate::expat_external_h::XML_Char,
    mut _len: libc::c_int,
) {
}

unsafe extern "C" fn nopStartElement(
    mut _userData: *mut libc::c_void,
    mut _name: *const crate::expat_external_h::XML_Char,
    mut _atts: *mut *const crate::expat_external_h::XML_Char,
) {
}

unsafe extern "C" fn nopEndElement(
    mut _userData: *mut libc::c_void,
    mut _name: *const crate::expat_external_h::XML_Char,
) {
}

unsafe extern "C" fn nopProcessingInstruction(
    mut _userData: *mut libc::c_void,
    mut _target: *const crate::expat_external_h::XML_Char,
    mut _data: *const crate::expat_external_h::XML_Char,
) {
}

unsafe extern "C" fn markup(
    mut userData: *mut libc::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: libc::c_int,
) {
    let mut fp: *mut crate::stdlib::FILE = (*(*(userData as crate::expat_h::XML_Parser
        as *mut *mut libc::c_void)
        as *mut XmlwfUserData))
        .fp;
    while len > 0 as libc::c_int {
        crate::stdlib::putc(*s as libc::c_int, fp);
        len -= 1;
        s = s.offset(1)
    }
}

unsafe extern "C" fn metaLocation(mut parser: crate::expat_h::XML_Parser) {
    let mut uri: *const crate::expat_external_h::XML_Char =
        ::c2rust_out::src::lib::xmlparse::XML_GetBase(parser);
    let mut fp: *mut crate::stdlib::FILE =
        (*(*(parser as *mut *mut libc::c_void) as *mut XmlwfUserData)).fp;
    if !uri.is_null() {
        ::c2rust_out::stdlib::fprintf(
            fp as *mut ::c2rust_out::stdlib::_IO_FILE,
            b" uri=\"%s\"\x00" as *const u8 as *const libc::c_char,
            uri,
        );
    }
    ::c2rust_out::stdlib::fprintf(
        fp as *mut ::c2rust_out::stdlib::_IO_FILE,
        b" byte=\"%ld\" nbytes=\"%d\" line=\"%lu\" col=\"%lu\"\x00" as *const u8
            as *const libc::c_char,
        ::c2rust_out::src::lib::xmlparse::XML_GetCurrentByteIndex(parser),
        ::c2rust_out::src::lib::xmlparse::XML_GetCurrentByteCount(parser),
        ::c2rust_out::src::lib::xmlparse::XML_GetCurrentLineNumber(parser),
        ::c2rust_out::src::lib::xmlparse::XML_GetCurrentColumnNumber(parser),
    );
}

unsafe extern "C" fn metaStartDocument(mut userData: *mut libc::c_void) {
    crate::stdlib::fputs(
        b"<document>\n\x00" as *const u8 as *const libc::c_char,
        (*(*(userData as crate::expat_h::XML_Parser as *mut *mut libc::c_void)
            as *mut XmlwfUserData))
            .fp,
    );
}

unsafe extern "C" fn metaEndDocument(mut userData: *mut libc::c_void) {
    crate::stdlib::fputs(
        b"</document>\n\x00" as *const u8 as *const libc::c_char,
        (*(*(userData as crate::expat_h::XML_Parser as *mut *mut libc::c_void)
            as *mut XmlwfUserData))
            .fp,
    );
}

unsafe extern "C" fn metaStartElement(
    mut userData: *mut libc::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    let mut specifiedAttsEnd: *mut *const crate::expat_external_h::XML_Char = atts
        .offset(::c2rust_out::src::lib::xmlparse::XML_GetSpecifiedAttributeCount(parser) as isize);
    let mut idAttPtr: *mut *const crate::expat_external_h::XML_Char =
        0 as *mut *const crate::expat_external_h::XML_Char;
    let mut idAttIndex: libc::c_int =
        ::c2rust_out::src::lib::xmlparse::XML_GetIdAttributeIndex(parser);
    if idAttIndex < 0 as libc::c_int {
        idAttPtr = 0 as *mut *const crate::expat_external_h::XML_Char
    } else {
        idAttPtr = atts.offset(idAttIndex as isize)
    }
    ::c2rust_out::stdlib::fprintf(
        fp as *mut ::c2rust_out::stdlib::_IO_FILE,
        b"<starttag name=\"%s\"\x00" as *const u8 as *const libc::c_char,
        name,
    );
    metaLocation(parser);
    if !(*atts).is_null() {
        crate::stdlib::fputs(b">\n\x00" as *const u8 as *const libc::c_char, fp);
        loop {
            ::c2rust_out::stdlib::fprintf(
                fp as *mut ::c2rust_out::stdlib::_IO_FILE,
                b"<attribute name=\"%s\" value=\"\x00" as *const u8 as *const libc::c_char,
                *atts.offset(0 as libc::c_int as isize),
            );
            characterData(
                data as *mut libc::c_void,
                *atts.offset(1 as libc::c_int as isize),
                ::c2rust_out::stdlib::strlen(*atts.offset(1 as libc::c_int as isize))
                    as libc::c_int,
            );
            if atts >= specifiedAttsEnd {
                crate::stdlib::fputs(
                    b"\" defaulted=\"yes\"/>\n\x00" as *const u8 as *const libc::c_char,
                    fp,
                );
            } else if atts == idAttPtr {
                crate::stdlib::fputs(
                    b"\" id=\"yes\"/>\n\x00" as *const u8 as *const libc::c_char,
                    fp,
                );
            } else {
                crate::stdlib::fputs(b"\"/>\n\x00" as *const u8 as *const libc::c_char, fp);
            }
            atts = atts.offset(2 as libc::c_int as isize);
            if (*atts).is_null() {
                break;
            }
        }
        crate::stdlib::fputs(b"</starttag>\n\x00" as *const u8 as *const libc::c_char, fp);
    } else {
        crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
    };
}

unsafe extern "C" fn metaEndElement(
    mut userData: *mut libc::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    ::c2rust_out::stdlib::fprintf(
        fp as *mut ::c2rust_out::stdlib::_IO_FILE,
        b"<endtag name=\"%s\"\x00" as *const u8 as *const libc::c_char,
        name,
    );
    metaLocation(parser);
    crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
}

unsafe extern "C" fn metaProcessingInstruction(
    mut userData: *mut libc::c_void,
    mut target: *const crate::expat_external_h::XML_Char,
    mut data: *const crate::expat_external_h::XML_Char,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut usrData: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*usrData).fp;
    ::c2rust_out::stdlib::fprintf(
        fp as *mut ::c2rust_out::stdlib::_IO_FILE,
        b"<pi target=\"%s\" data=\"\x00" as *const u8 as *const libc::c_char,
        target,
    );
    characterData(
        usrData as *mut libc::c_void,
        data,
        ::c2rust_out::stdlib::strlen(data) as libc::c_int,
    );
    crate::stdlib::putc('\"' as i32, fp);
    metaLocation(parser);
    crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
}

unsafe extern "C" fn metaComment(
    mut userData: *mut libc::c_void,
    mut data: *const crate::expat_external_h::XML_Char,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut usrData: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*usrData).fp;
    crate::stdlib::fputs(
        b"<comment data=\"\x00" as *const u8 as *const libc::c_char,
        fp,
    );
    characterData(
        usrData as *mut libc::c_void,
        data,
        ::c2rust_out::stdlib::strlen(data) as libc::c_int,
    );
    crate::stdlib::putc('\"' as i32, fp);
    metaLocation(parser);
    crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
}

unsafe extern "C" fn metaStartCdataSection(mut userData: *mut libc::c_void) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    crate::stdlib::fputs(b"<startcdata\x00" as *const u8 as *const libc::c_char, fp);
    metaLocation(parser);
    crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
}

unsafe extern "C" fn metaEndCdataSection(mut userData: *mut libc::c_void) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    crate::stdlib::fputs(b"<endcdata\x00" as *const u8 as *const libc::c_char, fp);
    metaLocation(parser);
    crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
}

unsafe extern "C" fn metaCharacterData(
    mut userData: *mut libc::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: libc::c_int,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    crate::stdlib::fputs(b"<chars str=\"\x00" as *const u8 as *const libc::c_char, fp);
    characterData(data as *mut libc::c_void, s, len);
    crate::stdlib::putc('\"' as i32, fp);
    metaLocation(parser);
    crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
}

unsafe extern "C" fn metaStartDoctypeDecl(
    mut userData: *mut libc::c_void,
    mut doctypeName: *const crate::expat_external_h::XML_Char,
    mut _sysid: *const crate::expat_external_h::XML_Char,
    mut _pubid: *const crate::expat_external_h::XML_Char,
    mut _has_internal_subset: libc::c_int,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    ::c2rust_out::stdlib::fprintf(
        fp as *mut ::c2rust_out::stdlib::_IO_FILE,
        b"<startdoctype name=\"%s\"\x00" as *const u8 as *const libc::c_char,
        doctypeName,
    );
    metaLocation(parser);
    crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
}

unsafe extern "C" fn metaEndDoctypeDecl(mut userData: *mut libc::c_void) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    crate::stdlib::fputs(b"<enddoctype\x00" as *const u8 as *const libc::c_char, fp);
    metaLocation(parser);
    crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
}

unsafe extern "C" fn metaNotationDecl(
    mut userData: *mut libc::c_void,
    mut notationName: *const crate::expat_external_h::XML_Char,
    mut _base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    ::c2rust_out::stdlib::fprintf(
        fp as *mut ::c2rust_out::stdlib::_IO_FILE,
        b"<notation name=\"%s\"\x00" as *const u8 as *const libc::c_char,
        notationName,
    );
    if !publicId.is_null() {
        ::c2rust_out::stdlib::fprintf(
            fp as *mut ::c2rust_out::stdlib::_IO_FILE,
            b" public=\"%s\"\x00" as *const u8 as *const libc::c_char,
            publicId,
        );
    }
    if !systemId.is_null() {
        crate::stdlib::fputs(b" system=\"\x00" as *const u8 as *const libc::c_char, fp);
        characterData(
            data as *mut libc::c_void,
            systemId,
            ::c2rust_out::stdlib::strlen(systemId) as libc::c_int,
        );
        crate::stdlib::putc('\"' as i32, fp);
    }
    metaLocation(parser);
    crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
}

unsafe extern "C" fn metaEntityDecl(
    mut userData: *mut libc::c_void,
    mut entityName: *const crate::expat_external_h::XML_Char,
    mut _is_param: libc::c_int,
    mut value: *const crate::expat_external_h::XML_Char,
    mut value_length: libc::c_int,
    mut _base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
    mut notationName: *const crate::expat_external_h::XML_Char,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    if !value.is_null() {
        ::c2rust_out::stdlib::fprintf(
            fp as *mut ::c2rust_out::stdlib::_IO_FILE,
            b"<entity name=\"%s\"\x00" as *const u8 as *const libc::c_char,
            entityName,
        );
        metaLocation(parser);
        crate::stdlib::putc('>' as i32, fp);
        characterData(data as *mut libc::c_void, value, value_length);
        crate::stdlib::fputs(b"</entity/>\n\x00" as *const u8 as *const libc::c_char, fp);
    } else if !notationName.is_null() {
        ::c2rust_out::stdlib::fprintf(
            fp as *mut ::c2rust_out::stdlib::_IO_FILE,
            b"<entity name=\"%s\"\x00" as *const u8 as *const libc::c_char,
            entityName,
        );
        if !publicId.is_null() {
            ::c2rust_out::stdlib::fprintf(
                fp as *mut ::c2rust_out::stdlib::_IO_FILE,
                b" public=\"%s\"\x00" as *const u8 as *const libc::c_char,
                publicId,
            );
        }
        crate::stdlib::fputs(b" system=\"\x00" as *const u8 as *const libc::c_char, fp);
        characterData(
            data as *mut libc::c_void,
            systemId,
            ::c2rust_out::stdlib::strlen(systemId) as libc::c_int,
        );
        crate::stdlib::putc('\"' as i32, fp);
        ::c2rust_out::stdlib::fprintf(
            fp as *mut ::c2rust_out::stdlib::_IO_FILE,
            b" notation=\"%s\"\x00" as *const u8 as *const libc::c_char,
            notationName,
        );
        metaLocation(parser);
        crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
    } else {
        ::c2rust_out::stdlib::fprintf(
            fp as *mut ::c2rust_out::stdlib::_IO_FILE,
            b"<entity name=\"%s\"\x00" as *const u8 as *const libc::c_char,
            entityName,
        );
        if !publicId.is_null() {
            ::c2rust_out::stdlib::fprintf(
                fp as *mut ::c2rust_out::stdlib::_IO_FILE,
                b" public=\"%s\"\x00" as *const u8 as *const libc::c_char,
                publicId,
            );
        }
        crate::stdlib::fputs(b" system=\"\x00" as *const u8 as *const libc::c_char, fp);
        characterData(
            data as *mut libc::c_void,
            systemId,
            ::c2rust_out::stdlib::strlen(systemId) as libc::c_int,
        );
        crate::stdlib::putc('\"' as i32, fp);
        metaLocation(parser);
        crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
    };
}

unsafe extern "C" fn metaStartNamespaceDecl(
    mut userData: *mut libc::c_void,
    mut prefix: *const crate::expat_external_h::XML_Char,
    mut uri: *const crate::expat_external_h::XML_Char,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    crate::stdlib::fputs(b"<startns\x00" as *const u8 as *const libc::c_char, fp);
    if !prefix.is_null() {
        ::c2rust_out::stdlib::fprintf(
            fp as *mut ::c2rust_out::stdlib::_IO_FILE,
            b" prefix=\"%s\"\x00" as *const u8 as *const libc::c_char,
            prefix,
        );
    }
    if !uri.is_null() {
        crate::stdlib::fputs(b" ns=\"\x00" as *const u8 as *const libc::c_char, fp);
        characterData(
            data as *mut libc::c_void,
            uri,
            ::c2rust_out::stdlib::strlen(uri) as libc::c_int,
        );
        crate::stdlib::fputs(b"\"/>\n\x00" as *const u8 as *const libc::c_char, fp);
    } else {
        crate::stdlib::fputs(b"/>\n\x00" as *const u8 as *const libc::c_char, fp);
    };
}

unsafe extern "C" fn metaEndNamespaceDecl(
    mut userData: *mut libc::c_void,
    mut prefix: *const crate::expat_external_h::XML_Char,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut libc::c_void) as *mut XmlwfUserData;
    let mut fp: *mut crate::stdlib::FILE = (*data).fp;
    if prefix.is_null() {
        crate::stdlib::fputs(b"<endns/>\n\x00" as *const u8 as *const libc::c_char, fp);
    } else {
        ::c2rust_out::stdlib::fprintf(
            fp as *mut ::c2rust_out::stdlib::_IO_FILE,
            b"<endns prefix=\"%s\"/>\n\x00" as *const u8 as *const libc::c_char,
            prefix,
        );
    };
}

unsafe extern "C" fn unknownEncodingConvert(
    mut data: *mut libc::c_void,
    mut p: *const libc::c_char,
) -> libc::c_int {
    return ::c2rust_out::src::xmlwf::codepage::codepageConvert(*(data as *mut libc::c_int), p);
}

unsafe extern "C" fn unknownEncoding(
    mut _userData: *mut libc::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut info: *mut ::c2rust_out::expat_h::XML_Encoding,
) -> libc::c_int {
    let mut cp: libc::c_int = 0;
    static mut prefixL: [crate::expat_external_h::XML_Char; 9] =
        [119, 105, 110, 100, 111, 119, 115, 45, 0];
    static mut prefixU: [crate::expat_external_h::XML_Char; 9] =
        [87, 73, 78, 68, 79, 87, 83, 45, 0];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while prefixU[i as usize] != 0 {
        if *name.offset(i as isize) as libc::c_int != prefixU[i as usize] as libc::c_int
            && *name.offset(i as isize) as libc::c_int != prefixL[i as usize] as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1
    }
    cp = 0 as libc::c_int;
    while *name.offset(i as isize) != 0 {
        static mut digits: [crate::expat_external_h::XML_Char; 11] =
            [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 0];
        let mut s: *const crate::expat_external_h::XML_Char =
            ::libc::strchr(digits.as_ptr(), *name.offset(i as isize) as libc::c_int);
        if s.is_null() {
            return 0 as libc::c_int;
        }
        cp *= 10 as libc::c_int;
        cp += s.wrapping_offset_from(digits.as_ptr()) as libc::c_long as libc::c_int;
        if cp >= 0x10000 as libc::c_int {
            return 0 as libc::c_int;
        }
        i += 1
    }
    if ::c2rust_out::src::xmlwf::codepage::codepageMap(cp, (*info).map.as_mut_ptr()) == 0 {
        return 0 as libc::c_int;
    }
    (*info).convert = Some(
        unknownEncodingConvert
            as unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char) -> libc::c_int,
    );
    /* We could just cast the code page integer to a void *,
    and avoid the use of release. */
    (*info).release = Some(::libc::free as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    (*info).data =
        ::c2rust_out::stdlib::malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if (*info).data.is_null() {
        return 0 as libc::c_int;
    }
    *((*info).data as *mut libc::c_int) = cp;
    return 1 as libc::c_int;
}

unsafe extern "C" fn notStandalone(mut _userData: *mut libc::c_void) -> libc::c_int {
    return 0 as libc::c_int;
}

unsafe extern "C" fn showVersion(mut prog: *mut crate::expat_external_h::XML_Char) {
    let mut s: *mut crate::expat_external_h::XML_Char = prog;
    let mut ch: crate::expat_external_h::XML_Char = 0;
    let mut features: *const ::c2rust_out::expat_h::XML_Feature =
        ::c2rust_out::src::lib::xmlparse::XML_GetFeatureList()
            as *const ::c2rust_out::expat_h::XML_Feature;
    loop {
        ch = *s;
        if !(ch as libc::c_int != 0 as libc::c_int) {
            break;
        }
        if ch as libc::c_int == '/' as i32 {
            prog = s.offset(1 as libc::c_int as isize)
        }
        s = s.offset(1)
    }
    ::c2rust_out::stdlib::fprintf(
        crate::stdlib::stdout as *mut ::c2rust_out::stdlib::_IO_FILE,
        b"%s using %s\n\x00" as *const u8 as *const libc::c_char,
        prog,
        ::c2rust_out::src::lib::xmlparse::XML_ExpatVersion(),
    );
    if !features.is_null()
        && (*features.offset(0 as libc::c_int as isize)).feature as libc::c_uint
            != ::c2rust_out::expat_h::XML_FEATURE_END as libc::c_int as libc::c_uint
    {
        let mut i: libc::c_int = 1 as libc::c_int;
        ::c2rust_out::stdlib::fprintf(
            crate::stdlib::stdout as *mut ::c2rust_out::stdlib::_IO_FILE,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*features.offset(0 as libc::c_int as isize)).name,
        );
        if (*features.offset(0 as libc::c_int as isize)).value != 0 {
            ::c2rust_out::stdlib::fprintf(
                crate::stdlib::stdout as *mut ::c2rust_out::stdlib::_IO_FILE,
                b"=%ld\x00" as *const u8 as *const libc::c_char,
                (*features.offset(0 as libc::c_int as isize)).value,
            );
        }
        while (*features.offset(i as isize)).feature as libc::c_uint
            != ::c2rust_out::expat_h::XML_FEATURE_END as libc::c_int as libc::c_uint
        {
            ::c2rust_out::stdlib::fprintf(
                crate::stdlib::stdout as *mut ::c2rust_out::stdlib::_IO_FILE,
                b", %s\x00" as *const u8 as *const libc::c_char,
                (*features.offset(i as isize)).name,
            );
            if (*features.offset(i as isize)).value != 0 {
                ::c2rust_out::stdlib::fprintf(
                    crate::stdlib::stdout as *mut ::c2rust_out::stdlib::_IO_FILE,
                    b"=%ld\x00" as *const u8 as *const libc::c_char,
                    (*features.offset(i as isize)).value,
                );
            }
            i += 1
        }
        ::c2rust_out::stdlib::fprintf(
            crate::stdlib::stdout as *mut ::c2rust_out::stdlib::_IO_FILE,
            b"\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}

unsafe extern "C" fn usage(
    mut prog: *const crate::expat_external_h::XML_Char,
    mut rc: libc::c_int,
) {
    ::c2rust_out::stdlib::fprintf(crate::stdlib::stderr as *mut ::c2rust_out::stdlib::_IO_FILE,
            b"usage: %s [-s] [-n] [-p] [-x] [-e ENCODING] [-w] [-r] [-d DIRECTORY]\n             [-c | -m | -t] [-N]\n             [FILE [FILE ...]]\n\nxmlwf - Determines if an XML document is well-formed\n\npositional arguments:\n  FILE          files to process (default: STDIN)\n\ninput control arguments:\n  -s            print an error if the document is not [s]tandalone\n  -n            enable [n]amespace processing\n  -p            enable processing external DTDs and [p]arameter entities\n  -x            enable processing of e[x]ternal entities\n  -e ENCODING   override any in-document [e]ncoding declaration\n  -w            enable support for [W]indows code pages\n  -r            disable memory-mapping and use normal file [r]ead IO calls instead\n\noutput control arguments:\n  -d DIRECTORY  output [d]estination directory\n  -c            write a [c]opy of input XML, not canonical XML\n  -m            write [m]eta XML, not canonical XML\n  -t            write no XML output for [t]iming of plain parsing\n  -N            enable adding doctype and [n]otation declarations\n\ninfo arguments:\n  -h            show this [h]elp message and exit\n  -v            show program\'s [v]ersion number and exit\n\nlibexpat is software libre, licensed under the MIT license.\nPlease report bugs at https://github.com/libexpat/libexpat/issues.  Thank you!\n\x00"
                as *const u8 as *const libc::c_char, prog);
    ::libc::exit(rc);
}

unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut crate::expat_external_h::XML_Char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut outputDir: *const crate::expat_external_h::XML_Char =
        ::c2rust_out::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    let mut encoding: *const crate::expat_external_h::XML_Char =
        ::c2rust_out::stddef_h::NULL as *const crate::expat_external_h::XML_Char;
    let mut processFlags: libc::c_uint =
        ::c2rust_out::src::xmlwf::xmlfile::XML_MAP_FILE as libc::c_uint;
    let mut windowsCodePages: libc::c_int = 0 as libc::c_int;
    let mut outputType: libc::c_int = 0 as libc::c_int;
    let mut useNamespaces: libc::c_int = 0 as libc::c_int;
    let mut requireStandalone: libc::c_int = 0 as libc::c_int;
    let mut requiresNotations: libc::c_int = 0 as libc::c_int;
    let mut paramEntityParsing: crate::expat_h::XML_ParamEntityParsing =
        ::c2rust_out::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
    let mut useStdin: libc::c_int = 0 as libc::c_int;
    let mut userData: XmlwfUserData = {
        let mut init = xmlwfUserData {
            fp: ::c2rust_out::stddef_h::NULL as *mut crate::stdlib::FILE,
            notationListHead: ::c2rust_out::stddef_h::NULL as *mut NotationList,
            currentDoctypeName: ::c2rust_out::stddef_h::NULL
                as *const crate::expat_external_h::XML_Char,
        };
        init
    };
    i = 1 as libc::c_int;
    j = 0 as libc::c_int;
    while i < argc {
        if j == 0 as libc::c_int {
            if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                != '-' as i32
            {
                break;
            }
            if *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                == '-' as i32
                && *(*argv.offset(i as isize)).offset(2 as libc::c_int as isize) as libc::c_int
                    == '\u{0}' as i32
            {
                i += 1;
                break;
            } else {
                j += 1
            }
        }
        let mut current_block_46: u64;
        match *(*argv.offset(i as isize)).offset(j as isize) as libc::c_int {
            114 => {
                processFlags &= !::c2rust_out::src::xmlwf::xmlfile::XML_MAP_FILE as libc::c_uint;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            115 => {
                requireStandalone = 1 as libc::c_int;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            110 => {
                useNamespaces = 1 as libc::c_int;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            112 => {
                paramEntityParsing = ::c2rust_out::expat_h::XML_PARAM_ENTITY_PARSING_ALWAYS;
                current_block_46 = 4092296097885336037;
            }
            120 => {
                current_block_46 = 4092296097885336037;
            }
            119 => {
                windowsCodePages = 1 as libc::c_int;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            109 => {
                outputType = 'm' as i32;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            99 => {
                outputType = 'c' as i32;
                useNamespaces = 0 as libc::c_int;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            116 => {
                outputType = 't' as i32;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            78 => {
                requiresNotations = 1 as libc::c_int;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            100 => {
                if *(*argv.offset(i as isize)).offset((j + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == '\u{0}' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(*argv.offset(0 as libc::c_int as isize), 2 as libc::c_int);
                    }
                    outputDir = *argv.offset(i as isize)
                } else {
                    outputDir = (*argv.offset(i as isize))
                        .offset(j as isize)
                        .offset(1 as libc::c_int as isize)
                }
                i += 1;
                j = 0 as libc::c_int;
                current_block_46 = 13707613154239713890;
            }
            101 => {
                if *(*argv.offset(i as isize)).offset((j + 1 as libc::c_int) as isize)
                    as libc::c_int
                    == '\u{0}' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(*argv.offset(0 as libc::c_int as isize), 2 as libc::c_int);
                    }
                    encoding = *argv.offset(i as isize)
                } else {
                    encoding = (*argv.offset(i as isize))
                        .offset(j as isize)
                        .offset(1 as libc::c_int as isize)
                }
                i += 1;
                j = 0 as libc::c_int;
                current_block_46 = 13707613154239713890;
            }
            104 => {
                usage(*argv.offset(0 as libc::c_int as isize), 0 as libc::c_int);
                return 0 as libc::c_int;
            }
            118 => {
                showVersion(*argv.offset(0 as libc::c_int as isize));
                return 0 as libc::c_int;
            }
            0 => {
                if j > 1 as libc::c_int {
                    i += 1;
                    j = 0 as libc::c_int;
                    current_block_46 = 13707613154239713890;
                } else {
                    current_block_46 = 16871217396860862036;
                }
            }
            _ => {
                current_block_46 = 16871217396860862036;
            }
        }
        match current_block_46 {
            4092296097885336037 =>
            /* fall through */
            {
                processFlags |=
                    ::c2rust_out::src::xmlwf::xmlfile::XML_EXTERNAL_ENTITIES as libc::c_uint;
                j += 1
            }
            16871217396860862036 =>
            /* fall through */
            {
                usage(*argv.offset(0 as libc::c_int as isize), 2 as libc::c_int);
            }
            _ => {}
        }
    }
    if i == argc {
        useStdin = 1 as libc::c_int;
        processFlags &= !::c2rust_out::src::xmlwf::xmlfile::XML_MAP_FILE as libc::c_uint;
        i -= 1
    }
    while i < argc {
        let mut outName: *mut crate::expat_external_h::XML_Char =
            0 as *mut crate::expat_external_h::XML_Char;
        let mut result: libc::c_int = 0;
        let mut parser: crate::expat_h::XML_Parser =
            0 as *mut ::c2rust_out::expat_h::XML_ParserStruct;
        if useNamespaces != 0 {
            parser = ::c2rust_out::src::lib::xmlparse::XML_ParserCreateNS(
                encoding,
                '\u{1}' as i32 as crate::expat_external_h::XML_Char,
            )
        } else {
            parser = ::c2rust_out::src::lib::xmlparse::XML_ParserCreate(encoding)
        }
        if parser.is_null() {
            ::c2rust_out::src::xmlwf::xmlfile::perror(
                b"Could not instantiate parser\x00" as *const u8 as *const libc::c_char,
            );
            ::libc::exit(1 as libc::c_int);
        }
        if requireStandalone != 0 {
            ::c2rust_out::src::lib::xmlparse::XML_SetNotStandaloneHandler(
                parser,
                Some(notStandalone as unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int),
            );
        }
        ::c2rust_out::src::lib::xmlparse::XML_SetParamEntityParsing(parser, paramEntityParsing);
        if outputType == 't' as i32 {
            /* This is for doing timings; this gives a more realistic estimate of
            the parsing time. */
            outputDir = 0 as *const crate::expat_external_h::XML_Char;
            ::c2rust_out::src::lib::xmlparse::XML_SetElementHandler(
                parser,
                Some(
                    nopStartElement
                        as unsafe extern "C" fn(
                            _: *mut libc::c_void,
                            _: *const crate::expat_external_h::XML_Char,
                            _: *mut *const crate::expat_external_h::XML_Char,
                        ) -> (),
                ),
                Some(
                    nopEndElement
                        as unsafe extern "C" fn(
                            _: *mut libc::c_void,
                            _: *const crate::expat_external_h::XML_Char,
                        ) -> (),
                ),
            );
            ::c2rust_out::src::lib::xmlparse::XML_SetCharacterDataHandler(
                parser,
                Some(
                    nopCharacterData
                        as unsafe extern "C" fn(
                            _: *mut libc::c_void,
                            _: *const crate::expat_external_h::XML_Char,
                            _: libc::c_int,
                        ) -> (),
                ),
            );
            ::c2rust_out::src::lib::xmlparse::XML_SetProcessingInstructionHandler(
                parser,
                Some(
                    nopProcessingInstruction
                        as unsafe extern "C" fn(
                            _: *mut libc::c_void,
                            _: *const crate::expat_external_h::XML_Char,
                            _: *const crate::expat_external_h::XML_Char,
                        ) -> (),
                ),
            );
        } else if !outputDir.is_null() {
            let mut delim: *const crate::expat_external_h::XML_Char =
                b"/\x00" as *const u8 as *const libc::c_char;
            let mut file: *const crate::expat_external_h::XML_Char = if useStdin != 0 {
                b"STDIN\x00" as *const u8 as *const libc::c_char
            } else {
                *argv.offset(i as isize) as *const libc::c_char
            };
            if useStdin == 0 {
                /* Jump after last (back)slash */
                let mut lastDelim: *const crate::expat_external_h::XML_Char = ::libc::strrchr(
                    file,
                    *delim.offset(0 as libc::c_int as isize) as libc::c_int,
                );
                if !lastDelim.is_null() {
                    file = lastDelim.offset(1 as libc::c_int as isize)
                }
            }
            outName = ::c2rust_out::stdlib::malloc(
                ::c2rust_out::stdlib::strlen(outputDir)
                    .wrapping_add(::c2rust_out::stdlib::strlen(file))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::expat_external_h::XML_Char>() as libc::c_ulong
                    ),
            ) as *mut crate::expat_external_h::XML_Char;
            ::libc::strcpy(outName, outputDir);
            ::libc::strcat(outName, delim);
            ::libc::strcat(outName, file);
            userData.fp =
                crate::stdlib::fopen(outName, b"wb\x00" as *const u8 as *const libc::c_char);
            if userData.fp.is_null() {
                ::c2rust_out::src::xmlwf::xmlfile::perror(outName);
                ::libc::exit(1 as libc::c_int);
            }
            crate::stdlib::setvbuf(
                userData.fp,
                ::c2rust_out::stddef_h::NULL as *mut libc::c_char,
                ::libc::_IOFBF,
                16384 as libc::c_int as crate::stddef_h::size_t,
            );
            ::c2rust_out::src::lib::xmlparse::XML_SetUserData(
                parser,
                &mut userData as *mut XmlwfUserData as *mut libc::c_void,
            );
            match outputType {
                109 => {
                    ::c2rust_out::src::lib::xmlparse::XML_UseParserAsHandlerArg(parser);
                    ::c2rust_out::src::lib::xmlparse::XML_SetElementHandler(
                        parser,
                        Some(
                            metaStartElement
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *mut *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                        Some(
                            metaEndElement
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetProcessingInstructionHandler(
                        parser,
                        Some(
                            metaProcessingInstruction
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetCommentHandler(
                        parser,
                        Some(
                            metaComment
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetCdataSectionHandler(
                        parser,
                        Some(
                            metaStartCdataSection
                                as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
                        ),
                        Some(
                            metaEndCdataSection as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetCharacterDataHandler(
                        parser,
                        Some(
                            metaCharacterData
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: libc::c_int,
                                ) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetDoctypeDeclHandler(
                        parser,
                        Some(
                            metaStartDoctypeDecl
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: libc::c_int,
                                ) -> (),
                        ),
                        Some(
                            metaEndDoctypeDecl as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetEntityDeclHandler(
                        parser,
                        Some(
                            metaEntityDecl
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: libc::c_int,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: libc::c_int,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetNotationDeclHandler(
                        parser,
                        Some(
                            metaNotationDecl
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetNamespaceDeclHandler(
                        parser,
                        Some(
                            metaStartNamespaceDecl
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                        Some(
                            metaEndNamespaceDecl
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                    );
                    metaStartDocument(parser as *mut libc::c_void);
                }
                99 => {
                    ::c2rust_out::src::lib::xmlparse::XML_UseParserAsHandlerArg(parser);
                    ::c2rust_out::src::lib::xmlparse::XML_SetDefaultHandler(
                        parser,
                        Some(
                            markup
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: libc::c_int,
                                ) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetElementHandler(
                        parser,
                        Some(
                            defaultStartElement
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *mut *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                        Some(
                            defaultEndElement
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetCharacterDataHandler(
                        parser,
                        Some(
                            defaultCharacterData
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: libc::c_int,
                                ) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetProcessingInstructionHandler(
                        parser,
                        Some(
                            defaultProcessingInstruction
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                    );
                }
                _ => {
                    if useNamespaces != 0 {
                        ::c2rust_out::src::lib::xmlparse::XML_SetElementHandler(
                            parser,
                            Some(
                                startElementNS
                                    as unsafe extern "C" fn(
                                        _: *mut libc::c_void,
                                        _: *const crate::expat_external_h::XML_Char,
                                        _: *mut *const crate::expat_external_h::XML_Char,
                                    )
                                        -> (),
                            ),
                            Some(
                                endElementNS
                                    as unsafe extern "C" fn(
                                        _: *mut libc::c_void,
                                        _: *const crate::expat_external_h::XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                    } else {
                        ::c2rust_out::src::lib::xmlparse::XML_SetElementHandler(
                            parser,
                            Some(
                                startElement
                                    as unsafe extern "C" fn(
                                        _: *mut libc::c_void,
                                        _: *const crate::expat_external_h::XML_Char,
                                        _: *mut *const crate::expat_external_h::XML_Char,
                                    )
                                        -> (),
                            ),
                            Some(
                                endElement
                                    as unsafe extern "C" fn(
                                        _: *mut libc::c_void,
                                        _: *const crate::expat_external_h::XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                    }
                    ::c2rust_out::src::lib::xmlparse::XML_SetCharacterDataHandler(
                        parser,
                        Some(
                            characterData
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: libc::c_int,
                                ) -> (),
                        ),
                    );
                    ::c2rust_out::src::lib::xmlparse::XML_SetProcessingInstructionHandler(
                        parser,
                        Some(
                            processingInstruction
                                as unsafe extern "C" fn(
                                    _: *mut libc::c_void,
                                    _: *const crate::expat_external_h::XML_Char,
                                    _: *const crate::expat_external_h::XML_Char,
                                ) -> (),
                        ),
                    );
                    if requiresNotations != 0 {
                        ::c2rust_out::src::lib::xmlparse::XML_SetDoctypeDeclHandler(
                            parser,
                            Some(
                                startDoctypeDecl
                                    as unsafe extern "C" fn(
                                        _: *mut libc::c_void,
                                        _: *const crate::expat_external_h::XML_Char,
                                        _: *const crate::expat_external_h::XML_Char,
                                        _: *const crate::expat_external_h::XML_Char,
                                        _: libc::c_int,
                                    )
                                        -> (),
                            ),
                            Some(
                                endDoctypeDecl as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
                            ),
                        );
                        ::c2rust_out::src::lib::xmlparse::XML_SetNotationDeclHandler(
                            parser,
                            Some(
                                notationDecl
                                    as unsafe extern "C" fn(
                                        _: *mut libc::c_void,
                                        _: *const crate::expat_external_h::XML_Char,
                                        _: *const crate::expat_external_h::XML_Char,
                                        _: *const crate::expat_external_h::XML_Char,
                                        _: *const crate::expat_external_h::XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                    }
                }
            }
        }
        if windowsCodePages != 0 {
            ::c2rust_out::src::lib::xmlparse::XML_SetUnknownEncodingHandler(
                parser,
                ::std::mem::transmute(Some(
                    unknownEncoding
                        as unsafe extern "C" fn(
                            _: *mut libc::c_void,
                            _: *const crate::expat_external_h::XML_Char,
                            _: *mut ::c2rust_out::expat_h::XML_Encoding,
                        ) -> libc::c_int,
                )),
                0 as *mut libc::c_void,
            );
        }
        result = ::c2rust_out::src::xmlwf::xmlfile::XML_ProcessFile(
            parser,
            if useStdin != 0 {
                ::c2rust_out::stddef_h::NULL as *mut crate::expat_external_h::XML_Char
            } else {
                *argv.offset(i as isize)
            },
            processFlags,
        );
        if !outputDir.is_null() {
            if outputType == 'm' as i32 {
                metaEndDocument(parser as *mut libc::c_void);
            }
            crate::stdlib::fclose(userData.fp);
            if result == 0 {
                ::libc::remove(outName);
            }
            ::libc::free(outName as *mut libc::c_void);
        }
        ::c2rust_out::src::lib::xmlparse::XML_ParserFree(parser);
        if result == 0 {
            ::libc::exit(2 as libc::c_int);
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[main]
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
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut crate::expat_external_h::XML_Char,
        ) as i32)
    }
}
