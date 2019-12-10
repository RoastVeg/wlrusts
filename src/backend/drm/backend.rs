use libc;
extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type udev;
    pub type udev_monitor;
    pub type session_impl;
    pub type wlr_renderer_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_texture_impl;
    pub type wlr_surface;
    pub type gbm_device;
    pub type gbm_bo;
    pub type gbm_surface;
    pub type _drmModeAtomicReq;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_event_loop_add_fd(loop_0: *mut wl_event_loop, fd: libc::c_int,
                            mask: uint32_t, func: wl_event_loop_fd_func_t,
                            data: *mut libc::c_void) -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn wl_display_add_destroy_listener(display: *mut wl_display,
                                       listener: *mut wl_listener);
    #[no_mangle]
    fn wlr_session_signal_add(session: *mut wlr_session, fd: libc::c_int,
                              listener: *mut wl_listener);
    #[no_mangle]
    fn wlr_session_close_file(session: *mut wlr_session, fd: libc::c_int);
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* *
 * Initializes common state on a wlr_backend and sets the implementation to the
 * provided wlr_backend_impl reference.
 */
    #[no_mangle]
    fn wlr_backend_init(backend: *mut wlr_backend,
                        impl_0: *const wlr_backend_impl);
    #[no_mangle]
    fn wlr_output_destroy(output: *mut wlr_output);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn drmGetVersion(fd: libc::c_int) -> drmVersionPtr;
    #[no_mangle]
    fn drmFreeVersion(_: drmVersionPtr);
    #[no_mangle]
    fn drmGetDeviceNameFromFd2(fd: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn scan_drm_connectors(state: *mut wlr_drm_backend);
    #[no_mangle]
    fn finish_drm_renderer(renderer: *mut wlr_drm_renderer);
    #[no_mangle]
    fn finish_drm_resources(drm: *mut wlr_drm_backend);
    #[no_mangle]
    fn restore_drm_outputs(drm: *mut wlr_drm_backend);
    #[no_mangle]
    fn init_drm_renderer(drm: *mut wlr_drm_backend,
                         renderer: *mut wlr_drm_renderer,
                         create_render: wlr_renderer_create_func_t) -> bool;
    #[no_mangle]
    fn init_drm_resources(drm: *mut wlr_drm_backend) -> bool;
    #[no_mangle]
    fn check_drm_features(drm: *mut wlr_drm_backend) -> bool;
    #[no_mangle]
    fn set_drm_connector_gamma(output: *mut wlr_output, size: size_t,
                               r: *const uint16_t, g: *const uint16_t,
                               b: *const uint16_t) -> bool;
    #[no_mangle]
    fn enable_drm_connector(output: *mut wlr_output, enable: bool) -> bool;
    #[no_mangle]
    fn drm_connector_set_mode(output: *mut wlr_output,
                              mode: *mut wlr_output_mode) -> bool;
    #[no_mangle]
    fn handle_drm_event(fd: libc::c_int, mask: uint32_t,
                        data: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __clockid_t = libc::c_int;
pub type clockid_t = __clockid_t;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const WL_EVENT_ERROR: C2RustUnnamed = 8;
pub const WL_EVENT_HANGUP: C2RustUnnamed = 4;
pub const WL_EVENT_WRITABLE: C2RustUnnamed = 2;
pub const WL_EVENT_READABLE: C2RustUnnamed = 1;
pub type wl_event_loop_fd_func_t
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: uint32_t,
                                _: *mut libc::c_void) -> libc::c_int>;
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
    pub events: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}
pub type khronos_int32_t = int32_t;
pub type EGLint = khronos_int32_t;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
/*
 * 32 bit regions
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_egl {
    pub platform: EGLenum,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
    pub exts_str: *const libc::c_char,
    pub exts: C2RustUnnamed_1,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub struct wlr_backend_impl {
    pub start: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> ()>,
    pub get_renderer: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                 -> *mut wlr_renderer>,
    pub get_session: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                -> *mut wlr_session>,
    pub get_presentation_clock: Option<unsafe extern "C" fn(_:
                                                                *mut wlr_backend)
                                           -> clockid_t>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed_2,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_3,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub destroy: wl_signal,
}
pub type wlr_renderer_create_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wlr_egl, _: EGLenum,
                                _: *mut libc::c_void, _: *mut EGLint,
                                _: EGLint) -> *mut wlr_renderer>;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}
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
    pub events: C2RustUnnamed_4,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub events: C2RustUnnamed_5,
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
pub struct C2RustUnnamed_5 {
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
pub struct wlr_output_impl {
    pub enable: Option<unsafe extern "C" fn(_: *mut wlr_output, _: bool)
                           -> bool>,
    pub set_mode: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                              _: *mut wlr_output_mode)
                             -> bool>,
    pub set_custom_mode: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                     _: int32_t, _: int32_t,
                                                     _: int32_t) -> bool>,
    pub set_cursor: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                _: *mut wlr_texture,
                                                _: int32_t,
                                                _: wl_output_transform,
                                                _: int32_t, _: int32_t,
                                                _: bool) -> bool>,
    pub move_cursor: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                 _: libc::c_int,
                                                 _: libc::c_int) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_output) -> ()>,
    pub attach_render: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                   _: *mut libc::c_int)
                                  -> bool>,
    pub commit: Option<unsafe extern "C" fn(_: *mut wlr_output) -> bool>,
    pub set_gamma: Option<unsafe extern "C" fn(_: *mut wlr_output, _: size_t,
                                               _: *const uint16_t,
                                               _: *const uint16_t,
                                               _: *const uint16_t) -> bool>,
    pub get_gamma_size: Option<unsafe extern "C" fn(_: *mut wlr_output)
                                   -> size_t>,
    pub export_dmabuf: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                   _:
                                                       *mut wlr_dmabuf_attributes)
                                  -> bool>,
    pub schedule_frame: Option<unsafe extern "C" fn(_: *mut wlr_output)
                                   -> bool>,
    pub attach_buffer: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                   _: *mut wlr_buffer)
                                  -> bool>,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _drmVersion {
    pub version_major: libc::c_int,
    pub version_minor: libc::c_int,
    pub version_patchlevel: libc::c_int,
    pub name_len: libc::c_int,
    pub name: *mut libc::c_char,
    pub date_len: libc::c_int,
    pub date: *mut libc::c_char,
    pub desc_len: libc::c_int,
    pub desc: *mut libc::c_char,
}
pub type drmVersion = _drmVersion;
pub type drmVersionPtr = *mut _drmVersion;
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
    // Used to provide atomic or legacy DRM functions
    // Enable or disable DPMS for connector
    // Pageflip on crtc. If mode is non-NULL perform a full modeset using it.
    // Enable the cursor buffer on crtc. Set bo to NULL to disable
    // Move the cursor on crtc
    // Set the gamma lut on crtc
    // Get the gamma lut size of a crtc
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
/*
 * These types contain the property ids for several DRM objects.
 * See https://01.org/linuxgraphics/gfx-docs/drm/gpu/drm-kms.html#kms-properties
 * for more details.
 */
// not guaranteed to exist
// atomic-modesetting only
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union wlr_drm_crtc_props {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub props: [uint32_t; 6],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub props: [uint32_t; 13],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub type drmModeCrtc = _drmModeCrtc;
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
/* *
 * Add mode to the list of available modes
 */
pub type drmModeModeInfo = _drmModeModeInfo;
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
pub type drmModeAtomicReq = _drmModeAtomicReq;
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
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub props: [uint32_t; 4],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
// Connector is available but no output is plugged in
// An output just has been plugged in and is waiting for a modeset
// DMA-BUF to be displayed on next commit
// Buffer submitted to the kernel but not yet displayed
// Buffer currently being displayed
#[no_mangle]
pub unsafe extern "C" fn get_drm_backend_from_backend(mut wlr_backend:
                                                          *mut wlr_backend)
 -> *mut wlr_drm_backend {
    if wlr_backend_is_drm(wlr_backend) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_backend_is_drm(wlr_backend)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/drm/backend.c\x00" as *const u8 as
                          *const libc::c_char, 20i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 75],
                                                &[libc::c_char; 75]>(b"struct wlr_drm_backend *get_drm_backend_from_backend(struct wlr_backend *)\x00")).as_ptr());
    };
    return wlr_backend as *mut wlr_drm_backend;
}
unsafe extern "C" fn backend_start(mut backend: *mut wlr_backend) -> bool {
    let mut drm: *mut wlr_drm_backend = get_drm_backend_from_backend(backend);
    scan_drm_connectors(drm);
    return 1i32 != 0;
}
unsafe extern "C" fn backend_destroy(mut backend: *mut wlr_backend) {
    if backend.is_null() { return }
    let mut drm: *mut wlr_drm_backend = get_drm_backend_from_backend(backend);
    restore_drm_outputs(drm);
    let mut conn: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
    let mut next: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
    conn =
        ((*drm).outputs.next as *mut libc::c_char).offset(-688) as
            *mut wlr_drm_connector;
    next =
        ((*conn).link.next as *mut libc::c_char).offset(-688) as
            *mut wlr_drm_connector;
    while &mut (*conn).link as *mut wl_list !=
              &mut (*drm).outputs as *mut wl_list {
        wlr_output_destroy(&mut (*conn).output);
        conn = next;
        next =
            ((*conn).link.next as *mut libc::c_char).offset(-688) as
                *mut wlr_drm_connector
    }
    wlr_signal_emit_safe(&mut (*backend).events.destroy,
                         backend as *mut libc::c_void);
    wl_list_remove(&mut (*drm).display_destroy.link);
    wl_list_remove(&mut (*drm).session_destroy.link);
    wl_list_remove(&mut (*drm).session_signal.link);
    wl_list_remove(&mut (*drm).drm_invalidated.link);
    finish_drm_resources(drm);
    finish_drm_renderer(&mut (*drm).renderer);
    wlr_session_close_file((*drm).session, (*drm).fd);
    wl_event_source_remove((*drm).drm_event);
    free(drm as *mut libc::c_void);
}
unsafe extern "C" fn backend_get_renderer(mut backend: *mut wlr_backend)
 -> *mut wlr_renderer {
    let mut drm: *mut wlr_drm_backend = get_drm_backend_from_backend(backend);
    if !(*drm).parent.is_null() {
        return (*(*drm).parent).renderer.wlr_rend
    } else { return (*drm).renderer.wlr_rend };
}
unsafe extern "C" fn backend_get_presentation_clock(mut backend:
                                                        *mut wlr_backend)
 -> clockid_t {
    let mut drm: *mut wlr_drm_backend = get_drm_backend_from_backend(backend);
    return (*drm).clock;
}
static mut backend_impl: wlr_backend_impl =
    unsafe {
        {
            let mut init =
                wlr_backend_impl{start:
                                     Some(backend_start as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> bool),
                                 destroy:
                                     Some(backend_destroy as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> ()),
                                 get_renderer:
                                     Some(backend_get_renderer as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> *mut wlr_renderer),
                                 get_session: None,
                                 get_presentation_clock:
                                     Some(backend_get_presentation_clock as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> clockid_t),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_is_drm(mut b: *mut wlr_backend) -> bool {
    return (*b).impl_0 ==
               &mut backend_impl as *mut wlr_backend_impl as
                   *const wlr_backend_impl;
}
unsafe extern "C" fn session_signal(mut listener: *mut wl_listener,
                                    mut data: *mut libc::c_void) {
    let mut drm: *mut wlr_drm_backend =
        (listener as *mut libc::c_char).offset(-168) as *mut wlr_drm_backend;
    let mut session: *mut wlr_session = data as *mut wlr_session;
    if (*session).active {
        _wlr_log(WLR_INFO,
                 b"[%s:%d] DRM fd resumed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/backend.c\x00" as *const u8 as
                     *const libc::c_char, 91i32);
        scan_drm_connectors(drm);
        let mut conn: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
        conn =
            ((*drm).outputs.next as *mut libc::c_char).offset(-688) as
                *mut wlr_drm_connector;
        while &mut (*conn).link as *mut wl_list !=
                  &mut (*drm).outputs as *mut wl_list {
            if (*conn).output.enabled {
                drm_connector_set_mode(&mut (*conn).output,
                                       (*conn).output.current_mode);
            } else { enable_drm_connector(&mut (*conn).output, 0i32 != 0); }
            if !(*conn).crtc.is_null() {
                let mut plane: *mut wlr_drm_plane = (*(*conn).crtc).cursor;
                (*(*drm).iface).crtc_set_cursor.expect("non-null function pointer")(drm,
                                                                                    (*conn).crtc,
                                                                                    if !plane.is_null()
                                                                                           &&
                                                                                           (*plane).cursor_enabled
                                                                                               as
                                                                                               libc::c_int
                                                                                               !=
                                                                                               0
                                                                                       {
                                                                                        (*plane).surf.back
                                                                                    } else {
                                                                                        0
                                                                                            as
                                                                                            *mut gbm_bo
                                                                                    });
                (*(*drm).iface).crtc_move_cursor.expect("non-null function pointer")(drm,
                                                                                     (*conn).crtc,
                                                                                     (*conn).cursor_x,
                                                                                     (*conn).cursor_y);
                if !(*(*conn).crtc).gamma_table.is_null() {
                    let mut size: size_t = (*(*conn).crtc).gamma_table_size;
                    let mut r: *mut uint16_t = (*(*conn).crtc).gamma_table;
                    let mut g: *mut uint16_t =
                        (*(*conn).crtc).gamma_table.offset(size as isize);
                    let mut b: *mut uint16_t =
                        (*(*conn).crtc).gamma_table.offset((2i32 as
                                                                libc::c_ulong).wrapping_mul(size)
                                                               as isize);
                    (*(*drm).iface).crtc_set_gamma.expect("non-null function pointer")(drm,
                                                                                       (*conn).crtc,
                                                                                       size,
                                                                                       r,
                                                                                       g,
                                                                                       b);
                } else {
                    set_drm_connector_gamma(&mut (*conn).output,
                                            0i32 as size_t,
                                            0 as *const uint16_t,
                                            0 as *const uint16_t,
                                            0 as *const uint16_t);
                }
            }
            conn =
                ((*conn).link.next as *mut libc::c_char).offset(-688) as
                    *mut wlr_drm_connector
        }
    } else {
        _wlr_log(WLR_INFO,
                 b"[%s:%d] DRM fd paused\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/backend.c\x00" as *const u8 as
                     *const libc::c_char, 124i32);
    };
}
unsafe extern "C" fn drm_invalidated(mut listener: *mut wl_listener,
                                     mut data: *mut libc::c_void) {
    let mut drm: *mut wlr_drm_backend =
        (listener as *mut libc::c_char).offset(-192) as *mut wlr_drm_backend;
    let mut name: *mut libc::c_char = drmGetDeviceNameFromFd2((*drm).fd);
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] %s invalidated\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/drm/backend.c\x00" as *const u8 as
                 *const libc::c_char, 133i32, name);
    free(name as *mut libc::c_void);
    scan_drm_connectors(drm);
}
unsafe extern "C" fn handle_session_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut drm: *mut wlr_drm_backend =
        (listener as *mut libc::c_char).offset(-144) as *mut wlr_drm_backend;
    backend_destroy(&mut (*drm).backend);
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut drm: *mut wlr_drm_backend =
        (listener as *mut libc::c_char).offset(-120) as *mut wlr_drm_backend;
    backend_destroy(&mut (*drm).backend);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * Creates a DRM backend using the specified GPU file descriptor (typically from
 * a device node in /dev/dri).
 *
 * To slave this to another DRM backend, pass it as the parent (which _must_ be
 * a DRM backend, other kinds of backends raise SIGABRT).
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_drm_backend_create(mut display: *mut wl_display,
                                                mut session: *mut wlr_session,
                                                mut gpu_fd: libc::c_int,
                                                mut parent: *mut wlr_backend,
                                                mut create_renderer_func:
                                                    wlr_renderer_create_func_t)
 -> *mut wlr_backend {
    if !display.is_null() && !session.is_null() && gpu_fd >= 0i32 {
    } else {
        __assert_fail(b"display && session && gpu_fd >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/drm/backend.c\x00" as *const u8 as
                          *const libc::c_char, 154i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 141],
                                                &[libc::c_char; 141]>(b"struct wlr_backend *wlr_drm_backend_create(struct wl_display *, struct wlr_session *, int, struct wlr_backend *, wlr_renderer_create_func_t)\x00")).as_ptr());
    };
    if parent.is_null() || wlr_backend_is_drm(parent) as libc::c_int != 0 {
    } else {
        __assert_fail(b"!parent || wlr_backend_is_drm(parent)\x00" as
                          *const u8 as *const libc::c_char,
                      b"../backend/drm/backend.c\x00" as *const u8 as
                          *const libc::c_char, 155i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 141],
                                                &[libc::c_char; 141]>(b"struct wlr_backend *wlr_drm_backend_create(struct wl_display *, struct wlr_session *, int, struct wlr_backend *, wlr_renderer_create_func_t)\x00")).as_ptr());
    };
    let mut name: *mut libc::c_char = drmGetDeviceNameFromFd2(gpu_fd);
    let mut version: *mut drmVersion = drmGetVersion(gpu_fd);
    _wlr_log(WLR_INFO,
             b"[%s:%d] Initializing DRM backend for %s (%s)\x00" as *const u8
                 as *const libc::c_char,
             b"../backend/drm/backend.c\x00" as *const u8 as
                 *const libc::c_char, 159i32, name, (*version).name);
    free(name as *mut libc::c_void);
    drmFreeVersion(version);
    let mut drm: *mut wlr_drm_backend =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_drm_backend>() as libc::c_ulong) as
            *mut wlr_drm_backend;
    if drm.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/backend.c\x00" as *const u8 as
                     *const libc::c_char, 165i32,
                 strerror(*__errno_location()));
        return 0 as *mut wlr_backend
    }
    wlr_backend_init(&mut (*drm).backend, &mut backend_impl);
    (*drm).session = session;
    wl_list_init(&mut (*drm).outputs);
    (*drm).fd = gpu_fd;
    if !parent.is_null() {
        (*drm).parent = get_drm_backend_from_backend(parent)
    }
    (*drm).drm_invalidated.notify =
        Some(drm_invalidated as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wlr_session_signal_add(session, gpu_fd, &mut (*drm).drm_invalidated);
    (*drm).display = display;
    let mut event_loop: *mut wl_event_loop =
        wl_display_get_event_loop(display);
    (*drm).drm_event =
        wl_event_loop_add_fd(event_loop, (*drm).fd,
                             WL_EVENT_READABLE as libc::c_int as uint32_t,
                             Some(handle_drm_event as
                                      unsafe extern "C" fn(_: libc::c_int,
                                                           _: uint32_t,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),
                             0 as *mut libc::c_void);
    if (*drm).drm_event.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create DRM event source\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/drm/backend.c\x00" as *const u8 as
                     *const libc::c_char, 187i32);
    } else {
        (*drm).session_signal.notify =
            Some(session_signal as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*session).session_signal,
                      &mut (*drm).session_signal);
        if check_drm_features(drm) {
            if init_drm_resources(drm) {
                if !init_drm_renderer(drm, &mut (*drm).renderer,
                                      create_renderer_func) {
                    _wlr_log(WLR_ERROR,
                             b"[%s:%d] Failed to initialize renderer\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../backend/drm/backend.c\x00" as *const u8 as
                                 *const libc::c_char, 203i32);
                } else {
                    (*drm).session_destroy.notify =
                        Some(handle_session_destroy as
                                 unsafe extern "C" fn(_: *mut wl_listener,
                                                      _: *mut libc::c_void)
                                     -> ());
                    wl_signal_add(&mut (*session).events.destroy,
                                  &mut (*drm).session_destroy);
                    (*drm).display_destroy.notify =
                        Some(handle_display_destroy as
                                 unsafe extern "C" fn(_: *mut wl_listener,
                                                      _: *mut libc::c_void)
                                     -> ());
                    wl_display_add_destroy_listener(display,
                                                    &mut (*drm).display_destroy);
                    return &mut (*drm).backend
                }
            }
        }
        wl_list_remove(&mut (*drm).session_signal.link);
        wl_event_source_remove((*drm).drm_event);
    }
    wlr_session_close_file((*drm).session, (*drm).fd);
    free(drm as *mut libc::c_void);
    return 0 as *mut wlr_backend;
}
