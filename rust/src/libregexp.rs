use ::c2rust_bitfields;

use std::process::abort;

use crate::cutils::{
    cstr_append_sized, cstr_compare, cstr_find_char, cstr_len, dbuf_error, dbuf_free, dbuf_init2,
    dbuf_put, dbuf_put_self, dbuf_put_u16, dbuf_put_u32, dbuf_putc, dbuf_realloc, pstrcpy,
    ptr_compare, DynBuf, PtrExt, BOOL, FALSE, TRUE,
};

use crate::libunicode::{
    cr_free, cr_init, cr_invert, cr_op, cr_realloc, cr_union1, lre_case_conv, lre_is_id_continue,
    lre_is_id_start, unicode_from_utf8, unicode_general_category, unicode_prop, unicode_script,
    unicode_to_utf8, CharRange, CharRangeOp,
};

use crate::quickjs::{lre_check_stack_overflow, lre_realloc};

pub type intptr_t = isize;
pub type uintptr_t = usize;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct packed_u32 {
    pub v: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct packed_u16 {
    pub v: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct REParseState {
    pub byte_code: DynBuf,
    pub buf_ptr: *const u8,
    pub buf_end: *const u8,
    pub buf_start: *const u8,
    pub re_flags: i32,
    pub is_utf16: BOOL,
    pub ignore_case: BOOL,
    pub dotall: BOOL,
    pub capture_count: i32,
    pub total_capture_count: i32,
    pub has_named_captures: i32,
    pub opaque: *mut std::ffi::c_void,
    pub group_names: DynBuf,
    pub u: REParseStateUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union REParseStateUnion {
    pub error_msg: [std::os::raw::c_char; 128],
    pub tmp_buf: [std::os::raw::c_char; 128],
}
pub const REOP_range32: C2RustUnnamed_2 = 22;
pub const REOP_range: C2RustUnnamed_2 = 21;
pub const REOP_bne_char_pos: C2RustUnnamed_2 = 26;
pub const REOP_drop: C2RustUnnamed_2 = 16;
pub const REOP_push_char_pos: C2RustUnnamed_2 = 25;
pub const REOP_push_i32: C2RustUnnamed_2 = 15;
pub const REOP_COUNT: C2RustUnnamed_2 = 29;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct REOpCode {
    pub size: u8,
}
pub const REOP_match: C2RustUnnamed_2 = 10;
pub const REOP_save_end: C2RustUnnamed_2 = 12;
pub const REOP_loop: C2RustUnnamed_2 = 14;
pub const REOP_split_goto_first: C2RustUnnamed_2 = 8;
pub const REOP_goto: C2RustUnnamed_2 = 7;
pub const REOP_split_next_first: C2RustUnnamed_2 = 9;
pub const REOP_save_reset: C2RustUnnamed_2 = 13;
pub const REOP_backward_back_reference: C2RustUnnamed_2 = 20;
pub const REOP_back_reference: C2RustUnnamed_2 = 19;
pub const REOP_save_start: C2RustUnnamed_2 = 11;
pub const REOP_prev: C2RustUnnamed_2 = 27;
pub const REOP_not_word_boundary: C2RustUnnamed_2 = 18;
pub const REOP_word_boundary: C2RustUnnamed_2 = 17;
pub const REOP_line_end: C2RustUnnamed_2 = 6;
pub const REOP_line_start: C2RustUnnamed_2 = 5;
pub const REOP_any: C2RustUnnamed_2 = 4;
pub const REOP_dot: C2RustUnnamed_2 = 3;
pub const REOP_char32: C2RustUnnamed_2 = 2;
pub const REOP_char: C2RustUnnamed_2 = 1;
pub const REOP_simple_greedy_quant: C2RustUnnamed_2 = 28;
pub const CHAR_RANGE_W: C2RustUnnamed_3 = 5;
pub const CHAR_RANGE_w: C2RustUnnamed_3 = 4;
pub const CHAR_RANGE_S: C2RustUnnamed_3 = 3;
pub const CHAR_RANGE_s: C2RustUnnamed_3 = 2;
pub const CHAR_RANGE_D: C2RustUnnamed_3 = 1;
pub const CHAR_RANGE_d: C2RustUnnamed_3 = 0;
pub const REOP_lookahead: C2RustUnnamed_2 = 23;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct REExecContext {
    pub cbuf: *const u8,
    pub cbuf_end: *const u8,
    pub cbuf_type: i32,
    pub capture_count: i32,
    pub stack_size_max: i32,
    pub multi_line: BOOL,
    pub ignore_case: BOOL,
    pub is_utf16: BOOL,
    pub opaque: *mut std::ffi::c_void,
    pub state_size: usize,
    pub state_stack: *mut u8,
    pub state_stack_size: usize,
    pub state_stack_len: usize,
}
pub type StackInt = uintptr_t;
pub type REExecStateEnum = u32;
pub const RE_EXEC_STATE_GREEDY_QUANT: REExecStateEnum = 3;
pub const RE_EXEC_STATE_NEGATIVE_LOOKAHEAD: REExecStateEnum = 2;
pub const RE_EXEC_STATE_LOOKAHEAD: REExecStateEnum = 1;
pub const RE_EXEC_STATE_SPLIT: REExecStateEnum = 0;

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct REExecState {
    #[bitfield(name = "type_0", ty = "REExecStateEnum", bits = "0..=7")]
    pub type_0: [u8; 1],
    pub stack_len: u8,
    pub count: usize,
    pub cptr: *const u8,
    pub pc: *const u8,
    pub buf: [*mut std::ffi::c_void; 0],
}
pub const REOP_negative_lookahead: C2RustUnnamed_2 = 24;
pub type C2RustUnnamed_2 = u32;
pub const REOP_invalid: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = u32;
#[inline]
unsafe fn get_u32(mut tab: *const u8) -> u32 {
    return (*(tab as *const packed_u32)).v;
}
#[inline]
unsafe fn put_u32(mut tab: *mut u8, mut val: u32) {
    (*(tab as *mut packed_u32)).v = val;
}
#[inline]
unsafe fn get_u16(mut tab: *const u8) -> u32 {
    return (*(tab as *const packed_u16)).v as u32;
}
#[inline]
unsafe fn from_hex(mut c: i32) -> i32 {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    } else if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 10 as i32;
    } else if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 10 as i32;
    } else {
        return -(1 as i32);
    };
}
#[inline]
unsafe fn lre_js_is_ident_next(mut c: i32) -> i32 {
    if (c as u32) < 128 as i32 as u32 {
        return (lre_id_continue_table_ascii[(c >> 5 as i32) as usize] >> (c & 31 as i32)
            & 1 as i32 as u32) as i32;
    } else {
        return (lre_is_id_continue(c as u32) != 0 || c == 0x200c as i32 || c == 0x200d as i32)
            as i32;
    };
}
#[inline]
unsafe fn lre_js_is_ident_first(mut c: i32) -> i32 {
    if (c as u32) < 128 as i32 as u32 {
        return (lre_id_start_table_ascii[(c >> 5 as i32) as usize] >> (c & 31 as i32)
            & 1 as i32 as u32) as i32;
    } else {
        return lre_is_id_start(c as u32);
    };
}
#[inline]
unsafe fn cr_add_point(mut cr: *mut CharRange, mut v: u32) -> i32 {
    if (*cr).len >= (*cr).size {
        if cr_realloc(cr, (*cr).len + 1 as i32) != 0 {
            return -(1 as i32);
        }
    }
    let fresh0 = (*cr).len;
    (*cr).len = (*cr).len + 1;
    *(*cr).points.offset(fresh0 as isize) = v;
    return 0 as i32;
}
#[inline]
unsafe fn cr_union_interval(mut cr: *mut CharRange, mut c1: u32, mut c2: u32) -> i32 {
    let mut b_pt: [u32; 2] = [0; 2];
    b_pt[0 as i32 as usize] = c1;
    b_pt[1 as i32 as usize] = c2.wrapping_add(1 as i32 as u32);
    return cr_union1(cr, b_pt.as_mut_ptr(), 2 as i32);
}
static mut reopcode_info: [REOpCode; 29] = [
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 3 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 2 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 2 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 3 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 2 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 2 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 3 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 3 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 5 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 1 as i32 as u8,
        };
        init
    },
    {
        let mut init = REOpCode {
            size: 17 as i32 as u8,
        };
        init
    },
];
#[inline]
unsafe fn is_digit(mut c: i32) -> i32 {
    return (c >= '0' as i32 && c <= '9' as i32) as i32;
}
/* insert 'len' bytes at position 'pos'. Return < 0 if error. */
unsafe fn dbuf_insert(mut s: *mut DynBuf, mut pos: i32, mut len: i32) -> i32 {
    if dbuf_realloc(s, (*s).size.wrapping_add(len as usize)) != 0 {
        return -1;
    }

    ((*s).buf.offset(pos as isize).offset(len as isize) as *mut u8).copy_from_nonoverlapping(
        (*s).buf.offset(pos as isize) as *const u8,
        (*s).size.wrapping_sub(pos as usize),
    );
    (*s).size = ((*s).size).wrapping_add(len as usize);
    return 0 as i32;
}
/* canonicalize with the specific JS regexp rules */
unsafe fn lre_canonicalize(mut c: u32, mut is_utf16: BOOL) -> u32 {
    let mut res: [u32; 3] = [0; 3];
    let mut len: i32 = 0;
    if is_utf16 != 0 {
        if (c < 128 as i32 as u32) as i32 as i64 != 0 {
            if c >= 'A' as i32 as u32 && c <= 'Z' as i32 as u32 {
                c = c
                    .wrapping_sub('A' as i32 as u32)
                    .wrapping_add('a' as i32 as u32)
            }
        } else {
            lre_case_conv(res.as_mut_ptr(), c, 2 as i32);
            c = res[0 as i32 as usize]
        }
    } else if (c < 128 as i32 as u32) as i32 as i64 != 0 {
        if c >= 'a' as i32 as u32 && c <= 'z' as i32 as u32 {
            c = c
                .wrapping_sub('a' as i32 as u32)
                .wrapping_add('A' as i32 as u32)
        }
    } else {
        /* legacy regexp: to upper case if single char >= 128 */
        len = lre_case_conv(res.as_mut_ptr(), c, FALSE as i32);
        if len == 1 as i32 && res[0 as i32 as usize] >= 128 as i32 as u32 {
            c = res[0 as i32 as usize]
        }
    }
    return c;
}
static mut char_range_d: [u16; 3] = [
    1 as i32 as u16,
    0x30 as i32 as u16,
    (0x39 as i32 + 1 as i32) as u16,
];
/* code point ranges for Zs,Zl or Zp property */
static mut char_range_s: [u16; 21] = [
    10 as i32 as u16,
    0x9 as i32 as u16,
    (0xd as i32 + 1 as i32) as u16,
    0x20 as i32 as u16,
    (0x20 as i32 + 1 as i32) as u16,
    0xa0 as i32 as u16,
    (0xa0 as i32 + 1 as i32) as u16,
    0x1680 as i32 as u16,
    (0x1680 as i32 + 1 as i32) as u16,
    0x2000 as i32 as u16,
    (0x200a as i32 + 1 as i32) as u16,
    0x2028 as i32 as u16,
    (0x2029 as i32 + 1 as i32) as u16,
    0x202f as i32 as u16,
    (0x202f as i32 + 1 as i32) as u16,
    0x205f as i32 as u16,
    (0x205f as i32 + 1 as i32) as u16,
    0x3000 as i32 as u16,
    (0x3000 as i32 + 1 as i32) as u16,
    0xfeff as i32 as u16,
    (0xfeff as i32 + 1 as i32) as u16,
];
#[no_mangle]
pub unsafe fn lre_is_space(mut c: i32) -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut low: i32 = 0;
    let mut high: i32 = 0;
    n = (::std::mem::size_of::<[u16; 21]>() as u64)
        .wrapping_div(::std::mem::size_of::<u16>() as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_div(2 as i32 as u64) as i32;
    i = 0 as i32;
    while i < n {
        low = char_range_s[(2 as i32 * i + 1 as i32) as usize] as i32;
        if c < low {
            return FALSE as i32;
        }
        high = char_range_s[(2 as i32 * i + 2 as i32) as usize] as i32;
        if c < high {
            return TRUE as i32;
        }
        i += 1
    }
    return FALSE as i32;
}
#[no_mangle]
pub static mut lre_id_start_table_ascii: [u32; 4] = [
    0 as i32 as u32,
    0x10 as i32 as u32,
    0x87fffffe as u32,
    0x7fffffe as i32 as u32,
];
#[no_mangle]
pub static mut lre_id_continue_table_ascii: [u32; 4] = [
    0 as i32 as u32,
    0x3ff0010 as i32 as u32,
    0x87fffffe as u32,
    0x7fffffe as i32 as u32,
];
static mut char_range_w: [u16; 9] = [
    4 as i32 as u16,
    0x30 as i32 as u16,
    (0x39 as i32 + 1 as i32) as u16,
    0x41 as i32 as u16,
    (0x5a as i32 + 1 as i32) as u16,
    0x5f as i32 as u16,
    (0x5f as i32 + 1 as i32) as u16,
    0x61 as i32 as u16,
    (0x7a as i32 + 1 as i32) as u16,
];
static mut char_range_table: [*const u16; 3] = unsafe {
    [
        char_range_d.as_ptr(),
        char_range_s.as_ptr(),
        char_range_w.as_ptr(),
    ]
};
unsafe fn cr_init_char_range(mut s: *mut REParseState, mut cr: *mut CharRange, mut c: u32) -> i32 {
    let mut current_block: u64;
    let mut invert: BOOL = 0;
    let mut c_pt: *const u16 = 0 as *const u16;
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    invert = (c & 1 as i32 as u32) as BOOL;
    c_pt = char_range_table[(c >> 1 as i32) as usize];
    let fresh1 = c_pt;
    c_pt = c_pt.offset(1);
    len = *fresh1 as i32;
    cr_init(
        cr,
        (*s).opaque,
        Some(
            lre_realloc
                as unsafe fn(
                    _: *mut std::ffi::c_void,
                    _: *mut std::ffi::c_void,
                    _: usize,
                ) -> *mut std::ffi::c_void,
        ),
    );
    i = 0 as i32;
    loop {
        if !(i < len * 2 as i32) {
            current_block = 13513818773234778473;
            break;
        }
        if cr_add_point(cr, *c_pt.offset(i as isize) as u32) != 0 {
            current_block = 10474390507374119221;
            break;
        }
        i += 1
    }
    match current_block {
        13513818773234778473 => {
            if invert != 0 {
                if cr_invert(cr) != 0 {
                    current_block = 10474390507374119221;
                } else {
                    current_block = 2968425633554183086;
                }
            } else {
                current_block = 2968425633554183086;
            }
            match current_block {
                10474390507374119221 => {}
                _ => return 0 as i32,
            }
        }
        _ => {}
    }
    cr_free(cr);
    return -(1 as i32);
}
unsafe fn cr_canonicalize(mut cr: *mut CharRange) -> i32 {
    let mut a: CharRange = CharRange {
        len: 0,
        size: 0,
        points: 0 as *mut u32,
        mem_opaque: 0 as *mut std::ffi::c_void,
        realloc_func: None,
    };
    let mut pt: [u32; 2] = [0; 2];
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    cr_init(
        &mut a,
        (*cr).mem_opaque,
        Some(
            lre_realloc
                as unsafe fn(
                    _: *mut std::ffi::c_void,
                    _: *mut std::ffi::c_void,
                    _: usize,
                ) -> *mut std::ffi::c_void,
        ),
    );
    pt[0 as i32 as usize] = 'a' as i32 as u32;
    pt[1 as i32 as usize] = ('z' as i32 + 1 as i32) as u32;
    ret = cr_op(
        &mut a,
        (*cr).points,
        (*cr).len,
        pt.as_mut_ptr(),
        2 as i32,
        crate::libunicode::CR_OP_INTER as i32,
    );
    if !(ret != 0) {
        /* convert to upper case */
        /* XXX: the generic unicode case would be much more complicated
        and not really useful */
        i = 0 as i32;
        while i < a.len {
            let ref mut fresh2 = *a.points.offset(i as isize);
            *fresh2 = (*fresh2 as u32).wrapping_add(('A' as i32 - 'a' as i32) as u32) as u32 as u32;
            i += 1
        }
        /* Note: for simplicity we keep the lower case ranges */
        ret = cr_union1(cr, a.points, a.len)
    }
    cr_free(&mut a);
    return ret;
}
unsafe fn re_emit_op(mut s: *mut REParseState, mut op: i32) {
    dbuf_putc(&mut (*s).byte_code, op as u8);
}
/* return the offset of the u32 value */
unsafe fn re_emit_op_u32(mut s: *mut REParseState, mut op: i32, mut val: u32) -> i32 {
    let mut pos: i32 = 0;
    dbuf_putc(&mut (*s).byte_code, op as u8);
    pos = (*s).byte_code.size as i32;
    dbuf_put_u32(&mut (*s).byte_code, val);
    return pos;
}
unsafe fn re_emit_goto(mut s: *mut REParseState, mut op: i32, mut val: u32) -> i32 {
    let mut pos: i32 = 0;
    dbuf_putc(&mut (*s).byte_code, op as u8);
    pos = (*s).byte_code.size as i32;
    dbuf_put_u32(
        &mut (*s).byte_code,
        val.wrapping_sub((pos + 4 as i32) as u32),
    );
    return pos;
}
unsafe fn re_emit_op_u8(mut s: *mut REParseState, mut op: i32, mut val: u32) {
    dbuf_putc(&mut (*s).byte_code, op as u8);
    dbuf_putc(&mut (*s).byte_code, val as u8);
}
unsafe fn re_emit_op_u16(mut s: *mut REParseState, mut op: i32, mut val: u32) {
    dbuf_putc(&mut (*s).byte_code, op as u8);
    dbuf_put_u16(&mut (*s).byte_code, val as u16);
}
unsafe fn re_parse_error(mut s: *mut REParseState, msg: &str) -> i32 {
    cstr_append_sized((*s).u.error_msg.as_mut_ptr(), 128, msg);
    -1
}

unsafe fn re_parse_out_of_memory(mut s: *mut REParseState) -> i32 {
    return re_parse_error(s, "out of memory");
}

/* If allow_overflow is false, return -1 in case of
overflow. Otherwise return INT32_MAX. */
unsafe fn parse_digits(mut pp: *mut *const u8, mut allow_overflow: BOOL) -> i32 {
    let mut p: *const u8 = 0 as *const u8;
    let mut v: u64 = 0;
    let mut c: i32 = 0;
    p = *pp;
    v = 0 as i32 as u64;
    loop {
        c = *p as i32;
        if c < '0' as i32 || c > '9' as i32 {
            break;
        }
        v = v
            .wrapping_mul(10 as i32 as u64)
            .wrapping_add(c as u64)
            .wrapping_sub('0' as i32 as u64);
        if v >= 2147483647 as i32 as u64 {
            if allow_overflow != 0 {
                v = 2147483647 as i32 as u64
            } else {
                return -(1 as i32);
            }
        }
        p = p.offset(1)
    }
    *pp = p;
    return v as i32;
}
unsafe fn re_parse_expect(mut s: *mut REParseState, mut pp: *mut *const u8, mut c: i32) -> i32 {
    let mut p: *const u8 = 0 as *const u8;
    p = *pp;
    if *p as i32 != c {
        return re_parse_error(
            s,
            &format!(
                "expecting '{}'",
                std::char::from_u32(c as u32).unwrap_or('?')
            ),
        );
    }
    p = p.offset(1);
    *pp = p;
    return 0 as i32;
}
/* Parse an escape sequence, *pp points after the '\':
allow_utf16 value:
0 : no UTF-16 escapes allowed
1 : UTF-16 escapes allowed
2 : UTF-16 escapes allowed and escapes of surrogate pairs are
converted to a unicode character (unicode regexp case).

Return the unicode char and update *pp if recognized,
return -1 if malformed escape,
return -2 otherwise. */
#[no_mangle]
pub unsafe fn lre_parse_escape(mut pp: *mut *const u8, mut allow_utf16: i32) -> i32 {
    let mut p: *const u8 = 0 as *const u8;
    let mut c: u32 = 0;
    p = *pp;
    let fresh3 = p;
    p = p.offset(1);
    c = *fresh3 as u32;
    match c {
        98 => c = '\u{8}' as i32 as u32,
        102 => c = '\u{c}' as i32 as u32,
        110 => c = '\n' as i32 as u32,
        114 => c = '\r' as i32 as u32,
        116 => c = '\t' as i32 as u32,
        118 => c = '\u{b}' as i32 as u32,
        120 | 117 => {
            let mut h: i32 = 0;
            let mut n: i32 = 0;
            let mut i: i32 = 0;
            let mut c1: u32 = 0;
            if *p as i32 == '{' as i32 && allow_utf16 != 0 {
                p = p.offset(1);
                c = 0 as i32 as u32;
                loop {
                    let fresh4 = p;
                    p = p.offset(1);
                    h = from_hex(*fresh4 as i32);
                    if h < 0 as i32 {
                        return -(1 as i32);
                    }
                    c = c << 4 as i32 | h as u32;
                    if c > 0x10ffff as i32 as u32 {
                        return -(1 as i32);
                    }
                    if *p as i32 == '}' as i32 {
                        break;
                    }
                }
                p = p.offset(1)
            } else {
                if c == 'x' as i32 as u32 {
                    n = 2 as i32
                } else {
                    n = 4 as i32
                }
                c = 0 as i32 as u32;
                i = 0 as i32;
                while i < n {
                    let fresh5 = p;
                    p = p.offset(1);
                    h = from_hex(*fresh5 as i32);
                    if h < 0 as i32 {
                        return -(1 as i32);
                    }
                    c = c << 4 as i32 | h as u32;
                    i += 1
                }
                if c >= 0xd800 as i32 as u32
                    && c < 0xdc00 as i32 as u32
                    && allow_utf16 == 2 as i32
                    && *p.offset(0 as i32 as isize) as i32 == '\\' as i32
                    && *p.offset(1 as i32 as isize) as i32 == 'u' as i32
                {
                    /* convert an escaped surrogate pair into a
                    unicode char */
                    c1 = 0 as i32 as u32;
                    i = 0 as i32;
                    while i < 4 as i32 {
                        h = from_hex(*p.offset((2 as i32 + i) as isize) as i32);
                        if h < 0 as i32 {
                            break;
                        }
                        c1 = c1 << 4 as i32 | h as u32;
                        i += 1
                    }
                    if i == 4 as i32 && c1 >= 0xdc00 as i32 as u32 && c1 < 0xe000 as i32 as u32 {
                        p = p.offset(6 as i32 as isize);
                        c = ((c & 0x3ff as i32 as u32) << 10 as i32 | c1 & 0x3ff as i32 as u32)
                            .wrapping_add(0x10000 as i32 as u32)
                    }
                }
            }
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
            c = (c as u32).wrapping_sub('0' as i32 as u32) as u32 as u32;
            if allow_utf16 == 2 as i32 {
                /* only accept \0 not followed by digit */
                if c != 0 as i32 as u32 || is_digit(*p as i32) != 0 {
                    return -(1 as i32);
                }
            } else {
                /* parse a legacy octal sequence */
                let mut v: u32 = 0;
                v = (*p as i32 - '0' as i32) as u32;
                if !(v > 7 as i32 as u32) {
                    c = c << 3 as i32 | v;
                    p = p.offset(1);
                    if !(c >= 32 as i32 as u32) {
                        v = (*p as i32 - '0' as i32) as u32;
                        if !(v > 7 as i32 as u32) {
                            c = c << 3 as i32 | v;
                            p = p.offset(1)
                        }
                    }
                }
            }
        }
        _ => return -(2 as i32),
    }
    *pp = p;
    return c as i32;
}
/* XXX: we use the same chars for name and value */
unsafe fn is_unicode_char(mut c: i32) -> BOOL {
    return (c >= '0' as i32 && c <= '9' as i32
        || c >= 'A' as i32 && c <= 'Z' as i32
        || c >= 'a' as i32 && c <= 'z' as i32
        || c == '_' as i32) as i32;
}
unsafe fn parse_unicode_property(
    mut s: *mut REParseState,
    mut cr: *mut CharRange,
    mut pp: *mut *const u8,
    mut is_inv: BOOL,
) -> i32 {
    let mut current_block: u64;
    let mut p: *const u8 = 0 as *const u8;
    let mut name: [std::os::raw::c_char; 64] = [0; 64];
    let mut value: [std::os::raw::c_char; 64] = [0; 64];
    let mut q: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut script_ext: BOOL = 0;
    let mut ret: i32 = 0;
    p = *pp;
    if *p as i32 != '{' as i32 {
        return re_parse_error(s, "expecting \'{\' after \\p");
    }
    p = p.offset(1);
    q = name.as_mut_ptr();
    loop {
        if !(is_unicode_char(*p as i32) != 0) {
            current_block = 2868539653012386629;
            break;
        }
        if q.wrapping_offset_from(name.as_mut_ptr()) as i64 as u64
            >= (::std::mem::size_of::<[std::os::raw::c_char; 64]>() as u64)
                .wrapping_sub(1 as i32 as u64)
        {
            current_block = 15694547443304570712;
            break;
        }
        let fresh6 = p;
        p = p.offset(1);
        let fresh7 = q;
        q = q.offset(1);
        *fresh7 = *fresh6 as std::os::raw::c_char
    }
    match current_block {
        2868539653012386629 => {
            *q = '\u{0}' as i32 as std::os::raw::c_char;
            q = value.as_mut_ptr();
            if *p as i32 == '=' as i32 {
                p = p.offset(1);
                while is_unicode_char(*p as i32) != 0 {
                    if q.wrapping_offset_from(value.as_mut_ptr()) as i64 as u64
                        >= (::std::mem::size_of::<[std::os::raw::c_char; 64]>() as u64)
                            .wrapping_sub(1 as i32 as u64)
                    {
                        return re_parse_error(s, "unknown unicode property value");
                    }
                    let fresh8 = p;
                    p = p.offset(1);
                    let fresh9 = q;
                    q = q.offset(1);
                    *fresh9 = *fresh8 as std::os::raw::c_char
                }
            }
            *q = '\u{0}' as i32 as std::os::raw::c_char;
            if *p as i32 != '}' as i32 {
                return re_parse_error(s, "expecting \'}\'");
            }
            p = p.offset(1);
            //    printf("name=%s value=%s\n", name, value);
            if cstr_compare(name.as_mut_ptr(), b"Script\x00" as *const u8 as *const i8) == 0
                || cstr_compare(name.as_mut_ptr(), b"sc\x00" as *const u8 as *const i8) == 0
            {
                script_ext = FALSE as i32;
                current_block = 11427802459928075752;
            } else if cstr_compare(
                name.as_mut_ptr(),
                b"Script_Extensions\x00" as *const u8 as *const i8,
            ) == 0
                || cstr_compare(
                    name.as_mut_ptr(),
                    b"scx\x00" as *const u8 as *const std::os::raw::c_char,
                ) == 0
            {
                script_ext = TRUE as i32;
                current_block = 11427802459928075752;
            } else if cstr_compare(
                name.as_mut_ptr(),
                b"General_Category\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0
                || cstr_compare(
                    name.as_mut_ptr(),
                    b"gc\x00" as *const u8 as *const std::os::raw::c_char,
                ) == 0
            {
                cr_init(
                    cr,
                    (*s).opaque,
                    Some(
                        lre_realloc
                            as unsafe fn(
                                _: *mut std::ffi::c_void,
                                _: *mut std::ffi::c_void,
                                _: usize,
                            ) -> *mut std::ffi::c_void,
                    ),
                );
                ret = unicode_general_category(cr, value.as_mut_ptr());
                if ret != 0 {
                    cr_free(cr);
                    if ret == -(2 as i32) {
                        return re_parse_error(s, "unknown unicode general category");
                    } else {
                        current_block = 13855183787650136026;
                    }
                } else {
                    current_block = 9353995356876505083;
                }
            } else if value[0 as i32 as usize] as i32 == '\u{0}' as i32 {
                cr_init(
                    cr,
                    (*s).opaque,
                    Some(
                        lre_realloc
                            as unsafe fn(
                                _: *mut std::ffi::c_void,
                                _: *mut std::ffi::c_void,
                                _: usize,
                            ) -> *mut std::ffi::c_void,
                    ),
                );
                ret = unicode_general_category(cr, name.as_mut_ptr());
                if ret == -(1 as i32) {
                    cr_free(cr);
                    current_block = 13855183787650136026;
                } else if ret < 0 as i32 {
                    ret = unicode_prop(cr, name.as_mut_ptr());
                    if ret != 0 {
                        cr_free(cr);
                        if ret == -(2 as i32) {
                            current_block = 15694547443304570712;
                        } else {
                            current_block = 13855183787650136026;
                        }
                    } else {
                        current_block = 9353995356876505083;
                    }
                } else {
                    current_block = 9353995356876505083;
                }
            } else {
                current_block = 15694547443304570712;
            }
            match current_block {
                15694547443304570712 => {}
                _ => {
                    match current_block {
                        11427802459928075752 => {
                            cr_init(
                                cr,
                                (*s).opaque,
                                Some(
                                    lre_realloc
                                        as unsafe fn(
                                            _: *mut std::ffi::c_void,
                                            _: *mut std::ffi::c_void,
                                            _: usize,
                                        )
                                            -> *mut std::ffi::c_void,
                                ),
                            );
                            ret = unicode_script(cr, value.as_mut_ptr(), script_ext);
                            if ret != 0 {
                                cr_free(cr);
                                if ret == -(2 as i32) {
                                    return re_parse_error(s, "unknown unicode script");
                                } else {
                                    current_block = 13855183787650136026;
                                }
                            } else {
                                current_block = 9353995356876505083;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        13855183787650136026 => return re_parse_out_of_memory(s),
                        _ => {
                            if is_inv != 0 {
                                if cr_invert(cr) != 0 {
                                    cr_free(cr);
                                    return -(1 as i32);
                                }
                            }
                            *pp = p;
                            return 0 as i32;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return re_parse_error(s, "unknown unicode property name");
}
/* CONFIG_ALL_UNICODE */
/* return -1 if error otherwise the character or a class range
(CLASS_RANGE_BASE). In case of class range, 'cr' is
initialized. Otherwise, it is ignored. */
unsafe fn get_class_atom(
    mut s: *mut REParseState,
    mut cr: *mut CharRange,
    mut pp: *mut *const u8,
    mut inclass: BOOL,
) -> i32 {
    let mut current_block: u64;
    let mut p: *const u8 = 0 as *const u8;
    let mut c: u32 = 0;
    let mut ret: i32 = 0;
    p = *pp;
    c = *p as u32;
    match c {
        92 => {
            p = p.offset(1);
            if p >= (*s).buf_end {
                current_block = 15043249707509330652;
            } else {
                let fresh10 = p;
                p = p.offset(1);
                c = *fresh10 as u32;
                match c {
                    100 => {
                        c = CHAR_RANGE_d as i32 as u32;
                        current_block = 6299577067972458399;
                    }
                    68 => {
                        c = CHAR_RANGE_D as i32 as u32;
                        current_block = 6299577067972458399;
                    }
                    115 => {
                        c = CHAR_RANGE_s as i32 as u32;
                        current_block = 6299577067972458399;
                    }
                    83 => {
                        c = CHAR_RANGE_S as i32 as u32;
                        current_block = 6299577067972458399;
                    }
                    119 => {
                        c = CHAR_RANGE_w as i32 as u32;
                        current_block = 6299577067972458399;
                    }
                    87 => {
                        c = CHAR_RANGE_W as i32 as u32;
                        current_block = 6299577067972458399;
                    }
                    99 => {
                        c = *p as u32;
                        if c >= 'a' as i32 as u32 && c <= 'z' as i32 as u32
                            || c >= 'A' as i32 as u32 && c <= 'Z' as i32 as u32
                            || (c >= '0' as i32 as u32 && c <= '9' as i32 as u32
                                || c == '_' as i32 as u32)
                                && inclass != 0
                                && (*s).is_utf16 == 0
                        {
                            /* Annex B.1.4 */
                            c &= 0x1f as i32 as u32;
                            p = p.offset(1);
                            current_block = 5159818223158340697;
                        } else if (*s).is_utf16 != 0 {
                            current_block = 14177022781132759768;
                        } else {
                            /* otherwise return '\' and 'c' */
                            p = p.offset(-1);
                            c = '\\' as i32 as u32;
                            current_block = 5159818223158340697;
                        }
                    }
                    112 | 80 => {
                        if (*s).is_utf16 != 0 {
                            if parse_unicode_property(
                                s,
                                cr,
                                &mut p,
                                (c == 'P' as i32 as u32) as i32,
                            ) != 0
                            {
                                return -(1 as i32);
                            }
                            c = 0x40000000 as i32 as u32;
                            current_block = 5159818223158340697;
                        } else {
                            current_block = 17562902476658757774;
                        }
                    }
                    _ => {
                        current_block = 17562902476658757774;
                    }
                }
                match current_block {
                    5159818223158340697 => {}
                    _ => {
                        match current_block {
                            17562902476658757774 =>
                            /* fall thru */
                            {
                                p = p.offset(-1);
                                ret = lre_parse_escape(&mut p, (*s).is_utf16 * 2 as i32);
                                if ret >= 0 as i32 {
                                    c = ret as u32;
                                    current_block = 5159818223158340697;
                                } else if ret == -(2 as i32)
                                    && *p as i32 != '\u{0}' as i32
                                    && !cstr_find_char(
                                        b"^$\\.*+?()[]{}|/\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        *p as i8,
                                    )
                                    .is_null()
                                {
                                    /* always valid to escape these characters */
                                    current_block = 16047511676058799569;
                                } else if (*s).is_utf16 != 0 {
                                    current_block = 14177022781132759768;
                                } else {
                                    /* just ignore the '\' */
                                    current_block = 16047511676058799569;
                                }
                            }
                            6299577067972458399 => {
                                if cr_init_char_range(s, cr, c) != 0 {
                                    return -(1 as i32);
                                }
                                c = 0x40000000 as i32 as u32;
                                current_block = 5159818223158340697;
                            }
                            _ => {}
                        }
                        match current_block {
                            16047511676058799569 => {}
                            5159818223158340697 => {}
                            _ => {
                                return re_parse_error(
                                    s,
                                    "invalid escape sequence in regular expression",
                                )
                            }
                        }
                    }
                }
            }
        }
        0 => {
            if p >= (*s).buf_end {
                current_block = 15043249707509330652;
            } else {
                current_block = 16047511676058799569;
            }
        }
        _ => {
            current_block = 16047511676058799569;
        }
    }
    match current_block {
        16047511676058799569 =>
        /* fall thru */
        /* normal char */
        {
            if c >= 128 as i32 as u32 {
                c = unicode_from_utf8(p, 6 as i32, &mut p) as u32;
                if c > 0xffff as i32 as u32 && (*s).is_utf16 == 0 {
                    /* XXX: should handle non BMP-1 code points */
                    return re_parse_error(s, "malformed unicode char");
                }
            } else {
                p = p.offset(1)
            }
        }
        15043249707509330652 => return re_parse_error(s, "unexpected end"),
        _ => {}
    }
    *pp = p;
    return c as i32;
}
unsafe fn re_emit_range(mut s: *mut REParseState, mut cr: *const CharRange) -> i32 {
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut high: u32 = 0;
    len = ((*cr).len as u32).wrapping_div(2 as i32 as u32) as i32;
    if len >= 65535 as i32 {
        return re_parse_error(s, "too many ranges");
    }
    if len == 0 as i32 {
        /* not sure it can really happen. Emit a match that is always
        false */
        re_emit_op_u32(s, REOP_char32 as i32, -(1 as i32) as u32);
    } else {
        high = *(*cr).points.offset(((*cr).len - 1 as i32) as isize);
        if high == 4294967295 as u32 {
            high = *(*cr).points.offset(((*cr).len - 2 as i32) as isize)
        }
        if high <= 0xffff as i32 as u32 {
            /* can use 16 bit ranges with the conversion that 0xffff =
            infinity */
            re_emit_op_u16(s, REOP_range as i32, len as u32); /* skip '[' */
            i = 0 as i32;
            while i < (*cr).len {
                dbuf_put_u16(&mut (*s).byte_code, *(*cr).points.offset(i as isize) as u16);
                high =
                    (*(*cr).points.offset((i + 1 as i32) as isize)).wrapping_sub(1 as i32 as u32);
                if high == (4294967295 as u32).wrapping_sub(1 as i32 as u32) {
                    high = 0xffff as i32 as u32
                }
                dbuf_put_u16(&mut (*s).byte_code, high as u16);
                i += 2 as i32
            }
        } else {
            re_emit_op_u16(s, REOP_range32 as i32, len as u32);
            i = 0 as i32;
            while i < (*cr).len {
                dbuf_put_u32(&mut (*s).byte_code, *(*cr).points.offset(i as isize));
                dbuf_put_u32(
                    &mut (*s).byte_code,
                    (*(*cr).points.offset((i + 1 as i32) as isize)).wrapping_sub(1 as i32 as u32),
                );
                i += 2 as i32
            }
        }
    }
    return 0 as i32;
}
unsafe fn re_parse_char_class(mut s: *mut REParseState, mut pp: *mut *const u8) -> i32 {
    let mut current_block: u64;
    let mut p: *const u8 = 0 as *const u8;
    let mut c1: u32 = 0;
    let mut c2: u32 = 0;
    let mut cr_s: CharRange = CharRange {
        len: 0,
        size: 0,
        points: 0 as *mut u32,
        mem_opaque: 0 as *mut std::ffi::c_void,
        realloc_func: None,
    };
    let mut cr: *mut CharRange = &mut cr_s;
    let mut cr1_s: CharRange = CharRange {
        len: 0,
        size: 0,
        points: 0 as *mut u32,
        mem_opaque: 0 as *mut std::ffi::c_void,
        realloc_func: None,
    };
    let mut cr1: *mut CharRange = &mut cr1_s;
    let mut invert: BOOL = 0;
    cr_init(
        cr,
        (*s).opaque,
        Some(
            lre_realloc
                as unsafe fn(
                    _: *mut std::ffi::c_void,
                    _: *mut std::ffi::c_void,
                    _: usize,
                ) -> *mut std::ffi::c_void,
        ),
    );
    p = *pp;
    p = p.offset(1);
    invert = FALSE as i32;
    if *p as i32 == '^' as i32 {
        p = p.offset(1);
        invert = TRUE as i32
    }
    loop {
        if *p as i32 == ']' as i32 {
            current_block = 572715077006366937;
            break;
        }
        c1 = get_class_atom(s, cr1, &mut p, TRUE as i32) as u32;
        if (c1 as i32) < 0 as i32 {
            current_block = 10339678743498588791;
            break;
        }
        if *p as i32 == '-' as i32 && *p.offset(1 as i32 as isize) as i32 != ']' as i32 {
            let mut p0: *const u8 = p.offset(1 as i32 as isize);
            if c1 >= 0x40000000 as i32 as u32 {
                if (*s).is_utf16 != 0 {
                    cr_free(cr1);
                    current_block = 2404584983316808095;
                } else {
                    /* Annex B: match '-' character */
                    current_block = 3183214240085336568;
                }
            } else {
                c2 = get_class_atom(s, cr1, &mut p0, TRUE as i32) as u32;
                if (c2 as i32) < 0 as i32 {
                    current_block = 10339678743498588791;
                    break;
                }
                if c2 >= 0x40000000 as i32 as u32 {
                    cr_free(cr1);
                    if (*s).is_utf16 != 0 {
                        current_block = 2404584983316808095;
                    } else {
                        current_block = 3183214240085336568;
                    }
                } else {
                    p = p0;
                    if c2 < c1 {
                        current_block = 2404584983316808095;
                    } else if cr_union_interval(cr, c1, c2) != 0 {
                        current_block = 69003184344742520;
                        break;
                    } else {
                        continue;
                    }
                }
            }
            match current_block {
                3183214240085336568 => {}
                _ => {
                    re_parse_error(s, "invalid class range");
                    current_block = 10339678743498588791;
                    break;
                }
            }
        }
        /* Annex B: match '-' character */
        if c1 >= 0x40000000 as i32 as u32 {
            let mut ret: i32 = 0; /* skip ']' */
            ret = cr_union1(cr, (*cr1).points, (*cr1).len);
            cr_free(cr1);
            if ret != 0 {
                current_block = 69003184344742520;
                break;
            }
        } else if cr_union_interval(cr, c1, c1) != 0 {
            current_block = 69003184344742520;
            break;
        }
    }
    match current_block {
        572715077006366937 => {
            if (*s).ignore_case != 0 {
                if cr_canonicalize(cr) != 0 {
                    current_block = 69003184344742520;
                } else {
                    current_block = 1847472278776910194;
                }
            } else {
                current_block = 1847472278776910194;
            }
            match current_block {
                69003184344742520 => {}
                _ => {
                    if invert != 0 {
                        if cr_invert(cr) != 0 {
                            current_block = 69003184344742520;
                        } else {
                            current_block = 3160140712158701372;
                        }
                    } else {
                        current_block = 3160140712158701372;
                    }
                    match current_block {
                        69003184344742520 => {}
                        _ => {
                            if re_emit_range(s, cr) != 0 {
                                current_block = 10339678743498588791;
                            } else {
                                cr_free(cr);
                                p = p.offset(1);
                                *pp = p;
                                return 0 as i32;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        69003184344742520 => {
            re_parse_out_of_memory(s);
        }
        _ => {}
    }
    cr_free(cr);
    return -(1 as i32);
}
/* Return:
   1 if the opcodes in bc_buf[] always advance the character pointer.
   0 if the character pointer may not be advanced.
   -1 if the code may depend on side effects of its previous execution (backreference)
*/
unsafe fn re_check_advance(mut bc_buf: *const u8, mut bc_buf_len: i32) -> i32 {
    let mut current_block: u64; /* not known yet */
    let mut pos: i32 = 0;
    let mut opcode: i32 = 0;
    let mut ret: i32 = 0;
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut val: u32 = 0;
    let mut last: u32 = 0;
    let mut has_back_reference: BOOL = 0;
    let mut capture_bitmap: [u8; 255] = [0; 255];
    ret = -(2 as i32);
    pos = 0 as i32;
    has_back_reference = FALSE as i32;
    (capture_bitmap.as_mut_ptr() as *mut u8).write_bytes(0, std::mem::size_of::<[u8; 255]>());
    while pos < bc_buf_len {
        opcode = *bc_buf.offset(pos as isize) as i32;
        len = reopcode_info[opcode as usize].size as i32;
        match opcode {
            21 => {
                val = get_u16(bc_buf.offset(pos as isize).offset(1 as i32 as isize));
                len = (len as u32).wrapping_add(val.wrapping_mul(4 as i32 as u32)) as i32 as i32;
                current_block = 3230862739852528027;
            }
            22 => {
                val = get_u16(bc_buf.offset(pos as isize).offset(1 as i32 as isize));
                len = (len as u32).wrapping_add(val.wrapping_mul(8 as i32 as u32)) as i32 as i32;
                current_block = 3230862739852528027;
            }
            1 | 2 | 3 | 4 => {
                current_block = 3230862739852528027;
            }
            5 | 6 | 15 | 25 | 16 | 17 | 18 | 27 => {
                current_block = 9520865839495247062;
            }
            11 | 12 => {
                val = *bc_buf.offset((pos + 1 as i32) as isize) as u32;
                capture_bitmap[val as usize] =
                    (capture_bitmap[val as usize] as i32 | 1 as i32) as u8;
                current_block = 9520865839495247062;
            }
            13 => {
                val = *bc_buf.offset((pos + 1 as i32) as isize) as u32;
                last = *bc_buf.offset((pos + 2 as i32) as isize) as u32;
                while val < last {
                    let fresh11 = val;
                    val = val.wrapping_add(1);
                    capture_bitmap[fresh11 as usize] =
                        (capture_bitmap[fresh11 as usize] as i32 | 1 as i32) as u8
                }
                current_block = 9520865839495247062;
            }
            19 | 20 => {
                val = *bc_buf.offset((pos + 1 as i32) as isize) as u32;
                capture_bitmap[val as usize] =
                    (capture_bitmap[val as usize] as i32 | 2 as i32) as u8;
                has_back_reference = TRUE as i32;
                current_block = 9520865839495247062;
            }
            _ => {
                /* safe behvior: we cannot predict the outcome */
                if ret == -(2 as i32) {
                    ret = 0 as i32
                }
                current_block = 9520865839495247062;
            }
        }
        match current_block {
            3230862739852528027 => {
                if ret == -(2 as i32) {
                    ret = 1 as i32
                }
            }
            _ => {}
        }
        /* no effect */
        pos += len
    }
    if has_back_reference != 0 {
        /* check if there is back reference which references a capture
        made in the some code */
        i = 0 as i32;
        while i < 255 as i32 {
            if capture_bitmap[i as usize] as i32 == 3 as i32 {
                return -(1 as i32);
            }
            i += 1
        }
    }
    if ret == -(2 as i32) {
        ret = 0 as i32
    }
    return ret;
}
/* return -1 if a simple quantifier cannot be used. Otherwise return
the number of characters in the atom. */
unsafe fn re_is_simple_quantifier(mut bc_buf: *const u8, mut bc_buf_len: i32) -> i32 {
    let mut current_block: u64;
    let mut pos: i32 = 0;
    let mut opcode: i32 = 0;
    let mut len: i32 = 0;
    let mut count: i32 = 0;
    let mut val: u32 = 0;
    count = 0 as i32;
    pos = 0 as i32;
    while pos < bc_buf_len {
        opcode = *bc_buf.offset(pos as isize) as i32;
        len = reopcode_info[opcode as usize].size as i32;
        match opcode {
            21 => {
                val = get_u16(bc_buf.offset(pos as isize).offset(1 as i32 as isize));
                len = (len as u32).wrapping_add(val.wrapping_mul(4 as i32 as u32)) as i32 as i32;
                current_block = 14441628745024150911;
            }
            22 => {
                val = get_u16(bc_buf.offset(pos as isize).offset(1 as i32 as isize));
                len = (len as u32).wrapping_add(val.wrapping_mul(8 as i32 as u32)) as i32 as i32;
                current_block = 14441628745024150911;
            }
            1 | 2 | 3 | 4 => {
                current_block = 14441628745024150911;
            }
            5 | 6 | 17 | 18 => {
                current_block = 7149356873433890176;
            }
            _ => return -(1 as i32),
        }
        match current_block {
            14441628745024150911 => count += 1,
            _ => {}
        }
        pos += len
    }
    return count;
}
/* '*pp' is the first char after '<' */
unsafe fn re_parse_group_name(
    mut buf: *mut std::os::raw::c_char,
    mut buf_size: i32,
    mut pp: *mut *const u8,
    mut is_utf16: BOOL,
) -> i32 {
    let mut p: *const u8 = 0 as *const u8;
    let mut c: u32 = 0;
    let mut q: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    p = *pp;
    q = buf;
    loop {
        c = *p as u32;
        if c == '\\' as i32 as u32 {
            p = p.offset(1);
            if *p as i32 != 'u' as i32 {
                return -(1 as i32);
            }
            c = lre_parse_escape(&mut p, is_utf16 * 2 as i32) as u32
        } else {
            if c == '>' as i32 as u32 {
                break;
            }
            if c >= 128 as i32 as u32 {
                c = unicode_from_utf8(p, 6 as i32, &mut p) as u32
            } else {
                p = p.offset(1)
            }
        }
        if c > 0x10ffff as i32 as u32 {
            return -(1 as i32);
        }
        if q == buf {
            if lre_js_is_ident_first(c as i32) == 0 {
                return -(1 as i32);
            }
        } else if lre_js_is_ident_next(c as i32) == 0 {
            return -(1 as i32);
        }
        if q.wrapping_offset_from(buf) as i64 + 6 as i32 as i64 + 1 as i32 as i64 > buf_size as i64
        {
            return -(1 as i32);
        }
        if c < 128 as i32 as u32 {
            let fresh12 = q;
            q = q.offset(1);
            *fresh12 = c as std::os::raw::c_char
        } else {
            q = q.offset(unicode_to_utf8(q as *mut u8, c) as isize)
        }
    }
    if q == buf {
        return -(1 as i32);
    }
    *q = '\u{0}' as i32 as std::os::raw::c_char;
    p = p.offset(1);
    *pp = p;
    return 0 as i32;
}
/* if capture_name = NULL: return the number of captures + 1.
Otherwise, return the capture index corresponding to capture_name
or -1 if none */
unsafe fn re_parse_captures(
    mut s: *mut REParseState,
    mut phas_named_captures: *mut i32,
    mut capture_name: *const std::os::raw::c_char,
) -> i32 {
    let mut p: *const u8 = 0 as *const u8;
    let mut capture_index: i32 = 0;
    let mut name: [std::os::raw::c_char; 128] = [0; 128];
    capture_index = 1 as i32;
    *phas_named_captures = 0 as i32;
    p = (*s).buf_start;
    while p < (*s).buf_end {
        match *p as i32 {
            40 => {
                if *p.offset(1 as i32 as isize) as i32 == '?' as i32 {
                    if *p.offset(2 as i32 as isize) as i32 == '<' as i32
                        && *p.offset(3 as i32 as isize) as i32 != '=' as i32
                        && *p.offset(3 as i32 as isize) as i32 != '!' as i32
                    {
                        *phas_named_captures = 1 as i32;
                        /* potential named capture */
                        if !capture_name.is_null() {
                            p = p.offset(3 as i32 as isize);
                            if re_parse_group_name(
                                name.as_mut_ptr(),
                                ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as u64 as i32,
                                &mut p,
                                (*s).is_utf16,
                            ) == 0 as i32
                            {
                                if cstr_compare(name.as_mut_ptr(), capture_name) == 0 {
                                    return capture_index;
                                }
                            }
                        }
                        capture_index += 1;
                        if capture_index >= 255 as i32 {
                            break;
                        }
                    }
                } else {
                    capture_index += 1;
                    if capture_index >= 255 as i32 {
                        break;
                    }
                }
            }
            92 => p = p.offset(1),
            91 => {
                p = p.offset((1 as i32 + (*p as i32 == ']' as i32) as i32) as isize);
                while p < (*s).buf_end && *p as i32 != ']' as i32 {
                    if *p as i32 == '\\' as i32 {
                        p = p.offset(1)
                    }
                    p = p.offset(1)
                }
            }
            _ => {}
        }
        p = p.offset(1)
    }
    if !capture_name.is_null() {
        return -(1 as i32);
    } else {
        return capture_index;
    };
}
unsafe fn re_count_captures(mut s: *mut REParseState) -> i32 {
    if (*s).total_capture_count < 0 as i32 {
        (*s).total_capture_count = re_parse_captures(
            s,
            &mut (*s).has_named_captures,
            0 as *const std::os::raw::c_char,
        )
    }
    return (*s).total_capture_count;
}
unsafe fn re_has_named_captures(mut s: *mut REParseState) -> BOOL {
    if (*s).has_named_captures < 0 as i32 {
        re_count_captures(s);
    }
    return (*s).has_named_captures;
}
unsafe fn find_group_name(mut s: *mut REParseState, mut name: *const std::os::raw::c_char) -> i32 {
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut buf_end: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut len: usize = 0;
    let mut name_len: usize = 0;
    let mut capture_index: i32 = 0;
    name_len = cstr_len(name) as usize;
    p = (*s).group_names.buf as *mut std::os::raw::c_char;
    buf_end =
        ((*s).group_names.buf as *mut std::os::raw::c_char).offset((*s).group_names.size as isize);
    capture_index = 1 as i32;
    while p < buf_end {
        len = cstr_len(p) as usize;
        if len == name_len
            && ptr_compare(name as *const u8, p as *const u8, name_len as usize) == 0 as i32
        {
            return capture_index;
        }
        p = p.offset(len.wrapping_add(1) as isize);
        capture_index += 1
    }
    return -(1 as i32);
}
unsafe fn re_parse_term(mut s: *mut REParseState, mut is_backward_dir: BOOL) -> i32 {
    let mut q: *const u8 = 0 as *const u8;
    let mut current_block: u64;
    let mut p: *const u8 = 0 as *const u8;
    let mut c: i32 = 0;
    let mut last_atom_start: i32 = 0;
    let mut quant_min: i32 = 0;
    let mut quant_max: i32 = 0;
    let mut last_capture_count: i32 = 0;
    let mut greedy: BOOL = 0;
    let mut add_zero_advance_check: BOOL = 0;
    let mut is_neg: BOOL = 0;
    let mut is_backward_lookahead: BOOL = 0;
    let mut cr_s: CharRange = CharRange {
        len: 0,
        size: 0,
        points: 0 as *mut u32,
        mem_opaque: 0 as *mut std::ffi::c_void,
        realloc_func: None,
    };
    let mut cr: *mut CharRange = &mut cr_s;
    last_atom_start = -(1 as i32);
    last_capture_count = 0 as i32;
    p = (*s).buf_ptr;
    c = *p as i32;
    match c {
        94 => {
            p = p.offset(1);
            re_emit_op(s, REOP_line_start as i32);
            current_block = 12151070351325546249;
        }
        36 => {
            p = p.offset(1);
            re_emit_op(s, REOP_line_end as i32);
            current_block = 12151070351325546249;
        }
        46 => {
            p = p.offset(1);
            last_atom_start = (*s).byte_code.size as i32;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as i32);
            }
            re_emit_op(
                s,
                if (*s).dotall != 0 {
                    REOP_any as i32
                } else {
                    REOP_dot as i32
                },
            );
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as i32);
            }
            current_block = 12151070351325546249;
        }
        123 => {
            if (*s).is_utf16 != 0 {
                return re_parse_error(s, "syntax error");
            } else if is_digit(*p.offset(1 as i32 as isize) as i32) == 0 {
                current_block = 14272147528220428300;
            } else {
                let mut p1: *const u8 = p.offset(1 as i32 as isize);
                /* Annex B: error if it is like a repetition count */
                parse_digits(&mut p1, TRUE as i32);
                if *p1 as i32 == ',' as i32 {
                    p1 = p1.offset(1);
                    if is_digit(*p1 as i32) != 0 {
                        parse_digits(&mut p1, TRUE as i32);
                    }
                }
                if *p1 as i32 != '}' as i32 {
                    current_block = 14272147528220428300;
                } else {
                    current_block = 13992302657118620366;
                }
            }
        }
        42 | 43 | 63 => {
            current_block = 13992302657118620366;
        }
        40 => {
            let mut pos: i32 = 0;
            let mut capture_index: i32 = 0;
            let mut current_block_82: u64;
            if *p.offset(1 as i32 as isize) as i32 == '?' as i32 {
                if *p.offset(2 as i32 as isize) as i32 == ':' as i32 {
                    p = p.offset(3 as i32 as isize);
                    last_atom_start = (*s).byte_code.size as i32;
                    last_capture_count = (*s).capture_count;
                    (*s).buf_ptr = p;
                    if re_parse_disjunction(s, is_backward_dir) != 0 {
                        return -(1 as i32);
                    }
                    p = (*s).buf_ptr;
                    if re_parse_expect(s, &mut p, ')' as i32) != 0 {
                        return -(1 as i32);
                    }
                    current_block_82 = 12070711452894729854;
                } else {
                    if *p.offset(2 as i32 as isize) as i32 == '=' as i32
                        || *p.offset(2 as i32 as isize) as i32 == '!' as i32
                    {
                        is_neg = (*p.offset(2 as i32 as isize) as i32 == '!' as i32) as i32;
                        is_backward_lookahead = FALSE as i32;
                        p = p.offset(3 as i32 as isize);
                        current_block_82 = 6896909654598042645;
                    } else if *p.offset(2 as i32 as isize) as i32 == '<' as i32
                        && (*p.offset(3 as i32 as isize) as i32 == '=' as i32
                            || *p.offset(3 as i32 as isize) as i32 == '!' as i32)
                    {
                        pos = 0;
                        is_neg = (*p.offset(3 as i32 as isize) as i32 == '!' as i32) as i32;
                        is_backward_lookahead = TRUE as i32;
                        p = p.offset(4 as i32 as isize);
                        current_block_82 = 6896909654598042645;
                    } else {
                        if *p.offset(2 as i32 as isize) as i32 == '<' as i32 {
                            p = p.offset(3 as i32 as isize);
                            if re_parse_group_name(
                                (*s).u.tmp_buf.as_mut_ptr(),
                                ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as u64 as i32,
                                &mut p,
                                (*s).is_utf16,
                            ) != 0
                            {
                                return re_parse_error(s, "invalid group name");
                            }
                            if find_group_name(s, (*s).u.tmp_buf.as_mut_ptr()) > 0 as i32 {
                                return re_parse_error(s, "duplicate group name");
                            }
                            /* group name with a trailing zero */
                            dbuf_put(
                                &mut (*s).group_names,
                                (*s).u.tmp_buf.as_mut_ptr() as *mut u8,
                                cstr_len((*s).u.tmp_buf.as_mut_ptr()).wrapping_add(1) as usize,
                            );
                            (*s).has_named_captures = 1 as i32
                        } else {
                            return re_parse_error(s, "invalid group");
                        }
                        current_block_82 = 2791873586345300331;
                    }
                    match current_block_82 {
                        2791873586345300331 => {}
                        _ =>
                        /* lookahead */
                             /* Annex B allows lookahead to be used as an atom for
                        the quantifiers */
                        {
                            if (*s).is_utf16 == 0 && is_backward_lookahead == 0 {
                                last_atom_start = (*s).byte_code.size as i32;
                                last_capture_count = (*s).capture_count
                            }
                            pos =
                                re_emit_op_u32(s, REOP_lookahead as i32 + is_neg, 0 as i32 as u32);
                            (*s).buf_ptr = p;
                            if re_parse_disjunction(s, is_backward_lookahead) != 0 {
                                return -(1 as i32);
                            }
                            p = (*s).buf_ptr;
                            if re_parse_expect(s, &mut p, ')' as i32) != 0 {
                                return -(1 as i32);
                            }
                            re_emit_op(s, REOP_match as i32);
                            /* jump after the 'match' after the lookahead is successful */
                            if dbuf_error(&mut (*s).byte_code) != 0 {
                                return -(1 as i32);
                            }
                            put_u32(
                                (*s).byte_code.buf.offset(pos as isize),
                                (*s).byte_code.size.wrapping_sub((pos + 4) as usize) as u32,
                            );
                            current_block_82 = 12070711452894729854;
                        }
                    }
                }
            } else {
                capture_index = 0;
                p = p.offset(1);
                /* capture without group name */
                dbuf_putc(&mut (*s).group_names, 0 as i32 as u8);
                current_block_82 = 2791873586345300331;
            }
            match current_block_82 {
                2791873586345300331 => {
                    if (*s).capture_count >= 255 as i32 {
                        return re_parse_error(s, "too many captures");
                    }
                    last_atom_start = (*s).byte_code.size as i32;
                    last_capture_count = (*s).capture_count;
                    let fresh13 = (*s).capture_count;
                    (*s).capture_count = (*s).capture_count + 1;
                    capture_index = fresh13;
                    re_emit_op_u8(
                        s,
                        REOP_save_start as i32 + is_backward_dir,
                        capture_index as u32,
                    );
                    (*s).buf_ptr = p;
                    if re_parse_disjunction(s, is_backward_dir) != 0 {
                        return -(1 as i32);
                    }
                    p = (*s).buf_ptr;
                    re_emit_op_u8(
                        s,
                        REOP_save_start as i32 + 1 as i32 - is_backward_dir,
                        capture_index as u32,
                    );
                    if re_parse_expect(s, &mut p, ')' as i32) != 0 {
                        return -(1 as i32);
                    }
                }
                _ => {}
            }
            current_block = 12151070351325546249;
        }
        92 => {
            match *p.offset(1 as i32 as isize) as i32 {
                98 | 66 => {
                    current_block = 8883256177700706243;
                    match current_block {
                        8883256177700706243 => {
                            re_emit_op(
                                s,
                                REOP_word_boundary as i32
                                    + (*p.offset(1 as i32 as isize) as i32 != 'b' as i32) as i32,
                            );
                            p = p.offset(2 as i32 as isize);
                            current_block = 12151070351325546249;
                        }
                        16372213405346143698 => {
                            p = p.offset(2 as i32 as isize);
                            c = 0 as i32;
                            if (*s).is_utf16 != 0 {
                                if is_digit(*p as i32) != 0 {
                                    return re_parse_error(
                                        s,
                                        "invalid decimal escape in regular expression",
                                    );
                                }
                            } else if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                let fresh14 = p;
                                p = p.offset(1);
                                c = *fresh14 as i32 - '0' as i32;
                                if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                    let fresh15 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as i32) + *fresh15 as i32 - '0' as i32
                                }
                            }
                            current_block = 6173299948494125894;
                        }
                        17395932908762866334 => {
                            let mut p1_0: *const u8 = 0 as *const u8;
                            let mut dummy_res: i32 = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as i32 as isize) as i32 != '<' as i32 {
                                /* Annex B.1.4: accept legacy octal */
                                /* annex B: we tolerate invalid group names in non
                                unicode mode if there is no named capture
                                definition */
                                if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(s, "expecting group name");
                                } else {
                                    current_block = 14272147528220428300;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as i32 as isize);
                                if re_parse_group_name(
                                    (*s).u.tmp_buf.as_mut_ptr(),
                                    ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as u64
                                        as i32,
                                    &mut p1_0,
                                    (*s).is_utf16,
                                ) != 0
                                {
                                    if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(s, "invalid group name");
                                    } else {
                                        current_block = 14272147528220428300;
                                    }
                                } else {
                                    c = find_group_name(s, (*s).u.tmp_buf.as_mut_ptr());
                                    if c < 0 as i32 {
                                        /* no capture name parsed before, try to look
                                        after (inefficient, but hopefully not common */
                                        c = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            (*s).u.tmp_buf.as_mut_ptr(),
                                        );
                                        if c < 0 as i32 {
                                            if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(s, "group name not defined");
                                            } else {
                                                current_block = 14272147528220428300;
                                            }
                                        } else {
                                            current_block = 6528931666172833996;
                                        }
                                    } else {
                                        current_block = 6528931666172833996;
                                    }
                                    match current_block {
                                        14272147528220428300 => {}
                                        _ => {
                                            p = p1_0;
                                            current_block = 8853100982098053779;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            q = p;
                            c = parse_digits(&mut p, FALSE as i32);
                            if c < 0 as i32 || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_utf16 == 0 {
                                    /* Annex B.1.4: accept legacy octal */
                                    p = q;
                                    if *p as i32 <= '7' as i32 {
                                        c = 0 as i32;
                                        if *p as i32 <= '3' as i32 {
                                            let fresh16 = p;
                                            p = p.offset(1);
                                            c = *fresh16 as i32 - '0' as i32
                                        }
                                        if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                            let fresh17 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as i32) + *fresh17 as i32 - '0' as i32;
                                            if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                                let fresh18 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as i32) + *fresh18 as i32 - '0' as i32
                                            }
                                        }
                                    } else {
                                        let fresh19 = p;
                                        p = p.offset(1);
                                        c = *fresh19 as i32
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        "back reference out of range in regular expression",
                                    );
                                }
                                current_block = 6173299948494125894;
                            } else {
                                current_block = 8853100982098053779;
                            }
                        }
                    }
                    match current_block {
                        6173299948494125894 => {}
                        14272147528220428300 => {}
                        12151070351325546249 => {}
                        _ => {
                            last_atom_start = (*s).byte_code.size as i32;
                            last_capture_count = (*s).capture_count;
                            re_emit_op_u8(
                                s,
                                REOP_back_reference as i32 + is_backward_dir,
                                c as u32,
                            );
                            current_block = 12151070351325546249;
                        }
                    }
                }
                107 => {
                    current_block = 17395932908762866334;
                    match current_block {
                        8883256177700706243 => {
                            re_emit_op(
                                s,
                                REOP_word_boundary as i32
                                    + (*p.offset(1 as i32 as isize) as i32 != 'b' as i32) as i32,
                            );
                            p = p.offset(2 as i32 as isize);
                            current_block = 12151070351325546249;
                        }
                        16372213405346143698 => {
                            p = p.offset(2 as i32 as isize);
                            c = 0 as i32;
                            if (*s).is_utf16 != 0 {
                                if is_digit(*p as i32) != 0 {
                                    return re_parse_error(
                                        s,
                                        "invalid decimal escape in regular expression",
                                    );
                                }
                            } else if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                let fresh14 = p;
                                p = p.offset(1);
                                c = *fresh14 as i32 - '0' as i32;
                                if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                    let fresh15 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as i32) + *fresh15 as i32 - '0' as i32
                                }
                            }
                            current_block = 6173299948494125894;
                        }
                        17395932908762866334 => {
                            let mut p1_0: *const u8 = 0 as *const u8;
                            let mut dummy_res: i32 = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as i32 as isize) as i32 != '<' as i32 {
                                if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(s, "expecting group name");
                                } else {
                                    current_block = 14272147528220428300;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as i32 as isize);
                                if re_parse_group_name(
                                    (*s).u.tmp_buf.as_mut_ptr(),
                                    ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as u64
                                        as i32,
                                    &mut p1_0,
                                    (*s).is_utf16,
                                ) != 0
                                {
                                    if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(s, "invalid group name");
                                    } else {
                                        current_block = 14272147528220428300;
                                    }
                                } else {
                                    c = find_group_name(s, (*s).u.tmp_buf.as_mut_ptr());
                                    if c < 0 as i32 {
                                        c = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            (*s).u.tmp_buf.as_mut_ptr(),
                                        );
                                        if c < 0 as i32 {
                                            if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(s, "group name not defined");
                                            } else {
                                                current_block = 14272147528220428300;
                                            }
                                        } else {
                                            current_block = 6528931666172833996;
                                        }
                                    } else {
                                        current_block = 6528931666172833996;
                                    }
                                    match current_block {
                                        14272147528220428300 => {}
                                        _ => {
                                            p = p1_0;
                                            current_block = 8853100982098053779;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            q = p;
                            c = parse_digits(&mut p, FALSE as i32);
                            if c < 0 as i32 || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_utf16 == 0 {
                                    p = q;
                                    if *p as i32 <= '7' as i32 {
                                        c = 0 as i32;
                                        if *p as i32 <= '3' as i32 {
                                            let fresh16 = p;
                                            p = p.offset(1);
                                            c = *fresh16 as i32 - '0' as i32
                                        }
                                        if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                            let fresh17 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as i32) + *fresh17 as i32 - '0' as i32;
                                            if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                                let fresh18 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as i32) + *fresh18 as i32 - '0' as i32
                                            }
                                        }
                                    } else {
                                        let fresh19 = p;
                                        p = p.offset(1);
                                        c = *fresh19 as i32
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        "back reference out of range in regular expression",
                                    );
                                }
                                current_block = 6173299948494125894;
                            } else {
                                current_block = 8853100982098053779;
                            }
                        }
                    }
                    match current_block {
                        6173299948494125894 => {}
                        14272147528220428300 => {}
                        12151070351325546249 => {}
                        _ => {
                            last_atom_start = (*s).byte_code.size as i32;
                            last_capture_count = (*s).capture_count;
                            re_emit_op_u8(
                                s,
                                REOP_back_reference as i32 + is_backward_dir,
                                c as u32,
                            );
                            current_block = 12151070351325546249;
                        }
                    }
                }
                48 => {
                    current_block = 16372213405346143698;
                    match current_block {
                        8883256177700706243 => {
                            re_emit_op(
                                s,
                                REOP_word_boundary as i32
                                    + (*p.offset(1 as i32 as isize) as i32 != 'b' as i32) as i32,
                            );
                            p = p.offset(2 as i32 as isize);
                            current_block = 12151070351325546249;
                        }
                        16372213405346143698 => {
                            p = p.offset(2 as i32 as isize);
                            c = 0 as i32;
                            if (*s).is_utf16 != 0 {
                                if is_digit(*p as i32) != 0 {
                                    return re_parse_error(
                                        s,
                                        "invalid decimal escape in regular expression",
                                    );
                                }
                            } else if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                let fresh14 = p;
                                p = p.offset(1);
                                c = *fresh14 as i32 - '0' as i32;
                                if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                    let fresh15 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as i32) + *fresh15 as i32 - '0' as i32
                                }
                            }
                            current_block = 6173299948494125894;
                        }
                        17395932908762866334 => {
                            let mut p1_0: *const u8 = 0 as *const u8;
                            let mut dummy_res: i32 = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as i32 as isize) as i32 != '<' as i32 {
                                if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(s, "expecting group name");
                                } else {
                                    current_block = 14272147528220428300;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as i32 as isize);
                                if re_parse_group_name(
                                    (*s).u.tmp_buf.as_mut_ptr(),
                                    ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as u64
                                        as i32,
                                    &mut p1_0,
                                    (*s).is_utf16,
                                ) != 0
                                {
                                    if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(s, "invalid group name");
                                    } else {
                                        current_block = 14272147528220428300;
                                    }
                                } else {
                                    c = find_group_name(s, (*s).u.tmp_buf.as_mut_ptr());
                                    if c < 0 as i32 {
                                        c = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            (*s).u.tmp_buf.as_mut_ptr(),
                                        );
                                        if c < 0 as i32 {
                                            if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(s, "group name not defined");
                                            } else {
                                                current_block = 14272147528220428300;
                                            }
                                        } else {
                                            current_block = 6528931666172833996;
                                        }
                                    } else {
                                        current_block = 6528931666172833996;
                                    }
                                    match current_block {
                                        14272147528220428300 => {}
                                        _ => {
                                            p = p1_0;
                                            current_block = 8853100982098053779;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            q = p;
                            c = parse_digits(&mut p, FALSE as i32);
                            if c < 0 as i32 || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_utf16 == 0 {
                                    p = q;
                                    if *p as i32 <= '7' as i32 {
                                        c = 0 as i32;
                                        if *p as i32 <= '3' as i32 {
                                            let fresh16 = p;
                                            p = p.offset(1);
                                            c = *fresh16 as i32 - '0' as i32
                                        }
                                        if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                            let fresh17 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as i32) + *fresh17 as i32 - '0' as i32;
                                            if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                                let fresh18 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as i32) + *fresh18 as i32 - '0' as i32
                                            }
                                        }
                                    } else {
                                        let fresh19 = p;
                                        p = p.offset(1);
                                        c = *fresh19 as i32
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        "back reference out of range in regular expression",
                                    );
                                }
                                current_block = 6173299948494125894;
                            } else {
                                current_block = 8853100982098053779;
                            }
                        }
                    }
                    match current_block {
                        6173299948494125894 => {}
                        14272147528220428300 => {}
                        12151070351325546249 => {}
                        _ => {
                            last_atom_start = (*s).byte_code.size as i32;
                            last_capture_count = (*s).capture_count;
                            re_emit_op_u8(
                                s,
                                REOP_back_reference as i32 + is_backward_dir,
                                c as u32,
                            );
                            current_block = 12151070351325546249;
                        }
                    }
                }
                49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    current_block = 7337917895049117968;
                    match current_block {
                        8883256177700706243 => {
                            re_emit_op(
                                s,
                                REOP_word_boundary as i32
                                    + (*p.offset(1 as i32 as isize) as i32 != 'b' as i32) as i32,
                            );
                            p = p.offset(2 as i32 as isize);
                            current_block = 12151070351325546249;
                        }
                        16372213405346143698 => {
                            p = p.offset(2 as i32 as isize);
                            c = 0 as i32;
                            if (*s).is_utf16 != 0 {
                                if is_digit(*p as i32) != 0 {
                                    return re_parse_error(
                                        s,
                                        "invalid decimal escape in regular expression",
                                    );
                                }
                            } else if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                let fresh14 = p;
                                p = p.offset(1);
                                c = *fresh14 as i32 - '0' as i32;
                                if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                    let fresh15 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as i32) + *fresh15 as i32 - '0' as i32
                                }
                            }
                            current_block = 6173299948494125894;
                        }
                        17395932908762866334 => {
                            let mut p1_0: *const u8 = 0 as *const u8;
                            let mut dummy_res: i32 = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as i32 as isize) as i32 != '<' as i32 {
                                if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(s, "expecting group name");
                                } else {
                                    current_block = 14272147528220428300;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as i32 as isize);
                                if re_parse_group_name(
                                    (*s).u.tmp_buf.as_mut_ptr(),
                                    ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as u64
                                        as i32,
                                    &mut p1_0,
                                    (*s).is_utf16,
                                ) != 0
                                {
                                    if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(s, "invalid group name");
                                    } else {
                                        current_block = 14272147528220428300;
                                    }
                                } else {
                                    c = find_group_name(s, (*s).u.tmp_buf.as_mut_ptr());
                                    if c < 0 as i32 {
                                        c = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            (*s).u.tmp_buf.as_mut_ptr(),
                                        );
                                        if c < 0 as i32 {
                                            if (*s).is_utf16 != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(s, "group name not defined");
                                            } else {
                                                current_block = 14272147528220428300;
                                            }
                                        } else {
                                            current_block = 6528931666172833996;
                                        }
                                    } else {
                                        current_block = 6528931666172833996;
                                    }
                                    match current_block {
                                        14272147528220428300 => {}
                                        _ => {
                                            p = p1_0;
                                            current_block = 8853100982098053779;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            q = p;
                            c = parse_digits(&mut p, FALSE as i32);
                            if c < 0 as i32 || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_utf16 == 0 {
                                    p = q;
                                    if *p as i32 <= '7' as i32 {
                                        c = 0 as i32;
                                        if *p as i32 <= '3' as i32 {
                                            let fresh16 = p;
                                            p = p.offset(1);
                                            c = *fresh16 as i32 - '0' as i32
                                        }
                                        if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                            let fresh17 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as i32) + *fresh17 as i32 - '0' as i32;
                                            if *p as i32 >= '0' as i32 && *p as i32 <= '7' as i32 {
                                                let fresh18 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as i32) + *fresh18 as i32 - '0' as i32
                                            }
                                        }
                                    } else {
                                        let fresh19 = p;
                                        p = p.offset(1);
                                        c = *fresh19 as i32
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        "back reference out of range in regular expression",
                                    );
                                }
                                current_block = 6173299948494125894;
                            } else {
                                current_block = 8853100982098053779;
                            }
                        }
                    }
                    match current_block {
                        6173299948494125894 => {}
                        14272147528220428300 => {}
                        12151070351325546249 => {}
                        _ => {
                            last_atom_start = (*s).byte_code.size as i32;
                            last_capture_count = (*s).capture_count;
                            re_emit_op_u8(
                                s,
                                REOP_back_reference as i32 + is_backward_dir,
                                c as u32,
                            );
                            current_block = 12151070351325546249;
                        }
                    }
                }
                _ => {
                    current_block = 14272147528220428300;
                }
            }
        }
        91 => {
            last_atom_start = (*s).byte_code.size as i32;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as i32);
            }
            if re_parse_char_class(s, &mut p) != 0 {
                return -(1 as i32);
            }
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as i32);
            }
            current_block = 12151070351325546249;
        }
        93 | 125 => {
            if (*s).is_utf16 != 0 {
                return re_parse_error(s, "syntax error");
            }
            current_block = 14272147528220428300;
        }
        _ => {
            current_block = 14272147528220428300;
        }
    }
    match current_block {
        14272147528220428300 =>
        /* Annex B: we accept '{' not followed by digits as a
        normal atom */
        {
            c = get_class_atom(s, cr, &mut p, FALSE as i32);
            if c < 0 as i32 {
                return -(1 as i32);
            }
            current_block = 6173299948494125894;
        }
        13992302657118620366 =>
        /* fall thru */
        {
            return re_parse_error(s, "nothing to repeat")
        }
        _ => {}
    }
    match current_block {
        6173299948494125894 => {
            last_atom_start = (*s).byte_code.size as i32;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as i32);
            }
            if c >= 0x40000000 as i32 {
                let mut ret: i32 = 0;
                /* Note: canonicalization is not needed */
                ret = re_emit_range(s, cr);
                cr_free(cr);
                if ret != 0 {
                    return -(1 as i32);
                }
            } else {
                if (*s).ignore_case != 0 {
                    c = lre_canonicalize(c as u32, (*s).is_utf16) as i32
                }
                if c <= 0xffff as i32 {
                    re_emit_op_u16(s, REOP_char as i32, c as u32);
                } else {
                    re_emit_op_u32(s, REOP_char32 as i32, c as u32);
                }
            }
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as i32);
            }
        }
        _ => {}
    }
    /* quantifier */
    if last_atom_start >= 0 as i32 {
        c = *p as i32;
        match c {
            42 => {
                current_block = 10839580692822357552;
                match current_block {
                    10839580692822357552 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 2147483647 as i32;
                        current_block = 16210164921736915844;
                    }
                    16854397559383395509 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 1 as i32;
                        current_block = 16210164921736915844;
                    }
                    4902147573222903068 => {
                        p = p.offset(1);
                        quant_min = 1 as i32;
                        quant_max = 2147483647 as i32;
                        current_block = 16210164921736915844;
                    }
                    _ => {
                        let mut p1_1: *const u8 = p;
                        /* As an extension (see ES6 annex B), we accept '{' not
                        followed by digits as a normal atom */
                        if is_digit(*p.offset(1 as i32 as isize) as i32) == 0 {
                            if (*s).is_utf16 != 0 {
                                current_block = 6640267502916221715;
                            } else {
                                current_block = 3543436503030046430;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(&mut p, TRUE as i32);
                            quant_max = quant_min;
                            if *p as i32 == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as i32) != 0 {
                                    quant_max = parse_digits(&mut p, TRUE as i32);
                                    if quant_max < quant_min {
                                        current_block = 6640267502916221715;
                                    } else {
                                        current_block = 6530401058219605690;
                                    }
                                } else {
                                    quant_max = 2147483647 as i32;
                                    current_block = 6530401058219605690;
                                    /* infinity */
                                }
                            } else {
                                current_block = 6530401058219605690;
                            }
                            match current_block {
                                6640267502916221715 => {}
                                _ => {
                                    if *p as i32 != '}' as i32 && (*s).is_utf16 == 0 {
                                        /* Annex B: normal atom if invalid '{' syntax */
                                        p = p1_1;
                                        current_block = 3543436503030046430;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as i32);
                                        }
                                        current_block = 16210164921736915844;
                                    }
                                }
                            }
                        }
                        match current_block {
                            16210164921736915844 => {}
                            3543436503030046430 => {}
                            _ => return re_parse_error(s, "invalid repetition count"),
                        }
                    }
                }
                match current_block {
                    3543436503030046430 => {}
                    _ => {
                        greedy = TRUE as i32;
                        if *p as i32 == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as i32
                        }
                        if last_atom_start < 0 as i32 {
                            return re_parse_error(s, "nothing to repeat");
                        }
                        if greedy != 0 {
                            let mut len: i32 = 0;
                            let mut pos_0: i32 = 0;
                            if quant_max > 0 as i32 {
                                /* specific optimization for simple quantifiers */
                                if dbuf_error(&mut (*s).byte_code) != 0 {
                                    current_block = 5210424319564767178;
                                } else {
                                    len = re_is_simple_quantifier(
                                        (*s).byte_code.buf.offset(last_atom_start as isize),
                                        (*s).byte_code.size.wrapping_sub(last_atom_start as usize)
                                            as i32,
                                    );
                                    if len > 0 as i32 {
                                        re_emit_op(s, REOP_match as i32);
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            17 as i32,
                                        ) != 0
                                        {
                                            current_block = 5210424319564767178;
                                        } else {
                                            pos_0 = last_atom_start;
                                            let fresh20 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *(*s).byte_code.buf.offset(fresh20 as isize) =
                                                REOP_simple_greedy_quant as i32 as u8;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                (*s).byte_code
                                                    .size
                                                    .wrapping_sub(last_atom_start as usize)
                                                    .wrapping_sub(17)
                                                    as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                quant_min as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                quant_max as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                len as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            current_block = 3543436503030046430;
                                        }
                                    } else {
                                        current_block = 1707335883933721018;
                                    }
                                }
                            } else {
                                current_block = 1707335883933721018;
                            }
                            match current_block {
                                5210424319564767178 => {}
                                3543436503030046430 => {}
                                _ => {
                                    if dbuf_error(&mut (*s).byte_code) != 0 {
                                        current_block = 5210424319564767178;
                                    } else {
                                        add_zero_advance_check = (re_check_advance(
                                            (*s).byte_code.buf.offset(last_atom_start as isize),
                                            (*s).byte_code
                                                .size
                                                .wrapping_sub(last_atom_start as usize)
                                                as i32,
                                        ) == 0 as i32)
                                            as i32;
                                        current_block = 6744494640291411773;
                                    }
                                }
                            }
                        } else {
                            add_zero_advance_check = FALSE as i32;
                            current_block = 6744494640291411773;
                        }
                        match current_block {
                            3543436503030046430 => {}
                            _ => {
                                match current_block {
                                    6744494640291411773 => {
                                        let mut len_0: i32 = 0;
                                        let mut pos_1: i32 = 0;
                                        len_0 = (*s)
                                            .byte_code
                                            .size
                                            .wrapping_sub(last_atom_start as usize)
                                            as i32;
                                        if quant_min == 0 as i32 {
                                            /* need to reset the capture in case the atom is
                                            not executed */
                                            if last_capture_count != (*s).capture_count {
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    3 as i32,
                                                ) != 0
                                                {
                                                    current_block = 5210424319564767178;
                                                } else {
                                                    let fresh21 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh21 as isize) =
                                                        REOP_save_reset as i32 as u8;
                                                    let fresh22 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh22 as isize) =
                                                        last_capture_count as u8;
                                                    let fresh23 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh23 as isize) =
                                                        ((*s).capture_count - 1 as i32) as u8;
                                                    current_block = 6936584767197543976;
                                                }
                                            } else {
                                                current_block = 6936584767197543976;
                                            }
                                            match current_block {
                                                5210424319564767178 => {}
                                                _ => {
                                                    if quant_max == 0 as i32 {
                                                        (*s).byte_code.size =
                                                            last_atom_start as usize;
                                                        current_block = 9856786070414082169;
                                                    } else if quant_max == 1 as i32 {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as i32,
                                                        ) != 0
                                                        {
                                                            current_block = 5210424319564767178;
                                                        } else {
                                                            *(*s)
                                                                .byte_code
                                                                .buf
                                                                .offset(last_atom_start as isize) =
                                                                (REOP_split_goto_first as i32
                                                                    + greedy)
                                                                    as u8;
                                                            put_u32(
                                                                (*s).byte_code
                                                                    .buf
                                                                    .offset(
                                                                        last_atom_start as isize,
                                                                    )
                                                                    .offset(1 as i32 as isize),
                                                                len_0 as u32,
                                                            );
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if quant_max == 2147483647 as i32 {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as i32 + add_zero_advance_check,
                                                        ) != 0
                                                        {
                                                            current_block = 5210424319564767178;
                                                        } else {
                                                            *(*s)
                                                                .byte_code
                                                                .buf
                                                                .offset(last_atom_start as isize) =
                                                                (REOP_split_goto_first as i32
                                                                    + greedy)
                                                                    as u8;
                                                            put_u32(
                                                                (*s).byte_code
                                                                    .buf
                                                                    .offset(
                                                                        last_atom_start as isize,
                                                                    )
                                                                    .offset(1 as i32 as isize),
                                                                (len_0
                                                                    + 5 as i32
                                                                    + add_zero_advance_check)
                                                                    as u32,
                                                            );
                                                            if add_zero_advance_check != 0 {
                                                                /* avoid infinite loop by stoping the
                                                                recursion if no advance was made in the
                                                                atom (only works if the atom has no
                                                                side effect) */
                                                                *(*s).byte_code.buf.offset(
                                                                    (last_atom_start
                                                                        + 1 as i32
                                                                        + 4 as i32)
                                                                        as isize,
                                                                ) = REOP_push_char_pos as i32 as u8;
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_bne_char_pos as i32,
                                                                    last_atom_start as u32,
                                                                );
                                                            } else {
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_goto as i32,
                                                                    last_atom_start as u32,
                                                                );
                                                            }
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if dbuf_insert(
                                                        &mut (*s).byte_code,
                                                        last_atom_start,
                                                        10 as i32,
                                                    ) != 0
                                                    {
                                                        current_block = 5210424319564767178;
                                                    } else {
                                                        pos_1 = last_atom_start;
                                                        let fresh24 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *(*s)
                                                            .byte_code
                                                            .buf
                                                            .offset(fresh24 as isize) =
                                                            REOP_push_i32 as i32 as u8;
                                                        put_u32(
                                                            (*s).byte_code
                                                                .buf
                                                                .offset(pos_1 as isize),
                                                            quant_max as u32,
                                                        );
                                                        pos_1 += 4 as i32;
                                                        let fresh25 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *(*s)
                                                            .byte_code
                                                            .buf
                                                            .offset(fresh25 as isize) =
                                                            (REOP_split_goto_first as i32 + greedy)
                                                                as u8;
                                                        put_u32(
                                                            (*s).byte_code
                                                                .buf
                                                                .offset(pos_1 as isize),
                                                            (len_0 + 5 as i32) as u32,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as i32,
                                                            (last_atom_start + 5 as i32) as u32,
                                                        );
                                                        re_emit_op(s, REOP_drop as i32);
                                                        current_block = 9856786070414082169;
                                                    }
                                                }
                                            }
                                        } else if quant_min == 1 as i32
                                            && quant_max == 2147483647 as i32
                                            && add_zero_advance_check == 0
                                        {
                                            re_emit_goto(
                                                s,
                                                REOP_split_next_first as i32 - greedy,
                                                last_atom_start as u32,
                                            );
                                            current_block = 9856786070414082169;
                                        } else {
                                            if quant_min == 1 as i32 {
                                                current_block = 5684771287319053842;
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                5 as i32,
                                            ) != 0
                                            {
                                                current_block = 5210424319564767178;
                                            } else {
                                                *(*s)
                                                    .byte_code
                                                    .buf
                                                    .offset(last_atom_start as isize) =
                                                    REOP_push_i32 as i32 as u8;
                                                put_u32(
                                                    (*s).byte_code
                                                        .buf
                                                        .offset(last_atom_start as isize)
                                                        .offset(1 as i32 as isize),
                                                    quant_min as u32,
                                                );
                                                last_atom_start += 5 as i32;
                                                re_emit_goto(
                                                    s,
                                                    REOP_loop as i32,
                                                    last_atom_start as u32,
                                                );
                                                re_emit_op(s, REOP_drop as i32);
                                                current_block = 5684771287319053842;
                                            }
                                            match current_block {
                                                5210424319564767178 => {}
                                                _ =>
                                                /* nothing to add */
                                                {
                                                    if quant_max == 2147483647 as i32 {
                                                        pos_1 = (*s).byte_code.size as i32;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as i32 + greedy,
                                                            (len_0
                                                                + 5 as i32
                                                                + add_zero_advance_check)
                                                                as u32,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_op(
                                                                s,
                                                                REOP_push_char_pos as i32,
                                                            );
                                                        }
                                                        /* copy the atom */
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as usize,
                                                            len_0 as usize,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_bne_char_pos as i32,
                                                                pos_1 as u32,
                                                            );
                                                        } else {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_goto as i32,
                                                                pos_1 as u32,
                                                            );
                                                        }
                                                    } else if quant_max > quant_min {
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_push_i32 as i32,
                                                            (quant_max - quant_min) as u32,
                                                        );
                                                        pos_1 = (*s).byte_code.size as i32;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as i32 + greedy,
                                                            (len_0 + 5 as i32) as u32,
                                                        );
                                                        /* copy the atom */
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as usize,
                                                            len_0 as usize,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as i32,
                                                            pos_1 as u32,
                                                        );
                                                        re_emit_op(s, REOP_drop as i32);
                                                    }
                                                    current_block = 9856786070414082169;
                                                }
                                            }
                                        }
                                        match current_block {
                                            5210424319564767178 => {}
                                            _ => {
                                                last_atom_start = -(1 as i32);
                                                current_block = 3543436503030046430;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    3543436503030046430 => {}
                                    _ => return re_parse_out_of_memory(s),
                                }
                            }
                        }
                    }
                }
            }
            43 => {
                current_block = 4902147573222903068;
                match current_block {
                    10839580692822357552 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 2147483647 as i32;
                        current_block = 16210164921736915844;
                    }
                    16854397559383395509 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 1 as i32;
                        current_block = 16210164921736915844;
                    }
                    4902147573222903068 => {
                        p = p.offset(1);
                        quant_min = 1 as i32;
                        quant_max = 2147483647 as i32;
                        current_block = 16210164921736915844;
                    }
                    _ => {
                        let mut p1_1: *const u8 = p;
                        if is_digit(*p.offset(1 as i32 as isize) as i32) == 0 {
                            if (*s).is_utf16 != 0 {
                                current_block = 6640267502916221715;
                            } else {
                                current_block = 3543436503030046430;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(&mut p, TRUE as i32);
                            quant_max = quant_min;
                            if *p as i32 == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as i32) != 0 {
                                    quant_max = parse_digits(&mut p, TRUE as i32);
                                    if quant_max < quant_min {
                                        current_block = 6640267502916221715;
                                    } else {
                                        current_block = 6530401058219605690;
                                    }
                                } else {
                                    quant_max = 2147483647 as i32;
                                    current_block = 6530401058219605690;
                                }
                            } else {
                                current_block = 6530401058219605690;
                            }
                            match current_block {
                                6640267502916221715 => {}
                                _ => {
                                    if *p as i32 != '}' as i32 && (*s).is_utf16 == 0 {
                                        p = p1_1;
                                        current_block = 3543436503030046430;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as i32);
                                        }
                                        current_block = 16210164921736915844;
                                    }
                                }
                            }
                        }
                        match current_block {
                            16210164921736915844 => {}
                            3543436503030046430 => {}
                            _ => return re_parse_error(s, "invalid repetition count"),
                        }
                    }
                }
                match current_block {
                    3543436503030046430 => {}
                    _ => {
                        greedy = TRUE as i32;
                        if *p as i32 == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as i32
                        }
                        if last_atom_start < 0 as i32 {
                            return re_parse_error(s, "nothing to repeat");
                        }
                        if greedy != 0 {
                            let mut len: i32 = 0;
                            let mut pos_0: i32 = 0;
                            if quant_max > 0 as i32 {
                                if dbuf_error(&mut (*s).byte_code) != 0 {
                                    current_block = 5210424319564767178;
                                } else {
                                    len = re_is_simple_quantifier(
                                        (*s).byte_code.buf.offset(last_atom_start as isize),
                                        (*s).byte_code.size.wrapping_sub(last_atom_start as usize)
                                            as i32,
                                    );
                                    if len > 0 as i32 {
                                        re_emit_op(s, REOP_match as i32);
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            17 as i32,
                                        ) != 0
                                        {
                                            current_block = 5210424319564767178;
                                        } else {
                                            pos_0 = last_atom_start;
                                            let fresh20 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *(*s).byte_code.buf.offset(fresh20 as isize) =
                                                REOP_simple_greedy_quant as i32 as u8;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                (*s).byte_code
                                                    .size
                                                    .wrapping_sub(last_atom_start as usize)
                                                    .wrapping_sub(17)
                                                    as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                quant_min as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                quant_max as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                len as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            current_block = 3543436503030046430;
                                        }
                                    } else {
                                        current_block = 1707335883933721018;
                                    }
                                }
                            } else {
                                current_block = 1707335883933721018;
                            }
                            match current_block {
                                5210424319564767178 => {}
                                3543436503030046430 => {}
                                _ => {
                                    if dbuf_error(&mut (*s).byte_code) != 0 {
                                        current_block = 5210424319564767178;
                                    } else {
                                        add_zero_advance_check = (re_check_advance(
                                            (*s).byte_code.buf.offset(last_atom_start as isize),
                                            (*s).byte_code
                                                .size
                                                .wrapping_sub(last_atom_start as usize)
                                                as i32,
                                        ) == 0 as i32)
                                            as i32;
                                        current_block = 6744494640291411773;
                                    }
                                }
                            }
                        } else {
                            add_zero_advance_check = FALSE as i32;
                            current_block = 6744494640291411773;
                        }
                        match current_block {
                            3543436503030046430 => {}
                            _ => {
                                match current_block {
                                    6744494640291411773 => {
                                        let mut len_0: i32 = 0;
                                        let mut pos_1: i32 = 0;
                                        len_0 = (*s)
                                            .byte_code
                                            .size
                                            .wrapping_sub(last_atom_start as usize)
                                            as i32;
                                        if quant_min == 0 as i32 {
                                            if last_capture_count != (*s).capture_count {
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    3 as i32,
                                                ) != 0
                                                {
                                                    current_block = 5210424319564767178;
                                                } else {
                                                    let fresh21 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh21 as isize) =
                                                        REOP_save_reset as i32 as u8;
                                                    let fresh22 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh22 as isize) =
                                                        last_capture_count as u8;
                                                    let fresh23 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh23 as isize) =
                                                        ((*s).capture_count - 1 as i32) as u8;
                                                    current_block = 6936584767197543976;
                                                }
                                            } else {
                                                current_block = 6936584767197543976;
                                            }
                                            match current_block {
                                                5210424319564767178 => {}
                                                _ => {
                                                    if quant_max == 0 as i32 {
                                                        (*s).byte_code.size =
                                                            last_atom_start as usize;
                                                        current_block = 9856786070414082169;
                                                    } else if quant_max == 1 as i32 {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as i32,
                                                        ) != 0
                                                        {
                                                            current_block = 5210424319564767178;
                                                        } else {
                                                            *(*s)
                                                                .byte_code
                                                                .buf
                                                                .offset(last_atom_start as isize) =
                                                                (REOP_split_goto_first as i32
                                                                    + greedy)
                                                                    as u8;
                                                            put_u32(
                                                                (*s).byte_code
                                                                    .buf
                                                                    .offset(
                                                                        last_atom_start as isize,
                                                                    )
                                                                    .offset(1 as i32 as isize),
                                                                len_0 as u32,
                                                            );
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if quant_max == 2147483647 as i32 {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as i32 + add_zero_advance_check,
                                                        ) != 0
                                                        {
                                                            current_block = 5210424319564767178;
                                                        } else {
                                                            *(*s)
                                                                .byte_code
                                                                .buf
                                                                .offset(last_atom_start as isize) =
                                                                (REOP_split_goto_first as i32
                                                                    + greedy)
                                                                    as u8;
                                                            put_u32(
                                                                (*s).byte_code
                                                                    .buf
                                                                    .offset(
                                                                        last_atom_start as isize,
                                                                    )
                                                                    .offset(1 as i32 as isize),
                                                                (len_0
                                                                    + 5 as i32
                                                                    + add_zero_advance_check)
                                                                    as u32,
                                                            );
                                                            if add_zero_advance_check != 0 {
                                                                *(*s).byte_code.buf.offset(
                                                                    (last_atom_start
                                                                        + 1 as i32
                                                                        + 4 as i32)
                                                                        as isize,
                                                                ) = REOP_push_char_pos as i32 as u8;
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_bne_char_pos as i32,
                                                                    last_atom_start as u32,
                                                                );
                                                            } else {
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_goto as i32,
                                                                    last_atom_start as u32,
                                                                );
                                                            }
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if dbuf_insert(
                                                        &mut (*s).byte_code,
                                                        last_atom_start,
                                                        10 as i32,
                                                    ) != 0
                                                    {
                                                        current_block = 5210424319564767178;
                                                    } else {
                                                        pos_1 = last_atom_start;
                                                        let fresh24 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *(*s)
                                                            .byte_code
                                                            .buf
                                                            .offset(fresh24 as isize) =
                                                            REOP_push_i32 as i32 as u8;
                                                        put_u32(
                                                            (*s).byte_code
                                                                .buf
                                                                .offset(pos_1 as isize),
                                                            quant_max as u32,
                                                        );
                                                        pos_1 += 4 as i32;
                                                        let fresh25 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *(*s)
                                                            .byte_code
                                                            .buf
                                                            .offset(fresh25 as isize) =
                                                            (REOP_split_goto_first as i32 + greedy)
                                                                as u8;
                                                        put_u32(
                                                            (*s).byte_code
                                                                .buf
                                                                .offset(pos_1 as isize),
                                                            (len_0 + 5 as i32) as u32,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as i32,
                                                            (last_atom_start + 5 as i32) as u32,
                                                        );
                                                        re_emit_op(s, REOP_drop as i32);
                                                        current_block = 9856786070414082169;
                                                    }
                                                }
                                            }
                                        } else if quant_min == 1 as i32
                                            && quant_max == 2147483647 as i32
                                            && add_zero_advance_check == 0
                                        {
                                            re_emit_goto(
                                                s,
                                                REOP_split_next_first as i32 - greedy,
                                                last_atom_start as u32,
                                            );
                                            current_block = 9856786070414082169;
                                        } else {
                                            if quant_min == 1 as i32 {
                                                current_block = 5684771287319053842;
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                5 as i32,
                                            ) != 0
                                            {
                                                current_block = 5210424319564767178;
                                            } else {
                                                *(*s)
                                                    .byte_code
                                                    .buf
                                                    .offset(last_atom_start as isize) =
                                                    REOP_push_i32 as i32 as u8;
                                                put_u32(
                                                    (*s).byte_code
                                                        .buf
                                                        .offset(last_atom_start as isize)
                                                        .offset(1 as i32 as isize),
                                                    quant_min as u32,
                                                );
                                                last_atom_start += 5 as i32;
                                                re_emit_goto(
                                                    s,
                                                    REOP_loop as i32,
                                                    last_atom_start as u32,
                                                );
                                                re_emit_op(s, REOP_drop as i32);
                                                current_block = 5684771287319053842;
                                            }
                                            match current_block {
                                                5210424319564767178 => {}
                                                _ => {
                                                    if quant_max == 2147483647 as i32 {
                                                        pos_1 = (*s).byte_code.size as i32;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as i32 + greedy,
                                                            (len_0
                                                                + 5 as i32
                                                                + add_zero_advance_check)
                                                                as u32,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_op(
                                                                s,
                                                                REOP_push_char_pos as i32,
                                                            );
                                                        }
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as usize,
                                                            len_0 as usize,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_bne_char_pos as i32,
                                                                pos_1 as u32,
                                                            );
                                                        } else {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_goto as i32,
                                                                pos_1 as u32,
                                                            );
                                                        }
                                                    } else if quant_max > quant_min {
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_push_i32 as i32,
                                                            (quant_max - quant_min) as u32,
                                                        );
                                                        pos_1 = (*s).byte_code.size as i32;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as i32 + greedy,
                                                            (len_0 + 5 as i32) as u32,
                                                        );
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as usize,
                                                            len_0 as usize,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as i32,
                                                            pos_1 as u32,
                                                        );
                                                        re_emit_op(s, REOP_drop as i32);
                                                    }
                                                    current_block = 9856786070414082169;
                                                }
                                            }
                                        }
                                        match current_block {
                                            5210424319564767178 => {}
                                            _ => {
                                                last_atom_start = -(1 as i32);
                                                current_block = 3543436503030046430;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    3543436503030046430 => {}
                                    _ => return re_parse_out_of_memory(s),
                                }
                            }
                        }
                    }
                }
            }
            63 => {
                current_block = 16854397559383395509;
                match current_block {
                    10839580692822357552 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 2147483647 as i32;
                        current_block = 16210164921736915844;
                    }
                    16854397559383395509 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 1 as i32;
                        current_block = 16210164921736915844;
                    }
                    4902147573222903068 => {
                        p = p.offset(1);
                        quant_min = 1 as i32;
                        quant_max = 2147483647 as i32;
                        current_block = 16210164921736915844;
                    }
                    _ => {
                        let mut p1_1: *const u8 = p;
                        if is_digit(*p.offset(1 as i32 as isize) as i32) == 0 {
                            if (*s).is_utf16 != 0 {
                                current_block = 6640267502916221715;
                            } else {
                                current_block = 3543436503030046430;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(&mut p, TRUE as i32);
                            quant_max = quant_min;
                            if *p as i32 == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as i32) != 0 {
                                    quant_max = parse_digits(&mut p, TRUE as i32);
                                    if quant_max < quant_min {
                                        current_block = 6640267502916221715;
                                    } else {
                                        current_block = 6530401058219605690;
                                    }
                                } else {
                                    quant_max = 2147483647 as i32;
                                    current_block = 6530401058219605690;
                                }
                            } else {
                                current_block = 6530401058219605690;
                            }
                            match current_block {
                                6640267502916221715 => {}
                                _ => {
                                    if *p as i32 != '}' as i32 && (*s).is_utf16 == 0 {
                                        p = p1_1;
                                        current_block = 3543436503030046430;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as i32);
                                        }
                                        current_block = 16210164921736915844;
                                    }
                                }
                            }
                        }
                        match current_block {
                            16210164921736915844 => {}
                            3543436503030046430 => {}
                            _ => return re_parse_error(s, "invalid repetition count"),
                        }
                    }
                }
                match current_block {
                    3543436503030046430 => {}
                    _ => {
                        greedy = TRUE as i32;
                        if *p as i32 == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as i32
                        }
                        if last_atom_start < 0 as i32 {
                            return re_parse_error(s, "nothing to repeat");
                        }
                        if greedy != 0 {
                            let mut len: i32 = 0;
                            let mut pos_0: i32 = 0;
                            if quant_max > 0 as i32 {
                                if dbuf_error(&mut (*s).byte_code) != 0 {
                                    current_block = 5210424319564767178;
                                } else {
                                    len = re_is_simple_quantifier(
                                        (*s).byte_code.buf.offset(last_atom_start as isize),
                                        (*s).byte_code.size.wrapping_sub(last_atom_start as usize)
                                            as i32,
                                    );
                                    if len > 0 as i32 {
                                        re_emit_op(s, REOP_match as i32);
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            17 as i32,
                                        ) != 0
                                        {
                                            current_block = 5210424319564767178;
                                        } else {
                                            pos_0 = last_atom_start;
                                            let fresh20 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *(*s).byte_code.buf.offset(fresh20 as isize) =
                                                REOP_simple_greedy_quant as i32 as u8;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                (*s).byte_code
                                                    .size
                                                    .wrapping_sub(last_atom_start as usize)
                                                    .wrapping_sub(17)
                                                    as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                quant_min as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                quant_max as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                len as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            current_block = 3543436503030046430;
                                        }
                                    } else {
                                        current_block = 1707335883933721018;
                                    }
                                }
                            } else {
                                current_block = 1707335883933721018;
                            }
                            match current_block {
                                5210424319564767178 => {}
                                3543436503030046430 => {}
                                _ => {
                                    if dbuf_error(&mut (*s).byte_code) != 0 {
                                        current_block = 5210424319564767178;
                                    } else {
                                        add_zero_advance_check = (re_check_advance(
                                            (*s).byte_code.buf.offset(last_atom_start as isize),
                                            (*s).byte_code
                                                .size
                                                .wrapping_sub(last_atom_start as usize)
                                                as i32,
                                        ) == 0 as i32)
                                            as i32;
                                        current_block = 6744494640291411773;
                                    }
                                }
                            }
                        } else {
                            add_zero_advance_check = FALSE as i32;
                            current_block = 6744494640291411773;
                        }
                        match current_block {
                            3543436503030046430 => {}
                            _ => {
                                match current_block {
                                    6744494640291411773 => {
                                        let mut len_0: i32 = 0;
                                        let mut pos_1: i32 = 0;
                                        len_0 = (*s)
                                            .byte_code
                                            .size
                                            .wrapping_sub(last_atom_start as usize)
                                            as i32;
                                        if quant_min == 0 as i32 {
                                            if last_capture_count != (*s).capture_count {
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    3 as i32,
                                                ) != 0
                                                {
                                                    current_block = 5210424319564767178;
                                                } else {
                                                    let fresh21 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh21 as isize) =
                                                        REOP_save_reset as i32 as u8;
                                                    let fresh22 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh22 as isize) =
                                                        last_capture_count as u8;
                                                    let fresh23 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh23 as isize) =
                                                        ((*s).capture_count - 1 as i32) as u8;
                                                    current_block = 6936584767197543976;
                                                }
                                            } else {
                                                current_block = 6936584767197543976;
                                            }
                                            match current_block {
                                                5210424319564767178 => {}
                                                _ => {
                                                    if quant_max == 0 as i32 {
                                                        (*s).byte_code.size =
                                                            last_atom_start as usize;
                                                        current_block = 9856786070414082169;
                                                    } else if quant_max == 1 as i32 {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as i32,
                                                        ) != 0
                                                        {
                                                            current_block = 5210424319564767178;
                                                        } else {
                                                            *(*s)
                                                                .byte_code
                                                                .buf
                                                                .offset(last_atom_start as isize) =
                                                                (REOP_split_goto_first as i32
                                                                    + greedy)
                                                                    as u8;
                                                            put_u32(
                                                                (*s).byte_code
                                                                    .buf
                                                                    .offset(
                                                                        last_atom_start as isize,
                                                                    )
                                                                    .offset(1 as i32 as isize),
                                                                len_0 as u32,
                                                            );
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if quant_max == 2147483647 as i32 {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as i32 + add_zero_advance_check,
                                                        ) != 0
                                                        {
                                                            current_block = 5210424319564767178;
                                                        } else {
                                                            *(*s)
                                                                .byte_code
                                                                .buf
                                                                .offset(last_atom_start as isize) =
                                                                (REOP_split_goto_first as i32
                                                                    + greedy)
                                                                    as u8;
                                                            put_u32(
                                                                (*s).byte_code
                                                                    .buf
                                                                    .offset(
                                                                        last_atom_start as isize,
                                                                    )
                                                                    .offset(1 as i32 as isize),
                                                                (len_0
                                                                    + 5 as i32
                                                                    + add_zero_advance_check)
                                                                    as u32,
                                                            );
                                                            if add_zero_advance_check != 0 {
                                                                *(*s).byte_code.buf.offset(
                                                                    (last_atom_start
                                                                        + 1 as i32
                                                                        + 4 as i32)
                                                                        as isize,
                                                                ) = REOP_push_char_pos as i32 as u8;
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_bne_char_pos as i32,
                                                                    last_atom_start as u32,
                                                                );
                                                            } else {
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_goto as i32,
                                                                    last_atom_start as u32,
                                                                );
                                                            }
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if dbuf_insert(
                                                        &mut (*s).byte_code,
                                                        last_atom_start,
                                                        10 as i32,
                                                    ) != 0
                                                    {
                                                        current_block = 5210424319564767178;
                                                    } else {
                                                        pos_1 = last_atom_start;
                                                        let fresh24 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *(*s)
                                                            .byte_code
                                                            .buf
                                                            .offset(fresh24 as isize) =
                                                            REOP_push_i32 as i32 as u8;
                                                        put_u32(
                                                            (*s).byte_code
                                                                .buf
                                                                .offset(pos_1 as isize),
                                                            quant_max as u32,
                                                        );
                                                        pos_1 += 4 as i32;
                                                        let fresh25 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *(*s)
                                                            .byte_code
                                                            .buf
                                                            .offset(fresh25 as isize) =
                                                            (REOP_split_goto_first as i32 + greedy)
                                                                as u8;
                                                        put_u32(
                                                            (*s).byte_code
                                                                .buf
                                                                .offset(pos_1 as isize),
                                                            (len_0 + 5 as i32) as u32,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as i32,
                                                            (last_atom_start + 5 as i32) as u32,
                                                        );
                                                        re_emit_op(s, REOP_drop as i32);
                                                        current_block = 9856786070414082169;
                                                    }
                                                }
                                            }
                                        } else if quant_min == 1 as i32
                                            && quant_max == 2147483647 as i32
                                            && add_zero_advance_check == 0
                                        {
                                            re_emit_goto(
                                                s,
                                                REOP_split_next_first as i32 - greedy,
                                                last_atom_start as u32,
                                            );
                                            current_block = 9856786070414082169;
                                        } else {
                                            if quant_min == 1 as i32 {
                                                current_block = 5684771287319053842;
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                5 as i32,
                                            ) != 0
                                            {
                                                current_block = 5210424319564767178;
                                            } else {
                                                *(*s)
                                                    .byte_code
                                                    .buf
                                                    .offset(last_atom_start as isize) =
                                                    REOP_push_i32 as i32 as u8;
                                                put_u32(
                                                    (*s).byte_code
                                                        .buf
                                                        .offset(last_atom_start as isize)
                                                        .offset(1 as i32 as isize),
                                                    quant_min as u32,
                                                );
                                                last_atom_start += 5 as i32;
                                                re_emit_goto(
                                                    s,
                                                    REOP_loop as i32,
                                                    last_atom_start as u32,
                                                );
                                                re_emit_op(s, REOP_drop as i32);
                                                current_block = 5684771287319053842;
                                            }
                                            match current_block {
                                                5210424319564767178 => {}
                                                _ => {
                                                    if quant_max == 2147483647 as i32 {
                                                        pos_1 = (*s).byte_code.size as i32;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as i32 + greedy,
                                                            (len_0
                                                                + 5 as i32
                                                                + add_zero_advance_check)
                                                                as u32,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_op(
                                                                s,
                                                                REOP_push_char_pos as i32,
                                                            );
                                                        }
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as usize,
                                                            len_0 as usize,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_bne_char_pos as i32,
                                                                pos_1 as u32,
                                                            );
                                                        } else {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_goto as i32,
                                                                pos_1 as u32,
                                                            );
                                                        }
                                                    } else if quant_max > quant_min {
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_push_i32 as i32,
                                                            (quant_max - quant_min) as u32,
                                                        );
                                                        pos_1 = (*s).byte_code.size as i32;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as i32 + greedy,
                                                            (len_0 + 5 as i32) as u32,
                                                        );
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as usize,
                                                            len_0 as usize,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as i32,
                                                            pos_1 as u32,
                                                        );
                                                        re_emit_op(s, REOP_drop as i32);
                                                    }
                                                    current_block = 9856786070414082169;
                                                }
                                            }
                                        }
                                        match current_block {
                                            5210424319564767178 => {}
                                            _ => {
                                                last_atom_start = -(1 as i32);
                                                current_block = 3543436503030046430;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    3543436503030046430 => {}
                                    _ => return re_parse_out_of_memory(s),
                                }
                            }
                        }
                    }
                }
            }
            123 => {
                current_block = 12227374774078719326;
                match current_block {
                    10839580692822357552 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 2147483647 as i32;
                        current_block = 16210164921736915844;
                    }
                    16854397559383395509 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 1 as i32;
                        current_block = 16210164921736915844;
                    }
                    4902147573222903068 => {
                        p = p.offset(1);
                        quant_min = 1 as i32;
                        quant_max = 2147483647 as i32;
                        current_block = 16210164921736915844;
                    }
                    _ => {
                        let mut p1_1: *const u8 = p;
                        if is_digit(*p.offset(1 as i32 as isize) as i32) == 0 {
                            if (*s).is_utf16 != 0 {
                                current_block = 6640267502916221715;
                            } else {
                                current_block = 3543436503030046430;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(&mut p, TRUE as i32);
                            quant_max = quant_min;
                            if *p as i32 == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as i32) != 0 {
                                    quant_max = parse_digits(&mut p, TRUE as i32);
                                    if quant_max < quant_min {
                                        current_block = 6640267502916221715;
                                    } else {
                                        current_block = 6530401058219605690;
                                    }
                                } else {
                                    quant_max = 2147483647 as i32;
                                    current_block = 6530401058219605690;
                                }
                            } else {
                                current_block = 6530401058219605690;
                            }
                            match current_block {
                                6640267502916221715 => {}
                                _ => {
                                    if *p as i32 != '}' as i32 && (*s).is_utf16 == 0 {
                                        p = p1_1;
                                        current_block = 3543436503030046430;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as i32);
                                        }
                                        current_block = 16210164921736915844;
                                    }
                                }
                            }
                        }
                        match current_block {
                            16210164921736915844 => {}
                            3543436503030046430 => {}
                            _ => return re_parse_error(s, "invalid repetition count"),
                        }
                    }
                }
                match current_block {
                    3543436503030046430 => {}
                    _ => {
                        greedy = TRUE as i32;
                        if *p as i32 == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as i32
                        }
                        if last_atom_start < 0 as i32 {
                            return re_parse_error(s, "nothing to repeat");
                        }
                        if greedy != 0 {
                            let mut len: i32 = 0;
                            let mut pos_0: i32 = 0;
                            if quant_max > 0 as i32 {
                                if dbuf_error(&mut (*s).byte_code) != 0 {
                                    current_block = 5210424319564767178;
                                } else {
                                    len = re_is_simple_quantifier(
                                        (*s).byte_code.buf.offset(last_atom_start as isize),
                                        (*s).byte_code.size.wrapping_sub(last_atom_start as usize)
                                            as i32,
                                    );
                                    if len > 0 as i32 {
                                        re_emit_op(s, REOP_match as i32);
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            17 as i32,
                                        ) != 0
                                        {
                                            current_block = 5210424319564767178;
                                        } else {
                                            pos_0 = last_atom_start;
                                            let fresh20 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *(*s).byte_code.buf.offset(fresh20 as isize) =
                                                REOP_simple_greedy_quant as i32 as u8;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                (*s).byte_code
                                                    .size
                                                    .wrapping_sub(last_atom_start as usize)
                                                    .wrapping_sub(17)
                                                    as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                quant_min as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                quant_max as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            put_u32(
                                                &mut *(*s).byte_code.buf.offset(pos_0 as isize),
                                                len as u32,
                                            );
                                            pos_0 += 4 as i32;
                                            current_block = 3543436503030046430;
                                        }
                                    } else {
                                        current_block = 1707335883933721018;
                                    }
                                }
                            } else {
                                current_block = 1707335883933721018;
                            }
                            match current_block {
                                5210424319564767178 => {}
                                3543436503030046430 => {}
                                _ => {
                                    if dbuf_error(&mut (*s).byte_code) != 0 {
                                        current_block = 5210424319564767178;
                                    } else {
                                        add_zero_advance_check = (re_check_advance(
                                            (*s).byte_code.buf.offset(last_atom_start as isize),
                                            (*s).byte_code
                                                .size
                                                .wrapping_sub(last_atom_start as usize)
                                                as i32,
                                        ) == 0 as i32)
                                            as i32;
                                        current_block = 6744494640291411773;
                                    }
                                }
                            }
                        } else {
                            add_zero_advance_check = FALSE as i32;
                            current_block = 6744494640291411773;
                        }
                        match current_block {
                            3543436503030046430 => {}
                            _ => {
                                match current_block {
                                    6744494640291411773 => {
                                        let mut len_0: i32 = 0;
                                        let mut pos_1: i32 = 0;
                                        len_0 = (*s)
                                            .byte_code
                                            .size
                                            .wrapping_sub(last_atom_start as usize)
                                            as i32;
                                        if quant_min == 0 as i32 {
                                            if last_capture_count != (*s).capture_count {
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    3 as i32,
                                                ) != 0
                                                {
                                                    current_block = 5210424319564767178;
                                                } else {
                                                    let fresh21 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh21 as isize) =
                                                        REOP_save_reset as i32 as u8;
                                                    let fresh22 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh22 as isize) =
                                                        last_capture_count as u8;
                                                    let fresh23 = last_atom_start;
                                                    last_atom_start = last_atom_start + 1;
                                                    *(*s).byte_code.buf.offset(fresh23 as isize) =
                                                        ((*s).capture_count - 1 as i32) as u8;
                                                    current_block = 6936584767197543976;
                                                }
                                            } else {
                                                current_block = 6936584767197543976;
                                            }
                                            match current_block {
                                                5210424319564767178 => {}
                                                _ => {
                                                    if quant_max == 0 as i32 {
                                                        (*s).byte_code.size =
                                                            last_atom_start as usize;
                                                        current_block = 9856786070414082169;
                                                    } else if quant_max == 1 as i32 {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as i32,
                                                        ) != 0
                                                        {
                                                            current_block = 5210424319564767178;
                                                        } else {
                                                            *(*s)
                                                                .byte_code
                                                                .buf
                                                                .offset(last_atom_start as isize) =
                                                                (REOP_split_goto_first as i32
                                                                    + greedy)
                                                                    as u8;
                                                            put_u32(
                                                                (*s).byte_code
                                                                    .buf
                                                                    .offset(
                                                                        last_atom_start as isize,
                                                                    )
                                                                    .offset(1 as i32 as isize),
                                                                len_0 as u32,
                                                            );
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if quant_max == 2147483647 as i32 {
                                                        if dbuf_insert(
                                                            &mut (*s).byte_code,
                                                            last_atom_start,
                                                            5 as i32 + add_zero_advance_check,
                                                        ) != 0
                                                        {
                                                            current_block = 5210424319564767178;
                                                        } else {
                                                            *(*s)
                                                                .byte_code
                                                                .buf
                                                                .offset(last_atom_start as isize) =
                                                                (REOP_split_goto_first as i32
                                                                    + greedy)
                                                                    as u8;
                                                            put_u32(
                                                                (*s).byte_code
                                                                    .buf
                                                                    .offset(
                                                                        last_atom_start as isize,
                                                                    )
                                                                    .offset(1 as i32 as isize),
                                                                (len_0
                                                                    + 5 as i32
                                                                    + add_zero_advance_check)
                                                                    as u32,
                                                            );
                                                            if add_zero_advance_check != 0 {
                                                                *(*s).byte_code.buf.offset(
                                                                    (last_atom_start
                                                                        + 1 as i32
                                                                        + 4 as i32)
                                                                        as isize,
                                                                ) = REOP_push_char_pos as i32 as u8;
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_bne_char_pos as i32,
                                                                    last_atom_start as u32,
                                                                );
                                                            } else {
                                                                re_emit_goto(
                                                                    s,
                                                                    REOP_goto as i32,
                                                                    last_atom_start as u32,
                                                                );
                                                            }
                                                            current_block = 9856786070414082169;
                                                        }
                                                    } else if dbuf_insert(
                                                        &mut (*s).byte_code,
                                                        last_atom_start,
                                                        10 as i32,
                                                    ) != 0
                                                    {
                                                        current_block = 5210424319564767178;
                                                    } else {
                                                        pos_1 = last_atom_start;
                                                        let fresh24 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *(*s)
                                                            .byte_code
                                                            .buf
                                                            .offset(fresh24 as isize) =
                                                            REOP_push_i32 as i32 as u8;
                                                        put_u32(
                                                            (*s).byte_code
                                                                .buf
                                                                .offset(pos_1 as isize),
                                                            quant_max as u32,
                                                        );
                                                        pos_1 += 4 as i32;
                                                        let fresh25 = pos_1;
                                                        pos_1 = pos_1 + 1;
                                                        *(*s)
                                                            .byte_code
                                                            .buf
                                                            .offset(fresh25 as isize) =
                                                            (REOP_split_goto_first as i32 + greedy)
                                                                as u8;
                                                        put_u32(
                                                            (*s).byte_code
                                                                .buf
                                                                .offset(pos_1 as isize),
                                                            (len_0 + 5 as i32) as u32,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as i32,
                                                            (last_atom_start + 5 as i32) as u32,
                                                        );
                                                        re_emit_op(s, REOP_drop as i32);
                                                        current_block = 9856786070414082169;
                                                    }
                                                }
                                            }
                                        } else if quant_min == 1 as i32
                                            && quant_max == 2147483647 as i32
                                            && add_zero_advance_check == 0
                                        {
                                            re_emit_goto(
                                                s,
                                                REOP_split_next_first as i32 - greedy,
                                                last_atom_start as u32,
                                            );
                                            current_block = 9856786070414082169;
                                        } else {
                                            if quant_min == 1 as i32 {
                                                current_block = 5684771287319053842;
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                5 as i32,
                                            ) != 0
                                            {
                                                current_block = 5210424319564767178;
                                            } else {
                                                *(*s)
                                                    .byte_code
                                                    .buf
                                                    .offset(last_atom_start as isize) =
                                                    REOP_push_i32 as i32 as u8;
                                                put_u32(
                                                    (*s).byte_code
                                                        .buf
                                                        .offset(last_atom_start as isize)
                                                        .offset(1 as i32 as isize),
                                                    quant_min as u32,
                                                );
                                                last_atom_start += 5 as i32;
                                                re_emit_goto(
                                                    s,
                                                    REOP_loop as i32,
                                                    last_atom_start as u32,
                                                );
                                                re_emit_op(s, REOP_drop as i32);
                                                current_block = 5684771287319053842;
                                            }
                                            match current_block {
                                                5210424319564767178 => {}
                                                _ => {
                                                    if quant_max == 2147483647 as i32 {
                                                        pos_1 = (*s).byte_code.size as i32;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as i32 + greedy,
                                                            (len_0
                                                                + 5 as i32
                                                                + add_zero_advance_check)
                                                                as u32,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_op(
                                                                s,
                                                                REOP_push_char_pos as i32,
                                                            );
                                                        }
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as usize,
                                                            len_0 as usize,
                                                        );
                                                        if add_zero_advance_check != 0 {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_bne_char_pos as i32,
                                                                pos_1 as u32,
                                                            );
                                                        } else {
                                                            re_emit_goto(
                                                                s,
                                                                REOP_goto as i32,
                                                                pos_1 as u32,
                                                            );
                                                        }
                                                    } else if quant_max > quant_min {
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_push_i32 as i32,
                                                            (quant_max - quant_min) as u32,
                                                        );
                                                        pos_1 = (*s).byte_code.size as i32;
                                                        re_emit_op_u32(
                                                            s,
                                                            REOP_split_goto_first as i32 + greedy,
                                                            (len_0 + 5 as i32) as u32,
                                                        );
                                                        dbuf_put_self(
                                                            &mut (*s).byte_code,
                                                            last_atom_start as usize,
                                                            len_0 as usize,
                                                        );
                                                        re_emit_goto(
                                                            s,
                                                            REOP_loop as i32,
                                                            pos_1 as u32,
                                                        );
                                                        re_emit_op(s, REOP_drop as i32);
                                                    }
                                                    current_block = 9856786070414082169;
                                                }
                                            }
                                        }
                                        match current_block {
                                            5210424319564767178 => {}
                                            _ => {
                                                last_atom_start = -(1 as i32);
                                                current_block = 3543436503030046430;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    3543436503030046430 => {}
                                    _ => return re_parse_out_of_memory(s),
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    (*s).buf_ptr = p;
    return 0 as i32;
}
unsafe fn re_parse_alternative(mut s: *mut REParseState, mut is_backward_dir: BOOL) -> i32 {
    let mut p: *const u8 = 0 as *const u8;
    let mut ret: i32 = 0;
    let mut start: usize = 0;
    let mut term_start: usize = 0;
    let mut end: usize = 0;
    let mut term_size: usize = 0;
    start = (*s).byte_code.size;
    loop {
        p = (*s).buf_ptr;
        if p >= (*s).buf_end {
            break;
        }
        if *p as i32 == '|' as i32 || *p as i32 == ')' as i32 {
            break;
        }
        term_start = (*s).byte_code.size;
        ret = re_parse_term(s, is_backward_dir);
        if ret != 0 {
            return ret;
        }
        if is_backward_dir != 0 {
            /* reverse the order of the terms (XXX: inefficient, but
            speed is not really critical here) */
            end = (*s).byte_code.size;
            term_size = end.wrapping_sub(term_start);
            if dbuf_realloc(&mut (*s).byte_code, end.wrapping_add(term_size)) != 0 {
                return -(1 as i32);
            }
            ((*s)
                .byte_code
                .buf
                .offset(start as isize)
                .offset(term_size as isize) as *mut u8)
                .copy_from_nonoverlapping(
                    (*s).byte_code.buf.offset(start as isize) as *const u8,
                    end.wrapping_sub(start) as usize,
                );
            ((*s).byte_code.buf.offset(start as isize) as *mut u8).copy_from(
                (*s).byte_code.buf.offset(end as isize) as *const u8,
                term_size as usize,
            );
        }
    }
    return 0 as i32;
}
unsafe fn re_parse_disjunction(mut s: *mut REParseState, mut is_backward_dir: BOOL) -> i32 {
    let mut start: i32 = 0;
    let mut len: i32 = 0;
    let mut pos: i32 = 0;
    if lre_check_stack_overflow((*s).opaque, 0 as i32 as usize) != 0 {
        return re_parse_error(s, "stack overflow");
    }
    start = (*s).byte_code.size as i32;
    if re_parse_alternative(s, is_backward_dir) != 0 {
        return -(1 as i32);
    }
    while *(*s).buf_ptr as i32 == '|' as i32 {
        (*s).buf_ptr = (*s).buf_ptr.offset(1);
        len = (*s).byte_code.size.wrapping_sub(start as usize) as i32;
        /* insert a split before the first alternative */
        if dbuf_insert(&mut (*s).byte_code, start, 5 as i32) != 0 {
            return re_parse_out_of_memory(s);
        }
        *(*s).byte_code.buf.offset(start as isize) = REOP_split_next_first as i32 as u8;
        put_u32(
            (*s).byte_code
                .buf
                .offset(start as isize)
                .offset(1 as i32 as isize),
            (len + 5 as i32) as u32,
        );
        pos = re_emit_op_u32(s, REOP_goto as i32, 0 as i32 as u32);
        if re_parse_alternative(s, is_backward_dir) != 0 {
            return -(1 as i32);
        }
        /* patch the goto */
        len = (*s).byte_code.size.wrapping_sub((pos as usize + 4)) as i32;
        put_u32((*s).byte_code.buf.offset(pos as isize), len as u32);
    }
    return 0 as i32;
}
/* the control flow is recursive so the analysis can be linear */
unsafe fn compute_stack_size(mut bc_buf: *const u8, mut bc_buf_len: i32) -> i32 {
    let mut stack_size: i32 = 0;
    let mut stack_size_max: i32 = 0;
    let mut pos: i32 = 0;
    let mut opcode: i32 = 0;
    let mut len: i32 = 0;
    let mut val: u32 = 0;
    stack_size = 0 as i32;
    stack_size_max = 0 as i32;
    bc_buf = bc_buf.offset(7 as i32 as isize);
    bc_buf_len -= 7 as i32;
    pos = 0 as i32;
    while pos < bc_buf_len {
        opcode = *bc_buf.offset(pos as isize) as i32;
        len = reopcode_info[opcode as usize].size as i32;
        if opcode < REOP_COUNT as i32 {
        } else {
            assert!(opcode < REOP_COUNT as i32);
        }
        if pos + len <= bc_buf_len {
        } else {
            assert!(pos + len <= bc_buf_len);
        }
        match opcode {
            15 | 25 => {
                stack_size += 1;
                if stack_size > stack_size_max {
                    if stack_size > 255 as i32 {
                        return -(1 as i32);
                    }
                    stack_size_max = stack_size
                }
            }
            16 | 26 => {
                if stack_size > 0 as i32 {
                } else {
                    assert!(stack_size > 0);
                }
                stack_size -= 1
            }
            21 => {
                val = get_u16(bc_buf.offset(pos as isize).offset(1 as i32 as isize));
                len = (len as u32).wrapping_add(val.wrapping_mul(4 as i32 as u32)) as i32 as i32
            }
            22 => {
                val = get_u16(bc_buf.offset(pos as isize).offset(1 as i32 as isize));
                len = (len as u32).wrapping_add(val.wrapping_mul(8 as i32 as u32)) as i32 as i32
            }
            _ => {}
        }
        pos += len
    }
    return stack_size_max;
}
/* 'buf' must be a zero terminated UTF-8 string of length buf_len.
   Return NULL if error and allocate an error message in *perror_msg,
   otherwise the compiled bytecode and its length in plen.
*/
#[no_mangle]
pub unsafe fn lre_compile(
    mut plen: *mut i32,
    mut error_msg: *mut std::os::raw::c_char,
    mut error_msg_size: i32,
    mut buf: *const std::os::raw::c_char,
    mut buf_len: usize,
    mut re_flags: i32,
    mut opaque: *mut std::ffi::c_void,
) -> *mut u8 {
    let mut s_s: REParseState = REParseState {
        byte_code: DynBuf {
            buf: 0 as *mut u8,
            size: 0,
            allocated_size: 0,
            error: 0,
            realloc_func: None,
            opaque: 0 as *mut std::ffi::c_void,
        },
        buf_ptr: 0 as *const u8,
        buf_end: 0 as *const u8,
        buf_start: 0 as *const u8,
        re_flags: 0,
        is_utf16: 0,
        ignore_case: 0,
        dotall: 0,
        capture_count: 0,
        total_capture_count: 0,
        has_named_captures: 0,
        opaque: 0 as *mut std::ffi::c_void,
        group_names: DynBuf {
            buf: 0 as *mut u8,
            size: 0,
            allocated_size: 0,
            error: 0,
            realloc_func: None,
            opaque: 0 as *mut std::ffi::c_void,
        },
        u: REParseStateUnion {
            error_msg: [0; 128],
        },
    }; /* first element is the flags */
    let mut s: *mut REParseState = &mut s_s; /* second element is the number of captures */
    let mut stack_size: i32 = 0; /* stack size */
    let mut is_sticky: BOOL = 0; /* bytecode length */

    (s as *mut u8).write_bytes(0, std::mem::size_of::<REParseState>());

    (*s).opaque = opaque;
    (*s).buf_ptr = buf as *const u8;
    (*s).buf_end = (*s).buf_ptr.offset(buf_len as isize);
    (*s).buf_start = (*s).buf_ptr;
    (*s).re_flags = re_flags;
    (*s).is_utf16 = (re_flags & (1 as i32) << 4 as i32 != 0 as i32) as i32;
    is_sticky = (re_flags & (1 as i32) << 5 as i32 != 0 as i32) as i32;
    (*s).ignore_case = (re_flags & (1 as i32) << 1 as i32 != 0 as i32) as i32;
    (*s).dotall = (re_flags & (1 as i32) << 3 as i32 != 0 as i32) as i32;
    (*s).capture_count = 1 as i32;
    (*s).total_capture_count = -(1 as i32);
    (*s).has_named_captures = -(1 as i32);
    dbuf_init2(
        &mut (*s).byte_code,
        opaque,
        Some(
            lre_realloc
                as unsafe fn(
                    _: *mut std::ffi::c_void,
                    _: *mut std::ffi::c_void,
                    _: usize,
                ) -> *mut std::ffi::c_void,
        ),
    );
    dbuf_init2(
        &mut (*s).group_names,
        opaque,
        Some(
            lre_realloc
                as unsafe fn(
                    _: *mut std::ffi::c_void,
                    _: *mut std::ffi::c_void,
                    _: usize,
                ) -> *mut std::ffi::c_void,
        ),
    );
    dbuf_putc(&mut (*s).byte_code, re_flags as u8);
    dbuf_putc(&mut (*s).byte_code, 0 as i32 as u8);
    dbuf_putc(&mut (*s).byte_code, 0 as i32 as u8);
    dbuf_put_u32(&mut (*s).byte_code, 0 as i32 as u32);
    if is_sticky == 0 {
        /* iterate thru all positions (about the same as .*?( ... ) )
        .  We do it without an explicit loop so that lock step
        thread execution will be possible in an optimized
        implementation */
        re_emit_op_u32(
            s,
            REOP_split_goto_first as i32,
            (1 as i32 + 5 as i32) as u32,
        );
        re_emit_op(s, REOP_any as i32);
        re_emit_op_u32(
            s,
            REOP_goto as i32,
            -(5 as i32 + 1 as i32 + 5 as i32) as u32,
        );
    }
    re_emit_op_u8(s, REOP_save_start as i32, 0 as i32 as u32);
    if !(re_parse_disjunction(s, FALSE as i32) != 0) {
        re_emit_op_u8(s, REOP_save_end as i32, 0 as i32 as u32);
        re_emit_op(s, REOP_match as i32);
        if *(*s).buf_ptr as i32 != '\u{0}' as i32 {
            re_parse_error(s, "extraneous characters at the end");
        } else if dbuf_error(&mut (*s).byte_code) != 0 {
            re_parse_out_of_memory(s);
        } else {
            stack_size = compute_stack_size((*s).byte_code.buf, (*s).byte_code.size as i32);
            if stack_size < 0 as i32 {
                re_parse_error(s, "too many imbricated quantifiers");
            } else {
                *(*s).byte_code.buf.offset(1 as i32 as isize) = (*s).capture_count as u8;
                *(*s).byte_code.buf.offset(2 as i32 as isize) = stack_size as u8;
                put_u32(
                    (*s).byte_code.buf.offset(3 as i32 as isize),
                    (*s).byte_code.size.wrapping_sub(7) as u32,
                );
                /* add the named groups if needed */
                if (*s).group_names.size > ((*s).capture_count - 1) as usize {
                    dbuf_put(
                        &mut (*s).byte_code,
                        (*s).group_names.buf,
                        (*s).group_names.size as usize,
                    );
                    let ref mut fresh26 = *(*s).byte_code.buf.offset(0 as i32 as isize);
                    *fresh26 = (*fresh26 as i32 | (1 as i32) << 7 as i32) as u8
                }
                dbuf_free(&mut (*s).group_names);
                *error_msg.offset(0 as i32 as isize) = '\u{0}' as i32 as std::os::raw::c_char;
                *plen = (*s).byte_code.size as i32;
                return (*s).byte_code.buf;
            }
        }
    }
    dbuf_free(&mut (*s).byte_code);
    dbuf_free(&mut (*s).group_names);
    pstrcpy(error_msg, error_msg_size, (*s).u.error_msg.as_mut_ptr());
    *plen = 0 as i32;
    return 0 as *mut u8;
}
unsafe fn is_line_terminator(mut c: u32) -> BOOL {
    return (c == '\n' as i32 as u32
        || c == '\r' as i32 as u32
        || c == 0x2028 as i32 as u32
        || c == 0x2029 as i32 as u32) as i32;
}
unsafe fn is_word_char(mut c: u32) -> BOOL {
    return (c >= '0' as i32 as u32 && c <= '9' as i32 as u32
        || c >= 'a' as i32 as u32 && c <= 'z' as i32 as u32
        || c >= 'A' as i32 as u32 && c <= 'Z' as i32 as u32
        || c == '_' as i32 as u32) as i32;
}
unsafe fn push_state(
    mut s: *mut REExecContext,
    mut capture: *mut *mut u8,
    mut stack: *mut StackInt,
    mut stack_len: usize,
    mut pc: *const u8,
    mut cptr: *const u8,
    mut type_0: REExecStateEnum,
    mut count: usize,
) -> i32 {
    let mut rs: *mut REExecState = 0 as *mut REExecState;
    let mut new_stack: *mut u8 = 0 as *mut u8;
    let mut new_size: usize = 0;
    let mut i: usize = 0;
    let mut n: usize = 0;
    let mut stack_buf: *mut StackInt = 0 as *mut StackInt;
    if ((*s).state_stack_len.wrapping_add(1) > (*s).state_stack_size) as i32 as i64 != 0 {
        /* reallocate the stack */
        new_size = (*s).state_stack_size.wrapping_mul(3).wrapping_div(2);
        if new_size < 8 {
            new_size = 8
        }
        new_stack = lre_realloc(
            (*s).opaque,
            (*s).state_stack as *mut std::ffi::c_void,
            new_size.wrapping_mul((*s).state_size),
        ) as *mut u8;
        if new_stack.is_null() {
            return -(1 as i32);
        }
        (*s).state_stack_size = new_size;
        (*s).state_stack = new_stack
    }
    rs = (*s)
        .state_stack
        .offset((*s).state_stack_len.wrapping_mul((*s).state_size) as isize)
        as *mut REExecState;
    (*s).state_stack_len = (*s).state_stack_len.wrapping_add(1);
    (*rs).set_type_0(type_0);
    (*rs).count = count;
    (*rs).stack_len = stack_len as u8;
    (*rs).cptr = cptr;
    (*rs).pc = pc;
    n = (2 as i32 * (*s).capture_count) as usize;
    i = 0 as i32 as usize;
    while i < n {
        let ref mut fresh27 = *(*rs).buf.as_mut_ptr().offset(i as isize);
        *fresh27 = *capture.offset(i as isize) as *mut std::ffi::c_void;
        i = i.wrapping_add(1)
    }
    stack_buf = (*rs).buf.as_mut_ptr().offset(n as isize) as *mut StackInt;
    i = 0 as i32 as usize;
    while i < stack_len {
        *stack_buf.offset(i as isize) = *stack.offset(i as isize);
        i = i.wrapping_add(1)
    }
    return 0 as i32;
}
/* return 1 if match, 0 if not match or -1 if error. */
unsafe fn lre_exec_backtrack(
    mut s: *mut REExecContext,
    mut capture: *mut *mut u8,
    mut stack: *mut StackInt,
    mut stack_len: i32,
    mut pc: *const u8,
    mut cptr: *const u8,
    mut no_recurse: BOOL,
) -> intptr_t {
    let mut rs: *mut REExecState = 0 as *mut REExecState;
    let mut current_block: u64;
    let mut opcode: i32 = 0;
    let mut ret: i32 = 0;
    let mut cbuf_type: i32 = 0;
    let mut val: u32 = 0;
    let mut c: u32 = 0;
    let mut cbuf_end: *const u8 = 0 as *const u8;
    cbuf_type = (*s).cbuf_type;
    cbuf_end = (*s).cbuf_end;
    's_27: loop {
        //        printf("top=%p: pc=%d\n", th_list.top, (int)(pc - (bc_buf + RE_HEADER_LEN)));
        let fresh28 = pc;
        pc = pc.offset(1);
        opcode = *fresh28 as i32;
        match opcode {
            10 => {
                rs = 0 as *mut REExecState;
                if no_recurse != 0 {
                    return cptr as intptr_t;
                }
                ret = 1 as i32;
                current_block = 1027844253497686655;
            }
            2 => {
                val = get_u32(pc);
                pc = pc.offset(4 as i32 as isize);
                current_block = 9535040653783544971;
            }
            1 => {
                val = get_u16(pc);
                pc = pc.offset(2 as i32 as isize);
                current_block = 9535040653783544971;
            }
            8 | 9 => {
                let mut pc1: *const u8 = 0 as *const u8;
                val = get_u32(pc);
                pc = pc.offset(4 as i32 as isize);
                if opcode == REOP_split_next_first as i32 {
                    pc1 = pc.offset(val as i32 as isize)
                } else {
                    pc1 = pc;
                    pc = pc.offset(val as i32 as isize)
                }
                ret = push_state(
                    s,
                    capture,
                    stack,
                    stack_len as usize,
                    pc1,
                    cptr,
                    RE_EXEC_STATE_SPLIT,
                    0 as i32 as usize,
                );
                if ret < 0 as i32 {
                    return -(1 as i32) as intptr_t;
                }
                continue;
            }
            23 | 24 => {
                val = get_u32(pc);
                pc = pc.offset(4 as i32 as isize);
                ret = push_state(
                    s,
                    capture,
                    stack,
                    stack_len as usize,
                    pc.offset(val as i32 as isize),
                    cptr,
                    (RE_EXEC_STATE_LOOKAHEAD as i32 + opcode - REOP_lookahead as i32)
                        as REExecStateEnum,
                    0 as i32 as usize,
                );
                if ret < 0 as i32 {
                    return -(1 as i32) as intptr_t;
                }
                continue;
            }
            7 => {
                val = get_u32(pc);
                pc = pc.offset((4 as i32 + val as i32) as isize);
                continue;
            }
            5 => {
                if cptr == (*s).cbuf {
                    continue;
                }
                if (*s).multi_line == 0 {
                    current_block = 14487425527653873875;
                } else {
                    if cbuf_type == 0 as i32 {
                        c = *cptr.offset(-(1 as i32) as isize) as u32
                    } else {
                        let mut __c1_0: u32 = 0;
                        c = *(cptr as *mut u16).offset(-(1 as i32) as isize) as u32;
                        if c >= 0xdc00 as i32 as u32
                            && c < 0xe000 as i32 as u32
                            && cbuf_type == 2 as i32
                            && cptr.offset(-(4 as i32 as isize)) >= (*s).cbuf
                        {
                            __c1_0 = *(cptr as *mut u16).offset(-(2 as i32) as isize) as u32;
                            if __c1_0 >= 0xd800 as i32 as u32 && __c1_0 < 0xdc00 as i32 as u32 {
                                c = ((__c1_0 & 0x3ff as i32 as u32) << 10 as i32
                                    | c & 0x3ff as i32 as u32)
                                    .wrapping_add(0x10000 as i32 as u32)
                            }
                        }
                    }
                    if !(is_line_terminator(c) == 0) {
                        continue;
                    }
                    current_block = 14487425527653873875;
                }
            }
            6 => {
                if cptr == cbuf_end {
                    continue;
                }
                if (*s).multi_line == 0 {
                    current_block = 14487425527653873875;
                } else {
                    if cbuf_type == 0 as i32 {
                        c = *cptr.offset(0 as i32 as isize) as u32
                    } else {
                        let mut __c1_1: u32 = 0;
                        c = *(cptr as *mut u16).offset(0 as i32 as isize) as u32;
                        if c >= 0xd800 as i32 as u32
                            && c < 0xdc00 as i32 as u32
                            && cbuf_type == 2 as i32
                            && cptr.offset(2 as i32 as isize) < cbuf_end
                        {
                            __c1_1 = *(cptr as *mut u16).offset(1 as i32 as isize) as u32;
                            if __c1_1 >= 0xdc00 as i32 as u32 && __c1_1 < 0xe000 as i32 as u32 {
                                c = ((c & 0x3ff as i32 as u32) << 10 as i32
                                    | __c1_1 & 0x3ff as i32 as u32)
                                    .wrapping_add(0x10000 as i32 as u32)
                            }
                        }
                    }
                    if !(is_line_terminator(c) == 0) {
                        continue;
                    }
                    current_block = 14487425527653873875;
                }
            }
            3 => {
                if cptr == cbuf_end {
                    current_block = 14487425527653873875;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh30 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh30 as u32
                    } else {
                        let mut __c1_2: u32 = 0;
                        c = *(cptr as *mut u16) as u32;
                        cptr = cptr.offset(2 as i32 as isize);
                        if c >= 0xd800 as i32 as u32
                            && c < 0xdc00 as i32 as u32
                            && cbuf_type == 2 as i32
                            && cptr < cbuf_end
                        {
                            __c1_2 = *(cptr as *mut u16) as u32;
                            if __c1_2 >= 0xdc00 as i32 as u32 && __c1_2 < 0xe000 as i32 as u32 {
                                c = ((c & 0x3ff as i32 as u32) << 10 as i32
                                    | __c1_2 & 0x3ff as i32 as u32)
                                    .wrapping_add(0x10000 as i32 as u32);
                                cptr = cptr.offset(2 as i32 as isize)
                            }
                        }
                    }
                    if !(is_line_terminator(c) != 0) {
                        continue;
                    }
                    current_block = 14487425527653873875;
                }
            }
            4 => {
                if cptr == cbuf_end {
                    current_block = 14487425527653873875;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh31 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh31 as u32
                    } else {
                        let mut __c1_3: u32 = 0;
                        c = *(cptr as *mut u16) as u32;
                        cptr = cptr.offset(2 as i32 as isize);
                        if c >= 0xd800 as i32 as u32
                            && c < 0xdc00 as i32 as u32
                            && cbuf_type == 2 as i32
                            && cptr < cbuf_end
                        {
                            __c1_3 = *(cptr as *mut u16) as u32;
                            if __c1_3 >= 0xdc00 as i32 as u32 && __c1_3 < 0xe000 as i32 as u32 {
                                c = ((c & 0x3ff as i32 as u32) << 10 as i32
                                    | __c1_3 & 0x3ff as i32 as u32)
                                    .wrapping_add(0x10000 as i32 as u32);
                                cptr = cptr.offset(2 as i32 as isize)
                            }
                        }
                    }
                    continue;
                }
            }
            11 | 12 => {
                let fresh32 = pc;
                pc = pc.offset(1);
                val = *fresh32 as u32;
                if val < (*s).capture_count as u32 {
                } else {
                    assert!(val < (*s).capture_count as u32);
                }
                let ref mut fresh33 = *capture.offset(
                    (2 as i32 as u32)
                        .wrapping_mul(val)
                        .wrapping_add(opcode as u32)
                        .wrapping_sub(REOP_save_start as i32 as u32) as isize,
                );
                *fresh33 = cptr as *mut u8;
                continue;
            }
            13 => {
                let mut val2: u32 = 0;
                val = *pc.offset(0 as i32 as isize) as u32;
                val2 = *pc.offset(1 as i32 as isize) as u32;
                pc = pc.offset(2 as i32 as isize);
                if val2 < (*s).capture_count as u32 {
                } else {
                    assert!(val2 < (*s).capture_count as u32);
                }
                while val <= val2 {
                    let ref mut fresh34 =
                        *capture.offset((2 as i32 as u32).wrapping_mul(val) as isize);
                    *fresh34 = 0 as *mut u8;
                    let ref mut fresh35 = *capture.offset(
                        (2 as i32 as u32)
                            .wrapping_mul(val)
                            .wrapping_add(1 as i32 as u32) as isize,
                    );
                    *fresh35 = 0 as *mut u8;
                    val = val.wrapping_add(1)
                }
                continue;
            }
            15 => {
                val = get_u32(pc);
                pc = pc.offset(4 as i32 as isize);
                let fresh36 = stack_len;
                stack_len = stack_len + 1;
                *stack.offset(fresh36 as isize) = val as StackInt;
                continue;
            }
            16 => {
                stack_len -= 1;
                continue;
            }
            14 => {
                val = get_u32(pc);
                pc = pc.offset(4 as i32 as isize);
                let ref mut fresh37 = *stack.offset((stack_len - 1 as i32) as isize);
                *fresh37 = (*fresh37).wrapping_sub(1);
                if *fresh37 != 0 {
                    pc = pc.offset(val as isize)
                }
                continue;
            }
            25 => {
                let fresh38 = stack_len;
                stack_len = stack_len + 1;
                *stack.offset(fresh38 as isize) = cptr as uintptr_t;
                continue;
            }
            26 => {
                val = get_u32(pc);
                pc = pc.offset(4 as i32 as isize);
                stack_len -= 1;
                if *stack.offset(stack_len as isize) != cptr as uintptr_t {
                    pc = pc.offset(val as i32 as isize)
                }
                continue;
            }
            17 | 18 => {
                let mut v1: BOOL = 0;
                let mut v2: BOOL = 0;
                /* char before */
                if cptr == (*s).cbuf {
                    v1 = FALSE as i32
                } else {
                    if cbuf_type == 0 as i32 {
                        c = *cptr.offset(-(1 as i32) as isize) as u32
                    } else {
                        let mut __c1_4: u32 = 0;
                        c = *(cptr as *mut u16).offset(-(1 as i32) as isize) as u32;
                        if c >= 0xdc00 as i32 as u32
                            && c < 0xe000 as i32 as u32
                            && cbuf_type == 2 as i32
                            && cptr.offset(-(4 as i32 as isize)) >= (*s).cbuf
                        {
                            __c1_4 = *(cptr as *mut u16).offset(-(2 as i32) as isize) as u32;
                            if __c1_4 >= 0xd800 as i32 as u32 && __c1_4 < 0xdc00 as i32 as u32 {
                                c = ((__c1_4 & 0x3ff as i32 as u32) << 10 as i32
                                    | c & 0x3ff as i32 as u32)
                                    .wrapping_add(0x10000 as i32 as u32)
                            }
                        }
                    }
                    v1 = is_word_char(c)
                }
                /* current char */
                if cptr >= cbuf_end {
                    v2 = FALSE as i32
                } else {
                    if cbuf_type == 0 as i32 {
                        c = *cptr.offset(0 as i32 as isize) as u32
                    } else {
                        let mut __c1_5: u32 = 0; /* n must be >= 1 */
                        c = *(cptr as *mut u16).offset(0 as i32 as isize) as u32;
                        if c >= 0xd800 as i32 as u32
                            && c < 0xdc00 as i32 as u32
                            && cbuf_type == 2 as i32
                            && cptr.offset(2 as i32 as isize) < cbuf_end
                        {
                            __c1_5 = *(cptr as *mut u16).offset(1 as i32 as isize) as u32;
                            if __c1_5 >= 0xdc00 as i32 as u32 && __c1_5 < 0xe000 as i32 as u32 {
                                c = ((c & 0x3ff as i32 as u32) << 10 as i32
                                    | __c1_5 & 0x3ff as i32 as u32)
                                    .wrapping_add(0x10000 as i32 as u32)
                            }
                        }
                    }
                    v2 = is_word_char(c)
                }
                if !(v1 ^ v2 ^ REOP_not_word_boundary as i32 - opcode != 0) {
                    continue;
                }
                current_block = 14487425527653873875;
            }
            19 | 20 => {
                let mut cptr1: *const u8 = 0 as *const u8;
                let mut cptr1_end: *const u8 = 0 as *const u8;
                let mut cptr1_start: *const u8 = 0 as *const u8;
                let mut c1: u32 = 0;
                let mut c2: u32 = 0;
                let fresh39 = pc;
                pc = pc.offset(1);
                val = *fresh39 as u32;
                if val >= (*s).capture_count as u32 {
                    current_block = 14487425527653873875;
                } else {
                    cptr1_start = *capture.offset((2 as i32 as u32).wrapping_mul(val) as isize);
                    cptr1_end = *capture.offset(
                        (2 as i32 as u32)
                            .wrapping_mul(val)
                            .wrapping_add(1 as i32 as u32) as isize,
                    );
                    if cptr1_start.is_null() || cptr1_end.is_null() {
                        continue;
                    }
                    if opcode == REOP_back_reference as i32 {
                        cptr1 = cptr1_start;
                        loop {
                            if !(cptr1 < cptr1_end) {
                                continue 's_27;
                            }
                            if cptr >= cbuf_end {
                                break;
                            }
                            if cbuf_type == 0 as i32 {
                                let fresh40 = cptr1;
                                cptr1 = cptr1.offset(1);
                                c1 = *fresh40 as u32
                            } else {
                                let mut __c1_6: u32 = 0;
                                c1 = *(cptr1 as *mut u16) as u32;
                                cptr1 = cptr1.offset(2 as i32 as isize);
                                if c1 >= 0xd800 as i32 as u32
                                    && c1 < 0xdc00 as i32 as u32
                                    && cbuf_type == 2 as i32
                                    && cptr1 < cptr1_end
                                {
                                    __c1_6 = *(cptr1 as *mut u16) as u32;
                                    if __c1_6 >= 0xdc00 as i32 as u32
                                        && __c1_6 < 0xe000 as i32 as u32
                                    {
                                        c1 = ((c1 & 0x3ff as i32 as u32) << 10 as i32
                                            | __c1_6 & 0x3ff as i32 as u32)
                                            .wrapping_add(0x10000 as i32 as u32);
                                        cptr1 = cptr1.offset(2 as i32 as isize)
                                    }
                                }
                            }
                            if cbuf_type == 0 as i32 {
                                let fresh41 = cptr;
                                cptr = cptr.offset(1);
                                c2 = *fresh41 as u32
                            } else {
                                let mut __c1_7: u32 = 0;
                                c2 = *(cptr as *mut u16) as u32;
                                cptr = cptr.offset(2 as i32 as isize);
                                if c2 >= 0xd800 as i32 as u32
                                    && c2 < 0xdc00 as i32 as u32
                                    && cbuf_type == 2 as i32
                                    && cptr < cbuf_end
                                {
                                    __c1_7 = *(cptr as *mut u16) as u32;
                                    if __c1_7 >= 0xdc00 as i32 as u32
                                        && __c1_7 < 0xe000 as i32 as u32
                                    {
                                        c2 = ((c2 & 0x3ff as i32 as u32) << 10 as i32
                                            | __c1_7 & 0x3ff as i32 as u32)
                                            .wrapping_add(0x10000 as i32 as u32);
                                        cptr = cptr.offset(2 as i32 as isize)
                                    }
                                }
                            }
                            if (*s).ignore_case != 0 {
                                c1 = lre_canonicalize(c1, (*s).is_utf16);
                                c2 = lre_canonicalize(c2, (*s).is_utf16)
                            }
                            if c1 != c2 {
                                break;
                            }
                        }
                    } else {
                        cptr1 = cptr1_end;
                        loop {
                            if !(cptr1 > cptr1_start) {
                                continue 's_27;
                            }
                            if cptr == (*s).cbuf {
                                break;
                            }
                            if cbuf_type == 0 as i32 {
                                cptr1 = cptr1.offset(-1);
                                c1 = *cptr1.offset(0 as i32 as isize) as u32
                            } else {
                                let mut __c1_8: u32 = 0;
                                cptr1 = cptr1.offset(-(2 as i32 as isize));
                                c1 = *(cptr1 as *mut u16).offset(0 as i32 as isize) as u32;
                                if c1 >= 0xdc00 as i32 as u32
                                    && c1 < 0xe000 as i32 as u32
                                    && cbuf_type == 2 as i32
                                    && cptr1 > cptr1_start
                                {
                                    __c1_8 =
                                        *(cptr1 as *mut u16).offset(-(1 as i32) as isize) as u32;
                                    if __c1_8 >= 0xd800 as i32 as u32
                                        && __c1_8 < 0xdc00 as i32 as u32
                                    {
                                        cptr1 = cptr1.offset(-(2 as i32 as isize));
                                        c1 = ((__c1_8 & 0x3ff as i32 as u32) << 10 as i32
                                            | c1 & 0x3ff as i32 as u32)
                                            .wrapping_add(0x10000 as i32 as u32)
                                    }
                                }
                            }
                            if cbuf_type == 0 as i32 {
                                cptr = cptr.offset(-1);
                                c2 = *cptr.offset(0 as i32 as isize) as u32
                            } else {
                                let mut __c1_9: u32 = 0;
                                cptr = cptr.offset(-(2 as i32 as isize));
                                c2 = *(cptr as *mut u16).offset(0 as i32 as isize) as u32;
                                if c2 >= 0xdc00 as i32 as u32
                                    && c2 < 0xe000 as i32 as u32
                                    && cbuf_type == 2 as i32
                                    && cptr > (*s).cbuf
                                {
                                    __c1_9 =
                                        *(cptr as *mut u16).offset(-(1 as i32) as isize) as u32;
                                    if __c1_9 >= 0xd800 as i32 as u32
                                        && __c1_9 < 0xdc00 as i32 as u32
                                    {
                                        cptr = cptr.offset(-(2 as i32 as isize));
                                        c2 = ((__c1_9 & 0x3ff as i32 as u32) << 10 as i32
                                            | c2 & 0x3ff as i32 as u32)
                                            .wrapping_add(0x10000 as i32 as u32)
                                    }
                                }
                            }
                            if (*s).ignore_case != 0 {
                                c1 = lre_canonicalize(c1, (*s).is_utf16);
                                c2 = lre_canonicalize(c2, (*s).is_utf16)
                            }
                            if c1 != c2 {
                                break;
                            }
                        }
                    }
                    current_block = 14487425527653873875;
                }
            }
            21 => {
                let mut n: i32 = 0;
                let mut low: u32 = 0;
                let mut high: u32 = 0;
                let mut idx_min: u32 = 0;
                let mut idx_max: u32 = 0;
                let mut idx: u32 = 0;
                n = get_u16(pc) as i32;
                pc = pc.offset(2 as i32 as isize);
                if cptr >= cbuf_end {
                    current_block = 14487425527653873875;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh42 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh42 as u32
                    } else {
                        let mut __c1_10: u32 = 0;
                        c = *(cptr as *mut u16) as u32;
                        cptr = cptr.offset(2 as i32 as isize);
                        if c >= 0xd800 as i32 as u32
                            && c < 0xdc00 as i32 as u32
                            && cbuf_type == 2 as i32
                            && cptr < cbuf_end
                        {
                            __c1_10 = *(cptr as *mut u16) as u32;
                            if __c1_10 >= 0xdc00 as i32 as u32 && __c1_10 < 0xe000 as i32 as u32 {
                                c = ((c & 0x3ff as i32 as u32) << 10 as i32
                                    | __c1_10 & 0x3ff as i32 as u32)
                                    .wrapping_add(0x10000 as i32 as u32);
                                cptr = cptr.offset(2 as i32 as isize)
                            }
                        }
                    }
                    if (*s).ignore_case != 0 {
                        c = lre_canonicalize(c, (*s).is_utf16)
                    }
                    idx_min = 0 as i32 as u32;
                    low = get_u16(pc.offset((0 as i32 * 4 as i32) as isize));
                    if c < low {
                        current_block = 14487425527653873875;
                    } else {
                        idx_max = (n - 1 as i32) as u32;
                        high = get_u16(
                            pc.offset(idx_max.wrapping_mul(4 as i32 as u32) as isize)
                                .offset(2 as i32 as isize),
                        );
                        /* 0xffff in for last value means +infinity */
                        if (c >= 0xffff as i32 as u32) as i32 as i64 != 0
                            && high == 0xffff as i32 as u32
                        {
                            current_block = 1647272602482320956; /* n must be >= 1 */
                        } else if c > high {
                            current_block = 14487425527653873875;
                        } else {
                            loop {
                                if !(idx_min <= idx_max) {
                                    current_block = 14487425527653873875;
                                    break;
                                }
                                idx = idx_min.wrapping_add(idx_max).wrapping_div(2 as i32 as u32);
                                low =
                                    get_u16(pc.offset(idx.wrapping_mul(4 as i32 as u32) as isize));
                                high = get_u16(
                                    pc.offset(idx.wrapping_mul(4 as i32 as u32) as isize)
                                        .offset(2 as i32 as isize),
                                );
                                if c < low {
                                    idx_max = idx.wrapping_sub(1 as i32 as u32)
                                } else {
                                    if !(c > high) {
                                        current_block = 1647272602482320956;
                                        break;
                                    }
                                    idx_min = idx.wrapping_add(1 as i32 as u32)
                                }
                            }
                        }
                        match current_block {
                            14487425527653873875 => {}
                            _ => {
                                pc = pc.offset((4 as i32 * n) as isize);
                                continue;
                            }
                        }
                    }
                }
            }
            22 => {
                let mut n_0: i32 = 0;
                let mut low_0: u32 = 0;
                let mut high_0: u32 = 0;
                let mut idx_min_0: u32 = 0;
                let mut idx_max_0: u32 = 0;
                let mut idx_0: u32 = 0;
                n_0 = get_u16(pc) as i32;
                pc = pc.offset(2 as i32 as isize);
                if cptr >= cbuf_end {
                    current_block = 14487425527653873875;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh43 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh43 as u32
                    } else {
                        let mut __c1_11: u32 = 0;
                        c = *(cptr as *mut u16) as u32;
                        cptr = cptr.offset(2 as i32 as isize);
                        if c >= 0xd800 as i32 as u32
                            && c < 0xdc00 as i32 as u32
                            && cbuf_type == 2 as i32
                            && cptr < cbuf_end
                        {
                            __c1_11 = *(cptr as *mut u16) as u32;
                            if __c1_11 >= 0xdc00 as i32 as u32 && __c1_11 < 0xe000 as i32 as u32 {
                                c = ((c & 0x3ff as i32 as u32) << 10 as i32
                                    | __c1_11 & 0x3ff as i32 as u32)
                                    .wrapping_add(0x10000 as i32 as u32);
                                cptr = cptr.offset(2 as i32 as isize)
                            }
                        }
                    }
                    if (*s).ignore_case != 0 {
                        c = lre_canonicalize(c, (*s).is_utf16)
                    }
                    idx_min_0 = 0 as i32 as u32;
                    low_0 = get_u32(pc.offset((0 as i32 * 8 as i32) as isize));
                    if c < low_0 {
                        current_block = 14487425527653873875;
                    } else {
                        idx_max_0 = (n_0 - 1 as i32) as u32;
                        high_0 = get_u32(
                            pc.offset(idx_max_0.wrapping_mul(8 as i32 as u32) as isize)
                                .offset(4 as i32 as isize),
                        );
                        if c > high_0 {
                            current_block = 14487425527653873875;
                        } else {
                            loop {
                                if !(idx_min_0 <= idx_max_0) {
                                    current_block = 14487425527653873875;
                                    break;
                                }
                                idx_0 = idx_min_0
                                    .wrapping_add(idx_max_0)
                                    .wrapping_div(2 as i32 as u32);
                                low_0 = get_u32(
                                    pc.offset(idx_0.wrapping_mul(8 as i32 as u32) as isize),
                                );
                                high_0 = get_u32(
                                    pc.offset(idx_0.wrapping_mul(8 as i32 as u32) as isize)
                                        .offset(4 as i32 as isize),
                                );
                                if c < low_0 {
                                    idx_max_0 = idx_0.wrapping_sub(1 as i32 as u32)
                                } else {
                                    if !(c > high_0) {
                                        current_block = 13310972100609651845;
                                        break;
                                    }
                                    idx_min_0 = idx_0.wrapping_add(1 as i32 as u32)
                                }
                            }
                            match current_block {
                                14487425527653873875 => {}
                                _ => {
                                    pc = pc.offset((8 as i32 * n_0) as isize);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            27 => {
                /* go to the previous char */
                if cptr == (*s).cbuf {
                    current_block = 14487425527653873875;
                } else {
                    if cbuf_type == 0 as i32 {
                        cptr = cptr.offset(-1)
                    } else {
                        cptr = cptr.offset(-(2 as i32 as isize));
                        if cbuf_type == 2 as i32 {
                            c = *(cptr as *mut u16).offset(0 as i32 as isize) as u32;
                            if c >= 0xdc00 as i32 as u32
                                && c < 0xe000 as i32 as u32
                                && cptr > (*s).cbuf
                            {
                                c = *(cptr as *mut u16).offset(-(1 as i32) as isize) as u32;
                                if c >= 0xd800 as i32 as u32 && c < 0xdc00 as i32 as u32 {
                                    cptr = cptr.offset(-(2 as i32 as isize))
                                }
                            }
                        }
                    }
                    continue;
                }
            }
            28 => {
                let mut next_pos: u32 = 0;
                let mut quant_min: u32 = 0;
                let mut quant_max: u32 = 0;
                let mut q: usize = 0;
                let mut res: intptr_t = 0;
                let mut pc1_0: *const u8 = 0 as *const u8;
                next_pos = get_u32(pc);
                quant_min = get_u32(pc.offset(4 as i32 as isize));
                quant_max = get_u32(pc.offset(8 as i32 as isize));
                pc = pc.offset(16 as i32 as isize);
                pc1_0 = pc;
                pc = pc.offset(next_pos as i32 as isize);
                q = 0 as i32 as usize;
                loop {
                    res =
                        lre_exec_backtrack(s, capture, stack, stack_len, pc1_0, cptr, TRUE as i32);
                    if res == -1 {
                        return res;
                    }
                    if res == 0 {
                        break;
                    }
                    cptr = res as *mut u8;
                    q = q.wrapping_add(1);
                    if q >= quant_max as usize && quant_max != 2147483647 {
                        break;
                    }
                }
                if q < quant_min as usize {
                    current_block = 14487425527653873875;
                } else {
                    if q > quant_min as usize {
                        /* will examine all matches down to quant_min */
                        ret = push_state(
                            s,
                            capture,
                            stack,
                            stack_len as usize,
                            pc1_0.offset(-(16 as i32 as isize)),
                            cptr,
                            RE_EXEC_STATE_GREEDY_QUANT,
                            q.wrapping_sub(quant_min as usize),
                        );
                        if ret < 0 as i32 {
                            return -(1 as i32) as intptr_t;
                        }
                    }
                    continue;
                }
            }
            _ => {
                abort();
            }
        }
        match current_block {
            9535040653783544971 => {
                if cptr >= cbuf_end {
                    current_block = 14487425527653873875;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh29 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh29 as u32
                    } else {
                        let mut __c1: u32 = 0;
                        c = *(cptr as *mut u16) as u32;
                        cptr = cptr.offset(2 as i32 as isize);
                        if c >= 0xd800 as i32 as u32
                            && c < 0xdc00 as i32 as u32
                            && cbuf_type == 2 as i32
                            && cptr < cbuf_end
                        {
                            __c1 = *(cptr as *mut u16) as u32;
                            if __c1 >= 0xdc00 as i32 as u32 && __c1 < 0xe000 as i32 as u32 {
                                c = ((c & 0x3ff as i32 as u32) << 10 as i32
                                    | __c1 & 0x3ff as i32 as u32)
                                    .wrapping_add(0x10000 as i32 as u32);
                                cptr = cptr.offset(2 as i32 as isize)
                            }
                        }
                    }
                    if (*s).ignore_case != 0 {
                        c = lre_canonicalize(c, (*s).is_utf16)
                    }
                    if !(val != c) {
                        continue;
                    }
                    current_block = 14487425527653873875;
                }
            }
            _ => {}
        }
        match current_block {
            14487425527653873875 => {
                if no_recurse != 0 {
                    return 0 as i32 as intptr_t;
                }
                ret = 0 as i32
            }
            _ => {}
        }
        let mut current_block_49: u64;
        loop {
            if (*s).state_stack_len == 0 {
                return ret as intptr_t;
            }
            rs = (*s).state_stack.offset(
                (*s).state_stack_len
                    .wrapping_sub(1)
                    .wrapping_mul((*s).state_size) as isize,
            ) as *mut REExecState;
            if (*rs).type_0() as i32 == RE_EXEC_STATE_SPLIT as i32 {
                if ret == 0 {
                    current_block_49 = 2041432150095197404;
                } else {
                    current_block_49 = 17075014677070940716;
                }
            } else if (*rs).type_0() as i32 == RE_EXEC_STATE_GREEDY_QUANT as i32 {
                if ret == 0 {
                    let mut char_count: u32 = 0;
                    let mut i: u32 = 0;

                    (capture as *mut u8).copy_from(
                        (*rs).buf.as_mut_ptr() as *const u8,
                        (std::mem::size_of::<*mut u8>())
                            .wrapping_mul(2)
                            .wrapping_mul((*s).capture_count as usize),
                    );
                    stack_len = (*rs).stack_len as i32;
                    (stack as *mut u8).copy_from(
                        (*rs)
                            .buf
                            .as_mut_ptr()
                            .offset((2 * (*s).capture_count as isize))
                            as *const u8,
                        (stack_len as usize).wrapping_mul(std::mem::size_of::<StackInt>()),
                    );
                    pc = (*rs).pc;
                    cptr = (*rs).cptr;
                    /* go backward */
                    char_count = get_u32(pc.offset(12 as i32 as isize));
                    i = 0 as i32 as u32;
                    while i < char_count {
                        if cbuf_type == 0 as i32 {
                            cptr = cptr.offset(-1)
                        } else {
                            cptr = cptr.offset(-(2 as i32 as isize));
                            if cbuf_type == 2 as i32 {
                                c = *(cptr as *mut u16).offset(0 as i32 as isize) as u32;
                                if c >= 0xdc00 as i32 as u32
                                    && c < 0xe000 as i32 as u32
                                    && cptr > (*s).cbuf
                                {
                                    c = *(cptr as *mut u16).offset(-(1 as i32) as isize) as u32;
                                    if c >= 0xd800 as i32 as u32 && c < 0xdc00 as i32 as u32 {
                                        cptr = cptr.offset(-(2 as i32 as isize))
                                    }
                                }
                            }
                        }
                        i = i.wrapping_add(1)
                    }
                    pc = pc
                        .offset(16 as i32 as isize)
                        .offset(get_u32(pc) as i32 as isize);
                    (*rs).cptr = cptr;
                    (*rs).count = (*rs).count.wrapping_sub(1);
                    if (*rs).count == 0 {
                        (*s).state_stack_len = (*s).state_stack_len.wrapping_sub(1)
                    }
                    break;
                } else {
                    current_block_49 = 17075014677070940716;
                }
            } else {
                ret = ((*rs).type_0() as i32 == RE_EXEC_STATE_LOOKAHEAD as i32 && ret != 0
                    || (*rs).type_0() as i32 == RE_EXEC_STATE_NEGATIVE_LOOKAHEAD as i32 && ret == 0)
                    as i32;
                if ret != 0 {
                    /* keep the capture in case of positive lookahead */
                    if (*rs).type_0() as i32 == RE_EXEC_STATE_LOOKAHEAD as i32 {
                        current_block_49 = 7340255856720145317;
                    } else {
                        current_block_49 = 2041432150095197404;
                    }
                } else {
                    current_block_49 = 17075014677070940716;
                }
            }
            match current_block_49 {
                17075014677070940716 => {
                    (*s).state_stack_len = (*s).state_stack_len.wrapping_sub(1);
                    continue;
                }
                2041432150095197404 => {
                    (capture as *mut u8).copy_from(
                        (*rs).buf.as_mut_ptr() as *const u8,
                        (std::mem::size_of::<*mut u8>())
                            .wrapping_mul(2)
                            .wrapping_mul((*s).capture_count as usize),
                    );
                }
                _ => {}
            }
            pc = (*rs).pc;
            cptr = (*rs).cptr;
            stack_len = (*rs).stack_len as i32;
            (stack as *mut u8).copy_from(
                (*rs)
                    .buf
                    .as_mut_ptr()
                    .offset((2 * (*s).capture_count as isize)) as *const u64
                    as *const u8,
                (stack_len as usize).wrapping_mul(std::mem::size_of::<StackInt>()),
            );
            (*s).state_stack_len = (*s).state_stack_len.wrapping_sub(1);
            break;
        }
    }
}
/* Return 1 if match, 0 if not match or -1 if error. cindex is the
starting position of the match and must be such as 0 <= cindex <=
clen. */
#[no_mangle]
pub unsafe fn lre_exec(
    mut capture: *mut *mut u8,
    mut bc_buf: *const u8,
    mut cbuf: *const u8,
    mut cindex: i32,
    mut clen: i32,
    mut cbuf_type: i32,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    let mut s_s: REExecContext = REExecContext {
        cbuf: 0 as *const u8,
        cbuf_end: 0 as *const u8,
        cbuf_type: 0,
        capture_count: 0,
        stack_size_max: 0,
        multi_line: 0,
        ignore_case: 0,
        is_utf16: 0,
        opaque: 0 as *mut std::ffi::c_void,
        state_size: 0,
        state_stack: 0 as *mut u8,
        state_stack_size: 0,
        state_stack_len: 0,
    };
    let mut s: *mut REExecContext = &mut s_s;
    let mut re_flags: i32 = 0;
    let mut i: i32 = 0;
    let mut alloca_size: i32 = 0;
    let mut ret: i32 = 0;
    let mut stack_buf: *mut StackInt = 0 as *mut StackInt;
    re_flags = *bc_buf.offset(0 as i32 as isize) as i32;
    (*s).multi_line = (re_flags & (1 as i32) << 2 as i32 != 0 as i32) as i32;
    (*s).ignore_case = (re_flags & (1 as i32) << 1 as i32 != 0 as i32) as i32;
    (*s).is_utf16 = (re_flags & (1 as i32) << 4 as i32 != 0 as i32) as i32;
    (*s).capture_count = *bc_buf.offset(1 as i32 as isize) as i32;
    (*s).stack_size_max = *bc_buf.offset(2 as i32 as isize) as i32;
    (*s).cbuf = cbuf;
    (*s).cbuf_end = cbuf.offset((clen << cbuf_type) as isize);
    (*s).cbuf_type = cbuf_type;
    if (*s).cbuf_type == 1 as i32 && (*s).is_utf16 != 0 {
        (*s).cbuf_type = 2 as i32
    }
    (*s).opaque = opaque;
    (*s).state_size = (::std::mem::size_of::<REExecState>())
        .wrapping_add(
            ((*s).capture_count as usize)
                .wrapping_mul(::std::mem::size_of::<*mut u8>())
                .wrapping_mul(2),
        )
        .wrapping_add(
            ((*s).stack_size_max as usize).wrapping_mul(::std::mem::size_of::<StackInt>()),
        );
    (*s).state_stack = 0 as *mut u8;
    (*s).state_stack_len = 0 as i32 as usize;
    (*s).state_stack_size = 0 as i32 as usize;
    i = 0 as i32;
    while i < (*s).capture_count * 2 as i32 {
        let ref mut fresh44 = *capture.offset(i as isize);
        *fresh44 = 0 as *mut u8;
        i += 1
    }
    alloca_size =
        ((*s).stack_size_max as u64).wrapping_mul(::std::mem::size_of::<StackInt>() as u64) as i32;
    let mut fresh45 = ::std::vec::from_elem(0, alloca_size as u64 as usize);
    stack_buf = fresh45.as_mut_ptr() as *mut StackInt;
    ret = lre_exec_backtrack(
        s,
        capture,
        stack_buf,
        0 as i32,
        bc_buf.offset(7 as i32 as isize),
        cbuf.offset((cindex << cbuf_type) as isize),
        FALSE as i32,
    ) as i32;
    lre_realloc(
        (*s).opaque,
        (*s).state_stack as *mut std::ffi::c_void,
        0 as i32 as usize,
    );
    return ret;
}
#[no_mangle]
pub unsafe fn lre_get_capture_count(mut bc_buf: *const u8) -> i32 {
    return *bc_buf.offset(1 as i32 as isize) as i32;
}
#[no_mangle]
pub unsafe fn lre_get_flags(mut bc_buf: *const u8) -> i32 {
    return *bc_buf.offset(0 as i32 as isize) as i32;
}
/* Return NULL if no group names. Otherwise, return a pointer to
'capture_count - 1' zero terminated UTF-8 strings. */
#[no_mangle]
pub unsafe fn lre_get_groupnames(mut bc_buf: *const u8) -> *const std::os::raw::c_char {
    let mut re_bytecode_len: u32 = 0;
    if lre_get_flags(bc_buf) & (1 as i32) << 7 as i32 == 0 as i32 {
        return 0 as *const std::os::raw::c_char;
    }
    re_bytecode_len = get_u32(bc_buf.offset(3 as i32 as isize));
    return bc_buf
        .offset(7 as i32 as isize)
        .offset(re_bytecode_len as isize) as *const std::os::raw::c_char;
}
