use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_surface;
    pub type wlr_texture;
    pub type wlr_backend;
    pub type wlr_output_impl;
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
    /* creation/destruction */
    #[no_mangle]
    fn pixman_region32_init(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_fini(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_copy(dest: *mut pixman_region32_t,
                            source: *mut pixman_region32_t) -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_union(new_reg: *mut pixman_region32_t,
                             reg1: *mut pixman_region32_t,
                             reg2: *mut pixman_region32_t) -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_intersect_rect(dest: *mut pixman_region32_t,
                                      source: *mut pixman_region32_t,
                                      x: libc::c_int, y: libc::c_int,
                                      width: libc::c_uint,
                                      height: libc::c_uint) -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_union_rect(dest: *mut pixman_region32_t,
                                  source: *mut pixman_region32_t,
                                  x: libc::c_int, y: libc::c_int,
                                  width: libc::c_uint, height: libc::c_uint)
     -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_not_empty(region: *mut pixman_region32_t)
     -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_extents(region: *mut pixman_region32_t)
     -> *mut pixman_box32_t;
    #[no_mangle]
    fn pixman_region32_n_rects(region: *mut pixman_region32_t) -> libc::c_int;
    #[no_mangle]
    fn pixman_region32_clear(region: *mut pixman_region32_t);
    /* *
 * Computes the transformed output resolution.
 */
    #[no_mangle]
    fn wlr_output_transformed_resolution(output: *mut wlr_output,
                                         width: *mut libc::c_int,
                                         height: *mut libc::c_int);
    /* *
 * Attach the renderer's buffer to the output. Compositors must call this
 * function before rendering. After they are done rendering, they should call
 * `wlr_output_commit` to submit the new frame.
 *
 * If non-NULL, `buffer_age` is set to the drawing buffer age in number of
 * frames or -1 if unknown. This is useful for damage tracking.
 */
    #[no_mangle]
    fn wlr_output_attach_render(output: *mut wlr_output,
                                buffer_age: *mut libc::c_int) -> bool;
    /* *
 * Manually schedules a `frame` event. If a `frame` event is already pending,
 * it is a no-op.
 */
    #[no_mangle]
    fn wlr_output_schedule_frame(output: *mut wlr_output);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_damage {
    pub output: *mut wlr_output,
    pub max_rects: libc::c_int,
    pub current: pixman_region32_t,
    pub previous: [pixman_region32_t; 2],
    pub previous_idx: size_t,
    pub events: C2RustUnnamed,
    pub output_destroy: wl_listener,
    pub output_mode: wl_listener,
    pub output_transform: wl_listener,
    pub output_scale: wl_listener,
    pub output_needs_frame: wl_listener,
    pub output_frame: wl_listener,
    pub output_commit: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub frame: wl_signal,
    pub destroy: wl_signal,
}
/*
 * 32 bit regions
 */
pub type pixman_region32_t = pixman_region32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_data_t = pixman_region32_data;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_box32_t = pixman_box32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
/* *
 * A compositor output region. This typically corresponds to a monitor that
 * displays part of the compositor space.
 *
 * The `frame` event will be emitted when it is a good time for the compositor
 * to submit a new frame.
 *
 * To render a new frame, compositors should call `wlr_output_attach_render`,
 * render and call `wlr_output_commit`. No rendering should happen outside a
 * `frame` event handler or before `wlr_output_attach_render`.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output {
    pub impl_0: *const crate::src::backend::drm::backend::wlr_output_impl,
    pub backend: *mut crate::src::backend::backend::wlr_backend,
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
    pub events: C2RustUnnamed_1,
    pub idle_frame: *mut wl_event_source,
    pub idle_done: *mut wl_event_source,
    pub attach_render_locks: libc::c_int,
    pub cursors: wl_list,
    pub hardware_cursor: *mut wlr_output_cursor,
    pub software_cursor_locks: libc::c_int,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
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
    pub texture: *mut crate::src::backend::drm::atomic::wlr_texture,
    pub surface: *mut crate::src::types::data_device::wlr_data_device::wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_0,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
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
/* *
 * Holds the double-buffered output state.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub damage: pixman_region32_t,
    pub buffer_type: wlr_output_state_buffer_type,
    pub buffer: *mut wlr_buffer,
    // if WLR_OUTPUT_STATE_BUFFER_SCANOUT
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut crate::src::backend::drm::atomic::wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
pub type wl_output_subpixel = libc::c_uint;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: wl_output_subpixel = 5;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: wl_output_subpixel = 4;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: wl_output_subpixel = 3;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: wl_output_subpixel = 2;
pub const WL_OUTPUT_SUBPIXEL_NONE: wl_output_subpixel = 1;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: wl_output_subpixel = 0;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}
/* **********************************************************

Copyright 1987, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from The Open Group.

Copyright 1987 by Digital Equipment Corporation, Maynard, Massachusetts.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name of Digital not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

******************************************************************/
/*
 * Copyright © 1998, 2004 Keith Packard
 * Copyright   2007 Red Hat, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of Keith Packard not be used in
 * advertising or publicity pertaining to distribution of the software without
 * specific, written prior permission.  Keith Packard makes no
 * representations about the suitability of this software for any purpose.  It
 * is provided "as is" without express or implied warranty.
 *
 * KEITH PACKARD DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL KEITH PACKARD BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Standard integers
 */
/*
 * Boolean
 */
pub type pixman_bool_t = libc::c_int;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type wlr_output_state_field = libc::c_uint;
pub const WLR_OUTPUT_STATE_DAMAGE: wlr_output_state_field = 2;
pub const WLR_OUTPUT_STATE_BUFFER: wlr_output_state_field = 1;
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&mut (*signal).listener_list);
}
#[inline]
unsafe extern "C" fn wl_signal_add(mut signal: *mut wl_signal,
                                   mut listener: *mut wl_listener) {
    wl_list_insert((*signal).listener_list.prev, &mut (*listener).link);
}
unsafe extern "C" fn output_handle_destroy(mut listener: *mut wl_listener,
                                           mut data: *mut libc::c_void) {
    let mut output_damage: *mut wlr_output_damage =
        (listener as *mut libc::c_char).offset(-128) as
            *mut wlr_output_damage;
    wlr_output_damage_destroy(output_damage);
}
unsafe extern "C" fn output_handle_mode(mut listener: *mut wl_listener,
                                        mut data: *mut libc::c_void) {
    let mut output_damage: *mut wlr_output_damage =
        (listener as *mut libc::c_char).offset(-152) as
            *mut wlr_output_damage;
    wlr_output_damage_add_whole(output_damage);
}
unsafe extern "C" fn output_handle_transform(mut listener: *mut wl_listener,
                                             mut data: *mut libc::c_void) {
    let mut output_damage: *mut wlr_output_damage =
        (listener as *mut libc::c_char).offset(-176) as
            *mut wlr_output_damage;
    wlr_output_damage_add_whole(output_damage);
}
unsafe extern "C" fn output_handle_scale(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut output_damage: *mut wlr_output_damage =
        (listener as *mut libc::c_char).offset(-200) as
            *mut wlr_output_damage;
    wlr_output_damage_add_whole(output_damage);
}
unsafe extern "C" fn output_handle_needs_frame(mut listener: *mut wl_listener,
                                               mut data: *mut libc::c_void) {
    let mut output_damage: *mut wlr_output_damage =
        (listener as *mut libc::c_char).offset(-224) as
            *mut wlr_output_damage;
    pixman_region32_union(&mut (*output_damage).current,
                          &mut (*output_damage).current,
                          &mut (*(*output_damage).output).damage);
    wlr_output_schedule_frame((*output_damage).output);
}
unsafe extern "C" fn output_handle_frame(mut listener: *mut wl_listener,
                                         mut data: *mut libc::c_void) {
    let mut output_damage: *mut wlr_output_damage =
        (listener as *mut libc::c_char).offset(-248) as
            *mut wlr_output_damage;
    if !(*(*output_damage).output).enabled { return }
    wlr_signal_emit_safe(&mut (*output_damage).events.frame,
                         output_damage as *mut libc::c_void);
}
unsafe extern "C" fn output_handle_commit(mut listener: *mut wl_listener,
                                          mut data: *mut libc::c_void) {
    let mut output_damage: *mut wlr_output_damage =
        (listener as *mut libc::c_char).offset(-272) as
            *mut wlr_output_damage;
    if (*(*output_damage).output).pending.committed &
           WLR_OUTPUT_STATE_BUFFER as libc::c_int as libc::c_uint == 0 {
        return
    }
    let mut prev: *mut pixman_region32_t = 0 as *mut pixman_region32_t;
    match (*(*output_damage).output).pending.buffer_type as libc::c_uint {
        0 => {
            // render-buffers have been swapped, rotate the damage
            // same as decrementing, but works on unsigned integers
            (*output_damage).previous_idx =
                ((*output_damage).previous_idx as
                     libc::c_ulong).wrapping_add((2i32 - 1i32) as
                                                     libc::c_ulong) as size_t
                    as size_t;
            (*output_damage).previous_idx =
                ((*output_damage).previous_idx as
                     libc::c_ulong).wrapping_rem(2i32 as libc::c_ulong) as
                    size_t as size_t;
            prev =
                &mut *(*output_damage).previous.as_mut_ptr().offset((*output_damage).previous_idx
                                                                        as
                                                                        isize)
                    as *mut pixman_region32_t;
            pixman_region32_copy(prev, &mut (*output_damage).current);
        }
        1 => {
            // accumulate render-buffer damage
            prev =
                &mut *(*output_damage).previous.as_mut_ptr().offset((*output_damage).previous_idx
                                                                        as
                                                                        isize)
                    as *mut pixman_region32_t;
            pixman_region32_union(prev, prev, &mut (*output_damage).current);
        }
        _ => { }
    }
    pixman_region32_clear(&mut (*output_damage).current);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * Damage tracking requires to keep track of previous frames' damage. To allow
 * damage tracking to work with triple buffering, a history of two frames is
 * required.
 */
/* *
 * Tracks damage for an output.
 *
 * The `frame` event will be emitted when it is a good time for the compositor
 * to submit a new frame.
 *
 * To render a new frame, compositors should call
 * `wlr_output_damage_attach_render`, render and call `wlr_output_commit`. No
 * rendering should happen outside a `frame` event handler or before
 * `wlr_output_damage_attach_render`.
 */
// max number of damaged rectangles
// in output-local coordinates
// circular queue for previous damage
#[no_mangle]
pub unsafe extern "C" fn wlr_output_damage_create(mut output: *mut wlr_output)
 -> *mut wlr_output_damage {
    let mut output_damage: *mut wlr_output_damage =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_output_damage>() as libc::c_ulong) as
            *mut wlr_output_damage;
    if output_damage.is_null() { return 0 as *mut wlr_output_damage }
    (*output_damage).output = output;
    (*output_damage).max_rects = 20i32;
    wl_signal_init(&mut (*output_damage).events.frame);
    wl_signal_init(&mut (*output_damage).events.destroy);
    pixman_region32_init(&mut (*output_damage).current);
    let mut i: size_t = 0i32 as size_t;
    while i < 2i32 as libc::c_ulong {
        pixman_region32_init(&mut *(*output_damage).previous.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize));
        i = i.wrapping_add(1)
    }
    wl_signal_add(&mut (*output).events.destroy,
                  &mut (*output_damage).output_destroy);
    (*output_damage).output_destroy.notify =
        Some(output_handle_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.mode,
                  &mut (*output_damage).output_mode);
    (*output_damage).output_mode.notify =
        Some(output_handle_mode as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.transform,
                  &mut (*output_damage).output_transform);
    (*output_damage).output_transform.notify =
        Some(output_handle_transform as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.scale,
                  &mut (*output_damage).output_scale);
    (*output_damage).output_scale.notify =
        Some(output_handle_scale as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.needs_frame,
                  &mut (*output_damage).output_needs_frame);
    (*output_damage).output_needs_frame.notify =
        Some(output_handle_needs_frame as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.frame,
                  &mut (*output_damage).output_frame);
    (*output_damage).output_frame.notify =
        Some(output_handle_frame as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*output).events.commit,
                  &mut (*output_damage).output_commit);
    (*output_damage).output_commit.notify =
        Some(output_handle_commit as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    return output_damage;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_damage_destroy(mut output_damage:
                                                       *mut wlr_output_damage) {
    if output_damage.is_null() { return }
    wlr_signal_emit_safe(&mut (*output_damage).events.destroy,
                         output_damage as *mut libc::c_void);
    wl_list_remove(&mut (*output_damage).output_destroy.link);
    wl_list_remove(&mut (*output_damage).output_mode.link);
    wl_list_remove(&mut (*output_damage).output_transform.link);
    wl_list_remove(&mut (*output_damage).output_scale.link);
    wl_list_remove(&mut (*output_damage).output_needs_frame.link);
    wl_list_remove(&mut (*output_damage).output_frame.link);
    wl_list_remove(&mut (*output_damage).output_commit.link);
    pixman_region32_fini(&mut (*output_damage).current);
    let mut i: size_t = 0i32 as size_t;
    while i < 2i32 as libc::c_ulong {
        pixman_region32_fini(&mut *(*output_damage).previous.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize));
        i = i.wrapping_add(1)
    }
    free(output_damage as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_damage_attach_render(mut output_damage:
                                                             *mut wlr_output_damage,
                                                         mut needs_frame:
                                                             *mut bool,
                                                         mut damage:
                                                             *mut pixman_region32_t)
 -> bool {
    let mut output: *mut wlr_output = (*output_damage).output;
    let mut buffer_age: libc::c_int = -1i32;
    if !wlr_output_attach_render(output, &mut buffer_age) { return 0i32 != 0 }
    *needs_frame =
        (*output).needs_frame as libc::c_int != 0 ||
            pixman_region32_not_empty(&mut (*output_damage).current) != 0;
    // Check if we can use damage tracking
    if buffer_age <= 0i32 || buffer_age - 1i32 > 2i32 {
        let mut width: libc::c_int = 0;
        let mut height: libc::c_int = 0;
        wlr_output_transformed_resolution(output, &mut width, &mut height);
        // Buffer new or too old, damage the whole output
        pixman_region32_union_rect(damage, damage, 0i32, 0i32,
                                   width as libc::c_uint,
                                   height as libc::c_uint);
        *needs_frame = 1i32 != 0
    } else {
        pixman_region32_copy(damage, &mut (*output_damage).current);
        // Accumulate damage from old buffers
        let mut idx: size_t = (*output_damage).previous_idx;
        let mut i: libc::c_int = 0i32;
        while i < buffer_age - 1i32 {
            let mut j: libc::c_int =
                idx.wrapping_add(i as
                                     libc::c_ulong).wrapping_rem(2i32 as
                                                                     libc::c_ulong)
                    as libc::c_int;
            pixman_region32_union(damage, damage,
                                  &mut *(*output_damage).previous.as_mut_ptr().offset(j
                                                                                          as
                                                                                          isize));
            i += 1
        }
        // Check the number of rectangles
        let mut n_rects: libc::c_int = pixman_region32_n_rects(damage);
        if n_rects > (*output_damage).max_rects {
            let mut extents: *mut pixman_box32_t =
                pixman_region32_extents(damage);
            pixman_region32_union_rect(damage, damage, (*extents).x1,
                                       (*extents).y1,
                                       ((*extents).x2 - (*extents).x1) as
                                           libc::c_uint,
                                       ((*extents).y2 - (*extents).y1) as
                                           libc::c_uint);
        }
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_damage_add(mut output_damage:
                                                   *mut wlr_output_damage,
                                               mut damage:
                                                   *mut pixman_region32_t) {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    wlr_output_transformed_resolution((*output_damage).output, &mut width,
                                      &mut height);
    pixman_region32_union(&mut (*output_damage).current,
                          &mut (*output_damage).current, damage);
    pixman_region32_intersect_rect(&mut (*output_damage).current,
                                   &mut (*output_damage).current, 0i32, 0i32,
                                   width as libc::c_uint,
                                   height as libc::c_uint);
    wlr_output_schedule_frame((*output_damage).output);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_output_damage_add_whole(mut output_damage:
                                                         *mut wlr_output_damage) {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    wlr_output_transformed_resolution((*output_damage).output, &mut width,
                                      &mut height);
    pixman_region32_union_rect(&mut (*output_damage).current,
                               &mut (*output_damage).current, 0i32, 0i32,
                               width as libc::c_uint, height as libc::c_uint);
    wlr_output_schedule_frame((*output_damage).output);
}
/* *
 * Attach the renderer's buffer to the output. Compositors must call this
 * function before rendering. After they are done rendering, they should call
 * `wlr_output_set_damage` and `wlr_output_commit` to submit the new frame.
 *
 * `needs_frame` will be set to true if a frame should be submitted. `damage`
 * will be set to the region of the output that needs to be repainted, in
 * output-buffer-local coordinates.
 *
 * The buffer damage region accumulates all damage since the buffer has last
 * been swapped. This is not to be confused with the output surface damage,
 * which only contains the changes between two frames.
 */
/* *
 * Accumulates damage and schedules a `frame` event.
 */
/* *
 * Damages the whole output and schedules a `frame` event.
 */
/* *
 * Accumulates damage from a box and schedules a `frame` event.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_output_damage_add_box(mut output_damage:
                                                       *mut wlr_output_damage,
                                                   mut box_0: *mut wlr_box) {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    wlr_output_transformed_resolution((*output_damage).output, &mut width,
                                      &mut height);
    pixman_region32_union_rect(&mut (*output_damage).current,
                               &mut (*output_damage).current, (*box_0).x,
                               (*box_0).y, (*box_0).width as libc::c_uint,
                               (*box_0).height as libc::c_uint);
    pixman_region32_intersect_rect(&mut (*output_damage).current,
                                   &mut (*output_damage).current, 0i32, 0i32,
                                   width as libc::c_uint,
                                   height as libc::c_uint);
    wlr_output_schedule_frame((*output_damage).output);
}
