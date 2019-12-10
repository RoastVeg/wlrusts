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
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_array_init(array: *mut wl_array);
    #[no_mangle]
    fn wl_array_release(array: *mut wl_array);
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
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
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
pub struct wlr_primary_selection_source {
    pub impl_0: *const wlr_primary_selection_source_impl,
    pub mime_types: wl_array,
    pub events: C2RustUnnamed_7,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_seat_request_set_primary_selection_event {
    pub source: *mut wlr_primary_selection_source,
    pub serial: uint32_t,
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
#[no_mangle]
pub unsafe extern "C" fn wlr_primary_selection_source_init(mut source:
                                                               *mut wlr_primary_selection_source,
                                                           mut impl_0:
                                                               *const wlr_primary_selection_source_impl) {
    if (*impl_0).send.is_some() {
    } else {
        __assert_fail(b"impl->send\x00" as *const u8 as *const libc::c_char,
                      b"../types/wlr_primary_selection.c\x00" as *const u8 as
                          *const libc::c_char, 10i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 128],
                                                &[libc::c_char; 128]>(b"void wlr_primary_selection_source_init(struct wlr_primary_selection_source *, const struct wlr_primary_selection_source_impl *)\x00")).as_ptr());
    };
    wl_array_init(&mut (*source).mime_types);
    wl_signal_init(&mut (*source).events.destroy);
    (*source).impl_0 = impl_0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_primary_selection_source_destroy(mut source:
                                                                  *mut wlr_primary_selection_source) {
    if source.is_null() { return }
    wlr_signal_emit_safe(&mut (*source).events.destroy,
                         source as *mut libc::c_void);
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    p = (*source).mime_types.data as *mut *mut libc::c_char;
    while (p as *const libc::c_char) <
              ((*source).mime_types.data as
                   *const libc::c_char).offset((*source).mime_types.size as
                                                   isize) {
        free(*p as *mut libc::c_void);
        p = p.offset(1)
    }
    wl_array_release(&mut (*source).mime_types);
    if (*(*source).impl_0).destroy.is_some() {
        (*(*source).impl_0).destroy.expect("non-null function pointer")(source);
    } else { free(source as *mut libc::c_void); };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_primary_selection_source_send(mut source:
                                                               *mut wlr_primary_selection_source,
                                                           mut mime_type:
                                                               *const libc::c_char,
                                                           mut fd: int32_t) {
    (*(*source).impl_0).send.expect("non-null function pointer")(source,
                                                                 mime_type,
                                                                 fd);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_request_set_primary_selection(mut seat:
                                                                    *mut wlr_seat,
                                                                mut client:
                                                                    *mut wlr_seat_client,
                                                                mut source:
                                                                    *mut wlr_primary_selection_source,
                                                                mut serial:
                                                                    uint32_t) {
    if !client.is_null() &&
           !wlr_seat_client_validate_event_serial(client, serial) {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Rejecting set_primary_selection request, serial %u was never given to client\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/wlr_primary_selection.c\x00" as *const u8 as
                     *const libc::c_char, 49i32, serial);
        return
    }
    if !(*seat).primary_selection_source.is_null() &&
           serial.wrapping_sub((*seat).primary_selection_serial) >
               4294967295u32.wrapping_div(2i32 as libc::c_uint) {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Rejecting set_primary_selection request, serial indicates superseded (%u < %u)\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/wlr_primary_selection.c\x00" as *const u8 as
                     *const libc::c_char, 57i32, serial,
                 (*seat).primary_selection_serial);
        return
    }
    let mut event: wlr_seat_request_set_primary_selection_event =
        {
            let mut init =
                wlr_seat_request_set_primary_selection_event{source: source,
                                                             serial: serial,};
            init
        };
    wlr_signal_emit_safe(&mut (*seat).events.request_set_primary_selection,
                         &mut event as
                             *mut wlr_seat_request_set_primary_selection_event
                             as *mut libc::c_void);
}
unsafe extern "C" fn seat_handle_primary_selection_source_destroy(mut listener:
                                                                      *mut wl_listener,
                                                                  mut data:
                                                                      *mut libc::c_void) {
    let mut seat: *mut wlr_seat =
        (listener as *mut libc::c_char).offset(-592) as *mut wlr_seat;
    wl_list_remove(&mut (*seat).primary_selection_source_destroy.link);
    (*seat).primary_selection_source = 0 as *mut wlr_primary_selection_source;
    wlr_signal_emit_safe(&mut (*seat).events.set_primary_selection,
                         seat as *mut libc::c_void);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * A data source implementation. Only the `send` function is mandatory.
 */
/* *
 * A source is the sending side of a selection.
 */
// source metadata
/* *
 * Request setting the primary selection. If `client` is not null, then the
 * serial will be checked against the set of serials sent to the client on that
 * seat.
 */
/* *
 * Sets the current primary selection for the seat. NULL can be provided to
 * clear it. This removes the previous one if there was any. In case the
 * selection doesn't come from a client, wl_display_next_serial() can be used to
 * generate a serial.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_set_primary_selection(mut seat:
                                                            *mut wlr_seat,
                                                        mut source:
                                                            *mut wlr_primary_selection_source,
                                                        mut serial:
                                                            uint32_t) {
    if (*seat).primary_selection_source == source {
        (*seat).primary_selection_serial = serial;
        return
    }
    if !(*seat).primary_selection_source.is_null() {
        wl_list_remove(&mut (*seat).primary_selection_source_destroy.link);
        wlr_primary_selection_source_destroy((*seat).primary_selection_source);
        (*seat).primary_selection_source =
            0 as *mut wlr_primary_selection_source
    }
    (*seat).primary_selection_source = source;
    (*seat).primary_selection_serial = serial;
    if !source.is_null() {
        (*seat).primary_selection_source_destroy.notify =
            Some(seat_handle_primary_selection_source_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*source).events.destroy,
                      &mut (*seat).primary_selection_source_destroy);
    }
    wlr_signal_emit_safe(&mut (*seat).events.set_primary_selection,
                         seat as *mut libc::c_void);
}
