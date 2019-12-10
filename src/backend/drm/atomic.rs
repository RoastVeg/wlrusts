use libc;
extern "C" {
    pub type gbm_device;
    pub type gbm_bo;
    pub type gbm_surface;
    pub type _drmModeAtomicReq;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_renderer_impl;
    pub type udev;
    pub type udev_monitor;
    pub type session_impl;
    pub type wlr_backend_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_texture_impl;
    pub type wlr_surface;
    pub type wlr_output_impl;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    // Will log all messages less than or equal to `verbosity`
// If `callback` is NULL, wlr will use its default logger.
// The function can be called multiple times to update the verbosity or
// callback function.
    // Returns the log verbosity provided to wlr_log_init
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn drmModeAtomicAlloc() -> drmModeAtomicReqPtr;
    #[no_mangle]
    fn drmModeAtomicGetCursor(req: drmModeAtomicReqPtr) -> libc::c_int;
    #[no_mangle]
    fn drmModeAtomicSetCursor(req: drmModeAtomicReqPtr, cursor: libc::c_int);
    #[no_mangle]
    fn drmModeAtomicAddProperty(req: drmModeAtomicReqPtr, object_id: uint32_t,
                                property_id: uint32_t, value: uint64_t)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeAtomicCommit(fd: libc::c_int, req: drmModeAtomicReqPtr,
                           flags: uint32_t, user_data: *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeCreatePropertyBlob(fd: libc::c_int, data: *const libc::c_void,
                                 size: size_t, id: *mut uint32_t)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeDestroyPropertyBlob(fd: libc::c_int, id: uint32_t)
     -> libc::c_int;
    #[no_mangle]
    fn get_drm_prop(fd: libc::c_int, obj: uint32_t, prop: uint32_t,
                    ret: *mut uint64_t) -> bool;
    #[no_mangle]
    static legacy_iface: wlr_drm_interface;
    #[no_mangle]
    fn legacy_crtc_move_cursor(drm: *mut wlr_drm_backend,
                               crtc: *mut wlr_drm_crtc, x: libc::c_int,
                               y: libc::c_int) -> bool;
    #[no_mangle]
    fn legacy_crtc_set_cursor(drm: *mut wlr_drm_backend,
                              crtc: *mut wlr_drm_crtc, bo: *mut gbm_bo)
     -> bool;
    #[no_mangle]
    fn get_drm_backend_from_backend(wlr_backend: *mut wlr_backend)
     -> *mut wlr_drm_backend;
    // Returns the DRM framebuffer id for a gbm_bo
    #[no_mangle]
    fn get_fb_for_bo(bo: *mut gbm_bo, drm_format: uint32_t,
                     with_modifiers: bool) -> uint32_t;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __clockid_t = libc::c_int;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
pub type clockid_t = __clockid_t;
pub type __u16 = libc::c_ushort;

#[repr(C)]#[derive(Copy, Clone)]
pub struct drm_color_lut {
    pub red: __u16,
    pub green: __u16,
    pub blue: __u16,
    pub reserved: __u16,
}

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
/* *
 * Add mode to the list of available modes
 */
pub type drmModeModeInfo = _drmModeModeInfo;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModeCrtc {
    pub crtc_id: uint32_t,
    pub buffer_id: uint32_t,
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
    pub mode_valid: libc::c_int,
    pub mode: drmModeModeInfo,
    pub gamma_size: libc::c_int,
}
pub type drmModeCrtc = _drmModeCrtc;
pub type drmModeAtomicReq = _drmModeAtomicReq;
pub type drmModeAtomicReqPtr = *mut _drmModeAtomicReq;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;

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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_crtc {
    pub id: uint32_t,
    pub mode_id: uint32_t,
    pub gamma_lut: uint32_t,
    pub atomic: *mut drmModeAtomicReq,
    pub legacy_crtc: *mut drmModeCrtc,
    pub primary: *mut wlr_drm_plane,
    pub cursor: *mut wlr_drm_plane,
    pub num_overlays: size_t,
    pub overlays: *mut uint32_t,
    pub props: wlr_drm_crtc_props,
    pub connectors: wl_list,
    pub gamma_table: *mut uint16_t,
    pub gamma_table_size: size_t,
    // Used to provide atomic or legacy DRM functions
    // Enable or disable DPMS for connector
    // Pageflip on crtc. If mode is non-NULL perform a full modeset using it.
    // Enable the cursor buffer on crtc. Set bo to NULL to disable
    // Move the cursor on crtc
    // Set the gamma lut on crtc
    // Get the gamma lut size of a crtc
}
/*
 * These types contain the property ids for several DRM objects.
 * See https://01.org/linuxgraphics/gfx-docs/drm/gpu/drm-kms.html#kms-properties
 * for more details.
 */
// not guaranteed to exist
// atomic-modesetting only

#[repr ( C )]#[derive(Copy, Clone)]
pub union wlr_drm_crtc_props {
    pub c2rust_unnamed: C2RustUnnamed,
    pub props: [uint32_t; 6],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub rotation: uint32_t,
    pub scaling_mode: uint32_t,
    pub active: uint32_t,
    pub mode_id: uint32_t,
    pub gamma_lut: uint32_t,
    pub gamma_lut_size: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_plane {
    pub type_0: uint32_t,
    pub id: uint32_t,
    pub surf: wlr_drm_surface,
    pub mgpu_surf: wlr_drm_surface,
    pub drm_format: uint32_t,
    pub formats: wlr_drm_format_set,
    pub matrix: [libc::c_float; 9],
    pub cursor_enabled: bool,
    pub cursor_hotspot_x: int32_t,
    pub cursor_hotspot_y: int32_t,
    pub props: wlr_drm_plane_props,
}

#[repr ( C )]#[derive(Copy, Clone)]
pub union wlr_drm_plane_props {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub props: [uint32_t; 13],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub cap: size_t,
    pub formats: *mut *mut wlr_drm_format,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_format {
    pub format: uint32_t,
    pub len: size_t,
    pub cap: size_t,
    pub modifiers: [uint64_t; 0],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_surface {
    pub renderer: *mut wlr_drm_renderer,
    pub width: uint32_t,
    pub height: uint32_t,
    pub gbm: *mut gbm_surface,
    pub egl: EGLSurface,
    pub front: *mut gbm_bo,
    pub back: *mut gbm_bo,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_renderer {
    pub fd: libc::c_int,
    pub gbm: *mut gbm_device,
    pub egl: wlr_egl,
    pub gbm_format: uint32_t,
    pub wlr_rend: *mut wlr_renderer,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* * Raised when destroyed, passed the wlr_backend reference */
/* * Raised when new inputs are added, passed the wlr_input_device */
/* * Raised when new outputs are added, passed the wlr_output */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_renderer {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_renderer_impl,
    pub events: C2RustUnnamed_1,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_egl {
    pub platform: EGLenum,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
    pub exts_str: *const libc::c_char,
    pub exts: C2RustUnnamed_2,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub bind_wayland_display_wl: bool,
    pub buffer_age_ext: bool,
    pub image_base_khr: bool,
    pub image_dma_buf_export_mesa: bool,
    pub image_dmabuf_import_ext: bool,
    pub image_dmabuf_import_modifiers_ext: bool,
    pub swap_buffers_with_damage_ext: bool,
    pub swap_buffers_with_damage_khr: bool,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_session {
    pub impl_0: *const crate::src::backend::session::direct::session_impl,
    pub session_signal: wl_signal,
    pub active: bool,
    pub vtnr: libc::c_uint,
    pub seat: [libc::c_char; 256],
    pub udev: *mut udev,
    pub mon: *mut udev_monitor,
    pub udev_event: *mut wl_event_source,
    pub devices: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_3,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub destroy: wl_signal,
}
/*
 * 32 bit regions
 */

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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_backend {
    pub impl_0: *const crate::src::backend::backend::wlr_backend_impl,
    pub events: C2RustUnnamed_4,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
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
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_texture {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_texture_impl,
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
    pub texture: *mut wlr_texture,
    pub surface: *mut crate::src::types::data_device::wlr_data_device::wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_5,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
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
    pub backend: *mut wlr_backend,
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
    pub events: C2RustUnnamed_6,
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
pub struct C2RustUnnamed_6 {
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
pub struct wlr_drm_backend {
    pub backend: wlr_backend,
    pub parent: *mut wlr_drm_backend,
    pub iface: *const wlr_drm_interface,
    pub clock: clockid_t,
    pub addfb2_modifiers: bool,
    pub fd: libc::c_int,
    pub num_crtcs: size_t,
    pub crtcs: *mut wlr_drm_crtc,
    pub display: *mut wl_display,
    pub drm_event: *mut wl_event_source,
    pub display_destroy: wl_listener,
    pub session_destroy: wl_listener,
    pub session_signal: wl_listener,
    pub drm_invalidated: wl_listener,
    pub outputs: wl_list,
    pub renderer: wlr_drm_renderer,
    pub session: *mut wlr_session,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_interface {
    pub conn_enable: Option<unsafe extern "C" fn(_: *mut wlr_drm_backend,
                                                 _: *mut wlr_drm_connector,
                                                 _: bool) -> bool>,
    pub crtc_pageflip: Option<unsafe extern "C" fn(_: *mut wlr_drm_backend,
                                                   _: *mut wlr_drm_connector,
                                                   _: *mut wlr_drm_crtc,
                                                   _: uint32_t,
                                                   _: *mut drmModeModeInfo)
                                  -> bool>,
    pub crtc_set_cursor: Option<unsafe extern "C" fn(_: *mut wlr_drm_backend,
                                                     _: *mut wlr_drm_crtc,
                                                     _: *mut gbm_bo) -> bool>,
    pub crtc_move_cursor: Option<unsafe extern "C" fn(_: *mut wlr_drm_backend,
                                                      _: *mut wlr_drm_crtc,
                                                      _: libc::c_int,
                                                      _: libc::c_int)
                                     -> bool>,
    pub crtc_set_gamma: Option<unsafe extern "C" fn(_: *mut wlr_drm_backend,
                                                    _: *mut wlr_drm_crtc,
                                                    _: size_t,
                                                    _: *mut uint16_t,
                                                    _: *mut uint16_t,
                                                    _: *mut uint16_t)
                                   -> bool>,
    pub crtc_get_gamma_size: Option<unsafe extern "C" fn(_:
                                                             *mut wlr_drm_backend,
                                                         _: *mut wlr_drm_crtc)
                                        -> size_t>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_connector {
    pub output: wlr_output,
    pub state: wlr_drm_connector_state,
    pub desired_mode: *mut wlr_output_mode,
    pub desired_enabled: bool,
    pub id: uint32_t,
    pub crtc: *mut wlr_drm_crtc,
    pub possible_crtc: uint32_t,
    pub props: wlr_drm_connector_props,
    pub width: uint32_t,
    pub height: uint32_t,
    pub cursor_x: int32_t,
    pub cursor_y: int32_t,
    pub old_crtc: *mut drmModeCrtc,
    pub pageflip_pending: bool,
    pub retry_pageflip: *mut wl_event_source,
    pub link: wl_list,
    pub pending_dmabuf: wlr_dmabuf_attributes,
    pub pending_buffer: *mut wlr_buffer,
    pub pending_bo: *mut gbm_bo,
    pub current_buffer: *mut wlr_buffer,
    pub current_bo: *mut gbm_bo,
}

#[repr ( C )]#[derive(Copy, Clone)]
pub union wlr_drm_connector_props {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub props: [uint32_t; 4],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub edid: uint32_t,
    pub dpms: uint32_t,
    pub link_status: uint32_t,
    pub path: uint32_t,
    pub crtc_id: uint32_t,
}
pub type wlr_drm_connector_state = libc::c_uint;
pub const WLR_DRM_CONN_CONNECTED: wlr_drm_connector_state = 3;
pub const WLR_DRM_CONN_CLEANUP: wlr_drm_connector_state = 2;
pub const WLR_DRM_CONN_NEEDS_MODESET: wlr_drm_connector_state = 1;
pub const WLR_DRM_CONN_DISCONNECTED: wlr_drm_connector_state = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct atomic {
    pub req: *mut drmModeAtomicReq,
    pub cursor: libc::c_int,
    pub failed: bool,
}
unsafe extern "C" fn atomic_begin(mut crtc: *mut wlr_drm_crtc,
                                  mut atom: *mut atomic) {
    if (*crtc).atomic.is_null() {
        (*crtc).atomic = drmModeAtomicAlloc();
        if (*crtc).atomic.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/drm/atomic.c\x00" as *const u8 as
                         *const libc::c_char, 20i32,
                     strerror(*__errno_location()));
            (*atom).failed = 1i32 != 0;
            return
        }
    }
    (*atom).req = (*crtc).atomic;
    (*atom).cursor = drmModeAtomicGetCursor((*atom).req);
    (*atom).failed = 0i32 != 0;
}
unsafe extern "C" fn atomic_end(mut drm_fd: libc::c_int,
                                mut atom: *mut atomic) -> bool {
    if (*atom).failed { return 0i32 != 0 }
    let mut flags: uint32_t = (0x100i32 | 0x200i32) as uint32_t;
    if drmModeAtomicCommit(drm_fd, (*atom).req, flags, 0 as *mut libc::c_void)
           != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Atomic test failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/atomic.c\x00" as *const u8 as
                     *const libc::c_char, 38i32,
                 strerror(*__errno_location()));
        drmModeAtomicSetCursor((*atom).req, (*atom).cursor);
        return 0i32 != 0
    }
    return 1i32 != 0;
}
unsafe extern "C" fn atomic_commit(mut drm_fd: libc::c_int,
                                   mut atom: *mut atomic,
                                   mut conn: *mut wlr_drm_connector,
                                   mut flags: uint32_t, mut modeset: bool)
 -> bool {
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*conn).output.backend);
    if (*atom).failed { return 0i32 != 0 }
    let mut ret: libc::c_int =
        drmModeAtomicCommit(drm_fd, (*atom).req, flags,
                            drm as *mut libc::c_void);
    if ret != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] %s: Atomic commit failed (%s): %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/drm/atomic.c\x00" as *const u8 as
                     *const libc::c_char, 57i32,
                 (*conn).output.name.as_mut_ptr(),
                 if modeset as libc::c_int != 0 {
                     b"modeset\x00" as *const u8 as *const libc::c_char
                 } else {
                     b"pageflip\x00" as *const u8 as *const libc::c_char
                 }, strerror(*__errno_location()));
        // Try to commit without new changes
        drmModeAtomicSetCursor((*atom).req, (*atom).cursor);
        if drmModeAtomicCommit(drm_fd, (*atom).req, flags,
                               drm as *mut libc::c_void) != 0 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] %s: Atomic commit without new changes failed (%s): %s\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/drm/atomic.c\x00" as *const u8 as
                         *const libc::c_char, 64i32,
                     (*conn).output.name.as_mut_ptr(),
                     if modeset as libc::c_int != 0 {
                         b"modeset\x00" as *const u8 as *const libc::c_char
                     } else {
                         b"pageflip\x00" as *const u8 as *const libc::c_char
                     }, strerror(*__errno_location()));
        }
    }
    drmModeAtomicSetCursor((*atom).req, 0i32);
    return ret == 0;
}
#[inline]
unsafe extern "C" fn atomic_add(mut atom: *mut atomic, mut id: uint32_t,
                                mut prop: uint32_t, mut val: uint64_t) {
    if !(*atom).failed &&
           drmModeAtomicAddProperty((*atom).req, id, prop, val) < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to add atomic DRM property: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/drm/atomic.c\x00" as *const u8 as
                     *const libc::c_char, 75i32,
                 strerror(*__errno_location()));
        (*atom).failed = 1i32 != 0
    };
}
unsafe extern "C" fn set_plane_props(mut atom: *mut atomic,
                                     mut plane: *mut wlr_drm_plane,
                                     mut crtc_id: uint32_t,
                                     mut fb_id: uint32_t,
                                     mut set_crtc_xy: bool) {
    let mut id: uint32_t = (*plane).id;
    let mut props: *const wlr_drm_plane_props = &mut (*plane).props;
    // The src_* properties are in 16.16 fixed point
    atomic_add(atom, id, (*props).c2rust_unnamed.src_x, 0i32 as uint64_t);
    atomic_add(atom, id, (*props).c2rust_unnamed.src_y, 0i32 as uint64_t);
    atomic_add(atom, id, (*props).c2rust_unnamed.src_w,
               ((*plane).surf.width as uint64_t) << 16i32);
    atomic_add(atom, id, (*props).c2rust_unnamed.src_h,
               ((*plane).surf.height as uint64_t) << 16i32);
    atomic_add(atom, id, (*props).c2rust_unnamed.crtc_w,
               (*plane).surf.width as uint64_t);
    atomic_add(atom, id, (*props).c2rust_unnamed.crtc_h,
               (*plane).surf.height as uint64_t);
    atomic_add(atom, id, (*props).c2rust_unnamed.fb_id, fb_id as uint64_t);
    atomic_add(atom, id, (*props).c2rust_unnamed.crtc_id,
               crtc_id as uint64_t);
    if set_crtc_xy {
        atomic_add(atom, id, (*props).c2rust_unnamed.crtc_x,
                   0i32 as uint64_t);
        atomic_add(atom, id, (*props).c2rust_unnamed.crtc_y,
                   0i32 as uint64_t);
    };
}
unsafe extern "C" fn atomic_crtc_pageflip(mut drm: *mut wlr_drm_backend,
                                          mut conn: *mut wlr_drm_connector,
                                          mut crtc: *mut wlr_drm_crtc,
                                          mut fb_id: uint32_t,
                                          mut mode: *mut drmModeModeInfo)
 -> bool {
    if !mode.is_null() {
        if (*crtc).mode_id != 0i32 as libc::c_uint {
            drmModeDestroyPropertyBlob((*drm).fd, (*crtc).mode_id);
        }
        if drmModeCreatePropertyBlob((*drm).fd, mode as *const libc::c_void,
                                     ::std::mem::size_of::<drmModeModeInfo>()
                                         as libc::c_ulong,
                                     &mut (*crtc).mode_id) != 0 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Unable to create property blob: %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/drm/atomic.c\x00" as *const u8 as
                         *const libc::c_char, 111i32,
                     strerror(*__errno_location()));
            return 0i32 != 0
        }
    }
    let mut flags: uint32_t = 0x1i32 as uint32_t;
    if !mode.is_null() {
        flags |= 0x400i32 as libc::c_uint
    } else { flags |= 0x200i32 as libc::c_uint }
    let mut atom: atomic =
        atomic{req: 0 as *mut drmModeAtomicReq, cursor: 0, failed: false,};
    atomic_begin(crtc, &mut atom);
    atomic_add(&mut atom, (*conn).id, (*conn).props.c2rust_unnamed.crtc_id,
               (*crtc).id as uint64_t);
    if !mode.is_null() &&
           (*conn).props.c2rust_unnamed.link_status != 0i32 as libc::c_uint {
        atomic_add(&mut atom, (*conn).id,
                   (*conn).props.c2rust_unnamed.link_status,
                   0i32 as uint64_t);
    }
    atomic_add(&mut atom, (*crtc).id, (*crtc).props.c2rust_unnamed.mode_id,
               (*crtc).mode_id as uint64_t);
    atomic_add(&mut atom, (*crtc).id, (*crtc).props.c2rust_unnamed.active,
               1i32 as uint64_t);
    set_plane_props(&mut atom, (*crtc).primary, (*crtc).id, fb_id, 1i32 != 0);
    return atomic_commit((*drm).fd, &mut atom, conn, flags, !mode.is_null());
}
unsafe extern "C" fn atomic_conn_enable(mut drm: *mut wlr_drm_backend,
                                        mut conn: *mut wlr_drm_connector,
                                        mut enable: bool) -> bool {
    let mut crtc: *mut wlr_drm_crtc = (*conn).crtc;
    if crtc.is_null() { return !enable }
    let mut atom: atomic =
        atomic{req: 0 as *mut drmModeAtomicReq, cursor: 0, failed: false,};
    atomic_begin(crtc, &mut atom);
    atomic_add(&mut atom, (*crtc).id, (*crtc).props.c2rust_unnamed.active,
               enable as uint64_t);
    if enable {
        atomic_add(&mut atom, (*conn).id,
                   (*conn).props.c2rust_unnamed.crtc_id,
                   (*crtc).id as uint64_t);
        atomic_add(&mut atom, (*crtc).id,
                   (*crtc).props.c2rust_unnamed.mode_id,
                   (*crtc).mode_id as uint64_t);
    } else {
        atomic_add(&mut atom, (*conn).id,
                   (*conn).props.c2rust_unnamed.crtc_id, 0i32 as uint64_t);
        atomic_add(&mut atom, (*crtc).id,
                   (*crtc).props.c2rust_unnamed.mode_id, 0i32 as uint64_t);
    }
    return atomic_commit((*drm).fd, &mut atom, conn, 0x400i32 as uint32_t,
                         1i32 != 0);
}
unsafe extern "C" fn atomic_crtc_set_cursor(mut drm: *mut wlr_drm_backend,
                                            mut crtc: *mut wlr_drm_crtc,
                                            mut bo: *mut gbm_bo) -> bool {
    if crtc.is_null() || (*crtc).cursor.is_null() { return 1i32 != 0 }
    let mut plane: *mut wlr_drm_plane = (*crtc).cursor;
    // We can't use atomic operations on fake planes
    if (*plane).id == 0i32 as libc::c_uint {
        return legacy_crtc_set_cursor(drm, crtc, bo)
    }
    let mut atom: atomic =
        atomic{req: 0 as *mut drmModeAtomicReq, cursor: 0, failed: false,};
    atomic_begin(crtc, &mut atom);
    if !bo.is_null() {
        let mut fb_id: uint32_t =
            get_fb_for_bo(bo, (*plane).drm_format, (*drm).addfb2_modifiers);
        set_plane_props(&mut atom, plane, (*crtc).id, fb_id, 0i32 != 0);
    } else {
        atomic_add(&mut atom, (*plane).id,
                   (*plane).props.c2rust_unnamed.fb_id, 0i32 as uint64_t);
        atomic_add(&mut atom, (*plane).id,
                   (*plane).props.c2rust_unnamed.crtc_id, 0i32 as uint64_t);
    }
    return atomic_end((*drm).fd, &mut atom);
}
unsafe extern "C" fn atomic_crtc_move_cursor(mut drm: *mut wlr_drm_backend,
                                             mut crtc: *mut wlr_drm_crtc,
                                             mut x: libc::c_int,
                                             mut y: libc::c_int) -> bool {
    if crtc.is_null() || (*crtc).cursor.is_null() { return 1i32 != 0 }
    let mut plane: *mut wlr_drm_plane = (*crtc).cursor;
    // We can't use atomic operations on fake planes
    if (*plane).id == 0i32 as libc::c_uint {
        return legacy_crtc_move_cursor(drm, crtc, x, y)
    }
    let mut atom: atomic =
        atomic{req: 0 as *mut drmModeAtomicReq, cursor: 0, failed: false,};
    atomic_begin(crtc, &mut atom);
    atomic_add(&mut atom, (*plane).id, (*plane).props.c2rust_unnamed.crtc_x,
               x as uint64_t);
    atomic_add(&mut atom, (*plane).id, (*plane).props.c2rust_unnamed.crtc_y,
               y as uint64_t);
    return atomic_end((*drm).fd, &mut atom);
}
unsafe extern "C" fn atomic_crtc_set_gamma(mut drm: *mut wlr_drm_backend,
                                           mut crtc: *mut wlr_drm_crtc,
                                           mut size: size_t,
                                           mut r: *mut uint16_t,
                                           mut g: *mut uint16_t,
                                           mut b: *mut uint16_t) -> bool {
    // Fallback to legacy gamma interface when gamma properties are not available
	// (can happen on older Intel GPUs that support gamma but not degamma).
    if (*crtc).props.c2rust_unnamed.gamma_lut == 0i32 as libc::c_uint {
        return legacy_iface.crtc_set_gamma.expect("non-null function pointer")(drm,
                                                                               crtc,
                                                                               size,
                                                                               r,
                                                                               g,
                                                                               b)
    }
    let mut gamma: *mut drm_color_lut =
        malloc(size.wrapping_mul(::std::mem::size_of::<drm_color_lut>() as
                                     libc::c_ulong)) as *mut drm_color_lut;
    if gamma.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate gamma table\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/atomic.c\x00" as *const u8 as
                     *const libc::c_char, 216i32);
        return 0i32 != 0
    }
    let mut i: size_t = 0i32 as size_t;
    while i < size {
        (*gamma.offset(i as isize)).red = *r.offset(i as isize);
        (*gamma.offset(i as isize)).green = *g.offset(i as isize);
        (*gamma.offset(i as isize)).blue = *b.offset(i as isize);
        i = i.wrapping_add(1)
    }
    if (*crtc).gamma_lut != 0i32 as libc::c_uint {
        drmModeDestroyPropertyBlob((*drm).fd, (*crtc).gamma_lut);
    }
    if drmModeCreatePropertyBlob((*drm).fd, gamma as *const libc::c_void,
                                 size.wrapping_mul(::std::mem::size_of::<drm_color_lut>()
                                                       as libc::c_ulong),
                                 &mut (*crtc).gamma_lut) != 0 {
        free(gamma as *mut libc::c_void);
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unable to create property blob: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/drm/atomic.c\x00" as *const u8 as
                     *const libc::c_char, 233i32,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    free(gamma as *mut libc::c_void);
    let mut atom: atomic =
        atomic{req: 0 as *mut drmModeAtomicReq, cursor: 0, failed: false,};
    atomic_begin(crtc, &mut atom);
    atomic_add(&mut atom, (*crtc).id, (*crtc).props.c2rust_unnamed.gamma_lut,
               (*crtc).gamma_lut as uint64_t);
    return atomic_end((*drm).fd, &mut atom);
}
unsafe extern "C" fn atomic_crtc_get_gamma_size(mut drm: *mut wlr_drm_backend,
                                                mut crtc: *mut wlr_drm_crtc)
 -> size_t {
    if (*crtc).props.c2rust_unnamed.gamma_lut_size == 0i32 as libc::c_uint {
        return legacy_iface.crtc_get_gamma_size.expect("non-null function pointer")(drm,
                                                                                    crtc)
    }
    let mut gamma_lut_size: uint64_t = 0;
    if !get_drm_prop((*drm).fd, (*crtc).id,
                     (*crtc).props.c2rust_unnamed.gamma_lut_size,
                     &mut gamma_lut_size) {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unable to get gamma lut size\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/atomic.c\x00" as *const u8 as
                     *const libc::c_char, 253i32);
        return 0i32 as size_t
    }
    return gamma_lut_size;
}
#[no_mangle]
pub static mut atomic_iface: wlr_drm_interface =
{
    let mut init =
        wlr_drm_interface{conn_enable:
                          Some(atomic_conn_enable as
                               unsafe extern "C" fn(_:
                                                    *mut wlr_drm_backend,
                                                    _:
                                                    *mut wlr_drm_connector,
                                                    _: bool)
                                                    -> bool),
                          crtc_pageflip:
                          Some(atomic_crtc_pageflip as
                               unsafe extern "C" fn(_:
                                                    *mut wlr_drm_backend,
                                                    _:
                                                    *mut wlr_drm_connector,
                                                    _:
                                                    *mut wlr_drm_crtc,
                                                    _:
                                                    uint32_t,
                                                    _:
                                                    *mut drmModeModeInfo)
                                                    -> bool),
                          crtc_set_cursor:
                          Some(atomic_crtc_set_cursor as
                               unsafe extern "C" fn(_:
                                                    *mut wlr_drm_backend,
                                                    _:
                                                    *mut wlr_drm_crtc,
                                                    _:
                                                    *mut gbm_bo)
                                                    -> bool),
                          crtc_move_cursor:
                          Some(atomic_crtc_move_cursor as
                               unsafe extern "C" fn(_:
                                                    *mut wlr_drm_backend,
                                                    _:
                                                    *mut wlr_drm_crtc,
                                                    _:
                                                    libc::c_int,
                                                    _:
                                                    libc::c_int)
                                                    -> bool),
                          crtc_set_gamma:
                          Some(atomic_crtc_set_gamma as
                               unsafe extern "C" fn(_:
                                                    *mut wlr_drm_backend,
                                                    _:
                                                    *mut wlr_drm_crtc,
                                                    _: size_t,
                                                    _:
                                                    *mut uint16_t,
                                                    _:
                                                    *mut uint16_t,
                                                    _:
                                                    *mut uint16_t)
                                                    -> bool),
                          crtc_get_gamma_size:
                          Some(atomic_crtc_get_gamma_size as
                               unsafe extern "C" fn(_:
                                                    *mut wlr_drm_backend,
                                                    _:
                                                    *mut wlr_drm_crtc)
                                                    -> size_t),};
    init
};
