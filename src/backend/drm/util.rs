use libc;
extern "C" {
    pub type gbm_device;
    pub type gbm_bo;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_texture;
    pub type wlr_surface;
    pub type wlr_backend;
    pub type wlr_output_impl;
    #[no_mangle]
    fn gbm_device_get_fd(gbm: *mut gbm_device) -> libc::c_int;
    #[no_mangle]
    fn gbm_bo_get_width(bo: *mut gbm_bo) -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_height(bo: *mut gbm_bo) -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_stride_for_plane(bo: *mut gbm_bo, plane: libc::c_int)
     -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_offset(bo: *mut gbm_bo, plane: libc::c_int) -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_device(bo: *mut gbm_bo) -> *mut gbm_device;
    #[no_mangle]
    fn gbm_bo_get_modifier(bo: *mut gbm_bo) -> uint64_t;
    #[no_mangle]
    fn gbm_bo_get_plane_count(bo: *mut gbm_bo) -> libc::c_int;
    #[no_mangle]
    fn gbm_bo_get_handle_for_plane(bo: *mut gbm_bo, plane: libc::c_int)
     -> gbm_bo_handle;
    #[no_mangle]
    fn gbm_bo_set_user_data(bo: *mut gbm_bo, data: *mut libc::c_void,
                            destroy_user_data:
                                Option<unsafe extern "C" fn(_: *mut gbm_bo,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>);
    #[no_mangle]
    fn gbm_bo_get_user_data(bo: *mut gbm_bo) -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn drmModeAddFB2(fd: libc::c_int, width: uint32_t, height: uint32_t,
                     pixel_format: uint32_t, bo_handles: *const uint32_t,
                     pitches: *const uint32_t, offsets: *const uint32_t,
                     buf_id: *mut uint32_t, flags: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn drmModeAddFB2WithModifiers(fd: libc::c_int, width: uint32_t,
                                  height: uint32_t, pixel_format: uint32_t,
                                  bo_handles: *const uint32_t,
                                  pitches: *const uint32_t,
                                  offsets: *const uint32_t,
                                  modifier: *const uint64_t,
                                  buf_id: *mut uint32_t, flags: uint32_t)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeRmFB(fd: libc::c_int, bufferId: uint32_t) -> libc::c_int;
}
pub type __u64 = libc::c_ulonglong;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;

#[repr ( C )]#[derive(Copy, Clone)]
pub union gbm_bo_handle {
    pub ptr: *mut libc::c_void,
    pub s32: int32_t,
    pub u32_0: uint32_t,
    pub s64: int64_t,
    pub u64_0: uint64_t,
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
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_object {
    pub interface: *const wl_interface,
    pub implementation: *const libc::c_void,
    pub id: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_interface {
    pub name: *const libc::c_char,
    pub version: libc::c_int,
    pub method_count: libc::c_int,
    pub methods: *const wl_message,
    pub event_count: libc::c_int,
    pub events: *const wl_message,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_message {
    pub name: *const libc::c_char,
    pub signature: *const libc::c_char,
    pub types: *mut *const wl_interface,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

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
pub struct wl_resource {
    pub object: wl_object,
    pub destroy: wl_resource_destroy_func_t,
    pub link: wl_list,
    pub destroy_signal: wl_signal,
    pub client: *mut wl_client,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
pub type wl_resource_destroy_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_resource) -> ()>;
pub type wl_output_subpixel = libc::c_uint;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: wl_output_subpixel = 5;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: wl_output_subpixel = 4;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: wl_output_subpixel = 3;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: wl_output_subpixel = 2;
pub const WL_OUTPUT_SUBPIXEL_NONE: wl_output_subpixel = 1;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: wl_output_subpixel = 0;
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * A client buffer.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut crate::src::backend::drm::atomic::wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_cursor {
    pub output: *mut wlr_output,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub enabled: bool,
    pub visible: bool,
    pub width: uint32_t,
    pub height: uint32_t,
    pub hotspot_x: int32_t,
    pub hotspot_y: int32_t,
    pub link: wl_list,
    pub texture: *mut crate::src::backend::drm::atomic::wlr_texture,
    pub surface: *mut crate::src::types::data_device::wlr_data_device::wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
}
/* *
 * A compositor output region. This typically corresponds to a monitor that
 * displays part of the compositor space.
 *
 * The `frame` event will be emitted when it is a good time for the compositor
 * to submit a new frame.
 *
 * To render a new frame, compositors should call `wlr_output_attach_render`,
 * render and call `wlr_output_commit`. No rendering should happen outside a
 * `frame` event handler or before `wlr_output_attach_render`.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output {
    pub impl_0: *const crate::src::backend::drm::backend::wlr_output_impl,
    pub backend: *mut crate::src::backend::backend::wlr_backend,
    pub display: *mut wl_display,
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub name: [libc::c_char; 24],
    pub make: [libc::c_char; 56],
    pub model: [libc::c_char; 16],
    pub serial: [libc::c_char; 16],
    pub phys_width: int32_t,
    pub phys_height: int32_t,
    pub modes: wl_list,
    pub current_mode: *mut wlr_output_mode,
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub enabled: bool,
    pub scale: libc::c_float,
    pub subpixel: wl_output_subpixel,
    pub transform: wl_output_transform,
    pub needs_frame: bool,
    pub damage: pixman_region32_t,
    pub frame_pending: bool,
    pub transform_matrix: [libc::c_float; 9],
    pub pending: wlr_output_state,
    pub commit_seq: uint32_t,
    pub events: C2RustUnnamed_0,
    pub idle_frame: *mut wl_event_source,
    pub idle_done: *mut wl_event_source,
    pub attach_render_locks: libc::c_int,
    pub cursors: wl_list,
    pub hardware_cursor: *mut wlr_output_cursor,
    pub software_cursor_locks: libc::c_int,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub frame: wl_signal,
    pub needs_frame: wl_signal,
    pub precommit: wl_signal,
    pub commit: wl_signal,
    pub present: wl_signal,
    pub enable: wl_signal,
    pub mode: wl_signal,
    pub scale: wl_signal,
    pub transform: wl_signal,
    pub destroy: wl_signal,
}
/* *
 * Holds the double-buffered output state.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub damage: pixman_region32_t,
    pub buffer_type: wlr_output_state_buffer_type,
    pub buffer: *mut wlr_buffer,
    // if WLR_OUTPUT_STATE_BUFFER_SCANOUT
}
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModeModeInfo {
    pub clock: uint32_t,
    pub hdisplay: uint16_t,
    pub hsync_start: uint16_t,
    pub hsync_end: uint16_t,
    pub htotal: uint16_t,
    pub hskew: uint16_t,
    pub vdisplay: uint16_t,
    pub vsync_start: uint16_t,
    pub vsync_end: uint16_t,
    pub vtotal: uint16_t,
    pub vscan: uint16_t,
    pub vrefresh: uint32_t,
    pub flags: uint32_t,
    pub type_0: uint32_t,
    pub name: [libc::c_char; 32],
}
pub type drmModeModeInfo = _drmModeModeInfo;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SKIP: C2RustUnnamed_1 = 4294967294;
pub const UNMATCHED: C2RustUnnamed_1 = 4294967295;
/*
 * Store all of the non-recursive state in a struct, so we aren't literally
 * passing 12 arguments to a function.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct match_state {
    pub num_objs: size_t,
    pub objs: *const uint32_t,
    pub num_res: size_t,
    pub score: size_t,
    pub replaced: size_t,
    pub res: *mut uint32_t,
    pub best: *mut uint32_t,
    pub orig: *const uint32_t,
    pub exit_early: bool,
}
#[no_mangle]
pub unsafe extern "C" fn calculate_refresh_rate(mut mode:
                                                    *const drmModeModeInfo)
 -> int32_t {
    let mut refresh: int32_t =
        (((*mode).clock as libc::c_longlong * 1000000i64 /
              (*mode).htotal as libc::c_longlong +
              ((*mode).vtotal as libc::c_int / 2i32) as libc::c_longlong) /
             (*mode).vtotal as libc::c_longlong) as int32_t;
    if (*mode).flags & (1i32 << 4i32) as libc::c_uint != 0 { refresh *= 2i32 }
    if (*mode).flags & (1i32 << 5i32) as libc::c_uint != 0 { refresh /= 2i32 }
    if (*mode).vscan as libc::c_int > 1i32 {
        refresh /= (*mode).vscan as libc::c_int
    }
    return refresh;
}
// Constructed from http://edid.tv/manufacturer
unsafe extern "C" fn get_manufacturer(mut id: uint16_t)
 -> *const libc::c_char {
    match id as libc::c_int {
        1057 => {
            return b"Avolites Ltd\x00" as *const u8 as *const libc::c_char
        }
        1129 => {
            return b"Ancor Communications Inc\x00" as *const u8 as
                       *const libc::c_char
        }
        1138 => {
            return b"Acer Technologies\x00" as *const u8 as
                       *const libc::c_char
        }
        1153 => {
            return b"Addi-Data GmbH\x00" as *const u8 as *const libc::c_char
        }
        1552 => {
            return b"Apple Computer Inc\x00" as *const u8 as
                       *const libc::c_char
        }
        1643 => { return b"Ask A/S\x00" as *const u8 as *const libc::c_char }
        1748 => {
            return b"Avtek (Electronics) Pty Ltd\x00" as *const u8 as
                       *const libc::c_char
        }
        2511 => {
            return b"Bang & Olufsen\x00" as *const u8 as *const libc::c_char
        }
        3502 => {
            return b"Chimei Innolux Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        3503 => {
            return b"Chi Mei Optoelectronics corp.\x00" as *const u8 as
                       *const libc::c_char
        }
        3663 => {
            return b"Extraordinary Technologies PTY Limited\x00" as *const u8
                       as *const libc::c_char
        }
        4268 => {
            return b"Dell Inc.\x00" as *const u8 as *const libc::c_char
        }
        4323 => {
            return b"Data General Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        4590 => {
            return b"DENON, Ltd.\x00" as *const u8 as *const libc::c_char
        }
        5571 => {
            return b"Eizo Nanao Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        5640 => {
            return b"Epiphan Systems Inc.\x00" as *const u8 as
                       *const libc::c_char
        }
        5904 => {
            return b"Data Export Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        6601 => {
            return b"Funai Electric Co., Ltd.\x00" as *const u8 as
                       *const libc::c_char
        }
        6835 => {
            return b"Fujitsu Siemens Computers GmbH\x00" as *const u8 as
                       *const libc::c_char
        }
        7789 => {
            return b"Goldstar Company Ltd\x00" as *const u8 as
                       *const libc::c_char
        }
        8497 => {
            return b"Kaohsiung Opto Electronics Americas, Inc.\x00" as
                       *const u8 as *const libc::c_char
        }
        8804 => {
            return b"HannStar Display Corp\x00" as *const u8 as
                       *const libc::c_char
        }
        8835 => {
            return b"Hitachi Ltd\x00" as *const u8 as *const libc::c_char
        }
        8944 => {
            return b"Hewlett Packard\x00" as *const u8 as *const libc::c_char
        }
        9684 => {
            return b"Interphase Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        9688 => {
            return b"Communications Supply Corporation (A division of WESCO)\x00"
                       as *const u8 as *const libc::c_char
        }
        9861 => {
            return b"Integrated Tech Express Inc\x00" as *const u8 as
                       *const libc::c_char
        }
        9933 => {
            return b"Iiyama North America\x00" as *const u8 as
                       *const libc::c_char
        }
        12462 => {
            return b"Lenovo Group Limited\x00" as *const u8 as
                       *const libc::c_char
        }
        13368 => {
            return b"Rogen Tech Distribution Inc\x00" as *const u8 as
                       *const libc::c_char
        }
        13479 => {
            return b"Abeam Tech Ltd\x00" as *const u8 as *const libc::c_char
        }
        13481 => {
            return b"Panasonic Industry Company\x00" as *const u8 as
                       *const libc::c_char
        }
        13955 => {
            return b"Mars-Tech Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        13976 => { return b"Matrox\x00" as *const u8 as *const libc::c_char }
        14499 => {
            return b"NEC Corporation\x00" as *const u8 as *const libc::c_char
        }
        14520 => {
            return b"Nexgen Mediatech Inc.\x00" as *const u8 as
                       *const libc::c_char
        }
        15819 => {
            return b"ONKYO Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        15950 => {
            return b"ORION ELECTRIC CO., LTD.\x00" as *const u8 as
                       *const libc::c_char
        }
        16013 => {
            return b"Optoma Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        16082 => {
            return b"Oculus VR, Inc.\x00" as *const u8 as *const libc::c_char
        }
        16652 => {
            return b"Philips Consumer Electronics Company\x00" as *const u8 as
                       *const libc::c_char
        }
        16687 => {
            return b"Pioneer Electronic Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        16850 => {
            return b"Planar Systems, Inc.\x00" as *const u8 as
                       *const libc::c_char
        }
        17555 => {
            return b"Quanta Display Inc.\x00" as *const u8 as
                       *const libc::c_char
        }
        18484 => {
            return b"Rent-A-Tech\x00" as *const u8 as *const libc::c_char
        }
        18606 => {
            return b"Renesas Technology Corp.\x00" as *const u8 as
                       *const libc::c_char
        }
        19501 => {
            return b"Samsung Electric Company\x00" as *const u8 as
                       *const libc::c_char
        }
        19502 => {
            return b"Sanyo Electric Co., Ltd.\x00" as *const u8 as
                       *const libc::c_char
        }
        19619 => {
            return b"Seiko Epson Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        19728 => {
            return b"Sharp Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        19753 => {
            return b"Silicon Image, Inc.\x00" as *const u8 as
                       *const libc::c_char
        }
        19929 => { return b"Sony\x00" as *const u8 as *const libc::c_char }
        20100 => {
            return b"STD Computer Inc\x00" as *const u8 as *const libc::c_char
        }
        20179 => { return b"SVSI\x00" as *const u8 as *const libc::c_char }
        20270 => {
            return b"Synaptics Inc\x00" as *const u8 as *const libc::c_char
        }
        20588 => {
            return b"Technical Concepts Ltd\x00" as *const u8 as
                       *const libc::c_char
        }
        20976 => {
            return b"Orion Communications Co., Ltd.\x00" as *const u8 as
                       *const libc::c_char
        }
        21090 => {
            return b"Toshiba America Info Systems Inc\x00" as *const u8 as
                       *const libc::c_char
        }
        21108 => {
            return b"Transtream Inc\x00" as *const u8 as *const libc::c_char
        }
        21963 => { return b"Unknown\x00" as *const u8 as *const libc::c_char }
        22707 => {
            return b"Vestel Elektronik Sanayi ve Ticaret A. S.\x00" as
                       *const u8 as *const libc::c_char
        }
        22836 => {
            return b"Visitech AS\x00" as *const u8 as *const libc::c_char
        }
        22842 => {
            return b"VIZIO, Inc\x00" as *const u8 as *const libc::c_char
        }
        23139 => {
            return b"ViewSonic Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        26024 => {
            return b"Yamaha Corporation\x00" as *const u8 as
                       *const libc::c_char
        }
        _ => { return b"Unknown\x00" as *const u8 as *const libc::c_char }
    };
}
/* See https://en.wikipedia.org/wiki/Extended_Display_Identification_Data for layout of EDID data.
 * We don't parse the EDID properly. We just expect to receive valid data.
 */
#[no_mangle]
pub unsafe extern "C" fn parse_edid(mut output: *mut wlr_output,
                                    mut len: size_t,
                                    mut data: *const uint8_t) {
    if data.is_null() || len < 128i32 as libc::c_ulong {
        snprintf((*output).make.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong,
                 b"<Unknown>\x00" as *const u8 as *const libc::c_char);
        snprintf((*output).model.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                 b"<Unknown>\x00" as *const u8 as *const libc::c_char);
        return
    }
    let mut id: uint16_t =
        ((*data.offset(8) as libc::c_int) << 8i32 |
             *data.offset(9) as libc::c_int) as uint16_t;
    snprintf((*output).make.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong,
             b"%s\x00" as *const u8 as *const libc::c_char,
             get_manufacturer(id));
    let mut model: uint16_t =
        (*data.offset(10) as libc::c_int |
             (*data.offset(11) as libc::c_int) << 8i32) as uint16_t;
    snprintf((*output).model.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
             b"0x%04X\x00" as *const u8 as *const libc::c_char,
             model as libc::c_int);
    let mut serial: uint32_t =
        (*data.offset(12) as libc::c_int |
             (*data.offset(13) as libc::c_int) << 8i32 |
             (*data.offset(14) as libc::c_int) << 8i32 |
             (*data.offset(15) as libc::c_int) << 8i32) as uint32_t;
    snprintf((*output).serial.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
             b"0x%08X\x00" as *const u8 as *const libc::c_char, serial);
    let mut i: size_t = 72i32 as size_t;
    while i <= 108i32 as libc::c_ulong {
        let mut flag: uint16_t =
            ((*data.offset(i as isize) as libc::c_int) << 8i32 |
                 *data.offset(i.wrapping_add(1i32 as libc::c_ulong) as isize)
                     as libc::c_int) as uint16_t;
        if flag as libc::c_int == 0i32 &&
               *data.offset(i.wrapping_add(3i32 as libc::c_ulong) as isize) as
                   libc::c_int == 0xfci32 {
            sprintf((*output).model.as_mut_ptr(),
                    b"%.13s\x00" as *const u8 as *const libc::c_char,
                    &*data.offset(i.wrapping_add(5i32 as libc::c_ulong) as
                                      isize) as *const uint8_t);
            // Monitor names are terminated by newline if they're too short
            let mut nl: *mut libc::c_char =
                strchr((*output).model.as_mut_ptr(), '\n' as i32);
            if !nl.is_null() { *nl = '\u{0}' as i32 as libc::c_char }
        } else if flag as libc::c_int == 0i32 &&
                      *data.offset(i.wrapping_add(3i32 as libc::c_ulong) as
                                       isize) as libc::c_int == 0xffi32 {
            sprintf((*output).serial.as_mut_ptr(),
                    b"%.13s\x00" as *const u8 as *const libc::c_char,
                    &*data.offset(i.wrapping_add(5i32 as libc::c_ulong) as
                                      isize) as *const uint8_t);
            // Monitor serial numbers are terminated by newline if they're too
			// short
            let mut nl_0: *mut libc::c_char =
                strchr((*output).serial.as_mut_ptr(), '\n' as i32);
            if !nl_0.is_null() { *nl_0 = '\u{0}' as i32 as libc::c_char }
        }
        i =
            (i as libc::c_ulong).wrapping_add(18i32 as libc::c_ulong) as
                size_t as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn conn_get_name(mut type_id: uint32_t)
 -> *const libc::c_char {
    match type_id {
        0 => { return b"Unknown\x00" as *const u8 as *const libc::c_char }
        1 => { return b"VGA\x00" as *const u8 as *const libc::c_char }
        2 => { return b"DVI-I\x00" as *const u8 as *const libc::c_char }
        3 => { return b"DVI-D\x00" as *const u8 as *const libc::c_char }
        4 => { return b"DVI-A\x00" as *const u8 as *const libc::c_char }
        5 => { return b"Composite\x00" as *const u8 as *const libc::c_char }
        6 => { return b"SVIDEO\x00" as *const u8 as *const libc::c_char }
        7 => { return b"LVDS\x00" as *const u8 as *const libc::c_char }
        8 => { return b"Component\x00" as *const u8 as *const libc::c_char }
        9 => { return b"DIN\x00" as *const u8 as *const libc::c_char }
        10 => { return b"DP\x00" as *const u8 as *const libc::c_char }
        11 => { return b"HDMI-A\x00" as *const u8 as *const libc::c_char }
        12 => { return b"HDMI-B\x00" as *const u8 as *const libc::c_char }
        13 => { return b"TV\x00" as *const u8 as *const libc::c_char }
        14 => { return b"eDP\x00" as *const u8 as *const libc::c_char }
        15 => { return b"Virtual\x00" as *const u8 as *const libc::c_char }
        16 => { return b"DSI\x00" as *const u8 as *const libc::c_char }
        17 => { return b"DPI\x00" as *const u8 as *const libc::c_char }
        _ => { return b"Unknown\x00" as *const u8 as *const libc::c_char }
    };
}
unsafe extern "C" fn free_fb(mut bo: *mut gbm_bo,
                             mut data: *mut libc::c_void) {
    let mut id: uint32_t = data as uintptr_t as uint32_t;
    if id != 0 {
        let mut gbm: *mut gbm_device = gbm_bo_get_device(bo);
        drmModeRmFB(gbm_device_get_fd(gbm), id);
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_fb_for_bo(mut bo: *mut gbm_bo,
                                       mut drm_format: uint32_t,
                                       mut with_modifiers: bool) -> uint32_t {
    let mut id: uint32_t = gbm_bo_get_user_data(bo) as uintptr_t as uint32_t;
    if id != 0 { return id }
    let mut gbm: *mut gbm_device = gbm_bo_get_device(bo);
    let mut fd: libc::c_int = gbm_device_get_fd(gbm);
    let mut width: uint32_t = gbm_bo_get_width(bo);
    let mut height: uint32_t = gbm_bo_get_height(bo);
    let mut handles: [uint32_t; 4] = [0i32 as uint32_t, 0, 0, 0];
    let mut strides: [uint32_t; 4] = [0i32 as uint32_t, 0, 0, 0];
    let mut offsets: [uint32_t; 4] = [0i32 as uint32_t, 0, 0, 0];
    let mut modifiers: [uint64_t; 4] = [0i32 as uint64_t, 0, 0, 0];
    let mut i: libc::c_int = 0i32;
    while i < gbm_bo_get_plane_count(bo) {
        handles[i as usize] = gbm_bo_get_handle_for_plane(bo, i).u32_0;
        strides[i as usize] = gbm_bo_get_stride_for_plane(bo, i);
        offsets[i as usize] = gbm_bo_get_offset(bo, i);
        // KMS requires all BO planes to have the same modifier
        modifiers[i as usize] = gbm_bo_get_modifier(bo);
        i += 1
    }
    if with_modifiers as libc::c_int != 0 &&
           gbm_bo_get_modifier(bo) as libc::c_ulonglong !=
               (0i32 as __u64) << 56i32 |
                   (1u64 << 56i32).wrapping_sub(1i32 as libc::c_ulonglong) &
                       0xffffffffffffffu64 {
        if drmModeAddFB2WithModifiers(fd, width, height, drm_format,
                                      handles.as_mut_ptr() as *const uint32_t,
                                      strides.as_mut_ptr() as *const uint32_t,
                                      offsets.as_mut_ptr() as *const uint32_t,
                                      modifiers.as_mut_ptr() as
                                          *const uint64_t, &mut id,
                                      (1i32 << 1i32) as uint32_t) != 0 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Unable to add DRM framebuffer: %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/drm/util.c\x00" as *const u8 as
                         *const libc::c_char, 209i32,
                     strerror(*__errno_location()));
        }
    } else if drmModeAddFB2(fd, width, height, drm_format,
                            handles.as_mut_ptr() as *const uint32_t,
                            strides.as_mut_ptr() as *const uint32_t,
                            offsets.as_mut_ptr() as *const uint32_t, &mut id,
                            0i32 as uint32_t) != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unable to add DRM framebuffer: %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/drm/util.c\x00" as *const u8 as
                     *const libc::c_char, 214i32,
                 strerror(*__errno_location()));
    }
    gbm_bo_set_user_data(bo, id as uintptr_t as *mut libc::c_void,
                         Some(free_fb as
                                  unsafe extern "C" fn(_: *mut gbm_bo,
                                                       _: *mut libc::c_void)
                                      -> ()));
    return id;
}
#[inline]
unsafe extern "C" fn is_taken(mut n: size_t, mut arr: *const uint32_t,
                              mut key: uint32_t) -> bool {
    let mut i: size_t = 0i32 as size_t;
    while i < n {
        if *arr.offset(i as isize) == key { return 1i32 != 0 }
        i = i.wrapping_add(1)
    }
    return 0i32 != 0;
}
/*
 * skips: The number of SKIP elements encountered so far.
 * score: The number of resources we've matched so far.
 * replaced: The number of changes from the original solution.
 * i: The index of the current element.
 *
 * This tries to match a solution as close to st->orig as it can.
 *
 * Returns whether we've set a new best element with this solution.
 */
unsafe extern "C" fn match_obj_(mut st: *mut match_state, mut skips: size_t,
                                mut score: size_t, mut replaced: size_t,
                                mut i: size_t) -> bool {
    // Finished
    if i >= (*st).num_res {
        if score > (*st).score ||
               score == (*st).score && replaced < (*st).replaced {
            (*st).score = score;
            (*st).replaced = replaced;
            memcpy((*st).best as *mut libc::c_void,
                   (*st).res as *const libc::c_void,
                   (::std::mem::size_of::<uint32_t>() as
                        libc::c_ulong).wrapping_mul((*st).num_res));
            (*st).exit_early =
                ((*st).score == (*st).num_res.wrapping_sub(skips) ||
                     (*st).score == (*st).num_objs) &&
                    (*st).replaced == 0i32 as libc::c_ulong;
            return 1i32 != 0
        } else { return 0i32 != 0 }
    }
    if *(*st).orig.offset(i as isize) == SKIP as libc::c_uint {
        *(*st).res.offset(i as isize) = SKIP as libc::c_uint;
        return match_obj_(st, skips.wrapping_add(1i32 as libc::c_ulong),
                          score, replaced,
                          i.wrapping_add(1i32 as libc::c_ulong))
    }
    let mut has_best: bool = 0i32 != 0;
    /*
	 * Attempt to use the current solution first, to try and avoid
	 * recalculating everything
	 */
    if *(*st).orig.offset(i as isize) != UNMATCHED as libc::c_uint &&
           !is_taken(i, (*st).res, *(*st).orig.offset(i as isize)) {
        *(*st).res.offset(i as isize) = *(*st).orig.offset(i as isize);
        let mut obj_score: size_t =
            if *(*st).objs.offset(*(*st).res.offset(i as isize) as isize) !=
                   0i32 as libc::c_uint {
                1i32
            } else { 0i32 } as size_t;
        if match_obj_(st, skips, score.wrapping_add(obj_score), replaced,
                      i.wrapping_add(1i32 as libc::c_ulong)) {
            has_best = 1i32 != 0
        }
    }
    if *(*st).orig.offset(i as isize) == UNMATCHED as libc::c_uint {
        *(*st).res.offset(i as isize) = UNMATCHED as libc::c_uint;
        if match_obj_(st, skips, score, replaced,
                      i.wrapping_add(1i32 as libc::c_ulong)) {
            has_best = 1i32 != 0
        }
    }
    if (*st).exit_early { return 1i32 != 0 }
    if *(*st).orig.offset(i as isize) != UNMATCHED as libc::c_uint {
        replaced = replaced.wrapping_add(1)
    }
    let mut candidate: size_t = 0i32 as size_t;
    while candidate < (*st).num_objs {
        // We tried this earlier
        if !(candidate == *(*st).orig.offset(i as isize) as libc::c_ulong) {
            // Not compatible
            if !(*(*st).objs.offset(candidate as isize) &
                     (1i32 << i) as libc::c_uint == 0) {
                // Already taken
                if !is_taken(i, (*st).res, candidate as uint32_t) {
                    *(*st).res.offset(i as isize) = candidate as uint32_t;
                    let mut obj_score_0: size_t =
                        if *(*st).objs.offset(candidate as isize) !=
                               0i32 as libc::c_uint {
                            1i32
                        } else { 0i32 } as size_t;
                    if match_obj_(st, skips, score.wrapping_add(obj_score_0),
                                  replaced,
                                  i.wrapping_add(1i32 as libc::c_ulong)) {
                        has_best = 1i32 != 0
                    }
                    if (*st).exit_early { return 1i32 != 0 }
                }
            }
        }
        candidate = candidate.wrapping_add(1)
    }
    if has_best { return 1i32 != 0 }
    // Maybe this resource can't be matched
    *(*st).res.offset(i as isize) = UNMATCHED as libc::c_uint;
    return match_obj_(st, skips, score, replaced,
                      i.wrapping_add(1i32 as libc::c_ulong));
}
// Calculates a more accurate refresh rate (mHz) than what mode itself provides
// Populates the make/model/phys_{width,height} of output from the edid data
// Returns the string representation of a DRM output type
// Returns the DRM framebuffer id for a gbm_bo
// Part of match_obj
/*
 * Tries to match some DRM objects with some other DRM resource.
 * e.g. Match CRTCs with Encoders, CRTCs with Planes.
 *
 * objs contains a bit array which resources it can be matched with.
 * e.g. Bit 0 set means can be matched with res[0]
 *
 * res contains an index of which objs it is matched with or UNMATCHED.
 *
 * This solution is left in out.
 * Returns the total number of matched solutions.
 */
#[no_mangle]
pub unsafe extern "C" fn match_obj(mut num_objs: size_t,
                                   mut objs: *const uint32_t,
                                   mut num_res: size_t,
                                   mut res: *const uint32_t,
                                   mut out: *mut uint32_t) -> size_t {
    let vla = num_res as usize;
    let mut solution: Vec<uint32_t> = ::std::vec::from_elem(0, vla);
    let mut i: size_t = 0i32 as size_t;
    while i < num_res {
        *solution.as_mut_ptr().offset(i as isize) = UNMATCHED as libc::c_uint;
        i = i.wrapping_add(1)
    }
    let mut st: match_state =
        {
            let mut init =
                match_state{num_objs: num_objs,
                            objs: objs,
                            num_res: num_res,
                            score: 0i32 as size_t,
                            replaced: 18446744073709551615u64,
                            res: solution.as_mut_ptr(),
                            best: out,
                            orig: res,
                            exit_early: 0i32 != 0,};
            init
        };
    match_obj_(&mut st, 0i32 as size_t, 0i32 as size_t, 0i32 as size_t,
               0i32 as size_t);
    return st.score;
}
