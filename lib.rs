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

#[path = "src/ascii_h.rs"]
pub mod ascii_h;
#[path = "src/ascii_h_0.rs"]
pub mod ascii_h_0;
#[path = "src/expat_config_h.rs"]
pub mod expat_config_h;
#[path = "src/expat_external_h.rs"]
pub mod expat_external_h;
#[path = "src/expat_h.rs"]
pub mod expat_h;
#[path = "src/filemap_h.rs"]
pub mod filemap_h;
#[path = "src/internal.rs"]
pub mod internal;
#[path = "src/siphash_h.rs"]
pub mod siphash_h;
#[path = "src/stdbool_h.rs"]
pub mod stdbool_h;
#[path = "src/stddef_h.rs"]
pub mod stddef_h;
#[path = "src/stdlib.rs"]
pub mod stdlib;
#[path = "src/xmltchar_h.rs"]
pub mod xmltchar_h;
#[path = "src/xmltok_h.rs"]
pub mod xmltok_h;
#[path = "src/xmltok_impl_c.rs"]
pub mod xmltok_impl_c;
#[path = "src/xmltok_impl_h.rs"]
pub mod xmltok_impl_h;
extern crate libc;

pub mod src {
    pub mod lib {
        pub mod xmlparse;
        pub mod xmlrole;
        pub mod xmltok;
    } // mod lib
} // mod src
