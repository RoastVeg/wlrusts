use libc;
extern "C" {
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
    pub type wlr_pointer_impl;
    pub type wlr_tablet_pad_impl;
    pub type wlr_tablet_impl;
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    /* Note: these are circular dependencies */
    pub type wlr_input_device_impl;
    pub type wlr_texture;
    pub type wlr_renderer;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type wlr_tablet_manager_client_v2;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
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
    static zwp_tablet_v2_interface: wl_interface;
    #[no_mangle]
    fn get_or_create_tablet_seat(manager: *mut wlr_tablet_manager_v2,
                                 wlr_seat: *mut wlr_seat)
     -> *mut wlr_tablet_seat_v2;
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
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
/* *
 * @ingroup iface_zwp_tablet_v2
 * @struct zwp_tablet_v2_interface
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwp_tablet_v2_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
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
pub type wlr_input_device_type = libc::c_uint;
pub const WLR_INPUT_DEVICE_SWITCH: wlr_input_device_type = 5;
pub const WLR_INPUT_DEVICE_TABLET_PAD: wlr_input_device_type = 4;
pub const WLR_INPUT_DEVICE_TABLET_TOOL: wlr_input_device_type = 3;
pub const WLR_INPUT_DEVICE_TOUCH: wlr_input_device_type = 2;
pub const WLR_INPUT_DEVICE_POINTER: wlr_input_device_type = 1;
pub const WLR_INPUT_DEVICE_KEYBOARD: wlr_input_device_type = 0;
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_pointer {
    pub impl_0: *const crate::src::backend::headless::input_device::wlr_pointer_impl,
    pub events: C2RustUnnamed_1,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub motion: wl_signal,
    pub motion_absolute: wl_signal,
    pub button: wl_signal,
    pub axis: wl_signal,
    pub frame: wl_signal,
    pub swipe_begin: wl_signal,
    pub swipe_update: wl_signal,
    pub swipe_end: wl_signal,
    pub pinch_begin: wl_signal,
    pub pinch_update: wl_signal,
    pub pinch_end: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_input_device {
    pub impl_0: *const crate::src::backend::headless::backend::wlr_input_device_impl,
    pub type_0: wlr_input_device_type,
    pub vendor: libc::c_uint,
    pub product: libc::c_uint,
    pub name: *mut libc::c_char,
    pub width_mm: libc::c_double,
    pub height_mm: libc::c_double,
    pub output_name: *mut libc::c_char,
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub events: C2RustUnnamed_2,
    pub data: *mut libc::c_void,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
}

#[repr ( C )]#[derive(Copy, Clone)]
pub union C2RustUnnamed_3 {
    pub _device: *mut libc::c_void,
    pub keyboard: *mut wlr_keyboard,
    pub pointer: *mut wlr_pointer,
    pub switch_device: *mut wlr_switch,
    pub touch: *mut wlr_touch,
    pub tablet: *mut wlr_tablet,
    pub tablet_pad: *mut wlr_tablet_pad,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/*
 * NOTE: the wlr tablet pad implementation does not currently support tablets
 * with more than one mode. I don't own any such hardware so I cannot test it
 * and it is too complicated to make a meaningful implementation of blindly.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_pad {
    pub impl_0: *mut crate::src::backend::headless::input_device::wlr_tablet_pad_impl,
    pub events: C2RustUnnamed_4,
    pub button_count: size_t,
    pub ring_count: size_t,
    pub strip_count: size_t,
    pub groups: wl_list,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_list {
    pub capacity: size_t,
    pub length: size_t,
    pub items: *mut *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub button: wl_signal,
    pub ring: wl_signal,
    pub strip: wl_signal,
    pub attach_tablet: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/*
 * Copy+Paste from libinput, but this should neither use libinput, nor
 * tablet-unstable-v2 headers, so we can't include them
 */
/* * A generic pen */
/* * Eraser */
/* * A paintbrush-like tool */
/* * Physical drawing tool, e.g. Wacom Inking Pen */
/* * An airbrush-like tool */
/* * A mouse bound to the tablet */
/* * A mouse tool with a lens */
/* * A rotary device with positional and rotation data */
// Capabilities

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet {
    pub impl_0: *mut crate::src::backend::headless::input_device::wlr_tablet_impl,
    pub events: C2RustUnnamed_5,
    pub name: *mut libc::c_char,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub axis: wl_signal,
    pub proximity: wl_signal,
    pub tip: wl_signal,
    pub button: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_touch {
    pub impl_0: *const crate::src::backend::headless::input_device::wlr_touch_impl,
    pub events: C2RustUnnamed_6,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub down: wl_signal,
    pub up: wl_signal,
    pub motion: wl_signal,
    pub cancel: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_switch {
    pub impl_0: *mut crate::src::backend::headless::input_device::wlr_switch_impl,
    pub events: C2RustUnnamed_7,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub toggle: wl_signal,
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
    pub events: C2RustUnnamed_8,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
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
    pub events: C2RustUnnamed_9,
    pub serials: wlr_serial_ringset,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
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
    pub events: C2RustUnnamed_10,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
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
    pub events: C2RustUnnamed_11,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
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
    pub events: C2RustUnnamed_12,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_12 {
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
    pub events: C2RustUnnamed_13,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
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
pub struct wlr_tablet_seat_client_v2 {
    pub seat_link: wl_list,
    pub client_link: wl_list,
    pub wl_client: *mut wl_client,
    pub resource: *mut wl_resource,
    pub client: *mut crate::src::types::tablet_v2::wlr_tablet_v2::wlr_tablet_manager_client_v2,
    pub seat_client: *mut wlr_seat_client,
    pub seat_client_destroy: wl_listener,
    pub tools: wl_list,
    pub tablets: wl_list,
    pub pads: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_v2_tablet {
    pub link: wl_list,
    pub wlr_tablet: *mut wlr_tablet,
    pub wlr_device: *mut wlr_input_device,
    pub clients: wl_list,
    pub tool_destroy: wl_listener,
    pub current_client: *mut wlr_tablet_client_v2,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_client_v2 {
    pub seat_link: wl_list,
    pub tablet_link: wl_list,
    pub client: *mut wl_client,
    pub resource: *mut wl_resource,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_manager_v2 {
    pub wl_global: *mut wl_global,
    pub clients: wl_list,
    pub seats: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_14,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_14 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_seat_v2 {
    pub link: wl_list,
    pub wlr_seat: *mut wlr_seat,
    pub manager: *mut wlr_tablet_manager_v2,
    pub tablets: wl_list,
    pub tools: wl_list,
    pub pads: wl_list,
    pub clients: wl_list,
    pub seat_destroy: wl_listener,
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
#[inline]
unsafe extern "C" fn zwp_tablet_seat_v2_send_tablet_added(mut resource_:
                                                              *mut wl_resource,
                                                          mut id:
                                                              *mut wl_resource) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, id);
}
/* *
 * @ingroup iface_zwp_tablet_v2
 */
/* *
 * @ingroup iface_zwp_tablet_v2
 */
/* *
 * @ingroup iface_zwp_tablet_v2
 */
/* *
 * @ingroup iface_zwp_tablet_v2
 */
/* *
 * @ingroup iface_zwp_tablet_v2
 */
/* *
 * @ingroup iface_zwp_tablet_v2
 */
/* *
 * @ingroup iface_zwp_tablet_v2
 * Sends an name event to the client owning the resource.
 * @param resource_ The client's resource
 * @param name the device name
 */
#[inline]
unsafe extern "C" fn zwp_tablet_v2_send_name(mut resource_: *mut wl_resource,
                                             mut name: *const libc::c_char) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, name);
}
/* *
 * @ingroup iface_zwp_tablet_v2
 * Sends an id event to the client owning the resource.
 * @param resource_ The client's resource
 * @param vid USB vendor id
 * @param pid USB product id
 */
#[inline]
unsafe extern "C" fn zwp_tablet_v2_send_id(mut resource_: *mut wl_resource,
                                           mut vid: uint32_t,
                                           mut pid: uint32_t) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, vid, pid);
}
/* *
 * @ingroup iface_zwp_tablet_v2
 * Sends an path event to the client owning the resource.
 * @param resource_ The client's resource
 * @param path path to local device
 */
#[inline]
unsafe extern "C" fn zwp_tablet_v2_send_path(mut resource_: *mut wl_resource,
                                             mut path: *const libc::c_char) {
    wl_resource_post_event(resource_, 2i32 as uint32_t, path);
}
/* *
 * @ingroup iface_zwp_tablet_v2
 * Sends an done event to the client owning the resource.
 * @param resource_ The client's resource
 */
#[inline]
unsafe extern "C" fn zwp_tablet_v2_send_done(mut resource_:
                                                 *mut wl_resource) {
    wl_resource_post_event(resource_, 3i32 as uint32_t);
}
/* *
 * @ingroup iface_zwp_tablet_v2
 * Sends an removed event to the client owning the resource.
 * @param resource_ The client's resource
 */
#[inline]
unsafe extern "C" fn zwp_tablet_v2_send_removed(mut resource_:
                                                    *mut wl_resource) {
    wl_resource_post_event(resource_, 4i32 as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn destroy_tablet_v2(mut resource: *mut wl_resource) {
    let mut tablet: *mut wlr_tablet_client_v2 =
        tablet_client_from_resource(resource);
    if tablet.is_null() { return }
    wl_list_remove(&mut (*tablet).seat_link);
    wl_list_remove(&mut (*tablet).tablet_link);
    free(tablet as *mut libc::c_void);
    wl_resource_set_user_data(resource, 0 as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_v2_destroy(mut client: *mut wl_client,
                                              mut resource:
                                                  *mut wl_resource) {
    wl_resource_destroy(resource);
}
static mut tablet_impl: zwp_tablet_v2_interface =
    {
    
        {
            let mut init =
                zwp_tablet_v2_interface{destroy:
                                            Some(handle_tablet_v2_destroy as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource)
                                                         -> ()),};
            init
        }
};
unsafe extern "C" fn handle_wlr_tablet_destroy(mut listener: *mut wl_listener,
                                               mut data: *mut libc::c_void) {
    let mut tablet: *mut wlr_tablet_v2_tablet =
        (listener as *mut libc::c_char).offset(-48) as
            *mut wlr_tablet_v2_tablet;
    let mut pos: *mut wlr_tablet_client_v2 = 0 as *mut wlr_tablet_client_v2;
    let mut tmp: *mut wlr_tablet_client_v2 = 0 as *mut wlr_tablet_client_v2;
    pos =
        ((*tablet).clients.next as *mut libc::c_char).offset(-16) as
            *mut wlr_tablet_client_v2;
    tmp =
        ((*pos).tablet_link.next as *mut libc::c_char).offset(-16) as
            *mut wlr_tablet_client_v2;
    while &mut (*pos).tablet_link as *mut wl_list !=
              &mut (*tablet).clients as *mut wl_list {
        zwp_tablet_v2_send_removed((*pos).resource);
        pos = tmp;
        tmp =
            ((*pos).tablet_link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_tablet_client_v2
    }
    wl_list_remove(&mut (*tablet).clients);
    wl_list_remove(&mut (*tablet).link);
    wl_list_remove(&mut (*tablet).tool_destroy.link);
    free(tablet as *mut libc::c_void);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* This can probably be even lower,the tools don't have a lot of buttons */
// wlr_tablet_manager_client_v2::link
// wlr_tablet_seat_v2::link
// wlr_tablet_seat_v2::tablets
// wlr_tablet_client_v2::tablet_link
// wlr_tablet_seat_v2::tablets
// wlr_tablet_tool_client_v2::tool_link
// struct wlr_tablet_v2_event_cursor
// wlr_tablet_seat_v2::pads
// wlr_tablet_pad_client_v2::pad_link
// struct wlr_tablet_v2_event_feedback
// struct wlr_tablet_v2_event_feedback
// struct wlr_tablet_v2_event_feedback
#[no_mangle]
pub unsafe extern "C" fn wlr_tablet_create(mut manager:
                                               *mut wlr_tablet_manager_v2,
                                           mut wlr_seat: *mut wlr_seat,
                                           mut wlr_device:
                                               *mut wlr_input_device)
 -> *mut wlr_tablet_v2_tablet {
    if (*wlr_device).type_0 as libc::c_uint ==
           WLR_INPUT_DEVICE_TABLET_TOOL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"wlr_device->type == WLR_INPUT_DEVICE_TABLET_TOOL\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/tablet_v2/wlr_tablet_v2_tablet.c\x00" as
                          *const u8 as *const libc::c_char,
                      57i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 125],
                                                &[libc::c_char; 125]>(b"struct wlr_tablet_v2_tablet *wlr_tablet_create(struct wlr_tablet_manager_v2 *, struct wlr_seat *, struct wlr_input_device *)\x00")).as_ptr());
    };
    let mut seat: *mut wlr_tablet_seat_v2 =
        get_or_create_tablet_seat(manager, wlr_seat);
    if seat.is_null() { return 0 as *mut wlr_tablet_v2_tablet }
    let mut wlr_tablet: *mut wlr_tablet = (*wlr_device).c2rust_unnamed.tablet;
    let mut tablet: *mut wlr_tablet_v2_tablet =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_tablet_v2_tablet>() as libc::c_ulong)
            as *mut wlr_tablet_v2_tablet;
    if tablet.is_null() { return 0 as *mut wlr_tablet_v2_tablet }
    (*tablet).wlr_tablet = wlr_tablet;
    (*tablet).wlr_device = wlr_device;
    wl_list_init(&mut (*tablet).clients);
    (*tablet).tool_destroy.notify =
        Some(handle_wlr_tablet_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*wlr_device).events.destroy,
                  &mut (*tablet).tool_destroy);
    wl_list_insert(&mut (*seat).tablets, &mut (*tablet).link);
    // We need to create a tablet client for all clients on the seat
    let mut pos: *mut wlr_tablet_seat_client_v2 =
        0 as *mut wlr_tablet_seat_client_v2;
    pos =
        ((*seat).clients.next as *mut libc::c_char).offset(-0) as
            *mut wlr_tablet_seat_client_v2;
    while &mut (*pos).seat_link as *mut wl_list !=
              &mut (*seat).clients as *mut wl_list {
        // Tell the clients about the new tool
        add_tablet_client(pos, tablet);
        pos =
            ((*pos).seat_link.next as *mut libc::c_char).offset(-0) as
                *mut wlr_tablet_seat_client_v2
    }
    return tablet;
}
#[no_mangle]
pub unsafe extern "C" fn add_tablet_client(mut seat:
                                               *mut wlr_tablet_seat_client_v2,
                                           mut tablet:
                                               *mut wlr_tablet_v2_tablet) {
    let mut client: *mut wlr_tablet_client_v2 =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_tablet_client_v2>() as libc::c_ulong)
            as *mut wlr_tablet_client_v2;
    if client.is_null() { return }
    let mut version: uint32_t =
        wl_resource_get_version((*seat).resource) as uint32_t;
    (*client).resource =
        wl_resource_create((*seat).wl_client, &zwp_tablet_v2_interface,
                           version as libc::c_int, 0i32 as uint32_t);
    if (*client).resource.is_null() {
        wl_resource_post_no_memory((*seat).resource);
        free(client as *mut libc::c_void);
        return
    }
    wl_resource_set_implementation((*client).resource,
                                   &mut tablet_impl as
                                       *mut zwp_tablet_v2_interface as
                                       *const libc::c_void,
                                   client as *mut libc::c_void,
                                   Some(destroy_tablet_v2 as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    zwp_tablet_seat_v2_send_tablet_added((*seat).resource,
                                         (*client).resource);
    // Send the expected events
    if !(*(*tablet).wlr_tablet).name.is_null() {
        zwp_tablet_v2_send_name((*client).resource,
                                (*(*tablet).wlr_tablet).name);
    }
    zwp_tablet_v2_send_id((*client).resource, (*(*tablet).wlr_device).vendor,
                          (*(*tablet).wlr_device).product);
    let mut i: size_t = 0i32 as size_t;
    while i < (*(*tablet).wlr_tablet).paths.length {
        zwp_tablet_v2_send_path((*client).resource,
                                *(*(*tablet).wlr_tablet).paths.items.offset(i
                                                                                as
                                                                                isize)
                                    as *const libc::c_char);
        i = i.wrapping_add(1)
    }
    zwp_tablet_v2_send_done((*client).resource);
    (*client).client = (*seat).wl_client;
    wl_list_insert(&mut (*seat).tablets, &mut (*client).seat_link);
    wl_list_insert(&mut (*tablet).clients, &mut (*client).tablet_link);
}
// wlr_tablet_manager_v2::seats
// wlr_tablet_v2_tablet::link
// wlr_tablet_seat_v2_client::link
//wlr_tablet_tool_client_v2::link
//wlr_tablet_client_v2::link
//wlr_tablet_pad_client_v2::link
// wlr_tablet_seat_client_v2::tablet
// wlr_tablet_v2_tablet::clients
#[no_mangle]
pub unsafe extern "C" fn tablet_client_from_resource(mut resource:
                                                         *mut wl_resource)
 -> *mut wlr_tablet_client_v2 {
    if wl_resource_instance_of(resource, &zwp_tablet_v2_interface,
                               &mut tablet_impl as
                                   *mut zwp_tablet_v2_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &zwp_tablet_v2_interface, &tablet_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/tablet_v2/wlr_tablet_v2_tablet.c\x00" as
                          *const u8 as *const libc::c_char,
                      130i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 79],
                                                &[libc::c_char; 79]>(b"struct wlr_tablet_client_v2 *tablet_client_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_tablet_client_v2;
}
