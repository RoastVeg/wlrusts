use libc;
extern "C" {
    pub type _cap_struct;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn _Exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn socketpair(__domain: libc::c_int, __type: libc::c_int,
                  __protocol: libc::c_int, __fds: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn sendmsg(__fd: libc::c_int, __message: *const msghdr,
               __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn recvmsg(__fd: libc::c_int, __message: *mut msghdr,
               __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn drmDropMaster(fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn drmSetMaster(fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;
    #[no_mangle]
    fn cap_free(_: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn cap_get_flag(_: cap_t, _: cap_value_t, _: cap_flag_t,
                    _: *mut cap_flag_value_t) -> libc::c_int;
    #[no_mangle]
    fn cap_get_proc() -> cap_t;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type pid_t = __pid_t;
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
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
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
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SCM_RIGHTS: C2RustUnnamed_0 = 1;
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
pub type uint32_t = __uint32_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct msg {
    pub type_0: msg_type,
    pub path: [libc::c_char; 256],
}
pub type msg_type = libc::c_uint;
pub const MSG_END: msg_type = 3;
pub const MSG_DROPMASTER: msg_type = 2;
pub const MSG_SETMASTER: msg_type = 1;
pub const MSG_OPEN: msg_type = 0;
pub const DRM_MAJOR: C2RustUnnamed_1 = 226;
pub type cap_t = *mut _cap_struct;
pub const CAP_SET: cap_flag_value_t = 1;
pub type cap_flag_value_t = libc::c_uint;
pub const CAP_CLEAR: cap_flag_value_t = 0;
pub type cap_flag_t = libc::c_uint;
pub const CAP_INHERITABLE: cap_flag_t = 2;
pub const CAP_PERMITTED: cap_flag_t = 1;
pub const CAP_EFFECTIVE: cap_flag_t = 0;
pub type cap_value_t = libc::c_int;
pub type C2RustUnnamed_1 = libc::c_uint;
unsafe extern "C" fn have_permissions() -> bool {
    let mut cap: cap_t = cap_get_proc();
    let mut val: cap_flag_value_t = CAP_CLEAR;
    if cap.is_null() || cap_get_flag(cap, 21i32, CAP_PERMITTED, &mut val) != 0
           || val as libc::c_uint != CAP_SET as libc::c_int as libc::c_uint {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Do not have CAP_SYS_ADMIN; cannot become DRM master\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/session/direct-ipc.c\x00" as *const u8 as
                     *const libc::c_char, 35i32);
        cap_free(cap as *mut libc::c_void);
        return 0i32 != 0
    }
    cap_free(cap as *mut libc::c_void);
    return 1i32 != 0;
}
unsafe extern "C" fn send_msg(mut sock: libc::c_int, mut fd: libc::c_int,
                              mut buf: *mut libc::c_void,
                              mut buf_len: size_t) {
    let mut control: [libc::c_char; 24] =
        [0i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0];
    let mut iovec: iovec =
        { let mut init = iovec{iov_base: buf, iov_len: buf_len,}; init };
    let mut msghdr: msghdr =
        {
            let mut init =
                msghdr{msg_name: 0 as *mut libc::c_void,
                       msg_namelen: 0,
                       msg_iov: 0 as *mut iovec,
                       msg_iovlen: 0,
                       msg_control: 0 as *mut libc::c_void,
                       msg_controllen: 0,
                       msg_flags: 0,};
            init
        };
    if !buf.is_null() {
        msghdr.msg_iov = &mut iovec;
        msghdr.msg_iovlen = 1i32 as size_t
    }
    if fd >= 0i32 {
        msghdr.msg_control =
            &mut control as *mut [libc::c_char; 24] as *mut libc::c_void;
        msghdr.msg_controllen =
            ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong;
        let mut cmsg: *mut cmsghdr =
            if msghdr.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msghdr.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        *cmsg =
            {
                let mut init =
                    cmsghdr{cmsg_len:
                                ((::std::mem::size_of::<cmsghdr>() as
                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_sub(1i32
                                                                                                      as
                                                                                                      libc::c_ulong)
                                     &
                                     !(::std::mem::size_of::<size_t>() as
                                           libc::c_ulong).wrapping_sub(1i32 as
                                                                           libc::c_ulong)).wrapping_add(::std::mem::size_of::<libc::c_int>()
                                                                                                            as
                                                                                                            libc::c_ulong),
                            cmsg_level: 1i32,
                            cmsg_type: SCM_RIGHTS as libc::c_int,
                            __cmsg_data: [],};
                init
            };
        memcpy((*cmsg).__cmsg_data.as_mut_ptr() as *mut libc::c_void,
               &mut fd as *mut libc::c_int as *const libc::c_void,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    }
    let mut ret: ssize_t = 0;
    loop  {
        ret = sendmsg(sock, &mut msghdr, 0i32);
        if !(ret < 0i32 as libc::c_long && *__errno_location() == 4i32) {
            break ;
        }
    };
}
unsafe extern "C" fn recv_msg(mut sock: libc::c_int,
                              mut fd_out: *mut libc::c_int,
                              mut buf: *mut libc::c_void, mut buf_len: size_t)
 -> ssize_t {
    let mut control: [libc::c_char; 24] =
        [0i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0];
    let mut iovec: iovec =
        { let mut init = iovec{iov_base: buf, iov_len: buf_len,}; init };
    let mut msghdr: msghdr =
        {
            let mut init =
                msghdr{msg_name: 0 as *mut libc::c_void,
                       msg_namelen: 0,
                       msg_iov: 0 as *mut iovec,
                       msg_iovlen: 0,
                       msg_control: 0 as *mut libc::c_void,
                       msg_controllen: 0,
                       msg_flags: 0,};
            init
        };
    if !buf.is_null() {
        msghdr.msg_iov = &mut iovec;
        msghdr.msg_iovlen = 1i32 as size_t
    }
    if !fd_out.is_null() {
        msghdr.msg_control =
            &mut control as *mut [libc::c_char; 24] as *mut libc::c_void;
        msghdr.msg_controllen =
            ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
    }
    let mut ret: ssize_t = 0;
    loop  {
        ret = recvmsg(sock, &mut msghdr, MSG_CMSG_CLOEXEC as libc::c_int);
        if !(ret < 0i32 as libc::c_long && *__errno_location() == 4i32) {
            break ;
        }
    }
    if !fd_out.is_null() {
        let mut cmsg: *mut cmsghdr =
            if msghdr.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msghdr.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        if !cmsg.is_null() {
            memcpy(fd_out as *mut libc::c_void,
                   (*cmsg).__cmsg_data.as_mut_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
        } else { *fd_out = -1i32 }
    }
    return ret;
}
unsafe extern "C" fn communicate(mut sock: libc::c_int) {
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
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
    let mut maj: uint32_t = 0;
    let mut msg: msg = msg{type_0: MSG_OPEN, path: [0; 256],};
    let mut drm_fd: libc::c_int = -1i32;
    let mut running: bool = 1i32 != 0;
    while running as libc::c_int != 0 &&
              recv_msg(sock, &mut drm_fd,
                       &mut msg as *mut msg as *mut libc::c_void,
                       ::std::mem::size_of::<msg>() as libc::c_ulong) >
                  0i32 as libc::c_long {
        match msg.type_0 as libc::c_uint {
            0 => {
                *__errno_location() = 0i32;
                // These are the same flags that logind opens files with
                fd =
                    open(msg.path.as_mut_ptr(),
                         0o2i32 | 0o2000000i32 | 0o400i32 | 0o4000i32);
                ret = *__errno_location();
                if !(fd == -1i32) {
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
                        ret = *__errno_location()
                    } else {
                        maj = gnu_dev_major(st.st_rdev);
                        if maj != 13i32 as libc::c_uint &&
                               maj != DRM_MAJOR as libc::c_int as libc::c_uint
                           {
                            ret = 95i32
                        } else if maj ==
                                      DRM_MAJOR as libc::c_int as libc::c_uint
                                      && drmSetMaster(fd) != 0 {
                            ret = *__errno_location()
                        }
                    }
                }
                send_msg(sock, if ret != 0 { -1i32 } else { fd },
                         &mut ret as *mut libc::c_int as *mut libc::c_void,
                         ::std::mem::size_of::<libc::c_int>() as
                             libc::c_ulong);
                if fd >= 0i32 { close(fd); }
            }
            1 => {
                drmSetMaster(drm_fd);
                close(drm_fd);
                send_msg(sock, -1i32, 0 as *mut libc::c_void, 0i32 as size_t);
            }
            2 => {
                drmDropMaster(drm_fd);
                close(drm_fd);
                send_msg(sock, -1i32, 0 as *mut libc::c_void, 0i32 as size_t);
            }
            3 => {
                running = 0i32 != 0;
                send_msg(sock, -1i32, 0 as *mut libc::c_void, 0i32 as size_t);
            }
            _ => { }
        }
    }
    close(sock);
}
#[no_mangle]
pub unsafe extern "C" fn direct_ipc_open(mut sock: libc::c_int,
                                         mut path: *const libc::c_char)
 -> libc::c_int {
    let mut msg: msg =
        { let mut init = msg{type_0: MSG_OPEN, path: [0; 256],}; init };
    snprintf(msg.path.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
             b"%s\x00" as *const u8 as *const libc::c_char, path);
    send_msg(sock, -1i32, &mut msg as *mut msg as *mut libc::c_void,
             ::std::mem::size_of::<msg>() as libc::c_ulong);
    let mut fd: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut retry: libc::c_int = 0i32;
    loop  {
        ret =
            recv_msg(sock, &mut fd,
                     &mut err as *mut libc::c_int as *mut libc::c_void,
                     ::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as
                libc::c_int;
        if !(ret == 0i32 &&
                 { let fresh0 = retry; retry = retry + 1; (fresh0) < 3i32 }) {
            break ;
        }
    }
    return if err != 0 { -err } else { fd };
}
#[no_mangle]
pub unsafe extern "C" fn direct_ipc_setmaster(mut sock: libc::c_int,
                                              mut fd: libc::c_int) {
    let mut msg: msg =
        { let mut init = msg{type_0: MSG_SETMASTER, path: [0; 256],}; init };
    send_msg(sock, fd, &mut msg as *mut msg as *mut libc::c_void,
             ::std::mem::size_of::<msg>() as libc::c_ulong);
    recv_msg(sock, 0 as *mut libc::c_int, 0 as *mut libc::c_void,
             0i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn direct_ipc_dropmaster(mut sock: libc::c_int,
                                               mut fd: libc::c_int) {
    let mut msg: msg =
        { let mut init = msg{type_0: MSG_DROPMASTER, path: [0; 256],}; init };
    send_msg(sock, fd, &mut msg as *mut msg as *mut libc::c_void,
             ::std::mem::size_of::<msg>() as libc::c_ulong);
    recv_msg(sock, 0 as *mut libc::c_int, 0 as *mut libc::c_void,
             0i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn direct_ipc_finish(mut sock: libc::c_int,
                                           mut pid: pid_t) {
    let mut msg: msg =
        { let mut init = msg{type_0: MSG_END, path: [0; 256],}; init };
    send_msg(sock, -1i32, &mut msg as *mut msg as *mut libc::c_void,
             ::std::mem::size_of::<msg>() as libc::c_ulong);
    recv_msg(sock, 0 as *mut libc::c_int, 0 as *mut libc::c_void,
             0i32 as size_t);
    waitpid(pid, 0 as *mut libc::c_int, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn direct_ipc_init(mut pid_out: *mut pid_t)
 -> libc::c_int {
    if !have_permissions() { return -1i32 }
    let mut sock: [libc::c_int; 2] = [0; 2];
    if socketpair(1i32, SOCK_SEQPACKET as libc::c_int, 0i32,
                  sock.as_mut_ptr()) < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create socket pair: %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/session/direct-ipc.c\x00" as *const u8 as
                     *const libc::c_char, 250i32,
                 strerror(*__errno_location()));
        return -1i32
    }
    let mut pid: pid_t = fork();
    if pid < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Fork failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/session/direct-ipc.c\x00" as *const u8 as
                     *const libc::c_char, 256i32,
                 strerror(*__errno_location()));
        close(sock[0]);
        close(sock[1]);
        return -1i32
    } else {
        if pid == 0i32 { close(sock[0]); communicate(sock[1]); _Exit(0i32); }
    }
    close(sock[1]);
    *pid_out = pid;
    return sock[0];
}
