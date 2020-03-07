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
#![feature(const_transmute)]
#![feature(main)]
#![feature(const_in_array_repeat_expressions)]
#![feature(ptr_wrapping_offset_from)]
#![feature(try_reserve)]
#![feature(alloc_layout_extra)]

#[cfg(all(feature = "unicode_wchar_t", not(target_os = "windows")))]
compile_error!("Feature \"unicode_wchar_t\" is only supported on windows");

extern crate libc;

pub mod ascii_h;
pub mod expat_external_h;
pub mod expat_h;
pub mod filemap_h;
pub mod stdbool_h;
pub mod stdlib;
pub mod xmltchar_h;
pub mod xmltok_impl_h;

pub mod lib {
    pub mod nametab;
    pub mod xmlparse;
    pub mod xmlrole;
    pub mod xmltok;
    pub mod xmltok_impl;
}
