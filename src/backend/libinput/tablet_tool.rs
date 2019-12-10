use libc;
extern "C" {
    pub type udev_device;
    pub type libinput_device;
    pub type libinput_tablet_tool;
    pub type libinput_event;
    pub type libinput_event_tablet_tool;
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
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn udev_device_get_syspath(udev_device: *mut udev_device)
     -> *const libc::c_char;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn libinput_event_get_tablet_tool_event(event: *mut libinput_event)
     -> *mut libinput_event_tablet_tool;
    #[no_mangle]
    fn libinput_event_tablet_tool_x_has_changed(event:
                                                    *mut libinput_event_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_tablet_tool_y_has_changed(event:
                                                    *mut libinput_event_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_tablet_tool_pressure_has_changed(event:
                                                           *mut libinput_event_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_tablet_tool_distance_has_changed(event:
                                                           *mut libinput_event_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_tablet_tool_tilt_x_has_changed(event:
                                                         *mut libinput_event_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_tablet_tool_tilt_y_has_changed(event:
                                                         *mut libinput_event_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_tablet_tool_rotation_has_changed(event:
                                                           *mut libinput_event_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_tablet_tool_slider_has_changed(event:
                                                         *mut libinput_event_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_tablet_tool_wheel_has_changed(event:
                                                        *mut libinput_event_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_dx(event:
                                             *mut libinput_event_tablet_tool)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_dy(event:
                                             *mut libinput_event_tablet_tool)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_pressure(event:
                                                   *mut libinput_event_tablet_tool)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_distance(event:
                                                   *mut libinput_event_tablet_tool)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_tilt_x(event:
                                                 *mut libinput_event_tablet_tool)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_tilt_y(event:
                                                 *mut libinput_event_tablet_tool)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_rotation(event:
                                                   *mut libinput_event_tablet_tool)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_slider_position(event:
                                                          *mut libinput_event_tablet_tool)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_wheel_delta(event:
                                                      *mut libinput_event_tablet_tool)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_x_transformed(event:
                                                        *mut libinput_event_tablet_tool,
                                                    width: uint32_t)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_y_transformed(event:
                                                        *mut libinput_event_tablet_tool,
                                                    height: uint32_t)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_tool(event:
                                               *mut libinput_event_tablet_tool)
     -> *mut libinput_tablet_tool;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_proximity_state(event:
                                                          *mut libinput_event_tablet_tool)
     -> libinput_tablet_tool_proximity_state;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_tip_state(event:
                                                    *mut libinput_event_tablet_tool)
     -> libinput_tablet_tool_tip_state;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_button(event:
                                                 *mut libinput_event_tablet_tool)
     -> uint32_t;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_button_state(event:
                                                       *mut libinput_event_tablet_tool)
     -> libinput_button_state;
    #[no_mangle]
    fn libinput_event_tablet_tool_get_time_usec(event:
                                                    *mut libinput_event_tablet_tool)
     -> uint64_t;
    #[no_mangle]
    fn libinput_tablet_tool_get_type(tool: *mut libinput_tablet_tool)
     -> libinput_tablet_tool_type;
    #[no_mangle]
    fn libinput_tablet_tool_get_tool_id(tool: *mut libinput_tablet_tool)
     -> uint64_t;
    #[no_mangle]
    fn libinput_tablet_tool_ref(tool: *mut libinput_tablet_tool)
     -> *mut libinput_tablet_tool;
    #[no_mangle]
    fn libinput_tablet_tool_has_pressure(tool: *mut libinput_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_tablet_tool_has_distance(tool: *mut libinput_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_tablet_tool_has_tilt(tool: *mut libinput_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_tablet_tool_has_rotation(tool: *mut libinput_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_tablet_tool_has_slider(tool: *mut libinput_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_tablet_tool_has_wheel(tool: *mut libinput_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_tablet_tool_is_unique(tool: *mut libinput_tablet_tool)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_tablet_tool_get_serial(tool: *mut libinput_tablet_tool)
     -> uint64_t;
    #[no_mangle]
    fn libinput_tablet_tool_get_user_data(tool: *mut libinput_tablet_tool)
     -> *mut libc::c_void;
    #[no_mangle]
    fn libinput_tablet_tool_set_user_data(tool: *mut libinput_tablet_tool,
                                          user_data: *mut libc::c_void);
    #[no_mangle]
    fn libinput_device_get_name(device: *mut libinput_device)
     -> *const libc::c_char;
    #[no_mangle]
    fn libinput_device_get_udev_device(device: *mut libinput_device)
     -> *mut udev_device;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wlr_tablet_init(tablet: *mut wlr_tablet, impl_0: *mut wlr_tablet_impl);
    /* *
 * Initialize a list. Returns true on success, false on failure.
 */
    #[no_mangle]
    fn wlr_list_init(list: *mut wlr_list) -> bool;
    /* *
 * Add `item` to the end of a list.
 * Returns: new list length or `-1` on failure.
 */
    #[no_mangle]
    fn wlr_list_push(list: *mut wlr_list, item: *mut libc::c_void) -> ssize_t;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn get_appropriate_device(desired_type: wlr_input_device_type,
                              device: *mut libinput_device)
     -> *mut wlr_input_device;
    #[no_mangle]
    fn usec_to_msec(usec: uint64_t) -> uint32_t;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type ssize_t = __ssize_t;
pub type libinput_button_state = libc::c_uint;
pub const LIBINPUT_BUTTON_STATE_PRESSED: libinput_button_state = 1;
pub const LIBINPUT_BUTTON_STATE_RELEASED: libinput_button_state = 0;
pub type libinput_tablet_tool_type = libc::c_uint;
pub const LIBINPUT_TABLET_TOOL_TYPE_TOTEM: libinput_tablet_tool_type = 8;
pub const LIBINPUT_TABLET_TOOL_TYPE_LENS: libinput_tablet_tool_type = 7;
pub const LIBINPUT_TABLET_TOOL_TYPE_MOUSE: libinput_tablet_tool_type = 6;
pub const LIBINPUT_TABLET_TOOL_TYPE_AIRBRUSH: libinput_tablet_tool_type = 5;
pub const LIBINPUT_TABLET_TOOL_TYPE_PENCIL: libinput_tablet_tool_type = 4;
pub const LIBINPUT_TABLET_TOOL_TYPE_BRUSH: libinput_tablet_tool_type = 3;
pub const LIBINPUT_TABLET_TOOL_TYPE_ERASER: libinput_tablet_tool_type = 2;
pub const LIBINPUT_TABLET_TOOL_TYPE_PEN: libinput_tablet_tool_type = 1;
pub type libinput_tablet_tool_proximity_state = libc::c_uint;
pub const LIBINPUT_TABLET_TOOL_PROXIMITY_STATE_IN:
          libinput_tablet_tool_proximity_state =
    1;
pub const LIBINPUT_TABLET_TOOL_PROXIMITY_STATE_OUT:
          libinput_tablet_tool_proximity_state =
    0;
pub type libinput_tablet_tool_tip_state = libc::c_uint;
pub const LIBINPUT_TABLET_TOOL_TIP_DOWN: libinput_tablet_tool_tip_state = 1;
pub const LIBINPUT_TABLET_TOOL_TIP_UP: libinput_tablet_tool_tip_state = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_tablet) -> ()>,
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
/* Note: these are circular dependencies */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
}
pub type wlr_tablet_tool_type = libc::c_uint;
pub const WLR_TABLET_TOOL_TYPE_TOTEM: wlr_tablet_tool_type = 8;
pub const WLR_TABLET_TOOL_TYPE_LENS: wlr_tablet_tool_type = 7;
pub const WLR_TABLET_TOOL_TYPE_MOUSE: wlr_tablet_tool_type = 6;
pub const WLR_TABLET_TOOL_TYPE_AIRBRUSH: wlr_tablet_tool_type = 5;
pub const WLR_TABLET_TOOL_TYPE_PENCIL: wlr_tablet_tool_type = 4;
pub const WLR_TABLET_TOOL_TYPE_BRUSH: wlr_tablet_tool_type = 3;
pub const WLR_TABLET_TOOL_TYPE_ERASER: wlr_tablet_tool_type = 2;
pub const WLR_TABLET_TOOL_TYPE_PEN: wlr_tablet_tool_type = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
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
    pub events: C2RustUnnamed_8,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_tablet_tool_button {
    pub device: *mut wlr_input_device,
    pub tool: *mut wlr_tablet_tool,
    pub time_msec: uint32_t,
    pub button: uint32_t,
    pub state: wlr_button_state,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_libinput_tablet {
    pub wlr_tablet: wlr_tablet,
    pub tools: wl_list,
    // tablet_tool_list_elem::link
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_libinput_tablet_tool {
    pub wlr_tool: wlr_tablet_tool,
    pub libinput_tool: *mut libinput_tablet_tool,
    pub unique: bool,
    pub pad_refs: size_t,
}
// TODO: Maybe this should be a wlr_list? Do we keep it, or want to get rid of
// it?
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct tablet_tool_list_elem {
    pub link: wl_list,
    pub tool: *mut wlr_libinput_tablet_tool,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
unsafe extern "C" fn tablet_is_libinput(mut tablet: *mut wlr_tablet) -> bool {
    return (*tablet).impl_0 == &mut tablet_impl as *mut wlr_tablet_impl;
}
unsafe extern "C" fn destroy_tool(mut tool: *mut wlr_libinput_tablet_tool) {
    wlr_signal_emit_safe(&mut (*tool).wlr_tool.events.destroy,
                         &mut (*tool).wlr_tool as *mut wlr_tablet_tool as
                             *mut libc::c_void);
    libinput_tablet_tool_ref((*tool).libinput_tool);
    libinput_tablet_tool_set_user_data((*tool).libinput_tool,
                                       0 as *mut libc::c_void);
    free(tool as *mut libc::c_void);
}
unsafe extern "C" fn destroy_tablet(mut wlr_tablet: *mut wlr_tablet) {
    if tablet_is_libinput(wlr_tablet) as libc::c_int != 0 {
    } else {
        __assert_fail(b"tablet_is_libinput(wlr_tablet)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/libinput/tablet_tool.c\x00" as *const u8 as
                          *const libc::c_char, 55i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"void destroy_tablet(struct wlr_tablet *)\x00")).as_ptr());
    };
    let mut tablet: *mut wlr_libinput_tablet =
        (wlr_tablet as *mut libc::c_char).offset(-0) as
            *mut wlr_libinput_tablet;
    let mut pos: *mut tablet_tool_list_elem = 0 as *mut tablet_tool_list_elem;
    let mut tmp: *mut tablet_tool_list_elem = 0 as *mut tablet_tool_list_elem;
    pos =
        ((*tablet).tools.next as *mut libc::c_char).offset(-0) as
            *mut tablet_tool_list_elem;
    tmp =
        ((*pos).link.next as *mut libc::c_char).offset(-0) as
            *mut tablet_tool_list_elem;
    while &mut (*pos).link as *mut wl_list !=
              &mut (*tablet).tools as *mut wl_list {
        let mut tool: *mut wlr_libinput_tablet_tool = (*pos).tool;
        wl_list_remove(&mut (*pos).link);
        free(pos as *mut libc::c_void);
        (*tool).pad_refs = (*tool).pad_refs.wrapping_sub(1);
        if (*tool).pad_refs == 0i32 as libc::c_ulong { destroy_tool(tool); }
        pos = tmp;
        tmp =
            ((*pos).link.next as *mut libc::c_char).offset(-0) as
                *mut tablet_tool_list_elem
    }
    free(tablet as *mut libc::c_void);
}
static mut tablet_impl: wlr_tablet_impl =
    unsafe {
        {
            let mut init =
                wlr_tablet_impl{destroy:
                                    Some(destroy_tablet as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_tablet)
                                                 -> ()),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn create_libinput_tablet(mut libinput_dev:
                                                    *mut libinput_device)
 -> *mut wlr_tablet {
    if !libinput_dev.is_null() {
    } else {
        __assert_fail(b"libinput_dev\x00" as *const u8 as *const libc::c_char,
                      b"../backend/libinput/tablet_tool.c\x00" as *const u8 as
                          *const libc::c_char, 80i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"struct wlr_tablet *create_libinput_tablet(struct libinput_device *)\x00")).as_ptr());
    };
    let mut libinput_tablet: *mut wlr_libinput_tablet =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_libinput_tablet>() as libc::c_ulong)
            as *mut wlr_libinput_tablet;
    if libinput_tablet.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unable to allocate wlr_tablet_tool\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/libinput/tablet_tool.c\x00" as *const u8 as
                     *const libc::c_char, 84i32);
        return 0 as *mut wlr_tablet
    }
    let mut wlr_tablet: *mut wlr_tablet = &mut (*libinput_tablet).wlr_tablet;
    wlr_list_init(&mut (*wlr_tablet).paths);
    let mut udev: *mut udev_device =
        libinput_device_get_udev_device(libinput_dev);
    wlr_list_push(&mut (*wlr_tablet).paths,
                  strdup(udev_device_get_syspath(udev)) as *mut libc::c_void);
    (*wlr_tablet).name = strdup(libinput_device_get_name(libinput_dev));
    wl_list_init(&mut (*libinput_tablet).tools);
    wlr_tablet_init(wlr_tablet, &mut tablet_impl);
    return wlr_tablet;
}
unsafe extern "C" fn wlr_type_from_libinput_type(mut value:
                                                     libinput_tablet_tool_type)
 -> wlr_tablet_tool_type {
    match value as libc::c_uint {
        1 => { return WLR_TABLET_TOOL_TYPE_PEN }
        2 => { return WLR_TABLET_TOOL_TYPE_ERASER }
        3 => { return WLR_TABLET_TOOL_TYPE_BRUSH }
        4 => { return WLR_TABLET_TOOL_TYPE_PENCIL }
        5 => { return WLR_TABLET_TOOL_TYPE_AIRBRUSH }
        6 => { return WLR_TABLET_TOOL_TYPE_MOUSE }
        7 => { return WLR_TABLET_TOOL_TYPE_LENS }
        8 => { return WLR_TABLET_TOOL_TYPE_TOTEM }
        _ => { }
    }
    if 0i32 != 0 &&
           !(b"UNREACHABLE\x00" as *const u8 as *const libc::c_char).is_null()
       {
    } else {
        __assert_fail(b"false && \"UNREACHABLE\"\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/libinput/tablet_tool.c\x00" as *const u8 as
                          *const libc::c_char, 122i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 86],
                                                &[libc::c_char; 86]>(b"enum wlr_tablet_tool_type wlr_type_from_libinput_type(enum libinput_tablet_tool_type)\x00")).as_ptr());
    };
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn get_wlr_tablet_tool(mut tool: *mut libinput_tablet_tool)
 -> *mut wlr_libinput_tablet_tool {
    let mut ret: *mut wlr_libinput_tablet_tool =
        libinput_tablet_tool_get_user_data(tool) as
            *mut wlr_libinput_tablet_tool;
    if !ret.is_null() { return ret }
    ret =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_libinput_tablet_tool>() as
                   libc::c_ulong) as *mut wlr_libinput_tablet_tool;
    if ret.is_null() { return 0 as *mut wlr_libinput_tablet_tool }
    (*ret).libinput_tool = libinput_tablet_tool_ref(tool);
    (*ret).wlr_tool.pressure = libinput_tablet_tool_has_pressure(tool) != 0;
    (*ret).wlr_tool.distance = libinput_tablet_tool_has_distance(tool) != 0;
    (*ret).wlr_tool.tilt = libinput_tablet_tool_has_tilt(tool) != 0;
    (*ret).wlr_tool.rotation = libinput_tablet_tool_has_rotation(tool) != 0;
    (*ret).wlr_tool.slider = libinput_tablet_tool_has_slider(tool) != 0;
    (*ret).wlr_tool.wheel = libinput_tablet_tool_has_wheel(tool) != 0;
    (*ret).wlr_tool.hardware_serial = libinput_tablet_tool_get_serial(tool);
    (*ret).wlr_tool.hardware_wacom = libinput_tablet_tool_get_tool_id(tool);
    (*ret).wlr_tool.type_0 =
        wlr_type_from_libinput_type(libinput_tablet_tool_get_type(tool));
    (*ret).unique = libinput_tablet_tool_is_unique(tool) != 0;
    wl_signal_init(&mut (*ret).wlr_tool.events.destroy);
    libinput_tablet_tool_set_user_data(tool, ret as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn ensure_tool_reference(mut tool:
                                               *mut wlr_libinput_tablet_tool,
                                           mut wlr_dev: *mut wlr_tablet) {
    if tablet_is_libinput(wlr_dev) as libc::c_int != 0 {
    } else {
        __assert_fail(b"tablet_is_libinput(wlr_dev)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/libinput/tablet_tool.c\x00" as *const u8 as
                          *const libc::c_char, 162i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"void ensure_tool_reference(struct wlr_libinput_tablet_tool *, struct wlr_tablet *)\x00")).as_ptr());
    };
    let mut tablet: *mut wlr_libinput_tablet =
        (wlr_dev as *mut libc::c_char).offset(-0) as *mut wlr_libinput_tablet;
    let mut pos: *mut tablet_tool_list_elem = 0 as *mut tablet_tool_list_elem;
    pos =
        ((*tablet).tools.next as *mut libc::c_char).offset(-0) as
            *mut tablet_tool_list_elem;
    while &mut (*pos).link as *mut wl_list !=
              &mut (*tablet).tools as *mut wl_list {
        if (*pos).tool == tool {
            // We already have a ref
            // XXX: We *could* optimize the tool to the front of
			// the list here, since we will probably get the next
			// couple of events from the same tool.
			// BUT the list should always be rather short (probably
			// single digit amount of tools) so it might be more
			// work than it saves
            return
        }
        pos =
            ((*pos).link.next as *mut libc::c_char).offset(-0) as
                *mut tablet_tool_list_elem
    }
    let mut new: *mut tablet_tool_list_elem =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<tablet_tool_list_elem>() as
                   libc::c_ulong) as *mut tablet_tool_list_elem;
    if new.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate memory for tracking tablet tool\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/tablet_tool.c\x00" as *const u8 as
                     *const libc::c_char, 182i32);
        return
    }
    (*new).tool = tool;
    wl_list_insert(&mut (*tablet).tools, &mut (*new).link);
    (*tool).pad_refs = (*tool).pad_refs.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn handle_tablet_tool_axis(mut event:
                                                     *mut libinput_event,
                                                 mut libinput_dev:
                                                     *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TABLET_TOOL, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a tablet tool event for a device with no tablet tools?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/tablet_tool.c\x00" as *const u8 as
                     *const libc::c_char, 197i32);
        return
    }
    let mut tevent: *mut libinput_event_tablet_tool =
        libinput_event_get_tablet_tool_event(event);
    let mut wlr_event: wlr_event_tablet_tool_axis =
        {
            let mut init =
                wlr_event_tablet_tool_axis{device: 0 as *mut wlr_input_device,
                                           tool: 0 as *mut wlr_tablet_tool,
                                           time_msec: 0,
                                           updated_axes: 0,
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
    let mut tool: *mut wlr_libinput_tablet_tool =
        get_wlr_tablet_tool(libinput_event_tablet_tool_get_tool(tevent));
    ensure_tool_reference(tool, (*wlr_dev).c2rust_unnamed.tablet);
    wlr_event.device = wlr_dev;
    wlr_event.tool = &mut (*tool).wlr_tool;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_tablet_tool_get_time_usec(tevent));
    if libinput_event_tablet_tool_x_has_changed(tevent) != 0 {
        wlr_event.updated_axes |=
            WLR_TABLET_TOOL_AXIS_X as libc::c_int as libc::c_uint;
        wlr_event.x =
            libinput_event_tablet_tool_get_x_transformed(tevent,
                                                         1i32 as uint32_t);
        wlr_event.dx = libinput_event_tablet_tool_get_dx(tevent)
    }
    if libinput_event_tablet_tool_y_has_changed(tevent) != 0 {
        wlr_event.updated_axes |=
            WLR_TABLET_TOOL_AXIS_Y as libc::c_int as libc::c_uint;
        wlr_event.y =
            libinput_event_tablet_tool_get_y_transformed(tevent,
                                                         1i32 as uint32_t);
        wlr_event.dy = libinput_event_tablet_tool_get_dy(tevent)
    }
    if libinput_event_tablet_tool_pressure_has_changed(tevent) != 0 {
        wlr_event.updated_axes |=
            WLR_TABLET_TOOL_AXIS_PRESSURE as libc::c_int as libc::c_uint;
        wlr_event.pressure = libinput_event_tablet_tool_get_pressure(tevent)
    }
    if libinput_event_tablet_tool_distance_has_changed(tevent) != 0 {
        wlr_event.updated_axes |=
            WLR_TABLET_TOOL_AXIS_DISTANCE as libc::c_int as libc::c_uint;
        wlr_event.distance = libinput_event_tablet_tool_get_distance(tevent)
    }
    if libinput_event_tablet_tool_tilt_x_has_changed(tevent) != 0 {
        wlr_event.updated_axes |=
            WLR_TABLET_TOOL_AXIS_TILT_X as libc::c_int as libc::c_uint;
        wlr_event.tilt_x = libinput_event_tablet_tool_get_tilt_x(tevent)
    }
    if libinput_event_tablet_tool_tilt_y_has_changed(tevent) != 0 {
        wlr_event.updated_axes |=
            WLR_TABLET_TOOL_AXIS_TILT_Y as libc::c_int as libc::c_uint;
        wlr_event.tilt_y = libinput_event_tablet_tool_get_tilt_y(tevent)
    }
    if libinput_event_tablet_tool_rotation_has_changed(tevent) != 0 {
        wlr_event.updated_axes |=
            WLR_TABLET_TOOL_AXIS_ROTATION as libc::c_int as libc::c_uint;
        wlr_event.rotation = libinput_event_tablet_tool_get_rotation(tevent)
    }
    if libinput_event_tablet_tool_slider_has_changed(tevent) != 0 {
        wlr_event.updated_axes |=
            WLR_TABLET_TOOL_AXIS_SLIDER as libc::c_int as libc::c_uint;
        wlr_event.slider =
            libinput_event_tablet_tool_get_slider_position(tevent)
    }
    if libinput_event_tablet_tool_wheel_has_changed(tevent) != 0 {
        wlr_event.updated_axes |=
            WLR_TABLET_TOOL_AXIS_WHEEL as libc::c_int as libc::c_uint;
        wlr_event.wheel_delta =
            libinput_event_tablet_tool_get_wheel_delta(tevent)
    }
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.tablet).events.axis,
                         &mut wlr_event as *mut wlr_event_tablet_tool_axis as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_tablet_tool_proximity(mut event:
                                                          *mut libinput_event,
                                                      mut libinput_dev:
                                                          *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TABLET_TOOL, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a tablet tool event for a device with no tablet tools?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/tablet_tool.c\x00" as *const u8 as
                     *const libc::c_char, 258i32);
        return
    }
    let mut tevent: *mut libinput_event_tablet_tool =
        libinput_event_get_tablet_tool_event(event);
    let mut wlr_event: wlr_event_tablet_tool_proximity =
        {
            let mut init =
                wlr_event_tablet_tool_proximity{device:
                                                    0 as
                                                        *mut wlr_input_device,
                                                tool:
                                                    0 as *mut wlr_tablet_tool,
                                                time_msec: 0,
                                                x: 0.,
                                                y: 0.,
                                                state:
                                                    WLR_TABLET_TOOL_PROXIMITY_OUT,};
            init
        };
    let mut tool: *mut wlr_libinput_tablet_tool =
        get_wlr_tablet_tool(libinput_event_tablet_tool_get_tool(tevent));
    ensure_tool_reference(tool, (*wlr_dev).c2rust_unnamed.tablet);
    wlr_event.tool = &mut (*tool).wlr_tool;
    wlr_event.device = wlr_dev;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_tablet_tool_get_time_usec(tevent));
    match libinput_event_tablet_tool_get_proximity_state(tevent) as
              libc::c_uint {
        0 => { wlr_event.state = WLR_TABLET_TOOL_PROXIMITY_OUT }
        1 => { wlr_event.state = WLR_TABLET_TOOL_PROXIMITY_IN }
        _ => { }
    }
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.tablet).events.proximity,
                         &mut wlr_event as
                             *mut wlr_event_tablet_tool_proximity as
                             *mut libc::c_void);
    if libinput_event_tablet_tool_get_proximity_state(tevent) as libc::c_uint
           ==
           LIBINPUT_TABLET_TOOL_PROXIMITY_STATE_IN as libc::c_int as
               libc::c_uint {
        handle_tablet_tool_axis(event, libinput_dev);
    }
    // If the tool is not unique, libinput will not find it again after the
	// proximity out, so we should destroy it
    if !(*tool).unique &&
           libinput_event_tablet_tool_get_proximity_state(tevent) as
               libc::c_uint ==
               LIBINPUT_TABLET_TOOL_PROXIMITY_STATE_OUT as libc::c_int as
                   libc::c_uint {
        // The tool isn't unique, it can't be on multiple tablets
        if (*tool).pad_refs == 1i32 as libc::c_ulong {
        } else {
            __assert_fail(b"tool->pad_refs == 1\x00" as *const u8 as
                              *const libc::c_char,
                          b"../backend/libinput/tablet_tool.c\x00" as
                              *const u8 as *const libc::c_char,
                          293i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 85],
                                                    &[libc::c_char; 85]>(b"void handle_tablet_tool_proximity(struct libinput_event *, struct libinput_device *)\x00")).as_ptr());
        };
        if tablet_is_libinput((*wlr_dev).c2rust_unnamed.tablet) as libc::c_int
               != 0 {
        } else {
            __assert_fail(b"tablet_is_libinput(wlr_dev->tablet)\x00" as
                              *const u8 as *const libc::c_char,
                          b"../backend/libinput/tablet_tool.c\x00" as
                              *const u8 as *const libc::c_char,
                          294i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 85],
                                                    &[libc::c_char; 85]>(b"void handle_tablet_tool_proximity(struct libinput_event *, struct libinput_device *)\x00")).as_ptr());
        };
        let mut tablet: *mut wlr_libinput_tablet =
            ((*wlr_dev).c2rust_unnamed.tablet as *mut libc::c_char).offset(-0)
                as *mut wlr_libinput_tablet;
        let mut pos: *mut tablet_tool_list_elem =
            0 as *mut tablet_tool_list_elem;
        let mut tmp: *mut tablet_tool_list_elem =
            0 as *mut tablet_tool_list_elem;
        pos =
            ((*tablet).tools.next as *mut libc::c_char).offset(-0) as
                *mut tablet_tool_list_elem;
        tmp =
            ((*pos).link.next as *mut libc::c_char).offset(-0) as
                *mut tablet_tool_list_elem;
        while &mut (*pos).link as *mut wl_list !=
                  &mut (*tablet).tools as *mut wl_list {
            if (*pos).tool == tool {
                wl_list_remove(&mut (*pos).link);
                free(pos as *mut libc::c_void);
                break ;
            } else {
                pos = tmp;
                tmp =
                    ((*pos).link.next as *mut libc::c_char).offset(-0) as
                        *mut tablet_tool_list_elem
            }
        }
        destroy_tool(tool);
    };
}
#[no_mangle]
pub unsafe extern "C" fn handle_tablet_tool_tip(mut event:
                                                    *mut libinput_event,
                                                mut libinput_dev:
                                                    *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TABLET_TOOL, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a tablet tool event for a device with no tablet tools?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/tablet_tool.c\x00" as *const u8 as
                     *const libc::c_char, 318i32);
        return
    }
    handle_tablet_tool_axis(event, libinput_dev);
    let mut tevent: *mut libinput_event_tablet_tool =
        libinput_event_get_tablet_tool_event(event);
    let mut wlr_event: wlr_event_tablet_tool_tip =
        {
            let mut init =
                wlr_event_tablet_tool_tip{device: 0 as *mut wlr_input_device,
                                          tool: 0 as *mut wlr_tablet_tool,
                                          time_msec: 0,
                                          x: 0.,
                                          y: 0.,
                                          state: WLR_TABLET_TOOL_TIP_UP,};
            init
        };
    let mut tool: *mut wlr_libinput_tablet_tool =
        get_wlr_tablet_tool(libinput_event_tablet_tool_get_tool(tevent));
    ensure_tool_reference(tool, (*wlr_dev).c2rust_unnamed.tablet);
    wlr_event.device = wlr_dev;
    wlr_event.tool = &mut (*tool).wlr_tool;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_tablet_tool_get_time_usec(tevent));
    match libinput_event_tablet_tool_get_tip_state(tevent) as libc::c_uint {
        0 => { wlr_event.state = WLR_TABLET_TOOL_TIP_UP }
        1 => { wlr_event.state = WLR_TABLET_TOOL_TIP_DOWN }
        _ => { }
    }
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.tablet).events.tip,
                         &mut wlr_event as *mut wlr_event_tablet_tool_tip as
                             *mut libc::c_void);
}
// list of struct wl_list
#[no_mangle]
pub unsafe extern "C" fn handle_tablet_tool_button(mut event:
                                                       *mut libinput_event,
                                                   mut libinput_dev:
                                                       *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TABLET_TOOL, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a tablet tool event for a device with no tablet tools?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/tablet_tool.c\x00" as *const u8 as
                     *const libc::c_char, 350i32);
        return
    }
    handle_tablet_tool_axis(event, libinput_dev);
    let mut tevent: *mut libinput_event_tablet_tool =
        libinput_event_get_tablet_tool_event(event);
    let mut wlr_event: wlr_event_tablet_tool_button =
        {
            let mut init =
                wlr_event_tablet_tool_button{device:
                                                 0 as *mut wlr_input_device,
                                             tool: 0 as *mut wlr_tablet_tool,
                                             time_msec: 0,
                                             button: 0,
                                             state: WLR_BUTTON_RELEASED,};
            init
        };
    let mut tool: *mut wlr_libinput_tablet_tool =
        get_wlr_tablet_tool(libinput_event_tablet_tool_get_tool(tevent));
    ensure_tool_reference(tool, (*wlr_dev).c2rust_unnamed.tablet);
    wlr_event.device = wlr_dev;
    wlr_event.tool = &mut (*tool).wlr_tool;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_tablet_tool_get_time_usec(tevent));
    wlr_event.button = libinput_event_tablet_tool_get_button(tevent);
    match libinput_event_tablet_tool_get_button_state(tevent) as libc::c_uint
        {
        0 => { wlr_event.state = WLR_BUTTON_RELEASED }
        1 => { wlr_event.state = WLR_BUTTON_PRESSED }
        _ => { }
    }
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.tablet).events.button,
                         &mut wlr_event as *mut wlr_event_tablet_tool_button
                             as *mut libc::c_void);
}
