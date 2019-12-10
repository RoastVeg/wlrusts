use libc;
extern "C" {
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_texture;
    pub type wlr_renderer;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_length(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_resource_create(client: *mut wl_client,
                          interface: *const wl_interface,
                          version: libc::c_int, id: uint32_t)
     -> *mut wl_resource;
    #[no_mangle]
    fn wl_resource_set_implementation(resource: *mut wl_resource,
                                      implementation: *const libc::c_void,
                                      data: *mut libc::c_void,
                                      destroy: wl_resource_destroy_func_t);
    #[no_mangle]
    fn wl_resource_destroy(resource: *mut wl_resource);
    #[no_mangle]
    fn wl_resource_get_link(resource: *mut wl_resource) -> *mut wl_list;
    #[no_mangle]
    fn wl_resource_from_link(resource: *mut wl_list) -> *mut wl_resource;
    #[no_mangle]
    fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;
    #[no_mangle]
    fn wl_resource_set_user_data(resource: *mut wl_resource,
                                 data: *mut libc::c_void);
    #[no_mangle]
    fn wl_resource_get_user_data(resource: *mut wl_resource)
     -> *mut libc::c_void;
    #[no_mangle]
    fn wl_resource_instance_of(resource: *mut wl_resource,
                               interface: *const wl_interface,
                               implementation: *const libc::c_void)
     -> libc::c_int;
    // Returns the log verbosity provided to wlr_log_init
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    static wl_touch_interface: wl_interface;
    #[no_mangle]
    fn wlr_seat_client_for_wl_client(wlr_seat: *mut wlr_seat,
                                     wl_client: *mut wl_client)
     -> *mut wlr_seat_client;
    #[no_mangle]
    fn wlr_seat_client_next_serial(client: *mut wlr_seat_client) -> uint32_t;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type wl_fixed_t = int32_t;
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: int64_t,
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
    pub repeat_info: C2RustUnnamed_1,
    pub events: C2RustUnnamed_0,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
/*
 * 32 bit regions
 */
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
pub struct wl_touch_interface {
    pub release: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
}
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
    pub events: C2RustUnnamed_2,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub commit: wl_signal,
    pub new_subsurface: wl_signal,
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
// enum wlr_surface_state_field
// relative to previous position
// clipped to bounds
// wl_resource
// in surface-local coordinates
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
    pub events: C2RustUnnamed_3,
    pub serials: wlr_serial_ringset,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
    pub events: C2RustUnnamed_4,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub events: C2RustUnnamed_5,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub events: C2RustUnnamed_6,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub events: C2RustUnnamed_7,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
#[inline]
unsafe extern "C" fn wl_fixed_from_double(mut d: libc::c_double)
 -> wl_fixed_t {
    let mut u: C2RustUnnamed = C2RustUnnamed{d: 0.,};
    u.d = d + (3i64 << 51i32 - 8i32) as libc::c_double;
    return u.i as wl_fixed_t;
}
#[no_mangle]
pub static mut default_keyboard_grab_impl: wlr_keyboard_grab_interface =
    wlr_keyboard_grab_interface{enter: None,
                                key: None,
                                modifiers: None,
                                cancel: None,};
#[no_mangle]
pub static mut default_pointer_grab_impl: wlr_pointer_grab_interface =
    wlr_pointer_grab_interface{enter: None,
                               motion: None,
                               button: None,
                               axis: None,
                               frame: None,
                               cancel: None,};
#[inline]
unsafe extern "C" fn wl_touch_send_down(mut resource_: *mut wl_resource,
                                        mut serial: uint32_t,
                                        mut time: uint32_t,
                                        mut surface: *mut wl_resource,
                                        mut id: int32_t, mut x: wl_fixed_t,
                                        mut y: wl_fixed_t) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, serial, time, surface,
                           id, x, y);
}
#[inline]
unsafe extern "C" fn wl_touch_send_up(mut resource_: *mut wl_resource,
                                      mut serial: uint32_t,
                                      mut time: uint32_t, mut id: int32_t) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, serial, time, id);
}
#[inline]
unsafe extern "C" fn wl_touch_send_motion(mut resource_: *mut wl_resource,
                                          mut time: uint32_t, mut id: int32_t,
                                          mut x: wl_fixed_t,
                                          mut y: wl_fixed_t) {
    wl_resource_post_event(resource_, 2i32 as uint32_t, time, id, x, y);
}
#[inline]
unsafe extern "C" fn wl_touch_send_frame(mut resource_: *mut wl_resource) {
    wl_resource_post_event(resource_, 3i32 as uint32_t);
}
unsafe extern "C" fn default_touch_down(mut grab: *mut wlr_seat_touch_grab,
                                        mut time: uint32_t,
                                        mut point: *mut wlr_touch_point)
 -> uint32_t {
    return wlr_seat_touch_send_down((*grab).seat, (*point).surface, time,
                                    (*point).touch_id, (*point).sx,
                                    (*point).sy);
}
unsafe extern "C" fn default_touch_up(mut grab: *mut wlr_seat_touch_grab,
                                      mut time: uint32_t,
                                      mut point: *mut wlr_touch_point) {
    wlr_seat_touch_send_up((*grab).seat, time, (*point).touch_id);
}
unsafe extern "C" fn default_touch_motion(mut grab: *mut wlr_seat_touch_grab,
                                          mut time: uint32_t,
                                          mut point: *mut wlr_touch_point) {
    if (*point).focus_surface.is_null() ||
           (*point).focus_surface == (*point).surface {
        wlr_seat_touch_send_motion((*grab).seat, time, (*point).touch_id,
                                   (*point).sx, (*point).sy);
    };
}
unsafe extern "C" fn default_touch_enter(mut grab: *mut wlr_seat_touch_grab,
                                         mut time: uint32_t,
                                         mut point: *mut wlr_touch_point) {
    // not handled by default
}
unsafe extern "C" fn default_touch_cancel(mut grab:
                                              *mut wlr_seat_touch_grab) {
    // cannot be cancelled
}
#[no_mangle]
pub static mut default_touch_grab_impl: wlr_touch_grab_interface =
    unsafe {
        {
            let mut init =
                wlr_touch_grab_interface{down:
                                             Some(default_touch_down as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> uint32_t),
                                         up:
                                             Some(default_touch_up as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> ()),
                                         motion:
                                             Some(default_touch_motion as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> ()),
                                         enter:
                                             Some(default_touch_enter as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> ()),
                                         cancel:
                                             Some(default_touch_cancel as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab)
                                                          -> ()),};
            init
        }
    };
unsafe extern "C" fn touch_release(mut client: *mut wl_client,
                                   mut resource: *mut wl_resource) {
    wl_resource_destroy(resource);
}
static mut touch_impl: wl_touch_interface =
    unsafe {
        {
            let mut init =
                wl_touch_interface{release:
                                       Some(touch_release as
                                                unsafe extern "C" fn(_:
                                                                         *mut wl_client,
                                                                     _:
                                                                         *mut wl_resource)
                                                    -> ()),};
            init
        }
    };
unsafe extern "C" fn touch_handle_resource_destroy(mut resource:
                                                       *mut wl_resource) {
    wl_list_remove(wl_resource_get_link(resource));
    seat_client_destroy_touch(resource);
}
unsafe extern "C" fn seat_client_from_touch_resource(mut resource:
                                                         *mut wl_resource)
 -> *mut wlr_seat_client {
    if wl_resource_instance_of(resource, &wl_touch_interface,
                               &touch_impl as *const wl_touch_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_touch_interface, &touch_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/seat/wlr_seat_touch.c\x00" as *const u8 as
                          *const libc::c_char, 66i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 78],
                                                &[libc::c_char; 78]>(b"struct wlr_seat_client *seat_client_from_touch_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_seat_client;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_start_grab(mut wlr_seat:
                                                       *mut wlr_seat,
                                                   mut grab:
                                                       *mut wlr_seat_touch_grab) {
    (*grab).seat = wlr_seat;
    (*wlr_seat).touch_state.grab = grab;
    wlr_signal_emit_safe(&mut (*wlr_seat).events.touch_grab_begin,
                         grab as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_end_grab(mut wlr_seat:
                                                     *mut wlr_seat) {
    let mut grab: *mut wlr_seat_touch_grab = (*wlr_seat).touch_state.grab;
    if grab != (*wlr_seat).touch_state.default_grab {
        (*wlr_seat).touch_state.grab = (*wlr_seat).touch_state.default_grab;
        wlr_signal_emit_safe(&mut (*wlr_seat).events.touch_grab_end,
                             grab as *mut libc::c_void);
        if (*(*grab).interface).cancel.is_some() {
            (*(*grab).interface).cancel.expect("non-null function pointer")(grab);
        }
    };
}
unsafe extern "C" fn touch_point_clear_focus(mut point:
                                                 *mut wlr_touch_point) {
    if !(*point).focus_surface.is_null() {
        wl_list_remove(&mut (*point).focus_surface_destroy.link);
        (*point).focus_client = 0 as *mut wlr_seat_client;
        (*point).focus_surface = 0 as *mut wlr_surface
    };
}
unsafe extern "C" fn touch_point_destroy(mut point: *mut wlr_touch_point) {
    wlr_signal_emit_safe(&mut (*point).events.destroy,
                         point as *mut libc::c_void);
    touch_point_clear_focus(point);
    wl_list_remove(&mut (*point).surface_destroy.link);
    wl_list_remove(&mut (*point).client_destroy.link);
    wl_list_remove(&mut (*point).link);
    free(point as *mut libc::c_void);
}
unsafe extern "C" fn touch_point_handle_surface_destroy(mut listener:
                                                            *mut wl_listener,
                                                        mut data:
                                                            *mut libc::c_void) {
    let mut point: *mut wlr_touch_point =
        (listener as *mut libc::c_char).offset(-56) as *mut wlr_touch_point;
    // Touch point itself is destroyed on up event
    (*point).surface = 0 as *mut wlr_surface;
    wl_list_remove(&mut (*point).surface_destroy.link);
    wl_list_init(&mut (*point).surface_destroy.link);
}
unsafe extern "C" fn touch_point_handle_client_destroy(mut listener:
                                                           *mut wl_listener,
                                                       mut data:
                                                           *mut libc::c_void) {
    let mut point: *mut wlr_touch_point =
        (listener as *mut libc::c_char).offset(-104) as *mut wlr_touch_point;
    touch_point_destroy(point);
}
unsafe extern "C" fn touch_point_create(mut seat: *mut wlr_seat,
                                        mut touch_id: int32_t,
                                        mut surface: *mut wlr_surface,
                                        mut sx: libc::c_double,
                                        mut sy: libc::c_double)
 -> *mut wlr_touch_point {
    let mut wl_client: *mut wl_client =
        wl_resource_get_client((*surface).resource);
    let mut client: *mut wlr_seat_client =
        wlr_seat_client_for_wl_client(seat, wl_client);
    if client.is_null() || wl_list_empty(&mut (*client).touches) != 0 {
        // touch points are not valid without a connected client with touch
        return 0 as *mut wlr_touch_point
    }
    let mut point: *mut wlr_touch_point =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_touch_point>() as libc::c_ulong) as
            *mut wlr_touch_point;
    if point.is_null() { return 0 as *mut wlr_touch_point }
    (*point).touch_id = touch_id;
    (*point).surface = surface;
    (*point).client = client;
    (*point).sx = sx;
    (*point).sy = sy;
    wl_signal_init(&mut (*point).events.destroy);
    wl_signal_add(&mut (*surface).events.destroy,
                  &mut (*point).surface_destroy);
    (*point).surface_destroy.notify =
        Some(touch_point_handle_surface_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*client).events.destroy,
                  &mut (*point).client_destroy);
    (*point).client_destroy.notify =
        Some(touch_point_handle_client_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_list_insert(&mut (*seat).touch_state.touch_points, &mut (*point).link);
    return point;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_get_point(mut seat: *mut wlr_seat,
                                                  mut touch_id: int32_t)
 -> *mut wlr_touch_point {
    let mut point: *mut wlr_touch_point = 0 as *mut wlr_touch_point;
    point =
        ((*seat).touch_state.touch_points.next as
             *mut libc::c_char).offset(-144) as *mut wlr_touch_point;
    while &mut (*point).link as *mut wl_list !=
              &mut (*seat).touch_state.touch_points as *mut wl_list {
        if (*point).touch_id == touch_id { return point }
        point =
            ((*point).link.next as *mut libc::c_char).offset(-144) as
                *mut wlr_touch_point
    }
    return 0 as *mut wlr_touch_point;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_notify_down(mut seat: *mut wlr_seat,
                                                    mut surface:
                                                        *mut wlr_surface,
                                                    mut time: uint32_t,
                                                    mut touch_id: int32_t,
                                                    mut sx: libc::c_double,
                                                    mut sy: libc::c_double)
 -> uint32_t {
    clock_gettime(1i32, &mut (*seat).last_event);
    let mut grab: *mut wlr_seat_touch_grab = (*seat).touch_state.grab;
    let mut point: *mut wlr_touch_point =
        touch_point_create(seat, touch_id, surface, sx, sy);
    if point.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] could not create touch point\x00" as *const u8 as
                     *const libc::c_char,
                 b"../types/seat/wlr_seat_touch.c\x00" as *const u8 as
                     *const libc::c_char, 181i32);
        return 0i32 as uint32_t
    }
    let mut serial: uint32_t =
        (*(*grab).interface).down.expect("non-null function pointer")(grab,
                                                                      time,
                                                                      point);
    if serial == 0 { touch_point_destroy(point); return 0i32 as uint32_t }
    if serial != 0 && wlr_seat_touch_num_points(seat) == 1i32 {
        (*seat).touch_state.grab_serial = serial;
        (*seat).touch_state.grab_id = touch_id as uint32_t
    }
    return serial;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_notify_up(mut seat: *mut wlr_seat,
                                                  mut time: uint32_t,
                                                  mut touch_id: int32_t) {
    clock_gettime(1i32, &mut (*seat).last_event);
    let mut grab: *mut wlr_seat_touch_grab = (*seat).touch_state.grab;
    let mut point: *mut wlr_touch_point =
        wlr_seat_touch_get_point(seat, touch_id);
    if point.is_null() { return }
    (*(*grab).interface).up.expect("non-null function pointer")(grab, time,
                                                                point);
    touch_point_destroy(point);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_notify_motion(mut seat: *mut wlr_seat,
                                                      mut time: uint32_t,
                                                      mut touch_id: int32_t,
                                                      mut sx: libc::c_double,
                                                      mut sy:
                                                          libc::c_double) {
    clock_gettime(1i32, &mut (*seat).last_event);
    let mut grab: *mut wlr_seat_touch_grab = (*seat).touch_state.grab;
    let mut point: *mut wlr_touch_point =
        wlr_seat_touch_get_point(seat, touch_id);
    if point.is_null() { return }
    (*point).sx = sx;
    (*point).sy = sy;
    (*(*grab).interface).motion.expect("non-null function pointer")(grab,
                                                                    time,
                                                                    point);
}
unsafe extern "C" fn handle_point_focus_destroy(mut listener:
                                                    *mut wl_listener,
                                                mut data: *mut libc::c_void) {
    let mut point: *mut wlr_touch_point =
        (listener as *mut libc::c_char).offset(-80) as *mut wlr_touch_point;
    touch_point_clear_focus(point);
}
unsafe extern "C" fn touch_point_set_focus(mut point: *mut wlr_touch_point,
                                           mut surface: *mut wlr_surface,
                                           mut sx: libc::c_double,
                                           mut sy: libc::c_double) {
    if (*point).focus_surface == surface { return }
    touch_point_clear_focus(point);
    if !surface.is_null() && !(*surface).resource.is_null() {
        let mut client: *mut wlr_seat_client =
            wlr_seat_client_for_wl_client((*(*point).client).seat,
                                          wl_resource_get_client((*surface).resource));
        if !client.is_null() && wl_list_empty(&mut (*client).touches) == 0 {
            wl_signal_add(&mut (*surface).events.destroy,
                          &mut (*point).focus_surface_destroy);
            (*point).focus_surface_destroy.notify =
                Some(handle_point_focus_destroy as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            (*point).focus_surface = surface;
            (*point).focus_client = client;
            (*point).sx = sx;
            (*point).sy = sy
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_point_focus(mut seat: *mut wlr_seat,
                                                    mut surface:
                                                        *mut wlr_surface,
                                                    mut time: uint32_t,
                                                    mut touch_id: int32_t,
                                                    mut sx: libc::c_double,
                                                    mut sy: libc::c_double) {
    if !surface.is_null() {
    } else {
        __assert_fail(b"surface\x00" as *const u8 as *const libc::c_char,
                      b"../types/seat/wlr_seat_touch.c\x00" as *const u8 as
                          *const libc::c_char, 263i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 108],
                                                &[libc::c_char; 108]>(b"void wlr_seat_touch_point_focus(struct wlr_seat *, struct wlr_surface *, uint32_t, int32_t, double, double)\x00")).as_ptr());
    };
    let mut point: *mut wlr_touch_point =
        wlr_seat_touch_get_point(seat, touch_id);
    if point.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] got touch point focus for unknown touch point\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat_touch.c\x00" as *const u8 as
                     *const libc::c_char, 266i32);
        return
    }
    let mut focus: *mut wlr_surface = (*point).focus_surface;
    touch_point_set_focus(point, surface, sx, sy);
    if focus != (*point).focus_surface {
        let mut grab: *mut wlr_seat_touch_grab = (*seat).touch_state.grab;
        (*(*grab).interface).enter.expect("non-null function pointer")(grab,
                                                                       time,
                                                                       point);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_point_clear_focus(mut seat:
                                                              *mut wlr_seat,
                                                          mut time: uint32_t,
                                                          mut touch_id:
                                                              int32_t) {
    let mut point: *mut wlr_touch_point =
        wlr_seat_touch_get_point(seat, touch_id);
    if point.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] got touch point focus for unknown touch point\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat_touch.c\x00" as *const u8 as
                     *const libc::c_char, 282i32);
        return
    }
    touch_point_clear_focus(point);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_send_down(mut seat: *mut wlr_seat,
                                                  mut surface:
                                                      *mut wlr_surface,
                                                  mut time: uint32_t,
                                                  mut touch_id: int32_t,
                                                  mut sx: libc::c_double,
                                                  mut sy: libc::c_double)
 -> uint32_t {
    let mut point: *mut wlr_touch_point =
        wlr_seat_touch_get_point(seat, touch_id);
    if point.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] got touch down for unknown touch point\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat_touch.c\x00" as *const u8 as
                     *const libc::c_char, 294i32);
        return 0i32 as uint32_t
    }
    let mut serial: uint32_t = wlr_seat_client_next_serial((*point).client);
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*(*point).client).touches.next);
    while wl_resource_get_link(resource) !=
              &mut (*(*point).client).touches as *mut wl_list {
        if !seat_client_from_touch_resource(resource).is_null() {
            wl_touch_send_down(resource, serial, time, (*surface).resource,
                               touch_id, wl_fixed_from_double(sx),
                               wl_fixed_from_double(sy));
            wl_touch_send_frame(resource);
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    return serial;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_send_up(mut seat: *mut wlr_seat,
                                                mut time: uint32_t,
                                                mut touch_id: int32_t) {
    let mut point: *mut wlr_touch_point =
        wlr_seat_touch_get_point(seat, touch_id);
    if point.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] got touch up for unknown touch point\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat_touch.c\x00" as *const u8 as
                     *const libc::c_char, 315i32);
        return
    }
    let mut serial: uint32_t = wlr_seat_client_next_serial((*point).client);
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*(*point).client).touches.next);
    while wl_resource_get_link(resource) !=
              &mut (*(*point).client).touches as *mut wl_list {
        if !seat_client_from_touch_resource(resource).is_null() {
            wl_touch_send_up(resource, serial, time, touch_id);
            wl_touch_send_frame(resource);
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_send_motion(mut seat: *mut wlr_seat,
                                                    mut time: uint32_t,
                                                    mut touch_id: int32_t,
                                                    mut sx: libc::c_double,
                                                    mut sy: libc::c_double) {
    let mut point: *mut wlr_touch_point =
        wlr_seat_touch_get_point(seat, touch_id);
    if point.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] got touch motion for unknown touch point\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat_touch.c\x00" as *const u8 as
                     *const libc::c_char, 334i32);
        return
    }
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*(*point).client).touches.next);
    while wl_resource_get_link(resource) !=
              &mut (*(*point).client).touches as *mut wl_list {
        if !seat_client_from_touch_resource(resource).is_null() {
            wl_touch_send_motion(resource, time, touch_id,
                                 wl_fixed_from_double(sx),
                                 wl_fixed_from_double(sy));
            wl_touch_send_frame(resource);
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_num_points(mut seat: *mut wlr_seat)
 -> libc::c_int {
    return wl_list_length(&mut (*seat).touch_state.touch_points);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_touch_has_grab(mut seat: *mut wlr_seat)
 -> bool {
    return (*(*seat).touch_state.grab).interface !=
               &default_touch_grab_impl as *const wlr_touch_grab_interface;
}
#[no_mangle]
pub unsafe extern "C" fn seat_client_create_touch(mut seat_client:
                                                      *mut wlr_seat_client,
                                                  mut version: uint32_t,
                                                  mut id: uint32_t) {
    let mut resource: *mut wl_resource =
        wl_resource_create((*seat_client).client, &wl_touch_interface,
                           version as libc::c_int, id);
    if resource.is_null() {
        wl_client_post_no_memory((*seat_client).client);
        return
    }
    wl_resource_set_implementation(resource,
                                   &touch_impl as *const wl_touch_interface as
                                       *const libc::c_void,
                                   seat_client as *mut libc::c_void,
                                   Some(touch_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_insert(&mut (*seat_client).touches,
                   wl_resource_get_link(resource));
}
#[no_mangle]
pub unsafe extern "C" fn seat_client_destroy_touch(mut resource:
                                                       *mut wl_resource) {
    let mut seat_client: *mut wlr_seat_client =
        seat_client_from_touch_resource(resource);
    if seat_client.is_null() { return }
    wl_resource_set_user_data(resource, 0 as *mut libc::c_void);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * Contains state for a single client's bound wl_seat resource and can be used
 * to issue input events to that client. The lifetime of these objects is
 * managed by wlr_seat; some may be NULL.
 */
// lists of wl_resource
// set of serials which were sent to the client on this seat
	// for use by wlr_seat_client_{next_serial,validate_event_serial}
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
// wlr_touch_point::link
// wlr_data_offer::link
// `drag` goes away before `drag_source`, when the implicit grab ends
// wlr_data_offer::link
// wlr_seat_pointer_request_set_cursor_event
// wlr_seat_request_set_selection_event
// wlr_seat_request_set_primary_selection_event
// wlr_seat_request_start_drag_event
// wlr_drag
/* *
 * Allocates a new wlr_seat and adds a wl_seat global to the display.
 */
/* *
 * Destroys a wlr_seat and removes its wl_seat global.
 */
/* *
 * Gets a wlr_seat_client for the specified client, or returns NULL if no
 * client is bound for that client.
 */
/* *
 * Updates the capabilities available on this seat.
 * Will automatically send them to all clients.
 */
/* *
 * Updates the name of this seat.
 * Will automatically send it to all clients.
 */
/* *
 * Whether or not the surface has pointer focus
 */
/* *
 * Send a pointer enter event to the given surface and consider it to be the
 * focused surface for the pointer. This will send a leave event to the last
 * surface that was entered. Coordinates for the enter event are surface-local.
 * Compositor should use `wlr_seat_pointer_notify_enter()` to change pointer
 * focus to respect pointer grabs.
 */
/* *
 * Clear the focused surface for the pointer and leave all entered surfaces.
 */
/* *
 * Send a motion event to the surface with pointer focus. Coordinates for the
 * motion event are surface-local. Compositors should use
 * `wlr_seat_pointer_notify_motion()` to send motion events to respect pointer
 * grabs.
 */
/* *
 * Send a button event to the surface with pointer focus. Coordinates for the
 * button event are surface-local. Returns the serial. Compositors should use
 * `wlr_seat_pointer_notify_button()` to send button events to respect pointer
 * grabs.
 */
/* *
 * Send an axis event to the surface with pointer focus. Compositors should use
 * `wlr_seat_pointer_notify_axis()` to send axis events to respect pointer
 * grabs.
 **/
/* *
 * Send a frame event to the surface with pointer focus. Compositors should use
 * `wlr_seat_pointer_notify_frame()` to send axis events to respect pointer
 * grabs.
 */
/* *
 * Start a grab of the pointer of this seat. The grabber is responsible for
 * handling all pointer events until the grab ends.
 */
/* *
 * End the grab of the pointer of this seat. This reverts the grab back to the
 * default grab for the pointer.
 */
/* *
 * Notify the seat of a pointer enter event to the given surface and request it
 * to be the focused surface for the pointer. Pass surface-local coordinates
 * where the enter occurred.
 */
/* *
 * Notify the seat of motion over the given surface. Pass surface-local
 * coordinates where the pointer motion occurred.
 */
/* *
 * Notify the seat that a button has been pressed. Returns the serial of the
 * button press or zero if no button press was sent.
 */
/* *
 * Notify the seat of an axis event.
 */
/* *
 * Notify the seat of a frame event. Frame events are sent to end a group of
 * events that logically belong together. Motion, button and axis events should
 * all be followed by a frame event.
 */
/* *
 * Whether or not the pointer has a grab other than the default grab.
 */
/* *
 * Set this keyboard as the active keyboard for the seat.
 */
/* *
 * Get the active keyboard for the seat.
 */
/* *
 * Start a grab of the keyboard of this seat. The grabber is responsible for
 * handling all keyboard events until the grab ends.
 */
/* *
 * End the grab of the keyboard of this seat. This reverts the grab back to the
 * default grab for the keyboard.
 */
/* *
 * Send the keyboard key to focused keyboard resources. Compositors should use
 * `wlr_seat_notify_key()` to respect keyboard grabs.
 */
/* *
 * Notify the seat that a key has been pressed on the keyboard. Defers to any
 * keyboard grabs.
 */
/* *
 * Send the modifier state to focused keyboard resources. Compositors should use
 * `wlr_seat_keyboard_notify_modifiers()` to respect any keyboard grabs.
 */
/* *
 * Notify the seat that the modifiers for the keyboard have changed. Defers to
 * any keyboard grabs.
 */
/* *
 * Notify the seat that the keyboard focus has changed and request it to be the
 * focused surface for this keyboard. Defers to any current grab of the seat's
 * keyboard.
 */
/* *
 * Send a keyboard enter event to the given surface and consider it to be the
 * focused surface for the keyboard. This will send a leave event to the last
 * surface that was entered. Compositors should use
 * `wlr_seat_keyboard_notify_enter()` to change keyboard focus to respect
 * keyboard grabs.
 */
/* *
 * Clear the focused surface for the keyboard and leave all entered surfaces.
 */
/* *
 * Whether or not the keyboard has a grab other than the default grab
 */
/* *
 * Start a grab of the touch device of this seat. The grabber is responsible for
 * handling all touch events until the grab ends.
 */
/* *
 * End the grab of the touch device of this seat. This reverts the grab back to
 * the default grab for the touch device.
 */
/* *
 * Get the active touch point with the given `touch_id`. If the touch point does
 * not exist or is no longer active, returns NULL.
 */
/* *
 * Notify the seat of a touch down on the given surface. Defers to any grab of
 * the touch device.
 */
/* *
 * Notify the seat that the touch point given by `touch_id` is up. Defers to any
 * grab of the touch device.
 */
/* *
 * Notify the seat that the touch point given by `touch_id` has moved. Defers to
 * any grab of the touch device. The seat should be notified of touch motion
 * even if the surface is not the owner of the touch point for processing by
 * grabs.
 */
/* *
 * Notify the seat that the touch point given by `touch_id` has entered a new
 * surface. The surface is required. To clear focus, use
 * `wlr_seat_touch_point_clear_focus()`.
 */
/* *
 * Clear the focused surface for the touch point given by `touch_id`.
 */
/* *
 * Send a touch down event to the client of the given surface. All future touch
 * events for this point will go to this surface. If the touch down is valid,
 * this will add a new touch point with the given `touch_id`. The touch down may
 * not be valid if the surface seat client does not accept touch input.
 * Coordinates are surface-local. Compositors should use
 * `wlr_seat_touch_notify_down()` to respect any grabs of the touch device.
 */
/* *
 * Send a touch up event for the touch point given by the `touch_id`. The event
 * will go to the client for the surface given in the corresponding touch down
 * event. This will remove the touch point. Compositors should use
 * `wlr_seat_touch_notify_up()` to respect any grabs of the touch device.
 */
/* *
 * Send a touch motion event for the touch point given by the `touch_id`. The
 * event will go to the client for the surface given in the corresponding touch
 * down event. Compositors should use `wlr_seat_touch_notify_motion()` to
 * respect any grabs of the touch device.
 */
/* *
 * How many touch points are currently down for the seat.
 */
/* *
 * Whether or not the seat has a touch grab other than the default grab.
 */
/* *
 * Check whether this serial is valid to start a grab action such as an
 * interactive move or resize.
 */
/* *
 * Check whether this serial is valid to start a pointer grab action.
 */
/* *
 * Check whether this serial is valid to start a touch grab action. If it's the
 * case and point_ptr is non-NULL, *point_ptr is set to the touch point matching
 * the serial.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_validate_touch_grab_serial(mut seat:
                                                                 *mut wlr_seat,
                                                             mut origin:
                                                                 *mut wlr_surface,
                                                             mut serial:
                                                                 uint32_t,
                                                             mut point_ptr:
                                                                 *mut *mut wlr_touch_point)
 -> bool {
    if wlr_seat_touch_num_points(seat) != 1i32 ||
           (*seat).touch_state.grab_serial != serial {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Touch grab serial validation failed: num_points=%d grab_serial=%u (got %u)\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat_touch.c\x00" as *const u8 as
                     *const libc::c_char, 388i32,
                 wlr_seat_touch_num_points(seat),
                 (*seat).touch_state.grab_serial, serial);
        return 0i32 != 0
    }
    let mut point: *mut wlr_touch_point = 0 as *mut wlr_touch_point;
    point =
        ((*seat).touch_state.touch_points.next as
             *mut libc::c_char).offset(-144) as *mut wlr_touch_point;
    while &mut (*point).link as *mut wl_list !=
              &mut (*seat).touch_state.touch_points as *mut wl_list {
        if origin.is_null() || (*point).surface == origin {
            if !point_ptr.is_null() { *point_ptr = point }
            return 1i32 != 0
        }
        point =
            ((*point).link.next as *mut libc::c_char).offset(-144) as
                *mut wlr_touch_point
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Touch grab serial validation failed: invalid origin surface\x00"
                 as *const u8 as *const libc::c_char,
             b"../types/seat/wlr_seat_touch.c\x00" as *const u8 as
                 *const libc::c_char, 403i32);
    return 0i32 != 0;
}
