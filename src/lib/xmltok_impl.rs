use libc::{c_char, c_int, c_long, size_t};

use super::xmltok::{checkCharRefNumber, ATTRIBUTE, POSITION};
use super::xmltok::{XML_Convert_Result, XmlEncoding, XmlEncodingImpl};
use crate::ascii_h::*;
pub use crate::expat_external_h::XML_Size;
use crate::xmltok_h::*;
use crate::xmltok_impl_h::*;
use crate::lib::xmlparse::{ExpatBufRef, ExpatBufRefMut};

pub const other: C2RustUnnamed_2 = 0;
pub const other_0: C2RustUnnamed_2 = 0;
pub const other_1: C2RustUnnamed_2 = 0;
pub const inName: C2RustUnnamed_2 = 1;
pub const inName_0: C2RustUnnamed_2 = 1;
pub const inName_1: C2RustUnnamed_2 = 1;
pub const inValue: C2RustUnnamed_2 = 2;
pub const inValue_0: C2RustUnnamed_2 = 2;
pub const inValue_1: C2RustUnnamed_2 = 2;

macro_rules! MATCH_LEAD_CASES {
    {$e:expr, LEAD_CASE($n:ident) => $case:block $($tail:tt)*} => {
        match $e {
            BT_LEAD2 => {
                let $n: isize = 2;
                $case
            }
            BT_LEAD3 => {
                let $n: isize = 3;
                $case
            }
            BT_LEAD4 => {
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
                    return XML_TOK_PARTIAL_CHAR;
                }
                if $self.is_invalid_char($buf.as_ptr(), n) {
                    *$nextTokPtr = $buf.as_ptr();
                    return XML_TOK_INVALID;
                }
                $buf = $buf.inc_start(n);
            }
            BT_NONXML | BT_MALFORM | BT_TRAIL => {
                *$nextTokPtr = $buf.as_ptr();
                return XML_TOK_INVALID;
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
                    return XML_TOK_PARTIAL_CHAR;
                }
                if !$self.is_name_char($buf.as_ptr(), n) {
                    *$nextTokPtr = $buf.as_ptr();
                    return XML_TOK_INVALID;
                }
                $buf = $buf.inc_start(n as isize);
            }
            BT_NONASCII if !$self.is_name_char_minbpc($buf.as_ptr()) => {
                *$nextTokPtr = $buf.as_ptr();
                return XML_TOK_INVALID;
            }
            BT_NONASCII | BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
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
                    return XML_TOK_PARTIAL_CHAR;
                }
                if !$self.is_nmstrt_char($buf.as_ptr(), n) {
                    *$nextTokPtr = $buf.as_ptr();
                    return XML_TOK_INVALID;
                }
                $buf = $buf.inc_start(n as isize);
            }
            BT_NONASCII if !$self.is_nmstrt_char_minbpc($buf.as_ptr()) => {
                *$nextTokPtr = $buf.as_ptr();
                return XML_TOK_INVALID;
            }
            BT_NONASCII | BT_NMSTRT | BT_HEX => {
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
            return XML_TOK_PARTIAL;
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
    unsafe extern "C" fn scanComment(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if HAS_CHAR!(buf, self) {
            if !self.char_matches(buf.as_ptr(), ASCII_MINUS) {
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_INVALID;
            }
            buf = buf.inc_start((self.MINBPC()) as isize);
            while HAS_CHAR!(buf, self) {
                MATCH_INVALID_CASES! {
                    (buf, nextTokPtr, self),
                    match self.byte_type(buf.as_ptr()),
                    BT_MINUS => {
                        buf = buf.inc_start((self.MINBPC()) as isize);
                        REQUIRE_CHAR!(buf, self);
                        if self.char_matches(buf.as_ptr(), ASCII_MINUS) {
                            buf = buf.inc_start((self.MINBPC()) as isize);
                            REQUIRE_CHAR!(buf, self);
                            if !self.char_matches(buf.as_ptr(), ASCII_GT) {
                                *nextTokPtr = buf.as_ptr();
                                return XML_TOK_INVALID
                            }
                            *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                            return XML_TOK_COMMENT
                        }
                    }
                    _ => { buf = buf.inc_start((self.MINBPC()) as isize); }
                }
            }
        }
        return XML_TOK_PARTIAL;
    }

    /* ptr points to character following "<!" */
    unsafe extern "C" fn scanDecl(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        REQUIRE_CHAR!(buf, self);
        match self.byte_type(buf.as_ptr()) {
            BT_MINUS => return self.scanComment(buf.inc_start(self.MINBPC() as isize), nextTokPtr),
            BT_LSQB => {
                *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                return XML_TOK_COND_SECT_OPEN;
            }
            BT_NMSTRT | BT_HEX => buf = buf.inc_start((self.MINBPC()) as isize),
            _ => {
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_INVALID;
            }
        }
        while HAS_CHAR!(buf, self) {
            's_151: {
                match self.byte_type(buf.as_ptr()) {
                    BT_PERCNT => {
                        REQUIRE_CHARS!(buf, 2, self);
                        /* don't allow <!ENTITY% foo "whatever"> */
                        match self.byte_type(buf.as_ptr().offset(self.MINBPC())) {
                            BT_S | BT_CR | BT_LF | BT_PERCNT => {
                                *nextTokPtr = buf.as_ptr();
                                return XML_TOK_INVALID;
                            }
                            _ => {}
                        }
                    }
                    BT_S | BT_CR | BT_LF => {}
                    BT_NMSTRT | BT_HEX => {
                        buf = buf.inc_start((self.MINBPC()) as isize);
                        break 's_151;
                    }
                    _ => {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID;
                    }
                }
                /* fall through */
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_DECL_OPEN;
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn checkPiTarget(
        &self,
        mut buf: ExpatBufRef,
        mut tokPtr: *mut libc::c_int,
    ) -> libc::c_int {
        let mut upper: libc::c_int = 0 as libc::c_int;
        *tokPtr = XML_TOK_PI;
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
        *tokPtr = XML_TOK_XML_DECL;
        return 1 as libc::c_int;
    }

    /* ptr points to character following "<?" */
    unsafe extern "C" fn scanPi(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut tok: libc::c_int = 0;
        let target = buf.clone();
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                BT_S | BT_CR | BT_LF => {
                    if self.checkPiTarget(target.with_end(buf.as_ptr()), &mut tok) == 0
                       {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID
                    }
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    while HAS_CHAR!(buf, self) {
                        MATCH_INVALID_CASES! {
                            (buf, nextTokPtr, self),
                            match self.byte_type(buf.as_ptr()),
                            BT_QUEST => {
                                buf = buf.inc_start((self.MINBPC()) as isize);
                                REQUIRE_CHAR!(buf, self);
                                if self.char_matches(buf.as_ptr(), ASCII_GT) {
                                    *nextTokPtr =
                                        buf.as_ptr().offset(self.MINBPC());
                                    return tok
                                }
                            }
                            _ => { buf = buf.inc_start((self.MINBPC()) as isize) }
                        }
                    }
                    return XML_TOK_PARTIAL
                }
                BT_QUEST => {
                    if self.checkPiTarget(target.with_end(buf.as_ptr()), &mut tok) == 0
                       {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID
                    }
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    if self.char_matches(buf.as_ptr(), ASCII_GT) {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return tok
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_INVALID
                }
                _ => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_INVALID
                }
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanCdataSection(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        pub static mut CDATA_LSQB: [libc::c_char; 6] = [
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
                return XML_TOK_INVALID;
            }
            i += 1;
            buf = buf.inc_start((self.MINBPC()) as isize)
        }
        *nextTokPtr = buf.as_ptr();
        return XML_TOK_CDATA_SECT_OPEN;
    }

    /* ptr points to character following "</" */
    unsafe extern "C" fn scanEndTag(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                BT_S | BT_CR | BT_LF => {
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    while HAS_CHAR!(buf, self) {
                        match self.byte_type(buf.as_ptr()) {
                            BT_S | BT_CR | BT_LF => { }
                            BT_GT => {
                                *nextTokPtr =
                                    buf.as_ptr().offset(self.MINBPC());
                                return XML_TOK_END_TAG
                            }
                            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
                        }
                        buf = buf.inc_start((self.MINBPC()) as isize)
                    }
                    return XML_TOK_PARTIAL
                }
                BT_COLON => {
                    /* no need to check qname syntax here,
                    since end-tag must match exactly */
                    buf = buf.inc_start((self.MINBPC()) as isize);
                }
                BT_GT => {
                    *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                    return XML_TOK_END_TAG
                }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
            }
        }
        return XML_TOK_PARTIAL;
    }

    /* ptr points to character following "&#X" */
    unsafe extern "C" fn scanHexCharRef(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if HAS_CHAR!(buf, self) {
            match self.byte_type(buf.as_ptr()) {
                BT_DIGIT | BT_HEX => {}
                _ => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_INVALID;
                }
            }
            buf = buf.inc_start((self.MINBPC()) as isize);
            while HAS_CHAR!(buf, self) {
                match self.byte_type(buf.as_ptr()) {
                    BT_DIGIT | BT_HEX => {}
                    BT_SEMI => {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID;
                    }
                }
                buf = buf.inc_start((self.MINBPC()) as isize)
            }
        }
        return XML_TOK_PARTIAL;
    }

    /* ptr points to character following "&#" */
    unsafe extern "C" fn scanCharRef(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if HAS_CHAR!(buf, self) {
            if self.char_matches(buf.as_ptr(), ASCII_x) {
                return self.scanHexCharRef(buf.inc_start(self.MINBPC() as isize), nextTokPtr);
            }
            match self.byte_type(buf.as_ptr()) {
                BT_DIGIT => {}
                _ => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_INVALID;
                }
            }
            buf = buf.inc_start((self.MINBPC()) as isize);
            while HAS_CHAR!(buf, self) {
                match self.byte_type(buf.as_ptr()) {
                    BT_DIGIT => {}
                    BT_SEMI => {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID;
                    }
                }
                buf = buf.inc_start((self.MINBPC()) as isize)
            }
        }
        return XML_TOK_PARTIAL;
    }

    /* ptr points to character following "&" */
    unsafe extern "C" fn scanRef(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            BT_NUM => {
                return self.scanCharRef(
                    buf.inc_start((self.MINBPC()) as isize),
                    nextTokPtr,
                )
            }
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                BT_SEMI => {
                    *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                    return XML_TOK_ENTITY_REF
                }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
            }
        }
        return XML_TOK_PARTIAL;
    }

    /* ptr points to character following first character of attribute name */
    unsafe extern "C" fn scanAtts(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
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
                BT_COLON => {
                    if hadColon != 0 {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID
                    }
                    hadColon = 1;
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    CHECK_NMSTRT_CASES! {
                        (buf, nextTokPtr, self),
                        match self.byte_type(buf.as_ptr()),
                        _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
                    }
                }
                BT_S | BT_CR | BT_LF => {
                    loop  {
                        let mut t: C2RustUnnamed_2 = 0;
                        buf = buf.inc_start((self.MINBPC()) as isize);
                        REQUIRE_CHAR!(buf, self);
                        t = self.byte_type(buf.as_ptr());
                        if t == BT_EQUALS { break ; }
                        match t {
                            BT_S | BT_LF | BT_CR => { }
                            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
                        }
                    }
                    current_block = Label::EqFallthrough;
                }
                BT_EQUALS => { current_block = Label::EqFallthrough; }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
            }
            if current_block == Label::EqFallthrough {
                /* fall through */
                // BT_S | BT_CR | BT_LF | BT_EQUALS =>
                let mut open: C2RustUnnamed_2 = 0;
                hadColon = 0;
                loop {
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    open = self.byte_type(buf.as_ptr());
                    if open == BT_QUOT || open == BT_APOS {
                        break;
                    }
                    match open {
                        BT_S | BT_LF | BT_CR => {}
                        _ => {
                            *nextTokPtr = buf.as_ptr();
                            return XML_TOK_INVALID;
                        }
                    }
                }
                buf = buf.inc_start((self.MINBPC()) as isize);
                /* in attribute value */
                loop {
                    let mut t: C2RustUnnamed_2 = 0;
                    REQUIRE_CHAR!(buf, self);
                    t = self.byte_type(buf.as_ptr());
                    if t == open {
                        break;
                    }
                    MATCH_INVALID_CASES! {
                        (buf, nextTokPtr, self),
                        match t,
                        BT_AMP => {
                            let mut ptr = buf.as_ptr();
                            let mut tok: libc::c_int =
                                self.scanRef(buf.inc_start(self.MINBPC() as isize), &mut ptr);
                            buf = buf.with_start(ptr);
                            if tok <= 0 as libc::c_int {
                                if tok == XML_TOK_INVALID {
                                    *nextTokPtr = buf.as_ptr()
                                }
                                return tok
                            }
                        }
                        BT_LT => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
                        _ => { buf = buf.inc_start((self.MINBPC()) as isize) }
                    }
                }
                buf = buf.inc_start((self.MINBPC()) as isize);
                REQUIRE_CHAR!(buf, self);
                match self.byte_type(buf.as_ptr()) {
                    BT_S | BT_CR | BT_LF => { }
                    BT_SOL => {
                        // goto sol;
                        current_block = Label::Sol;
                    }
                    BT_GT => {
                        // goto gt;
                        current_block = Label::Gt;
                    }
                    _ => {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID;
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
                                BT_S | BT_CR | BT_LF => {
                                    continue;
                                }
                                BT_GT => {
                                    current_block = Label::Gt;
                                    break ;
                                }
                                BT_SOL => {
                                    current_block = Label::Sol;
                                    break ;
                                }
                                _ => {
                                    *nextTokPtr = buf.as_ptr();
                                    return XML_TOK_INVALID
                                }
                            }
                            break;
                        }
                    }
                }

                match current_block {
                    Label::Gt => {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_START_TAG_WITH_ATTS;
                    }
                    Label::Sol => {
                        buf = buf.inc_start((self.MINBPC()) as isize);
                        REQUIRE_CHAR!(buf, self);
                        if !self.char_matches(buf.as_ptr(), ASCII_GT) {
                            *nextTokPtr = buf.as_ptr();
                            return XML_TOK_INVALID;
                        }
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                    }
                    _ => { }
                }
            }
        }
        return XML_TOK_PARTIAL;
    }

    /* ptr points to character following "<" */
    unsafe extern "C" fn scanLt(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut hadColon: libc::c_int = 0;
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            BT_EXCL => {
                buf = buf.inc_start((self.MINBPC()) as isize);
                REQUIRE_CHAR!(buf, self);
                match self.byte_type(buf.as_ptr()) {
                    BT_MINUS => {
                        return self.scanComment(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
                    }
                    BT_LSQB => {
                        return self.scanCdataSection(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
                    }
                    _ => { }
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_INVALID
            }
            BT_QUEST => {
                return self.scanPi(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            BT_SOL => {
                return self.scanEndTag(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
        }
        hadColon = 0 as libc::c_int;
        /* we have a start-tag */
        /* we have a start-tag */
        /* we have a start-tag */
        while HAS_CHAR!(buf, self) {
            let mut current_block_161: u64 = 0;
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                BT_COLON => {
                    if hadColon != 0 {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID
                    }
                    hadColon = 1 as libc::c_int;
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    CHECK_NMSTRT_CASES! {
                        (buf, nextTokPtr, self),
                        match self.byte_type(buf.as_ptr()),
                        _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
                    }
                    current_block_161 = 12655303178690906525;
                }
                BT_S | BT_CR | BT_LF => {
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    loop  {
                        if !(HAS_CHAR!(buf, self)) {
                            current_block_161 = 13000670339742628194;
                            break ;
                        }
                        CHECK_NMSTRT_CASES! {
                            (buf, nextTokPtr, self),
                            match self.byte_type(buf.as_ptr()),
                            BT_GT => {
                                current_block_161 = 15370445274224965566;
                                break ;
                            }
                            BT_SOL => {
                                current_block_161 = 3926109038817298867;
                                break ;
                            }
                            BT_S | BT_CR | BT_LF => {
                                buf = buf.inc_start((self.MINBPC()) as isize);
                                continue ;
                            }
                            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
                        }
                        return self.scanAtts(buf, nextTokPtr)
                    }
                    match current_block_161 {
                        15370445274224965566 => { }
                        3926109038817298867 => { }
                        _ => { return XML_TOK_PARTIAL }
                    }
                }
                BT_GT => { current_block_161 = 15370445274224965566; }
                BT_SOL => { current_block_161 = 3926109038817298867; }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
            }
            match current_block_161 {
                3926109038817298867 => {
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    if !self.char_matches(buf.as_ptr(), ASCII_GT) {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                }
                15370445274224965566 => {
                    *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                    return XML_TOK_START_TAG_NO_ATTS;
                }
                16331546839105579257 => buf = buf.inc_start((self.MINBPC()) as isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    /* ptr points to character following "%" */
    unsafe extern "C" fn scanPercent(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            BT_S | BT_LF | BT_CR | BT_PERCNT => { *nextTokPtr = buf.as_ptr(); return XML_TOK_PERCENT }
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                BT_SEMI => {
                    *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                    return XML_TOK_PARAM_ENTITY_REF
                }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanPoundName(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        REQUIRE_CHAR!(buf, self);
        CHECK_NMSTRT_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                BT_CR | BT_LF | BT_S | BT_RPAR | BT_GT | BT_PERCNT | BT_VERBAR => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_POUND_NAME
                }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
            }
        }
        return -XML_TOK_POUND_NAME;
    }
    unsafe extern "C" fn scanLit(
        &self,
        mut open: C2RustUnnamed_2,

        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        while HAS_CHAR!(buf, self) {
            let mut t: C2RustUnnamed_2 = self.byte_type(buf.as_ptr());
            MATCH_INVALID_CASES! {
                (buf, nextTokPtr, self),
                match t,
                BT_QUOT | BT_APOS => {
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    if t == open {
                        if !HAS_CHAR!(buf, self) {
                            return -XML_TOK_LITERAL
                        }
                        *nextTokPtr = buf.as_ptr();
                        match self.byte_type(buf.as_ptr()) {
                            BT_S | BT_CR | BT_LF | BT_GT | BT_PERCNT | BT_LSQB => {
                                return XML_TOK_LITERAL
                            }
                            _ => { return XML_TOK_INVALID }
                        }
                    }
                }
                _ => { buf = buf.inc_start((self.MINBPC()) as isize) }
            }
        }
        return XML_TOK_PARTIAL;
    }
}

impl<T: XmlEncodingImpl> XmlTokImpl for T { }

impl<T: XmlEncodingImpl+XmlTokImpl> XmlEncoding for T {
    unsafe extern "C" fn cdataSectionTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if buf.is_empty() {
            return XML_TOK_NONE;
        }
        if self.MINBPC() > 1 {
            let mut n: size_t = buf.len();
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK_PARTIAL;
                }
                buf = buf.with_len(n);
            }
        }
        MATCH_INVALID_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            BT_RSQB => {
                buf = buf.inc_start((self.MINBPC()) as isize);
                REQUIRE_CHAR!(buf, self);
                if self.char_matches(buf.as_ptr(), ASCII_RSQB) {
                    let prev_buf = buf.clone();
                    buf = buf.inc_start((self.MINBPC()) as isize);
                    REQUIRE_CHAR!(buf, self);
                    if !self.char_matches(buf.as_ptr(), ASCII_GT) {
                        buf = prev_buf;
                    } else {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_CDATA_SECT_CLOSE
                    }
                }
            }
            BT_CR => {
                buf = buf.inc_start((self.MINBPC()) as isize);
                REQUIRE_CHAR!(buf, self);
                if self.byte_type(buf.as_ptr()) == BT_LF {
                    buf = buf.inc_start((self.MINBPC()) as isize)
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_DATA_NEWLINE
            }
            BT_LF => {
                *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                return XML_TOK_DATA_NEWLINE
            }
            _ => { buf = buf.inc_start((self.MINBPC()) as isize) }
        }
        while HAS_CHAR!(buf, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(buf.as_ptr()),
                LEAD_CASE(n) => {
                    if buf.len() < n as usize || self.is_invalid_char(buf.as_ptr(), n) {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_DATA_CHARS;
                    }
                    buf = buf.inc_start(n as isize);
                }
                BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF | BT_RSQB => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_DATA_CHARS
                }
                _ => { buf = buf.inc_start((self.MINBPC()) as isize) }
            }
        }
        *nextTokPtr = buf.as_ptr();
        return XML_TOK_DATA_CHARS;
    }
    unsafe extern "C" fn contentTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if buf.is_empty() {
            return XML_TOK_NONE;
        }
        if self.MINBPC() > 1 {
            let mut n: size_t = buf.len() as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK_PARTIAL;
                }
                buf = buf.with_len(n as usize)
            }
        }
        MATCH_INVALID_CASES! {
            (buf, nextTokPtr, self),
            match self.byte_type(buf.as_ptr()),
            BT_LT => {
                return self.scanLt(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            BT_AMP => {
                return self.scanRef(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            BT_CR => {
                buf = buf.inc_start(self.MINBPC() as isize);
                if !HAS_CHAR!(buf, self) {
                    return XML_TOK_TRAILING_CR
                }
                if self.byte_type(buf.as_ptr()) == BT_LF {
                    buf = buf.inc_start(self.MINBPC() as isize)
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_DATA_NEWLINE
            }
            BT_LF => {
                *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                return XML_TOK_DATA_NEWLINE
            }
            BT_RSQB => {
                buf = buf.inc_start(self.MINBPC() as isize);
                if !HAS_CHAR!(buf, self) {
                    return XML_TOK_TRAILING_RSQB
                }
                if self.char_matches(buf.as_ptr(), ASCII_RSQB) {
                    let prev_buf = buf.clone();
                    buf = buf.inc_start(self.MINBPC() as isize);
                    if !HAS_CHAR!(buf, self) {
                        return XML_TOK_TRAILING_RSQB
                    }
                    if !self.char_matches(buf.as_ptr(), ASCII_GT) {
                        buf = prev_buf;
                    } else { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
                }
            }
            _ => { buf = buf.inc_start(self.MINBPC() as isize) }
        }
        while HAS_CHAR!(buf, self) {
            let mut current_block_76: u64;
            MATCH_LEAD_CASES! {
                self.byte_type(buf.as_ptr()),
                LEAD_CASE(n) => {
                    if buf.len() < n as usize || self.is_invalid_char(buf.as_ptr(), n) {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_DATA_CHARS;
                    }
                    buf = buf.inc_start(n as isize);
                    current_block_76 = 10213293998891106930;
                }
                BT_RSQB => {
                    if HAS_CHARS!(buf, 2, self) {
                        if !self.char_matches(buf.as_ptr().offset(self.MINBPC()), ASCII_RSQB) {
                            buf = buf.inc_start(self.MINBPC() as isize);
                            current_block_76 = 10213293998891106930;
                        } else if HAS_CHARS!(buf, 3, self) {
                            if !self.char_matches(buf.as_ptr().offset(2 * self.MINBPC()), ASCII_GT) {
                                buf = buf.inc_start(self.MINBPC() as isize)
                            } else {
                                *nextTokPtr =
                                    buf.as_ptr().offset((2 * self.MINBPC()) as isize);
                                return XML_TOK_INVALID
                            }
                            current_block_76 = 10213293998891106930;
                        } else { current_block_76 = 4244197895050895038; }
                    } else { current_block_76 = 4244197895050895038; }
                }
                BT_AMP | BT_LT | BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF => {
                    current_block_76 = 4244197895050895038;
                }
                _ => {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    current_block_76 = 10213293998891106930;
                }
            }
            match current_block_76 {
                10213293998891106930 => {}
                _ => {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_DATA_CHARS;
                }
            }
        }
        *nextTokPtr = buf.as_ptr();
        return XML_TOK_DATA_CHARS;
    }
    unsafe extern "C" fn prologTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut tok: libc::c_int = 0;
        if buf.is_empty() {
            return XML_TOK_NONE;
        }
        if self.MINBPC() > 1 {
            let mut n: size_t = buf.len() as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK_PARTIAL;
                }
                buf = buf.with_len(n as usize)
            }
        }
        let mut current_block_112: u64;
        MATCH_LEAD_CASES! {
            self.byte_type(buf.as_ptr()),
            LEAD_CASE(n) => {
                if (buf.len() as c_long) < n as c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if self.is_nmstrt_char(buf.as_ptr(), n) {
                    buf = buf.inc_start(n as isize);
                    tok = XML_TOK_NAME;
                } else if self.is_name_char(buf.as_ptr(), n) {
                    buf = buf.inc_start(n as isize);
                    tok = XML_TOK_NMTOKEN
                } else {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_INVALID
                }
                current_block_112 = 2222055338596505704;
            }
            BT_QUOT => {
                return self.scanLit(BT_QUOT,
                                        buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            BT_APOS => {
                return self.scanLit(BT_APOS,
                                        buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            BT_LT => {
                buf = buf.inc_start(self.MINBPC() as isize);
                REQUIRE_CHAR!(buf, self);
                match self.byte_type(buf.as_ptr()) {
                    BT_EXCL => {
                        return self.scanDecl(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
                    }
                    BT_QUEST => {
                        return self.scanPi(buf.inc_start(self.MINBPC() as isize),
                                               nextTokPtr)
                    }
                    BT_NMSTRT | BT_HEX | BT_NONASCII | BT_LEAD2 | BT_LEAD3 | BT_LEAD4 => {
                        *nextTokPtr = buf.as_ptr().offset(-(self.MINBPC()));
                        return XML_TOK_INSTANCE_START
                    }
                    _ => { }
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_INVALID
            }
            BT_CR => {
                if buf.len() == self.MINBPC() as usize {
                    *nextTokPtr = buf.end();
                    /* indicate that this might be part of a CR/LF pair */
                    return -XML_TOK_PROLOG_S
                }
                current_block_112 = 1103933966285275534;
            }
            BT_S | BT_LF => { current_block_112 = 1103933966285275534; }
            BT_PERCNT => {
                return self.scanPercent(buf.inc_start(self.MINBPC() as isize), nextTokPtr)
            }
            BT_COMMA => {
                *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                return XML_TOK_COMMA
            }
            BT_LSQB => {
                *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                return XML_TOK_OPEN_BRACKET
            }
            BT_RSQB => {
                buf = buf.inc_start(self.MINBPC() as isize);
                if !HAS_CHAR!(buf, self) {
                    return -XML_TOK_CLOSE_BRACKET
                }
                if self.char_matches(buf.as_ptr(), ASCII_RSQB) {
                    REQUIRE_CHARS!(buf, 2, self);
                    if self.char_matches(buf.as_ptr().offset(self.MINBPC()), ASCII_GT) {
                        *nextTokPtr = buf.as_ptr().offset(2 * self.MINBPC());
                        return XML_TOK_COND_SECT_CLOSE
                    }
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_CLOSE_BRACKET
            }
            BT_LPAR => {
                *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                return XML_TOK_OPEN_PAREN
            }
            BT_RPAR => {
                buf = buf.inc_start(self.MINBPC() as isize);
                if !HAS_CHAR!(buf, self) {
                    return -XML_TOK_CLOSE_PAREN
                }
                match self.byte_type(buf.as_ptr()) {
                    BT_AST => {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_CLOSE_PAREN_ASTERISK
                    }
                    BT_QUEST => {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_CLOSE_PAREN_QUESTION
                    }
                    BT_PLUS => {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_CLOSE_PAREN_PLUS
                    }
                    BT_CR | BT_LF | BT_S | BT_GT | BT_COMMA | BT_VERBAR | BT_RPAR => {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_CLOSE_PAREN
                    }
                    _ => { }
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_INVALID
            }
            BT_VERBAR => {
                *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                return XML_TOK_OR
            }
            BT_GT => {
                *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                return XML_TOK_DECL_CLOSE
            }
            BT_NUM => {
                return self.scanPoundName(buf.inc_start(self.MINBPC() as isize),
                                              nextTokPtr)
            }
            BT_NMSTRT | BT_HEX => {
                tok = XML_TOK_NAME;
                buf = buf.inc_start(self.MINBPC() as isize);
                current_block_112 = 2222055338596505704;
            }
            BT_DIGIT | BT_NAME | BT_MINUS | BT_COLON => {
                tok = XML_TOK_NMTOKEN;
                buf = buf.inc_start(self.MINBPC() as isize);
                current_block_112 = 2222055338596505704;
            }
            BT_NONASCII => {
                if self.is_nmstrt_char_minbpc(buf.as_ptr()) {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    tok = XML_TOK_NAME;
                    current_block_112 = 2222055338596505704;
                } else if self.is_name_char_minbpc(buf.as_ptr()) {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    tok = XML_TOK_NMTOKEN;
                    current_block_112 = 2222055338596505704;
                } else {
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_INVALID
                }
            }
            _ => {
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_INVALID
            }
        }
        match current_block_112 {
            2222055338596505704 => {}
            _ =>
            /* fall through */
            {
                loop {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    if !HAS_CHAR!(buf, self) {
                        break;
                    }
                    let mut current_block_32: u64;
                    match self.byte_type(buf.as_ptr()) {
                        BT_S | BT_LF => {
                            current_block_32 = 14072441030219150333;
                        }
                        BT_CR => {
                            /* don't split CR/LF pair */
                            if buf.len() != self.MINBPC() as usize {
                                current_block_32 = 14072441030219150333;
                            } else {
                                current_block_32 = 11892121546066863882;
                            }
                        }
                        _ => {
                            current_block_32 = 11892121546066863882;
                        }
                    }
                    match current_block_32 {
                        14072441030219150333 => {}
                        _ =>
                        /* fall through */
                        {
                            *nextTokPtr = buf.as_ptr();
                            return XML_TOK_PROLOG_S;
                        }
                    }
                }
                *nextTokPtr = buf.as_ptr();
                return XML_TOK_PROLOG_S;
            }
        }
        while HAS_CHAR!(buf, self) {
            CHECK_NAME_CASES! {
                (buf, nextTokPtr, self),
                match self.byte_type(buf.as_ptr()),
                BT_GT | BT_RPAR | BT_COMMA | BT_VERBAR | BT_LSQB | BT_PERCNT | BT_S | BT_CR | BT_LF => {
                    *nextTokPtr = buf.as_ptr();
                    return tok
                }
                BT_COLON => {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    match tok {
                        XML_TOK_NAME => {
                            REQUIRE_CHAR!(buf, self);
                            tok = XML_TOK_PREFIXED_NAME;
                            CHECK_NAME_CASES! {
                                (buf, nextTokPtr, self),
                                match self.byte_type(buf.as_ptr()),
                                _ => {
                                    tok = XML_TOK_NMTOKEN;
                                }
                            }
                        }
                        XML_TOK_PREFIXED_NAME => { tok = XML_TOK_NMTOKEN }
                        _ => { }
                    }
                }
                BT_PLUS => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID
                    }
                    *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                    return XML_TOK_NAME_PLUS
                }
                BT_AST => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID
                    }
                    *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                    return XML_TOK_NAME_ASTERISK
                }
                BT_QUEST => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_INVALID
                    }
                    *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                    return XML_TOK_NAME_QUESTION
                }
                _ => { *nextTokPtr = buf.as_ptr(); return XML_TOK_INVALID }
            }
        }
        return -tok;
    }
    unsafe extern "C" fn attributeValueTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut start: *const libc::c_char = 0 as *const libc::c_char;
        if buf.is_empty() {
            return XML_TOK_NONE;
        } else {
            if !HAS_CHAR!(buf, self) {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                return XML_TOK_PARTIAL;
            }
        }
        start = buf.as_ptr();
        while HAS_CHAR!(buf, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(buf.as_ptr()),
                LEAD_CASE(n) => {
                    buf = buf.inc_start(n as isize);
                }
                BT_AMP => {
                    if buf.as_ptr() == start {
                        return self.scanRef(buf.inc_start(self.MINBPC() as isize),
                                                nextTokPtr)
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_DATA_CHARS
                }
                BT_LT => {
                    /* this is for inside entity references */
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_INVALID;
                }
                BT_LF => {
                    if buf.as_ptr() == start {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_DATA_CHARS
                }
                BT_CR => {
                    if buf.as_ptr() == start {
                        buf = buf.inc_start(self.MINBPC() as isize);
                        if !HAS_CHAR!(buf, self) {
                            return XML_TOK_TRAILING_CR
                        }
                        if self.byte_type(buf.as_ptr()) == BT_LF {
                            buf = buf.inc_start(self.MINBPC() as isize)
                        }
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_DATA_CHARS
                }
                BT_S => {
                    if buf.as_ptr() == start {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_ATTRIBUTE_VALUE_S
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_DATA_CHARS
                }
                _ => { buf = buf.inc_start(self.MINBPC() as isize) }
            }
        }
        *nextTokPtr = buf.as_ptr();
        return XML_TOK_DATA_CHARS;
    }
    unsafe extern "C" fn entityValueTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut start: *const libc::c_char = 0 as *const libc::c_char;
        if buf.is_empty() {
            return XML_TOK_NONE;
        } else {
            if !HAS_CHAR!(buf, self) {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                return XML_TOK_PARTIAL;
            }
        }
        start = buf.as_ptr();
        while HAS_CHAR!(buf, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(buf.as_ptr()),
                LEAD_CASE(n) => {
                    buf = buf.inc_start(n as isize);
                }
                BT_AMP => {
                    if buf.as_ptr() == start {
                        return self.scanRef(buf.inc_start(self.MINBPC() as isize),
                                                nextTokPtr)
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_DATA_CHARS
                }
                BT_PERCNT => {
                    if buf.as_ptr() == start {
                        let mut tok: libc::c_int =
                            self.scanPercent(buf.inc_start(self.MINBPC() as isize),
                                                 nextTokPtr);
                        return if tok == XML_TOK_PERCENT {
                                   XML_TOK_INVALID
                               } else { tok }
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_DATA_CHARS
                }
                BT_LF => {
                    if buf.as_ptr() == start {
                        *nextTokPtr = buf.as_ptr().offset(self.MINBPC());
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_DATA_CHARS
                }
                BT_CR => {
                    if buf.as_ptr() == start {
                        buf = buf.inc_start(self.MINBPC() as isize);
                        if !HAS_CHAR!(buf, self) {
                            return XML_TOK_TRAILING_CR
                        }
                        if self.byte_type(buf.as_ptr()) == BT_LF {
                            buf = buf.inc_start(self.MINBPC() as isize)
                        }
                        *nextTokPtr = buf.as_ptr();
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = buf.as_ptr();
                    return XML_TOK_DATA_CHARS
                }
                _ => { buf = buf.inc_start(self.MINBPC() as isize) }
            }
        }
        *nextTokPtr = buf.as_ptr();
        return XML_TOK_DATA_CHARS;
    }
    unsafe extern "C" fn ignoreSectionTok(
        &self,
        mut buf: ExpatBufRef,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
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
                BT_LT => {
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
                BT_RSQB => {
                    buf = buf.inc_start(self.MINBPC() as isize);
                    REQUIRE_CHAR!(buf, self);
                    if self.char_matches(buf.as_ptr(), ASCII_RSQB) {
                        buf = buf.inc_start(self.MINBPC() as isize);
                        REQUIRE_CHAR!(buf, self);
                        if self.char_matches(buf.as_ptr(), ASCII_GT) {
                            buf = buf.inc_start(self.MINBPC() as isize);
                            if level == 0 as libc::c_int {
                                *nextTokPtr = buf.as_ptr();
                                return XML_TOK_IGNORE_SECT
                            }
                            level -= 1
                        }
                    }
                }
                _ => { buf = buf.inc_start(self.MINBPC() as isize) }
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn isPublicId(
        &self,
        mut buf: ExpatBufRef,
        mut badPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        buf = buf.inc_start(self.MINBPC() as isize)
            .dec_end(self.MINBPC() as usize);
        while HAS_CHAR!(buf, self) {
            match self.byte_type(buf.as_ptr()) {
                BT_DIGIT | BT_HEX | BT_MINUS | BT_APOS | BT_LPAR | BT_RPAR | BT_PLUS | BT_COMMA | BT_SOL | BT_EQUALS | BT_QUEST | BT_CR | BT_LF | BT_SEMI | BT_EXCL | BT_AST
                | BT_PERCNT | BT_NUM | BT_COLON => { }
                BT_S => {
                    if self.char_matches(buf.as_ptr(), ASCII_TAB) {
                        *badPtr = buf.as_ptr();
                        return 0 as libc::c_int;
                    }
                }
                BT_NAME | BT_NMSTRT if self.byte_to_ascii(buf.as_ptr()) & !(0x7f as c_char) == 0 => { }
                _ => {
                    match self.byte_to_ascii(buf.as_ptr()) {
                        0x24 => {} /* $ */
                        0x40 => {} /* @ */
                        _ => {
                            *badPtr = buf.as_ptr();
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
            buf = buf.inc_start(self.MINBPC() as isize)
        }
        return 1 as libc::c_int;
    }

    /* This must only be called for a well-formed start-tag or empty
    element tag.  Returns the number of attributes.  Pointers to the
    first attsMax attributes are stored in atts.
     */
    unsafe extern "C" fn getAtts(
        &self,
        mut buf: ExpatBufRef,
        mut attsMax: libc::c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> libc::c_int {
        #[derive(PartialEq)]
        enum State {
            Other,
            InName,
            InValue,
        };
        let mut state = State::InName;
        let mut nAtts: libc::c_int = 0 as libc::c_int;

        /* defined when state == inValue;
        initialization just to shut up compilers */
        let mut open: C2RustUnnamed_2 = 0 as C2RustUnnamed_2; 

        macro_rules! START_NAME {
            () => {
                if state == State::Other {
                    if nAtts < attsMax {
                        let ref mut fresh120 = (*atts.offset(nAtts as isize)).name;
                        *fresh120 = buf.as_ptr();
                        (*atts.offset(nAtts as isize)).normalized = 1 as libc::c_int as libc::c_char
                    }
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
                   BT_NONASCII | BT_NMSTRT | BT_HEX => {
                       START_NAME!{}
                   }
                   BT_QUOT => {
                       if state != State::InValue {
                           if nAtts < attsMax {
                               let ref mut fresh127 =
                                   (*atts.offset(nAtts as isize)).valuePtr;
                               *fresh127 = buf.as_ptr().offset(self.MINBPC())
                           }
                           state = State::InValue;
                           open = BT_QUOT
                       } else if open == BT_QUOT {
                           state = State::Other;
                           if nAtts < attsMax {
                               let ref mut fresh128 =
                                   (*atts.offset(nAtts as isize)).valueEnd;
                               *fresh128 = buf.as_ptr()
                           }
                           nAtts += 1
                       }
                   }
                   BT_APOS => {
                       if state != State::InValue {
                           if nAtts < attsMax {
                               let ref mut fresh129 =
                                   (*atts.offset(nAtts as isize)).valuePtr;
                               *fresh129 = buf.as_ptr().offset(self.MINBPC())
                           }
                           state = State::InValue;
                           open = BT_APOS
                       } else if open == BT_APOS {
                           state = State::Other;
                           if nAtts < attsMax {
                               let ref mut fresh130 =
                                   (*atts.offset(nAtts as isize)).valueEnd;
                               *fresh130 = buf.as_ptr()
                           }
                           nAtts += 1
                       }
                   }
                   BT_AMP => {
                       if nAtts < attsMax {
                           (*atts.offset(nAtts as isize)).normalized =
                               0 as libc::c_int as libc::c_char
                       }
                   }
                   BT_S => {
                       if state == State::InName {
                           state = State::Other;
                       } else if state == State::InValue && nAtts < attsMax
                           && (*atts.offset(nAtts as isize)).normalized as libc::c_int != 0
                           && (buf.as_ptr() == (*atts.offset(nAtts as isize)).valuePtr
                               || self.byte_to_ascii(buf.as_ptr()) != ASCII_SPACE
                               || self.byte_to_ascii(buf.as_ptr().offset(self.MINBPC())) == ASCII_SPACE
                               || self.byte_type(buf.as_ptr().offset(self.MINBPC())) == open)
                       {
                           (*atts.offset(nAtts as isize)).normalized =
                               0 as libc::c_int as libc::c_char
                       }
                   }
                   BT_CR | BT_LF => {
                       /* This case ensures that the first attribute name is counted
                       Apart from that we could just change state on the quote. */
                       if state == State::InName {
                           state = State::Other;
                       } else if state == State::InValue && nAtts < attsMax {
                           (*atts.offset(nAtts as isize)).normalized =
                               0 as libc::c_int as libc::c_char
                       }
                   }
                   BT_GT | BT_SOL => {
                       if state != State::InValue {
                           return nAtts
                       }
                   }
                   _ => { }
               }
            buf = buf.inc_start(self.MINBPC() as isize)
        }
        /* not reached */
    }
    unsafe extern "C" fn charRefNumber(&self, mut buf: ExpatBufRef) -> libc::c_int {
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
        return checkCharRefNumber(result);
    }
    unsafe extern "C" fn predefinedEntityName(
        &self,
        mut buf: ExpatBufRef,
    ) -> libc::c_int {
        match buf.len() / self.MINBPC() as usize {
            2 => {
                if self.char_matches(buf.as_ptr().offset(self.MINBPC()), ASCII_t) {
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
    unsafe extern "C" fn nameMatchesAscii(
        &self,
        mut buf: ExpatBufRef,
        mut ptr2: *const libc::c_char,
    ) -> libc::c_int {
        while *ptr2 != 0 {
            if buf.len() < self.MINBPC() as usize {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the
                 * paranoia check is still valuable, however.
                 */
                return 0 as libc::c_int;
            }
            if !self.char_matches(buf.as_ptr(), *ptr2) {
                return 0 as libc::c_int;
            }
            buf = buf.inc_start(self.MINBPC() as isize);
            ptr2 = ptr2.offset(1)
        }
        return (buf.is_empty()) as libc::c_int;
    }
    unsafe extern "C" fn nameLength(&self, mut ptr: *const libc::c_char) -> libc::c_int {
        let mut start: *const libc::c_char = ptr;
        loop {
            MATCH_LEAD_CASES! {
                self.byte_type(ptr),
                LEAD_CASE(n) => {
                    ptr = ptr.offset(n);
                }
                BT_NONASCII | BT_NMSTRT | BT_COLON | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                    ptr = ptr.offset(self.MINBPC())
                }
                _ => {
                    return ptr.wrapping_offset_from(start) as libc::c_long as
                               libc::c_int
                }
            }
        }
    }
    unsafe extern "C" fn skipS(&self, mut ptr: *const libc::c_char) -> *const libc::c_char {
        loop {
            match self.byte_type(ptr) {
                BT_LF | BT_CR | BT_S => ptr = ptr.offset(self.MINBPC()),
                _ => return ptr,
            }
        }
    }
    unsafe extern "C" fn updatePosition(
        &self,
        mut buf: ExpatBufRef,
        mut pos: *mut POSITION,
    ) {
        while HAS_CHAR!(buf, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(buf.as_ptr()),
                LEAD_CASE(n) => {
                    buf = buf.inc_start(n as isize);
                }
                BT_LF => {
                    (*pos).columnNumber = -(1 as libc::c_int) as XML_Size;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    buf = buf.inc_start(self.MINBPC() as isize)
                }
                BT_CR => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    buf = buf.inc_start(self.MINBPC() as isize);
                    if HAS_CHAR!(buf, self) &&
                           self.byte_type(buf.as_ptr()) == BT_LF {
                        buf = buf.inc_start(self.MINBPC() as isize)
                    }
                    (*pos).columnNumber = -(1 as libc::c_int) as XML_Size
                }
                _ => { buf = buf.inc_start(self.MINBPC() as isize) }
            }
            (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1)
        }
    }

    unsafe extern "C" fn utf8Convert<'b, 'a: 'b>(
        &self,
        from_buf: &mut ExpatBufRef<'a>,
        to_buf: &'b mut ExpatBufRefMut<'a>,
    ) -> XML_Convert_Result {
        self.utf8Convert(from_buf, to_buf)
    }

    unsafe extern "C" fn utf16Convert(
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
