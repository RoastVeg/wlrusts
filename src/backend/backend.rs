use libc;
extern "C" {
    pub type udev;
    pub type udev_monitor;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type session_impl;
    pub type wlr_renderer;
    pub type wlr_surface;
    pub type wlr_texture;
    pub type wlr_output_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strtok_r(__s: *mut libc::c_char, __delim: *const libc::c_char,
                __save_ptr: *mut *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    /*
	 * Signal for when the session becomes active/inactive.
	 * It's called when we swap virtual terminal.
	 */
    /*
	 * 0 if virtual terminals are not supported
	 * i.e. seat != "seat0"
	 */
    /*
 * Opens a session, taking control of the current virtual terminal.
 * This should not be called if another program is already in control
 * of the terminal (Xorg, another Wayland compositor, etc.).
 *
 * If logind support is not enabled, you must have CAP_SYS_ADMIN or be root.
 * It is safe to drop privileges after this is called.
 *
 * Returns NULL on error.
 */
    #[no_mangle]
    fn wlr_session_create(disp: *mut wl_display) -> *mut wlr_session;
    /*
 * Closes a previously opened session and restores the virtual terminal.
 * You should call wlr_session_close_file on each files you opened
 * with wlr_session_open_file before you call this.
 */
    /*
 * Opens the file at path.
 * This can only be used to open DRM or evdev (input) devices.
 *
 * When the session becomes inactive:
 * - DRM files lose their DRM master status
 * - evdev files become invalid and should be closed
 *
 * Returns -errno on error.
 */
    /*
 * Closes a file previously opened with wlr_session_open_file.
 */
    /*
 * Changes the virtual terminal.
 */
    #[no_mangle]
    fn wlr_session_find_gpus(session: *mut wlr_session, ret_len: size_t,
                             ret: *mut libc::c_int) -> size_t;
    #[no_mangle]
    fn wlr_session_destroy(session: *mut wlr_session);
    #[no_mangle]
    fn wlr_drm_backend_create(display: *mut wl_display,
                              session: *mut wlr_session, gpu_fd: libc::c_int,
                              parent: *mut wlr_backend,
                              create_renderer_func:
                                  wlr_renderer_create_func_t)
     -> *mut wlr_backend;
    #[no_mangle]
    fn wlr_headless_backend_create(display: *mut wl_display,
                                   create_renderer_func:
                                       wlr_renderer_create_func_t)
     -> *mut wlr_backend;
    #[no_mangle]
    fn wlr_headless_add_output(backend: *mut wlr_backend, width: libc::c_uint,
                               height: libc::c_uint) -> *mut wlr_output;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    #[no_mangle]
    fn wlr_libinput_backend_create(display: *mut wl_display,
                                   session: *mut wlr_session)
     -> *mut wlr_backend;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* *
 * Creates a multi-backend. Multi-backends wrap an arbitrary number of backends
 * and aggregate their new_output/new_input signals.
 */
    #[no_mangle]
    fn wlr_multi_backend_create(display: *mut wl_display) -> *mut wlr_backend;
    /* *
 * Adds the given backend to the multi backend. This should be done before the
 * new backend is started.
 */
    #[no_mangle]
    fn wlr_multi_backend_add(multi: *mut wlr_backend,
                             backend: *mut wlr_backend) -> bool;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* *
 * Creates a noop backend. Noop backends do not have a framebuffer and are not
 * capable of rendering anything. They are useful for when there's no real
 * outputs connected; you can stash your views on a noop output until an output
 * is connected.
 */
    #[no_mangle]
    fn wlr_noop_backend_create(display: *mut wl_display) -> *mut wlr_backend;
    /* *
 * Create a new noop output.
 */
    #[no_mangle]
    fn wlr_noop_add_output(backend: *mut wlr_backend) -> *mut wlr_output;
    #[no_mangle]
    fn wlr_wl_backend_create(display: *mut wl_display,
                             remote: *const libc::c_char,
                             create_renderer_func: wlr_renderer_create_func_t)
     -> *mut wlr_backend;
    #[no_mangle]
    fn wlr_wl_output_create(backend: *mut wlr_backend) -> *mut wlr_output;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    /* *
 * Creates a new wlr_x11_backend. This backend will be created with no outputs;
 * you must use wlr_x11_output_create to add them.
 *
 * The `x11_display` argument is the name of the X Display socket. Set
 * to NULL for the default behaviour of XOpenDisplay.
 */
    #[no_mangle]
    fn wlr_x11_backend_create(display: *mut wl_display,
                              x11_display: *const libc::c_char,
                              create_renderer_func:
                                  wlr_renderer_create_func_t)
     -> *mut wlr_backend;
    /* *
 * Adds a new output to this backend. You may remove outputs by destroying them.
 * Note that if called before initializing the backend, this will return NULL
 * and your outputs will be created during initialization (and given to you via
 * the output_add signal).
 */
    #[no_mangle]
    fn wlr_x11_output_create(backend: *mut wlr_backend) -> *mut wlr_output;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* *
 * Creates an RDP backend. An RDP backend will create one output, keyboard, and
 * pointer for each client that connects.
 */
    #[no_mangle]
    fn wlr_rdp_backend_create(display: *mut wl_display,
                              create_renderer_func:
                                  wlr_renderer_create_func_t,
                              tls_cert_path: *const libc::c_char,
                              tls_key_path: *const libc::c_char)
     -> *mut wlr_backend;
    #[no_mangle]
    fn wlr_rdp_backend_set_address(wlr_backend: *mut wlr_backend,
                                   address: *const libc::c_char);
    #[no_mangle]
    fn wlr_rdp_backend_set_port(wlr_backend: *mut wlr_backend,
                                port: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __clockid_t = libc::c_int;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type clockid_t = __clockid_t;

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
    pub events: C2RustUnnamed,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
}
pub type khronos_int32_t = int32_t;
pub type EGLint = khronos_int32_t;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
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
    pub exts: C2RustUnnamed_0,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_backend_impl {
    pub start: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> ()>,
    pub get_renderer: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                 -> *mut crate::src::backend::drm::atomic::wlr_renderer>,
    pub get_session: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                -> *mut wlr_session>,
    pub get_presentation_clock: Option<unsafe extern "C" fn(_:
                                                                *mut wlr_backend)
                                           -> clockid_t>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed_1,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
pub type wlr_renderer_create_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wlr_egl, _: EGLenum,
                                _: *mut libc::c_void, _: *mut EGLint,
                                _: EGLint) -> *mut crate::src::backend::drm::atomic::wlr_renderer>;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_multi_backend {
    pub backend: wlr_backend,
    pub session: *mut wlr_session,
    pub backends: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_2,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub backend_add: wl_signal,
    pub backend_remove: wl_signal,
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
// mHz

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
    pub events: C2RustUnnamed_3,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
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
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
pub type wl_output_subpixel = libc::c_uint;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: wl_output_subpixel = 5;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: wl_output_subpixel = 4;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: wl_output_subpixel = 3;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: wl_output_subpixel = 2;
pub const WL_OUTPUT_SUBPIXEL_NONE: wl_output_subpixel = 1;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: wl_output_subpixel = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
/* *
 * Initializes common state on a wlr_backend and sets the implementation to the
 * provided wlr_backend_impl reference.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_init(mut backend: *mut wlr_backend,
                                          mut impl_0:
                                              *const wlr_backend_impl) {
    if !backend.is_null() {
    } else {
        __assert_fail(b"backend\x00" as *const u8 as *const libc::c_char,
                      b"../backend/backend.c\x00" as *const u8 as
                          *const libc::c_char, 30i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 77],
                                                &[libc::c_char; 77]>(b"void wlr_backend_init(struct wlr_backend *, const struct wlr_backend_impl *)\x00")).as_ptr());
    };
    (*backend).impl_0 = impl_0;
    wl_signal_init(&mut (*backend).events.destroy);
    wl_signal_init(&mut (*backend).events.new_input);
    wl_signal_init(&mut (*backend).events.new_output);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_start(mut backend: *mut wlr_backend)
 -> bool {
    if (*(*backend).impl_0).start.is_some() {
        return (*(*backend).impl_0).start.expect("non-null function pointer")(backend)
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_destroy(mut backend: *mut wlr_backend) {
    if backend.is_null() { return }
    if !(*backend).impl_0.is_null() && (*(*backend).impl_0).destroy.is_some()
       {
        (*(*backend).impl_0).destroy.expect("non-null function pointer")(backend);
    } else { free(backend as *mut libc::c_void); };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_get_renderer(mut backend:
                                                      *mut wlr_backend)
 -> *mut crate::src::backend::drm::atomic::wlr_renderer {
    if (*(*backend).impl_0).get_renderer.is_some() {
        return (*(*backend).impl_0).get_renderer.expect("non-null function pointer")(backend)
    }
    return 0 as *mut crate::src::backend::drm::atomic::wlr_renderer;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_get_session(mut backend:
                                                     *mut wlr_backend)
 -> *mut wlr_session {
    if (*(*backend).impl_0).get_session.is_some() {
        return (*(*backend).impl_0).get_session.expect("non-null function pointer")(backend)
    }
    return 0 as *mut wlr_session;
}
/* *
 * Start the backend. This may signal new_input or new_output immediately, but
 * may also wait until the display's event loop begins. Returns false on
 * failure.
 */
/* *
 * Destroy the backend and clean up all of its resources. Normally called
 * automatically when the wl_display is destroyed.
 */
/* *
 * Obtains the wlr_renderer reference this backend is using.
 */
/* *
 * Obtains the wlr_session reference from this backend if there is any.
 * Might return NULL for backends that don't use a session.
 */
/* *
 * Returns the clock used by the backend for presentation feedback.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_get_presentation_clock(mut backend:
                                                                *mut wlr_backend)
 -> clockid_t {
    if (*(*backend).impl_0).get_presentation_clock.is_some() {
        return (*(*backend).impl_0).get_presentation_clock.expect("non-null function pointer")(backend)
    }
    return 1i32;
}
unsafe extern "C" fn parse_outputs_env(mut name: *const libc::c_char)
 -> size_t {
    let mut outputs_str: *const libc::c_char = getenv(name);
    if outputs_str.is_null() { return 1i32 as size_t }
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outputs: libc::c_int =
        strtol(outputs_str, &mut end, 10i32) as libc::c_int;
    if *end as libc::c_int != 0 || outputs < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] %s specified with invalid integer, ignoring\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/backend.c\x00" as *const u8 as
                     *const libc::c_char, 86i32, name);
        return 1i32 as size_t
    }
    return outputs as size_t;
}
unsafe extern "C" fn attempt_wl_backend(mut display: *mut wl_display,
                                        mut create_renderer_func:
                                            wlr_renderer_create_func_t)
 -> *mut wlr_backend {
    let mut backend: *mut wlr_backend =
        wlr_wl_backend_create(display, 0 as *const libc::c_char,
                              create_renderer_func);
    if backend.is_null() { return 0 as *mut wlr_backend }
    let mut outputs: size_t =
        parse_outputs_env(b"WLR_WL_OUTPUTS\x00" as *const u8 as
                              *const libc::c_char);
    let mut i: size_t = 0i32 as size_t;
    while i < outputs { wlr_wl_output_create(backend); i = i.wrapping_add(1) }
    return backend;
}
unsafe extern "C" fn attempt_x11_backend(mut display: *mut wl_display,
                                         mut x11_display: *const libc::c_char,
                                         mut create_renderer_func:
                                             wlr_renderer_create_func_t)
 -> *mut wlr_backend {
    let mut backend: *mut wlr_backend =
        wlr_x11_backend_create(display, x11_display, create_renderer_func);
    if backend.is_null() { return 0 as *mut wlr_backend }
    let mut outputs: size_t =
        parse_outputs_env(b"WLR_X11_OUTPUTS\x00" as *const u8 as
                              *const libc::c_char);
    let mut i: size_t = 0i32 as size_t;
    while i < outputs {
        wlr_x11_output_create(backend);
        i = i.wrapping_add(1)
    }
    return backend;
}
unsafe extern "C" fn attempt_headless_backend(mut display: *mut wl_display,
                                              mut create_renderer_func:
                                                  wlr_renderer_create_func_t)
 -> *mut wlr_backend {
    let mut backend: *mut wlr_backend =
        wlr_headless_backend_create(display, create_renderer_func);
    if backend.is_null() { return 0 as *mut wlr_backend }
    let mut outputs: size_t =
        parse_outputs_env(b"WLR_HEADLESS_OUTPUTS\x00" as *const u8 as
                              *const libc::c_char);
    let mut i: size_t = 0i32 as size_t;
    while i < outputs {
        wlr_headless_add_output(backend, 1280i32 as libc::c_uint,
                                720i32 as libc::c_uint);
        i = i.wrapping_add(1)
    }
    return backend;
}
unsafe extern "C" fn attempt_rdp_backend(mut display: *mut wl_display,
                                         mut create_renderer_func:
                                             wlr_renderer_create_func_t)
 -> *mut wlr_backend {
    let mut cert_path: *const libc::c_char =
        getenv(b"WLR_RDP_TLS_CERT_PATH\x00" as *const u8 as
                   *const libc::c_char);
    let mut key_path: *const libc::c_char =
        getenv(b"WLR_RDP_TLS_KEY_PATH\x00" as *const u8 as
                   *const libc::c_char);
    if cert_path.is_null() || key_path.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] The RDP backend requires WLR_RDP_TLS_CERT_PATH and WLR_RDP_TLS_KEY_PATH to be set.\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/backend.c\x00" as *const u8 as
                     *const libc::c_char, 147i32);
        return 0 as *mut wlr_backend
    }
    let mut backend: *mut wlr_backend =
        wlr_rdp_backend_create(display, create_renderer_func, cert_path,
                               key_path);
    let mut address: *const libc::c_char =
        getenv(b"WLR_RDP_ADDRESS\x00" as *const u8 as *const libc::c_char);
    if !address.is_null() { wlr_rdp_backend_set_address(backend, address); }
    let mut _port: *const libc::c_char =
        getenv(b"WLR_RDP_PORT\x00" as *const u8 as *const libc::c_char);
    if !_port.is_null() {
        let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut port: libc::c_int =
            strtol(_port, &mut endptr, 10i32) as libc::c_int;
        if *endptr as libc::c_int != 0 || port <= 0i32 || port >= 1024i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Expected WLR_RDP_PORT to be a positive integer less than 1024\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/backend.c\x00" as *const u8 as
                         *const libc::c_char, 162i32);
            wlr_backend_destroy(backend);
            return 0 as *mut wlr_backend
        }
        wlr_rdp_backend_set_port(backend, port);
    }
    return backend;
}
unsafe extern "C" fn attempt_noop_backend(mut display: *mut wl_display)
 -> *mut wlr_backend {
    let mut backend: *mut wlr_backend = wlr_noop_backend_create(display);
    if backend.is_null() { return 0 as *mut wlr_backend }
    let mut outputs: size_t =
        parse_outputs_env(b"WLR_NOOP_OUTPUTS\x00" as *const u8 as
                              *const libc::c_char);
    let mut i: size_t = 0i32 as size_t;
    while i < outputs { wlr_noop_add_output(backend); i = i.wrapping_add(1) }
    return backend;
}
unsafe extern "C" fn attempt_drm_backend(mut display: *mut wl_display,
                                         mut backend: *mut wlr_backend,
                                         mut session: *mut wlr_session,
                                         mut create_renderer_func:
                                             wlr_renderer_create_func_t)
 -> *mut wlr_backend {
    let mut gpus: [libc::c_int; 8] = [0; 8];
    let mut num_gpus: size_t =
        wlr_session_find_gpus(session, 8i32 as size_t, gpus.as_mut_ptr());
    let mut primary_drm: *mut wlr_backend = 0 as *mut wlr_backend;
    _wlr_log(WLR_INFO,
             b"[%s:%d] Found %zu GPUs\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/backend.c\x00" as *const u8 as *const libc::c_char,
             192i32, num_gpus);
    let mut i: size_t = 0i32 as size_t;
    while i < num_gpus {
        let mut drm: *mut wlr_backend =
            wlr_drm_backend_create(display, session, gpus[i as usize],
                                   primary_drm, create_renderer_func);
        if drm.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to open DRM device %d\x00" as *const u8
                         as *const libc::c_char,
                     b"../backend/backend.c\x00" as *const u8 as
                         *const libc::c_char, 198i32, gpus[i as usize]);
        } else {
            if primary_drm.is_null() { primary_drm = drm }
            wlr_multi_backend_add(backend, drm);
        }
        i = i.wrapping_add(1)
    }
    return primary_drm;
}
unsafe extern "C" fn attempt_backend_by_name(mut display: *mut wl_display,
                                             mut backend: *mut wlr_backend,
                                             mut session:
                                                 *mut *mut wlr_session,
                                             mut name: *const libc::c_char,
                                             mut create_renderer_func:
                                                 wlr_renderer_create_func_t)
 -> *mut wlr_backend {
    if strcmp(name, b"wayland\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        return attempt_wl_backend(display, create_renderer_func)
    } else {
        if strcmp(name, b"x11\x00" as *const u8 as *const libc::c_char) ==
               0i32 {
            return attempt_x11_backend(display, 0 as *const libc::c_char,
                                       create_renderer_func)
        } else {
            if strcmp(name,
                      b"headless\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
                return attempt_headless_backend(display, create_renderer_func)
            } else {
                if strcmp(name,
                          b"rdp\x00" as *const u8 as *const libc::c_char) ==
                       0i32 {
                    return attempt_rdp_backend(display, create_renderer_func)
                } else {
                    if strcmp(name,
                              b"noop\x00" as *const u8 as *const libc::c_char)
                           == 0i32 {
                        return attempt_noop_backend(display)
                    } else {
                        if strcmp(name,
                                  b"drm\x00" as *const u8 as
                                      *const libc::c_char) == 0i32 ||
                               strcmp(name,
                                      b"libinput\x00" as *const u8 as
                                          *const libc::c_char) == 0i32 {
                            // DRM and libinput need a session
                            if (*session).is_null() {
                                *session = wlr_session_create(display);
                                if (*session).is_null() {
                                    _wlr_log(WLR_ERROR,
                                             b"[%s:%d] failed to start a session\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"../backend/backend.c\x00" as
                                                 *const u8 as
                                                 *const libc::c_char, 234i32);
                                    return 0 as *mut wlr_backend
                                }
                            }
                            if strcmp(name,
                                      b"libinput\x00" as *const u8 as
                                          *const libc::c_char) == 0i32 {
                                return wlr_libinput_backend_create(display,
                                                                   *session)
                            } else {
                                return attempt_drm_backend(display, backend,
                                                           *session,
                                                           create_renderer_func)
                            }
                        }
                    }
                }
            }
        }
    }
    _wlr_log(WLR_ERROR,
             b"[%s:%d] unrecognized backend \'%s\'\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/backend.c\x00" as *const u8 as *const libc::c_char,
             246i32, name);
    return 0 as *mut wlr_backend;
}
/* *
 * Automatically initializes the most suitable backend given the environment.
 * Will always return a multibackend. The backend is created but not started.
 * Returns NULL on failure.
 *
 * The compositor can request to initialize the backend's renderer by setting
 * the create_render_func. The callback must initialize the given wlr_egl and
 * return a valid wlr_renderer, or NULL if it has failed to initiaze it.
 * Pass NULL as create_renderer_func to use the backend's default renderer.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_autocreate(mut display: *mut wl_display,
                                                mut create_renderer_func:
                                                    wlr_renderer_create_func_t)
 -> *mut wlr_backend {
    let mut x11_display: *const libc::c_char = 0 as *const libc::c_char;
    let mut libinput: *mut wlr_backend = 0 as *mut wlr_backend;
    let mut primary_drm: *mut wlr_backend = 0 as *mut wlr_backend;
    let mut backend: *mut wlr_backend = wlr_multi_backend_create(display);
    let mut multi: *mut wlr_multi_backend = backend as *mut wlr_multi_backend;
    if backend.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] could not allocate multibackend\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/backend.c\x00" as *const u8 as
                     *const libc::c_char, 255i32);
        return 0 as *mut wlr_backend
    }
    let mut names: *mut libc::c_char =
        getenv(b"WLR_BACKENDS\x00" as *const u8 as *const libc::c_char);
    if !names.is_null() {
        names = strdup(names);
        if names.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] allocation failed\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/backend.c\x00" as *const u8 as
                         *const libc::c_char, 263i32);
            wlr_backend_destroy(backend);
            return 0 as *mut wlr_backend
        }
        let mut saveptr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name: *mut libc::c_char =
            strtok_r(names, b",\x00" as *const u8 as *const libc::c_char,
                     &mut saveptr);
        while !name.is_null() {
            let mut subbackend: *mut wlr_backend =
                attempt_backend_by_name(display, backend,
                                        &mut (*multi).session, name,
                                        create_renderer_func);
            if subbackend.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] failed to start backend \'%s\'\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/backend.c\x00" as *const u8 as
                             *const libc::c_char, 274i32, name);
                wlr_session_destroy((*multi).session);
                wlr_backend_destroy(backend);
                free(names as *mut libc::c_void);
                return 0 as *mut wlr_backend
            }
            if !wlr_multi_backend_add(backend, subbackend) {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] failed to add backend \'%s\'\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/backend.c\x00" as *const u8 as
                             *const libc::c_char, 282i32, name);
                wlr_session_destroy((*multi).session);
                wlr_backend_destroy(backend);
                free(names as *mut libc::c_void);
                return 0 as *mut wlr_backend
            }
            name =
                strtok_r(0 as *mut libc::c_char,
                         b",\x00" as *const u8 as *const libc::c_char,
                         &mut saveptr)
        }
        free(names as *mut libc::c_void);
        return backend
    }
    if !getenv(b"WAYLAND_DISPLAY\x00" as *const u8 as
                   *const libc::c_char).is_null() ||
           !getenv(b"_WAYLAND_DISPLAY\x00" as *const u8 as
                       *const libc::c_char).is_null() ||
           !getenv(b"WAYLAND_SOCKET\x00" as *const u8 as
                       *const libc::c_char).is_null() {
        let mut wl_backend: *mut wlr_backend =
            attempt_wl_backend(display, create_renderer_func);
        if !wl_backend.is_null() {
            wlr_multi_backend_add(backend, wl_backend);
            return backend
        }
    } else {
        x11_display =
            getenv(b"DISPLAY\x00" as *const u8 as *const libc::c_char);
        if !x11_display.is_null() {
            let mut x11_backend: *mut wlr_backend =
                attempt_x11_backend(display, x11_display,
                                    create_renderer_func);
            if !x11_backend.is_null() {
                wlr_multi_backend_add(backend, x11_backend);
                return backend
            }
        } else {
            // Attempt DRM+libinput
            (*multi).session = wlr_session_create(display);
            if (*multi).session.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to start a DRM session\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/backend.c\x00" as *const u8 as
                             *const libc::c_char, 325i32);
                wlr_backend_destroy(backend);
                return 0 as *mut wlr_backend
            }
            libinput = wlr_libinput_backend_create(display, (*multi).session);
            if libinput.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to start libinput backend\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/backend.c\x00" as *const u8 as
                             *const libc::c_char, 333i32);
                wlr_session_destroy((*multi).session);
                wlr_backend_destroy(backend);
                return 0 as *mut wlr_backend
            }
            wlr_multi_backend_add(backend, libinput);
            primary_drm =
                attempt_drm_backend(display, backend, (*multi).session,
                                    create_renderer_func);
            if primary_drm.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to open any DRM device\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/backend.c\x00" as *const u8 as
                             *const libc::c_char, 343i32);
                wlr_backend_destroy(libinput);
                wlr_session_destroy((*multi).session);
                wlr_backend_destroy(backend);
                return 0 as *mut wlr_backend
            }
            return backend
        }
    }
    wlr_backend_destroy(backend);
    return 0 as *mut wlr_backend;
}
