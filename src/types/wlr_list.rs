use libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
pub type ssize_t = __ssize_t;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_list {
    pub capacity: size_t,
    pub length: size_t,
    pub items: *mut *mut libc::c_void,
}
/* *
 * Initialize a list. Returns true on success, false on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_init(mut list: *mut wlr_list) -> bool {
    (*list).capacity = 10i32 as size_t;
    (*list).length = 0i32 as size_t;
    (*list).items =
        malloc((::std::mem::size_of::<*mut libc::c_void>() as
                    libc::c_ulong).wrapping_mul((*list).capacity)) as
            *mut *mut libc::c_void;
    if (*list).items.is_null() { return 0i32 != 0 }
    return 1i32 != 0;
}
unsafe extern "C" fn list_resize(mut list: *mut wlr_list) -> bool {
    if (*list).length == (*list).capacity {
        let mut new_items: *mut libc::c_void =
            realloc((*list).items as *mut libc::c_void,
                    (::std::mem::size_of::<*mut libc::c_void>() as
                         libc::c_ulong).wrapping_mul((*list).capacity.wrapping_add(10i32
                                                                                       as
                                                                                       libc::c_ulong)));
        if new_items.is_null() { return 0i32 != 0 }
        (*list).capacity =
            ((*list).capacity as
                 libc::c_ulong).wrapping_add(10i32 as libc::c_ulong) as size_t
                as size_t;
        (*list).items = new_items as *mut *mut libc::c_void
    }
    return 1i32 != 0;
}
/* *
 * Deinitialize a list.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_finish(mut list: *mut wlr_list) {
    free((*list).items as *mut libc::c_void);
}
/* *
 * Executes `callback` on each element in the list.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_for_each(mut list: *mut wlr_list,
                                           mut callback:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut libc::c_void)
                                                          -> ()>) {
    let mut i: size_t = 0i32 as size_t;
    while i < (*list).length {
        callback.expect("non-null function pointer")(*(*list).items.offset(i
                                                                               as
                                                                               isize));
        i = i.wrapping_add(1)
    };
}
/* *
 * Add `item` to the end of a list.
 * Returns: new list length or `-1` on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_push(mut list: *mut wlr_list,
                                       mut item: *mut libc::c_void)
 -> ssize_t {
    if !list_resize(list) { return -1i32 as ssize_t }
    let fresh0 = (*list).length;
    (*list).length = (*list).length.wrapping_add(1);
    let ref mut fresh1 = *(*list).items.offset(fresh0 as isize);
    *fresh1 = item;
    return (*list).length as ssize_t;
}
/* *
 * Place `item` into index `index` in the list.
 * Returns: new list length or `-1` on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_insert(mut list: *mut wlr_list,
                                         mut index: size_t,
                                         mut item: *mut libc::c_void)
 -> ssize_t {
    if !list_resize(list) { return -1i32 as ssize_t }
    memmove(&mut *(*list).items.offset(index.wrapping_add(1i32 as
                                                              libc::c_ulong)
                                           as isize) as *mut *mut libc::c_void
                as *mut libc::c_void,
            &mut *(*list).items.offset(index as isize) as
                *mut *mut libc::c_void as *const libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as
                 libc::c_ulong).wrapping_mul((*list).length.wrapping_sub(index)));
    (*list).length = (*list).length.wrapping_add(1);
    let ref mut fresh2 = *(*list).items.offset(index as isize);
    *fresh2 = item;
    return (*list).length as ssize_t;
}
/* *
 * Remove an item from the list.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_del(mut list: *mut wlr_list,
                                      mut index: size_t) {
    (*list).length = (*list).length.wrapping_sub(1);
    memmove(&mut *(*list).items.offset(index as isize) as
                *mut *mut libc::c_void as *mut libc::c_void,
            &mut *(*list).items.offset(index.wrapping_add(1i32 as
                                                              libc::c_ulong)
                                           as isize) as *mut *mut libc::c_void
                as *const libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as
                 libc::c_ulong).wrapping_mul((*list).length.wrapping_sub(index)));
}
/* *
 * Remove and return an item from the end of the list.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_pop(mut list: *mut wlr_list)
 -> *mut libc::c_void {
    if (*list).length == 0i32 as libc::c_ulong {
        return 0 as *mut libc::c_void
    }
    let mut last: *mut libc::c_void =
        *(*list).items.offset((*list).length.wrapping_sub(1i32 as
                                                              libc::c_ulong)
                                  as isize);
    wlr_list_del(list, (*list).length.wrapping_sub(1i32 as libc::c_ulong));
    return last;
}
/* *
 * Get a reference to the last item of a list without removal.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_peek(mut list: *mut wlr_list)
 -> *mut libc::c_void {
    if (*list).length == 0i32 as libc::c_ulong {
        return 0 as *mut libc::c_void
    }
    return *(*list).items.offset((*list).length.wrapping_sub(1i32 as
                                                                 libc::c_ulong)
                                     as isize);
}
/* *
 * Append each item in `source` to `list`.
 * Does not modify `source`.
 * Returns: new list length or `-1` on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_cat(mut list: *mut wlr_list,
                                      mut source: *const wlr_list)
 -> ssize_t {
    let mut old_len: size_t = (*list).length;
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i < (*source).length {
        if wlr_list_push(list, *(*source).items.offset(i as isize)) ==
               -1i32 as libc::c_long {
            (*list).length = old_len;
            return -1i32 as ssize_t
        }
        i = i.wrapping_add(1)
    }
    return (*list).length as ssize_t;
}
/* *
 * Sort a list using `qsort`.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_qsort(mut list: *mut wlr_list,
                                        mut compare:
                                            Option<unsafe extern "C" fn(_:
                                                                            *const libc::c_void,
                                                                        _:
                                                                            *const libc::c_void)
                                                       -> libc::c_int>) {
    qsort((*list).items as *mut libc::c_void, (*list).length,
          ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
          compare);
}
/* *
 * Return the index of the first item in the list that returns 0 for the given
 * `compare` function, or -1 if none matches.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_list_find(mut list: *mut wlr_list,
                                       mut compare:
                                           Option<unsafe extern "C" fn(_:
                                                                           *const libc::c_void,
                                                                       _:
                                                                           *const libc::c_void)
                                                      -> libc::c_int>,
                                       mut data: *const libc::c_void)
 -> ssize_t {
    let mut i: size_t = 0i32 as size_t;
    while i < (*list).length {
        let mut item: *mut libc::c_void = *(*list).items.offset(i as isize);
        if compare.expect("non-null function pointer")(item, data) == 0i32 {
            return i as ssize_t
        }
        i = i.wrapping_add(1)
    }
    return -1i32 as ssize_t;
}
