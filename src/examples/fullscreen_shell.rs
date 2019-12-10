use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    /* Generated by wayland-scanner 1.17.0 */
    pub type wl_client;
    pub type wl_global;
    pub type wlr_backend_impl;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    pub type wlr_output_impl;
    pub type wlr_output_layout_state;
    pub type tinywl_view;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
              __shortopts: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn setenv(__name: *const libc::c_char, __value: *const libc::c_char,
              __replace: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fork() -> __pid_t;
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
    fn wl_display_add_socket_auto(display: *mut wl_display)
     -> *const libc::c_char;
    #[no_mangle]
    fn wl_display_run(display: *mut wl_display);
    #[no_mangle]
    fn wl_display_destroy_clients(display: *mut wl_display);
    #[no_mangle]
    fn wlr_backend_get_renderer(backend: *mut wlr_backend)
     -> *mut wlr_renderer;
    #[no_mangle]
    fn wlr_backend_start(backend: *mut wlr_backend) -> bool;
    #[no_mangle]
    fn wlr_backend_autocreate(display: *mut wl_display,
                              create_renderer_func:
                                  wlr_renderer_create_func_t)
     -> *mut wlr_backend;
    #[no_mangle]
    fn wlr_renderer_begin(r: *mut wlr_renderer, width: libc::c_int,
                          height: libc::c_int);
    #[no_mangle]
    fn wlr_renderer_end(r: *mut wlr_renderer);
    #[no_mangle]
    fn wlr_renderer_clear(r: *mut wlr_renderer, color: *const libc::c_float);
    /* *
 * Renders the requested texture using the provided matrix.
 */
    #[no_mangle]
    fn wlr_render_texture_with_matrix(r: *mut wlr_renderer,
                                      texture: *mut wlr_texture,
                                      matrix: *const libc::c_float,
                                      alpha: libc::c_float) -> bool;
    #[no_mangle]
    fn wlr_renderer_init_wl_display(r: *mut wlr_renderer,
                                    wl_display: *mut wl_display);
    #[no_mangle]
    fn wlr_compositor_create(display: *mut wl_display,
                             renderer: *mut wlr_renderer)
     -> *mut wlr_compositor;
    #[no_mangle]
    fn wlr_fullscreen_shell_v1_create(display: *mut wl_display)
     -> *mut wlr_fullscreen_shell_v1;
    /* *
 * Create linux-dmabuf interface
 */
    #[no_mangle]
    fn wlr_linux_dmabuf_v1_create(display: *mut wl_display,
                                  renderer: *mut wlr_renderer)
     -> *mut wlr_linux_dmabuf_v1;
    /* * Shortcut for the various matrix operations involved in projecting the
 *  specified wlr_box onto a given orthographic projection with a given
 *  rotation. The result is written to mat, which can be applied to each
 *  coordinate of the box to get a new coordinate from [-1,1]. */
    #[no_mangle]
    fn wlr_matrix_project_box(mat: *mut libc::c_float, box_0: *const wlr_box,
                              transform: wl_output_transform,
                              rotation: libc::c_float,
                              projection: *const libc::c_float);
    #[no_mangle]
    fn wlr_output_create_global(output: *mut wlr_output);
    /* *
 * Sets the output mode. Enables the output if it's currently disabled.
 */
    #[no_mangle]
    fn wlr_output_set_mode(output: *mut wlr_output,
                           mode: *mut wlr_output_mode) -> bool;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* *
 * Creates a wlr_output_layout, which can be used to describing outputs in
 * physical space relative to one another, and perform various useful operations
 * on that state.
 */
    #[no_mangle]
    fn wlr_output_layout_create() -> *mut wlr_output_layout;
    /* *
 * Computes the transformed and scaled output resolution.
 */
    #[no_mangle]
    fn wlr_output_effective_resolution(output: *mut wlr_output,
                                       width: *mut libc::c_int,
                                       height: *mut libc::c_int);
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
 * Returns the transform that, when composed with `tr`, gives
 * `WL_OUTPUT_TRANSFORM_NORMAL`.
 */
    #[no_mangle]
    fn wlr_output_transform_invert(tr: wl_output_transform)
     -> wl_output_transform;
    /* *
 * Given x and y in layout coordinates, adjusts them to local output
 * coordinates relative to the given reference output.
 */
    /* *
 * Get the closest point on this layout from the given point from the reference
 * output. If reference is NULL, gets the closest point from the entire layout.
 */
    /* *
 * Get the box of the layout for the given reference output in layout
 * coordinates. If `reference` is NULL, the box will be for the extents of the
 * entire layout.
 */
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
    /* *
 * Get the texture of the buffer currently attached to this surface. Returns
 * NULL if no buffer is currently attached or if something went wrong with
 * uploading the buffer.
 */
    #[no_mangle]
    fn wlr_surface_get_texture(surface: *mut wlr_surface) -> *mut wlr_texture;
    #[no_mangle]
    fn wlr_surface_send_frame_done(surface: *mut wlr_surface,
                                   when: *const timespec);
    /* *
 * Call `iterator` on each surface in the surface tree, with the surface's
 * position relative to the root surface. The function is called from root to
 * leaves (in rendering order).
 */
    #[no_mangle]
    fn wlr_surface_for_each_surface(surface: *mut wlr_surface,
                                    iterator: wlr_surface_iterator_func_t,
                                    user_data: *mut libc::c_void);
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
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
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
pub type khronos_int32_t = int32_t;
pub type EGLint = khronos_int32_t;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
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
pub type wlr_renderer_create_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wlr_egl, _: EGLenum,
                                _: *mut libc::c_void, _: *mut EGLint,
                                _: EGLint) -> *mut wlr_renderer>;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_surface {
    pub resource: *mut wl_resource,
    pub renderer: *mut wlr_renderer,
    pub buffer: *mut wlr_buffer,
    pub sx: libc::c_int,
    pub sy: libc::c_int,
    pub buffer_damage: pixman_region32_t,
    pub opaque_region: pixman_region32_t,
    pub input_region: pixman_region32_t,
    pub current: wlr_surface_state,
    pub pending: wlr_surface_state,
    pub previous: wlr_surface_state,
    pub role: *const wlr_surface_role,
    pub role_data: *mut libc::c_void,
    pub events: C2RustUnnamed_2,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
    /* *
 * Get a subsurface from a surface. Can return NULL if the subsurface has been
 * destroyed.
 */
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub commit: wl_signal,
    pub new_subsurface: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_surface_role {
    pub name: *const libc::c_char,
    pub commit: Option<unsafe extern "C" fn(_: *mut wlr_surface) -> ()>,
    pub precommit: Option<unsafe extern "C" fn(_: *mut wlr_surface) -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_surface_state {
    pub committed: uint32_t,
    pub buffer_resource: *mut wl_resource,
    pub dx: int32_t,
    pub dy: int32_t,
    pub surface_damage: pixman_region32_t,
    pub buffer_damage: pixman_region32_t,
    pub opaque: pixman_region32_t,
    pub input: pixman_region32_t,
    pub transform: wl_output_transform,
    pub scale: int32_t,
    pub frame_callback_list: wl_list,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub buffer_width: libc::c_int,
    pub buffer_height: libc::c_int,
    pub buffer_destroy: wl_listener,
}
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
pub struct wlr_subcompositor {
    pub global: *mut wl_global,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub subcompositor: wlr_subcompositor,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_3,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
/* ZWP_FULLSCREEN_SHELL_V1_CAPABILITY_ENUM */
/* *
 * @ingroup iface_zwp_fullscreen_shell_v1
 * different method to set the surface fullscreen
 *
 * Hints to indicate to the compositor how to deal with a conflict
 * between the dimensions of the surface and the dimensions of the
 * output. The compositor is free to ignore this parameter.
 */
pub type zwp_fullscreen_shell_v1_present_method = libc::c_uint;
/* *
	 * scale the surface to the size of the output ignoring aspect ratio
	 */
pub const ZWP_FULLSCREEN_SHELL_V1_PRESENT_METHOD_STRETCH:
          zwp_fullscreen_shell_v1_present_method =
    4;
/* *
	 * scale the surface, preserving aspect ratio, to fully fill the output cropping if needed
	 */
pub const ZWP_FULLSCREEN_SHELL_V1_PRESENT_METHOD_ZOOM_CROP:
          zwp_fullscreen_shell_v1_present_method =
    3;
/* *
	 * scale the surface, preserving aspect ratio, to the largest size that will fit on the output
	 */
pub const ZWP_FULLSCREEN_SHELL_V1_PRESENT_METHOD_ZOOM:
          zwp_fullscreen_shell_v1_present_method =
    2;
/* *
	 * center the surface on the output
	 */
pub const ZWP_FULLSCREEN_SHELL_V1_PRESENT_METHOD_CENTER:
          zwp_fullscreen_shell_v1_present_method =
    1;
/* *
	 * no preference, apply default policy
	 */
pub const ZWP_FULLSCREEN_SHELL_V1_PRESENT_METHOD_DEFAULT:
          zwp_fullscreen_shell_v1_present_method =
    0;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_fullscreen_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2RustUnnamed_4,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub destroy: wl_signal,
    pub present_surface: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_fullscreen_shell_v1_present_surface_event {
    pub client: *mut wl_client,
    pub surface: *mut wlr_surface,
    pub method: zwp_fullscreen_shell_v1_present_method,
    pub output: *mut wlr_output,
    // can be NULL
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
    pub events: C2RustUnnamed_6,
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
    pub events: C2RustUnnamed_5,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
/* the protocol interface */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub events: C2RustUnnamed_7,
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub state: *mut wlr_output_layout_state,
    pub events: C2RustUnnamed_8,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
pub type wlr_surface_iterator_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wlr_surface, _: libc::c_int,
                                _: libc::c_int, _: *mut libc::c_void) -> ()>;
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
pub type wlr_log_func_t
    =
    Option<unsafe extern "C" fn(_: wlr_log_importance, _: *const libc::c_char,
                                _: ::std::ffi::VaList) -> ()>;
/* *
 * A minimal fullscreen-shell server. It only supports rendering.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct fullscreen_server {
    pub wl_display: *mut wl_display,
    pub backend: *mut wlr_backend,
    pub renderer: *mut wlr_renderer,
    pub fullscreen_shell: *mut wlr_fullscreen_shell_v1,
    pub present_surface: wl_listener,
    pub output_layout: *mut wlr_output_layout,
    pub outputs: wl_list,
    pub new_output: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct fullscreen_output {
    pub link: wl_list,
    pub server: *mut fullscreen_server,
    pub wlr_output: *mut wlr_output,
    pub surface: *mut wlr_surface,
    pub surface_destroy: wl_listener,
    pub frame: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct render_data {
    pub output: *mut wlr_output,
    pub renderer: *mut wlr_renderer,
    pub view: *mut tinywl_view,
    pub when: *mut timespec,
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
unsafe extern "C" fn render_surface(mut surface: *mut wlr_surface,
                                    mut sx: libc::c_int, mut sy: libc::c_int,
                                    mut data: *mut libc::c_void) {
    let mut rdata: *mut render_data = data as *mut render_data;
    let mut output: *mut wlr_output = (*rdata).output;
    let mut texture: *mut wlr_texture = wlr_surface_get_texture(surface);
    if texture.is_null() { return }
    let mut box_0: wlr_box =
        {
            let mut init =
                wlr_box{x:
                            (sx as libc::c_float * (*output).scale) as
                                libc::c_int,
                        y:
                            (sy as libc::c_float * (*output).scale) as
                                libc::c_int,
                        width:
                            ((*surface).current.width as libc::c_float *
                                 (*output).scale) as libc::c_int,
                        height:
                            ((*surface).current.height as libc::c_float *
                                 (*output).scale) as libc::c_int,};
            init
        };
    let mut matrix: [libc::c_float; 9] = [0.; 9];
    let mut transform: wl_output_transform =
        wlr_output_transform_invert((*surface).current.transform);
    wlr_matrix_project_box(matrix.as_mut_ptr(), &mut box_0, transform,
                           0i32 as libc::c_float,
                           (*output).transform_matrix.as_mut_ptr() as
                               *const libc::c_float);
    wlr_render_texture_with_matrix((*rdata).renderer, texture,
                                   matrix.as_mut_ptr() as
                                       *const libc::c_float,
                                   1i32 as libc::c_float);
    wlr_surface_send_frame_done(surface, (*rdata).when);
}
unsafe extern "C" fn output_handle_frame(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut output: *mut fullscreen_output =
        (listener as *mut libc::c_char).offset(-64) as *mut fullscreen_output;
    let mut renderer: *mut wlr_renderer = (*(*output).server).renderer;
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    clock_gettime(1i32, &mut now);
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    wlr_output_effective_resolution((*output).wlr_output, &mut width,
                                    &mut height);
    if !wlr_output_attach_render((*output).wlr_output, 0 as *mut libc::c_int)
       {
        return
    }
    wlr_renderer_begin(renderer, width, height);
    let mut color: [libc::c_float; 4] =
        [0.3f64 as libc::c_float, 0.3f64 as libc::c_float,
         0.3f64 as libc::c_float, 1.0f64 as libc::c_float];
    wlr_renderer_clear(renderer, color.as_mut_ptr() as *const libc::c_float);
    if !(*output).surface.is_null() {
        let mut rdata: render_data =
            {
                let mut init =
                    render_data{output: (*output).wlr_output,
                                renderer: renderer,
                                view: 0 as *mut tinywl_view,
                                when: &mut now,};
                init
            };
        wlr_surface_for_each_surface((*output).surface,
                                     Some(render_surface as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_surface,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> ()),
                                     &mut rdata as *mut render_data as
                                         *mut libc::c_void);
    }
    wlr_renderer_end(renderer);
    wlr_output_commit((*output).wlr_output);
}
unsafe extern "C" fn output_handle_surface_destroy(mut listener:
                                                       *mut wl_listener,
                                                   mut data:
                                                       *mut libc::c_void) {
    let mut output: *mut fullscreen_output =
        (listener as *mut libc::c_char).offset(-40) as *mut fullscreen_output;
    output_set_surface(output, 0 as *mut wlr_surface);
}
unsafe extern "C" fn output_set_surface(mut output: *mut fullscreen_output,
                                        mut surface: *mut wlr_surface) {
    if (*output).surface == surface { return }
    if !(*output).surface.is_null() {
        wl_list_remove(&mut (*output).surface_destroy.link);
        (*output).surface = 0 as *mut wlr_surface
    }
    if !surface.is_null() {
        (*output).surface_destroy.notify =
            Some(output_handle_surface_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ());
        wl_signal_add(&mut (*surface).events.destroy,
                      &mut (*output).surface_destroy);
        (*output).surface = surface
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Presenting surface %p on output %s\x00" as *const u8 as
                 *const libc::c_char,
             b"../examples/fullscreen-shell.c\x00" as *const u8 as
                 *const libc::c_char, 142i32, surface,
             (*(*output).wlr_output).name.as_mut_ptr());
}
unsafe extern "C" fn server_handle_new_output(mut listener: *mut wl_listener,
                                              mut data: *mut libc::c_void) {
    let mut server: *mut fullscreen_server =
        (listener as *mut libc::c_char).offset(-80) as *mut fullscreen_server;
    let mut wlr_output: *mut wlr_output = data as *mut wlr_output;
    if wl_list_empty(&mut (*wlr_output).modes) == 0 {
        let mut mode: *mut wlr_output_mode =
            ((*wlr_output).modes.prev as *mut libc::c_char).offset(-16) as
                *mut wlr_output_mode;
        wlr_output_set_mode(wlr_output, mode);
    }
    let mut output: *mut fullscreen_output =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<fullscreen_output>() as libc::c_ulong) as
            *mut fullscreen_output;
    (*output).wlr_output = wlr_output;
    (*output).server = server;
    (*output).frame.notify =
        Some(output_handle_frame as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*wlr_output).events.frame, &mut (*output).frame);
    wl_list_insert(&mut (*server).outputs, &mut (*output).link);
    wlr_output_layout_add_auto((*server).output_layout, wlr_output);
    wlr_output_create_global(wlr_output);
}
unsafe extern "C" fn server_handle_present_surface(mut listener:
                                                       *mut wl_listener,
                                                   mut data:
                                                       *mut libc::c_void) {
    let mut server: *mut fullscreen_server =
        (listener as *mut libc::c_char).offset(-32) as *mut fullscreen_server;
    let mut event: *mut wlr_fullscreen_shell_v1_present_surface_event =
        data as *mut wlr_fullscreen_shell_v1_present_surface_event;
    let mut output: *mut fullscreen_output = 0 as *mut fullscreen_output;
    output =
        ((*server).outputs.next as *mut libc::c_char).offset(-0) as
            *mut fullscreen_output;
    while &mut (*output).link as *mut wl_list !=
              &mut (*server).outputs as *mut wl_list {
        if (*event).output.is_null() ||
               (*event).output == (*output).wlr_output {
            output_set_surface(output, (*event).surface);
        }
        output =
            ((*output).link.next as *mut libc::c_char).offset(-0) as
                *mut fullscreen_output
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    wlr_log_init(WLR_DEBUG, None);
    let mut startup_cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    loop  {
        c =
            getopt(argc, argv as *const *mut libc::c_char,
                   b"s:\x00" as *const u8 as *const libc::c_char);
        if !(c != -1i32) { break ; }
        match c {
            115 => { startup_cmd = optarg }
            _ => {
                printf(b"usage: %s [-s startup-command]\n\x00" as *const u8 as
                           *const libc::c_char, *argv.offset(0));
                return 1i32
            }
        }
    }
    if optind < argc {
        printf(b"usage: %s [-s startup-command]\n\x00" as *const u8 as
                   *const libc::c_char, *argv.offset(0));
        return 1i32
    }
    let mut server: fullscreen_server =
        {
            let mut init =
                fullscreen_server{wl_display: 0 as *mut wl_display,
                                  backend: 0 as *mut wlr_backend,
                                  renderer: 0 as *mut wlr_renderer,
                                  fullscreen_shell:
                                      0 as *mut wlr_fullscreen_shell_v1,
                                  present_surface:
                                      wl_listener{link:
                                                      wl_list{prev:
                                                                  0 as
                                                                      *mut wl_list,
                                                              next:
                                                                  0 as
                                                                      *mut wl_list,},
                                                  notify: None,},
                                  output_layout: 0 as *mut wlr_output_layout,
                                  outputs:
                                      wl_list{prev: 0 as *mut wl_list,
                                              next: 0 as *mut wl_list,},
                                  new_output:
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
    server.wl_display = wl_display_create();
    server.backend = wlr_backend_autocreate(server.wl_display, None);
    server.renderer = wlr_backend_get_renderer(server.backend);
    wlr_renderer_init_wl_display(server.renderer, server.wl_display);
    wlr_compositor_create(server.wl_display, server.renderer);
    wlr_linux_dmabuf_v1_create(server.wl_display, server.renderer);
    server.output_layout = wlr_output_layout_create();
    wl_list_init(&mut server.outputs);
    server.new_output.notify =
        Some(server_handle_new_output as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*server.backend).events.new_output,
                  &mut server.new_output);
    server.fullscreen_shell =
        wlr_fullscreen_shell_v1_create(server.wl_display);
    server.present_surface.notify =
        Some(server_handle_present_surface as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*server.fullscreen_shell).events.present_surface,
                  &mut server.present_surface);
    let mut socket: *const libc::c_char =
        wl_display_add_socket_auto(server.wl_display);
    if socket.is_null() { wl_display_destroy(server.wl_display); return 1i32 }
    if !wlr_backend_start(server.backend) {
        wl_display_destroy(server.wl_display);
        return 1i32
    }
    setenv(b"WAYLAND_DISPLAY\x00" as *const u8 as *const libc::c_char, socket,
           1i32);
    if !startup_cmd.is_null() {
        if fork() == 0i32 {
            execl(b"/bin/sh\x00" as *const u8 as *const libc::c_char,
                  b"/bin/sh\x00" as *const u8 as *const libc::c_char,
                  b"-c\x00" as *const u8 as *const libc::c_char, startup_cmd,
                  0 as *mut libc::c_void);
        }
    }
    _wlr_log(WLR_INFO,
             b"[%s:%d] Running Wayland compositor on WAYLAND_DISPLAY=%s\x00"
                 as *const u8 as *const libc::c_char,
             b"../examples/fullscreen-shell.c\x00" as *const u8 as
                 *const libc::c_char, 242i32, socket);
    wl_display_run(server.wl_display);
    wl_display_destroy_clients(server.wl_display);
    wl_display_destroy(server.wl_display);
    return 0i32;
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
