use libc;
extern "C" {
    pub type wl_client;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    /* creation/destruction */
    #[no_mangle]
    fn pixman_region32_init(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_init_rect(region: *mut pixman_region32_t,
                                 x: libc::c_int, y: libc::c_int,
                                 width: libc::c_uint, height: libc::c_uint);
    #[no_mangle]
    fn pixman_region32_fini(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_union_rect(dest: *mut pixman_region32_t,
                                  source: *mut pixman_region32_t,
                                  x: libc::c_int, y: libc::c_int,
                                  width: libc::c_uint, height: libc::c_uint)
     -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_subtract(reg_d: *mut pixman_region32_t,
                                reg_m: *mut pixman_region32_t,
                                reg_s: *mut pixman_region32_t)
     -> pixman_bool_t;
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
    fn wl_resource_set_implementation(resource: *mut wl_resource,
                                      implementation: *const libc::c_void,
                                      data: *mut libc::c_void,
                                      destroy: wl_resource_destroy_func_t);
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_resource_create(client: *mut wl_client,
                          interface: *const wl_interface,
                          version: libc::c_int, id: uint32_t)
     -> *mut wl_resource;
    #[no_mangle]
    fn wl_resource_get_link(resource: *mut wl_resource) -> *mut wl_list;
    #[no_mangle]
    fn wl_resource_get_user_data(resource: *mut wl_resource)
     -> *mut libc::c_void;
    #[no_mangle]
    fn wl_resource_instance_of(resource: *mut wl_resource,
                               interface: *const wl_interface,
                               implementation: *const libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn wl_resource_destroy(resource: *mut wl_resource);
    #[no_mangle]
    static wl_region_interface: wl_interface;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
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
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
/*
 * 32 bit regions
 */
pub type pixman_region32_data_t = pixman_region32_data;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;

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
pub struct wl_region_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub add: Option<unsafe extern "C" fn(_: *mut wl_client,
                                         _: *mut wl_resource, _: int32_t,
                                         _: int32_t, _: int32_t, _: int32_t)
                        -> ()>,
    pub subtract: Option<unsafe extern "C" fn(_: *mut wl_client,
                                              _: *mut wl_resource, _: int32_t,
                                              _: int32_t, _: int32_t,
                                              _: int32_t) -> ()>,
}
unsafe extern "C" fn region_add(mut client: *mut wl_client,
                                mut resource: *mut wl_resource,
                                mut x: int32_t, mut y: int32_t,
                                mut width: int32_t, mut height: int32_t) {
    let mut region: *mut pixman_region32_t =
        wlr_region_from_resource(resource);
    pixman_region32_union_rect(region, region, x, y, width as libc::c_uint,
                               height as libc::c_uint);
}
unsafe extern "C" fn region_subtract(mut client: *mut wl_client,
                                     mut resource: *mut wl_resource,
                                     mut x: int32_t, mut y: int32_t,
                                     mut width: int32_t,
                                     mut height: int32_t) {
    let mut region: *mut pixman_region32_t =
        wlr_region_from_resource(resource);
    pixman_region32_union_rect(region, region, x, y, width as libc::c_uint,
                               height as libc::c_uint);
    let mut rect: pixman_region32_t =
        pixman_region32_t{extents:
                              pixman_box32_t{x1: 0, y1: 0, x2: 0, y2: 0,},
                          data: 0 as *mut pixman_region32_data_t,};
    pixman_region32_init_rect(&mut rect, x, y, width as libc::c_uint,
                              height as libc::c_uint);
    pixman_region32_subtract(region, region, &mut rect);
    pixman_region32_fini(&mut rect);
}
unsafe extern "C" fn region_destroy(mut client: *mut wl_client,
                                    mut resource: *mut wl_resource) {
    wl_resource_destroy(resource);
}
static mut region_impl: wl_region_interface =
    {
    
        {
            let mut init =
                wl_region_interface{destroy:
                                        Some(region_destroy as
                                                 unsafe extern "C" fn(_:
                                                                          *mut wl_client,
                                                                      _:
                                                                          *mut wl_resource)
                                                     -> ()),
                                    add:
                                        Some(region_add as
                                                 unsafe extern "C" fn(_:
                                                                          *mut wl_client,
                                                                      _:
                                                                          *mut wl_resource,
                                                                      _:
                                                                          int32_t,
                                                                      _:
                                                                          int32_t,
                                                                      _:
                                                                          int32_t,
                                                                      _:
                                                                          int32_t)
                                                     -> ()),
                                    subtract:
                                        Some(region_subtract as
                                                 unsafe extern "C" fn(_:
                                                                          *mut wl_client,
                                                                      _:
                                                                          *mut wl_resource,
                                                                      _:
                                                                          int32_t,
                                                                      _:
                                                                          int32_t,
                                                                      _:
                                                                          int32_t,
                                                                      _:
                                                                          int32_t)
                                                     -> ()),};
            init
        }
};
unsafe extern "C" fn region_handle_resource_destroy(mut resource:
                                                        *mut wl_resource) {
    let mut reg: *mut pixman_region32_t = wlr_region_from_resource(resource);
    wl_list_remove(wl_resource_get_link(resource));
    pixman_region32_fini(reg);
    free(reg as *mut libc::c_void);
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/*
 * Creates a new region resource with the provided new ID. If `resource_list` is
 * non-NULL, adds the region's resource to the list.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_region_create(mut client: *mut wl_client,
                                           mut version: uint32_t,
                                           mut id: uint32_t,
                                           mut resource_list: *mut wl_list)
 -> *mut wl_resource {
    let mut region: *mut pixman_region32_t =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<pixman_region32_t>() as libc::c_ulong) as
            *mut pixman_region32_t;
    if region.is_null() {
        wl_client_post_no_memory(client);
        return 0 as *mut wl_resource
    }
    pixman_region32_init(region);
    let mut region_resource: *mut wl_resource =
        wl_resource_create(client, &wl_region_interface,
                           version as libc::c_int, id);
    if region_resource.is_null() {
        free(region as *mut libc::c_void);
        wl_client_post_no_memory(client);
        return 0 as *mut wl_resource
    }
    wl_resource_set_implementation(region_resource,
                                   &region_impl as *const wl_region_interface
                                       as *const libc::c_void,
                                   region as *mut libc::c_void,
                                   Some(region_handle_resource_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
    let mut resource_link: *mut wl_list =
        wl_resource_get_link(region_resource);
    if !resource_list.is_null() {
        wl_list_insert(resource_list, resource_link);
    } else { wl_list_init(resource_link); }
    return region_resource;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_region_from_resource(mut resource:
                                                      *mut wl_resource)
 -> *mut pixman_region32_t {
    if wl_resource_instance_of(resource, &wl_region_interface,
                               &region_impl as *const wl_region_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &wl_region_interface, &region_impl)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/wlr_region.c\x00" as *const u8 as
                          *const libc::c_char, 76i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"pixman_region32_t *wlr_region_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as *mut pixman_region32_t;
}
