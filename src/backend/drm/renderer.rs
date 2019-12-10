use libc;
extern "C" {
    pub type gbm_device;
    pub type gbm_bo;
    pub type gbm_surface;
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
    pub type _drmModeAtomicReq;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn gbm_device_destroy(gbm: *mut gbm_device);
    #[no_mangle]
    fn gbm_create_device(fd: libc::c_int) -> *mut gbm_device;
    #[no_mangle]
    fn gbm_bo_import(gbm: *mut gbm_device, type_0: uint32_t,
                     buffer: *mut libc::c_void, usage: uint32_t)
     -> *mut gbm_bo;
    #[no_mangle]
    fn gbm_surface_destroy(surface: *mut gbm_surface);
    #[no_mangle]
    fn gbm_bo_get_width(bo: *mut gbm_bo) -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_height(bo: *mut gbm_bo) -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_stride_for_plane(bo: *mut gbm_bo, plane: libc::c_int)
     -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_format(bo: *mut gbm_bo) -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_offset(bo: *mut gbm_bo, plane: libc::c_int) -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_fd(bo: *mut gbm_bo) -> libc::c_int;
    #[no_mangle]
    fn gbm_bo_get_modifier(bo: *mut gbm_bo) -> uint64_t;
    #[no_mangle]
    fn gbm_bo_get_plane_count(bo: *mut gbm_bo) -> libc::c_int;
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
    fn gbm_surface_create(gbm: *mut gbm_device, width: uint32_t,
                          height: uint32_t, format: uint32_t, flags: uint32_t)
     -> *mut gbm_surface;
    #[no_mangle]
    fn gbm_surface_create_with_modifiers(gbm: *mut gbm_device,
                                         width: uint32_t, height: uint32_t,
                                         format: uint32_t,
                                         modifiers: *const uint64_t,
                                         count: libc::c_uint)
     -> *mut gbm_surface;
    #[no_mangle]
    fn gbm_surface_lock_front_buffer(surface: *mut gbm_surface)
     -> *mut gbm_bo;
    #[no_mangle]
    fn gbm_surface_release_buffer(surface: *mut gbm_surface, bo: *mut gbm_bo);
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    // TODO: Allocate and return a wlr_egl
/* *
 * Initializes an EGL context for the given platform and remote display.
 * Will attempt to load all possibly required api functions.
 */
    /* *
 * Frees all related EGL resources, makes the context not-current and
 * unbinds a bound wayland display.
 */
    /* *
 * Binds the given display to the EGL instance.
 * This will allow clients to create EGL surfaces from wayland ones and render
 * to it.
 */
    /* *
 * Returns a surface for the given native window
 * The window must match the remote display the wlr_egl was created with.
 */
    /* *
 * Creates an EGL image from the given wl_drm buffer resource.
 */
    /* *
 * Creates an EGL image from the given dmabuf attributes. Check usability
 * of the dmabuf with wlr_egl_check_import_dmabuf once first.
 */
    /* *
 * Get the available dmabuf formats
 */
    /* *
 * Destroys an EGL image created with the given wlr_egl.
 */
    #[no_mangle]
    fn wlr_egl_destroy_surface(egl: *mut wlr_egl, surface: EGLSurface)
     -> bool;
    #[no_mangle]
    fn wlr_egl_swap_buffers(egl: *mut wlr_egl, surface: EGLSurface,
                            damage: *mut pixman_region32_t) -> bool;
    #[no_mangle]
    fn wlr_egl_make_current(egl: *mut wlr_egl, surface: EGLSurface,
                            buffer_age: *mut libc::c_int) -> bool;
    #[no_mangle]
    fn wlr_egl_create_surface(egl: *mut wlr_egl, window: *mut libc::c_void)
     -> EGLSurface;
    #[no_mangle]
    fn wlr_egl_finish(egl: *mut wlr_egl);
    #[no_mangle]
    fn wlr_drm_format_set_get(set: *const wlr_drm_format_set,
                              format: uint32_t) -> *const wlr_drm_format;
    /* *
 * Create a new texture from a DMA-BUF. The returned texture is immutable.
 */
    #[no_mangle]
    fn wlr_texture_from_dmabuf(renderer: *mut wlr_renderer,
                               attribs: *mut wlr_dmabuf_attributes)
     -> *mut wlr_texture;
    /* *
 * Destroys this wlr_texture.
 */
    #[no_mangle]
    fn wlr_texture_destroy(texture: *mut wlr_texture);
    #[no_mangle]
    fn wlr_renderer_autocreate(egl: *mut wlr_egl, platform: EGLenum,
                               remote_display: *mut libc::c_void,
                               config_attribs: *mut EGLint, visual_id: EGLint)
     -> *mut wlr_renderer;
    #[no_mangle]
    fn wlr_renderer_begin(r: *mut wlr_renderer, width: libc::c_int,
                          height: libc::c_int);
    #[no_mangle]
    fn wlr_renderer_end(r: *mut wlr_renderer);
    #[no_mangle]
    fn wlr_renderer_clear(r: *mut wlr_renderer, color: *const libc::c_float);
    /* *
 * Renders the requested texture using the provided matrix.
 */
    #[no_mangle]
    fn wlr_render_texture_with_matrix(r: *mut wlr_renderer,
                                      texture: *mut wlr_texture,
                                      matrix: *const libc::c_float,
                                      alpha: libc::c_float) -> bool;
    /* *
 * Destroys this wlr_renderer. Textures must be destroyed separately.
 */
    #[no_mangle]
    fn wlr_renderer_destroy(renderer: *mut wlr_renderer);
    /* * Writes a 2D orthographic projection matrix to mat of (width, height) with a
 *  specified wl_output_transform*/
    #[no_mangle]
    fn wlr_matrix_projection(mat: *mut libc::c_float, width: libc::c_int,
                             height: libc::c_int,
                             transform: wl_output_transform);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
}
pub type __u64 = libc::c_ulonglong;
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
pub type gbm_bo_flags = libc::c_uint;
pub const GBM_BO_USE_LINEAR: gbm_bo_flags = 16;
pub const GBM_BO_USE_WRITE: gbm_bo_flags = 8;
pub const GBM_BO_USE_RENDERING: gbm_bo_flags = 4;
pub const GBM_BO_USE_CURSOR_64X64: gbm_bo_flags = 2;
pub const GBM_BO_USE_CURSOR: gbm_bo_flags = 2;
pub const GBM_BO_USE_SCANOUT: gbm_bo_flags = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct gbm_import_fd_data {
    pub fd: libc::c_int,
    pub width: uint32_t,
    pub height: uint32_t,
    pub stride: uint32_t,
    pub format: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct gbm_import_fd_modifier_data {
    pub width: uint32_t,
    pub height: uint32_t,
    pub format: uint32_t,
    pub num_fds: uint32_t,
    pub fds: [libc::c_int; 4],
    pub strides: [libc::c_int; 4],
    pub offsets: [libc::c_int; 4],
    pub modifier: uint64_t,
}
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
pub type khronos_int32_t = int32_t;
pub type EGLint = khronos_int32_t;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
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
pub type clockid_t = __clockid_t;
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
    pub exts: C2RustUnnamed,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_texture {
    pub impl_0: *const wlr_texture_impl,
}
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
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
#[no_mangle]
pub unsafe extern "C" fn init_drm_renderer(mut drm: *mut wlr_drm_backend,
                                           mut renderer:
                                               *mut wlr_drm_renderer,
                                           mut create_renderer_func:
                                               wlr_renderer_create_func_t)
 -> bool {
    (*renderer).gbm = gbm_create_device((*drm).fd);
    if (*renderer).gbm.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create GBM device\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/renderer.c\x00" as *const u8 as
                     *const libc::c_char, 20i32);
        return 0i32 != 0
    }
    if create_renderer_func.is_none() {
        create_renderer_func =
            Some(wlr_renderer_autocreate as
                     unsafe extern "C" fn(_: *mut wlr_egl, _: EGLenum,
                                          _: *mut libc::c_void,
                                          _: *mut EGLint, _: EGLint)
                         -> *mut wlr_renderer)
    }
    static mut config_attribs: [EGLint; 9] =
        [0x3024i32, 1i32, 0x3023i32, 1i32, 0x3022i32, 1i32, 0x3021i32, 1i32,
         0x3038i32];
    (*renderer).gbm_format =
        'A' as i32 as uint32_t | ('R' as i32 as uint32_t) << 8i32 |
            ('2' as i32 as uint32_t) << 16i32 |
            ('4' as i32 as uint32_t) << 24i32;
    (*renderer).wlr_rend =
        create_renderer_func.expect("non-null function pointer")(&mut (*renderer).egl,
                                                                 0x31d7i32 as
                                                                     EGLenum,
                                                                 (*renderer).gbm
                                                                     as
                                                                     *mut libc::c_void,
                                                                 config_attribs.as_mut_ptr(),
                                                                 (*renderer).gbm_format
                                                                     as
                                                                     EGLint);
    if (*renderer).wlr_rend.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create EGL/WLR renderer\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/drm/renderer.c\x00" as *const u8 as
                     *const libc::c_char, 41i32);
        gbm_device_destroy((*renderer).gbm);
        return 0i32 != 0
    } else { (*renderer).fd = (*drm).fd; return 1i32 != 0 };
}
#[no_mangle]
pub unsafe extern "C" fn finish_drm_renderer(mut renderer:
                                                 *mut wlr_drm_renderer) {
    if renderer.is_null() { return }
    wlr_renderer_destroy((*renderer).wlr_rend);
    wlr_egl_finish(&mut (*renderer).egl);
    gbm_device_destroy((*renderer).gbm);
}
#[no_mangle]
pub unsafe extern "C" fn init_drm_surface(mut surf: *mut wlr_drm_surface,
                                          mut renderer: *mut wlr_drm_renderer,
                                          mut width: uint32_t,
                                          mut height: uint32_t,
                                          mut format: uint32_t,
                                          mut set: *mut wlr_drm_format_set,
                                          mut flags: uint32_t) -> bool {
    if (*surf).width == width && (*surf).height == height { return 1i32 != 0 }
    (*surf).renderer = renderer;
    (*surf).width = width;
    (*surf).height = height;
    if !(*surf).gbm.is_null() {
        if !(*surf).front.is_null() {
            gbm_surface_release_buffer((*surf).gbm, (*surf).front);
            (*surf).front = 0 as *mut gbm_bo
        }
        if !(*surf).back.is_null() {
            gbm_surface_release_buffer((*surf).gbm, (*surf).back);
            (*surf).back = 0 as *mut gbm_bo
        }
        gbm_surface_destroy((*surf).gbm);
        (*surf).gbm = 0 as *mut gbm_surface
    }
    wlr_egl_destroy_surface(&mut (*(*surf).renderer).egl, (*surf).egl);
    if flags & GBM_BO_USE_LINEAR as libc::c_int as libc::c_uint == 0 &&
           !set.is_null() {
        let mut drm_format: *const wlr_drm_format =
            wlr_drm_format_set_get(set, format);
        if !drm_format.is_null() {
            (*surf).gbm =
                gbm_surface_create_with_modifiers((*renderer).gbm, width,
                                                  height, format,
                                                  (*drm_format).modifiers.as_ptr(),
                                                  (*drm_format).len as
                                                      libc::c_uint)
        }
    }
    if (*surf).gbm.is_null() {
        (*surf).gbm =
            gbm_surface_create((*renderer).gbm, width, height, format,
                               GBM_BO_USE_RENDERING as libc::c_int as
                                   libc::c_uint | flags)
    }
    if (*surf).gbm.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create GBM surface: %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/drm/renderer.c\x00" as *const u8 as
                     *const libc::c_char, 102i32,
                 strerror(*__errno_location()));
    } else {
        (*surf).egl =
            wlr_egl_create_surface(&mut (*renderer).egl,
                                   (*surf).gbm as *mut libc::c_void);
        if (*surf).egl.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to create EGL surface\x00" as *const u8
                         as *const libc::c_char,
                     b"../backend/drm/renderer.c\x00" as *const u8 as
                         *const libc::c_char, 108i32);
            gbm_surface_destroy((*surf).gbm);
        } else { return 1i32 != 0 }
    }
    memset(surf as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<wlr_drm_surface>() as libc::c_ulong);
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn finish_drm_surface(mut surf: *mut wlr_drm_surface) {
    if surf.is_null() || (*surf).renderer.is_null() { return }
    if !(*surf).front.is_null() {
        gbm_surface_release_buffer((*surf).gbm, (*surf).front);
    }
    if !(*surf).back.is_null() {
        gbm_surface_release_buffer((*surf).gbm, (*surf).back);
    }
    wlr_egl_destroy_surface(&mut (*(*surf).renderer).egl, (*surf).egl);
    if !(*surf).gbm.is_null() { gbm_surface_destroy((*surf).gbm); }
    memset(surf as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<wlr_drm_surface>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn make_drm_surface_current(mut surf:
                                                      *mut wlr_drm_surface,
                                                  mut buffer_damage:
                                                      *mut libc::c_int)
 -> bool {
    return wlr_egl_make_current(&mut (*(*surf).renderer).egl, (*surf).egl,
                                buffer_damage);
}
#[no_mangle]
pub unsafe extern "C" fn swap_drm_surface_buffers(mut surf:
                                                      *mut wlr_drm_surface,
                                                  mut damage:
                                                      *mut pixman_region32_t)
 -> *mut gbm_bo {
    if !(*surf).front.is_null() {
        gbm_surface_release_buffer((*surf).gbm, (*surf).front);
    }
    wlr_egl_swap_buffers(&mut (*(*surf).renderer).egl, (*surf).egl, damage);
    (*surf).front = (*surf).back;
    (*surf).back = gbm_surface_lock_front_buffer((*surf).gbm);
    return (*surf).back;
}
#[no_mangle]
pub unsafe extern "C" fn get_drm_surface_front(mut surf: *mut wlr_drm_surface)
 -> *mut gbm_bo {
    if !(*surf).front.is_null() { return (*surf).front }
    make_drm_surface_current(surf, 0 as *mut libc::c_int);
    let mut renderer: *mut wlr_renderer = (*(*surf).renderer).wlr_rend;
    wlr_renderer_begin(renderer, (*surf).width as libc::c_int,
                       (*surf).height as libc::c_int);
    wlr_renderer_clear(renderer,
                       [0.0f64 as libc::c_float, 0.0f64 as libc::c_float,
                        0.0f64 as libc::c_float,
                        1.0f64 as libc::c_float].as_mut_ptr() as
                           *const libc::c_float);
    wlr_renderer_end(renderer);
    return swap_drm_surface_buffers(surf, 0 as *mut pixman_region32_t);
}
#[no_mangle]
pub unsafe extern "C" fn post_drm_surface(mut surf: *mut wlr_drm_surface) {
    if !(*surf).front.is_null() {
        gbm_surface_release_buffer((*surf).gbm, (*surf).front);
        (*surf).front = 0 as *mut gbm_bo
    };
}
#[no_mangle]
pub unsafe extern "C" fn import_gbm_bo(mut renderer: *mut wlr_drm_renderer,
                                       mut attribs:
                                           *mut wlr_dmabuf_attributes)
 -> *mut gbm_bo {
    if (*attribs).modifier as libc::c_ulonglong ==
           (0i32 as __u64) << 56i32 |
               (1u64 << 56i32).wrapping_sub(1i32 as libc::c_ulonglong) &
                   0xffffffffffffffu64 && (*attribs).n_planes == 1i32 &&
           (*attribs).offset[0] == 0i32 as libc::c_uint {
        let mut data: gbm_import_fd_data =
            {
                let mut init =
                    gbm_import_fd_data{fd: (*attribs).fd[0],
                                       width: (*attribs).width as uint32_t,
                                       height: (*attribs).height as uint32_t,
                                       stride: (*attribs).stride[0],
                                       format: (*attribs).format,};
                init
            };
        return gbm_bo_import((*renderer).gbm, 0x5503i32 as uint32_t,
                             &mut data as *mut gbm_import_fd_data as
                                 *mut libc::c_void,
                             GBM_BO_USE_SCANOUT as libc::c_int as uint32_t)
    } else {
        let mut data_0: gbm_import_fd_modifier_data =
            {
                let mut init =
                    gbm_import_fd_modifier_data{width:
                                                    (*attribs).width as
                                                        uint32_t,
                                                height:
                                                    (*attribs).height as
                                                        uint32_t,
                                                format: (*attribs).format,
                                                num_fds:
                                                    (*attribs).n_planes as
                                                        uint32_t,
                                                fds: [0; 4],
                                                strides: [0; 4],
                                                offsets: [0; 4],
                                                modifier:
                                                    (*attribs).modifier,};
                init
            };
        if (*attribs).n_planes as size_t >
               (::std::mem::size_of::<[libc::c_int; 4]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong) {
            return 0 as *mut gbm_bo
        }
        let mut i: size_t = 0i32 as size_t;
        while i < (*attribs).n_planes as size_t {
            data_0.fds[i as usize] = (*attribs).fd[i as usize];
            data_0.strides[i as usize] =
                (*attribs).stride[i as usize] as libc::c_int;
            data_0.offsets[i as usize] =
                (*attribs).offset[i as usize] as libc::c_int;
            i = i.wrapping_add(1)
        }
        return gbm_bo_import((*renderer).gbm, 0x5504i32 as uint32_t,
                             &mut data_0 as *mut gbm_import_fd_modifier_data
                                 as *mut libc::c_void,
                             GBM_BO_USE_SCANOUT as libc::c_int as uint32_t)
    };
}
#[no_mangle]
pub unsafe extern "C" fn export_drm_bo(mut bo: *mut gbm_bo,
                                       mut attribs:
                                           *mut wlr_dmabuf_attributes)
 -> bool {
    memset(attribs as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<wlr_dmabuf_attributes>() as libc::c_ulong);
    (*attribs).n_planes = gbm_bo_get_plane_count(bo);
    if (*attribs).n_planes > 4i32 { return 0i32 != 0 }
    (*attribs).width = gbm_bo_get_width(bo) as int32_t;
    (*attribs).height = gbm_bo_get_height(bo) as int32_t;
    (*attribs).format = gbm_bo_get_format(bo);
    (*attribs).modifier = gbm_bo_get_modifier(bo);
    let mut i: libc::c_int = 0i32;
    while i < (*attribs).n_planes {
        (*attribs).offset[i as usize] = gbm_bo_get_offset(bo, i);
        (*attribs).stride[i as usize] = gbm_bo_get_stride_for_plane(bo, i);
        (*attribs).fd[i as usize] = gbm_bo_get_fd(bo);
        if (*attribs).fd[i as usize] < 0i32 {
            let mut j: libc::c_int = 0i32;
            while j < i { close((*attribs).fd[j as usize]); j += 1 }
            return 0i32 != 0
        }
        i += 1
    }
    return 1i32 != 0;
}
unsafe extern "C" fn free_tex(mut bo: *mut gbm_bo,
                              mut data: *mut libc::c_void) {
    let mut tex: *mut wlr_texture = data as *mut wlr_texture;
    wlr_texture_destroy(tex);
}
unsafe extern "C" fn get_tex_for_bo(mut renderer: *mut wlr_drm_renderer,
                                    mut bo: *mut gbm_bo) -> *mut wlr_texture {
    let mut tex: *mut wlr_texture =
        gbm_bo_get_user_data(bo) as *mut wlr_texture;
    if !tex.is_null() { return tex }
    let mut attribs: wlr_dmabuf_attributes =
        wlr_dmabuf_attributes{width: 0,
                              height: 0,
                              format: 0,
                              flags: 0,
                              modifier: 0,
                              n_planes: 0,
                              offset: [0; 4],
                              stride: [0; 4],
                              fd: [0; 4],};
    if !export_drm_bo(bo, &mut attribs) { return 0 as *mut wlr_texture }
    tex = wlr_texture_from_dmabuf((*renderer).wlr_rend, &mut attribs);
    if !tex.is_null() {
        gbm_bo_set_user_data(bo, tex as *mut libc::c_void,
                             Some(free_tex as
                                      unsafe extern "C" fn(_: *mut gbm_bo,
                                                           _:
                                                               *mut libc::c_void)
                                          -> ()));
    }
    return tex;
}
#[no_mangle]
pub unsafe extern "C" fn copy_drm_surface_mgpu(mut dest: *mut wlr_drm_surface,
                                               mut src: *mut gbm_bo)
 -> *mut gbm_bo {
    make_drm_surface_current(dest, 0 as *mut libc::c_int);
    let mut tex: *mut wlr_texture = get_tex_for_bo((*dest).renderer, src);
    if !tex.is_null() {
    } else {
        __assert_fail(b"tex\x00" as *const u8 as *const libc::c_char,
                      b"../backend/drm/renderer.c\x00" as *const u8 as
                          *const libc::c_char, 275i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"struct gbm_bo *copy_drm_surface_mgpu(struct wlr_drm_surface *, struct gbm_bo *)\x00")).as_ptr());
    };
    let mut mat: [libc::c_float; 9] = [0.; 9];
    wlr_matrix_projection(mat.as_mut_ptr(), 1i32, 1i32,
                          WL_OUTPUT_TRANSFORM_NORMAL);
    let mut renderer: *mut wlr_renderer = (*(*dest).renderer).wlr_rend;
    wlr_renderer_begin(renderer, (*dest).width as libc::c_int,
                       (*dest).height as libc::c_int);
    wlr_renderer_clear(renderer,
                       [0.0f64 as libc::c_float, 0.0f64 as libc::c_float,
                        0.0f64 as libc::c_float,
                        0.0f64 as libc::c_float].as_mut_ptr() as
                           *const libc::c_float);
    wlr_render_texture_with_matrix(renderer, tex,
                                   mat.as_mut_ptr() as *const libc::c_float,
                                   1.0f32);
    wlr_renderer_end(renderer);
    return swap_drm_surface_buffers(dest, 0 as *mut pixman_region32_t);
}
#[no_mangle]
pub unsafe extern "C" fn init_drm_plane_surfaces(mut plane:
                                                     *mut wlr_drm_plane,
                                                 mut drm:
                                                     *mut wlr_drm_backend,
                                                 mut width: int32_t,
                                                 mut height: uint32_t,
                                                 mut format: uint32_t,
                                                 mut with_modifiers: bool)
 -> bool {
    let mut format_set: *mut wlr_drm_format_set =
        if with_modifiers as libc::c_int != 0 {
            &mut (*plane).formats
        } else { 0 as *mut wlr_drm_format_set };
    if (*drm).parent.is_null() {
        return init_drm_surface(&mut (*plane).surf, &mut (*drm).renderer,
                                width as uint32_t, height, format, format_set,
                                GBM_BO_USE_SCANOUT as libc::c_int as uint32_t)
    }
    if !init_drm_surface(&mut (*plane).surf, &mut (*(*drm).parent).renderer,
                         width as uint32_t, height, format,
                         0 as *mut wlr_drm_format_set,
                         GBM_BO_USE_LINEAR as libc::c_int as uint32_t) {
        return 0i32 != 0
    }
    if !init_drm_surface(&mut (*plane).mgpu_surf, &mut (*drm).renderer,
                         width as uint32_t, height, format, format_set,
                         GBM_BO_USE_SCANOUT as libc::c_int as uint32_t) {
        finish_drm_surface(&mut (*plane).surf);
        return 0i32 != 0
    }
    return 1i32 != 0;
}
