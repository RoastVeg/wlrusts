use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_backend;
    pub type wlr_output_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert_list(list: *mut wl_list, other: *mut wl_list);
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
    #[no_mangle]
    fn wl_resource_post_error(resource: *mut wl_resource, code: uint32_t,
                              msg: *const libc::c_char, _: ...);
    #[no_mangle]
    fn wl_resource_post_no_memory(resource: *mut wl_resource);
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
    fn wl_resource_get_id(resource: *mut wl_resource) -> uint32_t;
    #[no_mangle]
    fn wl_resource_get_link(resource: *mut wl_resource) -> *mut wl_list;
    #[no_mangle]
    fn wl_resource_from_link(resource: *mut wl_list) -> *mut wl_resource;
    #[no_mangle]
    fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;
    #[no_mangle]
    fn wl_resource_set_user_data(resource: *mut wl_resource,
                                 data: *mut libc::c_void);
    #[no_mangle]
    fn wl_resource_get_user_data(resource: *mut wl_resource)
     -> *mut libc::c_void;
    #[no_mangle]
    fn wl_resource_instance_of(resource: *mut wl_resource,
                               interface: *const wl_interface,
                               implementation: *const libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn wl_resource_add_destroy_listener(resource: *mut wl_resource,
                                        listener: *mut wl_listener);
    #[no_mangle]
    static wl_callback_interface: wl_interface;
    #[no_mangle]
    static wl_surface_interface: wl_interface;
    #[no_mangle]
    static wl_subsurface_interface: wl_interface;
    #[no_mangle]
    fn wlr_output_transform_invert(tr: wl_output_transform)
     -> wl_output_transform;
    #[no_mangle]
    fn wlr_buffer_apply_damage(buffer: *mut wlr_buffer,
                               resource: *mut wl_resource,
                               damage: *mut pixman_region32_t)
     -> *mut wlr_buffer;
    #[no_mangle]
    fn wlr_buffer_unref(buffer: *mut wlr_buffer);
    #[no_mangle]
    fn wlr_buffer_create(renderer: *mut wlr_renderer,
                         resource: *mut wl_resource) -> *mut wlr_buffer;
    #[no_mangle]
    fn wlr_buffer_get_resource_size(resource: *mut wl_resource,
                                    renderer: *mut wlr_renderer,
                                    width: *mut libc::c_int,
                                    height: *mut libc::c_int) -> bool;
    #[no_mangle]
    fn wlr_texture_is_opaque(texture: *mut wlr_texture) -> bool;
    /* creation/destruction */
    #[no_mangle]
    fn pixman_region32_init(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_init_rect(region: *mut pixman_region32_t,
                                 x: libc::c_int, y: libc::c_int,
                                 width: libc::c_uint, height: libc::c_uint);
    #[no_mangle]
    fn pixman_region32_fini(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_copy(dest: *mut pixman_region32_t,
                            source: *mut pixman_region32_t) -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_union(new_reg: *mut pixman_region32_t,
                             reg1: *mut pixman_region32_t,
                             reg2: *mut pixman_region32_t) -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_intersect_rect(dest: *mut pixman_region32_t,
                                      source: *mut pixman_region32_t,
                                      x: libc::c_int, y: libc::c_int,
                                      width: libc::c_uint,
                                      height: libc::c_uint) -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_union_rect(dest: *mut pixman_region32_t,
                                  source: *mut pixman_region32_t,
                                  x: libc::c_int, y: libc::c_int,
                                  width: libc::c_uint, height: libc::c_uint)
     -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_contains_point(region: *mut pixman_region32_t,
                                      x: libc::c_int, y: libc::c_int,
                                      box_0: *mut pixman_box32_t)
     -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_clear(region: *mut pixman_region32_t);
    #[no_mangle]
    fn wlr_surface_is_subsurface(surface: *mut wlr_surface) -> bool;
    /* *
 * Get a subsurface from a surface. Can return NULL if the subsurface has been
 * destroyed.
 */
    #[no_mangle]
    fn wlr_subsurface_from_wlr_surface(surface: *mut wlr_surface)
     -> *mut wlr_subsurface;
    #[no_mangle]
    fn wlr_region_from_resource(resource: *mut wl_resource)
     -> *mut pixman_region32_t;
    // Will log all messages less than or equal to `verbosity`
// If `callback` is NULL, wlr will use its default logger.
// The function can be called multiple times to update the verbosity or
// callback function.
    // Returns the log verbosity provided to wlr_log_init
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
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
    /* *
 * Scales a region, ie. multiplies all its coordinates by `scale`.
 *
 * The resulting coordinates are rounded up or down so that the new region is
 * at least as big as the original one.
 */
    #[no_mangle]
    fn wlr_region_scale(dst: *mut pixman_region32_t,
                        src: *mut pixman_region32_t, scale: libc::c_float);
    /* *
 * Applies a transform to a region inside a box of size `width` x `height`.
 */
    #[no_mangle]
    fn wlr_region_transform(dst: *mut pixman_region32_t,
                            src: *mut pixman_region32_t,
                            transform: wl_output_transform,
                            width: libc::c_int, height: libc::c_int);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type wl_surface_error = libc::c_uint;
pub const WL_SURFACE_ERROR_INVALID_TRANSFORM: wl_surface_error = 1;
pub const WL_SURFACE_ERROR_INVALID_SCALE: wl_surface_error = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_surface_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub attach: Option<unsafe extern "C" fn(_: *mut wl_client,
                                            _: *mut wl_resource,
                                            _: *mut wl_resource, _: int32_t,
                                            _: int32_t) -> ()>,
    pub damage: Option<unsafe extern "C" fn(_: *mut wl_client,
                                            _: *mut wl_resource, _: int32_t,
                                            _: int32_t, _: int32_t,
                                            _: int32_t) -> ()>,
    pub frame: Option<unsafe extern "C" fn(_: *mut wl_client,
                                           _: *mut wl_resource, _: uint32_t)
                          -> ()>,
    pub set_opaque_region: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                       _: *mut wl_resource,
                                                       _: *mut wl_resource)
                                      -> ()>,
    pub set_input_region: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                      _: *mut wl_resource,
                                                      _: *mut wl_resource)
                                     -> ()>,
    pub commit: Option<unsafe extern "C" fn(_: *mut wl_client,
                                            _: *mut wl_resource) -> ()>,
    pub set_buffer_transform: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                          _: *mut wl_resource,
                                                          _: int32_t) -> ()>,
    pub set_buffer_scale: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                      _: *mut wl_resource,
                                                      _: int32_t) -> ()>,
    pub damage_buffer: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                   _: *mut wl_resource,
                                                   _: int32_t, _: int32_t,
                                                   _: int32_t, _: int32_t)
                                  -> ()>,
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
pub type wl_subsurface_error = libc::c_uint;
pub const WL_SUBSURFACE_ERROR_BAD_SURFACE: wl_subsurface_error = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_subsurface_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub set_position: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                  _: *mut wl_resource,
                                                  _: int32_t, _: int32_t)
                                 -> ()>,
    pub place_above: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                 _: *mut wl_resource,
                                                 _: *mut wl_resource) -> ()>,
    pub place_below: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                 _: *mut wl_resource,
                                                 _: *mut wl_resource) -> ()>,
    pub set_sync: Option<unsafe extern "C" fn(_: *mut wl_client,
                                              _: *mut wl_resource) -> ()>,
    pub set_desync: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                _: *mut wl_resource) -> ()>,
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
pub type pixman_bool_t = libc::c_int;
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
pub struct wlr_renderer {
    pub impl_0: *const wlr_renderer_impl,
    pub events: C2RustUnnamed,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
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
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_0,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_1,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub commit: wl_signal,
    pub new_subsurface: wl_signal,
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_surface_role {
    pub name: *const libc::c_char,
    pub commit: Option<unsafe extern "C" fn(_: *mut wlr_surface) -> ()>,
    pub precommit: Option<unsafe extern "C" fn(_: *mut wlr_surface) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output {
    pub impl_0: *const crate::src::backend::drm::backend::wlr_output_impl,
    pub backend: *mut crate::src::backend::backend::wlr_backend,
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
    pub events: C2RustUnnamed_2,
    pub idle_frame: *mut wl_event_source,
    pub idle_done: *mut wl_event_source,
    pub attach_render_locks: libc::c_int,
    pub cursors: wl_list,
    pub hardware_cursor: *mut wlr_output_cursor,
    pub software_cursor_locks: libc::c_int,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub damage: pixman_region32_t,
    pub buffer_type: wlr_output_state_buffer_type,
    pub buffer: *mut wlr_buffer,
}
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_subsurface {
    pub resource: *mut wl_resource,
    pub surface: *mut wlr_surface,
    pub parent: *mut wlr_surface,
    pub current: wlr_subsurface_state,
    pub pending: wlr_subsurface_state,
    pub cached: wlr_surface_state,
    pub has_cache: bool,
    pub synchronized: bool,
    pub reordered: bool,
    pub mapped: bool,
    pub parent_link: wl_list,
    pub parent_pending_link: wl_list,
    pub surface_destroy: wl_listener,
    pub parent_destroy: wl_listener,
    pub events: C2RustUnnamed_3,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub destroy: wl_signal,
    pub map: wl_signal,
    pub unmap: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_subsurface_state {
    pub x: int32_t,
    pub y: int32_t,
}
pub type wlr_surface_state_field = libc::c_uint;
pub const WLR_SURFACE_STATE_FRAME_CALLBACK_LIST: wlr_surface_state_field =
    128;
pub const WLR_SURFACE_STATE_SCALE: wlr_surface_state_field = 64;
pub const WLR_SURFACE_STATE_TRANSFORM: wlr_surface_state_field = 32;
pub const WLR_SURFACE_STATE_INPUT_REGION: wlr_surface_state_field = 16;
pub const WLR_SURFACE_STATE_OPAQUE_REGION: wlr_surface_state_field = 8;
pub const WLR_SURFACE_STATE_BUFFER_DAMAGE: wlr_surface_state_field = 4;
pub const WLR_SURFACE_STATE_SURFACE_DAMAGE: wlr_surface_state_field = 2;
pub const WLR_SURFACE_STATE_BUFFER: wlr_surface_state_field = 1;
pub type wlr_surface_iterator_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wlr_surface, _: libc::c_int,
                                _: libc::c_int, _: *mut libc::c_void) -> ()>;
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct bound_acc {
    pub min_x: int32_t,
    pub min_y: int32_t,
    pub max_x: int32_t,
    pub max_y: int32_t,
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
unsafe extern "C" fn wl_callback_send_done(mut resource_: *mut wl_resource,
                                           mut callback_data: uint32_t) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, callback_data);
}
#[inline]
unsafe extern "C" fn wl_surface_send_leave(mut resource_: *mut wl_resource,
                                           mut output: *mut wl_resource) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, output);
}
#[inline]
unsafe extern "C" fn wl_surface_send_enter(mut resource_: *mut wl_resource,
                                           mut output: *mut wl_resource) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, output);
}
unsafe extern "C" fn min(mut fst: libc::c_int, mut snd: libc::c_int)
 -> libc::c_int {
    if fst < snd { return fst } else { return snd };
}
unsafe extern "C" fn max(mut fst: libc::c_int, mut snd: libc::c_int)
 -> libc::c_int {
    if fst > snd { return fst } else { return snd };
}
unsafe extern "C" fn surface_state_reset_buffer(mut state:
                                                    *mut wlr_surface_state) {
    if !(*state).buffer_resource.is_null() {
        wl_list_remove(&mut (*state).buffer_destroy.link);
        (*state).buffer_resource = 0 as *mut wl_resource
    };
}
unsafe extern "C" fn surface_handle_buffer_destroy(mut listener:
                                                       *mut wl_listener,
                                                   mut data:
                                                       *mut libc::c_void) {
    let mut state: *mut wlr_surface_state =
        (listener as *mut libc::c_char).offset(-160) as
            *mut wlr_surface_state;
    surface_state_reset_buffer(state);
}
unsafe extern "C" fn surface_state_set_buffer(mut state:
                                                  *mut wlr_surface_state,
                                              mut buffer_resource:
                                                  *mut wl_resource) {
    surface_state_reset_buffer(state);
    (*state).buffer_resource = buffer_resource;
    if !buffer_resource.is_null() {
        wl_resource_add_destroy_listener(buffer_resource,
                                         &mut (*state).buffer_destroy);
        (*state).buffer_destroy.notify =
            Some(surface_handle_buffer_destroy as
                     unsafe extern "C" fn(_: *mut wl_listener,
                                          _: *mut libc::c_void) -> ())
    };
}
unsafe extern "C" fn surface_destroy(mut client: *mut wl_client,
                                     mut resource: *mut wl_resource) {
    wl_resource_destroy(resource);
}
unsafe extern "C" fn surface_attach(mut client: *mut wl_client,
                                    mut resource: *mut wl_resource,
                                    mut buffer: *mut wl_resource,
                                    mut dx: int32_t, mut dy: int32_t) {
    let mut surface: *mut wlr_surface = wlr_surface_from_resource(resource);
    (*surface).pending.committed |=
        WLR_SURFACE_STATE_BUFFER as libc::c_int as libc::c_uint;
    (*surface).pending.dx = dx;
    (*surface).pending.dy = dy;
    surface_state_set_buffer(&mut (*surface).pending, buffer);
}
unsafe extern "C" fn surface_damage(mut client: *mut wl_client,
                                    mut resource: *mut wl_resource,
                                    mut x: int32_t, mut y: int32_t,
                                    mut width: int32_t, mut height: int32_t) {
    let mut surface: *mut wlr_surface = wlr_surface_from_resource(resource);
    if width < 0i32 || height < 0i32 { return }
    (*surface).pending.committed |=
        WLR_SURFACE_STATE_SURFACE_DAMAGE as libc::c_int as libc::c_uint;
    pixman_region32_union_rect(&mut (*surface).pending.surface_damage,
                               &mut (*surface).pending.surface_damage, x, y,
                               width as libc::c_uint, height as libc::c_uint);
}
unsafe extern "C" fn callback_handle_resource_destroy(mut resource:
                                                          *mut wl_resource) {
    wl_list_remove(wl_resource_get_link(resource));
}
unsafe extern "C" fn surface_frame(mut client: *mut wl_client,
                                   mut resource: *mut wl_resource,
                                   mut callback: uint32_t) {
    let mut surface: *mut wlr_surface = wlr_surface_from_resource(resource);
    let mut callback_resource: *mut wl_resource =
        wl_resource_create(client, &wl_callback_interface, 1i32, callback);
    if callback_resource.is_null() {
        wl_resource_post_no_memory(resource);
        return
    }
    wl_resource_set_implementation(callback_resource,
                                   0 as *const libc::c_void,
                                   0 as *mut libc::c_void,
                                   Some(callback_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_insert((*surface).pending.frame_callback_list.prev,
                   wl_resource_get_link(callback_resource));
    (*surface).pending.committed |=
        WLR_SURFACE_STATE_FRAME_CALLBACK_LIST as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn surface_set_opaque_region(mut client: *mut wl_client,
                                               mut resource: *mut wl_resource,
                                               mut region_resource:
                                                   *mut wl_resource) {
    let mut surface: *mut wlr_surface = wlr_surface_from_resource(resource);
    (*surface).pending.committed |=
        WLR_SURFACE_STATE_OPAQUE_REGION as libc::c_int as libc::c_uint;
    if !region_resource.is_null() {
        let mut region: *mut pixman_region32_t =
            wlr_region_from_resource(region_resource);
        pixman_region32_copy(&mut (*surface).pending.opaque, region);
    } else { pixman_region32_clear(&mut (*surface).pending.opaque); };
}
unsafe extern "C" fn surface_set_input_region(mut client: *mut wl_client,
                                              mut resource: *mut wl_resource,
                                              mut region_resource:
                                                  *mut wl_resource) {
    let mut surface: *mut wlr_surface = wlr_surface_from_resource(resource);
    (*surface).pending.committed |=
        WLR_SURFACE_STATE_INPUT_REGION as libc::c_int as libc::c_uint;
    if !region_resource.is_null() {
        let mut region: *mut pixman_region32_t =
            wlr_region_from_resource(region_resource);
        pixman_region32_copy(&mut (*surface).pending.input, region);
    } else {
        pixman_region32_fini(&mut (*surface).pending.input);
        pixman_region32_init_rect(&mut (*surface).pending.input,
                                  -2147483647i32 - 1i32,
                                  -2147483647i32 - 1i32, 4294967295u32,
                                  4294967295u32);
    };
}
unsafe extern "C" fn surface_state_finalize(mut surface: *mut wlr_surface,
                                            mut state:
                                                *mut wlr_surface_state) {
    if (*state).committed &
           WLR_SURFACE_STATE_BUFFER as libc::c_int as libc::c_uint != 0 {
        if !(*state).buffer_resource.is_null() {
            wlr_buffer_get_resource_size((*state).buffer_resource,
                                         (*surface).renderer,
                                         &mut (*state).buffer_width,
                                         &mut (*state).buffer_height);
        } else {
            (*state).buffer_height = 0i32;
            (*state).buffer_width = (*state).buffer_height
        }
    }
    let mut width: libc::c_int = (*state).buffer_width / (*state).scale;
    let mut height: libc::c_int = (*state).buffer_height / (*state).scale;
    if (*state).transform as libc::c_uint &
           WL_OUTPUT_TRANSFORM_90 as libc::c_int as libc::c_uint !=
           0i32 as libc::c_uint {
        let mut tmp: libc::c_int = width;
        width = height;
        height = tmp
    }
    (*state).width = width;
    (*state).height = height;
    pixman_region32_intersect_rect(&mut (*state).surface_damage,
                                   &mut (*state).surface_damage, 0i32, 0i32,
                                   (*state).width as libc::c_uint,
                                   (*state).height as libc::c_uint);
    pixman_region32_intersect_rect(&mut (*state).buffer_damage,
                                   &mut (*state).buffer_damage, 0i32, 0i32,
                                   (*state).buffer_width as libc::c_uint,
                                   (*state).buffer_height as libc::c_uint);
}
unsafe extern "C" fn surface_update_damage(mut buffer_damage:
                                               *mut pixman_region32_t,
                                           mut current:
                                               *mut wlr_surface_state,
                                           mut pending:
                                               *mut wlr_surface_state) {
    pixman_region32_clear(buffer_damage);
    if (*pending).width != (*current).width ||
           (*pending).height != (*current).height {
        // Damage the whole buffer on resize
        pixman_region32_union_rect(buffer_damage, buffer_damage, 0i32, 0i32,
                                   (*pending).buffer_width as libc::c_uint,
                                   (*pending).buffer_height as libc::c_uint);
    } else {
        // Copy over surface damage + buffer damage
        let mut surface_damage_0: pixman_region32_t =
            pixman_region32_t{extents:
                                  pixman_box32_t{x1: 0, y1: 0, x2: 0, y2: 0,},
                              data: 0 as *mut pixman_region32_data_t,};
        pixman_region32_init(&mut surface_damage_0);
        pixman_region32_copy(&mut surface_damage_0,
                             &mut (*pending).surface_damage);
        wlr_region_transform(&mut surface_damage_0, &mut surface_damage_0,
                             wlr_output_transform_invert((*pending).transform),
                             (*pending).width, (*pending).height);
        wlr_region_scale(&mut surface_damage_0, &mut surface_damage_0,
                         (*pending).scale as libc::c_float);
        pixman_region32_union(buffer_damage, &mut (*pending).buffer_damage,
                              &mut surface_damage_0);
        pixman_region32_fini(&mut surface_damage_0);
    };
}
unsafe extern "C" fn surface_state_copy(mut state: *mut wlr_surface_state,
                                        mut next: *mut wlr_surface_state) {
    (*state).width = (*next).width;
    (*state).height = (*next).height;
    (*state).buffer_width = (*next).buffer_width;
    (*state).buffer_height = (*next).buffer_height;
    if (*next).committed &
           WLR_SURFACE_STATE_SCALE as libc::c_int as libc::c_uint != 0 {
        (*state).scale = (*next).scale
    }
    if (*next).committed &
           WLR_SURFACE_STATE_TRANSFORM as libc::c_int as libc::c_uint != 0 {
        (*state).transform = (*next).transform
    }
    if (*next).committed &
           WLR_SURFACE_STATE_BUFFER as libc::c_int as libc::c_uint != 0 {
        (*state).dx = (*next).dx;
        (*state).dy = (*next).dy
    } else { (*state).dy = 0i32; (*state).dx = (*state).dy }
    if (*next).committed &
           WLR_SURFACE_STATE_SURFACE_DAMAGE as libc::c_int as libc::c_uint !=
           0 {
        pixman_region32_copy(&mut (*state).surface_damage,
                             &mut (*next).surface_damage);
    } else { pixman_region32_clear(&mut (*state).surface_damage); }
    if (*next).committed &
           WLR_SURFACE_STATE_BUFFER_DAMAGE as libc::c_int as libc::c_uint != 0
       {
        pixman_region32_copy(&mut (*state).buffer_damage,
                             &mut (*next).buffer_damage);
    } else { pixman_region32_clear(&mut (*state).buffer_damage); }
    if (*next).committed &
           WLR_SURFACE_STATE_OPAQUE_REGION as libc::c_int as libc::c_uint != 0
       {
        pixman_region32_copy(&mut (*state).opaque, &mut (*next).opaque);
    }
    if (*next).committed &
           WLR_SURFACE_STATE_INPUT_REGION as libc::c_int as libc::c_uint != 0
       {
        pixman_region32_copy(&mut (*state).input, &mut (*next).input);
    }
    (*state).committed |= (*next).committed;
}
/* *
 * Append pending state to current state and clear pending state.
 */
unsafe extern "C" fn surface_state_move(mut state: *mut wlr_surface_state,
                                        mut next: *mut wlr_surface_state) {
    surface_state_copy(state, next);
    if (*next).committed &
           WLR_SURFACE_STATE_BUFFER as libc::c_int as libc::c_uint != 0 {
        surface_state_set_buffer(state, (*next).buffer_resource);
        surface_state_reset_buffer(next);
        (*next).dy = 0i32;
        (*next).dx = (*next).dy
    }
    if (*next).committed &
           WLR_SURFACE_STATE_SURFACE_DAMAGE as libc::c_int as libc::c_uint !=
           0 {
        pixman_region32_clear(&mut (*next).surface_damage);
    }
    if (*next).committed &
           WLR_SURFACE_STATE_BUFFER_DAMAGE as libc::c_int as libc::c_uint != 0
       {
        pixman_region32_clear(&mut (*next).buffer_damage);
    }
    if (*next).committed &
           WLR_SURFACE_STATE_FRAME_CALLBACK_LIST as libc::c_int as
               libc::c_uint != 0 {
        wl_list_insert_list(&mut (*state).frame_callback_list,
                            &mut (*next).frame_callback_list);
        wl_list_init(&mut (*next).frame_callback_list);
    }
    (*next).committed = 0i32 as uint32_t;
}
unsafe extern "C" fn surface_damage_subsurfaces(mut subsurface:
                                                    *mut wlr_subsurface) {
    // XXX: This is probably the wrong way to do it, because this damage should
	// come from the client, but weston doesn't do it correctly either and it
	// seems to work ok. See the comment on weston_surface_damage for more info
	// about a better approach.
    let mut surface: *mut wlr_surface = (*subsurface).surface;
    pixman_region32_union_rect(&mut (*surface).buffer_damage,
                               &mut (*surface).buffer_damage, 0i32, 0i32,
                               (*surface).current.buffer_width as
                                   libc::c_uint,
                               (*surface).current.buffer_height as
                                   libc::c_uint);
    (*subsurface).reordered = 0i32 != 0;
    let mut child: *mut wlr_subsurface = 0 as *mut wlr_subsurface;
    child =
        ((*(*subsurface).surface).subsurfaces.next as
             *mut libc::c_char).offset(-232) as *mut wlr_subsurface;
    while &mut (*child).parent_link as *mut wl_list !=
              &mut (*(*subsurface).surface).subsurfaces as *mut wl_list {
        surface_damage_subsurfaces(child);
        child =
            ((*child).parent_link.next as *mut libc::c_char).offset(-232) as
                *mut wlr_subsurface
    };
}
unsafe extern "C" fn surface_apply_damage(mut surface: *mut wlr_surface) {
    let mut resource: *mut wl_resource = (*surface).current.buffer_resource;
    if resource.is_null() {
        // NULL commit
        wlr_buffer_unref((*surface).buffer);
        (*surface).buffer = 0 as *mut wlr_buffer;
        return
    }
    if !(*surface).buffer.is_null() &&
           (*(*surface).buffer).released as libc::c_int != 0 {
        let mut updated_buffer: *mut wlr_buffer =
            wlr_buffer_apply_damage((*surface).buffer, resource,
                                    &mut (*surface).buffer_damage);
        if !updated_buffer.is_null() {
            (*surface).buffer = updated_buffer;
            return
        }
    }
    wlr_buffer_unref((*surface).buffer);
    (*surface).buffer = 0 as *mut wlr_buffer;
    let mut buffer: *mut wlr_buffer =
        wlr_buffer_create((*surface).renderer, resource);
    if buffer.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to upload buffer\x00" as *const u8 as
                     *const libc::c_char,
                 b"../types/wlr_surface.c\x00" as *const u8 as
                     *const libc::c_char, 304i32);
        return
    }
    (*surface).buffer = buffer;
}
unsafe extern "C" fn surface_update_opaque_region(mut surface:
                                                      *mut wlr_surface) {
    let mut texture: *mut wlr_texture = wlr_surface_get_texture(surface);
    if texture.is_null() {
        pixman_region32_clear(&mut (*surface).opaque_region);
        return
    }
    if wlr_texture_is_opaque(texture) {
        pixman_region32_init_rect(&mut (*surface).opaque_region, 0i32, 0i32,
                                  (*surface).current.width as libc::c_uint,
                                  (*surface).current.height as libc::c_uint);
        return
    }
    pixman_region32_intersect_rect(&mut (*surface).opaque_region,
                                   &mut (*surface).current.opaque, 0i32, 0i32,
                                   (*surface).current.width as libc::c_uint,
                                   (*surface).current.height as libc::c_uint);
}
unsafe extern "C" fn surface_update_input_region(mut surface:
                                                     *mut wlr_surface) {
    pixman_region32_intersect_rect(&mut (*surface).input_region,
                                   &mut (*surface).current.input, 0i32, 0i32,
                                   (*surface).current.width as libc::c_uint,
                                   (*surface).current.height as libc::c_uint);
}
unsafe extern "C" fn surface_commit_pending(mut surface: *mut wlr_surface) {
    surface_state_finalize(surface, &mut (*surface).pending);
    if !(*surface).role.is_null() && (*(*surface).role).precommit.is_some() {
        (*(*surface).role).precommit.expect("non-null function pointer")(surface);
    }
    let mut invalid_buffer: bool =
        (*surface).pending.committed &
            WLR_SURFACE_STATE_BUFFER as libc::c_int as libc::c_uint != 0;
    (*surface).sx += (*surface).pending.dx;
    (*surface).sy += (*surface).pending.dy;
    surface_update_damage(&mut (*surface).buffer_damage,
                          &mut (*surface).current, &mut (*surface).pending);
    surface_state_copy(&mut (*surface).previous, &mut (*surface).current);
    surface_state_move(&mut (*surface).current, &mut (*surface).pending);
    if invalid_buffer { surface_apply_damage(surface); }
    surface_update_opaque_region(surface);
    surface_update_input_region(surface);
    // commit subsurface order
    let mut subsurface: *mut wlr_subsurface = 0 as *mut wlr_subsurface;
    subsurface =
        ((*surface).subsurface_pending_list.prev as
             *mut libc::c_char).offset(-248) as *mut wlr_subsurface;
    while &mut (*subsurface).parent_pending_link as *mut wl_list !=
              &mut (*surface).subsurface_pending_list as *mut wl_list {
        wl_list_remove(&mut (*subsurface).parent_link);
        wl_list_insert(&mut (*surface).subsurfaces,
                       &mut (*subsurface).parent_link);
        if (*subsurface).reordered {
            // TODO: damage all the subsurfaces
            surface_damage_subsurfaces(subsurface);
        }
        subsurface =
            ((*subsurface).parent_pending_link.prev as
                 *mut libc::c_char).offset(-248) as *mut wlr_subsurface
    }
    if !(*surface).role.is_null() && (*(*surface).role).commit.is_some() {
        (*(*surface).role).commit.expect("non-null function pointer")(surface);
    }
    wlr_signal_emit_safe(&mut (*surface).events.commit,
                         surface as *mut libc::c_void);
}
unsafe extern "C" fn subsurface_is_synchronized(mut subsurface:
                                                    *mut wlr_subsurface)
 -> bool {
    while !subsurface.is_null() {
        if (*subsurface).synchronized { return 1i32 != 0 }
        if (*subsurface).parent.is_null() { return 0i32 != 0 }
        if !wlr_surface_is_subsurface((*subsurface).parent) { break ; }
        subsurface = wlr_subsurface_from_wlr_surface((*subsurface).parent)
    }
    return 0i32 != 0;
}
/* *
 * Recursive function to commit the effectively synchronized children.
 */
unsafe extern "C" fn subsurface_parent_commit(mut subsurface:
                                                  *mut wlr_subsurface,
                                              mut synchronized: bool) {
    let mut surface: *mut wlr_surface = (*subsurface).surface;
    if synchronized as libc::c_int != 0 ||
           (*subsurface).synchronized as libc::c_int != 0 {
        if (*subsurface).has_cache {
            surface_state_move(&mut (*surface).pending,
                               &mut (*subsurface).cached);
            surface_commit_pending(surface);
            (*subsurface).has_cache = 0i32 != 0;
            (*subsurface).cached.committed = 0i32 as uint32_t
        }
        let mut subsurface_0: *mut wlr_subsurface = 0 as *mut wlr_subsurface;
        subsurface_0 =
            ((*surface).subsurfaces.next as *mut libc::c_char).offset(-232) as
                *mut wlr_subsurface;
        while &mut (*subsurface_0).parent_link as *mut wl_list !=
                  &mut (*surface).subsurfaces as *mut wl_list {
            subsurface_parent_commit(subsurface_0, 1i32 != 0);
            subsurface_0 =
                ((*subsurface_0).parent_link.next as
                     *mut libc::c_char).offset(-232) as *mut wlr_subsurface
        }
    };
}
unsafe extern "C" fn subsurface_commit(mut subsurface: *mut wlr_subsurface) {
    let mut surface: *mut wlr_surface = (*subsurface).surface;
    if subsurface_is_synchronized(subsurface) {
        surface_state_move(&mut (*subsurface).cached,
                           &mut (*surface).pending);
        (*subsurface).has_cache = 1i32 != 0
    } else if (*subsurface).has_cache {
        surface_state_move(&mut (*surface).pending,
                           &mut (*subsurface).cached);
        surface_commit_pending(surface);
        (*subsurface).has_cache = 0i32 != 0
    } else { surface_commit_pending(surface); };
}
unsafe extern "C" fn surface_commit(mut client: *mut wl_client,
                                    mut resource: *mut wl_resource) {
    let mut surface: *mut wlr_surface = wlr_surface_from_resource(resource);
    let mut subsurface: *mut wlr_subsurface =
        if wlr_surface_is_subsurface(surface) as libc::c_int != 0 {
            wlr_subsurface_from_wlr_surface(surface)
        } else { 0 as *mut wlr_subsurface };
    if !subsurface.is_null() {
        subsurface_commit(subsurface);
    } else { surface_commit_pending(surface); }
    subsurface =
        ((*surface).subsurfaces.next as *mut libc::c_char).offset(-232) as
            *mut wlr_subsurface;
    while &mut (*subsurface).parent_link as *mut wl_list !=
              &mut (*surface).subsurfaces as *mut wl_list {
        subsurface_parent_commit(subsurface, 0i32 != 0);
        subsurface =
            ((*subsurface).parent_link.next as *mut libc::c_char).offset(-232)
                as *mut wlr_subsurface
    };
}
unsafe extern "C" fn surface_set_buffer_transform(mut client: *mut wl_client,
                                                  mut resource:
                                                      *mut wl_resource,
                                                  mut transform: int32_t) {
    if transform < WL_OUTPUT_TRANSFORM_NORMAL as libc::c_int ||
           transform > WL_OUTPUT_TRANSFORM_FLIPPED_270 as libc::c_int {
        wl_resource_post_error(resource,
                               WL_SURFACE_ERROR_INVALID_TRANSFORM as
                                   libc::c_int as uint32_t,
                               b"Specified transform value (%d) is invalid\x00"
                                   as *const u8 as *const libc::c_char,
                               transform);
        return
    }
    let mut surface: *mut wlr_surface = wlr_surface_from_resource(resource);
    (*surface).pending.committed |=
        WLR_SURFACE_STATE_TRANSFORM as libc::c_int as libc::c_uint;
    (*surface).pending.transform = transform as wl_output_transform;
}
unsafe extern "C" fn surface_set_buffer_scale(mut client: *mut wl_client,
                                              mut resource: *mut wl_resource,
                                              mut scale: int32_t) {
    if scale <= 0i32 {
        wl_resource_post_error(resource,
                               WL_SURFACE_ERROR_INVALID_SCALE as libc::c_int
                                   as uint32_t,
                               b"Specified scale value (%d) is not positive\x00"
                                   as *const u8 as *const libc::c_char,
                               scale);
        return
    }
    let mut surface: *mut wlr_surface = wlr_surface_from_resource(resource);
    (*surface).pending.committed |=
        WLR_SURFACE_STATE_SCALE as libc::c_int as libc::c_uint;
    (*surface).pending.scale = scale;
}
unsafe extern "C" fn surface_damage_buffer(mut client: *mut wl_client,
                                           mut resource: *mut wl_resource,
                                           mut x: int32_t, mut y: int32_t,
                                           mut width: int32_t,
                                           mut height: int32_t) {
    let mut surface: *mut wlr_surface = wlr_surface_from_resource(resource);
    if width < 0i32 || height < 0i32 { return }
    (*surface).pending.committed |=
        WLR_SURFACE_STATE_BUFFER_DAMAGE as libc::c_int as libc::c_uint;
    pixman_region32_union_rect(&mut (*surface).pending.buffer_damage,
                               &mut (*surface).pending.buffer_damage, x, y,
                               width as libc::c_uint, height as libc::c_uint);
}
static mut surface_interface: wl_surface_interface =
    {
    
        {
            let mut init =
                wl_surface_interface{destroy:
                                         Some(surface_destroy as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource)
                                                      -> ()),
                                     attach:
                                         Some(surface_attach as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           int32_t,
                                                                       _:
                                                                           int32_t)
                                                      -> ()),
                                     damage:
                                         Some(surface_damage as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           int32_t,
                                                                       _:
                                                                           int32_t,
                                                                       _:
                                                                           int32_t,
                                                                       _:
                                                                           int32_t)
                                                      -> ()),
                                     frame:
                                         Some(surface_frame as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           uint32_t)
                                                      -> ()),
                                     set_opaque_region:
                                         Some(surface_set_opaque_region as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           *mut wl_resource)
                                                      -> ()),
                                     set_input_region:
                                         Some(surface_set_input_region as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           *mut wl_resource)
                                                      -> ()),
                                     commit:
                                         Some(surface_commit as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource)
                                                      -> ()),
                                     set_buffer_transform:
                                         Some(surface_set_buffer_transform as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           int32_t)
                                                      -> ()),
                                     set_buffer_scale:
                                         Some(surface_set_buffer_scale as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           int32_t)
                                                      -> ()),
                                     damage_buffer:
                                         Some(surface_damage_buffer as
                                                  unsafe extern "C" fn(_:
                                                                           *mut wl_client,
                                                                       _:
                                                                           *mut wl_resource,
                                                                       _:
                                                                           int32_t,
                                                                       _:
                                                                           int32_t,
                                                                       _:
                                                                           int32_t,
                                                                       _:
                                                                           int32_t)
                                                      -> ()),};
            init
        }
};
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_from_resource(mut resource:
                                                       *mut wl_resource)
 -> *mut wlr_surface {
    if wl_resource_instance_of(resource, &wl_surface_interface,
                               &surface_interface as
                                   *const wl_surface_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_surface_interface, &surface_interface)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_surface.c\x00" as *const u8 as
                          *const libc::c_char, 506i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"struct wlr_surface *wlr_surface_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_surface;
}
unsafe extern "C" fn surface_state_init(mut state: *mut wlr_surface_state) {
    (*state).scale = 1i32;
    (*state).transform = WL_OUTPUT_TRANSFORM_NORMAL;
    wl_list_init(&mut (*state).frame_callback_list);
    pixman_region32_init(&mut (*state).surface_damage);
    pixman_region32_init(&mut (*state).buffer_damage);
    pixman_region32_init(&mut (*state).opaque);
    pixman_region32_init_rect(&mut (*state).input, -2147483647i32 - 1i32,
                              -2147483647i32 - 1i32, 4294967295u32,
                              4294967295u32);
}
unsafe extern "C" fn surface_state_finish(mut state: *mut wlr_surface_state) {
    surface_state_reset_buffer(state);
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    let mut tmp: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    tmp = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*state).frame_callback_list.next);
    tmp = wl_resource_from_link((*(*state).frame_callback_list.next).next);
    while wl_resource_get_link(resource) !=
              &mut (*state).frame_callback_list as *mut wl_list {
        wl_resource_destroy(resource);
        resource = tmp;
        tmp = wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    pixman_region32_fini(&mut (*state).surface_damage);
    pixman_region32_fini(&mut (*state).buffer_damage);
    pixman_region32_fini(&mut (*state).opaque);
    pixman_region32_fini(&mut (*state).input);
}
unsafe extern "C" fn subsurface_destroy(mut subsurface: *mut wlr_subsurface) {
    if subsurface.is_null() { return }
    subsurface_unmap(subsurface);
    wlr_signal_emit_safe(&mut (*subsurface).events.destroy,
                         subsurface as *mut libc::c_void);
    wl_list_remove(&mut (*subsurface).surface_destroy.link);
    surface_state_finish(&mut (*subsurface).cached);
    if !(*subsurface).parent.is_null() {
        wl_list_remove(&mut (*subsurface).parent_link);
        wl_list_remove(&mut (*subsurface).parent_pending_link);
        wl_list_remove(&mut (*subsurface).parent_destroy.link);
    }
    wl_resource_set_user_data((*subsurface).resource, 0 as *mut libc::c_void);
    if !(*subsurface).surface.is_null() {
        (*(*subsurface).surface).role_data = 0 as *mut libc::c_void
    }
    free(subsurface as *mut libc::c_void);
}
unsafe extern "C" fn surface_handle_resource_destroy(mut resource:
                                                         *mut wl_resource) {
    let mut surface: *mut wlr_surface = wlr_surface_from_resource(resource);
    wlr_signal_emit_safe(&mut (*surface).events.destroy,
                         surface as *mut libc::c_void);
    wl_list_remove(wl_resource_get_link((*surface).resource));
    wl_list_remove(&mut (*surface).renderer_destroy.link);
    surface_state_finish(&mut (*surface).pending);
    surface_state_finish(&mut (*surface).current);
    surface_state_finish(&mut (*surface).previous);
    pixman_region32_fini(&mut (*surface).buffer_damage);
    pixman_region32_fini(&mut (*surface).opaque_region);
    pixman_region32_fini(&mut (*surface).input_region);
    wlr_buffer_unref((*surface).buffer);
    free(surface as *mut libc::c_void);
}
unsafe extern "C" fn surface_handle_renderer_destroy(mut listener:
                                                         *mut wl_listener,
                                                     mut data:
                                                         *mut libc::c_void) {
    let mut surface: *mut wlr_surface =
        (listener as *mut libc::c_char).offset(-752) as *mut wlr_surface;
    wl_resource_destroy((*surface).resource);
}
/* *
 * Create a new surface resource with the provided new ID. If `resource_list`
 * is non-NULL, adds the surface's resource to the list.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_create(mut client: *mut wl_client,
                                            mut version: uint32_t,
                                            mut id: uint32_t,
                                            mut renderer: *mut wlr_renderer,
                                            mut resource_list: *mut wl_list)
 -> *mut wlr_surface {
    if version <= 4i32 as libc::c_uint {
    } else {
        __assert_fail(b"version <= SURFACE_VERSION\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_surface.c\x00" as *const u8 as
                          *const libc::c_char, 592i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 120],
                                                &[libc::c_char; 120]>(b"struct wlr_surface *wlr_surface_create(struct wl_client *, uint32_t, uint32_t, struct wlr_renderer *, struct wl_list *)\x00")).as_ptr());
    };
    let mut surface: *mut wlr_surface =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_surface>() as libc::c_ulong) as
            *mut wlr_surface;
    if surface.is_null() {
        wl_client_post_no_memory(client);
        return 0 as *mut wlr_surface
    }
    (*surface).resource =
        wl_resource_create(client, &wl_surface_interface,
                           version as libc::c_int, id);
    if (*surface).resource.is_null() {
        free(surface as *mut libc::c_void);
        wl_client_post_no_memory(client);
        return 0 as *mut wlr_surface
    }
    wl_resource_set_implementation((*surface).resource,
                                   &surface_interface as
                                       *const wl_surface_interface as
                                       *const libc::c_void,
                                   surface as *mut libc::c_void,
                                   Some(surface_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] New wlr_surface %p (res %p)\x00" as *const u8 as
                 *const libc::c_char,
             b"../types/wlr_surface.c\x00" as *const u8 as
                 *const libc::c_char, 609i32, surface, (*surface).resource);
    (*surface).renderer = renderer;
    surface_state_init(&mut (*surface).current);
    surface_state_init(&mut (*surface).pending);
    surface_state_init(&mut (*surface).previous);
    wl_signal_init(&mut (*surface).events.commit);
    wl_signal_init(&mut (*surface).events.destroy);
    wl_signal_init(&mut (*surface).events.new_subsurface);
    wl_list_init(&mut (*surface).subsurfaces);
    wl_list_init(&mut (*surface).subsurface_pending_list);
    pixman_region32_init(&mut (*surface).buffer_damage);
    pixman_region32_init(&mut (*surface).opaque_region);
    pixman_region32_init(&mut (*surface).input_region);
    wl_signal_add(&mut (*renderer).events.destroy,
                  &mut (*surface).renderer_destroy);
    (*surface).renderer_destroy.notify =
        Some(surface_handle_renderer_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    let mut resource_link: *mut wl_list =
        wl_resource_get_link((*surface).resource);
    if !resource_list.is_null() {
        wl_list_insert(resource_list, resource_link);
    } else { wl_list_init(resource_link); }
    return surface;
}
/* *
 * Get the texture of the buffer currently attached to this surface. Returns
 * NULL if no buffer is currently attached or if something went wrong with
 * uploading the buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_get_texture(mut surface:
                                                     *mut wlr_surface)
 -> *mut wlr_texture {
    if (*surface).buffer.is_null() { return 0 as *mut wlr_texture }
    return (*(*surface).buffer).texture;
}
/* *
 * Whether or not this surface currently has an attached buffer. A surface has
 * an attached buffer when it commits with a non-null buffer in its pending
 * state. A surface will not have a buffer if it has never committed one, has
 * committed a null buffer, or something went wrong with uploading the buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_has_buffer(mut surface: *mut wlr_surface)
 -> bool {
    return !wlr_surface_get_texture(surface).is_null();
}
/* *
 * Set the lifetime role for this surface. Returns 0 on success or -1 if the
 * role cannot be set.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_set_role(mut surface: *mut wlr_surface,
                                              mut role:
                                                  *const wlr_surface_role,
                                              mut role_data:
                                                  *mut libc::c_void,
                                              mut error_resource:
                                                  *mut wl_resource,
                                              mut error_code: uint32_t)
 -> bool {
    if !role.is_null() {
    } else {
        __assert_fail(b"role != NULL\x00" as *const u8 as *const libc::c_char,
                      b"../types/wlr_surface.c\x00" as *const u8 as
                          *const libc::c_char, 653i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 122],
                                                &[libc::c_char; 122]>(b"_Bool wlr_surface_set_role(struct wlr_surface *, const struct wlr_surface_role *, void *, struct wl_resource *, uint32_t)\x00")).as_ptr());
    };
    if !(*surface).role.is_null() && (*surface).role != role {
        if !error_resource.is_null() {
            wl_resource_post_error(error_resource, error_code,
                                   b"Cannot assign role %s to wl_surface@%d, already has role %s\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   (*role).name,
                                   wl_resource_get_id((*surface).resource),
                                   (*(*surface).role).name);
        }
        return 0i32 != 0
    }
    if (*surface).role_data.is_null() {
    } else {
        __assert_fail(b"surface->role_data == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_surface.c\x00" as *const u8 as
                          *const libc::c_char, 665i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 122],
                                                &[libc::c_char; 122]>(b"_Bool wlr_surface_set_role(struct wlr_surface *, const struct wlr_surface_role *, void *, struct wl_resource *, uint32_t)\x00")).as_ptr());
    };
    (*surface).role = role;
    (*surface).role_data = role_data;
    return 1i32 != 0;
}
unsafe extern "C" fn subsurface_from_resource(mut resource: *mut wl_resource)
 -> *mut wlr_subsurface {
    if wl_resource_instance_of(resource, &wl_subsurface_interface,
                               &subsurface_implementation as
                                   *const wl_subsurface_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_subsurface_interface, &subsurface_implementation)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_surface.c\x00" as *const u8 as
                          *const libc::c_char, 676i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"struct wlr_subsurface *subsurface_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_subsurface;
}
unsafe extern "C" fn subsurface_resource_destroy(mut resource:
                                                     *mut wl_resource) {
    let mut subsurface: *mut wlr_subsurface =
        subsurface_from_resource(resource);
    wl_list_remove(wl_resource_get_link(resource));
    subsurface_destroy(subsurface);
}
unsafe extern "C" fn subsurface_handle_destroy(mut client: *mut wl_client,
                                               mut resource:
                                                   *mut wl_resource) {
    wl_resource_destroy(resource);
}
unsafe extern "C" fn subsurface_handle_set_position(mut client:
                                                        *mut wl_client,
                                                    mut resource:
                                                        *mut wl_resource,
                                                    mut x: int32_t,
                                                    mut y: int32_t) {
    let mut subsurface: *mut wlr_subsurface =
        subsurface_from_resource(resource);
    if subsurface.is_null() { return }
    (*subsurface).pending.x = x;
    (*subsurface).pending.y = y;
}
unsafe extern "C" fn subsurface_find_sibling(mut subsurface:
                                                 *mut wlr_subsurface,
                                             mut surface: *mut wlr_surface)
 -> *mut wlr_subsurface {
    let mut parent: *mut wlr_surface = (*subsurface).parent;
    let mut sibling: *mut wlr_subsurface = 0 as *mut wlr_subsurface;
    sibling =
        ((*parent).subsurfaces.next as *mut libc::c_char).offset(-232) as
            *mut wlr_subsurface;
    while &mut (*sibling).parent_link as *mut wl_list !=
              &mut (*parent).subsurfaces as *mut wl_list {
        if (*sibling).surface == surface && sibling != subsurface {
            return sibling
        }
        sibling =
            ((*sibling).parent_link.next as *mut libc::c_char).offset(-232) as
                *mut wlr_subsurface
    }
    return 0 as *mut wlr_subsurface;
}
unsafe extern "C" fn subsurface_handle_place_above(mut client: *mut wl_client,
                                                   mut resource:
                                                       *mut wl_resource,
                                                   mut sibling_resource:
                                                       *mut wl_resource) {
    let mut subsurface: *mut wlr_subsurface =
        subsurface_from_resource(resource);
    if subsurface.is_null() { return }
    let mut sibling_surface: *mut wlr_surface =
        wlr_surface_from_resource(sibling_resource);
    let mut sibling: *mut wlr_subsurface =
        subsurface_find_sibling(subsurface, sibling_surface);
    if sibling.is_null() {
        wl_resource_post_error((*subsurface).resource,
                               WL_SUBSURFACE_ERROR_BAD_SURFACE as libc::c_int
                                   as uint32_t,
                               b"%s: wl_surface@%d is not a parent or sibling\x00"
                                   as *const u8 as *const libc::c_char,
                               b"place_above\x00" as *const u8 as
                                   *const libc::c_char,
                               wl_resource_get_id((*sibling_surface).resource));
        return
    }
    wl_list_remove(&mut (*subsurface).parent_pending_link);
    wl_list_insert(&mut (*sibling).parent_pending_link,
                   &mut (*subsurface).parent_pending_link);
    (*subsurface).reordered = 1i32 != 0;
}
unsafe extern "C" fn subsurface_handle_place_below(mut client: *mut wl_client,
                                                   mut resource:
                                                       *mut wl_resource,
                                                   mut sibling_resource:
                                                       *mut wl_resource) {
    let mut subsurface: *mut wlr_subsurface =
        subsurface_from_resource(resource);
    if subsurface.is_null() { return }
    let mut sibling_surface: *mut wlr_surface =
        wlr_surface_from_resource(sibling_resource);
    let mut sibling: *mut wlr_subsurface =
        subsurface_find_sibling(subsurface, sibling_surface);
    if sibling.is_null() {
        wl_resource_post_error((*subsurface).resource,
                               WL_SUBSURFACE_ERROR_BAD_SURFACE as libc::c_int
                                   as uint32_t,
                               b"%s: wl_surface@%d is not a parent or sibling\x00"
                                   as *const u8 as *const libc::c_char,
                               b"place_below\x00" as *const u8 as
                                   *const libc::c_char,
                               wl_resource_get_id((*sibling_surface).resource));
        return
    }
    wl_list_remove(&mut (*subsurface).parent_pending_link);
    wl_list_insert((*sibling).parent_pending_link.prev,
                   &mut (*subsurface).parent_pending_link);
    (*subsurface).reordered = 1i32 != 0;
}
unsafe extern "C" fn subsurface_handle_set_sync(mut client: *mut wl_client,
                                                mut resource:
                                                    *mut wl_resource) {
    let mut subsurface: *mut wlr_subsurface =
        subsurface_from_resource(resource);
    if subsurface.is_null() { return }
    (*subsurface).synchronized = 1i32 != 0;
}
unsafe extern "C" fn subsurface_handle_set_desync(mut client: *mut wl_client,
                                                  mut resource:
                                                      *mut wl_resource) {
    let mut subsurface: *mut wlr_subsurface =
        subsurface_from_resource(resource);
    if subsurface.is_null() { return }
    if (*subsurface).synchronized {
        (*subsurface).synchronized = 0i32 != 0;
        if !subsurface_is_synchronized(subsurface) {
            // TODO: do a synchronized commit to flush the cache
            subsurface_parent_commit(subsurface, 1i32 != 0);
        }
    };
}
static mut subsurface_implementation: wl_subsurface_interface =
    {
    
        {
            let mut init =
                wl_subsurface_interface{destroy:
                                            Some(subsurface_handle_destroy as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource)
                                                         -> ()),
                                        set_position:
                                            Some(subsurface_handle_set_position
                                                     as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource,
                                                                          _:
                                                                              int32_t,
                                                                          _:
                                                                              int32_t)
                                                         -> ()),
                                        place_above:
                                            Some(subsurface_handle_place_above
                                                     as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource,
                                                                          _:
                                                                              *mut wl_resource)
                                                         -> ()),
                                        place_below:
                                            Some(subsurface_handle_place_below
                                                     as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource,
                                                                          _:
                                                                              *mut wl_resource)
                                                         -> ()),
                                        set_sync:
                                            Some(subsurface_handle_set_sync as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource)
                                                         -> ()),
                                        set_desync:
                                            Some(subsurface_handle_set_desync
                                                     as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource)
                                                         -> ()),};
            init
        }
};
/* *
 * Checks if this subsurface needs to be marked as mapped. This can happen if:
 * - The subsurface has a buffer
 * - Its parent is mapped
 */
unsafe extern "C" fn subsurface_consider_map(mut subsurface:
                                                 *mut wlr_subsurface,
                                             mut check_parent: bool) {
    if (*subsurface).mapped as libc::c_int != 0 ||
           !wlr_surface_has_buffer((*subsurface).surface) {
        return
    }
    if check_parent {
        if (*subsurface).parent.is_null() { return }
        if wlr_surface_is_subsurface((*subsurface).parent) {
            let mut parent: *mut wlr_subsurface =
                wlr_subsurface_from_wlr_surface((*subsurface).parent);
            if parent.is_null() || !(*parent).mapped { return }
        }
    }
    // Now we can map the subsurface
    wlr_signal_emit_safe(&mut (*subsurface).events.map,
                         subsurface as *mut libc::c_void);
    (*subsurface).mapped = 1i32 != 0;
    // Try mapping all children too
    let mut child: *mut wlr_subsurface = 0 as *mut wlr_subsurface;
    child =
        ((*(*subsurface).surface).subsurfaces.next as
             *mut libc::c_char).offset(-232) as *mut wlr_subsurface;
    while &mut (*child).parent_link as *mut wl_list !=
              &mut (*(*subsurface).surface).subsurfaces as *mut wl_list {
        subsurface_consider_map(child, 0i32 != 0);
        child =
            ((*child).parent_link.next as *mut libc::c_char).offset(-232) as
                *mut wlr_subsurface
    };
}
unsafe extern "C" fn subsurface_unmap(mut subsurface: *mut wlr_subsurface) {
    if !(*subsurface).mapped { return }
    wlr_signal_emit_safe(&mut (*subsurface).events.unmap,
                         subsurface as *mut libc::c_void);
    (*subsurface).mapped = 0i32 != 0;
    // Unmap all children
    let mut child: *mut wlr_subsurface = 0 as *mut wlr_subsurface;
    child =
        ((*(*subsurface).surface).subsurfaces.next as
             *mut libc::c_char).offset(-232) as *mut wlr_subsurface;
    while &mut (*child).parent_link as *mut wl_list !=
              &mut (*(*subsurface).surface).subsurfaces as *mut wl_list {
        subsurface_unmap(child);
        child =
            ((*child).parent_link.next as *mut libc::c_char).offset(-232) as
                *mut wlr_subsurface
    };
}
unsafe extern "C" fn subsurface_role_commit(mut surface: *mut wlr_surface) {
    let mut subsurface: *mut wlr_subsurface =
        wlr_subsurface_from_wlr_surface(surface);
    if subsurface.is_null() { return }
    if (*subsurface).current.x != (*subsurface).pending.x ||
           (*subsurface).current.y != (*subsurface).pending.y {
        // Subsurface has moved
        let mut dx: libc::c_int =
            (*subsurface).current.x - (*subsurface).pending.x;
        let mut dy: libc::c_int =
            (*subsurface).current.y - (*subsurface).pending.y;
        (*subsurface).current.x = (*subsurface).pending.x;
        (*subsurface).current.y = (*subsurface).pending.y;
        if (*surface).current.transform as libc::c_uint &
               WL_OUTPUT_TRANSFORM_90 as libc::c_int as libc::c_uint !=
               0i32 as libc::c_uint {
            let mut tmp: libc::c_int = dx;
            dx = dy;
            dy = tmp
        }
        pixman_region32_union_rect(&mut (*surface).buffer_damage,
                                   &mut (*surface).buffer_damage,
                                   dx * (*surface).previous.scale,
                                   dy * (*surface).previous.scale,
                                   (*surface).previous.buffer_width as
                                       libc::c_uint,
                                   (*surface).previous.buffer_height as
                                       libc::c_uint);
        pixman_region32_union_rect(&mut (*surface).buffer_damage,
                                   &mut (*surface).buffer_damage, 0i32, 0i32,
                                   (*surface).current.buffer_width as
                                       libc::c_uint,
                                   (*surface).current.buffer_height as
                                       libc::c_uint);
    }
    subsurface_consider_map(subsurface, 1i32 != 0);
}
unsafe extern "C" fn subsurface_role_precommit(mut surface:
                                                   *mut wlr_surface) {
    let mut subsurface: *mut wlr_subsurface =
        wlr_subsurface_from_wlr_surface(surface);
    if subsurface.is_null() { return }
    if (*surface).pending.committed &
           WLR_SURFACE_STATE_BUFFER as libc::c_int as libc::c_uint != 0 &&
           (*surface).pending.buffer_resource.is_null() {
        // This is a NULL commit
        subsurface_unmap(subsurface);
    };
}
#[no_mangle]
pub static mut subsurface_role: wlr_surface_role =
    {
    
        {
            let mut init =
                wlr_surface_role{name:
                                     b"wl_subsurface\x00" as *const u8 as
                                         *const libc::c_char,
                                 commit:
                                     Some(subsurface_role_commit as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_surface)
                                                  -> ()),
                                 precommit:
                                     Some(subsurface_role_precommit as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_surface)
                                                  -> ()),};
            init
        }
};
unsafe extern "C" fn subsurface_handle_parent_destroy(mut listener:
                                                          *mut wl_listener,
                                                      mut data:
                                                          *mut libc::c_void) {
    let mut subsurface: *mut wlr_subsurface =
        (listener as *mut libc::c_char).offset(-288) as *mut wlr_subsurface;
    subsurface_unmap(subsurface);
    wl_list_remove(&mut (*subsurface).parent_link);
    wl_list_remove(&mut (*subsurface).parent_pending_link);
    wl_list_remove(&mut (*subsurface).parent_destroy.link);
    (*subsurface).parent = 0 as *mut wlr_surface;
}
unsafe extern "C" fn subsurface_handle_surface_destroy(mut listener:
                                                           *mut wl_listener,
                                                       mut data:
                                                           *mut libc::c_void) {
    let mut subsurface: *mut wlr_subsurface =
        (listener as *mut libc::c_char).offset(-264) as *mut wlr_subsurface;
    subsurface_destroy(subsurface);
}
/* *
 * Create a new subsurface resource with the provided new ID. If `resource_list`
 * is non-NULL, adds the subsurface's resource to the list.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_subsurface_create(mut surface: *mut wlr_surface,
                                               mut parent: *mut wlr_surface,
                                               mut version: uint32_t,
                                               mut id: uint32_t,
                                               mut resource_list:
                                                   *mut wl_list)
 -> *mut wlr_subsurface {
    if version <= 1i32 as libc::c_uint {
    } else {
        __assert_fail(b"version <= SUBSURFACE_VERSION\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_surface.c\x00" as *const u8 as
                          *const libc::c_char, 931i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 127],
                                                &[libc::c_char; 127]>(b"struct wlr_subsurface *wlr_subsurface_create(struct wlr_surface *, struct wlr_surface *, uint32_t, uint32_t, struct wl_list *)\x00")).as_ptr());
    };
    let mut client: *mut wl_client =
        wl_resource_get_client((*surface).resource);
    let mut subsurface: *mut wlr_subsurface =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_subsurface>() as libc::c_ulong) as
            *mut wlr_subsurface;
    if subsurface.is_null() {
        wl_client_post_no_memory(client);
        return 0 as *mut wlr_subsurface
    }
    surface_state_init(&mut (*subsurface).cached);
    (*subsurface).synchronized = 1i32 != 0;
    (*subsurface).surface = surface;
    (*subsurface).resource =
        wl_resource_create(client, &wl_subsurface_interface,
                           version as libc::c_int, id);
    if (*subsurface).resource.is_null() {
        surface_state_finish(&mut (*subsurface).cached);
        free(subsurface as *mut libc::c_void);
        wl_client_post_no_memory(client);
        return 0 as *mut wlr_subsurface
    }
    wl_resource_set_implementation((*subsurface).resource,
                                   &subsurface_implementation as
                                       *const wl_subsurface_interface as
                                       *const libc::c_void,
                                   subsurface as *mut libc::c_void,
                                   Some(subsurface_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_signal_init(&mut (*subsurface).events.destroy);
    wl_signal_init(&mut (*subsurface).events.map);
    wl_signal_init(&mut (*subsurface).events.unmap);
    wl_signal_add(&mut (*surface).events.destroy,
                  &mut (*subsurface).surface_destroy);
    (*subsurface).surface_destroy.notify =
        Some(subsurface_handle_surface_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    // link parent
    (*subsurface).parent = parent;
    wl_signal_add(&mut (*parent).events.destroy,
                  &mut (*subsurface).parent_destroy);
    (*subsurface).parent_destroy.notify =
        Some(subsurface_handle_parent_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_list_insert((*parent).subsurfaces.prev,
                   &mut (*subsurface).parent_link);
    wl_list_insert((*parent).subsurface_pending_list.prev,
                   &mut (*subsurface).parent_pending_link);
    (*surface).role_data = subsurface as *mut libc::c_void;
    let mut resource_link: *mut wl_list =
        wl_resource_get_link((*subsurface).resource);
    if !resource_list.is_null() {
        wl_list_insert(resource_list, resource_link);
    } else { wl_list_init(resource_link); }
    wlr_signal_emit_safe(&mut (*parent).events.new_subsurface,
                         subsurface as *mut libc::c_void);
    return subsurface;
}
/* *
 * Get the root of the subsurface tree for this surface.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_get_root_surface(mut surface:
                                                          *mut wlr_surface)
 -> *mut wlr_surface {
    while wlr_surface_is_subsurface(surface) {
        let mut subsurface: *mut wlr_subsurface =
            wlr_subsurface_from_wlr_surface(surface);
        if subsurface.is_null() { break ; }
        surface = (*subsurface).parent
    }
    return surface;
}
/* *
 * Check if the surface accepts input events at the given surface-local
 * coordinates. Does not check the surface's subsurfaces.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_point_accepts_input(mut surface:
                                                             *mut wlr_surface,
                                                         mut sx:
                                                             libc::c_double,
                                                         mut sy:
                                                             libc::c_double)
 -> bool {
    return sx >= 0i32 as libc::c_double &&
               sx < (*surface).current.width as libc::c_double &&
               sy >= 0i32 as libc::c_double &&
               sy < (*surface).current.height as libc::c_double &&
               pixman_region32_contains_point(&mut (*surface).current.input,
                                              floor(sx) as libc::c_int,
                                              floor(sy) as libc::c_int,
                                              0 as *mut pixman_box32_t) != 0;
}
/* *
 * Find a surface in this surface's tree that accepts input events at the given
 * surface-local coordinates. Returns the surface and coordinates in the leaf
 * surface coordinate system or NULL if no surface is found at that location.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_surface_at(mut surface: *mut wlr_surface,
                                                mut sx: libc::c_double,
                                                mut sy: libc::c_double,
                                                mut sub_x:
                                                    *mut libc::c_double,
                                                mut sub_y:
                                                    *mut libc::c_double)
 -> *mut wlr_surface {
    let mut subsurface: *mut wlr_subsurface = 0 as *mut wlr_subsurface;
    subsurface =
        ((*surface).subsurfaces.prev as *mut libc::c_char).offset(-232) as
            *mut wlr_subsurface;
    while &mut (*subsurface).parent_link as *mut wl_list !=
              &mut (*surface).subsurfaces as *mut wl_list {
        let mut _sub_x: libc::c_double =
            (*subsurface).current.x as libc::c_double;
        let mut _sub_y: libc::c_double =
            (*subsurface).current.y as libc::c_double;
        let mut sub: *mut wlr_surface =
            wlr_surface_surface_at((*subsurface).surface, sx - _sub_x,
                                   sy - _sub_y, sub_x, sub_y);
        if !sub.is_null() { return sub }
        subsurface =
            ((*subsurface).parent_link.prev as *mut libc::c_char).offset(-232)
                as *mut wlr_subsurface
    }
    if wlr_surface_point_accepts_input(surface, sx, sy) {
        if !sub_x.is_null() { *sub_x = sx }
        if !sub_y.is_null() { *sub_y = sy }
        return surface
    }
    return 0 as *mut wlr_surface;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_send_enter(mut surface: *mut wlr_surface,
                                                mut output: *mut wlr_output) {
    let mut client: *mut wl_client =
        wl_resource_get_client((*surface).resource);
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*output).resources.next);
    while wl_resource_get_link(resource) !=
              &mut (*output).resources as *mut wl_list {
        if client == wl_resource_get_client(resource) {
            wl_surface_send_enter((*surface).resource, resource);
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_send_leave(mut surface: *mut wlr_surface,
                                                mut output: *mut wlr_output) {
    let mut client: *mut wl_client =
        wl_resource_get_client((*surface).resource);
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*output).resources.next);
    while wl_resource_get_link(resource) !=
              &mut (*output).resources as *mut wl_list {
        if client == wl_resource_get_client(resource) {
            wl_surface_send_leave((*surface).resource, resource);
        }
        resource =
            wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
#[inline]
unsafe extern "C" fn timespec_to_msec(mut a: *const timespec) -> int64_t {
    return (*a).tv_sec * 1000i32 as libc::c_long +
               (*a).tv_nsec / 1000000i32 as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_send_frame_done(mut surface:
                                                         *mut wlr_surface,
                                                     mut when:
                                                         *const timespec) {
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    let mut tmp: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    tmp = 0 as *mut wl_resource;
    resource =
        wl_resource_from_link((*surface).current.frame_callback_list.next);
    tmp =
        wl_resource_from_link((*(*surface).current.frame_callback_list.next).next);
    while wl_resource_get_link(resource) !=
              &mut (*surface).current.frame_callback_list as *mut wl_list {
        wl_callback_send_done(resource, timespec_to_msec(when) as uint32_t);
        wl_resource_destroy(resource);
        resource = tmp;
        tmp = wl_resource_from_link((*wl_resource_get_link(resource)).next)
    };
}
unsafe extern "C" fn surface_for_each_surface(mut surface: *mut wlr_surface,
                                              mut x: libc::c_int,
                                              mut y: libc::c_int,
                                              mut iterator:
                                                  wlr_surface_iterator_func_t,
                                              mut user_data:
                                                  *mut libc::c_void) {
    iterator.expect("non-null function pointer")(surface, x, y, user_data);
    let mut subsurface: *mut wlr_subsurface = 0 as *mut wlr_subsurface;
    subsurface =
        ((*surface).subsurfaces.next as *mut libc::c_char).offset(-232) as
            *mut wlr_subsurface;
    while &mut (*subsurface).parent_link as *mut wl_list !=
              &mut (*surface).subsurfaces as *mut wl_list {
        let mut state: *mut wlr_subsurface_state = &mut (*subsurface).current;
        let mut sx: libc::c_int = (*state).x;
        let mut sy: libc::c_int = (*state).y;
        surface_for_each_surface((*subsurface).surface, x + sx, y + sy,
                                 iterator, user_data);
        subsurface =
            ((*subsurface).parent_link.next as *mut libc::c_char).offset(-232)
                as *mut wlr_subsurface
    };
}
/* *
 * Call `iterator` on each surface in the surface tree, with the surface's
 * position relative to the root surface. The function is called from root to
 * leaves (in rendering order).
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_for_each_surface(mut surface:
                                                          *mut wlr_surface,
                                                      mut iterator:
                                                          wlr_surface_iterator_func_t,
                                                      mut user_data:
                                                          *mut libc::c_void) {
    surface_for_each_surface(surface, 0i32, 0i32, iterator, user_data);
}
unsafe extern "C" fn handle_bounding_box_surface(mut surface:
                                                     *mut wlr_surface,
                                                 mut x: libc::c_int,
                                                 mut y: libc::c_int,
                                                 mut data:
                                                     *mut libc::c_void) {
    let mut acc: *mut bound_acc = data as *mut bound_acc;
    (*acc).min_x = min(x, (*acc).min_x);
    (*acc).min_y = min(y, (*acc).min_y);
    (*acc).max_x = max(x + (*surface).current.width, (*acc).max_x);
    (*acc).max_y = max(y + (*surface).current.height, (*acc).max_y);
}
/* *
 * Get the bounding box that contains the surface and all subsurfaces in
 * surface coordinates.
 * X and y may be negative, if there are subsurfaces with negative position.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_get_extends(mut surface:
                                                     *mut wlr_surface,
                                                 mut box_0: *mut wlr_box) {
    let mut acc: bound_acc =
        {
            let mut init =
                bound_acc{min_x: 0i32,
                          min_y: 0i32,
                          max_x: (*surface).current.width,
                          max_y: (*surface).current.height,};
            init
        };
    wlr_surface_for_each_surface(surface,
                                 Some(handle_bounding_box_surface as
                                          unsafe extern "C" fn(_:
                                                                   *mut wlr_surface,
                                                               _: libc::c_int,
                                                               _: libc::c_int,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> ()),
                                 &mut acc as *mut bound_acc as
                                     *mut libc::c_void);
    (*box_0).x = acc.min_x;
    (*box_0).y = acc.min_y;
    (*box_0).width = acc.max_x - acc.min_x;
    (*box_0).height = acc.max_y - acc.min_y;
}
/* *
 * Get the effective damage to the surface in terms of surface local
 * coordinates. This includes damage induced by resizing and moving the
 * surface. The damage is not expected to be bounded by the surface itself.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_get_effective_damage(mut surface:
                                                              *mut wlr_surface,
                                                          mut damage:
                                                              *mut pixman_region32_t) {
    pixman_region32_clear(damage);
    // Transform and copy the buffer damage in terms of surface coordinates.
    wlr_region_transform(damage, &mut (*surface).buffer_damage,
                         (*surface).current.transform,
                         (*surface).current.buffer_width,
                         (*surface).current.buffer_height);
    wlr_region_scale(damage, damage,
                     (1.0f64 /
                          (*surface).current.scale as libc::c_float as
                              libc::c_double) as libc::c_float);
    // On resize, damage the previous bounds of the surface. The current bounds
	// have already been damaged in surface_update_damage.
    if (*surface).previous.width > (*surface).current.width ||
           (*surface).previous.height > (*surface).current.height {
        pixman_region32_union_rect(damage, damage, 0i32, 0i32,
                                   (*surface).previous.width as libc::c_uint,
                                   (*surface).previous.height as
                                       libc::c_uint);
    }
    // On move, damage where the surface was with its old dimensions.
    if (*surface).current.dx != 0i32 || (*surface).current.dy != 0i32 {
        let mut prev_x: libc::c_int = -(*surface).current.dx;
        let mut prev_y: libc::c_int = -(*surface).current.dy;
        if (*surface).previous.transform as libc::c_uint &
               WL_OUTPUT_TRANSFORM_90 as libc::c_int as libc::c_uint !=
               0i32 as libc::c_uint {
            let mut temp: libc::c_int = prev_x;
            prev_x = prev_y;
            prev_y = temp
        }
        pixman_region32_union_rect(damage, damage, prev_x, prev_y,
                                   (*surface).previous.width as libc::c_uint,
                                   (*surface).previous.height as
                                       libc::c_uint);
    };
}
