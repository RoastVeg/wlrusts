use libc;
extern "C" {
    pub type wl_display;
    pub type wl_client;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
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
#[no_mangle]
pub unsafe extern "C" fn wlr_texture_init(mut texture: *mut wlr_texture,
                                          mut impl_0:
                                              *const wlr_texture_impl) {
    if (*impl_0).get_size.is_some() {
    } else {
        __assert_fail(b"impl->get_size\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/wlr_texture.c\x00" as *const u8 as
                          *const libc::c_char, 9i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 77],
                                                &[libc::c_char; 77]>(b"void wlr_texture_init(struct wlr_texture *, const struct wlr_texture_impl *)\x00")).as_ptr());
    };
    if (*impl_0).write_pixels.is_some() {
    } else {
        __assert_fail(b"impl->write_pixels\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/wlr_texture.c\x00" as *const u8 as
                          *const libc::c_char, 10i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 77],
                                                &[libc::c_char; 77]>(b"void wlr_texture_init(struct wlr_texture *, const struct wlr_texture_impl *)\x00")).as_ptr());
    };
    (*texture).impl_0 = impl_0;
}
/* *
 * Destroys this wlr_texture.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_texture_destroy(mut texture: *mut wlr_texture) {
    if !texture.is_null() && !(*texture).impl_0.is_null() &&
           (*(*texture).impl_0).destroy.is_some() {
        (*(*texture).impl_0).destroy.expect("non-null function pointer")(texture);
    } else { free(texture as *mut libc::c_void); };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_texture_from_pixels(mut renderer:
                                                     *mut wlr_renderer,
                                                 mut wl_fmt: wl_shm_format,
                                                 mut stride: uint32_t,
                                                 mut width: uint32_t,
                                                 mut height: uint32_t,
                                                 mut data:
                                                     *const libc::c_void)
 -> *mut wlr_texture {
    return (*(*renderer).impl_0).texture_from_pixels.expect("non-null function pointer")(renderer,
                                                                                         wl_fmt,
                                                                                         stride,
                                                                                         width,
                                                                                         height,
                                                                                         data);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_texture_from_wl_drm(mut renderer:
                                                     *mut wlr_renderer,
                                                 mut data: *mut wl_resource)
 -> *mut wlr_texture {
    if (*(*renderer).impl_0).texture_from_wl_drm.is_none() {
        return 0 as *mut wlr_texture
    }
    return (*(*renderer).impl_0).texture_from_wl_drm.expect("non-null function pointer")(renderer,
                                                                                         data);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_texture_from_dmabuf(mut renderer:
                                                     *mut wlr_renderer,
                                                 mut attribs:
                                                     *mut wlr_dmabuf_attributes)
 -> *mut wlr_texture {
    if (*(*renderer).impl_0).texture_from_dmabuf.is_none() {
        return 0 as *mut wlr_texture
    }
    return (*(*renderer).impl_0).texture_from_dmabuf.expect("non-null function pointer")(renderer,
                                                                                         attribs);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_texture_get_size(mut texture: *mut wlr_texture,
                                              mut width: *mut libc::c_int,
                                              mut height: *mut libc::c_int) {
    (*(*texture).impl_0).get_size.expect("non-null function pointer")(texture,
                                                                      width,
                                                                      height);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_texture_is_opaque(mut texture: *mut wlr_texture)
 -> bool {
    if (*(*texture).impl_0).is_opaque.is_none() { return 0i32 != 0 }
    return (*(*texture).impl_0).is_opaque.expect("non-null function pointer")(texture);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_texture_write_pixels(mut texture:
                                                      *mut wlr_texture,
                                                  mut stride: uint32_t,
                                                  mut width: uint32_t,
                                                  mut height: uint32_t,
                                                  mut src_x: uint32_t,
                                                  mut src_y: uint32_t,
                                                  mut dst_x: uint32_t,
                                                  mut dst_y: uint32_t,
                                                  mut data:
                                                      *const libc::c_void)
 -> bool {
    return (*(*texture).impl_0).write_pixels.expect("non-null function pointer")(texture,
                                                                                 stride,
                                                                                 width,
                                                                                 height,
                                                                                 src_x,
                                                                                 src_y,
                                                                                 dst_x,
                                                                                 dst_y,
                                                                                 data);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
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
#[no_mangle]
pub unsafe extern "C" fn wlr_texture_to_dmabuf(mut texture: *mut wlr_texture,
                                               mut attribs:
                                                   *mut wlr_dmabuf_attributes)
 -> bool {
    if (*(*texture).impl_0).to_dmabuf.is_none() { return 0i32 != 0 }
    return (*(*texture).impl_0).to_dmabuf.expect("non-null function pointer")(texture,
                                                                              attribs);
}
