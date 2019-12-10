use libc;
extern "C" {
    pub type libinput_device;
    pub type libinput_event;
    pub type libinput_event_touch;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /*
 * NOTE: the wlr tablet pad implementation does not currently support tablets
 * with more than one mode. I don't own any such hardware so I cannot test it
 * and it is too complicated to make a meaningful implementation of blindly.
 */
    pub type wlr_tablet_pad_impl;
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
    pub type wlr_tablet_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_switch_impl;
    pub type wlr_pointer_impl;
    pub type xkb_state;
    pub type xkb_keymap;
    pub type wlr_keyboard_group;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_keyboard_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn libinput_event_get_touch_event(event: *mut libinput_event)
     -> *mut libinput_event_touch;
    #[no_mangle]
    fn libinput_event_touch_get_time_usec(event: *mut libinput_event_touch)
     -> uint64_t;
    #[no_mangle]
    fn libinput_event_touch_get_seat_slot(event: *mut libinput_event_touch)
     -> int32_t;
    #[no_mangle]
    fn libinput_event_touch_get_x_transformed(event:
                                                  *mut libinput_event_touch,
                                              width: uint32_t)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_touch_get_y_transformed(event:
                                                  *mut libinput_event_touch,
                                              height: uint32_t)
     -> libc::c_double;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn wlr_touch_init(touch: *mut wlr_touch, impl_0: *const wlr_touch_impl);
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_touch_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_touch) -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_touch {
    pub impl_0: *const wlr_touch_impl,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub down: wl_signal,
    pub up: wl_signal,
    pub motion: wl_signal,
    pub cancel: wl_signal,
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
pub struct wlr_input_device {
    pub impl_0: *const wlr_input_device_impl,
    pub type_0: wlr_input_device_type,
    pub vendor: libc::c_uint,
    pub product: libc::c_uint,
    pub name: *mut libc::c_char,
    pub width_mm: libc::c_double,
    pub height_mm: libc::c_double,
    pub output_name: *mut libc::c_char,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub events: C2RustUnnamed_0,
    pub data: *mut libc::c_void,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_1 {
    pub _device: *mut libc::c_void,
    pub keyboard: *mut wlr_keyboard,
    pub pointer: *mut wlr_pointer,
    pub switch_device: *mut wlr_switch,
    pub touch: *mut wlr_touch,
    pub tablet: *mut wlr_tablet,
    pub tablet_pad: *mut wlr_tablet_pad,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet_pad {
    pub impl_0: *mut wlr_tablet_pad_impl,
    pub events: C2RustUnnamed_2,
    pub button_count: size_t,
    pub ring_count: size_t,
    pub strip_count: size_t,
    pub groups: wl_list,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_list {
    pub capacity: size_t,
    pub length: size_t,
    pub items: *mut *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub button: wl_signal,
    pub ring: wl_signal,
    pub strip: wl_signal,
    pub attach_tablet: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet {
    pub impl_0: *mut wlr_tablet_impl,
    pub events: C2RustUnnamed_3,
    pub name: *mut libc::c_char,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub axis: wl_signal,
    pub proximity: wl_signal,
    pub tip: wl_signal,
    pub button: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_switch {
    pub impl_0: *mut wlr_switch_impl,
    pub events: C2RustUnnamed_4,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub toggle: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
    pub events: C2RustUnnamed_5,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub repeat_info: C2RustUnnamed_7,
    pub events: C2RustUnnamed_6,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub rate: int32_t,
    pub delay: int32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_keyboard_modifiers {
    pub depressed: xkb_mod_mask_t,
    pub latched: xkb_mod_mask_t,
    pub locked: xkb_mod_mask_t,
    pub group: xkb_mod_mask_t,
}
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
pub type wlr_input_device_type = libc::c_uint;
pub const WLR_INPUT_DEVICE_SWITCH: wlr_input_device_type = 5;
pub const WLR_INPUT_DEVICE_TABLET_PAD: wlr_input_device_type = 4;
pub const WLR_INPUT_DEVICE_TABLET_TOOL: wlr_input_device_type = 3;
pub const WLR_INPUT_DEVICE_TOUCH: wlr_input_device_type = 2;
pub const WLR_INPUT_DEVICE_POINTER: wlr_input_device_type = 1;
pub const WLR_INPUT_DEVICE_KEYBOARD: wlr_input_device_type = 0;
/* Note: these are circular dependencies */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
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
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
#[no_mangle]
pub unsafe extern "C" fn create_libinput_touch(mut libinput_dev:
                                                   *mut libinput_device)
 -> *mut wlr_touch {
    if !libinput_dev.is_null() {
    } else {
        __assert_fail(b"libinput_dev\x00" as *const u8 as *const libc::c_char,
                      b"../backend/libinput/touch.c\x00" as *const u8 as
                          *const libc::c_char, 13i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"struct wlr_touch *create_libinput_touch(struct libinput_device *)\x00")).as_ptr());
    };
    let mut wlr_touch: *mut wlr_touch =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_touch>() as libc::c_ulong) as
            *mut wlr_touch;
    if wlr_touch.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unable to allocate wlr_touch\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/libinput/touch.c\x00" as *const u8 as
                     *const libc::c_char, 16i32);
        return 0 as *mut wlr_touch
    }
    wlr_touch_init(wlr_touch, 0 as *const wlr_touch_impl);
    return wlr_touch;
}
#[no_mangle]
pub unsafe extern "C" fn handle_touch_down(mut event: *mut libinput_event,
                                           mut libinput_dev:
                                               *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TOUCH, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a touch event for a device with no touch?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/touch.c\x00" as *const u8 as
                     *const libc::c_char, 28i32);
        return
    }
    let mut tevent: *mut libinput_event_touch =
        libinput_event_get_touch_event(event);
    let mut wlr_event: wlr_event_touch_down =
        {
            let mut init =
                wlr_event_touch_down{device: 0 as *mut wlr_input_device,
                                     time_msec: 0,
                                     touch_id: 0,
                                     x: 0.,
                                     y: 0.,};
            init
        };
    wlr_event.device = wlr_dev;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_touch_get_time_usec(tevent));
    wlr_event.touch_id = libinput_event_touch_get_seat_slot(tevent);
    wlr_event.x =
        libinput_event_touch_get_x_transformed(tevent, 1i32 as uint32_t);
    wlr_event.y =
        libinput_event_touch_get_y_transformed(tevent, 1i32 as uint32_t);
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.touch).events.down,
                         &mut wlr_event as *mut wlr_event_touch_down as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_touch_up(mut event: *mut libinput_event,
                                         mut libinput_dev:
                                             *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TOUCH, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a touch event for a device with no touch?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/touch.c\x00" as *const u8 as
                     *const libc::c_char, 48i32);
        return
    }
    let mut tevent: *mut libinput_event_touch =
        libinput_event_get_touch_event(event);
    let mut wlr_event: wlr_event_touch_up =
        {
            let mut init =
                wlr_event_touch_up{device: 0 as *mut wlr_input_device,
                                   time_msec: 0,
                                   touch_id: 0,};
            init
        };
    wlr_event.device = wlr_dev;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_touch_get_time_usec(tevent));
    wlr_event.touch_id = libinput_event_touch_get_seat_slot(tevent);
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.touch).events.up,
                         &mut wlr_event as *mut wlr_event_touch_up as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_touch_motion(mut event: *mut libinput_event,
                                             mut libinput_dev:
                                                 *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TOUCH, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a touch event for a device with no touch?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/touch.c\x00" as *const u8 as
                     *const libc::c_char, 66i32);
        return
    }
    let mut tevent: *mut libinput_event_touch =
        libinput_event_get_touch_event(event);
    let mut wlr_event: wlr_event_touch_motion =
        {
            let mut init =
                wlr_event_touch_motion{device: 0 as *mut wlr_input_device,
                                       time_msec: 0,
                                       touch_id: 0,
                                       x: 0.,
                                       y: 0.,};
            init
        };
    wlr_event.device = wlr_dev;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_touch_get_time_usec(tevent));
    wlr_event.touch_id = libinput_event_touch_get_seat_slot(tevent);
    wlr_event.x =
        libinput_event_touch_get_x_transformed(tevent, 1i32 as uint32_t);
    wlr_event.y =
        libinput_event_touch_get_y_transformed(tevent, 1i32 as uint32_t);
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.touch).events.motion,
                         &mut wlr_event as *mut wlr_event_touch_motion as
                             *mut libc::c_void);
}
// list of struct wl_list
#[no_mangle]
pub unsafe extern "C" fn handle_touch_cancel(mut event: *mut libinput_event,
                                             mut libinput_dev:
                                                 *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TOUCH, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a touch event for a device with no touch?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/touch.c\x00" as *const u8 as
                     *const libc::c_char, 86i32);
        return
    }
    let mut tevent: *mut libinput_event_touch =
        libinput_event_get_touch_event(event);
    let mut wlr_event: wlr_event_touch_cancel =
        {
            let mut init =
                wlr_event_touch_cancel{device: 0 as *mut wlr_input_device,
                                       time_msec: 0,
                                       touch_id: 0,};
            init
        };
    wlr_event.device = wlr_dev;
    wlr_event.time_msec =
        usec_to_msec(libinput_event_touch_get_time_usec(tevent));
    wlr_event.touch_id = libinput_event_touch_get_seat_slot(tevent);
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.touch).events.cancel,
                         &mut wlr_event as *mut wlr_event_touch_cancel as
                             *mut libc::c_void);
}
