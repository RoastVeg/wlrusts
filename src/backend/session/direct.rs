use libc;
extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type udev;
    pub type udev_monitor;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
     -> libc::c_int;
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
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_event_loop_add_signal(loop_0: *mut wl_event_loop,
                                signal_number: libc::c_int,
                                func: wl_event_loop_signal_func_t,
                                data: *mut libc::c_void)
     -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn direct_ipc_open(sock: libc::c_int, path: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn direct_ipc_setmaster(sock: libc::c_int, fd: libc::c_int);
    #[no_mangle]
    fn direct_ipc_dropmaster(sock: libc::c_int, fd: libc::c_int);
    #[no_mangle]
    fn direct_ipc_finish(sock: libc::c_int, pid: pid_t);
    #[no_mangle]
    fn direct_ipc_init(pid_out: *mut pid_t) -> libc::c_int;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct vt_mode {
    pub mode: libc::c_char,
    pub waitv: libc::c_char,
    pub relsig: libc::c_short,
    pub acqsig: libc::c_short,
    pub frsig: libc::c_short,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct vt_stat {
    pub v_active: libc::c_ushort,
    pub v_signal: libc::c_ushort,
    pub v_state: libc::c_ushort,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
pub type wl_event_loop_signal_func_t
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_void)
               -> libc::c_int>;

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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

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
    pub events: C2RustUnnamed,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_device {
    pub fd: libc::c_int,
    pub dev: dev_t,
    pub signal: wl_signal,
    pub link: wl_list,
}
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DRM_MAJOR: C2RustUnnamed_0 = 226;

#[repr(C)]#[derive(Copy, Clone)]
pub struct direct_session {
    pub base: wlr_session,
    pub tty_fd: libc::c_int,
    pub old_kbmode: libc::c_int,
    pub sock: libc::c_int,
    pub child: pid_t,
    pub vt_source: *mut wl_event_source,
}
unsafe extern "C" fn direct_session_from_session(mut base: *mut wlr_session)
 -> *mut direct_session {
    if (*base).impl_0 == &session_direct as *const session_impl {
    } else {
        __assert_fail(b"base->impl == &session_direct\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/session/direct.c\x00" as *const u8 as
                          *const libc::c_char, 39i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"struct direct_session *direct_session_from_session(struct wlr_session *)\x00")).as_ptr());
    };
    return base as *mut direct_session;
}
unsafe extern "C" fn direct_session_open(mut base: *mut wlr_session,
                                         mut path: *const libc::c_char)
 -> libc::c_int {
    let mut session: *mut direct_session = direct_session_from_session(base);
    let mut fd: libc::c_int = direct_ipc_open((*session).sock, path);
    if fd < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to open %s: %s%s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/direct.c\x00" as *const u8 as
                     *const libc::c_char, 49i32, path, strerror(-fd),
                 if fd == -22i32 {
                     b"; is another display server running?\x00" as *const u8
                         as *const libc::c_char
                 } else { b"\x00" as *const u8 as *const libc::c_char });
        return fd
    }
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
    if fstat(fd, &mut st) < 0i32 { close(fd); return -*__errno_location() }
    if gnu_dev_major(st.st_rdev) == DRM_MAJOR as libc::c_int as libc::c_uint {
        direct_ipc_setmaster((*session).sock, fd);
    }
    return fd;
}
unsafe extern "C" fn direct_session_close(mut base: *mut wlr_session,
                                          mut fd: libc::c_int) {
    let mut session: *mut direct_session = direct_session_from_session(base);
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
                 b"[%s:%d] Stat failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/direct.c\x00" as *const u8 as
                     *const libc::c_char, 71i32,
                 strerror(*__errno_location()));
        close(fd);
        return
    }
    if gnu_dev_major(st.st_rdev) == DRM_MAJOR as libc::c_int as libc::c_uint {
        direct_ipc_dropmaster((*session).sock, fd);
    } else if gnu_dev_major(st.st_rdev) == 13i32 as libc::c_uint {
        ioctl(fd,
              (1u32 << 0i32 + 8i32 + 8i32 + 14i32 |
                   (('E' as i32) << 0i32 + 8i32) as libc::c_uint |
                   (0x91i32 << 0i32) as libc::c_uint) as libc::c_ulong |
                  (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) <<
                      0i32 + 8i32 + 8i32, 0i32);
    }
    close(fd);
}
unsafe extern "C" fn direct_change_vt(mut base: *mut wlr_session,
                                      mut vt: libc::c_uint) -> bool {
    let mut session: *mut direct_session = direct_session_from_session(base);
    // Only seat0 has VTs associated with it
    if strcmp((*session).base.seat.as_mut_ptr(),
              b"seat0\x00" as *const u8 as *const libc::c_char) != 0i32 {
        return 1i32 != 0
    }
    return ioctl((*session).tty_fd, 0x5606i32 as libc::c_ulong,
                 vt as libc::c_int) == 0i32;
}
unsafe extern "C" fn direct_session_destroy(mut base: *mut wlr_session) {
    let mut session: *mut direct_session = direct_session_from_session(base);
    if strcmp((*session).base.seat.as_mut_ptr(),
              b"seat0\x00" as *const u8 as *const libc::c_char) == 0i32 {
        let mut mode: vt_mode =
            {
                let mut init =
                    vt_mode{mode: 0i32 as libc::c_char,
                            waitv: 0,
                            relsig: 0,
                            acqsig: 0,
                            frsig: 0,};
                init
            };
        *__errno_location() = 0i32;
        ioctl((*session).tty_fd, 0x4b45i32 as libc::c_ulong,
              (*session).old_kbmode);
        ioctl((*session).tty_fd, 0x4b3ai32 as libc::c_ulong, 0i32);
        ioctl((*session).tty_fd, 0x5602i32 as libc::c_ulong,
              &mut mode as *mut vt_mode);
        if *__errno_location() != 0 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to restore tty\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/session/direct.c\x00" as *const u8 as
                         *const libc::c_char, 110i32);
        }
        wl_event_source_remove((*session).vt_source);
        close((*session).tty_fd);
    }
    direct_ipc_finish((*session).sock, (*session).child);
    close((*session).sock);
    free(session as *mut libc::c_void);
}
unsafe extern "C" fn vt_handler(mut signo: libc::c_int,
                                mut data: *mut libc::c_void) -> libc::c_int {
    let mut session: *mut direct_session = data as *mut direct_session;
    if (*session).base.active {
        (*session).base.active = 0i32 != 0;
        wlr_signal_emit_safe(&mut (*session).base.session_signal,
                             session as *mut libc::c_void);
        let mut dev: *mut wlr_device = 0 as *mut wlr_device;
        dev =
            ((*session).base.devices.next as *mut libc::c_char).offset(-32) as
                *mut wlr_device;
        while &mut (*dev).link as *mut wl_list !=
                  &mut (*session).base.devices as *mut wl_list {
            if gnu_dev_major((*dev).dev) ==
                   DRM_MAJOR as libc::c_int as libc::c_uint {
                direct_ipc_dropmaster((*session).sock, (*dev).fd);
            }
            dev =
                ((*dev).link.next as *mut libc::c_char).offset(-32) as
                    *mut wlr_device
        }
        ioctl((*session).tty_fd, 0x5605i32 as libc::c_ulong, 1i32);
    } else {
        ioctl((*session).tty_fd, 0x5605i32 as libc::c_ulong, 0x2i32);
        let mut dev_0: *mut wlr_device = 0 as *mut wlr_device;
        dev_0 =
            ((*session).base.devices.next as *mut libc::c_char).offset(-32) as
                *mut wlr_device;
        while &mut (*dev_0).link as *mut wl_list !=
                  &mut (*session).base.devices as *mut wl_list {
            if gnu_dev_major((*dev_0).dev) ==
                   DRM_MAJOR as libc::c_int as libc::c_uint {
                direct_ipc_setmaster((*session).sock, (*dev_0).fd);
            }
            dev_0 =
                ((*dev_0).link.next as *mut libc::c_char).offset(-32) as
                    *mut wlr_device
        }
        (*session).base.active = 1i32 != 0;
        wlr_signal_emit_safe(&mut (*session).base.session_signal,
                             session as *mut libc::c_void);
    }
    return 1i32;
}
unsafe extern "C" fn setup_tty(mut session: *mut direct_session,
                               mut display: *mut wl_display) -> bool {
    let mut tty: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut kd_mode: libc::c_int = 0;
    let mut old_kbmode: libc::c_int = 0;
    let mut mode: vt_mode =
        vt_mode{mode: 0, waitv: 0, relsig: 0, acqsig: 0, frsig: 0,};
    let mut loop_0: *mut wl_event_loop = 0 as *mut wl_event_loop;
    let mut default_tty: bool = 0i32 != 0;
    let mut tty_path: *const libc::c_char =
        getenv(b"WLR_DIRECT_TTY\x00" as *const u8 as *const libc::c_char);
    if tty_path.is_null() {
        tty_path = b"/dev/tty\x00" as *const u8 as *const libc::c_char;
        default_tty = 1i32 != 0
    }
    let mut fd: libc::c_int = open(tty_path, 0o2i32 | 0o2000000i32);
    if fd == -1i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot open %s: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/direct.c\x00" as *const u8 as
                     *const libc::c_char, 170i32, tty_path,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    let mut vt_stat: vt_stat = vt_stat{v_active: 0, v_signal: 0, v_state: 0,};
    if ioctl(fd, 0x5603i32 as libc::c_ulong, &mut vt_stat as *mut vt_stat) !=
           0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not get current tty number: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/session/direct.c\x00" as *const u8 as
                     *const libc::c_char, 176i32,
                 strerror(*__errno_location()));
    } else {
        tty = vt_stat.v_active as libc::c_int;
        ret = 0;
        kd_mode = 0;
        old_kbmode = 0;
        ret =
            ioctl(fd, 0x4b3bi32 as libc::c_ulong,
                  &mut kd_mode as *mut libc::c_int);
        if ret != 0 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to get tty mode: %s\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/session/direct.c\x00" as *const u8 as
                         *const libc::c_char, 185i32,
                     strerror(*__errno_location()));
        } else if default_tty as libc::c_int != 0 && kd_mode != 0i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] tty already in graphics mode; is another display server running?\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/session/direct.c\x00" as *const u8 as
                         *const libc::c_char, 191i32);
        } else {
            ioctl(fd, 0x5606i32 as libc::c_ulong, tty);
            ioctl(fd, 0x5607i32 as libc::c_ulong, tty);
            if ioctl(fd, 0x4b44i32 as libc::c_ulong,
                     &mut old_kbmode as *mut libc::c_int) != 0 {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to read keyboard mode: %s\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/session/direct.c\x00" as *const u8 as
                             *const libc::c_char, 199i32,
                         strerror(*__errno_location()));
            } else if ioctl(fd, 0x4b45i32 as libc::c_ulong, 0x4i32) != 0 {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to set keyboard mode: %s\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/session/direct.c\x00" as *const u8 as
                             *const libc::c_char, 204i32,
                         strerror(*__errno_location()));
            } else if ioctl(fd, 0x4b3ai32 as libc::c_ulong, 0x1i32) != 0 {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to set graphics mode on tty: %s\x00"
                             as *const u8 as *const libc::c_char,
                         b"../backend/session/direct.c\x00" as *const u8 as
                             *const libc::c_char, 209i32,
                         strerror(*__errno_location()));
            } else {
                mode =
                    {
                        let mut init =
                            vt_mode{mode: 0x1i32 as libc::c_char,
                                    waitv: 0,
                                    relsig: 12i32 as libc::c_short,
                                    acqsig: 12i32 as libc::c_short,
                                    frsig: 0,};
                        init
                    };
                if ioctl(fd, 0x5602i32 as libc::c_ulong,
                         &mut mode as *mut vt_mode) < 0i32 {
                    _wlr_log(WLR_ERROR,
                             b"[%s:%d] Failed to take control of tty\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../backend/session/direct.c\x00" as *const u8
                                 as *const libc::c_char, 220i32);
                } else {
                    loop_0 = wl_display_get_event_loop(display);
                    (*session).vt_source =
                        wl_event_loop_add_signal(loop_0, 12i32,
                                                 Some(vt_handler as
                                                          unsafe extern "C" fn(_:
                                                                                   libc::c_int,
                                                                               _:
                                                                                   *mut libc::c_void)
                                                              -> libc::c_int),
                                                 session as
                                                     *mut libc::c_void);
                    if !(*session).vt_source.is_null() {
                        (*session).base.vtnr = tty as libc::c_uint;
                        (*session).tty_fd = fd;
                        (*session).old_kbmode = old_kbmode;
                        return 1i32 != 0
                    }
                }
            }
        }
    }
    close(fd);
    return 0i32 != 0;
}
unsafe extern "C" fn direct_session_create(mut disp: *mut wl_display)
 -> *mut wlr_session {
    let mut seat: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block: u64;
    let mut session: *mut direct_session =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<direct_session>() as libc::c_ulong) as
            *mut direct_session;
    if session.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/direct.c\x00" as *const u8 as
                     *const libc::c_char, 245i32,
                 strerror(*__errno_location()));
        return 0 as *mut wlr_session
    }
    (*session).sock = direct_ipc_init(&mut (*session).child);
    if !((*session).sock == -1i32) {
        seat = getenv(b"XDG_SEAT\x00" as *const u8 as *const libc::c_char);
        if seat.is_null() {
            seat = b"seat0\x00" as *const u8 as *const libc::c_char
        }
        if strcmp(seat, b"seat0\x00" as *const u8 as *const libc::c_char) ==
               0i32 {
            if !setup_tty(session, disp) {
                direct_ipc_finish((*session).sock, (*session).child);
                close((*session).sock);
                current_block = 9241686129834928522;
            } else { current_block = 17407779659766490442; }
        } else {
            (*session).base.vtnr = 0i32 as libc::c_uint;
            (*session).tty_fd = -1i32;
            current_block = 17407779659766490442;
        }
        match current_block {
            9241686129834928522 => { }
            _ => {
                snprintf((*session).base.seat.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 256]>() as
                             libc::c_ulong,
                         b"%s\x00" as *const u8 as *const libc::c_char, seat);
                (*session).base.impl_0 = &session_direct;
                _wlr_log(WLR_INFO,
                         b"[%s:%d] Successfully loaded direct session\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/session/direct.c\x00" as *const u8 as
                             *const libc::c_char, 271i32);
                return &mut (*session).base
            }
        }
    }
    free(session as *mut libc::c_void);
    return 0 as *mut wlr_session;
}
#[no_mangle]
pub static mut session_direct: session_impl =
    {
    
        {
            let mut init =
                session_impl{create:
                                 Some(direct_session_create as
                                          unsafe extern "C" fn(_:
                                                                   *mut wl_display)
                                              -> *mut wlr_session),
                             destroy:
                                 Some(direct_session_destroy as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session)
                                              -> ()),
                             open:
                                 Some(direct_session_open as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session,
                                                               _:
                                                                   *const libc::c_char)
                                              -> libc::c_int),
                             close:
                                 Some(direct_session_close as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session,
                                                               _: libc::c_int)
                                              -> ()),
                             change_vt:
                                 Some(direct_change_vt as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_session,
                                                               _:
                                                                   libc::c_uint)
                                              -> bool),};
            init
        }
};
