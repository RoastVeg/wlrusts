use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type udev;
    pub type udev_monitor;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_listener {
    pub link: wl_list,
    pub notify: wl_notify_func_t,
}
pub type wl_notify_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_listener, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct session_impl {
    pub create: Option<unsafe extern "C" fn(_: *mut wl_display)
                           -> *mut wlr_session>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_session) -> ()>,
    pub open: Option<unsafe extern "C" fn(_: *mut wlr_session,
                                          _: *const libc::c_char)
                         -> libc::c_int>,
    pub close: Option<unsafe extern "C" fn(_: *mut wlr_session,
                                           _: libc::c_int) -> ()>,
    pub change_vt: Option<unsafe extern "C" fn(_: *mut wlr_session,
                                               _: libc::c_uint) -> bool>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_session {
    pub impl_0: *const session_impl,
    pub session_signal: wl_signal,
    pub active: bool,
    pub vtnr: libc::c_uint,
    pub seat: [libc::c_char; 256],
    pub udev: *mut udev,
    pub mon: *mut udev_monitor,
    pub udev_event: *mut wl_event_source,
    pub devices: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
}
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
unsafe extern "C" fn noop_session_open(mut base: *mut wlr_session,
                                       mut path: *const libc::c_char)
 -> libc::c_int {
    return open(path, 0o2i32 | 0o2000000i32);
}
unsafe extern "C" fn noop_session_close(mut base: *mut wlr_session,
                                        mut fd: libc::c_int) {
    close(fd);
}
unsafe extern "C" fn noop_change_vt(mut base: *mut wlr_session,
                                    mut vt: libc::c_uint) -> bool {
    return 0i32 != 0;
}
unsafe extern "C" fn noop_session_destroy(mut base: *mut wlr_session) {
    free(base as *mut libc::c_void);
}
unsafe extern "C" fn noop_session_create(mut disp: *mut wl_display)
 -> *mut wlr_session {
    let mut session: *mut wlr_session =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_session>() as libc::c_ulong) as
            *mut wlr_session;
    if session.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/noop.c\x00" as *const u8 as
                     *const libc::c_char, 32i32,
                 strerror(*__errno_location()));
        return 0 as *mut wlr_session
    }
    (*session).impl_0 = &session_noop;
    _wlr_log(WLR_INFO,
             b"[%s:%d] Successfully initialized noop session\x00" as *const u8
                 as *const libc::c_char,
             b"../backend/session/noop.c\x00" as *const u8 as
                 *const libc::c_char, 38i32);
    return session;
}
#[no_mangle]
pub static mut session_noop: session_impl =
    unsafe {
        {
            let mut init =
                session_impl{create:
                                 Some(noop_session_create as
                                          unsafe extern "C" fn(_:
                                                                   *mut wl_display)
                                              -> *mut wlr_session),
                             destroy:
                                 Some(noop_session_destroy as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session)
                                              -> ()),
                             open:
                                 Some(noop_session_open as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session,
                                                               _:
                                                                   *const libc::c_char)
                                              -> libc::c_int),
                             close:
                                 Some(noop_session_close as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session,
                                                               _: libc::c_int)
                                              -> ()),
                             change_vt:
                                 Some(noop_change_vt as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session,
                                                               _:
                                                                   libc::c_uint)
                                              -> bool),};
            init
        }
    };
