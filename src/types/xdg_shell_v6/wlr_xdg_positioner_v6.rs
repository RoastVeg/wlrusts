use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_client;
    pub type wl_global;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static zxdg_positioner_v6_interface: wl_interface;
    #[no_mangle]
    fn wl_client_post_no_memory(client: *mut wl_client);
    #[no_mangle]
    fn wl_resource_post_error(resource: *mut wl_resource, code: uint32_t,
                              msg: *const libc::c_char, _: ...);
    #[no_mangle]
    fn wl_resource_create(client: *mut wl_client,
                          interface: *const wl_interface,
                          version: libc::c_int, id: uint32_t)
     -> *mut wl_resource;
    #[no_mangle]
    fn wl_resource_set_implementation(resource: *mut wl_resource,
                                      implementation: *const libc::c_void,
                                      data: *mut libc::c_void,
                                      destroy: wl_resource_destroy_func_t);
    #[no_mangle]
    fn wl_resource_destroy(resource: *mut wl_resource);
    #[no_mangle]
    fn wl_resource_get_user_data(resource: *mut wl_resource)
     -> *mut libc::c_void;
    #[no_mangle]
    fn wl_resource_get_version(resource: *mut wl_resource) -> libc::c_int;
    #[no_mangle]
    fn wl_resource_instance_of(resource: *mut wl_resource,
                               interface: *const wl_interface,
                               implementation: *const libc::c_void)
     -> libc::c_int;
}
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
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_box {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type zxdg_positioner_v6_error = libc::c_uint;
pub const ZXDG_POSITIONER_V6_ERROR_INVALID_INPUT: zxdg_positioner_v6_error =
    0;
pub type zxdg_positioner_v6_anchor = libc::c_uint;
pub const ZXDG_POSITIONER_V6_ANCHOR_RIGHT: zxdg_positioner_v6_anchor = 8;
pub const ZXDG_POSITIONER_V6_ANCHOR_LEFT: zxdg_positioner_v6_anchor = 4;
pub const ZXDG_POSITIONER_V6_ANCHOR_BOTTOM: zxdg_positioner_v6_anchor = 2;
pub const ZXDG_POSITIONER_V6_ANCHOR_TOP: zxdg_positioner_v6_anchor = 1;
pub const ZXDG_POSITIONER_V6_ANCHOR_NONE: zxdg_positioner_v6_anchor = 0;
pub type zxdg_positioner_v6_gravity = libc::c_uint;
pub const ZXDG_POSITIONER_V6_GRAVITY_RIGHT: zxdg_positioner_v6_gravity = 8;
pub const ZXDG_POSITIONER_V6_GRAVITY_LEFT: zxdg_positioner_v6_gravity = 4;
pub const ZXDG_POSITIONER_V6_GRAVITY_BOTTOM: zxdg_positioner_v6_gravity = 2;
pub const ZXDG_POSITIONER_V6_GRAVITY_TOP: zxdg_positioner_v6_gravity = 1;
pub const ZXDG_POSITIONER_V6_GRAVITY_NONE: zxdg_positioner_v6_gravity = 0;
pub type zxdg_positioner_v6_constraint_adjustment = libc::c_uint;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_RESIZE_Y:
          zxdg_positioner_v6_constraint_adjustment =
    32;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_RESIZE_X:
          zxdg_positioner_v6_constraint_adjustment =
    16;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_FLIP_Y:
          zxdg_positioner_v6_constraint_adjustment =
    8;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_FLIP_X:
          zxdg_positioner_v6_constraint_adjustment =
    4;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_SLIDE_Y:
          zxdg_positioner_v6_constraint_adjustment =
    2;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_SLIDE_X:
          zxdg_positioner_v6_constraint_adjustment =
    1;
pub const ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_NONE:
          zxdg_positioner_v6_constraint_adjustment =
    0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct zxdg_positioner_v6_interface {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wl_client,
                                             _: *mut wl_resource) -> ()>,
    pub set_size: Option<unsafe extern "C" fn(_: *mut wl_client,
                                              _: *mut wl_resource, _: int32_t,
                                              _: int32_t) -> ()>,
    pub set_anchor_rect: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                     _: *mut wl_resource,
                                                     _: int32_t, _: int32_t,
                                                     _: int32_t, _: int32_t)
                                    -> ()>,
    pub set_anchor: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                _: *mut wl_resource,
                                                _: uint32_t) -> ()>,
    pub set_gravity: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                 _: *mut wl_resource,
                                                 _: uint32_t) -> ()>,
    pub set_constraint_adjustment: Option<unsafe extern "C" fn(_:
                                                                   *mut wl_client,
                                                               _:
                                                                   *mut wl_resource,
                                                               _: uint32_t)
                                              -> ()>,
    pub set_offset: Option<unsafe extern "C" fn(_: *mut wl_client,
                                                _: *mut wl_resource,
                                                _: int32_t, _: int32_t)
                               -> ()>,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_shell_v6 {
    pub global: *mut wl_global,
    pub clients: wl_list,
    pub popup_grabs: wl_list,
    pub ping_timeout: uint32_t,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_client_v6 {
    pub shell: *mut wlr_xdg_shell_v6,
    pub resource: *mut wl_resource,
    pub client: *mut wl_client,
    pub surfaces: wl_list,
    pub link: wl_list,
    pub ping_serial: uint32_t,
    pub ping_timer: *mut wl_event_source,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_positioner_v6 {
    pub anchor_rect: wlr_box,
    pub anchor: zxdg_positioner_v6_anchor,
    pub gravity: zxdg_positioner_v6_gravity,
    pub constraint_adjustment: zxdg_positioner_v6_constraint_adjustment,
    pub size: C2RustUnnamed_1,
    pub offset: C2RustUnnamed_0,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub x: int32_t,
    pub y: int32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub width: int32_t,
    pub height: int32_t,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_xdg_positioner_v6_resource {
    pub resource: *mut wl_resource,
    pub attrs: wlr_xdg_positioner_v6,
}
#[no_mangle]
pub unsafe extern "C" fn get_xdg_positioner_v6_from_resource(mut resource:
                                                                 *mut wl_resource)
 -> *mut wlr_xdg_positioner_v6_resource {
    if wl_resource_instance_of(resource, &zxdg_positioner_v6_interface,
                               &zxdg_positioner_v6_implementation as
                                   *const zxdg_positioner_v6_interface as
                                   *const libc::c_void) != 0 {
    } else {
        __assert_fail(b"wl_resource_instance_of(resource, &zxdg_positioner_v6_interface, &zxdg_positioner_v6_implementation)\x00"
                          as *const u8 as *const libc::c_char,
                      b"../types/xdg_shell_v6/wlr_xdg_positioner_v6.c\x00" as
                          *const u8 as *const libc::c_char,
                      11i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 97],
                                                &[libc::c_char; 97]>(b"struct wlr_xdg_positioner_v6_resource *get_xdg_positioner_v6_from_resource(struct wl_resource *)\x00")).as_ptr());
    };
    return wl_resource_get_user_data(resource) as
               *mut wlr_xdg_positioner_v6_resource;
}
unsafe extern "C" fn xdg_positioner_destroy(mut resource: *mut wl_resource) {
    let mut positioner: *mut wlr_xdg_positioner_v6_resource =
        get_xdg_positioner_v6_from_resource(resource);
    free(positioner as *mut libc::c_void);
}
unsafe extern "C" fn xdg_positioner_handle_destroy(mut client: *mut wl_client,
                                                   mut resource:
                                                       *mut wl_resource) {
    wl_resource_destroy(resource);
}
unsafe extern "C" fn xdg_positioner_handle_set_size(mut client:
                                                        *mut wl_client,
                                                    mut resource:
                                                        *mut wl_resource,
                                                    mut width: int32_t,
                                                    mut height: int32_t) {
    let mut positioner: *mut wlr_xdg_positioner_v6_resource =
        get_xdg_positioner_v6_from_resource(resource);
    if width < 1i32 || height < 1i32 {
        wl_resource_post_error(resource,
                               ZXDG_POSITIONER_V6_ERROR_INVALID_INPUT as
                                   libc::c_int as uint32_t,
                               b"width and height must be positive and non-zero\x00"
                                   as *const u8 as *const libc::c_char);
        return
    }
    (*positioner).attrs.size.width = width;
    (*positioner).attrs.size.height = height;
}
unsafe extern "C" fn xdg_positioner_handle_set_anchor_rect(mut client:
                                                               *mut wl_client,
                                                           mut resource:
                                                               *mut wl_resource,
                                                           mut x: int32_t,
                                                           mut y: int32_t,
                                                           mut width: int32_t,
                                                           mut height:
                                                               int32_t) {
    let mut positioner: *mut wlr_xdg_positioner_v6_resource =
        get_xdg_positioner_v6_from_resource(resource);
    if width < 1i32 || height < 1i32 {
        wl_resource_post_error(resource,
                               ZXDG_POSITIONER_V6_ERROR_INVALID_INPUT as
                                   libc::c_int as uint32_t,
                               b"width and height must be positive and non-zero\x00"
                                   as *const u8 as *const libc::c_char);
        return
    }
    (*positioner).attrs.anchor_rect.x = x;
    (*positioner).attrs.anchor_rect.y = y;
    (*positioner).attrs.anchor_rect.width = width;
    (*positioner).attrs.anchor_rect.height = height;
}
unsafe extern "C" fn xdg_positioner_handle_set_anchor(mut client:
                                                          *mut wl_client,
                                                      mut resource:
                                                          *mut wl_resource,
                                                      mut anchor: uint32_t) {
    let mut positioner: *mut wlr_xdg_positioner_v6_resource =
        get_xdg_positioner_v6_from_resource(resource);
    if anchor & ZXDG_POSITIONER_V6_ANCHOR_TOP as libc::c_int as libc::c_uint
           != 0 &&
           anchor &
               ZXDG_POSITIONER_V6_ANCHOR_BOTTOM as libc::c_int as libc::c_uint
               != 0 ||
           anchor &
               ZXDG_POSITIONER_V6_ANCHOR_LEFT as libc::c_int as libc::c_uint
               != 0 &&
               anchor &
                   ZXDG_POSITIONER_V6_ANCHOR_RIGHT as libc::c_int as
                       libc::c_uint != 0 {
        wl_resource_post_error(resource,
                               ZXDG_POSITIONER_V6_ERROR_INVALID_INPUT as
                                   libc::c_int as uint32_t,
                               b"same-axis values are not allowed\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    (*positioner).attrs.anchor = anchor as zxdg_positioner_v6_anchor;
}
unsafe extern "C" fn xdg_positioner_handle_set_gravity(mut client:
                                                           *mut wl_client,
                                                       mut resource:
                                                           *mut wl_resource,
                                                       mut gravity:
                                                           uint32_t) {
    let mut positioner: *mut wlr_xdg_positioner_v6_resource =
        get_xdg_positioner_v6_from_resource(resource);
    if gravity & ZXDG_POSITIONER_V6_GRAVITY_TOP as libc::c_int as libc::c_uint
           != 0 &&
           gravity &
               ZXDG_POSITIONER_V6_GRAVITY_BOTTOM as libc::c_int as
                   libc::c_uint != 0 ||
           gravity &
               ZXDG_POSITIONER_V6_GRAVITY_LEFT as libc::c_int as libc::c_uint
               != 0 &&
               gravity &
                   ZXDG_POSITIONER_V6_GRAVITY_RIGHT as libc::c_int as
                       libc::c_uint != 0 {
        wl_resource_post_error(resource,
                               ZXDG_POSITIONER_V6_ERROR_INVALID_INPUT as
                                   libc::c_int as uint32_t,
                               b"same-axis values are not allowed\x00" as
                                   *const u8 as *const libc::c_char);
        return
    }
    (*positioner).attrs.gravity = gravity as zxdg_positioner_v6_gravity;
}
unsafe extern "C" fn xdg_positioner_handle_set_constraint_adjustment(mut client:
                                                                         *mut wl_client,
                                                                     mut resource:
                                                                         *mut wl_resource,
                                                                     mut constraint_adjustment:
                                                                         uint32_t) {
    let mut positioner: *mut wlr_xdg_positioner_v6_resource =
        get_xdg_positioner_v6_from_resource(resource);
    (*positioner).attrs.constraint_adjustment =
        constraint_adjustment as zxdg_positioner_v6_constraint_adjustment;
}
unsafe extern "C" fn xdg_positioner_handle_set_offset(mut client:
                                                          *mut wl_client,
                                                      mut resource:
                                                          *mut wl_resource,
                                                      mut x: int32_t,
                                                      mut y: int32_t) {
    let mut positioner: *mut wlr_xdg_positioner_v6_resource =
        get_xdg_positioner_v6_from_resource(resource);
    (*positioner).attrs.offset.x = x;
    (*positioner).attrs.offset.y = y;
}
static mut zxdg_positioner_v6_implementation: zxdg_positioner_v6_interface =
    {
    
        {
            let mut init =
                zxdg_positioner_v6_interface{destroy:
                                                 Some(xdg_positioner_handle_destroy
                                                          as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut wl_client,
                                                                               _:
                                                                                   *mut wl_resource)
                                                              -> ()),
                                             set_size:
                                                 Some(xdg_positioner_handle_set_size
                                                          as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut wl_client,
                                                                               _:
                                                                                   *mut wl_resource,
                                                                               _:
                                                                                   int32_t,
                                                                               _:
                                                                                   int32_t)
                                                              -> ()),
                                             set_anchor_rect:
                                                 Some(xdg_positioner_handle_set_anchor_rect
                                                          as
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
                                             set_anchor:
                                                 Some(xdg_positioner_handle_set_anchor
                                                          as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut wl_client,
                                                                               _:
                                                                                   *mut wl_resource,
                                                                               _:
                                                                                   uint32_t)
                                                              -> ()),
                                             set_gravity:
                                                 Some(xdg_positioner_handle_set_gravity
                                                          as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut wl_client,
                                                                               _:
                                                                                   *mut wl_resource,
                                                                               _:
                                                                                   uint32_t)
                                                              -> ()),
                                             set_constraint_adjustment:
                                                 Some(xdg_positioner_handle_set_constraint_adjustment
                                                          as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut wl_client,
                                                                               _:
                                                                                   *mut wl_resource,
                                                                               _:
                                                                                   uint32_t)
                                                              -> ()),
                                             set_offset:
                                                 Some(xdg_positioner_handle_set_offset
                                                          as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut wl_client,
                                                                               _:
                                                                                   *mut wl_resource,
                                                                               _:
                                                                                   int32_t,
                                                                               _:
                                                                                   int32_t)
                                                              -> ()),};
            init
        }
};
#[no_mangle]
pub unsafe extern "C" fn wlr_xdg_positioner_v6_get_geometry(mut positioner:
                                                                *mut wlr_xdg_positioner_v6)
 -> wlr_box {
    let mut geometry: wlr_box =
        {
            let mut init =
                wlr_box{x: (*positioner).offset.x,
                        y: (*positioner).offset.y,
                        width: (*positioner).size.width,
                        height: (*positioner).size.height,};
            init
        };
    if (*positioner).anchor as libc::c_uint &
           ZXDG_POSITIONER_V6_ANCHOR_TOP as libc::c_int as libc::c_uint != 0 {
        geometry.y += (*positioner).anchor_rect.y
    } else if (*positioner).anchor as libc::c_uint &
                  ZXDG_POSITIONER_V6_ANCHOR_BOTTOM as libc::c_int as
                      libc::c_uint != 0 {
        geometry.y +=
            (*positioner).anchor_rect.y + (*positioner).anchor_rect.height
    } else {
        geometry.y +=
            (*positioner).anchor_rect.y +
                (*positioner).anchor_rect.height / 2i32
    }
    if (*positioner).anchor as libc::c_uint &
           ZXDG_POSITIONER_V6_ANCHOR_LEFT as libc::c_int as libc::c_uint != 0
       {
        geometry.x += (*positioner).anchor_rect.x
    } else if (*positioner).anchor as libc::c_uint &
                  ZXDG_POSITIONER_V6_ANCHOR_RIGHT as libc::c_int as
                      libc::c_uint != 0 {
        geometry.x +=
            (*positioner).anchor_rect.x + (*positioner).anchor_rect.width
    } else {
        geometry.x +=
            (*positioner).anchor_rect.x +
                (*positioner).anchor_rect.width / 2i32
    }
    if (*positioner).gravity as libc::c_uint &
           ZXDG_POSITIONER_V6_GRAVITY_TOP as libc::c_int as libc::c_uint != 0
       {
        geometry.y -= geometry.height
    } else if (*positioner).gravity as libc::c_uint &
                  ZXDG_POSITIONER_V6_GRAVITY_BOTTOM as libc::c_int as
                      libc::c_uint == 0 {
        geometry.y -= geometry.height / 2i32
    }
    if (*positioner).gravity as libc::c_uint &
           ZXDG_POSITIONER_V6_GRAVITY_LEFT as libc::c_int as libc::c_uint != 0
       {
        geometry.x -= geometry.width
    } else if (*positioner).gravity as libc::c_uint &
                  ZXDG_POSITIONER_V6_GRAVITY_RIGHT as libc::c_int as
                      libc::c_uint == 0 {
        geometry.x -= geometry.width / 2i32
    }
    if (*positioner).constraint_adjustment as libc::c_uint ==
           ZXDG_POSITIONER_V6_CONSTRAINT_ADJUSTMENT_NONE as libc::c_int as
               libc::c_uint {
        return geometry
    }
    return geometry;
}
#[no_mangle]
pub unsafe extern "C" fn wlr_positioner_v6_invert_x(mut positioner:
                                                        *mut wlr_xdg_positioner_v6) {
    if (*positioner).anchor as libc::c_uint &
           ZXDG_POSITIONER_V6_ANCHOR_LEFT as libc::c_int as libc::c_uint != 0
       {
        (*positioner).anchor =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_anchor>((*positioner).anchor
                                                                   as
                                                                   libc::c_uint
                                                                   &
                                                                   !(ZXDG_POSITIONER_V6_ANCHOR_LEFT
                                                                         as
                                                                         libc::c_int)
                                                                       as
                                                                       libc::c_uint);
        (*positioner).anchor =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_anchor>((*positioner).anchor
                                                                   as
                                                                   libc::c_uint
                                                                   |
                                                                   ZXDG_POSITIONER_V6_ANCHOR_RIGHT
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
    } else if (*positioner).anchor as libc::c_uint &
                  ZXDG_POSITIONER_V6_ANCHOR_RIGHT as libc::c_int as
                      libc::c_uint != 0 {
        (*positioner).anchor =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_anchor>((*positioner).anchor
                                                                   as
                                                                   libc::c_uint
                                                                   &
                                                                   !(ZXDG_POSITIONER_V6_ANCHOR_RIGHT
                                                                         as
                                                                         libc::c_int)
                                                                       as
                                                                       libc::c_uint);
        (*positioner).anchor =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_anchor>((*positioner).anchor
                                                                   as
                                                                   libc::c_uint
                                                                   |
                                                                   ZXDG_POSITIONER_V6_ANCHOR_LEFT
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
    }
    if (*positioner).gravity as libc::c_uint &
           ZXDG_POSITIONER_V6_GRAVITY_RIGHT as libc::c_int as libc::c_uint !=
           0 {
        (*positioner).gravity =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_gravity>((*positioner).gravity
                                                                    as
                                                                    libc::c_uint
                                                                    &
                                                                    !(ZXDG_POSITIONER_V6_GRAVITY_RIGHT
                                                                          as
                                                                          libc::c_int)
                                                                        as
                                                                        libc::c_uint);
        (*positioner).gravity =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_gravity>((*positioner).gravity
                                                                    as
                                                                    libc::c_uint
                                                                    |
                                                                    ZXDG_POSITIONER_V6_GRAVITY_LEFT
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
    } else if (*positioner).gravity as libc::c_uint &
                  ZXDG_POSITIONER_V6_GRAVITY_LEFT as libc::c_int as
                      libc::c_uint != 0 {
        (*positioner).gravity =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_gravity>((*positioner).gravity
                                                                    as
                                                                    libc::c_uint
                                                                    &
                                                                    !(ZXDG_POSITIONER_V6_GRAVITY_LEFT
                                                                          as
                                                                          libc::c_int)
                                                                        as
                                                                        libc::c_uint);
        (*positioner).gravity =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_gravity>((*positioner).gravity
                                                                    as
                                                                    libc::c_uint
                                                                    |
                                                                    ZXDG_POSITIONER_V6_GRAVITY_RIGHT
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
    };
}
/*
 * This protocol is obsolete and will be removed in a future version. The
 * recommended replacement is xdg-shell.
 */
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * An interface enabling clients to turn their wl_surfaces into windows in a
 * desktop environment.
 */
/* *
		 * The `new_surface` event signals that a client has requested to
		 * create a new shell surface. At this point, the surface is ready to
		 * be configured but is not mapped or ready receive input events. The
		 * surface will be ready to be managed on the `map` event.
		 */
// wlr_xdg_shell_v6::clients
// Position of the popup relative to the upper left corner of the window
	// geometry of the parent surface
// wlr_xdg_popup_grab_v6::popups
// each seat gets a popup grab
// wlr_xdg_shell_v6::popup_grabs
// Since the fullscreen request may be made before the toplevel's surface
	// is mapped, this is used to store the requested fullscreen output (if
	// any) for wlr_xdg_toplevel_v6::client_pending.
/* *
 * An xdg-surface is a user interface element requiring management by the
 * compositor. An xdg-surface alone isn't useful, a role should be assigned to
 * it in order to map it.
 *
 * When a surface has a role and is ready to be displayed, the `map` event is
 * emitted. When a surface should no longer be displayed, the `unmap` event is
 * emitted. The `unmap` event is guaranteed to be emitted before the `destroy`
 * event if the view is destroyed when mapped.
 */
// wlr_xdg_surface_v6::configure_list
// wlr_xdg_client_v6::surfaces
// wlr_xdg_popup_v6::link
/* *
		 * The `map` event signals that the shell surface is ready to be
		 * managed by the compositor and rendered on the screen. At this point,
		 * the surface has configured its properties, has had the opportunity
		 * to bind to the seat to receive input events, and has a buffer that
		 * is ready to be rendered. You can now safely add this surface to a
		 * list of views.
		 */
/* *
		 * The `unmap` event signals that the surface is no longer in a state
		 * where it should be shown on the screen. This might happen if the
		 * surface no longer has a displayable buffer because either the
		 * surface has been hidden or is about to be destroyed.
		 */
/* *
 * Send a ping to the surface. If the surface does not respond in a reasonable
 * amount of time, the ping_timeout event will be emitted.
 */
/* *
 * Request that this toplevel surface be the given size. Returns the associated
 * configure serial.
 */
/* *
 * Request that this toplevel surface show itself in an activated or deactivated
 * state. Returns the associated configure serial.
 */
/* *
 * Request that this toplevel surface consider itself maximized or not
 * maximized. Returns the associated configure serial.
 */
/* *
 * Request that this toplevel surface consider itself fullscreen or not
 * fullscreen. Returns the associated configure serial.
 */
/* *
 * Request that this toplevel surface consider itself to be resizing or not
 * resizing. Returns the associated configure serial.
 */
/* *
 * Request that this xdg surface closes.
 */
/* *
 * Find a surface within this xdg-surface tree at the given surface-local
 * coordinates. Returns the surface and coordinates in the leaf surface
 * coordinate system or NULL if no surface is found at that location.
 */
/* *
 * Get the geometry for this positioner based on the anchor rect, gravity, and
 * size of this positioner.
 */
/* *
 * Get the anchor point for this popup in the toplevel parent's coordinate system.
 */
/* *
 * Convert the given coordinates in the popup coordinate system to the toplevel
 * surface coordinate system.
 */
/* *
 * Set the geometry of this popup to unconstrain it according to its
 * xdg-positioner rules. The box should be in the popup's root toplevel parent
 * surface coordinate system.
 */
/* *
  Invert the right/left anchor and gravity for this positioner. This can be
  used to "flip" the positioner around the anchor rect in the x direction.
 */
/* *
  Invert the top/bottom anchor and gravity for this positioner. This can be
  used to "flip" the positioner around the anchor rect in the y direction.
 */
#[no_mangle]
pub unsafe extern "C" fn wlr_positioner_v6_invert_y(mut positioner:
                                                        *mut wlr_xdg_positioner_v6) {
    if (*positioner).anchor as libc::c_uint &
           ZXDG_POSITIONER_V6_ANCHOR_TOP as libc::c_int as libc::c_uint != 0 {
        (*positioner).anchor =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_anchor>((*positioner).anchor
                                                                   as
                                                                   libc::c_uint
                                                                   &
                                                                   !(ZXDG_POSITIONER_V6_ANCHOR_TOP
                                                                         as
                                                                         libc::c_int)
                                                                       as
                                                                       libc::c_uint);
        (*positioner).anchor =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_anchor>((*positioner).anchor
                                                                   as
                                                                   libc::c_uint
                                                                   |
                                                                   ZXDG_POSITIONER_V6_ANCHOR_BOTTOM
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
    } else if (*positioner).anchor as libc::c_uint &
                  ZXDG_POSITIONER_V6_ANCHOR_BOTTOM as libc::c_int as
                      libc::c_uint != 0 {
        (*positioner).anchor =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_anchor>((*positioner).anchor
                                                                   as
                                                                   libc::c_uint
                                                                   &
                                                                   !(ZXDG_POSITIONER_V6_ANCHOR_BOTTOM
                                                                         as
                                                                         libc::c_int)
                                                                       as
                                                                       libc::c_uint);
        (*positioner).anchor =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_anchor>((*positioner).anchor
                                                                   as
                                                                   libc::c_uint
                                                                   |
                                                                   ZXDG_POSITIONER_V6_ANCHOR_TOP
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
    }
    if (*positioner).gravity as libc::c_uint &
           ZXDG_POSITIONER_V6_GRAVITY_TOP as libc::c_int as libc::c_uint != 0
       {
        (*positioner).gravity =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_gravity>((*positioner).gravity
                                                                    as
                                                                    libc::c_uint
                                                                    &
                                                                    !(ZXDG_POSITIONER_V6_GRAVITY_TOP
                                                                          as
                                                                          libc::c_int)
                                                                        as
                                                                        libc::c_uint);
        (*positioner).gravity =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_gravity>((*positioner).gravity
                                                                    as
                                                                    libc::c_uint
                                                                    |
                                                                    ZXDG_POSITIONER_V6_GRAVITY_BOTTOM
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
    } else if (*positioner).gravity as libc::c_uint &
                  ZXDG_POSITIONER_V6_GRAVITY_BOTTOM as libc::c_int as
                      libc::c_uint != 0 {
        (*positioner).gravity =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_gravity>((*positioner).gravity
                                                                    as
                                                                    libc::c_uint
                                                                    &
                                                                    !(ZXDG_POSITIONER_V6_GRAVITY_BOTTOM
                                                                          as
                                                                          libc::c_int)
                                                                        as
                                                                        libc::c_uint);
        (*positioner).gravity =
            ::std::mem::transmute::<libc::c_uint,
                                    zxdg_positioner_v6_gravity>((*positioner).gravity
                                                                    as
                                                                    libc::c_uint
                                                                    |
                                                                    ZXDG_POSITIONER_V6_GRAVITY_TOP
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
    };
}
#[no_mangle]
pub unsafe extern "C" fn create_xdg_positioner_v6(mut client:
                                                      *mut wlr_xdg_client_v6,
                                                  mut id: uint32_t) {
    let mut positioner: *mut wlr_xdg_positioner_v6_resource =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_xdg_positioner_v6_resource>() as
                   libc::c_ulong) as *mut wlr_xdg_positioner_v6_resource;
    if positioner.is_null() {
        wl_client_post_no_memory((*client).client);
        return
    }
    (*positioner).resource =
        wl_resource_create((*client).client, &zxdg_positioner_v6_interface,
                           wl_resource_get_version((*client).resource), id);
    if (*positioner).resource.is_null() {
        free(positioner as *mut libc::c_void);
        wl_client_post_no_memory((*client).client);
        return
    }
    wl_resource_set_implementation((*positioner).resource,
                                   &zxdg_positioner_v6_implementation as
                                       *const zxdg_positioner_v6_interface as
                                       *const libc::c_void,
                                   positioner as *mut libc::c_void,
                                   Some(xdg_positioner_destroy as
                                            unsafe extern "C" fn(_:
                                                                     *mut wl_resource)
                                                -> ()));
}
