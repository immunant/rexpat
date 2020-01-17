use libc::{c_char, c_uchar, c_ushort, c_int, c_uint};
use crate::expat_external_h::XML_Char;
use crate::lib::xmltok::{
    MOZ_XmlGetUtf16InternalEncodingNS,
    MOZ_XmlUtf16Encode,
    XML_TOK_INVALID,
    XML_TOK_CHAR_REF,
    XML_TOK_ENTITY_REF,
    normal_encoding,
    unicode_byte_type,
};
use crate::lib::xmltok::nametab_h::{namingBitmap, nmstrtPages, namePages};
use crate::lib::xmltok::xmltok_impl_c::{
    little2_scanRef,
    big2_scanRef,
};
use crate::xmltok_impl_h::{
    BT_COLON_0,
    BT_DIGIT,
    BT_HEX,
    BT_MINUS,
    BT_NAME,
    BT_NMSTRT,
    BT_NONASCII,
};

unsafe fn BYTE_TYPE(ptr: *const c_char) -> c_uint {
    let (msb, lsb) = if cfg!(target_endian = "little") {
        (1, 0)
    } else {
        (0, 1)
    };
    if *ptr.offset(msb) == 0 as c_char {
        let enc = MOZ_XmlGetUtf16InternalEncodingNS();
        (*(enc as *mut normal_encoding)).type_0[*ptr.offset(lsb) as usize]
            as c_uint
    } else {
        unicode_byte_type(*ptr.offset(msb), *ptr.offset(lsb)) as c_uint
    }
}

unsafe fn UCS2_GET_NAMING(pages: &[c_uchar], hi: c_char, lo: c_char) -> c_uint {
    namingBitmap[(pages[hi as usize] << 3) as usize + ((lo) >> 5) as usize]
        & (1 << ((lo) & 0x1F))
}

unsafe fn IS_NAME_CHAR_MINBPC(p: *const c_char) -> bool {
    let (msb, lsb) = if cfg!(target_endian = "little") {
        (1, 0)
    } else {
        (0, 1)
    };
    UCS2_GET_NAMING(&namePages, *p.offset(msb), *p.offset(lsb)) != 0
}

unsafe fn IS_NMSTRT_CHAR_MINBPC(p: *const c_char) -> bool {
    let (msb, lsb) = if cfg!(target_endian = "little") {
        (1, 0)
    } else {
        (0, 1)
    };
    UCS2_GET_NAMING(&nmstrtPages, *p.offset(msb), *p.offset(lsb)) != 0
}

const MOZ_EXPAT_VALID_QNAME: c_int = 0;
const MOZ_EXPAT_EMPTY_QNAME: c_int = (1 << 0);
const MOZ_EXPAT_INVALID_CHARACTER: c_int = (1 << 1);
const MOZ_EXPAT_MALFORMED: c_int = (1 << 2);

#[no_mangle]
pub unsafe extern "C" fn MOZ_XMLCheckQName(
    mut ptr: *const c_char,
    end: *const c_char,
    ns_aware: c_int,
    colon: *mut *const c_char,
) -> c_int {
    let mut nmstrt = true;
    *colon = std::ptr::null();
    if ptr == end {
        return MOZ_EXPAT_EMPTY_QNAME;
    }
    while ptr != end {
        match BYTE_TYPE(ptr) {
            BT_COLON_0 => {
                /* We're namespace-aware and either first or last character is a colon
                   or we've already seen a colon. */
                if ns_aware != 0 &&
                    (nmstrt || !(*colon).is_null() || ptr.offset(2) == end)
                {
                    return MOZ_EXPAT_MALFORMED;
                }
                *colon = ptr;
                nmstrt = ns_aware != 0; /* e.g. "a:0" should be valid if !ns_aware */
            }
            BT_NONASCII => {
                if !IS_NAME_CHAR_MINBPC(ptr) ||
                    (nmstrt && (*colon).is_null() && !IS_NMSTRT_CHAR_MINBPC(ptr))
                {
                    return MOZ_EXPAT_INVALID_CHARACTER;
                }
                if nmstrt && !(*colon).is_null() && !IS_NMSTRT_CHAR_MINBPC(ptr) {
                    /* If a non-starting character like a number is right after the colon,
                       this is a namespace error, not invalid character */
                    return MOZ_EXPAT_MALFORMED;
                }
                nmstrt = false;
            }
            BT_NMSTRT | BT_HEX => nmstrt = false,
            BT_DIGIT | BT_NAME | BT_MINUS => {
                if nmstrt {
                    return MOZ_EXPAT_INVALID_CHARACTER;
                }
            }
            _ => return MOZ_EXPAT_INVALID_CHARACTER
        }
        ptr = ptr.offset(2);
    }
    MOZ_EXPAT_VALID_QNAME
}

#[no_mangle]
pub unsafe extern "C" fn MOZ_XMLIsLetter(ptr: *const c_char) -> c_int {
    match BYTE_TYPE(ptr) {
        BT_NONASCII if !IS_NMSTRT_CHAR_MINBPC(ptr) => 0,
        BT_NONASCII | BT_NMSTRT | BT_HEX  => 1,
        _ => 0
    }
}

#[no_mangle]
pub unsafe extern "C" fn MOZ_XMLIsNCNameChar(ptr: *const c_char) -> c_int {
    match BYTE_TYPE(ptr) {
        BT_NONASCII if !IS_NAME_CHAR_MINBPC(ptr) => 0,
        BT_NONASCII | BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => 1,
        _ => 0
    }
}

#[no_mangle]
pub unsafe extern "C" fn MOZ_XMLTranslateEntity(
    ptr: *const c_char,
    end: *const c_char,
    next: *mut *const c_char,
    result: *mut XML_Char,
) -> c_int {
    // Can we assert here somehow?
    // MOZ_ASSERT(*ptr == '&');

    let enc = MOZ_XmlGetUtf16InternalEncodingNS();
    let enc_mbpc = (*enc).minBytesPerChar as isize;
    /* scanRef expects to be pointed to the char after the '&'. */
    let tok = if cfg!(target_endian = "little") {
        little2_scanRef(enc, ptr.offset(enc_mbpc), end, next)
    } else {
        big2_scanRef(enc, ptr.offset(enc_mbpc), end, next)
    };
    if tok <= XML_TOK_INVALID {
        return 0;
    }

    match tok {
        XML_TOK_CHAR_REF => {
            /* XmlCharRefNumber expects to be pointed to the '&'. */
            let n = (*enc).charRefNumber.expect("non-null function pointer")(enc, ptr);

            /* We could get away with just < 0, but better safe than sorry. */
            if n <= 0 {
                0
            } else {
                MOZ_XmlUtf16Encode(n, result as *mut c_ushort)
            }
        }

        XML_TOK_ENTITY_REF => {
			/* XmlPredefinedEntityName expects to be pointed to the char after '&'.

			   *next points to after the semicolon, so the entity ends at
			   *next - enc->minBytesPerChar. */
            let XmlPredefinedEntityName = (*enc)
                .predefinedEntityName
                .expect("non-null function pointer");
            let ch = XmlPredefinedEntityName(enc, ptr.offset(enc_mbpc),
                                             *next.offset(-enc_mbpc)) as XML_Char;
            if ch == 0 {
                0
            } else {
                *result = ch;
                1
            }
        }

        _ => 0
    }
}
