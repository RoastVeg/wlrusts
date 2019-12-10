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
    pub type wlr_pointer_impl;
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    pub type wl_proxy;
    pub type wl_buffer;
    pub type wl_callback;
    pub type wl_compositor;
    pub type wl_keyboard;
    pub type wl_pointer;
    pub type wl_registry;
    pub type wl_seat;
    pub type wl_surface;
    pub type zwp_tablet_manager_v2;
    pub type zwp_tablet_pad_group_v2;
    pub type zwp_tablet_pad_ring_v2;
    pub type zwp_tablet_pad_strip_v2;
    pub type zwp_tablet_pad_v2;
    pub type zwp_tablet_seat_v2;
    pub type zwp_tablet_tool_v2;
    pub type zwp_tablet_v2;
    pub type wl_egl_window;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_backend_impl;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    pub type wlr_surface;
    pub type wlr_output_impl;
    pub type zxdg_toplevel_decoration_v1;
    pub type xdg_toplevel;
    pub type xdg_surface;
    pub type zwp_relative_pointer_v1;
    pub type zwp_pointer_gesture_pinch_v1;
    pub type zwp_pointer_gesture_swipe_v1;
    pub type zwp_relative_pointer_manager_v1;
    pub type zwp_linux_dmabuf_v1;
    pub type wp_presentation;
    pub type zwp_pointer_gestures_v1;
    pub type zxdg_decoration_manager_v1;
    pub type xdg_wm_base;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    #[no_mangle]
    fn wlr_tablet_pad_destroy(pad: *mut wlr_tablet_pad);
    #[no_mangle]
    fn wlr_tablet_pad_init(pad: *mut wlr_tablet_pad,
                           impl_0: *mut wlr_tablet_pad_impl);
    #[no_mangle]
    fn wlr_list_push(list: *mut wlr_list, item: *mut libc::c_void) -> ssize_t;
    #[no_mangle]
    fn wlr_tablet_init(tablet: *mut wlr_tablet, impl_0: *mut wlr_tablet_impl);
    #[no_mangle]
    fn wlr_input_device_destroy(dev: *mut wlr_input_device);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn wl_proxy_get_user_data(proxy: *mut wl_proxy) -> *mut libc::c_void;
    #[no_mangle]
    fn wl_proxy_set_user_data(proxy: *mut wl_proxy,
                              user_data: *mut libc::c_void);
    #[no_mangle]
    fn wl_proxy_add_listener(proxy: *mut wl_proxy,
                             implementation:
                                 *mut Option<unsafe extern "C" fn() -> ()>,
                             data: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn wl_proxy_destroy(proxy: *mut wl_proxy);
    #[no_mangle]
    fn wl_proxy_marshal_constructor(proxy: *mut wl_proxy, opcode: uint32_t,
                                    interface: *const wl_interface, _: ...)
     -> *mut wl_proxy;
    #[no_mangle]
    static zwp_tablet_seat_v2_interface: wl_interface;
    #[no_mangle]
    fn wl_proxy_marshal(p: *mut wl_proxy, opcode: uint32_t, _: ...);
    #[no_mangle]
    fn create_wl_input_device(backend: *mut wlr_wl_backend,
                              type_0: wlr_input_device_type)
     -> *mut wlr_wl_input_device;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

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
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;

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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_pointer {
    pub impl_0: *const crate::src::backend::headless::input_device::wlr_pointer_impl,
    pub events: C2RustUnnamed_2,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_input_device {
    pub impl_0: *const wlr_input_device_impl,
    pub type_0: wlr_input_device_type,
    pub vendor: libc::c_uint,
    pub product: libc::c_uint,
    pub name: *mut libc::c_char,
    pub width_mm: libc::c_double,
    pub height_mm: libc::c_double,
    pub output_name: *mut libc::c_char,
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub events: C2RustUnnamed_3,
    pub data: *mut libc::c_void,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub destroy: wl_signal,
}

#[repr ( C )]#[derive(Copy, Clone)]
pub union C2RustUnnamed_4 {
    pub _device: *mut libc::c_void,
    pub keyboard: *mut wlr_keyboard,
    pub pointer: *mut wlr_pointer,
    pub switch_device: *mut wlr_switch,
    pub touch: *mut wlr_touch,
    pub tablet: *mut wlr_tablet,
    pub tablet_pad: *mut wlr_tablet_pad,
}
/* Note: these are circular dependencies */
// Or 0 if not applicable to this device
/* wlr_input_device.type determines which of these is valid */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_pad {
    pub impl_0: *mut wlr_tablet_pad_impl,
    pub events: C2RustUnnamed_5,
    pub button_count: size_t,
    pub ring_count: size_t,
    pub strip_count: size_t,
    pub groups: wl_list,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_list {
    pub capacity: size_t,
    pub length: size_t,
    pub items: *mut *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub button: wl_signal,
    pub ring: wl_signal,
    pub strip: wl_signal,
    pub attach_tablet: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_pad_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_tablet_pad) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet {
    pub impl_0: *mut wlr_tablet_impl,
    pub events: C2RustUnnamed_6,
    pub name: *mut libc::c_char,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub axis: wl_signal,
    pub proximity: wl_signal,
    pub tip: wl_signal,
    pub button: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_tablet) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_touch {
    pub impl_0: *const crate::src::backend::headless::input_device::wlr_touch_impl,
    pub events: C2RustUnnamed_7,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub down: wl_signal,
    pub up: wl_signal,
    pub motion: wl_signal,
    pub cancel: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_switch {
    pub impl_0: *mut crate::src::backend::headless::input_device::wlr_switch_impl,
    pub events: C2RustUnnamed_8,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
    pub toggle: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
}
pub type wlr_axis_source = libc::c_uint;
pub const WLR_AXIS_SOURCE_WHEEL_TILT: wlr_axis_source = 3;
pub const WLR_AXIS_SOURCE_CONTINUOUS: wlr_axis_source = 2;
pub const WLR_AXIS_SOURCE_FINGER: wlr_axis_source = 1;
pub const WLR_AXIS_SOURCE_WHEEL: wlr_axis_source = 0;
pub type wlr_tablet_tool_type = libc::c_uint;
pub const WLR_TABLET_TOOL_TYPE_TOTEM: wlr_tablet_tool_type = 8;
pub const WLR_TABLET_TOOL_TYPE_LENS: wlr_tablet_tool_type = 7;
pub const WLR_TABLET_TOOL_TYPE_MOUSE: wlr_tablet_tool_type = 6;
pub const WLR_TABLET_TOOL_TYPE_AIRBRUSH: wlr_tablet_tool_type = 5;
pub const WLR_TABLET_TOOL_TYPE_PENCIL: wlr_tablet_tool_type = 4;
pub const WLR_TABLET_TOOL_TYPE_BRUSH: wlr_tablet_tool_type = 3;
pub const WLR_TABLET_TOOL_TYPE_ERASER: wlr_tablet_tool_type = 2;
pub const WLR_TABLET_TOOL_TYPE_PEN: wlr_tablet_tool_type = 1;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_tool {
    pub type_0: wlr_tablet_tool_type,
    pub hardware_serial: uint64_t,
    pub hardware_wacom: uint64_t,
    pub tilt: bool,
    pub pressure: bool,
    pub distance: bool,
    pub rotation: bool,
    pub slider: bool,
    pub wheel: bool,
    pub events: C2RustUnnamed_9,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub destroy: wl_signal,
}
pub type wlr_tablet_tool_axes = libc::c_uint;
pub const WLR_TABLET_TOOL_AXIS_WHEEL: wlr_tablet_tool_axes = 256;
pub const WLR_TABLET_TOOL_AXIS_SLIDER: wlr_tablet_tool_axes = 128;
pub const WLR_TABLET_TOOL_AXIS_ROTATION: wlr_tablet_tool_axes = 64;
pub const WLR_TABLET_TOOL_AXIS_TILT_Y: wlr_tablet_tool_axes = 32;
pub const WLR_TABLET_TOOL_AXIS_TILT_X: wlr_tablet_tool_axes = 16;
pub const WLR_TABLET_TOOL_AXIS_PRESSURE: wlr_tablet_tool_axes = 8;
pub const WLR_TABLET_TOOL_AXIS_DISTANCE: wlr_tablet_tool_axes = 4;
pub const WLR_TABLET_TOOL_AXIS_Y: wlr_tablet_tool_axes = 2;
pub const WLR_TABLET_TOOL_AXIS_X: wlr_tablet_tool_axes = 1;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_tablet_tool_axis {
    pub device: *mut wlr_input_device,
    pub tool: *mut wlr_tablet_tool,
    pub time_msec: uint32_t,
    pub updated_axes: uint32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub dx: libc::c_double,
    pub dy: libc::c_double,
    pub pressure: libc::c_double,
    pub distance: libc::c_double,
    pub tilt_x: libc::c_double,
    pub tilt_y: libc::c_double,
    pub rotation: libc::c_double,
    pub slider: libc::c_double,
    pub wheel_delta: libc::c_double,
}
pub type wlr_tablet_tool_proximity_state = libc::c_uint;
pub const WLR_TABLET_TOOL_PROXIMITY_IN: wlr_tablet_tool_proximity_state = 1;
pub const WLR_TABLET_TOOL_PROXIMITY_OUT: wlr_tablet_tool_proximity_state = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_tablet_tool_proximity {
    pub device: *mut wlr_input_device,
    pub tool: *mut wlr_tablet_tool,
    pub time_msec: uint32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub state: wlr_tablet_tool_proximity_state,
}
pub type wlr_tablet_tool_tip_state = libc::c_uint;
pub const WLR_TABLET_TOOL_TIP_DOWN: wlr_tablet_tool_tip_state = 1;
pub const WLR_TABLET_TOOL_TIP_UP: wlr_tablet_tool_tip_state = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_tablet_tool_tip {
    pub device: *mut wlr_input_device,
    pub tool: *mut wlr_tablet_tool,
    pub time_msec: uint32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub state: wlr_tablet_tool_tip_state,
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
// char *
// From 0..1
// Relative to last event
// From 0..1
// From 0..1

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_tablet_tool_button {
    pub device: *mut wlr_input_device,
    pub tool: *mut wlr_tablet_tool,
    pub time_msec: uint32_t,
    pub button: uint32_t,
    pub state: wlr_button_state,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_pad_group {
    pub link: wl_list,
    pub button_count: size_t,
    pub buttons: *mut libc::c_uint,
    pub strip_count: size_t,
    pub strips: *mut libc::c_uint,
    pub ring_count: size_t,
    pub rings: *mut libc::c_uint,
    pub mode_count: libc::c_uint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_tablet_pad_button {
    pub time_msec: uint32_t,
    pub button: uint32_t,
    pub state: wlr_button_state,
    pub mode: libc::c_uint,
    pub group: libc::c_uint,
}
pub type wlr_tablet_pad_ring_source = libc::c_uint;
pub const WLR_TABLET_PAD_RING_SOURCE_FINGER: wlr_tablet_pad_ring_source = 1;
pub const WLR_TABLET_PAD_RING_SOURCE_UNKNOWN: wlr_tablet_pad_ring_source = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_tablet_pad_ring {
    pub time_msec: uint32_t,
    pub source: wlr_tablet_pad_ring_source,
    pub ring: uint32_t,
    pub position: libc::c_double,
    pub mode: libc::c_uint,
}
pub type wlr_tablet_pad_strip_source = libc::c_uint;
pub const WLR_TABLET_PAD_STRIP_SOURCE_FINGER: wlr_tablet_pad_strip_source = 1;
pub const WLR_TABLET_PAD_STRIP_SOURCE_UNKNOWN: wlr_tablet_pad_strip_source =
    0;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/*
 * NOTE: the wlr tablet pad implementation does not currently support tablets
 * with more than one mode. I don't own any such hardware so I cannot test it
 * and it is too complicated to make a meaningful implementation of blindly.
 */
//struct wlr_tablet_tool
// wlr_tablet_pad_group::link
// char *

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_tablet_pad_strip {
    pub time_msec: uint32_t,
    pub source: wlr_tablet_pad_strip_source,
    pub strip: uint32_t,
    pub position: libc::c_double,
    pub mode: libc::c_uint,
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwp_tablet_seat_v2_listener {
    pub tablet_added: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *mut zwp_tablet_seat_v2,
                                                  _: *mut zwp_tablet_v2)
                                 -> ()>,
    pub tool_added: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: *mut zwp_tablet_seat_v2,
                                                _: *mut zwp_tablet_tool_v2)
                               -> ()>,
    pub pad_added: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: *mut zwp_tablet_seat_v2,
                                               _: *mut zwp_tablet_pad_v2)
                              -> ()>,
}
pub type zwp_tablet_tool_v2_type = libc::c_uint;
pub const ZWP_TABLET_TOOL_V2_TYPE_LENS: zwp_tablet_tool_v2_type = 327;
pub const ZWP_TABLET_TOOL_V2_TYPE_MOUSE: zwp_tablet_tool_v2_type = 326;
pub const ZWP_TABLET_TOOL_V2_TYPE_FINGER: zwp_tablet_tool_v2_type = 325;
pub const ZWP_TABLET_TOOL_V2_TYPE_AIRBRUSH: zwp_tablet_tool_v2_type = 324;
pub const ZWP_TABLET_TOOL_V2_TYPE_PENCIL: zwp_tablet_tool_v2_type = 323;
pub const ZWP_TABLET_TOOL_V2_TYPE_BRUSH: zwp_tablet_tool_v2_type = 322;
pub const ZWP_TABLET_TOOL_V2_TYPE_ERASER: zwp_tablet_tool_v2_type = 321;
pub const ZWP_TABLET_TOOL_V2_TYPE_PEN: zwp_tablet_tool_v2_type = 320;
pub type zwp_tablet_tool_v2_capability = libc::c_uint;
pub const ZWP_TABLET_TOOL_V2_CAPABILITY_WHEEL: zwp_tablet_tool_v2_capability =
    6;
pub const ZWP_TABLET_TOOL_V2_CAPABILITY_SLIDER: zwp_tablet_tool_v2_capability
          =
    5;
pub const ZWP_TABLET_TOOL_V2_CAPABILITY_ROTATION:
          zwp_tablet_tool_v2_capability =
    4;
pub const ZWP_TABLET_TOOL_V2_CAPABILITY_DISTANCE:
          zwp_tablet_tool_v2_capability =
    3;
pub const ZWP_TABLET_TOOL_V2_CAPABILITY_PRESSURE:
          zwp_tablet_tool_v2_capability =
    2;
pub const ZWP_TABLET_TOOL_V2_CAPABILITY_TILT: zwp_tablet_tool_v2_capability =
    1;
pub type zwp_tablet_tool_v2_button_state = libc::c_uint;
pub const ZWP_TABLET_TOOL_V2_BUTTON_STATE_PRESSED:
          zwp_tablet_tool_v2_button_state =
    1;
pub const ZWP_TABLET_TOOL_V2_BUTTON_STATE_RELEASED:
          zwp_tablet_tool_v2_button_state =
    0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwp_tablet_tool_v2_listener {
    pub type_0: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *mut zwp_tablet_tool_v2,
                                            _: uint32_t) -> ()>,
    pub hardware_serial: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _:
                                                         *mut zwp_tablet_tool_v2,
                                                     _: uint32_t, _: uint32_t)
                                    -> ()>,
    pub hardware_id_wacom: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                       _:
                                                           *mut zwp_tablet_tool_v2,
                                                       _: uint32_t,
                                                       _: uint32_t) -> ()>,
    pub capability: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: *mut zwp_tablet_tool_v2,
                                                _: uint32_t) -> ()>,
    pub done: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_tool_v2) -> ()>,
    pub removed: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                             _: *mut zwp_tablet_tool_v2)
                            -> ()>,
    pub proximity_in: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *mut zwp_tablet_tool_v2,
                                                  _: uint32_t,
                                                  _: *mut zwp_tablet_v2,
                                                  _: *mut wl_surface) -> ()>,
    pub proximity_out: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                   _: *mut zwp_tablet_tool_v2)
                                  -> ()>,
    pub down: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_tool_v2,
                                          _: uint32_t) -> ()>,
    pub up: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                        _: *mut zwp_tablet_tool_v2) -> ()>,
    pub motion: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *mut zwp_tablet_tool_v2,
                                            _: wl_fixed_t, _: wl_fixed_t)
                           -> ()>,
    pub pressure: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: *mut zwp_tablet_tool_v2,
                                              _: uint32_t) -> ()>,
    pub distance: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: *mut zwp_tablet_tool_v2,
                                              _: uint32_t) -> ()>,
    pub tilt: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_tool_v2,
                                          _: wl_fixed_t, _: wl_fixed_t)
                         -> ()>,
    pub rotation: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: *mut zwp_tablet_tool_v2,
                                              _: wl_fixed_t) -> ()>,
    pub slider: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *mut zwp_tablet_tool_v2,
                                            _: int32_t) -> ()>,
    pub wheel: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut zwp_tablet_tool_v2,
                                           _: wl_fixed_t, _: int32_t) -> ()>,
    pub button: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *mut zwp_tablet_tool_v2,
                                            _: uint32_t, _: uint32_t,
                                            _: uint32_t) -> ()>,
    pub frame: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut zwp_tablet_tool_v2,
                                           _: uint32_t) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwp_tablet_v2_listener {
    pub name: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_v2,
                                          _: *const libc::c_char) -> ()>,
    pub id: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                        _: *mut zwp_tablet_v2, _: uint32_t,
                                        _: uint32_t) -> ()>,
    pub path: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_v2,
                                          _: *const libc::c_char) -> ()>,
    pub done: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_v2) -> ()>,
    pub removed: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                             _: *mut zwp_tablet_v2) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwp_tablet_pad_ring_v2_listener {
    pub source: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *mut zwp_tablet_pad_ring_v2,
                                            _: uint32_t) -> ()>,
    pub angle: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut zwp_tablet_pad_ring_v2,
                                           _: wl_fixed_t) -> ()>,
    pub stop: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_pad_ring_v2)
                         -> ()>,
    pub frame: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut zwp_tablet_pad_ring_v2,
                                           _: uint32_t) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwp_tablet_pad_strip_v2_listener {
    pub source: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *mut zwp_tablet_pad_strip_v2,
                                            _: uint32_t) -> ()>,
    pub position: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: *mut zwp_tablet_pad_strip_v2,
                                              _: uint32_t) -> ()>,
    pub stop: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_pad_strip_v2)
                         -> ()>,
    pub frame: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut zwp_tablet_pad_strip_v2,
                                           _: uint32_t) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwp_tablet_pad_group_v2_listener {
    pub buttons: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                             _: *mut zwp_tablet_pad_group_v2,
                                             _: *mut wl_array) -> ()>,
    pub ring: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_pad_group_v2,
                                          _: *mut zwp_tablet_pad_ring_v2)
                         -> ()>,
    pub strip: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut zwp_tablet_pad_group_v2,
                                           _: *mut zwp_tablet_pad_strip_v2)
                          -> ()>,
    pub modes: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut zwp_tablet_pad_group_v2,
                                           _: uint32_t) -> ()>,
    pub done: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_pad_group_v2)
                         -> ()>,
    pub mode_switch: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _:
                                                     *mut zwp_tablet_pad_group_v2,
                                                 _: uint32_t, _: uint32_t,
                                                 _: uint32_t) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwp_tablet_pad_v2_listener {
    pub group: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut zwp_tablet_pad_v2,
                                           _: *mut zwp_tablet_pad_group_v2)
                          -> ()>,
    pub path: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_pad_v2,
                                          _: *const libc::c_char) -> ()>,
    pub buttons: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                             _: *mut zwp_tablet_pad_v2,
                                             _: uint32_t) -> ()>,
    pub done: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut zwp_tablet_pad_v2) -> ()>,
    pub button: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *mut zwp_tablet_pad_v2,
                                            _: uint32_t, _: uint32_t,
                                            _: uint32_t) -> ()>,
    pub enter: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut zwp_tablet_pad_v2,
                                           _: uint32_t, _: *mut zwp_tablet_v2,
                                           _: *mut wl_surface) -> ()>,
    pub leave: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut zwp_tablet_pad_v2,
                                           _: uint32_t, _: *mut wl_surface)
                          -> ()>,
    pub removed: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                             _: *mut zwp_tablet_pad_v2)
                            -> ()>,
}
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;

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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_format {
    pub format: uint32_t,
    pub len: size_t,
    pub cap: size_t,
    pub modifiers: [uint64_t; 0],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub cap: size_t,
    pub formats: *mut *mut wlr_drm_format,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_egl {
    pub platform: EGLenum,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
    pub exts_str: *const libc::c_char,
    pub exts: C2RustUnnamed_10,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
    pub bind_wayland_display_wl: bool,
    pub buffer_age_ext: bool,
    pub image_base_khr: bool,
    pub image_dma_buf_export_mesa: bool,
    pub image_dmabuf_import_ext: bool,
    pub image_dmabuf_import_modifiers_ext: bool,
    pub swap_buffers_with_damage_ext: bool,
    pub swap_buffers_with_damage_khr: bool,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_backend {
    pub impl_0: *const crate::src::backend::backend::wlr_backend_impl,
    pub events: C2RustUnnamed_11,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_renderer {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_renderer_impl,
    pub events: C2RustUnnamed_12,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_12 {
    pub destroy: wl_signal,
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
/* *
 * A client buffer.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_texture {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_texture_impl,
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
    pub texture: *mut wlr_texture,
    pub surface: *mut crate::src::types::data_device::wlr_data_device::wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_13,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
    pub destroy: wl_signal,
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
    pub events: C2RustUnnamed_14,
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
pub struct C2RustUnnamed_14 {
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
pub struct wlr_wl_backend {
    pub backend: wlr_backend,
    pub started: bool,
    pub local_display: *mut wl_display,
    pub devices: wl_list,
    pub outputs: wl_list,
    pub egl: wlr_egl,
    pub renderer: *mut wlr_renderer,
    pub requested_outputs: size_t,
    pub last_output_num: size_t,
    pub local_display_destroy: wl_listener,
    pub remote_display: *mut wl_display,
    pub remote_display_src: *mut wl_event_source,
    pub registry: *mut wl_registry,
    pub compositor: *mut wl_compositor,
    pub xdg_wm_base: *mut xdg_wm_base,
    pub zxdg_decoration_manager_v1: *mut zxdg_decoration_manager_v1,
    pub zwp_pointer_gestures_v1: *mut zwp_pointer_gestures_v1,
    pub presentation: *mut wp_presentation,
    pub zwp_linux_dmabuf_v1: *mut zwp_linux_dmabuf_v1,
    pub zwp_relative_pointer_manager_v1: *mut zwp_relative_pointer_manager_v1,
    pub seat: *mut wl_seat,
    pub pointer: *mut wl_pointer,
    pub keyboard: *mut wl_keyboard,
    pub current_pointer: *mut wlr_wl_pointer,
    pub tablet_manager: *mut zwp_tablet_manager_v2,
    pub seat_name: *mut libc::c_char,
    pub linux_dmabuf_v1_formats: wlr_drm_format_set,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_wl_pointer {
    pub wlr_pointer: wlr_pointer,
    pub input_device: *mut wlr_wl_input_device,
    pub wl_pointer: *mut wl_pointer,
    pub gesture_swipe: *mut zwp_pointer_gesture_swipe_v1,
    pub gesture_pinch: *mut zwp_pointer_gesture_pinch_v1,
    pub relative_pointer: *mut zwp_relative_pointer_v1,
    pub axis_source: wlr_axis_source,
    pub axis_discrete: int32_t,
    pub output: *mut wlr_wl_output,
    pub output_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_wl_output {
    pub wlr_output: wlr_output,
    pub backend: *mut wlr_wl_backend,
    pub link: wl_list,
    pub surface: *mut wl_surface,
    pub frame_callback: *mut wl_callback,
    pub xdg_surface: *mut xdg_surface,
    pub xdg_toplevel: *mut xdg_toplevel,
    pub zxdg_toplevel_decoration_v1: *mut zxdg_toplevel_decoration_v1,
    pub egl_window: *mut wl_egl_window,
    pub egl_surface: EGLSurface,
    pub pending_wl_buffer: *mut wl_buffer,
    pub current_wl_buffer: *mut wl_buffer,
    pub current_buffer: *mut wlr_buffer,
    pub presentation_feedbacks: wl_list,
    pub enter_serial: uint32_t,
    pub cursor: C2RustUnnamed_15,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_15 {
    pub surface: *mut wl_surface,
    pub egl_window: *mut wl_egl_window,
    pub hotspot_x: int32_t,
    pub hotspot_y: int32_t,
    pub width: int32_t,
    pub height: int32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_wl_input_device {
    pub wlr_input_device: wlr_input_device,
    pub fingers: uint32_t,
    pub backend: *mut wlr_wl_backend,
    pub resource: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_wl_tablet_seat {
    pub tablet_seat: *mut zwp_tablet_seat_v2,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_wl_tablet_pad_group {
    pub pad_group: *mut zwp_tablet_pad_group_v2,
    pub pad: *mut wlr_tablet_pad,
    pub mode: libc::c_uint,
    pub group: wlr_tablet_pad_group,
    pub rings: wl_list,
    pub strips: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_wl_tablet_pad_strip {
    pub link: wl_list,
    pub strip: *mut zwp_tablet_pad_strip_v2,
    pub group: *mut wlr_wl_tablet_pad_group,
    pub index: size_t,
    pub source: wlr_tablet_pad_strip_source,
    pub position: libc::c_double,
    pub stopped: bool,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_wl_tablet_pad_ring {
    pub link: wl_list,
    pub ring: *mut zwp_tablet_pad_ring_v2,
    pub group: *mut wlr_wl_tablet_pad_group,
    pub index: size_t,
    pub source: wlr_tablet_pad_ring_source,
    pub angle: libc::c_double,
    pub stopped: bool,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_wl_tablet_tool {
    pub tool: *mut zwp_tablet_tool_v2,
    pub wlr_tool: wlr_tablet_tool,
    pub tablet: *mut wlr_wl_input_device,
    pub pre_x: libc::c_double,
    pub pre_y: libc::c_double,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub pressure: libc::c_double,
    pub distance: libc::c_double,
    pub tilt_x: libc::c_double,
    pub tilt_y: libc::c_double,
    pub rotation: libc::c_double,
    pub slider: libc::c_double,
    pub wheel_delta: libc::c_double,
    pub is_in: bool,
    pub is_out: bool,
    pub is_up: bool,
    pub is_down: bool,
}
#[inline]
unsafe extern "C" fn wl_fixed_to_double(mut f: wl_fixed_t) -> libc::c_double {
    let mut u: C2RustUnnamed = C2RustUnnamed{d: 0.,};
    u.i =
        ((1023i64 + 44i64 << 52i32) + (1i64 << 51i32) + f as libc::c_longlong)
            as int64_t;
    return u.d - (3i64 << 43i32) as libc::c_double;
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[inline]
unsafe extern "C" fn zwp_tablet_manager_v2_get_tablet_seat(mut zwp_tablet_manager_v2:
                                                               *mut zwp_tablet_manager_v2,
                                                           mut seat:
                                                               *mut wl_seat)
 -> *mut zwp_tablet_seat_v2 {
    let mut tablet_seat: *mut wl_proxy = 0 as *mut wl_proxy;
    tablet_seat =
        wl_proxy_marshal_constructor(zwp_tablet_manager_v2 as *mut wl_proxy,
                                     0i32 as uint32_t,
                                     &zwp_tablet_seat_v2_interface as
                                         *const wl_interface,
                                     0 as *mut libc::c_void, seat);
    return tablet_seat as *mut zwp_tablet_seat_v2;
}
#[inline]
unsafe extern "C" fn zwp_tablet_seat_v2_add_listener(mut zwp_tablet_seat_v2:
                                                         *mut zwp_tablet_seat_v2,
                                                     mut listener:
                                                         *const zwp_tablet_seat_v2_listener,
                                                     mut data:
                                                         *mut libc::c_void)
 -> libc::c_int {
    return wl_proxy_add_listener(zwp_tablet_seat_v2 as *mut wl_proxy,
                                 listener as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, data);
}
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 * a physical tool type
 *
 * Describes the physical type of a tool. The physical type of a tool
 * generally defines its base usage.
 *
 * The mouse tool represents a mouse-shaped tool that is not a relative
 * device but bound to the tablet's surface, providing absolute
 * coordinates.
 *
 * The lens tool is a mouse-shaped tool with an attached lens to
 * provide precision focus.
 */
/* *
	 * Pen
	 */
/* *
	 * Eraser
	 */
/* *
	 * Brush
	 */
/* *
	 * Pencil
	 */
/* *
	 * Airbrush
	 */
/* *
	 * Finger
	 */
/* *
	 * Mouse
	 */
/* *
	 * Lens
	 */
/* ZWP_TABLET_TOOL_V2_TYPE_ENUM */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 * capability flags for a tool
 *
 * Describes extra capabilities on a tablet.
 *
 * Any tool must provide x and y values, extra axes are
 * device-specific.
 */
/* *
	 * Tilt axes
	 */
/* *
	 * Pressure axis
	 */
/* *
	 * Distance axis
	 */
/* *
	 * Z-rotation axis
	 */
/* *
	 * Slider axis
	 */
/* *
	 * Wheel axis
	 */
/* ZWP_TABLET_TOOL_V2_CAPABILITY_ENUM */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 * physical button state
 *
 * Describes the physical state of a button that produced the button event.
 */
/* *
	 * button is not pressed
	 */
/* *
	 * button is pressed
	 */
/* ZWP_TABLET_TOOL_V2_BUTTON_STATE_ENUM */
/* *
	 * given wl_surface has another role
	 */
/* ZWP_TABLET_TOOL_V2_ERROR_ENUM */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 * @struct zwp_tablet_tool_v2_listener
 */
/* *
	 * tool type
	 *
	 * The tool type is the high-level type of the tool and usually
	 * decides the interaction expected from this tool.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet_tool.done event.
	 * @param tool_type the physical tool type
	 */
/* *
	 * unique hardware serial number of the tool
	 *
	 * If the physical tool can be identified by a unique 64-bit
	 * serial number, this event notifies the client of this serial
	 * number.
	 *
	 * If multiple tablets are available in the same seat and the tool
	 * is uniquely identifiable by the serial number, that tool may
	 * move between tablets.
	 *
	 * Otherwise, if the tool has no serial number and this event is
	 * missing, the tool is tied to the tablet it first comes into
	 * proximity with. Even if the physical tool is used on multiple
	 * tablets, separate wp_tablet_tool objects will be created, one
	 * per tablet.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet_tool.done event.
	 * @param hardware_serial_hi the unique serial number of the tool, most significant bits
	 * @param hardware_serial_lo the unique serial number of the tool, least significant bits
	 */
/* *
	 * hardware id notification in Wacom's format
	 *
	 * This event notifies the client of a hardware id available on
	 * this tool.
	 *
	 * The hardware id is a device-specific 64-bit id that provides
	 * extra information about the tool in use, beyond the wl_tool.type
	 * enumeration. The format of the id is specific to tablets made by
	 * Wacom Inc. For example, the hardware id of a Wacom Grip Pen (a
	 * stylus) is 0x802.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet_tool.done event.
	 * @param hardware_id_hi the hardware id, most significant bits
	 * @param hardware_id_lo the hardware id, least significant bits
	 */
/* *
	 * tool capability notification
	 *
	 * This event notifies the client of any capabilities of this
	 * tool, beyond the main set of x/y axes and tip up/down detection.
	 *
	 * One event is sent for each extra capability available on this
	 * tool.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet_tool.done event.
	 * @param capability the capability
	 */
/* *
	 * tool description events sequence complete
	 *
	 * This event signals the end of the initial burst of descriptive
	 * events. A client may consider the static description of the tool
	 * to be complete and finalize initialization of the tool.
	 */
/* *
	 * tool removed
	 *
	 * This event is sent when the tool is removed from the system
	 * and will send no further events. Should the physical tool come
	 * back into proximity later, a new wp_tablet_tool object will be
	 * created.
	 *
	 * It is compositor-dependent when a tool is removed. A compositor
	 * may remove a tool on proximity out, tablet removal or any other
	 * reason. A compositor may also keep a tool alive until shutdown.
	 *
	 * If the tool is currently in proximity, a proximity_out event
	 * will be sent before the removed event. See
	 * wp_tablet_tool.proximity_out for the handling of any buttons
	 * logically down.
	 *
	 * When this event is received, the client must
	 * wp_tablet_tool.destroy the object.
	 */
/* *
	 * proximity in event
	 *
	 * Notification that this tool is focused on a certain surface.
	 *
	 * This event can be received when the tool has moved from one
	 * surface to another, or when the tool has come back into
	 * proximity above the surface.
	 *
	 * If any button is logically down when the tool comes into
	 * proximity, the respective button event is sent after the
	 * proximity_in event but within the same frame as the proximity_in
	 * event.
	 * @param tablet The tablet the tool is in proximity of
	 * @param surface The current surface the tablet tool is over
	 */
/* *
	 * proximity out event
	 *
	 * Notification that this tool has either left proximity, or is
	 * no longer focused on a certain surface.
	 *
	 * When the tablet tool leaves proximity of the tablet, button
	 * release events are sent for each button that was held down at
	 * the time of leaving proximity. These events are sent before the
	 * proximity_out event but within the same wp_tablet.frame.
	 *
	 * If the tool stays within proximity of the tablet, but the focus
	 * changes from one surface to another, a button release event may
	 * not be sent until the button is actually released or the tool
	 * leaves the proximity of the tablet.
	 */
/* *
	 * tablet tool is making contact
	 *
	 * Sent whenever the tablet tool comes in contact with the
	 * surface of the tablet.
	 *
	 * If the tool is already in contact with the tablet when entering
	 * the input region, the client owning said region will receive a
	 * wp_tablet.proximity_in event, followed by a wp_tablet.down event
	 * and a wp_tablet.frame event.
	 *
	 * Note that this event describes logical contact, not physical
	 * contact. On some devices, a compositor may not consider a tool
	 * in logical contact until a minimum physical pressure threshold
	 * is exceeded.
	 */
/* *
	 * tablet tool is no longer making contact
	 *
	 * Sent whenever the tablet tool stops making contact with the
	 * surface of the tablet, or when the tablet tool moves out of the
	 * input region and the compositor grab (if any) is dismissed.
	 *
	 * If the tablet tool moves out of the input region while in
	 * contact with the surface of the tablet and the compositor does
	 * not have an ongoing grab on the surface, the client owning said
	 * region will receive a wp_tablet.up event, followed by a
	 * wp_tablet.proximity_out event and a wp_tablet.frame event. If
	 * the compositor has an ongoing grab on this device, this event
	 * sequence is sent whenever the grab is dismissed in the future.
	 *
	 * Note that this event describes logical contact, not physical
	 * contact. On some devices, a compositor may not consider a tool
	 * out of logical contact until physical pressure falls below a
	 * specific threshold.
	 */
/* *
	 * motion event
	 *
	 * Sent whenever a tablet tool moves.
	 * @param x surface-local x coordinate
	 * @param y surface-local y coordinate
	 */
/* *
	 * pressure change event
	 *
	 * Sent whenever the pressure axis on a tool changes. The value
	 * of this event is normalized to a value between 0 and 65535.
	 *
	 * Note that pressure may be nonzero even when a tool is not in
	 * logical contact. See the down and up events for more details.
	 * @param pressure The current pressure value
	 */
/* *
	 * distance change event
	 *
	 * Sent whenever the distance axis on a tool changes. The value
	 * of this event is normalized to a value between 0 and 65535.
	 *
	 * Note that distance may be nonzero even when a tool is not in
	 * logical contact. See the down and up events for more details.
	 * @param distance The current distance value
	 */
/* *
	 * tilt change event
	 *
	 * Sent whenever one or both of the tilt axes on a tool change.
	 * Each tilt value is in degrees, relative to the z-axis of the
	 * tablet. The angle is positive when the top of a tool tilts along
	 * the positive x or y axis.
	 * @param tilt_x The current value of the X tilt axis
	 * @param tilt_y The current value of the Y tilt axis
	 */
/* *
	 * z-rotation change event
	 *
	 * Sent whenever the z-rotation axis on the tool changes. The
	 * rotation value is in degrees clockwise from the tool's logical
	 * neutral position.
	 * @param degrees The current rotation of the Z axis
	 */
/* *
	 * Slider position change event
	 *
	 * Sent whenever the slider position on the tool changes. The
	 * value is normalized between -65535 and 65535, with 0 as the
	 * logical neutral position of the slider.
	 *
	 * The slider is available on e.g. the Wacom Airbrush tool.
	 * @param position The current position of slider
	 */
/* *
	 * Wheel delta event
	 *
	 * Sent whenever the wheel on the tool emits an event. This event
	 * contains two values for the same axis change. The degrees value
	 * is in the same orientation as the wl_pointer.vertical_scroll
	 * axis. The clicks value is in discrete logical clicks of the
	 * mouse wheel. This value may be zero if the movement of the wheel
	 * was less than one logical click.
	 *
	 * Clients should choose either value and avoid mixing degrees and
	 * clicks. The compositor may accumulate values smaller than a
	 * logical click and emulate click events when a certain threshold
	 * is met. Thus, wl_tablet_tool.wheel events with non-zero clicks
	 * values may have different degrees values.
	 * @param degrees The wheel delta in degrees
	 * @param clicks The wheel delta in discrete clicks
	 */
/* *
	 * button event
	 *
	 * Sent whenever a button on the tool is pressed or released.
	 *
	 * If a button is held down when the tool moves in or out of
	 * proximity, button events are generated by the compositor. See
	 * wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for
	 * details.
	 * @param button The button whose state has changed
	 * @param state Whether the button was pressed or released
	 */
/* *
	 * frame event
	 *
	 * Marks the end of a series of axis and/or button updates from
	 * the tablet. The Wayland protocol requires axis updates to be
	 * sent sequentially, however all events within a frame should be
	 * considered one hardware event.
	 * @param time The time of the event with millisecond granularity
	 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 */
/* * @ingroup iface_zwp_tablet_tool_v2 */
/* * @ingroup iface_zwp_tablet_tool_v2 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 *
 * Sets the surface of the cursor used for this tool on the given
 * tablet. This request only takes effect if the tool is in proximity
 * of one of the requesting client's surfaces or the surface parameter
 * is the current pointer surface. If there was a previous surface set
 * with this request it is replaced. If surface is NULL, the cursor
 * image is hidden.
 *
 * The parameters hotspot_x and hotspot_y define the position of the
 * pointer surface relative to the pointer location. Its top-left corner
 * is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
 * coordinates of the pointer location, in surface-local coordinates.
 *
 * On surface.attach requests to the pointer surface, hotspot_x and
 * hotspot_y are decremented by the x and y parameters passed to the
 * request. Attach must be confirmed by wl_surface.commit as usual.
 *
 * The hotspot can also be updated by passing the currently set pointer
 * surface to this request with new values for hotspot_x and hotspot_y.
 *
 * The current and pending input regions of the wl_surface are cleared,
 * and wl_surface.set_input_region is ignored until the wl_surface is no
 * longer used as the cursor. When the use as a cursor ends, the current
 * and pending input regions become undefined, and the wl_surface is
 * unmapped.
 *
 * This request gives the surface the role of a wp_tablet_tool cursor. A
 * surface may only ever be used as the cursor surface for one
 * wp_tablet_tool. If the surface already has another role or has
 * previously been used as cursor surface for a different tool, a
 * protocol error is raised.
 */
/* *
 * @ingroup iface_zwp_tablet_tool_v2
 *
 * This destroys the client's resource for this tool object.
 */
/* *
 * @ingroup iface_zwp_tablet_v2
 * @struct zwp_tablet_v2_listener
 */
/* *
	 * tablet device name
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet.done event.
	 * @param name the device name
	 */
/* *
	 * tablet device USB vendor/product id
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet.done event.
	 * @param vid USB vendor id
	 * @param pid USB product id
	 */
/* *
	 * path to the device
	 *
	 * A system-specific device path that indicates which device is
	 * behind this wp_tablet. This information may be used to gather
	 * additional information about the device, e.g. through libwacom.
	 *
	 * A device may have more than one device path. If so, multiple
	 * wp_tablet.path events are sent. A device may be emulated and not
	 * have a device path, and in that case this event will not be
	 * sent.
	 *
	 * The format of the path is unspecified, it may be a device node,
	 * a sysfs path, or some other identifier. It is up to the client
	 * to identify the string provided.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet.done event.
	 * @param path path to local device
	 */
/* *
	 * tablet description events sequence complete
	 *
	 * This event is sent immediately to signal the end of the
	 * initial burst of descriptive events. A client may consider the
	 * static description of the tablet to be complete and finalize
	 * initialization of the tablet.
	 */
/* *
	 * tablet removed event
	 *
	 * Sent when the tablet has been removed from the system. When a
	 * tablet is removed, some tools may be removed.
	 *
	 * When this event is received, the client must wp_tablet.destroy
	 * the object.
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
 */
/* *
 * @ingroup iface_zwp_tablet_v2
 */
/* * @ingroup iface_zwp_tablet_v2 */
/* * @ingroup iface_zwp_tablet_v2 */
/* *
 * @ingroup iface_zwp_tablet_v2
 *
 * This destroys the client's resource for this tablet object.
 */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 * ring axis source
 *
 * Describes the source types for ring events. This indicates to the
 * client how a ring event was physically generated; a client may
 * adjust the user interface accordingly. For example, events
 * from a "finger" source may trigger kinetic scrolling.
 */
/* *
	 * finger
	 */
/* ZWP_TABLET_PAD_RING_V2_SOURCE_ENUM */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 * @struct zwp_tablet_pad_ring_v2_listener
 */
/* *
	 * ring event source
	 *
	 * Source information for ring events.
	 *
	 * This event does not occur on its own. It is sent before a
	 * wp_tablet_pad_ring.frame event and carries the source
	 * information for all events within that frame.
	 *
	 * The source specifies how this event was generated. If the source
	 * is wp_tablet_pad_ring.source.finger, a wp_tablet_pad_ring.stop
	 * event will be sent when the user lifts the finger off the
	 * device.
	 *
	 * This event is optional. If the source is unknown for an
	 * interaction, no event is sent.
	 * @param source the event source
	 */
/* *
	 * angle changed
	 *
	 * Sent whenever the angle on a ring changes.
	 *
	 * The angle is provided in degrees clockwise from the logical
	 * north of the ring in the pad's current rotation.
	 * @param degrees the current angle in degrees
	 */
/* *
	 * interaction stopped
	 *
	 * Stop notification for ring events.
	 *
	 * For some wp_tablet_pad_ring.source types, a
	 * wp_tablet_pad_ring.stop event is sent to notify a client that
	 * the interaction with the ring has terminated. This enables the
	 * client to implement kinetic scrolling. See the
	 * wp_tablet_pad_ring.source documentation for information on when
	 * this event may be generated.
	 *
	 * Any wp_tablet_pad_ring.angle events with the same source after
	 * this event should be considered as the start of a new
	 * interaction.
	 */
/* *
	 * end of a ring event sequence
	 *
	 * Indicates the end of a set of ring events that logically
	 * belong together. A client is expected to accumulate the data in
	 * all events within the frame before proceeding.
	 *
	 * All wp_tablet_pad_ring events before a wp_tablet_pad_ring.frame
	 * event belong logically together. For example, on termination of
	 * a finger interaction on a ring the compositor will send a
	 * wp_tablet_pad_ring.source event, a wp_tablet_pad_ring.stop event
	 * and a wp_tablet_pad_ring.frame event.
	 *
	 * A wp_tablet_pad_ring.frame event is sent for every logical event
	 * group, even if the group only contains a single
	 * wp_tablet_pad_ring event. Specifically, a client may get a
	 * sequence: angle, frame, angle, frame, etc.
	 * @param time timestamp with millisecond granularity
	 */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 */
/* * @ingroup iface_zwp_tablet_pad_ring_v2 */
/* * @ingroup iface_zwp_tablet_pad_ring_v2 */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 *
 * Request that the compositor use the provided feedback string
 * associated with this ring. This request should be issued immediately
 * after a wp_tablet_pad_group.mode_switch event from the corresponding
 * group is received, or whenever the ring is mapped to a different
 * action. See wp_tablet_pad_group.mode_switch for more details.
 *
 * Clients are encouraged to provide context-aware descriptions for
 * the actions associated with the ring; compositors may use this
 * information to offer visual feedback about the button layout
 * (eg. on-screen displays).
 *
 * The provided string 'description' is a UTF-8 encoded string to be
 * associated with this ring, and is considered user-visible; general
 * internationalization rules apply.
 *
 * The serial argument will be that of the last
 * wp_tablet_pad_group.mode_switch event received for the group of this
 * ring. Requests providing other serials than the most recent one will be
 * ignored.
 */
/* *
 * @ingroup iface_zwp_tablet_pad_ring_v2
 *
 * This destroys the client's resource for this ring object.
 */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 * strip axis source
 *
 * Describes the source types for strip events. This indicates to the
 * client how a strip event was physically generated; a client may
 * adjust the user interface accordingly. For example, events
 * from a "finger" source may trigger kinetic scrolling.
 */
/* *
	 * finger
	 */
/* ZWP_TABLET_PAD_STRIP_V2_SOURCE_ENUM */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 * @struct zwp_tablet_pad_strip_v2_listener
 */
/* *
	 * strip event source
	 *
	 * Source information for strip events.
	 *
	 * This event does not occur on its own. It is sent before a
	 * wp_tablet_pad_strip.frame event and carries the source
	 * information for all events within that frame.
	 *
	 * The source specifies how this event was generated. If the source
	 * is wp_tablet_pad_strip.source.finger, a wp_tablet_pad_strip.stop
	 * event will be sent when the user lifts their finger off the
	 * device.
	 *
	 * This event is optional. If the source is unknown for an
	 * interaction, no event is sent.
	 * @param source the event source
	 */
/* *
	 * position changed
	 *
	 * Sent whenever the position on a strip changes.
	 *
	 * The position is normalized to a range of [0, 65535], the 0-value
	 * represents the top-most and/or left-most position of the strip
	 * in the pad's current rotation.
	 * @param position the current position
	 */
/* *
	 * interaction stopped
	 *
	 * Stop notification for strip events.
	 *
	 * For some wp_tablet_pad_strip.source types, a
	 * wp_tablet_pad_strip.stop event is sent to notify a client that
	 * the interaction with the strip has terminated. This enables the
	 * client to implement kinetic scrolling. See the
	 * wp_tablet_pad_strip.source documentation for information on when
	 * this event may be generated.
	 *
	 * Any wp_tablet_pad_strip.position events with the same source
	 * after this event should be considered as the start of a new
	 * interaction.
	 */
/* *
	 * end of a strip event sequence
	 *
	 * Indicates the end of a set of events that represent one
	 * logical hardware strip event. A client is expected to accumulate
	 * the data in all events within the frame before proceeding.
	 *
	 * All wp_tablet_pad_strip events before a
	 * wp_tablet_pad_strip.frame event belong logically together. For
	 * example, on termination of a finger interaction on a strip the
	 * compositor will send a wp_tablet_pad_strip.source event, a
	 * wp_tablet_pad_strip.stop event and a wp_tablet_pad_strip.frame
	 * event.
	 *
	 * A wp_tablet_pad_strip.frame event is sent for every logical
	 * event group, even if the group only contains a single
	 * wp_tablet_pad_strip event. Specifically, a client may get a
	 * sequence: position, frame, position, frame, etc.
	 * @param time timestamp with millisecond granularity
	 */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 */
/* * @ingroup iface_zwp_tablet_pad_strip_v2 */
/* * @ingroup iface_zwp_tablet_pad_strip_v2 */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 *
 * Requests the compositor to use the provided feedback string
 * associated with this strip. This request should be issued immediately
 * after a wp_tablet_pad_group.mode_switch event from the corresponding
 * group is received, or whenever the strip is mapped to a different
 * action. See wp_tablet_pad_group.mode_switch for more details.
 *
 * Clients are encouraged to provide context-aware descriptions for
 * the actions associated with the strip, and compositors may use this
 * information to offer visual feedback about the button layout
 * (eg. on-screen displays).
 *
 * The provided string 'description' is a UTF-8 encoded string to be
 * associated with this ring, and is considered user-visible; general
 * internationalization rules apply.
 *
 * The serial argument will be that of the last
 * wp_tablet_pad_group.mode_switch event received for the group of this
 * strip. Requests providing other serials than the most recent one will be
 * ignored.
 */
/* *
 * @ingroup iface_zwp_tablet_pad_strip_v2
 *
 * This destroys the client's resource for this strip object.
 */
/* *
 * @ingroup iface_zwp_tablet_pad_group_v2
 * @struct zwp_tablet_pad_group_v2_listener
 */
/* *
	 * buttons announced
	 *
	 * Sent on wp_tablet_pad_group initialization to announce the
	 * available buttons in the group. Button indices start at 0, a
	 * button may only be in one group at a time.
	 *
	 * This event is first sent in the initial burst of events before
	 * the wp_tablet_pad_group.done event.
	 *
	 * Some buttons are reserved by the compositor. These buttons may
	 * not be assigned to any wp_tablet_pad_group. Compositors may
	 * broadcast this event in the case of changes to the mapping of
	 * these reserved buttons. If the compositor happens to reserve all
	 * buttons in a group, this event will be sent with an empty array.
	 * @param buttons buttons in this group
	 */
/* *
	 * ring announced
	 *
	 * Sent on wp_tablet_pad_group initialization to announce
	 * available rings. One event is sent for each ring available on
	 * this pad group.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet_pad_group.done event.
	 */
/* *
	 * strip announced
	 *
	 * Sent on wp_tablet_pad initialization to announce available
	 * strips. One event is sent for each strip available on this pad
	 * group.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet_pad_group.done event.
	 */
/* *
	 * mode-switch ability announced
	 *
	 * Sent on wp_tablet_pad_group initialization to announce that
	 * the pad group may switch between modes. A client may use a mode
	 * to store a specific configuration for buttons, rings and strips
	 * and use the wl_tablet_pad_group.mode_switch event to toggle
	 * between these configurations. Mode indices start at 0.
	 *
	 * Switching modes is compositor-dependent. See the
	 * wp_tablet_pad_group.mode_switch event for more details.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet_pad_group.done event. This event is only sent when
	 * more than more than one mode is available.
	 * @param modes the number of modes
	 */
/* *
	 * tablet group description events sequence complete
	 *
	 * This event is sent immediately to signal the end of the
	 * initial burst of descriptive events. A client may consider the
	 * static description of the tablet to be complete and finalize
	 * initialization of the tablet group.
	 */
/* *
	 * mode switch event
	 *
	 * Notification that the mode was switched.
	 *
	 * A mode applies to all buttons, rings and strips in a group
	 * simultaneously, but a client is not required to assign different
	 * actions for each mode. For example, a client may have
	 * mode-specific button mappings but map the ring to vertical
	 * scrolling in all modes. Mode indices start at 0.
	 *
	 * Switching modes is compositor-dependent. The compositor may
	 * provide visual cues to the client about the mode, e.g. by
	 * toggling LEDs on the tablet device. Mode-switching may be
	 * software-controlled or controlled by one or more physical
	 * buttons. For example, on a Wacom Intuos Pro, the button inside
	 * the ring may be assigned to switch between modes.
	 *
	 * The compositor will also send this event after
	 * wp_tablet_pad.enter on each group in order to notify of the
	 * current mode. Groups that only feature one mode will use mode=0
	 * when emitting this event.
	 *
	 * If a button action in the new mode differs from the action in
	 * the previous mode, the client should immediately issue a
	 * wp_tablet_pad.set_feedback request for each changed button.
	 *
	 * If a ring or strip action in the new mode differs from the
	 * action in the previous mode, the client should immediately issue
	 * a wp_tablet_ring.set_feedback or wp_tablet_strip.set_feedback
	 * request for each changed ring or strip.
	 * @param time the time of the event with millisecond granularity
	 * @param mode the new mode of the pad
	 */
/* *
 * @ingroup iface_zwp_tablet_pad_group_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_group_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_group_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_group_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_group_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_group_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_group_v2
 */
/* *
 * @ingroup iface_zwp_tablet_pad_group_v2
 */
/* * @ingroup iface_zwp_tablet_pad_group_v2 */
/* * @ingroup iface_zwp_tablet_pad_group_v2 */
/* *
 * @ingroup iface_zwp_tablet_pad_group_v2
 *
 * Destroy the wp_tablet_pad_group object. Objects created from this object
 * are unaffected and should be destroyed separately.
 */
/* *
 * @ingroup iface_zwp_tablet_pad_v2
 * physical button state
 *
 * Describes the physical state of a button that caused the button
 * event.
 */
/* *
	 * the button is not pressed
	 */
/* *
	 * the button is pressed
	 */
/* ZWP_TABLET_PAD_V2_BUTTON_STATE_ENUM */
/* *
 * @ingroup iface_zwp_tablet_pad_v2
 * @struct zwp_tablet_pad_v2_listener
 */
/* *
	 * group announced
	 *
	 * Sent on wp_tablet_pad initialization to announce available
	 * groups. One event is sent for each pad group available.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet_pad.done event. At least one group will be announced.
	 */
/* *
	 * path to the device
	 *
	 * A system-specific device path that indicates which device is
	 * behind this wp_tablet_pad. This information may be used to
	 * gather additional information about the device, e.g. through
	 * libwacom.
	 *
	 * The format of the path is unspecified, it may be a device node,
	 * a sysfs path, or some other identifier. It is up to the client
	 * to identify the string provided.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet_pad.done event.
	 * @param path path to local device
	 */
/* *
	 * buttons announced
	 *
	 * Sent on wp_tablet_pad initialization to announce the available
	 * buttons.
	 *
	 * This event is sent in the initial burst of events before the
	 * wp_tablet_pad.done event. This event is only sent when at least
	 * one button is available.
	 * @param buttons the number of buttons
	 */
/* *
	 * pad description event sequence complete
	 *
	 * This event signals the end of the initial burst of descriptive
	 * events. A client may consider the static description of the pad
	 * to be complete and finalize initialization of the pad.
	 */
/* *
	 * physical button state
	 *
	 * Sent whenever the physical state of a button changes.
	 * @param time the time of the event with millisecond granularity
	 * @param button the index of the button that changed state
	 */
/* *
	 * enter event
	 *
	 * Notification that this pad is focused on the specified
	 * surface.
	 * @param serial serial number of the enter event
	 * @param tablet the tablet the pad is attached to
	 * @param surface surface the pad is focused on
	 */
/* *
	 * enter event
	 *
	 * Notification that this pad is no longer focused on the
	 * specified surface.
	 * @param serial serial number of the leave event
	 * @param surface surface the pad is no longer focused on
	 */
/* *
	 * pad removed event
	 *
	 * Sent when the pad has been removed from the system. When a
	 * tablet is removed its pad(s) will be removed too.
	 *
	 * When this event is received, the client must destroy all rings,
	 * strips and groups that were offered by this pad, and issue
	 * wp_tablet_pad.destroy the pad itself.
	 */
/* *
 * @ingroup iface_zwp_tablet_pad_v2
 */
#[inline]
unsafe extern "C" fn zwp_tablet_pad_v2_add_listener(mut zwp_tablet_pad_v2:
                                                        *mut zwp_tablet_pad_v2,
                                                    mut listener:
                                                        *const zwp_tablet_pad_v2_listener,
                                                    mut data:
                                                        *mut libc::c_void)
 -> libc::c_int {
    return wl_proxy_add_listener(zwp_tablet_pad_v2 as *mut wl_proxy,
                                 listener as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, data);
}
#[inline]
unsafe extern "C" fn zwp_tablet_tool_v2_destroy(mut zwp_tablet_tool_v2:
                                                    *mut zwp_tablet_tool_v2) {
    wl_proxy_marshal(zwp_tablet_tool_v2 as *mut wl_proxy, 1i32 as uint32_t);
    wl_proxy_destroy(zwp_tablet_tool_v2 as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn zwp_tablet_v2_add_listener(mut zwp_tablet_v2:
                                                    *mut zwp_tablet_v2,
                                                mut listener:
                                                    *const zwp_tablet_v2_listener,
                                                mut data: *mut libc::c_void)
 -> libc::c_int {
    return wl_proxy_add_listener(zwp_tablet_v2 as *mut wl_proxy,
                                 listener as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, data);
}
#[inline]
unsafe extern "C" fn zwp_tablet_v2_set_user_data(mut zwp_tablet_v2:
                                                     *mut zwp_tablet_v2,
                                                 mut user_data:
                                                     *mut libc::c_void) {
    wl_proxy_set_user_data(zwp_tablet_v2 as *mut wl_proxy, user_data);
}
#[inline]
unsafe extern "C" fn zwp_tablet_v2_get_user_data(mut zwp_tablet_v2:
                                                     *mut zwp_tablet_v2)
 -> *mut libc::c_void {
    return wl_proxy_get_user_data(zwp_tablet_v2 as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn zwp_tablet_v2_destroy(mut zwp_tablet_v2:
                                               *mut zwp_tablet_v2) {
    wl_proxy_marshal(zwp_tablet_v2 as *mut wl_proxy, 0i32 as uint32_t);
    wl_proxy_destroy(zwp_tablet_v2 as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn zwp_tablet_pad_ring_v2_add_listener(mut zwp_tablet_pad_ring_v2:
                                                             *mut zwp_tablet_pad_ring_v2,
                                                         mut listener:
                                                             *const zwp_tablet_pad_ring_v2_listener,
                                                         mut data:
                                                             *mut libc::c_void)
 -> libc::c_int {
    return wl_proxy_add_listener(zwp_tablet_pad_ring_v2 as *mut wl_proxy,
                                 listener as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, data);
}
#[inline]
unsafe extern "C" fn zwp_tablet_pad_ring_v2_destroy(mut zwp_tablet_pad_ring_v2:
                                                        *mut zwp_tablet_pad_ring_v2) {
    wl_proxy_marshal(zwp_tablet_pad_ring_v2 as *mut wl_proxy,
                     1i32 as uint32_t);
    wl_proxy_destroy(zwp_tablet_pad_ring_v2 as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn zwp_tablet_pad_strip_v2_add_listener(mut zwp_tablet_pad_strip_v2:
                                                              *mut zwp_tablet_pad_strip_v2,
                                                          mut listener:
                                                              *const zwp_tablet_pad_strip_v2_listener,
                                                          mut data:
                                                              *mut libc::c_void)
 -> libc::c_int {
    return wl_proxy_add_listener(zwp_tablet_pad_strip_v2 as *mut wl_proxy,
                                 listener as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, data);
}
#[inline]
unsafe extern "C" fn zwp_tablet_pad_strip_v2_destroy(mut zwp_tablet_pad_strip_v2:
                                                         *mut zwp_tablet_pad_strip_v2) {
    wl_proxy_marshal(zwp_tablet_pad_strip_v2 as *mut wl_proxy,
                     1i32 as uint32_t);
    wl_proxy_destroy(zwp_tablet_pad_strip_v2 as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn zwp_tablet_pad_group_v2_add_listener(mut zwp_tablet_pad_group_v2:
                                                              *mut zwp_tablet_pad_group_v2,
                                                          mut listener:
                                                              *const zwp_tablet_pad_group_v2_listener,
                                                          mut data:
                                                              *mut libc::c_void)
 -> libc::c_int {
    return wl_proxy_add_listener(zwp_tablet_pad_group_v2 as *mut wl_proxy,
                                 listener as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, data);
}
#[inline]
unsafe extern "C" fn zwp_tablet_pad_group_v2_destroy(mut zwp_tablet_pad_group_v2:
                                                         *mut zwp_tablet_pad_group_v2) {
    wl_proxy_marshal(zwp_tablet_pad_group_v2 as *mut wl_proxy,
                     0i32 as uint32_t);
    wl_proxy_destroy(zwp_tablet_pad_group_v2 as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn zwp_tablet_tool_v2_add_listener(mut zwp_tablet_tool_v2:
                                                         *mut zwp_tablet_tool_v2,
                                                     mut listener:
                                                         *const zwp_tablet_tool_v2_listener,
                                                     mut data:
                                                         *mut libc::c_void)
 -> libc::c_int {
    return wl_proxy_add_listener(zwp_tablet_tool_v2 as *mut wl_proxy,
                                 listener as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, data);
}
/* *
 * @ingroup iface_zwp_tablet_pad_v2
 *
 * Destroy the wp_tablet_pad object. Objects created from this object
 * are unaffected and should be destroyed separately.
 */
#[inline]
unsafe extern "C" fn zwp_tablet_pad_v2_destroy(mut zwp_tablet_pad_v2:
                                                   *mut zwp_tablet_pad_v2) {
    wl_proxy_marshal(zwp_tablet_pad_v2 as *mut wl_proxy, 1i32 as uint32_t);
    wl_proxy_destroy(zwp_tablet_pad_v2 as *mut wl_proxy);
}
// wlr_wl_tablet_pad_strips::link
unsafe extern "C" fn get_current_time_msec() -> uint32_t {
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    clock_gettime(1i32, &mut now);
    return (now.tv_nsec / (1000i32 * 1000i32) as libc::c_long +
                now.tv_sec * 1000i32 as libc::c_long) as uint32_t;
}
unsafe extern "C" fn handle_tablet_pad_ring_source(mut data:
                                                       *mut libc::c_void,
                                                   mut zwp_tablet_pad_ring_v2:
                                                       *mut zwp_tablet_pad_ring_v2,
                                                   mut source: uint32_t) {
    let mut ring: *mut wlr_wl_tablet_pad_ring =
        data as *mut wlr_wl_tablet_pad_ring;
    (*ring).source = source as wlr_tablet_pad_ring_source;
}
unsafe extern "C" fn handle_tablet_pad_ring_angle(mut data: *mut libc::c_void,
                                                  mut zwp_tablet_pad_ring_v2:
                                                      *mut zwp_tablet_pad_ring_v2,
                                                  mut degrees: wl_fixed_t) {
    let mut ring: *mut wlr_wl_tablet_pad_ring =
        data as *mut wlr_wl_tablet_pad_ring;
    (*ring).angle = wl_fixed_to_double(degrees);
}
unsafe extern "C" fn handle_tablet_pad_ring_stop(mut data: *mut libc::c_void,
                                                 mut zwp_tablet_pad_ring_v2:
                                                     *mut zwp_tablet_pad_ring_v2) {
    let mut ring: *mut wlr_wl_tablet_pad_ring =
        data as *mut wlr_wl_tablet_pad_ring;
    (*ring).stopped = 1i32 != 0;
}
unsafe extern "C" fn handle_tablet_pad_ring_frame(mut data: *mut libc::c_void,
                                                  mut zwp_tablet_pad_ring_v2:
                                                      *mut zwp_tablet_pad_ring_v2,
                                                  mut time: uint32_t) {
    let mut ring: *mut wlr_wl_tablet_pad_ring =
        data as *mut wlr_wl_tablet_pad_ring;
    let mut evt: wlr_event_tablet_pad_ring =
        {
            let mut init =
                wlr_event_tablet_pad_ring{time_msec: time,
                                          source: (*ring).source,
                                          ring: (*ring).index as uint32_t,
                                          position: (*ring).angle,
                                          mode: (*(*ring).group).mode,};
            init
        };
    if (*ring).angle >= 0i32 as libc::c_double {
        wlr_signal_emit_safe(&mut (*(*(*ring).group).pad).events.ring,
                             &mut evt as *mut wlr_event_tablet_pad_ring as
                                 *mut libc::c_void);
    }
    if (*ring).stopped {
        evt.position = -1i32 as libc::c_double;
        wlr_signal_emit_safe(&mut (*(*(*ring).group).pad).events.ring,
                             &mut evt as *mut wlr_event_tablet_pad_ring as
                                 *mut libc::c_void);
    }
    (*ring).angle = -1i32 as libc::c_double;
    (*ring).stopped = 0i32 != 0;
    (*ring).source = WLR_TABLET_PAD_RING_SOURCE_UNKNOWN;
}
static mut tablet_pad_ring_listener: zwp_tablet_pad_ring_v2_listener =
    {
    
        {
            let mut init =
                zwp_tablet_pad_ring_v2_listener{source:
                                                    Some(handle_tablet_pad_ring_source
                                                             as
                                                             unsafe extern "C" fn(_:
                                                                                      *mut libc::c_void,
                                                                                  _:
                                                                                      *mut zwp_tablet_pad_ring_v2,
                                                                                  _:
                                                                                      uint32_t)
                                                                 -> ()),
                                                angle:
                                                    Some(handle_tablet_pad_ring_angle
                                                             as
                                                             unsafe extern "C" fn(_:
                                                                                      *mut libc::c_void,
                                                                                  _:
                                                                                      *mut zwp_tablet_pad_ring_v2,
                                                                                  _:
                                                                                      wl_fixed_t)
                                                                 -> ()),
                                                stop:
                                                    Some(handle_tablet_pad_ring_stop
                                                             as
                                                             unsafe extern "C" fn(_:
                                                                                      *mut libc::c_void,
                                                                                  _:
                                                                                      *mut zwp_tablet_pad_ring_v2)
                                                                 -> ()),
                                                frame:
                                                    Some(handle_tablet_pad_ring_frame
                                                             as
                                                             unsafe extern "C" fn(_:
                                                                                      *mut libc::c_void,
                                                                                  _:
                                                                                      *mut zwp_tablet_pad_ring_v2,
                                                                                  _:
                                                                                      uint32_t)
                                                                 -> ()),};
            init
        }
};
unsafe extern "C" fn handle_tablet_pad_strip_source(mut data:
                                                        *mut libc::c_void,
                                                    mut zwp_tablet_pad_strip_v2:
                                                        *mut zwp_tablet_pad_strip_v2,
                                                    mut source: uint32_t) {
    let mut strip: *mut wlr_wl_tablet_pad_strip =
        data as *mut wlr_wl_tablet_pad_strip;
    (*strip).source = source as wlr_tablet_pad_strip_source;
}
unsafe extern "C" fn handle_tablet_pad_strip_position(mut data:
                                                          *mut libc::c_void,
                                                      mut zwp_tablet_pad_strip_v2:
                                                          *mut zwp_tablet_pad_strip_v2,
                                                      mut position:
                                                          uint32_t) {
    let mut strip: *mut wlr_wl_tablet_pad_strip =
        data as *mut wlr_wl_tablet_pad_strip;
    (*strip).position = position as libc::c_double / 65536.0f64;
}
unsafe extern "C" fn handle_tablet_pad_strip_stop(mut data: *mut libc::c_void,
                                                  mut zwp_tablet_pad_strip_v2:
                                                      *mut zwp_tablet_pad_strip_v2) {
    let mut strip: *mut wlr_wl_tablet_pad_strip =
        data as *mut wlr_wl_tablet_pad_strip;
    (*strip).stopped = 1i32 != 0;
}
unsafe extern "C" fn handle_tablet_pad_strip_frame(mut data:
                                                       *mut libc::c_void,
                                                   mut zwp_tablet_pad_strip_v2:
                                                       *mut zwp_tablet_pad_strip_v2,
                                                   mut time: uint32_t) {
    let mut strip: *mut wlr_wl_tablet_pad_strip =
        data as *mut wlr_wl_tablet_pad_strip;
    let mut evt: wlr_event_tablet_pad_strip =
        {
            let mut init =
                wlr_event_tablet_pad_strip{time_msec: time,
                                           source: (*strip).source,
                                           strip: (*strip).index as uint32_t,
                                           position: (*strip).position,
                                           mode: (*(*strip).group).mode,};
            init
        };
    if (*strip).position >= 0i32 as libc::c_double {
        wlr_signal_emit_safe(&mut (*(*(*strip).group).pad).events.strip,
                             &mut evt as *mut wlr_event_tablet_pad_strip as
                                 *mut libc::c_void);
    }
    if (*strip).stopped {
        evt.position = -1i32 as libc::c_double;
        wlr_signal_emit_safe(&mut (*(*(*strip).group).pad).events.strip,
                             &mut evt as *mut wlr_event_tablet_pad_strip as
                                 *mut libc::c_void);
    }
    (*strip).position = -1i32 as libc::c_double;
    (*strip).stopped = 0i32 != 0;
    (*strip).source = WLR_TABLET_PAD_STRIP_SOURCE_UNKNOWN;
}
static mut tablet_pad_strip_listener: zwp_tablet_pad_strip_v2_listener =
    {
    
        {
            let mut init =
                zwp_tablet_pad_strip_v2_listener{source:
                                                     Some(handle_tablet_pad_strip_source
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut libc::c_void,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_strip_v2,
                                                                                   _:
                                                                                       uint32_t)
                                                                  -> ()),
                                                 position:
                                                     Some(handle_tablet_pad_strip_position
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut libc::c_void,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_strip_v2,
                                                                                   _:
                                                                                       uint32_t)
                                                                  -> ()),
                                                 stop:
                                                     Some(handle_tablet_pad_strip_stop
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut libc::c_void,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_strip_v2)
                                                                  -> ()),
                                                 frame:
                                                     Some(handle_tablet_pad_strip_frame
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut libc::c_void,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_strip_v2,
                                                                                   _:
                                                                                       uint32_t)
                                                                  -> ()),};
            init
        }
};
unsafe extern "C" fn handle_tablet_pad_group_buttons(mut data:
                                                         *mut libc::c_void,
                                                     mut pad_group:
                                                         *mut zwp_tablet_pad_group_v2,
                                                     mut buttons:
                                                         *mut wl_array) {
    let mut group: *mut wlr_wl_tablet_pad_group =
        data as *mut wlr_wl_tablet_pad_group;
    free((*group).group.buttons as *mut libc::c_void);
    (*group).group.buttons =
        calloc(1i32 as libc::c_ulong, (*buttons).size) as *mut libc::c_uint;
    if (*group).group.buttons.is_null() {
        // FIXME: Add actual error handling
        return
    }
    (*group).group.button_count =
        (*buttons).size.wrapping_div(::std::mem::size_of::<libc::c_int>() as
                                         libc::c_ulong);
    memcpy((*group).group.buttons as *mut libc::c_void, (*buttons).data,
           (*buttons).size);
}
unsafe extern "C" fn handle_tablet_pad_group_modes(mut data:
                                                       *mut libc::c_void,
                                                   mut pad_group:
                                                       *mut zwp_tablet_pad_group_v2,
                                                   mut modes: uint32_t) {
    let mut group: *mut wlr_wl_tablet_pad_group =
        data as *mut wlr_wl_tablet_pad_group;
    (*group).group.mode_count = modes;
}
unsafe extern "C" fn handle_tablet_pad_group_ring(mut data: *mut libc::c_void,
                                                  mut pad_group:
                                                      *mut zwp_tablet_pad_group_v2,
                                                  mut ring:
                                                      *mut zwp_tablet_pad_ring_v2) {
    let mut group: *mut wlr_wl_tablet_pad_group =
        data as *mut wlr_wl_tablet_pad_group;
    let mut tablet_ring: *mut wlr_wl_tablet_pad_ring =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_wl_tablet_pad_ring>() as
                   libc::c_ulong) as *mut wlr_wl_tablet_pad_ring;
    if tablet_ring.is_null() { zwp_tablet_pad_ring_v2_destroy(ring); return }
    let fresh0 = (*(*group).pad).ring_count;
    (*(*group).pad).ring_count = (*(*group).pad).ring_count.wrapping_add(1);
    (*tablet_ring).index = fresh0;
    (*tablet_ring).group = group;
    zwp_tablet_pad_ring_v2_add_listener(ring, &tablet_pad_ring_listener,
                                        tablet_ring as *mut libc::c_void);
    (*group).group.ring_count = (*group).group.ring_count.wrapping_add(1);
    (*group).group.rings =
        realloc((*group).group.rings as *mut libc::c_void,
                (*group).group.ring_count.wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                           as libc::c_ulong))
            as *mut libc::c_uint;
    *(*group).group.rings.offset((*group).group.ring_count.wrapping_sub(1i32
                                                                            as
                                                                            libc::c_ulong)
                                     as isize) =
        (*tablet_ring).index as libc::c_uint;
}
unsafe extern "C" fn handle_tablet_pad_group_strip(mut data:
                                                       *mut libc::c_void,
                                                   mut pad_group:
                                                       *mut zwp_tablet_pad_group_v2,
                                                   mut strip:
                                                       *mut zwp_tablet_pad_strip_v2) {
    let mut group: *mut wlr_wl_tablet_pad_group =
        data as *mut wlr_wl_tablet_pad_group;
    let mut tablet_strip: *mut wlr_wl_tablet_pad_strip =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_wl_tablet_pad_strip>() as
                   libc::c_ulong) as *mut wlr_wl_tablet_pad_strip;
    if tablet_strip.is_null() {
        zwp_tablet_pad_strip_v2_destroy(strip);
        return
    }
    let fresh1 = (*(*group).pad).strip_count;
    (*(*group).pad).strip_count = (*(*group).pad).strip_count.wrapping_add(1);
    (*tablet_strip).index = fresh1;
    (*tablet_strip).group = group;
    zwp_tablet_pad_strip_v2_add_listener(strip, &tablet_pad_strip_listener,
                                         tablet_strip as *mut libc::c_void);
    (*group).group.strip_count = (*group).group.strip_count.wrapping_add(1);
    (*group).group.strips =
        realloc((*group).group.strips as *mut libc::c_void,
                (*group).group.strip_count.wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                            as libc::c_ulong))
            as *mut libc::c_uint;
    *(*group).group.strips.offset((*group).group.strip_count.wrapping_sub(1i32
                                                                              as
                                                                              libc::c_ulong)
                                      as isize) =
        (*tablet_strip).index as libc::c_uint;
}
unsafe extern "C" fn handle_tablet_pad_group_done(mut data: *mut libc::c_void,
                                                  mut pad_group:
                                                      *mut zwp_tablet_pad_group_v2) {
    /* Empty for now */
}
unsafe extern "C" fn handle_tablet_pad_group_mode_switch(mut data:
                                                             *mut libc::c_void,
                                                         mut pad_group:
                                                             *mut zwp_tablet_pad_group_v2,
                                                         mut time: uint32_t,
                                                         mut serial: uint32_t,
                                                         mut mode: uint32_t) {
    let mut group: *mut wlr_wl_tablet_pad_group =
        data as *mut wlr_wl_tablet_pad_group;
    (*group).mode = mode;
}
/* This isn't in the listener, but keep the naming scheme around since the
 * other removed functions work like this, and pad sub-resources are just a bit
 * special */
unsafe extern "C" fn handle_tablet_pad_group_removed(mut group:
                                                         *mut wlr_wl_tablet_pad_group) {
    /* No need to remove the ::link on strips rings as long as we do *not*
	 * wl_list_remove on the wl_groups ring/strip attributes here */
    let mut ring: *mut wlr_wl_tablet_pad_ring =
        0 as *mut wlr_wl_tablet_pad_ring;
    let mut tmp_ring: *mut wlr_wl_tablet_pad_ring =
        0 as *mut wlr_wl_tablet_pad_ring;
    ring =
        ((*group).rings.next as *mut libc::c_char).offset(-0) as
            *mut wlr_wl_tablet_pad_ring;
    tmp_ring =
        ((*ring).link.next as *mut libc::c_char).offset(-0) as
            *mut wlr_wl_tablet_pad_ring;
    while &mut (*ring).link as *mut wl_list !=
              &mut (*group).rings as *mut wl_list {
        zwp_tablet_pad_ring_v2_destroy((*ring).ring);
        free(ring as *mut libc::c_void);
        ring = tmp_ring;
        tmp_ring =
            ((*ring).link.next as *mut libc::c_char).offset(-0) as
                *mut wlr_wl_tablet_pad_ring
    }
    let mut strip: *mut wlr_wl_tablet_pad_strip =
        0 as *mut wlr_wl_tablet_pad_strip;
    let mut tmp_strip: *mut wlr_wl_tablet_pad_strip =
        0 as *mut wlr_wl_tablet_pad_strip;
    strip =
        ((*group).strips.next as *mut libc::c_char).offset(-0) as
            *mut wlr_wl_tablet_pad_strip;
    tmp_strip =
        ((*strip).link.next as *mut libc::c_char).offset(-0) as
            *mut wlr_wl_tablet_pad_strip;
    while &mut (*strip).link as *mut wl_list !=
              &mut (*group).strips as *mut wl_list {
        zwp_tablet_pad_strip_v2_destroy((*strip).strip);
        free(strip as *mut libc::c_void);
        strip = tmp_strip;
        tmp_strip =
            ((*strip).link.next as *mut libc::c_char).offset(-0) as
                *mut wlr_wl_tablet_pad_strip
    }
    zwp_tablet_pad_group_v2_destroy((*group).pad_group);
    /* While I'm pretty sure we have control over this as well, it's
	 * outside the scope of a single function, so better be safe than
	 * sorry */
    wl_list_remove(&mut (*group).group.link);
    free(group as *mut libc::c_void);
}
static mut tablet_pad_group_listener: zwp_tablet_pad_group_v2_listener =
    {
    
        {
            let mut init =
                zwp_tablet_pad_group_v2_listener{buttons:
                                                     Some(handle_tablet_pad_group_buttons
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut libc::c_void,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_group_v2,
                                                                                   _:
                                                                                       *mut wl_array)
                                                                  -> ()),
                                                 ring:
                                                     Some(handle_tablet_pad_group_ring
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut libc::c_void,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_group_v2,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_ring_v2)
                                                                  -> ()),
                                                 strip:
                                                     Some(handle_tablet_pad_group_strip
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut libc::c_void,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_group_v2,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_strip_v2)
                                                                  -> ()),
                                                 modes:
                                                     Some(handle_tablet_pad_group_modes
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut libc::c_void,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_group_v2,
                                                                                   _:
                                                                                       uint32_t)
                                                                  -> ()),
                                                 done:
                                                     Some(handle_tablet_pad_group_done
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut libc::c_void,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_group_v2)
                                                                  -> ()),
                                                 mode_switch:
                                                     Some(handle_tablet_pad_group_mode_switch
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut libc::c_void,
                                                                                   _:
                                                                                       *mut zwp_tablet_pad_group_v2,
                                                                                   _:
                                                                                       uint32_t,
                                                                                   _:
                                                                                       uint32_t,
                                                                                   _:
                                                                                       uint32_t)
                                                                  -> ()),};
            init
        }
};
unsafe extern "C" fn handle_tablet_pad_group(mut data: *mut libc::c_void,
                                             mut zwp_tablet_pad:
                                                 *mut zwp_tablet_pad_v2,
                                             mut pad_group:
                                                 *mut zwp_tablet_pad_group_v2) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    let mut pad: *mut wlr_tablet_pad =
        (*dev).wlr_input_device.c2rust_unnamed.tablet_pad;
    let mut group: *mut wlr_wl_tablet_pad_group =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_wl_tablet_pad_group>() as
                   libc::c_ulong) as *mut wlr_wl_tablet_pad_group;
    if group.is_null() { zwp_tablet_pad_group_v2_destroy(pad_group); return }
    (*group).pad_group = pad_group;
    (*group).pad = pad;
    wl_list_init(&mut (*group).rings);
    wl_list_init(&mut (*group).strips);
    zwp_tablet_pad_group_v2_add_listener(pad_group,
                                         &tablet_pad_group_listener,
                                         group as *mut libc::c_void);
    wl_list_insert(&mut (*pad).groups, &mut (*group).group.link);
}
unsafe extern "C" fn handle_tablet_pad_path(mut data: *mut libc::c_void,
                                            mut zwp_tablet_pad_v2:
                                                *mut zwp_tablet_pad_v2,
                                            mut path: *const libc::c_char) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    let mut tablet_pad: *mut wlr_tablet_pad =
        (*dev).wlr_input_device.c2rust_unnamed.tablet_pad;
    wlr_list_push(&mut (*tablet_pad).paths,
                  strdup(path) as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_pad_buttons(mut data: *mut libc::c_void,
                                               mut zwp_tablet_pad_v2:
                                                   *mut zwp_tablet_pad_v2,
                                               mut buttons: uint32_t) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    let mut tablet_pad: *mut wlr_tablet_pad =
        (*dev).wlr_input_device.c2rust_unnamed.tablet_pad;
    (*tablet_pad).button_count = buttons as size_t;
}
unsafe extern "C" fn handle_tablet_pad_button(mut data: *mut libc::c_void,
                                              mut zwp_tablet_pad_v2:
                                                  *mut zwp_tablet_pad_v2,
                                              mut time: uint32_t,
                                              mut button: uint32_t,
                                              mut state: uint32_t) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    let mut tablet_pad: *mut wlr_tablet_pad =
        (*dev).wlr_input_device.c2rust_unnamed.tablet_pad;
    let mut evt: wlr_event_tablet_pad_button =
        {
            let mut init =
                wlr_event_tablet_pad_button{time_msec: time,
                                            button: button,
                                            state: state as wlr_button_state,
                                            mode: 0i32 as libc::c_uint,
                                            group: 0i32 as libc::c_uint,};
            init
        };
    wlr_signal_emit_safe(&mut (*tablet_pad).events.button,
                         &mut evt as *mut wlr_event_tablet_pad_button as
                             *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_pad_done(mut data: *mut libc::c_void,
                                            mut zwp_tablet_pad_v2:
                                                *mut zwp_tablet_pad_v2) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    wlr_signal_emit_safe(&mut (*(*dev).backend).backend.events.new_input,
                         &mut (*dev).wlr_input_device as *mut wlr_input_device
                             as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_pad_enter(mut data: *mut libc::c_void,
                                             mut zwp_tablet_pad_v2:
                                                 *mut zwp_tablet_pad_v2,
                                             mut serial: uint32_t,
                                             mut tablet_p: *mut zwp_tablet_v2,
                                             mut surface: *mut wl_surface) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    let mut tablet_pad: *mut wlr_tablet_pad =
        (*dev).wlr_input_device.c2rust_unnamed.tablet_pad;
    let mut tab_dev: *mut wlr_wl_input_device =
        zwp_tablet_v2_get_user_data(tablet_p) as *mut wlr_wl_input_device;
    let mut tablet: *mut wlr_input_device = &mut (*tab_dev).wlr_input_device;
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Tablet: %p\n\x00" as *const u8 as *const libc::c_char,
             b"../backend/wayland/tablet_v2.c\x00" as *const u8 as
                 *const libc::c_char, 388i32, tablet);
    wlr_signal_emit_safe(&mut (*tablet_pad).events.attach_tablet,
                         tablet as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_pad_leave(mut data: *mut libc::c_void,
                                             mut zwp_tablet_pad_v2:
                                                 *mut zwp_tablet_pad_v2,
                                             mut serial: uint32_t,
                                             mut surface: *mut wl_surface) {
    /* Empty. Probably staying that way, unless we want to create/destroy
	 * tablet on enter/leave events (ehh) */
}
unsafe extern "C" fn handle_tablet_pad_removed(mut data: *mut libc::c_void,
                                               mut zwp_tablet_pad_v2:
                                                   *mut zwp_tablet_pad_v2) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    let mut tablet_pad: *mut wlr_tablet_pad =
        (*dev).wlr_input_device.c2rust_unnamed.tablet_pad;
    /* This doesn't free anything, but emits the destroy signal */
    wlr_input_device_destroy(&mut (*dev).wlr_input_device);
    /* This is a bit ugly, but we need to remove it from our list */
    wl_list_remove(&mut (*dev).wlr_input_device.link);
    let mut group: *mut wlr_wl_tablet_pad_group =
        0 as *mut wlr_wl_tablet_pad_group;
    let mut it: *mut wlr_wl_tablet_pad_group =
        0 as *mut wlr_wl_tablet_pad_group;
    group =
        ((*tablet_pad).groups.next as *mut libc::c_char).offset(-24) as
            *mut wlr_wl_tablet_pad_group;
    it =
        ((*group).group.link.next as *mut libc::c_char).offset(-24) as
            *mut wlr_wl_tablet_pad_group;
    while &mut (*group).group.link as *mut wl_list !=
              &mut (*tablet_pad).groups as *mut wl_list {
        handle_tablet_pad_group_removed(group);
        group = it;
        it =
            ((*group).group.link.next as *mut libc::c_char).offset(-24) as
                *mut wlr_wl_tablet_pad_group
    }
    /* This frees */
    wlr_tablet_pad_destroy(tablet_pad);
    zwp_tablet_pad_v2_destroy((*dev).resource as *mut zwp_tablet_pad_v2);
    free(dev as *mut libc::c_void);
}
static mut tablet_pad_listener: zwp_tablet_pad_v2_listener =
    {
    
        {
            let mut init =
                zwp_tablet_pad_v2_listener{group:
                                               Some(handle_tablet_pad_group as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 *mut zwp_tablet_pad_v2,
                                                                             _:
                                                                                 *mut zwp_tablet_pad_group_v2)
                                                            -> ()),
                                           path:
                                               Some(handle_tablet_pad_path as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 *mut zwp_tablet_pad_v2,
                                                                             _:
                                                                                 *const libc::c_char)
                                                            -> ()),
                                           buttons:
                                               Some(handle_tablet_pad_buttons
                                                        as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 *mut zwp_tablet_pad_v2,
                                                                             _:
                                                                                 uint32_t)
                                                            -> ()),
                                           done:
                                               Some(handle_tablet_pad_done as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 *mut zwp_tablet_pad_v2)
                                                            -> ()),
                                           button:
                                               Some(handle_tablet_pad_button
                                                        as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 *mut zwp_tablet_pad_v2,
                                                                             _:
                                                                                 uint32_t,
                                                                             _:
                                                                                 uint32_t,
                                                                             _:
                                                                                 uint32_t)
                                                            -> ()),
                                           enter:
                                               Some(handle_tablet_pad_enter as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 *mut zwp_tablet_pad_v2,
                                                                             _:
                                                                                 uint32_t,
                                                                             _:
                                                                                 *mut zwp_tablet_v2,
                                                                             _:
                                                                                 *mut wl_surface)
                                                            -> ()),
                                           leave:
                                               Some(handle_tablet_pad_leave as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 *mut zwp_tablet_pad_v2,
                                                                             _:
                                                                                 uint32_t,
                                                                             _:
                                                                                 *mut wl_surface)
                                                            -> ()),
                                           removed:
                                               Some(handle_tablet_pad_removed
                                                        as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 *mut zwp_tablet_pad_v2)
                                                            -> ()),};
            init
        }
};
unsafe extern "C" fn handle_pad_added(mut data: *mut libc::c_void,
                                      mut zwp_tablet_seat_v2:
                                          *mut zwp_tablet_seat_v2,
                                      mut id: *mut zwp_tablet_pad_v2) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] New tablet pad\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/wayland/tablet_v2.c\x00" as *const u8 as
                 *const libc::c_char, 435i32);
    let mut backend: *mut wlr_wl_backend = data as *mut wlr_wl_backend;
    let mut dev: *mut wlr_wl_input_device =
        create_wl_input_device(backend, WLR_INPUT_DEVICE_TABLET_PAD);
    if dev.is_null() {
        /* This leaks a couple of server-sent resource ids. iirc this
		 * shouldn't ever be a problem, but it isn't exactly nice
		 * either. */
        zwp_tablet_pad_v2_destroy(id);
        return
    }
    (*dev).resource = id as *mut libc::c_void;
    let mut wlr_dev: *mut wlr_input_device = &mut (*dev).wlr_input_device;
    (*wlr_dev).c2rust_unnamed.tablet_pad =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_tablet_pad>() as libc::c_ulong) as
            *mut wlr_tablet_pad;
    if (*wlr_dev).c2rust_unnamed.tablet_pad.is_null() {
        /* This leaks a couple of server-sent resource ids. iirc this
		 * shouldn't ever be a problem, but it isn't exactly nice
		 * either. */
        free(dev as *mut libc::c_void);
        zwp_tablet_pad_v2_destroy(id);
        return
    }
    wlr_tablet_pad_init((*wlr_dev).c2rust_unnamed.tablet_pad,
                        0 as *mut wlr_tablet_pad_impl);
    zwp_tablet_pad_v2_add_listener(id, &tablet_pad_listener,
                                   dev as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_tool_done(mut data: *mut libc::c_void,
                                             mut id:
                                                 *mut zwp_tablet_tool_v2) {
    /* empty */
}
unsafe extern "C" fn tablet_type_to_wlr_type(mut type_0:
                                                 zwp_tablet_tool_v2_type)
 -> wlr_tablet_tool_type {
    match type_0 as libc::c_uint {
        320 => { return WLR_TABLET_TOOL_TYPE_PEN }
        321 => { return WLR_TABLET_TOOL_TYPE_ERASER }
        322 => { return WLR_TABLET_TOOL_TYPE_BRUSH }
        323 => { return WLR_TABLET_TOOL_TYPE_PENCIL }
        324 => { return WLR_TABLET_TOOL_TYPE_AIRBRUSH }
        326 => { return WLR_TABLET_TOOL_TYPE_MOUSE }
        327 => { return WLR_TABLET_TOOL_TYPE_LENS }
        _ => { }
    }
    if 0i32 != 0 &&
           !(b"Unreachable\x00" as *const u8 as *const libc::c_char).is_null()
       {
    } else {
        __assert_fail(b"false && \"Unreachable\"\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/wayland/tablet_v2.c\x00" as *const u8 as
                          *const libc::c_char, 489i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"enum wlr_tablet_tool_type tablet_type_to_wlr_type(enum zwp_tablet_tool_v2_type)\x00")).as_ptr());
    };
      panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn handle_tablet_tool_type(mut data: *mut libc::c_void,
                                             mut id: *mut zwp_tablet_tool_v2,
                                             mut tool_type: uint32_t) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).wlr_tool.type_0 =
        tablet_type_to_wlr_type(tool_type as zwp_tablet_tool_v2_type);
}
unsafe extern "C" fn handle_tablet_tool_serial(mut data: *mut libc::c_void,
                                               mut id:
                                                   *mut zwp_tablet_tool_v2,
                                               mut high: uint32_t,
                                               mut low: uint32_t) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).wlr_tool.hardware_serial =
        (high as uint64_t) << 32i32 | low as uint64_t;
}
unsafe extern "C" fn handle_tablet_tool_id_wacom(mut data: *mut libc::c_void,
                                                 mut id:
                                                     *mut zwp_tablet_tool_v2,
                                                 mut high: uint32_t,
                                                 mut low: uint32_t) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).wlr_tool.hardware_wacom =
        (high as uint64_t) << 32i32 | low as uint64_t;
}
unsafe extern "C" fn handle_tablet_tool_capability(mut data:
                                                       *mut libc::c_void,
                                                   mut id:
                                                       *mut zwp_tablet_tool_v2,
                                                   mut capability: uint32_t) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    let mut cap: zwp_tablet_tool_v2_capability =
        capability as zwp_tablet_tool_v2_capability;
    match cap as libc::c_uint {
        1 => { (*tool).wlr_tool.tilt = 1i32 != 0 }
        2 => { (*tool).wlr_tool.pressure = 1i32 != 0 }
        3 => { (*tool).wlr_tool.distance = 1i32 != 0 }
        4 => { (*tool).wlr_tool.rotation = 1i32 != 0 }
        5 => { (*tool).wlr_tool.slider = 1i32 != 0 }
        6 => { (*tool).wlr_tool.wheel = 1i32 != 0 }
        _ => { }
    };
}
unsafe extern "C" fn handle_tablet_tool_proximity_in(mut data:
                                                         *mut libc::c_void,
                                                     mut id:
                                                         *mut zwp_tablet_tool_v2,
                                                     mut serial: uint32_t,
                                                     mut tablet_id:
                                                         *mut zwp_tablet_v2,
                                                     mut surface:
                                                         *mut wl_surface) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).is_in = 1i32 != 0;
    (*tool).tablet =
        zwp_tablet_v2_get_user_data(tablet_id) as *mut wlr_wl_input_device;
}
unsafe extern "C" fn handle_tablet_tool_proximity_out(mut data:
                                                          *mut libc::c_void,
                                                      mut id:
                                                          *mut zwp_tablet_tool_v2) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).is_out = 1i32 != 0;
}
unsafe extern "C" fn handle_tablet_tool_down(mut data: *mut libc::c_void,
                                             mut id: *mut zwp_tablet_tool_v2,
                                             mut serial: libc::c_uint) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).is_down = 1i32 != 0;
}
unsafe extern "C" fn handle_tablet_tool_up(mut data: *mut libc::c_void,
                                           mut id: *mut zwp_tablet_tool_v2) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).is_up = 1i32 != 0;
}
unsafe extern "C" fn handle_tablet_tool_motion(mut data: *mut libc::c_void,
                                               mut id:
                                                   *mut zwp_tablet_tool_v2,
                                               mut x: wl_fixed_t,
                                               mut y: wl_fixed_t) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).x = wl_fixed_to_double(x);
    (*tool).y = wl_fixed_to_double(y);
}
unsafe extern "C" fn handle_tablet_tool_pressure(mut data: *mut libc::c_void,
                                                 mut id:
                                                     *mut zwp_tablet_tool_v2,
                                                 mut pressure: uint32_t) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).pressure = pressure as libc::c_double / 65535.0f64;
}
unsafe extern "C" fn handle_tablet_tool_distance(mut data: *mut libc::c_void,
                                                 mut id:
                                                     *mut zwp_tablet_tool_v2,
                                                 mut distance: uint32_t) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).distance = distance as libc::c_double / 65535.0f64;
}
unsafe extern "C" fn handle_tablet_tool_tilt(mut data: *mut libc::c_void,
                                             mut id: *mut zwp_tablet_tool_v2,
                                             mut x: wl_fixed_t,
                                             mut y: wl_fixed_t) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).tilt_x = wl_fixed_to_double(x);
    (*tool).tilt_y = wl_fixed_to_double(y);
}
unsafe extern "C" fn handle_tablet_tool_rotation(mut data: *mut libc::c_void,
                                                 mut id:
                                                     *mut zwp_tablet_tool_v2,
                                                 mut rotation: wl_fixed_t) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).rotation = wl_fixed_to_double(rotation);
}
unsafe extern "C" fn handle_tablet_tool_slider(mut data: *mut libc::c_void,
                                               mut id:
                                                   *mut zwp_tablet_tool_v2,
                                               mut slider: libc::c_int) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).slider = slider as libc::c_double / 65535.0f64;
}
// TODO: This looks wrong :/
unsafe extern "C" fn handle_tablet_tool_wheel(mut data: *mut libc::c_void,
                                              mut id: *mut zwp_tablet_tool_v2,
                                              mut degree: wl_fixed_t,
                                              mut clicks: libc::c_int) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    (*tool).wheel_delta = wl_fixed_to_double(degree);
}
unsafe extern "C" fn handle_tablet_tool_button(mut data: *mut libc::c_void,
                                               mut id:
                                                   *mut zwp_tablet_tool_v2,
                                               mut serial: uint32_t,
                                               mut button: uint32_t,
                                               mut state: uint32_t) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    let mut tablet: *mut wlr_tablet =
        (*(*tool).tablet).wlr_input_device.c2rust_unnamed.tablet;
    let mut evt: wlr_event_tablet_tool_button =
        {
            let mut init =
                wlr_event_tablet_tool_button{device:
                                                 &mut (*(*tool).tablet).wlr_input_device,
                                             tool: &mut (*tool).wlr_tool,
                                             time_msec:
                                                 get_current_time_msec(),
                                             button: button,
                                             state:
                                                 if state ==
                                                        ZWP_TABLET_TOOL_V2_BUTTON_STATE_RELEASED
                                                            as libc::c_int as
                                                            libc::c_uint {
                                                     WLR_BUTTON_RELEASED as
                                                         libc::c_int
                                                 } else {
                                                     WLR_BUTTON_PRESSED as
                                                         libc::c_int
                                                 } as wlr_button_state,};
            init
        };
    wlr_signal_emit_safe(&mut (*tablet).events.button,
                         &mut evt as *mut wlr_event_tablet_tool_button as
                             *mut libc::c_void);
}
unsafe extern "C" fn clear_tablet_tool_values(mut tool:
                                                  *mut wlr_wl_tablet_tool) {
    (*tool).is_in = 0i32 != 0;
    (*tool).is_out = (*tool).is_in;
    (*tool).is_down = 0i32 != 0;
    (*tool).is_up = (*tool).is_down;
    (*tool).y = ::std::f32::NAN as libc::c_double;
    (*tool).x = (*tool).y;
    (*tool).pressure = ::std::f32::NAN as libc::c_double;
    (*tool).distance = ::std::f32::NAN as libc::c_double;
    (*tool).tilt_y = ::std::f32::NAN as libc::c_double;
    (*tool).tilt_x = (*tool).tilt_y;
    (*tool).rotation = ::std::f32::NAN as libc::c_double;
    (*tool).slider = ::std::f32::NAN as libc::c_double;
    (*tool).wheel_delta = ::std::f32::NAN as libc::c_double;
}
unsafe extern "C" fn handle_tablet_tool_frame(mut data: *mut libc::c_void,
                                              mut id: *mut zwp_tablet_tool_v2,
                                              mut time: uint32_t) {
    let mut tablet: *mut wlr_tablet = 0 as *mut wlr_tablet;
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    if !((*tool).is_out as libc::c_int != 0 &&
             (*tool).is_in as libc::c_int != 0) {
        tablet = (*(*tool).tablet).wlr_input_device.c2rust_unnamed.tablet;
        if (*tool).is_in {
            let mut evt: wlr_event_tablet_tool_proximity =
                {
                    let mut init =
                        wlr_event_tablet_tool_proximity{device:
                                                            &mut (*(*tool).tablet).wlr_input_device,
                                                        tool:
                                                            &mut (*tool).wlr_tool,
                                                        time_msec: time,
                                                        x: (*tool).x,
                                                        y: (*tool).y,
                                                        state:
                                                            WLR_TABLET_TOOL_PROXIMITY_IN,};
                    init
                };
            wlr_signal_emit_safe(&mut (*tablet).events.proximity,
                                 &mut evt as
                                     *mut wlr_event_tablet_tool_proximity as
                                     *mut libc::c_void);
        }
        let mut evt_0: wlr_event_tablet_tool_axis =
            {
                let mut init =
                    wlr_event_tablet_tool_axis{device:
                                                   &mut (*(*tool).tablet).wlr_input_device,
                                               tool: &mut (*tool).wlr_tool,
                                               time_msec: time,
                                               updated_axes: 0i32 as uint32_t,
                                               x: 0.,
                                               y: 0.,
                                               dx: 0.,
                                               dy: 0.,
                                               pressure: 0.,
                                               distance: 0.,
                                               tilt_x: 0.,
                                               tilt_y: 0.,
                                               rotation: 0.,
                                               slider: 0.,
                                               wheel_delta: 0.,};
                init
            };
        if (*tool).x == (*tool).x && !(*tool).is_in {
            evt_0.updated_axes |=
                WLR_TABLET_TOOL_AXIS_X as libc::c_int as libc::c_uint;
            evt_0.x = (*tool).x
        }
        if (*tool).y == (*tool).y && !(*tool).is_in {
            evt_0.updated_axes |=
                WLR_TABLET_TOOL_AXIS_Y as libc::c_int as libc::c_uint;
            evt_0.y = (*tool).y
        }
        if (*tool).pressure == (*tool).pressure {
            evt_0.updated_axes |=
                WLR_TABLET_TOOL_AXIS_PRESSURE as libc::c_int as libc::c_uint;
            evt_0.pressure = (*tool).pressure
        }
        if (*tool).distance == (*tool).distance {
            evt_0.updated_axes |=
                WLR_TABLET_TOOL_AXIS_DISTANCE as libc::c_int as libc::c_uint;
            evt_0.distance = (*tool).distance
        }
        if (*tool).tilt_x == (*tool).tilt_x {
            evt_0.updated_axes |=
                WLR_TABLET_TOOL_AXIS_TILT_X as libc::c_int as libc::c_uint;
            evt_0.tilt_x = (*tool).tilt_x
        }
        if (*tool).tilt_y == (*tool).tilt_y {
            evt_0.updated_axes |=
                WLR_TABLET_TOOL_AXIS_TILT_Y as libc::c_int as libc::c_uint;
            evt_0.tilt_y = (*tool).tilt_y
        }
        if (*tool).rotation == (*tool).rotation {
            evt_0.updated_axes |=
                WLR_TABLET_TOOL_AXIS_ROTATION as libc::c_int as libc::c_uint;
            evt_0.rotation = (*tool).rotation
        }
        if (*tool).slider == (*tool).slider {
            evt_0.updated_axes |=
                WLR_TABLET_TOOL_AXIS_SLIDER as libc::c_int as libc::c_uint;
            evt_0.slider = (*tool).slider
        }
        if (*tool).wheel_delta == (*tool).wheel_delta {
            evt_0.updated_axes |=
                WLR_TABLET_TOOL_AXIS_WHEEL as libc::c_int as libc::c_uint;
            evt_0.wheel_delta = (*tool).wheel_delta
        }
        if evt_0.updated_axes != 0 {
            wlr_signal_emit_safe(&mut (*tablet).events.proximity,
                                 &mut evt_0 as *mut wlr_event_tablet_tool_axis
                                     as *mut libc::c_void);
        }
        /* This will always send down then up if we got both.
	 * Maybe we should send them right away, in case we get up then both in
	 * series?
	 * Downside: Here we have the frame time, if we sent right away, we
	 * need to generate the time */
        if (*tool).is_down {
            let mut evt_1: wlr_event_tablet_tool_tip =
                {
                    let mut init =
                        wlr_event_tablet_tool_tip{device:
                                                      &mut (*(*tool).tablet).wlr_input_device,
                                                  tool: &mut (*tool).wlr_tool,
                                                  time_msec: time,
                                                  x: (*tool).x,
                                                  y: (*tool).y,
                                                  state:
                                                      WLR_TABLET_TOOL_TIP_DOWN,};
                    init
                };
            wlr_signal_emit_safe(&mut (*tablet).events.tip,
                                 &mut evt_1 as *mut wlr_event_tablet_tool_tip
                                     as *mut libc::c_void);
        }
        if (*tool).is_up {
            let mut evt_2: wlr_event_tablet_tool_tip =
                {
                    let mut init =
                        wlr_event_tablet_tool_tip{device:
                                                      &mut (*(*tool).tablet).wlr_input_device,
                                                  tool: &mut (*tool).wlr_tool,
                                                  time_msec: time,
                                                  x: (*tool).x,
                                                  y: (*tool).y,
                                                  state:
                                                      WLR_TABLET_TOOL_TIP_UP,};
                    init
                };
            wlr_signal_emit_safe(&mut (*tablet).events.tip,
                                 &mut evt_2 as *mut wlr_event_tablet_tool_tip
                                     as *mut libc::c_void);
        }
        if (*tool).is_out {
            let mut evt_3: wlr_event_tablet_tool_proximity =
                {
                    let mut init =
                        wlr_event_tablet_tool_proximity{device:
                                                            &mut (*(*tool).tablet).wlr_input_device,
                                                        tool:
                                                            &mut (*tool).wlr_tool,
                                                        time_msec: time,
                                                        x: (*tool).x,
                                                        y: (*tool).y,
                                                        state:
                                                            WLR_TABLET_TOOL_PROXIMITY_OUT,};
                    init
                };
            wlr_signal_emit_safe(&mut (*tablet).events.proximity,
                                 &mut evt_3 as
                                     *mut wlr_event_tablet_tool_proximity as
                                     *mut libc::c_void);
        }
    }
    /* we got a tablet tool coming in and out of proximity before
		 * we could process it. Just ignore anything it did */
    clear_tablet_tool_values(tool);
}
unsafe extern "C" fn handle_tablet_tool_removed(mut data: *mut libc::c_void,
                                                mut id:
                                                    *mut zwp_tablet_tool_v2) {
    let mut tool: *mut wlr_wl_tablet_tool = data as *mut wlr_wl_tablet_tool;
    zwp_tablet_tool_v2_destroy((*tool).tool);
    wlr_signal_emit_safe(&mut (*tool).wlr_tool.events.destroy,
                         &mut (*tool).wlr_tool as *mut wlr_tablet_tool as
                             *mut libc::c_void);
    free(tool as *mut libc::c_void);
}
static mut tablet_tool_listener: zwp_tablet_tool_v2_listener =
    {
    
        {
            let mut init =
                zwp_tablet_tool_v2_listener{type_0:
                                                Some(handle_tablet_tool_type
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  uint32_t)
                                                             -> ()),
                                            hardware_serial:
                                                Some(handle_tablet_tool_serial
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  uint32_t,
                                                                              _:
                                                                                  uint32_t)
                                                             -> ()),
                                            hardware_id_wacom:
                                                Some(handle_tablet_tool_id_wacom
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  uint32_t,
                                                                              _:
                                                                                  uint32_t)
                                                             -> ()),
                                            capability:
                                                Some(handle_tablet_tool_capability
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  uint32_t)
                                                             -> ()),
                                            done:
                                                Some(handle_tablet_tool_done
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2)
                                                             -> ()),
                                            removed:
                                                Some(handle_tablet_tool_removed
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2)
                                                             -> ()),
                                            proximity_in:
                                                Some(handle_tablet_tool_proximity_in
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  uint32_t,
                                                                              _:
                                                                                  *mut zwp_tablet_v2,
                                                                              _:
                                                                                  *mut wl_surface)
                                                             -> ()),
                                            proximity_out:
                                                Some(handle_tablet_tool_proximity_out
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2)
                                                             -> ()),
                                            down:
                                                Some(handle_tablet_tool_down
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  libc::c_uint)
                                                             -> ()),
                                            up:
                                                Some(handle_tablet_tool_up as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2)
                                                             -> ()),
                                            motion:
                                                Some(handle_tablet_tool_motion
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  wl_fixed_t,
                                                                              _:
                                                                                  wl_fixed_t)
                                                             -> ()),
                                            pressure:
                                                Some(handle_tablet_tool_pressure
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  uint32_t)
                                                             -> ()),
                                            distance:
                                                Some(handle_tablet_tool_distance
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  uint32_t)
                                                             -> ()),
                                            tilt:
                                                Some(handle_tablet_tool_tilt
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  wl_fixed_t,
                                                                              _:
                                                                                  wl_fixed_t)
                                                             -> ()),
                                            rotation:
                                                Some(handle_tablet_tool_rotation
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  wl_fixed_t)
                                                             -> ()),
                                            slider:
                                                Some(handle_tablet_tool_slider
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  libc::c_int)
                                                             -> ()),
                                            wheel:
                                                Some(handle_tablet_tool_wheel
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  wl_fixed_t,
                                                                              _:
                                                                                  libc::c_int)
                                                             -> ()),
                                            button:
                                                Some(handle_tablet_tool_button
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  uint32_t,
                                                                              _:
                                                                                  uint32_t,
                                                                              _:
                                                                                  uint32_t)
                                                             -> ()),
                                            frame:
                                                Some(handle_tablet_tool_frame
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2,
                                                                              _:
                                                                                  uint32_t)
                                                             -> ()),};
            init
        }
};
unsafe extern "C" fn handle_tool_added(mut data: *mut libc::c_void,
                                       mut zwp_tablet_seat_v2:
                                           *mut zwp_tablet_seat_v2,
                                       mut id: *mut zwp_tablet_tool_v2) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] New tablet tool\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/wayland/tablet_v2.c\x00" as *const u8 as
                 *const libc::c_char, 823i32);
    let mut tool: *mut wlr_wl_tablet_tool =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_wl_tablet_tool>() as libc::c_ulong)
            as *mut wlr_wl_tablet_tool;
    if tool.is_null() { zwp_tablet_tool_v2_destroy(id); return }
    (*tool).tool = id;
    clear_tablet_tool_values(tool);
    wl_signal_init(&mut (*tool).wlr_tool.events.destroy);
    zwp_tablet_tool_v2_add_listener(id, &tablet_tool_listener,
                                    tool as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_name(mut data: *mut libc::c_void,
                                        mut zwp_tablet_v2: *mut zwp_tablet_v2,
                                        mut name: *const libc::c_char) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    let mut tablet: *mut wlr_tablet =
        (*dev).wlr_input_device.c2rust_unnamed.tablet;
    free((*tablet).name as *mut libc::c_void);
    (*tablet).name = strdup(name);
}
unsafe extern "C" fn handle_tablet_id(mut data: *mut libc::c_void,
                                      mut zwp_tablet_v2: *mut zwp_tablet_v2,
                                      mut vid: uint32_t, mut pid: uint32_t) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    (*dev).wlr_input_device.vendor = vid;
    (*dev).wlr_input_device.product = pid;
}
unsafe extern "C" fn handle_tablet_path(mut data: *mut libc::c_void,
                                        mut zwp_tablet_v2: *mut zwp_tablet_v2,
                                        mut path: *const libc::c_char) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    let mut tablet: *mut wlr_tablet =
        (*dev).wlr_input_device.c2rust_unnamed.tablet;
    wlr_list_push(&mut (*tablet).paths, strdup(path) as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_done(mut data: *mut libc::c_void,
                                        mut zwp_tablet_v2:
                                            *mut zwp_tablet_v2) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    wlr_signal_emit_safe(&mut (*(*dev).backend).backend.events.new_input,
                         &mut (*dev).wlr_input_device as *mut wlr_input_device
                             as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_removed(mut data: *mut libc::c_void,
                                           mut zwp_tablet_v2:
                                               *mut zwp_tablet_v2) {
    let mut dev: *mut wlr_wl_input_device = data as *mut wlr_wl_input_device;
    /* This doesn't free anything, but emits the destroy signal */
    wlr_input_device_destroy(&mut (*dev).wlr_input_device);
    /* This is a bit ugly, but we need to remove it from our list */
    wl_list_remove(&mut (*dev).wlr_input_device.link);
    zwp_tablet_v2_destroy((*dev).resource as *mut zwp_tablet_v2);
    free(dev as *mut libc::c_void);
}
static mut tablet_listener: zwp_tablet_v2_listener =
    {
    
        {
            let mut init =
                zwp_tablet_v2_listener{name:
                                           Some(handle_tablet_name as
                                                    unsafe extern "C" fn(_:
                                                                             *mut libc::c_void,
                                                                         _:
                                                                             *mut zwp_tablet_v2,
                                                                         _:
                                                                             *const libc::c_char)
                                                        -> ()),
                                       id:
                                           Some(handle_tablet_id as
                                                    unsafe extern "C" fn(_:
                                                                             *mut libc::c_void,
                                                                         _:
                                                                             *mut zwp_tablet_v2,
                                                                         _:
                                                                             uint32_t,
                                                                         _:
                                                                             uint32_t)
                                                        -> ()),
                                       path:
                                           Some(handle_tablet_path as
                                                    unsafe extern "C" fn(_:
                                                                             *mut libc::c_void,
                                                                         _:
                                                                             *mut zwp_tablet_v2,
                                                                         _:
                                                                             *const libc::c_char)
                                                        -> ()),
                                       done:
                                           Some(handle_tablet_done as
                                                    unsafe extern "C" fn(_:
                                                                             *mut libc::c_void,
                                                                         _:
                                                                             *mut zwp_tablet_v2)
                                                        -> ()),
                                       removed:
                                           Some(handle_tablet_removed as
                                                    unsafe extern "C" fn(_:
                                                                             *mut libc::c_void,
                                                                         _:
                                                                             *mut zwp_tablet_v2)
                                                        -> ()),};
            init
        }
};
unsafe extern "C" fn handle_tab_added(mut data: *mut libc::c_void,
                                      mut zwp_tablet_seat_v2:
                                          *mut zwp_tablet_seat_v2,
                                      mut id: *mut zwp_tablet_v2) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] New tablet\x00" as *const u8 as *const libc::c_char,
             b"../backend/wayland/tablet_v2.c\x00" as *const u8 as
                 *const libc::c_char, 890i32);
    let mut backend: *mut wlr_wl_backend = data as *mut wlr_wl_backend;
    let mut dev: *mut wlr_wl_input_device =
        create_wl_input_device(backend, WLR_INPUT_DEVICE_TABLET_TOOL);
    if dev.is_null() { zwp_tablet_v2_destroy(id); return }
    (*dev).resource = id as *mut libc::c_void;
    let mut wlr_dev: *mut wlr_input_device = &mut (*dev).wlr_input_device;
    (*wlr_dev).c2rust_unnamed.tablet =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_tablet>() as libc::c_ulong) as
            *mut wlr_tablet;
    if (*wlr_dev).c2rust_unnamed.tablet.is_null() {
        zwp_tablet_v2_destroy(id);
        return
    }
    zwp_tablet_v2_set_user_data(id,
                                (*wlr_dev).c2rust_unnamed.tablet as
                                    *mut libc::c_void);
    wlr_tablet_init((*wlr_dev).c2rust_unnamed.tablet,
                    0 as *mut wlr_tablet_impl);
    zwp_tablet_v2_add_listener(id, &tablet_listener,
                               dev as *mut libc::c_void);
}
static mut tablet_seat_listener: zwp_tablet_seat_v2_listener =
    {
    
        {
            let mut init =
                zwp_tablet_seat_v2_listener{tablet_added:
                                                Some(handle_tab_added as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_seat_v2,
                                                                              _:
                                                                                  *mut zwp_tablet_v2)
                                                             -> ()),
                                            tool_added:
                                                Some(handle_tool_added as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_seat_v2,
                                                                              _:
                                                                                  *mut zwp_tablet_tool_v2)
                                                             -> ()),
                                            pad_added:
                                                Some(handle_pad_added as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut zwp_tablet_seat_v2,
                                                                              _:
                                                                                  *mut zwp_tablet_pad_v2)
                                                             -> ()),};
            init
        }
};
/* local state */
/* remote state */
#[no_mangle]
pub unsafe extern "C" fn wl_add_tablet_seat(mut manager:
                                                *mut zwp_tablet_manager_v2,
                                            mut seat: *mut wl_seat,
                                            mut backend: *mut wlr_wl_backend)
 -> *mut wlr_wl_tablet_seat {
    let mut ret: *mut wlr_wl_tablet_seat =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_wl_tablet_seat>() as libc::c_ulong)
            as *mut wlr_wl_tablet_seat;
    (*ret).tablet_seat = zwp_tablet_manager_v2_get_tablet_seat(manager, seat);
    if (*ret).tablet_seat.is_null() {
        free(ret as *mut libc::c_void);
        return 0 as *mut wlr_wl_tablet_seat
    }
    zwp_tablet_seat_v2_add_listener((*ret).tablet_seat, &tablet_seat_listener,
                                    backend as *mut libc::c_void);
    return ret;
}
