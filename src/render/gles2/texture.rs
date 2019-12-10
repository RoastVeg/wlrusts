use libc;
extern "C" {
    pub type wl_display;
    pub type wl_client;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn glBindTexture(target: GLenum, texture: GLuint);
    #[no_mangle]
    fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
    #[no_mangle]
    fn glTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint,
                       yoffset: GLint, width: GLsizei, height: GLsizei,
                       format: GLenum, type_0: GLenum,
                       pixels: *const libc::c_void);
    #[no_mangle]
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    #[no_mangle]
    fn glPixelStorei(pname: GLenum, param: GLint);
    #[no_mangle]
    fn glTexImage2D(target: GLenum, level: GLint, internalformat: GLint,
                    width: GLsizei, height: GLsizei, border: GLint,
                    format: GLenum, type_0: GLenum,
                    pixels: *const libc::c_void);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wlr_egl_is_current(egl: *mut wlr_egl) -> bool;
    #[no_mangle]
    fn wlr_egl_make_current(egl: *mut wlr_egl, surface: EGLSurface,
                            buffer_age: *mut libc::c_int) -> bool;
    #[no_mangle]
    fn wlr_egl_destroy_image(egl: *mut wlr_egl, image: EGLImageKHR) -> bool;
    #[no_mangle]
    fn wlr_egl_export_image_to_dmabuf(egl: *mut wlr_egl, image: EGLImageKHR,
                                      width: int32_t, height: int32_t,
                                      flags: uint32_t,
                                      attribs: *mut wlr_dmabuf_attributes)
     -> bool;
    #[no_mangle]
    fn wlr_egl_create_image_from_dmabuf(egl: *mut wlr_egl,
                                        attributes:
                                            *mut wlr_dmabuf_attributes)
     -> EGLImageKHR;
    #[no_mangle]
    fn wlr_egl_create_image_from_wl_drm(egl: *mut wlr_egl,
                                        data: *mut wl_resource,
                                        fmt: *mut EGLint,
                                        width: *mut libc::c_int,
                                        height: *mut libc::c_int,
                                        inverted_y: *mut bool) -> EGLImageKHR;
    #[no_mangle]
    fn wlr_texture_init(texture: *mut wlr_texture,
                        impl_0: *const wlr_texture_impl);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    // Returns the log verbosity provided to wlr_log_init
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    static mut eglCreateImageKHR: PFNEGLCREATEIMAGEKHRPROC;
    #[no_mangle]
    static mut glEGLImageTargetTexture2DOES:
           PFNGLEGLIMAGETARGETTEXTURE2DOESPROC;
    #[no_mangle]
    fn pop_gles2_marker();
    #[no_mangle]
    fn get_gles2_format_from_wl(fmt: wl_shm_format)
     -> *const wlr_gles2_pixel_format;
    #[no_mangle]
    fn push_gles2_marker(file: *const libc::c_char,
                         func: *const libc::c_char);
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type khronos_int32_t = int32_t;
pub type GLenum = libc::c_uint;
pub type GLuint = libc::c_uint;
pub type GLint = libc::c_int;
pub type GLsizei = libc::c_int;
pub type GLeglImageOES = *mut libc::c_void;
pub type PFNGLEGLIMAGETARGETTEXTURE2DOESPROC
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLeglImageOES) -> ()>;
pub type size_t = libc::c_ulong;
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
pub type EGLint = khronos_int32_t;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
pub type EGLClientBuffer = *mut libc::c_void;
pub type EGLImageKHR = *mut libc::c_void;
pub type PFNEGLCREATEIMAGEKHRPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLContext, _: EGLenum,
                                _: EGLClientBuffer, _: *const EGLint)
               -> EGLImageKHR>;
pub type wlr_dmabuf_attributes_flags = libc::c_uint;
pub const WLR_DMABUF_ATTRIBUTES_FLAGS_BOTTOM_FIRST:
          wlr_dmabuf_attributes_flags =
    4;
pub const WLR_DMABUF_ATTRIBUTES_FLAGS_INTERLACED: wlr_dmabuf_attributes_flags
          =
    2;
pub const WLR_DMABUF_ATTRIBUTES_FLAGS_Y_INVERT: wlr_dmabuf_attributes_flags =
    1;
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
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
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
    // used to interpret upload data
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_gles2_pixel_format {
    pub wl_format: wl_shm_format,
    pub gl_format: GLint,
    pub gl_type: GLint,
    pub depth: libc::c_int,
    pub bpp: libc::c_int,
    pub has_alpha: bool,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_gles2_texture_attribs {
    pub target: GLenum,
    pub tex: GLuint,
    pub inverted_y: bool,
    pub has_alpha: bool,
}
#[no_mangle]
pub unsafe extern "C" fn wlr_texture_is_gles2(mut wlr_texture:
                                                  *mut wlr_texture) -> bool {
    return (*wlr_texture).impl_0 == &texture_impl as *const wlr_texture_impl;
}
#[no_mangle]
pub unsafe extern "C" fn gles2_get_texture(mut wlr_texture: *mut wlr_texture)
 -> *mut wlr_gles2_texture {
    if wlr_texture_is_gles2(wlr_texture) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_texture_is_gles2(wlr_texture)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/gles2/texture.c\x00" as *const u8 as
                          *const libc::c_char, 26i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"struct wlr_gles2_texture *gles2_get_texture(struct wlr_texture *)\x00")).as_ptr());
    };
    return wlr_texture as *mut wlr_gles2_texture;
}
unsafe extern "C" fn get_gles2_texture_in_context(mut wlr_texture:
                                                      *mut wlr_texture)
 -> *mut wlr_gles2_texture {
    let mut texture: *mut wlr_gles2_texture = gles2_get_texture(wlr_texture);
    if !wlr_egl_is_current((*texture).egl) {
        wlr_egl_make_current((*texture).egl, 0 as EGLSurface,
                             0 as *mut libc::c_int);
    }
    return texture;
}
unsafe extern "C" fn gles2_texture_get_size(mut wlr_texture: *mut wlr_texture,
                                            mut width: *mut libc::c_int,
                                            mut height: *mut libc::c_int) {
    let mut texture: *mut wlr_gles2_texture = gles2_get_texture(wlr_texture);
    *width = (*texture).width;
    *height = (*texture).height;
}
unsafe extern "C" fn gles2_texture_is_opaque(mut wlr_texture:
                                                 *mut wlr_texture) -> bool {
    let mut texture: *mut wlr_gles2_texture = gles2_get_texture(wlr_texture);
    return !(*texture).has_alpha;
}
unsafe extern "C" fn gles2_texture_write_pixels(mut wlr_texture:
                                                    *mut wlr_texture,
                                                mut stride: uint32_t,
                                                mut width: uint32_t,
                                                mut height: uint32_t,
                                                mut src_x: uint32_t,
                                                mut src_y: uint32_t,
                                                mut dst_x: uint32_t,
                                                mut dst_y: uint32_t,
                                                mut data: *const libc::c_void)
 -> bool {
    let mut texture: *mut wlr_gles2_texture =
        get_gles2_texture_in_context(wlr_texture);
    if (*texture).target != 0xde1i32 as libc::c_uint {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot write pixels to immutable texture\x00" as
                     *const u8 as *const libc::c_char,
                 b"../render/gles2/texture.c\x00" as *const u8 as
                     *const libc::c_char, 59i32);
        return 0i32 != 0
    }
    let mut fmt: *const wlr_gles2_pixel_format =
        get_gles2_format_from_wl((*texture).wl_format);
    if !fmt.is_null() {
    } else {
        __assert_fail(b"fmt\x00" as *const u8 as *const libc::c_char,
                      b"../render/gles2/texture.c\x00" as *const u8 as
                          *const libc::c_char, 65i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 139],
                                                &[libc::c_char; 139]>(b"_Bool gles2_texture_write_pixels(struct wlr_texture *, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t, const void *)\x00")).as_ptr());
    };
    // TODO: what if the unpack subimage extension isn't supported?
    push_gles2_marker(b"../render/gles2/texture.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"gles2_texture_write_pixels\x00")).as_ptr()); // texture can't be written anyways
    glBindTexture(0xde1i32 as GLenum, (*texture).tex);
    glPixelStorei(0xcf2i32 as GLenum,
                  stride.wrapping_div(((*fmt).bpp / 8i32) as libc::c_uint) as
                      GLint);
    glPixelStorei(0xcf4i32 as GLenum, src_x as GLint);
    glPixelStorei(0xcf3i32 as GLenum, src_y as GLint);
    glTexSubImage2D(0xde1i32 as GLenum, 0i32, dst_x as GLint, dst_y as GLint,
                    width as GLsizei, height as GLsizei,
                    (*fmt).gl_format as GLenum, (*fmt).gl_type as GLenum,
                    data);
    glPixelStorei(0xcf2i32 as GLenum, 0i32);
    glPixelStorei(0xcf4i32 as GLenum, 0i32);
    glPixelStorei(0xcf3i32 as GLenum, 0i32);
    pop_gles2_marker();
    return 1i32 != 0;
}
unsafe extern "C" fn gles2_texture_to_dmabuf(mut wlr_texture:
                                                 *mut wlr_texture,
                                             mut attribs:
                                                 *mut wlr_dmabuf_attributes)
 -> bool {
    let mut texture: *mut wlr_gles2_texture = gles2_get_texture(wlr_texture);
    if (*texture).image.is_null() {
        if (*texture).target == 0xde1i32 as libc::c_uint {
        } else {
            __assert_fail(b"texture->target == GL_TEXTURE_2D\x00" as *const u8
                              as *const libc::c_char,
                          b"../render/gles2/texture.c\x00" as *const u8 as
                              *const libc::c_char, 92i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 84],
                                                    &[libc::c_char; 84]>(b"_Bool gles2_texture_to_dmabuf(struct wlr_texture *, struct wlr_dmabuf_attributes *)\x00")).as_ptr());
        };
        if eglCreateImageKHR.is_none() { return 0i32 != 0 }
        (*texture).image =
            eglCreateImageKHR.expect("non-null function pointer")((*(*texture).egl).display,
                                                                  (*(*texture).egl).context,
                                                                  0x30b1i32 as
                                                                      EGLenum,
                                                                  (*texture).tex
                                                                      as
                                                                      uintptr_t
                                                                      as
                                                                      EGLClientBuffer,
                                                                  0 as
                                                                      *const EGLint);
        if (*texture).image.is_null() { return 0i32 != 0 }
    }
    let mut flags: uint32_t = 0i32 as uint32_t;
    if (*texture).inverted_y {
        flags |=
            WLR_DMABUF_ATTRIBUTES_FLAGS_Y_INVERT as libc::c_int as
                libc::c_uint
    }
    return wlr_egl_export_image_to_dmabuf((*texture).egl, (*texture).image,
                                          (*texture).width, (*texture).height,
                                          flags, attribs);
}
unsafe extern "C" fn gles2_texture_destroy(mut wlr_texture:
                                               *mut wlr_texture) {
    if wlr_texture.is_null() { return }
    let mut texture: *mut wlr_gles2_texture = gles2_get_texture(wlr_texture);
    if !wlr_egl_is_current((*texture).egl) {
        wlr_egl_make_current((*texture).egl, 0 as EGLSurface,
                             0 as *mut libc::c_int);
    }
    push_gles2_marker(b"../render/gles2/texture.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"gles2_texture_destroy\x00")).as_ptr());
    glDeleteTextures(1i32, &mut (*texture).tex);
    wlr_egl_destroy_image((*texture).egl, (*texture).image);
    pop_gles2_marker();
    free(texture as *mut libc::c_void);
}
static mut texture_impl: wlr_texture_impl =
    unsafe {
        {
            let mut init =
                wlr_texture_impl{get_size:
                                     Some(gles2_texture_get_size as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_texture,
                                                                   _:
                                                                       *mut libc::c_int,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> ()),
                                 is_opaque:
                                     Some(gles2_texture_is_opaque as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_texture)
                                                  -> bool),
                                 write_pixels:
                                     Some(gles2_texture_write_pixels as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_texture,
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
                                                                       *const libc::c_void)
                                                  -> bool),
                                 to_dmabuf:
                                     Some(gles2_texture_to_dmabuf as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_texture,
                                                                   _:
                                                                       *mut wlr_dmabuf_attributes)
                                                  -> bool),
                                 destroy:
                                     Some(gles2_texture_destroy as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_texture)
                                                  -> ()),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn wlr_gles2_texture_from_pixels(mut egl: *mut wlr_egl,
                                                       mut wl_fmt:
                                                           wl_shm_format,
                                                       mut stride: uint32_t,
                                                       mut width: uint32_t,
                                                       mut height: uint32_t,
                                                       mut data:
                                                           *const libc::c_void)
 -> *mut wlr_texture {
    if !wlr_egl_is_current(egl) {
        wlr_egl_make_current(egl, 0 as EGLSurface, 0 as *mut libc::c_int);
    }
    let mut fmt: *const wlr_gles2_pixel_format =
        get_gles2_format_from_wl(wl_fmt);
    if fmt.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unsupported pixel format %u\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/gles2/texture.c\x00" as *const u8 as
                     *const libc::c_char, 153i32, wl_fmt as libc::c_uint);
        return 0 as *mut wlr_texture
    }
    let mut texture: *mut wlr_gles2_texture =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_gles2_texture>() as libc::c_ulong) as
            *mut wlr_gles2_texture;
    if texture.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/gles2/texture.c\x00" as *const u8 as
                     *const libc::c_char, 160i32);
        return 0 as *mut wlr_texture
    }
    wlr_texture_init(&mut (*texture).wlr_texture, &texture_impl);
    (*texture).egl = egl;
    (*texture).width = width as libc::c_int;
    (*texture).height = height as libc::c_int;
    (*texture).target = 0xde1i32 as GLenum;
    (*texture).has_alpha = (*fmt).has_alpha;
    (*texture).wl_format = (*fmt).wl_format;
    push_gles2_marker(b"../render/gles2/texture.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"wlr_gles2_texture_from_pixels\x00")).as_ptr());
    glGenTextures(1i32, &mut (*texture).tex);
    glBindTexture(0xde1i32 as GLenum, (*texture).tex);
    glPixelStorei(0xcf2i32 as GLenum,
                  stride.wrapping_div(((*fmt).bpp / 8i32) as libc::c_uint) as
                      GLint);
    glTexImage2D(0xde1i32 as GLenum, 0i32, (*fmt).gl_format, width as GLsizei,
                 height as GLsizei, 0i32, (*fmt).gl_format as GLenum,
                 (*fmt).gl_type as GLenum, data);
    glPixelStorei(0xcf2i32 as GLenum, 0i32);
    pop_gles2_marker();
    return &mut (*texture).wlr_texture;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_gles2_texture_from_wl_drm(mut egl: *mut wlr_egl,
                                                       mut data:
                                                           *mut wl_resource)
 -> *mut wlr_texture {
    if !wlr_egl_is_current(egl) {
        wlr_egl_make_current(egl, 0 as EGLSurface, 0 as *mut libc::c_int);
    }
    if glEGLImageTargetTexture2DOES.is_none() { return 0 as *mut wlr_texture }
    let mut texture: *mut wlr_gles2_texture =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_gles2_texture>() as libc::c_ulong) as
            *mut wlr_gles2_texture;
    if texture.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/gles2/texture.c\x00" as *const u8 as
                     *const libc::c_char, 198i32);
        return 0 as *mut wlr_texture
    }
    wlr_texture_init(&mut (*texture).wlr_texture, &texture_impl);
    (*texture).egl = egl;
    let mut fmt: EGLint = 0;
    (*texture).wl_format = 4294967295 as wl_shm_format;
    (*texture).image =
        wlr_egl_create_image_from_wl_drm(egl, data, &mut fmt,
                                         &mut (*texture).width,
                                         &mut (*texture).height,
                                         &mut (*texture).inverted_y);
    if (*texture).image.is_null() {
        free(texture as *mut libc::c_void);
        return 0 as *mut wlr_texture
    }
    match fmt {
        12381 => { (*texture).has_alpha = 0i32 != 0 }
        12382 | 12762 => { (*texture).has_alpha = 1i32 != 0 }
        _ => {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Invalid or unsupported EGL buffer format\x00"
                         as *const u8 as *const libc::c_char,
                     b"../render/gles2/texture.c\x00" as *const u8 as
                         *const libc::c_char, 222i32);
            free(texture as *mut libc::c_void);
            return 0 as *mut wlr_texture
        }
    }
    (*texture).target = 0x8d65i32 as GLenum;
    push_gles2_marker(b"../render/gles2/texture.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"wlr_gles2_texture_from_wl_drm\x00")).as_ptr());
    glGenTextures(1i32, &mut (*texture).tex);
    glBindTexture(0x8d65i32 as GLenum, (*texture).tex);
    glEGLImageTargetTexture2DOES.expect("non-null function pointer")(0x8d65i32
                                                                         as
                                                                         GLenum,
                                                                     (*texture).image);
    pop_gles2_marker();
    return &mut (*texture).wlr_texture;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_gles2_texture_from_dmabuf(mut egl: *mut wlr_egl,
                                                       mut attribs:
                                                           *mut wlr_dmabuf_attributes)
 -> *mut wlr_texture {
    if !wlr_egl_is_current(egl) {
        wlr_egl_make_current(egl, 0 as EGLSurface, 0 as *mut libc::c_int);
    }
    if glEGLImageTargetTexture2DOES.is_none() { return 0 as *mut wlr_texture }
    if !(*egl).exts.image_dmabuf_import_ext {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot create DMA-BUF texture: EGL extension unavailable\x00"
                     as *const u8 as *const libc::c_char,
                 b"../render/gles2/texture.c\x00" as *const u8 as
                     *const libc::c_char, 251i32);
        return 0 as *mut wlr_texture
    }
    match (*attribs).format & !(1i32 << 31i32) as libc::c_uint {
        1448695129 | 1431918169 | 1498831189 | 1498765654 | 1448433985 => {
            // TODO: YUV based formats not yet supported, require multiple images
            return 0 as *mut wlr_texture
        }
        _ => { }
    } // texture can't be written anyways
    let mut texture: *mut wlr_gles2_texture =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_gles2_texture>() as libc::c_ulong) as
            *mut wlr_gles2_texture;
    if texture.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/gles2/texture.c\x00" as *const u8 as
                     *const libc::c_char, 270i32);
        return 0 as *mut wlr_texture
    }
    wlr_texture_init(&mut (*texture).wlr_texture, &texture_impl);
    (*texture).egl = egl;
    (*texture).width = (*attribs).width;
    (*texture).height = (*attribs).height;
    (*texture).target = 0x8d65i32 as GLenum;
    (*texture).has_alpha = 1i32 != 0;
    (*texture).wl_format = 4294967295 as wl_shm_format;
    (*texture).inverted_y =
        (*attribs).flags &
            WLR_DMABUF_ATTRIBUTES_FLAGS_Y_INVERT as libc::c_int as
                libc::c_uint != 0i32 as libc::c_uint;
    (*texture).image = wlr_egl_create_image_from_dmabuf(egl, attribs);
    if (*texture).image.is_null() {
        free(texture as *mut libc::c_void);
        return 0 as *mut wlr_texture
    }
    push_gles2_marker(b"../render/gles2/texture.c\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"wlr_gles2_texture_from_dmabuf\x00")).as_ptr());
    glGenTextures(1i32, &mut (*texture).tex);
    glBindTexture(0x8d65i32 as GLenum, (*texture).tex);
    glEGLImageTargetTexture2DOES.expect("non-null function pointer")(0x8d65i32
                                                                         as
                                                                         GLenum,
                                                                     (*texture).image);
    pop_gles2_marker();
    return &mut (*texture).wlr_texture;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* either GL_TEXTURE_2D or GL_TEXTURE_EXTERNAL_OES */
#[no_mangle]
pub unsafe extern "C" fn wlr_gles2_texture_get_attribs(mut wlr_texture:
                                                           *mut wlr_texture,
                                                       mut attribs:
                                                           *mut wlr_gles2_texture_attribs) {
    let mut texture: *mut wlr_gles2_texture = gles2_get_texture(wlr_texture);
    memset(attribs as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<wlr_gles2_texture_attribs>() as
               libc::c_ulong);
    (*attribs).target = (*texture).target;
    (*attribs).tex = (*texture).tex;
    (*attribs).inverted_y = (*texture).inverted_y;
    (*attribs).has_alpha = (*texture).has_alpha;
}
