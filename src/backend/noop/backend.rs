use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type udev;
    pub type udev_monitor;
    pub type session_impl;
    pub type wlr_renderer;
    pub type wlr_texture;
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
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wlr_output_update_enabled(output: *mut wlr_output, enabled: bool);
    #[no_mangle]
    fn wlr_output_destroy(output: *mut wlr_output);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    /* *
 * Initializes common state on a wlr_backend and sets the implementation to the
 * provided wlr_backend_impl reference.
 */
    #[no_mangle]
    fn wlr_backend_init(backend: *mut wlr_backend,
                        impl_0: *const wlr_backend_impl);
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
    pub events: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
    pub events: C2RustUnnamed_1,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
    pub events: C2RustUnnamed_2,
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
pub struct C2RustUnnamed_2 {
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
pub struct wlr_noop_backend {
    pub backend: wlr_backend,
    pub display: *mut wl_display,
    pub outputs: wl_list,
    pub last_output_num: size_t,
    pub started: bool,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_noop_output {
    pub wlr_output: wlr_output,
    pub backend: *mut wlr_noop_backend,
    pub link: wl_list,
}
#[no_mangle]
pub unsafe extern "C" fn noop_backend_from_backend(mut wlr_backend:
                                                       *mut wlr_backend)
 -> *mut wlr_noop_backend {
    if wlr_backend_is_noop(wlr_backend) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_backend_is_noop(wlr_backend)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/noop/backend.c\x00" as *const u8 as
                          *const libc::c_char, 10i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"struct wlr_noop_backend *noop_backend_from_backend(struct wlr_backend *)\x00")).as_ptr());
    };
    return wlr_backend as *mut wlr_noop_backend;
}
unsafe extern "C" fn backend_start(mut wlr_backend: *mut wlr_backend)
 -> bool {
    let mut backend: *mut wlr_noop_backend =
        noop_backend_from_backend(wlr_backend);
    _wlr_log(WLR_INFO,
             b"[%s:%d] Starting noop backend\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/noop/backend.c\x00" as *const u8 as
                 *const libc::c_char, 16i32);
    let mut output: *mut wlr_noop_output = 0 as *mut wlr_noop_output;
    output =
        ((*backend).outputs.next as *mut libc::c_char).offset(-600) as
            *mut wlr_noop_output;
    while &mut (*output).link as *mut wl_list !=
              &mut (*backend).outputs as *mut wl_list {
        wlr_output_update_enabled(&mut (*output).wlr_output, 1i32 != 0);
        wlr_signal_emit_safe(&mut (*backend).backend.events.new_output,
                             &mut (*output).wlr_output as *mut wlr_output as
                                 *mut libc::c_void);
        output =
            ((*output).link.next as *mut libc::c_char).offset(-600) as
                *mut wlr_noop_output
    }
    (*backend).started = 1i32 != 0;
    return 1i32 != 0;
}
unsafe extern "C" fn backend_destroy(mut wlr_backend: *mut wlr_backend) {
    let mut backend: *mut wlr_noop_backend =
        noop_backend_from_backend(wlr_backend);
    if wlr_backend.is_null() { return }
    let mut output: *mut wlr_noop_output = 0 as *mut wlr_noop_output;
    let mut output_tmp: *mut wlr_noop_output = 0 as *mut wlr_noop_output;
    output =
        ((*backend).outputs.next as *mut libc::c_char).offset(-600) as
            *mut wlr_noop_output;
    output_tmp =
        ((*output).link.next as *mut libc::c_char).offset(-600) as
            *mut wlr_noop_output;
    while &mut (*output).link as *mut wl_list !=
              &mut (*backend).outputs as *mut wl_list {
        wlr_output_destroy(&mut (*output).wlr_output);
        output = output_tmp;
        output_tmp =
            ((*output).link.next as *mut libc::c_char).offset(-600) as
                *mut wlr_noop_output
    }
    wlr_signal_emit_safe(&mut (*wlr_backend).events.destroy,
                         backend as *mut libc::c_void);
    free(backend as *mut libc::c_void);
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
                                 get_renderer: None,
                                 get_session: None,
                                 get_presentation_clock: None,};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn wlr_noop_backend_create(mut display: *mut wl_display)
 -> *mut wlr_backend {
    _wlr_log(WLR_INFO,
             b"[%s:%d] Creating noop backend\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/noop/backend.c\x00" as *const u8 as
                 *const libc::c_char, 51i32);
    let mut backend: *mut wlr_noop_backend =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_noop_backend>() as libc::c_ulong) as
            *mut wlr_noop_backend;
    if backend.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate wlr_noop_backend\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/noop/backend.c\x00" as *const u8 as
                     *const libc::c_char, 56i32);
        return 0 as *mut wlr_backend
    }
    wlr_backend_init(&mut (*backend).backend, &backend_impl);
    (*backend).display = display;
    wl_list_init(&mut (*backend).outputs);
    return &mut (*backend).backend;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_is_noop(mut backend: *mut wlr_backend)
 -> bool {
    return (*backend).impl_0 == &backend_impl as *const wlr_backend_impl;
}
