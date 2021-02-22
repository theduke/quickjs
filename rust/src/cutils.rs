use std::io::Write;
use std::os::raw::c_char;

extern "C" {
    #[no_mangle]
    fn memcmp(_: *const u8, _: *const u8, _: usize) -> i32;
}

pub type __builtin_va_list = [__va_list_tag; 1];

pub unsafe extern "C" fn cstr_snprintf(
    buf: *mut c_char,
    buf_size: usize,
    format_str: *const c_char,
    mut args: ...
) -> i32 {
    cstr_vsnprintf(buf, buf_size, format_str, args.as_va_list())
}

pub unsafe extern "C" fn cstr_vsnprintf(
    buf: *mut c_char,
    buf_size: usize,
    format_str: *const c_char,
    args: std::ffi::VaList,
) -> i32 {
    use printf_compat::{format, output};

    let mut slice: &mut [u8] = std::slice::from_raw_parts_mut(buf as *mut u8, buf_size);
    let mut c = std::io::Cursor::new(slice);
    let bytes_written = format(format_str, args, output::io_write(&mut c));
    bytes_written
}

pub trait PtrExt {
    // Backport removed std library method.
    // Source: https://github.com/RalfJung/rust/blob/467415d50cdf8a0d15ec19dc63251443b35d4cee/src/libcore/ptr/const_ptr.rs#L340
    // This is backported because c2rust uses thid method quite a bit.
    // TODO: remove all usage
    fn wrapping_offset_from(self, origin: Self) -> isize;
}

impl<T> PtrExt for *const T
where
    T: Sized,
{
    fn wrapping_offset_from(self, origin: Self) -> isize {
        let pointee_size = std::mem::size_of::<T>();
        assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);

        let d = isize::wrapping_sub(self as _, origin as _);
        d.wrapping_div(pointee_size as _)
    }
}

impl<T> PtrExt for *mut T
where
    T: Sized,
{
    fn wrapping_offset_from(self, origin: Self) -> isize {
        let pointee_size = std::mem::size_of::<T>();
        assert!(0 < pointee_size && pointee_size <= isize::MAX as usize);

        let d = isize::wrapping_sub(self as _, origin as _);
        d.wrapping_div(pointee_size as _)
    }
}

pub unsafe fn cstr_len(mut s: *const c_char) -> usize {
    let mut tail = s as *const u8;
    while *tail != b'\0' {
        tail = tail.add(1);
    }
    (tail as usize) - (s as usize)
}

pub unsafe fn cstr_append_sized(
    target: *mut c_char,
    target_len: usize,
    value: &str,
) -> *mut c_char {
    if target_len > 0 {
        let bytes = if target_len >= value.len() + 1 {
            value.as_bytes()
        } else {
            &value.as_bytes()[0..target_len - 1]
        };

        target.copy_from(bytes.as_ptr() as *const c_char, bytes.len());
        *target.add(bytes.len()) = b'\0' as i8;
        target
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn cstr_parse_f64(s: *const c_char) -> f64 {
    std::ffi::CStr::from_ptr(s)
        .to_str()
        .ok()
        .and_then(|x| x.parse::<f64>().ok())
        .unwrap_or(0.0)
}

pub unsafe fn cstr_parse_i64(s: *const c_char) -> i64 {
    std::ffi::CStr::from_ptr(s)
        .to_str()
        .ok()
        .and_then(|x| x.parse::<i64>().ok())
        .unwrap_or(0)
}

pub unsafe fn cstr_copy(mut dest: *mut c_char, src: *const c_char) -> *const c_char {
    dest.copy_from(src, cstr_len(src) + 1);
    dest
}

// FIXME: this is a naive implementaton and much slower than a libc impl.
pub unsafe fn cstr_find_char(mut s: *const c_char, c: c_char) -> *const c_char {
    loop {
        let cur = *s;
        if cur == c || cur == b'\0' as i8 {
            return s;
        }
        s = s.add(1);
    }
}

// FIXME: this is a naive implementaton and much slower than a libc impl.
pub unsafe fn cstr_find_char_right(mut s: *const u8, c: u8) -> *const u8 {
    let mut matched = std::ptr::null();
    while *s != b'\0' {
        if *s == c {
            matched = s;
            s = s.add(1);
        }
    }
    matched
}

pub unsafe fn cstr_append(mut target: *mut u8, mut value: *const u8) -> *mut u8 {
    let start = target;
    while *target != b'\0' {
        target = target.add(1);
    }
    while *value != b'\0' {
        *target = *value;
        target = target.add(1);
        value = value.add(1);
    }

    // Write final \0 byte.
    *target = b'\0';

    start
}

pub unsafe fn ptr_search(source: *const u8, value: u8, len: usize) -> *const u8 {
    let mut head = source;
    let end = source.add(len);
    while head != end {
        if *head == value {
            return head;
        }
        head = head.add(1);
    }
    std::ptr::null()
}

#[inline]
pub unsafe fn global_alloc(size: usize) -> *mut u8 {
    let layout =
        std::alloc::Layout::from_size_align(size, std::mem::align_of::<u8>()).expect("Bad layout");
    std::alloc::alloc(layout)
}

#[inline]
pub unsafe fn global_realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(new_size, std::mem::align_of::<u8>())
        .expect("Bad layout");
    std::alloc::realloc(ptr, layout, new_size)
}

#[inline]
pub unsafe fn ptr_compare(a: *const u8, b: *const u8, len: usize) -> i32 {
    memcmp(a, b, len)
}

// TODO: remove!
pub unsafe fn cstr_compare(a: *const i8, b: *const i8) -> i32 {
    let mut a = a as *const u8;
    let mut b = b as *const u8;

    let mut c1: u8;
    let mut c2: u8;

    loop {
        c1 = *a;
        a = a.add(1);
        c2 = *b;
        b = b.add(1);
        if c1 == b'\0' {
            return (c1 - c2) as i32;
        } else if c1 == c2 {
            break;
        }
    }

    (c1 - c2) as i32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut std::ffi::c_void,
    pub reg_save_area: *mut std::ffi::c_void,
}
pub type size_t = u64;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type va_list = __builtin_va_list;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = u64;
pub type BOOL = i32;
pub type C2RustUnnamed = u32;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
pub type DynBufReallocFunc = unsafe extern "C" fn(
    _: *mut std::ffi::c_void,
    _: *mut std::ffi::c_void,
    _: size_t,
) -> *mut std::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DynBuf {
    pub buf: *mut uint8_t,
    pub size: size_t,
    pub allocated_size: size_t,
    pub error: BOOL,
    pub realloc_func: Option<DynBufReallocFunc>,
    pub opaque: *mut std::ffi::c_void,
}
pub type cmp_f = Option<
    unsafe extern "C" fn(
        _: *const std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: *mut std::ffi::c_void,
    ) -> i32,
>;
pub type exchange_f = Option<
    unsafe extern "C" fn(_: *mut std::ffi::c_void, _: *mut std::ffi::c_void, _: size_t) -> (),
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub base: *mut uint8_t,
    pub count: size_t,
    pub depth: i32,
}
/*
 * C utilities
 *
 * Copyright (c) 2017 Fabrice Bellard
 * Copyright (c) 2018 Charlie Gordon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */
#[no_mangle]
pub unsafe extern "C" fn pstrcpy(
    mut buf: *mut std::os::raw::c_char,
    mut buf_size: i32,
    mut str: *const std::os::raw::c_char,
) {
    let mut c: i32 = 0;
    let mut q: *mut std::os::raw::c_char = buf;
    if buf_size <= 0 as i32 {
        return;
    }
    loop {
        let fresh0 = str;
        str = str.offset(1);
        c = *fresh0 as i32;
        if c == 0 as i32 || q >= buf.offset(buf_size as isize).offset(-(1 as i32 as isize)) {
            break;
        }
        let fresh1 = q;
        q = q.offset(1);
        *fresh1 = c as std::os::raw::c_char
    }
    *q = '\u{0}' as i32 as std::os::raw::c_char;
}
/* strcat and truncate. */
#[no_mangle]
pub unsafe extern "C" fn pstrcat(
    mut buf: *mut std::os::raw::c_char,
    mut buf_size: i32,
    mut s: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut len: i32 = 0;
    len = cstr_len(buf) as i32;
    if len < buf_size {
        pstrcpy(buf.offset(len as isize), buf_size - len, s);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn strstart(
    mut str: *const std::os::raw::c_char,
    mut val: *const std::os::raw::c_char,
    mut ptr: *mut *const std::os::raw::c_char,
) -> i32 {
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut q: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    p = str;
    q = val;
    while *q as i32 != '\u{0}' as i32 {
        if *p as i32 != *q as i32 {
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
#[no_mangle]
pub unsafe extern "C" fn has_suffix(
    mut str: *const std::os::raw::c_char,
    mut suffix: *const std::os::raw::c_char,
) -> i32 {
    let mut len = cstr_len(str);
    let mut slen = cstr_len(suffix);
    return (len >= slen
        && ptr_compare(
            str.offset(len as isize).offset(-(slen as isize)) as *const u8,
            suffix as *const u8,
            slen as usize,
        ) == 0) as i32;
}

#[inline]
pub unsafe extern "C" fn dbuf_error(mut s: *mut DynBuf) -> BOOL {
    return (*s).error;
}

#[inline]
pub unsafe extern "C" fn dbuf_set_error(mut s: *mut DynBuf) {
    (*s).error = TRUE as i32;
}

/* Dynamic buffer package */
unsafe extern "C" fn dbuf_default_realloc(
    mut opaque: *mut std::ffi::c_void,
    mut ptr: *mut std::ffi::c_void,
    mut size: size_t,
) -> *mut std::ffi::c_void {
    global_realloc(ptr as *mut u8, size as usize) as *mut std::ffi::c_void
}

#[no_mangle]
pub unsafe extern "C" fn dbuf_init2(
    mut s: *mut DynBuf,
    mut opaque: *mut std::ffi::c_void,
    mut realloc_func: Option<DynBufReallocFunc>,
) {
    (s as *mut u8).write_bytes(0, std::mem::size_of::<DynBuf>());

    if realloc_func.is_none() {
        realloc_func = Some(
            dbuf_default_realloc
                as unsafe extern "C" fn(
                    _: *mut std::ffi::c_void,
                    _: *mut std::ffi::c_void,
                    _: size_t,
                ) -> *mut std::ffi::c_void,
        )
    }
    (*s).opaque = opaque;
    (*s).realloc_func = realloc_func;
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_init(mut s: *mut DynBuf) {
    dbuf_init2(s, 0 as *mut std::ffi::c_void, None);
}
/* return < 0 if error */
#[no_mangle]
pub unsafe extern "C" fn dbuf_realloc(mut s: *mut DynBuf, mut new_size: size_t) -> i32 {
    let mut size: size_t = 0;
    let mut new_buf: *mut uint8_t = 0 as *mut uint8_t;
    if new_size > (*s).allocated_size {
        if (*s).error != 0 {
            return -(1 as i32);
        }
        size = (*s).allocated_size.wrapping_mul(3).wrapping_div(2);
        if size > new_size {
            new_size = size
        }
        new_buf = (*s).realloc_func.expect("non-null function pointer")(
            (*s).opaque,
            (*s).buf as *mut std::ffi::c_void,
            new_size,
        ) as *mut uint8_t;
        if new_buf.is_null() {
            (*s).error = TRUE as i32;
            return -(1 as i32);
        }
        (*s).buf = new_buf;
        (*s).allocated_size = new_size
    }
    return 0 as i32;
}

#[inline]
pub unsafe extern "C" fn dbuf_put_u16(mut s: *mut DynBuf, mut val: uint16_t) -> i32 {
    return dbuf_put(s, &mut val as *mut uint16_t as *mut uint8_t, 2);
}

#[inline]
pub unsafe extern "C" fn dbuf_put_u32(mut s: *mut DynBuf, mut val: uint32_t) -> i32 {
    return dbuf_put(s, &mut val as *mut u32 as *mut uint8_t, 4);
}

#[inline]
pub unsafe extern "C" fn dbuf_put_u64(mut s: *mut DynBuf, mut val: uint64_t) -> i32 {
    return dbuf_put(s, &mut val as *mut uint64_t as *mut uint8_t, 8);
}

#[no_mangle]
pub unsafe fn dbuf_write(
    mut s: *mut DynBuf,
    mut offset: size_t,
    mut data: *const uint8_t,
    mut len: size_t,
) -> i32 {
    let mut end: size_t = 0;
    end = offset.wrapping_add(len);

    if dbuf_realloc(s, end) != 0 {
        return -(1 as i32);
    }
    (*s).buf
        .offset(offset as isize)
        .copy_from(data, len as usize);

    if end > (*s).size {
        (*s).size = end
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn dbuf_put(
    mut s: *mut DynBuf,
    mut data: *const uint8_t,
    mut len: usize,
) -> i32 {
    if ((*s).size.wrapping_add(len as u64) > (*s).allocated_size) {
        if dbuf_realloc(s, (*s).size.wrapping_add(len as u64)) != 0 {
            return -1;
        }
    }
    (*s).buf
        .offset((*s).size as isize)
        .copy_from(data, len as usize);
    (*s).size = ((*s).size as u64).wrapping_add(len as u64) as size_t;
    0
}

#[no_mangle]
pub unsafe extern "C" fn dbuf_put_self(
    mut s: *mut DynBuf,
    mut offset: size_t,
    mut len: size_t,
) -> i32 {
    if ((*s).size.wrapping_add(len) > (*s).allocated_size) as i32 as i64 != 0 {
        if dbuf_realloc(s, (*s).size.wrapping_add(len)) != 0 {
            return -1;
        }
    }
    (*s).buf
        .offset((*s).size as isize)
        .copy_from((*s).buf.offset(offset as isize), len as usize);

    (*s).size = ((*s).size as u64).wrapping_add(len) as size_t as size_t;
    0
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_putc(mut s: *mut DynBuf, mut c: uint8_t) -> i32 {
    return dbuf_put(s, &mut c, 1);
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_putstr(
    mut s: *mut DynBuf,
    mut str: *const std::os::raw::c_char,
) -> i32 {
    return dbuf_put(s, str as *const uint8_t, cstr_len(str));
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_printf(
    mut s: *mut DynBuf,
    mut fmt: *const std::os::raw::c_char,
    mut args: ...
) -> i32 {
    let mut ap: ::std::ffi::VaListImpl;
    let mut buf: [std::os::raw::c_char; 128] = [0; 128];
    let mut len = 0;
    ap = args.clone();
    len = cstr_vsnprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 128]>(),
        fmt,
        ap.as_va_list(),
    );
    if (len as u64) < ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as u64 {
        /* fast case */
        return dbuf_put(s, buf.as_mut_ptr() as *mut uint8_t, len as usize);
    } else {
        if dbuf_realloc(
            s,
            (*s).size
                .wrapping_add(len as u64)
                .wrapping_add(1 as i32 as u64),
        ) != 0
        {
            return -(1 as i32);
        }
        ap = args.clone();
        cstr_vsnprintf(
            (*s).buf.offset((*s).size as isize) as *mut std::os::raw::c_char,
            (*s).allocated_size.wrapping_sub((*s).size) as usize,
            fmt,
            ap.as_va_list(),
        );
        (*s).size = ((*s).size as u64).wrapping_add(len as u64) as size_t as size_t
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn dbuf_free(mut s: *mut DynBuf) {
    /* we test s->buf as a fail safe to avoid crashing if dbuf_free()
    is called twice */
    if !(*s).buf.is_null() {
        (*s).realloc_func.expect("non-null function pointer")(
            (*s).opaque,
            (*s).buf as *mut std::ffi::c_void,
            0 as i32 as size_t,
        );
    }
    (s as *mut u8).write_bytes(0, std::mem::size_of::<DynBuf>());
}

unsafe extern "C" fn exchange_bytes(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint8_t = a as *mut uint8_t;
    let mut bp: *mut uint8_t = b as *mut uint8_t;
    loop {
        let fresh15 = size;
        size = size.wrapping_sub(1);
        if !(fresh15 != 0 as i32 as u64) {
            break;
        }
        let mut t: uint8_t = *ap;
        let fresh16 = ap;
        ap = ap.offset(1);
        *fresh16 = *bp;
        let fresh17 = bp;
        bp = bp.offset(1);
        *fresh17 = t
    }
}
unsafe extern "C" fn exchange_one_byte(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint8_t = a as *mut uint8_t;
    let mut bp: *mut uint8_t = b as *mut uint8_t;
    let mut t: uint8_t = *ap;
    *ap = *bp;
    *bp = t;
}
unsafe extern "C" fn exchange_int16s(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint16_t = a as *mut uint16_t;
    let mut bp: *mut uint16_t = b as *mut uint16_t;
    size = (size as u64).wrapping_div(::std::mem::size_of::<uint16_t>() as u64) as size_t as size_t;
    loop {
        let fresh18 = size;
        size = size.wrapping_sub(1);
        if !(fresh18 != 0 as i32 as u64) {
            break;
        }
        let mut t: uint16_t = *ap;
        let fresh19 = ap;
        ap = ap.offset(1);
        *fresh19 = *bp;
        let fresh20 = bp;
        bp = bp.offset(1);
        *fresh20 = t
    }
}
unsafe extern "C" fn exchange_one_int16(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint16_t = a as *mut uint16_t;
    let mut bp: *mut uint16_t = b as *mut uint16_t;
    let mut t: uint16_t = *ap;
    *ap = *bp;
    *bp = t;
}
unsafe extern "C" fn exchange_int32s(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint32_t = a as *mut uint32_t;
    let mut bp: *mut uint32_t = b as *mut uint32_t;
    size = (size as u64).wrapping_div(::std::mem::size_of::<uint32_t>() as u64) as size_t as size_t;
    loop {
        let fresh21 = size;
        size = size.wrapping_sub(1);
        if !(fresh21 != 0 as i32 as u64) {
            break;
        }
        let mut t: uint32_t = *ap;
        let fresh22 = ap;
        ap = ap.offset(1);
        *fresh22 = *bp;
        let fresh23 = bp;
        bp = bp.offset(1);
        *fresh23 = t
    }
}
unsafe extern "C" fn exchange_one_int32(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint32_t = a as *mut uint32_t;
    let mut bp: *mut uint32_t = b as *mut uint32_t;
    let mut t: uint32_t = *ap;
    *ap = *bp;
    *bp = t;
}
unsafe extern "C" fn exchange_int64s(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint64_t = a as *mut uint64_t;
    let mut bp: *mut uint64_t = b as *mut uint64_t;
    size = (size as u64).wrapping_div(::std::mem::size_of::<uint64_t>() as u64) as size_t as size_t;
    loop {
        let fresh24 = size;
        size = size.wrapping_sub(1);
        if !(fresh24 != 0 as i32 as u64) {
            break;
        }
        let mut t: uint64_t = *ap;
        let fresh25 = ap;
        ap = ap.offset(1);
        *fresh25 = *bp;
        let fresh26 = bp;
        bp = bp.offset(1);
        *fresh26 = t
    }
}
unsafe extern "C" fn exchange_one_int64(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint64_t = a as *mut uint64_t;
    let mut bp: *mut uint64_t = b as *mut uint64_t;
    let mut t: uint64_t = *ap;
    *ap = *bp;
    *bp = t;
}
unsafe extern "C" fn exchange_int128s(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint64_t = a as *mut uint64_t;
    let mut bp: *mut uint64_t = b as *mut uint64_t;
    size = (size as u64)
        .wrapping_div((::std::mem::size_of::<uint64_t>() as u64).wrapping_mul(2 as i32 as u64))
        as size_t as size_t;
    loop {
        let fresh27 = size;
        size = size.wrapping_sub(1);
        if !(fresh27 != 0 as i32 as u64) {
            break;
        }
        let mut t: uint64_t = *ap.offset(0 as i32 as isize);
        let mut u: uint64_t = *ap.offset(1 as i32 as isize);
        *ap.offset(0 as i32 as isize) = *bp.offset(0 as i32 as isize);
        *ap.offset(1 as i32 as isize) = *bp.offset(1 as i32 as isize);
        *bp.offset(0 as i32 as isize) = t;
        *bp.offset(1 as i32 as isize) = u;
        ap = ap.offset(2 as i32 as isize);
        bp = bp.offset(2 as i32 as isize)
    }
}
unsafe extern "C" fn exchange_one_int128(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut size: size_t,
) {
    let mut ap: *mut uint64_t = a as *mut uint64_t;
    let mut bp: *mut uint64_t = b as *mut uint64_t;
    let mut t: uint64_t = *ap.offset(0 as i32 as isize);
    let mut u: uint64_t = *ap.offset(1 as i32 as isize);
    *ap.offset(0 as i32 as isize) = *bp.offset(0 as i32 as isize);
    *ap.offset(1 as i32 as isize) = *bp.offset(1 as i32 as isize);
    *bp.offset(0 as i32 as isize) = t;
    *bp.offset(1 as i32 as isize) = u;
}
#[inline]
unsafe extern "C" fn exchange_func(
    mut base: *const std::ffi::c_void,
    mut size: size_t,
) -> exchange_f {
    match (base as uintptr_t | size) & 15 as i32 as u64 {
        0 => {
            if size == (::std::mem::size_of::<uint64_t>() as u64).wrapping_mul(2 as i32 as u64) {
                return Some(
                    exchange_one_int128
                        as unsafe extern "C" fn(
                            _: *mut std::ffi::c_void,
                            _: *mut std::ffi::c_void,
                            _: size_t,
                        ) -> (),
                );
            } else {
                return Some(
                    exchange_int128s
                        as unsafe extern "C" fn(
                            _: *mut std::ffi::c_void,
                            _: *mut std::ffi::c_void,
                            _: size_t,
                        ) -> (),
                );
            }
        }
        8 => {
            if size == ::std::mem::size_of::<uint64_t>() as u64 {
                return Some(
                    exchange_one_int64
                        as unsafe extern "C" fn(
                            _: *mut std::ffi::c_void,
                            _: *mut std::ffi::c_void,
                            _: size_t,
                        ) -> (),
                );
            } else {
                return Some(
                    exchange_int64s
                        as unsafe extern "C" fn(
                            _: *mut std::ffi::c_void,
                            _: *mut std::ffi::c_void,
                            _: size_t,
                        ) -> (),
                );
            }
        }
        4 | 12 => {
            if size == ::std::mem::size_of::<uint32_t>() as u64 {
                return Some(
                    exchange_one_int32
                        as unsafe extern "C" fn(
                            _: *mut std::ffi::c_void,
                            _: *mut std::ffi::c_void,
                            _: size_t,
                        ) -> (),
                );
            } else {
                return Some(
                    exchange_int32s
                        as unsafe extern "C" fn(
                            _: *mut std::ffi::c_void,
                            _: *mut std::ffi::c_void,
                            _: size_t,
                        ) -> (),
                );
            }
        }
        2 | 6 | 10 | 14 => {
            if size == ::std::mem::size_of::<uint16_t>() as u64 {
                return Some(
                    exchange_one_int16
                        as unsafe extern "C" fn(
                            _: *mut std::ffi::c_void,
                            _: *mut std::ffi::c_void,
                            _: size_t,
                        ) -> (),
                );
            } else {
                return Some(
                    exchange_int16s
                        as unsafe extern "C" fn(
                            _: *mut std::ffi::c_void,
                            _: *mut std::ffi::c_void,
                            _: size_t,
                        ) -> (),
                );
            }
        }
        _ => {
            if size == 1 as i32 as u64 {
                return Some(
                    exchange_one_byte
                        as unsafe extern "C" fn(
                            _: *mut std::ffi::c_void,
                            _: *mut std::ffi::c_void,
                            _: size_t,
                        ) -> (),
                );
            } else {
                return Some(
                    exchange_bytes
                        as unsafe extern "C" fn(
                            _: *mut std::ffi::c_void,
                            _: *mut std::ffi::c_void,
                            _: size_t,
                        ) -> (),
                );
            }
        }
    };
}
unsafe extern "C" fn heapsortx(
    mut base: *mut std::ffi::c_void,
    mut nmemb: size_t,
    mut size: size_t,
    mut cmp: cmp_f,
    mut opaque: *mut std::ffi::c_void,
) {
    let mut basep: *mut uint8_t = base as *mut uint8_t;
    let mut i: size_t = 0;
    let mut n: size_t = 0;
    let mut c: size_t = 0;
    let mut r: size_t = 0;
    let mut swap: exchange_f = exchange_func(base, size);
    if nmemb > 1 as i32 as u64 {
        i = nmemb.wrapping_div(2 as i32 as u64).wrapping_mul(size);
        n = nmemb.wrapping_mul(size);
        while i > 0 as i32 as u64 {
            i = (i as u64).wrapping_sub(size) as size_t as size_t;
            r = i;
            loop {
                c = r.wrapping_mul(2 as i32 as u64).wrapping_add(size);
                if !(c < n) {
                    break;
                }
                if c < n.wrapping_sub(size)
                    && cmp.expect("non-null function pointer")(
                        basep.offset(c as isize) as *const std::ffi::c_void,
                        basep.offset(c as isize).offset(size as isize) as *const std::ffi::c_void,
                        opaque,
                    ) <= 0 as i32
                {
                    c = (c as u64).wrapping_add(size) as size_t as size_t
                }
                if cmp.expect("non-null function pointer")(
                    basep.offset(r as isize) as *const std::ffi::c_void,
                    basep.offset(c as isize) as *const std::ffi::c_void,
                    opaque,
                ) > 0 as i32
                {
                    break;
                }
                swap.expect("non-null function pointer")(
                    basep.offset(r as isize) as *mut std::ffi::c_void,
                    basep.offset(c as isize) as *mut std::ffi::c_void,
                    size,
                );
                r = c
            }
        }
        i = n.wrapping_sub(size);
        while i > 0 as i32 as u64 {
            swap.expect("non-null function pointer")(
                basep as *mut std::ffi::c_void,
                basep.offset(i as isize) as *mut std::ffi::c_void,
                size,
            );
            r = 0 as i32 as size_t;
            loop {
                c = r.wrapping_mul(2 as i32 as u64).wrapping_add(size);
                if !(c < i) {
                    break;
                }
                if c < i.wrapping_sub(size)
                    && cmp.expect("non-null function pointer")(
                        basep.offset(c as isize) as *const std::ffi::c_void,
                        basep.offset(c as isize).offset(size as isize) as *const std::ffi::c_void,
                        opaque,
                    ) <= 0 as i32
                {
                    c = (c as u64).wrapping_add(size) as size_t as size_t
                }
                if cmp.expect("non-null function pointer")(
                    basep.offset(r as isize) as *const std::ffi::c_void,
                    basep.offset(c as isize) as *const std::ffi::c_void,
                    opaque,
                ) > 0 as i32
                {
                    break;
                }
                swap.expect("non-null function pointer")(
                    basep.offset(r as isize) as *mut std::ffi::c_void,
                    basep.offset(c as isize) as *mut std::ffi::c_void,
                    size,
                );
                r = c
            }
            i = (i as u64).wrapping_sub(size) as size_t as size_t
        }
    };
}
#[inline]
unsafe extern "C" fn med3(
    mut a: *mut std::ffi::c_void,
    mut b: *mut std::ffi::c_void,
    mut c: *mut std::ffi::c_void,
    mut cmp: cmp_f,
    mut opaque: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    return if cmp.expect("non-null function pointer")(a, b, opaque) < 0 as i32 {
        if cmp.expect("non-null function pointer")(b, c, opaque) < 0 as i32 {
            b
        } else if cmp.expect("non-null function pointer")(a, c, opaque) < 0 as i32 {
            c
        } else {
            a
        }
    } else if cmp.expect("non-null function pointer")(b, c, opaque) > 0 as i32 {
        b
    } else if cmp.expect("non-null function pointer")(a, c, opaque) < 0 as i32 {
        a
    } else {
        c
    };
}
/* pointer based version with local stack and insertion sort threshhold */
#[no_mangle]
pub unsafe extern "C" fn rqsort(
    mut base: *mut std::ffi::c_void,
    mut nmemb: size_t,
    mut size: size_t,
    mut cmp: cmp_f,
    mut opaque: *mut std::ffi::c_void,
) {
    let mut stack: [C2RustUnnamed_0; 50] = [C2RustUnnamed_0 {
        base: 0 as *mut uint8_t,
        count: 0,
        depth: 0,
    }; 50];
    let mut sp: *mut C2RustUnnamed_0 = stack.as_mut_ptr();
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    let mut pi: *mut uint8_t = 0 as *mut uint8_t;
    let mut pj: *mut uint8_t = 0 as *mut uint8_t;
    let mut plt: *mut uint8_t = 0 as *mut uint8_t;
    let mut pgt: *mut uint8_t = 0 as *mut uint8_t;
    let mut top: *mut uint8_t = 0 as *mut uint8_t;
    let mut m: *mut uint8_t = 0 as *mut uint8_t;
    let mut m4: size_t = 0;
    let mut i: size_t = 0;
    let mut lt: size_t = 0;
    let mut gt: size_t = 0;
    let mut span: size_t = 0;
    let mut span2: size_t = 0;
    let mut c: i32 = 0;
    let mut depth: i32 = 0;
    let mut swap: exchange_f = exchange_func(base, size);
    let mut swap_block: exchange_f = exchange_func(base, size | 128 as i32 as u64);
    if nmemb < 2 as i32 as u64 || size <= 0 as i32 as u64 {
        return;
    }
    (*sp).base = base as *mut uint8_t;
    (*sp).count = nmemb;
    (*sp).depth = 0 as i32;
    sp = sp.offset(1);
    while sp > stack.as_mut_ptr() {
        sp = sp.offset(-1);
        ptr = (*sp).base;
        nmemb = (*sp).count;
        depth = (*sp).depth;
        while nmemb > 6 as i32 as u64 {
            depth += 1;
            if depth > 50 as i32 {
                /* depth check to ensure worst case logarithmic time */
                heapsortx(ptr as *mut std::ffi::c_void, nmemb, size, cmp, opaque);
                nmemb = 0 as i32 as size_t;
                break;
            } else {
                /* select median of 3 from 1/4, 1/2, 3/4 positions */
                /* should use median of 5 or 9? */
                m4 = (nmemb >> 2 as i32).wrapping_mul(size); /* move the pivot to the start or the array */
                m = med3(
                    ptr.offset(m4 as isize) as *mut std::ffi::c_void,
                    ptr.offset((2 as i32 as u64).wrapping_mul(m4) as isize)
                        as *mut std::ffi::c_void,
                    ptr.offset((3 as i32 as u64).wrapping_mul(m4) as isize)
                        as *mut std::ffi::c_void,
                    cmp,
                    opaque,
                ) as *mut uint8_t;
                swap.expect("non-null function pointer")(
                    ptr as *mut std::ffi::c_void,
                    m as *mut std::ffi::c_void,
                    size,
                );
                lt = 1 as i32 as size_t;
                i = lt;
                plt = ptr.offset(size as isize);
                pi = plt;
                gt = nmemb;
                top = ptr.offset(nmemb.wrapping_mul(size) as isize);
                pgt = top;
                pj = pgt;
                loop {
                    while pi < pj && {
                        c = cmp.expect("non-null function pointer")(
                            ptr as *const std::ffi::c_void,
                            pi as *const std::ffi::c_void,
                            opaque,
                        );
                        (c) >= 0 as i32
                    } {
                        if c == 0 as i32 {
                            swap.expect("non-null function pointer")(
                                plt as *mut std::ffi::c_void,
                                pi as *mut std::ffi::c_void,
                                size,
                            );
                            lt = lt.wrapping_add(1);
                            plt = plt.offset(size as isize)
                        }
                        i = i.wrapping_add(1);
                        pi = pi.offset(size as isize)
                    }
                    loop {
                        pj = pj.offset(-(size as isize));
                        if !(pi < pj && {
                            c = cmp.expect("non-null function pointer")(
                                ptr as *const std::ffi::c_void,
                                pj as *const std::ffi::c_void,
                                opaque,
                            );
                            (c) <= 0 as i32
                        }) {
                            break;
                        }
                        if c == 0 as i32 {
                            gt = gt.wrapping_sub(1);
                            pgt = pgt.offset(-(size as isize));
                            swap.expect("non-null function pointer")(
                                pgt as *mut std::ffi::c_void,
                                pj as *mut std::ffi::c_void,
                                size,
                            );
                        }
                    }
                    if pi >= pj {
                        break;
                    }
                    swap.expect("non-null function pointer")(
                        pi as *mut std::ffi::c_void,
                        pj as *mut std::ffi::c_void,
                        size,
                    );
                    i = i.wrapping_add(1);
                    pi = pi.offset(size as isize)
                }
                /* array has 4 parts:
                 * from 0 to lt excluded: elements identical to pivot
                 * from lt to pi excluded: elements smaller than pivot
                 * from pi to gt excluded: elements greater than pivot
                 * from gt to n excluded: elements identical to pivot
                 */
                /* move elements identical to pivot in the middle of the array: */
                /* swap values in ranges [0..lt[ and [i-lt..i[
                  swapping the smallest span between lt and i-lt is sufficient
                */
                span = plt.wrapping_offset_from(ptr) as i64 as size_t;
                span2 = pi.wrapping_offset_from(plt) as i64 as size_t;
                lt = i.wrapping_sub(lt);
                if span > span2 {
                    span = span2
                }
                swap_block.expect("non-null function pointer")(
                    ptr as *mut std::ffi::c_void,
                    pi.offset(-(span as isize)) as *mut std::ffi::c_void,
                    span,
                );
                /* swap values in ranges [gt..top[ and [i..top-(top-gt)[
                  swapping the smallest span between top-gt and gt-i is sufficient
                */
                span = top.wrapping_offset_from(pgt) as i64 as size_t;
                span2 = pgt.wrapping_offset_from(pi) as i64 as size_t;
                pgt = top.offset(-(span2 as isize));
                gt = nmemb.wrapping_sub(gt.wrapping_sub(i));
                if span > span2 {
                    span = span2
                }
                swap_block.expect("non-null function pointer")(
                    pi as *mut std::ffi::c_void,
                    top.offset(-(span as isize)) as *mut std::ffi::c_void,
                    span,
                );
                /* now array has 3 parts:
                 * from 0 to lt excluded: elements smaller than pivot
                 * from lt to gt excluded: elements identical to pivot
                 * from gt to n excluded: elements greater than pivot
                 */
                /* stack the larger segment and keep processing the smaller one
                to minimize stack use for pathological distributions */
                if lt > nmemb.wrapping_sub(gt) {
                    (*sp).base = ptr;
                    (*sp).count = lt;
                    (*sp).depth = depth;
                    sp = sp.offset(1);
                    ptr = pgt;
                    nmemb = (nmemb as u64).wrapping_sub(gt) as size_t as size_t
                } else {
                    (*sp).base = pgt;
                    (*sp).count = nmemb.wrapping_sub(gt);
                    (*sp).depth = depth;
                    sp = sp.offset(1);
                    nmemb = lt
                }
            }
        }
        /* Use insertion sort for small fragments */
        pi = ptr.offset(size as isize);
        top = ptr.offset(nmemb.wrapping_mul(size) as isize);
        while pi < top {
            pj = pi;
            while pj > ptr
                && cmp.expect("non-null function pointer")(
                    pj.offset(-(size as isize)) as *const std::ffi::c_void,
                    pj as *const std::ffi::c_void,
                    opaque,
                ) > 0 as i32
            {
                swap.expect("non-null function pointer")(
                    pj as *mut std::ffi::c_void,
                    pj.offset(-(size as isize)) as *mut std::ffi::c_void,
                    size,
                );
                pj = pj.offset(-(size as isize))
            }
            pi = pi.offset(size as isize)
        }
    }
}
