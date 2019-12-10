use libc;
extern "C" {
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
    pub type wlr_primary_selection_source;
    pub type xcb_connection_t;
    pub type xcb_errors_context_t;
    pub type wlr_xwayland_cursor;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    // wlr_seat::{selection_offers,drag_offers}
    /* *
 * A data source implementation. Only the `send` function is mandatory. Refer to
 * the matching wl_data_source_* functions documentation to know what they do.
 */
    // source metadata
    // source status
    // drag'n'drop status
    // can be NULL
    // can be NULL
    // can be NULL
    // if WLR_DRAG_GRAB_TOUCH
    // wlr_drag_motion_event
    // wlr_drag_drop_event
    /* *
 * Create a wl data device manager global for this display.
 */
    /* *
 * Requests a selection to be set for the seat. If the request comes from
 * a client, then set `client` to be the matching seat client so that this
 * function can verify that the serial provided was once sent to the client
 * on this seat.
 */
    /* *
 * Sets the current selection for the seat. NULL can be provided to clear it.
 * This removes the previous one if there was any. In case the selection doesn't
 * come from a client, wl_display_next_serial() can be used to generate a
 * serial.
 */
    /* *
 * Creates a new drag. To request to start the drag, call
 * `wlr_seat_request_start_drag`.
 */
    /* *
 * Requests a drag to be started on the seat.
 */
    /* *
 * Starts a drag on the seat. This starts an implicit keyboard grab, but doesn't
 * start a pointer or a touch grab.
 */
    /* *
 * Starts a pointer drag on the seat. This starts implicit keyboard and pointer
 * grabs.
 */
    /* *
 * Starts a touch drag on the seat. This starts implicit keyboard and touch
 * grabs.
 */
    /* *
 * Initializes the data source with the provided implementation.
 */
    /* *
 * Sends the data as the specified MIME type over the passed file descriptor,
 * then close it.
 */
    /* *
 * Notifies the data source that a target accepts one of the offered MIME types.
 * If a target doesn't accept any of the offered types, `mime_type` is NULL.
 */
    /* *
 * Notifies the data source it is no longer valid and should be destroyed. That
 * destroys immediately the data source.
 */
    /* *
 * Notifies the data source that the drop operation was performed. This does not
 * indicate acceptance.
 *
 * The data source may still be used in the future and should not be destroyed
 * here.
 */
    /* *
 * Notifies the data source that the drag-and-drop operation concluded. That
 * potentially destroys immediately the data source.
 */
    /* *
 * Notifies the data source that a target accepts the drag with the specified
 * action.
 *
 * This shouldn't be called after `wlr_data_source_dnd_drop` unless the
 * drag-and-drop operation ended in an "ask" action.
 */
    #[no_mangle]
    fn wlr_data_source_dnd_action(source: *mut wlr_data_source,
                                  action: wl_data_device_manager_dnd_action);
    #[no_mangle]
    fn wlr_data_source_dnd_finish(source: *mut wlr_data_source);
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
    fn xwm_mime_type_to_atom(xwm: *mut wlr_xwm, mime_type: *mut libc::c_char)
     -> xcb_atom_t;
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
pub type size_t = libc::c_ulong;
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
    pub repeat_info: C2RustUnnamed_0,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_1,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub destroy: wl_signal,
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
pub struct wlr_data_source {
    pub impl_0: *const wlr_data_source_impl,
    pub mime_types: wl_array,
    pub actions: int32_t,
    pub accepted: bool,
    pub current_dnd_action: wl_data_device_manager_dnd_action,
    pub compositor_action: uint32_t,
    pub events: C2RustUnnamed_8,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
    pub events: C2RustUnnamed_9,
    pub point_destroy: wl_listener,
    pub source_destroy: wl_listener,
    pub seat_client_destroy: wl_listener,
    pub icon_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
    pub events: C2RustUnnamed_10,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub struct wlr_drag_motion_event {
    pub drag: *mut wlr_drag,
    pub time: uint32_t,
    pub sx: libc::c_double,
    pub sy: libc::c_double,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_drag_drop_event {
    pub drag: *mut wlr_drag,
    pub time: uint32_t,
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
#[repr ( C )]
pub union xcb_client_message_data_t {
    pub data8: [uint8_t; 20],
    pub data16: [uint16_t; 10],
    pub data32: [uint32_t; 5],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_client_message_event_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub type_0: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}
pub type xcb_prop_mode_t = libc::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
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
    pub events: C2RustUnnamed_11,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
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
    pub events: C2RustUnnamed_12,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
    pub events: C2RustUnnamed_13,
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
pub struct C2RustUnnamed_13 {
    pub ready: wl_signal,
    pub new_surface: wl_signal,
}
pub const DND_LEAVE: atom_name = 52;
pub const DND_DROP: atom_name = 53;
pub const DND_POSITION: atom_name = 50;
pub const DND_ACTION_ASK: atom_name = 59;
pub const DND_ACTION_MOVE: atom_name = 57;
pub const DND_ACTION_COPY: atom_name = 58;
pub const DND_ENTER: atom_name = 51;
pub const DND_TYPE_LIST: atom_name = 56;
pub type atom_name = libc::c_uint;
pub const ATOM_LAST: atom_name = 62;
pub const _NET_CLIENT_LIST: atom_name = 61;
pub const DND_ACTION_PRIVATE: atom_name = 60;
pub const DND_PROXY: atom_name = 55;
pub const DND_FINISHED: atom_name = 54;
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
pub const TIMESTAMP: atom_name = 35;
pub const TEXT: atom_name = 34;
pub const INCR: atom_name = 33;
pub const CLIPBOARD_MANAGER: atom_name = 32;
pub const TARGETS: atom_name = 31;
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
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
unsafe extern "C" fn data_device_manager_dnd_action_to_atom(mut xwm:
                                                                *mut wlr_xwm,
                                                            mut action:
                                                                wl_data_device_manager_dnd_action)
 -> xcb_atom_t {
    if action as libc::c_uint &
           WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY as libc::c_int as
               libc::c_uint != 0 {
        return (*xwm).atoms[DND_ACTION_COPY as libc::c_int as usize]
    } else {
        if action as libc::c_uint &
               WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE as libc::c_int as
                   libc::c_uint != 0 {
            return (*xwm).atoms[DND_ACTION_MOVE as libc::c_int as usize]
        } else {
            if action as libc::c_uint &
                   WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK as libc::c_int as
                       libc::c_uint != 0 {
                return (*xwm).atoms[DND_ACTION_ASK as libc::c_int as usize]
            }
        }
    }
    return XCB_ATOM_NONE as libc::c_int as xcb_atom_t;
}
unsafe extern "C" fn data_device_manager_dnd_action_from_atom(mut xwm:
                                                                  *mut wlr_xwm,
                                                              mut atom:
                                                                  atom_name)
 -> wl_data_device_manager_dnd_action {
    if atom as libc::c_uint ==
           (*xwm).atoms[DND_ACTION_COPY as libc::c_int as usize] ||
           atom as libc::c_uint ==
               (*xwm).atoms[DND_ACTION_PRIVATE as libc::c_int as usize] {
        return WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY
    } else {
        if atom as libc::c_uint ==
               (*xwm).atoms[DND_ACTION_MOVE as libc::c_int as usize] {
            return WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE
        } else {
            if atom as libc::c_uint ==
                   (*xwm).atoms[DND_ACTION_ASK as libc::c_int as usize] {
                return WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK
            }
        }
    }
    return WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE;
}
unsafe extern "C" fn xwm_dnd_send_event(mut xwm: *mut wlr_xwm,
                                        mut type_0: xcb_atom_t,
                                        mut data:
                                            *mut xcb_client_message_data_t) {
    let mut dest: *mut wlr_xwayland_surface = (*xwm).drag_focus;
    if !dest.is_null() {
    } else {
        __assert_fail(b"dest != NULL\x00" as *const u8 as *const libc::c_char,
                      b"../xwayland/selection/dnd.c\x00" as *const u8 as
                          *const libc::c_char, 42i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"void xwm_dnd_send_event(struct wlr_xwm *, xcb_atom_t, xcb_client_message_data_t *)\x00")).as_ptr());
    };
    let mut event: xcb_client_message_event_t =
        {
            let mut init =
                xcb_client_message_event_t{response_type: 33i32 as uint8_t,
                                           format: 32i32 as uint8_t,
                                           sequence: 0i32 as uint16_t,
                                           window: (*dest).window_id,
                                           type_0: type_0,
                                           data: *data,};
            init
        };
    xcb_send_event((*xwm).xcb_conn, 0i32 as uint8_t, (*dest).window_id,
                   XCB_EVENT_MASK_NO_EVENT as libc::c_int as uint32_t,
                   &mut event as *mut xcb_client_message_event_t as
                       *const libc::c_char);
    xcb_flush((*xwm).xcb_conn);
}
unsafe extern "C" fn xwm_dnd_send_enter(mut xwm: *mut wlr_xwm) {
    let mut drag: *mut wlr_drag = (*xwm).drag;
    if !drag.is_null() {
    } else {
        __assert_fail(b"drag != NULL\x00" as *const u8 as *const libc::c_char,
                      b"../xwayland/selection/dnd.c\x00" as *const u8 as
                          *const libc::c_char, 63i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"void xwm_dnd_send_enter(struct wlr_xwm *)\x00")).as_ptr());
    };
    let mut mime_types: *mut wl_array = &mut (*(*drag).source).mime_types;
    let mut data: xcb_client_message_data_t =
        xcb_client_message_data_t{data8:
                                      [0i32 as uint8_t, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
    data.data32[0] = (*xwm).dnd_window;
    data.data32[1] = (5i32 << 24i32) as uint32_t;
    // If we have 3 MIME types or less, we can send them directly in the
	// DND_ENTER message
    let mut n: size_t =
        (*mime_types).size.wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                            as libc::c_ulong);
    if n <= 3i32 as libc::c_ulong {
        let mut i: size_t = 0i32 as size_t;
        let mut mime_type_ptr: *mut *mut libc::c_char =
            0 as *mut *mut libc::c_char;
        mime_type_ptr = (*mime_types).data as *mut *mut libc::c_char;
        while (mime_type_ptr as *const libc::c_char) <
                  ((*mime_types).data as
                       *const libc::c_char).offset((*mime_types).size as
                                                       isize) {
            let mut mime_type: *mut libc::c_char = *mime_type_ptr;
            data.data32[(2i32 as libc::c_ulong).wrapping_add(i) as usize] =
                xwm_mime_type_to_atom(xwm, mime_type);
            i = i.wrapping_add(1);
            mime_type_ptr = mime_type_ptr.offset(1)
        }
    } else {
        // Let the client know that targets are not contained in the message
		// data and must be retrieved with the DND_TYPE_LIST property
        data.data32[1] |= 1i32 as libc::c_uint;
        let vla = n as usize;
        let mut targets: Vec<xcb_atom_t> = ::std::vec::from_elem(0, vla);
        let mut i_0: size_t = 0i32 as size_t;
        let mut mime_type_ptr_0: *mut *mut libc::c_char =
            0 as *mut *mut libc::c_char;
        mime_type_ptr_0 = (*mime_types).data as *mut *mut libc::c_char;
        while (mime_type_ptr_0 as *const libc::c_char) <
                  ((*mime_types).data as
                       *const libc::c_char).offset((*mime_types).size as
                                                       isize) {
            let mut mime_type_0: *mut libc::c_char = *mime_type_ptr_0;
            *targets.as_mut_ptr().offset(i_0 as isize) =
                xwm_mime_type_to_atom(xwm, mime_type_0);
            i_0 = i_0.wrapping_add(1);
            mime_type_ptr_0 = mime_type_ptr_0.offset(1)
        }
        xcb_change_property((*xwm).xcb_conn,
                            XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                            (*xwm).dnd_window,
                            (*xwm).atoms[DND_TYPE_LIST as libc::c_int as
                                             usize],
                            XCB_ATOM_ATOM as libc::c_int as xcb_atom_t,
                            32i32 as uint8_t, n as uint32_t,
                            targets.as_mut_ptr() as *const libc::c_void);
    }
    xwm_dnd_send_event(xwm, (*xwm).atoms[DND_ENTER as libc::c_int as usize],
                       &mut data);
}
unsafe extern "C" fn xwm_dnd_send_position(mut xwm: *mut wlr_xwm,
                                           mut time: uint32_t, mut x: int16_t,
                                           mut y: int16_t) {
    let mut drag: *mut wlr_drag = (*xwm).drag;
    if !drag.is_null() {
    } else {
        __assert_fail(b"drag != NULL\x00" as *const u8 as *const libc::c_char,
                      b"../xwayland/selection/dnd.c\x00" as *const u8 as
                          *const libc::c_char, 110i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"void xwm_dnd_send_position(struct wlr_xwm *, uint32_t, int16_t, int16_t)\x00")).as_ptr());
    };
    let mut data: xcb_client_message_data_t =
        xcb_client_message_data_t{data8:
                                      [0i32 as uint8_t, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
    data.data32[0] = (*xwm).dnd_window;
    data.data32[2] =
        ((x as libc::c_int) << 16i32 | y as libc::c_int) as uint32_t;
    data.data32[3] = time;
    data.data32[4] =
        data_device_manager_dnd_action_to_atom(xwm,
                                               (*(*drag).source).actions as
                                                   wl_data_device_manager_dnd_action);
    xwm_dnd_send_event(xwm,
                       (*xwm).atoms[DND_POSITION as libc::c_int as usize],
                       &mut data);
}
unsafe extern "C" fn xwm_dnd_send_drop(mut xwm: *mut wlr_xwm,
                                       mut time: uint32_t) {
    let mut drag: *mut wlr_drag = (*xwm).drag;
    if !drag.is_null() {
    } else {
        __assert_fail(b"drag != NULL\x00" as *const u8 as *const libc::c_char,
                      b"../xwayland/selection/dnd.c\x00" as *const u8 as
                          *const libc::c_char, 124i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void xwm_dnd_send_drop(struct wlr_xwm *, uint32_t)\x00")).as_ptr());
    };
    let mut dest: *mut wlr_xwayland_surface = (*xwm).drag_focus;
    if !dest.is_null() {
    } else {
        __assert_fail(b"dest != NULL\x00" as *const u8 as *const libc::c_char,
                      b"../xwayland/selection/dnd.c\x00" as *const u8 as
                          *const libc::c_char, 126i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void xwm_dnd_send_drop(struct wlr_xwm *, uint32_t)\x00")).as_ptr());
    };
    let mut data: xcb_client_message_data_t =
        xcb_client_message_data_t{data8:
                                      [0i32 as uint8_t, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
    data.data32[0] = (*xwm).dnd_window;
    data.data32[2] = time;
    xwm_dnd_send_event(xwm, (*xwm).atoms[DND_DROP as libc::c_int as usize],
                       &mut data);
}
unsafe extern "C" fn xwm_dnd_send_leave(mut xwm: *mut wlr_xwm) {
    let mut drag: *mut wlr_drag = (*xwm).drag;
    if !drag.is_null() {
    } else {
        __assert_fail(b"drag != NULL\x00" as *const u8 as *const libc::c_char,
                      b"../xwayland/selection/dnd.c\x00" as *const u8 as
                          *const libc::c_char, 137i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"void xwm_dnd_send_leave(struct wlr_xwm *)\x00")).as_ptr());
    };
    let mut dest: *mut wlr_xwayland_surface = (*xwm).drag_focus;
    if !dest.is_null() {
    } else {
        __assert_fail(b"dest != NULL\x00" as *const u8 as *const libc::c_char,
                      b"../xwayland/selection/dnd.c\x00" as *const u8 as
                          *const libc::c_char, 139i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"void xwm_dnd_send_leave(struct wlr_xwm *)\x00")).as_ptr());
    };
    let mut data: xcb_client_message_data_t =
        xcb_client_message_data_t{data8:
                                      [0i32 as uint8_t, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
    data.data32[0] = (*xwm).dnd_window;
    xwm_dnd_send_event(xwm, (*xwm).atoms[DND_LEAVE as libc::c_int as usize],
                       &mut data);
}
/* This is in xcb/xcb_event.h, but pulling xcb-util just for a constant
 * others redefine anyway is meh
 */
// wlr_xwayland_surface::link
// wlr_xwayland_surface::unpaired_link
/*static void xwm_dnd_send_finished(struct wlr_xwm *xwm) {
	struct wlr_drag *drag = xwm->drag;
	assert(drag != NULL);
	struct wlr_xwayland_surface *dest = xwm->drag_focus;
	assert(dest != NULL);

	xcb_client_message_data_t data = { 0 };
	data.data32[0] = xwm->dnd_window;
	data.data32[1] = drag->source->accepted;

	if (drag->source->accepted) {
		data.data32[2] = data_device_manager_dnd_action_to_atom(xwm,
			drag->source->current_dnd_action);
	}

	xwm_dnd_send_event(xwm, xwm->atoms[DND_FINISHED], &data);
}*/
#[no_mangle]
pub unsafe extern "C" fn xwm_handle_selection_client_message(mut xwm:
                                                                 *mut wlr_xwm,
                                                             mut ev:
                                                                 *mut xcb_client_message_event_t)
 -> libc::c_int {
    if (*ev).type_0 == (*xwm).atoms[DND_STATUS as libc::c_int as usize] {
        if (*xwm).drag.is_null() {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] ignoring XdndStatus client message because there\'s no drag\x00"
                         as *const u8 as *const libc::c_char,
                     b"../xwayland/selection/dnd.c\x00" as *const u8 as
                         *const libc::c_char, 170i32);
            return 1i32
        }
        let mut data: *mut xcb_client_message_data_t = &mut (*ev).data;
        let mut target_window: xcb_window_t = (*data).data32[0];
        let mut accepted: bool =
            (*data).data32[1] & 1i32 as libc::c_uint != 0;
        let mut action_atom: xcb_atom_t = (*data).data32[4];
        if (*xwm).drag_focus.is_null() ||
               target_window != (*(*xwm).drag_focus).window_id {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] ignoring XdndStatus client message because it doesn\'t match the current drag focus window ID\x00"
                         as *const u8 as *const libc::c_char,
                     b"../xwayland/selection/dnd.c\x00" as *const u8 as
                         *const libc::c_char, 182i32);
            return 1i32
        }
        let mut action: wl_data_device_manager_dnd_action =
            data_device_manager_dnd_action_from_atom(xwm,
                                                     action_atom as
                                                         atom_name);
        let mut drag: *mut wlr_drag = (*xwm).drag;
        if !drag.is_null() {
        } else {
            __assert_fail(b"drag != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"../xwayland/selection/dnd.c\x00" as *const u8 as
                              *const libc::c_char, 190i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 88],
                                                    &[libc::c_char; 88]>(b"int xwm_handle_selection_client_message(struct wlr_xwm *, xcb_client_message_event_t *)\x00")).as_ptr());
        };
        (*(*drag).source).accepted = accepted;
        wlr_data_source_dnd_action((*drag).source, action);
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] DND_STATUS window=%d accepted=%d action=%d\x00" as
                     *const u8 as *const libc::c_char,
                 b"../xwayland/selection/dnd.c\x00" as *const u8 as
                     *const libc::c_char, 196i32, target_window,
                 accepted as libc::c_int, action as libc::c_uint);
        return 1i32
    } else if (*ev).type_0 ==
                  (*xwm).atoms[DND_FINISHED as libc::c_int as usize] {
        // This should only happen after the drag has ended, but before the drag
		// source is destroyed
        if (*xwm).seat.is_null() || (*(*xwm).seat).drag_source.is_null() ||
               !(*xwm).drag.is_null() {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] ignoring XdndFinished client message because there\'s no finished drag\x00"
                         as *const u8 as *const libc::c_char,
                     b"../xwayland/selection/dnd.c\x00" as *const u8 as
                         *const libc::c_char, 204i32);
            return 1i32
        }
        let mut source: *mut wlr_data_source = (*(*xwm).seat).drag_source;
        let mut data_0: *mut xcb_client_message_data_t = &mut (*ev).data;
        let mut target_window_0: xcb_window_t = (*data_0).data32[0];
        let mut performed: bool =
            (*data_0).data32[1] & 1i32 as libc::c_uint != 0;
        let mut action_atom_0: xcb_atom_t = (*data_0).data32[2];
        if (*xwm).drag_focus.is_null() ||
               target_window_0 != (*(*xwm).drag_focus).window_id {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] ignoring XdndFinished client message because it doesn\'t match the finished drag focus window ID\x00"
                         as *const u8 as *const libc::c_char,
                     b"../xwayland/selection/dnd.c\x00" as *const u8 as
                         *const libc::c_char, 218i32);
            return 1i32
        }
        let mut action_0: wl_data_device_manager_dnd_action =
            data_device_manager_dnd_action_from_atom(xwm,
                                                     action_atom_0 as
                                                         atom_name);
        if performed { wlr_data_source_dnd_finish(source); }
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] DND_FINISH window=%d performed=%d action=%d\x00" as
                     *const u8 as *const libc::c_char,
                 b"../xwayland/selection/dnd.c\x00" as *const u8 as
                     *const libc::c_char, 230i32, target_window_0,
                 performed as libc::c_int, action_0 as libc::c_uint);
        return 1i32
    } else { return 0i32 };
}
unsafe extern "C" fn seat_handle_drag_focus(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut drag: *mut wlr_drag = data as *mut wlr_drag;
    let mut xwm: *mut wlr_xwm =
        (listener as *mut libc::c_char).offset(-1000) as *mut wlr_xwm;
    let mut focus: *mut wlr_xwayland_surface = 0 as *mut wlr_xwayland_surface;
    if !(*drag).focus.is_null() {
        // TODO: check for subsurfaces?
        let mut surface: *mut wlr_xwayland_surface =
            0 as *mut wlr_xwayland_surface;
        surface =
            ((*xwm).surfaces.next as *mut libc::c_char).offset(-24) as
                *mut wlr_xwayland_surface;
        while &mut (*surface).link as *mut wl_list !=
                  &mut (*xwm).surfaces as *mut wl_list {
            if (*surface).surface == (*drag).focus {
                focus = surface;
                break ;
            } else {
                surface =
                    ((*surface).link.next as *mut libc::c_char).offset(-24) as
                        *mut wlr_xwayland_surface
            }
        }
    }
    if focus == (*xwm).drag_focus { return }
    if !(*xwm).drag_focus.is_null() {
        wlr_data_source_dnd_action((*drag).source,
                                   WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE);
        xwm_dnd_send_leave(xwm);
    }
    (*xwm).drag_focus = focus;
    if !(*xwm).drag_focus.is_null() { xwm_dnd_send_enter(xwm); };
}
unsafe extern "C" fn seat_handle_drag_motion(mut listener: *mut wl_listener,
                                             mut data: *mut libc::c_void) {
    let mut xwm: *mut wlr_xwm =
        (listener as *mut libc::c_char).offset(-1024) as *mut wlr_xwm;
    let mut event: *mut wlr_drag_motion_event =
        data as *mut wlr_drag_motion_event;
    let mut surface: *mut wlr_xwayland_surface = (*xwm).drag_focus;
    if surface.is_null() {
        return
        // No xwayland surface focused
    }
    xwm_dnd_send_position(xwm, (*event).time,
                          ((*surface).x as libc::c_int +
                               (*event).sx as int16_t as libc::c_int) as
                              int16_t,
                          ((*surface).y as libc::c_int +
                               (*event).sy as int16_t as libc::c_int) as
                              int16_t);
}
unsafe extern "C" fn seat_handle_drag_drop(mut listener: *mut wl_listener,
                                           mut data: *mut libc::c_void) {
    let mut xwm: *mut wlr_xwm =
        (listener as *mut libc::c_char).offset(-1048) as *mut wlr_xwm;
    let mut event: *mut wlr_drag_drop_event =
        data as *mut wlr_drag_drop_event;
    if (*xwm).drag_focus.is_null() {
        return
        // No xwayland surface focused
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Wayland drag dropped over an Xwayland window\x00" as
                 *const u8 as *const libc::c_char,
             b"../xwayland/selection/dnd.c\x00" as *const u8 as
                 *const libc::c_char, 291i32);
    xwm_dnd_send_drop(xwm, (*event).time);
}
unsafe extern "C" fn seat_handle_drag_destroy(mut listener: *mut wl_listener,
                                              mut data: *mut libc::c_void) {
    let mut xwm: *mut wlr_xwm =
        (listener as *mut libc::c_char).offset(-1072) as *mut wlr_xwm;
    // Don't reset drag focus yet because the target will read the drag source
	// right after
    if !(*xwm).drag_focus.is_null() && !(*(*(*xwm).drag).source).accepted {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Wayland drag cancelled over an Xwayland window\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/selection/dnd.c\x00" as *const u8 as
                     *const libc::c_char, 301i32);
        xwm_dnd_send_leave(xwm);
    }
    wl_list_remove(&mut (*xwm).seat_drag_focus.link);
    wl_list_remove(&mut (*xwm).seat_drag_motion.link);
    wl_list_remove(&mut (*xwm).seat_drag_drop.link);
    wl_list_remove(&mut (*xwm).seat_drag_destroy.link);
    (*xwm).drag = 0 as *mut wlr_drag;
}
unsafe extern "C" fn seat_handle_drag_source_destroy(mut listener:
                                                         *mut wl_listener,
                                                     mut data:
                                                         *mut libc::c_void) {
    let mut xwm: *mut wlr_xwm =
        (listener as *mut libc::c_char).offset(-1096) as *mut wlr_xwm;
    wl_list_remove(&mut (*xwm).seat_drag_source_destroy.link);
    (*xwm).drag_focus = 0 as *mut wlr_xwayland_surface;
}
// when sending to x11
// when receiving from x11
#[no_mangle]
pub unsafe extern "C" fn xwm_seat_handle_start_drag(mut xwm: *mut wlr_xwm,
                                                    mut drag: *mut wlr_drag) {
    (*xwm).drag = drag;
    (*xwm).drag_focus = 0 as *mut wlr_xwayland_surface;
    if !drag.is_null() {
        wl_signal_add(&mut (*drag).events.focus, &mut (*xwm).seat_drag_focus);
        (*xwm).seat_drag_focus.notify =
            Some(seat_handle_drag_focus as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*drag).events.motion,
                      &mut (*xwm).seat_drag_motion);
        (*xwm).seat_drag_motion.notify =
            Some(seat_handle_drag_motion as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*drag).events.drop, &mut (*xwm).seat_drag_drop);
        (*xwm).seat_drag_drop.notify =
            Some(seat_handle_drag_drop as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*drag).events.destroy,
                      &mut (*xwm).seat_drag_destroy);
        (*xwm).seat_drag_destroy.notify =
            Some(seat_handle_drag_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*drag).source).events.destroy,
                      &mut (*xwm).seat_drag_source_destroy);
        (*xwm).seat_drag_source_destroy.notify =
            Some(seat_handle_drag_source_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ())
    };
}
