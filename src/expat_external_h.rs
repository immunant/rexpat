use libc::{c_char, c_long, c_ulong};
pub type XML_Char = c_char;
pub type XML_LChar = c_char;
/* XML_UNICODE */

/* Use large integers for file/stream positions. */
pub type XML_Index = c_long;
pub type XML_Size = c_ulong;
