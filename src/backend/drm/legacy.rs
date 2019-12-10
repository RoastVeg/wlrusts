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
    pub type udev;
    pub type udev_monitor;
    pub type session_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_backend_impl;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    pub type wlr_surface;
    pub type wlr_output_impl;
    #[no_mangle]
    fn gbm_bo_get_handle(bo: *mut gbm_bo) -> gbm_bo_handle;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn drmModeSetCrtc(fd: libc::c_int, crtcId: uint32_t, bufferId: uint32_t,
                      x: uint32_t, y: uint32_t, connectors: *mut uint32_t,
                      count: libc::c_int, mode: drmModeModeInfoPtr)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeSetCursor(fd: libc::c_int, crtcId: uint32_t,
                        bo_handle: uint32_t, width: uint32_t,
                        height: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn drmModeMoveCursor(fd: libc::c_int, crtcId: uint32_t, x: libc::c_int,
                         y: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn drmModeConnectorSetProperty(fd: libc::c_int, connector_id: uint32_t,
                                   property_id: uint32_t, value: uint64_t)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeCrtcSetGamma(fd: libc::c_int, crtc_id: uint32_t, size: uint32_t,
                           red: *mut uint16_t, green: *mut uint16_t,
                           blue: *mut uint16_t) -> libc::c_int;
    #[no_mangle]
    fn drmModePageFlip(fd: libc::c_int, crtc_id: uint32_t, fb_id: uint32_t,
                       flags: uint32_t, user_data: *mut libc::c_void)
     -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __clockid_t = libc::c_int;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive ( Copy, Clone )]
#[repr ( C )]
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
pub type clockid_t = __clockid_t;
#[derive ( Copy, Clone )]
#[repr(C)]
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
pub type drmModeModeInfoPtr = *mut _drmModeModeInfo;
#[derive ( Copy, Clone )]
#[repr(C)]
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
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_object {
    pub interface: *const wl_interface,
    pub implementation: *const libc::c_void,
    pub id: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_interface {
    pub name: *const libc::c_char,
    pub version: libc::c_int,
    pub method_count: libc::c_int,
    pub methods: *const wl_message,
    pub event_count: libc::c_int,
    pub events: *const wl_message,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_message {
    pub name: *const libc::c_char,
    pub signature: *const libc::c_char,
    pub types: *mut *const wl_interface,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
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
pub struct wl_resource {
    pub object: wl_object,
    pub destroy: wl_resource_destroy_func_t,
    pub link: wl_list,
    pub destroy_signal: wl_signal,
    pub client: *mut wl_client,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
pub type wl_resource_destroy_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_resource) -> ()>;
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
    pub events: C2RustUnnamed,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;
/*
 * Misc structs
 */
/*
 * Fixed point matrices
 */
/* forward declaration (sorry) */
/*
 * Floating point matrices
 */
/* The SEPARABLE_CONVOLUTION filter takes the following parameters:
     *
     *         width:           integer given as 16.16 fixpoint number
     *         height:          integer given as 16.16 fixpoint number
     *         x_phase_bits:	integer given as 16.16 fixpoint
     *         y_phase_bits:	integer given as 16.16 fixpoint
     *         xtables:         (1 << x_phase_bits) tables of size width
     *         ytables:         (1 << y_phase_bits) tables of size height
     *
     * When sampling at (x, y), the location is first rounded to one of
     * n_x_phases * n_y_phases subpixel positions. These subpixel positions
     * determine an xtable and a ytable to use.
     *
     * Conceptually a width x height matrix is then formed in which each entry
     * is the product of the corresponding entries in the x and y tables.
     * This matrix is then aligned with the image pixels such that its center
     * is as close as possible to the subpixel location chosen earlier. Then
     * the image is convolved with the matrix and the resulting pixel returned.
     */
/*
 * Regions
 */
/*  pixman_box16_t	rects[size];   in memory but not explicitly declared */
/* This function exists only to make it possible to preserve
 * the X ABI - it should go away at first opportunity.
 */
/* creation/destruction */
/* manipulation */
/*
 * 32 bit regions
 */
/*  pixman_box32_t	rects[size];   in memory but not explicitly declared */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_drm_format {
    pub format: uint32_t,
    pub len: size_t,
    pub cap: size_t,
    pub modifiers: [uint64_t; 0],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub cap: size_t,
    pub formats: *mut *mut wlr_drm_format,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_egl {
    pub platform: EGLenum,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
    pub exts_str: *const libc::c_char,
    pub exts: C2RustUnnamed_0,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub bind_wayland_display_wl: bool,
    pub buffer_age_ext: bool,
    pub image_base_khr: bool,
    pub image_dma_buf_export_mesa: bool,
    pub image_dmabuf_import_ext: bool,
    pub image_dmabuf_import_modifiers_ext: bool,
    pub swap_buffers_with_damage_ext: bool,
    pub swap_buffers_with_damage_khr: bool,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed_1,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_2,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * A client buffer.
 */
/* *
	 * The buffer resource, if any. Will be NULL if the client destroys it.
	 */
/* *
	 * The buffer's texture, if any. A buffer will not have a texture if the
	 * client destroys the buffer before it has been released.
	 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_texture {
    pub impl_0: *const wlr_texture_impl,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
    pub surface: *mut wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_3,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output {
    pub impl_0: *const wlr_output_impl,
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
    pub events: C2RustUnnamed_4,
    pub idle_frame: *mut wl_event_source,
    pub idle_done: *mut wl_event_source,
    pub attach_render_locks: libc::c_int,
    pub cursors: wl_list,
    pub hardware_cursor: *mut wlr_output_cursor,
    pub software_cursor_locks: libc::c_int,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub damage: pixman_region32_t,
    pub buffer_type: wlr_output_state_buffer_type,
    pub buffer: *mut wlr_buffer,
}
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_drm_renderer {
    pub fd: libc::c_int,
    pub gbm: *mut gbm_device,
    pub egl: wlr_egl,
    pub gbm_format: uint32_t,
    pub wlr_rend: *mut wlr_renderer,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union wlr_drm_crtc_props {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub props: [uint32_t; 6],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub rotation: uint32_t,
    pub scaling_mode: uint32_t,
    pub active: uint32_t,
    pub mode_id: uint32_t,
    pub gamma_lut: uint32_t,
    pub gamma_lut_size: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union wlr_drm_plane_props {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub props: [uint32_t; 13],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_drm_surface {
    pub renderer: *mut wlr_drm_renderer,
    pub width: uint32_t,
    pub height: uint32_t,
    pub gbm: *mut gbm_surface,
    pub egl: EGLSurface,
    pub front: *mut gbm_bo,
    pub back: *mut gbm_bo,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union wlr_drm_connector_props {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub props: [uint32_t; 4],
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
unsafe extern "C" fn legacy_crtc_pageflip(mut drm: *mut wlr_drm_backend,
                                          mut conn: *mut wlr_drm_connector,
                                          mut crtc: *mut wlr_drm_crtc,
                                          mut fb_id: uint32_t,
                                          mut mode: *mut drmModeModeInfo)
 -> bool {
    if !mode.is_null() {
        if drmModeSetCrtc((*drm).fd, (*crtc).id, fb_id, 0i32 as uint32_t,
                          0i32 as uint32_t, &mut (*conn).id, 1i32, mode) != 0
           {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] %s: Failed to set CRTC: %s\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/drm/legacy.c\x00" as *const u8 as
                         *const libc::c_char, 15i32,
                     (*conn).output.name.as_mut_ptr(),
                     strerror(*__errno_location()));
            return 0i32 != 0
        }
    }
    if drmModePageFlip((*drm).fd, (*crtc).id, fb_id, 0x1i32 as uint32_t,
                       drm as *mut libc::c_void) != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] %s: Failed to page flip: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/legacy.c\x00" as *const u8 as
                     *const libc::c_char, 21i32,
                 (*conn).output.name.as_mut_ptr(),
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    return 1i32 != 0;
}
unsafe extern "C" fn legacy_conn_enable(mut drm: *mut wlr_drm_backend,
                                        mut conn: *mut wlr_drm_connector,
                                        mut enable: bool) -> bool {
    let mut ret: libc::c_int =
        drmModeConnectorSetProperty((*drm).fd, (*conn).id,
                                    (*conn).props.c2rust_unnamed.dpms,
                                    if enable as libc::c_int != 0 {
                                        0i32
                                    } else { 3i32 } as uint64_t);
    if !enable {
        drmModeSetCrtc((*drm).fd, (*(*conn).crtc).id, 0i32 as uint32_t,
                       0i32 as uint32_t, 0i32 as uint32_t, 0 as *mut uint32_t,
                       0i32, 0 as drmModeModeInfoPtr);
    }
    return ret >= 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn legacy_crtc_set_cursor(mut drm: *mut wlr_drm_backend,
                                                mut crtc: *mut wlr_drm_crtc,
                                                mut bo: *mut gbm_bo) -> bool {
    if crtc.is_null() || (*crtc).cursor.is_null() { return 1i32 != 0 }
    if bo.is_null() {
        if drmModeSetCursor((*drm).fd, (*crtc).id, 0i32 as uint32_t,
                            0i32 as uint32_t, 0i32 as uint32_t) != 0 {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] Failed to clear hardware cursor: %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/drm/legacy.c\x00" as *const u8 as
                         *const libc::c_char, 49i32,
                     strerror(*__errno_location()));
            return 0i32 != 0
        }
        return 1i32 != 0
    }
    let mut plane: *mut wlr_drm_plane = (*crtc).cursor;
    if drmModeSetCursor((*drm).fd, (*crtc).id, gbm_bo_get_handle(bo).u32_0,
                        (*plane).surf.width, (*plane).surf.height) != 0 {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Failed to set hardware cursor: %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/drm/legacy.c\x00" as *const u8 as
                     *const libc::c_char, 59i32,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    return 1i32 != 0;
}
// ARGB8888 or XRGB8888
// Only used by cursor
// Atomic modesetting only
// Legacy only
/*
	 * We don't support overlay planes yet, but we keep track of them to
	 * give to DRM lease clients.
	 */
// Connector is available but no output is plugged in
// An output just has been plugged in and is waiting for a modeset
// DMA-BUF to be displayed on next commit
// Buffer submitted to the kernel but not yet displayed
// Buffer currently being displayed
#[no_mangle]
pub unsafe extern "C" fn legacy_crtc_move_cursor(mut drm:
                                                     *mut wlr_drm_backend,
                                                 mut crtc: *mut wlr_drm_crtc,
                                                 mut x: libc::c_int,
                                                 mut y: libc::c_int) -> bool {
    return drmModeMoveCursor((*drm).fd, (*crtc).id, x, y) == 0;
}
unsafe extern "C" fn legacy_crtc_set_gamma(mut drm: *mut wlr_drm_backend,
                                           mut crtc: *mut wlr_drm_crtc,
                                           mut size: size_t,
                                           mut r: *mut uint16_t,
                                           mut g: *mut uint16_t,
                                           mut b: *mut uint16_t) -> bool {
    return drmModeCrtcSetGamma((*drm).fd, (*crtc).id, size as uint32_t, r, g,
                               b) == 0;
}
unsafe extern "C" fn legacy_crtc_get_gamma_size(mut drm: *mut wlr_drm_backend,
                                                mut crtc: *mut wlr_drm_crtc)
 -> size_t {
    return (*(*crtc).legacy_crtc).gamma_size as size_t;
}
#[no_mangle]
pub static mut legacy_iface: wlr_drm_interface =
    unsafe {
        {
            let mut init =
                wlr_drm_interface{conn_enable:
                                      Some(legacy_conn_enable as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_drm_backend,
                                                                    _:
                                                                        *mut wlr_drm_connector,
                                                                    _: bool)
                                                   -> bool),
                                  crtc_pageflip:
                                      Some(legacy_crtc_pageflip as
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
                                      Some(legacy_crtc_set_cursor as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_drm_backend,
                                                                    _:
                                                                        *mut wlr_drm_crtc,
                                                                    _:
                                                                        *mut gbm_bo)
                                                   -> bool),
                                  crtc_move_cursor:
                                      Some(legacy_crtc_move_cursor as
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
                                      Some(legacy_crtc_set_gamma as
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
                                      Some(legacy_crtc_get_gamma_size as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_drm_backend,
                                                                    _:
                                                                        *mut wlr_drm_crtc)
                                                   -> size_t),};
            init
        }
    };
