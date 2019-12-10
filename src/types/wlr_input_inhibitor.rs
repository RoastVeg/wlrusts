use libc;
extern "C" {
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
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
    fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;
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
    static zwlr_input_inhibitor_v1_interface: wl_interface;
    #[no_mangle]
    static zwlr_input_inhibit_manager_v1_interface: wl_interface;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type __uint32_t = libc::c_uint;
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_input_inhibit_manager {
    pub global: *mut wl_global,
    pub active_client: *mut wl_client,
    pub active_inhibitor: *mut wl_resource,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub activate: wl_signal,
    pub deactivate: wl_signal,
    pub destroy: wl_signal,
}
/* *
 * @ingroup iface_zwlr_input_inhibit_manager_v1
 */
/* *
 * @ingroup iface_zwlr_input_inhibitor_v1
 * @struct zwlr_input_inhibitor_v1_interface
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwlr_input_inhibitor_v1_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
}
/* *
 * @page page_iface_zwlr_input_inhibit_manager_v1 zwlr_input_inhibit_manager_v1
 * @section page_iface_zwlr_input_inhibit_manager_v1_desc Description
 *
 * Clients can use this interface to prevent input events from being sent to
 * any surfaces but its own, which is useful for example in lock screen
 * software. It is assumed that access to this interface will be locked down
 * to whitelisted clients by the compositor.
 * @section page_iface_zwlr_input_inhibit_manager_v1_api API
 * See @ref iface_zwlr_input_inhibit_manager_v1.
 */
/* *
 * @defgroup iface_zwlr_input_inhibit_manager_v1 The zwlr_input_inhibit_manager_v1 interface
 *
 * Clients can use this interface to prevent input events from being sent to
 * any surfaces but its own, which is useful for example in lock screen
 * software. It is assumed that access to this interface will be locked down
 * to whitelisted clients by the compositor.
 */
/* *
 * @page page_iface_zwlr_input_inhibitor_v1 zwlr_input_inhibitor_v1
 * @section page_iface_zwlr_input_inhibitor_v1_desc Description
 *
 * While this resource exists, input to clients other than the owner of the
 * inhibitor resource will not receive input events. The client that owns
 * this resource will receive all input events normally. The compositor will
 * also disable all of its own input processing (such as keyboard shortcuts)
 * while the inhibitor is active.
 *
 * The compositor may continue to send input events to selected clients,
 * such as an on-screen keyboard (via the input-method protocol).
 * @section page_iface_zwlr_input_inhibitor_v1_api API
 * See @ref iface_zwlr_input_inhibitor_v1.
 */
/* *
 * @defgroup iface_zwlr_input_inhibitor_v1 The zwlr_input_inhibitor_v1 interface
 *
 * While this resource exists, input to clients other than the owner of the
 * inhibitor resource will not receive input events. The client that owns
 * this resource will receive all input events normally. The compositor will
 * also disable all of its own input processing (such as keyboard shortcuts)
 * while the inhibitor is active.
 *
 * The compositor may continue to send input events to selected clients,
 * such as an on-screen keyboard (via the input-method protocol).
 */
/* *
	 * an input inhibitor is already in use on the compositor
	 */
/* ZWLR_INPUT_INHIBIT_MANAGER_V1_ERROR_ENUM */
/* *
 * @ingroup iface_zwlr_input_inhibit_manager_v1
 * @struct zwlr_input_inhibit_manager_v1_interface
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct zwlr_input_inhibit_manager_v1_interface {
    pub get_inhibitor: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                   _: *mut wl_resource,
                                                   _: uint32_t) -> ()>,
}
pub const ZWLR_INPUT_INHIBIT_MANAGER_V1_ERROR_ALREADY_INHIBITED:
          zwlr_input_inhibit_manager_v1_error =
    0;
pub type zwlr_input_inhibit_manager_v1_error = libc::c_uint;
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
unsafe extern "C" fn input_inhibit_manager_from_resource(mut resource:
                                                             *mut wl_resource)
 -> *mut wlr_input_inhibit_manager {
    if wl_resource_instance_of(resource,
                               &zwlr_input_inhibit_manager_v1_interface,
                               &inhibit_manager_implementation as
                                   *const zwlr_input_inhibit_manager_v1_interface
                                   as *const libc::c_void) != 0 ||
           wl_resource_instance_of(resource,
                                   &zwlr_input_inhibitor_v1_interface,
                                   &mut input_inhibitor_implementation as
                                       *mut zwlr_input_inhibitor_v1_interface
                                       as *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &zwlr_input_inhibit_manager_v1_interface, &inhibit_manager_implementation) || wl_resource_instance_of(resource, &zwlr_input_inhibitor_v1_interface, &input_inhibitor_implementation)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_input_inhibitor.c\x00" as *const u8 as
                          *const libc::c_char, 19i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 92],
                                                &[libc::c_char; 92]>(b"struct wlr_input_inhibit_manager *input_inhibit_manager_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as
               *mut wlr_input_inhibit_manager;
}
unsafe extern "C" fn input_inhibit_manager_deactivate(mut manager:
                                                          *mut wlr_input_inhibit_manager) {
    if (*manager).active_client.is_null() &&
           (*manager).active_inhibitor.is_null() {
        return
    }
    (*manager).active_client = 0 as *mut wl_client;
    (*manager).active_inhibitor = 0 as *mut wl_resource;
    wlr_signal_emit_safe(&mut (*manager).events.deactivate,
                         manager as *mut libc::c_void);
}
unsafe extern "C" fn input_inhibitor_destroy(mut client: *mut wl_client,
                                             mut resource: *mut wl_resource) {
    let mut manager: *mut wlr_input_inhibit_manager =
        input_inhibit_manager_from_resource(resource);
    input_inhibit_manager_deactivate(manager);
    wl_resource_destroy(resource);
}
unsafe extern "C" fn input_inhibitor_resource_destroy(mut resource:
                                                          *mut wl_resource) {
    let mut manager: *mut wlr_input_inhibit_manager =
        input_inhibit_manager_from_resource(resource);
    input_inhibit_manager_deactivate(manager);
}
static mut input_inhibitor_implementation: zwlr_input_inhibitor_v1_interface =
    {
    
        {
            let mut init =
                zwlr_input_inhibitor_v1_interface{destroy:
                                                      Some(input_inhibitor_destroy
                                                               as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut wl_client,
                                                                                    _:
                                                                                        *mut wl_resource)
                                                                   -> ()),};
            init
        }
};
unsafe extern "C" fn inhibit_manager_get_inhibitor(mut client: *mut wl_client,
                                                   mut resource:
                                                       *mut wl_resource,
                                                   mut id: uint32_t) {
    let mut manager: *mut wlr_input_inhibit_manager =
        input_inhibit_manager_from_resource(resource);
    if !(*manager).active_client.is_null() ||
           !(*manager).active_inhibitor.is_null() {
        wl_resource_post_error(resource,
                               ZWLR_INPUT_INHIBIT_MANAGER_V1_ERROR_ALREADY_INHIBITED
                                   as libc::c_int as uint32_t,
                               b"this compositor already has input inhibited\x00"
                                   as *const u8 as *const libc::c_char);
        return
    }
    let mut wl_resource: *mut wl_resource =
        wl_resource_create(client, &zwlr_input_inhibitor_v1_interface,
                           wl_resource_get_version(resource), id);
    if wl_resource.is_null() { wl_client_post_no_memory(client); }
    wl_resource_set_implementation(wl_resource,
                                   &mut input_inhibitor_implementation as
                                       *mut zwlr_input_inhibitor_v1_interface
                                       as *const libc::c_void,
                                   manager as *mut libc::c_void,
                                   Some(input_inhibitor_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    (*manager).active_client = client;
    (*manager).active_inhibitor = wl_resource;
    wlr_signal_emit_safe(&mut (*manager).events.activate,
                         manager as *mut libc::c_void);
}
static mut inhibit_manager_implementation:
       zwlr_input_inhibit_manager_v1_interface =
    {
    
        {
            let mut init =
                zwlr_input_inhibit_manager_v1_interface{get_inhibitor:
                                                            Some(inhibit_manager_get_inhibitor
                                                                     as
                                                                     unsafe extern "C" fn(_:
                                                                                              *mut wl_client,
                                                                                          _:
                                                                                              *mut wl_resource,
                                                                                          _:
                                                                                              uint32_t)
                                                                         ->
                                                                             ()),};
            init
        }
};
unsafe extern "C" fn input_manager_resource_destroy(mut resource:
                                                        *mut wl_resource) {
    let mut manager: *mut wlr_input_inhibit_manager =
        input_inhibit_manager_from_resource(resource);
    let mut client: *mut wl_client = wl_resource_get_client(resource);
    if (*manager).active_client == client {
        input_inhibit_manager_deactivate(manager);
    };
}
unsafe extern "C" fn inhibit_manager_bind(mut wl_client: *mut wl_client,
                                          mut data: *mut libc::c_void,
                                          mut version: uint32_t,
                                          mut id: uint32_t) {
    let mut manager: *mut wlr_input_inhibit_manager =
        data as *mut wlr_input_inhibit_manager;
    if !wl_client.is_null() && !manager.is_null() {
    } else {
        __assert_fail(b"wl_client && manager\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_input_inhibitor.c\x00" as *const u8 as
                          *const libc::c_char, 93i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"void inhibit_manager_bind(struct wl_client *, void *, uint32_t, uint32_t)\x00")).as_ptr());
    };
    let mut wl_resource: *mut wl_resource =
        wl_resource_create(wl_client,
                           &zwlr_input_inhibit_manager_v1_interface,
                           version as libc::c_int, id);
    if wl_resource.is_null() { wl_client_post_no_memory(wl_client); return }
    wl_resource_set_implementation(wl_resource,
                                   &inhibit_manager_implementation as
                                       *const zwlr_input_inhibit_manager_v1_interface
                                       as *const libc::c_void,
                                   manager as *mut libc::c_void,
                                   Some(input_manager_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
}
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut manager: *mut wlr_input_inhibit_manager =
        (listener as *mut libc::c_char).offset(-24) as
            *mut wlr_input_inhibit_manager;
    wlr_signal_emit_safe(&mut (*manager).events.destroy,
                         manager as *mut libc::c_void);
    wl_list_remove(&mut (*manager).display_destroy.link);
    wl_global_destroy((*manager).global);
    free(manager as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_input_inhibit_manager_create(mut display:
                                                              *mut wl_display)
 -> *mut wlr_input_inhibit_manager {
    // TODO: Client destroy
    let mut manager: *mut wlr_input_inhibit_manager =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_input_inhibit_manager>() as
                   libc::c_ulong) as *mut wlr_input_inhibit_manager;
    if manager.is_null() { return 0 as *mut wlr_input_inhibit_manager }
    (*manager).global =
        wl_global_create(display, &zwlr_input_inhibit_manager_v1_interface,
                         1i32, manager as *mut libc::c_void,
                         Some(inhibit_manager_bind as
                                  unsafe extern "C" fn(_: *mut wl_client,
                                                       _: *mut libc::c_void,
                                                       _: uint32_t,
                                                       _: uint32_t) -> ()));
    if (*manager).global.is_null() {
        free(manager as *mut libc::c_void);
        return 0 as *mut wlr_input_inhibit_manager
    }
    wl_signal_init(&mut (*manager).events.activate);
    wl_signal_init(&mut (*manager).events.deactivate);
    wl_signal_init(&mut (*manager).events.destroy);
    (*manager).display_destroy.notify =
        Some(handle_display_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_display_add_destroy_listener(display, &mut (*manager).display_destroy);
    return manager;
}
