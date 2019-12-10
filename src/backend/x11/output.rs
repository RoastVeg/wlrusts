use libc;
extern "C" {
    pub type xcb_connection_t;
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type _XDisplay;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_backend_impl;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    pub type wlr_surface;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_tablet_pad_impl;
    pub type wlr_tablet_impl;
    pub type wlr_switch_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn xcb_setup_vendor(R: *const xcb_setup_t) -> *mut libc::c_char;
    #[no_mangle]
    fn xcb_setup_vendor_length(R: *const xcb_setup_t) -> libc::c_int;
    #[no_mangle]
    fn xcb_create_window(c: *mut xcb_connection_t, depth: uint8_t,
                         wid: xcb_window_t, parent: xcb_window_t, x: int16_t,
                         y: int16_t, width: uint16_t, height: uint16_t,
                         border_width: uint16_t, _class: uint16_t,
                         visual: xcb_visualid_t, value_mask: uint32_t,
                         value_list: *const libc::c_void)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_configure_window_checked(c: *mut xcb_connection_t,
                                    window: xcb_window_t,
                                    value_mask: uint16_t,
                                    value_list: *const libc::c_void)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_change_property(c: *mut xcb_connection_t, mode: uint8_t,
                           window: xcb_window_t, property: xcb_atom_t,
                           type_0: xcb_atom_t, format: uint8_t,
                           data_len: uint32_t, data: *const libc::c_void)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    #[no_mangle]
    fn xcb_flush(c: *mut xcb_connection_t) -> libc::c_int;
    #[no_mangle]
    fn xcb_request_check(c: *mut xcb_connection_t, cookie: xcb_void_cookie_t)
     -> *mut xcb_generic_error_t;
    #[no_mangle]
    fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
    #[no_mangle]
    fn xcb_input_xi_select_events(c: *mut xcb_connection_t,
                                  window: xcb_window_t, num_mask: uint16_t,
                                  masks: *const xcb_input_event_mask_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn wl_event_loop_add_timer(loop_0: *mut wl_event_loop,
                               func: wl_event_loop_timer_func_t,
                               data: *mut libc::c_void)
     -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_timer_update(source: *mut wl_event_source,
                                    ms_delay: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    #[no_mangle]
    fn wlr_output_send_present(output: *mut wlr_output,
                               event: *mut wlr_output_event_present);
    #[no_mangle]
    fn wlr_output_send_frame(output: *mut wlr_output);
    #[no_mangle]
    fn wlr_output_update_enabled(output: *mut wlr_output, enabled: bool);
    #[no_mangle]
    fn wlr_output_update_custom_mode(output: *mut wlr_output, width: int32_t,
                                     height: int32_t, refresh: int32_t);
    #[no_mangle]
    fn wlr_output_init(output: *mut wlr_output, backend: *mut wlr_backend,
                       impl_0: *const wlr_output_impl,
                       display: *mut wl_display);
    /* *
 * Returns a surface for the given native window
 * The window must match the remote display the wlr_egl was created with.
 */
    #[no_mangle]
    fn wlr_egl_create_surface(egl: *mut wlr_egl, window: *mut libc::c_void)
     -> EGLSurface;
    #[no_mangle]
    fn wlr_egl_make_current(egl: *mut wlr_egl, surface: EGLSurface,
                            buffer_age: *mut libc::c_int) -> bool;
    #[no_mangle]
    fn wlr_egl_swap_buffers(egl: *mut wlr_egl, surface: EGLSurface,
                            damage: *mut pixman_region32_t) -> bool;
    #[no_mangle]
    fn wlr_egl_destroy_surface(egl: *mut wlr_egl, surface: EGLSurface)
     -> bool;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn wlr_pointer_init(pointer: *mut wlr_pointer,
                        impl_0: *const wlr_pointer_impl);
    #[no_mangle]
    fn wlr_touch_init(touch: *mut wlr_touch, impl_0: *const wlr_touch_impl);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn get_x11_backend_from_backend(wlr_backend: *mut wlr_backend)
     -> *mut wlr_x11_backend;
    #[no_mangle]
    static touch_impl: wlr_touch_impl;
    #[no_mangle]
    static input_device_impl: wlr_input_device_impl;
    #[no_mangle]
    fn wlr_input_device_init(wlr_device: *mut wlr_input_device,
                             type_0: wlr_input_device_type,
                             impl_0: *const wlr_input_device_impl,
                             name: *const libc::c_char, vendor: libc::c_int,
                             product: libc::c_int);
    #[no_mangle]
    static pointer_impl: wlr_pointer_impl;
    #[no_mangle]
    fn update_x11_pointer_position(output: *mut wlr_x11_output,
                                   time: xcb_timestamp_t);
    #[no_mangle]
    fn wlr_input_device_destroy(dev: *mut wlr_input_device);
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
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: libc::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
pub type xcb_event_mask_t = libc::c_uint;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
pub const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
pub const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
pub const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
pub const XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
pub const XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
pub const XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
pub const XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
pub const XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
pub const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
pub const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
pub const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;
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
pub type xcb_window_class_t = libc::c_uint;
pub const XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
pub type xcb_cw_t = libc::c_uint;
pub const XCB_CW_CURSOR: xcb_cw_t = 16384;
pub const XCB_CW_COLORMAP: xcb_cw_t = 8192;
pub const XCB_CW_DONT_PROPAGATE: xcb_cw_t = 4096;
pub const XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
pub const XCB_CW_SAVE_UNDER: xcb_cw_t = 1024;
pub const XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 512;
pub const XCB_CW_BACKING_PIXEL: xcb_cw_t = 256;
pub const XCB_CW_BACKING_PLANES: xcb_cw_t = 128;
pub const XCB_CW_BACKING_STORE: xcb_cw_t = 64;
pub const XCB_CW_WIN_GRAVITY: xcb_cw_t = 32;
pub const XCB_CW_BIT_GRAVITY: xcb_cw_t = 16;
pub const XCB_CW_BORDER_PIXEL: xcb_cw_t = 8;
pub const XCB_CW_BORDER_PIXMAP: xcb_cw_t = 4;
pub const XCB_CW_BACK_PIXEL: xcb_cw_t = 2;
pub const XCB_CW_BACK_PIXMAP: xcb_cw_t = 1;
pub type xcb_config_window_t = libc::c_uint;
pub const XCB_CONFIG_WINDOW_STACK_MODE: xcb_config_window_t = 64;
pub const XCB_CONFIG_WINDOW_SIBLING: xcb_config_window_t = 32;
pub const XCB_CONFIG_WINDOW_BORDER_WIDTH: xcb_config_window_t = 16;
pub const XCB_CONFIG_WINDOW_HEIGHT: xcb_config_window_t = 8;
pub const XCB_CONFIG_WINDOW_WIDTH: xcb_config_window_t = 4;
pub const XCB_CONFIG_WINDOW_Y: xcb_config_window_t = 2;
pub const XCB_CONFIG_WINDOW_X: xcb_config_window_t = 1;
pub type xcb_prop_mode_t = libc::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
pub type xcb_input_device_id_t = uint16_t;
pub type xcb_input_device_t = libc::c_uint;
pub const XCB_INPUT_DEVICE_ALL_MASTER: xcb_input_device_t = 1;
pub const XCB_INPUT_DEVICE_ALL: xcb_input_device_t = 0;
pub type xcb_input_xi_event_mask_t = libc::c_uint;
pub const XCB_INPUT_XI_EVENT_MASK_BARRIER_LEAVE: xcb_input_xi_event_mask_t =
    67108864;
pub const XCB_INPUT_XI_EVENT_MASK_BARRIER_HIT: xcb_input_xi_event_mask_t =
    33554432;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_END: xcb_input_xi_event_mask_t =
    16777216;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_UPDATE: xcb_input_xi_event_mask_t
          =
    8388608;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_BEGIN: xcb_input_xi_event_mask_t =
    4194304;
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_OWNERSHIP: xcb_input_xi_event_mask_t =
    2097152;
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_END: xcb_input_xi_event_mask_t =
    1048576;
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_UPDATE: xcb_input_xi_event_mask_t =
    524288;
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_BEGIN: xcb_input_xi_event_mask_t =
    262144;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_MOTION: xcb_input_xi_event_mask_t =
    131072;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_BUTTON_RELEASE:
          xcb_input_xi_event_mask_t =
    65536;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_BUTTON_PRESS: xcb_input_xi_event_mask_t
          =
    32768;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_KEY_RELEASE: xcb_input_xi_event_mask_t =
    16384;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_KEY_PRESS: xcb_input_xi_event_mask_t =
    8192;
pub const XCB_INPUT_XI_EVENT_MASK_PROPERTY: xcb_input_xi_event_mask_t = 4096;
pub const XCB_INPUT_XI_EVENT_MASK_HIERARCHY: xcb_input_xi_event_mask_t = 2048;
pub const XCB_INPUT_XI_EVENT_MASK_FOCUS_OUT: xcb_input_xi_event_mask_t = 1024;
pub const XCB_INPUT_XI_EVENT_MASK_FOCUS_IN: xcb_input_xi_event_mask_t = 512;
pub const XCB_INPUT_XI_EVENT_MASK_LEAVE: xcb_input_xi_event_mask_t = 256;
pub const XCB_INPUT_XI_EVENT_MASK_ENTER: xcb_input_xi_event_mask_t = 128;
pub const XCB_INPUT_XI_EVENT_MASK_MOTION: xcb_input_xi_event_mask_t = 64;
pub const XCB_INPUT_XI_EVENT_MASK_BUTTON_RELEASE: xcb_input_xi_event_mask_t =
    32;
pub const XCB_INPUT_XI_EVENT_MASK_BUTTON_PRESS: xcb_input_xi_event_mask_t =
    16;
pub const XCB_INPUT_XI_EVENT_MASK_KEY_RELEASE: xcb_input_xi_event_mask_t = 8;
pub const XCB_INPUT_XI_EVENT_MASK_KEY_PRESS: xcb_input_xi_event_mask_t = 4;
pub const XCB_INPUT_XI_EVENT_MASK_DEVICE_CHANGED: xcb_input_xi_event_mask_t =
    2;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xcb_input_event_mask_t {
    pub deviceid: xcb_input_device_id_t,
    pub mask_len: uint16_t,
}
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
pub type wl_event_loop_timer_func_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
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
pub type Display = _XDisplay;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
/*
 * 32 bit regions
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
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
    pub exts: C2RustUnnamed,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub bind_wayland_display_wl: bool,
    pub buffer_age_ext: bool,
    pub image_base_khr: bool,
    pub image_dma_buf_export_mesa: bool,
    pub image_dmabuf_import_ext: bool,
    pub image_dmabuf_import_modifiers_ext: bool,
    pub swap_buffers_with_damage_ext: bool,
    pub swap_buffers_with_damage_khr: bool,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_1,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub destroy: wl_signal,
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
    pub events: C2RustUnnamed_2,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub events: C2RustUnnamed_3,
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
pub struct C2RustUnnamed_3 {
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
pub type wlr_output_state_field = libc::c_uint;
pub const WLR_OUTPUT_STATE_DAMAGE: wlr_output_state_field = 2;
pub const WLR_OUTPUT_STATE_BUFFER: wlr_output_state_field = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_event_present {
    pub output: *mut wlr_output,
    pub commit_seq: uint32_t,
    pub when: *mut timespec,
    pub seq: libc::c_uint,
    pub refresh: libc::c_int,
    pub flags: uint32_t,
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
pub struct wlr_switch {
    pub impl_0: *mut wlr_switch_impl,
    pub events: C2RustUnnamed_11,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub toggle: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
    pub events: C2RustUnnamed_12,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_pointer) -> ()>,
}
/* Note: these are circular dependencies */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
}
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
// 60 Hz
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
    pub atoms: C2RustUnnamed_13,
    pub time: xcb_timestamp_t,
    pub xinput_opcode: uint8_t,
    pub display_destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub wm_protocols: xcb_atom_t,
    pub wm_delete_window: xcb_atom_t,
    pub net_wm_name: xcb_atom_t,
    pub utf8_string: xcb_atom_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub head: xcb_input_event_mask_t,
    pub mask: xcb_input_xi_event_mask_t,
}
unsafe extern "C" fn signal_frame(mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut output: *mut wlr_x11_output = data as *mut wlr_x11_output;
    wlr_output_send_frame(&mut (*output).wlr_output);
    wl_event_source_timer_update((*output).frame_timer,
                                 (*output).frame_delay);
    return 0i32;
}
unsafe extern "C" fn parse_xcb_setup(mut output: *mut wlr_output,
                                     mut xcb: *mut xcb_connection_t) {
    let mut xcb_setup: *const xcb_setup_t = xcb_get_setup(xcb);
    snprintf((*output).make.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong,
             b"%.*s\x00" as *const u8 as *const libc::c_char,
             xcb_setup_vendor_length(xcb_setup), xcb_setup_vendor(xcb_setup));
    snprintf((*output).model.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
             b"%u.%u\x00" as *const u8 as *const libc::c_char,
             (*xcb_setup).protocol_major_version as libc::c_int,
             (*xcb_setup).protocol_minor_version as libc::c_int);
}
unsafe extern "C" fn get_x11_output_from_output(mut wlr_output:
                                                    *mut wlr_output)
 -> *mut wlr_x11_output {
    if wlr_output_is_x11(wlr_output) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_output_is_x11(wlr_output)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/x11/output.c\x00" as *const u8 as
                          *const libc::c_char, 39i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 71],
                                                &[libc::c_char; 71]>(b"struct wlr_x11_output *get_x11_output_from_output(struct wlr_output *)\x00")).as_ptr());
    };
    return wlr_output as *mut wlr_x11_output;
}
unsafe extern "C" fn output_set_refresh(mut wlr_output: *mut wlr_output,
                                        mut refresh: int32_t) {
    let mut output: *mut wlr_x11_output =
        get_x11_output_from_output(wlr_output);
    if refresh <= 0i32 { refresh = 60i32 * 1000i32 }
    wlr_output_update_custom_mode(&mut (*output).wlr_output,
                                  (*wlr_output).width, (*wlr_output).height,
                                  refresh);
    (*output).frame_delay = 1000000i32 / refresh;
}
unsafe extern "C" fn output_set_custom_mode(mut wlr_output: *mut wlr_output,
                                            mut width: int32_t,
                                            mut height: int32_t,
                                            mut refresh: int32_t) -> bool {
    let mut output: *mut wlr_x11_output =
        get_x11_output_from_output(wlr_output);
    let mut x11: *mut wlr_x11_backend = (*output).x11;
    output_set_refresh(&mut (*output).wlr_output, refresh);
    let values: [uint32_t; 2] = [width as uint32_t, height as uint32_t];
    let mut cookie: xcb_void_cookie_t =
        xcb_configure_window_checked((*x11).xcb, (*output).win,
                                     (XCB_CONFIG_WINDOW_WIDTH as libc::c_int |
                                          XCB_CONFIG_WINDOW_HEIGHT as
                                              libc::c_int) as uint16_t,
                                     values.as_ptr() as *const libc::c_void);
    let mut error: *mut xcb_generic_error_t = 0 as *mut xcb_generic_error_t;
    error = xcb_request_check((*x11).xcb, cookie);
    if !error.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not set window size to %dx%d\n\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/x11/output.c\x00" as *const u8 as
                     *const libc::c_char, 71i32, width, height);
        free(error as *mut libc::c_void);
        return 0i32 != 0
    }
    return 1i32 != 0;
}
unsafe extern "C" fn output_destroy(mut wlr_output: *mut wlr_output) {
    let mut output: *mut wlr_x11_output =
        get_x11_output_from_output(wlr_output);
    let mut x11: *mut wlr_x11_backend = (*output).x11;
    wlr_input_device_destroy(&mut (*output).pointer_dev);
    wlr_input_device_destroy(&mut (*output).touch_dev);
    wl_list_remove(&mut (*output).link);
    wl_event_source_remove((*output).frame_timer);
    wlr_egl_destroy_surface(&mut (*x11).egl, (*output).surf);
    xcb_destroy_window((*x11).xcb, (*output).win);
    xcb_flush((*x11).xcb);
    free(output as *mut libc::c_void);
}
unsafe extern "C" fn output_attach_render(mut wlr_output: *mut wlr_output,
                                          mut buffer_age: *mut libc::c_int)
 -> bool {
    let mut output: *mut wlr_x11_output =
        get_x11_output_from_output(wlr_output);
    let mut x11: *mut wlr_x11_backend = (*output).x11;
    return wlr_egl_make_current(&mut (*x11).egl, (*output).surf, buffer_age);
}
unsafe extern "C" fn output_commit(mut wlr_output: *mut wlr_output) -> bool {
    let mut output: *mut wlr_x11_output =
        get_x11_output_from_output(wlr_output);
    let mut x11: *mut wlr_x11_backend = (*output).x11;
    let mut damage: *mut pixman_region32_t = 0 as *mut pixman_region32_t;
    if (*wlr_output).pending.committed &
           WLR_OUTPUT_STATE_DAMAGE as libc::c_int as libc::c_uint != 0 {
        damage = &mut (*wlr_output).pending.damage
    }
    if !wlr_egl_swap_buffers(&mut (*x11).egl, (*output).surf, damage) {
        return 0i32 != 0
    }
    wlr_output_send_present(wlr_output, 0 as *mut wlr_output_event_present);
    return 1i32 != 0;
}
static mut output_impl: wlr_output_impl =
    unsafe {
        {
            let mut init =
                wlr_output_impl{enable: None,
                                set_mode: None,
                                set_custom_mode:
                                    Some(output_set_custom_mode as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _: int32_t,
                                                                  _: int32_t,
                                                                  _: int32_t)
                                                 -> bool),
                                set_cursor: None,
                                move_cursor: None,
                                destroy:
                                    Some(output_destroy as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output)
                                                 -> ()),
                                attach_render:
                                    Some(output_attach_render as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _:
                                                                      *mut libc::c_int)
                                                 -> bool),
                                commit:
                                    Some(output_commit as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output)
                                                 -> bool),
                                set_gamma: None,
                                get_gamma_size: None,
                                export_dmabuf: None,
                                schedule_frame: None,
                                attach_buffer: None,};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn wlr_x11_output_create(mut backend: *mut wlr_backend)
 -> *mut wlr_output {
    let mut x11: *mut wlr_x11_backend = get_x11_backend_from_backend(backend);
    if !(*x11).started {
        (*x11).requested_outputs = (*x11).requested_outputs.wrapping_add(1);
        return 0 as *mut wlr_output
    }
    let mut output: *mut wlr_x11_output =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_x11_output>() as libc::c_ulong) as
            *mut wlr_x11_output;
    if output.is_null() { return 0 as *mut wlr_output }
    (*output).x11 = x11;
    let mut wlr_output: *mut wlr_output = &mut (*output).wlr_output;
    wlr_output_init(wlr_output, &mut (*x11).backend, &output_impl,
                    (*x11).wl_display);
    (*wlr_output).width = 1024i32;
    (*wlr_output).height = 768i32;
    output_set_refresh(&mut (*output).wlr_output, 0i32);
    (*x11).last_output_num = (*x11).last_output_num.wrapping_add(1);
    snprintf((*wlr_output).name.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
             b"X11-%zd\x00" as *const u8 as *const libc::c_char,
             (*x11).last_output_num);
    parse_xcb_setup(wlr_output, (*x11).xcb);
    let mut mask: uint32_t = XCB_CW_EVENT_MASK as libc::c_int as uint32_t;
    let mut values: [uint32_t; 1] =
        [(XCB_EVENT_MASK_EXPOSURE as libc::c_int |
              XCB_EVENT_MASK_STRUCTURE_NOTIFY as libc::c_int) as uint32_t];
    (*output).win = xcb_generate_id((*x11).xcb);
    xcb_create_window((*x11).xcb, 0i64 as uint8_t, (*output).win,
                      (*(*x11).screen).root, 0i32 as int16_t, 0i32 as int16_t,
                      (*wlr_output).width as uint16_t,
                      (*wlr_output).height as uint16_t, 1i32 as uint16_t,
                      XCB_WINDOW_CLASS_INPUT_OUTPUT as libc::c_int as
                          uint16_t, (*(*x11).screen).root_visual, mask,
                      values.as_mut_ptr() as *const libc::c_void);
    let mut xinput_mask: C2RustUnnamed_14 =
        {
            let mut init =
                C2RustUnnamed_14{head:
                                     {
                                         let mut init =
                                             xcb_input_event_mask_t{deviceid:
                                                                        XCB_INPUT_DEVICE_ALL_MASTER
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            xcb_input_device_id_t,
                                                                    mask_len:
                                                                        1i32
                                                                            as
                                                                            uint16_t,};
                                         init
                                     },
                                 mask:
                                     (XCB_INPUT_XI_EVENT_MASK_KEY_PRESS as
                                          libc::c_int |
                                          XCB_INPUT_XI_EVENT_MASK_KEY_RELEASE
                                              as libc::c_int |
                                          XCB_INPUT_XI_EVENT_MASK_BUTTON_PRESS
                                              as libc::c_int |
                                          XCB_INPUT_XI_EVENT_MASK_BUTTON_RELEASE
                                              as libc::c_int |
                                          XCB_INPUT_XI_EVENT_MASK_MOTION as
                                              libc::c_int |
                                          XCB_INPUT_XI_EVENT_MASK_ENTER as
                                              libc::c_int |
                                          XCB_INPUT_XI_EVENT_MASK_LEAVE as
                                              libc::c_int |
                                          XCB_INPUT_XI_EVENT_MASK_TOUCH_BEGIN
                                              as libc::c_int |
                                          XCB_INPUT_XI_EVENT_MASK_TOUCH_END as
                                              libc::c_int |
                                          XCB_INPUT_XI_EVENT_MASK_TOUCH_UPDATE
                                              as libc::c_int) as
                                         xcb_input_xi_event_mask_t,};
            init
        };
    xcb_input_xi_select_events((*x11).xcb, (*output).win, 1i32 as uint16_t,
                               &mut xinput_mask.head);
    (*output).surf =
        wlr_egl_create_surface(&mut (*x11).egl,
                               &mut (*output).win as *mut xcb_window_t as
                                   *mut libc::c_void);
    if (*output).surf.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create EGL surface\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/x11/output.c\x00" as *const u8 as
                     *const libc::c_char, 181i32);
        free(output as *mut libc::c_void);
        return 0 as *mut wlr_output
    }
    xcb_change_property((*x11).xcb,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*output).win, (*x11).atoms.wm_protocols,
                        XCB_ATOM_ATOM as libc::c_int as xcb_atom_t,
                        32i32 as uint8_t, 1i32 as uint32_t,
                        &mut (*x11).atoms.wm_delete_window as *mut xcb_atom_t
                            as *const libc::c_void);
    wlr_x11_output_set_title(wlr_output, 0 as *const libc::c_char);
    xcb_map_window((*x11).xcb, (*output).win);
    xcb_flush((*x11).xcb);
    let mut ev: *mut wl_event_loop =
        wl_display_get_event_loop((*x11).wl_display);
    (*output).frame_timer =
        wl_event_loop_add_timer(ev,
                                Some(signal_frame as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_void)
                                             -> libc::c_int),
                                output as *mut libc::c_void);
    wl_list_insert(&mut (*x11).outputs, &mut (*output).link);
    wl_event_source_timer_update((*output).frame_timer,
                                 (*output).frame_delay);
    wlr_output_update_enabled(wlr_output, 1i32 != 0);
    wlr_input_device_init(&mut (*output).pointer_dev,
                          WLR_INPUT_DEVICE_POINTER, &input_device_impl,
                          b"X11 pointer\x00" as *const u8 as
                              *const libc::c_char, 0i32, 0i32);
    wlr_pointer_init(&mut (*output).pointer, &pointer_impl);
    (*output).pointer_dev.c2rust_unnamed.pointer = &mut (*output).pointer;
    (*output).pointer_dev.output_name =
        strdup((*wlr_output).name.as_mut_ptr());
    wlr_input_device_init(&mut (*output).touch_dev, WLR_INPUT_DEVICE_TOUCH,
                          &input_device_impl,
                          b"X11 touch\x00" as *const u8 as
                              *const libc::c_char, 0i32, 0i32);
    wlr_touch_init(&mut (*output).touch, &touch_impl);
    (*output).touch_dev.c2rust_unnamed.touch = &mut (*output).touch;
    (*output).touch_dev.output_name = strdup((*wlr_output).name.as_mut_ptr());
    wl_list_init(&mut (*output).touchpoints);
    wlr_signal_emit_safe(&mut (*x11).backend.events.new_output,
                         wlr_output as *mut libc::c_void);
    wlr_signal_emit_safe(&mut (*x11).backend.events.new_input,
                         &mut (*output).pointer_dev as *mut wlr_input_device
                             as *mut libc::c_void);
    wlr_signal_emit_safe(&mut (*x11).backend.events.new_input,
                         &mut (*output).touch_dev as *mut wlr_input_device as
                             *mut libc::c_void);
    return wlr_output;
}
#[no_mangle]
pub unsafe extern "C" fn handle_x11_configure_notify(mut output:
                                                         *mut wlr_x11_output,
                                                     mut ev:
                                                         *mut xcb_configure_notify_event_t) {
    // ignore events that set an invalid size:
    if (*ev).width as libc::c_int > 0i32 && (*ev).height as libc::c_int > 0i32
       {
        wlr_output_update_custom_mode(&mut (*output).wlr_output,
                                      (*ev).width as int32_t,
                                      (*ev).height as int32_t,
                                      (*output).wlr_output.refresh);
        // Move the pointer to its new location
        update_x11_pointer_position(output, (*(*output).x11).time);
    } else {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Ignoring X11 configure event for height=%d, width=%d\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/x11/output.c\x00" as *const u8 as
                     *const libc::c_char, 235i32, (*ev).width as libc::c_int,
                 (*ev).height as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_is_x11(mut wlr_output: *mut wlr_output)
 -> bool {
    return (*wlr_output).impl_0 == &output_impl as *const wlr_output_impl;
}
/* *
 * Creates a new wlr_x11_backend. This backend will be created with no outputs;
 * you must use wlr_x11_output_create to add them.
 *
 * The `x11_display` argument is the name of the X Display socket. Set
 * to NULL for the default behaviour of XOpenDisplay.
 */
/* *
 * Adds a new output to this backend. You may remove outputs by destroying them.
 * Note that if called before initializing the backend, this will return NULL
 * and your outputs will be created during initialization (and given to you via
 * the output_add signal).
 */
/* *
 * True if the given backend is a wlr_x11_backend.
 */
/* *
 * True if the given input device is a wlr_x11_input_device.
 */
/* *
 * True if the given output is a wlr_x11_output.
 */
/* *
 * Sets the title of a wlr_output which is an X11 window.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_x11_output_set_title(mut output: *mut wlr_output,
                                                  mut title:
                                                      *const libc::c_char) {
    let mut x11_output: *mut wlr_x11_output =
        get_x11_output_from_output(output);
    let mut wl_title: [libc::c_char; 32] = [0; 32];
    if title.is_null() {
        if snprintf(wl_title.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 32]>() as
                        libc::c_ulong,
                    b"wlroots - %s\x00" as *const u8 as *const libc::c_char,
                    (*output).name.as_mut_ptr()) <= 0i32 {
            return
        }
        title = wl_title.as_mut_ptr()
    }
    xcb_change_property((*(*x11_output).x11).xcb,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*x11_output).win,
                        (*(*x11_output).x11).atoms.net_wm_name,
                        (*(*x11_output).x11).atoms.utf8_string,
                        8i32 as uint8_t, strlen(title) as uint32_t,
                        title as *const libc::c_void);
}
