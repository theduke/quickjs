use slimjs::quickjs::*;

use std::ffi::CString;

fn make_cstring(value: impl Into<Vec<u8>>) -> CString {
    CString::new(value).unwrap()
}

#[test]
fn eval() {
    let code = "1 + 1";

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

        dbg!(value_raw.tag);
    }
}
