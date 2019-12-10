use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    /* Generated by wayland-scanner 1.17.0 */
    pub type wl_client;
    pub type wl_global;
    pub type wlr_texture;
    pub type wlr_surface;
    pub type wlr_backend;
    pub type wlr_output_impl;
    pub type wlr_output_layout_state;
    pub type wlr_output_layout_output_state;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wlr_output_layout_get(layout: *mut wlr_output_layout,
                             reference: *mut wlr_output)
     -> *mut wlr_output_layout_output;
    #[no_mangle]
    fn wlr_output_from_resource(resource: *mut wl_resource)
     -> *mut wlr_output;
    #[no_mangle]
    fn wlr_output_effective_resolution(output: *mut wlr_output,
                                       width: *mut libc::c_int,
                                       height: *mut libc::c_int);
    #[no_mangle]
    fn wlr_output_schedule_done(output: *mut wlr_output);
    #[no_mangle]
    fn wl_display_add_destroy_listener(display: *mut wl_display,
                                       listener: *mut wl_listener);
    #[no_mangle]
    fn wl_global_create(display: *mut wl_display,
                        interface: *const wl_interface, version: libc::c_int,
                        data: *mut libc::c_void, bind: wl_global_bind_func_t)
     -> *mut wl_global;
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
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
    fn wl_resource_get_link(resource: *mut wl_resource) -> *mut wl_list;
    #[no_mangle]
    fn wl_resource_from_link(resource: *mut wl_list) -> *mut wl_resource;
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
    /* *
 * @page page_iface_zxdg_output_manager_v1 zxdg_output_manager_v1
 * @section page_iface_zxdg_output_manager_v1_desc Description
 *
 * A global factory interface for xdg_output objects.
 * @section page_iface_zxdg_output_manager_v1_api API
 * See @ref iface_zxdg_output_manager_v1.
 */
/* *
 * @defgroup iface_zxdg_output_manager_v1 The zxdg_output_manager_v1 interface
 *
 * A global factory interface for xdg_output objects.
 */
    #[no_mangle]
    static zxdg_output_manager_v1_interface: wl_interface;
    /* *
 * @page page_iface_zxdg_output_v1 zxdg_output_v1
 * @section page_iface_zxdg_output_v1_desc Description
 *
 * An xdg_output describes part of the compositor geometry.
 *
 * This typically corresponds to a monitor that displays part of the
 * compositor space.
 *
 * For objects version 3 onwards, after all xdg_output properties have been
 * sent (when the object is created and when properties are updated), a
 * wl_output.done event is sent. This allows changes to the output
 * properties to be seen as atomic, even if they happen via multiple events.
 * @section page_iface_zxdg_output_v1_api API
 * See @ref iface_zxdg_output_v1.
 */
/* *
 * @defgroup iface_zxdg_output_v1 The zxdg_output_v1 interface
 *
 * An xdg_output describes part of the compositor geometry.
 *
 * This typically corresponds to a monitor that displays part of the
 * compositor space.
 *
 * For objects version 3 onwards, after all xdg_output properties have been
 * sent (when the object is created and when properties are updated), a
 * wl_output.done event is sent. This allows changes to the output
 * properties to be seen as atomic, even if they happen via multiple events.
 */
    #[no_mangle]
    static zxdg_output_v1_interface: wl_interface;
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut crate::src::backend::drm::atomic::wlr_texture,
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
    pub texture: *mut crate::src::backend::drm::atomic::wlr_texture,
    pub surface: *mut crate::src::types::data_device::wlr_data_device::wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
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
    pub events: C2RustUnnamed_0,
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
pub struct C2RustUnnamed_0 {
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
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub state: *mut crate::src::types::wlr_output_layout::wlr_output_layout_state,
    pub events: C2RustUnnamed_1,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_layout_output {
    pub output: *mut wlr_output,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub link: wl_list,
    pub state: *mut crate::src::types::wlr_output_layout::wlr_output_layout_output_state,
    pub events: C2RustUnnamed_2,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_output_v1 {
    pub manager: *mut wlr_xdg_output_manager_v1,
    pub resources: wl_list,
    pub link: wl_list,
    pub layout_output: *mut wlr_output_layout_output,
    pub x: int32_t,
    pub y: int32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_output_manager_v1 {
    pub global: *mut wl_global,
    pub layout: *mut wlr_output_layout,
    pub outputs: wl_list,
    pub events: C2RustUnnamed_3,
    pub display_destroy: wl_listener,
    pub layout_add: wl_listener,
    pub layout_change: wl_listener,
    pub layout_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub destroy: wl_signal,
}
/* *
 * @ingroup iface_zxdg_output_manager_v1
 * @struct zxdg_output_manager_v1_interface
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct zxdg_output_manager_v1_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub get_xdg_output: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                    _: *mut wl_resource,
                                                    _: uint32_t,
                                                    _: *mut wl_resource)
                                   -> ()>,
}
/* *
 * @ingroup iface_zxdg_output_manager_v1
 */
/* *
 * @ingroup iface_zxdg_output_manager_v1
 */
/* *
 * @ingroup iface_zxdg_output_v1
 * @struct zxdg_output_v1_interface
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct zxdg_output_v1_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
}
#[inline]
unsafe extern "C" fn wl_output_send_done(mut resource_: *mut wl_resource) {
    wl_resource_post_event(resource_, 2i32 as uint32_t);
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
/* *
 * @ingroup iface_zxdg_output_v1
 */
/* *
 * @ingroup iface_zxdg_output_v1
 * Sends an logical_position event to the client owning the resource.
 * @param resource_ The client's resource
 * @param x x position within the global compositor space
 * @param y y position within the global compositor space
 */
#[inline]
unsafe extern "C" fn zxdg_output_v1_send_logical_position(mut resource_:
                                                              *mut wl_resource,
                                                          mut x: int32_t,
                                                          mut y: int32_t) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, x, y);
}
/* *
 * @ingroup iface_zxdg_output_v1
 * Sends an logical_size event to the client owning the resource.
 * @param resource_ The client's resource
 * @param width width in global compositor space
 * @param height height in global compositor space
 */
#[inline]
unsafe extern "C" fn zxdg_output_v1_send_logical_size(mut resource_:
                                                          *mut wl_resource,
                                                      mut width: int32_t,
                                                      mut height: int32_t) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, width, height);
}
/* *
 * @ingroup iface_zxdg_output_v1
 * Sends an done event to the client owning the resource.
 * @param resource_ The client's resource
 */
#[inline]
unsafe extern "C" fn zxdg_output_v1_send_done(mut resource_:
                                                  *mut wl_resource) {
    wl_resource_post_event(resource_, 2i32 as uint32_t);
}
/* *
 * @ingroup iface_zxdg_output_v1
 * Sends an name event to the client owning the resource.
 * @param resource_ The client's resource
 * @param name output name
 */
#[inline]
unsafe extern "C" fn zxdg_output_v1_send_name(mut resource_: *mut wl_resource,
                                              mut name: *const libc::c_char) {
    wl_resource_post_event(resource_, 3i32 as uint32_t, name);
}
/* *
 * @ingroup iface_zxdg_output_v1
 * Sends an description event to the client owning the resource.
 * @param resource_ The client's resource
 * @param description output description
 */
#[inline]
unsafe extern "C" fn zxdg_output_v1_send_description(mut resource_:
                                                         *mut wl_resource,
                                                     mut description:
                                                         *const libc::c_char) {
    wl_resource_post_event(resource_, 4i32 as uint32_t, description);
}
unsafe extern "C" fn output_handle_destroy(mut client: *mut wl_client,
                                           mut resource: *mut wl_resource) {
    wl_resource_destroy(resource);
}
static mut output_implementation: zxdg_output_v1_interface =
    {
    
        {
            let mut init =
                zxdg_output_v1_interface{destroy:
                                             Some(output_handle_destroy as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wl_client,
                                                                           _:
                                                                               *mut wl_resource)
                                                          -> ()),};
            init
        }
};
unsafe extern "C" fn output_handle_resource_destroy(mut resource:
                                                        *mut wl_resource) {
    wl_list_remove(wl_resource_get_link(resource));
}
unsafe extern "C" fn output_send_details(mut xdg_output:
                                             *mut wlr_xdg_output_v1,
                                         mut resource: *mut wl_resource) {
    zxdg_output_v1_send_logical_position(resource, (*xdg_output).x,
                                         (*xdg_output).y);
    zxdg_output_v1_send_logical_size(resource, (*xdg_output).width,
                                     (*xdg_output).height);
    if wl_resource_get_version(resource) < 3i32 {
        zxdg_output_v1_send_done(resource);
    };
}
unsafe extern "C" fn output_update(mut xdg_output: *mut wlr_xdg_output_v1) {
    let mut layout_output: *mut wlr_output_layout_output =
        (*xdg_output).layout_output;
    let mut updated: bool = 0i32 != 0;
    if (*layout_output).x != (*xdg_output).x ||
           (*layout_output).y != (*xdg_output).y {
        (*xdg_output).x = (*layout_output).x;
        (*xdg_output).y = (*layout_output).y;
        updated = 1i32 != 0
    }
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    wlr_output_effective_resolution((*layout_output).output, &mut width,
                                    &mut height);
    if (*xdg_output).width != width || (*xdg_output).height != height {
        (*xdg_output).width = width;
        (*xdg_output).height = height;
        updated = 1i32 != 0
    }
    if updated {
        let mut resource: *mut wl_resource = 0 as *mut wl_resource;
        resource = 0 as *mut wl_resource;
        resource = wl_resource_from_link((*xdg_output).resources.next);
        while wl_resource_get_link(resource) !=
                  &mut (*xdg_output).resources as *mut wl_list {
            output_send_details(xdg_output, resource);
            resource =
                wl_resource_from_link((*wl_resource_get_link(resource)).next)
        }
        wlr_output_schedule_done((*(*xdg_output).layout_output).output);
    };
}
unsafe extern "C" fn output_destroy(mut output: *mut wlr_xdg_output_v1) {
    let mut resource: *mut wl_resource = 0 as *mut wl_resource;
    let mut tmp: *mut wl_resource = 0 as *mut wl_resource;
    resource = 0 as *mut wl_resource;
    tmp = 0 as *mut wl_resource;
    resource = wl_resource_from_link((*output).resources.next);
    tmp = wl_resource_from_link((*(*output).resources.next).next);
    while wl_resource_get_link(resource) !=
              &mut (*output).resources as *mut wl_list {
        wl_list_remove(wl_resource_get_link(resource));
        wl_list_init(wl_resource_get_link(resource));
        resource = tmp;
        tmp = wl_resource_from_link((*wl_resource_get_link(resource)).next)
    }
    wl_list_remove(&mut (*output).destroy.link);
    wl_list_remove(&mut (*output).link);
    free(output as *mut libc::c_void);
}
unsafe extern "C" fn output_manager_handle_destroy(mut client: *mut wl_client,
                                                   mut resource:
                                                       *mut wl_resource) {
    wl_resource_destroy(resource);
}
unsafe extern "C" fn output_manager_handle_get_xdg_output(mut client:
                                                              *mut wl_client,
                                                          mut resource:
                                                              *mut wl_resource,
                                                          mut id: uint32_t,
                                                          mut output_resource:
                                                              *mut wl_resource) {
    if wl_resource_instance_of(resource, &zxdg_output_manager_v1_interface,
                               &output_manager_implementation as
                                   *const zxdg_output_manager_v1_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &zxdg_output_manager_v1_interface, &output_manager_implementation)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_xdg_output_v1.c\x00" as *const u8 as
                          *const libc::c_char, 90i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 116],
                                                &[libc::c_char; 116]>(b"void output_manager_handle_get_xdg_output(struct wl_client *, struct wl_resource *, uint32_t, struct wl_resource *)\x00")).as_ptr());
    };
    let mut manager: *mut wlr_xdg_output_manager_v1 =
        wl_resource_get_user_data(resource) as *mut wlr_xdg_output_manager_v1;
    let mut layout: *mut wlr_output_layout = (*manager).layout;
    let mut output: *mut wlr_output =
        wlr_output_from_resource(output_resource);
    let mut layout_output: *mut wlr_output_layout_output =
        wlr_output_layout_get(layout, output);
    if !layout_output.is_null() {
    } else {
        __assert_fail(b"layout_output\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_xdg_output_v1.c\x00" as *const u8 as
                          *const libc::c_char, 99i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 116],
                                                &[libc::c_char; 116]>(b"void output_manager_handle_get_xdg_output(struct wl_client *, struct wl_resource *, uint32_t, struct wl_resource *)\x00")).as_ptr());
    };
    let mut _xdg_output: *mut wlr_xdg_output_v1 = 0 as *mut wlr_xdg_output_v1;
    let mut xdg_output: *mut wlr_xdg_output_v1 = 0 as *mut wlr_xdg_output_v1;
    _xdg_output =
        ((*manager).outputs.next as *mut libc::c_char).offset(-24) as
            *mut wlr_xdg_output_v1;
    while &mut (*_xdg_output).link as *mut wl_list !=
              &mut (*manager).outputs as *mut wl_list {
        if (*_xdg_output).layout_output == layout_output {
            xdg_output = _xdg_output;
            break ;
        } else {
            _xdg_output =
                ((*_xdg_output).link.next as *mut libc::c_char).offset(-24) as
                    *mut wlr_xdg_output_v1
        }
    }
    if !xdg_output.is_null() {
    } else {
        __assert_fail(b"xdg_output\x00" as *const u8 as *const libc::c_char,
                      b"../types/wlr_xdg_output_v1.c\x00" as *const u8 as
                          *const libc::c_char, 108i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 116],
                                                &[libc::c_char; 116]>(b"void output_manager_handle_get_xdg_output(struct wl_client *, struct wl_resource *, uint32_t, struct wl_resource *)\x00")).as_ptr());
    };
    let mut xdg_output_resource: *mut wl_resource =
        wl_resource_create(client, &zxdg_output_v1_interface,
                           wl_resource_get_version(resource), id);
    if xdg_output_resource.is_null() {
        wl_client_post_no_memory(client);
        return
    }
    wl_resource_set_implementation(xdg_output_resource,
                                   &output_implementation as
                                       *const zxdg_output_v1_interface as
                                       *const libc::c_void,
                                   0 as *mut libc::c_void,
                                   Some(output_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_insert(&mut (*xdg_output).resources,
                   wl_resource_get_link(xdg_output_resource));
    // Name and description should only be sent once per output
    let mut version: uint32_t =
        wl_resource_get_version(xdg_output_resource) as uint32_t;
    if version >= 2i32 as libc::c_uint {
        zxdg_output_v1_send_name(xdg_output_resource,
                                 (*output).name.as_mut_ptr());
    }
    if version >= 2i32 as libc::c_uint {
        let mut description: [libc::c_char; 128] = [0; 128];
        snprintf(description.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 128]>() as
                     libc::c_ulong,
                 b"%s %s %s (%s)\x00" as *const u8 as *const libc::c_char,
                 (*output).make.as_mut_ptr(), (*output).model.as_mut_ptr(),
                 (*output).serial.as_mut_ptr(), (*output).name.as_mut_ptr());
        zxdg_output_v1_send_description(xdg_output_resource,
                                        description.as_mut_ptr());
    }
    output_send_details(xdg_output, xdg_output_resource);
    wl_output_send_done(output_resource);
}
static mut output_manager_implementation: zxdg_output_manager_v1_interface =
    {
    
        {
            let mut init =
                zxdg_output_manager_v1_interface{destroy:
                                                     Some(output_manager_handle_destroy
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut wl_client,
                                                                                   _:
                                                                                       *mut wl_resource)
                                                                  -> ()),
                                                 get_xdg_output:
                                                     Some(output_manager_handle_get_xdg_output
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut wl_client,
                                                                                   _:
                                                                                       *mut wl_resource,
                                                                                   _:
                                                                                       uint32_t,
                                                                                   _:
                                                                                       *mut wl_resource)
                                                                  -> ()),};
            init
        }
};
unsafe extern "C" fn output_manager_bind(mut wl_client: *mut wl_client,
                                         mut data: *mut libc::c_void,
                                         mut version: uint32_t,
                                         mut id: uint32_t) {
    let mut manager: *mut wlr_xdg_output_manager_v1 =
        data as *mut wlr_xdg_output_manager_v1;
    let mut resource: *mut wl_resource =
        wl_resource_create(wl_client, &zxdg_output_manager_v1_interface,
                           version as libc::c_int, id);
    if resource.is_null() { wl_client_post_no_memory(wl_client); return }
    wl_resource_set_implementation(resource,
                                   &output_manager_implementation as
                                       *const zxdg_output_manager_v1_interface
                                       as *const libc::c_void,
                                   manager as *mut libc::c_void, None);
}
unsafe extern "C" fn handle_output_destroy(mut listener: *mut wl_listener,
                                           mut data: *mut libc::c_void) {
    let mut output: *mut wlr_xdg_output_v1 =
        (listener as *mut libc::c_char).offset(-64) as *mut wlr_xdg_output_v1;
    output_destroy(output);
}
unsafe extern "C" fn add_output(mut manager: *mut wlr_xdg_output_manager_v1,
                                mut layout_output:
                                    *mut wlr_output_layout_output) {
    let mut output: *mut wlr_xdg_output_v1 =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xdg_output_v1>() as libc::c_ulong) as
            *mut wlr_xdg_output_v1;
    if output.is_null() { return }
    wl_list_init(&mut (*output).resources);
    (*output).manager = manager;
    (*output).layout_output = layout_output;
    (*output).destroy.notify =
        Some(handle_output_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*layout_output).events.destroy,
                  &mut (*output).destroy);
    wl_list_insert(&mut (*manager).outputs, &mut (*output).link);
    output_update(output);
}
unsafe extern "C" fn output_manager_send_details(mut manager:
                                                     *mut wlr_xdg_output_manager_v1) {
    let mut output: *mut wlr_xdg_output_v1 = 0 as *mut wlr_xdg_output_v1;
    output =
        ((*manager).outputs.next as *mut libc::c_char).offset(-24) as
            *mut wlr_xdg_output_v1;
    while &mut (*output).link as *mut wl_list !=
              &mut (*manager).outputs as *mut wl_list {
        output_update(output);
        output =
            ((*output).link.next as *mut libc::c_char).offset(-24) as
                *mut wlr_xdg_output_v1
    };
}
unsafe extern "C" fn handle_layout_add(mut listener: *mut wl_listener,
                                       mut data: *mut libc::c_void) {
    let mut manager: *mut wlr_xdg_output_manager_v1 =
        (listener as *mut libc::c_char).offset(-72) as
            *mut wlr_xdg_output_manager_v1;
    let mut layout_output: *mut wlr_output_layout_output =
        data as *mut wlr_output_layout_output;
    add_output(manager, layout_output);
}
unsafe extern "C" fn handle_layout_change(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut manager: *mut wlr_xdg_output_manager_v1 =
        (listener as *mut libc::c_char).offset(-96) as
            *mut wlr_xdg_output_manager_v1;
    output_manager_send_details(manager);
}
unsafe extern "C" fn manager_destroy(mut manager:
                                         *mut wlr_xdg_output_manager_v1) {
    wlr_signal_emit_safe(&mut (*manager).events.destroy,
                         manager as *mut libc::c_void);
    wl_list_remove(&mut (*manager).display_destroy.link);
    wl_list_remove(&mut (*manager).layout_add.link);
    wl_list_remove(&mut (*manager).layout_change.link);
    wl_list_remove(&mut (*manager).layout_destroy.link);
    free(manager as *mut libc::c_void);
}
unsafe extern "C" fn handle_layout_destroy(mut listener: *mut wl_listener,
                                           mut data: *mut libc::c_void) {
    let mut manager: *mut wlr_xdg_output_manager_v1 =
        (listener as *mut libc::c_char).offset(-120) as
            *mut wlr_xdg_output_manager_v1;
    manager_destroy(manager);
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut manager: *mut wlr_xdg_output_manager_v1 =
        (listener as *mut libc::c_char).offset(-48) as
            *mut wlr_xdg_output_manager_v1;
    manager_destroy(manager);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_output_manager_v1_create(mut display:
                                                              *mut wl_display,
                                                          mut layout:
                                                              *mut wlr_output_layout)
 -> *mut wlr_xdg_output_manager_v1 {
    // TODO: require wayland-protocols 1.18 and remove this condition
    let mut version: libc::c_int = 3i32;
    if version > zxdg_output_manager_v1_interface.version {
        version = zxdg_output_manager_v1_interface.version
    }
    let mut manager: *mut wlr_xdg_output_manager_v1 =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xdg_output_manager_v1>() as
                   libc::c_ulong) as *mut wlr_xdg_output_manager_v1;
    if manager.is_null() { return 0 as *mut wlr_xdg_output_manager_v1 }
    (*manager).layout = layout;
    (*manager).global =
        wl_global_create(display, &zxdg_output_manager_v1_interface, version,
                         manager as *mut libc::c_void,
                         Some(output_manager_bind as
                                  unsafe extern "C" fn(_: *mut wl_client,
                                                       _: *mut libc::c_void,
                                                       _: uint32_t,
                                                       _: uint32_t) -> ()));
    if (*manager).global.is_null() {
        free(manager as *mut libc::c_void);
        return 0 as *mut wlr_xdg_output_manager_v1
    }
    wl_list_init(&mut (*manager).outputs);
    let mut layout_output: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    layout_output =
        ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    while &mut (*layout_output).link as *mut wl_list !=
              &mut (*layout).outputs as *mut wl_list {
        add_output(manager, layout_output);
        layout_output =
            ((*layout_output).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output
    }
    wl_signal_init(&mut (*manager).events.destroy);
    (*manager).layout_add.notify =
        Some(handle_layout_add as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*layout).events.add, &mut (*manager).layout_add);
    (*manager).layout_change.notify =
        Some(handle_layout_change as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*layout).events.change,
                  &mut (*manager).layout_change);
    (*manager).layout_destroy.notify =
        Some(handle_layout_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*layout).events.destroy,
                  &mut (*manager).layout_destroy);
    (*manager).display_destroy.notify =
        Some(handle_display_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_display_add_destroy_listener(display, &mut (*manager).display_destroy);
    return manager;
}
