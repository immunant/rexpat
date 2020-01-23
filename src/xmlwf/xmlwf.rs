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

use ::expat_rs::expat_h::{
    XML_Encoding, XML_Feature, XML_FEATURE_END, XML_PARAM_ENTITY_PARSING_ALWAYS,
    XML_PARAM_ENTITY_PARSING_NEVER,
};
use ::expat_rs::lib::xmlparse::{
    XML_DefaultCurrent, XML_ExpatVersion, XML_GetBase, XML_GetCurrentByteCount,
    XML_GetCurrentByteIndex, XML_GetFeatureList, XML_GetIdAttributeIndex,
    XML_GetSpecifiedAttributeCount, XML_ParserCreate, XML_ParserCreateNS,
    XML_SetCdataSectionHandler, XML_SetCharacterDataHandler, XML_SetCommentHandler,
    XML_SetDefaultHandler, XML_SetDoctypeDeclHandler, XML_SetElementHandler,
    XML_SetEntityDeclHandler, XML_SetNamespaceDeclHandler, XML_SetNotStandaloneHandler,
    XML_SetNotationDeclHandler, XML_SetParamEntityParsing, XML_SetProcessingInstructionHandler,
    XML_SetUnknownEncodingHandler, XML_SetUserData, XML_UseParserAsHandlerArg,
};
use ::expat_rs::stdlib::{__assert_fail, malloc, memcpy, strlen};
use ::libc::{exit, free, remove, strcat, strchr, strcmp, strcpy, strrchr, _IOFBF};

use ::std::mem::transmute;

pub use expat_rs::*;
use libc::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub mod codepage;
pub mod readfilemap;
pub mod xmlfile;

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
    pub notationName: *const XML_Char,
    pub systemId: *const XML_Char,
    pub publicId: *const XML_Char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xmlwfUserData {
    pub fp: *mut FILE,
    pub notationListHead: *mut NotationList,
    pub currentDoctypeName: *const XML_Char,
}

pub type XmlwfUserData = xmlwfUserData;
/* This ensures proper sorting. */

unsafe extern "C" fn characterData(
    mut userData: *mut c_void,
    mut s: *const XML_Char,
    mut len: c_int,
) {
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    while len > 0 {
        match *s as c_int {
            38 => {
                fputs(b"&amp;\x00".as_ptr() as *const c_char, fp);
            }
            60 => {
                fputs(b"&lt;\x00".as_ptr() as *const c_char, fp);
            }
            62 => {
                fputs(b"&gt;\x00".as_ptr() as *const c_char, fp);
            }
            34 => {
                fputs(b"&quot;\x00".as_ptr() as *const c_char, fp);
            }
            9 | 10 | 13 => {
                ::expat_rs::stdlib::fprintf(
                    fp,
                    b"&#%d;\x00".as_ptr() as *const c_char,
                    *s as c_int,
                );
            }
            _ => {
                putc(*s as c_int, fp);
            }
        }
        len -= 1;
        s = s.offset(1)
    }
}

unsafe extern "C" fn attributeValue(mut fp: *mut FILE, mut s: *const XML_Char) {
    putc('=' as i32, fp);
    putc('\"' as i32, fp);
    if !s.is_null() {
    } else {
        __assert_fail(
            b"s\x00".as_ptr() as *const c_char,
            b"/home/sjcrane/projects/c2rust/libexpat/upstream/expat/xmlwf/xmlwf.c\x00".as_ptr()
                as *const c_char,
            110u32,
            __ASSERT_FUNCTION.as_ptr(),
        );
    }
    loop {
        match *s as c_int {
            0 | 1 => {
                putc('\"' as i32, fp);
                return;
            }
            38 => {
                fputs(b"&amp;\x00".as_ptr() as *const c_char, fp);
            }
            60 => {
                fputs(b"&lt;\x00".as_ptr() as *const c_char, fp);
            }
            34 => {
                fputs(b"&quot;\x00".as_ptr() as *const c_char, fp);
            }
            62 => {
                fputs(b"&gt;\x00".as_ptr() as *const c_char, fp);
            }
            9 | 10 | 13 => {
                ::expat_rs::stdlib::fprintf(
                    fp,
                    b"&#%d;\x00".as_ptr() as *const c_char,
                    *s as c_int,
                );
            }
            _ => {
                putc(*s as c_int, fp);
            }
        }
        s = s.offset(1)
    }
}
/* Lexicographically comparing UTF-8 encoded attribute values,
is equivalent to lexicographically comparing based on the character number. */

unsafe extern "C" fn attcmp(mut att1: *const c_void, mut att2: *const c_void) -> c_int {
    return strcmp(
        *(att1 as *mut *const XML_Char),
        *(att2 as *mut *const XML_Char),
    );
}

unsafe extern "C" fn startElement(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut nAtts: c_int = 0;
    let mut p: *mut *const XML_Char = 0 as *mut *const XML_Char;
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    putc('<' as i32, fp);
    fputs(name, fp);
    p = atts;
    while !(*p).is_null() {
        p = p.offset(1)
    }
    nAtts = (p.wrapping_offset_from(atts) as c_long >> 1) as c_int;
    if nAtts > 1 {
        qsort(
            atts as *mut c_void,
            nAtts as size_t,
            (::std::mem::size_of::<*mut XML_Char>() as c_ulong).wrapping_mul(2u64),
            Some(attcmp as unsafe extern "C" fn(_: *const c_void, _: *const c_void) -> c_int),
        );
    }
    while !(*atts).is_null() {
        putc(' ' as i32, fp);
        let fresh0 = atts;
        atts = atts.offset(1);
        fputs(*fresh0, fp);
        attributeValue(fp, *atts);
        atts = atts.offset(1)
    }
    putc('>' as i32, fp);
}

unsafe extern "C" fn endElement(mut userData: *mut c_void, mut name: *const XML_Char) {
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    putc('<' as i32, fp);
    putc('/' as i32, fp);
    fputs(name, fp);
    putc('>' as i32, fp);
}

unsafe extern "C" fn nsattcmp(mut p1: *const c_void, mut p2: *const c_void) -> c_int {
    let mut att1: *const XML_Char = *(p1 as *mut *const XML_Char);
    let mut att2: *const XML_Char = *(p2 as *mut *const XML_Char);
    let mut sep1: c_int = (strrchr(att1, '\u{1}' as i32) != 0 as *mut c_char) as c_int;
    let mut sep2: c_int = (strrchr(att1, '\u{1}' as i32) != 0 as *mut c_char) as c_int;
    if sep1 != sep2 {
        return sep1 - sep2;
    }
    return strcmp(att1, att2);
}

unsafe extern "C" fn startElementNS(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut nAtts: c_int = 0;
    let mut nsi: c_int = 0;
    let mut p: *mut *const XML_Char = 0 as *mut *const XML_Char;
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    let mut sep: *const XML_Char = 0 as *const XML_Char;
    putc('<' as i32, fp);
    sep = strrchr(name, '\u{1}' as i32);
    if !sep.is_null() {
        fputs(b"n1:\x00".as_ptr() as *const c_char, fp);
        fputs(sep.offset(1), fp);
        fputs(b" xmlns:n1\x00".as_ptr() as *const c_char, fp);
        attributeValue(fp, name);
        nsi = 2
    } else {
        fputs(name, fp);
        nsi = 1
    }
    p = atts;
    while !(*p).is_null() {
        p = p.offset(1)
    }
    nAtts = (p.wrapping_offset_from(atts) as c_long >> 1) as c_int;
    if nAtts > 1 {
        qsort(
            atts as *mut c_void,
            nAtts as size_t,
            (::std::mem::size_of::<*mut XML_Char>() as c_ulong).wrapping_mul(2u64),
            Some(nsattcmp as unsafe extern "C" fn(_: *const c_void, _: *const c_void) -> c_int),
        );
    }
    while !(*atts).is_null() {
        let fresh1 = atts;
        atts = atts.offset(1);
        name = *fresh1;
        sep = strrchr(name, '\u{1}' as i32);
        putc(' ' as i32, fp);
        if !sep.is_null() {
            ::expat_rs::stdlib::fprintf(fp, b"n%d:\x00".as_ptr() as *const c_char, nsi);
            fputs(sep.offset(1isize), fp);
        } else {
            fputs(name, fp);
        }
        attributeValue(fp, *atts);
        if !sep.is_null() {
            let fresh2 = nsi;
            nsi = nsi + 1;
            ::expat_rs::stdlib::fprintf(fp, b" xmlns:n%d\x00".as_ptr() as *const c_char, fresh2);
            attributeValue(fp, name);
        }
        atts = atts.offset(1)
    }
    putc('>' as i32, fp);
}

unsafe extern "C" fn endElementNS(mut userData: *mut c_void, mut name: *const XML_Char) {
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    let mut sep: *const XML_Char = 0 as *const XML_Char;
    putc('<' as i32, fp);
    putc('/' as i32, fp);
    sep = strrchr(name, '\u{1}' as i32);
    if !sep.is_null() {
        fputs(b"n1:\x00".as_ptr() as *const c_char, fp);
        fputs(sep.offset(1isize), fp);
    } else {
        fputs(name, fp);
    }
    putc('>' as i32, fp);
}

unsafe extern "C" fn processingInstruction(
    mut userData: *mut c_void,
    mut target: *const XML_Char,
    mut data: *const XML_Char,
) {
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    putc('<' as i32, fp);
    putc('?' as i32, fp);
    fputs(target, fp);
    putc(' ' as i32, fp);
    fputs(data, fp);
    putc('?' as i32, fp);
    putc('>' as i32, fp);
}

unsafe extern "C" fn xcsdup(mut s: *const XML_Char) -> *mut XML_Char {
    let mut result: *mut XML_Char = 0 as *mut XML_Char;
    let mut count: c_int = 0;
    let mut numBytes: c_int = 0;
    loop
    /* Get the length of the string, including terminator */
    {
        let fresh3 = count;
        count = count + 1;
        if !(*s.offset(fresh3 as isize) as c_int != 0) {
            break;
        }
    }
    numBytes =
        (count as c_ulong).wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong) as c_int;
    result = malloc(numBytes as c_ulong) as *mut XML_Char;
    if result.is_null() {
        return ::expat_rs::stddef_h::NULL as *mut XML_Char;
    }
    memcpy(
        result as *mut c_void,
        s as *const c_void,
        numBytes as c_ulong,
    );
    return result;
}

unsafe extern "C" fn startDoctypeDecl(
    mut userData: *mut c_void,
    mut doctypeName: *const XML_Char,
    mut _sysid: *const XML_Char,
    mut _publid: *const XML_Char,
    mut _has_internal_subset: c_int,
) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    (*data).currentDoctypeName = xcsdup(doctypeName);
}

unsafe extern "C" fn freeNotations(mut data: *mut XmlwfUserData) {
    let mut notationListHead: *mut NotationList = (*data).notationListHead;
    while !notationListHead.is_null() {
        let mut next: *mut NotationList = (*notationListHead).next;
        free((*notationListHead).notationName as *mut c_void);
        free((*notationListHead).systemId as *mut c_void);
        free((*notationListHead).publicId as *mut c_void);
        free(notationListHead as *mut c_void);
        notationListHead = next
    }
    (*data).notationListHead = ::expat_rs::stddef_h::NULL as *mut NotationList;
}

unsafe extern "C" fn xcscmp(mut xs: *const XML_Char, mut xt: *const XML_Char) -> c_int {
    while *xs as c_int != 0 && *xt as c_int != 0 {
        if (*xs as c_int) < *xt as c_int {
            return -(1i32);
        }
        if *xs as c_int > *xt as c_int {
            return 1i32;
        }
        xs = xs.offset(1);
        xt = xt.offset(1)
    }
    if (*xs as c_int) < *xt as c_int {
        return -(1i32);
    }
    if *xs as c_int > *xt as c_int {
        return 1i32;
    }
    return 0;
}

unsafe extern "C" fn notationCmp(mut a: *const c_void, mut b: *const c_void) -> c_int {
    let n1: *const NotationList = *(a as *mut *mut NotationList);
    let n2: *const NotationList = *(b as *mut *mut NotationList);
    return xcscmp((*n1).notationName, (*n2).notationName);
}

unsafe extern "C" fn endDoctypeDecl(mut userData: *mut c_void) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    let mut notations: *mut *mut NotationList = 0 as *mut *mut NotationList;
    let mut notationCount: c_int = 0;
    let mut p: *mut NotationList = 0 as *mut NotationList;
    let mut i: c_int = 0;
    /* How many notations do we have? */
    p = (*data).notationListHead;
    while !p.is_null() {
        notationCount += 1;
        p = (*p).next
    }
    if notationCount == 0 {
        /* Nothing to report */
        free((*data).currentDoctypeName as *mut c_void);
        (*data).currentDoctypeName = ::expat_rs::stddef_h::NULL as *const XML_Char;
        return;
    }
    notations = malloc(
        (notationCount as c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut NotationList>() as c_ulong),
    ) as *mut *mut NotationList;
    if notations.is_null() {
        ::expat_rs::stdlib::fprintf(
            stderr,
            b"Unable to sort notations\x00".as_ptr() as *const c_char,
        );
        freeNotations(data);
        return;
    }
    p = (*data).notationListHead;
    i = 0;
    while i < notationCount {
        let ref mut fresh4 = *notations.offset(i as isize);
        *fresh4 = p;
        p = (*p).next;
        i += 1
    }
    qsort(
        notations as *mut c_void,
        notationCount as size_t,
        ::std::mem::size_of::<*mut NotationList>() as c_ulong,
        Some(notationCmp as unsafe extern "C" fn(_: *const c_void, _: *const c_void) -> c_int),
    );
    /* Output the DOCTYPE header */
    fputs(b"<!DOCTYPE \x00".as_ptr() as *const c_char, (*data).fp);
    fputs((*data).currentDoctypeName, (*data).fp);
    fputs(b" [\n\x00".as_ptr() as *const c_char, (*data).fp);
    /* Now the NOTATIONs */
    i = 0;
    while i < notationCount {
        fputs(b"<!NOTATION \x00".as_ptr() as *const c_char, (*data).fp);
        fputs((**notations.offset(i as isize)).notationName, (*data).fp);
        if !(**notations.offset(i as isize)).publicId.is_null() {
            fputs(b" PUBLIC \'\x00".as_ptr() as *const c_char, (*data).fp);
            fputs((**notations.offset(i as isize)).publicId, (*data).fp);
            putc('\'' as i32, (*data).fp);
            if !(**notations.offset(i as isize)).systemId.is_null() {
                putc(' ' as i32, (*data).fp);
                putc('\'' as i32, (*data).fp);
                fputs((**notations.offset(i as isize)).systemId, (*data).fp);
                putc('\'' as i32, (*data).fp);
            }
        } else if !(**notations.offset(i as isize)).systemId.is_null() {
            fputs(b" SYSTEM \'\x00".as_ptr() as *const c_char, (*data).fp);
            fputs((**notations.offset(i as isize)).systemId, (*data).fp);
            putc('\'' as i32, (*data).fp);
        }
        putc('>' as i32, (*data).fp);
        putc('\n' as i32, (*data).fp);
        i += 1
    }
    /* Finally end the DOCTYPE */
    fputs(b"]>\n\x00".as_ptr() as *const c_char, (*data).fp);
    free(notations as *mut c_void);
    freeNotations(data);
    free((*data).currentDoctypeName as *mut c_void);
    (*data).currentDoctypeName = ::expat_rs::stddef_h::NULL as *const XML_Char;
}

unsafe extern "C" fn notationDecl(
    mut userData: *mut c_void,
    mut notationName: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    let mut entry: *mut NotationList =
        malloc(::std::mem::size_of::<NotationList>() as c_ulong) as *mut NotationList;
    let mut errorMessage: *const c_char =
        b"Unable to store NOTATION for output\n\x00".as_ptr() as *const c_char;
    if entry.is_null() {
        fputs(errorMessage, stderr);
        return;
        /* Nothing we can really do about this */
    } /* Safe if it's NULL */
    (*entry).notationName = xcsdup(notationName);
    if (*entry).notationName.is_null() {
        fputs(errorMessage, stderr);
        free(entry as *mut c_void);
        return;
    }
    if !systemId.is_null() {
        (*entry).systemId = xcsdup(systemId);
        if (*entry).systemId.is_null() {
            fputs(errorMessage, stderr);
            free((*entry).notationName as *mut c_void);
            free(entry as *mut c_void);
            return;
        }
    } else {
        (*entry).systemId = ::expat_rs::stddef_h::NULL as *const XML_Char
    }
    if !publicId.is_null() {
        (*entry).publicId = xcsdup(publicId);
        if (*entry).publicId.is_null() {
            fputs(errorMessage, stderr);
            free((*entry).systemId as *mut c_void);
            free((*entry).notationName as *mut c_void);
            free(entry as *mut c_void);
            return;
        }
    } else {
        (*entry).publicId = ::expat_rs::stddef_h::NULL as *const XML_Char
    }
    (*entry).next = (*data).notationListHead;
    (*data).notationListHead = entry;
}
/* not W3C14N */

unsafe extern "C" fn defaultCharacterData(
    mut userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
    XML_DefaultCurrent(userData as XML_Parser);
}

unsafe extern "C" fn defaultStartElement(
    mut userData: *mut c_void,
    mut _name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
    XML_DefaultCurrent(userData as XML_Parser);
}

unsafe extern "C" fn defaultEndElement(mut userData: *mut c_void, mut _name: *const XML_Char) {
    XML_DefaultCurrent(userData as XML_Parser);
}

unsafe extern "C" fn defaultProcessingInstruction(
    mut userData: *mut c_void,
    mut _target: *const XML_Char,
    mut _data: *const XML_Char,
) {
    XML_DefaultCurrent(userData as XML_Parser);
}

unsafe extern "C" fn nopCharacterData(
    mut _userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
}

unsafe extern "C" fn nopStartElement(
    mut _userData: *mut c_void,
    mut _name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
}

unsafe extern "C" fn nopEndElement(mut _userData: *mut c_void, mut _name: *const XML_Char) {}

unsafe extern "C" fn nopProcessingInstruction(
    mut _userData: *mut c_void,
    mut _target: *const XML_Char,
    mut _data: *const XML_Char,
) {
}

unsafe extern "C" fn markup(mut userData: *mut c_void, mut s: *const XML_Char, mut len: c_int) {
    let mut fp: *mut FILE = (*(*(userData as *mut *mut c_void) as *mut XmlwfUserData)).fp;
    while len > 0 {
        putc(*s as c_int, fp);
        len -= 1;
        s = s.offset(1)
    }
}

unsafe extern "C" fn metaLocation(mut parser: XML_Parser) {
    let mut uri: *const XML_Char = XML_GetBase(parser);
    let mut fp: *mut FILE = (*(*(parser as *mut *mut c_void) as *mut XmlwfUserData)).fp;
    if !uri.is_null() {
        ::expat_rs::stdlib::fprintf(fp, b" uri=\"%s\"\x00".as_ptr() as *const c_char, uri);
    }
    ::expat_rs::stdlib::fprintf(
        fp,
        b" byte=\"%ld\" nbytes=\"%d\" line=\"%lu\" col=\"%lu\"\x00".as_ptr() as *const c_char,
        XML_GetCurrentByteIndex(parser),
        XML_GetCurrentByteCount(parser),
        ::expat_rs::lib::xmlparse::XML_GetCurrentLineNumber(parser),
        ::expat_rs::lib::xmlparse::XML_GetCurrentColumnNumber(parser),
    );
}

unsafe extern "C" fn metaStartDocument(mut userData: *mut c_void) {
    fputs(
        b"<document>\n\x00".as_ptr() as *const c_char,
        (*(*(userData as *mut *mut c_void) as *mut XmlwfUserData)).fp,
    );
}

unsafe extern "C" fn metaEndDocument(mut userData: *mut c_void) {
    fputs(
        b"</document>\n\x00".as_ptr() as *const c_char,
        (*(*(userData as *mut *mut c_void) as *mut XmlwfUserData)).fp,
    );
}

unsafe extern "C" fn metaStartElement(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    let mut specifiedAttsEnd: *mut *const XML_Char =
        atts.offset(XML_GetSpecifiedAttributeCount(parser) as isize);
    let mut idAttPtr: *mut *const XML_Char = 0 as *mut *const XML_Char;
    let mut idAttIndex: c_int = XML_GetIdAttributeIndex(parser);
    if idAttIndex < 0 {
        idAttPtr = 0 as *mut *const XML_Char
    } else {
        idAttPtr = atts.offset(idAttIndex as isize)
    }
    ::expat_rs::stdlib::fprintf(
        fp,
        b"<starttag name=\"%s\"\x00".as_ptr() as *const c_char,
        name,
    );
    metaLocation(parser);
    if !(*atts).is_null() {
        fputs(b">\n\x00".as_ptr() as *const c_char, fp);
        loop {
            ::expat_rs::stdlib::fprintf(
                fp,
                b"<attribute name=\"%s\" value=\"\x00".as_ptr() as *const c_char,
                *atts.offset(0isize),
            );
            characterData(
                data as *mut c_void,
                *atts.offset(1),
                strlen(*atts.offset(1)) as c_int,
            );
            if atts >= specifiedAttsEnd {
                fputs(
                    b"\" defaulted=\"yes\"/>\n\x00".as_ptr() as *const c_char,
                    fp,
                );
            } else if atts == idAttPtr {
                fputs(b"\" id=\"yes\"/>\n\x00".as_ptr() as *const c_char, fp);
            } else {
                fputs(b"\"/>\n\x00".as_ptr() as *const c_char, fp);
            }
            atts = atts.offset(2);
            if (*atts).is_null() {
                break;
            }
        }
        fputs(b"</starttag>\n\x00".as_ptr() as *const c_char, fp);
    } else {
        fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
    };
}

unsafe extern "C" fn metaEndElement(mut userData: *mut c_void, mut name: *const XML_Char) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    ::expat_rs::stdlib::fprintf(
        fp,
        b"<endtag name=\"%s\"\x00".as_ptr() as *const c_char,
        name,
    );
    metaLocation(parser);
    fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
}

unsafe extern "C" fn metaProcessingInstruction(
    mut userData: *mut c_void,
    mut target: *const XML_Char,
    mut data: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut usrData: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*usrData).fp;
    ::expat_rs::stdlib::fprintf(
        fp,
        b"<pi target=\"%s\" data=\"\x00".as_ptr() as *const c_char,
        target,
    );
    characterData(usrData as *mut c_void, data, strlen(data) as c_int);
    putc('\"' as i32, fp);
    metaLocation(parser);
    fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
}

unsafe extern "C" fn metaComment(mut userData: *mut c_void, mut data: *const XML_Char) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut usrData: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*usrData).fp;
    fputs(b"<comment data=\"\x00".as_ptr() as *const c_char, fp);
    characterData(usrData as *mut c_void, data, strlen(data) as c_int);
    putc('\"' as i32, fp);
    metaLocation(parser);
    fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
}

unsafe extern "C" fn metaStartCdataSection(mut userData: *mut c_void) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fputs(b"<startcdata\x00".as_ptr() as *const c_char, fp);
    metaLocation(parser);
    fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
}

unsafe extern "C" fn metaEndCdataSection(mut userData: *mut c_void) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fputs(b"<endcdata\x00".as_ptr() as *const c_char, fp);
    metaLocation(parser);
    fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
}

unsafe extern "C" fn metaCharacterData(
    mut userData: *mut c_void,
    mut s: *const XML_Char,
    mut len: c_int,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fputs(b"<chars str=\"\x00".as_ptr() as *const c_char, fp);
    characterData(data as *mut c_void, s, len);
    putc('\"' as i32, fp);
    metaLocation(parser);
    fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
}

unsafe extern "C" fn metaStartDoctypeDecl(
    mut userData: *mut c_void,
    mut doctypeName: *const XML_Char,
    mut _sysid: *const XML_Char,
    mut _pubid: *const XML_Char,
    mut _has_internal_subset: c_int,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    ::expat_rs::stdlib::fprintf(
        fp,
        b"<startdoctype name=\"%s\"\x00".as_ptr() as *const c_char,
        doctypeName,
    );
    metaLocation(parser);
    fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
}

unsafe extern "C" fn metaEndDoctypeDecl(mut userData: *mut c_void) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fputs(b"<enddoctype\x00".as_ptr() as *const c_char, fp);
    metaLocation(parser);
    fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
}

unsafe extern "C" fn metaNotationDecl(
    mut userData: *mut c_void,
    mut notationName: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    ::expat_rs::stdlib::fprintf(
        fp,
        b"<notation name=\"%s\"\x00".as_ptr() as *const c_char,
        notationName,
    );
    if !publicId.is_null() {
        ::expat_rs::stdlib::fprintf(
            fp,
            b" public=\"%s\"\x00".as_ptr() as *const c_char,
            publicId,
        );
    }
    if !systemId.is_null() {
        fputs(b" system=\"\x00".as_ptr() as *const c_char, fp);
        characterData(data as *mut c_void, systemId, strlen(systemId) as c_int);
        putc('\"' as i32, fp);
    }
    metaLocation(parser);
    fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
}

unsafe extern "C" fn metaEntityDecl(
    mut userData: *mut c_void,
    mut entityName: *const XML_Char,
    mut _is_param: c_int,
    mut value: *const XML_Char,
    mut value_length: c_int,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
    mut notationName: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    if !value.is_null() {
        ::expat_rs::stdlib::fprintf(
            fp,
            b"<entity name=\"%s\"\x00".as_ptr() as *const c_char,
            entityName,
        );
        metaLocation(parser);
        putc('>' as i32, fp);
        characterData(data as *mut c_void, value, value_length);
        fputs(b"</entity/>\n\x00".as_ptr() as *const c_char, fp);
    } else if !notationName.is_null() {
        ::expat_rs::stdlib::fprintf(
            fp,
            b"<entity name=\"%s\"\x00".as_ptr() as *const c_char,
            entityName,
        );
        if !publicId.is_null() {
            ::expat_rs::stdlib::fprintf(
                fp,
                b" public=\"%s\"\x00".as_ptr() as *const c_char,
                publicId,
            );
        }
        fputs(b" system=\"\x00".as_ptr() as *const c_char, fp);
        characterData(data as *mut c_void, systemId, strlen(systemId) as c_int);
        putc('\"' as i32, fp);
        ::expat_rs::stdlib::fprintf(
            fp,
            b" notation=\"%s\"\x00".as_ptr() as *const c_char,
            notationName,
        );
        metaLocation(parser);
        fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
    } else {
        ::expat_rs::stdlib::fprintf(
            fp,
            b"<entity name=\"%s\"\x00".as_ptr() as *const c_char,
            entityName,
        );
        if !publicId.is_null() {
            ::expat_rs::stdlib::fprintf(
                fp,
                b" public=\"%s\"\x00".as_ptr() as *const c_char,
                publicId,
            );
        }
        fputs(b" system=\"\x00".as_ptr() as *const c_char, fp);
        characterData(data as *mut c_void, systemId, strlen(systemId) as c_int);
        putc('\"' as i32, fp);
        metaLocation(parser);
        fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
    };
}

unsafe extern "C" fn metaStartNamespaceDecl(
    mut userData: *mut c_void,
    mut prefix: *const XML_Char,
    mut uri: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fputs(b"<startns\x00".as_ptr() as *const c_char, fp);
    if !prefix.is_null() {
        ::expat_rs::stdlib::fprintf(fp, b" prefix=\"%s\"\x00".as_ptr() as *const c_char, prefix);
    }
    if !uri.is_null() {
        fputs(b" ns=\"\x00".as_ptr() as *const c_char, fp);
        characterData(data as *mut c_void, uri, strlen(uri) as c_int);
        fputs(b"\"/>\n\x00".as_ptr() as *const c_char, fp);
    } else {
        fputs(b"/>\n\x00".as_ptr() as *const c_char, fp);
    };
}

unsafe extern "C" fn metaEndNamespaceDecl(mut userData: *mut c_void, mut prefix: *const XML_Char) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = *(parser as *mut *mut c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    if prefix.is_null() {
        fputs(b"<endns/>\n\x00".as_ptr() as *const c_char, fp);
    } else {
        ::expat_rs::stdlib::fprintf(
            fp,
            b"<endns prefix=\"%s\"/>\n\x00".as_ptr() as *const c_char,
            prefix,
        );
    };
}

unsafe extern "C" fn unknownEncodingConvert(mut data: *mut c_void, mut p: *const c_char) -> c_int {
    return crate::codepage::codepageConvert(*(data as *mut c_int), p);
}

unsafe extern "C" fn unknownEncoding(
    mut _userData: *mut c_void,
    mut name: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> c_int {
    let mut cp: c_int = 0;
    static mut prefixL: [XML_Char; 9] = [119, 105, 110, 100, 111, 119, 115, 45, 0];
    static mut prefixU: [XML_Char; 9] = [87, 73, 78, 68, 79, 87, 83, 45, 0];
    let mut i: c_int = 0;
    i = 0;
    while prefixU[i as usize] != 0 {
        if *name.offset(i as isize) as c_int != prefixU[i as usize] as c_int
            && *name.offset(i as isize) as c_int != prefixL[i as usize] as c_int
        {
            return 0i32;
        }
        i += 1
    }
    cp = 0;
    while *name.offset(i as isize) != 0 {
        static mut digits: [XML_Char; 11] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 0];
        let mut s: *const XML_Char = strchr(digits.as_ptr(), *name.offset(i as isize) as c_int);
        if s.is_null() {
            return 0i32;
        }
        cp *= 10;
        cp += s.wrapping_offset_from(digits.as_ptr()) as c_int;
        if cp >= 0x10000 {
            return 0i32;
        }
        i += 1
    }
    if crate::codepage::codepageMap(cp, (*info).map.as_mut_ptr()) == 0 {
        return 0i32;
    }
    (*info).convert = Some(
        unknownEncodingConvert as unsafe extern "C" fn(_: *mut c_void, _: *const c_char) -> c_int,
    );
    /* We could just cast the code page integer to a void *,
    and avoid the use of release. */
    (*info).release = Some(free as unsafe extern "C" fn(_: *mut c_void) -> ());
    (*info).data = malloc(::std::mem::size_of::<c_int>() as c_ulong);
    if (*info).data.is_null() {
        return 0i32;
    }
    *((*info).data as *mut c_int) = cp;
    return 1;
}

unsafe extern "C" fn notStandalone(mut _userData: *mut c_void) -> c_int {
    return 0;
}

unsafe extern "C" fn showVersion(mut prog: *mut XML_Char) {
    let mut s: *mut XML_Char = prog;
    let mut ch: XML_Char = 0;
    let mut features: *const XML_Feature = XML_GetFeatureList();
    loop {
        ch = *s;
        if !(ch as c_int != 0) {
            break;
        }
        if ch as c_int == '/' as i32 {
            prog = s.offset(1)
        }
        s = s.offset(1)
    }
    ::expat_rs::stdlib::fprintf(
        stdout,
        b"%s using %s\n\x00".as_ptr() as *const c_char,
        prog,
        XML_ExpatVersion(),
    );
    if !features.is_null() && (*features.offset(0)).feature != XML_FEATURE_END {
        let mut i: c_int = 1;
        ::expat_rs::stdlib::fprintf(
            stdout,
            b"%s\x00".as_ptr() as *const c_char,
            (*features.offset(0isize)).name,
        );
        if (*features.offset(0)).value != 0 {
            ::expat_rs::stdlib::fprintf(
                stdout,
                b"=%ld\x00".as_ptr() as *const c_char,
                (*features.offset(0isize)).value,
            );
        }
        while (*features.offset(i as isize)).feature != XML_FEATURE_END {
            ::expat_rs::stdlib::fprintf(
                stdout,
                b", %s\x00".as_ptr() as *const c_char,
                (*features.offset(i as isize)).name,
            );
            if (*features.offset(i as isize)).value != 0 {
                ::expat_rs::stdlib::fprintf(
                    stdout,
                    b"=%ld\x00".as_ptr() as *const c_char,
                    (*features.offset(i as isize)).value,
                );
            }
            i += 1
        }
        ::expat_rs::stdlib::fprintf(stdout, b"\n\x00".as_ptr() as *const c_char);
    };
}

unsafe extern "C" fn usage(mut prog: *const XML_Char, mut rc: c_int) {
    ::expat_rs::stdlib::fprintf(stderr,
            
            b"usage: %s [-s] [-n] [-p] [-x] [-e ENCODING] [-w] [-r] [-d DIRECTORY]\n             [-c | -m | -t] [-N]\n             [FILE [FILE ...]]\n\nxmlwf - Determines if an XML document is well-formed\n\npositional arguments:\n  FILE          files to process (default: STDIN)\n\ninput control arguments:\n  -s            print an error if the document is not [s]tandalone\n  -n            enable [n]amespace processing\n  -p            enable processing external DTDs and [p]arameter entities\n  -x            enable processing of e[x]ternal entities\n  -e ENCODING   override any in-document [e]ncoding declaration\n  -w            enable support for [W]indows code pages\n  -r            disable memory-mapping and use normal file [r]ead IO calls instead\n\noutput control arguments:\n  -d DIRECTORY  output [d]estination directory\n  -c            write a [c]opy of input XML, not canonical XML\n  -m            write [m]eta XML, not canonical XML\n  -t            write no XML output for [t]iming of plain parsing\n  -N            enable adding doctype and [n]otation declarations\n\ninfo arguments:\n  -h            show this [h]elp message and exit\n  -v            show program\'s [v]ersion number and exit\n\nlibexpat is software libre, licensed under the MIT license.\nPlease report bugs at https://github.com/libexpat/libexpat/issues.  Thank you!\n\x00".as_ptr() as *const c_char, prog);
    exit(rc);
}

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut XML_Char) -> c_int {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut outputDir: *const XML_Char = ::expat_rs::stddef_h::NULL as *const XML_Char;
    let mut encoding: *const XML_Char = ::expat_rs::stddef_h::NULL as *const XML_Char;
    let mut processFlags: c_uint = crate::xmlfile::XML_MAP_FILE as c_uint;
    let mut windowsCodePages: c_int = 0;
    let mut outputType: c_int = 0;
    let mut useNamespaces: c_int = 0;
    let mut requireStandalone: c_int = 0;
    let mut requiresNotations: c_int = 0;
    let mut paramEntityParsing: XML_ParamEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
    let mut useStdin: c_int = 0;
    let mut userData: XmlwfUserData = {
        let mut init = xmlwfUserData {
            fp: ::expat_rs::stddef_h::NULL as *mut FILE,
            notationListHead: ::expat_rs::stddef_h::NULL as *mut NotationList,
            currentDoctypeName: ::expat_rs::stddef_h::NULL as *const XML_Char,
        };
        init
    };
    i = 1;
    j = 0;
    while i < argc {
        if j == 0 {
            if *(*argv.offset(i as isize)).offset(0) as c_int != '-' as i32 {
                break;
            }
            if *(*argv.offset(i as isize)).offset(1) as c_int == '-' as i32
                && *(*argv.offset(i as isize)).offset(2) as c_int == '\u{0}' as i32
            {
                i += 1;
                break;
            } else {
                j += 1
            }
        }
        let mut current_block_46: u64;
        match *(*argv.offset(i as isize)).offset(j as isize) as c_int {
            114 => {
                processFlags &= !crate::xmlfile::XML_MAP_FILE as c_uint;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            115 => {
                requireStandalone = 1;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            110 => {
                useNamespaces = 1;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            112 => {
                paramEntityParsing = XML_PARAM_ENTITY_PARSING_ALWAYS;
                current_block_46 = 4092296097885336037;
            }
            120 => {
                current_block_46 = 4092296097885336037;
            }
            119 => {
                windowsCodePages = 1;
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
                useNamespaces = 0;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            116 => {
                outputType = 't' as i32;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            78 => {
                requiresNotations = 1;
                j += 1;
                current_block_46 = 13707613154239713890;
            }
            100 => {
                if *(*argv.offset(i as isize)).offset((j + 1) as isize) as c_int == '\u{0}' as i32 {
                    i += 1;
                    if i == argc {
                        usage(*argv.offset(0isize), 2i32);
                    }
                    outputDir = *argv.offset(i as isize)
                } else {
                    outputDir = (*argv.offset(i as isize)).offset(j as isize).offset(1)
                }
                i += 1;
                j = 0;
                current_block_46 = 13707613154239713890;
            }
            101 => {
                if *(*argv.offset(i as isize)).offset((j + 1) as isize) as c_int == '\u{0}' as i32 {
                    i += 1;
                    if i == argc {
                        usage(*argv.offset(0isize), 2i32);
                    }
                    encoding = *argv.offset(i as isize)
                } else {
                    encoding = (*argv.offset(i as isize)).offset(j as isize).offset(1)
                }
                i += 1;
                j = 0;
                current_block_46 = 13707613154239713890;
            }
            104 => {
                usage(*argv.offset(0), 0);
                return 0i32;
            }
            118 => {
                showVersion(*argv.offset(0));
                return 0i32;
            }
            0 => {
                if j > 1 {
                    i += 1;
                    j = 0;
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
                processFlags |= crate::xmlfile::XML_EXTERNAL_ENTITIES as c_uint;
                j += 1
            }
            16871217396860862036 =>
            /* fall through */
            {
                usage(*argv.offset(0isize), 2i32);
            }
            _ => {}
        }
    }
    if i == argc {
        useStdin = 1;
        processFlags &= !crate::xmlfile::XML_MAP_FILE as c_uint;
        i -= 1
    }
    while i < argc {
        let mut outName: *mut XML_Char = 0 as *mut XML_Char;
        let mut result: c_int = 0;
        let mut parser: XML_Parser = 0 as *mut ::expat_rs::expat_h::XML_ParserStruct;
        if useNamespaces != 0 {
            parser = XML_ParserCreateNS(encoding, '\u{1}' as XML_Char)
        } else {
            parser = XML_ParserCreate(encoding)
        }
        if parser.is_null() {
            crate::xmlfile::perror(b"Could not instantiate parser\x00".as_ptr() as *const c_char);
            exit(1i32);
        }
        if requireStandalone != 0 {
            XML_SetNotStandaloneHandler(
                parser,
                Some(notStandalone as unsafe extern "C" fn(_: *mut c_void) -> c_int),
            );
        }
        XML_SetParamEntityParsing(parser, paramEntityParsing);
        if outputType == 't' as i32 {
            /* This is for doing timings; this gives a more realistic estimate of
            the parsing time. */
            outputDir = 0 as *const XML_Char;
            XML_SetElementHandler(
                parser,
                Some(
                    nopStartElement
                        as unsafe extern "C" fn(
                            _: *mut c_void,
                            _: *const XML_Char,
                            _: *mut *const XML_Char,
                        ) -> (),
                ),
                Some(
                    nopEndElement as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
                ),
            );
            XML_SetCharacterDataHandler(
                parser,
                Some(
                    nopCharacterData
                        as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char, _: c_int) -> (),
                ),
            );
            XML_SetProcessingInstructionHandler(
                parser,
                Some(
                    nopProcessingInstruction
                        as unsafe extern "C" fn(
                            _: *mut c_void,
                            _: *const XML_Char,
                            _: *const XML_Char,
                        ) -> (),
                ),
            );
        } else if !outputDir.is_null() {
            let mut delim: *const XML_Char = b"/\x00".as_ptr() as *const c_char;
            let mut file: *const XML_Char = if useStdin != 0 {
                b"STDIN\x00".as_ptr() as *const c_char
            } else {
                *argv.offset(i as isize) as *const c_char
            };
            if useStdin == 0 {
                /* Jump after last (back)slash */
                let mut lastDelim: *const XML_Char = strrchr(file, *delim.offset(0) as c_int);
                if !lastDelim.is_null() {
                    file = lastDelim.offset(1)
                }
            }
            outName = malloc(
                strlen(outputDir)
                    .wrapping_add(strlen(file))
                    .wrapping_add(2u64)
                    .wrapping_mul(::std::mem::size_of::<XML_Char>() as c_ulong),
            ) as *mut XML_Char;
            strcpy(outName, outputDir);
            strcat(outName, delim);
            strcat(outName, file);
            userData.fp = fopen(outName, b"wb\x00".as_ptr() as *const c_char);
            if userData.fp.is_null() {
                crate::xmlfile::perror(outName);
                exit(1i32);
            }
            setvbuf(
                userData.fp,
                ::expat_rs::stddef_h::NULL as *mut c_char,
                _IOFBF,
                16384,
            );
            XML_SetUserData(parser, &mut userData as *mut XmlwfUserData as *mut c_void);
            match outputType {
                109 => {
                    XML_UseParserAsHandlerArg(parser);
                    XML_SetElementHandler(
                        parser,
                        Some(
                            metaStartElement
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: *mut *const XML_Char,
                                ) -> (),
                        ),
                        Some(
                            metaEndElement
                                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
                        ),
                    );
                    XML_SetProcessingInstructionHandler(
                        parser,
                        Some(
                            metaProcessingInstruction
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: *const XML_Char,
                                ) -> (),
                        ),
                    );
                    XML_SetCommentHandler(
                        parser,
                        Some(
                            metaComment
                                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
                        ),
                    );
                    XML_SetCdataSectionHandler(
                        parser,
                        Some(metaStartCdataSection as unsafe extern "C" fn(_: *mut c_void) -> ()),
                        Some(metaEndCdataSection as unsafe extern "C" fn(_: *mut c_void) -> ()),
                    );
                    XML_SetCharacterDataHandler(
                        parser,
                        Some(
                            metaCharacterData
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: c_int,
                                ) -> (),
                        ),
                    );
                    XML_SetDoctypeDeclHandler(
                        parser,
                        Some(
                            metaStartDoctypeDecl
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: *const XML_Char,
                                    _: *const XML_Char,
                                    _: c_int,
                                ) -> (),
                        ),
                        Some(metaEndDoctypeDecl as unsafe extern "C" fn(_: *mut c_void) -> ()),
                    );
                    XML_SetEntityDeclHandler(
                        parser,
                        Some(
                            metaEntityDecl
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
                        parser,
                        Some(
                            metaNotationDecl
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: *const XML_Char,
                                    _: *const XML_Char,
                                    _: *const XML_Char,
                                ) -> (),
                        ),
                    );
                    XML_SetNamespaceDeclHandler(
                        parser,
                        Some(
                            metaStartNamespaceDecl
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: *const XML_Char,
                                ) -> (),
                        ),
                        Some(
                            metaEndNamespaceDecl
                                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
                        ),
                    );
                    metaStartDocument(parser as *mut c_void);
                }
                99 => {
                    XML_UseParserAsHandlerArg(parser);
                    XML_SetDefaultHandler(
                        parser,
                        Some(
                            markup
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: c_int,
                                ) -> (),
                        ),
                    );
                    XML_SetElementHandler(
                        parser,
                        Some(
                            defaultStartElement
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: *mut *const XML_Char,
                                ) -> (),
                        ),
                        Some(
                            defaultEndElement
                                as unsafe extern "C" fn(_: *mut c_void, _: *const XML_Char) -> (),
                        ),
                    );
                    XML_SetCharacterDataHandler(
                        parser,
                        Some(
                            defaultCharacterData
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: c_int,
                                ) -> (),
                        ),
                    );
                    XML_SetProcessingInstructionHandler(
                        parser,
                        Some(
                            defaultProcessingInstruction
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: *const XML_Char,
                                ) -> (),
                        ),
                    );
                }
                _ => {
                    if useNamespaces != 0 {
                        XML_SetElementHandler(
                            parser,
                            Some(
                                startElementNS
                                    as unsafe extern "C" fn(
                                        _: *mut c_void,
                                        _: *const XML_Char,
                                        _: *mut *const XML_Char,
                                    )
                                        -> (),
                            ),
                            Some(
                                endElementNS
                                    as unsafe extern "C" fn(
                                        _: *mut c_void,
                                        _: *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                    } else {
                        XML_SetElementHandler(
                            parser,
                            Some(
                                startElement
                                    as unsafe extern "C" fn(
                                        _: *mut c_void,
                                        _: *const XML_Char,
                                        _: *mut *const XML_Char,
                                    )
                                        -> (),
                            ),
                            Some(
                                endElement
                                    as unsafe extern "C" fn(
                                        _: *mut c_void,
                                        _: *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                    }
                    XML_SetCharacterDataHandler(
                        parser,
                        Some(
                            characterData
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: c_int,
                                ) -> (),
                        ),
                    );
                    XML_SetProcessingInstructionHandler(
                        parser,
                        Some(
                            processingInstruction
                                as unsafe extern "C" fn(
                                    _: *mut c_void,
                                    _: *const XML_Char,
                                    _: *const XML_Char,
                                ) -> (),
                        ),
                    );
                    if requiresNotations != 0 {
                        XML_SetDoctypeDeclHandler(
                            parser,
                            Some(
                                startDoctypeDecl
                                    as unsafe extern "C" fn(
                                        _: *mut c_void,
                                        _: *const XML_Char,
                                        _: *const XML_Char,
                                        _: *const XML_Char,
                                        _: c_int,
                                    )
                                        -> (),
                            ),
                            Some(endDoctypeDecl as unsafe extern "C" fn(_: *mut c_void) -> ()),
                        );
                        XML_SetNotationDeclHandler(
                            parser,
                            Some(
                                notationDecl
                                    as unsafe extern "C" fn(
                                        _: *mut c_void,
                                        _: *const XML_Char,
                                        _: *const XML_Char,
                                        _: *const XML_Char,
                                        _: *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                    }
                }
            }
        }
        if windowsCodePages != 0 {
            XML_SetUnknownEncodingHandler(
                parser,
                transmute(Some(
                    unknownEncoding
                        as unsafe extern "C" fn(
                            _: *mut c_void,
                            _: *const XML_Char,
                            _: *mut XML_Encoding,
                        ) -> c_int,
                )),
                0 as *mut c_void,
            );
        }
        result = crate::xmlfile::XML_ProcessFile(
            parser,
            if useStdin != 0 {
                ::expat_rs::stddef_h::NULL as *mut XML_Char
            } else {
                *argv.offset(i as isize)
            },
            processFlags,
        );
        if !outputDir.is_null() {
            if outputType == 'm' as i32 {
                metaEndDocument(parser as *mut c_void);
            }
            fclose(userData.fp);
            if result == 0 {
                remove(outName);
            }
            free(outName as *mut c_void);
        }
        ::expat_rs::lib::xmlparse::XML_ParserFree(parser);
        if result == 0 {
            exit(2i32);
        }
        i += 1
    }
    return 0;
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
    unsafe { ::std::process::exit(main_0((args.len() - 1) as libc::c_int, args.as_mut_ptr())) }
}
