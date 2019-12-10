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
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
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
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
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
    fn wl_resource_get_version(resource: *mut wl_resource) -> libc::c_int;
    #[no_mangle]
    fn wl_resource_instance_of(resource: *mut wl_resource,
                               interface: *const wl_interface,
                               implementation: *const libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    static wl_pointer_interface: wl_interface;
    /* *
 * Set the lifetime role for this surface. Returns 0 on success or -1 if the
 * role cannot be set.
 */
    #[no_mangle]
    fn wlr_surface_set_role(surface: *mut wlr_surface,
                            role: *const wlr_surface_role,
                            role_data: *mut libc::c_void,
                            error_resource: *mut wl_resource,
                            error_code: uint32_t) -> bool;
    #[no_mangle]
    fn wlr_surface_from_resource(resource: *mut wl_resource)
     -> *mut wlr_surface;
    #[no_mangle]
    fn wlr_seat_client_for_wl_client(wlr_seat: *mut wlr_seat,
                                     wl_client: *mut wl_client)
     -> *mut wlr_seat_client;
    #[no_mangle]
    fn wlr_seat_client_next_serial(client: *mut wlr_seat_client) -> uint32_t;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
    /* *
 * Add `target` to `values` if it doesn't exist
 * "set"s should only be modified with set_* functions
 * Values MUST be greater than 0
 */
    #[no_mangle]
    fn set_add(values: *mut uint32_t, len: *mut size_t, cap: size_t,
               target: uint32_t) -> bool;
    /* *
 * Remove `target` from `values` if it exists
 * "set"s should only be modified with set_* functions
 * Values MUST be greater than 0
 */
    #[no_mangle]
    fn set_remove(values: *mut uint32_t, len: *mut size_t, cap: size_t,
                  target: uint32_t) -> bool;
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
pub type wl_pointer_error = libc::c_uint;
pub const WL_POINTER_ERROR_ROLE: wl_pointer_error = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_pointer_interface {
    pub set_cursor: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                _: *mut wl_resource,
                                                _: uint32_t,
                                                _: *mut wl_resource,
                                                _: int32_t, _: int32_t)
                               -> ()>,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat_pointer_request_set_cursor_event {
    pub seat_client: *mut wlr_seat_client,
    pub surface: *mut wlr_surface,
    pub serial: uint32_t,
    pub hotspot_x: int32_t,
    pub hotspot_y: int32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat_pointer_focus_change_event {
    pub seat: *mut wlr_seat,
    pub old_surface: *mut wlr_surface,
    pub new_surface: *mut wlr_surface,
    pub sx: libc::c_double,
    pub sy: libc::c_double,
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
#[inline]
unsafe extern "C" fn wl_pointer_send_enter(mut resource_: *mut wl_resource,
                                           mut serial: uint32_t,
                                           mut surface: *mut wl_resource,
                                           mut surface_x: wl_fixed_t,
                                           mut surface_y: wl_fixed_t) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, serial, surface,
                           surface_x, surface_y);
}
#[inline]
unsafe extern "C" fn wl_pointer_send_leave(mut resource_: *mut wl_resource,
                                           mut serial: uint32_t,
                                           mut surface: *mut wl_resource) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, serial, surface);
}
#[inline]
unsafe extern "C" fn wl_pointer_send_motion(mut resource_: *mut wl_resource,
                                            mut time: uint32_t,
                                            mut surface_x: wl_fixed_t,
                                            mut surface_y: wl_fixed_t) {
    wl_resource_post_event(resource_, 2i32 as uint32_t, time, surface_x,
                           surface_y);
}
#[inline]
unsafe extern "C" fn wl_pointer_send_button(mut resource_: *mut wl_resource,
                                            mut serial: uint32_t,
                                            mut time: uint32_t,
                                            mut button: uint32_t,
                                            mut state: uint32_t) {
    wl_resource_post_event(resource_, 3i32 as uint32_t, serial, time, button,
                           state);
}
#[inline]
unsafe extern "C" fn wl_pointer_send_axis(mut resource_: *mut wl_resource,
                                          mut time: uint32_t,
                                          mut axis: uint32_t,
                                          mut value: wl_fixed_t) {
    wl_resource_post_event(resource_, 4i32 as uint32_t, time, axis, value);
}
#[inline]
unsafe extern "C" fn wl_pointer_send_frame(mut resource_: *mut wl_resource) {
    wl_resource_post_event(resource_, 5i32 as uint32_t);
}
#[inline]
unsafe extern "C" fn wl_pointer_send_axis_source(mut resource_:
                                                     *mut wl_resource,
                                                 mut axis_source: uint32_t) {
    wl_resource_post_event(resource_, 6i32 as uint32_t, axis_source);
}
#[inline]
unsafe extern "C" fn wl_pointer_send_axis_stop(mut resource_:
                                                   *mut wl_resource,
                                               mut time: uint32_t,
                                               mut axis: uint32_t) {
    wl_resource_post_event(resource_, 7i32 as uint32_t, time, axis);
}
#[inline]
unsafe extern "C" fn wl_pointer_send_axis_discrete(mut resource_:
                                                       *mut wl_resource,
                                                   mut axis: uint32_t,
                                                   mut discrete: int32_t) {
    wl_resource_post_event(resource_, 8i32 as uint32_t, axis, discrete);
}
#[no_mangle]
pub static mut default_touch_grab_impl: wlr_touch_grab_interface =
    wlr_touch_grab_interface{down: None,
                             up: None,
                             motion: None,
                             enter: None,
                             cancel: None,};
#[no_mangle]
pub static mut default_keyboard_grab_impl: wlr_keyboard_grab_interface =
    wlr_keyboard_grab_interface{enter: None,
                                key: None,
                                modifiers: None,
                                cancel: None,};
unsafe extern "C" fn default_pointer_enter(mut grab:
                                               *mut wlr_seat_pointer_grab,
                                           mut surface: *mut wlr_surface,
                                           mut sx: libc::c_double,
                                           mut sy: libc::c_double) {
    wlr_seat_pointer_enter((*grab).seat, surface, sx, sy);
}
unsafe extern "C" fn default_pointer_motion(mut grab:
                                                *mut wlr_seat_pointer_grab,
                                            mut time: uint32_t,
                                            mut sx: libc::c_double,
                                            mut sy: libc::c_double) {
    wlr_seat_pointer_send_motion((*grab).seat, time, sx, sy);
}
unsafe extern "C" fn default_pointer_button(mut grab:
                                                *mut wlr_seat_pointer_grab,
                                            mut time: uint32_t,
                                            mut button: uint32_t,
                                            mut state: wlr_button_state)
 -> uint32_t {
    return wlr_seat_pointer_send_button((*grab).seat, time, button, state);
}
unsafe extern "C" fn default_pointer_axis(mut grab:
                                              *mut wlr_seat_pointer_grab,
                                          mut time: uint32_t,
                                          mut orientation:
                                              wlr_axis_orientation,
                                          mut value: libc::c_double,
                                          mut value_discrete: int32_t,
                                          mut source: wlr_axis_source) {
    wlr_seat_pointer_send_axis((*grab).seat, time, orientation, value,
                               value_discrete, source);
}
unsafe extern "C" fn default_pointer_frame(mut grab:
                                               *mut wlr_seat_pointer_grab) {
    wlr_seat_pointer_send_frame((*grab).seat);
}
unsafe extern "C" fn default_pointer_cancel(mut grab:
                                                *mut wlr_seat_pointer_grab) {
    // cannot be cancelled
}
#[no_mangle]
pub static mut default_pointer_grab_impl: wlr_pointer_grab_interface =
    unsafe {
        {
            let mut init =
                wlr_pointer_grab_interface{enter:
                                               Some(default_pointer_enter as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wlr_seat_pointer_grab,
                                                                             _:
                                                                                 *mut wlr_surface,
                                                                             _:
                                                                                 libc::c_double,
                                                                             _:
                                                                                 libc::c_double)
                                                            -> ()),
                                           motion:
                                               Some(default_pointer_motion as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wlr_seat_pointer_grab,
                                                                             _:
                                                                                 uint32_t,
                                                                             _:
                                                                                 libc::c_double,
                                                                             _:
                                                                                 libc::c_double)
                                                            -> ()),
                                           button:
                                               Some(default_pointer_button as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wlr_seat_pointer_grab,
                                                                             _:
                                                                                 uint32_t,
                                                                             _:
                                                                                 uint32_t,
                                                                             _:
                                                                                 wlr_button_state)
                                                            -> uint32_t),
                                           axis:
                                               Some(default_pointer_axis as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wlr_seat_pointer_grab,
                                                                             _:
                                                                                 uint32_t,
                                                                             _:
                                                                                 wlr_axis_orientation,
                                                                             _:
                                                                                 libc::c_double,
                                                                             _:
                                                                                 int32_t,
                                                                             _:
                                                                                 wlr_axis_source)
                                                            -> ()),
                                           frame:
                                               Some(default_pointer_frame as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wlr_seat_pointer_grab)
                                                            -> ()),
                                           cancel:
                                               Some(default_pointer_cancel as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wlr_seat_pointer_grab)
                                                            -> ()),};
            init
        }
    };
unsafe extern "C" fn pointer_send_frame(mut resource: *mut wl_resource) {
    if wl_resource_get_version(resource) >= 5i32 {
        wl_pointer_send_frame(resource);
    };
}
/* *
 * Check whether this serial is valid to start a touch grab action. If it's the
 * case and point_ptr is non-NULL, *point_ptr is set to the touch point matching
 * the serial.
 */
/* *
 * Return a new serial (from wl_display_serial_next()) for the client, and
 * update the seat client's set of valid serials. Use this for all input
 * events; otherwise wlr_seat_client_validate_event_serial() may fail when
 * handed a correctly functioning client's request serials.
 */
/* *
 * Return true if the serial number could have been produced by
 * wlr_seat_client_next_serial() and is "older" (by less than UINT32_MAX/2) than
 * the current display serial value.
 *
 * This function should have no false negatives, and the only false positive
 * responses allowed are for elements that are still "older" than the current
 * display serial value and also older than all serial values remaining in
 * the seat client's serial ring buffer, if that buffer is also full.
 */
/* *
 * Get a seat client from a seat resource. Returns NULL if inert.
 */
/* *
 * Get a seat client from a pointer resource. Returns NULL if inert.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_client_from_pointer_resource(mut resource:
                                                                   *mut wl_resource)
 -> *mut wlr_seat_client {
    if wl_resource_instance_of(resource, &wl_pointer_interface,
                               &pointer_impl as *const wl_pointer_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_pointer_interface, &pointer_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/seat/wlr_seat_pointer.c\x00" as *const u8 as
                          *const libc::c_char, 65i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[libc::c_char; 84]>(b"struct wlr_seat_client *wlr_seat_client_from_pointer_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_seat_client;
}
static mut pointer_cursor_surface_role: wlr_surface_role =
    {
        let mut init =
            wlr_surface_role{name:
                                 b"wl_pointer-cursor\x00" as *const u8 as
                                     *const libc::c_char,
                             commit: None,
                             precommit: None,};
        init
    };
unsafe extern "C" fn pointer_set_cursor(mut client: *mut wl_client,
                                        mut pointer_resource:
                                            *mut wl_resource,
                                        mut serial: uint32_t,
                                        mut surface_resource:
                                            *mut wl_resource,
                                        mut hotspot_x: int32_t,
                                        mut hotspot_y: int32_t) {
    let mut seat_client: *mut wlr_seat_client =
        wlr_seat_client_from_pointer_resource(pointer_resource);
    if seat_client.is_null() { return }
    let mut surface: *mut wlr_surface = 0 as *mut wlr_surface;
    if !surface_resource.is_null() {
        surface = wlr_surface_from_resource(surface_resource);
        if !wlr_surface_set_role(surface, &pointer_cursor_surface_role,
                                 0 as *mut libc::c_void, surface_resource,
                                 WL_POINTER_ERROR_ROLE as libc::c_int as
                                     uint32_t) {
            return
        }
    }
    let mut event: wlr_seat_pointer_request_set_cursor_event =
        {
            let mut init =
                wlr_seat_pointer_request_set_cursor_event{seat_client:
                                                              seat_client,
                                                          surface: surface,
                                                          serial: serial,
                                                          hotspot_x:
                                                              hotspot_x,
                                                          hotspot_y:
                                                              hotspot_y,};
            init
        };
    wlr_signal_emit_safe(&mut (*(*seat_client).seat).events.request_set_cursor,
                         &mut event as
                             *mut wlr_seat_pointer_request_set_cursor_event as
                             *mut libc::c_void);
}
unsafe extern "C" fn pointer_release(mut client: *mut wl_client,
                                     mut resource: *mut wl_resource) {
    wl_resource_destroy(resource);
}
static mut pointer_impl: wl_pointer_interface =
    unsafe {
        {
            let mut init =
                wl_pointer_interface{set_cursor:
                                         Some(pointer_set_cursor as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           uint32_t,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           int32_t,
                                                                       _:
                                                                           int32_t)
                                                      -> ()),
                                     release:
                                         Some(pointer_release as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource)
                                                      -> ()),};
            init
        }
    };
unsafe extern "C" fn pointer_handle_resource_destroy(mut resource:
                                                         *mut wl_resource) {
    wl_list_remove(wl_resource_get_link(resource));
    seat_client_destroy_pointer(resource);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_surface_has_focus(mut wlr_seat:
                                                                *mut wlr_seat,
                                                            mut surface:
                                                                *mut wlr_surface)
 -> bool {
    return surface == (*wlr_seat).pointer_state.focused_surface;
}
unsafe extern "C" fn seat_pointer_handle_surface_destroy(mut listener:
                                                             *mut wl_listener,
                                                         mut data:
                                                             *mut libc::c_void) {
    let mut state: *mut wlr_seat_pointer_state =
        (listener as *mut libc::c_char).offset(-144) as
            *mut wlr_seat_pointer_state;
    wl_list_remove(&mut (*state).surface_destroy.link);
    wl_list_init(&mut (*state).surface_destroy.link);
    wlr_seat_pointer_clear_focus((*state).seat);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_enter(mut wlr_seat: *mut wlr_seat,
                                                mut surface: *mut wlr_surface,
                                                mut sx: libc::c_double,
                                                mut sy: libc::c_double) {
    if (*wlr_seat).pointer_state.focused_surface == surface {
        // this surface already got an enter notify
        return
    }
    let mut client: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
    if !surface.is_null() {
        let mut wl_client: *mut wl_client =
            wl_resource_get_client((*surface).resource);
        client = wlr_seat_client_for_wl_client(wlr_seat, wl_client)
    }
    let mut focused_client: *mut wlr_seat_client =
        (*wlr_seat).pointer_state.focused_client;
    let mut focused_surface: *mut wlr_surface =
        (*wlr_seat).pointer_state.focused_surface;
    // leave the previously entered surface
    if !focused_client.is_null() && !focused_surface.is_null() {
        let mut serial: uint32_t =
            wlr_seat_client_next_serial(focused_client);
        let mut resource: *mut wl_resource = 0 as *mut wl_resource;
        resource = 0 as *mut wl_resource;
        resource = wl_resource_from_link((*focused_client).pointers.next);
        while wl_resource_get_link(resource) !=
                  &mut (*focused_client).pointers as *mut wl_list {
            if !wlr_seat_client_from_pointer_resource(resource).is_null() {
                wl_pointer_send_leave(resource, serial,
                                      (*focused_surface).resource);
                pointer_send_frame(resource);
            }
            resource =
                wl_resource_from_link((*wl_resource_get_link(resource)).next)
        }
    }
    // enter the current surface
    if !client.is_null() && !surface.is_null() {
        let mut serial_0: uint32_t = wlr_seat_client_next_serial(client);
        let mut resource_0: *mut wl_resource = 0 as *mut wl_resource;
        resource_0 = 0 as *mut wl_resource;
        resource_0 = wl_resource_from_link((*client).pointers.next);
        while wl_resource_get_link(resource_0) !=
                  &mut (*client).pointers as *mut wl_list {
            if !wlr_seat_client_from_pointer_resource(resource_0).is_null() {
                wl_pointer_send_enter(resource_0, serial_0,
                                      (*surface).resource,
                                      wl_fixed_from_double(sx),
                                      wl_fixed_from_double(sy));
                pointer_send_frame(resource_0);
            }
            resource_0 =
                wl_resource_from_link((*wl_resource_get_link(resource_0)).next)
        }
    }
    // reinitialize the focus destroy events
    wl_list_remove(&mut (*wlr_seat).pointer_state.surface_destroy.link);
    wl_list_init(&mut (*wlr_seat).pointer_state.surface_destroy.link);
    if !surface.is_null() {
        wl_signal_add(&mut (*surface).events.destroy,
                      &mut (*wlr_seat).pointer_state.surface_destroy);
        (*wlr_seat).pointer_state.surface_destroy.notify =
            Some(seat_pointer_handle_surface_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ())
    }
    (*wlr_seat).pointer_state.focused_client = client;
    (*wlr_seat).pointer_state.focused_surface = surface;
    if !surface.is_null() {
        (*wlr_seat).pointer_state.sx = sx;
        (*wlr_seat).pointer_state.sy = sy
    } else {
        (*wlr_seat).pointer_state.sx = ::std::f32::NAN as libc::c_double;
        (*wlr_seat).pointer_state.sy = ::std::f32::NAN as libc::c_double
    }
    let mut event: wlr_seat_pointer_focus_change_event =
        {
            let mut init =
                wlr_seat_pointer_focus_change_event{seat: wlr_seat,
                                                    old_surface:
                                                        focused_surface,
                                                    new_surface: surface,
                                                    sx: sx,
                                                    sy: sy,};
            init
        };
    wlr_signal_emit_safe(&mut (*wlr_seat).pointer_state.events.focus_change,
                         &mut event as
                             *mut wlr_seat_pointer_focus_change_event as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_clear_focus(mut wlr_seat:
                                                          *mut wlr_seat) {
    wlr_seat_pointer_enter(wlr_seat, 0 as *mut wlr_surface,
                           0i32 as libc::c_double, 0i32 as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_send_motion(mut wlr_seat:
                                                          *mut wlr_seat,
                                                      mut time: uint32_t,
                                                      mut sx: libc::c_double,
                                                      mut sy:
                                                          libc::c_double) {
    let mut client: *mut wlr_seat_client =
        (*wlr_seat).pointer_state.focused_client;
    if client.is_null() { return }
    if (*wlr_seat).pointer_state.sx == sx &&
           (*wlr_seat).pointer_state.sy == sy {
        return
    }
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).pointers.next);
    while wl_resource_get_link(resource) !=
              &mut (*client).pointers as *mut wl_list {
        if !wlr_seat_client_from_pointer_resource(resource).is_null() {
            wl_pointer_send_motion(resource, time, wl_fixed_from_double(sx),
                                   wl_fixed_from_double(sy));
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    (*wlr_seat).pointer_state.sx = sx;
    (*wlr_seat).pointer_state.sy = sy;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_send_button(mut wlr_seat:
                                                          *mut wlr_seat,
                                                      mut time: uint32_t,
                                                      mut button: uint32_t,
                                                      mut state:
                                                          wlr_button_state)
 -> uint32_t {
    let mut client: *mut wlr_seat_client =
        (*wlr_seat).pointer_state.focused_client;
    if client.is_null() { return 0i32 as uint32_t }
    let mut serial: uint32_t = wlr_seat_client_next_serial(client);
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).pointers.next);
    while wl_resource_get_link(resource) !=
              &mut (*client).pointers as *mut wl_list {
        if !wlr_seat_client_from_pointer_resource(resource).is_null() {
            wl_pointer_send_button(resource, serial, time, button,
                                   state as uint32_t);
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    return serial;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_send_axis(mut wlr_seat:
                                                        *mut wlr_seat,
                                                    mut time: uint32_t,
                                                    mut orientation:
                                                        wlr_axis_orientation,
                                                    mut value: libc::c_double,
                                                    mut value_discrete:
                                                        int32_t,
                                                    mut source:
                                                        wlr_axis_source) {
    let mut client: *mut wlr_seat_client =
        (*wlr_seat).pointer_state.focused_client;
    if client.is_null() { return }
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).pointers.next);
    while wl_resource_get_link(resource) !=
              &mut (*client).pointers as *mut wl_list {
        if !wlr_seat_client_from_pointer_resource(resource).is_null() {
            let mut version: uint32_t =
                wl_resource_get_version(resource) as uint32_t;
            if version >= 5i32 as libc::c_uint {
                wl_pointer_send_axis_source(resource, source as uint32_t);
            }
            if value != 0. {
                if value_discrete != 0 && version >= 5i32 as libc::c_uint {
                    wl_pointer_send_axis_discrete(resource,
                                                  orientation as uint32_t,
                                                  value_discrete);
                }
                wl_pointer_send_axis(resource, time, orientation as uint32_t,
                                     wl_fixed_from_double(value));
            } else if version >= 5i32 as libc::c_uint {
                wl_pointer_send_axis_stop(resource, time,
                                          orientation as uint32_t);
            }
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_send_frame(mut wlr_seat:
                                                         *mut wlr_seat) {
    let mut client: *mut wlr_seat_client =
        (*wlr_seat).pointer_state.focused_client;
    if client.is_null() { return }
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).pointers.next);
    while wl_resource_get_link(resource) !=
              &mut (*client).pointers as *mut wl_list {
        if !wlr_seat_client_from_pointer_resource(resource).is_null() {
            pointer_send_frame(resource);
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_start_grab(mut wlr_seat:
                                                         *mut wlr_seat,
                                                     mut grab:
                                                         *mut wlr_seat_pointer_grab) {
    if !wlr_seat.is_null() {
    } else {
        __assert_fail(b"wlr_seat\x00" as *const u8 as *const libc::c_char,
                      b"../types/seat/wlr_seat_pointer.c\x00" as *const u8 as
                          *const libc::c_char, 309i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[libc::c_char; 84]>(b"void wlr_seat_pointer_start_grab(struct wlr_seat *, struct wlr_seat_pointer_grab *)\x00")).as_ptr());
    };
    (*grab).seat = wlr_seat;
    (*wlr_seat).pointer_state.grab = grab;
    wlr_signal_emit_safe(&mut (*wlr_seat).events.pointer_grab_begin,
                         grab as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_end_grab(mut wlr_seat:
                                                       *mut wlr_seat) {
    let mut grab: *mut wlr_seat_pointer_grab = (*wlr_seat).pointer_state.grab;
    if grab != (*wlr_seat).pointer_state.default_grab {
        (*wlr_seat).pointer_state.grab =
            (*wlr_seat).pointer_state.default_grab;
        wlr_signal_emit_safe(&mut (*wlr_seat).events.pointer_grab_end,
                             grab as *mut libc::c_void);
        if (*(*grab).interface).cancel.is_some() {
            (*(*grab).interface).cancel.expect("non-null function pointer")(grab);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_notify_enter(mut wlr_seat:
                                                           *mut wlr_seat,
                                                       mut surface:
                                                           *mut wlr_surface,
                                                       mut sx: libc::c_double,
                                                       mut sy:
                                                           libc::c_double) {
    let mut grab: *mut wlr_seat_pointer_grab = (*wlr_seat).pointer_state.grab;
    (*(*grab).interface).enter.expect("non-null function pointer")(grab,
                                                                   surface,
                                                                   sx, sy);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_notify_motion(mut wlr_seat:
                                                            *mut wlr_seat,
                                                        mut time: uint32_t,
                                                        mut sx:
                                                            libc::c_double,
                                                        mut sy:
                                                            libc::c_double) {
    clock_gettime(1i32, &mut (*wlr_seat).last_event);
    let mut grab: *mut wlr_seat_pointer_grab = (*wlr_seat).pointer_state.grab;
    (*(*grab).interface).motion.expect("non-null function pointer")(grab,
                                                                    time, sx,
                                                                    sy);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_notify_button(mut wlr_seat:
                                                            *mut wlr_seat,
                                                        mut time: uint32_t,
                                                        mut button: uint32_t,
                                                        mut state:
                                                            wlr_button_state)
 -> uint32_t {
    clock_gettime(1i32, &mut (*wlr_seat).last_event);
    let mut pointer_state: *mut wlr_seat_pointer_state =
        &mut (*wlr_seat).pointer_state;
    if state as libc::c_uint ==
           WLR_BUTTON_PRESSED as libc::c_int as libc::c_uint {
        if (*pointer_state).button_count == 0i32 as libc::c_ulong {
            (*pointer_state).grab_button = button;
            (*pointer_state).grab_time = time
        }
        set_add((*pointer_state).buttons.as_mut_ptr(),
                &mut (*pointer_state).button_count, 16i32 as size_t, button);
    } else {
        set_remove((*pointer_state).buttons.as_mut_ptr(),
                   &mut (*pointer_state).button_count, 16i32 as size_t,
                   button);
    }
    let mut grab: *mut wlr_seat_pointer_grab = (*pointer_state).grab;
    let mut serial: uint32_t =
        (*(*grab).interface).button.expect("non-null function pointer")(grab,
                                                                        time,
                                                                        button,
                                                                        state);
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] button_count=%zu grab_serial=%u serial=%u\x00" as
                 *const u8 as *const libc::c_char,
             b"../types/seat/wlr_seat_pointer.c\x00" as *const u8 as
                 *const libc::c_char, 364i32, (*pointer_state).button_count,
             (*pointer_state).grab_serial, serial);
    if serial != 0 && (*pointer_state).button_count == 1i32 as libc::c_ulong
           &&
           state as libc::c_uint ==
               WLR_BUTTON_PRESSED as libc::c_int as libc::c_uint {
        (*pointer_state).grab_serial = serial
    }
    return serial;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_notify_axis(mut wlr_seat:
                                                          *mut wlr_seat,
                                                      mut time: uint32_t,
                                                      mut orientation:
                                                          wlr_axis_orientation,
                                                      mut value:
                                                          libc::c_double,
                                                      mut value_discrete:
                                                          int32_t,
                                                      mut source:
                                                          wlr_axis_source) {
    clock_gettime(1i32, &mut (*wlr_seat).last_event);
    let mut grab: *mut wlr_seat_pointer_grab = (*wlr_seat).pointer_state.grab;
    (*(*grab).interface).axis.expect("non-null function pointer")(grab, time,
                                                                  orientation,
                                                                  value,
                                                                  value_discrete,
                                                                  source);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_notify_frame(mut wlr_seat:
                                                           *mut wlr_seat) {
    clock_gettime(1i32, &mut (*wlr_seat).last_event);
    let mut grab: *mut wlr_seat_pointer_grab = (*wlr_seat).pointer_state.grab;
    if (*(*grab).interface).frame.is_some() {
        (*(*grab).interface).frame.expect("non-null function pointer")(grab);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_pointer_has_grab(mut seat: *mut wlr_seat)
 -> bool {
    return (*(*seat).pointer_state.grab).interface !=
               &default_pointer_grab_impl as
                   *const wlr_pointer_grab_interface;
}
#[no_mangle]
pub unsafe extern "C" fn seat_client_create_pointer(mut seat_client:
                                                        *mut wlr_seat_client,
                                                    mut version: uint32_t,
                                                    mut id: uint32_t) {
    let mut resource: *mut wl_resource =
        wl_resource_create((*seat_client).client, &wl_pointer_interface,
                           version as libc::c_int, id);
    if resource.is_null() {
        wl_client_post_no_memory((*seat_client).client);
        return
    }
    wl_resource_set_implementation(resource,
                                   &pointer_impl as
                                       *const wl_pointer_interface as
                                       *const libc::c_void,
                                   seat_client as *mut libc::c_void,
                                   Some(pointer_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_insert(&mut (*seat_client).pointers,
                   wl_resource_get_link(resource));
}
#[no_mangle]
pub unsafe extern "C" fn seat_client_destroy_pointer(mut resource:
                                                         *mut wl_resource) {
    let mut seat_client: *mut wlr_seat_client =
        wlr_seat_client_from_pointer_resource(resource);
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
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_validate_pointer_grab_serial(mut seat:
                                                                   *mut wlr_seat,
                                                               mut origin:
                                                                   *mut wlr_surface,
                                                               mut serial:
                                                                   uint32_t)
 -> bool {
    if (*seat).pointer_state.button_count != 1i32 as libc::c_ulong ||
           (*seat).pointer_state.grab_serial != serial {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Pointer grab serial validation failed: button_count=%zu grab_serial=%u (got %u)\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat_pointer.c\x00" as *const u8 as
                     *const libc::c_char, 425i32,
                 (*seat).pointer_state.button_count,
                 (*seat).pointer_state.grab_serial, serial);
        return 0i32 != 0
    }
    if !origin.is_null() && (*seat).pointer_state.focused_surface != origin {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Pointer grab serial validation failed: invalid origin surface\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat_pointer.c\x00" as *const u8 as
                     *const libc::c_char, 431i32);
        return 0i32 != 0
    }
    return 1i32 != 0;
}
