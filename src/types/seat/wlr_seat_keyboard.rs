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
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* Note: these are circular dependencies */
    pub type wlr_input_device_impl;
    pub type wlr_texture;
    pub type wlr_renderer;
    pub type wlr_primary_selection_source;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn mmap(__addr: *mut libc::c_void, __len: size_t, __prot: libc::c_int,
            __flags: libc::c_int, __fd: libc::c_int, __offset: __off64_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
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
    #[no_mangle]
    fn wl_array_add(array: *mut wl_array, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
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
    static wl_keyboard_interface: wl_interface;
    #[no_mangle]
    fn wlr_seat_client_for_wl_client(wlr_seat: *mut wlr_seat,
                                     wl_client: *mut wl_client)
     -> *mut wlr_seat_client;
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
    #[no_mangle]
    fn wlr_seat_client_next_serial(client: *mut wlr_seat_client) -> uint32_t;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    /* *
 * Creates a new wl_data_offer if there is a wl_data_source currently set as
 * the seat selection and sends it to the seat client, followed by the
 * wl_data_device.selection() event.  If there is no current selection, the
 * wl_data_device.selection() event will carry a NULL wl_data_offer.  If the
 * client does not have a wl_data_device for the seat nothing will be done.
 */
    #[no_mangle]
    fn seat_client_send_selection(seat_client: *mut wlr_seat_client);
    #[no_mangle]
    fn allocate_shm_file(size: size_t) -> libc::c_int;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
    pub events: C2RustUnnamed_1,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_input_device {
    pub impl_0: *const wlr_input_device_impl,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet_pad {
    pub impl_0: *mut wlr_tablet_pad_impl,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_list {
    pub capacity: size_t,
    pub length: size_t,
    pub items: *mut *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet {
    pub impl_0: *mut wlr_tablet_impl,
    pub events: C2RustUnnamed_5,
    pub name: *mut libc::c_char,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_touch {
    pub impl_0: *const wlr_touch_impl,
    pub events: C2RustUnnamed_6,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_switch {
    pub impl_0: *mut wlr_switch_impl,
    pub events: C2RustUnnamed_7,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
pub type wl_keyboard_keymap_format = libc::c_uint;
pub const WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1: wl_keyboard_keymap_format = 1;
pub const WL_KEYBOARD_KEYMAP_FORMAT_NO_KEYMAP: wl_keyboard_keymap_format = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_keyboard_interface {
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
    pub events: C2RustUnnamed_8,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
    pub events: C2RustUnnamed_9,
    pub serials: wlr_serial_ringset,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
    pub events: C2RustUnnamed_10,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
    pub events: C2RustUnnamed_11,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
    pub events: C2RustUnnamed_12,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
    pub events: C2RustUnnamed_13,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
    pub events: C2RustUnnamed_14,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
    pub events: C2RustUnnamed_15,
    pub point_destroy: wl_listener,
    pub source_destroy: wl_listener,
    pub seat_client_destroy: wl_listener,
    pub icon_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_15 {
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
    pub events: C2RustUnnamed_16,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_16 {
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
pub struct wlr_seat_keyboard_focus_change_event {
    pub seat: *mut wlr_seat,
    pub old_surface: *mut wlr_surface,
    pub new_surface: *mut wlr_surface,
}
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
unsafe extern "C" fn wl_keyboard_send_keymap(mut resource_: *mut wl_resource,
                                             mut format: uint32_t,
                                             mut fd: int32_t,
                                             mut size: uint32_t) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, format, fd, size);
}
#[inline]
unsafe extern "C" fn wl_keyboard_send_enter(mut resource_: *mut wl_resource,
                                            mut serial: uint32_t,
                                            mut surface: *mut wl_resource,
                                            mut keys: *mut wl_array) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, serial, surface,
                           keys);
}
#[inline]
unsafe extern "C" fn wl_keyboard_send_leave(mut resource_: *mut wl_resource,
                                            mut serial: uint32_t,
                                            mut surface: *mut wl_resource) {
    wl_resource_post_event(resource_, 2i32 as uint32_t, serial, surface);
}
#[inline]
unsafe extern "C" fn wl_keyboard_send_key(mut resource_: *mut wl_resource,
                                          mut serial: uint32_t,
                                          mut time: uint32_t,
                                          mut key: uint32_t,
                                          mut state: uint32_t) {
    wl_resource_post_event(resource_, 3i32 as uint32_t, serial, time, key,
                           state);
}
#[inline]
unsafe extern "C" fn wl_keyboard_send_modifiers(mut resource_:
                                                    *mut wl_resource,
                                                mut serial: uint32_t,
                                                mut mods_depressed: uint32_t,
                                                mut mods_latched: uint32_t,
                                                mut mods_locked: uint32_t,
                                                mut group: uint32_t) {
    wl_resource_post_event(resource_, 4i32 as uint32_t, serial,
                           mods_depressed, mods_latched, mods_locked, group);
}
#[inline]
unsafe extern "C" fn wl_keyboard_send_repeat_info(mut resource_:
                                                      *mut wl_resource,
                                                  mut rate: int32_t,
                                                  mut delay: int32_t) {
    wl_resource_post_event(resource_, 5i32 as uint32_t, rate, delay);
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
pub static mut default_touch_grab_impl: wlr_touch_grab_interface =
    wlr_touch_grab_interface{down: None,
                             up: None,
                             motion: None,
                             enter: None,
                             cancel: None,};
unsafe extern "C" fn default_keyboard_enter(mut grab:
                                                *mut wlr_seat_keyboard_grab,
                                            mut surface: *mut wlr_surface,
                                            mut keycodes: *mut uint32_t,
                                            mut num_keycodes: size_t,
                                            mut modifiers:
                                                *mut wlr_keyboard_modifiers) {
    wlr_seat_keyboard_enter((*grab).seat, surface, keycodes, num_keycodes,
                            modifiers);
}
unsafe extern "C" fn default_keyboard_key(mut grab:
                                              *mut wlr_seat_keyboard_grab,
                                          mut time: uint32_t,
                                          mut key: uint32_t,
                                          mut state: uint32_t) {
    wlr_seat_keyboard_send_key((*grab).seat, time, key, state);
}
unsafe extern "C" fn default_keyboard_modifiers(mut grab:
                                                    *mut wlr_seat_keyboard_grab,
                                                mut modifiers:
                                                    *mut wlr_keyboard_modifiers) {
    wlr_seat_keyboard_send_modifiers((*grab).seat, modifiers);
}
unsafe extern "C" fn default_keyboard_cancel(mut grab:
                                                 *mut wlr_seat_keyboard_grab) {
    // cannot be cancelled
}
#[no_mangle]
pub static mut default_keyboard_grab_impl: wlr_keyboard_grab_interface =
    unsafe {
        {
            let mut init =
                wlr_keyboard_grab_interface{enter:
                                                Some(default_keyboard_enter as
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
                                                Some(default_keyboard_key as
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
                                                Some(default_keyboard_modifiers
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut wlr_seat_keyboard_grab,
                                                                              _:
                                                                                  *mut wlr_keyboard_modifiers)
                                                             -> ()),
                                            cancel:
                                                Some(default_keyboard_cancel
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut wlr_seat_keyboard_grab)
                                                             -> ()),};
            init
        }
    };
unsafe extern "C" fn keyboard_release(mut client: *mut wl_client,
                                      mut resource: *mut wl_resource) {
    wl_resource_destroy(resource);
}
static mut keyboard_impl: wl_keyboard_interface =
    unsafe {
        {
            let mut init =
                wl_keyboard_interface{release:
                                          Some(keyboard_release as
                                                   unsafe extern "C" fn(_:
                                                                            *mut wl_client,
                                                                        _:
                                                                            *mut wl_resource)
                                                       -> ()),};
            init
        }
    };
unsafe extern "C" fn seat_client_from_keyboard_resource(mut resource:
                                                            *mut wl_resource)
 -> *mut wlr_seat_client {
    if wl_resource_instance_of(resource, &wl_keyboard_interface,
                               &keyboard_impl as *const wl_keyboard_interface
                                   as *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_keyboard_interface, &keyboard_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/seat/wlr_seat_keyboard.c\x00" as *const u8 as
                          *const libc::c_char, 58i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 81],
                                                &[libc::c_char; 81]>(b"struct wlr_seat_client *seat_client_from_keyboard_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_seat_client;
}
unsafe extern "C" fn keyboard_handle_resource_destroy(mut resource:
                                                          *mut wl_resource) {
    wl_list_remove(wl_resource_get_link(resource));
    seat_client_destroy_keyboard(resource);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_keyboard_send_key(mut wlr_seat:
                                                        *mut wlr_seat,
                                                    mut time: uint32_t,
                                                    mut key: uint32_t,
                                                    mut state: uint32_t) {
    let mut client: *mut wlr_seat_client =
        (*wlr_seat).keyboard_state.focused_client;
    if client.is_null() { return }
    let mut serial: uint32_t = wlr_seat_client_next_serial(client);
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).keyboards.next);
    while wl_resource_get_link(resource) !=
              &mut (*client).keyboards as *mut wl_list {
        if !seat_client_from_keyboard_resource(resource).is_null() {
            wl_keyboard_send_key(resource, serial, time, key, state);
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
unsafe extern "C" fn handle_keyboard_keymap(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut state: *mut wlr_seat_keyboard_state =
        (listener as *mut libc::c_char).offset(-56) as
            *mut wlr_seat_keyboard_state;
    let mut client: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
    let mut keyboard: *mut wlr_keyboard = data as *mut wlr_keyboard;
    if keyboard == (*state).keyboard {
        client =
            ((*(*state).seat).clients.next as *mut libc::c_char).offset(-16)
                as *mut wlr_seat_client;
        while &mut (*client).link as *mut wl_list !=
                  &mut (*(*state).seat).clients as *mut wl_list {
            seat_client_send_keymap(client, (*state).keyboard);
            client =
                ((*client).link.next as *mut libc::c_char).offset(-16) as
                    *mut wlr_seat_client
        }
    };
}
unsafe extern "C" fn handle_keyboard_repeat_info(mut listener:
                                                     *mut wl_listener,
                                                 mut data:
                                                     *mut libc::c_void) {
    let mut state: *mut wlr_seat_keyboard_state =
        (listener as *mut libc::c_char).offset(-80) as
            *mut wlr_seat_keyboard_state;
    let mut client: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
    client =
        ((*(*state).seat).clients.next as *mut libc::c_char).offset(-16) as
            *mut wlr_seat_client;
    while &mut (*client).link as *mut wl_list !=
              &mut (*(*state).seat).clients as *mut wl_list {
        seat_client_send_repeat_info(client, (*state).keyboard);
        client =
            ((*client).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_seat_client
    };
}
unsafe extern "C" fn handle_keyboard_destroy(mut listener: *mut wl_listener,
                                             mut data: *mut libc::c_void) {
    let mut state: *mut wlr_seat_keyboard_state =
        (listener as *mut libc::c_char).offset(-32) as
            *mut wlr_seat_keyboard_state;
    wlr_seat_set_keyboard((*state).seat, 0 as *mut wlr_input_device);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_set_keyboard(mut seat: *mut wlr_seat,
                                               mut device:
                                                   *mut wlr_input_device) {
    // TODO call this on device key event before the event reaches the
	// compositor and set a pending keyboard and then send the new keyboard
	// state on the next keyboard notify event.
    let mut keyboard: *mut wlr_keyboard =
        if !device.is_null() {
            (*device).c2rust_unnamed.keyboard
        } else { 0 as *mut wlr_keyboard };
    if (*seat).keyboard_state.keyboard == keyboard { return }
    if !(*seat).keyboard_state.keyboard.is_null() {
        wl_list_remove(&mut (*seat).keyboard_state.keyboard_destroy.link);
        wl_list_remove(&mut (*seat).keyboard_state.keyboard_keymap.link);
        wl_list_remove(&mut (*seat).keyboard_state.keyboard_repeat_info.link);
        (*seat).keyboard_state.keyboard = 0 as *mut wlr_keyboard
    }
    if !keyboard.is_null() {
        if (*device).type_0 as libc::c_uint ==
               WLR_INPUT_DEVICE_KEYBOARD as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"device->type == WLR_INPUT_DEVICE_KEYBOARD\x00" as
                              *const u8 as *const libc::c_char,
                          b"../types/seat/wlr_seat_keyboard.c\x00" as
                              *const u8 as *const libc::c_char,
                          138i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 73],
                                                    &[libc::c_char; 73]>(b"void wlr_seat_set_keyboard(struct wlr_seat *, struct wlr_input_device *)\x00")).as_ptr());
        };
        (*seat).keyboard_state.keyboard = keyboard;
        wl_signal_add(&mut (*device).events.destroy,
                      &mut (*seat).keyboard_state.keyboard_destroy);
        (*seat).keyboard_state.keyboard_destroy.notify =
            Some(handle_keyboard_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.keyboard).events.keymap,
                      &mut (*seat).keyboard_state.keyboard_keymap);
        (*seat).keyboard_state.keyboard_keymap.notify =
            Some(handle_keyboard_keymap as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.keyboard).events.repeat_info,
                      &mut (*seat).keyboard_state.keyboard_repeat_info);
        (*seat).keyboard_state.keyboard_repeat_info.notify =
            Some(handle_keyboard_repeat_info as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        let mut client: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
        client =
            ((*seat).clients.next as *mut libc::c_char).offset(-16) as
                *mut wlr_seat_client;
        while &mut (*client).link as *mut wl_list !=
                  &mut (*seat).clients as *mut wl_list {
            seat_client_send_keymap(client, keyboard);
            seat_client_send_repeat_info(client, keyboard);
            client =
                ((*client).link.next as *mut libc::c_char).offset(-16) as
                    *mut wlr_seat_client
        }
        wlr_seat_keyboard_send_modifiers(seat, &mut (*keyboard).modifiers);
    } else { (*seat).keyboard_state.keyboard = 0 as *mut wlr_keyboard };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_get_keyboard(mut seat: *mut wlr_seat)
 -> *mut wlr_keyboard {
    return (*seat).keyboard_state.keyboard;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_keyboard_start_grab(mut wlr_seat:
                                                          *mut wlr_seat,
                                                      mut grab:
                                                          *mut wlr_seat_keyboard_grab) {
    (*grab).seat = wlr_seat;
    (*wlr_seat).keyboard_state.grab = grab;
    wlr_signal_emit_safe(&mut (*wlr_seat).events.keyboard_grab_begin,
                         grab as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_keyboard_end_grab(mut wlr_seat:
                                                        *mut wlr_seat) {
    let mut grab: *mut wlr_seat_keyboard_grab =
        (*wlr_seat).keyboard_state.grab;
    if grab != (*wlr_seat).keyboard_state.default_grab {
        (*wlr_seat).keyboard_state.grab =
            (*wlr_seat).keyboard_state.default_grab;
        wlr_signal_emit_safe(&mut (*wlr_seat).events.keyboard_grab_end,
                             grab as *mut libc::c_void);
        if (*(*grab).interface).cancel.is_some() {
            (*(*grab).interface).cancel.expect("non-null function pointer")(grab);
        }
    };
}
unsafe extern "C" fn seat_keyboard_handle_surface_destroy(mut listener:
                                                              *mut wl_listener,
                                                          mut data:
                                                              *mut libc::c_void) {
    let mut state: *mut wlr_seat_keyboard_state =
        (listener as *mut libc::c_char).offset(-104) as
            *mut wlr_seat_keyboard_state;
    wl_list_remove(&mut (*state).surface_destroy.link);
    wl_list_init(&mut (*state).surface_destroy.link);
    wlr_seat_keyboard_clear_focus((*state).seat);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_keyboard_send_modifiers(mut seat:
                                                              *mut wlr_seat,
                                                          mut modifiers:
                                                              *mut wlr_keyboard_modifiers) {
    let mut client: *mut wlr_seat_client =
        (*seat).keyboard_state.focused_client;
    if client.is_null() { return }
    let mut serial: uint32_t = wlr_seat_client_next_serial(client);
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).keyboards.next);
    while wl_resource_get_link(resource) !=
              &mut (*client).keyboards as *mut wl_list {
        if !seat_client_from_keyboard_resource(resource).is_null() {
            if modifiers.is_null() {
                wl_keyboard_send_modifiers(resource, serial, 0i32 as uint32_t,
                                           0i32 as uint32_t, 0i32 as uint32_t,
                                           0i32 as uint32_t);
            } else {
                wl_keyboard_send_modifiers(resource, serial,
                                           (*modifiers).depressed,
                                           (*modifiers).latched,
                                           (*modifiers).locked,
                                           (*modifiers).group);
            }
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_keyboard_enter(mut seat: *mut wlr_seat,
                                                 mut surface:
                                                     *mut wlr_surface,
                                                 mut keycodes: *mut uint32_t,
                                                 mut num_keycodes: size_t,
                                                 mut modifiers:
                                                     *mut wlr_keyboard_modifiers) {
    if (*seat).keyboard_state.focused_surface == surface {
        // this surface already got an enter notify
        return
    }
    let mut client: *mut wlr_seat_client = 0 as *mut wlr_seat_client;
    if !surface.is_null() {
        let mut wl_client: *mut wl_client =
            wl_resource_get_client((*surface).resource);
        client = wlr_seat_client_for_wl_client(seat, wl_client)
    }
    let mut focused_client: *mut wlr_seat_client =
        (*seat).keyboard_state.focused_client;
    let mut focused_surface: *mut wlr_surface =
        (*seat).keyboard_state.focused_surface;
    // leave the previously entered surface
    if !focused_client.is_null() && !focused_surface.is_null() {
        let mut serial: uint32_t =
            wlr_seat_client_next_serial(focused_client);
        let mut resource: *mut wl_resource = 0 as *mut wl_resource;
        resource = 0 as *mut wl_resource;
        resource = wl_resource_from_link((*focused_client).keyboards.next);
        while wl_resource_get_link(resource) !=
                  &mut (*focused_client).keyboards as *mut wl_list {
            if !seat_client_from_keyboard_resource(resource).is_null() {
                wl_keyboard_send_leave(resource, serial,
                                       (*focused_surface).resource);
            }
            resource =
                wl_resource_from_link((*wl_resource_get_link(resource)).next)
        }
    }
    // enter the current surface
    if !client.is_null() {
        let mut keys: wl_array =
            wl_array{size: 0, alloc: 0, data: 0 as *mut libc::c_void,};
        wl_array_init(&mut keys);
        let mut i: size_t = 0i32 as size_t;
        while i < num_keycodes {
            let mut p: *mut uint32_t =
                wl_array_add(&mut keys,
                             ::std::mem::size_of::<uint32_t>() as
                                 libc::c_ulong) as *mut uint32_t;
            if p.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Cannot allocate memory, skipping keycode: %d\n\x00"
                             as *const u8 as *const libc::c_char,
                         b"../types/seat/wlr_seat_keyboard.c\x00" as *const u8
                             as *const libc::c_char, 261i32,
                         *keycodes.offset(i as isize));
            } else { *p = *keycodes.offset(i as isize) }
            i = i.wrapping_add(1)
        }
        let mut serial_0: uint32_t = wlr_seat_client_next_serial(client);
        let mut resource_0: *mut wl_resource = 0 as *mut wl_resource;
        resource_0 = 0 as *mut wl_resource;
        resource_0 = wl_resource_from_link((*client).keyboards.next);
        while wl_resource_get_link(resource_0) !=
                  &mut (*client).keyboards as *mut wl_list {
            if !seat_client_from_keyboard_resource(resource_0).is_null() {
                wl_keyboard_send_enter(resource_0, serial_0,
                                       (*surface).resource, &mut keys);
            }
            resource_0 =
                wl_resource_from_link((*wl_resource_get_link(resource_0)).next)
        }
        wl_array_release(&mut keys);
    }
    // reinitialize the focus destroy events
    wl_list_remove(&mut (*seat).keyboard_state.surface_destroy.link);
    wl_list_init(&mut (*seat).keyboard_state.surface_destroy.link);
    if !surface.is_null() {
        wl_signal_add(&mut (*surface).events.destroy,
                      &mut (*seat).keyboard_state.surface_destroy);
        (*seat).keyboard_state.surface_destroy.notify =
            Some(seat_keyboard_handle_surface_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ())
    }
    (*seat).keyboard_state.focused_client = client;
    (*seat).keyboard_state.focused_surface = surface;
    if !client.is_null() {
        // tell new client about any modifier change last,
		// as it targets seat->keyboard_state.focused_client
        wlr_seat_keyboard_send_modifiers(seat, modifiers);
        seat_client_send_selection(client);
    }
    let mut event: wlr_seat_keyboard_focus_change_event =
        {
            let mut init =
                wlr_seat_keyboard_focus_change_event{seat: seat,
                                                     old_surface:
                                                         focused_surface,
                                                     new_surface: surface,};
            init
        };
    wlr_signal_emit_safe(&mut (*seat).keyboard_state.events.focus_change,
                         &mut event as
                             *mut wlr_seat_keyboard_focus_change_event as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_keyboard_notify_enter(mut seat:
                                                            *mut wlr_seat,
                                                        mut surface:
                                                            *mut wlr_surface,
                                                        mut keycodes:
                                                            *mut uint32_t,
                                                        mut num_keycodes:
                                                            size_t,
                                                        mut modifiers:
                                                            *mut wlr_keyboard_modifiers) {
    let mut grab: *mut wlr_seat_keyboard_grab = (*seat).keyboard_state.grab;
    (*(*grab).interface).enter.expect("non-null function pointer")(grab,
                                                                   surface,
                                                                   keycodes,
                                                                   num_keycodes,
                                                                   modifiers);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_keyboard_clear_focus(mut seat:
                                                           *mut wlr_seat) {
    // TODO respect grabs here?
    wlr_seat_keyboard_enter(seat, 0 as *mut wlr_surface, 0 as *mut uint32_t,
                            0i32 as size_t, 0 as *mut wlr_keyboard_modifiers);
}
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
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_keyboard_has_grab(mut seat: *mut wlr_seat)
 -> bool {
    return (*(*seat).keyboard_state.grab).interface !=
               &default_keyboard_grab_impl as
                   *const wlr_keyboard_grab_interface;
}
/* *
 * Send the modifier state to focused keyboard resources. Compositors should use
 * `wlr_seat_keyboard_notify_modifiers()` to respect any keyboard grabs.
 */
/* *
 * Notify the seat that the modifiers for the keyboard have changed. Defers to
 * any keyboard grabs.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_keyboard_notify_modifiers(mut seat:
                                                                *mut wlr_seat,
                                                            mut modifiers:
                                                                *mut wlr_keyboard_modifiers) {
    clock_gettime(1i32, &mut (*seat).last_event);
    let mut grab: *mut wlr_seat_keyboard_grab = (*seat).keyboard_state.grab;
    (*(*grab).interface).modifiers.expect("non-null function pointer")(grab,
                                                                       modifiers);
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
#[no_mangle]
pub unsafe extern "C" fn wlr_seat_keyboard_notify_key(mut seat: *mut wlr_seat,
                                                      mut time: uint32_t,
                                                      mut key: uint32_t,
                                                      mut state: uint32_t) {
    clock_gettime(1i32, &mut (*seat).last_event);
    let mut grab: *mut wlr_seat_keyboard_grab = (*seat).keyboard_state.grab;
    (*(*grab).interface).key.expect("non-null function pointer")(grab, time,
                                                                 key, state);
}
unsafe extern "C" fn seat_client_send_keymap(mut client: *mut wlr_seat_client,
                                             mut keyboard:
                                                 *mut wlr_keyboard) {
    if keyboard.is_null() { return }
    // TODO: We should probably lift all of the keys set by the other
	// keyboard
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).keyboards.next);
    while wl_resource_get_link(resource) !=
              &mut (*client).keyboards as *mut wl_list {
        if !seat_client_from_keyboard_resource(resource).is_null() {
            let mut keymap_fd: libc::c_int =
                allocate_shm_file((*keyboard).keymap_size);
            if keymap_fd < 0i32 {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] creating a keymap file for %zu bytes failed\x00"
                             as *const u8 as *const libc::c_char,
                         b"../types/seat/wlr_seat_keyboard.c\x00" as *const u8
                             as *const libc::c_char, 353i32,
                         (*keyboard).keymap_size);
            } else {
                let mut ptr: *mut libc::c_void =
                    mmap(0 as *mut libc::c_void, (*keyboard).keymap_size,
                         0x1i32 | 0x2i32, 0x1i32, keymap_fd,
                         0i32 as __off64_t);
                if ptr == -1i32 as *mut libc::c_void {
                    _wlr_log(WLR_ERROR,
                             b"[%s:%d] failed to mmap() %zu bytes\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../types/seat/wlr_seat_keyboard.c\x00" as
                                 *const u8 as *const libc::c_char, 360i32,
                             (*keyboard).keymap_size);
                    close(keymap_fd);
                } else {
                    strcpy(ptr as *mut libc::c_char,
                           (*keyboard).keymap_string);
                    munmap(ptr, (*keyboard).keymap_size);
                    wl_keyboard_send_keymap(resource,
                                            WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1
                                                as libc::c_int as uint32_t,
                                            keymap_fd,
                                            (*keyboard).keymap_size as
                                                uint32_t);
                    close(keymap_fd);
                }
            }
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
unsafe extern "C" fn seat_client_send_repeat_info(mut client:
                                                      *mut wlr_seat_client,
                                                  mut keyboard:
                                                      *mut wlr_keyboard) {
    if keyboard.is_null() { return }
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*client).keyboards.next);
    while wl_resource_get_link(resource) !=
              &mut (*client).keyboards as *mut wl_list {
        if !seat_client_from_keyboard_resource(resource).is_null() {
            if wl_resource_get_version(resource) >= 4i32 {
                wl_keyboard_send_repeat_info(resource,
                                             (*keyboard).repeat_info.rate,
                                             (*keyboard).repeat_info.delay);
            }
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
#[no_mangle]
pub unsafe extern "C" fn seat_client_create_keyboard(mut seat_client:
                                                         *mut wlr_seat_client,
                                                     mut version: uint32_t,
                                                     mut id: uint32_t) {
    let mut resource: *mut wl_resource =
        wl_resource_create((*seat_client).client, &wl_keyboard_interface,
                           version as libc::c_int, id);
    if resource.is_null() {
        wl_client_post_no_memory((*seat_client).client);
        return
    }
    wl_resource_set_implementation(resource,
                                   &keyboard_impl as
                                       *const wl_keyboard_interface as
                                       *const libc::c_void,
                                   seat_client as *mut libc::c_void,
                                   Some(keyboard_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_insert(&mut (*seat_client).keyboards,
                   wl_resource_get_link(resource));
    let mut keyboard: *mut wlr_keyboard =
        (*(*seat_client).seat).keyboard_state.keyboard;
    seat_client_send_keymap(seat_client, keyboard);
    seat_client_send_repeat_info(seat_client, keyboard);
    // TODO possibly handle the case where this keyboard needs an enter
	// right away
}
#[no_mangle]
pub unsafe extern "C" fn seat_client_destroy_keyboard(mut resource:
                                                          *mut wl_resource) {
    let mut seat_client: *mut wlr_seat_client =
        seat_client_from_keyboard_resource(resource);
    if seat_client.is_null() { return }
    wl_resource_set_user_data(resource, 0 as *mut libc::c_void);
}
