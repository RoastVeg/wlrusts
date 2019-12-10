use libc;
extern "C" {
    pub type udev;
    pub type udev_list_entry;
    pub type udev_device;
    pub type udev_monitor;
    pub type udev_enumerate;
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn udev_unref(udev: *mut udev) -> *mut udev;
    #[no_mangle]
    fn udev_new() -> *mut udev;
    #[no_mangle]
    fn udev_list_entry_get_next(list_entry: *mut udev_list_entry)
     -> *mut udev_list_entry;
    #[no_mangle]
    fn udev_list_entry_get_name(list_entry: *mut udev_list_entry)
     -> *const libc::c_char;
    #[no_mangle]
    fn udev_device_unref(udev_device: *mut udev_device) -> *mut udev_device;
    #[no_mangle]
    fn udev_device_new_from_syspath(udev: *mut udev,
                                    syspath: *const libc::c_char)
     -> *mut udev_device;
    #[no_mangle]
    fn udev_device_get_parent_with_subsystem_devtype(udev_device:
                                                         *mut udev_device,
                                                     subsystem:
                                                         *const libc::c_char,
                                                     devtype:
                                                         *const libc::c_char)
     -> *mut udev_device;
    #[no_mangle]
    fn udev_device_get_sysname(udev_device: *mut udev_device)
     -> *const libc::c_char;
    #[no_mangle]
    fn udev_device_get_devnode(udev_device: *mut udev_device)
     -> *const libc::c_char;
    #[no_mangle]
    fn udev_enumerate_get_list_entry(udev_enumerate: *mut udev_enumerate)
     -> *mut udev_list_entry;
    #[no_mangle]
    fn udev_enumerate_scan_devices(udev_enumerate: *mut udev_enumerate)
     -> libc::c_int;
    #[no_mangle]
    fn udev_enumerate_add_match_sysname(udev_enumerate: *mut udev_enumerate,
                                        sysname: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn udev_enumerate_add_match_subsystem(udev_enumerate: *mut udev_enumerate,
                                          subsystem: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn udev_enumerate_new(udev: *mut udev) -> *mut udev_enumerate;
    #[no_mangle]
    fn udev_enumerate_unref(udev_enumerate: *mut udev_enumerate)
     -> *mut udev_enumerate;
    #[no_mangle]
    fn udev_monitor_filter_add_match_subsystem_devtype(udev_monitor:
                                                           *mut udev_monitor,
                                                       subsystem:
                                                           *const libc::c_char,
                                                       devtype:
                                                           *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn udev_monitor_receive_device(udev_monitor: *mut udev_monitor)
     -> *mut udev_device;
    #[no_mangle]
    fn udev_monitor_get_fd(udev_monitor: *mut udev_monitor) -> libc::c_int;
    #[no_mangle]
    fn udev_monitor_enable_receiving(udev_monitor: *mut udev_monitor)
     -> libc::c_int;
    #[no_mangle]
    fn udev_monitor_new_from_netlink(udev: *mut udev,
                                     name: *const libc::c_char)
     -> *mut udev_monitor;
    #[no_mangle]
    fn udev_monitor_unref(udev_monitor: *mut udev_monitor)
     -> *mut udev_monitor;
    #[no_mangle]
    fn udev_device_get_property_value(udev_device: *mut udev_device,
                                      key: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn udev_device_get_devnum(udev_device: *mut udev_device) -> dev_t;
    #[no_mangle]
    fn udev_device_get_action(udev_device: *mut udev_device)
     -> *const libc::c_char;
    #[no_mangle]
    fn udev_device_get_sysattr_value(udev_device: *mut udev_device,
                                     sysattr: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strtok_r(__s: *mut libc::c_char, __delim: *const libc::c_char,
                __save_ptr: *mut *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_event_loop_add_fd(loop_0: *mut wl_event_loop, fd: libc::c_int,
                            mask: uint32_t, func: wl_event_loop_fd_func_t,
                            data: *mut libc::c_void) -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn wl_display_add_destroy_listener(display: *mut wl_display,
                                       listener: *mut wl_listener);
    // Returns the log verbosity provided to wlr_log_init
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn drmGetVersion(fd: libc::c_int) -> drmVersionPtr;
    #[no_mangle]
    fn drmFreeVersion(_: drmVersionPtr);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
    #[no_mangle]
    static session_logind: session_impl;
    #[no_mangle]
    static session_direct: session_impl;
    #[no_mangle]
    static session_noop: session_impl;
}
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type dev_t = __dev_t;
pub type size_t = libc::c_ulong;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
pub type uint32_t = __uint32_t;
#[derive ( Copy, Clone )]
#[repr(C)]
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
pub struct wlr_device {
    pub fd: libc::c_int,
    pub dev: dev_t,
    pub signal: wl_signal,
    pub link: wl_list,
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
    pub events: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
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
pub type drmVersion = _drmVersion;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _drmVersion {
    pub version_major: libc::c_int,
    pub version_minor: libc::c_int,
    pub version_patchlevel: libc::c_int,
    pub name_len: libc::c_int,
    pub name: *mut libc::c_char,
    pub date_len: libc::c_int,
    pub date: *mut libc::c_char,
    pub desc_len: libc::c_int,
    pub desc: *mut libc::c_char,
}
pub type drmVersionPtr = *mut _drmVersion;
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
static mut impls: [*const session_impl; 3] =
    unsafe {
        [&session_logind as *const session_impl,
         &session_direct as *const session_impl, 0 as *const session_impl]
    };
unsafe extern "C" fn udev_event(mut fd: libc::c_int, mut mask: uint32_t,
                                mut data: *mut libc::c_void) -> libc::c_int {
    let mut devnum: dev_t = 0;
    let mut dev: *mut wlr_device = 0 as *mut wlr_device;
    let mut session: *mut wlr_session = data as *mut wlr_session;
    let mut udev_dev: *mut udev_device =
        udev_monitor_receive_device((*session).mon);
    if udev_dev.is_null() { return 1i32 }
    let mut action: *const libc::c_char = udev_device_get_action(udev_dev);
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] udev event for %s (%s)\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/session/session.c\x00" as *const u8 as
                 *const libc::c_char, 41i32,
             udev_device_get_sysname(udev_dev), action);
    if !(action.is_null() ||
             strcmp(action, b"change\x00" as *const u8 as *const libc::c_char)
                 != 0i32) {
        devnum = udev_device_get_devnum(udev_dev);
        dev = 0 as *mut wlr_device;
        dev =
            ((*session).devices.next as *mut libc::c_char).offset(-32) as
                *mut wlr_device;
        while &mut (*dev).link as *mut wl_list !=
                  &mut (*session).devices as *mut wl_list {
            if (*dev).dev == devnum {
                wlr_signal_emit_safe(&mut (*dev).signal,
                                     session as *mut libc::c_void);
                break ;
            } else {
                dev =
                    ((*dev).link.next as *mut libc::c_char).offset(-32) as
                        *mut wlr_device
            }
        }
    }
    udev_device_unref(udev_dev);
    return 1i32;
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut session: *mut wlr_session =
        (listener as *mut libc::c_char).offset(-328) as *mut wlr_session;
    wlr_session_destroy(session);
}
/*
 * Opens a session, taking control of the current virtual terminal.
 * This should not be called if another program is already in control
 * of the terminal (Xorg, another Wayland compositor, etc.).
 *
 * If logind support is not enabled, you must have CAP_SYS_ADMIN or be root.
 * It is safe to drop privileges after this is called.
 *
 * Returns NULL on error.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_session_create(mut disp: *mut wl_display)
 -> *mut wlr_session {
    let mut event_loop: *mut wl_event_loop = 0 as *mut wl_event_loop;
    let mut fd: libc::c_int = 0;
    let mut session: *mut wlr_session = 0 as *mut wlr_session;
    let mut env_wlr_session: *const libc::c_char =
        getenv(b"WLR_SESSION\x00" as *const u8 as *const libc::c_char);
    if !env_wlr_session.is_null() {
        if strcmp(env_wlr_session,
                  b"logind\x00" as *const u8 as *const libc::c_char) == 0i32
               ||
               strcmp(env_wlr_session,
                      b"systemd\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
            session =
                session_logind.create.expect("non-null function pointer")(disp)
        } else if strcmp(env_wlr_session,
                         b"direct\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
            session =
                session_direct.create.expect("non-null function pointer")(disp)
        } else if strcmp(env_wlr_session,
                         b"noop\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
            session =
                session_noop.create.expect("non-null function pointer")(disp)
        } else {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Unsupported WLR_SESSION: %s\x00" as *const u8
                         as *const libc::c_char,
                     b"../backend/session/session.c\x00" as *const u8 as
                         *const libc::c_char, 86i32, env_wlr_session);
        }
    } else {
        let mut iter: *mut *const session_impl =
            0 as *mut *const session_impl;
        iter = impls.as_mut_ptr();
        while session.is_null() && !(*iter).is_null() {
            session =
                (**iter).create.expect("non-null function pointer")(disp);
            iter = iter.offset(1)
        }
    }
    if session.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to load session backend\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/session.c\x00" as *const u8 as
                     *const libc::c_char, 96i32);
        return 0 as *mut wlr_session
    }
    (*session).active = 1i32 != 0;
    wl_signal_init(&mut (*session).session_signal);
    wl_signal_init(&mut (*session).events.destroy);
    wl_list_init(&mut (*session).devices);
    (*session).udev = udev_new();
    if (*session).udev.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create udev context: %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/session/session.c\x00" as *const u8 as
                     *const libc::c_char, 107i32,
                 strerror(*__errno_location()));
    } else {
        (*session).mon =
            udev_monitor_new_from_netlink((*session).udev,
                                          b"udev\x00" as *const u8 as
                                              *const libc::c_char);
        if (*session).mon.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to create udev monitor: %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/session/session.c\x00" as *const u8 as
                         *const libc::c_char, 113i32,
                     strerror(*__errno_location()));
        } else {
            udev_monitor_filter_add_match_subsystem_devtype((*session).mon,
                                                            b"drm\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            0 as
                                                                *const libc::c_char);
            udev_monitor_enable_receiving((*session).mon);
            event_loop = wl_display_get_event_loop(disp);
            fd = udev_monitor_get_fd((*session).mon);
            (*session).udev_event =
                wl_event_loop_add_fd(event_loop, fd,
                                     WL_EVENT_READABLE as libc::c_int as
                                         uint32_t,
                                     Some(udev_event as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       uint32_t,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> libc::c_int),
                                     session as *mut libc::c_void);
            if (*session).udev_event.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to create udev event source: %s\x00"
                             as *const u8 as *const libc::c_char,
                         b"../backend/session/session.c\x00" as *const u8 as
                             *const libc::c_char, 126i32,
                         strerror(*__errno_location()));
                udev_monitor_unref((*session).mon);
            } else {
                (*session).display_destroy.notify =
                    Some(handle_display_destroy as
                             unsafe extern "C" fn(_: *mut wl_listener,
                                                  _: *mut libc::c_void)
                                 -> ());
                wl_display_add_destroy_listener(disp,
                                                &mut (*session).display_destroy);
                return session
            }
        }
        udev_unref((*session).udev);
    }
    (*(*session).impl_0).destroy.expect("non-null function pointer")(session);
    return 0 as *mut wlr_session;
}
/*
 * Closes a previously opened session and restores the virtual terminal.
 * You should call wlr_session_close_file on each files you opened
 * with wlr_session_open_file before you call this.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_session_destroy(mut session: *mut wlr_session) {
    if session.is_null() { return }
    wlr_signal_emit_safe(&mut (*session).events.destroy,
                         session as *mut libc::c_void);
    wl_list_remove(&mut (*session).display_destroy.link);
    wl_event_source_remove((*session).udev_event);
    udev_monitor_unref((*session).mon);
    udev_unref((*session).udev);
    (*(*session).impl_0).destroy.expect("non-null function pointer")(session);
}
/*
 * Opens the file at path.
 * This can only be used to open DRM or evdev (input) devices.
 *
 * When the session becomes inactive:
 * - DRM files lose their DRM master status
 * - evdev files become invalid and should be closed
 *
 * Returns -errno on error.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_session_open_file(mut session: *mut wlr_session,
                                               mut path: *const libc::c_char)
 -> libc::c_int {
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
    let mut fd: libc::c_int =
        (*(*session).impl_0).open.expect("non-null function pointer")(session,
                                                                      path);
    if fd < 0i32 { return fd }
    let mut dev: *mut wlr_device =
        malloc(::std::mem::size_of::<wlr_device>() as libc::c_ulong) as
            *mut wlr_device;
    if dev.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/session.c\x00" as *const u8 as
                     *const libc::c_char, 167i32,
                 strerror(*__errno_location()));
    } else {
        st =
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
                     b"[%s:%d] Stat failed: %s\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/session/session.c\x00" as *const u8 as
                         *const libc::c_char, 173i32,
                     strerror(*__errno_location()));
        } else {
            (*dev).fd = fd;
            (*dev).dev = st.st_rdev;
            wl_signal_init(&mut (*dev).signal);
            wl_list_insert(&mut (*session).devices, &mut (*dev).link);
            return fd
        }
    }
    free(dev as *mut libc::c_void);
    return fd;
}
unsafe extern "C" fn find_device(mut session: *mut wlr_session,
                                 mut fd: libc::c_int) -> *mut wlr_device {
    let mut dev: *mut wlr_device = 0 as *mut wlr_device;
    dev =
        ((*session).devices.next as *mut libc::c_char).offset(-32) as
            *mut wlr_device;
    while &mut (*dev).link as *mut wl_list !=
              &mut (*session).devices as *mut wl_list {
        if (*dev).fd == fd { return dev }
        dev =
            ((*dev).link.next as *mut libc::c_char).offset(-32) as
                *mut wlr_device
    }
    _wlr_log(WLR_ERROR,
             b"[%s:%d] Tried to use fd %d not opened by session\x00" as
                 *const u8 as *const libc::c_char,
             b"../backend/session/session.c\x00" as *const u8 as
                 *const libc::c_char, 198i32, fd);
    if 0i32 != 0 {
    } else {
        __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                      b"../backend/session/session.c\x00" as *const u8 as
                          *const libc::c_char, 199i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"struct wlr_device *find_device(struct wlr_session *, int)\x00")).as_ptr());
    };
    return 0 as *mut wlr_device;
}
/*
 * Closes a file previously opened with wlr_session_open_file.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_session_close_file(mut session: *mut wlr_session,
                                                mut fd: libc::c_int) {
    let mut dev: *mut wlr_device = find_device(session, fd);
    (*(*session).impl_0).close.expect("non-null function pointer")(session,
                                                                   fd);
    wl_list_remove(&mut (*dev).link);
    free(dev as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_session_signal_add(mut session: *mut wlr_session,
                                                mut fd: libc::c_int,
                                                mut listener:
                                                    *mut wl_listener) {
    let mut dev: *mut wlr_device = find_device(session, fd);
    wl_signal_add(&mut (*dev).signal, listener);
}
/*
 * Changes the virtual terminal.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_session_change_vt(mut session: *mut wlr_session,
                                               mut vt: libc::c_uint) -> bool {
    if session.is_null() { return 0i32 != 0 }
    return (*(*session).impl_0).change_vt.expect("non-null function pointer")(session,
                                                                              vt);
}
/* Tests if 'path' is KMS compatible by trying to open it.
 * It leaves the open device in *fd_out it it succeeds.
 */
unsafe extern "C" fn open_if_kms(mut session: *mut wlr_session,
                                 mut path: *const libc::c_char)
 -> libc::c_int {
    if path.is_null() { return -1i32 }
    let mut fd: libc::c_int = wlr_session_open_file(session, path);
    if fd < 0i32 { return -1i32 }
    let mut ver: *mut drmVersion = drmGetVersion(fd);
    if ver.is_null() {
        wlr_session_close_file(session, fd);
        return -1i32
    } else { drmFreeVersion(ver); return fd };
}
unsafe extern "C" fn explicit_find_gpus(mut session: *mut wlr_session,
                                        mut ret_len: size_t,
                                        mut ret: *mut libc::c_int,
                                        mut str: *const libc::c_char)
 -> size_t {
    let mut gpus: *mut libc::c_char = strdup(str);
    if gpus.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/session.c\x00" as *const u8 as
                     *const libc::c_char, 257i32,
                 strerror(*__errno_location()));
        return 0i32 as size_t
    }
    let mut i: size_t = 0i32 as size_t;
    let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char =
        strtok_r(gpus, b":\x00" as *const u8 as *const libc::c_char,
                 &mut save);
    while !(i >= ret_len) {
        *ret.offset(i as isize) = open_if_kms(session, ptr);
        if *ret.offset(i as isize) < 0i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Unable to open %s as DRM device\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/session/session.c\x00" as *const u8 as
                         *const libc::c_char, 271i32, ptr);
        } else { i = i.wrapping_add(1) }
        ptr =
            strtok_r(0 as *mut libc::c_char,
                     b":\x00" as *const u8 as *const libc::c_char, &mut save);
        if ptr.is_null() { break ; }
    }
    free(gpus as *mut libc::c_void);
    return i;
}
/* Tries to find the primary GPU by checking for the "boot_vga" attribute.
 * If it's not found, it returns the first valid GPU it finds.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_session_find_gpus(mut session: *mut wlr_session,
                                               mut ret_len: size_t,
                                               mut ret: *mut libc::c_int)
 -> size_t {
    let mut explicit: *const libc::c_char =
        getenv(b"WLR_DRM_DEVICES\x00" as *const u8 as *const libc::c_char);
    if !explicit.is_null() {
        return explicit_find_gpus(session, ret_len, ret, explicit)
    }
    let mut en: *mut udev_enumerate = udev_enumerate_new((*session).udev);
    if en.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create udev enumeration\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/session/session.c\x00" as *const u8 as
                     *const libc::c_char, 298i32);
        return -1i32 as size_t
    }
    udev_enumerate_add_match_subsystem(en,
                                       b"drm\x00" as *const u8 as
                                           *const libc::c_char);
    udev_enumerate_add_match_sysname(en,
                                     b"card[0-9]*\x00" as *const u8 as
                                         *const libc::c_char);
    udev_enumerate_scan_devices(en);
    let mut entry: *mut udev_list_entry = 0 as *mut udev_list_entry;
    let mut i: size_t = 0i32 as size_t;
    entry = udev_enumerate_get_list_entry(en);
    while !entry.is_null() {
        if i == ret_len { break ; }
        let mut is_boot_vga: bool = 0i32 != 0;
        let mut path: *const libc::c_char = udev_list_entry_get_name(entry);
        let mut dev: *mut udev_device =
            udev_device_new_from_syspath((*session).udev, path);
        if !dev.is_null() {
            let mut seat: *const libc::c_char =
                udev_device_get_property_value(dev,
                                               b"ID_SEAT\x00" as *const u8 as
                                                   *const libc::c_char);
            if seat.is_null() {
                seat = b"seat0\x00" as *const u8 as *const libc::c_char
            }
            if (*session).seat[0] as libc::c_int != 0 &&
                   strcmp((*session).seat.as_mut_ptr(), seat) != 0i32 {
                udev_device_unref(dev);
            } else {
                // This is owned by 'dev', so we don't need to free it
                let mut pci: *mut udev_device =
                    udev_device_get_parent_with_subsystem_devtype(dev,
                                                                  b"pci\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  0 as
                                                                      *const libc::c_char);
                if !pci.is_null() {
                    let mut id: *const libc::c_char =
                        udev_device_get_sysattr_value(pci,
                                                      b"boot_vga\x00" as
                                                          *const u8 as
                                                          *const libc::c_char);
                    if !id.is_null() &&
                           strcmp(id,
                                  b"1\x00" as *const u8 as
                                      *const libc::c_char) == 0i32 {
                        is_boot_vga = 1i32 != 0
                    }
                }
                let mut fd: libc::c_int =
                    open_if_kms(session, udev_device_get_devnode(dev));
                if fd < 0i32 {
                    udev_device_unref(dev);
                } else {
                    udev_device_unref(dev);
                    *ret.offset(i as isize) = fd;
                    if is_boot_vga {
                        let mut tmp: libc::c_int = *ret.offset(0);
                        *ret.offset(0) = *ret.offset(i as isize);
                        *ret.offset(i as isize) = tmp
                    }
                    i = i.wrapping_add(1)
                }
            }
        }
        entry = udev_list_entry_get_next(entry)
    }
    udev_enumerate_unref(en);
    return i;
}
