use libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn XcursorImagesDestroy(images: *mut XcursorImages);
    #[no_mangle]
    fn xcursor_load_theme(theme: *const libc::c_char, size: libc::c_int,
                          load_callback_0:
                              Option<unsafe extern "C" fn(_:
                                                              *mut XcursorImages,
                                                          _:
                                                              *mut libc::c_void)
                                         -> ()>,
                          user_data: *mut libc::c_void);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
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
 * Copyright © 2012 Intel Corporation
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial
 * portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
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
    /* length of the animation in ms */
}
/* *
 * Container for an Xcursor theme.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xcursor_theme {
    pub cursor_count: libc::c_uint,
    pub cursors: *mut *mut wlr_xcursor,
    pub name: *mut libc::c_char,
    pub size: libc::c_int,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct cursor_metadata {
    pub name: *const libc::c_char,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub hotspot_x: libc::c_int,
    pub hotspot_y: libc::c_int,
    pub offset: size_t,
}
/*
 * Other data structures exposed by the library API
 */
pub type XcursorImages = _XcursorImages;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _XcursorImages {
    pub nimage: libc::c_int,
    pub images: *mut *mut XcursorImage,
    pub name: *mut libc::c_char,
}
pub type XcursorImage = _XcursorImage;

#[repr(C)]#[derive(Copy, Clone)]
pub struct _XcursorImage {
    pub version: XcursorUInt,
    pub size: XcursorDim,
    pub width: XcursorDim,
    pub height: XcursorDim,
    pub xhot: XcursorDim,
    pub yhot: XcursorDim,
    pub delay: XcursorUInt,
    pub pixels: *mut XcursorPixel,
}
pub type XcursorPixel = XcursorUInt;
pub type XcursorUInt = libc::c_uint;
pub type XcursorDim = XcursorUInt;
/* number of images */
/* array of XcursorImage pointers */
/* name used to load images */
/* version of the image data */
/* nominal size for matching */
/* actual width */
/* actual height */
/* hot spot x (must be inside image) */
/* hot spot y (must be inside image) */
/* animation delay to next frame (ms) */
/* pointer to pixels */
/*
 * Copyright © 2012 Intel Corporation
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial
 * portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
unsafe extern "C" fn xcursor_destroy(mut cursor: *mut wlr_xcursor) {
    let mut i: size_t = 0i32 as size_t;
    while i < (*cursor).image_count as libc::c_ulong {
        free((**(*cursor).images.offset(i as isize)).buffer as
                 *mut libc::c_void);
        free(*(*cursor).images.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free((*cursor).images as *mut libc::c_void);
    free((*cursor).name as *mut libc::c_void);
    free(cursor as *mut libc::c_void);
}
/*
* Copyright 1999 SuSE, Inc.
*
* Permission is hereby granted, free of charge, to any person obtaining
* a copy of this software and associated documentation files (the
* "Software"), to deal in the Software without restriction, including
* without limitation the rights to use, copy, modify, merge, publish,
* distribute, sublicense, and/or sell copies of the Software, and to
* permit persons to whom the Software is furnished to do so, subject to
* the following conditions:
*
* The above copyright notice and this permission notice (including the
* next paragraph) shall be included in all copies or substantial
* portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
* EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
* MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
* NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
* BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
* ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
* CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*
* Author:  Keith Packard, SuSE, Inc.
*/
static mut cursor_data: [uint32_t; 3008] =
    [0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0i32 as uint32_t,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0xff000000u32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xff000000u32, 0xff000000u32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xffffffffu32,
     0xffffffffu32, 0xffffffffu32, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0xffffffffu32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xffffffffu32, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t,
     0i32 as uint32_t, 0i32 as uint32_t, 0xffffffffu32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xff000000u32,
     0xff000000u32, 0xff000000u32, 0xff000000u32, 0xffffffffu32,
     0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t, 0i32 as uint32_t];
static mut cursor_metadata: [cursor_metadata; 13] =
    [{
         let mut init =
             cursor_metadata{name:
                                 b"bottom_left_corner\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 16i32,
                             height: 16i32,
                             hotspot_x: 1i32,
                             hotspot_y: 14i32,
                             offset: 0i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"bottom_right_corner\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 16i32,
                             height: 16i32,
                             hotspot_x: 14i32,
                             hotspot_y: 14i32,
                             offset: 256i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"bottom_side\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 15i32,
                             height: 16i32,
                             hotspot_x: 7i32,
                             hotspot_y: 14i32,
                             offset: 512i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"grabbing\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 16i32,
                             height: 16i32,
                             hotspot_x: 8i32,
                             hotspot_y: 8i32,
                             offset: 752i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"left_ptr\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 10i32,
                             height: 16i32,
                             hotspot_x: 1i32,
                             hotspot_y: 1i32,
                             offset: 1008i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"left_side\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 16i32,
                             height: 15i32,
                             hotspot_x: 1i32,
                             hotspot_y: 7i32,
                             offset: 1168i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"right_side\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 16i32,
                             height: 15i32,
                             hotspot_x: 14i32,
                             hotspot_y: 7i32,
                             offset: 1408i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"top_left_corner\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 16i32,
                             height: 16i32,
                             hotspot_x: 1i32,
                             hotspot_y: 1i32,
                             offset: 1648i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"top_right_corner\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 16i32,
                             height: 16i32,
                             hotspot_x: 14i32,
                             hotspot_y: 1i32,
                             offset: 1904i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"top_side\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 15i32,
                             height: 16i32,
                             hotspot_x: 7i32,
                             hotspot_y: 1i32,
                             offset: 2160i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"xterm\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 9i32,
                             height: 16i32,
                             hotspot_x: 4i32,
                             hotspot_y: 8i32,
                             offset: 2400i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"hand1\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 13i32,
                             height: 16i32,
                             hotspot_x: 12i32,
                             hotspot_y: 0i32,
                             offset: 2544i32 as size_t,};
         init
     },
     {
         let mut init =
             cursor_metadata{name:
                                 b"watch\x00" as *const u8 as
                                     *const libc::c_char,
                             width: 16i32,
                             height: 16i32,
                             hotspot_x: 15i32,
                             hotspot_y: 9i32,
                             offset: 2752i32 as size_t,};
         init
     }];
unsafe extern "C" fn xcursor_create_from_data(mut metadata:
                                                  *mut cursor_metadata,
                                              mut theme:
                                                  *mut wlr_xcursor_theme)
 -> *mut wlr_xcursor {
    let mut cursor: *mut wlr_xcursor = 0 as *mut wlr_xcursor;
    let mut image: *mut wlr_xcursor_image = 0 as *mut wlr_xcursor_image;
    let mut size: libc::c_int = 0;
    cursor =
        malloc(::std::mem::size_of::<wlr_xcursor>() as libc::c_ulong) as
            *mut wlr_xcursor;
    if cursor.is_null() { return 0 as *mut wlr_xcursor }
    (*cursor).image_count = 1i32 as libc::c_uint;
    (*cursor).images =
        malloc(::std::mem::size_of::<*mut wlr_xcursor_image>() as
                   libc::c_ulong) as *mut *mut wlr_xcursor_image;
    if !(*cursor).images.is_null() {
        (*cursor).name = strdup((*metadata).name);
        (*cursor).total_delay = 0i32 as uint32_t;
        image =
            malloc(::std::mem::size_of::<wlr_xcursor_image>() as
                       libc::c_ulong) as *mut wlr_xcursor_image;
        if !image.is_null() {
            let ref mut fresh0 = *(*cursor).images.offset(0);
            *fresh0 = image;
            (*image).buffer = 0 as *mut uint8_t;
            (*image).width = (*metadata).width as uint32_t;
            (*image).height = (*metadata).height as uint32_t;
            (*image).hotspot_x = (*metadata).hotspot_x as uint32_t;
            (*image).hotspot_y = (*metadata).hotspot_y as uint32_t;
            (*image).delay = 0i32 as uint32_t;
            size =
                (((*metadata).width * (*metadata).height) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint32_t>()
                                                     as libc::c_ulong) as
                    libc::c_int;
            (*image).buffer = malloc(size as libc::c_ulong) as *mut uint8_t;
            if (*image).buffer.is_null() {
                free(image as *mut libc::c_void);
            } else {
                memcpy((*image).buffer as *mut libc::c_void,
                       cursor_data.as_mut_ptr().offset((*metadata).offset as
                                                           isize) as
                           *const libc::c_void, size as libc::c_ulong);
                return cursor
            }
        }
        free((*cursor).name as *mut libc::c_void);
        free((*cursor).images as *mut libc::c_void);
    }
    free(cursor as *mut libc::c_void);
    return 0 as *mut wlr_xcursor;
}
unsafe extern "C" fn load_default_theme(mut theme: *mut wlr_xcursor_theme) {
    let mut i: uint32_t = 0;
    free((*theme).name as *mut libc::c_void);
    (*theme).name =
        strdup(b"default\x00" as *const u8 as *const libc::c_char);
    (*theme).cursor_count =
        (::std::mem::size_of::<[cursor_metadata; 13]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<cursor_metadata>()
                                             as libc::c_ulong) as
            libc::c_uint;
    (*theme).cursors =
        malloc(((*theme).cursor_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut wlr_xcursor>()
                                                    as libc::c_ulong)) as
            *mut *mut wlr_xcursor;
    if (*theme).cursors.is_null() {
        (*theme).cursor_count = 0i32 as libc::c_uint;
        return
    }
    i = 0i32 as uint32_t;
    while i < (*theme).cursor_count {
        let ref mut fresh1 = *(*theme).cursors.offset(i as isize);
        *fresh1 =
            xcursor_create_from_data(&mut *cursor_metadata.as_mut_ptr().offset(i
                                                                                   as
                                                                                   isize),
                                     theme);
        if (*(*theme).cursors.offset(i as isize)).is_null() { break ; }
        i = i.wrapping_add(1)
    }
    (*theme).cursor_count = i;
}
unsafe extern "C" fn xcursor_create_from_xcursor_images(mut images:
                                                            *mut XcursorImages,
                                                        mut theme:
                                                            *mut wlr_xcursor_theme)
 -> *mut wlr_xcursor {
    let mut cursor: *mut wlr_xcursor = 0 as *mut wlr_xcursor;
    let mut image: *mut wlr_xcursor_image = 0 as *mut wlr_xcursor_image;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    cursor =
        malloc(::std::mem::size_of::<wlr_xcursor>() as libc::c_ulong) as
            *mut wlr_xcursor;
    if cursor.is_null() { return 0 as *mut wlr_xcursor }
    (*cursor).images =
        malloc(((*images).nimage as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut wlr_xcursor_image>()
                                                    as libc::c_ulong)) as
            *mut *mut wlr_xcursor_image;
    if (*cursor).images.is_null() {
        free(cursor as *mut libc::c_void);
        return 0 as *mut wlr_xcursor
    }
    (*cursor).name = strdup((*images).name);
    (*cursor).total_delay = 0i32 as uint32_t;
    i = 0i32;
    while i < (*images).nimage {
        image =
            malloc(::std::mem::size_of::<wlr_xcursor_image>() as
                       libc::c_ulong) as *mut wlr_xcursor_image;
        if image.is_null() { break ; }
        (*image).buffer = 0 as *mut uint8_t;
        (*image).width = (**(*images).images.offset(i as isize)).width;
        (*image).height = (**(*images).images.offset(i as isize)).height;
        (*image).hotspot_x = (**(*images).images.offset(i as isize)).xhot;
        (*image).hotspot_y = (**(*images).images.offset(i as isize)).yhot;
        (*image).delay = (**(*images).images.offset(i as isize)).delay;
        size =
            (*image).width.wrapping_mul((*image).height).wrapping_mul(4i32 as
                                                                          libc::c_uint)
                as libc::c_int;
        (*image).buffer = malloc(size as libc::c_ulong) as *mut uint8_t;
        if (*image).buffer.is_null() {
            free(image as *mut libc::c_void);
            break ;
        } else {
            /* copy pixels to shm pool */
            memcpy((*image).buffer as *mut libc::c_void,
                   (**(*images).images.offset(i as isize)).pixels as
                       *const libc::c_void, size as libc::c_ulong);
            (*cursor).total_delay =
                ((*cursor).total_delay as
                     libc::c_uint).wrapping_add((*image).delay) as uint32_t as
                    uint32_t;
            let ref mut fresh2 = *(*cursor).images.offset(i as isize);
            *fresh2 = image;
            i += 1
        }
    }
    (*cursor).image_count = i as libc::c_uint;
    if (*cursor).image_count == 0i32 as libc::c_uint {
        free((*cursor).name as *mut libc::c_void);
        free((*cursor).images as *mut libc::c_void);
        free(cursor as *mut libc::c_void);
        return 0 as *mut wlr_xcursor
    }
    return cursor;
}
unsafe extern "C" fn load_callback(mut images: *mut XcursorImages,
                                   mut data: *mut libc::c_void) {
    let mut theme: *mut wlr_xcursor_theme = data as *mut wlr_xcursor_theme;
    let mut cursor: *mut wlr_xcursor = 0 as *mut wlr_xcursor;
    if !wlr_xcursor_theme_get_cursor(theme, (*images).name).is_null() {
        XcursorImagesDestroy(images);
        return
    }
    cursor = xcursor_create_from_xcursor_images(images, theme);
    if !cursor.is_null() {
        let mut cursors: *mut *mut wlr_xcursor = 0 as *mut *mut wlr_xcursor;
        (*theme).cursor_count = (*theme).cursor_count.wrapping_add(1);
        cursors =
            realloc((*theme).cursors as *mut libc::c_void,
                    ((*theme).cursor_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut wlr_xcursor>()
                                                         as libc::c_ulong)) as
                *mut *mut wlr_xcursor;
        if cursors.is_null() {
            (*theme).cursor_count = (*theme).cursor_count.wrapping_sub(1);
            free(cursor as *mut libc::c_void);
        } else {
            (*theme).cursors = cursors;
            let ref mut fresh3 =
                *(*theme).cursors.offset((*theme).cursor_count.wrapping_sub(1i32
                                                                                as
                                                                                libc::c_uint)
                                             as isize);
            *fresh3 = cursor
        }
    }
    XcursorImagesDestroy(images);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xcursor_theme_load(mut name: *const libc::c_char,
                                                mut size: libc::c_int)
 -> *mut wlr_xcursor_theme {
    let mut theme: *mut wlr_xcursor_theme = 0 as *mut wlr_xcursor_theme;
    theme =
        malloc(::std::mem::size_of::<wlr_xcursor_theme>() as libc::c_ulong) as
            *mut wlr_xcursor_theme;
    if theme.is_null() { return 0 as *mut wlr_xcursor_theme }
    if name.is_null() {
        name = b"default\x00" as *const u8 as *const libc::c_char
    }
    (*theme).name = strdup(name);
    if (*theme).name.is_null() {
        free(theme as *mut libc::c_void);
        return 0 as *mut wlr_xcursor_theme
    } else {
        (*theme).size = size;
        (*theme).cursor_count = 0i32 as libc::c_uint;
        (*theme).cursors = 0 as *mut *mut wlr_xcursor;
        xcursor_load_theme(name, size,
                           Some(load_callback as
                                    unsafe extern "C" fn(_:
                                                             *mut XcursorImages,
                                                         _: *mut libc::c_void)
                                        -> ()), theme as *mut libc::c_void);
        if (*theme).cursor_count == 0i32 as libc::c_uint {
            load_default_theme(theme);
        }
        _wlr_log(WLR_DEBUG,
                 b"[%s:%d] Loaded cursor theme \'%s\', available cursors:\x00"
                     as *const u8 as *const libc::c_char,
                 b"../xcursor/wlr_xcursor.c\x00" as *const u8 as
                     *const libc::c_char, 244i32, (*theme).name);
        let mut i: size_t = 0i32 as size_t;
        while i < (*theme).cursor_count as libc::c_ulong {
            let mut c: *mut wlr_xcursor =
                *(*theme).cursors.offset(i as isize);
            let mut i_0: *mut wlr_xcursor_image = *(*c).images.offset(0);
            _wlr_log(WLR_DEBUG,
                     b"[%s:%d] %s (%u images) %dx%d+%d,%d\x00" as *const u8 as
                         *const libc::c_char,
                     b"../xcursor/wlr_xcursor.c\x00" as *const u8 as
                         *const libc::c_char, 250i32, (*c).name,
                     (*c).image_count, (*i_0).width, (*i_0).height,
                     (*i_0).hotspot_x, (*i_0).hotspot_y);
            i = i.wrapping_add(1)
        }
        return theme
    };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xcursor_theme_destroy(mut theme:
                                                       *mut wlr_xcursor_theme) {
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while i < (*theme).cursor_count {
        xcursor_destroy(*(*theme).cursors.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*theme).name as *mut libc::c_void);
    free((*theme).cursors as *mut libc::c_void);
    free(theme as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xcursor_theme_get_cursor(mut theme:
                                                          *mut wlr_xcursor_theme,
                                                      mut name:
                                                          *const libc::c_char)
 -> *mut wlr_xcursor {
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while i < (*theme).cursor_count {
        if strcmp(name, (**(*theme).cursors.offset(i as isize)).name) == 0i32
           {
            return *(*theme).cursors.offset(i as isize)
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut wlr_xcursor;
}
unsafe extern "C" fn xcursor_frame_and_duration(mut cursor: *mut wlr_xcursor,
                                                mut time: uint32_t,
                                                mut duration: *mut uint32_t)
 -> libc::c_int {
    let mut t: uint32_t = 0;
    let mut i: libc::c_int = 0;
    if (*cursor).image_count == 1i32 as libc::c_uint {
        if !duration.is_null() { *duration = 0i32 as uint32_t }
        return 0i32
    }
    i = 0i32;
    t = time.wrapping_rem((*cursor).total_delay);
    /* If there is a 0 delay in the image set then this
	 * loop breaks on it and we display that cursor until
	 * time % cursor->total_delay wraps again.
	 * Since a 0 delay is silly, and we've never actually
	 * seen one in a cursor file, we haven't bothered to
	 * "fix" this.
	 */
    while t.wrapping_sub((**(*cursor).images.offset(i as isize)).delay) < t {
        let fresh4 = i;
        i = i + 1;
        t =
            (t as
                 libc::c_uint).wrapping_sub((**(*cursor).images.offset(fresh4
                                                                           as
                                                                           isize)).delay)
                as uint32_t as uint32_t
    }
    if duration.is_null() { return i }
    /* Make sure we don't accidentally tell the caller this is
	 * a static cursor image.
	 */
    if t >= (**(*cursor).images.offset(i as isize)).delay {
        *duration = 1i32 as uint32_t
    } else {
        *duration =
            (**(*cursor).images.offset(i as isize)).delay.wrapping_sub(t)
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_xcursor_frame(mut _cursor: *mut wlr_xcursor,
                                           mut time: uint32_t)
 -> libc::c_int {
    return xcursor_frame_and_duration(_cursor, time, 0 as *mut uint32_t);
}
/* *
 * Loads the named xcursor theme at the given cursor size (in pixels). This is
 * useful if you need cursor images for your compositor to use when a
 * client-side cursors is not available or you wish to override client-side
 * cursors for a particular UI interaction (such as using a grab cursor when
 * moving a window around).
 */
/* *
 * Obtains a wlr_xcursor image for the specified cursor name (e.g. "left_ptr").
 */
/* *
 * Returns the current frame number for an animated cursor give a monotonic time
 * reference.
 */
/* *
 * Get the name of the resize cursor image for the given edges.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_xcursor_get_resize_name(mut edges: wlr_edges)
 -> *const libc::c_char {
    if edges as libc::c_uint & WLR_EDGE_TOP as libc::c_int as libc::c_uint !=
           0 {
        if edges as libc::c_uint &
               WLR_EDGE_RIGHT as libc::c_int as libc::c_uint != 0 {
            return b"ne-resize\x00" as *const u8 as *const libc::c_char
        } else {
            if edges as libc::c_uint &
                   WLR_EDGE_LEFT as libc::c_int as libc::c_uint != 0 {
                return b"nw-resize\x00" as *const u8 as *const libc::c_char
            }
        }
        return b"n-resize\x00" as *const u8 as *const libc::c_char
    } else {
        if edges as libc::c_uint &
               WLR_EDGE_BOTTOM as libc::c_int as libc::c_uint != 0 {
            if edges as libc::c_uint &
                   WLR_EDGE_RIGHT as libc::c_int as libc::c_uint != 0 {
                return b"se-resize\x00" as *const u8 as *const libc::c_char
            } else {
                if edges as libc::c_uint &
                       WLR_EDGE_LEFT as libc::c_int as libc::c_uint != 0 {
                    return b"sw-resize\x00" as *const u8 as
                               *const libc::c_char
                }
            }
            return b"s-resize\x00" as *const u8 as *const libc::c_char
        } else {
            if edges as libc::c_uint &
                   WLR_EDGE_RIGHT as libc::c_int as libc::c_uint != 0 {
                return b"e-resize\x00" as *const u8 as *const libc::c_char
            } else {
                if edges as libc::c_uint &
                       WLR_EDGE_LEFT as libc::c_int as libc::c_uint != 0 {
                    return b"w-resize\x00" as *const u8 as *const libc::c_char
                }
            }
        }
    }
    return b"se-resize\x00" as *const u8 as *const libc::c_char;
    // fallback
}
