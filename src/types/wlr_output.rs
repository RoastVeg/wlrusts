use libc;
extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    pub type wlr_backend_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn ceilf(_: libc::c_float) -> libc::c_float;
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
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_event_loop_add_idle(loop_0: *mut wl_event_loop,
                              func: wl_event_loop_idle_func_t,
                              data: *mut libc::c_void)
     -> *mut wl_event_source;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn wl_display_add_destroy_listener(display: *mut wl_display,
                                       listener: *mut wl_listener);
    #[no_mangle]
    fn wl_global_create(display: *mut wl_display,
                        interface: *const wl_interface, version: libc::c_int,
                        data: *mut libc::c_void, bind: wl_global_bind_func_t)
     -> *mut wl_global;
    #[no_mangle]
    fn wl_global_destroy(global: *mut wl_global);
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
    #[no_mangle]
    fn wl_resource_create(client: *mut wl_client,
                          interface: *const wl_interface,
                          version: libc::c_int, id: uint32_t)
     -> *mut wl_resource;
    #[no_mangle]
    fn wl_resource_set_implementation(resource: *mut wl_resource,
                                      implementation: *const libc::c_void,
                                      data: *mut libc::c_void,
                                      destroy: wl_resource_destroy_func_t);
    #[no_mangle]
    fn wl_resource_destroy(resource: *mut wl_resource);
    #[no_mangle]
    fn wl_resource_get_link(resource: *mut wl_resource) -> *mut wl_list;
    #[no_mangle]
    fn wl_resource_from_link(resource: *mut wl_list) -> *mut wl_resource;
    #[no_mangle]
    fn wl_resource_set_user_data(resource: *mut wl_resource,
                                 data: *mut libc::c_void);
    #[no_mangle]
    fn wl_resource_get_user_data(resource: *mut wl_resource)
     -> *mut libc::c_void;
    #[no_mangle]
    fn wl_resource_get_version(resource: *mut wl_resource) -> libc::c_int;
    #[no_mangle]
    fn wl_resource_instance_of(resource: *mut wl_resource,
                               interface: *const wl_interface,
                               implementation: *const libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn wlr_buffer_unref(buffer: *mut wlr_buffer);
    #[no_mangle]
    fn wlr_buffer_ref(buffer: *mut wlr_buffer) -> *mut wlr_buffer;
    #[no_mangle]
    fn wlr_box_transform(dest: *mut wlr_box, box_0: *const wlr_box,
                         transform: wl_output_transform, width: libc::c_int,
                         height: libc::c_int);
    #[no_mangle]
    fn wlr_box_intersection(dest: *mut wlr_box, box_a: *const wlr_box,
                            box_b: *const wlr_box) -> bool;
    /* *
 * Obtains the wlr_renderer reference this backend is using.
 */
    #[no_mangle]
    fn wlr_backend_get_renderer(backend: *mut wlr_backend)
     -> *mut wlr_renderer;
    /* *
 * Returns the clock used by the backend for presentation feedback.
 */
    #[no_mangle]
    fn wlr_backend_get_presentation_clock(backend: *mut wlr_backend)
     -> clockid_t;
    #[no_mangle]
    static wl_output_interface: wl_interface;
    #[no_mangle]
    fn pixman_region32_union_rect(dest: *mut pixman_region32_t,
                                  source: *mut pixman_region32_t,
                                  x: libc::c_int, y: libc::c_int,
                                  width: libc::c_uint, height: libc::c_uint)
     -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_intersect_rect(dest: *mut pixman_region32_t,
                                      source: *mut pixman_region32_t,
                                      x: libc::c_int, y: libc::c_int,
                                      width: libc::c_uint,
                                      height: libc::c_uint) -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_intersect(new_reg: *mut pixman_region32_t,
                                 reg1: *mut pixman_region32_t,
                                 reg2: *mut pixman_region32_t)
     -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_fini(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_init(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_not_empty(region: *mut pixman_region32_t)
     -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_rectangles(region: *mut pixman_region32_t,
                                  n_rects: *mut libc::c_int)
     -> *mut pixman_box32_t;
    #[no_mangle]
    fn pixman_region32_clear(region: *mut pixman_region32_t);
    #[no_mangle]
    fn wlr_texture_destroy(texture: *mut wlr_texture);
    #[no_mangle]
    fn wlr_renderer_scissor(r: *mut wlr_renderer, box_0: *mut wlr_box);
    #[no_mangle]
    fn wlr_render_texture_with_matrix(r: *mut wlr_renderer,
                                      texture: *mut wlr_texture,
                                      matrix: *const libc::c_float,
                                      alpha: libc::c_float) -> bool;
    #[no_mangle]
    fn wlr_texture_from_pixels(renderer: *mut wlr_renderer,
                               wl_fmt: wl_shm_format, stride: uint32_t,
                               width: uint32_t, height: uint32_t,
                               data: *const libc::c_void) -> *mut wlr_texture;
    /* * Writes a 2D orthographic projection matrix to mat of (width, height) with a
 *  specified wl_output_transform*/
    #[no_mangle]
    fn wlr_matrix_projection(mat: *mut libc::c_float, width: libc::c_int,
                             height: libc::c_int,
                             transform: wl_output_transform);
    /* * Shortcut for the various matrix operations involved in projecting the
 *  specified wlr_box onto a given orthographic projection with a given
 *  rotation. The result is written to mat, which can be applied to each
 *  coordinate of the box to get a new coordinate from [-1,1]. */
    #[no_mangle]
    fn wlr_matrix_project_box(mat: *mut libc::c_float, box_0: *const wlr_box,
                              transform: wl_output_transform,
                              rotation: libc::c_float,
                              projection: *const libc::c_float);
    /* *
 * Create a new surface resource with the provided new ID. If `resource_list`
 * is non-NULL, adds the surface's resource to the list.
 */
    /* *
 * Set the lifetime role for this surface. Returns 0 on success or -1 if the
 * role cannot be set.
 */
    /* *
 * Whether or not this surface currently has an attached buffer. A surface has
 * an attached buffer when it commits with a non-null buffer in its pending
 * state. A surface will not have a buffer if it has never committed one, has
 * committed a null buffer, or something went wrong with uploading the buffer.
 */
    /* *
 * Get the texture of the buffer currently attached to this surface. Returns
 * NULL if no buffer is currently attached or if something went wrong with
 * uploading the buffer.
 */
    /* *
 * Create a new subsurface resource with the provided new ID. If `resource_list`
 * is non-NULL, adds the subsurface's resource to the list.
 */
    /* *
 * Get the root of the subsurface tree for this surface.
 */
    /* *
 * Check if the surface accepts input events at the given surface-local
 * coordinates. Does not check the surface's subsurfaces.
 */
    /* *
 * Find a surface in this surface's tree that accepts input events at the given
 * surface-local coordinates. Returns the surface and coordinates in the leaf
 * surface coordinate system or NULL if no surface is found at that location.
 */
    #[no_mangle]
    fn wlr_surface_send_leave(surface: *mut wlr_surface,
                              output: *mut wlr_output);
    #[no_mangle]
    fn wlr_surface_has_buffer(surface: *mut wlr_surface) -> bool;
    #[no_mangle]
    fn wlr_surface_send_enter(surface: *mut wlr_surface,
                              output: *mut wlr_output);
    #[no_mangle]
    fn wlr_surface_get_texture(surface: *mut wlr_surface) -> *mut wlr_texture;
    #[no_mangle]
    fn wlr_surface_send_frame_done(surface: *mut wlr_surface,
                                   when: *const timespec);
    // Will log all messages less than or equal to `verbosity`
// If `callback` is NULL, wlr will use its default logger.
// The function can be called multiple times to update the verbosity or
// callback function.
    // Returns the log verbosity provided to wlr_log_init
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type wl_event_loop_idle_func_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
pub type wl_global_bind_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_client, _: *mut libc::c_void,
                                _: uint32_t, _: uint32_t) -> ()>;
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
pub type pixman_bool_t = libc::c_int;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_renderer_impl {
    pub begin: Option<unsafe extern "C" fn(_: *mut wlr_renderer, _: uint32_t,
                                           _: uint32_t) -> ()>,
    pub end: Option<unsafe extern "C" fn(_: *mut wlr_renderer) -> ()>,
    pub clear: Option<unsafe extern "C" fn(_: *mut wlr_renderer,
                                           _: *const libc::c_float) -> ()>,
    pub scissor: Option<unsafe extern "C" fn(_: *mut wlr_renderer,
                                             _: *mut wlr_box) -> ()>,
    pub render_texture_with_matrix: Option<unsafe extern "C" fn(_:
                                                                    *mut wlr_renderer,
                                                                _:
                                                                    *mut wlr_texture,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    libc::c_float)
                                               -> bool>,
    pub render_quad_with_matrix: Option<unsafe extern "C" fn(_:
                                                                 *mut wlr_renderer,
                                                             _:
                                                                 *const libc::c_float,
                                                             _:
                                                                 *const libc::c_float)
                                            -> ()>,
    pub render_ellipse_with_matrix: Option<unsafe extern "C" fn(_:
                                                                    *mut wlr_renderer,
                                                                _:
                                                                    *const libc::c_float,
                                                                _:
                                                                    *const libc::c_float)
                                               -> ()>,
    pub formats: Option<unsafe extern "C" fn(_: *mut wlr_renderer,
                                             _: *mut size_t)
                            -> *const wl_shm_format>,
    pub format_supported: Option<unsafe extern "C" fn(_: *mut wlr_renderer,
                                                      _: wl_shm_format)
                                     -> bool>,
    pub resource_is_wl_drm_buffer: Option<unsafe extern "C" fn(_:
                                                                   *mut wlr_renderer,
                                                               _:
                                                                   *mut wl_resource)
                                              -> bool>,
    pub wl_drm_buffer_get_size: Option<unsafe extern "C" fn(_:
                                                                *mut wlr_renderer,
                                                            _:
                                                                *mut wl_resource,
                                                            _:
                                                                *mut libc::c_int,
                                                            _:
                                                                *mut libc::c_int)
                                           -> ()>,
    pub get_dmabuf_formats: Option<unsafe extern "C" fn(_: *mut wlr_renderer)
                                       -> *const wlr_drm_format_set>,
    pub preferred_read_format: Option<unsafe extern "C" fn(_:
                                                               *mut wlr_renderer)
                                          -> wl_shm_format>,
    pub read_pixels: Option<unsafe extern "C" fn(_: *mut wlr_renderer,
                                                 _: wl_shm_format,
                                                 _: *mut uint32_t,
                                                 _: uint32_t, _: uint32_t,
                                                 _: uint32_t, _: uint32_t,
                                                 _: uint32_t, _: uint32_t,
                                                 _: uint32_t,
                                                 _: *mut libc::c_void)
                                -> bool>,
    pub texture_from_pixels: Option<unsafe extern "C" fn(_: *mut wlr_renderer,
                                                         _: wl_shm_format,
                                                         _: uint32_t,
                                                         _: uint32_t,
                                                         _: uint32_t,
                                                         _:
                                                             *const libc::c_void)
                                        -> *mut wlr_texture>,
    pub texture_from_wl_drm: Option<unsafe extern "C" fn(_: *mut wlr_renderer,
                                                         _: *mut wl_resource)
                                        -> *mut wlr_texture>,
    pub texture_from_dmabuf: Option<unsafe extern "C" fn(_: *mut wlr_renderer,
                                                         _:
                                                             *mut wlr_dmabuf_attributes)
                                        -> *mut wlr_texture>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_renderer) -> ()>,
    pub init_wl_display: Option<unsafe extern "C" fn(_: *mut wlr_renderer,
                                                     _: *mut wl_display)
                                    -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_texture {
    pub impl_0: *const wlr_texture_impl,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_texture_impl {
    pub get_size: Option<unsafe extern "C" fn(_: *mut wlr_texture,
                                              _: *mut libc::c_int,
                                              _: *mut libc::c_int) -> ()>,
    pub is_opaque: Option<unsafe extern "C" fn(_: *mut wlr_texture) -> bool>,
    pub write_pixels: Option<unsafe extern "C" fn(_: *mut wlr_texture,
                                                  _: uint32_t, _: uint32_t,
                                                  _: uint32_t, _: uint32_t,
                                                  _: uint32_t, _: uint32_t,
                                                  _: uint32_t,
                                                  _: *const libc::c_void)
                                 -> bool>,
    pub to_dmabuf: Option<unsafe extern "C" fn(_: *mut wlr_texture,
                                               _: *mut wlr_dmabuf_attributes)
                              -> bool>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_texture) -> ()>,
    /* *
 * Create a new texture from raw pixel data. `stride` is in bytes. The returned
 * texture is mutable.
 */
    /* *
 * Create a new texture from a wl_drm resource. The returned texture is
 * immutable.
 */
    /* *
 * Create a new texture from a DMA-BUF. The returned texture is immutable.
 */
    /* *
 * Get the texture width and height.
 */
    /* *
 * Returns true if this texture is using a fully opaque format.
 */
    /* *
  * Update a texture with raw pixels. The texture must be mutable, and the input
  * data must have the same pixel format that the texture was created with.
  */
    /* *
 * Destroys this wlr_texture.
 */
}
pub type wl_shm_format = libc::c_uint;
pub const WL_SHM_FORMAT_YVU444: wl_shm_format = 875714137;
pub const WL_SHM_FORMAT_YUV444: wl_shm_format = 875713881;
pub const WL_SHM_FORMAT_YVU422: wl_shm_format = 909203033;
pub const WL_SHM_FORMAT_YUV422: wl_shm_format = 909202777;
pub const WL_SHM_FORMAT_YVU420: wl_shm_format = 842094169;
pub const WL_SHM_FORMAT_YUV420: wl_shm_format = 842093913;
pub const WL_SHM_FORMAT_YVU411: wl_shm_format = 825316953;
pub const WL_SHM_FORMAT_YUV411: wl_shm_format = 825316697;
pub const WL_SHM_FORMAT_YVU410: wl_shm_format = 961893977;
pub const WL_SHM_FORMAT_YUV410: wl_shm_format = 961959257;
pub const WL_SHM_FORMAT_NV61: wl_shm_format = 825644622;
pub const WL_SHM_FORMAT_NV16: wl_shm_format = 909203022;
pub const WL_SHM_FORMAT_NV21: wl_shm_format = 825382478;
pub const WL_SHM_FORMAT_NV12: wl_shm_format = 842094158;
pub const WL_SHM_FORMAT_AYUV: wl_shm_format = 1448433985;
pub const WL_SHM_FORMAT_VYUY: wl_shm_format = 1498765654;
pub const WL_SHM_FORMAT_UYVY: wl_shm_format = 1498831189;
pub const WL_SHM_FORMAT_YVYU: wl_shm_format = 1431918169;
pub const WL_SHM_FORMAT_YUYV: wl_shm_format = 1448695129;
pub const WL_SHM_FORMAT_BGRA1010102: wl_shm_format = 808665410;
pub const WL_SHM_FORMAT_RGBA1010102: wl_shm_format = 808665426;
pub const WL_SHM_FORMAT_ABGR2101010: wl_shm_format = 808665665;
pub const WL_SHM_FORMAT_ARGB2101010: wl_shm_format = 808669761;
pub const WL_SHM_FORMAT_BGRX1010102: wl_shm_format = 808671298;
pub const WL_SHM_FORMAT_RGBX1010102: wl_shm_format = 808671314;
pub const WL_SHM_FORMAT_XBGR2101010: wl_shm_format = 808665688;
pub const WL_SHM_FORMAT_XRGB2101010: wl_shm_format = 808669784;
pub const WL_SHM_FORMAT_BGRA8888: wl_shm_format = 875708738;
pub const WL_SHM_FORMAT_RGBA8888: wl_shm_format = 875708754;
pub const WL_SHM_FORMAT_ABGR8888: wl_shm_format = 875708993;
pub const WL_SHM_FORMAT_BGRX8888: wl_shm_format = 875714626;
pub const WL_SHM_FORMAT_RGBX8888: wl_shm_format = 875714642;
pub const WL_SHM_FORMAT_XBGR8888: wl_shm_format = 875709016;
pub const WL_SHM_FORMAT_BGR888: wl_shm_format = 875710274;
pub const WL_SHM_FORMAT_RGB888: wl_shm_format = 875710290;
pub const WL_SHM_FORMAT_BGR565: wl_shm_format = 909199170;
pub const WL_SHM_FORMAT_RGB565: wl_shm_format = 909199186;
pub const WL_SHM_FORMAT_BGRA5551: wl_shm_format = 892420418;
pub const WL_SHM_FORMAT_RGBA5551: wl_shm_format = 892420434;
pub const WL_SHM_FORMAT_ABGR1555: wl_shm_format = 892420673;
pub const WL_SHM_FORMAT_ARGB1555: wl_shm_format = 892424769;
pub const WL_SHM_FORMAT_BGRX5551: wl_shm_format = 892426306;
pub const WL_SHM_FORMAT_RGBX5551: wl_shm_format = 892426322;
pub const WL_SHM_FORMAT_XBGR1555: wl_shm_format = 892420696;
pub const WL_SHM_FORMAT_XRGB1555: wl_shm_format = 892424792;
pub const WL_SHM_FORMAT_BGRA4444: wl_shm_format = 842088770;
pub const WL_SHM_FORMAT_RGBA4444: wl_shm_format = 842088786;
pub const WL_SHM_FORMAT_ABGR4444: wl_shm_format = 842089025;
pub const WL_SHM_FORMAT_ARGB4444: wl_shm_format = 842093121;
pub const WL_SHM_FORMAT_BGRX4444: wl_shm_format = 842094658;
pub const WL_SHM_FORMAT_RGBX4444: wl_shm_format = 842094674;
pub const WL_SHM_FORMAT_XBGR4444: wl_shm_format = 842089048;
pub const WL_SHM_FORMAT_XRGB4444: wl_shm_format = 842093144;
pub const WL_SHM_FORMAT_BGR233: wl_shm_format = 944916290;
pub const WL_SHM_FORMAT_RGB332: wl_shm_format = 943867730;
pub const WL_SHM_FORMAT_C8: wl_shm_format = 538982467;
pub const WL_SHM_FORMAT_XRGB8888: wl_shm_format = 1;
pub const WL_SHM_FORMAT_ARGB8888: wl_shm_format = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
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
pub type wl_output_mode = libc::c_uint;
pub const WL_OUTPUT_MODE_PREFERRED: wl_output_mode = 2;
pub const WL_OUTPUT_MODE_CURRENT: wl_output_mode = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_output_interface {
    pub release: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
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
    pub events: C2RustUnnamed_1,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub destroy: wl_signal,
}
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
pub struct wlr_output_event_precommit {
    pub output: *mut wlr_output,
    pub when: *mut timespec,
}
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
#[inline(always)]
unsafe extern "C" fn __tg_ceil(mut __x: libc::c_float) -> libc::c_float {
    return ceilf(__x);
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
#[inline]
unsafe extern "C" fn wl_output_send_scale(mut resource_: *mut wl_resource,
                                          mut factor: int32_t) {
    wl_resource_post_event(resource_, 3i32 as uint32_t, factor);
}
#[inline]
unsafe extern "C" fn wl_output_send_done(mut resource_: *mut wl_resource) {
    wl_resource_post_event(resource_, 2i32 as uint32_t);
}
#[inline]
unsafe extern "C" fn wl_output_send_mode(mut resource_: *mut wl_resource,
                                         mut flags: uint32_t,
                                         mut width: int32_t,
                                         mut height: int32_t,
                                         mut refresh: int32_t) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, flags, width, height,
                           refresh);
}
#[inline]
unsafe extern "C" fn wl_output_send_geometry(mut resource_: *mut wl_resource,
                                             mut x: int32_t, mut y: int32_t,
                                             mut physical_width: int32_t,
                                             mut physical_height: int32_t,
                                             mut subpixel: int32_t,
                                             mut make: *const libc::c_char,
                                             mut model: *const libc::c_char,
                                             mut transform: int32_t) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, x, y, physical_width,
                           physical_height, subpixel, make, model, transform);
}
unsafe extern "C" fn send_geometry(mut resource: *mut wl_resource) {
    let mut output: *mut wlr_output = wlr_output_from_resource(resource);
    wl_output_send_geometry(resource, 0i32, 0i32, (*output).phys_width,
                            (*output).phys_height,
                            (*output).subpixel as int32_t,
                            (*output).make.as_mut_ptr(),
                            (*output).model.as_mut_ptr(),
                            (*output).transform as int32_t);
}
unsafe extern "C" fn send_all_modes(mut resource: *mut wl_resource) {
    let mut output: *mut wlr_output = wlr_output_from_resource(resource);
    let mut mode: *mut wlr_output_mode = 0 as *mut wlr_output_mode;
    mode =
        ((*output).modes.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_mode;
    while &mut (*mode).link as *mut wl_list !=
              &mut (*output).modes as *mut wl_list {
        let mut flags: uint32_t = 0i32 as uint32_t;
        if (*mode).preferred {
            flags |= WL_OUTPUT_MODE_PREFERRED as libc::c_int as libc::c_uint
        }
        if (*output).current_mode == mode {
            flags |= WL_OUTPUT_MODE_CURRENT as libc::c_int as libc::c_uint
        }
        wl_output_send_mode(resource, flags, (*mode).width, (*mode).height,
                            (*mode).refresh);
        mode =
            ((*mode).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_mode
    }
    if wl_list_empty(&mut (*output).modes) != 0 {
        // Output has no mode, send the current width/height
        wl_output_send_mode(resource,
                            WL_OUTPUT_MODE_CURRENT as libc::c_int as uint32_t,
                            (*output).width, (*output).height,
                            (*output).refresh);
    };
}
unsafe extern "C" fn send_current_mode(mut resource: *mut wl_resource) {
    let mut output: *mut wlr_output = wlr_output_from_resource(resource);
    if !(*output).current_mode.is_null() {
        let mut mode: *mut wlr_output_mode = (*output).current_mode;
        let mut flags: uint32_t =
            WL_OUTPUT_MODE_CURRENT as libc::c_int as uint32_t;
        if (*mode).preferred {
            flags |= WL_OUTPUT_MODE_PREFERRED as libc::c_int as libc::c_uint
        }
        wl_output_send_mode(resource, flags, (*mode).width, (*mode).height,
                            (*mode).refresh);
    } else {
        // Output has no mode
        wl_output_send_mode(resource,
                            WL_OUTPUT_MODE_CURRENT as libc::c_int as uint32_t,
                            (*output).width, (*output).height,
                            (*output).refresh);
    };
}
unsafe extern "C" fn send_scale(mut resource: *mut wl_resource) {
    let mut output: *mut wlr_output = wlr_output_from_resource(resource);
    let mut version: uint32_t = wl_resource_get_version(resource) as uint32_t;
    if version >= 2i32 as libc::c_uint {
        wl_output_send_scale(resource,
                             __tg_ceil((*output).scale) as uint32_t as
                                 int32_t);
    };
}
unsafe extern "C" fn send_done(mut resource: *mut wl_resource) {
    let mut version: uint32_t = wl_resource_get_version(resource) as uint32_t;
    if version >= 2i32 as libc::c_uint { wl_output_send_done(resource); };
}
unsafe extern "C" fn output_handle_resource_destroy(mut resource:
                                                        *mut wl_resource) {
    wl_list_remove(wl_resource_get_link(resource));
}
unsafe extern "C" fn output_handle_release(mut client: *mut wl_client,
                                           mut resource: *mut wl_resource) {
    wl_resource_destroy(resource);
}
static mut output_impl: wl_output_interface =
    unsafe {
        {
            let mut init =
                wl_output_interface{release:
                                        Some(output_handle_release as
                                                 unsafe extern "C" fn(_:
                                                                          *mut wl_client,
                                                                      _:
                                                                          *mut wl_resource)
                                                     -> ()),};
            init
        }
    };
unsafe extern "C" fn output_bind(mut wl_client: *mut wl_client,
                                 mut data: *mut libc::c_void,
                                 mut version: uint32_t, mut id: uint32_t) {
    let mut output: *mut wlr_output = data as *mut wlr_output;
    let mut resource: *mut wl_resource =
        wl_resource_create(wl_client, &wl_output_interface,
                           version as libc::c_int, id);
    if resource.is_null() { wl_client_post_no_memory(wl_client); return }
    wl_resource_set_implementation(resource,
                                   &output_impl as *const wl_output_interface
                                       as *const libc::c_void,
                                   output as *mut libc::c_void,
                                   Some(output_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_insert(&mut (*output).resources, wl_resource_get_link(resource));
    send_geometry(resource);
    send_all_modes(resource);
    send_scale(resource);
    send_done(resource);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_create_global(mut output:
                                                      *mut wlr_output) {
    if !(*output).global.is_null() { return }
    (*output).global =
        wl_global_create((*output).display, &wl_output_interface, 3i32,
                         output as *mut libc::c_void,
                         Some(output_bind as
                                  unsafe extern "C" fn(_: *mut wl_client,
                                                       _: *mut libc::c_void,
                                                       _: uint32_t,
                                                       _: uint32_t) -> ()));
    if (*output).global.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate wl_output global\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_output.c\x00" as *const u8 as
                     *const libc::c_char, 124i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_destroy_global(mut output:
                                                       *mut wlr_output) {
    if (*output).global.is_null() { return }
    // Make all output resources inert
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    let mut tmp: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    tmp = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*output).resources.next);
    tmp = wl_resource_from_link((*(*output).resources.next).next);
    while wl_resource_get_link(resource) !=
              &mut (*output).resources as *mut wl_list {
        wl_resource_set_user_data(resource, 0 as *mut libc::c_void);
        wl_list_remove(wl_resource_get_link(resource));
        wl_list_init(wl_resource_get_link(resource));
        resource = tmp;
        tmp = wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    wl_global_destroy((*output).global);
    (*output).global = 0 as *mut wl_global;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_update_enabled(mut output:
                                                       *mut wlr_output,
                                                   mut enabled: bool) {
    if (*output).enabled as libc::c_int == enabled as libc::c_int { return }
    (*output).enabled = enabled;
    wlr_signal_emit_safe(&mut (*output).events.enable,
                         output as *mut libc::c_void);
}
unsafe extern "C" fn output_update_matrix(mut output: *mut wlr_output) {
    wlr_matrix_projection((*output).transform_matrix.as_mut_ptr(),
                          (*output).width, (*output).height,
                          (*output).transform);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_enable(mut output: *mut wlr_output,
                                           mut enable: bool) -> bool {
    if (*output).enabled as libc::c_int == enable as libc::c_int {
        return 1i32 != 0
    }
    if (*(*output).impl_0).enable.is_some() {
        return (*(*output).impl_0).enable.expect("non-null function pointer")(output,
                                                                              enable)
    }
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_set_mode(mut output: *mut wlr_output,
                                             mut mode: *mut wlr_output_mode)
 -> bool {
    if (*output).impl_0.is_null() || (*(*output).impl_0).set_mode.is_none() {
        return 0i32 != 0
    }
    if (*output).current_mode == mode { return 1i32 != 0 }
    return (*(*output).impl_0).set_mode.expect("non-null function pointer")(output,
                                                                            mode);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_set_custom_mode(mut output:
                                                        *mut wlr_output,
                                                    mut width: int32_t,
                                                    mut height: int32_t,
                                                    mut refresh: int32_t)
 -> bool {
    if (*output).impl_0.is_null() ||
           (*(*output).impl_0).set_custom_mode.is_none() {
        return 0i32 != 0
    }
    if (*output).width == width && (*output).height == height &&
           (*output).refresh == refresh {
        return 1i32 != 0
    }
    return (*(*output).impl_0).set_custom_mode.expect("non-null function pointer")(output,
                                                                                   width,
                                                                                   height,
                                                                                   refresh);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_update_mode(mut output: *mut wlr_output,
                                                mut mode:
                                                    *mut wlr_output_mode) {
    (*output).current_mode = mode;
    if !mode.is_null() {
        wlr_output_update_custom_mode(output, (*mode).width, (*mode).height,
                                      (*mode).refresh);
    } else { wlr_output_update_custom_mode(output, 0i32, 0i32, 0i32); };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_update_custom_mode(mut output:
                                                           *mut wlr_output,
                                                       mut width: int32_t,
                                                       mut height: int32_t,
                                                       mut refresh: int32_t) {
    if (*output).width == width && (*output).height == height &&
           (*output).refresh == refresh {
        return
    }
    (*output).width = width;
    (*output).height = height;
    output_update_matrix(output);
    (*output).refresh = refresh;
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*output).resources.next);
    while wl_resource_get_link(resource) !=
              &mut (*output).resources as *mut wl_list {
        send_current_mode(resource);
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    wlr_output_schedule_done(output);
    wlr_signal_emit_safe(&mut (*output).events.mode,
                         output as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_set_transform(mut output: *mut wlr_output,
                                                  mut transform:
                                                      wl_output_transform) {
    if (*output).transform as libc::c_uint == transform as libc::c_uint {
        return
    }
    (*output).transform = transform;
    output_update_matrix(output);
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*output).resources.next);
    while wl_resource_get_link(resource) !=
              &mut (*output).resources as *mut wl_list {
        send_geometry(resource);
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    wlr_output_schedule_done(output);
    wlr_signal_emit_safe(&mut (*output).events.transform,
                         output as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_set_scale(mut output: *mut wlr_output,
                                              mut scale: libc::c_float) {
    if (*output).scale == scale { return }
    (*output).scale = scale;
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*output).resources.next);
    while wl_resource_get_link(resource) !=
              &mut (*output).resources as *mut wl_list {
        send_scale(resource);
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    wlr_output_schedule_done(output);
    wlr_signal_emit_safe(&mut (*output).events.scale,
                         output as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_set_subpixel(mut output: *mut wlr_output,
                                                 mut subpixel:
                                                     wl_output_subpixel) {
    if (*output).subpixel as libc::c_uint == subpixel as libc::c_uint {
        return
    }
    (*output).subpixel = subpixel;
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*output).resources.next);
    while wl_resource_get_link(resource) !=
              &mut (*output).resources as *mut wl_list {
        send_geometry(resource);
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    wlr_output_schedule_done(output);
}
unsafe extern "C" fn schedule_done_handle_idle_timer(mut data:
                                                         *mut libc::c_void) {
    let mut output: *mut wlr_output = data as *mut wlr_output;
    (*output).idle_done = 0 as *mut wl_event_source;
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*output).resources.next);
    while wl_resource_get_link(resource) !=
              &mut (*output).resources as *mut wl_list {
        let mut version: uint32_t =
            wl_resource_get_version(resource) as uint32_t;
        if version >= 2i32 as libc::c_uint { wl_output_send_done(resource); }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_schedule_done(mut output:
                                                      *mut wlr_output) {
    if !(*output).idle_done.is_null() {
        return
        // Already scheduled
    }
    let mut ev: *mut wl_event_loop =
        wl_display_get_event_loop((*output).display);
    (*output).idle_done =
        wl_event_loop_add_idle(ev,
                               Some(schedule_done_handle_idle_timer as
                                        unsafe extern "C" fn(_:
                                                                 *mut libc::c_void)
                                            -> ()),
                               output as *mut libc::c_void);
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut output: *mut wlr_output =
        (listener as *mut libc::c_char).offset(-560) as *mut wlr_output;
    wlr_output_destroy_global(output);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_init(mut output: *mut wlr_output,
                                         mut backend: *mut wlr_backend,
                                         mut impl_0: *const wlr_output_impl,
                                         mut display: *mut wl_display) {
    if (*impl_0).attach_render.is_some() && (*impl_0).commit.is_some() {
    } else {
        __assert_fail(b"impl->attach_render && impl->commit\x00" as *const u8
                          as *const libc::c_char,
                      b"../types/wlr_output.c\x00" as *const u8 as
                          *const libc::c_char, 304i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 117],
                                                &[libc::c_char; 117]>(b"void wlr_output_init(struct wlr_output *, struct wlr_backend *, const struct wlr_output_impl *, struct wl_display *)\x00")).as_ptr());
    };
    if (*impl_0).set_cursor.is_some() || (*impl_0).move_cursor.is_some() {
        if (*impl_0).set_cursor.is_some() && (*impl_0).move_cursor.is_some() {
        } else {
            __assert_fail(b"impl->set_cursor && impl->move_cursor\x00" as
                              *const u8 as *const libc::c_char,
                          b"../types/wlr_output.c\x00" as *const u8 as
                              *const libc::c_char, 306i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 117],
                                                    &[libc::c_char; 117]>(b"void wlr_output_init(struct wlr_output *, struct wlr_backend *, const struct wlr_output_impl *, struct wl_display *)\x00")).as_ptr());
        };
    }
    (*output).backend = backend;
    (*output).impl_0 = impl_0;
    (*output).display = display;
    wl_list_init(&mut (*output).modes);
    (*output).transform = WL_OUTPUT_TRANSFORM_NORMAL;
    (*output).scale = 1i32 as libc::c_float;
    (*output).commit_seq = 0i32 as uint32_t;
    wl_list_init(&mut (*output).cursors);
    wl_list_init(&mut (*output).resources);
    wl_signal_init(&mut (*output).events.frame);
    wl_signal_init(&mut (*output).events.needs_frame);
    wl_signal_init(&mut (*output).events.precommit);
    wl_signal_init(&mut (*output).events.commit);
    wl_signal_init(&mut (*output).events.present);
    wl_signal_init(&mut (*output).events.enable);
    wl_signal_init(&mut (*output).events.mode);
    wl_signal_init(&mut (*output).events.scale);
    wl_signal_init(&mut (*output).events.transform);
    wl_signal_init(&mut (*output).events.destroy);
    pixman_region32_init(&mut (*output).damage);
    pixman_region32_init(&mut (*output).pending.damage);
    let mut no_hardware_cursors: *const libc::c_char =
        getenv(b"WLR_NO_HARDWARE_CURSORS\x00" as *const u8 as
                   *const libc::c_char);
    if !no_hardware_cursors.is_null() &&
           strcmp(no_hardware_cursors,
                  b"1\x00" as *const u8 as *const libc::c_char) == 0i32 {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] WLR_NO_HARDWARE_CURSORS set, forcing software cursors\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/wlr_output.c\x00" as *const u8 as
                     *const libc::c_char, 333i32);
        (*output).software_cursor_locks = 1i32
    }
    (*output).display_destroy.notify =
        Some(handle_display_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_display_add_destroy_listener(display, &mut (*output).display_destroy);
    (*output).frame_pending = 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_destroy(mut output: *mut wlr_output) {
    if output.is_null() { return }
    wl_list_remove(&mut (*output).display_destroy.link);
    wlr_output_destroy_global(output);
    wlr_signal_emit_safe(&mut (*output).events.destroy,
                         output as *mut libc::c_void);
    // The backend is responsible for free-ing the list of modes
    let mut cursor: *mut wlr_output_cursor = 0 as *mut wlr_output_cursor;
    let mut tmp_cursor: *mut wlr_output_cursor = 0 as *mut wlr_output_cursor;
    cursor =
        ((*output).cursors.next as *mut libc::c_char).offset(-48) as
            *mut wlr_output_cursor;
    tmp_cursor =
        ((*cursor).link.next as *mut libc::c_char).offset(-48) as
            *mut wlr_output_cursor;
    while &mut (*cursor).link as *mut wl_list !=
              &mut (*output).cursors as *mut wl_list {
        wlr_output_cursor_destroy(cursor);
        cursor = tmp_cursor;
        tmp_cursor =
            ((*cursor).link.next as *mut libc::c_char).offset(-48) as
                *mut wlr_output_cursor
    }
    if !(*output).idle_frame.is_null() {
        wl_event_source_remove((*output).idle_frame);
    }
    if !(*output).idle_done.is_null() {
        wl_event_source_remove((*output).idle_done);
    }
    pixman_region32_fini(&mut (*output).pending.damage);
    pixman_region32_fini(&mut (*output).damage);
    if !(*output).impl_0.is_null() && (*(*output).impl_0).destroy.is_some() {
        (*(*output).impl_0).destroy.expect("non-null function pointer")(output);
    } else { free(output as *mut libc::c_void); };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_transformed_resolution(mut output:
                                                               *mut wlr_output,
                                                           mut width:
                                                               *mut libc::c_int,
                                                           mut height:
                                                               *mut libc::c_int) {
    if ((*output).transform as
            libc::c_uint).wrapping_rem(2i32 as libc::c_uint) ==
           0i32 as libc::c_uint {
        *width = (*output).width;
        *height = (*output).height
    } else { *width = (*output).height; *height = (*output).width };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_effective_resolution(mut output:
                                                             *mut wlr_output,
                                                         mut width:
                                                             *mut libc::c_int,
                                                         mut height:
                                                             *mut libc::c_int) {
    wlr_output_transformed_resolution(output, width, height);
    *width = (*width as libc::c_float / (*output).scale) as libc::c_int;
    *height = (*height as libc::c_float / (*output).scale) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_preferred_mode(mut output:
                                                       *mut wlr_output)
 -> *mut wlr_output_mode {
    if wl_list_empty(&mut (*output).modes) != 0 {
        return 0 as *mut wlr_output_mode
    }
    let mut mode: *mut wlr_output_mode = 0 as *mut wlr_output_mode;
    mode =
        ((*output).modes.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_mode;
    while &mut (*mode).link as *mut wl_list !=
              &mut (*output).modes as *mut wl_list {
        if (*mode).preferred { return mode }
        mode =
            ((*mode).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_mode
    }
    // No preferred mode, choose the last one
    return mode;
}
unsafe extern "C" fn output_state_clear_buffer(mut state:
                                                   *mut wlr_output_state) {
    if (*state).committed &
           WLR_OUTPUT_STATE_BUFFER as libc::c_int as libc::c_uint == 0 {
        return
    }
    wlr_buffer_unref((*state).buffer);
    (*state).buffer = 0 as *mut wlr_buffer;
    (*state).committed &=
        !(WLR_OUTPUT_STATE_BUFFER as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_attach_render(mut output: *mut wlr_output,
                                                  mut buffer_age:
                                                      *mut libc::c_int)
 -> bool {
    if !(*(*output).impl_0).attach_render.expect("non-null function pointer")(output,
                                                                              buffer_age)
       {
        return 0i32 != 0
    }
    output_state_clear_buffer(&mut (*output).pending);
    (*output).pending.committed |=
        WLR_OUTPUT_STATE_BUFFER as libc::c_int as libc::c_uint;
    (*output).pending.buffer_type = WLR_OUTPUT_STATE_BUFFER_RENDER;
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_preferred_read_format(mut output:
                                                              *mut wlr_output,
                                                          mut fmt:
                                                              *mut wl_shm_format)
 -> bool {
    if !(*(*output).impl_0).attach_render.expect("non-null function pointer")(output,
                                                                              0
                                                                                  as
                                                                                  *mut libc::c_int)
       {
        return 0i32 != 0
    }
    let mut renderer: *mut wlr_renderer =
        wlr_backend_get_renderer((*output).backend);
    if (*(*renderer).impl_0).preferred_read_format.is_none() ||
           (*(*renderer).impl_0).read_pixels.is_none() {
        return 0i32 != 0
    }
    *fmt =
        (*(*renderer).impl_0).preferred_read_format.expect("non-null function pointer")(renderer);
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_set_damage(mut output: *mut wlr_output,
                                               mut damage:
                                                   *mut pixman_region32_t) {
    pixman_region32_intersect_rect(&mut (*output).pending.damage, damage,
                                   0i32, 0i32,
                                   (*output).width as libc::c_uint,
                                   (*output).height as libc::c_uint);
    (*output).pending.committed |=
        WLR_OUTPUT_STATE_DAMAGE as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn output_state_clear(mut state: *mut wlr_output_state) {
    output_state_clear_buffer(state);
    pixman_region32_clear(&mut (*state).damage);
    (*state).committed = 0i32 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_commit(mut output: *mut wlr_output)
 -> bool {
    if (*output).frame_pending {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Tried to commit when a frame is pending\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_output.c\x00" as *const u8 as
                     *const libc::c_char, 463i32);
        return 0i32 != 0
    }
    if !(*output).idle_frame.is_null() {
        wl_event_source_remove((*output).idle_frame);
        (*output).idle_frame = 0 as *mut wl_event_source
    }
    if (*output).pending.committed &
           WLR_OUTPUT_STATE_BUFFER as libc::c_int as libc::c_uint == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Tried to commit without attaching a buffer\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_output.c\x00" as *const u8 as
                     *const libc::c_char, 472i32);
        return 0i32 != 0
    }
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    clock_gettime(1i32, &mut now);
    let mut event: wlr_output_event_precommit =
        {
            let mut init =
                wlr_output_event_precommit{output: output, when: &mut now,};
            init
        };
    wlr_signal_emit_safe(&mut (*output).events.precommit,
                         &mut event as *mut wlr_output_event_precommit as
                             *mut libc::c_void);
    if !(*(*output).impl_0).commit.expect("non-null function pointer")(output)
       {
        output_state_clear(&mut (*output).pending);
        return 0i32 != 0
    }
    let mut cursor: *mut wlr_output_cursor = 0 as *mut wlr_output_cursor;
    cursor =
        ((*output).cursors.next as *mut libc::c_char).offset(-48) as
            *mut wlr_output_cursor;
    while &mut (*cursor).link as *mut wl_list !=
              &mut (*output).cursors as *mut wl_list {
        if !(!(*cursor).enabled || !(*cursor).visible ||
                 (*cursor).surface.is_null()) {
            wlr_surface_send_frame_done((*cursor).surface, &mut now);
        }
        cursor =
            ((*cursor).link.next as *mut libc::c_char).offset(-48) as
                *mut wlr_output_cursor
    }
    (*output).commit_seq = (*output).commit_seq.wrapping_add(1);
    wlr_signal_emit_safe(&mut (*output).events.commit,
                         output as *mut libc::c_void);
    (*output).frame_pending = 1i32 != 0;
    (*output).needs_frame = 0i32 != 0;
    output_state_clear(&mut (*output).pending);
    pixman_region32_clear(&mut (*output).damage);
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_attach_buffer(mut output: *mut wlr_output,
                                                  mut buffer: *mut wlr_buffer)
 -> bool {
    if (*(*output).impl_0).attach_buffer.is_none() { return 0i32 != 0 }
    if (*output).attach_render_locks > 0i32 { return 0i32 != 0 }
    // If the output has at least one software cursor, refuse to attach the
	// buffer
    let mut cursor: *mut wlr_output_cursor = 0 as *mut wlr_output_cursor;
    cursor =
        ((*output).cursors.next as *mut libc::c_char).offset(-48) as
            *mut wlr_output_cursor;
    while &mut (*cursor).link as *mut wl_list !=
              &mut (*output).cursors as *mut wl_list {
        if (*cursor).enabled as libc::c_int != 0 &&
               (*cursor).visible as libc::c_int != 0 &&
               cursor != (*output).hardware_cursor {
            return 0i32 != 0
        }
        cursor =
            ((*cursor).link.next as *mut libc::c_char).offset(-48) as
                *mut wlr_output_cursor
    }
    if !(*(*output).impl_0).attach_buffer.expect("non-null function pointer")(output,
                                                                              buffer)
       {
        return 0i32 != 0
    }
    output_state_clear_buffer(&mut (*output).pending);
    (*output).pending.committed |=
        WLR_OUTPUT_STATE_BUFFER as libc::c_int as libc::c_uint;
    (*output).pending.buffer_type = WLR_OUTPUT_STATE_BUFFER_SCANOUT;
    (*output).pending.buffer = wlr_buffer_ref(buffer);
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_send_frame(mut output: *mut wlr_output) {
    (*output).frame_pending = 0i32 != 0;
    wlr_signal_emit_safe(&mut (*output).events.frame,
                         output as *mut libc::c_void);
}
unsafe extern "C" fn schedule_frame_handle_idle_timer(mut data:
                                                          *mut libc::c_void) {
    let mut output: *mut wlr_output = data as *mut wlr_output;
    (*output).idle_frame = 0 as *mut wl_event_source;
    if !(*output).frame_pending &&
           (*(*output).impl_0).schedule_frame.is_some() {
        // Ask the backend to send a frame event when appropriate
        if (*(*output).impl_0).schedule_frame.expect("non-null function pointer")(output)
           {
            (*output).frame_pending = 1i32 != 0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_schedule_frame(mut output:
                                                       *mut wlr_output) {
    if (*output).frame_pending as libc::c_int != 0 ||
           !(*output).idle_frame.is_null() {
        return
    }
    // We're using an idle timer here in case a buffer swap happens right after
	// this function is called
    let mut ev: *mut wl_event_loop =
        wl_display_get_event_loop((*output).display);
    (*output).idle_frame =
        wl_event_loop_add_idle(ev,
                               Some(schedule_frame_handle_idle_timer as
                                        unsafe extern "C" fn(_:
                                                                 *mut libc::c_void)
                                            -> ()),
                               output as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_send_present(mut output: *mut wlr_output,
                                                 mut event:
                                                     *mut wlr_output_event_present) {
    let mut _event: wlr_output_event_present =
        {
            let mut init =
                wlr_output_event_present{output: 0 as *mut wlr_output,
                                         commit_seq: 0,
                                         when: 0 as *mut timespec,
                                         seq: 0,
                                         refresh: 0,
                                         flags: 0,};
            init
        };
    if event.is_null() {
        event = &mut _event;
        (*event).commit_seq =
            (*output).commit_seq.wrapping_add(1i32 as libc::c_uint)
    }
    (*event).output = output;
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    if (*event).when.is_null() {
        let mut clock: clockid_t =
            wlr_backend_get_presentation_clock((*output).backend);
        *__errno_location() = 0i32;
        if clock_gettime(clock, &mut now) != 0i32 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] failed to send output present event: failed to read clock: %s\x00"
                         as *const u8 as *const libc::c_char,
                     b"../types/wlr_output.c\x00" as *const u8 as
                         *const libc::c_char, 583i32,
                     strerror(*__errno_location()));
            return
        }
        (*event).when = &mut now
    }
    wlr_signal_emit_safe(&mut (*output).events.present,
                         event as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_set_gamma(mut output: *mut wlr_output,
                                              mut size: size_t,
                                              mut r: *const uint16_t,
                                              mut g: *const uint16_t,
                                              mut b: *const uint16_t)
 -> bool {
    if (*(*output).impl_0).set_gamma.is_none() { return 0i32 != 0 }
    return (*(*output).impl_0).set_gamma.expect("non-null function pointer")(output,
                                                                             size,
                                                                             r,
                                                                             g,
                                                                             b);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_get_gamma_size(mut output:
                                                       *mut wlr_output)
 -> size_t {
    if (*(*output).impl_0).get_gamma_size.is_none() { return 0i32 as size_t }
    return (*(*output).impl_0).get_gamma_size.expect("non-null function pointer")(output);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_export_dmabuf(mut output: *mut wlr_output,
                                                  mut attribs:
                                                      *mut wlr_dmabuf_attributes)
 -> bool {
    if (*(*output).impl_0).export_dmabuf.is_none() { return 0i32 != 0 }
    return (*(*output).impl_0).export_dmabuf.expect("non-null function pointer")(output,
                                                                                 attribs);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_update_needs_frame(mut output:
                                                           *mut wlr_output) {
    (*output).needs_frame = 1i32 != 0;
    wlr_signal_emit_safe(&mut (*output).events.needs_frame,
                         output as *mut libc::c_void);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_output_damage_whole(mut output:
                                                     *mut wlr_output) {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    wlr_output_transformed_resolution(output, &mut width, &mut height);
    pixman_region32_union_rect(&mut (*output).damage, &mut (*output).damage,
                               0i32, 0i32, width as libc::c_uint,
                               height as libc::c_uint);
    wlr_output_update_needs_frame(output);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_from_resource(mut resource:
                                                      *mut wl_resource)
 -> *mut wlr_output {
    if wl_resource_instance_of(resource, &wl_output_interface,
                               &output_impl as *const wl_output_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_output_interface, &output_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_output.c\x00" as *const u8 as
                          *const libc::c_char, 631i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"struct wlr_output *wlr_output_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_output;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_lock_attach_render(mut output:
                                                           *mut wlr_output,
                                                       mut lock: bool) {
    if lock {
        (*output).attach_render_locks += 1
    } else {
        if (*output).attach_render_locks > 0i32 {
        } else {
            __assert_fail(b"output->attach_render_locks > 0\x00" as *const u8
                              as *const libc::c_char,
                          b"../types/wlr_output.c\x00" as *const u8 as
                              *const libc::c_char, 639i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 63],
                                                    &[libc::c_char; 63]>(b"void wlr_output_lock_attach_render(struct wlr_output *, _Bool)\x00")).as_ptr());
        };
        (*output).attach_render_locks -= 1
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] %s direct scan-out on output \'%s\' (locks: %d)\x00" as
                 *const u8 as *const libc::c_char,
             b"../types/wlr_output.c\x00" as *const u8 as *const libc::c_char,
             644i32,
             if lock as libc::c_int != 0 {
                 b"Disabling\x00" as *const u8 as *const libc::c_char
             } else { b"Enabling\x00" as *const u8 as *const libc::c_char },
             (*output).name.as_mut_ptr(), (*output).attach_render_locks);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_lock_software_cursors(mut output:
                                                              *mut wlr_output,
                                                          mut lock: bool) {
    if lock {
        (*output).software_cursor_locks += 1
    } else {
        if (*output).software_cursor_locks > 0i32 {
        } else {
            __assert_fail(b"output->software_cursor_locks > 0\x00" as
                              *const u8 as *const libc::c_char,
                          b"../types/wlr_output.c\x00" as *const u8 as
                              *const libc::c_char, 653i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 66],
                                                    &[libc::c_char; 66]>(b"void wlr_output_lock_software_cursors(struct wlr_output *, _Bool)\x00")).as_ptr());
        };
        (*output).software_cursor_locks -= 1
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] %s hardware cursors on output \'%s\' (locks: %d)\x00"
                 as *const u8 as *const libc::c_char,
             b"../types/wlr_output.c\x00" as *const u8 as *const libc::c_char,
             658i32,
             if lock as libc::c_int != 0 {
                 b"Disabling\x00" as *const u8 as *const libc::c_char
             } else { b"Enabling\x00" as *const u8 as *const libc::c_char },
             (*output).name.as_mut_ptr(), (*output).software_cursor_locks);
    if (*output).software_cursor_locks > 0i32 &&
           !(*output).hardware_cursor.is_null() {
        if (*(*output).impl_0).set_cursor.is_some() {
        } else {
            __assert_fail(b"output->impl->set_cursor\x00" as *const u8 as
                              *const libc::c_char,
                          b"../types/wlr_output.c\x00" as *const u8 as
                              *const libc::c_char, 661i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 66],
                                                    &[libc::c_char; 66]>(b"void wlr_output_lock_software_cursors(struct wlr_output *, _Bool)\x00")).as_ptr());
        };
        (*(*output).impl_0).set_cursor.expect("non-null function pointer")(output,
                                                                           0
                                                                               as
                                                                               *mut wlr_texture,
                                                                           1i32,
                                                                           WL_OUTPUT_TRANSFORM_NORMAL,
                                                                           0i32,
                                                                           0i32,
                                                                           1i32
                                                                               !=
                                                                               0);
        output_cursor_damage_whole((*output).hardware_cursor);
        (*output).hardware_cursor = 0 as *mut wlr_output_cursor
    };
    // If it's possible to use hardware cursors again, don't switch immediately
	// since a recorder is likely to lock software cursors for the next frame
	// again.
}
unsafe extern "C" fn output_scissor(mut output: *mut wlr_output,
                                    mut rect: *mut pixman_box32_t) {
    let mut renderer: *mut wlr_renderer =
        wlr_backend_get_renderer((*output).backend);
    if !renderer.is_null() {
    } else {
        __assert_fail(b"renderer\x00" as *const u8 as *const libc::c_char,
                      b"../types/wlr_output.c\x00" as *const u8 as
                          *const libc::c_char, 675i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 59],
                                                &[libc::c_char; 59]>(b"void output_scissor(struct wlr_output *, pixman_box32_t *)\x00")).as_ptr());
    };
    let mut box_0: wlr_box =
        {
            let mut init =
                wlr_box{x: (*rect).x1,
                        y: (*rect).y1,
                        width: (*rect).x2 - (*rect).x1,
                        height: (*rect).y2 - (*rect).y1,};
            init
        };
    let mut ow: libc::c_int = 0;
    let mut oh: libc::c_int = 0;
    wlr_output_transformed_resolution(output, &mut ow, &mut oh);
    let mut transform: wl_output_transform =
        wlr_output_transform_invert((*output).transform);
    wlr_box_transform(&mut box_0, &mut box_0, transform, ow, oh);
    wlr_renderer_scissor(renderer, &mut box_0);
}
unsafe extern "C" fn output_cursor_render(mut cursor: *mut wlr_output_cursor,
                                          mut damage:
                                              *mut pixman_region32_t) {
    let mut matrix: [libc::c_float; 9] = [0.; 9];
    let mut nrects: libc::c_int = 0;
    let mut rects: *mut pixman_box32_t = 0 as *mut pixman_box32_t;
    let mut renderer: *mut wlr_renderer =
        wlr_backend_get_renderer((*(*cursor).output).backend);
    if !renderer.is_null() {
    } else {
        __assert_fail(b"renderer\x00" as *const u8 as *const libc::c_char,
                      b"../types/wlr_output.c\x00" as *const u8 as
                          *const libc::c_char, 701i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 75],
                                                &[libc::c_char; 75]>(b"void output_cursor_render(struct wlr_output_cursor *, pixman_region32_t *)\x00")).as_ptr());
    };
    let mut texture: *mut wlr_texture = (*cursor).texture;
    if !(*cursor).surface.is_null() {
        texture = wlr_surface_get_texture((*cursor).surface)
    }
    if texture.is_null() { return }
    let mut box_0: wlr_box = wlr_box{x: 0, y: 0, width: 0, height: 0,};
    output_cursor_get_box(cursor, &mut box_0);
    let mut surface_damage: pixman_region32_t =
        pixman_region32_t{extents:
                              pixman_box32_t{x1: 0, y1: 0, x2: 0, y2: 0,},
                          data: 0 as *mut pixman_region32_data_t,};
    pixman_region32_init(&mut surface_damage);
    pixman_region32_union_rect(&mut surface_damage, &mut surface_damage,
                               box_0.x, box_0.y, box_0.width as libc::c_uint,
                               box_0.height as libc::c_uint);
    pixman_region32_intersect(&mut surface_damage, &mut surface_damage,
                              damage);
    if !(pixman_region32_not_empty(&mut surface_damage) == 0) {
        matrix = [0.; 9];
        wlr_matrix_project_box(matrix.as_mut_ptr(), &mut box_0,
                               WL_OUTPUT_TRANSFORM_NORMAL,
                               0i32 as libc::c_float,
                               (*(*cursor).output).transform_matrix.as_mut_ptr()
                                   as *const libc::c_float);
        nrects = 0;
        rects = pixman_region32_rectangles(&mut surface_damage, &mut nrects);
        let mut i: libc::c_int = 0i32;
        while i < nrects {
            output_scissor((*cursor).output, &mut *rects.offset(i as isize));
            wlr_render_texture_with_matrix(renderer, texture,
                                           matrix.as_mut_ptr() as
                                               *const libc::c_float, 1.0f32);
            i += 1
        }
        wlr_renderer_scissor(renderer, 0 as *mut wlr_box);
    }
    pixman_region32_fini(&mut surface_damage);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_render_software_cursors(mut output:
                                                                *mut wlr_output,
                                                            mut damage:
                                                                *mut pixman_region32_t) {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    wlr_output_transformed_resolution(output, &mut width, &mut height);
    let mut render_damage: pixman_region32_t =
        pixman_region32_t{extents:
                              pixman_box32_t{x1: 0, y1: 0, x2: 0, y2: 0,},
                          data: 0 as *mut pixman_region32_data_t,};
    pixman_region32_init(&mut render_damage);
    pixman_region32_union_rect(&mut render_damage, &mut render_damage, 0i32,
                               0i32, width as libc::c_uint,
                               height as libc::c_uint);
    if !damage.is_null() {
        // Damage tracking supported
        pixman_region32_intersect(&mut render_damage, &mut render_damage,
                                  damage);
    }
    if pixman_region32_not_empty(&mut render_damage) != 0 {
        let mut cursor: *mut wlr_output_cursor = 0 as *mut wlr_output_cursor;
        cursor =
            ((*output).cursors.next as *mut libc::c_char).offset(-48) as
                *mut wlr_output_cursor;
        while &mut (*cursor).link as *mut wl_list !=
                  &mut (*output).cursors as *mut wl_list {
            if !(!(*cursor).enabled || !(*cursor).visible ||
                     (*output).hardware_cursor == cursor) {
                output_cursor_render(cursor, &mut render_damage);
            }
            cursor =
                ((*cursor).link.next as *mut libc::c_char).offset(-48) as
                    *mut wlr_output_cursor
        }
    }
    pixman_region32_fini(&mut render_damage);
}
/* *
 * Returns the cursor box, scaled for its output.
 */
unsafe extern "C" fn output_cursor_get_box(mut cursor: *mut wlr_output_cursor,
                                           mut box_0: *mut wlr_box) {
    (*box_0).x =
        ((*cursor).x - (*cursor).hotspot_x as libc::c_double) as libc::c_int;
    (*box_0).y =
        ((*cursor).y - (*cursor).hotspot_y as libc::c_double) as libc::c_int;
    (*box_0).width = (*cursor).width as libc::c_int;
    (*box_0).height = (*cursor).height as libc::c_int;
}
unsafe extern "C" fn output_cursor_damage_whole(mut cursor:
                                                    *mut wlr_output_cursor) {
    let mut box_0: wlr_box = wlr_box{x: 0, y: 0, width: 0, height: 0,};
    output_cursor_get_box(cursor, &mut box_0);
    pixman_region32_union_rect(&mut (*(*cursor).output).damage,
                               &mut (*(*cursor).output).damage, box_0.x,
                               box_0.y, box_0.width as libc::c_uint,
                               box_0.height as libc::c_uint);
    wlr_output_update_needs_frame((*cursor).output);
}
unsafe extern "C" fn output_cursor_reset(mut cursor: *mut wlr_output_cursor) {
    if (*(*cursor).output).hardware_cursor != cursor {
        output_cursor_damage_whole(cursor);
    }
    if !(*cursor).surface.is_null() {
        wl_list_remove(&mut (*cursor).surface_commit.link);
        wl_list_remove(&mut (*cursor).surface_destroy.link);
        if (*cursor).visible {
            wlr_surface_send_leave((*cursor).surface, (*cursor).output);
        }
        (*cursor).surface = 0 as *mut wlr_surface
    };
}
unsafe extern "C" fn output_cursor_update_visible(mut cursor:
                                                      *mut wlr_output_cursor) {
    let mut output_box: wlr_box = wlr_box{x: 0, y: 0, width: 0, height: 0,};
    output_box.y = 0i32;
    output_box.x = output_box.y;
    wlr_output_transformed_resolution((*cursor).output, &mut output_box.width,
                                      &mut output_box.height);
    let mut cursor_box: wlr_box = wlr_box{x: 0, y: 0, width: 0, height: 0,};
    output_cursor_get_box(cursor, &mut cursor_box);
    let mut intersection: wlr_box = wlr_box{x: 0, y: 0, width: 0, height: 0,};
    let mut visible: bool =
        wlr_box_intersection(&mut intersection, &mut output_box,
                             &mut cursor_box);
    if !(*cursor).surface.is_null() {
        if (*cursor).visible as libc::c_int != 0 && !visible {
            wlr_surface_send_leave((*cursor).surface, (*cursor).output);
        }
        if !(*cursor).visible && visible as libc::c_int != 0 {
            wlr_surface_send_enter((*cursor).surface, (*cursor).output);
        }
    }
    (*cursor).visible = visible;
}
unsafe extern "C" fn output_cursor_attempt_hardware(mut cursor:
                                                        *mut wlr_output_cursor)
 -> bool {
    let mut scale: int32_t = (*(*cursor).output).scale as int32_t;
    let mut transform: wl_output_transform = WL_OUTPUT_TRANSFORM_NORMAL;
    let mut texture: *mut wlr_texture = (*cursor).texture;
    if !(*cursor).surface.is_null() {
        texture = wlr_surface_get_texture((*cursor).surface);
        scale = (*(*cursor).surface).current.scale;
        transform = (*(*cursor).surface).current.transform
    }
    if (*(*cursor).output).software_cursor_locks > 0i32 { return 0i32 != 0 }
    let mut hwcur: *mut wlr_output_cursor =
        (*(*cursor).output).hardware_cursor;
    if (*(*(*cursor).output).impl_0).set_cursor.is_some() &&
           (hwcur.is_null() || hwcur == cursor) {
        // If the cursor was hidden or was a software cursor, the hardware
		// cursor position is outdated
        if (*(*(*cursor).output).impl_0).move_cursor.is_some() {
        } else {
            __assert_fail(b"cursor->output->impl->move_cursor\x00" as
                              *const u8 as *const libc::c_char,
                          b"../types/wlr_output.c\x00" as *const u8 as
                              *const libc::c_char, 844i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 65],
                                                    &[libc::c_char; 65]>(b"_Bool output_cursor_attempt_hardware(struct wlr_output_cursor *)\x00")).as_ptr());
        };
        (*(*(*cursor).output).impl_0).move_cursor.expect("non-null function pointer")((*cursor).output,
                                                                                      (*cursor).x
                                                                                          as
                                                                                          libc::c_int,
                                                                                      (*cursor).y
                                                                                          as
                                                                                          libc::c_int);
        if (*(*(*cursor).output).impl_0).set_cursor.expect("non-null function pointer")((*cursor).output,
                                                                                        texture,
                                                                                        scale,
                                                                                        transform,
                                                                                        (*cursor).hotspot_x,
                                                                                        (*cursor).hotspot_y,
                                                                                        1i32
                                                                                            !=
                                                                                            0)
           {
            (*(*cursor).output).hardware_cursor = cursor;
            return 1i32 != 0
        }
    }
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_cursor_set_image(mut cursor:
                                                         *mut wlr_output_cursor,
                                                     mut pixels:
                                                         *const uint8_t,
                                                     mut stride: int32_t,
                                                     mut width: uint32_t,
                                                     mut height: uint32_t,
                                                     mut hotspot_x: int32_t,
                                                     mut hotspot_y: int32_t)
 -> bool {
    let mut renderer: *mut wlr_renderer =
        wlr_backend_get_renderer((*(*cursor).output).backend);
    if renderer.is_null() {
        // if the backend has no renderer, we can't draw a cursor, but this is
		// actually okay, for ex. with the noop backend
        return 1i32 != 0
    }
    output_cursor_reset(cursor);
    (*cursor).width = width;
    (*cursor).height = height;
    (*cursor).hotspot_x = hotspot_x;
    (*cursor).hotspot_y = hotspot_y;
    output_cursor_update_visible(cursor);
    wlr_texture_destroy((*cursor).texture);
    (*cursor).texture = 0 as *mut wlr_texture;
    (*cursor).enabled = 0i32 != 0;
    if !pixels.is_null() {
        (*cursor).texture =
            wlr_texture_from_pixels(renderer, WL_SHM_FORMAT_ARGB8888,
                                    stride as uint32_t, width, height,
                                    pixels as *const libc::c_void);
        if (*cursor).texture.is_null() { return 0i32 != 0 }
        (*cursor).enabled = 1i32 != 0
    }
    if output_cursor_attempt_hardware(cursor) { return 1i32 != 0 }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Falling back to software cursor on output \'%s\'\x00"
                 as *const u8 as *const libc::c_char,
             b"../types/wlr_output.c\x00" as *const u8 as *const libc::c_char,
             893i32, (*(*cursor).output).name.as_mut_ptr());
    output_cursor_damage_whole(cursor);
    return 1i32 != 0;
}
unsafe extern "C" fn output_cursor_commit(mut cursor: *mut wlr_output_cursor,
                                          mut update_hotspot: bool) {
    if (*(*cursor).output).hardware_cursor != cursor {
        output_cursor_damage_whole(cursor);
    }
    let mut surface: *mut wlr_surface = (*cursor).surface;
    if !surface.is_null() {
    } else {
        __assert_fail(b"surface != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_output.c\x00" as *const u8 as
                          *const libc::c_char, 905i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 61],
                                                &[libc::c_char; 61]>(b"void output_cursor_commit(struct wlr_output_cursor *, _Bool)\x00")).as_ptr());
    };
    // Some clients commit a cursor surface with a NULL buffer to hide it.
    (*cursor).enabled = wlr_surface_has_buffer(surface);
    (*cursor).width =
        ((*surface).current.width as libc::c_float *
             (*(*cursor).output).scale) as uint32_t;
    (*cursor).height =
        ((*surface).current.height as libc::c_float *
             (*(*cursor).output).scale) as uint32_t;
    output_cursor_update_visible(cursor);
    if update_hotspot {
        (*cursor).hotspot_x =
            ((*cursor).hotspot_x as libc::c_float -
                 (*surface).current.dx as libc::c_float *
                     (*(*cursor).output).scale) as int32_t;
        (*cursor).hotspot_y =
            ((*cursor).hotspot_y as libc::c_float -
                 (*surface).current.dy as libc::c_float *
                     (*(*cursor).output).scale) as int32_t
    }
    if output_cursor_attempt_hardware(cursor) { return }
    // Fallback to software cursor
    output_cursor_damage_whole(cursor);
}
unsafe extern "C" fn output_cursor_handle_commit(mut listener:
                                                     *mut wl_listener,
                                                 mut data:
                                                     *mut libc::c_void) {
    let mut cursor: *mut wlr_output_cursor =
        (listener as *mut libc::c_char).offset(-80) as *mut wlr_output_cursor;
    output_cursor_commit(cursor, 1i32 != 0);
}
unsafe extern "C" fn output_cursor_handle_destroy(mut listener:
                                                      *mut wl_listener,
                                                  mut data:
                                                      *mut libc::c_void) {
    let mut cursor: *mut wlr_output_cursor =
        (listener as *mut libc::c_char).offset(-104) as
            *mut wlr_output_cursor;
    output_cursor_reset(cursor);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_cursor_set_surface(mut cursor:
                                                           *mut wlr_output_cursor,
                                                       mut surface:
                                                           *mut wlr_surface,
                                                       mut hotspot_x: int32_t,
                                                       mut hotspot_y:
                                                           int32_t) {
    hotspot_x =
        (hotspot_x as libc::c_float * (*(*cursor).output).scale) as int32_t;
    hotspot_y =
        (hotspot_y as libc::c_float * (*(*cursor).output).scale) as int32_t;
    if !surface.is_null() && surface == (*cursor).surface {
        // Only update the hotspot: surface hasn't changed
        if (*(*cursor).output).hardware_cursor != cursor {
            output_cursor_damage_whole(cursor);
        }
        (*cursor).hotspot_x = hotspot_x;
        (*cursor).hotspot_y = hotspot_y;
        if (*(*cursor).output).hardware_cursor != cursor {
            output_cursor_damage_whole(cursor);
        } else {
            if (*(*(*cursor).output).impl_0).set_cursor.is_some() {
            } else {
                __assert_fail(b"cursor->output->impl->set_cursor\x00" as
                                  *const u8 as *const libc::c_char,
                              b"../types/wlr_output.c\x00" as *const u8 as
                                  *const libc::c_char, 955i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 103],
                                                        &[libc::c_char; 103]>(b"void wlr_output_cursor_set_surface(struct wlr_output_cursor *, struct wlr_surface *, int32_t, int32_t)\x00")).as_ptr());
            };
            (*(*(*cursor).output).impl_0).set_cursor.expect("non-null function pointer")((*cursor).output,
                                                                                         0
                                                                                             as
                                                                                             *mut wlr_texture,
                                                                                         1i32,
                                                                                         WL_OUTPUT_TRANSFORM_NORMAL,
                                                                                         hotspot_x,
                                                                                         hotspot_y,
                                                                                         0i32
                                                                                             !=
                                                                                             0);
        }
        return
    }
    output_cursor_reset(cursor);
    (*cursor).surface = surface;
    (*cursor).hotspot_x = hotspot_x;
    (*cursor).hotspot_y = hotspot_y;
    if !surface.is_null() {
        wl_signal_add(&mut (*surface).events.commit,
                      &mut (*cursor).surface_commit);
        wl_signal_add(&mut (*surface).events.destroy,
                      &mut (*cursor).surface_destroy);
        (*cursor).visible = 0i32 != 0;
        output_cursor_commit(cursor, 0i32 != 0);
    } else {
        (*cursor).enabled = 0i32 != 0;
        (*cursor).width = 0i32 as uint32_t;
        (*cursor).height = 0i32 as uint32_t;
        if (*(*cursor).output).hardware_cursor == cursor {
            if (*(*(*cursor).output).impl_0).set_cursor.is_some() {
            } else {
                __assert_fail(b"cursor->output->impl->set_cursor\x00" as
                                  *const u8 as *const libc::c_char,
                              b"../types/wlr_output.c\x00" as *const u8 as
                                  *const libc::c_char, 980i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 103],
                                                        &[libc::c_char; 103]>(b"void wlr_output_cursor_set_surface(struct wlr_output_cursor *, struct wlr_surface *, int32_t, int32_t)\x00")).as_ptr());
            };
            (*(*(*cursor).output).impl_0).set_cursor.expect("non-null function pointer")((*cursor).output,
                                                                                         0
                                                                                             as
                                                                                             *mut wlr_texture,
                                                                                         1i32,
                                                                                         WL_OUTPUT_TRANSFORM_NORMAL,
                                                                                         0i32,
                                                                                         0i32,
                                                                                         1i32
                                                                                             !=
                                                                                             0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_cursor_move(mut cursor:
                                                    *mut wlr_output_cursor,
                                                mut x: libc::c_double,
                                                mut y: libc::c_double)
 -> bool {
    if (*cursor).x == x && (*cursor).y == y { return 1i32 != 0 }
    if (*(*cursor).output).hardware_cursor != cursor {
        output_cursor_damage_whole(cursor);
    }
    let mut was_visible: bool = (*cursor).visible;
    x *= (*(*cursor).output).scale as libc::c_double;
    y *= (*(*cursor).output).scale as libc::c_double;
    (*cursor).x = x;
    (*cursor).y = y;
    output_cursor_update_visible(cursor);
    if !was_visible && !(*cursor).visible {
        // Cursor is still hidden, do nothing
        return 1i32 != 0
    } // default position is at (0, 0)
    if (*(*cursor).output).hardware_cursor != cursor {
        output_cursor_damage_whole(cursor);
        return 1i32 != 0
    }
    if (*(*(*cursor).output).impl_0).move_cursor.is_some() {
    } else {
        __assert_fail(b"cursor->output->impl->move_cursor\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_output.c\x00" as *const u8 as
                          *const libc::c_char, 1014i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"_Bool wlr_output_cursor_move(struct wlr_output_cursor *, double, double)\x00")).as_ptr());
    };
    return (*(*(*cursor).output).impl_0).move_cursor.expect("non-null function pointer")((*cursor).output,
                                                                                         x
                                                                                             as
                                                                                             libc::c_int,
                                                                                         y
                                                                                             as
                                                                                             libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_cursor_create(mut output: *mut wlr_output)
 -> *mut wlr_output_cursor {
    let mut cursor: *mut wlr_output_cursor =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_output_cursor>() as libc::c_ulong) as
            *mut wlr_output_cursor;
    if cursor.is_null() { return 0 as *mut wlr_output_cursor }
    (*cursor).output = output;
    wl_signal_init(&mut (*cursor).events.destroy);
    wl_list_init(&mut (*cursor).surface_commit.link);
    (*cursor).surface_commit.notify =
        Some(output_cursor_handle_commit as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_list_init(&mut (*cursor).surface_destroy.link);
    (*cursor).surface_destroy.notify =
        Some(output_cursor_handle_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_list_insert(&mut (*output).cursors, &mut (*cursor).link);
    (*cursor).visible = 1i32 != 0;
    return cursor;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_cursor_destroy(mut cursor:
                                                       *mut wlr_output_cursor) {
    if cursor.is_null() { return }
    output_cursor_reset(cursor);
    wlr_signal_emit_safe(&mut (*cursor).events.destroy,
                         cursor as *mut libc::c_void);
    if (*(*cursor).output).hardware_cursor == cursor {
        // If this cursor was the hardware cursor, disable it
        if (*(*(*cursor).output).impl_0).set_cursor.is_some() {
            (*(*(*cursor).output).impl_0).set_cursor.expect("non-null function pointer")((*cursor).output,
                                                                                         0
                                                                                             as
                                                                                             *mut wlr_texture,
                                                                                         1i32,
                                                                                         WL_OUTPUT_TRANSFORM_NORMAL,
                                                                                         0i32,
                                                                                         0i32,
                                                                                         1i32
                                                                                             !=
                                                                                             0);
        }
        (*(*cursor).output).hardware_cursor = 0 as *mut wlr_output_cursor
    }
    wlr_texture_destroy((*cursor).texture);
    wl_list_remove(&mut (*cursor).link);
    free(cursor as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_transform_invert(mut tr:
                                                         wl_output_transform)
 -> wl_output_transform {
    if tr as libc::c_uint &
           WL_OUTPUT_TRANSFORM_90 as libc::c_int as libc::c_uint != 0 &&
           tr as libc::c_uint &
               WL_OUTPUT_TRANSFORM_FLIPPED as libc::c_int as libc::c_uint == 0
       {
        tr =
            ::std::mem::transmute::<libc::c_uint,
                                    wl_output_transform>(tr as libc::c_uint ^
                                                             WL_OUTPUT_TRANSFORM_180
                                                                 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint)
    }
    return tr;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
// mHz
// only when using a software cursor without a surface
// only when using a cursor surface
/* *
 * Holds the double-buffered output state.
 */
// enum wlr_output_state_field
// output-buffer-local coordinates
// only valid if WLR_OUTPUT_STATE_BUFFER
// if WLR_OUTPUT_STATE_BUFFER_SCANOUT
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
// mm
// Note: some backends may have zero modes
// wlr_output_mode::link
// mHz, may be zero
// damage for cursors and fullscreen surface, in output-local coordinates
// Commit sequence number. Incremented on each commit, may overflow.
// Request to render a frame
// Emitted when buffers need to be swapped (because software cursors or
		// fullscreen damage or because of backend-specific logic)
// Emitted right before commit
// wlr_output_event_precommit
// Emitted right after commit
// Emitted right after the buffer has been presented to the user
// wlr_output_event_present
// number of locks forcing rendering
// wlr_output_cursor::link
// number of locks forcing software cursors
// The presentation was synchronized to the "vertical retrace" by the
	// display hardware such that tearing does not happen.
// The display hardware provided measurements that the hardware driver
	// converted into a presentation timestamp.
// The display hardware signalled that it started using the new image
	// content.
// The presentation of this update was done zero-copy.
// Frame submission for which this presentation event is for (see
	// wlr_output.commit_seq).
// Time when the content update turned into light the first time.
// Vertical retrace counter. Zero if unavailable.
// Prediction of how many nanoseconds after `when` the very next output
	// refresh may occur. Zero if unknown.
// nsec
// enum wlr_output_present_flag
/* *
 * Enables or disables the output. A disabled output is turned off and doesn't
 * emit `frame` events.
 */
/* *
 * Returns the preferred mode for this output. If the output doesn't support
 * modes, returns NULL.
 */
/* *
 * Sets the output mode. Enables the output if it's currently disabled.
 */
/* *
 * Sets a custom mode on the output. If modes are available, they are preferred.
 * Setting `refresh` to zero lets the backend pick a preferred value.
 */
/* *
 * Schedule a done event.
 *
 * This is intended to be used by wl_output add-on interfaces.
 */
/* *
 * Computes the transformed output resolution.
 */
/* *
 * Computes the transformed and scaled output resolution.
 */
/* *
 * Attach the renderer's buffer to the output. Compositors must call this
 * function before rendering. After they are done rendering, they should call
 * `wlr_output_commit` to submit the new frame.
 *
 * If non-NULL, `buffer_age` is set to the drawing buffer age in number of
 * frames or -1 if unknown. This is useful for damage tracking.
 */
/* *
 * Attach a buffer to the output. Compositors should call `wlr_output_commit`
 * to submit the new frame.
 */
/* *
 * Get the preferred format for reading pixels.
 * This function might change the current rendering context.
 */
/* *
 * Set the damage region for the frame to be submitted. This is the region of
 * the screen that has changed since the last frame.
 *
 * Compositors implementing damage tracking should call this function with the
 * damaged region in output-buffer-local coordinates (ie. scaled and
 * transformed).
 *
 * This region is not to be confused with the renderer's buffer damage, ie. the
 * region compositors need to repaint. Compositors usually need to repaint more
 * than what changed since last frame since multiple render buffers are used.
 */
/* *
 * Commit the pending output state. If `wlr_output_attach_render` has been
 * called, the pending frame will be submitted for display.
 *
 * This function schedules a `frame` event.
 */
/* *
 * Manually schedules a `frame` event. If a `frame` event is already pending,
 * it is a no-op.
 */
/* *
 * Returns the maximum length of each gamma ramp, or 0 if unsupported.
 */
/* *
 * Sets the gamma table for this output. `r`, `g` and `b` are gamma ramps for
 * red, green and blue. `size` is the length of the ramps and must not exceed
 * the value returned by `wlr_output_get_gamma_size`.
 *
 * Providing zero-sized ramps resets the gamma table.
 */
/* *
 * Locks the output to only use rendering instead of direct scan-out. This is
 * useful if direct scan-out needs to be temporarily disabled (e.g. during
 * screen capture). There must be as many unlocks as there have been locks to
 * restore the original state. There should never be an unlock before a lock.
 */
/* *
 * Locks the output to only use software cursors instead of hardware cursors.
 * This is useful if hardware cursors need to be temporarily disabled (e.g.
 * during screen capture). There must be as many unlocks as there have been
 * locks to restore the original state. There should never be an unlock before
 * a lock.
 */
/* *
 * Renders software cursors. This is a utility function that can be called when
 * compositors render.
 */
/* *
 * Sets the cursor image. The image must be already scaled for the output.
 */
/* *
 * Returns the transform that, when composed with `tr`, gives
 * `WL_OUTPUT_TRANSFORM_NORMAL`.
 */
/* *
 * Returns a transform that, when applied, has the same effect as applying
 * sequentially `tr_a` and `tr_b`.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_output_transform_compose(mut tr_a:
                                                          wl_output_transform,
                                                      mut tr_b:
                                                          wl_output_transform)
 -> wl_output_transform {
    let mut flipped: uint32_t =
        (tr_a as libc::c_uint ^ tr_b as libc::c_uint) &
            WL_OUTPUT_TRANSFORM_FLIPPED as libc::c_int as libc::c_uint;
    let mut rotated: uint32_t =
        (tr_a as libc::c_uint).wrapping_add(tr_b as libc::c_uint) &
            (WL_OUTPUT_TRANSFORM_90 as libc::c_int |
                 WL_OUTPUT_TRANSFORM_180 as libc::c_int) as libc::c_uint;
    return (flipped | rotated) as wl_output_transform;
}
