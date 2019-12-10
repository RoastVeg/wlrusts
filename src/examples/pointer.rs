use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_renderer_impl;
    pub type wlr_backend_impl;
    pub type wlr_texture_impl;
    pub type xkb_context;
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
    pub type wlr_tablet_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* Note: these are circular dependencies */
    pub type wlr_input_device_impl;
    pub type wlr_surface;
    pub type wlr_output_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_output_layout_state;
    pub type wlr_cursor_state;
    pub type compositor_state;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn wl_display_create() -> *mut wl_display;
    #[no_mangle]
    fn wl_display_destroy(display: *mut wl_display);
    #[no_mangle]
    fn wl_display_terminate(display: *mut wl_display);
    #[no_mangle]
    fn wl_display_run(display: *mut wl_display);
    #[no_mangle]
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wlr_backend_get_renderer(backend: *mut wlr_backend)
     -> *mut wlr_renderer;
    #[no_mangle]
    fn wlr_backend_destroy(backend: *mut wlr_backend);
    #[no_mangle]
    fn wlr_backend_start(backend: *mut wlr_backend) -> bool;
    #[no_mangle]
    fn wlr_backend_autocreate(display: *mut wl_display,
                              create_renderer_func:
                                  wlr_renderer_create_func_t)
     -> *mut wlr_backend;
    #[no_mangle]
    fn wlr_renderer_clear(r: *mut wlr_renderer, color: *const libc::c_float);
    #[no_mangle]
    fn wlr_renderer_end(r: *mut wlr_renderer);
    #[no_mangle]
    fn wlr_renderer_begin(r: *mut wlr_renderer, width: libc::c_int,
                          height: libc::c_int);
    /* *
 * Creates a wlr_output_layout, which can be used to describing outputs in
 * physical space relative to one another, and perform various useful operations
 * on that state.
 */
    #[no_mangle]
    fn wlr_output_layout_create() -> *mut wlr_output_layout;
    #[no_mangle]
    fn wlr_output_layout_destroy(layout: *mut wlr_output_layout);
    #[no_mangle]
    fn wlr_output_layout_remove(layout: *mut wlr_output_layout,
                                output: *mut wlr_output);
    #[no_mangle]
    fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
    #[no_mangle]
    fn xkb_context_unref(context: *mut xkb_context);
    #[no_mangle]
    fn xkb_keymap_new_from_names(context: *mut xkb_context,
                                 names: *const xkb_rule_names,
                                 flags: xkb_keymap_compile_flags)
     -> *mut xkb_keymap;
    #[no_mangle]
    fn xkb_keymap_unref(keymap: *mut xkb_keymap);
    #[no_mangle]
    fn xkb_state_key_get_syms(state: *mut xkb_state, key: xkb_keycode_t,
                              syms_out: *mut *const xkb_keysym_t)
     -> libc::c_int;
    #[no_mangle]
    fn wlr_keyboard_set_keymap(kb: *mut wlr_keyboard,
                               keymap: *mut xkb_keymap);
    /* *
 * Sets the output mode. Enables the output if it's currently disabled.
 */
    #[no_mangle]
    fn wlr_output_set_mode(output: *mut wlr_output,
                           mode: *mut wlr_output_mode) -> bool;
    /* *
 * Attach the renderer's buffer to the output. Compositors must call this
 * function before rendering. After they are done rendering, they should call
 * `wlr_output_commit` to submit the new frame.
 *
 * If non-NULL, `buffer_age` is set to the drawing buffer age in number of
 * frames or -1 if unknown. This is useful for damage tracking.
 */
    #[no_mangle]
    fn wlr_output_attach_render(output: *mut wlr_output,
                                buffer_age: *mut libc::c_int) -> bool;
    /* *
 * Commit the pending output state. If `wlr_output_attach_render` has been
 * called, the pending frame will be submitted for display.
 *
 * This function schedules a `frame` event.
 */
    #[no_mangle]
    fn wlr_output_commit(output: *mut wlr_output) -> bool;
    /* *
 * Renders software cursors. This is a utility function that can be called when
 * compositors render.
 */
    #[no_mangle]
    fn wlr_output_render_software_cursors(output: *mut wlr_output,
                                          damage: *mut pixman_region32_t);
    /* *
* Add an auto configured output to the layout. This will place the output in a
* sensible location in the layout. The coordinates of the output in the layout
* may adjust dynamically when the layout changes. If the output is already in
* the layout, it will become auto configured. If the position of the output is
* set such as with `wlr_output_layout_move()`, the output will become manually
* configured.
*/
    #[no_mangle]
    fn wlr_output_layout_add_auto(layout: *mut wlr_output_layout,
                                  output: *mut wlr_output);
    #[no_mangle]
    fn wlr_cursor_create() -> *mut wlr_cursor;
    #[no_mangle]
    fn wlr_cursor_destroy(cur: *mut wlr_cursor);
    #[no_mangle]
    fn wlr_cursor_warp_absolute(cur: *mut wlr_cursor,
                                dev: *mut wlr_input_device, x: libc::c_double,
                                y: libc::c_double);
    #[no_mangle]
    fn wlr_cursor_move(cur: *mut wlr_cursor, dev: *mut wlr_input_device,
                       delta_x: libc::c_double, delta_y: libc::c_double);
    #[no_mangle]
    fn wlr_cursor_attach_input_device(cur: *mut wlr_cursor,
                                      dev: *mut wlr_input_device);
    #[no_mangle]
    fn wlr_cursor_attach_output_layout(cur: *mut wlr_cursor,
                                       l: *mut wlr_output_layout);
    /* *
 * Creates a new XCursor manager with the given xcursor theme name and base size
 * (for use when scale=1).
 */
    #[no_mangle]
    fn wlr_xcursor_manager_create(name: *const libc::c_char, size: uint32_t)
     -> *mut wlr_xcursor_manager;
    #[no_mangle]
    fn wlr_xcursor_manager_destroy(manager: *mut wlr_xcursor_manager);
    /* *
 * Ensures an xcursor theme at the given scale factor is loaded in the manager.
 */
    #[no_mangle]
    fn wlr_xcursor_manager_load(manager: *mut wlr_xcursor_manager,
                                scale: libc::c_float) -> libc::c_int;
    /* *
 * Set a wlr_cursor's cursor image to the specified cursor name for all scale
 * factors. wlr_cursor will take over from this point and ensure the correct
 * cursor is used on each output, assuming a wlr_output_layout is attached to
 * it.
 */
    #[no_mangle]
    fn wlr_xcursor_manager_set_cursor_image(manager: *mut wlr_xcursor_manager,
                                            name: *const libc::c_char,
                                            cursor: *mut wlr_cursor);
    // Will log all messages less than or equal to `verbosity`
// If `callback` is NULL, wlr will use its default logger.
// The function can be called multiple times to update the verbosity or
// callback function.
    #[no_mangle]
    fn wlr_log_init(verbosity: wlr_log_importance, callback: wlr_log_func_t);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* * Raised when destroyed, passed the wlr_backend reference */
/* * Raised when new inputs are added, passed the wlr_input_device */
/* * Raised when new outputs are added, passed the wlr_output */
pub type wlr_renderer_create_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wlr_egl, _: EGLenum,
                                _: *mut libc::c_void, _: *mut EGLint,
                                _: EGLint) -> *mut wlr_renderer>;
pub type EGLint = khronos_int32_t;
pub type khronos_int32_t = int32_t;
pub type EGLenum = libc::c_uint;
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
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub cap: size_t,
    pub formats: *mut *mut wlr_drm_format,
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
pub type EGLContext = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLDisplay = *mut libc::c_void;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}
pub type wlr_log_func_t
    =
    Option<unsafe extern "C" fn(_: wlr_log_importance, _: *const libc::c_char,
                                _: ::std::ffi::VaList) -> ()>;
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
pub type pixman_region32_t = pixman_region32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_box32_t = pixman_box32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
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
pub struct wlr_texture {
    pub impl_0: *const wlr_texture_impl,
}
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
pub type xkb_keycode_t = uint32_t;
pub type xkb_keysym_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct xkb_rule_names {
    pub rules: *const libc::c_char,
    pub model: *const libc::c_char,
    pub layout: *const libc::c_char,
    pub variant: *const libc::c_char,
    pub options: *const libc::c_char,
}
pub type xkb_context_flags = libc::c_uint;
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
pub type xkb_keymap_compile_flags = libc::c_uint;
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
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
pub struct wlr_event_pointer_motion {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub delta_x: libc::c_double,
    pub delta_y: libc::c_double,
    pub unaccel_dx: libc::c_double,
    pub unaccel_dy: libc::c_double,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_motion_absolute {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_button {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub button: uint32_t,
    pub state: wlr_button_state,
}
pub type wlr_axis_source = libc::c_uint;
pub const WLR_AXIS_SOURCE_WHEEL_TILT: wlr_axis_source = 3;
pub const WLR_AXIS_SOURCE_CONTINUOUS: wlr_axis_source = 2;
pub const WLR_AXIS_SOURCE_FINGER: wlr_axis_source = 1;
pub const WLR_AXIS_SOURCE_WHEEL: wlr_axis_source = 0;
pub type wlr_axis_orientation = libc::c_uint;
pub const WLR_AXIS_ORIENTATION_HORIZONTAL: wlr_axis_orientation = 1;
pub const WLR_AXIS_ORIENTATION_VERTICAL: wlr_axis_orientation = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_pointer_axis {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub source: wlr_axis_source,
    pub orientation: wlr_axis_orientation,
    pub delta: libc::c_double,
    pub delta_discrete: int32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_touch_down {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_touch_up {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_touch_motion {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type wlr_tablet_tool_type = libc::c_uint;
pub const WLR_TABLET_TOOL_TYPE_TOTEM: wlr_tablet_tool_type = 8;
pub const WLR_TABLET_TOOL_TYPE_LENS: wlr_tablet_tool_type = 7;
pub const WLR_TABLET_TOOL_TYPE_MOUSE: wlr_tablet_tool_type = 6;
pub const WLR_TABLET_TOOL_TYPE_AIRBRUSH: wlr_tablet_tool_type = 5;
pub const WLR_TABLET_TOOL_TYPE_PENCIL: wlr_tablet_tool_type = 4;
pub const WLR_TABLET_TOOL_TYPE_BRUSH: wlr_tablet_tool_type = 3;
pub const WLR_TABLET_TOOL_TYPE_ERASER: wlr_tablet_tool_type = 2;
pub const WLR_TABLET_TOOL_TYPE_PEN: wlr_tablet_tool_type = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet_tool {
    pub type_0: wlr_tablet_tool_type,
    pub hardware_serial: uint64_t,
    pub hardware_wacom: uint64_t,
    pub tilt: bool,
    pub pressure: bool,
    pub distance: bool,
    pub rotation: bool,
    pub slider: bool,
    pub wheel: bool,
    pub events: C2RustUnnamed_11,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub destroy: wl_signal,
}
// char *
pub type wlr_tablet_tool_axes = libc::c_uint;
pub const WLR_TABLET_TOOL_AXIS_WHEEL: wlr_tablet_tool_axes = 256;
pub const WLR_TABLET_TOOL_AXIS_SLIDER: wlr_tablet_tool_axes = 128;
pub const WLR_TABLET_TOOL_AXIS_ROTATION: wlr_tablet_tool_axes = 64;
pub const WLR_TABLET_TOOL_AXIS_TILT_Y: wlr_tablet_tool_axes = 32;
pub const WLR_TABLET_TOOL_AXIS_TILT_X: wlr_tablet_tool_axes = 16;
pub const WLR_TABLET_TOOL_AXIS_PRESSURE: wlr_tablet_tool_axes = 8;
pub const WLR_TABLET_TOOL_AXIS_DISTANCE: wlr_tablet_tool_axes = 4;
pub const WLR_TABLET_TOOL_AXIS_Y: wlr_tablet_tool_axes = 2;
pub const WLR_TABLET_TOOL_AXIS_X: wlr_tablet_tool_axes = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_event_tablet_tool_axis {
    pub device: *mut wlr_input_device,
    pub tool: *mut wlr_tablet_tool,
    pub time_msec: uint32_t,
    pub updated_axes: uint32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub dx: libc::c_double,
    pub dy: libc::c_double,
    pub pressure: libc::c_double,
    pub distance: libc::c_double,
    pub tilt_x: libc::c_double,
    pub tilt_y: libc::c_double,
    pub rotation: libc::c_double,
    pub slider: libc::c_double,
    pub wheel_delta: libc::c_double,
}
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
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
    pub events: C2RustUnnamed_12,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub destroy: wl_signal,
}
/* *
 * A compositor output region. This typically corresponds to a monitor that
 * displays part of the compositor space.
 *
 * The `frame` event will be emitted when it is a good time for the compositor
 * to submit a new frame.
 *
 * To render a new frame, compositors should call `wlr_output_attach_render`,
 * render and call `wlr_output_commit`. No rendering should happen outside a
 * `frame` event handler or before `wlr_output_attach_render`.
 */
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
    pub events: C2RustUnnamed_13,
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
pub struct C2RustUnnamed_13 {
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
/* *
 * Holds the double-buffered output state.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub damage: pixman_region32_t,
    pub buffer_type: wlr_output_state_buffer_type,
    pub buffer: *mut wlr_buffer,
    // if WLR_OUTPUT_STATE_BUFFER_SCANOUT
}
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub state: *mut wlr_output_layout_state,
    pub events: C2RustUnnamed_14,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_cursor {
    pub state: *mut wlr_cursor_state,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub events: C2RustUnnamed_15,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_15 {
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
    pub touch_up: wl_signal,
    pub touch_down: wl_signal,
    pub touch_motion: wl_signal,
    pub touch_cancel: wl_signal,
    pub tablet_tool_axis: wl_signal,
    pub tablet_tool_proximity: wl_signal,
    pub tablet_tool_tip: wl_signal,
    pub tablet_tool_button: wl_signal,
}
/* *
 * wlr_xcursor_manager dynamically loads xcursor themes at sizes necessary for
 * use on outputs at arbitrary scale factors. You should call
 * wlr_xcursor_manager_load for each output you will show your cursor on, with
 * the scale factor parameter set to that output's scale factor.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xcursor_manager {
    pub name: *mut libc::c_char,
    pub size: uint32_t,
    pub scaled_themes: wl_list,
    // wlr_xcursor_manager_theme::link
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_state {
    pub display: *mut wl_display,
    pub compositor: *mut compositor_state,
    pub xcursor_manager: *mut wlr_xcursor_manager,
    pub cursor: *mut wlr_cursor,
    pub cur_x: libc::c_double,
    pub cur_y: libc::c_double,
    pub default_color: [libc::c_float; 4],
    pub clear_color: [libc::c_float; 4],
    pub layout: *mut wlr_output_layout,
    pub devices: wl_list,
    pub last_frame: timespec,
    pub new_output: wl_listener,
    pub new_input: wl_listener,
    pub cursor_motion: wl_listener,
    pub cursor_motion_absolute: wl_listener,
    pub cursor_button: wl_listener,
    pub cursor_axis: wl_listener,
    pub touch_motion: wl_listener,
    pub touch_up: wl_listener,
    pub touch_down: wl_listener,
    pub touch_cancel: wl_listener,
    pub touch_points: wl_list,
    pub tablet_tool_axis: wl_listener,
    pub tablet_tool_proxmity: wl_listener,
    pub tablet_tool_tip: wl_listener,
    pub tablet_tool_button: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct touch_point {
    pub touch_id: int32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_output {
    pub state: *mut sample_state,
    pub output: *mut wlr_output,
    pub frame: wl_listener,
    pub destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_keyboard {
    pub state: *mut sample_state,
    pub device: *mut wlr_input_device,
    pub key: wl_listener,
    pub destroy: wl_listener,
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
unsafe extern "C" fn warp_to_touch(mut state: *mut sample_state,
                                   mut dev: *mut wlr_input_device) {
    if wl_list_empty(&mut (*state).touch_points) != 0 { return }
    let mut x: libc::c_double = 0i32 as libc::c_double;
    let mut y: libc::c_double = 0i32 as libc::c_double;
    let mut n: size_t = 0i32 as size_t;
    let mut point: *mut touch_point = 0 as *mut touch_point;
    point =
        ((*state).touch_points.next as *mut libc::c_char).offset(-24) as
            *mut touch_point;
    while &mut (*point).link as *mut wl_list !=
              &mut (*state).touch_points as *mut wl_list {
        x += (*point).x;
        y += (*point).y;
        n = n.wrapping_add(1);
        point =
            ((*point).link.next as *mut libc::c_char).offset(-24) as
                *mut touch_point
    }
    x /= n as libc::c_double;
    y /= n as libc::c_double;
    wlr_cursor_warp_absolute((*state).cursor, dev, x, y);
}
unsafe extern "C" fn output_frame_notify(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut sample_output: *mut sample_output =
        (listener as *mut libc::c_char).offset(-16) as *mut sample_output;
    let mut state: *mut sample_state = (*sample_output).state;
    let mut wlr_output: *mut wlr_output = (*sample_output).output;
    let mut renderer: *mut wlr_renderer =
        wlr_backend_get_renderer((*wlr_output).backend);
    if !renderer.is_null() {
    } else {
        __assert_fail(b"renderer\x00" as *const u8 as *const libc::c_char,
                      b"../examples/pointer.c\x00" as *const u8 as
                          *const libc::c_char, 98i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void output_frame_notify(struct wl_listener *, void *)\x00")).as_ptr());
    };
    wlr_output_attach_render(wlr_output, 0 as *mut libc::c_int);
    wlr_renderer_begin(renderer, (*wlr_output).width, (*wlr_output).height);
    wlr_renderer_clear(renderer,
                       (*state).clear_color.as_mut_ptr() as
                           *const libc::c_float);
    wlr_output_render_software_cursors(wlr_output,
                                       0 as *mut pixman_region32_t);
    wlr_output_commit(wlr_output);
    wlr_renderer_end(renderer);
}
unsafe extern "C" fn handle_cursor_motion(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-168) as *mut sample_state;
    let mut event: *mut wlr_event_pointer_motion =
        data as *mut wlr_event_pointer_motion;
    wlr_cursor_move((*sample).cursor, (*event).device, (*event).delta_x,
                    (*event).delta_y);
}
unsafe extern "C" fn handle_cursor_motion_absolute(mut listener:
                                                       *mut wl_listener,
                                                   mut data:
                                                       *mut libc::c_void) {
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-192) as *mut sample_state;
    let mut event: *mut wlr_event_pointer_motion_absolute =
        data as *mut wlr_event_pointer_motion_absolute;
    (*sample).cur_x = (*event).x;
    (*sample).cur_y = (*event).y;
    wlr_cursor_warp_absolute((*sample).cursor, (*event).device,
                             (*sample).cur_x, (*sample).cur_y);
}
unsafe extern "C" fn handle_cursor_button(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-216) as *mut sample_state;
    let mut event: *mut wlr_event_pointer_button =
        data as *mut wlr_event_pointer_button;
    let mut color: *mut [libc::c_float; 4] = 0 as *mut [libc::c_float; 4];
    if (*event).state as libc::c_uint ==
           WLR_BUTTON_RELEASED as libc::c_int as libc::c_uint {
        color = &mut (*sample).default_color;
        memcpy(&mut (*sample).clear_color as *mut [libc::c_float; 4] as
                   *mut libc::c_void, color as *const libc::c_void,
               ::std::mem::size_of::<[libc::c_float; 4]>() as libc::c_ulong);
    } else {
        let mut red: [libc::c_float; 4] =
            [0.25f32, 0.25f32, 0.25f32, 1i32 as libc::c_float];
        red[(*event).button.wrapping_rem(3i32 as libc::c_uint) as usize] =
            1i32 as libc::c_float;
        color = &mut red;
        memcpy(&mut (*sample).clear_color as *mut [libc::c_float; 4] as
                   *mut libc::c_void, color as *const libc::c_void,
               ::std::mem::size_of::<[libc::c_float; 4]>() as libc::c_ulong);
    };
}
unsafe extern "C" fn handle_cursor_axis(mut listener: *mut wl_listener,
                                        mut data: *mut libc::c_void) {
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-240) as *mut sample_state;
    let mut event: *mut wlr_event_pointer_axis =
        data as *mut wlr_event_pointer_axis;
    let mut i: size_t = 0i32 as size_t;
    while i < 3i32 as libc::c_ulong {
        (*sample).default_color[i as usize] +=
            if (*event).delta > 0i32 as libc::c_double {
                -0.05f32
            } else { 0.05f32 };
        if (*sample).default_color[i as usize] > 1.0f32 {
            (*sample).default_color[i as usize] = 1.0f32
        }
        if (*sample).default_color[i as usize] < 0.0f32 {
            (*sample).default_color[i as usize] = 0.0f32
        }
        i = i.wrapping_add(1)
    }
    memcpy(&mut (*sample).clear_color as *mut [libc::c_float; 4] as
               *mut libc::c_void,
           &mut (*sample).default_color as *mut [libc::c_float; 4] as
               *const libc::c_void,
           ::std::mem::size_of::<[libc::c_float; 4]>() as libc::c_ulong);
}
unsafe extern "C" fn handle_touch_up(mut listener: *mut wl_listener,
                                     mut data: *mut libc::c_void) {
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-288) as *mut sample_state;
    let mut event: *mut wlr_event_touch_up = data as *mut wlr_event_touch_up;
    let mut point: *mut touch_point = 0 as *mut touch_point;
    let mut tmp: *mut touch_point = 0 as *mut touch_point;
    point =
        ((*sample).touch_points.next as *mut libc::c_char).offset(-24) as
            *mut touch_point;
    tmp =
        ((*point).link.next as *mut libc::c_char).offset(-24) as
            *mut touch_point;
    while &mut (*point).link as *mut wl_list !=
              &mut (*sample).touch_points as *mut wl_list {
        if (*point).touch_id == (*event).touch_id {
            wl_list_remove(&mut (*point).link);
            break ;
        } else {
            point = tmp;
            tmp =
                ((*point).link.next as *mut libc::c_char).offset(-24) as
                    *mut touch_point
        }
    }
    warp_to_touch(sample, (*event).device);
}
unsafe extern "C" fn handle_touch_down(mut listener: *mut wl_listener,
                                       mut data: *mut libc::c_void) {
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-312) as *mut sample_state;
    let mut event: *mut wlr_event_touch_down =
        data as *mut wlr_event_touch_down;
    let mut point: *mut touch_point =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<touch_point>() as libc::c_ulong) as
            *mut touch_point;
    (*point).touch_id = (*event).touch_id;
    (*point).x = (*event).x;
    (*point).y = (*event).y;
    wl_list_insert(&mut (*sample).touch_points, &mut (*point).link);
    warp_to_touch(sample, (*event).device);
}
unsafe extern "C" fn handle_touch_motion(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-264) as *mut sample_state;
    let mut event: *mut wlr_event_touch_motion =
        data as *mut wlr_event_touch_motion;
    let mut point: *mut touch_point = 0 as *mut touch_point;
    point =
        ((*sample).touch_points.next as *mut libc::c_char).offset(-24) as
            *mut touch_point;
    while &mut (*point).link as *mut wl_list !=
              &mut (*sample).touch_points as *mut wl_list {
        if (*point).touch_id == (*event).touch_id {
            (*point).x = (*event).x;
            (*point).y = (*event).y;
            break ;
        } else {
            point =
                ((*point).link.next as *mut libc::c_char).offset(-24) as
                    *mut touch_point
        }
    }
    warp_to_touch(sample, (*event).device);
}
unsafe extern "C" fn handle_touch_cancel(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] TODO: touch cancel\x00" as *const u8 as
                 *const libc::c_char,
             b"../examples/pointer.c\x00" as *const u8 as *const libc::c_char,
             210i32);
}
unsafe extern "C" fn handle_tablet_tool_axis(mut listener: *mut wl_listener,
                                             mut data: *mut libc::c_void) {
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-376) as *mut sample_state;
    let mut event: *mut wlr_event_tablet_tool_axis =
        data as *mut wlr_event_tablet_tool_axis;
    if (*event).updated_axes &
           WLR_TABLET_TOOL_AXIS_X as libc::c_int as libc::c_uint != 0 &&
           (*event).updated_axes &
               WLR_TABLET_TOOL_AXIS_Y as libc::c_int as libc::c_uint != 0 {
        wlr_cursor_warp_absolute((*sample).cursor, (*event).device,
                                 (*event).x, (*event).y);
    };
}
unsafe extern "C" fn keyboard_key_notify(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut keyboard: *mut sample_keyboard =
        (listener as *mut libc::c_char).offset(-16) as *mut sample_keyboard;
    let mut sample: *mut sample_state = (*keyboard).state;
    let mut event: *mut wlr_event_keyboard_key =
        data as *mut wlr_event_keyboard_key;
    let mut keycode: uint32_t =
        (*event).keycode.wrapping_add(8i32 as libc::c_uint);
    let mut syms: *const xkb_keysym_t = 0 as *const xkb_keysym_t;
    let mut nsyms: libc::c_int =
        xkb_state_key_get_syms((*(*(*keyboard).device).c2rust_unnamed.keyboard).xkb_state,
                               keycode, &mut syms);
    let mut i: libc::c_int = 0i32;
    while i < nsyms {
        let mut sym: xkb_keysym_t = *syms.offset(i as isize);
        if sym == 0xff1bi32 as libc::c_uint {
            wl_display_terminate((*sample).display);
        }
        i += 1
    };
}
unsafe extern "C" fn output_remove_notify(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut sample_output: *mut sample_output =
        (listener as *mut libc::c_char).offset(-40) as *mut sample_output;
    let mut sample: *mut sample_state = (*sample_output).state;
    wlr_output_layout_remove((*sample).layout, (*sample_output).output);
    wl_list_remove(&mut (*sample_output).frame.link);
    wl_list_remove(&mut (*sample_output).destroy.link);
    free(sample_output as *mut libc::c_void);
}
unsafe extern "C" fn new_output_notify(mut listener: *mut wl_listener,
                                       mut data: *mut libc::c_void) {
    let mut output: *mut wlr_output = data as *mut wlr_output;
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-120) as *mut sample_state;
    let mut sample_output: *mut sample_output =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<sample_output>() as libc::c_ulong) as
            *mut sample_output;
    if wl_list_empty(&mut (*output).modes) == 0 {
        let mut mode: *mut wlr_output_mode =
            ((*output).modes.prev as *mut libc::c_char).offset(-16) as
                *mut wlr_output_mode;
        wlr_output_set_mode(output, mode);
    }
    (*sample_output).output = output;
    (*sample_output).state = sample;
    wl_signal_add(&mut (*output).events.frame, &mut (*sample_output).frame);
    (*sample_output).frame.notify =
        Some(output_frame_notify as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.destroy,
                  &mut (*sample_output).destroy);
    (*sample_output).destroy.notify =
        Some(output_remove_notify as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wlr_output_layout_add_auto((*sample).layout, (*sample_output).output);
    wlr_xcursor_manager_load((*sample).xcursor_manager, (*output).scale);
    wlr_xcursor_manager_set_cursor_image((*sample).xcursor_manager,
                                         b"left_ptr\x00" as *const u8 as
                                             *const libc::c_char,
                                         (*sample).cursor);
}
unsafe extern "C" fn keyboard_destroy_notify(mut listener: *mut wl_listener,
                                             mut data: *mut libc::c_void) {
    let mut keyboard: *mut sample_keyboard =
        (listener as *mut libc::c_char).offset(-40) as *mut sample_keyboard;
    wl_list_remove(&mut (*keyboard).destroy.link);
    wl_list_remove(&mut (*keyboard).key.link);
    free(keyboard as *mut libc::c_void);
}
unsafe extern "C" fn new_input_notify(mut listener: *mut wl_listener,
                                      mut data: *mut libc::c_void) {
    let mut device: *mut wlr_input_device = data as *mut wlr_input_device;
    let mut state: *mut sample_state =
        (listener as *mut libc::c_char).offset(-144) as *mut sample_state;
    match (*device).type_0 as libc::c_uint {
        1 | 2 | 3 => {
            wlr_cursor_attach_input_device((*state).cursor, device);
        }
        0 => {
            let mut keyboard: *mut sample_keyboard =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<sample_keyboard>() as
                           libc::c_ulong) as *mut sample_keyboard;
            (*keyboard).device = device;
            (*keyboard).state = state;
            wl_signal_add(&mut (*device).events.destroy,
                          &mut (*keyboard).destroy);
            (*keyboard).destroy.notify =
                Some(keyboard_destroy_notify as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            wl_signal_add(&mut (*(*device).c2rust_unnamed.keyboard).events.key,
                          &mut (*keyboard).key);
            (*keyboard).key.notify =
                Some(keyboard_key_notify as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            let mut rules: xkb_rule_names =
                {
                    let mut init =
                        xkb_rule_names{rules: 0 as *const libc::c_char,
                                       model: 0 as *const libc::c_char,
                                       layout: 0 as *const libc::c_char,
                                       variant: 0 as *const libc::c_char,
                                       options: 0 as *const libc::c_char,};
                    init
                };
            rules.rules =
                getenv(b"XKB_DEFAULT_RULES\x00" as *const u8 as
                           *const libc::c_char);
            rules.model =
                getenv(b"XKB_DEFAULT_MODEL\x00" as *const u8 as
                           *const libc::c_char);
            rules.layout =
                getenv(b"XKB_DEFAULT_LAYOUT\x00" as *const u8 as
                           *const libc::c_char);
            rules.variant =
                getenv(b"XKB_DEFAULT_VARIANT\x00" as *const u8 as
                           *const libc::c_char);
            rules.options =
                getenv(b"XKB_DEFAULT_OPTIONS\x00" as *const u8 as
                           *const libc::c_char);
            let mut context: *mut xkb_context =
                xkb_context_new(XKB_CONTEXT_NO_FLAGS);
            if context.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to create XKB context\x00" as
                             *const u8 as *const libc::c_char,
                         b"../examples/pointer.c\x00" as *const u8 as
                             *const libc::c_char, 304i32);
                exit(1i32);
            }
            let mut keymap: *mut xkb_keymap =
                xkb_keymap_new_from_names(context, &mut rules,
                                          XKB_KEYMAP_COMPILE_NO_FLAGS);
            if keymap.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to create XKB keymap\x00" as
                             *const u8 as *const libc::c_char,
                         b"../examples/pointer.c\x00" as *const u8 as
                             *const libc::c_char, 310i32);
                exit(1i32);
            }
            wlr_keyboard_set_keymap((*device).c2rust_unnamed.keyboard,
                                    keymap);
            xkb_keymap_unref(keymap);
            xkb_context_unref(context);
        }
        _ => { }
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    wlr_log_init(WLR_DEBUG, None);
    let mut display: *mut wl_display = wl_display_create();
    let mut state: sample_state =
        {
            let mut init =
                sample_state{display: display,
                             compositor: 0 as *mut compositor_state,
                             xcursor_manager: 0 as *mut wlr_xcursor_manager,
                             cursor: 0 as *mut wlr_cursor,
                             cur_x: 0.,
                             cur_y: 0.,
                             default_color:
                                 [0.25f32, 0.25f32, 0.25f32,
                                  1i32 as libc::c_float],
                             clear_color:
                                 [0.25f32, 0.25f32, 0.25f32,
                                  1i32 as libc::c_float],
                             layout: 0 as *mut wlr_output_layout,
                             devices:
                                 wl_list{prev: 0 as *mut wl_list,
                                         next: 0 as *mut wl_list,},
                             last_frame: timespec{tv_sec: 0, tv_nsec: 0,},
                             new_output:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             new_input:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             cursor_motion:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             cursor_motion_absolute:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             cursor_button:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             cursor_axis:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             touch_motion:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             touch_up:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             touch_down:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             touch_cancel:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             touch_points:
                                 wl_list{prev: 0 as *mut wl_list,
                                         next: 0 as *mut wl_list,},
                             tablet_tool_axis:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             tablet_tool_proxmity:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             tablet_tool_tip:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},
                             tablet_tool_button:
                                 wl_listener{link:
                                                 wl_list{prev:
                                                             0 as
                                                                 *mut wl_list,
                                                         next:
                                                             0 as
                                                                 *mut wl_list,},
                                             notify: None,},};
            init
        };
    let mut wlr: *mut wlr_backend = wlr_backend_autocreate(display, None);
    if wlr.is_null() { exit(1i32); }
    state.cursor = wlr_cursor_create();
    state.layout = wlr_output_layout_create();
    wlr_cursor_attach_output_layout(state.cursor, state.layout);
    //wlr_cursor_map_to_region(state.cursor, state.config->cursor.mapped_box);
    wl_list_init(&mut state.devices);
    wl_list_init(&mut state.touch_points);
    // pointer events
    wl_signal_add(&mut (*state.cursor).events.motion,
                  &mut state.cursor_motion);
    state.cursor_motion.notify =
        Some(handle_cursor_motion as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*state.cursor).events.motion_absolute,
                  &mut state.cursor_motion_absolute);
    state.cursor_motion_absolute.notify =
        Some(handle_cursor_motion_absolute as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*state.cursor).events.button,
                  &mut state.cursor_button);
    state.cursor_button.notify =
        Some(handle_cursor_button as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*state.cursor).events.axis, &mut state.cursor_axis);
    state.cursor_axis.notify =
        Some(handle_cursor_axis as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    // touch events
    wl_signal_add(&mut (*state.cursor).events.touch_up, &mut state.touch_up);
    state.touch_up.notify =
        Some(handle_touch_up as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*state.cursor).events.touch_down,
                  &mut state.touch_down);
    state.touch_down.notify =
        Some(handle_touch_down as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*state.cursor).events.touch_motion,
                  &mut state.touch_motion);
    state.touch_motion.notify =
        Some(handle_touch_motion as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*state.cursor).events.touch_cancel,
                  &mut state.touch_cancel);
    state.touch_cancel.notify =
        Some(handle_touch_cancel as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*wlr).events.new_input, &mut state.new_input);
    state.new_input.notify =
        Some(new_input_notify as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*wlr).events.new_output, &mut state.new_output);
    state.new_output.notify =
        Some(new_output_notify as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    // tool events
    wl_signal_add(&mut (*state.cursor).events.tablet_tool_axis,
                  &mut state.tablet_tool_axis);
    state.tablet_tool_axis.notify =
        Some(handle_tablet_tool_axis as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    state.xcursor_manager =
        wlr_xcursor_manager_create(b"default\x00" as *const u8 as
                                       *const libc::c_char,
                                   24i32 as uint32_t);
    if state.xcursor_manager.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to load left_ptr cursor\x00" as *const u8 as
                     *const libc::c_char,
                 b"../examples/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 383i32);
        return 1i32
    }
    wlr_xcursor_manager_set_cursor_image(state.xcursor_manager,
                                         b"left_ptr\x00" as *const u8 as
                                             *const libc::c_char,
                                         state.cursor);
    clock_gettime(1i32, &mut state.last_frame);
    if !wlr_backend_start(wlr) {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to start backend\x00" as *const u8 as
                     *const libc::c_char,
                 b"../examples/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 393i32);
        wlr_backend_destroy(wlr);
        exit(1i32);
    }
    wl_display_run(display);
    wl_display_destroy(display);
    wlr_xcursor_manager_destroy(state.xcursor_manager);
    wlr_cursor_destroy(state.cursor);
    wlr_output_layout_destroy(state.layout);
    return 0;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
