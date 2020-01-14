use ::libc::INT_MAX;
use libc::c_int;
pub const XML_MAX_CHUNK_LEN: c_int = INT_MAX / 2i32 + 1i32;
