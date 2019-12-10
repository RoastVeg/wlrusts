use libc;
extern "C" {
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wlr_gles2_renderer_create(egl: *mut wlr_egl) -> *mut wlr_renderer;
    #[no_mangle]
    fn wlr_texture_get_size(texture: *mut wlr_texture,
                            width: *mut libc::c_int,
                            height: *mut libc::c_int);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_display_init_shm(display: *mut wl_display) -> libc::c_int;
    #[no_mangle]
    fn wl_display_add_shm_format(display: *mut wl_display, format: uint32_t)
     -> *mut uint32_t;
    // TODO: Allocate and return a wlr_egl
/* *
 * Initializes an EGL context for the given platform and remote display.
 * Will attempt to load all possibly required api functions.
 */
    #[no_mangle]
    fn wlr_egl_init(egl: *mut wlr_egl, platform: EGLenum,
                    remote_display: *mut libc::c_void,
                    config_attribs: *mut EGLint, visual_id: EGLint) -> bool;
    /* *
 * Frees all related EGL resources, makes the context not-current and
 * unbinds a bound wayland display.
 */
    #[no_mangle]
    fn wlr_egl_finish(egl: *mut wlr_egl);
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
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
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type khronos_int32_t = int32_t;
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
pub type EGLint = khronos_int32_t;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
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
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}
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
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
/* the protocol interface */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub events: C2RustUnnamed_1,
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub destroy: wl_signal,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_init(mut renderer: *mut wlr_renderer,
                                           mut impl_0:
                                               *const wlr_renderer_impl) {
    if (*impl_0).begin.is_some() {
    } else {
        __assert_fail(b"impl->begin\x00" as *const u8 as *const libc::c_char,
                      b"../render/wlr_renderer.c\x00" as *const u8 as
                          *const libc::c_char, 14i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void wlr_renderer_init(struct wlr_renderer *, const struct wlr_renderer_impl *)\x00")).as_ptr());
    };
    if (*impl_0).clear.is_some() {
    } else {
        __assert_fail(b"impl->clear\x00" as *const u8 as *const libc::c_char,
                      b"../render/wlr_renderer.c\x00" as *const u8 as
                          *const libc::c_char, 15i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void wlr_renderer_init(struct wlr_renderer *, const struct wlr_renderer_impl *)\x00")).as_ptr());
    };
    if (*impl_0).scissor.is_some() {
    } else {
        __assert_fail(b"impl->scissor\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/wlr_renderer.c\x00" as *const u8 as
                          *const libc::c_char, 16i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void wlr_renderer_init(struct wlr_renderer *, const struct wlr_renderer_impl *)\x00")).as_ptr());
    };
    if (*impl_0).render_texture_with_matrix.is_some() {
    } else {
        __assert_fail(b"impl->render_texture_with_matrix\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/wlr_renderer.c\x00" as *const u8 as
                          *const libc::c_char, 17i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void wlr_renderer_init(struct wlr_renderer *, const struct wlr_renderer_impl *)\x00")).as_ptr());
    };
    if (*impl_0).render_quad_with_matrix.is_some() {
    } else {
        __assert_fail(b"impl->render_quad_with_matrix\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/wlr_renderer.c\x00" as *const u8 as
                          *const libc::c_char, 18i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void wlr_renderer_init(struct wlr_renderer *, const struct wlr_renderer_impl *)\x00")).as_ptr());
    };
    if (*impl_0).render_ellipse_with_matrix.is_some() {
    } else {
        __assert_fail(b"impl->render_ellipse_with_matrix\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/wlr_renderer.c\x00" as *const u8 as
                          *const libc::c_char, 19i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void wlr_renderer_init(struct wlr_renderer *, const struct wlr_renderer_impl *)\x00")).as_ptr());
    };
    if (*impl_0).formats.is_some() {
    } else {
        __assert_fail(b"impl->formats\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/wlr_renderer.c\x00" as *const u8 as
                          *const libc::c_char, 20i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void wlr_renderer_init(struct wlr_renderer *, const struct wlr_renderer_impl *)\x00")).as_ptr());
    };
    if (*impl_0).format_supported.is_some() {
    } else {
        __assert_fail(b"impl->format_supported\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/wlr_renderer.c\x00" as *const u8 as
                          *const libc::c_char, 21i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void wlr_renderer_init(struct wlr_renderer *, const struct wlr_renderer_impl *)\x00")).as_ptr());
    };
    if (*impl_0).texture_from_pixels.is_some() {
    } else {
        __assert_fail(b"impl->texture_from_pixels\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/wlr_renderer.c\x00" as *const u8 as
                          *const libc::c_char, 22i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void wlr_renderer_init(struct wlr_renderer *, const struct wlr_renderer_impl *)\x00")).as_ptr());
    };
    (*renderer).impl_0 = impl_0;
    wl_signal_init(&mut (*renderer).events.destroy);
}
/* *
 * Destroys this wlr_renderer. Textures must be destroyed separately.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_destroy(mut r: *mut wlr_renderer) {
    if r.is_null() { return }
    wlr_signal_emit_safe(&mut (*r).events.destroy, r as *mut libc::c_void);
    if !(*r).impl_0.is_null() && (*(*r).impl_0).destroy.is_some() {
        (*(*r).impl_0).destroy.expect("non-null function pointer")(r);
    } else { free(r as *mut libc::c_void); };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_begin(mut r: *mut wlr_renderer,
                                            mut width: libc::c_int,
                                            mut height: libc::c_int) {
    (*(*r).impl_0).begin.expect("non-null function pointer")(r,
                                                             width as
                                                                 uint32_t,
                                                             height as
                                                                 uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_end(mut r: *mut wlr_renderer) {
    if (*(*r).impl_0).end.is_some() {
        (*(*r).impl_0).end.expect("non-null function pointer")(r);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_clear(mut r: *mut wlr_renderer,
                                            mut color: *const libc::c_float) {
    (*(*r).impl_0).clear.expect("non-null function pointer")(r, color);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_scissor(mut r: *mut wlr_renderer,
                                              mut box_0: *mut wlr_box) {
    (*(*r).impl_0).scissor.expect("non-null function pointer")(r, box_0);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_render_texture(mut r: *mut wlr_renderer,
                                            mut texture: *mut wlr_texture,
                                            mut projection:
                                                *const libc::c_float,
                                            mut x: libc::c_int,
                                            mut y: libc::c_int,
                                            mut alpha: libc::c_float)
 -> bool {
    let mut box_0: wlr_box =
        { let mut init = wlr_box{x: x, y: y, width: 0, height: 0,}; init };
    wlr_texture_get_size(texture, &mut box_0.width, &mut box_0.height);
    let mut matrix: [libc::c_float; 9] = [0.; 9];
    wlr_matrix_project_box(matrix.as_mut_ptr(), &mut box_0,
                           WL_OUTPUT_TRANSFORM_NORMAL, 0i32 as libc::c_float,
                           projection);
    return wlr_render_texture_with_matrix(r, texture,
                                          matrix.as_mut_ptr() as
                                              *const libc::c_float, alpha);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_render_texture_with_matrix(mut r:
                                                            *mut wlr_renderer,
                                                        mut texture:
                                                            *mut wlr_texture,
                                                        mut matrix:
                                                            *const libc::c_float,
                                                        mut alpha:
                                                            libc::c_float)
 -> bool {
    return (*(*r).impl_0).render_texture_with_matrix.expect("non-null function pointer")(r,
                                                                                         texture,
                                                                                         matrix,
                                                                                         alpha);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_render_rect(mut r: *mut wlr_renderer,
                                         mut box_0: *const wlr_box,
                                         mut color: *const libc::c_float,
                                         mut projection:
                                             *const libc::c_float) {
    let mut matrix: [libc::c_float; 9] = [0.; 9];
    wlr_matrix_project_box(matrix.as_mut_ptr(), box_0,
                           WL_OUTPUT_TRANSFORM_NORMAL, 0i32 as libc::c_float,
                           projection);
    wlr_render_quad_with_matrix(r, color,
                                matrix.as_mut_ptr() as *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_render_quad_with_matrix(mut r: *mut wlr_renderer,
                                                     mut color:
                                                         *const libc::c_float,
                                                     mut matrix:
                                                         *const libc::c_float) {
    (*(*r).impl_0).render_quad_with_matrix.expect("non-null function pointer")(r,
                                                                               color,
                                                                               matrix);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_render_ellipse(mut r: *mut wlr_renderer,
                                            mut box_0: *const wlr_box,
                                            mut color: *const libc::c_float,
                                            mut projection:
                                                *const libc::c_float) {
    let mut matrix: [libc::c_float; 9] = [0.; 9];
    wlr_matrix_project_box(matrix.as_mut_ptr(), box_0,
                           WL_OUTPUT_TRANSFORM_NORMAL, 0i32 as libc::c_float,
                           projection);
    wlr_render_ellipse_with_matrix(r, color,
                                   matrix.as_mut_ptr() as
                                       *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_render_ellipse_with_matrix(mut r:
                                                            *mut wlr_renderer,
                                                        mut color:
                                                            *const libc::c_float,
                                                        mut matrix:
                                                            *const libc::c_float) {
    (*(*r).impl_0).render_ellipse_with_matrix.expect("non-null function pointer")(r,
                                                                                  color,
                                                                                  matrix);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_get_formats(mut r: *mut wlr_renderer,
                                                  mut len: *mut size_t)
 -> *const wl_shm_format {
    return (*(*r).impl_0).formats.expect("non-null function pointer")(r, len);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_resource_is_wl_drm_buffer(mut r:
                                                                    *mut wlr_renderer,
                                                                mut resource:
                                                                    *mut wl_resource)
 -> bool {
    if (*(*r).impl_0).resource_is_wl_drm_buffer.is_none() { return 0i32 != 0 }
    return (*(*r).impl_0).resource_is_wl_drm_buffer.expect("non-null function pointer")(r,
                                                                                        resource);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_wl_drm_buffer_get_size(mut r:
                                                                 *mut wlr_renderer,
                                                             mut buffer:
                                                                 *mut wl_resource,
                                                             mut width:
                                                                 *mut libc::c_int,
                                                             mut height:
                                                                 *mut libc::c_int) {
    if (*(*r).impl_0).wl_drm_buffer_get_size.is_none() { return }
    return (*(*r).impl_0).wl_drm_buffer_get_size.expect("non-null function pointer")(r,
                                                                                     buffer,
                                                                                     width,
                                                                                     height);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_get_dmabuf_formats(mut r:
                                                             *mut wlr_renderer)
 -> *const wlr_drm_format_set {
    if (*(*r).impl_0).get_dmabuf_formats.is_none() {
        return 0 as *const wlr_drm_format_set
    }
    return (*(*r).impl_0).get_dmabuf_formats.expect("non-null function pointer")(r);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_read_pixels(mut r: *mut wlr_renderer,
                                                  mut fmt: wl_shm_format,
                                                  mut flags: *mut uint32_t,
                                                  mut stride: uint32_t,
                                                  mut width: uint32_t,
                                                  mut height: uint32_t,
                                                  mut src_x: uint32_t,
                                                  mut src_y: uint32_t,
                                                  mut dst_x: uint32_t,
                                                  mut dst_y: uint32_t,
                                                  mut data: *mut libc::c_void)
 -> bool {
    if (*(*r).impl_0).read_pixels.is_none() { return 0i32 != 0 }
    return (*(*r).impl_0).read_pixels.expect("non-null function pointer")(r,
                                                                          fmt,
                                                                          flags,
                                                                          stride,
                                                                          width,
                                                                          height,
                                                                          src_x,
                                                                          src_y,
                                                                          dst_x,
                                                                          dst_y,
                                                                          data);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_format_supported(mut r:
                                                           *mut wlr_renderer,
                                                       mut fmt: wl_shm_format)
 -> bool {
    return (*(*r).impl_0).format_supported.expect("non-null function pointer")(r,
                                                                               fmt);
}
/* *
 * Defines a scissor box. Only pixels that lie within the scissor box can be
 * modified by drawing functions. Providing a NULL `box` disables the scissor
 * box.
 */
/* *
 * Renders the requested texture.
 */
/* *
 * Renders the requested texture using the provided matrix.
 */
/* *
 * Renders a solid rectangle in the specified color.
 */
/* *
 * Renders a solid quadrangle in the specified color with the specified matrix.
 */
/* *
 * Renders a solid ellipse in the specified color.
 */
/* *
 * Renders a solid ellipse in the specified color with the specified matrix.
 */
/* *
 * Returns a list of pixel formats supported by this renderer.
 */
/* *
 * Returns true if this wl_buffer is a wl_drm buffer.
 */
/* *
 * Gets the width and height of a wl_drm buffer.
 */
/* *
 * Get the available DMA-BUF formats.
 */
/* *
 * Reads out of pixels of the currently bound surface into data. `stride` is in
 * bytes.
 *
 * If `flags` is not NULl, the caller indicates that it accepts frame flags
 * defined in `enum wlr_renderer_read_pixels_flags`.
 */
/* *
 * Checks if a format is supported.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_init_wl_display(mut r:
                                                          *mut wlr_renderer,
                                                      mut wl_display:
                                                          *mut wl_display) {
    if wl_display_init_shm(wl_display) != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to initialize shm\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/wlr_renderer.c\x00" as *const u8 as
                     *const libc::c_char, 153i32);
        return
    }
    let mut len: size_t = 0;
    let mut formats: *const wl_shm_format =
        wlr_renderer_get_formats(r, &mut len);
    if formats.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to initialize shm: cannot get formats\x00"
                     as *const u8 as *const libc::c_char,
                 b"../render/wlr_renderer.c\x00" as *const u8 as
                     *const libc::c_char, 160i32);
        return
    }
    let mut i: size_t = 0i32 as size_t;
    while i < len {
        // These formats are already added by default
        if *formats.offset(i as isize) as libc::c_uint !=
               WL_SHM_FORMAT_ARGB8888 as libc::c_int as libc::c_uint &&
               *formats.offset(i as isize) as libc::c_uint !=
                   WL_SHM_FORMAT_XRGB8888 as libc::c_int as libc::c_uint {
            wl_display_add_shm_format(wl_display,
                                      *formats.offset(i as isize) as
                                          uint32_t);
        }
        i = i.wrapping_add(1)
    }
    if (*(*r).impl_0).texture_from_dmabuf.is_some() {
        wlr_linux_dmabuf_v1_create(wl_display, r);
    }
    if (*(*r).impl_0).init_wl_display.is_some() {
        (*(*r).impl_0).init_wl_display.expect("non-null function pointer")(r,
                                                                           wl_display);
    };
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_renderer_autocreate(mut egl: *mut wlr_egl,
                                                 mut platform: EGLenum,
                                                 mut remote_display:
                                                     *mut libc::c_void,
                                                 mut config_attribs:
                                                     *mut EGLint,
                                                 mut visual_id: EGLint)
 -> *mut wlr_renderer {
    // Append GLES2-specific bits to the provided EGL config attributes
    let mut gles2_config_attribs: [EGLint; 3] =
        [0x3040i32, 0x4i32, 0x3038i32]; // not including terminating EGL_NONE
    let mut config_attribs_len: size_t = 0i32 as size_t;
    while !config_attribs.is_null() &&
              *config_attribs.offset(config_attribs_len as isize) != 0x3038i32
          {
        config_attribs_len = config_attribs_len.wrapping_add(1)
    }
    let mut all_config_attribs_len: size_t =
        config_attribs_len.wrapping_add((::std::mem::size_of::<[EGLint; 3]>()
                                             as
                                             libc::c_ulong).wrapping_div(::std::mem::size_of::<EGLint>()
                                                                             as
                                                                             libc::c_ulong));
    let vla = all_config_attribs_len as usize;
    let mut all_config_attribs: Vec<EGLint> = ::std::vec::from_elem(0, vla);
    if config_attribs_len > 0i32 as libc::c_ulong {
        memcpy(all_config_attribs.as_mut_ptr() as *mut libc::c_void,
               config_attribs as *const libc::c_void,
               config_attribs_len.wrapping_mul(::std::mem::size_of::<EGLint>()
                                                   as libc::c_ulong));
    }
    memcpy(&mut *all_config_attribs.as_mut_ptr().offset(config_attribs_len as
                                                            isize) as
               *mut EGLint as *mut libc::c_void,
           gles2_config_attribs.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[EGLint; 3]>() as libc::c_ulong);
    if !wlr_egl_init(egl, platform, remote_display,
                     all_config_attribs.as_mut_ptr(), visual_id) {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not initialize EGL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/wlr_renderer.c\x00" as *const u8 as
                     *const libc::c_char, 208i32);
        return 0 as *mut wlr_renderer
    }
    let mut renderer: *mut wlr_renderer = wlr_gles2_renderer_create(egl);
    if renderer.is_null() { wlr_egl_finish(egl); }
    return renderer;
}
