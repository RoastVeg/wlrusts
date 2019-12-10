use libc;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub motion: wl_signal,
    pub motion_absolute: wl_signal,
    pub button: wl_signal,
    pub axis: wl_signal,
    pub frame: wl_signal,
    pub swipe_begin: wl_signal,
    pub swipe_update: wl_signal,
    pub swipe_end: wl_signal,
    pub pinch_begin: wl_signal,
    pub pinch_update: wl_signal,
    pub pinch_end: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_pointer) -> ()>,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_pointer_init(mut pointer: *mut wlr_pointer,
                                          mut impl_0:
                                              *const wlr_pointer_impl) {
    (*pointer).impl_0 = impl_0;
    wl_signal_init(&mut (*pointer).events.motion);
    wl_signal_init(&mut (*pointer).events.motion_absolute);
    wl_signal_init(&mut (*pointer).events.button);
    wl_signal_init(&mut (*pointer).events.axis);
    wl_signal_init(&mut (*pointer).events.frame);
    wl_signal_init(&mut (*pointer).events.swipe_begin);
    wl_signal_init(&mut (*pointer).events.swipe_update);
    wl_signal_init(&mut (*pointer).events.swipe_end);
    wl_signal_init(&mut (*pointer).events.pinch_begin);
    wl_signal_init(&mut (*pointer).events.pinch_update);
    wl_signal_init(&mut (*pointer).events.pinch_end);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_pointer_destroy(mut pointer: *mut wlr_pointer) {
    if pointer.is_null() { return }
    if !(*pointer).impl_0.is_null() && (*(*pointer).impl_0).destroy.is_some()
       {
        (*(*pointer).impl_0).destroy.expect("non-null function pointer")(pointer);
    } else { free(pointer as *mut libc::c_void); };
}
