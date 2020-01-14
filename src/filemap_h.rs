use ::libc::INT_MAX;
use libc::c_int;
pub const XML_MAX_CHUNK_LEN: c_int = INT_MAX / 2 as c_int + 1 as c_int;
