use libc;
extern "C" {
    /* Generated by wayland-scanner 1.17.0 */
    /*
 * Copyright © 2008-2013 Kristian Høgsberg
 * Copyright © 2013      Rafael Antognolli
 * Copyright © 2013      Jasper St. Pierre
 * Copyright © 2010-2013 Intel Corporation
 * Copyright © 2015-2017 Samsung Electronics Co., Ltd
 * Copyright © 2015-2017 Red Hat Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */
    #[no_mangle]
    static wl_output_interface: wl_interface;
    #[no_mangle]
    static wl_seat_interface: wl_interface;
    #[no_mangle]
    static wl_surface_interface: wl_interface;
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_message {
    pub name: *const libc::c_char,
    pub signature: *const libc::c_char,
    pub types: *mut *const wl_interface,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_interface {
    pub name: *const libc::c_char,
    pub version: libc::c_int,
    pub method_count: libc::c_int,
    pub methods: *const wl_message,
    pub event_count: libc::c_int,
    pub events: *const wl_message,
}
static mut types: [*const wl_interface; 24] =
    unsafe {
        [0 as *const wl_interface, 0 as *const wl_interface,
         0 as *const wl_interface, 0 as *const wl_interface,
         &xdg_positioner_interface as *const wl_interface,
         &xdg_surface_interface as *const wl_interface,
         &wl_surface_interface as *const wl_interface,
         &xdg_toplevel_interface as *const wl_interface,
         &xdg_popup_interface as *const wl_interface,
         &xdg_surface_interface as *const wl_interface,
         &xdg_positioner_interface as *const wl_interface,
         &xdg_toplevel_interface as *const wl_interface,
         &wl_seat_interface as *const wl_interface, 0 as *const wl_interface,
         0 as *const wl_interface, 0 as *const wl_interface,
         &wl_seat_interface as *const wl_interface, 0 as *const wl_interface,
         &wl_seat_interface as *const wl_interface, 0 as *const wl_interface,
         0 as *const wl_interface,
         &wl_output_interface as *const wl_interface,
         &wl_seat_interface as *const wl_interface, 0 as *const wl_interface]
    };
// Initialized in run_static_initializers
static mut xdg_wm_base_requests: [wl_message; 4] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 4];
// Initialized in run_static_initializers
static mut xdg_wm_base_events: [wl_message; 1] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 1];
#[no_mangle]
pub static mut xdg_wm_base_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"xdg_wm_base\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 2i32,
                             method_count: 4i32,
                             methods: xdg_wm_base_requests.as_ptr(),
                             event_count: 1i32,
                             events: xdg_wm_base_events.as_ptr(),};
            init
        }
    };
// Initialized in run_static_initializers
static mut xdg_positioner_requests: [wl_message; 7] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 7];
#[no_mangle]
pub static mut xdg_positioner_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"xdg_positioner\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 2i32,
                             method_count: 7i32,
                             methods: xdg_positioner_requests.as_ptr(),
                             event_count: 0i32,
                             events: 0 as *const wl_message,};
            init
        }
    };
// Initialized in run_static_initializers
static mut xdg_surface_requests: [wl_message; 5] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 5];
// Initialized in run_static_initializers
static mut xdg_surface_events: [wl_message; 1] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 1];
#[no_mangle]
pub static mut xdg_surface_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"xdg_surface\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 2i32,
                             method_count: 5i32,
                             methods: xdg_surface_requests.as_ptr(),
                             event_count: 1i32,
                             events: xdg_surface_events.as_ptr(),};
            init
        }
    };
// Initialized in run_static_initializers
static mut xdg_toplevel_requests: [wl_message; 14] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 14];
// Initialized in run_static_initializers
static mut xdg_toplevel_events: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
#[no_mangle]
pub static mut xdg_toplevel_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"xdg_toplevel\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 2i32,
                             method_count: 14i32,
                             methods: xdg_toplevel_requests.as_ptr(),
                             event_count: 2i32,
                             events: xdg_toplevel_events.as_ptr(),};
            init
        }
    };
// Initialized in run_static_initializers
static mut xdg_popup_requests: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
// Initialized in run_static_initializers
static mut xdg_popup_events: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
#[no_mangle]
pub static mut xdg_popup_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"xdg_popup\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 2i32,
                             method_count: 2i32,
                             methods: xdg_popup_requests.as_ptr(),
                             event_count: 2i32,
                             events: xdg_popup_events.as_ptr(),};
            init
        }
    };
unsafe extern "C" fn run_static_initializers() {
    xdg_wm_base_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"create_positioner\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"n\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(4),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"get_xdg_surface\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"no\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(5),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"pong\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    xdg_wm_base_events =
        [{
             let mut init =
                 wl_message{name:
                                b"ping\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    xdg_positioner_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_size\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"ii\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_anchor_rect\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"iiii\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_anchor\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_gravity\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_constraint_adjustment\x00" as *const u8
                                    as *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_offset\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"ii\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    xdg_surface_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"get_toplevel\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"n\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(7),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"get_popup\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"n?oo\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(8),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_window_geometry\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"iiii\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"ack_configure\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    xdg_surface_events =
        [{
             let mut init =
                 wl_message{name:
                                b"configure\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    xdg_toplevel_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_parent\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"?o\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(11),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_title\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"s\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_app_id\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"s\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"show_window_menu\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"ouii\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(12),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"move\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"ou\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(16),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"resize\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"ouu\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(18),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_max_size\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"ii\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_min_size\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"ii\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_maximized\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"unset_maximized\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_fullscreen\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"?o\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(21),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"unset_fullscreen\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_minimized\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    xdg_toplevel_events =
        [{
             let mut init =
                 wl_message{name:
                                b"configure\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"iia\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"close\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    xdg_popup_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"grab\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"ou\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(22),};
             init
         }];
    xdg_popup_events =
        [{
             let mut init =
                 wl_message{name:
                                b"configure\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"iiii\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"popup_done\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }]
}
#[used]
#[cfg_attr ( target_os = "linux", link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows", link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos", link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
