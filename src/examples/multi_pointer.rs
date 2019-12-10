use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_renderer_impl;
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
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_touch_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_switch_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_pointer_impl;
    pub type xkb_state;
    pub type xkb_keymap;
    pub type wlr_keyboard_group;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_keyboard_impl;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* Note: these are circular dependencies */
    pub type wlr_input_device_impl;
    pub type wlr_surface;
    pub type wlr_texture_impl;
    pub type wlr_backend_impl;
    pub type wlr_output_impl;
    pub type xkb_context;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_output_layout_state;
    pub type wlr_cursor_state;
    #[no_mangle]
    fn glClear(mask: GLbitfield);
    #[no_mangle]
    fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat,
                    alpha: GLfloat);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_display_create() -> *mut wl_display;
    #[no_mangle]
    fn wl_display_destroy(display: *mut wl_display);
    #[no_mangle]
    fn wl_display_terminate(display: *mut wl_display);
    #[no_mangle]
    fn wl_display_run(display: *mut wl_display);
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
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* *
 * wlr_cursor implements the behavior of the "cursor", that is, the image on the
 * screen typically moved about with a mouse or so. It provides tracking for
 * this in global coordinates, and integrates with wlr_output,
 * wlr_output_layout, and wlr_input_device. You can use it to abstract multiple
 * input devices over a single cursor, constrain cursor movement to the usable
 * area of a wlr_output_layout and communicate position updates to the hardware
 * cursor, constrain specific input devices to specific outputs or regions of
 * the screen, and so on.
 */
    /* *
	 * The interpretation of these signals is the responsibility of the
	 * compositor, but some helpers are provided for your benefit. If you
	 * receive a relative motion event, for example, you may want to call
	 * wlr_cursor_move. If you receive an absolute event, call
	 * wlr_cursor_warp_absolute. If you pass an input device into these
	 * functions, it will apply the region/output constraints associated with
	 * that device to the resulting cursor motion. If an output layout is
	 * attached, these functions will constrain the resulting cursor motion to
	 * within the usable space of the output layout.
	 *
	 * Re-broadcasting these signals to, for example, a wlr_seat, is also your
	 * responsibility.
	 */
    /* *
 * Warp the cursor to the given x and y in layout coordinates. If x and y are
 * out of the layout boundaries or constraints, no warp will happen.
 *
 * `dev` may be passed to respect device mapping constraints. If `dev` is NULL,
 * device mapping constraints will be ignored.
 *
 * Returns true when the cursor warp was successful.
 */
    /* *
 * Convert absolute 0..1 coordinates to layout coordinates.
 *
 * `dev` may be passed to respect device mapping constraints. If `dev` is NULL,
 * device mapping constraints will be ignored.
 */
    /* *
 * Warp the cursor to the given x and y coordinates. If the given point is out
 * of the layout boundaries or constraints, the closest point will be used.
 * If one coordinate is NAN, it will be ignored.
 *
 * `dev` may be passed to respect device mapping constraints. If `dev` is NULL,
 * device mapping constraints will be ignored.
 */
    /* *
 * Warp the cursor to the given x and y in absolute 0..1 coordinates. If the
 * given point is out of the layout boundaries or constraints, the closest point
 * will be used. If one coordinate is NAN, it will be ignored.
 *
 * `dev` may be passed to respect device mapping constraints. If `dev` is NULL,
 * device mapping constraints will be ignored.
 */
    /* *
 * Move the cursor in the direction of the given x and y layout coordinates. If
 * one coordinate is NAN, it will be ignored.
 *
 * `dev` may be passed to respect device mapping constraints. If `dev` is NULL,
 * device mapping constraints will be ignored.
 */
    /* *
 * Set the cursor image. stride is given in bytes. If pixels is NULL, hides the
 * cursor.
 *
 * If scale isn't zero, the image is only set on outputs having the provided
 * scale.
 */
    /* *
 * Set the cursor surface. The surface can be committed to update the cursor
 * image. The surface position is subtracted from the hotspot. A NULL surface
 * commit hides the cursor.
 */
    /* *
 * Attaches this input device to this cursor. The input device must be one of:
 *
 * - WLR_INPUT_DEVICE_POINTER
 * - WLR_INPUT_DEVICE_TOUCH
 * - WLR_INPUT_DEVICE_TABLET_TOOL
 */
    /* *
 * Uses the given layout to establish the boundaries and movement semantics of
 * this cursor. Cursors without an output layout allow infinite movement in any
 * direction and do not support absolute input events.
 */
    /* *
 * Attaches this cursor to the given output, which must be among the outputs in
 * the current output_layout for this cursor. This call is invalid for a cursor
 * without an associated output layout.
 */
    /* *
 * Maps all input from a specific input device to a given output. The input
 * device must be attached to this cursor and the output must be among the
 * outputs in the attached output layout.
 */
    #[no_mangle]
    fn wlr_cursor_map_input_to_output(cur: *mut wlr_cursor,
                                      dev: *mut wlr_input_device,
                                      output: *mut wlr_output);
    #[no_mangle]
    fn wlr_cursor_map_to_output(cur: *mut wlr_cursor,
                                output: *mut wlr_output);
    #[no_mangle]
    fn wlr_cursor_attach_output_layout(cur: *mut wlr_cursor,
                                       l: *mut wlr_output_layout);
    #[no_mangle]
    fn wlr_output_set_mode(output: *mut wlr_output,
                           mode: *mut wlr_output_mode) -> bool;
    #[no_mangle]
    fn wlr_keyboard_set_keymap(kb: *mut wlr_keyboard,
                               keymap: *mut xkb_keymap);
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
 * Creates a wlr_output_layout, which can be used to describing outputs in
 * physical space relative to one another, and perform various useful operations
 * on that state.
 */
    #[no_mangle]
    fn wlr_output_layout_create() -> *mut wlr_output_layout;
    #[no_mangle]
    fn wlr_output_layout_destroy(layout: *mut wlr_output_layout);
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
    fn wlr_cursor_warp(cur: *mut wlr_cursor, dev: *mut wlr_input_device,
                       lx: libc::c_double, ly: libc::c_double) -> bool;
    #[no_mangle]
    fn wlr_cursor_warp_absolute(cur: *mut wlr_cursor,
                                dev: *mut wlr_input_device, x: libc::c_double,
                                y: libc::c_double);
    #[no_mangle]
    fn wlr_cursor_move(cur: *mut wlr_cursor, dev: *mut wlr_input_device,
                       delta_x: libc::c_double, delta_y: libc::c_double);
    #[no_mangle]
    fn wlr_cursor_set_image(cur: *mut wlr_cursor, pixels: *const uint8_t,
                            stride: int32_t, width: uint32_t,
                            height: uint32_t, hotspot_x: int32_t,
                            hotspot_y: int32_t, scale: libc::c_float);
    #[no_mangle]
    fn wlr_cursor_attach_input_device(cur: *mut wlr_cursor,
                                      dev: *mut wlr_input_device);
    #[no_mangle]
    fn wlr_cursor_detach_input_device(cur: *mut wlr_cursor,
                                      dev: *mut wlr_input_device);
    // Will log all messages less than or equal to `verbosity`
// If `callback` is NULL, wlr will use its default logger.
// The function can be called multiple times to update the verbosity or
// callback function.
    #[no_mangle]
    fn wlr_log_init(verbosity: wlr_log_importance, callback: wlr_log_func_t);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    /* *
 * Loads the named xcursor theme at the given cursor size (in pixels). This is
 * useful if you need cursor images for your compositor to use when a
 * client-side cursors is not available or you wish to override client-side
 * cursors for a particular UI interaction (such as using a grab cursor when
 * moving a window around).
 */
    #[no_mangle]
    fn wlr_xcursor_theme_load(name: *const libc::c_char, size: libc::c_int)
     -> *mut wlr_xcursor_theme;
    #[no_mangle]
    fn wlr_xcursor_theme_destroy(theme: *mut wlr_xcursor_theme);
    /* *
 * Obtains a wlr_xcursor image for the specified cursor name (e.g. "left_ptr").
 */
    #[no_mangle]
    fn wlr_xcursor_theme_get_cursor(theme: *mut wlr_xcursor_theme,
                                    name: *const libc::c_char)
     -> *mut wlr_xcursor;
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type khronos_int32_t = int32_t;
pub type khronos_float_t = libc::c_float;
pub type GLfloat = khronos_float_t;
pub type GLbitfield = libc::c_uint;
pub type size_t = libc::c_ulong;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
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
pub type EGLenum = libc::c_uint;
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
    pub repeat_info: C2RustUnnamed_9,
    pub events: C2RustUnnamed_8,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
pub type wlr_input_device_type = libc::c_uint;
pub const WLR_INPUT_DEVICE_SWITCH: wlr_input_device_type = 5;
pub const WLR_INPUT_DEVICE_TABLET_PAD: wlr_input_device_type = 4;
pub const WLR_INPUT_DEVICE_TABLET_TOOL: wlr_input_device_type = 3;
pub const WLR_INPUT_DEVICE_TOUCH: wlr_input_device_type = 2;
pub const WLR_INPUT_DEVICE_POINTER: wlr_input_device_type = 1;
pub const WLR_INPUT_DEVICE_KEYBOARD: wlr_input_device_type = 0;
pub type pixman_region32_t = pixman_region32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
/* **********************************************************

Copyright 1987, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from The Open Group.

Copyright 1987 by Digital Equipment Corporation, Maynard, Massachusetts.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name of Digital not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

******************************************************************/
/*
 * Copyright © 1998, 2004 Keith Packard
 * Copyright   2007 Red Hat, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of Keith Packard not be used in
 * advertising or publicity pertaining to distribution of the software without
 * specific, written prior permission.  Keith Packard makes no
 * representations about the suitability of this software for any purpose.  It
 * is provided "as is" without express or implied warranty.
 *
 * KEITH PACKARD DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL KEITH PACKARD BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Standard integers
 */
/*
 * Boolean
 */
/*
 * Fixpoint numbers
 */
/*
 * Misc structs
 */
/*
 * Fixed point matrices
 */
/* forward declaration (sorry) */
/*
 * Floating point matrices
 */
/* The SEPARABLE_CONVOLUTION filter takes the following parameters:
     *
     *         width:           integer given as 16.16 fixpoint number
     *         height:          integer given as 16.16 fixpoint number
     *         x_phase_bits:	integer given as 16.16 fixpoint
     *         y_phase_bits:	integer given as 16.16 fixpoint
     *         xtables:         (1 << x_phase_bits) tables of size width
     *         ytables:         (1 << y_phase_bits) tables of size height
     *
     * When sampling at (x, y), the location is first rounded to one of
     * n_x_phases * n_y_phases subpixel positions. These subpixel positions
     * determine an xtable and a ytable to use.
     *
     * Conceptually a width x height matrix is then formed in which each entry
     * is the product of the corresponding entries in the x and y tables.
     * This matrix is then aligned with the image pixels such that its center
     * is as close as possible to the subpixel location chosen earlier. Then
     * the image is convolved with the matrix and the resulting pixel returned.
     */
/*
 * Regions
 */
/*  pixman_box16_t	rects[size];   in memory but not explicitly declared */
/* This function exists only to make it possible to preserve
 * the X ABI - it should go away at first opportunity.
 */
/* creation/destruction */
/* manipulation */
/*
 * 32 bit regions
 */
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
    pub events: C2RustUnnamed_11,
    pub idle_frame: *mut wl_event_source,
    pub idle_done: *mut wl_event_source,
    pub attach_render_locks: libc::c_int,
    pub cursors: wl_list,
    pub hardware_cursor: *mut wlr_output_cursor,
    pub software_cursor_locks: libc::c_int,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
// mHz
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
    pub events: C2RustUnnamed_10,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_texture {
    pub impl_0: *const wlr_texture_impl,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
pub type wl_output_subpixel = libc::c_uint;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: wl_output_subpixel = 5;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: wl_output_subpixel = 4;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: wl_output_subpixel = 3;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: wl_output_subpixel = 2;
pub const WL_OUTPUT_SUBPIXEL_NONE: wl_output_subpixel = 1;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: wl_output_subpixel = 0;
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
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed_12,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
pub type xkb_keycode_t = uint32_t;
pub type xkb_keysym_t = uint32_t;
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
pub struct wlr_event_pointer_motion_absolute {
    pub device: *mut wlr_input_device,
    pub time_msec: uint32_t,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub state: *mut wlr_output_layout_state,
    pub events: C2RustUnnamed_13,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
    pub events: C2RustUnnamed_14,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
/*
 * Copyright © 2012 Intel Corporation
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial
 * portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xcursor_image {
    pub width: uint32_t,
    pub height: uint32_t,
    pub hotspot_x: uint32_t,
    pub hotspot_y: uint32_t,
    pub delay: uint32_t,
    pub buffer: *mut uint8_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xcursor {
    pub image_count: libc::c_uint,
    pub images: *mut *mut wlr_xcursor_image,
    pub name: *mut libc::c_char,
    pub total_delay: uint32_t,
    /* length of the animation in ms */
}
/* *
 * Container for an Xcursor theme.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_xcursor_theme {
    pub cursor_count: libc::c_uint,
    pub cursors: *mut *mut wlr_xcursor,
    pub name: *mut libc::c_char,
    pub size: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_state {
    pub display: *mut wl_display,
    pub xcursor: *mut wlr_xcursor,
    pub default_color: [libc::c_float; 4],
    pub clear_color: [libc::c_float; 4],
    pub layout: *mut wlr_output_layout,
    pub cursors: wl_list,
    pub pointers: wl_list,
    pub outputs: wl_list,
    pub last_frame: timespec,
    pub new_output: wl_listener,
    pub new_input: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_cursor {
    pub sample: *mut sample_state,
    pub device: *mut wlr_input_device,
    pub cursor: *mut wlr_cursor,
    pub link: wl_list,
    pub cursor_motion: wl_listener,
    pub cursor_motion_absolute: wl_listener,
    pub cursor_button: wl_listener,
    pub cursor_axis: wl_listener,
    pub destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_pointer {
    pub device: *mut wlr_input_device,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_output {
    pub sample: *mut sample_state,
    pub output: *mut wlr_output,
    pub frame: wl_listener,
    pub destroy: wl_listener,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sample_keyboard {
    pub sample: *mut sample_state,
    pub device: *mut wlr_input_device,
    pub key: wl_listener,
    pub destroy: wl_listener,
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
unsafe extern "C" fn configure_cursor(mut cursor: *mut wlr_cursor,
                                      mut device: *mut wlr_input_device,
                                      mut sample: *mut sample_state) {
    let mut output: *mut sample_output = 0 as *mut sample_output;
    _wlr_log(WLR_ERROR,
             b"[%s:%d] Configuring cursor %p for device %p\x00" as *const u8
                 as *const libc::c_char,
             b"../examples/multi-pointer.c\x00" as *const u8 as
                 *const libc::c_char, 74i32, cursor, device);
    // reset mappings
    wlr_cursor_map_to_output(cursor, 0 as *mut wlr_output);
    wlr_cursor_detach_input_device(cursor, device);
    wlr_cursor_map_input_to_output(cursor, device, 0 as *mut wlr_output);
    wlr_cursor_attach_input_device(cursor, device);
    // configure device to output mappings
    output =
        ((*sample).outputs.next as *mut libc::c_char).offset(-64) as
            *mut sample_output;
    while &mut (*output).link as *mut wl_list !=
              &mut (*sample).outputs as *mut wl_list {
        wlr_cursor_map_to_output(cursor, (*output).output);
        wlr_cursor_map_input_to_output(cursor, device, (*output).output);
        output =
            ((*output).link.next as *mut libc::c_char).offset(-64) as
                *mut sample_output
    };
}
unsafe extern "C" fn output_frame_notify(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut output: *mut sample_output =
        (listener as *mut libc::c_char).offset(-16) as *mut sample_output;
    let mut sample: *mut sample_state = (*output).sample;
    let mut wlr_output: *mut wlr_output = (*output).output;
    wlr_output_attach_render(wlr_output, 0 as *mut libc::c_int);
    glClearColor((*sample).clear_color[0], (*sample).clear_color[1],
                 (*sample).clear_color[2], (*sample).clear_color[3]);
    glClear(0x4000i32 as GLbitfield);
    wlr_output_render_software_cursors(wlr_output,
                                       0 as *mut pixman_region32_t);
    wlr_output_commit(wlr_output);
}
unsafe extern "C" fn handle_cursor_motion(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut cursor: *mut sample_cursor =
        (listener as *mut libc::c_char).offset(-40) as *mut sample_cursor;
    let mut event: *mut wlr_event_pointer_motion =
        data as *mut wlr_event_pointer_motion;
    wlr_cursor_move((*cursor).cursor, (*event).device, (*event).delta_x,
                    (*event).delta_y);
}
unsafe extern "C" fn handle_cursor_motion_absolute(mut listener:
                                                       *mut wl_listener,
                                                   mut data:
                                                       *mut libc::c_void) {
    let mut cursor: *mut sample_cursor =
        (listener as *mut libc::c_char).offset(-64) as *mut sample_cursor;
    let mut event: *mut wlr_event_pointer_motion_absolute =
        data as *mut wlr_event_pointer_motion_absolute;
    wlr_cursor_warp_absolute((*cursor).cursor, (*event).device, (*event).x,
                             (*event).y);
}
unsafe extern "C" fn cursor_destroy(mut cursor: *mut sample_cursor) {
    wl_list_remove(&mut (*cursor).link);
    wl_list_remove(&mut (*cursor).cursor_motion.link);
    wl_list_remove(&mut (*cursor).cursor_motion_absolute.link);
    wlr_cursor_destroy((*cursor).cursor);
    free(cursor as *mut libc::c_void);
}
unsafe extern "C" fn output_remove_notify(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut sample_output: *mut sample_output =
        (listener as *mut libc::c_char).offset(-40) as *mut sample_output;
    let mut sample: *mut sample_state = (*sample_output).sample;
    wl_list_remove(&mut (*sample_output).frame.link);
    wl_list_remove(&mut (*sample_output).destroy.link);
    wl_list_remove(&mut (*sample_output).link);
    free(sample_output as *mut libc::c_void);
    let mut cursor: *mut sample_cursor = 0 as *mut sample_cursor;
    cursor =
        ((*sample).cursors.next as *mut libc::c_char).offset(-24) as
            *mut sample_cursor;
    while &mut (*cursor).link as *mut wl_list !=
              &mut (*sample).cursors as *mut wl_list {
        configure_cursor((*cursor).cursor, (*cursor).device, sample);
        cursor =
            ((*cursor).link.next as *mut libc::c_char).offset(-24) as
                *mut sample_cursor
    };
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
    (*sample_output).sample = sample;
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
    wlr_output_layout_add_auto((*sample).layout, output);
    let mut cursor: *mut sample_cursor = 0 as *mut sample_cursor;
    cursor =
        ((*sample).cursors.next as *mut libc::c_char).offset(-24) as
            *mut sample_cursor;
    while &mut (*cursor).link as *mut wl_list !=
              &mut (*sample).cursors as *mut wl_list {
        configure_cursor((*cursor).cursor, (*cursor).device, sample);
        let mut image: *mut wlr_xcursor_image =
            *(*(*sample).xcursor).images.offset(0);
        wlr_cursor_set_image((*cursor).cursor, (*image).buffer,
                             (*image).width.wrapping_mul(4i32 as libc::c_uint)
                                 as int32_t, (*image).width, (*image).height,
                             (*image).hotspot_x as int32_t,
                             (*image).hotspot_y as int32_t,
                             0i32 as libc::c_float);
        wlr_cursor_warp((*cursor).cursor, 0 as *mut wlr_input_device,
                        (*(*cursor).cursor).x, (*(*cursor).cursor).y);
        cursor =
            ((*cursor).link.next as *mut libc::c_char).offset(-24) as
                *mut sample_cursor
    }
    wl_list_insert(&mut (*sample).outputs, &mut (*sample_output).link);
}
unsafe extern "C" fn keyboard_key_notify(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut keyboard: *mut sample_keyboard =
        (listener as *mut libc::c_char).offset(-16) as *mut sample_keyboard;
    let mut sample: *mut sample_state = (*keyboard).sample;
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
    let mut sample: *mut sample_state =
        (listener as *mut libc::c_char).offset(-144) as *mut sample_state;
    let mut keyboard: *mut sample_keyboard = 0 as *mut sample_keyboard;
    let mut rules: xkb_rule_names =
        xkb_rule_names{rules: 0 as *const libc::c_char,
                       model: 0 as *const libc::c_char,
                       layout: 0 as *const libc::c_char,
                       variant: 0 as *const libc::c_char,
                       options: 0 as *const libc::c_char,};
    let mut context: *mut xkb_context = 0 as *mut xkb_context;
    let mut keymap: *mut xkb_keymap = 0 as *mut xkb_keymap;
    match (*device).type_0 as libc::c_uint {
        0 => {
            keyboard =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<sample_keyboard>() as
                           libc::c_ulong) as *mut sample_keyboard;
            (*keyboard).device = device;
            (*keyboard).sample = sample;
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
            rules =
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
            context = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
            if context.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to create XKB context\x00" as
                             *const u8 as *const libc::c_char,
                         b"../examples/multi-pointer.c\x00" as *const u8 as
                             *const libc::c_char, 219i32);
                exit(1i32);
            }
            keymap =
                xkb_keymap_new_from_names(context, &mut rules,
                                          XKB_KEYMAP_COMPILE_NO_FLAGS);
            if keymap.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to create XKB keymap\x00" as
                             *const u8 as *const libc::c_char,
                         b"../examples/multi-pointer.c\x00" as *const u8 as
                             *const libc::c_char, 225i32);
                exit(1i32);
            }
            wlr_keyboard_set_keymap((*device).c2rust_unnamed.keyboard,
                                    keymap);
            xkb_keymap_unref(keymap);
            xkb_context_unref(context);
        }
        1 => {
            let mut cursor: *mut sample_cursor =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<sample_cursor>() as
                           libc::c_ulong) as *mut sample_cursor;
            let mut pointer: *mut sample_pointer =
                calloc(1i32 as libc::c_ulong,
                       ::std::mem::size_of::<sample_pointer>() as
                           libc::c_ulong) as *mut sample_pointer;
            (*pointer).device = device;
            (*cursor).sample = sample;
            (*cursor).device = device;
            (*cursor).cursor = wlr_cursor_create();
            wlr_cursor_attach_output_layout((*cursor).cursor,
                                            (*sample).layout);
            wl_signal_add(&mut (*(*cursor).cursor).events.motion,
                          &mut (*cursor).cursor_motion);
            (*cursor).cursor_motion.notify =
                Some(handle_cursor_motion as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            wl_signal_add(&mut (*(*cursor).cursor).events.motion_absolute,
                          &mut (*cursor).cursor_motion_absolute);
            (*cursor).cursor_motion_absolute.notify =
                Some(handle_cursor_motion_absolute as
                         unsafe extern "C" fn(_: *mut wl_listener,
                                              _: *mut libc::c_void) -> ());
            wlr_cursor_attach_input_device((*cursor).cursor, device);
            configure_cursor((*cursor).cursor, device, sample);
            let mut image: *mut wlr_xcursor_image =
                *(*(*sample).xcursor).images.offset(0);
            wlr_cursor_set_image((*cursor).cursor, (*image).buffer,
                                 (*image).width.wrapping_mul(4i32 as
                                                                 libc::c_uint)
                                     as int32_t, (*image).width,
                                 (*image).height,
                                 (*image).hotspot_x as int32_t,
                                 (*image).hotspot_y as int32_t,
                                 0i32 as libc::c_float);
            wl_list_insert(&mut (*sample).cursors, &mut (*cursor).link);
            wl_list_insert(&mut (*sample).pointers, &mut (*pointer).link);
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
                             xcursor: 0 as *mut wlr_xcursor,
                             default_color:
                                 [0.25f32, 0.25f32, 0.25f32,
                                  1i32 as libc::c_float],
                             clear_color:
                                 [0.25f32, 0.25f32, 0.25f32,
                                  1i32 as libc::c_float],
                             layout: 0 as *mut wlr_output_layout,
                             cursors:
                                 wl_list{prev: 0 as *mut wl_list,
                                         next: 0 as *mut wl_list,},
                             pointers:
                                 wl_list{prev: 0 as *mut wl_list,
                                         next: 0 as *mut wl_list,},
                             outputs:
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
                                             notify: None,},};
            init
        };
    let mut wlr: *mut wlr_backend = wlr_backend_autocreate(display, None);
    if wlr.is_null() { exit(1i32); }
    wl_list_init(&mut state.cursors);
    wl_list_init(&mut state.pointers);
    wl_list_init(&mut state.outputs);
    state.layout = wlr_output_layout_create();
    wl_signal_add(&mut (*wlr).events.new_output, &mut state.new_output);
    state.new_output.notify =
        Some(new_output_notify as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*wlr).events.new_input, &mut state.new_input);
    state.new_input.notify =
        Some(new_input_notify as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    clock_gettime(1i32, &mut state.last_frame);
    let mut theme: *mut wlr_xcursor_theme =
        wlr_xcursor_theme_load(b"default\x00" as *const u8 as
                                   *const libc::c_char, 16i32);
    if theme.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to load cursor theme\x00" as *const u8 as
                     *const libc::c_char,
                 b"../examples/multi-pointer.c\x00" as *const u8 as
                     *const libc::c_char, 291i32);
        return 1i32
    }
    state.xcursor =
        wlr_xcursor_theme_get_cursor(theme,
                                     b"left_ptr\x00" as *const u8 as
                                         *const libc::c_char);
    if state.xcursor.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to load left_ptr cursor\x00" as *const u8 as
                     *const libc::c_char,
                 b"../examples/multi-pointer.c\x00" as *const u8 as
                     *const libc::c_char, 296i32);
        return 1i32
    }
    if !wlr_backend_start(wlr) {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to start backend\x00" as *const u8 as
                     *const libc::c_char,
                 b"../examples/multi-pointer.c\x00" as *const u8 as
                     *const libc::c_char, 301i32);
        wlr_backend_destroy(wlr);
        exit(1i32);
    }
    wl_display_run(display);
    wl_display_destroy(display);
    let mut cursor: *mut sample_cursor = 0 as *mut sample_cursor;
    let mut tmp_cursor: *mut sample_cursor = 0 as *mut sample_cursor;
    cursor =
        (state.cursors.next as *mut libc::c_char).offset(-24) as
            *mut sample_cursor;
    tmp_cursor =
        ((*cursor).link.next as *mut libc::c_char).offset(-24) as
            *mut sample_cursor;
    while &mut (*cursor).link as *mut wl_list !=
              &mut state.cursors as *mut wl_list {
        cursor_destroy(cursor);
        cursor = tmp_cursor;
        tmp_cursor =
            ((*cursor).link.next as *mut libc::c_char).offset(-24) as
                *mut sample_cursor
    }
    let mut pointer: *mut sample_pointer = 0 as *mut sample_pointer;
    let mut tmp_pointer: *mut sample_pointer = 0 as *mut sample_pointer;
    pointer =
        (state.pointers.next as *mut libc::c_char).offset(-8) as
            *mut sample_pointer;
    tmp_pointer =
        ((*pointer).link.next as *mut libc::c_char).offset(-8) as
            *mut sample_pointer;
    while &mut (*pointer).link as *mut wl_list !=
              &mut state.pointers as *mut wl_list {
        free(pointer as *mut libc::c_void);
        pointer = tmp_pointer;
        tmp_pointer =
            ((*pointer).link.next as *mut libc::c_char).offset(-8) as
                *mut sample_pointer
    }
    wlr_xcursor_theme_destroy(theme);
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
