use libc;
extern "C" {
    pub type xcb_connection_t;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_group;
    pub type wlr_tablet_pad_impl;
    pub type wlr_tablet_impl;
    pub type wlr_switch_impl;
    pub type _XDisplay;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_backend_impl;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    pub type wlr_surface;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn xcb_query_pointer(c: *mut xcb_connection_t, window: xcb_window_t)
     -> xcb_query_pointer_cookie_t;
    #[no_mangle]
    fn xcb_query_pointer_reply(c: *mut xcb_connection_t,
                               cookie: xcb_query_pointer_cookie_t,
                               e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_query_pointer_reply_t;
    #[no_mangle]
    fn xcb_flush(c: *mut xcb_connection_t) -> libc::c_int;
    #[no_mangle]
    fn xcb_xfixes_show_cursor(c: *mut xcb_connection_t, window: xcb_window_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_xfixes_hide_cursor(c: *mut xcb_connection_t, window: xcb_window_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wlr_keyboard_notify_key(keyboard: *mut wlr_keyboard,
                               event: *mut wlr_event_keyboard_key);
    #[no_mangle]
    fn wlr_keyboard_notify_modifiers(keyboard: *mut wlr_keyboard,
                                     mods_depressed: uint32_t,
                                     mods_latched: uint32_t,
                                     mods_locked: uint32_t, group: uint32_t);
    #[no_mangle]
    fn get_x11_output_from_window_id(x11: *mut wlr_x11_backend,
                                     window: xcb_window_t)
     -> *mut wlr_x11_output;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_generic_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub resource_id: uint32_t,
    pub minor_code: uint16_t,
    pub major_code: uint8_t,
    pub pad0: uint8_t,
    pub pad: [uint32_t; 5],
    pub full_sequence: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_void_cookie_t {
    pub sequence: libc::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_ge_generic_event_t {
    pub response_type: uint8_t,
    pub extension: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub event_type: uint16_t,
    pub pad0: [uint8_t; 22],
    pub full_sequence: uint32_t,
}
pub type xcb_button_index_t = libc::c_uint;
pub const XCB_BUTTON_INDEX_5: xcb_button_index_t = 5;
pub const XCB_BUTTON_INDEX_4: xcb_button_index_t = 4;
pub const XCB_BUTTON_INDEX_3: xcb_button_index_t = 3;
pub const XCB_BUTTON_INDEX_2: xcb_button_index_t = 2;
pub const XCB_BUTTON_INDEX_1: xcb_button_index_t = 1;
pub const XCB_BUTTON_INDEX_ANY: xcb_button_index_t = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_query_pointer_cookie_t {
    pub sequence: libc::c_uint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_query_pointer_reply_t {
    pub response_type: uint8_t,
    pub same_screen: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub win_x: int16_t,
    pub win_y: int16_t,
    pub mask: uint16_t,
    pub pad0: [uint8_t; 2],
}
pub type xcb_input_device_id_t = uint16_t;
pub type xcb_input_fp1616_t = int32_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_input_group_info_t {
    pub base: uint8_t,
    pub latched: uint8_t,
    pub locked: uint8_t,
    pub effective: uint8_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_input_modifier_info_t {
    pub base: uint32_t,
    pub latched: uint32_t,
    pub locked: uint32_t,
    pub effective: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_input_key_press_event_t {
    pub response_type: uint8_t,
    pub extension: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub event_type: uint16_t,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: uint32_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: uint32_t,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub buttons_len: uint16_t,
    pub valuators_len: uint16_t,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [uint8_t; 2],
    pub flags: uint32_t,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}
pub type xcb_input_key_release_event_t = xcb_input_key_press_event_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_input_button_press_event_t {
    pub response_type: uint8_t,
    pub extension: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub event_type: uint16_t,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: uint32_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: uint32_t,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub buttons_len: uint16_t,
    pub valuators_len: uint16_t,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [uint8_t; 2],
    pub flags: uint32_t,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}
pub type xcb_input_button_release_event_t = xcb_input_button_press_event_t;
pub type xcb_input_motion_event_t = xcb_input_button_press_event_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_input_enter_event_t {
    pub response_type: uint8_t,
    pub extension: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub event_type: uint16_t,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub sourceid: xcb_input_device_id_t,
    pub mode: uint8_t,
    pub detail: uint8_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: uint32_t,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub same_screen: uint8_t,
    pub focus: uint8_t,
    pub buttons_len: uint16_t,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}
pub type xcb_input_leave_event_t = xcb_input_enter_event_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_input_touch_begin_event_t {
    pub response_type: uint8_t,
    pub extension: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub event_type: uint16_t,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: uint32_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: uint32_t,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub buttons_len: uint16_t,
    pub valuators_len: uint16_t,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [uint8_t; 2],
    pub flags: uint32_t,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}
pub type xcb_input_touch_update_event_t = xcb_input_touch_begin_event_t;
pub type xcb_input_touch_end_event_t = xcb_input_touch_begin_event_t;
pub type wlr_button_state = libc::c_uint;
pub const WLR_BUTTON_PRESSED: wlr_button_state = 1;
pub const WLR_BUTTON_RELEASED: wlr_button_state = 0;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
pub type wlr_input_device_type = libc::c_uint;
pub const WLR_INPUT_DEVICE_SWITCH: wlr_input_device_type = 5;
pub const WLR_INPUT_DEVICE_TABLET_PAD: wlr_input_device_type = 4;
pub const WLR_INPUT_DEVICE_TABLET_TOOL: wlr_input_device_type = 3;
pub const WLR_INPUT_DEVICE_TOUCH: wlr_input_device_type = 2;
pub const WLR_INPUT_DEVICE_POINTER: wlr_input_device_type = 1;
pub const WLR_INPUT_DEVICE_KEYBOARD: wlr_input_device_type = 0;

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
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_keyboard_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_keyboard) -> ()>,
    pub led_update: Option<unsafe extern "C" fn(_: *mut wlr_keyboard,
                                                _: uint32_t) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_keyboard {
    pub impl_0: *const wlr_keyboard_impl,
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
pub struct wlr_keyboard_modifiers {
    pub depressed: xkb_mod_mask_t,
    pub latched: xkb_mod_mask_t,
    pub locked: xkb_mod_mask_t,
    pub group: xkb_mod_mask_t,
}
pub type wlr_key_state = libc::c_uint;
pub const WLR_KEY_PRESSED: wlr_key_state = 1;
pub const WLR_KEY_RELEASED: wlr_key_state = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_keyboard_key {
    pub time_msec: uint32_t,
    pub keycode: uint32_t,
    pub update_state: bool,
    pub state: wlr_key_state,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_pointer_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_pointer) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
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
    pub impl_0: *const wlr_touch_impl,
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_touch_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_touch) -> ()>,
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
/* Note: these are circular dependencies */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_pointer_motion_absolute {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_pointer_button {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub button: uint32_t,
    pub state: wlr_button_state,
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
pub struct wlr_event_pointer_axis {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub source: wlr_axis_source,
    pub orientation: wlr_axis_orientation,
    pub delta: libc::c_double,
    pub delta_discrete: int32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_touch_down {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_touch_up {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_touch_motion {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type Display = _XDisplay;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_dmabuf_attributes {
    pub width: int32_t,
    pub height: int32_t,
    pub format: uint32_t,
    pub flags: uint32_t,
    pub modifier: uint64_t,
    pub n_planes: libc::c_int,
    pub offset: [uint32_t; 4],
    pub stride: [uint32_t; 4],
    pub fd: [libc::c_int; 4],
}

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
    pub exts: C2RustUnnamed_8,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
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
    pub events: C2RustUnnamed_9,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_renderer {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_renderer_impl,
    pub events: C2RustUnnamed_10,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
    pub destroy: wl_signal,
}
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
/* *
 * A client buffer.
 */
/* *
	 * The buffer resource, if any. Will be NULL if the client destroys it.
	 */
/* *
	 * The buffer's texture, if any. A buffer will not have a texture if the
	 * client destroys the buffer before it has been released.
	 */

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
    pub events: C2RustUnnamed_11,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
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
    pub events: C2RustUnnamed_12,
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
pub struct C2RustUnnamed_12 {
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
pub struct wlr_output_impl {
    pub enable: Option<unsafe extern "C" fn(_: *mut wlr_output, _: bool)
                           -> bool>,
    pub set_mode: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                              _: *mut wlr_output_mode)
                             -> bool>,
    pub set_custom_mode: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                     _: int32_t, _: int32_t,
                                                     _: int32_t) -> bool>,
    pub set_cursor: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                _: *mut wlr_texture,
                                                _: int32_t,
                                                _: wl_output_transform,
                                                _: int32_t, _: int32_t,
                                                _: bool) -> bool>,
    pub move_cursor: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                 _: libc::c_int,
                                                 _: libc::c_int) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_output) -> ()>,
    pub attach_render: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                   _: *mut libc::c_int)
                                  -> bool>,
    pub commit: Option<unsafe extern "C" fn(_: *mut wlr_output) -> bool>,
    pub set_gamma: Option<unsafe extern "C" fn(_: *mut wlr_output, _: size_t,
                                               _: *const uint16_t,
                                               _: *const uint16_t,
                                               _: *const uint16_t) -> bool>,
    pub get_gamma_size: Option<unsafe extern "C" fn(_: *mut wlr_output)
                                   -> size_t>,
    pub export_dmabuf: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                   _:
                                                       *mut wlr_dmabuf_attributes)
                                  -> bool>,
    pub schedule_frame: Option<unsafe extern "C" fn(_: *mut wlr_output)
                                   -> bool>,
    pub attach_buffer: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                   _: *mut wlr_buffer)
                                  -> bool>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_x11_backend {
    pub backend: wlr_backend,
    pub wl_display: *mut wl_display,
    pub started: bool,
    pub xlib_conn: *mut Display,
    pub xcb: *mut xcb_connection_t,
    pub screen: *mut xcb_screen_t,
    pub requested_outputs: size_t,
    pub last_output_num: size_t,
    pub outputs: wl_list,
    pub keyboard: wlr_keyboard,
    pub keyboard_dev: wlr_input_device,
    pub egl: wlr_egl,
    pub renderer: *mut wlr_renderer,
    pub event_source: *mut wl_event_source,
    pub atoms: C2RustUnnamed_13,
    pub time: xcb_timestamp_t,
    pub xinput_opcode: uint8_t,
    pub display_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
    pub wm_protocols: xcb_atom_t,
    pub wm_delete_window: xcb_atom_t,
    pub net_wm_name: xcb_atom_t,
    pub utf8_string: xcb_atom_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_x11_output {
    pub wlr_output: wlr_output,
    pub x11: *mut wlr_x11_backend,
    pub link: wl_list,
    pub win: xcb_window_t,
    pub surf: EGLSurface,
    pub pointer: wlr_pointer,
    pub pointer_dev: wlr_input_device,
    pub touch: wlr_touch,
    pub touch_dev: wlr_input_device,
    pub touchpoints: wl_list,
    pub frame_timer: *mut wl_event_source,
    pub frame_delay: libc::c_int,
    pub cursor_hidden: bool,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_x11_touchpoint {
    pub x11_id: uint32_t,
    pub wayland_id: libc::c_int,
    pub link: wl_list,
}
unsafe extern "C" fn send_key_event(mut x11: *mut wlr_x11_backend,
                                    mut key: uint32_t, mut st: wlr_key_state,
                                    mut time: xcb_timestamp_t) {
    let mut ev: wlr_event_keyboard_key =
        {
            let mut init =
                wlr_event_keyboard_key{time_msec: time,
                                       keycode: key,
                                       update_state: 1i32 != 0,
                                       state: st,};
            init
        };
    wlr_keyboard_notify_key(&mut (*x11).keyboard, &mut ev);
}
unsafe extern "C" fn send_button_event(mut output: *mut wlr_x11_output,
                                       mut key: uint32_t,
                                       mut st: wlr_button_state,
                                       mut time: xcb_timestamp_t) {
    let mut ev: wlr_event_pointer_button =
        {
            let mut init =
                wlr_event_pointer_button{device: &mut (*output).pointer_dev,
                                         time_msec: time,
                                         button: key,
                                         state: st,};
            init
        };
    wlr_signal_emit_safe(&mut (*output).pointer.events.button,
                         &mut ev as *mut wlr_event_pointer_button as
                             *mut libc::c_void);
    wlr_signal_emit_safe(&mut (*output).pointer.events.frame,
                         &mut (*output).pointer as *mut wlr_pointer as
                             *mut libc::c_void);
}
unsafe extern "C" fn send_axis_event(mut output: *mut wlr_x11_output,
                                     mut delta: int32_t,
                                     mut time: xcb_timestamp_t) {
    let mut ev: wlr_event_pointer_axis =
        {
            let mut init =
                wlr_event_pointer_axis{device: &mut (*output).pointer_dev,
                                       time_msec: time,
                                       source: WLR_AXIS_SOURCE_WHEEL,
                                       orientation:
                                           WLR_AXIS_ORIENTATION_VERTICAL,
                                       delta:
                                           (delta * 15i32) as libc::c_double,
                                       delta_discrete: delta,};
            init
        };
    wlr_signal_emit_safe(&mut (*output).pointer.events.axis,
                         &mut ev as *mut wlr_event_pointer_axis as
                             *mut libc::c_void);
    wlr_signal_emit_safe(&mut (*output).pointer.events.frame,
                         &mut (*output).pointer as *mut wlr_pointer as
                             *mut libc::c_void);
}
unsafe extern "C" fn send_pointer_position_event(mut output:
                                                     *mut wlr_x11_output,
                                                 mut x: int16_t,
                                                 mut y: int16_t,
                                                 mut time: xcb_timestamp_t) {
    let mut ev: wlr_event_pointer_motion_absolute =
        {
            let mut init =
                wlr_event_pointer_motion_absolute{device:
                                                      &mut (*output).pointer_dev,
                                                  time_msec: time,
                                                  x:
                                                      x as libc::c_double /
                                                          (*output).wlr_output.width
                                                              as
                                                              libc::c_double,
                                                  y:
                                                      y as libc::c_double /
                                                          (*output).wlr_output.height
                                                              as
                                                              libc::c_double,};
            init
        };
    wlr_signal_emit_safe(&mut (*output).pointer.events.motion_absolute,
                         &mut ev as *mut wlr_event_pointer_motion_absolute as
                             *mut libc::c_void);
    wlr_signal_emit_safe(&mut (*output).pointer.events.frame,
                         &mut (*output).pointer as *mut wlr_pointer as
                             *mut libc::c_void);
}
unsafe extern "C" fn send_touch_down_event(mut output: *mut wlr_x11_output,
                                           mut x: int16_t, mut y: int16_t,
                                           mut touch_id: int32_t,
                                           mut time: xcb_timestamp_t) {
    let mut ev: wlr_event_touch_down =
        {
            let mut init =
                wlr_event_touch_down{device: &mut (*output).touch_dev,
                                     time_msec: time,
                                     touch_id: touch_id,
                                     x:
                                         x as libc::c_double /
                                             (*output).wlr_output.width as
                                                 libc::c_double,
                                     y:
                                         y as libc::c_double /
                                             (*output).wlr_output.height as
                                                 libc::c_double,};
            init
        };
    wlr_signal_emit_safe(&mut (*output).touch.events.down,
                         &mut ev as *mut wlr_event_touch_down as
                             *mut libc::c_void);
}
unsafe extern "C" fn send_touch_motion_event(mut output: *mut wlr_x11_output,
                                             mut x: int16_t, mut y: int16_t,
                                             mut touch_id: int32_t,
                                             mut time: xcb_timestamp_t) {
    let mut ev: wlr_event_touch_motion =
        {
            let mut init =
                wlr_event_touch_motion{device: &mut (*output).touch_dev,
                                       time_msec: time,
                                       touch_id: touch_id,
                                       x:
                                           x as libc::c_double /
                                               (*output).wlr_output.width as
                                                   libc::c_double,
                                       y:
                                           y as libc::c_double /
                                               (*output).wlr_output.height as
                                                   libc::c_double,};
            init
        };
    wlr_signal_emit_safe(&mut (*output).touch.events.motion,
                         &mut ev as *mut wlr_event_touch_motion as
                             *mut libc::c_void);
}
unsafe extern "C" fn send_touch_up_event(mut output: *mut wlr_x11_output,
                                         mut touch_id: int32_t,
                                         mut time: xcb_timestamp_t) {
    let mut ev: wlr_event_touch_up =
        {
            let mut init =
                wlr_event_touch_up{device: &mut (*output).touch_dev,
                                   time_msec: time,
                                   touch_id: touch_id,};
            init
        };
    wlr_signal_emit_safe(&mut (*output).touch.events.up,
                         &mut ev as *mut wlr_event_touch_up as
                             *mut libc::c_void);
}
unsafe extern "C" fn get_touchpoint_from_x11_touch_id(mut output:
                                                          *mut wlr_x11_output,
                                                      mut id: uint32_t)
 -> *mut wlr_x11_touchpoint {
    let mut touchpoint: *mut wlr_x11_touchpoint =
        0 as *mut wlr_x11_touchpoint;
    touchpoint =
        ((*output).touchpoints.next as *mut libc::c_char).offset(-8) as
            *mut wlr_x11_touchpoint;
    while &mut (*touchpoint).link as *mut wl_list !=
              &mut (*output).touchpoints as *mut wl_list {
        if (*touchpoint).x11_id == id { return touchpoint }
        touchpoint =
            ((*touchpoint).link.next as *mut libc::c_char).offset(-8) as
                *mut wlr_x11_touchpoint
    }
    return 0 as *mut wlr_x11_touchpoint;
}
#[no_mangle]
pub unsafe extern "C" fn handle_x11_xinput_event(mut x11:
                                                     *mut wlr_x11_backend,
                                                 mut event:
                                                     *mut xcb_ge_generic_event_t) {
    let mut output: *mut wlr_x11_output = 0 as *mut wlr_x11_output;
    match (*event).event_type as libc::c_int {
        2 => {
            let mut ev: *mut xcb_input_key_press_event_t =
                event as *mut xcb_input_key_press_event_t;
            wlr_keyboard_notify_modifiers(&mut (*x11).keyboard,
                                          (*ev).mods.base, (*ev).mods.latched,
                                          (*ev).mods.locked,
                                          (*ev).mods.effective);
            send_key_event(x11,
                           (*ev).detail.wrapping_sub(8i32 as libc::c_uint),
                           WLR_KEY_PRESSED, (*ev).time);
            (*x11).time = (*ev).time
        }
        3 => {
            let mut ev_0: *mut xcb_input_key_release_event_t =
                event as *mut xcb_input_key_release_event_t;
            wlr_keyboard_notify_modifiers(&mut (*x11).keyboard,
                                          (*ev_0).mods.base,
                                          (*ev_0).mods.latched,
                                          (*ev_0).mods.locked,
                                          (*ev_0).mods.effective);
            send_key_event(x11,
                           (*ev_0).detail.wrapping_sub(8i32 as libc::c_uint),
                           WLR_KEY_RELEASED, (*ev_0).time);
            (*x11).time = (*ev_0).time
        }
        4 => {
            let mut ev_1: *mut xcb_input_button_press_event_t =
                event as *mut xcb_input_button_press_event_t;
            output = get_x11_output_from_window_id(x11, (*ev_1).event);
            if output.is_null() { return }
            match (*ev_1).detail {
                1 => {
                    send_button_event(output, 0x110i32 as uint32_t,
                                      WLR_BUTTON_PRESSED, (*ev_1).time);
                }
                2 => {
                    send_button_event(output, 0x112i32 as uint32_t,
                                      WLR_BUTTON_PRESSED, (*ev_1).time);
                }
                3 => {
                    send_button_event(output, 0x111i32 as uint32_t,
                                      WLR_BUTTON_PRESSED, (*ev_1).time);
                }
                4 => { send_axis_event(output, -1i32, (*ev_1).time); }
                5 => { send_axis_event(output, 1i32, (*ev_1).time); }
                _ => { }
            }
            (*x11).time = (*ev_1).time
        }
        5 => {
            let mut ev_2: *mut xcb_input_button_release_event_t =
                event as *mut xcb_input_button_release_event_t;
            output = get_x11_output_from_window_id(x11, (*ev_2).event);
            if output.is_null() { return }
            match (*ev_2).detail {
                1 => {
                    send_button_event(output, 0x110i32 as uint32_t,
                                      WLR_BUTTON_RELEASED, (*ev_2).time);
                }
                2 => {
                    send_button_event(output, 0x112i32 as uint32_t,
                                      WLR_BUTTON_RELEASED, (*ev_2).time);
                }
                3 => {
                    send_button_event(output, 0x111i32 as uint32_t,
                                      WLR_BUTTON_RELEASED, (*ev_2).time);
                }
                _ => { }
            }
            (*x11).time = (*ev_2).time
        }
        6 => {
            let mut ev_3: *mut xcb_input_motion_event_t =
                event as *mut xcb_input_motion_event_t;
            output = get_x11_output_from_window_id(x11, (*ev_3).event);
            if output.is_null() { return }
            send_pointer_position_event(output,
                                        ((*ev_3).event_x >> 16i32) as int16_t,
                                        ((*ev_3).event_y >> 16i32) as int16_t,
                                        (*ev_3).time);
            (*x11).time = (*ev_3).time
        }
        7 => {
            let mut ev_4: *mut xcb_input_enter_event_t =
                event as *mut xcb_input_enter_event_t;
            output = get_x11_output_from_window_id(x11, (*ev_4).event);
            if output.is_null() { return }
            if !(*output).cursor_hidden {
                xcb_xfixes_hide_cursor((*x11).xcb, (*output).win);
                xcb_flush((*x11).xcb);
                (*output).cursor_hidden = 1i32 != 0
            }
        }
        8 => {
            let mut ev_5: *mut xcb_input_leave_event_t =
                event as *mut xcb_input_leave_event_t;
            output = get_x11_output_from_window_id(x11, (*ev_5).event);
            if output.is_null() { return }
            if (*output).cursor_hidden {
                xcb_xfixes_show_cursor((*x11).xcb, (*output).win);
                xcb_flush((*x11).xcb);
                (*output).cursor_hidden = 0i32 != 0
            }
        }
        18 => {
            let mut ev_6: *mut xcb_input_touch_begin_event_t =
                event as *mut xcb_input_touch_begin_event_t;
            output = get_x11_output_from_window_id(x11, (*ev_6).event);
            if output.is_null() { return }
            let mut id: int32_t = 0i32;
            if wl_list_empty(&mut (*output).touchpoints) == 0 {
                let mut last_touchpoint: *mut wlr_x11_touchpoint =
                    ((*output).touchpoints.next as
                         *mut libc::c_char).offset(-8) as
                        *mut wlr_x11_touchpoint;
                id = (*last_touchpoint).wayland_id + 1i32
            }
            let mut touchpoint: *mut wlr_x11_touchpoint =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<wlr_x11_touchpoint>() as
                           libc::c_ulong) as *mut wlr_x11_touchpoint;
            (*touchpoint).x11_id = (*ev_6).detail;
            (*touchpoint).wayland_id = id;
            wl_list_init(&mut (*touchpoint).link);
            wl_list_insert(&mut (*output).touchpoints,
                           &mut (*touchpoint).link);
            send_touch_down_event(output,
                                  ((*ev_6).event_x >> 16i32) as int16_t,
                                  ((*ev_6).event_y >> 16i32) as int16_t,
                                  (*touchpoint).wayland_id, (*ev_6).time);
            (*x11).time = (*ev_6).time
        }
        20 => {
            let mut ev_7: *mut xcb_input_touch_end_event_t =
                event as *mut xcb_input_touch_end_event_t;
            output = get_x11_output_from_window_id(x11, (*ev_7).event);
            if output.is_null() { return }
            let mut touchpoint_0: *mut wlr_x11_touchpoint =
                get_touchpoint_from_x11_touch_id(output, (*ev_7).detail);
            if touchpoint_0.is_null() { return }
            send_touch_up_event(output, (*touchpoint_0).wayland_id,
                                (*ev_7).time);
            (*x11).time = (*ev_7).time;
            wl_list_remove(&mut (*touchpoint_0).link);
            free(touchpoint_0 as *mut libc::c_void);
        }
        19 => {
            let mut ev_8: *mut xcb_input_touch_update_event_t =
                event as *mut xcb_input_touch_update_event_t;
            output = get_x11_output_from_window_id(x11, (*ev_8).event);
            if output.is_null() { return }
            let mut touchpoint_1: *mut wlr_x11_touchpoint =
                get_touchpoint_from_x11_touch_id(output, (*ev_8).detail);
            if touchpoint_1.is_null() { return }
            send_touch_motion_event(output,
                                    ((*ev_8).event_x >> 16i32) as int16_t,
                                    ((*ev_8).event_y >> 16i32) as int16_t,
                                    (*touchpoint_1).wayland_id, (*ev_8).time);
            (*x11).time = (*ev_8).time
        }
        _ => { }
    };
}
unsafe extern "C" fn input_device_destroy(mut wlr_device:
                                              *mut wlr_input_device) {
    // Don't free the input device, it's on the stack
}
#[no_mangle]
pub static mut input_device_impl: wlr_input_device_impl =
    {
    
        {
            let mut init =
                wlr_input_device_impl{destroy:
                                          Some(input_device_destroy as
                                                   unsafe extern "C" fn(_:
                                                                            *mut wlr_input_device)
                                                       -> ()),};
            init
        }
};
unsafe extern "C" fn keyboard_destroy(mut wlr_keyboard: *mut wlr_keyboard) {
    // Don't free the keyboard, it's on the stack
}
#[no_mangle]
pub static mut keyboard_impl: wlr_keyboard_impl =
    {
    
        {
            let mut init =
                wlr_keyboard_impl{destroy:
                                      Some(keyboard_destroy as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_keyboard)
                                                   -> ()),
                                  led_update: None,};
            init
        }
};
unsafe extern "C" fn pointer_destroy(mut wlr_pointer: *mut wlr_pointer) {
    // Don't free the pointer, it's on the stack
}
#[no_mangle]
pub static mut pointer_impl: wlr_pointer_impl =
    {
    
        {
            let mut init =
                wlr_pointer_impl{destroy:
                                     Some(pointer_destroy as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_pointer)
                                                  -> ()),};
            init
        }
};
unsafe extern "C" fn touch_destroy(mut wlr_touch: *mut wlr_touch) {
    // Don't free the touch, it's on the stack
}
#[no_mangle]
pub static mut touch_impl: wlr_touch_impl =
    {
    
        {
            let mut init =
                wlr_touch_impl{destroy:
                                   Some(touch_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wlr_touch)
                                                -> ()),};
            init
        }
};
// 60 Hz
// wlr_x11_backend::outputs
// wlr_x11_touchpoint::link
// wlr_x11_output::touch_points
// wlr_x11_output::link
// The time we last received an event
#[no_mangle]
pub unsafe extern "C" fn update_x11_pointer_position(mut output:
                                                         *mut wlr_x11_output,
                                                     mut time:
                                                         xcb_timestamp_t) {
    let mut x11: *mut wlr_x11_backend = (*output).x11;
    let mut cookie: xcb_query_pointer_cookie_t =
        xcb_query_pointer((*x11).xcb, (*output).win);
    let mut reply: *mut xcb_query_pointer_reply_t =
        xcb_query_pointer_reply((*x11).xcb, cookie,
                                0 as *mut *mut xcb_generic_error_t);
    if reply.is_null() { return }
    send_pointer_position_event(output, (*reply).win_x, (*reply).win_y, time);
    free(reply as *mut libc::c_void);
}
/* *
 * Creates a new wlr_x11_backend. This backend will be created with no outputs;
 * you must use wlr_x11_output_create to add them.
 *
 * The `x11_display` argument is the name of the X Display socket. Set
 * to NULL for the default behaviour of XOpenDisplay.
 */
/* *
 * Adds a new output to this backend. You may remove outputs by destroying them.
 * Note that if called before initializing the backend, this will return NULL
 * and your outputs will be created during initialization (and given to you via
 * the output_add signal).
 */
/* *
 * True if the given backend is a wlr_x11_backend.
 */
/* *
 * True if the given input device is a wlr_x11_input_device.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_input_device_is_x11(mut wlr_dev:
                                                     *mut wlr_input_device)
 -> bool {
    return (*wlr_dev).impl_0 ==
               &input_device_impl as *const wlr_input_device_impl;
}
