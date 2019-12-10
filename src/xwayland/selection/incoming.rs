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
    pub type wlr_xwayland_cursor;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
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
    fn wl_display_next_serial(display: *mut wl_display) -> uint32_t;
    #[no_mangle]
    fn wlr_data_source_destroy(source: *mut wlr_data_source);
    #[no_mangle]
    fn wlr_data_source_init(source: *mut wlr_data_source,
                            impl_0: *const wlr_data_source_impl);
    #[no_mangle]
    fn wlr_seat_request_set_selection(seat: *mut wlr_seat,
                                      client: *mut wlr_seat_client,
                                      source: *mut wlr_data_source,
                                      serial: uint32_t);
    #[no_mangle]
    fn wlr_primary_selection_source_init(source:
                                             *mut wlr_primary_selection_source,
                                         impl_0:
                                             *const wlr_primary_selection_source_impl);
    #[no_mangle]
    fn wlr_primary_selection_source_destroy(source:
                                                *mut wlr_primary_selection_source);
    /* *
 * Request setting the primary selection. If `client` is not null, then the
 * serial will be checked against the set of serials sent to the client on that
 * seat.
 */
    #[no_mangle]
    fn wlr_seat_request_set_primary_selection(seat: *mut wlr_seat,
                                              client: *mut wlr_seat_client,
                                              source:
                                                  *mut wlr_primary_selection_source,
                                              serial: uint32_t);
    /* *
 * Sets the current primary selection for the seat. NULL can be provided to
 * clear it. This removes the previous one if there was any. In case the
 * selection doesn't come from a client, wl_display_next_serial() can be used to
 * generate a serial.
 */
    #[no_mangle]
    fn wlr_seat_set_primary_selection(seat: *mut wlr_seat,
                                      source:
                                          *mut wlr_primary_selection_source,
                                      serial: uint32_t);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn xcb_get_atom_name(c: *mut xcb_connection_t, atom: xcb_atom_t)
     -> xcb_get_atom_name_cookie_t;
    #[no_mangle]
    fn xcb_get_atom_name_name(R: *const xcb_get_atom_name_reply_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn xcb_get_atom_name_name_length(R: *const xcb_get_atom_name_reply_t)
     -> libc::c_int;
    #[no_mangle]
    fn xcb_get_atom_name_reply(c: *mut xcb_connection_t,
                               cookie: xcb_get_atom_name_cookie_t,
                               e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_get_atom_name_reply_t;
    #[no_mangle]
    fn xcb_delete_property(c: *mut xcb_connection_t, window: xcb_window_t,
                           property: xcb_atom_t) -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_get_property(c: *mut xcb_connection_t, _delete: uint8_t,
                        window: xcb_window_t, property: xcb_atom_t,
                        type_0: xcb_atom_t, long_offset: uint32_t,
                        long_length: uint32_t) -> xcb_get_property_cookie_t;
    #[no_mangle]
    fn xcb_get_property_value(R: *const xcb_get_property_reply_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn xcb_get_property_value_length(R: *const xcb_get_property_reply_t)
     -> libc::c_int;
    #[no_mangle]
    fn xcb_get_property_reply(c: *mut xcb_connection_t,
                              cookie: xcb_get_property_cookie_t,
                              e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_get_property_reply_t;
    #[no_mangle]
    fn xcb_convert_selection(c: *mut xcb_connection_t,
                             requestor: xcb_window_t, selection: xcb_atom_t,
                             target: xcb_atom_t, property: xcb_atom_t,
                             time: xcb_timestamp_t) -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_flush(c: *mut xcb_connection_t) -> libc::c_int;
    #[no_mangle]
    fn xwm_selection_transfer_remove_source(transfer:
                                                *mut wlr_xwm_selection_transfer);
    #[no_mangle]
    fn xwm_selection_transfer_close_source_fd(transfer:
                                                  *mut wlr_xwm_selection_transfer);
    #[no_mangle]
    fn xwm_selection_transfer_destroy_property_reply(transfer:
                                                         *mut wlr_xwm_selection_transfer);
    #[no_mangle]
    fn xwm_get_selection(xwm: *mut wlr_xwm, selection_atom: xcb_atom_t)
     -> *mut wlr_xwm_selection;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_2,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
// mHz
// only when using a software cursor without a surface
// only when using a cursor surface
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
    pub events: C2RustUnnamed_3,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
    pub events: C2RustUnnamed_4,
    pub serials: wlr_serial_ringset,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub events: C2RustUnnamed_5,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
    pub events: C2RustUnnamed_6,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub events: C2RustUnnamed_7,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub events: C2RustUnnamed_8,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub struct wlr_data_source {
    pub impl_0: *const wlr_data_source_impl,
    pub mime_types: wl_array,
    pub actions: int32_t,
    pub accepted: bool,
    pub current_dnd_action: wl_data_device_manager_dnd_action,
    pub compositor_action: uint32_t,
    pub events: C2RustUnnamed_9,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub focus: wl_signal,
    pub motion: wl_signal,
    pub drop: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_drag_icon {
    pub drag: *mut wlr_drag,
    pub surface: *mut wlr_surface,
    pub mapped: bool,
    pub events: C2RustUnnamed_11,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub map: wl_signal,
    pub unmap: wl_signal,
    pub destroy: wl_signal,
}
pub type wlr_drag_grab_type = libc::c_uint;
pub const WLR_DRAG_GRAB_KEYBOARD_TOUCH: wlr_drag_grab_type = 2;
pub const WLR_DRAG_GRAB_KEYBOARD_POINTER: wlr_drag_grab_type = 1;
pub const WLR_DRAG_GRAB_KEYBOARD: wlr_drag_grab_type = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_primary_selection_source {
    pub impl_0: *const wlr_primary_selection_source_impl,
    pub mime_types: wl_array,
    pub events: C2RustUnnamed_12,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_generic_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub resource_id: uint32_t,
    pub minor_code: uint16_t,
    pub major_code: uint8_t,
    pub pad0: uint8_t,
    pub pad: [uint32_t; 5],
    pub full_sequence: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
pub type xcb_window_enum_t = libc::c_uint;
pub const XCB_WINDOW_NONE: xcb_window_enum_t = 0;
pub type xcb_time_t = libc::c_uint;
pub const XCB_TIME_CURRENT_TIME: xcb_time_t = 0;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_get_atom_name_cookie_t {
    pub sequence: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_get_atom_name_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub name_len: uint16_t,
    pub pad1: [uint8_t; 22],
}
pub type xcb_get_property_type_t = libc::c_uint;
pub const XCB_GET_PROPERTY_TYPE_ANY: xcb_get_property_type_t = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_get_property_cookie_t {
    pub sequence: libc::c_uint,
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
pub type xcb_render_pictformat_t = uint32_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_xfixes_selection_notify_event_t {
    pub response_type: uint8_t,
    pub subtype: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
    pub timestamp: xcb_timestamp_t,
    pub selection_timestamp: xcb_timestamp_t,
    pub pad0: [uint8_t; 8],
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
/* *
 * An Xwayland user interface component. It has an absolute position in
 * layout-local coordinates.
 *
 * When a surface is ready to be displayed, the `map` event is emitted. When a
 * surface should no longer be displayed, the `unmap` event is emitted. The
 * `unmap` event is guaranteed to be emitted before the `destroy` event if the
 * view is destroyed when mapped.
 */
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
    pub events: C2RustUnnamed_13,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub ready: wl_signal,
    pub new_surface: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub subcompositor: wlr_subcompositor,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_15,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_subcompositor {
    pub global: *mut wl_global,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x11_data_source {
    pub base: wlr_data_source,
    pub selection: *mut wlr_xwm_selection,
    pub mime_types_atoms: wl_array,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x11_primary_selection_source {
    pub base: wlr_primary_selection_source,
    pub selection: *mut wlr_xwm_selection,
    pub mime_types_atoms: wl_array,
}
pub const WL_SELECTION: atom_name = 30;
pub const INCR: atom_name = 33;
pub const TIMESTAMP: atom_name = 35;
pub const TARGETS: atom_name = 31;
pub const TEXT: atom_name = 34;
pub const UTF8_STRING: atom_name = 8;
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
pub const DELETE: atom_name = 36;
pub const CLIPBOARD_MANAGER: atom_name = 32;
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
pub const MOTIF_WM_HINTS: atom_name = 7;
pub const WM_WINDOW_ROLE: atom_name = 6;
pub const WM_SIZE_HINTS: atom_name = 5;
pub const WM_NORMAL_HINTS: atom_name = 4;
pub const WM_HINTS: atom_name = 3;
pub const WM_PROTOCOLS: atom_name = 2;
pub const WM_DELETE_WINDOW: atom_name = 1;
pub const WL_SURFACE_ID: atom_name = 0;
/* *
 * Write the X11 selection to a Wayland client.
 */
unsafe extern "C" fn xwm_data_source_write(mut fd: libc::c_int,
                                           mut mask: uint32_t,
                                           mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut transfer: *mut wlr_xwm_selection_transfer =
        data as *mut wlr_xwm_selection_transfer;
    let mut xwm: *mut wlr_xwm = (*(*transfer).selection).xwm;
    let mut property: *mut libc::c_char =
        xcb_get_property_value((*transfer).property_reply) as
            *mut libc::c_char;
    let mut remainder: libc::c_int =
        xcb_get_property_value_length((*transfer).property_reply) -
            (*transfer).property_start;
    let mut len: ssize_t =
        write(fd,
              property.offset((*transfer).property_start as isize) as
                  *const libc::c_void, remainder as size_t);
    if len == -1i32 as libc::c_long {
        xwm_selection_transfer_destroy_property_reply(transfer);
        xwm_selection_transfer_remove_source(transfer);
        xwm_selection_transfer_close_source_fd(transfer);
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] write error to target fd: %m\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/selection/incoming.c\x00" as *const u8 as
                     *const libc::c_char, 30i32);
        return 1i32
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] wrote %zd (chunk size %zd) of %d bytes\x00" as
                 *const u8 as *const libc::c_char,
             b"../xwayland/selection/incoming.c\x00" as *const u8 as
                 *const libc::c_char, 36i32,
             (*transfer).property_start as libc::c_long + len, len,
             xcb_get_property_value_length((*transfer).property_reply));
    (*transfer).property_start =
        ((*transfer).property_start as libc::c_long + len) as libc::c_int;
    if len == remainder as libc::c_long {
        xwm_selection_transfer_destroy_property_reply(transfer);
        xwm_selection_transfer_remove_source(transfer);
        if (*transfer).incr {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] deleting property\x00" as *const u8 as
                         *const libc::c_char,
                     b"../xwayland/selection/incoming.c\x00" as *const u8 as
                         *const libc::c_char, 44i32);
            xcb_delete_property((*xwm).xcb_conn,
                                (*(*transfer).selection).window,
                                (*xwm).atoms[WL_SELECTION as libc::c_int as
                                                 usize]);
            xcb_flush((*xwm).xcb_conn);
        } else {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] transfer complete\x00" as *const u8 as
                         *const libc::c_char,
                     b"../xwayland/selection/incoming.c\x00" as *const u8 as
                         *const libc::c_char, 49i32);
            xwm_selection_transfer_close_source_fd(transfer);
        }
    }
    return 1i32;
}
unsafe extern "C" fn xwm_write_property(mut transfer:
                                            *mut wlr_xwm_selection_transfer,
                                        mut reply:
                                            *mut xcb_get_property_reply_t) {
    let mut xwm: *mut wlr_xwm = (*(*transfer).selection).xwm;
    (*transfer).property_start = 0i32;
    (*transfer).property_reply = reply;
    xwm_data_source_write((*transfer).source_fd,
                          WL_EVENT_WRITABLE as libc::c_int as uint32_t,
                          transfer as *mut libc::c_void);
    if !(*transfer).property_reply.is_null() {
        let mut loop_0: *mut wl_event_loop =
            wl_display_get_event_loop((*(*xwm).xwayland).wl_display);
        (*transfer).source =
            wl_event_loop_add_fd(loop_0, (*transfer).source_fd,
                                 WL_EVENT_WRITABLE as libc::c_int as uint32_t,
                                 Some(xwm_data_source_write as
                                          unsafe extern "C" fn(_: libc::c_int,
                                                               _: uint32_t,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> libc::c_int),
                                 transfer as *mut libc::c_void)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xwm_get_incr_chunk(mut transfer:
                                                *mut wlr_xwm_selection_transfer) {
    let mut xwm: *mut wlr_xwm = (*(*transfer).selection).xwm;
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] xwm_get_incr_chunk\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/selection/incoming.c\x00" as *const u8 as
                 *const libc::c_char, 77i32);
    let mut cookie: xcb_get_property_cookie_t =
        xcb_get_property((*xwm).xcb_conn, 0i32 as uint8_t,
                         (*(*transfer).selection).window,
                         (*xwm).atoms[WL_SELECTION as libc::c_int as usize],
                         XCB_GET_PROPERTY_TYPE_ANY as libc::c_int as
                             xcb_atom_t, 0i32 as uint32_t,
                         0x1fffffffi32 as uint32_t);
    let mut reply: *mut xcb_get_property_reply_t =
        xcb_get_property_reply((*xwm).xcb_conn, cookie,
                               0 as *mut *mut xcb_generic_error_t);
    if reply.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] cannot get selection property\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/selection/incoming.c\x00" as *const u8 as
                     *const libc::c_char, 91i32);
        return
    }
    //dump_property(xwm, xwm->atoms[WL_SELECTION], reply);
    if xcb_get_property_value_length(reply) > 0i32 {
        /* Reply's ownership is transferred to xwm, which is responsible
		 * for freeing it */
        xwm_write_property(transfer, reply);
    } else {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] transfer complete\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/selection/incoming.c\x00" as *const u8 as
                     *const libc::c_char, 101i32);
        xwm_selection_transfer_close_source_fd(transfer);
        free(reply as *mut libc::c_void);
    };
}
unsafe extern "C" fn xwm_selection_get_data(mut selection:
                                                *mut wlr_xwm_selection) {
    let mut xwm: *mut wlr_xwm = (*selection).xwm;
    let mut cookie: xcb_get_property_cookie_t =
        xcb_get_property((*xwm).xcb_conn, 1i32 as uint8_t,
                         (*selection).window,
                         (*xwm).atoms[WL_SELECTION as libc::c_int as usize],
                         XCB_GET_PROPERTY_TYPE_ANY as libc::c_int as
                             xcb_atom_t, 0i32 as uint32_t,
                         0x1fffffffi32 as uint32_t);
    let mut reply: *mut xcb_get_property_reply_t =
        xcb_get_property_reply((*xwm).xcb_conn, cookie,
                               0 as *mut *mut xcb_generic_error_t);
    if reply.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot get selection property\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/selection/incoming.c\x00" as *const u8 as
                     *const libc::c_char, 122i32);
        return
    }
    let mut transfer: *mut wlr_xwm_selection_transfer =
        &mut (*selection).incoming;
    if (*reply).type_0 == (*xwm).atoms[INCR as libc::c_int as usize] {
        (*transfer).incr = 1i32 != 0;
        free(reply as *mut libc::c_void);
    } else {
        (*transfer).incr = 0i32 != 0;
        // reply's ownership is transferred to wm, which is responsible
		// for freeing it
        xwm_write_property(transfer, reply); // not a C string
    };
}
unsafe extern "C" fn source_send(mut selection: *mut wlr_xwm_selection,
                                 mut mime_types: *mut wl_array,
                                 mut mime_types_atoms: *mut wl_array,
                                 mut requested_mime_type: *const libc::c_char,
                                 mut fd: libc::c_int) {
    let mut xwm: *mut wlr_xwm = (*selection).xwm;
    let mut transfer: *mut wlr_xwm_selection_transfer =
        &mut (*selection).incoming;
    let mut atoms: *mut xcb_atom_t =
        (*mime_types_atoms).data as *mut xcb_atom_t;
    let mut found: bool = 0i32 != 0;
    let mut mime_type_atom: xcb_atom_t = 0;
    let mut mime_type_ptr: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char;
    let mut i: size_t = 0i32 as size_t;
    mime_type_ptr = (*mime_types).data as *mut *mut libc::c_char;
    while (mime_type_ptr as *const libc::c_char) <
              ((*mime_types).data as
                   *const libc::c_char).offset((*mime_types).size as isize) {
        let mut mime_type: *mut libc::c_char = *mime_type_ptr;
        if strcmp(mime_type, requested_mime_type) == 0i32 {
            found = 1i32 != 0;
            mime_type_atom = *atoms.offset(i as isize);
            break ;
        } else {
            i = i.wrapping_add(1);
            mime_type_ptr = mime_type_ptr.offset(1)
        }
    }
    if !found {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Cannot send X11 selection to Wayland: unsupported MIME type\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/selection/incoming.c\x00" as *const u8 as
                     *const libc::c_char, 160i32);
        return
    }
    xcb_convert_selection((*xwm).xcb_conn, (*selection).window,
                          (*selection).atom, mime_type_atom,
                          (*xwm).atoms[WL_SELECTION as libc::c_int as usize],
                          XCB_TIME_CURRENT_TIME as libc::c_int as
                              xcb_timestamp_t);
    xcb_flush((*xwm).xcb_conn);
    fcntl(fd, 4i32, 0o1i32 | 0o4000i32);
    (*transfer).source_fd = fd;
}
#[no_mangle]
pub unsafe extern "C" fn data_source_is_xwayland(mut wlr_source:
                                                     *mut wlr_data_source)
 -> bool {
    return (*wlr_source).impl_0 ==
               &data_source_impl as *const wlr_data_source_impl;
}
unsafe extern "C" fn data_source_from_wlr_data_source(mut wlr_source:
                                                          *mut wlr_data_source)
 -> *mut x11_data_source {
    if data_source_is_xwayland(wlr_source) as libc::c_int != 0 {
    } else {
        __assert_fail(b"data_source_is_xwayland(wlr_source)\x00" as *const u8
                          as *const libc::c_char,
                      b"../xwayland/selection/incoming.c\x00" as *const u8 as
                          *const libc::c_char, 192i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"struct x11_data_source *data_source_from_wlr_data_source(struct wlr_data_source *)\x00")).as_ptr());
    };
    return wlr_source as *mut x11_data_source;
}
unsafe extern "C" fn data_source_send(mut wlr_source: *mut wlr_data_source,
                                      mut mime_type: *const libc::c_char,
                                      mut fd: int32_t) {
    let mut source: *mut x11_data_source =
        data_source_from_wlr_data_source(wlr_source);
    let mut selection: *mut wlr_xwm_selection = (*source).selection;
    source_send(selection, &mut (*wlr_source).mime_types,
                &mut (*source).mime_types_atoms, mime_type, fd);
}
unsafe extern "C" fn data_source_destroy(mut wlr_source:
                                             *mut wlr_data_source) {
    let mut source: *mut x11_data_source =
        data_source_from_wlr_data_source(wlr_source);
    wl_array_release(&mut (*source).mime_types_atoms);
    free(source as *mut libc::c_void);
}
static mut data_source_impl: wlr_data_source_impl =
    unsafe {
        {
            let mut init =
                wlr_data_source_impl{send:
                                         Some(data_source_send as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wlr_data_source,
                                                                       _:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           int32_t)
                                                      -> ()),
                                     accept: None,
                                     destroy:
                                         Some(data_source_destroy as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wlr_data_source)
                                                      -> ()),
                                     dnd_drop: None,
                                     dnd_finish: None,
                                     dnd_action: None,};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn primary_selection_source_is_xwayland(mut wlr_source:
                                                                  *mut wlr_primary_selection_source)
 -> bool {
    return (*wlr_source).impl_0 ==
               &primary_selection_source_impl as
                   *const wlr_primary_selection_source_impl;
}
unsafe extern "C" fn primary_selection_source_send(mut wlr_source:
                                                       *mut wlr_primary_selection_source,
                                                   mut mime_type:
                                                       *const libc::c_char,
                                                   mut fd: libc::c_int) {
    let mut source: *mut x11_primary_selection_source =
        wlr_source as *mut x11_primary_selection_source;
    let mut selection: *mut wlr_xwm_selection = (*source).selection;
    source_send(selection, &mut (*wlr_source).mime_types,
                &mut (*source).mime_types_atoms, mime_type, fd);
}
unsafe extern "C" fn primary_selection_source_destroy(mut wlr_source:
                                                          *mut wlr_primary_selection_source) {
    let mut source: *mut x11_primary_selection_source =
        wlr_source as *mut x11_primary_selection_source;
    wl_array_release(&mut (*source).mime_types_atoms);
    free(source as *mut libc::c_void);
}
static mut primary_selection_source_impl: wlr_primary_selection_source_impl =
    unsafe {
        {
            let mut init =
                wlr_primary_selection_source_impl{send:
                                                      Some(primary_selection_source_send
                                                               as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut wlr_primary_selection_source,
                                                                                    _:
                                                                                        *const libc::c_char,
                                                                                    _:
                                                                                        libc::c_int)
                                                                   -> ()),
                                                  destroy:
                                                      Some(primary_selection_source_destroy
                                                               as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut wlr_primary_selection_source)
                                                                   -> ()),};
            init
        }
    };
unsafe extern "C" fn source_get_targets(mut selection: *mut wlr_xwm_selection,
                                        mut mime_types: *mut wl_array,
                                        mut mime_types_atoms: *mut wl_array)
 -> bool {
    let mut xwm: *mut wlr_xwm = (*selection).xwm;
    let mut cookie: xcb_get_property_cookie_t =
        xcb_get_property((*xwm).xcb_conn, 1i32 as uint8_t,
                         (*selection).window,
                         (*xwm).atoms[WL_SELECTION as libc::c_int as usize],
                         XCB_GET_PROPERTY_TYPE_ANY as libc::c_int as
                             xcb_atom_t, 0i32 as uint32_t,
                         4096i32 as uint32_t);
    let mut reply: *mut xcb_get_property_reply_t =
        xcb_get_property_reply((*xwm).xcb_conn, cookie,
                               0 as *mut *mut xcb_generic_error_t);
    if reply.is_null() { return 0i32 != 0 }
    if (*reply).type_0 != XCB_ATOM_ATOM as libc::c_int as libc::c_uint {
        free(reply as *mut libc::c_void);
        return 0i32 != 0
    }
    let mut value: *mut xcb_atom_t =
        xcb_get_property_value(reply) as *mut xcb_atom_t;
    let mut current_block_18: u64;
    let mut i: uint32_t = 0i32 as uint32_t;
    while i < (*reply).value_len {
        let mut mime_type: *mut libc::c_char = 0 as *mut libc::c_char;
        if *value.offset(i as isize) ==
               (*xwm).atoms[UTF8_STRING as libc::c_int as usize] {
            mime_type =
                strdup(b"text/plain;charset=utf-8\x00" as *const u8 as
                           *const libc::c_char);
            current_block_18 = 1608152415753874203;
        } else if *value.offset(i as isize) ==
                      (*xwm).atoms[TEXT as libc::c_int as usize] {
            mime_type =
                strdup(b"text/plain\x00" as *const u8 as *const libc::c_char);
            current_block_18 = 1608152415753874203;
        } else if *value.offset(i as isize) !=
                      (*xwm).atoms[TARGETS as libc::c_int as usize] &&
                      *value.offset(i as isize) !=
                          (*xwm).atoms[TIMESTAMP as libc::c_int as usize] {
            let mut name_cookie: xcb_get_atom_name_cookie_t =
                xcb_get_atom_name((*xwm).xcb_conn, *value.offset(i as isize));
            let mut name_reply: *mut xcb_get_atom_name_reply_t =
                xcb_get_atom_name_reply((*xwm).xcb_conn, name_cookie,
                                        0 as *mut *mut xcb_generic_error_t);
            if name_reply.is_null() {
                current_block_18 = 13513818773234778473;
            } else {
                let mut len: size_t =
                    xcb_get_atom_name_name_length(name_reply) as size_t;
                let mut name: *mut libc::c_char =
                    xcb_get_atom_name_name(name_reply);
                if !memchr(name as *const libc::c_void, '/' as i32,
                           len).is_null() {
                    mime_type =
                        malloc(len.wrapping_add(1i32 as
                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                    as
                                                                                    libc::c_ulong))
                            as *mut libc::c_char;
                    if mime_type.is_null() {
                        free(name_reply as *mut libc::c_void);
                        current_block_18 = 13513818773234778473;
                    } else {
                        memcpy(mime_type as *mut libc::c_void,
                               name as *const libc::c_void, len);
                        *mime_type.offset(len as isize) =
                            '\u{0}' as i32 as libc::c_char;
                        current_block_18 = 2232869372362427478;
                    }
                } else { current_block_18 = 2232869372362427478; }
                match current_block_18 {
                    13513818773234778473 => { }
                    _ => {
                        free(name_reply as *mut libc::c_void);
                        current_block_18 = 1608152415753874203;
                    }
                }
            }
        } else { current_block_18 = 1608152415753874203; }
        match current_block_18 {
            1608152415753874203 => {
                if !mime_type.is_null() {
                    let mut mime_type_ptr: *mut *mut libc::c_char =
                        wl_array_add(mime_types,
                                     ::std::mem::size_of::<*mut libc::c_char>()
                                         as libc::c_ulong) as
                            *mut *mut libc::c_char;
                    if mime_type_ptr.is_null() {
                        free(mime_type as *mut libc::c_void);
                        break ;
                    } else {
                        *mime_type_ptr = mime_type;
                        let mut atom_ptr: *mut xcb_atom_t =
                            wl_array_add(mime_types_atoms,
                                         ::std::mem::size_of::<xcb_atom_t>()
                                             as libc::c_ulong) as
                                *mut xcb_atom_t;
                        if atom_ptr.is_null() { break ; }
                        *atom_ptr = *value.offset(i as isize)
                    }
                }
            }
            _ => { }
        }
        i = i.wrapping_add(1)
    }
    free(reply as *mut libc::c_void);
    return 1i32 != 0;
}
unsafe extern "C" fn xwm_selection_get_targets(mut selection:
                                                   *mut wlr_xwm_selection) {
    // set the wayland selection to the X11 selection
    let mut xwm: *mut wlr_xwm = (*selection).xwm;
    if selection == &mut (*xwm).clipboard_selection as *mut wlr_xwm_selection
       {
        let mut source: *mut x11_data_source =
            calloc(1i32 as libc::c_ulong,
                   ::std::mem::size_of::<x11_data_source>() as libc::c_ulong)
                as *mut x11_data_source;
        if source.is_null() { return }
        wlr_data_source_init(&mut (*source).base, &data_source_impl);
        (*source).selection = selection;
        wl_array_init(&mut (*source).mime_types_atoms);
        let mut ok: bool =
            source_get_targets(selection, &mut (*source).base.mime_types,
                               &mut (*source).mime_types_atoms);
        if ok {
            wlr_seat_request_set_selection((*xwm).seat,
                                           0 as *mut wlr_seat_client,
                                           &mut (*source).base,
                                           wl_display_next_serial((*(*xwm).xwayland).wl_display));
        } else { wlr_data_source_destroy(&mut (*source).base); }
    } else if selection ==
                  &mut (*xwm).primary_selection as *mut wlr_xwm_selection {
        let mut source_0: *mut x11_primary_selection_source =
            calloc(1i32 as libc::c_ulong,
                   ::std::mem::size_of::<x11_primary_selection_source>() as
                       libc::c_ulong) as *mut x11_primary_selection_source;
        if source_0.is_null() { return }
        wlr_primary_selection_source_init(&mut (*source_0).base,
                                          &primary_selection_source_impl);
        (*source_0).selection = selection;
        wl_array_init(&mut (*source_0).mime_types_atoms);
        let mut ok_0: bool =
            source_get_targets(selection, &mut (*source_0).base.mime_types,
                               &mut (*source_0).mime_types_atoms);
        if ok_0 {
            wlr_seat_set_primary_selection((*xwm).seat, &mut (*source_0).base,
                                           wl_display_next_serial((*(*xwm).xwayland).wl_display));
        } else {
            wlr_primary_selection_source_destroy(&mut (*source_0).base);
        }
    } else {
        (selection) == &mut (*xwm).dnd_selection as *mut wlr_xwm_selection;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xwm_handle_selection_notify(mut xwm: *mut wlr_xwm,
                                                     mut event:
                                                         *mut xcb_selection_notify_event_t) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_SELECTION_NOTIFY (selection=%u, property=%u, target=%u)\x00"
                 as *const u8 as *const libc::c_char,
             b"../xwayland/selection/incoming.c\x00" as *const u8 as
                 *const libc::c_char, 386i32, (*event).selection,
             (*event).property, (*event).target);
    let mut selection: *mut wlr_xwm_selection =
        xwm_get_selection(xwm, (*event).selection);
    if selection.is_null() { return }
    if (*event).property == XCB_ATOM_NONE as libc::c_int as libc::c_uint {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] convert selection failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/selection/incoming.c\x00" as *const u8 as
                     *const libc::c_char, 395i32);
    } else if (*event).target == (*xwm).atoms[TARGETS as libc::c_int as usize]
     {
        // No xwayland surface focused, deny access to clipboard
        if (*xwm).focus_surface.is_null() {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] denying write access to clipboard: no xwayland surface focused\x00"
                         as *const u8 as *const libc::c_char,
                     b"../xwayland/selection/incoming.c\x00" as *const u8 as
                         *const libc::c_char, 400i32);
            return
        }
        // This sets the Wayland clipboard (by calling wlr_seat_set_selection)
        xwm_selection_get_targets(selection);
    } else { xwm_selection_get_data(selection); };
}
#[no_mangle]
pub unsafe extern "C" fn xwm_handle_xfixes_selection_notify(mut xwm:
                                                                *mut wlr_xwm,
                                                            mut event:
                                                                *mut xcb_xfixes_selection_notify_event_t)
 -> libc::c_int {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_XFIXES_SELECTION_NOTIFY (selection=%u, owner=%u)\x00"
                 as *const u8 as *const libc::c_char,
             b"../xwayland/selection/incoming.c\x00" as *const u8 as
                 *const libc::c_char, 414i32, (*event).selection,
             (*event).owner);
    let mut selection: *mut wlr_xwm_selection =
        xwm_get_selection(xwm, (*event).selection);
    if selection.is_null() { return 0i32 }
    if (*event).owner == XCB_WINDOW_NONE as libc::c_int as libc::c_uint {
        if (*selection).owner != (*selection).window {
            // A real X client selection went away, not our
			// proxy selection
            if selection ==
                   &mut (*xwm).clipboard_selection as *mut wlr_xwm_selection {
                wlr_seat_request_set_selection((*xwm).seat,
                                               0 as *mut wlr_seat_client,
                                               0 as *mut wlr_data_source,
                                               wl_display_next_serial((*(*xwm).xwayland).wl_display));
            } else if selection ==
                          &mut (*xwm).primary_selection as
                              *mut wlr_xwm_selection {
                wlr_seat_request_set_primary_selection((*xwm).seat,
                                                       0 as
                                                           *mut wlr_seat_client,
                                                       0 as
                                                           *mut wlr_primary_selection_source,
                                                       wl_display_next_serial((*(*xwm).xwayland).wl_display));
            } else if !(selection ==
                            &mut (*xwm).dnd_selection as
                                *mut wlr_xwm_selection) {
                _wlr_log(WLR_DEBUG,
                         b"[%s:%d] X11 selection has been cleared, but cannot clear Wayland selection\x00"
                             as *const u8 as *const libc::c_char,
                         b"../xwayland/selection/incoming.c\x00" as *const u8
                             as *const libc::c_char, 436i32);
            }
        }
        (*selection).owner = XCB_WINDOW_NONE as libc::c_int as xcb_window_t;
        return 1i32
    }
    (*selection).owner = (*event).owner;
    // We have to use XCB_TIME_CURRENT_TIME when we claim the
	// selection, so grab the actual timestamp here so we can
	// answer TIMESTAMP conversion requests correctly.
    if (*event).owner == (*selection).window {
        (*selection).timestamp = (*event).timestamp;
        return 1i32
    }
    let mut transfer: *mut wlr_xwm_selection_transfer =
        &mut (*selection).incoming;
    (*transfer).incr = 0i32 != 0;
    // doing this will give a selection notify where we actually handle the sync
    xcb_convert_selection((*xwm).xcb_conn, (*selection).window,
                          (*selection).atom,
                          (*xwm).atoms[TARGETS as libc::c_int as usize],
                          (*xwm).atoms[WL_SELECTION as libc::c_int as usize],
                          (*event).timestamp);
    xcb_flush((*xwm).xcb_conn);
    return 1i32;
}
