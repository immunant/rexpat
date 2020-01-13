extern "C" {
    #[no_mangle]
    pub fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    pub fn _setjmp(_: *mut crate::stdlib::__jmp_buf_tag) -> libc::c_int;

    #[no_mangle]
    pub fn longjmp(_: *mut crate::stdlib::__jmp_buf_tag, _: libc::c_int) -> !;
    #[no_mangle]
    pub fn getrandom(
        __buffer: *mut libc::c_void,
        __length: crate::stddef_h::size_t,
        __flags: libc::c_uint,
    ) -> crate::stdlib::ssize_t;
    #[no_mangle]
    pub static mut stdout: *mut crate::stdlib::FILE;

    #[no_mangle]
    pub static mut stderr: *mut crate::stdlib::FILE;

    #[no_mangle]
    pub fn fclose(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

    #[no_mangle]
    pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut crate::stdlib::FILE;

    #[no_mangle]
    pub fn setvbuf(
        __stream: *mut crate::stdlib::FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: crate::stddef_h::size_t,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn putc(__c: libc::c_int, __stream: *mut crate::stdlib::FILE) -> libc::c_int;

    #[no_mangle]
    pub fn fputs(__s: *const libc::c_char, __stream: *mut crate::stdlib::FILE) -> libc::c_int;
    #[no_mangle]
    pub fn qsort(
        __base: *mut libc::c_void,
        __nmemb: crate::stddef_h::size_t,
        __size: crate::stddef_h::size_t,
        __compar: crate::stdlib::__compar_fn_t,
    );

    #[no_mangle]
    pub fn fprintf(_: *mut crate::stdlib::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

    #[no_mangle]
    pub fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;

    #[no_mangle]
    pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    pub fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

    #[no_mangle]
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    pub type _IO_marker;

    pub type _IO_codecvt;

    pub type _IO_wide_data;
    #[no_mangle]
    pub fn fstat(__fd: libc::c_int, __buf: *mut crate::stdlib::stat) -> libc::c_int;
    #[no_mangle]
    pub fn gettimeofday(
        __tv: *mut ::libc::timeval,
        __tz: crate::stdlib::__timezone_ptr_t,
    ) -> libc::c_int;
    #[no_mangle]
    pub fn read(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: crate::stddef_h::size_t,
    ) -> crate::stdlib::ssize_t;
}
pub const __ASSERT_FUNCTION: [libc::c_char; 46] = unsafe {
    *::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
        b"void attributeValue(FILE *, const XML_Char *)\x00",
    )
};
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;

// =============== BEGIN FILE_h ================
pub type FILE = crate::stdlib::_IO_FILE;
// ================ END FILE_h ================
// =============== BEGIN __sigset_t_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
// ================ END __sigset_t_h ================
// =============== BEGIN include_setjmp_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: crate::stdlib::__jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: crate::stdlib::__sigset_t,
}
pub type jmp_buf = [crate::stdlib::__jmp_buf_tag; 1];
// ================ END include_setjmp_h ================
// =============== BEGIN random_h ================
pub const GRND_NONBLOCK: libc::c_int = 0x1 as libc::c_int;
// ================ END random_h ================
// =============== BEGIN setjmp_h ================
pub type __jmp_buf = [libc::c_long; 8];
// ================ END setjmp_h ================
// =============== BEGIN stat_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: crate::stdlib::__dev_t,
    pub st_ino: crate::stdlib::__ino_t,
    pub st_nlink: crate::stdlib::__nlink_t,
    pub st_mode: crate::stdlib::__mode_t,
    pub st_uid: crate::stdlib::__uid_t,
    pub st_gid: crate::stdlib::__gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: crate::stdlib::__dev_t,
    pub st_size: crate::stdlib::__off_t,
    pub st_blksize: crate::stdlib::__blksize_t,
    pub st_blocks: crate::stdlib::__blkcnt_t,
    pub st_atim: ::libc::timespec,
    pub st_mtim: ::libc::timespec,
    pub st_ctim: ::libc::timespec,
    pub __glibc_reserved: [crate::stdlib::__syscall_slong_t; 3],
}
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
// ================ END stat_h ================
// =============== BEGIN stdint_uintn_h ================
pub type uint64_t = crate::stdlib::__uint64_t;
// ================ END stdint_uintn_h ================
// =============== BEGIN stdio_h ================
pub type ssize_t = crate::stdlib::__ssize_t;
// ================ END stdio_h ================
// =============== BEGIN struct_FILE_h ================
pub type _IO_lock_t = ();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut crate::stdlib::_IO_marker,
    pub _chain: *mut crate::stdlib::_IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: crate::stdlib::__off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: crate::stdlib::__off64_t,
    pub _codecvt: *mut crate::stdlib::_IO_codecvt,
    pub _wide_data: *mut crate::stdlib::_IO_wide_data,
    pub _freeres_list: *mut crate::stdlib::_IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: crate::stddef_h::size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
// ================ END struct_FILE_h ================
// =============== BEGIN time_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut crate::stdlib::timezone;
// ================ END time_h ================
// =============== BEGIN types_h ================
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
