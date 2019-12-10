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
/* Note: these are circular dependencies */
// Or 0 if not applicable to this device
/* wlr_input_device.type determines which of these is valid */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_switch {
    pub impl_0: *mut wlr_switch_impl,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub toggle: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_switch_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_switch) -> ()>,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_switch_init(mut switch_device: *mut wlr_switch,
                                         mut impl_0: *mut wlr_switch_impl) {
    (*switch_device).impl_0 = impl_0;
    wl_signal_init(&mut (*switch_device).events.toggle);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_switch_destroy(mut switch_device:
                                                *mut wlr_switch) {
    if switch_device.is_null() { return }
    if !(*switch_device).impl_0.is_null() &&
           (*(*switch_device).impl_0).destroy.is_some() {
        (*(*switch_device).impl_0).destroy.expect("non-null function pointer")(switch_device);
    } else { free(switch_device as *mut libc::c_void); };
}
