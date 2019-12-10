use libc;
extern "C" {
    /* Generated by wayland-scanner 1.17.0 */
    /*
 * Copyright © 2014      Jonas Ådahl
 * Copyright © 2015      Red Hat Inc.
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
    static wl_pointer_interface: wl_interface;
    #[no_mangle]
    static wl_region_interface: wl_interface;
    #[no_mangle]
    static wl_surface_interface: wl_interface;
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct wl_message {
    pub name: *const libc::c_char,
    pub signature: *const libc::c_char,
    pub types: *mut *const wl_interface,
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
static mut types: [*const wl_interface; 14] =
    unsafe {
        [0 as *const wl_interface, 0 as *const wl_interface,
         &zwp_locked_pointer_v1_interface as *const wl_interface,
         &wl_surface_interface as *const wl_interface,
         &wl_pointer_interface as *const wl_interface,
         &wl_region_interface as *const wl_interface,
         0 as *const wl_interface,
         &zwp_confined_pointer_v1_interface as *const wl_interface,
         &wl_surface_interface as *const wl_interface,
         &wl_pointer_interface as *const wl_interface,
         &wl_region_interface as *const wl_interface,
         0 as *const wl_interface,
         &wl_region_interface as *const wl_interface,
         &wl_region_interface as *const wl_interface]
    };
// Initialized in run_static_initializers
static mut zwp_pointer_constraints_v1_requests: [wl_message; 3] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 3];
#[no_mangle]
pub static mut zwp_pointer_constraints_v1_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_pointer_constraints_v1\x00" as
                                     *const u8 as *const libc::c_char,
                             version: 1i32,
                             method_count: 3i32,
                             methods:
                                 zwp_pointer_constraints_v1_requests.as_ptr(),
                             event_count: 0i32,
                             events: 0 as *const wl_message,};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwp_locked_pointer_v1_requests: [wl_message; 3] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 3];
// Initialized in run_static_initializers
static mut zwp_locked_pointer_v1_events: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
#[no_mangle]
pub static mut zwp_locked_pointer_v1_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_locked_pointer_v1\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 1i32,
                             method_count: 3i32,
                             methods: zwp_locked_pointer_v1_requests.as_ptr(),
                             event_count: 2i32,
                             events: zwp_locked_pointer_v1_events.as_ptr(),};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwp_confined_pointer_v1_requests: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
// Initialized in run_static_initializers
static mut zwp_confined_pointer_v1_events: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
#[no_mangle]
pub static mut zwp_confined_pointer_v1_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_confined_pointer_v1\x00" as *const u8
                                     as *const libc::c_char,
                             version: 1i32,
                             method_count: 2i32,
                             methods:
                                 zwp_confined_pointer_v1_requests.as_ptr(),
                             event_count: 2i32,
                             events:
                                 zwp_confined_pointer_v1_events.as_ptr(),};
            init
        }
    };
unsafe extern "C" fn run_static_initializers() {
    zwp_pointer_constraints_v1_requests =
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
                                b"lock_pointer\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"noo?ou\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(2),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"confine_pointer\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"noo?ou\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(7),};
             init
         }];
    zwp_locked_pointer_v1_requests =
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
                                b"set_cursor_position_hint\x00" as *const u8
                                    as *const libc::c_char,
                            signature:
                                b"ff\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"set_region\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"?o\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(12),};
             init
         }];
    zwp_locked_pointer_v1_events =
        [{
             let mut init =
                 wl_message{name:
                                b"locked\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"unlocked\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_confined_pointer_v1_requests =
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
                                b"set_region\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"?o\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(13),};
             init
         }];
    zwp_confined_pointer_v1_events =
        [{
             let mut init =
                 wl_message{name:
                                b"confined\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"unconfined\x00" as *const u8 as
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
