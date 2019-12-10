use libc;
extern "C" {
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn pixman_region32_init_rects(region: *mut pixman_region32_t,
                                  boxes: *const pixman_box32_t,
                                  count: libc::c_int) -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_fini(region: *mut pixman_region32_t);
    #[no_mangle]
    fn pixman_region32_copy(dest: *mut pixman_region32_t,
                            source: *mut pixman_region32_t) -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_contains_point(region: *mut pixman_region32_t,
                                      x: libc::c_int, y: libc::c_int,
                                      box_0: *mut pixman_box32_t)
     -> pixman_bool_t;
    #[no_mangle]
    fn pixman_region32_rectangles(region: *mut pixman_region32_t,
                                  n_rects: *mut libc::c_int)
     -> *mut pixman_box32_t;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
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
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
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
/* *
 * Scales a region, ie. multiplies all its coordinates by `scale`.
 *
 * The resulting coordinates are rounded up or down so that the new region is
 * at least as big as the original one.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_region_scale(mut dst: *mut pixman_region32_t,
                                          mut src: *mut pixman_region32_t,
                                          mut scale: libc::c_float) {
    if scale == 1i32 as libc::c_float {
        pixman_region32_copy(dst, src);
        return
    }
    let mut nrects: libc::c_int = 0;
    let mut src_rects: *mut pixman_box32_t =
        pixman_region32_rectangles(src, &mut nrects);
    let mut dst_rects: *mut pixman_box32_t =
        malloc((nrects as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<pixman_box32_t>()
                                                    as libc::c_ulong)) as
            *mut pixman_box32_t;
    if dst_rects.is_null() { return }
    let mut i: libc::c_int = 0i32;
    while i < nrects {
        (*dst_rects.offset(i as isize)).x1 =
            floor(((*src_rects.offset(i as isize)).x1 as libc::c_float *
                       scale) as libc::c_double) as int32_t;
        (*dst_rects.offset(i as isize)).x2 =
            ceil(((*src_rects.offset(i as isize)).x2 as libc::c_float * scale)
                     as libc::c_double) as int32_t;
        (*dst_rects.offset(i as isize)).y1 =
            floor(((*src_rects.offset(i as isize)).y1 as libc::c_float *
                       scale) as libc::c_double) as int32_t;
        (*dst_rects.offset(i as isize)).y2 =
            ceil(((*src_rects.offset(i as isize)).y2 as libc::c_float * scale)
                     as libc::c_double) as int32_t;
        i += 1
    }
    pixman_region32_fini(dst);
    pixman_region32_init_rects(dst, dst_rects, nrects);
    free(dst_rects as *mut libc::c_void);
}
/* *
 * Applies a transform to a region inside a box of size `width` x `height`.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_region_transform(mut dst: *mut pixman_region32_t,
                                              mut src: *mut pixman_region32_t,
                                              mut transform:
                                                  wl_output_transform,
                                              mut width: libc::c_int,
                                              mut height: libc::c_int) {
    if transform as libc::c_uint ==
           WL_OUTPUT_TRANSFORM_NORMAL as libc::c_int as libc::c_uint {
        pixman_region32_copy(dst, src);
        return
    }
    let mut nrects: libc::c_int = 0;
    let mut src_rects: *mut pixman_box32_t =
        pixman_region32_rectangles(src, &mut nrects);
    let mut dst_rects: *mut pixman_box32_t =
        malloc((nrects as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<pixman_box32_t>()
                                                    as libc::c_ulong)) as
            *mut pixman_box32_t;
    if dst_rects.is_null() { return }
    let mut i: libc::c_int = 0i32;
    while i < nrects {
        match transform as libc::c_uint {
            0 => {
                (*dst_rects.offset(i as isize)).x1 =
                    (*src_rects.offset(i as isize)).x1;
                (*dst_rects.offset(i as isize)).y1 =
                    (*src_rects.offset(i as isize)).y1;
                (*dst_rects.offset(i as isize)).x2 =
                    (*src_rects.offset(i as isize)).x2;
                (*dst_rects.offset(i as isize)).y2 =
                    (*src_rects.offset(i as isize)).y2
            }
            1 => {
                (*dst_rects.offset(i as isize)).x1 =
                    (*src_rects.offset(i as isize)).y1;
                (*dst_rects.offset(i as isize)).y1 =
                    width - (*src_rects.offset(i as isize)).x2;
                (*dst_rects.offset(i as isize)).x2 =
                    (*src_rects.offset(i as isize)).y2;
                (*dst_rects.offset(i as isize)).y2 =
                    width - (*src_rects.offset(i as isize)).x1
            }
            2 => {
                (*dst_rects.offset(i as isize)).x1 =
                    width - (*src_rects.offset(i as isize)).x2;
                (*dst_rects.offset(i as isize)).y1 =
                    height - (*src_rects.offset(i as isize)).y2;
                (*dst_rects.offset(i as isize)).x2 =
                    width - (*src_rects.offset(i as isize)).x1;
                (*dst_rects.offset(i as isize)).y2 =
                    height - (*src_rects.offset(i as isize)).y1
            }
            3 => {
                (*dst_rects.offset(i as isize)).x1 =
                    height - (*src_rects.offset(i as isize)).y2;
                (*dst_rects.offset(i as isize)).y1 =
                    (*src_rects.offset(i as isize)).x1;
                (*dst_rects.offset(i as isize)).x2 =
                    height - (*src_rects.offset(i as isize)).y1;
                (*dst_rects.offset(i as isize)).y2 =
                    (*src_rects.offset(i as isize)).x2
            }
            4 => {
                (*dst_rects.offset(i as isize)).x1 =
                    width - (*src_rects.offset(i as isize)).x2;
                (*dst_rects.offset(i as isize)).y1 =
                    (*src_rects.offset(i as isize)).y1;
                (*dst_rects.offset(i as isize)).x2 =
                    width - (*src_rects.offset(i as isize)).x1;
                (*dst_rects.offset(i as isize)).y2 =
                    (*src_rects.offset(i as isize)).y2
            }
            5 => {
                (*dst_rects.offset(i as isize)).x1 =
                    height - (*src_rects.offset(i as isize)).y2;
                (*dst_rects.offset(i as isize)).y1 =
                    width - (*src_rects.offset(i as isize)).x2;
                (*dst_rects.offset(i as isize)).x2 =
                    height - (*src_rects.offset(i as isize)).y1;
                (*dst_rects.offset(i as isize)).y2 =
                    width - (*src_rects.offset(i as isize)).x1
            }
            6 => {
                (*dst_rects.offset(i as isize)).x1 =
                    (*src_rects.offset(i as isize)).x1;
                (*dst_rects.offset(i as isize)).y1 =
                    height - (*src_rects.offset(i as isize)).y2;
                (*dst_rects.offset(i as isize)).x2 =
                    (*src_rects.offset(i as isize)).x2;
                (*dst_rects.offset(i as isize)).y2 =
                    height - (*src_rects.offset(i as isize)).y1
            }
            7 => {
                (*dst_rects.offset(i as isize)).x1 =
                    (*src_rects.offset(i as isize)).y1;
                (*dst_rects.offset(i as isize)).y1 =
                    (*src_rects.offset(i as isize)).x1;
                (*dst_rects.offset(i as isize)).x2 =
                    (*src_rects.offset(i as isize)).y2;
                (*dst_rects.offset(i as isize)).y2 =
                    (*src_rects.offset(i as isize)).x2
            }
            _ => { }
        }
        i += 1
    }
    pixman_region32_fini(dst);
    pixman_region32_init_rects(dst, dst_rects, nrects);
    free(dst_rects as *mut libc::c_void);
}
/* *
 * Expands the region of `distance`. If `distance` is negative, it shrinks the
 * region.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_region_expand(mut dst: *mut pixman_region32_t,
                                           mut src: *mut pixman_region32_t,
                                           mut distance: libc::c_int) {
    if distance == 0i32 { pixman_region32_copy(dst, src); return }
    let mut nrects: libc::c_int = 0;
    let mut src_rects: *mut pixman_box32_t =
        pixman_region32_rectangles(src, &mut nrects);
    let mut dst_rects: *mut pixman_box32_t =
        malloc((nrects as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<pixman_box32_t>()
                                                    as libc::c_ulong)) as
            *mut pixman_box32_t;
    if dst_rects.is_null() { return }
    let mut i: libc::c_int = 0i32;
    while i < nrects {
        (*dst_rects.offset(i as isize)).x1 =
            (*src_rects.offset(i as isize)).x1 - distance;
        (*dst_rects.offset(i as isize)).x2 =
            (*src_rects.offset(i as isize)).x2 + distance;
        (*dst_rects.offset(i as isize)).y1 =
            (*src_rects.offset(i as isize)).y1 - distance;
        (*dst_rects.offset(i as isize)).y2 =
            (*src_rects.offset(i as isize)).y2 + distance;
        i += 1
    }
    pixman_region32_fini(dst);
    pixman_region32_init_rects(dst, dst_rects, nrects);
    free(dst_rects as *mut libc::c_void);
}
/*
 * Builds the smallest possible region that contains the region rotated about
 * the point (ox, oy).
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_region_rotated_bounds(mut dst:
                                                       *mut pixman_region32_t,
                                                   mut src:
                                                       *mut pixman_region32_t,
                                                   mut rotation:
                                                       libc::c_float,
                                                   mut ox: libc::c_int,
                                                   mut oy: libc::c_int) {
    if rotation == 0i32 as libc::c_float {
        pixman_region32_copy(dst, src);
        return
    }
    let mut nrects: libc::c_int = 0;
    let mut src_rects: *mut pixman_box32_t =
        pixman_region32_rectangles(src, &mut nrects);
    let mut dst_rects: *mut pixman_box32_t =
        malloc((nrects as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<pixman_box32_t>()
                                                    as libc::c_ulong)) as
            *mut pixman_box32_t;
    if dst_rects.is_null() { return }
    let mut i: libc::c_int = 0i32;
    while i < nrects {
        let mut x1: libc::c_double =
            ((*src_rects.offset(i as isize)).x1 - ox) as libc::c_double;
        let mut y1: libc::c_double =
            ((*src_rects.offset(i as isize)).y1 - oy) as libc::c_double;
        let mut x2: libc::c_double =
            ((*src_rects.offset(i as isize)).x2 - ox) as libc::c_double;
        let mut y2: libc::c_double =
            ((*src_rects.offset(i as isize)).y2 - oy) as libc::c_double;
        let mut rx1: libc::c_double =
            x1 * cos(rotation as libc::c_double) -
                y1 * sin(rotation as libc::c_double);
        let mut ry1: libc::c_double =
            x1 * sin(rotation as libc::c_double) +
                y1 * cos(rotation as libc::c_double);
        let mut rx2: libc::c_double =
            x2 * cos(rotation as libc::c_double) -
                y1 * sin(rotation as libc::c_double);
        let mut ry2: libc::c_double =
            x2 * sin(rotation as libc::c_double) +
                y1 * cos(rotation as libc::c_double);
        let mut rx3: libc::c_double =
            x2 * cos(rotation as libc::c_double) -
                y2 * sin(rotation as libc::c_double);
        let mut ry3: libc::c_double =
            x2 * sin(rotation as libc::c_double) +
                y2 * cos(rotation as libc::c_double);
        let mut rx4: libc::c_double =
            x1 * cos(rotation as libc::c_double) -
                y2 * sin(rotation as libc::c_double);
        let mut ry4: libc::c_double =
            x1 * sin(rotation as libc::c_double) +
                y2 * cos(rotation as libc::c_double);
        x1 = fmin(fmin(rx1, rx2), fmin(rx3, rx4));
        y1 = fmin(fmin(ry1, ry2), fmin(ry3, ry4));
        x2 = fmax(fmax(rx1, rx2), fmax(rx3, rx4));
        y2 = fmax(fmax(ry1, ry2), fmax(ry3, ry4));
        (*dst_rects.offset(i as isize)).x1 =
            floor(ox as libc::c_double + x1) as int32_t;
        (*dst_rects.offset(i as isize)).x2 =
            ceil(ox as libc::c_double + x2) as int32_t;
        (*dst_rects.offset(i as isize)).y1 =
            floor(oy as libc::c_double + y1) as int32_t;
        (*dst_rects.offset(i as isize)).y2 =
            ceil(oy as libc::c_double + y2) as int32_t;
        i += 1
    }
    pixman_region32_fini(dst);
    pixman_region32_init_rects(dst, dst_rects, nrects);
    free(dst_rects as *mut libc::c_void);
}
unsafe extern "C" fn region_confine(mut region: *mut pixman_region32_t,
                                    mut x1: libc::c_double,
                                    mut y1: libc::c_double,
                                    mut x2: libc::c_double,
                                    mut y2: libc::c_double,
                                    mut x2_out: *mut libc::c_double,
                                    mut y2_out: *mut libc::c_double,
                                    mut box_0: pixman_box32_t) {
    let mut x_clamped: libc::c_double =
        fmax(fmin(x2, (box_0.x2 - 1i32) as libc::c_double),
             box_0.x1 as libc::c_double);
    let mut y_clamped: libc::c_double =
        fmax(fmin(y2, (box_0.y2 - 1i32) as libc::c_double),
             box_0.y1 as libc::c_double);
    // If the target coordinates are above box.{x,y}2 - 1, but less than
	// box.{x,y}2, then they are still within the box.
    if floor(x_clamped) == floor(x2) && floor(y_clamped) == floor(y2) {
        *x2_out = x2;
        *y2_out = y2;
        return
    }
    let mut dx: libc::c_double = x2 - x1;
    let mut dy: libc::c_double = y2 - y1;
    // We use fabs to avoid negative zeroes and thus avoid a bug
	// with negative infinity.
    let mut delta: libc::c_double =
        fmin(fabs(x_clamped - x1) / fabs(dx),
             fabs(y_clamped - y1) / fabs(dy));
    // We clamp it again due to precision errors.
    let mut x: libc::c_double =
        fmax(fmin(delta * dx + x1, (box_0.x2 - 1i32) as libc::c_double),
             box_0.x1 as libc::c_double);
    let mut y: libc::c_double =
        fmax(fmin(delta * dy + y1, (box_0.y2 - 1i32) as libc::c_double),
             box_0.y1 as libc::c_double);
    // Go one unit past the boundary to find an adjacent box.
    let mut x_ext: libc::c_int =
        (floor(x) +
             (if dx == 0i32 as libc::c_double {
                  0i32
              } else {
                  (if dx > 0i32 as libc::c_double { 1i32 } else { -1i32 })
              }) as libc::c_double) as libc::c_int;
    let mut y_ext: libc::c_int =
        (floor(y) +
             (if dy == 0i32 as libc::c_double {
                  0i32
              } else {
                  (if dy > 0i32 as libc::c_double { 1i32 } else { -1i32 })
              }) as libc::c_double) as libc::c_int;
    if pixman_region32_contains_point(region, x_ext, y_ext, &mut box_0) != 0 {
        return region_confine(region, x1, y1, x2, y2, x2_out, y2_out, box_0)
    } else {
        if dx == 0i32 as libc::c_double || dy == 0i32 as libc::c_double {
            *x2_out = x;
            *y2_out = y
        } else {
            let mut bordering_x: bool =
                x == box_0.x1 as libc::c_double ||
                    x == (box_0.x2 - 1i32) as libc::c_double;
            let mut bordering_y: bool =
                y == box_0.y1 as libc::c_double ||
                    y == (box_0.y2 - 1i32) as libc::c_double;
            if bordering_x as libc::c_int == bordering_y as libc::c_int {
                let mut x2_potential: libc::c_double = 0.;
                let mut y2_potential: libc::c_double = 0.;
                let mut tmp1: libc::c_double = 0.;
                let mut tmp2: libc::c_double = 0.;
                region_confine(region, x, y, x, y2, &mut tmp1,
                               &mut y2_potential, box_0);
                region_confine(region, x, y, x2, y, &mut x2_potential,
                               &mut tmp2, box_0);
                if fabs(x2_potential - x) > fabs(y2_potential - y) {
                    *x2_out = x2_potential;
                    *y2_out = y
                } else { *x2_out = x; *y2_out = y2_potential }
            } else if bordering_x {
                return region_confine(region, x, y, x, y2, x2_out, y2_out,
                                      box_0)
            } else {
                if bordering_y {
                    return region_confine(region, x, y, x2, y, x2_out, y2_out,
                                          box_0)
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_region_confine(mut region:
                                                *mut pixman_region32_t,
                                            mut x1: libc::c_double,
                                            mut y1: libc::c_double,
                                            mut x2: libc::c_double,
                                            mut y2: libc::c_double,
                                            mut x2_out: *mut libc::c_double,
                                            mut y2_out: *mut libc::c_double)
 -> bool {
    let mut box_0: pixman_box32_t =
        pixman_box32_t{x1: 0, y1: 0, x2: 0, y2: 0,};
    if pixman_region32_contains_point(region, floor(x1) as libc::c_int,
                                      floor(y1) as libc::c_int, &mut box_0) !=
           0 {
        region_confine(region, x1, y1, x2, y2, x2_out, y2_out, box_0);
        return 1i32 != 0
    } else { return 0i32 != 0 };
}
