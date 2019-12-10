use libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
}
pub type __mode_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;

#[repr(C)]#[derive(Copy, Clone)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
static mut lock_fmt: [libc::c_char; 15] =
    [47, 116, 109, 112, 47, 46, 88, 37, 100, 45, 108, 111, 99, 107, 0];
static mut socket_dir: [libc::c_char; 15] =
    [47, 116, 109, 112, 47, 46, 88, 49, 49, 45, 117, 110, 105, 120, 0];
static mut socket_fmt: [libc::c_char; 19] =
    [47, 116, 109, 112, 47, 46, 88, 49, 49, 45, 117, 110, 105, 120, 47, 88,
     37, 100, 0];
#[no_mangle]
pub unsafe extern "C" fn set_cloexec(mut fd: libc::c_int, mut cloexec: bool)
 -> bool {
    let mut flags: libc::c_int = fcntl(fd, 1i32);
    if flags == -1i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] fcntl failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/sockets.c\x00" as *const u8 as
                     *const libc::c_char, 27i32,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    if cloexec { flags = flags | 1i32 } else { flags = flags & !1i32 }
    if fcntl(fd, 2i32, flags) == -1i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] fcntl failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/sockets.c\x00" as *const u8 as
                     *const libc::c_char, 36i32,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    return 1i32 != 0;
}
unsafe extern "C" fn open_socket(mut addr: *mut sockaddr_un,
                                 mut path_size: size_t) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut size: socklen_t =
        2u64.wrapping_add(path_size).wrapping_add(1i32 as libc::c_ulong) as
            socklen_t;
    fd = socket(1i32, SOCK_STREAM as libc::c_int, 0i32);
    if fd < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create socket %c%s: %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../xwayland/sockets.c\x00" as *const u8 as
                     *const libc::c_char, 50i32,
                 if (*addr).sun_path[0] as libc::c_int != 0 {
                     (*addr).sun_path[0] as libc::c_int
                 } else { '@' as i32 },
                 (*addr).sun_path.as_mut_ptr().offset(1),
                 strerror(*__errno_location()));
        return -1i32
    }
    if !set_cloexec(fd, 1i32 != 0) { close(fd); return -1i32 }
    if (*addr).sun_path[0] != 0 { unlink((*addr).sun_path.as_mut_ptr()); }
    if bind(fd, addr as *mut sockaddr, size) < 0i32 {
        rc = *__errno_location();
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to bind socket %c%s: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/sockets.c\x00" as *const u8 as
                     *const libc::c_char, 65i32,
                 if (*addr).sun_path[0] as libc::c_int != 0 {
                     (*addr).sun_path[0] as libc::c_int
                 } else { '@' as i32 },
                 (*addr).sun_path.as_mut_ptr().offset(1),
                 strerror(*__errno_location()));
    } else if listen(fd, 1i32) < 0i32 {
        rc = *__errno_location();
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to listen to socket %c%s: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../xwayland/sockets.c\x00" as *const u8 as
                     *const libc::c_char, 72i32,
                 if (*addr).sun_path[0] as libc::c_int != 0 {
                     (*addr).sun_path[0] as libc::c_int
                 } else { '@' as i32 },
                 (*addr).sun_path.as_mut_ptr().offset(1),
                 strerror(*__errno_location()));
    } else { return fd }
    close(fd);
    if (*addr).sun_path[0] != 0 { unlink((*addr).sun_path.as_mut_ptr()); }
    *__errno_location() = rc;
    return -1i32;
}
unsafe extern "C" fn open_sockets(mut socks: *mut libc::c_int,
                                  mut display: libc::c_int) -> bool {
    let mut addr: sockaddr_un =
        {
            let mut init =
                sockaddr_un{sun_family: 1i32 as sa_family_t,
                            sun_path: [0; 108],};
            init
        };
    let mut path_size: size_t = 0;
    mkdir(socket_dir.as_ptr(), 0o777i32 as __mode_t);
    addr.sun_path[0] = 0i32 as libc::c_char;
    path_size =
        snprintf(addr.sun_path.as_mut_ptr().offset(1),
                 (::std::mem::size_of::<[libc::c_char; 108]>() as
                      libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong),
                 socket_fmt.as_ptr(), display) as size_t;
    *socks.offset(0) = open_socket(&mut addr, path_size);
    if *socks.offset(0) < 0i32 { return 0i32 != 0 }
    path_size =
        snprintf(addr.sun_path.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 108]>() as
                     libc::c_ulong, socket_fmt.as_ptr(), display) as size_t;
    *socks.offset(1) = open_socket(&mut addr, path_size);
    if *socks.offset(1) < 0i32 {
        close(*socks.offset(0));
        *socks.offset(0) = -1i32;
        return 0i32 != 0
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn unlink_display_sockets(mut display: libc::c_int) {
    let mut sun_path: [libc::c_char; 64] = [0; 64];
    snprintf(sun_path.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
             socket_fmt.as_ptr(), display);
    unlink(sun_path.as_mut_ptr());
    snprintf(sun_path.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
             lock_fmt.as_ptr(), display);
    unlink(sun_path.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn open_display_sockets(mut socks: *mut libc::c_int)
 -> libc::c_int {
    let mut lock_fd: libc::c_int = 0;
    let mut display: libc::c_int = 0;
    let mut lock_name: [libc::c_char; 64] = [0; 64];
    display = 0i32;
    while display <= 32i32 {
        snprintf(lock_name.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                 lock_fmt.as_ptr(), display);
        lock_fd =
            open(lock_name.as_mut_ptr(),
                 0o1i32 | 0o100i32 | 0o200i32 | 0o2000000i32, 0o444i32);
        if lock_fd >= 0i32 {
            if !open_sockets(socks, display) {
                unlink(lock_name.as_mut_ptr());
                close(lock_fd);
            } else {
                let mut pid: [libc::c_char; 12] = [0; 12];
                snprintf(pid.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 12]>() as
                             libc::c_ulong,
                         b"%10d\x00" as *const u8 as *const libc::c_char,
                         getpid());
                if write(lock_fd, pid.as_mut_ptr() as *const libc::c_void,
                         (::std::mem::size_of::<[libc::c_char; 12]>() as
                              libc::c_ulong).wrapping_sub(1i32 as
                                                              libc::c_ulong))
                       as libc::c_ulong !=
                       (::std::mem::size_of::<[libc::c_char; 12]>() as
                            libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong)
                   {
                    unlink(lock_name.as_mut_ptr());
                    close(lock_fd);
                } else { close(lock_fd); break ; }
            }
        } else {
            lock_fd = open(lock_name.as_mut_ptr(), 0i32 | 0o2000000i32);
            if !(lock_fd < 0i32) {
                let mut pid_0: [libc::c_char; 12] =
                    [0i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                let mut end_pid: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut bytes: ssize_t =
                    read(lock_fd, pid_0.as_mut_ptr() as *mut libc::c_void,
                         (::std::mem::size_of::<[libc::c_char; 12]>() as
                              libc::c_ulong).wrapping_sub(1i32 as
                                                              libc::c_ulong));
                close(lock_fd);
                if !(bytes as libc::c_ulong !=
                         (::std::mem::size_of::<[libc::c_char; 12]>() as
                              libc::c_ulong).wrapping_sub(1i32 as
                                                              libc::c_ulong))
                   {
                    let mut read_pid: libc::c_long = 0;
                    read_pid =
                        strtol(pid_0.as_mut_ptr(), &mut end_pid, 10i32);
                    if !(read_pid < 0i32 as libc::c_long ||
                             read_pid > 2147483647i32 as libc::c_long ||
                             end_pid !=
                                 pid_0.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_char; 12]>()
                                                               as
                                                               libc::c_ulong
                                                               as
                                                               isize).offset(-2))
                       {
                        *__errno_location() = 0i32;
                        if kill(read_pid as pid_t, 0i32) != 0i32 &&
                               *__errno_location() == 3i32 {
                            if !(unlink(lock_name.as_mut_ptr()) != 0i32) {
                                // retry
                                display -= 1
                            }
                        }
                    }
                }
            }
        }
        display += 1
    }
    if display > 32i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] No display available in the first 33\x00" as
                     *const u8 as *const libc::c_char,
                 b"../xwayland/sockets.c\x00" as *const u8 as
                     *const libc::c_char, 181i32);
        return -1i32
    }
    return display;
}
