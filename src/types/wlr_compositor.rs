use libc;
extern "C" {
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
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
    fn wl_resource_post_error(resource: *mut wl_resource, code: uint32_t,
                              msg: *const libc::c_char, _: ...);
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
    static wl_subcompositor_interface: wl_interface;
    #[no_mangle]
    static wl_compositor_interface: wl_interface;
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /*
 * Creates a new region resource with the provided new ID. If `resource_list` is
 * non-NULL, adds the region's resource to the list.
 */
    #[no_mangle]
    fn wlr_region_create(client: *mut wl_client, version: uint32_t,
                         id: uint32_t, resource_list: *mut wl_list)
     -> *mut wl_resource;
    /* *
 * Create a new surface resource with the provided new ID. If `resource_list`
 * is non-NULL, adds the surface's resource to the list.
 */
    #[no_mangle]
    fn wlr_surface_create(client: *mut wl_client, version: uint32_t,
                          id: uint32_t, renderer: *mut wlr_renderer,
                          resource_list: *mut wl_list) -> *mut wlr_surface;
    /* *
 * Set the lifetime role for this surface. Returns 0 on success or -1 if the
 * role cannot be set.
 */
    #[no_mangle]
    fn wlr_surface_set_role(surface: *mut wlr_surface,
                            role: *const wlr_surface_role,
                            role_data: *mut libc::c_void,
                            error_resource: *mut wl_resource,
                            error_code: uint32_t) -> bool;
    /* *
 * Create a new subsurface resource with the provided new ID. If `resource_list`
 * is non-NULL, adds the subsurface's resource to the list.
 */
    #[no_mangle]
    fn wlr_subsurface_create(surface: *mut wlr_surface,
                             parent: *mut wlr_surface, version: uint32_t,
                             id: uint32_t, resource_list: *mut wl_list)
     -> *mut wlr_subsurface;
    /* *
 * Get the root of the subsurface tree for this surface.
 */
    #[no_mangle]
    fn wlr_surface_get_root_surface(surface: *mut wlr_surface)
     -> *mut wlr_surface;
    #[no_mangle]
    fn wlr_surface_from_resource(resource: *mut wl_resource)
     -> *mut wlr_surface;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
    #[no_mangle]
    static subsurface_role: wlr_surface_role;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_compositor_interface {
    pub create_surface: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                    _: *mut wl_resource,
                                                    _: uint32_t) -> ()>,
    pub create_region: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                   _: *mut wl_resource,
                                                   _: uint32_t) -> ()>,
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
pub type wl_subcompositor_error = libc::c_uint;
pub const WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE: wl_subcompositor_error = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_subcompositor_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub get_subsurface: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                    _: *mut wl_resource,
                                                    _: uint32_t,
                                                    _: *mut wl_resource,
                                                    _: *mut wl_resource)
                                   -> ()>,
}
/*
 * 32 bit regions
 */
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
pub struct wlr_texture {
    pub impl_0: *const wlr_texture_impl,
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
    pub events: C2RustUnnamed_0,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * A client buffer.
 */
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
    pub events: C2RustUnnamed_1,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
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
#[derive ( Copy, Clone )]
#[repr(C)]
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
    pub events: C2RustUnnamed_2,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
    pub map: wl_signal,
    pub unmap: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_subsurface_state {
    pub x: int32_t,
    pub y: int32_t,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_is_subsurface(mut surface:
                                                       *mut wlr_surface)
 -> bool {
    return (*surface).role == &subsurface_role as *const wlr_surface_role;
}
/* *
 * Get a subsurface from a surface. Can return NULL if the subsurface has been
 * destroyed.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_subsurface_from_wlr_surface(mut surface:
                                                             *mut wlr_surface)
 -> *mut wlr_subsurface {
    if wlr_surface_is_subsurface(surface) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_surface_is_subsurface(surface)\x00" as *const u8
                          as *const libc::c_char,
                      b"../types/wlr_compositor.c\x00" as *const u8 as
                          *const libc::c_char, 21i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 77],
                                                &[libc::c_char; 77]>(b"struct wlr_subsurface *wlr_subsurface_from_wlr_surface(struct wlr_surface *)\x00")).as_ptr());
    };
    return (*surface).role_data as *mut wlr_subsurface;
}
unsafe extern "C" fn subcompositor_handle_destroy(mut client: *mut wl_client,
                                                  mut resource:
                                                      *mut wl_resource) {
    wl_resource_destroy(resource);
}
unsafe extern "C" fn subcompositor_handle_get_subsurface(mut client:
                                                             *mut wl_client,
                                                         mut resource:
                                                             *mut wl_resource,
                                                         mut id: uint32_t,
                                                         mut surface_resource:
                                                             *mut wl_resource,
                                                         mut parent_resource:
                                                             *mut wl_resource) {
    let mut surface: *mut wlr_surface =
        wlr_surface_from_resource(surface_resource);
    let mut parent: *mut wlr_surface =
        wlr_surface_from_resource(parent_resource);
    static mut msg: [libc::c_char; 31] =
        [103, 101, 116, 95, 115, 117, 98, 115, 117, 114, 102, 97, 99, 101, 58,
         32, 119, 108, 95, 115, 117, 98, 115, 117, 114, 102, 97, 99, 101, 64,
         0];
    if surface == parent {
        wl_resource_post_error(resource,
                               WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE as
                                   libc::c_int as uint32_t,
                               b"%s%d: wl_surface@%d cannot be its own parent\x00"
                                   as *const u8 as *const libc::c_char,
                               msg.as_ptr(), id,
                               wl_resource_get_id(surface_resource));
        return
    }
    if wlr_surface_is_subsurface(surface) as libc::c_int != 0 &&
           !wlr_subsurface_from_wlr_surface(surface).is_null() {
        wl_resource_post_error(resource,
                               WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE as
                                   libc::c_int as uint32_t,
                               b"%s%d: wl_surface@%d is already a sub-surface\x00"
                                   as *const u8 as *const libc::c_char,
                               msg.as_ptr(), id,
                               wl_resource_get_id(surface_resource));
        return
    }
    if wlr_surface_get_root_surface(parent) == surface {
        wl_resource_post_error(resource,
                               WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE as
                                   libc::c_int as uint32_t,
                               b"%s%d: wl_surface@%d is an ancestor of parent\x00"
                                   as *const u8 as *const libc::c_char,
                               msg.as_ptr(), id,
                               wl_resource_get_id(surface_resource));
        return
    }
    if !wlr_surface_set_role(surface, &subsurface_role,
                             0 as *mut libc::c_void, resource,
                             WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE as libc::c_int
                                 as uint32_t) {
        return
    }
    wlr_subsurface_create(surface, parent,
                          wl_resource_get_version(resource) as uint32_t, id,
                          0 as *mut wl_list);
}
static mut subcompositor_impl: wl_subcompositor_interface =
    unsafe {
        {
            let mut init =
                wl_subcompositor_interface{destroy:
                                               Some(subcompositor_handle_destroy
                                                        as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wl_client,
                                                                             _:
                                                                                 *mut wl_resource)
                                                            -> ()),
                                           get_subsurface:
                                               Some(subcompositor_handle_get_subsurface
                                                        as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut wl_client,
                                                                             _:
                                                                                 *mut wl_resource,
                                                                             _:
                                                                                 uint32_t,
                                                                             _:
                                                                                 *mut wl_resource,
                                                                             _:
                                                                                 *mut wl_resource)
                                                            -> ()),};
            init
        }
    };
unsafe extern "C" fn subcompositor_bind(mut client: *mut wl_client,
                                        mut data: *mut libc::c_void,
                                        mut version: uint32_t,
                                        mut id: uint32_t) {
    let mut subcompositor: *mut wlr_subcompositor =
        data as *mut wlr_subcompositor;
    let mut resource: *mut wl_resource =
        wl_resource_create(client, &wl_subcompositor_interface, 1i32, id);
    if resource.is_null() { wl_client_post_no_memory(client); return }
    wl_resource_set_implementation(resource,
                                   &subcompositor_impl as
                                       *const wl_subcompositor_interface as
                                       *const libc::c_void,
                                   subcompositor as *mut libc::c_void, None);
}
unsafe extern "C" fn subcompositor_init(mut subcompositor:
                                            *mut wlr_subcompositor,
                                        mut display: *mut wl_display)
 -> bool {
    (*subcompositor).global =
        wl_global_create(display, &wl_subcompositor_interface, 1i32,
                         subcompositor as *mut libc::c_void,
                         Some(subcompositor_bind as
                                  unsafe extern "C" fn(_: *mut wl_client,
                                                       _: *mut libc::c_void,
                                                       _: uint32_t,
                                                       _: uint32_t) -> ()));
    if (*subcompositor).global.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not allocate subcompositor global: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_compositor.c\x00" as *const u8 as
                     *const libc::c_char, 97i32,
                 strerror(*__errno_location()));
        return 0i32 != 0
    }
    return 1i32 != 0;
}
unsafe extern "C" fn subcompositor_finish(mut subcompositor:
                                              *mut wlr_subcompositor) {
    wl_global_destroy((*subcompositor).global);
}
unsafe extern "C" fn compositor_from_resource(mut resource: *mut wl_resource)
 -> *mut wlr_compositor {
    if wl_resource_instance_of(resource, &wl_compositor_interface,
                               &compositor_impl as
                                   *const wl_compositor_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_compositor_interface, &compositor_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_compositor.c\x00" as *const u8 as
                          *const libc::c_char, 114i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"struct wlr_compositor *compositor_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_compositor;
}
unsafe extern "C" fn compositor_create_surface(mut client: *mut wl_client,
                                               mut resource: *mut wl_resource,
                                               mut id: uint32_t) {
    let mut compositor: *mut wlr_compositor =
        compositor_from_resource(resource);
    let mut surface: *mut wlr_surface =
        wlr_surface_create(client,
                           wl_resource_get_version(resource) as uint32_t, id,
                           (*compositor).renderer, 0 as *mut wl_list);
    if surface.is_null() { wl_client_post_no_memory(client); return }
    wlr_signal_emit_safe(&mut (*compositor).events.new_surface,
                         surface as *mut libc::c_void);
}
unsafe extern "C" fn compositor_create_region(mut client: *mut wl_client,
                                              mut resource: *mut wl_resource,
                                              mut id: uint32_t) {
    wlr_region_create(client, wl_resource_get_version(resource) as uint32_t,
                      id, 0 as *mut wl_list);
}
static mut compositor_impl: wl_compositor_interface =
    unsafe {
        {
            let mut init =
                wl_compositor_interface{create_surface:
                                            Some(compositor_create_surface as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource,
                                                                          _:
                                                                              uint32_t)
                                                         -> ()),
                                        create_region:
                                            Some(compositor_create_region as
                                                     unsafe extern "C" fn(_:
                                                                              *mut wl_client,
                                                                          _:
                                                                              *mut wl_resource,
                                                                          _:
                                                                              uint32_t)
                                                         -> ()),};
            init
        }
    };
unsafe extern "C" fn compositor_bind(mut wl_client: *mut wl_client,
                                     mut data: *mut libc::c_void,
                                     mut version: uint32_t,
                                     mut id: uint32_t) {
    let mut compositor: *mut wlr_compositor = data as *mut wlr_compositor;
    let mut resource: *mut wl_resource =
        wl_resource_create(wl_client, &wl_compositor_interface,
                           version as libc::c_int, id);
    if resource.is_null() { wl_client_post_no_memory(wl_client); return }
    wl_resource_set_implementation(resource,
                                   &compositor_impl as
                                       *const wl_compositor_interface as
                                       *const libc::c_void,
                                   compositor as *mut libc::c_void, None);
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut compositor: *mut wlr_compositor =
        (listener as *mut libc::c_char).offset(-24) as *mut wlr_compositor;
    wlr_signal_emit_safe(&mut (*compositor).events.destroy,
                         compositor as *mut libc::c_void);
    subcompositor_finish(&mut (*compositor).subcompositor);
    wl_list_remove(&mut (*compositor).display_destroy.link);
    wl_global_destroy((*compositor).global);
    free(compositor as *mut libc::c_void);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_compositor_create(mut display: *mut wl_display,
                                               mut renderer:
                                                   *mut wlr_renderer)
 -> *mut wlr_compositor {
    let mut compositor: *mut wlr_compositor =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_compositor>() as libc::c_ulong) as
            *mut wlr_compositor;
    if compositor.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not allocate wlr compositor: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_compositor.c\x00" as *const u8 as
                     *const libc::c_char, 170i32,
                 strerror(*__errno_location()));
        return 0 as *mut wlr_compositor
    }
    (*compositor).global =
        wl_global_create(display, &wl_compositor_interface, 4i32,
                         compositor as *mut libc::c_void,
                         Some(compositor_bind as
                                  unsafe extern "C" fn(_: *mut wl_client,
                                                       _: *mut libc::c_void,
                                                       _: uint32_t,
                                                       _: uint32_t) -> ()));
    if (*compositor).global.is_null() {
        free(compositor as *mut libc::c_void);
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not allocate compositor global: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_compositor.c\x00" as *const u8 as
                     *const libc::c_char, 178i32,
                 strerror(*__errno_location()));
        return 0 as *mut wlr_compositor
    }
    (*compositor).renderer = renderer;
    wl_signal_init(&mut (*compositor).events.new_surface);
    wl_signal_init(&mut (*compositor).events.destroy);
    subcompositor_init(&mut (*compositor).subcompositor, display);
    (*compositor).display_destroy.notify =
        Some(handle_display_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_display_add_destroy_listener(display,
                                    &mut (*compositor).display_destroy);
    return compositor;
}
