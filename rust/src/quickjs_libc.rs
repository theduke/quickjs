use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;

use crate::cutils::PtrExt;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type JSRuntime;
    pub type JSContext;
    pub type JSGCObjectHeader;
    pub type JSModuleDef;
    #[no_mangle]
    fn strtol(_: *const std::os::raw::c_char, _: *mut *mut std::os::raw::c_char, _: i32) -> i64;
    #[no_mangle]
    fn malloc(_: u64) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut std::ffi::c_void);
    #[no_mangle]
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    #[no_mangle]
    fn exit(_: i32) -> !;
    #[no_mangle]
    fn getenv(__name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn select(
        __nfds: i32,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> i32;
    #[no_mangle]
    fn setenv(
        __name: *const std::os::raw::c_char,
        __value: *const std::os::raw::c_char,
        __replace: i32,
    ) -> i32;
    #[no_mangle]
    fn unsetenv(__name: *const std::os::raw::c_char) -> i32;
    #[no_mangle]
    fn realpath(
        __name: *const std::os::raw::c_char,
        __resolved: *mut std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn remove(__filename: *const std::os::raw::c_char) -> i32;
    #[no_mangle]
    fn rename(__old: *const std::os::raw::c_char, __new: *const std::os::raw::c_char) -> i32;
    #[no_mangle]
    fn tmpfile() -> *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fdopen(__fd: i32, __modes: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> i32;
    #[no_mangle]
    fn snprintf(
        _: *mut std::os::raw::c_char,
        _: u64,
        _: *const std::os::raw::c_char,
        _: ...
    ) -> i32;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    #[no_mangle]
    fn putc(__c: i32, __stream: *mut FILE) -> i32;
    #[no_mangle]
    fn fread(_: *mut std::ffi::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    #[no_mangle]
    fn fwrite(_: *const std::ffi::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> i64;
    #[no_mangle]
    fn fseeko(__stream: *mut FILE, __off: __off_t, __whence: i32) -> i32;
    #[no_mangle]
    fn ftello(__stream: *mut FILE) -> __off_t;
    #[no_mangle]
    fn clearerr(__stream: *mut FILE);
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn popen(
        __command: *const std::os::raw::c_char,
        __modes: *const std::os::raw::c_char,
    ) -> *mut FILE;
    #[no_mangle]
    fn pclose(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: u64,
    ) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn memset(_: *mut std::ffi::c_void, _: i32, _: u64) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn strcpy(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> i32;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: i32) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> u64;
    #[no_mangle]
    fn strerror(_: i32) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strspn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> u64;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: u32,
        __function: *const std::os::raw::c_char,
    ) -> !;
    #[no_mangle]
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    #[no_mangle]
    fn close(__fd: i32) -> i32;
    #[no_mangle]
    fn read(__fd: i32, __buf: *mut std::ffi::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn write(__fd: i32, __buf: *const std::ffi::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: *mut i32) -> i32;
    #[no_mangle]
    fn chdir(__path: *const std::os::raw::c_char) -> i32;
    #[no_mangle]
    fn getcwd(__buf: *mut std::os::raw::c_char, __size: size_t) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn dup(__fd: i32) -> i32;
    #[no_mangle]
    fn dup2(__fd: i32, __fd2: i32) -> i32;
    #[no_mangle]
    static mut environ: *mut *mut std::os::raw::c_char;
    #[no_mangle]
    fn execve(
        __path: *const std::os::raw::c_char,
        __argv: *const *mut std::os::raw::c_char,
        __envp: *const *mut std::os::raw::c_char,
    ) -> i32;
    #[no_mangle]
    fn _exit(_: i32) -> !;
    #[no_mangle]
    fn sysconf(__name: i32) -> i64;
    #[no_mangle]
    fn setuid(__uid: __uid_t) -> i32;
    #[no_mangle]
    fn setgid(__gid: __gid_t) -> i32;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn isatty(__fd: i32) -> i32;
    #[no_mangle]
    fn symlink(__from: *const std::os::raw::c_char, __to: *const std::os::raw::c_char) -> i32;
    #[no_mangle]
    fn readlink(
        __path: *const std::os::raw::c_char,
        __buf: *mut std::os::raw::c_char,
        __len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn __errno_location() -> *mut i32;
    #[no_mangle]
    fn open(__file: *const std::os::raw::c_char, __oflag: i32, _: ...) -> i32;
    #[no_mangle]
    fn utimes(__file: *const std::os::raw::c_char, __tvp: *const timeval) -> i32;
    #[no_mangle]
    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> i32;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> i32;
    #[no_mangle]
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: i32) -> i32;
    #[no_mangle]
    fn mkdir(__path: *const std::os::raw::c_char, __mode: __mode_t) -> i32;
    #[no_mangle]
    fn __xstat(__ver: i32, __filename: *const std::os::raw::c_char, __stat_buf: *mut stat) -> i32;
    #[no_mangle]
    fn __lxstat(__ver: i32, __filename: *const std::os::raw::c_char, __stat_buf: *mut stat) -> i32;
    #[no_mangle]
    fn opendir(__name: *const std::os::raw::c_char) -> *mut DIR;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> i32;
    #[no_mangle]
    fn dlopen(__file: *const std::os::raw::c_char, __mode: i32) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn dlclose(__handle: *mut std::ffi::c_void) -> i32;
    #[no_mangle]
    fn dlsym(
        __handle: *mut std::ffi::c_void,
        __name: *const std::os::raw::c_char,
    ) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn tcgetattr(__fd: i32, __termios_p: *mut termios) -> i32;
    #[no_mangle]
    fn tcsetattr(__fd: i32, __optional_actions: i32, __termios_p: *const termios) -> i32;
    #[no_mangle]
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut i32, __options: i32) -> __pid_t;
    #[no_mangle]
    fn pstrcpy(buf: *mut std::os::raw::c_char, buf_size: i32, str: *const std::os::raw::c_char);
    #[no_mangle]
    fn pstrcat(
        buf: *mut std::os::raw::c_char,
        buf_size: i32,
        s: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn has_suffix(str: *const std::os::raw::c_char, suffix: *const std::os::raw::c_char) -> i32;
    #[no_mangle]
    fn dbuf_init2(
        s: *mut DynBuf,
        opaque: *mut std::ffi::c_void,
        realloc_func: Option<DynBufReallocFunc>,
    );
    #[no_mangle]
    fn dbuf_put(s: *mut DynBuf, data: *const uint8_t, len: size_t) -> i32;
    #[no_mangle]
    fn dbuf_putc(s: *mut DynBuf, c: uint8_t) -> i32;
    #[no_mangle]
    fn dbuf_putstr(s: *mut DynBuf, str: *const std::os::raw::c_char) -> i32;
    #[no_mangle]
    fn dbuf_printf(s: *mut DynBuf, fmt: *const std::os::raw::c_char, _: ...) -> i32;
    #[no_mangle]
    fn dbuf_free(s: *mut DynBuf);
    #[no_mangle]
    fn unicode_to_utf8(buf: *mut uint8_t, c: u32) -> i32;
    #[no_mangle]
    fn unicode_from_utf8(p: *const uint8_t, max_len: i32, pp: *mut *const uint8_t) -> i32;
    #[no_mangle]
    fn JS_GetRuntimeOpaque(rt: *mut JSRuntime) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn JS_SetRuntimeOpaque(rt: *mut JSRuntime, opaque: *mut std::ffi::c_void);
    #[no_mangle]
    fn JS_MarkValue(rt: *mut JSRuntime, val: JSValue, mark_func: Option<JS_MarkFunc>);
    #[no_mangle]
    fn JS_RunGC(rt: *mut JSRuntime);
    #[no_mangle]
    fn JS_GetRuntime(ctx: *mut JSContext) -> *mut JSRuntime;
    #[no_mangle]
    fn JS_SetClassProto(ctx: *mut JSContext, class_id: JSClassID, obj: JSValue);
    #[no_mangle]
    fn js_free_rt(rt: *mut JSRuntime, ptr: *mut std::ffi::c_void);
    #[no_mangle]
    fn js_realloc_rt(
        rt: *mut JSRuntime,
        ptr: *mut std::ffi::c_void,
        size: size_t,
    ) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn js_malloc(ctx: *mut JSContext, size: size_t) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn js_free(ctx: *mut JSContext, ptr: *mut std::ffi::c_void);
    #[no_mangle]
    fn js_mallocz(ctx: *mut JSContext, size: size_t) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn JS_NewAtomLen(ctx: *mut JSContext, str: *const std::os::raw::c_char, len: size_t) -> JSAtom;
    #[no_mangle]
    fn JS_FreeAtom(ctx: *mut JSContext, v: JSAtom);
    #[no_mangle]
    fn JS_AtomToCString(ctx: *mut JSContext, atom: JSAtom) -> *const std::os::raw::c_char;
    #[no_mangle]
    fn JS_NewClassID(pclass_id: *mut JSClassID) -> JSClassID;
    #[no_mangle]
    fn JS_NewClass(rt: *mut JSRuntime, class_id: JSClassID, class_def: *const JSClassDef) -> i32;
    #[no_mangle]
    fn JS_NewBigInt64(ctx: *mut JSContext, v: int64_t) -> JSValue;
    #[no_mangle]
    fn JS_GetException(ctx: *mut JSContext) -> JSValue;
    #[no_mangle]
    fn JS_IsError(ctx: *mut JSContext, val: JSValue) -> i32;
    #[no_mangle]
    fn JS_ResetUncatchableError(ctx: *mut JSContext);
    #[no_mangle]
    fn JS_ThrowTypeError(ctx: *mut JSContext, fmt: *const std::os::raw::c_char, _: ...) -> JSValue;
    #[no_mangle]
    fn JS_ThrowReferenceError(
        ctx: *mut JSContext,
        fmt: *const std::os::raw::c_char,
        _: ...
    ) -> JSValue;
    #[no_mangle]
    fn JS_ThrowRangeError(ctx: *mut JSContext, fmt: *const std::os::raw::c_char, _: ...)
        -> JSValue;
    #[no_mangle]
    fn JS_ThrowOutOfMemory(ctx: *mut JSContext) -> JSValue;
    #[no_mangle]
    fn __JS_FreeValue(ctx: *mut JSContext, v: JSValue);
    #[no_mangle]
    fn __JS_FreeValueRT(rt: *mut JSRuntime, v: JSValue);
    #[no_mangle]
    fn JS_ToBool(ctx: *mut JSContext, val: JSValue) -> i32;
    #[no_mangle]
    fn JS_ToInt32(ctx: *mut JSContext, pres: *mut int32_t, val: JSValue) -> i32;
    #[no_mangle]
    fn JS_ToInt64(ctx: *mut JSContext, pres: *mut int64_t, val: JSValue) -> i32;
    #[no_mangle]
    fn JS_ToIndex(ctx: *mut JSContext, plen: *mut uint64_t, val: JSValue) -> i32;
    #[no_mangle]
    fn JS_ToFloat64(ctx: *mut JSContext, pres: *mut f64, val: JSValue) -> i32;
    #[no_mangle]
    fn JS_ToInt64Ext(ctx: *mut JSContext, pres: *mut int64_t, val: JSValue) -> i32;
    #[no_mangle]
    fn JS_NewStringLen(
        ctx: *mut JSContext,
        str1: *const std::os::raw::c_char,
        len1: size_t,
    ) -> JSValue;
    #[no_mangle]
    fn JS_NewString(ctx: *mut JSContext, str: *const std::os::raw::c_char) -> JSValue;
    #[no_mangle]
    fn JS_ToCStringLen2(
        ctx: *mut JSContext,
        plen: *mut size_t,
        val1: JSValue,
        cesu8: i32,
    ) -> *const std::os::raw::c_char;
    #[no_mangle]
    fn JS_FreeCString(ctx: *mut JSContext, ptr: *const std::os::raw::c_char);
    #[no_mangle]
    fn JS_NewObjectClass(ctx: *mut JSContext, class_id: i32) -> JSValue;
    #[no_mangle]
    fn JS_NewObject(ctx: *mut JSContext) -> JSValue;
    #[no_mangle]
    fn JS_IsFunction(ctx: *mut JSContext, val: JSValue) -> i32;
    #[no_mangle]
    fn JS_NewArray(ctx: *mut JSContext) -> JSValue;
    #[no_mangle]
    fn JS_GetPropertyInternal(
        ctx: *mut JSContext,
        obj: JSValue,
        prop: JSAtom,
        receiver: JSValue,
        throw_ref_error: i32,
    ) -> JSValue;
    #[no_mangle]
    fn JS_GetPropertyStr(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: *const std::os::raw::c_char,
    ) -> JSValue;
    #[no_mangle]
    fn JS_GetPropertyUint32(ctx: *mut JSContext, this_obj: JSValue, idx: uint32_t) -> JSValue;
    #[no_mangle]
    fn JS_SetPropertyUint32(
        ctx: *mut JSContext,
        this_obj: JSValue,
        idx: uint32_t,
        val: JSValue,
    ) -> i32;
    #[no_mangle]
    fn JS_SetPropertyStr(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: *const std::os::raw::c_char,
        val: JSValue,
    ) -> i32;
    #[no_mangle]
    fn JS_GetOwnPropertyNames(
        ctx: *mut JSContext,
        ptab: *mut *mut JSPropertyEnum,
        plen: *mut uint32_t,
        obj: JSValue,
        flags: i32,
    ) -> i32;
    #[no_mangle]
    fn JS_Call(
        ctx: *mut JSContext,
        func_obj: JSValue,
        this_obj: JSValue,
        argc: i32,
        argv: *mut JSValue,
    ) -> JSValue;
    #[no_mangle]
    fn JS_Eval(
        ctx: *mut JSContext,
        input: *const std::os::raw::c_char,
        input_len: size_t,
        filename: *const std::os::raw::c_char,
        eval_flags: i32,
    ) -> JSValue;
    #[no_mangle]
    fn JS_GetGlobalObject(ctx: *mut JSContext) -> JSValue;
    #[no_mangle]
    fn JS_DefinePropertyValue(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: JSAtom,
        val: JSValue,
        flags: i32,
    ) -> i32;
    #[no_mangle]
    fn JS_DefinePropertyValueUint32(
        ctx: *mut JSContext,
        this_obj: JSValue,
        idx: uint32_t,
        val: JSValue,
        flags: i32,
    ) -> i32;
    #[no_mangle]
    fn JS_DefinePropertyValueStr(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: *const std::os::raw::c_char,
        val: JSValue,
        flags: i32,
    ) -> i32;
    #[no_mangle]
    fn JS_SetOpaque(obj: JSValue, opaque: *mut std::ffi::c_void);
    #[no_mangle]
    fn JS_GetOpaque(obj: JSValue, class_id: JSClassID) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn JS_GetOpaque2(
        ctx: *mut JSContext,
        obj: JSValue,
        class_id: JSClassID,
    ) -> *mut std::ffi::c_void;
    #[no_mangle]
    fn JS_ParseJSON2(
        ctx: *mut JSContext,
        buf: *const std::os::raw::c_char,
        buf_len: size_t,
        filename: *const std::os::raw::c_char,
        flags: i32,
    ) -> JSValue;
    #[no_mangle]
    fn JS_NewArrayBufferCopy(ctx: *mut JSContext, buf: *const uint8_t, len: size_t) -> JSValue;
    #[no_mangle]
    fn JS_GetArrayBuffer(ctx: *mut JSContext, psize: *mut size_t, obj: JSValue) -> *mut uint8_t;
    #[no_mangle]
    fn JS_SetInterruptHandler(
        rt: *mut JSRuntime,
        cb: Option<JSInterruptHandler>,
        opaque: *mut std::ffi::c_void,
    );
    #[no_mangle]
    fn JS_GetImportMeta(ctx: *mut JSContext, m: *mut JSModuleDef) -> JSValue;
    #[no_mangle]
    fn JS_GetModuleName(ctx: *mut JSContext, m: *mut JSModuleDef) -> JSAtom;
    #[no_mangle]
    fn JS_ExecutePendingJob(rt: *mut JSRuntime, pctx: *mut *mut JSContext) -> i32;
    #[no_mangle]
    fn JS_ReadObject(
        ctx: *mut JSContext,
        buf: *const uint8_t,
        buf_len: size_t,
        flags: i32,
    ) -> JSValue;
    #[no_mangle]
    fn JS_EvalFunction(ctx: *mut JSContext, fun_obj: JSValue) -> JSValue;
    #[no_mangle]
    fn JS_ResolveModule(ctx: *mut JSContext, obj: JSValue) -> i32;
    #[no_mangle]
    fn JS_NewCFunction2(
        ctx: *mut JSContext,
        func: Option<JSCFunction>,
        name: *const std::os::raw::c_char,
        length: i32,
        cproto: JSCFunctionEnum,
        magic: i32,
    ) -> JSValue;
    #[no_mangle]
    fn JS_SetPropertyFunctionList(
        ctx: *mut JSContext,
        obj: JSValue,
        tab: *const JSCFunctionListEntry,
        len: i32,
    );
    #[no_mangle]
    fn JS_NewCModule(
        ctx: *mut JSContext,
        name_str: *const std::os::raw::c_char,
        func: Option<JSModuleInitFunc>,
    ) -> *mut JSModuleDef;
    #[no_mangle]
    fn JS_AddModuleExport(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        name_str: *const std::os::raw::c_char,
    ) -> i32;
    #[no_mangle]
    fn JS_AddModuleExportList(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        tab: *const JSCFunctionListEntry,
        len: i32,
    ) -> i32;
    #[no_mangle]
    fn JS_SetModuleExportList(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        tab: *const JSCFunctionListEntry,
        len: i32,
    ) -> i32;
    #[no_mangle]
    fn JS_SetModuleExport(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        export_name: *const std::os::raw::c_char,
        val: JSValue,
    ) -> i32;
}
pub type size_t = u64;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __int16_t = i16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __clockid_t = i32;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = i64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: std::os::raw::c_char,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [std::os::raw::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type C2RustUnnamed = u32;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: i32) -> ()>;
pub type sighandler_t = __sighandler_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: u16,
    pub d_type: std::os::raw::c_uchar,
    pub d_name: [std::os::raw::c_char; 256],
}
pub type DIR = __dirstream;
pub type cc_t = std::os::raw::c_uchar;
pub type speed_t = u32;
pub type tcflag_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}
pub type BOOL = i32;
pub type C2RustUnnamed_0 = u32;
pub const TRUE: C2RustUnnamed_0 = 1;
pub const FALSE: C2RustUnnamed_0 = 0;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}
pub type JSClassID = uint32_t;
pub type JSAtom = uint32_t;
pub type C2RustUnnamed_1 = i32;
pub const JS_TAG_FLOAT64: C2RustUnnamed_1 = 7;
pub const JS_TAG_EXCEPTION: C2RustUnnamed_1 = 6;
pub const JS_TAG_CATCH_OFFSET: C2RustUnnamed_1 = 5;
pub const JS_TAG_UNINITIALIZED: C2RustUnnamed_1 = 4;
pub const JS_TAG_UNDEFINED: C2RustUnnamed_1 = 3;
pub const JS_TAG_NULL: C2RustUnnamed_1 = 2;
pub const JS_TAG_BOOL: C2RustUnnamed_1 = 1;
pub const JS_TAG_INT: C2RustUnnamed_1 = 0;
pub const JS_TAG_OBJECT: C2RustUnnamed_1 = -1;
pub const JS_TAG_FUNCTION_BYTECODE: C2RustUnnamed_1 = -2;
pub const JS_TAG_MODULE: C2RustUnnamed_1 = -3;
pub const JS_TAG_STRING: C2RustUnnamed_1 = -7;
pub const JS_TAG_SYMBOL: C2RustUnnamed_1 = -8;
pub const JS_TAG_BIG_FLOAT: C2RustUnnamed_1 = -9;
pub const JS_TAG_BIG_INT: C2RustUnnamed_1 = -10;
pub const JS_TAG_BIG_DECIMAL: C2RustUnnamed_1 = -11;
pub const JS_TAG_FIRST: C2RustUnnamed_1 = -11;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSRefCountHeader {
    pub ref_count: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSValueUnion {
    pub int32: int32_t,
    pub float64: f64,
    pub ptr: *mut std::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSValue {
    pub u: JSValueUnion,
    pub tag: int64_t,
}
pub type JSCFunction =
    unsafe extern "C" fn(_: *mut JSContext, _: JSValue, _: i32, _: *mut JSValue) -> JSValue;
pub type JS_MarkFunc = unsafe extern "C" fn(_: *mut JSRuntime, _: *mut JSGCObjectHeader) -> ();

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSPropertyEnum {
    pub is_enumerable: i32,
    pub atom: JSAtom,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSPropertyDescriptor {
    pub flags: i32,
    pub value: JSValue,
    pub getter: JSValue,
    pub setter: JSValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSClassExoticMethods {
    pub get_own_property: Option<
        unsafe extern "C" fn(
            _: *mut JSContext,
            _: *mut JSPropertyDescriptor,
            _: JSValue,
            _: JSAtom,
        ) -> i32,
    >,
    pub get_own_property_names: Option<
        unsafe extern "C" fn(
            _: *mut JSContext,
            _: *mut *mut JSPropertyEnum,
            _: *mut uint32_t,
            _: JSValue,
        ) -> i32,
    >,
    pub delete_property:
        Option<unsafe extern "C" fn(_: *mut JSContext, _: JSValue, _: JSAtom) -> i32>,
    pub define_own_property: Option<
        unsafe extern "C" fn(
            _: *mut JSContext,
            _: JSValue,
            _: JSAtom,
            _: JSValue,
            _: JSValue,
            _: JSValue,
            _: i32,
        ) -> i32,
    >,
    pub has_property: Option<unsafe extern "C" fn(_: *mut JSContext, _: JSValue, _: JSAtom) -> i32>,
    pub get_property: Option<
        unsafe extern "C" fn(_: *mut JSContext, _: JSValue, _: JSAtom, _: JSValue) -> JSValue,
    >,
    pub set_property: Option<
        unsafe extern "C" fn(
            _: *mut JSContext,
            _: JSValue,
            _: JSAtom,
            _: JSValue,
            _: JSValue,
            _: i32,
        ) -> i32,
    >,
}
pub type JSClassFinalizer = unsafe extern "C" fn(_: *mut JSRuntime, _: JSValue) -> ();
pub type JSClassGCMark =
    unsafe extern "C" fn(_: *mut JSRuntime, _: JSValue, _: Option<JS_MarkFunc>) -> ();
pub type JSClassCall = unsafe extern "C" fn(
    _: *mut JSContext,
    _: JSValue,
    _: JSValue,
    _: i32,
    _: *mut JSValue,
    _: i32,
) -> JSValue;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSClassDef {
    pub class_name: *const std::os::raw::c_char,
    pub finalizer: Option<JSClassFinalizer>,
    pub gc_mark: Option<JSClassGCMark>,
    pub call: Option<JSClassCall>,
    pub exotic: *mut JSClassExoticMethods,
}
pub type JSInterruptHandler =
    unsafe extern "C" fn(_: *mut JSRuntime, _: *mut std::ffi::c_void) -> i32;
pub type JSCFunctionEnum = u32;
pub const JS_CFUNC_iterator_next: JSCFunctionEnum = 12;
pub const JS_CFUNC_setter_magic: JSCFunctionEnum = 11;
pub const JS_CFUNC_getter_magic: JSCFunctionEnum = 10;
pub const JS_CFUNC_setter: JSCFunctionEnum = 9;
pub const JS_CFUNC_getter: JSCFunctionEnum = 8;
pub const JS_CFUNC_f_f_f: JSCFunctionEnum = 7;
pub const JS_CFUNC_f_f: JSCFunctionEnum = 6;
pub const JS_CFUNC_constructor_or_func_magic: JSCFunctionEnum = 5;
pub const JS_CFUNC_constructor_or_func: JSCFunctionEnum = 4;
pub const JS_CFUNC_constructor_magic: JSCFunctionEnum = 3;
pub const JS_CFUNC_constructor: JSCFunctionEnum = 2;
pub const JS_CFUNC_generic_magic: JSCFunctionEnum = 1;
pub const JS_CFUNC_generic: JSCFunctionEnum = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSCFunctionType {
    pub generic: Option<JSCFunction>,
    pub generic_magic: Option<
        unsafe extern "C" fn(
            _: *mut JSContext,
            _: JSValue,
            _: i32,
            _: *mut JSValue,
            _: i32,
        ) -> JSValue,
    >,
    pub constructor: Option<JSCFunction>,
    pub constructor_magic: Option<
        unsafe extern "C" fn(
            _: *mut JSContext,
            _: JSValue,
            _: i32,
            _: *mut JSValue,
            _: i32,
        ) -> JSValue,
    >,
    pub constructor_or_func: Option<JSCFunction>,
    pub f_f: Option<unsafe extern "C" fn(_: f64) -> f64>,
    pub f_f_f: Option<unsafe extern "C" fn(_: f64, _: f64) -> f64>,
    pub getter: Option<unsafe extern "C" fn(_: *mut JSContext, _: JSValue) -> JSValue>,
    pub setter: Option<unsafe extern "C" fn(_: *mut JSContext, _: JSValue, _: JSValue) -> JSValue>,
    pub getter_magic:
        Option<unsafe extern "C" fn(_: *mut JSContext, _: JSValue, _: i32) -> JSValue>,
    pub setter_magic:
        Option<unsafe extern "C" fn(_: *mut JSContext, _: JSValue, _: JSValue, _: i32) -> JSValue>,
    pub iterator_next: Option<
        unsafe extern "C" fn(
            _: *mut JSContext,
            _: JSValue,
            _: i32,
            _: *mut JSValue,
            _: *mut i32,
            _: i32,
        ) -> JSValue,
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSCFunctionListEntry {
    pub name: *const std::os::raw::c_char,
    pub prop_flags: uint8_t,
    pub def_type: uint8_t,
    pub magic: int16_t,
    pub u: C2RustUnnamed_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_2 {
    pub func: C2RustUnnamed_6,
    pub getset: C2RustUnnamed_5,
    pub alias: C2RustUnnamed_4,
    pub prop_list: C2RustUnnamed_3,
    pub str_0: *const std::os::raw::c_char,
    pub i32_0: int32_t,
    pub i64_0: int64_t,
    pub f64_0: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub tab: *const JSCFunctionListEntry,
    pub len: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub name: *const std::os::raw::c_char,
    pub base: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub get: JSCFunctionType,
    pub set: JSCFunctionType,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub length: uint8_t,
    pub cproto: uint8_t,
    pub cfunc: JSCFunctionType,
}
pub type JSModuleInitFunc = unsafe extern "C" fn(_: *mut JSContext, _: *mut JSModuleDef) -> i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSSTDFile {
    pub f: *mut FILE,
    pub close_in_finalizer: BOOL,
    pub is_popen: BOOL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSThreadState {
    pub os_rw_handlers: list_head,
    pub os_signal_handlers: list_head,
    pub os_timers: list_head,
    pub port_list: list_head,
    pub eval_script_recurse: i32,
    pub recv_pipe: *mut JSWorkerMessagePipe,
    pub send_pipe: *mut JSWorkerMessagePipe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSWorkerMessagePipe {
    pub ref_count: i32,
    pub msg_queue: list_head,
    pub read_fd: i32,
    pub write_fd: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSOSTimer {
    pub link: list_head,
    pub has_object: BOOL,
    pub timeout: int64_t,
    pub func: JSValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSOSSignalHandler {
    pub link: list_head,
    pub sig_num: i32,
    pub func: JSValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSOSRWHandler {
    pub link: list_head,
    pub fd: i32,
    pub rw_func: [JSValue; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSWorkerMessageHandler {
    pub link: list_head,
    pub recv_pipe: *mut JSWorkerMessagePipe,
    pub on_message_func: JSValue,
}
pub type JSInitModuleFunc =
    unsafe extern "C" fn(_: *mut JSContext, _: *const std::os::raw::c_char) -> *mut JSModuleDef;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const std::os::raw::c_char) -> i32 {
    return strtol(
        __nptr,
        0 as *mut std::ffi::c_void as *mut *mut std::os::raw::c_char,
        10 as i32,
    ) as i32;
}
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const std::os::raw::c_char,
    mut __statbuf: *mut stat,
) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const std::os::raw::c_char,
    mut __statbuf: *mut stat,
) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn max_int(mut a: i32, mut b: i32) -> i32 {
    if a > b {
        return a;
    } else {
        return b;
    };
}
#[inline]
unsafe extern "C" fn dbuf_error(mut s: *mut DynBuf) -> BOOL {
    return (*s).error;
}
#[inline]
unsafe extern "C" fn init_list_head(mut head: *mut list_head) {
    (*head).prev = head;
    (*head).next = head;
}
#[inline]
unsafe extern "C" fn __list_add(
    mut el: *mut list_head,
    mut prev: *mut list_head,
    mut next: *mut list_head,
) {
    (*prev).next = el;
    (*el).prev = prev;
    (*el).next = next;
    (*next).prev = el;
}
#[inline]
unsafe extern "C" fn list_add_tail(mut el: *mut list_head, mut head: *mut list_head) {
    __list_add(el, (*head).prev, head);
}
#[inline]
unsafe extern "C" fn list_del(mut el: *mut list_head) {
    let mut prev: *mut list_head = 0 as *mut list_head;
    let mut next: *mut list_head = 0 as *mut list_head;
    prev = (*el).prev;
    next = (*el).next;
    (*prev).next = next;
    (*next).prev = prev;
    (*el).prev = 0 as *mut list_head;
    (*el).next = 0 as *mut list_head;
}
#[inline]
unsafe extern "C" fn list_empty(mut el: *mut list_head) -> i32 {
    return ((*el).next == el) as i32;
}
#[inline]
unsafe extern "C" fn __JS_NewFloat64(mut ctx: *mut JSContext, mut d: f64) -> JSValue {
    let mut v: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    v.tag = JS_TAG_FLOAT64 as i32 as int64_t;
    v.u.float64 = d;
    return v;
}
#[inline(always)]
unsafe extern "C" fn JS_NewBool(mut ctx: *mut JSContext, mut val: i32) -> JSValue {
    return {
        let mut init = JSValue {
            u: JSValueUnion {
                int32: (val != 0 as i32) as i32,
            },
            tag: JS_TAG_BOOL as i32 as int64_t,
        };
        init
    };
}
#[inline(always)]
unsafe extern "C" fn JS_NewInt32(mut ctx: *mut JSContext, mut val: int32_t) -> JSValue {
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: val },
            tag: JS_TAG_INT as i32 as int64_t,
        };
        init
    };
}
#[inline(always)]
unsafe extern "C" fn JS_NewInt64(mut ctx: *mut JSContext, mut val: int64_t) -> JSValue {
    let mut v: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    if val == val as int32_t as i64 {
        v = JS_NewInt32(ctx, val as int32_t)
    } else {
        v = __JS_NewFloat64(ctx, val as f64)
    }
    return v;
}
#[inline]
unsafe extern "C" fn JS_IsBigInt(mut ctx: *mut JSContext, mut v: JSValue) -> i32 {
    let mut tag: i32 = v.tag as int32_t;
    return (tag == JS_TAG_BIG_INT as i32) as i32;
}
#[inline]
unsafe extern "C" fn JS_IsNull(mut v: JSValue) -> i32 {
    return (v.tag as int32_t == JS_TAG_NULL as i32) as i32;
}
#[inline]
unsafe extern "C" fn JS_IsUndefined(mut v: JSValue) -> i32 {
    return (v.tag as int32_t == JS_TAG_UNDEFINED as i32) as i32;
}
#[inline]
unsafe extern "C" fn JS_IsException(mut v: JSValue) -> i32 {
    return (v.tag as int32_t == JS_TAG_EXCEPTION as i32) as i32 as i64 as i32;
}
#[inline]
unsafe extern "C" fn JS_IsString(mut v: JSValue) -> i32 {
    return (v.tag as int32_t == JS_TAG_STRING as i32) as i32;
}
#[inline]
unsafe extern "C" fn JS_FreeValue(mut ctx: *mut JSContext, mut v: JSValue) {
    if v.tag as int32_t as u32 >= JS_TAG_FIRST as i32 as u32 {
        let mut p: *mut JSRefCountHeader = v.u.ptr as *mut JSRefCountHeader;
        (*p).ref_count -= 1;
        if (*p).ref_count <= 0 as i32 {
            __JS_FreeValue(ctx, v);
        }
    };
}
#[inline]
unsafe extern "C" fn JS_FreeValueRT(mut rt: *mut JSRuntime, mut v: JSValue) {
    if v.tag as int32_t as u32 >= JS_TAG_FIRST as i32 as u32 {
        let mut p: *mut JSRefCountHeader = v.u.ptr as *mut JSRefCountHeader;
        (*p).ref_count -= 1;
        if (*p).ref_count <= 0 as i32 {
            __JS_FreeValueRT(rt, v);
        }
    };
}
#[inline]
unsafe extern "C" fn JS_DupValue(mut ctx: *mut JSContext, mut v: JSValue) -> JSValue {
    if v.tag as int32_t as u32 >= JS_TAG_FIRST as i32 as u32 {
        let mut p: *mut JSRefCountHeader = v.u.ptr as *mut JSRefCountHeader;
        (*p).ref_count += 1
    }
    return v;
}
#[inline]
unsafe extern "C" fn JS_ToUint32(
    mut ctx: *mut JSContext,
    mut pres: *mut uint32_t,
    mut val: JSValue,
) -> i32 {
    return JS_ToInt32(ctx, pres as *mut int32_t, val);
}
#[inline]
unsafe extern "C" fn JS_ToCStringLen(
    mut ctx: *mut JSContext,
    mut plen: *mut size_t,
    mut val1: JSValue,
) -> *const std::os::raw::c_char {
    return JS_ToCStringLen2(ctx, plen, val1, 0 as i32);
}
#[inline]
unsafe extern "C" fn JS_ToCString(
    mut ctx: *mut JSContext,
    mut val1: JSValue,
) -> *const std::os::raw::c_char {
    return JS_ToCStringLen2(ctx, 0 as *mut size_t, val1, 0 as i32);
}
#[inline(always)]
unsafe extern "C" fn JS_GetProperty(
    mut ctx: *mut JSContext,
    mut this_obj: JSValue,
    mut prop: JSAtom,
) -> JSValue {
    return JS_GetPropertyInternal(ctx, this_obj, prop, this_obj, 0 as i32);
}
#[inline]
unsafe extern "C" fn JS_NewCFunction(
    mut ctx: *mut JSContext,
    mut func: Option<JSCFunction>,
    mut name: *const std::os::raw::c_char,
    mut length: i32,
) -> JSValue {
    return JS_NewCFunction2(ctx, func, name, length, JS_CFUNC_generic, 0 as i32);
}
static mut os_pending_signals: uint64_t = 0;
static mut os_poll_func: Option<unsafe extern "C" fn(_: *mut JSContext) -> i32> = None;
unsafe extern "C" fn js_std_dbuf_init(mut ctx: *mut JSContext, mut s: *mut DynBuf) {
    dbuf_init2(
        s,
        JS_GetRuntime(ctx) as *mut std::ffi::c_void,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut JSRuntime,
                    _: *mut std::ffi::c_void,
                    _: size_t,
                ) -> *mut std::ffi::c_void,
            >,
            Option<DynBufReallocFunc>,
        >(Some(
            js_realloc_rt
                as unsafe extern "C" fn(
                    _: *mut JSRuntime,
                    _: *mut std::ffi::c_void,
                    _: size_t,
                ) -> *mut std::ffi::c_void,
        )),
    );
}
unsafe extern "C" fn my_isdigit(mut c: i32) -> BOOL {
    return (c >= '0' as i32 && c <= '9' as i32) as i32;
}
unsafe extern "C" fn js_printf_internal(
    mut ctx: *mut JSContext,
    mut argc: i32,
    mut argv: *mut JSValue,
    mut fp: *mut FILE,
) -> JSValue {
    let mut current_block: u64;
    let mut fmtbuf: [std::os::raw::c_char; 32] = [0; 32];
    let mut cbuf: [uint8_t; 7] = [0; 7];
    let mut res: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut dbuf: DynBuf = DynBuf {
        buf: 0 as *mut uint8_t,
        size: 0,
        allocated_size: 0,
        error: 0,
        realloc_func: None,
        opaque: 0 as *mut std::ffi::c_void,
    };
    let mut fmt_str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut fmt: *const uint8_t = 0 as *const uint8_t;
    let mut fmt_end: *const uint8_t = 0 as *const uint8_t;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut q: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut len: i32 = 0;
    let mut mod_0: i32 = 0;
    let mut fmt_len: size_t = 0;
    let mut int32_arg: int32_t = 0;
    let mut int64_arg: int64_t = 0;
    let mut double_arg: f64 = 0.;
    let mut string_arg: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    /* Use indirect call to dbuf_printf to prevent gcc warning */
    let mut dbuf_printf_fun: Option<
        unsafe extern "C" fn(_: *mut DynBuf, _: *const std::os::raw::c_char, _: ...) -> i32,
    > = ::std::mem::transmute::<
        *mut std::ffi::c_void,
        Option<unsafe extern "C" fn(_: *mut DynBuf, _: *const std::os::raw::c_char, _: ...) -> i32>,
    >(::std::mem::transmute::<
        Option<unsafe extern "C" fn(_: *mut DynBuf, _: *const std::os::raw::c_char, _: ...) -> i32>,
        *mut std::ffi::c_void,
    >(Some(
        dbuf_printf
            as unsafe extern "C" fn(_: *mut DynBuf, _: *const std::os::raw::c_char, _: ...) -> i32,
    ))); /* copy '%' */
    js_std_dbuf_init(ctx, &mut dbuf);
    if argc > 0 as i32 {
        fmt_str = JS_ToCStringLen(ctx, &mut fmt_len, *argv.offset(0 as i32 as isize));
        if fmt_str.is_null() {
            current_block = 8304123123824580522;
        } else {
            i = 1 as i32;
            fmt = fmt_str as *const uint8_t;
            fmt_end = fmt.offset(fmt_len as isize);
            loop {
                if !(fmt < fmt_end) {
                    current_block = 11865390570819897086;
                    break;
                }
                p = fmt;
                while fmt < fmt_end && *fmt as i32 != '%' as i32 {
                    fmt = fmt.offset(1)
                }
                dbuf_put(&mut dbuf, p, fmt.wrapping_offset_from(p) as i64 as size_t);
                if fmt >= fmt_end {
                    current_block = 11865390570819897086;
                    break;
                }
                q = fmtbuf.as_mut_ptr();
                let fresh0 = fmt;
                fmt = fmt.offset(1);
                let fresh1 = q;
                q = q.offset(1);
                *fresh1 = *fresh0 as std::os::raw::c_char;
                loop
                /* flags */
                {
                    c = *fmt as i32;
                    if !(c == '0' as i32
                        || c == '#' as i32
                        || c == '+' as i32
                        || c == '-' as i32
                        || c == ' ' as i32
                        || c == '\'' as i32)
                    {
                        current_block = 17788412896529399552;
                        break;
                    }
                    if q >= fmtbuf
                        .as_mut_ptr()
                        .offset(::std::mem::size_of::<[std::os::raw::c_char; 32]>() as u64 as isize)
                        .offset(-(1 as i32 as isize))
                    {
                        current_block = 11188143500741601598;
                        break;
                    }
                    let fresh2 = q;
                    q = q.offset(1);
                    *fresh2 = c as std::os::raw::c_char;
                    fmt = fmt.offset(1)
                }
                match current_block {
                    17788412896529399552 =>
                    /* width */
                    {
                        if *fmt as i32 == '*' as i32 {
                            if i >= argc {
                                current_block = 10409069600589371798;
                            } else {
                                let fresh3 = i;
                                i = i + 1;
                                if JS_ToInt32(ctx, &mut int32_arg, *argv.offset(fresh3 as isize))
                                    != 0
                                {
                                    current_block = 8304123123824580522;
                                    break;
                                }
                                q = q.offset(snprintf(
                                    q,
                                    fmtbuf
                                        .as_mut_ptr()
                                        .offset(::std::mem::size_of::<[std::os::raw::c_char; 32]>()
                                            as u64
                                            as isize)
                                        .wrapping_offset_from(q)
                                        as i64 as u64,
                                    b"%d\x00" as *const u8 as *const std::os::raw::c_char,
                                    int32_arg,
                                ) as isize);
                                fmt = fmt.offset(1);
                                current_block = 3934796541983872331;
                            }
                        } else {
                            loop {
                                if !(my_isdigit(*fmt as i32) != 0) {
                                    current_block = 3934796541983872331;
                                    break;
                                }
                                if q >= fmtbuf
                                    .as_mut_ptr()
                                    .offset(::std::mem::size_of::<[std::os::raw::c_char; 32]>()
                                        as u64 as isize)
                                    .offset(-(1 as i32 as isize))
                                {
                                    current_block = 11188143500741601598;
                                    break;
                                }
                                let fresh4 = fmt;
                                fmt = fmt.offset(1);
                                let fresh5 = q;
                                q = q.offset(1);
                                *fresh5 = *fresh4 as std::os::raw::c_char
                            }
                        }
                        match current_block {
                            11188143500741601598 => {}
                            _ => {
                                match current_block {
                                    3934796541983872331 => {
                                        if *fmt as i32 == '.' as i32 {
                                            if q >= fmtbuf
                                                .as_mut_ptr()
                                                .offset(::std::mem::size_of::<
                                                    [std::os::raw::c_char; 32],
                                                >(
                                                )
                                                    as u64
                                                    as isize)
                                                .offset(-(1 as i32 as isize))
                                            {
                                                current_block = 11188143500741601598;
                                            } else {
                                                let fresh6 = fmt;
                                                fmt = fmt.offset(1);
                                                let fresh7 = q;
                                                q = q.offset(1);
                                                *fresh7 = *fresh6 as std::os::raw::c_char;
                                                if *fmt as i32 == '*' as i32 {
                                                    if i >= argc {
                                                        current_block = 10409069600589371798;
                                                    } else {
                                                        let fresh8 = i;
                                                        i = i + 1;
                                                        if JS_ToInt32(
                                                            ctx,
                                                            &mut int32_arg,
                                                            *argv.offset(fresh8 as isize),
                                                        ) != 0
                                                        {
                                                            current_block = 8304123123824580522;
                                                            break;
                                                        }
                                                        q = q.offset(snprintf(
                                                            q,
                                                            fmtbuf
                                                                .as_mut_ptr()
                                                                .offset(::std::mem::size_of::<
                                                                    [std::os::raw::c_char; 32],
                                                                >(
                                                                )
                                                                    as u64
                                                                    as isize)
                                                                .wrapping_offset_from(q)
                                                                as i64
                                                                as u64,
                                                            b"%d\x00" as *const u8
                                                                as *const std::os::raw::c_char,
                                                            int32_arg,
                                                        )
                                                            as isize);
                                                        fmt = fmt.offset(1);
                                                        current_block = 13763002826403452995;
                                                    }
                                                } else {
                                                    loop {
                                                        if !(my_isdigit(*fmt as i32) != 0) {
                                                            current_block = 13763002826403452995;
                                                            break;
                                                        }
                                                        if q >= fmtbuf
                                                            .as_mut_ptr()
                                                            .offset(::std::mem::size_of::<
                                                                [std::os::raw::c_char; 32],
                                                            >(
                                                            )
                                                                as u64
                                                                as isize)
                                                            .offset(-(1 as i32 as isize))
                                                        {
                                                            current_block = 11188143500741601598;
                                                            break;
                                                        }
                                                        let fresh9 = fmt;
                                                        fmt = fmt.offset(1);
                                                        let fresh10 = q;
                                                        q = q.offset(1);
                                                        *fresh10 = *fresh9 as std::os::raw::c_char
                                                    }
                                                }
                                            }
                                        } else {
                                            current_block = 13763002826403452995;
                                        }
                                        match current_block {
                                            11188143500741601598 => {}
                                            10409069600589371798 => {}
                                            _ => {
                                                /* we only support the "l" modifier for 64 bit numbers */
                                                mod_0 = ' ' as i32;
                                                if *fmt as i32 == 'l' as i32 {
                                                    let fresh11 = fmt;
                                                    fmt = fmt.offset(1);
                                                    mod_0 = *fresh11 as i32
                                                }
                                                /* type */
                                                let fresh12 = fmt;
                                                fmt = fmt.offset(1);
                                                c = *fresh12 as i32;
                                                if q >= fmtbuf
                                                    .as_mut_ptr()
                                                    .offset(::std::mem::size_of::<
                                                        [std::os::raw::c_char; 32],
                                                    >(
                                                    )
                                                        as u64
                                                        as isize)
                                                    .offset(-(1 as i32 as isize))
                                                {
                                                    current_block = 11188143500741601598;
                                                } else {
                                                    let fresh13 = q;
                                                    q = q.offset(1);
                                                    *fresh13 = c as std::os::raw::c_char;
                                                    *q = '\u{0}' as i32 as std::os::raw::c_char;
                                                    match c {
                                                        99 => {
                                                            current_block = 14889457129568826429;
                                                            match current_block {
                                                                10165534174818562619 => {
                                                                    dbuf_putc(
                                                                        &mut dbuf,
                                                                        '%' as i32 as uint8_t,
                                                                    );
                                                                    continue;
                                                                }
                                                                2394966745432187107 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh16 = i;
                                                                        i = i + 1;
                                                                        if JS_ToInt64Ext(
                                                                            ctx,
                                                                            &mut int64_arg,
                                                                            *argv.offset(
                                                                                fresh16 as isize,
                                                                            ),
                                                                        ) != 0
                                                                        {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        if mod_0 == 'l' as i32 {
                                                                            /* 64 bit number */
                                                                            if !(q
                                                                                     >=
                                                                                     fmtbuf.as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 32]>()
                                                                                                                    as
                                                                                                                    u64
                                                                                                                    as
                                                                                                                    isize).offset(-(2
                                                                                                                                        as
                                                                                                                                        i32
                                                                                                                                        as
                                                                                                                                        isize)))
                                                                               {
                                                                                *q.offset(1
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    *q.offset(-(1
                                                                                                    as
                                                                                                    i32)
                                                                                                  as
                                                                                                  isize);
                                                                                let ref mut fresh17 =
                                                                                    *q.offset(0
                                                                                                  as
                                                                                                  i32
                                                                                                  as
                                                                                                  isize);
                                                                                *fresh17
                                                                                    =
                                                                                    'l'
                                                                                        as
                                                                                        i32
                                                                                        as
                                                                                        std::os::raw::c_char;
                                                                                *q.offset(-(1
                                                                                                as
                                                                                                i32)
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    *fresh17;
                                                                                *q.offset(2
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    '\u{0}'
                                                                                        as
                                                                                        i32
                                                                                        as
                                                                                        std::os::raw::c_char;
                                                                                dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                        as
                                                                                                                                        *mut DynBuf,
                                                                                                                                    fmtbuf.as_mut_ptr(),
                                                                                                                                    int64_arg
                                                                                                                                        as
                                                                                                                                        i64);
                                                                                continue
                                                                                    ;
                                                                            }
                                                                        } else {
                                                                            dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                    as
                                                                                                                                    *mut DynBuf,
                                                                                                                                fmtbuf.as_mut_ptr(),
                                                                                                                                int64_arg
                                                                                                                                    as
                                                                                                                                    i32);
                                                                            continue;
                                                                        }
                                                                        current_block =
                                                                            11188143500741601598;
                                                                    }
                                                                }
                                                                3213037896719562574 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        /* XXX: handle strings containing null characters */
                                                                        let fresh18 = i;
                                                                        i = i + 1;
                                                                        string_arg = JS_ToCString(
                                                                            ctx,
                                                                            *argv.offset(
                                                                                fresh18 as isize,
                                                                            ),
                                                                        );
                                                                        if string_arg.is_null() {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                as
                                                                                                                                *mut DynBuf,
                                                                                                                            fmtbuf.as_mut_ptr(),
                                                                                                                            string_arg);
                                                                        JS_FreeCString(
                                                                            ctx, string_arg,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                                1348390045057916302 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh19 = i;
                                                                        i = i + 1;
                                                                        if JS_ToFloat64(
                                                                            ctx,
                                                                            &mut double_arg,
                                                                            *argv.offset(
                                                                                fresh19 as isize,
                                                                            ),
                                                                        ) != 0
                                                                        {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                as
                                                                                                                                *mut DynBuf,
                                                                                                                            fmtbuf.as_mut_ptr(),
                                                                                                                            double_arg);
                                                                        continue;
                                                                    }
                                                                }
                                                                _ => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        if JS_IsString(
                                                                            *argv
                                                                                .offset(i as isize),
                                                                        ) != 0
                                                                        {
                                                                            let fresh14 = i;
                                                                            i = i + 1;
                                                                            string_arg
                                                                                =
                                                                                JS_ToCString(ctx,
                                                                                             *argv.offset(fresh14
                                                                                                              as
                                                                                                              isize));
                                                                            if string_arg.is_null()
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    8304123123824580522;
                                                                                break;
                                                                            }
                                                                            int32_arg
                                                                                =
                                                                                unicode_from_utf8(string_arg
                                                                                                      as
                                                                                                      *mut uint8_t,
                                                                                                  6
                                                                                                      as
                                                                                                      i32,
                                                                                                  &mut p);
                                                                            JS_FreeCString(
                                                                                ctx, string_arg,
                                                                            );
                                                                        } else {
                                                                            let fresh15 = i;
                                                                            i = i + 1;
                                                                            if JS_ToInt32(
                                                                                ctx,
                                                                                &mut int32_arg,
                                                                                *argv.offset(
                                                                                    fresh15
                                                                                        as isize,
                                                                                ),
                                                                            ) != 0
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    8304123123824580522;
                                                                                break;
                                                                            }
                                                                        }
                                                                        /* handle utf-8 encoding explicitly */
                                                                        if int32_arg as u32
                                                                            > 0x10ffff as i32 as u32
                                                                        {
                                                                            int32_arg =
                                                                                0xfffd as i32
                                                                        }
                                                                        /* ignore conversion flags, width and precision */
                                                                        len = unicode_to_utf8(
                                                                            cbuf.as_mut_ptr(),
                                                                            int32_arg as u32,
                                                                        );
                                                                        dbuf_put(
                                                                            &mut dbuf,
                                                                            cbuf.as_mut_ptr(),
                                                                            len as size_t,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        100 | 105 | 111 | 117 | 120 | 88 => {
                                                            current_block = 2394966745432187107;
                                                            match current_block {
                                                                10165534174818562619 => {
                                                                    dbuf_putc(
                                                                        &mut dbuf,
                                                                        '%' as i32 as uint8_t,
                                                                    );
                                                                    continue;
                                                                }
                                                                2394966745432187107 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh16 = i;
                                                                        i = i + 1;
                                                                        if JS_ToInt64Ext(
                                                                            ctx,
                                                                            &mut int64_arg,
                                                                            *argv.offset(
                                                                                fresh16 as isize,
                                                                            ),
                                                                        ) != 0
                                                                        {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        if mod_0 == 'l' as i32 {
                                                                            if !(q
                                                                                     >=
                                                                                     fmtbuf.as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 32]>()
                                                                                                                    as
                                                                                                                    u64
                                                                                                                    as
                                                                                                                    isize).offset(-(2
                                                                                                                                        as
                                                                                                                                        i32
                                                                                                                                        as
                                                                                                                                        isize)))
                                                                               {
                                                                                *q.offset(1
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    *q.offset(-(1
                                                                                                    as
                                                                                                    i32)
                                                                                                  as
                                                                                                  isize);
                                                                                let ref mut fresh17 =
                                                                                    *q.offset(0
                                                                                                  as
                                                                                                  i32
                                                                                                  as
                                                                                                  isize);
                                                                                *fresh17
                                                                                    =
                                                                                    'l'
                                                                                        as
                                                                                        i32
                                                                                        as
                                                                                        std::os::raw::c_char;
                                                                                *q.offset(-(1
                                                                                                as
                                                                                                i32)
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    *fresh17;
                                                                                *q.offset(2
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    '\u{0}'
                                                                                        as
                                                                                        i32
                                                                                        as
                                                                                        std::os::raw::c_char;
                                                                                dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                        as
                                                                                                                                        *mut DynBuf,
                                                                                                                                    fmtbuf.as_mut_ptr(),
                                                                                                                                    int64_arg
                                                                                                                                        as
                                                                                                                                        i64);
                                                                                continue
                                                                                    ;
                                                                            }
                                                                        } else {
                                                                            dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                    as
                                                                                                                                    *mut DynBuf,
                                                                                                                                fmtbuf.as_mut_ptr(),
                                                                                                                                int64_arg
                                                                                                                                    as
                                                                                                                                    i32);
                                                                            continue;
                                                                        }
                                                                        current_block =
                                                                            11188143500741601598;
                                                                    }
                                                                }
                                                                3213037896719562574 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh18 = i;
                                                                        i = i + 1;
                                                                        string_arg = JS_ToCString(
                                                                            ctx,
                                                                            *argv.offset(
                                                                                fresh18 as isize,
                                                                            ),
                                                                        );
                                                                        if string_arg.is_null() {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                as
                                                                                                                                *mut DynBuf,
                                                                                                                            fmtbuf.as_mut_ptr(),
                                                                                                                            string_arg);
                                                                        JS_FreeCString(
                                                                            ctx, string_arg,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                                1348390045057916302 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh19 = i;
                                                                        i = i + 1;
                                                                        if JS_ToFloat64(
                                                                            ctx,
                                                                            &mut double_arg,
                                                                            *argv.offset(
                                                                                fresh19 as isize,
                                                                            ),
                                                                        ) != 0
                                                                        {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                as
                                                                                                                                *mut DynBuf,
                                                                                                                            fmtbuf.as_mut_ptr(),
                                                                                                                            double_arg);
                                                                        continue;
                                                                    }
                                                                }
                                                                _ => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        if JS_IsString(
                                                                            *argv
                                                                                .offset(i as isize),
                                                                        ) != 0
                                                                        {
                                                                            let fresh14 = i;
                                                                            i = i + 1;
                                                                            string_arg
                                                                                =
                                                                                JS_ToCString(ctx,
                                                                                             *argv.offset(fresh14
                                                                                                              as
                                                                                                              isize));
                                                                            if string_arg.is_null()
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    8304123123824580522;
                                                                                break;
                                                                            }
                                                                            int32_arg
                                                                                =
                                                                                unicode_from_utf8(string_arg
                                                                                                      as
                                                                                                      *mut uint8_t,
                                                                                                  6
                                                                                                      as
                                                                                                      i32,
                                                                                                  &mut p);
                                                                            JS_FreeCString(
                                                                                ctx, string_arg,
                                                                            );
                                                                        } else {
                                                                            let fresh15 = i;
                                                                            i = i + 1;
                                                                            if JS_ToInt32(
                                                                                ctx,
                                                                                &mut int32_arg,
                                                                                *argv.offset(
                                                                                    fresh15
                                                                                        as isize,
                                                                                ),
                                                                            ) != 0
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    8304123123824580522;
                                                                                break;
                                                                            }
                                                                        }
                                                                        if int32_arg as u32
                                                                            > 0x10ffff as i32 as u32
                                                                        {
                                                                            int32_arg =
                                                                                0xfffd as i32
                                                                        }
                                                                        len = unicode_to_utf8(
                                                                            cbuf.as_mut_ptr(),
                                                                            int32_arg as u32,
                                                                        );
                                                                        dbuf_put(
                                                                            &mut dbuf,
                                                                            cbuf.as_mut_ptr(),
                                                                            len as size_t,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        115 => {
                                                            current_block = 3213037896719562574;
                                                            match current_block {
                                                                10165534174818562619 => {
                                                                    dbuf_putc(
                                                                        &mut dbuf,
                                                                        '%' as i32 as uint8_t,
                                                                    );
                                                                    continue;
                                                                }
                                                                2394966745432187107 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh16 = i;
                                                                        i = i + 1;
                                                                        if JS_ToInt64Ext(
                                                                            ctx,
                                                                            &mut int64_arg,
                                                                            *argv.offset(
                                                                                fresh16 as isize,
                                                                            ),
                                                                        ) != 0
                                                                        {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        if mod_0 == 'l' as i32 {
                                                                            if !(q
                                                                                     >=
                                                                                     fmtbuf.as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 32]>()
                                                                                                                    as
                                                                                                                    u64
                                                                                                                    as
                                                                                                                    isize).offset(-(2
                                                                                                                                        as
                                                                                                                                        i32
                                                                                                                                        as
                                                                                                                                        isize)))
                                                                               {
                                                                                *q.offset(1
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    *q.offset(-(1
                                                                                                    as
                                                                                                    i32)
                                                                                                  as
                                                                                                  isize);
                                                                                let ref mut fresh17 =
                                                                                    *q.offset(0
                                                                                                  as
                                                                                                  i32
                                                                                                  as
                                                                                                  isize);
                                                                                *fresh17
                                                                                    =
                                                                                    'l'
                                                                                        as
                                                                                        i32
                                                                                        as
                                                                                        std::os::raw::c_char;
                                                                                *q.offset(-(1
                                                                                                as
                                                                                                i32)
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    *fresh17;
                                                                                *q.offset(2
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    '\u{0}'
                                                                                        as
                                                                                        i32
                                                                                        as
                                                                                        std::os::raw::c_char;
                                                                                dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                        as
                                                                                                                                        *mut DynBuf,
                                                                                                                                    fmtbuf.as_mut_ptr(),
                                                                                                                                    int64_arg
                                                                                                                                        as
                                                                                                                                        i64);
                                                                                continue
                                                                                    ;
                                                                            }
                                                                        } else {
                                                                            dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                    as
                                                                                                                                    *mut DynBuf,
                                                                                                                                fmtbuf.as_mut_ptr(),
                                                                                                                                int64_arg
                                                                                                                                    as
                                                                                                                                    i32);
                                                                            continue;
                                                                        }
                                                                        current_block =
                                                                            11188143500741601598;
                                                                    }
                                                                }
                                                                3213037896719562574 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh18 = i;
                                                                        i = i + 1;
                                                                        string_arg = JS_ToCString(
                                                                            ctx,
                                                                            *argv.offset(
                                                                                fresh18 as isize,
                                                                            ),
                                                                        );
                                                                        if string_arg.is_null() {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                as
                                                                                                                                *mut DynBuf,
                                                                                                                            fmtbuf.as_mut_ptr(),
                                                                                                                            string_arg);
                                                                        JS_FreeCString(
                                                                            ctx, string_arg,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                                1348390045057916302 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh19 = i;
                                                                        i = i + 1;
                                                                        if JS_ToFloat64(
                                                                            ctx,
                                                                            &mut double_arg,
                                                                            *argv.offset(
                                                                                fresh19 as isize,
                                                                            ),
                                                                        ) != 0
                                                                        {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                as
                                                                                                                                *mut DynBuf,
                                                                                                                            fmtbuf.as_mut_ptr(),
                                                                                                                            double_arg);
                                                                        continue;
                                                                    }
                                                                }
                                                                _ => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        if JS_IsString(
                                                                            *argv
                                                                                .offset(i as isize),
                                                                        ) != 0
                                                                        {
                                                                            let fresh14 = i;
                                                                            i = i + 1;
                                                                            string_arg
                                                                                =
                                                                                JS_ToCString(ctx,
                                                                                             *argv.offset(fresh14
                                                                                                              as
                                                                                                              isize));
                                                                            if string_arg.is_null()
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    8304123123824580522;
                                                                                break;
                                                                            }
                                                                            int32_arg
                                                                                =
                                                                                unicode_from_utf8(string_arg
                                                                                                      as
                                                                                                      *mut uint8_t,
                                                                                                  6
                                                                                                      as
                                                                                                      i32,
                                                                                                  &mut p);
                                                                            JS_FreeCString(
                                                                                ctx, string_arg,
                                                                            );
                                                                        } else {
                                                                            let fresh15 = i;
                                                                            i = i + 1;
                                                                            if JS_ToInt32(
                                                                                ctx,
                                                                                &mut int32_arg,
                                                                                *argv.offset(
                                                                                    fresh15
                                                                                        as isize,
                                                                                ),
                                                                            ) != 0
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    8304123123824580522;
                                                                                break;
                                                                            }
                                                                        }
                                                                        if int32_arg as u32
                                                                            > 0x10ffff as i32 as u32
                                                                        {
                                                                            int32_arg =
                                                                                0xfffd as i32
                                                                        }
                                                                        len = unicode_to_utf8(
                                                                            cbuf.as_mut_ptr(),
                                                                            int32_arg as u32,
                                                                        );
                                                                        dbuf_put(
                                                                            &mut dbuf,
                                                                            cbuf.as_mut_ptr(),
                                                                            len as size_t,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        101 | 102 | 103 | 97 | 69 | 70 | 71
                                                        | 65 => {
                                                            current_block = 1348390045057916302;
                                                            match current_block {
                                                                10165534174818562619 => {
                                                                    dbuf_putc(
                                                                        &mut dbuf,
                                                                        '%' as i32 as uint8_t,
                                                                    );
                                                                    continue;
                                                                }
                                                                2394966745432187107 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh16 = i;
                                                                        i = i + 1;
                                                                        if JS_ToInt64Ext(
                                                                            ctx,
                                                                            &mut int64_arg,
                                                                            *argv.offset(
                                                                                fresh16 as isize,
                                                                            ),
                                                                        ) != 0
                                                                        {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        if mod_0 == 'l' as i32 {
                                                                            if !(q
                                                                                     >=
                                                                                     fmtbuf.as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 32]>()
                                                                                                                    as
                                                                                                                    u64
                                                                                                                    as
                                                                                                                    isize).offset(-(2
                                                                                                                                        as
                                                                                                                                        i32
                                                                                                                                        as
                                                                                                                                        isize)))
                                                                               {
                                                                                *q.offset(1
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    *q.offset(-(1
                                                                                                    as
                                                                                                    i32)
                                                                                                  as
                                                                                                  isize);
                                                                                let ref mut fresh17 =
                                                                                    *q.offset(0
                                                                                                  as
                                                                                                  i32
                                                                                                  as
                                                                                                  isize);
                                                                                *fresh17
                                                                                    =
                                                                                    'l'
                                                                                        as
                                                                                        i32
                                                                                        as
                                                                                        std::os::raw::c_char;
                                                                                *q.offset(-(1
                                                                                                as
                                                                                                i32)
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    *fresh17;
                                                                                *q.offset(2
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    '\u{0}'
                                                                                        as
                                                                                        i32
                                                                                        as
                                                                                        std::os::raw::c_char;
                                                                                dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                        as
                                                                                                                                        *mut DynBuf,
                                                                                                                                    fmtbuf.as_mut_ptr(),
                                                                                                                                    int64_arg
                                                                                                                                        as
                                                                                                                                        i64);
                                                                                continue
                                                                                    ;
                                                                            }
                                                                        } else {
                                                                            dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                    as
                                                                                                                                    *mut DynBuf,
                                                                                                                                fmtbuf.as_mut_ptr(),
                                                                                                                                int64_arg
                                                                                                                                    as
                                                                                                                                    i32);
                                                                            continue;
                                                                        }
                                                                        current_block =
                                                                            11188143500741601598;
                                                                    }
                                                                }
                                                                3213037896719562574 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh18 = i;
                                                                        i = i + 1;
                                                                        string_arg = JS_ToCString(
                                                                            ctx,
                                                                            *argv.offset(
                                                                                fresh18 as isize,
                                                                            ),
                                                                        );
                                                                        if string_arg.is_null() {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                as
                                                                                                                                *mut DynBuf,
                                                                                                                            fmtbuf.as_mut_ptr(),
                                                                                                                            string_arg);
                                                                        JS_FreeCString(
                                                                            ctx, string_arg,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                                1348390045057916302 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh19 = i;
                                                                        i = i + 1;
                                                                        if JS_ToFloat64(
                                                                            ctx,
                                                                            &mut double_arg,
                                                                            *argv.offset(
                                                                                fresh19 as isize,
                                                                            ),
                                                                        ) != 0
                                                                        {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                as
                                                                                                                                *mut DynBuf,
                                                                                                                            fmtbuf.as_mut_ptr(),
                                                                                                                            double_arg);
                                                                        continue;
                                                                    }
                                                                }
                                                                _ => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        if JS_IsString(
                                                                            *argv
                                                                                .offset(i as isize),
                                                                        ) != 0
                                                                        {
                                                                            let fresh14 = i;
                                                                            i = i + 1;
                                                                            string_arg
                                                                                =
                                                                                JS_ToCString(ctx,
                                                                                             *argv.offset(fresh14
                                                                                                              as
                                                                                                              isize));
                                                                            if string_arg.is_null()
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    8304123123824580522;
                                                                                break;
                                                                            }
                                                                            int32_arg
                                                                                =
                                                                                unicode_from_utf8(string_arg
                                                                                                      as
                                                                                                      *mut uint8_t,
                                                                                                  6
                                                                                                      as
                                                                                                      i32,
                                                                                                  &mut p);
                                                                            JS_FreeCString(
                                                                                ctx, string_arg,
                                                                            );
                                                                        } else {
                                                                            let fresh15 = i;
                                                                            i = i + 1;
                                                                            if JS_ToInt32(
                                                                                ctx,
                                                                                &mut int32_arg,
                                                                                *argv.offset(
                                                                                    fresh15
                                                                                        as isize,
                                                                                ),
                                                                            ) != 0
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    8304123123824580522;
                                                                                break;
                                                                            }
                                                                        }
                                                                        if int32_arg as u32
                                                                            > 0x10ffff as i32 as u32
                                                                        {
                                                                            int32_arg =
                                                                                0xfffd as i32
                                                                        }
                                                                        len = unicode_to_utf8(
                                                                            cbuf.as_mut_ptr(),
                                                                            int32_arg as u32,
                                                                        );
                                                                        dbuf_put(
                                                                            &mut dbuf,
                                                                            cbuf.as_mut_ptr(),
                                                                            len as size_t,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        37 => {
                                                            current_block = 10165534174818562619;
                                                            match current_block {
                                                                10165534174818562619 => {
                                                                    dbuf_putc(
                                                                        &mut dbuf,
                                                                        '%' as i32 as uint8_t,
                                                                    );
                                                                    continue;
                                                                }
                                                                2394966745432187107 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh16 = i;
                                                                        i = i + 1;
                                                                        if JS_ToInt64Ext(
                                                                            ctx,
                                                                            &mut int64_arg,
                                                                            *argv.offset(
                                                                                fresh16 as isize,
                                                                            ),
                                                                        ) != 0
                                                                        {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        if mod_0 == 'l' as i32 {
                                                                            if !(q
                                                                                     >=
                                                                                     fmtbuf.as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 32]>()
                                                                                                                    as
                                                                                                                    u64
                                                                                                                    as
                                                                                                                    isize).offset(-(2
                                                                                                                                        as
                                                                                                                                        i32
                                                                                                                                        as
                                                                                                                                        isize)))
                                                                               {
                                                                                *q.offset(1
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    *q.offset(-(1
                                                                                                    as
                                                                                                    i32)
                                                                                                  as
                                                                                                  isize);
                                                                                let ref mut fresh17 =
                                                                                    *q.offset(0
                                                                                                  as
                                                                                                  i32
                                                                                                  as
                                                                                                  isize);
                                                                                *fresh17
                                                                                    =
                                                                                    'l'
                                                                                        as
                                                                                        i32
                                                                                        as
                                                                                        std::os::raw::c_char;
                                                                                *q.offset(-(1
                                                                                                as
                                                                                                i32)
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    *fresh17;
                                                                                *q.offset(2
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              isize)
                                                                                    =
                                                                                    '\u{0}'
                                                                                        as
                                                                                        i32
                                                                                        as
                                                                                        std::os::raw::c_char;
                                                                                dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                        as
                                                                                                                                        *mut DynBuf,
                                                                                                                                    fmtbuf.as_mut_ptr(),
                                                                                                                                    int64_arg
                                                                                                                                        as
                                                                                                                                        i64);
                                                                                continue
                                                                                    ;
                                                                            }
                                                                        } else {
                                                                            dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                    as
                                                                                                                                    *mut DynBuf,
                                                                                                                                fmtbuf.as_mut_ptr(),
                                                                                                                                int64_arg
                                                                                                                                    as
                                                                                                                                    i32);
                                                                            continue;
                                                                        }
                                                                        current_block =
                                                                            11188143500741601598;
                                                                    }
                                                                }
                                                                3213037896719562574 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh18 = i;
                                                                        i = i + 1;
                                                                        string_arg = JS_ToCString(
                                                                            ctx,
                                                                            *argv.offset(
                                                                                fresh18 as isize,
                                                                            ),
                                                                        );
                                                                        if string_arg.is_null() {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                as
                                                                                                                                *mut DynBuf,
                                                                                                                            fmtbuf.as_mut_ptr(),
                                                                                                                            string_arg);
                                                                        JS_FreeCString(
                                                                            ctx, string_arg,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                                1348390045057916302 => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        let fresh19 = i;
                                                                        i = i + 1;
                                                                        if JS_ToFloat64(
                                                                            ctx,
                                                                            &mut double_arg,
                                                                            *argv.offset(
                                                                                fresh19 as isize,
                                                                            ),
                                                                        ) != 0
                                                                        {
                                                                            current_block =
                                                                                8304123123824580522;
                                                                            break;
                                                                        }
                                                                        dbuf_printf_fun.expect("non-null function pointer")(&mut dbuf
                                                                                                                                as
                                                                                                                                *mut DynBuf,
                                                                                                                            fmtbuf.as_mut_ptr(),
                                                                                                                            double_arg);
                                                                        continue;
                                                                    }
                                                                }
                                                                _ => {
                                                                    if i >= argc {
                                                                        current_block =
                                                                            10409069600589371798;
                                                                    } else {
                                                                        if JS_IsString(
                                                                            *argv
                                                                                .offset(i as isize),
                                                                        ) != 0
                                                                        {
                                                                            let fresh14 = i;
                                                                            i = i + 1;
                                                                            string_arg
                                                                                =
                                                                                JS_ToCString(ctx,
                                                                                             *argv.offset(fresh14
                                                                                                              as
                                                                                                              isize));
                                                                            if string_arg.is_null()
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    8304123123824580522;
                                                                                break;
                                                                            }
                                                                            int32_arg
                                                                                =
                                                                                unicode_from_utf8(string_arg
                                                                                                      as
                                                                                                      *mut uint8_t,
                                                                                                  6
                                                                                                      as
                                                                                                      i32,
                                                                                                  &mut p);
                                                                            JS_FreeCString(
                                                                                ctx, string_arg,
                                                                            );
                                                                        } else {
                                                                            let fresh15 = i;
                                                                            i = i + 1;
                                                                            if JS_ToInt32(
                                                                                ctx,
                                                                                &mut int32_arg,
                                                                                *argv.offset(
                                                                                    fresh15
                                                                                        as isize,
                                                                                ),
                                                                            ) != 0
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    8304123123824580522;
                                                                                break;
                                                                            }
                                                                        }
                                                                        if int32_arg as u32
                                                                            > 0x10ffff as i32 as u32
                                                                        {
                                                                            int32_arg =
                                                                                0xfffd as i32
                                                                        }
                                                                        len = unicode_to_utf8(
                                                                            cbuf.as_mut_ptr(),
                                                                            int32_arg as u32,
                                                                        );
                                                                        dbuf_put(
                                                                            &mut dbuf,
                                                                            cbuf.as_mut_ptr(),
                                                                            len as size_t,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            current_block = 11188143500741601598;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    11188143500741601598 => {}
                                    _ => {
                                        JS_ThrowReferenceError(
                                            ctx,
                                            b"missing argument for conversion specifier\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                        current_block = 8304123123824580522;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                /* XXX: should support an extension mechanism */
                JS_ThrowTypeError(
                    ctx,
                    b"invalid conversion specifier in format string\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
                current_block = 8304123123824580522;
                break;
            }
            match current_block {
                8304123123824580522 => {}
                _ => {
                    JS_FreeCString(ctx, fmt_str);
                    current_block = 16593409533420678784;
                }
            }
        }
        match current_block {
            16593409533420678784 => {}
            _ => {
                dbuf_free(&mut dbuf);
                return {
                    let mut init = JSValue {
                        u: JSValueUnion { int32: 0 as i32 },
                        tag: JS_TAG_EXCEPTION as i32 as int64_t,
                    };
                    init
                };
            }
        }
    }
    if dbuf.error != 0 {
        res = JS_ThrowOutOfMemory(ctx)
    } else if !fp.is_null() {
        len = fwrite(
            dbuf.buf as *const std::ffi::c_void,
            1 as i32 as u64,
            dbuf.size,
            fp,
        ) as i32;
        res = JS_NewInt32(ctx, len)
    } else {
        res = JS_NewStringLen(ctx, dbuf.buf as *mut std::os::raw::c_char, dbuf.size)
    }
    dbuf_free(&mut dbuf);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn js_load_file(
    mut ctx: *mut JSContext,
    mut pbuf_len: *mut size_t,
    mut filename: *const std::os::raw::c_char,
) -> *mut uint8_t {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    let mut buf_len: size_t = 0;
    let mut lret: i64 = 0;
    f = fopen(
        filename,
        b"rb\x00" as *const u8 as *const std::os::raw::c_char,
    );
    if f.is_null() {
        return 0 as *mut uint8_t;
    }
    if !(fseek(f, 0 as i32 as i64, 2 as i32) < 0 as i32) {
        lret = ftell(f);
        if !(lret < 0 as i32 as i64) {
            /* XXX: on Linux, ftell() return LONG_MAX for directories */
            if lret == 9223372036854775807 as i64 {
                *__errno_location() = 21 as i32
            } else {
                buf_len = lret as size_t;
                if !(fseek(f, 0 as i32 as i64, 0 as i32) < 0 as i32) {
                    if !ctx.is_null() {
                        buf = js_malloc(ctx, buf_len.wrapping_add(1 as i32 as u64)) as *mut uint8_t
                    } else {
                        buf = malloc(buf_len.wrapping_add(1 as i32 as u64)) as *mut uint8_t
                    }
                    if !buf.is_null() {
                        if fread(buf as *mut std::ffi::c_void, 1 as i32 as u64, buf_len, f)
                            != buf_len
                        {
                            *__errno_location() = 5 as i32;
                            if !ctx.is_null() {
                                js_free(ctx, buf as *mut std::ffi::c_void);
                            } else {
                                free(buf as *mut std::ffi::c_void);
                            }
                        } else {
                            *buf.offset(buf_len as isize) = '\u{0}' as i32 as uint8_t;
                            fclose(f);
                            *pbuf_len = buf_len;
                            return buf;
                        }
                    }
                }
            }
        }
    }
    fclose(f);
    return 0 as *mut uint8_t;
}
/* load and evaluate a file */
unsafe extern "C" fn js_loadScript(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    let mut filename: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut buf_len: size_t = 0;
    filename = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if filename.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    buf = js_load_file(ctx, &mut buf_len, filename);
    if buf.is_null() {
        JS_ThrowReferenceError(
            ctx,
            b"could not load \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
            filename,
        );
        JS_FreeCString(ctx, filename);
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = JS_Eval(
        ctx,
        buf as *mut std::os::raw::c_char,
        buf_len,
        filename,
        (0 as i32) << 0 as i32,
    );
    js_free(ctx, buf as *mut std::ffi::c_void);
    JS_FreeCString(ctx, filename);
    return ret;
}
/* load a file as a UTF-8 encoded string */
unsafe extern "C" fn js_std_loadFile(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    let mut filename: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut buf_len: size_t = 0;
    filename = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if filename.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    buf = js_load_file(ctx, &mut buf_len, filename);
    JS_FreeCString(ctx, filename);
    if buf.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_NULL as i32 as int64_t,
            };
            init
        };
    }
    ret = JS_NewStringLen(ctx, buf as *mut std::os::raw::c_char, buf_len);
    js_free(ctx, buf as *mut std::ffi::c_void);
    return ret;
}
unsafe extern "C" fn js_module_loader_so(
    mut ctx: *mut JSContext,
    mut module_name: *const std::os::raw::c_char,
) -> *mut JSModuleDef {
    let mut m: *mut JSModuleDef = 0 as *mut JSModuleDef;
    let mut hd: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut init: Option<JSInitModuleFunc> = None;
    let mut filename: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    if strchr(module_name, '/' as i32).is_null() {
        /* must add a '/' so that the DLL is not searched in the
        system library paths */
        filename = js_malloc(
            ctx,
            strlen(module_name)
                .wrapping_add(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
        ) as *mut std::os::raw::c_char;
        if filename.is_null() {
            return 0 as *mut JSModuleDef;
        }
        strcpy(
            filename,
            b"./\x00" as *const u8 as *const std::os::raw::c_char,
        );
        strcpy(filename.offset(2 as i32 as isize), module_name);
    } else {
        filename = module_name as *mut std::os::raw::c_char
    }
    /* C module */
    hd = dlopen(filename, 0x2 as i32 | 0 as i32);
    if filename != module_name as *mut std::os::raw::c_char {
        js_free(ctx, filename as *mut std::ffi::c_void);
    }
    if hd.is_null() {
        JS_ThrowReferenceError(
            ctx,
            b"could not load module filename \'%s\' as shared library\x00" as *const u8
                as *const std::os::raw::c_char,
            module_name,
        );
    } else {
        init = ::std::mem::transmute::<*mut std::ffi::c_void, Option<JSInitModuleFunc>>(dlsym(
            hd,
            b"js_init_module\x00" as *const u8 as *const std::os::raw::c_char,
        ));
        if init.is_none() {
            JS_ThrowReferenceError(
                ctx,
                b"could not load module filename \'%s\': js_init_module not found\x00" as *const u8
                    as *const std::os::raw::c_char,
                module_name,
            );
        } else {
            m = init.expect("non-null function pointer")(ctx, module_name);
            if m.is_null() {
                JS_ThrowReferenceError(
                    ctx,
                    b"could not load module filename \'%s\': initialization error\x00" as *const u8
                        as *const std::os::raw::c_char,
                    module_name,
                );
            } else {
                return m;
            }
        }
    }
    if !hd.is_null() {
        dlclose(hd);
    }
    return 0 as *mut JSModuleDef;
}
/* !_WIN32 */
#[no_mangle]
pub unsafe extern "C" fn js_module_set_import_meta(
    mut ctx: *mut JSContext,
    mut func_val: JSValue,
    mut use_realpath: i32,
    mut is_main: i32,
) -> i32 {
    let mut m: *mut JSModuleDef = 0 as *mut JSModuleDef;
    let mut buf: [std::os::raw::c_char; 4112] = [0; 4112];
    let mut meta_obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut module_name_atom: JSAtom = 0;
    let mut module_name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if func_val.tag as int32_t == JS_TAG_MODULE as i32 {
    } else {
        __assert_fail(
            b"JS_VALUE_GET_TAG(func_val) == JS_TAG_MODULE\x00" as *const u8
                as *const std::os::raw::c_char,
            b"quickjs-libc.c\x00" as *const u8 as *const std::os::raw::c_char,
            523 as i32 as u32,
            (*::std::mem::transmute::<&[u8; 62], &[std::os::raw::c_char; 62]>(
                b"int js_module_set_import_meta(JSContext *, JSValue, int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    m = func_val.u.ptr as *mut JSModuleDef;
    module_name_atom = JS_GetModuleName(ctx, m);
    module_name = JS_AtomToCString(ctx, module_name_atom);
    JS_FreeAtom(ctx, module_name_atom);
    if module_name.is_null() {
        return -(1 as i32);
    }
    if strchr(module_name, ':' as i32).is_null() {
        strcpy(
            buf.as_mut_ptr(),
            b"file://\x00" as *const u8 as *const std::os::raw::c_char,
        );
        /* realpath() cannot be used with modules compiled with qjsc
        because the corresponding module source code is not
        necessarily present */
        if use_realpath != 0 {
            let mut res: *mut std::os::raw::c_char = realpath(
                module_name,
                buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize),
            );
            if res.is_null() {
                JS_ThrowTypeError(
                    ctx,
                    b"realpath failure\x00" as *const u8 as *const std::os::raw::c_char,
                );
                JS_FreeCString(ctx, module_name);
                return -(1 as i32);
            }
        } else {
            pstrcat(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[std::os::raw::c_char; 4112]>() as u64 as i32,
                module_name,
            );
        }
    } else {
        pstrcpy(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 4112]>() as u64 as i32,
            module_name,
        );
    }
    JS_FreeCString(ctx, module_name);
    meta_obj = JS_GetImportMeta(ctx, m);
    if JS_IsException(meta_obj) != 0 {
        return -(1 as i32);
    }
    JS_DefinePropertyValueStr(
        ctx,
        meta_obj,
        b"url\x00" as *const u8 as *const std::os::raw::c_char,
        JS_NewString(ctx, buf.as_mut_ptr()),
        (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
    );
    JS_DefinePropertyValueStr(
        ctx,
        meta_obj,
        b"main\x00" as *const u8 as *const std::os::raw::c_char,
        JS_NewBool(ctx, is_main),
        (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
    );
    JS_FreeValue(ctx, meta_obj);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn js_module_loader(
    mut ctx: *mut JSContext,
    mut module_name: *const std::os::raw::c_char,
    mut opaque: *mut std::ffi::c_void,
) -> *mut JSModuleDef {
    let mut m: *mut JSModuleDef = 0 as *mut JSModuleDef;
    if has_suffix(
        module_name,
        b".so\x00" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        m = js_module_loader_so(ctx, module_name)
    } else {
        let mut buf_len: size_t = 0;
        let mut buf: *mut uint8_t = 0 as *mut uint8_t;
        let mut func_val: JSValue = JSValue {
            u: JSValueUnion { int32: 0 },
            tag: 0,
        };
        buf = js_load_file(ctx, &mut buf_len, module_name);
        if buf.is_null() {
            JS_ThrowReferenceError(
                ctx,
                b"could not load module filename \'%s\'\x00" as *const u8
                    as *const std::os::raw::c_char,
                module_name,
            );
            return 0 as *mut JSModuleDef;
        }
        /* compile the module */
        func_val = JS_Eval(
            ctx,
            buf as *mut std::os::raw::c_char,
            buf_len,
            module_name,
            (1 as i32) << 0 as i32 | (1 as i32) << 5 as i32,
        );
        js_free(ctx, buf as *mut std::ffi::c_void);
        if JS_IsException(func_val) != 0 {
            return 0 as *mut JSModuleDef;
        }
        /* XXX: could propagate the exception */
        js_module_set_import_meta(ctx, func_val, TRUE as i32, FALSE as i32);
        /* the module is already referenced, so we must free it */
        m = func_val.u.ptr as *mut JSModuleDef;
        JS_FreeValue(ctx, func_val);
    }
    return m;
}
unsafe extern "C" fn js_std_exit(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut status: i32 = 0;
    if JS_ToInt32(ctx, &mut status, *argv.offset(0 as i32 as isize)) != 0 {
        status = -(1 as i32)
    }
    exit(status);
}
unsafe extern "C" fn js_std_getenv(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    name = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if name.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    str = getenv(name);
    JS_FreeCString(ctx, name);
    if str.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_UNDEFINED as i32 as int64_t,
            };
            init
        };
    } else {
        return JS_NewString(ctx, str);
    };
}
/* _WIN32 */
unsafe extern "C" fn js_std_setenv(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut value: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    name = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if name.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    value = JS_ToCString(ctx, *argv.offset(1 as i32 as isize));
    if value.is_null() {
        JS_FreeCString(ctx, name);
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    setenv(name, value, TRUE as i32);
    JS_FreeCString(ctx, name);
    JS_FreeCString(ctx, value);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn js_std_unsetenv(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    name = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if name.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    unsetenv(name);
    JS_FreeCString(ctx, name);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
/* return an object containing the list of the available environment
variables. */
unsafe extern "C" fn js_std_getenviron(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut current_block: u64;
    let mut envp: *mut *mut std::os::raw::c_char = 0 as *mut *mut std::os::raw::c_char;
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut value: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut idx: uint32_t = 0;
    let mut name_len: size_t = 0;
    let mut atom: JSAtom = 0;
    let mut ret: i32 = 0;
    obj = JS_NewObject(ctx);
    if JS_IsException(obj) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    envp = environ;
    idx = 0 as i32 as uint32_t;
    loop {
        if (*envp.offset(idx as isize)).is_null() {
            current_block = 13242334135786603907;
            break;
        }
        name = *envp.offset(idx as isize);
        p = strchr(name, '=' as i32);
        name_len = p.wrapping_offset_from(name) as i64 as size_t;
        if !p.is_null() {
            value = p.offset(1 as i32 as isize);
            atom = JS_NewAtomLen(ctx, name, name_len);
            if atom == 0 as i32 as u32 {
                current_block = 13771861275519314594;
                break;
            }
            ret = JS_DefinePropertyValue(
                ctx,
                obj,
                atom,
                JS_NewString(ctx, value),
                (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
            );
            JS_FreeAtom(ctx, atom);
            if ret < 0 as i32 {
                current_block = 13771861275519314594;
                break;
            }
        }
        idx = idx.wrapping_add(1)
    }
    match current_block {
        13242334135786603907 => return obj,
        _ => {
            JS_FreeValue(ctx, obj);
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            };
        }
    };
}
unsafe extern "C" fn js_std_gc(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    JS_RunGC(JS_GetRuntime(ctx));
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn interrupt_handler(
    mut rt: *mut JSRuntime,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    return (os_pending_signals >> 2 as i32 & 1 as i32 as u64) as i32;
}
unsafe extern "C" fn get_bool_option(
    mut ctx: *mut JSContext,
    mut pbool: *mut BOOL,
    mut obj: JSValue,
    mut option: *const std::os::raw::c_char,
) -> i32 {
    let mut val: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    val = JS_GetPropertyStr(ctx, obj, option);
    if JS_IsException(val) != 0 {
        return -(1 as i32);
    }
    if JS_IsUndefined(val) == 0 {
        *pbool = JS_ToBool(ctx, val)
    }
    JS_FreeValue(ctx, val);
    return 0 as i32;
}
unsafe extern "C" fn js_evalScript(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut rt: *mut JSRuntime = JS_GetRuntime(ctx);
    let mut ts: *mut JSThreadState = JS_GetRuntimeOpaque(rt) as *mut JSThreadState;
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut len: size_t = 0;
    let mut ret: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut options_obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut backtrace_barrier: BOOL = FALSE as i32;
    let mut flags: i32 = 0;
    if argc >= 2 as i32 {
        options_obj = *argv.offset(1 as i32 as isize);
        if get_bool_option(
            ctx,
            &mut backtrace_barrier,
            options_obj,
            b"backtrace_barrier\x00" as *const u8 as *const std::os::raw::c_char,
        ) != 0
        {
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            };
        }
    }
    str = JS_ToCStringLen(ctx, &mut len, *argv.offset(0 as i32 as isize));
    if str.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if (*ts).recv_pipe.is_null() && {
        (*ts).eval_script_recurse += 1;
        ((*ts).eval_script_recurse) == 1 as i32
    } {
        /* install the interrupt handler */
        JS_SetInterruptHandler(
            JS_GetRuntime(ctx),
            Some(
                interrupt_handler
                    as unsafe extern "C" fn(_: *mut JSRuntime, _: *mut std::ffi::c_void) -> i32,
            ),
            0 as *mut std::ffi::c_void,
        );
    }
    flags = (0 as i32) << 0 as i32;
    if backtrace_barrier != 0 {
        flags |= (1 as i32) << 6 as i32
    }
    ret = JS_Eval(
        ctx,
        str,
        len,
        b"<evalScript>\x00" as *const u8 as *const std::os::raw::c_char,
        flags,
    );
    JS_FreeCString(ctx, str);
    if (*ts).recv_pipe.is_null() && {
        (*ts).eval_script_recurse -= 1;
        ((*ts).eval_script_recurse) == 0 as i32
    } {
        /* remove the interrupt handler */
        JS_SetInterruptHandler(JS_GetRuntime(ctx), None, 0 as *mut std::ffi::c_void);
        os_pending_signals &= !((1 as i32 as uint64_t) << 2 as i32);
        /* convert the uncatchable "interrupted" error into a normal error
        so that it can be caught by the REPL */
        if JS_IsException(ret) != 0 {
            JS_ResetUncatchableError(ctx);
        }
    }
    return ret;
}
static mut js_std_file_class_id: JSClassID = 0;
unsafe extern "C" fn js_std_file_finalizer(mut rt: *mut JSRuntime, mut val: JSValue) {
    let mut s: *mut JSSTDFile = JS_GetOpaque(val, js_std_file_class_id) as *mut JSSTDFile;
    if !s.is_null() {
        if !(*s).f.is_null() && (*s).close_in_finalizer != 0 {
            if (*s).is_popen != 0 {
                pclose((*s).f);
            } else {
                fclose((*s).f);
            }
        }
        js_free_rt(rt, s as *mut std::ffi::c_void);
    };
}
unsafe extern "C" fn js_get_errno(mut ret: ssize_t) -> ssize_t {
    if ret == -(1 as i32) as i64 {
        ret = -*__errno_location() as ssize_t
    }
    return ret;
}
unsafe extern "C" fn js_std_strerror(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut err: i32 = 0;
    if JS_ToInt32(ctx, &mut err, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    return JS_NewString(ctx, strerror(err));
}
unsafe extern "C" fn js_std_parseExtJSON(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut len: size_t = 0;
    str = JS_ToCStringLen(ctx, &mut len, *argv.offset(0 as i32 as isize));
    if str.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    obj = JS_ParseJSON2(
        ctx,
        str,
        len,
        b"<input>\x00" as *const u8 as *const std::os::raw::c_char,
        (1 as i32) << 0 as i32,
    );
    JS_FreeCString(ctx, str);
    return obj;
}
unsafe extern "C" fn js_new_std_file(
    mut ctx: *mut JSContext,
    mut f: *mut FILE,
    mut close_in_finalizer: BOOL,
    mut is_popen: BOOL,
) -> JSValue {
    let mut s: *mut JSSTDFile = 0 as *mut JSSTDFile;
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    obj = JS_NewObjectClass(ctx, js_std_file_class_id as i32);
    if JS_IsException(obj) != 0 {
        return obj;
    }
    s = js_mallocz(ctx, ::std::mem::size_of::<JSSTDFile>() as u64) as *mut JSSTDFile;
    if s.is_null() {
        JS_FreeValue(ctx, obj);
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    (*s).close_in_finalizer = close_in_finalizer;
    (*s).is_popen = is_popen;
    (*s).f = f;
    JS_SetOpaque(obj, s as *mut std::ffi::c_void);
    return obj;
}
unsafe extern "C" fn js_set_error_object(mut ctx: *mut JSContext, mut obj: JSValue, mut err: i32) {
    if JS_IsUndefined(obj) == 0 {
        JS_SetPropertyStr(
            ctx,
            obj,
            b"errno\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt32(ctx, err),
        );
    };
}
unsafe extern "C" fn js_std_open(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut filename: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut mode: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut err: i32 = 0;
    filename = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if !filename.is_null() {
        mode = JS_ToCString(ctx, *argv.offset(1 as i32 as isize));
        if !mode.is_null() {
            if *mode.offset(strspn(
                mode,
                b"rwa+b\x00" as *const u8 as *const std::os::raw::c_char,
            ) as isize) as i32
                != '\u{0}' as i32
            {
                JS_ThrowTypeError(
                    ctx,
                    b"invalid file mode\x00" as *const u8 as *const std::os::raw::c_char,
                );
            } else {
                f = fopen(filename, mode);
                if f.is_null() {
                    err = *__errno_location()
                } else {
                    err = 0 as i32
                }
                if argc >= 3 as i32 {
                    js_set_error_object(ctx, *argv.offset(2 as i32 as isize), err);
                }
                JS_FreeCString(ctx, filename);
                JS_FreeCString(ctx, mode);
                if f.is_null() {
                    return {
                        let mut init = JSValue {
                            u: JSValueUnion { int32: 0 as i32 },
                            tag: JS_TAG_NULL as i32 as int64_t,
                        };
                        init
                    };
                }
                return js_new_std_file(ctx, f, TRUE as i32, FALSE as i32);
            }
        }
    }
    JS_FreeCString(ctx, filename);
    JS_FreeCString(ctx, mode);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_EXCEPTION as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn js_std_popen(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut filename: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut mode: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut err: i32 = 0;
    filename = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if !filename.is_null() {
        mode = JS_ToCString(ctx, *argv.offset(1 as i32 as isize));
        if !mode.is_null() {
            if *mode.offset(
                strspn(mode, b"rw\x00" as *const u8 as *const std::os::raw::c_char) as isize,
            ) as i32
                != '\u{0}' as i32
            {
                JS_ThrowTypeError(
                    ctx,
                    b"invalid file mode\x00" as *const u8 as *const std::os::raw::c_char,
                );
            } else {
                f = popen(filename, mode);
                if f.is_null() {
                    err = *__errno_location()
                } else {
                    err = 0 as i32
                }
                if argc >= 3 as i32 {
                    js_set_error_object(ctx, *argv.offset(2 as i32 as isize), err);
                }
                JS_FreeCString(ctx, filename);
                JS_FreeCString(ctx, mode);
                if f.is_null() {
                    return {
                        let mut init = JSValue {
                            u: JSValueUnion { int32: 0 as i32 },
                            tag: JS_TAG_NULL as i32 as int64_t,
                        };
                        init
                    };
                }
                return js_new_std_file(ctx, f, TRUE as i32, TRUE as i32);
            }
        }
    }
    JS_FreeCString(ctx, filename);
    JS_FreeCString(ctx, mode);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_EXCEPTION as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn js_std_fdopen(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut mode: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut fd: i32 = 0;
    let mut err: i32 = 0;
    if JS_ToInt32(ctx, &mut fd, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    mode = JS_ToCString(ctx, *argv.offset(1 as i32 as isize));
    if !mode.is_null() {
        if *mode.offset(strspn(
            mode,
            b"rwa+\x00" as *const u8 as *const std::os::raw::c_char,
        ) as isize) as i32
            != '\u{0}' as i32
        {
            JS_ThrowTypeError(
                ctx,
                b"invalid file mode\x00" as *const u8 as *const std::os::raw::c_char,
            );
        } else {
            f = fdopen(fd, mode);
            if f.is_null() {
                err = *__errno_location()
            } else {
                err = 0 as i32
            }
            if argc >= 3 as i32 {
                js_set_error_object(ctx, *argv.offset(2 as i32 as isize), err);
            }
            JS_FreeCString(ctx, mode);
            if f.is_null() {
                return {
                    let mut init = JSValue {
                        u: JSValueUnion { int32: 0 as i32 },
                        tag: JS_TAG_NULL as i32 as int64_t,
                    };
                    init
                };
            }
            return js_new_std_file(ctx, f, TRUE as i32, FALSE as i32);
        }
    }
    JS_FreeCString(ctx, mode);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_EXCEPTION as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn js_std_tmpfile(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = 0 as *mut FILE;
    f = tmpfile();
    if argc >= 1 as i32 {
        js_set_error_object(
            ctx,
            *argv.offset(0 as i32 as isize),
            if !f.is_null() {
                0 as i32
            } else {
                *__errno_location()
            },
        );
    }
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_NULL as i32 as int64_t,
            };
            init
        };
    }
    return js_new_std_file(ctx, f, TRUE as i32, FALSE as i32);
}
unsafe extern "C" fn js_std_sprintf(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    return js_printf_internal(ctx, argc, argv, 0 as *mut FILE);
}
unsafe extern "C" fn js_std_printf(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    return js_printf_internal(ctx, argc, argv, stdout);
}
unsafe extern "C" fn js_std_file_get(mut ctx: *mut JSContext, mut obj: JSValue) -> *mut FILE {
    let mut s: *mut JSSTDFile = JS_GetOpaque2(ctx, obj, js_std_file_class_id) as *mut JSSTDFile;
    if s.is_null() {
        return 0 as *mut FILE;
    }
    if (*s).f.is_null() {
        JS_ThrowTypeError(
            ctx,
            b"invalid file handle\x00" as *const u8 as *const std::os::raw::c_char,
        );
        return 0 as *mut FILE;
    }
    return (*s).f;
}
unsafe extern "C" fn js_std_file_puts(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
    mut magic: i32,
) -> JSValue {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: i32 = 0;
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut len: size_t = 0;
    if magic == 0 as i32 {
        f = stdout
    } else {
        f = js_std_file_get(ctx, this_val);
        if f.is_null() {
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            };
        }
    }
    i = 0 as i32;
    while i < argc {
        str = JS_ToCStringLen(ctx, &mut len, *argv.offset(i as isize));
        if str.is_null() {
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            };
        }
        fwrite(str as *const std::ffi::c_void, 1 as i32 as u64, len, f);
        JS_FreeCString(ctx, str);
        i += 1
    }
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn js_std_file_close(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut s: *mut JSSTDFile =
        JS_GetOpaque2(ctx, this_val, js_std_file_class_id) as *mut JSSTDFile;
    let mut err: i32 = 0;
    if s.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if (*s).f.is_null() {
        return JS_ThrowTypeError(
            ctx,
            b"invalid file handle\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if (*s).is_popen != 0 {
        err = js_get_errno(pclose((*s).f) as ssize_t) as i32
    } else {
        err = js_get_errno(fclose((*s).f) as ssize_t) as i32
    }
    (*s).f = 0 as *mut FILE;
    return JS_NewInt32(ctx, err);
}
unsafe extern "C" fn js_std_file_printf(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    return js_printf_internal(ctx, argc, argv, f);
}
unsafe extern "C" fn js_std_file_flush(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    fflush(f);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn js_std_file_tell(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
    mut is_bigint: i32,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    let mut pos: int64_t = 0;
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    pos = ftello(f);
    if is_bigint != 0 {
        return JS_NewBigInt64(ctx, pos);
    } else {
        return JS_NewInt64(ctx, pos);
    };
}
unsafe extern "C" fn js_std_file_seek(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    let mut pos: int64_t = 0;
    let mut whence: i32 = 0;
    let mut ret: i32 = 0;
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToInt64Ext(ctx, &mut pos, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToInt32(ctx, &mut whence, *argv.offset(1 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = fseeko(f, pos, whence);
    if ret < 0 as i32 {
        ret = -*__errno_location()
    }
    return JS_NewInt32(ctx, ret);
}
unsafe extern "C" fn js_std_file_eof(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    return JS_NewBool(ctx, feof(f));
}
unsafe extern "C" fn js_std_file_error(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    return JS_NewBool(ctx, ferror(f));
}
unsafe extern "C" fn js_std_file_clearerr(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    clearerr(f);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn js_std_file_fileno(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    return JS_NewInt32(ctx, fileno(f));
}
unsafe extern "C" fn js_std_file_read_write(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
    mut magic: i32,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    let mut pos: uint64_t = 0;
    let mut len: uint64_t = 0;
    let mut size: size_t = 0;
    let mut ret: size_t = 0;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToIndex(ctx, &mut pos, *argv.offset(1 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToIndex(ctx, &mut len, *argv.offset(2 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    buf = JS_GetArrayBuffer(ctx, &mut size, *argv.offset(0 as i32 as isize));
    if buf.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if pos.wrapping_add(len) > size {
        return JS_ThrowRangeError(
            ctx,
            b"read/write array buffer overflow\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if magic != 0 {
        ret = fwrite(
            buf.offset(pos as isize) as *const std::ffi::c_void,
            1 as i32 as u64,
            len,
            f,
        )
    } else {
        ret = fread(
            buf.offset(pos as isize) as *mut std::ffi::c_void,
            1 as i32 as u64,
            len,
            f,
        )
    }
    return JS_NewInt64(ctx, ret as int64_t);
}
/* XXX: could use less memory and go faster */
unsafe extern "C" fn js_std_file_getline(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    let mut c: i32 = 0;
    let mut dbuf: DynBuf = DynBuf {
        buf: 0 as *mut uint8_t,
        size: 0,
        allocated_size: 0,
        error: 0,
        realloc_func: None,
        opaque: 0 as *mut std::ffi::c_void,
    };
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    js_std_dbuf_init(ctx, &mut dbuf);
    loop {
        c = fgetc(f);
        if c == -(1 as i32) {
            if !(dbuf.size == 0 as i32 as u64) {
                break;
            }
            /* EOF */
            dbuf_free(&mut dbuf);
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_NULL as i32 as int64_t,
                };
                init
            };
        } else {
            if c == '\n' as i32 {
                break;
            }
            if dbuf_putc(&mut dbuf, c as uint8_t) != 0 {
                dbuf_free(&mut dbuf);
                return JS_ThrowOutOfMemory(ctx);
            }
        }
    }
    obj = JS_NewStringLen(ctx, dbuf.buf as *const std::os::raw::c_char, dbuf.size);
    dbuf_free(&mut dbuf);
    return obj;
}
/* XXX: could use less memory and go faster */
unsafe extern "C" fn js_std_file_readAsString(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    let mut c: i32 = 0;
    let mut dbuf: DynBuf = DynBuf {
        buf: 0 as *mut uint8_t,
        size: 0,
        allocated_size: 0,
        error: 0,
        realloc_func: None,
        opaque: 0 as *mut std::ffi::c_void,
    };
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut max_size64: uint64_t = 0;
    let mut max_size: size_t = 0;
    let mut max_size_val: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if argc >= 1 as i32 {
        max_size_val = *argv.offset(0 as i32 as isize)
    } else {
        max_size_val = {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_UNDEFINED as i32 as int64_t,
            };
            init
        }
    }
    max_size = -(1 as i32) as size_t;
    if JS_IsUndefined(max_size_val) == 0 {
        if JS_ToIndex(ctx, &mut max_size64, max_size_val) != 0 {
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            };
        }
        if max_size64 < max_size {
            max_size = max_size64
        }
    }
    js_std_dbuf_init(ctx, &mut dbuf);
    while max_size != 0 as i32 as u64 {
        c = fgetc(f);
        if c == -(1 as i32) {
            break;
        }
        if dbuf_putc(&mut dbuf, c as uint8_t) != 0 {
            dbuf_free(&mut dbuf);
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            };
        }
        max_size = max_size.wrapping_sub(1)
    }
    obj = JS_NewStringLen(ctx, dbuf.buf as *const std::os::raw::c_char, dbuf.size);
    dbuf_free(&mut dbuf);
    return obj;
}
unsafe extern "C" fn js_std_file_getByte(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    return JS_NewInt32(ctx, fgetc(f));
}
unsafe extern "C" fn js_std_file_putByte(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut f: *mut FILE = js_std_file_get(ctx, this_val);
    let mut c: i32 = 0;
    if f.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToInt32(ctx, &mut c, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    c = fputc(c, f);
    return JS_NewInt32(ctx, c);
}
unsafe extern "C" fn http_get_header_line(
    mut f: *mut FILE,
    mut buf: *mut std::os::raw::c_char,
    mut buf_size: size_t,
    mut dbuf: *mut DynBuf,
) -> i32 {
    let mut c: i32 = 0;
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    p = buf;
    loop {
        c = fgetc(f);
        if c < 0 as i32 {
            return -(1 as i32);
        }
        if (p.wrapping_offset_from(buf) as i64 as u64) < buf_size.wrapping_sub(1 as i32 as u64) {
            let fresh20 = p;
            p = p.offset(1);
            *fresh20 = c as std::os::raw::c_char
        }
        if !dbuf.is_null() {
            dbuf_putc(dbuf, c as uint8_t);
        }
        if c == '\n' as i32 {
            break;
        }
    }
    *p = '\u{0}' as i32 as std::os::raw::c_char;
    return 0 as i32;
}
unsafe extern "C" fn http_get_status(mut buf: *const std::os::raw::c_char) -> i32 {
    let mut p: *const std::os::raw::c_char = buf;
    while *p as i32 != ' ' as i32 && *p as i32 != '\u{0}' as i32 {
        p = p.offset(1)
    }
    if *p as i32 != ' ' as i32 {
        return 0 as i32;
    }
    while *p as i32 == ' ' as i32 {
        p = p.offset(1)
    }
    return atoi(p);
}
unsafe extern "C" fn js_std_urlGet(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut current_block: u64;
    let mut url: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut cmd_buf: DynBuf = DynBuf {
        buf: 0 as *mut uint8_t,
        size: 0,
        allocated_size: 0,
        error: 0,
        realloc_func: None,
        opaque: 0 as *mut std::ffi::c_void,
    };
    let mut data_buf_s: DynBuf = DynBuf {
        buf: 0 as *mut uint8_t,
        size: 0,
        allocated_size: 0,
        error: 0,
        realloc_func: None,
        opaque: 0 as *mut std::ffi::c_void,
    };
    let mut data_buf: *mut DynBuf = &mut data_buf_s;
    let mut header_buf_s: DynBuf = DynBuf {
        buf: 0 as *mut uint8_t,
        size: 0,
        allocated_size: 0,
        error: 0,
        realloc_func: None,
        opaque: 0 as *mut std::ffi::c_void,
    };
    let mut header_buf: *mut DynBuf = &mut header_buf_s;
    let mut buf: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    let mut c: i32 = 0;
    let mut status: i32 = 0;
    let mut response: JSValue = {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
    let mut ret_obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut options_obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut binary_flag: BOOL = 0;
    let mut full_flag: BOOL = 0;
    url = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if url.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    binary_flag = FALSE as i32;
    full_flag = FALSE as i32;
    if argc >= 2 as i32 {
        's_85: {
            options_obj = *argv.offset(1 as i32 as isize);
            if !(get_bool_option(
                ctx,
                &mut binary_flag,
                options_obj,
                b"binary\x00" as *const u8 as *const std::os::raw::c_char,
            ) != 0)
            {
                if !(get_bool_option(
                    ctx,
                    &mut full_flag,
                    options_obj,
                    b"full\x00" as *const u8 as *const std::os::raw::c_char,
                ) != 0)
                {
                    break 's_85;
                }
            }
            JS_FreeCString(ctx, url);
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            };
        }
    }
    js_std_dbuf_init(ctx, &mut cmd_buf);
    dbuf_printf(
        &mut cmd_buf as *mut DynBuf,
        b"%s \'\'\x00" as *const u8 as *const std::os::raw::c_char,
        b"curl -s -i\x00" as *const u8 as *const std::os::raw::c_char,
    );
    len = strlen(url);
    i = 0 as i32 as size_t;
    while i < len {
        c = *url.offset(i as isize) as i32;
        if c == '\'' as i32 || c == '\\' as i32 {
            dbuf_putc(&mut cmd_buf, '\\' as i32 as uint8_t);
        }
        dbuf_putc(&mut cmd_buf, c as uint8_t);
        i = i.wrapping_add(1)
    }
    JS_FreeCString(ctx, url);
    dbuf_putstr(
        &mut cmd_buf,
        b"\'\'\x00" as *const u8 as *const std::os::raw::c_char,
    );
    dbuf_putc(&mut cmd_buf, '\u{0}' as i32 as uint8_t);
    if dbuf_error(&mut cmd_buf) != 0 {
        dbuf_free(&mut cmd_buf);
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    //    printf("%s\n", (char *)cmd_buf.buf);
    f = popen(
        cmd_buf.buf as *mut std::os::raw::c_char,
        b"r\x00" as *const u8 as *const std::os::raw::c_char,
    );
    dbuf_free(&mut cmd_buf);
    if f.is_null() {
        return JS_ThrowTypeError(
            ctx,
            b"could not start curl\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    js_std_dbuf_init(ctx, data_buf);
    js_std_dbuf_init(ctx, header_buf);
    buf = js_malloc(ctx, 4096 as i32 as size_t) as *mut std::os::raw::c_char;
    if !buf.is_null() {
        /* get the HTTP status */
        if http_get_header_line(f, buf, 4096 as i32 as size_t, 0 as *mut DynBuf) < 0 as i32 {
            status = 0 as i32;
            current_block = 10762102765549496082;
        } else {
            status = http_get_status(buf);
            if full_flag == 0 && !(status >= 200 as i32 && status <= 299 as i32) {
                current_block = 10762102765549496082;
            } else {
                loop
                /* wait until there is an empty line */
                {
                    if http_get_header_line(f, buf, 4096 as i32 as size_t, header_buf) < 0 as i32 {
                        current_block = 10762102765549496082; /* remove the trailing CRLF */
                        break;
                    }
                    if !(strcmp(buf, b"\r\n\x00" as *const u8 as *const std::os::raw::c_char) == 0)
                    {
                        continue;
                    }
                    if dbuf_error(header_buf) != 0 {
                        current_block = 3690914394173635162;
                        break;
                    } else {
                        current_block = 5892776923941496671;
                        break;
                    }
                }
                match current_block {
                    10762102765549496082 => {}
                    3690914394173635162 => {}
                    _ => {
                        (*header_buf).size = ((*header_buf).size as u64)
                            .wrapping_sub(2 as i32 as u64)
                            as size_t as size_t;
                        loop
                        /* download the data */
                        {
                            len = fread(
                                buf as *mut std::ffi::c_void,
                                1 as i32 as u64,
                                4096 as i32 as u64,
                                f,
                            );
                            if len == 0 as i32 as u64 {
                                break;
                            }
                            dbuf_put(data_buf, buf as *mut uint8_t, len);
                        }
                        if dbuf_error(data_buf) != 0 {
                            current_block = 3690914394173635162;
                        } else {
                            if binary_flag != 0 {
                                response =
                                    JS_NewArrayBufferCopy(ctx, (*data_buf).buf, (*data_buf).size)
                            } else {
                                response = JS_NewStringLen(
                                    ctx,
                                    (*data_buf).buf as *mut std::os::raw::c_char,
                                    (*data_buf).size,
                                )
                            }
                            if JS_IsException(response) != 0 {
                                current_block = 3690914394173635162;
                            } else {
                                current_block = 11561494241259596775;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            3690914394173635162 => {}
            _ => {
                match current_block {
                    10762102765549496082 => {
                        response = {
                            let mut init = JSValue {
                                u: JSValueUnion { int32: 0 as i32 },
                                tag: JS_TAG_NULL as i32 as int64_t,
                            };
                            init
                        }
                    }
                    _ => {}
                }
                js_free(ctx, buf as *mut std::ffi::c_void);
                buf = 0 as *mut std::os::raw::c_char;
                pclose(f);
                f = 0 as *mut FILE;
                dbuf_free(data_buf);
                data_buf = 0 as *mut DynBuf;
                if full_flag != 0 {
                    ret_obj = JS_NewObject(ctx);
                    if JS_IsException(ret_obj) != 0 {
                        current_block = 3690914394173635162;
                    } else {
                        JS_DefinePropertyValueStr(
                            ctx,
                            ret_obj,
                            b"response\x00" as *const u8 as *const std::os::raw::c_char,
                            response,
                            (1 as i32) << 0 as i32
                                | (1 as i32) << 1 as i32
                                | (1 as i32) << 2 as i32,
                        );
                        if JS_IsNull(response) == 0 {
                            JS_DefinePropertyValueStr(
                                ctx,
                                ret_obj,
                                b"responseHeaders\x00" as *const u8 as *const std::os::raw::c_char,
                                JS_NewStringLen(
                                    ctx,
                                    (*header_buf).buf as *mut std::os::raw::c_char,
                                    (*header_buf).size,
                                ),
                                (1 as i32) << 0 as i32
                                    | (1 as i32) << 1 as i32
                                    | (1 as i32) << 2 as i32,
                            );
                            JS_DefinePropertyValueStr(
                                ctx,
                                ret_obj,
                                b"status\x00" as *const u8 as *const std::os::raw::c_char,
                                JS_NewInt32(ctx, status),
                                (1 as i32) << 0 as i32
                                    | (1 as i32) << 1 as i32
                                    | (1 as i32) << 2 as i32,
                            );
                        }
                        current_block = 993425571616822999;
                    }
                } else {
                    ret_obj = response;
                    current_block = 993425571616822999;
                }
                match current_block {
                    3690914394173635162 => {}
                    _ => {
                        dbuf_free(header_buf);
                        return ret_obj;
                    }
                }
            }
        }
    }
    if !f.is_null() {
        pclose(f);
    }
    js_free(ctx, buf as *mut std::ffi::c_void);
    if !data_buf.is_null() {
        dbuf_free(data_buf);
    }
    if !header_buf.is_null() {
        dbuf_free(header_buf);
    }
    JS_FreeValue(ctx, response);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_EXCEPTION as i32 as int64_t,
        };
        init
    };
}
static mut js_std_file_class: JSClassDef = unsafe {
    {
        let mut init = JSClassDef {
            class_name: b"FILE\x00" as *const u8 as *const std::os::raw::c_char,
            finalizer: Some(
                js_std_file_finalizer as unsafe extern "C" fn(_: *mut JSRuntime, _: JSValue) -> (),
            ),
            gc_mark: None,
            call: None,
            exotic: 0 as *const JSClassExoticMethods as *mut JSClassExoticMethods,
        };
        init
    }
};
static mut js_std_error_props: [JSCFunctionListEntry; 11] = [
    {
        let mut init = JSCFunctionListEntry {
            name: b"EINVAL\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 22 as i32 },
        };
        init
    },
    {
        let mut init = JSCFunctionListEntry {
            name: b"EIO\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 5 as i32 },
        };
        init
    },
    {
        let mut init = JSCFunctionListEntry {
            name: b"EACCES\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 13 as i32 },
        };
        init
    },
    {
        let mut init = JSCFunctionListEntry {
            name: b"EEXIST\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 17 as i32 },
        };
        init
    },
    {
        let mut init = JSCFunctionListEntry {
            name: b"ENOSPC\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 28 as i32 },
        };
        init
    },
    {
        let mut init = JSCFunctionListEntry {
            name: b"ENOSYS\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 38 as i32 },
        };
        init
    },
    {
        let mut init = JSCFunctionListEntry {
            name: b"EBUSY\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 16 as i32 },
        };
        init
    },
    {
        let mut init = JSCFunctionListEntry {
            name: b"ENOENT\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 2 as i32 },
        };
        init
    },
    {
        let mut init = JSCFunctionListEntry {
            name: b"EPERM\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 1 as i32 },
        };
        init
    },
    {
        let mut init = JSCFunctionListEntry {
            name: b"EPIPE\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 32 as i32 },
        };
        init
    },
    {
        let mut init = JSCFunctionListEntry {
            name: b"EBADF\x00" as *const u8 as *const std::os::raw::c_char,
            prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
            def_type: 4 as i32 as uint8_t,
            magic: 0 as i32 as int16_t,
            u: C2RustUnnamed_2 { i32_0: 9 as i32 },
        };
        init
    },
];
// Initialized in run_static_initializers
static mut js_std_funcs: [JSCFunctionListEntry; 23] = [JSCFunctionListEntry {
    name: 0 as *const std::os::raw::c_char,
    prop_flags: 0,
    def_type: 0,
    magic: 0,
    u: C2RustUnnamed_2 {
        func: C2RustUnnamed_6 {
            length: 0,
            cproto: 0,
            cfunc: JSCFunctionType { generic: None },
        },
    },
}; 23];
static mut js_std_file_proto_funcs: [JSCFunctionListEntry; 17] = unsafe {
    [
        {
            let mut init = JSCFunctionListEntry {
                name: b"close\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_close
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"puts\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 1 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_std_file_puts
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"printf\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_printf
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"flush\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_flush
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"tell\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_std_file_tell
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"tello\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 1 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_std_file_tell
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"seek\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_seek
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"eof\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_eof
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"fileno\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_fileno
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"error\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_error
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"clearerr\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_clearerr
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"read\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 3 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_std_file_read_write
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"write\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 1 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 3 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_std_file_read_write
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"getline\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_getline
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"readAsString\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_readAsString
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"getByte\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_getByte
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"putByte\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_file_putByte
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
    ]
};
unsafe extern "C" fn js_std_init(mut ctx: *mut JSContext, mut m: *mut JSModuleDef) -> i32 {
    let mut proto: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    /* FILE class */
    /* the class ID is created once */
    JS_NewClassID(&mut js_std_file_class_id);
    /* the class is created once per runtime */
    JS_NewClass(
        JS_GetRuntime(ctx),
        js_std_file_class_id,
        &mut js_std_file_class,
    );
    proto = JS_NewObject(ctx);
    JS_SetPropertyFunctionList(
        ctx,
        proto,
        js_std_file_proto_funcs.as_ptr(),
        (::std::mem::size_of::<[JSCFunctionListEntry; 17]>() as u64)
            .wrapping_div(::std::mem::size_of::<JSCFunctionListEntry>() as u64) as i32,
    );
    JS_SetClassProto(ctx, js_std_file_class_id, proto);
    JS_SetModuleExportList(
        ctx,
        m,
        js_std_funcs.as_ptr(),
        (::std::mem::size_of::<[JSCFunctionListEntry; 23]>() as u64)
            .wrapping_div(::std::mem::size_of::<JSCFunctionListEntry>() as u64) as i32,
    );
    JS_SetModuleExport(
        ctx,
        m,
        b"in\x00" as *const u8 as *const std::os::raw::c_char,
        js_new_std_file(ctx, stdin, FALSE as i32, FALSE as i32),
    );
    JS_SetModuleExport(
        ctx,
        m,
        b"out\x00" as *const u8 as *const std::os::raw::c_char,
        js_new_std_file(ctx, stdout, FALSE as i32, FALSE as i32),
    );
    JS_SetModuleExport(
        ctx,
        m,
        b"err\x00" as *const u8 as *const std::os::raw::c_char,
        js_new_std_file(ctx, stderr, FALSE as i32, FALSE as i32),
    );
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn js_init_module_std(
    mut ctx: *mut JSContext,
    mut module_name: *const std::os::raw::c_char,
) -> *mut JSModuleDef {
    let mut m: *mut JSModuleDef = 0 as *mut JSModuleDef;
    m = JS_NewCModule(
        ctx,
        module_name,
        Some(js_std_init as unsafe extern "C" fn(_: *mut JSContext, _: *mut JSModuleDef) -> i32),
    );
    if m.is_null() {
        return 0 as *mut JSModuleDef;
    }
    JS_AddModuleExportList(
        ctx,
        m,
        js_std_funcs.as_ptr(),
        (::std::mem::size_of::<[JSCFunctionListEntry; 23]>() as u64)
            .wrapping_div(::std::mem::size_of::<JSCFunctionListEntry>() as u64) as i32,
    );
    JS_AddModuleExport(
        ctx,
        m,
        b"in\x00" as *const u8 as *const std::os::raw::c_char,
    );
    JS_AddModuleExport(
        ctx,
        m,
        b"out\x00" as *const u8 as *const std::os::raw::c_char,
    );
    JS_AddModuleExport(
        ctx,
        m,
        b"err\x00" as *const u8 as *const std::os::raw::c_char,
    );
    return m;
}
/* *********************************************************/
/* 'os' object */
unsafe extern "C" fn js_os_open(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut current_block: u64;
    let mut filename: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut flags: i32 = 0;
    let mut mode: i32 = 0;
    let mut ret: i32 = 0;
    filename = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if filename.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if !(JS_ToInt32(ctx, &mut flags, *argv.offset(1 as i32 as isize)) != 0) {
        if argc >= 3 as i32 && JS_IsUndefined(*argv.offset(2 as i32 as isize)) == 0 {
            if JS_ToInt32(ctx, &mut mode, *argv.offset(2 as i32 as isize)) != 0 {
                current_block = 14372557507581650628;
            } else {
                current_block = 17965632435239708295;
            }
        } else {
            mode = 0o666 as i32;
            current_block = 17965632435239708295;
        }
        match current_block {
            14372557507581650628 => {}
            _ => {
                ret = js_get_errno(open(filename, flags, mode) as ssize_t) as i32;
                JS_FreeCString(ctx, filename);
                return JS_NewInt32(ctx, ret);
            }
        }
    }
    JS_FreeCString(ctx, filename);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_EXCEPTION as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn js_os_close(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut fd: i32 = 0;
    let mut ret: i32 = 0;
    if JS_ToInt32(ctx, &mut fd, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = js_get_errno(close(fd) as ssize_t) as i32;
    return JS_NewInt32(ctx, ret);
}
unsafe extern "C" fn js_os_seek(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut fd: i32 = 0;
    let mut whence: i32 = 0;
    let mut pos: int64_t = 0;
    let mut ret: int64_t = 0;
    let mut is_bigint: BOOL = 0;
    if JS_ToInt32(ctx, &mut fd, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    is_bigint = JS_IsBigInt(ctx, *argv.offset(1 as i32 as isize));
    if JS_ToInt64Ext(ctx, &mut pos, *argv.offset(1 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToInt32(ctx, &mut whence, *argv.offset(2 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = lseek(fd, pos, whence);
    if ret == -(1 as i32) as i64 {
        ret = -*__errno_location() as int64_t
    }
    if is_bigint != 0 {
        return JS_NewBigInt64(ctx, ret);
    } else {
        return JS_NewInt64(ctx, ret);
    };
}
unsafe extern "C" fn js_os_read_write(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
    mut magic: i32,
) -> JSValue {
    let mut fd: i32 = 0;
    let mut pos: uint64_t = 0;
    let mut len: uint64_t = 0;
    let mut size: size_t = 0;
    let mut ret: ssize_t = 0;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    if JS_ToInt32(ctx, &mut fd, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToIndex(ctx, &mut pos, *argv.offset(2 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToIndex(ctx, &mut len, *argv.offset(3 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    buf = JS_GetArrayBuffer(ctx, &mut size, *argv.offset(1 as i32 as isize));
    if buf.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if pos.wrapping_add(len) > size {
        return JS_ThrowRangeError(
            ctx,
            b"read/write array buffer overflow\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if magic != 0 {
        ret = js_get_errno(write(
            fd,
            buf.offset(pos as isize) as *const std::ffi::c_void,
            len,
        ))
    } else {
        ret = js_get_errno(read(
            fd,
            buf.offset(pos as isize) as *mut std::ffi::c_void,
            len,
        ))
    }
    return JS_NewInt64(ctx, ret);
}
unsafe extern "C" fn js_os_isatty(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut fd: i32 = 0;
    if JS_ToInt32(ctx, &mut fd, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    return JS_NewBool(ctx, (isatty(fd) == 1 as i32) as i32);
}
unsafe extern "C" fn js_os_ttyGetWinSize(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut fd: i32 = 0;
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    if JS_ToInt32(ctx, &mut fd, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if ioctl(fd, 0x5413 as i32 as u64, &mut ws as *mut winsize) == 0 as i32
        && ws.ws_col as i32 >= 4 as i32
        && ws.ws_row as i32 >= 4 as i32
    {
        obj = JS_NewArray(ctx);
        if JS_IsException(obj) != 0 {
            return obj;
        }
        JS_DefinePropertyValueUint32(
            ctx,
            obj,
            0 as i32 as uint32_t,
            JS_NewInt32(ctx, ws.ws_col as int32_t),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueUint32(
            ctx,
            obj,
            1 as i32 as uint32_t,
            JS_NewInt32(ctx, ws.ws_row as int32_t),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        return obj;
    } else {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_NULL as i32 as int64_t,
            };
            init
        };
    };
}
static mut oldtty: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
unsafe extern "C" fn term_exit() {
    tcsetattr(0 as i32, 0 as i32, &mut oldtty);
}
/* XXX: should add a way to go back to normal mode */
unsafe extern "C" fn js_os_ttySetRaw(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut tty: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut fd: i32 = 0;
    if JS_ToInt32(ctx, &mut fd, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    memset(
        &mut tty as *mut termios as *mut std::ffi::c_void,
        0 as i32,
        ::std::mem::size_of::<termios>() as u64,
    );
    tcgetattr(fd, &mut tty);
    oldtty = tty;
    tty.c_iflag &= !(0o1 as i32
        | 0o2 as i32
        | 0o10 as i32
        | 0o40 as i32
        | 0o100 as i32
        | 0o200 as i32
        | 0o400 as i32
        | 0o2000 as i32) as u32;
    tty.c_oflag |= 0o1 as i32 as u32;
    tty.c_lflag &= !(0o10 as i32 | 0o100 as i32 | 0o2 as i32 | 0o100000 as i32) as u32;
    tty.c_cflag &= !(0o60 as i32 | 0o400 as i32) as u32;
    tty.c_cflag |= 0o60 as i32 as u32;
    tty.c_cc[6 as i32 as usize] = 1 as i32 as cc_t;
    tty.c_cc[5 as i32 as usize] = 0 as i32 as cc_t;
    tcsetattr(fd, 0 as i32, &mut tty);
    atexit(Some(term_exit as unsafe extern "C" fn() -> ()));
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
/* !_WIN32 */
unsafe extern "C" fn js_os_remove(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut filename: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: i32 = 0;
    filename = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if filename.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = js_get_errno(remove(filename) as ssize_t) as i32;
    JS_FreeCString(ctx, filename);
    return JS_NewInt32(ctx, ret);
}
unsafe extern "C" fn js_os_rename(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut oldpath: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut newpath: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: i32 = 0;
    oldpath = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if oldpath.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    newpath = JS_ToCString(ctx, *argv.offset(1 as i32 as isize));
    if newpath.is_null() {
        JS_FreeCString(ctx, oldpath);
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = js_get_errno(rename(oldpath, newpath) as ssize_t) as i32;
    JS_FreeCString(ctx, oldpath);
    JS_FreeCString(ctx, newpath);
    return JS_NewInt32(ctx, ret);
}
unsafe extern "C" fn is_main_thread(mut rt: *mut JSRuntime) -> BOOL {
    let mut ts: *mut JSThreadState = JS_GetRuntimeOpaque(rt) as *mut JSThreadState;
    return (*ts).recv_pipe.is_null() as i32;
}
unsafe extern "C" fn find_rh(mut ts: *mut JSThreadState, mut fd: i32) -> *mut JSOSRWHandler {
    let mut rh: *mut JSOSRWHandler = 0 as *mut JSOSRWHandler;
    let mut el: *mut list_head = 0 as *mut list_head;
    el = (*ts).os_rw_handlers.next;
    while el != &mut (*ts).os_rw_handlers as *mut list_head {
        rh = (el as *mut uint8_t)
            .offset(-(&mut (*(0 as *mut JSOSRWHandler)).link as *mut list_head as size_t as isize))
            as *mut JSOSRWHandler;
        if (*rh).fd == fd {
            return rh;
        }
        el = (*el).next
    }
    return 0 as *mut JSOSRWHandler;
}
unsafe extern "C" fn free_rw_handler(mut rt: *mut JSRuntime, mut rh: *mut JSOSRWHandler) {
    let mut i: i32 = 0;
    list_del(&mut (*rh).link);
    i = 0 as i32;
    while i < 2 as i32 {
        JS_FreeValueRT(rt, (*rh).rw_func[i as usize]);
        i += 1
    }
    js_free_rt(rt, rh as *mut std::ffi::c_void);
}
unsafe extern "C" fn js_os_setReadHandler(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
    mut magic: i32,
) -> JSValue {
    let mut rt: *mut JSRuntime = JS_GetRuntime(ctx);
    let mut ts: *mut JSThreadState = JS_GetRuntimeOpaque(rt) as *mut JSThreadState;
    let mut rh: *mut JSOSRWHandler = 0 as *mut JSOSRWHandler;
    let mut fd: i32 = 0;
    let mut func: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    if JS_ToInt32(ctx, &mut fd, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    func = *argv.offset(1 as i32 as isize);
    if JS_IsNull(func) != 0 {
        rh = find_rh(ts, fd);
        if !rh.is_null() {
            JS_FreeValue(ctx, (*rh).rw_func[magic as usize]);
            (*rh).rw_func[magic as usize] = {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_NULL as i32 as int64_t,
                };
                init
            };
            if JS_IsNull((*rh).rw_func[0 as i32 as usize]) != 0
                && JS_IsNull((*rh).rw_func[1 as i32 as usize]) != 0
            {
                /* remove the entry */
                free_rw_handler(JS_GetRuntime(ctx), rh);
            }
        }
    } else {
        if JS_IsFunction(ctx, func) == 0 {
            return JS_ThrowTypeError(
                ctx,
                b"not a function\x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
        rh = find_rh(ts, fd);
        if rh.is_null() {
            rh = js_mallocz(ctx, ::std::mem::size_of::<JSOSRWHandler>() as u64)
                as *mut JSOSRWHandler;
            if rh.is_null() {
                return {
                    let mut init = JSValue {
                        u: JSValueUnion { int32: 0 as i32 },
                        tag: JS_TAG_EXCEPTION as i32 as int64_t,
                    };
                    init
                };
            }
            (*rh).fd = fd;
            (*rh).rw_func[0 as i32 as usize] = {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_NULL as i32 as int64_t,
                };
                init
            };
            (*rh).rw_func[1 as i32 as usize] = {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_NULL as i32 as int64_t,
                };
                init
            };
            list_add_tail(&mut (*rh).link, &mut (*ts).os_rw_handlers);
        }
        JS_FreeValue(ctx, (*rh).rw_func[magic as usize]);
        (*rh).rw_func[magic as usize] = JS_DupValue(ctx, func)
    }
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn find_sh(
    mut ts: *mut JSThreadState,
    mut sig_num: i32,
) -> *mut JSOSSignalHandler {
    let mut sh: *mut JSOSSignalHandler = 0 as *mut JSOSSignalHandler;
    let mut el: *mut list_head = 0 as *mut list_head;
    el = (*ts).os_signal_handlers.next;
    while el != &mut (*ts).os_signal_handlers as *mut list_head {
        sh = (el as *mut uint8_t).offset(
            -(&mut (*(0 as *mut JSOSSignalHandler)).link as *mut list_head as size_t as isize),
        ) as *mut JSOSSignalHandler;
        if (*sh).sig_num == sig_num {
            return sh;
        }
        el = (*el).next
    }
    return 0 as *mut JSOSSignalHandler;
}
unsafe extern "C" fn free_sh(mut rt: *mut JSRuntime, mut sh: *mut JSOSSignalHandler) {
    list_del(&mut (*sh).link);
    JS_FreeValueRT(rt, (*sh).func);
    js_free_rt(rt, sh as *mut std::ffi::c_void);
}
unsafe extern "C" fn os_signal_handler(mut sig_num: i32) {
    os_pending_signals |= (1 as i32 as uint64_t) << sig_num;
}
unsafe extern "C" fn js_os_signal(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut rt: *mut JSRuntime = JS_GetRuntime(ctx);
    let mut ts: *mut JSThreadState = JS_GetRuntimeOpaque(rt) as *mut JSThreadState;
    let mut sh: *mut JSOSSignalHandler = 0 as *mut JSOSSignalHandler;
    let mut sig_num: uint32_t = 0;
    let mut func: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut handler: sighandler_t = None;
    if is_main_thread(rt) == 0 {
        return JS_ThrowTypeError(
            ctx,
            b"signal handler can only be set in the main thread\x00" as *const u8
                as *const std::os::raw::c_char,
        );
    }
    if JS_ToUint32(ctx, &mut sig_num, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if sig_num >= 64 as i32 as u32 {
        return JS_ThrowRangeError(
            ctx,
            b"invalid signal number\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    func = *argv.offset(1 as i32 as isize);
    /* func = null: SIG_DFL, func = undefined, SIG_IGN */
    if JS_IsNull(func) != 0 || JS_IsUndefined(func) != 0 {
        sh = find_sh(ts, sig_num as i32);
        if !sh.is_null() {
            free_sh(JS_GetRuntime(ctx), sh);
        }
        if JS_IsNull(func) != 0 {
            handler = None
        } else {
            handler = ::std::mem::transmute::<isize, __sighandler_t>(1 as i32 as isize)
        }
        signal(sig_num as i32, handler);
    } else {
        if JS_IsFunction(ctx, func) == 0 {
            return JS_ThrowTypeError(
                ctx,
                b"not a function\x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
        sh = find_sh(ts, sig_num as i32);
        if sh.is_null() {
            sh = js_mallocz(ctx, ::std::mem::size_of::<JSOSSignalHandler>() as u64)
                as *mut JSOSSignalHandler;
            if sh.is_null() {
                return {
                    let mut init = JSValue {
                        u: JSValueUnion { int32: 0 as i32 },
                        tag: JS_TAG_EXCEPTION as i32 as int64_t,
                    };
                    init
                };
            }
            (*sh).sig_num = sig_num as i32;
            list_add_tail(&mut (*sh).link, &mut (*ts).os_signal_handlers);
        }
        JS_FreeValue(ctx, (*sh).func);
        (*sh).func = JS_DupValue(ctx, func);
        signal(
            sig_num as i32,
            Some(os_signal_handler as unsafe extern "C" fn(_: i32) -> ()),
        );
    }
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
unsafe extern "C" fn get_time_ms() -> int64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(1 as i32, &mut ts);
    return (ts.tv_sec as uint64_t)
        .wrapping_mul(1000 as i32 as u64)
        .wrapping_add((ts.tv_nsec / 1000000 as i32 as i64) as u64) as int64_t;
}
unsafe extern "C" fn unlink_timer(mut rt: *mut JSRuntime, mut th: *mut JSOSTimer) {
    if !(*th).link.prev.is_null() {
        list_del(&mut (*th).link);
        (*th).link.next = 0 as *mut list_head;
        (*th).link.prev = (*th).link.next
    };
}
unsafe extern "C" fn free_timer(mut rt: *mut JSRuntime, mut th: *mut JSOSTimer) {
    JS_FreeValueRT(rt, (*th).func);
    js_free_rt(rt, th as *mut std::ffi::c_void);
}
static mut js_os_timer_class_id: JSClassID = 0;
unsafe extern "C" fn js_os_timer_finalizer(mut rt: *mut JSRuntime, mut val: JSValue) {
    let mut th: *mut JSOSTimer = JS_GetOpaque(val, js_os_timer_class_id) as *mut JSOSTimer;
    if !th.is_null() {
        (*th).has_object = FALSE as i32;
        if (*th).link.prev.is_null() {
            free_timer(rt, th);
        }
    };
}
unsafe extern "C" fn js_os_timer_mark(
    mut rt: *mut JSRuntime,
    mut val: JSValue,
    mut mark_func: Option<JS_MarkFunc>,
) {
    let mut th: *mut JSOSTimer = JS_GetOpaque(val, js_os_timer_class_id) as *mut JSOSTimer;
    if !th.is_null() {
        JS_MarkValue(rt, (*th).func, mark_func);
    };
}
unsafe extern "C" fn js_os_setTimeout(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut rt: *mut JSRuntime = JS_GetRuntime(ctx);
    let mut ts: *mut JSThreadState = JS_GetRuntimeOpaque(rt) as *mut JSThreadState;
    let mut delay: int64_t = 0;
    let mut func: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut th: *mut JSOSTimer = 0 as *mut JSOSTimer;
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    func = *argv.offset(0 as i32 as isize);
    if JS_IsFunction(ctx, func) == 0 {
        return JS_ThrowTypeError(
            ctx,
            b"not a function\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if JS_ToInt64(ctx, &mut delay, *argv.offset(1 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    obj = JS_NewObjectClass(ctx, js_os_timer_class_id as i32);
    if JS_IsException(obj) != 0 {
        return obj;
    }
    th = js_mallocz(ctx, ::std::mem::size_of::<JSOSTimer>() as u64) as *mut JSOSTimer;
    if th.is_null() {
        JS_FreeValue(ctx, obj);
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    (*th).has_object = TRUE as i32;
    (*th).timeout = get_time_ms() + delay;
    (*th).func = JS_DupValue(ctx, func);
    list_add_tail(&mut (*th).link, &mut (*ts).os_timers);
    JS_SetOpaque(obj, th as *mut std::ffi::c_void);
    return obj;
}
unsafe extern "C" fn js_os_clearTimeout(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut th: *mut JSOSTimer =
        JS_GetOpaque2(ctx, *argv.offset(0 as i32 as isize), js_os_timer_class_id) as *mut JSOSTimer;
    if th.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    unlink_timer(JS_GetRuntime(ctx), th);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
static mut js_os_timer_class: JSClassDef = unsafe {
    {
        let mut init = JSClassDef {
            class_name: b"OSTimer\x00" as *const u8 as *const std::os::raw::c_char,
            finalizer: Some(
                js_os_timer_finalizer as unsafe extern "C" fn(_: *mut JSRuntime, _: JSValue) -> (),
            ),
            gc_mark: Some(
                js_os_timer_mark
                    as unsafe extern "C" fn(
                        _: *mut JSRuntime,
                        _: JSValue,
                        _: Option<JS_MarkFunc>,
                    ) -> (),
            ),
            call: None,
            exotic: 0 as *const JSClassExoticMethods as *mut JSClassExoticMethods,
        };
        init
    }
};
unsafe extern "C" fn call_handler(mut ctx: *mut JSContext, mut func: JSValue) {
    let mut ret: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut func1: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    /* 'func' might be destroyed when calling itself (if it frees the
    handler), so must take extra care */
    func1 = JS_DupValue(ctx, func);
    ret = JS_Call(
        ctx,
        func1,
        {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_UNDEFINED as i32 as int64_t,
            };
            init
        },
        0 as i32,
        0 as *mut JSValue,
    );
    JS_FreeValue(ctx, func1);
    if JS_IsException(ret) != 0 {
        js_std_dump_error(ctx);
    }
    JS_FreeValue(ctx, ret);
}
unsafe extern "C" fn handle_posted_message(
    mut rt: *mut JSRuntime,
    mut ctx: *mut JSContext,
    mut port: *mut JSWorkerMessageHandler,
) -> i32 {
    return 0 as i32;
}
unsafe extern "C" fn js_os_poll(mut ctx: *mut JSContext) -> i32 {
    let mut current_block: u64;
    let mut rt: *mut JSRuntime = JS_GetRuntime(ctx);
    let mut ts: *mut JSThreadState = JS_GetRuntimeOpaque(rt) as *mut JSThreadState;
    let mut ret: i32 = 0;
    let mut fd_max: i32 = 0;
    let mut min_delay: i32 = 0;
    let mut cur_time: int64_t = 0;
    let mut delay: int64_t = 0;
    let mut rfds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut wfds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut rh: *mut JSOSRWHandler = 0 as *mut JSOSRWHandler;
    let mut el: *mut list_head = 0 as *mut list_head;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut tvp: *mut timeval = 0 as *mut timeval;
    /* only check signals in the main thread */
    if (*ts).recv_pipe.is_null() && (os_pending_signals != 0 as i32 as u64) as i32 as i64 != 0 {
        let mut sh: *mut JSOSSignalHandler = 0 as *mut JSOSSignalHandler; /* no more events */
        let mut mask: uint64_t = 0;
        el = (*ts).os_signal_handlers.next;
        while el != &mut (*ts).os_signal_handlers as *mut list_head {
            sh = (el as *mut uint8_t).offset(
                -(&mut (*(0 as *mut JSOSSignalHandler)).link as *mut list_head as size_t as isize),
            ) as *mut JSOSSignalHandler;
            mask = (1 as i32 as uint64_t) << (*sh).sig_num;
            if os_pending_signals & mask != 0 {
                os_pending_signals &= !mask;
                call_handler(ctx, (*sh).func);
                return 0 as i32;
            }
            el = (*el).next
        }
    }
    if list_empty(&mut (*ts).os_rw_handlers) != 0
        && list_empty(&mut (*ts).os_timers) != 0
        && list_empty(&mut (*ts).port_list) != 0
    {
        return -(1 as i32);
    }
    if list_empty(&mut (*ts).os_timers) == 0 {
        cur_time = get_time_ms();
        min_delay = 10000 as i32;
        el = (*ts).os_timers.next;
        while el != &mut (*ts).os_timers as *mut list_head {
            let mut th: *mut JSOSTimer = (el as *mut uint8_t)
                .offset(-(&mut (*(0 as *mut JSOSTimer)).link as *mut list_head as size_t as isize))
                as *mut JSOSTimer;
            delay = (*th).timeout - cur_time;
            if delay <= 0 as i32 as i64 {
                let mut func: JSValue = JSValue {
                    u: JSValueUnion { int32: 0 },
                    tag: 0,
                };
                /* the timer expired */
                func = (*th).func;
                (*th).func = {
                    let mut init = JSValue {
                        u: JSValueUnion { int32: 0 as i32 },
                        tag: JS_TAG_UNDEFINED as i32 as int64_t,
                    };
                    init
                };
                unlink_timer(rt, th);
                if (*th).has_object == 0 {
                    free_timer(rt, th);
                }
                call_handler(ctx, func);
                JS_FreeValue(ctx, func);
                return 0 as i32;
            } else {
                if delay < min_delay as i64 {
                    min_delay = delay as i32
                }
            }
            el = (*el).next
        }
        tv.tv_sec = (min_delay / 1000 as i32) as __time_t;
        tv.tv_usec = (min_delay % 1000 as i32 * 1000 as i32) as __suseconds_t;
        tvp = &mut tv
    } else {
        tvp = 0 as *mut timeval
    }
    let mut __d0: i32 = 0;
    let mut __d1: i32 = 0;
    let fresh21 = &mut __d0;
    let fresh22;
    let fresh23 = &mut __d1;
    let fresh24;
    let fresh25 = (::std::mem::size_of::<fd_set>() as u64)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as u64);
    let fresh26 = &mut *rfds.fds_bits.as_mut_ptr().offset(0 as i32 as isize) as *mut __fd_mask;
    llvm_asm!("cld; rep; stosq" : "={cx}" (fresh22), "={di}" (fresh24) : "{ax}"
     (0 as i32), "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh25)), "1"
     (c2rust_asm_casts::AsmCast::cast_in(fresh23, fresh26)) : "memory" :
     "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh25, fresh22);
    c2rust_asm_casts::AsmCast::cast_out(fresh23, fresh26, fresh24);
    let mut __d0_0: i32 = 0;
    let mut __d1_0: i32 = 0;
    let fresh27 = &mut __d0_0;
    let fresh28;
    let fresh29 = &mut __d1_0;
    let fresh30;
    let fresh31 = (::std::mem::size_of::<fd_set>() as u64)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as u64);
    let fresh32 = &mut *wfds.fds_bits.as_mut_ptr().offset(0 as i32 as isize) as *mut __fd_mask;
    llvm_asm!("cld; rep; stosq" : "={cx}" (fresh28), "={di}" (fresh30) : "{ax}"
     (0 as i32), "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh27, fresh31)), "1"
     (c2rust_asm_casts::AsmCast::cast_in(fresh29, fresh32)) : "memory" :
     "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh27, fresh31, fresh28);
    c2rust_asm_casts::AsmCast::cast_out(fresh29, fresh32, fresh30);
    fd_max = -(1 as i32);
    el = (*ts).os_rw_handlers.next;
    while el != &mut (*ts).os_rw_handlers as *mut list_head {
        rh = (el as *mut uint8_t)
            .offset(-(&mut (*(0 as *mut JSOSRWHandler)).link as *mut list_head as size_t as isize))
            as *mut JSOSRWHandler;
        fd_max = max_int(fd_max, (*rh).fd);
        if JS_IsNull((*rh).rw_func[0 as i32 as usize]) == 0 {
            rfds.fds_bits[((*rh).fd / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize] |= ((1 as u64)
                << (*rh).fd % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask
        }
        if JS_IsNull((*rh).rw_func[1 as i32 as usize]) == 0 {
            wfds.fds_bits[((*rh).fd / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize] |= ((1 as u64)
                << (*rh).fd % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask
        }
        el = (*el).next
    }
    el = (*ts).port_list.next;
    while el != &mut (*ts).port_list as *mut list_head {
        let mut port: *mut JSWorkerMessageHandler = (el as *mut uint8_t).offset(
            -(&mut (*(0 as *mut JSWorkerMessageHandler)).link as *mut list_head as size_t as isize),
        ) as *mut JSWorkerMessageHandler;
        if JS_IsNull((*port).on_message_func) == 0 {
            let mut ps: *mut JSWorkerMessagePipe = (*port).recv_pipe;
            fd_max = max_int(fd_max, (*ps).read_fd);
            rfds.fds_bits[((*ps).read_fd
                / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize] |= ((1 as u64)
                << (*ps).read_fd % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask
        }
        el = (*el).next
    }
    ret = select(
        fd_max + 1 as i32,
        &mut rfds,
        &mut wfds,
        0 as *mut fd_set,
        tvp,
    );
    if ret > 0 as i32 {
        el = (*ts).os_rw_handlers.next;
        loop {
            if !(el != &mut (*ts).os_rw_handlers as *mut list_head) {
                current_block = 5854763015135596753;
                break;
            }
            rh = (el as *mut uint8_t).offset(
                -(&mut (*(0 as *mut JSOSRWHandler)).link as *mut list_head as size_t as isize),
            ) as *mut JSOSRWHandler;
            if JS_IsNull((*rh).rw_func[0 as i32 as usize]) == 0
                && rfds.fds_bits[((*rh).fd
                    / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    & ((1 as u64)
                        << (*rh).fd % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask
                    != 0 as i32 as i64
            {
                call_handler(ctx, (*rh).rw_func[0 as i32 as usize]);
                /* must stop because the list may have been modified */
                current_block = 5725136613830740200;
                break;
            } else if JS_IsNull((*rh).rw_func[1 as i32 as usize]) == 0
                && wfds.fds_bits[((*rh).fd
                    / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    & ((1 as u64)
                        << (*rh).fd % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask
                    != 0 as i32 as i64
            {
                call_handler(ctx, (*rh).rw_func[1 as i32 as usize]);
                current_block = 5725136613830740200;
                break;
            } else {
                el = (*el).next
            }
        }
        match current_block {
            5725136613830740200 => {}
            _ => {
                el = (*ts).port_list.next;
                while el != &mut (*ts).port_list as *mut list_head {
                    let mut port_0: *mut JSWorkerMessageHandler = (el as *mut uint8_t).offset(
                        -(&mut (*(0 as *mut JSWorkerMessageHandler)).link as *mut list_head
                            as size_t as isize),
                    )
                        as *mut JSWorkerMessageHandler;
                    if JS_IsNull((*port_0).on_message_func) == 0 {
                        let mut ps_0: *mut JSWorkerMessagePipe = (*port_0).recv_pipe;
                        if rfds.fds_bits[((*ps_0).read_fd
                            / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                            as usize]
                            & ((1 as u64)
                                << (*ps_0).read_fd
                                    % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as u64 as i32))
                                as __fd_mask
                            != 0 as i32 as i64
                        {
                            if handle_posted_message(rt, ctx, port_0) != 0 {
                                break;
                            }
                        }
                    }
                    el = (*el).next
                }
            }
        }
    }
    /* must stop because the list may have been modified */
    return 0 as i32;
}
/* !_WIN32 */
unsafe extern "C" fn make_obj_error(
    mut ctx: *mut JSContext,
    mut obj: JSValue,
    mut err: i32,
) -> JSValue {
    let mut arr: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    if JS_IsException(obj) != 0 {
        return obj;
    }
    arr = JS_NewArray(ctx);
    if JS_IsException(arr) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    JS_DefinePropertyValueUint32(
        ctx,
        arr,
        0 as i32 as uint32_t,
        obj,
        (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
    );
    JS_DefinePropertyValueUint32(
        ctx,
        arr,
        1 as i32 as uint32_t,
        JS_NewInt32(ctx, err),
        (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
    );
    return arr;
}
unsafe extern "C" fn make_string_error(
    mut ctx: *mut JSContext,
    mut buf: *const std::os::raw::c_char,
    mut err: i32,
) -> JSValue {
    return make_obj_error(ctx, JS_NewString(ctx, buf), err);
}
/* return [cwd, errorcode] */
unsafe extern "C" fn js_os_getcwd(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut buf: [std::os::raw::c_char; 4096] = [0; 4096];
    let mut err: i32 = 0;
    if getcwd(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 4096]>() as u64,
    )
    .is_null()
    {
        buf[0 as i32 as usize] = '\u{0}' as i32 as std::os::raw::c_char;
        err = *__errno_location()
    } else {
        err = 0 as i32
    }
    return make_string_error(ctx, buf.as_mut_ptr(), err);
}
unsafe extern "C" fn js_os_chdir(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut target: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut err: i32 = 0;
    target = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if target.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    err = js_get_errno(chdir(target) as ssize_t) as i32;
    JS_FreeCString(ctx, target);
    return JS_NewInt32(ctx, err);
}
unsafe extern "C" fn js_os_mkdir(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut mode: i32 = 0;
    let mut ret: i32 = 0;
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if argc >= 2 as i32 {
        if JS_ToInt32(ctx, &mut mode, *argv.offset(1 as i32 as isize)) != 0 {
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            };
        }
    } else {
        mode = 0o777 as i32
    }
    path = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if path.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = js_get_errno(mkdir(path, mode as __mode_t) as ssize_t) as i32;
    JS_FreeCString(ctx, path);
    return JS_NewInt32(ctx, ret);
}
/* return [array, errorcode] */
unsafe extern "C" fn js_os_readdir(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut f: *mut DIR = 0 as *mut DIR;
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut err: i32 = 0;
    let mut len: uint32_t = 0;
    path = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if path.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    obj = JS_NewArray(ctx);
    if JS_IsException(obj) != 0 {
        JS_FreeCString(ctx, path);
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    f = opendir(path);
    if f.is_null() {
        err = *__errno_location()
    } else {
        err = 0 as i32
    }
    JS_FreeCString(ctx, path);
    if !f.is_null() {
        len = 0 as i32 as uint32_t;
        loop {
            *__errno_location() = 0 as i32;
            d = readdir(f);
            if d.is_null() {
                err = *__errno_location();
                break;
            } else {
                let fresh33 = len;
                len = len.wrapping_add(1);
                JS_DefinePropertyValueUint32(
                    ctx,
                    obj,
                    fresh33,
                    JS_NewString(ctx, (*d).d_name.as_mut_ptr()),
                    (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
                );
            }
        }
        closedir(f);
    }
    return make_obj_error(ctx, obj, err);
}
unsafe extern "C" fn timespec_to_ms(mut tv: *const timespec) -> int64_t {
    return (*tv).tv_sec * 1000 as i32 as i64 + (*tv).tv_nsec / 1000000 as i32 as i64;
}
/* return [obj, errcode] */
unsafe extern "C" fn js_os_stat(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
    mut is_lstat: i32,
) -> JSValue {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut err: i32 = 0;
    let mut res: i32 = 0;
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    path = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if path.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if is_lstat != 0 {
        res = lstat(path, &mut st)
    } else {
        res = stat(path, &mut st)
    }
    JS_FreeCString(ctx, path);
    if res < 0 as i32 {
        err = *__errno_location();
        obj = {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_NULL as i32 as int64_t,
            };
            init
        }
    } else {
        err = 0 as i32;
        obj = JS_NewObject(ctx);
        if JS_IsException(obj) != 0 {
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            };
        }
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"dev\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, st.st_dev as int64_t),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"ino\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, st.st_ino as int64_t),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"mode\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt32(ctx, st.st_mode as int32_t),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"nlink\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, st.st_nlink as int64_t),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"uid\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, st.st_uid as int64_t),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"gid\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, st.st_gid as int64_t),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"rdev\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, st.st_rdev as int64_t),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"size\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, st.st_size),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"blocks\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, st.st_blocks),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"atime\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, timespec_to_ms(&mut st.st_atim)),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"mtime\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, timespec_to_ms(&mut st.st_mtim)),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
        JS_DefinePropertyValueStr(
            ctx,
            obj,
            b"ctime\x00" as *const u8 as *const std::os::raw::c_char,
            JS_NewInt64(ctx, timespec_to_ms(&mut st.st_ctim)),
            (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
        );
    }
    return make_obj_error(ctx, obj, err);
}
unsafe extern "C" fn ms_to_timeval(mut tv: *mut timeval, mut v: uint64_t) {
    (*tv).tv_sec = v.wrapping_div(1000 as i32 as u64) as __time_t;
    (*tv).tv_usec = v
        .wrapping_rem(1000 as i32 as u64)
        .wrapping_mul(1000 as i32 as u64) as __suseconds_t;
}
unsafe extern "C" fn js_os_utimes(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut atime: int64_t = 0;
    let mut mtime: int64_t = 0;
    let mut ret: i32 = 0;
    if JS_ToInt64(ctx, &mut atime, *argv.offset(1 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToInt64(ctx, &mut mtime, *argv.offset(2 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    path = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if path.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    let mut times: [timeval; 2] = [timeval {
        tv_sec: 0,
        tv_usec: 0,
    }; 2];
    ms_to_timeval(
        &mut *times.as_mut_ptr().offset(0 as i32 as isize),
        atime as uint64_t,
    );
    ms_to_timeval(
        &mut *times.as_mut_ptr().offset(1 as i32 as isize),
        mtime as uint64_t,
    );
    ret = js_get_errno(utimes(path, times.as_mut_ptr() as *const timeval) as ssize_t) as i32;
    JS_FreeCString(ctx, path);
    return JS_NewInt32(ctx, ret);
}
/* return [path, errorcode] */
unsafe extern "C" fn js_os_realpath(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut buf: [std::os::raw::c_char; 4096] = [0; 4096];
    let mut res: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut err: i32 = 0;
    path = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if path.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    res = realpath(path, buf.as_mut_ptr());
    JS_FreeCString(ctx, path);
    if res.is_null() {
        buf[0 as i32 as usize] = '\u{0}' as i32 as std::os::raw::c_char;
        err = *__errno_location()
    } else {
        err = 0 as i32
    }
    return make_string_error(ctx, buf.as_mut_ptr(), err);
}
unsafe extern "C" fn js_os_symlink(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut target: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut linkpath: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut err: i32 = 0;
    target = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if target.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    linkpath = JS_ToCString(ctx, *argv.offset(1 as i32 as isize));
    if linkpath.is_null() {
        JS_FreeCString(ctx, target);
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    err = js_get_errno(symlink(target, linkpath) as ssize_t) as i32;
    JS_FreeCString(ctx, target);
    JS_FreeCString(ctx, linkpath);
    return JS_NewInt32(ctx, err);
}
/* return [path, errorcode] */
unsafe extern "C" fn js_os_readlink(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut buf: [std::os::raw::c_char; 4096] = [0; 4096];
    let mut err: i32 = 0;
    let mut res: ssize_t = 0;
    path = JS_ToCString(ctx, *argv.offset(0 as i32 as isize));
    if path.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    res = readlink(
        path,
        buf.as_mut_ptr(),
        (::std::mem::size_of::<[std::os::raw::c_char; 4096]>() as u64)
            .wrapping_sub(1 as i32 as u64),
    );
    if res < 0 as i32 as i64 {
        buf[0 as i32 as usize] = '\u{0}' as i32 as std::os::raw::c_char;
        err = *__errno_location()
    } else {
        buf[res as usize] = '\u{0}' as i32 as std::os::raw::c_char;
        err = 0 as i32
    }
    JS_FreeCString(ctx, path);
    return make_string_error(ctx, buf.as_mut_ptr(), err);
}
unsafe extern "C" fn build_envp(
    mut ctx: *mut JSContext,
    mut obj: JSValue,
) -> *mut *mut std::os::raw::c_char {
    let mut current_block: u64;
    let mut len: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut tab: *mut JSPropertyEnum = 0 as *mut JSPropertyEnum;
    let mut envp: *mut *mut std::os::raw::c_char = 0 as *mut *mut std::os::raw::c_char;
    let mut pair: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut key: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut val: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut key_len: size_t = 0;
    let mut str_len: size_t = 0;
    if JS_GetOwnPropertyNames(
        ctx,
        &mut tab,
        &mut len,
        obj,
        (1 as i32) << 0 as i32 | (1 as i32) << 4 as i32,
    ) < 0 as i32
    {
        return 0 as *mut *mut std::os::raw::c_char;
    }
    envp = js_mallocz(
        ctx,
        (::std::mem::size_of::<*mut std::os::raw::c_char>() as u64)
            .wrapping_mul((len as size_t).wrapping_add(1 as i32 as u64)),
    ) as *mut *mut std::os::raw::c_char;
    if envp.is_null() {
        current_block = 11032202465615545148;
    } else {
        i = 0 as i32 as uint32_t;
        loop {
            if !(i < len) {
                current_block = 3322716925953924881;
                break;
            }
            val = JS_GetProperty(ctx, obj, (*tab.offset(i as isize)).atom);
            if JS_IsException(val) != 0 {
                current_block = 11032202465615545148;
                break;
            }
            str = JS_ToCString(ctx, val);
            JS_FreeValue(ctx, val);
            if str.is_null() {
                current_block = 11032202465615545148;
                break;
            }
            key = JS_AtomToCString(ctx, (*tab.offset(i as isize)).atom);
            if key.is_null() {
                JS_FreeCString(ctx, str);
                current_block = 11032202465615545148;
                break;
            } else {
                key_len = strlen(key);
                str_len = strlen(str);
                pair = js_malloc(
                    ctx,
                    key_len.wrapping_add(str_len).wrapping_add(2 as i32 as u64),
                ) as *mut std::os::raw::c_char;
                if pair.is_null() {
                    JS_FreeCString(ctx, key);
                    JS_FreeCString(ctx, str);
                    current_block = 11032202465615545148;
                    break;
                } else {
                    memcpy(
                        pair as *mut std::ffi::c_void,
                        key as *const std::ffi::c_void,
                        key_len,
                    );
                    *pair.offset(key_len as isize) = '=' as i32 as std::os::raw::c_char;
                    memcpy(
                        pair.offset(key_len as isize).offset(1 as i32 as isize)
                            as *mut std::ffi::c_void,
                        str as *const std::ffi::c_void,
                        str_len,
                    );
                    *pair.offset(
                        key_len.wrapping_add(1 as i32 as u64).wrapping_add(str_len) as isize
                    ) = '\u{0}' as i32 as std::os::raw::c_char;
                    let ref mut fresh34 = *envp.offset(i as isize);
                    *fresh34 = pair;
                    JS_FreeCString(ctx, key);
                    JS_FreeCString(ctx, str);
                    i = i.wrapping_add(1)
                }
            }
        }
    }
    match current_block {
        11032202465615545148 => {
            if !envp.is_null() {
                i = 0 as i32 as uint32_t;
                while i < len {
                    js_free(ctx, *envp.offset(i as isize) as *mut std::ffi::c_void);
                    i = i.wrapping_add(1)
                }
                js_free(ctx, envp as *mut std::ffi::c_void);
                envp = 0 as *mut *mut std::os::raw::c_char
            }
        }
        _ => {}
    }
    i = 0 as i32 as uint32_t;
    while i < len {
        JS_FreeAtom(ctx, (*tab.offset(i as isize)).atom);
        i = i.wrapping_add(1)
    }
    js_free(ctx, tab as *mut std::ffi::c_void);
    return envp;
}
/* execvpe is not available on non GNU systems */
unsafe extern "C" fn my_execvpe(
    mut filename: *const std::os::raw::c_char,
    mut argv: *mut *mut std::os::raw::c_char,
    mut envp: *mut *mut std::os::raw::c_char,
) -> i32 {
    let mut path: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut p_next: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut p1: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut buf: [std::os::raw::c_char; 4096] = [0; 4096];
    let mut filename_len: size_t = 0;
    let mut path_len: size_t = 0;
    let mut eacces_error: BOOL = 0;
    filename_len = strlen(filename);
    if filename_len == 0 as i32 as u64 {
        *__errno_location() = 2 as i32;
        return -(1 as i32);
    }
    if !strchr(filename, '/' as i32).is_null() {
        return execve(
            filename,
            argv as *const *mut std::os::raw::c_char,
            envp as *const *mut std::os::raw::c_char,
        );
    }
    path = getenv(b"PATH\x00" as *const u8 as *const std::os::raw::c_char);
    if path.is_null() {
        path = b"/bin:/usr/bin\x00" as *const u8 as *const std::os::raw::c_char
            as *mut std::os::raw::c_char
    }
    eacces_error = FALSE as i32;
    p = path;
    p = path;
    while !p.is_null() {
        p1 = strchr(p, ':' as i32);
        if p1.is_null() {
            p_next = 0 as *mut std::os::raw::c_char;
            path_len = strlen(p)
        } else {
            p_next = p1.offset(1 as i32 as isize);
            path_len = p1.wrapping_offset_from(p) as i64 as size_t
        }
        /* path too long */
        if !(path_len
            .wrapping_add(1 as i32 as u64)
            .wrapping_add(filename_len)
            .wrapping_add(1 as i32 as u64)
            > 4096 as i32 as u64)
        {
            memcpy(
                buf.as_mut_ptr() as *mut std::ffi::c_void,
                p as *const std::ffi::c_void,
                path_len,
            );
            buf[path_len as usize] = '/' as i32 as std::os::raw::c_char;
            memcpy(
                buf.as_mut_ptr()
                    .offset(path_len as isize)
                    .offset(1 as i32 as isize) as *mut std::ffi::c_void,
                filename as *const std::ffi::c_void,
                filename_len,
            );
            buf[path_len
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(filename_len) as usize] = '\u{0}' as i32 as std::os::raw::c_char;
            execve(
                buf.as_mut_ptr(),
                argv as *const *mut std::os::raw::c_char,
                envp as *const *mut std::os::raw::c_char,
            );
            match *__errno_location() {
                13 => eacces_error = TRUE as i32,
                2 | 20 => {}
                _ => return -(1 as i32),
            }
        }
        p = p_next
    }
    if eacces_error != 0 {
        *__errno_location() = 13 as i32
    }
    return -(1 as i32);
}
/* exec(args[, options]) -> exitcode */
unsafe extern "C" fn js_os_exec(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut current_block: u64;
    let mut options: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut args: JSValue = *argv.offset(0 as i32 as isize);
    let mut val: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut ret_val: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut exec_argv: *mut *const std::os::raw::c_char = 0 as *mut *const std::os::raw::c_char;
    let mut file: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut cwd: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut envp: *mut *mut std::os::raw::c_char = environ;
    let mut exec_argc: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut ret: i32 = 0;
    let mut pid: i32 = 0;
    let mut status: i32 = 0;
    let mut block_flag: BOOL = TRUE as i32;
    let mut use_path: BOOL = TRUE as i32;
    static mut std_name: [*const std::os::raw::c_char; 3] = [
        b"stdin\x00" as *const u8 as *const std::os::raw::c_char,
        b"stdout\x00" as *const u8 as *const std::os::raw::c_char,
        b"stderr\x00" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut std_fds: [i32; 3] = [0; 3];
    let mut uid: uint32_t = -(1 as i32) as uint32_t;
    let mut gid: uint32_t = -(1 as i32) as uint32_t;
    val = JS_GetPropertyStr(
        ctx,
        args,
        b"length\x00" as *const u8 as *const std::os::raw::c_char,
    );
    if JS_IsException(val) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = JS_ToUint32(ctx, &mut exec_argc, val);
    JS_FreeValue(ctx, val);
    if ret != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    /* arbitrary limit to avoid overflow */
    if exec_argc < 1 as i32 as u32 || exec_argc > 65535 as i32 as u32 {
        return JS_ThrowTypeError(
            ctx,
            b"invalid number of arguments\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    exec_argv = js_mallocz(
        ctx,
        (::std::mem::size_of::<*const std::os::raw::c_char>() as u64)
            .wrapping_mul(exec_argc.wrapping_add(1 as i32 as u32) as u64),
    ) as *mut *const std::os::raw::c_char;
    if exec_argv.is_null() {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    i = 0 as i32 as uint32_t;
    loop {
        if !(i < exec_argc) {
            current_block = 17478428563724192186;
            break;
        }
        val = JS_GetPropertyUint32(ctx, args, i);
        if JS_IsException(val) != 0 {
            current_block = 6626686957411452786;
            break;
        }
        str = JS_ToCString(ctx, val);
        JS_FreeValue(ctx, val);
        if str.is_null() {
            current_block = 6626686957411452786;
            break;
        }
        let ref mut fresh35 = *exec_argv.offset(i as isize);
        *fresh35 = str;
        i = i.wrapping_add(1)
    }
    match current_block {
        17478428563724192186 => {
            let ref mut fresh36 = *exec_argv.offset(exec_argc as isize);
            *fresh36 = 0 as *const std::os::raw::c_char;
            i = 0 as i32 as uint32_t;
            while i < 3 as i32 as u32 {
                std_fds[i as usize] = i as i32;
                i = i.wrapping_add(1)
            }
            /* get the options, if any */
            if argc >= 2 as i32 {
                options = *argv.offset(1 as i32 as isize);
                if get_bool_option(
                    ctx,
                    &mut block_flag,
                    options,
                    b"block\x00" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    current_block = 6626686957411452786;
                } else if get_bool_option(
                    ctx,
                    &mut use_path,
                    options,
                    b"usePath\x00" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    current_block = 6626686957411452786;
                } else {
                    val = JS_GetPropertyStr(
                        ctx,
                        options,
                        b"file\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                    if JS_IsException(val) != 0 {
                        current_block = 6626686957411452786;
                    } else {
                        if JS_IsUndefined(val) == 0 {
                            file = JS_ToCString(ctx, val);
                            JS_FreeValue(ctx, val);
                            if file.is_null() {
                                current_block = 6626686957411452786;
                            } else {
                                current_block = 7746103178988627676;
                            }
                        } else {
                            current_block = 7746103178988627676;
                        }
                        match current_block {
                            6626686957411452786 => {}
                            _ => {
                                val = JS_GetPropertyStr(
                                    ctx,
                                    options,
                                    b"cwd\x00" as *const u8 as *const std::os::raw::c_char,
                                );
                                if JS_IsException(val) != 0 {
                                    current_block = 6626686957411452786;
                                } else {
                                    if JS_IsUndefined(val) == 0 {
                                        cwd = JS_ToCString(ctx, val);
                                        JS_FreeValue(ctx, val);
                                        if cwd.is_null() {
                                            current_block = 6626686957411452786;
                                        } else {
                                            current_block = 6717214610478484138;
                                        }
                                    } else {
                                        current_block = 6717214610478484138;
                                    }
                                    match current_block {
                                        6626686957411452786 => {}
                                        _ =>
                                        /* stdin/stdout/stderr handles */
                                        {
                                            i = 0 as i32 as uint32_t;
                                            loop {
                                                if !(i < 3 as i32 as u32) {
                                                    current_block = 10753070352654377903;
                                                    break;
                                                }
                                                val = JS_GetPropertyStr(
                                                    ctx,
                                                    options,
                                                    std_name[i as usize],
                                                );
                                                if JS_IsException(val) != 0 {
                                                    current_block = 6626686957411452786;
                                                    break;
                                                }
                                                if JS_IsUndefined(val) == 0 {
                                                    let mut fd: i32 = 0;
                                                    ret = JS_ToInt32(ctx, &mut fd, val);
                                                    JS_FreeValue(ctx, val);
                                                    if ret != 0 {
                                                        current_block = 6626686957411452786;
                                                        break;
                                                    }
                                                    std_fds[i as usize] = fd
                                                }
                                                i = i.wrapping_add(1)
                                            }
                                            match current_block {
                                                6626686957411452786 => {}
                                                _ => {
                                                    val = JS_GetPropertyStr(
                                                        ctx,
                                                        options,
                                                        b"env\x00" as *const u8
                                                            as *const std::os::raw::c_char,
                                                    );
                                                    if JS_IsException(val) != 0 {
                                                        current_block = 6626686957411452786;
                                                    } else {
                                                        if JS_IsUndefined(val) == 0 {
                                                            envp = build_envp(ctx, val);
                                                            JS_FreeValue(ctx, val);
                                                            if envp.is_null() {
                                                                current_block = 6626686957411452786;
                                                            } else {
                                                                current_block = 3392087639489470149;
                                                            }
                                                        } else {
                                                            current_block = 3392087639489470149;
                                                        }
                                                        match current_block {
                                                            6626686957411452786 => {}
                                                            _ => {
                                                                val = JS_GetPropertyStr(
                                                                    ctx,
                                                                    options,
                                                                    b"uid\x00" as *const u8
                                                                        as *const std::os::raw::c_char,
                                                                );
                                                                if JS_IsException(val) != 0 {
                                                                    current_block =
                                                                        6626686957411452786;
                                                                } else {
                                                                    if JS_IsUndefined(val) == 0 {
                                                                        ret = JS_ToUint32(
                                                                            ctx, &mut uid, val,
                                                                        );
                                                                        JS_FreeValue(ctx, val);
                                                                        if ret != 0 {
                                                                            current_block =
                                                                                6626686957411452786;
                                                                        } else {
                                                                            current_block =
                                                                                6281126495347172768;
                                                                        }
                                                                    } else {
                                                                        current_block =
                                                                            6281126495347172768;
                                                                    }
                                                                    match current_block {
                                                                        6626686957411452786 => {}
                                                                        _ => {
                                                                            val
                                                                                =
                                                                                JS_GetPropertyStr(ctx,
                                                                                                  options,
                                                                                                  b"gid\x00"
                                                                                                      as
                                                                                                      *const u8
                                                                                                      as
                                                                                                      *const std::os::raw::c_char);
                                                                            if JS_IsException(val)
                                                                                != 0
                                                                            {
                                                                                current_block
                                                                                    =
                                                                                    6626686957411452786;
                                                                            } else if JS_IsUndefined(
                                                                                val,
                                                                            ) == 0
                                                                            {
                                                                                ret = JS_ToUint32(
                                                                                    ctx, &mut gid,
                                                                                    val,
                                                                                );
                                                                                JS_FreeValue(
                                                                                    ctx, val,
                                                                                );
                                                                                if ret != 0 {
                                                                                    current_block
                                                                                        =
                                                                                        6626686957411452786;
                                                                                } else {
                                                                                    current_block
                                                                                        =
                                                                                        919954187481050311;
                                                                                }
                                                                            } else {
                                                                                current_block
                                                                                    =
                                                                                    919954187481050311;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                current_block = 919954187481050311;
            }
            match current_block {
                6626686957411452786 => {}
                _ => {
                    pid = fork();
                    if pid < 0 as i32 {
                        JS_ThrowTypeError(
                            ctx,
                            b"fork error\x00" as *const u8 as *const std::os::raw::c_char,
                        );
                        current_block = 6626686957411452786;
                    } else {
                        if pid == 0 as i32 {
                            /* child */
                            let mut fd_max: i32 = sysconf(_SC_OPEN_MAX as i32) as i32;
                            /* remap the stdin/stdout/stderr handles if necessary */
                            i = 0 as i32 as uint32_t;
                            while i < 3 as i32 as u32 {
                                if std_fds[i as usize] as u32 != i {
                                    if dup2(std_fds[i as usize], i as i32) < 0 as i32 {
                                        _exit(127 as i32);
                                    }
                                }
                                i = i.wrapping_add(1)
                            }
                            i = 3 as i32 as uint32_t;
                            while i < fd_max as u32 {
                                close(i as i32);
                                i = i.wrapping_add(1)
                            }
                            if !cwd.is_null() {
                                if chdir(cwd) < 0 as i32 {
                                    _exit(127 as i32);
                                }
                            }
                            if uid != -(1 as i32) as u32 {
                                if setuid(uid) < 0 as i32 {
                                    _exit(127 as i32);
                                }
                            }
                            if gid != -(1 as i32) as u32 {
                                if setgid(gid) < 0 as i32 {
                                    _exit(127 as i32);
                                }
                            }
                            if file.is_null() {
                                file = *exec_argv.offset(0 as i32 as isize)
                            }
                            if use_path != 0 {
                                ret = my_execvpe(
                                    file,
                                    exec_argv as *mut *mut std::os::raw::c_char,
                                    envp,
                                )
                            } else {
                                ret = execve(
                                    file,
                                    exec_argv as *mut *mut std::os::raw::c_char
                                        as *const *mut std::os::raw::c_char,
                                    envp as *const *mut std::os::raw::c_char,
                                )
                            }
                            _exit(127 as i32);
                        }
                        /* parent */
                        if block_flag != 0 {
                            loop {
                                ret = waitpid(pid, &mut status, 0 as i32);
                                if !(ret == pid) {
                                    continue;
                                }
                                if status & 0x7f as i32 == 0 as i32 {
                                    ret = (status & 0xff00 as i32) >> 8 as i32;
                                    break;
                                } else {
                                    if !(((status & 0x7f as i32) + 1 as i32) as std::os::raw::c_char
                                        as i32
                                        >> 1 as i32
                                        > 0 as i32)
                                    {
                                        continue;
                                    }
                                    ret = -(status & 0x7f as i32);
                                    break;
                                }
                            }
                        } else {
                            ret = pid
                        }
                        ret_val = JS_NewInt32(ctx, ret);
                        current_block = 452399006379774259;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        6626686957411452786 => {
            ret_val = {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            }
        }
        _ => {}
    }
    JS_FreeCString(ctx, file);
    JS_FreeCString(ctx, cwd);
    i = 0 as i32 as uint32_t;
    while i < exec_argc {
        JS_FreeCString(ctx, *exec_argv.offset(i as isize));
        i = i.wrapping_add(1)
    }
    js_free(ctx, exec_argv as *mut std::ffi::c_void);
    if envp != environ {
        let mut p: *mut *mut std::os::raw::c_char = 0 as *mut *mut std::os::raw::c_char;
        p = envp;
        while !(*p).is_null() {
            js_free(ctx, *p as *mut std::ffi::c_void);
            p = p.offset(1)
        }
        js_free(ctx, envp as *mut std::ffi::c_void);
    }
    return ret_val;
}
/* waitpid(pid, block) -> [pid, status] */
unsafe extern "C" fn js_os_waitpid(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut pid: i32 = 0;
    let mut status: i32 = 0;
    let mut options: i32 = 0;
    let mut ret: i32 = 0;
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    if JS_ToInt32(ctx, &mut pid, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToInt32(ctx, &mut options, *argv.offset(1 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = waitpid(pid, &mut status, options);
    if ret < 0 as i32 {
        ret = -*__errno_location();
        status = 0 as i32
    }
    obj = JS_NewArray(ctx);
    if JS_IsException(obj) != 0 {
        return obj;
    }
    JS_DefinePropertyValueUint32(
        ctx,
        obj,
        0 as i32 as uint32_t,
        JS_NewInt32(ctx, ret),
        (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
    );
    JS_DefinePropertyValueUint32(
        ctx,
        obj,
        1 as i32 as uint32_t,
        JS_NewInt32(ctx, status),
        (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
    );
    return obj;
}
/* pipe() -> [read_fd, write_fd] or null if error */
unsafe extern "C" fn js_os_pipe(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut pipe_fds: [i32; 2] = [0; 2];
    let mut ret: i32 = 0;
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    ret = pipe(pipe_fds.as_mut_ptr());
    if ret < 0 as i32 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_NULL as i32 as int64_t,
            };
            init
        };
    }
    obj = JS_NewArray(ctx);
    if JS_IsException(obj) != 0 {
        return obj;
    }
    JS_DefinePropertyValueUint32(
        ctx,
        obj,
        0 as i32 as uint32_t,
        JS_NewInt32(ctx, pipe_fds[0 as i32 as usize]),
        (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
    );
    JS_DefinePropertyValueUint32(
        ctx,
        obj,
        1 as i32 as uint32_t,
        JS_NewInt32(ctx, pipe_fds[1 as i32 as usize]),
        (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32,
    );
    return obj;
}
/* kill(pid, sig) */
unsafe extern "C" fn js_os_kill(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut pid: i32 = 0;
    let mut sig: i32 = 0;
    let mut ret: i32 = 0;
    if JS_ToInt32(ctx, &mut pid, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToInt32(ctx, &mut sig, *argv.offset(1 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = js_get_errno(kill(pid, sig) as ssize_t) as i32;
    return JS_NewInt32(ctx, ret);
}
/* sleep(delay_ms) */
unsafe extern "C" fn js_os_sleep(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut delay: int64_t = 0;
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ret: i32 = 0;
    if JS_ToInt64(ctx, &mut delay, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ts.tv_sec = delay / 1000 as i32 as i64;
    ts.tv_nsec = delay % 1000 as i32 as i64 * 1000000 as i32 as i64;
    ret = js_get_errno(nanosleep(&mut ts, 0 as *mut timespec) as ssize_t) as i32;
    return JS_NewInt32(ctx, ret);
}
/* dup(fd) */
unsafe extern "C" fn js_os_dup(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut fd: i32 = 0;
    let mut ret: i32 = 0;
    if JS_ToInt32(ctx, &mut fd, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = js_get_errno(dup(fd) as ssize_t) as i32;
    return JS_NewInt32(ctx, ret);
}
/* dup2(fd) */
unsafe extern "C" fn js_os_dup2(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut fd: i32 = 0;
    let mut fd2: i32 = 0;
    let mut ret: i32 = 0;
    if JS_ToInt32(ctx, &mut fd, *argv.offset(0 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    if JS_ToInt32(ctx, &mut fd2, *argv.offset(1 as i32 as isize)) != 0 {
        return {
            let mut init = JSValue {
                u: JSValueUnion { int32: 0 as i32 },
                tag: JS_TAG_EXCEPTION as i32 as int64_t,
            };
            init
        };
    }
    ret = js_get_errno(dup2(fd, fd2) as ssize_t) as i32;
    return JS_NewInt32(ctx, ret);
}
/* !_WIN32 */
/* USE_WORKER */
#[no_mangle]
pub unsafe extern "C" fn js_std_set_worker_new_context_func(
    mut func: Option<unsafe extern "C" fn(_: *mut JSRuntime) -> *mut JSContext>,
) {
}
static mut js_os_funcs: [JSCFunctionListEntry; 68] = unsafe {
    [
        {
            let mut init = JSCFunctionListEntry {
                name: b"open\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_open
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"O_RDONLY\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 0 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"O_WRONLY\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 0o1 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"O_RDWR\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 0o2 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"O_APPEND\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o2000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"O_CREAT\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o100 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"O_EXCL\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o200 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"O_TRUNC\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o1000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"close\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_close
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"seek\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 3 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_seek
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"read\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 4 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_os_read_write
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"write\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 1 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 4 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_os_read_write
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"isatty\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_isatty
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"ttyGetWinSize\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_ttyGetWinSize
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"ttySetRaw\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_ttySetRaw
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"remove\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_remove
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"rename\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_rename
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"setReadHandler\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_os_setReadHandler
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"setWriteHandler\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 1 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_os_setReadHandler
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"signal\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_signal
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGINT\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 2 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGABRT\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 6 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGFPE\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 8 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGILL\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 4 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGSEGV\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 11 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGTERM\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 15 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGQUIT\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 3 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGPIPE\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 13 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGALRM\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 14 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGUSR1\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 10 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGUSR2\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 12 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGCHLD\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 17 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGCONT\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 18 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGSTOP\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 19 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGTSTP\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 20 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGTTIN\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 21 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SIGTTOU\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 22 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"setTimeout\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_setTimeout
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"clearTimeout\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_clearTimeout
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"platform\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: 0 as i32 as uint8_t,
                def_type: 3 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    str_0: b"linux\x00" as *const u8 as *const std::os::raw::c_char,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"getcwd\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_getcwd
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"chdir\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_chdir
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"mkdir\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_mkdir
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"readdir\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_readdir
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"S_IFMT\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o170000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"S_IFIFO\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o10000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"S_IFCHR\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o20000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"S_IFDIR\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o40000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"S_IFBLK\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o60000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"S_IFREG\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o100000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"S_IFSOCK\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o140000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"S_IFLNK\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o120000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"S_ISGID\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o2000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"S_ISUID\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    i32_0: 0o4000 as i32,
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"stat\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_os_stat
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"utimes\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 3 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_utimes
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"lstat\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 1 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_os_stat
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"realpath\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_realpath
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"symlink\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_symlink
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"readlink\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_readlink
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"exec\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_exec
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"waitpid\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_waitpid
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"WNOHANG\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 1 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"pipe\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_pipe
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"kill\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_kill
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"sleep\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_sleep
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"dup\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_dup
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"dup2\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_os_dup2
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
    ]
};
unsafe extern "C" fn js_os_init(mut ctx: *mut JSContext, mut m: *mut JSModuleDef) -> i32 {
    os_poll_func = Some(js_os_poll as unsafe extern "C" fn(_: *mut JSContext) -> i32);
    /* OSTimer class */
    JS_NewClassID(&mut js_os_timer_class_id);
    JS_NewClass(
        JS_GetRuntime(ctx),
        js_os_timer_class_id,
        &mut js_os_timer_class,
    );
    /* USE_WORKER */
    return JS_SetModuleExportList(
        ctx,
        m,
        js_os_funcs.as_ptr(),
        (::std::mem::size_of::<[JSCFunctionListEntry; 68]>() as u64)
            .wrapping_div(::std::mem::size_of::<JSCFunctionListEntry>() as u64) as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_init_module_os(
    mut ctx: *mut JSContext,
    mut module_name: *const std::os::raw::c_char,
) -> *mut JSModuleDef {
    let mut m: *mut JSModuleDef = 0 as *mut JSModuleDef;
    m = JS_NewCModule(
        ctx,
        module_name,
        Some(js_os_init as unsafe extern "C" fn(_: *mut JSContext, _: *mut JSModuleDef) -> i32),
    );
    if m.is_null() {
        return 0 as *mut JSModuleDef;
    }
    JS_AddModuleExportList(
        ctx,
        m,
        js_os_funcs.as_ptr(),
        (::std::mem::size_of::<[JSCFunctionListEntry; 68]>() as u64)
            .wrapping_div(::std::mem::size_of::<JSCFunctionListEntry>() as u64) as i32,
    );
    return m;
}
/* *********************************************************/
unsafe extern "C" fn js_print(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: i32,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut i: i32 = 0;
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut len: size_t = 0;
    i = 0 as i32;
    while i < argc {
        if i != 0 as i32 {
            putchar(' ' as i32);
        }
        str = JS_ToCStringLen(ctx, &mut len, *argv.offset(i as isize));
        if str.is_null() {
            return {
                let mut init = JSValue {
                    u: JSValueUnion { int32: 0 as i32 },
                    tag: JS_TAG_EXCEPTION as i32 as int64_t,
                };
                init
            };
        }
        fwrite(str as *const std::ffi::c_void, 1 as i32 as u64, len, stdout);
        JS_FreeCString(ctx, str);
        i += 1
    }
    putchar('\n' as i32);
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: 0 as i32 },
            tag: JS_TAG_UNDEFINED as i32 as int64_t,
        };
        init
    };
}
#[no_mangle]
pub unsafe extern "C" fn js_std_add_helpers(
    mut ctx: *mut JSContext,
    mut argc: i32,
    mut argv: *mut *mut std::os::raw::c_char,
) {
    let mut global_obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut console: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut args: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut i: i32 = 0;
    /* XXX: should these global definitions be enumerable? */
    global_obj = JS_GetGlobalObject(ctx);
    console = JS_NewObject(ctx);
    JS_SetPropertyStr(
        ctx,
        console,
        b"log\x00" as *const u8 as *const std::os::raw::c_char,
        JS_NewCFunction(
            ctx,
            Some(
                js_print
                    as unsafe extern "C" fn(
                        _: *mut JSContext,
                        _: JSValue,
                        _: i32,
                        _: *mut JSValue,
                    ) -> JSValue,
            ),
            b"log\x00" as *const u8 as *const std::os::raw::c_char,
            1 as i32,
        ),
    );
    JS_SetPropertyStr(
        ctx,
        global_obj,
        b"console\x00" as *const u8 as *const std::os::raw::c_char,
        console,
    );
    /* same methods as the mozilla JS shell */
    if argc >= 0 as i32 {
        args = JS_NewArray(ctx);
        i = 0 as i32;
        while i < argc {
            JS_SetPropertyUint32(
                ctx,
                args,
                i as uint32_t,
                JS_NewString(ctx, *argv.offset(i as isize)),
            );
            i += 1
        }
        JS_SetPropertyStr(
            ctx,
            global_obj,
            b"scriptArgs\x00" as *const u8 as *const std::os::raw::c_char,
            args,
        );
    }
    JS_SetPropertyStr(
        ctx,
        global_obj,
        b"print\x00" as *const u8 as *const std::os::raw::c_char,
        JS_NewCFunction(
            ctx,
            Some(
                js_print
                    as unsafe extern "C" fn(
                        _: *mut JSContext,
                        _: JSValue,
                        _: i32,
                        _: *mut JSValue,
                    ) -> JSValue,
            ),
            b"print\x00" as *const u8 as *const std::os::raw::c_char,
            1 as i32,
        ),
    );
    JS_SetPropertyStr(
        ctx,
        global_obj,
        b"__loadScript\x00" as *const u8 as *const std::os::raw::c_char,
        JS_NewCFunction(
            ctx,
            Some(
                js_loadScript
                    as unsafe extern "C" fn(
                        _: *mut JSContext,
                        _: JSValue,
                        _: i32,
                        _: *mut JSValue,
                    ) -> JSValue,
            ),
            b"__loadScript\x00" as *const u8 as *const std::os::raw::c_char,
            1 as i32,
        ),
    );
    JS_FreeValue(ctx, global_obj);
}
#[no_mangle]
pub unsafe extern "C" fn js_std_init_handlers(mut rt: *mut JSRuntime) {
    let mut ts: *mut JSThreadState = 0 as *mut JSThreadState;
    ts = malloc(::std::mem::size_of::<JSThreadState>() as u64) as *mut JSThreadState;
    if ts.is_null() {
        fprintf(
            stderr,
            b"Could not allocate memory for the worker\x00" as *const u8
                as *const std::os::raw::c_char,
        );
        exit(1 as i32);
    }
    memset(
        ts as *mut std::ffi::c_void,
        0 as i32,
        ::std::mem::size_of::<JSThreadState>() as u64,
    );
    init_list_head(&mut (*ts).os_rw_handlers);
    init_list_head(&mut (*ts).os_signal_handlers);
    init_list_head(&mut (*ts).os_timers);
    init_list_head(&mut (*ts).port_list);
    JS_SetRuntimeOpaque(rt, ts as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn js_std_free_handlers(mut rt: *mut JSRuntime) {
    let mut ts: *mut JSThreadState = JS_GetRuntimeOpaque(rt) as *mut JSThreadState;
    let mut el: *mut list_head = 0 as *mut list_head;
    let mut el1: *mut list_head = 0 as *mut list_head;
    el = (*ts).os_rw_handlers.next;
    el1 = (*el).next;
    while el != &mut (*ts).os_rw_handlers as *mut list_head {
        let mut rh: *mut JSOSRWHandler = (el as *mut uint8_t)
            .offset(-(&mut (*(0 as *mut JSOSRWHandler)).link as *mut list_head as size_t as isize))
            as *mut JSOSRWHandler;
        free_rw_handler(rt, rh);
        el = el1;
        el1 = (*el).next
    }
    el = (*ts).os_signal_handlers.next;
    el1 = (*el).next;
    while el != &mut (*ts).os_signal_handlers as *mut list_head {
        let mut sh: *mut JSOSSignalHandler = (el as *mut uint8_t).offset(
            -(&mut (*(0 as *mut JSOSSignalHandler)).link as *mut list_head as size_t as isize),
        ) as *mut JSOSSignalHandler;
        free_sh(rt, sh);
        el = el1;
        el1 = (*el).next
    }
    el = (*ts).os_timers.next;
    el1 = (*el).next;
    while el != &mut (*ts).os_timers as *mut list_head {
        let mut th: *mut JSOSTimer = (el as *mut uint8_t)
            .offset(-(&mut (*(0 as *mut JSOSTimer)).link as *mut list_head as size_t as isize))
            as *mut JSOSTimer;
        unlink_timer(rt, th);
        if (*th).has_object == 0 {
            free_timer(rt, th);
        }
        el = el1;
        el1 = (*el).next
    }
    free(ts as *mut std::ffi::c_void);
    JS_SetRuntimeOpaque(rt, 0 as *mut std::ffi::c_void);
    /* fail safe */
}
unsafe extern "C" fn js_dump_obj(mut ctx: *mut JSContext, mut f: *mut FILE, mut val: JSValue) {
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    str = JS_ToCString(ctx, val);
    if !str.is_null() {
        fprintf(
            f,
            b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
            str,
        );
        JS_FreeCString(ctx, str);
    } else {
        fprintf(
            f,
            b"[exception]\n\x00" as *const u8 as *const std::os::raw::c_char,
        );
    };
}
unsafe extern "C" fn js_std_dump_error1(mut ctx: *mut JSContext, mut exception_val: JSValue) {
    let mut val: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut is_error: BOOL = 0;
    is_error = JS_IsError(ctx, exception_val);
    js_dump_obj(ctx, stderr, exception_val);
    if is_error != 0 {
        val = JS_GetPropertyStr(
            ctx,
            exception_val,
            b"stack\x00" as *const u8 as *const std::os::raw::c_char,
        );
        if JS_IsUndefined(val) == 0 {
            js_dump_obj(ctx, stderr, val);
        }
        JS_FreeValue(ctx, val);
    };
}
#[no_mangle]
pub unsafe extern "C" fn js_std_dump_error(mut ctx: *mut JSContext) {
    let mut exception_val: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    exception_val = JS_GetException(ctx);
    js_std_dump_error1(ctx, exception_val);
    JS_FreeValue(ctx, exception_val);
}
#[no_mangle]
pub unsafe extern "C" fn js_std_promise_rejection_tracker(
    mut ctx: *mut JSContext,
    mut promise: JSValue,
    mut reason: JSValue,
    mut is_handled: BOOL,
    mut opaque: *mut std::ffi::c_void,
) {
    if is_handled == 0 {
        fprintf(
            stderr,
            b"Possibly unhandled promise rejection: \x00" as *const u8
                as *const std::os::raw::c_char,
        );
        js_std_dump_error1(ctx, reason);
    };
}
/* main loop which calls the user JS callbacks */
#[no_mangle]
pub unsafe extern "C" fn js_std_loop(mut ctx: *mut JSContext) {
    let mut ctx1: *mut JSContext = 0 as *mut JSContext;
    let mut err: i32 = 0;
    loop {
        loop
        /* execute the pending jobs */
        {
            err = JS_ExecutePendingJob(JS_GetRuntime(ctx), &mut ctx1);
            if !(err <= 0 as i32) {
                continue;
            }
            if err < 0 as i32 {
                js_std_dump_error(ctx1);
            }
            break;
        }
        if os_poll_func.is_none() || os_poll_func.expect("non-null function pointer")(ctx) != 0 {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_std_eval_binary(
    mut ctx: *mut JSContext,
    mut buf: *const uint8_t,
    mut buf_len: size_t,
    mut load_only: i32,
) {
    let mut current_block: u64;
    let mut obj: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut val: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    obj = JS_ReadObject(ctx, buf, buf_len, (1 as i32) << 0 as i32);
    if !(JS_IsException(obj) != 0) {
        if load_only != 0 {
            if obj.tag as int32_t == JS_TAG_MODULE as i32 {
                js_module_set_import_meta(ctx, obj, FALSE as i32, FALSE as i32);
            }
            current_block = 17833034027772472439;
        } else {
            if obj.tag as int32_t == JS_TAG_MODULE as i32 {
                if JS_ResolveModule(ctx, obj) < 0 as i32 {
                    JS_FreeValue(ctx, obj);
                    current_block = 18159705403766749827;
                } else {
                    js_module_set_import_meta(ctx, obj, FALSE as i32, TRUE as i32);
                    current_block = 7651349459974463963;
                }
            } else {
                current_block = 7651349459974463963;
            }
            match current_block {
                18159705403766749827 => {}
                _ => {
                    val = JS_EvalFunction(ctx, obj);
                    if JS_IsException(val) != 0 {
                        current_block = 18159705403766749827;
                    } else {
                        JS_FreeValue(ctx, val);
                        current_block = 17833034027772472439;
                    }
                }
            }
        }
        match current_block {
            18159705403766749827 => {}
            _ => {
                return;
            }
        }
    }
    js_std_dump_error(ctx);
    exit(1 as i32);
}
unsafe extern "C" fn run_static_initializers() {
    js_std_funcs = [
        {
            let mut init = JSCFunctionListEntry {
                name: b"exit\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_exit
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"gc\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_gc
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"evalScript\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_evalScript
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"loadScript\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_loadScript
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"getenv\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_getenv
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"setenv\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_setenv
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"unsetenv\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_unsetenv
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"getenviron\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_getenviron
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"urlGet\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_urlGet
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"loadFile\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_loadFile
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"strerror\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_strerror
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"parseExtJSON\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_parseExtJSON
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"open\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_open
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"popen\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_popen
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"fdopen\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 2 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_fdopen
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"tmpfile\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 0 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_tmpfile
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"puts\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic_magic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic_magic: Some(
                                    js_std_file_puts
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                            _: i32,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"printf\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_printf
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"sprintf\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 1 as i32 | (1 as i32) << 0 as i32) as uint8_t,
                def_type: 0 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    func: {
                        let mut init = C2RustUnnamed_6 {
                            length: 1 as i32 as uint8_t,
                            cproto: JS_CFUNC_generic as i32 as uint8_t,
                            cfunc: JSCFunctionType {
                                generic: Some(
                                    js_std_sprintf
                                        as unsafe extern "C" fn(
                                            _: *mut JSContext,
                                            _: JSValue,
                                            _: i32,
                                            _: *mut JSValue,
                                        )
                                            -> JSValue,
                                ),
                            },
                        };
                        init
                    },
                },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SEEK_SET\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 0 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SEEK_CUR\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 1 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"SEEK_END\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 4 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 { i32_0: 2 as i32 },
            };
            init
        },
        {
            let mut init = JSCFunctionListEntry {
                name: b"Error\x00" as *const u8 as *const std::os::raw::c_char,
                prop_flags: ((1 as i32) << 0 as i32) as uint8_t,
                def_type: 8 as i32 as uint8_t,
                magic: 0 as i32 as int16_t,
                u: C2RustUnnamed_2 {
                    prop_list: {
                        let mut init = C2RustUnnamed_3 {
                            tab: js_std_error_props.as_ptr(),
                            len: (::std::mem::size_of::<[JSCFunctionListEntry; 11]>() as u64)
                                .wrapping_div(::std::mem::size_of::<JSCFunctionListEntry>() as u64)
                                as i32,
                        };
                        init
                    },
                },
            };
            init
        },
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
