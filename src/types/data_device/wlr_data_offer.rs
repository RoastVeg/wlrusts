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
    fn ffs(__i: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
    #[no_mangle]
    fn wl_resource_post_error(resource: *mut wl_resource, code: uint32_t,
                              msg: *const libc::c_char, _: ...);
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
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
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
    fn wlr_data_source_destroy(source: *mut wlr_data_source);
    #[no_mangle]
    fn wlr_data_source_accept(source: *mut wlr_data_source, serial: uint32_t,
                              mime_type: *const libc::c_char);
    #[no_mangle]
    static wl_data_offer_interface: wl_interface;
    #[no_mangle]
    fn wlr_data_source_send(source: *mut wlr_data_source,
                            mime_type: *const libc::c_char, fd: int32_t);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn seat_client_from_data_device_resource(resource: *mut wl_resource)
     -> *mut wlr_seat_client;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
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
    pub repeat_info: C2RustUnnamed_0,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
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
pub type wl_data_offer_error = libc::c_uint;
pub const WL_DATA_OFFER_ERROR_INVALID_OFFER: wl_data_offer_error = 3;
pub const WL_DATA_OFFER_ERROR_INVALID_ACTION: wl_data_offer_error = 2;
pub const WL_DATA_OFFER_ERROR_INVALID_ACTION_MASK: wl_data_offer_error = 1;
pub const WL_DATA_OFFER_ERROR_INVALID_FINISH: wl_data_offer_error = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_data_offer_interface {
    pub accept: Option<unsafe extern "C" fn(_: *mut wl_client,
                                            _: *mut wl_resource, _: uint32_t,
                                            _: *const libc::c_char) -> ()>,
    pub receive: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource,
                                             _: *const libc::c_char,
                                             _: int32_t) -> ()>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub finish: Option<unsafe extern "C" fn(_: *mut wl_client,
                                            _: *mut wl_resource) -> ()>,
    pub set_actions: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                 _: *mut wl_resource,
                                                 _: uint32_t, _: uint32_t)
                                -> ()>,
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
    pub events: C2RustUnnamed_1,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_2,
    pub serials: wlr_serial_ringset,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
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
    pub events: C2RustUnnamed_3,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_4,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
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
    pub events: C2RustUnnamed_5,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
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
    pub events: C2RustUnnamed_6,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
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
    pub events: C2RustUnnamed_7,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
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
    pub events: C2RustUnnamed_8,
    pub point_destroy: wl_listener,
    pub source_destroy: wl_listener,
    pub seat_client_destroy: wl_listener,
    pub icon_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
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
    pub events: C2RustUnnamed_9,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub map: wl_signal,
    pub unmap: wl_signal,
    pub destroy: wl_signal,
}
pub type wlr_drag_grab_type = libc::c_uint;
pub const WLR_DRAG_GRAB_KEYBOARD_TOUCH: wlr_drag_grab_type = 2;
pub const WLR_DRAG_GRAB_KEYBOARD_POINTER: wlr_drag_grab_type = 1;
pub const WLR_DRAG_GRAB_KEYBOARD: wlr_drag_grab_type = 0;
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
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
#[inline]
unsafe extern "C" fn wl_data_offer_send_offer(mut resource_: *mut wl_resource,
                                              mut mime_type:
                                                  *const libc::c_char) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, mime_type);
}
#[inline]
unsafe extern "C" fn wl_data_offer_send_action(mut resource_:
                                                   *mut wl_resource,
                                               mut dnd_action: uint32_t) {
    wl_resource_post_event(resource_, 2i32 as uint32_t, dnd_action);
}
#[inline]
unsafe extern "C" fn wl_data_device_send_data_offer(mut resource_:
                                                        *mut wl_resource,
                                                    mut id:
                                                        *mut wl_resource) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, id);
}
unsafe extern "C" fn data_offer_from_resource(mut resource: *mut wl_resource)
 -> *mut wlr_data_offer {
    if wl_resource_instance_of(resource, &wl_data_offer_interface,
                               &data_offer_impl as
                                   *const wl_data_offer_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_data_offer_interface, &data_offer_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/data_device/wlr_data_offer.c\x00" as
                          *const u8 as *const libc::c_char,
                      18i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"struct wlr_data_offer *data_offer_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_data_offer;
}
unsafe extern "C" fn data_offer_choose_action(mut offer: *mut wlr_data_offer)
 -> uint32_t {
    let mut offer_actions: uint32_t = 0;
    let mut preferred_action: uint32_t = 0i32 as uint32_t;
    if wl_resource_get_version((*offer).resource) >= 3i32 {
        offer_actions = (*offer).actions;
        preferred_action = (*offer).preferred_action as uint32_t
    } else {
        offer_actions =
            WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY as libc::c_int as uint32_t
    }
    let mut source_actions: uint32_t = 0;
    if (*(*offer).source).actions >= 0i32 {
        source_actions = (*(*offer).source).actions as uint32_t
    } else {
        source_actions =
            WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY as libc::c_int as uint32_t
    }
    let mut available_actions: uint32_t = offer_actions & source_actions;
    if available_actions == 0 {
        return WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE as libc::c_int as
                   uint32_t
    }
    if (*(*offer).source).compositor_action & available_actions != 0 {
        return (*(*offer).source).compositor_action
    }
    // If the dest side has a preferred DnD action, use it
    if preferred_action & available_actions != 0i32 as libc::c_uint {
        return preferred_action
    }
    // Use the first found action, in bit order
    return (1i32 << ffs(available_actions as libc::c_int) - 1i32) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn data_offer_update_action(mut offer:
                                                      *mut wlr_data_offer) {
    if (*offer).type_0 as libc::c_uint ==
           WLR_DATA_OFFER_DRAG as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"offer->type == WLR_DATA_OFFER_DRAG\x00" as *const u8
                          as *const libc::c_char,
                      b"../types/data_device/wlr_data_offer.c\x00" as
                          *const u8 as *const libc::c_char,
                      58i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void data_offer_update_action(struct wlr_data_offer *)\x00")).as_ptr());
    };
    let mut action: uint32_t = data_offer_choose_action(offer);
    if (*(*offer).source).current_dnd_action as libc::c_uint == action {
        return
    }
    (*(*offer).source).current_dnd_action =
        action as wl_data_device_manager_dnd_action;
    if (*offer).in_ask { return }
    wlr_data_source_dnd_action((*offer).source,
                               action as wl_data_device_manager_dnd_action);
    if wl_resource_get_version((*offer).resource) >= 3i32 {
        wl_data_offer_send_action((*offer).resource, action);
    };
}
unsafe extern "C" fn data_offer_handle_accept(mut client: *mut wl_client,
                                              mut resource: *mut wl_resource,
                                              mut serial: uint32_t,
                                              mut mime_type:
                                                  *const libc::c_char) {
    let mut offer: *mut wlr_data_offer = data_offer_from_resource(resource);
    if offer.is_null() { return }
    if (*offer).type_0 as libc::c_uint !=
           WLR_DATA_OFFER_DRAG as libc::c_int as libc::c_uint {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Ignoring wl_data_offer.accept request on a non-drag-and-drop offer\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/data_device/wlr_data_offer.c\x00" as *const u8 as
                     *const libc::c_char, 87i32);
        return
    }
    wlr_data_source_accept((*offer).source, serial, mime_type);
}
unsafe extern "C" fn data_offer_handle_receive(mut client: *mut wl_client,
                                               mut resource: *mut wl_resource,
                                               mut mime_type:
                                                   *const libc::c_char,
                                               mut fd: int32_t) {
    let mut offer: *mut wlr_data_offer = data_offer_from_resource(resource);
    if offer.is_null() { close(fd); return }
    wlr_data_source_send((*offer).source, mime_type, fd);
}
unsafe extern "C" fn data_offer_source_dnd_finish(mut offer:
                                                      *mut wlr_data_offer) {
    let mut source: *mut wlr_data_source = (*offer).source;
    if (*source).actions < 0i32 { return }
    if (*offer).in_ask {
        wlr_data_source_dnd_action(source, (*source).current_dnd_action);
    }
    wlr_data_source_dnd_finish(source);
}
unsafe extern "C" fn data_offer_handle_destroy(mut client: *mut wl_client,
                                               mut resource:
                                                   *mut wl_resource) {
    wl_resource_destroy(resource);
}
unsafe extern "C" fn data_offer_handle_finish(mut client: *mut wl_client,
                                              mut resource:
                                                  *mut wl_resource) {
    let mut offer: *mut wlr_data_offer = data_offer_from_resource(resource);
    if offer.is_null() { return }
    // TODO: also fail while we have a drag-and-drop grab
    if (*offer).type_0 as libc::c_uint !=
           WLR_DATA_OFFER_DRAG as libc::c_int as libc::c_uint {
        wl_resource_post_error((*offer).resource,
                               WL_DATA_OFFER_ERROR_INVALID_FINISH as
                                   libc::c_int as uint32_t,
                               b"Offer is not drag-and-drop\x00" as *const u8
                                   as *const libc::c_char);
        return
    }
    if !(*(*offer).source).accepted {
        wl_resource_post_error((*offer).resource,
                               WL_DATA_OFFER_ERROR_INVALID_FINISH as
                                   libc::c_int as uint32_t,
                               b"Premature finish request\x00" as *const u8 as
                                   *const libc::c_char);
        return
    }
    let mut action: wl_data_device_manager_dnd_action =
        (*(*offer).source).current_dnd_action;
    if action as libc::c_uint ==
           WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE as libc::c_int as
               libc::c_uint ||
           action as libc::c_uint ==
               WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK as libc::c_int as
                   libc::c_uint {
        wl_resource_post_error((*offer).resource,
                               WL_DATA_OFFER_ERROR_INVALID_FINISH as
                                   libc::c_int as uint32_t,
                               b"Offer finished with an invalid action\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    data_offer_source_dnd_finish(offer);
    data_offer_destroy(offer);
}
unsafe extern "C" fn data_offer_handle_set_actions(mut client: *mut wl_client,
                                                   mut resource:
                                                       *mut wl_resource,
                                                   mut actions: uint32_t,
                                                   mut preferred_action:
                                                       uint32_t) {
    let mut offer: *mut wlr_data_offer = data_offer_from_resource(resource);
    if offer.is_null() { return }
    if actions &
           !(WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY as libc::c_int |
                 WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE as libc::c_int |
                 WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK as libc::c_int) as
               libc::c_uint != 0 {
        wl_resource_post_error((*offer).resource,
                               WL_DATA_OFFER_ERROR_INVALID_ACTION_MASK as
                                   libc::c_int as uint32_t,
                               b"invalid action mask %x\x00" as *const u8 as
                                   *const libc::c_char, actions);
        return
    }
    if preferred_action != 0 &&
           (preferred_action & actions == 0 ||
                preferred_action.count_ones() as i32 > 1i32) {
        wl_resource_post_error((*offer).resource,
                               WL_DATA_OFFER_ERROR_INVALID_ACTION as
                                   libc::c_int as uint32_t,
                               b"invalid action %x\x00" as *const u8 as
                                   *const libc::c_char, preferred_action);
        return
    }
    if (*offer).type_0 as libc::c_uint !=
           WLR_DATA_OFFER_DRAG as libc::c_int as libc::c_uint {
        wl_resource_post_error((*offer).resource,
                               WL_DATA_OFFER_ERROR_INVALID_OFFER as
                                   libc::c_int as uint32_t,
                               b"set_action can only be sent to drag-and-drop offers\x00"
                                   as *const u8 as *const libc::c_char);
        return
    }
    (*offer).actions = actions;
    (*offer).preferred_action =
        preferred_action as wl_data_device_manager_dnd_action;
    data_offer_update_action(offer);
}
#[no_mangle]
pub unsafe extern "C" fn data_offer_destroy(mut offer: *mut wlr_data_offer) {
    if offer.is_null() { return }
    wl_list_remove(&mut (*offer).source_destroy.link);
    wl_list_remove(&mut (*offer).link);
    if (*offer).type_0 as libc::c_uint ==
           WLR_DATA_OFFER_DRAG as libc::c_int as libc::c_uint &&
           !(*offer).source.is_null() {
        // If the drag destination has version < 3, wl_data_offer.finish
		// won't be called, so do this here as a safety net, because
		// we still want the version >= 3 drag source to be happy.
        if wl_resource_get_version((*offer).resource) < 3i32 {
            data_offer_source_dnd_finish(offer);
        } else if (*(*(*offer).source).impl_0).dnd_finish.is_some() {
            wlr_data_source_destroy((*offer).source);
        }
    }
    // Make the resource inert
    wl_resource_set_user_data((*offer).resource, 0 as *mut libc::c_void);
    free(offer as *mut libc::c_void);
}
static mut data_offer_impl: wl_data_offer_interface =
    {
    
        {
            let mut init =
                wl_data_offer_interface{accept:
                                            Some(data_offer_handle_accept as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource,
                                                                          _:
                                                                              uint32_t,
                                                                          _:
                                                                              *const libc::c_char)
                                                         -> ()),
                                        receive:
                                            Some(data_offer_handle_receive as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource,
                                                                          _:
                                                                              *const libc::c_char,
                                                                          _:
                                                                              int32_t)
                                                         -> ()),
                                        destroy:
                                            Some(data_offer_handle_destroy as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource)
                                                         -> ()),
                                        finish:
                                            Some(data_offer_handle_finish as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource)
                                                         -> ()),
                                        set_actions:
                                            Some(data_offer_handle_set_actions
                                                     as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource,
                                                                          _:
                                                                              uint32_t,
                                                                          _:
                                                                              uint32_t)
                                                         -> ()),};
            init
        }
};
unsafe extern "C" fn data_offer_handle_resource_destroy(mut resource:
                                                            *mut wl_resource) {
    let mut offer: *mut wlr_data_offer = data_offer_from_resource(resource);
    data_offer_destroy(offer);
}
unsafe extern "C" fn data_offer_handle_source_destroy(mut listener:
                                                          *mut wl_listener,
                                                      mut data:
                                                          *mut libc::c_void) {
    let mut offer: *mut wlr_data_offer =
        (listener as *mut libc::c_char).offset(-56) as *mut wlr_data_offer;
    // Prevent data_offer_destroy from destroying the source again
    (*offer).source =
        0 as *mut wlr_data_source; // a NULL source means no selection
    data_offer_destroy(offer);
}
#[no_mangle]
pub unsafe extern "C" fn data_offer_create(mut device_resource:
                                               *mut wl_resource,
                                           mut source: *mut wlr_data_source,
                                           mut type_0: wlr_data_offer_type)
 -> *mut wlr_data_offer {
    let mut seat_client: *mut wlr_seat_client =
        seat_client_from_data_device_resource(device_resource);
    if !seat_client.is_null() {
    } else {
        __assert_fail(b"seat_client != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/data_device/wlr_data_offer.c\x00" as
                          *const u8 as *const libc::c_char,
                      243i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 115],
                                                &[libc::c_char; 115]>(b"struct wlr_data_offer *data_offer_create(struct wl_resource *, struct wlr_data_source *, enum wlr_data_offer_type)\x00")).as_ptr());
    };
    if !source.is_null() {
    } else {
        __assert_fail(b"source != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/data_device/wlr_data_offer.c\x00" as
                          *const u8 as *const libc::c_char,
                      244i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 115],
                                                &[libc::c_char; 115]>(b"struct wlr_data_offer *data_offer_create(struct wl_resource *, struct wlr_data_source *, enum wlr_data_offer_type)\x00")).as_ptr());
    };
    let mut offer: *mut wlr_data_offer =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_data_offer>() as libc::c_ulong) as
            *mut wlr_data_offer;
    if offer.is_null() { return 0 as *mut wlr_data_offer }
    (*offer).source = source;
    (*offer).type_0 = type_0;
    let mut client: *mut wl_client = wl_resource_get_client(device_resource);
    let mut version: uint32_t =
        wl_resource_get_version(device_resource) as uint32_t;
    (*offer).resource =
        wl_resource_create(client, &wl_data_offer_interface,
                           version as libc::c_int, 0i32 as uint32_t);
    if (*offer).resource.is_null() {
        free(offer as *mut libc::c_void);
        return 0 as *mut wlr_data_offer
    }
    wl_resource_set_implementation((*offer).resource,
                                   &data_offer_impl as
                                       *const wl_data_offer_interface as
                                       *const libc::c_void,
                                   offer as *mut libc::c_void,
                                   Some(data_offer_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    match type_0 as libc::c_uint {
        0 => {
            wl_list_insert(&mut (*(*seat_client).seat).selection_offers,
                           &mut (*offer).link);
        }
        1 => {
            wl_list_insert(&mut (*(*seat_client).seat).drag_offers,
                           &mut (*offer).link);
        }
        _ => { }
    }
    (*offer).source_destroy.notify =
        Some(data_offer_handle_source_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*source).events.destroy,
                  &mut (*offer).source_destroy);
    wl_data_device_send_data_offer(device_resource, (*offer).resource);
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    p = (*source).mime_types.data as *mut *mut libc::c_char;
    while (p as *const libc::c_char) <
              ((*source).mime_types.data as
                   *const libc::c_char).offset((*source).mime_types.size as
                                                   isize) {
        wl_data_offer_send_offer((*offer).resource, *p);
        p = p.offset(1)
    }
    return offer;
}
