use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type udev;
    pub type udev_monitor;
    pub type session_impl;
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
    fn wl_list_length(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wl_display_add_destroy_listener(display: *mut wl_display,
                                       listener: *mut wl_listener);
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* *
 * Initializes common state on a wlr_backend and sets the implementation to the
 * provided wlr_backend_impl reference.
 */
    #[no_mangle]
    fn wlr_backend_init(backend: *mut wlr_backend,
                        impl_0: *const wlr_backend_impl);
    /*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
    /* * Raised when destroyed, passed the wlr_backend reference */
    /* * Raised when new inputs are added, passed the wlr_input_device */
    /* * Raised when new outputs are added, passed the wlr_output */
    /* *
 * Automatically initializes the most suitable backend given the environment.
 * Will always return a multibackend. The backend is created but not started.
 * Returns NULL on failure.
 *
 * The compositor can request to initialize the backend's renderer by setting
 * the create_render_func. The callback must initialize the given wlr_egl and
 * return a valid wlr_renderer, or NULL if it has failed to initiaze it.
 * Pass NULL as create_renderer_func to use the backend's default renderer.
 */
    /* *
 * Start the backend. This may signal new_input or new_output immediately, but
 * may also wait until the display's event loop begins. Returns false on
 * failure.
 */
    /* *
 * Destroy the backend and clean up all of its resources. Normally called
 * automatically when the wl_display is destroyed.
 */
    /* *
 * Obtains the wlr_renderer reference this backend is using.
 */
    /* *
 * Obtains the wlr_session reference from this backend if there is any.
 * Might return NULL for backends that don't use a session.
 */
    /* *
 * Returns the clock used by the backend for presentation feedback.
 */
    #[no_mangle]
    fn wlr_backend_get_presentation_clock(backend: *mut wlr_backend)
     -> clockid_t;
    #[no_mangle]
    fn wlr_backend_get_renderer(backend: *mut wlr_backend)
     -> *mut wlr_renderer;
    #[no_mangle]
    fn wlr_backend_destroy(backend: *mut wlr_backend);
    #[no_mangle]
    fn wlr_backend_start(backend: *mut wlr_backend) -> bool;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type __clockid_t = libc::c_int;
pub type clockid_t = __clockid_t;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_session {
    pub impl_0: *const session_impl,
    pub session_signal: wl_signal,
    pub active: bool,
    pub vtnr: libc::c_uint,
    pub seat: [libc::c_char; 256],
    pub udev: *mut udev,
    pub mon: *mut udev_monitor,
    pub udev_event: *mut wl_event_source,
    pub devices: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
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
pub struct wlr_multi_backend {
    pub backend: wlr_backend,
    pub session: *mut wlr_session,
    pub backends: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_1,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub backend_add: wl_signal,
    pub backend_remove: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct subbackend_state {
    pub backend: *mut wlr_backend,
    pub container: *mut wlr_backend,
    pub new_input: wl_listener,
    pub new_output: wl_listener,
    pub destroy: wl_listener,
    pub link: wl_list,
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
unsafe extern "C" fn multi_backend_from_backend(mut wlr_backend:
                                                    *mut wlr_backend)
 -> *mut wlr_multi_backend {
    if wlr_backend_is_multi(wlr_backend) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_backend_is_multi(wlr_backend)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/multi/backend.c\x00" as *const u8 as
                          *const libc::c_char, 23i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 75],
                                                &[libc::c_char; 75]>(b"struct wlr_multi_backend *multi_backend_from_backend(struct wlr_backend *)\x00")).as_ptr());
    };
    return wlr_backend as *mut wlr_multi_backend;
}
unsafe extern "C" fn multi_backend_start(mut wlr_backend: *mut wlr_backend)
 -> bool {
    let mut backend: *mut wlr_multi_backend =
        multi_backend_from_backend(wlr_backend);
    let mut sub: *mut subbackend_state = 0 as *mut subbackend_state;
    sub =
        ((*backend).backends.next as *mut libc::c_char).offset(-88) as
            *mut subbackend_state;
    while &mut (*sub).link as *mut wl_list !=
              &mut (*backend).backends as *mut wl_list {
        if !wlr_backend_start((*sub).backend) {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to initialize backend.\x00" as *const u8
                         as *const libc::c_char,
                     b"../backend/multi/backend.c\x00" as *const u8 as
                         *const libc::c_char, 32i32);
            return 0i32 != 0
        }
        sub =
            ((*sub).link.next as *mut libc::c_char).offset(-88) as
                *mut subbackend_state
    }
    return 1i32 != 0;
}
unsafe extern "C" fn subbackend_state_destroy(mut sub:
                                                  *mut subbackend_state) {
    wl_list_remove(&mut (*sub).new_input.link);
    wl_list_remove(&mut (*sub).new_output.link);
    wl_list_remove(&mut (*sub).destroy.link);
    wl_list_remove(&mut (*sub).link);
    free(sub as *mut libc::c_void);
}
unsafe extern "C" fn multi_backend_destroy(mut wlr_backend:
                                               *mut wlr_backend) {
    let mut backend: *mut wlr_multi_backend =
        multi_backend_from_backend(wlr_backend);
    wl_list_remove(&mut (*backend).display_destroy.link);
    let mut sub: *mut subbackend_state = 0 as *mut subbackend_state;
    let mut next: *mut subbackend_state = 0 as *mut subbackend_state;
    sub =
        ((*backend).backends.next as *mut libc::c_char).offset(-88) as
            *mut subbackend_state;
    next =
        ((*sub).link.next as *mut libc::c_char).offset(-88) as
            *mut subbackend_state;
    while &mut (*sub).link as *mut wl_list !=
              &mut (*backend).backends as *mut wl_list {
        wlr_backend_destroy((*sub).backend);
        sub = next;
        next =
            ((*sub).link.next as *mut libc::c_char).offset(-88) as
                *mut subbackend_state
    }
    // Destroy this backend only after removing all sub-backends
    wlr_signal_emit_safe(&mut (*wlr_backend).events.destroy,
                         backend as *mut libc::c_void);
    free(backend as *mut libc::c_void);
}
unsafe extern "C" fn multi_backend_get_renderer(mut backend: *mut wlr_backend)
 -> *mut wlr_renderer {
    let mut multi: *mut wlr_multi_backend =
        multi_backend_from_backend(backend);
    let mut sub: *mut subbackend_state = 0 as *mut subbackend_state;
    sub =
        ((*multi).backends.next as *mut libc::c_char).offset(-88) as
            *mut subbackend_state;
    while &mut (*sub).link as *mut wl_list !=
              &mut (*multi).backends as *mut wl_list {
        let mut rend: *mut wlr_renderer =
            wlr_backend_get_renderer((*sub).backend);
        if !rend.is_null() { return rend }
        sub =
            ((*sub).link.next as *mut libc::c_char).offset(-88) as
                *mut subbackend_state
    }
    return 0 as *mut wlr_renderer;
}
unsafe extern "C" fn multi_backend_get_session(mut _backend: *mut wlr_backend)
 -> *mut wlr_session {
    let mut backend: *mut wlr_multi_backend =
        multi_backend_from_backend(_backend);
    return (*backend).session;
}
unsafe extern "C" fn multi_backend_get_presentation_clock(mut backend:
                                                              *mut wlr_backend)
 -> clockid_t {
    let mut multi: *mut wlr_multi_backend =
        multi_backend_from_backend(backend);
    let mut sub: *mut subbackend_state = 0 as *mut subbackend_state;
    sub =
        ((*multi).backends.next as *mut libc::c_char).offset(-88) as
            *mut subbackend_state;
    while &mut (*sub).link as *mut wl_list !=
              &mut (*multi).backends as *mut wl_list {
        if (*(*(*sub).backend).impl_0).get_presentation_clock.is_some() {
            return wlr_backend_get_presentation_clock((*sub).backend)
        }
        sub =
            ((*sub).link.next as *mut libc::c_char).offset(-88) as
                *mut subbackend_state
    }
    return 1i32;
}
#[no_mangle]
pub static mut backend_impl: wlr_backend_impl =
    unsafe {
        {
            let mut init =
                wlr_backend_impl{start:
                                     Some(multi_backend_start as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> bool),
                                 destroy:
                                     Some(multi_backend_destroy as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> ()),
                                 get_renderer:
                                     Some(multi_backend_get_renderer as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> *mut wlr_renderer),
                                 get_session:
                                     Some(multi_backend_get_session as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> *mut wlr_session),
                                 get_presentation_clock:
                                     Some(multi_backend_get_presentation_clock
                                              as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_backend)
                                                  -> clockid_t),};
            init
        }
    };
unsafe extern "C" fn handle_display_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut backend: *mut wlr_multi_backend =
        (listener as *mut libc::c_char).offset(-80) as *mut wlr_multi_backend;
    multi_backend_destroy(backend as *mut wlr_backend);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_multi_backend_create(mut display:
                                                      *mut wl_display)
 -> *mut wlr_backend {
    let mut backend: *mut wlr_multi_backend =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_multi_backend>() as libc::c_ulong) as
            *mut wlr_multi_backend;
    if backend.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Backend allocation failed\x00" as *const u8 as
                     *const libc::c_char,
                 b"../backend/multi/backend.c\x00" as *const u8 as
                     *const libc::c_char, 114i32);
        return 0 as *mut wlr_backend
    }
    wl_list_init(&mut (*backend).backends);
    wlr_backend_init(&mut (*backend).backend, &mut backend_impl);
    wl_signal_init(&mut (*backend).events.backend_add);
    wl_signal_init(&mut (*backend).events.backend_remove);
    (*backend).display_destroy.notify =
        Some(handle_display_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_display_add_destroy_listener(display, &mut (*backend).display_destroy);
    return &mut (*backend).backend;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_backend_is_multi(mut b: *mut wlr_backend)
 -> bool {
    return (*b).impl_0 ==
               &mut backend_impl as *mut wlr_backend_impl as
                   *const wlr_backend_impl;
}
unsafe extern "C" fn new_input_reemit(mut listener: *mut wl_listener,
                                      mut data: *mut libc::c_void) {
    let mut state: *mut subbackend_state =
        (listener as *mut libc::c_char).offset(-16) as *mut subbackend_state;
    wlr_signal_emit_safe(&mut (*(*state).container).events.new_input, data);
}
unsafe extern "C" fn new_output_reemit(mut listener: *mut wl_listener,
                                       mut data: *mut libc::c_void) {
    let mut state: *mut subbackend_state =
        (listener as *mut libc::c_char).offset(-40) as *mut subbackend_state;
    wlr_signal_emit_safe(&mut (*(*state).container).events.new_output, data);
}
unsafe extern "C" fn handle_subbackend_destroy(mut listener: *mut wl_listener,
                                               mut data: *mut libc::c_void) {
    let mut state: *mut subbackend_state =
        (listener as *mut libc::c_char).offset(-64) as *mut subbackend_state;
    subbackend_state_destroy(state);
}
unsafe extern "C" fn multi_backend_get_subbackend(mut multi:
                                                      *mut wlr_multi_backend,
                                                  mut backend:
                                                      *mut wlr_backend)
 -> *mut subbackend_state {
    let mut sub: *mut subbackend_state = 0 as *mut subbackend_state;
    sub =
        ((*multi).backends.next as *mut libc::c_char).offset(-88) as
            *mut subbackend_state;
    while &mut (*sub).link as *mut wl_list !=
              &mut (*multi).backends as *mut wl_list {
        if (*sub).backend == backend { return sub }
        sub =
            ((*sub).link.next as *mut libc::c_char).offset(-88) as
                *mut subbackend_state
    }
    return 0 as *mut subbackend_state;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_multi_backend_add(mut _multi: *mut wlr_backend,
                                               mut backend: *mut wlr_backend)
 -> bool {
    let mut multi: *mut wlr_multi_backend =
        multi_backend_from_backend(_multi);
    if !multi_backend_get_subbackend(multi, backend).is_null() {
        // already added
        return 1i32 != 0
    }
    let mut multi_renderer: *mut wlr_renderer =
        multi_backend_get_renderer(&mut (*multi).backend);
    let mut backend_renderer: *mut wlr_renderer =
        wlr_backend_get_renderer(backend);
    if !multi_renderer.is_null() && !backend_renderer.is_null() &&
           multi_renderer != backend_renderer {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not add backend: multiple renderers at the same time aren\'t supported\x00"
                     as *const u8 as *const libc::c_char,
                 b"../backend/multi/backend.c\x00" as *const u8 as
                     *const libc::c_char, 177i32);
        return 0i32 != 0
    }
    let mut sub: *mut subbackend_state =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<subbackend_state>() as libc::c_ulong) as
            *mut subbackend_state;
    if sub.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not add backend: allocation failed\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/multi/backend.c\x00" as *const u8 as
                     *const libc::c_char, 183i32);
        return 0i32 != 0
    }
    wl_list_insert(&mut (*multi).backends, &mut (*sub).link);
    (*sub).backend = backend;
    (*sub).container = &mut (*multi).backend;
    wl_signal_add(&mut (*backend).events.destroy, &mut (*sub).destroy);
    (*sub).destroy.notify =
        Some(handle_subbackend_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*backend).events.new_input, &mut (*sub).new_input);
    (*sub).new_input.notify =
        Some(new_input_reemit as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*backend).events.new_output, &mut (*sub).new_output);
    (*sub).new_output.notify =
        Some(new_output_reemit as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wlr_signal_emit_safe(&mut (*multi).events.backend_add,
                         backend as *mut libc::c_void);
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_multi_backend_remove(mut _multi:
                                                      *mut wlr_backend,
                                                  mut backend:
                                                      *mut wlr_backend) {
    let mut multi: *mut wlr_multi_backend =
        multi_backend_from_backend(_multi);
    let mut sub: *mut subbackend_state =
        multi_backend_get_subbackend(multi, backend);
    if !sub.is_null() {
        wlr_signal_emit_safe(&mut (*multi).events.backend_remove,
                             backend as *mut libc::c_void);
        subbackend_state_destroy(sub);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_multi_is_empty(mut _backend: *mut wlr_backend)
 -> bool {
    if wlr_backend_is_multi(_backend) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_backend_is_multi(_backend)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/multi/backend.c\x00" as *const u8 as
                          *const libc::c_char, 218i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"_Bool wlr_multi_is_empty(struct wlr_backend *)\x00")).as_ptr());
    };
    let mut backend: *mut wlr_multi_backend =
        _backend as *mut wlr_multi_backend;
    return wl_list_length(&mut (*backend).backends) < 1i32;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * Creates a multi-backend. Multi-backends wrap an arbitrary number of backends
 * and aggregate their new_output/new_input signals.
 */
/* *
 * Adds the given backend to the multi backend. This should be done before the
 * new backend is started.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_multi_for_each_backend(mut _backend:
                                                        *mut wlr_backend,
                                                    mut callback:
                                                        Option<unsafe extern "C" fn(_:
                                                                                        *mut wlr_backend,
                                                                                    _:
                                                                                        *mut libc::c_void)
                                                                   -> ()>,
                                                    mut data:
                                                        *mut libc::c_void) {
    if wlr_backend_is_multi(_backend) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_backend_is_multi(_backend)\x00" as *const u8 as
                          *const libc::c_char,
                      b"../backend/multi/backend.c\x00" as *const u8 as
                          *const libc::c_char, 225i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 102],
                                                &[libc::c_char; 102]>(b"void wlr_multi_for_each_backend(struct wlr_backend *, void (*)(struct wlr_backend *, void *), void *)\x00")).as_ptr());
    };
    let mut backend: *mut wlr_multi_backend =
        _backend as *mut wlr_multi_backend;
    let mut sub: *mut subbackend_state = 0 as *mut subbackend_state;
    sub =
        ((*backend).backends.next as *mut libc::c_char).offset(-88) as
            *mut subbackend_state;
    while &mut (*sub).link as *mut wl_list !=
              &mut (*backend).backends as *mut wl_list {
        callback.expect("non-null function pointer")((*sub).backend, data);
        sub =
            ((*sub).link.next as *mut libc::c_char).offset(-88) as
                *mut subbackend_state
    };
}
