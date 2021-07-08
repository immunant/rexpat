use libc::{c_char, c_int, c_long, size_t};
use std::cell::Cell;
use super::xmltok::{checkCharRefNumber, Attribute, Position};
use super::xmltok::{XML_Convert_Result, XmlEncoding, XmlEncodingImpl, XML_TOK};
use crate::ascii_h::*;
use crate::expat_h::{XML_Error};
pub use crate::expat_external_h::XML_Size;
use crate::xmltok_impl_h::ByteType;
use crate::lib::xmlparse::{ExpatBufRef, ExpatBufRefMut};

macro_rules! MATCH_LEAD_CASES {
    {$e:expr, LEAD_CASE($n:ident) => $case:block $($tail:tt)*} => {
        match $e {
            ByteType::LEAD2 => {
                let $n: isize = 2;
                $case
            }
            ByteType::LEAD3 => {
                let $n: isize = 3;
                $case
            }
            ByteType::LEAD4 => {
                let $n: isize = 4;
                $case
            }
            $($tail)*
        }
    };
}

macro_rules! MATCH_INVALID_CASES {
    {
        ($buf:ident, $nextTokPtr:ident, $self:ident),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $buf.len() < n as usize {
                    return XML_TOK::PARTIAL_CHAR;
                }
                if $self.is_invalid_char($buf.as_ptr(), n) {
                    *$nextTokPtr = $buf.as_ptr();
                    return XML_TOK::INVALID;
                }
                $buf = $buf.inc_start(n);
            }
            ByteType::NONXML | ByteType::MALFORM | ByteType::TRAIL => {
                *$nextTokPtr = $buf.as_ptr();

                return XML_TOK::INVALID;
            }
            $($tail)*
        }
    }
}

macro_rules! CHECK_NAME_CASES {
    {
        ($buf:ident, $nextTokPtr:ident, $self:ident),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $buf.len() < n as usize {
                    return XML_TOK::PARTIAL_CHAR;
                }
                if !$self.is_name_char($buf.as_ptr(), n) {
                    *$nextTokPtr = $buf.as_ptr();
                    return XML_TOK::INVALID;
                }
                $buf = $buf.inc_start(n as isize);
            }
            ByteType::NONASCII if !$self.is_name_char_minbpc($buf.as_ptr()) => {
                *$nextTokPtr = $buf.as_ptr();
                return XML_TOK::INVALID;
            }
            ByteType::NONASCII | ByteType::NMSTRT | ByteType::HEX | ByteType::DIGIT | ByteType::NAME | ByteType::MINUS => {
                $buf = $buf.inc_start($self.MINBPC() as isize);
            }
            $($tail)*
        }
    }
}

macro_rules! CHECK_NMSTRT_CASES {
    {
        ($buf:ident, $nextTokPtr:ident, $self:ident),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $buf.len() < n as usize {
                    return XML_TOK::PARTIAL_CHAR;
                }
                if !$self.is_nmstrt_char($buf.as_ptr(), n) {
                    *$nextTokPtr = $buf.as_ptr();
                    return XML_TOK::INVALID;
                }
                $buf = $buf.inc_start(n as isize);
            }
            ByteType::NONASCII if !$self.is_nmstrt_char_minbpc($buf.as_ptr()) => {
                *$nextTokPtr = $buf.as_ptr();
                return XML_TOK::INVALID;
            }
            ByteType::NONASCII | ByteType::NMSTRT | ByteType::HEX => {
                $buf = $buf.inc_start($self.MINBPC() as isize);
            }
            $($tail)*
        }
    }
}


macro_rules! HAS_CHARS {
    ($buf:ident, $count:expr, $self:ident) => {
        $buf.len() as c_long >= ($self.MINBPC() * $count) as c_long
    };
}

macro_rules! HAS_CHAR {
    ($buf:ident, $self:ident) => {
        HAS_CHARS!($buf, 1, $self)
    };
}

macro_rules! REQUIRE_CHARS {
    ($buf:ident, $count:expr, $self:ident) => {
        if !HAS_CHARS!($buf, $count, $self) {
            return XML_TOK::PARTIAL;
        }
    };
}

macro_rules! REQUIRE_CHAR {
    ($buf:ident, $self:ident) => {
        REQUIRE_CHARS!($buf, 1, $self)
    };
}


pub trait XmlTokImpl: XmlEncodingImpl {
    /* ptr points to character following "<!-" */
    fn scanComment(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        if HAS_CHAR!(buf, self) {
            if !self.char_matches(buf.as_ptr(), ASCII_MINUS) {
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::INVALID;
            }
            buf = buf.inc_start((self.MINBPC()) as isize);
            while HAS_CHAR!(buf, self) {
                MATCH_INVALID_CASES! {
                    (buf, nextTokPtr, self),
                    match self.byte_type(buf.as_ptr()),
                    ByteType::MINUS => {
                        buf = buf.inc_start((self.MINBPC()) as isize);
                        REQUIRE_CHAR!(buf, self);
                        if self.char_matches(buf.as_ptr(), ASCII_MINUS) {
                            buf = buf.inc_start((self.MINBPC()) as isize);
                            REQUIRE_CHAR!(buf, self);
                            if !self.char_matches(buf.as_ptr(), ASCII_GT) {
                                *nextTokPtr = buf.as_ptr();
                                return XML_TOK::INVALID
                            }
                            *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                            return XML_TOK::COMMENT
                        }
                    }
                    _ => { buf = buf.inc_start((self.MINBPC()) as isize); }
                }
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following "<!" */
    fn scanDecl(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        REQUIRE_CHAR!(buf, self);
        match self.byte_type(buf.as_ptr()) {
            ByteType::MINUS => return self.scanComment(buf.inc_start(self.MINBPC() as isize), nextTokPtr),
            ByteType::LSQB => {
                *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                return XML_TOK::COND_SECT_OPEN;
            }
            ByteType::NMSTRT | ByteType::HEX => buf = buf.inc_start((self.MINBPC()) as isize),
            _ => {
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::INVALID;
            }
        }
        while HAS_CHAR!(buf, self) {
            match self.byte_type(buf.as_ptr()) {
                ByteType::PERCNT => {
                    REQUIRE_CHARS!(buf, 2, self);
                    /* don't allow <!ENTITY% foo "whatever"> */
                    match self.byte_type(buf.inc_start(self.MINBPC()).as_ptr()) {
                        ByteType::S | ByteType::CR | ByteType::LF | ByteType::PERCNT => {
                            *nextTokPtr = buf.as_ptr();
                            return XML_TOK::INVALID;
                        }
                        _ => {}
                    }
                }
                ByteType::S | ByteType::CR | ByteType::LF => {}
                ByteType::NMSTRT | ByteType::HEX => {
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    continue;
                }
                _ => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::INVALID;
                }
            }
            /* fall through */
            *nextTokPtr = buf.as_ptr();
            return XML_TOK::DECL_OPEN;
        }
        XML_TOK::PARTIAL
    }
    fn checkPiTarget(
        &self,
        mut buf: ExpatBufRef,
        mut tokPtr: &mut XML_TOK,
    ) -> libc::c_int {
        let mut upper: libc::c_int = 0 as libc::c_int;
        *tokPtr = XML_TOK::PI;
        if buf.len() as libc::c_long != (self.MINBPC() * 3) as libc::c_long {
            return 1 as libc::c_int;
        }
        match self.byte_to_ascii(buf.as_ptr()) {
            ASCII_x => {}
            ASCII_X => upper = 1 as libc::c_int,
            _ => return 1 as libc::c_int,
        }
        buf = buf.inc_start((self.MINBPC()) as isize);
        match self.byte_to_ascii(buf.as_ptr()) {
            ASCII_m => {}
            ASCII_M => upper = 1 as libc::c_int,
            _ => return 1 as libc::c_int,
        }
        buf = buf.inc_start((self.MINBPC()) as isize);
        match self.byte_to_ascii(buf.as_ptr()) {
            ASCII_l => {}
            ASCII_L => upper = 1 as libc::c_int,
            _ => return 1 as libc::c_int,
        }
        if upper != 0 {
            return 0 as libc::c_int;
        }
        *tokPtr = XML_TOK::XML_DECL;
        1 as libc::c_int
    }

    /* ptr points to character following "<?" */
    fn scanPi(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        let mut tok = XML_TOK::INVALID;
        let target = buf.clone();
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                ByteType::S | ByteType::CR | ByteType::LF => {
                    if self.checkPiTarget(target.with_end(buf.as_ptr()), &mut tok) == 0
                       {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::INVALID
                    }
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    while HAS_CHAR!(buf, self) {
                        MATCH_INVALID_CASES! {
                            (buf, nextTokPtr, self),
                            match self.byte_type(buf.as_ptr()),
                            ByteType::QUEST => {
                                buf = buf.inc_start((self.MINBPC()) as isize);
                                REQUIRE_CHAR!(buf, self);
                                if self.char_matches(buf.as_ptr(), ASCII_GT) {
                                    *nextTokPtr =
                                        buf.inc_start(self.MINBPC()).as_ptr();
                                    return tok
                                }
                            }
                            _ => { buf = buf.inc_start((self.MINBPC()) as isize) }
                        }
                    }
                    return XML_TOK::PARTIAL
                }
                ByteType::QUEST => {
                    if self.checkPiTarget(target.with_end(buf.as_ptr()), &mut tok) == 0
                       {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::INVALID
                    }
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    if self.char_matches(buf.as_ptr(), ASCII_GT) {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return tok
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::INVALID
                }
                _ => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::INVALID
                }
            }
        }
        XML_TOK::PARTIAL
    }
    fn scanCdataSection(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        pub static CDATA_LSQB: [libc::c_char; 6] = [
            ASCII_C as libc::c_char,
            ASCII_D as libc::c_char,
            ASCII_A as libc::c_char,
            ASCII_T as libc::c_char,
            ASCII_A as libc::c_char,
            ASCII_LSQB as libc::c_char,
        ];
        let mut i: libc::c_int = 0;
        /* CDATA[ */
        REQUIRE_CHARS!(buf, 6, self);
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            if !self.char_matches(buf.as_ptr(), CDATA_LSQB[i as usize]) {
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::INVALID;
            }
            i += 1;
            buf = buf.inc_start((self.MINBPC()) as isize)
        }
        *nextTokPtr = buf.as_ptr();
        XML_TOK::CDATA_SECT_OPEN
    }

    /* ptr points to character following "</" */
    fn scanEndTag(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                ByteType::S | ByteType::CR | ByteType::LF => {
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    while HAS_CHAR!(buf, self) {
                        match self.byte_type(buf.as_ptr()) {
                            ByteType::S | ByteType::CR | ByteType::LF => { }
                            ByteType::GT => {
                                *nextTokPtr =
                                    buf.inc_start(self.MINBPC()).as_ptr();
                                return XML_TOK::END_TAG
                            }
                            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
                        }
                        buf = buf.inc_start((self.MINBPC()) as isize)
                    }
                    return XML_TOK::PARTIAL
                }
                ByteType::COLON => {
                    /* no need to check qname syntax here,
                    since end-tag must match exactly */
                    buf = buf.inc_start((self.MINBPC()) as isize);
                }
                ByteType::GT => {
                    *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                    return XML_TOK::END_TAG
                }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following "&#X" */
    fn scanHexCharRef(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        if HAS_CHAR!(buf, self) {
            match self.byte_type(buf.as_ptr()) {
                ByteType::DIGIT | ByteType::HEX => {}
                _ => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::INVALID;
                }
            }
            buf = buf.inc_start((self.MINBPC()) as isize);
            while HAS_CHAR!(buf, self) {
                match self.byte_type(buf.as_ptr()) {
                    ByteType::DIGIT | ByteType::HEX => {}
                    ByteType::SEMI => {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::INVALID;
                    }
                }
                buf = buf.inc_start((self.MINBPC()) as isize)
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following "&#" */
    fn scanCharRef(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        if HAS_CHAR!(buf, self) {
            if self.char_matches(buf.as_ptr(), ASCII_x) {
                return self.scanHexCharRef(buf.inc_start(self.MINBPC() as isize), nextTokPtr);
            }
            match self.byte_type(buf.as_ptr()) {
                ByteType::DIGIT => {}
                _ => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::INVALID;
                }
            }
            buf = buf.inc_start((self.MINBPC()) as isize);
            while HAS_CHAR!(buf, self) {
                match self.byte_type(buf.as_ptr()) {
                    ByteType::DIGIT => {}
                    ByteType::SEMI => {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::INVALID;
                    }
                }
                buf = buf.inc_start((self.MINBPC()) as isize)
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following "&" */
    fn scanRef(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            ByteType::NUM => {
                return self.scanCharRef(
                    buf.inc_start((self.MINBPC()) as isize),
                    nextTokPtr,
                )
            }
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                ByteType::SEMI => {
                    *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                    return XML_TOK::ENTITY_REF
                }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following first character of attribute name */
    fn scanAtts(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        #[derive(PartialEq)]
        enum Label {
            None,
            Sol,
            Gt,
            EqFallthrough,
        }

        let mut hadColon: libc::c_int = 0 as libc::c_int;
        while HAS_CHAR!(buf, self) {
            let mut current_block: Label = Label::None;
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                ByteType::COLON => {
                    if hadColon != 0 {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::INVALID
                    }
                    hadColon = 1;
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    CHECK_NMSTRT_CASES! {
                        (buf, nextTokPtr, self),
                        match self.byte_type(buf.as_ptr()),
                        _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
                    }
                }
                ByteType::S | ByteType::CR | ByteType::LF => {
                    loop  {
                        buf = buf.inc_start((self.MINBPC()) as isize);
                        REQUIRE_CHAR!(buf, self);
                        let mut t = self.byte_type(buf.as_ptr());
                        if t == ByteType::EQUALS { break ; }
                        match t {
                            ByteType::S | ByteType::LF | ByteType::CR => { }
                            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
                        }
                    }
                    current_block = Label::EqFallthrough;
                }
                ByteType::EQUALS => { current_block = Label::EqFallthrough; }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
            }
            if current_block == Label::EqFallthrough {
                /* fall through */
                // ByteType::S | ByteType::CR | ByteType::LF | ByteType::EQUALS =>
                let mut open  = ByteType::NONXML;
                hadColon = 0;
                loop {
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    open = self.byte_type(buf.as_ptr());
                    if open == ByteType::QUOT || open == ByteType::APOS {
                        break;
                    }
                    match open {
                        ByteType::S | ByteType::LF | ByteType::CR => {}
                        _ => {
                            *nextTokPtr = buf.as_ptr();
                            return XML_TOK::INVALID;
                        }
                    }
                }
                buf = buf.inc_start((self.MINBPC()) as isize);
                /* in attribute value */
                loop {
                    REQUIRE_CHAR!(buf, self);
                    let mut t = self.byte_type(buf.as_ptr());
                    if t == open {
                        break;
                    }
                    MATCH_INVALID_CASES! {
                        (buf, nextTokPtr, self),
                        match t,
                        ByteType::AMP => {
                            let mut ptr = buf.as_ptr();
                            let mut tok = self.scanRef(buf.inc_start(self.MINBPC() as isize), &mut ptr);
                            buf = buf.with_start(ptr);
                            if tok.is_error() {
                                if tok == XML_TOK::INVALID {
                                    *nextTokPtr = buf.as_ptr()
                                }
                                return tok
                            }
                        }
                        ByteType::LT => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
                        _ => { buf = buf.inc_start((self.MINBPC()) as isize) }
                    }
                }
                buf = buf.inc_start((self.MINBPC()) as isize);
                REQUIRE_CHAR!(buf, self);
                match self.byte_type(buf.as_ptr()) {
                    ByteType::S | ByteType::CR | ByteType::LF => { }
                    ByteType::SOL => {
                        // goto sol;
                        current_block = Label::Sol;
                    }
                    ByteType::GT => {
                        // goto gt;
                        current_block = Label::Gt;
                    }
                    _ => {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::INVALID;
                    }
                }
                match current_block {
                    Label::Sol | Label::Gt => { }
                    _ => {
                        // Didn't take goto
                        /* ptr points to closing quote */
                        loop {
                            buf = buf.inc_start((self.MINBPC()) as isize);
                            REQUIRE_CHAR!(buf, self);
                            CHECK_NMSTRT_CASES! {
                                (buf, nextTokPtr, self),
                                match self.byte_type(buf.as_ptr()),
                                ByteType::S | ByteType::CR | ByteType::LF => {
                                    continue;
                                }
                                ByteType::GT => {
                                    current_block = Label::Gt;
                                    break ;
                                }
                                ByteType::SOL => {
                                    current_block = Label::Sol;
                                    break ;
                                }
                                _ => {
                                    *nextTokPtr = buf.as_ptr();
                                    return XML_TOK::INVALID
                                }
                            }
                            break;
                        }
                    }
                }

                match current_block {
                    Label::Gt => {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::START_TAG_WITH_ATTS;
                    }
                    Label::Sol => {
                        buf = buf.inc_start((self.MINBPC()) as isize);
                        REQUIRE_CHAR!(buf, self);
                        if !self.char_matches(buf.as_ptr(), ASCII_GT) {
                            *nextTokPtr = buf.as_ptr();
                            return XML_TOK::INVALID;
                        }
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::EMPTY_ELEMENT_WITH_ATTS;
                    }
                    _ => { }
                }
            }
        }
        XML_TOK::PARTIAL
    }

    fn scanLtHelper(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        buf = buf.inc_start((self.MINBPC()) as isize);
        REQUIRE_CHAR!(buf, self);
        if !self.char_matches(buf.as_ptr(), ASCII_GT) {
            *nextTokPtr = buf.as_ptr();
            return XML_TOK::INVALID;
        }
        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
        XML_TOK::EMPTY_ELEMENT_NO_ATTS
    }

    /* ptr points to character following "<" */
    fn scanLt(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            ByteType::EXCL => {
                buf = buf.inc_start((self.MINBPC()) as isize);
                REQUIRE_CHAR!(buf, self);
                match self.byte_type(buf.as_ptr()) {
                    ByteType::MINUS => {
                        return self.scanComment(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
                    }
                    ByteType::LSQB => {
                        return self.scanCdataSection(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
                    }
                    _ => { }
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::INVALID
            }
            ByteType::QUEST => {
                return self.scanPi(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            ByteType::SOL => {
                return self.scanEndTag(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
        }
        let mut hadColon = false;
        /* we have a start-tag */
        /* we have a start-tag */
        /* we have a start-tag */
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                ByteType::COLON => {
                    if hadColon {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::INVALID
                    }
                    hadColon = true;
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    CHECK_NMSTRT_CASES! {
                        (buf, nextTokPtr, self),
                        match self.byte_type(buf.as_ptr()),
                        _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
                    }
                }
                ByteType::S | ByteType::CR | ByteType::LF => {
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    loop  {
                        if !(HAS_CHAR!(buf, self)) {
                            break ;
                        }
                        CHECK_NMSTRT_CASES! {
                            (buf, nextTokPtr, self),
                            match self.byte_type(buf.as_ptr()),
                            ByteType::GT => {
                                *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                                return XML_TOK::START_TAG_NO_ATTS;
                            }
                            ByteType::SOL => {
                                return self.scanLtHelper(buf, nextTokPtr);
                            }
                            ByteType::S | ByteType::CR | ByteType::LF => {
                                buf = buf.inc_start((self.MINBPC()) as isize);
                                continue ;
                            }
                            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
                        }
                        return self.scanAtts(buf, nextTokPtr)
                    }
                    return XML_TOK::PARTIAL;
                }
                ByteType::GT => {
                    *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                    return XML_TOK::START_TAG_NO_ATTS;
                 }
                ByteType::SOL => { return self.scanLtHelper(buf, nextTokPtr); }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following "%" */
    fn scanPercent(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            ByteType::S | ByteType::LF | ByteType::CR | ByteType::PERCNT => { *nextTokPtr = buf.as_ptr(); return XML_TOK::PERCENT }
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                ByteType::SEMI => {
                    *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                    return XML_TOK::PARAM_ENTITY_REF
                }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
            }
        }
        XML_TOK::PARTIAL
    }
    fn scanPoundName(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                ByteType::CR | ByteType::LF | ByteType::S | ByteType::RPAR | ByteType::GT | ByteType::PERCNT | ByteType::VERBAR => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::POUND_NAME
                }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
            }
        }
        return XML_TOK::POUND_NAME_NEG;
    }
    fn scanLit(
        &self,
        mut open: ByteType,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        while HAS_CHAR!(buf, self) {
            let mut t = self.byte_type(buf.as_ptr());
            MATCH_INVALID_CASES! {
                (buf, nextTokPtr, self),
                match t,
                ByteType::QUOT | ByteType::APOS => {
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    if t == open {
                        if !HAS_CHAR!(buf, self) {
                            return XML_TOK::LITERAL_NEG
                        }
                        *nextTokPtr = buf.as_ptr();
                        match self.byte_type(buf.as_ptr()) {
                            ByteType::S | ByteType::CR | ByteType::LF | ByteType::GT | ByteType::PERCNT | ByteType::LSQB => {
                                return XML_TOK::LITERAL
                            }
                            _ => { return XML_TOK::INVALID }
                        }
                    }
                }
                _ => { buf = buf.inc_start((self.MINBPC()) as isize) }
            }
        }
        XML_TOK::PARTIAL
    }
}

impl<T: XmlEncodingImpl> XmlTokImpl for T { }

impl<T: XmlEncodingImpl+XmlTokImpl> XmlEncoding for T {
    fn cdataSectionTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        if buf.is_empty() {
            return XML_TOK::NONE;
        }
        if self.MINBPC() > 1 {
            let mut n: size_t = buf.len();
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK::PARTIAL;
                }
                buf = buf.with_len(n);
            }
        }
        MATCH_INVALID_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            ByteType::RSQB => {
                buf = buf.inc_start((self.MINBPC()) as isize);
                REQUIRE_CHAR!(buf, self);
                if self.char_matches(buf.as_ptr(), ASCII_RSQB) {
                    let prev_buf = buf.clone();
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    if !self.char_matches(buf.as_ptr(), ASCII_GT) {
                        buf = prev_buf;
                    } else {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::CDATA_SECT_CLOSE
                    }
                }
            }
            ByteType::CR => {
                buf = buf.inc_start((self.MINBPC()) as isize);
                REQUIRE_CHAR!(buf, self);
                if self.byte_type(buf.as_ptr()) == ByteType::LF {
                    buf = buf.inc_start((self.MINBPC()) as isize)
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::DATA_NEWLINE
            }
            ByteType::LF => {
                *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                return XML_TOK::DATA_NEWLINE
            }
            _ => { buf = buf.inc_start((self.MINBPC()) as isize) }
        }
        while HAS_CHAR!(buf, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(buf.as_ptr()),
                LEAD_CASE(n) => {
                    if buf.len() < n as usize || self.is_invalid_char(buf.as_ptr(), n) {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::DATA_CHARS;
                    }
                    buf = buf.inc_start(n as isize);
                }
                ByteType::NONXML | ByteType::MALFORM | ByteType::TRAIL | ByteType::CR | ByteType::LF | ByteType::RSQB => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::DATA_CHARS
                }
                _ => { buf = buf.inc_start((self.MINBPC()) as isize) }
            }
        }
        *nextTokPtr = buf.as_ptr();
        XML_TOK::DATA_CHARS
    }
    fn contentTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        if buf.is_empty() {
            return XML_TOK::NONE;
        }
        if self.MINBPC() > 1 {
            let mut n: size_t = buf.len() as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK::PARTIAL;
                }
                buf = buf.with_len(n as usize)
            }
        }
        MATCH_INVALID_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            ByteType::LT => {
                return self.scanLt(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            ByteType::AMP => {
                return self.scanRef(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            ByteType::CR => {
                buf = buf.inc_start(self.MINBPC() as isize);
                if !HAS_CHAR!(buf, self) {
                    return XML_TOK::TRAILING_CR
                }
                if self.byte_type(buf.as_ptr()) == ByteType::LF {
                    buf = buf.inc_start(self.MINBPC() as isize)
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::DATA_NEWLINE
            }
            ByteType::LF => {
                *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                return XML_TOK::DATA_NEWLINE
            }
            ByteType::RSQB => {
                buf = buf.inc_start(self.MINBPC() as isize);
                if !HAS_CHAR!(buf, self) {
                    return XML_TOK::TRAILING_RSQB
                }
                if self.char_matches(buf.as_ptr(), ASCII_RSQB) {
                    let prev_buf = buf.clone();
                    buf = buf.inc_start(self.MINBPC() as isize);
                    if !HAS_CHAR!(buf, self) {
                        return XML_TOK::TRAILING_RSQB
                    }
                    if !self.char_matches(buf.as_ptr(), ASCII_GT) {
                        buf = prev_buf;
                    } else { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
                }
            }
            _ => { buf = buf.inc_start(self.MINBPC() as isize) }
        }
        while HAS_CHAR!(buf, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(buf.as_ptr()),
                LEAD_CASE(n) => {
                    if buf.len() < n as usize || self.is_invalid_char(buf.as_ptr(), n) {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::DATA_CHARS;
                    }
                    buf = buf.inc_start(n as isize);
                }
                ByteType::RSQB => {
                    if HAS_CHARS!(buf, 2, self) {
                        if !self.char_matches(buf.inc_start(self.MINBPC()).as_ptr(), ASCII_RSQB) {
                            buf = buf.inc_start(self.MINBPC() as isize);
                        } else if HAS_CHARS!(buf, 3, self) {
                            if !self.char_matches(buf.inc_start(2 * self.MINBPC()).as_ptr(), ASCII_GT) {
                                buf = buf.inc_start(self.MINBPC() as isize)
                            } else {
                                *nextTokPtr = buf.inc_start(2 * self.MINBPC()).as_ptr();
                                return XML_TOK::INVALID
                            }
                        } else {
                            break;
                         }
                    } else {
                        break;
                     }
                }
                ByteType::AMP | ByteType::LT | ByteType::NONXML |
                ByteType::MALFORM | ByteType::TRAIL | ByteType::CR |
                ByteType::LF => {
                    break;
                }
                _ => { buf = buf.inc_start(self.MINBPC() as isize); }
            }
        }
        *nextTokPtr = buf.as_ptr();
        XML_TOK::DATA_CHARS
    }
    fn prologTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        let mut tok = XML_TOK::INVALID;
        if buf.is_empty() {
            return XML_TOK::NONE;
        }
        if self.MINBPC() > 1 {
            let mut n: size_t = buf.len() as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK::PARTIAL;
                }
                buf = buf.with_len(n as usize)
            }
        }
        MATCH_LEAD_CASES! {
            self.byte_type(buf.as_ptr()),
            LEAD_CASE(n) => {
                if (buf.len() as c_long) < n as c_long {
                    return XML_TOK::PARTIAL_CHAR;
                }
                if self.is_nmstrt_char(buf.as_ptr(), n) {
                    buf = buf.inc_start(n as isize);
                    tok = XML_TOK::NAME;
                } else if self.is_name_char(buf.as_ptr(), n) {
                    buf = buf.inc_start(n as isize);
                    tok = XML_TOK::NMTOKEN
                } else {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::INVALID
                }
            }
            ByteType::QUOT => {
                return self.scanLit(ByteType::QUOT,
                                        buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            ByteType::APOS => {
                return self.scanLit(ByteType::APOS,
                                        buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            ByteType::LT => {
                buf = buf.inc_start(self.MINBPC() as isize);
                REQUIRE_CHAR!(buf, self);
                match self.byte_type(buf.as_ptr()) {
                    ByteType::EXCL => {
                        return self.scanDecl(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
                    }
                    ByteType::QUEST => {
                        return self.scanPi(buf.inc_start(self.MINBPC() as isize),
                                               nextTokPtr)
                    }
                    ByteType::NMSTRT | ByteType::HEX | ByteType::NONASCII | ByteType::LEAD2 | ByteType::LEAD3 | ByteType::LEAD4 => {
                        unsafe { *nextTokPtr = buf.as_ptr().offset(-(self.MINBPC())); }
                        return XML_TOK::INSTANCE_START
                    }
                    _ => { }
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::INVALID
            }
            ByteType::CR if buf.len() == self.MINBPC() as usize => {
                *nextTokPtr = buf.end();
                /* indicate that this might be part of a CR/LF pair */
                return XML_TOK::PROLOG_S_NEG
            }
            ByteType::S | ByteType::LF | ByteType::CR => {
                loop {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    if !HAS_CHAR!(buf, self) {
                        break;
                    }
                    let b = self.byte_type(buf.as_ptr());
                    match b {
                        ByteType::CR if buf.len() == self.MINBPC() as usize => {
                            /* don't split CR/LF pair */
                            *nextTokPtr = buf.as_ptr();
                            return XML_TOK::PROLOG_S;
                        }
                        ByteType::S | ByteType::LF | ByteType::CR => {
                            /* do nothing */
                        }
                        _ => {
                            *nextTokPtr = buf.as_ptr();
                            return XML_TOK::PROLOG_S;
                        }
                    }
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::PROLOG_S;
            }
            ByteType::PERCNT => {
                return self.scanPercent(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            ByteType::COMMA => {
                *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                return XML_TOK::COMMA
            }
            ByteType::LSQB => {
                *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                return XML_TOK::OPEN_BRACKET
            }
            ByteType::RSQB => {
                buf = buf.inc_start(self.MINBPC() as isize);
                if !HAS_CHAR!(buf, self) {
                    return XML_TOK::CLOSE_BRACKET_NEG
                }
                if self.char_matches(buf.as_ptr(), ASCII_RSQB) {
                    REQUIRE_CHARS!(buf, 2, self);
                    if self.char_matches(buf.inc_start(self.MINBPC()).as_ptr(), ASCII_GT) {
                        *nextTokPtr = buf.inc_start(2 * self.MINBPC()).as_ptr();
                        return XML_TOK::COND_SECT_CLOSE
                    }
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::CLOSE_BRACKET
            }
            ByteType::LPAR => {
                *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                return XML_TOK::OPEN_PAREN
            }
            ByteType::RPAR => {
                buf = buf.inc_start(self.MINBPC() as isize);
                if !HAS_CHAR!(buf, self) {
                    return XML_TOK::CLOSE_PAREN_NEG
                }
                match self.byte_type(buf.as_ptr()) {
                    ByteType::AST => {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::CLOSE_PAREN_ASTERISK
                    }
                    ByteType::QUEST => {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::CLOSE_PAREN_QUESTION
                    }
                    ByteType::PLUS => {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::CLOSE_PAREN_PLUS
                    }
                    ByteType::CR | ByteType::LF | ByteType::S | ByteType::GT | ByteType::COMMA | ByteType::VERBAR | ByteType::RPAR => {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::CLOSE_PAREN
                    }
                    _ => { }
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::INVALID
            }
            ByteType::VERBAR => {
                *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                return XML_TOK::OR
            }
            ByteType::GT => {
                *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                return XML_TOK::DECL_CLOSE
            }
            ByteType::NUM => {
                return self.scanPoundName(buf.inc_start(self.MINBPC() as isize),
                                              nextTokPtr)
            }
            ByteType::NMSTRT | ByteType::HEX => {
                tok = XML_TOK::NAME;
                buf = buf.inc_start(self.MINBPC() as isize);
            }
            ByteType::DIGIT | ByteType::NAME | ByteType::MINUS | ByteType::COLON => {
                tok = XML_TOK::NMTOKEN;
                buf = buf.inc_start(self.MINBPC() as isize);
            }
            ByteType::NONASCII => {
                if self.is_nmstrt_char_minbpc(buf.as_ptr()) {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    tok = XML_TOK::NAME;
                } else if self.is_name_char_minbpc(buf.as_ptr()) {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    tok = XML_TOK::NMTOKEN;
                } else {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::INVALID
                }
            }
            _ => {
                *nextTokPtr = buf.as_ptr();
                return XML_TOK::INVALID
            }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                ByteType::GT | ByteType::RPAR | ByteType::COMMA | ByteType::VERBAR | ByteType::LSQB | ByteType::PERCNT | ByteType::S | ByteType::CR | ByteType::LF => {
                    *nextTokPtr = buf.as_ptr();
                    return tok
                }
                ByteType::COLON => {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    match tok {
                        XML_TOK::NAME => {
                            REQUIRE_CHAR!(buf, self);
                            tok = XML_TOK::PREFIXED_NAME;
                            CHECK_NAME_CASES! {
                                (buf, nextTokPtr, self),
                                match self.byte_type(buf.as_ptr()),
                                _ => {
                                    tok = XML_TOK::NMTOKEN;
                                }
                            }
                        }
                        XML_TOK::PREFIXED_NAME => { tok = XML_TOK::NMTOKEN }
                        _ => { }
                    }
                }
                ByteType::PLUS => {
                    if tok == XML_TOK::NMTOKEN {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::INVALID
                    }
                    *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                    return XML_TOK::NAME_PLUS
                }
                ByteType::AST => {
                    if tok == XML_TOK::NMTOKEN {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::INVALID
                    }
                    *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                    return XML_TOK::NAME_ASTERISK
                }
                ByteType::QUEST => {
                    if tok == XML_TOK::NMTOKEN {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::INVALID
                    }
                    *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                    return XML_TOK::NAME_QUESTION
                }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK::INVALID }
            }
        }
        tok.negate()
    }
    fn attributeValueTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        let mut start: *const libc::c_char = 0 as *const libc::c_char;
        if buf.is_empty() {
            return XML_TOK::NONE;
        } else {
            if !HAS_CHAR!(buf, self) {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                return XML_TOK::PARTIAL;
            }
        }
        start = buf.as_ptr();
        while HAS_CHAR!(buf, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(buf.as_ptr()),
                LEAD_CASE(n) => {
                    buf = buf.inc_start(n as isize);
                }
                ByteType::AMP => {
                    if buf.as_ptr() == start {
                        return self.scanRef(buf.inc_start(self.MINBPC() as isize),
                                                nextTokPtr)
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::DATA_CHARS
                }
                ByteType::LT => {
                    /* this is for inside entity references */
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::INVALID;
                }
                ByteType::LF => {
                    if buf.as_ptr() == start {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::DATA_NEWLINE
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::DATA_CHARS
                }
                ByteType::CR => {
                    if buf.as_ptr() == start {
                        buf = buf.inc_start(self.MINBPC() as isize);
                        if !HAS_CHAR!(buf, self) {
                            return XML_TOK::TRAILING_CR
                        }
                        if self.byte_type(buf.as_ptr()) == ByteType::LF {
                            buf = buf.inc_start(self.MINBPC() as isize)
                        }
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::DATA_NEWLINE
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::DATA_CHARS
                }
                ByteType::S => {
                    if buf.as_ptr() == start {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::ATTRIBUTE_VALUE_S
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::DATA_CHARS
                }
                _ => { buf = buf.inc_start(self.MINBPC() as isize) }
            }
        }
        *nextTokPtr = buf.as_ptr();
        XML_TOK::DATA_CHARS
    }
    fn entityValueTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        let mut start: *const libc::c_char = 0 as *const libc::c_char;
        if buf.is_empty() {
            return XML_TOK::NONE;
        } else {
            if !HAS_CHAR!(buf, self) {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                return XML_TOK::PARTIAL;
            }
        }
        start = buf.as_ptr();
        while HAS_CHAR!(buf, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(buf.as_ptr()),
                LEAD_CASE(n) => {
                    buf = buf.inc_start(n as isize);
                }
                ByteType::AMP => {
                    if buf.as_ptr() == start {
                        return self.scanRef(buf.inc_start(self.MINBPC() as isize),
                                                nextTokPtr)
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::DATA_CHARS
                }
                ByteType::PERCNT => {
                    if buf.as_ptr() == start {
                        let tok =
                            self.scanPercent(buf.inc_start(self.MINBPC() as isize),
                                                 nextTokPtr);
                        return if tok == XML_TOK::PERCENT {
                                   XML_TOK::INVALID
                               } else { tok }
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::DATA_CHARS
                }
                ByteType::LF => {
                    if buf.as_ptr() == start {
                        *nextTokPtr = buf.inc_start(self.MINBPC()).as_ptr();
                        return XML_TOK::DATA_NEWLINE
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::DATA_CHARS
                }
                ByteType::CR => {
                    if buf.as_ptr() == start {
                        buf = buf.inc_start(self.MINBPC() as isize);
                        if !HAS_CHAR!(buf, self) {
                            return XML_TOK::TRAILING_CR
                        }
                        if self.byte_type(buf.as_ptr()) == ByteType::LF {
                            buf = buf.inc_start(self.MINBPC() as isize)
                        }
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK::DATA_NEWLINE
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK::DATA_CHARS
                }
                _ => { buf = buf.inc_start(self.MINBPC() as isize) }
            }
        }
        *nextTokPtr = buf.as_ptr();
        XML_TOK::DATA_CHARS
    }
    fn ignoreSectionTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: &mut *const libc::c_char,
    ) -> XML_TOK {
        let mut level: libc::c_int = 0 as libc::c_int;
        if self.MINBPC() > 1 {
            let mut n: size_t = buf.len() as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                buf = buf.with_len(n as usize)
            }
        }
        while HAS_CHAR!(buf, self) {
            MATCH_INVALID_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                ByteType::LT => {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    REQUIRE_CHAR!(buf, self);
                    if self.char_matches(buf.as_ptr(), ASCII_EXCL) {
                        buf = buf.inc_start(self.MINBPC() as isize);
                        REQUIRE_CHAR!(buf, self);
                        if self.char_matches(buf.as_ptr(), ASCII_LSQB) {
                            level += 1;
                            buf = buf.inc_start(self.MINBPC() as isize)
                        }
                    }
                }
                ByteType::RSQB => {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    REQUIRE_CHAR!(buf, self);
                    if self.char_matches(buf.as_ptr(), ASCII_RSQB) {
                        buf = buf.inc_start(self.MINBPC() as isize);
                        REQUIRE_CHAR!(buf, self);
                        if self.char_matches(buf.as_ptr(), ASCII_GT) {
                            buf = buf.inc_start(self.MINBPC() as isize);
                            if level == 0 as libc::c_int {
                                *nextTokPtr = buf.as_ptr();
                                return XML_TOK::IGNORE_SECT
                            }
                            level -= 1
                        }
                    }
                }
                _ => { buf = buf.inc_start(self.MINBPC() as isize) }
            }
        }
        XML_TOK::PARTIAL
    }
    fn isPublicId(
        &self,
        mut buf: ExpatBufRef,
        mut badPtr: &Cell<*const libc::c_char>,
    ) -> libc::c_int {
        buf = buf.inc_start(self.MINBPC() as isize)
            .dec_end(self.MINBPC() as usize);
        while HAS_CHAR!(buf, self) {
            match self.byte_type(buf.as_ptr()) {
                ByteType::DIGIT | ByteType::HEX | ByteType::MINUS | ByteType::APOS | ByteType::LPAR | ByteType::RPAR | ByteType::PLUS | ByteType::COMMA | ByteType::SOL | ByteType::EQUALS | ByteType::QUEST | ByteType::CR | ByteType::LF | ByteType::SEMI | ByteType::EXCL | ByteType::AST
                | ByteType::PERCNT | ByteType::NUM | ByteType::COLON => { }
                ByteType::S => {
                    if self.char_matches(buf.as_ptr(), ASCII_TAB) {
                        badPtr.set(buf.as_ptr());
                        return 0 as libc::c_int;
                    }
                }
                ByteType::NAME | ByteType::NMSTRT if self.byte_to_ascii(buf.as_ptr()) & !(0x7f as c_char) == 0 => { }
                _ => {
                    match self.byte_to_ascii(buf.as_ptr()) {
                        0x24 => {} /* $ */
                        0x40 => {} /* @ */
                        _ => {
                            badPtr.set(buf.as_ptr());
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
            buf = buf.inc_start(self.MINBPC() as isize)
        }
        1 as libc::c_int
    }

    /* This must only be called for a well-formed start-tag or empty
    element tag.  Returns the number of attributes.  Pointers to the
    first attsMax attributes are stored in atts.
     */
    fn getAtts(
        &self,
        mut buf: ExpatBufRef,
        f: &mut dyn FnMut(Attribute) -> XML_Error,
    ) -> XML_Error {
        #[derive(PartialEq)]
        enum State {
            Other,
            InName,
            InValue,
        };
        let mut state = State::InName;
        let mut att = Attribute {
            name: std::ptr::null(),
            valuePtr: std::ptr::null(),
            valueEnd: std::ptr::null(),
            normalized: false,
        };

        /* defined when state == inValue;
        initialization just to shut up compilers */
        let mut open: ByteType = ByteType::NONXML;

        macro_rules! START_NAME {
            () => {
                if state == State::Other {
                    att.name = buf.as_ptr();
                    att.normalized = true;
                    state = State::InName;
                }
            };
        }

        buf = buf.inc_start(self.MINBPC() as isize);
        loop {
            MATCH_LEAD_CASES! {
                   self.byte_type(buf.as_ptr()),
                   LEAD_CASE(n) => {
                       START_NAME!{}
                       buf = buf.inc_start(n - self.MINBPC());
                   }
                   ByteType::NONASCII | ByteType::NMSTRT | ByteType::HEX => {
                       START_NAME!{}
                   }
                   ByteType::QUOT => {
                       if state != State::InValue {
                           att.valuePtr = buf.inc_start(self.MINBPC()).as_ptr();
                           state = State::InValue;
                           open = ByteType::QUOT
                       } else if open == ByteType::QUOT {
                           state = State::Other;
                           att.valueEnd = buf.as_ptr();

                           let res = f(att);
                           if res != XML_Error::NONE {
                               return res;
                           }
                       }
                   }
                   ByteType::APOS => {
                       if state != State::InValue {
                           att.valuePtr = buf.inc_start(self.MINBPC()).as_ptr();
                           state = State::InValue;
                           open = ByteType::APOS
                       } else if open == ByteType::APOS {
                           state = State::Other;
                           att.valueEnd = buf.as_ptr();

                           let res = f(att);
                           if res != XML_Error::NONE {
                               return res;
                           }
                       }
                   }
                   ByteType::AMP => {
                       att.normalized = false;
                   }
                   ByteType::S => {
                       if state == State::InName {
                           state = State::Other;
                       } else if state == State::InValue
                           && att.normalized
                           && (buf.as_ptr() == att.valuePtr
                               || self.byte_to_ascii(buf.as_ptr()) != ASCII_SPACE
                               || self.byte_to_ascii(buf.inc_start(self.MINBPC()).as_ptr()) == ASCII_SPACE
                               || self.byte_type(buf.inc_start(self.MINBPC()).as_ptr()) == open)
                       {
                           att.normalized = false;
                       }
                   }
                   ByteType::CR | ByteType::LF => {
                       /* This case ensures that the first attribute name is counted
                       Apart from that we could just change state on the quote. */
                       if state == State::InName {
                           state = State::Other;
                       } else if state == State::InValue {
                           att.normalized = false;
                       }
                   }
                   ByteType::GT | ByteType::SOL => {
                       if state != State::InValue {
                           return XML_Error::NONE
                       }
                   }
                   _ => { }
               }
            buf = buf.inc_start(self.MINBPC() as isize)
        }
        /* not reached */
    }
    fn charRefNumber(&self, mut buf: ExpatBufRef) -> libc::c_int {
        let mut result: libc::c_int = 0 as libc::c_int;
        buf = buf.inc_start((2 * self.MINBPC()) as isize);
        if self.char_matches(buf.as_ptr(), ASCII_x) {
            buf = buf.inc_start(self.MINBPC() as isize);
            while !self.char_matches(buf.as_ptr(), ASCII_SEMI) {
                let mut c: c_char = self.byte_to_ascii(buf.as_ptr());
                match c {
                    ASCII_0 | ASCII_1 | ASCII_2 | ASCII_3 | ASCII_4 | ASCII_5 | ASCII_6
                    | ASCII_7 | ASCII_8 | ASCII_9 => {
                        result <<= 4 as libc::c_int;
                        result |= (c - ASCII_0) as c_int;
                    }
                    ASCII_A | ASCII_B | ASCII_C | ASCII_D | ASCII_E | ASCII_F => {
                        result <<= 4 as libc::c_int;
                        result += 10 as libc::c_int + (c - ASCII_A) as c_int
                    }
                    ASCII_a | ASCII_b | ASCII_c | ASCII_d | ASCII_e | ASCII_f => {
                        result <<= 4 as libc::c_int;
                        result += 10 as libc::c_int + (c - ASCII_a) as c_int
                    }
                    _ => {}
                }
                if result >= 0x110000 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                buf = buf.inc_start(self.MINBPC() as isize)
            }
        } else {
            while !self.char_matches(buf.as_ptr(), ASCII_SEMI) {
                let mut c_0: c_char = self.byte_to_ascii(buf.as_ptr());
                result *= 10 as libc::c_int;
                result += (c_0 - ASCII_0) as c_int;
                if result >= 0x110000 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                buf = buf.inc_start(self.MINBPC() as isize)
            }
        }
        checkCharRefNumber(result)
    }
    fn predefinedEntityName(
        &self,
        mut buf: ExpatBufRef,
    ) -> libc::c_int {
        match buf.len() / self.MINBPC() as usize {
            2 => {
                if self.char_matches(buf.inc_start(self.MINBPC()).as_ptr(), ASCII_t) {
                    match self.byte_to_ascii(buf.as_ptr()) {
                        ASCII_l => return ASCII_LT as c_int,
                        ASCII_g => return ASCII_GT as c_int,
                        _ => {}
                    }
                }
            }
            3 => {
                if self.char_matches(buf.as_ptr(), ASCII_a) {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    if self.char_matches(buf.as_ptr(), ASCII_m) {
                        buf = buf.inc_start(self.MINBPC() as isize);
                        if self.char_matches(buf.as_ptr(), ASCII_p) {
                            return ASCII_AMP as c_int;
                        }
                    }
                }
            }
            4 => match self.byte_to_ascii(buf.as_ptr()) {
                ASCII_q => {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    if self.char_matches(buf.as_ptr(), ASCII_u) {
                        buf = buf.inc_start(self.MINBPC() as isize);
                        if self.char_matches(buf.as_ptr(), ASCII_o) {
                            buf = buf.inc_start(self.MINBPC() as isize);
                            if self.char_matches(buf.as_ptr(), ASCII_t) {
                                return ASCII_QUOT as c_int;
                            }
                        }
                    }
                }
                ASCII_a => {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    if self.char_matches(buf.as_ptr(), ASCII_p) {
                        buf = buf.inc_start(self.MINBPC() as isize);
                        if self.char_matches(buf.as_ptr(), ASCII_o) {
                            buf = buf.inc_start(self.MINBPC() as isize);
                            if self.char_matches(buf.as_ptr(), ASCII_s) {
                                return ASCII_APOS as c_int;
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        return 0 as libc::c_int;
    }
    fn nameMatchesAscii(
        &self,
        mut buf: ExpatBufRef,
        mut ptr2: &[libc::c_char],
    ) -> bool {
        for ch in ptr2 {
            if buf.len() < self.MINBPC() as usize {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the
                 * paranoia check is still valuable, however.
                 */
                return false;
            }
            if !self.char_matches(buf.as_ptr(), *ch) {
                return false;
            }
            buf = buf.inc_start(self.MINBPC() as isize);
        }
        buf.is_empty()
    }
    unsafe fn nameLength(&self, mut ptr: *const libc::c_char) -> libc::c_int {
        let mut start: *const libc::c_char = ptr;
        loop {
            MATCH_LEAD_CASES! {
                self.byte_type(ptr),
                LEAD_CASE(n) => {
                    ptr = ptr.offset(n);
                }
                ByteType::NONASCII | ByteType::NMSTRT | ByteType::COLON | ByteType::HEX | ByteType::DIGIT | ByteType::NAME | ByteType::MINUS => {
                    ptr = ptr.offset(self.MINBPC())
                }
                _ => {
                    return ptr.wrapping_offset_from(start) as libc::c_long as
                               libc::c_int
                }
            }
        }
    }
    unsafe fn skipS(&self, mut ptr: *const libc::c_char) -> *const libc::c_char {
        loop {
            match self.byte_type(ptr) {
                ByteType::LF | ByteType::CR | ByteType::S => ptr = ptr.offset(self.MINBPC()),
                _ => return ptr,
            }
        }
    }
    fn updatePosition(
        &self,
        mut buf: ExpatBufRef,
        mut pos: &mut Position,
    ) {
        while HAS_CHAR!(buf, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(buf.as_ptr()),
                LEAD_CASE(n) => {
                    buf = buf.inc_start(n as isize);
                }
                ByteType::LF => {
                    (*pos).columnNumber = -(1 as libc::c_int) as XML_Size;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    buf = buf.inc_start(self.MINBPC() as isize)
                }
                ByteType::CR => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    buf = buf.inc_start(self.MINBPC() as isize);
                    if HAS_CHAR!(buf, self) &&
                           self.byte_type(buf.as_ptr()) == ByteType::LF {
                        buf = buf.inc_start(self.MINBPC() as isize)
                    }
                    (*pos).columnNumber = -(1 as libc::c_int) as XML_Size
                }
                _ => { buf = buf.inc_start(self.MINBPC() as isize) }
            }
            (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1)
        }
    }

    fn utf8Convert<'r, 'a: 'r, 'b: 'r>(
        &self,
        from_buf: &'r mut ExpatBufRef<'a>,
        to_buf: &'r mut ExpatBufRefMut<'b>,
    ) -> XML_Convert_Result {
        self.utf8Convert(from_buf, to_buf)
    }

    fn utf16Convert(
        &self,
        from_buf: &mut ExpatBufRef,
        to_buf: &mut ExpatBufRefMut<u16>,
    ) -> XML_Convert_Result {
        self.utf16Convert(from_buf, to_buf)
    }

    fn minBytesPerChar(&self) -> c_int {
        self.MINBPC() as c_int
    }

    fn isUtf8(&self) -> bool {
        self.isUtf8()
    }

    fn isUtf16(&self) -> bool {
        self.isUtf16()
    }
}
