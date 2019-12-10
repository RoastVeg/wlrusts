use libc;
extern "C" {
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
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
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn wlr_box_closest_point(mut box_0: *const wlr_box,
                                               mut x: libc::c_double,
                                               mut y: libc::c_double,
                                               mut dest_x:
                                                   *mut libc::c_double,
                                               mut dest_y:
                                                   *mut libc::c_double) {
    // if box is empty, then it contains no points, so no closest point either
    if (*box_0).width <= 0i32 || (*box_0).height <= 0i32 {
        *dest_x = ::std::f32::NAN as libc::c_double;
        *dest_y = ::std::f32::NAN as libc::c_double;
        return
    }
    // find the closest x point
    if x < (*box_0).x as libc::c_double {
        *dest_x = (*box_0).x as libc::c_double
    } else if x >= ((*box_0).x + (*box_0).width) as libc::c_double {
        *dest_x = ((*box_0).x + (*box_0).width - 1i32) as libc::c_double
    } else { *dest_x = x }
    // find closest y point
    if y < (*box_0).y as libc::c_double {
        *dest_y = (*box_0).y as libc::c_double
    } else if y >= ((*box_0).y + (*box_0).height) as libc::c_double {
        *dest_y = ((*box_0).y + (*box_0).height - 1i32) as libc::c_double
    } else { *dest_y = y };
}
#[no_mangle]
pub unsafe extern "C" fn wlr_box_empty(mut box_0: *const wlr_box) -> bool {
    return box_0.is_null() || (*box_0).width <= 0i32 ||
               (*box_0).height <= 0i32;
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_box_intersection(mut dest: *mut wlr_box,
                                              mut box_a: *const wlr_box,
                                              mut box_b: *const wlr_box)
 -> bool {
    let mut a_empty: bool = wlr_box_empty(box_a);
    let mut b_empty: bool = wlr_box_empty(box_b);
    if a_empty as libc::c_int != 0 || b_empty as libc::c_int != 0 {
        (*dest).x = 0i32;
        (*dest).y = 0i32;
        (*dest).width = -100i32;
        (*dest).height = -100i32;
        return 0i32 != 0
    }
    let mut x1: libc::c_int =
        fmax((*box_a).x as libc::c_double, (*box_b).x as libc::c_double) as
            libc::c_int;
    let mut y1: libc::c_int =
        fmax((*box_a).y as libc::c_double, (*box_b).y as libc::c_double) as
            libc::c_int;
    let mut x2: libc::c_int =
        fmin(((*box_a).x + (*box_a).width) as libc::c_double,
             ((*box_b).x + (*box_b).width) as libc::c_double) as libc::c_int;
    let mut y2: libc::c_int =
        fmin(((*box_a).y + (*box_a).height) as libc::c_double,
             ((*box_b).y + (*box_b).height) as libc::c_double) as libc::c_int;
    (*dest).x = x1;
    (*dest).y = y1;
    (*dest).width = x2 - x1;
    (*dest).height = y2 - y1;
    return !wlr_box_empty(dest);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_box_contains_point(mut box_0: *const wlr_box,
                                                mut x: libc::c_double,
                                                mut y: libc::c_double)
 -> bool {
    if wlr_box_empty(box_0) {
        return 0i32 != 0
    } else {
        return x >= (*box_0).x as libc::c_double &&
                   x < ((*box_0).x + (*box_0).width) as libc::c_double &&
                   y >= (*box_0).y as libc::c_double &&
                   y < ((*box_0).y + (*box_0).height) as libc::c_double
    };
}
/* *
 * Transforms a box inside a `width` x `height` box.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_box_transform(mut dest: *mut wlr_box,
                                           mut box_0: *const wlr_box,
                                           mut transform: wl_output_transform,
                                           mut width: libc::c_int,
                                           mut height: libc::c_int) {
    let mut src: wlr_box = *box_0;
    if (transform as libc::c_uint).wrapping_rem(2i32 as libc::c_uint) ==
           0i32 as libc::c_uint {
        (*dest).width = src.width;
        (*dest).height = src.height
    } else { (*dest).width = src.height; (*dest).height = src.width }
    match transform as libc::c_uint {
        0 => { (*dest).x = src.x; (*dest).y = src.y }
        1 => { (*dest).x = src.y; (*dest).y = width - src.x - src.width }
        2 => {
            (*dest).x = width - src.x - src.width;
            (*dest).y = height - src.y - src.height
        }
        3 => { (*dest).x = height - src.y - src.height; (*dest).y = src.x }
        4 => { (*dest).x = width - src.x - src.width; (*dest).y = src.y }
        5 => {
            (*dest).x = height - src.y - src.height;
            (*dest).y = width - src.x - src.width
        }
        6 => { (*dest).x = src.x; (*dest).y = height - src.y - src.height }
        7 => { (*dest).x = src.y; (*dest).y = src.x }
        _ => { }
    };
}
/* *
 * Creates the smallest box that contains the box rotated about its center.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_box_rotated_bounds(mut dest: *mut wlr_box,
                                                mut box_0: *const wlr_box,
                                                mut rotation: libc::c_float) {
    if rotation == 0i32 as libc::c_float { *dest = *box_0; return }
    let mut ox: libc::c_double =
        (*box_0).x as libc::c_double +
            (*box_0).width as libc::c_double / 2i32 as libc::c_double;
    let mut oy: libc::c_double =
        (*box_0).y as libc::c_double +
            (*box_0).height as libc::c_double / 2i32 as libc::c_double;
    let mut c: libc::c_double = fabs(cos(rotation as libc::c_double));
    let mut s: libc::c_double = fabs(sin(rotation as libc::c_double));
    let mut x1: libc::c_double =
        ox + ((*box_0).x as libc::c_double - ox) * c +
            ((*box_0).y as libc::c_double - oy) * s;
    let mut x2: libc::c_double =
        ox + (((*box_0).x + (*box_0).width) as libc::c_double - ox) * c +
            (((*box_0).y + (*box_0).height) as libc::c_double - oy) * s;
    let mut y1: libc::c_double =
        oy + ((*box_0).x as libc::c_double - ox) * s +
            ((*box_0).y as libc::c_double - oy) * c;
    let mut y2: libc::c_double =
        oy + (((*box_0).x + (*box_0).width) as libc::c_double - ox) * s +
            (((*box_0).y + (*box_0).height) as libc::c_double - oy) * c;
    (*dest).x = floor(fmin(x1, x2)) as libc::c_int;
    (*dest).width = ceil(fmax(x1, x2) - fmin(x1, x2)) as libc::c_int;
    (*dest).y = floor(fmin(y1, y2)) as libc::c_int;
    (*dest).height = ceil(fmax(y1, y2) - fmin(y1, y2)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_box_from_pixman_box32(mut dest: *mut wlr_box,
                                                   box_0: pixman_box32_t) {
    *dest =
        {
            let mut init =
                wlr_box{x: box_0.x1,
                        y: box_0.y1,
                        width: box_0.x2 - box_0.x1,
                        height: box_0.y2 - box_0.y1,};
            init
        };
}
