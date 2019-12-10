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
    fn wl_display_add_destroy_listener(display: *mut wl_display,
                                       listener: *mut wl_listener);
    #[no_mangle]
    fn wl_global_create(display: *mut wl_display,
                        interface: *const wl_interface, version: libc::c_int,
                        data: *mut libc::c_void, bind: wl_global_bind_func_t)
     -> *mut wl_global;
    #[no_mangle]
    fn wl_global_destroy(global: *mut wl_global);
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
    #[no_mangle]
    fn wl_resource_post_no_memory(resource: *mut wl_resource);
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
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wlr_data_source_destroy(source: *mut wlr_data_source);
    #[no_mangle]
    fn wlr_seat_request_start_drag(seat: *mut wlr_seat, drag: *mut wlr_drag,
                                   origin: *mut wlr_surface,
                                   serial: uint32_t);
    #[no_mangle]
    fn wlr_drag_create(seat_client: *mut wlr_seat_client,
                       source: *mut wlr_data_source,
                       icon_surface: *mut wlr_surface) -> *mut wlr_drag;
    #[no_mangle]
    static wl_data_device_interface: wl_interface;
    #[no_mangle]
    static wl_data_device_manager_interface: wl_interface;
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
    #[no_mangle]
    fn wlr_seat_client_validate_event_serial(client: *mut wlr_seat_client,
                                             serial: uint32_t) -> bool;
    /* *
 * Get a seat client from a seat resource. Returns NULL if inert.
 */
    #[no_mangle]
    fn wlr_seat_client_from_resource(resource: *mut wl_resource)
     -> *mut wlr_seat_client;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    static drag_icon_surface_role: wlr_surface_role;
    #[no_mangle]
    fn data_offer_create(device_resource: *mut wl_resource,
                         source: *mut wlr_data_source,
                         type_0: wlr_data_offer_type) -> *mut wlr_data_offer;
    #[no_mangle]
    fn data_offer_destroy(offer: *mut wlr_data_offer);
    #[no_mangle]
    fn client_data_source_create(client: *mut wl_client, version: uint32_t,
                                 id: uint32_t, resource_list: *mut wl_list)
     -> *mut wlr_client_data_source;
    #[no_mangle]
    fn client_data_source_from_resource(resource: *mut wl_resource)
     -> *mut wlr_client_data_source;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
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
pub type wl_global_bind_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_client, _: *mut libc::c_void,
                                _: uint32_t, _: uint32_t) -> ()>;
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
pub type wl_data_device_error = libc::c_uint;
pub const WL_DATA_DEVICE_ERROR_ROLE: wl_data_device_error = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_data_device_interface {
    pub start_drag: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                _: *mut wl_resource,
                                                _: *mut wl_resource,
                                                _: *mut wl_resource,
                                                _: *mut wl_resource,
                                                _: uint32_t) -> ()>,
    pub set_selection: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                   _: *mut wl_resource,
                                                   _: *mut wl_resource,
                                                   _: uint32_t) -> ()>,
    pub release: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
}
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_data_device_manager_interface {
    pub create_data_source: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                        _: *mut wl_resource,
                                                        _: uint32_t) -> ()>,
    pub get_data_device: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                     _: *mut wl_resource,
                                                     _: uint32_t,
                                                     _: *mut wl_resource)
                                    -> ()>,
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
    pub events: C2RustUnnamed_1,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
    pub events: C2RustUnnamed_2,
    pub serials: wlr_serial_ringset,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub events: C2RustUnnamed_3,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
    pub events: C2RustUnnamed_4,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub events: C2RustUnnamed_5,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub events: C2RustUnnamed_6,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub events: C2RustUnnamed_7,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub events: C2RustUnnamed_8,
    pub point_destroy: wl_listener,
    pub source_destroy: wl_listener,
    pub seat_client_destroy: wl_listener,
    pub icon_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
    pub events: C2RustUnnamed_9,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct wlr_seat_request_set_selection_event {
    pub source: *mut wlr_data_source,
    pub serial: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_data_device_manager {
    pub global: *mut wl_global,
    pub data_sources: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_10,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub destroy: wl_signal,
}
pub type wlr_data_offer_type = libc::c_uint;
pub const WLR_DATA_OFFER_DRAG: wlr_data_offer_type = 1;
pub const WLR_DATA_OFFER_SELECTION: wlr_data_offer_type = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_client_data_source {
    pub source: wlr_data_source,
    pub impl_0: wlr_data_source_impl,
    pub resource: *mut wl_resource,
    pub finalized: bool,
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
unsafe extern "C" fn wl_data_device_send_selection(mut resource_:
                                                       *mut wl_resource,
                                                   mut id: *mut wl_resource) {
    wl_resource_post_event(resource_, 5i32 as uint32_t, id);
}
#[no_mangle]
pub unsafe extern "C" fn seat_client_from_data_device_resource(mut resource:
                                                                   *mut wl_resource)
 -> *mut wlr_seat_client {
    if wl_resource_instance_of(resource, &wl_data_device_interface,
                               &data_device_impl as
                                   *const wl_data_device_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_data_device_interface, &data_device_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/data_device/wlr_data_device.c\x00" as
                          *const u8 as *const libc::c_char,
                      20i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[libc::c_char; 84]>(b"struct wlr_seat_client *seat_client_from_data_device_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_seat_client;
}
unsafe extern "C" fn data_device_set_selection(mut client: *mut wl_client,
                                               mut device_resource:
                                                   *mut wl_resource,
                                               mut source_resource:
                                                   *mut wl_resource,
                                               mut serial: uint32_t) {
    let mut seat_client: *mut wlr_seat_client =
        seat_client_from_data_device_resource(device_resource);
    if seat_client.is_null() { return }
    let mut source: *mut wlr_client_data_source =
        0 as *mut wlr_client_data_source;
    if !source_resource.is_null() {
        source = client_data_source_from_resource(source_resource)
    }
    if !source.is_null() { (*source).finalized = 1i32 != 0 }
    let mut wlr_source: *mut wlr_data_source =
        if !source.is_null() {
            &mut (*source).source
        } else { 0 as *mut wlr_data_source };
    wlr_seat_request_set_selection((*seat_client).seat, seat_client,
                                   wlr_source, serial);
}
unsafe extern "C" fn data_device_start_drag(mut client: *mut wl_client,
                                            mut device_resource:
                                                *mut wl_resource,
                                            mut source_resource:
                                                *mut wl_resource,
                                            mut origin_resource:
                                                *mut wl_resource,
                                            mut icon_resource:
                                                *mut wl_resource,
                                            mut serial: uint32_t) {
    let mut seat_client: *mut wlr_seat_client =
        seat_client_from_data_device_resource(device_resource);
    if seat_client.is_null() { return }
    let mut origin: *mut wlr_surface =
        wlr_surface_from_resource(origin_resource);
    let mut source: *mut wlr_client_data_source =
        0 as *mut wlr_client_data_source;
    if !source_resource.is_null() {
        source = client_data_source_from_resource(source_resource)
    }
    let mut icon: *mut wlr_surface = 0 as *mut wlr_surface;
    if !icon_resource.is_null() {
        icon = wlr_surface_from_resource(icon_resource);
        if !wlr_surface_set_role(icon, &drag_icon_surface_role,
                                 0 as *mut libc::c_void, icon_resource,
                                 WL_DATA_DEVICE_ERROR_ROLE as libc::c_int as
                                     uint32_t) {
            return
        }
    }
    let mut wlr_source: *mut wlr_data_source =
        if !source.is_null() {
            &mut (*source).source
        } else { 0 as *mut wlr_data_source };
    let mut drag: *mut wlr_drag =
        wlr_drag_create(seat_client, wlr_source, icon);
    if drag.is_null() { wl_resource_post_no_memory(device_resource); return }
    if !source.is_null() { (*source).finalized = 1i32 != 0 }
    wlr_seat_request_start_drag((*seat_client).seat, drag, origin, serial);
}
unsafe extern "C" fn data_device_release(mut client: *mut wl_client,
                                         mut resource: *mut wl_resource) {
    wl_resource_destroy(resource);
}
static mut data_device_impl: wl_data_device_interface =
    unsafe {
        {
            let mut init =
                wl_data_device_interface{start_drag:
                                             Some(data_device_start_drag as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wl_client,
                                                                           _:
                                                                               *mut wl_resource,
                                                                           _:
                                                                               *mut wl_resource,
                                                                           _:
                                                                               *mut wl_resource,
                                                                           _:
                                                                               *mut wl_resource,
                                                                           _:
                                                                               uint32_t)
                                                          -> ()),
                                         set_selection:
                                             Some(data_device_set_selection as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wl_client,
                                                                           _:
                                                                               *mut wl_resource,
                                                                           _:
                                                                               *mut wl_resource,
                                                                           _:
                                                                               uint32_t)
                                                          -> ()),
                                         release:
                                             Some(data_device_release as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wl_client,
                                                                           _:
                                                                               *mut wl_resource)
                                                          -> ()),};
            init
        }
    };
unsafe extern "C" fn data_device_handle_resource_destroy(mut resource:
                                                             *mut wl_resource) {
    wl_list_remove(wl_resource_get_link(resource));
}
unsafe extern "C" fn device_resource_send_selection(mut device_resource:
                                                        *mut wl_resource) {
    let mut seat_client: *mut wlr_seat_client =
        seat_client_from_data_device_resource(device_resource);
    if !seat_client.is_null() {
    } else {
        __assert_fail(b"seat_client != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/data_device/wlr_data_device.c\x00" as
                          *const u8 as *const libc::c_char,
                      108i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"void device_resource_send_selection(struct wl_resource *)\x00")).as_ptr());
    };
    let mut source: *mut wlr_data_source =
        (*(*seat_client).seat).selection_source;
    if !source.is_null() {
        let mut offer: *mut wlr_data_offer =
            data_offer_create(device_resource, source,
                              WLR_DATA_OFFER_SELECTION);
        if offer.is_null() {
            wl_client_post_no_memory((*seat_client).client);
            return
        }
        wl_data_device_send_selection(device_resource, (*offer).resource);
    } else {
        wl_data_device_send_selection(device_resource, 0 as *mut wl_resource);
    };
}
/* *
 * Creates a new wl_data_offer if there is a wl_data_source currently set as
 * the seat selection and sends it to the seat client, followed by the
 * wl_data_device.selection() event.  If there is no current selection, the
 * wl_data_device.selection() event will carry a NULL wl_data_offer.  If the
 * client does not have a wl_data_device for the seat nothing will be done.
 */
#[no_mangle]
pub unsafe extern "C" fn seat_client_send_selection(mut seat_client:
                                                        *mut wlr_seat_client) {
    let mut source: *mut wlr_data_source =
        (*(*seat_client).seat).selection_source;
    if !source.is_null() { (*source).accepted = 0i32 != 0 }
    // Make all current offers inert
    let mut offer: *mut wlr_data_offer = 0 as *mut wlr_data_offer;
    let mut tmp: *mut wlr_data_offer = 0 as *mut wlr_data_offer;
    offer =
        ((*(*seat_client).seat).selection_offers.next as
             *mut libc::c_char).offset(-24) as *mut wlr_data_offer;
    tmp =
        ((*offer).link.next as *mut libc::c_char).offset(-24) as
            *mut wlr_data_offer;
    while &mut (*offer).link as *mut wl_list !=
              &mut (*(*seat_client).seat).selection_offers as *mut wl_list {
        data_offer_destroy(offer);
        offer = tmp;
        tmp =
            ((*offer).link.next as *mut libc::c_char).offset(-24) as
                *mut wlr_data_offer
    }
    let mut device_resource: *mut wl_resource = 0 as *mut wl_resource;
    device_resource = 0 as *mut wl_resource;
    device_resource = wl_resource_from_link((*seat_client).data_devices.next);
    while wl_resource_get_link(device_resource) !=
              &mut (*seat_client).data_devices as *mut wl_list {
        device_resource_send_selection(device_resource);
        device_resource =
            wl_resource_from_link((*wl_resource_get_link(device_resource)).next)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_request_set_selection(mut seat:
                                                            *mut wlr_seat,
                                                        mut client:
                                                            *mut wlr_seat_client,
                                                        mut source:
                                                            *mut wlr_data_source,
                                                        mut serial:
                                                            uint32_t) {
    if !client.is_null() &&
           !wlr_seat_client_validate_event_serial(client, serial) {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Rejecting set_selection request, serial %u was never given to client\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/data_device/wlr_data_device.c\x00" as *const u8 as
                     *const libc::c_char, 149i32, serial);
        return
    }
    if !(*seat).selection_source.is_null() &&
           serial.wrapping_sub((*seat).selection_serial) >
               4294967295u32.wrapping_div(2i32 as libc::c_uint) {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Rejecting set_selection request, serial indicates superseded (%u < %u)\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/data_device/wlr_data_device.c\x00" as *const u8 as
                     *const libc::c_char, 156i32, serial,
                 (*seat).selection_serial);
        return
    }
    let mut event: wlr_seat_request_set_selection_event =
        {
            let mut init =
                wlr_seat_request_set_selection_event{source: source,
                                                     serial: serial,};
            init
        };
    wlr_signal_emit_safe(&mut (*seat).events.request_set_selection,
                         &mut event as
                             *mut wlr_seat_request_set_selection_event as
                             *mut libc::c_void);
}
unsafe extern "C" fn seat_handle_selection_source_destroy(mut listener:
                                                              *mut wl_listener,
                                                          mut data:
                                                              *mut libc::c_void) {
    let mut seat: *mut wlr_seat =
        (listener as *mut libc::c_char).offset(-568) as *mut wlr_seat;
    wl_list_remove(&mut (*seat).selection_source_destroy.link);
    (*seat).selection_source = 0 as *mut wlr_data_source;
    let mut focused_client: *mut wlr_seat_client =
        (*seat).keyboard_state.focused_client;
    if !focused_client.is_null() {
        seat_client_send_selection(focused_client);
    }
    wlr_signal_emit_safe(&mut (*seat).events.set_selection,
                         seat as *mut libc::c_void);
}
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
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_set_selection(mut seat: *mut wlr_seat,
                                                mut source:
                                                    *mut wlr_data_source,
                                                mut serial: uint32_t) {
    if (*seat).selection_source == source {
        (*seat).selection_serial = serial;
        return
    }
    if !(*seat).selection_source.is_null() {
        wl_list_remove(&mut (*seat).selection_source_destroy.link);
        wlr_data_source_destroy((*seat).selection_source);
        (*seat).selection_source = 0 as *mut wlr_data_source
    }
    (*seat).selection_source = source;
    (*seat).selection_serial = serial;
    if !source.is_null() {
        (*seat).selection_source_destroy.notify =
            Some(seat_handle_selection_source_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*source).events.destroy,
                      &mut (*seat).selection_source_destroy);
    }
    let mut focused_client: *mut wlr_seat_client =
        (*seat).keyboard_state.focused_client;
    if !focused_client.is_null() {
        seat_client_send_selection(focused_client);
    }
    wlr_signal_emit_safe(&mut (*seat).events.set_selection,
                         seat as *mut libc::c_void);
}
unsafe extern "C" fn data_device_manager_from_resource(mut resource:
                                                           *mut wl_resource)
 -> *mut wlr_data_device_manager {
    if wl_resource_instance_of(resource, &wl_data_device_manager_interface,
                               &data_device_manager_impl as
                                   *const wl_data_device_manager_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_data_device_manager_interface, &data_device_manager_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/data_device/wlr_data_device.c\x00" as
                          *const u8 as *const libc::c_char,
                      222i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 88],
                                                &[libc::c_char; 88]>(b"struct wlr_data_device_manager *data_device_manager_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as
               *mut wlr_data_device_manager;
}
unsafe extern "C" fn data_device_manager_get_data_device(mut client:
                                                             *mut wl_client,
                                                         mut manager_resource:
                                                             *mut wl_resource,
                                                         mut id: uint32_t,
                                                         mut seat_resource:
                                                             *mut wl_resource) {
    let mut seat_client: *mut wlr_seat_client =
        wlr_seat_client_from_resource(seat_resource);
    let mut version: uint32_t =
        wl_resource_get_version(manager_resource) as uint32_t;
    let mut resource: *mut wl_resource =
        wl_resource_create(client, &wl_data_device_interface,
                           version as libc::c_int, id);
    if resource.is_null() {
        wl_resource_post_no_memory(manager_resource);
        return
    }
    wl_resource_set_implementation(resource,
                                   &data_device_impl as
                                       *const wl_data_device_interface as
                                       *const libc::c_void,
                                   seat_client as *mut libc::c_void,
                                   Some(data_device_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_insert(&mut (*seat_client).data_devices,
                   wl_resource_get_link(resource));
    let mut focused_client: *mut wlr_seat_client =
        (*(*seat_client).seat).keyboard_state.focused_client;
    if focused_client == seat_client {
        device_resource_send_selection(resource);
    };
}
unsafe extern "C" fn data_device_manager_create_data_source(mut client:
                                                                *mut wl_client,
                                                            mut manager_resource:
                                                                *mut wl_resource,
                                                            mut id:
                                                                uint32_t) {
    let mut manager: *mut wlr_data_device_manager =
        data_device_manager_from_resource(manager_resource);
    client_data_source_create(client,
                              wl_resource_get_version(manager_resource) as
                                  uint32_t, id, &mut (*manager).data_sources);
}
static mut data_device_manager_impl: wl_data_device_manager_interface =
    unsafe {
        {
            let mut init =
                wl_data_device_manager_interface{create_data_source:
                                                     Some(data_device_manager_create_data_source
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut wl_client,
                                                                                   _:
                                                                                       *mut wl_resource,
                                                                                   _:
                                                                                       uint32_t)
                                                                  -> ()),
                                                 get_data_device:
                                                     Some(data_device_manager_get_data_device
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut wl_client,
                                                                                   _:
                                                                                       *mut wl_resource,
                                                                                   _:
                                                                                       uint32_t,
                                                                                   _:
                                                                                       *mut wl_resource)
                                                                  -> ()),};
            init
        }
    };
unsafe extern "C" fn data_device_manager_bind(mut client: *mut wl_client,
                                              mut data: *mut libc::c_void,
                                              mut version: uint32_t,
                                              mut id: uint32_t) {
    let mut manager: *mut wlr_data_device_manager =
        data as *mut wlr_data_device_manager;
    let mut resource: *mut wl_resource =
        wl_resource_create(client, &wl_data_device_manager_interface,
                           version as libc::c_int, id);
    if resource.is_null() { wl_client_post_no_memory(client); return }
    wl_resource_set_implementation(resource,
                                   &data_device_manager_impl as
                                       *const wl_data_device_manager_interface
                                       as *const libc::c_void,
                                   manager as *mut libc::c_void, None);
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut manager: *mut wlr_data_device_manager =
        (listener as *mut libc::c_char).offset(-24) as
            *mut wlr_data_device_manager;
    wlr_signal_emit_safe(&mut (*manager).events.destroy,
                         manager as *mut libc::c_void);
    wl_list_remove(&mut (*manager).display_destroy.link);
    wl_global_destroy((*manager).global);
    free(manager as *mut libc::c_void);
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
#[no_mangle]
pub unsafe extern "C" fn wlr_data_device_manager_create(mut display:
                                                            *mut wl_display)
 -> *mut wlr_data_device_manager {
    let mut manager: *mut wlr_data_device_manager =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_data_device_manager>() as
                   libc::c_ulong) as *mut wlr_data_device_manager;
    if manager.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] could not create data device manager\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/data_device/wlr_data_device.c\x00" as *const u8 as
                     *const libc::c_char, 293i32);
        return 0 as *mut wlr_data_device_manager
    }
    wl_list_init(&mut (*manager).data_sources);
    wl_signal_init(&mut (*manager).events.destroy);
    (*manager).global =
        wl_global_create(display, &wl_data_device_manager_interface, 3i32,
                         manager as *mut libc::c_void,
                         Some(data_device_manager_bind as
                                  unsafe extern "C" fn(_: *mut wl_client,
                                                       _: *mut libc::c_void,
                                                       _: uint32_t,
                                                       _: uint32_t) -> ()));
    if (*manager).global.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] could not create data device manager wl_global\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/data_device/wlr_data_device.c\x00" as *const u8 as
                     *const libc::c_char, 304i32);
        free(manager as *mut libc::c_void);
        return 0 as *mut wlr_data_device_manager
    }
    (*manager).display_destroy.notify =
        Some(handle_display_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_display_add_destroy_listener(display, &mut (*manager).display_destroy);
    return manager;
}
