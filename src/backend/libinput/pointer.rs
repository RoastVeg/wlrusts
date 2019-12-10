use libc;
extern "C" {
    pub type libinput_device;
    pub type libinput_event;
    pub type libinput_event_pointer;
    pub type libinput_event_gesture;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_touch_impl;
    pub type wlr_tablet_pad_impl;
    pub type wlr_tablet_impl;
    pub type wlr_switch_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn libinput_event_get_pointer_event(event: *mut libinput_event)
     -> *mut libinput_event_pointer;
    #[no_mangle]
    fn libinput_event_get_gesture_event(event: *mut libinput_event)
     -> *mut libinput_event_gesture;
    #[no_mangle]
    fn libinput_event_pointer_get_time_usec(event:
                                                *mut libinput_event_pointer)
     -> uint64_t;
    #[no_mangle]
    fn libinput_event_pointer_get_dx(event: *mut libinput_event_pointer)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_pointer_get_dy(event: *mut libinput_event_pointer)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_pointer_get_dx_unaccelerated(event:
                                                       *mut libinput_event_pointer)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_pointer_get_dy_unaccelerated(event:
                                                       *mut libinput_event_pointer)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_pointer_get_absolute_x_transformed(event:
                                                             *mut libinput_event_pointer,
                                                         width: uint32_t)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_pointer_get_absolute_y_transformed(event:
                                                             *mut libinput_event_pointer,
                                                         height: uint32_t)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_pointer_get_button(event: *mut libinput_event_pointer)
     -> uint32_t;
    #[no_mangle]
    fn libinput_event_pointer_get_button_state(event:
                                                   *mut libinput_event_pointer)
     -> libinput_button_state;
    #[no_mangle]
    fn libinput_event_pointer_has_axis(event: *mut libinput_event_pointer,
                                       axis: libinput_pointer_axis)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_pointer_get_axis_value(event:
                                                 *mut libinput_event_pointer,
                                             axis: libinput_pointer_axis)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_pointer_get_axis_source(event:
                                                  *mut libinput_event_pointer)
     -> libinput_pointer_axis_source;
    #[no_mangle]
    fn libinput_event_pointer_get_axis_value_discrete(event:
                                                          *mut libinput_event_pointer,
                                                      axis:
                                                          libinput_pointer_axis)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_gesture_get_time_usec(event:
                                                *mut libinput_event_gesture)
     -> uint64_t;
    #[no_mangle]
    fn libinput_event_gesture_get_finger_count(event:
                                                   *mut libinput_event_gesture)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_gesture_get_cancelled(event:
                                                *mut libinput_event_gesture)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_event_gesture_get_dx(event: *mut libinput_event_gesture)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_gesture_get_dy(event: *mut libinput_event_gesture)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_gesture_get_scale(event: *mut libinput_event_gesture)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_gesture_get_angle_delta(event:
                                                  *mut libinput_event_gesture)
     -> libc::c_double;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn wlr_pointer_init(pointer: *mut wlr_pointer,
                        impl_0: *const wlr_pointer_impl);
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
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type libinput_button_state = libc::c_uint;
pub const LIBINPUT_BUTTON_STATE_PRESSED: libinput_button_state = 1;
pub const LIBINPUT_BUTTON_STATE_RELEASED: libinput_button_state = 0;
pub type libinput_pointer_axis = libc::c_uint;
pub const LIBINPUT_POINTER_AXIS_SCROLL_HORIZONTAL: libinput_pointer_axis = 1;
pub const LIBINPUT_POINTER_AXIS_SCROLL_VERTICAL: libinput_pointer_axis = 0;
pub type libinput_pointer_axis_source = libc::c_uint;
pub const LIBINPUT_POINTER_AXIS_SOURCE_WHEEL_TILT:
          libinput_pointer_axis_source =
    4;
pub const LIBINPUT_POINTER_AXIS_SOURCE_CONTINUOUS:
          libinput_pointer_axis_source =
    3;
pub const LIBINPUT_POINTER_AXIS_SOURCE_FINGER: libinput_pointer_axis_source =
    2;
pub const LIBINPUT_POINTER_AXIS_SOURCE_WHEEL: libinput_pointer_axis_source =
    1;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
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
pub struct wlr_touch {
    pub impl_0: *const crate::src::backend::headless::input_device::wlr_touch_impl,
    pub events: C2RustUnnamed_1,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub down: wl_signal,
    pub up: wl_signal,
    pub motion: wl_signal,
    pub cancel: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* Note: these are circular dependencies */
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
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub events: C2RustUnnamed_2,
    pub data: *mut libc::c_void,
    pub link: wl_list,
    // From 0..1
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
pub struct wlr_switch {
    pub impl_0: *mut crate::src::backend::headless::input_device::wlr_switch_impl,
    pub events: C2RustUnnamed_6,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub toggle: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
    pub events: C2RustUnnamed_7,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
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
pub struct wlr_pointer_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_pointer) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_pointer_motion {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub delta_x: libc::c_double,
    pub delta_y: libc::c_double,
    pub unaccel_dx: libc::c_double,
    pub unaccel_dy: libc::c_double,
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
pub struct wlr_event_pointer_swipe_begin {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_pointer_swipe_update {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
    pub dx: libc::c_double,
    pub dy: libc::c_double,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_pointer_swipe_end {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub cancelled: bool,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_pointer_pinch_begin {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_pointer_pinch_update {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
    pub dx: libc::c_double,
    pub dy: libc::c_double,
    pub scale: libc::c_double,
    pub rotation: libc::c_double,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_event_pointer_pinch_end {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub cancelled: bool,
}
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
#[no_mangle]
pub unsafe extern "C" fn create_libinput_pointer(mut libinput_dev:
                                                     *mut libinput_device)
 -> *mut wlr_pointer {
    if !libinput_dev.is_null() {
    } else {
        __assert_fail(b"libinput_dev\x00" as *const u8 as *const libc::c_char,
                      b"../backend/libinput/pointer.c\x00" as *const u8 as
                          *const libc::c_char, 13i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"struct wlr_pointer *create_libinput_pointer(struct libinput_device *)\x00")).as_ptr());
    };
    let mut wlr_pointer: *mut wlr_pointer =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_pointer>() as libc::c_ulong) as
            *mut wlr_pointer;
    if wlr_pointer.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unable to allocate wlr_pointer\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 16i32);
        return 0 as *mut wlr_pointer
    }
    wlr_pointer_init(wlr_pointer, 0 as *const wlr_pointer_impl);
    return wlr_pointer;
}
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_motion(mut event: *mut libinput_event,
                                               mut libinput_dev:
                                                   *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_POINTER, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a pointer event for a device with no pointers?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 28i32);
        return
    }
    let mut pevent: *mut libinput_event_pointer =
        libinput_event_get_pointer_event(event);
    let mut wlr_event: wlr_event_pointer_motion =
        {
            let mut init =
                wlr_event_pointer_motion{device: 0 as *mut wlr_input_device,
                                         time_msec: 0,
                                         delta_x: 0.,
                                         delta_y: 0.,
                                         unaccel_dx: 0.,
                                         unaccel_dy: 0.,};
            init
        };
    wlr_event.device = wlr_dev;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_pointer_get_time_usec(pevent));
    wlr_event.delta_x = libinput_event_pointer_get_dx(pevent);
    wlr_event.delta_y = libinput_event_pointer_get_dy(pevent);
    wlr_event.unaccel_dx =
        libinput_event_pointer_get_dx_unaccelerated(pevent);
    wlr_event.unaccel_dy =
        libinput_event_pointer_get_dy_unaccelerated(pevent);
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.motion,
                         &mut wlr_event as *mut wlr_event_pointer_motion as
                             *mut libc::c_void);
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.frame,
                         (*wlr_dev).c2rust_unnamed.pointer as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_motion_abs(mut event:
                                                       *mut libinput_event,
                                                   mut libinput_dev:
                                                       *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_POINTER, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a pointer event for a device with no pointers?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 50i32);
        return
    }
    let mut pevent: *mut libinput_event_pointer =
        libinput_event_get_pointer_event(event);
    let mut wlr_event: wlr_event_pointer_motion_absolute =
        {
            let mut init =
                wlr_event_pointer_motion_absolute{device:
                                                      0 as
                                                          *mut wlr_input_device,
                                                  time_msec: 0,
                                                  x: 0.,
                                                  y: 0.,};
            init
        };
    wlr_event.device = wlr_dev;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_pointer_get_time_usec(pevent));
    wlr_event.x =
        libinput_event_pointer_get_absolute_x_transformed(pevent,
                                                          1i32 as uint32_t);
    wlr_event.y =
        libinput_event_pointer_get_absolute_y_transformed(pevent,
                                                          1i32 as uint32_t);
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.motion_absolute,
                         &mut wlr_event as
                             *mut wlr_event_pointer_motion_absolute as
                             *mut libc::c_void);
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.frame,
                         (*wlr_dev).c2rust_unnamed.pointer as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_button(mut event: *mut libinput_event,
                                               mut libinput_dev:
                                                   *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_POINTER, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a pointer event for a device with no pointers?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 70i32);
        return
    }
    let mut pevent: *mut libinput_event_pointer =
        libinput_event_get_pointer_event(event);
    let mut wlr_event: wlr_event_pointer_button =
        {
            let mut init =
                wlr_event_pointer_button{device: 0 as *mut wlr_input_device,
                                         time_msec: 0,
                                         button: 0,
                                         state: WLR_BUTTON_RELEASED,};
            init
        };
    wlr_event.device = wlr_dev;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_pointer_get_time_usec(pevent));
    wlr_event.button = libinput_event_pointer_get_button(pevent);
    match libinput_event_pointer_get_button_state(pevent) as libc::c_uint {
        1 => { wlr_event.state = WLR_BUTTON_PRESSED }
        0 => { wlr_event.state = WLR_BUTTON_RELEASED }
        _ => { }
    }
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.button,
                         &mut wlr_event as *mut wlr_event_pointer_button as
                             *mut libc::c_void);
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.frame,
                         (*wlr_dev).c2rust_unnamed.pointer as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_axis(mut event: *mut libinput_event,
                                             mut libinput_dev:
                                                 *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_POINTER, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a pointer event for a device with no pointers?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 97i32);
        return
    }
    let mut pevent: *mut libinput_event_pointer =
        libinput_event_get_pointer_event(event);
    let mut wlr_event: wlr_event_pointer_axis =
        {
            let mut init =
                wlr_event_pointer_axis{device: 0 as *mut wlr_input_device,
                                       time_msec: 0,
                                       source: WLR_AXIS_SOURCE_WHEEL,
                                       orientation:
                                           WLR_AXIS_ORIENTATION_VERTICAL,
                                       delta: 0.,
                                       delta_discrete: 0,};
            init
        };
    wlr_event.device = wlr_dev;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_pointer_get_time_usec(pevent));
    match libinput_event_pointer_get_axis_source(pevent) as libc::c_uint {
        1 => { wlr_event.source = WLR_AXIS_SOURCE_WHEEL }
        2 => { wlr_event.source = WLR_AXIS_SOURCE_FINGER }
        3 => { wlr_event.source = WLR_AXIS_SOURCE_CONTINUOUS }
        4 => { wlr_event.source = WLR_AXIS_SOURCE_WHEEL_TILT }
        _ => { }
    }
    let axes: [libinput_pointer_axis; 2] =
        [LIBINPUT_POINTER_AXIS_SCROLL_VERTICAL,
         LIBINPUT_POINTER_AXIS_SCROLL_HORIZONTAL];
    let mut i: size_t = 0i32 as size_t;
    while i <
              (::std::mem::size_of::<[libinput_pointer_axis; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<libinput_pointer_axis>()
                                                   as libc::c_ulong) {
        if libinput_event_pointer_has_axis(pevent, axes[i as usize]) != 0 {
            match axes[i as usize] as libc::c_uint {
                0 => { wlr_event.orientation = WLR_AXIS_ORIENTATION_VERTICAL }
                1 => {
                    wlr_event.orientation = WLR_AXIS_ORIENTATION_HORIZONTAL
                }
                _ => { }
            }
            wlr_event.delta =
                libinput_event_pointer_get_axis_value(pevent,
                                                      axes[i as usize]);
            wlr_event.delta_discrete =
                libinput_event_pointer_get_axis_value_discrete(pevent,
                                                               axes[i as
                                                                        usize])
                    as int32_t;
            wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.axis,
                                 &mut wlr_event as *mut wlr_event_pointer_axis
                                     as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.frame,
                         (*wlr_dev).c2rust_unnamed.pointer as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_swipe_begin(mut event:
                                                        *mut libinput_event,
                                                    mut libinput_dev:
                                                        *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_POINTER, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a pointer gesture event for a device with no pointers?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 149i32);
        return
    }
    let mut gevent: *mut libinput_event_gesture =
        libinput_event_get_gesture_event(event);
    let mut wlr_event: wlr_event_pointer_swipe_begin =
        {
            let mut init =
                wlr_event_pointer_swipe_begin{device: wlr_dev,
                                              time_msec:
                                                  usec_to_msec(libinput_event_gesture_get_time_usec(gevent)),
                                              fingers:
                                                  libinput_event_gesture_get_finger_count(gevent)
                                                      as uint32_t,};
            init
        };
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.swipe_begin,
                         &mut wlr_event as *mut wlr_event_pointer_swipe_begin
                             as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_swipe_update(mut event:
                                                         *mut libinput_event,
                                                     mut libinput_dev:
                                                         *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_POINTER, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a pointer gesture event for a device with no pointers?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 168i32);
        return
    }
    let mut gevent: *mut libinput_event_gesture =
        libinput_event_get_gesture_event(event);
    let mut wlr_event: wlr_event_pointer_swipe_update =
        {
            let mut init =
                wlr_event_pointer_swipe_update{device: wlr_dev,
                                               time_msec:
                                                   usec_to_msec(libinput_event_gesture_get_time_usec(gevent)),
                                               fingers:
                                                   libinput_event_gesture_get_finger_count(gevent)
                                                       as uint32_t,
                                               dx:
                                                   libinput_event_gesture_get_dx(gevent),
                                               dy:
                                                   libinput_event_gesture_get_dy(gevent),};
            init
        };
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.swipe_update,
                         &mut wlr_event as *mut wlr_event_pointer_swipe_update
                             as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_swipe_end(mut event:
                                                      *mut libinput_event,
                                                  mut libinput_dev:
                                                      *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_POINTER, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a pointer gesture event for a device with no pointers?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 189i32);
        return
    }
    let mut gevent: *mut libinput_event_gesture =
        libinput_event_get_gesture_event(event);
    let mut wlr_event: wlr_event_pointer_swipe_end =
        {
            let mut init =
                wlr_event_pointer_swipe_end{device: wlr_dev,
                                            time_msec:
                                                usec_to_msec(libinput_event_gesture_get_time_usec(gevent)),
                                            cancelled:
                                                libinput_event_gesture_get_cancelled(gevent)
                                                    != 0,};
            init
        };
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.swipe_end,
                         &mut wlr_event as *mut wlr_event_pointer_swipe_end as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_pinch_begin(mut event:
                                                        *mut libinput_event,
                                                    mut libinput_dev:
                                                        *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_POINTER, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a pointer gesture event for a device with no pointers?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 208i32);
        return
    }
    let mut gevent: *mut libinput_event_gesture =
        libinput_event_get_gesture_event(event);
    let mut wlr_event: wlr_event_pointer_pinch_begin =
        {
            let mut init =
                wlr_event_pointer_pinch_begin{device: wlr_dev,
                                              time_msec:
                                                  usec_to_msec(libinput_event_gesture_get_time_usec(gevent)),
                                              fingers:
                                                  libinput_event_gesture_get_finger_count(gevent)
                                                      as uint32_t,};
            init
        };
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.pinch_begin,
                         &mut wlr_event as *mut wlr_event_pointer_pinch_begin
                             as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_pinch_update(mut event:
                                                         *mut libinput_event,
                                                     mut libinput_dev:
                                                         *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_POINTER, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a pointer gesture event for a device with no pointers?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 227i32);
        return
    }
    let mut gevent: *mut libinput_event_gesture =
        libinput_event_get_gesture_event(event);
    let mut wlr_event: wlr_event_pointer_pinch_update =
        {
            let mut init =
                wlr_event_pointer_pinch_update{device: wlr_dev,
                                               time_msec:
                                                   usec_to_msec(libinput_event_gesture_get_time_usec(gevent)),
                                               fingers:
                                                   libinput_event_gesture_get_finger_count(gevent)
                                                       as uint32_t,
                                               dx:
                                                   libinput_event_gesture_get_dx(gevent),
                                               dy:
                                                   libinput_event_gesture_get_dy(gevent),
                                               scale:
                                                   libinput_event_gesture_get_scale(gevent),
                                               rotation:
                                                   libinput_event_gesture_get_angle_delta(gevent),};
            init
        };
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.pinch_update,
                         &mut wlr_event as *mut wlr_event_pointer_pinch_update
                             as *mut libc::c_void);
}
// list of struct wl_list
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_pinch_end(mut event:
                                                      *mut libinput_event,
                                                  mut libinput_dev:
                                                      *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_POINTER, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a pointer gesture event for a device with no pointers?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 250i32);
        return
    }
    let mut gevent: *mut libinput_event_gesture =
        libinput_event_get_gesture_event(event);
    let mut wlr_event: wlr_event_pointer_pinch_end =
        {
            let mut init =
                wlr_event_pointer_pinch_end{device: wlr_dev,
                                            time_msec:
                                                usec_to_msec(libinput_event_gesture_get_time_usec(gevent)),
                                            cancelled:
                                                libinput_event_gesture_get_cancelled(gevent)
                                                    != 0,};
            init
        };
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.pointer).events.pinch_end,
                         &mut wlr_event as *mut wlr_event_pointer_pinch_end as
                             *mut libc::c_void);
}
