use libc;
extern "C" {
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
}
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
unsafe extern "C" fn handle_noop(mut listener: *mut wl_listener,
                                 mut data: *mut libc::c_void) {
    // Do nothing
}
#[no_mangle]
pub unsafe extern "C" fn wlr_signal_emit_safe(mut signal: *mut wl_signal,
                                              mut data: *mut libc::c_void) {
    let mut cursor: wl_listener =
        wl_listener{link:
                        wl_list{prev: 0 as *mut wl_list,
                                next: 0 as *mut wl_list,},
                    notify: None,};
    let mut end: wl_listener =
        wl_listener{link:
                        wl_list{prev: 0 as *mut wl_list,
                                next: 0 as *mut wl_list,},
                    notify: None,};
    /* Add two special markers: one cursor and one end marker. This way, we know
	 * that we've already called listeners on the left of the cursor and that we
	 * don't want to call listeners on the right of the end marker. The 'it'
	 * function can remove any element it wants from the list without troubles.
	 * wl_list_for_each_safe tries to be safe but it fails: it works fine
	 * if the current item is removed, but not if the next one is. */
    wl_list_insert(&mut (*signal).listener_list, &mut cursor.link);
    cursor.notify =
        Some(handle_noop as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_list_insert((*signal).listener_list.prev, &mut end.link);
    end.notify =
        Some(handle_noop as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    while cursor.link.next != &mut end.link as *mut wl_list {
        let mut pos: *mut wl_list = cursor.link.next;
        let mut l: *mut wl_listener =
            (pos as *mut libc::c_char).offset(-0) as *mut wl_listener;
        wl_list_remove(&mut cursor.link);
        wl_list_insert(pos, &mut cursor.link);
        (*l).notify.expect("non-null function pointer")(l, data);
    }
    wl_list_remove(&mut cursor.link);
    wl_list_remove(&mut end.link);
}
