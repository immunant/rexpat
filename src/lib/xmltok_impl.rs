use libc::{c_char, c_int, c_long, c_ushort, size_t};

use super::xmltok::{checkCharRefNumber, ATTRIBUTE, POSITION};
use super::xmltok::{XML_Convert_Result, XmlEncoding, XmlEncodingImpl};
use crate::ascii_h::*;
pub use crate::expat_external_h::XML_Size;
use crate::xmltok_h::*;
use crate::xmltok_impl_h::*;

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
        ($ptr:ident, $end:ident, $nextTokPtr:ident, $self:ident),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $end.wrapping_offset_from($ptr) < n {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if $self.is_invalid_char($ptr, n) {
                    *$nextTokPtr = $ptr;
                    return XML_TOK_INVALID;
                }
                $ptr = $ptr.offset(n);
            }
            BT_NONXML | BT_MALFORM | BT_TRAIL => {
                *$nextTokPtr = $ptr;
                return XML_TOK_INVALID;
            }
            $($tail)*
        }
    }
}

macro_rules! CHECK_NAME_CASES {
    {
        ($ptr:ident, $end:ident, $nextTokPtr:ident, $self:ident),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $end.wrapping_offset_from($ptr) < n {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if !$self.is_name_char($ptr, n) {
                    *$nextTokPtr = $ptr;
                    return XML_TOK_INVALID;
                }
                $ptr = $ptr.offset(n);
            }
            BT_NONASCII if !$self.is_name_char_minbpc($ptr) => {
                *$nextTokPtr = $ptr;
                return XML_TOK_INVALID;
            }
            BT_NONASCII | BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                $ptr = $ptr.offset($self.MINBPC());
            }
            $($tail)*
        }
    }
}

macro_rules! CHECK_NMSTRT_CASES {
    {
        ($ptr:ident, $end:ident, $nextTokPtr:ident, $self:ident),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $end.wrapping_offset_from($ptr) < n {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if !$self.is_nmstrt_char($ptr, n) {
                    *$nextTokPtr = $ptr;
                    return XML_TOK_INVALID;
                }
                $ptr = $ptr.offset(n);
            }
            BT_NONASCII if !$self.is_nmstrt_char_minbpc($ptr) => {
                *$nextTokPtr = $ptr;
                return XML_TOK_INVALID;
            }
            BT_NONASCII | BT_NMSTRT | BT_HEX => {
                $ptr = $ptr.offset($self.MINBPC());
            }
            $($tail)*
        }
    }
}


macro_rules! HAS_CHARS {
    ($ptr:expr, $end:expr, $count:expr, $self:ident) => {
        $end.wrapping_offset_from($ptr) as c_long >= ($self.MINBPC() * $count) as c_long
    };
}

macro_rules! HAS_CHAR {
    ($ptr:expr, $end:expr, $self:ident) => {
        HAS_CHARS!($ptr, $end, 1, $self)
    };
}

macro_rules! REQUIRE_CHARS {
    ($ptr:expr, $end:expr, $count:expr, $self:ident) => {
        if !HAS_CHARS!($ptr, $end, $count, $self) {
            return XML_TOK_PARTIAL;
        }
    };
}

macro_rules! REQUIRE_CHAR {
    ($ptr:expr, $end:expr, $self:ident) => {
        REQUIRE_CHARS!($ptr, $end, 1, $self)
    };
}


pub trait XmlTokImpl: XmlEncodingImpl {
    /* ptr points to character following "<!-" */
    unsafe extern "C" fn scanComment(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if HAS_CHAR!(ptr, end, self) {
            if !self.char_matches(ptr, ASCII_MINUS) {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            ptr = ptr.offset(self.MINBPC() as isize);
            while HAS_CHAR!(ptr, end, self) {
                MATCH_INVALID_CASES! {
                    (ptr, end, nextTokPtr, self),
                    match self.byte_type(ptr),
                    BT_MINUS => {
                        ptr = ptr.offset(self.MINBPC() as isize);
                        REQUIRE_CHAR!(ptr, end, self);
                        if self.char_matches(ptr, ASCII_MINUS) {
                            ptr = ptr.offset(self.MINBPC() as isize);
                            REQUIRE_CHAR!(ptr, end, self);
                            if !self.char_matches(ptr, ASCII_GT) {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID
                            }
                            *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                            return XML_TOK_COMMENT
                        }
                    }
                    _ => { ptr = ptr.offset(self.MINBPC() as isize); }
                }
            }
        }
        return XML_TOK_PARTIAL;
    }

    /* ptr points to character following "<!" */
    unsafe extern "C" fn scanDecl(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        REQUIRE_CHAR!(ptr, end, self);
        match self.byte_type(ptr) {
            BT_MINUS => return self.scanComment(ptr.offset(self.MINBPC() as isize), end, nextTokPtr),
            BT_LSQB => {
                *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                return XML_TOK_COND_SECT_OPEN;
            }
            BT_NMSTRT | BT_HEX => ptr = ptr.offset(self.MINBPC() as isize),
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        while HAS_CHAR!(ptr, end, self) {
            's_151: {
                match self.byte_type(ptr) {
                    BT_PERCNT => {
                        REQUIRE_CHARS!(ptr, end, 2, self);
                        /* don't allow <!ENTITY% foo "whatever"> */
                        match self.byte_type(ptr.offset(self.MINBPC())) {
                            BT_S | BT_CR | BT_LF | BT_PERCNT => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => {}
                        }
                    }
                    BT_S | BT_CR | BT_LF => {}
                    BT_NMSTRT | BT_HEX => {
                        ptr = ptr.offset(self.MINBPC() as isize);
                        break 's_151;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                /* fall through */
                *nextTokPtr = ptr;
                return XML_TOK_DECL_OPEN;
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn checkPiTarget(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut tokPtr: *mut libc::c_int,
    ) -> libc::c_int {
        let mut upper: libc::c_int = 0 as libc::c_int;
        *tokPtr = XML_TOK_PI;
        if end.wrapping_offset_from(ptr) as libc::c_long != (self.MINBPC() * 3) as libc::c_long {
            return 1 as libc::c_int;
        }
        match self.byte_to_ascii(ptr) {
            ASCII_x => {}
            ASCII_X => upper = 1 as libc::c_int,
            _ => return 1 as libc::c_int,
        }
        ptr = ptr.offset(self.MINBPC() as isize);
        match self.byte_to_ascii(ptr) {
            ASCII_m => {}
            ASCII_M => upper = 1 as libc::c_int,
            _ => return 1 as libc::c_int,
        }
        ptr = ptr.offset(self.MINBPC() as isize);
        match self.byte_to_ascii(ptr) {
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
    unsafe extern "C" fn scanPi(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut tok: libc::c_int = 0;
        let mut target: *const libc::c_char = ptr;
        REQUIRE_CHAR!(ptr, end, self);
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, self),
            match self.byte_type(ptr),
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        while HAS_CHAR!(ptr, end, self) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, self),
                match self.byte_type(ptr),
                BT_S | BT_CR | BT_LF => {
                    if self.checkPiTarget(target, ptr, &mut tok) == 0
                       {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    ptr = ptr.offset(self.MINBPC() as isize);
                    while HAS_CHAR!(ptr, end, self) {
                        MATCH_INVALID_CASES! {
                            (ptr, end, nextTokPtr, self),
                            match self.byte_type(ptr),
                            BT_QUEST => {
                                ptr = ptr.offset(self.MINBPC() as isize);
                                REQUIRE_CHAR!(ptr, end, self);
                                if self.char_matches(ptr, ASCII_GT) {
                                    *nextTokPtr =
                                        ptr.offset(self.MINBPC() as isize);
                                    return tok
                                }
                            }
                            _ => { ptr = ptr.offset(self.MINBPC() as isize) }
                        }
                    }
                    return XML_TOK_PARTIAL
                }
                BT_QUEST => {
                    if self.checkPiTarget(target, ptr, &mut tok) == 0
                       {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    ptr = ptr.offset(self.MINBPC() as isize);
                    REQUIRE_CHAR!(ptr, end, self);
                    if self.char_matches(ptr, ASCII_GT) {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return tok
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID
                }
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanCdataSection(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
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
        REQUIRE_CHARS!(ptr, end, 6, self);
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            if !self.char_matches(ptr, CDATA_LSQB[i as usize]) {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            i += 1;
            ptr = ptr.offset(self.MINBPC() as isize)
        }
        *nextTokPtr = ptr;
        return XML_TOK_CDATA_SECT_OPEN;
    }
    unsafe extern "C" fn scanEndTag(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        REQUIRE_CHAR!(ptr, end, self);
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, self),
            match self.byte_type(ptr),
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        while HAS_CHAR!(ptr, end, self) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, self),
                match self.byte_type(ptr),
                BT_S | BT_CR | BT_LF => {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    while HAS_CHAR!(ptr, end, self) {
                        match self.byte_type(ptr) {
                            BT_S | BT_CR | BT_LF => { }
                            BT_GT => {
                                *nextTokPtr =
                                    ptr.offset(self.MINBPC() as isize);
                                return XML_TOK_END_TAG
                            }
                            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                        }
                        ptr = ptr.offset(self.MINBPC() as isize)
                    }
                    return XML_TOK_PARTIAL
                }
                BT_COLON => {
                    /* no need to check qname syntax here,
                    since end-tag must match exactly */
                    ptr = ptr.offset(self.MINBPC() as isize);
                }
                BT_GT => {
                    *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                    return XML_TOK_END_TAG
                }
                _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanHexCharRef(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if HAS_CHAR!(ptr, end, self) {
            match self.byte_type(ptr) {
                BT_DIGIT | BT_HEX => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr = ptr.offset(self.MINBPC() as isize);
            while HAS_CHAR!(ptr, end, self) {
                match self.byte_type(ptr) {
                    BT_DIGIT | BT_HEX => {}
                    BT_SEMI => {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                ptr = ptr.offset(self.MINBPC() as isize)
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanCharRef(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if HAS_CHAR!(ptr, end, self) {
            if self.char_matches(ptr, ASCII_x) {
                return self.scanHexCharRef(ptr.offset(self.MINBPC() as isize), end, nextTokPtr);
            }
            match self.byte_type(ptr) {
                BT_DIGIT => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr = ptr.offset(self.MINBPC() as isize);
            while HAS_CHAR!(ptr, end, self) {
                match self.byte_type(ptr) {
                    BT_DIGIT => {}
                    BT_SEMI => {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                ptr = ptr.offset(self.MINBPC() as isize)
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanRef(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        REQUIRE_CHAR!(ptr, end, self);
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, self),
            match self.byte_type(ptr),
            BT_NUM => {
                return self.scanCharRef(
                    ptr.offset(self.MINBPC() as isize),
                    end,
                    nextTokPtr,
                )
            }
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        while HAS_CHAR!(ptr, end, self) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, self),
                match self.byte_type(ptr),
                BT_SEMI => {
                    *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                    return XML_TOK_ENTITY_REF
                }
                _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanAtts(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
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
        while HAS_CHAR!(ptr, end, self) {
            let mut current_block: Label = Label::None;
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, self),
                match self.byte_type(ptr),
                BT_COLON => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    hadColon = 1;
                    ptr = ptr.offset(self.MINBPC() as isize);
                    REQUIRE_CHAR!(ptr, end, self);
                    CHECK_NMSTRT_CASES! {
                        (ptr, end, nextTokPtr, self),
                        match self.byte_type(ptr),
                        _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                    }
                }
                BT_S | BT_CR | BT_LF => {
                    loop  {
                        let mut t: C2RustUnnamed_2 = 0;
                        ptr = ptr.offset(self.MINBPC() as isize);
                        REQUIRE_CHAR!(ptr, end, self);
                        t = self.byte_type(ptr);
                        if t == BT_EQUALS { break ; }
                        match t {
                            BT_S | BT_LF | BT_CR => { }
                            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                        }
                    }
                    current_block = Label::EqFallthrough;
                }
                BT_EQUALS => { current_block = Label::EqFallthrough; }
                _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
            }
            if current_block == Label::EqFallthrough {
                /* fall through */
                // BT_S | BT_CR | BT_LF | BT_EQUALS =>
                let mut open: C2RustUnnamed_2 = 0;
                hadColon = 0;
                loop {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    REQUIRE_CHAR!(ptr, end, self);
                    open = self.byte_type(ptr);
                    if open == BT_QUOT || open == BT_APOS {
                        break;
                    }
                    match open {
                        BT_S | BT_LF | BT_CR => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                }
                ptr = ptr.offset(self.MINBPC() as isize);
                /* in attribute value */
                loop {
                    let mut t: C2RustUnnamed_2 = 0;
                    REQUIRE_CHAR!(ptr, end, self);
                    t = self.byte_type(ptr);
                    if t == open {
                        break;
                    }
                    MATCH_INVALID_CASES! {
                        (ptr, end, nextTokPtr, self),
                        match t,
                        BT_AMP => {
                            let mut tok: libc::c_int =
                                self.scanRef(ptr.offset(self.MINBPC() as isize),
                                             end, &mut ptr);
                            if tok <= 0 as libc::c_int {
                                if tok == XML_TOK_INVALID {
                                    *nextTokPtr = ptr
                                }
                                return tok
                            }
                        }
                        BT_LT => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                        _ => { ptr = ptr.offset(self.MINBPC() as isize) }
                    }
                }
                ptr = ptr.offset(self.MINBPC() as isize);
                REQUIRE_CHAR!(ptr, end, self);
                match self.byte_type(ptr) {
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
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                match current_block {
                    Label::Sol | Label::Gt => { }
                    _ => {
                        // Didn't take goto
                        /* ptr points to closing quote */
                        loop {
                            ptr = ptr.offset(self.MINBPC() as isize);
                            REQUIRE_CHAR!(ptr, end, self);
                            CHECK_NMSTRT_CASES! {
                                (ptr, end, nextTokPtr, self),
                                match self.byte_type(ptr),
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
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID
                                }
                            }
                            break;
                        }
                    }
                }

                match current_block {
                    Label::Gt => {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_START_TAG_WITH_ATTS;
                    }
                    Label::Sol => {
                        ptr = ptr.offset(self.MINBPC() as isize);
                        REQUIRE_CHAR!(ptr, end, self);
                        if !self.char_matches(ptr, ASCII_GT) {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                    }
                    _ => { }
                }
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanLt(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut hadColon: libc::c_int = 0;
        REQUIRE_CHAR!(ptr, end, self);
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, self),
            match self.byte_type(ptr),
            BT_EXCL => {
                ptr = ptr.offset(self.MINBPC() as isize);
                REQUIRE_CHAR!(ptr, end, self);
                match self.byte_type(ptr) {
                    BT_MINUS => {
                        return self.scanComment(ptr.offset(self.MINBPC() as isize),
                                                end, nextTokPtr)
                    }
                    BT_LSQB => {
                        return self.scanCdataSection(ptr.offset(self.MINBPC() as isize),
                                                     end, nextTokPtr)
                    }
                    _ => { }
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID
            }
            BT_QUEST => {
                return self.scanPi(ptr.offset(self.MINBPC() as isize),
                                       end, nextTokPtr)
            }
            BT_SOL => {
                return self.scanEndTag(ptr.offset(self.MINBPC() as isize),
                                       end, nextTokPtr)
            }
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        hadColon = 0 as libc::c_int;
        /* we have a start-tag */
        /* we have a start-tag */
        /* we have a start-tag */
        while HAS_CHAR!(ptr, end, self) {
            let mut current_block_161: u64 = 0;
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, self),
                match self.byte_type(ptr),
                BT_COLON => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    hadColon = 1 as libc::c_int;
                    ptr = ptr.offset(self.MINBPC() as isize);
                    REQUIRE_CHAR!(ptr, end, self);
                    CHECK_NMSTRT_CASES! {
                        (ptr, end, nextTokPtr, self),
                        match self.byte_type(ptr),
                        _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                    }
                    current_block_161 = 12655303178690906525;
                }
                BT_S | BT_CR | BT_LF => {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    loop  {
                        if !(HAS_CHAR!(ptr, end, self)) {
                            current_block_161 = 13000670339742628194;
                            break ;
                        }
                        CHECK_NMSTRT_CASES! {
                            (ptr, end, nextTokPtr, self),
                            match self.byte_type(ptr),
                            BT_GT => {
                                current_block_161 = 15370445274224965566;
                                break ;
                            }
                            BT_SOL => {
                                current_block_161 = 3926109038817298867;
                                break ;
                            }
                            BT_S | BT_CR | BT_LF => {
                                ptr = ptr.offset(self.MINBPC() as isize);
                                continue ;
                            }
                            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                        }
                        return self.scanAtts(ptr, end, nextTokPtr)
                    }
                    match current_block_161 {
                        15370445274224965566 => { }
                        3926109038817298867 => { }
                        _ => { return XML_TOK_PARTIAL }
                    }
                }
                BT_GT => { current_block_161 = 15370445274224965566; }
                BT_SOL => { current_block_161 = 3926109038817298867; }
                _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
            }
            match current_block_161 {
                3926109038817298867 => {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    REQUIRE_CHAR!(ptr, end, self);
                    if !self.char_matches(ptr, ASCII_GT) {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                }
                15370445274224965566 => {
                    *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                    return XML_TOK_START_TAG_NO_ATTS;
                }
                16331546839105579257 => ptr = ptr.offset(self.MINBPC()),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }

    /* ptr points to character following "%" */
    unsafe extern "C" fn scanPercent(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        REQUIRE_CHAR!(ptr, end, self);
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, self),
            match self.byte_type(ptr),
            BT_S | BT_LF | BT_CR | BT_PERCNT => { *nextTokPtr = ptr; return XML_TOK_PERCENT }
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        while HAS_CHAR!(ptr, end, self) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, self),
                match self.byte_type(ptr),
                BT_SEMI => {
                    *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                    return XML_TOK_PARAM_ENTITY_REF
                }
                _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanPoundName(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        REQUIRE_CHAR!(ptr, end, self);
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, self),
            match self.byte_type(ptr),
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        while HAS_CHAR!(ptr, end, self) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, self),
                match self.byte_type(ptr),
                BT_CR | BT_LF | BT_S | BT_RPAR | BT_GT | BT_PERCNT | BT_VERBAR => {
                    *nextTokPtr = ptr;
                    return XML_TOK_POUND_NAME
                }
                _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
            }
        }
        return -XML_TOK_POUND_NAME;
    }
    unsafe extern "C" fn scanLit(
        &self,
        mut open: C2RustUnnamed_2,

        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        while HAS_CHAR!(ptr, end, self) {
            let mut t: C2RustUnnamed_2 = self.byte_type(ptr);
            MATCH_INVALID_CASES! {
                (ptr, end, nextTokPtr, self),
                match t,
                BT_QUOT | BT_APOS => {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    if !(t != open) {
                        if !HAS_CHAR!(ptr, end, self) {
                            return -XML_TOK_LITERAL
                        }
                        *nextTokPtr = ptr;
                        match self.byte_type(ptr) {
                            BT_S | BT_CR | BT_LF | BT_GT | BT_PERCNT | BT_LSQB => {
                                return XML_TOK_LITERAL
                            }
                            _ => { return XML_TOK_INVALID }
                        }
                    }
                }
                _ => { ptr = ptr.offset(self.MINBPC() as isize) }
            }
        }
        return XML_TOK_PARTIAL;
    }
}

impl<T: XmlEncodingImpl> XmlTokImpl for T { }

impl<T: XmlEncodingImpl+XmlTokImpl> XmlEncoding for T {
    unsafe extern "C" fn cdataSectionTok(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if self.MINBPC() > 1 {
            let mut n: size_t = end.wrapping_offset_from(ptr) as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        MATCH_INVALID_CASES! {
            (ptr, end, nextTokPtr, self),
            match self.byte_type(ptr),
            BT_RSQB => {
                ptr = ptr.offset(self.MINBPC() as isize);
                REQUIRE_CHAR!(ptr, end, self);
                if self.char_matches(ptr, ASCII_RSQB) {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    REQUIRE_CHAR!(ptr, end, self);
                    if !self.char_matches(ptr, ASCII_GT) {
                        ptr = ptr.offset(-(self.MINBPC() as isize))
                    } else {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_CDATA_SECT_CLOSE
                    }
                }
            }
            BT_CR => {
                ptr = ptr.offset(self.MINBPC() as isize);
                REQUIRE_CHAR!(ptr, end, self);
                if self.byte_type(ptr) == BT_LF {
                    ptr = ptr.offset(self.MINBPC() as isize)
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE
            }
            BT_LF => {
                *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                return XML_TOK_DATA_NEWLINE
            }
            _ => { ptr = ptr.offset(self.MINBPC() as isize) }
        }
        while HAS_CHAR!(ptr, end, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(ptr),
                LEAD_CASE(n) => {
                    if end.wrapping_offset_from(ptr) < n || self.is_invalid_char(ptr, n) {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(n)
                }
                BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF | BT_RSQB => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                _ => { ptr = ptr.offset(self.MINBPC() as isize) }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
    unsafe extern "C" fn contentTok(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if self.MINBPC() > 1 {
            let mut n: size_t = end.wrapping_offset_from(ptr) as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        MATCH_INVALID_CASES! {
            (ptr, end, nextTokPtr, self),
            match self.byte_type(ptr),
            BT_LT => {
                return self.scanLt(ptr.offset(self.MINBPC() as isize),
                                       end, nextTokPtr)
            }
            BT_AMP => {
                return self.scanRef(ptr.offset(self.MINBPC() as isize),
                                        end, nextTokPtr)
            }
            BT_CR => {
                ptr = ptr.offset(self.MINBPC() as isize);
                if !HAS_CHAR!(ptr, end, self) {
                    return XML_TOK_TRAILING_CR
                }
                if self.byte_type(ptr) == BT_LF {
                    ptr = ptr.offset(self.MINBPC() as isize)
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE
            }
            BT_LF => {
                *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                return XML_TOK_DATA_NEWLINE
            }
            BT_RSQB => {
                ptr = ptr.offset(self.MINBPC() as isize);
                if !HAS_CHAR!(ptr, end, self) {
                    return XML_TOK_TRAILING_RSQB
                }
                if !self.char_matches(ptr, ASCII_RSQB) {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    if !HAS_CHAR!(ptr, end, self) {
                        return XML_TOK_TRAILING_RSQB
                    }
                    if !self.char_matches(ptr, ASCII_GT) {
                        ptr = ptr.offset(-(self.MINBPC() as isize))
                    } else { *nextTokPtr = ptr; return XML_TOK_INVALID }
                }
            }
            _ => { ptr = ptr.offset(self.MINBPC() as isize) }
        }
        while HAS_CHAR!(ptr, end, self) {
            let mut current_block_76: u64;
            MATCH_LEAD_CASES! {
                self.byte_type(ptr),
                LEAD_CASE(n) => {
                    if end.wrapping_offset_from(ptr) < n || self.is_invalid_char(ptr, n) {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(n);
                    current_block_76 = 10213293998891106930;
                }
                BT_RSQB => {
                    if HAS_CHARS!(ptr, end, 2, self) {
                        if !self.char_matches(ptr.offset(self.MINBPC()), ASCII_RSQB) {
                            ptr = ptr.offset(self.MINBPC() as isize);
                            current_block_76 = 10213293998891106930;
                        } else if HAS_CHARS!(ptr, end, 3, self) {
                            if !self.char_matches(ptr.offset(2 * self.MINBPC()), ASCII_GT) {
                                ptr = ptr.offset(self.MINBPC() as isize)
                            } else {
                                *nextTokPtr =
                                    ptr.offset((2 * self.MINBPC()) as isize);
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
                    ptr = ptr.offset(self.MINBPC() as isize);
                    current_block_76 = 10213293998891106930;
                }
            }
            match current_block_76 {
                10213293998891106930 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
    unsafe extern "C" fn prologTok(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut tok: libc::c_int = 0;
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if self.MINBPC() > 1 {
            let mut n: size_t = end.wrapping_offset_from(ptr) as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        let mut current_block_112: u64;
        MATCH_LEAD_CASES! {
            self.byte_type(ptr),
            LEAD_CASE(n) => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if self.is_nmstrt_char(ptr, 2) {
                    ptr = ptr.offset(n);
                    tok = XML_TOK_NAME;
                } else if self.is_name_char(ptr, 2) {
                    ptr = ptr.offset(n);
                    tok = XML_TOK_NMTOKEN
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID
                }
                current_block_112 = 2222055338596505704;
            }
            BT_QUOT => {
                return self.scanLit(BT_QUOT,
                                        ptr.offset(self.MINBPC() as isize),
                                        end, nextTokPtr)
            }
            BT_APOS => {
                return self.scanLit(BT_APOS,
                                        ptr.offset(self.MINBPC() as isize),
                                        end, nextTokPtr)
            }
            BT_LT => {
                ptr = ptr.offset(self.MINBPC() as isize);
                REQUIRE_CHAR!(ptr, end, self);
                match self.byte_type(ptr) {
                    BT_EXCL => {
                        return self.scanDecl(ptr.offset(self.MINBPC() as
                                                                isize), end,
                                                 nextTokPtr)
                    }
                    BT_QUEST => {
                        return self.scanPi(ptr.offset(self.MINBPC() as
                                                              isize), end,
                                               nextTokPtr)
                    }
                    BT_NMSTRT | BT_HEX | BT_NONASCII | BT_LEAD2 | BT_LEAD3 | BT_LEAD4 => {
                        *nextTokPtr = ptr.offset(-(self.MINBPC() as isize));
                        return XML_TOK_INSTANCE_START
                    }
                    _ => { }
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID
            }
            BT_CR => {
                if ptr.offset(self.MINBPC() as isize) == end {
                    *nextTokPtr = end;
                    /* indicate that this might be part of a CR/LF pair */
                    /* indicate that this might be part of a CR/LF pair */
                    /* indicate that this might be part of a CR/LF pair */
                    return -XML_TOK_PROLOG_S
                }
                current_block_112 = 1103933966285275534;
            }
            BT_S | BT_LF => { current_block_112 = 1103933966285275534; }
            BT_PERCNT => {
                return self.scanPercent(ptr.offset(self.MINBPC() as isize),
                                            end, nextTokPtr)
            }
            BT_COMMA => {
                *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                return XML_TOK_COMMA
            }
            BT_LSQB => {
                *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                return XML_TOK_OPEN_BRACKET
            }
            BT_RSQB => {
                ptr = ptr.offset(self.MINBPC() as isize);
                if !HAS_CHAR!(ptr, end, self) {
                    return -XML_TOK_CLOSE_BRACKET
                }
                if self.char_matches(ptr, ASCII_RSQB) {
                    REQUIRE_CHARS!(ptr, end, 2, self);
                    if self.char_matches(ptr.offset(self.MINBPC()), ASCII_GT) {
                        *nextTokPtr =
                            ptr.offset((2 * self.MINBPC()) as isize);
                        return XML_TOK_COND_SECT_CLOSE
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET
            }
            BT_LPAR => {
                *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                return XML_TOK_OPEN_PAREN
            }
            BT_RPAR => {
                ptr = ptr.offset(self.MINBPC() as isize);
                if !HAS_CHAR!(ptr, end, self) {
                    return -XML_TOK_CLOSE_PAREN
                }
                match self.byte_type(ptr) {
                    BT_AST => {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK
                    }
                    BT_QUEST => {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION
                    }
                    BT_PLUS => {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_CLOSE_PAREN_PLUS
                    }
                    BT_CR | BT_LF | BT_S | BT_GT | BT_COMMA | BT_VERBAR | BT_RPAR => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN
                    }
                    _ => { }
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID
            }
            BT_VERBAR => {
                *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                return XML_TOK_OR
            }
            BT_GT => {
                *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                return XML_TOK_DECL_CLOSE
            }
            BT_NUM => {
                return self.scanPoundName(ptr.offset(self.MINBPC() as
                                                             isize), end,
                                              nextTokPtr)
            }
            BT_NMSTRT | BT_HEX => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(self.MINBPC() as isize);
                current_block_112 = 2222055338596505704;
            }
            BT_DIGIT | BT_NAME | BT_MINUS | BT_COLON => {
                tok = XML_TOK_NMTOKEN;
                ptr = ptr.offset(self.MINBPC() as isize);
                current_block_112 = 2222055338596505704;
            }
            BT_NONASCII => {
                if self.is_nmstrt_char_minbpc(ptr) {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    tok = XML_TOK_NAME;
                } else if self.is_name_char_minbpc(ptr) {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    tok = XML_TOK_NMTOKEN;
                }
                current_block_112 = 2222055338596505704;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID
            }
        }
        match current_block_112 {
            2222055338596505704 => {}
            _ =>
            /* fall through */
            /* fall through */
            /* fall through */
            {
                loop {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    if !HAS_CHAR!(ptr, end, self) {
                        break;
                    }
                    let mut current_block_32: u64;
                    match self.byte_type(ptr) {
                        BT_S | BT_LF => {
                            current_block_32 = 14072441030219150333;
                        }
                        BT_CR => {
                            /* don't split CR/LF pair */
                            /* don't split CR/LF pair */
                            /* don't split CR/LF pair */
                            if ptr.offset(self.MINBPC() as isize) != end {
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
                        /* fall through */
                        /* fall through */
                        {
                            *nextTokPtr = ptr;
                            return XML_TOK_PROLOG_S;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_PROLOG_S;
            }
        }
        while HAS_CHAR!(ptr, end, self) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, self),
                match self.byte_type(ptr),
                BT_GT | BT_RPAR | BT_COMMA | BT_VERBAR | BT_LSQB | BT_PERCNT | BT_S | BT_CR | BT_LF => {
                    *nextTokPtr = ptr;
                    return tok
                }
                BT_COLON => {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    match tok {
                        XML_TOK_NAME => {
                            REQUIRE_CHAR!(ptr, end, self);
                            tok = XML_TOK_PREFIXED_NAME;
                            CHECK_NAME_CASES! {
                                (ptr, end, nextTokPtr, self),
                                match self.byte_type(ptr),
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
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                    return XML_TOK_NAME_PLUS
                }
                BT_AST => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                    return XML_TOK_NAME_ASTERISK
                }
                BT_QUEST => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                    return XML_TOK_NAME_QUESTION
                }
                _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
            }
        }
        return -tok;
    }
    unsafe extern "C" fn attributeValueTok(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut start: *const libc::c_char = 0 as *const libc::c_char;
        if ptr >= end {
            return XML_TOK_NONE;
        } else {
            if !HAS_CHAR!(ptr, end, self) {
                return XML_TOK_PARTIAL;
            }
        }
        start = ptr;
        while HAS_CHAR!(ptr, end, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(ptr),
                LEAD_CASE(n) => {
                    ptr = ptr.offset(n);
                }
                BT_AMP => {
                    if ptr == start {
                        return self.scanRef(ptr.offset(self.MINBPC() as
                                                               isize), end,
                                                nextTokPtr)
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                BT_LT => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                BT_LF => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                BT_CR => {
                    if ptr == start {
                        ptr = ptr.offset(self.MINBPC() as isize);
                        if !HAS_CHAR!(ptr, end, self) {
                            return XML_TOK_TRAILING_CR
                        }
                        if self.byte_type(ptr) == BT_LF {
                            ptr = ptr.offset(self.MINBPC() as isize)
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                BT_S => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                _ => { ptr = ptr.offset(self.MINBPC() as isize) }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
    unsafe extern "C" fn entityValueTok(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut start: *const libc::c_char = 0 as *const libc::c_char;
        if ptr >= end {
            return XML_TOK_NONE;
        } else {
            if !HAS_CHAR!(ptr, end, self) {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                return XML_TOK_PARTIAL;
            }
        }
        start = ptr;
        while HAS_CHAR!(ptr, end, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(ptr),
                LEAD_CASE(n) => {
                    ptr = ptr.offset(n);
                }
                BT_AMP => {
                    if ptr == start {
                        return self.scanRef(ptr.offset(self.MINBPC() as
                                                               isize), end,
                                                nextTokPtr)
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                BT_PERCNT => {
                    if ptr == start {
                        let mut tok: libc::c_int =
                            self.scanPercent(ptr.offset(self.MINBPC() as
                                                                isize), end,
                                                 nextTokPtr);
                        return if tok == XML_TOK_PERCENT {
                                   XML_TOK_INVALID
                               } else { tok }
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                BT_LF => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(self.MINBPC() as isize);
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                BT_CR => {
                    if ptr == start {
                        ptr = ptr.offset(self.MINBPC() as isize);
                        if !HAS_CHAR!(ptr, end, self) {
                            return XML_TOK_TRAILING_CR
                        }
                        if self.byte_type(ptr) == BT_LF {
                            ptr = ptr.offset(self.MINBPC() as isize)
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                _ => { ptr = ptr.offset(self.MINBPC() as isize) }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
    unsafe extern "C" fn ignoreSectionTok(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        let mut level: libc::c_int = 0 as libc::c_int;
        if self.MINBPC() > 1 {
            let mut n: size_t = end.wrapping_offset_from(ptr) as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                end = ptr.offset(n as isize)
            }
        }
        while HAS_CHAR!(ptr, end, self) {
            MATCH_INVALID_CASES! {
                (ptr, end, nextTokPtr, self),
                match self.byte_type(ptr),
                BT_LT => {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    REQUIRE_CHAR!(ptr, end, self);
                    if self.char_matches(ptr, ASCII_EXCL) {
                        ptr = ptr.offset(self.MINBPC() as isize);
                        REQUIRE_CHAR!(ptr, end, self);
                        if self.char_matches(ptr, ASCII_LSQB) {
                            level += 1;
                            ptr = ptr.offset(self.MINBPC() as isize)
                        }
                    }
                }
                BT_RSQB => {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    REQUIRE_CHAR!(ptr, end, self);
                    if self.char_matches(ptr, ASCII_RSQB) {
                        ptr = ptr.offset(self.MINBPC() as isize);
                        REQUIRE_CHAR!(ptr, end, self);
                        if self.char_matches(ptr, ASCII_GT) {
                            ptr = ptr.offset(self.MINBPC() as isize);
                            if level == 0 as libc::c_int {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT
                            }
                            level -= 1
                        }
                    }
                }
                _ => { ptr = ptr.offset(self.MINBPC() as isize) }
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn isPublicId(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut badPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        ptr = ptr.offset(self.MINBPC() as isize);
        end = end.offset(-(self.MINBPC() as isize));
        while HAS_CHAR!(ptr, end, self) {
            let mut current_block_8: u64;
            match self.byte_type(ptr) {
                BT_DIGIT | BT_HEX | BT_MINUS | BT_APOS | BT_LPAR | BT_RPAR | BT_PLUS | BT_COMMA | BT_SOL | BT_EQUALS | BT_QUEST | BT_CR | BT_LF | BT_SEMI | BT_EXCL | 33
                | BT_PERCNT | BT_NUM | BT_COLON => {
                    current_block_8 = 13242334135786603907;
                }
                BT_S => {
                    if self.char_matches(ptr, ASCII_TAB) {
                        *badPtr = ptr;
                        return 0 as libc::c_int;
                    }
                    current_block_8 = 13242334135786603907;
                }
                BT_NAME | BT_NMSTRT => {
                    if self.byte_to_ascii(ptr) & !(0x7f as c_char) == 0 {
                        current_block_8 = 13242334135786603907;
                    } else {
                        current_block_8 = 13935456465286830489;
                    }
                }
                _ => {
                    current_block_8 = 13935456465286830489;
                }
            }
            match current_block_8 {
                13935456465286830489 =>
                /* fall through */
                /* fall through */
                /* fall through */
                {
                    match self.byte_to_ascii(ptr) {
                        36 => {}
                        64 => {}
                        _ => {
                            *badPtr = ptr;
                            return 0 as libc::c_int;
                        }
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(self.MINBPC() as isize)
        }
        return 1 as libc::c_int;
    }
    unsafe extern "C" fn getAtts(
        &self,
        mut ptr: *const libc::c_char,
        mut attsMax: libc::c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> libc::c_int {
        let mut state: C2RustUnnamed_2 = inName;
        let mut nAtts: libc::c_int = 0 as libc::c_int;
        let mut open: C2RustUnnamed_2 = 0 as C2RustUnnamed_2;

        macro_rules! START_NAME {
            () => {
                if state as libc::c_uint == other as libc::c_int as libc::c_uint {
                    if nAtts < attsMax {
                        let ref mut fresh120 = (*atts.offset(nAtts as isize)).name;
                        *fresh120 = ptr;
                        (*atts.offset(nAtts as isize)).normalized = 1 as libc::c_int as libc::c_char
                    }
                    state = inName
                }
            };
        }
        /* defined when state == inValue;
        initialization just to shut up compilers */
        /* defined when state == inValue;
        initialization just to shut up compilers */
        /* defined when state == inValue;
        initialization just to shut up compilers */
        ptr = ptr.offset(self.MINBPC() as isize);
        loop {
            MATCH_LEAD_CASES! {
                   self.byte_type(ptr),
                   LEAD_CASE(n) => {
                       START_NAME!{}
                       ptr = ptr.offset((n - self.MINBPC()) as isize)
                   }
                   BT_NONASCII | BT_NMSTRT | BT_HEX => {
                       if state as libc::c_uint ==
                              other as libc::c_int as libc::c_uint {
                           if nAtts < attsMax {
                               let ref mut fresh126 =
                                   (*atts.offset(nAtts as isize)).name;
                               *fresh126 = ptr;
                               (*atts.offset(nAtts as isize)).normalized =
                                   1 as libc::c_int as libc::c_char
                           }
                           state = inName
                       }
                   }
                   BT_QUOT => {
                       if state as libc::c_uint !=
                              inValue as libc::c_int as libc::c_uint {
                           if nAtts < attsMax {
                               let ref mut fresh127 =
                                   (*atts.offset(nAtts as isize)).valuePtr;
                               *fresh127 = ptr.offset(self.MINBPC() as isize)
                           }
                           state = inValue;
                           open = BT_QUOT
                       } else if open == BT_QUOT {
                           state = other;
                           if nAtts < attsMax {
                               let ref mut fresh128 =
                                   (*atts.offset(nAtts as isize)).valueEnd;
                               *fresh128 = ptr
                           }
                           nAtts += 1
                       }
                   }
                   BT_APOS => {
                       if state as libc::c_uint !=
                              inValue as libc::c_int as libc::c_uint {
                           if nAtts < attsMax {
                               let ref mut fresh129 =
                                   (*atts.offset(nAtts as isize)).valuePtr;
                               *fresh129 = ptr.offset(self.MINBPC() as isize)
                           }
                           state = inValue;
                           open = BT_APOS
                       } else if open == BT_APOS {
                           state = other;
                           if nAtts < attsMax {
                               let ref mut fresh130 =
                                   (*atts.offset(nAtts as isize)).valueEnd;
                               *fresh130 = ptr
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
                       if state as libc::c_uint ==
                              inName as libc::c_int as libc::c_uint {
                           state = other
                       } else if state as libc::c_uint ==
                                     inValue as libc::c_int as libc::c_uint &&
                                     nAtts < attsMax &&
                                     (*atts.offset(nAtts as isize)).normalized as
                                         libc::c_int != 0 &&
                                     (ptr ==
                                          (*atts.offset(nAtts as isize)).valuePtr
                                          ||
                                          self.byte_to_ascii(ptr) != ASCII_SPACE
                                          ||
                                          self.byte_to_ascii(ptr.offset(self.MINBPC()))
                                              == ASCII_SPACE ||
                                          self.byte_type(ptr.offset(self.MINBPC())) ==
                                              open) {
                           (*atts.offset(nAtts as isize)).normalized =
                               0 as libc::c_int as libc::c_char
                       }
                   }
                   BT_CR | BT_LF => {
                       /* This case ensures that the first attribute name is counted
            Apart from that we could just change state on the quote. */
                       /* This case ensures that the first attribute name is counted
            Apart from that we could just change state on the quote. */
                       /* This case ensures that the first attribute name is counted
            Apart from that we could just change state on the quote. */
                       if state as libc::c_uint ==
                              inName as libc::c_int as libc::c_uint {
                           state = other
                       } else if state as libc::c_uint ==
                                     inValue as libc::c_int as libc::c_uint &&
                                     nAtts < attsMax {
                           (*atts.offset(nAtts as isize)).normalized =
                               0 as libc::c_int as libc::c_char
                       }
                   }
                   BT_GT | BT_SOL => {
                       if state as libc::c_uint !=
                              inValue as libc::c_int as libc::c_uint {
                           return nAtts
                       }
                   }
                   _ => { }
               }
            ptr = ptr.offset(self.MINBPC() as isize)
        }
        /* not reached */
        /* not reached */
        /* not reached */
    }
    unsafe extern "C" fn charRefNumber(&self, mut ptr: *const libc::c_char) -> libc::c_int {
        let mut result: libc::c_int = 0 as libc::c_int;
        ptr = ptr.offset((2 * self.MINBPC()) as isize);
        if self.char_matches(ptr, ASCII_x) {
            ptr = ptr.offset(self.MINBPC() as isize);
            while !self.char_matches(ptr, ASCII_SEMI) {
                let mut c: c_char = self.byte_to_ascii(ptr);
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
                ptr = ptr.offset(self.MINBPC() as isize)
            }
        } else {
            while !self.char_matches(ptr, ASCII_SEMI) {
                let mut c_0: c_char = self.byte_to_ascii(ptr);
                result *= 10 as libc::c_int;
                result += (c_0 - ASCII_0) as c_int;
                if result >= 0x110000 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                ptr = ptr.offset(self.MINBPC() as isize)
            }
        }
        return checkCharRefNumber(result);
    }
    unsafe extern "C" fn predefinedEntityName(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
    ) -> libc::c_int {
        match end.wrapping_offset_from(ptr) / self.MINBPC() {
            2 => {
                if self.char_matches(ptr.offset(self.MINBPC()), ASCII_t) {
                    match self.byte_to_ascii(ptr) {
                        ASCII_l => return ASCII_LT as c_int,
                        ASCII_g => return ASCII_GT as c_int,
                        _ => {}
                    }
                }
            }
            3 => {
                if self.char_matches(ptr, ASCII_a) {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    if self.char_matches(ptr, ASCII_m) {
                        ptr = ptr.offset(self.MINBPC() as isize);
                        if self.char_matches(ptr, ASCII_p) {
                            return ASCII_AMP as c_int;
                        }
                    }
                }
            }
            4 => match self.byte_to_ascii(ptr) {
                ASCII_q => {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    if self.char_matches(ptr, ASCII_u) {
                        ptr = ptr.offset(self.MINBPC() as isize);
                        if self.char_matches(ptr, ASCII_o) {
                            ptr = ptr.offset(self.MINBPC() as isize);
                            if self.char_matches(ptr, ASCII_t) {
                                return ASCII_QUOT as c_int;
                            }
                        }
                    }
                }
                ASCII_a => {
                    ptr = ptr.offset(self.MINBPC() as isize);
                    if self.char_matches(ptr, ASCII_p) {
                        ptr = ptr.offset(self.MINBPC() as isize);
                        if self.char_matches(ptr, ASCII_o) {
                            ptr = ptr.offset(self.MINBPC() as isize);
                            if self.char_matches(ptr, ASCII_s) {
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
        mut ptr1: *const libc::c_char,
        mut end1: *const libc::c_char,
        mut ptr2: *const libc::c_char,
    ) -> libc::c_int {
        while *ptr2 != 0 {
            if end1.wrapping_offset_from(ptr1) < self.MINBPC() {
                return 0 as libc::c_int;
            }
            if !self.char_matches(ptr1, *ptr2) {
                return 0 as libc::c_int;
            }
            ptr1 = ptr1.offset(self.MINBPC() as isize);
            ptr2 = ptr2.offset(1)
        }
        return (ptr1 == end1) as libc::c_int;
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
                    ptr = ptr.offset(self.MINBPC() as isize)
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
                BT_LF | BT_CR | BT_S => ptr = ptr.offset(self.MINBPC() as isize),
                _ => return ptr,
            }
        }
    }
    unsafe extern "C" fn updatePosition(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut pos: *mut POSITION,
    ) {
        while HAS_CHAR!(ptr, end, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(ptr),
                LEAD_CASE(n) => {
                    ptr = ptr.offset(n);
                }
                BT_LF => {
                    (*pos).columnNumber = -(1 as libc::c_int) as XML_Size;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(self.MINBPC() as isize)
                }
                BT_CR => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(self.MINBPC() as isize);
                    if HAS_CHAR!(ptr, end, self) &&
                           self.byte_type(ptr) == BT_LF {
                        ptr = ptr.offset(self.MINBPC() as isize)
                    }
                    (*pos).columnNumber = -(1 as libc::c_int) as XML_Size
                }
                _ => { ptr = ptr.offset(self.MINBPC() as isize) }
            }
            (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1)
        }
    }

    unsafe extern "C" fn utf8Convert(
        &self,
        fromP: *mut *const c_char,
        fromLim: *const c_char,
        toP: *mut *mut c_char,
        toLim: *const c_char,
    ) -> XML_Convert_Result {
        self.utf8Convert(fromP, fromLim, toP, toLim)
    }

    unsafe extern "C" fn utf16Convert(
        &self,
        fromP: *mut *const c_char,
        fromLim: *const c_char,
        toP: *mut *mut c_ushort,
        toLim: *const c_ushort,
    ) -> XML_Convert_Result {
        self.utf16Convert(fromP, fromLim, toP, toLim)
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
