use libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn bsearch(__key: *const libc::c_void, __base: *const libc::c_void,
               __nmemb: size_t, __size: size_t, __compar: __compar_fn_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn drmModeGetProperty(fd: libc::c_int, propertyId: uint32_t)
     -> drmModePropertyPtr;
    #[no_mangle]
    fn drmModeFreeProperty(ptr: drmModePropertyPtr);
    #[no_mangle]
    fn drmModeGetPropertyBlob(fd: libc::c_int, blob_id: uint32_t)
     -> drmModePropertyBlobPtr;
    #[no_mangle]
    fn drmModeFreePropertyBlob(ptr: drmModePropertyBlobPtr);
    #[no_mangle]
    fn drmModeObjectGetProperties(fd: libc::c_int, object_id: uint32_t,
                                  object_type: uint32_t)
     -> drmModeObjectPropertiesPtr;
    #[no_mangle]
    fn drmModeFreeObjectProperties(ptr: drmModeObjectPropertiesPtr);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
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
pub type __u64 = libc::c_ulonglong;

#[repr(C)]#[derive(Copy, Clone)]
pub struct drm_mode_property_enum {
    pub value: __u64,
    pub name: [libc::c_char; 32],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModePropertyBlob {
    pub id: uint32_t,
    pub length: uint32_t,
    pub data: *mut libc::c_void,
}
pub type drmModePropertyBlobRes = _drmModePropertyBlob;
pub type drmModePropertyBlobPtr = *mut _drmModePropertyBlob;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModeProperty {
    pub prop_id: uint32_t,
    pub flags: uint32_t,
    pub name: [libc::c_char; 32],
    pub count_values: libc::c_int,
    pub values: *mut uint64_t,
    pub count_enums: libc::c_int,
    pub enums: *mut drm_mode_property_enum,
    pub count_blobs: libc::c_int,
    pub blob_ids: *mut uint32_t,
}
pub type drmModePropertyRes = _drmModeProperty;
pub type drmModePropertyPtr = *mut _drmModeProperty;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModeObjectProperties {
    pub count_props: uint32_t,
    pub props: *mut uint32_t,
    pub prop_values: *mut uint64_t,
}
pub type drmModeObjectProperties = _drmModeObjectProperties;
pub type drmModeObjectPropertiesPtr = *mut _drmModeObjectProperties;
/*
 * These types contain the property ids for several DRM objects.
 * See https://01.org/linuxgraphics/gfx-docs/drm/gpu/drm-kms.html#kms-properties
 * for more details.
 */

#[repr ( C )]#[derive(Copy, Clone)]
pub union wlr_drm_connector_props {
    pub c2rust_unnamed: C2RustUnnamed,
    pub props: [uint32_t; 4],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub edid: uint32_t,
    pub dpms: uint32_t,
    pub link_status: uint32_t,
    pub path: uint32_t,
    pub crtc_id: uint32_t,
}

#[repr ( C )]#[derive(Copy, Clone)]
pub union wlr_drm_crtc_props {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub props: [uint32_t; 6],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub rotation: uint32_t,
    pub scaling_mode: uint32_t,
    pub active: uint32_t,
    pub mode_id: uint32_t,
    pub gamma_lut: uint32_t,
    pub gamma_lut_size: uint32_t,
}

#[repr ( C )]#[derive(Copy, Clone)]
pub union wlr_drm_plane_props {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub props: [uint32_t; 13],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub type_0: uint32_t,
    pub rotation: uint32_t,
    pub in_formats: uint32_t,
    pub src_x: uint32_t,
    pub src_y: uint32_t,
    pub src_w: uint32_t,
    pub src_h: uint32_t,
    pub crtc_x: uint32_t,
    pub crtc_y: uint32_t,
    pub crtc_w: uint32_t,
    pub crtc_h: uint32_t,
    pub fb_id: uint32_t,
    pub crtc_id: uint32_t,
}
/*
 * Creates a mapping between property names and an array index where to store
 * the ids.  The prop_info arrays must be sorted by name, as bsearch is used to
 * search them.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct prop_info {
    pub name: *const libc::c_char,
    pub index: size_t,
}
// Initialized in run_static_initializers
static mut connector_info: [prop_info; 5] =
    [prop_info{name: 0 as *const libc::c_char, index: 0,}; 5];
// Initialized in run_static_initializers
static mut crtc_info: [prop_info; 6] =
    [prop_info{name: 0 as *const libc::c_char, index: 0,}; 6];
// Initialized in run_static_initializers
static mut plane_info: [prop_info; 12] =
    [prop_info{name: 0 as *const libc::c_char, index: 0,}; 12];
unsafe extern "C" fn cmp_prop_info(mut arg1: *const libc::c_void,
                                   mut arg2: *const libc::c_void)
 -> libc::c_int {
    let mut key: *const libc::c_char = arg1 as *const libc::c_char;
    let mut elem: *const prop_info = arg2 as *const prop_info;
    return strcmp(key, (*elem).name);
}
unsafe extern "C" fn scan_properties(mut fd: libc::c_int, mut id: uint32_t,
                                     mut type_0: uint32_t,
                                     mut result: *mut uint32_t,
                                     mut info: *const prop_info,
                                     mut info_len: size_t) -> bool {
    let mut props: *mut drmModeObjectProperties =
        drmModeObjectGetProperties(fd, id, type_0);
    if props.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to get DRM object properties: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/drm/properties.c\x00" as *const u8 as
                     *const libc::c_char, 69i32,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    let mut i: uint32_t = 0i32 as uint32_t;
    while i < (*props).count_props {
        let mut prop: *mut drmModePropertyRes =
            drmModeGetProperty(fd, *(*props).props.offset(i as isize));
        if prop.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to get DRM object property: %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/drm/properties.c\x00" as *const u8 as
                         *const libc::c_char, 76i32,
                     strerror(*__errno_location()));
        } else {
            let mut p: *const prop_info =
                bsearch((*prop).name.as_mut_ptr() as *const libc::c_void,
                        info as *const libc::c_void, info_len,
                        ::std::mem::size_of::<prop_info>() as libc::c_ulong,
                        Some(cmp_prop_info as
                                 unsafe extern "C" fn(_: *const libc::c_void,
                                                      _: *const libc::c_void)
                                     -> libc::c_int)) as *const prop_info;
            if !p.is_null() {
                *result.offset((*p).index as isize) = (*prop).prop_id
            }
            drmModeFreeProperty(prop);
        }
        i = i.wrapping_add(1)
    }
    drmModeFreeObjectProperties(props);
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn get_drm_connector_props(mut fd: libc::c_int,
                                                 mut id: uint32_t,
                                                 mut out:
                                                     *mut wlr_drm_connector_props)
 -> bool {
    return scan_properties(fd, id, 0xc0c0c0c0u32, (*out).props.as_mut_ptr(),
                           connector_info.as_ptr(),
                           (::std::mem::size_of::<[prop_info; 5]>() as
                                libc::c_ulong).wrapping_div(::std::mem::size_of::<prop_info>()
                                                                as
                                                                libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn get_drm_crtc_props(mut fd: libc::c_int,
                                            mut id: uint32_t,
                                            mut out: *mut wlr_drm_crtc_props)
 -> bool {
    return scan_properties(fd, id, 0xccccccccu32, (*out).props.as_mut_ptr(),
                           crtc_info.as_ptr(),
                           (::std::mem::size_of::<[prop_info; 6]>() as
                                libc::c_ulong).wrapping_div(::std::mem::size_of::<prop_info>()
                                                                as
                                                                libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn get_drm_plane_props(mut fd: libc::c_int,
                                             mut id: uint32_t,
                                             mut out:
                                                 *mut wlr_drm_plane_props)
 -> bool {
    return scan_properties(fd, id, 0xeeeeeeeeu32, (*out).props.as_mut_ptr(),
                           plane_info.as_ptr(),
                           (::std::mem::size_of::<[prop_info; 12]>() as
                                libc::c_ulong).wrapping_div(::std::mem::size_of::<prop_info>()
                                                                as
                                                                libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn get_drm_prop(mut fd: libc::c_int, mut obj: uint32_t,
                                      mut prop: uint32_t,
                                      mut ret: *mut uint64_t) -> bool {
    let mut props: *mut drmModeObjectProperties =
        drmModeObjectGetProperties(fd, obj, 0i32 as uint32_t);
    if props.is_null() { return 0i32 != 0 }
    let mut found: bool = 0i32 != 0;
    let mut i: uint32_t = 0i32 as uint32_t;
    while i < (*props).count_props {
        if *(*props).props.offset(i as isize) == prop {
            *ret = *(*props).prop_values.offset(i as isize);
            found = 1i32 != 0;
            break ;
        } else { i = i.wrapping_add(1) }
    }
    drmModeFreeObjectProperties(props);
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn get_drm_prop_blob(mut fd: libc::c_int,
                                           mut obj: uint32_t,
                                           mut prop: uint32_t,
                                           mut ret_len: *mut size_t)
 -> *mut libc::c_void {
    let mut blob_id: uint64_t = 0;
    if !get_drm_prop(fd, obj, prop, &mut blob_id) {
        return 0 as *mut libc::c_void
    }
    let mut blob: *mut drmModePropertyBlobRes =
        drmModeGetPropertyBlob(fd, blob_id as uint32_t);
    if blob.is_null() { return 0 as *mut libc::c_void }
    let mut ptr: *mut libc::c_void = malloc((*blob).length as libc::c_ulong);
    if ptr.is_null() {
        drmModeFreePropertyBlob(blob);
        return 0 as *mut libc::c_void
    }
    memcpy(ptr, (*blob).data, (*blob).length as libc::c_ulong);
    *ret_len = (*blob).length as size_t;
    drmModeFreePropertyBlob(blob);
    return ptr;
}
unsafe extern "C" fn run_static_initializers() {
    connector_info =
        [{
             let mut init =
                 prop_info{name:
                               b"CRTC_ID\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               16u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"DPMS\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               4u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                     as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"EDID\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               0u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                     as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"PATH\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               12u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"link-status\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               8u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                     as libc::c_ulong),};
             init
         }];
    crtc_info =
        [{
             let mut init =
                 prop_info{name:
                               b"ACTIVE\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               8u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                     as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"GAMMA_LUT\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               16u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"GAMMA_LUT_SIZE\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               20u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"MODE_ID\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               12u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"rotation\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               0u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                     as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"scaling mode\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               4u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                     as libc::c_ulong),};
             init
         }];
    plane_info =
        [{
             let mut init =
                 prop_info{name:
                               b"CRTC_H\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               40u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"CRTC_ID\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               48u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"CRTC_W\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               36u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"CRTC_X\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               28u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"CRTC_Y\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               32u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"FB_ID\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               44u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"IN_FORMATS\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               8u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                     as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"SRC_H\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               24u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"SRC_W\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               20u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"SRC_X\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               12u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"SRC_Y\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               16u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                      as libc::c_ulong),};
             init
         },
         {
             let mut init =
                 prop_info{name:
                               b"type\x00" as *const u8 as
                                   *const libc::c_char,
                           index:
                               0u64.wrapping_div(::std::mem::size_of::<uint32_t>()
                                                     as libc::c_ulong),};
             init
         }]
}
#[used]
#[cfg_attr ( target_os = "linux", link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows", link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos", link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
