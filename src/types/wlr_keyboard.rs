use libc;
extern "C" {
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_group;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    /* *
 * Add `target` to `values` if it doesn't exist
 * "set"s should only be modified with set_* functions
 * Values MUST be greater than 0
 */
    #[no_mangle]
    fn set_add(values: *mut uint32_t, len: *mut size_t, cap: size_t,
               target: uint32_t) -> bool;
    /* *
 * Remove `target` from `values` if it exists
 * "set"s should only be modified with set_* functions
 * Values MUST be greater than 0
 */
    #[no_mangle]
    fn set_remove(values: *mut uint32_t, len: *mut size_t, cap: size_t,
                  target: uint32_t) -> bool;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn xkb_state_unref(state: *mut xkb_state);
    #[no_mangle]
    fn xkb_state_update_key(state: *mut xkb_state, key: xkb_keycode_t,
                            direction: xkb_key_direction)
     -> xkb_state_component;
    #[no_mangle]
    fn xkb_state_update_mask(state: *mut xkb_state,
                             depressed_mods: xkb_mod_mask_t,
                             latched_mods: xkb_mod_mask_t,
                             locked_mods: xkb_mod_mask_t,
                             depressed_layout: xkb_layout_index_t,
                             latched_layout: xkb_layout_index_t,
                             locked_layout: xkb_layout_index_t)
     -> xkb_state_component;
    #[no_mangle]
    fn xkb_state_serialize_mods(state: *mut xkb_state,
                                components: xkb_state_component)
     -> xkb_mod_mask_t;
    #[no_mangle]
    fn xkb_state_serialize_layout(state: *mut xkb_state,
                                  components: xkb_state_component)
     -> xkb_layout_index_t;
    #[no_mangle]
    fn xkb_state_new(keymap: *mut xkb_keymap) -> *mut xkb_state;
    #[no_mangle]
    fn xkb_keymap_led_get_index(keymap: *mut xkb_keymap,
                                name: *const libc::c_char) -> xkb_led_index_t;
    #[no_mangle]
    fn xkb_keymap_ref(keymap: *mut xkb_keymap) -> *mut xkb_keymap;
    #[no_mangle]
    fn xkb_keymap_unref(keymap: *mut xkb_keymap);
    #[no_mangle]
    fn xkb_keymap_get_as_string(keymap: *mut xkb_keymap,
                                format: xkb_keymap_format)
     -> *mut libc::c_char;
    #[no_mangle]
    fn xkb_keymap_mod_get_index(keymap: *mut xkb_keymap,
                                name: *const libc::c_char) -> xkb_mod_index_t;
    #[no_mangle]
    fn xkb_state_led_index_is_active(state: *mut xkb_state,
                                     idx: xkb_led_index_t) -> libc::c_int;
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
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
pub type xkb_keycode_t = uint32_t;
pub type xkb_layout_index_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
pub type xkb_keymap_format = libc::c_uint;
pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
pub type xkb_key_direction = libc::c_uint;
pub const XKB_KEY_DOWN: xkb_key_direction = 1;
pub const XKB_KEY_UP: xkb_key_direction = 0;
pub type xkb_state_component = libc::c_uint;
pub const XKB_STATE_LEDS: xkb_state_component = 256;
pub const XKB_STATE_LAYOUT_EFFECTIVE: xkb_state_component = 128;
pub const XKB_STATE_LAYOUT_LOCKED: xkb_state_component = 64;
pub const XKB_STATE_LAYOUT_LATCHED: xkb_state_component = 32;
pub const XKB_STATE_LAYOUT_DEPRESSED: xkb_state_component = 16;
pub const XKB_STATE_MODS_EFFECTIVE: xkb_state_component = 8;
pub const XKB_STATE_MODS_LOCKED: xkb_state_component = 4;
pub const XKB_STATE_MODS_LATCHED: xkb_state_component = 2;
pub const XKB_STATE_MODS_DEPRESSED: xkb_state_component = 1;

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
unsafe extern "C" fn keyboard_led_update(mut keyboard: *mut wlr_keyboard) {
    if (*keyboard).xkb_state.is_null() { return }
    let mut leds: uint32_t = 0i32 as uint32_t;
    let mut i: uint32_t = 0i32 as uint32_t;
    while i < 3i32 as libc::c_uint {
        if xkb_state_led_index_is_active((*keyboard).xkb_state,
                                         (*keyboard).led_indexes[i as usize])
               != 0 {
            leds |= (1i32 << i) as libc::c_uint
        }
        i = i.wrapping_add(1)
    }
    wlr_keyboard_led_update(keyboard, leds);
}
/* *
 * Update the modifier state of the wlr-keyboard. Returns true if the modifier
 * state changed.
 */
unsafe extern "C" fn keyboard_modifier_update(mut keyboard: *mut wlr_keyboard)
 -> bool {
    if (*keyboard).xkb_state.is_null() { return 0i32 != 0 }
    let mut depressed: xkb_mod_mask_t =
        xkb_state_serialize_mods((*keyboard).xkb_state,
                                 XKB_STATE_MODS_DEPRESSED);
    let mut latched: xkb_mod_mask_t =
        xkb_state_serialize_mods((*keyboard).xkb_state,
                                 XKB_STATE_MODS_LATCHED);
    let mut locked: xkb_mod_mask_t =
        xkb_state_serialize_mods((*keyboard).xkb_state,
                                 XKB_STATE_MODS_LOCKED);
    let mut group: xkb_mod_mask_t =
        xkb_state_serialize_layout((*keyboard).xkb_state,
                                   XKB_STATE_LAYOUT_EFFECTIVE);
    if depressed == (*keyboard).modifiers.depressed &&
           latched == (*keyboard).modifiers.latched &&
           locked == (*keyboard).modifiers.locked &&
           group == (*keyboard).modifiers.group {
        return 0i32 != 0
    }
    (*keyboard).modifiers.depressed = depressed;
    (*keyboard).modifiers.latched = latched;
    (*keyboard).modifiers.locked = locked;
    (*keyboard).modifiers.group = group;
    return 1i32 != 0;
}
unsafe extern "C" fn keyboard_key_update(mut keyboard: *mut wlr_keyboard,
                                         mut event:
                                             *mut wlr_event_keyboard_key) {
    if (*event).state as libc::c_uint ==
           WLR_KEY_PRESSED as libc::c_int as libc::c_uint {
        set_add((*keyboard).keycodes.as_mut_ptr(),
                &mut (*keyboard).num_keycodes, 32i32 as size_t,
                (*event).keycode);
    }
    if (*event).state as libc::c_uint ==
           WLR_KEY_RELEASED as libc::c_int as libc::c_uint {
        set_remove((*keyboard).keycodes.as_mut_ptr(),
                   &mut (*keyboard).num_keycodes, 32i32 as size_t,
                   (*event).keycode);
    }
    if (*keyboard).num_keycodes <= 32i32 as libc::c_ulong {
    } else {
        __assert_fail(b"keyboard->num_keycodes <= WLR_KEYBOARD_KEYS_CAP\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_keyboard.c\x00" as *const u8 as
                          *const libc::c_char, 69i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 81],
                                                &[libc::c_char; 81]>(b"void keyboard_key_update(struct wlr_keyboard *, struct wlr_event_keyboard_key *)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_notify_modifiers(mut keyboard:
                                                           *mut wlr_keyboard,
                                                       mut mods_depressed:
                                                           uint32_t,
                                                       mut mods_latched:
                                                           uint32_t,
                                                       mut mods_locked:
                                                           uint32_t,
                                                       mut group: uint32_t) {
    if (*keyboard).xkb_state.is_null() { return }
    xkb_state_update_mask((*keyboard).xkb_state, mods_depressed, mods_latched,
                          mods_locked, 0i32 as xkb_layout_index_t,
                          0i32 as xkb_layout_index_t, group);
    let mut updated: bool = keyboard_modifier_update(keyboard);
    if updated {
        wlr_signal_emit_safe(&mut (*keyboard).events.modifiers,
                             keyboard as *mut libc::c_void);
    }
    keyboard_led_update(keyboard);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_notify_key(mut keyboard:
                                                     *mut wlr_keyboard,
                                                 mut event:
                                                     *mut wlr_event_keyboard_key) {
    keyboard_key_update(keyboard, event);
    wlr_signal_emit_safe(&mut (*keyboard).events.key,
                         event as *mut libc::c_void);
    if (*keyboard).xkb_state.is_null() { return }
    if (*event).update_state {
        let mut keycode: uint32_t =
            (*event).keycode.wrapping_add(8i32 as libc::c_uint);
        xkb_state_update_key((*keyboard).xkb_state, keycode,
                             if (*event).state as libc::c_uint ==
                                    WLR_KEY_PRESSED as libc::c_int as
                                        libc::c_uint {
                                 XKB_KEY_DOWN as libc::c_int
                             } else { XKB_KEY_UP as libc::c_int } as
                                 xkb_key_direction);
    }
    let mut updated: bool = keyboard_modifier_update(keyboard);
    if updated {
        wlr_signal_emit_safe(&mut (*keyboard).events.modifiers,
                             keyboard as *mut libc::c_void);
    }
    keyboard_led_update(keyboard);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_init(mut kb: *mut wlr_keyboard,
                                           mut impl_0:
                                               *const wlr_keyboard_impl) {
    (*kb).impl_0 = impl_0;
    wl_signal_init(&mut (*kb).events.key);
    wl_signal_init(&mut (*kb).events.modifiers);
    wl_signal_init(&mut (*kb).events.keymap);
    wl_signal_init(&mut (*kb).events.repeat_info);
    wl_signal_init(&mut (*kb).events.destroy);
    // Sane defaults
    (*kb).repeat_info.rate = 25i32;
    (*kb).repeat_info.delay = 600i32;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_destroy(mut kb: *mut wlr_keyboard) {
    if kb.is_null() { return }
    wlr_signal_emit_safe(&mut (*kb).events.destroy, kb as *mut libc::c_void);
    xkb_state_unref((*kb).xkb_state);
    xkb_keymap_unref((*kb).keymap);
    free((*kb).keymap_string as *mut libc::c_void);
    if !(*kb).impl_0.is_null() && (*(*kb).impl_0).destroy.is_some() {
        (*(*kb).impl_0).destroy.expect("non-null function pointer")(kb);
    } else {
        wl_list_remove(&mut (*kb).events.key.listener_list);
        free(kb as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_led_update(mut kb: *mut wlr_keyboard,
                                                 mut leds: uint32_t) {
    if !(*kb).impl_0.is_null() && (*(*kb).impl_0).led_update.is_some() {
        (*(*kb).impl_0).led_update.expect("non-null function pointer")(kb,
                                                                       leds);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_set_keymap(mut kb: *mut wlr_keyboard,
                                                 mut keymap:
                                                     *mut xkb_keymap) {
    let mut led_names: [*const libc::c_char; 3] =
        [0 as *const libc::c_char; 3];
    let mut mod_names: [*const libc::c_char; 8] =
        [0 as *const libc::c_char; 8];
    let mut tmp_keymap_string: *mut libc::c_char = 0 as *mut libc::c_char;
    xkb_keymap_unref((*kb).keymap);
    (*kb).keymap = xkb_keymap_ref(keymap);
    xkb_state_unref((*kb).xkb_state);
    (*kb).xkb_state = xkb_state_new((*kb).keymap);
    if (*kb).xkb_state.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create XKB state\x00" as *const u8 as
                     *const libc::c_char,
                 b"../types/wlr_keyboard.c\x00" as *const u8 as
                     *const libc::c_char, 156i32);
    } else {
        led_names =
            [b"Num Lock\x00" as *const u8 as *const libc::c_char,
             b"Caps Lock\x00" as *const u8 as *const libc::c_char,
             b"Scroll Lock\x00" as *const u8 as *const libc::c_char];
        let mut i: size_t = 0i32 as size_t;
        while i < 3i32 as libc::c_ulong {
            (*kb).led_indexes[i as usize] =
                xkb_keymap_led_get_index((*kb).keymap, led_names[i as usize]);
            i = i.wrapping_add(1)
        }
        mod_names =
            [b"Shift\x00" as *const u8 as *const libc::c_char,
             b"Lock\x00" as *const u8 as *const libc::c_char,
             b"Control\x00" as *const u8 as *const libc::c_char,
             b"Mod1\x00" as *const u8 as *const libc::c_char,
             b"Mod2\x00" as *const u8 as *const libc::c_char,
             b"Mod3\x00" as *const u8 as *const libc::c_char,
             b"Mod4\x00" as *const u8 as *const libc::c_char,
             b"Mod5\x00" as *const u8 as *const libc::c_char];
        // TODO: there's also "Ctrl", "Alt"?
        let mut i_0: size_t = 0i32 as size_t;
        while i_0 < 8i32 as libc::c_ulong {
            (*kb).mod_indexes[i_0 as usize] =
                xkb_keymap_mod_get_index((*kb).keymap,
                                         mod_names[i_0 as usize]);
            i_0 = i_0.wrapping_add(1)
        }
        tmp_keymap_string =
            xkb_keymap_get_as_string((*kb).keymap, XKB_KEYMAP_FORMAT_TEXT_V1);
        if tmp_keymap_string.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to get string version of keymap\x00" as
                         *const u8 as *const libc::c_char,
                     b"../types/wlr_keyboard.c\x00" as *const u8 as
                         *const libc::c_char, 187i32);
        } else {
            free((*kb).keymap_string as *mut libc::c_void);
            (*kb).keymap_string = tmp_keymap_string;
            (*kb).keymap_size =
                strlen((*kb).keymap_string).wrapping_add(1i32 as
                                                             libc::c_ulong);
            let mut i_1: size_t = 0i32 as size_t;
            while i_1 < (*kb).num_keycodes {
                let mut keycode: xkb_keycode_t =
                    (*kb).keycodes[i_1 as
                                       usize].wrapping_add(8i32 as
                                                               libc::c_uint);
                xkb_state_update_key((*kb).xkb_state, keycode, XKB_KEY_DOWN);
                i_1 = i_1.wrapping_add(1)
            }
            keyboard_modifier_update(kb);
            wlr_signal_emit_safe(&mut (*kb).events.keymap,
                                 kb as *mut libc::c_void);
            return
        }
    }
    xkb_state_unref((*kb).xkb_state);
    (*kb).xkb_state = 0 as *mut xkb_state;
    xkb_keymap_unref(keymap);
    (*kb).keymap = 0 as *mut xkb_keymap;
    free((*kb).keymap_string as *mut libc::c_void);
    (*kb).keymap_string = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_set_repeat_info(mut kb:
                                                          *mut wlr_keyboard,
                                                      mut rate: int32_t,
                                                      mut delay: int32_t) {
    if (*kb).repeat_info.rate == rate && (*kb).repeat_info.delay == delay {
        return
    }
    (*kb).repeat_info.rate = rate;
    (*kb).repeat_info.delay = delay;
    wlr_signal_emit_safe(&mut (*kb).events.repeat_info,
                         kb as *mut libc::c_void);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
		 * The `key` event signals with a `wlr_event_keyboard_key` event that a
		 * key has been pressed or released on the keyboard. This event is
		 * emitted before the xkb state of the keyboard has been updated
		 * (including modifiers).
		 */
/* *
		 * The `modifiers` event signals that the modifier state of the
		 * `wlr_keyboard` has been updated. At this time, you can read the
		 * modifier state of the `wlr_keyboard` and handle the updated state by
		 * sending it to clients.
		 */
// if backend doesn't update modifiers on its own
/* *
 * Sets the keyboard repeat info. `rate` is in key repeats/second and delay is
 * in milliseconds.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_keyboard_get_modifiers(mut kb: *mut wlr_keyboard)
 -> uint32_t {
    let mut mask: xkb_mod_mask_t =
        (*kb).modifiers.depressed | (*kb).modifiers.latched;
    let mut modifiers: uint32_t = 0i32 as uint32_t;
    let mut i: size_t = 0i32 as size_t;
    while i < 8i32 as libc::c_ulong {
        if (*kb).mod_indexes[i as usize] != 0xffffffffu32 &&
               mask & (1i32 << (*kb).mod_indexes[i as usize]) as libc::c_uint
                   != 0 {
            modifiers |= (1i32 << i) as libc::c_uint
        }
        i = i.wrapping_add(1)
    }
    return modifiers;
}
