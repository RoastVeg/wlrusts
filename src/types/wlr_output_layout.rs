use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_texture;
    pub type wlr_surface;
    pub type wlr_backend;
    pub type wlr_output_impl;
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
    fn wl_list_empty(list: *const wl_list) -> libc::c_int;
    #[no_mangle]
    fn wlr_box_contains_point(box_0: *const wlr_box, x: libc::c_double,
                              y: libc::c_double) -> bool;
    #[no_mangle]
    fn wlr_box_intersection(dest: *mut wlr_box, box_a: *const wlr_box,
                            box_b: *const wlr_box) -> bool;
    #[no_mangle]
    fn wlr_box_closest_point(box_0: *const wlr_box, x: libc::c_double,
                             y: libc::c_double, dest_x: *mut libc::c_double,
                             dest_y: *mut libc::c_double);
    #[no_mangle]
    fn wlr_output_create_global(output: *mut wlr_output);
    #[no_mangle]
    fn wlr_output_destroy_global(output: *mut wlr_output);
    #[no_mangle]
    fn wlr_output_effective_resolution(output: *mut wlr_output,
                                       width: *mut libc::c_int,
                                       height: *mut libc::c_int);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
/*
 * 32 bit regions
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;
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
pub type wl_output_subpixel = libc::c_uint;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: wl_output_subpixel = 5;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: wl_output_subpixel = 4;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: wl_output_subpixel = 3;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: wl_output_subpixel = 2;
pub const WL_OUTPUT_SUBPIXEL_NONE: wl_output_subpixel = 1;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: wl_output_subpixel = 0;
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * A client buffer.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_cursor {
    pub output: *mut wlr_output,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub enabled: bool,
    pub visible: bool,
    pub width: uint32_t,
    pub height: uint32_t,
    pub hotspot_x: int32_t,
    pub hotspot_y: int32_t,
    pub link: wl_list,
    pub texture: *mut wlr_texture,
    pub surface: *mut wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output {
    pub impl_0: *const wlr_output_impl,
    pub backend: *mut wlr_backend,
    pub display: *mut wl_display,
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub name: [libc::c_char; 24],
    pub make: [libc::c_char; 56],
    pub model: [libc::c_char; 16],
    pub serial: [libc::c_char; 16],
    pub phys_width: int32_t,
    pub phys_height: int32_t,
    pub modes: wl_list,
    pub current_mode: *mut wlr_output_mode,
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub enabled: bool,
    pub scale: libc::c_float,
    pub subpixel: wl_output_subpixel,
    pub transform: wl_output_transform,
    pub needs_frame: bool,
    pub damage: pixman_region32_t,
    pub frame_pending: bool,
    pub transform_matrix: [libc::c_float; 9],
    pub pending: wlr_output_state,
    pub commit_seq: uint32_t,
    pub events: C2RustUnnamed_0,
    pub idle_frame: *mut wl_event_source,
    pub idle_done: *mut wl_event_source,
    pub attach_render_locks: libc::c_int,
    pub cursors: wl_list,
    pub hardware_cursor: *mut wlr_output_cursor,
    pub software_cursor_locks: libc::c_int,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub frame: wl_signal,
    pub needs_frame: wl_signal,
    pub precommit: wl_signal,
    pub commit: wl_signal,
    pub present: wl_signal,
    pub enable: wl_signal,
    pub mode: wl_signal,
    pub scale: wl_signal,
    pub transform: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub damage: pixman_region32_t,
    pub buffer_type: wlr_output_state_buffer_type,
    pub buffer: *mut wlr_buffer,
}
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_layout_state {
    pub _box: wlr_box,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub state: *mut wlr_output_layout_state,
    pub events: C2RustUnnamed_1,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_layout_output_state {
    pub layout: *mut wlr_output_layout,
    pub l_output: *mut wlr_output_layout_output,
    pub _box: wlr_box,
    pub auto_configured: bool,
    pub mode: wl_listener,
    pub scale: wl_listener,
    pub transform: wl_listener,
    pub output_destroy: wl_listener,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_layout_output {
    pub output: *mut wlr_output,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub link: wl_list,
    pub state: *mut wlr_output_layout_output_state,
    pub events: C2RustUnnamed_2,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * Creates a wlr_output_layout, which can be used to describing outputs in
 * physical space relative to one another, and perform various useful operations
 * on that state.
 */
/* *
 * Given x and y in layout coordinates, adjusts them to local output
 * coordinates relative to the given reference output.
 */
/* *
 * Get the closest point on this layout from the given point from the reference
 * output. If reference is NULL, gets the closest point from the entire layout.
 */
/* *
 * Get the box of the layout for the given reference output in layout
 * coordinates. If `reference` is NULL, the box will be for the extents of the
 * entire layout.
 */
/* *
* Add an auto configured output to the layout. This will place the output in a
* sensible location in the layout. The coordinates of the output in the layout
* may adjust dynamically when the layout changes. If the output is already in
* the layout, it will become auto configured. If the position of the output is
* set such as with `wlr_output_layout_move()`, the output will become manually
* configured.
*/
/* *
 * Get the output closest to the center of the layout extents.
 */
pub type wlr_direction = libc::c_uint;
pub const WLR_DIRECTION_RIGHT: wlr_direction = 8;
pub const WLR_DIRECTION_LEFT: wlr_direction = 4;
pub const WLR_DIRECTION_DOWN: wlr_direction = 2;
pub const WLR_DIRECTION_UP: wlr_direction = 1;
pub type distance_selection_method = libc::c_uint;
pub const FARTHEST: distance_selection_method = 1;
pub const NEAREST: distance_selection_method = 0;
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_create()
 -> *mut wlr_output_layout {
    let mut layout: *mut wlr_output_layout =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_output_layout>() as libc::c_ulong) as
            *mut wlr_output_layout;
    if layout.is_null() { return 0 as *mut wlr_output_layout }
    (*layout).state =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_output_layout_state>() as
                   libc::c_ulong) as *mut wlr_output_layout_state;
    if (*layout).state.is_null() {
        free(layout as *mut libc::c_void);
        return 0 as *mut wlr_output_layout
    }
    wl_list_init(&mut (*layout).outputs);
    wl_signal_init(&mut (*layout).events.add);
    wl_signal_init(&mut (*layout).events.change);
    wl_signal_init(&mut (*layout).events.destroy);
    return layout;
}
unsafe extern "C" fn output_layout_output_destroy(mut l_output:
                                                      *mut wlr_output_layout_output) {
    wlr_signal_emit_safe(&mut (*l_output).events.destroy,
                         l_output as *mut libc::c_void);
    wlr_output_destroy_global((*l_output).output);
    wl_list_remove(&mut (*(*l_output).state).mode.link);
    wl_list_remove(&mut (*(*l_output).state).scale.link);
    wl_list_remove(&mut (*(*l_output).state).transform.link);
    wl_list_remove(&mut (*(*l_output).state).output_destroy.link);
    wl_list_remove(&mut (*l_output).link);
    free((*l_output).state as *mut libc::c_void);
    free(l_output as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_destroy(mut layout:
                                                       *mut wlr_output_layout) {
    if layout.is_null() { return }
    wlr_signal_emit_safe(&mut (*layout).events.destroy,
                         layout as *mut libc::c_void);
    let mut l_output: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    let mut temp: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    l_output =
        ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    temp =
        ((*l_output).link.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    while &mut (*l_output).link as *mut wl_list !=
              &mut (*layout).outputs as *mut wl_list {
        output_layout_output_destroy(l_output);
        l_output = temp;
        temp =
            ((*l_output).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output
    }
    free((*layout).state as *mut libc::c_void);
    free(layout as *mut libc::c_void);
}
unsafe extern "C" fn output_layout_output_get_box(mut l_output:
                                                      *mut wlr_output_layout_output)
 -> *mut wlr_box {
    (*(*l_output).state)._box.x = (*l_output).x;
    (*(*l_output).state)._box.y = (*l_output).y;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    wlr_output_effective_resolution((*l_output).output, &mut width,
                                    &mut height);
    (*(*l_output).state)._box.width = width;
    (*(*l_output).state)._box.height = height;
    return &mut (*(*l_output).state)._box;
}
/* *
 * This must be called whenever the layout changes to reconfigure the auto
 * configured outputs and emit the `changed` event.
 *
 * Auto configured outputs are placed to the right of the north east corner of
 * the rightmost output in the layout in a horizontal line.
 */
unsafe extern "C" fn output_layout_reconfigure(mut layout:
                                                   *mut wlr_output_layout) {
    let mut max_x: libc::c_int =
        -2147483647i32 - 1i32; // y value for the max_x output
    let mut max_x_y: libc::c_int = -2147483647i32 - 1i32;
    // find the rightmost x coordinate occupied by a manually configured output
	// in the layout
    let mut l_output: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    l_output =
        ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    while &mut (*l_output).link as *mut wl_list !=
              &mut (*layout).outputs as *mut wl_list {
        if !(*(*l_output).state).auto_configured {
            let mut box_0: *mut wlr_box =
                output_layout_output_get_box(l_output);
            if (*box_0).x + (*box_0).width > max_x {
                max_x = (*box_0).x + (*box_0).width;
                max_x_y = (*box_0).y
            }
        }
        l_output =
            ((*l_output).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output
    }
    if max_x == -2147483647i32 - 1i32 {
        // there are no manually configured outputs
        max_x = 0i32;
        max_x_y = 0i32
    }
    l_output =
        ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    while &mut (*l_output).link as *mut wl_list !=
              &mut (*layout).outputs as *mut wl_list {
        if (*(*l_output).state).auto_configured {
            let mut box_1: *mut wlr_box =
                output_layout_output_get_box(l_output);
            (*l_output).x = max_x;
            (*l_output).y = max_x_y;
            max_x += (*box_1).width
        }
        l_output =
            ((*l_output).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output
    }
    wlr_signal_emit_safe(&mut (*layout).events.change,
                         layout as *mut libc::c_void);
}
unsafe extern "C" fn output_update_global(mut output: *mut wlr_output) {
    // Don't expose the output if it doesn't have a current mode
    if wl_list_empty(&mut (*output).modes) != 0 ||
           !(*output).current_mode.is_null() {
        wlr_output_create_global(output);
    } else { wlr_output_destroy_global(output); };
}
unsafe extern "C" fn handle_output_mode(mut listener: *mut wl_listener,
                                        mut data: *mut libc::c_void) {
    let mut state: *mut wlr_output_layout_output_state =
        (listener as *mut libc::c_char).offset(-40) as
            *mut wlr_output_layout_output_state;
    output_layout_reconfigure((*state).layout);
    output_update_global((*(*state).l_output).output);
}
unsafe extern "C" fn handle_output_scale(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut state: *mut wlr_output_layout_output_state =
        (listener as *mut libc::c_char).offset(-64) as
            *mut wlr_output_layout_output_state;
    output_layout_reconfigure((*state).layout);
}
unsafe extern "C" fn handle_output_transform(mut listener: *mut wl_listener,
                                             mut data: *mut libc::c_void) {
    let mut state: *mut wlr_output_layout_output_state =
        (listener as *mut libc::c_char).offset(-88) as
            *mut wlr_output_layout_output_state;
    output_layout_reconfigure((*state).layout);
}
unsafe extern "C" fn handle_output_destroy(mut listener: *mut wl_listener,
                                           mut data: *mut libc::c_void) {
    let mut state: *mut wlr_output_layout_output_state =
        (listener as *mut libc::c_char).offset(-112) as
            *mut wlr_output_layout_output_state;
    let mut layout: *mut wlr_output_layout = (*state).layout;
    output_layout_output_destroy((*state).l_output);
    output_layout_reconfigure(layout);
}
unsafe extern "C" fn output_layout_output_create(mut layout:
                                                     *mut wlr_output_layout,
                                                 mut output: *mut wlr_output)
 -> *mut wlr_output_layout_output {
    let mut l_output: *mut wlr_output_layout_output =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_output_layout_output>() as
                   libc::c_ulong) as *mut wlr_output_layout_output;
    if l_output.is_null() { return 0 as *mut wlr_output_layout_output }
    (*l_output).state =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_output_layout_output_state>() as
                   libc::c_ulong) as *mut wlr_output_layout_output_state;
    if (*l_output).state.is_null() {
        free(l_output as *mut libc::c_void);
        return 0 as *mut wlr_output_layout_output
    }
    (*(*l_output).state).l_output = l_output;
    (*(*l_output).state).layout = layout;
    (*l_output).output = output;
    wl_signal_init(&mut (*l_output).events.destroy);
    wl_list_insert(&mut (*layout).outputs, &mut (*l_output).link);
    wl_signal_add(&mut (*output).events.mode, &mut (*(*l_output).state).mode);
    (*(*l_output).state).mode.notify =
        Some(handle_output_mode as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.scale,
                  &mut (*(*l_output).state).scale);
    (*(*l_output).state).scale.notify =
        Some(handle_output_scale as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.transform,
                  &mut (*(*l_output).state).transform);
    (*(*l_output).state).transform.notify =
        Some(handle_output_transform as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.destroy,
                  &mut (*(*l_output).state).output_destroy);
    (*(*l_output).state).output_destroy.notify =
        Some(handle_output_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    return l_output;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_add(mut layout:
                                                   *mut wlr_output_layout,
                                               mut output: *mut wlr_output,
                                               mut lx: libc::c_int,
                                               mut ly: libc::c_int) {
    let mut l_output: *mut wlr_output_layout_output =
        wlr_output_layout_get(layout, output);
    let mut is_new: bool = l_output.is_null();
    if l_output.is_null() {
        l_output = output_layout_output_create(layout, output);
        if l_output.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to create wlr_output_layout_output\x00"
                         as *const u8 as *const libc::c_char,
                     b"../types/wlr_output_layout.c\x00" as *const u8 as
                         *const libc::c_char, 208i32);
            return
        }
    }
    (*l_output).x = lx;
    (*l_output).y = ly;
    (*(*l_output).state).auto_configured = 0i32 != 0;
    output_layout_reconfigure(layout);
    output_update_global(output);
    if is_new {
        wlr_signal_emit_safe(&mut (*layout).events.add,
                             l_output as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_get(mut layout:
                                                   *mut wlr_output_layout,
                                               mut reference: *mut wlr_output)
 -> *mut wlr_output_layout_output {
    let mut l_output: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    l_output =
        ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    while &mut (*l_output).link as *mut wl_list !=
              &mut (*layout).outputs as *mut wl_list {
        if (*l_output).output == reference { return l_output }
        l_output =
            ((*l_output).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output
    }
    return 0 as *mut wlr_output_layout_output;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_contains_point(mut layout:
                                                              *mut wlr_output_layout,
                                                          mut reference:
                                                              *mut wlr_output,
                                                          mut lx: libc::c_int,
                                                          mut ly: libc::c_int)
 -> bool {
    if !reference.is_null() {
        let mut l_output: *mut wlr_output_layout_output =
            wlr_output_layout_get(layout, reference);
        let mut box_0: *mut wlr_box = output_layout_output_get_box(l_output);
        return wlr_box_contains_point(box_0, lx as libc::c_double,
                                      ly as libc::c_double)
    } else {
        return !wlr_output_layout_output_at(layout, lx as libc::c_double,
                                            ly as libc::c_double).is_null()
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_intersects(mut layout:
                                                          *mut wlr_output_layout,
                                                      mut reference:
                                                          *mut wlr_output,
                                                      mut target_lbox:
                                                          *const wlr_box)
 -> bool {
    let mut out_box: wlr_box = wlr_box{x: 0, y: 0, width: 0, height: 0,};
    if reference.is_null() {
        let mut l_output: *mut wlr_output_layout_output =
            0 as *mut wlr_output_layout_output;
        l_output =
            ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output;
        while &mut (*l_output).link as *mut wl_list !=
                  &mut (*layout).outputs as *mut wl_list {
            let mut output_box: *mut wlr_box =
                output_layout_output_get_box(l_output);
            if wlr_box_intersection(&mut out_box, output_box, target_lbox) {
                return 1i32 != 0
            }
            l_output =
                ((*l_output).link.next as *mut libc::c_char).offset(-16) as
                    *mut wlr_output_layout_output
        }
        return 0i32 != 0
    } else {
        let mut l_output_0: *mut wlr_output_layout_output =
            wlr_output_layout_get(layout, reference);
        if l_output_0.is_null() { return 0i32 != 0 }
        let mut output_box_0: *mut wlr_box =
            output_layout_output_get_box(l_output_0);
        return wlr_box_intersection(&mut out_box, output_box_0, target_lbox)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_output_at(mut layout:
                                                         *mut wlr_output_layout,
                                                     mut lx: libc::c_double,
                                                     mut ly: libc::c_double)
 -> *mut wlr_output {
    let mut l_output: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    l_output =
        ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    while &mut (*l_output).link as *mut wl_list !=
              &mut (*layout).outputs as *mut wl_list {
        let mut box_0: *mut wlr_box = output_layout_output_get_box(l_output);
        if wlr_box_contains_point(box_0, lx, ly) { return (*l_output).output }
        l_output =
            ((*l_output).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output
    }
    return 0 as *mut wlr_output;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_move(mut layout:
                                                    *mut wlr_output_layout,
                                                mut output: *mut wlr_output,
                                                mut lx: libc::c_int,
                                                mut ly: libc::c_int) {
    let mut l_output: *mut wlr_output_layout_output =
        wlr_output_layout_get(layout, output);
    if !l_output.is_null() {
        (*l_output).x = lx;
        (*l_output).y = ly;
        (*(*l_output).state).auto_configured = 0i32 != 0;
        output_layout_reconfigure(layout);
    } else {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] output not found in this layout: %s\x00" as
                     *const u8 as *const libc::c_char,
                 b"../types/wlr_output_layout.c\x00" as *const u8 as
                     *const libc::c_char, 295i32,
                 (*output).name.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_remove(mut layout:
                                                      *mut wlr_output_layout,
                                                  mut output:
                                                      *mut wlr_output) {
    let mut l_output: *mut wlr_output_layout_output =
        wlr_output_layout_get(layout, output);
    if !l_output.is_null() {
        output_layout_output_destroy(l_output);
        output_layout_reconfigure(layout);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_output_coords(mut layout:
                                                             *mut wlr_output_layout,
                                                         mut reference:
                                                             *mut wlr_output,
                                                         mut lx:
                                                             *mut libc::c_double,
                                                         mut ly:
                                                             *mut libc::c_double) {
    if !layout.is_null() && !reference.is_null() {
    } else {
        __assert_fail(b"layout && reference\x00" as *const u8 as
                          *const libc::c_char,
                      b"../types/wlr_output_layout.c\x00" as *const u8 as
                          *const libc::c_char, 311i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 106],
                                                &[libc::c_char; 106]>(b"void wlr_output_layout_output_coords(struct wlr_output_layout *, struct wlr_output *, double *, double *)\x00")).as_ptr());
    };
    let mut src_x: libc::c_double = *lx;
    let mut src_y: libc::c_double = *ly;
    let mut l_output: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    l_output =
        ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    while &mut (*l_output).link as *mut wl_list !=
              &mut (*layout).outputs as *mut wl_list {
        if (*l_output).output == reference {
            *lx = src_x - (*l_output).x as libc::c_double;
            *ly = src_y - (*l_output).y as libc::c_double;
            return
        }
        l_output =
            ((*l_output).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_closest_point(mut layout:
                                                             *mut wlr_output_layout,
                                                         mut reference:
                                                             *mut wlr_output,
                                                         mut lx:
                                                             libc::c_double,
                                                         mut ly:
                                                             libc::c_double,
                                                         mut dest_lx:
                                                             *mut libc::c_double,
                                                         mut dest_ly:
                                                             *mut libc::c_double) {
    if dest_lx.is_null() && dest_ly.is_null() { return }
    let mut min_x: libc::c_double = 0i32 as libc::c_double;
    let mut min_y: libc::c_double = 0i32 as libc::c_double;
    let mut min_distance: libc::c_double = 1.7976931348623157e+308f64;
    let mut l_output: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    l_output =
        ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    while &mut (*l_output).link as *mut wl_list !=
              &mut (*layout).outputs as *mut wl_list {
        if !(!reference.is_null() && reference != (*l_output).output) {
            let mut output_x: libc::c_double = 0.;
            let mut output_y: libc::c_double = 0.;
            let mut output_distance: libc::c_double = 0.;
            let mut box_0: *mut wlr_box =
                output_layout_output_get_box(l_output);
            wlr_box_closest_point(box_0, lx, ly, &mut output_x,
                                  &mut output_y);
            // calculate squared distance suitable for comparison
            output_distance =
                (lx - output_x) * (lx - output_x) +
                    (ly - output_y) * (ly - output_y);
            if output_distance == ::std::f32::INFINITY as libc::c_double {
                output_distance = 1.7976931348623157e+308f64
            }
            if output_distance < min_distance {
                min_x = output_x;
                min_y = output_y;
                min_distance = output_distance
            }
        }
        l_output =
            ((*l_output).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output
    }
    if !dest_lx.is_null() { *dest_lx = min_x }
    if !dest_ly.is_null() { *dest_ly = min_y };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_get_box(mut layout:
                                                       *mut wlr_output_layout,
                                                   mut reference:
                                                       *mut wlr_output)
 -> *mut wlr_box {
    let mut l_output: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    if !reference.is_null() {
        // output extents
        l_output = wlr_output_layout_get(layout, reference);
        if !l_output.is_null() {
            return output_layout_output_get_box(l_output)
        } else { return 0 as *mut wlr_box }
    } else {
        // layout extents
        let mut min_x: libc::c_int = 0i32;
        let mut max_x: libc::c_int = 0i32;
        let mut min_y: libc::c_int = 0i32;
        let mut max_y: libc::c_int = 0i32;
        if wl_list_empty(&mut (*layout).outputs) == 0 {
            min_y = 2147483647i32;
            min_x = min_y;
            max_y = -2147483647i32 - 1i32;
            max_x = max_y;
            l_output =
                ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
                    *mut wlr_output_layout_output;
            while &mut (*l_output).link as *mut wl_list !=
                      &mut (*layout).outputs as *mut wl_list {
                let mut box_0: *mut wlr_box =
                    output_layout_output_get_box(l_output);
                if (*box_0).x < min_x { min_x = (*box_0).x }
                if (*box_0).y < min_y { min_y = (*box_0).y }
                if (*box_0).x + (*box_0).width > max_x {
                    max_x = (*box_0).x + (*box_0).width
                }
                if (*box_0).y + (*box_0).height > max_y {
                    max_y = (*box_0).y + (*box_0).height
                }
                l_output =
                    ((*l_output).link.next as *mut libc::c_char).offset(-16)
                        as *mut wlr_output_layout_output
            }
        }
        (*(*layout).state)._box.x = min_x;
        (*(*layout).state)._box.y = min_y;
        (*(*layout).state)._box.width = max_x - min_x;
        (*(*layout).state)._box.height = max_y - min_y;
        return &mut (*(*layout).state)._box
    };
    // not reached
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_add_auto(mut layout:
                                                        *mut wlr_output_layout,
                                                    mut output:
                                                        *mut wlr_output) {
    let mut l_output: *mut wlr_output_layout_output =
        wlr_output_layout_get(layout, output);
    let mut is_new: bool = l_output.is_null();
    if l_output.is_null() {
        l_output = output_layout_output_create(layout, output);
        if l_output.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] Failed to create wlr_output_layout_output\x00"
                         as *const u8 as *const libc::c_char,
                     b"../types/wlr_output_layout.c\x00" as *const u8 as
                         *const libc::c_char, 420i32);
            return
        }
    }
    (*(*l_output).state).auto_configured = 1i32 != 0;
    output_layout_reconfigure(layout);
    output_update_global(output);
    if is_new {
        wlr_signal_emit_safe(&mut (*layout).events.add,
                             l_output as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_get_center_output(mut layout:
                                                                 *mut wlr_output_layout)
 -> *mut wlr_output {
    if wl_list_empty(&mut (*layout).outputs) != 0 {
        return 0 as *mut wlr_output
    }
    let mut extents: *mut wlr_box =
        wlr_output_layout_get_box(layout, 0 as *mut wlr_output);
    let mut center_x: libc::c_double =
        (*extents).width as libc::c_double / 2.0f64 +
            (*extents).x as libc::c_double;
    let mut center_y: libc::c_double =
        (*extents).height as libc::c_double / 2.0f64 +
            (*extents).y as libc::c_double;
    let mut dest_x: libc::c_double = 0i32 as libc::c_double;
    let mut dest_y: libc::c_double = 0i32 as libc::c_double;
    wlr_output_layout_closest_point(layout, 0 as *mut wlr_output, center_x,
                                    center_y, &mut dest_x, &mut dest_y);
    return wlr_output_layout_output_at(layout, dest_x, dest_y);
}
unsafe extern "C" fn wlr_output_layout_output_in_direction(mut layout:
                                                               *mut wlr_output_layout,
                                                           mut direction:
                                                               wlr_direction,
                                                           mut reference:
                                                               *mut wlr_output,
                                                           mut ref_lx:
                                                               libc::c_double,
                                                           mut ref_ly:
                                                               libc::c_double,
                                                           mut distance_method:
                                                               distance_selection_method)
 -> *mut wlr_output {
    if !reference.is_null() {
    } else {
        __assert_fail(b"reference\x00" as *const u8 as *const libc::c_char,
                      b"../types/wlr_output_layout.c\x00" as *const u8 as
                          *const libc::c_char, 460i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 174],
                                                &[libc::c_char; 174]>(b"struct wlr_output *wlr_output_layout_output_in_direction(struct wlr_output_layout *, enum wlr_direction, struct wlr_output *, double, double, enum distance_selection_method)\x00")).as_ptr());
    };
    let mut ref_box: *mut wlr_box =
        wlr_output_layout_get_box(layout, reference);
    let mut min_distance: libc::c_double =
        if distance_method as libc::c_uint ==
               NEAREST as libc::c_int as libc::c_uint {
            1.7976931348623157e+308f64
        } else { 2.2250738585072014e-308f64 };
    let mut closest_output: *mut wlr_output = 0 as *mut wlr_output;
    let mut l_output: *mut wlr_output_layout_output =
        0 as *mut wlr_output_layout_output;
    l_output =
        ((*layout).outputs.next as *mut libc::c_char).offset(-16) as
            *mut wlr_output_layout_output;
    while &mut (*l_output).link as *mut wl_list !=
              &mut (*layout).outputs as *mut wl_list {
        if !(!reference.is_null() && reference == (*l_output).output) {
            let mut box_0: *mut wlr_box =
                output_layout_output_get_box(l_output);
            let mut match_0: bool = 0i32 != 0;
            // test to make sure this output is in the given direction
            if direction as libc::c_uint &
                   WLR_DIRECTION_LEFT as libc::c_int as libc::c_uint != 0 {
                match_0 =
                    (*box_0).x + (*box_0).width <= (*ref_box).x ||
                        match_0 as libc::c_int != 0
            }
            if direction as libc::c_uint &
                   WLR_DIRECTION_RIGHT as libc::c_int as libc::c_uint != 0 {
                match_0 =
                    (*box_0).x >= (*ref_box).x + (*ref_box).width ||
                        match_0 as libc::c_int != 0
            }
            if direction as libc::c_uint &
                   WLR_DIRECTION_UP as libc::c_int as libc::c_uint != 0 {
                match_0 =
                    (*box_0).y + (*box_0).height <= (*ref_box).y ||
                        match_0 as libc::c_int != 0
            }
            if direction as libc::c_uint &
                   WLR_DIRECTION_DOWN as libc::c_int as libc::c_uint != 0 {
                match_0 =
                    (*box_0).y >= (*ref_box).y + (*ref_box).height ||
                        match_0 as libc::c_int != 0
            }
            if match_0 {
                // calculate distance from the given reference point
                let mut x: libc::c_double = 0.;
                let mut y: libc::c_double = 0.;
                wlr_output_layout_closest_point(layout, (*l_output).output,
                                                ref_lx, ref_ly, &mut x,
                                                &mut y);
                let mut distance: libc::c_double =
                    (x - ref_lx) * (x - ref_lx) + (y - ref_ly) * (y - ref_ly);
                if if distance_method as libc::c_uint ==
                          NEAREST as libc::c_int as libc::c_uint {
                       (distance < min_distance) as libc::c_int
                   } else { (distance > min_distance) as libc::c_int } != 0 {
                    min_distance = distance;
                    closest_output = (*l_output).output
                }
            }
        }
        l_output =
            ((*l_output).link.next as *mut libc::c_char).offset(-16) as
                *mut wlr_output_layout_output
    }
    return closest_output;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_adjacent_output(mut layout:
                                                               *mut wlr_output_layout,
                                                           mut direction:
                                                               wlr_direction,
                                                           mut reference:
                                                               *mut wlr_output,
                                                           mut ref_lx:
                                                               libc::c_double,
                                                           mut ref_ly:
                                                               libc::c_double)
 -> *mut wlr_output {
    return wlr_output_layout_output_in_direction(layout, direction, reference,
                                                 ref_lx, ref_ly, NEAREST);
}
/* *
 * Get the closest adjacent output to the reference output from the reference
 * point in the given direction.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_output_layout_farthest_output(mut layout:
                                                               *mut wlr_output_layout,
                                                           mut direction:
                                                               wlr_direction,
                                                           mut reference:
                                                               *mut wlr_output,
                                                           mut ref_lx:
                                                               libc::c_double,
                                                           mut ref_ly:
                                                               libc::c_double)
 -> *mut wlr_output {
    return wlr_output_layout_output_in_direction(layout, direction, reference,
                                                 ref_lx, ref_ly, FARTHEST);
}
