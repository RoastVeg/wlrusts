use libc;
extern "C" {
    pub type wl_display;
    pub type wl_client;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn glActiveTexture(texture: GLenum);
    #[no_mangle]
    fn glAttachShader(program: GLuint, shader: GLuint);
    #[no_mangle]
    fn glBindTexture(target: GLenum, texture: GLuint);
    #[no_mangle]
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    #[no_mangle]
    fn glClear(mask: GLbitfield);
    #[no_mangle]
    fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat,
                    alpha: GLfloat);
    #[no_mangle]
    fn glCompileShader(shader: GLuint);
    #[no_mangle]
    fn glCreateProgram() -> GLuint;
    #[no_mangle]
    fn glCreateShader(type_0: GLenum) -> GLuint;
    #[no_mangle]
    fn glDeleteProgram(program: GLuint);
    #[no_mangle]
    fn glDeleteShader(shader: GLuint);
    #[no_mangle]
    fn glDetachShader(program: GLuint, shader: GLuint);
    #[no_mangle]
    fn glDisable(cap: GLenum);
    #[no_mangle]
    fn glDisableVertexAttribArray(index: GLuint);
    #[no_mangle]
    fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
    #[no_mangle]
    fn glEnable(cap: GLenum);
    #[no_mangle]
    fn glEnableVertexAttribArray(index: GLuint);
    #[no_mangle]
    fn glFinish();
    #[no_mangle]
    fn glGetError() -> GLenum;
    #[no_mangle]
    fn glGetIntegerv(pname: GLenum, data: *mut GLint);
    #[no_mangle]
    fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
    #[no_mangle]
    fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
    #[no_mangle]
    fn glGetString(name: GLenum) -> *const GLubyte;
    #[no_mangle]
    fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
    #[no_mangle]
    fn glLinkProgram(program: GLuint);
    #[no_mangle]
    fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei,
                    format: GLenum, type_0: GLenum,
                    pixels: *mut libc::c_void);
    #[no_mangle]
    fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    #[no_mangle]
    fn glShaderSource(shader: GLuint, count: GLsizei,
                      string: *const *const GLchar, length: *const GLint);
    #[no_mangle]
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    #[no_mangle]
    fn glUniform1f(location: GLint, v0: GLfloat);
    #[no_mangle]
    fn glUniform1i(location: GLint, v0: GLint);
    #[no_mangle]
    fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat,
                   v3: GLfloat);
    #[no_mangle]
    fn glUniformMatrix3fv(location: GLint, count: GLsizei,
                          transpose: GLboolean, value: *const GLfloat);
    #[no_mangle]
    fn glUseProgram(program: GLuint);
    #[no_mangle]
    fn glVertexAttribPointer(index: GLuint, size: GLint, type_0: GLenum,
                             normalized: GLboolean, stride: GLsizei,
                             pointer: *const libc::c_void);
    #[no_mangle]
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn eglGetConfigAttrib(dpy: EGLDisplay, config: EGLConfig,
                          attribute: EGLint, value: *mut EGLint)
     -> EGLBoolean;
    #[no_mangle]
    fn wlr_egl_is_current(egl: *mut wlr_egl) -> bool;
    #[no_mangle]
    fn wlr_egl_make_current(egl: *mut wlr_egl, surface: EGLSurface,
                            buffer_age: *mut libc::c_int) -> bool;
    #[no_mangle]
    fn wlr_egl_get_dmabuf_formats(egl: *mut wlr_egl)
     -> *const wlr_drm_format_set;
    #[no_mangle]
    fn wlr_egl_bind_display(egl: *mut wlr_egl, local_display: *mut wl_display)
     -> bool;
    #[no_mangle]
    fn wlr_renderer_init(renderer: *mut wlr_renderer,
                         impl_0: *const wlr_renderer_impl);
    /* *
 * Transforms a box inside a `width` x `height` box.
 */
    #[no_mangle]
    fn wlr_box_transform(dest: *mut wlr_box, box_0: *const wlr_box,
                         transform: wl_output_transform, width: libc::c_int,
                         height: libc::c_int);
    #[no_mangle]
    fn wlr_matrix_transpose(mat: *mut libc::c_float, a: *const libc::c_float);
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    static mut glEGLImageTargetTexture2DOES:
           PFNGLEGLIMAGETARGETTEXTURE2DOESPROC;
    #[no_mangle]
    fn load_glapi() -> bool;
    #[no_mangle]
    static mut eglQueryWaylandBufferWL: PFNEGLQUERYWAYLANDBUFFERWL;
    #[no_mangle]
    static mut glDebugMessageCallbackKHR: PFNGLDEBUGMESSAGECALLBACKKHRPROC;
    #[no_mangle]
    static mut glDebugMessageControlKHR: PFNGLDEBUGMESSAGECONTROLKHRPROC;
    #[no_mangle]
    static mut glPopDebugGroupKHR: PFNGLPOPDEBUGGROUPKHRPROC;
    #[no_mangle]
    static mut glPushDebugGroupKHR: PFNGLPUSHDEBUGGROUPKHRPROC;
    #[no_mangle]
    fn gles2_get_texture(wlr_texture: *mut wlr_texture)
     -> *mut wlr_gles2_texture;
    #[no_mangle]
    fn get_gles2_wl_formats(len: *mut size_t) -> *const wl_shm_format;
    #[no_mangle]
    fn wlr_gles2_texture_from_dmabuf(egl: *mut wlr_egl,
                                     attribs: *mut wlr_dmabuf_attributes)
     -> *mut wlr_texture;
    #[no_mangle]
    fn get_gles2_format_from_gl(gl_format: GLint, gl_type: GLint, alpha: bool)
     -> *const wlr_gles2_pixel_format;
    #[no_mangle]
    fn wlr_gles2_texture_from_wl_drm(egl: *mut wlr_egl,
                                     data: *mut wl_resource)
     -> *mut wlr_texture;
    #[no_mangle]
    fn wlr_gles2_texture_from_pixels(egl: *mut wlr_egl, wl_fmt: wl_shm_format,
                                     stride: uint32_t, width: uint32_t,
                                     height: uint32_t,
                                     data: *const libc::c_void)
     -> *mut wlr_texture;
    #[no_mangle]
    fn get_gles2_format_from_wl(fmt: wl_shm_format)
     -> *const wlr_gles2_pixel_format;
    #[no_mangle]
    static quad_vertex_src: [GLchar; 0];
    #[no_mangle]
    static quad_fragment_src: [GLchar; 0];
    #[no_mangle]
    static ellipse_fragment_src: [GLchar; 0];
    #[no_mangle]
    static tex_vertex_src: [GLchar; 0];
    #[no_mangle]
    static tex_fragment_src_rgba: [GLchar; 0];
    #[no_mangle]
    static tex_fragment_src_rgbx: [GLchar; 0];
    #[no_mangle]
    static tex_fragment_src_external: [GLchar; 0];
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type khronos_int32_t = int32_t;
pub type khronos_uint8_t = libc::c_uchar;
pub type khronos_float_t = libc::c_float;
pub type GLenum = libc::c_uint;
pub type GLuint = libc::c_uint;
pub type GLchar = libc::c_char;
pub type GLfloat = khronos_float_t;
pub type GLbitfield = libc::c_uint;
pub type GLint = libc::c_int;
pub type GLboolean = libc::c_uchar;
pub type GLsizei = libc::c_int;
pub type GLubyte = khronos_uint8_t;
pub type GLDEBUGPROCKHR
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLuint, _: GLenum,
                                _: GLsizei, _: *const GLchar,
                                _: *const libc::c_void) -> ()>;
pub type PFNGLDEBUGMESSAGECONTROLKHRPROC
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLenum, _: GLsizei,
                                _: *const GLuint, _: GLboolean) -> ()>;
pub type PFNGLDEBUGMESSAGECALLBACKKHRPROC
    =
    Option<unsafe extern "C" fn(_: GLDEBUGPROCKHR, _: *const libc::c_void)
               -> ()>;
pub type PFNGLPUSHDEBUGGROUPKHRPROC
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLuint, _: GLsizei,
                                _: *const GLchar) -> ()>;
pub type PFNGLPOPDEBUGGROUPKHRPROC = Option<unsafe extern "C" fn() -> ()>;
pub type GLeglImageOES = *mut libc::c_void;
pub type PFNGLEGLIMAGETARGETTEXTURE2DOESPROC
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLeglImageOES) -> ()>;
pub type size_t = libc::c_ulong;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_object {
    pub interface: *const wl_interface,
    pub implementation: *const libc::c_void,
    pub id: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_interface {
    pub name: *const libc::c_char,
    pub version: libc::c_int,
    pub method_count: libc::c_int,
    pub methods: *const wl_message,
    pub event_count: libc::c_int,
    pub events: *const wl_message,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_message {
    pub name: *const libc::c_char,
    pub signature: *const libc::c_char,
    pub types: *mut *const wl_interface,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_resource {
    pub object: wl_object,
    pub destroy: wl_resource_destroy_func_t,
    pub link: wl_list,
    pub destroy_signal: wl_signal,
    pub client: *mut wl_client,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
pub type wl_resource_destroy_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_resource) -> ()>;
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
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
pub type EGLint = khronos_int32_t;
pub type EGLBoolean = libc::c_uint;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
pub type EGLImageKHR = *mut libc::c_void;

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_format {
    pub format: uint32_t,
    pub len: size_t,
    pub cap: size_t,
    pub modifiers: [uint64_t; 0],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub cap: size_t,
    pub formats: *mut *mut wlr_drm_format,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed_0,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_texture {
    pub impl_0: *const wlr_texture_impl,
}

#[repr(C)]#[derive(Copy, Clone)]
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
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
pub type wlr_renderer_read_pixels_flags = libc::c_uint;
pub const WLR_RENDERER_READ_PIXELS_Y_INVERT: wlr_renderer_read_pixels_flags =
    1;
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
pub type PFNEGLQUERYWAYLANDBUFFERWL
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: *mut wl_resource, _: EGLint,
                                _: *mut EGLint) -> EGLBoolean>;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_gles2_tex_shader {
    pub program: GLuint,
    pub proj: GLint,
    pub invert_y: GLint,
    pub tex: GLint,
    pub alpha: GLint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_gles2_pixel_format {
    pub wl_format: wl_shm_format,
    pub gl_format: GLint,
    pub gl_type: GLint,
    pub depth: libc::c_int,
    pub bpp: libc::c_int,
    pub has_alpha: bool,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_gles2_renderer {
    pub wlr_renderer: wlr_renderer,
    pub egl: *mut wlr_egl,
    pub exts_str: *const libc::c_char,
    pub exts: C2RustUnnamed_4,
    pub shaders: C2RustUnnamed_1,
    pub viewport_width: uint32_t,
    pub viewport_height: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub quad: C2RustUnnamed_3,
    pub ellipse: C2RustUnnamed_2,
    pub tex_rgba: wlr_gles2_tex_shader,
    pub tex_rgbx: wlr_gles2_tex_shader,
    pub tex_ext: wlr_gles2_tex_shader,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub program: GLuint,
    pub proj: GLint,
    pub color: GLint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub program: GLuint,
    pub proj: GLint,
    pub color: GLint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub read_format_bgra_ext: bool,
    pub debug_khr: bool,
    pub egl_image_external_oes: bool,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_gles2_texture {
    pub wlr_texture: wlr_texture,
    pub egl: *mut wlr_egl,
    pub target: GLenum,
    pub tex: GLuint,
    pub image: EGLImageKHR,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub inverted_y: bool,
    pub has_alpha: bool,
    pub wl_format: wl_shm_format,
}
unsafe extern "C" fn gles2_get_renderer(mut wlr_renderer: *mut wlr_renderer)
 -> *mut wlr_gles2_renderer {
    if (*wlr_renderer).impl_0 == &renderer_impl as *const wlr_renderer_impl {
    } else {
        __assert_fail(b"wlr_renderer->impl == &renderer_impl\x00" as *const u8
                          as *const libc::c_char,
                      b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char, 21i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 69],
                                                &[libc::c_char; 69]>(b"struct wlr_gles2_renderer *gles2_get_renderer(struct wlr_renderer *)\x00")).as_ptr());
    };
    return wlr_renderer as *mut wlr_gles2_renderer;
}
unsafe extern "C" fn gles2_get_renderer_in_context(mut wlr_renderer:
                                                       *mut wlr_renderer)
 -> *mut wlr_gles2_renderer {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer(wlr_renderer);
    if wlr_egl_is_current((*renderer).egl) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_egl_is_current(renderer->egl)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char, 28i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"struct wlr_gles2_renderer *gles2_get_renderer_in_context(struct wlr_renderer *)\x00")).as_ptr());
    };
    return renderer;
}
unsafe extern "C" fn gles2_begin(mut wlr_renderer: *mut wlr_renderer,
                                 mut width: uint32_t, mut height: uint32_t) {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer_in_context(wlr_renderer);
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"gles2_begin\x00")).as_ptr());
    glViewport(0i32, 0i32, width as GLsizei, height as GLsizei);
    (*renderer).viewport_width = width;
    (*renderer).viewport_height = height;
    // enable transparency
    glEnable(0xbe2i32 as GLenum);
    glBlendFunc(1i32 as GLenum, 0x303i32 as GLenum);
    // XXX: maybe we should save output projection and remove some of the need
	// for users to sling matricies themselves
    pop_gles2_marker();
}
unsafe extern "C" fn gles2_end(mut wlr_renderer: *mut wlr_renderer) {
    gles2_get_renderer_in_context(wlr_renderer);
    // no-op
}
unsafe extern "C" fn gles2_clear(mut wlr_renderer: *mut wlr_renderer,
                                 mut color: *const libc::c_float) {
    gles2_get_renderer_in_context(wlr_renderer);
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"gles2_clear\x00")).as_ptr());
    glClearColor(*color.offset(0), *color.offset(1), *color.offset(2),
                 *color.offset(3));
    glClear(0x4000i32 as GLbitfield);
    pop_gles2_marker();
}
unsafe extern "C" fn gles2_scissor(mut wlr_renderer: *mut wlr_renderer,
                                   mut box_0: *mut wlr_box) {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer_in_context(wlr_renderer);
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"gles2_scissor\x00")).as_ptr());
    if !box_0.is_null() {
        let mut gl_box: wlr_box = wlr_box{x: 0, y: 0, width: 0, height: 0,};
        wlr_box_transform(&mut gl_box, box_0, WL_OUTPUT_TRANSFORM_FLIPPED_180,
                          (*renderer).viewport_width as libc::c_int,
                          (*renderer).viewport_height as libc::c_int);
        glScissor(gl_box.x, gl_box.y, gl_box.width, gl_box.height);
        glEnable(0xc11i32 as GLenum);
    } else { glDisable(0xc11i32 as GLenum); }
    pop_gles2_marker();
}
unsafe extern "C" fn draw_quad() {
    let mut verts: [GLfloat; 8] =
        [1i32 as GLfloat, 0i32 as GLfloat, 0i32 as GLfloat, 0i32 as GLfloat,
         1i32 as GLfloat, 1i32 as GLfloat, 0i32 as GLfloat, 1i32 as GLfloat];
    let mut texcoord: [GLfloat; 8] =
        [1i32 as GLfloat, 0i32 as GLfloat, 0i32 as GLfloat, 0i32 as GLfloat,
         1i32 as GLfloat, 1i32 as GLfloat, 0i32 as GLfloat, 1i32 as GLfloat];
    glVertexAttribPointer(0i32 as GLuint, 2i32, 0x1406i32 as GLenum,
                          0i32 as GLboolean, 0i32,
                          verts.as_mut_ptr() as *const libc::c_void);
    glVertexAttribPointer(1i32 as GLuint, 2i32, 0x1406i32 as GLenum,
                          0i32 as GLboolean, 0i32,
                          texcoord.as_mut_ptr() as *const libc::c_void);
    glEnableVertexAttribArray(0i32 as GLuint);
    glEnableVertexAttribArray(1i32 as GLuint);
    glDrawArrays(0x5i32 as GLenum, 0i32, 4i32);
    glDisableVertexAttribArray(0i32 as GLuint);
    glDisableVertexAttribArray(1i32 as GLuint);
}
unsafe extern "C" fn gles2_render_texture_with_matrix(mut wlr_renderer:
                                                          *mut wlr_renderer,
                                                      mut wlr_texture:
                                                          *mut wlr_texture,
                                                      mut matrix:
                                                          *const libc::c_float,
                                                      mut alpha:
                                                          libc::c_float)
 -> bool {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer_in_context(wlr_renderer);
    let mut texture: *mut wlr_gles2_texture = gles2_get_texture(wlr_texture);
    let mut shader: *mut wlr_gles2_tex_shader =
        0 as *mut wlr_gles2_tex_shader;
    match (*texture).target {
        3553 => {
            if (*texture).has_alpha {
                shader = &mut (*renderer).shaders.tex_rgba
            } else { shader = &mut (*renderer).shaders.tex_rgbx }
        }
        36197 => {
            shader = &mut (*renderer).shaders.tex_ext;
            if !(*renderer).exts.egl_image_external_oes {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to render texture: GL_TEXTURE_EXTERNAL_OES not supported\x00"
                             as *const u8 as *const libc::c_char,
                         b"../render/gles2/renderer.c\x00" as *const u8 as
                             *const libc::c_char, 136i32);
                return 0i32 != 0
            }
        }
        _ => { abort(); }
    }
    // OpenGL ES 2 requires the glUniformMatrix3fv transpose parameter to be set
	// to GL_FALSE
    let mut transposition: [libc::c_float; 9] = [0.; 9];
    wlr_matrix_transpose(transposition.as_mut_ptr(), matrix);
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"gles2_render_texture_with_matrix\x00")).as_ptr());
    glActiveTexture(0x84c0i32 as GLenum);
    glBindTexture((*texture).target, (*texture).tex);
    glTexParameteri((*texture).target, 0x2801i32 as GLenum, 0x2601i32);
    glUseProgram((*shader).program);
    glUniformMatrix3fv((*shader).proj, 1i32, 0i32 as GLboolean,
                       transposition.as_mut_ptr());
    glUniform1i((*shader).invert_y, (*texture).inverted_y as GLint);
    glUniform1i((*shader).tex, 0i32);
    glUniform1f((*shader).alpha, alpha);
    draw_quad();
    pop_gles2_marker();
    return 1i32 != 0;
}
unsafe extern "C" fn gles2_render_quad_with_matrix(mut wlr_renderer:
                                                       *mut wlr_renderer,
                                                   mut color:
                                                       *const libc::c_float,
                                                   mut matrix:
                                                       *const libc::c_float) {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer_in_context(wlr_renderer);
    // OpenGL ES 2 requires the glUniformMatrix3fv transpose parameter to be set
	// to GL_FALSE
    let mut transposition: [libc::c_float; 9] = [0.; 9];
    wlr_matrix_transpose(transposition.as_mut_ptr(), matrix);
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"gles2_render_quad_with_matrix\x00")).as_ptr());
    glUseProgram((*renderer).shaders.quad.program);
    glUniformMatrix3fv((*renderer).shaders.quad.proj, 1i32, 0i32 as GLboolean,
                       transposition.as_mut_ptr());
    glUniform4f((*renderer).shaders.quad.color, *color.offset(0),
                *color.offset(1), *color.offset(2), *color.offset(3));
    draw_quad();
    pop_gles2_marker();
}
unsafe extern "C" fn gles2_render_ellipse_with_matrix(mut wlr_renderer:
                                                          *mut wlr_renderer,
                                                      mut color:
                                                          *const libc::c_float,
                                                      mut matrix:
                                                          *const libc::c_float) {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer_in_context(wlr_renderer);
    // OpenGL ES 2 requires the glUniformMatrix3fv transpose parameter to be set
	// to GL_FALSE
    let mut transposition: [libc::c_float; 9] = [0.; 9];
    wlr_matrix_transpose(transposition.as_mut_ptr(), matrix);
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"gles2_render_ellipse_with_matrix\x00")).as_ptr());
    glUseProgram((*renderer).shaders.ellipse.program);
    glUniformMatrix3fv((*renderer).shaders.ellipse.proj, 1i32,
                       0i32 as GLboolean, transposition.as_mut_ptr());
    glUniform4f((*renderer).shaders.ellipse.color, *color.offset(0),
                *color.offset(1), *color.offset(2), *color.offset(3));
    draw_quad();
    pop_gles2_marker();
}
unsafe extern "C" fn gles2_renderer_formats(mut wlr_renderer:
                                                *mut wlr_renderer,
                                            mut len: *mut size_t)
 -> *const wl_shm_format {
    return get_gles2_wl_formats(len);
}
unsafe extern "C" fn gles2_format_supported(mut wlr_renderer:
                                                *mut wlr_renderer,
                                            mut wl_fmt: wl_shm_format)
 -> bool {
    return !get_gles2_format_from_wl(wl_fmt).is_null();
}
unsafe extern "C" fn gles2_resource_is_wl_drm_buffer(mut wlr_renderer:
                                                         *mut wlr_renderer,
                                                     mut resource:
                                                         *mut wl_resource)
 -> bool {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer(wlr_renderer);
    if eglQueryWaylandBufferWL.is_none() { return 0i32 != 0 }
    let mut fmt: EGLint = 0;
    return eglQueryWaylandBufferWL.expect("non-null function pointer")((*(*renderer).egl).display,
                                                                       resource,
                                                                       0x3080i32,
                                                                       &mut fmt)
               != 0;
}
unsafe extern "C" fn gles2_wl_drm_buffer_get_size(mut wlr_renderer:
                                                      *mut wlr_renderer,
                                                  mut buffer:
                                                      *mut wl_resource,
                                                  mut width: *mut libc::c_int,
                                                  mut height:
                                                      *mut libc::c_int) {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer(wlr_renderer);
    if eglQueryWaylandBufferWL.is_none() { return }
    eglQueryWaylandBufferWL.expect("non-null function pointer")((*(*renderer).egl).display,
                                                                buffer,
                                                                0x3057i32,
                                                                width);
    eglQueryWaylandBufferWL.expect("non-null function pointer")((*(*renderer).egl).display,
                                                                buffer,
                                                                0x3056i32,
                                                                height);
}
unsafe extern "C" fn gles2_get_dmabuf_formats(mut wlr_renderer:
                                                  *mut wlr_renderer)
 -> *const wlr_drm_format_set {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer(wlr_renderer);
    return wlr_egl_get_dmabuf_formats((*renderer).egl);
}
unsafe extern "C" fn gles2_preferred_read_format(mut wlr_renderer:
                                                     *mut wlr_renderer)
 -> wl_shm_format {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer_in_context(wlr_renderer);
    let mut gl_format: GLint = -1i32;
    let mut gl_type: GLint = -1i32;
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"gles2_preferred_read_format\x00")).as_ptr());
    glGetIntegerv(0x8b9bi32 as GLenum, &mut gl_format);
    glGetIntegerv(0x8b9ai32 as GLenum, &mut gl_type);
    pop_gles2_marker();
    let mut alpha_size: EGLint = -1i32;
    eglGetConfigAttrib((*(*renderer).egl).display, (*(*renderer).egl).config,
                       0x3021i32, &mut alpha_size);
    let mut fmt: *const wlr_gles2_pixel_format =
        get_gles2_format_from_gl(gl_format, gl_type, alpha_size > 0i32);
    if !fmt.is_null() { return (*fmt).wl_format }
    if (*renderer).exts.read_format_bgra_ext { return WL_SHM_FORMAT_XRGB8888 }
    return WL_SHM_FORMAT_XBGR8888;
}
unsafe extern "C" fn gles2_read_pixels(mut wlr_renderer: *mut wlr_renderer,
                                       mut wl_fmt: wl_shm_format,
                                       mut flags: *mut uint32_t,
                                       mut stride: uint32_t,
                                       mut width: uint32_t,
                                       mut height: uint32_t,
                                       mut src_x: uint32_t,
                                       mut src_y: uint32_t,
                                       mut dst_x: uint32_t,
                                       mut dst_y: uint32_t,
                                       mut data: *mut libc::c_void) -> bool {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer_in_context(wlr_renderer);
    let mut fmt: *const wlr_gles2_pixel_format =
        get_gles2_format_from_wl(wl_fmt);
    if fmt.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot read pixels: unsupported pixel format\x00"
                     as *const u8 as *const libc::c_char,
                 b"../render/gles2/renderer.c\x00" as *const u8 as
                     *const libc::c_char, 286i32);
        return 0i32 != 0
    }
    if (*fmt).gl_format == 0x80e1i32 && !(*renderer).exts.read_format_bgra_ext
       {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot read pixels: missing GL_EXT_read_format_bgra extension\x00"
                     as *const u8 as *const libc::c_char,
                 b"../render/gles2/renderer.c\x00" as *const u8 as
                     *const libc::c_char, 292i32);
        return 0i32 != 0
    }
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"gles2_read_pixels\x00")).as_ptr());
    // Make sure any pending drawing is finished before we try to read it
    glFinish(); // Clear the error flag
    glGetError();
    let mut p: *mut libc::c_uchar =
        (data as
             *mut libc::c_uchar).offset(dst_y.wrapping_mul(stride) as isize);
    let mut pack_stride: uint32_t =
        width.wrapping_mul((*fmt).bpp as
                               libc::c_uint).wrapping_div(8i32 as
                                                              libc::c_uint);
    if pack_stride == stride && dst_x == 0i32 as libc::c_uint &&
           !flags.is_null() {
        // Under these particular conditions, we can read the pixels with only
		// one glReadPixels call
        glReadPixels(src_x as GLint,
                     (*renderer).viewport_height.wrapping_sub(height).wrapping_sub(src_y)
                         as GLint, width as GLsizei, height as GLsizei,
                     (*fmt).gl_format as GLenum, (*fmt).gl_type as GLenum,
                     p as *mut libc::c_void);
        *flags = WLR_RENDERER_READ_PIXELS_Y_INVERT as libc::c_int as uint32_t
    } else {
        // Unfortunately GLES2 doesn't support GL_PACK_*, so we have to read
		// the lines out row by row
        let mut i: size_t = 0i32 as size_t;
        while i < height as libc::c_ulong {
            glReadPixels(src_x as GLint,
                         ((*renderer).viewport_height.wrapping_sub(src_y) as
                              libc::c_ulong).wrapping_sub(i).wrapping_sub(1i32
                                                                              as
                                                                              libc::c_ulong)
                             as GLint, width as GLsizei, 1i32,
                         (*fmt).gl_format as GLenum, (*fmt).gl_type as GLenum,
                         p.offset(i.wrapping_mul(stride as libc::c_ulong) as
                                      isize).offset(dst_x.wrapping_mul((*fmt).bpp
                                                                           as
                                                                           libc::c_uint).wrapping_div(8i32
                                                                                                          as
                                                                                                          libc::c_uint)
                                                        as isize) as
                             *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        if !flags.is_null() { *flags = 0i32 as uint32_t }
    }
    pop_gles2_marker();
    return glGetError() == 0i32 as libc::c_uint;
}
unsafe extern "C" fn gles2_texture_from_pixels(mut wlr_renderer:
                                                   *mut wlr_renderer,
                                               mut wl_fmt: wl_shm_format,
                                               mut stride: uint32_t,
                                               mut width: uint32_t,
                                               mut height: uint32_t,
                                               mut data: *const libc::c_void)
 -> *mut wlr_texture {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer(wlr_renderer);
    return wlr_gles2_texture_from_pixels((*renderer).egl, wl_fmt, stride,
                                         width, height, data);
}
unsafe extern "C" fn gles2_texture_from_wl_drm(mut wlr_renderer:
                                                   *mut wlr_renderer,
                                               mut data: *mut wl_resource)
 -> *mut wlr_texture {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer(wlr_renderer);
    return wlr_gles2_texture_from_wl_drm((*renderer).egl, data);
}
unsafe extern "C" fn gles2_texture_from_dmabuf(mut wlr_renderer:
                                                   *mut wlr_renderer,
                                               mut attribs:
                                                   *mut wlr_dmabuf_attributes)
 -> *mut wlr_texture {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer(wlr_renderer);
    return wlr_gles2_texture_from_dmabuf((*renderer).egl, attribs);
}
unsafe extern "C" fn gles2_init_wl_display(mut wlr_renderer:
                                               *mut wlr_renderer,
                                           mut wl_display: *mut wl_display) {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer(wlr_renderer);
    if !wlr_egl_bind_display((*renderer).egl, wl_display) {
        _wlr_log(WLR_INFO,
                 b"[%s:%d] failed to bind wl_display to EGL\x00" as *const u8
                     as *const libc::c_char,
                 b"../render/gles2/renderer.c\x00" as *const u8 as
                     *const libc::c_char, 354i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_gles2_renderer_get_egl(mut wlr_renderer:
                                                        *mut wlr_renderer)
 -> *mut wlr_egl {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer(wlr_renderer);
    return (*renderer).egl;
}
unsafe extern "C" fn gles2_destroy(mut wlr_renderer: *mut wlr_renderer) {
    let mut renderer: *mut wlr_gles2_renderer =
        gles2_get_renderer(wlr_renderer);
    wlr_egl_make_current((*renderer).egl, 0 as EGLSurface,
                         0 as *mut libc::c_int);
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"gles2_destroy\x00")).as_ptr());
    glDeleteProgram((*renderer).shaders.quad.program);
    glDeleteProgram((*renderer).shaders.ellipse.program);
    glDeleteProgram((*renderer).shaders.tex_rgba.program);
    glDeleteProgram((*renderer).shaders.tex_rgbx.program);
    glDeleteProgram((*renderer).shaders.tex_ext.program);
    pop_gles2_marker();
    if (*renderer).exts.debug_khr {
        glDisable(0x92e0i32 as GLenum);
        glDebugMessageCallbackKHR.expect("non-null function pointer")(None,
                                                                      0 as
                                                                          *const libc::c_void);
    }
    free(renderer as *mut libc::c_void);
}
static mut renderer_impl: wlr_renderer_impl =
    {
    
        {
            let mut init =
                wlr_renderer_impl{begin:
                                      Some(gles2_begin as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        uint32_t)
                                                   -> ()),
                                  end:
                                      Some(gles2_end as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer)
                                                   -> ()),
                                  clear:
                                      Some(gles2_clear as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *const libc::c_float)
                                                   -> ()),
                                  scissor:
                                      Some(gles2_scissor as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *mut wlr_box)
                                                   -> ()),
                                  render_texture_with_matrix:
                                      Some(gles2_render_texture_with_matrix as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *mut wlr_texture,
                                                                    _:
                                                                        *const libc::c_float,
                                                                    _:
                                                                        libc::c_float)
                                                   -> bool),
                                  render_quad_with_matrix:
                                      Some(gles2_render_quad_with_matrix as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *const libc::c_float,
                                                                    _:
                                                                        *const libc::c_float)
                                                   -> ()),
                                  render_ellipse_with_matrix:
                                      Some(gles2_render_ellipse_with_matrix as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *const libc::c_float,
                                                                    _:
                                                                        *const libc::c_float)
                                                   -> ()),
                                  formats:
                                      Some(gles2_renderer_formats as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *mut size_t)
                                                   -> *const wl_shm_format),
                                  format_supported:
                                      Some(gles2_format_supported as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        wl_shm_format)
                                                   -> bool),
                                  resource_is_wl_drm_buffer:
                                      Some(gles2_resource_is_wl_drm_buffer as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *mut wl_resource)
                                                   -> bool),
                                  wl_drm_buffer_get_size:
                                      Some(gles2_wl_drm_buffer_get_size as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *mut wl_resource,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut libc::c_int)
                                                   -> ()),
                                  get_dmabuf_formats:
                                      Some(gles2_get_dmabuf_formats as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer)
                                                   ->
                                                       *const wlr_drm_format_set),
                                  preferred_read_format:
                                      Some(gles2_preferred_read_format as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer)
                                                   -> wl_shm_format),
                                  read_pixels:
                                      Some(gles2_read_pixels as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        wl_shm_format,
                                                                    _:
                                                                        *mut uint32_t,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> bool),
                                  texture_from_pixels:
                                      Some(gles2_texture_from_pixels as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        wl_shm_format,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        uint32_t,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> *mut wlr_texture),
                                  texture_from_wl_drm:
                                      Some(gles2_texture_from_wl_drm as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *mut wl_resource)
                                                   -> *mut wlr_texture),
                                  texture_from_dmabuf:
                                      Some(gles2_texture_from_dmabuf as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *mut wlr_dmabuf_attributes)
                                                   -> *mut wlr_texture),
                                  destroy:
                                      Some(gles2_destroy as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer)
                                                   -> ()),
                                  init_wl_display:
                                      Some(gles2_init_wl_display as
                                               unsafe extern "C" fn(_:
                                                                        *mut wlr_renderer,
                                                                    _:
                                                                        *mut wl_display)
                                                   -> ()),};
            init
        }
};
#[no_mangle]
pub unsafe extern "C" fn push_gles2_marker(mut file: *const libc::c_char,
                                           mut func: *const libc::c_char) {
    if glPushDebugGroupKHR.is_none() { return }
    let mut len: libc::c_int =
        snprintf(0 as *mut libc::c_char, 0i32 as libc::c_ulong,
                 b"%s:%s\x00" as *const u8 as *const libc::c_char, file, func)
            + 1i32;
    let vla = len as usize;
    let mut str: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
    snprintf(str.as_mut_ptr(), len as libc::c_ulong,
             b"%s:%s\x00" as *const u8 as *const libc::c_char, file, func);
    glPushDebugGroupKHR.expect("non-null function pointer")(0x824ai32 as
                                                                GLenum,
                                                            1i32 as GLuint,
                                                            -1i32,
                                                            str.as_mut_ptr());
}
// Basically:
	//   GL_TEXTURE_2D == mutable
	//   GL_TEXTURE_EXTERNAL_OES == immutable
// Only affects target == GL_TEXTURE_2D
// used to interpret upload data
#[no_mangle]
pub unsafe extern "C" fn pop_gles2_marker() {
    if glPopDebugGroupKHR.is_some() {
        glPopDebugGroupKHR.expect("non-null function pointer")();
    };
}
unsafe extern "C" fn gles2_log_importance_to_wlr(mut type_0: GLenum)
 -> wlr_log_importance {
    match type_0 {
        33356 => { return WLR_ERROR }
        33357 => { return WLR_DEBUG }
        33358 => { return WLR_ERROR }
        33359 => { return WLR_DEBUG }
        33360 => { return WLR_DEBUG }
        33361 => { return WLR_DEBUG }
        33384 => { return WLR_DEBUG }
        33385 => { return WLR_DEBUG }
        33386 => { return WLR_DEBUG }
        _ => { return WLR_DEBUG }
    };
}
unsafe extern "C" fn gles2_log(mut src: GLenum, mut type_0: GLenum,
                               mut id: GLuint, mut severity: GLenum,
                               mut len: GLsizei, mut msg: *const GLchar,
                               mut user: *const libc::c_void) {
    _wlr_log(gles2_log_importance_to_wlr(type_0),
             b"[GLES2] %s\x00" as *const u8 as *const libc::c_char, msg);
}
unsafe extern "C" fn compile_shader(mut type_0: GLuint,
                                    mut src: *const GLchar) -> GLuint {
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"compile_shader\x00")).as_ptr());
    let mut shader: GLuint = glCreateShader(type_0);
    glShaderSource(shader, 1i32, &mut src, 0 as *const GLint);
    glCompileShader(shader);
    let mut ok: GLint = 0;
    glGetShaderiv(shader, 0x8b81i32 as GLenum, &mut ok);
    if ok == 0i32 { glDeleteShader(shader); shader = 0i32 as GLuint }
    pop_gles2_marker();
    return shader;
}
unsafe extern "C" fn link_program(mut vert_src: *const GLchar,
                                  mut frag_src: *const GLchar) -> GLuint {
    let mut frag: GLuint = 0;
    let mut prog: GLuint = 0;
    let mut ok: GLint = 0;
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"link_program\x00")).as_ptr());
    let mut vert: GLuint = compile_shader(0x8b31i32 as GLuint, vert_src);
    if !(vert == 0) {
        frag = compile_shader(0x8b30i32 as GLuint, frag_src);
        if frag == 0 {
            glDeleteShader(vert);
        } else {
            prog = glCreateProgram();
            glAttachShader(prog, vert);
            glAttachShader(prog, frag);
            glLinkProgram(prog);
            glDetachShader(prog, vert);
            glDetachShader(prog, frag);
            glDeleteShader(vert);
            glDeleteShader(frag);
            ok = 0;
            glGetProgramiv(prog, 0x8b82i32 as GLenum, &mut ok);
            if ok == 0i32 {
                glDeleteProgram(prog);
            } else { pop_gles2_marker(); return prog }
        }
    }
    pop_gles2_marker();
    return 0i32 as GLuint;
}
unsafe extern "C" fn check_gl_ext(mut exts: *const libc::c_char,
                                  mut ext: *const libc::c_char) -> bool {
    let mut extlen: size_t = strlen(ext);
    let mut end: *const libc::c_char = exts.offset(strlen(exts) as isize);
    while exts < end {
        if *exts.offset(0) as libc::c_int == ' ' as i32 {
            exts = exts.offset(1)
        } else {
            let mut n: size_t =
                strcspn(exts, b" \x00" as *const u8 as *const libc::c_char);
            if n == extlen && strncmp(ext, exts, n) == 0i32 {
                return 1i32 != 0
            }
            exts = exts.offset(n as isize)
        }
    }
    return 0i32 != 0;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_gles2_renderer_create(mut egl: *mut wlr_egl)
 -> *mut wlr_renderer {
    let mut current_block: u64;
    if !load_glapi() { return 0 as *mut wlr_renderer }
    let mut renderer: *mut wlr_gles2_renderer =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_gles2_renderer>() as libc::c_ulong)
            as *mut wlr_gles2_renderer;
    if renderer.is_null() { return 0 as *mut wlr_renderer }
    wlr_renderer_init(&mut (*renderer).wlr_renderer, &renderer_impl);
    (*renderer).egl = egl;
    if !wlr_egl_make_current((*renderer).egl, 0 as EGLSurface,
                             0 as *mut libc::c_int) {
        free(renderer as *mut libc::c_void);
        return 0 as *mut wlr_renderer
    }
    (*renderer).exts_str =
        glGetString(0x1f03i32 as GLenum) as *const libc::c_char;
    _wlr_log(WLR_INFO,
             b"[%s:%d] Using %s\x00" as *const u8 as *const libc::c_char,
             b"../render/gles2/renderer.c\x00" as *const u8 as
                 *const libc::c_char, 546i32,
             glGetString(0x1f02i32 as GLenum));
    _wlr_log(WLR_INFO,
             b"[%s:%d] GL vendor: %s\x00" as *const u8 as *const libc::c_char,
             b"../render/gles2/renderer.c\x00" as *const u8 as
                 *const libc::c_char, 547i32,
             glGetString(0x1f00i32 as GLenum));
    _wlr_log(WLR_INFO,
             b"[%s:%d] GL renderer: %s\x00" as *const u8 as
                 *const libc::c_char,
             b"../render/gles2/renderer.c\x00" as *const u8 as
                 *const libc::c_char, 548i32,
             glGetString(0x1f01i32 as GLenum));
    _wlr_log(WLR_INFO,
             b"[%s:%d] Supported GLES2 extensions: %s\x00" as *const u8 as
                 *const libc::c_char,
             b"../render/gles2/renderer.c\x00" as *const u8 as
                 *const libc::c_char, 549i32, (*renderer).exts_str);
    if !check_gl_ext((*renderer).exts_str,
                     b"GL_EXT_texture_format_BGRA8888\x00" as *const u8 as
                         *const libc::c_char) {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] BGRA8888 format not supported by GLES2\x00" as
                     *const u8 as *const libc::c_char,
                 b"../render/gles2/renderer.c\x00" as *const u8 as
                     *const libc::c_char, 552i32);
        free(renderer as *mut libc::c_void);
        return 0 as *mut wlr_renderer
    }
    (*renderer).exts.read_format_bgra_ext =
        check_gl_ext((*renderer).exts_str,
                     b"GL_EXT_read_format_bgra\x00" as *const u8 as
                         *const libc::c_char);
    (*renderer).exts.debug_khr =
        check_gl_ext((*renderer).exts_str,
                     b"GL_KHR_debug\x00" as *const u8 as *const libc::c_char)
            as libc::c_int != 0 && glDebugMessageCallbackKHR.is_some() &&
            glDebugMessageControlKHR.is_some();
    (*renderer).exts.egl_image_external_oes =
        check_gl_ext((*renderer).exts_str,
                     b"GL_OES_EGL_image_external\x00" as *const u8 as
                         *const libc::c_char) as libc::c_int != 0 &&
            glEGLImageTargetTexture2DOES.is_some();
    if (*renderer).exts.debug_khr {
        glEnable(0x92e0i32 as GLenum);
        glEnable(0x8242i32 as GLenum);
        glDebugMessageCallbackKHR.expect("non-null function pointer")(Some(gles2_log
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        GLenum,
                                                                                                    _:
                                                                                                        GLenum,
                                                                                                    _:
                                                                                                        GLuint,
                                                                                                    _:
                                                                                                        GLenum,
                                                                                                    _:
                                                                                                        GLsizei,
                                                                                                    _:
                                                                                                        *const GLchar,
                                                                                                    _:
                                                                                                        *const libc::c_void)
                                                                                   ->
                                                                                       ()),
                                                                      0 as
                                                                          *const libc::c_void);
        // Silence unwanted message types
        glDebugMessageControlKHR.expect("non-null function pointer")(0x1100i32
                                                                         as
                                                                         GLenum,
                                                                     0x826ai32
                                                                         as
                                                                         GLenum,
                                                                     0x1100i32
                                                                         as
                                                                         GLenum,
                                                                     0i32,
                                                                     0 as
                                                                         *const GLuint,
                                                                     0i32 as
                                                                         GLboolean);
        glDebugMessageControlKHR.expect("non-null function pointer")(0x1100i32
                                                                         as
                                                                         GLenum,
                                                                     0x8269i32
                                                                         as
                                                                         GLenum,
                                                                     0x1100i32
                                                                         as
                                                                         GLenum,
                                                                     0i32,
                                                                     0 as
                                                                         *const GLuint,
                                                                     0i32 as
                                                                         GLboolean);
    }
    push_gles2_marker(b"../render/gles2/renderer.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"wlr_gles2_renderer_create\x00")).as_ptr());
    let mut prog: GLuint = 0;
    prog = link_program(quad_vertex_src.as_ptr(), quad_fragment_src.as_ptr());
    (*renderer).shaders.quad.program = prog;
    if !((*renderer).shaders.quad.program == 0) {
        (*renderer).shaders.quad.proj =
            glGetUniformLocation(prog,
                                 b"proj\x00" as *const u8 as
                                     *const libc::c_char);
        (*renderer).shaders.quad.color =
            glGetUniformLocation(prog,
                                 b"color\x00" as *const u8 as
                                     *const libc::c_char);
        prog =
            link_program(quad_vertex_src.as_ptr(),
                         ellipse_fragment_src.as_ptr());
        (*renderer).shaders.ellipse.program = prog;
        if !((*renderer).shaders.ellipse.program == 0) {
            (*renderer).shaders.ellipse.proj =
                glGetUniformLocation(prog,
                                     b"proj\x00" as *const u8 as
                                         *const libc::c_char);
            (*renderer).shaders.ellipse.color =
                glGetUniformLocation(prog,
                                     b"color\x00" as *const u8 as
                                         *const libc::c_char);
            prog =
                link_program(tex_vertex_src.as_ptr(),
                             tex_fragment_src_rgba.as_ptr());
            (*renderer).shaders.tex_rgba.program = prog;
            if !((*renderer).shaders.tex_rgba.program == 0) {
                (*renderer).shaders.tex_rgba.proj =
                    glGetUniformLocation(prog,
                                         b"proj\x00" as *const u8 as
                                             *const libc::c_char);
                (*renderer).shaders.tex_rgba.invert_y =
                    glGetUniformLocation(prog,
                                         b"invert_y\x00" as *const u8 as
                                             *const libc::c_char);
                (*renderer).shaders.tex_rgba.tex =
                    glGetUniformLocation(prog,
                                         b"tex\x00" as *const u8 as
                                             *const libc::c_char);
                (*renderer).shaders.tex_rgba.alpha =
                    glGetUniformLocation(prog,
                                         b"alpha\x00" as *const u8 as
                                             *const libc::c_char);
                prog =
                    link_program(tex_vertex_src.as_ptr(),
                                 tex_fragment_src_rgbx.as_ptr());
                (*renderer).shaders.tex_rgbx.program = prog;
                if !((*renderer).shaders.tex_rgbx.program == 0) {
                    (*renderer).shaders.tex_rgbx.proj =
                        glGetUniformLocation(prog,
                                             b"proj\x00" as *const u8 as
                                                 *const libc::c_char);
                    (*renderer).shaders.tex_rgbx.invert_y =
                        glGetUniformLocation(prog,
                                             b"invert_y\x00" as *const u8 as
                                                 *const libc::c_char);
                    (*renderer).shaders.tex_rgbx.tex =
                        glGetUniformLocation(prog,
                                             b"tex\x00" as *const u8 as
                                                 *const libc::c_char);
                    (*renderer).shaders.tex_rgbx.alpha =
                        glGetUniformLocation(prog,
                                             b"alpha\x00" as *const u8 as
                                                 *const libc::c_char);
                    if (*renderer).exts.egl_image_external_oes {
                        prog =
                            link_program(tex_vertex_src.as_ptr(),
                                         tex_fragment_src_external.as_ptr());
                        (*renderer).shaders.tex_ext.program = prog;
                        if (*renderer).shaders.tex_ext.program == 0 {
                            current_block = 8424609800197991461;
                        } else {
                            (*renderer).shaders.tex_ext.proj =
                                glGetUniformLocation(prog,
                                                     b"proj\x00" as *const u8
                                                         as
                                                         *const libc::c_char);
                            (*renderer).shaders.tex_ext.invert_y =
                                glGetUniformLocation(prog,
                                                     b"invert_y\x00" as
                                                         *const u8 as
                                                         *const libc::c_char);
                            (*renderer).shaders.tex_ext.tex =
                                glGetUniformLocation(prog,
                                                     b"tex\x00" as *const u8
                                                         as
                                                         *const libc::c_char);
                            (*renderer).shaders.tex_ext.alpha =
                                glGetUniformLocation(prog,
                                                     b"alpha\x00" as *const u8
                                                         as
                                                         *const libc::c_char);
                            current_block = 17233182392562552756;
                        }
                    } else { current_block = 17233182392562552756; }
                    match current_block {
                        8424609800197991461 => { }
                        _ => {
                            pop_gles2_marker();
                            return &mut (*renderer).wlr_renderer
                        }
                    }
                }
            }
        }
    }
    glDeleteProgram((*renderer).shaders.quad.program);
    glDeleteProgram((*renderer).shaders.ellipse.program);
    glDeleteProgram((*renderer).shaders.tex_rgba.program);
    glDeleteProgram((*renderer).shaders.tex_rgbx.program);
    glDeleteProgram((*renderer).shaders.tex_ext.program);
    pop_gles2_marker();
    if (*renderer).exts.debug_khr {
        glDisable(0x92e0i32 as GLenum);
        glDebugMessageCallbackKHR.expect("non-null function pointer")(None,
                                                                      0 as
                                                                          *const libc::c_void);
    }
    free(renderer as *mut libc::c_void);
    return 0 as *mut wlr_renderer;
}
