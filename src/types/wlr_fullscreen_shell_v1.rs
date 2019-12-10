use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_texture;
    pub type wlr_renderer;
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
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
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
    static zwp_fullscreen_shell_v1_interface: wl_interface;
    #[no_mangle]
    fn wlr_output_from_resource(resource: *mut wl_resource)
     -> *mut wlr_output;
    #[no_mangle]
    fn wlr_surface_from_resource(resource: *mut wl_resource)
     -> *mut wlr_surface;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;

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
pub type wl_global_bind_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_client, _: *mut libc::c_void,
                                _: uint32_t, _: uint32_t) -> ()>;

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
pub type zwp_fullscreen_shell_v1_present_method = libc::c_uint;
pub const ZWP_FULLSCREEN_SHELL_V1_PRESENT_METHOD_STRETCH:
          zwp_fullscreen_shell_v1_present_method =
    4;
pub const ZWP_FULLSCREEN_SHELL_V1_PRESENT_METHOD_ZOOM_CROP:
          zwp_fullscreen_shell_v1_present_method =
    3;
pub const ZWP_FULLSCREEN_SHELL_V1_PRESENT_METHOD_ZOOM:
          zwp_fullscreen_shell_v1_present_method =
    2;
pub const ZWP_FULLSCREEN_SHELL_V1_PRESENT_METHOD_CENTER:
          zwp_fullscreen_shell_v1_present_method =
    1;
pub const ZWP_FULLSCREEN_SHELL_V1_PRESENT_METHOD_DEFAULT:
          zwp_fullscreen_shell_v1_present_method =
    0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwp_fullscreen_shell_v1_interface {
    pub release: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub present_surface: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                     _: *mut wl_resource,
                                                     _: *mut wl_resource,
                                                     _: uint32_t,
                                                     _: *mut wl_resource)
                                    -> ()>,
    pub present_surface_for_mode: Option<unsafe extern "C" fn(_:
                                                                  *mut wl_client,
                                                              _:
                                                                  *mut wl_resource,
                                                              _:
                                                                  *mut wl_resource,
                                                              _:
                                                                  *mut wl_resource,
                                                              _: int32_t,
                                                              _: uint32_t)
                                             -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_fullscreen_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2RustUnnamed,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
    pub present_surface: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_fullscreen_shell_v1_present_surface_event {
    pub client: *mut wl_client,
    pub surface: *mut wlr_surface,
    pub method: zwp_fullscreen_shell_v1_present_method,
    pub output: *mut wlr_output,
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
    pub texture: *mut crate::src::backend::drm::atomic::wlr_texture,
    pub surface: *mut wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_0,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_surface {
    pub resource: *mut wl_resource,
    pub renderer: *mut crate::src::backend::drm::atomic::wlr_renderer,
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
pub type pixman_region32_t = pixman_region32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
/*
 * 32 bit regions
 */
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
    pub texture: *mut crate::src::backend::drm::atomic::wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
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
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[inline]
unsafe extern "C" fn zwp_fullscreen_shell_mode_feedback_v1_send_mode_failed(mut resource_:
                                                                                *mut wl_resource) {
    wl_resource_post_event(resource_, 1i32 as uint32_t);
}
unsafe extern "C" fn shell_from_resource(mut resource: *mut wl_resource)
 -> *mut wlr_fullscreen_shell_v1 {
    if wl_resource_instance_of(resource, &zwp_fullscreen_shell_v1_interface,
                               &shell_impl as
                                   *const zwp_fullscreen_shell_v1_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &zwp_fullscreen_shell_v1_interface, &shell_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_fullscreen_shell_v1.c\x00" as *const u8
                          as *const libc::c_char, 16i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"struct wlr_fullscreen_shell_v1 *shell_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as
               *mut wlr_fullscreen_shell_v1;
}
unsafe extern "C" fn shell_handle_present_surface(mut client: *mut wl_client,
                                                  mut shell_resource:
                                                      *mut wl_resource,
                                                  mut surface_resource:
                                                      *mut wl_resource,
                                                  mut method: uint32_t,
                                                  mut output_resource:
                                                      *mut wl_resource) {
    let mut shell: *mut wlr_fullscreen_shell_v1 =
        shell_from_resource(shell_resource);
    let mut surface: *mut wlr_surface = 0 as *mut wlr_surface;
    if !surface_resource.is_null() {
        surface = wlr_surface_from_resource(surface_resource)
    }
    let mut output: *mut wlr_output = 0 as *mut wlr_output;
    if !output_resource.is_null() {
        output = wlr_output_from_resource(output_resource)
    }
    let mut event: wlr_fullscreen_shell_v1_present_surface_event =
        {
            let mut init =
                wlr_fullscreen_shell_v1_present_surface_event{client: client,
                                                              surface:
                                                                  surface,
                                                              method:
                                                                  method as
                                                                      zwp_fullscreen_shell_v1_present_method,
                                                              output:
                                                                  output,};
            init
        };
    wlr_signal_emit_safe(&mut (*shell).events.present_surface,
                         &mut event as
                             *mut wlr_fullscreen_shell_v1_present_surface_event
                             as *mut libc::c_void);
}
unsafe extern "C" fn shell_handle_present_surface_for_mode(mut client:
                                                               *mut wl_client,
                                                           mut shell_resource:
                                                               *mut wl_resource,
                                                           mut surface_resource:
                                                               *mut wl_resource,
                                                           mut output_resource:
                                                               *mut wl_resource,
                                                           mut framerate:
                                                               int32_t,
                                                           mut feedback_id:
                                                               uint32_t) {
    let mut version: uint32_t =
        wl_resource_get_version(shell_resource) as uint32_t;
    let mut feedback_resource: *mut wl_resource =
        wl_resource_create(client, 0 as *const wl_interface,
                           version as libc::c_int, feedback_id);
    if feedback_resource.is_null() {
        wl_resource_post_no_memory(shell_resource);
        return
    }
    // TODO: add support for mode switch
    zwp_fullscreen_shell_mode_feedback_v1_send_mode_failed(feedback_resource);
    wl_resource_destroy(feedback_resource);
}
static mut shell_impl: zwp_fullscreen_shell_v1_interface =
    {
    
        {
            let mut init =
                zwp_fullscreen_shell_v1_interface{release: None,
                                                  present_surface:
                                                      Some(shell_handle_present_surface
                                                               as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut wl_client,
                                                                                    _:
                                                                                        *mut wl_resource,
                                                                                    _:
                                                                                        *mut wl_resource,
                                                                                    _:
                                                                                        uint32_t,
                                                                                    _:
                                                                                        *mut wl_resource)
                                                                   -> ()),
                                                  present_surface_for_mode:
                                                      Some(shell_handle_present_surface_for_mode
                                                               as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut wl_client,
                                                                                    _:
                                                                                        *mut wl_resource,
                                                                                    _:
                                                                                        *mut wl_resource,
                                                                                    _:
                                                                                        *mut wl_resource,
                                                                                    _:
                                                                                        int32_t,
                                                                                    _:
                                                                                        uint32_t)
                                                                   -> ()),};
            init
        }
};
unsafe extern "C" fn shell_bind(mut client: *mut wl_client,
                                mut data: *mut libc::c_void,
                                mut version: uint32_t, mut id: uint32_t) {
    let mut shell: *mut wlr_fullscreen_shell_v1 =
        data as *mut wlr_fullscreen_shell_v1;
    let mut resource: *mut wl_resource =
        wl_resource_create(client, &zwp_fullscreen_shell_v1_interface,
                           version as libc::c_int, id);
    if resource.is_null() { wl_client_post_no_memory(client); return }
    wl_resource_set_implementation(resource,
                                   &shell_impl as
                                       *const zwp_fullscreen_shell_v1_interface
                                       as *const libc::c_void,
                                   shell as *mut libc::c_void, None);
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut shell: *mut wlr_fullscreen_shell_v1 =
        (listener as *mut libc::c_char).offset(-40) as
            *mut wlr_fullscreen_shell_v1;
    wlr_signal_emit_safe(&mut (*shell).events.destroy,
                         shell as *mut libc::c_void);
    wl_list_remove(&mut (*shell).display_destroy.link);
    wl_global_destroy((*shell).global);
    free(shell as *mut libc::c_void);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
// wlr_fullscreen_shell_v1_present_surface_event
// can be NULL
// can be NULL
#[no_mangle]
pub unsafe extern "C" fn wlr_fullscreen_shell_v1_create(mut display:
                                                            *mut wl_display)
 -> *mut wlr_fullscreen_shell_v1 {
    let mut shell: *mut wlr_fullscreen_shell_v1 =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_fullscreen_shell_v1>() as
                   libc::c_ulong) as *mut wlr_fullscreen_shell_v1;
    if shell.is_null() { return 0 as *mut wlr_fullscreen_shell_v1 }
    wl_signal_init(&mut (*shell).events.destroy);
    wl_signal_init(&mut (*shell).events.present_surface);
    (*shell).global =
        wl_global_create(display, &zwp_fullscreen_shell_v1_interface, 1i32,
                         shell as *mut libc::c_void,
                         Some(shell_bind as
                                  unsafe extern "C" fn(_: *mut wl_client,
                                                       _: *mut libc::c_void,
                                                       _: uint32_t,
                                                       _: uint32_t) -> ()));
    if (*shell).global.is_null() {
        free(shell as *mut libc::c_void);
        return 0 as *mut wlr_fullscreen_shell_v1
    }
    (*shell).display_destroy.notify =
        Some(handle_display_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_display_add_destroy_listener(display, &mut (*shell).display_destroy);
    return shell;
}
