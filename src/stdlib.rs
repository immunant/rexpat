use crate::stddef_h::size_t;
use ::libc::timespec;
use libc::{c_char, c_int, c_long, c_schar, c_uint, c_ulong, c_ushort, c_void};
extern "C" {
    #[no_mangle]
    pub fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: c_uint,
        __function: *const c_char,
    ) -> !;
    #[no_mangle]
    pub fn _setjmp(_: *mut __jmp_buf_tag) -> c_int;

    #[no_mangle]
    pub fn clock() -> clock_t;

    #[no_mangle]
    pub fn longjmp(_: *mut __jmp_buf_tag, _: c_int) -> !;

    #[no_mangle]
    pub static mut stdout: *mut FILE;

    #[no_mangle]
    pub static mut stderr: *mut FILE;

    #[no_mangle]
    pub static mut stdin: *mut FILE;

    #[no_mangle]
    pub fn strncmp(_: *const c_char, _: *const c_char, _: c_ulong) -> c_int;

    #[no_mangle]
    pub fn fclose(__stream: *mut FILE) -> c_int;

    #[no_mangle]
    pub fn fopen(_: *const c_char, _: *const c_char) -> *mut FILE;

    #[no_mangle]
    pub fn fread(_: *mut c_void, _: c_ulong, _: c_ulong, _: *mut FILE) -> c_ulong;

    #[no_mangle]
    pub fn setvbuf(__stream: *mut FILE, __buf: *mut c_char, __modes: c_int, __n: size_t) -> c_int;

    #[no_mangle]
    pub fn putc(__c: c_int, __stream: *mut FILE) -> c_int;

    #[no_mangle]
    pub fn fputs(__s: *const c_char, __stream: *mut FILE) -> c_int;
    #[no_mangle]
    pub fn qsort(__base: *mut c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);

    #[no_mangle]
    pub fn fprintf(_: *mut FILE, _: *const c_char, _: ...) -> c_int;
    #[no_mangle]
    pub fn malloc(_: c_ulong) -> *mut c_void;

    #[no_mangle]
    pub fn calloc(_: c_ulong, _: c_ulong) -> *mut c_void;

    #[no_mangle]
    pub fn realloc(_: *mut c_void, _: c_ulong) -> *mut c_void;
    #[no_mangle]
    pub fn memcpy(_: *mut c_void, _: *const c_void, _: c_ulong) -> *mut c_void;

    #[no_mangle]
    pub fn memmove(_: *mut c_void, _: *const c_void, _: c_ulong) -> *mut c_void;

    #[no_mangle]
    pub fn memset(_: *mut c_void, _: c_int, _: c_ulong) -> *mut c_void;

    #[no_mangle]
    pub fn memcmp(_: *const c_void, _: *const c_void, _: c_ulong) -> c_int;

    #[no_mangle]
    pub fn strlen(_: *const c_char) -> c_ulong;
    #[no_mangle]
    pub fn fstat(__fd: c_int, __buf: *mut stat) -> c_int;
    #[no_mangle]
    pub fn read(__fd: c_int, __buf: *mut c_void, __nbytes: size_t) -> ssize_t;

    #[no_mangle]
    pub fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    );
}

// We never touch the fields using these types, and they're all hidden behind pointers.
// So we're using our own opaque struct (in the manner recommended by the nomicon) to
// avoid using the extern_types nightly feature.
#[repr(C)] pub struct _IO_marker { _private: [u8; 0] }
#[repr(C)] pub struct _IO_codecvt { _private: [u8; 0] }
#[repr(C)] pub struct _IO_wide_data { _private: [u8; 0] }

pub const __ASSERT_FUNCTION: [c_char; 46] = unsafe {
    *::std::mem::transmute::<&[u8; 46], &[c_char; 46]>(
        b"void attributeValue(FILE *, const XML_Char *)\x00",
    )
};
pub type __compar_fn_t = Option<unsafe extern "C" fn(_: *const c_void, _: *const c_void) -> c_int>;

// =============== BEGIN FILE_h ================
pub type FILE = _IO_FILE;
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
// =============== BEGIN stat_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub const __S_IFMT: c_int = 0o170000;
// ================ END stat_h ================
// =============== BEGIN stdint_uintn_h ================
pub type uint64_t = __uint64_t;
// ================ END stdint_uintn_h ================
// =============== BEGIN stdio_h ================
pub type ssize_t = __ssize_t;
// ================ END stdio_h ================
// =============== BEGIN struct_FILE_h ================
pub type _IO_lock_t = ();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: __off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub __pad5: size_t,
    pub _mode: c_int,
    pub _unused2: [c_char; 20],
}
// ================ END struct_FILE_h ================
// =============== BEGIN time_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timezone {
    pub tz_minuteswest: c_int,
    pub tz_dsttime: c_int,
}
pub type __timezone_ptr_t = *mut timezone;
// ================ END time_h ================
// =============== BEGIN types_h ================
pub type __uint64_t = c_ulong;
pub type __dev_t = c_ulong;
pub type __uid_t = c_uint;
pub type __gid_t = c_uint;
pub type __ino_t = c_ulong;
pub type __mode_t = c_uint;
pub type __nlink_t = c_ulong;
pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type __clock_t = c_long;
pub type __pid_t = c_int;
pub type __time_t = c_long;
pub type __suseconds_t = c_long;
pub type __blksize_t = c_long;
pub type __blkcnt_t = c_long;
pub type __ssize_t = c_long;
pub type __syscall_slong_t = c_long;
pub type intptr_t = c_long;

pub type clock_t = __clock_t;
pub const CLOCKS_PER_SEC: c_int = 1000000 as c_int;
pub const _STAT_VER_LINUX: c_int = 1 as c_int;
pub const _STAT_VER: c_int = _STAT_VER_LINUX;
