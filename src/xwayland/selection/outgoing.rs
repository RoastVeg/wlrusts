use libc;
extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_texture_impl;
    pub type wlr_renderer_impl;
    pub type xcb_connection_t;
    pub type xcb_errors_context_t;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_xwayland_cursor;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_length(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_array_init(array: *mut wl_array);
    #[no_mangle]
    fn wl_array_release(array: *mut wl_array);
    #[no_mangle]
    fn wl_array_add(array: *mut wl_array, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn wl_event_loop_add_fd(loop_0: *mut wl_event_loop, fd: libc::c_int,
                            mask: uint32_t, func: wl_event_loop_fd_func_t,
                            data: *mut libc::c_void) -> *mut wl_event_source;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn wlr_data_source_send(source: *mut wlr_data_source,
                            mime_type: *const libc::c_char, fd: int32_t);
    #[no_mangle]
    fn wlr_primary_selection_source_send(source:
                                             *mut wlr_primary_selection_source,
                                         mime_type: *const libc::c_char,
                                         fd: libc::c_int);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn xcb_change_property(c: *mut xcb_connection_t, mode: uint8_t,
                           window: xcb_window_t, property: xcb_atom_t,
                           type_0: xcb_atom_t, format: uint8_t,
                           data_len: uint32_t, data: *const libc::c_void)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_send_event(c: *mut xcb_connection_t, propagate: uint8_t,
                      destination: xcb_window_t, event_mask: uint32_t,
                      event: *const libc::c_char) -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_flush(c: *mut xcb_connection_t) -> libc::c_int;
    #[no_mangle]
    fn xwm_selection_transfer_remove_source(transfer:
                                                *mut wlr_xwm_selection_transfer);
    #[no_mangle]
    fn xwm_selection_transfer_close_source_fd(transfer:
                                                  *mut wlr_xwm_selection_transfer);
    #[no_mangle]
    fn xwm_mime_type_to_atom(xwm: *mut wlr_xwm, mime_type: *mut libc::c_char)
     -> xcb_atom_t;
    #[no_mangle]
    fn xwm_mime_type_from_atom(xwm: *mut wlr_xwm, atom: xcb_atom_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn xwm_get_selection(xwm: *mut wlr_xwm, selection_atom: xcb_atom_t)
     -> *mut wlr_xwm_selection;
    /* This is in xcb/xcb_event.h, but pulling xcb-util just for a constant
 * others redefine anyway is meh
 */
    // wlr_xwayland_surface::link
    // wlr_xwayland_surface::unpaired_link
    #[no_mangle]
    fn xwm_get_atom_name(xwm: *mut wlr_xwm, atom: xcb_atom_t)
     -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_keyboard_modifiers {
    pub depressed: xkb_mod_mask_t,
    pub latched: xkb_mod_mask_t,
    pub locked: xkb_mod_mask_t,
    pub group: xkb_mod_mask_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_keyboard {
    pub impl_0: *const crate::src::backend::headless::input_device::wlr_keyboard_impl,
    pub group: *mut crate::src::types::wlr_keyboard_group::wlr_keyboard_group,
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
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
pub type wl_data_device_manager_dnd_action = libc::c_uint;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK:
          wl_data_device_manager_dnd_action =
    4;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE:
          wl_data_device_manager_dnd_action =
    2;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY:
          wl_data_device_manager_dnd_action =
    1;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE:
          wl_data_device_manager_dnd_action =
    0;
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_texture {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_texture_impl,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_renderer {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_renderer_impl,
    pub events: C2RustUnnamed_2,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_3,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_surface_role {
    pub name: *const libc::c_char,
    pub commit: Option<unsafe extern "C" fn(_: *mut wlr_surface) -> ()>,
    pub precommit: Option<unsafe extern "C" fn(_: *mut wlr_surface) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_serial_range {
    pub min_incl: uint32_t,
    pub max_incl: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_serial_ringset {
    pub data: [wlr_serial_range; 128],
    pub end: libc::c_int,
    pub count: libc::c_int,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_client {
    pub client: *mut wl_client,
    pub seat: *mut wlr_seat,
    pub link: wl_list,
    pub resources: wl_list,
    pub pointers: wl_list,
    pub keyboards: wl_list,
    pub touches: wl_list,
    pub data_devices: wl_list,
    pub events: C2RustUnnamed_4,
    pub serials: wlr_serial_ringset,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_5,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_touch_state {
    pub seat: *mut wlr_seat,
    pub touch_points: wl_list,
    pub grab_serial: uint32_t,
    pub grab_id: uint32_t,
    pub grab: *mut wlr_seat_touch_grab,
    pub default_grab: *mut wlr_seat_touch_grab,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_touch_grab {
    pub interface: *const wlr_touch_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_6,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_7,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub focus_change: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_keyboard_grab {
    pub interface: *const wlr_keyboard_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_8,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
    pub focus_change: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_pointer_grab {
    pub interface: *const wlr_pointer_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_data_source {
    pub impl_0: *const wlr_data_source_impl,
    pub mime_types: wl_array,
    pub actions: int32_t,
    pub accepted: bool,
    pub current_dnd_action: wl_data_device_manager_dnd_action,
    pub compositor_action: uint32_t,
    pub events: C2RustUnnamed_9,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_data_source_impl {
    pub send: Option<unsafe extern "C" fn(_: *mut wlr_data_source,
                                          _: *const libc::c_char, _: int32_t)
                         -> ()>,
    pub accept: Option<unsafe extern "C" fn(_: *mut wlr_data_source,
                                            _: uint32_t,
                                            _: *const libc::c_char) -> ()>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_data_source) -> ()>,
    pub dnd_drop: Option<unsafe extern "C" fn(_: *mut wlr_data_source) -> ()>,
    pub dnd_finish: Option<unsafe extern "C" fn(_: *mut wlr_data_source)
                               -> ()>,
    pub dnd_action: Option<unsafe extern "C" fn(_: *mut wlr_data_source,
                                                _:
                                                    wl_data_device_manager_dnd_action)
                               -> ()>,
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drag {
    pub grab_type: wlr_drag_grab_type,
    pub keyboard_grab: wlr_seat_keyboard_grab,
    pub pointer_grab: wlr_seat_pointer_grab,
    pub touch_grab: wlr_seat_touch_grab,
    pub seat: *mut wlr_seat,
    pub seat_client: *mut wlr_seat_client,
    pub focus_client: *mut wlr_seat_client,
    pub icon: *mut wlr_drag_icon,
    pub focus: *mut wlr_surface,
    pub source: *mut wlr_data_source,
    pub started: bool,
    pub dropped: bool,
    pub cancelling: bool,
    pub grab_touch_id: int32_t,
    pub touch_id: int32_t,
    pub events: C2RustUnnamed_10,
    pub point_destroy: wl_listener,
    pub source_destroy: wl_listener,
    pub seat_client_destroy: wl_listener,
    pub icon_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
    pub focus: wl_signal,
    pub motion: wl_signal,
    pub drop: wl_signal,
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drag_icon {
    pub drag: *mut wlr_drag,
    pub surface: *mut wlr_surface,
    pub mapped: bool,
    pub events: C2RustUnnamed_11,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
    pub map: wl_signal,
    pub unmap: wl_signal,
    pub destroy: wl_signal,
}
pub type wlr_drag_grab_type = libc::c_uint;
pub const WLR_DRAG_GRAB_KEYBOARD_TOUCH: wlr_drag_grab_type = 2;
pub const WLR_DRAG_GRAB_KEYBOARD_POINTER: wlr_drag_grab_type = 1;
pub const WLR_DRAG_GRAB_KEYBOARD: wlr_drag_grab_type = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_primary_selection_source {
    pub impl_0: *const wlr_primary_selection_source_impl,
    pub mime_types: wl_array,
    pub events: C2RustUnnamed_12,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_12 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_primary_selection_source_impl {
    pub send: Option<unsafe extern "C" fn(_:
                                              *mut wlr_primary_selection_source,
                                          _: *const libc::c_char,
                                          _: libc::c_int) -> ()>,
    pub destroy: Option<unsafe extern "C" fn(_:
                                                 *mut wlr_primary_selection_source)
                            -> ()>,
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
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_void_cookie_t {
    pub sequence: libc::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_pixmap_t = uint32_t;
pub type xcb_cursor_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_event_mask_t = libc::c_uint;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
pub const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
pub const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
pub const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
pub const XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
pub const XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
pub const XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
pub const XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
pub const XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
pub const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
pub const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
pub const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;

#[repr(C)]#[derive(Copy, Clone)]
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
pub type xcb_atom_enum_t = libc::c_uint;
pub const XCB_ATOM_WM_TRANSIENT_FOR: xcb_atom_enum_t = 68;
pub const XCB_ATOM_WM_CLASS: xcb_atom_enum_t = 67;
pub const XCB_ATOM_CAP_HEIGHT: xcb_atom_enum_t = 66;
pub const XCB_ATOM_FULL_NAME: xcb_atom_enum_t = 65;
pub const XCB_ATOM_FAMILY_NAME: xcb_atom_enum_t = 64;
pub const XCB_ATOM_FONT_NAME: xcb_atom_enum_t = 63;
pub const XCB_ATOM_NOTICE: xcb_atom_enum_t = 62;
pub const XCB_ATOM_COPYRIGHT: xcb_atom_enum_t = 61;
pub const XCB_ATOM_RESOLUTION: xcb_atom_enum_t = 60;
pub const XCB_ATOM_POINT_SIZE: xcb_atom_enum_t = 59;
pub const XCB_ATOM_WEIGHT: xcb_atom_enum_t = 58;
pub const XCB_ATOM_QUAD_WIDTH: xcb_atom_enum_t = 57;
pub const XCB_ATOM_X_HEIGHT: xcb_atom_enum_t = 56;
pub const XCB_ATOM_ITALIC_ANGLE: xcb_atom_enum_t = 55;
pub const XCB_ATOM_STRIKEOUT_DESCENT: xcb_atom_enum_t = 54;
pub const XCB_ATOM_STRIKEOUT_ASCENT: xcb_atom_enum_t = 53;
pub const XCB_ATOM_UNDERLINE_THICKNESS: xcb_atom_enum_t = 52;
pub const XCB_ATOM_UNDERLINE_POSITION: xcb_atom_enum_t = 51;
pub const XCB_ATOM_SUBSCRIPT_Y: xcb_atom_enum_t = 50;
pub const XCB_ATOM_SUBSCRIPT_X: xcb_atom_enum_t = 49;
pub const XCB_ATOM_SUPERSCRIPT_Y: xcb_atom_enum_t = 48;
pub const XCB_ATOM_SUPERSCRIPT_X: xcb_atom_enum_t = 47;
pub const XCB_ATOM_END_SPACE: xcb_atom_enum_t = 46;
pub const XCB_ATOM_MAX_SPACE: xcb_atom_enum_t = 45;
pub const XCB_ATOM_NORM_SPACE: xcb_atom_enum_t = 44;
pub const XCB_ATOM_MIN_SPACE: xcb_atom_enum_t = 43;
pub const XCB_ATOM_WM_ZOOM_HINTS: xcb_atom_enum_t = 42;
pub const XCB_ATOM_WM_SIZE_HINTS: xcb_atom_enum_t = 41;
pub const XCB_ATOM_WM_NORMAL_HINTS: xcb_atom_enum_t = 40;
pub const XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;
pub const XCB_ATOM_WM_ICON_SIZE: xcb_atom_enum_t = 38;
pub const XCB_ATOM_WM_ICON_NAME: xcb_atom_enum_t = 37;
pub const XCB_ATOM_WM_CLIENT_MACHINE: xcb_atom_enum_t = 36;
pub const XCB_ATOM_WM_HINTS: xcb_atom_enum_t = 35;
pub const XCB_ATOM_WM_COMMAND: xcb_atom_enum_t = 34;
pub const XCB_ATOM_WINDOW: xcb_atom_enum_t = 33;
pub const XCB_ATOM_VISUALID: xcb_atom_enum_t = 32;
pub const XCB_ATOM_STRING: xcb_atom_enum_t = 31;
pub const XCB_ATOM_RGB_RED_MAP: xcb_atom_enum_t = 30;
pub const XCB_ATOM_RGB_GREEN_MAP: xcb_atom_enum_t = 29;
pub const XCB_ATOM_RGB_GRAY_MAP: xcb_atom_enum_t = 28;
pub const XCB_ATOM_RGB_DEFAULT_MAP: xcb_atom_enum_t = 27;
pub const XCB_ATOM_RGB_BLUE_MAP: xcb_atom_enum_t = 26;
pub const XCB_ATOM_RGB_BEST_MAP: xcb_atom_enum_t = 25;
pub const XCB_ATOM_RGB_COLOR_MAP: xcb_atom_enum_t = 24;
pub const XCB_ATOM_RESOURCE_MANAGER: xcb_atom_enum_t = 23;
pub const XCB_ATOM_RECTANGLE: xcb_atom_enum_t = 22;
pub const XCB_ATOM_POINT: xcb_atom_enum_t = 21;
pub const XCB_ATOM_PIXMAP: xcb_atom_enum_t = 20;
pub const XCB_ATOM_INTEGER: xcb_atom_enum_t = 19;
pub const XCB_ATOM_FONT: xcb_atom_enum_t = 18;
pub const XCB_ATOM_DRAWABLE: xcb_atom_enum_t = 17;
pub const XCB_ATOM_CUT_BUFFER7: xcb_atom_enum_t = 16;
pub const XCB_ATOM_CUT_BUFFER6: xcb_atom_enum_t = 15;
pub const XCB_ATOM_CUT_BUFFER5: xcb_atom_enum_t = 14;
pub const XCB_ATOM_CUT_BUFFER4: xcb_atom_enum_t = 13;
pub const XCB_ATOM_CUT_BUFFER3: xcb_atom_enum_t = 12;
pub const XCB_ATOM_CUT_BUFFER2: xcb_atom_enum_t = 11;
pub const XCB_ATOM_CUT_BUFFER1: xcb_atom_enum_t = 10;
pub const XCB_ATOM_CUT_BUFFER0: xcb_atom_enum_t = 9;
pub const XCB_ATOM_CURSOR: xcb_atom_enum_t = 8;
pub const XCB_ATOM_COLORMAP: xcb_atom_enum_t = 7;
pub const XCB_ATOM_CARDINAL: xcb_atom_enum_t = 6;
pub const XCB_ATOM_BITMAP: xcb_atom_enum_t = 5;
pub const XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
pub const XCB_ATOM_ARC: xcb_atom_enum_t = 3;
pub const XCB_ATOM_SECONDARY: xcb_atom_enum_t = 2;
pub const XCB_ATOM_PRIMARY: xcb_atom_enum_t = 1;
pub const XCB_ATOM_ANY: xcb_atom_enum_t = 0;
pub const XCB_ATOM_NONE: xcb_atom_enum_t = 0;

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_selection_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}
pub type xcb_prop_mode_t = libc::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
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
pub type xcb_render_pictformat_t = uint32_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwm_selection {
    pub xwm: *mut wlr_xwm,
    pub atom: xcb_atom_t,
    pub window: xcb_window_t,
    pub owner: xcb_window_t,
    pub timestamp: xcb_timestamp_t,
    pub incoming: wlr_xwm_selection_transfer,
    pub outgoing: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
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
/* *
 * An Xwayland user interface component. It has an absolute position in
 * layout-local coordinates.
 *
 * When a surface is ready to be displayed, the `map` event is emitted. When a
 * surface should no longer be displayed, the `unmap` event is emitted. The
 * `unmap` event is guaranteed to be emitted before the `destroy` event if the
 * view is destroyed when mapped.
 */

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_13,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
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

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwayland {
    pub pid: pid_t,
    pub client: *mut wl_client,
    pub sigusr1_source: *mut wl_event_source,
    pub xwm: *mut wlr_xwm,
    pub cursor: *mut crate::src::xwayland::xwayland::wlr_xwayland_cursor,
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
    pub events: C2RustUnnamed_14,
    pub user_event_handler: Option<unsafe extern "C" fn(_: *mut wlr_xwm,
                                                        _:
                                                            *mut xcb_generic_event_t)
                                       -> libc::c_int>,
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_14 {
    pub ready: wl_signal,
    pub new_surface: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub subcompositor: wlr_subcompositor,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_15,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_15 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_subcompositor {
    pub global: *mut wl_global,
}
pub const INCR: atom_name = 33;
pub const DELETE: atom_name = 36;
pub const TIMESTAMP: atom_name = 35;
pub const TARGETS: atom_name = 31;
pub const CLIPBOARD_MANAGER: atom_name = 32;
pub type atom_name = libc::c_uint;
pub const ATOM_LAST: atom_name = 62;
pub const _NET_CLIENT_LIST: atom_name = 61;
pub const DND_ACTION_PRIVATE: atom_name = 60;
pub const DND_ACTION_ASK: atom_name = 59;
pub const DND_ACTION_COPY: atom_name = 58;
pub const DND_ACTION_MOVE: atom_name = 57;
pub const DND_TYPE_LIST: atom_name = 56;
pub const DND_PROXY: atom_name = 55;
pub const DND_FINISHED: atom_name = 54;
pub const DND_DROP: atom_name = 53;
pub const DND_LEAVE: atom_name = 52;
pub const DND_ENTER: atom_name = 51;
pub const DND_POSITION: atom_name = 50;
pub const DND_STATUS: atom_name = 49;
pub const DND_AWARE: atom_name = 48;
pub const DND_SELECTION: atom_name = 47;
pub const NET_WM_WINDOW_TYPE_SPLASH: atom_name = 46;
pub const NET_WM_WINDOW_TYPE_NOTIFICATION: atom_name = 45;
pub const NET_WM_WINDOW_TYPE_MENU: atom_name = 44;
pub const NET_WM_WINDOW_TYPE_COMBO: atom_name = 43;
pub const NET_WM_WINDOW_TYPE_POPUP_MENU: atom_name = 42;
pub const NET_WM_WINDOW_TYPE_DROPDOWN_MENU: atom_name = 41;
pub const NET_WM_WINDOW_TYPE_DND: atom_name = 40;
pub const NET_WM_WINDOW_TYPE_TOOLTIP: atom_name = 39;
pub const NET_WM_WINDOW_TYPE_UTILITY: atom_name = 38;
pub const NET_WM_WINDOW_TYPE_NORMAL: atom_name = 37;
pub const TEXT: atom_name = 34;
pub const WL_SELECTION: atom_name = 30;
pub const PRIMARY: atom_name = 29;
pub const CLIPBOARD: atom_name = 28;
pub const WM_STATE: atom_name = 27;
pub const _NET_WM_PING: atom_name = 26;
pub const _NET_WM_STATE_MAXIMIZED_HORZ: atom_name = 25;
pub const _NET_WM_STATE_MAXIMIZED_VERT: atom_name = 24;
pub const _NET_WM_STATE_FULLSCREEN: atom_name = 23;
pub const _NET_WM_STATE_MODAL: atom_name = 22;
pub const _NET_SUPPORTING_WM_CHECK: atom_name = 21;
pub const _NET_WM_NAME: atom_name = 20;
pub const _NET_WM_MOVERESIZE: atom_name = 19;
pub const _NET_ACTIVE_WINDOW: atom_name = 18;
pub const WINDOW: atom_name = 17;
pub const WM_TAKE_FOCUS: atom_name = 16;
pub const NET_WM_WINDOW_TYPE: atom_name = 15;
pub const NET_WM_STATE: atom_name = 14;
pub const NET_WM_NAME: atom_name = 13;
pub const NET_WM_PID: atom_name = 12;
pub const NET_WM_CM_S0: atom_name = 11;
pub const NET_SUPPORTED: atom_name = 10;
pub const WM_S0: atom_name = 9;
pub const UTF8_STRING: atom_name = 8;
pub const MOTIF_WM_HINTS: atom_name = 7;
pub const WM_WINDOW_ROLE: atom_name = 6;
pub const WM_SIZE_HINTS: atom_name = 5;
pub const WM_NORMAL_HINTS: atom_name = 4;
pub const WM_HINTS: atom_name = 3;
pub const WM_PROTOCOLS: atom_name = 2;
pub const WM_DELETE_WINDOW: atom_name = 1;
pub const WL_SURFACE_ID: atom_name = 0;
unsafe extern "C" fn xwm_selection_send_notify(mut xwm: *mut wlr_xwm,
                                               mut req:
                                                   *mut xcb_selection_request_event_t,
                                               mut success: bool) {
    let mut selection_notify: xcb_selection_notify_event_t =
        {
            let mut init =
                xcb_selection_notify_event_t{response_type: 31i32 as uint8_t,
                                             pad0: 0,
                                             sequence: 0i32 as uint16_t,
                                             time: (*req).time,
                                             requestor: (*req).requestor,
                                             selection: (*req).selection,
                                             target: (*req).target,
                                             property:
                                                 if success as libc::c_int !=
                                                        0 {
                                                     (*req).property
                                                 } else {
                                                     XCB_ATOM_NONE as
                                                         libc::c_int as
                                                         libc::c_uint
                                                 },};
            init
        };
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] SendEvent destination=%d SelectionNotify(31) time=%d requestor=%d selection=%d target=%d property=%d\x00"
                 as *const u8 as *const libc::c_char,
             b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                 *const libc::c_char, 28i32, (*req).requestor, (*req).time,
             (*req).requestor, (*req).selection, (*req).target,
             selection_notify.property);
    xcb_send_event((*xwm).xcb_conn, 0i32 as uint8_t, (*req).requestor,
                   XCB_EVENT_MASK_NO_EVENT as libc::c_int as uint32_t,
                   &mut selection_notify as *mut xcb_selection_notify_event_t
                       as *const libc::c_char);
    xcb_flush((*xwm).xcb_conn);
}
unsafe extern "C" fn xwm_selection_flush_source_data(mut transfer:
                                                         *mut wlr_xwm_selection_transfer)
 -> libc::c_int {
    xcb_change_property((*(*(*transfer).selection).xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*transfer).request.requestor,
                        (*transfer).request.property,
                        (*transfer).request.target, 8i32 as uint8_t,
                        (*transfer).source_data.size as uint32_t,
                        (*transfer).source_data.data);
    xcb_flush((*(*(*transfer).selection).xwm).xcb_conn);
    (*transfer).property_set = 1i32 != 0;
    let mut length: size_t = (*transfer).source_data.size;
    (*transfer).source_data.size = 0i32 as size_t;
    return length as libc::c_int;
}
unsafe extern "C" fn xwm_selection_transfer_destroy_outgoing(mut transfer:
                                                                 *mut wlr_xwm_selection_transfer) {
    wl_list_remove(&mut (*transfer).outgoing_link);
    // Start next queued transfer
    let mut first: *mut wlr_xwm_selection_transfer =
        0 as *mut wlr_xwm_selection_transfer;
    if wl_list_empty(&mut (*(*transfer).selection).outgoing) == 0 {
        first =
            ((*(*transfer).selection).outgoing.prev as
                 *mut libc::c_char).offset(-88) as
                *mut wlr_xwm_selection_transfer;
        xwm_selection_transfer_start_outgoing(first);
    }
    xwm_selection_transfer_remove_source(transfer);
    xwm_selection_transfer_close_source_fd(transfer);
    wl_array_release(&mut (*transfer).source_data);
    free(transfer as *mut libc::c_void);
}
unsafe extern "C" fn xwm_data_source_read(mut fd: libc::c_int,
                                          mut mask: uint32_t,
                                          mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut available: size_t = 0;
    let mut len: ssize_t = 0;
    let mut current_block: u64;
    let mut transfer: *mut wlr_xwm_selection_transfer =
        data as *mut wlr_xwm_selection_transfer;
    let mut xwm: *mut wlr_xwm = (*(*transfer).selection).xwm;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut current: size_t = (*transfer).source_data.size;
    if (*transfer).source_data.size < (64i32 * 1024i32) as libc::c_ulong {
        p =
            wl_array_add(&mut (*transfer).source_data,
                         (64i32 * 1024i32) as size_t);
        if p.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Could not allocate selection source_data\x00"
                         as *const u8 as *const libc::c_char,
                     b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                         *const libc::c_char, 84i32);
            current_block = 2624304445916772840;
        } else { current_block = 5720623009719927633; }
    } else {
        p =
            ((*transfer).source_data.data as
                 *mut libc::c_char).offset((*transfer).source_data.size as
                                               isize) as *mut libc::c_void;
        current_block = 5720623009719927633;
    }
    match current_block {
        5720623009719927633 => {
            available = (*transfer).source_data.alloc.wrapping_sub(current);
            len = read(fd, p, available);
            if len == -1i32 as libc::c_long {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] read error from data source: %m\x00" as
                             *const u8 as *const libc::c_char,
                         b"../xwayland/selection/outgoing.c\x00" as *const u8
                             as *const libc::c_char, 94i32);
            } else {
                _wlr_log(WLR_DEBUG,
                         b"[%s:%d] read %zd bytes (available %zu, mask 0x%x)\x00"
                             as *const u8 as *const libc::c_char,
                         b"../xwayland/selection/outgoing.c\x00" as *const u8
                             as *const libc::c_char, 99i32, len, available,
                         mask);
                (*transfer).source_data.size =
                    current.wrapping_add(len as libc::c_ulong);
                if (*transfer).source_data.size >=
                       (64i32 * 1024i32) as libc::c_ulong {
                    if !(*transfer).incr {
                        _wlr_log(WLR_DEBUG,
                                 b"[%s:%d] got %zu bytes, starting incr\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"../xwayland/selection/outgoing.c\x00" as
                                     *const u8 as *const libc::c_char, 105i32,
                                 (*transfer).source_data.size);
                        let mut incr_chunk_size: size_t =
                            (64i32 * 1024i32) as size_t;
                        xcb_change_property((*xwm).xcb_conn,
                                            XCB_PROP_MODE_REPLACE as
                                                libc::c_int as uint8_t,
                                            (*transfer).request.requestor,
                                            (*transfer).request.property,
                                            (*xwm).atoms[INCR as libc::c_int
                                                             as usize],
                                            32i32 as uint8_t,
                                            1i32 as uint32_t,
                                            &mut incr_chunk_size as
                                                *mut size_t as
                                                *const libc::c_void);
                        (*transfer).incr = 1i32 != 0;
                        (*transfer).property_set = 1i32 != 0;
                        (*transfer).flush_property_on_delete = 1i32 != 0;
                        xwm_selection_transfer_remove_source(transfer);
                        xwm_selection_send_notify(xwm,
                                                  &mut (*transfer).request,
                                                  1i32 != 0);
                    } else if (*transfer).property_set {
                        _wlr_log(WLR_DEBUG,
                                 b"[%s:%d] got %zu bytes, waiting for property delete\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"../xwayland/selection/outgoing.c\x00" as
                                     *const u8 as *const libc::c_char, 122i32,
                                 (*transfer).source_data.size);
                        (*transfer).flush_property_on_delete = 1i32 != 0;
                        xwm_selection_transfer_remove_source(transfer);
                    } else {
                        _wlr_log(WLR_DEBUG,
                                 b"[%s:%d] got %zu bytes, property deleted, setting new property\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"../xwayland/selection/outgoing.c\x00" as
                                     *const u8 as *const libc::c_char, 128i32,
                                 (*transfer).source_data.size);
                        xwm_selection_flush_source_data(transfer);
                    }
                } else if len == 0i32 as libc::c_long && !(*transfer).incr {
                    _wlr_log(WLR_DEBUG,
                             b"[%s:%d] non-incr transfer complete\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../xwayland/selection/outgoing.c\x00" as
                                 *const u8 as *const libc::c_char, 132i32);
                    xwm_selection_flush_source_data(transfer);
                    xwm_selection_send_notify(xwm, &mut (*transfer).request,
                                              1i32 != 0);
                    xwm_selection_transfer_destroy_outgoing(transfer);
                } else if len == 0i32 as libc::c_long &&
                              (*transfer).incr as libc::c_int != 0 {
                    _wlr_log(WLR_DEBUG,
                             b"[%s:%d] incr transfer complete\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../xwayland/selection/outgoing.c\x00" as
                                 *const u8 as *const libc::c_char, 137i32);
                    (*transfer).flush_property_on_delete = 1i32 != 0;
                    if (*transfer).property_set {
                        _wlr_log(WLR_DEBUG,
                                 b"[%s:%d] got %zu bytes, waiting for property delete\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"../xwayland/selection/outgoing.c\x00" as
                                     *const u8 as *const libc::c_char, 142i32,
                                 (*transfer).source_data.size);
                    } else {
                        _wlr_log(WLR_DEBUG,
                                 b"[%s:%d] got %zu bytes, property deleted, setting new property\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"../xwayland/selection/outgoing.c\x00" as
                                     *const u8 as *const libc::c_char, 145i32,
                                 (*transfer).source_data.size);
                        xwm_selection_flush_source_data(transfer);
                    }
                    xwm_selection_transfer_remove_source(transfer);
                    xwm_selection_transfer_close_source_fd(transfer);
                } else {
                    _wlr_log(WLR_DEBUG,
                             b"[%s:%d] nothing happened, buffered the bytes\x00"
                                 as *const u8 as *const libc::c_char,
                             b"../xwayland/selection/outgoing.c\x00" as
                                 *const u8 as *const libc::c_char, 151i32);
                }
                return 1i32
            }
        }
        _ => { }
    }
    xwm_selection_send_notify(xwm, &mut (*transfer).request, 0i32 != 0);
    xwm_selection_transfer_destroy_outgoing(transfer);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn xwm_send_incr_chunk(mut transfer:
                                                 *mut wlr_xwm_selection_transfer) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] property deleted\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                 *const libc::c_char, 163i32);
    (*transfer).property_set = 0i32 != 0;
    if (*transfer).flush_property_on_delete {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] setting new property, %zu bytes\x00" as *const u8
                     as *const libc::c_char,
                 b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                     *const libc::c_char, 168i32,
                 (*transfer).source_data.size);
        (*transfer).flush_property_on_delete = 0i32 != 0;
        let mut length: libc::c_int =
            xwm_selection_flush_source_data(transfer);
        if (*transfer).source_fd >= 0i32 {
            xwm_selection_transfer_start_outgoing(transfer);
        } else if length > 0i32 {
            /* Transfer is all done, but queue a flush for
			 * the delete of the last chunk so we can set
			 * the 0 sized property to signal the end of
			 * the transfer. */
            (*transfer).flush_property_on_delete = 1i32 != 0;
            wl_array_release(&mut (*transfer).source_data);
            wl_array_init(&mut (*transfer).source_data);
        } else { xwm_selection_transfer_destroy_outgoing(transfer); }
    };
}
unsafe extern "C" fn xwm_selection_source_send(mut selection:
                                                   *mut wlr_xwm_selection,
                                               mut mime_type:
                                                   *const libc::c_char,
                                               mut fd: int32_t) {
    if selection ==
           &mut (*(*selection).xwm).clipboard_selection as
               *mut wlr_xwm_selection {
        let mut source: *mut wlr_data_source =
            (*(*(*selection).xwm).seat).selection_source;
        if !source.is_null() {
            wlr_data_source_send(source, mime_type, fd);
            return
        }
    } else if selection ==
                  &mut (*(*selection).xwm).primary_selection as
                      *mut wlr_xwm_selection {
        let mut source_0: *mut wlr_primary_selection_source =
            (*(*(*selection).xwm).seat).primary_selection_source;
        if !source_0.is_null() {
            wlr_primary_selection_source_send(source_0, mime_type, fd);
            return
        }
    } else if selection ==
                  &mut (*(*selection).xwm).dnd_selection as
                      *mut wlr_xwm_selection {
        let mut source_1: *mut wlr_data_source =
            (*(*(*selection).xwm).seat).drag_source;
        if !source_1.is_null() {
            wlr_data_source_send(source_1, mime_type, fd);
            return
        }
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] not sending selection: no selection source available\x00"
                 as *const u8 as *const libc::c_char,
             b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                 *const libc::c_char, 213i32);
}
unsafe extern "C" fn xwm_selection_transfer_start_outgoing(mut transfer:
                                                               *mut wlr_xwm_selection_transfer) {
    let mut xwm: *mut wlr_xwm = (*(*transfer).selection).xwm;
    let mut loop_0: *mut wl_event_loop =
        wl_display_get_event_loop((*(*xwm).xwayland).wl_display);
    (*transfer).source =
        wl_event_loop_add_fd(loop_0, (*transfer).source_fd,
                             WL_EVENT_READABLE as libc::c_int as uint32_t,
                             Some(xwm_data_source_read as
                                      unsafe extern "C" fn(_: libc::c_int,
                                                           _: uint32_t,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),
                             transfer as *mut libc::c_void);
}
unsafe extern "C" fn xwm_selection_source_get_mime_types(mut selection:
                                                             *mut wlr_xwm_selection)
 -> *mut wl_array {
    if selection ==
           &mut (*(*selection).xwm).clipboard_selection as
               *mut wlr_xwm_selection {
        let mut source: *mut wlr_data_source =
            (*(*(*selection).xwm).seat).selection_source;
        if !source.is_null() { return &mut (*source).mime_types }
    } else if selection ==
                  &mut (*(*selection).xwm).primary_selection as
                      *mut wlr_xwm_selection {
        let mut source_0: *mut wlr_primary_selection_source =
            (*(*(*selection).xwm).seat).primary_selection_source;
        if !source_0.is_null() { return &mut (*source_0).mime_types }
    } else if selection ==
                  &mut (*(*selection).xwm).dnd_selection as
                      *mut wlr_xwm_selection {
        let mut source_1: *mut wlr_data_source =
            (*(*(*selection).xwm).seat).drag_source;
        if !source_1.is_null() { return &mut (*source_1).mime_types }
    }
    return 0 as *mut wl_array;
}
/* *
 * Read the Wayland selection and send it to an Xwayland client.
 */
unsafe extern "C" fn xwm_selection_send_data(mut selection:
                                                 *mut wlr_xwm_selection,
                                             mut req:
                                                 *mut xcb_selection_request_event_t,
                                             mut mime_type:
                                                 *const libc::c_char) {
    // Check MIME type
    let mut mime_types: *mut wl_array =
        xwm_selection_source_get_mime_types(selection);
    if mime_types.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] not sending selection: no MIME type list available\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                     *const libc::c_char, 258i32);
        xwm_selection_send_notify((*selection).xwm, req, 0i32 != 0);
        return
    }
    let mut found: bool = 0i32 != 0;
    let mut mime_type_ptr: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char;
    mime_type_ptr = (*mime_types).data as *mut *mut libc::c_char;
    while (mime_type_ptr as *const libc::c_char) <
              ((*mime_types).data as
                   *const libc::c_char).offset((*mime_types).size as isize) {
        let mut t: *mut libc::c_char = *mime_type_ptr;
        if strcmp(t, mime_type) == 0i32 {
            found = 1i32 != 0;
            break ;
        } else { mime_type_ptr = mime_type_ptr.offset(1) }
    }
    if !found {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] not sending selection: requested an unsupported MIME type %s\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                     *const libc::c_char, 274i32, mime_type);
        xwm_selection_send_notify((*selection).xwm, req, 0i32 != 0);
        return
    }
    let mut transfer: *mut wlr_xwm_selection_transfer =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xwm_selection_transfer>() as
                   libc::c_ulong) as *mut wlr_xwm_selection_transfer;
    if transfer.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                     *const libc::c_char, 282i32);
        return
    }
    (*transfer).selection = selection;
    (*transfer).request = *req;
    wl_array_init(&mut (*transfer).source_data);
    let mut p: [libc::c_int; 2] = [0; 2];
    if pipe(p.as_mut_ptr()) == -1i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] pipe() failed: %m\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                     *const libc::c_char, 291i32);
        xwm_selection_send_notify((*selection).xwm, req, 0i32 != 0);
        return
    }
    fcntl(p[0], 2i32, 1i32);
    fcntl(p[0], 4i32, 0o4000i32);
    fcntl(p[1], 2i32, 1i32);
    fcntl(p[1], 4i32, 0o4000i32);
    (*transfer).source_fd = p[0];
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Sending Wayland selection %u to Xwayland window with MIME type %s, target %u\x00"
                 as *const u8 as *const libc::c_char,
             b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                 *const libc::c_char, 303i32, (*req).target, mime_type,
             (*req).target);
    xwm_selection_source_send(selection, mime_type, p[1]);
    wl_list_insert(&mut (*selection).outgoing,
                   &mut (*transfer).outgoing_link);
    // We can only handle one transfer at a time
    if wl_list_length(&mut (*selection).outgoing) == 1i32 {
        xwm_selection_transfer_start_outgoing(transfer);
    };
}
unsafe extern "C" fn xwm_selection_send_targets(mut selection:
                                                    *mut wlr_xwm_selection,
                                                mut req:
                                                    *mut xcb_selection_request_event_t) {
    let mut xwm: *mut wlr_xwm = (*selection).xwm;
    let mut mime_types: *mut wl_array =
        xwm_selection_source_get_mime_types(selection);
    if mime_types.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] not sending selection targets: no selection source available\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                     *const libc::c_char, 322i32);
        xwm_selection_send_notify((*selection).xwm, req, 0i32 != 0);
        return
    }
    let mut n: size_t =
        (2i32 as
             libc::c_ulong).wrapping_add((*mime_types).size.wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                                             as
                                                                             libc::c_ulong));
    let vla = n as usize;
    let mut targets: Vec<xcb_atom_t> = ::std::vec::from_elem(0, vla);
    *targets.as_mut_ptr().offset(0) =
        (*xwm).atoms[TIMESTAMP as libc::c_int as usize];
    *targets.as_mut_ptr().offset(1) =
        (*xwm).atoms[TARGETS as libc::c_int as usize];
    let mut i: size_t = 0i32 as size_t;
    let mut mime_type_ptr: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char;
    mime_type_ptr = (*mime_types).data as *mut *mut libc::c_char;
    while (mime_type_ptr as *const libc::c_char) <
              ((*mime_types).data as
                   *const libc::c_char).offset((*mime_types).size as isize) {
        let mut mime_type: *mut libc::c_char = *mime_type_ptr;
        *targets.as_mut_ptr().offset((2i32 as libc::c_ulong).wrapping_add(i)
                                         as isize) =
            xwm_mime_type_to_atom(xwm, mime_type);
        i = i.wrapping_add(1);
        mime_type_ptr = mime_type_ptr.offset(1)
    }
    xcb_change_property((*xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*req).requestor, (*req).property,
                        XCB_ATOM_ATOM as libc::c_int as xcb_atom_t,
                        32i32 as uint8_t, n as uint32_t,
                        targets.as_mut_ptr() as *const libc::c_void);
    xwm_selection_send_notify((*selection).xwm, req, 1i32 != 0);
}
unsafe extern "C" fn xwm_selection_send_timestamp(mut selection:
                                                      *mut wlr_xwm_selection,
                                                  mut req:
                                                      *mut xcb_selection_request_event_t) {
    xcb_change_property((*(*selection).xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*req).requestor, (*req).property,
                        XCB_ATOM_INTEGER as libc::c_int as xcb_atom_t,
                        32i32 as uint8_t, 1i32 as uint32_t,
                        &mut (*selection).timestamp as *mut xcb_timestamp_t as
                            *const libc::c_void);
    xwm_selection_send_notify((*selection).xwm, req, 1i32 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn xwm_handle_selection_request(mut xwm: *mut wlr_xwm,
                                                      mut req:
                                                          *mut xcb_selection_request_event_t) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_SELECTION_REQUEST (time=%u owner=%u, requestor=%u selection=%u, target=%u, property=%u)\x00"
                 as *const u8 as *const libc::c_char,
             b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                 *const libc::c_char, 369i32, (*req).time, (*req).owner,
             (*req).requestor, (*req).selection, (*req).target,
             (*req).property);
    if (*req).selection ==
           (*xwm).atoms[CLIPBOARD_MANAGER as libc::c_int as usize] {
        // The wlroots clipboard should already have grabbed the first target,
		// so just send selection notify now. This isn't synchronized with the
		// clipboard finishing getting the data, so there's a race here.
        xwm_selection_send_notify(xwm, req, 1i32 != 0);
        return
    }
    let mut selection: *mut wlr_xwm_selection =
        xwm_get_selection(xwm, (*req).selection);
    if selection.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] received selection request for unknown selection\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                     *const libc::c_char, 382i32);
        return
    }
    if (*selection).window != (*req).owner {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] received selection request with invalid owner\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                     *const libc::c_char, 387i32);
        return
    }
    // No xwayland surface focused, deny access to clipboard
    if (*xwm).focus_surface.is_null() && (*xwm).drag_focus.is_null() {
        let mut selection_name: *mut libc::c_char =
            xwm_get_atom_name(xwm, (*selection).atom);
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] denying read access to selection %u (%s): no xwayland surface focused\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                     *const libc::c_char, 395i32, (*selection).atom,
                 selection_name);
        free(selection_name as *mut libc::c_void);
        xwm_selection_send_notify(xwm, req, 0i32 != 0);
        return
    }
    if (*req).target == (*xwm).atoms[TARGETS as libc::c_int as usize] {
        xwm_selection_send_targets(selection, req);
    } else if (*req).target == (*xwm).atoms[TIMESTAMP as libc::c_int as usize]
     {
        xwm_selection_send_timestamp(selection, req);
    } else if (*req).target == (*xwm).atoms[DELETE as libc::c_int as usize] {
        xwm_selection_send_notify((*selection).xwm, req, 1i32 != 0);
    } else {
        // Send data
        let mut mime_type: *mut libc::c_char =
            xwm_mime_type_from_atom(xwm, (*req).target);
        if mime_type.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] ignoring selection request: unknown atom %u\x00"
                         as *const u8 as *const libc::c_char,
                     b"../xwayland/selection/outgoing.c\x00" as *const u8 as
                         *const libc::c_char, 412i32, (*req).target);
            xwm_selection_send_notify(xwm, req, 0i32 != 0);
            return
        }
        xwm_selection_send_data(selection, req, mime_type);
        free(mime_type as *mut libc::c_void);
    };
}
