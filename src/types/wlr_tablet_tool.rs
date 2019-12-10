use libc;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wlr_list_for_each(list: *mut wlr_list,
                         callback:
                             Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                        -> ()>);
    #[no_mangle]
    fn wlr_list_finish(list: *mut wlr_list);
}
pub type size_t = libc::c_ulong;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_signal {
    pub listener_list: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_list {
    pub capacity: size_t,
    pub length: size_t,
    pub items: *mut *mut libc::c_void,
}
/* Note: these are circular dependencies */
// Or 0 if not applicable to this device
/* wlr_input_device.type determines which of these is valid */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet {
    pub impl_0: *mut wlr_tablet_impl,
    pub events: C2RustUnnamed,
    pub name: *mut libc::c_char,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub axis: wl_signal,
    pub proximity: wl_signal,
    pub tip: wl_signal,
    pub button: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_tablet_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_tablet) -> ()>,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_tablet_init(mut tablet: *mut wlr_tablet,
                                         mut impl_0: *mut wlr_tablet_impl) {
    (*tablet).impl_0 = impl_0;
    wl_signal_init(&mut (*tablet).events.axis);
    wl_signal_init(&mut (*tablet).events.proximity);
    wl_signal_init(&mut (*tablet).events.tip);
    wl_signal_init(&mut (*tablet).events.button);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_tablet_destroy(mut tablet: *mut wlr_tablet) {
    if tablet.is_null() { return }
    wlr_list_for_each(&mut (*tablet).paths,
                      Some(free as
                               unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> ()));
    wlr_list_finish(&mut (*tablet).paths);
    if !(*tablet).impl_0.is_null() && (*(*tablet).impl_0).destroy.is_some() {
        (*(*tablet).impl_0).destroy.expect("non-null function pointer")(tablet);
    } else { free(tablet as *mut libc::c_void); };
}
