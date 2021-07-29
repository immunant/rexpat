/* Copyright (c) 2020 Immunant, Inc. */
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(main)]
#![feature(const_in_array_repeat_expressions)]
#![feature(ptr_wrapping_offset_from)]
#![feature(try_reserve)]
#![feature(alloc_layout_extra)]
// Required by !Send and !Sync
#![feature(optin_builtin_traits)]

#[cfg(all(feature = "unicode_wchar_t", not(target_os = "windows")))]
compile_error!("Feature \"unicode_wchar_t\" is only supported on windows");

extern crate libc;
#[macro_use]
extern crate rental;

pub mod ascii_h;
pub mod expat_external_h;
pub mod expat_h;
pub mod filemap_h;
pub mod stdbool_h;
pub mod stdlib;
pub mod string_pool;
pub mod xmltchar_h;
pub mod xmltok_impl_h;

mod fallible_rc;

pub mod lib {
    pub mod nametab;
    pub mod xmlparse;
    pub mod xmlrole;
    pub mod xmltok;
    pub mod xmltok_impl;
}
