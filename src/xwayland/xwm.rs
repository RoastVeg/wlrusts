use libc;
extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_texture_impl;
    pub type wlr_renderer_impl;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type xcb_connection_t;
    pub type xcb_extension_t;
    pub type xcb_errors_context_t;
    pub type wlr_xwayland_cursor;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
    #[no_mangle]
    fn wl_event_loop_add_fd(loop_0: *mut wl_event_loop, fd: libc::c_int,
                            mask: uint32_t, func: wl_event_loop_fd_func_t,
                            data: *mut libc::c_void) -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_loop_add_timer(loop_0: *mut wl_event_loop,
                               func: wl_event_loop_timer_func_t,
                               data: *mut libc::c_void)
     -> *mut wl_event_source;
    #[no_mangle]
    fn wl_event_source_timer_update(source: *mut wl_event_source,
                                    ms_delay: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wl_event_source_remove(source: *mut wl_event_source) -> libc::c_int;
    #[no_mangle]
    fn wl_event_source_check(source: *mut wl_event_source);
    #[no_mangle]
    fn wlr_surface_from_resource(resource: *mut wl_resource)
     -> *mut wlr_surface;
    #[no_mangle]
    fn wlr_surface_has_buffer(surface: *mut wlr_surface) -> bool;
    #[no_mangle]
    fn wlr_surface_set_role(surface: *mut wlr_surface,
                            role: *const wlr_surface_role,
                            role_data: *mut libc::c_void,
                            error_resource: *mut wl_resource,
                            error_code: uint32_t) -> bool;
    #[no_mangle]
    fn wl_display_get_event_loop(display: *mut wl_display)
     -> *mut wl_event_loop;
    #[no_mangle]
    fn wl_client_get_object(client: *mut wl_client, id: uint32_t)
     -> *mut wl_resource;
    #[no_mangle]
    fn wl_resource_get_id(resource: *mut wl_resource) -> uint32_t;
    #[no_mangle]
    fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strndup(__string: *const libc::c_char, __n: size_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    #[no_mangle]
    fn xcb_disconnect(c: *mut xcb_connection_t);
    #[no_mangle]
    fn xcb_connect_to_fd(fd: libc::c_int, auth_info: *mut xcb_auth_info_t)
     -> *mut xcb_connection_t;
    #[no_mangle]
    fn xcb_connection_has_error(c: *mut xcb_connection_t) -> libc::c_int;
    #[no_mangle]
    fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
    #[no_mangle]
    fn xcb_prefetch_extension_data(c: *mut xcb_connection_t,
                                   ext: *mut xcb_extension_t);
    #[no_mangle]
    fn xcb_get_extension_data(c: *mut xcb_connection_t,
                              ext: *mut xcb_extension_t)
     -> *const xcb_query_extension_reply_t;
    #[no_mangle]
    fn xcb_poll_for_event(c: *mut xcb_connection_t)
     -> *mut xcb_generic_event_t;
    #[no_mangle]
    fn xcb_flush(c: *mut xcb_connection_t) -> libc::c_int;
    #[no_mangle]
    fn xcb_depth_visuals_iterator(R: *const xcb_depth_t)
     -> xcb_visualtype_iterator_t;
    #[no_mangle]
    fn xcb_depth_next(i: *mut xcb_depth_iterator_t);
    #[no_mangle]
    fn xcb_screen_allowed_depths_iterator(R: *const xcb_screen_t)
     -> xcb_depth_iterator_t;
    #[no_mangle]
    fn xcb_setup_roots_iterator(R: *const xcb_setup_t)
     -> xcb_screen_iterator_t;
    #[no_mangle]
    fn xcb_create_window(c: *mut xcb_connection_t, depth: uint8_t,
                         wid: xcb_window_t, parent: xcb_window_t, x: int16_t,
                         y: int16_t, width: uint16_t, height: uint16_t,
                         border_width: uint16_t, _class: uint16_t,
                         visual: xcb_visualid_t, value_mask: uint32_t,
                         value_list: *const libc::c_void)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_change_window_attributes(c: *mut xcb_connection_t,
                                    window: xcb_window_t,
                                    value_mask: uint32_t,
                                    value_list: *const libc::c_void)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_configure_window(c: *mut xcb_connection_t, window: xcb_window_t,
                            value_mask: uint16_t,
                            value_list: *const libc::c_void)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_get_geometry(c: *mut xcb_connection_t, drawable: xcb_drawable_t)
     -> xcb_get_geometry_cookie_t;
    #[no_mangle]
    fn xcb_get_geometry_reply(c: *mut xcb_connection_t,
                              cookie: xcb_get_geometry_cookie_t,
                              e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_get_geometry_reply_t;
    #[no_mangle]
    fn xcb_intern_atom(c: *mut xcb_connection_t, only_if_exists: uint8_t,
                       name_len: uint16_t, name: *const libc::c_char)
     -> xcb_intern_atom_cookie_t;
    #[no_mangle]
    fn xcb_intern_atom_reply(c: *mut xcb_connection_t,
                             cookie: xcb_intern_atom_cookie_t,
                             e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_intern_atom_reply_t;
    #[no_mangle]
    fn xcb_get_atom_name(c: *mut xcb_connection_t, atom: xcb_atom_t)
     -> xcb_get_atom_name_cookie_t;
    #[no_mangle]
    fn xcb_get_atom_name_name(R: *const xcb_get_atom_name_reply_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn xcb_get_atom_name_name_length(R: *const xcb_get_atom_name_reply_t)
     -> libc::c_int;
    #[no_mangle]
    fn xcb_get_atom_name_reply(c: *mut xcb_connection_t,
                               cookie: xcb_get_atom_name_cookie_t,
                               e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_get_atom_name_reply_t;
    #[no_mangle]
    fn xcb_change_property(c: *mut xcb_connection_t, mode: uint8_t,
                           window: xcb_window_t, property: xcb_atom_t,
                           type_0: xcb_atom_t, format: uint8_t,
                           data_len: uint32_t, data: *const libc::c_void)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_get_property(c: *mut xcb_connection_t, _delete: uint8_t,
                        window: xcb_window_t, property: xcb_atom_t,
                        type_0: xcb_atom_t, long_offset: uint32_t,
                        long_length: uint32_t) -> xcb_get_property_cookie_t;
    #[no_mangle]
    fn xcb_get_property_value(R: *const xcb_get_property_reply_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn xcb_get_property_value_length(R: *const xcb_get_property_reply_t)
     -> libc::c_int;
    #[no_mangle]
    fn xcb_get_property_reply(c: *mut xcb_connection_t,
                              cookie: xcb_get_property_cookie_t,
                              e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_get_property_reply_t;
    #[no_mangle]
    fn xcb_set_selection_owner(c: *mut xcb_connection_t, owner: xcb_window_t,
                               selection: xcb_atom_t, time: xcb_timestamp_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_send_event(c: *mut xcb_connection_t, propagate: uint8_t,
                      destination: xcb_window_t, event_mask: uint32_t,
                      event: *const libc::c_char) -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_set_input_focus_checked(c: *mut xcb_connection_t,
                                   revert_to: uint8_t, focus: xcb_window_t,
                                   time: xcb_timestamp_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_set_input_focus(c: *mut xcb_connection_t, revert_to: uint8_t,
                           focus: xcb_window_t, time: xcb_timestamp_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_free_gc(c: *mut xcb_connection_t, gc: xcb_gcontext_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_put_image(c: *mut xcb_connection_t, format: uint8_t,
                     drawable: xcb_drawable_t, gc: xcb_gcontext_t,
                     width: uint16_t, height: uint16_t, dst_x: int16_t,
                     dst_y: int16_t, left_pad: uint8_t, depth: uint8_t,
                     data_len: uint32_t, data: *const uint8_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_create_colormap(c: *mut xcb_connection_t, alloc: uint8_t,
                           mid: xcb_colormap_t, window: xcb_window_t,
                           visual: xcb_visualid_t) -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_create_pixmap(c: *mut xcb_connection_t, depth: uint8_t,
                         pid: xcb_pixmap_t, drawable: xcb_drawable_t,
                         width: uint16_t, height: uint16_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_free_pixmap(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_create_gc(c: *mut xcb_connection_t, cid: xcb_gcontext_t,
                     drawable: xcb_drawable_t, value_mask: uint32_t,
                     value_list: *const libc::c_void) -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_free_colormap(c: *mut xcb_connection_t, cmap: xcb_colormap_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_kill_client(c: *mut xcb_connection_t, resource: uint32_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_free_cursor(c: *mut xcb_connection_t, cursor: xcb_cursor_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_render_pictforminfo_next(i:
                                        *mut xcb_render_pictforminfo_iterator_t);
    #[no_mangle]
    fn xcb_render_query_pict_formats(c: *mut xcb_connection_t)
     -> xcb_render_query_pict_formats_cookie_t;
    #[no_mangle]
    fn xcb_render_query_pict_formats_formats_iterator(R:
                                                          *const xcb_render_query_pict_formats_reply_t)
     -> xcb_render_pictforminfo_iterator_t;
    #[no_mangle]
    fn xcb_render_query_pict_formats_reply(c: *mut xcb_connection_t,
                                           cookie:
                                               xcb_render_query_pict_formats_cookie_t,
                                           e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_render_query_pict_formats_reply_t;
    #[no_mangle]
    fn xcb_render_create_picture(c: *mut xcb_connection_t,
                                 pid: xcb_render_picture_t,
                                 drawable: xcb_drawable_t,
                                 format: xcb_render_pictformat_t,
                                 value_mask: uint32_t,
                                 value_list: *const libc::c_void)
     -> xcb_void_cookie_t;
    #[no_mangle]
    fn xcb_render_create_cursor(c: *mut xcb_connection_t, cid: xcb_cursor_t,
                                source: xcb_render_picture_t, x: uint16_t,
                                y: uint16_t) -> xcb_void_cookie_t;
    #[no_mangle]
    static mut xcb_xfixes_id: xcb_extension_t;
    #[no_mangle]
    fn xcb_composite_redirect_subwindows(c: *mut xcb_connection_t,
                                         window: xcb_window_t,
                                         update: uint8_t)
     -> xcb_void_cookie_t;
    #[no_mangle]
    static mut xcb_composite_id: xcb_extension_t;
    #[no_mangle]
    fn xcb_xfixes_query_version(c: *mut xcb_connection_t,
                                client_major_version: uint32_t,
                                client_minor_version: uint32_t)
     -> xcb_xfixes_query_version_cookie_t;
    #[no_mangle]
    fn xcb_xfixes_query_version_reply(c: *mut xcb_connection_t,
                                      cookie:
                                          xcb_xfixes_query_version_cookie_t,
                                      e: *mut *mut xcb_generic_error_t)
     -> *mut xcb_xfixes_query_version_reply_t;
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
    #[no_mangle]
    fn xcb_icccm_get_wm_size_hints_from_reply(hints: *mut xcb_size_hints_t,
                                              reply:
                                                  *mut xcb_get_property_reply_t)
     -> uint8_t;
    #[no_mangle]
    fn xcb_icccm_wm_hints_get_urgency(hints: *mut xcb_icccm_wm_hints_t)
     -> uint32_t;
    #[no_mangle]
    fn xcb_icccm_get_wm_hints_from_reply(hints: *mut xcb_icccm_wm_hints_t,
                                         reply: *mut xcb_get_property_reply_t)
     -> uint8_t;
    #[no_mangle]
    fn xcb_errors_context_new(conn: *mut xcb_connection_t,
                              ctx: *mut *mut xcb_errors_context_t)
     -> libc::c_int;
    #[no_mangle]
    fn xcb_errors_context_free(ctx: *mut xcb_errors_context_t);
    #[no_mangle]
    fn xcb_errors_get_name_for_major_code(ctx: *mut xcb_errors_context_t,
                                          major_code: uint8_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn xcb_errors_get_name_for_minor_code(ctx: *mut xcb_errors_context_t,
                                          major_code: uint8_t,
                                          minor_code: uint16_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn xcb_errors_get_name_for_xcb_event(ctx: *mut xcb_errors_context_t,
                                         event: *mut xcb_generic_event_t,
                                         extension: *mut *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn xcb_errors_get_name_for_error(ctx: *mut xcb_errors_context_t,
                                     error_code: uint8_t,
                                     extension: *mut *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn xwm_handle_selection_event(xwm: *mut wlr_xwm,
                                  event: *mut xcb_generic_event_t)
     -> libc::c_int;
    #[no_mangle]
    fn xwm_handle_selection_client_message(xwm: *mut wlr_xwm,
                                           ev:
                                               *mut xcb_client_message_event_t)
     -> libc::c_int;
    #[no_mangle]
    fn xwm_selection_init(xwm: *mut wlr_xwm);
    #[no_mangle]
    fn xwm_selection_finish(xwm: *mut wlr_xwm);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
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
/*
 * Fixpoint numbers
 */
/*
 * Misc structs
 */
/*
 * Fixed point matrices
 */
/* forward declaration (sorry) */
/*
 * Floating point matrices
 */
/* The SEPARABLE_CONVOLUTION filter takes the following parameters:
     *
     *         width:           integer given as 16.16 fixpoint number
     *         height:          integer given as 16.16 fixpoint number
     *         x_phase_bits:	integer given as 16.16 fixpoint
     *         y_phase_bits:	integer given as 16.16 fixpoint
     *         xtables:         (1 << x_phase_bits) tables of size width
     *         ytables:         (1 << y_phase_bits) tables of size height
     *
     * When sampling at (x, y), the location is first rounded to one of
     * n_x_phases * n_y_phases subpixel positions. These subpixel positions
     * determine an xtable and a ytable to use.
     *
     * Conceptually a width x height matrix is then formed in which each entry
     * is the product of the corresponding entries in the x and y tables.
     * This matrix is then aligned with the image pixels such that its center
     * is as close as possible to the subpixel location chosen earlier. Then
     * the image is convolved with the matrix and the resulting pixel returned.
     */
/*
 * Regions
 */
/*  pixman_box16_t	rects[size];   in memory but not explicitly declared */
/* This function exists only to make it possible to preserve
 * the X ABI - it should go away at first opportunity.
 */
/* creation/destruction */
/* manipulation */
/*
 * 32 bit regions
 */
pub type pixman_box32_t = pixman_box32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;
pub type time_t = __time_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

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
pub struct wl_array {
    pub size: size_t,
    pub alloc: size_t,
    pub data: *mut libc::c_void,
}
pub type C2RustUnnamed = libc::c_uint;
pub const WL_EVENT_ERROR: C2RustUnnamed = 8;
pub const WL_EVENT_HANGUP: C2RustUnnamed = 4;
pub const WL_EVENT_WRITABLE: C2RustUnnamed = 2;
pub const WL_EVENT_READABLE: C2RustUnnamed = 1;
pub type wl_event_loop_fd_func_t
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: uint32_t,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type wl_event_loop_timer_func_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;

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
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_texture {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_texture_impl,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_renderer {
    pub impl_0: *const crate::src::render::gles2::renderer::wlr_renderer_impl,
    pub events: C2RustUnnamed_0,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
// mHz
// only when using a software cursor without a surface
// only when using a cursor surface
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_surface {
    pub resource: *mut wl_resource,
    pub renderer: *mut wlr_renderer,
    pub buffer: *mut wlr_buffer,
    pub sx: libc::c_int,
    pub sy: libc::c_int,
    pub buffer_damage: pixman_region32_t,
    pub opaque_region: pixman_region32_t,
    pub input_region: pixman_region32_t,
    pub current: wlr_surface_state,
    pub pending: wlr_surface_state,
    pub previous: wlr_surface_state,
    pub role: *const wlr_surface_role,
    pub role_data: *mut libc::c_void,
    pub events: C2RustUnnamed_1,
    pub subsurfaces: wl_list,
    pub subsurface_pending_list: wl_list,
    pub renderer_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub commit: wl_signal,
    pub new_subsurface: wl_signal,
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_surface_role {
    pub name: *const libc::c_char,
    pub commit: Option<unsafe extern "C" fn(_: *mut wlr_surface) -> ()>,
    pub precommit: Option<unsafe extern "C" fn(_: *mut wlr_surface) -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_surface_state {
    pub committed: uint32_t,
    pub buffer_resource: *mut wl_resource,
    pub dx: int32_t,
    pub dy: int32_t,
    pub surface_damage: pixman_region32_t,
    pub buffer_damage: pixman_region32_t,
    pub opaque: pixman_region32_t,
    pub input: pixman_region32_t,
    pub transform: wl_output_transform,
    pub scale: int32_t,
    pub frame_callback_list: wl_list,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub buffer_width: libc::c_int,
    pub buffer_height: libc::c_int,
    pub buffer_destroy: wl_listener,
}
pub type wlr_surface_state_field = libc::c_uint;
pub const WLR_SURFACE_STATE_FRAME_CALLBACK_LIST: wlr_surface_state_field =
    128;
pub const WLR_SURFACE_STATE_SCALE: wlr_surface_state_field = 64;
pub const WLR_SURFACE_STATE_TRANSFORM: wlr_surface_state_field = 32;
pub const WLR_SURFACE_STATE_INPUT_REGION: wlr_surface_state_field = 16;
pub const WLR_SURFACE_STATE_OPAQUE_REGION: wlr_surface_state_field = 8;
pub const WLR_SURFACE_STATE_BUFFER_DAMAGE: wlr_surface_state_field = 4;
pub const WLR_SURFACE_STATE_SURFACE_DAMAGE: wlr_surface_state_field = 2;
pub const WLR_SURFACE_STATE_BUFFER: wlr_surface_state_field = 1;
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
pub type wlr_edges = libc::c_uint;
pub const WLR_EDGE_RIGHT: wlr_edges = 8;
pub const WLR_EDGE_LEFT: wlr_edges = 4;
pub const WLR_EDGE_BOTTOM: wlr_edges = 2;
pub const WLR_EDGE_TOP: wlr_edges = 1;
pub const WLR_EDGE_NONE: wlr_edges = 0;
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_subcompositor {
    pub global: *mut wl_global,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub subcompositor: wlr_subcompositor,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_2,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
pub type wlr_button_state = libc::c_uint;
pub const WLR_BUTTON_PRESSED: wlr_button_state = 1;
pub const WLR_BUTTON_RELEASED: wlr_button_state = 0;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_keyboard_modifiers {
    pub depressed: xkb_mod_mask_t,
    pub latched: xkb_mod_mask_t,
    pub locked: xkb_mod_mask_t,
    pub group: xkb_mod_mask_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_keyboard {
    pub impl_0: *const crate::src::backend::headless::input_device::wlr_keyboard_impl,
    pub group: *mut crate::src::types::wlr_keyboard_group::wlr_keyboard_group,
    pub keymap_string: *mut libc::c_char,
    pub keymap_size: size_t,
    pub keymap: *mut xkb_keymap,
    pub xkb_state: *mut xkb_state,
    pub led_indexes: [xkb_led_index_t; 3],
    pub mod_indexes: [xkb_mod_index_t; 8],
    pub keycodes: [uint32_t; 32],
    pub num_keycodes: size_t,
    pub modifiers: wlr_keyboard_modifiers,
    pub repeat_info: C2RustUnnamed_4,
    pub events: C2RustUnnamed_3,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub rate: int32_t,
    pub delay: int32_t,
}
pub type wlr_axis_source = libc::c_uint;
pub const WLR_AXIS_SOURCE_WHEEL_TILT: wlr_axis_source = 3;
pub const WLR_AXIS_SOURCE_CONTINUOUS: wlr_axis_source = 2;
pub const WLR_AXIS_SOURCE_FINGER: wlr_axis_source = 1;
pub const WLR_AXIS_SOURCE_WHEEL: wlr_axis_source = 0;
pub type wlr_axis_orientation = libc::c_uint;
pub const WLR_AXIS_ORIENTATION_HORIZONTAL: wlr_axis_orientation = 1;
pub const WLR_AXIS_ORIENTATION_VERTICAL: wlr_axis_orientation = 0;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_serial_range {
    pub min_incl: uint32_t,
    pub max_incl: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_serial_ringset {
    pub data: [wlr_serial_range; 128],
    pub end: libc::c_int,
    pub count: libc::c_int,
}
/* *
 * Contains state for a single client's bound wl_seat resource and can be used
 * to issue input events to that client. The lifetime of these objects is
 * managed by wlr_seat; some may be NULL.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_client {
    pub client: *mut wl_client,
    pub seat: *mut wlr_seat,
    pub link: wl_list,
    pub resources: wl_list,
    pub pointers: wl_list,
    pub keyboards: wl_list,
    pub touches: wl_list,
    pub data_devices: wl_list,
    pub events: C2RustUnnamed_5,
    pub serials: wlr_serial_ringset,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat {
    pub global: *mut wl_global,
    pub display: *mut wl_display,
    pub clients: wl_list,
    pub name: *mut libc::c_char,
    pub capabilities: uint32_t,
    pub last_event: timespec,
    pub selection_source: *mut crate::src::types::data_device::wlr_data_device::wlr_data_source,
    pub selection_serial: uint32_t,
    pub selection_offers: wl_list,
    pub primary_selection_source: *mut crate::src::types::wlr_data_control_v1::wlr_primary_selection_source,
    pub primary_selection_serial: uint32_t,
    pub drag: *mut crate::src::types::data_device::wlr_data_device::wlr_drag,
    pub drag_source: *mut crate::src::types::data_device::wlr_data_device::wlr_data_source,
    pub drag_serial: uint32_t,
    pub drag_offers: wl_list,
    pub pointer_state: wlr_seat_pointer_state,
    pub keyboard_state: wlr_seat_keyboard_state,
    pub touch_state: wlr_seat_touch_state,
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
    pub events: C2RustUnnamed_6,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub pointer_grab_begin: wl_signal,
    pub pointer_grab_end: wl_signal,
    pub keyboard_grab_begin: wl_signal,
    pub keyboard_grab_end: wl_signal,
    pub touch_grab_begin: wl_signal,
    pub touch_grab_end: wl_signal,
    pub request_set_cursor: wl_signal,
    pub request_set_selection: wl_signal,
    pub set_selection: wl_signal,
    pub request_set_primary_selection: wl_signal,
    pub set_primary_selection: wl_signal,
    pub request_start_drag: wl_signal,
    pub start_drag: wl_signal,
    pub destroy: wl_signal,
}
// XXX this will conflict with the actual touch cancel which is different so
	// we need to rename this
/* *
 * Passed to `wlr_seat_touch_start_grab()` to start a grab of the touch device.
 * The grabber is responsible for handling touch events for the seat.
 */
/* *
 * Passed to `wlr_seat_keyboard_start_grab()` to start a grab of the keyboard.
 * The grabber is responsible for handling keyboard events for the seat.
 */
/* *
 * Passed to `wlr_seat_pointer_start_grab()` to start a grab of the pointer. The
 * grabber is responsible for handling pointer events for the seat.
 */
// wlr_seat_pointer_focus_change_event
// TODO: May be useful to be able to simulate keyboard input events
// wlr_seat_keyboard_focus_change_event

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_touch_state {
    pub seat: *mut wlr_seat,
    pub touch_points: wl_list,
    pub grab_serial: uint32_t,
    pub grab_id: uint32_t,
    pub grab: *mut wlr_seat_touch_grab,
    pub default_grab: *mut wlr_seat_touch_grab,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_touch_grab {
    pub interface: *const wlr_touch_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_touch_grab_interface {
    pub down: Option<unsafe extern "C" fn(_: *mut wlr_seat_touch_grab,
                                          _: uint32_t,
                                          _: *mut wlr_touch_point)
                         -> uint32_t>,
    pub up: Option<unsafe extern "C" fn(_: *mut wlr_seat_touch_grab,
                                        _: uint32_t, _: *mut wlr_touch_point)
                       -> ()>,
    pub motion: Option<unsafe extern "C" fn(_: *mut wlr_seat_touch_grab,
                                            _: uint32_t,
                                            _: *mut wlr_touch_point) -> ()>,
    pub enter: Option<unsafe extern "C" fn(_: *mut wlr_seat_touch_grab,
                                           _: uint32_t,
                                           _: *mut wlr_touch_point) -> ()>,
    pub cancel: Option<unsafe extern "C" fn(_: *mut wlr_seat_touch_grab)
                           -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_touch_point {
    pub touch_id: int32_t,
    pub surface: *mut wlr_surface,
    pub client: *mut wlr_seat_client,
    pub focus_surface: *mut wlr_surface,
    pub focus_client: *mut wlr_seat_client,
    pub sx: libc::c_double,
    pub sy: libc::c_double,
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
    pub events: C2RustUnnamed_7,
    pub link: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_keyboard_state {
    pub seat: *mut wlr_seat,
    pub keyboard: *mut wlr_keyboard,
    pub focused_client: *mut wlr_seat_client,
    pub focused_surface: *mut wlr_surface,
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
    pub grab: *mut wlr_seat_keyboard_grab,
    pub default_grab: *mut wlr_seat_keyboard_grab,
    pub events: C2RustUnnamed_8,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
    pub focus_change: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_keyboard_grab {
    pub interface: *const wlr_keyboard_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_keyboard_grab_interface {
    pub enter: Option<unsafe extern "C" fn(_: *mut wlr_seat_keyboard_grab,
                                           _: *mut wlr_surface,
                                           _: *mut uint32_t, _: size_t,
                                           _: *mut wlr_keyboard_modifiers)
                          -> ()>,
    pub key: Option<unsafe extern "C" fn(_: *mut wlr_seat_keyboard_grab,
                                         _: uint32_t, _: uint32_t,
                                         _: uint32_t) -> ()>,
    pub modifiers: Option<unsafe extern "C" fn(_: *mut wlr_seat_keyboard_grab,
                                               _: *mut wlr_keyboard_modifiers)
                              -> ()>,
    pub cancel: Option<unsafe extern "C" fn(_: *mut wlr_seat_keyboard_grab)
                           -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_pointer_state {
    pub seat: *mut wlr_seat,
    pub focused_client: *mut wlr_seat_client,
    pub focused_surface: *mut wlr_surface,
    pub sx: libc::c_double,
    pub sy: libc::c_double,
    pub grab: *mut wlr_seat_pointer_grab,
    pub default_grab: *mut wlr_seat_pointer_grab,
    pub buttons: [uint32_t; 16],
    pub button_count: size_t,
    pub grab_button: uint32_t,
    pub grab_serial: uint32_t,
    pub grab_time: uint32_t,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_9,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub focus_change: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_seat_pointer_grab {
    pub interface: *const wlr_pointer_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_pointer_grab_interface {
    pub enter: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab,
                                           _: *mut wlr_surface,
                                           _: libc::c_double,
                                           _: libc::c_double) -> ()>,
    pub motion: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab,
                                            _: uint32_t, _: libc::c_double,
                                            _: libc::c_double) -> ()>,
    pub button: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab,
                                            _: uint32_t, _: uint32_t,
                                            _: wlr_button_state) -> uint32_t>,
    pub axis: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab,
                                          _: uint32_t,
                                          _: wlr_axis_orientation,
                                          _: libc::c_double, _: int32_t,
                                          _: wlr_axis_source) -> ()>,
    pub frame: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab)
                          -> ()>,
    pub cancel: Option<unsafe extern "C" fn(_: *mut wlr_seat_pointer_grab)
                           -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_generic_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub resource_id: uint32_t,
    pub minor_code: uint16_t,
    pub major_code: uint8_t,
    pub pad0: uint8_t,
    pub pad: [uint32_t; 5],
    pub full_sequence: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_void_cookie_t {
    pub sequence: libc::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_pixmap_t = uint32_t;
pub type xcb_cursor_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_keycode_t = uint8_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub _class: uint8_t,
    pub bits_per_rgb_value: uint8_t,
    pub colormap_entries: uint16_t,
    pub red_mask: uint32_t,
    pub green_mask: uint32_t,
    pub blue_mask: uint32_t,
    pub pad0: [uint8_t; 4],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_visualtype_iterator_t {
    pub data: *mut xcb_visualtype_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_depth_t {
    pub depth: uint8_t,
    pub pad0: uint8_t,
    pub visuals_len: uint16_t,
    pub pad1: [uint8_t; 4],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_depth_iterator_t {
    pub data: *mut xcb_depth_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}
pub type xcb_event_mask_t = libc::c_uint;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
pub const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
pub const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
pub const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
pub const XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
pub const XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
pub const XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
pub const XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
pub const XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
pub const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
pub const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
pub const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_screen_iterator_t {
    pub data: *mut xcb_screen_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_setup_t {
    pub status: uint8_t,
    pub pad0: uint8_t,
    pub protocol_major_version: uint16_t,
    pub protocol_minor_version: uint16_t,
    pub length: uint16_t,
    pub release_number: uint32_t,
    pub resource_id_base: uint32_t,
    pub resource_id_mask: uint32_t,
    pub motion_buffer_size: uint32_t,
    pub vendor_len: uint16_t,
    pub maximum_request_length: uint16_t,
    pub roots_len: uint8_t,
    pub pixmap_formats_len: uint8_t,
    pub image_byte_order: uint8_t,
    pub bitmap_format_bit_order: uint8_t,
    pub bitmap_format_scanline_unit: uint8_t,
    pub bitmap_format_scanline_pad: uint8_t,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [uint8_t; 4],
}
pub type xcb_window_enum_t = libc::c_uint;
pub const XCB_WINDOW_NONE: xcb_window_enum_t = 0;
pub type xcb_notify_mode_t = libc::c_uint;
pub const XCB_NOTIFY_MODE_WHILE_GRABBED: xcb_notify_mode_t = 3;
pub const XCB_NOTIFY_MODE_UNGRAB: xcb_notify_mode_t = 2;
pub const XCB_NOTIFY_MODE_GRAB: xcb_notify_mode_t = 1;
pub const XCB_NOTIFY_MODE_NORMAL: xcb_notify_mode_t = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_focus_in_event_t {
    pub response_type: uint8_t,
    pub detail: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub mode: uint8_t,
    pub pad0: [uint8_t; 3],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_create_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub override_redirect: uint8_t,
    pub pad1: uint8_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_destroy_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_unmap_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub from_configure: uint8_t,
    pub pad1: [uint8_t; 3],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_map_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub override_redirect: uint8_t,
    pub pad1: [uint8_t; 3],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_map_request_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_configure_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub override_redirect: uint8_t,
    pub pad1: uint8_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_configure_request_event_t {
    pub response_type: uint8_t,
    pub stack_mode: uint8_t,
    pub sequence: uint16_t,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub sibling: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub value_mask: uint16_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_property_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub atom: xcb_atom_t,
    pub time: xcb_timestamp_t,
    pub state: uint8_t,
    pub pad1: [uint8_t; 3],
}
pub type xcb_time_t = libc::c_uint;
pub const XCB_TIME_CURRENT_TIME: xcb_time_t = 0;
pub type xcb_atom_enum_t = libc::c_uint;
pub const XCB_ATOM_WM_TRANSIENT_FOR: xcb_atom_enum_t = 68;
pub const XCB_ATOM_WM_CLASS: xcb_atom_enum_t = 67;
pub const XCB_ATOM_CAP_HEIGHT: xcb_atom_enum_t = 66;
pub const XCB_ATOM_FULL_NAME: xcb_atom_enum_t = 65;
pub const XCB_ATOM_FAMILY_NAME: xcb_atom_enum_t = 64;
pub const XCB_ATOM_FONT_NAME: xcb_atom_enum_t = 63;
pub const XCB_ATOM_NOTICE: xcb_atom_enum_t = 62;
pub const XCB_ATOM_COPYRIGHT: xcb_atom_enum_t = 61;
pub const XCB_ATOM_RESOLUTION: xcb_atom_enum_t = 60;
pub const XCB_ATOM_POINT_SIZE: xcb_atom_enum_t = 59;
pub const XCB_ATOM_WEIGHT: xcb_atom_enum_t = 58;
pub const XCB_ATOM_QUAD_WIDTH: xcb_atom_enum_t = 57;
pub const XCB_ATOM_X_HEIGHT: xcb_atom_enum_t = 56;
pub const XCB_ATOM_ITALIC_ANGLE: xcb_atom_enum_t = 55;
pub const XCB_ATOM_STRIKEOUT_DESCENT: xcb_atom_enum_t = 54;
pub const XCB_ATOM_STRIKEOUT_ASCENT: xcb_atom_enum_t = 53;
pub const XCB_ATOM_UNDERLINE_THICKNESS: xcb_atom_enum_t = 52;
pub const XCB_ATOM_UNDERLINE_POSITION: xcb_atom_enum_t = 51;
pub const XCB_ATOM_SUBSCRIPT_Y: xcb_atom_enum_t = 50;
pub const XCB_ATOM_SUBSCRIPT_X: xcb_atom_enum_t = 49;
pub const XCB_ATOM_SUPERSCRIPT_Y: xcb_atom_enum_t = 48;
pub const XCB_ATOM_SUPERSCRIPT_X: xcb_atom_enum_t = 47;
pub const XCB_ATOM_END_SPACE: xcb_atom_enum_t = 46;
pub const XCB_ATOM_MAX_SPACE: xcb_atom_enum_t = 45;
pub const XCB_ATOM_NORM_SPACE: xcb_atom_enum_t = 44;
pub const XCB_ATOM_MIN_SPACE: xcb_atom_enum_t = 43;
pub const XCB_ATOM_WM_ZOOM_HINTS: xcb_atom_enum_t = 42;
pub const XCB_ATOM_WM_SIZE_HINTS: xcb_atom_enum_t = 41;
pub const XCB_ATOM_WM_NORMAL_HINTS: xcb_atom_enum_t = 40;
pub const XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;
pub const XCB_ATOM_WM_ICON_SIZE: xcb_atom_enum_t = 38;
pub const XCB_ATOM_WM_ICON_NAME: xcb_atom_enum_t = 37;
pub const XCB_ATOM_WM_CLIENT_MACHINE: xcb_atom_enum_t = 36;
pub const XCB_ATOM_WM_HINTS: xcb_atom_enum_t = 35;
pub const XCB_ATOM_WM_COMMAND: xcb_atom_enum_t = 34;
pub const XCB_ATOM_WINDOW: xcb_atom_enum_t = 33;
pub const XCB_ATOM_VISUALID: xcb_atom_enum_t = 32;
pub const XCB_ATOM_STRING: xcb_atom_enum_t = 31;
pub const XCB_ATOM_RGB_RED_MAP: xcb_atom_enum_t = 30;
pub const XCB_ATOM_RGB_GREEN_MAP: xcb_atom_enum_t = 29;
pub const XCB_ATOM_RGB_GRAY_MAP: xcb_atom_enum_t = 28;
pub const XCB_ATOM_RGB_DEFAULT_MAP: xcb_atom_enum_t = 27;
pub const XCB_ATOM_RGB_BLUE_MAP: xcb_atom_enum_t = 26;
pub const XCB_ATOM_RGB_BEST_MAP: xcb_atom_enum_t = 25;
pub const XCB_ATOM_RGB_COLOR_MAP: xcb_atom_enum_t = 24;
pub const XCB_ATOM_RESOURCE_MANAGER: xcb_atom_enum_t = 23;
pub const XCB_ATOM_RECTANGLE: xcb_atom_enum_t = 22;
pub const XCB_ATOM_POINT: xcb_atom_enum_t = 21;
pub const XCB_ATOM_PIXMAP: xcb_atom_enum_t = 20;
pub const XCB_ATOM_INTEGER: xcb_atom_enum_t = 19;
pub const XCB_ATOM_FONT: xcb_atom_enum_t = 18;
pub const XCB_ATOM_DRAWABLE: xcb_atom_enum_t = 17;
pub const XCB_ATOM_CUT_BUFFER7: xcb_atom_enum_t = 16;
pub const XCB_ATOM_CUT_BUFFER6: xcb_atom_enum_t = 15;
pub const XCB_ATOM_CUT_BUFFER5: xcb_atom_enum_t = 14;
pub const XCB_ATOM_CUT_BUFFER4: xcb_atom_enum_t = 13;
pub const XCB_ATOM_CUT_BUFFER3: xcb_atom_enum_t = 12;
pub const XCB_ATOM_CUT_BUFFER2: xcb_atom_enum_t = 11;
pub const XCB_ATOM_CUT_BUFFER1: xcb_atom_enum_t = 10;
pub const XCB_ATOM_CUT_BUFFER0: xcb_atom_enum_t = 9;
pub const XCB_ATOM_CURSOR: xcb_atom_enum_t = 8;
pub const XCB_ATOM_COLORMAP: xcb_atom_enum_t = 7;
pub const XCB_ATOM_CARDINAL: xcb_atom_enum_t = 6;
pub const XCB_ATOM_BITMAP: xcb_atom_enum_t = 5;
pub const XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
pub const XCB_ATOM_ARC: xcb_atom_enum_t = 3;
pub const XCB_ATOM_SECONDARY: xcb_atom_enum_t = 2;
pub const XCB_ATOM_PRIMARY: xcb_atom_enum_t = 1;
pub const XCB_ATOM_ANY: xcb_atom_enum_t = 0;
pub const XCB_ATOM_NONE: xcb_atom_enum_t = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_selection_request_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}

#[repr ( C )]#[derive(Copy, Clone)]
pub union xcb_client_message_data_t {
    pub data8: [uint8_t; 20],
    pub data16: [uint16_t; 10],
    pub data32: [uint32_t; 5],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_client_message_event_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub type_0: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_value_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub bad_value: uint32_t,
    pub minor_opcode: uint16_t,
    pub major_opcode: uint8_t,
    pub pad0: uint8_t,
}
pub type xcb_window_class_t = libc::c_uint;
pub const XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
pub type xcb_cw_t = libc::c_uint;
pub const XCB_CW_CURSOR: xcb_cw_t = 16384;
pub const XCB_CW_COLORMAP: xcb_cw_t = 8192;
pub const XCB_CW_DONT_PROPAGATE: xcb_cw_t = 4096;
pub const XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
pub const XCB_CW_SAVE_UNDER: xcb_cw_t = 1024;
pub const XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 512;
pub const XCB_CW_BACKING_PIXEL: xcb_cw_t = 256;
pub const XCB_CW_BACKING_PLANES: xcb_cw_t = 128;
pub const XCB_CW_BACKING_STORE: xcb_cw_t = 64;
pub const XCB_CW_WIN_GRAVITY: xcb_cw_t = 32;
pub const XCB_CW_BIT_GRAVITY: xcb_cw_t = 16;
pub const XCB_CW_BORDER_PIXEL: xcb_cw_t = 8;
pub const XCB_CW_BORDER_PIXMAP: xcb_cw_t = 4;
pub const XCB_CW_BACK_PIXEL: xcb_cw_t = 2;
pub const XCB_CW_BACK_PIXMAP: xcb_cw_t = 1;
pub type xcb_config_window_t = libc::c_uint;
pub const XCB_CONFIG_WINDOW_STACK_MODE: xcb_config_window_t = 64;
pub const XCB_CONFIG_WINDOW_SIBLING: xcb_config_window_t = 32;
pub const XCB_CONFIG_WINDOW_BORDER_WIDTH: xcb_config_window_t = 16;
pub const XCB_CONFIG_WINDOW_HEIGHT: xcb_config_window_t = 8;
pub const XCB_CONFIG_WINDOW_WIDTH: xcb_config_window_t = 4;
pub const XCB_CONFIG_WINDOW_Y: xcb_config_window_t = 2;
pub const XCB_CONFIG_WINDOW_X: xcb_config_window_t = 1;
pub type xcb_stack_mode_t = libc::c_uint;
pub const XCB_STACK_MODE_OPPOSITE: xcb_stack_mode_t = 4;
pub const XCB_STACK_MODE_BOTTOM_IF: xcb_stack_mode_t = 3;
pub const XCB_STACK_MODE_TOP_IF: xcb_stack_mode_t = 2;
pub const XCB_STACK_MODE_BELOW: xcb_stack_mode_t = 1;
pub const XCB_STACK_MODE_ABOVE: xcb_stack_mode_t = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_get_geometry_cookie_t {
    pub sequence: libc::c_uint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_get_geometry_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub pad0: [uint8_t; 2],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_intern_atom_cookie_t {
    pub sequence: libc::c_uint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub atom: xcb_atom_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_get_atom_name_cookie_t {
    pub sequence: libc::c_uint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_get_atom_name_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub name_len: uint16_t,
    pub pad1: [uint8_t; 22],
}
pub type xcb_prop_mode_t = libc::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_get_property_cookie_t {
    pub sequence: libc::c_uint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_get_property_reply_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub type_0: xcb_atom_t,
    pub bytes_after: uint32_t,
    pub value_len: uint32_t,
    pub pad0: [uint8_t; 12],
}
pub type xcb_input_focus_t = libc::c_uint;
pub const XCB_INPUT_FOCUS_FOLLOW_KEYBOARD: xcb_input_focus_t = 3;
pub const XCB_INPUT_FOCUS_PARENT: xcb_input_focus_t = 2;
pub const XCB_INPUT_FOCUS_POINTER_ROOT: xcb_input_focus_t = 1;
pub const XCB_INPUT_FOCUS_NONE: xcb_input_focus_t = 0;
pub type xcb_image_format_t = libc::c_uint;
pub const XCB_IMAGE_FORMAT_Z_PIXMAP: xcb_image_format_t = 2;
pub const XCB_IMAGE_FORMAT_XY_PIXMAP: xcb_image_format_t = 1;
pub const XCB_IMAGE_FORMAT_XY_BITMAP: xcb_image_format_t = 0;
pub type xcb_colormap_alloc_t = libc::c_uint;
pub const XCB_COLORMAP_ALLOC_ALL: xcb_colormap_alloc_t = 1;
pub const XCB_COLORMAP_ALLOC_NONE: xcb_colormap_alloc_t = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_query_extension_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub present: uint8_t,
    pub major_opcode: uint8_t,
    pub first_event: uint8_t,
    pub first_error: uint8_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_auth_info_t {
    pub namelen: libc::c_int,
    pub name: *mut libc::c_char,
    pub datalen: libc::c_int,
    pub data: *mut libc::c_char,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwm {
    pub xwayland: *mut wlr_xwayland,
    pub event_source: *mut wl_event_source,
    pub seat: *mut wlr_seat,
    pub ping_timeout: uint32_t,
    pub atoms: [xcb_atom_t; 62],
    pub xcb_conn: *mut xcb_connection_t,
    pub screen: *mut xcb_screen_t,
    pub window: xcb_window_t,
    pub visual_id: xcb_visualid_t,
    pub colormap: xcb_colormap_t,
    pub render_format_id: xcb_render_pictformat_t,
    pub cursor: xcb_cursor_t,
    pub selection_window: xcb_window_t,
    pub clipboard_selection: wlr_xwm_selection,
    pub primary_selection: wlr_xwm_selection,
    pub dnd_window: xcb_window_t,
    pub dnd_selection: wlr_xwm_selection,
    pub focus_surface: *mut wlr_xwayland_surface,
    pub surfaces: wl_list,
    pub unpaired_surfaces: wl_list,
    pub drag: *mut crate::src::types::data_device::wlr_data_device::wlr_drag,
    pub drag_focus: *mut wlr_xwayland_surface,
    pub xfixes: *const xcb_query_extension_reply_t,
    pub errors_context: *mut xcb_errors_context_t,
    pub compositor_new_surface: wl_listener,
    pub compositor_destroy: wl_listener,
    pub seat_set_selection: wl_listener,
    pub seat_set_primary_selection: wl_listener,
    pub seat_start_drag: wl_listener,
    pub seat_drag_focus: wl_listener,
    pub seat_drag_motion: wl_listener,
    pub seat_drag_drop: wl_listener,
    pub seat_drag_destroy: wl_listener,
    pub seat_drag_source_destroy: wl_listener,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwayland_surface {
    pub window_id: xcb_window_t,
    pub xwm: *mut wlr_xwm,
    pub surface_id: uint32_t,
    pub link: wl_list,
    pub unpaired_link: wl_list,
    pub surface: *mut wlr_surface,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub saved_width: uint16_t,
    pub saved_height: uint16_t,
    pub override_redirect: bool,
    pub mapped: bool,
    pub title: *mut libc::c_char,
    pub class: *mut libc::c_char,
    pub instance: *mut libc::c_char,
    pub role: *mut libc::c_char,
    pub pid: pid_t,
    pub has_utf8_title: bool,
    pub children: wl_list,
    pub parent: *mut wlr_xwayland_surface,
    pub parent_link: wl_list,
    pub window_type: *mut xcb_atom_t,
    pub window_type_len: size_t,
    pub protocols: *mut xcb_atom_t,
    pub protocols_len: size_t,
    pub decorations: uint32_t,
    pub hints: *mut wlr_xwayland_surface_hints,
    pub hints_urgency: uint32_t,
    pub size_hints: *mut wlr_xwayland_surface_size_hints,
    pub pinging: bool,
    pub ping_timer: *mut wl_event_source,
    pub modal: bool,
    pub fullscreen: bool,
    pub maximized_vert: bool,
    pub maximized_horz: bool,
    pub has_alpha: bool,
    pub events: C2RustUnnamed_10,
    pub surface_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
    pub destroy: wl_signal,
    pub request_configure: wl_signal,
    pub request_move: wl_signal,
    pub request_resize: wl_signal,
    pub request_maximize: wl_signal,
    pub request_fullscreen: wl_signal,
    pub request_activate: wl_signal,
    pub map: wl_signal,
    pub unmap: wl_signal,
    pub set_title: wl_signal,
    pub set_class: wl_signal,
    pub set_role: wl_signal,
    pub set_parent: wl_signal,
    pub set_pid: wl_signal,
    pub set_window_type: wl_signal,
    pub set_hints: wl_signal,
    pub set_decorations: wl_signal,
    pub set_override_redirect: wl_signal,
    pub ping_timeout: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwayland_surface_size_hints {
    pub flags: uint32_t,
    pub x: int32_t,
    pub y: int32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub min_width: int32_t,
    pub min_height: int32_t,
    pub max_width: int32_t,
    pub max_height: int32_t,
    pub width_inc: int32_t,
    pub height_inc: int32_t,
    pub base_width: int32_t,
    pub base_height: int32_t,
    pub min_aspect_num: int32_t,
    pub min_aspect_den: int32_t,
    pub max_aspect_num: int32_t,
    pub max_aspect_den: int32_t,
    pub win_gravity: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwayland_surface_hints {
    pub flags: uint32_t,
    pub input: uint32_t,
    pub initial_state: int32_t,
    pub icon_pixmap: xcb_pixmap_t,
    pub icon_window: xcb_window_t,
    pub icon_x: int32_t,
    pub icon_y: int32_t,
    pub icon_mask: xcb_pixmap_t,
    pub window_group: xcb_window_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwm_selection {
    pub xwm: *mut wlr_xwm,
    pub atom: xcb_atom_t,
    pub window: xcb_window_t,
    pub owner: xcb_window_t,
    pub timestamp: xcb_timestamp_t,
    pub incoming: wlr_xwm_selection_transfer,
    pub outgoing: wl_list,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwm_selection_transfer {
    pub selection: *mut wlr_xwm_selection,
    pub incr: bool,
    pub flush_property_on_delete: bool,
    pub property_set: bool,
    pub source_data: wl_array,
    pub source_fd: libc::c_int,
    pub source: *mut wl_event_source,
    pub request: xcb_selection_request_event_t,
    pub outgoing_link: wl_list,
    pub property_start: libc::c_int,
    pub property_reply: *mut xcb_get_property_reply_t,
}
pub type xcb_render_pictformat_t = uint32_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwayland {
    pub pid: pid_t,
    pub client: *mut wl_client,
    pub sigusr1_source: *mut wl_event_source,
    pub xwm: *mut wlr_xwm,
    pub cursor: *mut crate::src::xwayland::xwayland::wlr_xwayland_cursor,
    pub wm_fd: [libc::c_int; 2],
    pub wl_fd: [libc::c_int; 2],
    pub server_start: time_t,
    pub display: libc::c_int,
    pub display_name: [libc::c_char; 16],
    pub x_fd: [libc::c_int; 2],
    pub x_fd_read_event: [*mut wl_event_source; 2],
    pub lazy: bool,
    pub wl_display: *mut wl_display,
    pub compositor: *mut wlr_compositor,
    pub seat: *mut wlr_seat,
    pub events: C2RustUnnamed_11,
    pub user_event_handler: Option<unsafe extern "C" fn(_: *mut wlr_xwm,
                                                        _:
                                                            *mut xcb_generic_event_t)
                                       -> libc::c_int>,
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
    pub ready: wl_signal,
    pub new_surface: wl_signal,
}
pub type wlr_xwayland_surface_decorations = libc::c_uint;
pub const WLR_XWAYLAND_SURFACE_DECORATIONS_NO_TITLE:
          wlr_xwayland_surface_decorations =
    2;
pub const WLR_XWAYLAND_SURFACE_DECORATIONS_NO_BORDER:
          wlr_xwayland_surface_decorations =
    1;
pub const WLR_XWAYLAND_SURFACE_DECORATIONS_ALL:
          wlr_xwayland_surface_decorations =
    0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwayland_surface_configure_event {
    pub surface: *mut wlr_xwayland_surface,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub mask: uint16_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwayland_move_event {
    pub surface: *mut wlr_xwayland_surface,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xwayland_resize_event {
    pub surface: *mut wlr_xwayland_surface,
    pub edges: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_render_pictforminfo_t {
    pub id: xcb_render_pictformat_t,
    pub type_0: uint8_t,
    pub depth: uint8_t,
    pub pad0: [uint8_t; 2],
    pub direct: xcb_render_directformat_t,
    pub colormap: xcb_colormap_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_render_directformat_t {
    pub red_shift: uint16_t,
    pub red_mask: uint16_t,
    pub green_shift: uint16_t,
    pub green_mask: uint16_t,
    pub blue_shift: uint16_t,
    pub blue_mask: uint16_t,
    pub alpha_shift: uint16_t,
    pub alpha_mask: uint16_t,
}
pub const WM_PROTOCOLS: atom_name = 2;
pub const WM_TAKE_FOCUS: atom_name = 16;
pub const WINDOW: atom_name = 17;
pub const _NET_ACTIVE_WINDOW: atom_name = 18;
pub const WM_DELETE_WINDOW: atom_name = 1;
pub const NET_WM_STATE: atom_name = 14;
pub const _NET_WM_STATE_MAXIMIZED_HORZ: atom_name = 25;
pub const _NET_WM_STATE_MAXIMIZED_VERT: atom_name = 24;
pub const _NET_WM_STATE_FULLSCREEN: atom_name = 23;
pub const _NET_WM_STATE_MODAL: atom_name = 22;
pub const _NET_CLIENT_LIST: atom_name = 61;
pub const _NET_WM_PING: atom_name = 26;
pub type atom_name = libc::c_uint;
pub const ATOM_LAST: atom_name = 62;
pub const DND_ACTION_PRIVATE: atom_name = 60;
pub const DND_ACTION_ASK: atom_name = 59;
pub const DND_ACTION_COPY: atom_name = 58;
pub const DND_ACTION_MOVE: atom_name = 57;
pub const DND_TYPE_LIST: atom_name = 56;
pub const DND_PROXY: atom_name = 55;
pub const DND_FINISHED: atom_name = 54;
pub const DND_DROP: atom_name = 53;
pub const DND_LEAVE: atom_name = 52;
pub const DND_ENTER: atom_name = 51;
pub const DND_POSITION: atom_name = 50;
pub const DND_STATUS: atom_name = 49;
pub const DND_AWARE: atom_name = 48;
pub const DND_SELECTION: atom_name = 47;
pub const NET_WM_WINDOW_TYPE_SPLASH: atom_name = 46;
pub const NET_WM_WINDOW_TYPE_NOTIFICATION: atom_name = 45;
pub const NET_WM_WINDOW_TYPE_MENU: atom_name = 44;
pub const NET_WM_WINDOW_TYPE_COMBO: atom_name = 43;
pub const NET_WM_WINDOW_TYPE_POPUP_MENU: atom_name = 42;
pub const NET_WM_WINDOW_TYPE_DROPDOWN_MENU: atom_name = 41;
pub const NET_WM_WINDOW_TYPE_DND: atom_name = 40;
pub const NET_WM_WINDOW_TYPE_TOOLTIP: atom_name = 39;
pub const NET_WM_WINDOW_TYPE_UTILITY: atom_name = 38;
pub const NET_WM_WINDOW_TYPE_NORMAL: atom_name = 37;
pub const DELETE: atom_name = 36;
pub const TIMESTAMP: atom_name = 35;
pub const TEXT: atom_name = 34;
pub const INCR: atom_name = 33;
pub const CLIPBOARD_MANAGER: atom_name = 32;
pub const TARGETS: atom_name = 31;
pub const WL_SELECTION: atom_name = 30;
pub const PRIMARY: atom_name = 29;
pub const CLIPBOARD: atom_name = 28;
pub const WM_STATE: atom_name = 27;
pub const _NET_SUPPORTING_WM_CHECK: atom_name = 21;
pub const _NET_WM_NAME: atom_name = 20;
pub const _NET_WM_MOVERESIZE: atom_name = 19;
pub const NET_WM_WINDOW_TYPE: atom_name = 15;
pub const NET_WM_NAME: atom_name = 13;
pub const NET_WM_PID: atom_name = 12;
pub const NET_WM_CM_S0: atom_name = 11;
pub const NET_SUPPORTED: atom_name = 10;
pub const WM_S0: atom_name = 9;
pub const UTF8_STRING: atom_name = 8;
pub const MOTIF_WM_HINTS: atom_name = 7;
pub const WM_WINDOW_ROLE: atom_name = 6;
pub const WM_SIZE_HINTS: atom_name = 5;
pub const WM_NORMAL_HINTS: atom_name = 4;
pub const WM_HINTS: atom_name = 3;
pub const WL_SURFACE_ID: atom_name = 0;
pub type xcb_render_picture_t = uint32_t;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_render_pictforminfo_iterator_t {
    pub data: *mut xcb_render_pictforminfo_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_render_query_pict_formats_cookie_t {
    pub sequence: libc::c_uint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_render_query_pict_formats_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub num_formats: uint32_t,
    pub num_screens: uint32_t,
    pub num_depths: uint32_t,
    pub num_visuals: uint32_t,
    pub num_subpixel: uint32_t,
    pub pad1: [uint8_t; 4],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_xfixes_query_version_cookie_t {
    pub sequence: libc::c_uint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_xfixes_query_version_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub major_version: uint32_t,
    pub minor_version: uint32_t,
    pub pad1: [uint8_t; 16],
}
pub type xcb_composite_redirect_t = libc::c_uint;
pub const XCB_COMPOSITE_REDIRECT_MANUAL: xcb_composite_redirect_t = 1;
pub const XCB_COMPOSITE_REDIRECT_AUTOMATIC: xcb_composite_redirect_t = 0;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const XCB_ICCCM_SIZE_HINT_P_WIN_GRAVITY: C2RustUnnamed_12 = 512;
pub const XCB_ICCCM_SIZE_HINT_BASE_SIZE: C2RustUnnamed_12 = 256;
pub const XCB_ICCCM_SIZE_HINT_P_ASPECT: C2RustUnnamed_12 = 128;
pub const XCB_ICCCM_SIZE_HINT_P_RESIZE_INC: C2RustUnnamed_12 = 64;
pub const XCB_ICCCM_SIZE_HINT_P_MAX_SIZE: C2RustUnnamed_12 = 32;
pub const XCB_ICCCM_SIZE_HINT_P_MIN_SIZE: C2RustUnnamed_12 = 16;
pub const XCB_ICCCM_SIZE_HINT_P_SIZE: C2RustUnnamed_12 = 8;
pub const XCB_ICCCM_SIZE_HINT_P_POSITION: C2RustUnnamed_12 = 4;
pub const XCB_ICCCM_SIZE_HINT_US_SIZE: C2RustUnnamed_12 = 2;
pub const XCB_ICCCM_SIZE_HINT_US_POSITION: C2RustUnnamed_12 = 1;

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_size_hints_t {
    pub flags: uint32_t,
    pub x: int32_t,
    pub y: int32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub min_width: int32_t,
    pub min_height: int32_t,
    pub max_width: int32_t,
    pub max_height: int32_t,
    pub width_inc: int32_t,
    pub height_inc: int32_t,
    pub min_aspect_num: int32_t,
    pub min_aspect_den: int32_t,
    pub max_aspect_num: int32_t,
    pub max_aspect_den: int32_t,
    pub base_width: int32_t,
    pub base_height: int32_t,
    pub win_gravity: uint32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct xcb_icccm_wm_hints_t {
    pub flags: int32_t,
    pub input: uint32_t,
    pub initial_state: int32_t,
    pub icon_pixmap: xcb_pixmap_t,
    pub icon_window: xcb_window_t,
    pub icon_x: int32_t,
    pub icon_y: int32_t,
    pub icon_mask: xcb_pixmap_t,
    pub window_group: xcb_window_t,
}
pub type C2RustUnnamed_13 = libc::c_uint;
pub const XCB_ICCCM_WM_HINT_X_URGENCY: C2RustUnnamed_13 = 256;
pub const XCB_ICCCM_WM_HINT_WINDOW_GROUP: C2RustUnnamed_13 = 64;
pub const XCB_ICCCM_WM_HINT_ICON_MASK: C2RustUnnamed_13 = 32;
pub const XCB_ICCCM_WM_HINT_ICON_POSITION: C2RustUnnamed_13 = 16;
pub const XCB_ICCCM_WM_HINT_ICON_WINDOW: C2RustUnnamed_13 = 8;
pub const XCB_ICCCM_WM_HINT_ICON_PIXMAP: C2RustUnnamed_13 = 4;
pub const XCB_ICCCM_WM_HINT_STATE: C2RustUnnamed_13 = 2;
pub const XCB_ICCCM_WM_HINT_INPUT: C2RustUnnamed_13 = 1;
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
pub static mut atom_map: [*const libc::c_char; 62] =
    [b"WL_SURFACE_ID\x00" as *const u8 as *const libc::c_char,
     b"WM_DELETE_WINDOW\x00" as *const u8 as *const libc::c_char,
     b"WM_PROTOCOLS\x00" as *const u8 as *const libc::c_char,
     b"WM_HINTS\x00" as *const u8 as *const libc::c_char,
     b"WM_NORMAL_HINTS\x00" as *const u8 as *const libc::c_char,
     b"WM_SIZE_HINTS\x00" as *const u8 as *const libc::c_char,
     b"WM_WINDOW_ROLE\x00" as *const u8 as *const libc::c_char,
     b"_MOTIF_WM_HINTS\x00" as *const u8 as *const libc::c_char,
     b"UTF8_STRING\x00" as *const u8 as *const libc::c_char,
     b"WM_S0\x00" as *const u8 as *const libc::c_char,
     b"_NET_SUPPORTED\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_CM_S0\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_PID\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_NAME\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_STATE\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE\x00" as *const u8 as *const libc::c_char,
     b"WM_TAKE_FOCUS\x00" as *const u8 as *const libc::c_char,
     b"WINDOW\x00" as *const u8 as *const libc::c_char,
     b"_NET_ACTIVE_WINDOW\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_MOVERESIZE\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_NAME\x00" as *const u8 as *const libc::c_char,
     b"_NET_SUPPORTING_WM_CHECK\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_STATE_MODAL\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_STATE_FULLSCREEN\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_STATE_MAXIMIZED_VERT\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_STATE_MAXIMIZED_HORZ\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_PING\x00" as *const u8 as *const libc::c_char,
     b"WM_STATE\x00" as *const u8 as *const libc::c_char,
     b"CLIPBOARD\x00" as *const u8 as *const libc::c_char,
     b"PRIMARY\x00" as *const u8 as *const libc::c_char,
     b"_WL_SELECTION\x00" as *const u8 as *const libc::c_char,
     b"TARGETS\x00" as *const u8 as *const libc::c_char,
     b"CLIPBOARD_MANAGER\x00" as *const u8 as *const libc::c_char,
     b"INCR\x00" as *const u8 as *const libc::c_char,
     b"TEXT\x00" as *const u8 as *const libc::c_char,
     b"TIMESTAMP\x00" as *const u8 as *const libc::c_char,
     b"DELETE\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE_NORMAL\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE_UTILITY\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE_TOOLTIP\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE_DND\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE_DROPDOWN_MENU\x00" as *const u8 as
         *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE_POPUP_MENU\x00" as *const u8 as
         *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE_COMBO\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE_MENU\x00" as *const u8 as *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE_NOTIFICATION\x00" as *const u8 as
         *const libc::c_char,
     b"_NET_WM_WINDOW_TYPE_SPLASH\x00" as *const u8 as *const libc::c_char,
     b"XdndSelection\x00" as *const u8 as *const libc::c_char,
     b"XdndAware\x00" as *const u8 as *const libc::c_char,
     b"XdndStatus\x00" as *const u8 as *const libc::c_char,
     b"XdndPosition\x00" as *const u8 as *const libc::c_char,
     b"XdndEnter\x00" as *const u8 as *const libc::c_char,
     b"XdndLeave\x00" as *const u8 as *const libc::c_char,
     b"XdndDrop\x00" as *const u8 as *const libc::c_char,
     b"XdndFinished\x00" as *const u8 as *const libc::c_char,
     b"XdndProxy\x00" as *const u8 as *const libc::c_char,
     b"XdndTypeList\x00" as *const u8 as *const libc::c_char,
     b"XdndActionMove\x00" as *const u8 as *const libc::c_char,
     b"XdndActionCopy\x00" as *const u8 as *const libc::c_char,
     b"XdndActionAsk\x00" as *const u8 as *const libc::c_char,
     b"XdndActionPrivate\x00" as *const u8 as *const libc::c_char,
     b"_NET_CLIENT_LIST\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn wlr_surface_is_xwayland_surface(mut surface:
                                                             *mut wlr_surface)
 -> bool {
    return (*surface).role ==
               &xwayland_surface_role as *const wlr_surface_role;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_surface_from_wlr_surface(mut surface:
                                                                   *mut wlr_surface)
 -> *mut wlr_xwayland_surface {
    if wlr_surface_is_xwayland_surface(surface) as libc::c_int != 0 {
    } else {
        __assert_fail(b"wlr_surface_is_xwayland_surface(surface)\x00" as
                          *const u8 as *const libc::c_char,
                      b"../xwayland/xwm.c\x00" as *const u8 as
                          *const libc::c_char, 92i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[libc::c_char; 89]>(b"struct wlr_xwayland_surface *wlr_xwayland_surface_from_wlr_surface(struct wlr_surface *)\x00")).as_ptr());
    };
    return (*surface).role_data as *mut wlr_xwayland_surface;
}
// TODO: replace this with hash table?
unsafe extern "C" fn lookup_surface(mut xwm: *mut wlr_xwm,
                                    mut window_id: xcb_window_t)
 -> *mut wlr_xwayland_surface {
    let mut surface: *mut wlr_xwayland_surface =
        0 as *mut wlr_xwayland_surface;
    surface =
        ((*xwm).surfaces.next as *mut libc::c_char).offset(-24) as
            *mut wlr_xwayland_surface;
    while &mut (*surface).link as *mut wl_list !=
              &mut (*xwm).surfaces as *mut wl_list {
        if (*surface).window_id == window_id { return surface }
        surface =
            ((*surface).link.next as *mut libc::c_char).offset(-24) as
                *mut wlr_xwayland_surface
    }
    return 0 as *mut wlr_xwayland_surface;
}
unsafe extern "C" fn xwayland_surface_handle_ping_timeout(mut data:
                                                              *mut libc::c_void)
 -> libc::c_int {
    let mut surface: *mut wlr_xwayland_surface =
        data as *mut wlr_xwayland_surface;
    wlr_signal_emit_safe(&mut (*surface).events.ping_timeout,
                         surface as *mut libc::c_void);
    (*surface).pinging = 0i32 != 0;
    return 1i32;
}
unsafe extern "C" fn xwayland_surface_create(mut xwm: *mut wlr_xwm,
                                             mut window_id: xcb_window_t,
                                             mut x: int16_t, mut y: int16_t,
                                             mut width: uint16_t,
                                             mut height: uint16_t,
                                             mut override_redirect: bool)
 -> *mut wlr_xwayland_surface {
    let mut surface: *mut wlr_xwayland_surface =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xwayland_surface>() as libc::c_ulong)
            as *mut wlr_xwayland_surface;
    if surface.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not allocate wlr xwayland surface\x00" as
                     *const u8 as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 122i32);
        return 0 as *mut wlr_xwayland_surface
    }
    let mut geometry_cookie: xcb_get_geometry_cookie_t =
        xcb_get_geometry((*xwm).xcb_conn, window_id);
    let mut values: [uint32_t; 1] = [0; 1];
    values[0] =
        (XCB_EVENT_MASK_FOCUS_CHANGE as libc::c_int |
             XCB_EVENT_MASK_PROPERTY_CHANGE as libc::c_int) as uint32_t;
    xcb_change_window_attributes((*xwm).xcb_conn, window_id,
                                 XCB_CW_EVENT_MASK as libc::c_int as uint32_t,
                                 values.as_mut_ptr() as *const libc::c_void);
    (*surface).xwm = xwm;
    (*surface).window_id = window_id;
    (*surface).x = x;
    (*surface).y = y;
    (*surface).width = width;
    (*surface).height = height;
    (*surface).override_redirect = override_redirect;
    wl_list_insert(&mut (*xwm).surfaces, &mut (*surface).link);
    wl_list_init(&mut (*surface).children);
    wl_list_init(&mut (*surface).parent_link);
    wl_signal_init(&mut (*surface).events.destroy);
    wl_signal_init(&mut (*surface).events.request_configure);
    wl_signal_init(&mut (*surface).events.request_move);
    wl_signal_init(&mut (*surface).events.request_resize);
    wl_signal_init(&mut (*surface).events.request_maximize);
    wl_signal_init(&mut (*surface).events.request_fullscreen);
    wl_signal_init(&mut (*surface).events.request_activate);
    wl_signal_init(&mut (*surface).events.map);
    wl_signal_init(&mut (*surface).events.unmap);
    wl_signal_init(&mut (*surface).events.set_class);
    wl_signal_init(&mut (*surface).events.set_role);
    wl_signal_init(&mut (*surface).events.set_title);
    wl_signal_init(&mut (*surface).events.set_parent);
    wl_signal_init(&mut (*surface).events.set_pid);
    wl_signal_init(&mut (*surface).events.set_window_type);
    wl_signal_init(&mut (*surface).events.set_hints);
    wl_signal_init(&mut (*surface).events.set_decorations);
    wl_signal_init(&mut (*surface).events.set_override_redirect);
    wl_signal_init(&mut (*surface).events.ping_timeout);
    let mut geometry_reply: *mut xcb_get_geometry_reply_t =
        xcb_get_geometry_reply((*xwm).xcb_conn, geometry_cookie,
                               0 as *mut *mut xcb_generic_error_t);
    if !geometry_reply.is_null() {
        (*surface).has_alpha = (*geometry_reply).depth as libc::c_int == 32i32
    }
    free(geometry_reply as *mut libc::c_void);
    let mut display: *mut wl_display = (*(*xwm).xwayland).wl_display;
    let mut loop_0: *mut wl_event_loop = wl_display_get_event_loop(display);
    (*surface).ping_timer =
        wl_event_loop_add_timer(loop_0,
                                Some(xwayland_surface_handle_ping_timeout as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_void)
                                             -> libc::c_int),
                                surface as *mut libc::c_void);
    if (*surface).ping_timer.is_null() {
        free(surface as *mut libc::c_void);
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not add timer to event loop\x00" as *const u8
                     as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 179i32);
        return 0 as *mut wlr_xwayland_surface
    }
    wlr_signal_emit_safe(&mut (*(*xwm).xwayland).events.new_surface,
                         surface as *mut libc::c_void);
    return surface;
}
unsafe extern "C" fn xwm_set_net_active_window(mut xwm: *mut wlr_xwm,
                                               mut window: xcb_window_t) {
    xcb_change_property((*xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*(*xwm).screen).root,
                        (*xwm).atoms[_NET_ACTIVE_WINDOW as libc::c_int as
                                         usize],
                        (*xwm).atoms[WINDOW as libc::c_int as usize],
                        32i32 as uint8_t, 1i32 as uint32_t,
                        &mut window as *mut xcb_window_t as
                            *const libc::c_void);
}
unsafe extern "C" fn xwm_send_wm_message(mut surface:
                                             *mut wlr_xwayland_surface,
                                         mut data:
                                             *mut xcb_client_message_data_t,
                                         mut event_mask: uint32_t) {
    let mut xwm: *mut wlr_xwm = (*surface).xwm;
    let mut event: xcb_client_message_event_t =
        {
            let mut init =
                xcb_client_message_event_t{response_type: 33i32 as uint8_t,
                                           format: 32i32 as uint8_t,
                                           sequence: 0i32 as uint16_t,
                                           window: (*surface).window_id,
                                           type_0:
                                               (*xwm).atoms[WM_PROTOCOLS as
                                                                libc::c_int as
                                                                usize],
                                           data: *data,};
            init
        };
    xcb_send_event((*xwm).xcb_conn, 0i32 as uint8_t, (*surface).window_id,
                   event_mask,
                   &mut event as *mut xcb_client_message_event_t as
                       *const libc::c_char);
    xcb_flush((*xwm).xcb_conn);
}
unsafe extern "C" fn xwm_set_net_client_list(mut xwm: *mut wlr_xwm) {
    let mut mapped_surfaces: size_t = 0i32 as size_t;
    let mut surface: *mut wlr_xwayland_surface =
        0 as *mut wlr_xwayland_surface;
    surface =
        ((*xwm).surfaces.next as *mut libc::c_char).offset(-24) as
            *mut wlr_xwayland_surface;
    while &mut (*surface).link as *mut wl_list !=
              &mut (*xwm).surfaces as *mut wl_list {
        if (*surface).mapped {
            mapped_surfaces = mapped_surfaces.wrapping_add(1)
        }
        surface =
            ((*surface).link.next as *mut libc::c_char).offset(-24) as
                *mut wlr_xwayland_surface
    }
    let vla = mapped_surfaces.wrapping_add(1i32 as libc::c_ulong) as usize;
    let mut windows: Vec<xcb_window_t> = ::std::vec::from_elem(0, vla);
    let mut index: size_t = 0i32 as size_t;
    surface =
        ((*xwm).surfaces.next as *mut libc::c_char).offset(-24) as
            *mut wlr_xwayland_surface;
    while &mut (*surface).link as *mut wl_list !=
              &mut (*xwm).surfaces as *mut wl_list {
        if (*surface).mapped {
            let fresh0 = index;
            index = index.wrapping_add(1);
            *windows.as_mut_ptr().offset(fresh0 as isize) =
                (*surface).window_id
        }
        surface =
            ((*surface).link.next as *mut libc::c_char).offset(-24) as
                *mut wlr_xwayland_surface
    }
    xcb_change_property((*xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*(*xwm).screen).root,
                        (*xwm).atoms[_NET_CLIENT_LIST as libc::c_int as
                                         usize],
                        XCB_ATOM_WINDOW as libc::c_int as xcb_atom_t,
                        32i32 as uint8_t, mapped_surfaces as uint32_t,
                        windows.as_mut_ptr() as *const libc::c_void);
}
unsafe extern "C" fn xwm_send_focus_window(mut xwm: *mut wlr_xwm,
                                           mut xsurface:
                                               *mut wlr_xwayland_surface) {
    if xsurface.is_null() {
        xcb_set_input_focus_checked((*xwm).xcb_conn,
                                    XCB_INPUT_FOCUS_POINTER_ROOT as
                                        libc::c_int as uint8_t,
                                    0i64 as xcb_window_t,
                                    0i64 as xcb_timestamp_t);
        return
    }
    if (*xsurface).override_redirect { return }
    let mut message_data: xcb_client_message_data_t =
        xcb_client_message_data_t{data8:
                                      [0i32 as uint8_t, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
    message_data.data32[0] =
        (*xwm).atoms[WM_TAKE_FOCUS as libc::c_int as usize];
    message_data.data32[1] = XCB_TIME_CURRENT_TIME as libc::c_int as uint32_t;
    if !(*xsurface).hints.is_null() && (*(*xsurface).hints).input == 0 {
        // if the surface doesn't allow the focus request, we will send him
		// only the take focus event. It will get the focus by itself.
        xwm_send_wm_message(xsurface, &mut message_data,
                            XCB_EVENT_MASK_NO_EVENT as libc::c_int as
                                uint32_t);
    } else {
        xwm_send_wm_message(xsurface, &mut message_data,
                            XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as
                                libc::c_int as uint32_t);
        xcb_set_input_focus((*xwm).xcb_conn,
                            XCB_INPUT_FOCUS_POINTER_ROOT as libc::c_int as
                                uint8_t, (*xsurface).window_id,
                            0i64 as xcb_timestamp_t);
    }
    let mut values: [uint32_t; 1] = [0; 1];
    values[0] = XCB_STACK_MODE_ABOVE as libc::c_int as uint32_t;
    xcb_configure_window((*xwm).xcb_conn, (*xsurface).window_id,
                         XCB_CONFIG_WINDOW_STACK_MODE as libc::c_int as
                             uint16_t,
                         values.as_mut_ptr() as *const libc::c_void);
}
unsafe extern "C" fn xwm_surface_activate(mut xwm: *mut wlr_xwm,
                                          mut xsurface:
                                              *mut wlr_xwayland_surface) {
    if (*xwm).focus_surface == xsurface ||
           !xsurface.is_null() &&
               (*xsurface).override_redirect as libc::c_int != 0 {
        return
    }
    if !xsurface.is_null() {
        xwm_set_net_active_window(xwm, (*xsurface).window_id);
    } else {
        xwm_set_net_active_window(xwm,
                                  XCB_WINDOW_NONE as libc::c_int as
                                      xcb_window_t);
    }
    xwm_send_focus_window(xwm, xsurface);
    (*xwm).focus_surface = xsurface;
    xcb_flush((*xwm).xcb_conn);
}
unsafe extern "C" fn xsurface_set_net_wm_state(mut xsurface:
                                                   *mut wlr_xwayland_surface) {
    let mut xwm: *mut wlr_xwm = (*xsurface).xwm;
    let mut property: [uint32_t; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    i = 0i32;
    if (*xsurface).modal {
        let fresh1 = i;
        i = i + 1;
        property[fresh1 as usize] =
            (*xwm).atoms[_NET_WM_STATE_MODAL as libc::c_int as usize]
    }
    if (*xsurface).fullscreen {
        let fresh2 = i;
        i = i + 1;
        property[fresh2 as usize] =
            (*xwm).atoms[_NET_WM_STATE_FULLSCREEN as libc::c_int as usize]
    }
    if (*xsurface).maximized_vert {
        let fresh3 = i;
        i = i + 1;
        property[fresh3 as usize] =
            (*xwm).atoms[_NET_WM_STATE_MAXIMIZED_VERT as libc::c_int as usize]
    }
    if (*xsurface).maximized_horz {
        let fresh4 = i;
        i = i + 1;
        property[fresh4 as usize] =
            (*xwm).atoms[_NET_WM_STATE_MAXIMIZED_HORZ as libc::c_int as usize]
    }
    xcb_change_property((*xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*xsurface).window_id,
                        (*xwm).atoms[NET_WM_STATE as libc::c_int as usize],
                        XCB_ATOM_ATOM as libc::c_int as xcb_atom_t,
                        32i32 as uint8_t, i as uint32_t,
                        property.as_mut_ptr() as *const libc::c_void);
}
unsafe extern "C" fn xwayland_surface_destroy(mut xsurface:
                                                  *mut wlr_xwayland_surface) {
    xsurface_unmap(xsurface);
    wlr_signal_emit_safe(&mut (*xsurface).events.destroy,
                         xsurface as *mut libc::c_void);
    if xsurface == (*(*xsurface).xwm).focus_surface {
        xwm_surface_activate((*xsurface).xwm, 0 as *mut wlr_xwayland_surface);
    }
    wl_list_remove(&mut (*xsurface).link);
    wl_list_remove(&mut (*xsurface).parent_link);
    let mut child: *mut wlr_xwayland_surface = 0 as *mut wlr_xwayland_surface;
    let mut next: *mut wlr_xwayland_surface = 0 as *mut wlr_xwayland_surface;
    child =
        ((*xsurface).children.next as *mut libc::c_char).offset(-144) as
            *mut wlr_xwayland_surface;
    next =
        ((*child).parent_link.next as *mut libc::c_char).offset(-144) as
            *mut wlr_xwayland_surface;
    while &mut (*child).parent_link as *mut wl_list !=
              &mut (*xsurface).children as *mut wl_list {
        wl_list_remove(&mut (*child).parent_link);
        wl_list_init(&mut (*child).parent_link);
        (*child).parent = 0 as *mut wlr_xwayland_surface;
        child = next;
        next =
            ((*child).parent_link.next as *mut libc::c_char).offset(-144) as
                *mut wlr_xwayland_surface
    }
    if (*xsurface).surface_id != 0 {
        wl_list_remove(&mut (*xsurface).unpaired_link);
    }
    if !(*xsurface).surface.is_null() {
        wl_list_remove(&mut (*xsurface).surface_destroy.link);
        (*(*xsurface).surface).role_data = 0 as *mut libc::c_void
    }
    wl_event_source_remove((*xsurface).ping_timer);
    free((*xsurface).title as *mut libc::c_void);
    free((*xsurface).class as *mut libc::c_void);
    free((*xsurface).instance as *mut libc::c_void);
    free((*xsurface).role as *mut libc::c_void);
    free((*xsurface).window_type as *mut libc::c_void);
    free((*xsurface).protocols as *mut libc::c_void);
    free((*xsurface).hints as *mut libc::c_void);
    free((*xsurface).size_hints as *mut libc::c_void);
    free(xsurface as *mut libc::c_void);
}
unsafe extern "C" fn read_surface_class(mut xwm: *mut wlr_xwm,
                                        mut surface:
                                            *mut wlr_xwayland_surface,
                                        mut reply:
                                            *mut xcb_get_property_reply_t) {
    if (*reply).type_0 != XCB_ATOM_STRING as libc::c_int as libc::c_uint &&
           (*reply).type_0 !=
               (*xwm).atoms[UTF8_STRING as libc::c_int as usize] {
        return
    }
    let mut len: size_t = xcb_get_property_value_length(reply) as size_t;
    let mut class: *mut libc::c_char =
        xcb_get_property_value(reply) as *mut libc::c_char;
    // Unpack two sequentially stored strings: instance, class
    let mut instance_len: size_t = strnlen(class, len);
    free((*surface).instance as *mut libc::c_void);
    if len > 0i32 as libc::c_ulong && instance_len < len {
        (*surface).instance = strndup(class, instance_len);
        class =
            class.offset(instance_len.wrapping_add(1i32 as libc::c_ulong) as
                             isize)
    } else { (*surface).instance = 0 as *mut libc::c_char }
    free((*surface).class as *mut libc::c_void);
    if len > 0i32 as libc::c_ulong {
        (*surface).class = strndup(class, len)
    } else { (*surface).class = 0 as *mut libc::c_char }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_ATOM_WM_CLASS: %s %s\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             391i32, (*surface).instance, (*surface).class);
    wlr_signal_emit_safe(&mut (*surface).events.set_class,
                         surface as *mut libc::c_void);
}
unsafe extern "C" fn read_surface_role(mut xwm: *mut wlr_xwm,
                                       mut xsurface:
                                           *mut wlr_xwayland_surface,
                                       mut reply:
                                           *mut xcb_get_property_reply_t) {
    if (*reply).type_0 != XCB_ATOM_STRING as libc::c_int as libc::c_uint &&
           (*reply).type_0 !=
               (*xwm).atoms[UTF8_STRING as libc::c_int as usize] {
        return
    }
    let mut len: size_t = xcb_get_property_value_length(reply) as size_t;
    let mut role: *mut libc::c_char =
        xcb_get_property_value(reply) as *mut libc::c_char;
    free((*xsurface).role as *mut libc::c_void);
    if len > 0i32 as libc::c_ulong {
        (*xsurface).role = strndup(role, len)
    } else { (*xsurface).role = 0 as *mut libc::c_char }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_ATOM_WM_WINDOW_ROLE: %s\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             413i32, (*xsurface).role);
    wlr_signal_emit_safe(&mut (*xsurface).events.set_role,
                         xsurface as *mut libc::c_void);
}
unsafe extern "C" fn read_surface_title(mut xwm: *mut wlr_xwm,
                                        mut xsurface:
                                            *mut wlr_xwayland_surface,
                                        mut reply:
                                            *mut xcb_get_property_reply_t) {
    if (*reply).type_0 != XCB_ATOM_STRING as libc::c_int as libc::c_uint &&
           (*reply).type_0 !=
               (*xwm).atoms[UTF8_STRING as libc::c_int as usize] {
        return
    }
    let mut is_utf8: bool =
        (*reply).type_0 == (*xwm).atoms[UTF8_STRING as libc::c_int as usize];
    if !is_utf8 && (*xsurface).has_utf8_title as libc::c_int != 0 { return }
    let mut len: size_t = xcb_get_property_value_length(reply) as size_t;
    let mut title: *mut libc::c_char =
        xcb_get_property_value(reply) as *mut libc::c_char;
    free((*xsurface).title as *mut libc::c_void);
    if len > 0i32 as libc::c_ulong {
        (*xsurface).title = strndup(title, len)
    } else { (*xsurface).title = 0 as *mut libc::c_char }
    (*xsurface).has_utf8_title = is_utf8;
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_ATOM_WM_NAME: %s\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             441i32, (*xsurface).title);
    wlr_signal_emit_safe(&mut (*xsurface).events.set_title,
                         xsurface as *mut libc::c_void);
}
unsafe extern "C" fn read_surface_parent(mut xwm: *mut wlr_xwm,
                                         mut xsurface:
                                             *mut wlr_xwayland_surface,
                                         mut reply:
                                             *mut xcb_get_property_reply_t) {
    if (*reply).type_0 != XCB_ATOM_WINDOW as libc::c_int as libc::c_uint {
        return
    }
    let mut xid: *mut xcb_window_t =
        xcb_get_property_value(reply) as *mut xcb_window_t;
    if !xid.is_null() {
        (*xsurface).parent = lookup_surface(xwm, *xid)
    } else { (*xsurface).parent = 0 as *mut wlr_xwayland_surface }
    wl_list_remove(&mut (*xsurface).parent_link);
    if !(*xsurface).parent.is_null() {
        wl_list_insert(&mut (*(*xsurface).parent).children,
                       &mut (*xsurface).parent_link);
    } else { wl_list_init(&mut (*xsurface).parent_link); }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_ATOM_WM_TRANSIENT_FOR: %p\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             466i32, (*xsurface).parent);
    wlr_signal_emit_safe(&mut (*xsurface).events.set_parent,
                         xsurface as *mut libc::c_void);
}
unsafe extern "C" fn read_surface_pid(mut xwm: *mut wlr_xwm,
                                      mut xsurface: *mut wlr_xwayland_surface,
                                      mut reply:
                                          *mut xcb_get_property_reply_t) {
    if (*reply).type_0 != XCB_ATOM_CARDINAL as libc::c_int as libc::c_uint {
        return
    }
    let mut pid: *mut pid_t = xcb_get_property_value(reply) as *mut pid_t;
    (*xsurface).pid = *pid;
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] NET_WM_PID %d\x00" as *const u8 as *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             479i32, (*xsurface).pid);
    wlr_signal_emit_safe(&mut (*xsurface).events.set_pid,
                         xsurface as *mut libc::c_void);
}
unsafe extern "C" fn read_surface_window_type(mut xwm: *mut wlr_xwm,
                                              mut xsurface:
                                                  *mut wlr_xwayland_surface,
                                              mut reply:
                                                  *mut xcb_get_property_reply_t) {
    if (*reply).type_0 != XCB_ATOM_ATOM as libc::c_int as libc::c_uint {
        return
    }
    let mut atoms: *mut xcb_atom_t =
        xcb_get_property_value(reply) as *mut xcb_atom_t;
    let mut atoms_len: size_t = (*reply).value_len as size_t;
    let mut atoms_size: size_t =
        (::std::mem::size_of::<xcb_atom_t>() as
             libc::c_ulong).wrapping_mul(atoms_len);
    free((*xsurface).window_type as *mut libc::c_void);
    (*xsurface).window_type = malloc(atoms_size) as *mut xcb_atom_t;
    if (*xsurface).window_type.is_null() { return }
    memcpy((*xsurface).window_type as *mut libc::c_void,
           atoms as *const libc::c_void, atoms_size);
    (*xsurface).window_type_len = atoms_len;
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] NET_WM_WINDOW_TYPE (%zu)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             502i32, atoms_len);
    wlr_signal_emit_safe(&mut (*xsurface).events.set_window_type,
                         xsurface as *mut libc::c_void);
}
unsafe extern "C" fn read_surface_protocols(mut xwm: *mut wlr_xwm,
                                            mut xsurface:
                                                *mut wlr_xwayland_surface,
                                            mut reply:
                                                *mut xcb_get_property_reply_t) {
    if (*reply).type_0 != XCB_ATOM_ATOM as libc::c_int as libc::c_uint {
        return
    }
    let mut atoms: *mut xcb_atom_t =
        xcb_get_property_value(reply) as *mut xcb_atom_t;
    let mut atoms_len: size_t = (*reply).value_len as size_t;
    let mut atoms_size: size_t =
        (::std::mem::size_of::<xcb_atom_t>() as
             libc::c_ulong).wrapping_mul(atoms_len);
    free((*xsurface).protocols as *mut libc::c_void);
    (*xsurface).protocols = malloc(atoms_size) as *mut xcb_atom_t;
    if (*xsurface).protocols.is_null() { return }
    memcpy((*xsurface).protocols as *mut libc::c_void,
           atoms as *const libc::c_void, atoms_size);
    (*xsurface).protocols_len = atoms_len;
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] WM_PROTOCOLS (%zu)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             525i32, atoms_len);
}
unsafe extern "C" fn read_surface_hints(mut xwm: *mut wlr_xwm,
                                        mut xsurface:
                                            *mut wlr_xwayland_surface,
                                        mut reply:
                                            *mut xcb_get_property_reply_t) {
    // According to the docs, reply->type == xwm->atoms[WM_HINTS]
	// In practice, reply->type == XCB_ATOM_ATOM
    if (*reply).value_len == 0i32 as libc::c_uint { return }
    let mut hints: xcb_icccm_wm_hints_t =
        xcb_icccm_wm_hints_t{flags: 0,
                             input: 0,
                             initial_state: 0,
                             icon_pixmap: 0,
                             icon_window: 0,
                             icon_x: 0,
                             icon_y: 0,
                             icon_mask: 0,
                             window_group: 0,};
    xcb_icccm_get_wm_hints_from_reply(&mut hints, reply);
    free((*xsurface).hints as *mut libc::c_void);
    (*xsurface).hints =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xwayland_surface_hints>() as
                   libc::c_ulong) as *mut wlr_xwayland_surface_hints;
    if (*xsurface).hints.is_null() { return }
    memcpy((*xsurface).hints as *mut libc::c_void,
           &mut hints as *mut xcb_icccm_wm_hints_t as *const libc::c_void,
           ::std::mem::size_of::<wlr_xwayland_surface_hints>() as
               libc::c_ulong);
    (*xsurface).hints_urgency = xcb_icccm_wm_hints_get_urgency(&mut hints);
    if (*(*xsurface).hints).flags &
           XCB_ICCCM_WM_HINT_INPUT as libc::c_int as libc::c_uint == 0 {
        // The client didn't specify whether it wants input.
		// Assume it does.
        (*(*xsurface).hints).input = 1i32 as uint32_t
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] WM_HINTS (%d)\x00" as *const u8 as *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             555i32, (*reply).value_len);
    wlr_signal_emit_safe(&mut (*xsurface).events.set_hints,
                         xsurface as *mut libc::c_void);
}
unsafe extern "C" fn read_surface_normal_hints(mut xwm: *mut wlr_xwm,
                                               mut xsurface:
                                                   *mut wlr_xwayland_surface,
                                               mut reply:
                                                   *mut xcb_get_property_reply_t) {
    if (*reply).type_0 != (*xwm).atoms[WM_SIZE_HINTS as libc::c_int as usize]
           || (*reply).value_len == 0i32 as libc::c_uint {
        return
    }
    let mut size_hints: xcb_size_hints_t =
        xcb_size_hints_t{flags: 0,
                         x: 0,
                         y: 0,
                         width: 0,
                         height: 0,
                         min_width: 0,
                         min_height: 0,
                         max_width: 0,
                         max_height: 0,
                         width_inc: 0,
                         height_inc: 0,
                         min_aspect_num: 0,
                         min_aspect_den: 0,
                         max_aspect_num: 0,
                         max_aspect_den: 0,
                         base_width: 0,
                         base_height: 0,
                         win_gravity: 0,};
    xcb_icccm_get_wm_size_hints_from_reply(&mut size_hints, reply);
    free((*xsurface).size_hints as *mut libc::c_void);
    (*xsurface).size_hints =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xwayland_surface_size_hints>() as
                   libc::c_ulong) as *mut wlr_xwayland_surface_size_hints;
    if (*xsurface).size_hints.is_null() { return }
    memcpy((*xsurface).size_hints as *mut libc::c_void,
           &mut size_hints as *mut xcb_size_hints_t as *const libc::c_void,
           ::std::mem::size_of::<wlr_xwayland_surface_size_hints>() as
               libc::c_ulong);
    let mut has_min_size_hints: bool =
        size_hints.flags &
            XCB_ICCCM_SIZE_HINT_P_MIN_SIZE as libc::c_int as libc::c_uint !=
            0i32 as libc::c_uint;
    let mut has_base_size_hints: bool =
        size_hints.flags &
            XCB_ICCCM_SIZE_HINT_BASE_SIZE as libc::c_int as libc::c_uint !=
            0i32 as libc::c_uint;
    /* ICCCM says that if absent, min size is equal to base size and vice versa */
    if !has_min_size_hints && !has_base_size_hints {
        (*(*xsurface).size_hints).min_width = -1i32; // not a C string
        (*(*xsurface).size_hints).min_height = -1i32;
        (*(*xsurface).size_hints).base_width = -1i32;
        (*(*xsurface).size_hints).base_height = -1i32
    } else if !has_base_size_hints {
        (*(*xsurface).size_hints).base_width =
            (*(*xsurface).size_hints).min_width;
        (*(*xsurface).size_hints).base_height =
            (*(*xsurface).size_hints).min_height
    } else if !has_min_size_hints {
        (*(*xsurface).size_hints).min_width =
            (*(*xsurface).size_hints).base_width;
        (*(*xsurface).size_hints).min_height =
            (*(*xsurface).size_hints).base_height
    }
    if size_hints.flags &
           XCB_ICCCM_SIZE_HINT_P_MAX_SIZE as libc::c_int as libc::c_uint ==
           0i32 as libc::c_uint {
        (*(*xsurface).size_hints).max_width = -1i32;
        (*(*xsurface).size_hints).max_height = -1i32
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] WM_NORMAL_HINTS (%d)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             607i32, (*reply).value_len);
}
unsafe extern "C" fn read_surface_motif_hints(mut xwm: *mut wlr_xwm,
                                              mut xsurface:
                                                  *mut wlr_xwayland_surface,
                                              mut reply:
                                                  *mut xcb_get_property_reply_t) {
    if (*reply).value_len < 5i32 as libc::c_uint { return }
    let mut motif_hints: *mut uint32_t =
        xcb_get_property_value(reply) as *mut uint32_t;
    if *motif_hints.offset(0) & (1i32 << 1i32) as libc::c_uint != 0 {
        (*xsurface).decorations =
            WLR_XWAYLAND_SURFACE_DECORATIONS_ALL as libc::c_int as uint32_t;
        let mut decorations: uint32_t = *motif_hints.offset(2);
        if decorations & (1i32 << 0i32) as libc::c_uint ==
               0i32 as libc::c_uint {
            if decorations & (1i32 << 1i32) as libc::c_uint ==
                   0i32 as libc::c_uint {
                (*xsurface).decorations |=
                    WLR_XWAYLAND_SURFACE_DECORATIONS_NO_BORDER as libc::c_int
                        as libc::c_uint
            }
            if decorations & (1i32 << 3i32) as libc::c_uint ==
                   0i32 as libc::c_uint {
                (*xsurface).decorations |=
                    WLR_XWAYLAND_SURFACE_DECORATIONS_NO_TITLE as libc::c_int
                        as libc::c_uint
            }
        }
        wlr_signal_emit_safe(&mut (*xsurface).events.set_decorations,
                             xsurface as *mut libc::c_void);
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] MOTIF_WM_HINTS (%d)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             651i32, (*reply).value_len);
}
unsafe extern "C" fn read_surface_net_wm_state(mut xwm: *mut wlr_xwm,
                                               mut xsurface:
                                                   *mut wlr_xwayland_surface,
                                               mut reply:
                                                   *mut xcb_get_property_reply_t) {
    (*xsurface).fullscreen = 0i32 != 0;
    let mut atom: *mut xcb_atom_t =
        xcb_get_property_value(reply) as *mut xcb_atom_t;
    let mut i: uint32_t = 0i32 as uint32_t;
    while i < (*reply).value_len {
        if *atom.offset(i as isize) ==
               (*xwm).atoms[_NET_WM_STATE_MODAL as libc::c_int as usize] {
            (*xsurface).modal = 1i32 != 0
        } else if *atom.offset(i as isize) ==
                      (*xwm).atoms[_NET_WM_STATE_FULLSCREEN as libc::c_int as
                                       usize] {
            (*xsurface).fullscreen = 1i32 != 0
        } else if *atom.offset(i as isize) ==
                      (*xwm).atoms[_NET_WM_STATE_MAXIMIZED_VERT as libc::c_int
                                       as usize] {
            (*xsurface).maximized_vert = 1i32 != 0
        } else if *atom.offset(i as isize) ==
                      (*xwm).atoms[_NET_WM_STATE_MAXIMIZED_HORZ as libc::c_int
                                       as usize] {
            (*xsurface).maximized_horz = 1i32 != 0
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xwm_get_atom_name(mut xwm: *mut wlr_xwm,
                                           mut atom: xcb_atom_t)
 -> *mut libc::c_char {
    let mut name_cookie: xcb_get_atom_name_cookie_t =
        xcb_get_atom_name((*xwm).xcb_conn, atom);
    let mut name_reply: *mut xcb_get_atom_name_reply_t =
        xcb_get_atom_name_reply((*xwm).xcb_conn, name_cookie,
                                0 as *mut *mut xcb_generic_error_t);
    if name_reply.is_null() { return 0 as *mut libc::c_char }
    let mut len: size_t = xcb_get_atom_name_name_length(name_reply) as size_t;
    let mut buf: *mut libc::c_char = xcb_get_atom_name_name(name_reply);
    let mut name: *mut libc::c_char = strndup(buf, len);
    free(name_reply as *mut libc::c_void);
    return name;
}
unsafe extern "C" fn read_surface_property(mut xwm: *mut wlr_xwm,
                                           mut xsurface:
                                               *mut wlr_xwayland_surface,
                                           mut property: xcb_atom_t) {
    let mut cookie: xcb_get_property_cookie_t =
        xcb_get_property((*xwm).xcb_conn, 0i32 as uint8_t,
                         (*xsurface).window_id, property,
                         XCB_ATOM_ANY as libc::c_int as xcb_atom_t,
                         0i32 as uint32_t, 2048i32 as uint32_t);
    let mut reply: *mut xcb_get_property_reply_t =
        xcb_get_property_reply((*xwm).xcb_conn, cookie,
                               0 as *mut *mut xcb_generic_error_t);
    if reply.is_null() { return }
    if property == XCB_ATOM_WM_CLASS as libc::c_int as libc::c_uint {
        read_surface_class(xwm, xsurface, reply);
    } else if property == XCB_ATOM_WM_NAME as libc::c_int as libc::c_uint ||
                  property ==
                      (*xwm).atoms[NET_WM_NAME as libc::c_int as usize] {
        read_surface_title(xwm, xsurface, reply);
    } else if property ==
                  XCB_ATOM_WM_TRANSIENT_FOR as libc::c_int as libc::c_uint {
        read_surface_parent(xwm, xsurface, reply);
    } else if property == (*xwm).atoms[NET_WM_PID as libc::c_int as usize] {
        read_surface_pid(xwm, xsurface, reply);
    } else if property ==
                  (*xwm).atoms[NET_WM_WINDOW_TYPE as libc::c_int as usize] {
        read_surface_window_type(xwm, xsurface, reply);
    } else if property == (*xwm).atoms[WM_PROTOCOLS as libc::c_int as usize] {
        read_surface_protocols(xwm, xsurface, reply);
    } else if property == (*xwm).atoms[NET_WM_STATE as libc::c_int as usize] {
        read_surface_net_wm_state(xwm, xsurface, reply);
    } else if property == (*xwm).atoms[WM_HINTS as libc::c_int as usize] {
        read_surface_hints(xwm, xsurface, reply);
    } else if property ==
                  (*xwm).atoms[WM_NORMAL_HINTS as libc::c_int as usize] {
        read_surface_normal_hints(xwm, xsurface, reply);
    } else if property == (*xwm).atoms[MOTIF_WM_HINTS as libc::c_int as usize]
     {
        read_surface_motif_hints(xwm, xsurface, reply);
    } else if property == (*xwm).atoms[WM_WINDOW_ROLE as libc::c_int as usize]
     {
        read_surface_role(xwm, xsurface, reply);
    } else {
        let mut prop_name: *mut libc::c_char =
            xwm_get_atom_name(xwm, property);
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] unhandled X11 property %u (%s) for window %u\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 723i32, property, prop_name, (*xsurface).window_id);
        free(prop_name as *mut libc::c_void);
    }
    free(reply as *mut libc::c_void);
}
unsafe extern "C" fn xwayland_surface_role_commit(mut wlr_surface:
                                                      *mut wlr_surface) {
    if (*wlr_surface).role ==
           &xwayland_surface_role as *const wlr_surface_role {
    } else {
        __assert_fail(b"wlr_surface->role == &xwayland_surface_role\x00" as
                          *const u8 as *const libc::c_char,
                      b"../xwayland/xwm.c\x00" as *const u8 as
                          *const libc::c_char, 731i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 56],
                                                &[libc::c_char; 56]>(b"void xwayland_surface_role_commit(struct wlr_surface *)\x00")).as_ptr());
    };
    let mut surface: *mut wlr_xwayland_surface =
        (*wlr_surface).role_data as *mut wlr_xwayland_surface;
    if surface.is_null() { return }
    if !(*surface).mapped &&
           wlr_surface_has_buffer((*surface).surface) as libc::c_int != 0 {
        wlr_signal_emit_safe(&mut (*surface).events.map,
                             surface as *mut libc::c_void);
        (*surface).mapped = 1i32 != 0;
        xwm_set_net_client_list((*surface).xwm);
    };
}
unsafe extern "C" fn xwayland_surface_role_precommit(mut wlr_surface:
                                                         *mut wlr_surface) {
    if (*wlr_surface).role ==
           &xwayland_surface_role as *const wlr_surface_role {
    } else {
        __assert_fail(b"wlr_surface->role == &xwayland_surface_role\x00" as
                          *const u8 as *const libc::c_char,
                      b"../xwayland/xwm.c\x00" as *const u8 as
                          *const libc::c_char, 745i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 59],
                                                &[libc::c_char; 59]>(b"void xwayland_surface_role_precommit(struct wlr_surface *)\x00")).as_ptr());
    };
    let mut surface: *mut wlr_xwayland_surface =
        (*wlr_surface).role_data as *mut wlr_xwayland_surface;
    if surface.is_null() { return }
    if (*wlr_surface).pending.committed &
           WLR_SURFACE_STATE_BUFFER as libc::c_int as libc::c_uint != 0 &&
           (*wlr_surface).pending.buffer_resource.is_null() {
        // This is a NULL commit
        if (*surface).mapped {
            wlr_signal_emit_safe(&mut (*surface).events.unmap,
                                 surface as *mut libc::c_void);
            (*surface).mapped = 0i32 != 0;
            xwm_set_net_client_list((*surface).xwm);
        }
    };
}
static mut xwayland_surface_role: wlr_surface_role =
    {
    
        {
            let mut init =
                wlr_surface_role{name:
                                     b"wlr_xwayland_surface\x00" as *const u8
                                         as *const libc::c_char,
                                 commit:
                                     Some(xwayland_surface_role_commit as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_surface)
                                                  -> ()),
                                 precommit:
                                     Some(xwayland_surface_role_precommit as
                                              unsafe extern "C" fn(_:
                                                                       *mut wlr_surface)
                                                  -> ()),};
            init
        }
};
unsafe extern "C" fn handle_surface_destroy(mut listener: *mut wl_listener,
                                            mut data: *mut libc::c_void) {
    let mut surface: *mut wlr_xwayland_surface =
        (listener as *mut libc::c_char).offset(-552) as
            *mut wlr_xwayland_surface;
    xsurface_unmap(surface);
}
unsafe extern "C" fn xwm_map_shell_surface(mut xwm: *mut wlr_xwm,
                                           mut xsurface:
                                               *mut wlr_xwayland_surface,
                                           mut surface: *mut wlr_surface) {
    if !wlr_surface_set_role(surface, &xwayland_surface_role,
                             xsurface as *mut libc::c_void,
                             0 as *mut wl_resource, 0i32 as uint32_t) {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to set xwayland surface role\x00" as
                     *const u8 as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 778i32);
        return
    }
    (*xsurface).surface = surface;
    // read all surface properties
    let props: [xcb_atom_t; 11] =
        [XCB_ATOM_WM_CLASS as libc::c_int as xcb_atom_t,
         XCB_ATOM_WM_NAME as libc::c_int as xcb_atom_t,
         XCB_ATOM_WM_TRANSIENT_FOR as libc::c_int as xcb_atom_t,
         (*xwm).atoms[WM_PROTOCOLS as libc::c_int as usize],
         (*xwm).atoms[WM_HINTS as libc::c_int as usize],
         (*xwm).atoms[WM_NORMAL_HINTS as libc::c_int as usize],
         (*xwm).atoms[MOTIF_WM_HINTS as libc::c_int as usize],
         (*xwm).atoms[NET_WM_STATE as libc::c_int as usize],
         (*xwm).atoms[NET_WM_WINDOW_TYPE as libc::c_int as usize],
         (*xwm).atoms[NET_WM_NAME as libc::c_int as usize],
         (*xwm).atoms[NET_WM_PID as libc::c_int as usize]];
    let mut i: size_t = 0i32 as size_t;
    while i <
              (::std::mem::size_of::<[xcb_atom_t; 11]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<xcb_atom_t>()
                                                   as libc::c_ulong) {
        read_surface_property(xwm, xsurface, props[i as usize]);
        i = i.wrapping_add(1)
    }
    (*xsurface).surface_destroy.notify =
        Some(handle_surface_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*surface).events.destroy,
                  &mut (*xsurface).surface_destroy);
}
unsafe extern "C" fn xsurface_unmap(mut surface: *mut wlr_xwayland_surface) {
    if (*surface).mapped {
        wlr_signal_emit_safe(&mut (*surface).events.unmap,
                             surface as *mut libc::c_void);
        (*surface).mapped = 0i32 != 0;
        xwm_set_net_client_list((*surface).xwm);
    }
    if (*surface).surface_id != 0 {
        // Make sure we're not on the unpaired surface list or we
		// could be assigned a surface during surface creation that
		// was mapped before this unmap request.
        wl_list_remove(&mut (*surface).unpaired_link);
        (*surface).surface_id = 0i32 as uint32_t
    }
    if !(*surface).surface.is_null() {
        wl_list_remove(&mut (*surface).surface_destroy.link);
        (*(*surface).surface).role_data = 0 as *mut libc::c_void;
        (*surface).surface = 0 as *mut wlr_surface
    };
}
unsafe extern "C" fn xwm_handle_create_notify(mut xwm: *mut wlr_xwm,
                                              mut ev:
                                                  *mut xcb_create_notify_event_t) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_CREATE_NOTIFY (%u)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             830i32, (*ev).window);
    if (*ev).window == (*xwm).window ||
           (*ev).window == (*xwm).selection_window ||
           (*ev).window == (*xwm).dnd_window {
        return
    }
    xwayland_surface_create(xwm, (*ev).window, (*ev).x, (*ev).y, (*ev).width,
                            (*ev).height, (*ev).override_redirect != 0);
}
unsafe extern "C" fn xwm_handle_destroy_notify(mut xwm: *mut wlr_xwm,
                                               mut ev:
                                                   *mut xcb_destroy_notify_event_t) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_DESTROY_NOTIFY (%u)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             844i32, (*ev).window);
    let mut xsurface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*ev).window);
    if xsurface.is_null() { return }
    xwayland_surface_destroy(xsurface);
}
unsafe extern "C" fn xwm_handle_configure_request(mut xwm: *mut wlr_xwm,
                                                  mut ev:
                                                      *mut xcb_configure_request_event_t) {
    let mut surface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*ev).window);
    if surface.is_null() { return }
    // TODO: handle ev->{parent,sibling}?
    let mut mask: uint16_t = (*ev).value_mask;
    let mut geo_mask: uint16_t =
        (XCB_CONFIG_WINDOW_X as libc::c_int |
             XCB_CONFIG_WINDOW_Y as libc::c_int |
             XCB_CONFIG_WINDOW_WIDTH as libc::c_int |
             XCB_CONFIG_WINDOW_HEIGHT as libc::c_int) as uint16_t;
    if mask as libc::c_int & geo_mask as libc::c_int == 0i32 { return }
    let mut wlr_event: wlr_xwayland_surface_configure_event =
        {
            let mut init =
                wlr_xwayland_surface_configure_event{surface: surface,
                                                     x:
                                                         if mask as
                                                                libc::c_int &
                                                                XCB_CONFIG_WINDOW_X
                                                                    as
                                                                    libc::c_int
                                                                != 0 {
                                                             (*ev).x as
                                                                 libc::c_int
                                                         } else {
                                                             (*surface).x as
                                                                 libc::c_int
                                                         } as int16_t,
                                                     y:
                                                         if mask as
                                                                libc::c_int &
                                                                XCB_CONFIG_WINDOW_Y
                                                                    as
                                                                    libc::c_int
                                                                != 0 {
                                                             (*ev).y as
                                                                 libc::c_int
                                                         } else {
                                                             (*surface).y as
                                                                 libc::c_int
                                                         } as int16_t,
                                                     width:
                                                         if mask as
                                                                libc::c_int &
                                                                XCB_CONFIG_WINDOW_WIDTH
                                                                    as
                                                                    libc::c_int
                                                                != 0 {
                                                             (*ev).width as
                                                                 libc::c_int
                                                         } else {
                                                             (*surface).width
                                                                 as
                                                                 libc::c_int
                                                         } as uint16_t,
                                                     height:
                                                         if mask as
                                                                libc::c_int &
                                                                XCB_CONFIG_WINDOW_HEIGHT
                                                                    as
                                                                    libc::c_int
                                                                != 0 {
                                                             (*ev).height as
                                                                 libc::c_int
                                                         } else {
                                                             (*surface).height
                                                                 as
                                                                 libc::c_int
                                                         } as uint16_t,
                                                     mask: mask,};
            init
        };
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_CONFIGURE_REQUEST (%u) [%ux%u+%d,%d]\x00" as
                 *const u8 as *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             877i32, (*ev).window, wlr_event.width as libc::c_int,
             wlr_event.height as libc::c_int, wlr_event.x as libc::c_int,
             wlr_event.y as libc::c_int);
    wlr_signal_emit_safe(&mut (*surface).events.request_configure,
                         &mut wlr_event as
                             *mut wlr_xwayland_surface_configure_event as
                             *mut libc::c_void);
}
unsafe extern "C" fn xwm_handle_configure_notify(mut xwm: *mut wlr_xwm,
                                                 mut ev:
                                                     *mut xcb_configure_notify_event_t) {
    let mut xsurface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*ev).window);
    if xsurface.is_null() { return }
    (*xsurface).x = (*ev).x;
    (*xsurface).y = (*ev).y;
    (*xsurface).width = (*ev).width;
    (*xsurface).height = (*ev).height;
    if (*xsurface).override_redirect as libc::c_int !=
           (*ev).override_redirect as libc::c_int {
        (*xsurface).override_redirect = (*ev).override_redirect != 0;
        wlr_signal_emit_safe(&mut (*xsurface).events.set_override_redirect,
                             xsurface as *mut libc::c_void);
    };
}
unsafe extern "C" fn xsurface_set_wm_state(mut xsurface:
                                               *mut wlr_xwayland_surface,
                                           mut state: int32_t) {
    let mut xwm: *mut wlr_xwm = (*xsurface).xwm;
    let mut property: [uint32_t; 2] = [0; 2];
    property[0] = state as uint32_t;
    property[1] = XCB_WINDOW_NONE as libc::c_int as uint32_t;
    xcb_change_property((*xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*xsurface).window_id,
                        (*xwm).atoms[WM_STATE as libc::c_int as usize],
                        (*xwm).atoms[WM_STATE as libc::c_int as usize],
                        32i32 as uint8_t, 2i32 as uint32_t,
                        property.as_mut_ptr() as *const libc::c_void);
}
unsafe extern "C" fn xwm_handle_map_request(mut xwm: *mut wlr_xwm,
                                            mut ev:
                                                *mut xcb_map_request_event_t) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_MAP_REQUEST (%u)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             923i32, (*ev).window);
    let mut xsurface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*ev).window);
    if xsurface.is_null() { return }
    xsurface_set_wm_state(xsurface, 1i32);
    xsurface_set_net_wm_state(xsurface);
    let mut values: [uint32_t; 1] = [0; 1];
    values[0] = XCB_STACK_MODE_BELOW as libc::c_int as uint32_t;
    xcb_configure_window((*xwm).xcb_conn, (*ev).window,
                         XCB_CONFIG_WINDOW_STACK_MODE as libc::c_int as
                             uint16_t,
                         values.as_mut_ptr() as *const libc::c_void);
    xcb_map_window((*xwm).xcb_conn, (*ev).window);
}
unsafe extern "C" fn xwm_handle_map_notify(mut xwm: *mut wlr_xwm,
                                           mut ev:
                                               *mut xcb_map_notify_event_t) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_MAP_NOTIFY (%u)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             942i32, (*ev).window);
    let mut xsurface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*ev).window);
    if xsurface.is_null() { return }
    if (*xsurface).override_redirect as libc::c_int !=
           (*ev).override_redirect as libc::c_int {
        (*xsurface).override_redirect = (*ev).override_redirect != 0;
        wlr_signal_emit_safe(&mut (*xsurface).events.set_override_redirect,
                             xsurface as *mut libc::c_void);
    };
}
unsafe extern "C" fn xwm_handle_unmap_notify(mut xwm: *mut wlr_xwm,
                                             mut ev:
                                                 *mut xcb_unmap_notify_event_t) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_UNMAP_NOTIFY (%u)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             956i32, (*ev).window);
    let mut xsurface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*ev).window);
    if xsurface.is_null() { return }
    xsurface_unmap(xsurface);
    xsurface_set_wm_state(xsurface, 0i32);
}
unsafe extern "C" fn xwm_handle_property_notify(mut xwm: *mut wlr_xwm,
                                                mut ev:
                                                    *mut xcb_property_notify_event_t) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_PROPERTY_NOTIFY (%u)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             968i32, (*ev).window);
    let mut xsurface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*ev).window);
    if xsurface.is_null() { return }
    read_surface_property(xwm, xsurface, (*ev).atom);
}
unsafe extern "C" fn xwm_handle_surface_id_message(mut xwm: *mut wlr_xwm,
                                                   mut ev:
                                                       *mut xcb_client_message_event_t) {
    let mut xsurface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*ev).window);
    if xsurface.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] client message WL_SURFACE_ID but no new window %u ?\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 983i32, (*ev).window);
        return
    }
    /* Check if we got notified after wayland surface create event */
    let mut id: uint32_t = (*ev).data.data32[0];
    let mut resource: *mut wl_resource =
        wl_client_get_object((*(*xwm).xwayland).client, id);
    if !resource.is_null() {
        let mut surface: *mut wlr_surface =
            wlr_surface_from_resource(resource);
        (*xsurface).surface_id = 0i32 as uint32_t;
        xwm_map_shell_surface(xwm, xsurface, surface);
    } else {
        (*xsurface).surface_id = id;
        wl_list_insert(&mut (*xwm).unpaired_surfaces,
                       &mut (*xsurface).unpaired_link);
    };
}
// cancel operation
unsafe extern "C" fn net_wm_edges_to_wlr(mut net_wm_edges: uint32_t)
 -> wlr_edges {
    let mut edges: wlr_edges = WLR_EDGE_NONE;
    match net_wm_edges {
        0 => {
            edges =
                (WLR_EDGE_TOP as libc::c_int | WLR_EDGE_LEFT as libc::c_int)
                    as wlr_edges
        }
        1 => { edges = WLR_EDGE_TOP }
        2 => {
            edges =
                (WLR_EDGE_TOP as libc::c_int | WLR_EDGE_RIGHT as libc::c_int)
                    as wlr_edges
        }
        3 => { edges = WLR_EDGE_RIGHT }
        4 => {
            edges =
                (WLR_EDGE_BOTTOM as libc::c_int |
                     WLR_EDGE_RIGHT as libc::c_int) as wlr_edges
        }
        5 => { edges = WLR_EDGE_BOTTOM }
        6 => {
            edges =
                (WLR_EDGE_BOTTOM as libc::c_int |
                     WLR_EDGE_LEFT as libc::c_int) as wlr_edges
        }
        7 => { edges = WLR_EDGE_LEFT }
        _ => { }
    }
    return edges;
}
unsafe extern "C" fn xwm_handle_net_wm_moveresize_message(mut xwm:
                                                              *mut wlr_xwm,
                                                          mut ev:
                                                              *mut xcb_client_message_event_t) {
    let mut xsurface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*ev).window);
    if xsurface.is_null() { return }
    // TODO: we should probably add input or seat info to this but we would just
	// be guessing
    let mut resize_event: wlr_xwayland_resize_event =
        wlr_xwayland_resize_event{surface: 0 as *mut wlr_xwayland_surface,
                                  edges: 0,};
    let mut move_event: wlr_xwayland_move_event =
        wlr_xwayland_move_event{surface: 0 as *mut wlr_xwayland_surface,};
    let mut detail: libc::c_int = (*ev).data.data32[2] as libc::c_int;
    match detail {
        8 => {
            move_event.surface = xsurface;
            wlr_signal_emit_safe(&mut (*xsurface).events.request_move,
                                 &mut move_event as
                                     *mut wlr_xwayland_move_event as
                                     *mut libc::c_void);
        }
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
            resize_event.surface = xsurface;
            resize_event.edges =
                net_wm_edges_to_wlr(detail as uint32_t) as uint32_t;
            wlr_signal_emit_safe(&mut (*xsurface).events.request_resize,
                                 &mut resize_event as
                                     *mut wlr_xwayland_resize_event as
                                     *mut libc::c_void);
        }
        11 | _ => { }
    };
}
unsafe extern "C" fn update_state(mut action: libc::c_int,
                                  mut state: *mut bool) -> bool {
    let mut new_state: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    match action {
        0 => { new_state = 0i32 }
        1 => { new_state = 1i32 }
        2 => { new_state = !*state as libc::c_int }
        _ => { return 0i32 != 0 }
    }
    changed = (*state as libc::c_int != new_state) as libc::c_int;
    *state = new_state != 0;
    return changed != 0;
}
#[inline]
unsafe extern "C" fn xsurface_is_maximized(mut xsurface:
                                               *mut wlr_xwayland_surface)
 -> bool {
    return (*xsurface).maximized_horz as libc::c_int != 0 &&
               (*xsurface).maximized_vert as libc::c_int != 0;
}
unsafe extern "C" fn xwm_handle_net_wm_state_message(mut xwm: *mut wlr_xwm,
                                                     mut client_message:
                                                         *mut xcb_client_message_event_t) {
    let mut xsurface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*client_message).window);
    if xsurface.is_null() { return }
    if (*client_message).format as libc::c_int != 32i32 { return }
    let mut fullscreen: bool = (*xsurface).fullscreen;
    let mut maximized: bool = xsurface_is_maximized(xsurface);
    let mut action: uint32_t = (*client_message).data.data32[0];
    let mut i: size_t = 0i32 as size_t;
    while i < 2i32 as libc::c_ulong {
        let mut property: uint32_t =
            (*client_message).data.data32[(1i32 as
                                               libc::c_ulong).wrapping_add(i)
                                              as usize];
        if property ==
               (*xwm).atoms[_NET_WM_STATE_MODAL as libc::c_int as usize] &&
               update_state(action as libc::c_int, &mut (*xsurface).modal) as
                   libc::c_int != 0 {
            xsurface_set_net_wm_state(xsurface);
        } else if property ==
                      (*xwm).atoms[_NET_WM_STATE_FULLSCREEN as libc::c_int as
                                       usize] &&
                      update_state(action as libc::c_int,
                                   &mut (*xsurface).fullscreen) as libc::c_int
                          != 0 {
            xsurface_set_net_wm_state(xsurface);
        } else if property ==
                      (*xwm).atoms[_NET_WM_STATE_MAXIMIZED_VERT as libc::c_int
                                       as usize] &&
                      update_state(action as libc::c_int,
                                   &mut (*xsurface).maximized_vert) as
                          libc::c_int != 0 {
            xsurface_set_net_wm_state(xsurface);
        } else if property ==
                      (*xwm).atoms[_NET_WM_STATE_MAXIMIZED_HORZ as libc::c_int
                                       as usize] &&
                      update_state(action as libc::c_int,
                                   &mut (*xsurface).maximized_horz) as
                          libc::c_int != 0 {
            xsurface_set_net_wm_state(xsurface);
        }
        i = i.wrapping_add(1)
    }
    // client_message->data.data32[3] is the source indication
	// all other values are set to 0
    if fullscreen as libc::c_int != (*xsurface).fullscreen as libc::c_int {
        if (*xsurface).fullscreen {
            (*xsurface).saved_width = (*xsurface).width;
            (*xsurface).saved_height = (*xsurface).height
        }
        wlr_signal_emit_safe(&mut (*xsurface).events.request_fullscreen,
                             xsurface as *mut libc::c_void);
    }
    if maximized as libc::c_int !=
           xsurface_is_maximized(xsurface) as libc::c_int {
        if xsurface_is_maximized(xsurface) {
            (*xsurface).saved_width = (*xsurface).width;
            (*xsurface).saved_height = (*xsurface).height
        }
        wlr_signal_emit_safe(&mut (*xsurface).events.request_maximize,
                             xsurface as *mut libc::c_void);
    };
}
unsafe extern "C" fn xwm_handle_wm_protocols_message(mut xwm: *mut wlr_xwm,
                                                     mut ev:
                                                         *mut xcb_client_message_event_t) {
    let mut type_0: xcb_atom_t = (*ev).data.data32[0];
    if type_0 == (*xwm).atoms[_NET_WM_PING as libc::c_int as usize] {
        let mut window_id: xcb_window_t = (*ev).data.data32[2];
        let mut surface: *mut wlr_xwayland_surface =
            lookup_surface(xwm, window_id);
        if surface.is_null() { return }
        if !(*surface).pinging { return }
        wl_event_source_timer_update((*surface).ping_timer, 0i32);
        (*surface).pinging = 0i32 != 0
    } else {
        let mut type_name: *mut libc::c_char = xwm_get_atom_name(xwm, type_0);
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] unhandled WM_PROTOCOLS client message %u (%s)\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1191i32, type_0, type_name);
        free(type_name as *mut libc::c_void);
    };
}
unsafe extern "C" fn xwm_handle_net_active_window_message(mut xwm:
                                                              *mut wlr_xwm,
                                                          mut ev:
                                                              *mut xcb_client_message_event_t) {
    let mut surface: *mut wlr_xwayland_surface =
        lookup_surface(xwm, (*ev).window);
    if surface.is_null() { return }
    wlr_signal_emit_safe(&mut (*surface).events.request_activate,
                         surface as *mut libc::c_void);
}
unsafe extern "C" fn xwm_handle_client_message(mut xwm: *mut wlr_xwm,
                                               mut ev:
                                                   *mut xcb_client_message_event_t) {
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] XCB_CLIENT_MESSAGE (%u)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             1207i32, (*ev).window);
    if (*ev).type_0 == (*xwm).atoms[WL_SURFACE_ID as libc::c_int as usize] {
        xwm_handle_surface_id_message(xwm, ev);
    } else if (*ev).type_0 ==
                  (*xwm).atoms[NET_WM_STATE as libc::c_int as usize] {
        xwm_handle_net_wm_state_message(xwm, ev);
    } else if (*ev).type_0 ==
                  (*xwm).atoms[_NET_WM_MOVERESIZE as libc::c_int as usize] {
        xwm_handle_net_wm_moveresize_message(xwm, ev);
    } else if (*ev).type_0 ==
                  (*xwm).atoms[WM_PROTOCOLS as libc::c_int as usize] {
        xwm_handle_wm_protocols_message(xwm, ev);
    } else if (*ev).type_0 ==
                  (*xwm).atoms[_NET_ACTIVE_WINDOW as libc::c_int as usize] {
        xwm_handle_net_active_window_message(xwm, ev);
    } else if xwm_handle_selection_client_message(xwm, ev) == 0 {
        let mut type_name: *mut libc::c_char =
            xwm_get_atom_name(xwm, (*ev).type_0);
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] unhandled x11 client message %u (%s)\x00" as
                     *const u8 as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1222i32, (*ev).type_0, type_name);
        free(type_name as *mut libc::c_void);
    };
}
unsafe extern "C" fn xwm_handle_focus_in(mut xwm: *mut wlr_xwm,
                                         mut ev: *mut xcb_focus_in_event_t) {
    // Do not interfere with grabs
    if (*ev).mode as libc::c_int == XCB_NOTIFY_MODE_GRAB as libc::c_int ||
           (*ev).mode as libc::c_int == XCB_NOTIFY_MODE_UNGRAB as libc::c_int
       {
        return
    }
    // Do not let X clients change the focus behind the compositor's
	// back. Reset the focus to the old one if it changed.
    if (*xwm).focus_surface.is_null() ||
           (*ev).event != (*(*xwm).focus_surface).window_id {
        xwm_send_focus_window(xwm, (*xwm).focus_surface);
    };
}
unsafe extern "C" fn xwm_handle_xcb_error(mut xwm: *mut wlr_xwm,
                                          mut ev: *mut xcb_value_error_t) {
    let mut minor_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut extension: *const libc::c_char = 0 as *const libc::c_char;
    let mut error_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut major_name: *const libc::c_char =
        xcb_errors_get_name_for_major_code((*xwm).errors_context,
                                           (*ev).major_opcode);
    if major_name.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] xcb error happened, but could not get major name\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1248i32);
    } else {
        minor_name =
            xcb_errors_get_name_for_minor_code((*xwm).errors_context,
                                               (*ev).major_opcode,
                                               (*ev).minor_opcode);
        extension = 0 as *const libc::c_char;
        error_name =
            xcb_errors_get_name_for_error((*xwm).errors_context,
                                          (*ev).error_code, &mut extension);
        if error_name.is_null() {
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] xcb error happened, but could not get error name\x00"
                         as *const u8 as *const libc::c_char,
                     b"../xwayland/xwm.c\x00" as *const u8 as
                         *const libc::c_char, 1261i32);
        } else {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] xcb error: op %s (%s), code %s (%s), sequence %u, value %u\x00"
                         as *const u8 as *const libc::c_char,
                     b"../xwayland/xwm.c\x00" as *const u8 as
                         *const libc::c_char, 1268i32, major_name,
                     if !minor_name.is_null() {
                         minor_name
                     } else {
                         b"no minor\x00" as *const u8 as *const libc::c_char
                     }, error_name,
                     if !extension.is_null() {
                         extension
                     } else {
                         b"no extension\x00" as *const u8 as
                             *const libc::c_char
                     }, (*ev).sequence as libc::c_int, (*ev).bad_value);
            return
        }
    }
    _wlr_log(WLR_ERROR,
             b"[%s:%d] xcb error: op %u:%u, code %u, sequence %u, value %u\x00"
                 as *const u8 as *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             1276i32, (*ev).major_opcode as libc::c_int,
             (*ev).minor_opcode as libc::c_int,
             (*ev).error_code as libc::c_int, (*ev).sequence as libc::c_int,
             (*ev).bad_value);
}
unsafe extern "C" fn xwm_handle_unhandled_event(mut xwm: *mut wlr_xwm,
                                                mut ev:
                                                    *mut xcb_generic_event_t) {
    let mut extension: *const libc::c_char = 0 as *const libc::c_char;
    let mut event_name: *const libc::c_char =
        xcb_errors_get_name_for_xcb_event((*xwm).errors_context, ev,
                                          &mut extension);
    if event_name.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] no name for unhandled event: %u\x00" as *const u8
                     as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1288i32, (*ev).response_type as libc::c_int);
        return
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] unhandled X11 event: %s (%u)\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             1292i32, event_name, (*ev).response_type as libc::c_int);
}
unsafe extern "C" fn x11_event_handler(mut fd: libc::c_int,
                                       mut mask: uint32_t,
                                       mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    let mut event: *mut xcb_generic_event_t = 0 as *mut xcb_generic_event_t;
    let mut xwm: *mut wlr_xwm = data as *mut wlr_xwm;
    loop  {
        event = xcb_poll_for_event((*xwm).xcb_conn);
        if event.is_null() { break ; }
        count += 1;
        if (*(*xwm).xwayland).user_event_handler.is_some() &&
               (*(*xwm).xwayland).user_event_handler.expect("non-null function pointer")(xwm,
                                                                                         event)
                   != 0 {
            break ;
        }
        if xwm_handle_selection_event(xwm, event) != 0 {
            free(event as *mut libc::c_void);
        } else {
            match (*event).response_type as libc::c_int & 0x7fi32 {
                16 => {
                    xwm_handle_create_notify(xwm,
                                             event as
                                                 *mut xcb_create_notify_event_t);
                }
                17 => {
                    xwm_handle_destroy_notify(xwm,
                                              event as
                                                  *mut xcb_destroy_notify_event_t);
                }
                23 => {
                    xwm_handle_configure_request(xwm,
                                                 event as
                                                     *mut xcb_configure_request_event_t);
                }
                22 => {
                    xwm_handle_configure_notify(xwm,
                                                event as
                                                    *mut xcb_configure_notify_event_t);
                }
                20 => {
                    xwm_handle_map_request(xwm,
                                           event as
                                               *mut xcb_map_request_event_t);
                }
                19 => {
                    xwm_handle_map_notify(xwm,
                                          event as
                                              *mut xcb_map_notify_event_t);
                }
                18 => {
                    xwm_handle_unmap_notify(xwm,
                                            event as
                                                *mut xcb_unmap_notify_event_t);
                }
                28 => {
                    xwm_handle_property_notify(xwm,
                                               event as
                                                   *mut xcb_property_notify_event_t);
                }
                33 => {
                    xwm_handle_client_message(xwm,
                                              event as
                                                  *mut xcb_client_message_event_t);
                }
                9 => {
                    xwm_handle_focus_in(xwm,
                                        event as *mut xcb_focus_in_event_t);
                }
                0 => {
                    xwm_handle_xcb_error(xwm,
                                         event as *mut xcb_value_error_t);
                }
                _ => { xwm_handle_unhandled_event(xwm, event); }
            }
            free(event as *mut libc::c_void);
        }
    }
    if count != 0 { xcb_flush((*xwm).xcb_conn); }
    return count;
}
unsafe extern "C" fn handle_compositor_new_surface(mut listener:
                                                       *mut wl_listener,
                                                   mut data:
                                                       *mut libc::c_void) {
    let mut xwm: *mut wlr_xwm =
        (listener as *mut libc::c_char).offset(-880) as *mut wlr_xwm;
    let mut surface: *mut wlr_surface = data as *mut wlr_surface;
    if wl_resource_get_client((*surface).resource) !=
           (*(*xwm).xwayland).client {
        return
    }
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] New xwayland surface: %p\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             1376i32, surface);
    let mut surface_id: uint32_t = wl_resource_get_id((*surface).resource);
    let mut xsurface: *mut wlr_xwayland_surface =
        0 as *mut wlr_xwayland_surface;
    xsurface =
        ((*xwm).unpaired_surfaces.next as *mut libc::c_char).offset(-40) as
            *mut wlr_xwayland_surface;
    while &mut (*xsurface).unpaired_link as *mut wl_list !=
              &mut (*xwm).unpaired_surfaces as *mut wl_list {
        if (*xsurface).surface_id == surface_id {
            xwm_map_shell_surface(xwm, xsurface, surface);
            (*xsurface).surface_id = 0i32 as uint32_t;
            wl_list_remove(&mut (*xsurface).unpaired_link);
            xcb_flush((*xwm).xcb_conn);
            return
        }
        xsurface =
            ((*xsurface).unpaired_link.next as *mut libc::c_char).offset(-40)
                as *mut wlr_xwayland_surface
    };
}
unsafe extern "C" fn handle_compositor_destroy(mut listener: *mut wl_listener,
                                               mut data: *mut libc::c_void) {
    let mut xwm: *mut wlr_xwm =
        (listener as *mut libc::c_char).offset(-904) as *mut wlr_xwm;
    wl_list_remove(&mut (*xwm).compositor_new_surface.link);
    wl_list_remove(&mut (*xwm).compositor_destroy.link);
    wl_list_init(&mut (*xwm).compositor_new_surface.link);
    wl_list_init(&mut (*xwm).compositor_destroy.link);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_surface_activate(mut xsurface:
                                                           *mut wlr_xwayland_surface,
                                                       mut activated: bool) {
    let mut focused: *mut wlr_xwayland_surface =
        (*(*xsurface).xwm).focus_surface;
    if activated {
        xwm_surface_activate((*xsurface).xwm, xsurface);
    } else if focused == xsurface {
        xwm_surface_activate((*xsurface).xwm, 0 as *mut wlr_xwayland_surface);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_surface_configure(mut xsurface:
                                                            *mut wlr_xwayland_surface,
                                                        mut x: int16_t,
                                                        mut y: int16_t,
                                                        mut width: uint16_t,
                                                        mut height:
                                                            uint16_t) {
    (*xsurface).x = x;
    (*xsurface).y = y;
    (*xsurface).width = width;
    (*xsurface).height = height;
    let mut xwm: *mut wlr_xwm = (*xsurface).xwm;
    let mut mask: uint32_t =
        (XCB_CONFIG_WINDOW_X as libc::c_int |
             XCB_CONFIG_WINDOW_Y as libc::c_int |
             XCB_CONFIG_WINDOW_WIDTH as libc::c_int |
             XCB_CONFIG_WINDOW_HEIGHT as libc::c_int |
             XCB_CONFIG_WINDOW_BORDER_WIDTH as libc::c_int) as uint32_t;
    let mut values: [uint32_t; 5] =
        [x as uint32_t, y as uint32_t, width as uint32_t, height as uint32_t,
         0i32 as uint32_t];
    xcb_configure_window((*xwm).xcb_conn, (*xsurface).window_id,
                         mask as uint16_t,
                         values.as_mut_ptr() as *const libc::c_void);
    xcb_flush((*xwm).xcb_conn);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_surface_close(mut xsurface:
                                                        *mut wlr_xwayland_surface) {
    let mut xwm: *mut wlr_xwm = (*xsurface).xwm;
    let mut supports_delete: bool = 0i32 != 0;
    let mut i: size_t = 0i32 as size_t;
    while i < (*xsurface).protocols_len {
        if *(*xsurface).protocols.offset(i as isize) ==
               (*xwm).atoms[WM_DELETE_WINDOW as libc::c_int as usize] {
            supports_delete = 1i32 != 0;
            break ;
        } else { i = i.wrapping_add(1) }
    }
    if supports_delete {
        let mut message_data: xcb_client_message_data_t =
            xcb_client_message_data_t{data8:
                                          [0i32 as uint8_t, 0, 0, 0, 0, 0, 0,
                                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                           0],};
        message_data.data32[0] =
            (*xwm).atoms[WM_DELETE_WINDOW as libc::c_int as usize];
        message_data.data32[1] = 0i64 as uint32_t;
        xwm_send_wm_message(xsurface, &mut message_data,
                            XCB_EVENT_MASK_NO_EVENT as libc::c_int as
                                uint32_t);
    } else {
        xcb_kill_client((*xwm).xcb_conn, (*xsurface).window_id);
        xcb_flush((*xwm).xcb_conn);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xwm_destroy(mut xwm: *mut wlr_xwm) {
    if xwm.is_null() { return }
    xwm_selection_finish(xwm);
    if (*xwm).cursor != 0 { xcb_free_cursor((*xwm).xcb_conn, (*xwm).cursor); }
    if (*xwm).colormap != 0 {
        xcb_free_colormap((*xwm).xcb_conn, (*xwm).colormap);
    }
    if (*xwm).window != 0 {
        xcb_destroy_window((*xwm).xcb_conn, (*xwm).window);
    }
    if !(*xwm).event_source.is_null() {
        wl_event_source_remove((*xwm).event_source);
    }
    if !(*xwm).errors_context.is_null() {
        xcb_errors_context_free((*xwm).errors_context);
    }
    let mut xsurface: *mut wlr_xwayland_surface =
        0 as *mut wlr_xwayland_surface;
    let mut tmp: *mut wlr_xwayland_surface = 0 as *mut wlr_xwayland_surface;
    xsurface =
        ((*xwm).surfaces.next as *mut libc::c_char).offset(-24) as
            *mut wlr_xwayland_surface;
    tmp =
        ((*xsurface).link.next as *mut libc::c_char).offset(-24) as
            *mut wlr_xwayland_surface;
    while &mut (*xsurface).link as *mut wl_list !=
              &mut (*xwm).surfaces as *mut wl_list {
        xwayland_surface_destroy(xsurface);
        xsurface = tmp;
        tmp =
            ((*xsurface).link.next as *mut libc::c_char).offset(-24) as
                *mut wlr_xwayland_surface
    }
    xsurface =
        ((*xwm).unpaired_surfaces.next as *mut libc::c_char).offset(-24) as
            *mut wlr_xwayland_surface;
    tmp =
        ((*xsurface).link.next as *mut libc::c_char).offset(-24) as
            *mut wlr_xwayland_surface;
    while &mut (*xsurface).link as *mut wl_list !=
              &mut (*xwm).unpaired_surfaces as *mut wl_list {
        xwayland_surface_destroy(xsurface);
        xsurface = tmp;
        tmp =
            ((*xsurface).link.next as *mut libc::c_char).offset(-24) as
                *mut wlr_xwayland_surface
    }
    wl_list_remove(&mut (*xwm).compositor_new_surface.link);
    wl_list_remove(&mut (*xwm).compositor_destroy.link);
    xcb_disconnect((*xwm).xcb_conn);
    free(xwm as *mut libc::c_void);
}
unsafe extern "C" fn xwm_get_resources(mut xwm: *mut wlr_xwm) {
    xcb_prefetch_extension_data((*xwm).xcb_conn, &mut xcb_xfixes_id);
    xcb_prefetch_extension_data((*xwm).xcb_conn, &mut xcb_composite_id);
    let mut i: size_t = 0;
    let mut cookies: [xcb_intern_atom_cookie_t; 62] =
        [xcb_intern_atom_cookie_t{sequence: 0,}; 62];
    i = 0i32 as size_t;
    while i < ATOM_LAST as libc::c_int as libc::c_ulong {
        cookies[i as usize] =
            xcb_intern_atom((*xwm).xcb_conn, 0i32 as uint8_t,
                            strlen(atom_map[i as usize]) as uint16_t,
                            atom_map[i as usize]);
        i = i.wrapping_add(1)
    }
    i = 0i32 as size_t;
    while i < ATOM_LAST as libc::c_int as libc::c_ulong {
        let mut error: *mut xcb_generic_error_t =
            0 as *mut xcb_generic_error_t;
        let mut reply: *mut xcb_intern_atom_reply_t =
            xcb_intern_atom_reply((*xwm).xcb_conn, cookies[i as usize],
                                  &mut error);
        if !reply.is_null() && error.is_null() {
            (*xwm).atoms[i as usize] = (*reply).atom
        }
        free(reply as *mut libc::c_void);
        if !error.is_null() {
            _wlr_log(WLR_ERROR,
                     b"[%s:%d] could not resolve atom %s, x11 error code %d\x00"
                         as *const u8 as *const libc::c_char,
                     b"../xwayland/xwm.c\x00" as *const u8 as
                         *const libc::c_char, 1507i32, atom_map[i as usize],
                     (*error).error_code as libc::c_int);
            free(error as *mut libc::c_void);
            return
        }
        i = i.wrapping_add(1)
    }
    (*xwm).xfixes =
        xcb_get_extension_data((*xwm).xcb_conn, &mut xcb_xfixes_id);
    if (*xwm).xfixes.is_null() || (*(*xwm).xfixes).present == 0 {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] xfixes not available\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1516i32);
    }
    let mut xfixes_cookie: xcb_xfixes_query_version_cookie_t =
        xcb_xfixes_query_version_cookie_t{sequence: 0,};
    let mut xfixes_reply: *mut xcb_xfixes_query_version_reply_t =
        0 as *mut xcb_xfixes_query_version_reply_t;
    xfixes_cookie =
        xcb_xfixes_query_version((*xwm).xcb_conn, 5i32 as uint32_t,
                                 0i32 as uint32_t);
    xfixes_reply =
        xcb_xfixes_query_version_reply((*xwm).xcb_conn, xfixes_cookie,
                                       0 as *mut *mut xcb_generic_error_t);
    _wlr_log(WLR_DEBUG,
             b"[%s:%d] xfixes version: %d.%d\x00" as *const u8 as
                 *const libc::c_char,
             b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
             1528i32, (*xfixes_reply).major_version,
             (*xfixes_reply).minor_version);
    free(xfixes_reply as *mut libc::c_void);
}
unsafe extern "C" fn xwm_create_wm_window(mut xwm: *mut wlr_xwm) {
    static mut name: [libc::c_char; 11] =
        [119, 108, 114, 111, 111, 116, 115, 32, 119, 109, 0];
    (*xwm).window = xcb_generate_id((*xwm).xcb_conn);
    xcb_create_window((*xwm).xcb_conn, 0i64 as uint8_t, (*xwm).window,
                      (*(*xwm).screen).root, 0i32 as int16_t, 0i32 as int16_t,
                      10i32 as uint16_t, 10i32 as uint16_t, 0i32 as uint16_t,
                      XCB_WINDOW_CLASS_INPUT_OUTPUT as libc::c_int as
                          uint16_t, (*(*xwm).screen).root_visual,
                      0i32 as uint32_t, 0 as *const libc::c_void);
    xcb_change_property((*xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*xwm).window,
                        (*xwm).atoms[_NET_WM_NAME as libc::c_int as usize],
                        (*xwm).atoms[UTF8_STRING as libc::c_int as usize],
                        8i32 as uint8_t, strlen(name.as_ptr()) as uint32_t,
                        name.as_ptr() as *const libc::c_void);
    xcb_change_property((*xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*(*xwm).screen).root,
                        (*xwm).atoms[_NET_SUPPORTING_WM_CHECK as libc::c_int
                                         as usize],
                        XCB_ATOM_WINDOW as libc::c_int as xcb_atom_t,
                        32i32 as uint8_t, 1i32 as uint32_t,
                        &mut (*xwm).window as *mut xcb_window_t as
                            *const libc::c_void);
    xcb_change_property((*xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*xwm).window,
                        (*xwm).atoms[_NET_SUPPORTING_WM_CHECK as libc::c_int
                                         as usize],
                        XCB_ATOM_WINDOW as libc::c_int as xcb_atom_t,
                        32i32 as uint8_t, 1i32 as uint32_t,
                        &mut (*xwm).window as *mut xcb_window_t as
                            *const libc::c_void);
    xcb_set_selection_owner((*xwm).xcb_conn, (*xwm).window,
                            (*xwm).atoms[WM_S0 as libc::c_int as usize],
                            0i64 as xcb_timestamp_t);
    xcb_set_selection_owner((*xwm).xcb_conn, (*xwm).window,
                            (*xwm).atoms[NET_WM_CM_S0 as libc::c_int as
                                             usize], 0i64 as xcb_timestamp_t);
}
// TODO use me to support 32 bit color somehow
unsafe extern "C" fn xwm_get_visual_and_colormap(mut xwm: *mut wlr_xwm) {
    let mut d_iter: xcb_depth_iterator_t =
        xcb_depth_iterator_t{data: 0 as *mut xcb_depth_t, rem: 0, index: 0,};
    let mut vt_iter: xcb_visualtype_iterator_t =
        xcb_visualtype_iterator_t{data: 0 as *mut xcb_visualtype_t,
                                  rem: 0,
                                  index: 0,};
    let mut visualtype: *mut xcb_visualtype_t = 0 as *mut xcb_visualtype_t;
    d_iter = xcb_screen_allowed_depths_iterator((*xwm).screen);
    visualtype = 0 as *mut xcb_visualtype_t;
    while d_iter.rem > 0i32 {
        if (*d_iter.data).depth as libc::c_int == 32i32 {
            vt_iter = xcb_depth_visuals_iterator(d_iter.data);
            visualtype = vt_iter.data;
            break ;
        } else { xcb_depth_next(&mut d_iter); }
    }
    if visualtype.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] No 32 bit visualtype\n\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1603i32);
        return
    }
    (*xwm).visual_id = (*visualtype).visual_id;
    (*xwm).colormap = xcb_generate_id((*xwm).xcb_conn);
    xcb_create_colormap((*xwm).xcb_conn,
                        XCB_COLORMAP_ALLOC_NONE as libc::c_int as uint8_t,
                        (*xwm).colormap, (*(*xwm).screen).root,
                        (*xwm).visual_id);
}
unsafe extern "C" fn xwm_get_render_format(mut xwm: *mut wlr_xwm) {
    let mut cookie: xcb_render_query_pict_formats_cookie_t =
        xcb_render_query_pict_formats((*xwm).xcb_conn);
    let mut reply: *mut xcb_render_query_pict_formats_reply_t =
        xcb_render_query_pict_formats_reply((*xwm).xcb_conn, cookie,
                                            0 as
                                                *mut *mut xcb_generic_error_t);
    if reply.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Did not get any reply from xcb_render_query_pict_formats\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1622i32);
        return
    }
    let mut iter: xcb_render_pictforminfo_iterator_t =
        xcb_render_query_pict_formats_formats_iterator(reply);
    let mut format: *mut xcb_render_pictforminfo_t =
        0 as *mut xcb_render_pictforminfo_t;
    while iter.rem > 0i32 {
        if (*iter.data).depth as libc::c_int == 32i32 {
            format = iter.data;
            break ;
        } else { xcb_render_pictforminfo_next(&mut iter); }
    }
    if format.is_null() {
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] No 32 bit render format\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1638i32);
        free(reply as *mut libc::c_void);
        return
    }
    (*xwm).render_format_id = (*format).id;
    free(reply as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xwm_set_cursor(mut xwm: *mut wlr_xwm,
                                        mut pixels: *const uint8_t,
                                        mut stride: uint32_t,
                                        mut width: uint32_t,
                                        mut height: uint32_t,
                                        mut hotspot_x: int32_t,
                                        mut hotspot_y: int32_t) {
    if (*xwm).render_format_id == 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Cannot set xwm cursor: no render format available\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1650i32);
        return
    }
    if (*xwm).cursor != 0 { xcb_free_cursor((*xwm).xcb_conn, (*xwm).cursor); }
    let mut depth: libc::c_int = 32i32;
    let mut pix: xcb_pixmap_t = xcb_generate_id((*xwm).xcb_conn);
    xcb_create_pixmap((*xwm).xcb_conn, depth as uint8_t, pix,
                      (*(*xwm).screen).root, width as uint16_t,
                      height as uint16_t);
    let mut pic: xcb_render_picture_t = xcb_generate_id((*xwm).xcb_conn);
    xcb_render_create_picture((*xwm).xcb_conn, pic, pix,
                              (*xwm).render_format_id, 0i32 as uint32_t,
                              0 as *const libc::c_void);
    let mut gc: xcb_gcontext_t = xcb_generate_id((*xwm).xcb_conn);
    xcb_create_gc((*xwm).xcb_conn, gc, pix, 0i32 as uint32_t,
                  0 as *const libc::c_void);
    xcb_put_image((*xwm).xcb_conn,
                  XCB_IMAGE_FORMAT_Z_PIXMAP as libc::c_int as uint8_t, pix,
                  gc, width as uint16_t, height as uint16_t, 0i32 as int16_t,
                  0i32 as int16_t, 0i32 as uint8_t, depth as uint8_t,
                  (stride.wrapping_mul(height) as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint8_t>()
                                                       as libc::c_ulong) as
                      uint32_t, pixels);
    xcb_free_gc((*xwm).xcb_conn, gc);
    (*xwm).cursor = xcb_generate_id((*xwm).xcb_conn);
    xcb_render_create_cursor((*xwm).xcb_conn, (*xwm).cursor, pic,
                             hotspot_x as uint16_t, hotspot_y as uint16_t);
    xcb_free_pixmap((*xwm).xcb_conn, pix);
    let mut values: [uint32_t; 1] = [(*xwm).cursor];
    xcb_change_window_attributes((*xwm).xcb_conn, (*(*xwm).screen).root,
                                 XCB_CW_CURSOR as libc::c_int as uint32_t,
                                 values.as_mut_ptr() as *const libc::c_void);
    xcb_flush((*xwm).xcb_conn);
}
#[no_mangle]
pub unsafe extern "C" fn xwm_create(mut wlr_xwayland: *mut wlr_xwayland)
 -> *mut wlr_xwm {
    let mut xwm: *mut wlr_xwm =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xwm>() as libc::c_ulong) as
            *mut wlr_xwm;
    if xwm.is_null() { return 0 as *mut wlr_xwm }
    (*xwm).xwayland = wlr_xwayland;
    wl_list_init(&mut (*xwm).surfaces);
    wl_list_init(&mut (*xwm).unpaired_surfaces);
    (*xwm).ping_timeout = 10000i32 as uint32_t;
    (*xwm).xcb_conn =
        xcb_connect_to_fd((*wlr_xwayland).wm_fd[0],
                          0 as *mut xcb_auth_info_t);
    let mut rc: libc::c_int = xcb_connection_has_error((*xwm).xcb_conn);
    if rc != 0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] xcb connect failed: %d\x00" as *const u8 as
                     *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1701i32, rc);
        close((*wlr_xwayland).wm_fd[0]);
        free(xwm as *mut libc::c_void);
        return 0 as *mut wlr_xwm
    }
    if xcb_errors_context_new((*xwm).xcb_conn, &mut (*xwm).errors_context) !=
           0 {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Could not allocate error context\x00" as *const u8
                     as *const libc::c_char,
                 b"../xwayland/xwm.c\x00" as *const u8 as *const libc::c_char,
                 1709i32);
        xwm_destroy(xwm);
        return 0 as *mut wlr_xwm
    }
    let mut screen_iterator: xcb_screen_iterator_t =
        xcb_setup_roots_iterator(xcb_get_setup((*xwm).xcb_conn));
    (*xwm).screen = screen_iterator.data;
    let mut event_loop: *mut wl_event_loop =
        wl_display_get_event_loop((*wlr_xwayland).wl_display);
    (*xwm).event_source =
        wl_event_loop_add_fd(event_loop, (*wlr_xwayland).wm_fd[0],
                             WL_EVENT_READABLE as libc::c_int as uint32_t,
                             Some(x11_event_handler as
                                      unsafe extern "C" fn(_: libc::c_int,
                                                           _: uint32_t,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),
                             xwm as *mut libc::c_void);
    wl_event_source_check((*xwm).event_source);
    xwm_get_resources(xwm);
    xwm_get_visual_and_colormap(xwm);
    xwm_get_render_format(xwm);
    let mut values: [uint32_t; 1] =
        [(XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as libc::c_int |
              XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as libc::c_int |
              XCB_EVENT_MASK_PROPERTY_CHANGE as libc::c_int) as uint32_t];
    xcb_change_window_attributes((*xwm).xcb_conn, (*(*xwm).screen).root,
                                 XCB_CW_EVENT_MASK as libc::c_int as uint32_t,
                                 values.as_mut_ptr() as *const libc::c_void);
    xcb_composite_redirect_subwindows((*xwm).xcb_conn, (*(*xwm).screen).root,
                                      XCB_COMPOSITE_REDIRECT_MANUAL as
                                          libc::c_int as uint8_t);
    let mut supported: [xcb_atom_t; 8] =
        [(*xwm).atoms[NET_WM_STATE as libc::c_int as usize],
         (*xwm).atoms[_NET_ACTIVE_WINDOW as libc::c_int as usize],
         (*xwm).atoms[_NET_WM_MOVERESIZE as libc::c_int as usize],
         (*xwm).atoms[_NET_WM_STATE_MODAL as libc::c_int as usize],
         (*xwm).atoms[_NET_WM_STATE_FULLSCREEN as libc::c_int as usize],
         (*xwm).atoms[_NET_WM_STATE_MAXIMIZED_VERT as libc::c_int as usize],
         (*xwm).atoms[_NET_WM_STATE_MAXIMIZED_HORZ as libc::c_int as usize],
         (*xwm).atoms[_NET_CLIENT_LIST as libc::c_int as usize]];
    xcb_change_property((*xwm).xcb_conn,
                        XCB_PROP_MODE_REPLACE as libc::c_int as uint8_t,
                        (*(*xwm).screen).root,
                        (*xwm).atoms[NET_SUPPORTED as libc::c_int as usize],
                        XCB_ATOM_ATOM as libc::c_int as xcb_atom_t,
                        32i32 as uint8_t,
                        (::std::mem::size_of::<[xcb_atom_t; 8]>() as
                             libc::c_ulong).wrapping_div(::std::mem::size_of::<xcb_atom_t>()
                                                             as libc::c_ulong)
                            as uint32_t,
                        supported.as_mut_ptr() as *const libc::c_void);
    xcb_flush((*xwm).xcb_conn);
    xwm_set_net_active_window(xwm,
                              XCB_WINDOW_NONE as libc::c_int as xcb_window_t);
    xwm_selection_init(xwm);
    (*xwm).compositor_new_surface.notify =
        Some(handle_compositor_new_surface as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*(*wlr_xwayland).compositor).events.new_surface,
                  &mut (*xwm).compositor_new_surface);
    (*xwm).compositor_destroy.notify =
        Some(handle_compositor_destroy as
                 unsafe extern "C" fn(_: *mut wl_listener,
                                      _: *mut libc::c_void) -> ());
    wl_signal_add(&mut (*(*wlr_xwayland).compositor).events.destroy,
                  &mut (*xwm).compositor_destroy);
    xwm_create_wm_window(xwm);
    xcb_flush((*xwm).xcb_conn);
    return xwm;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_surface_set_maximized(mut surface:
                                                                *mut wlr_xwayland_surface,
                                                            mut maximized:
                                                                bool) {
    (*surface).maximized_horz = maximized;
    (*surface).maximized_vert = maximized;
    xsurface_set_net_wm_state(surface);
    xcb_flush((*(*surface).xwm).xcb_conn);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_surface_set_fullscreen(mut surface:
                                                                 *mut wlr_xwayland_surface,
                                                             mut fullscreen:
                                                                 bool) {
    (*surface).fullscreen = fullscreen;
    xsurface_set_net_wm_state(surface);
    xcb_flush((*(*surface).xwm).xcb_conn);
}
#[no_mangle]
pub unsafe extern "C" fn xwm_atoms_contains(mut xwm: *mut wlr_xwm,
                                            mut atoms: *mut xcb_atom_t,
                                            mut num_atoms: size_t,
                                            mut needle: atom_name) -> bool {
    let mut atom: xcb_atom_t = (*xwm).atoms[needle as usize];
    let mut i: size_t = 0i32 as size_t;
    while i < num_atoms {
        if atom == *atoms.offset(i as isize) { return 1i32 != 0 }
        i = i.wrapping_add(1)
    }
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_surface_ping(mut surface:
                                                       *mut wlr_xwayland_surface) {
    let mut data: xcb_client_message_data_t =
        xcb_client_message_data_t{data8:
                                      [0i32 as uint8_t, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
    data.data32[0] =
        (*(*surface).xwm).atoms[_NET_WM_PING as libc::c_int as usize];
    data.data32[1] = 0i64 as uint32_t;
    data.data32[2] = (*surface).window_id;
    xwm_send_wm_message(surface, &mut data,
                        XCB_EVENT_MASK_NO_EVENT as libc::c_int as uint32_t);
    wl_event_source_timer_update((*surface).ping_timer,
                                 (*(*surface).xwm).ping_timeout as
                                     libc::c_int);
    (*surface).pinging = 1i32 != 0;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* Anything above display is reset on Xwayland restart, rest is conserved */
/* *
	 * Add a custom event handler to xwayland. Return 1 if the event was
	 * handled or 0 to use the default wlr-xwayland handler. wlr-xwayland will
	 * free the event.
	 */
/* *
 * An Xwayland user interface component. It has an absolute position in
 * layout-local coordinates.
 *
 * When a surface is ready to be displayed, the `map` event is emitted. When a
 * surface should no longer be displayed, the `unmap` event is emitted. The
 * `unmap` event is guaranteed to be emitted before the `destroy` event if the
 * view is destroyed when mapped.
 */
// wlr_xwayland_surface::parent_link
// wlr_xwayland_surface::children
// _NET_WM_STATE
// xcb_config_window_t
// TODO: maybe add a seat to these
/* * Create an Xwayland server.
 *
 * The server supports a lazy mode in which Xwayland is only started when a
 * client tries to connect.
 *
 * Note: wlr_xwayland will setup a global SIGUSR1 handler on the compositor
 * process.
 */
/* * Metric to guess if an OR window should "receive" focus
 *
 * In the pure X setups, window managers usually straight up ignore override
 * redirect windows, and never touch them. (we have to handle them for mapping)
 *
 * When such a window wants to receive keyboard input (e.g. rofi/dzen) it will
 * use mechanics we don't support (sniffing/grabbing input).
 * [Sadly this is unrelated to xwayland-keyboard-grab]
 *
 * To still support these windows, while keeping general OR semantics as is, we
 * need to hand a subset of windows focus.
 * The dirty truth is, we need to hand focus to any Xwayland window, though
 * pretending this window has focus makes it easier to handle unmap.
 *
 * This function provides a handy metric based on the window type to guess if
 * the OR window wants focus.
 * It's probably not perfect, nor exactly intended but works in practice.
 *
 * Returns: true if the window should receive focus
 *          false if it should be ignored
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_xwayland_or_surface_wants_focus(mut surface:
                                                                 *const wlr_xwayland_surface)
 -> bool {
    let mut ret: bool = 1i32 != 0;
    static mut needles: [atom_name; 9] =
        [NET_WM_WINDOW_TYPE_COMBO, NET_WM_WINDOW_TYPE_DND,
         NET_WM_WINDOW_TYPE_DROPDOWN_MENU, NET_WM_WINDOW_TYPE_MENU,
         NET_WM_WINDOW_TYPE_NOTIFICATION, NET_WM_WINDOW_TYPE_POPUP_MENU,
         NET_WM_WINDOW_TYPE_SPLASH, NET_WM_WINDOW_TYPE_TOOLTIP,
         NET_WM_WINDOW_TYPE_UTILITY];
    let mut i: size_t = 0i32 as size_t;
    while i <
              (::std::mem::size_of::<[atom_name; 9]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<atom_name>()
                                                   as libc::c_ulong) {
        if xwm_atoms_contains((*surface).xwm, (*surface).window_type,
                              (*surface).window_type_len, needles[i as usize])
           {
            ret = 0i32 != 0
        }
        i = i.wrapping_add(1)
    }
    return ret;
}
