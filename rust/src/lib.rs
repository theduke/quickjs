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
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![feature(untagged_unions)]
#![feature(llvm_asm)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
#[macro_use]
extern crate c2rust_asm_casts;

mod cutils;
mod libbf;
mod libregexp;
mod libunicode;
mod quickjs;

pub use quickjs::{
    JSValue, JS_Eval, JS_NewContext, JS_NewRuntime, JS_ToCStringLen2, JS_EVAL_TYPE_GLOBAL,
    JS_TAG_EXCEPTION, JS_TAG_STRING,
};

// #[cfg(not(target_arch = "wasm32"))]
// pub mod quickjs_libc;
