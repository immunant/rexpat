use libc::c_int;
use libc::c_long;
use libc::c_ulong;
pub type ptrdiff_t = c_long;
pub type size_t = c_ulong;
pub const NULL: c_int = 0 as c_int;
