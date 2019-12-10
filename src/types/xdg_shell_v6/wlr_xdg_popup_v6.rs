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
    pub type wlr_texture;
    pub type wlr_renderer;
    pub type wlr_backend;
    pub type wlr_output_impl;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
    #[no_mangle]
    fn wl_resource_post_error(resource: *mut wl_resource, code: uint32_t,
                              msg: *const libc::c_char, _: ...);
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
    fn destroy_xdg_surface_v6(surface: *mut wlr_xdg_surface_v6);
    #[no_mangle]
    fn unmap_xdg_surface_v6(surface: *mut wlr_xdg_surface_v6);
    #[no_mangle]
    fn schedule_xdg_surface_v6_configure(surface: *mut wlr_xdg_surface_v6)
     -> uint32_t;
    #[no_mangle]
    fn handle_xdg_surface_v6_commit(wlr_surface: *mut wlr_surface);
    #[no_mangle]
    fn handle_xdg_surface_v6_precommit(wlr_surface: *mut wlr_surface);
    #[no_mangle]
    fn wlr_positioner_v6_invert_x(positioner: *mut wlr_xdg_positioner_v6);
    #[no_mangle]
    fn wlr_positioner_v6_invert_y(positioner: *mut wlr_xdg_positioner_v6);
    #[no_mangle]
    fn wlr_xdg_positioner_v6_get_geometry(positioner:
                                              *mut wlr_xdg_positioner_v6)
     -> wlr_box;
    #[no_mangle]
    static zxdg_popup_v6_interface: wl_interface;
    #[no_mangle]
    fn wlr_seat_client_from_resource(resource: *mut wl_resource)
     -> *mut wlr_seat_client;
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
    fn wlr_seat_pointer_enter(wlr_seat: *mut wlr_seat,
                              surface: *mut wlr_surface, sx: libc::c_double,
                              sy: libc::c_double);
    #[no_mangle]
    fn wlr_seat_pointer_clear_focus(wlr_seat: *mut wlr_seat);
    #[no_mangle]
    fn wlr_seat_pointer_send_motion(wlr_seat: *mut wlr_seat,
                                    time_msec: uint32_t, sx: libc::c_double,
                                    sy: libc::c_double);
    #[no_mangle]
    fn wlr_seat_pointer_send_button(wlr_seat: *mut wlr_seat,
                                    time_msec: uint32_t, button: uint32_t,
                                    state: wlr_button_state) -> uint32_t;
    #[no_mangle]
    fn wlr_seat_pointer_send_axis(wlr_seat: *mut wlr_seat,
                                  time_msec: uint32_t,
                                  orientation: wlr_axis_orientation,
                                  value: libc::c_double,
                                  value_discrete: int32_t,
                                  source: wlr_axis_source);
    #[no_mangle]
    fn wlr_seat_pointer_send_frame(wlr_seat: *mut wlr_seat);
    #[no_mangle]
    fn wlr_seat_pointer_start_grab(wlr_seat: *mut wlr_seat,
                                   grab: *mut wlr_seat_pointer_grab);
    #[no_mangle]
    fn wlr_seat_pointer_end_grab(wlr_seat: *mut wlr_seat);
    #[no_mangle]
    fn wlr_seat_keyboard_start_grab(wlr_seat: *mut wlr_seat,
                                    grab: *mut wlr_seat_keyboard_grab);
    #[no_mangle]
    fn wlr_seat_keyboard_end_grab(wlr_seat: *mut wlr_seat);
    #[no_mangle]
    fn wlr_seat_keyboard_send_key(seat: *mut wlr_seat, time_msec: uint32_t,
                                  key: uint32_t, state: uint32_t);
    #[no_mangle]
    fn wlr_seat_keyboard_send_modifiers(seat: *mut wlr_seat,
                                        modifiers:
                                            *mut wlr_keyboard_modifiers);
    #[no_mangle]
    fn wlr_seat_touch_start_grab(wlr_seat: *mut wlr_seat,
                                 grab: *mut wlr_seat_touch_grab);
    #[no_mangle]
    fn wlr_seat_touch_end_grab(wlr_seat: *mut wlr_seat);
    #[no_mangle]
    fn wlr_seat_touch_send_down(seat: *mut wlr_seat,
                                surface: *mut wlr_surface,
                                time_msec: uint32_t, touch_id: int32_t,
                                sx: libc::c_double, sy: libc::c_double)
     -> uint32_t;
    #[no_mangle]
    fn wlr_seat_touch_send_up(seat: *mut wlr_seat, time_msec: uint32_t,
                              touch_id: int32_t);
    #[no_mangle]
    fn wlr_seat_touch_send_motion(seat: *mut wlr_seat, time_msec: uint32_t,
                                  touch_id: int32_t, sx: libc::c_double,
                                  sy: libc::c_double);
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
pub type wl_output_subpixel = libc::c_uint;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: wl_output_subpixel = 5;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: wl_output_subpixel = 4;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: wl_output_subpixel = 3;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: wl_output_subpixel = 2;
pub const WL_OUTPUT_SUBPIXEL_NONE: wl_output_subpixel = 1;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: wl_output_subpixel = 0;
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}

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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_cursor {
    pub output: *mut wlr_output,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub enabled: bool,
    pub visible: bool,
    pub width: uint32_t,
    pub height: uint32_t,
    pub hotspot_x: int32_t,
    pub hotspot_y: int32_t,
    pub link: wl_list,
    pub texture: *mut crate::src::backend::drm::atomic::wlr_texture,
    pub surface: *mut wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_1,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub destroy: wl_signal,
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
/* *
 * A compositor output region. This typically corresponds to a monitor that
 * displays part of the compositor space.
 *
 * The `frame` event will be emitted when it is a good time for the compositor
 * to submit a new frame.
 *
 * To render a new frame, compositors should call `wlr_output_attach_render`,
 * render and call `wlr_output_commit`. No rendering should happen outside a
 * `frame` event handler or before `wlr_output_attach_render`.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output {
    pub impl_0: *const crate::src::backend::drm::backend::wlr_output_impl,
    pub backend: *mut crate::src::backend::backend::wlr_backend,
    pub display: *mut wl_display,
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub name: [libc::c_char; 24],
    pub make: [libc::c_char; 56],
    pub model: [libc::c_char; 16],
    pub serial: [libc::c_char; 16],
    pub phys_width: int32_t,
    pub phys_height: int32_t,
    pub modes: wl_list,
    pub current_mode: *mut wlr_output_mode,
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub enabled: bool,
    pub scale: libc::c_float,
    pub subpixel: wl_output_subpixel,
    pub transform: wl_output_transform,
    pub needs_frame: bool,
    pub damage: pixman_region32_t,
    pub frame_pending: bool,
    pub transform_matrix: [libc::c_float; 9],
    pub pending: wlr_output_state,
    pub commit_seq: uint32_t,
    pub events: C2RustUnnamed_3,
    pub idle_frame: *mut wl_event_source,
    pub idle_done: *mut wl_event_source,
    pub attach_render_locks: libc::c_int,
    pub cursors: wl_list,
    pub hardware_cursor: *mut wlr_output_cursor,
    pub software_cursor_locks: libc::c_int,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub frame: wl_signal,
    pub needs_frame: wl_signal,
    pub precommit: wl_signal,
    pub commit: wl_signal,
    pub present: wl_signal,
    pub enable: wl_signal,
    pub mode: wl_signal,
    pub scale: wl_signal,
    pub transform: wl_signal,
    pub destroy: wl_signal,
}
/* *
 * Holds the double-buffered output state.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub damage: pixman_region32_t,
    pub buffer_type: wlr_output_state_buffer_type,
    pub buffer: *mut wlr_buffer,
    // if WLR_OUTPUT_STATE_BUFFER_SCANOUT
}
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;

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
    pub selection_source: *mut crate::src::types::data_device::wlr_data_device::wlr_data_source,
    pub selection_serial: uint32_t,
    pub selection_offers: wl_list,
    pub primary_selection_source: *mut crate::src::types::wlr_data_control_v1::wlr_primary_selection_source,
    pub primary_selection_serial: uint32_t,
    pub drag: *mut crate::src::types::data_device::wlr_data_device::wlr_drag,
    pub drag_source: *mut crate::src::types::data_device::wlr_data_device::wlr_data_source,
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
pub type zxdg_shell_v6_error = libc::c_uint;
pub const ZXDG_SHELL_V6_ERROR_INVALID_POSITIONER: zxdg_shell_v6_error = 5;
pub const ZXDG_SHELL_V6_ERROR_INVALID_SURFACE_STATE: zxdg_shell_v6_error = 4;
pub const ZXDG_SHELL_V6_ERROR_INVALID_POPUP_PARENT: zxdg_shell_v6_error = 3;
pub const ZXDG_SHELL_V6_ERROR_NOT_THE_TOPMOST_POPUP: zxdg_shell_v6_error = 2;
pub const ZXDG_SHELL_V6_ERROR_DEFUNCT_SURFACES: zxdg_shell_v6_error = 1;
pub const ZXDG_SHELL_V6_ERROR_ROLE: zxdg_shell_v6_error = 0;
pub type zxdg_positioner_v6_anchor = libc::c_uint;
pub const ZXDG_POSITIONER_V6_ANCHOR_RIGHT: zxdg_positioner_v6_anchor = 8;
pub const ZXDG_POSITIONER_V6_ANCHOR_LEFT: zxdg_positioner_v6_anchor = 4;
pub const ZXDG_POSITIONER_V6_ANCHOR_BOTTOM: zxdg_positioner_v6_anchor = 2;
pub const ZXDG_POSITIONER_V6_ANCHOR_TOP: zxdg_positioner_v6_anchor = 1;
pub const ZXDG_POSITIONER_V6_ANCHOR_NONE: zxdg_positioner_v6_anchor = 0;
pub type zxdg_positioner_v6_gravity = libc::c_uint;
pub const ZXDG_POSITIONER_V6_GRAVITY_RIGHT: zxdg_positioner_v6_gravity = 8;
pub const ZXDG_POSITIONER_V6_GRAVITY_LEFT: zxdg_positioner_v6_gravity = 4;
pub const ZXDG_POSITIONER_V6_GRAVITY_BOTTOM: zxdg_positioner_v6_gravity = 2;
pub const ZXDG_POSITIONER_V6_GRAVITY_TOP: zxdg_positioner_v6_gravity = 1;
pub const ZXDG_POSITIONER_V6_GRAVITY_NONE: zxdg_positioner_v6_gravity = 0;
pub type zxdg_positioner_v6_constraint_adjustment = libc::c_uint;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_RESIZE_Y:
          zxdg_positioner_v6_constraint_adjustment =
    32;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_RESIZE_X:
          zxdg_positioner_v6_constraint_adjustment =
    16;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_FLIP_Y:
          zxdg_positioner_v6_constraint_adjustment =
    8;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_FLIP_X:
          zxdg_positioner_v6_constraint_adjustment =
    4;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_SLIDE_Y:
          zxdg_positioner_v6_constraint_adjustment =
    2;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_SLIDE_X:
          zxdg_positioner_v6_constraint_adjustment =
    1;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_NONE:
          zxdg_positioner_v6_constraint_adjustment =
    0;
pub type zxdg_popup_v6_error = libc::c_uint;
pub const ZXDG_POPUP_V6_ERROR_INVALID_GRAB: zxdg_popup_v6_error = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct zxdg_popup_v6_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub grab: Option<unsafe extern "C" fn(_: *mut wl_client,
                                          _: *mut wl_resource,
                                          _: *mut wl_resource, _: uint32_t)
                         -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_shell_v6 {
    pub global: *mut wl_global,
    pub clients: wl_list,
    pub popup_grabs: wl_list,
    pub ping_timeout: uint32_t,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_9,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_client_v6 {
    pub shell: *mut wlr_xdg_shell_v6,
    pub resource: *mut wl_resource,
    pub client: *mut wl_client,
    pub surfaces: wl_list,
    pub link: wl_list,
    pub ping_serial: uint32_t,
    pub ping_timer: *mut wl_event_source,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_positioner_v6 {
    pub anchor_rect: wlr_box,
    pub anchor: zxdg_positioner_v6_anchor,
    pub gravity: zxdg_positioner_v6_gravity,
    pub constraint_adjustment: zxdg_positioner_v6_constraint_adjustment,
    pub size: C2RustUnnamed_11,
    pub offset: C2RustUnnamed_10,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
    pub x: int32_t,
    pub y: int32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
    pub width: int32_t,
    pub height: int32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_popup_v6 {
    pub base: *mut wlr_xdg_surface_v6,
    pub link: wl_list,
    pub resource: *mut wl_resource,
    pub committed: bool,
    pub parent: *mut wlr_xdg_surface_v6,
    pub seat: *mut wlr_seat,
    pub geometry: wlr_box,
    pub positioner: wlr_xdg_positioner_v6,
    pub grab_link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_surface_v6 {
    pub client: *mut wlr_xdg_client_v6,
    pub resource: *mut wl_resource,
    pub surface: *mut wlr_surface,
    pub link: wl_list,
    pub role: wlr_xdg_surface_v6_role,
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub popups: wl_list,
    pub added: bool,
    pub configured: bool,
    pub mapped: bool,
    pub configure_serial: uint32_t,
    pub configure_idle: *mut wl_event_source,
    pub configure_next_serial: uint32_t,
    pub configure_list: wl_list,
    pub has_next_geometry: bool,
    pub next_geometry: wlr_box,
    pub geometry: wlr_box,
    pub surface_destroy: wl_listener,
    pub surface_commit: wl_listener,
    pub events: C2RustUnnamed_12,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_12 {
    pub destroy: wl_signal,
    pub ping_timeout: wl_signal,
    pub new_popup: wl_signal,
    pub map: wl_signal,
    pub unmap: wl_signal,
}

#[repr ( C )]#[derive(Copy, Clone)]
pub union C2RustUnnamed_13 {
    pub toplevel: *mut wlr_xdg_toplevel_v6,
    pub popup: *mut wlr_xdg_popup_v6,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_toplevel_v6 {
    pub resource: *mut wl_resource,
    pub base: *mut wlr_xdg_surface_v6,
    pub parent: *mut wlr_xdg_surface_v6,
    pub added: bool,
    pub client_pending: wlr_xdg_toplevel_v6_state,
    pub server_pending: wlr_xdg_toplevel_v6_state,
    pub current: wlr_xdg_toplevel_v6_state,
    pub title: *mut libc::c_char,
    pub app_id: *mut libc::c_char,
    pub events: C2RustUnnamed_14,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_14 {
    pub request_maximize: wl_signal,
    pub request_fullscreen: wl_signal,
    pub request_minimize: wl_signal,
    pub request_move: wl_signal,
    pub request_resize: wl_signal,
    pub request_show_window_menu: wl_signal,
    pub set_parent: wl_signal,
    pub set_title: wl_signal,
    pub set_app_id: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_toplevel_v6_state {
    pub maximized: bool,
    pub fullscreen: bool,
    pub resizing: bool,
    pub activated: bool,
    pub width: uint32_t,
    pub height: uint32_t,
    pub max_width: uint32_t,
    pub max_height: uint32_t,
    pub min_width: uint32_t,
    pub min_height: uint32_t,
    pub fullscreen_output: *mut wlr_output,
    pub fullscreen_output_destroy: wl_listener,
}
pub type wlr_xdg_surface_v6_role = libc::c_uint;
pub const WLR_XDG_SURFACE_V6_ROLE_POPUP: wlr_xdg_surface_v6_role = 2;
pub const WLR_XDG_SURFACE_V6_ROLE_TOPLEVEL: wlr_xdg_surface_v6_role = 1;
pub const WLR_XDG_SURFACE_V6_ROLE_NONE: wlr_xdg_surface_v6_role = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_popup_grab_v6 {
    pub client: *mut wl_client,
    pub pointer_grab: wlr_seat_pointer_grab,
    pub keyboard_grab: wlr_seat_keyboard_grab,
    pub touch_grab: wlr_seat_touch_grab,
    pub seat: *mut wlr_seat,
    pub popups: wl_list,
    pub link: wl_list,
    pub seat_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_positioner_v6_resource {
    pub resource: *mut wl_resource,
    pub attrs: wlr_xdg_positioner_v6,
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
/* *
 * @ingroup iface_zxdg_popup_v6
 */
/* *
 * @ingroup iface_zxdg_popup_v6
 */
/* *
 * @ingroup iface_zxdg_popup_v6
 */
/* *
 * @ingroup iface_zxdg_popup_v6
 */
/* *
 * @ingroup iface_zxdg_popup_v6
 * Sends an configure event to the client owning the resource.
 * @param resource_ The client's resource
 * @param x x position relative to parent surface window geometry
 * @param y y position relative to parent surface window geometry
 * @param width window geometry width
 * @param height window geometry height
 */
/* *
 * @ingroup iface_zxdg_popup_v6
 * Sends an popup_done event to the client owning the resource.
 * @param resource_ The client's resource
 */
#[inline]
unsafe extern "C" fn zxdg_popup_v6_send_popup_done(mut resource_:
                                                       *mut wl_resource) {
    wl_resource_post_event(resource_, 1i32 as uint32_t);
}
unsafe extern "C" fn xdg_popup_grab_get_topmost(mut grab:
                                                    *mut wlr_xdg_popup_grab_v6)
 -> *mut wlr_xdg_surface_v6 {
    let mut popup: *mut wlr_xdg_popup_v6 = 0 as *mut wlr_xdg_popup_v6;
    popup =
        ((*grab).popups.next as *mut libc::c_char).offset(-120) as
            *mut wlr_xdg_popup_v6;
    if &mut (*popup).grab_link as *mut wl_list !=
           &mut (*grab).popups as *mut wl_list {
        return (*popup).base
    }
    return 0 as *mut wlr_xdg_surface_v6;
}
unsafe extern "C" fn xdg_popup_grab_end(mut popup_grab:
                                            *mut wlr_xdg_popup_grab_v6) {
    let mut popup: *mut wlr_xdg_popup_v6 = 0 as *mut wlr_xdg_popup_v6;
    let mut tmp: *mut wlr_xdg_popup_v6 = 0 as *mut wlr_xdg_popup_v6;
    popup =
        ((*popup_grab).popups.next as *mut libc::c_char).offset(-120) as
            *mut wlr_xdg_popup_v6;
    tmp =
        ((*popup).grab_link.next as *mut libc::c_char).offset(-120) as
            *mut wlr_xdg_popup_v6;
    while &mut (*popup).grab_link as *mut wl_list !=
              &mut (*popup_grab).popups as *mut wl_list {
        zxdg_popup_v6_send_popup_done((*popup).resource);
        popup = tmp;
        tmp =
            ((*popup).grab_link.next as *mut libc::c_char).offset(-120) as
                *mut wlr_xdg_popup_v6
    }
    wlr_seat_pointer_end_grab((*popup_grab).seat);
    wlr_seat_keyboard_end_grab((*popup_grab).seat);
    wlr_seat_touch_end_grab((*popup_grab).seat);
}
unsafe extern "C" fn xdg_pointer_grab_enter(mut grab:
                                                *mut wlr_seat_pointer_grab,
                                            mut surface: *mut wlr_surface,
                                            mut sx: libc::c_double,
                                            mut sy: libc::c_double) {
    let mut popup_grab: *mut wlr_xdg_popup_grab_v6 =
        (*grab).data as *mut wlr_xdg_popup_grab_v6;
    if wl_resource_get_client((*surface).resource) == (*popup_grab).client {
        wlr_seat_pointer_enter((*grab).seat, surface, sx, sy);
    } else { wlr_seat_pointer_clear_focus((*grab).seat); };
}
unsafe extern "C" fn xdg_pointer_grab_motion(mut grab:
                                                 *mut wlr_seat_pointer_grab,
                                             mut time: uint32_t,
                                             mut sx: libc::c_double,
                                             mut sy: libc::c_double) {
    wlr_seat_pointer_send_motion((*grab).seat, time, sx, sy);
}
unsafe extern "C" fn xdg_pointer_grab_button(mut grab:
                                                 *mut wlr_seat_pointer_grab,
                                             mut time: uint32_t,
                                             mut button: uint32_t,
                                             mut state: uint32_t)
 -> uint32_t {
    let mut serial: uint32_t =
        wlr_seat_pointer_send_button((*grab).seat, time, button,
                                     state as wlr_button_state);
    if serial != 0 {
        return serial
    } else {
        xdg_popup_grab_end((*grab).data as *mut wlr_xdg_popup_grab_v6);
        return 0i32 as uint32_t
    };
}
unsafe extern "C" fn xdg_pointer_grab_axis(mut grab:
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
unsafe extern "C" fn xdg_pointer_grab_frame(mut grab:
                                                *mut wlr_seat_pointer_grab) {
    wlr_seat_pointer_send_frame((*grab).seat);
}
unsafe extern "C" fn xdg_pointer_grab_cancel(mut grab:
                                                 *mut wlr_seat_pointer_grab) {
    xdg_popup_grab_end((*grab).data as *mut wlr_xdg_popup_grab_v6);
}
static mut xdg_pointer_grab_impl: wlr_pointer_grab_interface =
    unsafe {
        {
            let mut init =
                wlr_pointer_grab_interface{enter:
                                               Some(xdg_pointer_grab_enter as
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
                                               Some(xdg_pointer_grab_motion as
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
                                                                                      uint32_t>>(Some(xdg_pointer_grab_button
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
                                               Some(xdg_pointer_grab_axis as
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
                                               Some(xdg_pointer_grab_frame as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wlr_seat_pointer_grab)
                                                            -> ()),
                                           cancel:
                                               Some(xdg_pointer_grab_cancel as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wlr_seat_pointer_grab)
                                                            -> ()),};
            init
        }
    };
unsafe extern "C" fn xdg_keyboard_grab_enter(mut grab:
                                                 *mut wlr_seat_keyboard_grab,
                                             mut surface: *mut wlr_surface,
                                             mut keycodes: *mut uint32_t,
                                             mut num_keycodes: size_t,
                                             mut modifiers:
                                                 *mut wlr_keyboard_modifiers) {
    // keyboard focus should remain on the popup
}
unsafe extern "C" fn xdg_keyboard_grab_key(mut grab:
                                               *mut wlr_seat_keyboard_grab,
                                           mut time: uint32_t,
                                           mut key: uint32_t,
                                           mut state: uint32_t) {
    wlr_seat_keyboard_send_key((*grab).seat, time, key, state);
}
unsafe extern "C" fn xdg_keyboard_grab_modifiers(mut grab:
                                                     *mut wlr_seat_keyboard_grab,
                                                 mut modifiers:
                                                     *mut wlr_keyboard_modifiers) {
    wlr_seat_keyboard_send_modifiers((*grab).seat, modifiers);
}
unsafe extern "C" fn xdg_keyboard_grab_cancel(mut grab:
                                                  *mut wlr_seat_keyboard_grab) {
    wlr_seat_pointer_end_grab((*grab).seat);
}
static mut xdg_keyboard_grab_impl: wlr_keyboard_grab_interface =
    {
    
        {
            let mut init =
                wlr_keyboard_grab_interface{enter:
                                                Some(xdg_keyboard_grab_enter
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
                                                Some(xdg_keyboard_grab_key as
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
                                                Some(xdg_keyboard_grab_modifiers
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut wlr_seat_keyboard_grab,
                                                                              _:
                                                                                  *mut wlr_keyboard_modifiers)
                                                             -> ()),
                                            cancel:
                                                Some(xdg_keyboard_grab_cancel
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut wlr_seat_keyboard_grab)
                                                             -> ()),};
            init
        }
};
unsafe extern "C" fn xdg_touch_grab_down(mut grab: *mut wlr_seat_touch_grab,
                                         mut time: uint32_t,
                                         mut point: *mut wlr_touch_point)
 -> uint32_t {
    let mut popup_grab: *mut wlr_xdg_popup_grab_v6 =
        (*grab).data as *mut wlr_xdg_popup_grab_v6;
    if wl_resource_get_client((*(*point).surface).resource) !=
           (*popup_grab).client {
        xdg_popup_grab_end((*grab).data as *mut wlr_xdg_popup_grab_v6);
        return 0i32 as uint32_t
    }
    return wlr_seat_touch_send_down((*grab).seat, (*point).surface, time,
                                    (*point).touch_id, (*point).sx,
                                    (*point).sy);
}
unsafe extern "C" fn xdg_touch_grab_up(mut grab: *mut wlr_seat_touch_grab,
                                       mut time: uint32_t,
                                       mut point: *mut wlr_touch_point) {
    wlr_seat_touch_send_up((*grab).seat, time, (*point).touch_id);
}
unsafe extern "C" fn xdg_touch_grab_motion(mut grab: *mut wlr_seat_touch_grab,
                                           mut time: uint32_t,
                                           mut point: *mut wlr_touch_point) {
    wlr_seat_touch_send_motion((*grab).seat, time, (*point).touch_id,
                               (*point).sx, (*point).sy);
}
unsafe extern "C" fn xdg_touch_grab_enter(mut grab: *mut wlr_seat_touch_grab,
                                          mut time: uint32_t,
                                          mut point: *mut wlr_touch_point) {
}
unsafe extern "C" fn xdg_touch_grab_cancel(mut grab:
                                               *mut wlr_seat_touch_grab) {
    wlr_seat_touch_end_grab((*grab).seat);
}
static mut xdg_touch_grab_impl: wlr_touch_grab_interface =
    {
    
        {
            let mut init =
                wlr_touch_grab_interface{down:
                                             Some(xdg_touch_grab_down as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> uint32_t),
                                         up:
                                             Some(xdg_touch_grab_up as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> ()),
                                         motion:
                                             Some(xdg_touch_grab_motion as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> ()),
                                         enter:
                                             Some(xdg_touch_grab_enter as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab,
                                                                           _:
                                                                               uint32_t,
                                                                           _:
                                                                               *mut wlr_touch_point)
                                                          -> ()),
                                         cancel:
                                             Some(xdg_touch_grab_cancel as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wlr_seat_touch_grab)
                                                          -> ()),};
            init
        }
};
unsafe extern "C" fn xdg_popup_grab_handle_seat_destroy(mut listener:
                                                            *mut wl_listener,
                                                        mut data:
                                                            *mut libc::c_void) {
    let mut xdg_grab: *mut wlr_xdg_popup_grab_v6 =
        (listener as *mut libc::c_char).offset(-120) as
            *mut wlr_xdg_popup_grab_v6;
    wl_list_remove(&mut (*xdg_grab).seat_destroy.link);
    let mut popup: *mut wlr_xdg_popup_v6 = 0 as *mut wlr_xdg_popup_v6;
    let mut next: *mut wlr_xdg_popup_v6 = 0 as *mut wlr_xdg_popup_v6;
    popup =
        ((*xdg_grab).popups.next as *mut libc::c_char).offset(-120) as
            *mut wlr_xdg_popup_v6;
    next =
        ((*popup).grab_link.next as *mut libc::c_char).offset(-120) as
            *mut wlr_xdg_popup_v6;
    while &mut (*popup).grab_link as *mut wl_list !=
              &mut (*xdg_grab).popups as *mut wl_list {
        destroy_xdg_surface_v6((*popup).base);
        popup = next;
        next =
            ((*popup).grab_link.next as *mut libc::c_char).offset(-120) as
                *mut wlr_xdg_popup_v6
    }
    wl_list_remove(&mut (*xdg_grab).link);
    free(xdg_grab as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn get_xdg_shell_v6_popup_grab_from_seat(mut shell:
                                                                   *mut wlr_xdg_shell_v6,
                                                               mut seat:
                                                                   *mut wlr_seat)
 -> *mut wlr_xdg_popup_grab_v6 {
    let mut xdg_grab: *mut wlr_xdg_popup_grab_v6 =
        0 as *mut wlr_xdg_popup_grab_v6;
    xdg_grab =
        ((*shell).popup_grabs.next as *mut libc::c_char).offset(-104) as
            *mut wlr_xdg_popup_grab_v6;
    while &mut (*xdg_grab).link as *mut wl_list !=
              &mut (*shell).popup_grabs as *mut wl_list {
        if (*xdg_grab).seat == seat { return xdg_grab }
        xdg_grab =
            ((*xdg_grab).link.next as *mut libc::c_char).offset(-104) as
                *mut wlr_xdg_popup_grab_v6
    }
    xdg_grab =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xdg_popup_grab_v6>() as
                   libc::c_ulong) as *mut wlr_xdg_popup_grab_v6;
    if xdg_grab.is_null() { return 0 as *mut wlr_xdg_popup_grab_v6 }
    (*xdg_grab).pointer_grab.data = xdg_grab as *mut libc::c_void;
    (*xdg_grab).pointer_grab.interface = &xdg_pointer_grab_impl;
    (*xdg_grab).keyboard_grab.data = xdg_grab as *mut libc::c_void;
    (*xdg_grab).keyboard_grab.interface = &xdg_keyboard_grab_impl;
    (*xdg_grab).touch_grab.data = xdg_grab as *mut libc::c_void;
    (*xdg_grab).touch_grab.interface = &xdg_touch_grab_impl;
    wl_list_init(&mut (*xdg_grab).popups);
    wl_list_insert(&mut (*shell).popup_grabs, &mut (*xdg_grab).link);
    (*xdg_grab).seat = seat;
    (*xdg_grab).seat_destroy.notify =
        Some(xdg_popup_grab_handle_seat_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*seat).events.destroy, &mut (*xdg_grab).seat_destroy);
    return xdg_grab;
}
unsafe extern "C" fn xdg_surface_from_xdg_popup_resource(mut resource:
                                                             *mut wl_resource)
 -> *mut wlr_xdg_surface_v6 {
    if wl_resource_instance_of(resource, &zxdg_popup_v6_interface,
                               &zxdg_popup_v6_implementation as
                                   *const zxdg_popup_v6_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &zxdg_popup_v6_interface, &zxdg_popup_v6_implementation)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/xdg_shell_v6/wlr_xdg_popup_v6.c\x00" as
                          *const u8 as *const libc::c_char,
                      200i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[libc::c_char; 85]>(b"struct wlr_xdg_surface_v6 *xdg_surface_from_xdg_popup_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_xdg_surface_v6;
}
#[no_mangle]
pub unsafe extern "C" fn destroy_xdg_popup_v6(mut surface:
                                                  *mut wlr_xdg_surface_v6) {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_V6_ROLE_POPUP as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_V6_ROLE_POPUP\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell_v6/wlr_xdg_popup_v6.c\x00" as
                          *const u8 as *const libc::c_char,
                      205i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void destroy_xdg_popup_v6(struct wlr_xdg_surface_v6 *)\x00")).as_ptr());
    };
    unmap_xdg_surface_v6(surface);
    wl_resource_set_user_data((*(*surface).c2rust_unnamed.popup).resource,
                              0 as *mut libc::c_void);
    wl_list_remove(&mut (*(*surface).c2rust_unnamed.popup).link);
    free((*surface).c2rust_unnamed.popup as *mut libc::c_void);
    (*surface).c2rust_unnamed.popup = 0 as *mut wlr_xdg_popup_v6;
    (*surface).role = WLR_XDG_SURFACE_V6_ROLE_NONE;
}
unsafe extern "C" fn xdg_popup_handle_grab(mut client: *mut wl_client,
                                           mut resource: *mut wl_resource,
                                           mut seat_resource:
                                               *mut wl_resource,
                                           mut serial: uint32_t) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        xdg_surface_from_xdg_popup_resource(resource);
    let mut seat_client: *mut wlr_seat_client =
        wlr_seat_client_from_resource(seat_resource);
    if surface.is_null() { return }
    if (*(*surface).c2rust_unnamed.popup).committed {
        wl_resource_post_error((*(*surface).c2rust_unnamed.popup).resource,
                               ZXDG_POPUP_V6_ERROR_INVALID_GRAB as libc::c_int
                                   as uint32_t,
                               b"xdg_popup is already mapped\x00" as *const u8
                                   as *const libc::c_char);
        return
    }
    let mut popup_grab: *mut wlr_xdg_popup_grab_v6 =
        get_xdg_shell_v6_popup_grab_from_seat((*(*surface).client).shell,
                                              (*seat_client).seat);
    let mut topmost: *mut wlr_xdg_surface_v6 =
        xdg_popup_grab_get_topmost(popup_grab);
    let mut parent_is_toplevel: bool =
        (*(*(*surface).c2rust_unnamed.popup).parent).role as libc::c_uint ==
            WLR_XDG_SURFACE_V6_ROLE_TOPLEVEL as libc::c_int as libc::c_uint;
    if topmost.is_null() && !parent_is_toplevel ||
           !topmost.is_null() &&
               topmost != (*(*surface).c2rust_unnamed.popup).parent {
        wl_resource_post_error((*(*surface).client).resource,
                               ZXDG_SHELL_V6_ERROR_NOT_THE_TOPMOST_POPUP as
                                   libc::c_int as uint32_t,
                               b"xdg_popup was not created on the topmost popup\x00"
                                   as *const u8 as *const libc::c_char);
        return
    }
    (*popup_grab).client = (*(*surface).client).client;
    (*(*surface).c2rust_unnamed.popup).seat = (*seat_client).seat;
    wl_list_insert(&mut (*popup_grab).popups,
                   &mut (*(*surface).c2rust_unnamed.popup).grab_link);
    wlr_seat_pointer_start_grab((*seat_client).seat,
                                &mut (*popup_grab).pointer_grab);
    wlr_seat_keyboard_start_grab((*seat_client).seat,
                                 &mut (*popup_grab).keyboard_grab);
    wlr_seat_touch_start_grab((*seat_client).seat,
                              &mut (*popup_grab).touch_grab);
}
unsafe extern "C" fn xdg_popup_handle_destroy(mut client: *mut wl_client,
                                              mut resource:
                                                  *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        xdg_surface_from_xdg_popup_resource(resource);
    if !surface.is_null() && wl_list_empty(&mut (*surface).popups) == 0 {
        wl_resource_post_error((*(*surface).client).resource,
                               ZXDG_SHELL_V6_ERROR_NOT_THE_TOPMOST_POPUP as
                                   libc::c_int as uint32_t,
                               b"xdg_popup was destroyed while it was not the topmost popup\x00"
                                   as *const u8 as *const libc::c_char);
        return
    }
    wl_resource_destroy(resource);
}
static mut zxdg_popup_v6_implementation: zxdg_popup_v6_interface =
    {
    
        {
            let mut init =
                zxdg_popup_v6_interface{destroy:
                                            Some(xdg_popup_handle_destroy as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource)
                                                         -> ()),
                                        grab:
                                            Some(xdg_popup_handle_grab as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource,
                                                                          _:
                                                                              *mut wl_resource,
                                                                          _:
                                                                              uint32_t)
                                                         -> ()),};
            init
        }
};
unsafe extern "C" fn xdg_popup_handle_resource_destroy(mut resource:
                                                           *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        xdg_surface_from_xdg_popup_resource(resource);
    if !surface.is_null() { destroy_xdg_popup_v6(surface); };
}
#[no_mangle]
pub unsafe extern "C" fn handle_xdg_surface_v6_popup_committed(mut surface:
                                                                   *mut wlr_xdg_surface_v6) {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_V6_ROLE_POPUP as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_V6_ROLE_POPUP\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell_v6/wlr_xdg_popup_v6.c\x00" as
                          *const u8 as *const libc::c_char,
                      292i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"void handle_xdg_surface_v6_popup_committed(struct wlr_xdg_surface_v6 *)\x00")).as_ptr());
    };
    if !(*(*surface).c2rust_unnamed.popup).committed {
        schedule_xdg_surface_v6_configure(surface);
        (*(*surface).c2rust_unnamed.popup).committed = 1i32 != 0
    };
}
#[no_mangle]
pub static mut xdg_popup_v6_surface_role: wlr_surface_role =
    {
    
        {
            let mut init =
                wlr_surface_role{name:
                                     b"xdg_popup_v6\x00" as *const u8 as
                                         *const libc::c_char,
                                 commit:
                                     Some(handle_xdg_surface_v6_commit as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_surface)
                                                  -> ()),
                                 precommit:
                                     Some(handle_xdg_surface_v6_precommit as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_surface)
                                                  -> ()),};
            init
        }
};
#[no_mangle]
pub unsafe extern "C" fn create_xdg_popup_v6(mut xdg_surface:
                                                 *mut wlr_xdg_surface_v6,
                                             mut parent:
                                                 *mut wlr_xdg_surface_v6,
                                             mut positioner:
                                                 *mut wlr_xdg_positioner_v6_resource,
                                             mut id: int32_t) {
    if (*positioner).attrs.size.width == 0i32 ||
           (*positioner).attrs.anchor_rect.width == 0i32 {
        wl_resource_post_error((*xdg_surface).resource,
                               ZXDG_SHELL_V6_ERROR_INVALID_POSITIONER as
                                   libc::c_int as uint32_t,
                               b"positioner object is not complete\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    if !wlr_surface_set_role((*xdg_surface).surface,
                             &xdg_popup_v6_surface_role,
                             xdg_surface as *mut libc::c_void,
                             (*xdg_surface).resource,
                             ZXDG_SHELL_V6_ERROR_ROLE as libc::c_int as
                                 uint32_t) {
        return
    }
    (*xdg_surface).c2rust_unnamed.popup =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xdg_popup_v6>() as libc::c_ulong) as
            *mut wlr_xdg_popup_v6;
    if (*xdg_surface).c2rust_unnamed.popup.is_null() {
        wl_resource_post_no_memory((*xdg_surface).resource);
        return
    }
    (*(*xdg_surface).c2rust_unnamed.popup).resource =
        wl_resource_create((*(*xdg_surface).client).client,
                           &zxdg_popup_v6_interface,
                           wl_resource_get_version((*xdg_surface).resource),
                           id as uint32_t);
    if (*(*xdg_surface).c2rust_unnamed.popup).resource.is_null() {
        free((*xdg_surface).c2rust_unnamed.popup as *mut libc::c_void);
        wl_resource_post_no_memory((*xdg_surface).resource);
        return
    }
    wl_resource_set_implementation((*(*xdg_surface).c2rust_unnamed.popup).resource,
                                   &zxdg_popup_v6_implementation as
                                       *const zxdg_popup_v6_interface as
                                       *const libc::c_void,
                                   xdg_surface as *mut libc::c_void,
                                   Some(xdg_popup_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    (*xdg_surface).role = WLR_XDG_SURFACE_V6_ROLE_POPUP;
    (*(*xdg_surface).c2rust_unnamed.popup).base = xdg_surface;
    (*(*xdg_surface).c2rust_unnamed.popup).parent = parent;
    (*(*xdg_surface).c2rust_unnamed.popup).geometry =
        wlr_xdg_positioner_v6_get_geometry(&mut (*positioner).attrs);
    // positioner properties
    memcpy(&mut (*(*xdg_surface).c2rust_unnamed.popup).positioner as
               *mut wlr_xdg_positioner_v6 as *mut libc::c_void,
           &mut (*positioner).attrs as *mut wlr_xdg_positioner_v6 as
               *const libc::c_void,
           ::std::mem::size_of::<wlr_xdg_positioner_v6>() as libc::c_ulong);
    wl_list_insert(&mut (*parent).popups,
                   &mut (*(*xdg_surface).c2rust_unnamed.popup).link);
    wlr_signal_emit_safe(&mut (*parent).events.new_popup,
                         (*xdg_surface).c2rust_unnamed.popup as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_popup_v6_get_anchor_point(mut popup:
                                                               *mut wlr_xdg_popup_v6,
                                                           mut root_sx:
                                                               *mut libc::c_int,
                                                           mut root_sy:
                                                               *mut libc::c_int) {
    let mut rect: wlr_box = (*popup).positioner.anchor_rect;
    let mut anchor: zxdg_positioner_v6_anchor = (*popup).positioner.anchor;
    let mut sx: libc::c_int = 0i32;
    let mut sy: libc::c_int = 0i32;
    if anchor as libc::c_uint ==
           ZXDG_POSITIONER_V6_ANCHOR_NONE as libc::c_int as libc::c_uint {
        sx = (rect.x + rect.width) / 2i32;
        sy = (rect.y + rect.height) / 2i32
    } else if anchor as libc::c_uint ==
                  ZXDG_POSITIONER_V6_ANCHOR_TOP as libc::c_int as libc::c_uint
     {
        sx = (rect.x + rect.width) / 2i32;
        sy = rect.y
    } else if anchor as libc::c_uint ==
                  ZXDG_POSITIONER_V6_ANCHOR_BOTTOM as libc::c_int as
                      libc::c_uint {
        sx = (rect.x + rect.width) / 2i32;
        sy = rect.y + rect.height
    } else if anchor as libc::c_uint ==
                  ZXDG_POSITIONER_V6_ANCHOR_LEFT as libc::c_int as
                      libc::c_uint {
        sx = rect.x;
        sy = (rect.y + rect.height) / 2i32
    } else if anchor as libc::c_uint ==
                  ZXDG_POSITIONER_V6_ANCHOR_RIGHT as libc::c_int as
                      libc::c_uint {
        sx = rect.x + rect.width;
        sy = (rect.y + rect.height) / 2i32
    } else if anchor as libc::c_uint ==
                  (ZXDG_POSITIONER_V6_ANCHOR_TOP as libc::c_int |
                       ZXDG_POSITIONER_V6_ANCHOR_LEFT as libc::c_int) as
                      libc::c_uint {
        sx = rect.x;
        sy = rect.y
    } else if anchor as libc::c_uint ==
                  (ZXDG_POSITIONER_V6_ANCHOR_TOP as libc::c_int |
                       ZXDG_POSITIONER_V6_ANCHOR_RIGHT as libc::c_int) as
                      libc::c_uint {
        sx = rect.x + rect.width;
        sy = rect.y
    } else if anchor as libc::c_uint ==
                  (ZXDG_POSITIONER_V6_ANCHOR_BOTTOM as libc::c_int |
                       ZXDG_POSITIONER_V6_ANCHOR_LEFT as libc::c_int) as
                      libc::c_uint {
        sx = rect.x;
        sy = rect.y + rect.height
    } else if anchor as libc::c_uint ==
                  (ZXDG_POSITIONER_V6_ANCHOR_BOTTOM as libc::c_int |
                       ZXDG_POSITIONER_V6_ANCHOR_RIGHT as libc::c_int) as
                      libc::c_uint {
        sx = rect.x + rect.width;
        sy = rect.y + rect.height
    }
    *root_sx = sx;
    *root_sy = sy;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_popup_v6_get_toplevel_coords(mut popup:
                                                                  *mut wlr_xdg_popup_v6,
                                                              mut popup_sx:
                                                                  libc::c_int,
                                                              mut popup_sy:
                                                                  libc::c_int,
                                                              mut toplevel_sx:
                                                                  *mut libc::c_int,
                                                              mut toplevel_sy:
                                                                  *mut libc::c_int) {
    let mut parent: *mut wlr_xdg_surface_v6 = (*popup).parent;
    while !parent.is_null() &&
              (*parent).role as libc::c_uint ==
                  WLR_XDG_SURFACE_V6_ROLE_POPUP as libc::c_int as libc::c_uint
          {
        popup_sx += (*(*parent).c2rust_unnamed.popup).geometry.x;
        popup_sy += (*(*parent).c2rust_unnamed.popup).geometry.y;
        parent = (*(*parent).c2rust_unnamed.popup).parent
    }
    if !parent.is_null() {
    } else {
        __assert_fail(b"parent\x00" as *const u8 as *const libc::c_char,
                      b"../types/xdg_shell_v6/wlr_xdg_popup_v6.c\x00" as
                          *const u8 as *const libc::c_char,
                      406i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 93],
                                                &[libc::c_char; 93]>(b"void wlr_xdg_popup_v6_get_toplevel_coords(struct wlr_xdg_popup_v6 *, int, int, int *, int *)\x00")).as_ptr());
    };
    *toplevel_sx = popup_sx + (*parent).geometry.x;
    *toplevel_sy = popup_sy + (*parent).geometry.y;
}
unsafe extern "C" fn xdg_popup_v6_box_constraints(mut popup:
                                                      *mut wlr_xdg_popup_v6,
                                                  mut toplevel_sx_box:
                                                      *mut wlr_box,
                                                  mut offset_x:
                                                      *mut libc::c_int,
                                                  mut offset_y:
                                                      *mut libc::c_int) {
    let mut popup_width: libc::c_int = (*popup).geometry.width;
    let mut popup_height: libc::c_int = (*popup).geometry.height;
    let mut anchor_sx: libc::c_int = 0i32;
    let mut anchor_sy: libc::c_int = 0i32;
    wlr_xdg_popup_v6_get_anchor_point(popup, &mut anchor_sx, &mut anchor_sy);
    let mut popup_sx: libc::c_int = 0i32;
    let mut popup_sy: libc::c_int = 0i32;
    wlr_xdg_popup_v6_get_toplevel_coords(popup, (*popup).geometry.x,
                                         (*popup).geometry.y, &mut popup_sx,
                                         &mut popup_sy);
    *offset_x = 0i32;
    *offset_y = 0i32;
    if popup_sx < (*toplevel_sx_box).x {
        *offset_x = (*toplevel_sx_box).x - popup_sx
    } else if popup_sx + popup_width >
                  (*toplevel_sx_box).x + (*toplevel_sx_box).width {
        *offset_x =
            (*toplevel_sx_box).x + (*toplevel_sx_box).width -
                (popup_sx + popup_width)
    }
    if popup_sy < (*toplevel_sx_box).y {
        *offset_y = (*toplevel_sx_box).y - popup_sy
    } else if popup_sy + popup_height >
                  (*toplevel_sx_box).y + (*toplevel_sx_box).height {
        *offset_y =
            (*toplevel_sx_box).y + (*toplevel_sx_box).height -
                (popup_sy + popup_height)
    };
}
unsafe extern "C" fn xdg_popup_v6_unconstrain_flip(mut popup:
                                                       *mut wlr_xdg_popup_v6,
                                                   mut toplevel_sx_box:
                                                       *mut wlr_box) -> bool {
    let mut offset_x: libc::c_int = 0i32;
    let mut offset_y: libc::c_int = 0i32;
    xdg_popup_v6_box_constraints(popup, toplevel_sx_box, &mut offset_x,
                                 &mut offset_y);
    if offset_x == 0 && offset_y == 0 { return 1i32 != 0 }
    let mut flip_x: bool =
        offset_x != 0 &&
            (*popup).positioner.constraint_adjustment as libc::c_uint &
                ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_FLIP_X as libc::c_int
                    as libc::c_uint != 0;
    let mut flip_y: bool =
        offset_y != 0 &&
            (*popup).positioner.constraint_adjustment as libc::c_uint &
                ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_FLIP_Y as libc::c_int
                    as libc::c_uint != 0;
    if flip_x { wlr_positioner_v6_invert_x(&mut (*popup).positioner); }
    if flip_y { wlr_positioner_v6_invert_y(&mut (*popup).positioner); }
    (*popup).geometry =
        wlr_xdg_positioner_v6_get_geometry(&mut (*popup).positioner);
    xdg_popup_v6_box_constraints(popup, toplevel_sx_box, &mut offset_x,
                                 &mut offset_y);
    if offset_x == 0 && offset_y == 0 {
        // no longer constrained
        return 1i32 != 0
    }
    // revert the positioner back if it didn't fix it and go to the next part
    if flip_x { wlr_positioner_v6_invert_x(&mut (*popup).positioner); }
    if flip_y { wlr_positioner_v6_invert_y(&mut (*popup).positioner); }
    (*popup).geometry =
        wlr_xdg_positioner_v6_get_geometry(&mut (*popup).positioner);
    return 0i32 != 0;
}
unsafe extern "C" fn xdg_popup_v6_unconstrain_slide(mut popup:
                                                        *mut wlr_xdg_popup_v6,
                                                    mut toplevel_sx_box:
                                                        *mut wlr_box)
 -> bool {
    let mut offset_x: libc::c_int = 0i32;
    let mut offset_y: libc::c_int = 0i32;
    xdg_popup_v6_box_constraints(popup, toplevel_sx_box, &mut offset_x,
                                 &mut offset_y);
    if offset_x == 0 && offset_y == 0 { return 1i32 != 0 }
    let mut slide_x: bool =
        offset_x != 0 &&
            (*popup).positioner.constraint_adjustment as libc::c_uint &
                ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_SLIDE_X as
                    libc::c_int as libc::c_uint != 0;
    let mut slide_y: bool =
        offset_y != 0 &&
            (*popup).positioner.constraint_adjustment as libc::c_uint &
                ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_SLIDE_Y as
                    libc::c_int as libc::c_uint != 0;
    if slide_x { (*popup).geometry.x += offset_x }
    if slide_y { (*popup).geometry.y += offset_y }
    let mut toplevel_x: libc::c_int = 0i32;
    let mut toplevel_y: libc::c_int = 0i32;
    wlr_xdg_popup_v6_get_toplevel_coords(popup, (*popup).geometry.x,
                                         (*popup).geometry.y, &mut toplevel_x,
                                         &mut toplevel_y);
    if slide_x as libc::c_int != 0 && toplevel_x < (*toplevel_sx_box).x {
        (*popup).geometry.x += (*toplevel_sx_box).x - toplevel_x
    }
    if slide_y as libc::c_int != 0 && toplevel_y < (*toplevel_sx_box).y {
        (*popup).geometry.y += (*toplevel_sx_box).y - toplevel_y
    }
    xdg_popup_v6_box_constraints(popup, toplevel_sx_box, &mut offset_x,
                                 &mut offset_y);
    return offset_x == 0 && offset_y == 0;
}
unsafe extern "C" fn xdg_popup_v6_unconstrain_resize(mut popup:
                                                         *mut wlr_xdg_popup_v6,
                                                     mut toplevel_sx_box:
                                                         *mut wlr_box)
 -> bool {
    let mut offset_x: libc::c_int = 0;
    let mut offset_y: libc::c_int = 0;
    xdg_popup_v6_box_constraints(popup, toplevel_sx_box, &mut offset_x,
                                 &mut offset_y);
    if offset_x == 0 && offset_y == 0 { return 1i32 != 0 }
    let mut resize_x: bool =
        offset_x != 0 &&
            (*popup).positioner.constraint_adjustment as libc::c_uint &
                ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_RESIZE_X as
                    libc::c_int as libc::c_uint != 0;
    let mut resize_y: bool =
        offset_y != 0 &&
            (*popup).positioner.constraint_adjustment as libc::c_uint &
                ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_RESIZE_Y as
                    libc::c_int as libc::c_uint != 0;
    if resize_x { (*popup).geometry.width -= offset_x }
    if resize_y { (*popup).geometry.height -= offset_y }
    xdg_popup_v6_box_constraints(popup, toplevel_sx_box, &mut offset_x,
                                 &mut offset_y);
    return offset_x == 0 && offset_y == 0;
}
/*
 * This protocol is obsolete and will be removed in a future version. The
 * recommended replacement is xdg-shell.
 */
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * An interface enabling clients to turn their wl_surfaces into windows in a
 * desktop environment.
 */
/* *
		 * The `new_surface` event signals that a client has requested to
		 * create a new shell surface. At this point, the surface is ready to
		 * be configured but is not mapped or ready receive input events. The
		 * surface will be ready to be managed on the `map` event.
		 */
// wlr_xdg_shell_v6::clients
// Position of the popup relative to the upper left corner of the window
	// geometry of the parent surface
// wlr_xdg_popup_grab_v6::popups
// each seat gets a popup grab
// wlr_xdg_shell_v6::popup_grabs
// Since the fullscreen request may be made before the toplevel's surface
	// is mapped, this is used to store the requested fullscreen output (if
	// any) for wlr_xdg_toplevel_v6::client_pending.
/* *
 * An xdg-surface is a user interface element requiring management by the
 * compositor. An xdg-surface alone isn't useful, a role should be assigned to
 * it in order to map it.
 *
 * When a surface has a role and is ready to be displayed, the `map` event is
 * emitted. When a surface should no longer be displayed, the `unmap` event is
 * emitted. The `unmap` event is guaranteed to be emitted before the `destroy`
 * event if the view is destroyed when mapped.
 */
// wlr_xdg_surface_v6::configure_list
// wlr_xdg_client_v6::surfaces
// wlr_xdg_popup_v6::link
/* *
		 * The `map` event signals that the shell surface is ready to be
		 * managed by the compositor and rendered on the screen. At this point,
		 * the surface has configured its properties, has had the opportunity
		 * to bind to the seat to receive input events, and has a buffer that
		 * is ready to be rendered. You can now safely add this surface to a
		 * list of views.
		 */
/* *
		 * The `unmap` event signals that the surface is no longer in a state
		 * where it should be shown on the screen. This might happen if the
		 * surface no longer has a displayable buffer because either the
		 * surface has been hidden or is about to be destroyed.
		 */
/* *
 * Send a ping to the surface. If the surface does not respond in a reasonable
 * amount of time, the ping_timeout event will be emitted.
 */
/* *
 * Request that this toplevel surface be the given size. Returns the associated
 * configure serial.
 */
/* *
 * Request that this toplevel surface show itself in an activated or deactivated
 * state. Returns the associated configure serial.
 */
/* *
 * Request that this toplevel surface consider itself maximized or not
 * maximized. Returns the associated configure serial.
 */
/* *
 * Request that this toplevel surface consider itself fullscreen or not
 * fullscreen. Returns the associated configure serial.
 */
/* *
 * Request that this toplevel surface consider itself to be resizing or not
 * resizing. Returns the associated configure serial.
 */
/* *
 * Request that this xdg surface closes.
 */
/* *
 * Find a surface within this xdg-surface tree at the given surface-local
 * coordinates. Returns the surface and coordinates in the leaf surface
 * coordinate system or NULL if no surface is found at that location.
 */
/* *
 * Get the geometry for this positioner based on the anchor rect, gravity, and
 * size of this positioner.
 */
/* *
 * Get the anchor point for this popup in the toplevel parent's coordinate system.
 */
/* *
 * Convert the given coordinates in the popup coordinate system to the toplevel
 * surface coordinate system.
 */
/* *
 * Set the geometry of this popup to unconstrain it according to its
 * xdg-positioner rules. The box should be in the popup's root toplevel parent
 * surface coordinate system.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_popup_v6_unconstrain_from_box(mut popup:
                                                                   *mut wlr_xdg_popup_v6,
                                                               mut toplevel_sx_box:
                                                                   *mut wlr_box) {
    if xdg_popup_v6_unconstrain_flip(popup, toplevel_sx_box) { return }
    if xdg_popup_v6_unconstrain_slide(popup, toplevel_sx_box) { return }
    if xdg_popup_v6_unconstrain_resize(popup, toplevel_sx_box) { return };
}
