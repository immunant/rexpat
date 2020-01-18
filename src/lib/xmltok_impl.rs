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
        ($ptr:ident, $end:ident, $nextTokPtr:ident, $enc:ty),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $end.wrapping_offset_from($ptr) < n {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if <$enc>::is_invalid_char($ptr, n) {
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
        ($ptr:ident, $end:ident, $nextTokPtr:ident, $enc:ty),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $end.wrapping_offset_from($ptr) < n {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if !<$enc>::is_name_char($ptr, n) {
                    *$nextTokPtr = $ptr;
                    return XML_TOK_INVALID;
                }
                $ptr = $ptr.offset(n);
            }
            BT_NONASCII if !<$enc>::is_name_char_minbpc($ptr) => {
                *$nextTokPtr = $ptr;
                return XML_TOK_INVALID;
            }
            BT_NONASCII | BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                $ptr = $ptr.offset(<$enc>::MINBPC);
            }
            $($tail)*
        }
    }
}

macro_rules! CHECK_NMSTRT_CASES {
    {
        ($ptr:ident, $end:ident, $nextTokPtr:ident, $enc:ty),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $end.wrapping_offset_from($ptr) < n {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if !<$enc>::is_nmstrt_char($ptr, n) {
                    *$nextTokPtr = $ptr;
                    return XML_TOK_INVALID;
                }
                $ptr = $ptr.offset(n);
            }
            BT_NONASCII if !<$enc>::is_nmstrt_char_minbpc($ptr) => {
                *$nextTokPtr = $ptr;
                return XML_TOK_INVALID;
            }
            BT_NONASCII | BT_NMSTRT | BT_HEX => {
                $ptr = $ptr.offset(<$enc>::MINBPC);
            }
            $($tail)*
        }
    }
}

pub struct XmlTokImpl<E: XmlEncodingImpl>(std::marker::PhantomData<E>);

macro_rules! HAS_CHARS {
    ($enc:expr, $ptr:expr, $end:expr, $count:expr) => {
        $end.wrapping_offset_from($ptr) as c_long >= (E::MINBPC * $count) as c_long
    };
}

macro_rules! HAS_CHAR {
    ($enc:expr, $ptr:expr, $end:expr) => {
        HAS_CHARS!($enc, $ptr, $end, 1)
    };
}

impl<E: XmlEncodingImpl> XmlTokImpl<E> {
    pub fn new() -> Self {
        Self(std::marker::PhantomData)
    }

    unsafe extern "C" fn scanComment(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if HAS_CHAR!(enc, ptr, end) {
            if !E::char_matches(ptr, ASCII_MINUS) {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            ptr = ptr.offset(E::MINBPC as isize);
            while HAS_CHAR!(enc, ptr, end) {
                MATCH_INVALID_CASES! {
                    (ptr, end, nextTokPtr, E),
                    match E::byte_type(ptr),
                    BT_MINUS => {
                        ptr = ptr.offset(E::MINBPC as isize);
                        if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                                 (1 as libc::c_int * 1 as libc::c_int) as
                                     libc::c_long) {
                            return XML_TOK_PARTIAL
                        }
                        if E::char_matches(ptr, ASCII_MINUS) {
                            ptr = ptr.offset(E::MINBPC as isize);
                            if !(end.wrapping_offset_from(ptr) as libc::c_long
                                     >=
                                     (1 as libc::c_int * 1 as libc::c_int) as
                                         libc::c_long) {
                                return XML_TOK_PARTIAL
                            }
                            if !E::char_matches(ptr, ASCII_GT) {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID
                            }
                            *nextTokPtr = ptr.offset(E::MINBPC as isize);
                            return XML_TOK_COMMENT
                        }
                    }
                    _ => { ptr = ptr.offset(E::MINBPC as isize); }
                }
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanDecl(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if !(end.wrapping_offset_from(ptr) as libc::c_long
            >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        match E::byte_type(ptr) {
            27 => return self.scanComment(ptr.offset(E::MINBPC as isize), end, nextTokPtr),
            20 => {
                *nextTokPtr = ptr.offset(E::MINBPC as isize);
                return XML_TOK_COND_SECT_OPEN;
            }
            22 | 24 => ptr = ptr.offset(E::MINBPC as isize),
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        while HAS_CHAR!(enc, ptr, end) {
            's_151: {
                match E::byte_type(ptr) {
                    30 => {
                        if !(end.wrapping_offset_from(ptr) as libc::c_long
                            >= (2 as libc::c_int * 1 as libc::c_int) as libc::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        /* don't allow <!ENTITY% foo "whatever"> */
                        /* don't allow <!ENTITY% foo "whatever"> */
                        /* don't allow <!ENTITY% foo "whatever"> */
                        match E::byte_type(ptr.offset(E::MINBPC)) {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(E::MINBPC as isize);
                        break 's_151;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                /* fall through */
                /* fall through */
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
        if end.wrapping_offset_from(ptr) as libc::c_long != (E::MINBPC * 3) as libc::c_long {
            return 1 as libc::c_int;
        }
        match E::byte_to_ascii(ptr) {
            ASCII_x => {}
            ASCII_X => upper = 1 as libc::c_int,
            _ => return 1 as libc::c_int,
        }
        ptr = ptr.offset(E::MINBPC as isize);
        match E::byte_to_ascii(ptr) {
            ASCII_m => {}
            ASCII_M => upper = 1 as libc::c_int,
            _ => return 1 as libc::c_int,
        }
        ptr = ptr.offset(E::MINBPC as isize);
        match E::byte_to_ascii(ptr) {
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
        if !(end.wrapping_offset_from(ptr) as libc::c_long
            >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, E),
            match E::byte_type(ptr),
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        while HAS_CHAR!(enc, ptr, end) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, E),
                match E::byte_type(ptr),
                21 | 9 | 10 => {
                    if self.checkPiTarget(target, ptr, &mut tok) == 0
                       {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    ptr = ptr.offset(E::MINBPC as isize);
                    while HAS_CHAR!(enc, ptr, end) {
                        MATCH_INVALID_CASES! {
                            (ptr, end, nextTokPtr, E),
                            match E::byte_type(ptr),
                            15 => {
                                ptr = ptr.offset(E::MINBPC as isize);
                                if !(end.wrapping_offset_from(ptr) as
                                         libc::c_long >=
                                         (1 as libc::c_int * 1 as libc::c_int)
                                             as libc::c_long) {
                                    return XML_TOK_PARTIAL
                                }
                                if E::char_matches(ptr, ASCII_GT) {
                                    *nextTokPtr =
                                        ptr.offset(E::MINBPC as isize);
                                    return tok
                                }
                            }
                            _ => { ptr = ptr.offset(E::MINBPC as isize) }
                        }
                    }
                    return XML_TOK_PARTIAL
                }
                15 => {
                    if self.checkPiTarget(target, ptr, &mut tok) == 0
                       {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                             (1 as libc::c_int * 1 as libc::c_int) as
                                 libc::c_long) {
                        return XML_TOK_PARTIAL
                    }
                    if E::char_matches(ptr, ASCII_GT) {
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
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
        if !(end.wrapping_offset_from(ptr) as libc::c_long
            >= (6 as libc::c_int * 1 as libc::c_int) as libc::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            if !E::char_matches(ptr, CDATA_LSQB[i as usize]) {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            i += 1;
            ptr = ptr.offset(E::MINBPC as isize)
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
        if !(end.wrapping_offset_from(ptr) as libc::c_long
            >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, E),
            match E::byte_type(ptr),
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        while HAS_CHAR!(enc, ptr, end) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, E),
                match E::byte_type(ptr),
                21 | 9 | 10 => {
                    ptr = ptr.offset(E::MINBPC as isize);
                    while HAS_CHAR!(enc, ptr, end) {
                        match E::byte_type(ptr) {
                            21 | 9 | 10 => { }
                            11 => {
                                *nextTokPtr =
                                    ptr.offset(E::MINBPC as isize);
                                return XML_TOK_END_TAG
                            }
                            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                        }
                        ptr = ptr.offset(E::MINBPC as isize)
                    }
                    return XML_TOK_PARTIAL
                }
                23 => {
                    /* no need to check qname syntax here,
                    since end-tag must match exactly */
                    ptr = ptr.offset(E::MINBPC as isize);
                }
                11 => {
                    *nextTokPtr = ptr.offset(E::MINBPC as isize);
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
        if HAS_CHAR!(enc, ptr, end) {
            match E::byte_type(ptr) {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr = ptr.offset(E::MINBPC as isize);
            while HAS_CHAR!(enc, ptr, end) {
                match E::byte_type(ptr) {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                ptr = ptr.offset(E::MINBPC as isize)
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
        if HAS_CHAR!(enc, ptr, end) {
            if E::char_matches(ptr, ASCII_x) {
                return self.scanHexCharRef(ptr.offset(E::MINBPC as isize), end, nextTokPtr);
            }
            match E::byte_type(ptr) {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            ptr = ptr.offset(E::MINBPC as isize);
            while HAS_CHAR!(enc, ptr, end) {
                match E::byte_type(ptr) {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
                        return XML_TOK_CHAR_REF;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
                ptr = ptr.offset(E::MINBPC as isize)
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
        if !(end.wrapping_offset_from(ptr) as libc::c_long
            >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, E),
            match E::byte_type(ptr),
            19 => {
                return self.scanCharRef(
                    ptr.offset(E::MINBPC as isize),
                    end,
                    nextTokPtr,
                )
            }
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        while HAS_CHAR!(enc, ptr, end) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, E),
                match E::byte_type(ptr),
                18 => {
                    *nextTokPtr = ptr.offset(E::MINBPC as isize);
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
        while HAS_CHAR!(enc, ptr, end) {
            let mut current_block: Label = Label::None;
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, E),
                match E::byte_type(ptr),
                BT_COLON => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    hadColon = 1;
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                             (1 as libc::c_int * 1 as libc::c_int) as
                                 libc::c_long) {
                        return XML_TOK_PARTIAL
                    }
                    CHECK_NMSTRT_CASES! {
                        (ptr, end, nextTokPtr, E),
                        match E::byte_type(ptr),
                        _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                    }
                }
                BT_S | BT_CR | BT_LF => {
                    loop  {
                        let mut t: C2RustUnnamed_2 = 0;
                        ptr = ptr.offset(E::MINBPC as isize);
                        if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                                 (1 as libc::c_int * 1 as libc::c_int) as
                                     libc::c_long) {
                            return XML_TOK_PARTIAL
                        }
                        t = E::byte_type(ptr);
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
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !(end.wrapping_offset_from(ptr) as libc::c_long
                         >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    open = E::byte_type(ptr);
                    if open == BT_QUOT || open == BT_APOS {
                        break;
                    }
                    match open {
                        21 | 10 | 9 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                }
                ptr = ptr.offset(E::MINBPC as isize);
                /* in attribute value */
                loop {
                    let mut t: C2RustUnnamed_2 = 0;
                    if !(end.wrapping_offset_from(ptr) as libc::c_long
                         >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    t = E::byte_type(ptr);
                    if t == open {
                        break;
                    }
                    MATCH_INVALID_CASES! {
                        (ptr, end, nextTokPtr, E),
                        match t,
                        BT_AMP => {
                            let mut tok: libc::c_int =
                                self.scanRef(ptr.offset(E::MINBPC as isize),
                                             end, &mut ptr);
                            if tok <= 0 as libc::c_int {
                                if tok == XML_TOK_INVALID {
                                    *nextTokPtr = ptr
                                }
                                return tok
                            }
                        }
                        BT_LT => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                        _ => { ptr = ptr.offset(E::MINBPC as isize) }
                    }
                }
                ptr = ptr.offset(E::MINBPC as isize);
                if !(end.wrapping_offset_from(ptr) as libc::c_long
                     >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match E::byte_type(ptr) {
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
                            ptr = ptr.offset(E::MINBPC as isize);
                            if !(end.wrapping_offset_from(ptr) as libc::c_long
                                 >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
                            {
                                return XML_TOK_PARTIAL;
                            }
                            CHECK_NMSTRT_CASES! {
                                (ptr, end, nextTokPtr, E),
                                match E::byte_type(ptr),
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
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
                        return XML_TOK_START_TAG_WITH_ATTS;
                    }
                    Label::Sol => {
                        ptr = ptr.offset(E::MINBPC as isize);
                        if !(end.wrapping_offset_from(ptr) as libc::c_long
                             >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if !E::char_matches(ptr, ASCII_GT) {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
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
        if !(end.wrapping_offset_from(ptr) as libc::c_long
            >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, E),
            match E::byte_type(ptr),
            16 => {
                ptr = ptr.offset(E::MINBPC as isize);
                if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                         (1 as libc::c_int * 1 as libc::c_int) as
                             libc::c_long) {
                    return XML_TOK_PARTIAL
                }
                match E::byte_type(ptr) {
                    27 => {
                        return self.scanComment(ptr.offset(E::MINBPC as isize),
                                                end, nextTokPtr)
                    }
                    20 => {
                        return self.scanCdataSection(ptr.offset(E::MINBPC as isize),
                                                     end, nextTokPtr)
                    }
                    _ => { }
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID
            }
            15 => {
                return self.scanPi(ptr.offset(E::MINBPC as isize),
                                       end, nextTokPtr)
            }
            17 => {
                return self.scanEndTag(ptr.offset(E::MINBPC as isize),
                                       end, nextTokPtr)
            }
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        hadColon = 0 as libc::c_int;
        /* we have a start-tag */
        /* we have a start-tag */
        /* we have a start-tag */
        while HAS_CHAR!(enc, ptr, end) {
            let mut current_block_161: u64 = 0;
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, E),
                match E::byte_type(ptr),
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    hadColon = 1 as libc::c_int;
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                             (1 as libc::c_int * 1 as libc::c_int) as
                                 libc::c_long) {
                        return XML_TOK_PARTIAL
                    }
                    CHECK_NMSTRT_CASES! {
                        (ptr, end, nextTokPtr, E),
                        match E::byte_type(ptr),
                        _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                    }
                    current_block_161 = 12655303178690906525;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(E::MINBPC as isize);
                    loop  {
                        if !(HAS_CHAR!(enc, ptr, end)) {
                            current_block_161 = 13000670339742628194;
                            break ;
                        }
                        CHECK_NMSTRT_CASES! {
                            (ptr, end, nextTokPtr, E),
                            match E::byte_type(ptr),
                            11 => {
                                current_block_161 = 15370445274224965566;
                                break ;
                            }
                            17 => {
                                current_block_161 = 3926109038817298867;
                                break ;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(E::MINBPC as isize);
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
                11 => { current_block_161 = 15370445274224965566; }
                17 => { current_block_161 = 3926109038817298867; }
                _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
            }
            match current_block_161 {
                3926109038817298867 => {
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !(end.wrapping_offset_from(ptr) as libc::c_long
                        >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if !E::char_matches(ptr, ASCII_GT) {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(E::MINBPC as isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                }
                15370445274224965566 => {
                    *nextTokPtr = ptr.offset(E::MINBPC as isize);
                    return XML_TOK_START_TAG_NO_ATTS;
                }
                16331546839105579257 => ptr = ptr.offset(1 as libc::c_int as isize),
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
    unsafe extern "C" fn scanPercent(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if !(end.wrapping_offset_from(ptr) as libc::c_long
            >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, E),
            match E::byte_type(ptr),
            21 | 10 | 9 | 30 => { *nextTokPtr = ptr; return XML_TOK_PERCENT }
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        while HAS_CHAR!(enc, ptr, end) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, E),
                match E::byte_type(ptr),
                18 => {
                    *nextTokPtr = ptr.offset(E::MINBPC as isize);
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
        if !(end.wrapping_offset_from(ptr) as libc::c_long
            >= (1 as libc::c_int * 1 as libc::c_int) as libc::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        CHECK_NMSTRT_CASES! {
            (ptr, end, nextTokPtr, E),
            match E::byte_type(ptr),
            _ => { *nextTokPtr = ptr; return XML_TOK_INVALID }
        }
        while HAS_CHAR!(enc, ptr, end) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, E),
                match E::byte_type(ptr),
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
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
        while HAS_CHAR!(enc, ptr, end) {
            let mut t: C2RustUnnamed_2 = E::byte_type(ptr);
            MATCH_INVALID_CASES! {
                (ptr, end, nextTokPtr, E),
                match t,
                BT_QUOT | BT_APOS => {
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !(t != open) {
                        if !HAS_CHAR!(enc, ptr, end) {
                            return -XML_TOK_LITERAL
                        }
                        *nextTokPtr = ptr;
                        match E::byte_type(ptr) {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                return XML_TOK_LITERAL
                            }
                            _ => { return XML_TOK_INVALID }
                        }
                    }
                }
                _ => { ptr = ptr.offset(E::MINBPC as isize) }
            }
        }
        return XML_TOK_PARTIAL;
    }
}

impl<E: XmlEncodingImpl> XmlEncoding for XmlTokImpl<E> {
    unsafe extern "C" fn cdataSectionTok(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
        mut nextTokPtr: *mut *const libc::c_char,
    ) -> libc::c_int {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if E::MINBPC > 1 {
            let mut n: size_t = end.wrapping_offset_from(ptr) as libc::c_long as size_t;
            if n & (E::MINBPC - 1) as size_t != 0 {
                n &= !(E::MINBPC - 1) as size_t;
                if n == 0 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        MATCH_INVALID_CASES! {
            (ptr, end, nextTokPtr, E),
            match E::byte_type(ptr),
            4 => {
                ptr = ptr.offset(E::MINBPC as isize);
                if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                         (1 as libc::c_int * 1 as libc::c_int) as
                             libc::c_long) {
                    return XML_TOK_PARTIAL
                }
                if E::char_matches(ptr, ASCII_RSQB) {
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                             (1 as libc::c_int * 1 as libc::c_int) as
                                 libc::c_long) {
                        return XML_TOK_PARTIAL
                    }
                    if !E::char_matches(ptr, ASCII_GT) {
                        ptr = ptr.offset(-(E::MINBPC as isize))
                    } else {
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
                        return XML_TOK_CDATA_SECT_CLOSE
                    }
                }
            }
            9 => {
                ptr = ptr.offset(E::MINBPC as isize);
                if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                         (1 as libc::c_int * 1 as libc::c_int) as
                             libc::c_long) {
                    return XML_TOK_PARTIAL
                }
                if E::byte_type(ptr) == BT_LF {
                    ptr = ptr.offset(E::MINBPC as isize)
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE
            }
            10 => {
                *nextTokPtr = ptr.offset(E::MINBPC as isize);
                return XML_TOK_DATA_NEWLINE
            }
            _ => { ptr = ptr.offset(E::MINBPC as isize) }
        }
        while HAS_CHAR!(enc, ptr, end) {
            MATCH_LEAD_CASES! {
                E::byte_type(ptr),
                LEAD_CASE(n) => {
                    if end.wrapping_offset_from(ptr) < n || E::is_invalid_char(ptr, n) {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(n)
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                _ => { ptr = ptr.offset(E::MINBPC as isize) }
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
        if E::MINBPC > 1 {
            let mut n: size_t = end.wrapping_offset_from(ptr) as libc::c_long as size_t;
            if n & (E::MINBPC - 1) as size_t != 0 {
                n &= !(E::MINBPC - 1) as size_t;
                if n == 0 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        MATCH_INVALID_CASES! {
            (ptr, end, nextTokPtr, E),
            match E::byte_type(ptr),
            2 => {
                return self.scanLt(ptr.offset(E::MINBPC as isize),
                                       end, nextTokPtr)
            }
            3 => {
                return self.scanRef(ptr.offset(E::MINBPC as isize),
                                        end, nextTokPtr)
            }
            9 => {
                ptr = ptr.offset(E::MINBPC as isize);
                if !HAS_CHAR!(enc, ptr, end) {
                    return XML_TOK_TRAILING_CR
                }
                if E::byte_type(ptr) == BT_LF {
                    ptr = ptr.offset(E::MINBPC as isize)
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE
            }
            10 => {
                *nextTokPtr = ptr.offset(E::MINBPC as isize);
                return XML_TOK_DATA_NEWLINE
            }
            4 => {
                ptr = ptr.offset(E::MINBPC as isize);
                if !HAS_CHAR!(enc, ptr, end) {
                    return XML_TOK_TRAILING_RSQB
                }
                if !E::char_matches(ptr, ASCII_RSQB) {
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !HAS_CHAR!(enc, ptr, end) {
                        return XML_TOK_TRAILING_RSQB
                    }
                    if !E::char_matches(ptr, ASCII_GT) {
                        ptr = ptr.offset(-(E::MINBPC as isize))
                    } else { *nextTokPtr = ptr; return XML_TOK_INVALID }
                }
            }
            _ => { ptr = ptr.offset(E::MINBPC as isize) }
        }
        while HAS_CHAR!(enc, ptr, end) {
            let mut current_block_76: u64;
            MATCH_LEAD_CASES! {
                E::byte_type(ptr),
                LEAD_CASE(n) => {
                    if end.wrapping_offset_from(ptr) < n || E::is_invalid_char(ptr, n) {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(n);
                    current_block_76 = 10213293998891106930;
                }
                4 => {
                    if HAS_CHARS!(enc, ptr, end, 2) {
                        if !E::char_matches(ptr.offset(E::MINBPC), ASCII_RSQB) {
                            ptr = ptr.offset(E::MINBPC as isize);
                            current_block_76 = 10213293998891106930;
                        } else if HAS_CHARS!(enc, ptr, end, 3) {
                            if !E::char_matches(ptr.offset(2 * E::MINBPC), ASCII_GT) {
                                ptr = ptr.offset(E::MINBPC as isize)
                            } else {
                                *nextTokPtr =
                                    ptr.offset((2 * E::MINBPC) as isize);
                                return XML_TOK_INVALID
                            }
                            current_block_76 = 10213293998891106930;
                        } else { current_block_76 = 4244197895050895038; }
                    } else { current_block_76 = 4244197895050895038; }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    current_block_76 = 4244197895050895038;
                }
                _ => {
                    ptr = ptr.offset(E::MINBPC as isize);
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
        if E::MINBPC > 1 {
            let mut n: size_t = end.wrapping_offset_from(ptr) as libc::c_long as size_t;
            if n & (E::MINBPC - 1) as size_t != 0 {
                n &= !(E::MINBPC - 1) as size_t;
                if n == 0 {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize)
            }
        }
        let mut current_block_112: u64;
        MATCH_LEAD_CASES! {
            E::byte_type(ptr),
            LEAD_CASE(n) => {
                if (end.wrapping_offset_from(ptr) as c_long) < 2 {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if E::is_nmstrt_char(ptr, 2) {
                    ptr = ptr.offset(n);
                    tok = XML_TOK_NAME;
                } else if E::is_name_char(ptr, 2) {
                    ptr = ptr.offset(n);
                    tok = XML_TOK_NMTOKEN
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID
                }
                current_block_112 = 2222055338596505704;
            }
            12 => {
                return self.scanLit(BT_QUOT,
                                        ptr.offset(E::MINBPC as isize),
                                        end, nextTokPtr)
            }
            13 => {
                return self.scanLit(BT_APOS,
                                        ptr.offset(E::MINBPC as isize),
                                        end, nextTokPtr)
            }
            2 => {
                ptr = ptr.offset(E::MINBPC as isize);
                if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                         (1 as libc::c_int * 1 as libc::c_int) as
                             libc::c_long) {
                    return XML_TOK_PARTIAL
                }
                match E::byte_type(ptr) {
                    16 => {
                        return self.scanDecl(ptr.offset(E::MINBPC as
                                                                isize), end,
                                                 nextTokPtr)
                    }
                    15 => {
                        return self.scanPi(ptr.offset(E::MINBPC as
                                                              isize), end,
                                               nextTokPtr)
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(E::MINBPC as isize));
                        return XML_TOK_INSTANCE_START
                    }
                    _ => { }
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID
            }
            9 => {
                if ptr.offset(E::MINBPC as isize) == end {
                    *nextTokPtr = end;
                    /* indicate that this might be part of a CR/LF pair */
                    /* indicate that this might be part of a CR/LF pair */
                    /* indicate that this might be part of a CR/LF pair */
                    return -XML_TOK_PROLOG_S
                }
                current_block_112 = 1103933966285275534;
            }
            21 | 10 => { current_block_112 = 1103933966285275534; }
            30 => {
                return self.scanPercent(ptr.offset(E::MINBPC as isize),
                                            end, nextTokPtr)
            }
            35 => {
                *nextTokPtr = ptr.offset(E::MINBPC as isize);
                return XML_TOK_COMMA
            }
            20 => {
                *nextTokPtr = ptr.offset(E::MINBPC as isize);
                return XML_TOK_OPEN_BRACKET
            }
            4 => {
                ptr = ptr.offset(E::MINBPC as isize);
                if !HAS_CHAR!(enc, ptr, end) {
                    return -XML_TOK_CLOSE_BRACKET
                }
                if E::char_matches(ptr, ASCII_RSQB) {
                    if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                             (2 as libc::c_int * 1 as libc::c_int) as
                                 libc::c_long) {
                        return XML_TOK_PARTIAL
                    }
                    if E::char_matches(ptr.offset(E::MINBPC), ASCII_GT) {
                        *nextTokPtr =
                            ptr.offset((2 * E::MINBPC) as isize);
                        return XML_TOK_COND_SECT_CLOSE
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET
            }
            31 => {
                *nextTokPtr = ptr.offset(E::MINBPC as isize);
                return XML_TOK_OPEN_PAREN
            }
            32 => {
                ptr = ptr.offset(E::MINBPC as isize);
                if !HAS_CHAR!(enc, ptr, end) {
                    return -XML_TOK_CLOSE_PAREN
                }
                match E::byte_type(ptr) {
                    33 => {
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
                        return XML_TOK_CLOSE_PAREN_PLUS
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN
                    }
                    _ => { }
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID
            }
            36 => {
                *nextTokPtr = ptr.offset(E::MINBPC as isize);
                return XML_TOK_OR
            }
            11 => {
                *nextTokPtr = ptr.offset(E::MINBPC as isize);
                return XML_TOK_DECL_CLOSE
            }
            19 => {
                return self.scanPoundName(ptr.offset(E::MINBPC as
                                                             isize), end,
                                              nextTokPtr)
            }
            22 | 24 => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(E::MINBPC as isize);
                current_block_112 = 2222055338596505704;
            }
            25 | 26 | 27 | 23 => {
                tok = XML_TOK_NMTOKEN;
                ptr = ptr.offset(E::MINBPC as isize);
                current_block_112 = 2222055338596505704;
            }
            29 | _ => {
                /* fall through */
                /* fall through */
                /* fall through */
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
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !HAS_CHAR!(enc, ptr, end) {
                        break;
                    }
                    let mut current_block_32: u64;
                    match E::byte_type(ptr) {
                        21 | 10 => {
                            current_block_32 = 14072441030219150333;
                        }
                        9 => {
                            /* don't split CR/LF pair */
                            /* don't split CR/LF pair */
                            /* don't split CR/LF pair */
                            if ptr.offset(E::MINBPC as isize) != end {
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
        while HAS_CHAR!(enc, ptr, end) {
            CHECK_NAME_CASES! {
                (ptr, end, nextTokPtr, E),
                match E::byte_type(ptr),
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok
                }
                23 => {
                    ptr = ptr.offset(E::MINBPC as isize);
                    match tok {
                        XML_TOK_NAME => {
                            if !(end.wrapping_offset_from(ptr) as libc::c_long
                                     >=
                                     (1 as libc::c_int * 1 as libc::c_int) as
                                         libc::c_long) {
                                return XML_TOK_PARTIAL
                            }
                            tok = XML_TOK_PREFIXED_NAME;
                            CHECK_NAME_CASES! {
                                (ptr, end, nextTokPtr, E),
                                match E::byte_type(ptr),
                                _ => {
                                    tok = XML_TOK_NMTOKEN;
                                }
                            }
                        }
                        XML_TOK_PREFIXED_NAME => { tok = XML_TOK_NMTOKEN }
                        _ => { }
                    }
                }
                34 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    *nextTokPtr = ptr.offset(E::MINBPC as isize);
                    return XML_TOK_NAME_PLUS
                }
                33 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    *nextTokPtr = ptr.offset(E::MINBPC as isize);
                    return XML_TOK_NAME_ASTERISK
                }
                15 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID
                    }
                    *nextTokPtr = ptr.offset(E::MINBPC as isize);
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
            if !HAS_CHAR!(enc, ptr, end) {
                return XML_TOK_PARTIAL;
            }
        }
        start = ptr;
        while HAS_CHAR!(enc, ptr, end) {
            MATCH_LEAD_CASES! {
                E::byte_type(ptr),
                LEAD_CASE(n) => {
                    ptr = ptr.offset(n);
                }
                3 => {
                    if ptr == start {
                        return self.scanRef(ptr.offset(E::MINBPC as
                                                               isize), end,
                                                nextTokPtr)
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                2 => { *nextTokPtr = ptr; return XML_TOK_INVALID }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(E::MINBPC as isize);
                        if !HAS_CHAR!(enc, ptr, end) {
                            return XML_TOK_TRAILING_CR
                        }
                        if E::byte_type(ptr) == BT_LF {
                            ptr = ptr.offset(E::MINBPC as isize)
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                _ => { ptr = ptr.offset(E::MINBPC as isize) }
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
            if !HAS_CHAR!(enc, ptr, end) {
                return XML_TOK_PARTIAL;
            }
        }
        start = ptr;
        while HAS_CHAR!(enc, ptr, end) {
            MATCH_LEAD_CASES! {
                E::byte_type(ptr),
                LEAD_CASE(n) => {
                    ptr = ptr.offset(n);
                }
                3 => {
                    if ptr == start {
                        return self.scanRef(ptr.offset(E::MINBPC as
                                                               isize), end,
                                                nextTokPtr)
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                30 => {
                    if ptr == start {
                        let mut tok: libc::c_int =
                            self.scanPercent(ptr.offset(E::MINBPC as
                                                                isize), end,
                                                 nextTokPtr);
                        return if tok == XML_TOK_PERCENT {
                                   XML_TOK_INVALID
                               } else { tok }
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(E::MINBPC as isize);
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(E::MINBPC as isize);
                        if !HAS_CHAR!(enc, ptr, end) {
                            return XML_TOK_TRAILING_CR
                        }
                        if E::byte_type(ptr) == BT_LF {
                            ptr = ptr.offset(E::MINBPC as isize)
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS
                }
                _ => { ptr = ptr.offset(E::MINBPC as isize) }
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
        if E::MINBPC > 1 {
            let mut n: size_t = end.wrapping_offset_from(ptr) as libc::c_long as size_t;
            if n & (E::MINBPC - 1) as size_t != 0 {
                n &= !(E::MINBPC - 1) as size_t;
                end = ptr.offset(n as isize)
            }
        }
        while HAS_CHAR!(enc, ptr, end) {
            MATCH_INVALID_CASES! {
                (ptr, end, nextTokPtr, E),
                match E::byte_type(ptr),
                2 => {
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                             (1 as libc::c_int * 1 as libc::c_int) as
                                 libc::c_long) {
                        return XML_TOK_PARTIAL
                    }
                    if E::char_matches(ptr, ASCII_EXCL) {
                        ptr = ptr.offset(E::MINBPC as isize);
                        if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                                 (1 as libc::c_int * 1 as libc::c_int) as
                                     libc::c_long) {
                            return XML_TOK_PARTIAL
                        }
                        if E::char_matches(ptr, ASCII_LSQB) {
                            level += 1;
                            ptr = ptr.offset(E::MINBPC as isize)
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(E::MINBPC as isize);
                    if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                             (1 as libc::c_int * 1 as libc::c_int) as
                                 libc::c_long) {
                        return XML_TOK_PARTIAL
                    }
                    if E::char_matches(ptr, ASCII_RSQB) {
                        ptr = ptr.offset(E::MINBPC as isize);
                        if !(end.wrapping_offset_from(ptr) as libc::c_long >=
                                 (1 as libc::c_int * 1 as libc::c_int) as
                                     libc::c_long) {
                            return XML_TOK_PARTIAL
                        }
                        if E::char_matches(ptr, ASCII_GT) {
                            ptr = ptr.offset(E::MINBPC as isize);
                            if level == 0 as libc::c_int {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT
                            }
                            level -= 1
                        }
                    }
                }
                _ => { ptr = ptr.offset(E::MINBPC as isize) }
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
        ptr = ptr.offset(E::MINBPC as isize);
        end = end.offset(-(E::MINBPC as isize));
        while HAS_CHAR!(enc, ptr, end) {
            let mut current_block_8: u64;
            match E::byte_type(ptr) {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    current_block_8 = 13242334135786603907;
                }
                21 => {
                    if E::char_matches(ptr, ASCII_TAB) {
                        *badPtr = ptr;
                        return 0 as libc::c_int;
                    }
                    current_block_8 = 13242334135786603907;
                }
                26 | 22 => {
                    if E::byte_to_ascii(ptr) & !(0x7f as c_char) == 0 {
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
                    match E::byte_to_ascii(ptr) {
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
            ptr = ptr.offset(E::MINBPC as isize)
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
        ptr = ptr.offset(E::MINBPC as isize);
        loop {
            MATCH_LEAD_CASES! {
                   E::byte_type(ptr),
                   LEAD_CASE(n) => {
                       START_NAME!{}
                       ptr = ptr.offset((n - E::MINBPC) as isize)
                   }
                   29 | 22 | 24 => {
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
                   12 => {
                       if state as libc::c_uint !=
                              inValue as libc::c_int as libc::c_uint {
                           if nAtts < attsMax {
                               let ref mut fresh127 =
                                   (*atts.offset(nAtts as isize)).valuePtr;
                               *fresh127 = ptr.offset(E::MINBPC as isize)
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
                   13 => {
                       if state as libc::c_uint !=
                              inValue as libc::c_int as libc::c_uint {
                           if nAtts < attsMax {
                               let ref mut fresh129 =
                                   (*atts.offset(nAtts as isize)).valuePtr;
                               *fresh129 = ptr.offset(E::MINBPC as isize)
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
                   3 => {
                       if nAtts < attsMax {
                           (*atts.offset(nAtts as isize)).normalized =
                               0 as libc::c_int as libc::c_char
                       }
                   }
                   21 => {
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
                                          E::byte_to_ascii(ptr) != ASCII_SPACE
                                          ||
                                          E::byte_to_ascii(ptr.offset(E::MINBPC))
                                              == ASCII_SPACE ||
                                          E::byte_type(ptr.offset(E::MINBPC)) ==
                                              open) {
                           (*atts.offset(nAtts as isize)).normalized =
                               0 as libc::c_int as libc::c_char
                       }
                   }
                   9 | 10 => {
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
                   11 | 17 => {
                       if state as libc::c_uint !=
                              inValue as libc::c_int as libc::c_uint {
                           return nAtts
                       }
                   }
                   _ => { }
               }
            ptr = ptr.offset(E::MINBPC as isize)
        }
        /* not reached */
        /* not reached */
        /* not reached */
    }
    unsafe extern "C" fn charRefNumber(&self, mut ptr: *const libc::c_char) -> libc::c_int {
        let mut result: libc::c_int = 0 as libc::c_int;
        ptr = ptr.offset((2 * E::MINBPC) as isize);
        if E::char_matches(ptr, ASCII_x) {
            ptr = ptr.offset(E::MINBPC as isize);
            while !E::char_matches(ptr, ASCII_SEMI) {
                let mut c: c_char = E::byte_to_ascii(ptr);
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
                ptr = ptr.offset(E::MINBPC as isize)
            }
        } else {
            while !E::char_matches(ptr, ASCII_SEMI) {
                let mut c_0: c_char = E::byte_to_ascii(ptr);
                result *= 10 as libc::c_int;
                result += (c_0 - ASCII_0) as c_int;
                if result >= 0x110000 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                ptr = ptr.offset(E::MINBPC as isize)
            }
        }
        return checkCharRefNumber(result);
    }
    unsafe extern "C" fn predefinedEntityName(
        &self,
        mut ptr: *const libc::c_char,
        mut end: *const libc::c_char,
    ) -> libc::c_int {
        match end.wrapping_offset_from(ptr) / E::MINBPC {
            2 => {
                if E::char_matches(ptr.offset(E::MINBPC), ASCII_t) {
                    match E::byte_to_ascii(ptr) {
                        ASCII_l => return ASCII_LT as c_int,
                        ASCII_g => return ASCII_GT as c_int,
                        _ => {}
                    }
                }
            }
            3 => {
                if E::char_matches(ptr, ASCII_a) {
                    ptr = ptr.offset(E::MINBPC as isize);
                    if E::char_matches(ptr, ASCII_m) {
                        ptr = ptr.offset(E::MINBPC as isize);
                        if E::char_matches(ptr, ASCII_p) {
                            return ASCII_AMP as c_int;
                        }
                    }
                }
            }
            4 => match E::byte_to_ascii(ptr) {
                ASCII_q => {
                    ptr = ptr.offset(E::MINBPC as isize);
                    if E::char_matches(ptr, ASCII_u) {
                        ptr = ptr.offset(E::MINBPC as isize);
                        if E::char_matches(ptr, ASCII_o) {
                            ptr = ptr.offset(E::MINBPC as isize);
                            if E::char_matches(ptr, ASCII_t) {
                                return ASCII_QUOT as c_int;
                            }
                        }
                    }
                }
                ASCII_a => {
                    ptr = ptr.offset(E::MINBPC as isize);
                    if E::char_matches(ptr, ASCII_p) {
                        ptr = ptr.offset(E::MINBPC as isize);
                        if E::char_matches(ptr, ASCII_o) {
                            ptr = ptr.offset(E::MINBPC as isize);
                            if E::char_matches(ptr, ASCII_s) {
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
            if end1.wrapping_offset_from(ptr1) < E::MINBPC {
                return 0 as libc::c_int;
            }
            if !E::char_matches(ptr1, *ptr2) {
                return 0 as libc::c_int;
            }
            ptr1 = ptr1.offset(E::MINBPC as isize);
            ptr2 = ptr2.offset(1)
        }
        return (ptr1 == end1) as libc::c_int;
    }
    unsafe extern "C" fn nameLength(&self, mut ptr: *const libc::c_char) -> libc::c_int {
        let mut start: *const libc::c_char = ptr;
        loop {
            MATCH_LEAD_CASES! {
                E::byte_type(ptr),
                LEAD_CASE(n) => {
                    ptr = ptr.offset(n);
                }
                29 | 22 | 23 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.offset(E::MINBPC as isize)
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
            match E::byte_type(ptr) {
                10 | 9 | 21 => ptr = ptr.offset(E::MINBPC as isize),
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
        while HAS_CHAR!(enc, ptr, end) {
            MATCH_LEAD_CASES! {
                E::byte_type(ptr),
                LEAD_CASE(n) => {
                    ptr = ptr.offset(n);
                }
                10 => {
                    (*pos).columnNumber = -(1 as libc::c_int) as XML_Size;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(E::MINBPC as isize)
                }
                9 => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(E::MINBPC as isize);
                    if HAS_CHAR!(enc, ptr, end) &&
                           E::byte_type(ptr) == BT_LF {
                        ptr = ptr.offset(E::MINBPC as isize)
                    }
                    (*pos).columnNumber = -(1 as libc::c_int) as XML_Size
                }
                _ => { ptr = ptr.offset(E::MINBPC as isize) }
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
        E::utf8Convert(fromP, fromLim, toP, toLim)
    }

    unsafe extern "C" fn utf16Convert(
        &self,
        fromP: *mut *const c_char,
        fromLim: *const c_char,
        toP: *mut *mut c_ushort,
        toLim: *const c_ushort,
    ) -> XML_Convert_Result {
        E::utf16Convert(fromP, fromLim, toP, toLim)
    }

    fn minBytesPerChar(&self) -> c_int {
        E::MINBPC as c_int
    }

    fn isUtf8(&self) -> bool {
        E::isUtf8
    }

    fn isUtf16(&self) -> bool {
        E::isUtf16
    }
}
