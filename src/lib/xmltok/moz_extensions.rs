use libc::{c_char, c_ushort, c_int};
use crate::expat_external_h::XML_Char;
use crate::lib::xmltok::{
    MOZ_XmlGetUtf16InternalEncodingNS,
    MOZ_XmlUtf16Encode,
    XmlEncodingImpl,
    XML_TOK_INVALID,
    XML_TOK_CHAR_REF,
    XML_TOK_ENTITY_REF,
};
use crate::lib::xmltok_impl::XmlTokImpl;
use crate::xmltok_impl_h::ByteType;

#[cfg(target_endian = "little")]
use crate::lib::xmltok::internal_little2_encoding_ns as encoding;

#[cfg(target_endian = "big")]
use crate::lib::xmltok::internal_big2_encoding_ns as encoding;

macro_rules! BYTE_TYPE {
    ($p:expr) => {
        encoding.as_ref().unwrap().byte_type($p)
    };
}

macro_rules! IS_NAME_CHAR_MINBPC {
    ($p:expr) => {
        encoding.as_ref().unwrap().is_name_char_minbpc($p)
    };
}

macro_rules! IS_NMSTRT_CHAR_MINBPC {
    ($p:expr) => {
        encoding.as_ref().unwrap().is_nmstrt_char_minbpc($p)
    };
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
        match BYTE_TYPE!(ptr) {
            ByteType::COLON => {
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
            ByteType::NONASCII => {
                if !IS_NAME_CHAR_MINBPC!(ptr) ||
                    (nmstrt && (*colon).is_null() && !IS_NMSTRT_CHAR_MINBPC!(ptr))
                {
                    return MOZ_EXPAT_INVALID_CHARACTER;
                }
                if nmstrt && !(*colon).is_null() && !IS_NMSTRT_CHAR_MINBPC!(ptr) {
                    /* If a non-starting character like a number is right after the colon,
                       this is a namespace error, not invalid character */
                    return MOZ_EXPAT_MALFORMED;
                }
                nmstrt = false;
            }
            ByteType::NMSTRT | ByteType::HEX => nmstrt = false,
            ByteType::DIGIT | ByteType::NAME | ByteType::MINUS => {
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
    match BYTE_TYPE!(ptr) {
        ByteType::NONASCII if !IS_NMSTRT_CHAR_MINBPC!(ptr) => 0,
        ByteType::NONASCII | ByteType::NMSTRT | ByteType::HEX  => 1,
        _ => 0
    }
}

#[no_mangle]
pub unsafe extern "C" fn MOZ_XMLIsNCNameChar(ptr: *const c_char) -> c_int {
    match BYTE_TYPE!(ptr) {
        ByteType::NONASCII if !IS_NAME_CHAR_MINBPC!(ptr) => 0,
        ByteType::NONASCII | ByteType::NMSTRT | ByteType::HEX | ByteType::DIGIT | ByteType::NAME | ByteType::MINUS => 1,
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
    let enc_mbpc = (*enc).minBytesPerChar() as isize;
    /* scanRef expects to be pointed to the char after the '&'. */
    let tok = encoding.as_ref().unwrap().scanRef(ptr.offset(enc_mbpc), end, next);
    if tok <= XML_TOK_INVALID {
        return 0;
    }

    match tok {
        XML_TOK_CHAR_REF => {
            /* XmlCharRefNumber expects to be pointed to the '&'. */
            let n = (*enc).charRefNumber(ptr);

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
            let ch = (*enc).predefinedEntityName(ptr.offset(enc_mbpc), *next.offset(-enc_mbpc)) as XML_Char;
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
