#[cfg(all(feature = "unicode", not(feature = "unicode_wchar_t")))]
mod char_types {
    pub type XML_Char = libc::c_ushort;
    pub type XML_LChar = libc::c_char;
}

#[cfg(all(feature = "unicode", feature = "unicode_wchar_t"))]
mod char_types {
    pub type XML_Char = libc::wchar_t;
    pub type XML_LChar = libc::wchar_t;
}

#[cfg(not(feature = "unicode"))]
mod char_types {
    pub type XML_Char = libc::c_char;
    pub type XML_LChar = libc::c_char;
}

pub use char_types::*;

/* Use large integers for file/stream positions. */
pub type XML_Index = libc::c_long;
pub type XML_Size = libc::c_ulong;
