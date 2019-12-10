use libc;
extern "C" {
    pub type gbm_device;
    pub type gbm_bo;
    pub type gbm_surface;
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_texture_impl;
    pub type wlr_renderer_impl;
    pub type _drmModeAtomicReq;
    pub type wlr_surface;
    pub type udev_monitor;
    pub type udev;
    pub type session_impl;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn gbm_bo_destroy(bo: *mut gbm_bo);
    #[no_mangle]
    fn gbm_bo_get_format(bo: *mut gbm_bo) -> uint32_t;
    #[no_mangle]
    fn glFinish();
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn ffs(__i: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_length(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_event_loop_add_timer(loop_0: *mut wl_event_loop,
                               func: wl_event_loop_timer_func_t,
                               data: *mut libc::c_void)
     -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_timer_update(source: *mut wl_event_source,
                                    ms_delay: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn wlr_drm_format_set_add(set: *mut wlr_drm_format_set, format: uint32_t,
                              modifier: uint64_t) -> bool;
    #[no_mangle]
    fn wlr_drm_format_set_has(set: *const wlr_drm_format_set,
                              format: uint32_t, modifier: uint64_t) -> bool;
    #[no_mangle]
    fn wlr_drm_format_set_finish(set: *mut wlr_drm_format_set);
    /* *
 * Reference the buffer.
 */
    #[no_mangle]
    fn wlr_buffer_ref(buffer: *mut wlr_buffer) -> *mut wlr_buffer;
    /* *
 * Unreference the buffer. After this call, `buffer` may not be accessed
 * anymore.
 */
    #[no_mangle]
    fn wlr_buffer_unref(buffer: *mut wlr_buffer);
    /* *
 * Reads the DMA-BUF attributes of the buffer. If this buffer isn't a DMA-BUF,
 * returns false.
 */
    #[no_mangle]
    fn wlr_buffer_get_dmabuf(buffer: *mut wlr_buffer,
                             attribs: *mut wlr_dmabuf_attributes) -> bool;
    #[no_mangle]
    fn wlr_output_destroy(output: *mut wlr_output);
    /* *
 * Computes the transformed output resolution.
 */
    #[no_mangle]
    fn wlr_output_transformed_resolution(output: *mut wlr_output,
                                         width: *mut libc::c_int,
                                         height: *mut libc::c_int);
    /* *
 * Returns the transform that, when composed with `tr`, gives
 * `WL_OUTPUT_TRANSFORM_NORMAL`.
 */
    #[no_mangle]
    fn wlr_output_transform_invert(tr: wl_output_transform)
     -> wl_output_transform;
    #[no_mangle]
    fn wlr_output_init(output: *mut wlr_output, backend: *mut wlr_backend,
                       impl_0: *const wlr_output_impl,
                       display: *mut wl_display);
    #[no_mangle]
    fn wlr_output_update_mode(output: *mut wlr_output,
                              mode: *mut wlr_output_mode);
    #[no_mangle]
    fn wlr_output_update_enabled(output: *mut wlr_output, enabled: bool);
    #[no_mangle]
    fn wlr_output_update_needs_frame(output: *mut wlr_output);
    #[no_mangle]
    fn wlr_output_damage_whole(output: *mut wlr_output);
    #[no_mangle]
    fn wlr_output_send_frame(output: *mut wlr_output);
    #[no_mangle]
    fn wlr_output_send_present(output: *mut wlr_output,
                               event: *mut wlr_output_event_present);
    /* *
 * Transforms a box inside a `width` x `height` box.
 */
    #[no_mangle]
    fn wlr_box_transform(dest: *mut wlr_box, box_0: *const wlr_box,
                         transform: wl_output_transform, width: libc::c_int,
                         height: libc::c_int);
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
    fn wlr_texture_get_size(texture: *mut wlr_texture,
                            width: *mut libc::c_int,
                            height: *mut libc::c_int);
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
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn drmGetCap(fd: libc::c_int, capability: uint64_t, value: *mut uint64_t)
     -> libc::c_int;
    #[no_mangle]
    fn drmSetClientCap(fd: libc::c_int, capability: uint64_t, value: uint64_t)
     -> libc::c_int;
    #[no_mangle]
    fn drmHandleEvent(fd: libc::c_int, evctx: drmEventContextPtr)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeFreeResources(ptr: drmModeResPtr);
    #[no_mangle]
    fn drmModeFreeCrtc(ptr: drmModeCrtcPtr);
    #[no_mangle]
    fn drmModeFreeConnector(ptr: drmModeConnectorPtr);
    #[no_mangle]
    fn drmModeFreeEncoder(ptr: drmModeEncoderPtr);
    #[no_mangle]
    fn drmModeFreePlane(ptr: drmModePlanePtr);
    #[no_mangle]
    fn drmModeFreePlaneResources(ptr: drmModePlaneResPtr);
    #[no_mangle]
    fn drmModeGetResources(fd: libc::c_int) -> drmModeResPtr;
    #[no_mangle]
    fn drmModeGetCrtc(fd: libc::c_int, crtcId: uint32_t) -> drmModeCrtcPtr;
    #[no_mangle]
    fn drmModeSetCrtc(fd: libc::c_int, crtcId: uint32_t, bufferId: uint32_t,
                      x: uint32_t, y: uint32_t, connectors: *mut uint32_t,
                      count: libc::c_int, mode: drmModeModeInfoPtr)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeSetCursor(fd: libc::c_int, crtcId: uint32_t,
                        bo_handle: uint32_t, width: uint32_t,
                        height: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn drmModeGetEncoder(fd: libc::c_int, encoder_id: uint32_t)
     -> drmModeEncoderPtr;
    #[no_mangle]
    fn drmModeGetConnector(fd: libc::c_int, connectorId: uint32_t)
     -> drmModeConnectorPtr;
    #[no_mangle]
    fn drmModeGetPropertyBlob(fd: libc::c_int, blob_id: uint32_t)
     -> drmModePropertyBlobPtr;
    #[no_mangle]
    fn drmModeFreePropertyBlob(ptr: drmModePropertyBlobPtr);
    #[no_mangle]
    fn drmModeGetPlaneResources(fd: libc::c_int) -> drmModePlaneResPtr;
    #[no_mangle]
    fn drmModeGetPlane(fd: libc::c_int, plane_id: uint32_t)
     -> drmModePlanePtr;
    #[no_mangle]
    fn drmModeAtomicFree(req: drmModeAtomicReqPtr);
    #[no_mangle]
    fn drmModeDestroyPropertyBlob(fd: libc::c_int, id: uint32_t)
     -> libc::c_int;
    #[no_mangle]
    fn generate_cvt_mode(mode: *mut drmModeModeInfo, hdisplay: libc::c_int,
                         vdisplay: libc::c_int, vrefresh: libc::c_float,
                         reduced: bool, interlaced: bool);
    // DMA-BUF to be displayed on next commit
    // Buffer submitted to the kernel but not yet displayed
    // Buffer currently being displayed
    #[no_mangle]
    fn get_drm_backend_from_backend(wlr_backend: *mut wlr_backend)
     -> *mut wlr_drm_backend;
    #[no_mangle]
    fn import_gbm_bo(renderer: *mut wlr_drm_renderer,
                     attribs: *mut wlr_dmabuf_attributes) -> *mut gbm_bo;
    #[no_mangle]
    fn swap_drm_surface_buffers(surf: *mut wlr_drm_surface,
                                damage: *mut pixman_region32_t)
     -> *mut gbm_bo;
    #[no_mangle]
    fn make_drm_surface_current(surf: *mut wlr_drm_surface,
                                buffer_age: *mut libc::c_int) -> bool;
    #[no_mangle]
    fn finish_drm_surface(surf: *mut wlr_drm_surface);
    #[no_mangle]
    fn init_drm_surface(surf: *mut wlr_drm_surface,
                        renderer: *mut wlr_drm_renderer, width: uint32_t,
                        height: uint32_t, format: uint32_t,
                        set: *mut wlr_drm_format_set, flags: uint32_t)
     -> bool;
    #[no_mangle]
    fn get_drm_surface_front(surf: *mut wlr_drm_surface) -> *mut gbm_bo;
    #[no_mangle]
    fn init_drm_plane_surfaces(plane: *mut wlr_drm_plane,
                               drm: *mut wlr_drm_backend, width: int32_t,
                               height: uint32_t, format: uint32_t,
                               with_modifiers: bool) -> bool;
    #[no_mangle]
    fn copy_drm_surface_mgpu(dest: *mut wlr_drm_surface, src: *mut gbm_bo)
     -> *mut gbm_bo;
    #[no_mangle]
    fn export_drm_bo(bo: *mut gbm_bo, attribs: *mut wlr_dmabuf_attributes)
     -> bool;
    #[no_mangle]
    static atomic_iface: wlr_drm_interface;
    #[no_mangle]
    static legacy_iface: wlr_drm_interface;
    #[no_mangle]
    fn get_drm_connector_props(fd: libc::c_int, id: uint32_t,
                               out: *mut wlr_drm_connector_props) -> bool;
    #[no_mangle]
    fn get_drm_crtc_props(fd: libc::c_int, id: uint32_t,
                          out: *mut wlr_drm_crtc_props) -> bool;
    #[no_mangle]
    fn get_drm_plane_props(fd: libc::c_int, id: uint32_t,
                           out: *mut wlr_drm_plane_props) -> bool;
    #[no_mangle]
    fn get_drm_prop(fd: libc::c_int, obj: uint32_t, prop: uint32_t,
                    ret: *mut uint64_t) -> bool;
    #[no_mangle]
    fn get_drm_prop_blob(fd: libc::c_int, obj: uint32_t, prop: uint32_t,
                         ret_len: *mut size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn post_drm_surface(surf: *mut wlr_drm_surface);
    // Calculates a more accurate refresh rate (mHz) than what mode itself provides
    #[no_mangle]
    fn calculate_refresh_rate(mode: *const drmModeModeInfo) -> int32_t;
    // Populates the make/model/phys_{width,height} of output from the edid data
    #[no_mangle]
    fn parse_edid(output: *mut wlr_output, len: size_t, data: *const uint8_t);
    // Returns the string representation of a DRM output type
    #[no_mangle]
    fn conn_get_name(type_id: uint32_t) -> *const libc::c_char;
    // Returns the DRM framebuffer id for a gbm_bo
    #[no_mangle]
    fn get_fb_for_bo(bo: *mut gbm_bo, drm_format: uint32_t,
                     with_modifiers: bool) -> uint32_t;
    /*
 * Tries to match some DRM objects with some other DRM resource.
 * e.g. Match CRTCs with Encoders, CRTCs with Planes.
 *
 * objs contains a bit array which resources it can be matched with.
 * e.g. Bit 0 set means can be matched with res[0]
 *
 * res contains an index of which objs it is matched with or UNMATCHED.
 *
 * This solution is left in out.
 * Returns the total number of matched solutions.
 */
    #[no_mangle]
    fn match_obj(num_objs: size_t, objs: *const uint32_t, num_res: size_t,
                 res: *const uint32_t, out: *mut uint32_t) -> size_t;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type __u32 = libc::c_uint;
pub type __u64 = libc::c_ulonglong;

#[repr(C)]#[derive(Copy, Clone)]
pub struct drm_format_modifier_blob {
    pub version: __u32,
    pub flags: __u32,
    pub count_formats: __u32,
    pub formats_offset: __u32,
    pub count_modifiers: __u32,
    pub modifiers_offset: __u32,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct drm_format_modifier {
    pub formats: __u64,
    pub offset: __u32,
    pub pad: __u32,
    pub modifier: __u64,
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type gbm_bo_flags = libc::c_uint;
pub const GBM_BO_USE_LINEAR: gbm_bo_flags = 16;
pub const GBM_BO_USE_WRITE: gbm_bo_flags = 8;
pub const GBM_BO_USE_RENDERING: gbm_bo_flags = 4;
pub const GBM_BO_USE_CURSOR_64X64: gbm_bo_flags = 2;
pub const GBM_BO_USE_CURSOR: gbm_bo_flags = 2;
pub const GBM_BO_USE_SCANOUT: gbm_bo_flags = 1;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

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
pub type wl_event_loop_timer_func_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;

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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub cap: size_t,
    pub formats: *mut *mut wlr_drm_format,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_format {
    pub format: uint32_t,
    pub len: size_t,
    pub cap: size_t,
    pub modifiers: [uint64_t; 0],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_connector {
    pub output: wlr_output,
    pub state: wlr_drm_connector_state,
    pub desired_mode: *mut wlr_output_mode,
    pub desired_enabled: bool,
    pub id: uint32_t,
    pub crtc: *mut wlr_drm_crtc,
    pub possible_crtc: uint32_t,
    pub props: wlr_drm_connector_props,
    pub width: uint32_t,
    pub height: uint32_t,
    pub cursor_x: int32_t,
    pub cursor_y: int32_t,
    pub old_crtc: *mut drmModeCrtc,
    pub pageflip_pending: bool,
    pub retry_pageflip: *mut wl_event_source,
    pub link: wl_list,
    pub pending_dmabuf: wlr_dmabuf_attributes,
    pub pending_buffer: *mut wlr_buffer,
    pub pending_bo: *mut gbm_bo,
    pub current_buffer: *mut wlr_buffer,
    pub current_bo: *mut gbm_bo,
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
pub struct wlr_texture {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_texture_impl,
}

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
pub type drmModeCrtc = _drmModeCrtc;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModeCrtc {
    pub crtc_id: uint32_t,
    pub buffer_id: uint32_t,
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
    pub mode_valid: libc::c_int,
    pub mode: drmModeModeInfo,
    pub gamma_size: libc::c_int,
}
/* *
 * Add mode to the list of available modes
 */
pub type drmModeModeInfo = _drmModeModeInfo;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModeModeInfo {
    pub clock: uint32_t,
    pub hdisplay: uint16_t,
    pub hsync_start: uint16_t,
    pub hsync_end: uint16_t,
    pub htotal: uint16_t,
    pub hskew: uint16_t,
    pub vdisplay: uint16_t,
    pub vsync_start: uint16_t,
    pub vsync_end: uint16_t,
    pub vtotal: uint16_t,
    pub vscan: uint16_t,
    pub vrefresh: uint32_t,
    pub flags: uint32_t,
    pub type_0: uint32_t,
    pub name: [libc::c_char; 32],
}
/*
 * These types contain the property ids for several DRM objects.
 * See https://01.org/linuxgraphics/gfx-docs/drm/gpu/drm-kms.html#kms-properties
 * for more details.
 */

#[repr ( C )]#[derive(Copy, Clone)]
pub union wlr_drm_connector_props {
    pub c2rust_unnamed: C2RustUnnamed,
    pub props: [uint32_t; 4],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub edid: uint32_t,
    pub dpms: uint32_t,
    pub link_status: uint32_t,
    pub path: uint32_t,
    pub crtc_id: uint32_t,
}
// ARGB8888 or XRGB8888
// Only used by cursor

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_crtc {
    pub id: uint32_t,
    pub mode_id: uint32_t,
    pub gamma_lut: uint32_t,
    pub atomic: *mut drmModeAtomicReq,
    pub legacy_crtc: *mut drmModeCrtc,
    pub primary: *mut wlr_drm_plane,
    pub cursor: *mut wlr_drm_plane,
    pub num_overlays: size_t,
    pub overlays: *mut uint32_t,
    pub props: wlr_drm_crtc_props,
    pub connectors: wl_list,
    pub gamma_table: *mut uint16_t,
    pub gamma_table_size: size_t,
    // Used to provide atomic or legacy DRM functions
    // Enable or disable DPMS for connector
    // Pageflip on crtc. If mode is non-NULL perform a full modeset using it.
    // Enable the cursor buffer on crtc. Set bo to NULL to disable
    // Move the cursor on crtc
    // Set the gamma lut on crtc
    // Get the gamma lut size of a crtc
}

#[repr ( C )]#[derive(Copy, Clone)]
pub union wlr_drm_crtc_props {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub props: [uint32_t; 6],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub rotation: uint32_t,
    pub scaling_mode: uint32_t,
    pub active: uint32_t,
    pub mode_id: uint32_t,
    pub gamma_lut: uint32_t,
    pub gamma_lut_size: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_plane {
    pub type_0: uint32_t,
    pub id: uint32_t,
    pub surf: wlr_drm_surface,
    pub mgpu_surf: wlr_drm_surface,
    pub drm_format: uint32_t,
    pub formats: wlr_drm_format_set,
    pub matrix: [libc::c_float; 9],
    pub cursor_enabled: bool,
    pub cursor_hotspot_x: int32_t,
    pub cursor_hotspot_y: int32_t,
    pub props: wlr_drm_plane_props,
}

#[repr ( C )]#[derive(Copy, Clone)]
pub union wlr_drm_plane_props {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub props: [uint32_t; 13],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub type_0: uint32_t,
    pub rotation: uint32_t,
    pub in_formats: uint32_t,
    pub src_x: uint32_t,
    pub src_y: uint32_t,
    pub src_w: uint32_t,
    pub src_h: uint32_t,
    pub crtc_x: uint32_t,
    pub crtc_y: uint32_t,
    pub crtc_w: uint32_t,
    pub crtc_h: uint32_t,
    pub fb_id: uint32_t,
    pub crtc_id: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_surface {
    pub renderer: *mut wlr_drm_renderer,
    pub width: uint32_t,
    pub height: uint32_t,
    pub gbm: *mut gbm_surface,
    pub egl: EGLSurface,
    pub front: *mut gbm_bo,
    pub back: *mut gbm_bo,
}
pub type EGLSurface = *mut libc::c_void;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_renderer {
    pub fd: libc::c_int,
    pub gbm: *mut gbm_device,
    pub egl: wlr_egl,
    pub gbm_format: uint32_t,
    pub wlr_rend: *mut wlr_renderer,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * A client buffer.
 */
/* *
	 * The buffer resource, if any. Will be NULL if the client destroys it.
	 */
/* *
	 * The buffer's texture, if any. A buffer will not have a texture if the
	 * client destroys the buffer before it has been released.
	 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_renderer {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_renderer_impl,
    pub events: C2RustUnnamed_2,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_egl {
    pub platform: EGLenum,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
    pub exts_str: *const libc::c_char,
    pub exts: C2RustUnnamed_3,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
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
pub type EGLenum = libc::c_uint;
pub type drmModeAtomicReq = _drmModeAtomicReq;
pub type wlr_drm_connector_state = libc::c_uint;
pub const WLR_DRM_CONN_CONNECTED: wlr_drm_connector_state = 3;
pub const WLR_DRM_CONN_CLEANUP: wlr_drm_connector_state = 2;
// An output just has been plugged in and is waiting for a modeset
pub const WLR_DRM_CONN_NEEDS_MODESET: wlr_drm_connector_state = 1;
// Connector is available but no output is plugged in
pub const WLR_DRM_CONN_DISCONNECTED: wlr_drm_connector_state = 0;
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

#[repr(C)]#[derive(Copy, Clone)]
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
    pub events: C2RustUnnamed_5,
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
    pub surface: *mut crate::src::types::data_device::wlr_data_device::wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_4,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
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
pub type pixman_region32_t = pixman_region32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_data_t = pixman_region32_data;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_box32_t = pixman_box32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
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
pub type wl_output_subpixel = libc::c_uint;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: wl_output_subpixel = 5;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: wl_output_subpixel = 4;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: wl_output_subpixel = 3;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: wl_output_subpixel = 2;
pub const WL_OUTPUT_SUBPIXEL_NONE: wl_output_subpixel = 1;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: wl_output_subpixel = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed_6,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_backend_impl {
    pub start: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> ()>,
    pub get_renderer: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                 -> *mut wlr_renderer>,
    pub get_session: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                -> *mut wlr_session>,
    pub get_presentation_clock: Option<unsafe extern "C" fn(_:
                                                                *mut wlr_backend)
                                           -> clockid_t>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_session {
    pub impl_0: *const crate::src::backend::session::direct::session_impl,
    pub session_signal: wl_signal,
    pub active: bool,
    pub vtnr: libc::c_uint,
    pub seat: [libc::c_char; 256],
    pub udev: *mut udev,
    pub mon: *mut udev_monitor,
    pub udev_event: *mut wl_event_source,
    pub devices: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_7,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub destroy: wl_signal,
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

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type wlr_output_state_field = libc::c_uint;
pub const WLR_OUTPUT_STATE_DAMAGE: wlr_output_state_field = 2;
pub const WLR_OUTPUT_STATE_BUFFER: wlr_output_state_field = 1;
pub type wlr_output_present_flag = libc::c_uint;
// The presentation of this update was done zero-copy.
pub const WLR_OUTPUT_PRESENT_ZERO_COPY: wlr_output_present_flag = 8;
// The display hardware signalled that it started using the new image
	// content.
pub const WLR_OUTPUT_PRESENT_HW_COMPLETION: wlr_output_present_flag = 4;
// The display hardware provided measurements that the hardware driver
	// converted into a presentation timestamp.
pub const WLR_OUTPUT_PRESENT_HW_CLOCK: wlr_output_present_flag = 2;
// The presentation was synchronized to the "vertical retrace" by the
	// display hardware such that tearing does not happen.
pub const WLR_OUTPUT_PRESENT_VSYNC: wlr_output_present_flag = 1;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_event_present {
    pub output: *mut wlr_output,
    pub commit_seq: uint32_t,
    pub when: *mut timespec,
    pub seq: libc::c_uint,
    pub refresh: libc::c_int,
    pub flags: uint32_t,
    // enum wlr_output_present_flag
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmEventContext {
    pub version: libc::c_int,
    pub vblank_handler: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_uint,
                                                    _: libc::c_uint,
                                                    _: libc::c_uint,
                                                    _: *mut libc::c_void)
                                   -> ()>,
    pub page_flip_handler: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_uint,
                                                       _: libc::c_uint,
                                                       _: libc::c_uint,
                                                       _: *mut libc::c_void)
                                      -> ()>,
    pub page_flip_handler2: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_uint,
                                                        _: libc::c_uint,
                                                        _: libc::c_uint,
                                                        _: libc::c_uint,
                                                        _: *mut libc::c_void)
                                       -> ()>,
    pub sequence_handler: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: uint64_t,
                                                      _: uint64_t,
                                                      _: uint64_t) -> ()>,
}
pub type drmEventContext = _drmEventContext;
pub type drmEventContextPtr = *mut _drmEventContext;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModeRes {
    pub count_fbs: libc::c_int,
    pub fbs: *mut uint32_t,
    pub count_crtcs: libc::c_int,
    pub crtcs: *mut uint32_t,
    pub count_connectors: libc::c_int,
    pub connectors: *mut uint32_t,
    pub count_encoders: libc::c_int,
    pub encoders: *mut uint32_t,
    pub min_width: uint32_t,
    pub max_width: uint32_t,
    pub min_height: uint32_t,
    pub max_height: uint32_t,
}
pub type drmModeRes = _drmModeRes;
pub type drmModeResPtr = *mut _drmModeRes;
pub type drmModeModeInfoPtr = *mut _drmModeModeInfo;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModePropertyBlob {
    pub id: uint32_t,
    pub length: uint32_t,
    pub data: *mut libc::c_void,
}
pub type drmModePropertyBlobRes = _drmModePropertyBlob;
pub type drmModePropertyBlobPtr = *mut _drmModePropertyBlob;
pub type drmModeCrtcPtr = *mut _drmModeCrtc;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModeEncoder {
    pub encoder_id: uint32_t,
    pub encoder_type: uint32_t,
    pub crtc_id: uint32_t,
    pub possible_crtcs: uint32_t,
    pub possible_clones: uint32_t,
}
pub type drmModeEncoder = _drmModeEncoder;
pub type drmModeEncoderPtr = *mut _drmModeEncoder;
pub type drmModeConnection = libc::c_uint;
pub const DRM_MODE_UNKNOWNCONNECTION: drmModeConnection = 3;
pub const DRM_MODE_DISCONNECTED: drmModeConnection = 2;
pub const DRM_MODE_CONNECTED: drmModeConnection = 1;
pub type drmModeSubPixel = libc::c_uint;
pub const DRM_MODE_SUBPIXEL_NONE: drmModeSubPixel = 6;
pub const DRM_MODE_SUBPIXEL_VERTICAL_BGR: drmModeSubPixel = 5;
pub const DRM_MODE_SUBPIXEL_VERTICAL_RGB: drmModeSubPixel = 4;
pub const DRM_MODE_SUBPIXEL_HORIZONTAL_BGR: drmModeSubPixel = 3;
pub const DRM_MODE_SUBPIXEL_HORIZONTAL_RGB: drmModeSubPixel = 2;
pub const DRM_MODE_SUBPIXEL_UNKNOWN: drmModeSubPixel = 1;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModeConnector {
    pub connector_id: uint32_t,
    pub encoder_id: uint32_t,
    pub connector_type: uint32_t,
    pub connector_type_id: uint32_t,
    pub connection: drmModeConnection,
    pub mmWidth: uint32_t,
    pub mmHeight: uint32_t,
    pub subpixel: drmModeSubPixel,
    pub count_modes: libc::c_int,
    pub modes: drmModeModeInfoPtr,
    pub count_props: libc::c_int,
    pub props: *mut uint32_t,
    pub prop_values: *mut uint64_t,
    pub count_encoders: libc::c_int,
    pub encoders: *mut uint32_t,
}
pub type drmModeConnector = _drmModeConnector;
pub type drmModeConnectorPtr = *mut _drmModeConnector;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModePlane {
    pub count_formats: uint32_t,
    pub formats: *mut uint32_t,
    pub plane_id: uint32_t,
    pub crtc_id: uint32_t,
    pub fb_id: uint32_t,
    pub crtc_x: uint32_t,
    pub crtc_y: uint32_t,
    pub x: uint32_t,
    pub y: uint32_t,
    pub possible_crtcs: uint32_t,
    pub gamma_size: uint32_t,
}
pub type drmModePlane = _drmModePlane;
pub type drmModePlanePtr = *mut _drmModePlane;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _drmModePlaneRes {
    pub count_planes: uint32_t,
    pub planes: *mut uint32_t,
}
pub type drmModePlaneRes = _drmModePlaneRes;
pub type drmModePlaneResPtr = *mut _drmModePlaneRes;
pub type drmModeAtomicReqPtr = *mut _drmModeAtomicReq;
// Atomic modesetting only
// Legacy only
/*
	 * We don't support overlay planes yet, but we keep track of them to
	 * give to DRM lease clients.
	 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_backend {
    pub backend: wlr_backend,
    pub parent: *mut wlr_drm_backend,
    pub iface: *const wlr_drm_interface,
    pub clock: clockid_t,
    pub addfb2_modifiers: bool,
    pub fd: libc::c_int,
    pub num_crtcs: size_t,
    pub crtcs: *mut wlr_drm_crtc,
    pub display: *mut wl_display,
    pub drm_event: *mut wl_event_source,
    pub display_destroy: wl_listener,
    pub session_destroy: wl_listener,
    pub session_signal: wl_listener,
    pub drm_invalidated: wl_listener,
    pub outputs: wl_list,
    pub renderer: wlr_drm_renderer,
    pub session: *mut wlr_session,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_interface {
    pub conn_enable: Option<unsafe extern "C" fn(_: *mut wlr_drm_backend,
                                                 _: *mut wlr_drm_connector,
                                                 _: bool) -> bool>,
    pub crtc_pageflip: Option<unsafe extern "C" fn(_: *mut wlr_drm_backend,
                                                   _: *mut wlr_drm_connector,
                                                   _: *mut wlr_drm_crtc,
                                                   _: uint32_t,
                                                   _: *mut drmModeModeInfo)
                                  -> bool>,
    pub crtc_set_cursor: Option<unsafe extern "C" fn(_: *mut wlr_drm_backend,
                                                     _: *mut wlr_drm_crtc,
                                                     _: *mut gbm_bo) -> bool>,
    pub crtc_move_cursor: Option<unsafe extern "C" fn(_: *mut wlr_drm_backend,
                                                      _: *mut wlr_drm_crtc,
                                                      _: libc::c_int,
                                                      _: libc::c_int)
                                     -> bool>,
    pub crtc_set_gamma: Option<unsafe extern "C" fn(_: *mut wlr_drm_backend,
                                                    _: *mut wlr_drm_crtc,
                                                    _: size_t,
                                                    _: *mut uint16_t,
                                                    _: *mut uint16_t,
                                                    _: *mut uint16_t)
                                   -> bool>,
    pub crtc_get_gamma_size: Option<unsafe extern "C" fn(_:
                                                             *mut wlr_drm_backend,
                                                         _: *mut wlr_drm_crtc)
                                        -> size_t>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_drm_mode {
    pub wlr_mode: wlr_output_mode,
    pub drm_mode: drmModeModeInfo,
}
pub const UNMATCHED: C2RustUnnamed_8 = 4294967295;
// Part of match_obj
pub type C2RustUnnamed_8 = libc::c_uint;
pub const SKIP: C2RustUnnamed_8 = 4294967294;
#[no_mangle]
pub unsafe extern "C" fn check_drm_features(mut drm: *mut wlr_drm_backend)
 -> bool {
    let mut cap: uint64_t = 0;
    if !(*drm).parent.is_null() {
        if drmGetCap((*drm).fd, 0x5i32 as uint64_t, &mut cap) != 0 ||
               cap & 0x1i32 as libc::c_ulong == 0 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] PRIME import not supported on secondary GPU\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 38i32);
            return 0i32 != 0
        }
        if drmGetCap((*(*drm).parent).fd, 0x5i32 as uint64_t, &mut cap) != 0
               || cap & 0x2i32 as libc::c_ulong == 0 {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] PRIME export not supported on primary GPU\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 45i32);
            return 0i32 != 0
        }
    }
    if drmSetClientCap((*drm).fd, 2i32 as uint64_t, 1i32 as uint64_t) != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] DRM universal planes unsupported\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 51i32);
        return 0i32 != 0
    }
    if drmGetCap((*drm).fd, 0x12i32 as uint64_t, &mut cap) != 0 || cap == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] DRM_CRTC_IN_VBLANK_EVENT unsupported\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 56i32);
        return 0i32 != 0
    }
    let mut no_atomic: *const libc::c_char =
        getenv(b"WLR_DRM_NO_ATOMIC\x00" as *const u8 as *const libc::c_char);
    if !no_atomic.is_null() &&
           strcmp(no_atomic, b"1\x00" as *const u8 as *const libc::c_char) ==
               0i32 {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] WLR_DRM_NO_ATOMIC set, forcing legacy DRM interface\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 63i32);
        (*drm).iface = &legacy_iface
    } else if drmSetClientCap((*drm).fd, 3i32 as uint64_t, 1i32 as uint64_t)
                  != 0 {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Atomic modesetting unsupported, using legacy DRM interface\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 67i32);
        (*drm).iface = &legacy_iface
    } else {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Using atomic DRM interface\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 70i32);
        (*drm).iface = &atomic_iface
    }
    let mut ret: libc::c_int =
        drmGetCap((*drm).fd, 0x6i32 as uint64_t, &mut cap);
    (*drm).clock =
        if ret == 0i32 && cap == 1i32 as libc::c_ulong { 1i32 } else { 0i32 };
    ret = drmGetCap((*drm).fd, 0x10i32 as uint64_t, &mut cap);
    (*drm).addfb2_modifiers = ret == 0i32 && cap == 1i32 as libc::c_ulong;
    return 1i32 != 0;
}
unsafe extern "C" fn add_plane(mut drm: *mut wlr_drm_backend,
                               mut crtc: *mut wlr_drm_crtc,
                               mut drm_plane: *mut drmModePlane,
                               mut type_0: uint32_t,
                               mut props: *mut wlr_drm_plane_props) -> bool {
    let mut current_block: u64;
    if !(type_0 == 1i32 as libc::c_uint && !(*crtc).primary.is_null()) {
    } else {
        __assert_fail(b"!(type == DRM_PLANE_TYPE_PRIMARY && crtc->primary)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../backend/drm/drm.c\x00" as *const u8 as
                          *const libc::c_char, 86i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 120],
                                                &[libc::c_char; 120]>(b"_Bool add_plane(struct wlr_drm_backend *, struct wlr_drm_crtc *, drmModePlane *, uint32_t, union wlr_drm_plane_props *)\x00")).as_ptr());
    };
    if type_0 == 2i32 as libc::c_uint && !(*crtc).cursor.is_null() {
        return 1i32 != 0
    }
    let mut p: *mut wlr_drm_plane =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_drm_plane>() as libc::c_ulong) as
            *mut wlr_drm_plane;
    if p.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 94i32,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    (*p).type_0 = type_0;
    (*p).id = (*drm_plane).plane_id;
    (*p).props = *props;
    let mut j: size_t = 0i32 as size_t;
    while j < (*drm_plane).count_formats as libc::c_ulong {
        wlr_drm_format_set_add(&mut (*p).formats,
                               *(*drm_plane).formats.offset(j as isize),
                               ((0i32 as __u64) << 56i32 |
                                    (1u64 <<
                                         56i32).wrapping_sub(1i32 as
                                                                 libc::c_ulonglong)
                                        & 0xffffffffffffffu64) as uint64_t);
        j = j.wrapping_add(1)
    }
    // Choose an RGB format for the plane
    let mut rgb_format: uint32_t = 0i32 as uint32_t;
    let mut j_0: size_t = 0i32 as size_t;
    while j_0 < (*drm_plane).count_formats as libc::c_ulong {
        let mut fmt: uint32_t = *(*drm_plane).formats.offset(j_0 as isize);
        if fmt ==
               'A' as i32 as __u32 | ('R' as i32 as __u32) << 8i32 |
                   ('2' as i32 as __u32) << 16i32 |
                   ('4' as i32 as __u32) << 24i32 {
            // Prefer formats with alpha channel
            rgb_format = fmt;
            break ;
        } else {
            if fmt ==
                   'X' as i32 as __u32 | ('R' as i32 as __u32) << 8i32 |
                       ('2' as i32 as __u32) << 16i32 |
                       ('4' as i32 as __u32) << 24i32 {
                rgb_format = fmt
            }
            j_0 = j_0.wrapping_add(1)
        }
    }
    (*p).drm_format = rgb_format;
    if (*p).props.c2rust_unnamed.in_formats != 0 {
        let mut blob_id: uint64_t = 0;
        if !get_drm_prop((*drm).fd, (*p).id,
                         (*p).props.c2rust_unnamed.in_formats, &mut blob_id) {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to read IN_FORMATS property\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 125i32);
            current_block = 4800124200219039627;
        } else {
            let mut blob: *mut drmModePropertyBlobRes =
                drmModeGetPropertyBlob((*drm).fd, blob_id as uint32_t);
            if blob.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Failed to read IN_FORMATS blob\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/drm/drm.c\x00" as *const u8 as
                             *const libc::c_char, 131i32);
                current_block = 4800124200219039627;
            } else {
                let mut data: *mut drm_format_modifier_blob =
                    (*blob).data as *mut drm_format_modifier_blob;
                let mut fmts: *mut uint32_t =
                    (data as
                         *mut libc::c_char).offset((*data).formats_offset as
                                                       isize) as
                        *mut uint32_t;
                let mut mods: *mut drm_format_modifier =
                    (data as
                         *mut libc::c_char).offset((*data).modifiers_offset as
                                                       isize) as
                        *mut drm_format_modifier;
                let mut i: uint32_t = 0i32 as uint32_t;
                while i < (*data).count_modifiers {
                    let mut j_1: libc::c_int = 0i32;
                    while j_1 < 64i32 {
                        if (*mods.offset(i as isize)).formats &
                               ((1i32 as uint64_t) << j_1) as
                                   libc::c_ulonglong != 0 {
                            wlr_drm_format_set_add(&mut (*p).formats,
                                                   *fmts.offset((j_1 as
                                                                     libc::c_uint).wrapping_add((*mods.offset(i
                                                                                                                  as
                                                                                                                  isize)).offset)
                                                                    as isize),
                                                   (*mods.offset(i as
                                                                     isize)).modifier
                                                       as uint64_t);
                        }
                        j_1 += 1
                    }
                    i = i.wrapping_add(1)
                }
                drmModeFreePropertyBlob(blob);
                current_block = 15512526488502093901;
            }
        }
        match current_block {
            15512526488502093901 => { }
            _ => { free(p as *mut libc::c_void); return 0i32 != 0 }
        }
    }
    match type_0 {
        1 => { (*crtc).primary = p }
        2 => { (*crtc).cursor = p }
        _ => { abort(); }
    }
    return 1i32 != 0;
}
unsafe extern "C" fn init_planes(mut drm: *mut wlr_drm_backend) -> bool {
    let mut current_block: u64;
    let mut plane_res: *mut drmModePlaneRes =
        drmModeGetPlaneResources((*drm).fd);
    if plane_res.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to get DRM plane resources: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 172i32,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    _wlr_log(WLR_INFO,
             b"[%s:%d] Found %u DRM planes\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             176i32, (*plane_res).count_planes);
    let mut i: uint32_t = 0i32 as uint32_t;
    loop  {
        if !(i < (*plane_res).count_planes) {
            current_block = 4761528863920922185;
            break ;
        }
        let mut id: uint32_t = *(*plane_res).planes.offset(i as isize);
        let mut plane: *mut drmModePlane = drmModeGetPlane((*drm).fd, id);
        if plane.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to get DRM plane: %s\x00" as *const u8
                         as *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 183i32,
                     strerror(*__errno_location()));
            current_block = 12516797604046731087;
            break ;
        } else {
            let mut props: wlr_drm_plane_props =
                wlr_drm_plane_props{c2rust_unnamed:
                                        {
                                            let mut init =
                                                C2RustUnnamed_1{type_0:
                                                                    0i32 as
                                                                        uint32_t,
                                                                rotation: 0,
                                                                in_formats: 0,
                                                                src_x: 0,
                                                                src_y: 0,
                                                                src_w: 0,
                                                                src_h: 0,
                                                                crtc_x: 0,
                                                                crtc_y: 0,
                                                                crtc_w: 0,
                                                                crtc_h: 0,
                                                                fb_id: 0,
                                                                crtc_id: 0,};
                                            init
                                        },};
            if !get_drm_plane_props((*drm).fd, id, &mut props) {
                drmModeFreePlane(plane);
                current_block = 12516797604046731087;
                break ;
            } else {
                let mut type_0: uint64_t = 0;
                if !get_drm_prop((*drm).fd, id, props.c2rust_unnamed.type_0,
                                 &mut type_0) {
                    drmModeFreePlane(plane);
                    current_block = 12516797604046731087;
                    break ;
                } else {
                    /*
		 * This is a very naive implementation of the plane matching
		 * logic. Primary and cursor planes should only work on a
		 * single CRTC, and this should be perfectly adequate, but
		 * overlay planes can potentially work with multiple CRTCs,
		 * meaning this could return inefficient/skewed results.
		 *
		 * However, we don't really care about overlay planes, as we
		 * don't support them yet. We only bother to keep basic
		 * tracking of them for DRM lease clients.
		 *
		 * possible_crtcs is a bitmask of crtcs, where each bit is an
		 * index into drmModeRes.crtcs. So if bit 0 is set (ffs starts
		 * counting from 1), crtc 0 is possible.
		 */
                    let mut crtc_bit: libc::c_int =
                        ffs((*plane).possible_crtcs as libc::c_int) - 1i32;
                    // This would be a kernel bug
                    if crtc_bit >= 0i32 &&
                           (crtc_bit as size_t) < (*drm).num_crtcs {
                    } else {
                        __assert_fail(b"crtc_bit >= 0 && (size_t)crtc_bit < drm->num_crtcs\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"../backend/drm/drm.c\x00" as *const u8
                                          as *const libc::c_char,
                                      217i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 44],
                                                                &[libc::c_char; 44]>(b"_Bool init_planes(struct wlr_drm_backend *)\x00")).as_ptr());
                    };
                    let mut crtc: *mut wlr_drm_crtc =
                        &mut *(*drm).crtcs.offset(crtc_bit as isize) as
                            *mut wlr_drm_crtc;
                    if type_0 == 0i32 as libc::c_ulong {
                        let mut tmp: *mut uint32_t =
                            realloc((*crtc).overlays as *mut libc::c_void,
                                    (::std::mem::size_of::<uint32_t>() as
                                         libc::c_ulong).wrapping_mul((*crtc).num_overlays.wrapping_add(1i32
                                                                                                           as
                                                                                                           libc::c_ulong)))
                                as *mut uint32_t;
                        if !tmp.is_null() {
                            (*crtc).overlays = tmp;
                            let fresh0 = (*crtc).num_overlays;
                            (*crtc).num_overlays =
                                (*crtc).num_overlays.wrapping_add(1);
                            *(*crtc).overlays.offset(fresh0 as isize) = id
                        }
                        drmModeFreePlane(plane);
                    } else if !add_plane(drm, crtc, plane, type_0 as uint32_t,
                                         &mut props) {
                        drmModeFreePlane(plane);
                        current_block = 12516797604046731087;
                        break ;
                    } else { drmModeFreePlane(plane); }
                    i = i.wrapping_add(1)
                }
            }
        }
    }
    match current_block {
        12516797604046731087 => {
            drmModeFreePlaneResources(plane_res);
            return 0i32 != 0
        }
        _ => { drmModeFreePlaneResources(plane_res); return 1i32 != 0 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_drm_resources(mut drm: *mut wlr_drm_backend)
 -> bool {
    let mut res: *mut drmModeRes = drmModeGetResources((*drm).fd);
    if res.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to get DRM resources: %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 251i32,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    _wlr_log(WLR_INFO,
             b"[%s:%d] Found %d DRM CRTCs\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             255i32, (*res).count_crtcs);
    (*drm).num_crtcs = (*res).count_crtcs as size_t;
    if (*drm).num_crtcs == 0i32 as libc::c_ulong {
        drmModeFreeResources(res);
        return 1i32 != 0
    }
    (*drm).crtcs =
        calloc((*drm).num_crtcs,
               ::std::mem::size_of::<wlr_drm_crtc>() as libc::c_ulong) as
            *mut wlr_drm_crtc;
    if (*drm).crtcs.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 265i32,
                 strerror(*__errno_location()));
    } else {
        let mut i: size_t = 0i32 as size_t;
        while i < (*drm).num_crtcs {
            let mut crtc: *mut wlr_drm_crtc =
                &mut *(*drm).crtcs.offset(i as isize) as *mut wlr_drm_crtc;
            (*crtc).id = *(*res).crtcs.offset(i as isize);
            (*crtc).legacy_crtc = drmModeGetCrtc((*drm).fd, (*crtc).id);
            get_drm_crtc_props((*drm).fd, (*crtc).id, &mut (*crtc).props);
            i = i.wrapping_add(1)
        }
        if !init_planes(drm) {
            free((*drm).crtcs as *mut libc::c_void);
        } else { drmModeFreeResources(res); return 1i32 != 0 }
    }
    drmModeFreeResources(res);
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn finish_drm_resources(mut drm: *mut wlr_drm_backend) {
    if drm.is_null() { return }
    let mut i: size_t = 0i32 as size_t;
    while i < (*drm).num_crtcs {
        let mut crtc: *mut wlr_drm_crtc =
            &mut *(*drm).crtcs.offset(i as isize) as *mut wlr_drm_crtc;
        drmModeAtomicFree((*crtc).atomic);
        drmModeFreeCrtc((*crtc).legacy_crtc);
        if (*crtc).mode_id != 0 {
            drmModeDestroyPropertyBlob((*drm).fd, (*crtc).mode_id);
        }
        if (*crtc).gamma_lut != 0 {
            drmModeDestroyPropertyBlob((*drm).fd, (*crtc).gamma_lut);
        }
        free((*crtc).gamma_table as *mut libc::c_void);
        if !(*crtc).primary.is_null() {
            wlr_drm_format_set_finish(&mut (*(*crtc).primary).formats);
            free((*crtc).primary as *mut libc::c_void);
        }
        if !(*crtc).cursor.is_null() {
            wlr_drm_format_set_finish(&mut (*(*crtc).cursor).formats);
            free((*crtc).cursor as *mut libc::c_void);
        }
        free((*crtc).overlays as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free((*drm).crtcs as *mut libc::c_void);
}
unsafe extern "C" fn get_drm_connector_from_output(mut wlr_output:
                                                       *mut wlr_output)
 -> *mut wlr_drm_connector {
    if wlr_output_is_drm(wlr_output) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_output_is_drm(wlr_output)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/drm/drm.c\x00" as *const u8 as
                          *const libc::c_char, 327i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 77],
                                                &[libc::c_char; 77]>(b"struct wlr_drm_connector *get_drm_connector_from_output(struct wlr_output *)\x00")).as_ptr());
    };
    return wlr_output as *mut wlr_drm_connector;
}
unsafe extern "C" fn drm_connector_attach_render(mut output: *mut wlr_output,
                                                 mut buffer_age:
                                                     *mut libc::c_int)
 -> bool {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    return make_drm_surface_current(&mut (*(*(*conn).crtc).primary).surf,
                                    buffer_age);
}
unsafe extern "C" fn drm_connector_commit(mut output: *mut wlr_output)
 -> bool {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*output).backend);
    if !(*(*drm).session).active { return 0i32 != 0 }
    let mut crtc: *mut wlr_drm_crtc = (*conn).crtc;
    if crtc.is_null() { return 0i32 != 0 }
    let mut plane: *mut wlr_drm_plane = (*crtc).primary;
    let mut damage: *mut pixman_region32_t = 0 as *mut pixman_region32_t;
    if (*output).pending.committed &
           WLR_OUTPUT_STATE_DAMAGE as libc::c_int as libc::c_uint != 0 {
        damage = &mut (*output).pending.damage
    }
    let mut bo: *mut gbm_bo = 0 as *mut gbm_bo;
    let mut fb_id: uint32_t = 0i32 as uint32_t;
    if (*output).pending.committed &
           WLR_OUTPUT_STATE_BUFFER as libc::c_int as libc::c_uint != 0 {
    } else {
        __assert_fail(b"output->pending.committed & WLR_OUTPUT_STATE_BUFFER\x00"
                          as *const u8 as *const libc::c_char,
                      b"../backend/drm/drm.c\x00" as *const u8 as
                          *const libc::c_char, 357i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"_Bool drm_connector_commit(struct wlr_output *)\x00")).as_ptr());
    };
    match (*output).pending.buffer_type as libc::c_uint {
        0 => {
            bo = swap_drm_surface_buffers(&mut (*plane).surf, damage);
            if bo.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] swap_drm_surface_buffers failed\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/drm/drm.c\x00" as *const u8 as
                             *const libc::c_char, 362i32);
                return 0i32 != 0
            }
            if !(*drm).parent.is_null() {
                bo = copy_drm_surface_mgpu(&mut (*plane).mgpu_surf, bo);
                if bo.is_null() {
                    _wlr_log(WLR_ERROR,
                             b"[%s:%d] copy_drm_surface_mgpu failed\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../backend/drm/drm.c\x00" as *const u8 as
                                 *const libc::c_char, 369i32);
                    return 0i32 != 0
                }
            }
            fb_id =
                get_fb_for_bo(bo, (*plane).drm_format,
                              (*drm).addfb2_modifiers);
            if fb_id == 0i32 as libc::c_uint {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] get_fb_for_bo failed\x00" as *const u8 as
                             *const libc::c_char,
                         b"../backend/drm/drm.c\x00" as *const u8 as
                             *const libc::c_char, 375i32);
                return 0i32 != 0
            }
        }
        1 => {
            bo =
                import_gbm_bo(&mut (*drm).renderer,
                              &mut (*conn).pending_dmabuf);
            if bo.is_null() {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] import_gbm_bo failed\x00" as *const u8 as
                             *const libc::c_char,
                         b"../backend/drm/drm.c\x00" as *const u8 as
                             *const libc::c_char, 382i32);
                return 0i32 != 0
            }
            if !(*conn).pending_bo.is_null() {
                gbm_bo_destroy((*conn).pending_bo);
            }
            (*conn).pending_bo = bo;
            fb_id =
                get_fb_for_bo(bo, gbm_bo_get_format(bo),
                              (*drm).addfb2_modifiers);
            if fb_id == 0i32 as libc::c_uint {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] get_fb_for_bo failed\x00" as *const u8 as
                             *const libc::c_char,
                         b"../backend/drm/drm.c\x00" as *const u8 as
                             *const libc::c_char, 393i32);
                return 0i32 != 0
            }
        }
        _ => { }
    }
    if (*conn).pageflip_pending {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Skipping pageflip on output \'%s\'\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 400i32,
                 (*conn).output.name.as_mut_ptr());
        return 0i32 != 0
    }
    if !(*(*drm).iface).crtc_pageflip.expect("non-null function pointer")(drm,
                                                                          conn,
                                                                          crtc,
                                                                          fb_id,
                                                                          0 as
                                                                              *mut drmModeModeInfo)
       {
        return 0i32 != 0
    }
    (*conn).pageflip_pending = 1i32 != 0;
    if (*output).pending.buffer_type as libc::c_uint ==
           WLR_OUTPUT_STATE_BUFFER_SCANOUT as libc::c_int as libc::c_uint {
        wlr_buffer_unref((*conn).pending_buffer);
        (*conn).pending_buffer = wlr_buffer_ref((*output).pending.buffer)
    }
    wlr_output_update_enabled(output, 1i32 != 0);
    return 1i32 != 0;
}
unsafe extern "C" fn fill_empty_gamma_table(mut size: size_t,
                                            mut r: *mut uint16_t,
                                            mut g: *mut uint16_t,
                                            mut b: *mut uint16_t) {
    let mut i: uint32_t = 0i32 as uint32_t;
    while (i as libc::c_ulong) < size {
        let mut val: uint16_t =
            ((0xffffi32 as uint32_t).wrapping_mul(i) as
                 libc::c_ulong).wrapping_div(size.wrapping_sub(1i32 as
                                                                   libc::c_ulong))
                as uint16_t;
        let ref mut fresh1 = *b.offset(i as isize);
        *fresh1 = val;
        let ref mut fresh2 = *g.offset(i as isize);
        *fresh2 = *fresh1;
        *r.offset(i as isize) = *fresh2;
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn drm_connector_get_gamma_size(mut output: *mut wlr_output)
 -> size_t {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*output).backend);
    if !(*conn).crtc.is_null() {
        return (*(*drm).iface).crtc_get_gamma_size.expect("non-null function pointer")(drm,
                                                                                       (*conn).crtc)
    }
    return 0i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn set_drm_connector_gamma(mut output: *mut wlr_output,
                                                 mut size: size_t,
                                                 mut r: *const uint16_t,
                                                 mut g: *const uint16_t,
                                                 mut b: *const uint16_t)
 -> bool {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*output).backend);
    if (*conn).crtc.is_null() { return 0i32 != 0 }
    let mut reset: bool = 0i32 != 0;
    if size == 0i32 as libc::c_ulong {
        reset = 1i32 != 0;
        size = drm_connector_get_gamma_size(output);
        if size == 0i32 as libc::c_ulong { return 0i32 != 0 }
    }
    let mut gamma_table: *mut uint16_t =
        malloc((3i32 as
                    libc::c_ulong).wrapping_mul(size).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                       as
                                                                       libc::c_ulong))
            as *mut uint16_t;
    if gamma_table.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate gamma table\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 457i32);
        return 0i32 != 0
    }
    let mut _r: *mut uint16_t = gamma_table;
    let mut _g: *mut uint16_t = gamma_table.offset(size as isize);
    let mut _b: *mut uint16_t =
        gamma_table.offset((2i32 as libc::c_ulong).wrapping_mul(size) as
                               isize);
    if reset {
        fill_empty_gamma_table(size, _r, _g, _b);
    } else {
        memcpy(_r as *mut libc::c_void, r as *const libc::c_void,
               size.wrapping_mul(::std::mem::size_of::<uint16_t>() as
                                     libc::c_ulong));
        memcpy(_g as *mut libc::c_void, g as *const libc::c_void,
               size.wrapping_mul(::std::mem::size_of::<uint16_t>() as
                                     libc::c_ulong));
        memcpy(_b as *mut libc::c_void, b as *const libc::c_void,
               size.wrapping_mul(::std::mem::size_of::<uint16_t>() as
                                     libc::c_ulong));
    }
    let mut ok: bool =
        (*(*drm).iface).crtc_set_gamma.expect("non-null function pointer")(drm,
                                                                           (*conn).crtc,
                                                                           size,
                                                                           _r,
                                                                           _g,
                                                                           _b);
    if ok {
        wlr_output_update_needs_frame(output);
        free((*(*conn).crtc).gamma_table as *mut libc::c_void);
        (*(*conn).crtc).gamma_table = gamma_table;
        (*(*conn).crtc).gamma_table_size = size
    } else { free(gamma_table as *mut libc::c_void); }
    return ok;
}
unsafe extern "C" fn drm_connector_export_dmabuf(mut output: *mut wlr_output,
                                                 mut attribs:
                                                     *mut wlr_dmabuf_attributes)
 -> bool {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*output).backend);
    if !(*(*drm).session).active { return 0i32 != 0 }
    let mut crtc: *mut wlr_drm_crtc = (*conn).crtc;
    if crtc.is_null() { return 0i32 != 0 }
    let mut plane: *mut wlr_drm_plane = (*crtc).primary;
    let mut surf: *mut wlr_drm_surface = &mut (*plane).surf;
    return export_drm_bo((*surf).back, attribs);
}
unsafe extern "C" fn drm_connector_pageflip_renderer(mut conn:
                                                         *mut wlr_drm_connector,
                                                     mut mode:
                                                         *mut wlr_drm_mode)
 -> bool {
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*conn).output.backend);
    let mut crtc: *mut wlr_drm_crtc = (*conn).crtc;
    if crtc.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Page-flip failed on connector \'%s\': no CRTC\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 511i32,
                 (*conn).output.name.as_mut_ptr());
        return 0i32 != 0
    }
    let mut plane: *mut wlr_drm_plane = (*crtc).primary;
    let mut bo: *mut gbm_bo =
        get_drm_surface_front(if !(*drm).parent.is_null() {
                                  &mut (*plane).mgpu_surf
                              } else { &mut (*plane).surf });
    let mut fb_id: uint32_t =
        get_fb_for_bo(bo, (*plane).drm_format, (*drm).addfb2_modifiers);
    return (*(*drm).iface).crtc_pageflip.expect("non-null function pointer")(drm,
                                                                             conn,
                                                                             crtc,
                                                                             fb_id,
                                                                             &mut (*mode).drm_mode);
}
unsafe extern "C" fn drm_connector_start_renderer(mut conn:
                                                      *mut wlr_drm_connector) {
    if (*conn).state as libc::c_uint !=
           WLR_DRM_CONN_CONNECTED as libc::c_int as libc::c_uint {
        return
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Starting renderer on output \'%s\'\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             527i32, (*conn).output.name.as_mut_ptr());
    let mut mode: *mut wlr_drm_mode =
        (*conn).output.current_mode as *mut wlr_drm_mode;
    if drm_connector_pageflip_renderer(conn, mode) {
        (*conn).pageflip_pending = 1i32 != 0;
        wlr_output_update_enabled(&mut (*conn).output, 1i32 != 0);
    } else {
        wl_event_source_timer_update((*conn).retry_pageflip,
                                     (1000000.0f32 /
                                          (*(*conn).output.current_mode).refresh
                                              as libc::c_float) as
                                         libc::c_int);
    };
}
unsafe extern "C" fn drm_connector_init_renderer(mut conn:
                                                     *mut wlr_drm_connector,
                                                 mut mode: *mut wlr_drm_mode)
 -> bool {
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*conn).output.backend);
    if (*conn).state as libc::c_uint !=
           WLR_DRM_CONN_CONNECTED as libc::c_int as libc::c_uint &&
           (*conn).state as libc::c_uint !=
               WLR_DRM_CONN_NEEDS_MODESET as libc::c_int as libc::c_uint {
        return 0i32 != 0
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Initializing renderer on connector \'%s\'\x00" as
                 *const u8 as *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             550i32, (*conn).output.name.as_mut_ptr());
    let mut crtc: *mut wlr_drm_crtc = (*conn).crtc;
    if crtc.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to initialize renderer on connector \'%s\': no CRTC\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 555i32,
                 (*conn).output.name.as_mut_ptr());
        return 0i32 != 0
    }
    let mut plane: *mut wlr_drm_plane = (*crtc).primary;
    let mut width: libc::c_int = (*mode).wlr_mode.width;
    let mut height: libc::c_int = (*mode).wlr_mode.height;
    let mut format: uint32_t = (*drm).renderer.gbm_format;
    if !init_drm_plane_surfaces(plane, drm, width, height as uint32_t, format,
                                1i32 != 0) ||
           !drm_connector_pageflip_renderer(conn, mode) {
        // If page-flipping with modifiers enabled doesn't work, retry without
		// modifiers
        _wlr_log(WLR_INFO,
                 b"[%s:%d] Page-flip failed with primary FB modifiers enabled, retrying without modifiers\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 569i32);
        finish_drm_surface(&mut (*plane).surf);
        finish_drm_surface(&mut (*plane).mgpu_surf);
        if !init_drm_plane_surfaces(plane, drm, width, height as uint32_t,
                                    format, 0i32 != 0) {
            return 0i32 != 0
        }
        if !drm_connector_pageflip_renderer(conn, mode) {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to initialize renderer on connector \'%s\': initial page-flip failed\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 578i32,
                     (*conn).output.name.as_mut_ptr());
            return 0i32 != 0
        }
    }
    return 1i32 != 0;
}
unsafe extern "C" fn attempt_enable_needs_modeset(mut drm:
                                                      *mut wlr_drm_backend) {
    // Try to modeset any output that has a desired mode and a CRTC (ie. was
	// lacking a CRTC on last modeset)
    let mut conn: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
    conn =
        ((*drm).outputs.next as *mut libc::c_char).offset(-688) as
            *mut wlr_drm_connector;
    while &mut (*conn).link as *mut wl_list !=
              &mut (*drm).outputs as *mut wl_list {
        if (*conn).state as libc::c_uint ==
               WLR_DRM_CONN_NEEDS_MODESET as libc::c_int as libc::c_uint &&
               !(*conn).crtc.is_null() && !(*conn).desired_mode.is_null() &&
               (*conn).desired_enabled as libc::c_int != 0 {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] Output %s has a desired mode and a CRTC, attempting a modeset\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 597i32,
                     (*conn).output.name.as_mut_ptr());
            drm_connector_set_mode(&mut (*conn).output, (*conn).desired_mode);
        }
        conn =
            ((*conn).link.next as *mut libc::c_char).offset(-688) as
                *mut wlr_drm_connector
    };
}
#[no_mangle]
pub unsafe extern "C" fn enable_drm_connector(mut output: *mut wlr_output,
                                              mut enable: bool) -> bool {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*output).backend);
    if (*conn).state as libc::c_uint !=
           WLR_DRM_CONN_CONNECTED as libc::c_int as libc::c_uint &&
           (*conn).state as libc::c_uint !=
               WLR_DRM_CONN_NEEDS_MODESET as libc::c_int as libc::c_uint {
        return 0i32 != 0
    }
    (*conn).desired_enabled = enable;
    if enable as libc::c_int != 0 && (*conn).crtc.is_null() {
        // Maybe we can steal a CRTC from a disabled output
        realloc_crtcs(drm);
    }
    let mut ok: bool =
        (*(*drm).iface).conn_enable.expect("non-null function pointer")(drm,
                                                                        conn,
                                                                        enable);
    if !ok { return 0i32 != 0 }
    if enable {
        drm_connector_start_renderer(conn);
    } else { realloc_crtcs(drm); attempt_enable_needs_modeset(drm); }
    wlr_output_update_enabled(&mut (*conn).output, enable);
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn drm_connector_set_mode(mut output: *mut wlr_output,
                                                mut wlr_mode:
                                                    *mut wlr_output_mode)
 -> bool {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*output).backend);
    if (*conn).crtc.is_null() {
        // Maybe we can steal a CRTC from a disabled output
        realloc_crtcs(drm);
    }
    if (*conn).crtc.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot modeset \'%s\': no CRTC for this connector\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 647i32,
                 (*conn).output.name.as_mut_ptr());
        // Save the desired mode for later, when we'll get a proper CRTC
        (*conn).desired_mode = wlr_mode;
        return 0i32 != 0
    }
    _wlr_log(WLR_INFO,
             b"[%s:%d] Modesetting \'%s\' with \'%ux%u@%u mHz\'\x00" as
                 *const u8 as *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             655i32, (*conn).output.name.as_mut_ptr(), (*wlr_mode).width,
             (*wlr_mode).height, (*wlr_mode).refresh);
    let mut mode: *mut wlr_drm_mode = wlr_mode as *mut wlr_drm_mode;
    if !drm_connector_init_renderer(conn, mode) {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to initialize renderer for plane\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 659i32);
        return 0i32 != 0
    }
    (*conn).state = WLR_DRM_CONN_CONNECTED;
    (*conn).desired_mode = 0 as *mut wlr_output_mode;
    wlr_output_update_mode(&mut (*conn).output, wlr_mode);
    wlr_output_update_enabled(&mut (*conn).output, 1i32 != 0);
    (*conn).desired_enabled = 1i32 != 0;
    // When switching VTs, the mode is not updated but the buffers become
	// invalid, so we need to manually damage the output here
    wlr_output_damage_whole(&mut (*conn).output);
    return 1i32 != 0;
}
unsafe extern "C" fn drm_connector_set_custom_mode(mut output:
                                                       *mut wlr_output,
                                                   mut width: int32_t,
                                                   mut height: int32_t,
                                                   mut refresh: int32_t)
 -> bool {
    let mut mode: drmModeModeInfo =
        {
            let mut init =
                _drmModeModeInfo{clock: 0i32 as uint32_t,
                                 hdisplay: 0,
                                 hsync_start: 0,
                                 hsync_end: 0,
                                 htotal: 0,
                                 hskew: 0,
                                 vdisplay: 0,
                                 vsync_start: 0,
                                 vsync_end: 0,
                                 vtotal: 0,
                                 vscan: 0,
                                 vrefresh: 0,
                                 flags: 0,
                                 type_0: 0,
                                 name: [0; 32],};
            init
        };
    generate_cvt_mode(&mut mode, width, height,
                      refresh as libc::c_float / 1000i32 as libc::c_float,
                      0i32 != 0, 0i32 != 0);
    mode.type_0 = (1i32 << 5i32) as uint32_t;
    let mut wlr_mode: *mut wlr_output_mode =
        wlr_drm_connector_add_mode(output, &mut mode);
    if wlr_mode.is_null() { return 0i32 != 0 }
    return drm_connector_set_mode(output, wlr_mode);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_drm_connector_add_mode(mut output:
                                                        *mut wlr_output,
                                                    mut modeinfo:
                                                        *const drmModeModeInfo)
 -> *mut wlr_output_mode {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    if (*modeinfo).type_0 != (1i32 << 5i32) as libc::c_uint {
        return 0 as *mut wlr_output_mode
    }
    let mut wlr_mode: *mut wlr_output_mode = 0 as *mut wlr_output_mode;
    wlr_mode =
        ((*conn).output.modes.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_mode;
    while &mut (*wlr_mode).link as *mut wl_list !=
              &mut (*conn).output.modes as *mut wl_list {
        let mut mode: *mut wlr_drm_mode = wlr_mode as *mut wlr_drm_mode;
        if memcmp(&mut (*mode).drm_mode as *mut drmModeModeInfo as
                      *const libc::c_void, modeinfo as *const libc::c_void,
                  ::std::mem::size_of::<drmModeModeInfo>() as libc::c_ulong)
               == 0i32 {
            return wlr_mode
        }
        wlr_mode =
            ((*wlr_mode).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_mode
    }
    let mut mode_0: *mut wlr_drm_mode =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_drm_mode>() as libc::c_ulong) as
            *mut wlr_drm_mode;
    if mode_0.is_null() { return 0 as *mut wlr_output_mode }
    memcpy(&mut (*mode_0).drm_mode as *mut drmModeModeInfo as
               *mut libc::c_void, modeinfo as *const libc::c_void,
           ::std::mem::size_of::<drmModeModeInfo>() as libc::c_ulong);
    (*mode_0).wlr_mode.width = (*mode_0).drm_mode.hdisplay as int32_t;
    (*mode_0).wlr_mode.height = (*mode_0).drm_mode.vdisplay as int32_t;
    (*mode_0).wlr_mode.refresh = calculate_refresh_rate(modeinfo);
    _wlr_log(WLR_INFO,
             b"[%s:%d] Registered custom mode %dx%d@%d\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             719i32, (*mode_0).wlr_mode.width, (*mode_0).wlr_mode.height,
             (*mode_0).wlr_mode.refresh);
    wl_list_insert(&mut (*conn).output.modes, &mut (*mode_0).wlr_mode.link);
    return &mut (*mode_0).wlr_mode;
}
unsafe extern "C" fn drm_connector_set_cursor(mut output: *mut wlr_output,
                                              mut texture: *mut wlr_texture,
                                              mut scale: int32_t,
                                              mut transform:
                                                  wl_output_transform,
                                              mut hotspot_x: int32_t,
                                              mut hotspot_y: int32_t,
                                              mut update_texture: bool)
 -> bool {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*output).backend);
    let mut crtc: *mut wlr_drm_crtc = (*conn).crtc;
    if crtc.is_null() { return 0i32 != 0 }
    let mut plane: *mut wlr_drm_plane = (*crtc).cursor;
    if plane.is_null() {
        // We don't have a real cursor plane, so we make a fake one
        plane =
            calloc(1i32 as libc::c_ulong,
                   ::std::mem::size_of::<wlr_drm_plane>() as libc::c_ulong) as
                *mut wlr_drm_plane;
        if plane.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Allocation failed: %s\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 742i32,
                     strerror(*__errno_location()));
            return 0i32 != 0
        }
        (*crtc).cursor = plane
    }
    if (*plane).surf.gbm.is_null() {
        let mut ret: libc::c_int = 0;
        let mut w: uint64_t = 0;
        let mut h: uint64_t = 0;
        ret = drmGetCap((*drm).fd, 0x8i32 as uint64_t, &mut w);
        w = if ret != 0 { 64i32 as libc::c_ulong } else { w };
        ret = drmGetCap((*drm).fd, 0x9i32 as uint64_t, &mut h);
        h = if ret != 0 { 64i32 as libc::c_ulong } else { h };
        if (*drm).parent.is_null() {
            if !init_drm_surface(&mut (*plane).surf, &mut (*drm).renderer,
                                 w as uint32_t, h as uint32_t,
                                 (*drm).renderer.gbm_format,
                                 0 as *mut wlr_drm_format_set,
                                 (GBM_BO_USE_LINEAR as libc::c_int |
                                      GBM_BO_USE_SCANOUT as libc::c_int) as
                                     uint32_t) {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Cannot allocate cursor resources\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/drm/drm.c\x00" as *const u8 as
                             *const libc::c_char, 760i32);
                return 0i32 != 0
            }
        } else {
            if !init_drm_surface(&mut (*plane).surf,
                                 &mut (*(*drm).parent).renderer,
                                 w as uint32_t, h as uint32_t,
                                 (*(*drm).parent).renderer.gbm_format,
                                 0 as *mut wlr_drm_format_set,
                                 GBM_BO_USE_LINEAR as libc::c_int as uint32_t)
               {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Cannot allocate cursor resources\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/drm/drm.c\x00" as *const u8 as
                             *const libc::c_char, 767i32);
                return 0i32 != 0
            }
            if !init_drm_surface(&mut (*plane).mgpu_surf,
                                 &mut (*drm).renderer, w as uint32_t,
                                 h as uint32_t, (*drm).renderer.gbm_format,
                                 0 as *mut wlr_drm_format_set,
                                 (GBM_BO_USE_LINEAR as libc::c_int |
                                      GBM_BO_USE_SCANOUT as libc::c_int) as
                                     uint32_t) {
                _wlr_log(WLR_ERROR,
                         b"[%s:%d] Cannot allocate cursor resources\x00" as
                             *const u8 as *const libc::c_char,
                         b"../backend/drm/drm.c\x00" as *const u8 as
                             *const libc::c_char, 774i32);
                return 0i32 != 0
            }
        }
    }
    wlr_matrix_projection((*plane).matrix.as_mut_ptr(),
                          (*plane).surf.width as libc::c_int,
                          (*plane).surf.height as libc::c_int,
                          (*output).transform);
    let mut hotspot: wlr_box =
        {
            let mut init =
                wlr_box{x: hotspot_x, y: hotspot_y, width: 0, height: 0,};
            init
        };
    wlr_box_transform(&mut hotspot, &mut hotspot,
                      wlr_output_transform_invert((*output).transform),
                      (*plane).surf.width as libc::c_int,
                      (*plane).surf.height as libc::c_int);
    if (*plane).cursor_hotspot_x != hotspot.x ||
           (*plane).cursor_hotspot_y != hotspot.y {
        // Update cursor hotspot
        (*conn).cursor_x -= hotspot.x - (*plane).cursor_hotspot_x;
        (*conn).cursor_y -= hotspot.y - (*plane).cursor_hotspot_y;
        (*plane).cursor_hotspot_x = hotspot.x;
        (*plane).cursor_hotspot_y = hotspot.y;
        if !(*(*drm).iface).crtc_move_cursor.expect("non-null function pointer")(drm,
                                                                                 (*conn).crtc,
                                                                                 (*conn).cursor_x,
                                                                                 (*conn).cursor_y)
           {
            return 0i32 != 0
        }
        wlr_output_update_needs_frame(output);
    }
    if !update_texture {
        // Don't update cursor image
        return 1i32 != 0
    }
    (*plane).cursor_enabled = 0i32 != 0;
    if !texture.is_null() {
        let mut width: libc::c_int = 0;
        let mut height: libc::c_int = 0;
        wlr_texture_get_size(texture, &mut width, &mut height);
        width =
            (width as libc::c_float * (*output).scale /
                 scale as libc::c_float) as libc::c_int;
        height =
            (height as libc::c_float * (*output).scale /
                 scale as libc::c_float) as libc::c_int;
        if width > (*plane).surf.width as libc::c_int ||
               height > (*plane).surf.height as libc::c_int {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Cursor too large (max %dx%d)\x00" as *const u8
                         as *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 818i32,
                     (*plane).surf.width as libc::c_int,
                     (*plane).surf.height as libc::c_int);
            return 0i32 != 0
        }
        make_drm_surface_current(&mut (*plane).surf, 0 as *mut libc::c_int);
        let mut rend: *mut wlr_renderer = (*(*plane).surf.renderer).wlr_rend;
        let mut cursor_box: wlr_box =
            {
                let mut init =
                    wlr_box{x: 0, y: 0, width: width, height: height,};
                init
            };
        let mut matrix: [libc::c_float; 9] = [0.; 9];
        wlr_matrix_project_box(matrix.as_mut_ptr(), &mut cursor_box,
                               transform, 0i32 as libc::c_float,
                               (*plane).matrix.as_mut_ptr() as
                                   *const libc::c_float);
        wlr_renderer_begin(rend, (*plane).surf.width as libc::c_int,
                           (*plane).surf.height as libc::c_int);
        wlr_renderer_clear(rend,
                           [0.0f64 as libc::c_float, 0.0f64 as libc::c_float,
                            0.0f64 as libc::c_float,
                            0.0f64 as libc::c_float].as_mut_ptr() as
                               *const libc::c_float);
        wlr_render_texture_with_matrix(rend, texture,
                                       matrix.as_mut_ptr() as
                                           *const libc::c_float,
                                       1.0f64 as libc::c_float);
        wlr_renderer_end(rend);
        swap_drm_surface_buffers(&mut (*plane).surf,
                                 0 as *mut pixman_region32_t);
        (*plane).cursor_enabled = 1i32 != 0
    }
    if !(*(*drm).session).active {
        return 1i32 != 0
        // will be committed when session is resumed
    }
    let mut bo: *mut gbm_bo =
        if (*plane).cursor_enabled as libc::c_int != 0 {
            (*plane).surf.back
        } else { 0 as *mut gbm_bo };
    if !bo.is_null() && !(*drm).parent.is_null() {
        bo = copy_drm_surface_mgpu(&mut (*plane).mgpu_surf, bo)
    }
    if !bo.is_null() {
        // workaround for nouveau
		// Buffers created with GBM_BO_USER_LINEAR are placed in NOUVEAU_GEM_DOMAIN_GART.
		// When the bo is attached to the cursor plane it is moved to NOUVEAU_GEM_DOMAIN_VRAM.
		// However, this does not wait for the render operations to complete, leaving an empty surface.
		// see https://bugs.freedesktop.org/show_bug.cgi?id=109631
		// The render operations can be waited for using:
        glFinish();
    }
    let mut ok: bool =
        (*(*drm).iface).crtc_set_cursor.expect("non-null function pointer")(drm,
                                                                            crtc,
                                                                            bo);
    if ok { wlr_output_update_needs_frame(output); }
    return ok;
}
unsafe extern "C" fn drm_connector_move_cursor(mut output: *mut wlr_output,
                                               mut x: libc::c_int,
                                               mut y: libc::c_int) -> bool {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*output).backend);
    if (*conn).crtc.is_null() { return 0i32 != 0 }
    let mut plane: *mut wlr_drm_plane = (*(*conn).crtc).cursor;
    let mut box_0: wlr_box =
        { let mut init = wlr_box{x: x, y: y, width: 0, height: 0,}; init };
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    wlr_output_transformed_resolution(output, &mut width, &mut height);
    let mut transform: wl_output_transform =
        wlr_output_transform_invert((*output).transform);
    wlr_box_transform(&mut box_0, &mut box_0, transform, width, height);
    if !plane.is_null() {
        box_0.x -= (*plane).cursor_hotspot_x;
        box_0.y -= (*plane).cursor_hotspot_y
    }
    (*conn).cursor_x = box_0.x;
    (*conn).cursor_y = box_0.y;
    if !(*(*drm).session).active {
        return 1i32 != 0
        // will be committed when session is resumed
    }
    let mut ok: bool =
        (*(*drm).iface).crtc_move_cursor.expect("non-null function pointer")(drm,
                                                                             (*conn).crtc,
                                                                             box_0.x,
                                                                             box_0.y);
    if ok { wlr_output_update_needs_frame(output); }
    return ok;
}
unsafe extern "C" fn drm_connector_schedule_frame(mut output: *mut wlr_output)
 -> bool {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*output).backend);
    if !(*(*drm).session).active { return 0i32 != 0 }
    // We need to figure out where we are in the vblank cycle
	// TODO: try using drmWaitVBlank and fallback to pageflipping
    let mut crtc: *mut wlr_drm_crtc = (*conn).crtc;
    if crtc.is_null() { return 0i32 != 0 }
    let mut plane: *mut wlr_drm_plane = (*crtc).primary;
    let mut bo: *mut gbm_bo = (*plane).surf.back;
    if bo.is_null() {
        // We haven't swapped buffers yet -- can't do a pageflip
        wlr_output_send_frame(output);
        return 1i32 != 0
    }
    if !(*drm).parent.is_null() {
        bo = copy_drm_surface_mgpu(&mut (*plane).mgpu_surf, bo)
    }
    if (*conn).pageflip_pending {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Skipping pageflip on output \'%s\'\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 930i32,
                 (*conn).output.name.as_mut_ptr());
        return 1i32 != 0
    }
    let mut fb_id: uint32_t =
        get_fb_for_bo(bo, (*plane).drm_format, (*drm).addfb2_modifiers);
    if !(*(*drm).iface).crtc_pageflip.expect("non-null function pointer")(drm,
                                                                          conn,
                                                                          crtc,
                                                                          fb_id,
                                                                          0 as
                                                                              *mut drmModeModeInfo)
       {
        return 0i32 != 0
    }
    (*conn).pageflip_pending = 1i32 != 0;
    wlr_output_update_enabled(output, 1i32 != 0);
    return 1i32 != 0;
}
unsafe extern "C" fn strip_alpha_channel(mut format: uint32_t) -> uint32_t {
    match format {
        875713089 => {
            return 'X' as i32 as __u32 | ('R' as i32 as __u32) << 8i32 |
                       ('2' as i32 as __u32) << 16i32 |
                       ('4' as i32 as __u32) << 24i32
        }
        _ => { return 0i32 as uint32_t }
    };
}
unsafe extern "C" fn drm_connector_attach_buffer(mut output: *mut wlr_output,
                                                 mut buffer: *mut wlr_buffer)
 -> bool {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*output).backend);
    if !(*(*drm).session).active { return 0i32 != 0 }
    let mut crtc: *mut wlr_drm_crtc = (*conn).crtc;
    if crtc.is_null() { return 0i32 != 0 }
    let mut attribs: wlr_dmabuf_attributes =
        wlr_dmabuf_attributes{width: 0,
                              height: 0,
                              format: 0,
                              flags: 0,
                              modifier: 0,
                              n_planes: 0,
                              offset: [0; 4],
                              stride: [0; 4],
                              fd: [0; 4],};
    if !wlr_buffer_get_dmabuf(buffer, &mut attribs) { return 0i32 != 0 }
    if attribs.flags != 0i32 as libc::c_uint { return 0i32 != 0 }
    if attribs.width != (*output).width || attribs.height != (*output).height
       {
        return 0i32 != 0
    }
    if !wlr_drm_format_set_has(&mut (*(*crtc).primary).formats,
                               attribs.format, attribs.modifier) {
        // The format isn't supported by the plane. Try stripping the alpha
		// channel, if any.
        let mut format: uint32_t = strip_alpha_channel(attribs.format);
        if format != 0i32 as libc::c_uint &&
               wlr_drm_format_set_has(&mut (*(*crtc).primary).formats, format,
                                      attribs.modifier) as libc::c_int != 0 {
            attribs.format = format
        } else { return 0i32 != 0 }
    }
    memcpy(&mut (*conn).pending_dmabuf as *mut wlr_dmabuf_attributes as
               *mut libc::c_void,
           &mut attribs as *mut wlr_dmabuf_attributes as *const libc::c_void,
           ::std::mem::size_of::<wlr_dmabuf_attributes>() as libc::c_ulong);
    return 1i32 != 0;
}
unsafe extern "C" fn drm_connector_destroy(mut output: *mut wlr_output) {
    let mut conn: *mut wlr_drm_connector =
        get_drm_connector_from_output(output);
    drm_connector_cleanup(conn);
    drmModeFreeCrtc((*conn).old_crtc);
    wl_event_source_remove((*conn).retry_pageflip);
    wl_list_remove(&mut (*conn).link);
    free(conn as *mut libc::c_void);
}
static mut output_impl: wlr_output_impl =
    {
    
        {
            let mut init =
                wlr_output_impl{enable:
                                    Some(enable_drm_connector as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _: bool)
                                                 -> bool),
                                set_mode:
                                    Some(drm_connector_set_mode as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _:
                                                                      *mut wlr_output_mode)
                                                 -> bool),
                                set_custom_mode:
                                    Some(drm_connector_set_custom_mode as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _: int32_t,
                                                                  _: int32_t,
                                                                  _: int32_t)
                                                 -> bool),
                                set_cursor:
                                    Some(drm_connector_set_cursor as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _:
                                                                      *mut wlr_texture,
                                                                  _: int32_t,
                                                                  _:
                                                                      wl_output_transform,
                                                                  _: int32_t,
                                                                  _: int32_t,
                                                                  _: bool)
                                                 -> bool),
                                move_cursor:
                                    Some(drm_connector_move_cursor as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int)
                                                 -> bool),
                                destroy:
                                    Some(drm_connector_destroy as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output)
                                                 -> ()),
                                attach_render:
                                    Some(drm_connector_attach_render as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _:
                                                                      *mut libc::c_int)
                                                 -> bool),
                                commit:
                                    Some(drm_connector_commit as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output)
                                                 -> bool),
                                set_gamma:
                                    Some(set_drm_connector_gamma as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _: size_t,
                                                                  _:
                                                                      *const uint16_t,
                                                                  _:
                                                                      *const uint16_t,
                                                                  _:
                                                                      *const uint16_t)
                                                 -> bool),
                                get_gamma_size:
                                    Some(drm_connector_get_gamma_size as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output)
                                                 -> size_t),
                                export_dmabuf:
                                    Some(drm_connector_export_dmabuf as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _:
                                                                      *mut wlr_dmabuf_attributes)
                                                 -> bool),
                                schedule_frame:
                                    Some(drm_connector_schedule_frame as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output)
                                                 -> bool),
                                attach_buffer:
                                    Some(drm_connector_attach_buffer as
                                             unsafe extern "C" fn(_:
                                                                      *mut wlr_output,
                                                                  _:
                                                                      *mut wlr_buffer)
                                                 -> bool),};
            init
        }
};
#[no_mangle]
pub unsafe extern "C" fn wlr_output_is_drm(mut output: *mut wlr_output)
 -> bool {
    return (*output).impl_0 == &output_impl as *const wlr_output_impl;
}
unsafe extern "C" fn retry_pageflip(mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut conn: *mut wlr_drm_connector = data as *mut wlr_drm_connector;
    _wlr_log(WLR_INFO,
             b"[%s:%d] %s: Retrying pageflip\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             1026i32, (*conn).output.name.as_mut_ptr());
    drm_connector_start_renderer(conn);
    return 0i32;
}
static mut subpixel_map: [int32_t; 7] =
    [0, WL_OUTPUT_SUBPIXEL_UNKNOWN as libc::c_int,
     WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB as libc::c_int,
     WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR as libc::c_int,
     WL_OUTPUT_SUBPIXEL_VERTICAL_RGB as libc::c_int,
     WL_OUTPUT_SUBPIXEL_VERTICAL_BGR as libc::c_int,
     WL_OUTPUT_SUBPIXEL_NONE as libc::c_int];
unsafe extern "C" fn dealloc_crtc(mut conn: *mut wlr_drm_connector) {
    let mut drm: *mut wlr_drm_backend =
        get_drm_backend_from_backend((*conn).output.backend);
    if (*conn).crtc.is_null() { return }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] De-allocating CRTC %zu for output \'%s\'\x00" as
                 *const u8 as *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             1048i32,
             (*conn).crtc.wrapping_offset_from((*drm).crtcs) as libc::c_long,
             (*conn).output.name.as_mut_ptr());
    set_drm_connector_gamma(&mut (*conn).output, 0i32 as size_t,
                            0 as *const uint16_t, 0 as *const uint16_t,
                            0 as *const uint16_t);
    finish_drm_surface(&mut (*(*(*conn).crtc).primary).surf);
    finish_drm_surface(&mut (*(*(*conn).crtc).cursor).surf);
    (*(*drm).iface).conn_enable.expect("non-null function pointer")(drm, conn,
                                                                    0i32 !=
                                                                        0);
    (*conn).crtc = 0 as *mut wlr_drm_crtc;
}
unsafe extern "C" fn realloc_crtcs(mut drm: *mut wlr_drm_backend) {
    if (*drm).num_crtcs > 0i32 as libc::c_ulong {
    } else {
        __assert_fail(b"drm->num_crtcs > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/drm/drm.c\x00" as *const u8 as
                          *const libc::c_char, 1060i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void realloc_crtcs(struct wlr_drm_backend *)\x00")).as_ptr());
    };
    let mut num_outputs: size_t =
        wl_list_length(&mut (*drm).outputs) as size_t;
    if num_outputs == 0i32 as libc::c_ulong { return }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] Reallocating CRTCs\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             1067i32);
    let vla = num_outputs as usize;
    let mut connectors: Vec<*mut wlr_drm_connector> =
        ::std::vec::from_elem(0 as *mut wlr_drm_connector, vla);
    let vla_0 = num_outputs as usize;
    let mut connector_constraints: Vec<uint32_t> =
        ::std::vec::from_elem(0, vla_0);
    let vla_1 = (*drm).num_crtcs as usize;
    let mut previous_match: Vec<uint32_t> = ::std::vec::from_elem(0, vla_1);
    let vla_2 = (*drm).num_crtcs as usize;
    let mut new_match: Vec<uint32_t> = ::std::vec::from_elem(0, vla_2);
    let mut i: size_t = 0i32 as size_t;
    while i < (*drm).num_crtcs {
        *previous_match.as_mut_ptr().offset(i as isize) =
            UNMATCHED as libc::c_uint;
        i = i.wrapping_add(1)
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] State before reallocation:\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             1078i32);
    let mut i_0: size_t = 0i32 as size_t;
    let mut conn: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
    conn =
        ((*drm).outputs.next as *mut libc::c_char).offset(-688) as
            *mut wlr_drm_connector;
    while &mut (*conn).link as *mut wl_list !=
              &mut (*drm).outputs as *mut wl_list {
        let ref mut fresh3 = *connectors.as_mut_ptr().offset(i_0 as isize);
        *fresh3 = conn;
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d]   \'%s\' crtc=%d state=%d desired_enabled=%d\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 1087i32,
                 (*conn).output.name.as_mut_ptr(),
                 if !(*conn).crtc.is_null() {
                     (*conn).crtc.wrapping_offset_from((*drm).crtcs) as
                         libc::c_long as libc::c_int
                 } else { -1i32 }, (*conn).state as libc::c_uint,
                 (*conn).desired_enabled as libc::c_int);
        if !(*conn).crtc.is_null() {
            *previous_match.as_mut_ptr().offset((*conn).crtc.wrapping_offset_from((*drm).crtcs)
                                                    as libc::c_long as isize)
                = i_0 as uint32_t
        }
        // Only search CRTCs for user-enabled outputs (that are already
		// connected or in need of a modeset)
        if ((*conn).state as libc::c_uint ==
                WLR_DRM_CONN_CONNECTED as libc::c_int as libc::c_uint ||
                (*conn).state as libc::c_uint ==
                    WLR_DRM_CONN_NEEDS_MODESET as libc::c_int as libc::c_uint)
               && (*conn).desired_enabled as libc::c_int != 0 {
            *connector_constraints.as_mut_ptr().offset(i_0 as isize) =
                (*conn).possible_crtc
        } else {
            // Will always fail to match anything
            *connector_constraints.as_mut_ptr().offset(i_0 as isize) =
                0i32 as uint32_t
        }
        i_0 = i_0.wrapping_add(1);
        conn =
            ((*conn).link.next as *mut libc::c_char).offset(-688) as
                *mut wlr_drm_connector
    }
    match_obj(num_outputs, connector_constraints.as_mut_ptr(),
              (*drm).num_crtcs, previous_match.as_mut_ptr(),
              new_match.as_mut_ptr());
    // Converts our crtc=>connector result into a connector=>crtc one.
    let vla_3 = num_outputs as usize;
    let mut connector_match: Vec<ssize_t> = ::std::vec::from_elem(0, vla_3);
    let mut i_1: size_t = 0i32 as size_t;
    while i_1 < num_outputs {
        *connector_match.as_mut_ptr().offset(i_1 as isize) = -1i32 as ssize_t;
        i_1 = i_1.wrapping_add(1)
    }
    let mut i_2: size_t = 0i32 as size_t;
    while i_2 < (*drm).num_crtcs {
        if *new_match.as_mut_ptr().offset(i_2 as isize) !=
               UNMATCHED as libc::c_uint {
            *connector_match.as_mut_ptr().offset(*new_match.as_mut_ptr().offset(i_2
                                                                                    as
                                                                                    isize)
                                                     as isize) =
                i_2 as ssize_t
        }
        i_2 = i_2.wrapping_add(1)
    }
    /*
	 * In the case that we add a new connector (hotplug) and we fail to
	 * match everything, we prefer to fail the new connector and keep all
	 * of the old mappings instead.
	 */
    let mut i_3: size_t = 0i32 as size_t;
    while i_3 < num_outputs {
        let mut conn_0: *mut wlr_drm_connector =
            *connectors.as_mut_ptr().offset(i_3 as isize);
        if (*conn_0).state as libc::c_uint ==
               WLR_DRM_CONN_CONNECTED as libc::c_int as libc::c_uint &&
               (*conn_0).desired_enabled as libc::c_int != 0 &&
               *connector_match.as_mut_ptr().offset(i_3 as isize) ==
                   -1i32 as libc::c_long {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] Could not match a CRTC for previously connected output; keeping old configuration\x00"
                         as *const u8 as *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 1132i32);
            return
        }
        i_3 = i_3.wrapping_add(1)
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] State after reallocation:\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             1136i32);
    // Apply new configuration
    let mut i_4: size_t = 0i32 as size_t;
    while i_4 < num_outputs {
        let mut conn_1: *mut wlr_drm_connector =
            *connectors.as_mut_ptr().offset(i_4 as isize);
        let mut prev_enabled: bool = !(*conn_1).crtc.is_null();
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d]   \'%s\' crtc=%zd state=%d desired_enabled=%d\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 1146i32,
                 (*conn_1).output.name.as_mut_ptr(),
                 *connector_match.as_mut_ptr().offset(i_4 as isize),
                 (*conn_1).state as libc::c_uint,
                 (*conn_1).desired_enabled as libc::c_int);
        // We don't need to change anything.
        if !(prev_enabled as libc::c_int != 0 &&
                 *connector_match.as_mut_ptr().offset(i_4 as isize) ==
                     (*conn_1).crtc.wrapping_offset_from((*drm).crtcs) as
                         libc::c_long) {
            dealloc_crtc(conn_1);
            if *connector_match.as_mut_ptr().offset(i_4 as isize) ==
                   -1i32 as libc::c_long {
                if prev_enabled {
                    _wlr_log(WLR_DEBUG,
                             b"[%s:%d] Output has %s lost its CRTC\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../backend/drm/drm.c\x00" as *const u8 as
                                 *const libc::c_char, 1158i32,
                             (*conn_1).output.name.as_mut_ptr());
                    (*conn_1).state = WLR_DRM_CONN_NEEDS_MODESET;
                    wlr_output_update_enabled(&mut (*conn_1).output,
                                              0i32 != 0);
                    (*conn_1).desired_mode = (*conn_1).output.current_mode;
                    wlr_output_update_mode(&mut (*conn_1).output,
                                           0 as *mut wlr_output_mode);
                }
            } else {
                (*conn_1).crtc =
                    &mut *(*drm).crtcs.offset(*connector_match.as_mut_ptr().offset(i_4
                                                                                       as
                                                                                       isize)
                                                  as isize) as
                        *mut wlr_drm_crtc;
                // Only realloc buffers if we have actually been modeset
                if !((*conn_1).state as libc::c_uint !=
                         WLR_DRM_CONN_CONNECTED as libc::c_int as
                             libc::c_uint) {
                    let mut mode: *mut wlr_drm_mode =
                        (*conn_1).output.current_mode as *mut wlr_drm_mode;
                    if !drm_connector_init_renderer(conn_1, mode) {
                        _wlr_log(WLR_ERROR,
                                 b"[%s:%d] Failed to initialize renderer for plane\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"../backend/drm/drm.c\x00" as *const u8 as
                                     *const libc::c_char, 1177i32);
                        drm_connector_cleanup(conn_1);
                        break ;
                    } else { wlr_output_damage_whole(&mut (*conn_1).output); }
                }
            }
        }
        i_4 = i_4.wrapping_add(1)
    };
}
unsafe extern "C" fn get_possible_crtcs(mut fd: libc::c_int,
                                        mut res: *mut drmModeRes,
                                        mut conn: *mut drmModeConnector,
                                        mut is_mst: bool) -> uint32_t {
    let mut ret: uint32_t = 0i32 as uint32_t;
    let mut i: libc::c_int = 0i32;
    while i < (*conn).count_encoders {
        let mut enc: *mut drmModeEncoder =
            drmModeGetEncoder(fd, *(*conn).encoders.offset(i as isize));
        if !enc.is_null() {
            ret |= (*enc).possible_crtcs;
            drmModeFreeEncoder(enc);
        }
        i += 1
    }
    // Sometimes DP MST connectors report no encoders, so we'll loop though
	// all of the encoders of the MST type instead.
	// TODO: See if there is a better solution.
    if !is_mst || ret != 0 { return ret }
    let mut i_0: libc::c_int = 0i32;
    while i_0 < (*res).count_encoders {
        let mut enc_0: *mut drmModeEncoder =
            drmModeGetEncoder(fd, *(*res).encoders.offset(i_0 as isize));
        if !enc_0.is_null() {
            if (*enc_0).encoder_type == 7i32 as libc::c_uint {
                ret |= (*enc_0).possible_crtcs
            }
            drmModeFreeEncoder(enc_0);
        }
        i_0 += 1
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn scan_drm_connectors(mut drm: *mut wlr_drm_backend) {
    /*
	 * This GPU is not really a modesetting device.
	 * It's just being used as a renderer.
	 */
    if (*drm).num_crtcs == 0i32 as libc::c_ulong { return }
    _wlr_log(WLR_INFO,
             b"[%s:%d] Scanning DRM connectors\x00" as *const u8 as
                 *const libc::c_char,
             b"../backend/drm/drm.c\x00" as *const u8 as *const libc::c_char,
             1234i32);
    let mut res: *mut drmModeRes = drmModeGetResources((*drm).fd);
    if res.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to get DRM resources: %s\x00" as *const u8
                     as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 1238i32,
                 strerror(*__errno_location()));
        return
    }
    let mut seen_len: size_t = wl_list_length(&mut (*drm).outputs) as size_t;
    // +1 so length can never be 0, which is undefined behaviour.
	// Last element isn't used.
    let vla = seen_len.wrapping_add(1i32 as libc::c_ulong) as usize;
    let mut seen: Vec<bool> = ::std::vec::from_elem(false, vla);
    memset(seen.as_mut_ptr() as *mut libc::c_void, 0i32,
           (vla * ::std::mem::size_of::<bool>()) as libc::c_ulong);
    let mut new_outputs_len: size_t = 0i32 as size_t;
    let vla_0 = ((*res).count_connectors + 1i32) as usize;
    let mut new_outputs: Vec<*mut wlr_drm_connector> =
        ::std::vec::from_elem(0 as *mut wlr_drm_connector, vla_0);
    let mut current_block_82: u64;
    let mut i: libc::c_int = 0i32;
    while i < (*res).count_connectors {
        let mut drm_conn: *mut drmModeConnector =
            drmModeGetConnector((*drm).fd,
                                *(*res).connectors.offset(i as isize));
        if drm_conn.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to get DRM connector: %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 1254i32,
                     strerror(*__errno_location()));
        } else {
            let mut curr_enc: *mut drmModeEncoder =
                drmModeGetEncoder((*drm).fd, (*drm_conn).encoder_id);
            let mut index: ssize_t = -1i32 as ssize_t;
            let mut c: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
            let mut wlr_conn: *mut wlr_drm_connector =
                0 as *mut wlr_drm_connector;
            c =
                ((*drm).outputs.next as *mut libc::c_char).offset(-688) as
                    *mut wlr_drm_connector;
            while &mut (*c).link as *mut wl_list !=
                      &mut (*drm).outputs as *mut wl_list {
                index += 1;
                if (*c).id == (*drm_conn).connector_id {
                    wlr_conn = c;
                    break ;
                } else {
                    c =
                        ((*c).link.next as *mut libc::c_char).offset(-688) as
                            *mut wlr_drm_connector
                }
            }
            if wlr_conn.is_null() {
                wlr_conn =
                    calloc(1i32 as libc::c_ulong,
                           ::std::mem::size_of::<wlr_drm_connector>() as
                               libc::c_ulong) as *mut wlr_drm_connector;
                if wlr_conn.is_null() {
                    _wlr_log(WLR_ERROR,
                             b"[%s:%d] Allocation failed: %s\x00" as *const u8
                                 as *const libc::c_char,
                             b"../backend/drm/drm.c\x00" as *const u8 as
                                 *const libc::c_char, 1273i32,
                             strerror(*__errno_location()));
                    drmModeFreeEncoder(curr_enc);
                    drmModeFreeConnector(drm_conn);
                    current_block_82 = 13536709405535804910;
                } else {
                    wlr_output_init(&mut (*wlr_conn).output,
                                    &mut (*drm).backend, &output_impl,
                                    (*drm).display);
                    let mut ev: *mut wl_event_loop =
                        wl_display_get_event_loop((*drm).display);
                    (*wlr_conn).retry_pageflip =
                        wl_event_loop_add_timer(ev,
                                                Some(retry_pageflip as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void)
                                                             -> libc::c_int),
                                                wlr_conn as
                                                    *mut libc::c_void);
                    (*wlr_conn).state = WLR_DRM_CONN_DISCONNECTED;
                    (*wlr_conn).id = (*drm_conn).connector_id;
                    snprintf((*wlr_conn).output.name.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 24]>() as
                                 libc::c_ulong,
                             b"%s-%u\x00" as *const u8 as *const libc::c_char,
                             conn_get_name((*drm_conn).connector_type),
                             (*drm_conn).connector_type_id);
                    if !curr_enc.is_null() {
                        (*wlr_conn).old_crtc =
                            drmModeGetCrtc((*drm).fd, (*curr_enc).crtc_id)
                    }
                    wl_list_insert((*drm).outputs.prev,
                                   &mut (*wlr_conn).link);
                    _wlr_log(WLR_INFO,
                             b"[%s:%d] Found connector \'%s\'\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../backend/drm/drm.c\x00" as *const u8 as
                                 *const libc::c_char, 1297i32,
                             (*wlr_conn).output.name.as_mut_ptr());
                    current_block_82 = 3934796541983872331;
                }
            } else {
                *seen.as_mut_ptr().offset(index as isize) = 1i32 != 0;
                current_block_82 = 3934796541983872331;
            }
            match current_block_82 {
                13536709405535804910 => { }
                _ => {
                    if !curr_enc.is_null() {
                        let mut i_0: size_t = 0i32 as size_t;
                        while i_0 < (*drm).num_crtcs {
                            if (*(*drm).crtcs.offset(i_0 as isize)).id ==
                                   (*curr_enc).crtc_id {
                                (*wlr_conn).crtc =
                                    &mut *(*drm).crtcs.offset(i_0 as isize) as
                                        *mut wlr_drm_crtc;
                                break ;
                            } else { i_0 = i_0.wrapping_add(1) }
                        }
                    } else { (*wlr_conn).crtc = 0 as *mut wlr_drm_crtc }
                    // This can only happen *after* hotplug, since we haven't read the
		// connector properties yet
                    if (*wlr_conn).props.c2rust_unnamed.link_status !=
                           0i32 as libc::c_uint {
                        let mut link_status: uint64_t = 0;
                        if !get_drm_prop((*drm).fd, (*wlr_conn).id,
                                         (*wlr_conn).props.c2rust_unnamed.link_status,
                                         &mut link_status) {
                            _wlr_log(WLR_ERROR,
                                     b"[%s:%d] Failed to get link status for \'%s\'\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"../backend/drm/drm.c\x00" as *const u8
                                         as *const libc::c_char, 1320i32,
                                     (*wlr_conn).output.name.as_mut_ptr());
                            current_block_82 = 13536709405535804910;
                        } else {
                            if link_status == 1i32 as libc::c_ulong {
                                // We need to reload our list of modes and force a modeset
                                _wlr_log(WLR_INFO,
                                         b"[%s:%d] Bad link for \'%s\'\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"../backend/drm/drm.c\x00" as
                                             *const u8 as *const libc::c_char,
                                         1326i32,
                                         (*wlr_conn).output.name.as_mut_ptr());
                                drm_connector_cleanup(wlr_conn);
                            }
                            current_block_82 = 9241535491006583629;
                        }
                    } else { current_block_82 = 9241535491006583629; }
                    match current_block_82 {
                        13536709405535804910 => { }
                        _ => {
                            if (*wlr_conn).state as libc::c_uint ==
                                   WLR_DRM_CONN_DISCONNECTED as libc::c_int as
                                       libc::c_uint &&
                                   (*drm_conn).connection as libc::c_uint ==
                                       DRM_MODE_CONNECTED as libc::c_int as
                                           libc::c_uint {
                                _wlr_log(WLR_INFO,
                                         b"[%s:%d] \'%s\' connected\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"../backend/drm/drm.c\x00" as
                                             *const u8 as *const libc::c_char,
                                         1333i32,
                                         (*wlr_conn).output.name.as_mut_ptr());
                                _wlr_log(WLR_DEBUG,
                                         b"[%s:%d] Current CRTC: %d\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"../backend/drm/drm.c\x00" as
                                             *const u8 as *const libc::c_char,
                                         1335i32,
                                         if !(*wlr_conn).crtc.is_null() {
                                             (*(*wlr_conn).crtc).id as
                                                 libc::c_int
                                         } else { -1i32 });
                                (*wlr_conn).output.phys_width =
                                    (*drm_conn).mmWidth as int32_t;
                                (*wlr_conn).output.phys_height =
                                    (*drm_conn).mmHeight as int32_t;
                                _wlr_log(WLR_INFO,
                                         b"[%s:%d] Physical size: %dx%d\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"../backend/drm/drm.c\x00" as
                                             *const u8 as *const libc::c_char,
                                         1340i32,
                                         (*wlr_conn).output.phys_width,
                                         (*wlr_conn).output.phys_height);
                                (*wlr_conn).output.subpixel =
                                    subpixel_map[(*drm_conn).subpixel as
                                                     usize] as
                                        wl_output_subpixel;
                                get_drm_connector_props((*drm).fd,
                                                        (*wlr_conn).id,
                                                        &mut (*wlr_conn).props);
                                let mut edid_len: size_t = 0i32 as size_t;
                                let mut edid: *mut uint8_t =
                                    get_drm_prop_blob((*drm).fd,
                                                      (*wlr_conn).id,
                                                      (*wlr_conn).props.c2rust_unnamed.edid,
                                                      &mut edid_len) as
                                        *mut uint8_t;
                                parse_edid(&mut (*wlr_conn).output, edid_len,
                                           edid);
                                free(edid as *mut libc::c_void);
                                _wlr_log(WLR_INFO,
                                         b"[%s:%d] Detected modes:\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"../backend/drm/drm.c\x00" as
                                             *const u8 as *const libc::c_char,
                                         1351i32);
                                let mut i_1: libc::c_int = 0i32;
                                while i_1 < (*drm_conn).count_modes {
                                    let mut mode: *mut wlr_drm_mode =
                                        calloc(1i32 as libc::c_ulong,
                                               ::std::mem::size_of::<wlr_drm_mode>()
                                                   as libc::c_ulong) as
                                            *mut wlr_drm_mode;
                                    if mode.is_null() {
                                        _wlr_log(WLR_ERROR,
                                                 b"[%s:%d] Allocation failed: %s\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"../backend/drm/drm.c\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 1356i32,
                                                 strerror(*__errno_location()));
                                    } else if (*(*drm_conn).modes.offset(i_1
                                                                             as
                                                                             isize)).flags
                                                  &
                                                  (1i32 << 4i32) as
                                                      libc::c_uint != 0 {
                                        free(mode as *mut libc::c_void);
                                    } else {
                                        (*mode).drm_mode =
                                            *(*drm_conn).modes.offset(i_1 as
                                                                          isize);
                                        (*mode).wlr_mode.width =
                                            (*mode).drm_mode.hdisplay as
                                                int32_t;
                                        (*mode).wlr_mode.height =
                                            (*mode).drm_mode.vdisplay as
                                                int32_t;
                                        (*mode).wlr_mode.refresh =
                                            calculate_refresh_rate(&mut (*mode).drm_mode);
                                        if (*mode).drm_mode.type_0 &
                                               (1i32 << 3i32) as libc::c_uint
                                               != 0 {
                                            (*mode).wlr_mode.preferred =
                                                1i32 != 0
                                        }
                                        _wlr_log(WLR_INFO,
                                                 b"[%s:%d]   %dx%d@%d\x00" as
                                                     *const u8 as
                                                     *const libc::c_char,
                                                 b"../backend/drm/drm.c\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 1375i32,
                                                 (*mode).wlr_mode.width,
                                                 (*mode).wlr_mode.height,
                                                 (*mode).wlr_mode.refresh);
                                        wl_list_insert(&mut (*wlr_conn).output.modes,
                                                       &mut (*mode).wlr_mode.link);
                                    }
                                    i_1 += 1
                                }
                                let mut path_len: size_t = 0;
                                let mut is_mst: bool = 0i32 != 0;
                                let mut path: *mut libc::c_char =
                                    get_drm_prop_blob((*drm).fd,
                                                      (*wlr_conn).id,
                                                      (*wlr_conn).props.c2rust_unnamed.path,
                                                      &mut path_len) as
                                        *mut libc::c_char;
                                if path_len > 4i32 as libc::c_ulong &&
                                       !path.is_null() &&
                                       strncmp(path,
                                               b"mst:\x00" as *const u8 as
                                                   *const libc::c_char,
                                               4i32 as libc::c_ulong) == 0i32
                                   {
                                    is_mst = 1i32 != 0
                                }
                                free(path as *mut libc::c_void);
                                (*wlr_conn).possible_crtc =
                                    get_possible_crtcs((*drm).fd, res,
                                                       drm_conn, is_mst);
                                if (*wlr_conn).possible_crtc ==
                                       0i32 as libc::c_uint {
                                    _wlr_log(WLR_ERROR,
                                             b"[%s:%d] No CRTC possible for connector \'%s\'\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"../backend/drm/drm.c\x00" as
                                                 *const u8 as
                                                 *const libc::c_char, 1393i32,
                                             (*wlr_conn).output.name.as_mut_ptr());
                                }
                                wlr_output_update_enabled(&mut (*wlr_conn).output,
                                                          !(*wlr_conn).crtc.is_null());
                                (*wlr_conn).desired_enabled = 1i32 != 0;
                                (*wlr_conn).state =
                                    WLR_DRM_CONN_NEEDS_MODESET;
                                let fresh4 = new_outputs_len;
                                new_outputs_len =
                                    new_outputs_len.wrapping_add(1);
                                let ref mut fresh5 =
                                    *new_outputs.as_mut_ptr().offset(fresh4 as
                                                                         isize);
                                *fresh5 = wlr_conn
                            } else if ((*wlr_conn).state as libc::c_uint ==
                                           WLR_DRM_CONN_CONNECTED as
                                               libc::c_int as libc::c_uint ||
                                           (*wlr_conn).state as libc::c_uint
                                               ==
                                               WLR_DRM_CONN_NEEDS_MODESET as
                                                   libc::c_int as
                                                   libc::c_uint) &&
                                          (*drm_conn).connection as
                                              libc::c_uint !=
                                              DRM_MODE_CONNECTED as
                                                  libc::c_int as libc::c_uint
                             {
                                _wlr_log(WLR_INFO,
                                         b"[%s:%d] \'%s\' disconnected\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"../backend/drm/drm.c\x00" as
                                             *const u8 as *const libc::c_char,
                                         1404i32,
                                         (*wlr_conn).output.name.as_mut_ptr());
                                drm_connector_cleanup(wlr_conn);
                            }
                            drmModeFreeEncoder(curr_enc);
                            drmModeFreeConnector(drm_conn);
                        }
                    }
                }
            }
        }
        i += 1
    }
    drmModeFreeResources(res);
    // Iterate in reverse order because we'll remove items from the list and
	// still want indices to remain correct.
    let mut conn: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
    let mut tmp_conn: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
    let mut index_0: size_t = wl_list_length(&mut (*drm).outputs) as size_t;
    conn =
        ((*drm).outputs.prev as *mut libc::c_char).offset(-688) as
            *mut wlr_drm_connector;
    tmp_conn =
        ((*conn).link.prev as *mut libc::c_char).offset(-688) as
            *mut wlr_drm_connector;
    while &mut (*conn).link as *mut wl_list !=
              &mut (*drm).outputs as *mut wl_list {
        index_0 = index_0.wrapping_sub(1);
        if !(index_0 >= seen_len ||
                 *seen.as_mut_ptr().offset(index_0 as isize) as libc::c_int !=
                     0) {
            _wlr_log(WLR_INFO,
                     b"[%s:%d] \'%s\' disappeared\x00" as *const u8 as
                         *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 1425i32,
                     (*conn).output.name.as_mut_ptr());
            drm_connector_cleanup(conn);
            wlr_output_destroy(&mut (*conn).output);
        }
        conn = tmp_conn;
        tmp_conn =
            ((*conn).link.prev as *mut libc::c_char).offset(-688) as
                *mut wlr_drm_connector
    }
    realloc_crtcs(drm);
    let mut i_2: size_t = 0i32 as size_t;
    while i_2 < new_outputs_len {
        let mut conn_0: *mut wlr_drm_connector =
            *new_outputs.as_mut_ptr().offset(i_2 as isize);
        _wlr_log(WLR_INFO,
                 b"[%s:%d] Requesting modeset for \'%s\'\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 1437i32,
                 (*conn_0).output.name.as_mut_ptr());
        wlr_signal_emit_safe(&mut (*drm).backend.events.new_output,
                             &mut (*conn_0).output as *mut wlr_output as
                                 *mut libc::c_void);
        i_2 = i_2.wrapping_add(1)
    }
    attempt_enable_needs_modeset(drm);
}
unsafe extern "C" fn mhz_to_nsec(mut mhz: libc::c_int) -> libc::c_int {
    return (1000000000000i64 / mhz as libc::c_longlong) as libc::c_int;
}
unsafe extern "C" fn page_flip_handler(mut fd: libc::c_int,
                                       mut seq: libc::c_uint,
                                       mut tv_sec: libc::c_uint,
                                       mut tv_usec: libc::c_uint,
                                       mut crtc_id: libc::c_uint,
                                       mut data: *mut libc::c_void) {
    let mut drm: *mut wlr_drm_backend = data as *mut wlr_drm_backend;
    let mut conn: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
    let mut search: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
    search =
        ((*drm).outputs.next as *mut libc::c_char).offset(-688) as
            *mut wlr_drm_connector;
    while &mut (*search).link as *mut wl_list !=
              &mut (*drm).outputs as *mut wl_list {
        if !(*search).crtc.is_null() && (*(*search).crtc).id == crtc_id {
            conn = search
        }
        search =
            ((*search).link.next as *mut libc::c_char).offset(-688) as
                *mut wlr_drm_connector
    }
    if conn.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] No connector for crtc_id %u\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 1462i32, crtc_id);
        return
    }
    (*conn).pageflip_pending = 0i32 != 0;
    if (*conn).state as libc::c_uint !=
           WLR_DRM_CONN_CONNECTED as libc::c_int as libc::c_uint ||
           (*conn).crtc.is_null() {
        return
    }
    // Release the old buffer as it's not displayed anymore. The pending
	// buffer becomes the current buffer.
    wlr_buffer_unref((*conn).current_buffer);
    (*conn).current_buffer = (*conn).pending_buffer;
    (*conn).pending_buffer = 0 as *mut wlr_buffer;
    if !(*conn).current_bo.is_null() { gbm_bo_destroy((*conn).current_bo); }
    (*conn).current_bo = (*conn).pending_bo;
    (*conn).pending_bo = 0 as *mut gbm_bo;
    let mut present_flags: uint32_t =
        (WLR_OUTPUT_PRESENT_VSYNC as libc::c_int |
             WLR_OUTPUT_PRESENT_HW_CLOCK as libc::c_int |
             WLR_OUTPUT_PRESENT_HW_COMPLETION as libc::c_int) as uint32_t;
    if !(*conn).current_buffer.is_null() {
        present_flags |=
            WLR_OUTPUT_PRESENT_ZERO_COPY as libc::c_int as libc::c_uint
    } else {
        post_drm_surface(&mut (*(*(*conn).crtc).primary).surf);
        if !(*drm).parent.is_null() {
            post_drm_surface(&mut (*(*(*conn).crtc).primary).mgpu_surf);
        }
    }
    let mut present_time: timespec =
        {
            let mut init =
                timespec{tv_sec: tv_sec as __time_t,
                         tv_nsec:
                             tv_usec.wrapping_mul(1000i32 as libc::c_uint) as
                                 __syscall_slong_t,};
            init
        };
    let mut present_event: wlr_output_event_present =
        {
            let mut init =
                wlr_output_event_present{output: 0 as *mut wlr_output,
                                         commit_seq:
                                             (*conn).output.commit_seq,
                                         when: &mut present_time,
                                         seq: seq,
                                         refresh:
                                             mhz_to_nsec((*conn).output.refresh),
                                         flags: present_flags,};
            init
        };
    wlr_output_send_present(&mut (*conn).output, &mut present_event);
    if (*(*drm).session).active {
        wlr_output_send_frame(&mut (*conn).output);
    };
}
#[no_mangle]
pub unsafe extern "C" fn handle_drm_event(mut fd: libc::c_int,
                                          mut mask: uint32_t,
                                          mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut event: drmEventContext =
        {
            let mut init =
                _drmEventContext{version: 3i32,
                                 vblank_handler: None,
                                 page_flip_handler: None,
                                 page_flip_handler2:
                                     Some(page_flip_handler as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_uint,
                                                                   _:
                                                                       libc::c_uint,
                                                                   _:
                                                                       libc::c_uint,
                                                                   _:
                                                                       libc::c_uint,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> ()),
                                 sequence_handler: None,};
            init
        };
    drmHandleEvent(fd, &mut event);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn restore_drm_outputs(mut drm: *mut wlr_drm_backend) {
    let mut to_close: uint64_t =
        (1u64 <<
             wl_list_length(&mut (*drm).outputs)).wrapping_sub(1i32 as
                                                                   libc::c_ulong);
    let mut conn: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
    conn =
        ((*drm).outputs.next as *mut libc::c_char).offset(-688) as
            *mut wlr_drm_connector;
    while &mut (*conn).link as *mut wl_list !=
              &mut (*drm).outputs as *mut wl_list {
        if (*conn).state as libc::c_uint ==
               WLR_DRM_CONN_CONNECTED as libc::c_int as libc::c_uint {
            (*conn).state = WLR_DRM_CONN_CLEANUP
        }
        conn =
            ((*conn).link.next as *mut libc::c_char).offset(-688) as
                *mut wlr_drm_connector
    }
    let mut timeout: time_t = time(0 as *mut time_t) + 5i32 as libc::c_long;
    while to_close != 0 && time(0 as *mut time_t) < timeout {
        handle_drm_event((*drm).fd, 0i32 as uint32_t, 0 as *mut libc::c_void);
        let mut i: size_t = 0i32 as size_t;
        let mut conn_0: *mut wlr_drm_connector = 0 as *mut wlr_drm_connector;
        conn_0 =
            ((*drm).outputs.next as *mut libc::c_char).offset(-688) as
                *mut wlr_drm_connector;
        while &mut (*conn_0).link as *mut wl_list !=
                  &mut (*drm).outputs as *mut wl_list {
            if (*conn_0).state as libc::c_uint !=
                   WLR_DRM_CONN_CLEANUP as libc::c_int as libc::c_uint ||
                   !(*conn_0).pageflip_pending {
                to_close &= !(1u64 << i)
            }
            i = i.wrapping_add(1);
            conn_0 =
                ((*conn_0).link.next as *mut libc::c_char).offset(-688) as
                    *mut wlr_drm_connector
        }
    }
    if to_close != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Timed out stopping output renderers\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/drm/drm.c\x00" as *const u8 as
                     *const libc::c_char, 1550i32);
    }
    conn =
        ((*drm).outputs.next as *mut libc::c_char).offset(-688) as
            *mut wlr_drm_connector;
    while &mut (*conn).link as *mut wl_list !=
              &mut (*drm).outputs as *mut wl_list {
        let mut crtc: *mut drmModeCrtc = (*conn).old_crtc;
        if !crtc.is_null() {
            drmModeSetCrtc((*drm).fd, (*crtc).crtc_id, (*crtc).buffer_id,
                           (*crtc).x, (*crtc).y, &mut (*conn).id, 1i32,
                           &mut (*crtc).mode);
            drmModeSetCursor((*drm).fd, (*crtc).crtc_id, 0i32 as uint32_t,
                             0i32 as uint32_t, 0i32 as uint32_t);
        }
        conn =
            ((*conn).link.next as *mut libc::c_char).offset(-688) as
                *mut wlr_drm_connector
    };
}
unsafe extern "C" fn drm_connector_cleanup(mut conn: *mut wlr_drm_connector) {
    if conn.is_null() { return }
    let mut mode: *mut wlr_drm_mode = 0 as *mut wlr_drm_mode;
    let mut tmp: *mut wlr_drm_mode = 0 as *mut wlr_drm_mode;
    let mut current_block_28: u64;
    match (*conn).state as libc::c_uint {
        3 | 2 => {
            (*conn).output.current_mode = 0 as *mut wlr_output_mode;
            (*conn).desired_mode = 0 as *mut wlr_output_mode;
            mode = 0 as *mut wlr_drm_mode;
            tmp = 0 as *mut wlr_drm_mode;
            mode =
                ((*conn).output.modes.next as *mut libc::c_char).offset(-16)
                    as *mut wlr_drm_mode;
            tmp =
                ((*mode).wlr_mode.link.next as *mut libc::c_char).offset(-16)
                    as *mut wlr_drm_mode;
            while &mut (*mode).wlr_mode.link as *mut wl_list !=
                      &mut (*conn).output.modes as *mut wl_list {
                wl_list_remove(&mut (*mode).wlr_mode.link);
                free(mode as *mut libc::c_void);
                mode = tmp;
                tmp =
                    ((*mode).wlr_mode.link.next as
                         *mut libc::c_char).offset(-16) as *mut wlr_drm_mode
            }
            (*conn).output.enabled = 0i32 != 0;
            (*conn).output.refresh = 0i32;
            (*conn).output.height = (*conn).output.refresh;
            (*conn).output.width = (*conn).output.height;
            memset(&mut (*conn).output.make as *mut [libc::c_char; 56] as
                       *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<[libc::c_char; 56]>() as
                       libc::c_ulong);
            memset(&mut (*conn).output.model as *mut [libc::c_char; 16] as
                       *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<[libc::c_char; 16]>() as
                       libc::c_ulong);
            memset(&mut (*conn).output.serial as *mut [libc::c_char; 16] as
                       *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<[libc::c_char; 16]>() as
                       libc::c_ulong);
            if !(*conn).output.idle_frame.is_null() {
                wl_event_source_remove((*conn).output.idle_frame);
                (*conn).output.idle_frame = 0 as *mut wl_event_source
            }
            (*conn).output.needs_frame = 0i32 != 0;
            (*conn).output.frame_pending = 0i32 != 0;
            wlr_buffer_unref((*conn).pending_buffer);
            wlr_buffer_unref((*conn).current_buffer);
            (*conn).current_buffer = 0 as *mut wlr_buffer;
            (*conn).pending_buffer = (*conn).current_buffer;
            current_block_28 = 2131046021463112523;
        }
        1 => { current_block_28 = 2131046021463112523; }
        0 | _ => { current_block_28 = 14648156034262866959; }
    }
    match current_block_28 {
        2131046021463112523 =>
        /* Fallthrough */
        {
            _wlr_log(WLR_INFO,
                     b"[%s:%d] Emitting destruction signal for \'%s\'\x00" as
                         *const u8 as *const libc::c_char,
                     b"../backend/drm/drm.c\x00" as *const u8 as
                         *const libc::c_char, 1602i32,
                     (*conn).output.name.as_mut_ptr());
            dealloc_crtc(conn);
            (*conn).possible_crtc = 0i32 as uint32_t;
            (*conn).desired_mode = 0 as *mut wlr_output_mode;
            wlr_signal_emit_safe(&mut (*conn).output.events.destroy,
                                 &mut (*conn).output as *mut wlr_output as
                                     *mut libc::c_void);
        }
        _ => { }
    }
    (*conn).state = WLR_DRM_CONN_DISCONNECTED;
}
