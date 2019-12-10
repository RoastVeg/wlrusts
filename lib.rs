#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]




pub mod src {
    pub mod backend {
        pub mod backend;
        pub mod drm {
            pub mod atomic;
            pub mod backend;
            pub mod cvt;
            pub mod drm;
            pub mod legacy;
            pub mod properties;
            pub mod renderer;
            pub mod util;
        } // mod drm
        pub mod headless {
            pub mod backend;
            pub mod input_device;
            pub mod output;
        } // mod headless
        pub mod libinput {
            pub mod backend;
            pub mod events;
            pub mod keyboard;
            pub mod pointer;
            pub mod switch;
            pub mod tablet_pad;
            pub mod tablet_tool;
            pub mod touch;
        } // mod libinput
        pub mod multi {
            pub mod backend;
        } // mod multi
        pub mod noop {
            pub mod backend;
            pub mod output;
        } // mod noop
        pub mod rdp {
            pub mod backend;
            pub mod keyboard;
            pub mod listener;
            pub mod output;
            pub mod peer;
            pub mod pointer;
        } // mod rdp
        pub mod session {
            pub mod direct;
            pub mod direct_ipc;
            pub mod logind;
            pub mod noop;
            pub mod session;
        } // mod session
        pub mod wayland {
            pub mod backend;
            pub mod output;
            pub mod tablet_v2;
            pub mod wl_seat;
        } // mod wayland
        pub mod x11 {
            pub mod backend;
            pub mod input_device;
            pub mod output;
        } // mod x11
    } // mod backend
    pub mod build {
        pub mod protocol {
            pub mod fullscreen_shell_unstable_v1_protocol;
            pub mod gtk_primary_selection_protocol;
            pub mod idle_inhibit_unstable_v1_protocol;
            pub mod idle_protocol;
            pub mod input_method_unstable_v2_protocol;
            pub mod linux_dmabuf_unstable_v1_protocol;
            pub mod pointer_constraints_unstable_v1_protocol;
            pub mod pointer_gestures_unstable_v1_protocol;
            pub mod presentation_time_protocol;
            pub mod primary_selection_unstable_v1_protocol;
            pub mod relative_pointer_unstable_v1_protocol;
            pub mod server_decoration_protocol;
            pub mod tablet_unstable_v2_protocol;
            pub mod text_input_unstable_v3_protocol;
            pub mod virtual_keyboard_unstable_v1_protocol;
            pub mod wlr_data_control_unstable_v1_protocol;
            pub mod wlr_export_dmabuf_unstable_v1_protocol;
            pub mod wlr_foreign_toplevel_management_unstable_v1_protocol;
            pub mod wlr_gamma_control_unstable_v1_protocol;
            pub mod wlr_input_inhibitor_unstable_v1_protocol;
            pub mod wlr_layer_shell_unstable_v1_protocol;
            pub mod wlr_output_management_unstable_v1_protocol;
            pub mod wlr_screencopy_unstable_v1_protocol;
            pub mod xdg_decoration_unstable_v1_protocol;
            pub mod xdg_output_unstable_v1_protocol;
            pub mod xdg_shell_protocol;
            pub mod xdg_shell_unstable_v6_protocol;
        } // mod protocol
        pub mod render {
            pub mod glapi;
        } // mod render
    } // mod build
    /*
    pub mod examples {
        pub mod cat;
        pub mod dmabuf_capture;
        pub mod foreign_toplevel;
        pub mod fullscreen_shell;
        pub mod gamma_control;
        pub mod idle;
        pub mod idle_inhibit;
        pub mod input_inhibitor;
        pub mod input_method;
        pub mod layer_shell;
        pub mod multi_pointer;
        pub mod output_layout;
        pub mod pointer;
        pub mod pointer_constraints;
        pub mod relative_pointer_unstable_v1;
        pub mod rotation;
        pub mod screencopy;
        pub mod simple;
        pub mod tablet;
        pub mod text_input;
        pub mod toplevel_decoration;
        pub mod touch;
    } // mod examples
    */
    pub mod render {
        pub mod dmabuf;
        pub mod drm_format_set;
        pub mod egl;
        pub mod gles2 {
            pub mod pixel_format;
            pub mod renderer;
            pub mod shaders;
            pub mod texture;
        } // mod gles2
        pub mod wlr_renderer;
        pub mod wlr_texture;
    } // mod render
    pub mod types {
        pub mod data_device {
            pub mod wlr_data_device;
            pub mod wlr_data_offer;
            pub mod wlr_data_source;
            pub mod wlr_drag;
        } // mod data_device
        /*
        pub mod seat {
            pub mod wlr_seat;
            pub mod wlr_seat_keyboard;
            pub mod wlr_seat_pointer;
            pub mod wlr_seat_touch;
        } // mod seat
         */
        pub mod tablet_v2 {
            pub mod wlr_tablet_v2;
            pub mod wlr_tablet_v2_pad;
            pub mod wlr_tablet_v2_tablet;
            pub mod wlr_tablet_v2_tool;
        } // mod tablet_v2
        pub mod wlr_box;
        pub mod wlr_buffer;
        pub mod wlr_compositor;
        pub mod wlr_cursor;
        pub mod wlr_data_control_v1;
        pub mod wlr_export_dmabuf_v1;
        pub mod wlr_foreign_toplevel_management_v1;
        pub mod wlr_fullscreen_shell_v1;
        pub mod wlr_gamma_control_v1;
        pub mod wlr_gtk_primary_selection;
        pub mod wlr_idle;
        pub mod wlr_idle_inhibit_v1;
        pub mod wlr_input_device;
        pub mod wlr_input_inhibitor;
        pub mod wlr_input_method_v2;
        pub mod wlr_keyboard;
        pub mod wlr_keyboard_group;
        pub mod wlr_layer_shell_v1;
        pub mod wlr_linux_dmabuf_v1;
        pub mod wlr_list;
        pub mod wlr_matrix;
        pub mod wlr_output;
        pub mod wlr_output_damage;
        pub mod wlr_output_layout;
        pub mod wlr_output_management_v1;
        pub mod wlr_pointer;
        pub mod wlr_pointer_constraints_v1;
        pub mod wlr_pointer_gestures_v1;
        pub mod wlr_presentation_time;
        pub mod wlr_primary_selection;
        pub mod wlr_primary_selection_v1;
        pub mod wlr_region;
        pub mod wlr_relative_pointer_v1;
        pub mod wlr_screencopy_v1;
        pub mod wlr_server_decoration;
        pub mod wlr_surface;
        pub mod wlr_switch;
        pub mod wlr_tablet_pad;
        pub mod wlr_tablet_tool;
        pub mod wlr_text_input_v3;
        pub mod wlr_touch;
        pub mod wlr_virtual_keyboard_v1;
        pub mod wlr_xcursor_manager;
        pub mod wlr_xdg_decoration_v1;
        pub mod wlr_xdg_output_v1;
        pub mod xdg_shell {
            pub mod wlr_xdg_popup;
            pub mod wlr_xdg_positioner;
            pub mod wlr_xdg_shell;
            pub mod wlr_xdg_surface;
            pub mod wlr_xdg_toplevel;
        } // mod xdg_shell
        pub mod xdg_shell_v6 {
            pub mod wlr_xdg_popup_v6;
            pub mod wlr_xdg_positioner_v6;
            pub mod wlr_xdg_shell_v6;
            pub mod wlr_xdg_surface_v6;
            pub mod wlr_xdg_toplevel_v6;
        } // mod xdg_shell_v6
    } // mod types
    pub mod util {
        pub mod array;
        pub mod log;
        pub mod region;
        pub mod shm;
        pub mod signal;
    } // mod util
    pub mod xcursor {
        pub mod wlr_xcursor;
        pub mod xcursor;
    } // mod xcursor
    pub mod xwayland {
        pub mod selection {
            pub mod dnd;
            pub mod incoming;
            pub mod outgoing;
            pub mod selection;
        } // mod selection
        pub mod sockets;
        pub mod xwayland;
        pub mod xwm;
    } // mod xwayland
} // mod src

