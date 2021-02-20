use slimjs::quickjs::*;

use std::ffi::CString;

fn make_cstring(value: impl Into<Vec<u8>>) -> CString {
    CString::new(value).unwrap()
}

fn main() {
    let code = "\"abc\".length";

    unsafe {
        let rt = JS_NewRuntime();
        let ctx = JS_NewContext(rt);

        let filename = "script.js";
        let filename_c = make_cstring(filename);
        let code_c = make_cstring(code);

        let value_raw = JS_Eval(
            ctx,
            code_c.as_ptr(),
            code.len() as _,
            filename_c.as_ptr(),
            JS_EVAL_TYPE_GLOBAL as i32,
        );

        if value_raw.tag == 0 {
            let ptr = unsafe { JS_ToCStringLen2(ctx, std::ptr::null_mut(), value_raw, 0) };

            if ptr.is_null() {
                panic!("null ptr");
            }

            let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };

            let s = cstr
                .to_str()
                .unwrap()
                .to_string();

            eprintln!("error: {}", s);
        }

        dbg!(value_raw);
    }
}
