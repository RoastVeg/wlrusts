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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_touch_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_touch) -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_touch {
    pub impl_0: *const wlr_touch_impl,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub down: wl_signal,
    pub up: wl_signal,
    pub motion: wl_signal,
    pub cancel: wl_signal,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_touch_init(mut touch: *mut wlr_touch,
                                        mut impl_0: *const wlr_touch_impl) {
    (*touch).impl_0 = impl_0;
    wl_signal_init(&mut (*touch).events.down);
    wl_signal_init(&mut (*touch).events.up);
    wl_signal_init(&mut (*touch).events.motion);
    wl_signal_init(&mut (*touch).events.cancel);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_touch_destroy(mut touch: *mut wlr_touch) {
    if !touch.is_null() && !(*touch).impl_0.is_null() &&
           (*(*touch).impl_0).destroy.is_some() {
        (*(*touch).impl_0).destroy.expect("non-null function pointer")(touch);
    } else { free(touch as *mut libc::c_void); };
}
