use libc;
extern "C" {
    pub type wl_client;
    pub type wl_shm_buffer;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
    #[no_mangle]
    fn wl_resource_post_error(resource: *mut wl_resource, code: uint32_t,
                              msg: *const libc::c_char, _: ...);
    #[no_mangle]
    fn wl_resource_get_class(resource: *mut wl_resource)
     -> *const libc::c_char;
    #[no_mangle]
    fn wl_resource_add_destroy_listener(resource: *mut wl_resource,
                                        listener: *mut wl_listener);
    #[no_mangle]
    fn wl_shm_buffer_get(resource: *mut wl_resource) -> *mut wl_shm_buffer;
    #[no_mangle]
    fn wl_shm_buffer_begin_access(buffer: *mut wl_shm_buffer);
    #[no_mangle]
    fn wl_shm_buffer_end_access(buffer: *mut wl_shm_buffer);
    #[no_mangle]
    fn wl_shm_buffer_get_data(buffer: *mut wl_shm_buffer)
     -> *mut libc::c_void;
    #[no_mangle]
    fn wl_shm_buffer_get_stride(buffer: *mut wl_shm_buffer) -> int32_t;
    #[no_mangle]
    fn wl_shm_buffer_get_format(buffer: *mut wl_shm_buffer) -> uint32_t;
    #[no_mangle]
    fn wl_shm_buffer_get_width(buffer: *mut wl_shm_buffer) -> int32_t;
    #[no_mangle]
    fn wl_shm_buffer_get_height(buffer: *mut wl_shm_buffer) -> int32_t;
    #[no_mangle]
    static wl_buffer_interface: wl_interface;
    #[no_mangle]
    fn wlr_renderer_wl_drm_buffer_get_size(renderer: *mut wlr_renderer,
                                           buffer: *mut wl_resource,
                                           width: *mut libc::c_int,
                                           height: *mut libc::c_int);
    #[no_mangle]
    fn wlr_renderer_resource_is_wl_drm_buffer(renderer: *mut wlr_renderer,
                                              buffer: *mut wl_resource)
     -> bool;
    #[no_mangle]
    fn pixman_region32_rectangles(region: *mut pixman_region32_t,
                                  n_rects: *mut libc::c_int)
     -> *mut pixman_box32_t;
    #[no_mangle]
    fn wlr_texture_from_pixels(renderer: *mut wlr_renderer,
                               wl_fmt: wl_shm_format, stride: uint32_t,
                               width: uint32_t, height: uint32_t,
                               data: *const libc::c_void) -> *mut wlr_texture;
    /* *
 * Create a new texture from a wl_drm resource. The returned texture is
 * immutable.
 */
    #[no_mangle]
    fn wlr_texture_from_wl_drm(renderer: *mut wlr_renderer,
                               data: *mut wl_resource) -> *mut wlr_texture;
    /* *
 * Create a new texture from a DMA-BUF. The returned texture is immutable.
 */
    #[no_mangle]
    fn wlr_texture_from_dmabuf(renderer: *mut wlr_renderer,
                               attribs: *mut wlr_dmabuf_attributes)
     -> *mut wlr_texture;
    /* *
 * Get the texture width and height.
 */
    #[no_mangle]
    fn wlr_texture_get_size(texture: *mut wlr_texture,
                            width: *mut libc::c_int,
                            height: *mut libc::c_int);
    /* *
  * Update a texture with raw pixels. The texture must be mutable, and the input
  * data must have the same pixel format that the texture was created with.
  */
    #[no_mangle]
    fn wlr_texture_write_pixels(texture: *mut wlr_texture, stride: uint32_t,
                                width: uint32_t, height: uint32_t,
                                src_x: uint32_t, src_y: uint32_t,
                                dst_x: uint32_t, dst_y: uint32_t,
                                data: *const libc::c_void) -> bool;
    /* *
 * Destroys this wlr_texture.
 */
    #[no_mangle]
    fn wlr_texture_destroy(texture: *mut wlr_texture);
    /* *
 * Returns true if the given resource was created via the linux-dmabuf
 * buffer protocol, false otherwise
 */
    #[no_mangle]
    fn wlr_dmabuf_v1_resource_is_buffer(buffer_resource: *mut wl_resource)
     -> bool;
    /* *
 * Returns the wlr_dmabuf_buffer if the given resource was created
 * via the linux-dmabuf buffer protocol
 */
    #[no_mangle]
    fn wlr_dmabuf_v1_buffer_from_buffer_resource(buffer_resource:
                                                     *mut wl_resource)
     -> *mut wlr_dmabuf_v1_buffer;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    // Will log all messages less than or equal to `verbosity`
// If `callback` is NULL, wlr will use its default logger.
// The function can be called multiple times to update the verbosity or
// callback function.
    // Returns the log verbosity provided to wlr_log_init
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

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
pub struct wl_listener {
    pub link: wl_list,
    pub notify: wl_notify_func_t,
}
pub type wl_notify_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_listener, _: *mut libc::c_void)
               -> ()>;

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
/*
 * 32 bit regions
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;

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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_renderer {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_renderer_impl,
    pub events: C2RustUnnamed,
    /* *
 * Create a new texture from raw pixel data. `stride` is in bytes. The returned
 * texture is mutable.
 */
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_texture {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_texture_impl,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * A client buffer.
 */

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_dmabuf_v1_buffer {
    pub renderer: *mut wlr_renderer,
    pub buffer_resource: *mut wl_resource,
    pub params_resource: *mut wl_resource,
    pub attributes: wlr_dmabuf_attributes,
    pub has_modifier: bool,
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
#[inline]
unsafe extern "C" fn wl_buffer_send_release(mut resource_: *mut wl_resource) {
    wl_resource_post_event(resource_, 0i32 as uint32_t);
}
/* *
 * Check if a resource is a wl_buffer resource.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_resource_is_buffer(mut resource:
                                                    *mut wl_resource)
 -> bool {
    return strcmp(wl_resource_get_class(resource), wl_buffer_interface.name)
               == 0i32;
}
/* *
 * Get the size of a wl_buffer resource.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_buffer_get_resource_size(mut resource:
                                                          *mut wl_resource,
                                                      mut renderer:
                                                          *mut wlr_renderer,
                                                      mut width:
                                                          *mut libc::c_int,
                                                      mut height:
                                                          *mut libc::c_int)
 -> bool {
    if wlr_resource_is_buffer(resource) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_resource_is_buffer(resource)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_buffer.c\x00" as *const u8 as
                          *const libc::c_char, 14i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 94],
                                                &[libc::c_char; 94]>(b"_Bool wlr_buffer_get_resource_size(struct wl_resource *, struct wlr_renderer *, int *, int *)\x00")).as_ptr());
    };
    let mut shm_buf: *mut wl_shm_buffer = wl_shm_buffer_get(resource);
    if !shm_buf.is_null() {
        *width = wl_shm_buffer_get_width(shm_buf);
        *height = wl_shm_buffer_get_height(shm_buf)
    } else if wlr_renderer_resource_is_wl_drm_buffer(renderer, resource) {
        wlr_renderer_wl_drm_buffer_get_size(renderer, resource, width,
                                            height);
    } else if wlr_dmabuf_v1_resource_is_buffer(resource) {
        let mut dmabuf: *mut wlr_dmabuf_v1_buffer =
            wlr_dmabuf_v1_buffer_from_buffer_resource(resource);
        *width = (*dmabuf).attributes.width;
        *height = (*dmabuf).attributes.height
    } else { *height = 0i32; *width = *height; return 0i32 != 0 }
    return 1i32 != 0;
}
unsafe extern "C" fn buffer_resource_handle_destroy(mut listener:
                                                        *mut wl_listener,
                                                    mut data:
                                                        *mut libc::c_void) {
    let mut buffer: *mut wlr_buffer =
        (listener as *mut libc::c_char).offset(-32) as *mut wlr_buffer;
    wl_list_remove(&mut (*buffer).resource_destroy.link);
    wl_list_init(&mut (*buffer).resource_destroy.link);
    (*buffer).resource = 0 as *mut wl_resource;
    // At this point, if the wl_buffer comes from linux-dmabuf or wl_drm, we
	// still haven't released it (ie. we'll read it in the future) but the
	// client destroyed it. Reading the texture itself should be fine because
	// we still hold a reference to the DMA-BUF via the texture. However the
	// client could decide to re-use the same DMA-BUF for something else, in
	// which case we'll read garbage. We decide to accept this risk.
}
/* *
 * Upload a buffer to the GPU and reference it.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_buffer_create(mut renderer: *mut wlr_renderer,
                                           mut resource: *mut wl_resource)
 -> *mut wlr_buffer {
    if wlr_resource_is_buffer(resource) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_resource_is_buffer(resource)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_buffer.c\x00" as *const u8 as
                          *const libc::c_char, 56i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 82],
                                                &[libc::c_char; 82]>(b"struct wlr_buffer *wlr_buffer_create(struct wlr_renderer *, struct wl_resource *)\x00")).as_ptr());
    };
    let mut texture: *mut wlr_texture = 0 as *mut wlr_texture;
    let mut released: bool = 0i32 != 0;
    let mut shm_buf: *mut wl_shm_buffer = wl_shm_buffer_get(resource);
    if !shm_buf.is_null() {
        let mut fmt: wl_shm_format =
            wl_shm_buffer_get_format(shm_buf) as wl_shm_format;
        let mut stride: int32_t = wl_shm_buffer_get_stride(shm_buf);
        let mut width: int32_t = wl_shm_buffer_get_width(shm_buf);
        let mut height: int32_t = wl_shm_buffer_get_height(shm_buf);
        wl_shm_buffer_begin_access(shm_buf);
        let mut data: *mut libc::c_void = wl_shm_buffer_get_data(shm_buf);
        texture =
            wlr_texture_from_pixels(renderer, fmt, stride as uint32_t,
                                    width as uint32_t, height as uint32_t,
                                    data);
        wl_shm_buffer_end_access(shm_buf);
        // We have uploaded the data, we don't need to access the wl_buffer
		// anymore
        wl_buffer_send_release(resource);
        released = 1i32 != 0
    } else if wlr_renderer_resource_is_wl_drm_buffer(renderer, resource) {
        texture = wlr_texture_from_wl_drm(renderer, resource)
    } else if wlr_dmabuf_v1_resource_is_buffer(resource) {
        let mut dmabuf: *mut wlr_dmabuf_v1_buffer =
            wlr_dmabuf_v1_buffer_from_buffer_resource(resource);
        texture = wlr_texture_from_dmabuf(renderer, &mut (*dmabuf).attributes)
        // We have imported the DMA-BUF, but we need to prevent the client from
		// re-using the same DMA-BUF for the next frames, so we don't release
		// the buffer yet.
    } else {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot upload texture: unknown buffer type\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_buffer.c\x00" as *const u8 as
                     *const libc::c_char, 89i32);
        // Instead of just logging the error, also disconnect the client with a
		// fatal protocol error so that it's clear something went wrong.
        wl_resource_post_error(resource, 0i32 as uint32_t,
                               b"unknown buffer type\x00" as *const u8 as
                                   *const libc::c_char);
        return 0 as *mut wlr_buffer
    }
    if texture.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to upload texture\x00" as *const u8 as
                     *const libc::c_char,
                 b"../types/wlr_buffer.c\x00" as *const u8 as
                     *const libc::c_char, 98i32);
        return 0 as *mut wlr_buffer
    }
    let mut buffer: *mut wlr_buffer =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_buffer>() as libc::c_ulong) as
            *mut wlr_buffer;
    if buffer.is_null() {
        wlr_texture_destroy(texture);
        return 0 as *mut wlr_buffer
    }
    (*buffer).resource = resource;
    (*buffer).texture = texture;
    (*buffer).released = released;
    (*buffer).n_refs = 1i32 as size_t;
    wl_resource_add_destroy_listener(resource,
                                     &mut (*buffer).resource_destroy);
    (*buffer).resource_destroy.notify =
        Some(buffer_resource_handle_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    return buffer;
}
/* *
 * Reference the buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_buffer_ref(mut buffer: *mut wlr_buffer)
 -> *mut wlr_buffer {
    (*buffer).n_refs = (*buffer).n_refs.wrapping_add(1);
    return buffer;
}
/* *
 * Unreference the buffer. After this call, `buffer` may not be accessed
 * anymore.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_buffer_unref(mut buffer: *mut wlr_buffer) {
    if buffer.is_null() { return }
    if (*buffer).n_refs > 0i32 as libc::c_ulong {
    } else {
        __assert_fail(b"buffer->n_refs > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_buffer.c\x00" as *const u8 as
                          *const libc::c_char, 128i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 43],
                                                &[libc::c_char; 43]>(b"void wlr_buffer_unref(struct wlr_buffer *)\x00")).as_ptr());
    };
    (*buffer).n_refs = (*buffer).n_refs.wrapping_sub(1);
    if (*buffer).n_refs > 0i32 as libc::c_ulong { return }
    if !(*buffer).released && !(*buffer).resource.is_null() {
        wl_buffer_send_release((*buffer).resource);
    }
    wl_list_remove(&mut (*buffer).resource_destroy.link);
    wlr_texture_destroy((*buffer).texture);
    free(buffer as *mut libc::c_void);
}
/* *
 * Try to update the buffer's content. On success, returns the updated buffer
 * and destroys the provided `buffer`. On error, `buffer` is intact and NULL is
 * returned.
 *
 * Fails if there's more than one reference to the buffer or if the texture
 * isn't mutable.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_buffer_apply_damage(mut buffer: *mut wlr_buffer,
                                                 mut resource:
                                                     *mut wl_resource,
                                                 mut damage:
                                                     *mut pixman_region32_t)
 -> *mut wlr_buffer {
    if wlr_resource_is_buffer(resource) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_resource_is_buffer(resource)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_buffer.c\x00" as *const u8 as
                          *const libc::c_char, 145i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 107],
                                                &[libc::c_char; 107]>(b"struct wlr_buffer *wlr_buffer_apply_damage(struct wlr_buffer *, struct wl_resource *, pixman_region32_t *)\x00")).as_ptr());
    };
    if (*buffer).n_refs > 1i32 as libc::c_ulong {
        // Someone else still has a reference to the buffer
        return 0 as *mut wlr_buffer
    }
    let mut shm_buf: *mut wl_shm_buffer = wl_shm_buffer_get(resource);
    let mut old_shm_buf: *mut wl_shm_buffer =
        wl_shm_buffer_get((*buffer).resource);
    if shm_buf.is_null() || old_shm_buf.is_null() {
        // Uploading only damaged regions only works for wl_shm buffers and
		// mutable textures (created from wl_shm buffer)
        return 0 as *mut wlr_buffer
    }
    let mut new_fmt: wl_shm_format =
        wl_shm_buffer_get_format(shm_buf) as wl_shm_format;
    let mut old_fmt: wl_shm_format =
        wl_shm_buffer_get_format(old_shm_buf) as wl_shm_format;
    if new_fmt as libc::c_uint != old_fmt as libc::c_uint {
        // Uploading to textures can't change the format
        return 0 as *mut wlr_buffer
    }
    let mut stride: int32_t = wl_shm_buffer_get_stride(shm_buf);
    let mut width: int32_t = wl_shm_buffer_get_width(shm_buf);
    let mut height: int32_t = wl_shm_buffer_get_height(shm_buf);
    let mut texture_width: int32_t = 0;
    let mut texture_height: int32_t = 0;
    wlr_texture_get_size((*buffer).texture, &mut texture_width,
                         &mut texture_height);
    if width != texture_width || height != texture_height {
        return 0 as *mut wlr_buffer
    }
    wl_shm_buffer_begin_access(shm_buf);
    let mut data: *mut libc::c_void = wl_shm_buffer_get_data(shm_buf);
    let mut n: libc::c_int = 0;
    let mut rects: *mut pixman_box32_t =
        pixman_region32_rectangles(damage, &mut n);
    let mut i: libc::c_int = 0i32;
    while i < n {
        let mut r: *mut pixman_box32_t =
            &mut *rects.offset(i as isize) as *mut pixman_box32_t;
        if !wlr_texture_write_pixels((*buffer).texture, stride as uint32_t,
                                     ((*r).x2 - (*r).x1) as uint32_t,
                                     ((*r).y2 - (*r).y1) as uint32_t,
                                     (*r).x1 as uint32_t, (*r).y1 as uint32_t,
                                     (*r).x1 as uint32_t, (*r).y1 as uint32_t,
                                     data) {
            wl_shm_buffer_end_access(shm_buf);
            return 0 as *mut wlr_buffer
        }
        i += 1
    }
    wl_shm_buffer_end_access(shm_buf);
    // We have uploaded the data, we don't need to access the wl_buffer
	// anymore
    wl_buffer_send_release(resource);
    wl_list_remove(&mut (*buffer).resource_destroy.link);
    wl_resource_add_destroy_listener(resource,
                                     &mut (*buffer).resource_destroy);
    (*buffer).resource_destroy.notify =
        Some(buffer_resource_handle_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    (*buffer).resource = resource;
    (*buffer).released = 1i32 != 0;
    return buffer;
}
/* *
 * Reads the DMA-BUF attributes of the buffer. If this buffer isn't a DMA-BUF,
 * returns false.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_buffer_get_dmabuf(mut buffer: *mut wlr_buffer,
                                               mut attribs:
                                                   *mut wlr_dmabuf_attributes)
 -> bool {
    if (*buffer).resource.is_null() { return 0i32 != 0 }
    let mut buffer_resource: *mut wl_resource = (*buffer).resource;
    if !wlr_dmabuf_v1_resource_is_buffer(buffer_resource) { return 0i32 != 0 }
    let mut dmabuf_buffer: *mut wlr_dmabuf_v1_buffer =
        wlr_dmabuf_v1_buffer_from_buffer_resource(buffer_resource);
    memcpy(attribs as *mut libc::c_void,
           &mut (*dmabuf_buffer).attributes as *mut wlr_dmabuf_attributes as
               *const libc::c_void,
           ::std::mem::size_of::<wlr_dmabuf_attributes>() as libc::c_ulong);
    return 1i32 != 0;
}
