use crate::quickjs_libc::FILE;
use std::process::abort;

use crate::cutils::{
    dbuf_error, dbuf_init2, dbuf_printf, dbuf_put, dbuf_putc, dbuf_putstr, dbuf_set_error, DynBuf,
    DynBufReallocFunc,
};

pub type size_t = u64;
pub type __int8_t = std::os::raw::c_char;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type int8_t = __int8_t;
pub type int64_t = __int64_t;

pub type _IO_lock_t = ();
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = i64;
pub type BOOL = i32;
pub type C2RustUnnamed = u32;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;

pub type uint128_t = u128;
pub type slimb_t = int64_t;
pub type limb_t = uint64_t;
pub type dlimb_t = uint128_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bf_t {
    pub ctx: *mut bf_context_t,
    pub sign: i32,
    pub expn: slimb_t,
    pub len: limb_t,
    pub tab: *mut limb_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bf_context_t {
    pub realloc_opaque: *mut std::ffi::c_void,
    pub realloc_func: Option<bf_realloc_func_t>,
    pub log2_cache: BFConstCache,
    pub pi_cache: BFConstCache,
    pub ntt_state: *mut BFNTTState,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BFNTTState {
    pub ctx: *mut bf_context_t,
    pub ntt_mods_div: [limb_t; 5],
    pub ntt_proot_pow: [[[limb_t; 52]; 2]; 5],
    pub ntt_proot_pow_inv: [[[limb_t; 52]; 2]; 5],
    pub ntt_trig: [[[*mut NTTLimb; 20]; 2]; 5],
    pub ntt_len_inv: [[[limb_t; 2]; 52]; 5],
    pub ntt_mods_cr_inv: [limb_t; 10],
}
pub type NTTLimb = limb_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BFConstCache {
    pub val: bf_t,
    pub prec: limb_t,
}
pub type bf_realloc_func_t = unsafe extern "C" fn(
    _: *mut std::ffi::c_void,
    _: *mut std::ffi::c_void,
    _: size_t,
) -> *mut std::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfdec_t {
    pub ctx: *mut bf_context_t,
    pub sign: i32,
    pub expn: slimb_t,
    pub len: limb_t,
    pub tab: *mut limb_t,
}

pub type bf_rnd_t = u32;

pub const BF_RNDF: bf_rnd_t = 6;
pub const BF_RNDA: bf_rnd_t = 5;
pub const BF_RNDNA: bf_rnd_t = 4;
pub const BF_RNDU: bf_rnd_t = 3;
pub const BF_RNDD: bf_rnd_t = 2;
pub const BF_RNDZ: bf_rnd_t = 1;
pub const BF_RNDN: bf_rnd_t = 0;
pub type bf_flags_t = uint32_t;

const LIMB_LOG2_BITS: u64 = 6;
const LIMB_BITS: u64 = 1 << LIMB_LOG2_BITS;

const BF_RAW_EXP_MIN: i64 = std::i64::MIN;
const BF_RAW_EXP_MAX: i64 = std::i64::MAX;

const LIMB_DIGITS: u64 = 19;
const BF_DEC_BASE: u64 = 10_000_000_000_000_000_000;

/* in bits */
/* minimum number of bits for the exponent */
const BF_EXP_BITS_MIN: u64 = 3;
/* maximum number of bits for the exponent */
const BF_EXP_BITS_MAX: u64 = (LIMB_BITS - 3);
/* extended range for exponent, used internally */
const BF_EXT_EXP_BITS_MAX: u64 = (BF_EXP_BITS_MAX + 1);
/* minimum possible precision */
const BF_PREC_MIN: u64 = 2;
/* minimum possible precision */
const BF_PREC_MAX: u64 = (1 << (LIMB_BITS - 2)) - 2;
/* some operations support infinite precision */
/* infinite precision */
const BF_PREC_INF: u64 = (BF_PREC_MAX + 1);

const BF_CHKSUM_MOD: u64 = 975620677 * 9795002197;

const BF_EXP_ZERO: i64 = BF_RAW_EXP_MIN;
const BF_EXP_INF: i64 = (BF_RAW_EXP_MAX - 1);
const BF_EXP_NAN: i64 = BF_RAW_EXP_MAX;

const NTT_MOD_LOG2_MIN: u64 = 50;
const NTT_MOD_LOG2_MAX: u64 = 51;
const NB_MODS: u64 = 5;
const NTT_PROOT_2EXP: u64 = 39;

#[repr(C)]
#[derive(Copy, Clone)]
pub union Float64Union {
    pub d: f64,
    pub u: uint64_t,
}
pub type bf_op2_func_t = unsafe extern "C" fn(
    _: *mut bf_t,
    _: *const bf_t,
    _: *const bf_t,
    _: limb_t,
    _: bf_flags_t,
) -> i32;
pub type mp_size_t = intptr_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FastDivData {
    pub m1: limb_t,
    pub shift1: int8_t,
    pub shift2: int8_t,
}
/* ZivFunc should compute the result 'r' with faithful rounding at
precision 'prec'. For efficiency purposes, the final bf_round()
does not need to be done in the function. */
pub type ZivFunc =
    unsafe extern "C" fn(_: *mut bf_t, _: *const bf_t, _: limb_t, _: *mut std::ffi::c_void) -> i32;

#[inline]
unsafe extern "C" fn clz64(mut a: uint64_t) -> i32 {
    return (a as u64).leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn ctz64(mut a: uint64_t) -> i32 {
    return (a as u64).trailing_zeros() as i32;
}

#[inline]
pub unsafe extern "C" fn bf_get_exp_bits(mut flags: bf_flags_t) -> i32 {
    let mut e: i32 = 0;
    e = (flags >> 5 as i32 & 0x3f as i32 as u32) as i32;
    if e == 0x3f as i32 {
        return ((1 as i32) << 6 as i32) - 3 as i32 + 1 as i32;
    } else {
        return ((1 as i32) << 6 as i32) - 3 as i32 - e;
    };
}
#[inline]
pub unsafe extern "C" fn bf_set_exp_bits(mut n: i32) -> bf_flags_t {
    return ((((1 as i32) << 6 as i32) - 3 as i32 - n & 0x3f as i32) << 5 as i32) as bf_flags_t;
}
#[inline]
unsafe extern "C" fn bf_max(mut a: slimb_t, mut b: slimb_t) -> slimb_t {
    if a > b {
        return a;
    } else {
        return b;
    };
}
#[inline]
unsafe extern "C" fn bf_min(mut a: slimb_t, mut b: slimb_t) -> slimb_t {
    if a < b {
        return a;
    } else {
        return b;
    };
}
#[inline]
pub unsafe extern "C" fn bf_realloc(
    mut s: *mut bf_context_t,
    mut ptr: *mut std::ffi::c_void,
    mut size: size_t,
) -> *mut std::ffi::c_void {
    return (*s).realloc_func.expect("non-null function pointer")((*s).realloc_opaque, ptr, size);
}
#[inline]
unsafe extern "C" fn bf_malloc(
    mut s: *mut bf_context_t,
    mut size: size_t,
) -> *mut std::ffi::c_void {
    return bf_realloc(s, 0 as *mut std::ffi::c_void, size);
}
#[inline]
pub unsafe extern "C" fn bf_free(mut s: *mut bf_context_t, mut ptr: *mut std::ffi::c_void) {
    if !ptr.is_null() {
        bf_realloc(s, ptr, 0 as i32 as size_t);
    };
}
#[inline]
pub unsafe extern "C" fn bf_delete(mut r: *mut bf_t) {
    let mut s: *mut bf_context_t = (*r).ctx;
    if !s.is_null() && !(*r).tab.is_null() {
        bf_realloc(s, (*r).tab as *mut std::ffi::c_void, 0 as i32 as size_t);
    };
}
#[inline]
pub unsafe extern "C" fn bf_neg(mut r: *mut bf_t) {
    (*r).sign ^= 1 as i32;
}
#[inline]
pub unsafe extern "C" fn bf_is_finite(mut a: *const bf_t) -> i32 {
    return ((*a).expn < 9223372036854775807 as i64 - 1 as i32 as i64) as i32;
}

#[inline]
pub unsafe extern "C" fn bf_is_zero(mut a: *const bf_t) -> i32 {
    return ((*a).expn == -(9223372036854775807 as i64) - 1 as i32 as i64) as i32;
}

#[inline]
pub unsafe extern "C" fn bf_is_nan(mut a: *const bf_t) -> i32 {
    return ((*a).expn == 9223372036854775807 as i64) as i32;
}
#[inline]
pub unsafe extern "C" fn bf_cmp_eq(mut a: *const bf_t, mut b: *const bf_t) -> i32 {
    return (bf_cmp(a, b) == 0 as i32) as i32;
}

#[inline]
pub unsafe extern "C" fn bf_cmp_le(mut a: *const bf_t, mut b: *const bf_t) -> i32 {
    return (bf_cmp(a, b) <= 0 as i32) as i32;
}

#[inline]
pub unsafe extern "C" fn bf_cmp_lt(mut a: *const bf_t, mut b: *const bf_t) -> i32 {
    return (bf_cmp(a, b) < 0 as i32) as i32;
}
#[inline]
pub unsafe extern "C" fn bfdec_init(mut s: *mut bf_context_t, mut r: *mut bfdec_t) {
    bf_init(s, r as *mut bf_t);
}

#[inline]
pub unsafe extern "C" fn bfdec_cmp_le(mut a: *const bfdec_t, mut b: *const bfdec_t) -> i32 {
    return (bfdec_cmp(a, b) <= 0 as i32) as i32;
}

#[inline]
pub unsafe extern "C" fn bfdec_cmp_eq(mut a: *const bfdec_t, mut b: *const bfdec_t) -> i32 {
    return (bfdec_cmp(a, b) == 0 as i32) as i32;
}

#[inline]
pub unsafe extern "C" fn bfdec_cmp_lt(mut a: *const bfdec_t, mut b: *const bfdec_t) -> i32 {
    return (bfdec_cmp(a, b) < 0 as i32) as i32;
}

#[inline]
pub unsafe extern "C" fn bfdec_neg(mut r: *mut bfdec_t) {
    (*r).sign ^= 1 as i32;
}
#[inline]
pub unsafe extern "C" fn bfdec_delete(mut r: *mut bfdec_t) {
    bf_delete(r as *mut bf_t);
}
#[inline]
pub unsafe extern "C" fn bfdec_is_nan(mut a: *const bfdec_t) -> i32 {
    return ((*a).expn == 9223372036854775807 as i64) as i32;
}
#[inline]
pub unsafe extern "C" fn bfdec_set_nan(mut r: *mut bfdec_t) {
    bf_set_nan(r as *mut bf_t);
}
#[inline]
pub unsafe extern "C" fn bfdec_set_zero(mut r: *mut bfdec_t, mut is_neg: i32) {
    bf_set_zero(r as *mut bf_t, is_neg);
}
#[inline]
pub unsafe extern "C" fn bfdec_set_inf(mut r: *mut bfdec_t, mut is_neg: i32) {
    bf_set_inf(r as *mut bf_t, is_neg);
}
#[inline]
pub unsafe extern "C" fn bfdec_set(mut r: *mut bfdec_t, mut a: *const bfdec_t) -> i32 {
    return bf_set(r as *mut bf_t, a as *mut bf_t);
}
#[inline]
pub unsafe extern "C" fn bfdec_move(mut r: *mut bfdec_t, mut a: *mut bfdec_t) {
    bf_move(r as *mut bf_t, a as *mut bf_t);
}
#[inline]
pub unsafe extern "C" fn bfdec_cmpu(mut a: *const bfdec_t, mut b: *const bfdec_t) -> i32 {
    return bf_cmpu(a as *const bf_t, b as *const bf_t);
}

#[inline]
pub unsafe extern "C" fn bfdec_cmp(mut a: *const bfdec_t, mut b: *const bfdec_t) -> i32 {
    return bf_cmp(a as *const bf_t, b as *const bf_t);
}
#[inline]
unsafe extern "C" fn bfdec_resize(mut r: *mut bfdec_t, mut len: limb_t) -> i32 {
    return bf_resize(r as *mut bf_t, len);
}
/* could leading zeros */
#[inline]
unsafe extern "C" fn clz(mut a: limb_t) -> i32 {
    if a == 0 as i32 as u64 {
        return (1 as i32) << 6 as i32;
    } else {
        return clz64(a);
    };
}
#[inline]
unsafe extern "C" fn ctz(mut a: limb_t) -> i32 {
    if a == 0 as i32 as u64 {
        return (1 as i32) << 6 as i32;
    } else {
        return ctz64(a);
    };
}
#[inline]
unsafe extern "C" fn ceil_log2(mut a: limb_t) -> i32 {
    if a <= 1 as i32 as u64 {
        return 0 as i32;
    } else {
        return ((1 as i32) << 6 as i32) - clz(a.wrapping_sub(1 as i32 as u64));
    };
}
/* b must be >= 1 */
#[inline]
unsafe extern "C" fn ceil_div(mut a: slimb_t, mut b: slimb_t) -> slimb_t {
    if a >= 0 as i32 as i64 {
        return (a + b - 1 as i32 as i64) / b;
    } else {
        return a / b;
    };
}
/* b must be >= 1 */
#[inline]
unsafe extern "C" fn floor_div(mut a: slimb_t, mut b: slimb_t) -> slimb_t {
    if a >= 0 as i32 as i64 {
        return a / b;
    } else {
        return (a - b + 1 as i32 as i64) / b;
    };
}
/* return r = a modulo b (0 <= r <= b - 1. b must be >= 1 */
#[inline]
unsafe extern "C" fn smod(mut a: slimb_t, mut b: slimb_t) -> limb_t {
    a = a % b;
    if a < 0 as i32 as i64 {
        a += b
    }
    return a as limb_t;
}
/* signed addition with saturation */
#[inline]
unsafe extern "C" fn sat_add(mut a: slimb_t, mut b: slimb_t) -> slimb_t {
    let mut r: slimb_t = 0;
    r = a + b;
    /* overflow ? */
    if (a ^ r) & (b ^ r) < 0 as i32 as i64 {
        r = ((a >> ((1 as i32) << 6 as i32) - 1 as i32) as u64
            ^ ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 1 as i32)
                .wrapping_sub(1 as i32 as u64)) as slimb_t
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn bf_context_init(
    mut s: *mut bf_context_t,
    mut realloc_func: Option<bf_realloc_func_t>,
    mut realloc_opaque: *mut std::ffi::c_void,
) {
    (s as *mut u8).write_bytes(0, std::mem::size_of::<bf_context_t>());
    (*s).realloc_func = realloc_func;
    (*s).realloc_opaque = realloc_opaque;
}
#[no_mangle]
pub unsafe extern "C" fn bf_context_end(mut s: *mut bf_context_t) {
    bf_clear_cache(s);
}
#[no_mangle]
pub unsafe extern "C" fn bf_init(mut s: *mut bf_context_t, mut r: *mut bf_t) {
    (*r).ctx = s;
    (*r).sign = 0 as i32;
    (*r).expn = -(9223372036854775807 as i64) - 1 as i32 as i64;
    (*r).len = 0 as i32 as limb_t;
    (*r).tab = 0 as *mut limb_t;
}
/* return 0 if OK, -1 if alloc error */
#[no_mangle]
pub unsafe extern "C" fn bf_resize(mut r: *mut bf_t, mut len: limb_t) -> i32 {
    let mut tab: *mut limb_t = 0 as *mut limb_t;
    if len != (*r).len {
        tab = bf_realloc(
            (*r).ctx,
            (*r).tab as *mut std::ffi::c_void,
            len.wrapping_mul(::std::mem::size_of::<limb_t>() as u64),
        ) as *mut limb_t;
        if tab.is_null() && len != 0 as i32 as u64 {
            return -(1 as i32);
        }
        (*r).tab = tab;
        (*r).len = len
    }
    return 0 as i32;
}
/* return 0 or BF_ST_MEM_ERROR */
#[no_mangle]
pub unsafe extern "C" fn bf_set_ui(mut r: *mut bf_t, mut a: uint64_t) -> i32 {
    (*r).sign = 0 as i32;
    if a == 0 as i32 as u64 {
        (*r).expn = -(9223372036854775807 as i64) - 1 as i32 as i64;
        bf_resize(r, 0 as i32 as limb_t);
    /* cannot fail */
    } else {
        let mut shift: i32 = 0;
        if bf_resize(r, 1 as i32 as limb_t) != 0 {
            bf_set_nan(r);
            return (1 as i32) << 5 as i32;
        } else {
            shift = clz(a);
            *(*r).tab.offset(0 as i32 as isize) = a << shift;
            (*r).expn = (((1 as i32) << 6 as i32) - shift) as slimb_t
        }
    }
    return 0 as i32;
}
/* return 0 or BF_ST_MEM_ERROR */
#[no_mangle]
pub unsafe extern "C" fn bf_set_si(mut r: *mut bf_t, mut a: int64_t) -> i32 {
    let mut ret: i32 = 0; /* cannot fail */
    if a < 0 as i32 as i64 {
        ret = bf_set_ui(r, -a as uint64_t); /* cannot fail */
        (*r).sign = 1 as i32
    } else {
        ret = bf_set_ui(r, a as uint64_t)
    } /* cannot fail */
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bf_set_nan(mut r: *mut bf_t) {
    bf_resize(r, 0 as i32 as limb_t);
    (*r).expn = 9223372036854775807 as i64;
    (*r).sign = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bf_set_zero(mut r: *mut bf_t, mut is_neg: i32) {
    bf_resize(r, 0 as i32 as limb_t);
    (*r).expn = -(9223372036854775807 as i64) - 1 as i32 as i64;
    (*r).sign = is_neg;
}
#[no_mangle]
pub unsafe extern "C" fn bf_set_inf(mut r: *mut bf_t, mut is_neg: i32) {
    bf_resize(r, 0 as i32 as limb_t);
    (*r).expn = 9223372036854775807 as i64 - 1 as i32 as i64;
    (*r).sign = is_neg;
}
/* return 0 or BF_ST_MEM_ERROR */
#[no_mangle]
pub unsafe extern "C" fn bf_set(mut r: *mut bf_t, mut a: *const bf_t) -> i32 {
    if r == a as *mut bf_t {
        return 0 as i32;
    }
    if bf_resize(r, (*a).len) != 0 {
        bf_set_nan(r);
        return (1 as i32) << 5 as i32;
    }
    (*r).sign = (*a).sign;
    (*r).expn = (*a).expn;
    ((*r).tab as *mut u8).copy_from(
        (*a).tab as *const u8,
        ((*a).len as usize).wrapping_mul(std::mem::size_of::<limb_t>()),
    );
    return 0 as i32;
}
/* equivalent to bf_set(r, a); bf_delete(a) */
#[no_mangle]
pub unsafe extern "C" fn bf_move(mut r: *mut bf_t, mut a: *mut bf_t) {
    let mut s: *mut bf_context_t = (*r).ctx;
    if r == a {
        return;
    }
    bf_free(s, (*r).tab as *mut std::ffi::c_void);
    *r = *a;
}
unsafe extern "C" fn get_limbz(mut a: *const bf_t, mut idx: limb_t) -> limb_t {
    if idx >= (*a).len {
        return 0 as i32 as limb_t;
    } else {
        return *(*a).tab.offset(idx as isize);
    };
}
/* get LIMB_BITS at bit position 'pos' in tab */
#[inline]
unsafe extern "C" fn get_bits(mut tab: *const limb_t, mut len: limb_t, mut pos: slimb_t) -> limb_t {
    let mut i: limb_t = 0;
    let mut a0: limb_t = 0;
    let mut a1: limb_t = 0;
    let mut p: i32 = 0;
    i = (pos >> 6 as i32) as limb_t;
    p = (pos & (((1 as i32) << 6 as i32) - 1 as i32) as i64) as i32;
    if i < len {
        a0 = *tab.offset(i as isize)
    } else {
        a0 = 0 as i32 as limb_t
    }
    if p == 0 as i32 {
        return a0;
    } else {
        i = i.wrapping_add(1);
        if i < len {
            a1 = *tab.offset(i as isize)
        } else {
            a1 = 0 as i32 as limb_t
        }
        return a0 >> p | a1 << ((1 as i32) << 6 as i32) - p;
    };
}
#[inline]
unsafe extern "C" fn get_bit(mut tab: *const limb_t, mut len: limb_t, mut pos: slimb_t) -> limb_t {
    let mut i: slimb_t = 0;
    i = pos >> 6 as i32;
    if i < 0 as i32 as i64 || i as u64 >= len {
        return 0 as i32 as limb_t;
    }
    return *tab.offset(i as isize) >> (pos & (((1 as i32) << 6 as i32) - 1 as i32) as i64)
        & 1 as i32 as u64;
}
#[inline]
unsafe extern "C" fn limb_mask(mut start: i32, mut last: i32) -> limb_t {
    let mut v: limb_t = 0;
    let mut n: i32 = 0;
    n = last - start + 1 as i32;
    if n == (1 as i32) << 6 as i32 {
        v = -(1 as i32) as limb_t
    } else {
        v = ((1 as i32 as limb_t) << n).wrapping_sub(1 as i32 as u64) << start
    }
    return v;
}
unsafe extern "C" fn mp_scan_nz(mut tab: *const limb_t, mut n: mp_size_t) -> limb_t {
    let mut i: mp_size_t = 0;
    i = 0 as i32 as mp_size_t;
    while i < n {
        if *tab.offset(i as isize) != 0 as i32 as u64 {
            return 1 as i32 as limb_t;
        }
        i += 1
    }
    return 0 as i32 as limb_t;
}
/* return != 0 if one bit between 0 and bit_pos inclusive is not zero. */
#[inline]
unsafe extern "C" fn scan_bit_nz(mut r: *const bf_t, mut bit_pos: slimb_t) -> limb_t {
    let mut pos: slimb_t = 0;
    let mut v: limb_t = 0;
    pos = bit_pos >> 6 as i32;
    if pos < 0 as i32 as i64 {
        return 0 as i32 as limb_t;
    }
    v = *(*r).tab.offset(pos as isize)
        & limb_mask(
            0 as i32,
            (bit_pos & (((1 as i32) << 6 as i32) - 1 as i32) as i64) as i32,
        );
    if v != 0 as i32 as u64 {
        return 1 as i32 as limb_t;
    }
    pos -= 1;
    while pos >= 0 as i32 as i64 {
        if *(*r).tab.offset(pos as isize) != 0 as i32 as u64 {
            return 1 as i32 as limb_t;
        }
        pos -= 1
    }
    return 0 as i32 as limb_t;
}
/* return the addend for rounding. Note that prec can be <= 0 (for
BF_FLAG_RADPNT_PREC) */
unsafe extern "C" fn bf_get_rnd_add(
    mut pret: *mut i32,
    mut r: *const bf_t,
    mut l: limb_t,
    mut prec: slimb_t,
    mut rnd_mode: i32,
) -> i32 {
    let mut add_one: i32 = 0;
    let mut inexact: i32 = 0;
    let mut bit1: limb_t = 0;
    let mut bit0: limb_t = 0;
    if rnd_mode == BF_RNDF as i32 {
        bit0 = 1 as i32 as limb_t
    /* faithful rounding does not honor the INEXACT flag */
    } else {
        /* starting limb for bit 'prec + 1' */
        bit0 = scan_bit_nz(
            r,
            l.wrapping_mul(((1 as i32) << 6 as i32) as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_sub(bf_max(0 as i32 as slimb_t, prec + 1 as i32 as i64) as u64)
                as slimb_t,
        )
    }
    /* get the bit at 'prec' */
    bit1 = get_bit(
        (*r).tab,
        l,
        l.wrapping_mul(((1 as i32) << 6 as i32) as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_sub(prec as u64) as slimb_t,
    );
    inexact = (bit1 | bit0 != 0 as i32 as u64) as i32;
    add_one = 0 as i32;
    match rnd_mode {
        1 => {}
        0 => {
            if bit1 != 0 {
                if bit0 != 0 {
                    add_one = 1 as i32
                } else {
                    /* round to even */
                    add_one = get_bit(
                        (*r).tab,
                        l,
                        l.wrapping_mul(((1 as i32) << 6 as i32) as u64)
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_sub((prec - 1 as i32 as i64) as u64)
                            as slimb_t,
                    ) as i32
                }
            }
        }
        2 | 3 => {
            if (*r).sign == (rnd_mode == BF_RNDD as i32) as i32 {
                add_one = inexact
            }
        }
        5 => add_one = inexact,
        4 | 6 => add_one = bit1 as i32,
        _ => {
            abort();
        }
    }
    if inexact != 0 {
        *pret |= (1 as i32) << 4 as i32
    }
    return add_one;
}
unsafe extern "C" fn bf_set_overflow(
    mut r: *mut bf_t,
    mut sign: i32,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut i: slimb_t = 0;
    let mut l: slimb_t = 0;
    let mut e_max: slimb_t = 0;
    let mut rnd_mode: i32 = 0;
    rnd_mode = (flags & 0x7 as i32 as u32) as i32;
    if prec
        == ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64)
        || rnd_mode == BF_RNDN as i32
        || rnd_mode == BF_RNDNA as i32
        || rnd_mode == BF_RNDA as i32
        || rnd_mode == BF_RNDD as i32 && sign == 1 as i32
        || rnd_mode == BF_RNDU as i32 && sign == 0 as i32
    {
        bf_set_inf(r, sign);
    } else {
        /* set to maximum finite number */
        l = prec
            .wrapping_add(((1 as i32) << 6 as i32) as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(((1 as i32) << 6 as i32) as u64) as slimb_t;
        if bf_resize(r, l as limb_t) != 0 {
            bf_set_nan(r);
            return (1 as i32) << 5 as i32;
        }
        *(*r).tab.offset(0 as i32 as isize) = limb_mask(
            (prec.wrapping_neg() & (((1 as i32) << 6 as i32) - 1 as i32) as u64) as i32,
            ((1 as i32) << 6 as i32) - 1 as i32,
        );
        i = 1 as i32 as slimb_t;
        while i < l {
            *(*r).tab.offset(i as isize) = -(1 as i32) as limb_t;
            i += 1
        }
        e_max = ((1 as i32 as limb_t) << bf_get_exp_bits(flags) - 1 as i32) as slimb_t;
        (*r).expn = e_max;
        (*r).sign = sign
    }
    return (1 as i32) << 2 as i32 | (1 as i32) << 4 as i32;
}
/* round to prec1 bits assuming 'r' is non zero and finite. 'r' is
assumed to have length 'l' (1 <= l <= r->len). Note: 'prec1' can be
infinite (BF_PREC_INF). 'ret' is 0 or BF_ST_INEXACT if the result
is known to be inexact. Can fail with BF_ST_MEM_ERROR in case of
overflow not returning infinity. */
unsafe extern "C" fn __bf_round(
    mut r: *mut bf_t,
    mut prec1: limb_t,
    mut flags: bf_flags_t,
    mut l: limb_t,
    mut ret: i32,
) -> i32 {
    let mut current_block: u64;
    let mut v: limb_t = 0;
    let mut a: limb_t = 0;
    let mut shift: i32 = 0;
    let mut add_one: i32 = 0;
    let mut rnd_mode: i32 = 0;
    let mut i: slimb_t = 0;
    let mut bit_pos: slimb_t = 0;
    let mut pos: slimb_t = 0;
    let mut e_min: slimb_t = 0;
    let mut e_max: slimb_t = 0;
    let mut e_range: slimb_t = 0;
    let mut prec: slimb_t = 0;
    /* e_min and e_max are computed to match the IEEE 754 conventions */
    e_range = ((1 as i32 as limb_t) << bf_get_exp_bits(flags) - 1 as i32) as slimb_t;
    e_min = -e_range + 3 as i32 as i64;
    e_max = e_range;
    if flags & ((1 as i32) << 4 as i32) as u32 != 0 {
        /* 'prec' is the precision after the radix point */
        if prec1
            != ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
        {
            prec = ((*r).expn as u64).wrapping_add(prec1) as slimb_t
        } else {
            prec = prec1 as slimb_t
        }
    } else if ((*r).expn < e_min) as i32 as i64 != 0 && flags & ((1 as i32) << 3 as i32) as u32 != 0
    {
        /* restrict the precision in case of potentially subnormal
        result */
        if prec1
            != ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
        {
        } else {
            assert!(prec1 != BF_PREC_INF);
        }
        prec = prec1.wrapping_sub((e_min - (*r).expn) as u64) as slimb_t
    } else {
        prec = prec1 as slimb_t
    }
    /* round to prec bits */
    rnd_mode = (flags & 0x7 as i32 as u32) as i32; /* cannot fail */
    add_one = bf_get_rnd_add(&mut ret, r, l, prec, rnd_mode);
    if prec <= 0 as i32 as i64 {
        if add_one != 0 {
            bf_resize(r, 1 as i32 as limb_t);
            *(*r).tab.offset(0 as i32 as isize) =
                (1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 1 as i32;
            (*r).expn += 1 as i32 as i64 - prec;
            ret |= (1 as i32) << 3 as i32 | (1 as i32) << 4 as i32;
            return ret;
        }
    } else {
        if add_one != 0 {
            let mut carry: limb_t = 0;
            /* add one starting at digit 'prec - 1' */
            bit_pos = l
                .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_sub((prec - 1 as i32 as i64) as u64) as slimb_t;
            pos = bit_pos >> 6 as i32;
            carry =
                (1 as i32 as limb_t) << (bit_pos & (((1 as i32) << 6 as i32) - 1 as i32) as i64);
            i = pos;
            while (i as u64) < l {
                v = (*(*r).tab.offset(i as isize)).wrapping_add(carry);
                carry = (v < carry) as i32 as limb_t;
                *(*r).tab.offset(i as isize) = v;
                if carry == 0 as i32 as u64 {
                    break;
                }
                i += 1
            }
            if carry != 0 {
                /* shift right by one digit */
                v = 1 as i32 as limb_t;
                i = l.wrapping_sub(1 as i32 as u64) as slimb_t;
                while i >= pos {
                    a = *(*r).tab.offset(i as isize);
                    *(*r).tab.offset(i as isize) =
                        a >> 1 as i32 | v << ((1 as i32) << 6 as i32) - 1 as i32;
                    v = a;
                    i -= 1
                }
                (*r).expn += 1
            }
        }
        /* check underflow */
        if ((*r).expn < e_min) as i32 as i64 != 0 {
            if flags & ((1 as i32) << 3 as i32) as u32 != 0 {
                /* if inexact, also set the underflow flag */
                if ret & (1 as i32) << 4 as i32 != 0 {
                    ret |= (1 as i32) << 3 as i32
                }
                current_block = 13321564401369230990;
            } else {
                current_block = 1297731417707248850;
            }
        } else {
            current_block = 13321564401369230990;
        }
        match current_block {
            1297731417707248850 => {}
            _ => {
                /* check overflow */
                if ((*r).expn > e_max) as i32 as i64 != 0 {
                    return bf_set_overflow(r, (*r).sign, prec1, flags);
                }
                /* keep the bits starting at 'prec - 1' */
                bit_pos = l
                    .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_sub((prec - 1 as i32 as i64) as u64)
                    as slimb_t;
                i = bit_pos >> 6 as i32;
                if i >= 0 as i32 as i64 {
                    shift = (bit_pos & (((1 as i32) << 6 as i32) - 1 as i32) as i64) as i32;
                    if shift != 0 as i32 {
                        let ref mut fresh0 = *(*r).tab.offset(i as isize);
                        *fresh0 &= limb_mask(shift, ((1 as i32) << 6 as i32) - 1 as i32)
                    }
                } else {
                    i = 0 as i32 as slimb_t
                }
                /* remove trailing zeros */
                while *(*r).tab.offset(i as isize) == 0 as i32 as u64 {
                    i += 1
                } /* cannot fail */
                if i > 0 as i32 as i64 {
                    l = (l as u64).wrapping_sub(i as u64) as limb_t as limb_t;
                    ((*r).tab as *mut u8).copy_from_nonoverlapping(
                        (*r).tab.offset(i as isize) as *const u8,
                        (l as usize).wrapping_mul(std::mem::size_of::<limb_t>()),
                    );
                }
                bf_resize(r, l);
                return ret;
            }
        }
    }
    ret |= (1 as i32) << 3 as i32 | (1 as i32) << 4 as i32;
    bf_set_zero(r, (*r).sign);
    return ret;
}
/* 'r' must be a finite number. */
#[no_mangle]
pub unsafe extern "C" fn bf_normalize_and_round(
    mut r: *mut bf_t,
    mut prec1: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut l: limb_t = 0;
    let mut v: limb_t = 0;
    let mut a: limb_t = 0;
    let mut shift: i32 = 0;
    let mut ret: i32 = 0;
    let mut i: slimb_t = 0;
    //    bf_print_str("bf_renorm", r);
    l = (*r).len;
    while l > 0 as i32 as u64
        && *(*r).tab.offset(l.wrapping_sub(1 as i32 as u64) as isize) == 0 as i32 as u64
    {
        l = l.wrapping_sub(1)
    }
    if l == 0 as i32 as u64 {
        /* zero */
        (*r).expn = -(9223372036854775807 as i64) - 1 as i32 as i64; /* cannot fail */
        bf_resize(r, 0 as i32 as limb_t);
        ret = 0 as i32
    } else {
        (*r).expn = ((*r).expn as u64).wrapping_sub(
            (*r).len
                .wrapping_sub(l)
                .wrapping_mul(((1 as i32) << 6 as i32) as u64),
        ) as slimb_t as slimb_t;
        /* shift to have the MSB set to '1' */
        v = *(*r).tab.offset(l.wrapping_sub(1 as i32 as u64) as isize);
        shift = clz(v);
        if shift != 0 as i32 {
            v = 0 as i32 as limb_t;
            i = 0 as i32 as slimb_t;
            while (i as u64) < l {
                a = *(*r).tab.offset(i as isize);
                *(*r).tab.offset(i as isize) = a << shift | v >> ((1 as i32) << 6 as i32) - shift;
                v = a;
                i += 1
            }
            (*r).expn -= shift as i64
        }
        ret = __bf_round(r, prec1, flags, l, 0 as i32)
    }
    //    bf_print_str("r_final", r);
    return ret;
}
/* return true if rounding can be done at precision 'prec' assuming
the exact result r is such that |r-a| <= 2^(EXP(a)-k). */
/* XXX: check the case where the exponent would be incremented by the
rounding */
#[no_mangle]
pub unsafe extern "C" fn bf_can_round(
    mut a: *const bf_t,
    mut prec: slimb_t,
    mut rnd_mode: bf_rnd_t,
    mut k: slimb_t,
) -> i32 {
    let mut is_rndn: BOOL = 0;
    let mut bit_pos: slimb_t = 0;
    let mut n: slimb_t = 0;
    let mut bit: limb_t = 0;
    if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64
        || (*a).expn == 9223372036854775807 as i64
    {
        return FALSE as i32;
    }
    if rnd_mode as u32 == BF_RNDF as i32 as u32 {
        return (k >= prec + 1 as i32 as i64) as i32;
    }
    if (*a).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
        return FALSE as i32;
    }
    is_rndn = (rnd_mode as u32 == BF_RNDN as i32 as u32
        || rnd_mode as u32 == BF_RNDNA as i32 as u32) as i32;
    if k < prec + 2 as i32 as i64 {
        return FALSE as i32;
    }
    bit_pos = (*a)
        .len
        .wrapping_mul(((1 as i32) << 6 as i32) as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_sub(prec as u64) as slimb_t;
    n = k - prec;
    /* bit pattern for RNDN or RNDNA: 0111.. or 1000...
       for other rounding modes: 000... or 111...
    */
    bit = get_bit((*a).tab, (*a).len, bit_pos);
    bit_pos -= 1;
    n -= 1;
    bit ^= is_rndn as u64;
    /* XXX: slow, but a few iterations on average */
    while n != 0 as i32 as i64 {
        if get_bit((*a).tab, (*a).len, bit_pos) != bit {
            return TRUE as i32;
        }
        bit_pos -= 1;
        n -= 1
    }
    return FALSE as i32;
}
/* Cannot fail with BF_ST_MEM_ERROR. */
#[no_mangle]
pub unsafe extern "C" fn bf_round(
    mut r: *mut bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    if (*r).len == 0 as i32 as u64 {
        return 0 as i32;
    }
    return __bf_round(r, prec, flags, (*r).len, 0 as i32);
}

/*
#[no_mangle]
pub unsafe extern "C" fn mp_print_str(
    mut str: *const std::os::raw::c_char,
    mut tab: *const limb_t,
    mut n: limb_t,
) {
    let mut i: slimb_t = 0;
    printf(b"%s= 0x\x00" as *const u8 as *const std::os::raw::c_char, str);
    i = n.wrapping_sub(1 as i32 as u64) as slimb_t;
    while i >= 0 as i32 as i64 {
        if i as u64 != n.wrapping_sub(1 as i32 as u64) {
            printf(b"_\x00" as *const u8 as *const std::os::raw::c_char);
        }
        printf(
            b"%016lx\x00" as *const u8 as *const std::os::raw::c_char,
            *tab.offset(i as isize),
        );
        i -= 1
    }
    printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
}
*/

/*
/* for debugging */
#[no_mangle]
pub unsafe extern "C" fn bf_print_str(mut str: *const std::os::raw::c_char, mut a: *const bf_t) {
    let mut i: slimb_t = 0;
    printf(b"%s=\x00" as *const u8 as *const std::os::raw::c_char, str);
    if (*a).expn == 9223372036854775807 as i64 {
        printf(b"NaN\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        if (*a).sign != 0 {
            putchar('-' as i32);
        }
        if (*a).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
            putchar('0' as i32);
        } else if (*a).expn
            == 9223372036854775807 as i64 - 1 as i32 as i64
        {
            printf(b"Inf\x00" as *const u8 as *const std::os::raw::c_char);
        } else {
            printf(b"0x0.\x00" as *const u8 as *const std::os::raw::c_char);
            i = (*a).len.wrapping_sub(1 as i32 as u64) as slimb_t;
            while i >= 0 as i32 as i64 {
                printf(
                    b"%016lx\x00" as *const u8 as *const std::os::raw::c_char,
                    *(*a).tab.offset(i as isize),
                );
                i -= 1
            }
            printf(b"p%ld\x00" as *const u8 as *const std::os::raw::c_char, (*a).expn);
        }
    }
    printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
}
*/

/* compare the absolute value of 'a' and 'b'. Return < 0 if a < b, 0
if a = b and > 0 otherwise. */
#[no_mangle]
pub unsafe extern "C" fn bf_cmpu(mut a: *const bf_t, mut b: *const bf_t) -> i32 {
    let mut i: slimb_t = 0;
    let mut len: limb_t = 0;
    let mut v1: limb_t = 0;
    let mut v2: limb_t = 0;
    if (*a).expn != (*b).expn {
        if (*a).expn < (*b).expn {
            return -(1 as i32);
        } else {
            return 1 as i32;
        }
    }
    len = bf_max((*a).len as slimb_t, (*b).len as slimb_t) as limb_t;
    i = len.wrapping_sub(1 as i32 as u64) as slimb_t;
    while i >= 0 as i32 as i64 {
        v1 = get_limbz(a, (*a).len.wrapping_sub(len).wrapping_add(i as u64));
        v2 = get_limbz(b, (*b).len.wrapping_sub(len).wrapping_add(i as u64));
        if v1 != v2 {
            if v1 < v2 {
                return -(1 as i32);
            } else {
                return 1 as i32;
            }
        }
        i -= 1
    }
    return 0 as i32;
}
/* Full order: -0 < 0, NaN == NaN and NaN is larger than all other numbers */
#[no_mangle]
pub unsafe extern "C" fn bf_cmp_full(mut a: *const bf_t, mut b: *const bf_t) -> i32 {
    let mut res: i32 = 0;
    if (*a).expn == 9223372036854775807 as i64 || (*b).expn == 9223372036854775807 as i64 {
        if (*a).expn == (*b).expn {
            res = 0 as i32
        } else if (*a).expn == 9223372036854775807 as i64 {
            res = 1 as i32
        } else {
            res = -(1 as i32)
        }
    } else if (*a).sign != (*b).sign {
        res = 1 as i32 - 2 as i32 * (*a).sign
    } else {
        res = bf_cmpu(a, b);
        if (*a).sign != 0 {
            res = -res
        }
    }
    return res;
}
/* Standard floating point comparison: return 2 if one of the operands
is NaN (unordered) or -1, 0, 1 depending on the ordering assuming
-0 == +0 */
#[no_mangle]
pub unsafe extern "C" fn bf_cmp(mut a: *const bf_t, mut b: *const bf_t) -> i32 {
    let mut res: i32 = 0;
    if (*a).expn == 9223372036854775807 as i64 || (*b).expn == 9223372036854775807 as i64 {
        res = 2 as i32
    } else if (*a).sign != (*b).sign {
        if (*a).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
            && (*b).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
        {
            res = 0 as i32
        } else {
            res = 1 as i32 - 2 as i32 * (*a).sign
        }
    } else {
        res = bf_cmpu(a, b);
        if (*a).sign != 0 {
            res = -res
        }
    }
    return res;
}
/* Compute the number of bits 'n' matching the pattern:
   a= X1000..0
   b= X0111..1

   When computing a-b, the result will have at least n leading zero
   bits.

   Precondition: a > b and a.expn - b.expn = 0 or 1
*/
unsafe extern "C" fn count_cancelled_bits(mut a: *const bf_t, mut b: *const bf_t) -> limb_t {
    let mut current_block: u64;
    let mut bit_offset: slimb_t = 0;
    let mut b_offset: slimb_t = 0;
    let mut n: slimb_t = 0;
    let mut p: i32 = 0;
    let mut p1: i32 = 0;
    let mut v1: limb_t = 0;
    let mut v2: limb_t = 0;
    let mut mask: limb_t = 0;
    bit_offset = (*a)
        .len
        .wrapping_mul(((1 as i32) << 6 as i32) as u64)
        .wrapping_sub(1 as i32 as u64) as slimb_t;
    b_offset = (*b)
        .len
        .wrapping_sub((*a).len)
        .wrapping_mul(((1 as i32) << 6 as i32) as u64)
        .wrapping_sub((((1 as i32) << 6 as i32) - 1 as i32) as u64)
        .wrapping_add((*a).expn as u64)
        .wrapping_sub((*b).expn as u64) as slimb_t;
    n = 0 as i32 as slimb_t;
    loop
    /* first search the equals bits */
    {
        v1 = get_limbz(a, (bit_offset >> 6 as i32) as limb_t);
        v2 = get_bits((*b).tab, (*b).len, bit_offset + b_offset);
        //        printf("v1=" FMT_LIMB " v2=" FMT_LIMB "\n", v1, v2);
        if v1 != v2 {
            break;
        }
        n += ((1 as i32) << 6 as i32) as i64;
        bit_offset -= ((1 as i32) << 6 as i32) as i64
    }
    /* find the position of the first different bit */
    p = clz(v1 ^ v2) + 1 as i32;
    n += p as i64;
    /* then search for '0' in a and '1' in b */
    p = ((1 as i32) << 6 as i32) - p;
    if p > 0 as i32 {
        /* search in the trailing p bits of v1 and v2 */
        mask = limb_mask(0 as i32, p - 1 as i32);
        p1 = (bf_min(clz(v1 & mask) as slimb_t, clz(!v2 & mask) as slimb_t)
            - (((1 as i32) << 6 as i32) - p) as i64) as i32;
        n += p1 as i64;
        if p1 != p {
            current_block = 16142469409674772955;
        } else {
            current_block = 5948590327928692120;
        }
    } else {
        current_block = 5948590327928692120;
    }
    match current_block {
        5948590327928692120 => {
            bit_offset -= ((1 as i32) << 6 as i32) as i64;
            loop {
                v1 = get_limbz(a, (bit_offset >> 6 as i32) as limb_t);
                v2 = get_bits((*b).tab, (*b).len, bit_offset + b_offset);
                //        printf("v1=" FMT_LIMB " v2=" FMT_LIMB "\n", v1, v2);
                if v1 != 0 as i32 as u64 || v2 != -(1 as i32) as u64 {
                    /* different: count the matching bits */
                    p1 = bf_min(clz(v1) as slimb_t, clz(!v2) as slimb_t) as i32;
                    n += p1 as i64;
                    break;
                } else {
                    n += ((1 as i32) << 6 as i32) as i64;
                    bit_offset -= ((1 as i32) << 6 as i32) as i64
                }
            }
        }
        _ => {}
    }
    return n as limb_t;
}
unsafe extern "C" fn bf_add_internal(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut b_neg: i32,
) -> i32 {
    let mut d: slimb_t = 0;
    let mut a_offset: slimb_t = 0;
    let mut b_bit_offset: slimb_t = 0;
    let mut i: slimb_t = 0;
    let mut cancelled_bits: slimb_t = 0;
    let mut carry: limb_t = 0;
    let mut v1: limb_t = 0;
    let mut v2: limb_t = 0;
    let mut u: limb_t = 0;
    let mut r_len: limb_t = 0;
    let mut carry1: limb_t = 0;
    let mut precl: limb_t = 0;
    let mut tot_len: limb_t = 0;
    let mut z: limb_t = 0;
    let mut sub_mask: limb_t = 0;
    let mut current_block: u64;
    let mut tmp: *const bf_t = 0 as *const bf_t;
    let mut is_sub: i32 = 0;
    let mut ret: i32 = 0;
    let mut cmp_res: i32 = 0;
    let mut a_sign: i32 = 0;
    let mut b_sign: i32 = 0;
    a_sign = (*a).sign;
    b_sign = (*b).sign ^ b_neg;
    is_sub = a_sign ^ b_sign;
    cmp_res = bf_cmpu(a, b);
    if cmp_res < 0 as i32 {
        tmp = a;
        a = b;
        b = tmp;
        a_sign = b_sign
        /* b_sign is never used later */
    }
    /* abs(a) >= abs(b) */
    if cmp_res == 0 as i32
        && is_sub != 0
        && (*a).expn < 9223372036854775807 as i64 - 1 as i32 as i64
    {
        /* zero result */
        bf_set_zero(
            r,
            (flags & 0x7 as i32 as u32 == BF_RNDD as i32 as u32) as i32,
        );
        ret = 0 as i32
    } else {
        if (*a).len == 0 as i32 as u64 || (*b).len == 0 as i32 as u64 {
            ret = 0 as i32;
            if (*a).expn >= 9223372036854775807 as i64 - 1 as i32 as i64 {
                if (*a).expn == 9223372036854775807 as i64 {
                    /* at least one operand is NaN */
                    bf_set_nan(r);
                } else if (*b).expn == 9223372036854775807 as i64 - 1 as i32 as i64 && is_sub != 0 {
                    /* infinities with different signs */
                    bf_set_nan(r);
                    ret = (1 as i32) << 0 as i32
                } else {
                    bf_set_inf(r, a_sign);
                }
                current_block = 2616667235040759262;
            } else {
                /* at least one zero and not subtract */
                bf_set(r, a);
                (*r).sign = a_sign;
                current_block = 6887627722379705880;
            }
        } else {
            d = 0;
            a_offset = 0;
            b_bit_offset = 0;
            i = 0;
            cancelled_bits = 0;
            carry = 0;
            v1 = 0;
            v2 = 0;
            u = 0;
            r_len = 0;
            carry1 = 0;
            precl = 0;
            tot_len = 0;
            z = 0;
            sub_mask = 0;
            (*r).sign = a_sign;
            (*r).expn = (*a).expn;
            d = (*a).expn - (*b).expn;
            /* must add more precision for the leading cancelled bits in
            subtraction */
            if is_sub != 0 {
                if d <= 1 as i32 as i64 {
                    cancelled_bits = count_cancelled_bits(a, b) as slimb_t
                } else {
                    cancelled_bits = 1 as i32 as slimb_t
                }
            } else {
                cancelled_bits = 0 as i32 as slimb_t
            }
            /* add two extra bits for rounding */
            precl = (cancelled_bits as u64)
                .wrapping_add(prec)
                .wrapping_add(2 as i32 as u64)
                .wrapping_add(((1 as i32) << 6 as i32) as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_div(((1 as i32) << 6 as i32) as u64);
            tot_len = bf_max(
                (*a).len as slimb_t,
                (*b).len.wrapping_add(
                    ((d + ((1 as i32) << 6 as i32) as i64 - 1 as i32 as i64)
                        / ((1 as i32) << 6 as i32) as i64) as u64,
                ) as slimb_t,
            ) as limb_t;
            r_len = bf_min(precl as slimb_t, tot_len as slimb_t) as limb_t;
            if bf_resize(r, r_len) != 0 {
                current_block = 17777247491025708018;
            } else {
                a_offset = (*a).len.wrapping_sub(r_len) as slimb_t;
                b_bit_offset = (*b)
                    .len
                    .wrapping_sub(r_len)
                    .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                    .wrapping_add(d as u64) as slimb_t;
                /* compute the bits before for the rounding */
                carry = is_sub as limb_t;
                z = 0 as i32 as limb_t;
                sub_mask = -is_sub as limb_t;
                i = r_len.wrapping_sub(tot_len) as slimb_t;
                while i < 0 as i32 as i64 {
                    let mut ap: slimb_t = 0;
                    let mut bp: slimb_t = 0;
                    let mut inflag: BOOL = 0;
                    ap = a_offset + i;
                    bp = b_bit_offset + i * ((1 as i32) << 6 as i32) as i64;
                    inflag = FALSE as i32;
                    if ap >= 0 as i32 as i64 && (ap as u64) < (*a).len {
                        v1 = *(*a).tab.offset(ap as isize);
                        inflag = TRUE as i32
                    } else {
                        v1 = 0 as i32 as limb_t
                    }
                    if bp + ((1 as i32) << 6 as i32) as i64 > 0 as i32 as i64
                        && bp < (*b).len.wrapping_mul(((1 as i32) << 6 as i32) as u64) as slimb_t
                    {
                        v2 = get_bits((*b).tab, (*b).len, bp);
                        inflag = TRUE as i32
                    } else {
                        v2 = 0 as i32 as limb_t
                    }
                    if inflag == 0 {
                        /* outside 'a' and 'b': go directly to the next value
                        inside a or b so that the running time does not
                        depend on the exponent difference */
                        i = 0 as i32 as slimb_t;
                        if ap < 0 as i32 as i64 {
                            i = bf_min(i, -a_offset)
                        }
                        /* b_bit_offset + i * LIMB_BITS + LIMB_BITS >= 1
                           equivalent to
                           i >= ceil(-b_bit_offset + 1 - LIMB_BITS) / LIMB_BITS)
                        */
                        if bp + ((1 as i32) << 6 as i32) as i64 <= 0 as i32 as i64 {
                            i = bf_min(i, -b_bit_offset >> 6 as i32)
                        }
                    } else {
                        i += 1
                    }
                    v2 ^= sub_mask;
                    u = v1.wrapping_add(v2);
                    carry1 = (u < v1) as i32 as limb_t;
                    u = (u as u64).wrapping_add(carry) as limb_t as limb_t;
                    carry = (u < carry) as i32 as u64 | carry1;
                    z |= u
                }
                /* and the result */
                i = 0 as i32 as slimb_t;
                while (i as u64) < r_len {
                    v1 = get_limbz(a, (a_offset + i) as limb_t);
                    v2 = get_bits(
                        (*b).tab,
                        (*b).len,
                        b_bit_offset + i * ((1 as i32) << 6 as i32) as i64,
                    );
                    v2 ^= sub_mask;
                    u = v1.wrapping_add(v2);
                    carry1 = (u < v1) as i32 as limb_t;
                    u = (u as u64).wrapping_add(carry) as limb_t as limb_t;
                    carry = (u < carry) as i32 as u64 | carry1;
                    *(*r).tab.offset(i as isize) = u;
                    i += 1
                }
                /* set the extra bits for the rounding */
                let ref mut fresh1 = *(*r).tab.offset(0 as i32 as isize);
                *fresh1 |= (z != 0 as i32 as u64) as i32 as u64;
                /* carry is only possible in add case */
                if is_sub == 0 && carry != 0 {
                    if bf_resize(r, r_len.wrapping_add(1 as i32 as u64)) != 0 {
                        current_block = 17777247491025708018;
                    } else {
                        *(*r).tab.offset(r_len as isize) = 1 as i32 as limb_t;
                        (*r).expn += ((1 as i32) << 6 as i32) as i64;
                        current_block = 6887627722379705880;
                    }
                } else {
                    current_block = 6887627722379705880;
                }
            }
            match current_block {
                6887627722379705880 => {}
                _ => {
                    bf_set_nan(r);
                    return (1 as i32) << 5 as i32;
                }
            }
        }
        match current_block {
            2616667235040759262 => {}
            _ => ret = bf_normalize_and_round(r, prec, flags),
        }
    }
    return ret;
}
unsafe extern "C" fn __bf_add(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_add_internal(r, a, b, prec, flags, 0 as i32);
}
unsafe extern "C" fn __bf_sub(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_add_internal(r, a, b, prec, flags, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn mp_add(
    mut res: *mut limb_t,
    mut op1: *const limb_t,
    mut op2: *const limb_t,
    mut n: limb_t,
    mut carry: limb_t,
) -> limb_t {
    let mut i: slimb_t = 0;
    let mut k: limb_t = 0;
    let mut a: limb_t = 0;
    let mut v: limb_t = 0;
    let mut k1: limb_t = 0;
    k = carry;
    i = 0 as i32 as slimb_t;
    while (i as u64) < n {
        v = *op1.offset(i as isize);
        a = v.wrapping_add(*op2.offset(i as isize));
        k1 = (a < v) as i32 as limb_t;
        a = a.wrapping_add(k);
        k = (a < k) as i32 as u64 | k1;
        *res.offset(i as isize) = a;
        i += 1
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn mp_add_ui(mut tab: *mut limb_t, mut b: limb_t, mut n: size_t) -> limb_t {
    let mut i: size_t = 0;
    let mut k: limb_t = 0;
    let mut a: limb_t = 0;
    k = b;
    i = 0 as i32 as size_t;
    while i < n {
        if k == 0 as i32 as u64 {
            break;
        }
        a = (*tab.offset(i as isize)).wrapping_add(k);
        k = (a < k) as i32 as limb_t;
        *tab.offset(i as isize) = a;
        i = i.wrapping_add(1)
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn mp_sub(
    mut res: *mut limb_t,
    mut op1: *const limb_t,
    mut op2: *const limb_t,
    mut n: mp_size_t,
    mut carry: limb_t,
) -> limb_t {
    let mut i: i32 = 0;
    let mut k: limb_t = 0;
    let mut a: limb_t = 0;
    let mut v: limb_t = 0;
    let mut k1: limb_t = 0;
    k = carry;
    i = 0 as i32;
    while (i as i64) < n {
        v = *op1.offset(i as isize);
        a = v.wrapping_sub(*op2.offset(i as isize));
        k1 = (a > v) as i32 as limb_t;
        v = a.wrapping_sub(k);
        k = (v > a) as i32 as u64 | k1;
        *res.offset(i as isize) = v;
        i += 1
    }
    return k;
}
/* compute 0 - op2 */
unsafe extern "C" fn mp_neg(
    mut res: *mut limb_t,
    mut op2: *const limb_t,
    mut n: mp_size_t,
    mut carry: limb_t,
) -> limb_t {
    let mut i: i32 = 0;
    let mut k: limb_t = 0;
    let mut a: limb_t = 0;
    let mut v: limb_t = 0;
    let mut k1: limb_t = 0;
    k = carry;
    i = 0 as i32;
    while (i as i64) < n {
        v = 0 as i32 as limb_t;
        a = v.wrapping_sub(*op2.offset(i as isize));
        k1 = (a > v) as i32 as limb_t;
        v = a.wrapping_sub(k);
        k = (v > a) as i32 as u64 | k1;
        *res.offset(i as isize) = v;
        i += 1
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn mp_sub_ui(
    mut tab: *mut limb_t,
    mut b: limb_t,
    mut n: mp_size_t,
) -> limb_t {
    let mut i: mp_size_t = 0;
    let mut k: limb_t = 0;
    let mut a: limb_t = 0;
    let mut v: limb_t = 0;
    k = b;
    i = 0 as i32 as mp_size_t;
    while i < n {
        v = *tab.offset(i as isize);
        a = v.wrapping_sub(k);
        k = (a > v) as i32 as limb_t;
        *tab.offset(i as isize) = a;
        if k == 0 as i32 as u64 {
            break;
        }
        i += 1
    }
    return k;
}
/* r = (a + high*B^n) >> shift. Return the remainder r (0 <= r < 2^shift).
1 <= shift <= LIMB_BITS - 1 */
unsafe extern "C" fn mp_shr(
    mut tab_r: *mut limb_t,
    mut tab: *const limb_t,
    mut n: mp_size_t,
    mut shift: i32,
    mut high: limb_t,
) -> limb_t {
    let mut i: mp_size_t = 0;
    let mut l: limb_t = 0;
    let mut a: limb_t = 0;
    if shift >= 1 as i32 && shift < (1 as i32) << 6 as i32 {
    } else {
        assert!(shift >= 1 && (shift as u64) < LIMB_BITS);
    }
    l = high;
    i = n - 1 as i32 as i64;
    while i >= 0 as i32 as i64 {
        a = *tab.offset(i as isize);
        *tab_r.offset(i as isize) = a >> shift | l << ((1 as i32) << 6 as i32) - shift;
        l = a;
        i -= 1
    }
    return l & ((1 as i32 as limb_t) << shift).wrapping_sub(1 as i32 as u64);
}
/* tabr[] = taba[] * b + l. Return the high carry */
unsafe extern "C" fn mp_mul1(
    mut tabr: *mut limb_t,
    mut taba: *const limb_t,
    mut n: limb_t,
    mut b: limb_t,
    mut l: limb_t,
) -> limb_t {
    let mut i: limb_t = 0;
    let mut t: dlimb_t = 0;
    i = 0 as i32 as limb_t;
    while i < n {
        t = (*taba.offset(i as isize) as dlimb_t)
            .wrapping_mul(b as dlimb_t)
            .wrapping_add(l as u128);
        *tabr.offset(i as isize) = t as limb_t;
        l = (t >> ((1 as i32) << 6 as i32)) as limb_t;
        i = i.wrapping_add(1)
    }
    return l;
}
/* tabr[] += taba[] * b, return the high word. */
unsafe extern "C" fn mp_add_mul1(
    mut tabr: *mut limb_t,
    mut taba: *const limb_t,
    mut n: limb_t,
    mut b: limb_t,
) -> limb_t {
    let mut i: limb_t = 0;
    let mut l: limb_t = 0;
    let mut t: dlimb_t = 0;
    l = 0 as i32 as limb_t;
    i = 0 as i32 as limb_t;
    while i < n {
        t = (*taba.offset(i as isize) as dlimb_t)
            .wrapping_mul(b as dlimb_t)
            .wrapping_add(l as u128)
            .wrapping_add(*tabr.offset(i as isize) as u128);
        *tabr.offset(i as isize) = t as limb_t;
        l = (t >> ((1 as i32) << 6 as i32)) as limb_t;
        i = i.wrapping_add(1)
    }
    return l;
}
/* size of the result : op1_size + op2_size. */
unsafe extern "C" fn mp_mul_basecase(
    mut result: *mut limb_t,
    mut op1: *const limb_t,
    mut op1_size: limb_t,
    mut op2: *const limb_t,
    mut op2_size: limb_t,
) {
    let mut i: limb_t = 0;
    let mut r: limb_t = 0;
    *result.offset(op1_size as isize) = mp_mul1(
        result,
        op1,
        op1_size,
        *op2.offset(0 as i32 as isize),
        0 as i32 as limb_t,
    );
    i = 1 as i32 as limb_t;
    while i < op2_size {
        r = mp_add_mul1(
            result.offset(i as isize),
            op1,
            op1_size,
            *op2.offset(i as isize),
        );
        *result.offset(i.wrapping_add(op1_size) as isize) = r;
        i = i.wrapping_add(1)
    }
}
/* return 0 if OK, -1 if memory error */
/* XXX: change API so that result can be allocated */
#[no_mangle]
pub unsafe extern "C" fn mp_mul(
    mut s: *mut bf_context_t,
    mut result: *mut limb_t,
    mut op1: *const limb_t,
    mut op1_size: limb_t,
    mut op2: *const limb_t,
    mut op2_size: limb_t,
) -> i32 {
    if (bf_min(op1_size as slimb_t, op2_size as slimb_t) >= 100 as i32 as i64) as i32 as i64 != 0 {
        let mut r_s: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut r: *mut bf_t = &mut r_s;
        (*r).tab = result;
        /* XXX: optimize memory usage in API */
        if fft_mul(
            s,
            r,
            op1 as *mut limb_t,
            op1_size,
            op2 as *mut limb_t,
            op2_size,
            (1 as i32) << 2 as i32,
        ) != 0
        {
            return -(1 as i32);
        }
    } else {
        mp_mul_basecase(result, op1, op1_size, op2, op2_size);
    }
    return 0 as i32;
}
/* tabr[] -= taba[] * b. Return the value to substract to the high
word. */
unsafe extern "C" fn mp_sub_mul1(
    mut tabr: *mut limb_t,
    mut taba: *const limb_t,
    mut n: limb_t,
    mut b: limb_t,
) -> limb_t {
    let mut i: limb_t = 0;
    let mut l: limb_t = 0;
    let mut t: dlimb_t = 0;
    l = 0 as i32 as limb_t;
    i = 0 as i32 as limb_t;
    while i < n {
        t = (*tabr.offset(i as isize) as u128)
            .wrapping_sub((*taba.offset(i as isize) as dlimb_t).wrapping_mul(b as dlimb_t))
            .wrapping_sub(l as u128);
        *tabr.offset(i as isize) = t as limb_t;
        l = (t >> ((1 as i32) << 6 as i32)).wrapping_neg() as limb_t;
        i = i.wrapping_add(1)
    }
    return l;
}
/* WARNING: d must be >= 2^(LIMB_BITS-1) */
#[inline]
unsafe extern "C" fn udiv1norm_init(mut d: limb_t) -> limb_t {
    let mut a0: limb_t = 0;
    let mut a1: limb_t = 0;
    a1 = d.wrapping_neg().wrapping_sub(1 as i32 as u64);
    a0 = -(1 as i32) as limb_t;
    return ((a1 as dlimb_t) << ((1 as i32) << 6 as i32) | a0 as u128).wrapping_div(d as u128)
        as limb_t;
}
/* return the quotient and the remainder in '*pr'of 'a1*2^LIMB_BITS+a0
/ d' with 0 <= a1 < d. */
#[inline]
unsafe extern "C" fn udiv1norm(
    mut pr: *mut limb_t,
    mut a1: limb_t,
    mut a0: limb_t,
    mut d: limb_t,
    mut d_inv: limb_t,
) -> limb_t {
    let mut n1m: limb_t = 0;
    let mut n_adj: limb_t = 0;
    let mut q: limb_t = 0;
    let mut r: limb_t = 0;
    let mut ah: limb_t = 0;
    let mut a: dlimb_t = 0;
    n1m = (a0 as slimb_t >> ((1 as i32) << 6 as i32) - 1 as i32) as limb_t;
    n_adj = a0.wrapping_add(n1m & d);
    a = (d_inv as dlimb_t)
        .wrapping_mul(a1.wrapping_sub(n1m) as u128)
        .wrapping_add(n_adj as u128);
    q = (a >> ((1 as i32) << 6 as i32)).wrapping_add(a1 as u128) as limb_t;
    /* compute a - q * r and update q so that the remainder is\
    between 0 and d - 1 */
    a = (a1 as dlimb_t) << ((1 as i32) << 6 as i32) | a0 as u128;
    a = a
        .wrapping_sub((q as dlimb_t).wrapping_mul(d as u128))
        .wrapping_sub(d as u128);
    ah = (a >> ((1 as i32) << 6 as i32)) as limb_t;
    q = (q as u64).wrapping_add((1 as i32 as u64).wrapping_add(ah)) as limb_t as limb_t;
    r = (a as limb_t).wrapping_add(ah & d);
    *pr = r;
    return q;
}
/* b must be >= 1 << (LIMB_BITS - 1) */
unsafe extern "C" fn mp_div1norm(
    mut tabr: *mut limb_t,
    mut taba: *const limb_t,
    mut n: limb_t,
    mut b: limb_t,
    mut r: limb_t,
) -> limb_t {
    let mut i: slimb_t = 0;
    if n >= 3 as i32 as u64 {
        let mut b_inv: limb_t = 0;
        b_inv = udiv1norm_init(b);
        i = n.wrapping_sub(1 as i32 as u64) as slimb_t;
        while i >= 0 as i32 as i64 {
            *tabr.offset(i as isize) = udiv1norm(&mut r, r, *taba.offset(i as isize), b, b_inv);
            i -= 1
        }
    } else {
        let mut a1: dlimb_t = 0;
        i = n.wrapping_sub(1 as i32 as u64) as slimb_t;
        while i >= 0 as i32 as i64 {
            a1 = (r as dlimb_t) << ((1 as i32) << 6 as i32) | *taba.offset(i as isize) as u128;
            *tabr.offset(i as isize) = a1.wrapping_div(b as u128) as limb_t;
            r = a1.wrapping_rem(b as u128) as limb_t;
            i -= 1
        }
    }
    return r;
}
/* base case division: divides taba[0..na-1] by tabb[0..nb-1]. tabb[nb
- 1] must be >= 1 << (LIMB_BITS - 1). na - nb must be >= 0. 'taba'
is modified and contains the remainder (nb limbs). tabq[0..na-nb]
contains the quotient with tabq[na - nb] <= 1. */
unsafe extern "C" fn mp_divnorm(
    mut s: *mut bf_context_t,
    mut tabq: *mut limb_t,
    mut taba: *mut limb_t,
    mut na: limb_t,
    mut tabb: *const limb_t,
    mut nb: limb_t,
) -> i32 {
    let mut r: limb_t = 0;
    let mut a: limb_t = 0;
    let mut c: limb_t = 0;
    let mut q: limb_t = 0;
    let mut v: limb_t = 0;
    let mut b1: limb_t = 0;
    let mut b1_inv: limb_t = 0;
    let mut n: limb_t = 0;
    let mut dummy_r: limb_t = 0;
    let mut i: slimb_t = 0;
    let mut j: slimb_t = 0;
    b1 = *tabb.offset(nb.wrapping_sub(1 as i32 as u64) as isize);
    if nb == 1 as i32 as u64 {
        *taba.offset(0 as i32 as isize) = mp_div1norm(tabq, taba, na, b1, 0 as i32 as limb_t);
        return 0 as i32;
    }
    n = na.wrapping_sub(nb);
    if bf_min(n as slimb_t, nb as slimb_t) >= 50 as i32 as i64 {
        return mp_divnorm_large(s, tabq, taba, na, tabb, nb);
    }
    if n >= 3 as i32 as u64 {
        b1_inv = udiv1norm_init(b1)
    } else {
        b1_inv = 0 as i32 as limb_t
    }
    /* first iteration: the quotient is only 0 or 1 */
    q = 1 as i32 as limb_t;
    j = nb.wrapping_sub(1 as i32 as u64) as slimb_t;
    while j >= 0 as i32 as i64 {
        if *taba.offset(n.wrapping_add(j as u64) as isize) != *tabb.offset(j as isize) {
            if *taba.offset(n.wrapping_add(j as u64) as isize) < *tabb.offset(j as isize) {
                q = 0 as i32 as limb_t
            }
            break;
        } else {
            j -= 1
        }
    }
    *tabq.offset(n as isize) = q;
    if q != 0 {
        mp_sub(
            taba.offset(n as isize),
            taba.offset(n as isize),
            tabb,
            nb as mp_size_t,
            0 as i32 as limb_t,
        );
    }
    i = n.wrapping_sub(1 as i32 as u64) as slimb_t;
    while i >= 0 as i32 as i64 {
        if (*taba.offset((i as u64).wrapping_add(nb) as isize) >= b1) as i32 as i64 != 0 {
            q = -(1 as i32) as limb_t
        } else if b1_inv != 0 {
            q = udiv1norm(
                &mut dummy_r,
                *taba.offset((i as u64).wrapping_add(nb) as isize),
                *taba.offset((i as u64).wrapping_add(nb).wrapping_sub(1 as i32 as u64) as isize),
                b1,
                b1_inv,
            )
        } else {
            let mut al: dlimb_t = 0;
            al = (*taba.offset((i as u64).wrapping_add(nb) as isize) as dlimb_t)
                << ((1 as i32) << 6 as i32)
                | *taba.offset((i as u64).wrapping_add(nb).wrapping_sub(1 as i32 as u64) as isize)
                    as u128;
            q = al.wrapping_div(b1 as u128) as limb_t;
            r = al.wrapping_rem(b1 as u128) as limb_t
        }
        r = mp_sub_mul1(taba.offset(i as isize), tabb, nb, q);
        v = *taba.offset((i as u64).wrapping_add(nb) as isize);
        a = v.wrapping_sub(r);
        c = (a > v) as i32 as limb_t;
        *taba.offset((i as u64).wrapping_add(nb) as isize) = a;
        if c != 0 as i32 as u64 {
            loop
            /* negative result */
            {
                q = q.wrapping_sub(1);
                c = mp_add(
                    taba.offset(i as isize),
                    taba.offset(i as isize),
                    tabb,
                    nb,
                    0 as i32 as limb_t,
                );
                /* propagate carry and test if positive result */
                if !(c != 0 as i32 as u64) {
                    continue;
                }
                let ref mut fresh2 = *taba.offset((i as u64).wrapping_add(nb) as isize);
                *fresh2 = (*fresh2).wrapping_add(1);
                if *fresh2 == 0 as i32 as u64 {
                    break;
                }
            }
        }
        *tabq.offset(i as isize) = q;
        i -= 1
    }
    return 0 as i32;
}
/* compute r=B^(2*n)/a such as a*r < B^(2*n) < a*r + 2 with n >= 1. 'a'
has n limbs with a[n-1] >= B/2 and 'r' has n+1 limbs with r[n] = 1.

See Modern Computer Arithmetic by Richard P. Brent and Paul
Zimmermann, algorithm 3.5 */
#[no_mangle]
pub unsafe extern "C" fn mp_recip(
    mut s: *mut bf_context_t,
    mut tabr: *mut limb_t,
    mut taba: *const limb_t,
    mut n: limb_t,
) -> i32 {
    let mut current_block: u64;
    let mut l: mp_size_t = 0;
    let mut h: mp_size_t = 0;
    let mut k: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut tabxh: *mut limb_t = 0 as *mut limb_t;
    let mut tabt: *mut limb_t = 0 as *mut limb_t;
    let mut c: limb_t = 0;
    let mut tabu: *mut limb_t = 0 as *mut limb_t;
    if n <= 2 as i32 as u64 {
        /* return ceil(B^(2*n)/a) - 1 */
        /* XXX: could avoid allocation */
        tabu = bf_malloc(
            s,
            (::std::mem::size_of::<limb_t>() as u64).wrapping_mul(
                (2 as i32 as u64)
                    .wrapping_mul(n)
                    .wrapping_add(1 as i32 as u64),
            ),
        ) as *mut limb_t;
        tabt = bf_malloc(
            s,
            (::std::mem::size_of::<limb_t>() as u64).wrapping_mul(n.wrapping_add(2 as i32 as u64)),
        ) as *mut limb_t;
        if tabt.is_null() || tabu.is_null() {
            current_block = 14207563356106830746;
        } else {
            i = 0 as i32 as mp_size_t;
            while (i as u64) < (2 as i32 as u64).wrapping_mul(n) {
                *tabu.offset(i as isize) = 0 as i32 as limb_t;
                i += 1
            }
            *tabu.offset((2 as i32 as u64).wrapping_mul(n) as isize) = 1 as i32 as limb_t;
            if mp_divnorm(
                s,
                tabt,
                tabu,
                (2 as i32 as u64)
                    .wrapping_mul(n)
                    .wrapping_add(1 as i32 as u64),
                taba,
                n,
            ) != 0
            {
                current_block = 14207563356106830746;
            } else {
                i = 0 as i32 as mp_size_t;
                while (i as u64) < n.wrapping_add(1 as i32 as u64) {
                    *tabr.offset(i as isize) = *tabt.offset(i as isize);
                    i += 1
                }
                if mp_scan_nz(tabu, n as mp_size_t) == 0 as i32 as u64 {
                    /* only happens for a=B^n/2 */
                    mp_sub_ui(
                        tabr,
                        1 as i32 as limb_t,
                        n.wrapping_add(1 as i32 as u64) as mp_size_t,
                    );
                }
                current_block = 6450636197030046351;
            }
        }
    } else {
        l = n
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(2 as i32 as u64) as mp_size_t;
        h = n.wrapping_sub(l as u64) as mp_size_t;
        /* n=2p  -> l=p-1, h = p + 1, k = p + 3
           n=2p+1-> l=p,  h = p + 1; k = p + 2
        */
        tabt = bf_malloc(
            s,
            (::std::mem::size_of::<limb_t>() as u64)
                .wrapping_mul(n.wrapping_add(h as u64).wrapping_add(1 as i32 as u64)),
        ) as *mut limb_t;
        tabu = bf_malloc(
            s,
            (::std::mem::size_of::<limb_t>() as u64).wrapping_mul(
                n.wrapping_add((2 as i32 as i64 * h) as u64)
                    .wrapping_sub(l as u64)
                    .wrapping_add(2 as i32 as u64),
            ),
        ) as *mut limb_t;
        if tabt.is_null() || tabu.is_null() {
            current_block = 14207563356106830746;
        } else {
            tabxh = tabr.offset(l as isize);
            if mp_recip(s, tabxh, taba.offset(l as isize), h as limb_t) != 0 {
                current_block = 14207563356106830746;
            } else if mp_mul(s, tabt, taba, n, tabxh, (h + 1 as i32 as i64) as limb_t) != 0 {
                current_block = 14207563356106830746;
            } else {
                while *tabt.offset(n.wrapping_add(h as u64) as isize) != 0 as i32 as u64 {
                    mp_sub_ui(tabxh, 1 as i32 as limb_t, h + 1 as i32 as i64);
                    c = mp_sub(tabt, tabt, taba, n as mp_size_t, 0 as i32 as limb_t);
                    mp_sub_ui(tabt.offset(n as isize), c, h + 1 as i32 as i64);
                }
                /* T = B^(n+h) - T */
                mp_neg(
                    tabt,
                    tabt,
                    n.wrapping_add(h as u64).wrapping_add(1 as i32 as u64) as mp_size_t,
                    0 as i32 as limb_t,
                );
                let ref mut fresh3 = *tabt.offset(n.wrapping_add(h as u64) as isize);
                *fresh3 = (*fresh3).wrapping_add(1);
                if mp_mul(
                    s,
                    tabu,
                    tabt.offset(l as isize),
                    n.wrapping_add(h as u64)
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_sub(l as u64),
                    tabxh,
                    (h + 1 as i32 as i64) as limb_t,
                ) != 0
                {
                    current_block = 14207563356106830746;
                } else {
                    /* n + 2*h - l + 2 limbs */
                    k = 2 as i32 as i64 * h - l;
                    i = 0 as i32 as mp_size_t;
                    while i < l {
                        *tabr.offset(i as isize) = *tabu.offset((i + k) as isize);
                        i += 1
                    }
                    mp_add(
                        tabr.offset(l as isize),
                        tabr.offset(l as isize),
                        tabu.offset((2 as i32 as i64 * h) as isize),
                        h as limb_t,
                        0 as i32 as limb_t,
                    );
                    current_block = 6450636197030046351;
                }
            }
        }
    }
    match current_block {
        14207563356106830746 =>
        /* n + h + 1 limbs */
        {
            bf_free(s, tabt as *mut std::ffi::c_void);
            bf_free(s, tabu as *mut std::ffi::c_void);
            return -(1 as i32);
        }
        _ => {
            bf_free(s, tabt as *mut std::ffi::c_void);
            bf_free(s, tabu as *mut std::ffi::c_void);
            return 0 as i32;
        }
    };
}
/* return -1, 0 or 1 */
unsafe extern "C" fn mp_cmp(
    mut taba: *const limb_t,
    mut tabb: *const limb_t,
    mut n: mp_size_t,
) -> i32 {
    let mut i: mp_size_t = 0;
    i = n - 1 as i32 as i64;
    while i >= 0 as i32 as i64 {
        if *taba.offset(i as isize) != *tabb.offset(i as isize) {
            if *taba.offset(i as isize) < *tabb.offset(i as isize) {
                return -(1 as i32);
            } else {
                return 1 as i32;
            }
        }
        i -= 1
    }
    return 0 as i32;
}
//#define DEBUG_DIVNORM_LARGE
//#define DEBUG_DIVNORM_LARGE2
/* subquadratic divnorm */
unsafe extern "C" fn mp_divnorm_large(
    mut s: *mut bf_context_t,
    mut tabq: *mut limb_t,
    mut taba: *mut limb_t,
    mut na: limb_t,
    mut tabb: *const limb_t,
    mut nb: limb_t,
) -> i32 {
    let mut current_block: u64;
    let mut tabb_inv: *mut limb_t = 0 as *mut limb_t;
    let mut nq: limb_t = 0;
    let mut tabt: *mut limb_t = 0 as *mut limb_t;
    let mut i: limb_t = 0;
    let mut n: limb_t = 0;
    nq = na.wrapping_sub(nb);
    if nq >= 1 as i32 as u64 {
    } else {
        assert!(nq >= 1);
    }
    n = nq;
    if nq < nb {
        n = n.wrapping_add(1)
    }
    tabb_inv = bf_malloc(
        s,
        (::std::mem::size_of::<limb_t>() as u64).wrapping_mul(n.wrapping_add(1 as i32 as u64)),
    ) as *mut limb_t;
    tabt = bf_malloc(
        s,
        (::std::mem::size_of::<limb_t>() as u64)
            .wrapping_mul(2 as i32 as u64)
            .wrapping_mul(n.wrapping_add(1 as i32 as u64)),
    ) as *mut limb_t;
    if !(tabb_inv.is_null() || tabt.is_null()) {
        if n >= nb {
            i = 0 as i32 as limb_t;
            while i < n.wrapping_sub(nb) {
                *tabt.offset(i as isize) = 0 as i32 as limb_t;
                i = i.wrapping_add(1)
            }
            i = 0 as i32 as limb_t;
            while i < nb {
                *tabt.offset(i.wrapping_add(n).wrapping_sub(nb) as isize) =
                    *tabb.offset(i as isize);
                i = i.wrapping_add(1)
            }
            current_block = 18317007320854588510;
        } else {
            /* truncate B: need to increment it so that the approximate
            inverse is smaller that the exact inverse */
            i = 0 as i32 as limb_t;
            while i < n {
                *tabt.offset(i as isize) =
                    *tabb.offset(i.wrapping_add(nb).wrapping_sub(n) as isize);
                i = i.wrapping_add(1)
            }
            if mp_add_ui(tabt, 1 as i32 as limb_t, n) != 0 {
                /* tabt = B^n : tabb_inv = B^n */
                (tabb_inv as *mut u8)
                    .write_bytes(0, (n as usize).wrapping_mul(std::mem::size_of::<limb_t>()));
                *tabb_inv.offset(n as isize) = 1 as i32 as limb_t;
                current_block = 10280019852174212328;
            } else {
                current_block = 18317007320854588510;
            }
        }
        match current_block {
            18317007320854588510 => {
                if mp_recip(s, tabb_inv, tabt, n) != 0 {
                    current_block = 6525116257640989054;
                } else {
                    current_block = 10280019852174212328;
                }
            }
            _ => {}
        }
        match current_block {
            6525116257640989054 => {}
            _ =>
            /* Q=A*B^-1 */
            {
                if !(mp_mul(
                    s,
                    tabt,
                    tabb_inv,
                    n.wrapping_add(1 as i32 as u64),
                    taba.offset(na as isize)
                        .offset(-(n.wrapping_add(1 as i32 as u64) as isize)),
                    n.wrapping_add(1 as i32 as u64),
                ) != 0)
                {
                    i = 0 as i32 as limb_t;
                    while i < nq.wrapping_add(1 as i32 as u64) {
                        *tabq.offset(i as isize) = *tabt.offset(
                            i.wrapping_add(
                                (2 as i32 as u64).wrapping_mul(n.wrapping_add(1 as i32 as u64)),
                            )
                            .wrapping_sub(nq.wrapping_add(1 as i32 as u64))
                                as isize,
                        );
                        i = i.wrapping_add(1)
                    }
                    bf_free(s, tabt as *mut std::ffi::c_void);
                    bf_free(s, tabb_inv as *mut std::ffi::c_void);
                    tabb_inv = 0 as *mut limb_t;
                    /* R=A-B*Q */
                    tabt = bf_malloc(
                        s,
                        (::std::mem::size_of::<limb_t>() as u64)
                            .wrapping_mul(na.wrapping_add(1 as i32 as u64)),
                    ) as *mut limb_t;
                    if !tabt.is_null() {
                        if !(mp_mul(s, tabt, tabq, nq.wrapping_add(1 as i32 as u64), tabb, nb) != 0)
                        {
                            /* we add one more limb for the result */
                            mp_sub(
                                taba,
                                taba,
                                tabt,
                                nb.wrapping_add(1 as i32 as u64) as mp_size_t,
                                0 as i32 as limb_t,
                            );
                            bf_free(s, tabt as *mut std::ffi::c_void);
                            /* the approximated quotient is smaller than than the exact one,
                            hence we may have to increment it */
                            while !(*taba.offset(nb as isize) == 0 as i32 as u64
                                && mp_cmp(taba, tabb, nb as mp_size_t) < 0 as i32)
                            {
                                let ref mut fresh4 = *taba.offset(nb as isize);
                                *fresh4 = (*fresh4 as u64).wrapping_sub(mp_sub(
                                    taba,
                                    taba,
                                    tabb,
                                    nb as mp_size_t,
                                    0 as i32 as limb_t,
                                )) as limb_t as limb_t;
                                mp_add_ui(
                                    tabq,
                                    1 as i32 as limb_t,
                                    nq.wrapping_add(1 as i32 as u64),
                                );
                            }
                            return 0 as i32;
                        }
                    }
                }
            }
        }
    }
    bf_free(s, tabb_inv as *mut std::ffi::c_void);
    bf_free(s, tabt as *mut std::ffi::c_void);
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn bf_mul(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut ret: i32 = 0;
    let mut r_sign: i32 = 0;
    if (*a).len < (*b).len {
        let mut tmp: *const bf_t = a;
        a = b;
        b = tmp
    }
    r_sign = (*a).sign ^ (*b).sign;
    /* here b->len <= a->len */
    if (*b).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 || (*b).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            ret = 0 as i32
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64
            || (*b).expn == 9223372036854775807 as i64 - 1 as i32 as i64
        {
            if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64
                && (*b).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
                || (*a).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
                    && (*b).expn == 9223372036854775807 as i64 - 1 as i32 as i64
            {
                bf_set_nan(r);
                ret = (1 as i32) << 0 as i32
            } else {
                bf_set_inf(r, r_sign);
                ret = 0 as i32
            }
        } else {
            bf_set_zero(r, r_sign);
            ret = 0 as i32
        }
    } else {
        let mut current_block_47: u64;
        let mut tmp_0: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut r1: *mut bf_t = 0 as *mut bf_t;
        let mut a_len: limb_t = 0;
        let mut b_len: limb_t = 0;
        let mut precl: limb_t = 0;
        let mut a_tab: *mut limb_t = 0 as *mut limb_t;
        let mut b_tab: *mut limb_t = 0 as *mut limb_t;
        a_len = (*a).len;
        b_len = (*b).len;
        if flags & 0x7 as i32 as u32 == BF_RNDF as i32 as u32 {
            /* faithful rounding does not require using the full inputs */
            precl = prec
                .wrapping_add(2 as i32 as u64)
                .wrapping_add(((1 as i32) << 6 as i32) as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_div(((1 as i32) << 6 as i32) as u64);
            a_len = bf_min(a_len as slimb_t, precl as slimb_t) as limb_t;
            b_len = bf_min(b_len as slimb_t, precl as slimb_t) as limb_t
        }
        a_tab = (*a).tab.offset((*a).len as isize).offset(-(a_len as isize));
        b_tab = (*b).tab.offset((*b).len as isize).offset(-(b_len as isize));
        if b_len >= 100 as i32 as u64 {
            let mut mul_flags: i32 = 0 as i32;
            if r == a as *mut bf_t {
                mul_flags |= (1 as i32) << 0 as i32
            }
            if r == b as *mut bf_t {
                mul_flags |= (1 as i32) << 1 as i32
            }
            if fft_mul((*r).ctx, r, a_tab, a_len, b_tab, b_len, mul_flags) != 0 {
                current_block_47 = 8868191472144492663;
            } else {
                current_block_47 = 313581471991351815;
            }
        } else {
            if r == a as *mut bf_t || r == b as *mut bf_t {
                bf_init((*r).ctx, &mut tmp_0);
                r1 = r;
                r = &mut tmp_0
            }
            if bf_resize(r, a_len.wrapping_add(b_len)) != 0 {
                current_block_47 = 8868191472144492663;
            } else {
                mp_mul_basecase((*r).tab, a_tab, a_len, b_tab, b_len);
                current_block_47 = 313581471991351815;
            }
        }
        match current_block_47 {
            313581471991351815 => {
                (*r).sign = r_sign;
                (*r).expn = (*a).expn + (*b).expn;
                ret = bf_normalize_and_round(r, prec, flags)
            }
            _ => {
                bf_set_nan(r);
                ret = (1 as i32) << 5 as i32
            }
        }
        if r == &mut tmp_0 as *mut bf_t {
            bf_move(r1, &mut tmp_0);
        }
    }
    return ret;
}
/* multiply 'r' by 2^e */
#[no_mangle]
pub unsafe extern "C" fn bf_mul_2exp(
    mut r: *mut bf_t,
    mut e: slimb_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut e_max: slimb_t = 0;
    if (*r).len == 0 as i32 as u64 {
        return 0 as i32;
    }
    e_max = ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 3 as i32 + 1 as i32)
        .wrapping_sub(1 as i32 as u64) as slimb_t;
    e = bf_max(e, -e_max);
    e = bf_min(e, e_max);
    (*r).expn += e;
    return __bf_round(r, prec, flags, (*r).len, 0 as i32);
}
/* Return e such as a=m*2^e with m odd integer. return 0 if a is zero,
Infinite or Nan. */
#[no_mangle]
pub unsafe extern "C" fn bf_get_exp_min(mut a: *const bf_t) -> slimb_t {
    let mut i: slimb_t = 0;
    let mut v: limb_t = 0;
    let mut k: i32 = 0;
    i = 0 as i32 as slimb_t;
    while (i as u64) < (*a).len {
        v = *(*a).tab.offset(i as isize);
        if v != 0 as i32 as u64 {
            k = ctz(v);
            return ((*a).expn as u64)
                .wrapping_sub(
                    (*a).len
                        .wrapping_sub(i as u64)
                        .wrapping_mul(((1 as i32) << 6 as i32) as u64),
                )
                .wrapping_add(k as u64) as slimb_t;
        }
        i += 1
    }
    return 0 as i32 as slimb_t;
}
/* a and b must be finite numbers with a >= 0 and b > 0. 'q' is the
integer defined as floor(a/b) and r = a - q * b. */
unsafe extern "C" fn bf_tdivremu(
    mut q: *mut bf_t,
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
) {
    if bf_cmpu(a, b) < 0 as i32 {
        bf_set_ui(q, 0 as i32 as uint64_t);
        bf_set(r, a);
    } else {
        bf_div(
            q,
            a,
            b,
            bf_max((*a).expn - (*b).expn + 1 as i32 as i64, 2 as i32 as slimb_t) as limb_t,
            BF_RNDZ as i32 as bf_flags_t,
        );
        bf_rint(q, BF_RNDZ as i32);
        bf_mul(
            r,
            q,
            b,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        bf_sub(
            r,
            a,
            r,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
    };
}
unsafe extern "C" fn __bf_div(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut ret: i32 = 0;
    let mut r_sign: i32 = 0;
    let mut n: limb_t = 0;
    let mut nb: limb_t = 0;
    let mut precl: limb_t = 0;
    r_sign = (*a).sign ^ (*b).sign;
    if (*a).expn >= 9223372036854775807 as i64 - 1 as i32 as i64
        || (*b).expn >= 9223372036854775807 as i64 - 1 as i32 as i64
    {
        if (*a).expn == 9223372036854775807 as i64 || (*b).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64
            && (*b).expn == 9223372036854775807 as i64 - 1 as i32 as i64
        {
            bf_set_nan(r);
            return (1 as i32) << 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            bf_set_inf(r, r_sign);
            return 0 as i32;
        } else {
            bf_set_zero(r, r_sign);
            return 0 as i32;
        }
    } else {
        if (*a).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
            if (*b).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
                bf_set_nan(r);
                return (1 as i32) << 0 as i32;
            } else {
                bf_set_zero(r, r_sign);
                return 0 as i32;
            }
        } else {
            if (*b).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
                bf_set_inf(r, r_sign);
                return (1 as i32) << 1 as i32;
            }
        }
    }
    /* number of limbs of the quotient (2 extra bits for rounding) */
    precl = prec
        .wrapping_add(2 as i32 as u64)
        .wrapping_add(((1 as i32) << 6 as i32) as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_div(((1 as i32) << 6 as i32) as u64);
    nb = (*b).len;
    n = bf_max((*a).len as slimb_t, precl as slimb_t) as limb_t;
    let mut taba: *mut limb_t = 0 as *mut limb_t;
    let mut na: limb_t = 0;
    let mut d: slimb_t = 0;
    na = n.wrapping_add(nb);
    taba = bf_malloc(
        s,
        na.wrapping_add(1 as i32 as u64)
            .wrapping_mul(::std::mem::size_of::<limb_t>() as u64),
    ) as *mut limb_t;
    if !taba.is_null() {
        d = na.wrapping_sub((*a).len) as slimb_t;
        (taba as *mut u8).write_bytes(0, (d as usize).wrapping_mul(std::mem::size_of::<limb_t>()));
        (taba.offset(d as isize) as *mut u8).copy_from(
            (*a).tab as *const u8,
            ((*a).len as usize).wrapping_mul(std::mem::size_of::<limb_t>()),
        );
        if !(bf_resize(r, n.wrapping_add(1 as i32 as u64)) != 0) {
            if !(mp_divnorm(s, (*r).tab, taba, na, (*b).tab, nb) != 0) {
                /* see if non zero remainder */
                if mp_scan_nz(taba, nb as mp_size_t) != 0 {
                    let ref mut fresh5 = *(*r).tab.offset(0 as i32 as isize);
                    *fresh5 |= 1 as i32 as u64
                }
                bf_free((*r).ctx, taba as *mut std::ffi::c_void);
                (*r).expn = (*a).expn - (*b).expn + ((1 as i32) << 6 as i32) as i64;
                (*r).sign = r_sign;
                ret = bf_normalize_and_round(r, prec, flags);
                return ret;
            }
        }
        bf_free(s, taba as *mut std::ffi::c_void);
    }
    bf_set_nan(r);
    return (1 as i32) << 5 as i32;
}
/* division and remainder.

   rnd_mode is the rounding mode for the quotient. The additional
   rounding mode BF_RND_EUCLIDIAN is supported.

   'q' is an integer. 'r' is rounded with prec and flags (prec can be
   BF_PREC_INF).
*/
#[no_mangle]
pub unsafe extern "C" fn bf_divrem(
    mut q: *mut bf_t,
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut rnd_mode: i32,
) -> i32 {
    let mut current_block: u64;
    let mut a1_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut a1: *mut bf_t = &mut a1_s;
    let mut b1_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut b1: *mut bf_t = &mut b1_s;
    let mut q_sign: i32 = 0;
    let mut ret: i32 = 0;
    let mut is_ceil: BOOL = 0;
    let mut is_rndn: BOOL = 0;
    if q != a as *mut bf_t && q != b as *mut bf_t {
    } else {
        assert!(q as *const bf_t != a && q as *const bf_t != b);
    }
    if r != a as *mut bf_t && r != b as *mut bf_t {
    } else {
        assert!(r as *const bf_t != a && r as *const bf_t != b);
    }
    if q != r {
    } else {
        assert!(q != r);
    }
    if (*a).len == 0 as i32 as u64 || (*b).len == 0 as i32 as u64 {
        bf_set_zero(q, 0 as i32);
        if (*a).expn == 9223372036854775807 as i64 || (*b).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64
            || (*b).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
        {
            bf_set_nan(r);
            return (1 as i32) << 0 as i32;
        } else {
            bf_set(r, a);
            return bf_round(r, prec, flags);
        }
    }
    q_sign = (*a).sign ^ (*b).sign;
    is_rndn = (rnd_mode == BF_RNDN as i32 || rnd_mode == BF_RNDNA as i32) as i32;
    match rnd_mode {
        2 => is_ceil = q_sign,
        3 => is_ceil = q_sign ^ 1 as i32,
        5 => is_ceil = TRUE as i32,
        6 => is_ceil = (*a).sign,
        1 | 0 | 4 | _ => is_ceil = FALSE as i32,
    }
    (*a1).expn = (*a).expn;
    (*a1).tab = (*a).tab;
    (*a1).len = (*a).len;
    (*a1).sign = 0 as i32;
    (*b1).expn = (*b).expn;
    (*b1).tab = (*b).tab;
    (*b1).len = (*b).len;
    (*b1).sign = 0 as i32;
    /* XXX: could improve to avoid having a large 'q' */
    bf_tdivremu(q, r, a1, b1);
    if !(bf_is_nan(q) != 0 || bf_is_nan(r) != 0) {
        if (*r).len != 0 as i32 as u64 {
            if is_rndn != 0 {
                let mut res: i32 = 0;
                (*b1).expn -= 1;
                res = bf_cmpu(r, b1);
                (*b1).expn += 1;
                if res > 0 as i32
                    || res == 0 as i32
                        && (rnd_mode == BF_RNDNA as i32
                            || get_bit(
                                (*q).tab,
                                (*q).len,
                                (*q).len
                                    .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                                    .wrapping_sub((*q).expn as u64)
                                    as slimb_t,
                            ) != 0)
                {
                    current_block = 4189475965179813791;
                } else {
                    current_block = 16738040538446813684;
                }
            } else if is_ceil != 0 {
                current_block = 4189475965179813791;
            } else {
                current_block = 16738040538446813684;
            }
            match current_block {
                16738040538446813684 => {}
                _ => {
                    ret = bf_add_si(
                        q,
                        q,
                        1 as i32 as int64_t,
                        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                            .wrapping_sub(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64),
                        BF_RNDZ as i32 as bf_flags_t,
                    );
                    ret |= bf_sub(
                        r,
                        r,
                        b1,
                        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                            .wrapping_sub(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64),
                        BF_RNDZ as i32 as bf_flags_t,
                    );
                    if ret & (1 as i32) << 5 as i32 != 0 {
                        current_block = 17314202131361834381;
                    } else {
                        current_block = 16738040538446813684;
                    }
                }
            }
        } else {
            current_block = 16738040538446813684;
        }
        match current_block {
            17314202131361834381 => {}
            _ => {
                (*r).sign ^= (*a).sign;
                (*q).sign = q_sign;
                return bf_round(r, prec, flags);
            }
        }
    }
    bf_set_nan(q);
    bf_set_nan(r);
    return (1 as i32) << 5 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bf_rem(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut rnd_mode: i32,
) -> i32 {
    let mut q_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut q: *mut bf_t = &mut q_s;
    let mut ret: i32 = 0;
    bf_init((*r).ctx, q);
    ret = bf_divrem(q, r, a, b, prec, flags, rnd_mode);
    bf_delete(q);
    return ret;
}
#[inline]
unsafe extern "C" fn bf_get_limb(
    mut pres: *mut slimb_t,
    mut a: *const bf_t,
    mut flags: i32,
) -> i32 {
    return bf_get_int64(pres, a, flags);
}
#[no_mangle]
pub unsafe extern "C" fn bf_remquo(
    mut pq: *mut slimb_t,
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut rnd_mode: i32,
) -> i32 {
    let mut q_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut q: *mut bf_t = &mut q_s;
    let mut ret: i32 = 0;
    bf_init((*r).ctx, q);
    ret = bf_divrem(q, r, a, b, prec, flags, rnd_mode);
    bf_get_limb(pq, q, (1 as i32) << 0 as i32);
    bf_delete(q);
    return ret;
}
static mut sqrt_table: [uint16_t; 192] = [
    128 as i32 as uint16_t,
    128 as i32 as uint16_t,
    129 as i32 as uint16_t,
    130 as i32 as uint16_t,
    131 as i32 as uint16_t,
    132 as i32 as uint16_t,
    133 as i32 as uint16_t,
    134 as i32 as uint16_t,
    135 as i32 as uint16_t,
    136 as i32 as uint16_t,
    137 as i32 as uint16_t,
    138 as i32 as uint16_t,
    139 as i32 as uint16_t,
    140 as i32 as uint16_t,
    141 as i32 as uint16_t,
    142 as i32 as uint16_t,
    143 as i32 as uint16_t,
    144 as i32 as uint16_t,
    144 as i32 as uint16_t,
    145 as i32 as uint16_t,
    146 as i32 as uint16_t,
    147 as i32 as uint16_t,
    148 as i32 as uint16_t,
    149 as i32 as uint16_t,
    150 as i32 as uint16_t,
    150 as i32 as uint16_t,
    151 as i32 as uint16_t,
    152 as i32 as uint16_t,
    153 as i32 as uint16_t,
    154 as i32 as uint16_t,
    155 as i32 as uint16_t,
    155 as i32 as uint16_t,
    156 as i32 as uint16_t,
    157 as i32 as uint16_t,
    158 as i32 as uint16_t,
    159 as i32 as uint16_t,
    160 as i32 as uint16_t,
    160 as i32 as uint16_t,
    161 as i32 as uint16_t,
    162 as i32 as uint16_t,
    163 as i32 as uint16_t,
    163 as i32 as uint16_t,
    164 as i32 as uint16_t,
    165 as i32 as uint16_t,
    166 as i32 as uint16_t,
    167 as i32 as uint16_t,
    167 as i32 as uint16_t,
    168 as i32 as uint16_t,
    169 as i32 as uint16_t,
    170 as i32 as uint16_t,
    170 as i32 as uint16_t,
    171 as i32 as uint16_t,
    172 as i32 as uint16_t,
    173 as i32 as uint16_t,
    173 as i32 as uint16_t,
    174 as i32 as uint16_t,
    175 as i32 as uint16_t,
    176 as i32 as uint16_t,
    176 as i32 as uint16_t,
    177 as i32 as uint16_t,
    178 as i32 as uint16_t,
    178 as i32 as uint16_t,
    179 as i32 as uint16_t,
    180 as i32 as uint16_t,
    181 as i32 as uint16_t,
    181 as i32 as uint16_t,
    182 as i32 as uint16_t,
    183 as i32 as uint16_t,
    183 as i32 as uint16_t,
    184 as i32 as uint16_t,
    185 as i32 as uint16_t,
    185 as i32 as uint16_t,
    186 as i32 as uint16_t,
    187 as i32 as uint16_t,
    187 as i32 as uint16_t,
    188 as i32 as uint16_t,
    189 as i32 as uint16_t,
    189 as i32 as uint16_t,
    190 as i32 as uint16_t,
    191 as i32 as uint16_t,
    192 as i32 as uint16_t,
    192 as i32 as uint16_t,
    193 as i32 as uint16_t,
    193 as i32 as uint16_t,
    194 as i32 as uint16_t,
    195 as i32 as uint16_t,
    195 as i32 as uint16_t,
    196 as i32 as uint16_t,
    197 as i32 as uint16_t,
    197 as i32 as uint16_t,
    198 as i32 as uint16_t,
    199 as i32 as uint16_t,
    199 as i32 as uint16_t,
    200 as i32 as uint16_t,
    201 as i32 as uint16_t,
    201 as i32 as uint16_t,
    202 as i32 as uint16_t,
    203 as i32 as uint16_t,
    203 as i32 as uint16_t,
    204 as i32 as uint16_t,
    204 as i32 as uint16_t,
    205 as i32 as uint16_t,
    206 as i32 as uint16_t,
    206 as i32 as uint16_t,
    207 as i32 as uint16_t,
    208 as i32 as uint16_t,
    208 as i32 as uint16_t,
    209 as i32 as uint16_t,
    209 as i32 as uint16_t,
    210 as i32 as uint16_t,
    211 as i32 as uint16_t,
    211 as i32 as uint16_t,
    212 as i32 as uint16_t,
    212 as i32 as uint16_t,
    213 as i32 as uint16_t,
    214 as i32 as uint16_t,
    214 as i32 as uint16_t,
    215 as i32 as uint16_t,
    215 as i32 as uint16_t,
    216 as i32 as uint16_t,
    217 as i32 as uint16_t,
    217 as i32 as uint16_t,
    218 as i32 as uint16_t,
    218 as i32 as uint16_t,
    219 as i32 as uint16_t,
    219 as i32 as uint16_t,
    220 as i32 as uint16_t,
    221 as i32 as uint16_t,
    221 as i32 as uint16_t,
    222 as i32 as uint16_t,
    222 as i32 as uint16_t,
    223 as i32 as uint16_t,
    224 as i32 as uint16_t,
    224 as i32 as uint16_t,
    225 as i32 as uint16_t,
    225 as i32 as uint16_t,
    226 as i32 as uint16_t,
    226 as i32 as uint16_t,
    227 as i32 as uint16_t,
    227 as i32 as uint16_t,
    228 as i32 as uint16_t,
    229 as i32 as uint16_t,
    229 as i32 as uint16_t,
    230 as i32 as uint16_t,
    230 as i32 as uint16_t,
    231 as i32 as uint16_t,
    231 as i32 as uint16_t,
    232 as i32 as uint16_t,
    232 as i32 as uint16_t,
    233 as i32 as uint16_t,
    234 as i32 as uint16_t,
    234 as i32 as uint16_t,
    235 as i32 as uint16_t,
    235 as i32 as uint16_t,
    236 as i32 as uint16_t,
    236 as i32 as uint16_t,
    237 as i32 as uint16_t,
    237 as i32 as uint16_t,
    238 as i32 as uint16_t,
    238 as i32 as uint16_t,
    239 as i32 as uint16_t,
    240 as i32 as uint16_t,
    240 as i32 as uint16_t,
    241 as i32 as uint16_t,
    241 as i32 as uint16_t,
    242 as i32 as uint16_t,
    242 as i32 as uint16_t,
    243 as i32 as uint16_t,
    243 as i32 as uint16_t,
    244 as i32 as uint16_t,
    244 as i32 as uint16_t,
    245 as i32 as uint16_t,
    245 as i32 as uint16_t,
    246 as i32 as uint16_t,
    246 as i32 as uint16_t,
    247 as i32 as uint16_t,
    247 as i32 as uint16_t,
    248 as i32 as uint16_t,
    248 as i32 as uint16_t,
    249 as i32 as uint16_t,
    249 as i32 as uint16_t,
    250 as i32 as uint16_t,
    250 as i32 as uint16_t,
    251 as i32 as uint16_t,
    251 as i32 as uint16_t,
    252 as i32 as uint16_t,
    252 as i32 as uint16_t,
    253 as i32 as uint16_t,
    253 as i32 as uint16_t,
    254 as i32 as uint16_t,
    254 as i32 as uint16_t,
    255 as i32 as uint16_t,
];
/* a >= 2^(LIMB_BITS - 2).  Return (s, r) with s=floor(sqrt(a)) and
r=a-s^2. 0 <= r <= 2 * s */
unsafe extern "C" fn mp_sqrtrem1(mut pr: *mut limb_t, mut a: limb_t) -> limb_t {
    let mut s1: limb_t = 0;
    let mut r1: limb_t = 0;
    let mut s: limb_t = 0;
    let mut r: limb_t = 0;
    let mut q: limb_t = 0;
    let mut u: limb_t = 0;
    let mut num: limb_t = 0;
    /* use a table for the 16 -> 8 bit sqrt */
    s1 = sqrt_table
        [(a >> ((1 as i32) << 6 as i32) - 8 as i32).wrapping_sub(64 as i32 as u64) as usize]
        as limb_t;
    r1 = (a >> ((1 as i32) << 6 as i32) - 16 as i32).wrapping_sub(s1.wrapping_mul(s1));
    if r1 > (2 as i32 as u64).wrapping_mul(s1) {
        r1 = (r1 as u64).wrapping_sub(
            (2 as i32 as u64)
                .wrapping_mul(s1)
                .wrapping_add(1 as i32 as u64),
        ) as limb_t as limb_t;
        s1 = s1.wrapping_add(1)
    }
    /* one iteration to get a 32 -> 16 bit sqrt */
    num =
        r1 << 8 as i32 | a >> ((1 as i32) << 6 as i32) - 32 as i32 + 8 as i32 & 0xff as i32 as u64; /* q <= 2^8 */
    q = num.wrapping_div((2 as i32 as u64).wrapping_mul(s1));
    u = num.wrapping_rem((2 as i32 as u64).wrapping_mul(s1));
    s = (s1 << 8 as i32).wrapping_add(q);
    r = u << 8 as i32 | a >> ((1 as i32) << 6 as i32) - 32 as i32 & 0xff as i32 as u64;
    r = (r as u64).wrapping_sub(q.wrapping_mul(q)) as limb_t as limb_t;
    if (r as slimb_t) < 0 as i32 as i64 {
        s = s.wrapping_sub(1);
        r = (r as u64).wrapping_add(
            (2 as i32 as u64)
                .wrapping_mul(s)
                .wrapping_add(1 as i32 as u64),
        ) as limb_t as limb_t
    }
    s1 = s;
    r1 = r;
    /* one more iteration for 64 -> 32 bit sqrt */
    num = r1 << 16 as i32
        | a >> ((1 as i32) << 6 as i32) - 64 as i32 + 16 as i32 & 0xffff as i32 as u64; /* q <= 2^16 */
    q = num.wrapping_div((2 as i32 as u64).wrapping_mul(s1));
    u = num.wrapping_rem((2 as i32 as u64).wrapping_mul(s1));
    s = (s1 << 16 as i32).wrapping_add(q);
    r = u << 16 as i32 | a >> ((1 as i32) << 6 as i32) - 64 as i32 & 0xffff as i32 as u64;
    r = (r as u64).wrapping_sub(q.wrapping_mul(q)) as limb_t as limb_t;
    if (r as slimb_t) < 0 as i32 as i64 {
        s = s.wrapping_sub(1);
        r = (r as u64).wrapping_add(
            (2 as i32 as u64)
                .wrapping_mul(s)
                .wrapping_add(1 as i32 as u64),
        ) as limb_t as limb_t
    }
    *pr = r;
    return s;
}
/* return floor(sqrt(a)) */
#[no_mangle]
pub unsafe extern "C" fn bf_isqrt(mut a: limb_t) -> limb_t {
    let mut s: limb_t = 0; /* special case when q=2^l */
    let mut r: limb_t = 0;
    let mut k: i32 = 0;
    if a == 0 as i32 as u64 {
        return 0 as i32 as limb_t;
    }
    k = clz(a) & !(1 as i32);
    s = mp_sqrtrem1(&mut r, a << k);
    s >>= k >> 1 as i32;
    return s;
}
unsafe extern "C" fn mp_sqrtrem2(mut tabs: *mut limb_t, mut taba: *mut limb_t) -> limb_t {
    let mut s1: limb_t = 0;
    let mut r1: limb_t = 0;
    let mut s: limb_t = 0;
    let mut q: limb_t = 0;
    let mut u: limb_t = 0;
    let mut a0: limb_t = 0;
    let mut a1: limb_t = 0;
    let mut r: dlimb_t = 0;
    let mut num: dlimb_t = 0;
    let mut l: i32 = 0;
    a0 = *taba.offset(0 as i32 as isize);
    a1 = *taba.offset(1 as i32 as isize);
    s1 = mp_sqrtrem1(&mut r1, a1);
    l = ((1 as i32) << 6 as i32) / 2 as i32;
    num = (r1 as dlimb_t) << l | (a0 >> l) as u128;
    q = num.wrapping_div((2 as i32 as u64).wrapping_mul(s1) as u128) as limb_t;
    u = num.wrapping_rem((2 as i32 as u64).wrapping_mul(s1) as u128) as limb_t;
    s = (s1 << l).wrapping_add(q);
    r = (u as dlimb_t) << l
        | (a0 & ((1 as i32 as limb_t) << l).wrapping_sub(1 as i32 as u64)) as u128;
    if (q >> l != 0 as i32 as u64) as i32 as i64 != 0 {
        r = (r as u128).wrapping_sub((1 as i32 as dlimb_t) << ((1 as i32) << 6 as i32)) as dlimb_t
            as dlimb_t
    } else {
        r = (r as u128).wrapping_sub(q.wrapping_mul(q) as u128) as dlimb_t as dlimb_t
    }
    if ((r >> ((1 as i32) << 6 as i32)) as slimb_t) < 0 as i32 as i64 {
        s = s.wrapping_sub(1);
        r = (r as u128).wrapping_add(
            (2 as i32 as u128)
                .wrapping_mul(s as dlimb_t)
                .wrapping_add(1 as i32 as u128),
        ) as dlimb_t as dlimb_t
    }
    *tabs.offset(0 as i32 as isize) = s;
    *taba.offset(0 as i32 as isize) = r as limb_t;
    return (r >> ((1 as i32) << 6 as i32)) as limb_t;
}
//#define DEBUG_SQRTREM
/* tmp_buf must contain (n / 2 + 1 limbs). *prh contains the highest
limb of the remainder. */
unsafe extern "C" fn mp_sqrtrem_rec(
    mut s: *mut bf_context_t,
    mut tabs: *mut limb_t,
    mut taba: *mut limb_t,
    mut n: limb_t,
    mut tmp_buf: *mut limb_t,
    mut prh: *mut limb_t,
) -> i32 {
    let mut l: limb_t = 0;
    let mut h: limb_t = 0;
    let mut rh: limb_t = 0;
    let mut ql: limb_t = 0;
    let mut qh: limb_t = 0;
    let mut c: limb_t = 0;
    let mut i: limb_t = 0;
    if n == 1 as i32 as u64 {
        *prh = mp_sqrtrem2(tabs, taba);
        return 0 as i32;
    }
    l = n.wrapping_div(2 as i32 as u64);
    h = n.wrapping_sub(l);
    if mp_sqrtrem_rec(
        s,
        tabs.offset(l as isize),
        taba.offset((2 as i32 as u64).wrapping_mul(l) as isize),
        h,
        tmp_buf,
        &mut qh,
    ) != 0
    {
        return -(1 as i32);
    }
    /* the remainder is in taba + 2 * l. Its high bit is in qh */
    if qh != 0 {
        mp_sub(
            taba.offset((2 as i32 as u64).wrapping_mul(l) as isize),
            taba.offset((2 as i32 as u64).wrapping_mul(l) as isize),
            tabs.offset(l as isize),
            h as mp_size_t,
            0 as i32 as limb_t,
        );
    }
    /* instead of dividing by 2*s, divide by s (which is normalized)
    and update q and r */
    if mp_divnorm(
        s,
        tmp_buf,
        taba.offset(l as isize),
        n,
        tabs.offset(l as isize),
        h,
    ) != 0
    {
        return -(1 as i32);
    } /* 0 or 1 */
    qh = (qh as u64).wrapping_add(*tmp_buf.offset(l as isize)) as limb_t as limb_t;
    i = 0 as i32 as limb_t;
    while i < l {
        *tabs.offset(i as isize) = *tmp_buf.offset(i as isize);
        i = i.wrapping_add(1)
    }
    ql = mp_shr(tabs, tabs, l as mp_size_t, 1 as i32, qh & 1 as i32 as u64);
    qh = qh >> 1 as i32;
    if ql != 0 {
        rh = mp_add(
            taba.offset(l as isize),
            taba.offset(l as isize),
            tabs.offset(l as isize),
            h,
            0 as i32 as limb_t,
        )
    } else {
        rh = 0 as i32 as limb_t
    }
    mp_add_ui(tabs.offset(l as isize), qh, h);
    /* q = qh, tabs[l - 1 ... 0], r = taba[n - 1 ... l] */
    /* subtract q^2. if qh = 1 then q = B^l, so we can take shortcuts */
    if qh != 0 {
        c = qh
    } else {
        if mp_mul(s, taba.offset(n as isize), tabs, l, tabs, l) != 0 {
            return -(1 as i32);
        }
        c = mp_sub(
            taba,
            taba,
            taba.offset(n as isize),
            (2 as i32 as u64).wrapping_mul(l) as mp_size_t,
            0 as i32 as limb_t,
        )
    }
    rh = (rh as u64).wrapping_sub(mp_sub_ui(
        taba.offset((2 as i32 as u64).wrapping_mul(l) as isize),
        c,
        n.wrapping_sub((2 as i32 as u64).wrapping_mul(l)) as mp_size_t,
    )) as limb_t as limb_t;
    if (rh as slimb_t) < 0 as i32 as i64 {
        mp_sub_ui(tabs, 1 as i32 as limb_t, n as mp_size_t);
        rh = (rh as u64).wrapping_add(mp_add_mul1(taba, tabs, n, 2 as i32 as limb_t)) as limb_t
            as limb_t;
        rh = (rh as u64).wrapping_add(mp_add_ui(taba, 1 as i32 as limb_t, n)) as limb_t as limb_t
    }
    *prh = rh;
    return 0 as i32;
}
/* 'taba' has 2*n limbs with n >= 1 and taba[2*n-1] >= 2 ^ (LIMB_BITS
- 2). Return (s, r) with s=floor(sqrt(a)) and r=a-s^2. 0 <= r <= 2
* s. tabs has n limbs. r is returned in the lower n limbs of
taba. Its r[n] is the returned value of the function. */
/* Algorithm from the article "Karatsuba Square Root" by Paul Zimmermann and
inspirated from its GMP implementation */
#[no_mangle]
pub unsafe extern "C" fn mp_sqrtrem(
    mut s: *mut bf_context_t,
    mut tabs: *mut limb_t,
    mut taba: *mut limb_t,
    mut n: limb_t,
) -> i32 {
    let mut tmp_buf1: [limb_t; 8] = [0; 8];
    let mut tmp_buf: *mut limb_t = 0 as *mut limb_t;
    let mut n2: mp_size_t = 0;
    let mut ret: i32 = 0;
    n2 = n
        .wrapping_div(2 as i32 as u64)
        .wrapping_add(1 as i32 as u64) as mp_size_t;
    if n2 as u64
        <= (::std::mem::size_of::<[limb_t; 8]>() as u64)
            .wrapping_div(::std::mem::size_of::<limb_t>() as u64)
    {
        tmp_buf = tmp_buf1.as_mut_ptr()
    } else {
        tmp_buf = bf_malloc(
            s,
            (::std::mem::size_of::<limb_t>() as u64).wrapping_mul(n2 as u64),
        ) as *mut limb_t;
        if tmp_buf.is_null() {
            return -(1 as i32);
        }
    }
    ret = mp_sqrtrem_rec(s, tabs, taba, n, tmp_buf, taba.offset(n as isize));
    if tmp_buf != tmp_buf1.as_mut_ptr() {
        bf_free(s, tmp_buf as *mut std::ffi::c_void);
    }
    return ret;
}
/* Integer square root with remainder. 'a' must be an integer. r =
floor(sqrt(a)) and rem = a - r^2.  BF_ST_INEXACT is set if the result
is inexact. 'rem' can be NULL if the remainder is not needed. */
#[no_mangle]
pub unsafe extern "C" fn bf_sqrtrem(
    mut r: *mut bf_t,
    mut rem1: *mut bf_t,
    mut a: *const bf_t,
) -> i32 {
    let mut ret: i32 = 0;
    let mut current_block_30: u64;
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            current_block_30 = 7815301370352969686;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 && (*a).sign != 0 {
            current_block_30 = 7300594896447034620;
        } else {
            bf_set(r, a);
            current_block_30 = 7815301370352969686;
        }
        match current_block_30 {
            7300594896447034620 => {}
            _ => {
                if !rem1.is_null() {
                    bf_set_ui(rem1, 0 as i32 as uint64_t);
                }
                ret = 0 as i32;
                current_block_30 = 1836292691772056875;
            }
        }
    } else if (*a).sign != 0 {
        current_block_30 = 7300594896447034620;
    } else {
        let mut rem_s: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut rem: *mut bf_t = 0 as *mut bf_t;
        bf_sqrt(
            r,
            a,
            (((*a).expn + 1 as i32 as i64) / 2 as i32 as i64) as limb_t,
            BF_RNDZ as i32 as bf_flags_t,
        );
        bf_rint(r, BF_RNDZ as i32);
        /* see if the result is exact by computing the remainder */
        if !rem1.is_null() {
            rem = rem1
        } else {
            rem = &mut rem_s;
            bf_init((*r).ctx, rem);
        }
        /* XXX: could avoid recomputing the remainder */
        bf_mul(
            rem,
            r,
            r,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        bf_neg(rem);
        bf_add(
            rem,
            rem,
            a,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        if bf_is_nan(rem) != 0 {
            ret = (1 as i32) << 5 as i32
        } else if (*rem).len != 0 as i32 as u64 {
            ret = (1 as i32) << 4 as i32
        } else {
            ret = 0 as i32
        }
        if rem1.is_null() {
            bf_delete(rem);
        }
        current_block_30 = 1836292691772056875;
    }
    match current_block_30 {
        7300594896447034620 => {
            bf_set_nan(r);
            if !rem1.is_null() {
                bf_set_ui(rem1, 0 as i32 as uint64_t);
            }
            ret = (1 as i32) << 0 as i32
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bf_sqrt(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut current_block: u64;
    let mut s: *mut bf_context_t = (*a).ctx;
    let mut ret: i32 = 0;
    if r != a as *mut bf_t {
    } else {
        assert!(r as *const bf_t != a);
    }
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            current_block = 5720623009719927633;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 && (*a).sign != 0 {
            current_block = 12964998671468006942;
        } else {
            bf_set(r, a);
            current_block = 5720623009719927633;
        }
        match current_block {
            12964998671468006942 => {}
            _ => {
                ret = 0 as i32;
                current_block = 7427571413727699167;
            }
        }
    } else if (*a).sign != 0 {
        current_block = 12964998671468006942;
    } else {
        let mut a1: *mut limb_t = 0 as *mut limb_t;
        let mut n: slimb_t = 0;
        let mut n1: slimb_t = 0;
        let mut res: limb_t = 0;
        /* convert the mantissa to an integer with at least 2 *
        prec + 4 bits */
        n = (2 as i32 as u64)
            .wrapping_mul(prec.wrapping_add(2 as i32 as u64))
            .wrapping_add((2 as i32 * ((1 as i32) << 6 as i32)) as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div((2 as i32 * ((1 as i32) << 6 as i32)) as u64) as slimb_t;
        if bf_resize(r, n as limb_t) != 0 {
            current_block = 3531709610370321912;
        } else {
            a1 = bf_malloc(
                s,
                (::std::mem::size_of::<limb_t>() as u64)
                    .wrapping_mul(2 as i32 as u64)
                    .wrapping_mul(n as u64),
            ) as *mut limb_t;
            if a1.is_null() {
                current_block = 3531709610370321912;
            } else {
                n1 = bf_min(2 as i32 as i64 * n, (*a).len as slimb_t);
                (a1 as *mut u8).write_bytes(
                    0,
                    (2 * n as usize - n1 as usize).wrapping_mul(std::mem::size_of::<limb_t>()),
                );

                (a1.offset((2 * n as isize)).offset(-(n1 as isize)) as *mut u8).copy_from(
                    (*a).tab.offset((*a).len as isize).offset(-(n1 as isize)) as *const u8,
                    (n1 as usize).wrapping_mul(std::mem::size_of::<limb_t>()),
                );
                if (*a).expn & 1 as i32 as i64 != 0 {
                    res = mp_shr(a1, a1, 2 as i32 as i64 * n, 1 as i32, 0 as i32 as limb_t)
                } else {
                    res = 0 as i32 as limb_t
                }
                if mp_sqrtrem(s, (*r).tab, a1, n as limb_t) != 0 {
                    bf_free(s, a1 as *mut std::ffi::c_void);
                    current_block = 3531709610370321912;
                } else {
                    if res == 0 {
                        res = mp_scan_nz(a1, n + 1 as i32 as i64)
                    }
                    bf_free(s, a1 as *mut std::ffi::c_void);
                    if res == 0 {
                        res = mp_scan_nz((*a).tab, (*a).len.wrapping_sub(n1 as u64) as mp_size_t)
                    }
                    if res != 0 as i32 as u64 {
                        let ref mut fresh6 = *(*r).tab.offset(0 as i32 as isize);
                        *fresh6 |= 1 as i32 as u64
                    }
                    (*r).sign = 0 as i32;
                    (*r).expn = (*a).expn + 1 as i32 as i64 >> 1 as i32;
                    ret = bf_round(r, prec, flags);
                    current_block = 7427571413727699167;
                }
            }
        }
        match current_block {
            7427571413727699167 => {}
            _ => {
                bf_set_nan(r);
                return (1 as i32) << 5 as i32;
            }
        }
    }
    match current_block {
        12964998671468006942 => {
            bf_set_nan(r);
            ret = (1 as i32) << 0 as i32
        }
        _ => {}
    }
    return ret;
}
#[inline(never)]
unsafe extern "C" fn bf_op2(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut func: Option<bf_op2_func_t>,
) -> i32 {
    let mut tmp: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut ret: i32 = 0;
    if r == a as *mut bf_t || r == b as *mut bf_t {
        bf_init((*r).ctx, &mut tmp);
        ret = func.expect("non-null function pointer")(&mut tmp, a, b, prec, flags);
        bf_move(r, &mut tmp);
    } else {
        ret = func.expect("non-null function pointer")(r, a, b, prec, flags)
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bf_add(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_op2(
        r,
        a,
        b,
        prec,
        flags,
        Some(
            __bf_add
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: bf_flags_t,
                ) -> i32,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bf_sub(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_op2(
        r,
        a,
        b,
        prec,
        flags,
        Some(
            __bf_sub
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: bf_flags_t,
                ) -> i32,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bf_div(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_op2(
        r,
        a,
        b,
        prec,
        flags,
        Some(
            __bf_div
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: bf_flags_t,
                ) -> i32,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bf_mul_ui(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b1: uint64_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut b: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut ret: i32 = 0;
    bf_init((*r).ctx, &mut b);
    ret = bf_set_ui(&mut b, b1);
    ret |= bf_mul(r, a, &mut b, prec, flags);
    bf_delete(&mut b);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bf_mul_si(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b1: int64_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut b: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut ret: i32 = 0;
    bf_init((*r).ctx, &mut b);
    ret = bf_set_si(&mut b, b1);
    ret |= bf_mul(r, a, &mut b, prec, flags);
    bf_delete(&mut b);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bf_add_si(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b1: int64_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut b: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut ret: i32 = 0;
    bf_init((*r).ctx, &mut b);
    ret = bf_set_si(&mut b, b1);
    ret |= bf_add(r, a, &mut b, prec, flags);
    bf_delete(&mut b);
    return ret;
}
unsafe extern "C" fn bf_pow_ui(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: limb_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut ret: i32 = 0;
    let mut n_bits: i32 = 0;
    let mut i: i32 = 0;
    if r != a as *mut bf_t {
    } else {
        assert!(r as *const bf_t != a);
    }
    if b == 0 as i32 as u64 {
        return bf_set_ui(r, 1 as i32 as uint64_t);
    }
    ret = bf_set(r, a);
    n_bits = ((1 as i32) << 6 as i32) - clz(b);
    i = n_bits - 2 as i32;
    while i >= 0 as i32 {
        ret |= bf_mul(r, r, r, prec, flags);
        if b >> i & 1 as i32 as u64 != 0 {
            ret |= bf_mul(r, r, a, prec, flags)
        }
        i -= 1
    }
    return ret;
}
unsafe extern "C" fn bf_pow_ui_ui(
    mut r: *mut bf_t,
    mut a1: limb_t,
    mut b: limb_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut a: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut ret: i32 = 0;
    if a1 == 10 as i32 as u64 && b <= 19 as i32 as u64 {
        /* use precomputed powers. We do not round at this point
        because we expect the caller to do it */
        ret = bf_set_ui(r, mp_pow_dec[b as usize])
    } else {
        bf_init((*r).ctx, &mut a);
        ret = bf_set_ui(&mut a, a1);
        ret |= bf_pow_ui(r, &mut a, b, prec, flags);
        bf_delete(&mut a);
    }
    return ret;
}
/* convert to integer (infinite precision) */
#[no_mangle]
pub unsafe extern "C" fn bf_rint(mut r: *mut bf_t, mut rnd_mode: i32) -> i32 {
    return bf_round(
        r,
        0 as i32 as limb_t,
        (rnd_mode | (1 as i32) << 4 as i32) as bf_flags_t,
    ); /* minus zero is considered as positive */
}
#[inline]
unsafe extern "C" fn bf_logic_op1(mut a: limb_t, mut b: limb_t, mut op: i32) -> limb_t {
    match op {
        0 => return a | b,
        1 => return a ^ b,
        2 | _ => return a & b,
    }; /* minus zero is considered as positive */
}
unsafe extern "C" fn bf_logic_op(
    mut r: *mut bf_t,
    mut a1: *const bf_t,
    mut b1: *const bf_t,
    mut op: i32,
) -> i32 {
    let mut current_block: u64;
    let mut b1_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut a1_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut a: *mut bf_t = 0 as *mut bf_t;
    let mut b: *mut bf_t = 0 as *mut bf_t;
    let mut a_sign: limb_t = 0;
    let mut b_sign: limb_t = 0;
    let mut r_sign: limb_t = 0;
    let mut l: slimb_t = 0;
    let mut i: slimb_t = 0;
    let mut a_bit_offset: slimb_t = 0;
    let mut b_bit_offset: slimb_t = 0;
    let mut v1: limb_t = 0;
    let mut v2: limb_t = 0;
    let mut v1_mask: limb_t = 0;
    let mut v2_mask: limb_t = 0;
    let mut r_mask: limb_t = 0;
    let mut ret: i32 = 0;
    if r != a1 as *mut bf_t && r != b1 as *mut bf_t {
    } else {
        assert!(r as *const bf_t != a1 && r as *const bf_t != b1);
    }
    if (*a1).expn <= 0 as i32 as i64 {
        a_sign = 0 as i32 as limb_t
    } else {
        a_sign = (*a1).sign as limb_t
    }
    if (*b1).expn <= 0 as i32 as i64 {
        b_sign = 0 as i32 as limb_t
    } else {
        b_sign = (*b1).sign as limb_t
    }
    if a_sign != 0 {
        a = &mut a1_s;
        bf_init((*r).ctx, a);
        if bf_add_si(
            a,
            a1,
            1 as i32 as int64_t,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        ) != 0
        {
            b = 0 as *mut bf_t;
            current_block = 12427457426373845059;
        } else {
            current_block = 12039483399334584727;
        }
    } else {
        a = a1 as *mut bf_t;
        current_block = 12039483399334584727;
    }
    match current_block {
        12039483399334584727 => {
            if b_sign != 0 {
                b = &mut b1_s;
                bf_init((*r).ctx, b);
                if bf_add_si(
                    b,
                    b1,
                    1 as i32 as int64_t,
                    ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                        .wrapping_sub(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64),
                    BF_RNDZ as i32 as bf_flags_t,
                ) != 0
                {
                    current_block = 12427457426373845059;
                } else {
                    current_block = 14576567515993809846;
                }
            } else {
                b = b1 as *mut bf_t;
                current_block = 14576567515993809846;
            }
            match current_block {
                12427457426373845059 => {}
                _ => {
                    r_sign = bf_logic_op1(a_sign, b_sign, op);
                    if op == 2 as i32 && r_sign == 0 as i32 as u64 {
                        /* no need to compute extra zeros for and */
                        if a_sign == 0 as i32 as u64 && b_sign == 0 as i32 as u64 {
                            l = bf_min((*a).expn, (*b).expn)
                        } else if a_sign == 0 as i32 as u64 {
                            l = (*a).expn
                        } else {
                            l = (*b).expn
                        }
                    } else {
                        l = bf_max((*a).expn, (*b).expn)
                    }
                    /* Note: a or b can be zero */
                    l = (bf_max(l, 1 as i32 as slimb_t) + ((1 as i32) << 6 as i32) as i64
                        - 1 as i32 as i64)
                        / ((1 as i32) << 6 as i32) as i64; /* cannot fail */
                    if bf_resize(r, l as limb_t) != 0 {
                        current_block = 12427457426373845059;
                    } else {
                        a_bit_offset = (*a)
                            .len
                            .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                            .wrapping_sub((*a).expn as u64)
                            as slimb_t;
                        b_bit_offset = (*b)
                            .len
                            .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                            .wrapping_sub((*b).expn as u64)
                            as slimb_t;
                        v1_mask = a_sign.wrapping_neg();
                        v2_mask = b_sign.wrapping_neg();
                        r_mask = r_sign.wrapping_neg();
                        i = 0 as i32 as slimb_t;
                        while i < l {
                            v1 = get_bits(
                                (*a).tab,
                                (*a).len,
                                a_bit_offset + i * ((1 as i32) << 6 as i32) as i64,
                            ) ^ v1_mask;
                            v2 = get_bits(
                                (*b).tab,
                                (*b).len,
                                b_bit_offset + i * ((1 as i32) << 6 as i32) as i64,
                            ) ^ v2_mask;
                            *(*r).tab.offset(i as isize) = bf_logic_op1(v1, v2, op) ^ r_mask;
                            i += 1
                        }
                        (*r).expn = l * ((1 as i32) << 6 as i32) as i64;
                        (*r).sign = r_sign as i32;
                        bf_normalize_and_round(
                            r,
                            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                                .wrapping_sub(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64),
                            BF_RNDZ as i32 as bf_flags_t,
                        );
                        if r_sign != 0 {
                            if bf_add_si(
                                r,
                                r,
                                -(1 as i32) as int64_t,
                                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                                    .wrapping_sub(2 as i32 as u64)
                                    .wrapping_add(1 as i32 as u64),
                                BF_RNDZ as i32 as bf_flags_t,
                            ) != 0
                            {
                                current_block = 12427457426373845059;
                            } else {
                                current_block = 2989495919056355252;
                            }
                        } else {
                            current_block = 2989495919056355252;
                        }
                        match current_block {
                            12427457426373845059 => {}
                            _ => {
                                ret = 0 as i32;
                                current_block = 14390628953428984517;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        12427457426373845059 => {
            bf_set_nan(r);
            ret = (1 as i32) << 5 as i32
        }
        _ => {}
    }
    if a == &mut a1_s as *mut bf_t {
        bf_delete(a);
    }
    if b == &mut b1_s as *mut bf_t {
        bf_delete(b);
    }
    return ret;
}
/* 'a' and 'b' must be integers. Return 0 or BF_ST_MEM_ERROR. */
#[no_mangle]
pub unsafe extern "C" fn bf_logic_or(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
) -> i32 {
    return bf_logic_op(r, a, b, 0 as i32);
}
/* 'a' and 'b' must be integers. Return 0 or BF_ST_MEM_ERROR. */
#[no_mangle]
pub unsafe extern "C" fn bf_logic_xor(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
) -> i32 {
    return bf_logic_op(r, a, b, 1 as i32);
}
/* 'a' and 'b' must be integers. Return 0 or BF_ST_MEM_ERROR. */
#[no_mangle]
pub unsafe extern "C" fn bf_logic_and(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut b: *const bf_t,
) -> i32 {
    return bf_logic_op(r, a, b, 2 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn bf_get_float64(
    mut a: *const bf_t,
    mut pres: *mut f64,
    mut rnd_mode: bf_rnd_t,
) -> i32 {
    let mut u: Float64Union = Float64Union { d: 0. };
    let mut e: i32 = 0;
    let mut ret: i32 = 0;
    let mut m: uint64_t = 0;
    ret = 0 as i32;
    if (*a).expn == 9223372036854775807 as i64 {
        u.u = 0x7ff8000000000000 as i64 as uint64_t
    /* quiet nan */
    } else {
        let mut b_s: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut b: *mut bf_t = &mut b_s;
        bf_init((*a).ctx, b);
        bf_set(b, a);
        if bf_is_finite(b) != 0 {
            ret = bf_round(
                b,
                53 as i32 as limb_t,
                rnd_mode as u32 | ((1 as i32) << 3 as i32) as u32 | bf_set_exp_bits(11 as i32),
            )
        }
        if (*b).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            e = ((1 as i32) << 11 as i32) - 1 as i32;
            m = 0 as i32 as uint64_t
        } else if (*b).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
            e = 0 as i32;
            m = 0 as i32 as uint64_t
        } else {
            e = ((*b).expn + 1023 as i32 as i64 - 1 as i32 as i64) as i32;
            m = *(*b).tab.offset(0 as i32 as isize);
            if e <= 0 as i32 {
                /* subnormal */
                m = m >> 12 as i32 - e;
                e = 0 as i32
            } else {
                m = m << 1 as i32 >> 12 as i32
            }
        }
        u.u = m | (e as uint64_t) << 52 as i32 | ((*b).sign as uint64_t) << 63 as i32;
        bf_delete(b);
    }
    *pres = u.d;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bf_set_float64(mut a: *mut bf_t, mut d: f64) -> i32 {
    let mut current_block: u64;
    let mut u: Float64Union = Float64Union { d: 0. };
    let mut m: uint64_t = 0;
    let mut shift: i32 = 0;
    let mut e: i32 = 0;
    let mut sgn: i32 = 0;
    u.d = d;
    sgn = (u.u >> 63 as i32) as i32;
    e = (u.u >> 52 as i32 & (((1 as i32) << 11 as i32) - 1 as i32) as u64) as i32;
    m = u.u & ((1 as i32 as uint64_t) << 52 as i32).wrapping_sub(1 as i32 as u64);
    if e == ((1 as i32) << 11 as i32) - 1 as i32 {
        if m != 0 as i32 as u64 {
            bf_set_nan(a);
        } else {
            bf_set_inf(a, sgn);
        }
    } else {
        if e == 0 as i32 {
            if m == 0 as i32 as u64 {
                bf_set_zero(a, sgn);
                current_block = 9828876828309294594;
            } else {
                /* subnormal number */
                m <<= 12 as i32;
                shift = clz64(m);
                m <<= shift;
                e = -shift;
                current_block = 15442955482004205486;
            }
        } else {
            m = m << 11 as i32 | (1 as i32 as uint64_t) << 63 as i32;
            current_block = 15442955482004205486;
        }
        match current_block {
            9828876828309294594 => {}
            _ => {
                (*a).expn = (e - 1023 as i32 + 1 as i32) as slimb_t;
                if bf_resize(a, 1 as i32 as limb_t) != 0 {
                    bf_set_nan(a);
                    return (1 as i32) << 5 as i32;
                } else {
                    *(*a).tab.offset(0 as i32 as isize) = m;
                    (*a).sign = sgn
                }
            }
        }
    }
    return 0 as i32;
}
/* The rounding mode is always BF_RNDZ. Return BF_ST_INVALID_OP if there
is an overflow and 0 otherwise. */
#[no_mangle]
pub unsafe extern "C" fn bf_get_int32(
    mut pres: *mut i32,
    mut a: *const bf_t,
    mut flags: i32,
) -> i32 {
    let mut v: uint32_t = 0;
    let mut ret: i32 = 0;
    if (*a).expn >= 9223372036854775807 as i64 - 1 as i32 as i64 {
        ret = (1 as i32) << 0 as i32;
        if flags & (1 as i32) << 0 as i32 != 0 {
            v = 0 as i32 as uint32_t
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            v = (2147483647 as i32 as uint32_t).wrapping_add((*a).sign as u32)
        } else {
            v = 2147483647 as i32 as uint32_t
        }
    } else if (*a).expn <= 0 as i32 as i64 {
        v = 0 as i32 as uint32_t;
        ret = 0 as i32
    } else if (*a).expn <= 31 as i32 as i64 {
        v = (*(*a)
            .tab
            .offset((*a).len.wrapping_sub(1 as i32 as u64) as isize)
            >> ((1 as i32) << 6 as i32) as i64 - (*a).expn) as uint32_t;
        if (*a).sign != 0 {
            v = v.wrapping_neg()
        }
        ret = 0 as i32
    } else if flags & (1 as i32) << 0 as i32 == 0 {
        ret = (1 as i32) << 0 as i32;
        if (*a).sign != 0 {
            v = (2147483647 as i32 as uint32_t).wrapping_add(1 as i32 as u32);
            if (*a).expn == 32 as i32 as i64
                && *(*a)
                    .tab
                    .offset((*a).len.wrapping_sub(1 as i32 as u64) as isize)
                    >> ((1 as i32) << 6 as i32) - 32 as i32
                    == v as u64
            {
                ret = 0 as i32
            }
        } else {
            v = 2147483647 as i32 as uint32_t
        }
    } else {
        v = get_bits(
            (*a).tab,
            (*a).len,
            (*a).len
                .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                .wrapping_sub((*a).expn as u64) as slimb_t,
        ) as uint32_t;
        if (*a).sign != 0 {
            v = v.wrapping_neg()
        }
        ret = 0 as i32
    }
    *pres = v as i32;
    return ret;
}
/* The rounding mode is always BF_RNDZ. Return BF_ST_INVALID_OP if there
is an overflow and 0 otherwise. */
#[no_mangle]
pub unsafe extern "C" fn bf_get_int64(
    mut pres: *mut int64_t,
    mut a: *const bf_t,
    mut flags: i32,
) -> i32 {
    let mut v: uint64_t = 0;
    let mut ret: i32 = 0;
    if (*a).expn >= 9223372036854775807 as i64 - 1 as i32 as i64 {
        ret = (1 as i32) << 0 as i32;
        if flags & (1 as i32) << 0 as i32 != 0 {
            v = 0 as i32 as uint64_t
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            v = (9223372036854775807 as i64 as uint64_t).wrapping_add((*a).sign as u64)
        } else {
            v = 9223372036854775807 as i64 as uint64_t
        }
    } else if (*a).expn <= 0 as i32 as i64 {
        v = 0 as i32 as uint64_t;
        ret = 0 as i32
    } else if (*a).expn <= 63 as i32 as i64 {
        v = *(*a)
            .tab
            .offset((*a).len.wrapping_sub(1 as i32 as u64) as isize)
            >> ((1 as i32) << 6 as i32) as i64 - (*a).expn;
        if (*a).sign != 0 {
            v = v.wrapping_neg()
        }
        ret = 0 as i32
    } else if flags & (1 as i32) << 0 as i32 == 0 {
        ret = (1 as i32) << 0 as i32;
        if (*a).sign != 0 {
            let mut v1: uint64_t = 0;
            v = (9223372036854775807 as i64 as uint64_t).wrapping_add(1 as i32 as u64);
            if (*a).expn == 64 as i32 as i64 {
                v1 = *(*a)
                    .tab
                    .offset((*a).len.wrapping_sub(1 as i32 as u64) as isize);
                if v1 == v {
                    ret = 0 as i32
                }
            }
        } else {
            v = 9223372036854775807 as i64 as uint64_t
        }
    } else {
        let mut bit_pos: slimb_t = (*a)
            .len
            .wrapping_mul(((1 as i32) << 6 as i32) as u64)
            .wrapping_sub((*a).expn as u64) as slimb_t;
        v = get_bits((*a).tab, (*a).len, bit_pos);
        if (*a).sign != 0 {
            v = v.wrapping_neg()
        }
        ret = 0 as i32
    }
    *pres = v as int64_t;
    return ret;
}
/* The rounding mode is always BF_RNDZ. Return BF_ST_INVALID_OP if there
is an overflow and 0 otherwise. */
#[no_mangle]
pub unsafe extern "C" fn bf_get_uint64(mut pres: *mut uint64_t, mut a: *const bf_t) -> i32 {
    let mut v: uint64_t = 0;
    let mut ret: i32 = 0;
    let mut current_block_10: u64;
    if (*a).expn == 9223372036854775807 as i64 {
        current_block_10 = 8344853031916340450;
    } else if (*a).expn <= 0 as i32 as i64 {
        v = 0 as i32 as uint64_t;
        ret = 0 as i32;
        current_block_10 = 17407779659766490442;
    } else if (*a).sign != 0 {
        v = 0 as i32 as uint64_t;
        ret = (1 as i32) << 0 as i32;
        current_block_10 = 17407779659766490442;
    } else if (*a).expn <= 64 as i32 as i64 {
        v = *(*a)
            .tab
            .offset((*a).len.wrapping_sub(1 as i32 as u64) as isize)
            >> ((1 as i32) << 6 as i32) as i64 - (*a).expn;
        ret = 0 as i32;
        current_block_10 = 17407779659766490442;
    } else {
        current_block_10 = 8344853031916340450;
    }
    match current_block_10 {
        8344853031916340450 => {
            v = 18446744073709551615 as u64;
            ret = (1 as i32) << 0 as i32
        }
        _ => {}
    }
    *pres = v;
    return ret;
}
/* base conversion from radix */
static mut digits_per_limb_table: [uint8_t; 35] = [
    64 as i32 as uint8_t,
    40 as i32 as uint8_t,
    32 as i32 as uint8_t,
    27 as i32 as uint8_t,
    24 as i32 as uint8_t,
    22 as i32 as uint8_t,
    21 as i32 as uint8_t,
    20 as i32 as uint8_t,
    19 as i32 as uint8_t,
    18 as i32 as uint8_t,
    17 as i32 as uint8_t,
    17 as i32 as uint8_t,
    16 as i32 as uint8_t,
    16 as i32 as uint8_t,
    16 as i32 as uint8_t,
    15 as i32 as uint8_t,
    15 as i32 as uint8_t,
    15 as i32 as uint8_t,
    14 as i32 as uint8_t,
    14 as i32 as uint8_t,
    14 as i32 as uint8_t,
    14 as i32 as uint8_t,
    13 as i32 as uint8_t,
    13 as i32 as uint8_t,
    13 as i32 as uint8_t,
    13 as i32 as uint8_t,
    13 as i32 as uint8_t,
    13 as i32 as uint8_t,
    13 as i32 as uint8_t,
    12 as i32 as uint8_t,
    12 as i32 as uint8_t,
    12 as i32 as uint8_t,
    12 as i32 as uint8_t,
    12 as i32 as uint8_t,
    12 as i32 as uint8_t,
];
unsafe extern "C" fn get_limb_radix(mut radix: i32) -> limb_t {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut radixl: limb_t = 0;
    k = digits_per_limb_table[(radix - 2 as i32) as usize] as i32;
    radixl = radix as limb_t;
    i = 1 as i32;
    while i < k {
        radixl = (radixl as u64).wrapping_mul(radix as u64) as limb_t as limb_t;
        i += 1
    }
    return radixl;
}
/* return != 0 if error */
unsafe extern "C" fn bf_integer_from_radix_rec(
    mut r: *mut bf_t,
    mut tab: *const limb_t,
    mut n: limb_t,
    mut level: i32,
    mut n0: limb_t,
    mut radix: limb_t,
    mut pow_tab: *mut bf_t,
) -> i32 {
    let mut ret: i32 = 0;
    if n == 1 as i32 as u64 {
        ret = bf_set_ui(r, *tab.offset(0 as i32 as isize))
    } else {
        let mut T_s: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut T: *mut bf_t = &mut T_s;
        let mut B: *mut bf_t = 0 as *mut bf_t;
        let mut n1: limb_t = 0;
        let mut n2: limb_t = 0;
        n2 = (n0.wrapping_mul(2 as i32 as u64) >> level + 1 as i32)
            .wrapping_add(1 as i32 as u64)
            .wrapping_div(2 as i32 as u64);
        n1 = n.wrapping_sub(n2);
        //        printf("level=%d n0=%ld n1=%ld n2=%ld\n", level, n0, n1, n2);
        B = &mut *pow_tab.offset(level as isize) as *mut bf_t;
        if (*B).len == 0 as i32 as u64 {
            ret = bf_pow_ui_ui(
                B,
                radix,
                n2,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                BF_RNDZ as i32 as bf_flags_t,
            );
            if ret != 0 {
                return ret;
            }
        }
        ret = bf_integer_from_radix_rec(
            r,
            tab.offset(n2 as isize),
            n1,
            level + 1 as i32,
            n0,
            radix,
            pow_tab,
        );
        if ret != 0 {
            return ret;
        }
        ret = bf_mul(
            r,
            r,
            B,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        if ret != 0 {
            return ret;
        }
        bf_init((*r).ctx, T);
        ret = bf_integer_from_radix_rec(T, tab, n2, level + 1 as i32, n0, radix, pow_tab);
        if ret == 0 {
            ret = bf_add(
                r,
                r,
                T,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                BF_RNDZ as i32 as bf_flags_t,
            )
        }
        bf_delete(T);
    }
    return ret;
    //    bf_print_str("  r=", r);
}
/* return 0 if OK != 0 if memory error */
unsafe extern "C" fn bf_integer_from_radix(
    mut r: *mut bf_t,
    mut tab: *const limb_t,
    mut n: limb_t,
    mut radix: limb_t,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx; /* XXX: check */
    let mut pow_tab_len: i32 = 0;
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    let mut radixl: limb_t = 0;
    let mut pow_tab: *mut bf_t = 0 as *mut bf_t;
    radixl = get_limb_radix(radix as i32);
    pow_tab_len = ceil_log2(n) + 2 as i32;
    pow_tab = bf_malloc(
        s,
        (::std::mem::size_of::<bf_t>() as u64).wrapping_mul(pow_tab_len as u64),
    ) as *mut bf_t;
    if pow_tab.is_null() {
        return -(1 as i32);
    }
    i = 0 as i32;
    while i < pow_tab_len {
        bf_init((*r).ctx, &mut *pow_tab.offset(i as isize));
        i += 1
    }
    ret = bf_integer_from_radix_rec(r, tab, n, 0 as i32, n, radixl, pow_tab);
    i = 0 as i32;
    while i < pow_tab_len {
        bf_delete(&mut *pow_tab.offset(i as isize));
        i += 1
    }
    bf_free(s, pow_tab as *mut std::ffi::c_void);
    return ret;
}
/* compute and round T * radix^expn. */
#[no_mangle]
pub unsafe extern "C" fn bf_mul_pow_radix(
    mut r: *mut bf_t,
    mut T: *const bf_t,
    mut radix: limb_t,
    mut expn: slimb_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut ret: i32 = 0;
    let mut expn_sign: i32 = 0;
    let mut overflow: i32 = 0;
    let mut e: slimb_t = 0;
    let mut extra_bits: slimb_t = 0;
    let mut prec1: slimb_t = 0;
    let mut ziv_extra_bits: slimb_t = 0;
    let mut B_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut B: *mut bf_t = &mut B_s;
    if (*T).len == 0 as i32 as u64 {
        return bf_set(r, T);
    } else {
        if expn == 0 as i32 as i64 {
            ret = bf_set(r, T);
            ret |= bf_round(r, prec, flags);
            return ret;
        }
    }
    e = expn;
    expn_sign = 0 as i32;
    if e < 0 as i32 as i64 {
        e = -e;
        expn_sign = 1 as i32
    }
    bf_init((*r).ctx, B);
    if prec
        == ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64)
    {
        /* infinite precision: only used if the result is known to be exact */
        ret = bf_pow_ui_ui(
            B,
            radix,
            e as limb_t,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDN as i32 as bf_flags_t,
        );
        if expn_sign != 0 {
            ret |= bf_div(
                r,
                T,
                B,
                (*T).len.wrapping_mul(((1 as i32) << 6 as i32) as u64),
                BF_RNDN as i32 as bf_flags_t,
            )
        } else {
            ret |= bf_mul(
                r,
                T,
                B,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                BF_RNDN as i32 as bf_flags_t,
            )
        }
    } else {
        ziv_extra_bits = 16 as i32 as slimb_t;
        loop {
            prec1 = prec.wrapping_add(ziv_extra_bits as u64) as slimb_t;
            /* XXX: correct overflow/underflow handling */
            /* XXX: rigorous error analysis needed */
            extra_bits = (ceil_log2(e as limb_t) * 2 as i32 + 1 as i32) as slimb_t;
            ret = bf_pow_ui_ui(
                B,
                radix,
                e as limb_t,
                (prec1 + extra_bits) as limb_t,
                (BF_RNDN as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
            );
            overflow = (bf_is_finite(B) == 0) as i32;
            /* XXX: if bf_pow_ui_ui returns an exact result, can stop
            after the next operation */
            if expn_sign != 0 {
                ret |= bf_div(
                    r,
                    T,
                    B,
                    (prec1 + extra_bits) as limb_t,
                    (BF_RNDN as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
                )
            } else {
                ret |= bf_mul(
                    r,
                    T,
                    B,
                    (prec1 + extra_bits) as limb_t,
                    (BF_RNDN as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
                )
            }
            if ret & (1 as i32) << 5 as i32 != 0 {
                break;
            }
            if ret & (1 as i32) << 4 as i32 != 0
                && bf_can_round(
                    r,
                    prec as slimb_t,
                    (flags & 0x7 as i32 as u32) as bf_rnd_t,
                    prec1,
                ) == 0
                && overflow == 0
            {
                /* and more precision and retry */
                ziv_extra_bits = ziv_extra_bits + ziv_extra_bits / 2 as i32 as i64
            } else {
                /* XXX: need to use __bf_round() to pass the inexact
                flag for the subnormal case */
                ret = bf_round(r, prec, flags) | ret & (1 as i32) << 4 as i32;
                break;
            }
        }
    }
    bf_delete(B);
    return ret;
}
#[inline]
unsafe extern "C" fn to_digit(mut c: i32) -> i32 {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    } else if c >= 'A' as i32 && c <= 'Z' as i32 {
        return c - 'A' as i32 + 10 as i32;
    } else if c >= 'a' as i32 && c <= 'z' as i32 {
        return c - 'a' as i32 + 10 as i32;
    } else {
        return 36 as i32;
    };
}
/* add a limb at 'pos' and decrement pos. new space is created if
needed. Return 0 if OK, -1 if memory error */
unsafe extern "C" fn bf_add_limb(mut a: *mut bf_t, mut ppos: *mut slimb_t, mut v: limb_t) -> i32 {
    let mut pos: slimb_t = 0;
    pos = *ppos;
    if (pos < 0 as i32 as i64) as i32 as i64 != 0 {
        let mut new_size: limb_t = 0;
        let mut d: limb_t = 0;
        let mut new_tab: *mut limb_t = 0 as *mut limb_t;
        new_size = bf_max(
            (*a).len.wrapping_add(1 as i32 as u64) as slimb_t,
            (*a).len
                .wrapping_mul(3 as i32 as u64)
                .wrapping_div(2 as i32 as u64) as slimb_t,
        ) as limb_t;
        new_tab = bf_realloc(
            (*a).ctx,
            (*a).tab as *mut std::ffi::c_void,
            (::std::mem::size_of::<limb_t>() as u64).wrapping_mul(new_size),
        ) as *mut limb_t;
        if new_tab.is_null() {
            return -(1 as i32);
        }
        (*a).tab = new_tab;
        d = new_size.wrapping_sub((*a).len);
        ((*a).tab.offset(d as isize) as *mut u8).copy_from_nonoverlapping(
            (*a).tab as *const u8,
            ((*a).len as usize).wrapping_mul(std::mem::size_of::<limb_t>()),
        );
        (*a).len = new_size;
        pos = (pos as u64).wrapping_add(d) as slimb_t as slimb_t
    }
    let fresh7 = pos;
    pos = pos - 1;
    *(*a).tab.offset(fresh7 as isize) = v;
    *ppos = pos;
    return 0 as i32;
}
unsafe extern "C" fn bf_tolower(mut c: i32) -> i32 {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        c = c - 'A' as i32 + 'a' as i32
    }
    return c;
}
unsafe extern "C" fn strcasestart(
    mut str: *const std::os::raw::c_char,
    mut val: *const std::os::raw::c_char,
    mut ptr: *mut *const std::os::raw::c_char,
) -> i32 {
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut q: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    p = str;
    q = val;
    while *q as i32 != '\u{0}' as i32 {
        if bf_tolower(*p as i32) != *q as i32 {
            return 0 as i32;
        }
        p = p.offset(1);
        q = q.offset(1)
    }
    if !ptr.is_null() {
        *ptr = p
    }
    return 1 as i32;
}
unsafe extern "C" fn bf_atof_internal(
    mut r: *mut bf_t,
    mut pexponent: *mut slimb_t,
    mut str: *const std::os::raw::c_char,
    mut pnext: *mut *const std::os::raw::c_char,
    mut radix: i32,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut is_dec: BOOL,
) -> i32 {
    let mut current_block: u64;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut p_start: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut is_neg: i32 = 0;
    let mut radix_bits: i32 = 0;
    let mut exp_is_neg: i32 = 0;
    let mut ret: i32 = 0;
    let mut digits_per_limb: i32 = 0;
    let mut shift: i32 = 0;
    let mut cur_limb: limb_t = 0;
    let mut pos: slimb_t = 0;
    let mut expn: slimb_t = 0;
    let mut int_len: slimb_t = 0;
    let mut digit_count: slimb_t = 0;
    let mut has_decpt: BOOL = 0;
    let mut is_bin_exp: BOOL = 0;
    let mut a_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut a: *mut bf_t = 0 as *mut bf_t;
    *pexponent = 0 as i32 as slimb_t;
    p = str;
    if flags & ((1 as i32) << 18 as i32) as u32 == 0
        && radix <= 16 as i32
        && strcasestart(
            p,
            b"nan\x00" as *const u8 as *const std::os::raw::c_char,
            &mut p,
        ) != 0
    {
        bf_set_nan(r);
        ret = 0 as i32
    } else {
        is_neg = 0 as i32;
        if *p.offset(0 as i32 as isize) as i32 == '+' as i32 {
            p = p.offset(1);
            p_start = p
        } else if *p.offset(0 as i32 as isize) as i32 == '-' as i32 {
            is_neg = 1 as i32;
            p = p.offset(1);
            p_start = p
        } else {
            p_start = p
        }
        if *p.offset(0 as i32 as isize) as i32 == '0' as i32 {
            if (*p.offset(1 as i32 as isize) as i32 == 'x' as i32
                || *p.offset(1 as i32 as isize) as i32 == 'X' as i32)
                && (radix == 0 as i32 || radix == 16 as i32)
                && flags & ((1 as i32) << 16 as i32) as u32 == 0
            {
                radix = 16 as i32;
                p = p.offset(2 as i32 as isize);
                current_block = 2569451025026770673;
            } else if (*p.offset(1 as i32 as isize) as i32 == 'o' as i32
                || *p.offset(1 as i32 as isize) as i32 == 'O' as i32)
                && radix == 0 as i32
                && flags & ((1 as i32) << 17 as i32) as u32 != 0
            {
                p = p.offset(2 as i32 as isize);
                radix = 8 as i32;
                current_block = 2569451025026770673;
            } else if (*p.offset(1 as i32 as isize) as i32 == 'b' as i32
                || *p.offset(1 as i32 as isize) as i32 == 'B' as i32)
                && radix == 0 as i32
                && flags & ((1 as i32) << 17 as i32) as u32 != 0
            {
                p = p.offset(2 as i32 as isize);
                radix = 2 as i32;
                current_block = 2569451025026770673;
            } else {
                current_block = 3934796541983872331;
            }
            match current_block {
                3934796541983872331 => {}
                _ =>
                /* there must be a digit after the prefix */
                {
                    if to_digit(*p as uint8_t as i32) >= radix {
                        bf_set_nan(r); /* base is not a power of two */
                        ret = 0 as i32;
                        current_block = 11118440404757757489;
                    } else {
                        current_block = 3934796541983872331;
                    }
                }
            }
        } else if flags & ((1 as i32) << 18 as i32) as u32 == 0
            && radix <= 16 as i32
            && strcasestart(
                p,
                b"inf\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
        {
            bf_set_inf(r, is_neg);
            ret = 0 as i32;
            current_block = 11118440404757757489;
        } else {
            current_block = 3934796541983872331;
        }
        match current_block {
            11118440404757757489 => {}
            _ => {
                if radix == 0 as i32 {
                    radix = 10 as i32
                }
                if is_dec != 0 {
                    if radix == 10 as i32 {
                    } else {
                        assert!(radix == 10);
                    }
                    radix_bits = 0 as i32;
                    a = r
                } else if radix & radix - 1 as i32 != 0 as i32 {
                    radix_bits = 0 as i32;
                    a = &mut a_s;
                    bf_init((*r).ctx, a);
                } else {
                    radix_bits = ceil_log2(radix as limb_t);
                    a = r
                }
                /* skip leading zeros */
                /* XXX: could also skip zeros after the decimal point */
                while *p as i32 == '0' as i32 {
                    p = p.offset(1)
                }
                if radix_bits != 0 {
                    digits_per_limb = (1 as i32) << 6 as i32;
                    shift = digits_per_limb
                } else {
                    radix_bits = 0 as i32;
                    digits_per_limb = digits_per_limb_table[(radix - 2 as i32) as usize] as i32;
                    shift = digits_per_limb
                }
                cur_limb = 0 as i32 as limb_t;
                bf_resize(a, 1 as i32 as limb_t);
                pos = 0 as i32 as slimb_t;
                has_decpt = FALSE as i32;
                digit_count = 0 as i32 as slimb_t;
                int_len = digit_count;
                loop {
                    let mut c: limb_t = 0;
                    if *p as i32 == '.' as i32
                        && (p > p_start || to_digit(*p.offset(1 as i32 as isize) as i32) < radix)
                    {
                        if has_decpt != 0 {
                            current_block = 6014157347423944569;
                            break;
                        }
                        has_decpt = TRUE as i32;
                        int_len = digit_count;
                        p = p.offset(1)
                    }
                    c = to_digit(*p as i32) as limb_t;
                    if c >= radix as u64 {
                        current_block = 6014157347423944569;
                        break;
                    }
                    digit_count += 1;
                    p = p.offset(1);
                    if radix_bits != 0 {
                        shift -= radix_bits;
                        if shift <= 0 as i32 {
                            cur_limb |= c >> -shift;
                            if bf_add_limb(a, &mut pos, cur_limb) != 0 {
                                current_block = 2830896818224579846;
                                break;
                            }
                            if shift < 0 as i32 {
                                cur_limb = c << ((1 as i32) << 6 as i32) + shift
                            } else {
                                cur_limb = 0 as i32 as limb_t
                            }
                            shift += (1 as i32) << 6 as i32
                        } else {
                            cur_limb |= c << shift
                        }
                    } else {
                        cur_limb = cur_limb.wrapping_mul(radix as u64).wrapping_add(c);
                        shift -= 1;
                        if !(shift == 0 as i32) {
                            continue;
                        }
                        if bf_add_limb(a, &mut pos, cur_limb) != 0 {
                            current_block = 2830896818224579846;
                            break;
                        }
                        shift = digits_per_limb;
                        cur_limb = 0 as i32 as limb_t
                    }
                }
                match current_block {
                    6014157347423944569 => {
                        if has_decpt == 0 {
                            int_len = digit_count
                        }
                        /* add the last limb and pad with zeros */
                        if shift != digits_per_limb {
                            if radix_bits == 0 as i32 {
                                while shift != 0 as i32 {
                                    cur_limb = (cur_limb as u64).wrapping_mul(radix as u64)
                                        as limb_t
                                        as limb_t;
                                    shift -= 1
                                }
                            }
                            if bf_add_limb(a, &mut pos, cur_limb) != 0 {
                                current_block = 2830896818224579846;
                            } else {
                                current_block = 7639320476250304355;
                            }
                        } else {
                            current_block = 7639320476250304355;
                        }
                        match current_block {
                            2830896818224579846 => {}
                            _ => {
                                /* reset the next limbs to zero (we prefer to reallocate in the
                                renormalization) */
                                ((*a).tab as *mut u8).write_bytes(
                                    0,
                                    (pos as usize + 1).wrapping_mul(std::mem::size_of::<limb_t>()),
                                );
                                if p == p_start {
                                    ret = 0 as i32;
                                    if radix_bits == 0 {
                                        bf_delete(a);
                                    }
                                    bf_set_nan(r);
                                } else {
                                    /* parse the exponent, if any */
                                    expn = 0 as i32 as slimb_t;
                                    is_bin_exp = FALSE as i32;
                                    if (radix == 10 as i32
                                        && (*p as i32 == 'e' as i32 || *p as i32 == 'E' as i32)
                                        || radix != 10 as i32
                                            && (*p as i32 == '@' as i32
                                                || radix_bits != 0
                                                    && (*p as i32 == 'p' as i32
                                                        || *p as i32 == 'P' as i32)))
                                        && p > p_start
                                    {
                                        is_bin_exp = (*p as i32 == 'p' as i32
                                            || *p as i32 == 'P' as i32)
                                            as i32;
                                        p = p.offset(1);
                                        exp_is_neg = 0 as i32;
                                        if *p as i32 == '+' as i32 {
                                            p = p.offset(1)
                                        } else if *p as i32 == '-' as i32 {
                                            exp_is_neg = 1 as i32;
                                            p = p.offset(1)
                                        }
                                        loop {
                                            let mut c_0: i32 = 0;
                                            c_0 = to_digit(*p as i32);
                                            if c_0 >= 10 as i32 {
                                                current_block = 5089124893069931607;
                                                break;
                                            }
                                            if (expn
                                                > (9223372036854775807 as i64
                                                    - 2 as i32 as i64
                                                    - 9 as i32 as i64)
                                                    / 10 as i32 as i64)
                                                as i32
                                                as i64
                                                != 0
                                            {
                                                /* exponent overflow */
                                                if exp_is_neg != 0 {
                                                    bf_set_zero(r, is_neg);
                                                    ret = (1 as i32) << 3 as i32
                                                        | (1 as i32) << 4 as i32
                                                } else {
                                                    bf_set_inf(r, is_neg);
                                                    ret = (1 as i32) << 2 as i32
                                                        | (1 as i32) << 4 as i32
                                                }
                                                current_block = 11118440404757757489;
                                                break;
                                            } else {
                                                p = p.offset(1);
                                                expn = expn * 10 as i32 as i64 + c_0 as i64
                                            }
                                        }
                                        match current_block {
                                            11118440404757757489 => {}
                                            _ => {
                                                if exp_is_neg != 0 {
                                                    expn = -expn
                                                }
                                                current_block = 11227437541145425351;
                                            }
                                        }
                                    } else {
                                        current_block = 11227437541145425351;
                                    }
                                    match current_block {
                                        11118440404757757489 => {}
                                        _ => {
                                            if is_dec != 0 {
                                                (*a).expn = expn + int_len;
                                                (*a).sign = is_neg;
                                                ret = bfdec_normalize_and_round(
                                                    a as *mut bfdec_t,
                                                    prec,
                                                    flags,
                                                )
                                            } else if radix_bits != 0 {
                                                /* XXX: may overflow */
                                                if is_bin_exp == 0 {
                                                    expn *= radix_bits as i64
                                                } /* number of limbs */
                                                (*a).expn = expn + int_len * radix_bits as i64;
                                                (*a).sign = is_neg;
                                                ret = bf_normalize_and_round(a, prec, flags)
                                            } else {
                                                let mut l: limb_t = 0;
                                                pos += 1;
                                                l = (*a).len.wrapping_sub(pos as u64);
                                                if l == 0 as i32 as u64 {
                                                    bf_set_zero(r, is_neg);
                                                    ret = 0 as i32
                                                } else {
                                                    let mut T_s: bf_t = bf_t {
                                                        ctx: 0 as *mut bf_context_t,
                                                        sign: 0,
                                                        expn: 0,
                                                        len: 0,
                                                        tab: 0 as *mut limb_t,
                                                    };
                                                    let mut T: *mut bf_t = &mut T_s;
                                                    expn = (expn as u64).wrapping_sub(
                                                        l.wrapping_mul(digits_per_limb as u64)
                                                            .wrapping_sub(int_len as u64),
                                                    )
                                                        as slimb_t
                                                        as slimb_t;
                                                    bf_init((*r).ctx, T);
                                                    if bf_integer_from_radix(
                                                        T,
                                                        (*a).tab.offset(pos as isize),
                                                        l,
                                                        radix as limb_t,
                                                    ) != 0
                                                    {
                                                        bf_set_nan(r);
                                                        ret = (1 as i32) << 5 as i32
                                                    } else {
                                                        (*T).sign = is_neg;
                                                        if flags & ((1 as i32) << 19 as i32) as u32
                                                            != 0
                                                        {
                                                            /* return the exponent */
                                                            *pexponent = expn;
                                                            ret = bf_set(r, T)
                                                        } else {
                                                            ret = bf_mul_pow_radix(
                                                                r,
                                                                T,
                                                                radix as limb_t,
                                                                expn,
                                                                prec,
                                                                flags,
                                                            )
                                                        }
                                                    }
                                                    bf_delete(T);
                                                }
                                                bf_delete(a);
                                            }
                                        }
                                    }
                                }
                                current_block = 11118440404757757489;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    11118440404757757489 => {}
                    _ => {
                        ret = (1 as i32) << 5 as i32;
                        if radix_bits == 0 {
                            bf_delete(a);
                        }
                        bf_set_nan(r);
                    }
                }
            }
        }
    }
    if !pnext.is_null() {
        *pnext = p
    }
    return ret;
}
/*
   Return (status, n, exp). 'status' is the floating point status. 'n'
   is the parsed number.

   If (flags & BF_ATOF_EXPONENT) and if the radix is not a power of
   two, the parsed number is equal to r *
   (*pexponent)^radix. Otherwise *pexponent = 0.
*/
#[no_mangle]
pub unsafe extern "C" fn bf_atof2(
    mut r: *mut bf_t,
    mut pexponent: *mut slimb_t,
    mut str: *const std::os::raw::c_char,
    mut pnext: *mut *const std::os::raw::c_char,
    mut radix: i32,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_atof_internal(r, pexponent, str, pnext, radix, prec, flags, FALSE as i32);
}
#[no_mangle]
pub unsafe extern "C" fn bf_atof(
    mut r: *mut bf_t,
    mut str: *const std::os::raw::c_char,
    mut pnext: *mut *const std::os::raw::c_char,
    mut radix: i32,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut dummy_exp: slimb_t = 0;
    return bf_atof_internal(
        r,
        &mut dummy_exp,
        str,
        pnext,
        radix,
        prec,
        flags,
        FALSE as i32,
    );
}
/* base conversion to radix */
static mut inv_log2_radix: [[uint32_t; 3]; 35] = [
    [
        0x80000000 as u32,
        0 as i32 as uint32_t,
        0 as i32 as uint32_t,
    ],
    [
        0x50c24e60 as i32 as uint32_t,
        0xd4d4f4a7 as u32,
        0x21f57bc as i32 as uint32_t,
    ],
    [
        0x40000000 as i32 as uint32_t,
        0 as i32 as uint32_t,
        0 as i32 as uint32_t,
    ],
    [
        0x372068d2 as i32 as uint32_t,
        0xa1ee5ca as i32 as uint32_t,
        0x19ea911b as i32 as uint32_t,
    ],
    [
        0x3184648d as i32 as uint32_t,
        0xb8153e7a as u32,
        0x7fc2d2e1 as i32 as uint32_t,
    ],
    [
        0x2d983275 as i32 as uint32_t,
        0x9d5369c4 as u32,
        0x4dec1661 as i32 as uint32_t,
    ],
    [
        0x2aaaaaaa as i32 as uint32_t,
        0xaaaaaaaa as u32,
        0xaaaaaaab as u32,
    ],
    [
        0x28612730 as i32 as uint32_t,
        0x6a6a7a53 as i32 as uint32_t,
        0x810fabde as u32,
    ],
    [
        0x268826a1 as i32 as uint32_t,
        0x3ef3fde6 as i32 as uint32_t,
        0x23e2566b as i32 as uint32_t,
    ],
    [
        0x25001383 as i32 as uint32_t,
        0xbac8a744 as u32,
        0x385a3349 as i32 as uint32_t,
    ],
    [
        0x23b46706 as i32 as uint32_t,
        0x82c0c709 as u32,
        0x3f891718 as i32 as uint32_t,
    ],
    [
        0x229729f1 as i32 as uint32_t,
        0xb2c83ded as u32,
        0x15fba800 as i32 as uint32_t,
    ],
    [
        0x219e7ffd as i32 as uint32_t,
        0xa5ad572a as u32,
        0xe169744b as u32,
    ],
    [
        0x20c33b88 as i32 as uint32_t,
        0xda7c29aa as u32,
        0x9bddee52 as u32,
    ],
    [
        0x20000000 as i32 as uint32_t,
        0 as i32 as uint32_t,
        0 as i32 as uint32_t,
    ],
    [
        0x1f50b57e as i32 as uint32_t,
        0xac5884b3 as u32,
        0x70e28eee as i32 as uint32_t,
    ],
    [
        0x1eb22cc6 as i32 as uint32_t,
        0x8aa6e26f as u32,
        0x6d1a2a2 as i32 as uint32_t,
    ],
    [
        0x1e21e118 as i32 as uint32_t,
        0xc5daab1 as i32 as uint32_t,
        0x81b4f4bf as u32,
    ],
    [
        0x1d9dcd21 as i32 as uint32_t,
        0x439834e3 as i32 as uint32_t,
        0x81667575 as u32,
    ],
    [
        0x1d244c78 as i32 as uint32_t,
        0x367a0d64 as i32 as uint32_t,
        0xc8204d6d as u32,
    ],
    [
        0x1cb40589 as i32 as uint32_t,
        0xac173e0c as u32,
        0x3b7b16ba as i32 as uint32_t,
    ],
    [
        0x1c4bd95b as i32 as uint32_t,
        0xa8d72b0d as u32,
        0x5879f25a as i32 as uint32_t,
    ],
    [
        0x1bead768 as i32 as uint32_t,
        0x98f8ce4c as u32,
        0x66cc2858 as i32 as uint32_t,
    ],
    [
        0x1b903469 as i32 as uint32_t,
        0x50f72e5 as i32 as uint32_t,
        0xcf5488e as i32 as uint32_t,
    ],
    [
        0x1b3b433f as i32 as uint32_t,
        0x2eb06f14 as i32 as uint32_t,
        0x8c89719c as u32,
    ],
    [
        0x1aeb6f75 as i32 as uint32_t,
        0x9c46fc37 as u32,
        0xab5fc7e9 as u32,
    ],
    [
        0x1aa038eb as i32 as uint32_t,
        0xe3bfd17 as i32 as uint32_t,
        0x1bd62080 as i32 as uint32_t,
    ],
    [
        0x1a593062 as i32 as uint32_t,
        0xb38d8c56 as u32,
        0x7998ab45 as i32 as uint32_t,
    ],
    [
        0x1a15f4c3 as i32 as uint32_t,
        0x2b95a2e6 as i32 as uint32_t,
        0x46aed6a0 as i32 as uint32_t,
    ],
    [
        0x19d630dc as i32 as uint32_t,
        0xcc7ddef9 as u32,
        0x5aadd61b as i32 as uint32_t,
    ],
    [
        0x19999999 as i32 as uint32_t,
        0x99999999 as u32,
        0x9999999a as u32,
    ],
    [
        0x195fec80 as i32 as uint32_t,
        0x8a609430 as u32,
        0xe1106014 as u32,
    ],
    [
        0x1928ee7b as i32 as uint32_t,
        0xb4f22f9 as i32 as uint32_t,
        0x5f69791d as i32 as uint32_t,
    ],
    [
        0x18f46acf as i32 as uint32_t,
        0x8c06e318 as u32,
        0x4d2aeb2c as i32 as uint32_t,
    ],
    [
        0x18c23246 as i32 as uint32_t,
        0xdc0a9f3d as u32,
        0x3fe16970 as i32 as uint32_t,
    ],
];
static mut log2_radix: [limb_t; 35] = [
    0x2000000000000000 as i64 as limb_t,
    0x32b803473f7ad0f4 as i64 as limb_t,
    0x4000000000000000 as i64 as limb_t,
    0x4a4d3c25e68dc57f as i64 as limb_t,
    0x52b803473f7ad0f4 as i64 as limb_t,
    0x59d5d9fd5010b366 as i64 as limb_t,
    0x6000000000000000 as i64 as limb_t,
    0x6570068e7ef5a1e8 as i64 as limb_t,
    0x6a4d3c25e68dc57f as i64 as limb_t,
    0x6eb3a9f01975077f as i64 as limb_t,
    0x72b803473f7ad0f4 as i64 as limb_t,
    0x766a008e4788cbcd as i64 as limb_t,
    0x79d5d9fd5010b366 as i64 as limb_t,
    0x7d053f6d26089673 as i64 as limb_t,
    0x8000000000000000 as u64,
    0x82cc7edf592262d0 as u64,
    0x8570068e7ef5a1e8 as u64,
    0x87ef05ae409a0289 as u64,
    0x8a4d3c25e68dc57f as u64,
    0x8c8ddd448f8b845a as u64,
    0x8eb3a9f01975077f as u64,
    0x90c10500d63aa659 as u64,
    0x92b803473f7ad0f4 as u64,
    0x949a784bcd1b8afe as u64,
    0x966a008e4788cbcd as u64,
    0x982809d5be7072dc as u64,
    0x99d5d9fd5010b366 as u64,
    0x9b74948f5532da4b as u64,
    0x9d053f6d26089673 as u64,
    0x9e88c6b3626a72aa as u64,
    0xa000000000000000 as u64,
    0xa16bad3758efd873 as u64,
    0xa2cc7edf592262d0 as u64,
    0xa4231623369e78e6 as u64,
    0xa570068e7ef5a1e8 as u64,
];
/* compute floor(a*b) or ceil(a*b) with b = log2(radix) or
b=1/log2(radix). For is_inv = 0, strict accuracy is not guaranteed
when radix is not a power of two. */
#[no_mangle]
pub unsafe extern "C" fn bf_mul_log2_radix(
    mut a1: slimb_t,
    mut radix: u32,
    mut is_inv: i32,
    mut is_ceil1: i32,
) -> slimb_t {
    let mut is_neg: i32 = 0;
    let mut a: limb_t = 0;
    let mut is_ceil: BOOL = 0;
    is_ceil = is_ceil1;
    a = a1 as limb_t;
    if a1 < 0 as i32 as i64 {
        a = a.wrapping_neg();
        is_neg = 1 as i32
    } else {
        is_neg = 0 as i32
    }
    is_ceil ^= is_neg;
    if radix & radix.wrapping_sub(1 as i32 as u32) == 0 as i32 as u32 {
        let mut radix_bits: i32 = 0;
        /* radix is a power of two */
        radix_bits = ceil_log2(radix as limb_t);
        if is_inv != 0 {
            if is_ceil != 0 {
                a = (a as u64).wrapping_add((radix_bits - 1 as i32) as u64) as limb_t as limb_t
            }
            a = a.wrapping_div(radix_bits as u64)
        } else {
            a = a.wrapping_mul(radix_bits as u64)
        }
    } else {
        let mut tab: *const uint32_t = 0 as *const uint32_t;
        let mut b0: limb_t = 0;
        let mut b1: limb_t = 0;
        let mut t: dlimb_t = 0;
        if is_inv != 0 {
            tab = inv_log2_radix[radix.wrapping_sub(2 as i32 as u32) as usize].as_ptr();
            b1 = (*tab.offset(0 as i32 as isize) as limb_t) << 32 as i32
                | *tab.offset(1 as i32 as isize) as u64;
            b0 = (*tab.offset(2 as i32 as isize) as limb_t) << 32 as i32;
            t = (b0 as dlimb_t).wrapping_mul(a as dlimb_t);
            t = (b1 as dlimb_t)
                .wrapping_mul(a as dlimb_t)
                .wrapping_add(t >> ((1 as i32) << 6 as i32));
            a = (t >> ((1 as i32) << 6 as i32) - 1 as i32) as limb_t
        } else {
            b0 = log2_radix[radix.wrapping_sub(2 as i32 as u32) as usize];
            t = (b0 as dlimb_t).wrapping_mul(a as dlimb_t);
            a = (t >> ((1 as i32) << 6 as i32) - 3 as i32) as limb_t
        }
        /* a = floor(result) and 'result' cannot be an integer */
        a = (a as u64).wrapping_add(is_ceil as u64) as limb_t as limb_t
    }
    if is_neg != 0 {
        a = a.wrapping_neg()
    }
    return a as slimb_t;
}
/* 'n' is the number of output limbs */
unsafe extern "C" fn bf_integer_to_radix_rec(
    mut pow_tab: *mut bf_t,
    mut out: *mut limb_t,
    mut a: *const bf_t,
    mut n: limb_t,
    mut level: i32,
    mut n0: limb_t,
    mut radixl: limb_t,
    mut radixl_bits: u32,
) -> i32 {
    let mut current_block: u64;
    let mut n1: limb_t = 0;
    let mut n2: limb_t = 0;
    let mut q_prec: limb_t = 0;
    let mut ret: i32 = 0;
    if n >= 1 as i32 as u64 {
    } else {
        assert!(n >= 1);
    }
    if n == 1 as i32 as u64 {
        *out.offset(0 as i32 as isize) = get_bits(
            (*a).tab,
            (*a).len,
            (*a).len
                .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                .wrapping_sub((*a).expn as u64) as slimb_t,
        )
    } else if n == 2 as i32 as u64 {
        let mut t: dlimb_t = 0;
        let mut pos: slimb_t = 0;
        pos = (*a)
            .len
            .wrapping_mul(((1 as i32) << 6 as i32) as u64)
            .wrapping_sub((*a).expn as u64) as slimb_t;
        t = (get_bits((*a).tab, (*a).len, pos + ((1 as i32) << 6 as i32) as i64) as dlimb_t)
            << ((1 as i32) << 6 as i32)
            | get_bits((*a).tab, (*a).len, pos) as u128;
        if (radixl == 10000000000000000000 as u64) as i32 as i64 != 0 {
            /* use division by a constant when possible */
            *out.offset(0 as i32 as isize) =
                t.wrapping_rem(10000000000000000000 as u64 as u128) as limb_t;
            *out.offset(1 as i32 as isize) =
                t.wrapping_div(10000000000000000000 as u64 as u128) as limb_t
        } else {
            *out.offset(0 as i32 as isize) = t.wrapping_rem(radixl as u128) as limb_t;
            *out.offset(1 as i32 as isize) = t.wrapping_div(radixl as u128) as limb_t
        }
    } else {
        let mut Q: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut R: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut B: *mut bf_t = 0 as *mut bf_t;
        let mut B_inv: *mut bf_t = 0 as *mut bf_t;
        let mut q_add: i32 = 0;
        bf_init((*a).ctx, &mut Q);
        bf_init((*a).ctx, &mut R);
        n2 = (n0.wrapping_mul(2 as i32 as u64) >> level + 1 as i32)
            .wrapping_add(1 as i32 as u64)
            .wrapping_div(2 as i32 as u64);
        n1 = n.wrapping_sub(n2);
        B = &mut *pow_tab.offset((2 as i32 * level) as isize) as *mut bf_t;
        B_inv = &mut *pow_tab.offset((2 as i32 * level + 1 as i32) as isize) as *mut bf_t;
        ret = 0 as i32;
        if (*B).len == 0 as i32 as u64 {
            /* compute BASE^n2 */
            ret |= bf_pow_ui_ui(
                B,
                radixl,
                n2,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                BF_RNDZ as i32 as bf_flags_t,
            );
            /* we use enough bits for the maximum possible 'n1' value,
            i.e. n2 + 1 */
            ret |= bf_set_ui(&mut R, 1 as i32 as uint64_t);
            ret |= bf_div(
                B_inv,
                &mut R,
                B,
                n2.wrapping_add(1 as i32 as u64)
                    .wrapping_mul(radixl_bits as u64)
                    .wrapping_add(2 as i32 as u64),
                BF_RNDN as i32 as bf_flags_t,
            )
        }
        //        printf("%d: n1=% " PRId64 " n2=%" PRId64 "\n", level, n1, n2);
        q_prec = n1.wrapping_mul(radixl_bits as u64);
        ret |= bf_mul(&mut Q, a, B_inv, q_prec, BF_RNDN as i32 as bf_flags_t);
        ret |= bf_rint(&mut Q, BF_RNDZ as i32);
        ret |= bf_mul(
            &mut R,
            &mut Q,
            B,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        ret |= bf_sub(
            &mut R,
            a,
            &mut R,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        if ret & (1 as i32) << 5 as i32 != 0 {
            current_block = 12197237611908087741;
        } else {
            /* adjust if necessary */
            q_add = 0 as i32;
            loop {
                if !(R.sign != 0 && R.len != 0 as i32 as u64) {
                    current_block = 10692455896603418738;
                    break;
                }
                if bf_add(
                    &mut R,
                    &mut R,
                    B,
                    ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                        .wrapping_sub(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64),
                    BF_RNDZ as i32 as bf_flags_t,
                ) != 0
                {
                    current_block = 12197237611908087741;
                    break;
                }
                q_add -= 1
            }
            match current_block {
                12197237611908087741 => {}
                _ => {
                    loop {
                        if !(bf_cmpu(&mut R, B) >= 0 as i32) {
                            current_block = 9007357115414505193;
                            break;
                        }
                        if bf_sub(
                            &mut R,
                            &mut R,
                            B,
                            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                                .wrapping_sub(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64),
                            BF_RNDZ as i32 as bf_flags_t,
                        ) != 0
                        {
                            current_block = 12197237611908087741;
                            break;
                        }
                        q_add += 1
                    }
                    match current_block {
                        12197237611908087741 => {}
                        _ => {
                            if q_add != 0 as i32 {
                                if bf_add_si(
                                    &mut Q,
                                    &mut Q,
                                    q_add as int64_t,
                                    ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                                        .wrapping_sub(2 as i32 as u64)
                                        .wrapping_add(1 as i32 as u64),
                                    BF_RNDZ as i32 as bf_flags_t,
                                ) != 0
                                {
                                    current_block = 12197237611908087741;
                                } else {
                                    current_block = 3222590281903869779;
                                }
                            } else {
                                current_block = 3222590281903869779;
                            }
                            match current_block {
                                12197237611908087741 => {}
                                _ => {
                                    if bf_integer_to_radix_rec(
                                        pow_tab,
                                        out.offset(n2 as isize),
                                        &mut Q,
                                        n1,
                                        level + 1 as i32,
                                        n0,
                                        radixl,
                                        radixl_bits,
                                    ) != 0
                                    {
                                        current_block = 12197237611908087741;
                                    } else if bf_integer_to_radix_rec(
                                        pow_tab,
                                        out,
                                        &mut R,
                                        n2,
                                        level + 1 as i32,
                                        n0,
                                        radixl,
                                        radixl_bits,
                                    ) != 0
                                    {
                                        current_block = 12197237611908087741;
                                    } else {
                                        bf_delete(&mut Q);
                                        bf_delete(&mut R);
                                        current_block = 14945149239039849694;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            14945149239039849694 => {}
            _ => {
                bf_delete(&mut Q);
                bf_delete(&mut R);
                return -(1 as i32);
            }
        }
    }
    return 0 as i32;
}
/* return 0 if OK != 0 if memory error */
unsafe extern "C" fn bf_integer_to_radix(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut radixl: limb_t,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx; /* XXX: check */
    let mut r_len: limb_t = 0;
    let mut pow_tab: *mut bf_t = 0 as *mut bf_t;
    let mut i: i32 = 0;
    let mut pow_tab_len: i32 = 0;
    let mut ret: i32 = 0;
    r_len = (*r).len;
    pow_tab_len = (ceil_log2(r_len) + 2 as i32) * 2 as i32;
    pow_tab = bf_malloc(
        s,
        (::std::mem::size_of::<bf_t>() as u64).wrapping_mul(pow_tab_len as u64),
    ) as *mut bf_t;
    if pow_tab.is_null() {
        return -(1 as i32);
    }
    i = 0 as i32;
    while i < pow_tab_len {
        bf_init((*r).ctx, &mut *pow_tab.offset(i as isize));
        i += 1
    }
    ret = bf_integer_to_radix_rec(
        pow_tab,
        (*r).tab,
        a,
        r_len,
        0 as i32,
        r_len,
        radixl,
        ceil_log2(radixl) as u32,
    );
    i = 0 as i32;
    while i < pow_tab_len {
        bf_delete(&mut *pow_tab.offset(i as isize));
        i += 1
    }
    bf_free(s, pow_tab as *mut std::ffi::c_void);
    return ret;
}
/* a must be >= 0. 'P' is the wanted number of digits in radix
'radix'. 'r' is the mantissa represented as an integer. *pE
contains the exponent. Return != 0 if memory error. */
unsafe extern "C" fn bf_convert_to_radix(
    mut r: *mut bf_t,
    mut pE: *mut slimb_t,
    mut a: *const bf_t,
    mut radix: i32,
    mut P: limb_t,
    mut rnd_mode: bf_rnd_t,
    mut is_fixed_exponent: BOOL,
) -> i32 {
    let mut E: slimb_t = 0;
    let mut e: slimb_t = 0;
    let mut prec: slimb_t = 0;
    let mut extra_bits: slimb_t = 0;
    let mut ziv_extra_bits: slimb_t = 0;
    let mut prec0: slimb_t = 0;
    let mut B_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut B: *mut bf_t = &mut B_s;
    let mut e_sign: i32 = 0;
    let mut ret: i32 = 0;
    let mut res: i32 = 0;
    if (*a).len == 0 as i32 as u64 {
        /* zero case */
        *pE = 0 as i32 as slimb_t;
        return bf_set(r, a);
    }
    if is_fixed_exponent != 0 {
        E = *pE
    } else {
        /* compute the new exponent */
        E = 1 as i32 as i64
            + bf_mul_log2_radix(
                (*a).expn - 1 as i32 as i64,
                radix as u32,
                TRUE as i32,
                FALSE as i32,
            )
    }
    loop
    //    bf_print_str("a", a);
    //    printf("E=%ld P=%ld radix=%d\n", E, P, radix);
    {
        e = P.wrapping_sub(E as u64) as slimb_t;
        e_sign = 0 as i32;
        if e < 0 as i32 as i64 {
            e = -e;
            e_sign = 1 as i32
        }
        /* Note: precision for log2(radix) is not critical here */
        prec0 = bf_mul_log2_radix(P as slimb_t, radix as u32, FALSE as i32, TRUE as i32);
        ziv_extra_bits = 16 as i32 as slimb_t;
        loop {
            prec = prec0 + ziv_extra_bits;
            /* XXX: rigorous error analysis needed */
            extra_bits = (ceil_log2(e as limb_t) * 2 as i32 + 1 as i32) as slimb_t;
            ret = bf_pow_ui_ui(
                r,
                radix as limb_t,
                e as limb_t,
                (prec + extra_bits) as limb_t,
                (BF_RNDN as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
            );
            if e_sign == 0 {
                ret |= bf_mul(
                    r,
                    r,
                    a,
                    (prec + extra_bits) as limb_t,
                    (BF_RNDN as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
                )
            } else {
                ret |= bf_div(
                    r,
                    a,
                    r,
                    (prec + extra_bits) as limb_t,
                    (BF_RNDN as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
                )
            }
            if ret & (1 as i32) << 5 as i32 != 0 {
                return (1 as i32) << 5 as i32;
            }
            /* if the result is not exact, check that it can be safely
            rounded to an integer */
            if ret & (1 as i32) << 4 as i32 != 0 && bf_can_round(r, (*r).expn, rnd_mode, prec) == 0
            {
                /* and more precision and retry */
                ziv_extra_bits = ziv_extra_bits + ziv_extra_bits / 2 as i32 as i64
            } else {
                ret = bf_rint(r, rnd_mode as i32);
                if ret & (1 as i32) << 5 as i32 != 0 {
                    return (1 as i32) << 5 as i32;
                }
                break;
            }
        }
        if is_fixed_exponent != 0 {
            break;
        }
        /* check that the result is < B^P */
        /* XXX: do a fast approximate test first ? */
        bf_init((*r).ctx, B);
        ret = bf_pow_ui_ui(
            B,
            radix as limb_t,
            P,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        if ret != 0 {
            bf_delete(B);
            return ret;
        }
        res = bf_cmpu(r, B);
        bf_delete(B);
        if res < 0 as i32 {
            break;
        }
        /* try a larger exponent */
        E += 1
    }
    *pE = E;
    return 0 as i32;
}
unsafe extern "C" fn limb_to_a(
    mut buf: *mut std::os::raw::c_char,
    mut n: limb_t,
    mut radix: u32,
    mut len: i32,
) {
    let mut digit: i32 = 0;
    let mut i: i32 = 0;
    if radix == 10 as i32 as u32 {
        /* specific case with constant divisor */
        i = len - 1 as i32;
        while i >= 0 as i32 {
            digit = n.wrapping_rem(10 as i32 as u64) as i32;
            n = n.wrapping_div(10 as i32 as u64);
            *buf.offset(i as isize) = (digit + '0' as i32) as std::os::raw::c_char;
            i -= 1
        }
    } else {
        i = len - 1 as i32;
        while i >= 0 as i32 {
            digit = n.wrapping_rem(radix as u64) as i32;
            n = n.wrapping_div(radix as u64);
            if digit < 10 as i32 {
                digit += '0' as i32
            } else {
                digit += 'a' as i32 - 10 as i32
            }
            *buf.offset(i as isize) = digit as std::os::raw::c_char;
            i -= 1
        }
    };
}
/* for power of 2 radixes */
unsafe extern "C" fn limb_to_a2(
    mut buf: *mut std::os::raw::c_char,
    mut n: limb_t,
    mut radix_bits: u32,
    mut len: i32,
) {
    let mut digit: i32 = 0;
    let mut i: i32 = 0;
    let mut mask: u32 = 0;
    mask = (((1 as i32) << radix_bits) - 1 as i32) as u32;
    i = len - 1 as i32;
    while i >= 0 as i32 {
        digit = (n & mask as u64) as i32;
        n >>= radix_bits;
        if digit < 10 as i32 {
            digit += '0' as i32
        } else {
            digit += 'a' as i32 - 10 as i32
        }
        *buf.offset(i as isize) = digit as std::os::raw::c_char;
        i -= 1
    }
}
/* 'a' must be an integer if the is_dec = FALSE or if the radix is not
a power of two. A dot is added before the 'dot_pos' digit. dot_pos
= n_digits does not display the dot. 0 <= dot_pos <=
n_digits. n_digits >= 1. */
unsafe extern "C" fn output_digits(
    mut s: *mut DynBuf,
    mut a1: *const bf_t,
    mut radix: i32,
    mut n_digits: limb_t,
    mut dot_pos: limb_t,
    mut is_dec: BOOL,
) {
    let mut current_block: u64;
    let mut i: limb_t = 0;
    let mut v: limb_t = 0;
    let mut l: limb_t = 0;
    let mut pos: slimb_t = 0;
    let mut pos_incr: slimb_t = 0;
    let mut digits_per_limb: i32 = 0;
    let mut buf_pos: i32 = 0;
    let mut radix_bits: i32 = 0;
    let mut first_buf_pos: i32 = 0;
    let mut buf: [std::os::raw::c_char; 65] = [0; 65];
    let mut a_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut a: *mut bf_t = 0 as *mut bf_t;
    if is_dec != 0 {
        digits_per_limb = 19 as i32;
        a = a1 as *mut bf_t;
        radix_bits = 0 as i32;
        pos = (*a).len as slimb_t;
        pos_incr = 1 as i32 as slimb_t;
        first_buf_pos = 0 as i32;
        current_block = 5689316957504528238;
    } else if radix & radix - 1 as i32 == 0 as i32 {
        a = a1 as *mut bf_t;
        radix_bits = ceil_log2(radix as limb_t);
        digits_per_limb = ((1 as i32) << 6 as i32) / radix_bits;
        pos_incr = (digits_per_limb * radix_bits) as slimb_t;
        /* digits are aligned relative to the radix point */
        pos = (*a)
            .len
            .wrapping_mul(((1 as i32) << 6 as i32) as u64)
            .wrapping_add(smod(-(*a).expn, radix_bits as slimb_t)) as slimb_t;
        first_buf_pos = 0 as i32;
        current_block = 5689316957504528238;
    } else {
        let mut n: limb_t = 0;
        let mut radixl: limb_t = 0;
        digits_per_limb = digits_per_limb_table[(radix - 2 as i32) as usize] as i32;
        radixl = get_limb_radix(radix);
        a = &mut a_s;
        bf_init((*a1).ctx, a);
        n = n_digits
            .wrapping_add(digits_per_limb as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(digits_per_limb as u64);
        if bf_resize(a, n) != 0 {
            dbuf_set_error(s);
            current_block = 1306486858251134575;
        } else if bf_integer_to_radix(a, a1, radixl) != 0 {
            dbuf_set_error(s);
            current_block = 1306486858251134575;
        } else {
            radix_bits = 0 as i32;
            pos = n as slimb_t;
            pos_incr = 1 as i32 as slimb_t;
            first_buf_pos = ((pos * digits_per_limb as i64) as u64).wrapping_sub(n_digits) as i32;
            current_block = 5689316957504528238;
        }
    }
    match current_block {
        5689316957504528238 => {
            buf_pos = digits_per_limb;
            i = 0 as i32 as limb_t;
            while i < n_digits {
                if buf_pos == digits_per_limb {
                    pos -= pos_incr;
                    if radix_bits == 0 as i32 {
                        v = get_limbz(a, pos as limb_t);
                        limb_to_a(buf.as_mut_ptr(), v, radix as u32, digits_per_limb);
                    } else {
                        v = get_bits((*a).tab, (*a).len, pos);
                        limb_to_a2(buf.as_mut_ptr(), v, radix_bits as u32, digits_per_limb);
                    }
                    buf_pos = first_buf_pos;
                    first_buf_pos = 0 as i32
                }
                if i < dot_pos {
                    l = dot_pos
                } else {
                    if i == dot_pos {
                        dbuf_putc(s, '.' as i32 as uint8_t);
                    }
                    l = n_digits
                }
                l = bf_min(
                    (digits_per_limb - buf_pos) as slimb_t,
                    l.wrapping_sub(i) as slimb_t,
                ) as limb_t;
                dbuf_put(
                    s,
                    buf.as_mut_ptr().offset(buf_pos as isize) as *mut uint8_t,
                    l as usize,
                );
                buf_pos = (buf_pos as u64).wrapping_add(l) as i32 as i32;
                i = (i as u64).wrapping_add(l) as limb_t as limb_t
            }
        }
        _ => {}
    }
    if a != a1 as *mut bf_t {
        bf_delete(a);
    };
}
unsafe extern "C" fn bf_dbuf_realloc(
    mut opaque: *mut std::ffi::c_void,
    mut ptr: *mut std::ffi::c_void,
    mut size: size_t,
) -> *mut std::ffi::c_void {
    let mut s: *mut bf_context_t = opaque as *mut bf_context_t;
    return bf_realloc(s, ptr, size);
}
/* return the length in bytes. A trailing '\0' is added */
unsafe extern "C" fn bf_ftoa_internal(
    mut plen: *mut size_t,
    mut a2: *const bf_t,
    mut radix: i32,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut is_dec: BOOL,
) -> *mut std::os::raw::c_char {
    let mut a_s_0: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut a_0: *mut bf_t = 0 as *mut bf_t;
    let mut current_block: u64;
    let mut ctx: *mut bf_context_t = (*a2).ctx;
    let mut s_s: DynBuf = DynBuf {
        buf: 0 as *mut uint8_t,
        size: 0,
        allocated_size: 0,
        error: 0,
        realloc_func: None,
        opaque: 0 as *mut std::ffi::c_void,
    };
    let mut s: *mut DynBuf = &mut s_s;
    let mut radix_bits: i32 = 0;
    //    bf_print_str("ftoa", a2);
    //    printf("radix=%d\n", radix);
    dbuf_init2(
        s,
        ctx as *mut std::ffi::c_void,
        Some(
            bf_dbuf_realloc
                as unsafe extern "C" fn(
                    _: *mut std::ffi::c_void,
                    _: *mut std::ffi::c_void,
                    _: size_t,
                ) -> *mut std::ffi::c_void,
        ),
    );
    if (*a2).expn == 9223372036854775807 as i64 {
        dbuf_putstr(s, b"NaN\x00" as *const u8 as *const std::os::raw::c_char);
        current_block = 17418136423408909163;
    } else {
        if (*a2).sign != 0 {
            dbuf_putc(s, '-' as i32 as uint8_t);
        }
        if (*a2).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            if flags & ((1 as i32) << 22 as i32) as u32 != 0 {
                dbuf_putstr(
                    s,
                    b"Infinity\x00" as *const u8 as *const std::os::raw::c_char,
                );
            } else {
                dbuf_putstr(s, b"Inf\x00" as *const u8 as *const std::os::raw::c_char);
            }
            current_block = 17418136423408909163;
        } else {
            let mut fmt: i32 = 0;
            let mut ret: i32 = 0;
            let mut n_digits: slimb_t = 0;
            let mut n: slimb_t = 0;
            let mut i: slimb_t = 0;
            let mut n_max: slimb_t = 0;
            let mut n1: slimb_t = 0;
            let mut a1_s: bf_t = bf_t {
                ctx: 0 as *mut bf_context_t,
                sign: 0,
                expn: 0,
                len: 0,
                tab: 0 as *mut limb_t,
            };
            let mut a1: *mut bf_t = &mut a1_s;
            if radix & radix - 1 as i32 != 0 as i32 {
                radix_bits = 0 as i32
            } else {
                radix_bits = ceil_log2(radix as limb_t)
            }
            fmt = (flags & ((3 as i32) << 16 as i32) as u32) as i32;
            bf_init(ctx, a1);
            if fmt == (1 as i32) << 16 as i32 {
                if is_dec != 0 || radix_bits != 0 as i32 {
                    if bf_set(a1, a2) != 0 {
                        current_block = 7361972997577608855;
                    } else {
                        if is_dec != 0 {
                            if bfdec_round(
                                a1 as *mut bfdec_t,
                                prec,
                                flags & 0x7 as i32 as u32 | ((1 as i32) << 4 as i32) as u32,
                            ) & (1 as i32) << 5 as i32
                                != 0
                            {
                                current_block = 7361972997577608855;
                            } else {
                                n = (*a1).expn;
                                current_block = 11057878835866523405;
                            }
                        } else if bf_round(
                            a1,
                            prec.wrapping_mul(radix_bits as u64),
                            flags & 0x7 as i32 as u32 | ((1 as i32) << 4 as i32) as u32,
                        ) & (1 as i32) << 5 as i32
                            != 0
                        {
                            current_block = 7361972997577608855;
                        } else {
                            n = ceil_div((*a1).expn, radix_bits as slimb_t);
                            current_block = 11057878835866523405;
                        }
                        match current_block {
                            7361972997577608855 => {}
                            _ => {
                                if flags & ((1 as i32) << 21 as i32) as u32 != 0 {
                                    if radix == 16 as i32 {
                                        dbuf_putstr(
                                            s,
                                            b"0x\x00" as *const u8 as *const std::os::raw::c_char,
                                        );
                                    } else if radix == 8 as i32 {
                                        dbuf_putstr(
                                            s,
                                            b"0o\x00" as *const u8 as *const std::os::raw::c_char,
                                        );
                                    } else if radix == 2 as i32 {
                                        dbuf_putstr(
                                            s,
                                            b"0b\x00" as *const u8 as *const std::os::raw::c_char,
                                        );
                                    }
                                }
                                if (*a1).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
                                    dbuf_putstr(
                                        s,
                                        b"0\x00" as *const u8 as *const std::os::raw::c_char,
                                    );
                                    if prec > 0 as i32 as u64 {
                                        dbuf_putstr(
                                            s,
                                            b".\x00" as *const u8 as *const std::os::raw::c_char,
                                        );
                                        i = 0 as i32 as slimb_t;
                                        while (i as u64) < prec {
                                            dbuf_putc(s, '0' as i32 as uint8_t);
                                            i += 1
                                        }
                                    }
                                } else {
                                    n_digits = prec.wrapping_add(n as u64) as slimb_t;
                                    if n <= 0 as i32 as i64 {
                                        /* 0.x */
                                        dbuf_putstr(
                                            s,
                                            b"0.\x00" as *const u8 as *const std::os::raw::c_char,
                                        );
                                        i = 0 as i32 as slimb_t;
                                        while i < -n {
                                            dbuf_putc(s, '0' as i32 as uint8_t);
                                            i += 1
                                        }
                                        if n_digits > 0 as i32 as i64 {
                                            output_digits(
                                                s,
                                                a1,
                                                radix,
                                                n_digits as limb_t,
                                                n_digits as limb_t,
                                                is_dec,
                                            );
                                        }
                                    } else {
                                        output_digits(
                                            s,
                                            a1,
                                            radix,
                                            n_digits as limb_t,
                                            n as limb_t,
                                            is_dec,
                                        );
                                    }
                                }
                                current_block = 9985465603744958559;
                            }
                        }
                    }
                } else {
                    let mut pos: size_t = 0;
                    let mut start: size_t = 0;
                    let mut a_s: bf_t = bf_t {
                        ctx: 0 as *mut bf_context_t,
                        sign: 0,
                        expn: 0,
                        len: 0,
                        tab: 0 as *mut limb_t,
                    };
                    let mut a: *mut bf_t = &mut a_s;
                    /* make a positive number */
                    (*a).tab = (*a2).tab;
                    (*a).len = (*a2).len;
                    (*a).expn = (*a2).expn;
                    (*a).sign = 0 as i32;
                    /* one more digit for the rounding */
                    n = 1 as i32 as i64
                        + bf_mul_log2_radix(
                            bf_max((*a).expn, 0 as i32 as slimb_t),
                            radix as u32,
                            TRUE as i32,
                            TRUE as i32,
                        );
                    n_digits = (n as u64).wrapping_add(prec) as slimb_t;
                    n1 = n;
                    if bf_convert_to_radix(
                        a1,
                        &mut n1,
                        a,
                        radix,
                        n_digits as limb_t,
                        (flags & 0x7 as i32 as u32) as bf_rnd_t,
                        TRUE as i32,
                    ) != 0
                    {
                        current_block = 7361972997577608855;
                    } else {
                        start = (*s).size;
                        output_digits(s, a1, radix, n_digits as limb_t, n as limb_t, is_dec);
                        /* remove leading zeros because we allocated one more digit */
                        pos = start;
                        while pos.wrapping_add(1 as i32 as u64) < (*s).size
                            && *(*s).buf.offset(pos as isize) as i32 == '0' as i32
                            && *(*s).buf.offset(pos.wrapping_add(1 as i32 as u64) as isize) as i32
                                != '.' as i32
                        {
                            pos = pos.wrapping_add(1)
                        }
                        if pos > start {
                            ((*s).buf.offset(start as isize) as *mut u8).copy_from_nonoverlapping(
                                (*s).buf.offset(pos as isize) as *const u8,
                                (*s).size.wrapping_sub(pos) as usize,
                            );
                            (*s).size = ((*s).size as u64).wrapping_sub(pos.wrapping_sub(start))
                                as size_t as size_t
                        }
                        current_block = 9985465603744958559;
                    }
                }
            } else {
                if is_dec != 0 {
                    if bf_set(a1, a2) != 0 {
                        current_block = 7361972997577608855;
                    } else {
                        if fmt == (0 as i32) << 16 as i32 {
                            n_digits = prec as slimb_t;
                            n_max = n_digits;
                            if bfdec_round(a1 as *mut bfdec_t, prec, flags & 0x7 as i32 as u32)
                                & (1 as i32) << 5 as i32
                                != 0
                            {
                                current_block = 7361972997577608855;
                            } else {
                                current_block = 16789764818708874114;
                            }
                        } else {
                            /* prec is ignored */
                            n_digits = (*a1).len.wrapping_mul(19 as i32 as u64) as slimb_t;
                            prec = n_digits as limb_t;
                            /* remove the trailing zero digits */
                            while n_digits > 1 as i32 as i64
                                && get_digit(
                                    (*a1).tab,
                                    (*a1).len,
                                    prec.wrapping_sub(n_digits as u64) as slimb_t,
                                ) == 0 as i32 as u64
                            {
                                n_digits -= 1
                            }
                            n_max = n_digits + 4 as i32 as i64;
                            current_block = 16789764818708874114;
                        }
                        match current_block {
                            7361972997577608855 => {}
                            _ => {
                                n = (*a1).expn;
                                current_block = 4235089732467486934;
                            }
                        }
                    }
                } else if radix_bits != 0 as i32 {
                    if bf_set(a1, a2) != 0 {
                        current_block = 7361972997577608855;
                    } else {
                        if fmt == (0 as i32) << 16 as i32 {
                            let mut prec_bits: slimb_t = 0;
                            n_digits = prec as slimb_t;
                            n_max = n_digits;
                            /* align to the radix point */
                            prec_bits = prec
                                .wrapping_mul(radix_bits as u64)
                                .wrapping_sub(smod(-(*a1).expn, radix_bits as slimb_t))
                                as slimb_t;
                            if bf_round(a1, prec_bits as limb_t, flags & 0x7 as i32 as u32)
                                & (1 as i32) << 5 as i32
                                != 0
                            {
                                current_block = 7361972997577608855;
                            } else {
                                current_block = 6074735043880891984;
                            }
                        } else {
                            let mut digit_mask: limb_t = 0;
                            let mut pos_0: slimb_t = 0;
                            /* position of the digit before the most
                            significant digit in bits */
                            pos_0 = (*a1)
                                .len
                                .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                                .wrapping_add(smod(-(*a1).expn, radix_bits as slimb_t))
                                as slimb_t;
                            n_digits = ceil_div(pos_0, radix_bits as slimb_t);
                            /* remove the trailing zero digits */
                            digit_mask =
                                ((1 as i32 as limb_t) << radix_bits).wrapping_sub(1 as i32 as u64);
                            while n_digits > 1 as i32 as i64
                                && get_bits(
                                    (*a1).tab,
                                    (*a1).len,
                                    pos_0 - n_digits * radix_bits as i64,
                                ) & digit_mask
                                    == 0 as i32 as u64
                            {
                                n_digits -= 1
                            }
                            n_max = n_digits + 4 as i32 as i64;
                            current_block = 6074735043880891984;
                        }
                        match current_block {
                            7361972997577608855 => {}
                            _ => {
                                n = ceil_div((*a1).expn, radix_bits as slimb_t);
                                current_block = 4235089732467486934;
                            }
                        }
                    }
                } else {
                    a_s_0 = bf_t {
                        ctx: 0 as *mut bf_context_t,
                        sign: 0,
                        expn: 0,
                        len: 0,
                        tab: 0 as *mut limb_t,
                    };
                    a_0 = &mut a_s_0;
                    /* make a positive number */
                    (*a_0).tab = (*a2).tab;
                    (*a_0).len = (*a2).len;
                    (*a_0).expn = (*a2).expn;
                    (*a_0).sign = 0 as i32;
                    if fmt == (0 as i32) << 16 as i32 {
                        n_digits = prec as slimb_t;
                        n_max = n_digits;
                        current_block = 10763371041174037105;
                    } else {
                        let mut n_digits_max: slimb_t = 0;
                        let mut n_digits_min: slimb_t = 0;
                        if prec
                            != ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                                .wrapping_sub(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        {
                        } else {
                            assert!(prec != BF_PREC_INF);
                        }
                        n_digits = 1 as i32 as i64
                            + bf_mul_log2_radix(
                                prec as slimb_t,
                                radix as u32,
                                TRUE as i32,
                                TRUE as i32,
                            );
                        /* max number of digits for non exponential
                        notation. The rational is to have the same rule
                        as JS i.e. n_max = 21 for 64 bit float in base 10. */
                        n_max = n_digits + 4 as i32 as i64;
                        if fmt == (3 as i32) << 16 as i32 {
                            let mut b_s: bf_t = bf_t {
                                ctx: 0 as *mut bf_context_t,
                                sign: 0,
                                expn: 0,
                                len: 0,
                                tab: 0 as *mut limb_t,
                            };
                            let mut b: *mut bf_t = &mut b_s;
                            /* find the minimum number of digits by
                            dichotomy. */
                            /* XXX: inefficient */
                            n_digits_max = n_digits;
                            n_digits_min = 1 as i32 as slimb_t;
                            bf_init(ctx, b);
                            loop {
                                if !(n_digits_min < n_digits_max) {
                                    current_block = 13454018412769612774;
                                    break;
                                }
                                n_digits = (n_digits_min + n_digits_max) / 2 as i32 as i64;
                                if bf_convert_to_radix(
                                    a1,
                                    &mut n,
                                    a_0,
                                    radix,
                                    n_digits as limb_t,
                                    (flags & 0x7 as i32 as u32) as bf_rnd_t,
                                    FALSE as i32,
                                ) != 0
                                {
                                    bf_delete(b);
                                    current_block = 7361972997577608855;
                                    break;
                                } else {
                                    /* convert back to a number and compare */
                                    ret = bf_mul_pow_radix(
                                        b,
                                        a1,
                                        radix as limb_t,
                                        n - n_digits,
                                        prec,
                                        flags & !(0x7 as i32) as u32 | BF_RNDN as i32 as u32,
                                    );
                                    if ret & (1 as i32) << 5 as i32 != 0 {
                                        bf_delete(b);
                                        current_block = 7361972997577608855;
                                        break;
                                    } else if bf_cmpu(b, a_0) == 0 as i32 {
                                        n_digits_max = n_digits
                                    } else {
                                        n_digits_min = n_digits + 1 as i32 as i64
                                    }
                                }
                            }
                            match current_block {
                                7361972997577608855 => {}
                                _ => {
                                    bf_delete(b);
                                    n_digits = n_digits_max;
                                    current_block = 10763371041174037105;
                                }
                            }
                        } else {
                            current_block = 10763371041174037105;
                        }
                    }
                    match current_block {
                        7361972997577608855 => {}
                        _ => {
                            if bf_convert_to_radix(
                                a1,
                                &mut n,
                                a_0,
                                radix,
                                n_digits as limb_t,
                                (flags & 0x7 as i32 as u32) as bf_rnd_t,
                                FALSE as i32,
                            ) != 0
                            {
                                current_block = 7361972997577608855;
                            } else {
                                current_block = 4235089732467486934;
                            }
                        }
                    }
                }
                match current_block {
                    7361972997577608855 => {}
                    _ => {
                        if (*a1).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
                            && fmt != (0 as i32) << 16 as i32
                            && flags & ((1 as i32) << 20 as i32) as u32 == 0
                        {
                            /* just output zero */
                            dbuf_putstr(s, b"0\x00" as *const u8 as *const std::os::raw::c_char);
                        } else {
                            if flags & ((1 as i32) << 21 as i32) as u32 != 0 {
                                if radix == 16 as i32 {
                                    dbuf_putstr(
                                        s,
                                        b"0x\x00" as *const u8 as *const std::os::raw::c_char,
                                    );
                                } else if radix == 8 as i32 {
                                    dbuf_putstr(
                                        s,
                                        b"0o\x00" as *const u8 as *const std::os::raw::c_char,
                                    );
                                } else if radix == 2 as i32 {
                                    dbuf_putstr(
                                        s,
                                        b"0b\x00" as *const u8 as *const std::os::raw::c_char,
                                    );
                                }
                            }
                            if (*a1).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
                                n = 1 as i32 as slimb_t
                            }
                            if flags & ((1 as i32) << 20 as i32) as u32 != 0
                                || n <= -(6 as i32) as i64
                                || n > n_max
                            {
                                let mut fmt_0: *const std::os::raw::c_char =
                                    0 as *const std::os::raw::c_char;
                                /* exponential notation */
                                output_digits(
                                    s,
                                    a1,
                                    radix,
                                    n_digits as limb_t,
                                    1 as i32 as limb_t,
                                    is_dec,
                                );
                                if radix_bits != 0 as i32 && radix <= 16 as i32 {
                                    if flags & ((1 as i32) << 22 as i32) as u32 != 0 {
                                        fmt_0 =
                                            b"p%+ld\x00" as *const u8 as *const std::os::raw::c_char
                                    } else {
                                        fmt_0 =
                                            b"p%ld\x00" as *const u8 as *const std::os::raw::c_char
                                    }
                                    dbuf_printf(
                                        s,
                                        fmt_0,
                                        (n - 1 as i32 as i64) * radix_bits as i64,
                                    );
                                } else {
                                    if flags & ((1 as i32) << 22 as i32) as u32 != 0 {
                                        fmt_0 = b"%c%+ld\x00" as *const u8
                                            as *const std::os::raw::c_char
                                    } else {
                                        fmt_0 =
                                            b"%c%ld\x00" as *const u8 as *const std::os::raw::c_char
                                    }
                                    dbuf_printf(
                                        s,
                                        fmt_0,
                                        if radix <= 10 as i32 {
                                            'e' as i32
                                        } else {
                                            '@' as i32
                                        },
                                        n - 1 as i32 as i64,
                                    );
                                }
                            } else if n <= 0 as i32 as i64 {
                                /* 0.x */
                                dbuf_putstr(
                                    s,
                                    b"0.\x00" as *const u8 as *const std::os::raw::c_char,
                                );
                                i = 0 as i32 as slimb_t;
                                while i < -n {
                                    dbuf_putc(s, '0' as i32 as uint8_t);
                                    i += 1
                                }
                                output_digits(
                                    s,
                                    a1,
                                    radix,
                                    n_digits as limb_t,
                                    n_digits as limb_t,
                                    is_dec,
                                );
                            } else if n_digits <= n {
                                /* no dot */
                                output_digits(
                                    s,
                                    a1,
                                    radix,
                                    n_digits as limb_t,
                                    n_digits as limb_t,
                                    is_dec,
                                );
                                i = 0 as i32 as slimb_t;
                                while i < n - n_digits {
                                    dbuf_putc(s, '0' as i32 as uint8_t);
                                    i += 1
                                }
                            } else {
                                output_digits(
                                    s,
                                    a1,
                                    radix,
                                    n_digits as limb_t,
                                    n as limb_t,
                                    is_dec,
                                );
                            }
                        }
                        current_block = 9985465603744958559;
                    }
                }
            }
            match current_block {
                9985465603744958559 => {
                    bf_delete(a1);
                    current_block = 17418136423408909163;
                }
                _ => {
                    bf_delete(a1);
                    current_block = 2562566406710242701;
                }
            }
        }
    }
    match current_block {
        17418136423408909163 => {
            dbuf_putc(s, '\u{0}' as i32 as uint8_t);
            if !(dbuf_error(s) != 0) {
                if !plen.is_null() {
                    *plen = (*s).size.wrapping_sub(1 as i32 as u64)
                }
                return (*s).buf as *mut std::os::raw::c_char;
            }
        }
        _ => {}
    }
    bf_free(ctx, (*s).buf as *mut std::ffi::c_void);
    if !plen.is_null() {
        *plen = 0 as i32 as size_t
    }
    return 0 as *mut std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn bf_ftoa(
    mut plen: *mut size_t,
    mut a: *const bf_t,
    mut radix: i32,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> *mut std::os::raw::c_char {
    return bf_ftoa_internal(plen, a, radix, prec, flags, FALSE as i32);
}
/* **************************************************************/
/* transcendental functions */
/* Note: the algorithm is from MPFR */
unsafe extern "C" fn bf_const_log2_rec(
    mut T: *mut bf_t,
    mut P: *mut bf_t,
    mut Q: *mut bf_t,
    mut n1: limb_t,
    mut n2: limb_t,
    mut need_P: BOOL,
) {
    let mut s: *mut bf_context_t = (*T).ctx;
    if n2.wrapping_sub(n1) == 1 as i32 as u64 {
        if n1 == 0 as i32 as u64 {
            bf_set_ui(P, 3 as i32 as uint64_t);
        } else {
            bf_set_ui(P, n1);
            (*P).sign = 1 as i32
        }
        bf_set_ui(
            Q,
            (2 as i32 as u64)
                .wrapping_mul(n1)
                .wrapping_add(1 as i32 as u64),
        );
        (*Q).expn += 2 as i32 as i64;
        bf_set(T, P);
    } else {
        let mut m: limb_t = 0;
        let mut T1_s: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut T1: *mut bf_t = &mut T1_s;
        let mut P1_s: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut P1: *mut bf_t = &mut P1_s;
        let mut Q1_s: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut Q1: *mut bf_t = &mut Q1_s;
        m = n1.wrapping_add(n2.wrapping_sub(n1) >> 1 as i32);
        bf_const_log2_rec(T, P, Q, n1, m, TRUE as i32);
        bf_init(s, T1);
        bf_init(s, P1);
        bf_init(s, Q1);
        bf_const_log2_rec(T1, P1, Q1, m, n2, need_P);
        bf_mul(
            T,
            T,
            Q1,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        bf_mul(
            T1,
            T1,
            P,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        bf_add(
            T,
            T,
            T1,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        if need_P != 0 {
            bf_mul(
                P,
                P,
                P1,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                BF_RNDZ as i32 as bf_flags_t,
            );
        }
        bf_mul(
            Q,
            Q,
            Q1,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        bf_delete(T1);
        bf_delete(P1);
        bf_delete(Q1);
    };
}
/* compute log(2) with faithful rounding at precision 'prec' */
unsafe extern "C" fn bf_const_log2_internal(mut T: *mut bf_t, mut prec: limb_t) {
    let mut w: limb_t = 0;
    let mut N: limb_t = 0;
    let mut P_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut P: *mut bf_t = &mut P_s;
    let mut Q_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut Q: *mut bf_t = &mut Q_s;
    w = prec.wrapping_add(15 as i32 as u64);
    N = w
        .wrapping_div(3 as i32 as u64)
        .wrapping_add(1 as i32 as u64);
    bf_init((*T).ctx, P);
    bf_init((*T).ctx, Q);
    bf_const_log2_rec(T, P, Q, 0 as i32 as limb_t, N, FALSE as i32);
    bf_div(T, T, Q, prec, BF_RNDN as i32 as bf_flags_t);
    bf_delete(P);
    bf_delete(Q);
}
unsafe extern "C" fn chud_bs(
    mut P: *mut bf_t,
    mut Q: *mut bf_t,
    mut G: *mut bf_t,
    mut a: int64_t,
    mut b: int64_t,
    mut need_g: i32,
    mut prec: limb_t,
) {
    let mut s: *mut bf_context_t = (*P).ctx;
    let mut c: int64_t = 0;
    if a == b - 1 as i32 as i64 {
        let mut T0: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut T1: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        bf_init(s, &mut T0);
        bf_init(s, &mut T1);
        bf_set_ui(G, (2 as i32 as i64 * b - 1 as i32 as i64) as uint64_t);
        bf_mul_ui(
            G,
            G,
            (6 as i32 as i64 * b - 1 as i32 as i64) as uint64_t,
            prec,
            BF_RNDN as i32 as bf_flags_t,
        );
        bf_mul_ui(
            G,
            G,
            (6 as i32 as i64 * b - 5 as i32 as i64) as uint64_t,
            prec,
            BF_RNDN as i32 as bf_flags_t,
        );
        bf_set_ui(&mut T0, 545140134 as i32 as uint64_t);
        bf_mul_ui(
            &mut T0,
            &mut T0,
            b as uint64_t,
            prec,
            BF_RNDN as i32 as bf_flags_t,
        );
        bf_set_ui(&mut T1, 13591409 as i32 as uint64_t);
        bf_add(
            &mut T0,
            &mut T0,
            &mut T1,
            prec,
            BF_RNDN as i32 as bf_flags_t,
        );
        bf_mul(P, G, &mut T0, prec, BF_RNDN as i32 as bf_flags_t);
        (*P).sign = (b & 1 as i32 as i64) as i32;
        bf_set_ui(Q, b as uint64_t);
        bf_mul_ui(Q, Q, b as uint64_t, prec, BF_RNDN as i32 as bf_flags_t);
        bf_mul_ui(Q, Q, b as uint64_t, prec, BF_RNDN as i32 as bf_flags_t);
        bf_mul_ui(
            Q,
            Q,
            (640320 as i32 as uint64_t)
                .wrapping_mul(640320 as i32 as u64)
                .wrapping_mul(640320 as i32 as u64)
                .wrapping_div(24 as i32 as u64),
            prec,
            BF_RNDN as i32 as bf_flags_t,
        );
        bf_delete(&mut T0);
        bf_delete(&mut T1);
    } else {
        let mut P2: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut Q2: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut G2: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        bf_init(s, &mut P2);
        bf_init(s, &mut Q2);
        bf_init(s, &mut G2);
        c = (a + b) / 2 as i32 as i64;
        chud_bs(P, Q, G, a, c, 1 as i32, prec);
        chud_bs(&mut P2, &mut Q2, &mut G2, c, b, need_g, prec);
        /* Q = Q1 * Q2 */
        /* G = G1 * G2 */
        /* P = P1 * Q2 + P2 * G1 */
        bf_mul(&mut P2, &mut P2, G, prec, BF_RNDN as i32 as bf_flags_t);
        if need_g == 0 {
            bf_set_ui(G, 0 as i32 as uint64_t);
        }
        bf_mul(P, P, &mut Q2, prec, BF_RNDN as i32 as bf_flags_t);
        bf_add(P, P, &mut P2, prec, BF_RNDN as i32 as bf_flags_t);
        bf_delete(&mut P2);
        bf_mul(Q, Q, &mut Q2, prec, BF_RNDN as i32 as bf_flags_t);
        bf_delete(&mut Q2);
        if need_g != 0 {
            bf_mul(G, G, &mut G2, prec, BF_RNDN as i32 as bf_flags_t);
        }
        bf_delete(&mut G2);
    };
}
/* compute Pi with faithful rounding at precision 'prec' using the
Chudnovsky formula */
unsafe extern "C" fn bf_const_pi_internal(mut Q: *mut bf_t, mut prec: limb_t) {
    let mut s: *mut bf_context_t = (*Q).ctx;
    let mut n: int64_t = 0;
    let mut prec1: int64_t = 0;
    let mut P: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut G: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    /* number of serie terms */
    n = prec
        .wrapping_div(47 as i32 as u64)
        .wrapping_add(1 as i32 as u64) as int64_t;
    /* XXX: precision analysis */
    prec1 = prec.wrapping_add(32 as i32 as u64) as int64_t;
    bf_init(s, &mut P);
    bf_init(s, &mut G);
    chud_bs(
        &mut P,
        Q,
        &mut G,
        0 as i32 as int64_t,
        n,
        0 as i32,
        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64),
    );
    bf_mul_ui(
        &mut G,
        Q,
        13591409 as i32 as uint64_t,
        prec1 as limb_t,
        BF_RNDN as i32 as bf_flags_t,
    );
    bf_add(
        &mut P,
        &mut G,
        &mut P,
        prec1 as limb_t,
        BF_RNDN as i32 as bf_flags_t,
    );
    bf_div(Q, Q, &mut P, prec1 as limb_t, BF_RNDF as i32 as bf_flags_t);
    bf_set_ui(&mut P, 640320 as i32 as uint64_t);
    bf_sqrt(
        &mut G,
        &mut P,
        prec1 as limb_t,
        BF_RNDF as i32 as bf_flags_t,
    );
    bf_mul_ui(
        &mut G,
        &mut G,
        (640320 as i32 as uint64_t).wrapping_div(12 as i32 as u64),
        prec1 as limb_t,
        BF_RNDF as i32 as bf_flags_t,
    );
    bf_mul(Q, Q, &mut G, prec, BF_RNDN as i32 as bf_flags_t);
    bf_delete(&mut P);
    bf_delete(&mut G);
}
unsafe extern "C" fn bf_const_get(
    mut T: *mut bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut c: *mut BFConstCache,
    mut func: Option<unsafe extern "C" fn(_: *mut bf_t, _: limb_t) -> ()>,
    mut sign: i32,
) -> i32 {
    let mut ziv_extra_bits: limb_t = 0;
    let mut prec1: limb_t = 0;
    ziv_extra_bits = 32 as i32 as limb_t;
    loop {
        prec1 = prec.wrapping_add(ziv_extra_bits);
        if (*c).prec < prec1 {
            if (*c).val.len == 0 as i32 as u64 {
                bf_init((*T).ctx, &mut (*c).val);
            }
            func.expect("non-null function pointer")(&mut (*c).val, prec1);
            (*c).prec = prec1
        } else {
            prec1 = (*c).prec
        }
        bf_set(T, &mut (*c).val);
        (*T).sign = sign;
        if !(bf_can_round(
            T,
            prec as slimb_t,
            (flags & 0x7 as i32 as u32) as bf_rnd_t,
            prec1 as slimb_t,
        ) == 0)
        {
            break;
        }
        /* and more precision and retry */
        ziv_extra_bits = ziv_extra_bits.wrapping_add(ziv_extra_bits.wrapping_div(2 as i32 as u64))
    }
    return bf_round(T, prec, flags);
}
unsafe extern "C" fn bf_const_free(mut c: *mut BFConstCache) {
    bf_delete(&mut (*c).val);
    (c as *mut u8).write_bytes(0, std::mem::size_of::<BFConstCache>());
}
#[no_mangle]
pub unsafe extern "C" fn bf_const_log2(
    mut T: *mut bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut s: *mut bf_context_t = (*T).ctx;
    return bf_const_get(
        T,
        prec,
        flags,
        &mut (*s).log2_cache,
        Some(bf_const_log2_internal as unsafe extern "C" fn(_: *mut bf_t, _: limb_t) -> ()),
        0 as i32,
    );
}
/* return rounded pi * (1 - 2 * sign) */
unsafe extern "C" fn bf_const_pi_signed(
    mut T: *mut bf_t,
    mut sign: i32,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut s: *mut bf_context_t = (*T).ctx;
    return bf_const_get(
        T,
        prec,
        flags,
        &mut (*s).pi_cache,
        Some(bf_const_pi_internal as unsafe extern "C" fn(_: *mut bf_t, _: limb_t) -> ()),
        sign,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bf_const_pi(
    mut T: *mut bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_const_pi_signed(T, 0 as i32, prec, flags);
}
#[no_mangle]
pub unsafe extern "C" fn bf_clear_cache(mut s: *mut bf_context_t) {
    fft_clear_cache(s);
    bf_const_free(&mut (*s).log2_cache);
    bf_const_free(&mut (*s).pi_cache);
}
unsafe extern "C" fn bf_ziv_rounding(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut f: Option<ZivFunc>,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    let mut rnd_mode: i32 = 0;
    let mut ret: i32 = 0;
    let mut prec1: slimb_t = 0;
    let mut ziv_extra_bits: slimb_t = 0;
    rnd_mode = (flags & 0x7 as i32 as u32) as i32;
    if rnd_mode == BF_RNDF as i32 {
        /* no need to iterate */
        f.expect("non-null function pointer")(r, a, prec, opaque);
        ret = 0 as i32
    } else {
        ziv_extra_bits = 32 as i32 as slimb_t;
        loop {
            prec1 = prec.wrapping_add(ziv_extra_bits as u64) as slimb_t;
            ret = f.expect("non-null function pointer")(r, a, prec1 as limb_t, opaque);
            if ret & ((1 as i32) << 2 as i32 | (1 as i32) << 3 as i32 | (1 as i32) << 5 as i32) != 0
            {
                //            printf("ziv_extra_bits=%" PRId64 "\n", (int64_t)ziv_extra_bits);
                /* overflow or underflow should never happen because
                it indicates the rounding cannot be done correctly,
                but we do not catch all the cases */
                return ret;
            }
            if ret & (1 as i32) << 4 as i32 == 0 {
                ret = 0 as i32;
                break;
            } else if bf_can_round(r, prec as slimb_t, rnd_mode as bf_rnd_t, prec1) != 0 {
                ret = (1 as i32) << 4 as i32;
                break;
            } else {
                ziv_extra_bits = ziv_extra_bits * 2 as i32 as i64
            }
        }
    }
    if (*r).len == 0 as i32 as u64 {
        return ret;
    } else {
        return __bf_round(r, prec, flags, (*r).len, ret);
    };
}
/* if the result is exact, we can stop */
/* add (1 - 2*e_sign) * 2^e */
unsafe extern "C" fn bf_add_epsilon(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut e: slimb_t,
    mut e_sign: i32,
    mut prec: limb_t,
    mut flags: i32,
) -> i32 {
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut ret: i32 = 0;
    /* small argument case: result = 1 + epsilon * sign(x) */
    bf_init((*a).ctx, T);
    bf_set_ui(T, 1 as i32 as uint64_t);
    (*T).sign = e_sign;
    (*T).expn += e;
    ret = bf_add(r, r, T, prec, flags as bf_flags_t);
    bf_delete(T);
    return ret;
}
/* Compute the exponential using faithful rounding at precision 'prec'.
Note: the algorithm is from MPFR */
unsafe extern "C" fn bf_exp_internal(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut n: slimb_t = 0;
    let mut K: slimb_t = 0;
    let mut l: slimb_t = 0;
    let mut i: slimb_t = 0;
    let mut prec1: slimb_t = 0;
    if r != a as *mut bf_t {
    } else {
        assert!(r as *const bf_t != a);
    }
    /* argument reduction:
       T = a - n*log(2) with 0 <= T < log(2) and n integer.
    */
    bf_init(s, T);
    if (*a).expn <= -(1 as i32) as i64 {
        /* 0 <= abs(a) <= 0.5 */
        if (*a).sign != 0 {
            n = -(1 as i32) as slimb_t
        } else {
            n = 0 as i32 as slimb_t
        }
    } else {
        bf_const_log2(
            T,
            ((1 as i32) << 6 as i32) as limb_t,
            BF_RNDZ as i32 as bf_flags_t,
        );
        bf_div(
            T,
            a,
            T,
            ((1 as i32) << 6 as i32) as limb_t,
            BF_RNDD as i32 as bf_flags_t,
        );
        bf_get_limb(&mut n, T, 0 as i32);
    }
    K = bf_isqrt(
        prec.wrapping_add(1 as i32 as u64)
            .wrapping_div(2 as i32 as u64),
    ) as slimb_t;
    l = prec
        .wrapping_sub(1 as i32 as u64)
        .wrapping_div(K as u64)
        .wrapping_add(1 as i32 as u64) as slimb_t;
    /* XXX: precision analysis ? */
    prec1 = prec
        .wrapping_add((K + 2 as i32 as i64 * l + 18 as i32 as i64) as u64)
        .wrapping_add(K as u64)
        .wrapping_add(8 as i32 as u64) as slimb_t;
    if (*a).expn > 0 as i32 as i64 {
        prec1 += (*a).expn
    }
    //    printf("n=%ld K=%ld prec1=%ld\n", n, K, prec1);
    bf_const_log2(T, prec1 as limb_t, BF_RNDF as i32 as bf_flags_t);
    bf_mul_si(T, T, n, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    bf_sub(T, a, T, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    /* reduce the range of T */
    bf_mul_2exp(
        T,
        -K,
        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64),
        BF_RNDZ as i32 as bf_flags_t,
    );
    /* Taylor expansion around zero :
     1 + x + x^2/2 + ... + x^n/n!
     = (1 + x * (1 + x/2 * (1 + ... (x/n))))
    */
    let mut U_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut U: *mut bf_t = &mut U_s;
    bf_init(s, U);
    bf_set_ui(r, 1 as i32 as uint64_t);
    i = l;
    while i >= 1 as i32 as i64 {
        bf_set_ui(U, i as uint64_t);
        bf_div(U, T, U, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_mul(r, r, U, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_add_si(
            r,
            r,
            1 as i32 as int64_t,
            prec1 as limb_t,
            BF_RNDN as i32 as bf_flags_t,
        );
        i -= 1
    }
    bf_delete(U);
    bf_delete(T);
    /* undo the range reduction */
    i = 0 as i32 as slimb_t;
    while i < K {
        bf_mul(
            r,
            r,
            r,
            prec1 as limb_t,
            (BF_RNDN as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
        );
        i += 1
    }
    /* undo the argument reduction */
    bf_mul_2exp(
        r,
        n,
        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64),
        (BF_RNDZ as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
    );
    return (1 as i32) << 4 as i32;
}
/* crude overflow and underflow tests for exp(a). a_low <= a <= a_high */
unsafe extern "C" fn check_exp_underflow_overflow(
    mut s: *mut bf_context_t,
    mut r: *mut bf_t,
    mut a_low: *const bf_t,
    mut a_high: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut log2_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut log2: *mut bf_t = &mut log2_s;
    let mut e_min: slimb_t = 0;
    let mut e_max: slimb_t = 0;
    if (*a_high).expn <= 0 as i32 as i64 {
        return 0 as i32;
    }
    e_max = ((1 as i32 as limb_t) << bf_get_exp_bits(flags) - 1 as i32) as slimb_t;
    e_min = -e_max + 3 as i32 as i64;
    if flags & ((1 as i32) << 3 as i32) as u32 != 0 {
        e_min =
            (e_min as u64).wrapping_sub(prec.wrapping_sub(1 as i32 as u64)) as slimb_t as slimb_t
    }
    bf_init(s, T);
    bf_init(s, log2);
    bf_const_log2(
        log2,
        ((1 as i32) << 6 as i32) as limb_t,
        BF_RNDU as i32 as bf_flags_t,
    );
    bf_mul_ui(
        T,
        log2,
        e_max as uint64_t,
        ((1 as i32) << 6 as i32) as limb_t,
        BF_RNDU as i32 as bf_flags_t,
    );
    /* a_low > e_max * log(2) implies exp(a) > e_max */
    if bf_cmp_lt(T, a_low) > 0 as i32 {
        /* overflow */
        bf_delete(T);
        bf_delete(log2);
        return bf_set_overflow(r, 0 as i32, prec, flags);
    }
    /* a_high < (e_min - 2) * log(2) implies exp(a) < (e_min - 2) */
    bf_const_log2(
        log2,
        ((1 as i32) << 6 as i32) as limb_t,
        BF_RNDD as i32 as bf_flags_t,
    );
    bf_mul_si(
        T,
        log2,
        e_min - 2 as i32 as i64,
        ((1 as i32) << 6 as i32) as limb_t,
        BF_RNDD as i32 as bf_flags_t,
    );
    if bf_cmp_lt(a_high, T) != 0 {
        let mut rnd_mode: i32 = (flags & 0x7 as i32 as u32) as i32;
        /* underflow */
        bf_delete(T);
        bf_delete(log2);
        if rnd_mode == BF_RNDU as i32 {
            /* set the smallest value */
            bf_set_ui(r, 1 as i32 as uint64_t);
            (*r).expn = e_min
        } else {
            bf_set_zero(r, 0 as i32);
        }
        return (1 as i32) << 3 as i32 | (1 as i32) << 4 as i32;
    }
    bf_delete(log2);
    bf_delete(T);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bf_exp(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut ret: i32 = 0;
    if r != a as *mut bf_t {
    } else {
        assert!(r as *const bf_t != a);
    }
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            if (*a).sign != 0 {
                bf_set_zero(r, 0 as i32);
            } else {
                bf_set_inf(r, 0 as i32);
            }
        } else {
            bf_set_ui(r, 1 as i32 as uint64_t);
        }
        return 0 as i32;
    }
    ret = check_exp_underflow_overflow(s, r, a, a, prec, flags);
    if ret != 0 {
        return ret;
    }
    if (*a).expn < 0 as i32 as i64 && -(*a).expn as u64 >= prec.wrapping_add(2 as i32 as u64) {
        /* small argument case: result = 1 + epsilon * sign(x) */
        bf_set_ui(r, 1 as i32 as uint64_t);
        return bf_add_epsilon(
            r,
            r,
            prec.wrapping_add(2 as i32 as u64).wrapping_neg() as slimb_t,
            (*a).sign,
            prec,
            flags as i32,
        );
    }
    return bf_ziv_rounding(
        r,
        a,
        prec,
        flags,
        Some(
            bf_exp_internal
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: *mut std::ffi::c_void,
                ) -> i32,
        ),
        0 as *mut std::ffi::c_void,
    );
}
unsafe extern "C" fn bf_log_internal(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut U_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut U: *mut bf_t = &mut U_s;
    let mut V_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut V: *mut bf_t = &mut V_s;
    let mut n: slimb_t = 0;
    let mut prec1: slimb_t = 0;
    let mut l: slimb_t = 0;
    let mut i: slimb_t = 0;
    let mut K: slimb_t = 0;
    if r != a as *mut bf_t {
    } else {
        assert!(r as *const bf_t != a);
    }
    bf_init(s, T);
    /* argument reduction 1 */
    /* T=a*2^n with 2/3 <= T <= 4/3 */
    let mut U_s_0: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut U_0: *mut bf_t = &mut U_s_0;
    bf_set(T, a);
    n = (*T).expn;
    (*T).expn = 0 as i32 as slimb_t;
    /* U= ~ 2/3 */
    bf_init(s, U_0);
    bf_set_ui(U_0, 0xaaaaaaaa as u32 as uint64_t);
    (*U_0).expn = 0 as i32 as slimb_t;
    if bf_cmp_lt(T, U_0) != 0 {
        (*T).expn += 1;
        n -= 1
    }
    bf_delete(U_0);
    //    printf("n=%ld\n", n);
    //    bf_print_str("T", T);
    /* XXX: precision analysis */
    /* number of iterations for argument reduction 2 */
    K = bf_isqrt(
        prec.wrapping_add(1 as i32 as u64)
            .wrapping_div(2 as i32 as u64),
    ) as slimb_t;
    /* order of Taylor expansion */
    l = prec
        .wrapping_div((2 as i32 as i64 * K) as u64)
        .wrapping_add(1 as i32 as u64) as slimb_t;
    /* precision of the intermediate computations */
    prec1 = prec
        .wrapping_add(K as u64)
        .wrapping_add((2 as i32 as i64 * l) as u64)
        .wrapping_add(32 as i32 as u64) as slimb_t;
    bf_init(s, U);
    bf_init(s, V);
    /* Note: cancellation occurs here, so we use more precision (XXX:
    reduce the precision by computing the exact cancellation) */
    bf_add_si(
        T,
        T,
        -(1 as i32) as int64_t,
        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64),
        BF_RNDN as i32 as bf_flags_t,
    );
    /* argument reduction 2 */
    i = 0 as i32 as slimb_t;
    while i < K {
        /* T = T / (1 + sqrt(1 + T)) */
        bf_add_si(
            U,
            T,
            1 as i32 as int64_t,
            prec1 as limb_t,
            BF_RNDN as i32 as bf_flags_t,
        );
        bf_sqrt(V, U, prec1 as limb_t, BF_RNDF as i32 as bf_flags_t);
        bf_add_si(
            U,
            V,
            1 as i32 as int64_t,
            prec1 as limb_t,
            BF_RNDN as i32 as bf_flags_t,
        );
        bf_div(T, T, U, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        i += 1
    }
    let mut Y_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut Y: *mut bf_t = &mut Y_s;
    let mut Y2_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut Y2: *mut bf_t = &mut Y2_s;
    bf_init(s, Y);
    bf_init(s, Y2);
    /* compute ln(1+x) = ln((1+y)/(1-y)) with y=x/(2+x)
       = y + y^3/3 + ... + y^(2*l + 1) / (2*l+1)
       with Y=Y^2
       = y*(1+Y/3+Y^2/5+...) = y*(1+Y*(1/3+Y*(1/5 + ...)))
    */
    bf_add_si(
        Y,
        T,
        2 as i32 as int64_t,
        prec1 as limb_t,
        BF_RNDN as i32 as bf_flags_t,
    );
    bf_div(Y, T, Y, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    bf_mul(Y2, Y, Y, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    bf_set_ui(r, 0 as i32 as uint64_t);
    i = l;
    while i >= 1 as i32 as i64 {
        bf_set_ui(U, 1 as i32 as uint64_t);
        bf_set_ui(V, (2 as i32 as i64 * i + 1 as i32 as i64) as uint64_t);
        bf_div(U, U, V, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_add(r, r, U, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_mul(r, r, Y2, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        i -= 1
    }
    bf_add_si(
        r,
        r,
        1 as i32 as int64_t,
        prec1 as limb_t,
        BF_RNDN as i32 as bf_flags_t,
    );
    bf_mul(r, r, Y, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    bf_delete(Y);
    bf_delete(Y2);
    bf_delete(V);
    bf_delete(U);
    /* multiplication by 2 for the Taylor expansion and undo the
    argument reduction 2*/
    bf_mul_2exp(
        r,
        K + 1 as i32 as i64,
        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64),
        BF_RNDZ as i32 as bf_flags_t,
    );
    /* undo the argument reduction 1 */
    bf_const_log2(T, prec1 as limb_t, BF_RNDF as i32 as bf_flags_t);
    bf_mul_si(T, T, n, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    bf_add(r, r, T, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    bf_delete(T);
    return (1 as i32) << 4 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bf_log(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    if r != a as *mut bf_t {
    } else {
        assert!(r as *const bf_t != a)
    }
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            if (*a).sign != 0 {
                bf_set_nan(r);
                return (1 as i32) << 0 as i32;
            } else {
                bf_set_inf(r, 0 as i32);
                return 0 as i32;
            }
        } else {
            bf_set_inf(r, 1 as i32);
            return 0 as i32;
        }
    }
    if (*a).sign != 0 {
        bf_set_nan(r);
        return (1 as i32) << 0 as i32;
    }
    bf_init(s, T);
    bf_set_ui(T, 1 as i32 as uint64_t);
    if bf_cmp_eq(a, T) != 0 {
        bf_set_zero(r, 0 as i32);
        bf_delete(T);
        return 0 as i32;
    }
    bf_delete(T);
    return bf_ziv_rounding(
        r,
        a,
        prec,
        flags,
        Some(
            bf_log_internal
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: *mut std::ffi::c_void,
                ) -> i32,
        ),
        0 as *mut std::ffi::c_void,
    );
}
/* x and y finite and x > 0 */
unsafe extern "C" fn bf_pow_generic(
    mut r: *mut bf_t,
    mut x: *const bf_t,
    mut prec: limb_t,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut y: *const bf_t = opaque as *const bf_t;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut prec1: limb_t = 0;
    bf_init(s, T);
    /* XXX: proof for the added precision */
    prec1 = prec.wrapping_add(32 as i32 as u64); /* no overflow/underlow test needed */
    bf_log(
        T,
        x,
        prec1,
        (BF_RNDF as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
    );
    bf_mul(
        T,
        T,
        y,
        prec1,
        (BF_RNDF as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
    );
    if bf_is_nan(T) != 0 {
        bf_set_nan(r);
    } else {
        bf_exp_internal(r, T, prec1, 0 as *mut std::ffi::c_void);
    }
    bf_delete(T);
    return (1 as i32) << 4 as i32;
}
/* x and y finite, x > 0, y integer and y fits on one limb */
unsafe extern "C" fn bf_pow_int(
    mut r: *mut bf_t,
    mut x: *const bf_t,
    mut prec: limb_t,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut y: *const bf_t = opaque as *const bf_t;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut prec1: limb_t = 0;
    let mut ret: i32 = 0;
    let mut y1: slimb_t = 0;
    bf_get_limb(&mut y1, y, 0 as i32);
    if y1 < 0 as i32 as i64 {
        y1 = -y1
    }
    /* XXX: proof for the added precision */
    prec1 = prec
        .wrapping_add((ceil_log2(y1 as limb_t) * 2 as i32) as u64)
        .wrapping_add(8 as i32 as u64);
    ret = bf_pow_ui(
        r,
        x,
        if y1 < 0 as i32 as i64 { -y1 } else { y1 } as limb_t,
        prec1,
        (BF_RNDN as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
    );
    if (*y).sign != 0 {
        bf_init(s, T);
        bf_set_ui(T, 1 as i32 as uint64_t);
        ret |= bf_div(
            r,
            T,
            r,
            prec1,
            (BF_RNDN as i32 | (0x3f as i32) << 5 as i32) as bf_flags_t,
        );
        bf_delete(T);
    }
    return ret;
}
/* x must be a finite non zero float. Return TRUE if there is a
floating point number r such as x=r^(2^n) and return this floating
point number 'r'. Otherwise return FALSE and r is undefined. */
unsafe extern "C" fn check_exact_power2n(
    mut r: *mut bf_t,
    mut x: *const bf_t,
    mut n: slimb_t,
) -> BOOL {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut e: slimb_t = 0;
    let mut i: slimb_t = 0;
    let mut er: slimb_t = 0;
    let mut v: limb_t = 0;
    /* x = m*2^e with m odd integer */
    e = bf_get_exp_min(x);
    /* fast check on the exponent */
    if n > (((1 as i32) << 6 as i32) - 1 as i32) as i64 {
        if e != 0 as i32 as i64 {
            return FALSE as i32;
        }
        er = 0 as i32 as slimb_t
    } else {
        if e as u64 & ((1 as i32 as limb_t) << n).wrapping_sub(1 as i32 as u64) != 0 as i32 as u64 {
            return FALSE as i32;
        }
        er = e >> n
    }
    /* every perfect odd square = 1 modulo 8 */
    v = get_bits(
        (*x).tab,
        (*x).len,
        (*x).len
            .wrapping_mul(((1 as i32) << 6 as i32) as u64)
            .wrapping_sub((*x).expn as u64)
            .wrapping_add(e as u64) as slimb_t,
    );
    if v & 7 as i32 as u64 != 1 as i32 as u64 {
        return FALSE as i32;
    }
    bf_init(s, T);
    bf_set(T, x);
    (*T).expn -= e;
    i = 0 as i32 as slimb_t;
    while i < n {
        if i != 0 as i32 as i64 {
            bf_set(T, r);
        }
        if bf_sqrtrem(r, 0 as *mut bf_t, T) != 0 as i32 {
            return FALSE as i32;
        }
        i += 1
    }
    (*r).expn += er;
    return TRUE as i32;
}
/* prec = BF_PREC_INF is accepted for x and y integers and y >= 0 */
#[no_mangle]
pub unsafe extern "C" fn bf_pow(
    mut r: *mut bf_t,
    mut x: *const bf_t,
    mut y: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut T_bits: slimb_t = 0;
    let mut e: slimb_t = 0;
    let mut current_block: u64;
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut ytmp_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut y_is_int: BOOL = 0;
    let mut y_is_odd: BOOL = 0;
    let mut r_sign: i32 = 0;
    let mut ret: i32 = 0;
    let mut rnd_mode: i32 = 0;
    let mut y_emin: slimb_t = 0;
    if (*x).len == 0 as i32 as u64 || (*y).len == 0 as i32 as u64 {
        if (*y).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
            /* pow(x, 0) = 1 */
            bf_set_ui(r, 1 as i32 as uint64_t);
        } else if (*x).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
        } else {
            let mut cmp_x_abs_1: i32 = 0;
            bf_set_ui(r, 1 as i32 as uint64_t);
            cmp_x_abs_1 = bf_cmpu(x, r);
            if cmp_x_abs_1 == 0 as i32
                && flags & ((1 as i32) << 16 as i32) as u32 != 0
                && (*y).expn >= 9223372036854775807 as i64 - 1 as i32 as i64
            {
                bf_set_nan(r);
            } else if !(cmp_x_abs_1 == 0 as i32
                && ((*x).sign == 0 || (*y).expn != 9223372036854775807 as i64))
            {
                if (*y).expn == 9223372036854775807 as i64 {
                    bf_set_nan(r);
                } else if (*y).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
                    if (*y).sign == (cmp_x_abs_1 > 0 as i32) as i32 {
                        bf_set_zero(r, 0 as i32);
                    } else {
                        bf_set_inf(r, 0 as i32);
                    }
                } else {
                    y_emin = bf_get_exp_min(y);
                    y_is_odd = (y_emin == 0 as i32 as i64) as i32;
                    if (*y).sign
                        == ((*x).expn == -(9223372036854775807 as i64) - 1 as i32 as i64) as i32
                    {
                        bf_set_inf(r, y_is_odd & (*x).sign);
                        if (*y).sign != 0 {
                            /* pow(0, y) with y < 0 */
                            return (1 as i32) << 1 as i32;
                        }
                    } else {
                        bf_set_zero(r, y_is_odd & (*x).sign);
                    }
                }
            }
        }
        return 0 as i32;
    }
    bf_init(s, T);
    bf_set(T, x);
    y_emin = bf_get_exp_min(y);
    y_is_int = (y_emin >= 0 as i32 as i64) as i32;
    rnd_mode = (flags & 0x7 as i32 as u32) as i32;
    if (*x).sign != 0 {
        if y_is_int == 0 {
            bf_set_nan(r);
            bf_delete(T);
            return (1 as i32) << 0 as i32;
        }
        y_is_odd = (y_emin == 0 as i32 as i64) as i32;
        r_sign = y_is_odd;
        /* change the directed rounding mode if the sign of the result
        is changed */
        if r_sign != 0 && (rnd_mode == BF_RNDD as i32 || rnd_mode == BF_RNDU as i32) {
            flags ^= 1 as i32 as u32
        }
        bf_neg(T);
    } else {
        r_sign = 0 as i32
    }
    bf_set_ui(r, 1 as i32 as uint64_t);
    if bf_cmp_eq(T, r) != 0 {
        /* abs(x) = 1: nothing more to do */
        ret = 0 as i32
    } else {
        /* check the overflow/underflow cases */
        let mut al_s: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut al: *mut bf_t = &mut al_s;
        let mut ah_s: bf_t = bf_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut ah: *mut bf_t = &mut ah_s;
        let mut precl: limb_t = ((1 as i32) << 6 as i32) as limb_t;
        bf_init(s, al);
        bf_init(s, ah);
        /* compute bounds of log(abs(x)) * y with a low precision */
        /* XXX: compute bf_log() once */
        /* XXX: add a fast test before this slow test */
        bf_log(al, T, precl, BF_RNDD as i32 as bf_flags_t);
        bf_log(ah, T, precl, BF_RNDU as i32 as bf_flags_t);
        bf_mul(al, al, y, precl, (BF_RNDD as i32 ^ (*y).sign) as bf_flags_t);
        bf_mul(ah, ah, y, precl, (BF_RNDU as i32 ^ (*y).sign) as bf_flags_t);
        ret = check_exp_underflow_overflow(s, r, al, ah, prec, flags);
        bf_delete(al);
        bf_delete(ah);
        if !(ret != 0) {
            if y_is_int != 0 {
                T_bits = 0;
                e = 0;
                current_block = 12991220813362298223;
            } else if rnd_mode != BF_RNDF as i32 {
                let mut y1_0: *mut bf_t = 0 as *mut bf_t;
                if y_emin < 0 as i32 as i64 && check_exact_power2n(r, T, -y_emin) != 0 {
                    /* the problem is reduced to a power to an integer */
                    bf_set(T, r);
                    y1_0 = &mut ytmp_s;
                    (*y1_0).tab = (*y).tab;
                    (*y1_0).len = (*y).len;
                    (*y1_0).sign = (*y).sign;
                    (*y1_0).expn = (*y).expn - y_emin;
                    y = y1_0;
                    current_block = 12991220813362298223;
                } else {
                    current_block = 10289029935050646611;
                }
            } else {
                current_block = 10289029935050646611;
            }
            match current_block {
                12991220813362298223 => {
                    T_bits = (*T).expn - bf_get_exp_min(T);
                    if T_bits == 1 as i32 as i64 {
                        /* pow(2^b, y) = 2^(b*y) */
                        bf_mul_si(
                            T,
                            y,
                            (*T).expn - 1 as i32 as i64,
                            ((1 as i32) << 6 as i32) as limb_t,
                            BF_RNDZ as i32 as bf_flags_t,
                        );
                        bf_get_limb(&mut e, T, 0 as i32);
                        bf_set_ui(r, 1 as i32 as uint64_t);
                        ret = bf_mul_2exp(r, e, prec, flags);
                        current_block = 9694412348743700975;
                    } else if prec
                        == ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                            .wrapping_sub(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    {
                        let mut y1: slimb_t = 0;
                        /* specific case for infinite precision (integer case) */
                        bf_get_limb(&mut y1, y, 0 as i32);
                        if (*y).sign == 0 {
                        } else {
                            assert!((*y).sign != 0);
                        }
                        /* x must be an integer, so abs(x) >= 2 */
                        if y1 >= (1 as i32 as slimb_t) << ((1 as i32) << 6 as i32) - 3 as i32 {
                            bf_delete(T); /* no need to track exact results */
                            return bf_set_overflow(
                                r,
                                0 as i32,
                                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                                    .wrapping_sub(2 as i32 as u64)
                                    .wrapping_add(1 as i32 as u64),
                                flags,
                            );
                        }
                        ret = bf_pow_ui(
                            r,
                            T,
                            y1 as limb_t,
                            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                                .wrapping_sub(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64),
                            BF_RNDZ as i32 as bf_flags_t,
                        );
                        current_block = 9694412348743700975;
                    } else {
                        if (*y).expn <= 31 as i32 as i64 {
                            current_block = 2616667235040759262;
                        } else if (*y).sign != 0 {
                            current_block = 10289029935050646611;
                        } else if rnd_mode == BF_RNDF as i32 {
                            current_block = 10289029935050646611;
                        } else {
                            /* see if the result has a chance to be exact:
                               if x=a*2^b (a odd), x^y=a^y*2^(b*y)
                               x^y needs a precision of at least floor_log2(a)*y bits
                            */
                            bf_mul_si(
                                r,
                                y,
                                T_bits - 1 as i32 as i64,
                                ((1 as i32) << 6 as i32) as limb_t,
                                BF_RNDZ as i32 as bf_flags_t,
                            );
                            bf_get_limb(&mut e, r, 0 as i32);
                            if prec < e as u64 {
                                current_block = 10289029935050646611;
                            } else {
                                current_block = 2616667235040759262;
                            }
                        }
                        match current_block {
                            10289029935050646611 => {}
                            _ =>
                            /* small enough power: use exponentiation in all cases */
                            {
                                ret = bf_ziv_rounding(
                                    r,
                                    T,
                                    prec,
                                    flags,
                                    Some(
                                        bf_pow_int
                                            as unsafe extern "C" fn(
                                                _: *mut bf_t,
                                                _: *const bf_t,
                                                _: limb_t,
                                                _: *mut std::ffi::c_void,
                                            )
                                                -> i32,
                                    ),
                                    y as *mut std::ffi::c_void,
                                );
                                current_block = 9694412348743700975;
                            }
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                9694412348743700975 => {}
                _ =>
                /* cannot be exact */
                {
                    ret = bf_ziv_rounding(
                        r,
                        T,
                        prec,
                        flags,
                        Some(
                            bf_pow_generic
                                as unsafe extern "C" fn(
                                    _: *mut bf_t,
                                    _: *const bf_t,
                                    _: limb_t,
                                    _: *mut std::ffi::c_void,
                                ) -> i32,
                        ),
                        y as *mut std::ffi::c_void,
                    )
                }
            }
        }
    }
    bf_delete(T);
    (*r).sign = r_sign;
    return ret;
}
/* compute sqrt(-2*x-x^2) to get |sin(x)| from cos(x) - 1. */
unsafe extern "C" fn bf_sqrt_sin(mut r: *mut bf_t, mut x: *const bf_t, mut prec1: limb_t) {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    bf_init(s, T);
    bf_set(T, x);
    bf_mul(r, T, T, prec1, BF_RNDN as i32 as bf_flags_t);
    bf_mul_2exp(
        T,
        1 as i32 as slimb_t,
        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64),
        BF_RNDZ as i32 as bf_flags_t,
    );
    bf_add(T, T, r, prec1, BF_RNDN as i32 as bf_flags_t);
    bf_neg(T);
    bf_sqrt(r, T, prec1, BF_RNDF as i32 as bf_flags_t);
    bf_delete(T);
}
unsafe extern "C" fn bf_sincos(
    mut s: *mut bf_t,
    mut c: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
) -> i32 {
    let mut s1: *mut bf_context_t = (*a).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut U_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut U: *mut bf_t = &mut U_s;
    let mut r_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut r: *mut bf_t = &mut r_s;
    let mut K: slimb_t = 0;
    let mut prec1: slimb_t = 0;
    let mut i: slimb_t = 0;
    let mut l: slimb_t = 0;
    let mut mod_0: slimb_t = 0;
    let mut prec2: slimb_t = 0;
    let mut is_neg: i32 = 0;
    if c != a as *mut bf_t && s != a as *mut bf_t {
    } else {
        assert!(c as *const bf_t != a && s as *const bf_t != a);
    }
    bf_init(s1, T);
    bf_init(s1, U);
    bf_init(s1, r);
    /* XXX: precision analysis */
    K = bf_isqrt(prec.wrapping_div(2 as i32 as u64)) as slimb_t;
    l = prec
        .wrapping_div((2 as i32 as i64 * K) as u64)
        .wrapping_add(1 as i32 as u64) as slimb_t;
    prec1 = prec
        .wrapping_add((2 as i32 as i64 * K) as u64)
        .wrapping_add(l as u64)
        .wrapping_add(8 as i32 as u64) as slimb_t;
    /* after the modulo reduction, -pi/4 <= T <= pi/4 */
    if (*a).expn <= -(1 as i32) as i64 {
        /* abs(a) <= 0.25: no modulo reduction needed */
        bf_set(T, a);
        mod_0 = 0 as i32 as slimb_t
    } else {
        let mut cancel: slimb_t = 0;
        cancel = 0 as i32 as slimb_t;
        loop {
            prec2 = prec1 + (*a).expn + cancel;
            bf_const_pi(U, prec2 as limb_t, BF_RNDF as i32 as bf_flags_t);
            bf_mul_2exp(
                U,
                -(1 as i32) as slimb_t,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                BF_RNDZ as i32 as bf_flags_t,
            );
            bf_remquo(
                &mut mod_0,
                T,
                a,
                U,
                prec2 as limb_t,
                BF_RNDN as i32 as bf_flags_t,
                BF_RNDN as i32,
            );
            //            printf("T.expn=%ld prec2=%ld\n", T->expn, prec2);
            if mod_0 == 0 as i32 as i64
                || (*T).expn != -(9223372036854775807 as i64) - 1 as i32 as i64
                    && (*T).expn + prec2 >= prec1 - 1 as i32 as i64
            {
                break;
            }
            /* increase the number of bits until the precision is good enough */
            cancel = bf_max(
                -(*T).expn,
                (cancel + 1 as i32 as i64) * 3 as i32 as i64 / 2 as i32 as i64,
            )
        }
        mod_0 &= 3 as i32 as i64
    }
    is_neg = (*T).sign;
    /* compute cosm1(x) = cos(x) - 1 */
    bf_mul(T, T, T, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    bf_mul_2exp(
        T,
        -(2 as i32) as i64 * K,
        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64),
        BF_RNDZ as i32 as bf_flags_t,
    );
    /* Taylor expansion:
       -x^2/2 + x^4/4! - x^6/6! + ...
    */
    bf_set_ui(r, 1 as i32 as uint64_t);
    i = l;
    while i >= 1 as i32 as i64 {
        bf_set_ui(U, (2 as i32 as i64 * i - 1 as i32 as i64) as uint64_t);
        bf_mul_ui(
            U,
            U,
            (2 as i32 as i64 * i) as uint64_t,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        bf_div(U, T, U, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_mul(r, r, U, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_neg(r);
        if i != 1 as i32 as i64 {
            bf_add_si(
                r,
                r,
                1 as i32 as int64_t,
                prec1 as limb_t,
                BF_RNDN as i32 as bf_flags_t,
            );
        }
        i -= 1
    }
    bf_delete(U);
    /* undo argument reduction:
       cosm1(2*x)= 2*(2*cosm1(x)+cosm1(x)^2)
    */
    i = 0 as i32 as slimb_t;
    while i < K {
        bf_mul(T, r, r, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_mul_2exp(
            r,
            1 as i32 as slimb_t,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        bf_add(r, r, T, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_mul_2exp(
            r,
            1 as i32 as slimb_t,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        i += 1
    }
    bf_delete(T);
    if !c.is_null() {
        if mod_0 & 1 as i32 as i64 == 0 as i32 as i64 {
            bf_add_si(
                c,
                r,
                1 as i32 as int64_t,
                prec1 as limb_t,
                BF_RNDN as i32 as bf_flags_t,
            );
        } else {
            bf_sqrt_sin(c, r, prec1 as limb_t);
            (*c).sign = is_neg ^ 1 as i32
        }
        (*c).sign = ((*c).sign as i64 ^ mod_0 >> 1 as i32) as i32
    }
    if !s.is_null() {
        if mod_0 & 1 as i32 as i64 == 0 as i32 as i64 {
            bf_sqrt_sin(s, r, prec1 as limb_t);
            (*s).sign = is_neg
        } else {
            bf_add_si(
                s,
                r,
                1 as i32 as int64_t,
                prec1 as limb_t,
                BF_RNDN as i32 as bf_flags_t,
            );
        }
        (*s).sign = ((*s).sign as i64 ^ mod_0 >> 1 as i32) as i32
    }
    bf_delete(r);
    return (1 as i32) << 4 as i32;
}
unsafe extern "C" fn bf_cos_internal(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    return bf_sincos(0 as *mut bf_t, r, a, prec);
}
#[no_mangle]
pub unsafe extern "C" fn bf_cos(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            bf_set_nan(r);
            return (1 as i32) << 0 as i32;
        } else {
            bf_set_ui(r, 1 as i32 as uint64_t);
            return 0 as i32;
        }
    }
    /* small argument case: result = 1+r(x) with r(x) = -x^2/2 +
    O(X^4). We assume r(x) < 2^(2*EXP(x) - 1). */
    if (*a).expn < 0 as i32 as i64 {
        let mut e: slimb_t = 0;
        e = 2 as i32 as i64 * (*a).expn - 1 as i32 as i64;
        if (e as u64) < prec.wrapping_add(2 as i32 as u64).wrapping_neg() {
            bf_set_ui(r, 1 as i32 as uint64_t);
            return bf_add_epsilon(r, r, e, 1 as i32, prec, flags as i32);
        }
    }
    return bf_ziv_rounding(
        r,
        a,
        prec,
        flags,
        Some(
            bf_cos_internal
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: *mut std::ffi::c_void,
                ) -> i32,
        ),
        0 as *mut std::ffi::c_void,
    );
}
unsafe extern "C" fn bf_sin_internal(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    return bf_sincos(r, 0 as *mut bf_t, a, prec);
}
#[no_mangle]
pub unsafe extern "C" fn bf_sin(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            bf_set_nan(r);
            return (1 as i32) << 0 as i32;
        } else {
            bf_set_zero(r, (*a).sign);
            return 0 as i32;
        }
    }
    /* small argument case: result = x+r(x) with r(x) = -x^3/6 +
    O(X^5). We assume r(x) < 2^(3*EXP(x) - 2). */
    if (*a).expn < 0 as i32 as i64 {
        let mut e: slimb_t = 0;
        e = sat_add(2 as i32 as i64 * (*a).expn, (*a).expn - 2 as i32 as i64);
        if e < (*a).expn
            - bf_max(
                prec.wrapping_add(2 as i32 as u64) as slimb_t,
                (*a).len
                    .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                    .wrapping_add(2 as i32 as u64) as slimb_t,
            )
        {
            bf_set(r, a);
            return bf_add_epsilon(r, r, e, 1 as i32 - (*a).sign, prec, flags as i32);
        }
    }
    return bf_ziv_rounding(
        r,
        a,
        prec,
        flags,
        Some(
            bf_sin_internal
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: *mut std::ffi::c_void,
                ) -> i32,
        ),
        0 as *mut std::ffi::c_void,
    );
}
unsafe extern "C" fn bf_tan_internal(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut prec1: limb_t = 0;
    /* XXX: precision analysis */
    prec1 = prec.wrapping_add(8 as i32 as u64);
    bf_init(s, T);
    bf_sincos(r, T, a, prec1);
    bf_div(r, r, T, prec1, BF_RNDF as i32 as bf_flags_t);
    bf_delete(T);
    return (1 as i32) << 4 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bf_tan(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    if r != a as *mut bf_t {
    } else {
        assert!(r as *const bf_t != a);
    }
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            bf_set_nan(r);
            return (1 as i32) << 0 as i32;
        } else {
            bf_set_zero(r, (*a).sign);
            return 0 as i32;
        }
    }
    /* small argument case: result = x+r(x) with r(x) = x^3/3 +
    O(X^5). We assume r(x) < 2^(3*EXP(x) - 1). */
    if (*a).expn < 0 as i32 as i64 {
        let mut e: slimb_t = 0;
        e = sat_add(2 as i32 as i64 * (*a).expn, (*a).expn - 1 as i32 as i64);
        if e < (*a).expn
            - bf_max(
                prec.wrapping_add(2 as i32 as u64) as slimb_t,
                (*a).len
                    .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                    .wrapping_add(2 as i32 as u64) as slimb_t,
            )
        {
            bf_set(r, a);
            return bf_add_epsilon(r, r, e, (*a).sign, prec, flags as i32);
        }
    }
    return bf_ziv_rounding(
        r,
        a,
        prec,
        flags,
        Some(
            bf_tan_internal
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: *mut std::ffi::c_void,
                ) -> i32,
        ),
        0 as *mut std::ffi::c_void,
    );
}
/* if add_pi2 is true, add pi/2 to the result (used for acos(x) to
avoid cancellation) */
unsafe extern "C" fn bf_atan_internal(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut add_pi2: BOOL = opaque as intptr_t as BOOL;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut U_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut U: *mut bf_t = &mut U_s;
    let mut V_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut V: *mut bf_t = &mut V_s;
    let mut X2_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut X2: *mut bf_t = &mut X2_s;
    let mut cmp_1: i32 = 0;
    let mut prec1: slimb_t = 0;
    let mut i: slimb_t = 0;
    let mut K: slimb_t = 0;
    let mut l: slimb_t = 0;
    /* XXX: precision analysis */
    K = bf_isqrt(
        prec.wrapping_add(1 as i32 as u64)
            .wrapping_div(2 as i32 as u64),
    ) as slimb_t;
    l = prec
        .wrapping_div((2 as i32 as i64 * K) as u64)
        .wrapping_add(1 as i32 as u64) as slimb_t;
    prec1 = prec
        .wrapping_add(K as u64)
        .wrapping_add((2 as i32 as i64 * l) as u64)
        .wrapping_add(32 as i32 as u64) as slimb_t;
    //    printf("prec=%d K=%d l=%d prec1=%d\n", (int)prec, (int)K, (int)l, (int)prec1);
    bf_init(s, T); /* a >= 1 */
    cmp_1 = ((*a).expn >= 1 as i32 as i64) as i32;
    if cmp_1 != 0 {
        bf_set_ui(T, 1 as i32 as uint64_t);
        bf_div(T, T, a, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    } else {
        bf_set(T, a);
    }
    /* abs(T) <= 1 */
    /* argument reduction */
    bf_init(s, U);
    bf_init(s, V);
    bf_init(s, X2);
    i = 0 as i32 as slimb_t;
    while i < K {
        /* T = T / (1 + sqrt(1 + T^2)) */
        bf_mul(U, T, T, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_add_si(
            U,
            U,
            1 as i32 as int64_t,
            prec1 as limb_t,
            BF_RNDN as i32 as bf_flags_t,
        );
        bf_sqrt(V, U, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_add_si(
            V,
            V,
            1 as i32 as int64_t,
            prec1 as limb_t,
            BF_RNDN as i32 as bf_flags_t,
        );
        bf_div(T, T, V, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        i += 1
    }
    /* Taylor series:
       x - x^3/3 + ... + (-1)^ l * y^(2*l + 1) / (2*l+1)
    */
    bf_mul(X2, T, T, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    bf_set_ui(r, 0 as i32 as uint64_t);
    i = l;
    while i >= 1 as i32 as i64 {
        bf_set_si(U, 1 as i32 as int64_t);
        bf_set_ui(V, (2 as i32 as i64 * i + 1 as i32 as i64) as uint64_t);
        bf_div(U, U, V, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_neg(r);
        bf_add(r, r, U, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        bf_mul(r, r, X2, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
        i -= 1
    }
    bf_neg(r);
    bf_add_si(
        r,
        r,
        1 as i32 as int64_t,
        prec1 as limb_t,
        BF_RNDN as i32 as bf_flags_t,
    );
    bf_mul(r, r, T, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    /* undo the argument reduction */
    bf_mul_2exp(
        r,
        K,
        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64),
        BF_RNDZ as i32 as bf_flags_t,
    );
    bf_delete(U);
    bf_delete(V);
    bf_delete(X2);
    i = add_pi2 as slimb_t;
    if cmp_1 > 0 as i32 {
        /* undo the inversion : r = sign(a)*PI/2 - r */
        bf_neg(r);
        i += (1 as i32 - 2 as i32 * (*a).sign) as i64
    }
    /* add i*(pi/2) with -1 <= i <= 2 */
    if i != 0 as i32 as i64 {
        bf_const_pi(T, prec1 as limb_t, BF_RNDF as i32 as bf_flags_t);
        if i != 2 as i32 as i64 {
            bf_mul_2exp(
                T,
                -(1 as i32) as slimb_t,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                BF_RNDZ as i32 as bf_flags_t,
            );
        }
        (*T).sign = (i < 0 as i32 as i64) as i32;
        bf_add(r, T, r, prec1 as limb_t, BF_RNDN as i32 as bf_flags_t);
    }
    bf_delete(T);
    return (1 as i32) << 4 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bf_atan(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut res: i32 = 0;
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            /* -PI/2 or PI/2 */
            bf_const_pi_signed(r, (*a).sign, prec, flags);
            bf_mul_2exp(
                r,
                -(1 as i32) as slimb_t,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                BF_RNDZ as i32 as bf_flags_t,
            );
            return (1 as i32) << 4 as i32;
        } else {
            bf_set_zero(r, (*a).sign);
            return 0 as i32;
        }
    }
    bf_init(s, T);
    bf_set_ui(T, 1 as i32 as uint64_t);
    res = bf_cmpu(a, T);
    bf_delete(T);
    if res == 0 as i32 {
        /* short cut: abs(a) == 1 -> +/-pi/4 */
        bf_const_pi_signed(r, (*a).sign, prec, flags);
        bf_mul_2exp(
            r,
            -(2 as i32) as slimb_t,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        return (1 as i32) << 4 as i32;
    }
    /* small argument case: result = x+r(x) with r(x) = -x^3/3 +
    O(X^5). We assume r(x) < 2^(3*EXP(x) - 1). */
    if (*a).expn < 0 as i32 as i64 {
        let mut e: slimb_t = 0;
        e = sat_add(2 as i32 as i64 * (*a).expn, (*a).expn - 1 as i32 as i64);
        if e < (*a).expn
            - bf_max(
                prec.wrapping_add(2 as i32 as u64) as slimb_t,
                (*a).len
                    .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                    .wrapping_add(2 as i32 as u64) as slimb_t,
            )
        {
            bf_set(r, a);
            return bf_add_epsilon(r, r, e, 1 as i32 - (*a).sign, prec, flags as i32);
        }
    }
    return bf_ziv_rounding(
        r,
        a,
        prec,
        flags,
        Some(
            bf_atan_internal
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: *mut std::ffi::c_void,
                ) -> i32,
        ),
        0 as *mut std::ffi::c_void,
    );
}
unsafe extern "C" fn bf_atan2_internal(
    mut r: *mut bf_t,
    mut y: *const bf_t,
    mut prec: limb_t,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut x: *const bf_t = opaque as *const bf_t;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut prec1: limb_t = 0;
    let mut ret: i32 = 0;
    if (*y).expn == 9223372036854775807 as i64 || (*x).expn == 9223372036854775807 as i64 {
        bf_set_nan(r);
        return 0 as i32;
    }
    /* compute atan(y/x) assumming inf/inf = 1 and 0/0 = 0 */
    bf_init(s, T);
    prec1 = prec.wrapping_add(32 as i32 as u64);
    if (*y).expn == 9223372036854775807 as i64 - 1 as i32 as i64
        && (*x).expn == 9223372036854775807 as i64 - 1 as i32 as i64
    {
        bf_set_ui(T, 1 as i32 as uint64_t);
        (*T).sign = (*y).sign ^ (*x).sign
    } else if (*y).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
        && (*x).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
    {
        bf_set_zero(T, (*y).sign ^ (*x).sign);
    } else {
        bf_div(T, y, x, prec1, BF_RNDF as i32 as bf_flags_t);
    }
    ret = bf_atan(r, T, prec1, BF_RNDF as i32 as bf_flags_t);
    if (*x).sign != 0 {
        /* if x < 0 (it includes -0), return sign(y)*pi + atan(y/x) */
        bf_const_pi(T, prec1, BF_RNDF as i32 as bf_flags_t);
        (*T).sign = (*y).sign;
        bf_add(r, r, T, prec1, BF_RNDN as i32 as bf_flags_t);
        ret |= (1 as i32) << 4 as i32
    }
    bf_delete(T);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bf_atan2(
    mut r: *mut bf_t,
    mut y: *const bf_t,
    mut x: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_ziv_rounding(
        r,
        y,
        prec,
        flags,
        Some(
            bf_atan2_internal
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: *mut std::ffi::c_void,
                ) -> i32,
        ),
        x as *mut std::ffi::c_void,
    );
}
unsafe extern "C" fn bf_asin_internal(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut is_acos: BOOL = opaque as intptr_t as BOOL;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut prec1: limb_t = 0;
    let mut prec2: limb_t = 0;
    /* asin(x) = atan(x/sqrt(1-x^2))
    acos(x) = pi/2 - asin(x) */
    prec1 = prec.wrapping_add(8 as i32 as u64);
    /* increase the precision in x^2 to compensate the cancellation in
    (1-x^2) if x is close to 1 */
    /* XXX: use less precision when possible */
    if (*a).expn >= 0 as i32 as i64 {
        prec2 = ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64)
    } else {
        prec2 = prec1
    }
    bf_init(s, T);
    bf_mul(T, a, a, prec2, BF_RNDN as i32 as bf_flags_t);
    bf_neg(T);
    bf_add_si(
        T,
        T,
        1 as i32 as int64_t,
        prec2,
        BF_RNDN as i32 as bf_flags_t,
    );
    bf_sqrt(r, T, prec1, BF_RNDN as i32 as bf_flags_t);
    bf_div(T, a, r, prec1, BF_RNDN as i32 as bf_flags_t);
    if is_acos != 0 {
        bf_neg(T);
    }
    bf_atan_internal(r, T, prec1, is_acos as intptr_t as *mut std::ffi::c_void);
    bf_delete(T);
    return (1 as i32) << 4 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bf_asin(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut res: i32 = 0;
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            bf_set_nan(r);
            return (1 as i32) << 0 as i32;
        } else {
            bf_set_zero(r, (*a).sign);
            return 0 as i32;
        }
    }
    bf_init(s, T);
    bf_set_ui(T, 1 as i32 as uint64_t);
    res = bf_cmpu(a, T);
    bf_delete(T);
    if res > 0 as i32 {
        bf_set_nan(r);
        return (1 as i32) << 0 as i32;
    }
    /* small argument case: result = x+r(x) with r(x) = x^3/6 +
    O(X^5). We assume r(x) < 2^(3*EXP(x) - 2). */
    if (*a).expn < 0 as i32 as i64 {
        let mut e: slimb_t = 0;
        e = sat_add(2 as i32 as i64 * (*a).expn, (*a).expn - 2 as i32 as i64);
        if e < (*a).expn
            - bf_max(
                prec.wrapping_add(2 as i32 as u64) as slimb_t,
                (*a).len
                    .wrapping_mul(((1 as i32) << 6 as i32) as u64)
                    .wrapping_add(2 as i32 as u64) as slimb_t,
            )
        {
            bf_set(r, a);
            return bf_add_epsilon(r, r, e, (*a).sign, prec, flags as i32);
        }
    }
    return bf_ziv_rounding(
        r,
        a,
        prec,
        flags,
        Some(
            bf_asin_internal
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: *mut std::ffi::c_void,
                ) -> i32,
        ),
        0 as *mut std::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bf_acos(
    mut r: *mut bf_t,
    mut a: *const bf_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut T_s: bf_t = bf_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut T: *mut bf_t = &mut T_s;
    let mut res: i32 = 0;
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bf_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            bf_set_nan(r);
            return (1 as i32) << 0 as i32;
        } else {
            bf_const_pi(r, prec, flags);
            bf_mul_2exp(
                r,
                -(1 as i32) as slimb_t,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                BF_RNDZ as i32 as bf_flags_t,
            );
            return (1 as i32) << 4 as i32;
        }
    }
    bf_init(s, T);
    bf_set_ui(T, 1 as i32 as uint64_t);
    res = bf_cmpu(a, T);
    bf_delete(T);
    if res > 0 as i32 {
        bf_set_nan(r);
        return (1 as i32) << 0 as i32;
    } else {
        if res == 0 as i32 && (*a).sign == 0 as i32 {
            bf_set_zero(r, 0 as i32);
            return 0 as i32;
        }
    }
    return bf_ziv_rounding(
        r,
        a,
        prec,
        flags,
        Some(
            bf_asin_internal
                as unsafe extern "C" fn(
                    _: *mut bf_t,
                    _: *const bf_t,
                    _: limb_t,
                    _: *mut std::ffi::c_void,
                ) -> i32,
        ),
        TRUE as i32 as *mut std::ffi::c_void,
    );
}
#[inline]
unsafe extern "C" fn shld(mut a1: limb_t, mut a0: limb_t, mut shift: i64) -> limb_t {
    if shift != 0 as i32 as i64 {
        return a1 << shift | a0 >> ((1 as i32) << 6 as i32) as i64 - shift;
    } else {
        return a1;
    };
}
#[inline]
unsafe extern "C" fn fast_udiv(mut a: limb_t, mut s: *const FastDivData) -> limb_t {
    let mut t0: limb_t = 0;
    let mut t1: limb_t = 0;
    let mut __t: u128 = 0;
    __t = ((*s).m1 as u128).wrapping_mul(a as u128);
    t0 = __t as limb_t;
    t1 = (__t >> 64 as i32) as limb_t;
    t0 = a.wrapping_sub(t1) >> (*s).shift1 as i32;
    return t1.wrapping_add(t0) >> (*s).shift2 as i32;
}
/* contains 10^i */
#[no_mangle]
pub static mut mp_pow_dec: [limb_t; 20] = [
    1 as u32 as limb_t,
    10 as u32 as limb_t,
    100 as u32 as limb_t,
    1000 as u32 as limb_t,
    10000 as u32 as limb_t,
    100000 as u32 as limb_t,
    1000000 as u32 as limb_t,
    10000000 as u32 as limb_t,
    100000000 as u32 as limb_t,
    1000000000 as u32 as limb_t,
    10000000000 as u64,
    100000000000 as u64,
    1000000000000 as u64,
    10000000000000 as u64,
    100000000000000 as u64,
    1000000000000000 as u64,
    10000000000000000 as u64,
    100000000000000000 as u64,
    1000000000000000000 as u64,
    10000000000000000000 as u64,
];
/* precomputed from fast_udiv_init(10^i) */
static mut mp_pow_div: [FastDivData; 20] = [
    {
        let mut init = FastDivData {
            m1: 0x1 as i32 as limb_t,
            shift1: 0 as i32 as int8_t,
            shift2: 0 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x999999999999999a as u64,
            shift1: 1 as i32 as int8_t,
            shift2: 3 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x47ae147ae147ae15 as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 6 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x624dd2f1a9fbe77 as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 9 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0xa36e2eb1c432ca58 as u64,
            shift1: 1 as i32 as int8_t,
            shift2: 13 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x4f8b588e368f0847 as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 16 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0xc6f7a0b5ed8d36c as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 19 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0xad7f29abcaf48579 as u64,
            shift1: 1 as i32 as int8_t,
            shift2: 23 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x5798ee2308c39dfa as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 26 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x12e0be826d694b2f as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 29 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0xb7cdfd9d7bdbab7e as u64,
            shift1: 1 as i32 as int8_t,
            shift2: 33 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x5fd7fe17964955fe as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 36 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x19799812dea11198 as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 39 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0xc25c268497681c27 as u64,
            shift1: 1 as i32 as int8_t,
            shift2: 43 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x6849b86a12b9b01f as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 46 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x203af9ee756159b3 as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 49 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0xcd2b297d889bc2b7 as u64,
            shift1: 1 as i32 as int8_t,
            shift2: 53 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x70ef54646d496893 as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 56 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0x2725dd1d243aba0f as i64 as limb_t,
            shift1: 1 as i32 as int8_t,
            shift2: 59 as i32 as int8_t,
        };
        init
    },
    {
        let mut init = FastDivData {
            m1: 0xd83c94fb6d2ac34d as u64,
            shift1: 1 as i32 as int8_t,
            shift2: 63 as i32 as int8_t,
        };
        init
    },
];
/* divide by 10^shift with 0 <= shift <= LIMB_DIGITS */
#[inline]
unsafe extern "C" fn fast_shr_dec(mut a: limb_t, mut shift: i32) -> limb_t {
    return fast_udiv(a, &*mp_pow_div.as_ptr().offset(shift as isize));
}
/* division and remainder by 10^shift */
#[no_mangle]
pub unsafe extern "C" fn mp_add_dec(
    mut res: *mut limb_t,
    mut op1: *const limb_t,
    mut op2: *const limb_t,
    mut n: mp_size_t,
    mut carry: limb_t,
) -> limb_t {
    let mut base: limb_t = 10000000000000000000 as u64;
    let mut i: mp_size_t = 0;
    let mut k: limb_t = 0;
    let mut a: limb_t = 0;
    let mut v: limb_t = 0;
    k = carry;
    i = 0 as i32 as mp_size_t;
    while i < n {
        /* XXX: reuse the trick in add_mod */
        v = *op1.offset(i as isize);
        a = v
            .wrapping_add(*op2.offset(i as isize))
            .wrapping_add(k)
            .wrapping_sub(base);
        k = (a <= v) as i32 as limb_t;
        if k == 0 {
            a = (a as u64).wrapping_add(base) as limb_t as limb_t
        }
        *res.offset(i as isize) = a;
        i += 1
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn mp_add_ui_dec(
    mut tab: *mut limb_t,
    mut b: limb_t,
    mut n: mp_size_t,
) -> limb_t {
    let mut base: limb_t = 10000000000000000000 as u64;
    let mut i: mp_size_t = 0;
    let mut k: limb_t = 0;
    let mut a: limb_t = 0;
    let mut v: limb_t = 0;
    k = b;
    i = 0 as i32 as mp_size_t;
    while i < n {
        v = *tab.offset(i as isize);
        a = v.wrapping_add(k).wrapping_sub(base);
        k = (a <= v) as i32 as limb_t;
        if k == 0 {
            a = (a as u64).wrapping_add(base) as limb_t as limb_t
        }
        *tab.offset(i as isize) = a;
        if k == 0 as i32 as u64 {
            break;
        }
        i += 1
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn mp_sub_dec(
    mut res: *mut limb_t,
    mut op1: *const limb_t,
    mut op2: *const limb_t,
    mut n: mp_size_t,
    mut carry: limb_t,
) -> limb_t {
    let mut base: limb_t = 10000000000000000000 as u64;
    let mut i: mp_size_t = 0;
    let mut k: limb_t = 0;
    let mut v: limb_t = 0;
    let mut a: limb_t = 0;
    k = carry;
    i = 0 as i32 as mp_size_t;
    while i < n {
        v = *op1.offset(i as isize);
        a = v.wrapping_sub(*op2.offset(i as isize)).wrapping_sub(k);
        k = (a > v) as i32 as limb_t;
        if k != 0 {
            a = (a as u64).wrapping_add(base) as limb_t as limb_t
        }
        *res.offset(i as isize) = a;
        i += 1
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn mp_sub_ui_dec(
    mut tab: *mut limb_t,
    mut b: limb_t,
    mut n: mp_size_t,
) -> limb_t {
    let mut base: limb_t = 10000000000000000000 as u64;
    let mut i: mp_size_t = 0;
    let mut k: limb_t = 0;
    let mut v: limb_t = 0;
    let mut a: limb_t = 0;
    k = b;
    i = 0 as i32 as mp_size_t;
    while i < n {
        v = *tab.offset(i as isize);
        a = v.wrapping_sub(k);
        k = (a > v) as i32 as limb_t;
        if k != 0 {
            a = (a as u64).wrapping_add(base) as limb_t as limb_t
        }
        *tab.offset(i as isize) = a;
        if k == 0 as i32 as u64 {
            break;
        }
        i += 1
    }
    return k;
}
/* taba[] = taba[] * b + l. 0 <= b, l <= base - 1. Return the high carry */
#[no_mangle]
pub unsafe extern "C" fn mp_mul1_dec(
    mut tabr: *mut limb_t,
    mut taba: *const limb_t,
    mut n: mp_size_t,
    mut b: limb_t,
    mut l: limb_t,
) -> limb_t {
    let mut i: mp_size_t = 0;
    let mut t0: limb_t = 0;
    let mut t1: limb_t = 0;
    let mut r: limb_t = 0;
    i = 0 as i32 as mp_size_t;
    while i < n {
        let mut __t: u128 = 0;
        __t = (*taba.offset(i as isize) as u128).wrapping_mul(b as u128);
        t0 = __t as limb_t;
        t1 = (__t >> 64 as i32) as limb_t;
        let mut __t_0: limb_t = t0;
        t0 = (t0 as u64).wrapping_add(l) as limb_t as limb_t;
        t1 = (t1 as u64).wrapping_add((0 as i32 + (t0 < __t_0) as i32) as u64) as limb_t as limb_t;
        let mut __a0: uint64_t = 0;
        let mut __a1: uint64_t = 0;
        let mut __t0: uint64_t = 0;
        let mut __t1: uint64_t = 0;
        let mut __b: uint64_t = 10000000000000000000 as u64;
        __a0 = t0;
        __a1 = t1;
        __t0 = __a1;
        __t0 = shld(__t0, __a0, 1 as i32 as i64);
        let mut __t_1: u128 = 0;
        __t_1 = (__t0 as u128).wrapping_mul(17014118346046923173 as u64 as u128);
        __t1 = __t_1 as uint64_t;
        l = (__t_1 >> 64 as i32) as limb_t;
        let mut __t_2: u128 = 0;
        __t_2 = (l as u128).wrapping_mul(__b as u128);
        __t0 = __t_2 as uint64_t;
        __t1 = (__t_2 >> 64 as i32) as uint64_t;
        let mut __t_3: limb_t = __a0;
        __a0 = (__a0 as u64).wrapping_sub(__t0) as uint64_t as uint64_t;
        __a1 = (__a1 as u64).wrapping_sub(__t1.wrapping_add((__a0 > __t_3) as i32 as u64))
            as uint64_t as uint64_t;
        let mut __t_4: limb_t = __a0;
        __a0 =
            (__a0 as u64).wrapping_sub(__b.wrapping_mul(2 as i32 as u64)) as uint64_t as uint64_t;
        __a1 = (__a1 as u64).wrapping_sub((1 as i32 + (__a0 > __t_4) as i32) as u64) as uint64_t
            as uint64_t;
        __t0 = (__a1 as slimb_t >> 1 as i32) as uint64_t;
        l = (l as u64).wrapping_add((2 as i32 as u64).wrapping_add(__t0)) as limb_t as limb_t;
        let mut __t_5: limb_t = __a0;
        __a0 = (__a0 as u64).wrapping_add(__b & __t0) as uint64_t as uint64_t;
        __a1 = (__a1 as u64).wrapping_add((0 as i32 + (__a0 < __t_5) as i32) as u64) as uint64_t
            as uint64_t;
        l = (l as u64).wrapping_add(__a1) as limb_t as limb_t;
        __a0 = (__a0 as u64).wrapping_add(__b & __a1) as uint64_t as uint64_t;
        r = __a0;
        *tabr.offset(i as isize) = r;
        i += 1
    }
    return l;
}
/* tabr[] += taba[] * b. 0 <= b <= base - 1. Return the value to add
to the high word */
#[no_mangle]
pub unsafe extern "C" fn mp_add_mul1_dec(
    mut tabr: *mut limb_t,
    mut taba: *const limb_t,
    mut n: mp_size_t,
    mut b: limb_t,
) -> limb_t {
    let mut i: mp_size_t = 0;
    let mut l: limb_t = 0;
    let mut t0: limb_t = 0;
    let mut t1: limb_t = 0;
    let mut r: limb_t = 0;
    l = 0 as i32 as limb_t;
    i = 0 as i32 as mp_size_t;
    while i < n {
        let mut __t: u128 = 0;
        __t = (*taba.offset(i as isize) as u128).wrapping_mul(b as u128);
        t0 = __t as limb_t;
        t1 = (__t >> 64 as i32) as limb_t;
        let mut __t_0: limb_t = t0;
        t0 = (t0 as u64).wrapping_add(l) as limb_t as limb_t;
        t1 = (t1 as u64).wrapping_add((0 as i32 + (t0 < __t_0) as i32) as u64) as limb_t as limb_t;
        let mut __t_1: limb_t = t0;
        t0 = (t0 as u64).wrapping_add(*tabr.offset(i as isize)) as limb_t as limb_t;
        t1 = (t1 as u64).wrapping_add((0 as i32 + (t0 < __t_1) as i32) as u64) as limb_t as limb_t;
        let mut __a0: uint64_t = 0;
        let mut __a1: uint64_t = 0;
        let mut __t0: uint64_t = 0;
        let mut __t1: uint64_t = 0;
        let mut __b: uint64_t = 10000000000000000000 as u64;
        __a0 = t0;
        __a1 = t1;
        __t0 = __a1;
        __t0 = shld(__t0, __a0, 1 as i32 as i64);
        let mut __t_2: u128 = 0;
        __t_2 = (__t0 as u128).wrapping_mul(17014118346046923173 as u64 as u128);
        __t1 = __t_2 as uint64_t;
        l = (__t_2 >> 64 as i32) as limb_t;
        let mut __t_3: u128 = 0;
        __t_3 = (l as u128).wrapping_mul(__b as u128);
        __t0 = __t_3 as uint64_t;
        __t1 = (__t_3 >> 64 as i32) as uint64_t;
        let mut __t_4: limb_t = __a0;
        __a0 = (__a0 as u64).wrapping_sub(__t0) as uint64_t as uint64_t;
        __a1 = (__a1 as u64).wrapping_sub(__t1.wrapping_add((__a0 > __t_4) as i32 as u64))
            as uint64_t as uint64_t;
        let mut __t_5: limb_t = __a0;
        __a0 =
            (__a0 as u64).wrapping_sub(__b.wrapping_mul(2 as i32 as u64)) as uint64_t as uint64_t;
        __a1 = (__a1 as u64).wrapping_sub((1 as i32 + (__a0 > __t_5) as i32) as u64) as uint64_t
            as uint64_t;
        __t0 = (__a1 as slimb_t >> 1 as i32) as uint64_t;
        l = (l as u64).wrapping_add((2 as i32 as u64).wrapping_add(__t0)) as limb_t as limb_t;
        let mut __t_6: limb_t = __a0;
        __a0 = (__a0 as u64).wrapping_add(__b & __t0) as uint64_t as uint64_t;
        __a1 = (__a1 as u64).wrapping_add((0 as i32 + (__a0 < __t_6) as i32) as u64) as uint64_t
            as uint64_t;
        l = (l as u64).wrapping_add(__a1) as limb_t as limb_t;
        __a0 = (__a0 as u64).wrapping_add(__b & __a1) as uint64_t as uint64_t;
        r = __a0;
        *tabr.offset(i as isize) = r;
        i += 1
    }
    return l;
}
/* tabr[] -= taba[] * b. 0 <= b <= base - 1. Return the value to
substract to the high word. */
#[no_mangle]
pub unsafe extern "C" fn mp_sub_mul1_dec(
    mut tabr: *mut limb_t,
    mut taba: *const limb_t,
    mut n: mp_size_t,
    mut b: limb_t,
) -> limb_t {
    let mut base: limb_t = 10000000000000000000 as u64;
    let mut i: mp_size_t = 0;
    let mut l: limb_t = 0;
    let mut t0: limb_t = 0;
    let mut t1: limb_t = 0;
    let mut r: limb_t = 0;
    let mut a: limb_t = 0;
    let mut v: limb_t = 0;
    let mut c: limb_t = 0;
    /* XXX: optimize */
    l = 0 as i32 as limb_t;
    i = 0 as i32 as mp_size_t;
    while i < n {
        let mut __t: u128 = 0;
        __t = (*taba.offset(i as isize) as u128).wrapping_mul(b as u128);
        t0 = __t as limb_t;
        t1 = (__t >> 64 as i32) as limb_t;
        let mut __t_0: limb_t = t0;
        t0 = (t0 as u64).wrapping_add(l) as limb_t as limb_t;
        t1 = (t1 as u64).wrapping_add((0 as i32 + (t0 < __t_0) as i32) as u64) as limb_t as limb_t;
        let mut __a0: uint64_t = 0;
        let mut __a1: uint64_t = 0;
        let mut __t0: uint64_t = 0;
        let mut __t1: uint64_t = 0;
        let mut __b: uint64_t = 10000000000000000000 as u64;
        __a0 = t0;
        __a1 = t1;
        __t0 = __a1;
        __t0 = shld(__t0, __a0, 1 as i32 as i64);
        let mut __t_1: u128 = 0;
        __t_1 = (__t0 as u128).wrapping_mul(17014118346046923173 as u64 as u128);
        __t1 = __t_1 as uint64_t;
        l = (__t_1 >> 64 as i32) as limb_t;
        let mut __t_2: u128 = 0;
        __t_2 = (l as u128).wrapping_mul(__b as u128);
        __t0 = __t_2 as uint64_t;
        __t1 = (__t_2 >> 64 as i32) as uint64_t;
        let mut __t_3: limb_t = __a0;
        __a0 = (__a0 as u64).wrapping_sub(__t0) as uint64_t as uint64_t;
        __a1 = (__a1 as u64).wrapping_sub(__t1.wrapping_add((__a0 > __t_3) as i32 as u64))
            as uint64_t as uint64_t;
        let mut __t_4: limb_t = __a0;
        __a0 =
            (__a0 as u64).wrapping_sub(__b.wrapping_mul(2 as i32 as u64)) as uint64_t as uint64_t;
        __a1 = (__a1 as u64).wrapping_sub((1 as i32 + (__a0 > __t_4) as i32) as u64) as uint64_t
            as uint64_t;
        __t0 = (__a1 as slimb_t >> 1 as i32) as uint64_t;
        l = (l as u64).wrapping_add((2 as i32 as u64).wrapping_add(__t0)) as limb_t as limb_t;
        let mut __t_5: limb_t = __a0;
        __a0 = (__a0 as u64).wrapping_add(__b & __t0) as uint64_t as uint64_t;
        __a1 = (__a1 as u64).wrapping_add((0 as i32 + (__a0 < __t_5) as i32) as u64) as uint64_t
            as uint64_t;
        l = (l as u64).wrapping_add(__a1) as limb_t as limb_t;
        __a0 = (__a0 as u64).wrapping_add(__b & __a1) as uint64_t as uint64_t;
        r = __a0;
        v = *tabr.offset(i as isize);
        a = v.wrapping_sub(r);
        c = (a > v) as i32 as limb_t;
        if c != 0 {
            a = (a as u64).wrapping_add(base) as limb_t as limb_t
        }
        /* never bigger than base because r = 0 when l = base - 1 */
        l = (l as u64).wrapping_add(c) as limb_t as limb_t;
        *tabr.offset(i as isize) = a;
        i += 1
    }
    return l;
}
/* size of the result : op1_size + op2_size. */
#[no_mangle]
pub unsafe extern "C" fn mp_mul_basecase_dec(
    mut result: *mut limb_t,
    mut op1: *const limb_t,
    mut op1_size: mp_size_t,
    mut op2: *const limb_t,
    mut op2_size: mp_size_t,
) {
    let mut i: mp_size_t = 0;
    let mut r: limb_t = 0;
    *result.offset(op1_size as isize) = mp_mul1_dec(
        result,
        op1,
        op1_size,
        *op2.offset(0 as i32 as isize),
        0 as i32 as limb_t,
    );
    i = 1 as i32 as mp_size_t;
    while i < op2_size {
        r = mp_add_mul1_dec(
            result.offset(i as isize),
            op1,
            op1_size,
            *op2.offset(i as isize),
        );
        *result.offset((i + op1_size) as isize) = r;
        i += 1
    }
}
/* taba[] = (taba[] + r*base^na) / b. 0 <= b < base. 0 <= r <
b. Return the remainder. */
#[no_mangle]
pub unsafe extern "C" fn mp_div1_dec(
    mut tabr: *mut limb_t,
    mut taba: *const limb_t,
    mut na: mp_size_t,
    mut b: limb_t,
    mut r: limb_t,
) -> limb_t {
    let mut base: limb_t = 10000000000000000000 as u64;
    let mut i: mp_size_t = 0;
    let mut t0: limb_t = 0;
    let mut t1: limb_t = 0;
    let mut q: limb_t = 0;
    let mut shift: i32 = 0;
    if b == 2 as i32 as u64 {
        let mut base_div2: limb_t = 0;
        /* Note: only works if base is even */
        base_div2 = base >> 1 as i32;
        if r != 0 {
            r = base_div2
        }
        i = na - 1 as i32 as i64;
        while i >= 0 as i32 as i64 {
            t0 = *taba.offset(i as isize);
            *tabr.offset(i as isize) = (t0 >> 1 as i32).wrapping_add(r);
            r = 0 as i32 as limb_t;
            if t0 & 1 as i32 as u64 != 0 {
                r = base_div2
            }
            i -= 1
        }
        if r != 0 {
            r = 1 as i32 as limb_t
        }
    } else if na >= 3 as i32 as i64 {
        shift = clz(b);
        if shift == 0 as i32 {
            /* normalized case: b >= 2^(LIMB_BITS-1) */
            let mut b_inv: limb_t = 0;
            b_inv = udiv1norm_init(b);
            i = na - 1 as i32 as i64;
            while i >= 0 as i32 as i64 {
                let mut __t: u128 = 0;
                __t = (r as u128).wrapping_mul(base as u128);
                t0 = __t as limb_t;
                t1 = (__t >> 64 as i32) as limb_t;
                let mut __t_0: limb_t = t0;
                t0 = (t0 as u64).wrapping_add(*taba.offset(i as isize)) as limb_t as limb_t;
                t1 = (t1 as u64).wrapping_add((0 as i32 + (t0 < __t_0) as i32) as u64) as limb_t
                    as limb_t;
                q = udiv1norm(&mut r, t1, t0, b, b_inv);
                *tabr.offset(i as isize) = q;
                i -= 1
            }
        } else {
            let mut b_inv_0: limb_t = 0;
            b <<= shift;
            b_inv_0 = udiv1norm_init(b);
            i = na - 1 as i32 as i64;
            while i >= 0 as i32 as i64 {
                let mut __t_1: u128 = 0;
                __t_1 = (r as u128).wrapping_mul(base as u128);
                t0 = __t_1 as limb_t;
                t1 = (__t_1 >> 64 as i32) as limb_t;
                let mut __t_2: limb_t = t0;
                t0 = (t0 as u64).wrapping_add(*taba.offset(i as isize)) as limb_t as limb_t;
                t1 = (t1 as u64).wrapping_add((0 as i32 + (t0 < __t_2) as i32) as u64) as limb_t
                    as limb_t;
                t1 = t1 << shift | t0 >> ((1 as i32) << 6 as i32) - shift;
                t0 <<= shift;
                q = udiv1norm(&mut r, t1, t0, b, b_inv_0);
                r >>= shift;
                *tabr.offset(i as isize) = q;
                i -= 1
            }
        }
    } else {
        i = na - 1 as i32 as i64;
        while i >= 0 as i32 as i64 {
            let mut __t_3: u128 = 0;
            __t_3 = (r as u128).wrapping_mul(base as u128);
            t0 = __t_3 as limb_t;
            t1 = (__t_3 >> 64 as i32) as limb_t;
            let mut __t_4: limb_t = t0;
            t0 = (t0 as u64).wrapping_add(*taba.offset(i as isize)) as limb_t as limb_t;
            t1 = (t1 as u64).wrapping_add((0 as i32 + (t0 < __t_4) as i32) as u64) as limb_t
                as limb_t;
            let mut __t_5: u128 = 0;
            let mut __b: limb_t = b;
            __t_5 = (t1 as u128) << 64 as i32 | t0 as u128;
            q = __t_5.wrapping_div(__b as u128) as limb_t;
            r = __t_5.wrapping_rem(__b as u128) as limb_t;
            *tabr.offset(i as isize) = q;
            i -= 1
        }
    }
    return r;
}
//#define DEBUG_DIV_SLOW
/* return q = a / b and r = a % b.

   taba[na] must be allocated if tabb1[nb - 1] < B / 2.  tabb1[nb - 1]
   must be != zero. na must be >= nb. 's' can be NULL if tabb1[nb - 1]
   >= B / 2.

   The remainder is is returned in taba and contains nb libms. tabq
   contains na - nb + 1 limbs. No overlap is permitted.

   Running time of the standard method: (na - nb + 1) * nb
   Return 0 if OK, -1 if memory alloc error
*/
/* XXX: optimize */
unsafe extern "C" fn mp_div_dec(
    mut s: *mut bf_context_t,
    mut tabq: *mut limb_t,
    mut taba: *mut limb_t,
    mut na: mp_size_t,
    mut tabb1: *const limb_t,
    mut nb: mp_size_t,
) -> i32 {
    let mut base: limb_t = 10000000000000000000 as u64;
    let mut r: limb_t = 0;
    let mut mult: limb_t = 0;
    let mut t0: limb_t = 0;
    let mut t1: limb_t = 0;
    let mut a: limb_t = 0;
    let mut c: limb_t = 0;
    let mut q: limb_t = 0;
    let mut v: limb_t = 0;
    let mut tabb: *mut limb_t = 0 as *mut limb_t;
    let mut i: mp_size_t = 0;
    let mut j: mp_size_t = 0;
    let mut static_tabb: [limb_t; 16] = [0; 16];
    /* normalize tabb */
    r = *tabb1.offset((nb - 1 as i32 as i64) as isize);
    if r != 0 as i32 as u64 {
    } else {
        assert!(r != 0);
    }
    i = na - nb;
    if r >= (10000000000000000000 as u64).wrapping_div(2 as i32 as u64) {
        mult = 1 as i32 as limb_t;
        tabb = tabb1 as *mut limb_t;
        q = 1 as i32 as limb_t;
        j = nb - 1 as i32 as i64;
        while j >= 0 as i32 as i64 {
            if *taba.offset((i + j) as isize) != *tabb.offset(j as isize) {
                if *taba.offset((i + j) as isize) < *tabb.offset(j as isize) {
                    q = 0 as i32 as limb_t
                }
                break;
            } else {
                j -= 1
            }
        }
        *tabq.offset(i as isize) = q;
        if q != 0 {
            mp_sub_dec(
                taba.offset(i as isize),
                taba.offset(i as isize),
                tabb,
                nb,
                0 as i32 as limb_t,
            );
        }
        i -= 1
    } else {
        mult = base.wrapping_div(r.wrapping_add(1 as i32 as u64));
        if (nb <= 16 as i32 as i64) as i32 as i64 != 0 {
            tabb = static_tabb.as_mut_ptr()
        } else {
            tabb = bf_malloc(
                s,
                (::std::mem::size_of::<limb_t>() as u64).wrapping_mul(nb as u64),
            ) as *mut limb_t;
            if tabb.is_null() {
                return -(1 as i32);
            }
        }
        mp_mul1_dec(tabb, tabb1, nb, mult, 0 as i32 as limb_t);
        *taba.offset(na as isize) = mp_mul1_dec(taba, taba, na, mult, 0 as i32 as limb_t)
    }
    while i >= 0 as i32 as i64 {
        if (*taba.offset((i + nb) as isize) >= *tabb.offset((nb - 1 as i32 as i64) as isize)) as i32
            as i64
            != 0
        {
            /* XXX: check if it is really possible */
            q = base.wrapping_sub(1 as i32 as u64)
        } else {
            let mut __t: u128 = 0;
            __t = (*taba.offset((i + nb) as isize) as u128).wrapping_mul(base as u128);
            t0 = __t as limb_t;
            t1 = (__t >> 64 as i32) as limb_t;
            let mut __t_0: limb_t = t0;
            t0 = (t0 as u64).wrapping_add(*taba.offset((i + nb - 1 as i32 as i64) as isize))
                as limb_t as limb_t;
            t1 = (t1 as u64).wrapping_add((0 as i32 + (t0 < __t_0) as i32) as u64) as limb_t
                as limb_t;
            let mut __t_1: u128 = 0;
            let mut __b: limb_t = *tabb.offset((nb - 1 as i32 as i64) as isize);
            __t_1 = (t1 as u128) << 64 as i32 | t0 as u128;
            q = __t_1.wrapping_div(__b as u128) as limb_t;
            r = __t_1.wrapping_rem(__b as u128) as limb_t
        }
        //        printf("i=%d q1=%ld\n", i, q);
        r = mp_sub_mul1_dec(taba.offset(i as isize), tabb, nb, q);
        //        mp_dump("r1", taba + i, nb, bd);
        //        printf("r2=%ld\n", r);
        v = *taba.offset((i + nb) as isize);
        a = v.wrapping_sub(r);
        c = (a > v) as i32 as limb_t;
        if c != 0 {
            a = (a as u64).wrapping_add(base) as limb_t as limb_t
        }
        *taba.offset((i + nb) as isize) = a;
        if c != 0 as i32 as u64 {
            loop
            /* negative result */
            {
                q = q.wrapping_sub(1);
                c = mp_add_dec(
                    taba.offset(i as isize),
                    taba.offset(i as isize),
                    tabb,
                    nb,
                    0 as i32 as limb_t,
                );
                /* propagate carry and test if positive result */
                if !(c != 0 as i32 as u64) {
                    continue;
                }
                let ref mut fresh8 = *taba.offset((i + nb) as isize);
                *fresh8 = (*fresh8).wrapping_add(1);
                if *fresh8 == base {
                    break;
                }
            }
        }
        *tabq.offset(i as isize) = q;
        i -= 1
    }
    /* remove the normalization */
    if mult != 1 as i32 as u64 {
        mp_div1_dec(taba, taba, nb, mult, 0 as i32 as limb_t);
        if (tabb != static_tabb.as_mut_ptr()) as i32 as i64 != 0 {
            bf_free(s, tabb as *mut std::ffi::c_void);
        }
    }
    return 0 as i32;
}
/* divide by 10^shift */
unsafe extern "C" fn mp_shr_dec(
    mut tab_r: *mut limb_t,
    mut tab: *const limb_t,
    mut n: mp_size_t,
    mut shift: limb_t,
    mut high: limb_t,
) -> limb_t {
    let mut i: mp_size_t = 0;
    let mut l: limb_t = 0;
    let mut a: limb_t = 0;
    let mut q: limb_t = 0;
    let mut r: limb_t = 0;
    if shift >= 1 as i32 as u64 && shift < 19 as i32 as u64 {
    } else {
        assert!(shift >= 1 && shift < LIMB_DIGITS);
    }
    l = high;
    i = n - 1 as i32 as i64;
    while i >= 0 as i32 as i64 {
        a = *tab.offset(i as isize);
        q = fast_shr_dec(a, shift as i32);
        r = a.wrapping_sub(q.wrapping_mul(mp_pow_dec[shift as usize]));
        *tab_r.offset(i as isize) = q.wrapping_add(
            l.wrapping_mul(mp_pow_dec[(19 as i32 as u64).wrapping_sub(shift) as usize]),
        );
        l = r;
        i -= 1
    }
    return l;
}
/* multiply by 10^shift */
unsafe extern "C" fn mp_shl_dec(
    mut tab_r: *mut limb_t,
    mut tab: *const limb_t,
    mut n: mp_size_t,
    mut shift: limb_t,
    mut low: limb_t,
) -> limb_t {
    let mut i: mp_size_t = 0;
    let mut l: limb_t = 0;
    let mut a: limb_t = 0;
    let mut q: limb_t = 0;
    let mut r: limb_t = 0;
    if shift >= 1 as i32 as u64 && shift < 19 as i32 as u64 {
    } else {
        assert!(shift >= 1 && shift < LIMB_DIGITS);
    }
    l = low;
    i = 0 as i32 as mp_size_t;
    while i < n {
        a = *tab.offset(i as isize);
        q = fast_shr_dec(a, (19 as i32 as u64).wrapping_sub(shift) as i32);
        r = a.wrapping_sub(
            q.wrapping_mul(mp_pow_dec[(19 as i32 as u64).wrapping_sub(shift) as usize]),
        );
        *tab_r.offset(i as isize) = r.wrapping_mul(mp_pow_dec[shift as usize]).wrapping_add(l);
        l = q;
        i += 1
    }
    return l;
}
unsafe extern "C" fn mp_sqrtrem2_dec(mut tabs: *mut limb_t, mut taba: *mut limb_t) -> limb_t {
    let mut k: i32 = 0;
    let mut a: dlimb_t = 0;
    let mut b: dlimb_t = 0;
    let mut r: dlimb_t = 0;
    let mut taba1: [limb_t; 2] = [0; 2];
    let mut s: limb_t = 0;
    let mut r0: limb_t = 0;
    let mut r1: limb_t = 0;
    /* convert to binary and normalize */
    a = (*taba.offset(1 as i32 as isize) as dlimb_t)
        .wrapping_mul(10000000000000000000 as u64 as u128)
        .wrapping_add(*taba.offset(0 as i32 as isize) as u128);
    k = clz((a >> ((1 as i32) << 6 as i32)) as limb_t) & !(1 as i32);
    b = a << k;
    taba1[0 as i32 as usize] = b as limb_t;
    taba1[1 as i32 as usize] = (b >> ((1 as i32) << 6 as i32)) as limb_t;
    mp_sqrtrem2(&mut s, taba1.as_mut_ptr());
    s >>= k >> 1 as i32;
    /* convert the remainder back to decimal */
    r = a.wrapping_sub((s as dlimb_t).wrapping_mul(s as dlimb_t));
    let mut __a0: uint64_t = 0;
    let mut __a1: uint64_t = 0;
    let mut __t0: uint64_t = 0;
    let mut __t1: uint64_t = 0;
    let mut __b: uint64_t = 10000000000000000000 as u64;
    __a0 = r as uint64_t;
    __a1 = (r >> ((1 as i32) << 6 as i32)) as uint64_t;
    __t0 = __a1;
    __t0 = shld(__t0, __a0, 1 as i32 as i64);
    let mut __t: u128 = 0;
    __t = (__t0 as u128).wrapping_mul(17014118346046923173 as u64 as u128);
    __t1 = __t as uint64_t;
    r1 = (__t >> 64 as i32) as limb_t;
    let mut __t_0: u128 = 0;
    __t_0 = (r1 as u128).wrapping_mul(__b as u128);
    __t0 = __t_0 as uint64_t;
    __t1 = (__t_0 >> 64 as i32) as uint64_t;
    let mut __t_1: limb_t = __a0;
    __a0 = (__a0 as u64).wrapping_sub(__t0) as uint64_t as uint64_t;
    __a1 = (__a1 as u64).wrapping_sub(__t1.wrapping_add((__a0 > __t_1) as i32 as u64)) as uint64_t
        as uint64_t;
    let mut __t_2: limb_t = __a0;
    __a0 = (__a0 as u64).wrapping_sub(__b.wrapping_mul(2 as i32 as u64)) as uint64_t as uint64_t;
    __a1 = (__a1 as u64).wrapping_sub((1 as i32 + (__a0 > __t_2) as i32) as u64) as uint64_t
        as uint64_t;
    __t0 = (__a1 as slimb_t >> 1 as i32) as uint64_t;
    r1 = (r1 as u64).wrapping_add((2 as i32 as u64).wrapping_add(__t0)) as limb_t as limb_t;
    let mut __t_3: limb_t = __a0;
    __a0 = (__a0 as u64).wrapping_add(__b & __t0) as uint64_t as uint64_t;
    __a1 = (__a1 as u64).wrapping_add((0 as i32 + (__a0 < __t_3) as i32) as u64) as uint64_t
        as uint64_t;
    r1 = (r1 as u64).wrapping_add(__a1) as limb_t as limb_t;
    __a0 = (__a0 as u64).wrapping_add(__b & __a1) as uint64_t as uint64_t;
    r0 = __a0;
    *taba.offset(0 as i32 as isize) = r0;
    *tabs.offset(0 as i32 as isize) = s;
    return r1;
}
//#define DEBUG_SQRTREM_DEC
/* tmp_buf must contain (n / 2 + 1 limbs) */
unsafe extern "C" fn mp_sqrtrem_rec_dec(
    mut tabs: *mut limb_t,
    mut taba: *mut limb_t,
    mut n: limb_t,
    mut tmp_buf: *mut limb_t,
) -> limb_t {
    let mut l: limb_t = 0;
    let mut h: limb_t = 0;
    let mut rh: limb_t = 0;
    let mut ql: limb_t = 0;
    let mut qh: limb_t = 0;
    let mut c: limb_t = 0;
    let mut i: limb_t = 0;
    if n == 1 as i32 as u64 {
        return mp_sqrtrem2_dec(tabs, taba);
    }
    l = n.wrapping_div(2 as i32 as u64);
    h = n.wrapping_sub(l);
    qh = mp_sqrtrem_rec_dec(
        tabs.offset(l as isize),
        taba.offset((2 as i32 as u64).wrapping_mul(l) as isize),
        h,
        tmp_buf,
    );
    /* the remainder is in taba + 2 * l. Its high bit is in qh */
    if qh != 0 {
        mp_sub_dec(
            taba.offset((2 as i32 as u64).wrapping_mul(l) as isize),
            taba.offset((2 as i32 as u64).wrapping_mul(l) as isize),
            tabs.offset(l as isize),
            h as mp_size_t,
            0 as i32 as limb_t,
        );
    }
    /* instead of dividing by 2*s, divide by s (which is normalized)
    and update q and r */
    mp_div_dec(
        0 as *mut bf_context_t,
        tmp_buf,
        taba.offset(l as isize),
        n as mp_size_t,
        tabs.offset(l as isize),
        h as mp_size_t,
    ); /* 0 or 1 */
    qh = (qh as u64).wrapping_add(*tmp_buf.offset(l as isize)) as limb_t as limb_t;
    i = 0 as i32 as limb_t;
    while i < l {
        *tabs.offset(i as isize) = *tmp_buf.offset(i as isize);
        i = i.wrapping_add(1)
    }
    ql = mp_div1_dec(
        tabs,
        tabs,
        l as mp_size_t,
        2 as i32 as limb_t,
        qh & 1 as i32 as u64,
    );
    qh = qh >> 1 as i32;
    if ql != 0 {
        rh = mp_add_dec(
            taba.offset(l as isize),
            taba.offset(l as isize),
            tabs.offset(l as isize),
            h as mp_size_t,
            0 as i32 as limb_t,
        )
    } else {
        rh = 0 as i32 as limb_t
    }
    mp_add_ui_dec(tabs.offset(l as isize), qh, h as mp_size_t);
    /* q = qh, tabs[l - 1 ... 0], r = taba[n - 1 ... l] */
    /* subtract q^2. if qh = 1 then q = B^l, so we can take shortcuts */
    if qh != 0 {
        c = qh
    } else {
        mp_mul_basecase_dec(
            taba.offset(n as isize),
            tabs,
            l as mp_size_t,
            tabs,
            l as mp_size_t,
        );
        c = mp_sub_dec(
            taba,
            taba,
            taba.offset(n as isize),
            (2 as i32 as u64).wrapping_mul(l) as mp_size_t,
            0 as i32 as limb_t,
        )
    }
    rh = (rh as u64).wrapping_sub(mp_sub_ui_dec(
        taba.offset((2 as i32 as u64).wrapping_mul(l) as isize),
        c,
        n.wrapping_sub((2 as i32 as u64).wrapping_mul(l)) as mp_size_t,
    )) as limb_t as limb_t;
    if (rh as slimb_t) < 0 as i32 as i64 {
        mp_sub_ui_dec(tabs, 1 as i32 as limb_t, n as mp_size_t);
        rh = (rh as u64).wrapping_add(mp_add_mul1_dec(
            taba,
            tabs,
            n as mp_size_t,
            2 as i32 as limb_t,
        )) as limb_t as limb_t;
        rh = (rh as u64).wrapping_add(mp_add_ui_dec(taba, 1 as i32 as limb_t, n as mp_size_t))
            as limb_t as limb_t
    }
    return rh;
}
/* 'taba' has 2*n limbs with n >= 1 and taba[2*n-1] >= B/4. Return (s,
r) with s=floor(sqrt(a)) and r=a-s^2. 0 <= r <= 2 * s. tabs has n
limbs. r is returned in the lower n limbs of taba. Its r[n] is the
returned value of the function. */
#[no_mangle]
pub unsafe extern "C" fn mp_sqrtrem_dec(
    mut s: *mut bf_context_t,
    mut tabs: *mut limb_t,
    mut taba: *mut limb_t,
    mut n: limb_t,
) -> i32 {
    let mut tmp_buf1: [limb_t; 8] = [0; 8];
    let mut tmp_buf: *mut limb_t = 0 as *mut limb_t;
    let mut n2: mp_size_t = 0;
    n2 = n
        .wrapping_div(2 as i32 as u64)
        .wrapping_add(1 as i32 as u64) as mp_size_t;
    if n2 as u64
        <= (::std::mem::size_of::<[limb_t; 8]>() as u64)
            .wrapping_div(::std::mem::size_of::<limb_t>() as u64)
    {
        tmp_buf = tmp_buf1.as_mut_ptr()
    } else {
        tmp_buf = bf_malloc(
            s,
            (::std::mem::size_of::<limb_t>() as u64).wrapping_mul(n2 as u64),
        ) as *mut limb_t;
        if tmp_buf.is_null() {
            return -(1 as i32);
        }
    }
    *taba.offset(n as isize) = mp_sqrtrem_rec_dec(tabs, taba, n, tmp_buf);
    if tmp_buf != tmp_buf1.as_mut_ptr() {
        bf_free(s, tmp_buf as *mut std::ffi::c_void);
    }
    return 0 as i32;
}
/* return the number of leading zero digits, from 0 to LIMB_DIGITS */
unsafe extern "C" fn clz_dec(mut a: limb_t) -> i32 {
    if a == 0 as i32 as u64 {
        return 19 as i32;
    }
    match ((1 as i32) << 6 as i32) - 1 as i32 - clz(a) {
        0 => {
            /* 1-1 */
            return 19 as i32 - 1 as i32;
        }
        1 => {
            /* 2-3 */
            return 19 as i32 - 1 as i32;
        }
        2 => {
            /* 4-7 */
            return 19 as i32 - 1 as i32;
        }
        3 => {
            /* 8-15 */
            if a < 10 as i32 as u64 {
                return 19 as i32 - 1 as i32;
            } else {
                return 19 as i32 - 2 as i32;
            }
        }
        4 => {
            /* 16-31 */
            return 19 as i32 - 2 as i32;
        }
        5 => {
            /* 32-63 */
            return 19 as i32 - 2 as i32;
        }
        6 => {
            /* 64-127 */
            if a < 100 as i32 as u64 {
                return 19 as i32 - 2 as i32;
            } else {
                return 19 as i32 - 3 as i32;
            }
        }
        7 => {
            /* 128-255 */
            return 19 as i32 - 3 as i32;
        }
        8 => {
            /* 256-511 */
            return 19 as i32 - 3 as i32;
        }
        9 => {
            /* 512-1023 */
            if a < 1000 as i32 as u64 {
                return 19 as i32 - 3 as i32;
            } else {
                return 19 as i32 - 4 as i32;
            }
        }
        10 => {
            /* 1024-2047 */
            return 19 as i32 - 4 as i32;
        }
        11 => {
            /* 2048-4095 */
            return 19 as i32 - 4 as i32;
        }
        12 => {
            /* 4096-8191 */
            return 19 as i32 - 4 as i32;
        }
        13 => {
            /* 8192-16383 */
            if a < 10000 as i32 as u64 {
                return 19 as i32 - 4 as i32;
            } else {
                return 19 as i32 - 5 as i32;
            }
        }
        14 => {
            /* 16384-32767 */
            return 19 as i32 - 5 as i32;
        }
        15 => {
            /* 32768-65535 */
            return 19 as i32 - 5 as i32;
        }
        16 => {
            /* 65536-131071 */
            if a < 100000 as i32 as u64 {
                return 19 as i32 - 5 as i32;
            } else {
                return 19 as i32 - 6 as i32;
            }
        }
        17 => {
            /* 131072-262143 */
            return 19 as i32 - 6 as i32;
        }
        18 => {
            /* 262144-524287 */
            return 19 as i32 - 6 as i32;
        }
        19 => {
            /* 524288-1048575 */
            if a < 1000000 as i32 as u64 {
                return 19 as i32 - 6 as i32;
            } else {
                return 19 as i32 - 7 as i32;
            }
        }
        20 => {
            /* 1048576-2097151 */
            return 19 as i32 - 7 as i32;
        }
        21 => {
            /* 2097152-4194303 */
            return 19 as i32 - 7 as i32;
        }
        22 => {
            /* 4194304-8388607 */
            return 19 as i32 - 7 as i32;
        }
        23 => {
            /* 8388608-16777215 */
            if a < 10000000 as i32 as u64 {
                return 19 as i32 - 7 as i32;
            } else {
                return 19 as i32 - 8 as i32;
            }
        }
        24 => {
            /* 16777216-33554431 */
            return 19 as i32 - 8 as i32;
        }
        25 => {
            /* 33554432-67108863 */
            return 19 as i32 - 8 as i32;
        }
        26 => {
            /* 67108864-134217727 */
            if a < 100000000 as i32 as u64 {
                return 19 as i32 - 8 as i32;
            } else {
                return 19 as i32 - 9 as i32;
            }
        }
        27 => {
            /* 134217728-268435455 */
            return 19 as i32 - 9 as i32;
        }
        28 => {
            /* 268435456-536870911 */
            return 19 as i32 - 9 as i32;
        }
        29 => {
            /* 536870912-1073741823 */
            if a < 1000000000 as i32 as u64 {
                return 19 as i32 - 9 as i32;
            } else {
                return 19 as i32 - 10 as i32;
            }
        }
        30 => {
            /* 1073741824-2147483647 */
            return 19 as i32 - 10 as i32;
        }
        31 => {
            /* 2147483648-4294967295 */
            return 19 as i32 - 10 as i32;
        }
        32 => {
            /* 4294967296-8589934591 */
            return 19 as i32 - 10 as i32;
        }
        33 => {
            /* 8589934592-17179869183 */
            if a < 10000000000 as i64 as u64 {
                return 19 as i32 - 10 as i32;
            } else {
                return 19 as i32 - 11 as i32;
            }
        }
        34 => {
            /* 17179869184-34359738367 */
            return 19 as i32 - 11 as i32;
        }
        35 => {
            /* 34359738368-68719476735 */
            return 19 as i32 - 11 as i32;
        }
        36 => {
            /* 68719476736-137438953471 */
            if a < 100000000000 as i64 as u64 {
                return 19 as i32 - 11 as i32;
            } else {
                return 19 as i32 - 12 as i32;
            }
        }
        37 => {
            /* 137438953472-274877906943 */
            return 19 as i32 - 12 as i32;
        }
        38 => {
            /* 274877906944-549755813887 */
            return 19 as i32 - 12 as i32;
        }
        39 => {
            /* 549755813888-1099511627775 */
            if a < 1000000000000 as i64 as u64 {
                return 19 as i32 - 12 as i32;
            } else {
                return 19 as i32 - 13 as i32;
            }
        }
        40 => {
            /* 1099511627776-2199023255551 */
            return 19 as i32 - 13 as i32;
        }
        41 => {
            /* 2199023255552-4398046511103 */
            return 19 as i32 - 13 as i32;
        }
        42 => {
            /* 4398046511104-8796093022207 */
            return 19 as i32 - 13 as i32;
        }
        43 => {
            /* 8796093022208-17592186044415 */
            if a < 10000000000000 as i64 as u64 {
                return 19 as i32 - 13 as i32;
            } else {
                return 19 as i32 - 14 as i32;
            }
        }
        44 => {
            /* 17592186044416-35184372088831 */
            return 19 as i32 - 14 as i32;
        }
        45 => {
            /* 35184372088832-70368744177663 */
            return 19 as i32 - 14 as i32;
        }
        46 => {
            /* 70368744177664-140737488355327 */
            if a < 100000000000000 as i64 as u64 {
                return 19 as i32 - 14 as i32;
            } else {
                return 19 as i32 - 15 as i32;
            }
        }
        47 => {
            /* 140737488355328-281474976710655 */
            return 19 as i32 - 15 as i32;
        }
        48 => {
            /* 281474976710656-562949953421311 */
            return 19 as i32 - 15 as i32;
        }
        49 => {
            /* 562949953421312-1125899906842623 */
            if a < 1000000000000000 as i64 as u64 {
                return 19 as i32 - 15 as i32;
            } else {
                return 19 as i32 - 16 as i32;
            }
        }
        50 => {
            /* 1125899906842624-2251799813685247 */
            return 19 as i32 - 16 as i32;
        }
        51 => {
            /* 2251799813685248-4503599627370495 */
            return 19 as i32 - 16 as i32;
        }
        52 => {
            /* 4503599627370496-9007199254740991 */
            return 19 as i32 - 16 as i32;
        }
        53 => {
            /* 9007199254740992-18014398509481983 */
            if a < 10000000000000000 as i64 as u64 {
                return 19 as i32 - 16 as i32;
            } else {
                return 19 as i32 - 17 as i32;
            }
        }
        54 => {
            /* 18014398509481984-36028797018963967 */
            return 19 as i32 - 17 as i32;
        }
        55 => {
            /* 36028797018963968-72057594037927935 */
            return 19 as i32 - 17 as i32;
        }
        56 => {
            /* 72057594037927936-144115188075855871 */
            if a < 100000000000000000 as i64 as u64 {
                return 19 as i32 - 17 as i32;
            } else {
                return 19 as i32 - 18 as i32;
            }
        }
        57 => {
            /* 144115188075855872-288230376151711743 */
            return 19 as i32 - 18 as i32;
        }
        58 => {
            /* 288230376151711744-576460752303423487 */
            return 19 as i32 - 18 as i32;
        }
        59 => {
            /* 576460752303423488-1152921504606846975 */
            if a < 1000000000000000000 as i64 as u64 {
                return 19 as i32 - 18 as i32;
            } else {
                return 19 as i32 - 19 as i32;
            }
        }
        _ => return 0 as i32,
    };
}
/*
/* for debugging */
#[no_mangle]
pub unsafe extern "C" fn bfdec_print_str(mut str: *const std::os::raw::c_char, mut a: *const bfdec_t) {
    let mut i: slimb_t = 0;
    printf(b"%s=\x00" as *const u8 as *const std::os::raw::c_char, str);
    if (*a).expn == 9223372036854775807 as i64 {
        printf(b"NaN\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        if (*a).sign != 0 {
            putchar('-' as i32);
        }
        if (*a).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
            putchar('0' as i32);
        } else if (*a).expn
            == 9223372036854775807 as i64 - 1 as i32 as i64
        {
            printf(b"Inf\x00" as *const u8 as *const std::os::raw::c_char);
        } else {
            printf(b"0.\x00" as *const u8 as *const std::os::raw::c_char);
            i = (*a).len.wrapping_sub(1 as i32 as u64) as slimb_t;
            while i >= 0 as i32 as i64 {
                printf(
                    b"%0*lu\x00" as *const u8 as *const std::os::raw::c_char,
                    19 as i32,
                    *(*a).tab.offset(i as isize),
                );
                i -= 1
            }
            printf(b"e%ld\x00" as *const u8 as *const std::os::raw::c_char, (*a).expn);
        }
    }
    printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
}
*/

/* return != 0 if one digit between 0 and bit_pos inclusive is not zero. */
#[inline]
unsafe extern "C" fn scan_digit_nz(mut r: *const bfdec_t, mut bit_pos: slimb_t) -> limb_t {
    let mut pos: slimb_t = 0;
    let mut v: limb_t = 0;
    let mut q: limb_t = 0;
    let mut shift: i32 = 0;
    if bit_pos < 0 as i32 as i64 {
        return 0 as i32 as limb_t;
    }
    pos = (bit_pos as limb_t).wrapping_div(19 as i32 as u64) as slimb_t;
    shift = (bit_pos as limb_t).wrapping_rem(19 as i32 as u64) as i32;
    q = fast_shr_dec(*(*r).tab.offset(pos as isize), shift + 1 as i32);
    v = (*(*r).tab.offset(pos as isize))
        .wrapping_sub(q.wrapping_mul(mp_pow_dec[(shift + 1 as i32) as usize]));
    if v != 0 as i32 as u64 {
        return 1 as i32 as limb_t;
    }
    pos -= 1;
    while pos >= 0 as i32 as i64 {
        if *(*r).tab.offset(pos as isize) != 0 as i32 as u64 {
            return 1 as i32 as limb_t;
        }
        pos -= 1
    }
    return 0 as i32 as limb_t;
}
unsafe extern "C" fn get_digit(
    mut tab: *const limb_t,
    mut len: limb_t,
    mut pos: slimb_t,
) -> limb_t {
    let mut i: slimb_t = 0;
    let mut shift: i32 = 0;
    i = floor_div(pos, 19 as i32 as slimb_t);
    if i < 0 as i32 as i64 || i as u64 >= len {
        return 0 as i32 as limb_t;
    }
    shift = (pos - i * 19 as i32 as i64) as i32;
    return fast_shr_dec(*tab.offset(i as isize), shift).wrapping_rem(10 as i32 as u64);
}
/* return the addend for rounding. Note that prec can be <= 0 for bf_rint() */
unsafe extern "C" fn bfdec_get_rnd_add(
    mut pret: *mut i32,
    mut r: *const bfdec_t,
    mut l: limb_t,
    mut prec: slimb_t,
    mut rnd_mode: i32,
) -> i32 {
    let mut add_one: i32 = 0;
    let mut inexact: i32 = 0;
    let mut digit1: limb_t = 0;
    let mut digit0: limb_t = 0;
    //    bfdec_print_str("get_rnd_add", r);
    if rnd_mode == BF_RNDF as i32 {
        digit0 = 1 as i32 as limb_t
    /* faithful rounding does not honor the INEXACT flag */
    } else {
        /* starting limb for bit 'prec + 1' */
        digit0 = scan_digit_nz(
            r,
            l.wrapping_mul(19 as i32 as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_sub(bf_max(0 as i32 as slimb_t, prec + 1 as i32 as i64) as u64)
                as slimb_t,
        )
    }
    /* get the digit at 'prec' */
    digit1 = get_digit(
        (*r).tab,
        l,
        l.wrapping_mul(19 as i32 as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_sub(prec as u64) as slimb_t,
    );
    inexact = (digit1 | digit0 != 0 as i32 as u64) as i32;
    add_one = 0 as i32;
    match rnd_mode {
        1 => {}
        0 => {
            if digit1 == 5 as i32 as u64 {
                if digit0 != 0 {
                    add_one = 1 as i32
                } else {
                    /* round to even */
                    add_one = (get_digit(
                        (*r).tab,
                        l,
                        l.wrapping_mul(19 as i32 as u64)
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_sub((prec - 1 as i32 as i64) as u64)
                            as slimb_t,
                    ) & 1 as i32 as u64) as i32
                }
            } else if digit1 > 5 as i32 as u64 {
                add_one = 1 as i32
            }
        }
        2 | 3 => {
            if (*r).sign == (rnd_mode == BF_RNDD as i32) as i32 {
                add_one = inexact
            }
        }
        4 | 6 => add_one = (digit1 >= 5 as i32 as u64) as i32,
        5 => add_one = inexact,
        _ => {
            abort();
        }
    }
    if inexact != 0 {
        *pret |= (1 as i32) << 4 as i32
    }
    return add_one;
}
/* round to prec1 bits assuming 'r' is non zero and finite. 'r' is
  assumed to have length 'l' (1 <= l <= r->len). prec1 can be
  BF_PREC_INF. BF_FLAG_SUBNORMAL is not supported. Cannot fail with
  BF_ST_MEM_ERROR.
*/
unsafe extern "C" fn __bfdec_round(
    mut r: *mut bfdec_t,
    mut prec1: limb_t,
    mut flags: bf_flags_t,
    mut l: limb_t,
) -> i32 {
    let mut current_block: u64;
    let mut shift: i32 = 0;
    let mut add_one: i32 = 0;
    let mut rnd_mode: i32 = 0;
    let mut ret: i32 = 0;
    let mut i: slimb_t = 0;
    let mut bit_pos: slimb_t = 0;
    let mut pos: slimb_t = 0;
    let mut e_min: slimb_t = 0;
    let mut e_max: slimb_t = 0;
    let mut e_range: slimb_t = 0;
    let mut prec: slimb_t = 0;
    /* XXX: align to IEEE 754 2008 for decimal numbers ? */
    e_range = ((1 as i32 as limb_t) << bf_get_exp_bits(flags) - 1 as i32) as slimb_t;
    e_min = -e_range + 3 as i32 as i64;
    e_max = e_range;
    if flags & ((1 as i32) << 4 as i32) as u32 != 0 {
        /* 'prec' is the precision after the decimal point */
        if prec1
            != ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
        {
            prec = ((*r).expn as u64).wrapping_add(prec1) as slimb_t
        } else {
            prec = prec1 as slimb_t
        }
    } else if ((*r).expn < e_min) as i32 as i64 != 0 && flags & ((1 as i32) << 3 as i32) as u32 != 0
    {
        /* restrict the precision in case of potentially subnormal
        result */
        if prec1
            != ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
        {
        } else {
            assert!(prec1 != BF_PREC_INF);
        }
        prec = prec1.wrapping_sub((e_min - (*r).expn) as u64) as slimb_t
    } else {
        prec = prec1 as slimb_t
    }
    /* round to prec bits */
    rnd_mode = (flags & 0x7 as i32 as u32) as i32; /* cannot fail because r is non zero */
    ret = 0 as i32;
    add_one = bfdec_get_rnd_add(&mut ret, r, l, prec, rnd_mode);
    if prec <= 0 as i32 as i64 {
        if add_one != 0 {
            bfdec_resize(r, 1 as i32 as limb_t);
            *(*r).tab.offset(0 as i32 as isize) =
                (10000000000000000000 as u64).wrapping_div(10 as i32 as u64);
            (*r).expn += 1 as i32 as i64 - prec;
            ret |= (1 as i32) << 3 as i32 | (1 as i32) << 4 as i32;
            return ret;
        }
    } else {
        if add_one != 0 {
            let mut carry: limb_t = 0;
            /* add one starting at digit 'prec - 1' */
            bit_pos = l
                .wrapping_mul(19 as i32 as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_sub((prec - 1 as i32 as i64) as u64) as slimb_t;
            pos = bit_pos / 19 as i32 as i64;
            carry = mp_pow_dec[(bit_pos % 19 as i32 as i64) as usize];
            carry = mp_add_ui_dec(
                (*r).tab.offset(pos as isize),
                carry,
                l.wrapping_sub(pos as u64) as mp_size_t,
            );
            if carry != 0 {
                /* shift right by one digit */
                mp_shr_dec(
                    (*r).tab.offset(pos as isize),
                    (*r).tab.offset(pos as isize),
                    l.wrapping_sub(pos as u64) as mp_size_t,
                    1 as i32 as limb_t,
                    1 as i32 as limb_t,
                );
                (*r).expn += 1
            }
        }
        /* check underflow */
        if ((*r).expn < e_min) as i32 as i64 != 0 {
            if flags & ((1 as i32) << 3 as i32) as u32 != 0 {
                /* if inexact, also set the underflow flag */
                if ret & (1 as i32) << 4 as i32 != 0 {
                    ret |= (1 as i32) << 3 as i32
                }
                current_block = 18435049525520518667;
            } else {
                current_block = 10692295422897365397;
            }
        } else {
            current_block = 18435049525520518667;
        }
        match current_block {
            10692295422897365397 => {}
            _ => {
                /* check overflow */
                if ((*r).expn > e_max) as i32 as i64 != 0 {
                    bfdec_set_inf(r, (*r).sign);
                    ret |= (1 as i32) << 2 as i32 | (1 as i32) << 4 as i32;
                    return ret;
                }
                /* keep the bits starting at 'prec - 1' */
                bit_pos = l
                    .wrapping_mul(19 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_sub((prec - 1 as i32 as i64) as u64)
                    as slimb_t;
                i = floor_div(bit_pos, 19 as i32 as slimb_t);
                if i >= 0 as i32 as i64 {
                    shift = smod(bit_pos, 19 as i32 as slimb_t) as i32;
                    if shift != 0 as i32 {
                        *(*r).tab.offset(i as isize) =
                            fast_shr_dec(*(*r).tab.offset(i as isize), shift)
                                .wrapping_mul(mp_pow_dec[shift as usize])
                    }
                } else {
                    i = 0 as i32 as slimb_t
                }
                /* remove trailing zeros */
                while *(*r).tab.offset(i as isize) == 0 as i32 as u64 {
                    i += 1
                } /* cannot fail */
                if i > 0 as i32 as i64 {
                    l = (l as u64).wrapping_sub(i as u64) as limb_t as limb_t;
                    ((*r).tab as *mut u8).copy_from_nonoverlapping(
                        (*r).tab.offset(i as isize) as *const u8,
                        (l as usize).wrapping_mul(::std::mem::size_of::<limb_t>()),
                    );
                }
                bfdec_resize(r, l);
                return ret;
            }
        }
    }
    bfdec_set_zero(r, (*r).sign);
    ret |= (1 as i32) << 3 as i32 | (1 as i32) << 4 as i32;
    return ret;
}
/* Cannot fail with BF_ST_MEM_ERROR. */
#[no_mangle]
pub unsafe extern "C" fn bfdec_round(
    mut r: *mut bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    if (*r).len == 0 as i32 as u64 {
        return 0 as i32;
    }
    return __bfdec_round(r, prec, flags, (*r).len);
}
/* 'r' must be a finite number. Cannot fail with BF_ST_MEM_ERROR.  */
#[no_mangle]
pub unsafe extern "C" fn bfdec_normalize_and_round(
    mut r: *mut bfdec_t,
    mut prec1: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut l: limb_t = 0;
    let mut v: limb_t = 0;
    let mut shift: i32 = 0;
    let mut ret: i32 = 0;
    //    bfdec_print_str("bf_renorm", r);
    l = (*r).len;
    while l > 0 as i32 as u64
        && *(*r).tab.offset(l.wrapping_sub(1 as i32 as u64) as isize) == 0 as i32 as u64
    {
        l = l.wrapping_sub(1)
    }
    if l == 0 as i32 as u64 {
        /* zero */
        (*r).expn = -(9223372036854775807 as i64) - 1 as i32 as i64; /* cannot fail */
        bfdec_resize(r, 0 as i32 as limb_t);
        ret = 0 as i32
    } else {
        (*r).expn = ((*r).expn as u64)
            .wrapping_sub((*r).len.wrapping_sub(l).wrapping_mul(19 as i32 as u64))
            as slimb_t as slimb_t;
        /* shift to have the MSB set to '1' */
        v = *(*r).tab.offset(l.wrapping_sub(1 as i32 as u64) as isize);
        shift = clz_dec(v);
        if shift != 0 as i32 {
            mp_shl_dec(
                (*r).tab,
                (*r).tab,
                l as mp_size_t,
                shift as limb_t,
                0 as i32 as limb_t,
            );
            (*r).expn -= shift as i64
        }
        ret = __bfdec_round(r, prec1, flags, l)
    }
    //    bf_print_str("r_final", r);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_set_ui(mut r: *mut bfdec_t, mut v: uint64_t) -> i32 {
    let mut current_block: u64;
    if v >= 10000000000000000000 as u64 {
        if bfdec_resize(r, 2 as i32 as limb_t) != 0 {
            current_block = 13411379657240756855;
        } else {
            *(*r).tab.offset(0 as i32 as isize) = v.wrapping_rem(10000000000000000000 as u64);
            *(*r).tab.offset(1 as i32 as isize) = v.wrapping_div(10000000000000000000 as u64);
            (*r).expn = (2 as i32 * 19 as i32) as slimb_t;
            current_block = 8515828400728868193;
        }
    } else if bfdec_resize(r, 1 as i32 as limb_t) != 0 {
        current_block = 13411379657240756855;
    } else {
        *(*r).tab.offset(0 as i32 as isize) = v;
        (*r).expn = 19 as i32 as slimb_t;
        current_block = 8515828400728868193;
    }
    match current_block {
        8515828400728868193 => {
            (*r).sign = 0 as i32;
            return bfdec_normalize_and_round(
                r,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                0 as i32 as bf_flags_t,
            );
        }
        _ => {
            bfdec_set_nan(r);
            return (1 as i32) << 5 as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_set_si(mut r: *mut bfdec_t, mut v: int64_t) -> i32 {
    let mut ret: i32 = 0;
    if v < 0 as i32 as i64 {
        ret = bfdec_set_ui(r, -v as uint64_t);
        (*r).sign = 1 as i32
    } else {
        ret = bfdec_set_ui(r, v as uint64_t)
    }
    return ret;
}
unsafe extern "C" fn bfdec_add_internal(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut b_neg: i32,
) -> i32 {
    let mut d: slimb_t = 0;
    let mut a_offset: slimb_t = 0;
    let mut b_offset: slimb_t = 0;
    let mut i: slimb_t = 0;
    let mut r_len: slimb_t = 0;
    let mut carry: limb_t = 0;
    let mut b1_tab: *mut limb_t = 0 as *mut limb_t;
    let mut b_shift: i32 = 0;
    let mut b1_len: mp_size_t = 0;
    let mut current_block: u64;
    let mut s: *mut bf_context_t = (*r).ctx;
    let mut is_sub: i32 = 0;
    let mut cmp_res: i32 = 0;
    let mut a_sign: i32 = 0;
    let mut b_sign: i32 = 0;
    let mut ret: i32 = 0;
    a_sign = (*a).sign;
    b_sign = (*b).sign ^ b_neg;
    is_sub = a_sign ^ b_sign;
    cmp_res = bfdec_cmpu(a, b);
    if cmp_res < 0 as i32 {
        let mut tmp: *const bfdec_t = 0 as *const bfdec_t;
        tmp = a;
        a = b;
        b = tmp;
        a_sign = b_sign
        /* b_sign is never used later */
    }
    /* abs(a) >= abs(b) */
    if cmp_res == 0 as i32
        && is_sub != 0
        && (*a).expn < 9223372036854775807 as i64 - 1 as i32 as i64
    {
        /* zero result */
        bfdec_set_zero(
            r,
            (flags & 0x7 as i32 as u32 == BF_RNDD as i32 as u32) as i32,
        );
        ret = 0 as i32
    } else {
        if (*a).len == 0 as i32 as u64 || (*b).len == 0 as i32 as u64 {
            ret = 0 as i32;
            if (*a).expn >= 9223372036854775807 as i64 - 1 as i32 as i64 {
                if (*a).expn == 9223372036854775807 as i64 {
                    /* at least one operand is NaN */
                    bfdec_set_nan(r);
                    ret = 0 as i32
                } else if (*b).expn == 9223372036854775807 as i64 - 1 as i32 as i64 && is_sub != 0 {
                    /* infinities with different signs */
                    bfdec_set_nan(r);
                    ret = (1 as i32) << 0 as i32
                } else {
                    bfdec_set_inf(r, a_sign);
                }
                current_block = 8834769789432328951;
            } else {
                /* at least one zero and not subtract */
                if bfdec_set(r, a) != 0 {
                    return (1 as i32) << 5 as i32;
                }
                (*r).sign = a_sign;
                current_block = 2519986835199647032;
            }
        } else {
            d = 0;
            a_offset = 0;
            b_offset = 0;
            i = 0;
            r_len = 0;
            carry = 0;
            b1_tab = 0 as *mut limb_t;
            b_shift = 0;
            b1_len = 0;
            d = (*a).expn - (*b).expn;
            /* XXX: not efficient in time and memory if the precision is
            not infinite */
            r_len = bf_max(
                (*a).len as slimb_t,
                (*b).len.wrapping_add(
                    ((d + 19 as i32 as i64 - 1 as i32 as i64) / 19 as i32 as i64) as u64,
                ) as slimb_t,
            );
            if bfdec_resize(r, r_len as limb_t) != 0 {
                current_block = 5503229294326981584;
            } else {
                (*r).sign = a_sign;
                (*r).expn = (*a).expn;
                a_offset = (r_len as u64).wrapping_sub((*a).len) as slimb_t;
                i = 0 as i32 as slimb_t;
                while i < a_offset {
                    *(*r).tab.offset(i as isize) = 0 as i32 as limb_t;
                    i += 1
                }
                i = 0 as i32 as slimb_t;
                while (i as u64) < (*a).len {
                    *(*r).tab.offset((a_offset + i) as isize) = *(*a).tab.offset(i as isize);
                    i += 1
                }
                b_shift = (d % 19 as i32 as i64) as i32;
                if b_shift == 0 as i32 {
                    b1_len = (*b).len as mp_size_t;
                    b1_tab = (*b).tab;
                    current_block = 2516253395664191498;
                } else {
                    b1_len = (*b).len.wrapping_add(1 as i32 as u64) as mp_size_t;
                    b1_tab = bf_malloc(
                        s,
                        (::std::mem::size_of::<limb_t>() as u64).wrapping_mul(b1_len as u64),
                    ) as *mut limb_t;
                    if b1_tab.is_null() {
                        current_block = 5503229294326981584;
                    } else {
                        *b1_tab.offset(0 as i32 as isize) = mp_shr_dec(
                            b1_tab.offset(1 as i32 as isize),
                            (*b).tab,
                            (*b).len as mp_size_t,
                            b_shift as limb_t,
                            0 as i32 as limb_t,
                        )
                        .wrapping_mul(mp_pow_dec[(19 as i32 - b_shift) as usize]);
                        current_block = 2516253395664191498;
                    }
                }
                match current_block {
                    5503229294326981584 => {}
                    _ => {
                        b_offset = (r_len as u64).wrapping_sub((*b).len.wrapping_add(
                            ((d + 19 as i32 as i64 - 1 as i32 as i64) / 19 as i32 as i64) as u64,
                        )) as slimb_t;
                        if is_sub != 0 {
                            carry = mp_sub_dec(
                                (*r).tab.offset(b_offset as isize),
                                (*r).tab.offset(b_offset as isize),
                                b1_tab,
                                b1_len,
                                0 as i32 as limb_t,
                            );
                            if carry != 0 as i32 as u64 {
                                carry = mp_sub_ui_dec(
                                    (*r).tab.offset(b_offset as isize).offset(b1_len as isize),
                                    carry,
                                    r_len - (b_offset + b1_len),
                                );
                                if carry == 0 as i32 as u64 {
                                } else {
                                    assert!(carry == 0);
                                }
                            }
                            current_block = 13484060386966298149;
                        } else {
                            carry = mp_add_dec(
                                (*r).tab.offset(b_offset as isize),
                                (*r).tab.offset(b_offset as isize),
                                b1_tab,
                                b1_len,
                                0 as i32 as limb_t,
                            );
                            if carry != 0 as i32 as u64 {
                                carry = mp_add_ui_dec(
                                    (*r).tab.offset(b_offset as isize).offset(b1_len as isize),
                                    carry,
                                    r_len - (b_offset + b1_len),
                                )
                            }
                            if carry != 0 as i32 as u64 {
                                if bfdec_resize(r, (r_len + 1 as i32 as i64) as limb_t) != 0 {
                                    if b_shift != 0 as i32 {
                                        bf_free(s, b1_tab as *mut std::ffi::c_void);
                                    }
                                    current_block = 5503229294326981584;
                                } else {
                                    *(*r).tab.offset(r_len as isize) = 1 as i32 as limb_t;
                                    (*r).expn += 19 as i32 as i64;
                                    current_block = 13484060386966298149;
                                }
                            } else {
                                current_block = 13484060386966298149;
                            }
                        }
                        match current_block {
                            5503229294326981584 => {}
                            _ => {
                                if b_shift != 0 as i32 {
                                    bf_free(s, b1_tab as *mut std::ffi::c_void);
                                }
                                current_block = 2519986835199647032;
                            }
                        }
                    }
                }
            }
            match current_block {
                2519986835199647032 => {}
                _ => {
                    bfdec_set_nan(r);
                    return (1 as i32) << 5 as i32;
                }
            }
        }
        match current_block {
            8834769789432328951 => {}
            _ => ret = bfdec_normalize_and_round(r, prec, flags),
        }
    }
    return ret;
}
unsafe extern "C" fn __bfdec_add(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bfdec_add_internal(r, a, b, prec, flags, 0 as i32);
}
unsafe extern "C" fn __bfdec_sub(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bfdec_add_internal(r, a, b, prec, flags, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_add(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_op2(
        r as *mut bf_t,
        a as *mut bf_t,
        b as *mut bf_t,
        prec,
        flags,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut bfdec_t,
                    _: *const bfdec_t,
                    _: *const bfdec_t,
                    _: limb_t,
                    _: bf_flags_t,
                ) -> i32,
            >,
            Option<bf_op2_func_t>,
        >(Some(
            __bfdec_add
                as unsafe extern "C" fn(
                    _: *mut bfdec_t,
                    _: *const bfdec_t,
                    _: *const bfdec_t,
                    _: limb_t,
                    _: bf_flags_t,
                ) -> i32,
        )),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_sub(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_op2(
        r as *mut bf_t,
        a as *mut bf_t,
        b as *mut bf_t,
        prec,
        flags,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut bfdec_t,
                    _: *const bfdec_t,
                    _: *const bfdec_t,
                    _: limb_t,
                    _: bf_flags_t,
                ) -> i32,
            >,
            Option<bf_op2_func_t>,
        >(Some(
            __bfdec_sub
                as unsafe extern "C" fn(
                    _: *mut bfdec_t,
                    _: *const bfdec_t,
                    _: *const bfdec_t,
                    _: limb_t,
                    _: bf_flags_t,
                ) -> i32,
        )),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_mul(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut ret: i32 = 0;
    let mut r_sign: i32 = 0;
    if (*a).len < (*b).len {
        let mut tmp: *const bfdec_t = a;
        a = b;
        b = tmp
    }
    r_sign = (*a).sign ^ (*b).sign;
    /* here b->len <= a->len */
    if (*b).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 || (*b).expn == 9223372036854775807 as i64 {
            bfdec_set_nan(r);
            ret = 0 as i32
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64
            || (*b).expn == 9223372036854775807 as i64 - 1 as i32 as i64
        {
            if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64
                && (*b).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
                || (*a).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
                    && (*b).expn == 9223372036854775807 as i64 - 1 as i32 as i64
            {
                bfdec_set_nan(r);
                ret = (1 as i32) << 0 as i32
            } else {
                bfdec_set_inf(r, r_sign);
                ret = 0 as i32
            }
        } else {
            bfdec_set_zero(r, r_sign);
            ret = 0 as i32
        }
    } else {
        let mut tmp_0: bfdec_t = bfdec_t {
            ctx: 0 as *mut bf_context_t,
            sign: 0,
            expn: 0,
            len: 0,
            tab: 0 as *mut limb_t,
        };
        let mut r1: *mut bfdec_t = 0 as *mut bfdec_t;
        let mut a_len: limb_t = 0;
        let mut b_len: limb_t = 0;
        let mut a_tab: *mut limb_t = 0 as *mut limb_t;
        let mut b_tab: *mut limb_t = 0 as *mut limb_t;
        a_len = (*a).len;
        b_len = (*b).len;
        a_tab = (*a).tab;
        b_tab = (*b).tab;
        if r == a as *mut bfdec_t || r == b as *mut bfdec_t {
            bfdec_init((*r).ctx, &mut tmp_0);
            r1 = r;
            r = &mut tmp_0
        }
        if bfdec_resize(r, a_len.wrapping_add(b_len)) != 0 {
            bfdec_set_nan(r);
            ret = (1 as i32) << 5 as i32
        } else {
            mp_mul_basecase_dec(
                (*r).tab,
                a_tab,
                a_len as mp_size_t,
                b_tab,
                b_len as mp_size_t,
            );
            (*r).sign = r_sign;
            (*r).expn = (*a).expn + (*b).expn;
            ret = bfdec_normalize_and_round(r, prec, flags)
        }
        if r == &mut tmp_0 as *mut bfdec_t {
            bfdec_move(r1, &mut tmp_0);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_mul_si(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b1: int64_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut b: bfdec_t = bfdec_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut ret: i32 = 0;
    bfdec_init((*r).ctx, &mut b);
    ret = bfdec_set_si(&mut b, b1);
    ret |= bfdec_mul(r, a, &mut b, prec, flags);
    bfdec_delete(&mut b);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_add_si(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b1: int64_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut b: bfdec_t = bfdec_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut ret: i32 = 0;
    bfdec_init((*r).ctx, &mut b);
    ret = bfdec_set_si(&mut b, b1);
    ret |= bfdec_add(r, a, &mut b, prec, flags);
    bfdec_delete(&mut b);
    return ret;
}
unsafe extern "C" fn __bfdec_div(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut ret: i32 = 0;
    let mut r_sign: i32 = 0;
    let mut n: limb_t = 0;
    let mut nb: limb_t = 0;
    let mut precl: limb_t = 0;
    r_sign = (*a).sign ^ (*b).sign;
    if (*a).expn >= 9223372036854775807 as i64 - 1 as i32 as i64
        || (*b).expn >= 9223372036854775807 as i64 - 1 as i32 as i64
    {
        if (*a).expn == 9223372036854775807 as i64 || (*b).expn == 9223372036854775807 as i64 {
            bfdec_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64
            && (*b).expn == 9223372036854775807 as i64 - 1 as i32 as i64
        {
            bfdec_set_nan(r);
            return (1 as i32) << 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            bfdec_set_inf(r, r_sign);
            return 0 as i32;
        } else {
            bfdec_set_zero(r, r_sign);
            return 0 as i32;
        }
    } else {
        if (*a).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
            if (*b).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
                bfdec_set_nan(r);
                return (1 as i32) << 0 as i32;
            } else {
                bfdec_set_zero(r, r_sign);
                return 0 as i32;
            }
        } else {
            if (*b).expn == -(9223372036854775807 as i64) - 1 as i32 as i64 {
                bfdec_set_inf(r, r_sign);
                return (1 as i32) << 1 as i32;
            }
        }
    }
    nb = (*b).len;
    if prec
        == ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
            .wrapping_sub(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64)
    {
        /* infinite precision: return BF_ST_INVALID_OP if not an exact
        result */
        /* XXX: check */
        precl = nb.wrapping_add(1 as i32 as u64)
    } else if flags & ((1 as i32) << 4 as i32) as u32 != 0 {
        /* number of digits after the decimal point */
        /* XXX: check (2 extra digits for rounding + 2 digits) */
        precl = ((bf_max((*a).expn - (*b).expn, 0 as i32 as slimb_t) + 2 as i32 as i64) as u64)
            .wrapping_add(prec)
            .wrapping_add(2 as i32 as u64)
            .wrapping_add(19 as i32 as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(19 as i32 as u64)
    } else {
        /* number of limbs of the quotient (2 extra digits for rounding) */
        precl = prec
            .wrapping_add(2 as i32 as u64)
            .wrapping_add(19 as i32 as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(19 as i32 as u64)
    }
    n = bf_max((*a).len as slimb_t, precl as slimb_t) as limb_t;
    let mut taba: *mut limb_t = 0 as *mut limb_t;
    let mut na: limb_t = 0;
    let mut i: limb_t = 0;
    let mut d: slimb_t = 0;
    na = n.wrapping_add(nb);
    taba = bf_malloc(
        (*r).ctx,
        na.wrapping_add(1 as i32 as u64)
            .wrapping_mul(::std::mem::size_of::<limb_t>() as u64),
    ) as *mut limb_t;
    if !taba.is_null() {
        d = na.wrapping_sub((*a).len) as slimb_t;
        (taba as *mut u8).write_bytes(0, (d as usize).wrapping_mul(std::mem::size_of::<limb_t>()));
        (taba.offset(d as isize) as *mut u8).copy_from(
            (*a).tab as *const u8,
            ((*a).len as usize).wrapping_mul(std::mem::size_of::<limb_t>()),
        );
        if !(bfdec_resize(r, n.wrapping_add(1 as i32 as u64)) != 0) {
            if !(mp_div_dec(
                (*r).ctx,
                (*r).tab,
                taba,
                na as mp_size_t,
                (*b).tab,
                nb as mp_size_t,
            ) != 0)
            {
                /* see if non zero remainder */
                i = 0 as i32 as limb_t;
                while i < nb {
                    if *taba.offset(i as isize) != 0 as i32 as u64 {
                        break;
                    }
                    i = i.wrapping_add(1)
                }
                bf_free((*r).ctx, taba as *mut std::ffi::c_void);
                if i != nb {
                    if prec
                        == ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                            .wrapping_sub(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    {
                        bfdec_set_nan(r);
                        return (1 as i32) << 0 as i32;
                    } else {
                        let ref mut fresh9 = *(*r).tab.offset(0 as i32 as isize);
                        *fresh9 |= 1 as i32 as u64
                    }
                }
                (*r).expn = (*a).expn - (*b).expn + 19 as i32 as i64;
                (*r).sign = r_sign;
                ret = bfdec_normalize_and_round(r, prec, flags);
                return ret;
            }
        }
        bf_free((*r).ctx, taba as *mut std::ffi::c_void);
    }
    bfdec_set_nan(r);
    return (1 as i32) << 5 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_div(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    return bf_op2(
        r as *mut bf_t,
        a as *mut bf_t,
        b as *mut bf_t,
        prec,
        flags,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut bfdec_t,
                    _: *const bfdec_t,
                    _: *const bfdec_t,
                    _: limb_t,
                    _: bf_flags_t,
                ) -> i32,
            >,
            Option<bf_op2_func_t>,
        >(Some(
            __bfdec_div
                as unsafe extern "C" fn(
                    _: *mut bfdec_t,
                    _: *const bfdec_t,
                    _: *const bfdec_t,
                    _: limb_t,
                    _: bf_flags_t,
                ) -> i32,
        )),
    );
}
/* a and b must be finite numbers with a >= 0 and b > 0. 'q' is the
integer defined as floor(a/b) and r = a - q * b. */
unsafe extern "C" fn bfdec_tdivremu(
    mut s: *mut bf_context_t,
    mut q: *mut bfdec_t,
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
) {
    if bfdec_cmpu(a, b) < 0 as i32 {
        bfdec_set_ui(q, 0 as i32 as uint64_t);
        bfdec_set(r, a);
    } else {
        bfdec_div(
            q,
            a,
            b,
            0 as i32 as limb_t,
            (BF_RNDZ as i32 | (1 as i32) << 4 as i32) as bf_flags_t,
        );
        bfdec_mul(
            r,
            q,
            b,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        bfdec_sub(
            r,
            a,
            r,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
    };
}
/* division and remainder.

   rnd_mode is the rounding mode for the quotient. The additional
   rounding mode BF_RND_EUCLIDIAN is supported.

   'q' is an integer. 'r' is rounded with prec and flags (prec can be
   BF_PREC_INF).
*/
#[no_mangle]
pub unsafe extern "C" fn bfdec_divrem(
    mut q: *mut bfdec_t,
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut rnd_mode: i32,
) -> i32 {
    let mut current_block: u64;
    let mut s: *mut bf_context_t = (*q).ctx;
    let mut a1_s: bfdec_t = bfdec_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut a1: *mut bfdec_t = &mut a1_s;
    let mut b1_s: bfdec_t = bfdec_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut b1: *mut bfdec_t = &mut b1_s;
    let mut r1_s: bfdec_t = bfdec_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut r1: *mut bfdec_t = &mut r1_s;
    let mut q_sign: i32 = 0;
    let mut res: i32 = 0;
    let mut is_ceil: BOOL = 0;
    let mut is_rndn: BOOL = 0;
    if q != a as *mut bfdec_t && q != b as *mut bfdec_t {
    } else {
        assert!(q as *const bfdec_t != a && q as *const bfdec_t != b);
    }
    if r != a as *mut bfdec_t && r != b as *mut bfdec_t {
    } else {
        assert!(r as *const bfdec_t != a && r as *const bfdec_t != b);
    }
    if q != r {
    } else {
        assert!(q != r);
    }
    if (*a).len == 0 as i32 as u64 || (*b).len == 0 as i32 as u64 {
        bfdec_set_zero(q, 0 as i32);
        if (*a).expn == 9223372036854775807 as i64 || (*b).expn == 9223372036854775807 as i64 {
            bfdec_set_nan(r);
            return 0 as i32;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64
            || (*b).expn == -(9223372036854775807 as i64) - 1 as i32 as i64
        {
            bfdec_set_nan(r);
            return (1 as i32) << 0 as i32;
        } else {
            bfdec_set(r, a);
            return bfdec_round(r, prec, flags);
        }
    }
    q_sign = (*a).sign ^ (*b).sign;
    is_rndn = (rnd_mode == BF_RNDN as i32 || rnd_mode == BF_RNDNA as i32) as i32;
    match rnd_mode {
        2 => is_ceil = q_sign,
        3 => is_ceil = q_sign ^ 1 as i32,
        5 => is_ceil = TRUE as i32,
        6 => is_ceil = (*a).sign,
        1 | 0 | 4 | _ => is_ceil = FALSE as i32,
    }
    (*a1).expn = (*a).expn;
    (*a1).tab = (*a).tab;
    (*a1).len = (*a).len;
    (*a1).sign = 0 as i32;
    (*b1).expn = (*b).expn;
    (*b1).tab = (*b).tab;
    (*b1).len = (*b).len;
    (*b1).sign = 0 as i32;
    //    bfdec_print_str("a1", a1);
    //    bfdec_print_str("b1", b1);
    /* XXX: could improve to avoid having a large 'q' */
    bfdec_tdivremu(s, q, r, a1, b1);
    if !(bfdec_is_nan(q) != 0 || bfdec_is_nan(r) != 0) {
        //    bfdec_print_str("q", q);
        //    bfdec_print_str("r", r);
        if (*r).len != 0 as i32 as u64 {
            if is_rndn != 0 {
                bfdec_init(s, r1);
                if bfdec_set(r1, r) != 0 {
                    current_block = 18434615161324483346;
                } else if bfdec_mul_si(
                    r1,
                    r1,
                    2 as i32 as int64_t,
                    ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                        .wrapping_sub(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64),
                    BF_RNDZ as i32 as bf_flags_t,
                ) != 0
                {
                    bfdec_delete(r1);
                    current_block = 18434615161324483346;
                } else {
                    res = bfdec_cmpu(r1, b);
                    bfdec_delete(r1);
                    if res > 0 as i32
                        || res == 0 as i32
                            && (rnd_mode == BF_RNDNA as i32
                                || get_digit(
                                    (*q).tab,
                                    (*q).len,
                                    (*q).len
                                        .wrapping_mul(19 as i32 as u64)
                                        .wrapping_sub((*q).expn as u64)
                                        as slimb_t,
                                ) & 1 as i32 as u64
                                    != 0 as i32 as u64)
                    {
                        current_block = 7254512567665389648;
                    } else {
                        current_block = 8151474771948790331;
                    }
                }
            } else if is_ceil != 0 {
                current_block = 7254512567665389648;
            } else {
                current_block = 8151474771948790331;
            }
            match current_block {
                8151474771948790331 => {}
                18434615161324483346 => {}
                _ => {
                    res = bfdec_add_si(
                        q,
                        q,
                        1 as i32 as int64_t,
                        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                            .wrapping_sub(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64),
                        BF_RNDZ as i32 as bf_flags_t,
                    );
                    res |= bfdec_sub(
                        r,
                        r,
                        b1,
                        ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                            .wrapping_sub(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64),
                        BF_RNDZ as i32 as bf_flags_t,
                    );
                    if res & (1 as i32) << 5 as i32 != 0 {
                        current_block = 18434615161324483346;
                    } else {
                        current_block = 8151474771948790331;
                    }
                }
            }
        } else {
            current_block = 8151474771948790331;
        }
        match current_block {
            18434615161324483346 => {}
            _ => {
                (*r).sign ^= (*a).sign;
                (*q).sign = q_sign;
                return bfdec_round(r, prec, flags);
            }
        }
    }
    bfdec_set_nan(q);
    bfdec_set_nan(r);
    return (1 as i32) << 5 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_rem(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
    mut rnd_mode: i32,
) -> i32 {
    let mut q_s: bfdec_t = bfdec_t {
        ctx: 0 as *mut bf_context_t,
        sign: 0,
        expn: 0,
        len: 0,
        tab: 0 as *mut limb_t,
    };
    let mut q: *mut bfdec_t = &mut q_s;
    let mut ret: i32 = 0;
    bfdec_init((*r).ctx, q);
    ret = bfdec_divrem(q, r, a, b, prec, flags, rnd_mode);
    bfdec_delete(q);
    return ret;
}
/* convert to integer (infinite precision) */
#[no_mangle]
pub unsafe extern "C" fn bfdec_rint(mut r: *mut bfdec_t, mut rnd_mode: i32) -> i32 {
    return bfdec_round(
        r,
        0 as i32 as limb_t,
        (rnd_mode | (1 as i32) << 4 as i32) as bf_flags_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_sqrt(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut current_block: u64;
    let mut s: *mut bf_context_t = (*a).ctx;
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut a1: *mut limb_t = 0 as *mut limb_t;
    let mut v: limb_t = 0;
    let mut n: slimb_t = 0;
    let mut n1: slimb_t = 0;
    let mut prec1: slimb_t = 0;
    let mut res: limb_t = 0;
    if r != a as *mut bfdec_t {
    } else {
        assert!(r as *const bfdec_t != a);
    }
    if (*a).len == 0 as i32 as u64 {
        if (*a).expn == 9223372036854775807 as i64 {
            bfdec_set_nan(r);
            current_block = 11650488183268122163;
        } else if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 && (*a).sign != 0 {
            current_block = 5739228855100824132;
        } else {
            bfdec_set(r, a);
            current_block = 11650488183268122163;
        }
        match current_block {
            5739228855100824132 => {}
            _ => {
                ret = 0 as i32;
                current_block = 10380409671385728102;
            }
        }
    } else if (*a).sign != 0
        || prec
            == ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
    {
        current_block = 5739228855100824132;
    } else {
        if flags & ((1 as i32) << 4 as i32) as u32 != 0 {
            prec1 = bf_max(
                (floor_div((*a).expn + 1 as i32 as i64, 2 as i32 as slimb_t) as u64)
                    .wrapping_add(prec) as slimb_t,
                1 as i32 as slimb_t,
            )
        } else {
            prec1 = prec as slimb_t
        }
        /* convert the mantissa to an integer with at least 2 *
        prec + 4 digits */
        n = (2 as i32 as i64 * (prec1 + 2 as i32 as i64) + (2 as i32 * 19 as i32) as i64
            - 1 as i32 as i64)
            / (2 as i32 * 19 as i32) as i64;
        if bfdec_resize(r, n as limb_t) != 0 {
            current_block = 6745124862139863313;
        } else {
            a1 = bf_malloc(
                s,
                (::std::mem::size_of::<limb_t>() as u64)
                    .wrapping_mul(2 as i32 as u64)
                    .wrapping_mul(n as u64),
            ) as *mut limb_t;
            if a1.is_null() {
                current_block = 6745124862139863313;
            } else {
                n1 = bf_min(2 as i32 as i64 * n, (*a).len as slimb_t);
                (a1 as *mut u8).write_bytes(
                    0,
                    (2 * n as usize - n1 as usize).wrapping_mul(std::mem::size_of::<limb_t>()),
                );
                (a1.offset(2 * n as isize).offset(-(n1 as isize)) as *mut u8).copy_from(
                    (*a).tab.offset((*a).len as isize).offset(-(n1 as isize)) as *const u8,
                    (n1 as usize).wrapping_mul(std::mem::size_of::<limb_t>()),
                );
                if (*a).expn & 1 as i32 as i64 != 0 {
                    res = mp_shr_dec(
                        a1,
                        a1,
                        2 as i32 as i64 * n,
                        1 as i32 as limb_t,
                        0 as i32 as limb_t,
                    )
                } else {
                    res = 0 as i32 as limb_t
                }
                /* normalize so that a1 >= B^(2*n)/4. Not need for n = 1
                because mp_sqrtrem2_dec already does it */
                k = 0 as i32;
                if n > 1 as i32 as i64 {
                    v = *a1.offset((2 as i32 as i64 * n - 1 as i32 as i64) as isize);
                    while v < (10000000000000000000 as u64).wrapping_div(4 as i32 as u64) {
                        k += 1;
                        v = (v as u64).wrapping_mul(4 as i32 as u64) as limb_t as limb_t
                    }
                    if k != 0 as i32 {
                        mp_mul1_dec(
                            a1,
                            a1,
                            2 as i32 as i64 * n,
                            ((1 as i32) << 2 as i32 * k) as limb_t,
                            0 as i32 as limb_t,
                        );
                    }
                }
                if mp_sqrtrem_dec(s, (*r).tab, a1, n as limb_t) != 0 {
                    bf_free(s, a1 as *mut std::ffi::c_void);
                    current_block = 6745124862139863313;
                } else {
                    if k != 0 as i32 {
                        mp_div1_dec(
                            (*r).tab,
                            (*r).tab,
                            n,
                            ((1 as i32) << k) as limb_t,
                            0 as i32 as limb_t,
                        );
                    }
                    if res == 0 {
                        res = mp_scan_nz(a1, n + 1 as i32 as i64)
                    }
                    bf_free(s, a1 as *mut std::ffi::c_void);
                    if res == 0 {
                        res = mp_scan_nz((*a).tab, (*a).len.wrapping_sub(n1 as u64) as mp_size_t)
                    }
                    if res != 0 as i32 as u64 {
                        let ref mut fresh10 = *(*r).tab.offset(0 as i32 as isize);
                        *fresh10 |= 1 as i32 as u64
                    }
                    (*r).sign = 0 as i32;
                    (*r).expn = (*a).expn + 1 as i32 as i64 >> 1 as i32;
                    ret = bfdec_round(r, prec, flags);
                    current_block = 10380409671385728102;
                }
            }
        }
        match current_block {
            10380409671385728102 => {}
            _ => {
                bfdec_set_nan(r);
                return (1 as i32) << 5 as i32;
            }
        }
    }
    match current_block {
        5739228855100824132 => {
            bfdec_set_nan(r);
            ret = (1 as i32) << 0 as i32
        }
        _ => {}
    }
    return ret;
}
/* The rounding mode is always BF_RNDZ. Return BF_ST_OVERFLOW if there
is an overflow and 0 otherwise. No memory error is possible. */
#[no_mangle]
pub unsafe extern "C" fn bfdec_get_int32(mut pres: *mut i32, mut a: *const bfdec_t) -> i32 {
    let mut v: uint32_t = 0;
    let mut ret: i32 = 0;
    if (*a).expn >= 9223372036854775807 as i64 - 1 as i32 as i64 {
        ret = 0 as i32;
        if (*a).expn == 9223372036854775807 as i64 - 1 as i32 as i64 {
            v = (2147483647 as i32 as uint32_t).wrapping_add((*a).sign as u32)
        /* XXX: return overflow ? */
        } else {
            v = 2147483647 as i32 as uint32_t
        }
    } else if (*a).expn <= 0 as i32 as i64 {
        v = 0 as i32 as uint32_t;
        ret = 0 as i32
    } else if (*a).expn <= 9 as i32 as i64 {
        v = fast_shr_dec(
            *(*a)
                .tab
                .offset((*a).len.wrapping_sub(1 as i32 as u64) as isize),
            (19 as i32 as i64 - (*a).expn) as i32,
        ) as uint32_t;
        if (*a).sign != 0 {
            v = v.wrapping_neg()
        }
        ret = 0 as i32
    } else if (*a).expn == 10 as i32 as i64 {
        let mut v1: uint64_t = 0;
        let mut v_max: uint32_t = 0;
        v1 = fast_shr_dec(
            *(*a)
                .tab
                .offset((*a).len.wrapping_sub(1 as i32 as u64) as isize),
            (19 as i32 as i64 - (*a).expn) as i32,
        );
        v_max = (2147483647 as i32 as uint32_t).wrapping_add((*a).sign as u32);
        if v1 > v_max as u64 {
            v = v_max;
            ret = (1 as i32) << 2 as i32
        } else {
            v = v1 as uint32_t;
            if (*a).sign != 0 {
                v = v.wrapping_neg()
            }
            ret = 0 as i32
        }
    } else {
        v = (2147483647 as i32 as uint32_t).wrapping_add((*a).sign as u32);
        ret = (1 as i32) << 2 as i32
    }
    *pres = v as i32;
    return ret;
}
/* power to an integer with infinite precision */
#[no_mangle]
pub unsafe extern "C" fn bfdec_pow_ui(
    mut r: *mut bfdec_t,
    mut a: *const bfdec_t,
    mut b: limb_t,
) -> i32 {
    let mut ret: i32 = 0;
    let mut n_bits: i32 = 0;
    let mut i: i32 = 0;
    if r != a as *mut bfdec_t {
    } else {
        assert!(r as *const bfdec_t != a);
    }
    if b == 0 as i32 as u64 {
        return bfdec_set_ui(r, 1 as i32 as uint64_t);
    }
    ret = bfdec_set(r, a);
    n_bits = ((1 as i32) << 6 as i32) - clz(b);
    i = n_bits - 2 as i32;
    while i >= 0 as i32 {
        ret |= bfdec_mul(
            r,
            r,
            r,
            ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
            BF_RNDZ as i32 as bf_flags_t,
        );
        if b >> i & 1 as i32 as u64 != 0 {
            ret |= bfdec_mul(
                r,
                r,
                a,
                ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 2 as i32)
                    .wrapping_sub(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
                BF_RNDZ as i32 as bf_flags_t,
            )
        }
        i -= 1
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_ftoa(
    mut plen: *mut size_t,
    mut a: *const bfdec_t,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> *mut std::os::raw::c_char {
    return bf_ftoa_internal(plen, a as *const bf_t, 10 as i32, prec, flags, TRUE as i32);
}
#[no_mangle]
pub unsafe extern "C" fn bfdec_atof(
    mut r: *mut bfdec_t,
    mut str: *const std::os::raw::c_char,
    mut pnext: *mut *const std::os::raw::c_char,
    mut prec: limb_t,
    mut flags: bf_flags_t,
) -> i32 {
    let mut dummy_exp: slimb_t = 0;
    return bf_atof_internal(
        r as *mut bf_t,
        &mut dummy_exp,
        str,
        pnext,
        10 as i32,
        prec,
        flags,
        TRUE as i32,
    );
}
/* USE_BF_DEC */
/* **************************************************************/
/* Integer multiplication with FFT */
/* or LIMB_BITS at bit position 'pos' in tab */
#[inline]
unsafe extern "C" fn put_bits(
    mut tab: *mut limb_t,
    mut len: limb_t,
    mut pos: slimb_t,
    mut val: limb_t,
) {
    let mut i: limb_t = 0;
    let mut p: i32 = 0;
    i = (pos >> 6 as i32) as limb_t;
    p = (pos & (((1 as i32) << 6 as i32) - 1 as i32) as i64) as i32;
    if i < len {
        let ref mut fresh11 = *tab.offset(i as isize);
        *fresh11 |= val << p
    }
    if p != 0 as i32 {
        i = i.wrapping_add(1);
        if i < len {
            let ref mut fresh12 = *tab.offset(i as isize);
            *fresh12 |= val >> ((1 as i32) << 6 as i32) - p
        }
    };
}
static mut ntt_int_bits: [i32; 5] = [307 as i32, 246 as i32, 185 as i32, 123 as i32, 61 as i32];
static mut ntt_mods: [limb_t; 5] = [
    0x28d8000000000001 as i64 as limb_t,
    0x2a88000000000001 as i64 as limb_t,
    0x2ed8000000000001 as i64 as limb_t,
    0x3508000000000001 as i64 as limb_t,
    0x3aa8000000000001 as i64 as limb_t,
];
static mut ntt_proot: [[limb_t; 5]; 2] = [
    [
        0x1b8ea61034a2bea7 as i64 as limb_t,
        0x21a9762de58206fb as i64 as limb_t,
        0x2ca782f0756a8ea as i64 as limb_t,
        0x278384537a3e50a1 as i64 as limb_t,
        0x106e13fee74ce0ab as i64 as limb_t,
    ],
    [
        0x233513af133e13b8 as i64 as limb_t,
        0x1d13140d1c6f75f1 as i64 as limb_t,
        0x12cde57f97e3eeda as i64 as limb_t,
        0xd6149e23cbe654f as i64 as limb_t,
        0x36cd204f522a1379 as i64 as limb_t,
    ],
];
static mut ntt_mods_cr: [limb_t; 10] = [
    0x8a9ed097b425eea as i64 as limb_t,
    0x18a44aaaaaaaaab3 as i64 as limb_t,
    0x2493f57f57f57f5d as i64 as limb_t,
    0x126b8d0649a7f8d4 as i64 as limb_t,
    0x9d80ed7303b5ccc as i64 as limb_t,
    0x25b8bcf3cf3cf3d5 as i64 as limb_t,
    0x2ce6ce63398ce638 as i64 as limb_t,
    0xe31fad40a57eb59 as i64 as limb_t,
    0x2a3529fd4a7f52f as i64 as limb_t,
    0x3a5493e93e93e94a as i64 as limb_t,
];
/* add modulo with up to (LIMB_BITS-1) bit modulo */
#[inline]
unsafe extern "C" fn add_mod(mut a: limb_t, mut b: limb_t, mut m: limb_t) -> limb_t {
    let mut r: limb_t = 0;
    r = a.wrapping_add(b);
    if r >= m {
        r = (r as u64).wrapping_sub(m) as limb_t as limb_t
    }
    return r;
}
/* sub modulo with up to LIMB_BITS bit modulo */
#[inline]
unsafe extern "C" fn sub_mod(mut a: limb_t, mut b: limb_t, mut m: limb_t) -> limb_t {
    let mut r: limb_t = 0;
    r = a.wrapping_sub(b);
    if r > a {
        r = (r as u64).wrapping_add(m) as limb_t as limb_t
    }
    return r;
}
/* return (r0+r1*B) mod m
   precondition: 0 <= r0+r1*B < 2^(64+NTT_MOD_LOG2_MIN)
*/
#[inline]
unsafe extern "C" fn mod_fast(mut r: dlimb_t, mut m: limb_t, mut m_inv: limb_t) -> limb_t {
    let mut a1: limb_t = 0;
    let mut q: limb_t = 0;
    let mut t0: limb_t = 0;
    let mut r1: limb_t = 0;
    let mut r0: limb_t = 0;
    a1 = (r >> 61 as i32) as limb_t;
    q = ((a1 as dlimb_t).wrapping_mul(m_inv as u128) >> ((1 as i32) << 6 as i32)) as limb_t;
    r = r
        .wrapping_sub((q as dlimb_t).wrapping_mul(m as u128))
        .wrapping_sub(m.wrapping_mul(2 as i32 as u64) as u128);
    r1 = (r >> ((1 as i32) << 6 as i32)) as limb_t;
    t0 = (r1 as slimb_t >> 1 as i32) as limb_t;
    r = (r as u128).wrapping_add((m & t0) as u128) as dlimb_t as dlimb_t;
    r0 = r as limb_t;
    r1 = (r >> ((1 as i32) << 6 as i32)) as limb_t;
    r0 = (r0 as u64).wrapping_add(m & r1) as limb_t as limb_t;
    return r0;
}
/* faster version using precomputed modulo inverse.
precondition: 0 <= a * b < 2^(64+NTT_MOD_LOG2_MIN) */
#[inline]
unsafe extern "C" fn mul_mod_fast(
    mut a: limb_t,
    mut b: limb_t,
    mut m: limb_t,
    mut m_inv: limb_t,
) -> limb_t {
    let mut r: dlimb_t = 0;
    r = (a as dlimb_t).wrapping_mul(b as dlimb_t);
    return mod_fast(r, m, m_inv);
}
#[inline]
unsafe extern "C" fn init_mul_mod_fast(mut m: limb_t) -> limb_t {
    let mut t: dlimb_t = 0;
    if m < (1 as i32 as limb_t) << 62 as i32 {
    } else {
        assert!(m < 1 << NTT_MOD_LOG2_MAX);
    }
    if m >= (1 as i32 as limb_t) << 61 as i32 {
    } else {
        assert!(m >= 1 << NTT_MOD_LOG2_MIN);
    }
    t = (1 as i32 as dlimb_t) << ((1 as i32) << 6 as i32) + 61 as i32;
    return t.wrapping_div(m as u128) as limb_t;
}
/* Faster version used when the multiplier is constant. 0 <= a < 2^64,
0 <= b < m. */
#[inline]
unsafe extern "C" fn mul_mod_fast2(
    mut a: limb_t,
    mut b: limb_t,
    mut m: limb_t,
    mut b_inv: limb_t,
) -> limb_t {
    let mut r: limb_t = 0;
    let mut q: limb_t = 0;
    q = ((a as dlimb_t).wrapping_mul(b_inv as dlimb_t) >> ((1 as i32) << 6 as i32)) as limb_t;
    r = a.wrapping_mul(b).wrapping_sub(q.wrapping_mul(m));
    if r >= m {
        r = (r as u64).wrapping_sub(m) as limb_t as limb_t
    }
    return r;
}
/* Faster version used when the multiplier is constant. 0 <= a < 2^64,
0 <= b < m. Let r = a * b mod m. The return value is 'r' or 'r +
m'. */
#[inline]
unsafe extern "C" fn mul_mod_fast3(
    mut a: limb_t,
    mut b: limb_t,
    mut m: limb_t,
    mut b_inv: limb_t,
) -> limb_t {
    let mut r: limb_t = 0;
    let mut q: limb_t = 0;
    q = ((a as dlimb_t).wrapping_mul(b_inv as dlimb_t) >> ((1 as i32) << 6 as i32)) as limb_t;
    r = a.wrapping_mul(b).wrapping_sub(q.wrapping_mul(m));
    return r;
}
#[inline]
unsafe extern "C" fn init_mul_mod_fast2(mut b: limb_t, mut m: limb_t) -> limb_t {
    return ((b as dlimb_t) << ((1 as i32) << 6 as i32)).wrapping_div(m as u128) as limb_t;
}
unsafe extern "C" fn ntt_malloc(mut s: *mut BFNTTState, mut size: size_t) -> *mut std::ffi::c_void {
    return bf_malloc((*s).ctx, size);
}
unsafe extern "C" fn ntt_free(mut s: *mut BFNTTState, mut ptr: *mut std::ffi::c_void) {
    bf_free((*s).ctx, ptr);
}
#[inline]
unsafe extern "C" fn ntt_limb_to_int(mut a: NTTLimb, mut m: limb_t) -> limb_t {
    if a >= m {
        a = (a as u64).wrapping_sub(m) as NTTLimb as NTTLimb
    }
    return a;
}
#[inline]
unsafe extern "C" fn int_to_ntt_limb(mut a: slimb_t, mut m: limb_t) -> NTTLimb {
    return a as NTTLimb;
}
#[inline(never)]
unsafe extern "C" fn ntt_fft(
    mut s: *mut BFNTTState,
    mut out_buf: *mut NTTLimb,
    mut in_buf: *mut NTTLimb,
    mut tmp_buf: *mut NTTLimb,
    mut fft_len_log2: i32,
    mut inverse: i32,
    mut m_idx: i32,
) -> i32 {
    let mut nb_blocks: limb_t = 0;
    let mut fft_per_block: limb_t = 0;
    let mut p: limb_t = 0;
    let mut k: limb_t = 0;
    let mut n: limb_t = 0;
    let mut stride_in: limb_t = 0;
    let mut i: limb_t = 0;
    let mut j: limb_t = 0;
    let mut m: limb_t = 0;
    let mut m2: limb_t = 0;
    let mut tab_in: *mut NTTLimb = 0 as *mut NTTLimb;
    let mut tab_out: *mut NTTLimb = 0 as *mut NTTLimb;
    let mut tmp: *mut NTTLimb = 0 as *mut NTTLimb;
    let mut a0: NTTLimb = 0;
    let mut a1: NTTLimb = 0;
    let mut b0: NTTLimb = 0;
    let mut b1: NTTLimb = 0;
    let mut c: NTTLimb = 0;
    let mut trig: *mut NTTLimb = 0 as *mut NTTLimb;
    let mut c_inv: NTTLimb = 0;
    let mut l: i32 = 0;
    m = ntt_mods[m_idx as usize];
    m2 = (2 as i32 as u64).wrapping_mul(m);
    n = (1 as i32 as limb_t) << fft_len_log2;
    nb_blocks = n;
    fft_per_block = 1 as i32 as limb_t;
    stride_in = n.wrapping_div(2 as i32 as u64);
    tab_in = in_buf;
    tab_out = tmp_buf;
    l = fft_len_log2;
    while nb_blocks != 2 as i32 as u64 {
        nb_blocks >>= 1 as i32;
        p = 0 as i32 as limb_t;
        k = 0 as i32 as limb_t;
        trig = get_trig(s, l, inverse, m_idx);
        if trig.is_null() {
            return -(1 as i32);
        }
        i = 0 as i32 as limb_t;
        while i < nb_blocks {
            c = *trig.offset(0 as i32 as isize);
            c_inv = *trig.offset(1 as i32 as isize);
            trig = trig.offset(2 as i32 as isize);
            j = 0 as i32 as limb_t;
            while j < fft_per_block {
                a0 = *tab_in.offset(k.wrapping_add(j) as isize);
                a1 = *tab_in.offset(k.wrapping_add(j).wrapping_add(stride_in) as isize);
                b0 = add_mod(a0, a1, m2);
                b1 = a0.wrapping_sub(a1).wrapping_add(m2);
                b1 = mul_mod_fast3(b1, c, m, c_inv);
                *tab_out.offset(p.wrapping_add(j) as isize) = b0;
                *tab_out.offset(p.wrapping_add(j).wrapping_add(fft_per_block) as isize) = b1;
                j = j.wrapping_add(1)
            }
            k = (k as u64).wrapping_add(fft_per_block) as limb_t as limb_t;
            p = (p as u64).wrapping_add((2 as i32 as u64).wrapping_mul(fft_per_block)) as limb_t
                as limb_t;
            i = i.wrapping_add(1)
        }
        fft_per_block <<= 1 as i32;
        l -= 1;
        tmp = tab_in;
        tab_in = tab_out;
        tab_out = tmp
    }
    /* no twiddle in last step */
    tab_out = out_buf;
    k = 0 as i32 as limb_t;
    while k < stride_in {
        a0 = *tab_in.offset(k as isize);
        a1 = *tab_in.offset(k.wrapping_add(stride_in) as isize);
        b0 = add_mod(a0, a1, m2);
        b1 = sub_mod(a0, a1, m2);
        *tab_out.offset(k as isize) = b0;
        *tab_out.offset(k.wrapping_add(stride_in) as isize) = b1;
        k = k.wrapping_add(1)
    }
    return 0 as i32;
}
unsafe extern "C" fn ntt_vec_mul(
    mut s: *mut BFNTTState,
    mut tab1: *mut NTTLimb,
    mut tab2: *mut NTTLimb,
    mut fft_len_log2: i32,
    mut k_tot: i32,
    mut m_idx: i32,
) {
    let mut i: limb_t = 0;
    let mut norm: limb_t = 0;
    let mut norm_inv: limb_t = 0;
    let mut a: limb_t = 0;
    let mut n: limb_t = 0;
    let mut m: limb_t = 0;
    let mut m_inv: limb_t = 0;
    m = ntt_mods[m_idx as usize];
    m_inv = (*s).ntt_mods_div[m_idx as usize];
    norm = (*s).ntt_len_inv[m_idx as usize][k_tot as usize][0 as i32 as usize];
    norm_inv = (*s).ntt_len_inv[m_idx as usize][k_tot as usize][1 as i32 as usize];
    n = (1 as i32 as limb_t) << fft_len_log2;
    i = 0 as i32 as limb_t;
    while i < n {
        a = *tab1.offset(i as isize);
        /* need to reduce the range so that the product is <
        2^(LIMB_BITS+NTT_MOD_LOG2_MIN) */
        if a >= m {
            a = (a as u64).wrapping_sub(m) as limb_t as limb_t
        }
        a = mul_mod_fast(a, *tab2.offset(i as isize), m, m_inv);
        a = mul_mod_fast3(a, norm, m, norm_inv);
        *tab1.offset(i as isize) = a;
        i = i.wrapping_add(1)
    }
}
#[inline(never)]
unsafe extern "C" fn mul_trig(
    mut buf: *mut NTTLimb,
    mut n: limb_t,
    mut c_mul: limb_t,
    mut m: limb_t,
    mut m_inv: limb_t,
) {
    let mut i: limb_t = 0;
    let mut c0: limb_t = 0;
    let mut c_mul_inv: limb_t = 0;
    c0 = 1 as i32 as limb_t;
    c_mul_inv = init_mul_mod_fast2(c_mul, m);
    i = 0 as i32 as limb_t;
    while i < n {
        *buf.offset(i as isize) = mul_mod_fast(*buf.offset(i as isize), c0, m, m_inv);
        c0 = mul_mod_fast2(c0, c_mul, m, c_mul_inv);
        i = i.wrapping_add(1)
    }
}
/* !AVX2 */
#[inline(never)]
unsafe extern "C" fn get_trig(
    mut s: *mut BFNTTState,
    mut k: i32,
    mut inverse: i32,
    mut m_idx: i32,
) -> *mut NTTLimb {
    let mut tab: *mut NTTLimb = 0 as *mut NTTLimb;
    let mut i: limb_t = 0;
    let mut n2: limb_t = 0;
    let mut c: limb_t = 0;
    let mut c_mul: limb_t = 0;
    let mut m: limb_t = 0;
    let mut c_mul_inv: limb_t = 0;
    if k > 19 as i32 {
        return 0 as *mut NTTLimb;
    }
    tab = (*s).ntt_trig[m_idx as usize][inverse as usize][k as usize];
    if !tab.is_null() {
        return tab;
    }
    n2 = (1 as i32 as limb_t) << k - 1 as i32;
    m = ntt_mods[m_idx as usize];
    tab = ntt_malloc(
        s,
        (::std::mem::size_of::<NTTLimb>() as u64)
            .wrapping_mul(n2)
            .wrapping_mul(2 as i32 as u64),
    ) as *mut NTTLimb;
    if tab.is_null() {
        return 0 as *mut NTTLimb;
    }
    c = 1 as i32 as limb_t;
    c_mul = (*s).ntt_proot_pow[m_idx as usize][inverse as usize][k as usize];
    c_mul_inv = (*s).ntt_proot_pow_inv[m_idx as usize][inverse as usize][k as usize];
    i = 0 as i32 as limb_t;
    while i < n2 {
        *tab.offset((2 as i32 as u64).wrapping_mul(i) as isize) = int_to_ntt_limb(c as slimb_t, m);
        *tab.offset(
            (2 as i32 as u64)
                .wrapping_mul(i)
                .wrapping_add(1 as i32 as u64) as isize,
        ) = init_mul_mod_fast2(c, m);
        c = mul_mod_fast2(c, c_mul, m, c_mul_inv);
        i = i.wrapping_add(1)
    }
    (*s).ntt_trig[m_idx as usize][inverse as usize][k as usize] = tab;
    return tab;
}
unsafe extern "C" fn fft_clear_cache(mut s1: *mut bf_context_t) {
    let mut m_idx: i32 = 0;
    let mut inverse: i32 = 0;
    let mut k: i32 = 0;
    let mut s: *mut BFNTTState = (*s1).ntt_state;
    if !s.is_null() {
        m_idx = 0 as i32;
        while m_idx < 5 as i32 {
            inverse = 0 as i32;
            while inverse < 2 as i32 {
                k = 0 as i32;
                while k < 19 as i32 + 1 as i32 {
                    if !(*s).ntt_trig[m_idx as usize][inverse as usize][k as usize].is_null() {
                        ntt_free(
                            s,
                            (*s).ntt_trig[m_idx as usize][inverse as usize][k as usize]
                                as *mut std::ffi::c_void,
                        );
                        (*s).ntt_trig[m_idx as usize][inverse as usize][k as usize] =
                            0 as *mut NTTLimb
                    }
                    k += 1
                }
                inverse += 1
            }
            m_idx += 1
        }
        bf_free(s1, s as *mut std::ffi::c_void);
        (*s1).ntt_state = 0 as *mut BFNTTState
    };
}
/* dst = buf1, src = buf2 */
unsafe extern "C" fn ntt_fft_partial(
    mut s: *mut BFNTTState,
    mut buf1: *mut NTTLimb,
    mut k1: i32,
    mut k2: i32,
    mut n1: limb_t,
    mut n2: limb_t,
    mut inverse: i32,
    mut m_idx: limb_t,
) -> i32 {
    let mut current_block: u64;
    let mut i: limb_t = 0;
    let mut j: limb_t = 0;
    let mut c_mul: limb_t = 0;
    let mut c0: limb_t = 0;
    let mut m: limb_t = 0;
    let mut m_inv: limb_t = 0;
    let mut strip_len: limb_t = 0;
    let mut l: limb_t = 0;
    let mut buf2: *mut NTTLimb = 0 as *mut NTTLimb;
    let mut buf3: *mut NTTLimb = 0 as *mut NTTLimb;
    buf2 = 0 as *mut NTTLimb;
    buf3 = ntt_malloc(
        s,
        (::std::mem::size_of::<NTTLimb>() as u64).wrapping_mul(n1),
    ) as *mut NTTLimb;
    if !buf3.is_null() {
        if k2 == 0 as i32 {
            if ntt_fft(s, buf1, buf1, buf3, k1, inverse, m_idx as i32) != 0 {
                current_block = 7202299292956205343;
            } else {
                current_block = 3934796541983872331;
            }
        } else {
            strip_len = 16 as i32 as limb_t;
            buf2 = ntt_malloc(
                s,
                (::std::mem::size_of::<NTTLimb>() as u64)
                    .wrapping_mul(n1)
                    .wrapping_mul(strip_len),
            ) as *mut NTTLimb;
            if buf2.is_null() {
                current_block = 7202299292956205343;
            } else {
                m = ntt_mods[m_idx as usize];
                m_inv = (*s).ntt_mods_div[m_idx as usize];
                c0 = (*s).ntt_proot_pow[m_idx as usize][inverse as usize][(k1 + k2) as usize];
                c_mul = 1 as i32 as limb_t;
                if n2.wrapping_rem(strip_len) == 0 as i32 as u64 {
                } else {
                    assert!((n2 % strip_len) == 0);
                }
                j = 0 as i32 as limb_t;
                's_71: loop {
                    if !(j < n2) {
                        current_block = 14072441030219150333;
                        break;
                    }
                    i = 0 as i32 as limb_t;
                    while i < n1 {
                        l = 0 as i32 as limb_t;
                        while l < strip_len {
                            *buf2.offset(i.wrapping_add(l.wrapping_mul(n1)) as isize) =
                                *buf1.offset(
                                    i.wrapping_mul(n2).wrapping_add(j.wrapping_add(l)) as isize
                                );
                            l = l.wrapping_add(1)
                        }
                        i = i.wrapping_add(1)
                    }
                    l = 0 as i32 as limb_t;
                    while l < strip_len {
                        if inverse != 0 {
                            mul_trig(
                                buf2.offset(l.wrapping_mul(n1) as isize),
                                n1,
                                c_mul,
                                m,
                                m_inv,
                            );
                        }
                        if ntt_fft(
                            s,
                            buf2.offset(l.wrapping_mul(n1) as isize),
                            buf2.offset(l.wrapping_mul(n1) as isize),
                            buf3,
                            k1,
                            inverse,
                            m_idx as i32,
                        ) != 0
                        {
                            current_block = 7202299292956205343;
                            break 's_71;
                        }
                        if inverse == 0 {
                            mul_trig(
                                buf2.offset(l.wrapping_mul(n1) as isize),
                                n1,
                                c_mul,
                                m,
                                m_inv,
                            );
                        }
                        c_mul = mul_mod_fast(c_mul, c0, m, m_inv);
                        l = l.wrapping_add(1)
                    }
                    i = 0 as i32 as limb_t;
                    while i < n1 {
                        l = 0 as i32 as limb_t;
                        while l < strip_len {
                            *buf1.offset(
                                i.wrapping_mul(n2).wrapping_add(j.wrapping_add(l)) as isize
                            ) = *buf2.offset(i.wrapping_add(l.wrapping_mul(n1)) as isize);
                            l = l.wrapping_add(1)
                        }
                        i = i.wrapping_add(1)
                    }
                    j = (j as u64).wrapping_add(strip_len) as limb_t as limb_t
                }
                match current_block {
                    7202299292956205343 => {}
                    _ => {
                        ntt_free(s, buf2 as *mut std::ffi::c_void);
                        current_block = 3934796541983872331;
                    }
                }
            }
        }
        match current_block {
            7202299292956205343 => {}
            _ => {
                ntt_free(s, buf3 as *mut std::ffi::c_void);
                return 0 as i32;
            }
        }
    }
    ntt_free(s, buf2 as *mut std::ffi::c_void);
    ntt_free(s, buf3 as *mut std::ffi::c_void);
    return -(1 as i32);
}
/* dst = buf1, src = buf2, tmp = buf3 */
unsafe extern "C" fn ntt_conv(
    mut s: *mut BFNTTState,
    mut buf1: *mut NTTLimb,
    mut buf2: *mut NTTLimb,
    mut k: i32,
    mut k_tot: i32,
    mut m_idx: limb_t,
) -> i32 {
    let mut n1: limb_t = 0;
    let mut n2: limb_t = 0;
    let mut i: limb_t = 0;
    let mut k1: i32 = 0;
    let mut k2: i32 = 0;
    if k <= 19 as i32 {
        k1 = k
    } else {
        /* recursive split of the FFT */
        k1 = bf_min((k / 2 as i32) as slimb_t, 19 as i32 as slimb_t) as i32
    }
    k2 = k - k1;
    n1 = (1 as i32 as limb_t) << k1;
    n2 = (1 as i32 as limb_t) << k2;
    if ntt_fft_partial(s, buf1, k1, k2, n1, n2, 0 as i32, m_idx) != 0 {
        return -(1 as i32);
    }
    if ntt_fft_partial(s, buf2, k1, k2, n1, n2, 0 as i32, m_idx) != 0 {
        return -(1 as i32);
    }
    if k2 == 0 as i32 {
        ntt_vec_mul(s, buf1, buf2, k, k_tot, m_idx as i32);
    } else {
        i = 0 as i32 as limb_t;
        while i < n1 {
            ntt_conv(
                s,
                buf1.offset(i.wrapping_mul(n2) as isize),
                buf2.offset(i.wrapping_mul(n2) as isize),
                k2,
                k_tot,
                m_idx,
            );
            i = i.wrapping_add(1)
        }
    }
    if ntt_fft_partial(s, buf1, k1, k2, n1, n2, 1 as i32, m_idx) != 0 {
        return -(1 as i32);
    }
    return 0 as i32;
}
#[inline(never)]
unsafe extern "C" fn limb_to_ntt(
    mut s: *mut BFNTTState,
    mut tabr: *mut NTTLimb,
    mut fft_len: limb_t,
    mut taba: *const limb_t,
    mut a_len: limb_t,
    mut dpl: i32,
    mut first_m_idx: i32,
    mut nb_mods: i32,
) {
    let mut i: slimb_t = 0;
    let mut n: slimb_t = 0;
    let mut a: dlimb_t = 0;
    let mut b: dlimb_t = 0;
    let mut j: i32 = 0;
    let mut shift: i32 = 0;
    let mut base_mask1: limb_t = 0;
    let mut a0: limb_t = 0;
    let mut a1: limb_t = 0;
    let mut a2: limb_t = 0;
    let mut r: limb_t = 0;
    let mut m: limb_t = 0;
    let mut m_inv: limb_t = 0;
    (tabr as *mut u8).write_bytes(
        0,
        std::mem::size_of::<NTTLimb>()
            .wrapping_mul(fft_len as usize)
            .wrapping_mul(nb_mods as usize),
    );
    shift = dpl & ((1 as i32) << 6 as i32) - 1 as i32;
    if shift == 0 as i32 {
        base_mask1 = -(1 as i32) as limb_t
    } else {
        base_mask1 = ((1 as i32 as limb_t) << shift).wrapping_sub(1 as i32 as u64)
    }
    n = bf_min(
        fft_len as slimb_t,
        a_len
            .wrapping_mul(((1 as i32) << 6 as i32) as u64)
            .wrapping_add(dpl as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(dpl as u64) as slimb_t,
    );
    i = 0 as i32 as slimb_t;
    while i < n {
        a0 = get_bits(taba, a_len, i * dpl as i64);
        if dpl <= (1 as i32) << 6 as i32 {
            a0 &= base_mask1;
            a = a0 as dlimb_t
        } else {
            a1 = get_bits(
                taba,
                a_len,
                i * dpl as i64 + ((1 as i32) << 6 as i32) as i64,
            );
            if dpl <= ((1 as i32) << 6 as i32) + 61 as i32 {
                a = a0 as u128 | ((a1 & base_mask1) as dlimb_t) << ((1 as i32) << 6 as i32)
            } else {
                if dpl > 2 as i32 * ((1 as i32) << 6 as i32) {
                    a2 = get_bits(
                        taba,
                        a_len,
                        i * dpl as i64 + (((1 as i32) << 6 as i32) * 2 as i32) as i64,
                    ) & base_mask1
                } else {
                    a1 &= base_mask1;
                    a2 = 0 as i32 as limb_t
                }
                //            printf("a=0x%016lx%016lx%016lx\n", a2, a1, a0);
                a = (a0 >> ((1 as i32) << 6 as i32) - 62 as i32 + 61 as i32) as u128
                    | (a1 as dlimb_t) << 62 as i32 - 61 as i32
                    | (a2 as dlimb_t) << ((1 as i32) << 6 as i32) + 62 as i32 - 61 as i32; /* avoid warnings */
                a0 &= ((1 as i32 as limb_t) << ((1 as i32) << 6 as i32) - 62 as i32 + 61 as i32)
                    .wrapping_sub(1 as i32 as u64)
            }
        }
        j = 0 as i32;
        while j < nb_mods {
            m = ntt_mods[(first_m_idx + j) as usize];
            m_inv = (*s).ntt_mods_div[(first_m_idx + j) as usize];
            r = mod_fast(a, m, m_inv);
            if dpl > ((1 as i32) << 6 as i32) + 61 as i32 {
                b = (r as dlimb_t) << ((1 as i32) << 6 as i32) - 62 as i32 + 61 as i32 | a0 as u128;
                r = mod_fast(b, m, m_inv)
            }
            *tabr.offset((i as u64).wrapping_add((j as u64).wrapping_mul(fft_len)) as isize) =
                int_to_ntt_limb(r as slimb_t, m);
            j += 1
        }
        i += 1
    }
}
#[inline(never)]
unsafe extern "C" fn ntt_to_limb(
    mut s: *mut BFNTTState,
    mut tabr: *mut limb_t,
    mut r_len: limb_t,
    mut buf: *const NTTLimb,
    mut fft_len_log2: i32,
    mut dpl: i32,
    mut nb_mods: i32,
) {
    let mut mods: *const limb_t = ntt_mods
        .as_ptr()
        .offset(5 as i32 as isize)
        .offset(-(nb_mods as isize));
    let mut mods_cr: *const limb_t = 0 as *const limb_t;
    let mut mods_cr_inv: *const limb_t = 0 as *const limb_t;
    let mut y: [limb_t; 5] = [0; 5];
    let mut u: [limb_t; 5] = [0; 5];
    let mut carry: [limb_t; 5] = [0; 5];
    let mut fft_len: limb_t = 0;
    let mut base_mask1: limb_t = 0;
    let mut r: limb_t = 0;
    let mut i: slimb_t = 0;
    let mut len: slimb_t = 0;
    let mut pos: slimb_t = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut shift: i32 = 0;
    let mut n_limb1: i32 = 0;
    let mut t: dlimb_t = 0;
    j = 5 as i32 * (5 as i32 - 1 as i32) / 2 as i32 - nb_mods * (nb_mods - 1 as i32) / 2 as i32;
    mods_cr = ntt_mods_cr.as_ptr().offset(j as isize);
    mods_cr_inv = (*s).ntt_mods_cr_inv.as_mut_ptr().offset(j as isize);
    shift = dpl & ((1 as i32) << 6 as i32) - 1 as i32;
    if shift == 0 as i32 {
        base_mask1 = -(1 as i32) as limb_t
    } else {
        base_mask1 = ((1 as i32 as limb_t) << shift).wrapping_sub(1 as i32 as u64)
    }
    n_limb1 = (dpl as u32)
        .wrapping_sub(1 as i32 as u32)
        .wrapping_div(((1 as i32) << 6 as i32) as u32) as i32;
    j = 0 as i32;
    while j < 5 as i32 {
        carry[j as usize] = 0 as i32 as limb_t;
        j += 1
    }
    j = 0 as i32;
    while j < 5 as i32 {
        u[j as usize] = 0 as i32 as limb_t;
        j += 1
    }
    (tabr as *mut u8).write_bytes(
        0,
        std::mem::size_of::<limb_t>().wrapping_mul(r_len as usize),
    );
    fft_len = (1 as i32 as limb_t) << fft_len_log2;
    len = bf_min(
        fft_len as slimb_t,
        r_len
            .wrapping_mul(((1 as i32) << 6 as i32) as u64)
            .wrapping_add(dpl as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(dpl as u64) as slimb_t,
    );
    i = 0 as i32 as slimb_t;
    while i < len {
        j = 0 as i32;
        while j < nb_mods {
            y[j as usize] = ntt_limb_to_int(
                *buf.offset((i as u64).wrapping_add(fft_len.wrapping_mul(j as u64)) as isize),
                *mods.offset(j as isize),
            );
            j += 1
        }
        /* Chinese remainder to get mixed radix representation */
        l = 0 as i32;
        j = 0 as i32;
        while j < nb_mods - 1 as i32 {
            k = j + 1 as i32;
            while k < nb_mods {
                let mut m: limb_t = 0;
                m = *mods.offset(k as isize);
                /* Note: there is no overflow in the sub_mod() because
                the modulos are sorted by increasing order */
                y[k as usize] = mul_mod_fast2(
                    y[k as usize].wrapping_sub(y[j as usize]).wrapping_add(m),
                    *mods_cr.offset(l as isize),
                    m,
                    *mods_cr_inv.offset(l as isize),
                );
                l += 1;
                k += 1
            }
            j += 1
        }
        /* back to normal representation */
        u[0 as i32 as usize] = y[(nb_mods - 1 as i32) as usize];
        l = 1 as i32;
        j = nb_mods - 2 as i32;
        while j >= 1 as i32 {
            r = y[j as usize];
            k = 0 as i32;
            while k < l {
                t = (u[k as usize] as dlimb_t)
                    .wrapping_mul(*mods.offset(j as isize) as u128)
                    .wrapping_add(r as u128);
                r = (t >> ((1 as i32) << 6 as i32)) as limb_t;
                u[k as usize] = t as limb_t;
                k += 1
            }
            u[l as usize] = r;
            l += 1;
            j -= 1
        }
        /* last step adds the carry */
        r = y[0 as i32 as usize];
        k = 0 as i32;
        while k < l {
            t = (u[k as usize] as dlimb_t)
                .wrapping_mul(*mods.offset(j as isize) as u128)
                .wrapping_add(r as u128)
                .wrapping_add(carry[k as usize] as u128);
            r = (t >> ((1 as i32) << 6 as i32)) as limb_t;
            u[k as usize] = t as limb_t;
            k += 1
        }
        u[l as usize] = r.wrapping_add(carry[l as usize]);
        /* write the digits */
        pos = i * dpl as i64;
        j = 0 as i32;
        while j < n_limb1 {
            put_bits(tabr, r_len, pos, u[j as usize]);
            pos += ((1 as i32) << 6 as i32) as i64;
            j += 1
        }
        put_bits(tabr, r_len, pos, u[n_limb1 as usize] & base_mask1);
        /* shift by dpl digits and set the carry */
        if shift == 0 as i32 {
            j = n_limb1 + 1 as i32; /* 1/2 */
            while j < nb_mods {
                carry[(j - (n_limb1 + 1 as i32)) as usize] = u[j as usize];
                j += 1
            }
        } else {
            j = n_limb1;
            while j < nb_mods - 1 as i32 {
                carry[(j - n_limb1) as usize] = u[j as usize] >> shift
                    | u[(j + 1 as i32) as usize] << ((1 as i32) << 6 as i32) - shift;
                j += 1
            }
            carry[(nb_mods - 1 as i32 - n_limb1) as usize] =
                u[(nb_mods - 1 as i32) as usize] >> shift
        }
        i += 1
    }
}
unsafe extern "C" fn ntt_static_init(mut s1: *mut bf_context_t) -> i32 {
    let mut s: *mut BFNTTState = 0 as *mut BFNTTState;
    let mut inverse: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut c: limb_t = 0;
    let mut c_inv: limb_t = 0;
    let mut c_inv2: limb_t = 0;
    let mut m: limb_t = 0;
    let mut m_inv: limb_t = 0;
    if !(*s1).ntt_state.is_null() {
        return 0 as i32;
    }
    s = bf_malloc(s1, ::std::mem::size_of::<BFNTTState>() as u64) as *mut BFNTTState;
    if s.is_null() {
        return -(1 as i32);
    }
    (s as *mut u8).write_bytes(0, std::mem::size_of::<BFNTTState>());
    (*s1).ntt_state = s;
    (*s).ctx = s1;
    j = 0 as i32;
    while j < 5 as i32 {
        m = ntt_mods[j as usize];
        m_inv = init_mul_mod_fast(m);
        (*s).ntt_mods_div[j as usize] = m_inv;
        c_inv2 = m
            .wrapping_add(1 as i32 as u64)
            .wrapping_div(2 as i32 as u64);
        c_inv = 1 as i32 as limb_t;
        i = 0 as i32;
        while i <= 51 as i32 {
            (*s).ntt_len_inv[j as usize][i as usize][0 as i32 as usize] = c_inv;
            (*s).ntt_len_inv[j as usize][i as usize][1 as i32 as usize] =
                init_mul_mod_fast2(c_inv, m);
            c_inv = mul_mod_fast(c_inv, c_inv2, m, m_inv);
            i += 1
        }
        inverse = 0 as i32;
        while inverse < 2 as i32 {
            c = ntt_proot[inverse as usize][j as usize];
            i = 0 as i32;
            while i < 51 as i32 {
                (*s).ntt_proot_pow[j as usize][inverse as usize][(51 as i32 - i) as usize] = c;
                (*s).ntt_proot_pow_inv[j as usize][inverse as usize][(51 as i32 - i) as usize] =
                    init_mul_mod_fast2(c, m);
                c = mul_mod_fast(c, c, m, m_inv);
                i += 1
            }
            inverse += 1
        }
        j += 1
    }
    l = 0 as i32;
    j = 0 as i32;
    while j < 5 as i32 - 1 as i32 {
        k = j + 1 as i32;
        while k < 5 as i32 {
            (*s).ntt_mods_cr_inv[l as usize] =
                init_mul_mod_fast2(ntt_mods_cr[l as usize], ntt_mods[k as usize]);
            l += 1;
            k += 1
        }
        j += 1
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bf_get_fft_size(
    mut pdpl: *mut i32,
    mut pnb_mods: *mut i32,
    mut len: limb_t,
) -> i32 {
    let mut dpl: i32 = 0;
    let mut fft_len_log2: i32 = 0;
    let mut n_bits: i32 = 0;
    let mut nb_mods: i32 = 0;
    let mut dpl_found: i32 = 0;
    let mut fft_len_log2_found: i32 = 0;
    let mut int_bits: i32 = 0;
    let mut nb_mods_found: i32 = 0;
    let mut cost: limb_t = 0;
    let mut min_cost: limb_t = 0;
    min_cost = -(1 as i32) as limb_t;
    dpl_found = 0 as i32;
    nb_mods_found = 4 as i32;
    fft_len_log2_found = 0 as i32;
    nb_mods = 3 as i32;
    while nb_mods <= 5 as i32 {
        int_bits = ntt_int_bits[(5 as i32 - nb_mods) as usize];
        dpl = bf_min(
            ((int_bits - 4 as i32) / 2 as i32) as slimb_t,
            (2 as i32 * ((1 as i32) << 6 as i32) + 2 as i32 * 61 as i32 - 62 as i32) as slimb_t,
        ) as i32;
        loop {
            fft_len_log2 = ceil_log2(
                len.wrapping_mul(((1 as i32) << 6 as i32) as u64)
                    .wrapping_add(dpl as u64)
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_div(dpl as u64),
            );
            if fft_len_log2 > 51 as i32 {
                break;
            }
            n_bits = fft_len_log2 + 2 as i32 * dpl;
            if n_bits <= int_bits {
                cost = (((fft_len_log2 + 1 as i32) as limb_t) << fft_len_log2)
                    .wrapping_mul(nb_mods as u64);
                //                printf("n=%d dpl=%d: cost=%" PRId64 "\n", nb_mods, dpl, (int64_t)cost);
                if cost < min_cost {
                    min_cost = cost;
                    dpl_found = dpl;
                    nb_mods_found = nb_mods;
                    fft_len_log2_found = fft_len_log2
                }
                break;
            } else {
                dpl -= 1;
                if dpl == 0 as i32 {
                    break;
                }
            }
        }
        nb_mods += 1
    }
    if dpl_found == 0 {
        abort();
    }
    /* limit dpl if possible to reduce fixed cost of limb/NTT conversion */
    if dpl_found > ((1 as i32) << 6 as i32) + 61 as i32
        && ((((1 as i32) << 6 as i32) + 61 as i32) as limb_t) << fft_len_log2_found
            >= len.wrapping_mul(((1 as i32) << 6 as i32) as u64)
    {
        dpl_found = ((1 as i32) << 6 as i32) + 61 as i32
    }
    *pnb_mods = nb_mods_found;
    *pdpl = dpl_found;
    return fft_len_log2_found;
}
/* return 0 if OK, -1 if memory error */
#[inline(never)]
unsafe extern "C" fn fft_mul(
    mut s1: *mut bf_context_t,
    mut res: *mut bf_t,
    mut a_tab: *mut limb_t,
    mut a_len: limb_t,
    mut b_tab: *mut limb_t,
    mut b_len: limb_t,
    mut mul_flags: i32,
) -> i32 {
    let mut current_block: u64;
    let mut s: *mut BFNTTState = 0 as *mut BFNTTState;
    let mut dpl: i32 = 0;
    let mut fft_len_log2: i32 = 0;
    let mut j: i32 = 0;
    let mut nb_mods: i32 = 0;
    let mut reduced_mem: i32 = 0;
    let mut len: slimb_t = 0;
    let mut fft_len: slimb_t = 0;
    let mut buf1: *mut NTTLimb = 0 as *mut NTTLimb;
    let mut buf2: *mut NTTLimb = 0 as *mut NTTLimb;
    let mut ptr: *mut NTTLimb = 0 as *mut NTTLimb;
    if ntt_static_init(s1) != 0 {
        return -(1 as i32);
    }
    s = (*s1).ntt_state;
    /* find the optimal number of digits per limb (dpl) */
    len = a_len.wrapping_add(b_len) as slimb_t;
    fft_len_log2 = bf_get_fft_size(&mut dpl, &mut nb_mods, len as limb_t);
    fft_len = ((1 as i32 as uint64_t) << fft_len_log2) as slimb_t;
    //    printf("len=%" PRId64 " fft_len_log2=%d dpl=%d\n", len, fft_len_log2, dpl);
    if mul_flags & ((1 as i32) << 0 as i32 | (1 as i32) << 1 as i32) == 0 as i32 {
        if mul_flags & (1 as i32) << 2 as i32 == 0 {
            bf_resize(res, 0 as i32 as limb_t);
        }
    } else if mul_flags & (1 as i32) << 1 as i32 != 0 {
        let mut tmp_tab: *mut limb_t = 0 as *mut limb_t;
        let mut tmp_len: limb_t = 0;
        /* it is better to free 'b' first */
        tmp_tab = a_tab;
        a_tab = b_tab;
        b_tab = tmp_tab;
        tmp_len = a_len;
        a_len = b_len;
        b_len = tmp_len
    }
    buf1 = ntt_malloc(
        s,
        (::std::mem::size_of::<NTTLimb>() as u64)
            .wrapping_mul(fft_len as u64)
            .wrapping_mul(nb_mods as u64),
    ) as *mut NTTLimb;
    if buf1.is_null() {
        return -(1 as i32);
    }
    limb_to_ntt(
        s,
        buf1,
        fft_len as limb_t,
        a_tab,
        a_len,
        dpl,
        5 as i32 - nb_mods,
        nb_mods,
    );
    if mul_flags & ((1 as i32) << 0 as i32 | (1 as i32) << 1 as i32) == (1 as i32) << 0 as i32 {
        if mul_flags & (1 as i32) << 2 as i32 == 0 {
            bf_resize(res, 0 as i32 as limb_t);
        }
    }
    reduced_mem = (fft_len_log2 >= 14 as i32) as i32;
    if reduced_mem == 0 {
        buf2 = ntt_malloc(
            s,
            (::std::mem::size_of::<NTTLimb>() as u64)
                .wrapping_mul(fft_len as u64)
                .wrapping_mul(nb_mods as u64),
        ) as *mut NTTLimb;
        if buf2.is_null() {
            current_block = 11742859648667696368;
        } else {
            limb_to_ntt(
                s,
                buf2,
                fft_len as limb_t,
                b_tab,
                b_len,
                dpl,
                5 as i32 - nb_mods,
                nb_mods,
            );
            if mul_flags & (1 as i32) << 2 as i32 == 0 {
                bf_resize(res, 0 as i32 as limb_t);
            }
            current_block = 7245201122033322888;
        }
    /* in case res == b */
    } else {
        buf2 = ntt_malloc(
            s,
            (::std::mem::size_of::<NTTLimb>() as u64).wrapping_mul(fft_len as u64),
        ) as *mut NTTLimb; /* in case res == b and reduced mem */
        if buf2.is_null() {
            current_block = 11742859648667696368;
        } else {
            current_block = 7245201122033322888;
        }
    }
    match current_block {
        7245201122033322888 => {
            j = 0 as i32;
            loop {
                if !(j < nb_mods) {
                    current_block = 1356832168064818221;
                    break;
                }
                if reduced_mem != 0 {
                    limb_to_ntt(
                        s,
                        buf2,
                        fft_len as limb_t,
                        b_tab,
                        b_len,
                        dpl,
                        5 as i32 - nb_mods + j,
                        1 as i32,
                    );
                    ptr = buf2
                } else {
                    ptr = buf2.offset((fft_len * j as i64) as isize)
                }
                if ntt_conv(
                    s,
                    buf1.offset((fft_len * j as i64) as isize),
                    ptr,
                    fft_len_log2,
                    fft_len_log2,
                    (j + 5 as i32 - nb_mods) as limb_t,
                ) != 0
                {
                    current_block = 11742859648667696368;
                    break;
                }
                j += 1
            }
            match current_block {
                11742859648667696368 => {}
                _ => {
                    if mul_flags & (1 as i32) << 2 as i32 == 0 {
                        bf_resize(res, 0 as i32 as limb_t);
                    }
                    ntt_free(s, buf2 as *mut std::ffi::c_void);
                    buf2 = 0 as *mut NTTLimb;
                    if mul_flags & (1 as i32) << 2 as i32 == 0 {
                        if bf_resize(res, len as limb_t) != 0 {
                            current_block = 11742859648667696368;
                        } else {
                            current_block = 5891011138178424807;
                        }
                    } else {
                        current_block = 5891011138178424807;
                    }
                    match current_block {
                        11742859648667696368 => {}
                        _ => {
                            ntt_to_limb(
                                s,
                                (*res).tab,
                                len as limb_t,
                                buf1,
                                fft_len_log2,
                                dpl,
                                nb_mods,
                            );
                            ntt_free(s, buf1 as *mut std::ffi::c_void);
                            return 0 as i32;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    ntt_free(s, buf1 as *mut std::ffi::c_void);
    ntt_free(s, buf2 as *mut std::ffi::c_void);
    return -(1 as i32);
}
/* !USE_FFT_MUL */
/* USE_FFT_MUL */
