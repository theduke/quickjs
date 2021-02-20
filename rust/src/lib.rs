#![allow(dead_code)]
#![allow(unused)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;

pub mod cutils;
pub mod libbf;
pub mod libregexp;
pub mod libunicode;
pub mod quickjs;
pub mod quickjs_libc;
