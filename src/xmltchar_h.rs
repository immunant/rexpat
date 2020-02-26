use crate::stdlib::{fprintf, fputs, putc, strlen};
use ::libc::{open, fopen, perror, remove, strcat, strchr, strcmp, strcpy, strrchr, FILE};
use libc::{c_char, c_int, c_ulong};
pub const ftprintf: unsafe extern "C" fn(_: *mut FILE, _: *const c_char, _: ...) -> c_int = fprintf;
pub const tcscpy: unsafe extern "C" fn(_: *mut c_char, _: *const c_char) -> *mut c_char = strcpy;
pub const tcsrchr: unsafe extern "C" fn(_: *const c_char, _: c_int) -> *mut c_char = strrchr;
pub const tcslen: unsafe extern "C" fn(_: *const c_char) -> c_ulong = strlen;
pub const tperror: unsafe extern "C" fn(_: *const c_char) -> () = perror;
pub const topen: unsafe extern "C" fn(_: *const c_char, _: c_int, _: ...) -> c_int = open;
pub const tfopen: unsafe extern "C" fn(_: *const c_char, _: *const c_char) -> *mut FILE = fopen;

pub const fputts: unsafe extern "C" fn(_: *const c_char, _: *mut FILE) -> c_int = fputs;

pub const puttc: unsafe extern "C" fn(_: c_int, _: *mut FILE) -> c_int = putc;

pub const tcscmp: unsafe extern "C" fn(_: *const c_char, _: *const c_char) -> c_int = strcmp;

pub const tcscat: unsafe extern "C" fn(_: *mut c_char, _: *const c_char) -> *mut c_char = strcat;

pub const tcschr: unsafe extern "C" fn(_: *const c_char, _: c_int) -> *mut c_char = strchr;

pub const tremove: unsafe extern "C" fn(_: *const c_char) -> c_int = remove;
