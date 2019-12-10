use libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn shm_open(__name: *const libc::c_char, __oflag: libc::c_int,
                __mode: mode_t) -> libc::c_int;
    #[no_mangle]
    fn shm_unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type mode_t = __mode_t;
pub type size_t = libc::c_ulong;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
unsafe extern "C" fn randname(mut buf: *mut libc::c_char) {
    let mut ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    clock_gettime(0i32, &mut ts);
    let mut r: libc::c_long = ts.tv_nsec;
    let mut i: libc::c_int = 0i32;
    while i < 6i32 {
        *buf.offset(i as isize) =
            ('A' as i32 as libc::c_long + (r & 15i32 as libc::c_long) +
                 (r & 16i32 as libc::c_long) * 2i32 as libc::c_long) as
                libc::c_char;
        r >>= 5i32;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn create_shm_file() -> libc::c_int {
    let mut retries: libc::c_int = 100i32;
    loop  {
        let mut name: [libc::c_char; 16] =
            *::std::mem::transmute::<&[u8; 16],
                                     &mut [libc::c_char; 16]>(b"/wlroots-XXXXXX\x00");
        randname(name.as_mut_ptr().offset(strlen(name.as_mut_ptr()) as
                                              isize).offset(-6));
        retries -= 1;
        // CLOEXEC is guaranteed to be set by shm_open
        let mut fd: libc::c_int =
            shm_open(name.as_mut_ptr(), 0o2i32 | 0o100i32 | 0o200i32,
                     0o600i32 as mode_t);
        if fd >= 0i32 { shm_unlink(name.as_mut_ptr()); return fd }
        if !(retries > 0i32 && *__errno_location() == 17i32) { break ; }
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn allocate_shm_file(mut size: size_t) -> libc::c_int {
    let mut fd: libc::c_int = create_shm_file();
    if fd < 0i32 { return -1i32 }
    let mut ret: libc::c_int = 0;
    loop  {
        ret = ftruncate(fd, size as __off64_t);
        if !(ret < 0i32 && *__errno_location() == 4i32) { break ; }
    }
    if ret < 0i32 { close(fd); return -1i32 }
    return fd;
}
