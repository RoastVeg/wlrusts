use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_log_set_handler_server(handler: wl_log_func_t);
}
pub type __builtin_va_list = [__va_list_tag; 1];

#[repr(C)]#[derive(Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
pub type wl_log_func_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char, _: ::std::ffi::VaList)
               -> ()>;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_listener {
    pub link: wl_list,
    pub notify: wl_notify_func_t,
}
pub type wl_notify_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_listener, _: *mut libc::c_void)
               -> ()>;
/*
 * This is a stable interface of wlroots. Future changes will be limited to:
 *
 * - New functions
 * - New struct members
 * - New enum members
 *
 * Note that wlroots does not make an ABI compatibility promise - in the future,
 * the layout and size of structs used by wlroots may change, requiring code
 * depending on this header to be recompiled (but not edited).
 *
 * Breaking changes are announced by email and follow a 1-year deprecation
 * schedule. Send an email to ~sircmpwn/wlroots-announce+subscribe@lists.sr.ht
 * to receive these announcements.
 */
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
pub type wlr_log_func_t
    =
    Option<unsafe extern "C" fn(_: wlr_log_importance, _: *const libc::c_char,
                                _: ::std::ffi::VaList) -> ()>;
// for snprintf
static mut colored: bool = 1i32 != 0;
static mut log_importance: wlr_log_importance = WLR_ERROR;
static mut verbosity_colors: [*const libc::c_char; 4] =
    [b"\x00" as *const u8 as *const libc::c_char,
     b"\x1b[1;31m\x00" as *const u8 as *const libc::c_char,
     b"\x1b[1;34m\x00" as *const u8 as *const libc::c_char,
     b"\x1b[1;30m\x00" as *const u8 as *const libc::c_char];
unsafe extern "C" fn log_stderr(mut verbosity: wlr_log_importance,
                                mut fmt: *const libc::c_char,
                                mut args: ::std::ffi::VaList) {
    if verbosity as libc::c_uint > log_importance as libc::c_uint { return }
    // prefix the time to the log message
    let mut result: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           __tm_gmtoff: 0,
           __tm_zone: 0 as *const libc::c_char,};
    let mut t: time_t = time(0 as *mut time_t);
    let mut tm_info: *mut tm = localtime_r(&mut t, &mut result);
    let mut buffer: [libc::c_char; 26] = [0; 26];
    // generate time prefix
    strftime(buffer.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong,
             b"%F %T - \x00" as *const u8 as *const libc::c_char, tm_info);
    fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
            buffer.as_mut_ptr());
    let mut c: libc::c_uint =
        if (verbosity as libc::c_uint) <
               WLR_LOG_IMPORTANCE_LAST as libc::c_int as libc::c_uint {
            verbosity as libc::c_uint
        } else {
            (WLR_LOG_IMPORTANCE_LAST as libc::c_int - 1i32) as libc::c_uint
        };
    if colored as libc::c_int != 0 && isatty(2i32) != 0 {
        fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                verbosity_colors[c as usize]);
    }
    vfprintf(stderr, fmt, args.as_va_list());
    if colored as libc::c_int != 0 && isatty(2i32) != 0 {
        fprintf(stderr, b"\x1b[0m\x00" as *const u8 as *const libc::c_char);
    }
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
}
static mut log_callback: wlr_log_func_t =
    {
    
        Some(log_stderr as
                 unsafe extern "C" fn(_: wlr_log_importance,
                                      _: *const libc::c_char,
                                      _: ::std::ffi::VaList) -> ())
};
unsafe extern "C" fn log_wl(mut fmt: *const libc::c_char,
                            mut args: ::std::ffi::VaList) {
    static mut wlr_fmt: [libc::c_char; 1024] = [0; 1024];
    let mut n: libc::c_int =
        snprintf(wlr_fmt.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong,
                 b"[wayland] %s\x00" as *const u8 as *const libc::c_char,
                 fmt);
    if n > 0i32 && wlr_fmt[(n - 1i32) as usize] as libc::c_int == '\n' as i32
       {
        wlr_fmt[(n - 1i32) as usize] = '\u{0}' as i32 as libc::c_char
    }
    _wlr_vlog(WLR_INFO, wlr_fmt.as_mut_ptr(), args.as_va_list());
}
// Will log all messages less than or equal to `verbosity`
// If `callback` is NULL, wlr will use its default logger.
// The function can be called multiple times to update the verbosity or
// callback function.
#[no_mangle]
pub unsafe extern "C" fn wlr_log_init(mut verbosity: wlr_log_importance,
                                      mut callback: wlr_log_func_t) {
    if (verbosity as libc::c_uint) <
           WLR_LOG_IMPORTANCE_LAST as libc::c_int as libc::c_uint {
        log_importance = verbosity
    }
    if callback.is_some() { log_callback = callback }
    wl_log_set_handler_server(Some(log_wl as
                                       unsafe extern "C" fn(_:
                                                                *const libc::c_char,
                                                            _:
                                                                ::std::ffi::VaList)
                                           -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn _wlr_vlog(mut verbosity: wlr_log_importance,
                                   mut fmt: *const libc::c_char,
                                   mut args: ::std::ffi::VaList) {
    log_callback.expect("non-null function pointer")(verbosity, fmt,
                                                     args.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn _wlr_log(mut verbosity: wlr_log_importance,
                                  mut fmt: *const libc::c_char,
                                  mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    log_callback.expect("non-null function pointer")(verbosity, fmt,
                                                     args_0.as_va_list());
}
// Returns the log verbosity provided to wlr_log_init
#[no_mangle]
pub unsafe extern "C" fn wlr_log_get_verbosity() -> wlr_log_importance {
    return log_importance;
}
