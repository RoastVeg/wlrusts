use libc;
extern "C" {
    pub type udev;
    pub type udev_monitor;
    pub type libinput;
    pub type libinput_device;
    pub type libinput_event;
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type session_impl;
    pub type wlr_renderer;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_pointer_impl;
    pub type wlr_tablet_pad_impl;
    pub type wlr_tablet_impl;
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn libinput_event_destroy(event: *mut libinput_event);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn libinput_udev_create_context(interface: *const libinput_interface,
                                    user_data: *mut libc::c_void,
                                    udev: *mut udev) -> *mut libinput;
    #[no_mangle]
    fn libinput_udev_assign_seat(libinput: *mut libinput,
                                 seat_id: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn libinput_get_fd(libinput: *mut libinput) -> libc::c_int;
    #[no_mangle]
    fn libinput_dispatch(libinput: *mut libinput) -> libc::c_int;
    #[no_mangle]
    fn libinput_get_event(libinput: *mut libinput) -> *mut libinput_event;
    #[no_mangle]
    fn libinput_resume(libinput: *mut libinput) -> libc::c_int;
    #[no_mangle]
    fn libinput_suspend(libinput: *mut libinput);
    #[no_mangle]
    fn libinput_unref(libinput: *mut libinput) -> *mut libinput;
    #[no_mangle]
    fn libinput_log_set_priority(libinput: *mut libinput,
                                 priority: libinput_log_priority);
    #[no_mangle]
    fn libinput_log_set_handler(libinput: *mut libinput,
                                log_handler: libinput_log_handler);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_event_loop_add_fd(loop_0: *mut wl_event_loop, fd: libc::c_int,
                            mask: uint32_t, func: wl_event_loop_fd_func_t,
                            data: *mut libc::c_void) -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn wl_display_add_destroy_listener(display: *mut wl_display,
                                       listener: *mut wl_listener);
    /*
 * Opens the file at path.
 * This can only be used to open DRM or evdev (input) devices.
 *
 * When the session becomes inactive:
 * - DRM files lose their DRM master status
 * - evdev files become invalid and should be closed
 *
 * Returns -errno on error.
 */
    #[no_mangle]
    fn wlr_session_open_file(session: *mut wlr_session,
                             path: *const libc::c_char) -> libc::c_int;
    /*
 * Closes a file previously opened with wlr_session_open_file.
 */
    #[no_mangle]
    fn wlr_session_close_file(session: *mut wlr_session, fd: libc::c_int);
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* *
 * Initializes common state on a wlr_backend and sets the implementation to the
 * provided wlr_backend_impl reference.
 */
    #[no_mangle]
    fn wlr_backend_init(backend: *mut wlr_backend,
                        impl_0: *const wlr_backend_impl);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn _wlr_vlog(verbosity: wlr_log_importance, format: *const libc::c_char,
                 args: ::std::ffi::VaList);
    /* *
 * Initialize a list. Returns true on success, false on failure.
 */
    #[no_mangle]
    fn wlr_list_init(list: *mut wlr_list) -> bool;
    /* *
 * Deinitialize a list.
 */
    #[no_mangle]
    fn wlr_list_finish(list: *mut wlr_list);
    #[no_mangle]
    fn handle_libinput_event(state: *mut wlr_libinput_backend,
                             event: *mut libinput_event);
    #[no_mangle]
    fn wlr_input_device_destroy(dev: *mut wlr_input_device);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __clockid_t = libc::c_int;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type clockid_t = __clockid_t;
pub type libinput_log_priority = libc::c_uint;
pub const LIBINPUT_LOG_PRIORITY_ERROR: libinput_log_priority = 30;
pub const LIBINPUT_LOG_PRIORITY_INFO: libinput_log_priority = 20;
pub const LIBINPUT_LOG_PRIORITY_DEBUG: libinput_log_priority = 10;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct libinput_interface {
    pub open_restricted: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: libc::c_int,
                                                     _: *mut libc::c_void)
                                    -> libc::c_int>,
    pub close_restricted: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut libc::c_void)
                                     -> ()>,
}
pub type libinput_log_handler
    =
    Option<unsafe extern "C" fn(_: *mut libinput, _: libinput_log_priority,
                                _: *const libc::c_char, _: ::std::ffi::VaList)
               -> ()>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
pub type C2RustUnnamed = libc::c_uint;
pub const WL_EVENT_ERROR: C2RustUnnamed = 8;
pub const WL_EVENT_HANGUP: C2RustUnnamed = 4;
pub const WL_EVENT_WRITABLE: C2RustUnnamed = 2;
pub const WL_EVENT_READABLE: C2RustUnnamed = 1;
pub type wl_event_loop_fd_func_t
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: uint32_t,
                                _: *mut libc::c_void) -> libc::c_int>;
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
    pub events: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}
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
    pub events: C2RustUnnamed_1,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
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
    pub repeat_info: C2RustUnnamed_3,
    pub events: C2RustUnnamed_2,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub rate: int32_t,
    pub delay: int32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
    pub events: C2RustUnnamed_4,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub events: C2RustUnnamed_5,
    pub data: *mut libc::c_void,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_6 {
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
    pub events: C2RustUnnamed_7,
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
pub struct C2RustUnnamed_7 {
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
    pub events: C2RustUnnamed_8,
    pub name: *mut libc::c_char,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
    pub events: C2RustUnnamed_9,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
    pub events: C2RustUnnamed_10,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub toggle: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* Note: these are circular dependencies */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_libinput_input_device {
    pub wlr_input_device: wlr_input_device,
    pub handle: *mut libinput_device,
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
unsafe extern "C" fn get_libinput_backend_from_backend(mut wlr_backend:
                                                           *mut wlr_backend)
 -> *mut wlr_libinput_backend {
    if wlr_backend_is_libinput(wlr_backend) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_backend_is_libinput(wlr_backend)\x00" as *const u8
                          as *const libc::c_char,
                      b"../backend/libinput/backend.c\x00" as *const u8 as
                          *const libc::c_char, 12i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[libc::c_char; 85]>(b"struct wlr_libinput_backend *get_libinput_backend_from_backend(struct wlr_backend *)\x00")).as_ptr());
    };
    return wlr_backend as *mut wlr_libinput_backend;
}
unsafe extern "C" fn libinput_open_restricted(mut path: *const libc::c_char,
                                              mut flags: libc::c_int,
                                              mut _backend: *mut libc::c_void)
 -> libc::c_int {
    let mut backend: *mut wlr_libinput_backend =
        _backend as *mut wlr_libinput_backend;
    return wlr_session_open_file((*backend).session, path);
}
unsafe extern "C" fn libinput_close_restricted(mut fd: libc::c_int,
                                               mut _backend:
                                                   *mut libc::c_void) {
    let mut backend: *mut wlr_libinput_backend =
        _backend as *mut wlr_libinput_backend;
    wlr_session_close_file((*backend).session, fd);
}
static mut libinput_impl: libinput_interface =
    unsafe {
        {
            let mut init =
                libinput_interface{open_restricted:
                                       Some(libinput_open_restricted as
                                                unsafe extern "C" fn(_:
                                                                         *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> libc::c_int),
                                   close_restricted:
                                       Some(libinput_close_restricted as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),};
            init
        }
    };
unsafe extern "C" fn handle_libinput_readable(mut fd: libc::c_int,
                                              mut mask: uint32_t,
                                              mut _backend: *mut libc::c_void)
 -> libc::c_int {
    let mut backend: *mut wlr_libinput_backend =
        _backend as *mut wlr_libinput_backend;
    if libinput_dispatch((*backend).libinput_context) != 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to dispatch libinput\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/libinput/backend.c\x00" as *const u8 as
                     *const libc::c_char, 35i32);
        // TODO: some kind of abort?
        return 0i32
    }
    let mut event: *mut libinput_event = 0 as *mut libinput_event;
    loop  {
        event = libinput_get_event((*backend).libinput_context);
        if event.is_null() { break ; }
        handle_libinput_event(backend, event);
        libinput_event_destroy(event);
    }
    return 0i32;
}
unsafe extern "C" fn log_libinput(mut libinput_context: *mut libinput,
                                  mut priority: libinput_log_priority,
                                  mut fmt: *const libc::c_char,
                                  mut args: ::std::ffi::VaList) {
    _wlr_vlog(WLR_ERROR, fmt, args.as_va_list());
}
unsafe extern "C" fn backend_start(mut wlr_backend: *mut wlr_backend)
 -> bool {
    let mut backend: *mut wlr_libinput_backend =
        get_libinput_backend_from_backend(wlr_backend);
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Initializing libinput\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/libinput/backend.c\x00" as *const u8 as
                 *const libc::c_char, 55i32);
    (*backend).libinput_context =
        libinput_udev_create_context(&libinput_impl,
                                     backend as *mut libc::c_void,
                                     (*(*backend).session).udev);
    if (*backend).libinput_context.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create libinput context\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/libinput/backend.c\x00" as *const u8 as
                     *const libc::c_char, 60i32);
        return 0i32 != 0
    }
    if libinput_udev_assign_seat((*backend).libinput_context,
                                 (*(*backend).session).seat.as_mut_ptr()) !=
           0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to assign libinput seat\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/libinput/backend.c\x00" as *const u8 as
                     *const libc::c_char, 66i32);
        return 0i32 != 0
    }
    // TODO: More sophisticated logging
    libinput_log_set_handler((*backend).libinput_context,
                             Some(log_libinput as
                                      unsafe extern "C" fn(_: *mut libinput,
                                                           _:
                                                               libinput_log_priority,
                                                           _:
                                                               *const libc::c_char,
                                                           _:
                                                               ::std::ffi::VaList)
                                          -> ()));
    libinput_log_set_priority((*backend).libinput_context,
                              LIBINPUT_LOG_PRIORITY_ERROR);
    let mut libinput_fd: libc::c_int =
        libinput_get_fd((*backend).libinput_context);
    let mut no_devs: *mut libc::c_char =
        getenv(b"WLR_LIBINPUT_NO_DEVICES\x00" as *const u8 as
                   *const libc::c_char);
    if !no_devs.is_null() {
        if strcmp(no_devs, b"1\x00" as *const u8 as *const libc::c_char) !=
               0i32 {
            no_devs = 0 as *mut libc::c_char
        }
    }
    if no_devs.is_null() &&
           (*backend).wlr_device_lists.length == 0i32 as libc::c_ulong {
        handle_libinput_readable(libinput_fd,
                                 WL_EVENT_READABLE as libc::c_int as uint32_t,
                                 backend as *mut libc::c_void);
        if (*backend).wlr_device_lists.length == 0i32 as libc::c_ulong {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] libinput initialization failed, no input devices\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/libinput/backend.c\x00" as *const u8 as
                         *const libc::c_char, 84i32);
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Set WLR_LIBINPUT_NO_DEVICES=1 to suppress this check\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/libinput/backend.c\x00" as *const u8 as
                         *const libc::c_char, 85i32);
            return 0i32 != 0
        }
    }
    let mut event_loop: *mut wl_event_loop =
        wl_display_get_event_loop((*backend).display);
    if !(*backend).input_event.is_null() {
        wl_event_source_remove((*backend).input_event);
    }
    (*backend).input_event =
        wl_event_loop_add_fd(event_loop, libinput_fd,
                             WL_EVENT_READABLE as libc::c_int as uint32_t,
                             Some(handle_libinput_readable as
                                      unsafe extern "C" fn(_: libc::c_int,
                                                           _: uint32_t,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),
                             backend as *mut libc::c_void);
    if (*backend).input_event.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create input event on event loop\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/libinput/backend.c\x00" as *const u8 as
                     *const libc::c_char, 98i32);
        return 0i32 != 0
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] libinput successfully initialized\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/libinput/backend.c\x00" as *const u8 as
                 *const libc::c_char, 101i32);
    return 1i32 != 0;
}
unsafe extern "C" fn backend_destroy(mut wlr_backend: *mut wlr_backend) {
    if wlr_backend.is_null() { return }
    let mut backend: *mut wlr_libinput_backend =
        get_libinput_backend_from_backend(wlr_backend);
    let mut i: size_t = 0i32 as size_t;
    while i < (*backend).wlr_device_lists.length {
        let mut wlr_devices: *mut wl_list =
            *(*backend).wlr_device_lists.items.offset(i as isize) as
                *mut wl_list;
        let mut wlr_dev: *mut wlr_input_device = 0 as *mut wlr_input_device;
        let mut next: *mut wlr_input_device = 0 as *mut wlr_input_device;
        wlr_dev =
            ((*wlr_devices).next as *mut libc::c_char).offset(-88) as
                *mut wlr_input_device;
        next =
            ((*wlr_dev).link.next as *mut libc::c_char).offset(-88) as
                *mut wlr_input_device;
        while &mut (*wlr_dev).link as *mut wl_list != wlr_devices {
            wlr_input_device_destroy(wlr_dev);
            wlr_dev = next;
            next =
                ((*wlr_dev).link.next as *mut libc::c_char).offset(-88) as
                    *mut wlr_input_device
        }
        free(wlr_devices as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    wlr_signal_emit_safe(&mut (*wlr_backend).events.destroy,
                         wlr_backend as *mut libc::c_void);
    wl_list_remove(&mut (*backend).display_destroy.link);
    wl_list_remove(&mut (*backend).session_destroy.link);
    wl_list_remove(&mut (*backend).session_signal.link);
    wlr_list_finish(&mut (*backend).wlr_device_lists);
    if !(*backend).input_event.is_null() {
        wl_event_source_remove((*backend).input_event);
    }
    libinput_unref((*backend).libinput_context);
    free(backend as *mut libc::c_void);
}
static mut backend_impl: wlr_backend_impl =
    unsafe {
        {
            let mut init =
                wlr_backend_impl{start:
                                     Some(backend_start as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> bool),
                                 destroy:
                                     Some(backend_destroy as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> ()),
                                 get_renderer: None,
                                 get_session: None,
                                 get_presentation_clock: None,};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_is_libinput(mut b: *mut wlr_backend)
 -> bool {
    return (*b).impl_0 == &backend_impl as *const wlr_backend_impl;
}
unsafe extern "C" fn session_signal(mut listener: *mut wl_listener,
                                    mut data: *mut libc::c_void) {
    let mut backend: *mut wlr_libinput_backend =
        (listener as *mut libc::c_char).offset(-136) as
            *mut wlr_libinput_backend;
    let mut session: *mut wlr_session = data as *mut wlr_session;
    if (*backend).libinput_context.is_null() { return }
    if (*session).active {
        libinput_resume((*backend).libinput_context);
    } else { libinput_suspend((*backend).libinput_context); };
}
unsafe extern "C" fn handle_session_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut backend: *mut wlr_libinput_backend =
        (listener as *mut libc::c_char).offset(-112) as
            *mut wlr_libinput_backend;
    backend_destroy(&mut (*backend).backend);
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut backend: *mut wlr_libinput_backend =
        (listener as *mut libc::c_char).offset(-88) as
            *mut wlr_libinput_backend;
    backend_destroy(&mut (*backend).backend);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_libinput_backend_create(mut display:
                                                         *mut wl_display,
                                                     mut session:
                                                         *mut wlr_session)
 -> *mut wlr_backend {
    let mut backend: *mut wlr_libinput_backend =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_libinput_backend>() as libc::c_ulong)
            as *mut wlr_libinput_backend;
    if backend.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/libinput/backend.c\x00" as *const u8 as
                     *const libc::c_char, 177i32,
                 strerror(*__errno_location()));
        return 0 as *mut wlr_backend
    }
    wlr_backend_init(&mut (*backend).backend, &backend_impl);
    if !wlr_list_init(&mut (*backend).wlr_device_lists) {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/libinput/backend.c\x00" as *const u8 as
                     *const libc::c_char, 183i32,
                 strerror(*__errno_location()));
        free(backend as *mut libc::c_void);
        return 0 as *mut wlr_backend
    } else {
        (*backend).session = session;
        (*backend).display = display;
        (*backend).session_signal.notify =
            Some(session_signal as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*session).session_signal,
                      &mut (*backend).session_signal);
        (*backend).session_destroy.notify =
            Some(handle_session_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*session).events.destroy,
                      &mut (*backend).session_destroy);
        (*backend).display_destroy.notify =
            Some(handle_display_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_display_add_destroy_listener(display,
                                        &mut (*backend).display_destroy);
        return &mut (*backend).backend
    };
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* * Gets the underlying libinput_device handle for the given wlr_input_device */
#[no_mangle]
pub unsafe extern "C" fn wlr_libinput_get_device_handle(mut wlr_dev:
                                                            *mut wlr_input_device)
 -> *mut libinput_device {
    let mut dev: *mut wlr_libinput_input_device =
        wlr_dev as *mut wlr_libinput_input_device;
    return (*dev).handle;
}
// list of struct wl_list
#[no_mangle]
pub unsafe extern "C" fn usec_to_msec(mut usec: uint64_t) -> uint32_t {
    return usec.wrapping_div(1000i32 as libc::c_ulong) as uint32_t;
}
