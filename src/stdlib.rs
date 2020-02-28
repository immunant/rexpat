use ::libc::FILE;
use libc::{c_int, c_long, c_ulong, clock_t};
extern "C" {
    #[no_mangle]
    pub fn _setjmp(_: *mut __jmp_buf_tag) -> c_int;

    #[no_mangle]
    pub fn clock() -> clock_t;

    #[no_mangle]
    pub fn longjmp(_: *mut __jmp_buf_tag, _: c_int) -> !;

    #[cfg(all(unix, not(target_os = "macos")))]
    #[no_mangle]
    pub static mut stdout: *mut FILE;

    #[cfg(all(unix, not(target_os = "macos")))]
    #[no_mangle]
    pub static mut stderr: *mut FILE;

    #[cfg(all(unix, not(target_os = "macos")))]
    #[no_mangle]
    pub static mut stdin: *mut FILE;

    #[cfg(target_os = "macos")]
    #[no_mangle]
    #[link_name = "__stdoutp"]
    pub static mut stdout: *mut FILE;

    #[cfg(target_os = "macos")]
    #[no_mangle]
    #[link_name = "__stderrp"]
    pub static mut stderr: *mut FILE;

    #[cfg(target_os = "macos")]
    #[no_mangle]
    #[link_name = "__stdinp"]
    pub static mut stdin: *mut FILE;

    #[no_mangle]
    pub fn putc(__c: c_int, __stream: *mut FILE) -> c_int;
}

// We never touch the fields using these types, and they're all hidden behind pointers.
// So we're using our own opaque struct (in the manner recommended by the nomicon) to
// avoid using the extern_types nightly feature.
// #[repr(C)] pub struct _IO_marker { _private: [u8; 0] }
// #[repr(C)] pub struct _IO_codecvt { _private: [u8; 0] }
// #[repr(C)] pub struct _IO_wide_data { _private: [u8; 0] }

// =============== BEGIN FILE_h ================
// ================ END FILE_h ================
// =============== BEGIN __sigset_t_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16],
}
// ================ END __sigset_t_h ================
// =============== BEGIN include_setjmp_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
// ================ END include_setjmp_h ================
// =============== BEGIN random_h ================
pub const GRND_NONBLOCK: c_int = 0x1;
// ================ END random_h ================
// =============== BEGIN setjmp_h ================
pub type __jmp_buf = [c_long; 8];
// ================ END setjmp_h ================

pub const CLOCKS_PER_SEC: c_int = 1000000 as c_int;
