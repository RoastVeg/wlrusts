use libc;
extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type xcb_connection_t;
    pub type xcb_errors_context_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sigprocmask(__how: libc::c_int, __set: *const sigset_t,
                   __oset: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigwait(__set: *const sigset_t, __sig: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                 _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn setenv(__name: *const libc::c_char, __value: *const libc::c_char,
              __replace: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn socketpair(__domain: libc::c_int, __type: libc::c_int,
                  __protocol: libc::c_int, __fds: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn _exit(__status: libc::c_int) -> !;
    #[no_mangle]
    fn getppid() -> __pid_t;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
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
    fn wl_display_add_destroy_listener(display: *mut wl_display,
                                       listener: *mut wl_listener);
    #[no_mangle]
    fn wl_client_create(display: *mut wl_display, fd: libc::c_int)
     -> *mut wl_client;
    #[no_mangle]
    fn wl_client_destroy(client: *mut wl_client);
    #[no_mangle]
    fn wl_client_add_destroy_listener(client: *mut wl_client,
                                      listener: *mut wl_listener);
    #[no_mangle]
    fn wl_event_loop_add_signal(loop_0: *mut wl_event_loop,
                                signal_number: libc::c_int,
                                func: wl_event_loop_signal_func_t,
                                data: *mut libc::c_void)
     -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
    fn wlr_log_get_verbosity() -> wlr_log_importance;
    #[no_mangle]
    fn set_cloexec(fd: libc::c_int, cloexec: bool) -> bool;
    #[no_mangle]
    fn unlink_display_sockets(display: libc::c_int);
    #[no_mangle]
    fn open_display_sockets(socks: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal_0: *mut wl_signal,
                            data: *mut libc::c_void);
    #[no_mangle]
    fn xwm_create(wlr_xwayland: *mut wlr_xwayland) -> *mut wlr_xwm;
    #[no_mangle]
    fn xwm_set_seat(xwm: *mut wlr_xwm, seat: *mut wlr_seat);
    #[no_mangle]
    fn xwm_set_cursor(xwm: *mut wlr_xwm, pixels: *const uint8_t,
                      stride: uint32_t, width: uint32_t, height: uint32_t,
                      hotspot_x: int32_t, hotspot_y: int32_t);
    #[no_mangle]
    fn xwm_destroy(xwm: *mut wlr_xwm);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct wl_array {
    pub size: size_t,
    pub alloc: size_t,
    pub data: *mut libc::c_void,
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
pub type wl_event_loop_signal_func_t
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_void)
               -> libc::c_int>;
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
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_0,
    /* *
 * Create a new texture from raw pixel data. `stride` is in bytes. The returned
 * texture is mutable.
 */
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
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
pub struct wlr_surface {
    pub resource: *mut wl_resource,
    pub renderer: *mut wlr_renderer,
    pub buffer: *mut wlr_buffer,
    pub sx: libc::c_int,
    pub sy: libc::c_int,
    pub buffer_damage: pixman_region32_t,
    pub opaque_region: pixman_region32_t,
    pub input_region: pixman_region32_t,
    pub current: wlr_surface_state,
    pub pending: wlr_surface_state,
    pub previous: wlr_surface_state,
    pub role: *const wlr_surface_role,
    pub role_data: *mut libc::c_void,
    pub events: C2RustUnnamed_1,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
    /* *
 * Get a subsurface from a surface. Can return NULL if the subsurface has been
 * destroyed.
 */
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub commit: wl_signal,
    pub new_subsurface: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_surface_role {
    pub name: *const libc::c_char,
    pub commit: Option<unsafe extern "C" fn(_: *mut wlr_surface) -> ()>,
    pub precommit: Option<unsafe extern "C" fn(_: *mut wlr_surface) -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_surface_state {
    pub committed: uint32_t,
    pub buffer_resource: *mut wl_resource,
    pub dx: int32_t,
    pub dy: int32_t,
    pub surface_damage: pixman_region32_t,
    pub buffer_damage: pixman_region32_t,
    pub opaque: pixman_region32_t,
    pub input: pixman_region32_t,
    pub transform: wl_output_transform,
    pub scale: int32_t,
    pub frame_callback_list: wl_list,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub buffer_width: libc::c_int,
    pub buffer_height: libc::c_int,
    pub buffer_destroy: wl_listener,
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
pub struct wlr_subcompositor {
    pub global: *mut wl_global,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub subcompositor: wlr_subcompositor,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_2,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
pub type wlr_button_state = libc::c_uint;
pub const WLR_BUTTON_PRESSED: wlr_button_state = 1;
pub const WLR_BUTTON_RELEASED: wlr_button_state = 0;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
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
    pub repeat_info: C2RustUnnamed_4,
    pub events: C2RustUnnamed_3,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub rate: int32_t,
    pub delay: int32_t,
}
pub type wlr_axis_source = libc::c_uint;
pub const WLR_AXIS_SOURCE_WHEEL_TILT: wlr_axis_source = 3;
pub const WLR_AXIS_SOURCE_CONTINUOUS: wlr_axis_source = 2;
pub const WLR_AXIS_SOURCE_FINGER: wlr_axis_source = 1;
pub const WLR_AXIS_SOURCE_WHEEL: wlr_axis_source = 0;
pub type wlr_axis_orientation = libc::c_uint;
pub const WLR_AXIS_ORIENTATION_HORIZONTAL: wlr_axis_orientation = 1;
pub const WLR_AXIS_ORIENTATION_VERTICAL: wlr_axis_orientation = 0;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_serial_range {
    pub min_incl: uint32_t,
    pub max_incl: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_serial_ringset {
    pub data: [wlr_serial_range; 128],
    pub end: libc::c_int,
    pub count: libc::c_int,
}
/* *
 * Contains state for a single client's bound wl_seat resource and can be used
 * to issue input events to that client. The lifetime of these objects is
 * managed by wlr_seat; some may be NULL.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat_client {
    pub client: *mut wl_client,
    pub seat: *mut wlr_seat,
    pub link: wl_list,
    pub resources: wl_list,
    pub pointers: wl_list,
    pub keyboards: wl_list,
    pub touches: wl_list,
    pub data_devices: wl_list,
    pub events: C2RustUnnamed_5,
    pub serials: wlr_serial_ringset,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat {
    pub global: *mut wl_global,
    pub display: *mut wl_display,
    pub clients: wl_list,
    pub name: *mut libc::c_char,
    pub capabilities: uint32_t,
    pub last_event: timespec,
    pub selection_source: *mut wlr_data_source,
    pub selection_serial: uint32_t,
    pub selection_offers: wl_list,
    pub primary_selection_source: *mut wlr_primary_selection_source,
    pub primary_selection_serial: uint32_t,
    pub drag: *mut wlr_drag,
    pub drag_source: *mut wlr_data_source,
    pub drag_serial: uint32_t,
    pub drag_offers: wl_list,
    pub pointer_state: wlr_seat_pointer_state,
    pub keyboard_state: wlr_seat_keyboard_state,
    pub touch_state: wlr_seat_touch_state,
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
    pub events: C2RustUnnamed_6,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub pointer_grab_begin: wl_signal,
    pub pointer_grab_end: wl_signal,
    pub keyboard_grab_begin: wl_signal,
    pub keyboard_grab_end: wl_signal,
    pub touch_grab_begin: wl_signal,
    pub touch_grab_end: wl_signal,
    pub request_set_cursor: wl_signal,
    pub request_set_selection: wl_signal,
    pub set_selection: wl_signal,
    pub request_set_primary_selection: wl_signal,
    pub set_primary_selection: wl_signal,
    pub request_start_drag: wl_signal,
    pub start_drag: wl_signal,
    pub destroy: wl_signal,
}
// XXX this will conflict with the actual touch cancel which is different so
	// we need to rename this
/* *
 * Passed to `wlr_seat_touch_start_grab()` to start a grab of the touch device.
 * The grabber is responsible for handling touch events for the seat.
 */
/* *
 * Passed to `wlr_seat_keyboard_start_grab()` to start a grab of the keyboard.
 * The grabber is responsible for handling keyboard events for the seat.
 */
/* *
 * Passed to `wlr_seat_pointer_start_grab()` to start a grab of the pointer. The
 * grabber is responsible for handling pointer events for the seat.
 */
// wlr_seat_pointer_focus_change_event
// TODO: May be useful to be able to simulate keyboard input events
// wlr_seat_keyboard_focus_change_event
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat_touch_state {
    pub seat: *mut wlr_seat,
    pub touch_points: wl_list,
    pub grab_serial: uint32_t,
    pub grab_id: uint32_t,
    pub grab: *mut wlr_seat_touch_grab,
    pub default_grab: *mut wlr_seat_touch_grab,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat_touch_grab {
    pub interface: *const wlr_touch_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_touch_grab_interface {
    pub down: Option<unsafe extern "C" fn(_: *mut wlr_seat_touch_grab,
                                          _: uint32_t,
                                          _: *mut wlr_touch_point)
                         -> uint32_t>,
    pub up: Option<unsafe extern "C" fn(_: *mut wlr_seat_touch_grab,
                                        _: uint32_t, _: *mut wlr_touch_point)
                       -> ()>,
    pub motion: Option<unsafe extern "C" fn(_: *mut wlr_seat_touch_grab,
                                            _: uint32_t,
                                            _: *mut wlr_touch_point) -> ()>,
    pub enter: Option<unsafe extern "C" fn(_: *mut wlr_seat_touch_grab,
                                           _: uint32_t,
                                           _: *mut wlr_touch_point) -> ()>,
    pub cancel: Option<unsafe extern "C" fn(_: *mut wlr_seat_touch_grab)
                           -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_touch_point {
    pub touch_id: int32_t,
    pub surface: *mut wlr_surface,
    pub client: *mut wlr_seat_client,
    pub focus_surface: *mut wlr_surface,
    pub focus_client: *mut wlr_seat_client,
    pub sx: libc::c_double,
    pub sy: libc::c_double,
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
    pub events: C2RustUnnamed_7,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat_keyboard_state {
    pub seat: *mut wlr_seat,
    pub keyboard: *mut wlr_keyboard,
    pub focused_client: *mut wlr_seat_client,
    pub focused_surface: *mut wlr_surface,
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
    pub grab: *mut wlr_seat_keyboard_grab,
    pub default_grab: *mut wlr_seat_keyboard_grab,
    pub events: C2RustUnnamed_8,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub focus_change: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat_keyboard_grab {
    pub interface: *const wlr_keyboard_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_keyboard_grab_interface {
    pub enter: Option<unsafe extern "C" fn(_: *mut wlr_seat_keyboard_grab,
                                           _: *mut wlr_surface,
                                           _: *mut uint32_t, _: size_t,
                                           _: *mut wlr_keyboard_modifiers)
                          -> ()>,
    pub key: Option<unsafe extern "C" fn(_: *mut wlr_seat_keyboard_grab,
                                         _: uint32_t, _: uint32_t,
                                         _: uint32_t) -> ()>,
    pub modifiers: Option<unsafe extern "C" fn(_: *mut wlr_seat_keyboard_grab,
                                               _: *mut wlr_keyboard_modifiers)
                              -> ()>,
    pub cancel: Option<unsafe extern "C" fn(_: *mut wlr_seat_keyboard_grab)
                           -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat_pointer_state {
    pub seat: *mut wlr_seat,
    pub focused_client: *mut wlr_seat_client,
    pub focused_surface: *mut wlr_surface,
    pub sx: libc::c_double,
    pub sy: libc::c_double,
    pub grab: *mut wlr_seat_pointer_grab,
    pub default_grab: *mut wlr_seat_pointer_grab,
    pub buttons: [uint32_t; 16],
    pub button_count: size_t,
    pub grab_button: uint32_t,
    pub grab_serial: uint32_t,
    pub grab_time: uint32_t,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_9,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub focus_change: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat_pointer_grab {
    pub interface: *const wlr_pointer_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer_grab_interface {
    pub enter: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab,
                                           _: *mut wlr_surface,
                                           _: libc::c_double,
                                           _: libc::c_double) -> ()>,
    pub motion: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab,
                                            _: uint32_t, _: libc::c_double,
                                            _: libc::c_double) -> ()>,
    pub button: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab,
                                            _: uint32_t, _: uint32_t,
                                            _: wlr_button_state) -> uint32_t>,
    pub axis: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab,
                                          _: uint32_t,
                                          _: wlr_axis_orientation,
                                          _: libc::c_double, _: int32_t,
                                          _: wlr_axis_source) -> ()>,
    pub frame: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab)
                          -> ()>,
    pub cancel: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab)
                           -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}
pub type xcb_window_t = uint32_t;
pub type xcb_pixmap_t = uint32_t;
pub type xcb_cursor_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_selection_request_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_get_property_reply_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub type_0: xcb_atom_t,
    pub bytes_after: uint32_t,
    pub value_len: uint32_t,
    pub pad0: [uint8_t; 12],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub present: uint8_t,
    pub major_opcode: uint8_t,
    pub first_event: uint8_t,
    pub first_error: uint8_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xwm {
    pub xwayland: *mut wlr_xwayland,
    pub event_source: *mut wl_event_source,
    pub seat: *mut wlr_seat,
    pub ping_timeout: uint32_t,
    pub atoms: [xcb_atom_t; 62],
    pub xcb_conn: *mut xcb_connection_t,
    pub screen: *mut xcb_screen_t,
    pub window: xcb_window_t,
    pub visual_id: xcb_visualid_t,
    pub colormap: xcb_colormap_t,
    pub render_format_id: xcb_render_pictformat_t,
    pub cursor: xcb_cursor_t,
    pub selection_window: xcb_window_t,
    pub clipboard_selection: wlr_xwm_selection,
    pub primary_selection: wlr_xwm_selection,
    pub dnd_window: xcb_window_t,
    pub dnd_selection: wlr_xwm_selection,
    pub focus_surface: *mut wlr_xwayland_surface,
    pub surfaces: wl_list,
    pub unpaired_surfaces: wl_list,
    pub drag: *mut wlr_drag,
    pub drag_focus: *mut wlr_xwayland_surface,
    pub xfixes: *const xcb_query_extension_reply_t,
    pub errors_context: *mut xcb_errors_context_t,
    pub compositor_new_surface: wl_listener,
    pub compositor_destroy: wl_listener,
    pub seat_set_selection: wl_listener,
    pub seat_set_primary_selection: wl_listener,
    pub seat_start_drag: wl_listener,
    pub seat_drag_focus: wl_listener,
    pub seat_drag_motion: wl_listener,
    pub seat_drag_drop: wl_listener,
    pub seat_drag_destroy: wl_listener,
    pub seat_drag_source_destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xwayland_surface {
    pub window_id: xcb_window_t,
    pub xwm: *mut wlr_xwm,
    pub surface_id: uint32_t,
    pub link: wl_list,
    pub unpaired_link: wl_list,
    pub surface: *mut wlr_surface,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub saved_width: uint16_t,
    pub saved_height: uint16_t,
    pub override_redirect: bool,
    pub mapped: bool,
    pub title: *mut libc::c_char,
    pub class: *mut libc::c_char,
    pub instance: *mut libc::c_char,
    pub role: *mut libc::c_char,
    pub pid: pid_t,
    pub has_utf8_title: bool,
    pub children: wl_list,
    pub parent: *mut wlr_xwayland_surface,
    pub parent_link: wl_list,
    pub window_type: *mut xcb_atom_t,
    pub window_type_len: size_t,
    pub protocols: *mut xcb_atom_t,
    pub protocols_len: size_t,
    pub decorations: uint32_t,
    pub hints: *mut wlr_xwayland_surface_hints,
    pub hints_urgency: uint32_t,
    pub size_hints: *mut wlr_xwayland_surface_size_hints,
    pub pinging: bool,
    pub ping_timer: *mut wl_event_source,
    pub modal: bool,
    pub fullscreen: bool,
    pub maximized_vert: bool,
    pub maximized_horz: bool,
    pub has_alpha: bool,
    pub events: C2RustUnnamed_10,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub destroy: wl_signal,
    pub request_configure: wl_signal,
    pub request_move: wl_signal,
    pub request_resize: wl_signal,
    pub request_maximize: wl_signal,
    pub request_fullscreen: wl_signal,
    pub request_activate: wl_signal,
    pub map: wl_signal,
    pub unmap: wl_signal,
    pub set_title: wl_signal,
    pub set_class: wl_signal,
    pub set_role: wl_signal,
    pub set_parent: wl_signal,
    pub set_pid: wl_signal,
    pub set_window_type: wl_signal,
    pub set_hints: wl_signal,
    pub set_decorations: wl_signal,
    pub set_override_redirect: wl_signal,
    pub ping_timeout: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xwayland_surface_size_hints {
    pub flags: uint32_t,
    pub x: int32_t,
    pub y: int32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub min_width: int32_t,
    pub min_height: int32_t,
    pub max_width: int32_t,
    pub max_height: int32_t,
    pub width_inc: int32_t,
    pub height_inc: int32_t,
    pub base_width: int32_t,
    pub base_height: int32_t,
    pub min_aspect_num: int32_t,
    pub min_aspect_den: int32_t,
    pub max_aspect_num: int32_t,
    pub max_aspect_den: int32_t,
    pub win_gravity: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xwayland_surface_hints {
    pub flags: uint32_t,
    pub input: uint32_t,
    pub initial_state: int32_t,
    pub icon_pixmap: xcb_pixmap_t,
    pub icon_window: xcb_window_t,
    pub icon_x: int32_t,
    pub icon_y: int32_t,
    pub icon_mask: xcb_pixmap_t,
    pub window_group: xcb_window_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xwm_selection {
    pub xwm: *mut wlr_xwm,
    pub atom: xcb_atom_t,
    pub window: xcb_window_t,
    pub owner: xcb_window_t,
    pub timestamp: xcb_timestamp_t,
    pub incoming: wlr_xwm_selection_transfer,
    pub outgoing: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xwm_selection_transfer {
    pub selection: *mut wlr_xwm_selection,
    pub incr: bool,
    pub flush_property_on_delete: bool,
    pub property_set: bool,
    pub source_data: wl_array,
    pub source_fd: libc::c_int,
    pub source: *mut wl_event_source,
    pub request: xcb_selection_request_event_t,
    pub outgoing_link: wl_list,
    pub property_start: libc::c_int,
    pub property_reply: *mut xcb_get_property_reply_t,
}
pub type xcb_render_pictformat_t = uint32_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xwayland {
    pub pid: pid_t,
    pub client: *mut wl_client,
    pub sigusr1_source: *mut wl_event_source,
    pub xwm: *mut wlr_xwm,
    pub cursor: *mut wlr_xwayland_cursor,
    pub wm_fd: [libc::c_int; 2],
    pub wl_fd: [libc::c_int; 2],
    pub server_start: time_t,
    pub display: libc::c_int,
    pub display_name: [libc::c_char; 16],
    pub x_fd: [libc::c_int; 2],
    pub x_fd_read_event: [*mut wl_event_source; 2],
    pub lazy: bool,
    pub wl_display: *mut wl_display,
    pub compositor: *mut wlr_compositor,
    pub seat: *mut wlr_seat,
    pub events: C2RustUnnamed_11,
    pub user_event_handler: Option<unsafe extern "C" fn(_: *mut wlr_xwm,
                                                        _:
                                                            *mut xcb_generic_event_t)
                                       -> libc::c_int>,
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub ready: wl_signal,
    pub new_surface: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xwayland_cursor {
    pub pixels: *mut uint8_t,
    pub stride: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
    pub hotspot_x: int32_t,
    pub hotspot_y: int32_t,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal_0: *mut wl_signal) {
    wl_list_init(&mut (*signal_0).listener_list);
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal_0: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal_0).listener_list.prev, &mut (*listener).link);
}
unsafe extern "C" fn safe_close(mut fd: libc::c_int) {
    if fd >= 0i32 { close(fd); };
}
unsafe extern "C" fn fill_arg(mut argv: *mut *mut *mut libc::c_char,
                              mut fmt: *const libc::c_char, mut args: ...)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut cur_arg: *mut *mut libc::c_char = *argv;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    len =
        vsnprintf(0 as *mut libc::c_char, 0i32 as libc::c_ulong, fmt,
                  args_0.as_va_list()) + 1i32;
    while !(*cur_arg).is_null() { cur_arg = cur_arg.offset(1) }
    *cur_arg = malloc(len as libc::c_ulong) as *mut libc::c_char;
    if (*cur_arg).is_null() { return -1i32 }
    *argv = cur_arg;
    args_0 = args.clone();
    len = vsnprintf(*cur_arg, len as libc::c_ulong, fmt, args_0.as_va_list());
    return len;
}
unsafe extern "C" fn exec_xwayland(mut wlr_xwayland: *mut wlr_xwayland) {
    if !set_cloexec((*wlr_xwayland).x_fd[0], 0i32 != 0) ||
           !set_cloexec((*wlr_xwayland).x_fd[1], 0i32 != 0) ||
           !set_cloexec((*wlr_xwayland).wm_fd[1], 0i32 != 0) ||
           !set_cloexec((*wlr_xwayland).wl_fd[1], 0i32 != 0) {
        _exit(1i32);
    }
    /* Make Xwayland signal us when it's ready */
    signal(10i32,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1i32 as libc::intptr_t));
    let mut cmd: [libc::c_char; 9] =
        *::std::mem::transmute::<&[u8; 9],
                                 &mut [libc::c_char; 9]>(b"Xwayland\x00");
    let mut arg1: [libc::c_char; 10] =
        *::std::mem::transmute::<&[u8; 10],
                                 &mut [libc::c_char; 10]>(b"-rootless\x00");
    let mut arg2: [libc::c_char; 11] =
        *::std::mem::transmute::<&[u8; 11],
                                 &mut [libc::c_char; 11]>(b"-terminate\x00");
    let mut arg3: [libc::c_char; 8] =
        *::std::mem::transmute::<&[u8; 8],
                                 &mut [libc::c_char; 8]>(b"-listen\x00");
    let mut arg4: [libc::c_char; 4] =
        *::std::mem::transmute::<&[u8; 4],
                                 &mut [libc::c_char; 4]>(b"-wm\x00");
    let mut argv: [*mut libc::c_char; 11] =
        [cmd.as_mut_ptr(), 0 as *mut libc::c_char, arg1.as_mut_ptr(),
         arg2.as_mut_ptr(), arg3.as_mut_ptr(), 0 as *mut libc::c_char,
         arg3.as_mut_ptr(), 0 as *mut libc::c_char, arg4.as_mut_ptr(),
         0 as *mut libc::c_char, 0 as *mut libc::c_char];
    let mut cur_arg: *mut *mut libc::c_char = argv.as_mut_ptr();
    if fill_arg(&mut cur_arg as *mut *mut *mut libc::c_char,
                b":%d\x00" as *const u8 as *const libc::c_char,
                (*wlr_xwayland).display) < 0i32 ||
           fill_arg(&mut cur_arg as *mut *mut *mut libc::c_char,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (*wlr_xwayland).x_fd[0]) < 0i32 ||
           fill_arg(&mut cur_arg as *mut *mut *mut libc::c_char,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (*wlr_xwayland).x_fd[1]) < 0i32 ||
           fill_arg(&mut cur_arg as *mut *mut *mut libc::c_char,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (*wlr_xwayland).wm_fd[1]) < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] alloc/print failure: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/xwayland.c\x00" as *const u8 as
                     *const libc::c_char, 86i32,
                 strerror(*__errno_location()));
        _exit(1i32);
    }
    let mut wayland_socket_str: [libc::c_char; 16] = [0; 16];
    snprintf(wayland_socket_str.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
             b"%d\x00" as *const u8 as *const libc::c_char,
             (*wlr_xwayland).wl_fd[1]);
    setenv(b"WAYLAND_SOCKET\x00" as *const u8 as *const libc::c_char,
           wayland_socket_str.as_mut_ptr(), 1i32);
    _wlr_log(WLR_INFO,
             b"[%s:%d] WAYLAND_SOCKET=%d Xwayland :%d -rootless -terminate -listen %d -listen %d -wm %d\x00"
                 as *const u8 as *const libc::c_char,
             b"../xwayland/xwayland.c\x00" as *const u8 as
                 *const libc::c_char, 96i32, (*wlr_xwayland).wl_fd[1],
             (*wlr_xwayland).display, (*wlr_xwayland).x_fd[0],
             (*wlr_xwayland).x_fd[1], (*wlr_xwayland).wm_fd[1]);
    // Closes stdout/stderr depending on log verbosity
    let mut verbosity: wlr_log_importance = wlr_log_get_verbosity();
    let mut devnull: libc::c_int =
        open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
             0o1i32 | 0o100i32 | 0o2000000i32, 0o666i32);
    if devnull < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] XWayland: failed to open /dev/null: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../xwayland/xwayland.c\x00" as *const u8 as
                     *const libc::c_char, 102i32,
                 strerror(*__errno_location()));
        _exit(1i32);
    }
    if (verbosity as libc::c_uint) < WLR_INFO as libc::c_int as libc::c_uint {
        dup2(devnull, 1i32);
    }
    if (verbosity as libc::c_uint) < WLR_ERROR as libc::c_int as libc::c_uint
       {
        dup2(devnull, 2i32);
    }
    // This returns if and only if the call fails
    execvp(b"Xwayland\x00" as *const u8 as *const libc::c_char,
           argv.as_mut_ptr() as *const *mut libc::c_char);
    _wlr_log(WLR_ERROR,
             b"[%s:%d] failed to exec Xwayland: %s\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwayland.c\x00" as *const u8 as
                 *const libc::c_char, 115i32, strerror(*__errno_location()));
    close(devnull);
    _exit(1i32);
}
unsafe extern "C" fn xwayland_finish_server(mut wlr_xwayland:
                                                *mut wlr_xwayland) {
    if wlr_xwayland.is_null() || (*wlr_xwayland).display == -1i32 { return }
    if !(*wlr_xwayland).x_fd_read_event[0].is_null() {
        wl_event_source_remove((*wlr_xwayland).x_fd_read_event[0]);
        wl_event_source_remove((*wlr_xwayland).x_fd_read_event[1]);
        (*wlr_xwayland).x_fd_read_event[1] = 0 as *mut wl_event_source;
        (*wlr_xwayland).x_fd_read_event[0] =
            (*wlr_xwayland).x_fd_read_event[1]
    }
    if !(*wlr_xwayland).cursor.is_null() {
        free((*wlr_xwayland).cursor as *mut libc::c_void);
    }
    xwm_destroy((*wlr_xwayland).xwm);
    if !(*wlr_xwayland).client.is_null() {
        wl_list_remove(&mut (*wlr_xwayland).client_destroy.link);
        wl_client_destroy((*wlr_xwayland).client);
    }
    if !(*wlr_xwayland).sigusr1_source.is_null() {
        wl_event_source_remove((*wlr_xwayland).sigusr1_source);
    }
    safe_close((*wlr_xwayland).wl_fd[0]);
    safe_close((*wlr_xwayland).wl_fd[1]);
    safe_close((*wlr_xwayland).wm_fd[0]);
    safe_close((*wlr_xwayland).wm_fd[1]);
    memset(wlr_xwayland as *mut libc::c_void, 0i32, 64u64);
    (*wlr_xwayland).wl_fd[1] = -1i32;
    (*wlr_xwayland).wl_fd[0] = (*wlr_xwayland).wl_fd[1];
    (*wlr_xwayland).wm_fd[1] = -1i32;
    (*wlr_xwayland).wm_fd[0] = (*wlr_xwayland).wm_fd[1];
    /* We do not kill the Xwayland process, it dies to broken pipe
	 * after we close our side of the wm/wl fds. This is more reliable
	 * than trying to kill something that might no longer be Xwayland.
	 */
}
unsafe extern "C" fn xwayland_finish_display(mut wlr_xwayland:
                                                 *mut wlr_xwayland) {
    if wlr_xwayland.is_null() || (*wlr_xwayland).display == -1i32 { return }
    safe_close((*wlr_xwayland).x_fd[0]);
    safe_close((*wlr_xwayland).x_fd[1]);
    (*wlr_xwayland).x_fd[1] = -1i32;
    (*wlr_xwayland).x_fd[0] = (*wlr_xwayland).x_fd[1];
    wl_list_remove(&mut (*wlr_xwayland).display_destroy.link);
    unlink_display_sockets((*wlr_xwayland).display);
    (*wlr_xwayland).display = -1i32;
    (*wlr_xwayland).display_name[0] = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn handle_client_destroy(mut listener: *mut wl_listener,
                                           mut data: *mut libc::c_void) {
    let mut wlr_xwayland: *mut wlr_xwayland =
        (listener as *mut libc::c_char).offset(-184) as *mut wlr_xwayland;
    if !(*wlr_xwayland).sigusr1_source.is_null() {
        // Xwayland failed to start, let the sigusr1 handler deal with it
        return
    }
    // Don't call client destroy: it's being destroyed already
    (*wlr_xwayland).client = 0 as *mut wl_client;
    wl_list_remove(&mut (*wlr_xwayland).client_destroy.link);
    xwayland_finish_server(wlr_xwayland);
    if time(0 as *mut time_t) - (*wlr_xwayland).server_start >
           5i32 as libc::c_long {
        if (*wlr_xwayland).lazy {
            _wlr_log(WLR_INFO,
                     b"[%s:%d] Restarting Xwayland (lazy)\x00" as *const u8 as
                         *const libc::c_char,
                     b"../xwayland/xwayland.c\x00" as *const u8 as
                         *const libc::c_char, 196i32);
            xwayland_start_server_lazy(wlr_xwayland);
        } else {
            _wlr_log(WLR_INFO,
                     b"[%s:%d] Restarting Xwayland\x00" as *const u8 as
                         *const libc::c_char,
                     b"../xwayland/xwayland.c\x00" as *const u8 as
                         *const libc::c_char, 199i32);
            xwayland_start_server(wlr_xwayland);
        }
    };
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut wlr_xwayland: *mut wlr_xwayland =
        (listener as *mut libc::c_char).offset(-208) as *mut wlr_xwayland;
    // Don't call client destroy: the display is being destroyed, it's too late
    if !(*wlr_xwayland).client.is_null() {
        (*wlr_xwayland).client = 0 as *mut wl_client;
        wl_list_remove(&mut (*wlr_xwayland).client_destroy.link);
    }
    wlr_xwayland_destroy(wlr_xwayland);
}
unsafe extern "C" fn xserver_handle_ready(mut signal_number: libc::c_int,
                                          mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut wlr_xwayland: *mut wlr_xwayland = data as *mut wlr_xwayland;
    let mut stat_val: libc::c_int = -1i32;
    while waitpid((*wlr_xwayland).pid, &mut stat_val, 0i32) < 0i32 {
        if *__errno_location() == 4i32 { continue ; }
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] waitpid for Xwayland fork failed: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../xwayland/xwayland.c\x00" as *const u8 as
                     *const libc::c_char, 226i32,
                 strerror(*__errno_location()));
        return 1i32
    }
    if stat_val != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Xwayland startup failed, not setting up xwm\x00" as
                     *const u8 as *const libc::c_char,
                 b"../xwayland/xwayland.c\x00" as *const u8 as
                     *const libc::c_char, 230i32);
        return 1i32
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Xserver is ready\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwayland.c\x00" as *const u8 as
                 *const libc::c_char, 233i32);
    (*wlr_xwayland).xwm = xwm_create(wlr_xwayland);
    if (*wlr_xwayland).xwm.is_null() {
        xwayland_finish_server(wlr_xwayland);
        return 1i32
    }
    if !(*wlr_xwayland).seat.is_null() {
        xwm_set_seat((*wlr_xwayland).xwm, (*wlr_xwayland).seat);
    }
    wl_event_source_remove((*wlr_xwayland).sigusr1_source);
    (*wlr_xwayland).sigusr1_source = 0 as *mut wl_event_source;
    if !(*wlr_xwayland).cursor.is_null() {
        let mut cur: *mut wlr_xwayland_cursor = (*wlr_xwayland).cursor;
        xwm_set_cursor((*wlr_xwayland).xwm, (*cur).pixels, (*cur).stride,
                       (*cur).width, (*cur).height, (*cur).hotspot_x,
                       (*cur).hotspot_y);
        free(cur as *mut libc::c_void);
        (*wlr_xwayland).cursor = 0 as *mut wlr_xwayland_cursor
    }
    wlr_signal_emit_safe(&mut (*wlr_xwayland).events.ready,
                         wlr_xwayland as *mut libc::c_void);
    /* ready is a one-shot signal, fire and forget */
    wl_signal_init(&mut (*wlr_xwayland).events.ready);
    return 1i32;
    /* wayland event loop dispatcher's count */
}
unsafe extern "C" fn xwayland_socket_connected(mut fd: libc::c_int,
                                               mut mask: uint32_t,
                                               mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut wlr_xwayland: *mut wlr_xwayland =
        data as *mut wlr_xwayland; /* not ours anymore */
    wl_event_source_remove((*wlr_xwayland).x_fd_read_event[0]);
    wl_event_source_remove((*wlr_xwayland).x_fd_read_event[1]);
    (*wlr_xwayland).x_fd_read_event[1] = 0 as *mut wl_event_source;
    (*wlr_xwayland).x_fd_read_event[0] = (*wlr_xwayland).x_fd_read_event[1];
    xwayland_start_server(wlr_xwayland);
    return 0i32;
}
unsafe extern "C" fn xwayland_start_display(mut wlr_xwayland:
                                                *mut wlr_xwayland,
                                            mut wl_display: *mut wl_display)
 -> bool {
    (*wlr_xwayland).display_destroy.notify =
        Some(handle_display_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_display_add_destroy_listener(wl_display,
                                    &mut (*wlr_xwayland).display_destroy);
    (*wlr_xwayland).display =
        open_display_sockets((*wlr_xwayland).x_fd.as_mut_ptr());
    if (*wlr_xwayland).display < 0i32 {
        xwayland_finish_display(wlr_xwayland);
        return 0i32 != 0
    }
    snprintf((*wlr_xwayland).display_name.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
             b":%d\x00" as *const u8 as *const libc::c_char,
             (*wlr_xwayland).display);
    return 1i32 != 0;
}
unsafe extern "C" fn xwayland_start_server(mut wlr_xwayland:
                                               *mut wlr_xwayland) -> bool {
    if socketpair(1i32, SOCK_STREAM as libc::c_int, 0i32,
                  (*wlr_xwayland).wl_fd.as_mut_ptr()) != 0i32 ||
           socketpair(1i32, SOCK_STREAM as libc::c_int, 0i32,
                      (*wlr_xwayland).wm_fd.as_mut_ptr()) != 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] socketpair failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/xwayland.c\x00" as *const u8 as
                     *const libc::c_char, 297i32,
                 strerror(*__errno_location()));
        xwayland_finish_server(wlr_xwayland);
        return 0i32 != 0
    }
    if !set_cloexec((*wlr_xwayland).wl_fd[0], 1i32 != 0) ||
           !set_cloexec((*wlr_xwayland).wl_fd[1], 1i32 != 0) ||
           !set_cloexec((*wlr_xwayland).wm_fd[0], 1i32 != 0) ||
           !set_cloexec((*wlr_xwayland).wm_fd[1], 1i32 != 0) {
        xwayland_finish_server(wlr_xwayland);
        return 0i32 != 0
    }
    (*wlr_xwayland).server_start = time(0 as *mut time_t);
    (*wlr_xwayland).client =
        wl_client_create((*wlr_xwayland).wl_display,
                         (*wlr_xwayland).wl_fd[0]);
    if (*wlr_xwayland).client.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] wl_client_create failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/xwayland.c\x00" as *const u8 as
                     *const libc::c_char, 314i32,
                 strerror(*__errno_location()));
        xwayland_finish_server(wlr_xwayland);
        return 0i32 != 0
    }
    (*wlr_xwayland).wl_fd[0] = -1i32;
    (*wlr_xwayland).client_destroy.notify =
        Some(handle_client_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_client_add_destroy_listener((*wlr_xwayland).client,
                                   &mut (*wlr_xwayland).client_destroy);
    let mut loop_0: *mut wl_event_loop =
        wl_display_get_event_loop((*wlr_xwayland).wl_display);
    (*wlr_xwayland).sigusr1_source =
        wl_event_loop_add_signal(loop_0, 10i32,
                                 Some(xserver_handle_ready as
                                          unsafe extern "C" fn(_: libc::c_int,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> libc::c_int),
                                 wlr_xwayland as *mut libc::c_void);
    (*wlr_xwayland).pid = fork();
    if (*wlr_xwayland).pid < 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] fork failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/xwayland.c\x00" as *const u8 as
                     *const libc::c_char, 331i32,
                 strerror(*__errno_location()));
        xwayland_finish_server(wlr_xwayland);
        return 0i32 != 0
    } else {
        if (*wlr_xwayland).pid == 0i32 {
            /* Double-fork, but we need to forward SIGUSR1 once Xserver(1)
		 * is ready, or error if there was one. */
            let mut ppid: pid_t = getppid();
            let mut sigset: sigset_t = sigset_t{__val: [0; 16],};
            sigemptyset(&mut sigset);
            sigaddset(&mut sigset, 10i32);
            sigaddset(&mut sigset, 17i32);
            sigprocmask(0i32, &mut sigset, 0 as *mut sigset_t);
            let mut pid: pid_t = fork();
            if pid < 0i32 {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] second fork failed: %s\x00" as *const u8 as
                             *const libc::c_char,
                         b"../xwayland/xwayland.c\x00" as *const u8 as
                             *const libc::c_char, 346i32,
                         strerror(*__errno_location()));
                _exit(1i32);
            } else { if pid == 0i32 { exec_xwayland(wlr_xwayland); } }
            let mut sig: libc::c_int = 0;
            sigwait(&mut sigset, &mut sig);
            kill(ppid, 10i32);
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] sent SIGUSR1 to process %d\x00" as *const u8 as
                         *const libc::c_char,
                     b"../xwayland/xwayland.c\x00" as *const u8 as
                         *const libc::c_char, 355i32, ppid);
            if sig == 17i32 {
                waitpid(pid, 0 as *mut libc::c_int, 0i32);
                _exit(1i32);
            }
            _exit(0i32);
        }
    }
    /* close child fds */
	/* remain managing x sockets for lazy start */
    close((*wlr_xwayland).wl_fd[1]);
    close((*wlr_xwayland).wm_fd[1]);
    (*wlr_xwayland).wm_fd[1] = -1i32;
    (*wlr_xwayland).wl_fd[1] = (*wlr_xwayland).wm_fd[1];
    return 1i32 != 0;
}
unsafe extern "C" fn xwayland_start_server_lazy(mut wlr_xwayland:
                                                    *mut wlr_xwayland)
 -> bool {
    let mut loop_0: *mut wl_event_loop =
        wl_display_get_event_loop((*wlr_xwayland).wl_display);
    (*wlr_xwayland).x_fd_read_event[0] =
        wl_event_loop_add_fd(loop_0, (*wlr_xwayland).x_fd[0],
                             WL_EVENT_READABLE as libc::c_int as uint32_t,
                             Some(xwayland_socket_connected as
                                      unsafe extern "C" fn(_: libc::c_int,
                                                           _: uint32_t,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),
                             wlr_xwayland as *mut libc::c_void);
    (*wlr_xwayland).x_fd_read_event[1] =
        wl_event_loop_add_fd(loop_0, (*wlr_xwayland).x_fd[1],
                             WL_EVENT_READABLE as libc::c_int as uint32_t,
                             Some(xwayland_socket_connected as
                                      unsafe extern "C" fn(_: libc::c_int,
                                                           _: uint32_t,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),
                             wlr_xwayland as *mut libc::c_void);
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_destroy(mut wlr_xwayland:
                                                  *mut wlr_xwayland) {
    if wlr_xwayland.is_null() { return }
    wlr_xwayland_set_seat(wlr_xwayland, 0 as *mut wlr_seat);
    xwayland_finish_server(wlr_xwayland);
    xwayland_finish_display(wlr_xwayland);
    free(wlr_xwayland as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_create(mut wl_display: *mut wl_display,
                                             mut compositor:
                                                 *mut wlr_compositor,
                                             mut lazy: bool)
 -> *mut wlr_xwayland {
    let mut current_block: u64;
    let mut wlr_xwayland: *mut wlr_xwayland =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xwayland>() as libc::c_ulong) as
            *mut wlr_xwayland;
    if wlr_xwayland.is_null() { return 0 as *mut wlr_xwayland }
    (*wlr_xwayland).wl_display = wl_display;
    (*wlr_xwayland).compositor = compositor;
    (*wlr_xwayland).lazy = lazy;
    (*wlr_xwayland).x_fd[1] = -1i32;
    (*wlr_xwayland).x_fd[0] = (*wlr_xwayland).x_fd[1];
    (*wlr_xwayland).wl_fd[1] = -1i32;
    (*wlr_xwayland).wl_fd[0] = (*wlr_xwayland).wl_fd[1];
    (*wlr_xwayland).wm_fd[1] = -1i32;
    (*wlr_xwayland).wm_fd[0] = (*wlr_xwayland).wm_fd[1];
    wl_signal_init(&mut (*wlr_xwayland).events.new_surface);
    wl_signal_init(&mut (*wlr_xwayland).events.ready);
    if xwayland_start_display(wlr_xwayland, wl_display) {
        if (*wlr_xwayland).lazy {
            if !xwayland_start_server_lazy(wlr_xwayland) {
                current_block = 9506658434446853548;
            } else { current_block = 15976848397966268834; }
        } else if !xwayland_start_server(wlr_xwayland) {
            current_block = 9506658434446853548;
        } else { current_block = 15976848397966268834; }
        match current_block {
            15976848397966268834 => { return wlr_xwayland }
            _ => { xwayland_finish_display(wlr_xwayland); }
        }
    }
    free(wlr_xwayland as *mut libc::c_void);
    return 0 as *mut wlr_xwayland;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_set_cursor(mut wlr_xwayland:
                                                     *mut wlr_xwayland,
                                                 mut pixels: *mut uint8_t,
                                                 mut stride: uint32_t,
                                                 mut width: uint32_t,
                                                 mut height: uint32_t,
                                                 mut hotspot_x: int32_t,
                                                 mut hotspot_y: int32_t) {
    if !(*wlr_xwayland).xwm.is_null() {
        xwm_set_cursor((*wlr_xwayland).xwm, pixels, stride, width, height,
                       hotspot_x, hotspot_y);
        return
    }
    free((*wlr_xwayland).cursor as *mut libc::c_void);
    (*wlr_xwayland).cursor =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xwayland_cursor>() as libc::c_ulong)
            as *mut wlr_xwayland_cursor;
    if (*wlr_xwayland).cursor.is_null() { return }
    (*(*wlr_xwayland).cursor).pixels = pixels;
    (*(*wlr_xwayland).cursor).stride = stride;
    (*(*wlr_xwayland).cursor).width = width;
    (*(*wlr_xwayland).cursor).height = height;
    (*(*wlr_xwayland).cursor).hotspot_x = hotspot_x;
    (*(*wlr_xwayland).cursor).hotspot_y = hotspot_y;
}
unsafe extern "C" fn xwayland_handle_seat_destroy(mut listener:
                                                      *mut wl_listener,
                                                  mut data:
                                                      *mut libc::c_void) {
    let mut xwayland: *mut wlr_xwayland =
        (listener as *mut libc::c_char).offset(-232) as *mut wlr_xwayland;
    wlr_xwayland_set_seat(xwayland, 0 as *mut wlr_seat);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* Anything above display is reset on Xwayland restart, rest is conserved */
/* *
	 * Add a custom event handler to xwayland. Return 1 if the event was
	 * handled or 0 to use the default wlr-xwayland handler. wlr-xwayland will
	 * free the event.
	 */
/* *
 * An Xwayland user interface component. It has an absolute position in
 * layout-local coordinates.
 *
 * When a surface is ready to be displayed, the `map` event is emitted. When a
 * surface should no longer be displayed, the `unmap` event is emitted. The
 * `unmap` event is guaranteed to be emitted before the `destroy` event if the
 * view is destroyed when mapped.
 */
// wlr_xwayland_surface::parent_link
// wlr_xwayland_surface::children
// _NET_WM_STATE
// xcb_config_window_t
// TODO: maybe add a seat to these
/* * Create an Xwayland server.
 *
 * The server supports a lazy mode in which Xwayland is only started when a
 * client tries to connect.
 *
 * Note: wlr_xwayland will setup a global SIGUSR1 handler on the compositor
 * process.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_set_seat(mut xwayland:
                                                   *mut wlr_xwayland,
                                               mut seat: *mut wlr_seat) {
    if !(*xwayland).seat.is_null() {
        wl_list_remove(&mut (*xwayland).seat_destroy.link);
    }
    (*xwayland).seat = seat;
    if !(*xwayland).xwm.is_null() { xwm_set_seat((*xwayland).xwm, seat); }
    if seat.is_null() { return }
    (*xwayland).seat_destroy.notify =
        Some(xwayland_handle_seat_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*seat).events.destroy, &mut (*xwayland).seat_destroy);
}
