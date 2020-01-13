pub const ftprintf: unsafe extern "C" fn(
    _: *mut crate::stdlib::FILE,
    _: *const libc::c_char,
    _: ...
) -> libc::c_int = crate::stdlib::fprintf;
pub const tcscpy: unsafe extern "C" fn(
    _: *mut libc::c_char,
    _: *const libc::c_char,
) -> *mut libc::c_char = ::libc::strcpy;
pub const tcsrchr: unsafe extern "C" fn(
    _: *const libc::c_char,
    _: libc::c_int,
) -> *mut libc::c_char = ::libc::strrchr;
pub const tcslen: unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_ulong =
    crate::stdlib::strlen;
pub const tperror: unsafe extern "C" fn(_: *const libc::c_char) -> () = ::libc::perror;
pub const topen: unsafe extern "C" fn(
    _: *const libc::c_char,
    _: libc::c_int,
    _: ...
) -> libc::c_int = ::libc::open;
