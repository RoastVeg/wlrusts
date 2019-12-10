use libc;
extern "C" {
    pub type udev_device;
    pub type libinput_device;
    pub type libinput_event;
    pub type libinput_event_tablet_pad;
    pub type libinput_tablet_pad_mode_group;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_pointer_impl;
    pub type wlr_tablet_impl;
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn udev_device_get_syspath(udev_device: *mut udev_device)
     -> *const libc::c_char;
    #[no_mangle]
    fn libinput_tablet_pad_mode_group_get_num_modes(group:
                                                        *mut libinput_tablet_pad_mode_group)
     -> libc::c_uint;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn libinput_device_tablet_pad_get_num_mode_groups(device:
                                                          *mut libinput_device)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_device_tablet_pad_get_mode_group(device: *mut libinput_device,
                                                 index: libc::c_uint)
     -> *mut libinput_tablet_pad_mode_group;
    #[no_mangle]
    fn libinput_tablet_pad_mode_group_get_index(group:
                                                    *mut libinput_tablet_pad_mode_group)
     -> libc::c_uint;
    #[no_mangle]
    fn libinput_event_get_tablet_pad_event(event: *mut libinput_event)
     -> *mut libinput_event_tablet_pad;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_ring_position(event:
                                                       *mut libinput_event_tablet_pad)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_ring_number(event:
                                                     *mut libinput_event_tablet_pad)
     -> libc::c_uint;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_ring_source(event:
                                                     *mut libinput_event_tablet_pad)
     -> libinput_tablet_pad_ring_axis_source;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_strip_position(event:
                                                        *mut libinput_event_tablet_pad)
     -> libc::c_double;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_strip_number(event:
                                                      *mut libinput_event_tablet_pad)
     -> libc::c_uint;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_strip_source(event:
                                                      *mut libinput_event_tablet_pad)
     -> libinput_tablet_pad_strip_axis_source;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_button_number(event:
                                                       *mut libinput_event_tablet_pad)
     -> uint32_t;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_button_state(event:
                                                      *mut libinput_event_tablet_pad)
     -> libinput_button_state;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_mode(event:
                                              *mut libinput_event_tablet_pad)
     -> libc::c_uint;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_mode_group(event:
                                                    *mut libinput_event_tablet_pad)
     -> *mut libinput_tablet_pad_mode_group;
    #[no_mangle]
    fn libinput_event_tablet_pad_get_time_usec(event:
                                                   *mut libinput_event_tablet_pad)
     -> uint64_t;
    #[no_mangle]
    fn libinput_tablet_pad_mode_group_has_button(group:
                                                     *mut libinput_tablet_pad_mode_group,
                                                 button: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_tablet_pad_mode_group_has_ring(group:
                                                   *mut libinput_tablet_pad_mode_group,
                                               ring: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_tablet_pad_mode_group_has_strip(group:
                                                    *mut libinput_tablet_pad_mode_group,
                                                strip: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_device_get_udev_device(device: *mut libinput_device)
     -> *mut udev_device;
    #[no_mangle]
    fn libinput_device_tablet_pad_get_num_buttons(device:
                                                      *mut libinput_device)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_device_tablet_pad_get_num_rings(device: *mut libinput_device)
     -> libc::c_int;
    #[no_mangle]
    fn libinput_device_tablet_pad_get_num_strips(device: *mut libinput_device)
     -> libc::c_int;
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wlr_tablet_pad_init(pad: *mut wlr_tablet_pad,
                           impl_0: *mut wlr_tablet_pad_impl);
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
pub type libinput_tablet_pad_ring_axis_source = libc::c_uint;
pub const LIBINPUT_TABLET_PAD_RING_SOURCE_FINGER:
          libinput_tablet_pad_ring_axis_source =
    2;
pub const LIBINPUT_TABLET_PAD_RING_SOURCE_UNKNOWN:
          libinput_tablet_pad_ring_axis_source =
    1;
pub type libinput_tablet_pad_strip_axis_source = libc::c_uint;
pub const LIBINPUT_TABLET_PAD_STRIP_SOURCE_FINGER:
          libinput_tablet_pad_strip_axis_source =
    2;
pub const LIBINPUT_TABLET_PAD_STRIP_SOURCE_UNKNOWN:
          libinput_tablet_pad_strip_axis_source =
    1;
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
pub struct wlr_tablet_pad_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_tablet_pad) -> ()>,
}
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
/* Note: these are circular dependencies */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet_pad_group {
    pub link: wl_list,
    pub button_count: size_t,
    pub buttons: *mut libc::c_uint,
    pub strip_count: size_t,
    pub strips: *mut libc::c_uint,
    pub ring_count: size_t,
    pub rings: *mut libc::c_uint,
    pub mode_count: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_tablet_pad_button {
    pub time_msec: uint32_t,
    pub button: uint32_t,
    pub state: wlr_button_state,
    pub mode: libc::c_uint,
    pub group: libc::c_uint,
}
pub type wlr_tablet_pad_ring_source = libc::c_uint;
pub const WLR_TABLET_PAD_RING_SOURCE_FINGER: wlr_tablet_pad_ring_source = 1;
pub const WLR_TABLET_PAD_RING_SOURCE_UNKNOWN: wlr_tablet_pad_ring_source = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_tablet_pad_ring {
    pub time_msec: uint32_t,
    pub source: wlr_tablet_pad_ring_source,
    pub ring: uint32_t,
    pub position: libc::c_double,
    pub mode: libc::c_uint,
}
pub type wlr_tablet_pad_strip_source = libc::c_uint;
pub const WLR_TABLET_PAD_STRIP_SOURCE_FINGER: wlr_tablet_pad_strip_source = 1;
pub const WLR_TABLET_PAD_STRIP_SOURCE_UNKNOWN: wlr_tablet_pad_strip_source =
    0;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/*
 * NOTE: the wlr tablet pad implementation does not currently support tablets
 * with more than one mode. I don't own any such hardware so I cannot test it
 * and it is too complicated to make a meaningful implementation of blindly.
 */
//struct wlr_tablet_tool
// wlr_tablet_pad_group::link
// char *
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_tablet_pad_strip {
    pub time_msec: uint32_t,
    pub source: wlr_tablet_pad_strip_source,
    pub strip: uint32_t,
    pub position: libc::c_double,
    pub mode: libc::c_uint,
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
// FIXME: Decide on how to alloc/count here
unsafe extern "C" fn add_pad_group_from_libinput(mut pad: *mut wlr_tablet_pad,
                                                 mut device:
                                                     *mut libinput_device,
                                                 mut index: libc::c_uint) {
    let mut li_group: *mut libinput_tablet_pad_mode_group =
        libinput_device_tablet_pad_get_mode_group(device, index);
    let mut group: *mut wlr_tablet_pad_group =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_tablet_pad_group>() as libc::c_ulong)
            as *mut wlr_tablet_pad_group;
    if group.is_null() { return }
    let mut i: size_t = 0i32 as size_t;
    while i < (*pad).ring_count {
        if libinput_tablet_pad_mode_group_has_ring(li_group,
                                                   i as libc::c_uint) != 0 {
            (*group).ring_count = (*group).ring_count.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    (*group).rings =
        calloc(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
               (*group).ring_count) as *mut libc::c_uint;
    let mut ring: size_t = 0i32 as size_t;
    let mut i_0: size_t = 0i32 as size_t;
    while i_0 < (*pad).ring_count {
        if libinput_tablet_pad_mode_group_has_ring(li_group,
                                                   i_0 as libc::c_uint) != 0 {
            let fresh0 = ring;
            ring = ring.wrapping_add(1);
            *(*group).rings.offset(fresh0 as isize) = i_0 as libc::c_uint
        }
        i_0 = i_0.wrapping_add(1)
    }
    let mut i_1: size_t = 0i32 as size_t;
    while i_1 < (*pad).strip_count {
        if libinput_tablet_pad_mode_group_has_strip(li_group,
                                                    i_1 as libc::c_uint) != 0
           {
            (*group).strip_count = (*group).strip_count.wrapping_add(1)
        }
        i_1 = i_1.wrapping_add(1)
    }
    (*group).strips =
        calloc(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
               (*group).strip_count) as *mut libc::c_uint;
    let mut strip: size_t = 0i32 as size_t;
    let mut i_2: size_t = 0i32 as size_t;
    while i_2 < (*pad).strip_count {
        if libinput_tablet_pad_mode_group_has_strip(li_group,
                                                    i_2 as libc::c_uint) != 0
           {
            let fresh1 = strip;
            strip = strip.wrapping_add(1);
            *(*group).strips.offset(fresh1 as isize) = i_2 as libc::c_uint
        }
        i_2 = i_2.wrapping_add(1)
    }
    let mut i_3: size_t = 0i32 as size_t;
    while i_3 < (*pad).button_count {
        if libinput_tablet_pad_mode_group_has_button(li_group,
                                                     i_3 as libc::c_uint) != 0
           {
            (*group).button_count = (*group).button_count.wrapping_add(1)
        }
        i_3 = i_3.wrapping_add(1)
    }
    (*group).buttons =
        calloc(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
               (*group).button_count) as *mut libc::c_uint;
    let mut button: size_t = 0i32 as size_t;
    let mut i_4: size_t = 0i32 as size_t;
    while i_4 < (*pad).button_count {
        if libinput_tablet_pad_mode_group_has_button(li_group,
                                                     i_4 as libc::c_uint) != 0
           {
            let fresh2 = button;
            button = button.wrapping_add(1);
            *(*group).buttons.offset(fresh2 as isize) = i_4 as libc::c_uint
        }
        i_4 = i_4.wrapping_add(1)
    }
    (*group).mode_count =
        libinput_tablet_pad_mode_group_get_num_modes(li_group);
    wl_list_insert(&mut (*pad).groups, &mut (*group).link);
}
#[no_mangle]
pub unsafe extern "C" fn create_libinput_tablet_pad(mut libinput_dev:
                                                        *mut libinput_device)
 -> *mut wlr_tablet_pad {
    if !libinput_dev.is_null() {
    } else {
        __assert_fail(b"libinput_dev\x00" as *const u8 as *const libc::c_char,
                      b"../backend/libinput/tablet_pad.c\x00" as *const u8 as
                          *const libc::c_char, 71i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 76],
                                                &[libc::c_char; 76]>(b"struct wlr_tablet_pad *create_libinput_tablet_pad(struct libinput_device *)\x00")).as_ptr());
    };
    let mut wlr_tablet_pad: *mut wlr_tablet_pad =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_tablet_pad>() as libc::c_ulong) as
            *mut wlr_tablet_pad;
    if wlr_tablet_pad.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unable to allocate wlr_tablet_pad\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/libinput/tablet_pad.c\x00" as *const u8 as
                     *const libc::c_char, 75i32);
        return 0 as *mut wlr_tablet_pad
    }
    wlr_tablet_pad_init(wlr_tablet_pad, 0 as *mut wlr_tablet_pad_impl);
    (*wlr_tablet_pad).button_count =
        libinput_device_tablet_pad_get_num_buttons(libinput_dev) as size_t;
    (*wlr_tablet_pad).ring_count =
        libinput_device_tablet_pad_get_num_rings(libinput_dev) as size_t;
    (*wlr_tablet_pad).strip_count =
        libinput_device_tablet_pad_get_num_strips(libinput_dev) as size_t;
    let mut udev: *mut udev_device =
        libinput_device_get_udev_device(libinput_dev);
    wlr_list_push(&mut (*wlr_tablet_pad).paths,
                  strdup(udev_device_get_syspath(udev)) as *mut libc::c_void);
    let mut groups: libc::c_int =
        libinput_device_tablet_pad_get_num_mode_groups(libinput_dev);
    let mut i: libc::c_int = 0i32;
    while i < groups {
        add_pad_group_from_libinput(wlr_tablet_pad, libinput_dev,
                                    i as libc::c_uint);
        i += 1
    }
    return wlr_tablet_pad;
}
#[no_mangle]
pub unsafe extern "C" fn handle_tablet_pad_button(mut event:
                                                      *mut libinput_event,
                                                  mut libinput_dev:
                                                      *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TABLET_PAD, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a tablet pad event for a device with no tablet pad?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/tablet_pad.c\x00" as *const u8 as
                     *const libc::c_char, 104i32);
        return
    }
    let mut pevent: *mut libinput_event_tablet_pad =
        libinput_event_get_tablet_pad_event(event);
    let mut wlr_event: wlr_event_tablet_pad_button =
        {
            let mut init =
                wlr_event_tablet_pad_button{time_msec: 0i32 as uint32_t,
                                            button: 0,
                                            state: WLR_BUTTON_RELEASED,
                                            mode: 0,
                                            group: 0,};
            init
        };
    wlr_event.time_msec =
        usec_to_msec(libinput_event_tablet_pad_get_time_usec(pevent));
    wlr_event.button = libinput_event_tablet_pad_get_button_number(pevent);
    wlr_event.mode = libinput_event_tablet_pad_get_mode(pevent);
    wlr_event.group =
        libinput_tablet_pad_mode_group_get_index(libinput_event_tablet_pad_get_mode_group(pevent));
    match libinput_event_tablet_pad_get_button_state(pevent) as libc::c_uint {
        1 => { wlr_event.state = WLR_BUTTON_PRESSED }
        0 => { wlr_event.state = WLR_BUTTON_RELEASED }
        _ => { }
    }
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.tablet_pad).events.button,
                         &mut wlr_event as *mut wlr_event_tablet_pad_button as
                             *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_tablet_pad_ring(mut event:
                                                    *mut libinput_event,
                                                mut libinput_dev:
                                                    *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TABLET_PAD, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a tablet pad event for a device with no tablet pad?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/tablet_pad.c\x00" as *const u8 as
                     *const libc::c_char, 133i32);
        return
    }
    let mut pevent: *mut libinput_event_tablet_pad =
        libinput_event_get_tablet_pad_event(event);
    let mut wlr_event: wlr_event_tablet_pad_ring =
        {
            let mut init =
                wlr_event_tablet_pad_ring{time_msec: 0i32 as uint32_t,
                                          source:
                                              WLR_TABLET_PAD_RING_SOURCE_UNKNOWN,
                                          ring: 0,
                                          position: 0.,
                                          mode: 0,};
            init
        };
    wlr_event.time_msec =
        usec_to_msec(libinput_event_tablet_pad_get_time_usec(pevent));
    wlr_event.ring = libinput_event_tablet_pad_get_ring_number(pevent);
    wlr_event.position = libinput_event_tablet_pad_get_ring_position(pevent);
    wlr_event.mode = libinput_event_tablet_pad_get_mode(pevent);
    match libinput_event_tablet_pad_get_ring_source(pevent) as libc::c_uint {
        1 => { wlr_event.source = WLR_TABLET_PAD_RING_SOURCE_UNKNOWN }
        2 => { wlr_event.source = WLR_TABLET_PAD_RING_SOURCE_FINGER }
        _ => { }
    }
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.tablet_pad).events.ring,
                         &mut wlr_event as *mut wlr_event_tablet_pad_ring as
                             *mut libc::c_void);
}
// list of struct wl_list
#[no_mangle]
pub unsafe extern "C" fn handle_tablet_pad_strip(mut event:
                                                     *mut libinput_event,
                                                 mut libinput_dev:
                                                     *mut libinput_device) {
    let mut wlr_dev: *mut wlr_input_device =
        get_appropriate_device(WLR_INPUT_DEVICE_TABLET_PAD, libinput_dev);
    if wlr_dev.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Got a tablet pad event for a device with no tablet pad?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/libinput/tablet_pad.c\x00" as *const u8 as
                     *const libc::c_char, 161i32);
        return
    }
    let mut pevent: *mut libinput_event_tablet_pad =
        libinput_event_get_tablet_pad_event(event);
    let mut wlr_event: wlr_event_tablet_pad_strip =
        {
            let mut init =
                wlr_event_tablet_pad_strip{time_msec: 0i32 as uint32_t,
                                           source:
                                               WLR_TABLET_PAD_STRIP_SOURCE_UNKNOWN,
                                           strip: 0,
                                           position: 0.,
                                           mode: 0,};
            init
        };
    wlr_event.time_msec =
        usec_to_msec(libinput_event_tablet_pad_get_time_usec(pevent));
    wlr_event.strip = libinput_event_tablet_pad_get_strip_number(pevent);
    wlr_event.position = libinput_event_tablet_pad_get_strip_position(pevent);
    wlr_event.mode = libinput_event_tablet_pad_get_mode(pevent);
    match libinput_event_tablet_pad_get_strip_source(pevent) as libc::c_uint {
        1 => { wlr_event.source = WLR_TABLET_PAD_STRIP_SOURCE_UNKNOWN }
        2 => { wlr_event.source = WLR_TABLET_PAD_STRIP_SOURCE_FINGER }
        _ => { }
    }
    wlr_signal_emit_safe(&mut (*(*wlr_dev).c2rust_unnamed.tablet_pad).events.strip,
                         &mut wlr_event as *mut wlr_event_tablet_pad_strip as
                             *mut libc::c_void);
}
