use libc;
extern "C" {
    pub type _XDisplay;
    pub type wl_display;
    pub type wl_client;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wlr_drm_format_set_add(set: *mut wlr_drm_format_set, format: uint32_t,
                              modifier: uint64_t) -> bool;
    #[no_mangle]
    fn wlr_drm_format_set_finish(set: *mut wlr_drm_format_set);
    #[no_mangle]
    fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *const EGLint,
                       configs: *mut EGLConfig, config_size: EGLint,
                       num_config: *mut EGLint) -> EGLBoolean;
    #[no_mangle]
    fn eglCreateContext(dpy: EGLDisplay, config: EGLConfig,
                        share_context: EGLContext, attrib_list: *const EGLint)
     -> EGLContext;
    #[no_mangle]
    fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;
    #[no_mangle]
    fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    #[no_mangle]
    fn eglGetConfigAttrib(dpy: EGLDisplay, config: EGLConfig,
                          attribute: EGLint, value: *mut EGLint)
     -> EGLBoolean;
    #[no_mangle]
    fn eglGetConfigs(dpy: EGLDisplay, configs: *mut EGLConfig,
                     config_size: EGLint, num_config: *mut EGLint)
     -> EGLBoolean;
    #[no_mangle]
    fn eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface;
    #[no_mangle]
    fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint)
     -> EGLBoolean;
    #[no_mangle]
    fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface,
                      ctx: EGLContext) -> EGLBoolean;
    #[no_mangle]
    fn eglQueryContext(dpy: EGLDisplay, ctx: EGLContext, attribute: EGLint,
                       value: *mut EGLint) -> EGLBoolean;
    #[no_mangle]
    fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const libc::c_char;
    #[no_mangle]
    fn eglQuerySurface(dpy: EGLDisplay, surface: EGLSurface,
                       attribute: EGLint, value: *mut EGLint) -> EGLBoolean;
    #[no_mangle]
    fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    #[no_mangle]
    fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean;
    #[no_mangle]
    fn eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean;
    #[no_mangle]
    fn eglBindAPI(api: EGLenum) -> EGLBoolean;
    #[no_mangle]
    fn eglReleaseThread() -> EGLBoolean;
    #[no_mangle]
    fn eglGetCurrentContext() -> EGLContext;
    /* creation/destruction */
    #[no_mangle]
    fn pixman_region32_init(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_fini(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_rectangles(region: *mut pixman_region32_t,
                                  n_rects: *mut libc::c_int)
     -> *mut pixman_box32_t;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn wlr_region_transform(dst: *mut pixman_region32_t,
                            src: *mut pixman_region32_t,
                            transform: wl_output_transform,
                            width: libc::c_int, height: libc::c_int);
    #[no_mangle]
    static mut eglExportDMABUFImageMESA: PFNEGLEXPORTDMABUFIMAGEMESAPROC;
    #[no_mangle]
    static mut eglQueryWaylandBufferWL: PFNEGLQUERYWAYLANDBUFFERWL;
    #[no_mangle]
    static mut eglUnbindWaylandDisplayWL: PFNEGLUNBINDWAYLANDDISPLAYWL;
    #[no_mangle]
    static mut eglBindWaylandDisplayWL: PFNEGLBINDWAYLANDDISPLAYWL;
    #[no_mangle]
    static mut eglQueryDmaBufFormatsEXT: PFNEGLQUERYDMABUFFORMATSEXTPROC;
    #[no_mangle]
    static mut eglQueryDmaBufModifiersEXT: PFNEGLQUERYDMABUFMODIFIERSEXTPROC;
    #[no_mangle]
    static mut eglCreatePlatformWindowSurfaceEXT:
           PFNEGLCREATEPLATFORMWINDOWSURFACEEXTPROC;
    #[no_mangle]
    fn load_glapi() -> bool;
    #[no_mangle]
    static mut eglDebugMessageControlKHR: PFNEGLDEBUGMESSAGECONTROLKHRPROC;
    #[no_mangle]
    static mut eglGetPlatformDisplayEXT: PFNEGLGETPLATFORMDISPLAYEXTPROC;
    #[no_mangle]
    static mut eglCreateImageKHR: PFNEGLCREATEIMAGEKHRPROC;
    #[no_mangle]
    static mut eglDestroyImageKHR: PFNEGLDESTROYIMAGEKHRPROC;
    #[no_mangle]
    static mut eglSwapBuffersWithDamageEXT:
           PFNEGLSWAPBUFFERSWITHDAMAGEEXTPROC;
    #[no_mangle]
    static mut eglSwapBuffersWithDamageKHR:
           PFNEGLSWAPBUFFERSWITHDAMAGEKHRPROC;
    #[no_mangle]
    static mut eglExportDMABUFImageQueryMESA:
           PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC;
}
pub type __u32 = libc::c_uint;
pub type __u64 = libc::c_ulonglong;
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type khronos_int32_t = int32_t;
pub type khronos_uint64_t = uint64_t;
pub type Display = _XDisplay;
pub type EGLNativeDisplayType = *mut Display;
pub type EGLint = khronos_int32_t;
pub type EGLBoolean = libc::c_uint;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
pub type EGLClientBuffer = *mut libc::c_void;
pub type EGLAttrib = intptr_t;
pub type EGLImage = *mut libc::c_void;
pub type EGLLabelKHR = *mut libc::c_void;
pub type EGLDEBUGPROCKHR
    =
    Option<unsafe extern "C" fn(_: EGLenum, _: *const libc::c_char, _: EGLint,
                                _: EGLLabelKHR, _: EGLLabelKHR,
                                _: *const libc::c_char) -> ()>;
pub type PFNEGLDEBUGMESSAGECONTROLKHRPROC
    =
    Option<unsafe extern "C" fn(_: EGLDEBUGPROCKHR, _: *const EGLAttrib)
               -> EGLint>;
pub type EGLImageKHR = *mut libc::c_void;
pub type PFNEGLCREATEIMAGEKHRPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLContext, _: EGLenum,
                                _: EGLClientBuffer, _: *const EGLint)
               -> EGLImageKHR>;
pub type PFNEGLDESTROYIMAGEKHRPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLImageKHR) -> EGLBoolean>;
pub type EGLuint64KHR = khronos_uint64_t;
pub type PFNEGLSWAPBUFFERSWITHDAMAGEKHRPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLSurface, _: *mut EGLint,
                                _: EGLint) -> EGLBoolean>;
pub type PFNEGLQUERYDMABUFFORMATSEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLint, _: *mut EGLint,
                                _: *mut EGLint) -> EGLBoolean>;
pub type PFNEGLQUERYDMABUFMODIFIERSEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLint, _: EGLint,
                                _: *mut EGLuint64KHR, _: *mut EGLBoolean,
                                _: *mut EGLint) -> EGLBoolean>;
pub type PFNEGLGETPLATFORMDISPLAYEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLenum, _: *mut libc::c_void,
                                _: *const EGLint) -> EGLDisplay>;
pub type PFNEGLCREATEPLATFORMWINDOWSURFACEEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLConfig,
                                _: *mut libc::c_void, _: *const EGLint)
               -> EGLSurface>;
pub type PFNEGLSWAPBUFFERSWITHDAMAGEEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLSurface, _: *mut EGLint,
                                _: EGLint) -> EGLBoolean>;
pub type PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLImageKHR,
                                _: *mut libc::c_int, _: *mut libc::c_int,
                                _: *mut EGLuint64KHR) -> EGLBoolean>;
pub type PFNEGLEXPORTDMABUFIMAGEMESAPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLImageKHR,
                                _: *mut libc::c_int, _: *mut EGLint,
                                _: *mut EGLint) -> EGLBoolean>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;
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
/*  pixman_box32_t	rects[size];   in memory but not explicitly declared */
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
pub type PFNEGLUNBINDWAYLANDDISPLAYWL
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: *mut wl_display)
               -> EGLBoolean>;
pub type PFNEGLBINDWAYLANDDISPLAYWL
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: *mut wl_display)
               -> EGLBoolean>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub fd: EGLint,
    pub offset: EGLint,
    pub pitch: EGLint,
    pub mod_lo: EGLint,
    pub mod_hi: EGLint,
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
unsafe extern "C" fn egl_get_config(mut disp: EGLDisplay,
                                    mut attribs: *mut EGLint,
                                    mut out: *mut EGLConfig,
                                    mut visual_id: EGLint) -> bool {
    let mut count: EGLint = 0i32;
    let mut matched: EGLint = 0i32;
    let mut ret: EGLint = 0;
    ret =
        eglGetConfigs(disp, 0 as *mut EGLConfig, 0i32, &mut count) as EGLint;
    if ret == 0i32 || count == 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] eglGetConfigs returned no configs\x00" as *const u8
                     as *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 16i32);
        return 0i32 != 0
    }
    let vla = count as usize;
    let mut configs: Vec<EGLConfig> =
        ::std::vec::from_elem(0 as *mut libc::c_void, vla);
    ret =
        eglChooseConfig(disp, attribs, configs.as_mut_ptr(), count,
                        &mut matched) as EGLint;
    if ret == 0i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] eglChooseConfig failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 24i32);
        return 0i32 != 0
    }
    let mut i: libc::c_int = 0i32;
    while i < matched {
        let mut visual: EGLint = 0;
        if !(eglGetConfigAttrib(disp,
                                *configs.as_mut_ptr().offset(i as isize),
                                0x302ei32, &mut visual) == 0) {
            if visual_id == 0 || visual == visual_id {
                *out = *configs.as_mut_ptr().offset(i as isize);
                return 1i32 != 0
            }
        }
        i += 1
    }
    _wlr_log(WLR_ERROR,
             b"[%s:%d] no valid egl config found\x00" as *const u8 as
                 *const libc::c_char,
             b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
             41i32);
    return 0i32 != 0;
}
unsafe extern "C" fn egl_log_importance_to_wlr(mut type_0: EGLint)
 -> wlr_log_importance {
    match type_0 {
        13241 => { return WLR_ERROR }
        13242 => { return WLR_ERROR }
        13243 => { return WLR_ERROR }
        13244 => { return WLR_INFO }
        _ => { return WLR_INFO }
    };
}
unsafe extern "C" fn egl_log(mut error: EGLenum,
                             mut command: *const libc::c_char,
                             mut msg_type: EGLint, mut thread: EGLLabelKHR,
                             mut obj: EGLLabelKHR,
                             mut msg: *const libc::c_char) {
    _wlr_log(egl_log_importance_to_wlr(msg_type),
             b"[EGL] command: %s, error: 0x%x, message: \"%s\"\x00" as
                 *const u8 as *const libc::c_char, command, error, msg);
}
unsafe extern "C" fn check_egl_ext(mut exts: *const libc::c_char,
                                   mut ext: *const libc::c_char) -> bool {
    let mut extlen: size_t = strlen(ext);
    let mut end: *const libc::c_char = exts.offset(strlen(exts) as isize);
    while exts < end {
        if *exts as libc::c_int == ' ' as i32 {
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
unsafe extern "C" fn init_dmabuf_formats(mut egl: *mut wlr_egl) {
    let mut formats: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut formats_len: libc::c_int =
        get_egl_dmabuf_formats(egl, &mut formats);
    if formats_len < 0i32 { return }
    let mut i: libc::c_int = 0i32;
    while i < formats_len {
        let mut fmt: uint32_t = *formats.offset(i as isize) as uint32_t;
        let mut modifiers: *mut uint64_t = 0 as *mut uint64_t;
        let mut modifiers_len: libc::c_int =
            get_egl_dmabuf_modifiers(egl, fmt as libc::c_int, &mut modifiers);
        if !(modifiers_len < 0i32) {
            if modifiers_len == 0i32 {
                wlr_drm_format_set_add(&mut (*egl).dmabuf_formats, fmt,
                                       ((0i32 as __u64) << 56i32 |
                                            (1u64 <<
                                                 56i32).wrapping_sub(1i32 as
                                                                         libc::c_ulonglong)
                                                & 0xffffffffffffffu64) as
                                           uint64_t);
            }
            let mut j: libc::c_int = 0i32;
            while j < modifiers_len {
                wlr_drm_format_set_add(&mut (*egl).dmabuf_formats, fmt,
                                       *modifiers.offset(j as isize));
                j += 1
            }
            free(modifiers as *mut libc::c_void);
        }
        i += 1
    }
    let mut str_formats: *mut libc::c_char =
        malloc((formats_len * 5i32 + 1i32) as libc::c_ulong) as
            *mut libc::c_char;
    if !str_formats.is_null() {
        let mut i_0: libc::c_int = 0i32;
        while i_0 < formats_len {
            snprintf(&mut *str_formats.offset((i_0 * 5i32) as isize) as
                         *mut libc::c_char,
                     ((formats_len - i_0) * 5i32 + 1i32) as libc::c_ulong,
                     b"%.4s \x00" as *const u8 as *const libc::c_char,
                     &mut *formats.offset(i_0 as isize) as *mut libc::c_int as
                         *mut libc::c_char);
            i_0 += 1
        }
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Supported dmabuf buffer formats: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 119i32, str_formats);
        free(str_formats as *mut libc::c_void);
    }
    free(formats as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_init(mut egl: *mut wlr_egl,
                                      mut platform: EGLenum,
                                      mut remote_display: *mut libc::c_void,
                                      mut config_attribs: *mut EGLint,
                                      mut visual_id: EGLint) -> bool {
    let mut major: EGLint = 0;
    let mut minor: EGLint = 0;
    let mut ext_context_priority: bool = false;
    let mut atti: size_t = 0;
    let mut attribs: [EGLint; 5] = [0; 5];
    let mut request_high_priority: bool = false;
    if !load_glapi() { return 0i32 != 0 }
    if eglDebugMessageControlKHR.is_some() {
        static mut debug_attribs: [EGLAttrib; 9] =
            [0x33b9i32 as EGLAttrib, 1i32 as EGLAttrib,
             0x33bai32 as EGLAttrib, 1i32 as EGLAttrib,
             0x33bbi32 as EGLAttrib, 1i32 as EGLAttrib,
             0x33bci32 as EGLAttrib, 1i32 as EGLAttrib,
             0x3038i32 as EGLAttrib];
        eglDebugMessageControlKHR.expect("non-null function pointer")(Some(egl_log
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        EGLenum,
                                                                                                    _:
                                                                                                        *const libc::c_char,
                                                                                                    _:
                                                                                                        EGLint,
                                                                                                    _:
                                                                                                        EGLLabelKHR,
                                                                                                    _:
                                                                                                        EGLLabelKHR,
                                                                                                    _:
                                                                                                        *const libc::c_char)
                                                                                   ->
                                                                                       ()),
                                                                      debug_attribs.as_ptr());
    }
    if eglBindAPI(0x30a0i32 as EGLenum) == 0i32 as libc::c_uint {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to bind to the OpenGL ES API\x00" as
                     *const u8 as *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 144i32);
    } else {
        if platform == 0x31ddi32 as libc::c_uint {
            if remote_display.is_null() {
            } else {
                __assert_fail(b"remote_display == NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"../render/egl.c\x00" as *const u8 as
                                  *const libc::c_char, 149i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 72],
                                                        &[libc::c_char; 72]>(b"_Bool wlr_egl_init(struct wlr_egl *, EGLenum, void *, EGLint *, EGLint)\x00")).as_ptr());
            };
            (*egl).display =
                eglGetPlatformDisplayEXT.expect("non-null function pointer")(platform,
                                                                             0
                                                                                 as
                                                                                 EGLNativeDisplayType
                                                                                 as
                                                                                 *mut libc::c_void,
                                                                             0
                                                                                 as
                                                                                 *const EGLint)
        } else {
            (*egl).display =
                eglGetPlatformDisplayEXT.expect("non-null function pointer")(platform,
                                                                             remote_display,
                                                                             0
                                                                                 as
                                                                                 *const EGLint)
        }
        if (*egl).display.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to create EGL display\x00" as *const u8
                         as *const libc::c_char,
                     b"../render/egl.c\x00" as *const u8 as
                         *const libc::c_char, 155i32);
        } else {
            (*egl).platform = platform;
            major = 0;
            minor = 0;
            if eglInitialize((*egl).display, &mut major, &mut minor) ==
                   0i32 as libc::c_uint {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to initialize EGL\x00" as *const u8
                             as *const libc::c_char,
                         b"../render/egl.c\x00" as *const u8 as
                             *const libc::c_char, 163i32);
            } else if !egl_get_config((*egl).display, config_attribs,
                                      &mut (*egl).config, visual_id) {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to get EGL config\x00" as *const u8
                             as *const libc::c_char,
                         b"../render/egl.c\x00" as *const u8 as
                             *const libc::c_char, 168i32);
            } else {
                (*egl).exts_str = eglQueryString((*egl).display, 0x3055i32);
                _wlr_log(WLR_INFO,
                         b"[%s:%d] Using EGL %d.%d\x00" as *const u8 as
                             *const libc::c_char,
                         b"../render/egl.c\x00" as *const u8 as
                             *const libc::c_char, 174i32, major, minor);
                _wlr_log(WLR_INFO,
                         b"[%s:%d] Supported EGL extensions: %s\x00" as
                             *const u8 as *const libc::c_char,
                         b"../render/egl.c\x00" as *const u8 as
                             *const libc::c_char, 175i32, (*egl).exts_str);
                _wlr_log(WLR_INFO,
                         b"[%s:%d] EGL vendor: %s\x00" as *const u8 as
                             *const libc::c_char,
                         b"../render/egl.c\x00" as *const u8 as
                             *const libc::c_char, 176i32,
                         eglQueryString((*egl).display, 0x3053i32));
                (*egl).exts.image_base_khr =
                    check_egl_ext((*egl).exts_str,
                                  b"EGL_KHR_image_base\x00" as *const u8 as
                                      *const libc::c_char) as libc::c_int != 0
                        && eglCreateImageKHR.is_some() &&
                        eglDestroyImageKHR.is_some();
                (*egl).exts.buffer_age_ext =
                    check_egl_ext((*egl).exts_str,
                                  b"EGL_EXT_buffer_age\x00" as *const u8 as
                                      *const libc::c_char);
                (*egl).exts.swap_buffers_with_damage_ext =
                    check_egl_ext((*egl).exts_str,
                                  b"EGL_EXT_swap_buffers_with_damage\x00" as
                                      *const u8 as *const libc::c_char) as
                        libc::c_int != 0 &&
                        eglSwapBuffersWithDamageEXT.is_some();
                (*egl).exts.swap_buffers_with_damage_khr =
                    check_egl_ext((*egl).exts_str,
                                  b"EGL_KHR_swap_buffers_with_damage\x00" as
                                      *const u8 as *const libc::c_char) as
                        libc::c_int != 0 &&
                        eglSwapBuffersWithDamageKHR.is_some();
                (*egl).exts.image_dmabuf_import_ext =
                    check_egl_ext((*egl).exts_str,
                                  b"EGL_EXT_image_dma_buf_import\x00" as
                                      *const u8 as *const libc::c_char);
                (*egl).exts.image_dmabuf_import_modifiers_ext =
                    check_egl_ext((*egl).exts_str,
                                  b"EGL_EXT_image_dma_buf_import_modifiers\x00"
                                      as *const u8 as *const libc::c_char) as
                        libc::c_int != 0 && eglQueryDmaBufFormatsEXT.is_some()
                        && eglQueryDmaBufModifiersEXT.is_some();
                (*egl).exts.image_dma_buf_export_mesa =
                    check_egl_ext((*egl).exts_str,
                                  b"EGL_MESA_image_dma_buf_export\x00" as
                                      *const u8 as *const libc::c_char) as
                        libc::c_int != 0 &&
                        eglExportDMABUFImageQueryMESA.is_some() &&
                        eglExportDMABUFImageMESA.is_some();
                init_dmabuf_formats(egl);
                (*egl).exts.bind_wayland_display_wl =
                    check_egl_ext((*egl).exts_str,
                                  b"EGL_WL_bind_wayland_display\x00" as
                                      *const u8 as *const libc::c_char) as
                        libc::c_int != 0 && eglBindWaylandDisplayWL.is_some()
                        && eglUnbindWaylandDisplayWL.is_some() &&
                        eglQueryWaylandBufferWL.is_some();
                ext_context_priority =
                    check_egl_ext((*egl).exts_str,
                                  b"EGL_IMG_context_priority\x00" as *const u8
                                      as *const libc::c_char);
                atti = 0i32 as size_t;
                attribs = [0; 5];
                let fresh0 = atti;
                atti = atti.wrapping_add(1);
                attribs[fresh0 as usize] = 0x3098i32;
                let fresh1 = atti;
                atti = atti.wrapping_add(1);
                attribs[fresh1 as usize] = 2i32;
                // On DRM, request a high priority context if possible
                request_high_priority =
                    ext_context_priority as libc::c_int != 0 &&
                        platform == 0x31d7i32 as libc::c_uint;
                // Try to reschedule all of our rendering to be completed first. If it
	// fails, it will fallback to the default priority (MEDIUM).
                if request_high_priority {
                    let fresh2 = atti;
                    atti = atti.wrapping_add(1);
                    attribs[fresh2 as usize] = 0x3100i32;
                    let fresh3 = atti;
                    atti = atti.wrapping_add(1);
                    attribs[fresh3 as usize] = 0x3101i32
                }
                let fresh4 = atti;
                atti = atti.wrapping_add(1);
                attribs[fresh4 as usize] = 0x3038i32;
                if atti <=
                       (::std::mem::size_of::<[EGLint; 5]>() as
                            libc::c_ulong).wrapping_div(::std::mem::size_of::<EGLint>()
                                                            as libc::c_ulong)
                   {
                } else {
                    __assert_fail(b"atti <= sizeof(attribs)/sizeof(attribs[0])\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"../render/egl.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  228i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 72],
                                                            &[libc::c_char; 72]>(b"_Bool wlr_egl_init(struct wlr_egl *, EGLenum, void *, EGLint *, EGLint)\x00")).as_ptr());
                };
                (*egl).context =
                    eglCreateContext((*egl).display, (*egl).config,
                                     0 as EGLContext, attribs.as_mut_ptr());
                if (*egl).context.is_null() {
                    _wlr_log(WLR_ERROR,
                             b"[%s:%d] Failed to create EGL context\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../render/egl.c\x00" as *const u8 as
                                 *const libc::c_char, 233i32);
                } else {
                    if request_high_priority {
                        let mut priority: EGLint = 0x3102i32;
                        eglQueryContext((*egl).display, (*egl).context,
                                        0x3100i32, &mut priority);
                        if priority != 0x3101i32 {
                            _wlr_log(WLR_INFO,
                                     b"[%s:%d] Failed to obtain a high priority context\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"../render/egl.c\x00" as *const u8 as
                                         *const libc::c_char, 242i32);
                        } else {
                            _wlr_log(WLR_DEBUG,
                                     b"[%s:%d] Obtained high priority context\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"../render/egl.c\x00" as *const u8 as
                                         *const libc::c_char, 244i32);
                        }
                    }
                    if eglMakeCurrent((*egl).display, 0 as EGLSurface,
                                      0 as EGLSurface, (*egl).context) == 0 {
                        _wlr_log(WLR_ERROR,
                                 b"[%s:%d] Failed to make EGL context current\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"../render/egl.c\x00" as *const u8 as
                                     *const libc::c_char, 250i32);
                    } else { return 1i32 != 0 }
                }
            }
        }
    }
    eglMakeCurrent(0 as EGLDisplay, 0 as EGLSurface, 0 as EGLSurface,
                   0 as EGLContext);
    if !(*egl).display.is_null() { eglTerminate((*egl).display); }
    eglReleaseThread();
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_finish(mut egl: *mut wlr_egl) {
    if egl.is_null() { return }
    wlr_drm_format_set_finish(&mut (*egl).dmabuf_formats);
    eglMakeCurrent((*egl).display, 0 as EGLSurface, 0 as EGLSurface,
                   0 as EGLContext);
    if !(*egl).wl_display.is_null() {
        if (*egl).exts.bind_wayland_display_wl as libc::c_int != 0 {
        } else {
            __assert_fail(b"egl->exts.bind_wayland_display_wl\x00" as
                              *const u8 as *const libc::c_char,
                          b"../render/egl.c\x00" as *const u8 as
                              *const libc::c_char, 274i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"void wlr_egl_finish(struct wlr_egl *)\x00")).as_ptr());
        };
        eglUnbindWaylandDisplayWL.expect("non-null function pointer")((*egl).display,
                                                                      (*egl).wl_display);
    }
    eglDestroyContext((*egl).display, (*egl).context);
    eglTerminate((*egl).display);
    eglReleaseThread();
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_bind_display(mut egl: *mut wlr_egl,
                                              mut local_display:
                                                  *mut wl_display) -> bool {
    if !(*egl).exts.bind_wayland_display_wl { return 0i32 != 0 }
    if eglBindWaylandDisplayWL.expect("non-null function pointer")((*egl).display,
                                                                   local_display)
           != 0 {
        (*egl).wl_display = local_display;
        return 1i32 != 0
    }
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_destroy_image(mut egl: *mut wlr_egl,
                                               mut image: EGLImage) -> bool {
    if !(*egl).exts.image_base_khr { return 0i32 != 0 }
    if image.is_null() { return 1i32 != 0 }
    return eglDestroyImageKHR.expect("non-null function pointer")((*egl).display,
                                                                  image) != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_create_surface(mut egl: *mut wlr_egl,
                                                mut window: *mut libc::c_void)
 -> EGLSurface {
    if eglCreatePlatformWindowSurfaceEXT.is_some() {
    } else {
        __assert_fail(b"eglCreatePlatformWindowSurfaceEXT\x00" as *const u8 as
                          *const libc::c_char,
                      b"../render/egl.c\x00" as *const u8 as
                          *const libc::c_char, 307i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 60],
                                                &[libc::c_char; 60]>(b"EGLSurface wlr_egl_create_surface(struct wlr_egl *, void *)\x00")).as_ptr());
    };
    let mut surf: EGLSurface =
        eglCreatePlatformWindowSurfaceEXT.expect("non-null function pointer")((*egl).display,
                                                                              (*egl).config,
                                                                              window,
                                                                              0
                                                                                  as
                                                                                  *const EGLint);
    if surf.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to create EGL surface\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 311i32);
        return 0 as EGLSurface
    }
    return surf;
}
unsafe extern "C" fn egl_get_buffer_age(mut egl: *mut wlr_egl,
                                        mut surface: EGLSurface)
 -> libc::c_int {
    if !(*egl).exts.buffer_age_ext { return -1i32 }
    let mut buffer_age: EGLint = 0;
    let mut ok: EGLBoolean =
        eglQuerySurface((*egl).display, surface, 0x313di32, &mut buffer_age);
    if ok == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to get EGL surface buffer age\x00" as
                     *const u8 as *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 326i32);
        return -1i32
    }
    return buffer_age;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_make_current(mut egl: *mut wlr_egl,
                                              mut surface: EGLSurface,
                                              mut buffer_age:
                                                  *mut libc::c_int) -> bool {
    if eglMakeCurrent((*egl).display, surface, surface, (*egl).context) == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] eglMakeCurrent failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 336i32);
        return 0i32 != 0
    }
    if !buffer_age.is_null() {
        *buffer_age = egl_get_buffer_age(egl, surface)
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_is_current(mut egl: *mut wlr_egl) -> bool {
    return eglGetCurrentContext() == (*egl).context;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_swap_buffers(mut egl: *mut wlr_egl,
                                              mut surface: EGLSurface,
                                              mut damage:
                                                  *mut pixman_region32_t)
 -> bool {
    // Never block when swapping buffers on Wayland
    if (*egl).platform == 0x31d8i32 as libc::c_uint {
        eglSwapInterval((*egl).display, 0i32);
    }
    let mut ret: EGLBoolean = 0;
    if !damage.is_null() &&
           ((*egl).exts.swap_buffers_with_damage_ext as libc::c_int != 0 ||
                (*egl).exts.swap_buffers_with_damage_khr as libc::c_int != 0)
       {
        let mut width: EGLint = 0i32;
        let mut height: EGLint = 0i32;
        eglQuerySurface((*egl).display, surface, 0x3057i32, &mut width);
        eglQuerySurface((*egl).display, surface, 0x3056i32, &mut height);
        let mut flipped_damage: pixman_region32_t =
            pixman_region32_t{extents:
                                  pixman_box32_t{x1: 0, y1: 0, x2: 0, y2: 0,},
                              data: 0 as *mut pixman_region32_data_t,};
        pixman_region32_init(&mut flipped_damage);
        wlr_region_transform(&mut flipped_damage, damage,
                             WL_OUTPUT_TRANSFORM_FLIPPED_180, width, height);
        let mut nrects: libc::c_int = 0;
        let mut rects: *mut pixman_box32_t =
            pixman_region32_rectangles(&mut flipped_damage, &mut nrects);
        let vla = (4i32 * nrects + 1i32) as usize;
        let mut egl_damage: Vec<EGLint> = ::std::vec::from_elem(0, vla);
        let mut i: libc::c_int = 0i32;
        while i < nrects {
            *egl_damage.as_mut_ptr().offset((4i32 * i) as isize) =
                (*rects.offset(i as isize)).x1;
            *egl_damage.as_mut_ptr().offset((4i32 * i + 1i32) as isize) =
                (*rects.offset(i as isize)).y1;
            *egl_damage.as_mut_ptr().offset((4i32 * i + 2i32) as isize) =
                (*rects.offset(i as isize)).x2 -
                    (*rects.offset(i as isize)).x1;
            *egl_damage.as_mut_ptr().offset((4i32 * i + 3i32) as isize) =
                (*rects.offset(i as isize)).y2 -
                    (*rects.offset(i as isize)).y1;
            i += 1
        }
        pixman_region32_fini(&mut flipped_damage);
        if nrects == 0i32 {
            // Swapping with no rects is the same as swapping with the entire
			// surface damaged. To swap with no damage, we set the damage region
			// to a single empty rectangle.
            nrects = 1i32;
            memset(egl_damage.as_mut_ptr() as *mut libc::c_void, 0i32,
                   (vla * ::std::mem::size_of::<EGLint>()) as libc::c_ulong);
        }
        if (*egl).exts.swap_buffers_with_damage_ext {
            ret =
                eglSwapBuffersWithDamageEXT.expect("non-null function pointer")((*egl).display,
                                                                                surface,
                                                                                egl_damage.as_mut_ptr(),
                                                                                nrects)
        } else {
            ret =
                eglSwapBuffersWithDamageKHR.expect("non-null function pointer")((*egl).display,
                                                                                surface,
                                                                                egl_damage.as_mut_ptr(),
                                                                                nrects)
        }
    } else { ret = eglSwapBuffers((*egl).display, surface) }
    if ret == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] eglSwapBuffers failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 402i32);
        return 0i32 != 0
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_create_image_from_wl_drm(mut egl:
                                                              *mut wlr_egl,
                                                          mut data:
                                                              *mut wl_resource,
                                                          mut fmt:
                                                              *mut EGLint,
                                                          mut width:
                                                              *mut libc::c_int,
                                                          mut height:
                                                              *mut libc::c_int,
                                                          mut inverted_y:
                                                              *mut bool)
 -> EGLImageKHR {
    if !(*egl).exts.bind_wayland_display_wl || !(*egl).exts.image_base_khr {
        return 0 as *mut libc::c_void
    }
    if eglQueryWaylandBufferWL.expect("non-null function pointer")((*egl).display,
                                                                   data,
                                                                   0x3080i32,
                                                                   fmt) == 0 {
        return 0 as *mut libc::c_void
    }
    eglQueryWaylandBufferWL.expect("non-null function pointer")((*egl).display,
                                                                data,
                                                                0x3057i32,
                                                                width);
    eglQueryWaylandBufferWL.expect("non-null function pointer")((*egl).display,
                                                                data,
                                                                0x3056i32,
                                                                height);
    let mut _inverted_y: EGLint = 0;
    if eglQueryWaylandBufferWL.expect("non-null function pointer")((*egl).display,
                                                                   data,
                                                                   0x31dbi32,
                                                                   &mut _inverted_y)
           != 0 {
        *inverted_y = _inverted_y != 0
    } else { *inverted_y = 0i32 != 0 }
    let attribs: [EGLint; 3] = [0x31d6i32, 0i32, 0x3038i32];
    return eglCreateImageKHR.expect("non-null function pointer")((*egl).display,
                                                                 (*egl).context,
                                                                 0x31d5i32 as
                                                                     EGLenum,
                                                                 data as
                                                                     EGLClientBuffer,
                                                                 attribs.as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_create_image_from_dmabuf(mut egl:
                                                              *mut wlr_egl,
                                                          mut attributes:
                                                              *mut wlr_dmabuf_attributes)
 -> EGLImageKHR {
    if !(*egl).exts.image_base_khr || !(*egl).exts.image_dmabuf_import_ext {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] dmabuf import extension not present\x00" as
                     *const u8 as *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 441i32);
        return 0 as *mut libc::c_void
    }
    let mut has_modifier: bool = 0i32 != 0;
    // we assume the same way we assumed formats without the import_modifiers
	// extension that mod_linear is supported. The special mod mod_invalid
	// is sometimes used to signal modifier unawareness which is what we
	// have here
    if (*attributes).modifier as libc::c_ulonglong !=
           (0i32 as __u64) << 56i32 |
               (1u64 << 56i32).wrapping_sub(1i32 as libc::c_ulonglong) &
                   0xffffffffffffffu64 &&
           (*attributes).modifier as libc::c_ulonglong !=
               (0i32 as __u64) << 56i32 |
                   0i32 as libc::c_ulonglong & 0xffffffffffffffu64 {
        if !(*egl).exts.image_dmabuf_import_modifiers_ext {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] dmabuf modifiers extension not present\x00" as
                         *const u8 as *const libc::c_char,
                     b"../render/egl.c\x00" as *const u8 as
                         *const libc::c_char, 454i32);
            return 0 as *mut libc::c_void
        }
        has_modifier = 1i32 != 0
    }
    let mut atti: libc::c_uint = 0i32 as libc::c_uint;
    let mut attribs: [EGLint; 50] = [0; 50];
    let fresh5 = atti;
    atti = atti.wrapping_add(1);
    attribs[fresh5 as usize] = 0x3057i32;
    let fresh6 = atti;
    atti = atti.wrapping_add(1);
    attribs[fresh6 as usize] = (*attributes).width;
    let fresh7 = atti;
    atti = atti.wrapping_add(1);
    attribs[fresh7 as usize] = 0x3056i32;
    let fresh8 = atti;
    atti = atti.wrapping_add(1);
    attribs[fresh8 as usize] = (*attributes).height;
    let fresh9 = atti;
    atti = atti.wrapping_add(1);
    attribs[fresh9 as usize] = 0x3271i32;
    let fresh10 = atti;
    atti = atti.wrapping_add(1);
    attribs[fresh10 as usize] = (*attributes).format as EGLint;
    let mut attr_names: [C2RustUnnamed_0; 4] =
        [{
             let mut init =
                 C2RustUnnamed_0{fd: 0x3272i32,
                                 offset: 0x3273i32,
                                 pitch: 0x3274i32,
                                 mod_lo: 0x3443i32,
                                 mod_hi: 0x3444i32,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{fd: 0x3275i32,
                                 offset: 0x3276i32,
                                 pitch: 0x3277i32,
                                 mod_lo: 0x3445i32,
                                 mod_hi: 0x3446i32,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{fd: 0x3278i32,
                                 offset: 0x3279i32,
                                 pitch: 0x327ai32,
                                 mod_lo: 0x3447i32,
                                 mod_hi: 0x3448i32,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{fd: 0x3440i32,
                                 offset: 0x3441i32,
                                 pitch: 0x3442i32,
                                 mod_lo: 0x3449i32,
                                 mod_hi: 0x344ai32,};
             init
         }];
    let mut i: libc::c_int = 0i32;
    while i < (*attributes).n_planes {
        let fresh11 = atti;
        atti = atti.wrapping_add(1);
        attribs[fresh11 as usize] = attr_names[i as usize].fd;
        let fresh12 = atti;
        atti = atti.wrapping_add(1);
        attribs[fresh12 as usize] = (*attributes).fd[i as usize];
        let fresh13 = atti;
        atti = atti.wrapping_add(1);
        attribs[fresh13 as usize] = attr_names[i as usize].offset;
        let fresh14 = atti;
        atti = atti.wrapping_add(1);
        attribs[fresh14 as usize] =
            (*attributes).offset[i as usize] as EGLint;
        let fresh15 = atti;
        atti = atti.wrapping_add(1);
        attribs[fresh15 as usize] = attr_names[i as usize].pitch;
        let fresh16 = atti;
        atti = atti.wrapping_add(1);
        attribs[fresh16 as usize] =
            (*attributes).stride[i as usize] as EGLint;
        if has_modifier {
            let fresh17 = atti;
            atti = atti.wrapping_add(1);
            attribs[fresh17 as usize] = attr_names[i as usize].mod_lo;
            let fresh18 = atti;
            atti = atti.wrapping_add(1);
            attribs[fresh18 as usize] =
                ((*attributes).modifier & 0xffffffffu32 as libc::c_ulong) as
                    EGLint;
            let fresh19 = atti;
            atti = atti.wrapping_add(1);
            attribs[fresh19 as usize] = attr_names[i as usize].mod_hi;
            let fresh20 = atti;
            atti = atti.wrapping_add(1);
            attribs[fresh20 as usize] =
                ((*attributes).modifier >> 32i32) as EGLint
        }
        i += 1
    }
    let fresh21 = atti;
    atti = atti.wrapping_add(1);
    attribs[fresh21 as usize] = 0x3038i32;
    if (atti as libc::c_ulong) <
           (::std::mem::size_of::<[EGLint; 50]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<EGLint>() as
                                                libc::c_ulong) {
    } else {
        __assert_fail(b"atti < sizeof(attribs)/sizeof(attribs[0])\x00" as
                          *const u8 as *const libc::c_char,
                      b"../render/egl.c\x00" as *const u8 as
                          *const libc::c_char, 518i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 95],
                                                &[libc::c_char; 95]>(b"EGLImageKHR wlr_egl_create_image_from_dmabuf(struct wlr_egl *, struct wlr_dmabuf_attributes *)\x00")).as_ptr());
    };
    return eglCreateImageKHR.expect("non-null function pointer")((*egl).display,
                                                                 0 as
                                                                     EGLContext,
                                                                 0x3270i32 as
                                                                     EGLenum,
                                                                 0 as
                                                                     *mut libc::c_void,
                                                                 attribs.as_mut_ptr());
}
// Initialized in run_static_initializers
static mut num: libc::c_uint = 0;
unsafe extern "C" fn get_egl_dmabuf_formats(mut egl: *mut wlr_egl,
                                            mut formats:
                                                *mut *mut libc::c_int)
 -> libc::c_int {
    if !(*egl).exts.image_dmabuf_import_ext {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] DMA-BUF import extension not present\x00" as
                     *const u8 as *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 526i32);
        return -1i32
    }
    // when we only have the image_dmabuf_import extension we can't query
	// which formats are supported. These two are on almost always
	// supported; it's the intended way to just try to create buffers.
	// Just a guess but better than not supporting dmabufs at all,
	// given that the modifiers extension isn't supported everywhere.
    if !(*egl).exts.image_dmabuf_import_modifiers_ext {
        static mut fallback_formats: [libc::c_int; 2] =
            [('A' as i32 as __u32 | ('R' as i32 as __u32) << 8i32 |
                  ('2' as i32 as __u32) << 16i32 |
                  ('4' as i32 as __u32) << 24i32) as libc::c_int,
             ('X' as i32 as __u32 | ('R' as i32 as __u32) << 8i32 |
                  ('2' as i32 as __u32) << 16i32 |
                  ('4' as i32 as __u32) << 24i32) as libc::c_int];
        *formats =
            calloc(num as libc::c_ulong,
                   ::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as
                *mut libc::c_int;
        if (*formats).is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                         *const libc::c_char,
                     b"../render/egl.c\x00" as *const u8 as
                         *const libc::c_char, 545i32,
                     strerror(*__errno_location()));
            return -1i32
        }
        memcpy(*formats as *mut libc::c_void,
               fallback_formats.as_ptr() as *const libc::c_void,
               (num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong));
        return num as libc::c_int
    }
    let mut num_0: EGLint = 0;
    if eglQueryDmaBufFormatsEXT.expect("non-null function pointer")((*egl).display,
                                                                    0i32,
                                                                    0 as
                                                                        *mut EGLint,
                                                                    &mut num_0)
           == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to query number of dmabuf formats\x00" as
                     *const u8 as *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 555i32);
        return -1i32
    }
    *formats =
        calloc(num_0 as libc::c_ulong,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as
            *mut libc::c_int;
    if (*formats).is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 561i32, strerror(*__errno_location()));
        return -1i32
    }
    if eglQueryDmaBufFormatsEXT.expect("non-null function pointer")((*egl).display,
                                                                    num_0,
                                                                    *formats,
                                                                    &mut num_0)
           == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to query dmabuf format\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 566i32);
        free(*formats as *mut libc::c_void);
        return -1i32
    }
    return num_0;
}
unsafe extern "C" fn get_egl_dmabuf_modifiers(mut egl: *mut wlr_egl,
                                              mut format: libc::c_int,
                                              mut modifiers:
                                                  *mut *mut uint64_t)
 -> libc::c_int {
    if !(*egl).exts.image_dmabuf_import_ext {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] DMA-BUF extension not present\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 576i32);
        return -1i32
    }
    if !(*egl).exts.image_dmabuf_import_modifiers_ext {
        *modifiers = 0 as *mut uint64_t;
        return 0i32
    }
    let mut num_0: EGLint = 0;
    if eglQueryDmaBufModifiersEXT.expect("non-null function pointer")((*egl).display,
                                                                      format,
                                                                      0i32,
                                                                      0 as
                                                                          *mut EGLuint64KHR,
                                                                      0 as
                                                                          *mut EGLBoolean,
                                                                      &mut num_0)
           == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to query dmabuf number of modifiers\x00" as
                     *const u8 as *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 587i32);
        return -1i32
    }
    if num_0 == 0i32 { *modifiers = 0 as *mut uint64_t; return 0i32 }
    *modifiers =
        calloc(num_0 as libc::c_ulong,
               ::std::mem::size_of::<uint64_t>() as libc::c_ulong) as
            *mut uint64_t;
    if (*modifiers).is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 597i32, strerror(*__errno_location()));
        return -1i32
    }
    if eglQueryDmaBufModifiersEXT.expect("non-null function pointer")((*egl).display,
                                                                      format,
                                                                      num_0,
                                                                      *modifiers,
                                                                      0 as
                                                                          *mut EGLBoolean,
                                                                      &mut num_0)
           == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to query dmabuf modifiers\x00" as *const u8
                     as *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 603i32);
        free(*modifiers as *mut libc::c_void);
        return -1i32
    }
    return num_0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_get_dmabuf_formats(mut egl: *mut wlr_egl)
 -> *const wlr_drm_format_set {
    return &mut (*egl).dmabuf_formats;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_export_image_to_dmabuf(mut egl: *mut wlr_egl,
                                                        mut image:
                                                            EGLImageKHR,
                                                        mut width: int32_t,
                                                        mut height: int32_t,
                                                        mut flags: uint32_t,
                                                        mut attribs:
                                                            *mut wlr_dmabuf_attributes)
 -> bool {
    memset(attribs as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<wlr_dmabuf_attributes>() as libc::c_ulong);
    if !(*egl).exts.image_dma_buf_export_mesa { return 0i32 != 0 }
    // Only one set of modifiers is returned for all planes
    if eglExportDMABUFImageQueryMESA.expect("non-null function pointer")((*egl).display,
                                                                         image,
                                                                         &mut (*attribs).format
                                                                             as
                                                                             *mut uint32_t
                                                                             as
                                                                             *mut libc::c_int,
                                                                         &mut (*attribs).n_planes,
                                                                         &mut (*attribs).modifier)
           == 0 {
        return 0i32 != 0
    }
    if (*attribs).n_planes > 4i32 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] EGL returned %d planes, but only %d are supported\x00"
                     as *const u8 as *const libc::c_char,
                 b"../render/egl.c\x00" as *const u8 as *const libc::c_char,
                 630i32, (*attribs).n_planes, 4i32);
        return 0i32 != 0
    }
    if eglExportDMABUFImageMESA.expect("non-null function pointer")((*egl).display,
                                                                    image,
                                                                    (*attribs).fd.as_mut_ptr(),
                                                                    (*attribs).stride.as_mut_ptr()
                                                                        as
                                                                        *mut EGLint,
                                                                    (*attribs).offset.as_mut_ptr()
                                                                        as
                                                                        *mut EGLint)
           == 0 {
        return 0i32 != 0
    }
    (*attribs).width = width;
    (*attribs).height = height;
    (*attribs).flags = flags;
    return 1i32 != 0;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
// TODO: Allocate and return a wlr_egl
/* *
 * Initializes an EGL context for the given platform and remote display.
 * Will attempt to load all possibly required api functions.
 */
/* *
 * Frees all related EGL resources, makes the context not-current and
 * unbinds a bound wayland display.
 */
/* *
 * Binds the given display to the EGL instance.
 * This will allow clients to create EGL surfaces from wayland ones and render
 * to it.
 */
/* *
 * Returns a surface for the given native window
 * The window must match the remote display the wlr_egl was created with.
 */
/* *
 * Creates an EGL image from the given wl_drm buffer resource.
 */
/* *
 * Creates an EGL image from the given dmabuf attributes. Check usability
 * of the dmabuf with wlr_egl_check_import_dmabuf once first.
 */
/* *
 * Get the available dmabuf formats
 */
/* *
 * Destroys an EGL image created with the given wlr_egl.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_egl_destroy_surface(mut egl: *mut wlr_egl,
                                                 mut surface: EGLSurface)
 -> bool {
    if surface.is_null() { return 1i32 != 0 }
    if eglGetCurrentContext() == (*egl).context &&
           eglGetCurrentSurface(0x3059i32) == surface {
        // Reset the current EGL surface in case it's the one we're destroying,
		// otherwise the next wlr_egl_make_current call will result in a
		// use-after-free.
        wlr_egl_make_current(egl, 0 as *mut libc::c_void,
                             0 as *mut libc::c_int);
    }
    return eglDestroySurface((*egl).display, surface) != 0;
}
unsafe extern "C" fn run_static_initializers() {
    num =
        (::std::mem::size_of::<[libc::c_int; 2]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                             as libc::c_ulong) as libc::c_uint
}
#[used]
#[cfg_attr ( target_os = "linux", link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows", link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos", link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
