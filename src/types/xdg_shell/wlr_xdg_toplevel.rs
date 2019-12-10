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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn reset_xdg_surface(xdg_surface: *mut wlr_xdg_surface);
    #[no_mangle]
    fn handle_xdg_surface_commit(wlr_surface: *mut wlr_surface);
    #[no_mangle]
    fn handle_xdg_surface_precommit(wlr_surface: *mut wlr_surface);
    #[no_mangle]
    fn schedule_xdg_surface_configure(surface: *mut wlr_xdg_surface)
     -> uint32_t;
    #[no_mangle]
    static xdg_toplevel_interface: wl_interface;
    #[no_mangle]
    fn wlr_seat_client_from_resource(resource: *mut wl_resource)
     -> *mut wlr_seat_client;
    #[no_mangle]
    fn wlr_seat_validate_grab_serial(seat: *mut wlr_seat, serial: uint32_t)
     -> bool;
    #[no_mangle]
    fn wlr_surface_set_role(surface: *mut wlr_surface,
                            role: *const wlr_surface_role,
                            role_data: *mut libc::c_void,
                            error_resource: *mut wl_resource,
                            error_code: uint32_t) -> bool;
    #[no_mangle]
    fn wlr_output_from_resource(resource: *mut wl_resource)
     -> *mut wlr_output;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_array_init(array: *mut wl_array);
    #[no_mangle]
    fn wl_array_release(array: *mut wl_array);
    #[no_mangle]
    fn wl_array_add(array: *mut wl_array, size: size_t) -> *mut libc::c_void;
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
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
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
pub type wlr_edges = libc::c_uint;
pub const WLR_EDGE_RIGHT: wlr_edges = 8;
pub const WLR_EDGE_LEFT: wlr_edges = 4;
pub const WLR_EDGE_BOTTOM: wlr_edges = 2;
pub const WLR_EDGE_TOP: wlr_edges = 1;
pub const WLR_EDGE_NONE: wlr_edges = 0;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
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
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
    pub texture: *mut wlr_texture,
    pub surface: *mut wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
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
pub struct wlr_output {
    pub impl_0: *const wlr_output_impl,
    pub backend: *mut wlr_backend,
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
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub damage: pixman_region32_t,
    pub buffer_type: wlr_output_state_buffer_type,
    pub buffer: *mut wlr_buffer,
}
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;
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
pub type xdg_wm_base_error = libc::c_uint;
pub const XDG_WM_BASE_ERROR_INVALID_POSITIONER: xdg_wm_base_error = 5;
pub const XDG_WM_BASE_ERROR_INVALID_SURFACE_STATE: xdg_wm_base_error = 4;
pub const XDG_WM_BASE_ERROR_INVALID_POPUP_PARENT: xdg_wm_base_error = 3;
pub const XDG_WM_BASE_ERROR_NOT_THE_TOPMOST_POPUP: xdg_wm_base_error = 2;
pub const XDG_WM_BASE_ERROR_DEFUNCT_SURFACES: xdg_wm_base_error = 1;
pub const XDG_WM_BASE_ERROR_ROLE: xdg_wm_base_error = 0;
pub type xdg_positioner_anchor = libc::c_uint;
pub const XDG_POSITIONER_ANCHOR_BOTTOM_RIGHT: xdg_positioner_anchor = 8;
pub const XDG_POSITIONER_ANCHOR_TOP_RIGHT: xdg_positioner_anchor = 7;
pub const XDG_POSITIONER_ANCHOR_BOTTOM_LEFT: xdg_positioner_anchor = 6;
pub const XDG_POSITIONER_ANCHOR_TOP_LEFT: xdg_positioner_anchor = 5;
pub const XDG_POSITIONER_ANCHOR_RIGHT: xdg_positioner_anchor = 4;
pub const XDG_POSITIONER_ANCHOR_LEFT: xdg_positioner_anchor = 3;
pub const XDG_POSITIONER_ANCHOR_BOTTOM: xdg_positioner_anchor = 2;
pub const XDG_POSITIONER_ANCHOR_TOP: xdg_positioner_anchor = 1;
pub const XDG_POSITIONER_ANCHOR_NONE: xdg_positioner_anchor = 0;
pub type xdg_positioner_gravity = libc::c_uint;
pub const XDG_POSITIONER_GRAVITY_BOTTOM_RIGHT: xdg_positioner_gravity = 8;
pub const XDG_POSITIONER_GRAVITY_TOP_RIGHT: xdg_positioner_gravity = 7;
pub const XDG_POSITIONER_GRAVITY_BOTTOM_LEFT: xdg_positioner_gravity = 6;
pub const XDG_POSITIONER_GRAVITY_TOP_LEFT: xdg_positioner_gravity = 5;
pub const XDG_POSITIONER_GRAVITY_RIGHT: xdg_positioner_gravity = 4;
pub const XDG_POSITIONER_GRAVITY_LEFT: xdg_positioner_gravity = 3;
pub const XDG_POSITIONER_GRAVITY_BOTTOM: xdg_positioner_gravity = 2;
pub const XDG_POSITIONER_GRAVITY_TOP: xdg_positioner_gravity = 1;
pub const XDG_POSITIONER_GRAVITY_NONE: xdg_positioner_gravity = 0;
pub type xdg_positioner_constraint_adjustment = libc::c_uint;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_RESIZE_Y:
          xdg_positioner_constraint_adjustment =
    32;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_RESIZE_X:
          xdg_positioner_constraint_adjustment =
    16;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_FLIP_Y:
          xdg_positioner_constraint_adjustment =
    8;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_FLIP_X:
          xdg_positioner_constraint_adjustment =
    4;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_SLIDE_Y:
          xdg_positioner_constraint_adjustment =
    2;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_SLIDE_X:
          xdg_positioner_constraint_adjustment =
    1;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_NONE:
          xdg_positioner_constraint_adjustment =
    0;
pub type xdg_surface_error = libc::c_uint;
pub const XDG_SURFACE_ERROR_UNCONFIGURED_BUFFER: xdg_surface_error = 3;
pub const XDG_SURFACE_ERROR_ALREADY_CONSTRUCTED: xdg_surface_error = 2;
pub const XDG_SURFACE_ERROR_NOT_CONSTRUCTED: xdg_surface_error = 1;
pub type xdg_toplevel_state = libc::c_uint;
pub const XDG_TOPLEVEL_STATE_TILED_BOTTOM: xdg_toplevel_state = 8;
pub const XDG_TOPLEVEL_STATE_TILED_TOP: xdg_toplevel_state = 7;
pub const XDG_TOPLEVEL_STATE_TILED_RIGHT: xdg_toplevel_state = 6;
pub const XDG_TOPLEVEL_STATE_TILED_LEFT: xdg_toplevel_state = 5;
pub const XDG_TOPLEVEL_STATE_ACTIVATED: xdg_toplevel_state = 4;
pub const XDG_TOPLEVEL_STATE_RESIZING: xdg_toplevel_state = 3;
pub const XDG_TOPLEVEL_STATE_FULLSCREEN: xdg_toplevel_state = 2;
pub const XDG_TOPLEVEL_STATE_MAXIMIZED: xdg_toplevel_state = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xdg_toplevel_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub set_parent: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                _: *mut wl_resource,
                                                _: *mut wl_resource) -> ()>,
    pub set_title: Option<unsafe extern "C" fn(_: *mut wl_client,
                                               _: *mut wl_resource,
                                               _: *const libc::c_char) -> ()>,
    pub set_app_id: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                _: *mut wl_resource,
                                                _: *const libc::c_char)
                               -> ()>,
    pub show_window_menu: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                      _: *mut wl_resource,
                                                      _: *mut wl_resource,
                                                      _: uint32_t, _: int32_t,
                                                      _: int32_t) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(_: *mut wl_client,
                                            _: *mut wl_resource,
                                            _: *mut wl_resource, _: uint32_t)
                           -> ()>,
    pub resize: Option<unsafe extern "C" fn(_: *mut wl_client,
                                            _: *mut wl_resource,
                                            _: *mut wl_resource, _: uint32_t,
                                            _: uint32_t) -> ()>,
    pub set_max_size: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                  _: *mut wl_resource,
                                                  _: int32_t, _: int32_t)
                                 -> ()>,
    pub set_min_size: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                  _: *mut wl_resource,
                                                  _: int32_t, _: int32_t)
                                 -> ()>,
    pub set_maximized: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                   _: *mut wl_resource)
                                  -> ()>,
    pub unset_maximized: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                     _: *mut wl_resource)
                                    -> ()>,
    pub set_fullscreen: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                    _: *mut wl_resource,
                                                    _: *mut wl_resource)
                                   -> ()>,
    pub unset_fullscreen: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                      _: *mut wl_resource)
                                     -> ()>,
    pub set_minimized: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                   _: *mut wl_resource)
                                  -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_shell {
    pub global: *mut wl_global,
    pub clients: wl_list,
    pub popup_grabs: wl_list,
    pub ping_timeout: uint32_t,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_9,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_client {
    pub shell: *mut wlr_xdg_shell,
    pub resource: *mut wl_resource,
    pub client: *mut wl_client,
    pub surfaces: wl_list,
    pub link: wl_list,
    pub ping_serial: uint32_t,
    pub ping_timer: *mut wl_event_source,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_positioner {
    pub resource: *mut wl_resource,
    pub anchor_rect: wlr_box,
    pub anchor: xdg_positioner_anchor,
    pub gravity: xdg_positioner_gravity,
    pub constraint_adjustment: xdg_positioner_constraint_adjustment,
    pub size: C2RustUnnamed_11,
    pub offset: C2RustUnnamed_10,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub x: int32_t,
    pub y: int32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub width: int32_t,
    pub height: int32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_popup {
    pub base: *mut wlr_xdg_surface,
    pub link: wl_list,
    pub resource: *mut wl_resource,
    pub committed: bool,
    pub parent: *mut wlr_surface,
    pub seat: *mut wlr_seat,
    pub geometry: wlr_box,
    pub positioner: wlr_xdg_positioner,
    pub grab_link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_surface {
    pub client: *mut wlr_xdg_client,
    pub resource: *mut wl_resource,
    pub surface: *mut wlr_surface,
    pub link: wl_list,
    pub role: wlr_xdg_surface_role,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub destroy: wl_signal,
    pub ping_timeout: wl_signal,
    pub new_popup: wl_signal,
    pub map: wl_signal,
    pub unmap: wl_signal,
    pub configure: wl_signal,
    pub ack_configure: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_13 {
    pub toplevel: *mut wlr_xdg_toplevel,
    pub popup: *mut wlr_xdg_popup,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_toplevel {
    pub resource: *mut wl_resource,
    pub base: *mut wlr_xdg_surface,
    pub added: bool,
    pub parent: *mut wlr_xdg_surface,
    pub parent_unmap: wl_listener,
    pub client_pending: wlr_xdg_toplevel_state,
    pub server_pending: wlr_xdg_toplevel_state,
    pub current: wlr_xdg_toplevel_state,
    pub title: *mut libc::c_char,
    pub app_id: *mut libc::c_char,
    pub events: C2RustUnnamed_14,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_toplevel_state {
    pub maximized: bool,
    pub fullscreen: bool,
    pub resizing: bool,
    pub activated: bool,
    pub tiled: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
    pub max_width: uint32_t,
    pub max_height: uint32_t,
    pub min_width: uint32_t,
    pub min_height: uint32_t,
    pub fullscreen_output: *mut wlr_output,
    pub fullscreen_output_destroy: wl_listener,
}
pub type wlr_xdg_surface_role = libc::c_uint;
pub const WLR_XDG_SURFACE_ROLE_POPUP: wlr_xdg_surface_role = 2;
pub const WLR_XDG_SURFACE_ROLE_TOPLEVEL: wlr_xdg_surface_role = 1;
pub const WLR_XDG_SURFACE_ROLE_NONE: wlr_xdg_surface_role = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_surface_configure {
    pub surface: *mut wlr_xdg_surface,
    pub link: wl_list,
    pub serial: uint32_t,
    pub toplevel_state: *mut wlr_xdg_toplevel_state,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_toplevel_move_event {
    pub surface: *mut wlr_xdg_surface,
    pub seat: *mut wlr_seat_client,
    pub serial: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_toplevel_resize_event {
    pub surface: *mut wlr_xdg_surface,
    pub seat: *mut wlr_seat_client,
    pub serial: uint32_t,
    pub edges: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_toplevel_set_fullscreen_event {
    pub surface: *mut wlr_xdg_surface,
    pub fullscreen: bool,
    pub output: *mut wlr_output,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_toplevel_show_window_menu_event {
    pub surface: *mut wlr_xdg_surface,
    pub seat: *mut wlr_seat_client,
    pub serial: uint32_t,
    pub x: uint32_t,
    pub y: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub edge: wlr_edges,
    pub state: xdg_toplevel_state,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub state: wlr_xdg_toplevel_state,
    pub width: uint32_t,
    pub height: uint32_t,
}
#[inline]
unsafe extern "C" fn xdg_toplevel_send_configure(mut resource_:
                                                     *mut wl_resource,
                                                 mut width: int32_t,
                                                 mut height: int32_t,
                                                 mut states: *mut wl_array) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, width, height,
                           states);
}
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
pub unsafe extern "C" fn handle_xdg_toplevel_ack_configure(mut surface:
                                                               *mut wlr_xdg_surface,
                                                           mut configure:
                                                               *mut wlr_xdg_surface_configure) {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      13i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 101],
                                                &[libc::c_char; 101]>(b"void handle_xdg_toplevel_ack_configure(struct wlr_xdg_surface *, struct wlr_xdg_surface_configure *)\x00")).as_ptr());
    };
    if !(*configure).toplevel_state.is_null() {
    } else {
        __assert_fail(b"configure->toplevel_state != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      14i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 101],
                                                &[libc::c_char; 101]>(b"void handle_xdg_toplevel_ack_configure(struct wlr_xdg_surface *, struct wlr_xdg_surface_configure *)\x00")).as_ptr());
    };
    (*(*surface).c2rust_unnamed.toplevel).current.maximized =
        (*(*configure).toplevel_state).maximized;
    (*(*surface).c2rust_unnamed.toplevel).current.fullscreen =
        (*(*configure).toplevel_state).fullscreen;
    (*(*surface).c2rust_unnamed.toplevel).current.resizing =
        (*(*configure).toplevel_state).resizing;
    (*(*surface).c2rust_unnamed.toplevel).current.activated =
        (*(*configure).toplevel_state).activated;
    (*(*surface).c2rust_unnamed.toplevel).current.tiled =
        (*(*configure).toplevel_state).tiled;
}
#[no_mangle]
pub unsafe extern "C" fn compare_xdg_surface_toplevel_state(mut state:
                                                                *mut wlr_xdg_toplevel)
 -> bool {
    let mut configured: C2RustUnnamed_16 =
        C2RustUnnamed_16{state:
                             wlr_xdg_toplevel_state{maximized: false,
                                                    fullscreen: false,
                                                    resizing: false,
                                                    activated: false,
                                                    tiled: 0,
                                                    width: 0,
                                                    height: 0,
                                                    max_width: 0,
                                                    max_height: 0,
                                                    min_width: 0,
                                                    min_height: 0,
                                                    fullscreen_output:
                                                        0 as *mut wlr_output,
                                                    fullscreen_output_destroy:
                                                        wl_listener{link:
                                                                        wl_list{prev:
                                                                                    0
                                                                                        as
                                                                                        *mut wl_list,
                                                                                next:
                                                                                    0
                                                                                        as
                                                                                        *mut wl_list,},
                                                                    notify:
                                                                        None,},},
                         width: 0,
                         height: 0,};
    // is pending state different from current state?
    if !(*(*state).base).configured { return 0i32 != 0 }
    if wl_list_empty(&mut (*(*state).base).configure_list) != 0 {
        // last configure is actually the current state, just use it
        configured.state = (*state).current;
        configured.width =
            (*(*(*state).base).surface).current.width as uint32_t;
        configured.height =
            (*(*(*state).base).surface).current.height as uint32_t
    } else {
        let mut configure: *mut wlr_xdg_surface_configure =
            ((*(*state).base).configure_list.prev as
                 *mut libc::c_char).offset(-8) as
                *mut wlr_xdg_surface_configure;
        configured.state = *(*configure).toplevel_state;
        configured.width = (*(*configure).toplevel_state).width;
        configured.height = (*(*configure).toplevel_state).height
    }
    if (*state).server_pending.activated as libc::c_int !=
           configured.state.activated as libc::c_int {
        return 0i32 != 0
    }
    if (*state).server_pending.fullscreen as libc::c_int !=
           configured.state.fullscreen as libc::c_int {
        return 0i32 != 0
    }
    if (*state).server_pending.maximized as libc::c_int !=
           configured.state.maximized as libc::c_int {
        return 0i32 != 0
    }
    if (*state).server_pending.resizing as libc::c_int !=
           configured.state.resizing as libc::c_int {
        return 0i32 != 0
    }
    if (*state).server_pending.tiled != configured.state.tiled {
        return 0i32 != 0
    }
    if (*state).server_pending.width == configured.width &&
           (*state).server_pending.height == configured.height {
        return 1i32 != 0
    }
    if (*state).server_pending.width == 0i32 as libc::c_uint &&
           (*state).server_pending.height == 0i32 as libc::c_uint {
        return 1i32 != 0
    }
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn send_xdg_toplevel_configure(mut surface:
                                                         *mut wlr_xdg_surface,
                                                     mut configure:
                                                         *mut wlr_xdg_surface_configure) {
    let mut width: uint32_t = 0;
    let mut height: uint32_t = 0;
    let mut current_block: u64;
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      82i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 95],
                                                &[libc::c_char; 95]>(b"void send_xdg_toplevel_configure(struct wlr_xdg_surface *, struct wlr_xdg_surface_configure *)\x00")).as_ptr());
    };
    (*configure).toplevel_state =
        malloc(::std::mem::size_of::<wlr_xdg_toplevel_state>() as
                   libc::c_ulong) as *mut wlr_xdg_toplevel_state;
    if (*configure).toplevel_state.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as *const u8 as
                     *const libc::c_char, 86i32);
        wl_resource_post_no_memory((*(*surface).c2rust_unnamed.toplevel).resource);
        return
    }
    *(*configure).toplevel_state =
        (*(*surface).c2rust_unnamed.toplevel).server_pending;
    let mut states: wl_array =
        wl_array{size: 0, alloc: 0, data: 0 as *mut libc::c_void,};
    wl_array_init(&mut states);
    if (*(*surface).c2rust_unnamed.toplevel).server_pending.maximized {
        let mut s: *mut uint32_t =
            wl_array_add(&mut states,
                         ::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                as *mut uint32_t;
        if s.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Could not allocate state for maximized xdg_toplevel\x00"
                         as *const u8 as *const libc::c_char,
                     b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as *const u8
                         as *const libc::c_char, 97i32);
            current_block = 7464701719611813143;
        } else {
            *s = XDG_TOPLEVEL_STATE_MAXIMIZED as libc::c_int as uint32_t;
            current_block = 13586036798005543211;
        }
    } else { current_block = 13586036798005543211; }
    match current_block {
        13586036798005543211 => {
            if (*(*surface).c2rust_unnamed.toplevel).server_pending.fullscreen
               {
                let mut s_0: *mut uint32_t =
                    wl_array_add(&mut states,
                                 ::std::mem::size_of::<uint32_t>() as
                                     libc::c_ulong) as *mut uint32_t;
                if s_0.is_null() {
                    _wlr_log(WLR_ERROR,
                             b"[%s:%d] Could not allocate state for fullscreen xdg_toplevel\x00"
                                 as *const u8 as *const libc::c_char,
                             b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                                 *const u8 as *const libc::c_char, 105i32);
                    current_block = 7464701719611813143;
                } else {
                    *s_0 =
                        XDG_TOPLEVEL_STATE_FULLSCREEN as libc::c_int as
                            uint32_t;
                    current_block = 6057473163062296781;
                }
            } else { current_block = 6057473163062296781; }
            match current_block {
                7464701719611813143 => { }
                _ => {
                    if (*(*surface).c2rust_unnamed.toplevel).server_pending.resizing
                       {
                        let mut s_1: *mut uint32_t =
                            wl_array_add(&mut states,
                                         ::std::mem::size_of::<uint32_t>() as
                                             libc::c_ulong) as *mut uint32_t;
                        if s_1.is_null() {
                            _wlr_log(WLR_ERROR,
                                     b"[%s:%d] Could not allocate state for resizing xdg_toplevel\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"../types/xdg_shell/wlr_xdg_toplevel.c\x00"
                                         as *const u8 as *const libc::c_char,
                                     113i32);
                            current_block = 7464701719611813143;
                        } else {
                            *s_1 =
                                XDG_TOPLEVEL_STATE_RESIZING as libc::c_int as
                                    uint32_t;
                            current_block = 13472856163611868459;
                        }
                    } else { current_block = 13472856163611868459; }
                    match current_block {
                        7464701719611813143 => { }
                        _ => {
                            if (*(*surface).c2rust_unnamed.toplevel).server_pending.activated
                               {
                                let mut s_2: *mut uint32_t =
                                    wl_array_add(&mut states,
                                                 ::std::mem::size_of::<uint32_t>()
                                                     as libc::c_ulong) as
                                        *mut uint32_t;
                                if s_2.is_null() {
                                    _wlr_log(WLR_ERROR,
                                             b"[%s:%d] Could not allocate state for activated xdg_toplevel\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"../types/xdg_shell/wlr_xdg_toplevel.c\x00"
                                                 as *const u8 as
                                                 *const libc::c_char, 121i32);
                                    current_block = 7464701719611813143;
                                } else {
                                    *s_2 =
                                        XDG_TOPLEVEL_STATE_ACTIVATED as
                                            libc::c_int as uint32_t;
                                    current_block = 7205609094909031804;
                                }
                            } else { current_block = 7205609094909031804; }
                            match current_block {
                                7464701719611813143 => { }
                                _ => {
                                    if (*(*surface).c2rust_unnamed.toplevel).server_pending.tiled
                                           != 0 {
                                        if wl_resource_get_version((*surface).resource)
                                               >= 2i32 {
                                            let tiled: [C2RustUnnamed_15; 4] =
                                                [{
                                                     let mut init =
                                                         C2RustUnnamed_15{edge:
                                                                              WLR_EDGE_LEFT,
                                                                          state:
                                                                              XDG_TOPLEVEL_STATE_TILED_LEFT,};
                                                     init
                                                 },
                                                 {
                                                     let mut init =
                                                         C2RustUnnamed_15{edge:
                                                                              WLR_EDGE_RIGHT,
                                                                          state:
                                                                              XDG_TOPLEVEL_STATE_TILED_RIGHT,};
                                                     init
                                                 },
                                                 {
                                                     let mut init =
                                                         C2RustUnnamed_15{edge:
                                                                              WLR_EDGE_TOP,
                                                                          state:
                                                                              XDG_TOPLEVEL_STATE_TILED_TOP,};
                                                     init
                                                 },
                                                 {
                                                     let mut init =
                                                         C2RustUnnamed_15{edge:
                                                                              WLR_EDGE_BOTTOM,
                                                                          state:
                                                                              XDG_TOPLEVEL_STATE_TILED_BOTTOM,};
                                                     init
                                                 }];
                                            let mut i: size_t =
                                                0i32 as size_t;
                                            loop  {
                                                if !(i <
                                                         (::std::mem::size_of::<[C2RustUnnamed_15; 4]>()
                                                              as
                                                              libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_15>()
                                                                                              as
                                                                                              libc::c_ulong))
                                                   {
                                                    current_block =
                                                        313581471991351815;
                                                    break ;
                                                }
                                                if !((*(*surface).c2rust_unnamed.toplevel).server_pending.tiled
                                                         &
                                                         tiled[i as
                                                                   usize].edge
                                                             as libc::c_uint
                                                         ==
                                                         0i32 as libc::c_uint)
                                                   {
                                                    let mut s_3:
                                                            *mut uint32_t =
                                                        wl_array_add(&mut states,
                                                                     ::std::mem::size_of::<uint32_t>()
                                                                         as
                                                                         libc::c_ulong)
                                                            as *mut uint32_t;
                                                    if s_3.is_null() {
                                                        _wlr_log(WLR_ERROR,
                                                                 b"[%s:%d] Could not allocate state for tiled xdg_toplevel\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../types/xdg_shell/wlr_xdg_toplevel.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 148i32);
                                                        current_block =
                                                            7464701719611813143;
                                                        break ;
                                                    } else {
                                                        *s_3 =
                                                            tiled[i as
                                                                      usize].state
                                                                as uint32_t
                                                    }
                                                }
                                                i = i.wrapping_add(1)
                                            }
                                        } else if !(*(*surface).c2rust_unnamed.toplevel).server_pending.maximized
                                         {
                                            // This version doesn't support tiling, best we can do is make the
			// toplevel maximized
                                            let mut s_4: *mut uint32_t =
                                                wl_array_add(&mut states,
                                                             ::std::mem::size_of::<uint32_t>()
                                                                 as
                                                                 libc::c_ulong)
                                                    as *mut uint32_t;
                                            if s_4.is_null() {
                                                _wlr_log(WLR_ERROR,
                                                         b"[%s:%d] Could not allocate state for maximized xdg_toplevel\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         b"../types/xdg_shell/wlr_xdg_toplevel.c\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         159i32);
                                                current_block =
                                                    7464701719611813143;
                                            } else {
                                                *s_4 =
                                                    XDG_TOPLEVEL_STATE_MAXIMIZED
                                                        as libc::c_int as
                                                        uint32_t;
                                                current_block =
                                                    313581471991351815;
                                            }
                                        } else {
                                            current_block =
                                                313581471991351815;
                                        }
                                    } else {
                                        current_block = 313581471991351815;
                                    }
                                    match current_block {
                                        7464701719611813143 => { }
                                        _ => {
                                            width =
                                                (*(*surface).c2rust_unnamed.toplevel).server_pending.width;
                                            height =
                                                (*(*surface).c2rust_unnamed.toplevel).server_pending.height;
                                            xdg_toplevel_send_configure((*(*surface).c2rust_unnamed.toplevel).resource,
                                                                        width
                                                                            as
                                                                            int32_t,
                                                                        height
                                                                            as
                                                                            int32_t,
                                                                        &mut states);
                                            wl_array_release(&mut states);
                                            return
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    wl_array_release(&mut states);
    wl_resource_post_no_memory((*(*surface).c2rust_unnamed.toplevel).resource);
}
#[no_mangle]
pub unsafe extern "C" fn handle_xdg_surface_toplevel_committed(mut surface:
                                                                   *mut wlr_xdg_surface) {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      180i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 69],
                                                &[libc::c_char; 69]>(b"void handle_xdg_surface_toplevel_committed(struct wlr_xdg_surface *)\x00")).as_ptr());
    };
    if !(*(*surface).c2rust_unnamed.toplevel).added {
        // on the first commit, send a configure request to tell the client it
		// is added
        schedule_xdg_surface_configure(surface);
        (*(*surface).c2rust_unnamed.toplevel).added = 1i32 != 0;
        return
    }
    // update state that doesn't need compositor approval
    (*(*surface).c2rust_unnamed.toplevel).current.max_width =
        (*(*surface).c2rust_unnamed.toplevel).client_pending.max_width;
    (*(*surface).c2rust_unnamed.toplevel).current.min_width =
        (*(*surface).c2rust_unnamed.toplevel).client_pending.min_width;
    (*(*surface).c2rust_unnamed.toplevel).current.max_height =
        (*(*surface).c2rust_unnamed.toplevel).client_pending.max_height;
    (*(*surface).c2rust_unnamed.toplevel).current.min_height =
        (*(*surface).c2rust_unnamed.toplevel).client_pending.min_height;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_surface_from_toplevel_resource(mut resource:
                                                                    *mut wl_resource)
 -> *mut wlr_xdg_surface {
    if wl_resource_instance_of(resource, &xdg_toplevel_interface,
                               &xdg_toplevel_implementation as
                                   *const xdg_toplevel_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &xdg_toplevel_interface, &xdg_toplevel_implementation)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      206i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[libc::c_char; 85]>(b"struct wlr_xdg_surface *wlr_xdg_surface_from_toplevel_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_xdg_surface;
}
unsafe extern "C" fn handle_parent_unmap(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut toplevel: *mut wlr_xdg_toplevel =
        (listener as *mut libc::c_char).offset(-32) as *mut wlr_xdg_toplevel;
    set_parent((*toplevel).base,
               (*(*(*toplevel).parent).c2rust_unnamed.toplevel).parent);
}
unsafe extern "C" fn set_parent(mut surface: *mut wlr_xdg_surface,
                                mut parent: *mut wlr_xdg_surface) {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      221i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"void set_parent(struct wlr_xdg_surface *, struct wlr_xdg_surface *)\x00")).as_ptr());
    };
    if parent.is_null() ||
           (*parent).role as libc::c_uint ==
               WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"!parent || parent->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      222i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"void set_parent(struct wlr_xdg_surface *, struct wlr_xdg_surface *)\x00")).as_ptr());
    };
    if !(*(*surface).c2rust_unnamed.toplevel).parent.is_null() {
        wl_list_remove(&mut (*(*surface).c2rust_unnamed.toplevel).parent_unmap.link);
    }
    (*(*surface).c2rust_unnamed.toplevel).parent = parent;
    if !(*(*surface).c2rust_unnamed.toplevel).parent.is_null() {
        (*(*surface).c2rust_unnamed.toplevel).parent_unmap.notify =
            Some(handle_parent_unmap as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*(*surface).c2rust_unnamed.toplevel).parent).events.unmap,
                      &mut (*(*surface).c2rust_unnamed.toplevel).parent_unmap);
    }
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.set_parent,
                         surface as *mut libc::c_void);
}
unsafe extern "C" fn xdg_toplevel_handle_set_parent(mut client:
                                                        *mut wl_client,
                                                    mut resource:
                                                        *mut wl_resource,
                                                    mut parent_resource:
                                                        *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    let mut parent: *mut wlr_xdg_surface = 0 as *mut wlr_xdg_surface;
    if !parent_resource.is_null() {
        parent = wlr_xdg_surface_from_toplevel_resource(parent_resource)
    }
    set_parent(surface, parent);
}
unsafe extern "C" fn xdg_toplevel_handle_set_title(mut client: *mut wl_client,
                                                   mut resource:
                                                       *mut wl_resource,
                                                   mut title:
                                                       *const libc::c_char) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = strdup(title);
    if tmp.is_null() { return }
    free((*(*surface).c2rust_unnamed.toplevel).title as *mut libc::c_void);
    (*(*surface).c2rust_unnamed.toplevel).title = tmp;
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.set_title,
                         surface as *mut libc::c_void);
}
unsafe extern "C" fn xdg_toplevel_handle_set_app_id(mut client:
                                                        *mut wl_client,
                                                    mut resource:
                                                        *mut wl_resource,
                                                    mut app_id:
                                                        *const libc::c_char) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = strdup(app_id);
    if tmp.is_null() { return }
    free((*(*surface).c2rust_unnamed.toplevel).app_id as *mut libc::c_void);
    (*(*surface).c2rust_unnamed.toplevel).app_id = tmp;
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.set_app_id,
                         surface as *mut libc::c_void);
}
unsafe extern "C" fn xdg_toplevel_handle_show_window_menu(mut client:
                                                              *mut wl_client,
                                                          mut resource:
                                                              *mut wl_resource,
                                                          mut seat_resource:
                                                              *mut wl_resource,
                                                          mut serial:
                                                              uint32_t,
                                                          mut x: int32_t,
                                                          mut y: int32_t) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    let mut seat: *mut wlr_seat_client =
        wlr_seat_client_from_resource(seat_resource);
    if !(*surface).configured {
        wl_resource_post_error((*(*surface).c2rust_unnamed.toplevel).resource,
                               XDG_SURFACE_ERROR_NOT_CONSTRUCTED as
                                   libc::c_int as uint32_t,
                               b"surface has not been configured yet\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    if !wlr_seat_validate_grab_serial((*seat).seat, serial) {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] invalid serial for grab\x00" as *const u8 as
                     *const libc::c_char,
                 b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as *const u8 as
                     *const libc::c_char, 299i32);
        return
    }
    let mut event: wlr_xdg_toplevel_show_window_menu_event =
        {
            let mut init =
                wlr_xdg_toplevel_show_window_menu_event{surface: surface,
                                                        seat: seat,
                                                        serial: serial,
                                                        x: x as uint32_t,
                                                        y: y as uint32_t,};
            init
        };
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.request_show_window_menu,
                         &mut event as
                             *mut wlr_xdg_toplevel_show_window_menu_event as
                             *mut libc::c_void);
}
unsafe extern "C" fn xdg_toplevel_handle_move(mut client: *mut wl_client,
                                              mut resource: *mut wl_resource,
                                              mut seat_resource:
                                                  *mut wl_resource,
                                              mut serial: uint32_t) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    let mut seat: *mut wlr_seat_client =
        wlr_seat_client_from_resource(seat_resource);
    if !(*surface).configured {
        wl_resource_post_error((*(*surface).c2rust_unnamed.toplevel).resource,
                               XDG_SURFACE_ERROR_NOT_CONSTRUCTED as
                                   libc::c_int as uint32_t,
                               b"surface has not been configured yet\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    if !wlr_seat_validate_grab_serial((*seat).seat, serial) {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] invalid serial for grab\x00" as *const u8 as
                     *const libc::c_char,
                 b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as *const u8 as
                     *const libc::c_char, 330i32);
        return
    }
    let mut event: wlr_xdg_toplevel_move_event =
        {
            let mut init =
                wlr_xdg_toplevel_move_event{surface: surface,
                                            seat: seat,
                                            serial: serial,};
            init
        };
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.request_move,
                         &mut event as *mut wlr_xdg_toplevel_move_event as
                             *mut libc::c_void);
}
unsafe extern "C" fn xdg_toplevel_handle_resize(mut client: *mut wl_client,
                                                mut resource:
                                                    *mut wl_resource,
                                                mut seat_resource:
                                                    *mut wl_resource,
                                                mut serial: uint32_t,
                                                mut edges: uint32_t) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    let mut seat: *mut wlr_seat_client =
        wlr_seat_client_from_resource(seat_resource);
    if !(*surface).configured {
        wl_resource_post_error((*(*surface).c2rust_unnamed.toplevel).resource,
                               XDG_SURFACE_ERROR_NOT_CONSTRUCTED as
                                   libc::c_int as uint32_t,
                               b"surface has not been configured yet\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    if !wlr_seat_validate_grab_serial((*seat).seat, serial) {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] invalid serial for grab\x00" as *const u8 as
                     *const libc::c_char,
                 b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as *const u8 as
                     *const libc::c_char, 359i32);
        return
    }
    let mut event: wlr_xdg_toplevel_resize_event =
        {
            let mut init =
                wlr_xdg_toplevel_resize_event{surface: surface,
                                              seat: seat,
                                              serial: serial,
                                              edges: edges,};
            init
        };
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.request_resize,
                         &mut event as *mut wlr_xdg_toplevel_resize_event as
                             *mut libc::c_void);
}
unsafe extern "C" fn xdg_toplevel_handle_set_max_size(mut client:
                                                          *mut wl_client,
                                                      mut resource:
                                                          *mut wl_resource,
                                                      mut width: int32_t,
                                                      mut height: int32_t) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    (*(*surface).c2rust_unnamed.toplevel).client_pending.max_width =
        width as uint32_t;
    (*(*surface).c2rust_unnamed.toplevel).client_pending.max_height =
        height as uint32_t;
}
unsafe extern "C" fn xdg_toplevel_handle_set_min_size(mut client:
                                                          *mut wl_client,
                                                      mut resource:
                                                          *mut wl_resource,
                                                      mut width: int32_t,
                                                      mut height: int32_t) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    (*(*surface).c2rust_unnamed.toplevel).client_pending.min_width =
        width as uint32_t;
    (*(*surface).c2rust_unnamed.toplevel).client_pending.min_height =
        height as uint32_t;
}
unsafe extern "C" fn xdg_toplevel_handle_set_maximized(mut client:
                                                           *mut wl_client,
                                                       mut resource:
                                                           *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    (*(*surface).c2rust_unnamed.toplevel).client_pending.maximized =
        1i32 != 0;
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.request_maximize,
                         surface as *mut libc::c_void);
}
unsafe extern "C" fn xdg_toplevel_handle_unset_maximized(mut client:
                                                             *mut wl_client,
                                                         mut resource:
                                                             *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    (*(*surface).c2rust_unnamed.toplevel).client_pending.maximized =
        0i32 != 0;
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.request_maximize,
                         surface as *mut libc::c_void);
}
unsafe extern "C" fn handle_fullscreen_output_destroy(mut listener:
                                                          *mut wl_listener,
                                                      mut data:
                                                          *mut libc::c_void) {
    let mut state: *mut wlr_xdg_toplevel_state =
        (listener as *mut libc::c_char).offset(-40) as
            *mut wlr_xdg_toplevel_state;
    (*state).fullscreen_output = 0 as *mut wlr_output;
    wl_list_remove(&mut (*state).fullscreen_output_destroy.link);
}
unsafe extern "C" fn store_fullscreen_pending(mut surface:
                                                  *mut wlr_xdg_surface,
                                              mut fullscreen: bool,
                                              mut output: *mut wlr_output) {
    let mut state: *mut wlr_xdg_toplevel_state =
        &mut (*(*surface).c2rust_unnamed.toplevel).client_pending;
    (*state).fullscreen = fullscreen;
    if !(*state).fullscreen_output.is_null() {
        wl_list_remove(&mut (*state).fullscreen_output_destroy.link);
    }
    (*state).fullscreen_output = output;
    if !(*state).fullscreen_output.is_null() {
        (*state).fullscreen_output_destroy.notify =
            Some(handle_fullscreen_output_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*state).fullscreen_output).events.destroy,
                      &mut (*state).fullscreen_output_destroy);
    };
}
unsafe extern "C" fn xdg_toplevel_handle_set_fullscreen(mut client:
                                                            *mut wl_client,
                                                        mut resource:
                                                            *mut wl_resource,
                                                        mut output_resource:
                                                            *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    let mut output: *mut wlr_output = 0 as *mut wlr_output;
    if !output_resource.is_null() {
        output = wlr_output_from_resource(output_resource)
    }
    store_fullscreen_pending(surface, 1i32 != 0, output);
    let mut event: wlr_xdg_toplevel_set_fullscreen_event =
        {
            let mut init =
                wlr_xdg_toplevel_set_fullscreen_event{surface: surface,
                                                      fullscreen: 1i32 != 0,
                                                      output: output,};
            init
        };
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.request_fullscreen,
                         &mut event as
                             *mut wlr_xdg_toplevel_set_fullscreen_event as
                             *mut libc::c_void);
}
unsafe extern "C" fn xdg_toplevel_handle_unset_fullscreen(mut client:
                                                              *mut wl_client,
                                                          mut resource:
                                                              *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    store_fullscreen_pending(surface, 0i32 != 0, 0 as *mut wlr_output);
    let mut event: wlr_xdg_toplevel_set_fullscreen_event =
        {
            let mut init =
                wlr_xdg_toplevel_set_fullscreen_event{surface: surface,
                                                      fullscreen: 0i32 != 0,
                                                      output:
                                                          0 as
                                                              *mut wlr_output,};
            init
        };
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.request_fullscreen,
                         &mut event as
                             *mut wlr_xdg_toplevel_set_fullscreen_event as
                             *mut libc::c_void);
}
unsafe extern "C" fn xdg_toplevel_handle_set_minimized(mut client:
                                                           *mut wl_client,
                                                       mut resource:
                                                           *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    wlr_signal_emit_safe(&mut (*(*surface).c2rust_unnamed.toplevel).events.request_minimize,
                         surface as *mut libc::c_void);
}
unsafe extern "C" fn xdg_toplevel_handle_destroy(mut client: *mut wl_client,
                                                 mut resource:
                                                     *mut wl_resource) {
    wl_resource_destroy(resource);
}
static mut xdg_toplevel_implementation: xdg_toplevel_interface =
    unsafe {
        {
            let mut init =
                xdg_toplevel_interface{destroy:
                                           Some(xdg_toplevel_handle_destroy as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource)
                                                        -> ()),
                                       set_parent:
                                           Some(xdg_toplevel_handle_set_parent
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             *mut wl_resource)
                                                        -> ()),
                                       set_title:
                                           Some(xdg_toplevel_handle_set_title
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             *const libc::c_char)
                                                        -> ()),
                                       set_app_id:
                                           Some(xdg_toplevel_handle_set_app_id
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             *const libc::c_char)
                                                        -> ()),
                                       show_window_menu:
                                           Some(xdg_toplevel_handle_show_window_menu
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             uint32_t,
                                                                         _:
                                                                             int32_t,
                                                                         _:
                                                                             int32_t)
                                                        -> ()),
                                       move_0:
                                           Some(xdg_toplevel_handle_move as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             uint32_t)
                                                        -> ()),
                                       resize:
                                           Some(xdg_toplevel_handle_resize as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             uint32_t,
                                                                         _:
                                                                             uint32_t)
                                                        -> ()),
                                       set_max_size:
                                           Some(xdg_toplevel_handle_set_max_size
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             int32_t,
                                                                         _:
                                                                             int32_t)
                                                        -> ()),
                                       set_min_size:
                                           Some(xdg_toplevel_handle_set_min_size
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             int32_t,
                                                                         _:
                                                                             int32_t)
                                                        -> ()),
                                       set_maximized:
                                           Some(xdg_toplevel_handle_set_maximized
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource)
                                                        -> ()),
                                       unset_maximized:
                                           Some(xdg_toplevel_handle_unset_maximized
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource)
                                                        -> ()),
                                       set_fullscreen:
                                           Some(xdg_toplevel_handle_set_fullscreen
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource,
                                                                         _:
                                                                             *mut wl_resource)
                                                        -> ()),
                                       unset_fullscreen:
                                           Some(xdg_toplevel_handle_unset_fullscreen
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource)
                                                        -> ()),
                                       set_minimized:
                                           Some(xdg_toplevel_handle_set_minimized
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut wl_client,
                                                                         _:
                                                                             *mut wl_resource)
                                                        -> ()),};
            init
        }
    };
unsafe extern "C" fn xdg_toplevel_handle_resource_destroy(mut resource:
                                                              *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface =
        wlr_xdg_surface_from_toplevel_resource(resource);
    destroy_xdg_toplevel(surface);
}
#[no_mangle]
pub static mut xdg_toplevel_surface_role: wlr_surface_role =
    unsafe {
        {
            let mut init =
                wlr_surface_role{name:
                                     b"xdg_toplevel\x00" as *const u8 as
                                         *const libc::c_char,
                                 commit:
                                     Some(handle_xdg_surface_commit as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_surface)
                                                  -> ()),
                                 precommit:
                                     Some(handle_xdg_surface_precommit as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_surface)
                                                  -> ()),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn create_xdg_toplevel(mut xdg_surface:
                                                 *mut wlr_xdg_surface,
                                             mut id: uint32_t) {
    if !wlr_surface_set_role((*xdg_surface).surface,
                             &xdg_toplevel_surface_role,
                             xdg_surface as *mut libc::c_void,
                             (*xdg_surface).resource,
                             XDG_WM_BASE_ERROR_ROLE as libc::c_int as
                                 uint32_t) {
        return
    }
    if (*xdg_surface).role as libc::c_uint !=
           WLR_XDG_SURFACE_ROLE_NONE as libc::c_int as libc::c_uint {
        wl_resource_post_error((*xdg_surface).resource,
                               XDG_SURFACE_ERROR_ALREADY_CONSTRUCTED as
                                   libc::c_int as uint32_t,
                               b"xdg-surface has already been constructed\x00"
                                   as *const u8 as *const libc::c_char);
        return
    }
    if (*xdg_surface).c2rust_unnamed.toplevel.is_null() {
    } else {
        __assert_fail(b"xdg_surface->toplevel == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      521i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 61],
                                                &[libc::c_char; 61]>(b"void create_xdg_toplevel(struct wlr_xdg_surface *, uint32_t)\x00")).as_ptr());
    };
    (*xdg_surface).c2rust_unnamed.toplevel =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xdg_toplevel>() as libc::c_ulong) as
            *mut wlr_xdg_toplevel;
    if (*xdg_surface).c2rust_unnamed.toplevel.is_null() {
        wl_resource_post_no_memory((*xdg_surface).resource);
        return
    }
    (*(*xdg_surface).c2rust_unnamed.toplevel).base = xdg_surface;
    wl_signal_init(&mut (*(*xdg_surface).c2rust_unnamed.toplevel).events.request_maximize);
    wl_signal_init(&mut (*(*xdg_surface).c2rust_unnamed.toplevel).events.request_fullscreen);
    wl_signal_init(&mut (*(*xdg_surface).c2rust_unnamed.toplevel).events.request_minimize);
    wl_signal_init(&mut (*(*xdg_surface).c2rust_unnamed.toplevel).events.request_move);
    wl_signal_init(&mut (*(*xdg_surface).c2rust_unnamed.toplevel).events.request_resize);
    wl_signal_init(&mut (*(*xdg_surface).c2rust_unnamed.toplevel).events.request_show_window_menu);
    wl_signal_init(&mut (*(*xdg_surface).c2rust_unnamed.toplevel).events.set_parent);
    wl_signal_init(&mut (*(*xdg_surface).c2rust_unnamed.toplevel).events.set_title);
    wl_signal_init(&mut (*(*xdg_surface).c2rust_unnamed.toplevel).events.set_app_id);
    (*(*xdg_surface).c2rust_unnamed.toplevel).resource =
        wl_resource_create((*(*xdg_surface).client).client,
                           &xdg_toplevel_interface,
                           wl_resource_get_version((*xdg_surface).resource),
                           id);
    if (*(*xdg_surface).c2rust_unnamed.toplevel).resource.is_null() {
        free((*xdg_surface).c2rust_unnamed.toplevel as *mut libc::c_void);
        wl_resource_post_no_memory((*xdg_surface).resource);
        return
    }
    wl_resource_set_implementation((*(*xdg_surface).c2rust_unnamed.toplevel).resource,
                                   &xdg_toplevel_implementation as
                                       *const xdg_toplevel_interface as
                                       *const libc::c_void,
                                   xdg_surface as *mut libc::c_void,
                                   Some(xdg_toplevel_handle_resource_destroy
                                            as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    (*xdg_surface).role = WLR_XDG_SURFACE_ROLE_TOPLEVEL;
}
#[no_mangle]
pub unsafe extern "C" fn destroy_xdg_toplevel(mut xdg_surface:
                                                  *mut wlr_xdg_surface) {
    if xdg_surface.is_null() { return }
    if (*xdg_surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"xdg_surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      558i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 52],
                                                &[libc::c_char; 52]>(b"void destroy_xdg_toplevel(struct wlr_xdg_surface *)\x00")).as_ptr());
    };
    reset_xdg_surface(xdg_surface);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_toplevel_set_size(mut surface:
                                                       *mut wlr_xdg_surface,
                                                   mut width: uint32_t,
                                                   mut height: uint32_t)
 -> uint32_t {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      564i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 81],
                                                &[libc::c_char; 81]>(b"uint32_t wlr_xdg_toplevel_set_size(struct wlr_xdg_surface *, uint32_t, uint32_t)\x00")).as_ptr());
    };
    (*(*surface).c2rust_unnamed.toplevel).server_pending.width = width;
    (*(*surface).c2rust_unnamed.toplevel).server_pending.height = height;
    return schedule_xdg_surface_configure(surface);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_toplevel_set_activated(mut surface:
                                                            *mut wlr_xdg_surface,
                                                        mut activated: bool)
 -> uint32_t {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      573i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"uint32_t wlr_xdg_toplevel_set_activated(struct wlr_xdg_surface *, _Bool)\x00")).as_ptr());
    };
    (*(*surface).c2rust_unnamed.toplevel).server_pending.activated =
        activated;
    return schedule_xdg_surface_configure(surface);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_toplevel_set_maximized(mut surface:
                                                            *mut wlr_xdg_surface,
                                                        mut maximized: bool)
 -> uint32_t {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      581i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"uint32_t wlr_xdg_toplevel_set_maximized(struct wlr_xdg_surface *, _Bool)\x00")).as_ptr());
    };
    (*(*surface).c2rust_unnamed.toplevel).server_pending.maximized =
        maximized;
    return schedule_xdg_surface_configure(surface);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_toplevel_set_fullscreen(mut surface:
                                                             *mut wlr_xdg_surface,
                                                         mut fullscreen: bool)
 -> uint32_t {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      589i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"uint32_t wlr_xdg_toplevel_set_fullscreen(struct wlr_xdg_surface *, _Bool)\x00")).as_ptr());
    };
    (*(*surface).c2rust_unnamed.toplevel).server_pending.fullscreen =
        fullscreen;
    return schedule_xdg_surface_configure(surface);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_toplevel_set_resizing(mut surface:
                                                           *mut wlr_xdg_surface,
                                                       mut resizing: bool)
 -> uint32_t {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      597i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"uint32_t wlr_xdg_toplevel_set_resizing(struct wlr_xdg_surface *, _Bool)\x00")).as_ptr());
    };
    (*(*surface).c2rust_unnamed.toplevel).server_pending.resizing = resizing;
    return schedule_xdg_surface_configure(surface);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
		 * The `new_surface` event signals that a client has requested to
		 * create a new shell surface. At this point, the surface is ready to
		 * be configured but is not mapped or ready receive input events. The
		 * surface will be ready to be managed on the `map` event.
		 */
// wlr_xdg_shell::clients
// Position of the popup relative to the upper left corner of the window
	// geometry of the parent surface
// wlr_xdg_popup_grab::popups
// each seat gets a popup grab
// wlr_xdg_shell::popup_grabs
// enum wlr_edges
// Since the fullscreen request may be made before the toplevel's surface
	// is mapped, this is used to store the requested fullscreen output (if
	// any) for wlr_xdg_toplevel::client_pending.
// wlr_xdg_surface::configure_list
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
// wlr_xdg_client::surfaces
// wlr_xdg_popup::link
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
// for protocol extensions
// wlr_xdg_surface_configure
// wlr_xdg_surface_configure
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
 * Request that this toplevel surface consider itself in a tiled layout and some
 * edges are adjacent to another part of the tiling grid. `tiled_edges` is a
 * bitfield of `enum wlr_edges`. Returns the associated configure serial.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_toplevel_set_tiled(mut surface:
                                                        *mut wlr_xdg_surface,
                                                    mut tiled: uint32_t)
 -> uint32_t {
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_ROLE_TOPLEVEL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role == WLR_XDG_SURFACE_ROLE_TOPLEVEL\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell/wlr_xdg_toplevel.c\x00" as
                          *const u8 as *const libc::c_char,
                      605i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"uint32_t wlr_xdg_toplevel_set_tiled(struct wlr_xdg_surface *, uint32_t)\x00")).as_ptr());
    };
    (*(*surface).c2rust_unnamed.toplevel).server_pending.tiled = tiled;
    return schedule_xdg_surface_configure(surface);
}
