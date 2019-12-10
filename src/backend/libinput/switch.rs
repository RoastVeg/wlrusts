use libc;
extern "C" {
    pub type libinput_device;
    pub type libinput_event;
    pub type libinput_event_switch;
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
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn libinput_event_get_switch_event(event: *mut libinput_event)
     -> *mut libinput_event_switch;
    #[no_mangle]
    fn libinput_event_switch_get_switch(event: *mut libinput_event_switch)
     -> libinput_switch;
    #[no_mangle]
    fn libinput_event_switch_get_switch_state(event:
                                                  *mut libinput_event_switch)
     -> libinput_switch_state;
    #[no_mangle]
    fn libinput_event_switch_get_time_usec(event: *mut libinput_event_switch)
     -> uint64_t;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn libinput_device_get_name(device: *mut libinput_device)
     -> *const libc::c_char;
    #[no_mangle]
    fn wlr_switch_init(switch_device: *mut wlr_switch,
                       impl_0: *mut wlr_switch_impl);
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
pub type libinput_switch_state = libc::c_uint;
pub const LIBINPUT_SWITCH_STATE_ON: libinput_switch_state = 1;
pub const LIBINPUT_SWITCH_STATE_OFF: libinput_switch_state = 0;
pub type libinput_switch = libc::c_uint;
pub const LIBINPUT_SWITCH_TABLET_MODE: libinput_switch = 2;
pub const LIBINPUT_SWITCH_LID: libinput_switch = 1;
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
// Or 0 if not applicable to this device
/* wlr_input_device.type determines which of these is valid */
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_switch_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_switch) -> ()>,
}
/* Note: these are circular dependencies */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
}
pub type wlr_switch_type = libc::c_uint;
pub const WLR_SWITCH_TYPE_TABLET_MODE: wlr_switch_type = 2;
pub const WLR_SWITCH_TYPE_LID: wlr_switch_type = 1;
pub type wlr_switch_state = libc::c_uint;
pub const WLR_SWITCH_STATE_TOGGLE: wlr_switch_state = 2;
pub const WLR_SWITCH_STATE_ON: wlr_switch_state = 1;
pub const WLR_SWITCH_STATE_OFF: wlr_switch_state = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_switch_toggle {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub switch_type: wlr_switch_type,
    pub switch_state: wlr_switch_state,
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
#[no_mangle]
pub unsafe extern "C" fn create_libinput_switch(mut libinput_dev:
                                                    *mut libinput_device)
 -> *mut wlr_switch {
    if !libinput_dev.is_null() {
    } else {
        __assert_fail(b"libinput_dev\x00" as *const u8 as *const libc::c_char,
                      b"../backend/libinput/switch.c\x00" as *const u8 as
                          *const libc::c_char, 13i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"struct wlr_switch *create_libinput_switch(struct libinput_device *)\x00")).as_ptr());
    };
    let mut wlr_switch: *mut wlr_switch =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_switch>() as libc::c_ulong) as
            *mut wlr_switch;
    if wlr_switch.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unable to allocate wlr_switch\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/libinput/switch.c\x00" as *const u8 as
                     *const libc::c_char, 16i32);
        return 0 as *mut wlr_switch
    }
    wlr_switch_init(wlr_switch, 0 as *mut wlr_switch_impl);
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Created switch for device %s\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/libinput/switch.c\x00" as *const u8 as
                 *const libc::c_char, 20i32,
             libinput_device_get_name(libinput_dev));
    return wlr_switch;
}
// list of struct wl_list
#[no_mangle]
pub unsafe extern "C" fn handle_switch_toggle(mut event: *mut libinput_event,
                                              mut libinput_dev:
                                                  *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_SWITCH, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a switch event for a device with no switch?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/switch.c\x00" as *const u8 as
                     *const libc::c_char, 29i32);
        return
    }
    let mut sevent: *mut libinput_event_switch =
        libinput_event_get_switch_event(event);
    let mut wlr_event: wlr_event_switch_toggle =
        {
            let mut init =
                wlr_event_switch_toggle{device: 0 as *mut wlr_input_device,
                                        time_msec: 0,
                                        switch_type: 0 as wlr_switch_type,
                                        switch_state: WLR_SWITCH_STATE_OFF,};
            init
        };
    wlr_event.device = wlr_dev;
    match libinput_event_switch_get_switch(sevent) as libc::c_uint {
        1 => { wlr_event.switch_type = WLR_SWITCH_TYPE_LID }
        2 => { wlr_event.switch_type = WLR_SWITCH_TYPE_TABLET_MODE }
        _ => { }
    }
    match libinput_event_switch_get_switch_state(sevent) as libc::c_uint {
        0 => { wlr_event.switch_state = WLR_SWITCH_STATE_OFF }
        1 => { wlr_event.switch_state = WLR_SWITCH_STATE_ON }
        _ => { }
    }
    wlr_event.time_msec =
        usec_to_msec(libinput_event_switch_get_time_usec(sevent));
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.switch_device).events.toggle,
                         &mut wlr_event as *mut wlr_event_switch_toggle as
                             *mut libc::c_void);
}
