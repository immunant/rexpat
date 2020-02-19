use libc::{c_char, c_int};
use crate::expat_external_h::XML_Char;
use crate::lib::xmltok::{
    MOZ_XmlUtf16Encode,
    XmlEncoding,
    XmlEncodingImpl,
    XML_TOK::INVALID,
    XML_TOK::CHAR_REF,
    XML_TOK::ENTITY_REF,
};
use crate::lib::xmltok_impl::XmlTokImpl;
use crate::xmltok_impl_h::ByteType;
use crate::lib::xmlparse::{ExpatBufRef, XML_ENCODE_MAX};

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
    let enc_mbpc = encoding.as_ref().unwrap().MINBPC();
    /* scanRef expects to be pointed to the char after the '&'. */
    let buf = ExpatBufRef::new(ptr, end);
    let tok = encoding.as_ref().unwrap().scanRef(buf.inc_start(enc_mbpc), next);
    if tok <= XML_TOK::INVALID {
        return 0;
    }

    match tok {
        XML_TOK::CHAR_REF => {
            /* XmlCharRefNumber expects to be pointed to the '&'. */
            let n = encoding.as_ref().unwrap().charRefNumber(buf);

            /* We could get away with just < 0, but better safe than sorry. */
            if n <= 0 {
                0
            } else {
                let result_slice = std::slice::from_raw_parts_mut(result, XML_ENCODE_MAX);
                MOZ_XmlUtf16Encode(n, result_slice)
            }
        }

        XML_TOK::ENTITY_REF => {
	    /* XmlPredefinedEntityName expects to be pointed to the char after '&'.
            
	     *next points to after the semicolon, so the entity ends at
	     *next - enc->minBytesPerChar. */
            let ch = encoding.as_ref().unwrap().predefinedEntityName(
                buf.inc_start(enc_mbpc)
                   .with_end(*next)
                   .dec_end(enc_mbpc as usize)) as XML_Char;
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
