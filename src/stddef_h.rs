use libc::{c_int, c_long, c_ulong};
pub type ptrdiff_t = c_long;
pub type size_t = c_ulong;
pub const NULL: c_int = 0i32;
