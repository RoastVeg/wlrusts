use libc;
extern "C" {
    pub type wlr_cursor_state;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wlr_xcursor_theme_get_cursor(theme: *mut wlr_xcursor_theme,
                                    name: *const libc::c_char)
     -> *mut wlr_xcursor;
    #[no_mangle]
    fn wlr_xcursor_theme_destroy(theme: *mut wlr_xcursor_theme);
    #[no_mangle]
    fn wlr_xcursor_theme_load(name: *const libc::c_char, size: libc::c_int)
     -> *mut wlr_xcursor_theme;
    #[no_mangle]
    fn wlr_cursor_set_image(cur: *mut wlr_cursor, pixels: *const uint8_t,
                            stride: int32_t, width: uint32_t,
                            height: uint32_t, hotspot_x: int32_t,
                            hotspot_y: int32_t, scale: libc::c_float);
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;

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
pub struct wlr_cursor {
    pub state: *mut crate::src::types::wlr_cursor::wlr_cursor_state,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
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
    pub touch_up: wl_signal,
    pub touch_down: wl_signal,
    pub touch_motion: wl_signal,
    pub touch_cancel: wl_signal,
    pub tablet_tool_axis: wl_signal,
    pub tablet_tool_proximity: wl_signal,
    pub tablet_tool_tip: wl_signal,
    pub tablet_tool_button: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xcursor_image {
    pub width: uint32_t,
    pub height: uint32_t,
    pub hotspot_x: uint32_t,
    pub hotspot_y: uint32_t,
    pub delay: uint32_t,
    pub buffer: *mut uint8_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xcursor {
    pub image_count: libc::c_uint,
    pub images: *mut *mut wlr_xcursor_image,
    pub name: *mut libc::c_char,
    pub total_delay: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xcursor_theme {
    pub cursor_count: libc::c_uint,
    pub cursors: *mut *mut wlr_xcursor,
    pub name: *mut libc::c_char,
    pub size: libc::c_int,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xcursor_manager_theme {
    pub scale: libc::c_float,
    pub theme: *mut wlr_xcursor_theme,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xcursor_manager {
    pub name: *mut libc::c_char,
    pub size: uint32_t,
    pub scaled_themes: wl_list,
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xcursor_manager_create(mut name:
                                                        *const libc::c_char,
                                                    mut size: uint32_t)
 -> *mut wlr_xcursor_manager {
    let mut manager: *mut wlr_xcursor_manager =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xcursor_manager>() as libc::c_ulong)
            as *mut wlr_xcursor_manager;
    if manager.is_null() { return 0 as *mut wlr_xcursor_manager }
    if !name.is_null() { (*manager).name = strdup(name) }
    (*manager).size = size;
    wl_list_init(&mut (*manager).scaled_themes);
    return manager;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xcursor_manager_destroy(mut manager:
                                                         *mut wlr_xcursor_manager) {
    if manager.is_null() { return }
    let mut theme: *mut wlr_xcursor_manager_theme =
        0 as *mut wlr_xcursor_manager_theme;
    let mut tmp: *mut wlr_xcursor_manager_theme =
        0 as *mut wlr_xcursor_manager_theme;
    theme =
        ((*manager).scaled_themes.next as *mut libc::c_char).offset(-16) as
            *mut wlr_xcursor_manager_theme;
    tmp =
        ((*theme).link.next as *mut libc::c_char).offset(-16) as
            *mut wlr_xcursor_manager_theme;
    while &mut (*theme).link as *mut wl_list !=
              &mut (*manager).scaled_themes as *mut wl_list {
        wl_list_remove(&mut (*theme).link);
        wlr_xcursor_theme_destroy((*theme).theme);
        free(theme as *mut libc::c_void);
        theme = tmp;
        tmp =
            ((*theme).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_xcursor_manager_theme
    }
    free((*manager).name as *mut libc::c_void);
    free(manager as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xcursor_manager_load(mut manager:
                                                      *mut wlr_xcursor_manager,
                                                  mut scale: libc::c_float)
 -> libc::c_int {
    let mut theme: *mut wlr_xcursor_manager_theme =
        0 as *mut wlr_xcursor_manager_theme;
    theme =
        ((*manager).scaled_themes.next as *mut libc::c_char).offset(-16) as
            *mut wlr_xcursor_manager_theme;
    while &mut (*theme).link as *mut wl_list !=
              &mut (*manager).scaled_themes as *mut wl_list {
        if (*theme).scale == scale { return 0i32 }
        theme =
            ((*theme).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_xcursor_manager_theme
    }
    theme =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xcursor_manager_theme>() as
                   libc::c_ulong) as *mut wlr_xcursor_manager_theme;
    if theme.is_null() { return 1i32 }
    (*theme).scale = scale;
    (*theme).theme =
        wlr_xcursor_theme_load((*manager).name,
                               ((*manager).size as libc::c_float * scale) as
                                   libc::c_int);
    if (*theme).theme.is_null() {
        free(theme as *mut libc::c_void);
        return 1i32
    }
    wl_list_insert(&mut (*manager).scaled_themes, &mut (*theme).link);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xcursor_manager_get_xcursor(mut manager:
                                                             *mut wlr_xcursor_manager,
                                                         mut name:
                                                             *const libc::c_char,
                                                         mut scale:
                                                             libc::c_float)
 -> *mut wlr_xcursor {
    let mut theme: *mut wlr_xcursor_manager_theme =
        0 as *mut wlr_xcursor_manager_theme;
    theme =
        ((*manager).scaled_themes.next as *mut libc::c_char).offset(-16) as
            *mut wlr_xcursor_manager_theme;
    while &mut (*theme).link as *mut wl_list !=
              &mut (*manager).scaled_themes as *mut wl_list {
        if (*theme).scale == scale {
            return wlr_xcursor_theme_get_cursor((*theme).theme, name)
        }
        theme =
            ((*theme).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_xcursor_manager_theme
    }
    return 0 as *mut wlr_xcursor;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * An XCursor theme at a particular scale factor of the base size.
 */
/* *
 * wlr_xcursor_manager dynamically loads xcursor themes at sizes necessary for
 * use on outputs at arbitrary scale factors. You should call
 * wlr_xcursor_manager_load for each output you will show your cursor on, with
 * the scale factor parameter set to that output's scale factor.
 */
// wlr_xcursor_manager_theme::link
/* *
 * Creates a new XCursor manager with the given xcursor theme name and base size
 * (for use when scale=1).
 */
/* *
 * Ensures an xcursor theme at the given scale factor is loaded in the manager.
 */
/* *
 * Retrieves a wlr_xcursor reference for the given cursor name at the given
 * scale factor, or NULL if this wlr_xcursor_manager has not loaded a cursor
 * theme at the requested scale.
 */
/* *
 * Set a wlr_cursor's cursor image to the specified cursor name for all scale
 * factors. wlr_cursor will take over from this point and ensure the correct
 * cursor is used on each output, assuming a wlr_output_layout is attached to
 * it.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_xcursor_manager_set_cursor_image(mut manager:
                                                                  *mut wlr_xcursor_manager,
                                                              mut name:
                                                                  *const libc::c_char,
                                                              mut cursor:
                                                                  *mut wlr_cursor) {
    let mut theme: *mut wlr_xcursor_manager_theme =
        0 as *mut wlr_xcursor_manager_theme;
    theme =
        ((*manager).scaled_themes.next as *mut libc::c_char).offset(-16) as
            *mut wlr_xcursor_manager_theme;
    while &mut (*theme).link as *mut wl_list !=
              &mut (*manager).scaled_themes as *mut wl_list {
        let mut xcursor: *mut wlr_xcursor =
            wlr_xcursor_theme_get_cursor((*theme).theme, name);
        if !xcursor.is_null() {
            let mut image: *mut wlr_xcursor_image =
                *(*xcursor).images.offset(0);
            wlr_cursor_set_image(cursor, (*image).buffer,
                                 (*image).width.wrapping_mul(4i32 as
                                                                 libc::c_uint)
                                     as int32_t, (*image).width,
                                 (*image).height,
                                 (*image).hotspot_x as int32_t,
                                 (*image).hotspot_y as int32_t,
                                 (*theme).scale);
        }
        theme =
            ((*theme).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_xcursor_manager_theme
    };
}
