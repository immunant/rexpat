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
                let $n: usize = 2;
                $case
            }
            ByteType::LEAD3 => {
                let $n: usize = 3;
                $case
            }
            ByteType::LEAD4 => {
                let $n: usize = 4;
                $case
            }
            $($tail)*
        }
    };
}

macro_rules! MATCH_INVALID_CASES {
    {
        ($buf:ident, $idx:ident, $nextTokIdx:ident, $self:ident),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $buf[$idx..].len() < n {
                    return XML_TOK::PARTIAL_CHAR;
                }
                if $self.is_invalid_char(&$buf[$idx..], n) {
                    *$nextTokIdx = $idx;
                    return XML_TOK::INVALID;
                }
                $idx += n;
            }
            ByteType::NONXML | ByteType::MALFORM | ByteType::TRAIL => {
                *$nextTokIdx = $idx;

                return XML_TOK::INVALID;
            }
            $($tail)*
        }
    }
}

macro_rules! CHECK_NAME_CASES {
    {
        ($buf:ident, $idx:ident, $nextTokIdx:ident, $self:ident),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $buf[$idx..].len() < n {
                    return XML_TOK::PARTIAL_CHAR;
                }
                if !$self.is_name_char(&$buf[$idx..], n) {
                    *$nextTokIdx = $idx;
                    return XML_TOK::INVALID;
                }
                $idx += n;
            }
            ByteType::NONASCII if !$self.is_name_char_minbpc(&$buf[$idx..]) => {
                *$nextTokIdx = $idx;
                return XML_TOK::INVALID;
            }
            ByteType::NONASCII | ByteType::NMSTRT | ByteType::HEX | ByteType::DIGIT | ByteType::NAME | ByteType::MINUS => {
                $idx += $self.MINBPC();
            }
            $($tail)*
        }
    }
}

macro_rules! CHECK_NMSTRT_CASES {
    {
        ($buf:ident, $idx:ident, $nextTokIdx:ident, $self:ident),
        match $e:expr,
        $($tail:tt)*
    } => {
        MATCH_LEAD_CASES! {
            $e,
            LEAD_CASE(n) => {
                if $buf[$idx..].len() < n {
                    return XML_TOK::PARTIAL_CHAR;
                }
                if !$self.is_nmstrt_char(&$buf[$idx..], n) {
                    *$nextTokIdx = $idx;
                    return XML_TOK::INVALID;
                }
                $idx += n;
            }
            ByteType::NONASCII if !$self.is_nmstrt_char_minbpc(&$buf[$idx..]) => {
                *$nextTokIdx = $idx;
                return XML_TOK::INVALID;
            }
            ByteType::NONASCII | ByteType::NMSTRT | ByteType::HEX => {
                $idx += $self.MINBPC();
            }
            $($tail)*
        }
    }
}


macro_rules! HAS_CHARS {
    ($buf:ident, $idx:ident, $count:expr, $self:ident) => {
        $buf[$idx..].len() as c_long >= ($self.MINBPC() * $count) as c_long
    };
}

macro_rules! HAS_CHAR {
    ($buf:ident, $idx:ident, $self:ident) => {
        HAS_CHARS!($buf, $idx, 1, $self)
    };
}

macro_rules! REQUIRE_CHARS {
    ($buf:ident, $idx:ident, $count:expr, $self:ident) => {
        if !HAS_CHARS!($buf, $idx, $count, $self) {
            return XML_TOK::PARTIAL;
        }
    };
}

macro_rules! REQUIRE_CHAR {
    ($buf:ident, $idx:ident, $self:ident) => {
        REQUIRE_CHARS!($buf, $idx, 1, $self)
    };
}


pub trait XmlTokImpl: XmlEncodingImpl {
    /// Helper function that calls a given method and updates `nextTokIdx`
    /// correctly.
    #[inline]
    fn call_rec<F>(
        &self,
        f: F,
        buf: ExpatBufRef,
        idx: usize,
        nextTokIdx: &mut usize,
    ) -> XML_TOK
    where
        F: FnOnce(&Self, ExpatBufRef, &mut usize) -> XML_TOK,
    {
        let mut next_idx = usize::MAX;
        let result = f(self, buf.inc_start(idx), &mut next_idx);
        if next_idx != usize::MAX {
            *nextTokIdx = idx + next_idx;
        }
        result
    }

    /* ptr points to character following "<!-" */
    fn scanComment(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        if HAS_CHAR!(buf, idx, self) {
            if !self.char_matches(&buf[idx..], ASCII_MINUS) {
                *nextTokIdx = idx;
                return XML_TOK::INVALID;
            }
            idx += self.MINBPC();
            while HAS_CHAR!(buf, idx, self) {
                MATCH_INVALID_CASES! {
                    (buf, idx, nextTokIdx, self),
                    match self.byte_type(&buf[idx..]),
                    ByteType::MINUS => {
                        idx += self.MINBPC();
                        REQUIRE_CHAR!(buf, idx, self);
                        if self.char_matches(&buf[idx..], ASCII_MINUS) {
                            idx += self.MINBPC();
                            REQUIRE_CHAR!(buf, idx, self);
                            if !self.char_matches(&buf[idx..], ASCII_GT) {
                                *nextTokIdx = idx;
                                return XML_TOK::INVALID;
                            }
                            *nextTokIdx = idx + self.MINBPC();
                            return XML_TOK::COMMENT;
                        }
                    }
                    _ => { idx += self.MINBPC(); }
                }
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following "<!" */
    fn scanDecl(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        REQUIRE_CHAR!(buf, idx, self);
        match self.byte_type(&buf[idx..]) {
            ByteType::MINUS => {
                return self.call_rec(Self::scanComment, buf, idx + self.MINBPC(), nextTokIdx);
            }
            ByteType::LSQB => {
                *nextTokIdx = idx + self.MINBPC();
                return XML_TOK::COND_SECT_OPEN;
            }
            ByteType::NMSTRT | ByteType::HEX => idx += self.MINBPC(),
            _ => {
                *nextTokIdx = idx;
                return XML_TOK::INVALID;
            }
        }
        while HAS_CHAR!(buf, idx, self) {
            match self.byte_type(&buf[idx..]) {
                ByteType::PERCNT => {
                    REQUIRE_CHARS!(buf, idx, 2, self);
                    /* don't allow <!ENTITY% foo "whatever"> */
                    match self.byte_type(&buf[idx + self.MINBPC()..]) {
                        ByteType::S | ByteType::CR | ByteType::LF | ByteType::PERCNT => {
                            *nextTokIdx = idx;
                            return XML_TOK::INVALID;
                        }
                        _ => {}
                    }
                }
                ByteType::S | ByteType::CR | ByteType::LF => {}
                ByteType::NMSTRT | ByteType::HEX => {
                    idx += self.MINBPC();
                    continue;
                }
                _ => {
                    *nextTokIdx = idx;
                    return XML_TOK::INVALID;
                }
            }
            /* fall through */
            *nextTokIdx = idx;
            return XML_TOK::DECL_OPEN;
        }
        XML_TOK::PARTIAL
    }
    fn checkPiTarget(
        &self,
        buf: ExpatBufRef,
        mut tokPtr: &mut XML_TOK,
    ) -> libc::c_int {
        let mut idx = 0;
        let mut upper: libc::c_int = 0 as libc::c_int;
        *tokPtr = XML_TOK::PI;
        if buf[idx..].len() as libc::c_long != (self.MINBPC() * 3) as libc::c_long {
            return 1 as libc::c_int;
        }
        match self.byte_to_ascii(&buf[idx..]) {
            ASCII_x => {}
            ASCII_X => upper = 1 as libc::c_int,
            _ => return 1 as libc::c_int,
        }
        idx += self.MINBPC();
        match self.byte_to_ascii(&buf[idx..]) {
            ASCII_m => {}
            ASCII_M => upper = 1 as libc::c_int,
            _ => return 1 as libc::c_int,
        }
        idx += self.MINBPC();
        match self.byte_to_ascii(&buf[idx..]) {
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
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        let mut tok = XML_TOK::INVALID;
        let target = buf.clone();
        REQUIRE_CHAR!(buf, idx, self);
        CHECK_NMSTRT_CASES! {
            (buf, idx, nextTokIdx, self),
            match self.byte_type(&buf[idx..]),
            _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
        }
        while HAS_CHAR!(buf, idx, self) {
            CHECK_NAME_CASES! {
                (buf, idx, nextTokIdx, self),
                match self.byte_type(&buf[idx..]),
                ByteType::S | ByteType::CR | ByteType::LF => {
                    if self.checkPiTarget(target.with_end(buf[idx..].as_ptr()), &mut tok) == 0
                    {
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID;
                    }
                    idx += self.MINBPC();
                    while HAS_CHAR!(buf, idx, self) {
                        MATCH_INVALID_CASES! {
                            (buf, idx, nextTokIdx, self),
                            match self.byte_type(&buf[idx..]),
                            ByteType::QUEST => {
                                idx += self.MINBPC();
                                REQUIRE_CHAR!(buf, idx, self);
                                if self.char_matches(&buf[idx..], ASCII_GT) {
                                    *nextTokIdx = idx + self.MINBPC();
                                    return tok;
                                }
                            }
                            _ => { idx += self.MINBPC() }
                        }
                    }
                    return XML_TOK::PARTIAL
                }
                ByteType::QUEST => {
                    if self.checkPiTarget(target.with_end(buf[idx..].as_ptr()), &mut tok) == 0
                    {
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID
                    }
                    idx += self.MINBPC();
                    REQUIRE_CHAR!(buf, idx, self);
                    if self.char_matches(&buf[idx..], ASCII_GT) {
                        *nextTokIdx = idx + self.MINBPC();
                        return tok;
                    }
                    *nextTokIdx = idx;
                    return XML_TOK::INVALID;
                }
                _ => {
                    *nextTokIdx = idx;
                    return XML_TOK::INVALID;
                }
            }
        }
        XML_TOK::PARTIAL
    }
    fn scanCdataSection(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
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
        REQUIRE_CHARS!(buf, idx, 6, self);
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            if !self.char_matches(&buf[idx..], CDATA_LSQB[i as usize]) {
                *nextTokIdx = idx;
                return XML_TOK::INVALID;
            }
            i += 1;
            idx += self.MINBPC();
        }
        *nextTokIdx = idx;
        XML_TOK::CDATA_SECT_OPEN
    }

    /* ptr points to character following "</" */
    fn scanEndTag(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        REQUIRE_CHAR!(buf, idx, self);
        CHECK_NMSTRT_CASES! {
            (buf, idx, nextTokIdx, self),
            match self.byte_type(&buf[idx..]),
            _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
        }
        while HAS_CHAR!(buf, idx, self) {
            CHECK_NAME_CASES! {
                (buf, idx, nextTokIdx, self),
                match self.byte_type(&buf[idx..]),
                ByteType::S | ByteType::CR | ByteType::LF => {
                    idx += self.MINBPC();
                    while HAS_CHAR!(buf, idx, self) {
                        match self.byte_type(&buf[idx..]) {
                            ByteType::S | ByteType::CR | ByteType::LF => { }
                            ByteType::GT => {
                                *nextTokIdx = idx + self.MINBPC();
                                return XML_TOK::END_TAG;
                            }
                            _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
                        }
                        idx += self.MINBPC();
                    }
                    return XML_TOK::PARTIAL;
                }
                ByteType::COLON => {
                    /* no need to check qname syntax here,
                    since end-tag must match exactly */
                    idx += self.MINBPC();
                }
                ByteType::GT => {
                    *nextTokIdx = idx + self.MINBPC();
                    return XML_TOK::END_TAG
                }
                _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following "&#X" */
    fn scanHexCharRef(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        if HAS_CHAR!(buf, idx, self) {
            match self.byte_type(&buf[idx..]) {
                ByteType::DIGIT | ByteType::HEX => {}
                _ => {
                    *nextTokIdx = idx;
                    return XML_TOK::INVALID;
                }
            }
            idx += self.MINBPC();
            while HAS_CHAR!(buf, idx, self) {
                match self.byte_type(&buf[idx..]) {
                    ByteType::DIGIT | ByteType::HEX => {}
                    ByteType::SEMI => {
                        *nextTokIdx = idx + self.MINBPC();
                        return XML_TOK::CHAR_REF;
                    }
                    _ => {
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID;
                    }
                }
                idx += self.MINBPC();
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following "&#" */
    fn scanCharRef(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        if HAS_CHAR!(buf, idx, self) {
            if self.char_matches(&buf[idx..], ASCII_x) {
                return self.call_rec(Self::scanHexCharRef, buf, idx + self.MINBPC(), nextTokIdx);
            }
            match self.byte_type(&buf[idx..]) {
                ByteType::DIGIT => {}
                _ => {
                    *nextTokIdx = idx;
                    return XML_TOK::INVALID;
                }
            }
            idx += self.MINBPC();
            while HAS_CHAR!(buf, idx, self) {
                match self.byte_type(&buf[idx..]) {
                    ByteType::DIGIT => {}
                    ByteType::SEMI => {
                        *nextTokIdx = idx + self.MINBPC();
                        return XML_TOK::CHAR_REF;
                    }
                    _ => {
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID;
                    }
                }
                idx += self.MINBPC()
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following "&" */
    fn scanRef(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        REQUIRE_CHAR!(buf, idx, self);
        CHECK_NMSTRT_CASES! {
            (buf, idx, nextTokIdx, self),
            match self.byte_type(&buf[idx..]),
            ByteType::NUM => {
                return self.call_rec(
                    Self::scanCharRef,
                    buf,
                    idx + self.MINBPC(),
                    nextTokIdx,
                );
            }
            _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
        }
        while HAS_CHAR!(buf, idx, self) {
            CHECK_NAME_CASES! {
                (buf, idx, nextTokIdx, self),
                match self.byte_type(&buf[idx..]),
                ByteType::SEMI => {
                    *nextTokIdx = idx + self.MINBPC();
                    return XML_TOK::ENTITY_REF;
                }
                _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following first character of attribute name */
    fn scanAtts(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        #[derive(PartialEq)]
        enum Label {
            None,
            Sol,
            Gt,
            EqFallthrough,
        }

        let mut idx = 0;
        let mut hadColon = false;
        while HAS_CHAR!(buf, idx, self) {
            let mut current_block: Label = Label::None;
            CHECK_NAME_CASES! {
                (buf, idx, nextTokIdx, self),
                match self.byte_type(&buf[idx..]),
                ByteType::COLON => {
                    if hadColon {
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID;
                    }
                    hadColon = true;
                    idx += self.MINBPC();
                    REQUIRE_CHAR!(buf, idx, self);
                    CHECK_NMSTRT_CASES! {
                        (buf, idx, nextTokIdx, self),
                        match self.byte_type(&buf[idx..]),
                        _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
                    }
                }
                ByteType::S | ByteType::CR | ByteType::LF => {
                    loop  {
                        idx += self.MINBPC();
                        REQUIRE_CHAR!(buf, idx, self);
                        let mut t = self.byte_type(&buf[idx..]);
                        if t == ByteType::EQUALS { break; }
                        match t {
                            ByteType::S | ByteType::LF | ByteType::CR => { }
                            _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
                        }
                    }
                    current_block = Label::EqFallthrough;
                }
                ByteType::EQUALS => { current_block = Label::EqFallthrough; }
                _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
            }
            if current_block == Label::EqFallthrough {
                /* fall through */
                // ByteType::S | ByteType::CR | ByteType::LF | ByteType::EQUALS =>
                let mut open = ByteType::NONXML;
                hadColon = false;
                loop {
                    idx += self.MINBPC();
                    REQUIRE_CHAR!(buf, idx, self);
                    open = self.byte_type(&buf[idx..]);
                    if open == ByteType::QUOT || open == ByteType::APOS {
                        break;
                    }
                    match open {
                        ByteType::S | ByteType::LF | ByteType::CR => {}
                        _ => {
                            *nextTokIdx = idx;
                            return XML_TOK::INVALID;
                        }
                    }
                }
                idx += self.MINBPC();
                /* in attribute value */
                loop {
                    REQUIRE_CHAR!(buf, idx, self);
                    let mut t = self.byte_type(&buf[idx..]);
                    if t == open {
                        break;
                    }
                    MATCH_INVALID_CASES! {
                        (buf, idx, nextTokIdx, self),
                        match t,
                        ByteType::AMP => {
                            let mut next_idx = 0;
                            let mut tok = self.scanRef(buf.inc_start(idx + self.MINBPC()), &mut next_idx);
                            idx += self.MINBPC() + next_idx;
                            if tok.is_error() {
                                if tok == XML_TOK::INVALID {
                                    *nextTokIdx = idx;
                                }
                                return tok;
                            }
                        }
                        ByteType::LT => { *nextTokIdx = idx; return XML_TOK::INVALID }
                        _ => { idx += self.MINBPC() }
                    }
                }
                idx += self.MINBPC();
                REQUIRE_CHAR!(buf, idx, self);
                match self.byte_type(&buf[idx..]) {
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
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID;
                    }
                }
                match current_block {
                    Label::Sol | Label::Gt => { }
                    _ => {
                        // Didn't take goto
                        /* ptr points to closing quote */
                        loop {
                            idx += self.MINBPC();
                            REQUIRE_CHAR!(buf, idx, self);
                            CHECK_NMSTRT_CASES! {
                                (buf, idx, nextTokIdx, self),
                                match self.byte_type(&buf[idx..]),
                                ByteType::S | ByteType::CR | ByteType::LF => {
                                    continue;
                                }
                                ByteType::GT => {
                                    current_block = Label::Gt;
                                    break;
                                }
                                ByteType::SOL => {
                                    current_block = Label::Sol;
                                    break;
                                }
                                _ => {
                                    *nextTokIdx = idx;
                                    return XML_TOK::INVALID
                                }
                            }
                            break;
                        }
                    }
                }

                match current_block {
                    Label::Gt => {
                        *nextTokIdx = idx + self.MINBPC();
                        return XML_TOK::START_TAG_WITH_ATTS;
                    }
                    Label::Sol => {
                        idx += self.MINBPC();
                        REQUIRE_CHAR!(buf, idx, self);
                        if !self.char_matches(&buf[idx..], ASCII_GT) {
                            *nextTokIdx = idx;
                            return XML_TOK::INVALID;
                        }
                        *nextTokIdx = idx + self.MINBPC();
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
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = self.MINBPC();
        REQUIRE_CHAR!(buf, idx, self);
        if !self.char_matches(&buf[idx..], ASCII_GT) {
            *nextTokIdx = idx;
            return XML_TOK::INVALID;
        }
        *nextTokIdx = idx + self.MINBPC();
        XML_TOK::EMPTY_ELEMENT_NO_ATTS
    }

    /* ptr points to character following "<" */
    fn scanLt(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        REQUIRE_CHAR!(buf, idx, self);
        CHECK_NMSTRT_CASES! {
            (buf, idx, nextTokIdx, self),
            match self.byte_type(&buf[idx..]),
            ByteType::EXCL => {
                idx += self.MINBPC();
                REQUIRE_CHAR!(buf, idx, self);
                match self.byte_type(&buf[idx..]) {
                    ByteType::MINUS => {
                        return self.call_rec(Self::scanComment, buf, idx + self.MINBPC(), nextTokIdx);
                    }
                    ByteType::LSQB => {
                        return self.call_rec(Self::scanCdataSection, buf, idx + self.MINBPC(), nextTokIdx);
                    }
                    _ => { }
                }
                *nextTokIdx = idx;
                return XML_TOK::INVALID;
            }
            ByteType::QUEST => {
                return self.call_rec(Self::scanPi, buf, idx + self.MINBPC(), nextTokIdx);
            }
            ByteType::SOL => {
                return self.call_rec(Self::scanEndTag, buf, idx + self.MINBPC(), nextTokIdx);
            }
            _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
        }
        let mut hadColon = false;
        /* we have a start-tag */
        while HAS_CHAR!(buf, idx, self) {
            CHECK_NAME_CASES! {
                (buf, idx, nextTokIdx, self),
                match self.byte_type(&buf[idx..]),
                ByteType::COLON => {
                    if hadColon {
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID;
                    }
                    hadColon = true;
                    idx += self.MINBPC();
                    REQUIRE_CHAR!(buf, idx, self);
                    CHECK_NMSTRT_CASES! {
                        (buf, idx, nextTokIdx, self),
                        match self.byte_type(&buf[idx..]),
                        _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
                    }
                }
                ByteType::S | ByteType::CR | ByteType::LF => {
                    idx += self.MINBPC();
                    while HAS_CHAR!(buf, idx, self) {
                        CHECK_NMSTRT_CASES! {
                            (buf, idx, nextTokIdx, self),
                            match self.byte_type(&buf[idx..]),
                            ByteType::GT => {
                                *nextTokIdx = idx + self.MINBPC();
                                return XML_TOK::START_TAG_NO_ATTS;
                            }
                            ByteType::SOL => {
                                return self.call_rec(Self::scanLtHelper, buf, idx, nextTokIdx);
                            }
                            ByteType::S | ByteType::CR | ByteType::LF => {
                                idx += self.MINBPC();
                                continue;
                            }
                            _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
                        }
                        return self.call_rec(Self::scanAtts, buf, idx, nextTokIdx);
                    }
                    return XML_TOK::PARTIAL;
                }
                ByteType::GT => {
                    *nextTokIdx = idx + self.MINBPC();
                    return XML_TOK::START_TAG_NO_ATTS;
                 }
                ByteType::SOL => { return self.call_rec(Self::scanLtHelper, buf, idx, nextTokIdx); }
                _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
            }
        }
        XML_TOK::PARTIAL
    }

    /* ptr points to character following "%" */
    fn scanPercent(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        REQUIRE_CHAR!(buf, idx, self);
        CHECK_NMSTRT_CASES! {
            (buf, idx, nextTokIdx, self),
            match self.byte_type(&buf[idx..]),
            ByteType::S | ByteType::LF | ByteType::CR | ByteType::PERCNT => { *nextTokIdx = idx; return XML_TOK::PERCENT }
            _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
        }
        while HAS_CHAR!(buf, idx, self) {
            CHECK_NAME_CASES! {
                (buf, idx, nextTokIdx, self),
                match self.byte_type(&buf[idx..]),
                ByteType::SEMI => {
                    *nextTokIdx = idx + self.MINBPC();
                    return XML_TOK::PARAM_ENTITY_REF;
                }
                _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
            }
        }
        XML_TOK::PARTIAL
    }
    fn scanPoundName(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        REQUIRE_CHAR!(buf, idx, self);
        CHECK_NMSTRT_CASES! {
            (buf, idx, nextTokIdx, self),
            match self.byte_type(&buf[idx..]),
            _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
        }
        while HAS_CHAR!(buf, idx, self) {
            CHECK_NAME_CASES! {
                (buf, idx, nextTokIdx, self),
                match self.byte_type(&buf[idx..]),
                ByteType::CR | ByteType::LF | ByteType::S | ByteType::RPAR | ByteType::GT | ByteType::PERCNT | ByteType::VERBAR => {
                    *nextTokIdx = idx;
                    return XML_TOK::POUND_NAME;
                }
                _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
            }
        }
        return XML_TOK::POUND_NAME_NEG;
    }
    fn scanLit(
        &self,
        mut open: ByteType,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        while HAS_CHAR!(buf, idx, self) {
            let mut t = self.byte_type(&buf[idx..]);
            MATCH_INVALID_CASES! {
                (buf, idx, nextTokIdx, self),
                match t,
                ByteType::QUOT | ByteType::APOS => {
                    idx += self.MINBPC();
                    if t == open {
                        if !HAS_CHAR!(buf, idx, self) {
                            return XML_TOK::LITERAL_NEG;
                        }
                        *nextTokIdx = idx;
                        match self.byte_type(&buf[idx..]) {
                            ByteType::S | ByteType::CR | ByteType::LF | ByteType::GT | ByteType::PERCNT | ByteType::LSQB => {
                                return XML_TOK::LITERAL;
                            }
                            _ => { return XML_TOK::INVALID }
                        }
                    }
                }
                _ => { idx += self.MINBPC() }
            }
        }
        XML_TOK::PARTIAL
    }
}

impl<T: XmlEncodingImpl> XmlTokImpl for T { }

impl<T: XmlEncodingImpl+XmlTokImpl> XmlEncoding for T {
    fn cdataSectionTok(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        if buf.is_empty() {
            return XML_TOK::NONE;
        }
        let buf = if self.MINBPC() > 1 {
            let mut n: size_t = buf.len();
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK::PARTIAL;
                }
                buf.with_len(n)
            } else {
                buf
            }
        } else {
            buf
        };
        let mut idx = 0;
        MATCH_INVALID_CASES! {
            (buf, idx, nextTokIdx, self),
            match self.byte_type(&buf[idx..]),
            ByteType::RSQB => {
                idx += self.MINBPC();
                REQUIRE_CHAR!(buf, idx, self);
                if self.char_matches(&buf[idx..], ASCII_RSQB) {
                    let prev_idx = idx;
                    idx += self.MINBPC();
                    REQUIRE_CHAR!(buf, idx, self);
                    if !self.char_matches(&buf[idx..], ASCII_GT) {
                        idx = prev_idx;
                    } else {
                        *nextTokIdx = idx + self.MINBPC();
                        return XML_TOK::CDATA_SECT_CLOSE
                    }
                }
            }
            ByteType::CR => {
                idx += self.MINBPC();
                REQUIRE_CHAR!(buf, idx, self);
                if self.byte_type(&buf[idx..]) == ByteType::LF {
                    idx += self.MINBPC();
                }
                *nextTokIdx = idx;
                return XML_TOK::DATA_NEWLINE;
            }
            ByteType::LF => {
                *nextTokIdx = idx + self.MINBPC();
                return XML_TOK::DATA_NEWLINE;
            }
            _ => { idx += self.MINBPC() }
        }
        while HAS_CHAR!(buf, idx, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(&buf[idx..]),
                LEAD_CASE(n) => {
                    if buf[idx..].len() < n || self.is_invalid_char(&buf[idx..], n) {
                        *nextTokIdx = idx;
                        return XML_TOK::DATA_CHARS;
                    }
                    idx += n;
                }
                ByteType::NONXML | ByteType::MALFORM | ByteType::TRAIL | ByteType::CR | ByteType::LF | ByteType::RSQB => {
                    *nextTokIdx = idx;
                    return XML_TOK::DATA_CHARS;
                }
                _ => { idx += self.MINBPC() }
            }
        }
        *nextTokIdx = idx;
        XML_TOK::DATA_CHARS
    }
    fn contentTok(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        if buf.is_empty() {
            return XML_TOK::NONE;
        }
        let buf = if self.MINBPC() > 1 {
            let mut n: size_t = buf.len() as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK::PARTIAL;
                }
                buf.with_len(n as usize)
            } else {
                buf
            }
        } else {
            buf
        };
        let mut idx = 0;
        MATCH_INVALID_CASES! {
            (buf, idx, nextTokIdx, self),
            match self.byte_type(&buf[idx..]),
            ByteType::LT => {
                return self.call_rec(Self::scanLt, buf, idx + self.MINBPC(), nextTokIdx);
            }
            ByteType::AMP => {
                return self.call_rec(Self::scanRef, buf, idx + self.MINBPC(), nextTokIdx);
            }
            ByteType::CR => {
                idx += self.MINBPC();
                if !HAS_CHAR!(buf, idx, self) {
                    return XML_TOK::TRAILING_CR;
                }
                if self.byte_type(&buf[idx..]) == ByteType::LF {
                    idx += self.MINBPC();
                }
                *nextTokIdx = idx;
                return XML_TOK::DATA_NEWLINE;
            }
            ByteType::LF => {
                *nextTokIdx = idx + self.MINBPC();
                return XML_TOK::DATA_NEWLINE;
            }
            ByteType::RSQB => {
                idx += self.MINBPC();
                if !HAS_CHAR!(buf, idx, self) {
                    return XML_TOK::TRAILING_RSQB;
                }
                if self.char_matches(&buf[idx..], ASCII_RSQB) {
                    let prev_idx = idx;
                    idx += self.MINBPC();
                    if !HAS_CHAR!(buf, idx, self) {
                        return XML_TOK::TRAILING_RSQB;
                    }
                    if !self.char_matches(&buf[idx..], ASCII_GT) {
                        idx = prev_idx;
                    } else {
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID;
                    }
                }
            }
            _ => { idx += self.MINBPC() }
        }
        while HAS_CHAR!(buf, idx, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(&buf[idx..]),
                LEAD_CASE(n) => {
                    if buf[idx..].len() < n || self.is_invalid_char(&buf[idx..], n) {
                        *nextTokIdx = idx;
                        return XML_TOK::DATA_CHARS;
                    }
                    idx += n;
                }
                ByteType::RSQB => {
                    if HAS_CHARS!(buf, idx, 2, self) {
                        if !self.char_matches(&buf[idx + self.MINBPC()..], ASCII_RSQB) {
                            idx += self.MINBPC();
                        } else if HAS_CHARS!(buf, idx, 3, self) {
                            if !self.char_matches(&buf[idx + 2 * self.MINBPC()..], ASCII_GT) {
                                idx += self.MINBPC();
                            } else {
                                *nextTokIdx = idx + 2 * self.MINBPC();
                                return XML_TOK::INVALID;
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
                _ => { idx += self.MINBPC(); }
            }
        }
        *nextTokIdx = idx;
        XML_TOK::DATA_CHARS
    }
    fn prologTok(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut tok = XML_TOK::INVALID;
        if buf.is_empty() {
            return XML_TOK::NONE;
        }
        let buf = if self.MINBPC() > 1 {
            let mut n: size_t = buf.len() as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                if n == 0 {
                    return XML_TOK::PARTIAL;
                }
                buf.with_len(n as usize)
            } else {
                buf
            }
        } else {
            buf
        };
        let mut idx = 0;
        MATCH_LEAD_CASES! {
            self.byte_type(&buf[idx..]),
            LEAD_CASE(n) => {
                if (buf[idx..].len() as c_long) < n as c_long {
                    return XML_TOK::PARTIAL_CHAR;
                }
                if self.is_nmstrt_char(&buf[idx..], n) {
                    idx += n;
                    tok = XML_TOK::NAME;
                } else if self.is_name_char(&buf[idx..], n) {
                    idx += n;
                    tok = XML_TOK::NMTOKEN;
                } else {
                    *nextTokIdx = idx;
                    return XML_TOK::INVALID;
                }
            }
            ByteType::QUOT => {
                let mut next_idx = usize::MAX;
                let result = self.scanLit(
                    ByteType::QUOT,
                    buf.inc_start(idx + self.MINBPC()),
                    &mut next_idx);
                if next_idx != usize::MAX {
                    *nextTokIdx = idx + self.MINBPC() + next_idx;
                }
                return result;
            }
            ByteType::APOS => {
                let mut next_idx = usize::MAX;
                let result = self.scanLit(
                    ByteType::APOS,
                    buf.inc_start(idx + self.MINBPC()),
                    &mut next_idx);
                if next_idx != usize::MAX {
                    *nextTokIdx = idx + self.MINBPC() + next_idx;
                }
                return result;
            }
            ByteType::LT => {
                idx += self.MINBPC();
                REQUIRE_CHAR!(buf, idx, self);
                match self.byte_type(&buf[idx..]) {
                    ByteType::EXCL => {
                        return self.call_rec(Self::scanDecl, buf, idx + self.MINBPC(), nextTokIdx);
                    }
                    ByteType::QUEST => {
                        return self.call_rec(Self::scanPi, buf, idx + self.MINBPC(), nextTokIdx);
                    }
                    ByteType::NMSTRT | ByteType::HEX | ByteType::NONASCII | ByteType::LEAD2 | ByteType::LEAD3 | ByteType::LEAD4 => {
                        *nextTokIdx = idx - self.MINBPC();
                        return XML_TOK::INSTANCE_START;
                    }
                    _ => { }
                }
                *nextTokIdx = idx;
                return XML_TOK::INVALID;
            }
            ByteType::CR if buf[idx..].len() == self.MINBPC() => {
                *nextTokIdx = buf.len();
                /* indicate that this might be part of a CR/LF pair */
                return XML_TOK::PROLOG_S_NEG;
            }
            ByteType::S | ByteType::LF | ByteType::CR => {
                loop {
                    idx += self.MINBPC();
                    if !HAS_CHAR!(buf, idx, self) {
                        break;
                    }
                    let b = self.byte_type(&buf[idx..]);
                    match b {
                        ByteType::CR if buf[idx..].len() == self.MINBPC() => {
                            /* don't split CR/LF pair */
                            *nextTokIdx = idx;
                            return XML_TOK::PROLOG_S;
                        }
                        ByteType::S | ByteType::LF | ByteType::CR => {
                            /* do nothing */
                        }
                        _ => {
                            *nextTokIdx = idx;
                            return XML_TOK::PROLOG_S;
                        }
                    }
                }
                *nextTokIdx = idx;
                return XML_TOK::PROLOG_S;
            }
            ByteType::PERCNT => {
                return self.call_rec(Self::scanPercent, buf, idx + self.MINBPC(), nextTokIdx);
            }
            ByteType::COMMA => {
                *nextTokIdx = idx + self.MINBPC();
                return XML_TOK::COMMA;
            }
            ByteType::LSQB => {
                *nextTokIdx = idx + self.MINBPC();
                return XML_TOK::OPEN_BRACKET;
            }
            ByteType::RSQB => {
                idx += self.MINBPC();
                if !HAS_CHAR!(buf, idx, self) {
                    return XML_TOK::CLOSE_BRACKET_NEG;
                }
                if self.char_matches(&buf[idx..], ASCII_RSQB) {
                    REQUIRE_CHARS!(buf, idx, 2, self);
                    if self.char_matches(&buf[idx + self.MINBPC()..], ASCII_GT) {
                        *nextTokIdx = idx + 2 * self.MINBPC();
                        return XML_TOK::COND_SECT_CLOSE;
                    }
                }
                *nextTokIdx = idx;
                return XML_TOK::CLOSE_BRACKET;
            }
            ByteType::LPAR => {
                *nextTokIdx = idx + self.MINBPC();
                return XML_TOK::OPEN_PAREN;
            }
            ByteType::RPAR => {
                idx += self.MINBPC();
                if !HAS_CHAR!(buf, idx, self) {
                    return XML_TOK::CLOSE_PAREN_NEG;
                }
                match self.byte_type(&buf[idx..]) {
                    ByteType::AST => {
                        *nextTokIdx = idx + self.MINBPC();
                        return XML_TOK::CLOSE_PAREN_ASTERISK;
                    }
                    ByteType::QUEST => {
                        *nextTokIdx = idx + self.MINBPC();
                        return XML_TOK::CLOSE_PAREN_QUESTION;
                    }
                    ByteType::PLUS => {
                        *nextTokIdx = idx + self.MINBPC();
                        return XML_TOK::CLOSE_PAREN_PLUS;
                    }
                    ByteType::CR | ByteType::LF | ByteType::S | ByteType::GT | ByteType::COMMA | ByteType::VERBAR | ByteType::RPAR => {
                        *nextTokIdx = idx;
                        return XML_TOK::CLOSE_PAREN;
                    }
                    _ => { }
                }
                *nextTokIdx = idx;
                return XML_TOK::INVALID;
            }
            ByteType::VERBAR => {
                *nextTokIdx = idx + self.MINBPC();
                return XML_TOK::OR;
            }
            ByteType::GT => {
                *nextTokIdx = idx + self.MINBPC();
                return XML_TOK::DECL_CLOSE;
            }
            ByteType::NUM => {
                return self.call_rec(Self::scanPoundName, buf, idx + self.MINBPC(), nextTokIdx);
            }
            ByteType::NMSTRT | ByteType::HEX => {
                tok = XML_TOK::NAME;
                idx += self.MINBPC();
            }
            ByteType::DIGIT | ByteType::NAME | ByteType::MINUS | ByteType::COLON => {
                tok = XML_TOK::NMTOKEN;
                idx += self.MINBPC();
            }
            ByteType::NONASCII => {
                if self.is_nmstrt_char_minbpc(&buf[idx..]) {
                    idx += self.MINBPC();
                    tok = XML_TOK::NAME;
                } else if self.is_name_char_minbpc(&buf[idx..]) {
                    idx += self.MINBPC();
                    tok = XML_TOK::NMTOKEN;
                } else {
                    *nextTokIdx = idx;
                    return XML_TOK::INVALID
                }
            }
            _ => {
                *nextTokIdx = idx;
                return XML_TOK::INVALID
            }
        }
        while HAS_CHAR!(buf, idx, self) {
            CHECK_NAME_CASES! {
                (buf, idx, nextTokIdx, self),
                match self.byte_type(&buf[idx..]),
                ByteType::GT | ByteType::RPAR | ByteType::COMMA | ByteType::VERBAR | ByteType::LSQB | ByteType::PERCNT | ByteType::S | ByteType::CR | ByteType::LF => {
                    *nextTokIdx = idx;
                    return tok;
                }
                ByteType::COLON => {
                    idx += self.MINBPC();
                    match tok {
                        XML_TOK::NAME => {
                            REQUIRE_CHAR!(buf, idx, self);
                            tok = XML_TOK::PREFIXED_NAME;
                            CHECK_NAME_CASES! {
                                (buf, idx, nextTokIdx, self),
                                match self.byte_type(&buf[idx..]),
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
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID
                    }
                    *nextTokIdx = idx + self.MINBPC();
                    return XML_TOK::NAME_PLUS
                }
                ByteType::AST => {
                    if tok == XML_TOK::NMTOKEN {
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID
                    }
                    *nextTokIdx = idx + self.MINBPC();
                    return XML_TOK::NAME_ASTERISK
                }
                ByteType::QUEST => {
                    if tok == XML_TOK::NMTOKEN {
                        *nextTokIdx = idx;
                        return XML_TOK::INVALID
                    }
                    *nextTokIdx = idx + self.MINBPC();
                    return XML_TOK::NAME_QUESTION
                }
                _ => { *nextTokIdx = idx; return XML_TOK::INVALID }
            }
        }
        tok.negate()
    }
    fn attributeValueTok(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        if buf.is_empty() {
            return XML_TOK::NONE;
        } else {
            if !HAS_CHAR!(buf, idx, self) {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                return XML_TOK::PARTIAL;
            }
        }
        let start = idx;
        while HAS_CHAR!(buf, idx, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(&buf[idx..]),
                LEAD_CASE(n) => {
                    idx += n;
                }
                ByteType::AMP => {
                    if idx == start {
                        return self.call_rec(Self::scanRef, buf, idx + self.MINBPC(), nextTokIdx);
                    }
                    *nextTokIdx = idx;
                    return XML_TOK::DATA_CHARS;
                }
                ByteType::LT => {
                    /* this is for inside entity references */
                    *nextTokIdx = idx;
                    return XML_TOK::INVALID;
                }
                ByteType::LF => {
                    if idx == start {
                        *nextTokIdx = idx + self.MINBPC();
                        return XML_TOK::DATA_NEWLINE
                    }
                    *nextTokIdx = idx;
                    return XML_TOK::DATA_CHARS
                }
                ByteType::CR => {
                    if idx == start {
                        idx += self.MINBPC();
                        if !HAS_CHAR!(buf, idx, self) {
                            return XML_TOK::TRAILING_CR;
                        }
                        if self.byte_type(&buf[idx..]) == ByteType::LF {
                            idx += self.MINBPC();
                        }
                        *nextTokIdx = idx;
                        return XML_TOK::DATA_NEWLINE;
                    }
                    *nextTokIdx = idx;
                    return XML_TOK::DATA_CHARS;
                }
                ByteType::S => {
                    if idx == start {
                        *nextTokIdx = idx + self.MINBPC();
                        return XML_TOK::ATTRIBUTE_VALUE_S
                    }
                    *nextTokIdx = idx;
                    return XML_TOK::DATA_CHARS;
                }
                _ => { idx += self.MINBPC() }
            }
        }
        *nextTokIdx = idx;
        XML_TOK::DATA_CHARS
    }
    fn entityValueTok(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut idx = 0;
        if buf.is_empty() {
            return XML_TOK::NONE;
        } else {
            if !HAS_CHAR!(buf, idx, self) {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the paranoia
                 * check is still valuable, however.
                 */
                return XML_TOK::PARTIAL;
            }
        }
        let start = idx;
        while HAS_CHAR!(buf, idx, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(&buf[idx..]),
                LEAD_CASE(n) => {
                    idx += n;
                }
                ByteType::AMP => {
                    if idx == start {
                        return self.call_rec(Self::scanRef, buf, idx + self.MINBPC(), nextTokIdx);
                    }
                    *nextTokIdx = idx;
                    return XML_TOK::DATA_CHARS;
                }
                ByteType::PERCNT => {
                    if idx == start {
                        let tok =
                            self.scanPercent(buf.inc_start(idx + self.MINBPC()),
                                             nextTokIdx);
                        return if tok == XML_TOK::PERCENT {
                                   XML_TOK::INVALID
                               } else { tok }
                    }
                    *nextTokIdx = idx;
                    return XML_TOK::DATA_CHARS;
                }
                ByteType::LF => {
                    if idx == start {
                        *nextTokIdx = idx + self.MINBPC();
                        return XML_TOK::DATA_NEWLINE;
                    }
                    *nextTokIdx = idx;
                    return XML_TOK::DATA_CHARS;
                }
                ByteType::CR => {
                    if idx == start {
                        idx += self.MINBPC();
                        if !HAS_CHAR!(buf, idx, self) {
                            return XML_TOK::TRAILING_CR;
                        }
                        if self.byte_type(&buf[idx..]) == ByteType::LF {
                            idx += self.MINBPC();
                        }
                        *nextTokIdx = idx;
                        return XML_TOK::DATA_NEWLINE;
                    }
                    *nextTokIdx = idx;
                    return XML_TOK::DATA_CHARS;
                }
                _ => { idx += self.MINBPC() }
            }
        }
        *nextTokIdx = idx;
        XML_TOK::DATA_CHARS
    }
    fn ignoreSectionTok(
        &self,
        buf: ExpatBufRef,
        mut nextTokIdx: &mut usize,
    ) -> XML_TOK {
        let mut level: libc::c_int = 0 as libc::c_int;
        let buf = if self.MINBPC() > 1 {
            let mut n: size_t = buf.len() as libc::c_long as size_t;
            if n & (self.MINBPC() - 1) as size_t != 0 {
                n &= !(self.MINBPC() - 1) as size_t;
                buf.with_len(n as usize)
            } else {
                buf
            }
        } else {
            buf
        };
        let mut idx = 0;
        while HAS_CHAR!(buf, idx, self) {
            MATCH_INVALID_CASES! {
                (buf, idx, nextTokIdx, self),
                match self.byte_type(&buf[idx..]),
                ByteType::LT => {
                    idx += self.MINBPC();
                    REQUIRE_CHAR!(buf, idx, self);
                    if self.char_matches(&buf[idx..], ASCII_EXCL) {
                        idx += self.MINBPC();
                        REQUIRE_CHAR!(buf, idx, self);
                        if self.char_matches(&buf[idx..], ASCII_LSQB) {
                            level += 1;
                            idx += self.MINBPC();
                        }
                    }
                }
                ByteType::RSQB => {
                    idx += self.MINBPC();
                    REQUIRE_CHAR!(buf, idx, self);
                    if self.char_matches(&buf[idx..], ASCII_RSQB) {
                        idx += self.MINBPC();
                        REQUIRE_CHAR!(buf, idx, self);
                        if self.char_matches(&buf[idx..], ASCII_GT) {
                            idx += self.MINBPC();
                            if level == 0 as libc::c_int {
                                *nextTokIdx = idx;
                                return XML_TOK::IGNORE_SECT;
                            }
                            level -= 1;
                        }
                    }
                }
                _ => { idx += self.MINBPC() }
            }
        }
        XML_TOK::PARTIAL
    }
    fn isPublicId(
        &self,
        buf: ExpatBufRef,
        mut badPtr: &Cell<*const libc::c_char>,
    ) -> libc::c_int {
        let buf = buf.dec_end(self.MINBPC());
        let mut idx = self.MINBPC();
        while HAS_CHAR!(buf, idx, self) {
            match self.byte_type(&buf[idx..]) {
                ByteType::DIGIT | ByteType::HEX | ByteType::MINUS | ByteType::APOS | ByteType::LPAR | ByteType::RPAR | ByteType::PLUS | ByteType::COMMA | ByteType::SOL | ByteType::EQUALS | ByteType::QUEST | ByteType::CR | ByteType::LF | ByteType::SEMI | ByteType::EXCL | ByteType::AST
                | ByteType::PERCNT | ByteType::NUM | ByteType::COLON => { }
                ByteType::S => {
                    if self.char_matches(&buf[idx..], ASCII_TAB) {
                        badPtr.set(buf[idx..].as_ptr());
                        return 0;
                    }
                }
                ByteType::NAME | ByteType::NMSTRT if self.byte_to_ascii(&buf[idx..]) & !(0x7f as c_char) == 0 => { }
                _ => {
                    match self.byte_to_ascii(&buf[idx..]) {
                        0x24 => {} /* $ */
                        0x40 => {} /* @ */
                        _ => {
                            badPtr.set(buf[idx..].as_ptr());
                            return 0;
                        }
                    }
                }
            }
            idx += self.MINBPC();
        }
        1
    }

    /* This must only be called for a well-formed start-tag or empty
    element tag.  Returns the number of attributes.  Pointers to the
    first attsMax attributes are stored in atts.
     */
    fn getAtts(
        &self,
        buf: ExpatBufRef,
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

        let mut idx = self.MINBPC();
        macro_rules! START_NAME {
            () => {
                if state == State::Other {
                    att.name = buf[idx..].as_ptr();
                    att.normalized = true;
                    state = State::InName;
                }
            };
        }

        loop {
            MATCH_LEAD_CASES! {
                   self.byte_type(&buf[idx..]),
                   LEAD_CASE(n) => {
                       START_NAME!{}
                       idx += n - self.MINBPC();
                   }
                   ByteType::NONASCII | ByteType::NMSTRT | ByteType::HEX => {
                       START_NAME!{}
                   }
                   ByteType::QUOT => {
                       if state != State::InValue {
                           att.valuePtr = buf.inc_start(idx + self.MINBPC()).as_ptr();
                           state = State::InValue;
                           open = ByteType::QUOT;
                       } else if open == ByteType::QUOT {
                           state = State::Other;
                           att.valueEnd = buf[idx..].as_ptr();

                           let res = f(att);
                           if res != XML_Error::NONE {
                               return res;
                           }
                       }
                   }
                   ByteType::APOS => {
                       if state != State::InValue {
                           att.valuePtr = buf.inc_start(idx + self.MINBPC()).as_ptr();
                           state = State::InValue;
                           open = ByteType::APOS;
                       } else if open == ByteType::APOS {
                           state = State::Other;
                           att.valueEnd = buf[idx..].as_ptr();

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
                           && (buf[idx..].as_ptr() == att.valuePtr
                               || self.byte_to_ascii(&buf[idx..]) != ASCII_SPACE
                               || self.byte_to_ascii(&buf[idx + self.MINBPC()..]) == ASCII_SPACE
                               || self.byte_type(&buf[idx + self.MINBPC()..]) == open)
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
                           return XML_Error::NONE;
                       }
                   }
                   _ => { }
            }
            idx += self.MINBPC();
        }
        /* not reached */
    }
    fn charRefNumber(&self, buf: ExpatBufRef) -> libc::c_int {
        let mut idx = 0;
        let mut result: libc::c_int = 0 as libc::c_int;
        idx += 2 * self.MINBPC();
        if self.char_matches(&buf[idx..], ASCII_x) {
            idx += self.MINBPC();
            while !self.char_matches(&buf[idx..], ASCII_SEMI) {
                let mut c: c_char = self.byte_to_ascii(&buf[idx..]);
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
                idx += self.MINBPC();
            }
        } else {
            while !self.char_matches(&buf[idx..], ASCII_SEMI) {
                let mut c_0: c_char = self.byte_to_ascii(&buf[idx..]);
                result *= 10 as libc::c_int;
                result += (c_0 - ASCII_0) as c_int;
                if result >= 0x110000 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                idx += self.MINBPC();
            }
        }
        checkCharRefNumber(result)
    }
    fn predefinedEntityName(
        &self,
        buf: ExpatBufRef,
    ) -> libc::c_int {
        let mut idx = 0;
        match buf.len() / self.MINBPC() {
            2 => {
                if self.char_matches(&buf[idx + self.MINBPC()..], ASCII_t) {
                    match self.byte_to_ascii(&buf[idx..]) {
                        ASCII_l => return ASCII_LT as c_int,
                        ASCII_g => return ASCII_GT as c_int,
                        _ => {}
                    }
                }
            }
            3 => {
                if self.char_matches(&buf[idx..], ASCII_a) {
                    idx += self.MINBPC();
                    if self.char_matches(&buf[idx..], ASCII_m) {
                        idx += self.MINBPC();
                        if self.char_matches(&buf[idx..], ASCII_p) {
                            return ASCII_AMP as c_int;
                        }
                    }
                }
            }
            4 => match self.byte_to_ascii(&buf[idx..]) {
                ASCII_q => {
                    idx += self.MINBPC();
                    if self.char_matches(&buf[idx..], ASCII_u) {
                        idx += self.MINBPC();
                        if self.char_matches(&buf[idx..], ASCII_o) {
                            idx += self.MINBPC();
                            if self.char_matches(&buf[idx..], ASCII_t) {
                                return ASCII_QUOT as c_int;
                            }
                        }
                    }
                }
                ASCII_a => {
                    idx += self.MINBPC();
                    if self.char_matches(&buf[idx..], ASCII_p) {
                        idx += self.MINBPC();
                        if self.char_matches(&buf[idx..], ASCII_o) {
                            idx += self.MINBPC();
                            if self.char_matches(&buf[idx..], ASCII_s) {
                                return ASCII_APOS as c_int;
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        0
    }
    fn nameMatchesAscii(
        &self,
        buf: ExpatBufRef,
        mut ptr2: &[libc::c_char],
    ) -> bool {
        let mut idx = 0;
        for ch in ptr2 {
            if buf[idx..].len() < self.MINBPC() {
                /* This line cannot be executed.  The incoming data has already
                 * been tokenized once, so incomplete characters like this have
                 * already been eliminated from the input.  Retaining the
                 * paranoia check is still valuable, however.
                 */
                return false;
            }
            if !self.char_matches(&buf[idx..], *ch) {
                return false;
            }
            idx += self.MINBPC();
        }
        buf[idx..].is_empty()
    }
    fn nameLength(&self, buf: ExpatBufRef) -> libc::c_int {
        let mut idx = 0;
        loop {
            MATCH_LEAD_CASES! {
                self.byte_type(&buf[idx..]),
                LEAD_CASE(n) => {
                    idx += n;
                }
                ByteType::NONASCII | ByteType::NMSTRT | ByteType::COLON | ByteType::HEX | ByteType::DIGIT | ByteType::NAME | ByteType::MINUS => {
                    idx += self.MINBPC();
                }
                _ => {
                    return idx as libc::c_long as libc::c_int;
                }
            }
        }
    }
    fn skipS<'a>(&self, mut buf: ExpatBufRef<'a>) -> ExpatBufRef<'a> {
        loop {
            match self.byte_type(&buf[..]) {
                ByteType::LF | ByteType::CR | ByteType::S => buf = buf.inc_start(self.MINBPC()),
                _ => return buf,
            }
        }
    }
    fn updatePosition(
        &self,
        buf: ExpatBufRef,
        mut pos: &mut Position,
    ) {
        let mut idx = 0;
        while HAS_CHAR!(buf, idx, self) {
            MATCH_LEAD_CASES! {
                self.byte_type(&buf[idx..]),
                LEAD_CASE(n) => {
                    idx += n;
                }
                ByteType::LF => {
                    (*pos).columnNumber = -(1 as libc::c_int) as XML_Size;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    idx += self.MINBPC();
                }
                ByteType::CR => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    idx += self.MINBPC();
                    if HAS_CHAR!(buf, idx, self) &&
                           self.byte_type(&buf[idx..]) == ByteType::LF {
                        idx += self.MINBPC();
                    }
                    (*pos).columnNumber = -(1 as libc::c_int) as XML_Size;
                }
                _ => { idx += self.MINBPC() }
            }
            (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
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

    fn minBytesPerChar(&self) -> usize {
        self.MINBPC()
    }

    fn isUtf8(&self) -> bool {
        self.isUtf8()
    }

    fn isUtf16(&self) -> bool {
        self.isUtf16()
    }
}
