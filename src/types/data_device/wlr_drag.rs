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
    fn wl_display_next_serial(display: *mut wl_display) -> uint32_t;
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
    #[no_mangle]
    fn wl_resource_post_no_memory(resource: *mut wl_resource);
    #[no_mangle]
    fn wl_resource_get_link(resource: *mut wl_resource) -> *mut wl_list;
    #[no_mangle]
    fn wl_resource_from_link(resource: *mut wl_list) -> *mut wl_resource;
    #[no_mangle]
    fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;
    #[no_mangle]
    fn wl_resource_get_version(resource: *mut wl_resource) -> libc::c_int;
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    /* *
 * Whether or not this surface currently has an attached buffer. A surface has
 * an attached buffer when it commits with a non-null buffer in its pending
 * state. A surface will not have a buffer if it has never committed one, has
 * committed a null buffer, or something went wrong with uploading the buffer.
 */
    #[no_mangle]
    fn wlr_surface_has_buffer(surface: *mut wlr_surface) -> bool;
    /* *
 * Gets a wlr_seat_client for the specified client, or returns NULL if no
 * client is bound for that client.
 */
    #[no_mangle]
    fn wlr_seat_client_for_wl_client(wlr_seat: *mut wlr_seat,
                                     wl_client: *mut wl_client)
     -> *mut wlr_seat_client;
    /* *
 * Clear the focused surface for the pointer and leave all entered surfaces.
 */
    #[no_mangle]
    fn wlr_seat_pointer_clear_focus(wlr_seat: *mut wlr_seat);
    /* *
 * Start a grab of the pointer of this seat. The grabber is responsible for
 * handling all pointer events until the grab ends.
 */
    #[no_mangle]
    fn wlr_seat_pointer_start_grab(wlr_seat: *mut wlr_seat,
                                   grab: *mut wlr_seat_pointer_grab);
    /* *
 * End the grab of the pointer of this seat. This reverts the grab back to the
 * default grab for the pointer.
 */
    #[no_mangle]
    fn wlr_seat_pointer_end_grab(wlr_seat: *mut wlr_seat);
    /* *
 * Start a grab of the keyboard of this seat. The grabber is responsible for
 * handling all keyboard events until the grab ends.
 */
    #[no_mangle]
    fn wlr_seat_keyboard_start_grab(wlr_seat: *mut wlr_seat,
                                    grab: *mut wlr_seat_keyboard_grab);
    /* *
 * End the grab of the keyboard of this seat. This reverts the grab back to the
 * default grab for the keyboard.
 */
    #[no_mangle]
    fn wlr_seat_keyboard_end_grab(wlr_seat: *mut wlr_seat);
    /* *
 * Start a grab of the touch device of this seat. The grabber is responsible for
 * handling all touch events until the grab ends.
 */
    #[no_mangle]
    fn wlr_seat_touch_start_grab(wlr_seat: *mut wlr_seat,
                                 grab: *mut wlr_seat_touch_grab);
    /* *
 * End the grab of the touch device of this seat. This reverts the grab back to
 * the default grab for the touch device.
 */
    #[no_mangle]
    fn wlr_seat_touch_end_grab(wlr_seat: *mut wlr_seat);
    #[no_mangle]
    fn wlr_data_source_destroy(source: *mut wlr_data_source);
    #[no_mangle]
    fn wlr_data_source_dnd_drop(source: *mut wlr_data_source);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn data_offer_create(device_resource: *mut wl_resource,
                         source: *mut wlr_data_source,
                         type_0: wlr_data_offer_type) -> *mut wlr_data_offer;
    #[no_mangle]
    fn data_offer_update_action(offer: *mut wlr_data_offer);
    #[no_mangle]
    fn data_offer_destroy(offer: *mut wlr_data_offer);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type wl_fixed_t = int32_t;

#[repr ( C )]#[derive(Copy, Clone)]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: int64_t,
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type wl_pointer_button_state = libc::c_uint;
pub const WL_POINTER_BUTTON_STATE_PRESSED: wl_pointer_button_state = 1;
pub const WL_POINTER_BUTTON_STATE_RELEASED: wl_pointer_button_state = 0;
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
    pub texture: *mut crate::src::backend::drm::atomic::wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_surface {
    pub resource: *mut wl_resource,
    pub renderer: *mut crate::src::backend::drm::atomic::wlr_renderer,
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

#[repr(C)]#[derive(Copy, Clone)]
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

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
/* *
 * Contains state for a single client's bound wl_seat resource and can be used
 * to issue input events to that client. The lifetime of these objects is
 * managed by wlr_seat; some may be NULL.
 */

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
    pub events: C2RustUnnamed_3,
    pub serials: wlr_serial_ringset,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
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
    pub primary_selection_source: *mut crate::src::types::wlr_data_control_v1::wlr_primary_selection_source,
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

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_5,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
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
    pub events: C2RustUnnamed_6,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
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
    pub events: C2RustUnnamed_7,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
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
    pub events: C2RustUnnamed_8,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
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
    pub events: C2RustUnnamed_9,
    pub point_destroy: wl_listener,
    pub source_destroy: wl_listener,
    pub seat_client_destroy: wl_listener,
    pub icon_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
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
    pub events: C2RustUnnamed_10,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
    pub map: wl_signal,
    pub unmap: wl_signal,
    pub destroy: wl_signal,
}
pub type wlr_drag_grab_type = libc::c_uint;
pub const WLR_DRAG_GRAB_KEYBOARD_TOUCH: wlr_drag_grab_type = 2;
pub const WLR_DRAG_GRAB_KEYBOARD_POINTER: wlr_drag_grab_type = 1;
pub const WLR_DRAG_GRAB_KEYBOARD: wlr_drag_grab_type = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_request_start_drag_event {
    pub drag: *mut wlr_drag,
    pub origin: *mut wlr_surface,
    pub serial: uint32_t,
}
pub type wlr_data_offer_type = libc::c_uint;
pub const WLR_DATA_OFFER_DRAG: wlr_data_offer_type = 1;
pub const WLR_DATA_OFFER_SELECTION: wlr_data_offer_type = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_data_offer {
    pub resource: *mut wl_resource,
    pub source: *mut wlr_data_source,
    pub type_0: wlr_data_offer_type,
    pub link: wl_list,
    pub actions: uint32_t,
    pub preferred_action: wl_data_device_manager_dnd_action,
    pub in_ask: bool,
    pub source_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drag_motion_event {
    pub drag: *mut wlr_drag,
    pub time: uint32_t,
    pub sx: libc::c_double,
    pub sy: libc::c_double,
}

#[repr(C)]#[derive(Copy, Clone)]
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
#[inline]
unsafe extern "C" fn wl_data_offer_send_source_actions(mut resource_:
                                                           *mut wl_resource,
                                                       mut source_actions:
                                                           uint32_t) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, source_actions);
}
#[inline]
unsafe extern "C" fn wl_data_device_send_enter(mut resource_:
                                                   *mut wl_resource,
                                               mut serial: uint32_t,
                                               mut surface: *mut wl_resource,
                                               mut x: wl_fixed_t,
                                               mut y: wl_fixed_t,
                                               mut id: *mut wl_resource) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, serial, surface, x, y,
                           id);
}
#[inline]
unsafe extern "C" fn wl_data_device_send_leave(mut resource_:
                                                   *mut wl_resource) {
    wl_resource_post_event(resource_, 2i32 as uint32_t);
}
#[inline]
unsafe extern "C" fn wl_data_device_send_motion(mut resource_:
                                                    *mut wl_resource,
                                                mut time: uint32_t,
                                                mut x: wl_fixed_t,
                                                mut y: wl_fixed_t) {
    wl_resource_post_event(resource_, 3i32 as uint32_t, time, x, y);
}
#[inline]
unsafe extern "C" fn wl_data_device_send_drop(mut resource_:
                                                  *mut wl_resource) {
    wl_resource_post_event(resource_, 4i32 as uint32_t);
}
unsafe extern "C" fn drag_handle_seat_client_destroy(mut listener:
                                                         *mut wl_listener,
                                                     mut data:
                                                         *mut libc::c_void) {
    let mut drag: *mut wlr_drag =
        (listener as *mut libc::c_char).offset(-256) as *mut wlr_drag;
    (*drag).focus_client = 0 as *mut wlr_seat_client;
    wl_list_remove(&mut (*drag).seat_client_destroy.link);
}
unsafe extern "C" fn drag_set_focus(mut drag: *mut wlr_drag,
                                    mut surface: *mut wlr_surface,
                                    mut sx: libc::c_double,
                                    mut sy: libc::c_double) {
    let mut focus_client: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
    if (*drag).focus == surface { return }
    if !(*drag).focus_client.is_null() {
        wl_list_remove(&mut (*drag).seat_client_destroy.link);
        // If we're switching focus to another client, we want to destroy all
		// offers without destroying the source. If the drag operation ends, we
		// want to keep the offer around for the data transfer.
        let mut offer: *mut wlr_data_offer = 0 as *mut wlr_data_offer;
        let mut tmp: *mut wlr_data_offer = 0 as *mut wlr_data_offer;
        offer =
            ((*(*(*drag).focus_client).seat).drag_offers.next as
                 *mut libc::c_char).offset(-24) as *mut wlr_data_offer;
        tmp =
            ((*offer).link.next as *mut libc::c_char).offset(-24) as
                *mut wlr_data_offer;
        while &mut (*offer).link as *mut wl_list !=
                  &mut (*(*(*drag).focus_client).seat).drag_offers as
                      *mut wl_list {
            let mut client: *mut wl_client =
                wl_resource_get_client((*offer).resource);
            if !(*drag).dropped && (*offer).source == (*drag).source &&
                   client == (*(*drag).focus_client).client {
                (*offer).source = 0 as *mut wlr_data_source;
                data_offer_destroy(offer);
            }
            offer = tmp;
            tmp =
                ((*offer).link.next as *mut libc::c_char).offset(-24) as
                    *mut wlr_data_offer
        }
        let mut resource: *mut wl_resource = 0 as *mut wl_resource;
        resource = 0 as *mut wl_resource;
        resource =
            wl_resource_from_link((*(*drag).focus_client).data_devices.next);
        while wl_resource_get_link(resource) !=
                  &mut (*(*drag).focus_client).data_devices as *mut wl_list {
            wl_data_device_send_leave(resource);
            resource =
                wl_resource_from_link((*wl_resource_get_link(resource)).next)
        }
        (*drag).focus_client = 0 as *mut wlr_seat_client;
        (*drag).focus = 0 as *mut wlr_surface
    }
    if !surface.is_null() {
        if !((*drag).source.is_null() &&
                 wl_resource_get_client((*surface).resource) !=
                     (*(*drag).seat_client).client) {
            focus_client =
                wlr_seat_client_for_wl_client((*(*drag).seat_client).seat,
                                              wl_resource_get_client((*surface).resource));
            if !focus_client.is_null() {
                if !(*drag).source.is_null() {
                    (*(*drag).source).accepted = 0i32 != 0;
                    let mut serial: uint32_t =
                        wl_display_next_serial((*(*(*drag).seat_client).seat).display);
                    let mut device_resource: *mut wl_resource =
                        0 as *mut wl_resource;
                    device_resource = 0 as *mut wl_resource;
                    device_resource =
                        wl_resource_from_link((*focus_client).data_devices.next);
                    while wl_resource_get_link(device_resource) !=
                              &mut (*focus_client).data_devices as
                                  *mut wl_list {
                        let mut offer_0: *mut wlr_data_offer =
                            data_offer_create(device_resource, (*drag).source,
                                              WLR_DATA_OFFER_DRAG);
                        if offer_0.is_null() {
                            wl_resource_post_no_memory(device_resource);
                            return
                        }
                        data_offer_update_action(offer_0);
                        if wl_resource_get_version((*offer_0).resource) >=
                               3i32 {
                            wl_data_offer_send_source_actions((*offer_0).resource,
                                                              (*(*drag).source).actions
                                                                  as
                                                                  uint32_t);
                        }
                        wl_data_device_send_enter(device_resource, serial,
                                                  (*surface).resource,
                                                  wl_fixed_from_double(sx),
                                                  wl_fixed_from_double(sy),
                                                  (*offer_0).resource);
                        device_resource =
                            wl_resource_from_link((*wl_resource_get_link(device_resource)).next)
                    }
                }
                (*drag).focus = surface;
                (*drag).focus_client = focus_client;
                (*drag).seat_client_destroy.notify =
                    Some(drag_handle_seat_client_destroy as
                             unsafe extern "C" fn(_: *mut wl_listener,
                                                  _: *mut libc::c_void)
                                 -> ());
                wl_signal_add(&mut (*focus_client).events.destroy,
                              &mut (*drag).seat_client_destroy);
            }
        }
    }
    wlr_signal_emit_safe(&mut (*drag).events.focus,
                         drag as *mut libc::c_void);
}
unsafe extern "C" fn drag_icon_set_mapped(mut icon: *mut wlr_drag_icon,
                                          mut mapped: bool) {
    if mapped as libc::c_int != 0 && !(*icon).mapped {
        (*icon).mapped = 1i32 != 0;
        wlr_signal_emit_safe(&mut (*icon).events.map,
                             icon as *mut libc::c_void);
    } else if !mapped && (*icon).mapped as libc::c_int != 0 {
        wlr_signal_emit_safe(&mut (*icon).events.unmap,
                             icon as *mut libc::c_void);
        (*icon).mapped = 0i32 != 0
    };
}
unsafe extern "C" fn drag_destroy(mut drag: *mut wlr_drag) {
    if (*drag).cancelling { return }
    (*drag).cancelling = 1i32 != 0;
    wlr_signal_emit_safe(&mut (*drag).events.destroy,
                         drag as *mut libc::c_void);
    if (*drag).started {
        wlr_seat_keyboard_end_grab((*drag).seat);
        match (*drag).grab_type as libc::c_uint {
            1 => { wlr_seat_pointer_end_grab((*drag).seat); }
            2 => { wlr_seat_touch_end_grab((*drag).seat); }
            0 | _ => { }
        }
        drag_set_focus(drag, 0 as *mut wlr_surface, 0i32 as libc::c_double,
                       0i32 as libc::c_double);
        if (*(*drag).seat).drag == drag {
        } else {
            __assert_fail(b"drag->seat->drag == drag\x00" as *const u8 as
                              *const libc::c_char,
                          b"../types/data_device/wlr_drag.c\x00" as *const u8
                              as *const libc::c_char, 144i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 37],
                                                    &[libc::c_char; 37]>(b"void drag_destroy(struct wlr_drag *)\x00")).as_ptr());
        };
        (*(*drag).seat).drag = 0 as *mut wlr_drag
    }
    if !(*drag).source.is_null() {
        wl_list_remove(&mut (*drag).source_destroy.link);
    }
    drag_icon_destroy((*drag).icon);
    free(drag as *mut libc::c_void);
}
unsafe extern "C" fn drag_handle_pointer_enter(mut grab:
                                                   *mut wlr_seat_pointer_grab,
                                               mut surface: *mut wlr_surface,
                                               mut sx: libc::c_double,
                                               mut sy: libc::c_double) {
    let mut drag: *mut wlr_drag = (*grab).data as *mut wlr_drag;
    drag_set_focus(drag, surface, sx, sy);
}
unsafe extern "C" fn drag_handle_pointer_motion(mut grab:
                                                    *mut wlr_seat_pointer_grab,
                                                mut time: uint32_t,
                                                mut sx: libc::c_double,
                                                mut sy: libc::c_double) {
    let mut drag: *mut wlr_drag = (*grab).data as *mut wlr_drag;
    if !(*drag).focus.is_null() && !(*drag).focus_client.is_null() {
        let mut resource: *mut wl_resource = 0 as *mut wl_resource;
        resource = 0 as *mut wl_resource;
        resource =
            wl_resource_from_link((*(*drag).focus_client).data_devices.next);
        while wl_resource_get_link(resource) !=
                  &mut (*(*drag).focus_client).data_devices as *mut wl_list {
            wl_data_device_send_motion(resource, time,
                                       wl_fixed_from_double(sx),
                                       wl_fixed_from_double(sy));
            resource =
                wl_resource_from_link((*wl_resource_get_link(resource)).next)
        }
        let mut event: wlr_drag_motion_event =
            {
                let mut init =
                    wlr_drag_motion_event{drag: drag,
                                          time: time,
                                          sx: sx,
                                          sy: sy,};
                init
            };
        wlr_signal_emit_safe(&mut (*drag).events.motion,
                             &mut event as *mut wlr_drag_motion_event as
                                 *mut libc::c_void);
    };
}
unsafe extern "C" fn drag_drop(mut drag: *mut wlr_drag, mut time: uint32_t) {
    if !(*drag).focus_client.is_null() {
    } else {
        __assert_fail(b"drag->focus_client\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/data_device/wlr_drag.c\x00" as *const u8 as
                          *const libc::c_char, 183i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"void drag_drop(struct wlr_drag *, uint32_t)\x00")).as_ptr());
    };
    (*drag).dropped = 1i32 != 0;
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource =
        wl_resource_from_link((*(*drag).focus_client).data_devices.next);
    while wl_resource_get_link(resource) !=
              &mut (*(*drag).focus_client).data_devices as *mut wl_list {
        wl_data_device_send_drop(resource);
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    if !(*drag).source.is_null() { wlr_data_source_dnd_drop((*drag).source); }
    let mut event: wlr_drag_drop_event =
        { let mut init = wlr_drag_drop_event{drag: drag, time: time,}; init };
    wlr_signal_emit_safe(&mut (*drag).events.drop,
                         &mut event as *mut wlr_drag_drop_event as
                             *mut libc::c_void);
}
unsafe extern "C" fn drag_handle_pointer_button(mut grab:
                                                    *mut wlr_seat_pointer_grab,
                                                mut time: uint32_t,
                                                mut button: uint32_t,
                                                mut state: uint32_t)
 -> uint32_t {
    let mut drag: *mut wlr_drag = (*grab).data as *mut wlr_drag;
    if !(*drag).source.is_null() &&
           (*(*grab).seat).pointer_state.grab_button == button &&
           state ==
               WL_POINTER_BUTTON_STATE_RELEASED as libc::c_int as libc::c_uint
       {
        if !(*drag).focus_client.is_null() &&
               (*(*drag).source).current_dnd_action as libc::c_uint != 0 &&
               (*(*drag).source).accepted as libc::c_int != 0 {
            drag_drop(drag, time);
        } else if (*(*(*drag).source).impl_0).dnd_finish.is_some() {
            // This will end the grab and free `drag`
            wlr_data_source_destroy((*drag).source);
            return 0i32 as uint32_t
        }
    }
    if (*(*grab).seat).pointer_state.button_count == 0i32 as libc::c_ulong &&
           state ==
               WL_POINTER_BUTTON_STATE_RELEASED as libc::c_int as libc::c_uint
       {
        drag_destroy(drag);
    }
    return 0i32 as uint32_t;
}
unsafe extern "C" fn drag_handle_pointer_axis(mut grab:
                                                  *mut wlr_seat_pointer_grab,
                                              mut time: uint32_t,
                                              mut orientation:
                                                  wlr_axis_orientation,
                                              mut value: libc::c_double,
                                              mut value_discrete: int32_t,
                                              mut source: wlr_axis_source) {
    // This space is intentionally left blank
}
unsafe extern "C" fn drag_handle_pointer_cancel(mut grab:
                                                    *mut wlr_seat_pointer_grab) {
    let mut drag: *mut wlr_drag = (*grab).data as *mut wlr_drag;
    drag_destroy(drag);
}
static mut data_device_pointer_drag_interface: wlr_pointer_grab_interface =
    unsafe {
        {
            let mut init =
                wlr_pointer_grab_interface{enter:
                                               Some(drag_handle_pointer_enter
                                                        as
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
                                               Some(drag_handle_pointer_motion
                                                        as
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
                                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                       *mut wlr_seat_pointer_grab,
                                                                                                   _:
                                                                                                       uint32_t,
                                                                                                   _:
                                                                                                       uint32_t,
                                                                                                   _:
                                                                                                       uint32_t)
                                                                                  ->
                                                                                      uint32_t>,
                                                                       Option<unsafe extern "C" fn(_:
                                                                                                       *mut wlr_seat_pointer_grab,
                                                                                                   _:
                                                                                                       uint32_t,
                                                                                                   _:
                                                                                                       uint32_t,
                                                                                                   _:
                                                                                                       wlr_button_state)
                                                                                  ->
                                                                                      uint32_t>>(Some(drag_handle_pointer_button
                                                                                                          as
                                                                                                          unsafe extern "C" fn(_:
                                                                                                                                   *mut wlr_seat_pointer_grab,
                                                                                                                               _:
                                                                                                                                   uint32_t,
                                                                                                                               _:
                                                                                                                                   uint32_t,
                                                                                                                               _:
                                                                                                                                   uint32_t)
                                                                                                              ->
                                                                                                                  uint32_t)),
                                           axis:
                                               Some(drag_handle_pointer_axis
                                                        as
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
                                           frame: None,
                                           cancel:
                                               Some(drag_handle_pointer_cancel
                                                        as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wlr_seat_pointer_grab)
                                                            -> ()),};
            init
        }
    };
unsafe extern "C" fn drag_handle_touch_down(mut grab:
                                                *mut wlr_seat_touch_grab,
                                            mut time: uint32_t,
                                            mut point: *mut wlr_touch_point)
 -> uint32_t {
    // eat the event
    return 0i32 as uint32_t;
}
unsafe extern "C" fn drag_handle_touch_up(mut grab: *mut wlr_seat_touch_grab,
                                          mut time: uint32_t,
                                          mut point: *mut wlr_touch_point) {
    let mut drag: *mut wlr_drag = (*grab).data as *mut wlr_drag;
    if (*drag).grab_touch_id != (*point).touch_id { return }
    if !(*drag).focus_client.is_null() { drag_drop(drag, time); }
    drag_destroy(drag);
}
unsafe extern "C" fn drag_handle_touch_motion(mut grab:
                                                  *mut wlr_seat_touch_grab,
                                              mut time: uint32_t,
                                              mut point:
                                                  *mut wlr_touch_point) {
    let mut drag: *mut wlr_drag = (*grab).data as *mut wlr_drag;
    if !(*drag).focus.is_null() && !(*drag).focus_client.is_null() {
        let mut resource: *mut wl_resource = 0 as *mut wl_resource;
        resource = 0 as *mut wl_resource;
        resource =
            wl_resource_from_link((*(*drag).focus_client).data_devices.next);
        while wl_resource_get_link(resource) !=
                  &mut (*(*drag).focus_client).data_devices as *mut wl_list {
            wl_data_device_send_motion(resource, time,
                                       wl_fixed_from_double((*point).sx),
                                       wl_fixed_from_double((*point).sy));
            resource =
                wl_resource_from_link((*wl_resource_get_link(resource)).next)
        }
    };
}
unsafe extern "C" fn drag_handle_touch_enter(mut grab:
                                                 *mut wlr_seat_touch_grab,
                                             mut time: uint32_t,
                                             mut point:
                                                 *mut wlr_touch_point) {
    let mut drag: *mut wlr_drag = (*grab).data as *mut wlr_drag;
    drag_set_focus(drag, (*point).focus_surface, (*point).sx, (*point).sy);
}
unsafe extern "C" fn drag_handle_touch_cancel(mut grab:
                                                  *mut wlr_seat_touch_grab) {
    let mut drag: *mut wlr_drag = (*grab).data as *mut wlr_drag;
    drag_destroy(drag);
}
static mut data_device_touch_drag_interface: wlr_touch_grab_interface =
    {
    
        {
            let mut init =
                wlr_touch_grab_interface{down:
                                             Some(drag_handle_touch_down as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> uint32_t),
                                         up:
                                             Some(drag_handle_touch_up as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> ()),
                                         motion:
                                             Some(drag_handle_touch_motion as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> ()),
                                         enter:
                                             Some(drag_handle_touch_enter as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> ()),
                                         cancel:
                                             Some(drag_handle_touch_cancel as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab)
                                                          -> ()),};
            init
        }
};
unsafe extern "C" fn drag_handle_keyboard_enter(mut grab:
                                                    *mut wlr_seat_keyboard_grab,
                                                mut surface: *mut wlr_surface,
                                                mut keycodes: *mut uint32_t,
                                                mut num_keycodes: size_t,
                                                mut modifiers:
                                                    *mut wlr_keyboard_modifiers) {
    // nothing has keyboard focus during drags
}
unsafe extern "C" fn drag_handle_keyboard_key(mut grab:
                                                  *mut wlr_seat_keyboard_grab,
                                              mut time: uint32_t,
                                              mut key: uint32_t,
                                              mut state: uint32_t) {
    // no keyboard input during drags
}
unsafe extern "C" fn drag_handle_keyboard_modifiers(mut grab:
                                                        *mut wlr_seat_keyboard_grab,
                                                    mut modifiers:
                                                        *mut wlr_keyboard_modifiers) {
    //struct wlr_keyboard *keyboard = grab->seat->keyboard_state.keyboard;
	// TODO change the dnd action based on what modifier is pressed on the
	// keyboard
}
unsafe extern "C" fn drag_handle_keyboard_cancel(mut grab:
                                                     *mut wlr_seat_keyboard_grab) {
    let mut drag: *mut wlr_drag = (*grab).data as *mut wlr_drag;
    drag_destroy(drag);
}
static mut data_device_keyboard_drag_interface: wlr_keyboard_grab_interface =
    {
    
        {
            let mut init =
                wlr_keyboard_grab_interface{enter:
                                                Some(drag_handle_keyboard_enter
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut wlr_seat_keyboard_grab,
                                                                              _:
                                                                                  *mut wlr_surface,
                                                                              _:
                                                                                  *mut uint32_t,
                                                                              _:
                                                                                  size_t,
                                                                              _:
                                                                                  *mut wlr_keyboard_modifiers)
                                                             -> ()),
                                            key:
                                                Some(drag_handle_keyboard_key
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut wlr_seat_keyboard_grab,
                                                                              _:
                                                                                  uint32_t,
                                                                              _:
                                                                                  uint32_t,
                                                                              _:
                                                                                  uint32_t)
                                                             -> ()),
                                            modifiers:
                                                Some(drag_handle_keyboard_modifiers
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut wlr_seat_keyboard_grab,
                                                                              _:
                                                                                  *mut wlr_keyboard_modifiers)
                                                             -> ()),
                                            cancel:
                                                Some(drag_handle_keyboard_cancel
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut wlr_seat_keyboard_grab)
                                                             -> ()),};
            init
        }
};
unsafe extern "C" fn drag_handle_icon_destroy(mut listener: *mut wl_listener,
                                              mut data: *mut libc::c_void) {
    let mut drag: *mut wlr_drag =
        (listener as *mut libc::c_char).offset(-280) as *mut wlr_drag;
    (*drag).icon = 0 as *mut wlr_drag_icon;
}
unsafe extern "C" fn drag_handle_drag_source_destroy(mut listener:
                                                         *mut wl_listener,
                                                     mut data:
                                                         *mut libc::c_void) {
    let mut drag: *mut wlr_drag =
        (listener as *mut libc::c_char).offset(-232) as *mut wlr_drag;
    drag_destroy(drag);
}
unsafe extern "C" fn drag_icon_destroy(mut icon: *mut wlr_drag_icon) {
    if icon.is_null() { return }
    drag_icon_set_mapped(icon, 0i32 != 0);
    wlr_signal_emit_safe(&mut (*icon).events.destroy,
                         icon as *mut libc::c_void);
    (*(*icon).surface).role_data = 0 as *mut libc::c_void;
    wl_list_remove(&mut (*icon).surface_destroy.link);
    free(icon as *mut libc::c_void);
}
unsafe extern "C" fn drag_icon_handle_surface_destroy(mut listener:
                                                          *mut wl_listener,
                                                      mut data:
                                                          *mut libc::c_void) {
    let mut icon: *mut wlr_drag_icon =
        (listener as *mut libc::c_char).offset(-72) as *mut wlr_drag_icon;
    drag_icon_destroy(icon);
}
unsafe extern "C" fn drag_icon_surface_role_commit(mut surface:
                                                       *mut wlr_surface) {
    if (*surface).role == &drag_icon_surface_role as *const wlr_surface_role {
    } else {
        __assert_fail(b"surface->role == &drag_icon_surface_role\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/data_device/wlr_drag.c\x00" as *const u8 as
                          *const libc::c_char, 362i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"void drag_icon_surface_role_commit(struct wlr_surface *)\x00")).as_ptr());
    };
    let mut icon: *mut wlr_drag_icon =
        (*surface).role_data as *mut wlr_drag_icon;
    if icon.is_null() { return }
    drag_icon_set_mapped(icon, wlr_surface_has_buffer(surface));
}
#[no_mangle]
pub static mut drag_icon_surface_role: wlr_surface_role =
    {
    
        {
            let mut init =
                wlr_surface_role{name:
                                     b"wl_data_device-icon\x00" as *const u8
                                         as *const libc::c_char,
                                 commit:
                                     Some(drag_icon_surface_role_commit as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_surface)
                                                  -> ()),
                                 precommit: None,};
            init
        }
};
unsafe extern "C" fn drag_icon_create(mut drag: *mut wlr_drag,
                                      mut surface: *mut wlr_surface)
 -> *mut wlr_drag_icon {
    let mut icon: *mut wlr_drag_icon =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_drag_icon>() as libc::c_ulong) as
            *mut wlr_drag_icon;
    if icon.is_null() { return 0 as *mut wlr_drag_icon }
    (*icon).drag = drag;
    (*icon).surface = surface;
    wl_signal_init(&mut (*icon).events.map);
    wl_signal_init(&mut (*icon).events.unmap);
    wl_signal_init(&mut (*icon).events.destroy);
    wl_signal_add(&mut (*(*icon).surface).events.destroy,
                  &mut (*icon).surface_destroy);
    (*icon).surface_destroy.notify =
        Some(drag_icon_handle_surface_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    (*(*icon).surface).role_data = icon as *mut libc::c_void;
    if wlr_surface_has_buffer(surface) {
        drag_icon_set_mapped(icon, 1i32 != 0);
    }
    return icon;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_drag_create(mut seat_client:
                                             *mut wlr_seat_client,
                                         mut source: *mut wlr_data_source,
                                         mut icon_surface: *mut wlr_surface)
 -> *mut wlr_drag {
    let mut drag: *mut wlr_drag =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_drag>() as libc::c_ulong) as
            *mut wlr_drag;
    if drag.is_null() { return 0 as *mut wlr_drag }
    wl_signal_init(&mut (*drag).events.focus);
    wl_signal_init(&mut (*drag).events.motion);
    wl_signal_init(&mut (*drag).events.drop);
    wl_signal_init(&mut (*drag).events.destroy);
    (*drag).seat = (*seat_client).seat;
    (*drag).seat_client = seat_client;
    if !icon_surface.is_null() {
        let mut icon: *mut wlr_drag_icon =
            drag_icon_create(drag, icon_surface);
        if icon.is_null() {
            free(drag as *mut libc::c_void);
            return 0 as *mut wlr_drag
        }
        (*drag).icon = icon;
        (*drag).icon_destroy.notify =
            Some(drag_handle_icon_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*icon).events.destroy, &mut (*drag).icon_destroy);
    }
    (*drag).source = source;
    if !source.is_null() {
        (*drag).source_destroy.notify =
            Some(drag_handle_drag_source_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*source).events.destroy,
                      &mut (*drag).source_destroy);
    }
    (*drag).pointer_grab.data = drag as *mut libc::c_void;
    (*drag).pointer_grab.interface = &data_device_pointer_drag_interface;
    (*drag).touch_grab.data = drag as *mut libc::c_void;
    (*drag).touch_grab.interface = &data_device_touch_drag_interface;
    (*drag).keyboard_grab.data = drag as *mut libc::c_void;
    (*drag).keyboard_grab.interface = &data_device_keyboard_drag_interface;
    return drag;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_request_start_drag(mut seat: *mut wlr_seat,
                                                     mut drag: *mut wlr_drag,
                                                     mut origin:
                                                         *mut wlr_surface,
                                                     mut serial: uint32_t) {
    if (*drag).seat == seat {
    } else {
        __assert_fail(b"drag->seat == seat\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/data_device/wlr_drag.c\x00" as *const u8 as
                          *const libc::c_char, 451i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 103],
                                                &[libc::c_char; 103]>(b"void wlr_seat_request_start_drag(struct wlr_seat *, struct wlr_drag *, struct wlr_surface *, uint32_t)\x00")).as_ptr());
    };
    if !(*seat).drag.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Rejecting start_drag request, another drag-and-drop operation is already in progress\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/data_device/wlr_drag.c\x00" as *const u8 as
                     *const libc::c_char, 455i32);
        return
    }
    let mut event: wlr_seat_request_start_drag_event =
        {
            let mut init =
                wlr_seat_request_start_drag_event{drag: drag,
                                                  origin: origin,
                                                  serial: serial,};
            init
        };
    wlr_signal_emit_safe(&mut (*seat).events.request_start_drag,
                         &mut event as *mut wlr_seat_request_start_drag_event
                             as *mut libc::c_void);
}
unsafe extern "C" fn seat_handle_drag_source_destroy(mut listener:
                                                         *mut wl_listener,
                                                     mut data:
                                                         *mut libc::c_void) {
    let mut seat: *mut wlr_seat =
        (listener as *mut libc::c_char).offset(-616) as *mut wlr_seat;
    wl_list_remove(&mut (*seat).drag_source_destroy.link);
    (*seat).drag_source = 0 as *mut wlr_data_source;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_start_drag(mut seat: *mut wlr_seat,
                                             mut drag: *mut wlr_drag,
                                             mut serial: uint32_t) {
    if (*drag).seat == seat {
    } else {
        __assert_fail(b"drag->seat == seat\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/data_device/wlr_drag.c\x00" as *const u8 as
                          *const libc::c_char, 477i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"void wlr_seat_start_drag(struct wlr_seat *, struct wlr_drag *, uint32_t)\x00")).as_ptr());
    };
    if !(*drag).started {
    } else {
        __assert_fail(b"!drag->started\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/data_device/wlr_drag.c\x00" as *const u8 as
                          *const libc::c_char, 478i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"void wlr_seat_start_drag(struct wlr_seat *, struct wlr_drag *, uint32_t)\x00")).as_ptr());
    };
    (*drag).started = 1i32 != 0;
    wlr_seat_keyboard_start_grab(seat, &mut (*drag).keyboard_grab);
    (*seat).drag = drag;
    (*seat).drag_serial = serial;
    // We need to destroy the previous source, because listeners only expect one
	// active drag source at a time.
    wlr_data_source_destroy((*seat).drag_source);
    (*seat).drag_source = (*drag).source;
    if !(*drag).source.is_null() {
        (*seat).drag_source_destroy.notify =
            Some(seat_handle_drag_source_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*drag).source).events.destroy,
                      &mut (*seat).drag_source_destroy);
    }
    wlr_signal_emit_safe(&mut (*seat).events.start_drag,
                         drag as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_start_pointer_drag(mut seat: *mut wlr_seat,
                                                     mut drag: *mut wlr_drag,
                                                     mut serial: uint32_t) {
    (*drag).grab_type = WLR_DRAG_GRAB_KEYBOARD_POINTER;
    wlr_seat_pointer_clear_focus(seat);
    wlr_seat_pointer_start_grab(seat, &mut (*drag).pointer_grab);
    wlr_seat_start_drag(seat, drag, serial);
}
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
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_start_touch_drag(mut seat: *mut wlr_seat,
                                                   mut drag: *mut wlr_drag,
                                                   mut serial: uint32_t,
                                                   mut point:
                                                       *mut wlr_touch_point) {
    (*drag).grab_type = WLR_DRAG_GRAB_KEYBOARD_TOUCH;
    (*drag).grab_touch_id = (*seat).touch_state.grab_id as int32_t;
    (*drag).touch_id = (*point).touch_id;
    wlr_seat_touch_start_grab(seat, &mut (*drag).touch_grab);
    drag_set_focus(drag, (*point).surface, (*point).sx, (*point).sy);
    wlr_seat_start_drag(seat, drag, serial);
}
