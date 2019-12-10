use libc;
extern "C" {
    pub type libinput_device;
    pub type libinput_event;
    pub type libinput_event_keyboard;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_group;
    pub type wlr_pointer_impl;
    pub type wlr_tablet_pad_impl;
    pub type wlr_tablet_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_touch_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_switch_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn libinput_event_get_keyboard_event(event: *mut libinput_event)
     -> *mut libinput_event_keyboard;
    #[no_mangle]
    fn libinput_event_keyboard_get_time_usec(event:
                                                 *mut libinput_event_keyboard)
     -> uint64_t;
    #[no_mangle]
    fn libinput_event_keyboard_get_key(event: *mut libinput_event_keyboard)
     -> uint32_t;
    #[no_mangle]
    fn libinput_event_keyboard_get_key_state(event:
                                                 *mut libinput_event_keyboard)
     -> libinput_key_state;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn libinput_device_ref(device: *mut libinput_device)
     -> *mut libinput_device;
    #[no_mangle]
    fn libinput_device_unref(device: *mut libinput_device)
     -> *mut libinput_device;
    #[no_mangle]
    fn libinput_device_led_update(device: *mut libinput_device,
                                  leds: libinput_led);
    #[no_mangle]
    fn wlr_keyboard_notify_key(keyboard: *mut wlr_keyboard,
                               event: *mut wlr_event_keyboard_key);
    #[no_mangle]
    fn wlr_keyboard_init(keyboard: *mut wlr_keyboard,
                         impl_1: *const wlr_keyboard_impl);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn get_appropriate_device(desired_type: wlr_input_device_type,
                              device: *mut libinput_device)
     -> *mut wlr_input_device;
    #[no_mangle]
    fn usec_to_msec(usec: uint64_t) -> uint32_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type libinput_key_state = libc::c_uint;
pub const LIBINPUT_KEY_STATE_PRESSED: libinput_key_state = 1;
pub const LIBINPUT_KEY_STATE_RELEASED: libinput_key_state = 0;
pub type libinput_led = libc::c_uint;
pub const LIBINPUT_LED_SCROLL_LOCK: libinput_led = 4;
pub const LIBINPUT_LED_CAPS_LOCK: libinput_led = 2;
pub const LIBINPUT_LED_NUM_LOCK: libinput_led = 1;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
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
pub type wlr_input_device_type = libc::c_uint;
pub const WLR_INPUT_DEVICE_SWITCH: wlr_input_device_type = 5;
pub const WLR_INPUT_DEVICE_TABLET_PAD: wlr_input_device_type = 4;
pub const WLR_INPUT_DEVICE_TABLET_TOOL: wlr_input_device_type = 3;
pub const WLR_INPUT_DEVICE_TOUCH: wlr_input_device_type = 2;
pub const WLR_INPUT_DEVICE_POINTER: wlr_input_device_type = 1;
pub const WLR_INPUT_DEVICE_KEYBOARD: wlr_input_device_type = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_pointer {
    pub impl_0: *const crate::src::backend::headless::input_device::wlr_pointer_impl,
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_touch {
    pub impl_0: *const crate::src::backend::headless::input_device::wlr_touch_impl,
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
pub struct wlr_switch {
    pub impl_0: *mut crate::src::backend::headless::input_device::wlr_switch_impl,
    pub events: C2RustUnnamed_7,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub toggle: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* Note: these are circular dependencies */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_libinput_keyboard {
    pub wlr_keyboard: wlr_keyboard,
    pub libinput_dev: *mut libinput_device,
}
unsafe extern "C" fn get_libinput_keyboard_from_keyboard(mut wlr_kb:
                                                             *mut wlr_keyboard)
 -> *mut wlr_libinput_keyboard {
    if (*wlr_kb).impl_0 == &impl_0 as *const wlr_keyboard_impl {
    } else {
        __assert_fail(b"wlr_kb->impl == &impl\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/libinput/keyboard.c\x00" as *const u8 as
                          *const libc::c_char, 19i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[libc::c_char; 89]>(b"struct wlr_libinput_keyboard *get_libinput_keyboard_from_keyboard(struct wlr_keyboard *)\x00")).as_ptr());
    };
    return wlr_kb as *mut wlr_libinput_keyboard;
}
unsafe extern "C" fn keyboard_set_leds(mut wlr_kb: *mut wlr_keyboard,
                                       mut leds: uint32_t) {
    let mut kb: *mut wlr_libinput_keyboard =
        get_libinput_keyboard_from_keyboard(wlr_kb);
    libinput_device_led_update((*kb).libinput_dev, leds as libinput_led);
}
unsafe extern "C" fn keyboard_destroy(mut wlr_kb: *mut wlr_keyboard) {
    let mut kb: *mut wlr_libinput_keyboard =
        get_libinput_keyboard_from_keyboard(wlr_kb);
    libinput_device_unref((*kb).libinput_dev);
    free(kb as *mut libc::c_void);
}
static mut impl_0: wlr_keyboard_impl =
    {
    
        {
            let mut init =
                wlr_keyboard_impl{destroy:
                                      Some(keyboard_destroy as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_keyboard)
                                                   -> ()),
                                  led_update:
                                      Some(keyboard_set_leds as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_keyboard,
                                                                    _:
                                                                        uint32_t)
                                                   -> ()),};
            init
        }
};
#[no_mangle]
pub unsafe extern "C" fn create_libinput_keyboard(mut libinput_dev:
                                                      *mut libinput_device)
 -> *mut wlr_keyboard {
    let mut kb: *mut wlr_libinput_keyboard =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_libinput_keyboard>() as
                   libc::c_ulong) as *mut wlr_libinput_keyboard;
    if kb.is_null() { return 0 as *mut wlr_keyboard }
    (*kb).libinput_dev = libinput_dev;
    libinput_device_ref(libinput_dev);
    libinput_device_led_update(libinput_dev, 0 as libinput_led);
    let mut wlr_kb: *mut wlr_keyboard = &mut (*kb).wlr_keyboard;
    wlr_keyboard_init(wlr_kb, &impl_0);
    return wlr_kb;
}
// list of struct wl_list
#[no_mangle]
pub unsafe extern "C" fn handle_keyboard_key(mut event: *mut libinput_event,
                                             mut libinput_dev:
                                                 *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_KEYBOARD, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a keyboard event for a device with no keyboards?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/keyboard.c\x00" as *const u8 as
                     *const libc::c_char, 62i32);
        return
    }
    let mut kbevent: *mut libinput_event_keyboard =
        libinput_event_get_keyboard_event(event);
    let mut wlr_event: wlr_event_keyboard_key =
        {
            let mut init =
                wlr_event_keyboard_key{time_msec: 0i32 as uint32_t,
                                       keycode: 0,
                                       update_state: false,
                                       state: WLR_KEY_RELEASED,};
            init
        };
    wlr_event.time_msec =
        usec_to_msec(libinput_event_keyboard_get_time_usec(kbevent));
    wlr_event.keycode = libinput_event_keyboard_get_key(kbevent);
    let mut state: libinput_key_state =
        libinput_event_keyboard_get_key_state(kbevent);
    match state as libc::c_uint {
        0 => { wlr_event.state = WLR_KEY_RELEASED }
        1 => { wlr_event.state = WLR_KEY_PRESSED }
        _ => { }
    }
    wlr_event.update_state = 1i32 != 0;
    wlr_keyboard_notify_key((*wlr_dev).c2rust_unnamed.keyboard,
                            &mut wlr_event);
}
