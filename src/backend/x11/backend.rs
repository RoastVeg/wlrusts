use libc;
extern "C" {
    pub type xcb_connection_t;
    pub type xcb_extension_t;
    pub type _XDisplay;
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type udev;
    pub type udev_monitor;
    pub type session_impl;
    pub type wlr_renderer_impl;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_group;
    pub type wlr_tablet_pad_impl;
    pub type wlr_tablet_impl;
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_texture_impl;
    pub type wlr_surface;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn xcb_setup_roots_iterator(R: *const xcb_setup_t)
     -> xcb_screen_iterator_t;
    #[no_mangle]
    fn xcb_intern_atom(c: *mut xcb_connection_t, only_if_exists: uint8_t,
                       name_len: uint16_t, name: *const libc::c_char)
     -> xcb_intern_atom_cookie_t;
    #[no_mangle]
    fn xcb_intern_atom_reply(c: *mut xcb_connection_t,
                             cookie: xcb_intern_atom_cookie_t,
                             e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_intern_atom_reply_t;
    #[no_mangle]
    fn xcb_poll_for_event(c: *mut xcb_connection_t)
     -> *mut xcb_generic_event_t;
    #[no_mangle]
    fn xcb_get_extension_data(c: *mut xcb_connection_t,
                              ext: *mut xcb_extension_t)
     -> *const xcb_query_extension_reply_t;
    #[no_mangle]
    fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
    #[no_mangle]
    fn xcb_get_file_descriptor(c: *mut xcb_connection_t) -> libc::c_int;
    #[no_mangle]
    fn xcb_connection_has_error(c: *mut xcb_connection_t) -> libc::c_int;
    #[no_mangle]
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    #[no_mangle]
    fn XCloseDisplay(_: *mut Display) -> libc::c_int;
    #[no_mangle]
    fn XSetEventQueueOwner(dpy: *mut Display, owner: XEventQueueOwner);
    #[no_mangle]
    fn XGetXCBConnection(dpy: *mut Display) -> *mut xcb_connection_t;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_event_loop_add_fd(loop_0: *mut wl_event_loop, fd: libc::c_int,
                            mask: uint32_t, func: wl_event_loop_fd_func_t,
                            data: *mut libc::c_void) -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_event_source_check(source: *mut wl_event_source);
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn wl_display_terminate(display: *mut wl_display);
    #[no_mangle]
    fn wl_display_add_destroy_listener(display: *mut wl_display,
                                       listener: *mut wl_listener);
    #[no_mangle]
    static mut xcb_xfixes_id: xcb_extension_t;
    #[no_mangle]
    fn xcb_xfixes_query_version(c: *mut xcb_connection_t,
                                client_major_version: uint32_t,
                                client_minor_version: uint32_t)
     -> xcb_xfixes_query_version_cookie_t;
    #[no_mangle]
    fn xcb_xfixes_query_version_reply(c: *mut xcb_connection_t,
                                      cookie:
                                          xcb_xfixes_query_version_cookie_t,
                                      e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_xfixes_query_version_reply_t;
    #[no_mangle]
    static mut xcb_input_id: xcb_extension_t;
    #[no_mangle]
    fn xcb_input_xi_query_version(c: *mut xcb_connection_t,
                                  major_version: uint16_t,
                                  minor_version: uint16_t)
     -> xcb_input_xi_query_version_cookie_t;
    #[no_mangle]
    fn xcb_input_xi_query_version_reply(c: *mut xcb_connection_t,
                                        cookie:
                                            xcb_input_xi_query_version_cookie_t,
                                        e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_input_xi_query_version_reply_t;
    /* *
 * Frees all related EGL resources, makes the context not-current and
 * unbinds a bound wayland display.
 */
    #[no_mangle]
    fn wlr_egl_finish(egl: *mut wlr_egl);
    /* *
 * Initializes common state on a wlr_backend and sets the implementation to the
 * provided wlr_backend_impl reference.
 */
    #[no_mangle]
    fn wlr_backend_init(backend: *mut wlr_backend,
                        impl_0: *const wlr_backend_impl);
    #[no_mangle]
    fn wlr_x11_output_create(backend: *mut wlr_backend) -> *mut wlr_output;
    #[no_mangle]
    fn wlr_output_destroy(output: *mut wlr_output);
    #[no_mangle]
    fn wlr_input_device_init(wlr_device: *mut wlr_input_device,
                             type_0: wlr_input_device_type,
                             impl_0: *const wlr_input_device_impl,
                             name: *const libc::c_char, vendor: libc::c_int,
                             product: libc::c_int);
    #[no_mangle]
    fn wlr_input_device_destroy(dev: *mut wlr_input_device);
    #[no_mangle]
    fn wlr_keyboard_init(keyboard: *mut wlr_keyboard,
                         impl_0: *const wlr_keyboard_impl);
    #[no_mangle]
    fn wlr_renderer_autocreate(egl: *mut wlr_egl, platform: EGLenum,
                               remote_display: *mut libc::c_void,
                               config_attribs: *mut EGLint, visual_id: EGLint)
     -> *mut wlr_renderer;
    /* *
 * Destroys this wlr_renderer. Textures must be destroyed separately.
 */
    #[no_mangle]
    fn wlr_renderer_destroy(renderer: *mut wlr_renderer);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn wlr_output_update_needs_frame(output: *mut wlr_output);
    #[no_mangle]
    static keyboard_impl: wlr_keyboard_impl;
    #[no_mangle]
    static input_device_impl: wlr_input_device_impl;
    #[no_mangle]
    fn handle_x11_xinput_event(x11: *mut wlr_x11_backend,
                               event: *mut xcb_ge_generic_event_t);
    #[no_mangle]
    fn handle_x11_configure_notify(output: *mut wlr_x11_output,
                                   event: *mut xcb_configure_notify_event_t);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __clockid_t = libc::c_int;
pub type clockid_t = __clockid_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_generic_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub resource_id: uint32_t,
    pub minor_code: uint16_t,
    pub major_code: uint8_t,
    pub pad0: uint8_t,
    pub pad: [uint32_t; 5],
    pub full_sequence: uint32_t,
}
pub type xcb_window_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_screen_iterator_t {
    pub data: *mut xcb_screen_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_setup_t {
    pub status: uint8_t,
    pub pad0: uint8_t,
    pub protocol_major_version: uint16_t,
    pub protocol_minor_version: uint16_t,
    pub length: uint16_t,
    pub release_number: uint32_t,
    pub resource_id_base: uint32_t,
    pub resource_id_mask: uint32_t,
    pub motion_buffer_size: uint32_t,
    pub vendor_len: uint16_t,
    pub maximum_request_length: uint16_t,
    pub roots_len: uint8_t,
    pub pixmap_formats_len: uint8_t,
    pub image_byte_order: uint8_t,
    pub bitmap_format_bit_order: uint8_t,
    pub bitmap_format_scanline_unit: uint8_t,
    pub bitmap_format_scanline_pad: uint8_t,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [uint8_t; 4],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_expose_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub x: uint16_t,
    pub y: uint16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub count: uint16_t,
    pub pad1: [uint8_t; 2],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_configure_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub override_redirect: uint8_t,
    pub pad1: uint8_t,
}
pub type xcb_atom_enum_t = libc::c_uint;
pub const XCB_ATOM_WM_TRANSIENT_FOR: xcb_atom_enum_t = 68;
pub const XCB_ATOM_WM_CLASS: xcb_atom_enum_t = 67;
pub const XCB_ATOM_CAP_HEIGHT: xcb_atom_enum_t = 66;
pub const XCB_ATOM_FULL_NAME: xcb_atom_enum_t = 65;
pub const XCB_ATOM_FAMILY_NAME: xcb_atom_enum_t = 64;
pub const XCB_ATOM_FONT_NAME: xcb_atom_enum_t = 63;
pub const XCB_ATOM_NOTICE: xcb_atom_enum_t = 62;
pub const XCB_ATOM_COPYRIGHT: xcb_atom_enum_t = 61;
pub const XCB_ATOM_RESOLUTION: xcb_atom_enum_t = 60;
pub const XCB_ATOM_POINT_SIZE: xcb_atom_enum_t = 59;
pub const XCB_ATOM_WEIGHT: xcb_atom_enum_t = 58;
pub const XCB_ATOM_QUAD_WIDTH: xcb_atom_enum_t = 57;
pub const XCB_ATOM_X_HEIGHT: xcb_atom_enum_t = 56;
pub const XCB_ATOM_ITALIC_ANGLE: xcb_atom_enum_t = 55;
pub const XCB_ATOM_STRIKEOUT_DESCENT: xcb_atom_enum_t = 54;
pub const XCB_ATOM_STRIKEOUT_ASCENT: xcb_atom_enum_t = 53;
pub const XCB_ATOM_UNDERLINE_THICKNESS: xcb_atom_enum_t = 52;
pub const XCB_ATOM_UNDERLINE_POSITION: xcb_atom_enum_t = 51;
pub const XCB_ATOM_SUBSCRIPT_Y: xcb_atom_enum_t = 50;
pub const XCB_ATOM_SUBSCRIPT_X: xcb_atom_enum_t = 49;
pub const XCB_ATOM_SUPERSCRIPT_Y: xcb_atom_enum_t = 48;
pub const XCB_ATOM_SUPERSCRIPT_X: xcb_atom_enum_t = 47;
pub const XCB_ATOM_END_SPACE: xcb_atom_enum_t = 46;
pub const XCB_ATOM_MAX_SPACE: xcb_atom_enum_t = 45;
pub const XCB_ATOM_NORM_SPACE: xcb_atom_enum_t = 44;
pub const XCB_ATOM_MIN_SPACE: xcb_atom_enum_t = 43;
pub const XCB_ATOM_WM_ZOOM_HINTS: xcb_atom_enum_t = 42;
pub const XCB_ATOM_WM_SIZE_HINTS: xcb_atom_enum_t = 41;
pub const XCB_ATOM_WM_NORMAL_HINTS: xcb_atom_enum_t = 40;
pub const XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;
pub const XCB_ATOM_WM_ICON_SIZE: xcb_atom_enum_t = 38;
pub const XCB_ATOM_WM_ICON_NAME: xcb_atom_enum_t = 37;
pub const XCB_ATOM_WM_CLIENT_MACHINE: xcb_atom_enum_t = 36;
pub const XCB_ATOM_WM_HINTS: xcb_atom_enum_t = 35;
pub const XCB_ATOM_WM_COMMAND: xcb_atom_enum_t = 34;
pub const XCB_ATOM_WINDOW: xcb_atom_enum_t = 33;
pub const XCB_ATOM_VISUALID: xcb_atom_enum_t = 32;
pub const XCB_ATOM_STRING: xcb_atom_enum_t = 31;
pub const XCB_ATOM_RGB_RED_MAP: xcb_atom_enum_t = 30;
pub const XCB_ATOM_RGB_GREEN_MAP: xcb_atom_enum_t = 29;
pub const XCB_ATOM_RGB_GRAY_MAP: xcb_atom_enum_t = 28;
pub const XCB_ATOM_RGB_DEFAULT_MAP: xcb_atom_enum_t = 27;
pub const XCB_ATOM_RGB_BLUE_MAP: xcb_atom_enum_t = 26;
pub const XCB_ATOM_RGB_BEST_MAP: xcb_atom_enum_t = 25;
pub const XCB_ATOM_RGB_COLOR_MAP: xcb_atom_enum_t = 24;
pub const XCB_ATOM_RESOURCE_MANAGER: xcb_atom_enum_t = 23;
pub const XCB_ATOM_RECTANGLE: xcb_atom_enum_t = 22;
pub const XCB_ATOM_POINT: xcb_atom_enum_t = 21;
pub const XCB_ATOM_PIXMAP: xcb_atom_enum_t = 20;
pub const XCB_ATOM_INTEGER: xcb_atom_enum_t = 19;
pub const XCB_ATOM_FONT: xcb_atom_enum_t = 18;
pub const XCB_ATOM_DRAWABLE: xcb_atom_enum_t = 17;
pub const XCB_ATOM_CUT_BUFFER7: xcb_atom_enum_t = 16;
pub const XCB_ATOM_CUT_BUFFER6: xcb_atom_enum_t = 15;
pub const XCB_ATOM_CUT_BUFFER5: xcb_atom_enum_t = 14;
pub const XCB_ATOM_CUT_BUFFER4: xcb_atom_enum_t = 13;
pub const XCB_ATOM_CUT_BUFFER3: xcb_atom_enum_t = 12;
pub const XCB_ATOM_CUT_BUFFER2: xcb_atom_enum_t = 11;
pub const XCB_ATOM_CUT_BUFFER1: xcb_atom_enum_t = 10;
pub const XCB_ATOM_CUT_BUFFER0: xcb_atom_enum_t = 9;
pub const XCB_ATOM_CURSOR: xcb_atom_enum_t = 8;
pub const XCB_ATOM_COLORMAP: xcb_atom_enum_t = 7;
pub const XCB_ATOM_CARDINAL: xcb_atom_enum_t = 6;
pub const XCB_ATOM_BITMAP: xcb_atom_enum_t = 5;
pub const XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
pub const XCB_ATOM_ARC: xcb_atom_enum_t = 3;
pub const XCB_ATOM_SECONDARY: xcb_atom_enum_t = 2;
pub const XCB_ATOM_PRIMARY: xcb_atom_enum_t = 1;
pub const XCB_ATOM_ANY: xcb_atom_enum_t = 0;
pub const XCB_ATOM_NONE: xcb_atom_enum_t = 0;
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union xcb_client_message_data_t {
    pub data8: [uint8_t; 20],
    pub data16: [uint16_t; 10],
    pub data32: [uint32_t; 5],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_client_message_event_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub type_0: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_ge_generic_event_t {
    pub response_type: uint8_t,
    pub extension: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub event_type: uint16_t,
    pub pad0: [uint8_t; 22],
    pub full_sequence: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_intern_atom_cookie_t {
    pub sequence: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub atom: xcb_atom_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub present: uint8_t,
    pub major_opcode: uint8_t,
    pub first_event: uint8_t,
    pub first_error: uint8_t,
}
pub type Display = _XDisplay;
pub type XEventQueueOwner = libc::c_uint;
pub const XCBOwnsEventQueue: XEventQueueOwner = 1;
pub const XlibOwnsEventQueue: XEventQueueOwner = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_object {
    pub interface: *const wl_interface,
    pub implementation: *const libc::c_void,
    pub id: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_interface {
    pub name: *const libc::c_char,
    pub version: libc::c_int,
    pub method_count: libc::c_int,
    pub methods: *const wl_message,
    pub event_count: libc::c_int,
    pub events: *const wl_message,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_message {
    pub name: *const libc::c_char,
    pub signature: *const libc::c_char,
    pub types: *mut *const wl_interface,
}
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
pub struct wl_resource {
    pub object: wl_object,
    pub destroy: wl_resource_destroy_func_t,
    pub link: wl_list,
    pub destroy_signal: wl_signal,
    pub client: *mut wl_client,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
pub type wl_resource_destroy_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_resource) -> ()>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_xfixes_query_version_cookie_t {
    pub sequence: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_xfixes_query_version_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub major_version: uint32_t,
    pub minor_version: uint32_t,
    pub pad1: [uint8_t; 16],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_input_xi_query_version_cookie_t {
    pub sequence: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_input_xi_query_version_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub major_version: uint16_t,
    pub minor_version: uint16_t,
    pub pad1: [uint8_t; 20],
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
pub type khronos_int32_t = int32_t;
pub type EGLint = khronos_int32_t;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
/*
 * 32 bit regions
 */
pub type pixman_region32_data_t = pixman_region32_data;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_dmabuf_attributes {
    pub width: int32_t,
    pub height: int32_t,
    pub format: uint32_t,
    pub flags: uint32_t,
    pub modifier: uint64_t,
    pub n_planes: libc::c_int,
    pub offset: [uint32_t; 4],
    pub stride: [uint32_t; 4],
    pub fd: [libc::c_int; 4],
}
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
    pub exts: C2RustUnnamed_1,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
    pub events: C2RustUnnamed_2,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_3,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub destroy: wl_signal,
}
pub type wlr_renderer_create_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wlr_egl, _: EGLenum,
                                _: *mut libc::c_void, _: *mut EGLint,
                                _: EGLint) -> *mut wlr_renderer>;
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
    pub repeat_info: C2RustUnnamed_5,
    pub events: C2RustUnnamed_4,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
pub struct wlr_pointer_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_pointer) -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
    pub events: C2RustUnnamed_6,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub events: C2RustUnnamed_7,
    pub data: *mut libc::c_void,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_8 {
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
    pub events: C2RustUnnamed_9,
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
pub struct C2RustUnnamed_9 {
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
    pub events: C2RustUnnamed_10,
    pub name: *mut libc::c_char,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
    pub events: C2RustUnnamed_11,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
    pub events: C2RustUnnamed_12,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
pub type wl_output_subpixel = libc::c_uint;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: wl_output_subpixel = 5;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: wl_output_subpixel = 4;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: wl_output_subpixel = 3;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: wl_output_subpixel = 2;
pub const WL_OUTPUT_SUBPIXEL_NONE: wl_output_subpixel = 1;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: wl_output_subpixel = 0;
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * A client buffer.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_texture {
    pub impl_0: *const wlr_texture_impl,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_cursor {
    pub output: *mut wlr_output,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub enabled: bool,
    pub visible: bool,
    pub width: uint32_t,
    pub height: uint32_t,
    pub hotspot_x: int32_t,
    pub hotspot_y: int32_t,
    pub link: wl_list,
    pub texture: *mut wlr_texture,
    pub surface: *mut wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_13,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output {
    pub impl_0: *const wlr_output_impl,
    pub backend: *mut wlr_backend,
    pub display: *mut wl_display,
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub name: [libc::c_char; 24],
    pub make: [libc::c_char; 56],
    pub model: [libc::c_char; 16],
    pub serial: [libc::c_char; 16],
    pub phys_width: int32_t,
    pub phys_height: int32_t,
    pub modes: wl_list,
    pub current_mode: *mut wlr_output_mode,
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub enabled: bool,
    pub scale: libc::c_float,
    pub subpixel: wl_output_subpixel,
    pub transform: wl_output_transform,
    pub needs_frame: bool,
    pub damage: pixman_region32_t,
    pub frame_pending: bool,
    pub transform_matrix: [libc::c_float; 9],
    pub pending: wlr_output_state,
    pub commit_seq: uint32_t,
    pub events: C2RustUnnamed_14,
    pub idle_frame: *mut wl_event_source,
    pub idle_done: *mut wl_event_source,
    pub attach_render_locks: libc::c_int,
    pub cursors: wl_list,
    pub hardware_cursor: *mut wlr_output_cursor,
    pub software_cursor_locks: libc::c_int,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub frame: wl_signal,
    pub needs_frame: wl_signal,
    pub precommit: wl_signal,
    pub commit: wl_signal,
    pub present: wl_signal,
    pub enable: wl_signal,
    pub mode: wl_signal,
    pub scale: wl_signal,
    pub transform: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub damage: pixman_region32_t,
    pub buffer_type: wlr_output_state_buffer_type,
    pub buffer: *mut wlr_buffer,
}
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_impl {
    pub enable: Option<unsafe extern "C" fn(_: *mut wlr_output, _: bool)
                           -> bool>,
    pub set_mode: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                              _: *mut wlr_output_mode)
                             -> bool>,
    pub set_custom_mode: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                     _: int32_t, _: int32_t,
                                                     _: int32_t) -> bool>,
    pub set_cursor: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                _: *mut wlr_texture,
                                                _: int32_t,
                                                _: wl_output_transform,
                                                _: int32_t, _: int32_t,
                                                _: bool) -> bool>,
    pub move_cursor: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                 _: libc::c_int,
                                                 _: libc::c_int) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_output) -> ()>,
    pub attach_render: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                   _: *mut libc::c_int)
                                  -> bool>,
    pub commit: Option<unsafe extern "C" fn(_: *mut wlr_output) -> bool>,
    pub set_gamma: Option<unsafe extern "C" fn(_: *mut wlr_output, _: size_t,
                                               _: *const uint16_t,
                                               _: *const uint16_t,
                                               _: *const uint16_t) -> bool>,
    pub get_gamma_size: Option<unsafe extern "C" fn(_: *mut wlr_output)
                                   -> size_t>,
    pub export_dmabuf: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                   _:
                                                       *mut wlr_dmabuf_attributes)
                                  -> bool>,
    pub schedule_frame: Option<unsafe extern "C" fn(_: *mut wlr_output)
                                   -> bool>,
    pub attach_buffer: Option<unsafe extern "C" fn(_: *mut wlr_output,
                                                   _: *mut wlr_buffer)
                                  -> bool>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_x11_output {
    pub wlr_output: wlr_output,
    pub x11: *mut wlr_x11_backend,
    pub link: wl_list,
    pub win: xcb_window_t,
    pub surf: EGLSurface,
    pub pointer: wlr_pointer,
    pub pointer_dev: wlr_input_device,
    pub touch: wlr_touch,
    pub touch_dev: wlr_input_device,
    pub touchpoints: wl_list,
    pub frame_timer: *mut wl_event_source,
    pub frame_delay: libc::c_int,
    pub cursor_hidden: bool,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_x11_backend {
    pub backend: wlr_backend,
    pub wl_display: *mut wl_display,
    pub started: bool,
    pub xlib_conn: *mut Display,
    pub xcb: *mut xcb_connection_t,
    pub screen: *mut xcb_screen_t,
    pub requested_outputs: size_t,
    pub last_output_num: size_t,
    pub outputs: wl_list,
    pub keyboard: wlr_keyboard,
    pub keyboard_dev: wlr_input_device,
    pub egl: wlr_egl,
    pub renderer: *mut wlr_renderer,
    pub event_source: *mut wl_event_source,
    pub atoms: C2RustUnnamed_15,
    pub time: xcb_timestamp_t,
    pub xinput_opcode: uint8_t,
    pub display_destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub wm_protocols: xcb_atom_t,
    pub wm_delete_window: xcb_atom_t,
    pub net_wm_name: xcb_atom_t,
    pub utf8_string: xcb_atom_t,
}
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub name: *const libc::c_char,
    pub cookie: xcb_intern_atom_cookie_t,
    pub atom: *mut xcb_atom_t,
}
#[no_mangle]
pub unsafe extern "C" fn get_x11_output_from_window_id(mut x11:
                                                           *mut wlr_x11_backend,
                                                       mut window:
                                                           xcb_window_t)
 -> *mut wlr_x11_output {
    let mut output: *mut wlr_x11_output = 0 as *mut wlr_x11_output;
    output =
        ((*x11).outputs.next as *mut libc::c_char).offset(-600) as
            *mut wlr_x11_output;
    while &mut (*output).link as *mut wl_list !=
              &mut (*x11).outputs as *mut wl_list {
        if (*output).win == window { return output }
        output =
            ((*output).link.next as *mut libc::c_char).offset(-600) as
                *mut wlr_x11_output
    }
    return 0 as *mut wlr_x11_output;
}
unsafe extern "C" fn handle_x11_event(mut x11: *mut wlr_x11_backend,
                                      mut event: *mut xcb_generic_event_t) {
    match (*event).response_type as libc::c_int & 0x7fi32 {
        12 => {
            let mut ev: *mut xcb_expose_event_t =
                event as *mut xcb_expose_event_t;
            let mut output: *mut wlr_x11_output =
                get_x11_output_from_window_id(x11, (*ev).window);
            if !output.is_null() {
                wlr_output_update_needs_frame(&mut (*output).wlr_output);
            }
        }
        22 => {
            let mut ev_0: *mut xcb_configure_notify_event_t =
                event as *mut xcb_configure_notify_event_t;
            let mut output_0: *mut wlr_x11_output =
                get_x11_output_from_window_id(x11, (*ev_0).window);
            if !output_0.is_null() {
                handle_x11_configure_notify(output_0, ev_0);
            }
        }
        33 => {
            let mut ev_1: *mut xcb_client_message_event_t =
                event as *mut xcb_client_message_event_t;
            if (*ev_1).data.data32[0] == (*x11).atoms.wm_delete_window {
                let mut output_1: *mut wlr_x11_output =
                    get_x11_output_from_window_id(x11, (*ev_1).window);
                if !output_1.is_null() {
                    wlr_output_destroy(&mut (*output_1).wlr_output);
                }
            }
        }
        35 => {
            let mut ev_2: *mut xcb_ge_generic_event_t =
                event as *mut xcb_ge_generic_event_t;
            if (*ev_2).extension as libc::c_int ==
                   (*x11).xinput_opcode as libc::c_int {
                handle_x11_xinput_event(x11, ev_2);
            }
        }
        _ => { }
    };
}
unsafe extern "C" fn x11_event(mut fd: libc::c_int, mut mask: uint32_t,
                               mut data: *mut libc::c_void) -> libc::c_int {
    let mut x11: *mut wlr_x11_backend = data as *mut wlr_x11_backend;
    if mask & WL_EVENT_HANGUP as libc::c_int as libc::c_uint != 0 ||
           mask & WL_EVENT_ERROR as libc::c_int as libc::c_uint != 0 {
        wl_display_terminate((*x11).wl_display);
        return 0i32
    }
    let mut e: *mut xcb_generic_event_t = 0 as *mut xcb_generic_event_t;
    loop  {
        e = xcb_poll_for_event((*x11).xcb);
        if e.is_null() { break ; }
        handle_x11_event(x11, e);
        free(e as *mut libc::c_void);
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn get_x11_backend_from_backend(mut wlr_backend:
                                                          *mut wlr_backend)
 -> *mut wlr_x11_backend {
    if wlr_backend_is_x11(wlr_backend) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_backend_is_x11(wlr_backend)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/x11/backend.c\x00" as *const u8 as
                          *const libc::c_char, 102i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 75],
                                                &[libc::c_char; 75]>(b"struct wlr_x11_backend *get_x11_backend_from_backend(struct wlr_backend *)\x00")).as_ptr());
    };
    return wlr_backend as *mut wlr_x11_backend;
}
unsafe extern "C" fn backend_start(mut backend: *mut wlr_backend) -> bool {
    let mut x11: *mut wlr_x11_backend = get_x11_backend_from_backend(backend);
    (*x11).started = 1i32 != 0;
    wlr_signal_emit_safe(&mut (*x11).backend.events.new_input,
                         &mut (*x11).keyboard_dev as *mut wlr_input_device as
                             *mut libc::c_void);
    let mut i: size_t = 0i32 as size_t;
    while i < (*x11).requested_outputs {
        wlr_x11_output_create(&mut (*x11).backend);
        i = i.wrapping_add(1)
    }
    return 1i32 != 0;
}
unsafe extern "C" fn backend_destroy(mut backend: *mut wlr_backend) {
    if backend.is_null() { return }
    let mut x11: *mut wlr_x11_backend = get_x11_backend_from_backend(backend);
    let mut output: *mut wlr_x11_output = 0 as *mut wlr_x11_output;
    let mut tmp: *mut wlr_x11_output = 0 as *mut wlr_x11_output;
    output =
        ((*x11).outputs.next as *mut libc::c_char).offset(-600) as
            *mut wlr_x11_output;
    tmp =
        ((*output).link.next as *mut libc::c_char).offset(-600) as
            *mut wlr_x11_output;
    while &mut (*output).link as *mut wl_list !=
              &mut (*x11).outputs as *mut wl_list {
        wlr_output_destroy(&mut (*output).wlr_output);
        output = tmp;
        tmp =
            ((*output).link.next as *mut libc::c_char).offset(-600) as
                *mut wlr_x11_output
    }
    wlr_input_device_destroy(&mut (*x11).keyboard_dev);
    wlr_signal_emit_safe(&mut (*backend).events.destroy,
                         backend as *mut libc::c_void);
    if !(*x11).event_source.is_null() {
        wl_event_source_remove((*x11).event_source);
    }
    wl_list_remove(&mut (*x11).display_destroy.link);
    wlr_renderer_destroy((*x11).renderer);
    wlr_egl_finish(&mut (*x11).egl);
    if !(*x11).xlib_conn.is_null() { XCloseDisplay((*x11).xlib_conn); }
    free(x11 as *mut libc::c_void);
}
unsafe extern "C" fn backend_get_renderer(mut backend: *mut wlr_backend)
 -> *mut wlr_renderer {
    let mut x11: *mut wlr_x11_backend = get_x11_backend_from_backend(backend);
    return (*x11).renderer;
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
                                 get_renderer:
                                     Some(backend_get_renderer as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> *mut wlr_renderer),
                                 get_session: None,
                                 get_presentation_clock: None,};
            init
        }
    };
/* *
 * Adds a new output to this backend. You may remove outputs by destroying them.
 * Note that if called before initializing the backend, this will return NULL
 * and your outputs will be created during initialization (and given to you via
 * the output_add signal).
 */
/* *
 * True if the given backend is a wlr_x11_backend.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_is_x11(mut backend: *mut wlr_backend)
 -> bool {
    return (*backend).impl_0 == &backend_impl as *const wlr_backend_impl;
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut x11: *mut wlr_x11_backend =
        (listener as *mut libc::c_char).offset(-696) as *mut wlr_x11_backend;
    backend_destroy(&mut (*x11).backend);
}
/* *
 * Creates a new wlr_x11_backend. This backend will be created with no outputs;
 * you must use wlr_x11_output_create to add them.
 *
 * The `x11_display` argument is the name of the X Display socket. Set
 * to NULL for the default behaviour of XOpenDisplay.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_x11_backend_create(mut display: *mut wl_display,
                                                mut x11_display:
                                                    *const libc::c_char,
                                                mut create_renderer_func:
                                                    wlr_renderer_create_func_t)
 -> *mut wlr_backend {
    let mut atom: [C2RustUnnamed_16; 4] =
        [C2RustUnnamed_16{name: 0 as *const libc::c_char,
                          cookie: xcb_intern_atom_cookie_t{sequence: 0,},
                          atom: 0 as *mut xcb_atom_t,}; 4];
    let mut ext: *const xcb_query_extension_reply_t =
        0 as *const xcb_query_extension_reply_t;
    let mut fixes_cookie: xcb_xfixes_query_version_cookie_t =
        xcb_xfixes_query_version_cookie_t{sequence: 0,};
    let mut fixes_reply: *mut xcb_xfixes_query_version_reply_t =
        0 as *mut xcb_xfixes_query_version_reply_t;
    let mut xi_cookie: xcb_input_xi_query_version_cookie_t =
        xcb_input_xi_query_version_cookie_t{sequence: 0,};
    let mut xi_reply: *mut xcb_input_xi_query_version_reply_t =
        0 as *mut xcb_input_xi_query_version_reply_t;
    let mut fd: libc::c_int = 0;
    let mut ev: *mut wl_event_loop = 0 as *mut wl_event_loop;
    let mut events: uint32_t = 0;
    static mut config_attribs: [EGLint; 11] =
        [0x3033i32, 0x4i32, 0x3024i32, 1i32, 0x3023i32, 1i32, 0x3022i32, 1i32,
         0x3021i32, 0i32, 0x3038i32];
    let mut x11: *mut wlr_x11_backend =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_x11_backend>() as libc::c_ulong) as
            *mut wlr_x11_backend;
    if x11.is_null() { return 0 as *mut wlr_backend }
    wlr_backend_init(&mut (*x11).backend, &backend_impl);
    (*x11).wl_display = display;
    wl_list_init(&mut (*x11).outputs);
    (*x11).xlib_conn = XOpenDisplay(x11_display);
    if (*x11).xlib_conn.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to open X connection\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/x11/backend.c\x00" as *const u8 as
                     *const libc::c_char, 185i32);
    } else {
        (*x11).xcb = XGetXCBConnection((*x11).xlib_conn);
        if (*x11).xcb.is_null() || xcb_connection_has_error((*x11).xcb) != 0 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to open xcb connection\x00" as *const u8
                         as *const libc::c_char,
                     b"../backend/x11/backend.c\x00" as *const u8 as
                         *const libc::c_char, 191i32);
        } else {
            XSetEventQueueOwner((*x11).xlib_conn, XCBOwnsEventQueue);
            atom =
                [{
                     let mut init =
                         C2RustUnnamed_16{name:
                                              b"WM_PROTOCOLS\x00" as *const u8
                                                  as *const libc::c_char,
                                          cookie:
                                              xcb_intern_atom_cookie_t{sequence:
                                                                           0,},
                                          atom:
                                              &mut (*x11).atoms.wm_protocols,};
                     init
                 },
                 {
                     let mut init =
                         C2RustUnnamed_16{name:
                                              b"WM_DELETE_WINDOW\x00" as
                                                  *const u8 as
                                                  *const libc::c_char,
                                          cookie:
                                              xcb_intern_atom_cookie_t{sequence:
                                                                           0,},
                                          atom:
                                              &mut (*x11).atoms.wm_delete_window,};
                     init
                 },
                 {
                     let mut init =
                         C2RustUnnamed_16{name:
                                              b"_NET_WM_NAME\x00" as *const u8
                                                  as *const libc::c_char,
                                          cookie:
                                              xcb_intern_atom_cookie_t{sequence:
                                                                           0,},
                                          atom:
                                              &mut (*x11).atoms.net_wm_name,};
                     init
                 },
                 {
                     let mut init =
                         C2RustUnnamed_16{name:
                                              b"UTF8_STRING\x00" as *const u8
                                                  as *const libc::c_char,
                                          cookie:
                                              xcb_intern_atom_cookie_t{sequence:
                                                                           0,},
                                          atom:
                                              &mut (*x11).atoms.utf8_string,};
                     init
                 }];
            let mut i: size_t = 0i32 as size_t;
            while i <
                      (::std::mem::size_of::<[C2RustUnnamed_16; 4]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_16>()
                                                           as libc::c_ulong) {
                atom[i as usize].cookie =
                    xcb_intern_atom((*x11).xcb, 1i32 as uint8_t,
                                    strlen(atom[i as usize].name) as uint16_t,
                                    atom[i as usize].name);
                i = i.wrapping_add(1)
            }
            let mut i_0: size_t = 0i32 as size_t;
            while i_0 <
                      (::std::mem::size_of::<[C2RustUnnamed_16; 4]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_16>()
                                                           as libc::c_ulong) {
                let mut reply: *mut xcb_intern_atom_reply_t =
                    xcb_intern_atom_reply((*x11).xcb,
                                          atom[i_0 as usize].cookie,
                                          0 as *mut *mut xcb_generic_error_t);
                if !reply.is_null() {
                    *atom[i_0 as usize].atom = (*reply).atom;
                    free(reply as *mut libc::c_void);
                } else {
                    *atom[i_0 as usize].atom =
                        XCB_ATOM_NONE as libc::c_int as xcb_atom_t
                }
                i_0 = i_0.wrapping_add(1)
            }
            ext = 0 as *const xcb_query_extension_reply_t;
            ext = xcb_get_extension_data((*x11).xcb, &mut xcb_xfixes_id);
            if ext.is_null() || (*ext).present == 0 {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] X11 does not support Xfixes extension\x00"
                             as *const u8 as *const libc::c_char,
                         b"../backend/x11/backend.c\x00" as *const u8 as
                             *const libc::c_char, 229i32);
            } else {
                fixes_cookie =
                    xcb_xfixes_query_version((*x11).xcb, 4i32 as uint32_t,
                                             0i32 as uint32_t);
                fixes_reply =
                    xcb_xfixes_query_version_reply((*x11).xcb, fixes_cookie,
                                                   0 as
                                                       *mut *mut xcb_generic_error_t);
                if fixes_reply.is_null() ||
                       (*fixes_reply).major_version < 4i32 as libc::c_uint {
                    _wlr_log(WLR_ERROR,
                             b"[%s:%d] X11 does not support required Xfixes version\x00"
                                 as *const u8 as *const libc::c_char,
                             b"../backend/x11/backend.c\x00" as *const u8 as
                                 *const libc::c_char, 239i32);
                    free(fixes_reply as *mut libc::c_void);
                } else {
                    free(fixes_reply as *mut libc::c_void);
                    ext =
                        xcb_get_extension_data((*x11).xcb, &mut xcb_input_id);
                    if ext.is_null() || (*ext).present == 0 {
                        _wlr_log(WLR_ERROR,
                                 b"[%s:%d] X11 does not support Xinput extension\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"../backend/x11/backend.c\x00" as *const u8
                                     as *const libc::c_char, 247i32);
                    } else {
                        (*x11).xinput_opcode = (*ext).major_opcode;
                        xi_cookie =
                            xcb_input_xi_query_version((*x11).xcb,
                                                       2i32 as uint16_t,
                                                       0i32 as uint16_t);
                        xi_reply =
                            xcb_input_xi_query_version_reply((*x11).xcb,
                                                             xi_cookie,
                                                             0 as
                                                                 *mut *mut xcb_generic_error_t);
                        if xi_reply.is_null() ||
                               ((*xi_reply).major_version as libc::c_int) <
                                   2i32 {
                            _wlr_log(WLR_ERROR,
                                     b"[%s:%d] X11 does not support required Xinput version\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"../backend/x11/backend.c\x00" as
                                         *const u8 as *const libc::c_char,
                                     258i32);
                            free(xi_reply as *mut libc::c_void);
                        } else {
                            free(xi_reply as *mut libc::c_void);
                            fd = xcb_get_file_descriptor((*x11).xcb);
                            ev = wl_display_get_event_loop(display);
                            events =
                                (WL_EVENT_READABLE as libc::c_int |
                                     WL_EVENT_ERROR as libc::c_int |
                                     WL_EVENT_HANGUP as libc::c_int) as
                                    uint32_t;
                            (*x11).event_source =
                                wl_event_loop_add_fd(ev, fd, events,
                                                     Some(x11_event as
                                                              unsafe extern "C" fn(_:
                                                                                       libc::c_int,
                                                                                   _:
                                                                                       uint32_t,
                                                                                   _:
                                                                                       *mut libc::c_void)
                                                                  ->
                                                                      libc::c_int),
                                                     x11 as
                                                         *mut libc::c_void);
                            if (*x11).event_source.is_null() {
                                _wlr_log(WLR_ERROR,
                                         b"[%s:%d] Could not create event source\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"../backend/x11/backend.c\x00" as
                                             *const u8 as *const libc::c_char,
                                         269i32);
                            } else {
                                wl_event_source_check((*x11).event_source);
                                (*x11).screen =
                                    xcb_setup_roots_iterator(xcb_get_setup((*x11).xcb)).data;
                                if create_renderer_func.is_none() {
                                    create_renderer_func =
                                        Some(wlr_renderer_autocreate as
                                                 unsafe extern "C" fn(_:
                                                                          *mut wlr_egl,
                                                                      _:
                                                                          EGLenum,
                                                                      _:
                                                                          *mut libc::c_void,
                                                                      _:
                                                                          *mut EGLint,
                                                                      _:
                                                                          EGLint)
                                                     -> *mut wlr_renderer)
                                }
                                (*x11).renderer =
                                    create_renderer_func.expect("non-null function pointer")(&mut (*x11).egl,
                                                                                             0x31d5i32
                                                                                                 as
                                                                                                 EGLenum,
                                                                                             (*x11).xlib_conn
                                                                                                 as
                                                                                                 *mut libc::c_void,
                                                                                             config_attribs.as_mut_ptr(),
                                                                                             (*(*x11).screen).root_visual
                                                                                                 as
                                                                                                 EGLint);
                                if (*x11).renderer.is_null() {
                                    _wlr_log(WLR_ERROR,
                                             b"[%s:%d] Failed to create renderer\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"../backend/x11/backend.c\x00"
                                                 as *const u8 as
                                                 *const libc::c_char, 293i32);
                                    wl_event_source_remove((*x11).event_source);
                                } else {
                                    wlr_input_device_init(&mut (*x11).keyboard_dev,
                                                          WLR_INPUT_DEVICE_KEYBOARD,
                                                          &input_device_impl,
                                                          b"X11 keyboard\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          0i32, 0i32);
                                    wlr_keyboard_init(&mut (*x11).keyboard,
                                                      &keyboard_impl);
                                    (*x11).keyboard_dev.c2rust_unnamed.keyboard
                                        = &mut (*x11).keyboard;
                                    (*x11).display_destroy.notify =
                                        Some(handle_display_destroy as
                                                 unsafe extern "C" fn(_:
                                                                          *mut wl_listener,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> ());
                                    wl_display_add_destroy_listener(display,
                                                                    &mut (*x11).display_destroy);
                                    return &mut (*x11).backend
                                }
                            }
                        }
                    }
                }
            }
        }
        XCloseDisplay((*x11).xlib_conn);
    }
    free(x11 as *mut libc::c_void);
    return 0 as *mut wlr_backend;
}
