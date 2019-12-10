use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_renderer_impl;
    pub type wlr_backend_impl;
    pub type wlr_texture_impl;
    pub type wlr_surface;
    pub type wlr_output_impl;
    pub type xkb_context;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_pointer_impl;
    pub type wlr_tablet_pad_impl;
    pub type wlr_tablet_impl;
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    /* Note: these are circular dependencies */
    pub type wlr_input_device_impl;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn wl_display_create() -> *mut wl_display;
    #[no_mangle]
    fn wl_display_destroy(display: *mut wl_display);
    #[no_mangle]
    fn wl_display_terminate(display: *mut wl_display);
    #[no_mangle]
    fn wl_display_run(display: *mut wl_display);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wlr_backend_get_renderer(backend: *mut wlr_backend)
     -> *mut wlr_renderer;
    #[no_mangle]
    fn wlr_backend_destroy(backend: *mut wlr_backend);
    #[no_mangle]
    fn wlr_backend_start(backend: *mut wlr_backend) -> bool;
    #[no_mangle]
    fn wlr_backend_autocreate(display: *mut wl_display,
                              create_renderer_func:
                                  wlr_renderer_create_func_t)
     -> *mut wlr_backend;
    /* *
 * Sets the output mode. Enables the output if it's currently disabled.
 */
    #[no_mangle]
    fn wlr_output_set_mode(output: *mut wlr_output,
                           mode: *mut wlr_output_mode) -> bool;
    /* *
 * Computes the transformed and scaled output resolution.
 */
    #[no_mangle]
    fn wlr_output_effective_resolution(output: *mut wlr_output,
                                       width: *mut libc::c_int,
                                       height: *mut libc::c_int);
    /* *
 * Attach the renderer's buffer to the output. Compositors must call this
 * function before rendering. After they are done rendering, they should call
 * `wlr_output_commit` to submit the new frame.
 *
 * If non-NULL, `buffer_age` is set to the drawing buffer age in number of
 * frames or -1 if unknown. This is useful for damage tracking.
 */
    #[no_mangle]
    fn wlr_output_attach_render(output: *mut wlr_output,
                                buffer_age: *mut libc::c_int) -> bool;
    /* *
 * Commit the pending output state. If `wlr_output_attach_render` has been
 * called, the pending frame will be submitted for display.
 *
 * This function schedules a `frame` event.
 */
    #[no_mangle]
    fn wlr_output_commit(output: *mut wlr_output) -> bool;
    /* *
 * Create a new texture from raw pixel data. `stride` is in bytes. The returned
 * texture is mutable.
 */
    #[no_mangle]
    fn wlr_texture_from_pixels(renderer: *mut wlr_renderer,
                               wl_fmt: wl_shm_format, stride: uint32_t,
                               width: uint32_t, height: uint32_t,
                               data: *const libc::c_void) -> *mut wlr_texture;
    /* *
 * Get the texture width and height.
 */
    #[no_mangle]
    fn wlr_texture_get_size(texture: *mut wlr_texture,
                            width: *mut libc::c_int,
                            height: *mut libc::c_int);
    #[no_mangle]
    fn wlr_renderer_clear(r: *mut wlr_renderer, color: *const libc::c_float);
    /* *
 * Destroys this wlr_texture.
 */
    #[no_mangle]
    fn wlr_texture_destroy(texture: *mut wlr_texture);
    #[no_mangle]
    fn wlr_renderer_begin(r: *mut wlr_renderer, width: libc::c_int,
                          height: libc::c_int);
    #[no_mangle]
    fn wlr_renderer_end(r: *mut wlr_renderer);
    /* *
 * Renders the requested texture.
 */
    #[no_mangle]
    fn wlr_render_texture(r: *mut wlr_renderer, texture: *mut wlr_texture,
                          projection: *const libc::c_float, x: libc::c_int,
                          y: libc::c_int, alpha: libc::c_float) -> bool;
    #[no_mangle]
    fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
    #[no_mangle]
    fn xkb_context_unref(context: *mut xkb_context);
    #[no_mangle]
    fn xkb_keymap_new_from_names(context: *mut xkb_context,
                                 names: *const xkb_rule_names,
                                 flags: xkb_keymap_compile_flags)
     -> *mut xkb_keymap;
    #[no_mangle]
    fn xkb_keymap_unref(keymap: *mut xkb_keymap);
    #[no_mangle]
    fn xkb_state_key_get_syms(state: *mut xkb_state, key: xkb_keycode_t,
                              syms_out: *mut *const xkb_keysym_t)
     -> libc::c_int;
    #[no_mangle]
    fn wlr_keyboard_set_keymap(kb: *mut wlr_keyboard,
                               keymap: *mut xkb_keymap);
    // Will log all messages less than or equal to `verbosity`
// If `callback` is NULL, wlr will use its default logger.
// The function can be called multiple times to update the verbosity or
// callback function.
    #[no_mangle]
    fn wlr_log_init(verbosity: wlr_log_importance, callback: wlr_log_func_t);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    static cat_tex: gimp_texture;
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type khronos_int32_t = int32_t;
pub type size_t = libc::c_ulong;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* * Raised when destroyed, passed the wlr_backend reference */
/* * Raised when new inputs are added, passed the wlr_input_device */
/* * Raised when new outputs are added, passed the wlr_output */
pub type wlr_renderer_create_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wlr_egl, _: EGLenum,
                                _: *mut libc::c_void, _: *mut EGLint,
                                _: EGLint) -> *mut wlr_renderer>;
pub type EGLint = khronos_int32_t;
pub type EGLenum = libc::c_uint;
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
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub cap: size_t,
    pub formats: *mut *mut wlr_drm_format,
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
pub type EGLContext = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLDisplay = *mut libc::c_void;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}
pub type wlr_log_func_t
    =
    Option<unsafe extern "C" fn(_: wlr_log_importance, _: *const libc::c_char,
                                _: ::std::ffi::VaList) -> ()>;
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
pub type wl_shm_format = libc::c_uint;
pub const WL_SHM_FORMAT_YVU444: wl_shm_format = 875714137;
pub const WL_SHM_FORMAT_YUV444: wl_shm_format = 875713881;
pub const WL_SHM_FORMAT_YVU422: wl_shm_format = 909203033;
pub const WL_SHM_FORMAT_YUV422: wl_shm_format = 909202777;
pub const WL_SHM_FORMAT_YVU420: wl_shm_format = 842094169;
pub const WL_SHM_FORMAT_YUV420: wl_shm_format = 842093913;
pub const WL_SHM_FORMAT_YVU411: wl_shm_format = 825316953;
pub const WL_SHM_FORMAT_YUV411: wl_shm_format = 825316697;
pub const WL_SHM_FORMAT_YVU410: wl_shm_format = 961893977;
pub const WL_SHM_FORMAT_YUV410: wl_shm_format = 961959257;
pub const WL_SHM_FORMAT_NV61: wl_shm_format = 825644622;
pub const WL_SHM_FORMAT_NV16: wl_shm_format = 909203022;
pub const WL_SHM_FORMAT_NV21: wl_shm_format = 825382478;
pub const WL_SHM_FORMAT_NV12: wl_shm_format = 842094158;
pub const WL_SHM_FORMAT_AYUV: wl_shm_format = 1448433985;
pub const WL_SHM_FORMAT_VYUY: wl_shm_format = 1498765654;
pub const WL_SHM_FORMAT_UYVY: wl_shm_format = 1498831189;
pub const WL_SHM_FORMAT_YVYU: wl_shm_format = 1431918169;
pub const WL_SHM_FORMAT_YUYV: wl_shm_format = 1448695129;
pub const WL_SHM_FORMAT_BGRA1010102: wl_shm_format = 808665410;
pub const WL_SHM_FORMAT_RGBA1010102: wl_shm_format = 808665426;
pub const WL_SHM_FORMAT_ABGR2101010: wl_shm_format = 808665665;
pub const WL_SHM_FORMAT_ARGB2101010: wl_shm_format = 808669761;
pub const WL_SHM_FORMAT_BGRX1010102: wl_shm_format = 808671298;
pub const WL_SHM_FORMAT_RGBX1010102: wl_shm_format = 808671314;
pub const WL_SHM_FORMAT_XBGR2101010: wl_shm_format = 808665688;
pub const WL_SHM_FORMAT_XRGB2101010: wl_shm_format = 808669784;
pub const WL_SHM_FORMAT_BGRA8888: wl_shm_format = 875708738;
pub const WL_SHM_FORMAT_RGBA8888: wl_shm_format = 875708754;
pub const WL_SHM_FORMAT_ABGR8888: wl_shm_format = 875708993;
pub const WL_SHM_FORMAT_BGRX8888: wl_shm_format = 875714626;
pub const WL_SHM_FORMAT_RGBX8888: wl_shm_format = 875714642;
pub const WL_SHM_FORMAT_XBGR8888: wl_shm_format = 875709016;
pub const WL_SHM_FORMAT_BGR888: wl_shm_format = 875710274;
pub const WL_SHM_FORMAT_RGB888: wl_shm_format = 875710290;
pub const WL_SHM_FORMAT_BGR565: wl_shm_format = 909199170;
pub const WL_SHM_FORMAT_RGB565: wl_shm_format = 909199186;
pub const WL_SHM_FORMAT_BGRA5551: wl_shm_format = 892420418;
pub const WL_SHM_FORMAT_RGBA5551: wl_shm_format = 892420434;
pub const WL_SHM_FORMAT_ABGR1555: wl_shm_format = 892420673;
pub const WL_SHM_FORMAT_ARGB1555: wl_shm_format = 892424769;
pub const WL_SHM_FORMAT_BGRX5551: wl_shm_format = 892426306;
pub const WL_SHM_FORMAT_RGBX5551: wl_shm_format = 892426322;
pub const WL_SHM_FORMAT_XBGR1555: wl_shm_format = 892420696;
pub const WL_SHM_FORMAT_XRGB1555: wl_shm_format = 892424792;
pub const WL_SHM_FORMAT_BGRA4444: wl_shm_format = 842088770;
pub const WL_SHM_FORMAT_RGBA4444: wl_shm_format = 842088786;
pub const WL_SHM_FORMAT_ABGR4444: wl_shm_format = 842089025;
pub const WL_SHM_FORMAT_ARGB4444: wl_shm_format = 842093121;
pub const WL_SHM_FORMAT_BGRX4444: wl_shm_format = 842094658;
pub const WL_SHM_FORMAT_RGBX4444: wl_shm_format = 842094674;
pub const WL_SHM_FORMAT_XBGR4444: wl_shm_format = 842089048;
pub const WL_SHM_FORMAT_XRGB4444: wl_shm_format = 842093144;
pub const WL_SHM_FORMAT_BGR233: wl_shm_format = 944916290;
pub const WL_SHM_FORMAT_RGB332: wl_shm_format = 943867730;
pub const WL_SHM_FORMAT_C8: wl_shm_format = 538982467;
pub const WL_SHM_FORMAT_XRGB8888: wl_shm_format = 1;
pub const WL_SHM_FORMAT_ARGB8888: wl_shm_format = 0;
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
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
    pub events: C2RustUnnamed_2,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub events: C2RustUnnamed_3,
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
pub struct C2RustUnnamed_3 {
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
#[derive ( Copy, Clone )]
#[repr(C)]
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_list {
    pub capacity: size_t,
    pub length: size_t,
    pub items: *mut *mut libc::c_void,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
pub type wlr_input_device_type = libc::c_uint;
pub const WLR_INPUT_DEVICE_SWITCH: wlr_input_device_type = 5;
pub const WLR_INPUT_DEVICE_TABLET_PAD: wlr_input_device_type = 4;
pub const WLR_INPUT_DEVICE_TABLET_TOOL: wlr_input_device_type = 3;
pub const WLR_INPUT_DEVICE_TOUCH: wlr_input_device_type = 2;
pub const WLR_INPUT_DEVICE_POINTER: wlr_input_device_type = 1;
pub const WLR_INPUT_DEVICE_KEYBOARD: wlr_input_device_type = 0;
pub type xkb_keycode_t = uint32_t;
pub type xkb_keysym_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xkb_rule_names {
    pub rules: *const libc::c_char,
    pub model: *const libc::c_char,
    pub layout: *const libc::c_char,
    pub variant: *const libc::c_char,
    pub options: *const libc::c_char,
}
pub type xkb_context_flags = libc::c_uint;
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
pub type xkb_keymap_compile_flags = libc::c_uint;
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_keyboard_modifiers {
    pub depressed: xkb_mod_mask_t,
    pub latched: xkb_mod_mask_t,
    pub locked: xkb_mod_mask_t,
    pub group: xkb_mod_mask_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_keyboard {
    pub impl_0: *const wlr_keyboard_impl,
    pub group: *mut wlr_keyboard_group,
    pub keymap_string: *mut libc::c_char,
    pub keymap_size: size_t,
    pub keymap: *mut xkb_keymap,
    pub xkb_state: *mut xkb_state,
    pub led_indexes: [xkb_led_index_t; 3],
    pub mod_indexes: [xkb_mod_index_t; 8],
    pub keycodes: [uint32_t; 32],
    pub num_keycodes: size_t,
    pub modifiers: wlr_keyboard_modifiers,
    pub repeat_info: C2RustUnnamed_5,
    pub events: C2RustUnnamed_4,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub rate: int32_t,
    pub delay: int32_t,
}
pub type wlr_key_state = libc::c_uint;
pub const WLR_KEY_PRESSED: wlr_key_state = 1;
pub const WLR_KEY_RELEASED: wlr_key_state = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_keyboard_key {
    pub time_msec: uint32_t,
    pub keycode: uint32_t,
    pub update_state: bool,
    pub state: wlr_key_state,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
    pub events: C2RustUnnamed_6,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub motion: wl_signal,
    pub motion_absolute: wl_signal,
    pub button: wl_signal,
    pub axis: wl_signal,
    pub frame: wl_signal,
    pub swipe_begin: wl_signal,
    pub swipe_update: wl_signal,
    pub swipe_end: wl_signal,
    pub pinch_begin: wl_signal,
    pub pinch_update: wl_signal,
    pub pinch_end: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_input_device {
    pub impl_0: *const wlr_input_device_impl,
    pub type_0: wlr_input_device_type,
    pub vendor: libc::c_uint,
    pub product: libc::c_uint,
    pub name: *mut libc::c_char,
    pub width_mm: libc::c_double,
    pub height_mm: libc::c_double,
    pub output_name: *mut libc::c_char,
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub events: C2RustUnnamed_7,
    pub data: *mut libc::c_void,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_8 {
    pub _device: *mut libc::c_void,
    pub keyboard: *mut wlr_keyboard,
    pub pointer: *mut wlr_pointer,
    pub switch_device: *mut wlr_switch,
    pub touch: *mut wlr_touch,
    pub tablet: *mut wlr_tablet,
    pub tablet_pad: *mut wlr_tablet_pad,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/*
 * NOTE: the wlr tablet pad implementation does not currently support tablets
 * with more than one mode. I don't own any such hardware so I cannot test it
 * and it is too complicated to make a meaningful implementation of blindly.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet_pad {
    pub impl_0: *mut wlr_tablet_pad_impl,
    pub events: C2RustUnnamed_9,
    pub button_count: size_t,
    pub ring_count: size_t,
    pub strip_count: size_t,
    pub groups: wl_list,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub button: wl_signal,
    pub ring: wl_signal,
    pub strip: wl_signal,
    pub attach_tablet: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet {
    pub impl_0: *mut wlr_tablet_impl,
    pub events: C2RustUnnamed_10,
    pub name: *mut libc::c_char,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub axis: wl_signal,
    pub proximity: wl_signal,
    pub tip: wl_signal,
    pub button: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_touch {
    pub impl_0: *const wlr_touch_impl,
    pub events: C2RustUnnamed_11,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub down: wl_signal,
    pub up: wl_signal,
    pub motion: wl_signal,
    pub cancel: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_switch {
    pub impl_0: *mut wlr_switch_impl,
    pub events: C2RustUnnamed_12,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub toggle: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_touch_up {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_touch_motion {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct gimp_texture {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bytes_per_pixel: libc::c_uint,
    pub pixel_data: [libc::c_uchar; 65537],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_state {
    pub display: *mut wl_display,
    pub renderer: *mut wlr_renderer,
    pub cat_texture: *mut wlr_texture,
    pub touch_points: wl_list,
    pub last_frame: timespec,
    pub new_output: wl_listener,
    pub new_input: wl_listener,
    pub touch: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct touch_point {
    pub touch_id: int32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct touch_state {
    pub sample: *mut sample_state,
    pub device: *mut wlr_input_device,
    pub destroy: wl_listener,
    pub down: wl_listener,
    pub up: wl_listener,
    pub motion: wl_listener,
    pub link: wl_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_output {
    pub sample: *mut sample_state,
    pub output: *mut wlr_output,
    pub frame: wl_listener,
    pub destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_keyboard {
    pub sample: *mut sample_state,
    pub device: *mut wlr_input_device,
    pub key: wl_listener,
    pub destroy: wl_listener,
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
unsafe extern "C" fn output_frame_notify(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut sample_output: *mut sample_output =
        (listener as *mut libc::c_char).offset(-16) as *mut sample_output;
    let mut sample: *mut sample_state = (*sample_output).sample;
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    clock_gettime(1i32, &mut now);
    let mut wlr_output: *mut wlr_output = (*sample_output).output;
    let mut width: int32_t = 0;
    let mut height: int32_t = 0;
    wlr_output_effective_resolution(wlr_output, &mut width, &mut height);
    wlr_output_attach_render(wlr_output, 0 as *mut libc::c_int);
    wlr_renderer_begin((*sample).renderer, (*wlr_output).width,
                       (*wlr_output).height);
    wlr_renderer_clear((*sample).renderer,
                       [0.25f32, 0.25f32, 0.25f32,
                        1i32 as libc::c_float].as_mut_ptr() as
                           *const libc::c_float);
    let mut tex_width: libc::c_int = 0;
    let mut tex_height: libc::c_int = 0;
    wlr_texture_get_size((*sample).cat_texture, &mut tex_width,
                         &mut tex_height);
    let mut p: *mut touch_point = 0 as *mut touch_point;
    p =
        ((*sample).touch_points.next as *mut libc::c_char).offset(-24) as
            *mut touch_point;
    while &mut (*p).link as *mut wl_list !=
              &mut (*sample).touch_points as *mut wl_list {
        let mut x: libc::c_int =
            ((*p).x * width as libc::c_double) as libc::c_int -
                tex_width / 2i32;
        let mut y: libc::c_int =
            ((*p).y * height as libc::c_double) as libc::c_int -
                tex_height / 2i32;
        wlr_render_texture((*sample).renderer, (*sample).cat_texture,
                           (*wlr_output).transform_matrix.as_mut_ptr() as
                               *const libc::c_float, x, y, 1.0f32);
        p =
            ((*p).link.next as *mut libc::c_char).offset(-24) as
                *mut touch_point
    }
    wlr_renderer_end((*sample).renderer);
    wlr_output_commit(wlr_output);
    (*sample).last_frame = now;
}
unsafe extern "C" fn touch_down_notify(mut listener: *mut wl_listener,
                                       mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_touch_motion =
        data as *mut wlr_event_touch_motion;
    let mut tstate: *mut touch_state =
        (listener as *mut libc::c_char).offset(-40) as *mut touch_state;
    let mut sample: *mut sample_state = (*tstate).sample;
    let mut point: *mut touch_point =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<touch_point>() as libc::c_ulong) as
            *mut touch_point;
    (*point).touch_id = (*event).touch_id;
    (*point).x = (*event).x;
    (*point).y = (*event).y;
    wl_list_insert(&mut (*sample).touch_points, &mut (*point).link);
}
unsafe extern "C" fn touch_up_notify(mut listener: *mut wl_listener,
                                     mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_touch_up = data as *mut wlr_event_touch_up;
    let mut tstate: *mut touch_state =
        (listener as *mut libc::c_char).offset(-64) as *mut touch_state;
    let mut sample: *mut sample_state = (*tstate).sample;
    let mut point: *mut touch_point = 0 as *mut touch_point;
    let mut tmp: *mut touch_point = 0 as *mut touch_point;
    point =
        ((*sample).touch_points.next as *mut libc::c_char).offset(-24) as
            *mut touch_point;
    tmp =
        ((*point).link.next as *mut libc::c_char).offset(-24) as
            *mut touch_point;
    while &mut (*point).link as *mut wl_list !=
              &mut (*sample).touch_points as *mut wl_list {
        if (*point).touch_id == (*event).touch_id {
            wl_list_remove(&mut (*point).link);
            break ;
        } else {
            point = tmp;
            tmp =
                ((*point).link.next as *mut libc::c_char).offset(-24) as
                    *mut touch_point
        }
    };
}
unsafe extern "C" fn touch_motion_notify(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_touch_motion =
        data as *mut wlr_event_touch_motion;
    let mut tstate: *mut touch_state =
        (listener as *mut libc::c_char).offset(-88) as *mut touch_state;
    let mut sample: *mut sample_state = (*tstate).sample;
    let mut point: *mut touch_point = 0 as *mut touch_point;
    point =
        ((*sample).touch_points.next as *mut libc::c_char).offset(-24) as
            *mut touch_point;
    while &mut (*point).link as *mut wl_list !=
              &mut (*sample).touch_points as *mut wl_list {
        if (*point).touch_id == (*event).touch_id {
            (*point).x = (*event).x;
            (*point).y = (*event).y;
            break ;
        } else {
            point =
                ((*point).link.next as *mut libc::c_char).offset(-24) as
                    *mut touch_point
        }
    };
}
unsafe extern "C" fn touch_destroy_notify(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut tstate: *mut touch_state =
        (listener as *mut libc::c_char).offset(-16) as *mut touch_state;
    wl_list_remove(&mut (*tstate).link);
    wl_list_remove(&mut (*tstate).destroy.link);
    wl_list_remove(&mut (*tstate).down.link);
    wl_list_remove(&mut (*tstate).up.link);
    wl_list_remove(&mut (*tstate).motion.link);
    free(tstate as *mut libc::c_void);
}
unsafe extern "C" fn output_remove_notify(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut sample_output: *mut sample_output =
        (listener as *mut libc::c_char).offset(-40) as *mut sample_output;
    wl_list_remove(&mut (*sample_output).frame.link);
    wl_list_remove(&mut (*sample_output).destroy.link);
    free(sample_output as *mut libc::c_void);
}
unsafe extern "C" fn new_output_notify(mut listener: *mut wl_listener,
                                       mut data: *mut libc::c_void) {
    let mut output: *mut wlr_output = data as *mut wlr_output;
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-56) as *mut sample_state;
    let mut sample_output: *mut sample_output =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<sample_output>() as libc::c_ulong) as
            *mut sample_output;
    if wl_list_empty(&mut (*output).modes) == 0 {
        let mut mode: *mut wlr_output_mode =
            ((*output).modes.prev as *mut libc::c_char).offset(-16) as
                *mut wlr_output_mode;
        wlr_output_set_mode(output, mode);
    }
    (*sample_output).output = output;
    (*sample_output).sample = sample;
    wl_signal_add(&mut (*output).events.frame, &mut (*sample_output).frame);
    (*sample_output).frame.notify =
        Some(output_frame_notify as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.destroy,
                  &mut (*sample_output).destroy);
    (*sample_output).destroy.notify =
        Some(output_remove_notify as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
}
unsafe extern "C" fn keyboard_key_notify(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut keyboard: *mut sample_keyboard =
        (listener as *mut libc::c_char).offset(-16) as *mut sample_keyboard;
    let mut sample: *mut sample_state = (*keyboard).sample;
    let mut event: *mut wlr_event_keyboard_key =
        data as *mut wlr_event_keyboard_key;
    let mut keycode: uint32_t =
        (*event).keycode.wrapping_add(8i32 as libc::c_uint);
    let mut syms: *const xkb_keysym_t = 0 as *const xkb_keysym_t;
    let mut nsyms: libc::c_int =
        xkb_state_key_get_syms((*(*(*keyboard).device).c2rust_unnamed.keyboard).xkb_state,
                               keycode, &mut syms);
    let mut i: libc::c_int = 0i32;
    while i < nsyms {
        let mut sym: xkb_keysym_t = *syms.offset(i as isize);
        if sym == 0xff1bi32 as libc::c_uint {
            wl_display_terminate((*sample).display);
        }
        i += 1
    };
}
unsafe extern "C" fn keyboard_destroy_notify(mut listener: *mut wl_listener,
                                             mut data: *mut libc::c_void) {
    let mut keyboard: *mut sample_keyboard =
        (listener as *mut libc::c_char).offset(-40) as *mut sample_keyboard;
    wl_list_remove(&mut (*keyboard).destroy.link);
    wl_list_remove(&mut (*keyboard).key.link);
    free(keyboard as *mut libc::c_void);
}
unsafe extern "C" fn new_input_notify(mut listener: *mut wl_listener,
                                      mut data: *mut libc::c_void) {
    let mut device: *mut wlr_input_device = data as *mut wlr_input_device;
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-80) as *mut sample_state;
    let mut keyboard: *mut sample_keyboard = 0 as *mut sample_keyboard;
    let mut rules: xkb_rule_names =
        xkb_rule_names{rules: 0 as *const libc::c_char,
                       model: 0 as *const libc::c_char,
                       layout: 0 as *const libc::c_char,
                       variant: 0 as *const libc::c_char,
                       options: 0 as *const libc::c_char,};
    let mut context: *mut xkb_context = 0 as *mut xkb_context;
    let mut keymap: *mut xkb_keymap = 0 as *mut xkb_keymap;
    match (*device).type_0 as libc::c_uint {
        0 => {
            keyboard =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<sample_keyboard>() as
                           libc::c_ulong) as *mut sample_keyboard;
            (*keyboard).device = device;
            (*keyboard).sample = sample;
            wl_signal_add(&mut (*device).events.destroy,
                          &mut (*keyboard).destroy);
            (*keyboard).destroy.notify =
                Some(keyboard_destroy_notify as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            wl_signal_add(&mut (*(*device).c2rust_unnamed.keyboard).events.key,
                          &mut (*keyboard).key);
            (*keyboard).key.notify =
                Some(keyboard_key_notify as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            rules =
                {
                    let mut init =
                        xkb_rule_names{rules: 0 as *const libc::c_char,
                                       model: 0 as *const libc::c_char,
                                       layout: 0 as *const libc::c_char,
                                       variant: 0 as *const libc::c_char,
                                       options: 0 as *const libc::c_char,};
                    init
                };
            rules.rules =
                getenv(b"XKB_DEFAULT_RULES\x00" as *const u8 as
                           *const libc::c_char);
            rules.model =
                getenv(b"XKB_DEFAULT_MODEL\x00" as *const u8 as
                           *const libc::c_char);
            rules.layout =
                getenv(b"XKB_DEFAULT_LAYOUT\x00" as *const u8 as
                           *const libc::c_char);
            rules.variant =
                getenv(b"XKB_DEFAULT_VARIANT\x00" as *const u8 as
                           *const libc::c_char);
            rules.options =
                getenv(b"XKB_DEFAULT_OPTIONS\x00" as *const u8 as
                           *const libc::c_char);
            context = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
            if context.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to create XKB context\x00" as
                             *const u8 as *const libc::c_char,
                         b"../examples/touch.c\x00" as *const u8 as
                             *const libc::c_char, 209i32);
                exit(1i32);
            }
            keymap =
                xkb_keymap_new_from_names(context, &mut rules,
                                          XKB_KEYMAP_COMPILE_NO_FLAGS);
            if keymap.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to create XKB keymap\x00" as
                             *const u8 as *const libc::c_char,
                         b"../examples/touch.c\x00" as *const u8 as
                             *const libc::c_char, 215i32);
                exit(1i32);
            }
            wlr_keyboard_set_keymap((*device).c2rust_unnamed.keyboard,
                                    keymap);
            xkb_keymap_unref(keymap);
            xkb_context_unref(context);
        }
        2 => {
            let mut tstate: *mut touch_state =
                calloc(::std::mem::size_of::<touch_state>() as libc::c_ulong,
                       1i32 as libc::c_ulong) as *mut touch_state;
            (*tstate).device = device;
            (*tstate).sample = sample;
            (*tstate).destroy.notify =
                Some(touch_destroy_notify as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            wl_signal_add(&mut (*device).events.destroy,
                          &mut (*tstate).destroy);
            (*tstate).down.notify =
                Some(touch_down_notify as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            wl_signal_add(&mut (*(*device).c2rust_unnamed.touch).events.down,
                          &mut (*tstate).down);
            (*tstate).motion.notify =
                Some(touch_motion_notify as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            wl_signal_add(&mut (*(*device).c2rust_unnamed.touch).events.motion,
                          &mut (*tstate).motion);
            (*tstate).up.notify =
                Some(touch_up_notify as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            wl_signal_add(&mut (*(*device).c2rust_unnamed.touch).events.up,
                          &mut (*tstate).up);
            wl_list_insert(&mut (*sample).touch, &mut (*tstate).link);
        }
        _ => { }
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    wlr_log_init(WLR_DEBUG, None);
    let mut display: *mut wl_display = wl_display_create();
    let mut state: sample_state =
        {
            let mut init =
                sample_state{display: display,
                             renderer: 0 as *mut wlr_renderer,
                             cat_texture: 0 as *mut wlr_texture,
                             touch_points:
                                 wl_list{prev: 0 as *mut wl_list,
                                         next: 0 as *mut wl_list,},
                             last_frame: timespec{tv_sec: 0, tv_nsec: 0,},
                             new_output:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             new_input:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             touch:
                                 wl_list{prev: 0 as *mut wl_list,
                                         next: 0 as *mut wl_list,},};
            init
        };
    wl_list_init(&mut state.touch_points);
    wl_list_init(&mut state.touch);
    let mut wlr: *mut wlr_backend = wlr_backend_autocreate(display, None);
    if wlr.is_null() { exit(1i32); }
    wl_signal_add(&mut (*wlr).events.new_output, &mut state.new_output);
    state.new_output.notify =
        Some(new_output_notify as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*wlr).events.new_input, &mut state.new_input);
    state.new_input.notify =
        Some(new_input_notify as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    clock_gettime(1i32, &mut state.last_frame);
    state.renderer = wlr_backend_get_renderer(wlr);
    if state.renderer.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not start compositor, OOM\x00" as *const u8
                     as *const libc::c_char,
                 b"../examples/touch.c\x00" as *const u8 as
                     *const libc::c_char, 265i32);
        exit(1i32);
    }
    state.cat_texture =
        wlr_texture_from_pixels(state.renderer, WL_SHM_FORMAT_ARGB8888,
                                cat_tex.width.wrapping_mul(4i32 as
                                                               libc::c_uint),
                                cat_tex.width, cat_tex.height,
                                cat_tex.pixel_data.as_ptr() as
                                    *const libc::c_void);
    if state.cat_texture.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not start compositor, OOM\x00" as *const u8
                     as *const libc::c_char,
                 b"../examples/touch.c\x00" as *const u8 as
                     *const libc::c_char, 272i32);
        exit(1i32);
    }
    if !wlr_backend_start(wlr) {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to start backend\x00" as *const u8 as
                     *const libc::c_char,
                 b"../examples/touch.c\x00" as *const u8 as
                     *const libc::c_char, 277i32);
        wlr_backend_destroy(wlr);
        exit(1i32);
    }
    wl_display_run(display);
    wlr_texture_destroy(state.cat_texture);
    wl_display_destroy(display);
    return 0;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
