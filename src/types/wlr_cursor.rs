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
    pub type wlr_surface;
    pub type wlr_backend;
    pub type wlr_output_impl;
    pub type wlr_output_layout_state;
    pub type wlr_output_layout_output_state;
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
    fn wlr_box_closest_point(box_0: *const wlr_box, x: libc::c_double,
                             y: libc::c_double, dest_x: *mut libc::c_double,
                             dest_y: *mut libc::c_double);
    #[no_mangle]
    fn wlr_box_contains_point(box_0: *const wlr_box, x: libc::c_double,
                              y: libc::c_double) -> bool;
    #[no_mangle]
    fn wlr_box_empty(box_0: *const wlr_box) -> bool;
    #[no_mangle]
    fn wlr_output_layout_get_box(layout: *mut wlr_output_layout,
                                 reference: *mut wlr_output) -> *mut wlr_box;
    #[no_mangle]
    fn wlr_output_layout_closest_point(layout: *mut wlr_output_layout,
                                       reference: *mut wlr_output,
                                       lx: libc::c_double, ly: libc::c_double,
                                       dest_lx: *mut libc::c_double,
                                       dest_ly: *mut libc::c_double);
    #[no_mangle]
    fn wlr_output_layout_contains_point(layout: *mut wlr_output_layout,
                                        reference: *mut wlr_output,
                                        lx: libc::c_int, ly: libc::c_int)
     -> bool;
    #[no_mangle]
    fn wlr_output_layout_output_coords(layout: *mut wlr_output_layout,
                                       reference: *mut wlr_output,
                                       lx: *mut libc::c_double,
                                       ly: *mut libc::c_double);
    #[no_mangle]
    fn wlr_output_cursor_destroy(cursor: *mut wlr_output_cursor);
    #[no_mangle]
    fn wlr_output_cursor_move(cursor: *mut wlr_output_cursor,
                              x: libc::c_double, y: libc::c_double) -> bool;
    #[no_mangle]
    fn wlr_output_cursor_set_surface(cursor: *mut wlr_output_cursor,
                                     surface: *mut wlr_surface,
                                     hotspot_x: int32_t, hotspot_y: int32_t);
    #[no_mangle]
    fn wlr_output_cursor_set_image(cursor: *mut wlr_output_cursor,
                                   pixels: *const uint8_t, stride: int32_t,
                                   width: uint32_t, height: uint32_t,
                                   hotspot_x: int32_t, hotspot_y: int32_t)
     -> bool;
    #[no_mangle]
    fn wlr_output_cursor_create(output: *mut wlr_output)
     -> *mut wlr_output_cursor;
    // Will log all messages less than or equal to `verbosity`
// If `callback` is NULL, wlr will use its default logger.
// The function can be called multiple times to update the verbosity or
// callback function.
    // Returns the log verbosity provided to wlr_log_init
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct wlr_event_pointer_motion {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub delta_x: libc::c_double,
    pub delta_y: libc::c_double,
    pub unaccel_dx: libc::c_double,
    pub unaccel_dy: libc::c_double,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_motion_absolute {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_axis {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub source: wlr_axis_source,
    pub orientation: wlr_axis_orientation,
    pub delta: libc::c_double,
    pub delta_discrete: int32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_swipe_begin {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_swipe_update {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
    pub dx: libc::c_double,
    pub dy: libc::c_double,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_swipe_end {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub cancelled: bool,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_pinch_begin {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_pinch_update {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
    pub dx: libc::c_double,
    pub dy: libc::c_double,
    pub scale: libc::c_double,
    pub rotation: libc::c_double,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_pinch_end {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub cancelled: bool,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_touch_down {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_touch_up {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_touch_motion {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_touch_cancel {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
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
    pub events: C2RustUnnamed_9,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub destroy: wl_signal,
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
    pub events: C2RustUnnamed_10,
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
pub struct C2RustUnnamed_10 {
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
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub state: *mut wlr_output_layout_state,
    pub events: C2RustUnnamed_11,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_layout_output {
    pub output: *mut wlr_output,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub link: wl_list,
    pub state: *mut wlr_output_layout_output_state,
    pub events: C2RustUnnamed_12,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_cursor_state {
    pub cursor: *mut wlr_cursor,
    pub devices: wl_list,
    pub output_cursors: wl_list,
    pub layout: *mut wlr_output_layout,
    pub mapped_output: *mut wlr_output,
    pub mapped_box: *mut wlr_box,
    pub layout_add: wl_listener,
    pub layout_change: wl_listener,
    pub layout_destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_cursor {
    pub state: *mut wlr_cursor_state,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub events: C2RustUnnamed_13,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
    pub touch_up: wl_signal,
    pub touch_down: wl_signal,
    pub touch_motion: wl_signal,
    pub touch_cancel: wl_signal,
    pub tablet_tool_axis: wl_signal,
    pub tablet_tool_proximity: wl_signal,
    pub tablet_tool_tip: wl_signal,
    pub tablet_tool_button: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_cursor_device {
    pub cursor: *mut wlr_cursor,
    pub device: *mut wlr_input_device,
    pub link: wl_list,
    pub mapped_output: *mut wlr_output,
    pub mapped_box: *mut wlr_box,
    pub motion: wl_listener,
    pub motion_absolute: wl_listener,
    pub button: wl_listener,
    pub axis: wl_listener,
    pub frame: wl_listener,
    pub swipe_begin: wl_listener,
    pub swipe_update: wl_listener,
    pub swipe_end: wl_listener,
    pub pinch_begin: wl_listener,
    pub pinch_update: wl_listener,
    pub pinch_end: wl_listener,
    pub touch_down: wl_listener,
    pub touch_up: wl_listener,
    pub touch_motion: wl_listener,
    pub touch_cancel: wl_listener,
    pub tablet_tool_axis: wl_listener,
    pub tablet_tool_proximity: wl_listener,
    pub tablet_tool_tip: wl_listener,
    pub tablet_tool_button: wl_listener,
    pub destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_cursor_output_cursor {
    pub cursor: *mut wlr_cursor,
    pub output_cursor: *mut wlr_output_cursor,
    pub link: wl_list,
    pub layout_output_destroy: wl_listener,
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
pub unsafe extern "C" fn wlr_cursor_create() -> *mut wlr_cursor {
    let mut cur: *mut wlr_cursor =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_cursor>() as libc::c_ulong) as
            *mut wlr_cursor;
    if cur.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate wlr_cursor\x00" as *const u8 as
                     *const libc::c_char,
                 b"../types/wlr_cursor.c\x00" as *const u8 as
                     *const libc::c_char, 69i32);
        return 0 as *mut wlr_cursor
    }
    (*cur).state =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_cursor_state>() as libc::c_ulong) as
            *mut wlr_cursor_state;
    if (*cur).state.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate wlr_cursor_state\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_cursor.c\x00" as *const u8 as
                     *const libc::c_char, 75i32);
        free(cur as *mut libc::c_void);
        return 0 as *mut wlr_cursor
    }
    (*(*cur).state).cursor = cur;
    (*(*cur).state).mapped_output = 0 as *mut wlr_output;
    wl_list_init(&mut (*(*cur).state).devices);
    wl_list_init(&mut (*(*cur).state).output_cursors);
    // pointer signals
    wl_signal_init(&mut (*cur).events.motion);
    wl_signal_init(&mut (*cur).events.motion_absolute);
    wl_signal_init(&mut (*cur).events.button);
    wl_signal_init(&mut (*cur).events.axis);
    wl_signal_init(&mut (*cur).events.frame);
    wl_signal_init(&mut (*cur).events.swipe_begin);
    wl_signal_init(&mut (*cur).events.swipe_update);
    wl_signal_init(&mut (*cur).events.swipe_end);
    wl_signal_init(&mut (*cur).events.pinch_begin);
    wl_signal_init(&mut (*cur).events.pinch_update);
    wl_signal_init(&mut (*cur).events.pinch_end);
    // touch signals
    wl_signal_init(&mut (*cur).events.touch_up);
    wl_signal_init(&mut (*cur).events.touch_down);
    wl_signal_init(&mut (*cur).events.touch_motion);
    wl_signal_init(&mut (*cur).events.touch_cancel);
    // tablet tool signals
    wl_signal_init(&mut (*cur).events.tablet_tool_tip);
    wl_signal_init(&mut (*cur).events.tablet_tool_axis);
    wl_signal_init(&mut (*cur).events.tablet_tool_button);
    wl_signal_init(&mut (*cur).events.tablet_tool_proximity);
    (*cur).x = 100i32 as libc::c_double;
    (*cur).y = 100i32 as libc::c_double;
    return cur;
}
unsafe extern "C" fn output_cursor_destroy(mut output_cursor:
                                               *mut wlr_cursor_output_cursor) {
    wl_list_remove(&mut (*output_cursor).layout_output_destroy.link);
    wl_list_remove(&mut (*output_cursor).link);
    wlr_output_cursor_destroy((*output_cursor).output_cursor);
    free(output_cursor as *mut libc::c_void);
}
unsafe extern "C" fn cursor_detach_output_layout(mut cur: *mut wlr_cursor) {
    if (*(*cur).state).layout.is_null() { return }
    let mut output_cursor: *mut wlr_cursor_output_cursor =
        0 as *mut wlr_cursor_output_cursor;
    let mut tmp: *mut wlr_cursor_output_cursor =
        0 as *mut wlr_cursor_output_cursor;
    output_cursor =
        ((*(*cur).state).output_cursors.next as *mut libc::c_char).offset(-16)
            as *mut wlr_cursor_output_cursor;
    tmp =
        ((*output_cursor).link.next as *mut libc::c_char).offset(-16) as
            *mut wlr_cursor_output_cursor;
    while &mut (*output_cursor).link as *mut wl_list !=
              &mut (*(*cur).state).output_cursors as *mut wl_list {
        output_cursor_destroy(output_cursor);
        output_cursor = tmp;
        tmp =
            ((*output_cursor).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_cursor_output_cursor
    }
    wl_list_remove(&mut (*(*cur).state).layout_destroy.link);
    wl_list_remove(&mut (*(*cur).state).layout_change.link);
    wl_list_remove(&mut (*(*cur).state).layout_add.link);
    (*(*cur).state).layout = 0 as *mut wlr_output_layout;
}
unsafe extern "C" fn cursor_device_destroy(mut c_device:
                                               *mut wlr_cursor_device) {
    let mut dev: *mut wlr_input_device = (*c_device).device;
    if (*dev).type_0 as libc::c_uint ==
           WLR_INPUT_DEVICE_POINTER as libc::c_int as libc::c_uint {
        wl_list_remove(&mut (*c_device).motion.link);
        wl_list_remove(&mut (*c_device).motion_absolute.link);
        wl_list_remove(&mut (*c_device).button.link);
        wl_list_remove(&mut (*c_device).axis.link);
        wl_list_remove(&mut (*c_device).frame.link);
        wl_list_remove(&mut (*c_device).swipe_begin.link);
        wl_list_remove(&mut (*c_device).swipe_update.link);
        wl_list_remove(&mut (*c_device).swipe_end.link);
        wl_list_remove(&mut (*c_device).pinch_begin.link);
        wl_list_remove(&mut (*c_device).pinch_update.link);
        wl_list_remove(&mut (*c_device).pinch_end.link);
    } else if (*dev).type_0 as libc::c_uint ==
                  WLR_INPUT_DEVICE_TOUCH as libc::c_int as libc::c_uint {
        wl_list_remove(&mut (*c_device).touch_down.link);
        wl_list_remove(&mut (*c_device).touch_up.link);
        wl_list_remove(&mut (*c_device).touch_motion.link);
        wl_list_remove(&mut (*c_device).touch_cancel.link);
    } else if (*dev).type_0 as libc::c_uint ==
                  WLR_INPUT_DEVICE_TABLET_TOOL as libc::c_int as libc::c_uint
     {
        wl_list_remove(&mut (*c_device).tablet_tool_axis.link);
        wl_list_remove(&mut (*c_device).tablet_tool_proximity.link);
        wl_list_remove(&mut (*c_device).tablet_tool_tip.link);
        wl_list_remove(&mut (*c_device).tablet_tool_button.link);
    }
    wl_list_remove(&mut (*c_device).link);
    wl_list_remove(&mut (*c_device).destroy.link);
    free(c_device as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_destroy(mut cur: *mut wlr_cursor) {
    cursor_detach_output_layout(cur);
    let mut device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    let mut device_tmp: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    device =
        ((*(*cur).state).devices.next as *mut libc::c_char).offset(-16) as
            *mut wlr_cursor_device;
    device_tmp =
        ((*device).link.next as *mut libc::c_char).offset(-16) as
            *mut wlr_cursor_device;
    while &mut (*device).link as *mut wl_list !=
              &mut (*(*cur).state).devices as *mut wl_list {
        cursor_device_destroy(device);
        device = device_tmp;
        device_tmp =
            ((*device).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_cursor_device
    }
    free((*cur).state as *mut libc::c_void);
    free(cur as *mut libc::c_void);
}
unsafe extern "C" fn get_cursor_device(mut cur: *mut wlr_cursor,
                                       mut device: *mut wlr_input_device)
 -> *mut wlr_cursor_device {
    let mut c_device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    let mut ret: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    c_device =
        ((*(*cur).state).devices.next as *mut libc::c_char).offset(-16) as
            *mut wlr_cursor_device;
    while &mut (*c_device).link as *mut wl_list !=
              &mut (*(*cur).state).devices as *mut wl_list {
        if (*c_device).device == device {
            ret = c_device;
            break ;
        } else {
            c_device =
                ((*c_device).link.next as *mut libc::c_char).offset(-16) as
                    *mut wlr_cursor_device
        }
    }
    return ret;
}
unsafe extern "C" fn cursor_warp_unchecked(mut cur: *mut wlr_cursor,
                                           mut lx: libc::c_double,
                                           mut ly: libc::c_double) {
    if !(*(*cur).state).layout.is_null() {
    } else {
        __assert_fail(b"cur->state->layout\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_cursor.c\x00" as *const u8 as
                          *const libc::c_char, 201i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 64],
                                                &[libc::c_char; 64]>(b"void cursor_warp_unchecked(struct wlr_cursor *, double, double)\x00")).as_ptr());
    };
    let mut output_cursor: *mut wlr_cursor_output_cursor =
        0 as *mut wlr_cursor_output_cursor;
    output_cursor =
        ((*(*cur).state).output_cursors.next as *mut libc::c_char).offset(-16)
            as *mut wlr_cursor_output_cursor;
    while &mut (*output_cursor).link as *mut wl_list !=
              &mut (*(*cur).state).output_cursors as *mut wl_list {
        let mut output_x: libc::c_double = lx;
        let mut output_y: libc::c_double = ly;
        wlr_output_layout_output_coords((*(*cur).state).layout,
                                        (*(*output_cursor).output_cursor).output,
                                        &mut output_x, &mut output_y);
        wlr_output_cursor_move((*output_cursor).output_cursor, output_x,
                               output_y);
        output_cursor =
            ((*output_cursor).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_cursor_output_cursor
    }
    (*cur).x = lx;
    (*cur).y = ly;
}
/* *
 * Get the most specific mapping box for the device in this order:
 *
 * 1. device geometry mapping
 * 2. device output mapping
 * 3. cursor geometry mapping
 * 4. cursor output mapping
 *
 * Absolute movement for touch and pen devices will be relative to this box and
 * pointer movement will be constrained to this box.
 *
 * If none of these are set, returns NULL and absolute movement should be
 * relative to the extents of the layout.
 */
unsafe extern "C" fn get_mapping(mut cur: *mut wlr_cursor,
                                 mut dev: *mut wlr_input_device)
 -> *mut wlr_box {
    if !(*(*cur).state).layout.is_null() {
    } else {
        __assert_fail(b"cur->state->layout\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_cursor.c\x00" as *const u8 as
                          *const libc::c_char, 232i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 76],
                                                &[libc::c_char; 76]>(b"struct wlr_box *get_mapping(struct wlr_cursor *, struct wlr_input_device *)\x00")).as_ptr());
    };
    let mut c_device: *mut wlr_cursor_device = get_cursor_device(cur, dev);
    if !c_device.is_null() {
        if !(*c_device).mapped_box.is_null() { return (*c_device).mapped_box }
        if !(*c_device).mapped_output.is_null() {
            return wlr_output_layout_get_box((*(*cur).state).layout,
                                             (*c_device).mapped_output)
        }
    }
    if !(*(*cur).state).mapped_box.is_null() {
        return (*(*cur).state).mapped_box
    }
    if !(*(*cur).state).mapped_output.is_null() {
        return wlr_output_layout_get_box((*(*cur).state).layout,
                                         (*(*cur).state).mapped_output)
    }
    return 0 as *mut wlr_box;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_warp(mut cur: *mut wlr_cursor,
                                         mut dev: *mut wlr_input_device,
                                         mut lx: libc::c_double,
                                         mut ly: libc::c_double) -> bool {
    if !(*(*cur).state).layout.is_null() {
    } else {
        __assert_fail(b"cur->state->layout\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_cursor.c\x00" as *const u8 as
                          *const libc::c_char, 258i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 86],
                                                &[libc::c_char; 86]>(b"_Bool wlr_cursor_warp(struct wlr_cursor *, struct wlr_input_device *, double, double)\x00")).as_ptr());
    };
    let mut result: bool = 0i32 != 0;
    let mut mapping: *mut wlr_box = get_mapping(cur, dev);
    if !mapping.is_null() {
        result = wlr_box_contains_point(mapping, lx, ly)
    } else {
        result =
            wlr_output_layout_contains_point((*(*cur).state).layout,
                                             0 as *mut wlr_output,
                                             lx as libc::c_int,
                                             ly as libc::c_int)
    }
    if result { cursor_warp_unchecked(cur, lx, ly); }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_warp_closest(mut cur: *mut wlr_cursor,
                                                 mut dev:
                                                     *mut wlr_input_device,
                                                 mut lx: libc::c_double,
                                                 mut ly: libc::c_double) {
    let mut mapping: *mut wlr_box = get_mapping(cur, dev);
    if !mapping.is_null() {
        wlr_box_closest_point(mapping, lx, ly, &mut lx, &mut ly);
        if lx != lx || ly != ly {
            lx = 0i32 as libc::c_double;
            ly = 0i32 as libc::c_double
        }
    } else {
        wlr_output_layout_closest_point((*(*cur).state).layout,
                                        0 as *mut wlr_output, lx, ly, &mut lx,
                                        &mut ly);
    }
    cursor_warp_unchecked(cur, lx, ly);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_absolute_to_layout_coords(mut cur:
                                                                  *mut wlr_cursor,
                                                              mut dev:
                                                                  *mut wlr_input_device,
                                                              mut x:
                                                                  libc::c_double,
                                                              mut y:
                                                                  libc::c_double,
                                                              mut lx:
                                                                  *mut libc::c_double,
                                                              mut ly:
                                                                  *mut libc::c_double) {
    if !(*(*cur).state).layout.is_null() {
    } else {
        __assert_fail(b"cur->state->layout\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_cursor.c\x00" as *const u8 as
                          *const libc::c_char, 296i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 126],
                                                &[libc::c_char; 126]>(b"void wlr_cursor_absolute_to_layout_coords(struct wlr_cursor *, struct wlr_input_device *, double, double, double *, double *)\x00")).as_ptr());
    };
    let mut mapping: *mut wlr_box = get_mapping(cur, dev);
    if mapping.is_null() {
        mapping =
            wlr_output_layout_get_box((*(*cur).state).layout,
                                      0 as *mut wlr_output)
    }
    *lx =
        if x == x {
            ((*mapping).width as libc::c_double * x) +
                (*mapping).x as libc::c_double
        } else { (*cur).x };
    *ly =
        if y == y {
            ((*mapping).height as libc::c_double * y) +
                (*mapping).y as libc::c_double
        } else { (*cur).y };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_warp_absolute(mut cur: *mut wlr_cursor,
                                                  mut dev:
                                                      *mut wlr_input_device,
                                                  mut x: libc::c_double,
                                                  mut y: libc::c_double) {
    if !(*(*cur).state).layout.is_null() {
    } else {
        __assert_fail(b"cur->state->layout\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_cursor.c\x00" as *const u8 as
                          *const libc::c_char, 309i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 94],
                                                &[libc::c_char; 94]>(b"void wlr_cursor_warp_absolute(struct wlr_cursor *, struct wlr_input_device *, double, double)\x00")).as_ptr());
    };
    let mut lx: libc::c_double = 0.;
    let mut ly: libc::c_double = 0.;
    wlr_cursor_absolute_to_layout_coords(cur, dev, x, y, &mut lx, &mut ly);
    wlr_cursor_warp_closest(cur, dev, lx, ly);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_move(mut cur: *mut wlr_cursor,
                                         mut dev: *mut wlr_input_device,
                                         mut delta_x: libc::c_double,
                                         mut delta_y: libc::c_double) {
    if !(*(*cur).state).layout.is_null() {
    } else {
        __assert_fail(b"cur->state->layout\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_cursor.c\x00" as *const u8 as
                          *const libc::c_char, 319i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[libc::c_char; 85]>(b"void wlr_cursor_move(struct wlr_cursor *, struct wlr_input_device *, double, double)\x00")).as_ptr());
    };
    let mut lx: libc::c_double =
        if delta_x == delta_x { ((*cur).x) + delta_x } else { (*cur).x };
    let mut ly: libc::c_double =
        if delta_y == delta_y { ((*cur).y) + delta_y } else { (*cur).y };
    wlr_cursor_warp_closest(cur, dev, lx, ly);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_set_image(mut cur: *mut wlr_cursor,
                                              mut pixels: *const uint8_t,
                                              mut stride: int32_t,
                                              mut width: uint32_t,
                                              mut height: uint32_t,
                                              mut hotspot_x: int32_t,
                                              mut hotspot_y: int32_t,
                                              mut scale: libc::c_float) {
    let mut output_cursor: *mut wlr_cursor_output_cursor =
        0 as *mut wlr_cursor_output_cursor;
    output_cursor =
        ((*(*cur).state).output_cursors.next as *mut libc::c_char).offset(-16)
            as *mut wlr_cursor_output_cursor;
    while &mut (*output_cursor).link as *mut wl_list !=
              &mut (*(*cur).state).output_cursors as *mut wl_list {
        let mut output_scale: libc::c_float =
            (*(*(*output_cursor).output_cursor).output).scale;
        if !(scale > 0i32 as libc::c_float && output_scale != scale) {
            wlr_output_cursor_set_image((*output_cursor).output_cursor,
                                        pixels, stride, width, height,
                                        hotspot_x, hotspot_y);
        }
        output_cursor =
            ((*output_cursor).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_cursor_output_cursor
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_set_surface(mut cur: *mut wlr_cursor,
                                                mut surface: *mut wlr_surface,
                                                mut hotspot_x: int32_t,
                                                mut hotspot_y: int32_t) {
    let mut output_cursor: *mut wlr_cursor_output_cursor =
        0 as *mut wlr_cursor_output_cursor;
    output_cursor =
        ((*(*cur).state).output_cursors.next as *mut libc::c_char).offset(-16)
            as *mut wlr_cursor_output_cursor;
    while &mut (*output_cursor).link as *mut wl_list !=
              &mut (*(*cur).state).output_cursors as *mut wl_list {
        wlr_output_cursor_set_surface((*output_cursor).output_cursor, surface,
                                      hotspot_x, hotspot_y);
        output_cursor =
            ((*output_cursor).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_cursor_output_cursor
    };
}
unsafe extern "C" fn handle_pointer_motion(mut listener: *mut wl_listener,
                                           mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_pointer_motion =
        data as *mut wlr_event_pointer_motion;
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-48) as *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.motion,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn apply_output_transform(mut x: *mut libc::c_double,
                                            mut y: *mut libc::c_double,
                                            mut transform:
                                                wl_output_transform) {
    let mut dx: libc::c_double = 0.0f64;
    let mut dy: libc::c_double = 0.0f64;
    let mut width: libc::c_double = 1.0f64;
    let mut height: libc::c_double = 1.0f64;
    match transform as libc::c_uint {
        0 => { dx = *x; dy = *y }
        1 => { dx = *y; dy = width - *x }
        2 => { dx = width - *x; dy = height - *y }
        3 => { dx = height - *y; dy = *x }
        4 => { dx = width - *x; dy = *y }
        5 => { dx = height - *y; dy = width - *x }
        6 => { dx = *x; dy = height - *y }
        7 => { dx = *y; dy = *x }
        _ => { }
    }
    *x = dx;
    *y = dy;
}
unsafe extern "C" fn get_mapped_output(mut cursor_device:
                                           *mut wlr_cursor_device)
 -> *mut wlr_output {
    if !(*cursor_device).mapped_output.is_null() {
        return (*cursor_device).mapped_output
    }
    let mut cursor: *mut wlr_cursor = (*cursor_device).cursor;
    if !cursor.is_null() {
    } else {
        __assert_fail(b"cursor\x00" as *const u8 as *const libc::c_char,
                      b"../types/wlr_cursor.c\x00" as *const u8 as
                          *const libc::c_char, 408i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 65],
                                                &[libc::c_char; 65]>(b"struct wlr_output *get_mapped_output(struct wlr_cursor_device *)\x00")).as_ptr());
    };
    if !(*(*cursor).state).mapped_output.is_null() {
        return (*(*cursor).state).mapped_output
    }
    return 0 as *mut wlr_output;
}
unsafe extern "C" fn handle_pointer_motion_absolute(mut listener:
                                                        *mut wl_listener,
                                                    mut data:
                                                        *mut libc::c_void) {
    let mut event: *mut wlr_event_pointer_motion_absolute =
        data as *mut wlr_event_pointer_motion_absolute;
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-72) as *mut wlr_cursor_device;
    let mut output: *mut wlr_output = get_mapped_output(device);
    if !output.is_null() {
        apply_output_transform(&mut (*event).x, &mut (*event).y,
                               (*output).transform);
    }
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.motion_absolute,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_pointer_button(mut listener: *mut wl_listener,
                                           mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_pointer_button =
        data as *mut wlr_event_pointer_button;
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-96) as *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.button,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_pointer_axis(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_pointer_axis =
        data as *mut wlr_event_pointer_axis;
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-120) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.axis,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_pointer_frame(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-144) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.frame,
                         (*device).cursor as *mut libc::c_void);
}
unsafe extern "C" fn handle_pointer_swipe_begin(mut listener:
                                                    *mut wl_listener,
                                                mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_pointer_swipe_begin =
        data as *mut wlr_event_pointer_swipe_begin;
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-168) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.swipe_begin,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_pointer_swipe_update(mut listener:
                                                     *mut wl_listener,
                                                 mut data:
                                                     *mut libc::c_void) {
    let mut event: *mut wlr_event_pointer_swipe_update =
        data as *mut wlr_event_pointer_swipe_update;
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-192) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.swipe_update,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_pointer_swipe_end(mut listener: *mut wl_listener,
                                              mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_pointer_swipe_end =
        data as *mut wlr_event_pointer_swipe_end;
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-216) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.swipe_end,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_pointer_pinch_begin(mut listener:
                                                    *mut wl_listener,
                                                mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_pointer_pinch_begin =
        data as *mut wlr_event_pointer_pinch_begin;
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-240) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.pinch_begin,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_pointer_pinch_update(mut listener:
                                                     *mut wl_listener,
                                                 mut data:
                                                     *mut libc::c_void) {
    let mut event: *mut wlr_event_pointer_pinch_update =
        data as *mut wlr_event_pointer_pinch_update;
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-264) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.pinch_update,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_pointer_pinch_end(mut listener: *mut wl_listener,
                                              mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_pointer_pinch_end =
        data as *mut wlr_event_pointer_pinch_end;
    let mut device: *mut wlr_cursor_device =
        (listener as *mut libc::c_char).offset(-288) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.pinch_end,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_touch_up(mut listener: *mut wl_listener,
                                     mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_touch_up = data as *mut wlr_event_touch_up;
    let mut device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    device =
        (listener as *mut libc::c_char).offset(-336) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.touch_up,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_touch_down(mut listener: *mut wl_listener,
                                       mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_touch_down =
        data as *mut wlr_event_touch_down;
    let mut device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    device =
        (listener as *mut libc::c_char).offset(-312) as
            *mut wlr_cursor_device;
    let mut output: *mut wlr_output = get_mapped_output(device);
    if !output.is_null() {
        apply_output_transform(&mut (*event).x, &mut (*event).y,
                               (*output).transform);
    }
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.touch_down,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_touch_motion(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_touch_motion =
        data as *mut wlr_event_touch_motion;
    let mut device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    device =
        (listener as *mut libc::c_char).offset(-360) as
            *mut wlr_cursor_device;
    let mut output: *mut wlr_output = get_mapped_output(device);
    if !output.is_null() {
        apply_output_transform(&mut (*event).x, &mut (*event).y,
                               (*output).transform);
    }
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.touch_motion,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_touch_cancel(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_touch_cancel =
        data as *mut wlr_event_touch_cancel;
    let mut device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    device =
        (listener as *mut libc::c_char).offset(-384) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.touch_cancel,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_tool_tip(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_tablet_tool_tip =
        data as *mut wlr_event_tablet_tool_tip;
    let mut device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    device =
        (listener as *mut libc::c_char).offset(-456) as
            *mut wlr_cursor_device;
    let mut output: *mut wlr_output = get_mapped_output(device);
    if !output.is_null() {
        apply_output_transform(&mut (*event).x, &mut (*event).y,
                               (*output).transform);
    }
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.tablet_tool_tip,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_tool_axis(mut listener: *mut wl_listener,
                                             mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_tablet_tool_axis =
        data as *mut wlr_event_tablet_tool_axis;
    let mut device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    device =
        (listener as *mut libc::c_char).offset(-408) as
            *mut wlr_cursor_device;
    let mut output: *mut wlr_output = get_mapped_output(device);
    if !output.is_null() {
        apply_output_transform(&mut (*event).x, &mut (*event).y,
                               (*output).transform);
    }
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.tablet_tool_axis,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_tool_button(mut listener: *mut wl_listener,
                                               mut data: *mut libc::c_void) {
    let mut event: *mut wlr_event_tablet_tool_button =
        data as *mut wlr_event_tablet_tool_button;
    let mut device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    device =
        (listener as *mut libc::c_char).offset(-480) as
            *mut wlr_cursor_device;
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.tablet_tool_button,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_tablet_tool_proximity(mut listener:
                                                      *mut wl_listener,
                                                  mut data:
                                                      *mut libc::c_void) {
    let mut event: *mut wlr_event_tablet_tool_proximity =
        data as *mut wlr_event_tablet_tool_proximity;
    let mut device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    device =
        (listener as *mut libc::c_char).offset(-432) as
            *mut wlr_cursor_device;
    let mut output: *mut wlr_output = get_mapped_output(device);
    if !output.is_null() {
        apply_output_transform(&mut (*event).x, &mut (*event).y,
                               (*output).transform);
    }
    wlr_signal_emit_safe(&mut (*(*device).cursor).events.tablet_tool_proximity,
                         event as *mut libc::c_void);
}
unsafe extern "C" fn handle_device_destroy(mut listener: *mut wl_listener,
                                           mut data: *mut libc::c_void) {
    let mut c_device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    c_device =
        (listener as *mut libc::c_char).offset(-504) as
            *mut wlr_cursor_device;
    wlr_cursor_detach_input_device((*c_device).cursor, (*c_device).device);
}
unsafe extern "C" fn cursor_device_create(mut cursor: *mut wlr_cursor,
                                          mut device: *mut wlr_input_device)
 -> *mut wlr_cursor_device {
    let mut c_device: *mut wlr_cursor_device =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_cursor_device>() as libc::c_ulong) as
            *mut wlr_cursor_device;
    if c_device.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate wlr_cursor_device\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_cursor.c\x00" as *const u8 as
                     *const libc::c_char, 583i32);
        return 0 as *mut wlr_cursor_device
    }
    (*c_device).cursor = cursor;
    (*c_device).device = device;
    // listen to events
    wl_signal_add(&mut (*device).events.destroy, &mut (*c_device).destroy);
    (*c_device).destroy.notify =
        Some(handle_device_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    if (*device).type_0 as libc::c_uint ==
           WLR_INPUT_DEVICE_POINTER as libc::c_int as libc::c_uint {
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.motion,
                      &mut (*c_device).motion);
        (*c_device).motion.notify =
            Some(handle_pointer_motion as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.motion_absolute,
                      &mut (*c_device).motion_absolute);
        (*c_device).motion_absolute.notify =
            Some(handle_pointer_motion_absolute as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.button,
                      &mut (*c_device).button);
        (*c_device).button.notify =
            Some(handle_pointer_button as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.axis,
                      &mut (*c_device).axis);
        (*c_device).axis.notify =
            Some(handle_pointer_axis as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.frame,
                      &mut (*c_device).frame);
        (*c_device).frame.notify =
            Some(handle_pointer_frame as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.swipe_begin,
                      &mut (*c_device).swipe_begin);
        (*c_device).swipe_begin.notify =
            Some(handle_pointer_swipe_begin as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.swipe_update,
                      &mut (*c_device).swipe_update);
        (*c_device).swipe_update.notify =
            Some(handle_pointer_swipe_update as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.swipe_end,
                      &mut (*c_device).swipe_end);
        (*c_device).swipe_end.notify =
            Some(handle_pointer_swipe_end as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.pinch_begin,
                      &mut (*c_device).pinch_begin);
        (*c_device).pinch_begin.notify =
            Some(handle_pointer_pinch_begin as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.pinch_update,
                      &mut (*c_device).pinch_update);
        (*c_device).pinch_update.notify =
            Some(handle_pointer_pinch_update as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.pointer).events.pinch_end,
                      &mut (*c_device).pinch_end);
        (*c_device).pinch_end.notify =
            Some(handle_pointer_pinch_end as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ())
    } else if (*device).type_0 as libc::c_uint ==
                  WLR_INPUT_DEVICE_TOUCH as libc::c_int as libc::c_uint {
        wl_signal_add(&mut (*(*device).c2rust_unnamed.touch).events.motion,
                      &mut (*c_device).touch_motion);
        (*c_device).touch_motion.notify =
            Some(handle_touch_motion as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.touch).events.down,
                      &mut (*c_device).touch_down);
        (*c_device).touch_down.notify =
            Some(handle_touch_down as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.touch).events.up,
                      &mut (*c_device).touch_up);
        (*c_device).touch_up.notify =
            Some(handle_touch_up as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.touch).events.cancel,
                      &mut (*c_device).touch_cancel);
        (*c_device).touch_cancel.notify =
            Some(handle_touch_cancel as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ())
    } else if (*device).type_0 as libc::c_uint ==
                  WLR_INPUT_DEVICE_TABLET_TOOL as libc::c_int as libc::c_uint
     {
        wl_signal_add(&mut (*(*device).c2rust_unnamed.tablet).events.tip,
                      &mut (*c_device).tablet_tool_tip);
        (*c_device).tablet_tool_tip.notify =
            Some(handle_tablet_tool_tip as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.tablet).events.proximity,
                      &mut (*c_device).tablet_tool_proximity);
        (*c_device).tablet_tool_proximity.notify =
            Some(handle_tablet_tool_proximity as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.tablet).events.axis,
                      &mut (*c_device).tablet_tool_axis);
        (*c_device).tablet_tool_axis.notify =
            Some(handle_tablet_tool_axis as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*(*device).c2rust_unnamed.tablet).events.button,
                      &mut (*c_device).tablet_tool_button);
        (*c_device).tablet_tool_button.notify =
            Some(handle_tablet_tool_button as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ())
    }
    wl_list_insert(&mut (*(*cursor).state).devices, &mut (*c_device).link);
    return c_device;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_attach_input_device(mut cur:
                                                            *mut wlr_cursor,
                                                        mut dev:
                                                            *mut wlr_input_device) {
    if (*dev).type_0 as libc::c_uint !=
           WLR_INPUT_DEVICE_POINTER as libc::c_int as libc::c_uint &&
           (*dev).type_0 as libc::c_uint !=
               WLR_INPUT_DEVICE_TOUCH as libc::c_int as libc::c_uint &&
           (*dev).type_0 as libc::c_uint !=
               WLR_INPUT_DEVICE_TABLET_TOOL as libc::c_int as libc::c_uint {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] only device types of pointer, touch or tablet toolare supported\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/wlr_cursor.c\x00" as *const u8 as
                     *const libc::c_char, 669i32);
        return
    }
    // make sure it is not already attached
    let mut _dev: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    _dev =
        ((*(*cur).state).devices.next as *mut libc::c_char).offset(-16) as
            *mut wlr_cursor_device;
    while &mut (*_dev).link as *mut wl_list !=
              &mut (*(*cur).state).devices as *mut wl_list {
        if (*_dev).device == dev { return }
        _dev =
            ((*_dev).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_cursor_device
    }
    cursor_device_create(cur, dev);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_detach_input_device(mut cur:
                                                            *mut wlr_cursor,
                                                        mut dev:
                                                            *mut wlr_input_device) {
    let mut c_device: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    let mut tmp: *mut wlr_cursor_device = 0 as *mut wlr_cursor_device;
    c_device =
        ((*(*cur).state).devices.next as *mut libc::c_char).offset(-16) as
            *mut wlr_cursor_device;
    tmp =
        ((*c_device).link.next as *mut libc::c_char).offset(-16) as
            *mut wlr_cursor_device;
    while &mut (*c_device).link as *mut wl_list !=
              &mut (*(*cur).state).devices as *mut wl_list {
        if (*c_device).device == dev { cursor_device_destroy(c_device); }
        c_device = tmp;
        tmp =
            ((*c_device).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_cursor_device
    };
}
unsafe extern "C" fn handle_layout_destroy(mut listener: *mut wl_listener,
                                           mut data: *mut libc::c_void) {
    let mut state: *mut wlr_cursor_state =
        (listener as *mut libc::c_char).offset(-112) as *mut wlr_cursor_state;
    cursor_detach_output_layout((*state).cursor);
}
unsafe extern "C" fn handle_layout_output_destroy(mut listener:
                                                      *mut wl_listener,
                                                  mut data:
                                                      *mut libc::c_void) {
    let mut output_cursor: *mut wlr_cursor_output_cursor =
        (listener as *mut libc::c_char).offset(-32) as
            *mut wlr_cursor_output_cursor;
    //struct wlr_output_layout_output *l_output = data;
    output_cursor_destroy(output_cursor);
}
unsafe extern "C" fn layout_add(mut state: *mut wlr_cursor_state,
                                mut l_output: *mut wlr_output_layout_output) {
    let mut output_cursor: *mut wlr_cursor_output_cursor =
        0 as *mut wlr_cursor_output_cursor;
    output_cursor =
        ((*state).output_cursors.next as *mut libc::c_char).offset(-16) as
            *mut wlr_cursor_output_cursor;
    while &mut (*output_cursor).link as *mut wl_list !=
              &mut (*state).output_cursors as *mut wl_list {
        if (*(*output_cursor).output_cursor).output == (*l_output).output {
            return
            // already added
        }
        output_cursor =
            ((*output_cursor).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_cursor_output_cursor
    }
    output_cursor =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_cursor_output_cursor>() as
                   libc::c_ulong) as *mut wlr_cursor_output_cursor;
    if output_cursor.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate wlr_cursor_output_cursor\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_cursor.c\x00" as *const u8 as
                     *const libc::c_char, 719i32);
        return
    }
    (*output_cursor).cursor = (*state).cursor;
    (*output_cursor).output_cursor =
        wlr_output_cursor_create((*l_output).output);
    if (*output_cursor).output_cursor.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create wlr_output_cursor\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_cursor.c\x00" as *const u8 as
                     *const libc::c_char, 726i32);
        free(output_cursor as *mut libc::c_void);
        return
    }
    (*output_cursor).layout_output_destroy.notify =
        Some(handle_layout_output_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*l_output).events.destroy,
                  &mut (*output_cursor).layout_output_destroy);
    wl_list_insert(&mut (*state).output_cursors, &mut (*output_cursor).link);
}
unsafe extern "C" fn handle_layout_add(mut listener: *mut wl_listener,
                                       mut data: *mut libc::c_void) {
    let mut state: *mut wlr_cursor_state =
        (listener as *mut libc::c_char).offset(-64) as *mut wlr_cursor_state;
    let mut l_output: *mut wlr_output_layout_output =
        data as *mut wlr_output_layout_output;
    layout_add(state, l_output);
}
unsafe extern "C" fn handle_layout_change(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut state: *mut wlr_cursor_state =
        (listener as *mut libc::c_char).offset(-88) as *mut wlr_cursor_state;
    let mut layout: *mut wlr_output_layout = data as *mut wlr_output_layout;
    if !wlr_output_layout_contains_point(layout, 0 as *mut wlr_output,
                                         (*(*state).cursor).x as libc::c_int,
                                         (*(*state).cursor).y as libc::c_int)
       {
        // the output we were on has gone away so go to the closest boundary
		// point
        let mut x: libc::c_double = 0.;
        let mut y: libc::c_double = 0.;
        wlr_output_layout_closest_point(layout, 0 as *mut wlr_output,
                                        (*(*state).cursor).x,
                                        (*(*state).cursor).y, &mut x, &mut y);
        cursor_warp_unchecked((*state).cursor, x, y);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_attach_output_layout(mut cur:
                                                             *mut wlr_cursor,
                                                         mut l:
                                                             *mut wlr_output_layout) {
    cursor_detach_output_layout(cur);
    if l.is_null() { return }
    wl_signal_add(&mut (*l).events.add, &mut (*(*cur).state).layout_add);
    (*(*cur).state).layout_add.notify =
        Some(handle_layout_add as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*l).events.change,
                  &mut (*(*cur).state).layout_change);
    (*(*cur).state).layout_change.notify =
        Some(handle_layout_change as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*l).events.destroy,
                  &mut (*(*cur).state).layout_destroy);
    (*(*cur).state).layout_destroy.notify =
        Some(handle_layout_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    (*(*cur).state).layout = l;
    let mut l_output: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    l_output =
        ((*l).outputs.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    while &mut (*l_output).link as *mut wl_list !=
              &mut (*l).outputs as *mut wl_list {
        layout_add((*cur).state, l_output);
        l_output =
            ((*l_output).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_map_to_output(mut cur: *mut wlr_cursor,
                                                  mut output:
                                                      *mut wlr_output) {
    (*(*cur).state).mapped_output = output;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_map_input_to_output(mut cur:
                                                            *mut wlr_cursor,
                                                        mut dev:
                                                            *mut wlr_input_device,
                                                        mut output:
                                                            *mut wlr_output) {
    let mut c_device: *mut wlr_cursor_device = get_cursor_device(cur, dev);
    if c_device.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot map device \"%s\" to output(not found in this cursor)\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/wlr_cursor.c\x00" as *const u8 as
                     *const libc::c_char, 795i32, (*dev).name);
        return
    }
    (*c_device).mapped_output = output;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_map_to_region(mut cur: *mut wlr_cursor,
                                                  mut box_0: *mut wlr_box) {
    if !box_0.is_null() && wlr_box_empty(box_0) as libc::c_int != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] cannot map cursor to an empty region\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_cursor.c\x00" as *const u8 as
                     *const libc::c_char, 805i32);
        return
    }
    (*(*cur).state).mapped_box = box_0;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * wlr_cursor implements the behavior of the "cursor", that is, the image on the
 * screen typically moved about with a mouse or so. It provides tracking for
 * this in global coordinates, and integrates with wlr_output,
 * wlr_output_layout, and wlr_input_device. You can use it to abstract multiple
 * input devices over a single cursor, constrain cursor movement to the usable
 * area of a wlr_output_layout and communicate position updates to the hardware
 * cursor, constrain specific input devices to specific outputs or regions of
 * the screen, and so on.
 */
/* *
	 * The interpretation of these signals is the responsibility of the
	 * compositor, but some helpers are provided for your benefit. If you
	 * receive a relative motion event, for example, you may want to call
	 * wlr_cursor_move. If you receive an absolute event, call
	 * wlr_cursor_warp_absolute. If you pass an input device into these
	 * functions, it will apply the region/output constraints associated with
	 * that device to the resulting cursor motion. If an output layout is
	 * attached, these functions will constrain the resulting cursor motion to
	 * within the usable space of the output layout.
	 *
	 * Re-broadcasting these signals to, for example, a wlr_seat, is also your
	 * responsibility.
	 */
/* *
 * Warp the cursor to the given x and y in layout coordinates. If x and y are
 * out of the layout boundaries or constraints, no warp will happen.
 *
 * `dev` may be passed to respect device mapping constraints. If `dev` is NULL,
 * device mapping constraints will be ignored.
 *
 * Returns true when the cursor warp was successful.
 */
/* *
 * Convert absolute 0..1 coordinates to layout coordinates.
 *
 * `dev` may be passed to respect device mapping constraints. If `dev` is NULL,
 * device mapping constraints will be ignored.
 */
/* *
 * Warp the cursor to the given x and y coordinates. If the given point is out
 * of the layout boundaries or constraints, the closest point will be used.
 * If one coordinate is NAN, it will be ignored.
 *
 * `dev` may be passed to respect device mapping constraints. If `dev` is NULL,
 * device mapping constraints will be ignored.
 */
/* *
 * Warp the cursor to the given x and y in absolute 0..1 coordinates. If the
 * given point is out of the layout boundaries or constraints, the closest point
 * will be used. If one coordinate is NAN, it will be ignored.
 *
 * `dev` may be passed to respect device mapping constraints. If `dev` is NULL,
 * device mapping constraints will be ignored.
 */
/* *
 * Move the cursor in the direction of the given x and y layout coordinates. If
 * one coordinate is NAN, it will be ignored.
 *
 * `dev` may be passed to respect device mapping constraints. If `dev` is NULL,
 * device mapping constraints will be ignored.
 */
/* *
 * Set the cursor image. stride is given in bytes. If pixels is NULL, hides the
 * cursor.
 *
 * If scale isn't zero, the image is only set on outputs having the provided
 * scale.
 */
/* *
 * Set the cursor surface. The surface can be committed to update the cursor
 * image. The surface position is subtracted from the hotspot. A NULL surface
 * commit hides the cursor.
 */
/* *
 * Attaches this input device to this cursor. The input device must be one of:
 *
 * - WLR_INPUT_DEVICE_POINTER
 * - WLR_INPUT_DEVICE_TOUCH
 * - WLR_INPUT_DEVICE_TABLET_TOOL
 */
/* *
 * Uses the given layout to establish the boundaries and movement semantics of
 * this cursor. Cursors without an output layout allow infinite movement in any
 * direction and do not support absolute input events.
 */
/* *
 * Attaches this cursor to the given output, which must be among the outputs in
 * the current output_layout for this cursor. This call is invalid for a cursor
 * without an associated output layout.
 */
/* *
 * Maps all input from a specific input device to a given output. The input
 * device must be attached to this cursor and the output must be among the
 * outputs in the attached output layout.
 */
/* *
 * Maps this cursor to an arbitrary region on the associated wlr_output_layout.
 */
/* *
 * Maps inputs from this input device to an arbitrary region on the associated
 * wlr_output_layout.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_cursor_map_input_to_region(mut cur:
                                                            *mut wlr_cursor,
                                                        mut dev:
                                                            *mut wlr_input_device,
                                                        mut box_0:
                                                            *mut wlr_box) {
    if !box_0.is_null() && wlr_box_empty(box_0) as libc::c_int != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] cannot map device \"%s\" input to an empty region\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/wlr_cursor.c\x00" as *const u8 as
                     *const libc::c_char, 816i32, (*dev).name);
        return
    }
    let mut c_device: *mut wlr_cursor_device = get_cursor_device(cur, dev);
    if c_device.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot map device \"%s\" to geometry (not found inthis cursor)\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/wlr_cursor.c\x00" as *const u8 as
                     *const libc::c_char, 823i32, (*dev).name);
        return
    }
    (*c_device).mapped_box = box_0;
}
