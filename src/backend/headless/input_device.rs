use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_group;
    pub type udev;
    pub type udev_monitor;
    pub type session_impl;
    pub type wlr_renderer;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wlr_input_device_init(wlr_device: *mut wlr_input_device,
                             type_0: wlr_input_device_type,
                             impl_0: *const wlr_input_device_impl,
                             name: *const libc::c_char, vendor: libc::c_int,
                             product: libc::c_int);
    #[no_mangle]
    fn wlr_keyboard_init(keyboard: *mut wlr_keyboard,
                         impl_0: *const wlr_keyboard_impl);
    #[no_mangle]
    fn wlr_pointer_init(pointer: *mut wlr_pointer,
                        impl_0: *const wlr_pointer_impl);
    #[no_mangle]
    fn wlr_tablet_pad_init(pad: *mut wlr_tablet_pad,
                           impl_0: *mut wlr_tablet_pad_impl);
    #[no_mangle]
    fn wlr_tablet_init(tablet: *mut wlr_tablet, impl_0: *mut wlr_tablet_impl);
    #[no_mangle]
    fn wlr_touch_init(touch: *mut wlr_touch, impl_0: *const wlr_touch_impl);
    #[no_mangle]
    fn wlr_switch_init(switch_device: *mut wlr_switch,
                       impl_0: *mut wlr_switch_impl);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn headless_backend_from_backend(wlr_backend: *mut wlr_backend)
     -> *mut wlr_headless_backend;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
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
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __clockid_t = libc::c_int;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type clockid_t = __clockid_t;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_pointer) -> ()>,
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
/* Note: these are circular dependencies */
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
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
pub struct wlr_touch_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_touch) -> ()>,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_switch_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_switch) -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_session {
    pub impl_0: *const session_impl,
    pub session_signal: wl_signal,
    pub active: bool,
    pub vtnr: libc::c_uint,
    pub seat: [libc::c_char; 256],
    pub udev: *mut udev,
    pub mon: *mut udev_monitor,
    pub udev_event: *mut wl_event_source,
    pub devices: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_8,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub destroy: wl_signal,
}
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_drm_format {
    pub format: uint32_t,
    pub len: size_t,
    pub cap: size_t,
    pub modifiers: [uint64_t; 0],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub cap: size_t,
    pub formats: *mut *mut wlr_drm_format,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_egl {
    pub platform: EGLenum,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
    pub exts_str: *const libc::c_char,
    pub exts: C2RustUnnamed_9,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub bind_wayland_display_wl: bool,
    pub buffer_age_ext: bool,
    pub image_base_khr: bool,
    pub image_dma_buf_export_mesa: bool,
    pub image_dmabuf_import_ext: bool,
    pub image_dmabuf_import_modifiers_ext: bool,
    pub swap_buffers_with_damage_ext: bool,
    pub swap_buffers_with_damage_khr: bool,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_backend_impl {
    pub start: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> ()>,
    pub get_renderer: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                 -> *mut wlr_renderer>,
    pub get_session: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                -> *mut wlr_session>,
    pub get_presentation_clock: Option<unsafe extern "C" fn(_:
                                                                *mut wlr_backend)
                                           -> clockid_t>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed_10,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_headless_input_device {
    pub wlr_input_device: wlr_input_device,
    pub backend: *mut wlr_headless_backend,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_headless_backend {
    pub backend: wlr_backend,
    pub egl: wlr_egl,
    pub renderer: *mut wlr_renderer,
    pub display: *mut wl_display,
    pub outputs: wl_list,
    pub last_output_num: size_t,
    pub input_devices: wl_list,
    pub display_destroy: wl_listener,
    pub started: bool,
}
static mut input_device_impl: wlr_input_device_impl =
    { let mut init = wlr_input_device_impl{destroy: None,}; init };
#[no_mangle]
pub unsafe extern "C" fn wlr_input_device_is_headless(mut wlr_dev:
                                                          *mut wlr_input_device)
 -> bool {
    return (*wlr_dev).impl_0 ==
               &input_device_impl as *const wlr_input_device_impl;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * Creates a headless backend. A headless backend has no outputs or inputs by
 * default.
 */
/* *
 * Create a new headless output backed by an in-memory EGL framebuffer. You can
 * read pixels from this framebuffer via wlr_renderer_read_pixels but it is
 * otherwise not displayed.
 */
/* *
 * Creates a new input device. The caller is responsible for manually raising
 * any event signals on the new input device if it wants to simulate input
 * events.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_headless_add_input_device(mut wlr_backend:
                                                           *mut wlr_backend,
                                                       mut type_0:
                                                           wlr_input_device_type)
 -> *mut wlr_input_device {
    let mut current_block: u64;
    let mut backend: *mut wlr_headless_backend =
        headless_backend_from_backend(wlr_backend);
    let mut device: *mut wlr_headless_input_device =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_headless_input_device>() as
                   libc::c_ulong) as *mut wlr_headless_input_device;
    if device.is_null() { return 0 as *mut wlr_input_device }
    (*device).backend = backend;
    let mut vendor: libc::c_int = 0i32;
    let mut product: libc::c_int = 0i32;
    let mut name: *const libc::c_char =
        b"headless\x00" as *const u8 as *const libc::c_char;
    let mut wlr_device: *mut wlr_input_device =
        &mut (*device).wlr_input_device;
    wlr_input_device_init(wlr_device, type_0, &input_device_impl, name,
                          vendor, product);
    match type_0 as libc::c_uint {
        0 => {
            (*wlr_device).c2rust_unnamed.keyboard =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<wlr_keyboard>() as libc::c_ulong)
                    as *mut wlr_keyboard;
            if (*wlr_device).c2rust_unnamed.keyboard.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Unable to allocate wlr_keyboard\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/headless/input_device.c\x00" as
                             *const u8 as *const libc::c_char, 43i32);
                current_block = 18267909660717707325;
            } else {
                wlr_keyboard_init((*wlr_device).c2rust_unnamed.keyboard,
                                  0 as *const wlr_keyboard_impl);
                current_block = 3123434771885419771;
            }
        }
        1 => {
            (*wlr_device).c2rust_unnamed.pointer =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<wlr_pointer>() as libc::c_ulong)
                    as *mut wlr_pointer;
            if (*wlr_device).c2rust_unnamed.pointer.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Unable to allocate wlr_pointer\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/headless/input_device.c\x00" as
                             *const u8 as *const libc::c_char, 51i32);
                current_block = 18267909660717707325;
            } else {
                wlr_pointer_init((*wlr_device).c2rust_unnamed.pointer,
                                 0 as *const wlr_pointer_impl);
                current_block = 3123434771885419771;
            }
        }
        2 => {
            (*wlr_device).c2rust_unnamed.touch =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<wlr_touch>() as libc::c_ulong) as
                    *mut wlr_touch;
            if (*wlr_device).c2rust_unnamed.touch.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Unable to allocate wlr_touch\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/headless/input_device.c\x00" as
                             *const u8 as *const libc::c_char, 59i32);
                current_block = 18267909660717707325;
            } else {
                wlr_touch_init((*wlr_device).c2rust_unnamed.touch,
                               0 as *const wlr_touch_impl);
                current_block = 3123434771885419771;
            }
        }
        3 => {
            (*wlr_device).c2rust_unnamed.tablet =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<wlr_tablet>() as libc::c_ulong)
                    as *mut wlr_tablet;
            if (*wlr_device).c2rust_unnamed.tablet.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Unable to allocate wlr_tablet\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/headless/input_device.c\x00" as
                             *const u8 as *const libc::c_char, 67i32);
                current_block = 18267909660717707325;
            } else {
                wlr_tablet_init((*wlr_device).c2rust_unnamed.tablet,
                                0 as *mut wlr_tablet_impl);
                current_block = 3123434771885419771;
            }
        }
        4 => {
            (*wlr_device).c2rust_unnamed.tablet_pad =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<wlr_tablet_pad>() as
                           libc::c_ulong) as *mut wlr_tablet_pad;
            if (*wlr_device).c2rust_unnamed.tablet_pad.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Unable to allocate wlr_tablet_pad\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/headless/input_device.c\x00" as
                             *const u8 as *const libc::c_char, 75i32);
                current_block = 18267909660717707325;
            } else {
                wlr_tablet_pad_init((*wlr_device).c2rust_unnamed.tablet_pad,
                                    0 as *mut wlr_tablet_pad_impl);
                current_block = 3123434771885419771;
            }
        }
        5 => {
            (*wlr_device).c2rust_unnamed.switch_device =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<wlr_switch>() as libc::c_ulong)
                    as *mut wlr_switch;
            if (*wlr_device).c2rust_unnamed.switch_device.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Unable to allocate wlr_switch\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/headless/input_device.c\x00" as
                             *const u8 as *const libc::c_char, 83i32);
                current_block = 18267909660717707325;
            } else {
                wlr_switch_init((*wlr_device).c2rust_unnamed.switch_device,
                                0 as *mut wlr_switch_impl);
                current_block = 3123434771885419771;
            }
        }
        _ => { current_block = 3123434771885419771; }
    }
    match current_block {
        18267909660717707325 => {
            free(device as *mut libc::c_void);
            return 0 as *mut wlr_input_device
        }
        _ => {
            wl_list_insert(&mut (*backend).input_devices,
                           &mut (*wlr_device).link);
            if (*backend).started {
                wlr_signal_emit_safe(&mut (*backend).backend.events.new_input,
                                     wlr_device as *mut libc::c_void);
            }
            return wlr_device
        }
    };
}
