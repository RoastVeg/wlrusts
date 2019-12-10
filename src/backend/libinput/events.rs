use libc;
extern "C" {
    pub type udev;
    pub type udev_monitor;
    pub type libinput;
    pub type libinput_device;
    pub type libinput_event;
    pub type wl_event_source;
    pub type wl_display;
    pub type session_impl;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_pointer_impl;
    pub type wlr_tablet_pad_impl;
    pub type wlr_tablet_impl;
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    pub type wlr_renderer;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn libinput_event_get_type(event: *mut libinput_event)
     -> libinput_event_type;
    #[no_mangle]
    fn libinput_event_get_device(event: *mut libinput_event)
     -> *mut libinput_device;
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
    fn libinput_device_set_user_data(device: *mut libinput_device,
                                     user_data: *mut libc::c_void);
    #[no_mangle]
    fn libinput_device_get_user_data(device: *mut libinput_device)
     -> *mut libc::c_void;
    #[no_mangle]
    fn libinput_device_get_name(device: *mut libinput_device)
     -> *const libc::c_char;
    #[no_mangle]
    fn libinput_device_get_id_product(device: *mut libinput_device)
     -> libc::c_uint;
    #[no_mangle]
    fn libinput_device_get_id_vendor(device: *mut libinput_device)
     -> libc::c_uint;
    #[no_mangle]
    fn libinput_device_get_output_name(device: *mut libinput_device)
     -> *const libc::c_char;
    #[no_mangle]
    fn libinput_device_has_capability(device: *mut libinput_device,
                                      capability: libinput_device_capability)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_device_get_size(device: *mut libinput_device,
                                width: *mut libc::c_double,
                                height: *mut libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wlr_input_device_destroy(dev: *mut wlr_input_device);
    #[no_mangle]
    fn wlr_input_device_init(wlr_device: *mut wlr_input_device,
                             type_0: wlr_input_device_type,
                             impl_0: *const wlr_input_device_impl,
                             name: *const libc::c_char, vendor: libc::c_int,
                             product: libc::c_int);
    /* *
 * Add `item` to the end of a list.
 * Returns: new list length or `-1` on failure.
 */
    #[no_mangle]
    fn wlr_list_push(list: *mut wlr_list, item: *mut libc::c_void) -> ssize_t;
    /* *
 * Remove an item from the list.
 */
    #[no_mangle]
    fn wlr_list_del(list: *mut wlr_list, index: size_t);
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn create_libinput_keyboard(device: *mut libinput_device)
     -> *mut wlr_keyboard;
    #[no_mangle]
    fn create_libinput_pointer(device: *mut libinput_device)
     -> *mut wlr_pointer;
    #[no_mangle]
    fn create_libinput_touch(device: *mut libinput_device) -> *mut wlr_touch;
    #[no_mangle]
    fn create_libinput_tablet(device: *mut libinput_device)
     -> *mut wlr_tablet;
    #[no_mangle]
    fn create_libinput_tablet_pad(device: *mut libinput_device)
     -> *mut wlr_tablet_pad;
    #[no_mangle]
    fn create_libinput_switch(device: *mut libinput_device)
     -> *mut wlr_switch;
    #[no_mangle]
    fn handle_keyboard_key(event: *mut libinput_event,
                           device: *mut libinput_device);
    #[no_mangle]
    fn handle_pointer_motion(event: *mut libinput_event,
                             device: *mut libinput_device);
    #[no_mangle]
    fn handle_pointer_motion_abs(event: *mut libinput_event,
                                 device: *mut libinput_device);
    #[no_mangle]
    fn handle_pointer_button(event: *mut libinput_event,
                             device: *mut libinput_device);
    #[no_mangle]
    fn handle_pointer_axis(event: *mut libinput_event,
                           device: *mut libinput_device);
    #[no_mangle]
    fn handle_touch_down(event: *mut libinput_event,
                         device: *mut libinput_device);
    #[no_mangle]
    fn handle_touch_up(event: *mut libinput_event,
                       device: *mut libinput_device);
    #[no_mangle]
    fn handle_touch_motion(event: *mut libinput_event,
                           device: *mut libinput_device);
    #[no_mangle]
    fn handle_touch_cancel(event: *mut libinput_event,
                           device: *mut libinput_device);
    #[no_mangle]
    fn handle_tablet_tool_axis(event: *mut libinput_event,
                               device: *mut libinput_device);
    #[no_mangle]
    fn handle_tablet_tool_proximity(event: *mut libinput_event,
                                    device: *mut libinput_device);
    #[no_mangle]
    fn handle_tablet_tool_tip(event: *mut libinput_event,
                              device: *mut libinput_device);
    #[no_mangle]
    fn handle_tablet_tool_button(event: *mut libinput_event,
                                 device: *mut libinput_device);
    #[no_mangle]
    fn handle_tablet_pad_button(event: *mut libinput_event,
                                device: *mut libinput_device);
    #[no_mangle]
    fn handle_tablet_pad_ring(event: *mut libinput_event,
                              device: *mut libinput_device);
    #[no_mangle]
    fn handle_tablet_pad_strip(event: *mut libinput_event,
                               device: *mut libinput_device);
    #[no_mangle]
    fn handle_switch_toggle(event: *mut libinput_event,
                            device: *mut libinput_device);
    #[no_mangle]
    fn handle_pointer_swipe_begin(event: *mut libinput_event,
                                  device: *mut libinput_device);
    #[no_mangle]
    fn handle_pointer_swipe_update(event: *mut libinput_event,
                                   device: *mut libinput_device);
    #[no_mangle]
    fn handle_pointer_swipe_end(event: *mut libinput_event,
                                device: *mut libinput_device);
    #[no_mangle]
    fn handle_pointer_pinch_begin(event: *mut libinput_event,
                                  device: *mut libinput_device);
    #[no_mangle]
    fn handle_pointer_pinch_update(event: *mut libinput_event,
                                   device: *mut libinput_device);
    #[no_mangle]
    fn handle_pointer_pinch_end(event: *mut libinput_event,
                                device: *mut libinput_device);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
pub type libinput_device_capability = libc::c_uint;
pub const LIBINPUT_DEVICE_CAP_SWITCH: libinput_device_capability = 6;
pub const LIBINPUT_DEVICE_CAP_GESTURE: libinput_device_capability = 5;
pub const LIBINPUT_DEVICE_CAP_TABLET_PAD: libinput_device_capability = 4;
pub const LIBINPUT_DEVICE_CAP_TABLET_TOOL: libinput_device_capability = 3;
pub const LIBINPUT_DEVICE_CAP_TOUCH: libinput_device_capability = 2;
pub const LIBINPUT_DEVICE_CAP_POINTER: libinput_device_capability = 1;
pub const LIBINPUT_DEVICE_CAP_KEYBOARD: libinput_device_capability = 0;
pub type libinput_event_type = libc::c_uint;
pub const LIBINPUT_EVENT_SWITCH_TOGGLE: libinput_event_type = 900;
pub const LIBINPUT_EVENT_GESTURE_PINCH_END: libinput_event_type = 805;
pub const LIBINPUT_EVENT_GESTURE_PINCH_UPDATE: libinput_event_type = 804;
pub const LIBINPUT_EVENT_GESTURE_PINCH_BEGIN: libinput_event_type = 803;
pub const LIBINPUT_EVENT_GESTURE_SWIPE_END: libinput_event_type = 802;
pub const LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE: libinput_event_type = 801;
pub const LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN: libinput_event_type = 800;
pub const LIBINPUT_EVENT_TABLET_PAD_STRIP: libinput_event_type = 702;
pub const LIBINPUT_EVENT_TABLET_PAD_RING: libinput_event_type = 701;
pub const LIBINPUT_EVENT_TABLET_PAD_BUTTON: libinput_event_type = 700;
pub const LIBINPUT_EVENT_TABLET_TOOL_BUTTON: libinput_event_type = 603;
pub const LIBINPUT_EVENT_TABLET_TOOL_TIP: libinput_event_type = 602;
pub const LIBINPUT_EVENT_TABLET_TOOL_PROXIMITY: libinput_event_type = 601;
pub const LIBINPUT_EVENT_TABLET_TOOL_AXIS: libinput_event_type = 600;
pub const LIBINPUT_EVENT_TOUCH_FRAME: libinput_event_type = 504;
pub const LIBINPUT_EVENT_TOUCH_CANCEL: libinput_event_type = 503;
pub const LIBINPUT_EVENT_TOUCH_MOTION: libinput_event_type = 502;
pub const LIBINPUT_EVENT_TOUCH_UP: libinput_event_type = 501;
pub const LIBINPUT_EVENT_TOUCH_DOWN: libinput_event_type = 500;
pub const LIBINPUT_EVENT_POINTER_AXIS: libinput_event_type = 403;
pub const LIBINPUT_EVENT_POINTER_BUTTON: libinput_event_type = 402;
pub const LIBINPUT_EVENT_POINTER_MOTION_ABSOLUTE: libinput_event_type = 401;
pub const LIBINPUT_EVENT_POINTER_MOTION: libinput_event_type = 400;
pub const LIBINPUT_EVENT_KEYBOARD_KEY: libinput_event_type = 300;
pub const LIBINPUT_EVENT_DEVICE_REMOVED: libinput_event_type = 2;
pub const LIBINPUT_EVENT_DEVICE_ADDED: libinput_event_type = 1;
pub const LIBINPUT_EVENT_NONE: libinput_event_type = 0;

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
pub struct wl_signal {
    pub listener_list: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_session {
    pub impl_0: *const crate::src::backend::session::direct::session_impl,
    pub session_signal: wl_signal,
    pub active: bool,
    pub vtnr: libc::c_uint,
    pub seat: [libc::c_char; 256],
    pub udev: *mut udev,
    pub mon: *mut udev_monitor,
    pub udev_event: *mut wl_event_source,
    pub devices: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

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
    pub events: C2RustUnnamed_5,
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
pub struct C2RustUnnamed_5 {
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

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
pub struct wlr_backend_impl {
    pub start: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> ()>,
    pub get_renderer: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                 -> *mut crate::src::backend::drm::atomic::wlr_renderer>,
    pub get_session: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                -> *mut wlr_session>,
    pub get_presentation_clock: Option<unsafe extern "C" fn(_:
                                                                *mut wlr_backend)
                                           -> clockid_t>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed_9,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_libinput_input_device {
    pub wlr_input_device: wlr_input_device,
    pub handle: *mut libinput_device,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_libinput_backend {
    pub backend: wlr_backend,
    pub session: *mut wlr_session,
    pub display: *mut wl_display,
    pub libinput_context: *mut libinput,
    pub input_event: *mut wl_event_source,
    pub display_destroy: wl_listener,
    pub session_destroy: wl_listener,
    pub session_signal: wl_listener,
    pub wlr_device_lists: wlr_list,
}
unsafe extern "C" fn get_libinput_device_from_device(mut wlr_dev:
                                                         *mut wlr_input_device)
 -> *mut wlr_libinput_input_device {
    if wlr_input_device_is_libinput(wlr_dev) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_input_device_is_libinput(wlr_dev)\x00" as
                          *const u8 as *const libc::c_char,
                      b"../backend/libinput/events.c\x00" as *const u8 as
                          *const libc::c_char, 14i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 93],
                                                &[libc::c_char; 93]>(b"struct wlr_libinput_input_device *get_libinput_device_from_device(struct wlr_input_device *)\x00")).as_ptr());
    };
    return wlr_dev as *mut wlr_libinput_input_device;
}
#[no_mangle]
pub unsafe extern "C" fn get_appropriate_device(mut desired_type:
                                                    wlr_input_device_type,
                                                mut libinput_dev:
                                                    *mut libinput_device)
 -> *mut wlr_input_device {
    let mut wlr_devices: *mut wl_list =
        libinput_device_get_user_data(libinput_dev) as *mut wl_list;
    if wlr_devices.is_null() { return 0 as *mut wlr_input_device }
    let mut dev: *mut wlr_input_device = 0 as *mut wlr_input_device;
    dev =
        ((*wlr_devices).next as *mut libc::c_char).offset(-88) as
            *mut wlr_input_device;
    while &mut (*dev).link as *mut wl_list != wlr_devices {
        if (*dev).type_0 as libc::c_uint == desired_type as libc::c_uint {
            return dev
        }
        dev =
            ((*dev).link.next as *mut libc::c_char).offset(-88) as
                *mut wlr_input_device
    }
    return 0 as *mut wlr_input_device;
}
unsafe extern "C" fn input_device_destroy(mut wlr_dev:
                                              *mut wlr_input_device) {
    let mut dev: *mut wlr_libinput_input_device =
        get_libinput_device_from_device(wlr_dev);
    libinput_device_unref((*dev).handle);
    wl_list_remove(&mut (*dev).wlr_input_device.link);
    free(dev as *mut libc::c_void);
}
static mut input_device_impl: wlr_input_device_impl =
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
unsafe extern "C" fn allocate_device(mut backend: *mut wlr_libinput_backend,
                                     mut libinput_dev: *mut libinput_device,
                                     mut wlr_devices: *mut wl_list,
                                     mut type_0: wlr_input_device_type)
 -> *mut wlr_input_device {
    let mut vendor: libc::c_int =
        libinput_device_get_id_vendor(libinput_dev) as libc::c_int;
    let mut product: libc::c_int =
        libinput_device_get_id_product(libinput_dev) as libc::c_int;
    let mut name: *const libc::c_char =
        libinput_device_get_name(libinput_dev);
    let mut dev: *mut wlr_libinput_input_device =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_libinput_input_device>() as
                   libc::c_ulong) as *mut wlr_libinput_input_device;
    if dev.is_null() { return 0 as *mut wlr_input_device }
    let mut wlr_dev: *mut wlr_input_device = &mut (*dev).wlr_input_device;
    libinput_device_get_size(libinput_dev, &mut (*wlr_dev).width_mm,
                             &mut (*wlr_dev).height_mm);
    let mut output_name: *const libc::c_char =
        libinput_device_get_output_name(libinput_dev);
    if !output_name.is_null() { (*wlr_dev).output_name = strdup(output_name) }
    wl_list_insert(wlr_devices, &mut (*wlr_dev).link);
    (*dev).handle = libinput_dev;
    libinput_device_ref(libinput_dev);
    wlr_input_device_init(wlr_dev, type_0, &input_device_impl, name, vendor,
                          product);
    return wlr_dev;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* * Gets the underlying libinput_device handle for the given wlr_input_device */
#[no_mangle]
pub unsafe extern "C" fn wlr_input_device_is_libinput(mut wlr_dev:
                                                          *mut wlr_input_device)
 -> bool {
    return (*wlr_dev).impl_0 ==
               &input_device_impl as *const wlr_input_device_impl;
}
unsafe extern "C" fn handle_device_added(mut backend:
                                             *mut wlr_libinput_backend,
                                         mut libinput_dev:
                                             *mut libinput_device) {
    let mut current_block: u64;
    /*
	 * Note: the wlr API exposes only devices with a single capability, because
	 * that meshes better with how Wayland does things and is a bit simpler.
	 * However, libinput devices often have multiple capabilities - in such
	 * cases we have to create several devices.
	 */
    let mut vendor: libc::c_int =
        libinput_device_get_id_vendor(libinput_dev) as libc::c_int;
    let mut product: libc::c_int =
        libinput_device_get_id_product(libinput_dev) as libc::c_int;
    let mut name: *const libc::c_char =
        libinput_device_get_name(libinput_dev);
    let mut wlr_devices: *mut wl_list =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wl_list>() as libc::c_ulong) as
            *mut wl_list;
    if wlr_devices.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/libinput/events.c\x00" as *const u8 as
                     *const libc::c_char, 90i32);
        return
    }
    wl_list_init(wlr_devices);
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Added %s [%d:%d]\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/libinput/events.c\x00" as *const u8 as
                 *const libc::c_char, 94i32, name, vendor, product);
    if libinput_device_has_capability(libinput_dev,
                                      LIBINPUT_DEVICE_CAP_KEYBOARD) != 0 {
        let mut wlr_dev: *mut wlr_input_device =
            allocate_device(backend, libinput_dev, wlr_devices,
                            WLR_INPUT_DEVICE_KEYBOARD);
        if wlr_dev.is_null() {
            current_block = 8257739874863874789;
        } else {
            (*wlr_dev).c2rust_unnamed.keyboard =
                create_libinput_keyboard(libinput_dev);
            if (*wlr_dev).c2rust_unnamed.keyboard.is_null() {
                free(wlr_dev as *mut libc::c_void);
                current_block = 8257739874863874789;
            } else {
                wlr_signal_emit_safe(&mut (*backend).backend.events.new_input,
                                     wlr_dev as *mut libc::c_void);
                current_block = 7976072742316086414;
            }
        }
    } else { current_block = 7976072742316086414; }
    match current_block {
        7976072742316086414 => {
            if libinput_device_has_capability(libinput_dev,
                                              LIBINPUT_DEVICE_CAP_POINTER) !=
                   0 {
                let mut wlr_dev_0: *mut wlr_input_device =
                    allocate_device(backend, libinput_dev, wlr_devices,
                                    WLR_INPUT_DEVICE_POINTER);
                if wlr_dev_0.is_null() {
                    current_block = 8257739874863874789;
                } else {
                    (*wlr_dev_0).c2rust_unnamed.pointer =
                        create_libinput_pointer(libinput_dev);
                    if (*wlr_dev_0).c2rust_unnamed.pointer.is_null() {
                        free(wlr_dev_0 as *mut libc::c_void);
                        current_block = 8257739874863874789;
                    } else {
                        wlr_signal_emit_safe(&mut (*backend).backend.events.new_input,
                                             wlr_dev_0 as *mut libc::c_void);
                        current_block = 11307063007268554308;
                    }
                }
            } else { current_block = 11307063007268554308; }
            match current_block {
                8257739874863874789 => { }
                _ => {
                    if libinput_device_has_capability(libinput_dev,
                                                      LIBINPUT_DEVICE_CAP_TOUCH)
                           != 0 {
                        let mut wlr_dev_1: *mut wlr_input_device =
                            allocate_device(backend, libinput_dev,
                                            wlr_devices,
                                            WLR_INPUT_DEVICE_TOUCH);
                        if wlr_dev_1.is_null() {
                            current_block = 8257739874863874789;
                        } else {
                            (*wlr_dev_1).c2rust_unnamed.touch =
                                create_libinput_touch(libinput_dev);
                            if (*wlr_dev_1).c2rust_unnamed.touch.is_null() {
                                free(wlr_dev_1 as *mut libc::c_void);
                                current_block = 8257739874863874789;
                            } else {
                                wlr_signal_emit_safe(&mut (*backend).backend.events.new_input,
                                                     wlr_dev_1 as
                                                         *mut libc::c_void);
                                current_block = 15897653523371991391;
                            }
                        }
                    } else { current_block = 15897653523371991391; }
                    match current_block {
                        8257739874863874789 => { }
                        _ => {
                            if libinput_device_has_capability(libinput_dev,
                                                              LIBINPUT_DEVICE_CAP_TABLET_TOOL)
                                   != 0 {
                                let mut wlr_dev_2: *mut wlr_input_device =
                                    allocate_device(backend, libinput_dev,
                                                    wlr_devices,
                                                    WLR_INPUT_DEVICE_TABLET_TOOL);
                                if wlr_dev_2.is_null() {
                                    current_block = 8257739874863874789;
                                } else {
                                    (*wlr_dev_2).c2rust_unnamed.tablet =
                                        create_libinput_tablet(libinput_dev);
                                    if (*wlr_dev_2).c2rust_unnamed.tablet.is_null()
                                       {
                                        free(wlr_dev_2 as *mut libc::c_void);
                                        current_block = 8257739874863874789;
                                    } else {
                                        wlr_signal_emit_safe(&mut (*backend).backend.events.new_input,
                                                             wlr_dev_2 as
                                                                 *mut libc::c_void);
                                        current_block = 11743904203796629665;
                                    }
                                }
                            } else { current_block = 11743904203796629665; }
                            match current_block {
                                8257739874863874789 => { }
                                _ => {
                                    if libinput_device_has_capability(libinput_dev,
                                                                      LIBINPUT_DEVICE_CAP_TABLET_PAD)
                                           != 0 {
                                        let mut wlr_dev_3:
                                                *mut wlr_input_device =
                                            allocate_device(backend,
                                                            libinput_dev,
                                                            wlr_devices,
                                                            WLR_INPUT_DEVICE_TABLET_PAD);
                                        if wlr_dev_3.is_null() {
                                            current_block =
                                                8257739874863874789;
                                        } else {
                                            (*wlr_dev_3).c2rust_unnamed.tablet_pad
                                                =
                                                create_libinput_tablet_pad(libinput_dev);
                                            if (*wlr_dev_3).c2rust_unnamed.tablet_pad.is_null()
                                               {
                                                free(wlr_dev_3 as
                                                         *mut libc::c_void);
                                                current_block =
                                                    8257739874863874789;
                                            } else {
                                                wlr_signal_emit_safe(&mut (*backend).backend.events.new_input,
                                                                     wlr_dev_3
                                                                         as
                                                                         *mut libc::c_void);
                                                current_block =
                                                    12497913735442871383;
                                            }
                                        }
                                    } else {
                                        current_block = 12497913735442871383;
                                    }
                                    match current_block {
                                        8257739874863874789 => { }
                                        _ => {
                                            (libinput_device_has_capability(libinput_dev,
                                                                            LIBINPUT_DEVICE_CAP_GESTURE))
                                                != 0;
                                            if libinput_device_has_capability(libinput_dev,
                                                                              LIBINPUT_DEVICE_CAP_SWITCH)
                                                   != 0 {
                                                let mut wlr_dev_4:
                                                        *mut wlr_input_device =
                                                    allocate_device(backend,
                                                                    libinput_dev,
                                                                    wlr_devices,
                                                                    WLR_INPUT_DEVICE_SWITCH);
                                                if wlr_dev_4.is_null() {
                                                    current_block =
                                                        8257739874863874789;
                                                } else {
                                                    (*wlr_dev_4).c2rust_unnamed.switch_device
                                                        =
                                                        create_libinput_switch(libinput_dev);
                                                    if (*wlr_dev_4).c2rust_unnamed.switch_device.is_null()
                                                       {
                                                        free(wlr_dev_4 as
                                                                 *mut libc::c_void);
                                                        current_block =
                                                            8257739874863874789;
                                                    } else {
                                                        wlr_signal_emit_safe(&mut (*backend).backend.events.new_input,
                                                                             wlr_dev_4
                                                                                 as
                                                                                 *mut libc::c_void);
                                                        current_block =
                                                            9241535491006583629;
                                                    }
                                                }
                                            } else {
                                                current_block =
                                                    9241535491006583629;
                                            }
                                            match current_block {
                                                8257739874863874789 => { }
                                                _ => {
                                                    if wl_list_empty(wlr_devices)
                                                           == 0 {
                                                        libinput_device_set_user_data(libinput_dev,
                                                                                      wlr_devices
                                                                                          as
                                                                                          *mut libc::c_void);
                                                        wlr_list_push(&mut (*backend).wlr_device_lists,
                                                                      wlr_devices
                                                                          as
                                                                          *mut libc::c_void);
                                                    } else {
                                                        free(wlr_devices as
                                                                 *mut libc::c_void);
                                                    }
                                                    return
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    _wlr_log(WLR_ERROR,
             b"[%s:%d] Could not allocate new device\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/libinput/events.c\x00" as *const u8 as
                 *const libc::c_char, 194i32);
    let mut dev: *mut wlr_input_device = 0 as *mut wlr_input_device;
    let mut tmp_dev: *mut wlr_input_device = 0 as *mut wlr_input_device;
    dev =
        ((*wlr_devices).next as *mut libc::c_char).offset(-88) as
            *mut wlr_input_device;
    tmp_dev =
        ((*dev).link.next as *mut libc::c_char).offset(-88) as
            *mut wlr_input_device;
    while &mut (*dev).link as *mut wl_list != wlr_devices {
        free(dev as *mut libc::c_void);
        dev = tmp_dev;
        tmp_dev =
            ((*dev).link.next as *mut libc::c_char).offset(-88) as
                *mut wlr_input_device
    }
    free(wlr_devices as *mut libc::c_void);
}
unsafe extern "C" fn handle_device_removed(mut backend:
                                               *mut wlr_libinput_backend,
                                           mut libinput_dev:
                                               *mut libinput_device) {
    let mut wlr_devices: *mut wl_list =
        libinput_device_get_user_data(libinput_dev) as *mut wl_list;
    let mut vendor: libc::c_int =
        libinput_device_get_id_vendor(libinput_dev) as libc::c_int;
    let mut product: libc::c_int =
        libinput_device_get_id_product(libinput_dev) as libc::c_int;
    let mut name: *const libc::c_char =
        libinput_device_get_name(libinput_dev);
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Removing %s [%d:%d]\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/libinput/events.c\x00" as *const u8 as
                 *const libc::c_char, 208i32, name, vendor, product);
    if wlr_devices.is_null() { return }
    let mut dev: *mut wlr_input_device = 0 as *mut wlr_input_device;
    let mut tmp_dev: *mut wlr_input_device = 0 as *mut wlr_input_device;
    dev =
        ((*wlr_devices).next as *mut libc::c_char).offset(-88) as
            *mut wlr_input_device;
    tmp_dev =
        ((*dev).link.next as *mut libc::c_char).offset(-88) as
            *mut wlr_input_device;
    while &mut (*dev).link as *mut wl_list != wlr_devices {
        wlr_input_device_destroy(dev);
        dev = tmp_dev;
        tmp_dev =
            ((*dev).link.next as *mut libc::c_char).offset(-88) as
                *mut wlr_input_device
    }
    let mut i: size_t = 0i32 as size_t;
    while i < (*backend).wlr_device_lists.length {
        if *(*backend).wlr_device_lists.items.offset(i as isize) ==
               wlr_devices as *mut libc::c_void {
            wlr_list_del(&mut (*backend).wlr_device_lists, i);
            break ;
        } else { i = i.wrapping_add(1) }
    }
    free(wlr_devices as *mut libc::c_void);
}
// list of struct wl_list
#[no_mangle]
pub unsafe extern "C" fn handle_libinput_event(mut backend:
                                                   *mut wlr_libinput_backend,
                                               mut event:
                                                   *mut libinput_event) {
    let mut libinput_dev: *mut libinput_device =
        libinput_event_get_device(event);
    let mut event_type: libinput_event_type = libinput_event_get_type(event);
    match event_type as libc::c_uint {
        1 => { handle_device_added(backend, libinput_dev); }
        2 => { handle_device_removed(backend, libinput_dev); }
        300 => { handle_keyboard_key(event, libinput_dev); }
        400 => { handle_pointer_motion(event, libinput_dev); }
        401 => { handle_pointer_motion_abs(event, libinput_dev); }
        402 => { handle_pointer_button(event, libinput_dev); }
        403 => { handle_pointer_axis(event, libinput_dev); }
        500 => { handle_touch_down(event, libinput_dev); }
        501 => { handle_touch_up(event, libinput_dev); }
        502 => { handle_touch_motion(event, libinput_dev); }
        503 => { handle_touch_cancel(event, libinput_dev); }
        504 => { }
        600 => { handle_tablet_tool_axis(event, libinput_dev); }
        601 => { handle_tablet_tool_proximity(event, libinput_dev); }
        602 => { handle_tablet_tool_tip(event, libinput_dev); }
        603 => { handle_tablet_tool_button(event, libinput_dev); }
        700 => { handle_tablet_pad_button(event, libinput_dev); }
        701 => { handle_tablet_pad_ring(event, libinput_dev); }
        702 => { handle_tablet_pad_strip(event, libinput_dev); }
        900 => { handle_switch_toggle(event, libinput_dev); }
        800 => { handle_pointer_swipe_begin(event, libinput_dev); }
        801 => { handle_pointer_swipe_update(event, libinput_dev); }
        802 => { handle_pointer_swipe_end(event, libinput_dev); }
        803 => { handle_pointer_pinch_begin(event, libinput_dev); }
        804 => { handle_pointer_pinch_update(event, libinput_dev); }
        805 => { handle_pointer_pinch_end(event, libinput_dev); }
        _ => {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] Unknown libinput event %d\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/libinput/events.c\x00" as *const u8 as
                         *const libc::c_char, 309i32,
                     event_type as libc::c_uint);
        }
    };
}
