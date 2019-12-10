use libc;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
// https://www.geeksforgeeks.org/move-zeroes-end-array/
pub unsafe fn push_zeroes_to_end(mut arr: *mut uint32_t,
                                 mut n: size_t) -> size_t {
    let mut count: size_t = 0i32 as size_t;
    let mut i: size_t = 0i32 as size_t;
    while i < n {
        if *arr.offset(i as isize) != 0i32 as libc::c_uint {
            let fresh0 = count;
            count = count.wrapping_add(1);
            *arr.offset(fresh0 as isize) = *arr.offset(i as isize)
        }
        i = i.wrapping_add(1)
    }
    let mut ret: size_t = count;
    while count < n {
        let fresh1 = count;
        count = count.wrapping_add(1);
        *arr.offset(fresh1 as isize) = 0i32 as uint32_t
    }
    return ret;
}
pub unsafe fn set_add(mut values: *mut uint32_t,
                      mut len: *mut size_t, mut cap: size_t,
                      mut target: uint32_t) -> bool {
    if *len == cap { return 0i32 != 0 }
    let mut i: uint32_t = 0i32 as uint32_t;
    while (i as libc::c_ulong) < *len {
        if *values.offset(i as isize) == target { return 0i32 != 0 }
        i = i.wrapping_add(1)
    }
    let fresh2 = *len;
    *len = (*len).wrapping_add(1);
    *values.offset(fresh2 as isize) = target;
    return 0i32 != 0;
}
/* *
 * Add `target` to `values` if it doesn't exist
 * "set"s should only be modified with set_* functions
 * Values MUST be greater than 0
 */
/* *
 * Remove `target` from `values` if it exists
 * "set"s should only be modified with set_* functions
 * Values MUST be greater than 0
 */
pub unsafe fn set_remove(mut values: *mut uint32_t,
                         mut len: *mut size_t, mut cap: size_t,
                         mut target: uint32_t) -> bool {
    let mut i: uint32_t = 0i32 as uint32_t;
    while (i as libc::c_ulong) < *len {
        if *values.offset(i as isize) == target {
            // Set to 0 and swap with the end element so that
			// zeroes exist only after all the values.
            *len = (*len).wrapping_sub(1);
            let mut last_elem_pos: size_t = *len;
            *values.offset(i as isize) =
                *values.offset(last_elem_pos as isize);
            *values.offset(last_elem_pos as isize) = 0i32 as uint32_t;
            return 1i32 != 0
        }
        i = i.wrapping_add(1)
    }
    return 0i32 != 0;
}
