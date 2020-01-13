#[repr(C)]
#[derive(Copy, Clone)]
pub struct siphash {
    pub v0: crate::stdlib::uint64_t,
    pub v1: crate::stdlib::uint64_t,
    pub v2: crate::stdlib::uint64_t,
    pub v3: crate::stdlib::uint64_t,
    pub buf: [libc::c_uchar; 8],
    pub p: *mut libc::c_uchar,
    pub c: crate::stdlib::uint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sipkey {
    pub k: [crate::stdlib::uint64_t; 2],
}
