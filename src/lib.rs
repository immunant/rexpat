#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

extern crate libc;

pub mod ascii_h;
pub mod expat_config_h;
pub mod expat_external_h;
pub mod expat_h;
pub mod filemap_h;
pub mod internal;
pub mod siphash_h;
pub mod stdbool_h;
pub mod stddef_h;
pub mod stdlib;
pub mod xmltchar_h;
pub mod xmltok_h;
pub mod xmltok_impl_c;
pub mod xmltok_impl_h;

pub mod lib {
    pub mod xmlparse;
    pub mod xmlrole;
    pub mod xmltok;

    #[cfg(feature = "mozilla")]
    pub mod moz_extensions;
}
