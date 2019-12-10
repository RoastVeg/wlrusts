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
    #[no_mangle]
    fn wlr_list_init(list: *mut wlr_list) -> bool;
}
pub type size_t = libc::c_ulong;
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
pub struct wlr_tablet_pad {
    pub impl_0: *mut wlr_tablet_pad_impl,
    pub events: C2RustUnnamed,
    pub button_count: size_t,
    pub ring_count: size_t,
    pub strip_count: size_t,
    pub groups: wl_list,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_list {
    pub capacity: size_t,
    pub length: size_t,
    pub items: *mut *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub button: wl_signal,
    pub ring: wl_signal,
    pub strip: wl_signal,
    pub attach_tablet: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet_pad_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_tablet_pad) -> ()>,
}
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_tablet_pad_init(mut pad: *mut wlr_tablet_pad,
                                             mut impl_0:
                                                 *mut wlr_tablet_pad_impl) {
    (*pad).impl_0 = impl_0;
    wl_signal_init(&mut (*pad).events.button);
    wl_signal_init(&mut (*pad).events.ring);
    wl_signal_init(&mut (*pad).events.strip);
    wl_signal_init(&mut (*pad).events.attach_tablet);
    wl_list_init(&mut (*pad).groups);
    wlr_list_init(&mut (*pad).paths);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_tablet_pad_destroy(mut pad:
                                                    *mut wlr_tablet_pad) {
    if pad.is_null() { return }
    wlr_list_for_each(&mut (*pad).paths,
                      Some(free as
                               unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> ()));
    wlr_list_finish(&mut (*pad).paths);
    if !(*pad).impl_0.is_null() && (*(*pad).impl_0).destroy.is_some() {
        (*(*pad).impl_0).destroy.expect("non-null function pointer")(pad);
    } else { free(pad as *mut libc::c_void); };
}
