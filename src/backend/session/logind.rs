use libc;
extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type udev;
    pub type udev_monitor;
    pub type sd_bus;
    pub type sd_bus_message;
    pub type sd_bus_slot;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;
    #[no_mangle]
    fn gnu_dev_minor(__dev: __dev_t) -> libc::c_uint;
    #[no_mangle]
    fn gnu_dev_makedev(__major: libc::c_uint, __minor: libc::c_uint)
     -> __dev_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn getuid() -> __uid_t;
    #[no_mangle]
    fn wl_event_loop_add_fd(loop_0: *mut wl_event_loop, fd: libc::c_int,
                            mask: uint32_t, func: wl_event_loop_fd_func_t,
                            data: *mut libc::c_void) -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_event_loop_dispatch(loop_0: *mut wl_event_loop,
                              timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
    #[no_mangle]
    fn sd_bus_default_system(ret: *mut *mut sd_bus) -> libc::c_int;
    #[no_mangle]
    fn sd_bus_unref(bus: *mut sd_bus) -> *mut sd_bus;
    #[no_mangle]
    fn sd_bus_get_fd(bus: *mut sd_bus) -> libc::c_int;
    #[no_mangle]
    fn sd_bus_process(bus: *mut sd_bus, r: *mut *mut sd_bus_message)
     -> libc::c_int;
    #[no_mangle]
    fn sd_bus_message_unref(m: *mut sd_bus_message) -> *mut sd_bus_message;
    #[no_mangle]
    fn sd_bus_message_read(m: *mut sd_bus_message, types: *const libc::c_char,
                           _: ...) -> libc::c_int;
    #[no_mangle]
    fn sd_bus_message_read_basic(m: *mut sd_bus_message, type_0: libc::c_char,
                                 p: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn sd_bus_message_skip(m: *mut sd_bus_message, types: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn sd_bus_message_enter_container(m: *mut sd_bus_message,
                                      type_0: libc::c_char,
                                      contents: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn sd_bus_message_exit_container(m: *mut sd_bus_message) -> libc::c_int;
    #[no_mangle]
    fn sd_bus_call_method(bus: *mut sd_bus, destination: *const libc::c_char,
                          path: *const libc::c_char,
                          interface: *const libc::c_char,
                          member: *const libc::c_char,
                          ret_error: *mut sd_bus_error,
                          reply: *mut *mut sd_bus_message,
                          types: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sd_bus_get_property_trivial(bus: *mut sd_bus,
                                   destination: *const libc::c_char,
                                   path: *const libc::c_char,
                                   interface: *const libc::c_char,
                                   member: *const libc::c_char,
                                   ret_error: *mut sd_bus_error,
                                   type_0: libc::c_char,
                                   ret_ptr: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn sd_bus_match_signal(bus: *mut sd_bus, ret: *mut *mut sd_bus_slot,
                           sender: *const libc::c_char,
                           path: *const libc::c_char,
                           interface: *const libc::c_char,
                           member: *const libc::c_char,
                           callback: sd_bus_message_handler_t,
                           userdata: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn sd_bus_error_free(e: *mut sd_bus_error);
    #[no_mangle]
    fn sd_pid_get_session(pid: pid_t, session: *mut *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn sd_uid_get_display(uid: uid_t, session: *mut *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn sd_uid_get_sessions(uid: uid_t, require_active: libc::c_int,
                           sessions: *mut *mut *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn sd_session_is_active(session: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sd_session_get_state(session: *const libc::c_char,
                            state: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sd_session_get_seat(session: *const libc::c_char,
                           seat: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sd_session_get_type(session: *const libc::c_char,
                           type_0: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sd_session_get_class(session: *const libc::c_char,
                            clazz: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sd_session_get_vt(session: *const libc::c_char,
                         vtnr: *mut libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn sd_seat_can_graphical(seat: *const libc::c_char) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type dev_t = __dev_t;
pub type uid_t = __uid_t;
pub type uint32_t = __uint32_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
pub type C2RustUnnamed = libc::c_uint;
pub const WL_EVENT_ERROR: C2RustUnnamed = 8;
pub const WL_EVENT_HANGUP: C2RustUnnamed = 4;
pub const WL_EVENT_WRITABLE: C2RustUnnamed = 2;
pub const WL_EVENT_READABLE: C2RustUnnamed = 1;
pub type wl_event_loop_fd_func_t
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: uint32_t,
                                _: *mut libc::c_void) -> libc::c_int>;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_listener {
    pub link: wl_list,
    pub notify: wl_notify_func_t,
}
pub type wl_notify_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_listener, _: *mut libc::c_void)
               -> ()>;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_signal {
    pub listener_list: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_0,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_device {
    pub fd: libc::c_int,
    pub dev: dev_t,
    pub signal: wl_signal,
    pub link: wl_list,
}
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct sd_bus_error {
    pub name: *const libc::c_char,
    pub message: *const libc::c_char,
    pub _need_free: libc::c_int,
}
pub type sd_bus_message_handler_t
    =
    Option<unsafe extern "C" fn(_: *mut sd_bus_message, _: *mut libc::c_void,
                                _: *mut sd_bus_error) -> libc::c_int>;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const DRM_MAJOR: C2RustUnnamed_1 = 226;

#[repr(C)]#[derive(Copy, Clone)]
pub struct logind_session {
    pub base: wlr_session,
    pub bus: *mut sd_bus,
    pub event: *mut wl_event_source,
    pub id: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub seat_path: *mut libc::c_char,
    pub can_graphical: bool,
    pub has_drm: bool,
}
unsafe extern "C" fn logind_session_from_session(mut base: *mut wlr_session)
 -> *mut logind_session {
    if (*base).impl_0 == &session_logind as *const session_impl {
    } else {
        __assert_fail(b"base->impl == &session_logind\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/session/logind.c\x00" as *const u8 as
                          *const libc::c_char, 49i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"struct logind_session *logind_session_from_session(struct wlr_session *)\x00")).as_ptr());
    };
    return base as *mut logind_session;
}
unsafe extern "C" fn logind_take_device(mut base: *mut wlr_session,
                                        mut path: *const libc::c_char)
 -> libc::c_int {
    let mut paused: libc::c_int = 0;
    let mut session: *mut logind_session = logind_session_from_session(base);
    let mut fd: libc::c_int = -1i32;
    let mut ret: libc::c_int = 0;
    let mut msg: *mut sd_bus_message = 0 as *mut sd_bus_message;
    let mut error: sd_bus_error =
        {
            let mut init =
                sd_bus_error{name: 0 as *const libc::c_char,
                             message: 0 as *const libc::c_char,
                             _need_free: 0i32,};
            init
        };
    let mut st: stat =
        stat{st_dev: 0,
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
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if stat(path, &mut st) < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to stat \'%s\'\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 63i32, path);
        return -1i32
    }
    if gnu_dev_major(st.st_rdev) == DRM_MAJOR as libc::c_int as libc::c_uint {
        (*session).has_drm = 1i32 != 0
    }
    ret =
        sd_bus_call_method((*session).bus,
                           b"org.freedesktop.login1\x00" as *const u8 as
                               *const libc::c_char, (*session).path,
                           b"org.freedesktop.login1.Session\x00" as *const u8
                               as *const libc::c_char,
                           b"TakeDevice\x00" as *const u8 as
                               *const libc::c_char,
                           &mut error as *mut sd_bus_error,
                           &mut msg as *mut *mut sd_bus_message,
                           b"uu\x00" as *const u8 as *const libc::c_char,
                           gnu_dev_major(st.st_rdev),
                           gnu_dev_minor(st.st_rdev));
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to take device \'%s\': %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 76i32, path, error.message);
    } else {
        paused = 0i32;
        ret =
            sd_bus_message_read(msg,
                                b"hb\x00" as *const u8 as *const libc::c_char,
                                &mut fd as *mut libc::c_int,
                                &mut paused as *mut libc::c_int);
        if ret < 0i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to parse D-Bus response for \'%s\': %s\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/session/logind.c\x00" as *const u8 as
                         *const libc::c_char, 84i32, path, strerror(-ret));
        } else {
            // The original fd seems to be closed when the message is freed
	// so we just clone it.
            fd = fcntl(fd, 1030i32, 0i32);
            if fd < 0i32 {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to clone file descriptor for \'%s\': %s\x00"
                             as *const u8 as *const libc::c_char,
                         b"../backend/session/logind.c\x00" as *const u8 as
                             *const libc::c_char, 93i32, path,
                         strerror(*__errno_location()));
            }
        }
    }
    sd_bus_error_free(&mut error);
    sd_bus_message_unref(msg);
    return fd;
}
unsafe extern "C" fn logind_release_device(mut base: *mut wlr_session,
                                           mut fd: libc::c_int) {
    let mut session: *mut logind_session = logind_session_from_session(base);
    let mut st: stat =
        stat{st_dev: 0,
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
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if fstat(fd, &mut st) < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to stat device \'%d\': %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 109i32, fd,
                 strerror(*__errno_location()));
        return
    }
    let mut msg: *mut sd_bus_message = 0 as *mut sd_bus_message;
    let mut error: sd_bus_error =
        {
            let mut init =
                sd_bus_error{name: 0 as *const libc::c_char,
                             message: 0 as *const libc::c_char,
                             _need_free: 0i32,};
            init
        };
    let mut ret: libc::c_int =
        sd_bus_call_method((*session).bus,
                           b"org.freedesktop.login1\x00" as *const u8 as
                               *const libc::c_char, (*session).path,
                           b"org.freedesktop.login1.Session\x00" as *const u8
                               as *const libc::c_char,
                           b"ReleaseDevice\x00" as *const u8 as
                               *const libc::c_char,
                           &mut error as *mut sd_bus_error,
                           &mut msg as *mut *mut sd_bus_message,
                           b"uu\x00" as *const u8 as *const libc::c_char,
                           gnu_dev_major(st.st_rdev),
                           gnu_dev_minor(st.st_rdev));
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to release device \'%d\': %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 120i32, fd, error.message);
    }
    sd_bus_error_free(&mut error);
    sd_bus_message_unref(msg);
    close(fd);
}
unsafe extern "C" fn logind_change_vt(mut base: *mut wlr_session,
                                      mut vt: libc::c_uint) -> bool {
    let mut session: *mut logind_session = logind_session_from_session(base);
    // Only seat0 has VTs associated with it
    if strcmp((*session).base.seat.as_mut_ptr(),
              b"seat0\x00" as *const u8 as *const libc::c_char) != 0i32 {
        return 1i32 != 0
    }
    let mut ret: libc::c_int = 0;
    let mut msg: *mut sd_bus_message = 0 as *mut sd_bus_message;
    let mut error: sd_bus_error =
        {
            let mut init =
                sd_bus_error{name: 0 as *const libc::c_char,
                             message: 0 as *const libc::c_char,
                             _need_free: 0i32,};
            init
        };
    ret =
        sd_bus_call_method((*session).bus,
                           b"org.freedesktop.login1\x00" as *const u8 as
                               *const libc::c_char,
                           b"/org/freedesktop/login1/seat/seat0\x00" as
                               *const u8 as *const libc::c_char,
                           b"org.freedesktop.login1.Seat\x00" as *const u8 as
                               *const libc::c_char,
                           b"SwitchTo\x00" as *const u8 as
                               *const libc::c_char,
                           &mut error as *mut sd_bus_error,
                           &mut msg as *mut *mut sd_bus_message,
                           b"u\x00" as *const u8 as *const libc::c_char, vt);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to change to vt \'%d\'\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 144i32, vt);
    }
    sd_bus_error_free(&mut error);
    sd_bus_message_unref(msg);
    return ret >= 0i32;
}
unsafe extern "C" fn find_session_path(mut session: *mut logind_session)
 -> bool {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut msg: *mut sd_bus_message = 0 as *mut sd_bus_message;
    let mut error: sd_bus_error =
        {
            let mut init =
                sd_bus_error{name: 0 as *const libc::c_char,
                             message: 0 as *const libc::c_char,
                             _need_free: 0i32,};
            init
        };
    ret =
        sd_bus_call_method((*session).bus,
                           b"org.freedesktop.login1\x00" as *const u8 as
                               *const libc::c_char,
                           b"/org/freedesktop/login1\x00" as *const u8 as
                               *const libc::c_char,
                           b"org.freedesktop.login1.Manager\x00" as *const u8
                               as *const libc::c_char,
                           b"GetSession\x00" as *const u8 as
                               *const libc::c_char,
                           &mut error as *mut sd_bus_error,
                           &mut msg as *mut *mut sd_bus_message,
                           b"s\x00" as *const u8 as *const libc::c_char,
                           (*session).id);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to get session path: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 161i32, error.message);
    } else {
        path = 0 as *const libc::c_char;
        ret =
            sd_bus_message_read(msg,
                                b"o\x00" as *const u8 as *const libc::c_char,
                                &mut path as *mut *const libc::c_char);
        if ret < 0i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Could not parse session path: %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/session/logind.c\x00" as *const u8 as
                         *const libc::c_char, 168i32, error.message);
        } else { (*session).path = strdup(path) }
    }
    sd_bus_error_free(&mut error);
    sd_bus_message_unref(msg);
    return ret >= 0i32;
}
unsafe extern "C" fn find_seat_path(mut session: *mut logind_session)
 -> bool {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut msg: *mut sd_bus_message = 0 as *mut sd_bus_message;
    let mut error: sd_bus_error =
        {
            let mut init =
                sd_bus_error{name: 0 as *const libc::c_char,
                             message: 0 as *const libc::c_char,
                             _need_free: 0i32,};
            init
        };
    ret =
        sd_bus_call_method((*session).bus,
                           b"org.freedesktop.login1\x00" as *const u8 as
                               *const libc::c_char,
                           b"/org/freedesktop/login1\x00" as *const u8 as
                               *const libc::c_char,
                           b"org.freedesktop.login1.Manager\x00" as *const u8
                               as *const libc::c_char,
                           b"GetSeat\x00" as *const u8 as *const libc::c_char,
                           &mut error as *mut sd_bus_error,
                           &mut msg as *mut *mut sd_bus_message,
                           b"s\x00" as *const u8 as *const libc::c_char,
                           (*session).base.seat.as_mut_ptr());
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to get seat path: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 189i32, error.message);
    } else {
        path = 0 as *const libc::c_char;
        ret =
            sd_bus_message_read(msg,
                                b"o\x00" as *const u8 as *const libc::c_char,
                                &mut path as *mut *const libc::c_char);
        if ret < 0i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Could not parse seat path: %s\x00" as *const u8
                         as *const libc::c_char,
                     b"../backend/session/logind.c\x00" as *const u8 as
                         *const libc::c_char, 196i32, error.message);
        } else { (*session).seat_path = strdup(path) }
    }
    sd_bus_error_free(&mut error);
    sd_bus_message_unref(msg);
    return ret >= 0i32;
}
unsafe extern "C" fn session_activate(mut session: *mut logind_session)
 -> bool {
    let mut ret: libc::c_int = 0;
    let mut msg: *mut sd_bus_message = 0 as *mut sd_bus_message;
    let mut error: sd_bus_error =
        {
            let mut init =
                sd_bus_error{name: 0 as *const libc::c_char,
                             message: 0 as *const libc::c_char,
                             _need_free: 0i32,};
            init
        };
    ret =
        sd_bus_call_method((*session).bus,
                           b"org.freedesktop.login1\x00" as *const u8 as
                               *const libc::c_char, (*session).path,
                           b"org.freedesktop.login1.Session\x00" as *const u8
                               as *const libc::c_char,
                           b"Activate\x00" as *const u8 as
                               *const libc::c_char,
                           &mut error as *mut sd_bus_error,
                           &mut msg as *mut *mut sd_bus_message,
                           b"\x00" as *const u8 as *const libc::c_char);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to activate session: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 217i32, error.message);
    }
    sd_bus_error_free(&mut error);
    sd_bus_message_unref(msg);
    return ret >= 0i32;
}
unsafe extern "C" fn take_control(mut session: *mut logind_session) -> bool {
    let mut ret: libc::c_int = 0;
    let mut msg: *mut sd_bus_message = 0 as *mut sd_bus_message;
    let mut error: sd_bus_error =
        {
            let mut init =
                sd_bus_error{name: 0 as *const libc::c_char,
                             message: 0 as *const libc::c_char,
                             _need_free: 0i32,};
            init
        };
    ret =
        sd_bus_call_method((*session).bus,
                           b"org.freedesktop.login1\x00" as *const u8 as
                               *const libc::c_char, (*session).path,
                           b"org.freedesktop.login1.Session\x00" as *const u8
                               as *const libc::c_char,
                           b"TakeControl\x00" as *const u8 as
                               *const libc::c_char,
                           &mut error as *mut sd_bus_error,
                           &mut msg as *mut *mut sd_bus_message,
                           b"b\x00" as *const u8 as *const libc::c_char,
                           0i32);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to take control of session: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 235i32, error.message);
    }
    sd_bus_error_free(&mut error);
    sd_bus_message_unref(msg);
    return ret >= 0i32;
}
unsafe extern "C" fn release_control(mut session: *mut logind_session) {
    let mut ret: libc::c_int = 0;
    let mut msg: *mut sd_bus_message = 0 as *mut sd_bus_message;
    let mut error: sd_bus_error =
        {
            let mut init =
                sd_bus_error{name: 0 as *const libc::c_char,
                             message: 0 as *const libc::c_char,
                             _need_free: 0i32,};
            init
        };
    ret =
        sd_bus_call_method((*session).bus,
                           b"org.freedesktop.login1\x00" as *const u8 as
                               *const libc::c_char, (*session).path,
                           b"org.freedesktop.login1.Session\x00" as *const u8
                               as *const libc::c_char,
                           b"ReleaseControl\x00" as *const u8 as
                               *const libc::c_char,
                           &mut error as *mut sd_bus_error,
                           &mut msg as *mut *mut sd_bus_message,
                           b"\x00" as *const u8 as *const libc::c_char);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to release control of session: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 253i32, error.message);
    }
    sd_bus_error_free(&mut error);
    sd_bus_message_unref(msg);
}
unsafe extern "C" fn logind_session_destroy(mut base: *mut wlr_session) {
    let mut session: *mut logind_session = logind_session_from_session(base);
    release_control(session);
    wl_event_source_remove((*session).event);
    sd_bus_unref((*session).bus);
    free((*session).id as *mut libc::c_void);
    free((*session).path as *mut libc::c_void);
    free((*session).seat_path as *mut libc::c_void);
    free(session as *mut libc::c_void);
}
unsafe extern "C" fn session_removed(mut msg: *mut sd_bus_message,
                                     mut userdata: *mut libc::c_void,
                                     mut ret_error: *mut sd_bus_error)
 -> libc::c_int {
    _wlr_log(WLR_INFO,
             b"[%s:%d] SessionRemoved signal received\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/session/logind.c\x00" as *const u8 as
                 *const libc::c_char, 275i32);
    return 0i32;
}
unsafe extern "C" fn find_device(mut session: *mut wlr_session,
                                 mut devnum: dev_t) -> *mut wlr_device {
    let mut dev: *mut wlr_device = 0 as *mut wlr_device;
    dev =
        ((*session).devices.next as *mut libc::c_char).offset(-32) as
            *mut wlr_device;
    while &mut (*dev).link as *mut wl_list !=
              &mut (*session).devices as *mut wl_list {
        if (*dev).dev == devnum { return dev }
        dev =
            ((*dev).link.next as *mut libc::c_char).offset(-32) as
                *mut wlr_device
    }
    _wlr_log(WLR_ERROR,
             b"[%s:%d] Tried to use dev_t %lu not opened by session\x00" as
                 *const u8 as *const libc::c_char,
             b"../backend/session/logind.c\x00" as *const u8 as
                 *const libc::c_char, 290i32, devnum);
    if 0i32 != 0 {
    } else {
        __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                      b"../backend/session/logind.c\x00" as *const u8 as
                          *const libc::c_char, 291i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 60],
                                                &[libc::c_char; 60]>(b"struct wlr_device *find_device(struct wlr_session *, dev_t)\x00")).as_ptr());
    };
    return 0 as *mut wlr_device;
}
unsafe extern "C" fn pause_device(mut msg: *mut sd_bus_message,
                                  mut userdata: *mut libc::c_void,
                                  mut ret_error: *mut sd_bus_error)
 -> libc::c_int {
    let mut session: *mut logind_session = userdata as *mut logind_session;
    let mut ret: libc::c_int = 0;
    let mut major: uint32_t = 0;
    let mut minor: uint32_t = 0;
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    ret =
        sd_bus_message_read(msg,
                            b"uus\x00" as *const u8 as *const libc::c_char,
                            &mut major as *mut uint32_t,
                            &mut minor as *mut uint32_t,
                            &mut type_0 as *mut *const libc::c_char);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to parse D-Bus response for PauseDevice: %s\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 305i32, strerror(-ret));
    } else {
        if major == DRM_MAJOR as libc::c_int as libc::c_uint &&
               strcmp(type_0, b"gone\x00" as *const u8 as *const libc::c_char)
                   != 0i32 {
            if (*session).has_drm as libc::c_int != 0 {
            } else {
                __assert_fail(b"session->has_drm\x00" as *const u8 as
                                  *const libc::c_char,
                              b"../backend/session/logind.c\x00" as *const u8
                                  as *const libc::c_char,
                              310i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 59],
                                                        &[libc::c_char; 59]>(b"int pause_device(sd_bus_message *, void *, sd_bus_error *)\x00")).as_ptr());
            };
            (*session).base.active = 0i32 != 0;
            wlr_signal_emit_safe(&mut (*session).base.session_signal,
                                 session as *mut libc::c_void);
        }
        if strcmp(type_0, b"pause\x00" as *const u8 as *const libc::c_char) ==
               0i32 {
            ret =
                sd_bus_call_method((*session).bus,
                                   b"org.freedesktop.login1\x00" as *const u8
                                       as *const libc::c_char,
                                   (*session).path,
                                   b"org.freedesktop.login1.Session\x00" as
                                       *const u8 as *const libc::c_char,
                                   b"PauseDeviceComplete\x00" as *const u8 as
                                       *const libc::c_char, ret_error,
                                   &mut msg as *mut *mut sd_bus_message,
                                   b"uu\x00" as *const u8 as
                                       *const libc::c_char, major, minor);
            if ret < 0i32 {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to send PauseDeviceComplete signal: %s\x00"
                             as *const u8 as *const libc::c_char,
                         b"../backend/session/logind.c\x00" as *const u8 as
                             *const libc::c_char, 321i32, strerror(-ret));
            }
        }
    }
    return 0i32;
}
unsafe extern "C" fn resume_device(mut msg: *mut sd_bus_message,
                                   mut userdata: *mut libc::c_void,
                                   mut ret_error: *mut sd_bus_error)
 -> libc::c_int {
    let mut session: *mut logind_session = userdata as *mut logind_session;
    let mut ret: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut major: uint32_t = 0;
    let mut minor: uint32_t = 0;
    ret =
        sd_bus_message_read(msg,
                            b"uuh\x00" as *const u8 as *const libc::c_char,
                            &mut major as *mut uint32_t,
                            &mut minor as *mut uint32_t,
                            &mut fd as *mut libc::c_int);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to parse D-Bus response for ResumeDevice: %s\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 339i32, strerror(-ret));
    } else if major == DRM_MAJOR as libc::c_int as libc::c_uint {
        let mut dev: *mut wlr_device =
            find_device(&mut (*session).base, gnu_dev_makedev(major, minor));
        close((*dev).fd);
        if fcntl(fd, 1030i32, (*dev).fd) < 0i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to duplicate file descriptor: %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/session/logind.c\x00" as *const u8 as
                         *const libc::c_char, 348i32,
                     strerror(*__errno_location()));
        } else if !(*session).base.active {
            (*session).base.active = 1i32 != 0;
            wlr_signal_emit_safe(&mut (*session).base.session_signal,
                                 session as *mut libc::c_void);
        }
    }
    return 0i32;
}
unsafe extern "C" fn session_properties_changed(mut msg: *mut sd_bus_message,
                                                mut userdata:
                                                    *mut libc::c_void,
                                                mut ret_error:
                                                    *mut sd_bus_error)
 -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block: u64;
    let mut session: *mut logind_session = userdata as *mut logind_session;
    let mut ret: libc::c_int = 0i32;
    // if we have a drm fd we don't depend on this
    if (*session).has_drm { return 0i32 }
    // PropertiesChanged arg 1: interface
    let mut interface: *const libc::c_char =
        0 as *const libc::c_char; // skip path
    ret =
        sd_bus_message_read_basic(msg, 's' as i32 as libc::c_char,
                                  &mut interface as *mut *const libc::c_char
                                      as *mut libc::c_void);
    if !(ret < 0i32) {
        if strcmp(interface,
                  b"org.freedesktop.login1.Session\x00" as *const u8 as
                      *const libc::c_char) != 0i32 {
            // not interesting for us; ignore
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] ignoring PropertiesChanged from %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/session/logind.c\x00" as *const u8 as
                         *const libc::c_char, 381i32, interface);
            return 0i32
        }
        // PropertiesChanged arg 2: changed properties with values
        ret =
            sd_bus_message_enter_container(msg, 'a' as i32 as libc::c_char,
                                           b"{sv}\x00" as *const u8 as
                                               *const libc::c_char);
        if !(ret < 0i32) {
            s = 0 as *const libc::c_char;
            loop  {
                ret =
                    sd_bus_message_enter_container(msg,
                                                   'e' as i32 as libc::c_char,
                                                   b"sv\x00" as *const u8 as
                                                       *const libc::c_char);
                if !(ret > 0i32) {
                    current_block = 1118134448028020070;
                    break ;
                }
                ret =
                    sd_bus_message_read_basic(msg, 's' as i32 as libc::c_char,
                                              &mut s as
                                                  *mut *const libc::c_char as
                                                  *mut libc::c_void);
                if ret < 0i32 { current_block = 811591568610607247; break ; }
                if strcmp(s,
                          b"Active\x00" as *const u8 as *const libc::c_char)
                       == 0i32 {
                    let mut ret_0: libc::c_int = 0;
                    ret_0 =
                        sd_bus_message_enter_container(msg,
                                                       'v' as i32 as
                                                           libc::c_char,
                                                       b"b\x00" as *const u8
                                                           as
                                                           *const libc::c_char);
                    if ret_0 < 0i32 {
                        current_block = 811591568610607247;
                        break ;
                    }
                    let mut active: bool = false;
                    ret_0 =
                        sd_bus_message_read_basic(msg,
                                                  'b' as i32 as libc::c_char,
                                                  &mut active as *mut bool as
                                                      *mut libc::c_void);
                    if ret_0 < 0i32 {
                        current_block = 811591568610607247;
                        break ;
                    }
                    if (*session).base.active as libc::c_int !=
                           active as libc::c_int {
                        (*session).base.active = active;
                        wlr_signal_emit_safe(&mut (*session).base.session_signal,
                                             session as *mut libc::c_void);
                    }
                    return 0i32
                } else {
                    sd_bus_message_skip(msg,
                                        b"{sv}\x00" as *const u8 as
                                            *const libc::c_char);
                    ret = sd_bus_message_exit_container(msg);
                    if ret < 0i32 {
                        current_block = 811591568610607247;
                        break ;
                    }
                }
            }
            match current_block {
                811591568610607247 => { }
                _ => {
                    if !(ret < 0i32) {
                        ret = sd_bus_message_exit_container(msg);
                        if !(ret < 0i32) {
                            // PropertiesChanged arg 3: changed properties without values
                            sd_bus_message_enter_container(msg,
                                                           'a' as i32 as
                                                               libc::c_char,
                                                           b"s\x00" as
                                                               *const u8 as
                                                               *const libc::c_char);
                            loop  {
                                ret =
                                    sd_bus_message_read_basic(msg,
                                                              's' as i32 as
                                                                  libc::c_char,
                                                              &mut s as
                                                                  *mut *const libc::c_char
                                                                  as
                                                                  *mut libc::c_void);
                                if !(ret > 0i32) { break ; }
                                if strcmp(s,
                                          b"Active\x00" as *const u8 as
                                              *const libc::c_char) == 0i32 {
                                    let mut error: sd_bus_error =
                                        {
                                            let mut init =
                                                sd_bus_error{name:
                                                                 0 as
                                                                     *const libc::c_char,
                                                             message:
                                                                 0 as
                                                                     *const libc::c_char,
                                                             _need_free:
                                                                 0i32,};
                                            init
                                        };
                                    let mut active_0: bool = false;
                                    ret =
                                        sd_bus_get_property_trivial((*session).bus,
                                                                    b"org.freedesktop.login1\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    (*session).path,
                                                                    b"org.freedesktop.login1.Session\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b"Active\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    &mut error,
                                                                    'b' as i32
                                                                        as
                                                                        libc::c_char,
                                                                    &mut active_0
                                                                        as
                                                                        *mut bool
                                                                        as
                                                                        *mut libc::c_void);
                                    if ret < 0i32 {
                                        _wlr_log(WLR_ERROR,
                                                 b"[%s:%d] Failed to get \'Active\' property: %s\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"../backend/session/logind.c\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 447i32, error.message);
                                        return 0i32
                                    }
                                    if (*session).base.active as libc::c_int
                                           != active_0 as libc::c_int {
                                        (*session).base.active = active_0;
                                        wlr_signal_emit_safe(&mut (*session).base.session_signal,
                                                             session as
                                                                 *mut libc::c_void);
                                    }
                                    return 0i32
                                }
                            }
                            if !(ret < 0i32) { return 0i32 }
                        }
                    }
                }
            }
        }
    }
    _wlr_log(WLR_ERROR,
             b"[%s:%d] Failed to parse D-Bus PropertiesChanged: %s\x00" as
                 *const u8 as *const libc::c_char,
             b"../backend/session/logind.c\x00" as *const u8 as
                 *const libc::c_char, 467i32, strerror(-ret));
    return 0i32;
}
unsafe extern "C" fn seat_properties_changed(mut msg: *mut sd_bus_message,
                                             mut userdata: *mut libc::c_void,
                                             mut ret_error: *mut sd_bus_error)
 -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block: u64;
    let mut session: *mut logind_session = userdata as *mut logind_session;
    let mut ret: libc::c_int = 0i32;
    // if we have a drm fd we don't depend on this
    if (*session).has_drm { return 0i32 }
    // PropertiesChanged arg 1: interface
    let mut interface: *const libc::c_char =
        0 as *const libc::c_char; // skip path
    ret =
        sd_bus_message_read_basic(msg, 's' as i32 as libc::c_char,
                                  &mut interface as *mut *const libc::c_char
                                      as *mut libc::c_void);
    if !(ret < 0i32) {
        if strcmp(interface,
                  b"org.freedesktop.login1.Seat\x00" as *const u8 as
                      *const libc::c_char) != 0i32 {
            // not interesting for us; ignore
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] ignoring PropertiesChanged from %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/session/logind.c\x00" as *const u8 as
                         *const libc::c_char, 490i32, interface);
            return 0i32
        }
        // PropertiesChanged arg 2: changed properties with values
        ret =
            sd_bus_message_enter_container(msg, 'a' as i32 as libc::c_char,
                                           b"{sv}\x00" as *const u8 as
                                               *const libc::c_char);
        if !(ret < 0i32) {
            s = 0 as *const libc::c_char;
            loop  {
                ret =
                    sd_bus_message_enter_container(msg,
                                                   'e' as i32 as libc::c_char,
                                                   b"sv\x00" as *const u8 as
                                                       *const libc::c_char);
                if !(ret > 0i32) {
                    current_block = 4488286894823169796;
                    break ;
                }
                ret =
                    sd_bus_message_read_basic(msg, 's' as i32 as libc::c_char,
                                              &mut s as
                                                  *mut *const libc::c_char as
                                                  *mut libc::c_void);
                if ret < 0i32 { current_block = 7577173092109912252; break ; }
                if strcmp(s,
                          b"CanGraphical\x00" as *const u8 as
                              *const libc::c_char) == 0i32 {
                    let mut ret_0: libc::c_int = 0;
                    ret_0 =
                        sd_bus_message_enter_container(msg,
                                                       'v' as i32 as
                                                           libc::c_char,
                                                       b"b\x00" as *const u8
                                                           as
                                                           *const libc::c_char);
                    if ret_0 < 0i32 {
                        current_block = 7577173092109912252;
                        break ;
                    }
                    ret_0 =
                        sd_bus_message_read_basic(msg,
                                                  'b' as i32 as libc::c_char,
                                                  &mut (*session).can_graphical
                                                      as *mut bool as
                                                      *mut libc::c_void);
                    if ret_0 < 0i32 {
                        current_block = 7577173092109912252;
                        break ;
                    }
                    return 0i32
                } else {
                    sd_bus_message_skip(msg,
                                        b"{sv}\x00" as *const u8 as
                                            *const libc::c_char);
                    ret = sd_bus_message_exit_container(msg);
                    if ret < 0i32 {
                        current_block = 7577173092109912252;
                        break ;
                    }
                }
            }
            match current_block {
                7577173092109912252 => { }
                _ => {
                    if !(ret < 0i32) {
                        ret = sd_bus_message_exit_container(msg);
                        if !(ret < 0i32) {
                            // PropertiesChanged arg 3: changed properties without values
                            sd_bus_message_enter_container(msg,
                                                           'a' as i32 as
                                                               libc::c_char,
                                                           b"s\x00" as
                                                               *const u8 as
                                                               *const libc::c_char);
                            loop  {
                                ret =
                                    sd_bus_message_read_basic(msg,
                                                              's' as i32 as
                                                                  libc::c_char,
                                                              &mut s as
                                                                  *mut *const libc::c_char
                                                                  as
                                                                  *mut libc::c_void);
                                if !(ret > 0i32) { break ; }
                                if strcmp(s,
                                          b"CanGraphical\x00" as *const u8 as
                                              *const libc::c_char) == 0i32 {
                                    (*session).can_graphical =
                                        sd_seat_can_graphical((*session).base.seat.as_mut_ptr())
                                            != 0;
                                    return 0i32
                                }
                            }
                            if !(ret < 0i32) { return 0i32 }
                        }
                    }
                }
            }
        }
    }
    _wlr_log(WLR_ERROR,
             b"[%s:%d] Failed to parse D-Bus PropertiesChanged: %s\x00" as
                 *const u8 as *const libc::c_char,
             b"../backend/session/logind.c\x00" as *const u8 as
                 *const libc::c_char, 556i32, strerror(-ret));
    return 0i32;
}
unsafe extern "C" fn add_signal_matches(mut session: *mut logind_session)
 -> bool {
    static mut logind: *const libc::c_char =
        b"org.freedesktop.login1\x00" as *const u8 as *const libc::c_char;
    static mut logind_path: *const libc::c_char =
        b"/org/freedesktop/login1\x00" as *const u8 as *const libc::c_char;
    static mut manager_interface: *const libc::c_char =
        b"org.freedesktop.login1.Manager\x00" as *const u8 as
            *const libc::c_char;
    static mut session_interface: *const libc::c_char =
        b"org.freedesktop.login1.Session\x00" as *const u8 as
            *const libc::c_char;
    static mut property_interface: *const libc::c_char =
        b"org.freedesktop.DBus.Properties\x00" as *const u8 as
            *const libc::c_char;
    let mut ret: libc::c_int = 0;
    ret =
        sd_bus_match_signal((*session).bus, 0 as *mut *mut sd_bus_slot,
                            logind, logind_path, manager_interface,
                            b"SessionRemoved\x00" as *const u8 as
                                *const libc::c_char,
                            Some(session_removed as
                                     unsafe extern "C" fn(_:
                                                              *mut sd_bus_message,
                                                          _:
                                                              *mut libc::c_void,
                                                          _:
                                                              *mut sd_bus_error)
                                         -> libc::c_int),
                            session as *mut libc::c_void);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to add D-Bus match: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 571i32, strerror(-ret));
        return 0i32 != 0
    }
    ret =
        sd_bus_match_signal((*session).bus, 0 as *mut *mut sd_bus_slot,
                            logind, (*session).path, session_interface,
                            b"PauseDevice\x00" as *const u8 as
                                *const libc::c_char,
                            Some(pause_device as
                                     unsafe extern "C" fn(_:
                                                              *mut sd_bus_message,
                                                          _:
                                                              *mut libc::c_void,
                                                          _:
                                                              *mut sd_bus_error)
                                         -> libc::c_int),
                            session as *mut libc::c_void);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to add D-Bus match: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 578i32, strerror(-ret));
        return 0i32 != 0
    }
    ret =
        sd_bus_match_signal((*session).bus, 0 as *mut *mut sd_bus_slot,
                            logind, (*session).path, session_interface,
                            b"ResumeDevice\x00" as *const u8 as
                                *const libc::c_char,
                            Some(resume_device as
                                     unsafe extern "C" fn(_:
                                                              *mut sd_bus_message,
                                                          _:
                                                              *mut libc::c_void,
                                                          _:
                                                              *mut sd_bus_error)
                                         -> libc::c_int),
                            session as *mut libc::c_void);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to add D-Bus match: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 585i32, strerror(-ret));
        return 0i32 != 0
    }
    ret =
        sd_bus_match_signal((*session).bus, 0 as *mut *mut sd_bus_slot,
                            logind, (*session).path, property_interface,
                            b"PropertiesChanged\x00" as *const u8 as
                                *const libc::c_char,
                            Some(session_properties_changed as
                                     unsafe extern "C" fn(_:
                                                              *mut sd_bus_message,
                                                          _:
                                                              *mut libc::c_void,
                                                          _:
                                                              *mut sd_bus_error)
                                         -> libc::c_int),
                            session as *mut libc::c_void);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to add D-Bus match: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 593i32, strerror(-ret));
        return 0i32 != 0
    }
    ret =
        sd_bus_match_signal((*session).bus, 0 as *mut *mut sd_bus_slot,
                            logind, (*session).seat_path, property_interface,
                            b"PropertiesChanged\x00" as *const u8 as
                                *const libc::c_char,
                            Some(seat_properties_changed as
                                     unsafe extern "C" fn(_:
                                                              *mut sd_bus_message,
                                                          _:
                                                              *mut libc::c_void,
                                                          _:
                                                              *mut sd_bus_error)
                                         -> libc::c_int),
                            session as *mut libc::c_void);
    if ret < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to add D-Bus match: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 601i32, strerror(-ret));
        return 0i32 != 0
    }
    return 1i32 != 0;
}
unsafe extern "C" fn dbus_event(mut fd: libc::c_int, mut mask: uint32_t,
                                mut data: *mut libc::c_void) -> libc::c_int {
    let mut bus: *mut sd_bus = data as *mut sd_bus;
    // Do nothing.
    while sd_bus_process(bus, 0 as *mut *mut sd_bus_message) > 0i32 { }
    return 1i32;
}
unsafe extern "C" fn contains_str(mut needle: *const libc::c_char,
                                  mut haystack: *mut *const libc::c_char)
 -> bool {
    let mut i: libc::c_int = 0i32;
    while !(*haystack.offset(i as isize)).is_null() {
        if strcmp(*haystack.offset(i as isize), needle) == 0i32 {
            return 1i32 != 0
        }
        i += 1
    }
    return 0i32 != 0;
}
unsafe extern "C" fn get_greeter_session(mut session_id:
                                             *mut *mut libc::c_char) -> bool {
    let mut class: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user_sessions: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char;
    let mut user_session_count: libc::c_int =
        sd_uid_get_sessions(getuid(), 1i32, &mut user_sessions);
    if user_session_count < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to get sessions\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 632i32);
    } else if user_session_count == 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] User has no sessions\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 637i32);
    } else {
        let mut i: libc::c_int = 0i32;
        while i < user_session_count {
            let mut ret: libc::c_int =
                sd_session_get_class(*user_sessions.offset(i as isize),
                                     &mut class);
            if !(ret < 0i32) {
                if strcmp(class,
                          b"greeter\x00" as *const u8 as *const libc::c_char)
                       == 0i32 {
                    *session_id = strdup(*user_sessions.offset(i as isize));
                    break ;
                }
            }
            i += 1
        }
    }
    free(class as *mut libc::c_void);
    let mut i_0: libc::c_int = 0i32;
    while i_0 < user_session_count {
        free(*user_sessions.offset(i_0 as isize) as *mut libc::c_void);
        i_0 += 1
    }
    free(user_sessions as *mut libc::c_void);
    return !(*session_id).is_null();
}
unsafe extern "C" fn get_display_session(mut session_id:
                                             *mut *mut libc::c_char) -> bool {
    let mut graphical_session_types: [*const libc::c_char; 4] =
        [0 as *const libc::c_char; 4];
    let mut active_states: [*const libc::c_char; 3] =
        [0 as *const libc::c_char; 3];
    if !session_id.is_null() {
    } else {
        __assert_fail(b"session_id != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/session/logind.c\x00" as *const u8 as
                          *const libc::c_char, 664i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"_Bool get_display_session(char **)\x00")).as_ptr());
    };
    let mut ret: libc::c_int = 0;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut state: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xdg_session_id: *mut libc::c_char =
        getenv(b"XDG_SESSION_ID\x00" as *const u8 as *const libc::c_char);
    if !xdg_session_id.is_null() {
        // This just checks whether the supplied session ID is valid
        if sd_session_is_active(xdg_session_id) < 0i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Invalid XDG_SESSION_ID: \'%s\'\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/session/logind.c\x00" as *const u8 as
                         *const libc::c_char, 674i32, xdg_session_id);
        } else { *session_id = strdup(xdg_session_id); return 1i32 != 0 }
    } else {
        // If there's a session active for the current process then just use that
        ret = sd_pid_get_session(getpid(), session_id);
        if ret == 0i32 { return 1i32 != 0 }
        // Find any active sessions for the user if the process isn't part of an
	// active session itself
        ret = sd_uid_get_display(getuid(), session_id);
        if ret < 0i32 && ret != -61i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to get display: %s\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/session/logind.c\x00" as *const u8 as
                         *const libc::c_char, 691i32, strerror(-ret));
        } else if ret != 0i32 && !get_greeter_session(session_id) {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Couldn\'t find an active session or a greeter session\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/session/logind.c\x00" as *const u8 as
                         *const libc::c_char, 696i32);
        } else {
            if !(*session_id).is_null() {
            } else {
                __assert_fail(b"*session_id != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"../backend/session/logind.c\x00" as *const u8
                                  as *const libc::c_char,
                              700i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 35],
                                                        &[libc::c_char; 35]>(b"_Bool get_display_session(char **)\x00")).as_ptr());
            };
            // Check that the available session is graphical
            ret = sd_session_get_type(*session_id, &mut type_0);
            if ret < 0i32 {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Couldn\'t get a type for session \'%s\': %s\x00"
                             as *const u8 as *const libc::c_char,
                         b"../backend/session/logind.c\x00" as *const u8 as
                             *const libc::c_char, 706i32, *session_id,
                         strerror(-ret));
            } else {
                graphical_session_types =
                    [b"wayland\x00" as *const u8 as *const libc::c_char,
                     b"x11\x00" as *const u8 as *const libc::c_char,
                     b"mir\x00" as *const u8 as *const libc::c_char,
                     0 as *const libc::c_char];
                if !contains_str(type_0, graphical_session_types.as_mut_ptr())
                   {
                    _wlr_log(WLR_ERROR,
                             b"[%s:%d] Session \'%s\' isn\'t a graphical session (type: \'%s\')\x00"
                                 as *const u8 as *const libc::c_char,
                             b"../backend/session/logind.c\x00" as *const u8
                                 as *const libc::c_char, 713i32, *session_id,
                             type_0);
                } else {
                    // Check that the session is active
                    ret = sd_session_get_state(*session_id, &mut state);
                    if ret < 0i32 {
                        _wlr_log(WLR_ERROR,
                                 b"[%s:%d] Couldn\'t get state for session \'%s\': %s\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"../backend/session/logind.c\x00" as
                                     *const u8 as *const libc::c_char, 721i32,
                                 *session_id, strerror(-ret));
                    } else {
                        active_states =
                            [b"active\x00" as *const u8 as
                                 *const libc::c_char,
                             b"online\x00" as *const u8 as
                                 *const libc::c_char,
                             0 as *const libc::c_char];
                        if !contains_str(state, active_states.as_mut_ptr()) {
                            _wlr_log(WLR_ERROR,
                                     b"[%s:%d] Session \'%s\' is not active\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"../backend/session/logind.c\x00" as
                                         *const u8 as *const libc::c_char,
                                     727i32, *session_id);
                        } else {
                            free(type_0 as *mut libc::c_void);
                            free(state as *mut libc::c_void);
                            return 1i32 != 0
                        }
                    }
                }
            }
        }
    }
    free(type_0 as *mut libc::c_void);
    free(state as *mut libc::c_void);
    free(*session_id as *mut libc::c_void);
    *session_id = 0 as *mut libc::c_char;
    return 0i32 != 0;
}
unsafe extern "C" fn logind_session_create(mut disp: *mut wl_display)
 -> *mut wlr_session {
    let mut event_loop: *mut wl_event_loop = 0 as *mut wl_event_loop;
    let mut seat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut session: *mut logind_session =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<logind_session>() as libc::c_ulong) as
            *mut logind_session;
    if session.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/logind.c\x00" as *const u8 as
                     *const libc::c_char, 748i32,
                 strerror(*__errno_location()));
        return 0 as *mut wlr_session
    }
    if get_display_session(&mut (*session).id) {
        seat = 0 as *mut libc::c_char;
        ret = sd_session_get_seat((*session).id, &mut seat);
        if ret < 0i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to get seat id: %s\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/session/logind.c\x00" as *const u8 as
                         *const libc::c_char, 759i32, strerror(-ret));
        } else {
            snprintf((*session).base.seat.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 256]>() as
                         libc::c_ulong,
                     b"%s\x00" as *const u8 as *const libc::c_char, seat);
            if strcmp(seat, b"seat0\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                ret =
                    sd_session_get_vt((*session).id,
                                      &mut (*session).base.vtnr);
                if ret < 0i32 {
                    _wlr_log(WLR_ERROR,
                             b"[%s:%d] Session not running in virtual terminal\x00"
                                 as *const u8 as *const libc::c_char,
                             b"../backend/session/logind.c\x00" as *const u8
                                 as *const libc::c_char, 767i32);
                    current_block = 9241548199638810260;
                } else { current_block = 13056961889198038528; }
            } else { current_block = 13056961889198038528; }
            match current_block {
                9241548199638810260 => { }
                _ => {
                    free(seat as *mut libc::c_void);
                    ret = sd_bus_default_system(&mut (*session).bus);
                    if ret < 0i32 {
                        _wlr_log(WLR_ERROR,
                                 b"[%s:%d] Failed to open D-Bus connection: %s\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"../backend/session/logind.c\x00" as
                                     *const u8 as *const libc::c_char, 775i32,
                                 strerror(-ret));
                    } else if !find_session_path(session) {
                        sd_bus_unref((*session).bus);
                    } else if !find_seat_path(session) {
                        sd_bus_unref((*session).bus);
                        free((*session).path as *mut libc::c_void);
                    } else {
                        if add_signal_matches(session) {
                            event_loop = wl_display_get_event_loop(disp);
                            (*session).event =
                                wl_event_loop_add_fd(event_loop,
                                                     sd_bus_get_fd((*session).bus),
                                                     WL_EVENT_READABLE as
                                                         libc::c_int as
                                                         uint32_t,
                                                     Some(dbus_event as
                                                              unsafe extern "C" fn(_:
                                                                                       libc::c_int,
                                                                                   _:
                                                                                       uint32_t,
                                                                                   _:
                                                                                       *mut libc::c_void)
                                                                  ->
                                                                      libc::c_int),
                                                     (*session).bus as
                                                         *mut libc::c_void);
                            if session_activate(session) {
                                if take_control(session) {
                                    // Check for CanGraphical first
                                    (*session).can_graphical =
                                        sd_seat_can_graphical((*session).base.seat.as_mut_ptr())
                                            != 0;
                                    if !(*session).can_graphical {
                                        _wlr_log(WLR_INFO,
                                                 b"[%s:%d] Waiting for \'CanGraphical\' on seat %s\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"../backend/session/logind.c\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 809i32,
                                                 (*session).base.seat.as_mut_ptr());
                                    }
                                    loop  {
                                        if (*session).can_graphical {
                                            current_block =
                                                18435049525520518667;
                                            break ;
                                        }
                                        ret =
                                            wl_event_loop_dispatch(event_loop,
                                                                   -1i32);
                                        if !(ret < 0i32) { continue ; }
                                        _wlr_log(WLR_ERROR,
                                                 b"[%s:%d] Polling error waiting for \'CanGraphical\' on seat %s\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"../backend/session/logind.c\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 816i32,
                                                 (*session).base.seat.as_mut_ptr());
                                        current_block = 1130861444095256174;
                                        break ;
                                    }
                                    match current_block {
                                        1130861444095256174 => { }
                                        _ => {
                                            _wlr_log(WLR_INFO,
                                                     b"[%s:%d] Successfully loaded logind session\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     b"../backend/session/logind.c\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     821i32);
                                            (*session).base.impl_0 =
                                                &session_logind;
                                            return &mut (*session).base
                                        }
                                    }
                                }
                            }
                        }
                        sd_bus_unref((*session).bus);
                        free((*session).path as *mut libc::c_void);
                        free((*session).seat_path as *mut libc::c_void);
                    }
                }
            }
        }
    }
    free((*session).id as *mut libc::c_void);
    return 0 as *mut wlr_session;
}
#[no_mangle]
pub static mut session_logind: session_impl =
    {
    
        {
            let mut init =
                session_impl{create:
                                 Some(logind_session_create as
                                          unsafe extern "C" fn(_:
                                                                   *mut wl_display)
                                              -> *mut wlr_session),
                             destroy:
                                 Some(logind_session_destroy as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session)
                                              -> ()),
                             open:
                                 Some(logind_take_device as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session,
                                                               _:
                                                                   *const libc::c_char)
                                              -> libc::c_int),
                             close:
                                 Some(logind_release_device as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session,
                                                               _: libc::c_int)
                                              -> ()),
                             change_vt:
                                 Some(logind_change_vt as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session,
                                                               _:
                                                                   libc::c_uint)
                                              -> bool),};
            init
        }
};
