use libc;
extern "C" {
    /* Generated by wayland-scanner 1.17.0 */
    /*
 * Copyright 2014 © Stephen "Lyude" Chandler Paul
 * Copyright 2015-2016 © Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
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
    #[no_mangle]
    static wl_seat_interface: wl_interface;
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
static mut types: [*const wl_interface; 23] =
    unsafe {
        [0 as *const wl_interface, 0 as *const wl_interface,
         0 as *const wl_interface,
         &zwp_tablet_seat_v2_interface as *const wl_interface,
         &wl_seat_interface as *const wl_interface,
         &zwp_tablet_v2_interface as *const wl_interface,
         &zwp_tablet_tool_v2_interface as *const wl_interface,
         &zwp_tablet_pad_v2_interface as *const wl_interface,
         0 as *const wl_interface,
         &wl_surface_interface as *const wl_interface,
         0 as *const wl_interface, 0 as *const wl_interface,
         0 as *const wl_interface,
         &zwp_tablet_v2_interface as *const wl_interface,
         &wl_surface_interface as *const wl_interface,
         &zwp_tablet_pad_ring_v2_interface as *const wl_interface,
         &zwp_tablet_pad_strip_v2_interface as *const wl_interface,
         &zwp_tablet_pad_group_v2_interface as *const wl_interface,
         0 as *const wl_interface,
         &zwp_tablet_v2_interface as *const wl_interface,
         &wl_surface_interface as *const wl_interface,
         0 as *const wl_interface,
         &wl_surface_interface as *const wl_interface]
    };
// Initialized in run_static_initializers
static mut zwp_tablet_manager_v2_requests: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
#[no_mangle]
pub static mut zwp_tablet_manager_v2_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_tablet_manager_v2\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 1i32,
                             method_count: 2i32,
                             methods: zwp_tablet_manager_v2_requests.as_ptr(),
                             event_count: 0i32,
                             events: 0 as *const wl_message,};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwp_tablet_seat_v2_requests: [wl_message; 1] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 1];
// Initialized in run_static_initializers
static mut zwp_tablet_seat_v2_events: [wl_message; 3] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 3];
#[no_mangle]
pub static mut zwp_tablet_seat_v2_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_tablet_seat_v2\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 1i32,
                             method_count: 1i32,
                             methods: zwp_tablet_seat_v2_requests.as_ptr(),
                             event_count: 3i32,
                             events: zwp_tablet_seat_v2_events.as_ptr(),};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwp_tablet_tool_v2_requests: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
// Initialized in run_static_initializers
static mut zwp_tablet_tool_v2_events: [wl_message; 19] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 19];
#[no_mangle]
pub static mut zwp_tablet_tool_v2_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_tablet_tool_v2\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 1i32,
                             method_count: 2i32,
                             methods: zwp_tablet_tool_v2_requests.as_ptr(),
                             event_count: 19i32,
                             events: zwp_tablet_tool_v2_events.as_ptr(),};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwp_tablet_v2_requests: [wl_message; 1] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 1];
// Initialized in run_static_initializers
static mut zwp_tablet_v2_events: [wl_message; 5] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 5];
#[no_mangle]
pub static mut zwp_tablet_v2_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_tablet_v2\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 1i32,
                             method_count: 1i32,
                             methods: zwp_tablet_v2_requests.as_ptr(),
                             event_count: 5i32,
                             events: zwp_tablet_v2_events.as_ptr(),};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwp_tablet_pad_ring_v2_requests: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
// Initialized in run_static_initializers
static mut zwp_tablet_pad_ring_v2_events: [wl_message; 4] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 4];
#[no_mangle]
pub static mut zwp_tablet_pad_ring_v2_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_tablet_pad_ring_v2\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 1i32,
                             method_count: 2i32,
                             methods:
                                 zwp_tablet_pad_ring_v2_requests.as_ptr(),
                             event_count: 4i32,
                             events: zwp_tablet_pad_ring_v2_events.as_ptr(),};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwp_tablet_pad_strip_v2_requests: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
// Initialized in run_static_initializers
static mut zwp_tablet_pad_strip_v2_events: [wl_message; 4] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 4];
#[no_mangle]
pub static mut zwp_tablet_pad_strip_v2_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_tablet_pad_strip_v2\x00" as *const u8
                                     as *const libc::c_char,
                             version: 1i32,
                             method_count: 2i32,
                             methods:
                                 zwp_tablet_pad_strip_v2_requests.as_ptr(),
                             event_count: 4i32,
                             events:
                                 zwp_tablet_pad_strip_v2_events.as_ptr(),};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwp_tablet_pad_group_v2_requests: [wl_message; 1] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 1];
// Initialized in run_static_initializers
static mut zwp_tablet_pad_group_v2_events: [wl_message; 6] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 6];
#[no_mangle]
pub static mut zwp_tablet_pad_group_v2_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_tablet_pad_group_v2\x00" as *const u8
                                     as *const libc::c_char,
                             version: 1i32,
                             method_count: 1i32,
                             methods:
                                 zwp_tablet_pad_group_v2_requests.as_ptr(),
                             event_count: 6i32,
                             events:
                                 zwp_tablet_pad_group_v2_events.as_ptr(),};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwp_tablet_pad_v2_requests: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
// Initialized in run_static_initializers
static mut zwp_tablet_pad_v2_events: [wl_message; 8] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 8];
#[no_mangle]
pub static mut zwp_tablet_pad_v2_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_tablet_pad_v2\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 1i32,
                             method_count: 2i32,
                             methods: zwp_tablet_pad_v2_requests.as_ptr(),
                             event_count: 8i32,
                             events: zwp_tablet_pad_v2_events.as_ptr(),};
            init
        }
    };
unsafe extern "C" fn run_static_initializers() {
    zwp_tablet_manager_v2_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"get_tablet_seat\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"no\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(3),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_seat_v2_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_seat_v2_events =
        [{
             let mut init =
                 wl_message{name:
                                b"tablet_added\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"n\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(5),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"tool_added\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"n\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(6),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"pad_added\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"n\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(7),};
             init
         }];
    zwp_tablet_tool_v2_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"set_cursor\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u?oii\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(8),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_tool_v2_events =
        [{
             let mut init =
                 wl_message{name:
                                b"type\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"hardware_serial\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"uu\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"hardware_id_wacom\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"uu\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"capability\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"done\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"removed\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"proximity_in\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"uoo\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(12),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"proximity_out\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"down\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"up\x00" as *const u8 as *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"motion\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"ff\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"pressure\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"distance\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"tilt\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"ff\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"rotation\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"f\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"slider\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"i\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"wheel\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"fi\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"button\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"uuu\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"frame\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_v2_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_v2_events =
        [{
             let mut init =
                 wl_message{name:
                                b"name\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"s\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"id\x00" as *const u8 as *const libc::c_char,
                            signature:
                                b"uu\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"path\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"s\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"done\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"removed\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_pad_ring_v2_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"set_feedback\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"su\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_pad_ring_v2_events =
        [{
             let mut init =
                 wl_message{name:
                                b"source\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"angle\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"f\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"stop\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"frame\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_pad_strip_v2_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"set_feedback\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"su\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_pad_strip_v2_events =
        [{
             let mut init =
                 wl_message{name:
                                b"source\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"position\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"stop\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"frame\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_pad_group_v2_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_pad_group_v2_events =
        [{
             let mut init =
                 wl_message{name:
                                b"buttons\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"a\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"ring\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"n\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(15),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"strip\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"n\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(16),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"modes\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"done\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"mode_switch\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"uuu\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_pad_v2_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"set_feedback\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"usu\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_tablet_pad_v2_events =
        [{
             let mut init =
                 wl_message{name:
                                b"group\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"n\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(17),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"path\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"s\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"buttons\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"done\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"button\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"uuu\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"enter\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"uoo\x00" as *const u8 as
                                    *const libc::c_char,
                            types: types.as_mut_ptr().offset(18),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"leave\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"uo\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(21),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"removed\x00" as *const u8 as
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
