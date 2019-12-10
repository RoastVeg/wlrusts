use libc;
extern "C" {
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    // Will log all messages less than or equal to `verbosity`
// If `callback` is NULL, wlr will use its default logger.
// The function can be called multiple times to update the verbosity or
// callback function.
    // Returns the log verbosity provided to wlr_log_init
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_dmabuf_attributes {
    pub width: int32_t,
    pub height: int32_t,
    pub format: uint32_t,
    pub flags: uint32_t,
    pub modifier: uint64_t,
    pub n_planes: libc::c_int,
    pub offset: [uint32_t; 4],
    pub stride: [uint32_t; 4],
    pub fd: [libc::c_int; 4],
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
#[no_mangle]
pub unsafe extern "C" fn wlr_dmabuf_attributes_finish(mut attribs:
                                                          *mut wlr_dmabuf_attributes) {
    let mut i: libc::c_int = 0i32;
    while i < (*attribs).n_planes {
        close((*attribs).fd[i as usize]);
        (*attribs).fd[i as usize] = -1i32;
        i += 1
    }
    (*attribs).n_planes = 0i32;
}
/* *
 * Closes all file descriptors in the DMA-BUF attributes.
 */
/* *
 * Clones the DMA-BUF attributes.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_dmabuf_attributes_copy(mut dst:
                                                        *mut wlr_dmabuf_attributes,
                                                    mut src:
                                                        *mut wlr_dmabuf_attributes)
 -> bool {
    memcpy(dst as *mut libc::c_void, src as *const libc::c_void,
           ::std::mem::size_of::<wlr_dmabuf_attributes>() as libc::c_ulong);
    let mut i: libc::c_int = 0i32;
    while i < (*src).n_planes {
        (*dst).fd[i as usize] = fcntl((*src).fd[i as usize], 1030i32, 0i32);
        if (*dst).fd[i as usize] < 0i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] fcntl(F_DUPFD_CLOEXEC) failed: %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../render/dmabuf.c\x00" as *const u8 as
                         *const libc::c_char, 22i32,
                     strerror(*__errno_location()));
            return 0i32 != 0
        }
        i += 1
    }
    return 1i32 != 0;
}
