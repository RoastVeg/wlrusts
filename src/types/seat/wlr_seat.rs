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
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn wl_display_get_serial(display: *mut wl_display) -> uint32_t;
    #[no_mangle]
    fn wl_display_next_serial(display: *mut wl_display) -> uint32_t;
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
    fn wl_client_get_display(client: *mut wl_client) -> *mut wl_display;
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
    fn wl_resource_set_user_data(resource: *mut wl_resource,
                                 data: *mut libc::c_void);
    #[no_mangle]
    fn wl_resource_instance_of(resource: *mut wl_resource,
                               interface: *const wl_interface,
                               implementation: *const libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn wl_resource_get_version(resource: *mut wl_resource) -> libc::c_int;
    #[no_mangle]
    fn wl_resource_get_user_data(resource: *mut wl_resource)
     -> *mut libc::c_void;
    #[no_mangle]
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    static wl_seat_interface: wl_interface;
    #[no_mangle]
    fn wlr_data_source_destroy(source: *mut wlr_data_source);
    #[no_mangle]
    fn wlr_primary_selection_source_destroy(source:
                                                *mut wlr_primary_selection_source);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn seat_client_create_pointer(seat_client: *mut wlr_seat_client,
                                  version: uint32_t, id: uint32_t);
    #[no_mangle]
    fn seat_client_destroy_pointer(resource: *mut wl_resource);
    #[no_mangle]
    fn seat_client_create_keyboard(seat_client: *mut wlr_seat_client,
                                   version: uint32_t, id: uint32_t);
    #[no_mangle]
    fn seat_client_destroy_keyboard(resource: *mut wl_resource);
    #[no_mangle]
    fn seat_client_create_touch(seat_client: *mut wlr_seat_client,
                                version: uint32_t, id: uint32_t);
    #[no_mangle]
    fn seat_client_destroy_touch(resource: *mut wl_resource);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type wl_seat_capability = libc::c_uint;
pub const WL_SEAT_CAPABILITY_TOUCH: wl_seat_capability = 4;
pub const WL_SEAT_CAPABILITY_KEYBOARD: wl_seat_capability = 2;
pub const WL_SEAT_CAPABILITY_POINTER: wl_seat_capability = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_seat_interface {
    pub get_pointer: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                 _: *mut wl_resource,
                                                 _: uint32_t) -> ()>,
    pub get_keyboard: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                  _: *mut wl_resource,
                                                  _: uint32_t) -> ()>,
    pub get_touch: Option<unsafe extern "C" fn(_: *mut wl_client,
                                               _: *mut wl_resource,
                                               _: uint32_t) -> ()>,
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
pub struct wlr_primary_selection_source {
    pub impl_0: *const wlr_primary_selection_source_impl,
    pub mime_types: wl_array,
    pub events: C2RustUnnamed_10,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
unsafe extern "C" fn wl_seat_send_capabilities(mut resource_:
                                                   *mut wl_resource,
                                               mut capabilities: uint32_t) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, capabilities);
}
#[inline]
unsafe extern "C" fn wl_seat_send_name(mut resource_: *mut wl_resource,
                                       mut name: *const libc::c_char) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, name);
}
#[no_mangle]
pub static mut default_pointer_grab_impl: wlr_pointer_grab_interface =
    wlr_pointer_grab_interface{enter: None,
                               motion: None,
                               button: None,
                               axis: None,
                               frame: None,
                               cancel: None,};
#[no_mangle]
pub static mut default_keyboard_grab_impl: wlr_keyboard_grab_interface =
    wlr_keyboard_grab_interface{enter: None,
                                key: None,
                                modifiers: None,
                                cancel: None,};
#[no_mangle]
pub static mut default_touch_grab_impl: wlr_touch_grab_interface =
    wlr_touch_grab_interface{down: None,
                             up: None,
                             motion: None,
                             enter: None,
                             cancel: None,};
unsafe extern "C" fn seat_handle_get_pointer(mut client: *mut wl_client,
                                             mut seat_resource:
                                                 *mut wl_resource,
                                             mut id: uint32_t) {
    let mut seat_client: *mut wlr_seat_client =
        wlr_seat_client_from_resource(seat_resource);
    if (*(*seat_client).seat).capabilities &
           WL_SEAT_CAPABILITY_POINTER as libc::c_int as libc::c_uint == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Client sent get_pointer on seat without the pointer capability\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat.c\x00" as *const u8 as
                     *const libc::c_char, 23i32);
        return
    }
    let mut version: uint32_t =
        wl_resource_get_version(seat_resource) as uint32_t;
    seat_client_create_pointer(seat_client, version, id);
}
unsafe extern "C" fn seat_handle_get_keyboard(mut client: *mut wl_client,
                                              mut seat_resource:
                                                  *mut wl_resource,
                                              mut id: uint32_t) {
    let mut seat_client: *mut wlr_seat_client =
        wlr_seat_client_from_resource(seat_resource);
    if (*(*seat_client).seat).capabilities &
           WL_SEAT_CAPABILITY_KEYBOARD as libc::c_int as libc::c_uint == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Client sent get_keyboard on seat without the keyboard capability\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat.c\x00" as *const u8 as
                     *const libc::c_char, 37i32);
        return
    }
    let mut version: uint32_t =
        wl_resource_get_version(seat_resource) as uint32_t;
    seat_client_create_keyboard(seat_client, version, id);
}
unsafe extern "C" fn seat_handle_get_touch(mut client: *mut wl_client,
                                           mut seat_resource:
                                               *mut wl_resource,
                                           mut id: uint32_t) {
    let mut seat_client: *mut wlr_seat_client =
        wlr_seat_client_from_resource(seat_resource);
    if (*(*seat_client).seat).capabilities &
           WL_SEAT_CAPABILITY_TOUCH as libc::c_int as libc::c_uint == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Client sent get_touch on seat without the touch capability\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/seat/wlr_seat.c\x00" as *const u8 as
                     *const libc::c_char, 51i32);
        return
    }
    let mut version: uint32_t =
        wl_resource_get_version(seat_resource) as uint32_t;
    seat_client_create_touch(seat_client, version, id);
}
unsafe extern "C" fn seat_client_handle_resource_destroy(mut seat_resource:
                                                             *mut wl_resource) {
    let mut client: *mut wlr_seat_client =
        wlr_seat_client_from_resource(seat_resource);
    wl_list_remove(wl_resource_get_link(seat_resource));
    if wl_list_empty(&mut (*client).resources) == 0 { return }
    wlr_signal_emit_safe(&mut (*client).events.destroy,
                         client as *mut libc::c_void);
    if client == (*(*client).seat).pointer_state.focused_client {
        (*(*client).seat).pointer_state.focused_client =
            0 as *mut wlr_seat_client
    }
    if client == (*(*client).seat).keyboard_state.focused_client {
        (*(*client).seat).keyboard_state.focused_client =
            0 as *mut wlr_seat_client
    }
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    let mut tmp: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    tmp = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).pointers.next);
    tmp = wl_resource_from_link((*(*client).pointers.next).next);
    while wl_resource_get_link(resource) !=
              &mut (*client).pointers as *mut wl_list {
        wl_resource_destroy(resource);
        resource = tmp;
        tmp = wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    resource = 0 as *mut wl_resource;
    tmp = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).keyboards.next);
    tmp = wl_resource_from_link((*(*client).keyboards.next).next);
    while wl_resource_get_link(resource) !=
              &mut (*client).keyboards as *mut wl_list {
        wl_resource_destroy(resource);
        resource = tmp;
        tmp = wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    resource = 0 as *mut wl_resource;
    tmp = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).touches.next);
    tmp = wl_resource_from_link((*(*client).touches.next).next);
    while wl_resource_get_link(resource) !=
              &mut (*client).touches as *mut wl_list {
        wl_resource_destroy(resource);
        resource = tmp;
        tmp = wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    resource = 0 as *mut wl_resource;
    tmp = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).data_devices.next);
    tmp = wl_resource_from_link((*(*client).data_devices.next).next);
    while wl_resource_get_link(resource) !=
              &mut (*client).data_devices as *mut wl_list {
        // Make the data device inert
        wl_resource_set_user_data(resource, 0 as *mut libc::c_void);
        let mut link: *mut wl_list = wl_resource_get_link(resource);
        wl_list_remove(link);
        wl_list_init(link);
        resource = tmp;
        tmp = wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    wl_list_remove(&mut (*client).link);
    free(client as *mut libc::c_void);
}
unsafe extern "C" fn seat_handle_release(mut client: *mut wl_client,
                                         mut resource: *mut wl_resource) {
    wl_resource_destroy(resource);
}
static mut seat_impl: wl_seat_interface =
    unsafe {
        {
            let mut init =
                wl_seat_interface{get_pointer:
                                      Some(seat_handle_get_pointer as
                                               unsafe extern "C" fn(_:
                                                                        *mut wl_client,
                                                                    _:
                                                                        *mut wl_resource,
                                                                    _:
                                                                        uint32_t)
                                                   -> ()),
                                  get_keyboard:
                                      Some(seat_handle_get_keyboard as
                                               unsafe extern "C" fn(_:
                                                                        *mut wl_client,
                                                                    _:
                                                                        *mut wl_resource,
                                                                    _:
                                                                        uint32_t)
                                                   -> ()),
                                  get_touch:
                                      Some(seat_handle_get_touch as
                                               unsafe extern "C" fn(_:
                                                                        *mut wl_client,
                                                                    _:
                                                                        *mut wl_resource,
                                                                    _:
                                                                        uint32_t)
                                                   -> ()),
                                  release:
                                      Some(seat_handle_release as
                                               unsafe extern "C" fn(_:
                                                                        *mut wl_client,
                                                                    _:
                                                                        *mut wl_resource)
                                                   -> ()),};
            init
        }
    };
unsafe extern "C" fn seat_handle_bind(mut client: *mut wl_client,
                                      mut _wlr_seat: *mut libc::c_void,
                                      mut version: uint32_t,
                                      mut id: uint32_t) {
    let mut wlr_seat: *mut wlr_seat = _wlr_seat as *mut wlr_seat;
    if !client.is_null() && !wlr_seat.is_null() {
    } else {
        __assert_fail(b"client && wlr_seat\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/seat/wlr_seat.c\x00" as *const u8 as
                          *const libc::c_char, 116i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"void seat_handle_bind(struct wl_client *, void *, uint32_t, uint32_t)\x00")).as_ptr());
    };
    let mut wl_resource: *mut wl_resource =
        wl_resource_create(client, &wl_seat_interface, version as libc::c_int,
                           id);
    if wl_resource.is_null() { wl_client_post_no_memory(client); return }
    let mut seat_client: *mut wlr_seat_client =
        wlr_seat_client_for_wl_client(wlr_seat, client);
    if seat_client.is_null() {
        seat_client =
            calloc(1i32 as libc::c_ulong,
                   ::std::mem::size_of::<wlr_seat_client>() as libc::c_ulong)
                as *mut wlr_seat_client;
        if seat_client.is_null() {
            wl_resource_destroy(wl_resource);
            wl_client_post_no_memory(client);
            return
        }
        (*seat_client).client = client;
        (*seat_client).seat = wlr_seat;
        wl_list_init(&mut (*seat_client).resources);
        wl_list_init(&mut (*seat_client).pointers);
        wl_list_init(&mut (*seat_client).keyboards);
        wl_list_init(&mut (*seat_client).touches);
        wl_list_init(&mut (*seat_client).data_devices);
        wl_signal_init(&mut (*seat_client).events.destroy);
        wl_list_insert(&mut (*wlr_seat).clients, &mut (*seat_client).link);
    }
    wl_resource_set_implementation(wl_resource,
                                   &seat_impl as *const wl_seat_interface as
                                       *const libc::c_void,
                                   seat_client as *mut libc::c_void,
                                   Some(seat_client_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_insert(&mut (*seat_client).resources,
                   wl_resource_get_link(wl_resource));
    if version >= 2i32 as libc::c_uint {
        wl_seat_send_name(wl_resource, (*wlr_seat).name);
    }
    wl_seat_send_capabilities(wl_resource, (*wlr_seat).capabilities);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_destroy(mut seat: *mut wlr_seat) {
    if seat.is_null() { return }
    wlr_signal_emit_safe(&mut (*seat).events.destroy,
                         seat as *mut libc::c_void);
    wl_list_remove(&mut (*seat).display_destroy.link);
    wlr_data_source_destroy((*seat).selection_source);
    wlr_primary_selection_source_destroy((*seat).primary_selection_source);
    let mut client: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
    let mut tmp: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
    client =
        ((*seat).clients.next as *mut libc::c_char).offset(-16) as
            *mut wlr_seat_client;
    tmp =
        ((*client).link.next as *mut libc::c_char).offset(-16) as
            *mut wlr_seat_client;
    while &mut (*client).link as *mut wl_list !=
              &mut (*seat).clients as *mut wl_list {
        let mut resource: *mut wl_resource = 0 as *mut wl_resource;
        let mut next: *mut wl_resource = 0 as *mut wl_resource;
        /* wl_resource_for_each_safe isn't safe to use here, because the last
		 * wl_resource_destroy will also destroy the head we cannot do the last
		 * 'next' update that usually is harmless here.
		 * Work around this by breaking one step ahead
		 */
        resource = 0 as *mut wl_resource;
        next = 0 as *mut wl_resource;
        resource = wl_resource_from_link((*client).resources.next);
        next = wl_resource_from_link((*(*client).resources.next).next);
        while wl_resource_get_link(resource) !=
                  &mut (*client).resources as *mut wl_list {
            // will destroy other resources as well
            wl_resource_destroy(resource);
            if wl_resource_get_link(next) ==
                   &mut (*client).resources as *mut wl_list {
                break ;
            }
            resource = next;
            next =
                wl_resource_from_link((*wl_resource_get_link(resource)).next)
        }
        client = tmp;
        tmp =
            ((*client).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_seat_client
    }
    wl_global_destroy((*seat).global);
    free((*seat).pointer_state.default_grab as *mut libc::c_void);
    free((*seat).keyboard_state.default_grab as *mut libc::c_void);
    free((*seat).touch_state.default_grab as *mut libc::c_void);
    free((*seat).name as *mut libc::c_void);
    free(seat as *mut libc::c_void);
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut seat: *mut wlr_seat =
        (listener as *mut libc::c_char).offset(-544) as *mut wlr_seat;
    wlr_seat_destroy(seat);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_create(mut display: *mut wl_display,
                                         mut name: *const libc::c_char)
 -> *mut wlr_seat {
    let mut seat: *mut wlr_seat =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_seat>() as libc::c_ulong) as
            *mut wlr_seat;
    if seat.is_null() { return 0 as *mut wlr_seat }
    // pointer state
    (*seat).pointer_state.seat = seat;
    wl_list_init(&mut (*seat).pointer_state.surface_destroy.link);
    let mut pointer_grab: *mut wlr_seat_pointer_grab =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_seat_pointer_grab>() as
                   libc::c_ulong) as *mut wlr_seat_pointer_grab;
    if pointer_grab.is_null() {
        free(seat as *mut libc::c_void);
        return 0 as *mut wlr_seat
    }
    (*pointer_grab).interface = &default_pointer_grab_impl;
    (*pointer_grab).seat = seat;
    (*seat).pointer_state.default_grab = pointer_grab;
    (*seat).pointer_state.grab = pointer_grab;
    wl_signal_init(&mut (*seat).pointer_state.events.focus_change);
    // keyboard state
    let mut keyboard_grab: *mut wlr_seat_keyboard_grab =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_seat_keyboard_grab>() as
                   libc::c_ulong) as *mut wlr_seat_keyboard_grab;
    if keyboard_grab.is_null() {
        free(pointer_grab as *mut libc::c_void);
        free(seat as *mut libc::c_void);
        return 0 as *mut wlr_seat
    }
    (*keyboard_grab).interface = &default_keyboard_grab_impl;
    (*keyboard_grab).seat = seat;
    (*seat).keyboard_state.default_grab = keyboard_grab;
    (*seat).keyboard_state.grab = keyboard_grab;
    (*seat).keyboard_state.seat = seat;
    wl_list_init(&mut (*seat).keyboard_state.surface_destroy.link);
    wl_signal_init(&mut (*seat).keyboard_state.events.focus_change);
    // touch state
    let mut touch_grab: *mut wlr_seat_touch_grab =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_seat_touch_grab>() as libc::c_ulong)
            as *mut wlr_seat_touch_grab;
    if touch_grab.is_null() {
        free(pointer_grab as *mut libc::c_void);
        free(keyboard_grab as *mut libc::c_void);
        free(seat as *mut libc::c_void);
        return 0 as *mut wlr_seat
    }
    (*touch_grab).interface = &default_touch_grab_impl;
    (*touch_grab).seat = seat;
    (*seat).touch_state.default_grab = touch_grab;
    (*seat).touch_state.grab = touch_grab;
    (*seat).touch_state.seat = seat;
    wl_list_init(&mut (*seat).touch_state.touch_points);
    // TODO: always use SEAT_VERSION (requires libwayland 1.17)
    let mut version: uint32_t = 7i32 as uint32_t;
    if wl_seat_interface.version < 7i32 {
        version = wl_seat_interface.version as uint32_t
    }
    (*seat).global =
        wl_global_create(display, &wl_seat_interface, version as libc::c_int,
                         seat as *mut libc::c_void,
                         Some(seat_handle_bind as
                                  unsafe extern "C" fn(_: *mut wl_client,
                                                       _: *mut libc::c_void,
                                                       _: uint32_t,
                                                       _: uint32_t) -> ()));
    if (*seat).global.is_null() {
        free(touch_grab as *mut libc::c_void);
        free(pointer_grab as *mut libc::c_void);
        free(keyboard_grab as *mut libc::c_void);
        free(seat as *mut libc::c_void);
        return 0 as *mut wlr_seat
    }
    (*seat).display = display;
    (*seat).name = strdup(name);
    wl_list_init(&mut (*seat).clients);
    wl_list_init(&mut (*seat).selection_offers);
    wl_list_init(&mut (*seat).drag_offers);
    wl_signal_init(&mut (*seat).events.request_start_drag);
    wl_signal_init(&mut (*seat).events.start_drag);
    wl_signal_init(&mut (*seat).events.request_set_cursor);
    wl_signal_init(&mut (*seat).events.request_set_selection);
    wl_signal_init(&mut (*seat).events.set_selection);
    wl_signal_init(&mut (*seat).events.request_set_primary_selection);
    wl_signal_init(&mut (*seat).events.set_primary_selection);
    wl_signal_init(&mut (*seat).events.pointer_grab_begin);
    wl_signal_init(&mut (*seat).events.pointer_grab_end);
    wl_signal_init(&mut (*seat).events.keyboard_grab_begin);
    wl_signal_init(&mut (*seat).events.keyboard_grab_end);
    wl_signal_init(&mut (*seat).events.touch_grab_begin);
    wl_signal_init(&mut (*seat).events.touch_grab_end);
    wl_signal_init(&mut (*seat).events.destroy);
    (*seat).display_destroy.notify =
        Some(handle_display_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_display_add_destroy_listener(display, &mut (*seat).display_destroy);
    return seat;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_client_for_wl_client(mut wlr_seat:
                                                           *mut wlr_seat,
                                                       mut wl_client:
                                                           *mut wl_client)
 -> *mut wlr_seat_client {
    let mut seat_client: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
    seat_client =
        ((*wlr_seat).clients.next as *mut libc::c_char).offset(-16) as
            *mut wlr_seat_client;
    while &mut (*seat_client).link as *mut wl_list !=
              &mut (*wlr_seat).clients as *mut wl_list {
        if (*seat_client).client == wl_client { return seat_client }
        seat_client =
            ((*seat_client).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_seat_client
    }
    return 0 as *mut wlr_seat_client;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_set_capabilities(mut wlr_seat:
                                                       *mut wlr_seat,
                                                   mut capabilities:
                                                       uint32_t) {
    (*wlr_seat).capabilities = capabilities;
    let mut client: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
    client =
        ((*wlr_seat).clients.next as *mut libc::c_char).offset(-16) as
            *mut wlr_seat_client;
    while &mut (*client).link as *mut wl_list !=
              &mut (*wlr_seat).clients as *mut wl_list {
        // Make resources inert if necessary
        if capabilities &
               WL_SEAT_CAPABILITY_POINTER as libc::c_int as libc::c_uint ==
               0i32 as libc::c_uint {
            let mut resource: *mut wl_resource = 0 as *mut wl_resource;
            let mut tmp: *mut wl_resource = 0 as *mut wl_resource;
            resource = 0 as *mut wl_resource;
            tmp = 0 as *mut wl_resource;
            resource = wl_resource_from_link((*client).pointers.next);
            tmp = wl_resource_from_link((*(*client).pointers.next).next);
            while wl_resource_get_link(resource) !=
                      &mut (*client).pointers as *mut wl_list {
                seat_client_destroy_pointer(resource);
                resource = tmp;
                tmp =
                    wl_resource_from_link((*wl_resource_get_link(resource)).next)
            }
        }
        if capabilities &
               WL_SEAT_CAPABILITY_KEYBOARD as libc::c_int as libc::c_uint ==
               0i32 as libc::c_uint {
            let mut resource_0: *mut wl_resource = 0 as *mut wl_resource;
            let mut tmp_0: *mut wl_resource = 0 as *mut wl_resource;
            resource_0 = 0 as *mut wl_resource;
            tmp_0 = 0 as *mut wl_resource;
            resource_0 = wl_resource_from_link((*client).keyboards.next);
            tmp_0 = wl_resource_from_link((*(*client).keyboards.next).next);
            while wl_resource_get_link(resource_0) !=
                      &mut (*client).keyboards as *mut wl_list {
                seat_client_destroy_keyboard(resource_0);
                resource_0 = tmp_0;
                tmp_0 =
                    wl_resource_from_link((*wl_resource_get_link(resource_0)).next)
            }
        }
        if capabilities &
               WL_SEAT_CAPABILITY_TOUCH as libc::c_int as libc::c_uint ==
               0i32 as libc::c_uint {
            let mut resource_1: *mut wl_resource = 0 as *mut wl_resource;
            let mut tmp_1: *mut wl_resource = 0 as *mut wl_resource;
            resource_1 = 0 as *mut wl_resource;
            tmp_1 = 0 as *mut wl_resource;
            resource_1 = wl_resource_from_link((*client).touches.next);
            tmp_1 = wl_resource_from_link((*(*client).touches.next).next);
            while wl_resource_get_link(resource_1) !=
                      &mut (*client).touches as *mut wl_list {
                seat_client_destroy_touch(resource_1);
                resource_1 = tmp_1;
                tmp_1 =
                    wl_resource_from_link((*wl_resource_get_link(resource_1)).next)
            }
        }
        let mut resource_2: *mut wl_resource = 0 as *mut wl_resource;
        resource_2 = 0 as *mut wl_resource;
        resource_2 = wl_resource_from_link((*client).resources.next);
        while wl_resource_get_link(resource_2) !=
                  &mut (*client).resources as *mut wl_list {
            wl_seat_send_capabilities(resource_2, capabilities);
            resource_2 =
                wl_resource_from_link((*wl_resource_get_link(resource_2)).next)
        }
        client =
            ((*client).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_seat_client
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_set_name(mut wlr_seat: *mut wlr_seat,
                                           mut name: *const libc::c_char) {
    free((*wlr_seat).name as *mut libc::c_void);
    (*wlr_seat).name = strdup(name);
    let mut client: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
    client =
        ((*wlr_seat).clients.next as *mut libc::c_char).offset(-16) as
            *mut wlr_seat_client;
    while &mut (*client).link as *mut wl_list !=
              &mut (*wlr_seat).clients as *mut wl_list {
        let mut resource: *mut wl_resource = 0 as *mut wl_resource;
        resource = 0 as *mut wl_resource;
        resource = wl_resource_from_link((*client).resources.next);
        while wl_resource_get_link(resource) !=
                  &mut (*client).resources as *mut wl_list {
            wl_seat_send_name(resource, name);
            resource =
                wl_resource_from_link((*wl_resource_get_link(resource)).next)
        }
        client =
            ((*client).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_seat_client
    };
}
/* *
 * Get a seat client from a seat resource. Returns NULL if inert.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_client_from_resource(mut resource:
                                                           *mut wl_resource)
 -> *mut wlr_seat_client {
    if wl_resource_instance_of(resource, &wl_seat_interface,
                               &seat_impl as *const wl_seat_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_seat_interface, &seat_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/seat/wlr_seat.c\x00" as *const u8 as
                          *const libc::c_char, 363i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 76],
                                                &[libc::c_char; 76]>(b"struct wlr_seat_client *wlr_seat_client_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_seat_client;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_validate_grab_serial(mut seat:
                                                           *mut wlr_seat,
                                                       mut serial: uint32_t)
 -> bool {
    // TODO
	//return serial == seat->pointer_state.grab_serial ||
	//	serial == seat->touch_state.grab_serial;
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_client_next_serial(mut client:
                                                         *mut wlr_seat_client)
 -> uint32_t {
    let mut serial: uint32_t =
        wl_display_next_serial(wl_client_get_display((*client).client));
    let mut set: *mut wlr_serial_ringset = &mut (*client).serials;
    if (*set).count == 0i32 {
        (*set).data[0].min_incl = serial;
        (*set).data[0].max_incl = serial;
        (*set).count = 1i32;
        (*set).end = 0i32
    } else if (*set).data[(*set).end as
                              usize].max_incl.wrapping_add(1i32 as
                                                               libc::c_uint)
                  != serial {
        if (*set).count < 128i32 { (*set).count += 1 }
        (*set).end = ((*set).end + 1i32) % 128i32;
        (*set).data[(*set).end as usize].min_incl = serial;
        (*set).data[(*set).end as usize].max_incl = serial
    } else { (*set).data[(*set).end as usize].max_incl = serial }
    return serial;
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
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_client_validate_event_serial(mut client:
                                                                   *mut wlr_seat_client,
                                                               mut serial:
                                                                   uint32_t)
 -> bool {
    let mut cur: uint32_t =
        wl_display_get_serial(wl_client_get_display((*client).client));
    let mut set: *mut wlr_serial_ringset = &mut (*client).serials;
    let mut rev_dist: uint32_t = cur.wrapping_sub(serial);
    if rev_dist >= 4294967295u32.wrapping_div(2i32 as libc::c_uint) {
        // serial is closer to being 'newer' instead of 'older' than
		// the current serial, so it's either invalid or incredibly old
        return 0i32 != 0
    }
    let mut i: libc::c_int = 0i32;
    while i < (*set).count {
        let mut j: libc::c_int = ((*set).end - i + 128i32) % 128i32;
        if rev_dist < cur.wrapping_sub((*set).data[j as usize].max_incl) {
            return 0i32 != 0
        }
        if rev_dist <= cur.wrapping_sub((*set).data[j as usize].min_incl) {
            return 1i32 != 0
        }
        i += 1
    }
    // Iff the set is full, then `rev_dist` is large enough that serial
	// could already have been recycled out of the set.
    return (*set).count == 128i32;
}
