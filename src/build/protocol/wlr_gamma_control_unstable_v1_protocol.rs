use libc;
extern "C" {
    /* Generated by wayland-scanner 1.17.0 */
    /*
 * Copyright © 2015 Giulio camuffo
 * Copyright © 2018 Simon Ser
 *
 * Permission to use, copy, modify, distribute, and sell this
 * software and its documentation for any purpose is hereby granted
 * without fee, provided that the above copyright notice appear in
 * all copies and that both that copyright notice and this permission
 * notice appear in supporting documentation, and that the name of
 * the copyright holders not be used in advertising or publicity
 * pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied
 * warranty.
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS
 * SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS, IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
 * ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
 * THIS SOFTWARE.
 */
    #[no_mangle]
    static wl_output_interface: wl_interface;
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
static mut types: [*const wl_interface; 3] =
    unsafe {
        [0 as *const wl_interface,
         &zwlr_gamma_control_v1_interface as *const wl_interface,
         &wl_output_interface as *const wl_interface]
    };
// Initialized in run_static_initializers
static mut zwlr_gamma_control_manager_v1_requests: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
#[no_mangle]
pub static mut zwlr_gamma_control_manager_v1_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwlr_gamma_control_manager_v1\x00" as
                                     *const u8 as *const libc::c_char,
                             version: 1i32,
                             method_count: 2i32,
                             methods:
                                 zwlr_gamma_control_manager_v1_requests.as_ptr(),
                             event_count: 0i32,
                             events: 0 as *const wl_message,};
            init
        }
    };
// Initialized in run_static_initializers
static mut zwlr_gamma_control_v1_requests: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
// Initialized in run_static_initializers
static mut zwlr_gamma_control_v1_events: [wl_message; 2] =
    [wl_message{name: 0 as *const libc::c_char,
                signature: 0 as *const libc::c_char,
                types: 0 as *mut *const wl_interface,}; 2];
#[no_mangle]
pub static mut zwlr_gamma_control_v1_interface: wl_interface =
    unsafe {
        {
            let mut init =
                wl_interface{name:
                                 b"zwlr_gamma_control_v1\x00" as *const u8 as
                                     *const libc::c_char,
                             version: 1i32,
                             method_count: 2i32,
                             methods: zwlr_gamma_control_v1_requests.as_ptr(),
                             event_count: 2i32,
                             events: zwlr_gamma_control_v1_events.as_ptr(),};
            init
        }
    };
unsafe extern "C" fn run_static_initializers() {
    zwlr_gamma_control_manager_v1_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"get_gamma_control\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"no\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(1),};
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
    zwlr_gamma_control_v1_requests =
        [{
             let mut init =
                 wl_message{name:
                                b"set_gamma\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"h\x00" as *const u8 as *const libc::c_char,
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
    zwlr_gamma_control_v1_events =
        [{
             let mut init =
                 wl_message{name:
                                b"gamma_size\x00" as *const u8 as
                                    *const libc::c_char,
                            signature:
                                b"u\x00" as *const u8 as *const libc::c_char,
                            types: types.as_mut_ptr().offset(0),};
             init
         },
         {
             let mut init =
                 wl_message{name:
                                b"failed\x00" as *const u8 as
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
