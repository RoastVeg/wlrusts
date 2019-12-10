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
    pub type wlr_texture_impl;
    pub type wlr_surface;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_event_loop_add_timer(loop_0: *mut wl_event_loop,
                               func: wl_event_loop_timer_func_t,
                               data: *mut libc::c_void)
     -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_timer_update(source: *mut wl_event_source,
                                    ms_delay: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    #[no_mangle]
    fn wlr_output_send_present(output: *mut wlr_output,
                               event: *mut wlr_output_event_present);
    #[no_mangle]
    fn wlr_output_send_frame(output: *mut wlr_output);
    #[no_mangle]
    fn wlr_output_update_enabled(output: *mut wlr_output, enabled: bool);
    #[no_mangle]
    fn wlr_output_update_custom_mode(output: *mut wlr_output, width: int32_t,
                                     height: int32_t, refresh: int32_t);
    #[no_mangle]
    fn wlr_output_init(output: *mut wlr_output, backend: *mut wlr_backend,
                       impl_0: *const wlr_output_impl,
                       display: *mut wl_display);
    #[no_mangle]
    fn eglCreatePbufferSurface(dpy: EGLDisplay, config: EGLConfig,
                               attrib_list: *const EGLint) -> EGLSurface;
    #[no_mangle]
    fn wlr_egl_make_current(egl: *mut wlr_egl, surface: EGLSurface,
                            buffer_age: *mut libc::c_int) -> bool;
    #[no_mangle]
    fn wlr_egl_destroy_surface(egl: *mut wlr_egl, surface: EGLSurface)
     -> bool;
    #[no_mangle]
    fn wlr_output_destroy(output: *mut wlr_output);
    #[no_mangle]
    fn wlr_renderer_begin(r: *mut wlr_renderer, width: libc::c_int,
                          height: libc::c_int);
    #[no_mangle]
    fn wlr_renderer_end(r: *mut wlr_renderer);
    #[no_mangle]
    fn wlr_renderer_clear(r: *mut wlr_renderer, color: *const libc::c_float);
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn headless_backend_from_backend(wlr_backend: *mut wlr_backend)
     -> *mut wlr_headless_backend;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
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
pub type wl_event_loop_timer_func_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_event_present {
    pub output: *mut wlr_output,
    pub commit_seq: uint32_t,
    pub when: *mut timespec,
    pub seq: libc::c_uint,
    pub refresh: libc::c_int,
    pub flags: uint32_t,
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
pub struct wlr_headless_output {
    pub wlr_output: wlr_output,
    pub backend: *mut wlr_headless_backend,
    pub link: wl_list,
    pub egl_surface: *mut libc::c_void,
    pub frame_timer: *mut wl_event_source,
    pub frame_delay: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_headless_backend {
    pub backend: wlr_backend,
    pub egl: wlr_egl,
    pub renderer: *mut wlr_renderer,
    pub display: *mut wl_display,
    pub outputs: wl_list,
    pub last_output_num: size_t,
    pub input_devices: wl_list,
    pub display_destroy: wl_listener,
    pub started: bool,
}
unsafe extern "C" fn headless_output_from_output(mut wlr_output:
                                                     *mut wlr_output)
 -> *mut wlr_headless_output {
    if wlr_output_is_headless(wlr_output) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_output_is_headless(wlr_output)\x00" as *const u8
                          as *const libc::c_char,
                      b"../backend/headless/output.c\x00" as *const u8 as
                          *const libc::c_char, 11i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 77],
                                                &[libc::c_char; 77]>(b"struct wlr_headless_output *headless_output_from_output(struct wlr_output *)\x00")).as_ptr());
    };
    return wlr_output as *mut wlr_headless_output;
}
unsafe extern "C" fn egl_create_surface(mut egl: *mut wlr_egl,
                                        mut width: libc::c_uint,
                                        mut height: libc::c_uint)
 -> EGLSurface {
    let mut attribs: [EGLint; 5] =
        [0x3057i32, width as EGLint, 0x3056i32, height as EGLint, 0x3038i32];
    let mut surf: EGLSurface =
        eglCreatePbufferSurface((*egl).display, (*egl).config,
                                attribs.as_mut_ptr());
    if surf.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create EGL surface\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/headless/output.c\x00" as *const u8 as
                     *const libc::c_char, 21i32);
        return 0 as EGLSurface
    }
    return surf;
}
unsafe extern "C" fn output_set_custom_mode(mut wlr_output: *mut wlr_output,
                                            mut width: int32_t,
                                            mut height: int32_t,
                                            mut refresh: int32_t) -> bool {
    let mut output: *mut wlr_headless_output =
        headless_output_from_output(wlr_output);
    let mut backend: *mut wlr_headless_backend = (*output).backend;
    if refresh <= 0i32 { refresh = 60i32 * 1000i32 }
    wlr_egl_destroy_surface(&mut (*backend).egl, (*output).egl_surface);
    (*output).egl_surface =
        egl_create_surface(&mut (*backend).egl, width as libc::c_uint,
                           height as libc::c_uint);
    if (*output).egl_surface.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to recreate EGL surface\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/headless/output.c\x00" as *const u8 as
                     *const libc::c_char, 41i32);
        wlr_output_destroy(wlr_output);
        return 0i32 != 0
    }
    (*output).frame_delay = 1000000i32 / refresh;
    wlr_output_update_custom_mode(&mut (*output).wlr_output, width, height,
                                  refresh);
    return 1i32 != 0;
}
unsafe extern "C" fn output_attach_render(mut wlr_output: *mut wlr_output,
                                          mut buffer_age: *mut libc::c_int)
 -> bool {
    let mut output: *mut wlr_headless_output =
        headless_output_from_output(wlr_output);
    return wlr_egl_make_current(&mut (*(*output).backend).egl,
                                (*output).egl_surface, buffer_age);
}
unsafe extern "C" fn output_commit(mut wlr_output: *mut wlr_output) -> bool {
    // Nothing needs to be done for pbuffers
    wlr_output_send_present(wlr_output, 0 as *mut wlr_output_event_present);
    return 1i32 != 0;
}
unsafe extern "C" fn output_destroy(mut wlr_output: *mut wlr_output) {
    let mut output: *mut wlr_headless_output =
        headless_output_from_output(wlr_output);
    wl_list_remove(&mut (*output).link);
    wl_event_source_remove((*output).frame_timer);
    wlr_egl_destroy_surface(&mut (*(*output).backend).egl,
                            (*output).egl_surface);
    free(output as *mut libc::c_void);
}
static mut output_impl: wlr_output_impl =
    unsafe {
        {
            let mut init =
                wlr_output_impl{enable: None,
                                set_mode: None,
                                set_custom_mode:
                                    Some(output_set_custom_mode as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _: int32_t,
                                                                  _: int32_t,
                                                                  _: int32_t)
                                                 -> bool),
                                set_cursor: None,
                                move_cursor: None,
                                destroy:
                                    Some(output_destroy as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output)
                                                 -> ()),
                                attach_render:
                                    Some(output_attach_render as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _:
                                                                      *mut libc::c_int)
                                                 -> bool),
                                commit:
                                    Some(output_commit as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output)
                                                 -> bool),
                                set_gamma: None,
                                get_gamma_size: None,
                                export_dmabuf: None,
                                schedule_frame: None,
                                attach_buffer: None,};
            init
        }
    };
/* *
 * Creates a new input device. The caller is responsible for manually raising
 * any event signals on the new input device if it wants to simulate input
 * events.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_output_is_headless(mut wlr_output:
                                                    *mut wlr_output) -> bool {
    return (*wlr_output).impl_0 == &output_impl as *const wlr_output_impl;
}
unsafe extern "C" fn signal_frame(mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut output: *mut wlr_headless_output =
        data as *mut wlr_headless_output;
    wlr_output_send_frame(&mut (*output).wlr_output);
    wl_event_source_timer_update((*output).frame_timer,
                                 (*output).frame_delay);
    return 0i32;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * Creates a headless backend. A headless backend has no outputs or inputs by
 * default.
 */
/* *
 * Create a new headless output backed by an in-memory EGL framebuffer. You can
 * read pixels from this framebuffer via wlr_renderer_read_pixels but it is
 * otherwise not displayed.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_headless_add_output(mut wlr_backend:
                                                     *mut wlr_backend,
                                                 mut width: libc::c_uint,
                                                 mut height: libc::c_uint)
 -> *mut wlr_output {
    let mut ev: *mut wl_event_loop = 0 as *mut wl_event_loop;
    let mut backend: *mut wlr_headless_backend =
        headless_backend_from_backend(wlr_backend);
    let mut output: *mut wlr_headless_output =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_headless_output>() as libc::c_ulong)
            as *mut wlr_headless_output;
    if output.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate wlr_headless_output\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/headless/output.c\x00" as *const u8 as
                     *const libc::c_char, 104i32);
        return 0 as *mut wlr_output
    }
    (*output).backend = backend;
    wlr_output_init(&mut (*output).wlr_output, &mut (*backend).backend,
                    &output_impl, (*backend).display);
    let mut wlr_output: *mut wlr_output = &mut (*output).wlr_output;
    (*output).egl_surface =
        egl_create_surface(&mut (*backend).egl, width, height);
    if (*output).egl_surface.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create EGL surface\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/headless/output.c\x00" as *const u8 as
                     *const libc::c_char, 114i32);
    } else {
        output_set_custom_mode(wlr_output, width as int32_t,
                               height as int32_t, 0i32);
        strncpy((*wlr_output).make.as_mut_ptr(),
                b"headless\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong);
        strncpy((*wlr_output).model.as_mut_ptr(),
                b"headless\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong);
        (*backend).last_output_num =
            (*backend).last_output_num.wrapping_add(1);
        snprintf((*wlr_output).name.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
                 b"HEADLESS-%zd\x00" as *const u8 as *const libc::c_char,
                 (*backend).last_output_num);
        if wlr_egl_make_current(&mut (*(*output).backend).egl,
                                (*output).egl_surface, 0 as *mut libc::c_int)
           {
            wlr_renderer_begin((*backend).renderer, (*wlr_output).width,
                               (*wlr_output).height);
            wlr_renderer_clear((*backend).renderer,
                               [1.0f64 as libc::c_float,
                                1.0f64 as libc::c_float,
                                1.0f64 as libc::c_float,
                                1.0f64 as libc::c_float].as_mut_ptr() as
                                   *const libc::c_float);
            wlr_renderer_end((*backend).renderer);
            ev = wl_display_get_event_loop((*backend).display);
            (*output).frame_timer =
                wl_event_loop_add_timer(ev,
                                        Some(signal_frame as
                                                 unsafe extern "C" fn(_:
                                                                          *mut libc::c_void)
                                                     -> libc::c_int),
                                        output as *mut libc::c_void);
            wl_list_insert(&mut (*backend).outputs, &mut (*output).link);
            if (*backend).started {
                wl_event_source_timer_update((*output).frame_timer,
                                             (*output).frame_delay);
                wlr_output_update_enabled(wlr_output, 1i32 != 0);
                wlr_signal_emit_safe(&mut (*backend).backend.events.new_output,
                                     wlr_output as *mut libc::c_void);
            }
            return wlr_output
        }
    }
    wlr_output_destroy(&mut (*output).wlr_output);
    return 0 as *mut wlr_output;
}
