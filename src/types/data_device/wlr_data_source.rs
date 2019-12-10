use libc;
extern "C" {
    pub type wl_client;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_resource_post_event(resource: *mut wl_resource, opcode: uint32_t,
                              _: ...);
    #[no_mangle]
    fn wl_resource_post_error(resource: *mut wl_resource, code: uint32_t,
                              msg: *const libc::c_char, _: ...);
    #[no_mangle]
    fn wl_resource_post_no_memory(resource: *mut wl_resource);
    #[no_mangle]
    fn wl_resource_instance_of(resource: *mut wl_resource,
                               interface: *const wl_interface,
                               implementation: *const libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_array_init(array: *mut wl_array);
    #[no_mangle]
    fn wl_array_release(array: *mut wl_array);
    #[no_mangle]
    fn wl_array_add(array: *mut wl_array, size: size_t) -> *mut libc::c_void;
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
    fn wl_resource_set_user_data(resource: *mut wl_resource,
                                 data: *mut libc::c_void);
    #[no_mangle]
    fn wl_resource_get_user_data(resource: *mut wl_resource)
     -> *mut libc::c_void;
    #[no_mangle]
    fn wl_resource_get_version(resource: *mut wl_resource) -> libc::c_int;
    #[no_mangle]
    static wl_data_source_interface: wl_interface;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
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
pub struct wl_array {
    pub size: size_t,
    pub alloc: size_t,
    pub data: *mut libc::c_void,
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
pub type wl_data_source_error = libc::c_uint;
pub const WL_DATA_SOURCE_ERROR_INVALID_SOURCE: wl_data_source_error = 1;
pub const WL_DATA_SOURCE_ERROR_INVALID_ACTION_MASK: wl_data_source_error = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_data_source_interface {
    pub offer: Option<unsafe extern "C" fn(_: *mut wl_client,
                                           _: *mut wl_resource,
                                           _: *const libc::c_char) -> ()>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub set_actions: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                 _: *mut wl_resource,
                                                 _: uint32_t) -> ()>,
}
pub type wl_data_device_manager_dnd_action = libc::c_uint;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK:
          wl_data_device_manager_dnd_action =
    4;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE:
          wl_data_device_manager_dnd_action =
    2;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY:
          wl_data_device_manager_dnd_action =
    1;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE:
          wl_data_device_manager_dnd_action =
    0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_data_source {
    pub impl_0: *const wlr_data_source_impl,
    pub mime_types: wl_array,
    pub actions: int32_t,
    pub accepted: bool,
    pub current_dnd_action: wl_data_device_manager_dnd_action,
    pub compositor_action: uint32_t,
    pub events: C2RustUnnamed,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_data_source_impl {
    pub send: Option<unsafe extern "C" fn(_: *mut wlr_data_source,
                                          _: *const libc::c_char, _: int32_t)
                         -> ()>,
    pub accept: Option<unsafe extern "C" fn(_: *mut wlr_data_source,
                                            _: uint32_t,
                                            _: *const libc::c_char) -> ()>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_data_source) -> ()>,
    pub dnd_drop: Option<unsafe extern "C" fn(_: *mut wlr_data_source) -> ()>,
    pub dnd_finish: Option<unsafe extern "C" fn(_: *mut wlr_data_source)
                               -> ()>,
    pub dnd_action: Option<unsafe extern "C" fn(_: *mut wlr_data_source,
                                                _:
                                                    wl_data_device_manager_dnd_action)
                               -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_client_data_source {
    pub source: wlr_data_source,
    pub impl_0: wlr_data_source_impl,
    pub resource: *mut wl_resource,
    pub finalized: bool,
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
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[inline]
unsafe extern "C" fn wl_data_source_send_target(mut resource_:
                                                    *mut wl_resource,
                                                mut mime_type:
                                                    *const libc::c_char) {
    wl_resource_post_event(resource_, 0i32 as uint32_t, mime_type);
}
#[inline]
unsafe extern "C" fn wl_data_source_send_send(mut resource_: *mut wl_resource,
                                              mut mime_type:
                                                  *const libc::c_char,
                                              mut fd: int32_t) {
    wl_resource_post_event(resource_, 1i32 as uint32_t, mime_type, fd);
}
#[inline]
unsafe extern "C" fn wl_data_source_send_cancelled(mut resource_:
                                                       *mut wl_resource) {
    wl_resource_post_event(resource_, 2i32 as uint32_t);
}
#[inline]
unsafe extern "C" fn wl_data_source_send_dnd_drop_performed(mut resource_:
                                                                *mut wl_resource) {
    wl_resource_post_event(resource_, 3i32 as uint32_t);
}
#[inline]
unsafe extern "C" fn wl_data_source_send_dnd_finished(mut resource_:
                                                          *mut wl_resource) {
    wl_resource_post_event(resource_, 4i32 as uint32_t);
}
#[inline]
unsafe extern "C" fn wl_data_source_send_action(mut resource_:
                                                    *mut wl_resource,
                                                mut dnd_action: uint32_t) {
    wl_resource_post_event(resource_, 5i32 as uint32_t, dnd_action);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_data_source_init(mut source:
                                                  *mut wlr_data_source,
                                              mut impl_0:
                                                  *const wlr_data_source_impl) {
    if (*impl_0).send.is_some() {
    } else {
        __assert_fail(b"impl->send\x00" as *const u8 as *const libc::c_char,
                      b"../types/data_device/wlr_data_source.c\x00" as
                          *const u8 as *const libc::c_char,
                      16i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[libc::c_char; 89]>(b"void wlr_data_source_init(struct wlr_data_source *, const struct wlr_data_source_impl *)\x00")).as_ptr());
    };
    (*source).impl_0 = impl_0;
    wl_array_init(&mut (*source).mime_types);
    wl_signal_init(&mut (*source).events.destroy);
    (*source).actions = -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_data_source_send(mut source:
                                                  *mut wlr_data_source,
                                              mut mime_type:
                                                  *const libc::c_char,
                                              mut fd: int32_t) {
    (*(*source).impl_0).send.expect("non-null function pointer")(source,
                                                                 mime_type,
                                                                 fd);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_data_source_accept(mut source:
                                                    *mut wlr_data_source,
                                                mut serial: uint32_t,
                                                mut mime_type:
                                                    *const libc::c_char) {
    (*source).accepted = !mime_type.is_null();
    if (*(*source).impl_0).accept.is_some() {
        (*(*source).impl_0).accept.expect("non-null function pointer")(source,
                                                                       serial,
                                                                       mime_type);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_data_source_destroy(mut source:
                                                     *mut wlr_data_source) {
    if source.is_null() { return }
    wlr_signal_emit_safe(&mut (*source).events.destroy,
                         source as *mut libc::c_void);
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    p = (*source).mime_types.data as *mut *mut libc::c_char;
    while (p as *const libc::c_char) <
              ((*source).mime_types.data as
                   *const libc::c_char).offset((*source).mime_types.size as
                                                   isize) {
        free(*p as *mut libc::c_void);
        p = p.offset(1)
    }
    wl_array_release(&mut (*source).mime_types);
    if (*(*source).impl_0).destroy.is_some() {
        (*(*source).impl_0).destroy.expect("non-null function pointer")(source);
    } else { free(source as *mut libc::c_void); };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_data_source_dnd_drop(mut source:
                                                      *mut wlr_data_source) {
    if (*(*source).impl_0).dnd_drop.is_some() {
        (*(*source).impl_0).dnd_drop.expect("non-null function pointer")(source);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_data_source_dnd_finish(mut source:
                                                        *mut wlr_data_source) {
    if (*(*source).impl_0).dnd_finish.is_some() {
        (*(*source).impl_0).dnd_finish.expect("non-null function pointer")(source);
    };
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
// wlr_seat::{selection_offers,drag_offers}
/* *
 * A data source implementation. Only the `send` function is mandatory. Refer to
 * the matching wl_data_source_* functions documentation to know what they do.
 */
// source metadata
// source status
// drag'n'drop status
// can be NULL
// can be NULL
// can be NULL
// if WLR_DRAG_GRAB_TOUCH
// wlr_drag_motion_event
// wlr_drag_drop_event
/* *
 * Create a wl data device manager global for this display.
 */
/* *
 * Requests a selection to be set for the seat. If the request comes from
 * a client, then set `client` to be the matching seat client so that this
 * function can verify that the serial provided was once sent to the client
 * on this seat.
 */
/* *
 * Sets the current selection for the seat. NULL can be provided to clear it.
 * This removes the previous one if there was any. In case the selection doesn't
 * come from a client, wl_display_next_serial() can be used to generate a
 * serial.
 */
/* *
 * Creates a new drag. To request to start the drag, call
 * `wlr_seat_request_start_drag`.
 */
/* *
 * Requests a drag to be started on the seat.
 */
/* *
 * Starts a drag on the seat. This starts an implicit keyboard grab, but doesn't
 * start a pointer or a touch grab.
 */
/* *
 * Starts a pointer drag on the seat. This starts implicit keyboard and pointer
 * grabs.
 */
/* *
 * Starts a touch drag on the seat. This starts implicit keyboard and touch
 * grabs.
 */
/* *
 * Initializes the data source with the provided implementation.
 */
/* *
 * Sends the data as the specified MIME type over the passed file descriptor,
 * then close it.
 */
/* *
 * Notifies the data source that a target accepts one of the offered MIME types.
 * If a target doesn't accept any of the offered types, `mime_type` is NULL.
 */
/* *
 * Notifies the data source it is no longer valid and should be destroyed. That
 * destroys immediately the data source.
 */
/* *
 * Notifies the data source that the drop operation was performed. This does not
 * indicate acceptance.
 *
 * The data source may still be used in the future and should not be destroyed
 * here.
 */
/* *
 * Notifies the data source that the drag-and-drop operation concluded. That
 * potentially destroys immediately the data source.
 */
/* *
 * Notifies the data source that a target accepts the drag with the specified
 * action.
 *
 * This shouldn't be called after `wlr_data_source_dnd_drop` unless the
 * drag-and-drop operation ended in an "ask" action.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_data_source_dnd_action(mut source:
                                                        *mut wlr_data_source,
                                                    mut action:
                                                        wl_data_device_manager_dnd_action) {
    (*source).current_dnd_action = action;
    if (*(*source).impl_0).dnd_action.is_some() {
        (*(*source).impl_0).dnd_action.expect("non-null function pointer")(source,
                                                                           action);
    };
}
#[no_mangle]
pub unsafe extern "C" fn client_data_source_from_resource(mut resource:
                                                              *mut wl_resource)
 -> *mut wlr_client_data_source {
    if wl_resource_instance_of(resource, &wl_data_source_interface,
                               &data_source_impl as
                                   *const wl_data_source_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_data_source_interface, &data_source_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/data_device/wlr_data_source.c\x00" as
                          *const u8 as *const libc::c_char,
                      83i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 86],
                                                &[libc::c_char; 86]>(b"struct wlr_client_data_source *client_data_source_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut wlr_client_data_source;
}
unsafe extern "C" fn client_data_source_from_wlr_data_source(mut wlr_source:
                                                                 *mut wlr_data_source)
 -> *mut wlr_client_data_source {
    if (*(*wlr_source).impl_0).accept ==
           Some(client_data_source_accept as
                    unsafe extern "C" fn(_: *mut wlr_data_source, _: uint32_t,
                                         _: *const libc::c_char) -> ()) {
    } else {
        __assert_fail(b"wlr_source->impl->accept == client_data_source_accept\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/data_device/wlr_data_source.c\x00" as
                          *const u8 as *const libc::c_char,
                      92i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 97],
                                                &[libc::c_char; 97]>(b"struct wlr_client_data_source *client_data_source_from_wlr_data_source(struct wlr_data_source *)\x00")).as_ptr());
    };
    return wlr_source as *mut wlr_client_data_source;
}
unsafe extern "C" fn client_data_source_accept(mut wlr_source:
                                                   *mut wlr_data_source,
                                               mut serial: uint32_t,
                                               mut mime_type:
                                                   *const libc::c_char) {
    let mut source: *mut wlr_client_data_source =
        client_data_source_from_wlr_data_source(wlr_source);
    wl_data_source_send_target((*source).resource, mime_type);
}
unsafe extern "C" fn client_data_source_send(mut wlr_source:
                                                 *mut wlr_data_source,
                                             mut mime_type:
                                                 *const libc::c_char,
                                             mut fd: int32_t) {
    let mut source: *mut wlr_client_data_source =
        client_data_source_from_wlr_data_source(wlr_source);
    wl_data_source_send_send((*source).resource, mime_type, fd);
    close(fd);
}
unsafe extern "C" fn client_data_source_destroy(mut wlr_source:
                                                    *mut wlr_data_source) {
    let mut source: *mut wlr_client_data_source =
        client_data_source_from_wlr_data_source(wlr_source);
    wl_data_source_send_cancelled((*source).resource);
    wl_resource_set_user_data((*source).resource, 0 as *mut libc::c_void);
    free(source as *mut libc::c_void);
}
unsafe extern "C" fn client_data_source_dnd_drop(mut wlr_source:
                                                     *mut wlr_data_source) {
    let mut source: *mut wlr_client_data_source =
        client_data_source_from_wlr_data_source(wlr_source);
    if wl_resource_get_version((*source).resource) >= 3i32 {
    } else {
        __assert_fail(b"wl_resource_get_version(source->resource) >= WL_DATA_SOURCE_DND_DROP_PERFORMED_SINCE_VERSION\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/data_device/wlr_data_source.c\x00" as
                          *const u8 as *const libc::c_char,
                      123i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 59],
                                                &[libc::c_char; 59]>(b"void client_data_source_dnd_drop(struct wlr_data_source *)\x00")).as_ptr());
    };
    wl_data_source_send_dnd_drop_performed((*source).resource);
}
unsafe extern "C" fn client_data_source_dnd_finish(mut wlr_source:
                                                       *mut wlr_data_source) {
    let mut source: *mut wlr_client_data_source =
        client_data_source_from_wlr_data_source(wlr_source);
    if wl_resource_get_version((*source).resource) >= 3i32 {
    } else {
        __assert_fail(b"wl_resource_get_version(source->resource) >= WL_DATA_SOURCE_DND_FINISHED_SINCE_VERSION\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/data_device/wlr_data_source.c\x00" as
                          *const u8 as *const libc::c_char,
                      131i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 61],
                                                &[libc::c_char; 61]>(b"void client_data_source_dnd_finish(struct wlr_data_source *)\x00")).as_ptr());
    };
    wl_data_source_send_dnd_finished((*source).resource);
}
unsafe extern "C" fn client_data_source_dnd_action(mut wlr_source:
                                                       *mut wlr_data_source,
                                                   mut action:
                                                       wl_data_device_manager_dnd_action) {
    let mut source: *mut wlr_client_data_source =
        client_data_source_from_wlr_data_source(wlr_source);
    if wl_resource_get_version((*source).resource) >= 3i32 {
    } else {
        __assert_fail(b"wl_resource_get_version(source->resource) >= WL_DATA_SOURCE_ACTION_SINCE_VERSION\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/data_device/wlr_data_source.c\x00" as
                          *const u8 as *const libc::c_char,
                      140i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 101],
                                                &[libc::c_char; 101]>(b"void client_data_source_dnd_action(struct wlr_data_source *, enum wl_data_device_manager_dnd_action)\x00")).as_ptr());
    };
    wl_data_source_send_action((*source).resource, action as uint32_t);
}
unsafe extern "C" fn data_source_destroy(mut client: *mut wl_client,
                                         mut resource: *mut wl_resource) {
    wl_resource_destroy(resource);
}
unsafe extern "C" fn data_source_set_actions(mut client: *mut wl_client,
                                             mut resource: *mut wl_resource,
                                             mut dnd_actions: uint32_t) {
    let mut source: *mut wlr_client_data_source =
        client_data_source_from_resource(resource);
    if source.is_null() { return }
    if (*source).source.actions >= 0i32 {
        wl_resource_post_error((*source).resource,
                               WL_DATA_SOURCE_ERROR_INVALID_ACTION_MASK as
                                   libc::c_int as uint32_t,
                               b"cannot set actions more than once\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    if dnd_actions &
           !(WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY as libc::c_int |
                 WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE as libc::c_int |
                 WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK as libc::c_int) as
               libc::c_uint != 0 {
        wl_resource_post_error((*source).resource,
                               WL_DATA_SOURCE_ERROR_INVALID_ACTION_MASK as
                                   libc::c_int as uint32_t,
                               b"invalid action mask %x\x00" as *const u8 as
                                   *const libc::c_char, dnd_actions);
        return
    }
    if (*source).finalized {
        wl_resource_post_error((*source).resource,
                               WL_DATA_SOURCE_ERROR_INVALID_ACTION_MASK as
                                   libc::c_int as uint32_t,
                               b"invalid action change after wl_data_device.start_drag\x00"
                                   as *const u8 as *const libc::c_char);
        return
    }
    (*source).source.actions = dnd_actions as int32_t;
}
unsafe extern "C" fn data_source_offer(mut client: *mut wl_client,
                                       mut resource: *mut wl_resource,
                                       mut mime_type: *const libc::c_char) {
    let mut source: *mut wlr_client_data_source =
        client_data_source_from_resource(resource);
    if source.is_null() { return }
    if (*source).finalized {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Offering additional MIME type after wl_data_device.set_selection\x00"
                     as *const u8 as *const libc::c_char,
                 b"../types/data_device/wlr_data_source.c\x00" as *const u8 as
                     *const libc::c_char, 190i32);
    }
    let mut mime_type_ptr: *mut *const libc::c_char =
        0 as *mut *const libc::c_char;
    mime_type_ptr =
        (*source).source.mime_types.data as *mut *const libc::c_char;
    while (mime_type_ptr as *const libc::c_char) <
              ((*source).source.mime_types.data as
                   *const libc::c_char).offset((*source).source.mime_types.size
                                                   as isize) {
        if strcmp(*mime_type_ptr, mime_type) == 0i32 {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] Ignoring duplicate MIME type offer %s\x00" as
                         *const u8 as *const libc::c_char,
                     b"../types/data_device/wlr_data_source.c\x00" as
                         *const u8 as *const libc::c_char, 197i32, mime_type);
            return
        }
        mime_type_ptr = mime_type_ptr.offset(1)
    }
    let mut dup_mime_type: *mut libc::c_char = strdup(mime_type);
    if dup_mime_type.is_null() {
        wl_resource_post_no_memory(resource);
        return
    }
    let mut p: *mut *mut libc::c_char =
        wl_array_add(&mut (*source).source.mime_types,
                     ::std::mem::size_of::<*mut libc::c_char>() as
                         libc::c_ulong) as *mut *mut libc::c_char;
    if p.is_null() {
        free(dup_mime_type as *mut libc::c_void);
        wl_resource_post_no_memory(resource);
        return
    }
    *p = dup_mime_type;
}
static mut data_source_impl: wl_data_source_interface =
    unsafe {
        {
            let mut init =
                wl_data_source_interface{offer:
                                             Some(data_source_offer as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wl_client,
                                                                           _:
                                                                               *mut wl_resource,
                                                                           _:
                                                                               *const libc::c_char)
                                                          -> ()),
                                         destroy:
                                             Some(data_source_destroy as
                                                      unsafe extern "C" fn(_:
                                                                               *mut wl_client,
                                                                           _:
                                                                               *mut wl_resource)
                                                          -> ()),
                                         set_actions:
                                             Some(data_source_set_actions as
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
unsafe extern "C" fn data_source_handle_resource_destroy(mut resource:
                                                             *mut wl_resource) {
    let mut source: *mut wlr_client_data_source =
        client_data_source_from_resource(resource);
    if !source.is_null() { wlr_data_source_destroy(&mut (*source).source); }
    wl_list_remove(wl_resource_get_link(resource));
}
#[no_mangle]
pub unsafe extern "C" fn client_data_source_create(mut client: *mut wl_client,
                                                   mut version: uint32_t,
                                                   mut id: uint32_t,
                                                   mut resource_list:
                                                       *mut wl_list)
 -> *mut wlr_client_data_source {
    let mut source: *mut wlr_client_data_source =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_client_data_source>() as
                   libc::c_ulong) as *mut wlr_client_data_source;
    if source.is_null() { return 0 as *mut wlr_client_data_source }
    (*source).resource =
        wl_resource_create(client, &wl_data_source_interface,
                           version as libc::c_int, id);
    if (*source).resource.is_null() {
        wl_resource_post_no_memory((*source).resource);
        free(source as *mut libc::c_void);
        return 0 as *mut wlr_client_data_source
    }
    wl_resource_set_implementation((*source).resource,
                                   &data_source_impl as
                                       *const wl_data_source_interface as
                                       *const libc::c_void,
                                   source as *mut libc::c_void,
                                   Some(data_source_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    wl_list_insert(resource_list, wl_resource_get_link((*source).resource));
    (*source).impl_0.accept =
        Some(client_data_source_accept as
                 unsafe extern "C" fn(_: *mut wlr_data_source, _: uint32_t,
                                      _: *const libc::c_char) -> ());
    (*source).impl_0.send =
        Some(client_data_source_send as
                 unsafe extern "C" fn(_: *mut wlr_data_source,
                                      _: *const libc::c_char, _: int32_t)
                     -> ());
    (*source).impl_0.destroy =
        Some(client_data_source_destroy as
                 unsafe extern "C" fn(_: *mut wlr_data_source) -> ());
    if wl_resource_get_version((*source).resource) >= 3i32 {
        (*source).impl_0.dnd_drop =
            Some(client_data_source_dnd_drop as
                     unsafe extern "C" fn(_: *mut wlr_data_source) -> ())
    }
    if wl_resource_get_version((*source).resource) >= 3i32 {
        (*source).impl_0.dnd_finish =
            Some(client_data_source_dnd_finish as
                     unsafe extern "C" fn(_: *mut wlr_data_source) -> ())
    }
    if wl_resource_get_version((*source).resource) >= 3i32 {
        (*source).impl_0.dnd_action =
            Some(client_data_source_dnd_action as
                     unsafe extern "C" fn(_: *mut wlr_data_source,
                                          _:
                                              wl_data_device_manager_dnd_action)
                         -> ())
    }
    wlr_data_source_init(&mut (*source).source, &mut (*source).impl_0);
    return source;
}
