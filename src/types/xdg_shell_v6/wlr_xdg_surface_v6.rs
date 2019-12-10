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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_event_source_timer_update(source: *mut wl_event_source,
                                    ms_delay: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_event_loop_add_idle(loop_0: *mut wl_event_loop,
                              func: wl_event_loop_idle_func_t,
                              data: *mut libc::c_void)
     -> *mut wl_event_source;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn wl_display_next_serial(display: *mut wl_display) -> uint32_t;
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
    #[no_mangle]
    fn wl_resource_post_error(resource: *mut wl_resource, code: uint32_t,
                              msg: *const libc::c_char, _: ...);
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
    fn handle_xdg_surface_v6_toplevel_committed(surface:
                                                    *mut wlr_xdg_surface_v6);
    #[no_mangle]
    fn handle_xdg_surface_v6_popup_committed(surface:
                                                 *mut wlr_xdg_surface_v6);
    #[no_mangle]
    fn create_xdg_toplevel_v6(xdg_surface: *mut wlr_xdg_surface_v6,
                              id: uint32_t);
    #[no_mangle]
    fn create_xdg_popup_v6(xdg_surface: *mut wlr_xdg_surface_v6,
                           parent: *mut wlr_xdg_surface_v6,
                           positioner: *mut wlr_xdg_positioner_v6_resource,
                           id: int32_t);
    #[no_mangle]
    fn get_xdg_positioner_v6_from_resource(resource: *mut wl_resource)
     -> *mut wlr_xdg_positioner_v6_resource;
    #[no_mangle]
    fn handle_xdg_toplevel_v6_ack_configure(surface: *mut wlr_xdg_surface_v6,
                                            configure:
                                                *mut wlr_xdg_surface_v6_configure);
    #[no_mangle]
    fn get_xdg_shell_v6_popup_grab_from_seat(shell: *mut wlr_xdg_shell_v6,
                                             seat: *mut wlr_seat)
     -> *mut wlr_xdg_popup_grab_v6;
    #[no_mangle]
    fn destroy_xdg_toplevel_v6(surface: *mut wlr_xdg_surface_v6);
    #[no_mangle]
    fn destroy_xdg_popup_v6(surface: *mut wlr_xdg_surface_v6);
    #[no_mangle]
    fn compare_xdg_surface_v6_toplevel_state(state: *mut wlr_xdg_toplevel_v6)
     -> bool;
    #[no_mangle]
    fn send_xdg_toplevel_v6_configure(surface: *mut wlr_xdg_surface_v6,
                                      configure:
                                          *mut wlr_xdg_surface_v6_configure);
    #[no_mangle]
    static xdg_toplevel_v6_surface_role: wlr_surface_role;
    #[no_mangle]
    static xdg_popup_v6_surface_role: wlr_surface_role;
    #[no_mangle]
    static zxdg_surface_v6_interface: wl_interface;
    #[no_mangle]
    fn wlr_seat_keyboard_end_grab(wlr_seat: *mut wlr_seat);
    #[no_mangle]
    fn wlr_seat_pointer_end_grab(wlr_seat: *mut wlr_seat);
    #[no_mangle]
    fn wlr_surface_for_each_surface(surface: *mut wlr_surface,
                                    iterator: wlr_surface_iterator_func_t,
                                    user_data: *mut libc::c_void);
    #[no_mangle]
    fn wlr_surface_get_extends(surface: *mut wlr_surface,
                               box_0: *mut wlr_box);
    #[no_mangle]
    fn wlr_surface_surface_at(surface: *mut wlr_surface, sx: libc::c_double,
                              sy: libc::c_double, sub_x: *mut libc::c_double,
                              sub_y: *mut libc::c_double) -> *mut wlr_surface;
    #[no_mangle]
    fn wlr_surface_has_buffer(surface: *mut wlr_surface) -> bool;
    #[no_mangle]
    fn wlr_box_intersection(dest: *mut wlr_box, box_a: *const wlr_box,
                            box_b: *const wlr_box) -> bool;
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
pub type wl_event_loop_idle_func_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
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
/* *
 * Holds the double-buffered output state.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
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
pub type wlr_surface_state_field = libc::c_uint;
pub const WLR_SURFACE_STATE_FRAME_CALLBACK_LIST: wlr_surface_state_field =
    128;
pub const WLR_SURFACE_STATE_SCALE: wlr_surface_state_field = 64;
pub const WLR_SURFACE_STATE_TRANSFORM: wlr_surface_state_field = 32;
pub const WLR_SURFACE_STATE_INPUT_REGION: wlr_surface_state_field = 16;
pub const WLR_SURFACE_STATE_OPAQUE_REGION: wlr_surface_state_field = 8;
pub const WLR_SURFACE_STATE_BUFFER_DAMAGE: wlr_surface_state_field = 4;
pub const WLR_SURFACE_STATE_SURFACE_DAMAGE: wlr_surface_state_field = 2;
pub const WLR_SURFACE_STATE_BUFFER: wlr_surface_state_field = 1;
pub type wlr_surface_iterator_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wlr_surface, _: libc::c_int,
                                _: libc::c_int, _: *mut libc::c_void) -> ()>;
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
pub type zxdg_surface_v6_error = libc::c_uint;
pub const ZXDG_SURFACE_V6_ERROR_UNCONFIGURED_BUFFER: zxdg_surface_v6_error =
    3;
pub const ZXDG_SURFACE_V6_ERROR_ALREADY_CONSTRUCTED: zxdg_surface_v6_error =
    2;
pub const ZXDG_SURFACE_V6_ERROR_NOT_CONSTRUCTED: zxdg_surface_v6_error = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct zxdg_surface_v6_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub get_toplevel: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                  _: *mut wl_resource,
                                                  _: uint32_t) -> ()>,
    pub get_popup: Option<unsafe extern "C" fn(_: *mut wl_client,
                                               _: *mut wl_resource,
                                               _: uint32_t,
                                               _: *mut wl_resource,
                                               _: *mut wl_resource) -> ()>,
    pub set_window_geometry: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                         _: *mut wl_resource,
                                                         _: int32_t,
                                                         _: int32_t,
                                                         _: int32_t,
                                                         _: int32_t) -> ()>,
    pub ack_configure: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                   _: *mut wl_resource,
                                                   _: uint32_t) -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_shell_v6 {
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
pub struct wlr_xdg_client_v6 {
    pub shell: *mut wlr_xdg_shell_v6,
    pub resource: *mut wl_resource,
    pub client: *mut wl_client,
    pub surfaces: wl_list,
    pub link: wl_list,
    pub ping_serial: uint32_t,
    pub ping_timer: *mut wl_event_source,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_positioner_v6 {
    pub anchor_rect: wlr_box,
    pub anchor: zxdg_positioner_v6_anchor,
    pub gravity: zxdg_positioner_v6_gravity,
    pub constraint_adjustment: zxdg_positioner_v6_constraint_adjustment,
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
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub destroy: wl_signal,
    pub ping_timeout: wl_signal,
    pub new_popup: wl_signal,
    pub map: wl_signal,
    pub unmap: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_13 {
    pub toplevel: *mut wlr_xdg_toplevel_v6,
    pub popup: *mut wlr_xdg_popup_v6,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_surface_v6_configure {
    pub link: wl_list,
    pub serial: uint32_t,
    pub toplevel_state: *mut wlr_xdg_toplevel_v6_state,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xdg_surface_v6_iterator_data {
    pub user_iterator: wlr_surface_iterator_func_t,
    pub user_data: *mut libc::c_void,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xdg_positioner_v6_resource {
    pub resource: *mut wl_resource,
    pub attrs: wlr_xdg_positioner_v6,
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
#[inline]
unsafe extern "C" fn zxdg_popup_v6_send_configure(mut resource_:
                                                      *mut wl_resource,
                                                  mut x: int32_t,
                                                  mut y: int32_t,
                                                  mut width: int32_t,
                                                  mut height: int32_t) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, x, y, width, height);
}
#[inline]
unsafe extern "C" fn zxdg_toplevel_v6_send_close(mut resource_:
                                                     *mut wl_resource) {
    wl_resource_post_event(resource_, 1i32 as uint32_t);
}
#[inline]
unsafe extern "C" fn zxdg_surface_v6_send_configure(mut resource_:
                                                        *mut wl_resource,
                                                    mut serial: uint32_t) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, serial);
}
#[inline]
unsafe extern "C" fn zxdg_shell_v6_send_ping(mut resource_: *mut wl_resource,
                                             mut serial: uint32_t) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, serial);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_is_xdg_surface_v6(mut surface:
                                                           *mut wlr_surface)
 -> bool {
    return (*surface).role ==
               &xdg_toplevel_v6_surface_role as *const wlr_surface_role ||
               (*surface).role ==
                   &xdg_popup_v6_surface_role as *const wlr_surface_role;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_surface_v6_from_wlr_surface(mut surface:
                                                                 *mut wlr_surface)
 -> *mut wlr_xdg_surface_v6 {
    if wlr_surface_is_xdg_surface_v6(surface) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_surface_is_xdg_surface_v6(surface)\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00" as
                          *const u8 as *const libc::c_char,
                      15i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[libc::c_char; 85]>(b"struct wlr_xdg_surface_v6 *wlr_xdg_surface_v6_from_wlr_surface(struct wlr_surface *)\x00")).as_ptr());
    };
    return (*surface).role_data as *mut wlr_xdg_surface_v6;
}
unsafe extern "C" fn xdg_surface_from_resource(mut resource: *mut wl_resource)
 -> *mut wlr_xdg_surface_v6 {
    if wl_resource_instance_of(resource, &zxdg_surface_v6_interface,
                               &zxdg_surface_v6_implementation as
                                   *const zxdg_surface_v6_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &zxdg_surface_v6_interface, &zxdg_surface_v6_implementation)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00" as
                          *const u8 as *const libc::c_char,
                      24i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 75],
                                                &[libc::c_char; 75]>(b"struct wlr_xdg_surface_v6 *xdg_surface_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_xdg_surface_v6;
}
unsafe extern "C" fn xdg_surface_configure_destroy(mut configure:
                                                       *mut wlr_xdg_surface_v6_configure) {
    if configure.is_null() { return }
    wl_list_remove(&mut (*configure).link);
    free((*configure).toplevel_state as *mut libc::c_void);
    free(configure as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn unmap_xdg_surface_v6(mut surface:
                                                  *mut wlr_xdg_surface_v6) {
    if (*surface).role as libc::c_uint !=
           WLR_XDG_SURFACE_V6_ROLE_NONE as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"surface->role != WLR_XDG_SURFACE_V6_ROLE_NONE\x00" as
                          *const u8 as *const libc::c_char,
                      b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00" as
                          *const u8 as *const libc::c_char,
                      39i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void unmap_xdg_surface_v6(struct wlr_xdg_surface_v6 *)\x00")).as_ptr());
    };
    // TODO: probably need to ungrab before this event
    if (*surface).mapped {
        wlr_signal_emit_safe(&mut (*surface).events.unmap,
                             surface as *mut libc::c_void);
    }
    match (*surface).role as libc::c_uint {
        1 => {
            free((*(*surface).c2rust_unnamed.toplevel).title as
                     *mut libc::c_void);
            (*(*surface).c2rust_unnamed.toplevel).title =
                0 as *mut libc::c_char;
            free((*(*surface).c2rust_unnamed.toplevel).app_id as
                     *mut libc::c_void);
            (*(*surface).c2rust_unnamed.toplevel).app_id =
                0 as *mut libc::c_char
        }
        2 => {
            if !(*(*surface).c2rust_unnamed.popup).seat.is_null() {
                let mut grab: *mut wlr_xdg_popup_grab_v6 =
                    get_xdg_shell_v6_popup_grab_from_seat((*(*surface).client).shell,
                                                          (*(*surface).c2rust_unnamed.popup).seat);
                wl_list_remove(&mut (*(*surface).c2rust_unnamed.popup).grab_link);
                if wl_list_empty(&mut (*grab).popups) != 0 {
                    if (*(*grab).seat).pointer_state.grab ==
                           &mut (*grab).pointer_grab as
                               *mut wlr_seat_pointer_grab {
                        wlr_seat_pointer_end_grab((*grab).seat);
                    }
                    if (*(*grab).seat).keyboard_state.grab ==
                           &mut (*grab).keyboard_grab as
                               *mut wlr_seat_keyboard_grab {
                        wlr_seat_keyboard_end_grab((*grab).seat);
                    }
                }
                (*(*surface).c2rust_unnamed.popup).seat = 0 as *mut wlr_seat
            }
        }
        0 => {
            if 0i32 != 0 &&
                   !(b"not reached\x00" as *const u8 as
                         *const libc::c_char).is_null() {
            } else {
                __assert_fail(b"false && \"not reached\"\x00" as *const u8 as
                                  *const libc::c_char,
                              b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00"
                                  as *const u8 as *const libc::c_char,
                              74i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 55],
                                                        &[libc::c_char; 55]>(b"void unmap_xdg_surface_v6(struct wlr_xdg_surface_v6 *)\x00")).as_ptr());
            };
        }
        _ => { }
    }
    let mut configure: *mut wlr_xdg_surface_v6_configure =
        0 as *mut wlr_xdg_surface_v6_configure;
    let mut tmp: *mut wlr_xdg_surface_v6_configure =
        0 as *mut wlr_xdg_surface_v6_configure;
    configure =
        ((*surface).configure_list.next as *mut libc::c_char).offset(-0) as
            *mut wlr_xdg_surface_v6_configure;
    tmp =
        ((*configure).link.next as *mut libc::c_char).offset(-0) as
            *mut wlr_xdg_surface_v6_configure;
    while &mut (*configure).link as *mut wl_list !=
              &mut (*surface).configure_list as *mut wl_list {
        xdg_surface_configure_destroy(configure);
        configure = tmp;
        tmp =
            ((*configure).link.next as *mut libc::c_char).offset(-0) as
                *mut wlr_xdg_surface_v6_configure
    }
    (*surface).mapped = 0i32 != 0;
    (*surface).configured = (*surface).mapped;
    (*surface).configure_serial = 0i32 as uint32_t;
    if !(*surface).configure_idle.is_null() {
        wl_event_source_remove((*surface).configure_idle);
        (*surface).configure_idle = 0 as *mut wl_event_source
    }
    (*surface).configure_next_serial = 0i32 as uint32_t;
    (*surface).has_next_geometry = 0i32 != 0;
    memset(&mut (*surface).geometry as *mut wlr_box as *mut libc::c_void,
           0i32, ::std::mem::size_of::<wlr_box>() as libc::c_ulong);
    memset(&mut (*surface).next_geometry as *mut wlr_box as *mut libc::c_void,
           0i32, ::std::mem::size_of::<wlr_box>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn destroy_xdg_surface_v6(mut surface:
                                                    *mut wlr_xdg_surface_v6) {
    if (*surface).role as libc::c_uint !=
           WLR_XDG_SURFACE_V6_ROLE_NONE as libc::c_int as libc::c_uint {
        unmap_xdg_surface_v6(surface);
    }
    wlr_signal_emit_safe(&mut (*surface).events.destroy,
                         surface as *mut libc::c_void);
    let mut popup_state: *mut wlr_xdg_popup_v6 = 0 as *mut wlr_xdg_popup_v6;
    let mut next: *mut wlr_xdg_popup_v6 = 0 as *mut wlr_xdg_popup_v6;
    popup_state =
        ((*surface).popups.next as *mut libc::c_char).offset(-8) as
            *mut wlr_xdg_popup_v6;
    next =
        ((*popup_state).link.next as *mut libc::c_char).offset(-8) as
            *mut wlr_xdg_popup_v6;
    while &mut (*popup_state).link as *mut wl_list !=
              &mut (*surface).popups as *mut wl_list {
        zxdg_popup_v6_send_popup_done((*popup_state).resource);
        destroy_xdg_popup_v6((*popup_state).base);
        popup_state = next;
        next =
            ((*popup_state).link.next as *mut libc::c_char).offset(-8) as
                *mut wlr_xdg_popup_v6
    }
    match (*surface).role as libc::c_uint {
        1 => { destroy_xdg_toplevel_v6(surface); }
        2 => { destroy_xdg_popup_v6(surface); }
        0 | _ => { }
    }
    wl_resource_set_user_data((*surface).resource, 0 as *mut libc::c_void);
    (*(*surface).surface).role_data = 0 as *mut libc::c_void;
    wl_list_remove(&mut (*surface).link);
    wl_list_remove(&mut (*surface).surface_destroy.link);
    wl_list_remove(&mut (*surface).surface_commit.link);
    free(surface as *mut libc::c_void);
}
unsafe extern "C" fn xdg_surface_handle_get_toplevel(mut client:
                                                         *mut wl_client,
                                                     mut resource:
                                                         *mut wl_resource,
                                                     mut id: uint32_t) {
    let mut xdg_surface: *mut wlr_xdg_surface_v6 =
        xdg_surface_from_resource(resource);
    create_xdg_toplevel_v6(xdg_surface, id);
}
unsafe extern "C" fn xdg_surface_handle_get_popup(mut wl_client:
                                                      *mut wl_client,
                                                  mut resource:
                                                      *mut wl_resource,
                                                  mut id: uint32_t,
                                                  mut parent_resource:
                                                      *mut wl_resource,
                                                  mut positioner_resource:
                                                      *mut wl_resource) {
    let mut xdg_surface: *mut wlr_xdg_surface_v6 =
        xdg_surface_from_resource(resource);
    let mut parent: *mut wlr_xdg_surface_v6 =
        xdg_surface_from_resource(parent_resource);
    let mut positioner: *mut wlr_xdg_positioner_v6_resource =
        get_xdg_positioner_v6_from_resource(positioner_resource);
    create_xdg_popup_v6(xdg_surface, parent, positioner, id as int32_t);
}
unsafe extern "C" fn xdg_surface_handle_ack_configure(mut client:
                                                          *mut wl_client,
                                                      mut resource:
                                                          *mut wl_resource,
                                                      mut serial: uint32_t) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        xdg_surface_from_resource(resource);
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_V6_ROLE_NONE as libc::c_int as libc::c_uint {
        wl_resource_post_error((*surface).resource,
                               ZXDG_SURFACE_V6_ERROR_NOT_CONSTRUCTED as
                                   libc::c_int as uint32_t,
                               b"xdg_surface must have a role\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    let mut found: bool = 0i32 != 0;
    let mut configure: *mut wlr_xdg_surface_v6_configure =
        0 as *mut wlr_xdg_surface_v6_configure;
    let mut tmp: *mut wlr_xdg_surface_v6_configure =
        0 as *mut wlr_xdg_surface_v6_configure;
    configure =
        ((*surface).configure_list.next as *mut libc::c_char).offset(-0) as
            *mut wlr_xdg_surface_v6_configure;
    tmp =
        ((*configure).link.next as *mut libc::c_char).offset(-0) as
            *mut wlr_xdg_surface_v6_configure;
    while &mut (*configure).link as *mut wl_list !=
              &mut (*surface).configure_list as *mut wl_list {
        if (*configure).serial < serial {
            xdg_surface_configure_destroy(configure);
            configure = tmp;
            tmp =
                ((*configure).link.next as *mut libc::c_char).offset(-0) as
                    *mut wlr_xdg_surface_v6_configure
        } else {
            if !((*configure).serial == serial) { break ; }
            found = 1i32 != 0;
            break ;
        }
    }
    if !found {
        wl_resource_post_error((*(*surface).client).resource,
                               ZXDG_SHELL_V6_ERROR_INVALID_SURFACE_STATE as
                                   libc::c_int as uint32_t,
                               b"wrong configure serial: %u\x00" as *const u8
                                   as *const libc::c_char, serial);
        return
    }
    match (*surface).role as libc::c_uint {
        0 => {
            if 0i32 != 0 &&
                   !(b"not reached\x00" as *const u8 as
                         *const libc::c_char).is_null() {
            } else {
                __assert_fail(b"0 && \"not reached\"\x00" as *const u8 as
                                  *const libc::c_char,
                              b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00"
                                  as *const u8 as *const libc::c_char,
                              180i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 90],
                                                        &[libc::c_char; 90]>(b"void xdg_surface_handle_ack_configure(struct wl_client *, struct wl_resource *, uint32_t)\x00")).as_ptr());
            };
        }
        1 => { handle_xdg_toplevel_v6_ack_configure(surface, configure); }
        2 | _ => { }
    }
    (*surface).configured = 1i32 != 0;
    (*surface).configure_serial = serial;
    xdg_surface_configure_destroy(configure);
}
unsafe extern "C" fn xdg_surface_handle_set_window_geometry(mut client:
                                                                *mut wl_client,
                                                            mut resource:
                                                                *mut wl_resource,
                                                            mut x: int32_t,
                                                            mut y: int32_t,
                                                            mut width:
                                                                int32_t,
                                                            mut height:
                                                                int32_t) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        xdg_surface_from_resource(resource);
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_V6_ROLE_NONE as libc::c_int as libc::c_uint {
        wl_resource_post_error((*surface).resource,
                               ZXDG_SURFACE_V6_ERROR_NOT_CONSTRUCTED as
                                   libc::c_int as uint32_t,
                               b"xdg_surface must have a role\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    if width <= 0i32 || height <= 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Client tried to set invalid geometry\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00" as
                     *const u8 as *const libc::c_char, 208i32);
        wl_resource_post_error(resource, -1i32 as uint32_t,
                               b"Tried to set invalid xdg-surface geometry\x00"
                                   as *const u8 as *const libc::c_char);
    }
    (*surface).has_next_geometry = 1i32 != 0;
    (*surface).next_geometry.height = height;
    (*surface).next_geometry.width = width;
    (*surface).next_geometry.x = x;
    (*surface).next_geometry.y = y;
}
unsafe extern "C" fn xdg_surface_handle_destroy(mut client: *mut wl_client,
                                                mut resource:
                                                    *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        xdg_surface_from_resource(resource);
    if (*surface).role as libc::c_uint !=
           WLR_XDG_SURFACE_V6_ROLE_NONE as libc::c_int as libc::c_uint {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Tried to destroy an xdg_surface before its role object\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00" as
                     *const u8 as *const libc::c_char, 226i32);
        return
    }
    wl_resource_destroy(resource);
}
static mut zxdg_surface_v6_implementation: zxdg_surface_v6_interface =
    unsafe {
        {
            let mut init =
                zxdg_surface_v6_interface{destroy:
                                              Some(xdg_surface_handle_destroy
                                                       as
                                                       unsafe extern "C" fn(_:
                                                                                *mut wl_client,
                                                                            _:
                                                                                *mut wl_resource)
                                                           -> ()),
                                          get_toplevel:
                                              Some(xdg_surface_handle_get_toplevel
                                                       as
                                                       unsafe extern "C" fn(_:
                                                                                *mut wl_client,
                                                                            _:
                                                                                *mut wl_resource,
                                                                            _:
                                                                                uint32_t)
                                                           -> ()),
                                          get_popup:
                                              Some(xdg_surface_handle_get_popup
                                                       as
                                                       unsafe extern "C" fn(_:
                                                                                *mut wl_client,
                                                                            _:
                                                                                *mut wl_resource,
                                                                            _:
                                                                                uint32_t,
                                                                            _:
                                                                                *mut wl_resource,
                                                                            _:
                                                                                *mut wl_resource)
                                                           -> ()),
                                          set_window_geometry:
                                              Some(xdg_surface_handle_set_window_geometry
                                                       as
                                                       unsafe extern "C" fn(_:
                                                                                *mut wl_client,
                                                                            _:
                                                                                *mut wl_resource,
                                                                            _:
                                                                                int32_t,
                                                                            _:
                                                                                int32_t,
                                                                            _:
                                                                                int32_t,
                                                                            _:
                                                                                int32_t)
                                                           -> ()),
                                          ack_configure:
                                              Some(xdg_surface_handle_ack_configure
                                                       as
                                                       unsafe extern "C" fn(_:
                                                                                *mut wl_client,
                                                                            _:
                                                                                *mut wl_resource,
                                                                            _:
                                                                                uint32_t)
                                                           -> ()),};
            init
        }
    };
unsafe extern "C" fn xdg_surface_send_configure(mut user_data:
                                                    *mut libc::c_void) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        user_data as *mut wlr_xdg_surface_v6;
    (*surface).configure_idle = 0 as *mut wl_event_source;
    let mut configure: *mut wlr_xdg_surface_v6_configure =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xdg_surface_v6_configure>() as
                   libc::c_ulong) as *mut wlr_xdg_surface_v6_configure;
    if configure.is_null() {
        wl_client_post_no_memory((*(*surface).client).client);
        return
    }
    wl_list_insert((*surface).configure_list.prev, &mut (*configure).link);
    (*configure).serial = (*surface).configure_next_serial;
    match (*surface).role as libc::c_uint {
        0 => {
            if 0i32 != 0 &&
                   !(b"not reached\x00" as *const u8 as
                         *const libc::c_char).is_null() {
            } else {
                __assert_fail(b"0 && \"not reached\"\x00" as *const u8 as
                                  *const libc::c_char,
                              b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00"
                                  as *const u8 as *const libc::c_char,
                              258i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 40],
                                                        &[libc::c_char; 40]>(b"void xdg_surface_send_configure(void *)\x00")).as_ptr());
            };
        }
        1 => { send_xdg_toplevel_v6_configure(surface, configure); }
        2 => {
            zxdg_popup_v6_send_configure((*(*surface).c2rust_unnamed.popup).resource,
                                         (*(*surface).c2rust_unnamed.popup).geometry.x,
                                         (*(*surface).c2rust_unnamed.popup).geometry.y,
                                         (*(*surface).c2rust_unnamed.popup).geometry.width,
                                         (*(*surface).c2rust_unnamed.popup).geometry.height);
        }
        _ => { }
    }
    zxdg_surface_v6_send_configure((*surface).resource, (*configure).serial);
}
#[no_mangle]
pub unsafe extern "C" fn schedule_xdg_surface_v6_configure(mut surface:
                                                               *mut wlr_xdg_surface_v6)
 -> uint32_t {
    let mut display: *mut wl_display =
        wl_client_get_display((*(*surface).client).client);
    let mut loop_0: *mut wl_event_loop = wl_display_get_event_loop(display);
    let mut pending_same: bool = 0i32 != 0;
    match (*surface).role as libc::c_uint {
        0 => {
            if 0i32 != 0 &&
                   !(b"not reached\x00" as *const u8 as
                         *const libc::c_char).is_null() {
            } else {
                __assert_fail(b"0 && \"not reached\"\x00" as *const u8 as
                                  *const libc::c_char,
                              b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00"
                                  as *const u8 as *const libc::c_char,
                              282i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 72],
                                                        &[libc::c_char; 72]>(b"uint32_t schedule_xdg_surface_v6_configure(struct wlr_xdg_surface_v6 *)\x00")).as_ptr());
            };
        }
        1 => {
            pending_same =
                compare_xdg_surface_v6_toplevel_state((*surface).c2rust_unnamed.toplevel)
        }
        2 | _ => { }
    }
    if !(*surface).configure_idle.is_null() {
        if !pending_same {
            // configure request already scheduled
            return (*surface).configure_next_serial
        }
        // configure request not necessary anymore
        wl_event_source_remove((*surface).configure_idle);
        (*surface).configure_idle = 0 as *mut wl_event_source;
        return 0i32 as uint32_t
    } else {
        if pending_same {
            // configure request not necessary
            return 0i32 as uint32_t
        }
        (*surface).configure_next_serial = wl_display_next_serial(display);
        (*surface).configure_idle =
            wl_event_loop_add_idle(loop_0,
                                   Some(xdg_surface_send_configure as
                                            unsafe extern "C" fn(_:
                                                                     *mut libc::c_void)
                                                -> ()),
                                   surface as *mut libc::c_void);
        return (*surface).configure_next_serial
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_surface_v6_ping(mut surface:
                                                     *mut wlr_xdg_surface_v6) {
    if (*(*surface).client).ping_serial != 0i32 as libc::c_uint {
        // already pinged
        return
    }
    (*(*surface).client).ping_serial =
        wl_display_next_serial(wl_client_get_display((*(*surface).client).client));
    wl_event_source_timer_update((*(*surface).client).ping_timer,
                                 (*(*(*surface).client).shell).ping_timeout as
                                     libc::c_int);
    zxdg_shell_v6_send_ping((*(*surface).client).resource,
                            (*(*surface).client).ping_serial);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_surface_v6_send_close(mut surface:
                                                           *mut wlr_xdg_surface_v6) {
    match (*surface).role as libc::c_uint {
        0 => {
            if 0i32 != 0 &&
                   !(b"not reached\x00" as *const u8 as
                         *const libc::c_char).is_null() {
            } else {
                __assert_fail(b"0 && \"not reached\"\x00" as *const u8 as
                                  *const libc::c_char,
                              b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00"
                                  as *const u8 as *const libc::c_char,
                              332i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 64],
                                                        &[libc::c_char; 64]>(b"void wlr_xdg_surface_v6_send_close(struct wlr_xdg_surface_v6 *)\x00")).as_ptr());
            };
        }
        1 => {
            if !(*surface).c2rust_unnamed.toplevel.is_null() {
                zxdg_toplevel_v6_send_close((*(*surface).c2rust_unnamed.toplevel).resource);
            }
        }
        2 => {
            if !(*surface).c2rust_unnamed.popup.is_null() {
                zxdg_popup_v6_send_popup_done((*(*surface).c2rust_unnamed.popup).resource);
            }
        }
        _ => { }
    };
}
unsafe extern "C" fn xdg_surface_handle_surface_destroy(mut listener:
                                                            *mut wl_listener,
                                                        mut data:
                                                            *mut libc::c_void) {
    let mut xdg_surface: *mut wlr_xdg_surface_v6 =
        (listener as *mut libc::c_char).offset(-152) as
            *mut wlr_xdg_surface_v6;
    destroy_xdg_surface_v6(xdg_surface);
}
unsafe extern "C" fn xdg_surface_handle_surface_commit(mut listener:
                                                           *mut wl_listener,
                                                       mut data:
                                                           *mut libc::c_void) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        (listener as *mut libc::c_char).offset(-176) as
            *mut wlr_xdg_surface_v6;
    if wlr_surface_has_buffer((*surface).surface) as libc::c_int != 0 &&
           !(*surface).configured {
        wl_resource_post_error((*surface).resource,
                               ZXDG_SURFACE_V6_ERROR_UNCONFIGURED_BUFFER as
                                   libc::c_int as uint32_t,
                               b"xdg_surface has never been configured\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    if (*surface).role as libc::c_uint ==
           WLR_XDG_SURFACE_V6_ROLE_NONE as libc::c_int as libc::c_uint {
        wl_resource_post_error((*surface).resource,
                               ZXDG_SURFACE_V6_ERROR_NOT_CONSTRUCTED as
                                   libc::c_int as uint32_t,
                               b"xdg_surface must have a role\x00" as
                                   *const u8 as *const libc::c_char);
        return
    };
}
#[no_mangle]
pub unsafe extern "C" fn handle_xdg_surface_v6_commit(mut wlr_surface:
                                                          *mut wlr_surface) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        wlr_xdg_surface_v6_from_wlr_surface(wlr_surface);
    if surface.is_null() { return }
    if (*surface).has_next_geometry {
        (*surface).has_next_geometry = 0i32 != 0;
        (*surface).geometry.x = (*surface).next_geometry.x;
        (*surface).geometry.y = (*surface).next_geometry.y;
        (*surface).geometry.width = (*surface).next_geometry.width;
        (*surface).geometry.height = (*surface).next_geometry.height
    }
    let mut current_block_12: u64;
    match (*surface).role as libc::c_uint {
        0 => {
            if 0i32 != 0 {
            } else {
                __assert_fail(b"false\x00" as *const u8 as
                                  *const libc::c_char,
                              b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00"
                                  as *const u8 as *const libc::c_char,
                              391i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 56],
                                                        &[libc::c_char; 56]>(b"void handle_xdg_surface_v6_commit(struct wlr_surface *)\x00")).as_ptr());
            };
            current_block_12 = 9209585778364289773;
        }
        1 => { current_block_12 = 9209585778364289773; }
        2 => {
            handle_xdg_surface_v6_popup_committed(surface);
            current_block_12 = 12349973810996921269;
        }
        _ => { current_block_12 = 12349973810996921269; }
    }
    match current_block_12 {
        9209585778364289773 => {
            handle_xdg_surface_v6_toplevel_committed(surface);
        }
        _ => { }
    }
    if !(*surface).added {
        (*surface).added = 1i32 != 0;
        wlr_signal_emit_safe(&mut (*(*(*surface).client).shell).events.new_surface,
                             surface as *mut libc::c_void);
    }
    if (*surface).configured as libc::c_int != 0 &&
           wlr_surface_has_buffer((*surface).surface) as libc::c_int != 0 &&
           !(*surface).mapped {
        (*surface).mapped = 1i32 != 0;
        wlr_signal_emit_safe(&mut (*surface).events.map,
                             surface as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn handle_xdg_surface_v6_precommit(mut wlr_surface:
                                                             *mut wlr_surface) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        wlr_xdg_surface_v6_from_wlr_surface(wlr_surface);
    if surface.is_null() { return }
    if (*wlr_surface).pending.committed &
           WLR_SURFACE_STATE_BUFFER as libc::c_int as libc::c_uint != 0 &&
           (*wlr_surface).pending.buffer_resource.is_null() {
        // This is a NULL commit
        if (*surface).configured as libc::c_int != 0 &&
               (*surface).mapped as libc::c_int != 0 {
            unmap_xdg_surface_v6(surface);
        }
    };
}
unsafe extern "C" fn xdg_surface_handle_resource_destroy(mut resource:
                                                             *mut wl_resource) {
    let mut surface: *mut wlr_xdg_surface_v6 =
        xdg_surface_from_resource(resource);
    if !surface.is_null() { destroy_xdg_surface_v6(surface); };
}
#[no_mangle]
pub unsafe extern "C" fn create_xdg_surface_v6(mut client:
                                                   *mut wlr_xdg_client_v6,
                                               mut surface: *mut wlr_surface,
                                               mut id: uint32_t)
 -> *mut wlr_xdg_surface_v6 {
    if wlr_surface_has_buffer(surface) {
        wl_resource_post_error((*client).resource,
                               ZXDG_SURFACE_V6_ERROR_UNCONFIGURED_BUFFER as
                                   libc::c_int as uint32_t,
                               b"xdg_surface must not have a buffer at creation\x00"
                                   as *const u8 as *const libc::c_char);
        return 0 as *mut wlr_xdg_surface_v6
    }
    let mut xdg_surface: *mut wlr_xdg_surface_v6 =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xdg_surface_v6>() as libc::c_ulong)
            as *mut wlr_xdg_surface_v6;
    if xdg_surface.is_null() {
        wl_client_post_no_memory((*client).client);
        return 0 as *mut wlr_xdg_surface_v6
    }
    (*xdg_surface).client = client;
    (*xdg_surface).role = WLR_XDG_SURFACE_V6_ROLE_NONE;
    (*xdg_surface).surface = surface;
    (*xdg_surface).resource =
        wl_resource_create((*client).client, &zxdg_surface_v6_interface,
                           wl_resource_get_version((*client).resource), id);
    if (*xdg_surface).resource.is_null() {
        free(xdg_surface as *mut libc::c_void);
        wl_client_post_no_memory((*client).client);
        return 0 as *mut wlr_xdg_surface_v6
    }
    wl_resource_set_implementation((*xdg_surface).resource,
                                   &zxdg_surface_v6_implementation as
                                       *const zxdg_surface_v6_interface as
                                       *const libc::c_void,
                                   xdg_surface as *mut libc::c_void,
                                   Some(xdg_surface_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_init(&mut (*xdg_surface).configure_list);
    wl_list_init(&mut (*xdg_surface).popups);
    wl_signal_init(&mut (*xdg_surface).events.destroy);
    wl_signal_init(&mut (*xdg_surface).events.ping_timeout);
    wl_signal_init(&mut (*xdg_surface).events.new_popup);
    wl_signal_init(&mut (*xdg_surface).events.map);
    wl_signal_init(&mut (*xdg_surface).events.unmap);
    wl_signal_add(&mut (*(*xdg_surface).surface).events.destroy,
                  &mut (*xdg_surface).surface_destroy);
    (*xdg_surface).surface_destroy.notify =
        Some(xdg_surface_handle_surface_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*(*xdg_surface).surface).events.commit,
                  &mut (*xdg_surface).surface_commit);
    (*xdg_surface).surface_commit.notify =
        Some(xdg_surface_handle_surface_commit as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] new xdg_surface %p (res %p)\x00" as *const u8 as
                 *const libc::c_char,
             b"../types/xdg_shell_v6/wlr_xdg_surface_v6.c\x00" as *const u8 as
                 *const libc::c_char, 485i32, xdg_surface,
             (*xdg_surface).resource);
    wl_list_insert(&mut (*client).surfaces, &mut (*xdg_surface).link);
    return xdg_surface;
}
unsafe extern "C" fn xdg_popup_v6_get_position(mut popup:
                                                   *mut wlr_xdg_popup_v6,
                                               mut popup_sx:
                                                   *mut libc::c_double,
                                               mut popup_sy:
                                                   *mut libc::c_double) {
    let mut parent: *mut wlr_xdg_surface_v6 = (*popup).parent;
    let mut parent_geo: wlr_box = wlr_box{x: 0, y: 0, width: 0, height: 0,};
    wlr_xdg_surface_v6_get_geometry(parent, &mut parent_geo);
    *popup_sx =
        (parent_geo.x + (*popup).geometry.x - (*(*popup).base).geometry.x) as
            libc::c_double;
    *popup_sy =
        (parent_geo.y + (*popup).geometry.y - (*(*popup).base).geometry.y) as
            libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_surface_v6_surface_at(mut surface:
                                                           *mut wlr_xdg_surface_v6,
                                                       mut sx: libc::c_double,
                                                       mut sy: libc::c_double,
                                                       mut sub_x:
                                                           *mut libc::c_double,
                                                       mut sub_y:
                                                           *mut libc::c_double)
 -> *mut wlr_surface {
    let mut popup_state: *mut wlr_xdg_popup_v6 = 0 as *mut wlr_xdg_popup_v6;
    popup_state =
        ((*surface).popups.next as *mut libc::c_char).offset(-8) as
            *mut wlr_xdg_popup_v6;
    while &mut (*popup_state).link as *mut wl_list !=
              &mut (*surface).popups as *mut wl_list {
        let mut popup: *mut wlr_xdg_surface_v6 = (*popup_state).base;
        let mut popup_sx: libc::c_double = 0.;
        let mut popup_sy: libc::c_double = 0.;
        xdg_popup_v6_get_position(popup_state, &mut popup_sx, &mut popup_sy);
        let mut sub: *mut wlr_surface =
            wlr_xdg_surface_v6_surface_at(popup, sx - popup_sx, sy - popup_sy,
                                          sub_x, sub_y);
        if !sub.is_null() { return sub }
        popup_state =
            ((*popup_state).link.next as *mut libc::c_char).offset(-8) as
                *mut wlr_xdg_popup_v6
    }
    return wlr_surface_surface_at((*surface).surface, sx, sy, sub_x, sub_y);
}
unsafe extern "C" fn xdg_surface_v6_iterator(mut surface: *mut wlr_surface,
                                             mut sx: libc::c_int,
                                             mut sy: libc::c_int,
                                             mut data: *mut libc::c_void) {
    let mut iter_data: *mut xdg_surface_v6_iterator_data =
        data as *mut xdg_surface_v6_iterator_data;
    (*iter_data).user_iterator.expect("non-null function pointer")(surface,
                                                                   (*iter_data).x
                                                                       + sx,
                                                                   (*iter_data).y
                                                                       + sy,
                                                                   (*iter_data).user_data);
}
unsafe extern "C" fn xdg_surface_v6_for_each_surface(mut surface:
                                                         *mut wlr_xdg_surface_v6,
                                                     mut x: libc::c_int,
                                                     mut y: libc::c_int,
                                                     mut iterator:
                                                         wlr_surface_iterator_func_t,
                                                     mut user_data:
                                                         *mut libc::c_void) {
    let mut data: xdg_surface_v6_iterator_data =
        {
            let mut init =
                xdg_surface_v6_iterator_data{user_iterator: iterator,
                                             user_data: user_data,
                                             x: x,
                                             y: y,};
            init
        };
    wlr_surface_for_each_surface((*surface).surface,
                                 Some(xdg_surface_v6_iterator as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_surface,
                                                               _: libc::c_int,
                                                               _: libc::c_int,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> ()),
                                 &mut data as
                                     *mut xdg_surface_v6_iterator_data as
                                     *mut libc::c_void);
    let mut popup_state: *mut wlr_xdg_popup_v6 = 0 as *mut wlr_xdg_popup_v6;
    popup_state =
        ((*surface).popups.next as *mut libc::c_char).offset(-8) as
            *mut wlr_xdg_popup_v6;
    while &mut (*popup_state).link as *mut wl_list !=
              &mut (*surface).popups as *mut wl_list {
        let mut popup: *mut wlr_xdg_surface_v6 = (*popup_state).base;
        if (*popup).configured {
            let mut popup_sx: libc::c_double = 0.;
            let mut popup_sy: libc::c_double = 0.;
            xdg_popup_v6_get_position(popup_state, &mut popup_sx,
                                      &mut popup_sy);
            xdg_surface_v6_for_each_surface(popup,
                                            (x as libc::c_double + popup_sx)
                                                as libc::c_int,
                                            (y as libc::c_double + popup_sy)
                                                as libc::c_int, iterator,
                                            user_data);
        }
        popup_state =
            ((*popup_state).link.next as *mut libc::c_char).offset(-8) as
                *mut wlr_xdg_popup_v6
    };
}
unsafe extern "C" fn xdg_surface_v6_for_each_popup(mut surface:
                                                       *mut wlr_xdg_surface_v6,
                                                   mut x: libc::c_int,
                                                   mut y: libc::c_int,
                                                   mut iterator:
                                                       wlr_surface_iterator_func_t,
                                                   mut user_data:
                                                       *mut libc::c_void) {
    let mut popup_state: *mut wlr_xdg_popup_v6 = 0 as *mut wlr_xdg_popup_v6;
    popup_state =
        ((*surface).popups.next as *mut libc::c_char).offset(-8) as
            *mut wlr_xdg_popup_v6;
    while &mut (*popup_state).link as *mut wl_list !=
              &mut (*surface).popups as *mut wl_list {
        let mut popup: *mut wlr_xdg_surface_v6 = (*popup_state).base;
        if (*popup).configured {
            let mut popup_sx: libc::c_double = 0.;
            let mut popup_sy: libc::c_double = 0.;
            xdg_popup_v6_get_position(popup_state, &mut popup_sx,
                                      &mut popup_sy);
            iterator.expect("non-null function pointer")((*popup).surface,
                                                         (x as libc::c_double
                                                              + popup_sx) as
                                                             libc::c_int,
                                                         (y as libc::c_double
                                                              + popup_sy) as
                                                             libc::c_int,
                                                         user_data);
            xdg_surface_v6_for_each_popup(popup,
                                          (x as libc::c_double + popup_sx) as
                                              libc::c_int,
                                          (y as libc::c_double + popup_sy) as
                                              libc::c_int, iterator,
                                          user_data);
        }
        popup_state =
            ((*popup_state).link.next as *mut libc::c_char).offset(-8) as
                *mut wlr_xdg_popup_v6
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_surface_v6_for_each_surface(mut surface:
                                                                 *mut wlr_xdg_surface_v6,
                                                             mut iterator:
                                                                 wlr_surface_iterator_func_t,
                                                             mut user_data:
                                                                 *mut libc::c_void) {
    xdg_surface_v6_for_each_surface(surface, 0i32, 0i32, iterator, user_data);
}
/* *
 * Call `iterator` on each surface and popup in the xdg-surface tree, with the
 * surface's position relative to the root xdg-surface. The function is called
 * from root to leaves (in rendering order).
 */
/* *
 * Call `iterator` on each popup in the xdg-surface tree, with the popup's
 * position relative to the root xdg-surface. The function is called from root
 * to leaves (in rendering order).
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_surface_v6_for_each_popup(mut surface:
                                                               *mut wlr_xdg_surface_v6,
                                                           mut iterator:
                                                               wlr_surface_iterator_func_t,
                                                           mut user_data:
                                                               *mut libc::c_void) {
    xdg_surface_v6_for_each_popup(surface, 0i32, 0i32, iterator, user_data);
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
/* *
  Invert the right/left anchor and gravity for this positioner. This can be
  used to "flip" the positioner around the anchor rect in the x direction.
 */
/* *
  Invert the top/bottom anchor and gravity for this positioner. This can be
  used to "flip" the positioner around the anchor rect in the y direction.
 */
/* *
 * Get the surface geometry.
 * This is either the geometry as set by the client, or defaulted to the bounds
 * of the surface + the subsurfaces (as specified by the protocol).
 *
 * The x and y value can be <0
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_surface_v6_get_geometry(mut surface:
                                                             *mut wlr_xdg_surface_v6,
                                                         mut box_0:
                                                             *mut wlr_box) {
    wlr_surface_get_extends((*surface).surface, box_0);
    /* The client never set the geometry */
    if (*surface).geometry.width == 0 { return }
    wlr_box_intersection(box_0, &mut (*surface).geometry, box_0);
}
