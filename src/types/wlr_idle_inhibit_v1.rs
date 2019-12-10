use libc;
extern "C" {
    pub type wl_display;
    /* Generated by wayland-scanner 1.17.0 */
    pub type wl_client;
    pub type wl_global;
    pub type wlr_texture;
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
    pub type wlr_renderer;
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
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
    #[no_mangle]
    fn wl_resource_instance_of(resource: *mut wl_resource,
                               interface: *const wl_interface,
                               implementation: *const libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn wl_resource_get_user_data(resource: *mut wl_resource)
     -> *mut libc::c_void;
    #[no_mangle]
    fn wl_resource_set_user_data(resource: *mut wl_resource,
                                 data: *mut libc::c_void);
    #[no_mangle]
    fn wl_resource_destroy(resource: *mut wl_resource);
    #[no_mangle]
    fn wl_resource_set_implementation(resource: *mut wl_resource,
                                      implementation: *const libc::c_void,
                                      data: *mut libc::c_void,
                                      destroy: wl_resource_destroy_func_t);
    #[no_mangle]
    fn wl_resource_create(client: *mut wl_client,
                          interface: *const wl_interface,
                          version: libc::c_int, id: uint32_t)
     -> *mut wl_resource;
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_global_destroy(global: *mut wl_global);
    #[no_mangle]
    fn wl_global_create(display: *mut wl_display,
                        interface: *const wl_interface, version: libc::c_int,
                        data: *mut libc::c_void, bind: wl_global_bind_func_t)
     -> *mut wl_global;
    #[no_mangle]
    fn wl_display_add_destroy_listener(display: *mut wl_display,
                                       listener: *mut wl_listener);
    #[no_mangle]
    fn wlr_surface_from_resource(resource: *mut wl_resource)
     -> *mut wlr_surface;
    /* *
 * @page page_iface_zwp_idle_inhibit_manager_v1 zwp_idle_inhibit_manager_v1
 * @section page_iface_zwp_idle_inhibit_manager_v1_desc Description
 *
 * This interface permits inhibiting the idle behavior such as screen
 * blanking, locking, and screensaving.  The client binds the idle manager
 * globally, then creates idle-inhibitor objects for each surface.
 *
 * Warning! The protocol described in this file is experimental and
 * backward incompatible changes may be made. Backward compatible changes
 * may be added together with the corresponding interface version bump.
 * Backward incompatible changes are done by bumping the version number in
 * the protocol and interface names and resetting the interface version.
 * Once the protocol is to be declared stable, the 'z' prefix and the
 * version number in the protocol and interface names are removed and the
 * interface version number is reset.
 * @section page_iface_zwp_idle_inhibit_manager_v1_api API
 * See @ref iface_zwp_idle_inhibit_manager_v1.
 */
/* *
 * @defgroup iface_zwp_idle_inhibit_manager_v1 The zwp_idle_inhibit_manager_v1 interface
 *
 * This interface permits inhibiting the idle behavior such as screen
 * blanking, locking, and screensaving.  The client binds the idle manager
 * globally, then creates idle-inhibitor objects for each surface.
 *
 * Warning! The protocol described in this file is experimental and
 * backward incompatible changes may be made. Backward compatible changes
 * may be added together with the corresponding interface version bump.
 * Backward incompatible changes are done by bumping the version number in
 * the protocol and interface names and resetting the interface version.
 * Once the protocol is to be declared stable, the 'z' prefix and the
 * version number in the protocol and interface names are removed and the
 * interface version number is reset.
 */
    #[no_mangle]
    static zwp_idle_inhibit_manager_v1_interface: wl_interface;
    /* *
 * @page page_iface_zwp_idle_inhibitor_v1 zwp_idle_inhibitor_v1
 * @section page_iface_zwp_idle_inhibitor_v1_desc Description
 *
 * An idle inhibitor prevents the output that the associated surface is
 * visible on from being set to a state where it is not visually usable due
 * to lack of user interaction (e.g. blanked, dimmed, locked, set to power
 * save, etc.)  Any screensaver processes are also blocked from displaying.
 *
 * If the surface is destroyed, unmapped, becomes occluded, loses
 * visibility, or otherwise becomes not visually relevant for the user, the
 * idle inhibitor will not be honored by the compositor; if the surface
 * subsequently regains visibility the inhibitor takes effect once again.
 * Likewise, the inhibitor isn't honored if the system was already idled at
 * the time the inhibitor was established, although if the system later
 * de-idles and re-idles the inhibitor will take effect.
 * @section page_iface_zwp_idle_inhibitor_v1_api API
 * See @ref iface_zwp_idle_inhibitor_v1.
 */
/* *
 * @defgroup iface_zwp_idle_inhibitor_v1 The zwp_idle_inhibitor_v1 interface
 *
 * An idle inhibitor prevents the output that the associated surface is
 * visible on from being set to a state where it is not visually usable due
 * to lack of user interaction (e.g. blanked, dimmed, locked, set to power
 * save, etc.)  Any screensaver processes are also blocked from displaying.
 *
 * If the surface is destroyed, unmapped, becomes occluded, loses
 * visibility, or otherwise becomes not visually relevant for the user, the
 * idle inhibitor will not be honored by the compositor; if the surface
 * subsequently regains visibility the inhibitor takes effect once again.
 * Likewise, the inhibitor isn't honored if the system was already idled at
 * the time the inhibitor was established, although if the system later
 * de-idles and re-idles the inhibitor will take effect.
 */
    #[no_mangle]
    static zwp_idle_inhibitor_v1_interface: wl_interface;
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* This interface permits clients to inhibit the idle behavior such as
 * screenblanking, locking, and screensaving.
 *
 * This allows clients to ensure they stay visible instead of being hidden by
 * power-saving.
 *
 * Inhibitors are created for surfaces. They should only be in effect, while
 * this surface is visible.
 * The effect could also be limited to outputs it is displayed on (e.g.
 * dimm/dpms off outputs, except the one a video is displayed on).
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_idle_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_idle_inhibitor_v1 {
    pub surface: *mut wlr_surface,
    pub resource: *mut wl_resource,
    pub surface_destroy: wl_listener,
    pub link: wl_list,
    pub events: C2RustUnnamed_0,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
    pub events: C2RustUnnamed_1,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
pub type pixman_region32_t = pixman_region32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
/*
 * 32 bit regions
 */
pub type pixman_region32_data_t = pixman_region32_data;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_box32_t = pixman_box32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
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
/* *
 * @ingroup iface_zwp_idle_inhibit_manager_v1
 * @struct zwp_idle_inhibit_manager_v1_interface
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct zwp_idle_inhibit_manager_v1_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub create_inhibitor: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                      _: *mut wl_resource,
                                                      _: uint32_t,
                                                      _: *mut wl_resource)
                                     -> ()>,
}
/* *
 * @ingroup iface_zwp_idle_inhibit_manager_v1
 */
/* *
 * @ingroup iface_zwp_idle_inhibit_manager_v1
 */
/* *
 * @ingroup iface_zwp_idle_inhibitor_v1
 * @struct zwp_idle_inhibitor_v1_interface
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct zwp_idle_inhibitor_v1_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
unsafe extern "C" fn wlr_idle_inhibit_manager_v1_from_resource(mut resource:
                                                                   *mut wl_resource)
 -> *mut wlr_idle_inhibit_manager_v1 {
    if wl_resource_instance_of(resource,
                               &zwp_idle_inhibit_manager_v1_interface,
                               &idle_inhibit_impl as
                                   *const zwp_idle_inhibit_manager_v1_interface
                                   as *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &zwp_idle_inhibit_manager_v1_interface, &idle_inhibit_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_idle_inhibit_v1.c\x00" as *const u8 as
                          *const libc::c_char, 18i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 100],
                                                &[libc::c_char; 100]>(b"struct wlr_idle_inhibit_manager_v1 *wlr_idle_inhibit_manager_v1_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as
               *mut wlr_idle_inhibit_manager_v1;
}
unsafe extern "C" fn wlr_idle_inhibitor_v1_from_resource(mut resource:
                                                             *mut wl_resource)
 -> *mut wlr_idle_inhibitor_v1 {
    if wl_resource_instance_of(resource, &zwp_idle_inhibitor_v1_interface,
                               &idle_inhibitor_impl as
                                   *const zwp_idle_inhibitor_v1_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &zwp_idle_inhibitor_v1_interface, &idle_inhibitor_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_idle_inhibit_v1.c\x00" as *const u8 as
                          *const libc::c_char, 25i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 88],
                                                &[libc::c_char; 88]>(b"struct wlr_idle_inhibitor_v1 *wlr_idle_inhibitor_v1_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_idle_inhibitor_v1;
}
unsafe extern "C" fn idle_inhibitor_v1_destroy(mut inhibitor:
                                                   *mut wlr_idle_inhibitor_v1) {
    if inhibitor.is_null() { return }
    wlr_signal_emit_safe(&mut (*inhibitor).events.destroy,
                         (*inhibitor).surface as *mut libc::c_void);
    wl_resource_set_user_data((*inhibitor).resource, 0 as *mut libc::c_void);
    wl_list_remove(&mut (*inhibitor).link);
    wl_list_remove(&mut (*inhibitor).surface_destroy.link);
    free(inhibitor as *mut libc::c_void);
}
unsafe extern "C" fn idle_inhibitor_v1_handle_resource_destroy(mut resource:
                                                                   *mut wl_resource) {
    let mut inhibitor: *mut wlr_idle_inhibitor_v1 =
        wlr_idle_inhibitor_v1_from_resource(resource);
    idle_inhibitor_v1_destroy(inhibitor);
}
unsafe extern "C" fn idle_inhibitor_handle_surface_destroy(mut listener:
                                                               *mut wl_listener,
                                                           mut data:
                                                               *mut libc::c_void) {
    let mut inhibitor: *mut wlr_idle_inhibitor_v1 =
        (listener as *mut libc::c_char).offset(-16) as
            *mut wlr_idle_inhibitor_v1;
    idle_inhibitor_v1_destroy(inhibitor);
}
unsafe extern "C" fn idle_inhibitor_v1_handle_destroy(mut client:
                                                          *mut wl_client,
                                                      mut manager_resource:
                                                          *mut wl_resource) {
    wl_resource_destroy(manager_resource);
}
static mut idle_inhibitor_impl: zwp_idle_inhibitor_v1_interface =
    unsafe {
        {
            let mut init =
                zwp_idle_inhibitor_v1_interface{destroy:
                                                    Some(idle_inhibitor_v1_handle_destroy
                                                             as
                                                             unsafe extern "C" fn(_:
                                                                                      *mut wl_client,
                                                                                  _:
                                                                                      *mut wl_resource)
                                                                 -> ()),};
            init
        }
    };
unsafe extern "C" fn manager_handle_create_inhibitor(mut client:
                                                         *mut wl_client,
                                                     mut resource:
                                                         *mut wl_resource,
                                                     mut id: uint32_t,
                                                     mut surface_resource:
                                                         *mut wl_resource) {
    let mut surface: *mut wlr_surface =
        wlr_surface_from_resource(surface_resource);
    let mut manager: *mut wlr_idle_inhibit_manager_v1 =
        wlr_idle_inhibit_manager_v1_from_resource(resource);
    let mut inhibitor: *mut wlr_idle_inhibitor_v1 =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_idle_inhibitor_v1>() as
                   libc::c_ulong) as *mut wlr_idle_inhibitor_v1;
    if inhibitor.is_null() { wl_client_post_no_memory(client); return }
    let mut wl_resource: *mut wl_resource =
        wl_resource_create(client, &zwp_idle_inhibitor_v1_interface, 1i32,
                           id);
    if wl_resource.is_null() {
        wl_client_post_no_memory(client);
        free(inhibitor as *mut libc::c_void);
        return
    }
    (*inhibitor).resource = wl_resource;
    (*inhibitor).surface = surface;
    wl_signal_init(&mut (*inhibitor).events.destroy);
    (*inhibitor).surface_destroy.notify =
        Some(idle_inhibitor_handle_surface_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*surface).events.destroy,
                  &mut (*inhibitor).surface_destroy);
    wl_resource_set_implementation(wl_resource,
                                   &idle_inhibitor_impl as
                                       *const zwp_idle_inhibitor_v1_interface
                                       as *const libc::c_void,
                                   inhibitor as *mut libc::c_void,
                                   Some(idle_inhibitor_v1_handle_resource_destroy
                                            as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_insert(&mut (*manager).inhibitors, &mut (*inhibitor).link);
    wlr_signal_emit_safe(&mut (*manager).events.new_inhibitor,
                         inhibitor as *mut libc::c_void);
}
unsafe extern "C" fn manager_handle_destroy(mut client: *mut wl_client,
                                            mut manager_resource:
                                                *mut wl_resource) {
    wl_resource_destroy(manager_resource);
}
static mut idle_inhibit_impl: zwp_idle_inhibit_manager_v1_interface =
    unsafe {
        {
            let mut init =
                zwp_idle_inhibit_manager_v1_interface{destroy:
                                                          Some(manager_handle_destroy
                                                                   as
                                                                   unsafe extern "C" fn(_:
                                                                                            *mut wl_client,
                                                                                        _:
                                                                                            *mut wl_resource)
                                                                       -> ()),
                                                      create_inhibitor:
                                                          Some(manager_handle_create_inhibitor
                                                                   as
                                                                   unsafe extern "C" fn(_:
                                                                                            *mut wl_client,
                                                                                        _:
                                                                                            *mut wl_resource,
                                                                                        _:
                                                                                            uint32_t,
                                                                                        _:
                                                                                            *mut wl_resource)
                                                                       ->
                                                                           ()),};
            init
        }
    };
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut idle_inhibit: *mut wlr_idle_inhibit_manager_v1 =
        (listener as *mut libc::c_char).offset(-24) as
            *mut wlr_idle_inhibit_manager_v1;
    wlr_signal_emit_safe(&mut (*idle_inhibit).events.destroy,
                         idle_inhibit as *mut libc::c_void);
    wl_list_remove(&mut (*idle_inhibit).display_destroy.link);
    wl_global_destroy((*idle_inhibit).global);
    free(idle_inhibit as *mut libc::c_void);
}
unsafe extern "C" fn idle_inhibit_bind(mut wl_client: *mut wl_client,
                                       mut data: *mut libc::c_void,
                                       mut version: uint32_t,
                                       mut id: uint32_t) {
    let mut idle_inhibit: *mut wlr_idle_inhibit_manager_v1 =
        data as *mut wlr_idle_inhibit_manager_v1;
    let mut wl_resource: *mut wl_resource =
        wl_resource_create(wl_client, &zwp_idle_inhibit_manager_v1_interface,
                           version as libc::c_int, id);
    if wl_resource.is_null() { wl_client_post_no_memory(wl_client); return }
    wl_resource_set_implementation(wl_resource,
                                   &idle_inhibit_impl as
                                       *const zwp_idle_inhibit_manager_v1_interface
                                       as *const libc::c_void,
                                   idle_inhibit as *mut libc::c_void, None);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_idle_inhibit_v1_create(mut display:
                                                        *mut wl_display)
 -> *mut wlr_idle_inhibit_manager_v1 {
    let mut idle_inhibit: *mut wlr_idle_inhibit_manager_v1 =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_idle_inhibit_manager_v1>() as
                   libc::c_ulong) as *mut wlr_idle_inhibit_manager_v1;
    if idle_inhibit.is_null() { return 0 as *mut wlr_idle_inhibit_manager_v1 }
    wl_list_init(&mut (*idle_inhibit).inhibitors);
    wl_signal_init(&mut (*idle_inhibit).events.new_inhibitor);
    wl_signal_init(&mut (*idle_inhibit).events.destroy);
    (*idle_inhibit).global =
        wl_global_create(display, &zwp_idle_inhibit_manager_v1_interface,
                         1i32, idle_inhibit as *mut libc::c_void,
                         Some(idle_inhibit_bind as
                                  unsafe extern "C" fn(_: *mut wl_client,
                                                       _: *mut libc::c_void,
                                                       _: uint32_t,
                                                       _: uint32_t) -> ()));
    if (*idle_inhibit).global.is_null() {
        free(idle_inhibit as *mut libc::c_void);
        return 0 as *mut wlr_idle_inhibit_manager_v1
    }
    (*idle_inhibit).display_destroy.notify =
        Some(handle_display_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_display_add_destroy_listener(display,
                                    &mut (*idle_inhibit).display_destroy);
    return idle_inhibit;
}
