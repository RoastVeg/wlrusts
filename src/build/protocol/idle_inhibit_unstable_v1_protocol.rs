use libc;
extern "C" {
    /* Generated by wayland-scanner 1.17.0 */
    /*
 * Copyright © 2015 Samsung Electronics Co., Ltd
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
static mut types: [*const wl_interface; 2] =
    unsafe {
        [&zwp_idle_inhibitor_v1_interface as *const wl_interface,
         &wl_surface_interface as *const wl_interface]
    };
// Initialized in run_static_initializers
static mut zwp_idle_inhibit_manager_v1_requests: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
#[no_mangle]
pub static mut zwp_idle_inhibit_manager_v1_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_idle_inhibit_manager_v1\x00" as
                                     *const u8 as *const libc::c_char,
                             version: 1i32,
                             method_count: 2i32,
                             methods:
                                 zwp_idle_inhibit_manager_v1_requests.as_ptr(),
                             event_count: 0i32,
                             events: 0 as *const wl_message,};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwp_idle_inhibitor_v1_requests: [wl_message; 1] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 1];
#[no_mangle]
pub static mut zwp_idle_inhibitor_v1_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwp_idle_inhibitor_v1\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 1i32,
                             method_count: 1i32,
                             methods: zwp_idle_inhibitor_v1_requests.as_ptr(),
                             event_count: 0i32,
                             events: 0 as *const wl_message,};
            init
        }
    };
unsafe extern "C" fn run_static_initializers() {
    zwp_idle_inhibit_manager_v1_requests =
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
                                b"create_inhibitor\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"no\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         }];
    zwp_idle_inhibitor_v1_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"destroy\x00" as *const u8 as
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
