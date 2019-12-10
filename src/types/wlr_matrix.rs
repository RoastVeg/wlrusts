use libc;
extern "C" {
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn copysign(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
}
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
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn wlr_matrix_identity(mut mat: *mut libc::c_float) {
    static mut identity: [libc::c_float; 9] =
        [1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32,
         1.0f32];
    memcpy(mat as *mut libc::c_void, identity.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_float; 9]>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_matrix_multiply(mut mat: *mut libc::c_float,
                                             mut a: *const libc::c_float,
                                             mut b: *const libc::c_float) {
    let mut product: [libc::c_float; 9] = [0.; 9];
    product[0] =
        *a.offset(0) * *b.offset(0) + *a.offset(1) * *b.offset(3) +
            *a.offset(2) * *b.offset(6);
    product[1] =
        *a.offset(0) * *b.offset(1) + *a.offset(1) * *b.offset(4) +
            *a.offset(2) * *b.offset(7);
    product[2] =
        *a.offset(0) * *b.offset(2) + *a.offset(1) * *b.offset(5) +
            *a.offset(2) * *b.offset(8);
    product[3] =
        *a.offset(3) * *b.offset(0) + *a.offset(4) * *b.offset(3) +
            *a.offset(5) * *b.offset(6);
    product[4] =
        *a.offset(3) * *b.offset(1) + *a.offset(4) * *b.offset(4) +
            *a.offset(5) * *b.offset(7);
    product[5] =
        *a.offset(3) * *b.offset(2) + *a.offset(4) * *b.offset(5) +
            *a.offset(5) * *b.offset(8);
    product[6] =
        *a.offset(6) * *b.offset(0) + *a.offset(7) * *b.offset(3) +
            *a.offset(8) * *b.offset(6);
    product[7] =
        *a.offset(6) * *b.offset(1) + *a.offset(7) * *b.offset(4) +
            *a.offset(8) * *b.offset(7);
    product[8] =
        *a.offset(6) * *b.offset(2) + *a.offset(7) * *b.offset(5) +
            *a.offset(8) * *b.offset(8);
    memcpy(mat as *mut libc::c_void,
           product.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_float; 9]>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_matrix_transpose(mut mat: *mut libc::c_float,
                                              mut a: *const libc::c_float) {
    let mut transposition: [libc::c_float; 9] =
        [*a.offset(0), *a.offset(3), *a.offset(6), *a.offset(1), *a.offset(4),
         *a.offset(7), *a.offset(2), *a.offset(5), *a.offset(8)];
    memcpy(mat as *mut libc::c_void,
           transposition.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_float; 9]>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_matrix_translate(mut mat: *mut libc::c_float,
                                              mut x: libc::c_float,
                                              mut y: libc::c_float) {
    let mut translate: [libc::c_float; 9] =
        [1.0f32, 0.0f32, x, 0.0f32, 1.0f32, y, 0.0f32, 0.0f32, 1.0f32];
    wlr_matrix_multiply(mat, mat as *const libc::c_float,
                        translate.as_mut_ptr() as *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_matrix_scale(mut mat: *mut libc::c_float,
                                          mut x: libc::c_float,
                                          mut y: libc::c_float) {
    let mut scale: [libc::c_float; 9] =
        [x, 0.0f32, 0.0f32, 0.0f32, y, 0.0f32, 0.0f32, 0.0f32, 1.0f32];
    wlr_matrix_multiply(mat, mat as *const libc::c_float,
                        scale.as_mut_ptr() as *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn wlr_matrix_rotate(mut mat: *mut libc::c_float,
                                           mut rad: libc::c_float) {
    let mut rotate: [libc::c_float; 9] =
        [cos(rad as libc::c_double) as libc::c_float,
         -sin(rad as libc::c_double) as libc::c_float, 0.0f32,
         sin(rad as libc::c_double) as libc::c_float,
         cos(rad as libc::c_double) as libc::c_float, 0.0f32, 0.0f32, 0.0f32,
         1.0f32];
    wlr_matrix_multiply(mat, mat as *const libc::c_float,
                        rotate.as_mut_ptr() as *const libc::c_float);
}
static mut transforms: [[libc::c_float; 9]; 8] =
    [[1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32],
     [0.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32,
      1.0f32],
     [-1.0f32, 0.0f32, 0.0f32, 0.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32,
      1.0f32],
     [0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32,
      1.0f32],
     [-1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32,
      1.0f32],
     [0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32,
      1.0f32],
     [1.0f32, 0.0f32, 0.0f32, 0.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32,
      1.0f32],
     [0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32,
      1.0f32]];
#[no_mangle]
pub unsafe extern "C" fn wlr_matrix_transform(mut mat: *mut libc::c_float,
                                              mut transform:
                                                  wl_output_transform) {
    wlr_matrix_multiply(mat, mat as *const libc::c_float,
                        transforms[transform as usize].as_ptr());
}
// Equivalent to glOrtho(0, width, 0, height, 1, -1) with the transform applied
#[no_mangle]
pub unsafe extern "C" fn wlr_matrix_projection(mut mat: *mut libc::c_float,
                                               mut width: libc::c_int,
                                               mut height: libc::c_int,
                                               mut transform:
                                                   wl_output_transform) {
    memset(mat as *mut libc::c_void, 0i32,
           (::std::mem::size_of::<libc::c_float>() as
                libc::c_ulong).wrapping_mul(9i32 as libc::c_ulong));
    let mut t: *const libc::c_float = transforms[transform as usize].as_ptr();
    let mut x: libc::c_float = 2.0f32 / width as libc::c_float;
    let mut y: libc::c_float = 2.0f32 / height as libc::c_float;
    // Rotation + reflection
    *mat.offset(0) = x * *t.offset(0);
    *mat.offset(1) = x * *t.offset(1);
    *mat.offset(3) = y * -*t.offset(3);
    *mat.offset(4) = y * -*t.offset(4);
    // Translation
    *mat.offset(2) =
        -copysign(1.0f32 as libc::c_double,
                  (*mat.offset(0) + *mat.offset(1)) as libc::c_double) as
            libc::c_float;
    *mat.offset(5) =
        -copysign(1.0f32 as libc::c_double,
                  (*mat.offset(3) + *mat.offset(4)) as libc::c_double) as
            libc::c_float;
    // Identity
    *mat.offset(8) = 1.0f32;
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
/* * Writes the identity matrix into mat */
/* * mat ← a × b */
/* * Writes a 2D translation matrix to mat of magnitude (x, y) */
/* * Writes a 2D scale matrix to mat of magnitude (x, y) */
/* * Writes a 2D rotation matrix to mat at an angle of rad radians */
/* * Writes a transformation matrix which applies the specified
 *  wl_output_transform to mat */
/* * Writes a 2D orthographic projection matrix to mat of (width, height) with a
 *  specified wl_output_transform*/
/* * Shortcut for the various matrix operations involved in projecting the
 *  specified wlr_box onto a given orthographic projection with a given
 *  rotation. The result is written to mat, which can be applied to each
 *  coordinate of the box to get a new coordinate from [-1,1]. */
#[no_mangle]
pub unsafe extern "C" fn wlr_matrix_project_box(mut mat: *mut libc::c_float,
                                                mut box_0: *const wlr_box,
                                                mut transform:
                                                    wl_output_transform,
                                                mut rotation: libc::c_float,
                                                mut projection:
                                                    *const libc::c_float) {
    let mut x: libc::c_int = (*box_0).x;
    let mut y: libc::c_int = (*box_0).y;
    let mut width: libc::c_int = (*box_0).width;
    let mut height: libc::c_int = (*box_0).height;
    wlr_matrix_identity(mat);
    wlr_matrix_translate(mat, x as libc::c_float, y as libc::c_float);
    if rotation != 0i32 as libc::c_float {
        wlr_matrix_translate(mat, (width / 2i32) as libc::c_float,
                             (height / 2i32) as libc::c_float);
        wlr_matrix_rotate(mat, rotation);
        wlr_matrix_translate(mat, (-width / 2i32) as libc::c_float,
                             (-height / 2i32) as libc::c_float);
    }
    wlr_matrix_scale(mat, width as libc::c_float, height as libc::c_float);
    if transform as libc::c_uint !=
           WL_OUTPUT_TRANSFORM_NORMAL as libc::c_int as libc::c_uint {
        wlr_matrix_translate(mat, 0.5f64 as libc::c_float,
                             0.5f64 as libc::c_float);
        wlr_matrix_transform(mat, transform);
        wlr_matrix_translate(mat, -0.5f64 as libc::c_float,
                             -0.5f64 as libc::c_float);
    }
    wlr_matrix_multiply(mat, projection, mat as *const libc::c_float);
}
