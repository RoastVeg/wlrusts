use libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
}
pub type __u64 = libc::c_ulonglong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_format {
    pub format: uint32_t,
    pub len: size_t,
    pub cap: size_t,
    pub modifiers: [uint64_t; 0],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub cap: size_t,
    pub formats: *mut *mut wlr_drm_format,
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
pub unsafe extern "C" fn wlr_drm_format_set_finish(mut set:
                                                       *mut wlr_drm_format_set) {
    let mut i: size_t = 0i32 as size_t;
    while i < (*set).len {
        free(*(*set).formats.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free((*set).formats as *mut libc::c_void);
    (*set).len = 0i32 as size_t;
    (*set).cap = 0i32 as size_t;
    (*set).formats = 0 as *mut *mut wlr_drm_format;
}
unsafe extern "C" fn format_set_get_ref(mut set: *mut wlr_drm_format_set,
                                        mut format: uint32_t)
 -> *mut *mut wlr_drm_format {
    let mut i: size_t = 0i32 as size_t;
    while i < (*set).len {
        if (**(*set).formats.offset(i as isize)).format == format {
            return &mut *(*set).formats.offset(i as isize) as
                       *mut *mut wlr_drm_format
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut *mut wlr_drm_format;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_drm_format_set_get(mut set:
                                                    *const wlr_drm_format_set,
                                                mut format: uint32_t)
 -> *const wlr_drm_format {
    let mut ptr: *mut *mut wlr_drm_format =
        format_set_get_ref(set as *mut wlr_drm_format_set, format);
    return if !ptr.is_null() { *ptr } else { 0 as *mut wlr_drm_format };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_drm_format_set_has(mut set:
                                                    *const wlr_drm_format_set,
                                                mut format: uint32_t,
                                                mut modifier: uint64_t)
 -> bool {
    let mut fmt: *const wlr_drm_format = wlr_drm_format_set_get(set, format);
    if fmt.is_null() { return 0i32 != 0 }
    if modifier as libc::c_ulonglong ==
           (0i32 as __u64) << 56i32 |
               (1u64 << 56i32).wrapping_sub(1i32 as libc::c_ulonglong) &
                   0xffffffffffffffu64 {
        return 1i32 != 0
    }
    let mut i: size_t = 0i32 as size_t;
    while i < (*fmt).len {
        if *(*fmt).modifiers.as_ptr().offset(i as isize) == modifier {
            return 1i32 != 0
        }
        i = i.wrapping_add(1)
    }
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_drm_format_set_add(mut set:
                                                    *mut wlr_drm_format_set,
                                                mut format: uint32_t,
                                                mut modifier: uint64_t)
 -> bool {
    let mut ptr: *mut *mut wlr_drm_format = format_set_get_ref(set, format);
    if !ptr.is_null() {
        let mut fmt: *mut wlr_drm_format = *ptr;
        if modifier as libc::c_ulonglong ==
               (0i32 as __u64) << 56i32 |
                   (1u64 << 56i32).wrapping_sub(1i32 as libc::c_ulonglong) &
                       0xffffffffffffffu64 {
            return 1i32 != 0
        }
        let mut i: size_t = 0i32 as size_t;
        while i < (*fmt).len {
            if *(*fmt).modifiers.as_mut_ptr().offset(i as isize) == modifier {
                return 1i32 != 0
            }
            i = i.wrapping_add(1)
        }
        if (*fmt).len == (*fmt).cap {
            let mut cap: size_t =
                if (*fmt).cap != 0 {
                    (*fmt).cap.wrapping_mul(2i32 as libc::c_ulong)
                } else { 4i32 as libc::c_ulong };
            fmt =
                realloc(fmt as *mut libc::c_void,
                        (::std::mem::size_of::<wlr_drm_format>() as
                             libc::c_ulong).wrapping_add((::std::mem::size_of::<uint64_t>()
                                                              as
                                                              libc::c_ulong).wrapping_mul(cap)))
                    as *mut wlr_drm_format;
            if fmt.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                             *const libc::c_char,
                         b"../render/drm_format_set.c\x00" as *const u8 as
                             *const libc::c_char, 80i32,
                         strerror(*__errno_location()));
                return 0i32 != 0
            }
            (*fmt).cap = cap;
            *ptr = fmt
        }
        let fresh0 = (*fmt).len;
        (*fmt).len = (*fmt).len.wrapping_add(1);
        *(*fmt).modifiers.as_mut_ptr().offset(fresh0 as isize) = modifier;
        return 1i32 != 0
    }
    let mut cap_0: size_t =
        if modifier as libc::c_ulonglong !=
               (0i32 as __u64) << 56i32 |
                   (1u64 << 56i32).wrapping_sub(1i32 as libc::c_ulonglong) &
                       0xffffffffffffffu64 {
            4i32
        } else { 0i32 } as size_t;
    let mut fmt_0: *mut wlr_drm_format =
        calloc(1i32 as libc::c_ulong,
               (::std::mem::size_of::<wlr_drm_format>() as
                    libc::c_ulong).wrapping_add((::std::mem::size_of::<uint64_t>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(cap_0)))
            as *mut wlr_drm_format;
    if fmt_0.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/drm_format_set.c\x00" as *const u8 as
                     *const libc::c_char, 97i32,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    (*fmt_0).format = format;
    if cap_0 != 0 {
        (*fmt_0).cap = cap_0;
        (*fmt_0).len = 1i32 as size_t;
        *(*fmt_0).modifiers.as_mut_ptr().offset(0) = modifier
    }
    if (*set).len == (*set).cap {
        let mut new: size_t =
            if (*set).cap != 0 {
                (*set).cap.wrapping_mul(2i32 as libc::c_ulong)
            } else { 4i32 as libc::c_ulong };
        let mut tmp: *mut *mut wlr_drm_format =
            realloc((*set).formats as *mut libc::c_void,
                    (::std::mem::size_of::<wlr_drm_format>() as
                         libc::c_ulong).wrapping_add((::std::mem::size_of::<uint64_t>()
                                                          as
                                                          libc::c_ulong).wrapping_mul(new)))
                as *mut *mut wlr_drm_format;
        if tmp.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                         *const libc::c_char,
                     b"../render/drm_format_set.c\x00" as *const u8 as
                         *const libc::c_char, 114i32,
                     strerror(*__errno_location()));
            free(fmt_0 as *mut libc::c_void);
            return 0i32 != 0
        }
        (*set).cap = new;
        (*set).formats = tmp
    }
    let fresh1 = (*set).len;
    (*set).len = (*set).len.wrapping_add(1);
    let ref mut fresh2 = *(*set).formats.offset(fresh1 as isize);
    *fresh2 = fmt_0;
    return 1i32 != 0;
}
