use libc;
extern "C" {
    pub type xkb_keymap;
    pub type xkb_state;
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
    pub type wlr_tablet_impl;
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    pub type wlr_pointer_impl;
    /* Note: these are circular dependencies */
    pub type wlr_input_device_impl;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn xkb_keymap_get_as_string(keymap: *mut xkb_keymap,
                                format: xkb_keymap_format)
     -> *mut libc::c_char;
    #[no_mangle]
    fn wlr_keyboard_init(keyboard: *mut wlr_keyboard,
                         impl_1: *const wlr_keyboard_impl);
    #[no_mangle]
    fn wlr_keyboard_destroy(keyboard: *mut wlr_keyboard);
    #[no_mangle]
    fn wlr_keyboard_notify_key(keyboard: *mut wlr_keyboard,
                               event: *mut wlr_event_keyboard_key);
    #[no_mangle]
    fn wlr_keyboard_notify_modifiers(keyboard: *mut wlr_keyboard,
                                     mods_depressed: uint32_t,
                                     mods_latched: uint32_t,
                                     mods_locked: uint32_t, group: uint32_t);
    #[no_mangle]
    fn wlr_keyboard_set_keymap(kb: *mut wlr_keyboard,
                               keymap: *mut xkb_keymap);
    /* *
 * Sets the keyboard repeat info. `rate` is in key repeats/second and delay is
 * in milliseconds.
 */
    #[no_mangle]
    fn wlr_keyboard_set_repeat_info(kb: *mut wlr_keyboard, rate: int32_t,
                                    delay: int32_t);
    #[no_mangle]
    fn wlr_keyboard_led_update(keyboard: *mut wlr_keyboard, leds: uint32_t);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
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
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
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
pub struct wl_signal {
    pub listener_list: wl_list,
}
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
pub type xkb_keymap_format = libc::c_uint;
pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_keyboard_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_keyboard) -> ()>,
    pub led_update: Option<unsafe extern "C" fn(_: *mut wlr_keyboard,
                                                _: uint32_t) -> ()>,
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
pub struct wlr_keyboard_modifiers {
    pub depressed: xkb_mod_mask_t,
    pub latched: xkb_mod_mask_t,
    pub locked: xkb_mod_mask_t,
    pub group: xkb_mod_mask_t,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_keyboard_group {
    pub keyboard: wlr_keyboard,
    pub input_device: *mut wlr_input_device,
    pub devices: wl_list,
    pub keys: wl_list,
    pub data: *mut libc::c_void,
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
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub events: C2RustUnnamed_1,
    pub data: *mut libc::c_void,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_2 {
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
    pub events: C2RustUnnamed_3,
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
pub struct C2RustUnnamed_3 {
    pub button: wl_signal,
    pub ring: wl_signal,
    pub strip: wl_signal,
    pub attach_tablet: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet {
    pub impl_0: *mut wlr_tablet_impl,
    pub events: C2RustUnnamed_4,
    pub name: *mut libc::c_char,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub axis: wl_signal,
    pub proximity: wl_signal,
    pub tip: wl_signal,
    pub button: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_touch {
    pub impl_0: *const wlr_touch_impl,
    pub events: C2RustUnnamed_5,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub events: C2RustUnnamed_6,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub toggle: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
    pub events: C2RustUnnamed_7,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
pub type wlr_input_device_type = libc::c_uint;
pub const WLR_INPUT_DEVICE_SWITCH: wlr_input_device_type = 5;
pub const WLR_INPUT_DEVICE_TABLET_PAD: wlr_input_device_type = 4;
pub const WLR_INPUT_DEVICE_TABLET_TOOL: wlr_input_device_type = 3;
pub const WLR_INPUT_DEVICE_TOUCH: wlr_input_device_type = 2;
pub const WLR_INPUT_DEVICE_POINTER: wlr_input_device_type = 1;
pub const WLR_INPUT_DEVICE_KEYBOARD: wlr_input_device_type = 0;
pub type wlr_key_state = libc::c_uint;
pub const WLR_KEY_PRESSED: wlr_key_state = 1;
pub const WLR_KEY_RELEASED: wlr_key_state = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_keyboard_key {
    pub time_msec: uint32_t,
    pub keycode: uint32_t,
    pub update_state: bool,
    pub state: wlr_key_state,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct keyboard_group_device {
    pub keyboard: *mut wlr_keyboard,
    pub key: wl_listener,
    pub modifiers: wl_listener,
    pub keymap: wl_listener,
    pub repeat_info: wl_listener,
    pub destroy: wl_listener,
    pub link: wl_list,
    // wlr_keyboard_group::devices
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
pub struct keyboard_group_key {
    pub keycode: uint32_t,
    pub count: size_t,
    pub link: wl_list,
    // wlr_keyboard_group::keys
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
unsafe extern "C" fn keyboard_set_leds(mut kb: *mut wlr_keyboard,
                                       mut leds: uint32_t) {
    let mut group: *mut wlr_keyboard_group =
        wlr_keyboard_group_from_wlr_keyboard(kb);
    let mut device: *mut keyboard_group_device =
        0 as *mut keyboard_group_device;
    device =
        ((*group).devices.next as *mut libc::c_char).offset(-128) as
            *mut keyboard_group_device;
    while &mut (*device).link as *mut wl_list !=
              &mut (*group).devices as *mut wl_list {
        wlr_keyboard_led_update((*device).keyboard, leds);
        device =
            ((*device).link.next as *mut libc::c_char).offset(-128) as
                *mut keyboard_group_device
    };
}
unsafe extern "C" fn keyboard_destroy(mut kb: *mut wlr_keyboard) {
    // Just remove the event listeners. The keyboard will be freed as part of
	// the wlr_keyboard_group in wlr_keyboard_group_destroy.
    wl_list_remove(&mut (*kb).events.key.listener_list);
    wl_list_remove(&mut (*kb).events.modifiers.listener_list);
    wl_list_remove(&mut (*kb).events.keymap.listener_list);
    wl_list_remove(&mut (*kb).events.repeat_info.listener_list);
    wl_list_remove(&mut (*kb).events.destroy.listener_list);
}
static mut impl_0: wlr_keyboard_impl =
    unsafe {
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
pub unsafe extern "C" fn wlr_keyboard_group_create()
 -> *mut wlr_keyboard_group {
    let mut group: *mut wlr_keyboard_group =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_keyboard_group>() as libc::c_ulong)
            as *mut wlr_keyboard_group;
    if group.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate wlr_keyboard_group\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_keyboard_group.c\x00" as *const u8 as
                     *const libc::c_char, 56i32);
        return 0 as *mut wlr_keyboard_group
    }
    (*group).input_device =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_input_device>() as libc::c_ulong) as
            *mut wlr_input_device;
    if (*group).input_device.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate wlr_input_device for group\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/wlr_keyboard_group.c\x00" as *const u8 as
                     *const libc::c_char, 62i32);
        free(group as *mut libc::c_void);
        return 0 as *mut wlr_keyboard_group
    }
    wl_signal_init(&mut (*(*group).input_device).events.destroy);
    (*(*group).input_device).c2rust_unnamed.keyboard = &mut (*group).keyboard;
    wlr_keyboard_init(&mut (*group).keyboard, &impl_0);
    wl_list_init(&mut (*group).devices);
    wl_list_init(&mut (*group).keys);
    return group;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_group_from_wlr_keyboard(mut keyboard:
                                                                  *mut wlr_keyboard)
 -> *mut wlr_keyboard_group {
    if (*keyboard).impl_0 != &impl_0 as *const wlr_keyboard_impl {
        return 0 as *mut wlr_keyboard_group
    }
    return keyboard as *mut wlr_keyboard_group;
}
unsafe extern "C" fn keymaps_match(mut km1: *mut xkb_keymap,
                                   mut km2: *mut xkb_keymap) -> bool {
    if km1.is_null() || km2.is_null() { return 0i32 != 0 }
    let mut km1_str: *mut libc::c_char =
        xkb_keymap_get_as_string(km1, XKB_KEYMAP_FORMAT_TEXT_V1);
    let mut km2_str: *mut libc::c_char =
        xkb_keymap_get_as_string(km2, XKB_KEYMAP_FORMAT_TEXT_V1);
    let mut result: bool = strcmp(km1_str, km2_str) == 0i32;
    free(km1_str as *mut libc::c_void);
    free(km2_str as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn handle_keyboard_key(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut group_device: *mut keyboard_group_device =
        (listener as *mut libc::c_char).offset(-8) as
            *mut keyboard_group_device;
    let mut group: *mut wlr_keyboard_group =
        (*(*group_device).keyboard).group;
    let mut event: *mut wlr_event_keyboard_key =
        data as *mut wlr_event_keyboard_key;
    let mut key: *mut keyboard_group_key = 0 as *mut keyboard_group_key;
    let mut tmp: *mut keyboard_group_key = 0 as *mut keyboard_group_key;
    key =
        ((*group).keys.next as *mut libc::c_char).offset(-16) as
            *mut keyboard_group_key;
    tmp =
        ((*key).link.next as *mut libc::c_char).offset(-16) as
            *mut keyboard_group_key;
    while &mut (*key).link as *mut wl_list !=
              &mut (*group).keys as *mut wl_list {
        if (*key).keycode != (*event).keycode {
            key = tmp;
            tmp =
                ((*key).link.next as *mut libc::c_char).offset(-16) as
                    *mut keyboard_group_key
        } else {
            if (*event).state as libc::c_uint ==
                   WLR_KEY_PRESSED as libc::c_int as libc::c_uint {
                (*key).count = (*key).count.wrapping_add(1);
                return
            }
            if (*event).state as libc::c_uint ==
                   WLR_KEY_RELEASED as libc::c_int as libc::c_uint {
                (*key).count = (*key).count.wrapping_sub(1);
                if (*key).count > 0i32 as libc::c_ulong { return }
                wl_list_remove(&mut (*key).link);
                free(key as *mut libc::c_void);
            }
            break ;
        }
    }
    if (*event).state as libc::c_uint ==
           WLR_KEY_PRESSED as libc::c_int as libc::c_uint {
        let mut key_0: *mut keyboard_group_key =
            calloc(1i32 as libc::c_ulong,
                   ::std::mem::size_of::<keyboard_group_key>() as
                       libc::c_ulong) as *mut keyboard_group_key;
        if key_0.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to allocate keyboard_group_key\x00" as
                         *const u8 as *const libc::c_char,
                     b"../types/wlr_keyboard_group.c\x00" as *const u8 as
                         *const libc::c_char, 126i32);
            return
        }
        (*key_0).keycode = (*event).keycode;
        (*key_0).count = 1i32 as size_t;
        wl_list_insert(&mut (*group).keys, &mut (*key_0).link);
    }
    wlr_keyboard_notify_key(&mut (*(*(*group_device).keyboard).group).keyboard,
                            data as *mut wlr_event_keyboard_key);
}
unsafe extern "C" fn handle_keyboard_modifiers(mut listener: *mut wl_listener,
                                               mut data: *mut libc::c_void) {
    // Sync the effective layout (group modifier) to all keyboards. The rest of
	// the modifiers will be derived from the wlr_keyboard_group's key state
    let mut group_device: *mut keyboard_group_device =
        (listener as *mut libc::c_char).offset(-32) as
            *mut keyboard_group_device;
    let mut mods: wlr_keyboard_modifiers =
        (*(*group_device).keyboard).modifiers;
    let mut device: *mut keyboard_group_device =
        0 as *mut keyboard_group_device;
    device =
        ((*(*(*group_device).keyboard).group).devices.next as
             *mut libc::c_char).offset(-128) as *mut keyboard_group_device;
    while &mut (*device).link as *mut wl_list !=
              &mut (*(*(*group_device).keyboard).group).devices as
                  *mut wl_list {
        if mods.depressed != (*(*device).keyboard).modifiers.depressed ||
               mods.latched != (*(*device).keyboard).modifiers.latched ||
               mods.locked != (*(*device).keyboard).modifiers.locked ||
               mods.group != (*(*device).keyboard).modifiers.group {
            wlr_keyboard_notify_modifiers((*device).keyboard, mods.depressed,
                                          mods.latched, mods.locked,
                                          mods.group);
            return
        }
        device =
            ((*device).link.next as *mut libc::c_char).offset(-128) as
                *mut keyboard_group_device
    }
    wlr_keyboard_notify_modifiers(&mut (*(*(*group_device).keyboard).group).keyboard,
                                  mods.depressed, mods.latched, mods.locked,
                                  mods.group);
}
unsafe extern "C" fn handle_keyboard_keymap(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut group_device: *mut keyboard_group_device =
        (listener as *mut libc::c_char).offset(-56) as
            *mut keyboard_group_device;
    let mut keyboard: *mut wlr_keyboard = (*group_device).keyboard;
    if !keymaps_match((*(*keyboard).group).keyboard.keymap,
                      (*keyboard).keymap) {
        let mut device: *mut keyboard_group_device =
            0 as *mut keyboard_group_device;
        device =
            ((*(*keyboard).group).devices.next as
                 *mut libc::c_char).offset(-128) as
                *mut keyboard_group_device;
        while &mut (*device).link as *mut wl_list !=
                  &mut (*(*keyboard).group).devices as *mut wl_list {
            if !keymaps_match((*keyboard).keymap,
                              (*(*device).keyboard).keymap) {
                wlr_keyboard_set_keymap((*device).keyboard,
                                        (*keyboard).keymap);
                return
            }
            device =
                ((*device).link.next as *mut libc::c_char).offset(-128) as
                    *mut keyboard_group_device
        }
    }
    wlr_keyboard_set_keymap(&mut (*(*keyboard).group).keyboard,
                            (*keyboard).keymap);
}
unsafe extern "C" fn handle_keyboard_repeat_info(mut listener:
                                                     *mut wl_listener,
                                                 mut data:
                                                     *mut libc::c_void) {
    let mut group_device: *mut keyboard_group_device =
        (listener as *mut libc::c_char).offset(-80) as
            *mut keyboard_group_device;
    let mut keyboard: *mut wlr_keyboard = (*group_device).keyboard;
    let mut device: *mut keyboard_group_device =
        0 as *mut keyboard_group_device;
    device =
        ((*(*keyboard).group).devices.next as *mut libc::c_char).offset(-128)
            as *mut keyboard_group_device;
    while &mut (*device).link as *mut wl_list !=
              &mut (*(*keyboard).group).devices as *mut wl_list {
        let mut devkb: *mut wlr_keyboard = (*device).keyboard;
        if (*devkb).repeat_info.rate != (*keyboard).repeat_info.rate ||
               (*devkb).repeat_info.delay != (*keyboard).repeat_info.delay {
            wlr_keyboard_set_repeat_info(devkb, (*keyboard).repeat_info.rate,
                                         (*keyboard).repeat_info.delay);
            return
        }
        device =
            ((*device).link.next as *mut libc::c_char).offset(-128) as
                *mut keyboard_group_device
    }
    wlr_keyboard_set_repeat_info(&mut (*(*keyboard).group).keyboard,
                                 (*keyboard).repeat_info.rate,
                                 (*keyboard).repeat_info.delay);
}
unsafe extern "C" fn refresh_state(mut device: *mut keyboard_group_device,
                                   mut state: wlr_key_state) {
    let mut i: size_t = 0i32 as size_t;
    while i < (*(*device).keyboard).num_keycodes {
        let mut event: *mut wlr_event_keyboard_key =
            calloc(1i32 as libc::c_ulong,
                   ::std::mem::size_of::<wlr_event_keyboard_key>() as
                       libc::c_ulong) as *mut wlr_event_keyboard_key;
        if event.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to allocate wlr_event_keyboard_key\x00"
                         as *const u8 as *const libc::c_char,
                     b"../types/wlr_keyboard_group.c\x00" as *const u8 as
                         *const libc::c_char, 206i32);
            // TODO: Handle corrupt state somehow
        } else {
            let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
            clock_gettime(1i32, &mut now);
            (*event).time_msec =
                (now.tv_sec * 1000i32 as libc::c_long +
                     now.tv_nsec / 1000000i32 as libc::c_long) as uint32_t;
            (*event).keycode = (*(*device).keyboard).keycodes[i as usize];
            (*event).update_state = 1i32 != 0;
            (*event).state = state;
            handle_keyboard_key(&mut (*device).key,
                                event as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn remove_keyboard_group_device(mut device:
                                                      *mut keyboard_group_device) {
    refresh_state(device, WLR_KEY_RELEASED);
    (*(*device).keyboard).group = 0 as *mut wlr_keyboard_group;
    wl_list_remove(&mut (*device).link);
    wl_list_remove(&mut (*device).key.link);
    wl_list_remove(&mut (*device).modifiers.link);
    wl_list_remove(&mut (*device).keymap.link);
    wl_list_remove(&mut (*device).repeat_info.link);
    wl_list_remove(&mut (*device).destroy.link);
    free(device as *mut libc::c_void);
}
unsafe extern "C" fn handle_keyboard_destroy(mut listener: *mut wl_listener,
                                             mut data: *mut libc::c_void) {
    let mut device: *mut keyboard_group_device =
        (listener as *mut libc::c_char).offset(-104) as
            *mut keyboard_group_device;
    remove_keyboard_group_device(device);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_group_add_keyboard(mut group:
                                                             *mut wlr_keyboard_group,
                                                         mut keyboard:
                                                             *mut wlr_keyboard)
 -> bool {
    if !(*keyboard).group.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] A wlr_keyboard can only belong to one group\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_keyboard_group.c\x00" as *const u8 as
                     *const libc::c_char, 240i32);
        return 0i32 != 0
    }
    if (*keyboard).impl_0 == &impl_0 as *const wlr_keyboard_impl {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot add a group\'s keyboard to a group\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_keyboard_group.c\x00" as *const u8 as
                     *const libc::c_char, 245i32);
        return 0i32 != 0
    }
    if !keymaps_match((*group).keyboard.keymap, (*keyboard).keymap) {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Device keymap does not match keyboard group\'s\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/wlr_keyboard_group.c\x00" as *const u8 as
                     *const libc::c_char, 250i32);
        return 0i32 != 0
    }
    let mut device: *mut keyboard_group_device =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<keyboard_group_device>() as
                   libc::c_ulong) as *mut keyboard_group_device;
    if device.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate keyboard_group_device\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_keyboard_group.c\x00" as *const u8 as
                     *const libc::c_char, 257i32);
        return 0i32 != 0
    }
    (*device).keyboard = keyboard;
    (*keyboard).group = group;
    wl_list_insert(&mut (*group).devices, &mut (*device).link);
    wl_signal_add(&mut (*keyboard).events.key, &mut (*device).key);
    (*device).key.notify =
        Some(handle_keyboard_key as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*keyboard).events.modifiers,
                  &mut (*device).modifiers);
    (*device).modifiers.notify =
        Some(handle_keyboard_modifiers as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*keyboard).events.keymap, &mut (*device).keymap);
    (*device).keymap.notify =
        Some(handle_keyboard_keymap as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*keyboard).events.repeat_info,
                  &mut (*device).repeat_info);
    (*device).repeat_info.notify =
        Some(handle_keyboard_repeat_info as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*keyboard).events.destroy, &mut (*device).destroy);
    (*device).destroy.notify =
        Some(handle_keyboard_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    let mut group_kb: *mut wlr_keyboard = &mut (*group).keyboard;
    if (*keyboard).modifiers.group != (*group_kb).modifiers.group {
        wlr_keyboard_notify_modifiers(keyboard,
                                      (*keyboard).modifiers.depressed,
                                      (*keyboard).modifiers.latched,
                                      (*keyboard).modifiers.locked,
                                      (*group_kb).modifiers.group);
    }
    if (*keyboard).repeat_info.rate != (*group_kb).repeat_info.rate ||
           (*keyboard).repeat_info.delay != (*group_kb).repeat_info.delay {
        wlr_keyboard_set_repeat_info(keyboard, (*group_kb).repeat_info.rate,
                                     (*group_kb).repeat_info.delay);
    }
    refresh_state(device, WLR_KEY_PRESSED);
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_group_remove_keyboard(mut group:
                                                                *mut wlr_keyboard_group,
                                                            mut keyboard:
                                                                *mut wlr_keyboard) {
    let mut device: *mut keyboard_group_device =
        0 as *mut keyboard_group_device;
    let mut tmp: *mut keyboard_group_device = 0 as *mut keyboard_group_device;
    device =
        ((*group).devices.next as *mut libc::c_char).offset(-128) as
            *mut keyboard_group_device;
    tmp =
        ((*device).link.next as *mut libc::c_char).offset(-128) as
            *mut keyboard_group_device;
    while &mut (*device).link as *mut wl_list !=
              &mut (*group).devices as *mut wl_list {
        if (*device).keyboard == keyboard {
            remove_keyboard_group_device(device);
            return
        }
        device = tmp;
        tmp =
            ((*device).link.next as *mut libc::c_char).offset(-128) as
                *mut keyboard_group_device
    }
    _wlr_log(WLR_ERROR,
             b"[%s:%d] keyboard not found in group\x00" as *const u8 as
                 *const libc::c_char,
             b"../types/wlr_keyboard_group.c\x00" as *const u8 as
                 *const libc::c_char, 305i32);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_group_destroy(mut group:
                                                        *mut wlr_keyboard_group) {
    let mut device: *mut keyboard_group_device =
        0 as *mut keyboard_group_device;
    let mut tmp: *mut keyboard_group_device = 0 as *mut keyboard_group_device;
    device =
        ((*group).devices.next as *mut libc::c_char).offset(-128) as
            *mut keyboard_group_device;
    tmp =
        ((*device).link.next as *mut libc::c_char).offset(-128) as
            *mut keyboard_group_device;
    while &mut (*device).link as *mut wl_list !=
              &mut (*group).devices as *mut wl_list {
        wlr_keyboard_group_remove_keyboard(group, (*device).keyboard);
        device = tmp;
        tmp =
            ((*device).link.next as *mut libc::c_char).offset(-128) as
                *mut keyboard_group_device
    }
    wlr_keyboard_destroy(&mut (*group).keyboard);
    wl_list_remove(&mut (*(*group).input_device).events.destroy.listener_list);
    free((*group).input_device as *mut libc::c_void);
    free(group as *mut libc::c_void);
}
