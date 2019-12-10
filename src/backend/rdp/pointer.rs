use libc;
extern "C" {
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_tablet_pad_impl;
    pub type wlr_tablet_impl;
    pub type wlr_touch_impl;
    pub type wlr_switch_impl;
    pub type _wLog;
    pub type _NSC_CONTEXT_PRIV;
    pub type _RFX_CONTEXT_PRIV;
    /* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * FreeRDP Interface
 *
 * Copyright 2009-2011 Jay Sorg
 * Copyright 2015 Thincast Technologies GmbH
 * Copyright 2015 DI (FH) Martin Haimberger <martin.haimberger@thincast.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
    pub type rdp_rdp;
    pub type rdp_gdi;
    pub type rdp_rail;
    pub type rdp_cache;
    pub type rdp_channels;
    pub type _PROGRESSIVE_CONTEXT;
    pub type _CLEAR_CONTEXT;
    pub type rdp_update_proxy;
    pub type rdp_input_proxy;
    pub type pixman_image;
    pub type udev;
    pub type udev_monitor;
    pub type session_impl;
    pub type wlr_renderer;
    pub type wlr_texture;
    pub type wlr_surface;
    pub type wlr_output_impl;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn wlr_pointer_init(pointer: *mut wlr_pointer,
                        impl_0: *const wlr_pointer_impl);
    #[no_mangle]
    fn wlr_input_device_init(wlr_device: *mut wlr_input_device,
                             type_0: wlr_input_device_type,
                             impl_0: *const wlr_input_device_impl,
                             name: *const libc::c_char, vendor: libc::c_int,
                             product: libc::c_int);
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn wlr_signal_emit_safe(signal: *mut wl_signal, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type clockid_t = __clockid_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_object {
    pub interface: *const wl_interface,
    pub implementation: *const libc::c_void,
    pub id: uint32_t,
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_message {
    pub name: *const libc::c_char,
    pub signature: *const libc::c_char,
    pub types: *mut *const wl_interface,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_listener {
    pub link: wl_list,
    pub notify: wl_notify_func_t,
}
pub type wl_notify_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_listener, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_resource {
    pub object: wl_object,
    pub destroy: wl_resource_destroy_func_t,
    pub link: wl_list,
    pub destroy_signal: wl_signal,
    pub client: *mut wl_client,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
pub type wl_resource_destroy_func_t
    =
    Option<unsafe extern "C" fn(_: *mut wl_resource) -> ()>;
pub type wlr_input_device_type = libc::c_uint;
pub const WLR_INPUT_DEVICE_SWITCH: wlr_input_device_type = 5;
pub const WLR_INPUT_DEVICE_TABLET_PAD: wlr_input_device_type = 4;
pub const WLR_INPUT_DEVICE_TABLET_TOOL: wlr_input_device_type = 3;
pub const WLR_INPUT_DEVICE_TOUCH: wlr_input_device_type = 2;
pub const WLR_INPUT_DEVICE_POINTER: wlr_input_device_type = 1;
pub const WLR_INPUT_DEVICE_KEYBOARD: wlr_input_device_type = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_keyboard_modifiers {
    pub depressed: xkb_mod_mask_t,
    pub latched: xkb_mod_mask_t,
    pub locked: xkb_mod_mask_t,
    pub group: xkb_mod_mask_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_keyboard {
    pub impl_0: *const wlr_keyboard_impl,
    pub group: *mut wlr_keyboard_group,
    pub keymap_string: *mut libc::c_char,
    pub keymap_size: size_t,
    pub keymap: *mut xkb_keymap,
    pub xkb_state: *mut xkb_state,
    pub led_indexes: [xkb_led_index_t; 3],
    pub mod_indexes: [xkb_mod_index_t; 8],
    pub keycodes: [uint32_t; 32],
    pub num_keycodes: size_t,
    pub modifiers: wlr_keyboard_modifiers,
    pub repeat_info: C2RustUnnamed_0,
    pub events: C2RustUnnamed,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub rate: int32_t,
    pub delay: int32_t,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_pointer) -> ()>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_pointer {
    pub impl_0: *const wlr_pointer_impl,
    pub events: C2RustUnnamed_1,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub motion: wl_signal,
    pub motion_absolute: wl_signal,
    pub button: wl_signal,
    pub axis: wl_signal,
    pub frame: wl_signal,
    pub swipe_begin: wl_signal,
    pub swipe_update: wl_signal,
    pub swipe_end: wl_signal,
    pub pinch_begin: wl_signal,
    pub pinch_update: wl_signal,
    pub pinch_end: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_input_device {
    pub impl_0: *const wlr_input_device_impl,
    pub type_0: wlr_input_device_type,
    pub vendor: libc::c_uint,
    pub product: libc::c_uint,
    pub name: *mut libc::c_char,
    pub width_mm: libc::c_double,
    pub height_mm: libc::c_double,
    pub output_name: *mut libc::c_char,
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub events: C2RustUnnamed_2,
    pub data: *mut libc::c_void,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub destroy: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_3 {
    pub _device: *mut libc::c_void,
    pub keyboard: *mut wlr_keyboard,
    pub pointer: *mut wlr_pointer,
    pub switch_device: *mut wlr_switch,
    pub touch: *mut wlr_touch,
    pub tablet: *mut wlr_tablet,
    pub tablet_pad: *mut wlr_tablet_pad,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/*
 * NOTE: the wlr tablet pad implementation does not currently support tablets
 * with more than one mode. I don't own any such hardware so I cannot test it
 * and it is too complicated to make a meaningful implementation of blindly.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet_pad {
    pub impl_0: *mut wlr_tablet_pad_impl,
    pub events: C2RustUnnamed_4,
    pub button_count: size_t,
    pub ring_count: size_t,
    pub strip_count: size_t,
    pub groups: wl_list,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_list {
    pub capacity: size_t,
    pub length: size_t,
    pub items: *mut *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub button: wl_signal,
    pub ring: wl_signal,
    pub strip: wl_signal,
    pub attach_tablet: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/*
 * Copy+Paste from libinput, but this should neither use libinput, nor
 * tablet-unstable-v2 headers, so we can't include them
 */
/* * A generic pen */
/* * Eraser */
/* * A paintbrush-like tool */
/* * Physical drawing tool, e.g. Wacom Inking Pen */
/* * An airbrush-like tool */
/* * A mouse bound to the tablet */
/* * A mouse tool with a lens */
/* * A rotary device with positional and rotation data */
// Capabilities
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_tablet {
    pub impl_0: *mut wlr_tablet_impl,
    pub events: C2RustUnnamed_5,
    pub name: *mut libc::c_char,
    pub paths: wlr_list,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub axis: wl_signal,
    pub proximity: wl_signal,
    pub tip: wl_signal,
    pub button: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_touch {
    pub impl_0: *const wlr_touch_impl,
    pub events: C2RustUnnamed_6,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub down: wl_signal,
    pub up: wl_signal,
    pub motion: wl_signal,
    pub cancel: wl_signal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_switch {
    pub impl_0: *mut wlr_switch_impl,
    pub events: C2RustUnnamed_7,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub toggle: wl_signal,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* Note: these are circular dependencies */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_input_device_impl {
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_input_device) -> ()>,
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
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
pub type __uint8 = uint8_t;
pub type __int16 = int16_t;
pub type __uint16 = uint16_t;
pub type __int32 = int32_t;
pub type __uint32 = uint32_t;
pub type __int64 = int64_t;
pub type __uint64 = uint64_t;
/* WINPR_HAVE_STDINT_H */
/* WINPR_HAVE_STDINT_H */
pub type PVOID = *mut libc::c_void;
/* X11/Xmd.h typedef collision with BOOL */
/* objc.h typedef collision with BOOL */
pub type BOOL = __int32;
/* X11/Xmd.h typedef collision with BYTE */
pub type BYTE = __uint8;
pub type WCHAR = __uint16;
pub type LONG = __int32;
pub type UINT = __uint32;
pub type ULONG = __uint32;
pub type INT16 = __int16;
pub type INT32 = __int32;
pub type INT64 = __int64;
pub type UINT16 = __uint16;
pub type UINT32 = __uint32;
pub type UINT64 = __uint64;
pub type WORD = __uint16;
pub type DWORD = __uint32;
pub type ULONG_PTR = uint64_t;
pub type FLOAT = libc::c_float;
pub type HANDLE = *mut libc::c_void;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _SYSTEMTIME {
    pub wYear: WORD,
    pub wMonth: WORD,
    pub wDayOfWeek: WORD,
    pub wDay: WORD,
    pub wHour: WORD,
    pub wMinute: WORD,
    pub wSecond: WORD,
    pub wMilliseconds: WORD,
}
pub type SYSTEMTIME = _SYSTEMTIME;
/* Critical Section */
/* *
 * Linux NPTL thread synchronization primitives are implemented using
 * the futex system calls ... we can't beat futex with a spin loop.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _RTL_CRITICAL_SECTION {
    pub DebugInfo: PVOID,
    pub LockCount: LONG,
    pub RecursionCount: LONG,
    pub OwningThread: HANDLE,
    pub LockSemaphore: HANDLE,
    pub SpinCount: ULONG_PTR,
}
pub type RTL_CRITICAL_SECTION = _RTL_CRITICAL_SECTION;
pub type CRITICAL_SECTION = RTL_CRITICAL_SECTION;
pub type wLog = _wLog;
/* *
 * WinPR: Windows Portable Runtime
 * Windows Terminal Services API
 *
 * Copyright 2013 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 * Copyright 2015 DI (FH) Martin Haimberger <martin.haimberger@thincast.com>
 * Copyright 2015 Copyright 2015 Thincast Technologies GmbH
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/* *
 * Virtual Channel Protocol (pchannel.h)
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct tagCHANNEL_DEF {
    pub name: [libc::c_char; 8],
    pub options: ULONG,
}
pub type CHANNEL_DEF = tagCHANNEL_DEF;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Type Definitions
 *
 * Copyright 2009-2011 Jay Sorg
 * Copyright 2011 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _PALETTE_ENTRY {
    pub red: BYTE,
    pub green: BYTE,
    pub blue: BYTE,
}
pub type PALETTE_ENTRY = _PALETTE_ENTRY;
/* *
 * WinPR: Windows Portable Runtime
 * Time Zone
 *
 * Copyright 2012 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _TIME_ZONE_INFORMATION {
    pub Bias: LONG,
    pub StandardName: [WCHAR; 32],
    pub StandardDate: SYSTEMTIME,
    pub StandardBias: LONG,
    pub DaylightName: [WCHAR; 32],
    pub DaylightDate: SYSTEMTIME,
    pub DaylightBias: LONG,
}
pub type LPTIME_ZONE_INFORMATION = *mut _TIME_ZONE_INFORMATION;
/* Logon Error Info */
/* Server Status Info */
/* Compression Flags */
/* Desktop Rotation Flags */
/* ARC_CS_PRIVATE_PACKET */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ARC_CS_PRIVATE_PACKET {
    pub cbLen: UINT32,
    pub version: UINT32,
    pub logonId: UINT32,
    pub securityVerifier: [BYTE; 16],
}
/* ARC_SC_PRIVATE_PACKET */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ARC_SC_PRIVATE_PACKET {
    pub cbLen: UINT32,
    pub version: UINT32,
    pub logonId: UINT32,
    pub arcRandomBits: [BYTE; 16],
}
/* Certificates */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_CertBlob {
    pub length: UINT32,
    pub data: *mut BYTE,
}
pub type rdpCertBlob = rdp_CertBlob;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_X509CertChain {
    pub count: UINT32,
    pub array: *mut rdpCertBlob,
}
pub type rdpX509CertChain = rdp_X509CertChain;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_CertInfo {
    pub Modulus: *mut BYTE,
    pub ModulusLength: DWORD,
    pub exponent: [BYTE; 4],
}
pub type rdpCertInfo = rdp_CertInfo;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_certificate {
    pub cert_info: rdpCertInfo,
    pub x509_cert_chain: *mut rdpX509CertChain,
}
pub type rdpCertificate = rdp_certificate;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_rsa_key {
    pub Modulus: *mut BYTE,
    pub ModulusLength: DWORD,
    pub PrivateExponent: *mut BYTE,
    pub PrivateExponentLength: DWORD,
    pub exponent: [BYTE; 4],
}
pub type rdpRsaKey = rdp_rsa_key;
/* Channels */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _ADDIN_ARGV {
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type ADDIN_ARGV = _ADDIN_ARGV;
/* Extensions */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_ext_set {
    pub name: [libc::c_char; 256],
    pub data: *mut libc::c_void,
    /* plugin data */
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _BITMAP_CACHE_V2_CELL_INFO {
    pub numEntries: UINT32,
    pub persistent: BOOL,
}
pub type BITMAP_CACHE_V2_CELL_INFO = _BITMAP_CACHE_V2_CELL_INFO;
/* Glyph Cache */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _GLYPH_CACHE_DEFINITION {
    pub cacheEntries: UINT16,
    pub cacheMaximumCellSize: UINT16,
}
pub type GLYPH_CACHE_DEFINITION = _GLYPH_CACHE_DEFINITION;
/* Monitors */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _MONITOR_DEF {
    pub left: INT32,
    pub top: INT32,
    pub right: INT32,
    pub bottom: INT32,
    pub flags: UINT32,
}
pub type MONITOR_DEF = _MONITOR_DEF;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _MONITOR_ATTRIBUTES {
    pub physicalWidth: UINT32,
    pub physicalHeight: UINT32,
    pub orientation: UINT32,
    pub desktopScaleFactor: UINT32,
    pub deviceScaleFactor: UINT32,
}
pub type MONITOR_ATTRIBUTES = _MONITOR_ATTRIBUTES;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_monitor {
    pub x: INT32,
    pub y: INT32,
    pub width: INT32,
    pub height: INT32,
    pub is_primary: UINT32,
    pub orig_screen: UINT32,
    pub attributes: MONITOR_ATTRIBUTES,
}
pub type rdpMonitor = rdp_monitor;
/* Device Redirection */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _RDPDR_DEVICE {
    pub Id: UINT32,
    pub Type: UINT32,
    pub Name: *mut libc::c_char,
}
pub type RDPDR_DEVICE = _RDPDR_DEVICE;
/* Settings */
/* *
 * FreeRDP Settings Ids
 * This is generated with a script parsing the rdpSettings data structure
 */
/* *
 * FreeRDP Settings Data Structure
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_settings {
    pub instance: *mut libc::c_void,
    pub padding001: [UINT64; 15],
    pub ServerMode: BOOL,
    pub ShareId: UINT32,
    pub PduSource: UINT32,
    pub ServerPort: UINT32,
    pub ServerHostname: *mut libc::c_char,
    pub Username: *mut libc::c_char,
    pub Password: *mut libc::c_char,
    pub Domain: *mut libc::c_char,
    pub PasswordHash: *mut libc::c_char,
    pub WaitForOutputBufferFlush: BOOL,
    pub MaxTimeInCheckLoop: UINT32,
    pub AcceptedCert: *mut libc::c_char,
    pub AcceptedCertLength: UINT32,
    pub padding0064: [UINT64; 35],
    pub padding0128: [UINT64; 64],
    pub RdpVersion: UINT32,
    pub DesktopWidth: UINT32,
    pub DesktopHeight: UINT32,
    pub ColorDepth: UINT32,
    pub ConnectionType: UINT32,
    pub ClientBuild: UINT32,
    pub ClientHostname: *mut libc::c_char,
    pub ClientProductId: *mut libc::c_char,
    pub EarlyCapabilityFlags: UINT32,
    pub NetworkAutoDetect: BOOL,
    pub SupportAsymetricKeys: BOOL,
    pub SupportErrorInfoPdu: BOOL,
    pub SupportStatusInfoPdu: BOOL,
    pub SupportMonitorLayoutPdu: BOOL,
    pub SupportGraphicsPipeline: BOOL,
    pub SupportDynamicTimeZone: BOOL,
    pub SupportHeartbeatPdu: BOOL,
    pub DesktopPhysicalWidth: UINT32,
    pub DesktopPhysicalHeight: UINT32,
    pub DesktopOrientation: UINT16,
    pub DesktopScaleFactor: UINT32,
    pub DeviceScaleFactor: UINT32,
    pub padding0192: [UINT64; 42],
    pub UseRdpSecurityLayer: BOOL,
    pub EncryptionMethods: UINT32,
    pub ExtEncryptionMethods: UINT32,
    pub EncryptionLevel: UINT32,
    pub ServerRandom: *mut BYTE,
    pub ServerRandomLength: UINT32,
    pub ServerCertificate: *mut BYTE,
    pub ServerCertificateLength: UINT32,
    pub ClientRandom: *mut BYTE,
    pub ClientRandomLength: UINT32,
    pub padding0256: [UINT64; 54],
    pub ChannelCount: UINT32,
    pub ChannelDefArraySize: UINT32,
    pub ChannelDefArray: *mut CHANNEL_DEF,
    pub padding0320: [UINT64; 61],
    pub ClusterInfoFlags: UINT32,
    pub RedirectedSessionId: UINT32,
    pub ConsoleSession: BOOL,
    pub padding0384: [UINT64; 61],
    pub MonitorCount: UINT32,
    pub MonitorDefArraySize: UINT32,
    pub MonitorDefArray: *mut rdpMonitor,
    pub SpanMonitors: BOOL,
    pub UseMultimon: BOOL,
    pub ForceMultimon: BOOL,
    pub DesktopPosX: UINT32,
    pub DesktopPosY: UINT32,
    pub ListMonitors: BOOL,
    pub MonitorIds: *mut UINT32,
    pub NumMonitorIds: UINT32,
    pub MonitorLocalShiftX: UINT32,
    pub MonitorLocalShiftY: UINT32,
    pub HasMonitorAttributes: BOOL,
    pub padding0448: [UINT64; 50],
    pub padding0512: [UINT64; 64],
    pub MultitransportFlags: UINT32,
    pub SupportMultitransport: BOOL,
    pub padding0576: [UINT64; 62],
    pub padding0640: [UINT64; 64],
    pub AlternateShell: *mut libc::c_char,
    pub ShellWorkingDirectory: *mut libc::c_char,
    pub padding0704: [UINT64; 62],
    pub AutoLogonEnabled: BOOL,
    pub CompressionEnabled: BOOL,
    pub DisableCtrlAltDel: BOOL,
    pub EnableWindowsKey: BOOL,
    pub MaximizeShell: BOOL,
    pub LogonNotify: BOOL,
    pub LogonErrors: BOOL,
    pub MouseAttached: BOOL,
    pub MouseHasWheel: BOOL,
    pub RemoteConsoleAudio: BOOL,
    pub AudioPlayback: BOOL,
    pub AudioCapture: BOOL,
    pub VideoDisable: BOOL,
    pub PasswordIsSmartcardPin: BOOL,
    pub UsingSavedCredentials: BOOL,
    pub ForceEncryptedCsPdu: BOOL,
    pub HiDefRemoteApp: BOOL,
    pub CompressionLevel: UINT32,
    pub padding0768: [UINT64; 46],
    pub IPv6Enabled: BOOL,
    pub ClientAddress: *mut libc::c_char,
    pub ClientDir: *mut libc::c_char,
    pub padding0832: [UINT64; 61],
    pub AutoReconnectionEnabled: BOOL,
    pub AutoReconnectMaxRetries: UINT32,
    pub ClientAutoReconnectCookie: *mut ARC_CS_PRIVATE_PACKET,
    pub ServerAutoReconnectCookie: *mut ARC_SC_PRIVATE_PACKET,
    pub PrintReconnectCookie: BOOL,
    pub padding0896: [UINT64; 59],
    pub ClientTimeZone: LPTIME_ZONE_INFORMATION,
    pub DynamicDSTTimeZoneKeyName: *mut libc::c_char,
    pub DynamicDaylightTimeDisabled: BOOL,
    pub padding0960: [UINT64; 61],
    pub PerformanceFlags: UINT32,
    pub AllowFontSmoothing: BOOL,
    pub DisableWallpaper: BOOL,
    pub DisableFullWindowDrag: BOOL,
    pub DisableMenuAnims: BOOL,
    pub DisableThemes: BOOL,
    pub DisableCursorShadow: BOOL,
    pub DisableCursorBlinking: BOOL,
    pub AllowDesktopComposition: BOOL,
    pub padding1024: [UINT64; 55],
    pub RemoteAssistanceMode: BOOL,
    pub RemoteAssistanceSessionId: *mut libc::c_char,
    pub RemoteAssistancePassStub: *mut libc::c_char,
    pub RemoteAssistancePassword: *mut libc::c_char,
    pub RemoteAssistanceRCTicket: *mut libc::c_char,
    pub EncomspVirtualChannel: BOOL,
    pub RemdeskVirtualChannel: BOOL,
    pub LyncRdpMode: BOOL,
    pub padding1088: [UINT64; 56],
    pub TlsSecurity: BOOL,
    pub NlaSecurity: BOOL,
    pub RdpSecurity: BOOL,
    pub ExtSecurity: BOOL,
    pub Authentication: BOOL,
    pub RequestedProtocols: UINT32,
    pub SelectedProtocol: UINT32,
    pub NegotiationFlags: UINT32,
    pub NegotiateSecurityLayer: BOOL,
    pub RestrictedAdminModeRequired: BOOL,
    pub AuthenticationServiceClass: *mut libc::c_char,
    pub DisableCredentialsDelegation: BOOL,
    pub AuthenticationLevel: UINT32,
    pub AllowedTlsCiphers: *mut libc::c_char,
    pub VmConnectMode: BOOL,
    pub NtlmSamFile: *mut libc::c_char,
    pub FIPSMode: BOOL,
    pub TlsSecLevel: UINT32,
    pub padding1152: [UINT64; 46],
    pub MstscCookieMode: BOOL,
    pub CookieMaxLength: UINT32,
    pub PreconnectionId: UINT32,
    pub PreconnectionBlob: *mut libc::c_char,
    pub SendPreconnectionPdu: BOOL,
    pub padding1216: [UINT64; 59],
    pub RedirectionFlags: UINT32,
    pub TargetNetAddress: *mut libc::c_char,
    pub LoadBalanceInfo: *mut BYTE,
    pub LoadBalanceInfoLength: UINT32,
    pub RedirectionUsername: *mut libc::c_char,
    pub RedirectionDomain: *mut libc::c_char,
    pub RedirectionPassword: *mut BYTE,
    pub RedirectionPasswordLength: UINT32,
    pub RedirectionTargetFQDN: *mut libc::c_char,
    pub RedirectionTargetNetBiosName: *mut libc::c_char,
    pub RedirectionTsvUrl: *mut BYTE,
    pub RedirectionTsvUrlLength: UINT32,
    pub TargetNetAddressCount: UINT32,
    pub TargetNetAddresses: *mut *mut libc::c_char,
    pub TargetNetPorts: *mut UINT32,
    pub RedirectionAcceptedCert: *mut libc::c_char,
    pub RedirectionAcceptedCertLength: UINT32,
    pub RedirectionPreferType: UINT32,
    pub padding1280: [UINT64; 46],
    pub Password51: *mut BYTE,
    pub Password51Length: UINT32,
    pub SmartcardLogon: BOOL,
    pub padding1344: [UINT64; 61],
    pub KerberosKdc: *mut libc::c_char,
    pub KerberosRealm: *mut libc::c_char,
    pub padding1408: [UINT64; 62],
    pub IgnoreCertificate: BOOL,
    pub CertificateName: *mut libc::c_char,
    pub CertificateFile: *mut libc::c_char,
    pub PrivateKeyFile: *mut libc::c_char,
    pub RdpKeyFile: *mut libc::c_char,
    pub RdpServerRsaKey: *mut rdpRsaKey,
    pub RdpServerCertificate: *mut rdpCertificate,
    pub ExternalCertificateManagement: BOOL,
    pub CertificateContent: *mut libc::c_char,
    pub PrivateKeyContent: *mut libc::c_char,
    pub RdpKeyContent: *mut libc::c_char,
    pub AutoAcceptCertificate: BOOL,
    pub padding1472: [UINT64; 52],
    pub padding1536: [UINT64; 64],
    pub Workarea: BOOL,
    pub Fullscreen: BOOL,
    pub PercentScreen: UINT32,
    pub GrabKeyboard: BOOL,
    pub Decorations: BOOL,
    pub MouseMotion: BOOL,
    pub WindowTitle: *mut libc::c_char,
    pub ParentWindowId: UINT64,
    pub AsyncInput: BOOL,
    pub AsyncUpdate: BOOL,
    pub AsyncChannels: BOOL,
    pub padding1548: [UINT64; 1],
    pub ToggleFullscreen: BOOL,
    pub WmClass: *mut libc::c_char,
    pub EmbeddedWindow: BOOL,
    pub SmartSizing: BOOL,
    pub XPan: libc::c_int,
    pub YPan: libc::c_int,
    pub SmartSizingWidth: UINT32,
    pub SmartSizingHeight: UINT32,
    pub PercentScreenUseWidth: BOOL,
    pub PercentScreenUseHeight: BOOL,
    pub DynamicResolutionUpdate: BOOL,
    pub padding1601: [UINT64; 42],
    pub SoftwareGdi: BOOL,
    pub LocalConnection: BOOL,
    pub AuthenticationOnly: BOOL,
    pub CredentialsFromStdin: BOOL,
    pub UnmapButtons: BOOL,
    pub OldLicenseBehaviour: BOOL,
    pub padding1664: [UINT64; 57],
    pub ComputerName: *mut libc::c_char,
    pub padding1728: [UINT64; 63],
    pub ConnectionFile: *mut libc::c_char,
    pub AssistanceFile: *mut libc::c_char,
    pub padding1792: [UINT64; 62],
    pub HomePath: *mut libc::c_char,
    pub ConfigPath: *mut libc::c_char,
    pub CurrentPath: *mut libc::c_char,
    pub padding1856: [UINT64; 61],
    pub DumpRemoteFx: BOOL,
    pub PlayRemoteFx: BOOL,
    pub DumpRemoteFxFile: *mut libc::c_char,
    pub PlayRemoteFxFile: *mut libc::c_char,
    pub padding1920: [UINT64; 60],
    pub padding1984: [UINT64; 64],
    pub GatewayUsageMethod: UINT32,
    pub GatewayPort: UINT32,
    pub GatewayHostname: *mut libc::c_char,
    pub GatewayUsername: *mut libc::c_char,
    pub GatewayPassword: *mut libc::c_char,
    pub GatewayDomain: *mut libc::c_char,
    pub GatewayCredentialsSource: UINT32,
    pub GatewayUseSameCredentials: BOOL,
    pub GatewayEnabled: BOOL,
    pub GatewayBypassLocal: BOOL,
    pub GatewayRpcTransport: BOOL,
    pub GatewayHttpTransport: BOOL,
    pub GatewayUdpTransport: BOOL,
    pub GatewayAccessToken: *mut libc::c_char,
    pub GatewayAcceptedCert: *mut libc::c_char,
    pub GatewayAcceptedCertLength: UINT32,
    pub padding2015: [UINT64; 15],
    pub ProxyType: UINT32,
    pub ProxyHostname: *mut libc::c_char,
    pub ProxyPort: UINT16,
    pub ProxyUsername: *mut libc::c_char,
    pub ProxyPassword: *mut libc::c_char,
    pub padding2112: [UINT64; 92],
    pub RemoteApplicationMode: BOOL,
    pub RemoteApplicationName: *mut libc::c_char,
    pub RemoteApplicationIcon: *mut libc::c_char,
    pub RemoteApplicationProgram: *mut libc::c_char,
    pub RemoteApplicationFile: *mut libc::c_char,
    pub RemoteApplicationGuid: *mut libc::c_char,
    pub RemoteApplicationCmdLine: *mut libc::c_char,
    pub RemoteApplicationExpandCmdLine: UINT32,
    pub RemoteApplicationExpandWorkingDir: UINT32,
    pub DisableRemoteAppCapsCheck: BOOL,
    pub RemoteAppNumIconCaches: UINT32,
    pub RemoteAppNumIconCacheEntries: UINT32,
    pub RemoteAppLanguageBarSupported: BOOL,
    pub RemoteWndSupportLevel: UINT32,
    pub padding2176: [UINT64; 50],
    pub padding2240: [UINT64; 64],
    pub ReceivedCapabilities: *mut BYTE,
    pub ReceivedCapabilitiesSize: UINT32,
    pub padding2304: [UINT64; 62],
    pub OsMajorType: UINT32,
    pub OsMinorType: UINT32,
    pub RefreshRect: BOOL,
    pub SuppressOutput: BOOL,
    pub FastPathOutput: BOOL,
    pub SaltedChecksum: BOOL,
    pub LongCredentialsSupported: BOOL,
    pub NoBitmapCompressionHeader: BOOL,
    pub BitmapCompressionDisabled: BOOL,
    pub padding2368: [UINT64; 55],
    pub DesktopResize: BOOL,
    pub DrawAllowDynamicColorFidelity: BOOL,
    pub DrawAllowColorSubsampling: BOOL,
    pub DrawAllowSkipAlpha: BOOL,
    pub padding2432: [UINT64; 60],
    pub OrderSupport: *mut BYTE,
    pub BitmapCacheV3Enabled: BOOL,
    pub AltSecFrameMarkerSupport: BOOL,
    pub AllowUnanouncedOrdersFromServer: BOOL,
    pub padding2497: [UINT64; 61],
    pub BitmapCacheEnabled: BOOL,
    pub BitmapCacheVersion: UINT32,
    pub AllowCacheWaitingList: BOOL,
    pub BitmapCachePersistEnabled: BOOL,
    pub BitmapCacheV2NumCells: UINT32,
    pub BitmapCacheV2CellInfo: *mut BITMAP_CACHE_V2_CELL_INFO,
    pub padding2560: [UINT64; 57],
    pub ColorPointerFlag: BOOL,
    pub PointerCacheSize: UINT32,
    pub padding2624: [UINT64; 62],
    pub KeyboardLayout: UINT32,
    pub KeyboardType: UINT32,
    pub KeyboardSubType: UINT32,
    pub KeyboardFunctionKey: UINT32,
    pub ImeFileName: *mut libc::c_char,
    pub UnicodeInput: BOOL,
    pub FastPathInput: BOOL,
    pub MultiTouchInput: BOOL,
    pub MultiTouchGestures: BOOL,
    pub KeyboardHook: UINT32,
    pub HasHorizontalWheel: BOOL,
    pub HasExtendedMouseEvent: BOOL,
    pub padding2688: [UINT64; 52],
    pub BrushSupportLevel: UINT32,
    pub padding2752: [UINT64; 63],
    pub GlyphSupportLevel: UINT32,
    pub GlyphCache: *mut GLYPH_CACHE_DEFINITION,
    pub FragCache: *mut GLYPH_CACHE_DEFINITION,
    pub padding2816: [UINT64; 61],
    pub OffscreenSupportLevel: UINT32,
    pub OffscreenCacheSize: UINT32,
    pub OffscreenCacheEntries: UINT32,
    pub padding2880: [UINT64; 61],
    pub VirtualChannelCompressionFlags: UINT32,
    pub VirtualChannelChunkSize: UINT32,
    pub padding2944: [UINT64; 62],
    pub SoundBeepsEnabled: BOOL,
    pub padding3008: [UINT64; 63],
    pub padding3072: [UINT64; 64],
    pub padding3136: [UINT64; 64],
    pub padding3200: [UINT64; 64],
    pub padding3264: [UINT64; 64],
    pub padding3328: [UINT64; 64],
    pub MultifragMaxRequestSize: UINT32,
    pub padding3392: [UINT64; 63],
    pub LargePointerFlag: UINT32,
    pub padding3456: [UINT64; 63],
    pub CompDeskSupportLevel: UINT32,
    pub padding3520: [UINT64; 63],
    pub SurfaceCommandsEnabled: BOOL,
    pub FrameMarkerCommandEnabled: BOOL,
    pub SurfaceFrameMarkerEnabled: BOOL,
    pub padding3584: [UINT64; 61],
    pub padding3648: [UINT64; 64],
    pub RemoteFxOnly: BOOL,
    pub RemoteFxCodec: BOOL,
    pub RemoteFxCodecId: UINT32,
    pub RemoteFxCodecMode: UINT32,
    pub RemoteFxImageCodec: BOOL,
    pub RemoteFxCaptureFlags: UINT32,
    pub padding3712: [UINT64; 58],
    pub NSCodec: BOOL,
    pub NSCodecId: UINT32,
    pub FrameAcknowledge: UINT32,
    pub NSCodecColorLossLevel: UINT32,
    pub NSCodecAllowSubsampling: BOOL,
    pub NSCodecAllowDynamicColorFidelity: BOOL,
    pub padding3776: [UINT64; 58],
    pub JpegCodec: BOOL,
    pub JpegCodecId: UINT32,
    pub JpegQuality: UINT32,
    pub padding3840: [UINT64; 61],
    pub GfxThinClient: BOOL,
    pub GfxSmallCache: BOOL,
    pub GfxProgressive: BOOL,
    pub GfxProgressiveV2: BOOL,
    pub GfxH264: BOOL,
    pub GfxAVC444: BOOL,
    pub GfxSendQoeAck: BOOL,
    pub GfxAVC444v2: BOOL,
    pub padding3904: [UINT64; 56],
    pub BitmapCacheV3CodecId: UINT32,
    pub padding3968: [UINT64; 63],
    pub DrawNineGridEnabled: BOOL,
    pub DrawNineGridCacheSize: UINT32,
    pub DrawNineGridCacheEntries: UINT32,
    pub padding4032: [UINT64; 61],
    pub DrawGdiPlusEnabled: BOOL,
    pub DrawGdiPlusCacheEnabled: BOOL,
    pub padding4096: [UINT64; 62],
    pub padding4160: [UINT64; 64],
    pub DeviceRedirection: BOOL,
    pub DeviceCount: UINT32,
    pub DeviceArraySize: UINT32,
    pub DeviceArray: *mut *mut RDPDR_DEVICE,
    pub padding4288: [UINT64; 124],
    pub RedirectDrives: BOOL,
    pub RedirectHomeDrive: BOOL,
    pub DrivesToRedirect: *mut libc::c_char,
    pub padding4416: [UINT64; 125],
    pub RedirectSmartCards: BOOL,
    pub padding4544: [UINT64; 127],
    pub RedirectPrinters: BOOL,
    pub padding4672: [UINT64; 127],
    pub RedirectSerialPorts: BOOL,
    pub RedirectParallelPorts: BOOL,
    pub PreferIPv6OverIPv4: BOOL,
    pub padding4800: [UINT64; 125],
    pub RedirectClipboard: BOOL,
    pub padding4928: [UINT64; 127],
    pub StaticChannelCount: UINT32,
    pub StaticChannelArraySize: UINT32,
    pub StaticChannelArray: *mut *mut ADDIN_ARGV,
    pub padding5056: [UINT64; 125],
    pub DynamicChannelCount: UINT32,
    pub DynamicChannelArraySize: UINT32,
    pub DynamicChannelArray: *mut *mut ADDIN_ARGV,
    pub SupportDynamicChannels: BOOL,
    pub padding5184: [UINT64; 124],
    pub SupportEchoChannel: BOOL,
    pub SupportDisplayControl: BOOL,
    pub SupportGeometryTracking: BOOL,
    pub SupportSSHAgentChannel: BOOL,
    pub SupportVideoOptimized: BOOL,
    pub padding5312: [UINT64; 123],
    pub num_extensions: libc::c_int,
    pub extensions: [rdp_ext_set; 16],
    pub SettingsModified: *mut BYTE,
    pub ActionScript: *mut libc::c_char,
    pub Floatbar: BOOL,
}
pub type rdpSettings = rdp_settings;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _RECTANGLE_16 {
    pub left: UINT16,
    pub top: UINT16,
    pub right: UINT16,
    pub bottom: UINT16,
}
pub type RECTANGLE_16 = _RECTANGLE_16;
/*
 * WinPR: Windows Portable Runtime
 * Stream Utils
 *
 * Copyright 2011 Vic Lee
 * Copyright 2012 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 * Copyright 2017 Armin Novak <armin.novak@thincast.com>
 * Copyright 2017 Thincast Technologies GmbH
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _wStreamPool {
    pub aSize: libc::c_int,
    pub aCapacity: libc::c_int,
    pub aArray: *mut *mut wStream,
    pub uSize: libc::c_int,
    pub uCapacity: libc::c_int,
    pub uArray: *mut *mut wStream,
    pub lock: CRITICAL_SECTION,
    pub synchronized: BOOL,
    pub defaultSize: size_t,
}
pub type wStream = _wStream;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _wStream {
    pub buffer: *mut BYTE,
    pub pointer: *mut BYTE,
    pub length: size_t,
    pub capacity: size_t,
    pub count: DWORD,
    pub pool: *mut wStreamPool,
    pub isAllocatedStream: BOOL,
    pub isOwner: BOOL,
}
pub type wStreamPool = _wStreamPool;
/* *
 * WinPR: Windows Portable Runtime
 * Collections
 *
 * Copyright 2012 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub type OBJECT_NEW_FN
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>;
pub type OBJECT_INIT_FN
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type OBJECT_UNINIT_FN
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type OBJECT_FREE_FN
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type OBJECT_EQUALS_FN
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> BOOL>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _wObject {
    pub fnObjectNew: OBJECT_NEW_FN,
    pub fnObjectInit: OBJECT_INIT_FN,
    pub fnObjectUninit: OBJECT_UNINIT_FN,
    pub fnObjectFree: OBJECT_FREE_FN,
    pub fnObjectEquals: OBJECT_EQUALS_FN,
}
pub type wObject = _wObject;
/* Message Queue */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _wMessage {
    pub id: UINT32,
    pub context: *mut libc::c_void,
    pub wParam: *mut libc::c_void,
    pub lParam: *mut libc::c_void,
    pub time: UINT64,
    pub Free: MESSAGE_FREE_FN,
}
pub type MESSAGE_FREE_FN
    =
    Option<unsafe extern "C" fn(_: *mut wMessage) -> ()>;
pub type wMessage = _wMessage;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _wMessageQueue {
    pub head: libc::c_int,
    pub tail: libc::c_int,
    pub size: libc::c_int,
    pub capacity: libc::c_int,
    pub array: *mut wMessage,
    pub lock: CRITICAL_SECTION,
    pub event: HANDLE,
    pub object: wObject,
}
pub type wMessageQueue = _wMessageQueue;
/* Publisher/Subscriber Pattern */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _wEventArgs {
    pub Size: DWORD,
    pub Sender: *const libc::c_char,
}
pub type wEventArgs = _wEventArgs;
pub type pEventHandler
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut wEventArgs)
               -> ()>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _wEventType {
    pub EventName: *const libc::c_char,
    pub EventArgs: wEventArgs,
    pub EventHandlerCount: libc::c_int,
    pub EventHandlers: [pEventHandler; 32],
}
pub type wEventType = _wEventType;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _wPubSub {
    pub lock: CRITICAL_SECTION,
    pub synchronized: BOOL,
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub events: *mut wEventType,
}
pub type wPubSub = _wPubSub;
pub type NSC_CONTEXT_PRIV = _NSC_CONTEXT_PRIV;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _NSC_CONTEXT {
    pub OrgByteCount: [UINT32; 4],
    pub format: UINT32,
    pub width: UINT16,
    pub height: UINT16,
    pub BitmapData: *mut BYTE,
    pub BitmapDataLength: UINT32,
    pub Planes: *mut BYTE,
    pub PlaneByteCount: [UINT32; 4],
    pub ColorLossLevel: UINT32,
    pub ChromaSubsamplingLevel: UINT32,
    pub DynamicColorFidelity: BOOL,
    pub palette: *const BYTE,
    pub decode: Option<unsafe extern "C" fn(_: *mut NSC_CONTEXT) -> BOOL>,
    pub encode: Option<unsafe extern "C" fn(_: *mut NSC_CONTEXT,
                                            _: *const BYTE, _: UINT32)
                           -> BOOL>,
    pub priv_0: *mut NSC_CONTEXT_PRIV,
}
pub type NSC_CONTEXT = _NSC_CONTEXT;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * RemoteFX Codec
 *
 * Copyright 2011 Vic Lee
 * Copyright 2016 Armin Novak <armin.novak@thincast.com>
 * Copyright 2016 Thincast Technologies GmbH
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _RFX_RECT {
    pub x: UINT16,
    pub y: UINT16,
    pub width: UINT16,
    pub height: UINT16,
}
pub type RFX_RECT = _RFX_RECT;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _RFX_CONTEXT {
    pub state: RFX_STATE,
    pub encoder: BOOL,
    pub flags: UINT16,
    pub properties: UINT16,
    pub width: UINT16,
    pub height: UINT16,
    pub mode: RLGR_MODE,
    pub version: UINT32,
    pub codec_id: UINT32,
    pub codec_version: UINT32,
    pub pixel_format: UINT32,
    pub bits_per_pixel: BYTE,
    pub palette: *const BYTE,
    pub frameIdx: UINT32,
    pub numQuant: BYTE,
    pub quants: *mut UINT32,
    pub quantIdxY: BYTE,
    pub quantIdxCb: BYTE,
    pub quantIdxCr: BYTE,
    pub decodedHeaderBlocks: UINT32,
    pub quantization_decode: Option<unsafe extern "C" fn(_: *mut INT16,
                                                         _: *const UINT32)
                                        -> ()>,
    pub quantization_encode: Option<unsafe extern "C" fn(_: *mut INT16,
                                                         _: *const UINT32)
                                        -> ()>,
    pub dwt_2d_decode: Option<unsafe extern "C" fn(_: *mut INT16,
                                                   _: *mut INT16) -> ()>,
    pub dwt_2d_encode: Option<unsafe extern "C" fn(_: *mut INT16,
                                                   _: *mut INT16) -> ()>,
    pub rlgr_decode: Option<unsafe extern "C" fn(_: RLGR_MODE, _: *const BYTE,
                                                 _: UINT32, _: *mut INT16,
                                                 _: UINT32) -> libc::c_int>,
    pub rlgr_encode: Option<unsafe extern "C" fn(_: RLGR_MODE,
                                                 _: *const INT16, _: UINT32,
                                                 _: *mut BYTE, _: UINT32)
                                -> libc::c_int>,
    pub priv_0: *mut RFX_CONTEXT_PRIV,
}
pub type RFX_CONTEXT_PRIV = _RFX_CONTEXT_PRIV;
pub type RLGR_MODE = _RLGR_MODE;
pub type _RLGR_MODE = libc::c_uint;
pub const RLGR3: _RLGR_MODE = 1;
pub const RLGR1: _RLGR_MODE = 0;
pub type RFX_STATE = _RFX_STATE;
pub type _RFX_STATE = libc::c_uint;
pub const RFX_STATE_FINAL: _RFX_STATE = 5;
pub const RFX_STATE_FRAME_DATA_SENT: _RFX_STATE = 4;
pub const RFX_STATE_SEND_FRAME_DATA: _RFX_STATE = 3;
pub const RFX_STATE_SEND_HEADERS: _RFX_STATE = 2;
pub const RFX_STATE_SERVER_UNINITIALIZED: _RFX_STATE = 1;
pub const RFX_STATE_INITIAL: _RFX_STATE = 0;
pub type RFX_CONTEXT = _RFX_CONTEXT;
pub type rdpRdp = rdp_rdp;
pub type rdpGdi = rdp_gdi;
pub type rdpRail = rdp_rail;
pub type rdpCache = rdp_cache;
pub type rdpChannels = rdp_channels;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_graphics {
    pub context: *mut rdpContext,
    pub Bitmap_Prototype: *mut rdpBitmap,
    pub Pointer_Prototype: *mut rdpPointer,
    pub Glyph_Prototype: *mut rdpGlyph,
    pub paddingA: [UINT32; 12],
    /* * @brief Callback used if user interaction is required to accept
 *         an unknown certificate.
 *
 *  @param common_name      The certificate registered hostname.
 *  @param subject          The common name of the certificate.
 *  @param issuer           The issuer of the certificate.
 *  @param fingerprint      The fingerprint of the certificate.
 *  @param host_mismatch    A flag indicating the certificate
 *                          subject does not match the host connecting to.
 *
 *  @return 1 to accept and store a certificate, 2 to accept
 *          a certificate only for this session, 0 otherwise.
 */
    /* * @brief Callback used if user interaction is required to accept
 *         a changed certificate.
 *
 *  @param common_name      The certificate registered hostname.
 *  @param subject          The common name of the new certificate.
 *  @param issuer           The issuer of the new certificate.
 *  @param fingerprint      The fingerprint of the new certificate.
 *  @param old_subject      The common name of the old certificate.
 *  @param old_issuer       The issuer of the new certificate.
 *  @param old_fingerprint  The fingerprint of the old certificate.
 *
 *  @return 1 to accept and store a certificate, 2 to accept
 *          a certificate only for this session, 0 otherwise.
 */
    /* *
 * Defines the context for a given instance of RDP connection.
 * It is embedded in the rdp_freerdp structure, and allocated by a call to freerdp_context_new().
 * It is deallocated by a call to freerdp_context_free().
 */
    /* *< (offset 0)
						  Pointer to a rdp_freerdp structure.
						  This is a back-link to retrieve the freerdp instance from the context.
						  It is set by the freerdp_context_new() function */
    /* *< (offset 1)
						   Pointer to the client peer.
						   This is set by a call to freerdp_peer_context_new() during peer initialization.
						   This field is used only on the server side. */
    /* *< (offset 2) true when context is in server mode */
    /* 3 */
    /* 4 */
    /* *< (offset 16)
				   Number of arguments given to the program at launch time.
				   Used to keep this data available and used later on, typically just before connection initialization.
				   @see freerdp_parse_args() */
    /* *< (offset 17)
					List of arguments given to the program at launch time.
					Used to keep this data available and used later on, typically just before connection initialization.
					@see freerdp_parse_args() */
    /* (offset 18) */
}
pub type rdpGlyph = rdp_glyph;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_glyph {
    pub size: size_t,
    pub New: pGlyph_New,
    pub Free: pGlyph_Free,
    pub Draw: pGlyph_Draw,
    pub BeginDraw: pGlyph_BeginDraw,
    pub EndDraw: pGlyph_EndDraw,
    pub SetBounds: pGlyph_SetBounds,
    pub paddingA: [UINT32; 9],
    pub x: INT32,
    pub y: INT32,
    pub cx: UINT32,
    pub cy: UINT32,
    pub cb: UINT32,
    pub aj: *mut BYTE,
    pub paddingB: [UINT32; 10],
}
pub type pGlyph_SetBounds
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: INT32, _: INT32,
                                _: INT32, _: INT32) -> BOOL>;
pub type rdpContext = rdp_context;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_context {
    pub instance: *mut freerdp,
    pub peer: *mut freerdp_peer,
    pub ServerMode: BOOL,
    pub LastError: UINT32,
    pub paddingA: [UINT64; 12],
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub pubSub: *mut wPubSub,
    pub channelErrorEvent: HANDLE,
    pub channelErrorNum: UINT,
    pub errorDescription: *mut libc::c_char,
    pub paddingB: [UINT64; 10],
    pub rdp: *mut rdpRdp,
    pub gdi: *mut rdpGdi,
    pub rail: *mut rdpRail,
    pub cache: *mut rdpCache,
    pub channels: *mut rdpChannels,
    pub graphics: *mut rdpGraphics,
    pub input: *mut rdpInput,
    pub update: *mut rdpUpdate,
    pub settings: *mut rdpSettings,
    pub metrics: *mut rdpMetrics,
    pub codecs: *mut rdpCodecs,
    pub autodetect: *mut rdpAutoDetect,
    pub abortEvent: HANDLE,
    pub disconnectUltimatum: libc::c_int,
    pub paddingC: [UINT64; 18],
    pub paddingD: [UINT64; 32],
    pub paddingE: [UINT64; 32],
}
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Auto-Detect PDUs
 *
 * Copyright 2014 Dell Software <Mike.McDonald@software.dell.com>
 * Copyright 2014 Vic Lee
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub type rdpAutoDetect = rdp_autodetect;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_autodetect {
    pub context: *mut rdpContext,
    pub rttMeasureStartTime: UINT32,
    pub bandwidthMeasureStartTime: UINT32,
    pub bandwidthMeasureTimeDelta: UINT32,
    pub bandwidthMeasureByteCount: UINT32,
    pub netCharBandwidth: UINT32,
    pub netCharBaseRTT: UINT32,
    pub netCharAverageRTT: UINT32,
    pub bandwidthMeasureStarted: BOOL,
    pub paddingA: [UINT64; 7],
    pub RTTMeasureRequest: pRTTMeasureRequest,
    pub RTTMeasureResponse: pRTTMeasureResponse,
    pub BandwidthMeasureStart: pBandwidthMeasureStart,
    pub BandwidthMeasureStop: pBandwidthMeasureStop,
    pub BandwidthMeasureResults: pBandwidthMeasureResults,
    pub NetworkCharacteristicsResult: pNetworkCharacteristicsResult,
    pub ClientBandwidthMeasureResult: pClientBandwidthMeasureResult,
    pub paddingB: [UINT64; 9],
}
pub type pClientBandwidthMeasureResult
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut rdpAutoDetect)
               -> BOOL>;
pub type pNetworkCharacteristicsResult
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT16) -> BOOL>;
pub type pBandwidthMeasureResults
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT16) -> BOOL>;
pub type pBandwidthMeasureStop
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT16) -> BOOL>;
pub type pBandwidthMeasureStart
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT16) -> BOOL>;
pub type pRTTMeasureResponse
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT16) -> BOOL>;
pub type pRTTMeasureRequest
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT16) -> BOOL>;
pub type rdpCodecs = rdp_codecs;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_codecs {
    pub context: *mut rdpContext,
    pub rfx: *mut RFX_CONTEXT,
    pub nsc: *mut NSC_CONTEXT,
    pub h264: *mut H264_CONTEXT,
    pub clear: *mut CLEAR_CONTEXT,
    pub progressive: *mut PROGRESSIVE_CONTEXT,
    pub planar: *mut BITMAP_PLANAR_CONTEXT,
    pub interleaved: *mut BITMAP_INTERLEAVED_CONTEXT,
}
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Interleaved RLE Bitmap Codec
 *
 * Copyright 2014 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub type BITMAP_INTERLEAVED_CONTEXT = _BITMAP_INTERLEAVED_CONTEXT;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _BITMAP_INTERLEAVED_CONTEXT {
    pub Compressor: BOOL,
    pub TempSize: UINT32,
    pub TempBuffer: *mut BYTE,
    pub bts: *mut wStream,
}
pub type BITMAP_PLANAR_CONTEXT = _BITMAP_PLANAR_CONTEXT;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _BITMAP_PLANAR_CONTEXT {
    pub maxWidth: UINT32,
    pub maxHeight: UINT32,
    pub maxPlaneSize: UINT32,
    pub AllowSkipAlpha: BOOL,
    pub AllowRunLengthEncoding: BOOL,
    pub AllowColorSubsampling: BOOL,
    pub AllowDynamicColorFidelity: BOOL,
    pub ColorLossLevel: UINT32,
    pub planes: [*mut BYTE; 4],
    pub planesBuffer: *mut BYTE,
    pub deltaPlanes: [*mut BYTE; 4],
    pub deltaPlanesBuffer: *mut BYTE,
    pub rlePlanes: [*mut BYTE; 4],
    pub rlePlanesBuffer: *mut BYTE,
    pub pTempData: *mut BYTE,
    pub nTempStep: UINT32,
}
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Progressive Codec Bitmap Compression
 *
 * Copyright 2014 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub type PROGRESSIVE_CONTEXT = _PROGRESSIVE_CONTEXT;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * ClearCodec Bitmap Compression
 *
 * Copyright 2014 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub type CLEAR_CONTEXT = _CLEAR_CONTEXT;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * H.264 Bitmap Compression
 *
 * Copyright 2014 Mike McDonald <Mike.McDonald@software.dell.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub type H264_CONTEXT = _H264_CONTEXT;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _H264_CONTEXT {
    pub Compressor: BOOL,
    pub width: UINT32,
    pub height: UINT32,
    pub RateControlMode: H264_RATECONTROL_MODE,
    pub BitRate: UINT32,
    pub FrameRate: FLOAT,
    pub QP: UINT32,
    pub NumberOfThreads: UINT32,
    pub iStride: [UINT32; 3],
    pub pYUVData: [*mut BYTE; 3],
    pub iYUV444Size: [UINT32; 3],
    pub iYUV444Stride: [UINT32; 3],
    pub pYUV444Data: [*mut BYTE; 3],
    pub numSystemData: UINT32,
    pub pSystemData: *mut libc::c_void,
    pub subsystem: *mut H264_CONTEXT_SUBSYSTEM,
    pub lumaData: *mut libc::c_void,
    pub log: *mut wLog,
}
pub type H264_CONTEXT_SUBSYSTEM = _H264_CONTEXT_SUBSYSTEM;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _H264_CONTEXT_SUBSYSTEM {
    pub name: *const libc::c_char,
    pub Init: pfnH264SubsystemInit,
    pub Uninit: pfnH264SubsystemUninit,
    pub Decompress: pfnH264SubsystemDecompress,
    pub Compress: pfnH264SubsystemCompress,
}
pub type pfnH264SubsystemCompress
    =
    Option<unsafe extern "C" fn(_: *mut H264_CONTEXT, _: *mut *const BYTE,
                                _: *const UINT32, _: *mut *mut BYTE,
                                _: *mut UINT32) -> libc::c_int>;
pub type pfnH264SubsystemDecompress
    =
    Option<unsafe extern "C" fn(_: *mut H264_CONTEXT, _: *const BYTE,
                                _: UINT32) -> libc::c_int>;
pub type pfnH264SubsystemUninit
    =
    Option<unsafe extern "C" fn(_: *mut H264_CONTEXT) -> ()>;
pub type pfnH264SubsystemInit
    =
    Option<unsafe extern "C" fn(_: *mut H264_CONTEXT) -> BOOL>;
pub type H264_RATECONTROL_MODE = _H264_RATECONTROL_MODE;
pub type _H264_RATECONTROL_MODE = libc::c_uint;
pub const H264_RATECONTROL_CQP: _H264_RATECONTROL_MODE = 1;
pub const H264_RATECONTROL_VBR: _H264_RATECONTROL_MODE = 0;
pub type rdpMetrics = rdp_metrics;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_metrics {
    pub context: *mut rdpContext,
    pub TotalCompressedBytes: UINT64,
    pub TotalUncompressedBytes: UINT64,
    pub TotalCompressionRatio: libc::c_double,
}
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Update Interface API
 *
 * Copyright 2011 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub type rdpUpdate = rdp_update;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_update {
    pub context: *mut rdpContext,
    pub paddingA: [UINT32; 15],
    pub BeginPaint: pBeginPaint,
    pub EndPaint: pEndPaint,
    pub SetBounds: pSetBounds,
    pub Synchronize: pSynchronize,
    pub DesktopResize: pDesktopResize,
    pub BitmapUpdate: pBitmapUpdate,
    pub Palette: pPalette,
    pub PlaySound: pPlaySound,
    pub SetKeyboardIndicators: pSetKeyboardIndicators,
    pub SetKeyboardImeStatus: pSetKeyboardImeStatus,
    pub paddingB: [UINT32; 6],
    pub pointer: *mut rdpPointerUpdate,
    pub primary: *mut rdpPrimaryUpdate,
    pub secondary: *mut rdpSecondaryUpdate,
    pub altsec: *mut rdpAltSecUpdate,
    pub window: *mut rdpWindowUpdate,
    pub paddingC: [UINT32; 11],
    pub RefreshRect: pRefreshRect,
    pub SuppressOutput: pSuppressOutput,
    pub RemoteMonitors: pRemoteMonitors,
    pub paddingD: [UINT32; 13],
    pub SurfaceCommand: pSurfaceCommand,
    pub SurfaceBits: pSurfaceBits,
    pub SurfaceFrameMarker: pSurfaceFrameMarker,
    pub SurfaceFrameBits: pSurfaceFrameBits,
    pub SurfaceFrameAcknowledge: pSurfaceFrameAcknowledge,
    pub SaveSessionInfo: pSaveSessionInfo,
    pub paddingE: [UINT32; 10],
    pub log: *mut wLog,
    pub dump_rfx: BOOL,
    pub play_rfx: BOOL,
    pub pcap_rfx: *mut rdpPcap,
    pub initialState: BOOL,
    pub asynchronous: BOOL,
    pub proxy: *mut rdpUpdateProxy,
    pub queue: *mut wMessageQueue,
    pub us: *mut wStream,
    pub numberOrders: UINT16,
    pub combineUpdates: BOOL,
    pub currentBounds: rdpBounds,
    pub previousBounds: rdpBounds,
}
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Primary Drawing Orders Interface API
 *
 * Copyright 2011 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub type rdpBounds = rdp_bounds;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_bounds {
    pub left: INT32,
    pub top: INT32,
    pub right: INT32,
    pub bottom: INT32,
}
pub type rdpUpdateProxy = rdp_update_proxy;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * pcap File Format Utils
 *
 * Copyright 2011 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/* magic number */
/* major version number */
/* minor version number */
/* GMT to local correction */
/* accuracy of timestamps */
/* max length of captured packets, in octets */
/* data link type */
/* timestamp seconds */
/* timestamp microseconds */
/* number of octets of packet saved in file */
/* actual length of packet */
pub type rdpPcap = rdp_pcap;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_pcap {
    pub fp: *mut FILE,
    pub name: *mut libc::c_char,
    pub write: BOOL,
    pub file_size: INT64,
    pub record_count: libc::c_int,
    pub header: pcap_header,
    pub head: *mut pcap_record,
    pub tail: *mut pcap_record,
    pub record: *mut pcap_record,
}
pub type pcap_record = _pcap_record;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _pcap_record {
    pub header: pcap_record_header,
    pub data: *mut libc::c_void,
    pub length: UINT32,
    pub next: *mut pcap_record,
}
pub type pcap_record_header = _pcap_record_header;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _pcap_record_header {
    pub ts_sec: UINT32,
    pub ts_usec: UINT32,
    pub incl_len: UINT32,
    pub orig_len: UINT32,
}
pub type pcap_header = _pcap_header;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _pcap_header {
    pub magic_number: UINT32,
    pub version_major: UINT16,
    pub version_minor: UINT16,
    pub thiszone: INT32,
    pub sigfigs: UINT32,
    pub snaplen: UINT32,
    pub network: UINT32,
}
pub type pSaveSessionInfo
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT32,
                                _: *mut libc::c_void) -> BOOL>;
pub type pSurfaceFrameAcknowledge
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT32) -> BOOL>;
pub type pSurfaceFrameBits
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const SURFACE_BITS_COMMAND, _: BOOL,
                                _: BOOL, _: UINT32) -> BOOL>;
pub type SURFACE_BITS_COMMAND = _SURFACE_BITS_COMMAND;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _SURFACE_BITS_COMMAND {
    pub cmdType: UINT32,
    pub destLeft: UINT32,
    pub destTop: UINT32,
    pub destRight: UINT32,
    pub destBottom: UINT32,
    pub bmp: TS_BITMAP_DATA_EX,
    pub skipCompression: BOOL,
}
pub type TS_BITMAP_DATA_EX = _TS_BITMAP_DATA_EX;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _TS_BITMAP_DATA_EX {
    pub bpp: BYTE,
    pub flags: BYTE,
    pub codecID: UINT16,
    pub width: UINT16,
    pub height: UINT16,
    pub bitmapDataLength: UINT32,
    pub exBitmapDataHeader: TS_COMPRESSED_BITMAP_HEADER_EX,
    pub bitmapData: *mut BYTE,
}
pub type TS_COMPRESSED_BITMAP_HEADER_EX = _TS_COMPRESSED_BITMAP_HEADER_EX;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _TS_COMPRESSED_BITMAP_HEADER_EX {
    pub highUniqueId: UINT32,
    pub lowUniqueId: UINT32,
    pub tmMilliseconds: UINT64,
    pub tmSeconds: UINT64,
}
pub type pSurfaceFrameMarker
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const SURFACE_FRAME_MARKER) -> BOOL>;
pub type SURFACE_FRAME_MARKER = _SURFACE_FRAME_MARKER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _SURFACE_FRAME_MARKER {
    pub frameAction: UINT32,
    pub frameId: UINT32,
}
pub type pSurfaceBits
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const SURFACE_BITS_COMMAND) -> BOOL>;
pub type pSurfaceCommand
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut wStream) -> BOOL>;
pub type pRemoteMonitors
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT32,
                                _: *const MONITOR_DEF) -> BOOL>;
pub type pSuppressOutput
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: BYTE,
                                _: *const RECTANGLE_16) -> BOOL>;
pub type pRefreshRect
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: BYTE,
                                _: *const RECTANGLE_16) -> BOOL>;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Window Alternate Secondary Drawing Orders Interface API
 *
 * Copyright 2011 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/* Window Order Header Flags */
/* Window Show States */
/* Window Styles */
/* Extended Window Styles */
/* *
 * This is a custom extended window style used by XRDP
 * instructing the client to use local window decorations
 */
/* 0 */
/* 1 */
/* 16 */
/* 17 */
/* 18 */
/* 19 */
/* 20 */
/* 21 */
/* 22 */
/* 23 */
/* 24 */
/* 25 */
/* 26 */
/* internal */
pub type rdpWindowUpdate = rdp_window_update;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_window_update {
    pub context: *mut rdpContext,
    pub paddingA: [UINT32; 15],
    pub WindowCreate: pWindowCreate,
    pub WindowUpdate: pWindowUpdate,
    pub WindowIcon: pWindowIcon,
    pub WindowCachedIcon: pWindowCachedIcon,
    pub WindowDelete: pWindowDelete,
    pub NotifyIconCreate: pNotifyIconCreate,
    pub NotifyIconUpdate: pNotifyIconUpdate,
    pub NotifyIconDelete: pNotifyIconDelete,
    pub MonitoredDesktop: pMonitoredDesktop,
    pub NonMonitoredDesktop: pNonMonitoredDesktop,
    pub paddingB: [UINT32; 6],
    pub orderInfo: WINDOW_ORDER_INFO,
    pub window_state: WINDOW_STATE_ORDER,
    pub window_icon: WINDOW_ICON_ORDER,
    pub window_cached_icon: WINDOW_CACHED_ICON_ORDER,
    pub notify_icon_state: NOTIFY_ICON_STATE_ORDER,
    pub monitored_desktop: MONITORED_DESKTOP_ORDER,
}
pub type MONITORED_DESKTOP_ORDER = _MONITORED_DESKTOP_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _MONITORED_DESKTOP_ORDER {
    pub activeWindowId: UINT32,
    pub numWindowIds: UINT32,
    pub windowIds: *mut UINT32,
}
pub type NOTIFY_ICON_STATE_ORDER = _NOTIFY_ICON_STATE_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _NOTIFY_ICON_STATE_ORDER {
    pub version: UINT32,
    pub toolTip: RAIL_UNICODE_STRING,
    pub infoTip: NOTIFY_ICON_INFOTIP,
    pub state: UINT32,
    pub icon: ICON_INFO,
    pub cachedIcon: CACHED_ICON_INFO,
}
pub type CACHED_ICON_INFO = _CACHED_ICON_INFO;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _CACHED_ICON_INFO {
    pub cacheEntry: UINT32,
    pub cacheId: UINT32,
}
pub type ICON_INFO = _ICON_INFO;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _ICON_INFO {
    pub cacheEntry: UINT32,
    pub cacheId: UINT32,
    pub bpp: UINT32,
    pub width: UINT32,
    pub height: UINT32,
    pub cbColorTable: UINT32,
    pub cbBitsMask: UINT32,
    pub cbBitsColor: UINT32,
    pub bitsMask: *mut BYTE,
    pub colorTable: *mut BYTE,
    pub bitsColor: *mut BYTE,
}
pub type NOTIFY_ICON_INFOTIP = _NOTIFY_ICON_INFOTIP;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _NOTIFY_ICON_INFOTIP {
    pub timeout: UINT32,
    pub flags: UINT32,
    pub text: RAIL_UNICODE_STRING,
    pub title: RAIL_UNICODE_STRING,
}
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Remote Applications Integrated Locally (RAIL)
 *
 * Copyright 2011 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 * Copyright 2011 Roman Barabanov <romanbarabanov@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/* RAIL PDU flags */
/* Notification Icon Balloon Tooltip */
/* Client Execute PDU Flags */
/* Server Execute Result PDU */
/* Client System Parameters Update PDU */
/* Server System Parameters Update PDU */
/*Bit mask values for SPI_ parameters*/
/* Client System Command PDU */
/* Client Notify Event PDU */
/* Client Information PDU */
/* HIGHCONTRAST flags values */
/* Server Move/Size Start PDU */
/* Language Bar Information PDU */
/* Extended Handshake Flags */
/* Language Profile Information Flags */
pub type RAIL_UNICODE_STRING = _RAIL_UNICODE_STRING;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _RAIL_UNICODE_STRING {
    pub length: UINT16,
    pub string: *mut BYTE,
}
pub type WINDOW_CACHED_ICON_ORDER = _WINDOW_CACHED_ICON_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _WINDOW_CACHED_ICON_ORDER {
    pub cachedIcon: CACHED_ICON_INFO,
}
pub type WINDOW_ICON_ORDER = _WINDOW_ICON_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _WINDOW_ICON_ORDER {
    pub iconInfo: *mut ICON_INFO,
}
pub type WINDOW_STATE_ORDER = _WINDOW_STATE_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _WINDOW_STATE_ORDER {
    pub ownerWindowId: UINT32,
    pub style: UINT32,
    pub extendedStyle: UINT32,
    pub showState: UINT32,
    pub titleInfo: RAIL_UNICODE_STRING,
    pub clientOffsetX: INT32,
    pub clientOffsetY: INT32,
    pub clientAreaWidth: UINT32,
    pub clientAreaHeight: UINT32,
    pub RPContent: UINT32,
    pub rootParentHandle: UINT32,
    pub windowOffsetX: INT32,
    pub windowOffsetY: INT32,
    pub windowClientDeltaX: INT32,
    pub windowClientDeltaY: INT32,
    pub windowWidth: UINT32,
    pub windowHeight: UINT32,
    pub numWindowRects: UINT32,
    pub windowRects: *mut RECTANGLE_16,
    pub visibleOffsetX: INT32,
    pub visibleOffsetY: INT32,
    pub numVisibilityRects: UINT32,
    pub visibilityRects: *mut RECTANGLE_16,
}
pub type WINDOW_ORDER_INFO = _WINDOW_ORDER_INFO;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _WINDOW_ORDER_INFO {
    pub windowId: UINT32,
    pub fieldFlags: UINT32,
    pub notifyIconId: UINT32,
}
pub type pNonMonitoredDesktop
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut WINDOW_ORDER_INFO)
               -> BOOL>;
pub type pMonitoredDesktop
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut WINDOW_ORDER_INFO,
                                _: *mut MONITORED_DESKTOP_ORDER) -> BOOL>;
pub type pNotifyIconDelete
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut WINDOW_ORDER_INFO)
               -> BOOL>;
pub type pNotifyIconUpdate
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut WINDOW_ORDER_INFO,
                                _: *mut NOTIFY_ICON_STATE_ORDER) -> BOOL>;
pub type pNotifyIconCreate
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut WINDOW_ORDER_INFO,
                                _: *mut NOTIFY_ICON_STATE_ORDER) -> BOOL>;
pub type pWindowDelete
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut WINDOW_ORDER_INFO)
               -> BOOL>;
pub type pWindowCachedIcon
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut WINDOW_ORDER_INFO,
                                _: *mut WINDOW_CACHED_ICON_ORDER) -> BOOL>;
pub type pWindowIcon
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut WINDOW_ORDER_INFO,
                                _: *mut WINDOW_ICON_ORDER) -> BOOL>;
pub type pWindowUpdate
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut WINDOW_ORDER_INFO,
                                _: *mut WINDOW_STATE_ORDER) -> BOOL>;
pub type pWindowCreate
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut WINDOW_ORDER_INFO,
                                _: *mut WINDOW_STATE_ORDER) -> BOOL>;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Alternate Secondary Drawing Orders Interface API
 *
 * Copyright 2011 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/* 0 */
/* 1 */
/* 16 */
/* 17 */
/* 18 */
/* 19 */
/* 20 */
/* 21 */
/* 22 */
/* 23 */
/* 24 */
/* 25 */
/* 26 */
/* 27 */
/* 28 */
/* internal */
pub type rdpAltSecUpdate = rdp_altsec_update;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_altsec_update {
    pub context: *mut rdpContext,
    pub paddingA: [UINT32; 15],
    pub CreateOffscreenBitmap: pCreateOffscreenBitmap,
    pub SwitchSurface: pSwitchSurface,
    pub CreateNineGridBitmap: pCreateNineGridBitmap,
    pub FrameMarker: pFrameMarker,
    pub StreamBitmapFirst: pStreamBitmapFirst,
    pub StreamBitmapNext: pStreamBitmapNext,
    pub DrawGdiPlusFirst: pDrawGdiPlusFirst,
    pub DrawGdiPlusNext: pDrawGdiPlusNext,
    pub DrawGdiPlusEnd: pDrawGdiPlusEnd,
    pub DrawGdiPlusCacheFirst: pDrawGdiPlusCacheFirst,
    pub DrawGdiPlusCacheNext: pDrawGdiPlusCacheNext,
    pub DrawGdiPlusCacheEnd: pDrawGdiPlusCacheEnd,
    pub paddingB: [UINT32; 4],
    pub create_offscreen_bitmap: CREATE_OFFSCREEN_BITMAP_ORDER,
    pub switch_surface: SWITCH_SURFACE_ORDER,
    pub create_nine_grid_bitmap: CREATE_NINE_GRID_BITMAP_ORDER,
    pub frame_marker: FRAME_MARKER_ORDER,
    pub stream_bitmap_first: STREAM_BITMAP_FIRST_ORDER,
    pub stream_bitmap_next: STREAM_BITMAP_NEXT_ORDER,
    pub draw_gdiplus_cache_first: DRAW_GDIPLUS_CACHE_FIRST_ORDER,
    pub draw_gdiplus_cache_next: DRAW_GDIPLUS_CACHE_NEXT_ORDER,
    pub draw_gdiplus_cache_end: DRAW_GDIPLUS_CACHE_END_ORDER,
    pub draw_gdiplus_first: DRAW_GDIPLUS_FIRST_ORDER,
    pub draw_gdiplus_next: DRAW_GDIPLUS_NEXT_ORDER,
    pub draw_gdiplus_end: DRAW_GDIPLUS_END_ORDER,
}
pub type DRAW_GDIPLUS_END_ORDER = _DRAW_GDIPLUS_END_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _DRAW_GDIPLUS_END_ORDER {
    pub cbSize: UINT32,
    pub cbTotalSize: UINT32,
    pub cbTotalEmfSize: UINT32,
    pub emfRecords: *mut BYTE,
}
pub type DRAW_GDIPLUS_NEXT_ORDER = _DRAW_GDIPLUS_NEXT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _DRAW_GDIPLUS_NEXT_ORDER {
    pub cbSize: UINT32,
    pub emfRecords: *mut BYTE,
}
pub type DRAW_GDIPLUS_FIRST_ORDER = _DRAW_GDIPLUS_FIRST_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _DRAW_GDIPLUS_FIRST_ORDER {
    pub cbSize: UINT32,
    pub cbTotalSize: UINT32,
    pub cbTotalEmfSize: UINT32,
    pub emfRecords: *mut BYTE,
}
pub type DRAW_GDIPLUS_CACHE_END_ORDER = _DRAW_GDIPLUS_CACHE_END_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _DRAW_GDIPLUS_CACHE_END_ORDER {
    pub flags: UINT32,
    pub cacheType: UINT32,
    pub cacheIndex: UINT32,
    pub cbSize: UINT32,
    pub cbTotalSize: UINT32,
    pub emfRecords: *mut BYTE,
}
pub type DRAW_GDIPLUS_CACHE_NEXT_ORDER = _DRAW_GDIPLUS_CACHE_NEXT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _DRAW_GDIPLUS_CACHE_NEXT_ORDER {
    pub flags: UINT32,
    pub cacheType: UINT32,
    pub cacheIndex: UINT32,
    pub cbSize: UINT32,
    pub emfRecords: *mut BYTE,
}
pub type DRAW_GDIPLUS_CACHE_FIRST_ORDER = _DRAW_GDIPLUS_CACHE_FIRST_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _DRAW_GDIPLUS_CACHE_FIRST_ORDER {
    pub flags: UINT32,
    pub cacheType: UINT32,
    pub cacheIndex: UINT32,
    pub cbSize: UINT32,
    pub cbTotalSize: UINT32,
    pub emfRecords: *mut BYTE,
}
pub type STREAM_BITMAP_NEXT_ORDER = _STREAM_BITMAP_NEXT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _STREAM_BITMAP_NEXT_ORDER {
    pub bitmapFlags: UINT32,
    pub bitmapType: UINT32,
    pub bitmapBlockSize: UINT32,
    pub bitmapBlock: *mut BYTE,
}
pub type STREAM_BITMAP_FIRST_ORDER = _STREAM_BITMAP_FIRST_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _STREAM_BITMAP_FIRST_ORDER {
    pub bitmapFlags: UINT32,
    pub bitmapBpp: UINT32,
    pub bitmapType: UINT32,
    pub bitmapWidth: UINT32,
    pub bitmapHeight: UINT32,
    pub bitmapSize: UINT32,
    pub bitmapBlockSize: UINT32,
    pub bitmapBlock: *mut BYTE,
}
pub type FRAME_MARKER_ORDER = _FRAME_MARKER_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _FRAME_MARKER_ORDER {
    pub action: UINT32,
}
pub type CREATE_NINE_GRID_BITMAP_ORDER = _CREATE_NINE_GRID_BITMAP_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _CREATE_NINE_GRID_BITMAP_ORDER {
    pub bitmapBpp: UINT32,
    pub bitmapId: UINT32,
    pub cx: UINT32,
    pub cy: UINT32,
    pub nineGridInfo: NINE_GRID_BITMAP_INFO,
}
pub type NINE_GRID_BITMAP_INFO = _NINE_GRID_BITMAP_INFO;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _NINE_GRID_BITMAP_INFO {
    pub flFlags: UINT32,
    pub ulLeftWidth: UINT32,
    pub ulRightWidth: UINT32,
    pub ulTopHeight: UINT32,
    pub ulBottomHeight: UINT32,
    pub crTransparent: UINT32,
}
pub type SWITCH_SURFACE_ORDER = _SWITCH_SURFACE_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _SWITCH_SURFACE_ORDER {
    pub bitmapId: UINT32,
}
pub type CREATE_OFFSCREEN_BITMAP_ORDER = _CREATE_OFFSCREEN_BITMAP_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _CREATE_OFFSCREEN_BITMAP_ORDER {
    pub id: UINT32,
    pub cx: UINT32,
    pub cy: UINT32,
    pub deleteList: OFFSCREEN_DELETE_LIST,
}
pub type OFFSCREEN_DELETE_LIST = _OFFSCREEN_DELETE_LIST;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _OFFSCREEN_DELETE_LIST {
    pub sIndices: UINT32,
    pub cIndices: UINT32,
    pub indices: *mut UINT16,
}
pub type pDrawGdiPlusCacheEnd
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const DRAW_GDIPLUS_CACHE_END_ORDER)
               -> BOOL>;
pub type pDrawGdiPlusCacheNext
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const DRAW_GDIPLUS_CACHE_NEXT_ORDER)
               -> BOOL>;
pub type pDrawGdiPlusCacheFirst
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const DRAW_GDIPLUS_CACHE_FIRST_ORDER)
               -> BOOL>;
pub type pDrawGdiPlusEnd
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const DRAW_GDIPLUS_END_ORDER) -> BOOL>;
pub type pDrawGdiPlusNext
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const DRAW_GDIPLUS_NEXT_ORDER) -> BOOL>;
pub type pDrawGdiPlusFirst
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const DRAW_GDIPLUS_FIRST_ORDER) -> BOOL>;
pub type pStreamBitmapNext
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const STREAM_BITMAP_NEXT_ORDER) -> BOOL>;
pub type pStreamBitmapFirst
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const STREAM_BITMAP_FIRST_ORDER) -> BOOL>;
pub type pFrameMarker
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const FRAME_MARKER_ORDER) -> BOOL>;
pub type pCreateNineGridBitmap
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const CREATE_NINE_GRID_BITMAP_ORDER)
               -> BOOL>;
pub type pSwitchSurface
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const SWITCH_SURFACE_ORDER) -> BOOL>;
pub type pCreateOffscreenBitmap
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const CREATE_OFFSCREEN_BITMAP_ORDER)
               -> BOOL>;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Secondary Drawing Orders Interface API
 *
 * Copyright 2011 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/* 0 */
/* 1 */
/* 16 */
/* 17 */
/* 18 */
/* 19 */
/* 20 */
/* 21 */
/* 22 */
/* 23 */
/* internal */
pub type rdpSecondaryUpdate = rdp_secondary_update;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_secondary_update {
    pub context: *mut rdpContext,
    pub paddingA: [UINT32; 15],
    pub CacheBitmap: pCacheBitmap,
    pub CacheBitmapV2: pCacheBitmapV2,
    pub CacheBitmapV3: pCacheBitmapV3,
    pub CacheColorTable: pCacheColorTable,
    pub CacheGlyph: pCacheGlyph,
    pub CacheGlyphV2: pCacheGlyphV2,
    pub CacheBrush: pCacheBrush,
    pub paddingE: [UINT32; 9],
    pub glyph_v2: BOOL,
}
pub type pCacheBrush
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const CACHE_BRUSH_ORDER) -> BOOL>;
pub type CACHE_BRUSH_ORDER = _CACHE_BRUSH_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _CACHE_BRUSH_ORDER {
    pub index: UINT32,
    pub bpp: UINT32,
    pub cx: UINT32,
    pub cy: UINT32,
    pub style: UINT32,
    pub length: UINT32,
    pub data: [BYTE; 256],
}
pub type pCacheGlyphV2
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const CACHE_GLYPH_V2_ORDER) -> BOOL>;
pub type CACHE_GLYPH_V2_ORDER = _CACHE_GLYPH_V2_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _CACHE_GLYPH_V2_ORDER {
    pub cacheId: UINT32,
    pub flags: UINT32,
    pub cGlyphs: UINT32,
    pub glyphData: [GLYPH_DATA_V2; 256],
    pub unicodeCharacters: *mut WCHAR,
}
pub type GLYPH_DATA_V2 = _GLYPH_DATA_V2;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _GLYPH_DATA_V2 {
    pub cacheIndex: UINT32,
    pub x: INT32,
    pub y: INT32,
    pub cx: UINT32,
    pub cy: UINT32,
    pub cb: UINT32,
    pub aj: *mut BYTE,
}
pub type pCacheGlyph
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const CACHE_GLYPH_ORDER) -> BOOL>;
pub type CACHE_GLYPH_ORDER = _CACHE_GLYPH_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _CACHE_GLYPH_ORDER {
    pub cacheId: UINT32,
    pub cGlyphs: UINT32,
    pub glyphData: [GLYPH_DATA; 256],
    pub unicodeCharacters: *mut WCHAR,
}
pub type GLYPH_DATA = _GLYPH_DATA;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _GLYPH_DATA {
    pub cacheIndex: UINT32,
    pub x: INT16,
    pub y: INT16,
    pub cx: UINT32,
    pub cy: UINT32,
    pub cb: UINT32,
    pub aj: *mut BYTE,
}
pub type pCacheColorTable
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const CACHE_COLOR_TABLE_ORDER) -> BOOL>;
pub type CACHE_COLOR_TABLE_ORDER = _CACHE_COLOR_TABLE_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _CACHE_COLOR_TABLE_ORDER {
    pub cacheIndex: UINT32,
    pub numberColors: UINT32,
    pub colorTable: [UINT32; 256],
}
pub type pCacheBitmapV3
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *mut CACHE_BITMAP_V3_ORDER) -> BOOL>;
pub type CACHE_BITMAP_V3_ORDER = _CACHE_BITMAP_V3_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _CACHE_BITMAP_V3_ORDER {
    pub cacheId: UINT32,
    pub bpp: UINT32,
    pub flags: UINT32,
    pub cacheIndex: UINT32,
    pub key1: UINT32,
    pub key2: UINT32,
    pub bitmapData: BITMAP_DATA_EX,
}
pub type BITMAP_DATA_EX = _BITMAP_DATA_EX;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _BITMAP_DATA_EX {
    pub bpp: UINT32,
    pub codecID: UINT32,
    pub width: UINT32,
    pub height: UINT32,
    pub length: UINT32,
    pub data: *mut BYTE,
}
pub type pCacheBitmapV2
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *mut CACHE_BITMAP_V2_ORDER) -> BOOL>;
pub type CACHE_BITMAP_V2_ORDER = _CACHE_BITMAP_V2_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _CACHE_BITMAP_V2_ORDER {
    pub cacheId: UINT32,
    pub flags: UINT32,
    pub key1: UINT32,
    pub key2: UINT32,
    pub bitmapBpp: UINT32,
    pub bitmapWidth: UINT32,
    pub bitmapHeight: UINT32,
    pub bitmapLength: UINT32,
    pub cacheIndex: UINT32,
    pub compressed: BOOL,
    pub cbCompFirstRowSize: UINT32,
    pub cbCompMainBodySize: UINT32,
    pub cbScanWidth: UINT32,
    pub cbUncompressedSize: UINT32,
    pub bitmapDataStream: *mut BYTE,
}
pub type pCacheBitmap
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const CACHE_BITMAP_ORDER) -> BOOL>;
pub type CACHE_BITMAP_ORDER = _CACHE_BITMAP_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _CACHE_BITMAP_ORDER {
    pub cacheId: UINT32,
    pub bitmapBpp: UINT32,
    pub bitmapWidth: UINT32,
    pub bitmapHeight: UINT32,
    pub bitmapLength: UINT32,
    pub cacheIndex: UINT32,
    pub compressed: BOOL,
    pub bitmapComprHdr: [BYTE; 8],
    pub bitmapDataStream: *mut BYTE,
}
/* 0 */
/* 1 */
/* 16 */
/* 17 */
/* 18 */
/* 19 */
/* 20 */
/* 21 */
/* 22 */
/* 23 */
/* 24 */
/* 25 */
/* 26 */
/* 27 */
/* 28 */
/* 29 */
/* 30 */
/* 31 */
/* 32 */
/* 33 */
/* 34 */
/* 35 */
/* 36 */
/* 37 */
/* 38 */
/* internal */
pub type rdpPrimaryUpdate = rdp_primary_update;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_primary_update {
    pub context: *mut rdpContext,
    pub paddingA: [UINT32; 15],
    pub DstBlt: pDstBlt,
    pub PatBlt: pPatBlt,
    pub ScrBlt: pScrBlt,
    pub OpaqueRect: pOpaqueRect,
    pub DrawNineGrid: pDrawNineGrid,
    pub MultiDstBlt: pMultiDstBlt,
    pub MultiPatBlt: pMultiPatBlt,
    pub MultiScrBlt: pMultiScrBlt,
    pub MultiOpaqueRect: pMultiOpaqueRect,
    pub MultiDrawNineGrid: pMultiDrawNineGrid,
    pub LineTo: pLineTo,
    pub Polyline: pPolyline,
    pub MemBlt: pMemBlt,
    pub Mem3Blt: pMem3Blt,
    pub SaveBitmap: pSaveBitmap,
    pub GlyphIndex: pGlyphIndex,
    pub FastIndex: pFastIndex,
    pub FastGlyph: pFastGlyph,
    pub PolygonSC: pPolygonSC,
    pub PolygonCB: pPolygonCB,
    pub EllipseSC: pEllipseSC,
    pub EllipseCB: pEllipseCB,
    pub paddingB: [UINT32; 10],
    pub order_info: ORDER_INFO,
    pub dstblt: DSTBLT_ORDER,
    pub patblt: PATBLT_ORDER,
    pub scrblt: SCRBLT_ORDER,
    pub opaque_rect: OPAQUE_RECT_ORDER,
    pub draw_nine_grid: DRAW_NINE_GRID_ORDER,
    pub multi_dstblt: MULTI_DSTBLT_ORDER,
    pub multi_patblt: MULTI_PATBLT_ORDER,
    pub multi_scrblt: MULTI_SCRBLT_ORDER,
    pub multi_opaque_rect: MULTI_OPAQUE_RECT_ORDER,
    pub multi_draw_nine_grid: MULTI_DRAW_NINE_GRID_ORDER,
    pub line_to: LINE_TO_ORDER,
    pub polyline: POLYLINE_ORDER,
    pub memblt: MEMBLT_ORDER,
    pub mem3blt: MEM3BLT_ORDER,
    pub save_bitmap: SAVE_BITMAP_ORDER,
    pub glyph_index: GLYPH_INDEX_ORDER,
    pub fast_index: FAST_INDEX_ORDER,
    pub fast_glyph: FAST_GLYPH_ORDER,
    pub polygon_sc: POLYGON_SC_ORDER,
    pub polygon_cb: POLYGON_CB_ORDER,
    pub ellipse_sc: ELLIPSE_SC_ORDER,
    pub ellipse_cb: ELLIPSE_CB_ORDER,
}
pub type ELLIPSE_CB_ORDER = _ELLIPSE_CB_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _ELLIPSE_CB_ORDER {
    pub leftRect: INT32,
    pub topRect: INT32,
    pub rightRect: INT32,
    pub bottomRect: INT32,
    pub bRop2: UINT32,
    pub fillMode: UINT32,
    pub backColor: UINT32,
    pub foreColor: UINT32,
    pub brush: rdpBrush,
}
pub type rdpBrush = rdp_brush;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_brush {
    pub x: UINT32,
    pub y: UINT32,
    pub bpp: UINT32,
    pub style: UINT32,
    pub hatch: UINT32,
    pub index: UINT32,
    pub data: *mut BYTE,
    pub p8x8: [BYTE; 8],
}
pub type ELLIPSE_SC_ORDER = _ELLIPSE_SC_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _ELLIPSE_SC_ORDER {
    pub leftRect: INT32,
    pub topRect: INT32,
    pub rightRect: INT32,
    pub bottomRect: INT32,
    pub bRop2: UINT32,
    pub fillMode: UINT32,
    pub color: UINT32,
}
pub type POLYGON_CB_ORDER = _POLYGON_CB_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _POLYGON_CB_ORDER {
    pub xStart: INT32,
    pub yStart: INT32,
    pub bRop2: UINT32,
    pub backMode: UINT32,
    pub fillMode: UINT32,
    pub backColor: UINT32,
    pub foreColor: UINT32,
    pub brush: rdpBrush,
    pub numPoints: UINT32,
    pub cbData: UINT32,
    pub points: *mut DELTA_POINT,
}
pub type DELTA_POINT = _DELTA_POINT;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _DELTA_POINT {
    pub x: INT32,
    pub y: INT32,
}
pub type POLYGON_SC_ORDER = _POLYGON_SC_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _POLYGON_SC_ORDER {
    pub xStart: INT32,
    pub yStart: INT32,
    pub bRop2: UINT32,
    pub fillMode: UINT32,
    pub brushColor: UINT32,
    pub numPoints: UINT32,
    pub cbData: UINT32,
    pub points: *mut DELTA_POINT,
}
pub type FAST_GLYPH_ORDER = _FAST_GLYPH_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _FAST_GLYPH_ORDER {
    pub cacheId: UINT32,
    pub flAccel: UINT32,
    pub ulCharInc: UINT32,
    pub backColor: UINT32,
    pub foreColor: UINT32,
    pub bkLeft: INT32,
    pub bkTop: INT32,
    pub bkRight: INT32,
    pub bkBottom: INT32,
    pub opLeft: INT32,
    pub opTop: INT32,
    pub opRight: INT32,
    pub opBottom: INT32,
    pub x: INT32,
    pub y: INT32,
    pub cbData: UINT32,
    pub data: [BYTE; 256],
    pub glyphData: GLYPH_DATA_V2,
}
pub type FAST_INDEX_ORDER = _FAST_INDEX_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _FAST_INDEX_ORDER {
    pub cacheId: UINT32,
    pub flAccel: UINT32,
    pub ulCharInc: UINT32,
    pub backColor: UINT32,
    pub foreColor: UINT32,
    pub bkLeft: INT32,
    pub bkTop: INT32,
    pub bkRight: INT32,
    pub bkBottom: INT32,
    pub opLeft: INT32,
    pub opTop: INT32,
    pub opRight: INT32,
    pub opBottom: INT32,
    pub opaqueRect: BOOL,
    pub x: INT32,
    pub y: INT32,
    pub cbData: UINT32,
    pub data: [BYTE; 256],
}
pub type GLYPH_INDEX_ORDER = _GLYPH_INDEX_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _GLYPH_INDEX_ORDER {
    pub cacheId: UINT32,
    pub flAccel: UINT32,
    pub ulCharInc: UINT32,
    pub fOpRedundant: UINT32,
    pub backColor: UINT32,
    pub foreColor: UINT32,
    pub bkLeft: INT32,
    pub bkTop: INT32,
    pub bkRight: INT32,
    pub bkBottom: INT32,
    pub opLeft: INT32,
    pub opTop: INT32,
    pub opRight: INT32,
    pub opBottom: INT32,
    pub brush: rdpBrush,
    pub x: INT32,
    pub y: INT32,
    pub cbData: UINT32,
    pub data: [BYTE; 256],
}
pub type SAVE_BITMAP_ORDER = _SAVE_BITMAP_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _SAVE_BITMAP_ORDER {
    pub savedBitmapPosition: UINT32,
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nRightRect: INT32,
    pub nBottomRect: INT32,
    pub operation: UINT32,
}
pub type MEM3BLT_ORDER = _MEM3BLT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _MEM3BLT_ORDER {
    pub cacheId: UINT32,
    pub colorIndex: UINT32,
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nWidth: INT32,
    pub nHeight: INT32,
    pub bRop: UINT32,
    pub nXSrc: INT32,
    pub nYSrc: INT32,
    pub backColor: UINT32,
    pub foreColor: UINT32,
    pub brush: rdpBrush,
    pub cacheIndex: UINT32,
    pub bitmap: *mut rdpBitmap,
}
pub type rdpBitmap = rdp_bitmap;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_bitmap {
    pub size: size_t,
    pub New: pBitmap_New,
    pub Free: pBitmap_Free,
    pub Paint: pBitmap_Paint,
    pub Decompress: pBitmap_Decompress,
    pub SetSurface: pBitmap_SetSurface,
    pub paddingA: [UINT32; 10],
    pub left: UINT32,
    pub top: UINT32,
    pub right: UINT32,
    pub bottom: UINT32,
    pub width: UINT32,
    pub height: UINT32,
    pub format: UINT32,
    pub flags: UINT32,
    pub length: UINT32,
    pub data: *mut BYTE,
    pub paddingB: [UINT32; 6],
    pub compressed: BOOL,
    pub ephemeral: BOOL,
    pub paddingC: [UINT32; 30],
}
pub type pBitmap_SetSurface
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut rdpBitmap,
                                _: BOOL) -> BOOL>;
pub type pBitmap_Decompress
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut rdpBitmap,
                                _: *const BYTE, _: UINT32, _: UINT32,
                                _: UINT32, _: UINT32, _: BOOL, _: UINT32)
               -> BOOL>;
pub type pBitmap_Paint
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut rdpBitmap)
               -> BOOL>;
pub type pBitmap_Free
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut rdpBitmap) -> ()>;
pub type pBitmap_New
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut rdpBitmap)
               -> BOOL>;
pub type MEMBLT_ORDER = _MEMBLT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _MEMBLT_ORDER {
    pub cacheId: UINT32,
    pub colorIndex: UINT32,
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nWidth: INT32,
    pub nHeight: INT32,
    pub bRop: UINT32,
    pub nXSrc: INT32,
    pub nYSrc: INT32,
    pub cacheIndex: UINT32,
    pub bitmap: *mut rdpBitmap,
}
pub type POLYLINE_ORDER = _POLYLINE_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _POLYLINE_ORDER {
    pub xStart: INT32,
    pub yStart: INT32,
    pub bRop2: UINT32,
    pub penColor: UINT32,
    pub numDeltaEntries: UINT32,
    pub cbData: UINT32,
    pub points: *mut DELTA_POINT,
}
pub type LINE_TO_ORDER = _LINE_TO_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _LINE_TO_ORDER {
    pub backMode: UINT32,
    pub nXStart: INT32,
    pub nYStart: INT32,
    pub nXEnd: INT32,
    pub nYEnd: INT32,
    pub backColor: UINT32,
    pub bRop2: UINT32,
    pub penStyle: UINT32,
    pub penWidth: UINT32,
    pub penColor: UINT32,
}
pub type MULTI_DRAW_NINE_GRID_ORDER = _MULTI_DRAW_NINE_GRID_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _MULTI_DRAW_NINE_GRID_ORDER {
    pub srcLeft: INT32,
    pub srcTop: INT32,
    pub srcRight: INT32,
    pub srcBottom: INT32,
    pub bitmapId: UINT32,
    pub nDeltaEntries: UINT32,
    pub cbData: UINT32,
    pub rectangles: [DELTA_RECT; 45],
}
pub type DELTA_RECT = _DELTA_RECT;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _DELTA_RECT {
    pub left: INT32,
    pub top: INT32,
    pub width: INT32,
    pub height: INT32,
}
pub type MULTI_OPAQUE_RECT_ORDER = _MULTI_OPAQUE_RECT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _MULTI_OPAQUE_RECT_ORDER {
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nWidth: INT32,
    pub nHeight: INT32,
    pub color: UINT32,
    pub numRectangles: UINT32,
    pub cbData: UINT32,
    pub rectangles: [DELTA_RECT; 45],
}
pub type MULTI_SCRBLT_ORDER = _MULTI_SCRBLT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _MULTI_SCRBLT_ORDER {
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nWidth: INT32,
    pub nHeight: INT32,
    pub bRop: UINT32,
    pub nXSrc: INT32,
    pub nYSrc: INT32,
    pub numRectangles: UINT32,
    pub cbData: UINT32,
    pub rectangles: [DELTA_RECT; 45],
}
pub type MULTI_PATBLT_ORDER = _MULTI_PATBLT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _MULTI_PATBLT_ORDER {
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nWidth: INT32,
    pub nHeight: INT32,
    pub bRop: UINT32,
    pub backColor: UINT32,
    pub foreColor: UINT32,
    pub brush: rdpBrush,
    pub numRectangles: UINT32,
    pub cbData: UINT32,
    pub rectangles: [DELTA_RECT; 45],
}
pub type MULTI_DSTBLT_ORDER = _MULTI_DSTBLT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _MULTI_DSTBLT_ORDER {
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nWidth: INT32,
    pub nHeight: INT32,
    pub bRop: UINT32,
    pub numRectangles: UINT32,
    pub cbData: UINT32,
    pub rectangles: [DELTA_RECT; 45],
}
pub type DRAW_NINE_GRID_ORDER = _DRAW_NINE_GRID_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _DRAW_NINE_GRID_ORDER {
    pub srcLeft: INT32,
    pub srcTop: INT32,
    pub srcRight: INT32,
    pub srcBottom: INT32,
    pub bitmapId: UINT32,
}
pub type OPAQUE_RECT_ORDER = _OPAQUE_RECT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _OPAQUE_RECT_ORDER {
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nWidth: INT32,
    pub nHeight: INT32,
    pub color: UINT32,
}
pub type SCRBLT_ORDER = _SCRBLT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _SCRBLT_ORDER {
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nWidth: INT32,
    pub nHeight: INT32,
    pub bRop: UINT32,
    pub nXSrc: INT32,
    pub nYSrc: INT32,
}
pub type PATBLT_ORDER = _PATBLT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _PATBLT_ORDER {
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nWidth: INT32,
    pub nHeight: INT32,
    pub bRop: UINT32,
    pub backColor: UINT32,
    pub foreColor: UINT32,
    pub brush: rdpBrush,
}
pub type DSTBLT_ORDER = _DSTBLT_ORDER;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _DSTBLT_ORDER {
    pub nLeftRect: INT32,
    pub nTopRect: INT32,
    pub nWidth: INT32,
    pub nHeight: INT32,
    pub bRop: UINT32,
}
pub type ORDER_INFO = _ORDER_INFO;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _ORDER_INFO {
    pub controlFlags: UINT32,
    pub orderType: UINT32,
    pub fieldFlags: UINT32,
    pub boundsFlags: UINT32,
    pub bounds: rdpBounds,
    pub deltaCoordinates: BOOL,
}
pub type pEllipseCB
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const ELLIPSE_CB_ORDER) -> BOOL>;
pub type pEllipseSC
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const ELLIPSE_SC_ORDER) -> BOOL>;
pub type pPolygonCB
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut POLYGON_CB_ORDER)
               -> BOOL>;
pub type pPolygonSC
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const POLYGON_SC_ORDER) -> BOOL>;
pub type pFastGlyph
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const FAST_GLYPH_ORDER) -> BOOL>;
pub type pFastIndex
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const FAST_INDEX_ORDER) -> BOOL>;
pub type pGlyphIndex
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut GLYPH_INDEX_ORDER)
               -> BOOL>;
pub type pSaveBitmap
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const SAVE_BITMAP_ORDER) -> BOOL>;
pub type pMem3Blt
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut MEM3BLT_ORDER)
               -> BOOL>;
pub type pMemBlt
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut MEMBLT_ORDER)
               -> BOOL>;
pub type pPolyline
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *const POLYLINE_ORDER)
               -> BOOL>;
pub type pLineTo
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *const LINE_TO_ORDER)
               -> BOOL>;
pub type pMultiDrawNineGrid
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const MULTI_DRAW_NINE_GRID_ORDER)
               -> BOOL>;
pub type pMultiOpaqueRect
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const MULTI_OPAQUE_RECT_ORDER) -> BOOL>;
pub type pMultiScrBlt
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const MULTI_SCRBLT_ORDER) -> BOOL>;
pub type pMultiPatBlt
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const MULTI_PATBLT_ORDER) -> BOOL>;
pub type pMultiDstBlt
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const MULTI_DSTBLT_ORDER) -> BOOL>;
pub type pDrawNineGrid
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const DRAW_NINE_GRID_ORDER) -> BOOL>;
pub type pOpaqueRect
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const OPAQUE_RECT_ORDER) -> BOOL>;
pub type pScrBlt
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *const SCRBLT_ORDER)
               -> BOOL>;
pub type pPatBlt
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut PATBLT_ORDER)
               -> BOOL>;
pub type pDstBlt
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *const DSTBLT_ORDER)
               -> BOOL>;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Pointer Updates Interface API
 *
 * Copyright 2011 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/* 0 */
/* 1 */
/* 16 */
/* 17 */
/* 18 */
/* 19 */
/* 20 */
/* 21 */
pub type rdpPointerUpdate = rdp_pointer_update;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_pointer_update {
    pub context: *mut rdpContext,
    pub paddingA: [UINT32; 15],
    pub PointerPosition: pPointerPosition,
    pub PointerSystem: pPointerSystem,
    pub PointerColor: pPointerColor,
    pub PointerNew: pPointerNew,
    pub PointerCached: pPointerCached,
    pub paddingB: [UINT32; 11],
}
pub type pPointerCached
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const POINTER_CACHED_UPDATE) -> BOOL>;
pub type POINTER_CACHED_UPDATE = _POINTER_CACHED_UPDATE;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _POINTER_CACHED_UPDATE {
    pub cacheIndex: UINT32,
}
pub type pPointerNew
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const POINTER_NEW_UPDATE) -> BOOL>;
pub type POINTER_NEW_UPDATE = _POINTER_NEW_UPDATE;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _POINTER_NEW_UPDATE {
    pub xorBpp: UINT32,
    pub colorPtrAttr: POINTER_COLOR_UPDATE,
}
pub type POINTER_COLOR_UPDATE = _POINTER_COLOR_UPDATE;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _POINTER_COLOR_UPDATE {
    pub cacheIndex: UINT32,
    pub xPos: UINT32,
    pub yPos: UINT32,
    pub width: UINT32,
    pub height: UINT32,
    pub lengthAndMask: UINT32,
    pub lengthXorMask: UINT32,
    pub xorMaskData: *mut BYTE,
    pub andMaskData: *mut BYTE,
}
pub type pPointerColor
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const POINTER_COLOR_UPDATE) -> BOOL>;
pub type pPointerSystem
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const POINTER_SYSTEM_UPDATE) -> BOOL>;
pub type POINTER_SYSTEM_UPDATE = _POINTER_SYSTEM_UPDATE;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _POINTER_SYSTEM_UPDATE {
    pub type_0: UINT32,
}
pub type pPointerPosition
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const POINTER_POSITION_UPDATE) -> BOOL>;
pub type POINTER_POSITION_UPDATE = _POINTER_POSITION_UPDATE;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _POINTER_POSITION_UPDATE {
    pub xPos: UINT32,
    pub yPos: UINT32,
}
pub type pSetKeyboardImeStatus
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT16, _: UINT32,
                                _: UINT32) -> BOOL>;
pub type pSetKeyboardIndicators
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT16) -> BOOL>;
pub type pPlaySound
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext,
                                _: *const PLAY_SOUND_UPDATE) -> BOOL>;
pub type PLAY_SOUND_UPDATE = _PLAY_SOUND_UPDATE;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _PLAY_SOUND_UPDATE {
    pub duration: UINT32,
    pub frequency: UINT32,
}
pub type pPalette
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *const PALETTE_UPDATE)
               -> BOOL>;
pub type PALETTE_UPDATE = _PALETTE_UPDATE;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _PALETTE_UPDATE {
    pub number: UINT32,
    pub entries: [PALETTE_ENTRY; 256],
}
pub type pBitmapUpdate
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *const BITMAP_UPDATE)
               -> BOOL>;
pub type BITMAP_UPDATE = _BITMAP_UPDATE;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _BITMAP_UPDATE {
    pub count: UINT32,
    pub number: UINT32,
    pub rectangles: *mut BITMAP_DATA,
    pub skipCompression: BOOL,
}
pub type BITMAP_DATA = _BITMAP_DATA;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _BITMAP_DATA {
    pub destLeft: UINT32,
    pub destTop: UINT32,
    pub destRight: UINT32,
    pub destBottom: UINT32,
    pub width: UINT32,
    pub height: UINT32,
    pub bitsPerPixel: UINT32,
    pub flags: UINT32,
    pub bitmapLength: UINT32,
    pub cbCompFirstRowSize: UINT32,
    pub cbCompMainBodySize: UINT32,
    pub cbScanWidth: UINT32,
    pub cbUncompressedSize: UINT32,
    pub bitmapDataStream: *mut BYTE,
    pub compressed: BOOL,
}
pub type pDesktopResize
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext) -> BOOL>;
pub type pSynchronize
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext) -> BOOL>;
pub type pSetBounds
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *const rdpBounds)
               -> BOOL>;
pub type pEndPaint = Option<unsafe extern "C" fn(_: *mut rdpContext) -> BOOL>;
pub type pBeginPaint
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext) -> BOOL>;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * Input Interface API
 *
 * Copyright 2011 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub type rdpInput = rdp_input;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_input {
    pub context: *mut rdpContext,
    pub param1: *mut libc::c_void,
    pub paddingA: [UINT32; 14],
    pub SynchronizeEvent: pSynchronizeEvent,
    pub KeyboardEvent: pKeyboardEvent,
    pub UnicodeKeyboardEvent: pUnicodeKeyboardEvent,
    pub MouseEvent: pMouseEvent,
    pub ExtendedMouseEvent: pExtendedMouseEvent,
    pub FocusInEvent: pFocusInEvent,
    pub KeyboardPauseEvent: pKeyboardPauseEvent,
    pub paddingB: [UINT32; 9],
    pub asynchronous: BOOL,
    pub proxy: *mut rdpInputProxy,
    pub queue: *mut wMessageQueue,
}
pub type rdpInputProxy = rdp_input_proxy;
pub type pKeyboardPauseEvent
    =
    Option<unsafe extern "C" fn(_: *mut rdpInput) -> BOOL>;
pub type pFocusInEvent
    =
    Option<unsafe extern "C" fn(_: *mut rdpInput, _: UINT16) -> BOOL>;
pub type pExtendedMouseEvent
    =
    Option<unsafe extern "C" fn(_: *mut rdpInput, _: UINT16, _: UINT16,
                                _: UINT16) -> BOOL>;
pub type pMouseEvent
    =
    Option<unsafe extern "C" fn(_: *mut rdpInput, _: UINT16, _: UINT16,
                                _: UINT16) -> BOOL>;
pub type pUnicodeKeyboardEvent
    =
    Option<unsafe extern "C" fn(_: *mut rdpInput, _: UINT16, _: UINT16)
               -> BOOL>;
pub type pKeyboardEvent
    =
    Option<unsafe extern "C" fn(_: *mut rdpInput, _: UINT16, _: UINT16)
               -> BOOL>;
pub type pSynchronizeEvent
    =
    Option<unsafe extern "C" fn(_: *mut rdpInput, _: UINT32) -> BOOL>;
pub type rdpGraphics = rdp_graphics;
pub type freerdp_peer = rdp_freerdp_peer;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_freerdp_peer {
    pub context: *mut rdpContext,
    pub sockfd: libc::c_int,
    pub hostname: [libc::c_char; 50],
    pub input: *mut rdpInput,
    pub update: *mut rdpUpdate,
    pub settings: *mut rdpSettings,
    pub autodetect: *mut rdpAutoDetect,
    pub ContextExtra: *mut libc::c_void,
    pub ContextSize: size_t,
    pub ContextNew: psPeerContextNew,
    pub ContextFree: psPeerContextFree,
    pub Initialize: psPeerInitialize,
    pub GetFileDescriptor: psPeerGetFileDescriptor,
    pub GetEventHandle: psPeerGetEventHandle,
    pub GetReceiveEventHandle: psPeerGetReceiveEventHandle,
    pub CheckFileDescriptor: psPeerCheckFileDescriptor,
    pub Close: psPeerClose,
    pub Disconnect: psPeerDisconnect,
    pub Capabilities: psPeerCapabilities,
    pub PostConnect: psPeerPostConnect,
    pub Activate: psPeerActivate,
    pub Logon: psPeerLogon,
    pub SendChannelData: psPeerSendChannelData,
    pub ReceiveChannelData: psPeerReceiveChannelData,
    pub VirtualChannelOpen: psPeerVirtualChannelOpen,
    pub VirtualChannelClose: psPeerVirtualChannelClose,
    pub VirtualChannelRead: psPeerVirtualChannelRead,
    pub VirtualChannelWrite: psPeerVirtualChannelWrite,
    pub VirtualChannelGetData: psPeerVirtualChannelGetData,
    pub VirtualChannelSetData: psPeerVirtualChannelSetData,
    pub pId: libc::c_int,
    pub ack_frame_id: UINT32,
    pub local: BOOL,
    pub connected: BOOL,
    pub activated: BOOL,
    pub authenticated: BOOL,
    pub identity: SEC_WINNT_AUTH_IDENTITY,
    pub IsWriteBlocked: psPeerIsWriteBlocked,
    pub DrainOutputBuffer: psPeerDrainOutputBuffer,
    pub HasMoreToRead: psPeerHasMoreToRead,
    pub GetEventHandles: psPeerGetEventHandles,
    pub AdjustMonitorsLayout: psPeerAdjustMonitorsLayout,
    pub ClientCapabilities: psPeerClientCapabilities,
    pub ComputeNtlmHash: psPeerComputeNtlmHash,
}
/* *
 * WinPR: Windows Portable Runtime
 * NTLM Utils
 *
 * Copyright 2012 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub type psPeerComputeNtlmHash
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                _: *const SEC_WINNT_AUTH_IDENTITY,
                                _: *const SecBuffer, _: *const BYTE,
                                _: *const BYTE, _: *const SecBuffer,
                                _: *mut BYTE) -> SECURITY_STATUS>;
/* *
 * WinPR: Windows Portable Runtime
 * Security Support Provider Interface (SSPI)
 *
 * Copyright 2012-2014 Marc-Andre Moreau <marcandre.moreau@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/* Security Context Attributes */
/* Security Credentials Attributes */
/* InitializeSecurityContext Flags */
/* AcceptSecurityContext Flags */
/* TSPasswordCreds */
/* TSPasswordCreds */
/* TSPasswordCreds */
/* _AUTH_IDENTITY_DEFINED */
/* Buffer Types */
/* Security Buffer Flags */
pub type SecBuffer = _SecBuffer;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _SecBuffer {
    pub cbBuffer: ULONG,
    pub BufferType: ULONG,
    pub pvBuffer: *mut libc::c_void,
}
pub type SEC_WINNT_AUTH_IDENTITY = _SEC_WINNT_AUTH_IDENTITY;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _SEC_WINNT_AUTH_IDENTITY {
    pub User: *mut UINT16,
    pub UserLength: UINT32,
    pub Domain: *mut UINT16,
    pub DomainLength: UINT32,
    pub Password: *mut UINT16,
    pub PasswordLength: UINT32,
    pub Flags: UINT32,
}
pub type SECURITY_STATUS = LONG;
pub type psPeerClientCapabilities
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> BOOL>;
pub type psPeerAdjustMonitorsLayout
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> BOOL>;
pub type psPeerGetEventHandles
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: *mut HANDLE,
                                _: DWORD) -> DWORD>;
pub type psPeerHasMoreToRead
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> BOOL>;
pub type psPeerDrainOutputBuffer
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> libc::c_int>;
pub type psPeerIsWriteBlocked
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> BOOL>;
pub type psPeerVirtualChannelSetData
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: HANDLE,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type psPeerVirtualChannelGetData
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: HANDLE)
               -> *mut libc::c_void>;
pub type psPeerVirtualChannelWrite
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: HANDLE, _: *mut BYTE,
                                _: UINT32) -> libc::c_int>;
pub type psPeerVirtualChannelRead
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: HANDLE, _: *mut BYTE,
                                _: UINT32) -> libc::c_int>;
pub type psPeerVirtualChannelClose
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: HANDLE) -> BOOL>;
pub type psPeerVirtualChannelOpen
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: *const libc::c_char,
                                _: UINT32) -> HANDLE>;
pub type psPeerReceiveChannelData
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: UINT16,
                                _: *const BYTE, _: libc::c_int,
                                _: libc::c_int, _: libc::c_int)
               -> libc::c_int>;
pub type psPeerSendChannelData
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: UINT16,
                                _: *const BYTE, _: libc::c_int)
               -> libc::c_int>;
pub type psPeerLogon
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer,
                                _: *mut SEC_WINNT_AUTH_IDENTITY, _: BOOL)
               -> BOOL>;
pub type psPeerActivate
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> BOOL>;
pub type psPeerPostConnect
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> BOOL>;
pub type psPeerCapabilities
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> BOOL>;
pub type psPeerDisconnect
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> ()>;
pub type psPeerClose
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> BOOL>;
pub type psPeerCheckFileDescriptor
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> BOOL>;
pub type psPeerGetReceiveEventHandle
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> HANDLE>;
pub type psPeerGetEventHandle
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> HANDLE>;
pub type psPeerGetFileDescriptor
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer,
                                _: *mut *mut libc::c_void,
                                _: *mut libc::c_int) -> BOOL>;
pub type psPeerInitialize
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer) -> BOOL>;
pub type psPeerContextFree
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: *mut rdpContext)
               -> ()>;
pub type psPeerContextNew
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_peer, _: *mut rdpContext)
               -> BOOL>;
pub type freerdp = rdp_freerdp;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_freerdp {
    pub context: *mut rdpContext,
    pub pClientEntryPoints: *mut RDP_CLIENT_ENTRY_POINTS,
    pub paddingA: [UINT64; 14],
    pub input: *mut rdpInput,
    pub update: *mut rdpUpdate,
    pub settings: *mut rdpSettings,
    pub autodetect: *mut rdpAutoDetect,
    pub paddingB: [UINT64; 12],
    pub ContextSize: size_t,
    pub ContextNew: pContextNew,
    pub ContextFree: pContextFree,
    pub paddingC: [UINT64; 12],
    pub ConnectionCallbackState: UINT,
    pub PreConnect: pPreConnect,
    pub PostConnect: pPostConnect,
    pub Authenticate: pAuthenticate,
    pub VerifyCertificate: pVerifyCertificate,
    pub VerifyChangedCertificate: pVerifyChangedCertificate,
    pub VerifyX509Certificate: pVerifyX509Certificate,
    pub LogonErrorInfo: pLogonErrorInfo,
    pub PostDisconnect: pPostDisconnect,
    pub GatewayAuthenticate: pAuthenticate,
    pub paddingD: [UINT64; 7],
    pub SendChannelData: pSendChannelData,
    pub ReceiveChannelData: pReceiveChannelData,
    pub paddingE: [UINT64; 14],
}
pub type pReceiveChannelData
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: UINT16, _: *mut BYTE,
                                _: libc::c_int, _: libc::c_int,
                                _: libc::c_int) -> libc::c_int>;
pub type pSendChannelData
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: UINT16, _: *mut BYTE,
                                _: libc::c_int) -> libc::c_int>;
pub type pAuthenticate
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: *mut *mut libc::c_char,
                                _: *mut *mut libc::c_char,
                                _: *mut *mut libc::c_char) -> BOOL>;
pub type pPostDisconnect
    =
    Option<unsafe extern "C" fn(_: *mut freerdp) -> ()>;
pub type pLogonErrorInfo
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: UINT32, _: UINT32)
               -> libc::c_int>;
pub type pVerifyX509Certificate
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: *mut BYTE, _: libc::c_int,
                                _: *const libc::c_char, _: libc::c_int,
                                _: DWORD) -> libc::c_int>;
pub type pVerifyChangedCertificate
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: *const libc::c_char,
                                _: *const libc::c_char,
                                _: *const libc::c_char,
                                _: *const libc::c_char,
                                _: *const libc::c_char,
                                _: *const libc::c_char,
                                _: *const libc::c_char) -> DWORD>;
pub type pVerifyCertificate
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: *const libc::c_char,
                                _: *const libc::c_char,
                                _: *const libc::c_char,
                                _: *const libc::c_char, _: BOOL) -> DWORD>;
pub type pPostConnect = Option<unsafe extern "C" fn(_: *mut freerdp) -> BOOL>;
pub type pPreConnect = Option<unsafe extern "C" fn(_: *mut freerdp) -> BOOL>;
pub type pContextFree
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: *mut rdpContext) -> ()>;
pub type pContextNew
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: *mut rdpContext) -> BOOL>;
pub type RDP_CLIENT_ENTRY_POINTS = RDP_CLIENT_ENTRY_POINTS_V1;
pub type RDP_CLIENT_ENTRY_POINTS_V1 = rdp_client_entry_points_v1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_client_entry_points_v1 {
    pub Size: DWORD,
    pub Version: DWORD,
    pub settings: *mut rdpSettings,
    pub GlobalInit: pRdpGlobalInit,
    pub GlobalUninit: pRdpGlobalUninit,
    pub ContextSize: DWORD,
    pub ClientNew: pRdpClientNew,
    pub ClientFree: pRdpClientFree,
    pub ClientStart: pRdpClientStart,
    pub ClientStop: pRdpClientStop,
}
pub type pRdpClientStop
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext) -> libc::c_int>;
pub type pRdpClientStart
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext) -> libc::c_int>;
pub type pRdpClientFree
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: *mut rdpContext) -> ()>;
pub type pRdpClientNew
    =
    Option<unsafe extern "C" fn(_: *mut freerdp, _: *mut rdpContext) -> BOOL>;
pub type pRdpGlobalUninit = Option<unsafe extern "C" fn() -> ()>;
pub type pRdpGlobalInit = Option<unsafe extern "C" fn() -> BOOL>;
pub type pGlyph_EndDraw
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: INT32, _: INT32,
                                _: INT32, _: INT32, _: UINT32, _: UINT32)
               -> BOOL>;
pub type pGlyph_BeginDraw
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: INT32, _: INT32,
                                _: INT32, _: INT32, _: UINT32, _: UINT32,
                                _: BOOL) -> BOOL>;
pub type pGlyph_Draw
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *const rdpGlyph,
                                _: INT32, _: INT32, _: INT32, _: INT32,
                                _: INT32, _: INT32, _: BOOL) -> BOOL>;
pub type pGlyph_Free
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut rdpGlyph) -> ()>;
pub type pGlyph_New
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *const rdpGlyph)
               -> BOOL>;
pub type rdpPointer = rdp_pointer;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_pointer {
    pub size: size_t,
    pub New: pPointer_New,
    pub Free: pPointer_Free,
    pub Set: pPointer_Set,
    pub SetNull: pPointer_SetNull,
    pub SetDefault: pPointer_SetDefault,
    pub SetPosition: pPointer_SetPosition,
    pub paddingA: [UINT32; 9],
    pub xPos: UINT32,
    pub yPos: UINT32,
    pub width: UINT32,
    pub height: UINT32,
    pub xorBpp: UINT32,
    pub lengthAndMask: UINT32,
    pub lengthXorMask: UINT32,
    pub xorMaskData: *mut BYTE,
    pub andMaskData: *mut BYTE,
    pub paddingB: [UINT32; 7],
}
pub type pPointer_SetPosition
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: UINT32, _: UINT32)
               -> BOOL>;
pub type pPointer_SetDefault
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext) -> BOOL>;
pub type pPointer_SetNull
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext) -> BOOL>;
pub type pPointer_Set
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *const rdpPointer)
               -> BOOL>;
pub type pPointer_Free
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut rdpPointer)
               -> ()>;
pub type pPointer_New
    =
    Option<unsafe extern "C" fn(_: *mut rdpContext, _: *mut rdpPointer)
               -> BOOL>;
/* *
 * FreeRDP: A Remote Desktop Protocol Implementation
 * RDP Server Listener
 *
 * Copyright 2011 Vic Lee
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rdp_freerdp_listener {
    pub info: *mut libc::c_void,
    pub listener: *mut libc::c_void,
    pub param1: *mut libc::c_void,
    pub param2: *mut libc::c_void,
    pub param3: *mut libc::c_void,
    pub param4: *mut libc::c_void,
    pub Open: psListenerOpen,
    pub OpenLocal: psListenerOpenLocal,
    pub GetFileDescriptor: psListenerGetFileDescriptor,
    pub GetEventHandles: psListenerGetEventHandles,
    pub CheckFileDescriptor: psListenerCheckFileDescriptor,
    pub Close: psListenerClose,
    pub PeerAccepted: psPeerAccepted,
    pub OpenFromSocket: psListenerOpenFromSocket,
}
pub type psListenerOpenFromSocket
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_listener, _: libc::c_int)
               -> BOOL>;
pub type freerdp_listener = rdp_freerdp_listener;
pub type psPeerAccepted
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_listener,
                                _: *mut freerdp_peer) -> BOOL>;
pub type psListenerClose
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_listener) -> ()>;
pub type psListenerCheckFileDescriptor
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_listener) -> BOOL>;
pub type psListenerGetEventHandles
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_listener, _: *mut HANDLE,
                                _: DWORD) -> DWORD>;
pub type psListenerGetFileDescriptor
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_listener,
                                _: *mut *mut libc::c_void,
                                _: *mut libc::c_int) -> BOOL>;
pub type psListenerOpenLocal
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_listener,
                                _: *const libc::c_char) -> BOOL>;
pub type psListenerOpen
    =
    Option<unsafe extern "C" fn(_: *mut freerdp_listener,
                                _: *const libc::c_char, _: UINT16) -> BOOL>;
pub type pixman_image_t = pixman_image;
/*
 * 32 bit regions
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_session {
    pub impl_0: *const session_impl,
    pub session_signal: wl_signal,
    pub active: bool,
    pub vtnr: libc::c_uint,
    pub seat: [libc::c_char; 256],
    pub udev: *mut udev,
    pub mon: *mut udev_monitor,
    pub udev_event: *mut wl_event_source,
    pub devices: wl_list,
    pub display_destroy: wl_listener,
    pub events: C2RustUnnamed_8,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub destroy: wl_signal,
}
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type EGLenum = libc::c_uint;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_drm_format {
    pub format: uint32_t,
    pub len: size_t,
    pub cap: size_t,
    pub modifiers: [uint64_t; 0],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub cap: size_t,
    pub formats: *mut *mut wlr_drm_format,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_egl {
    pub platform: EGLenum,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
    pub exts_str: *const libc::c_char,
    pub exts: C2RustUnnamed_9,
    pub wl_display: *mut wl_display,
    pub dmabuf_formats: wlr_drm_format_set,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub bind_wayland_display_wl: bool,
    pub buffer_age_ext: bool,
    pub image_base_khr: bool,
    pub image_dma_buf_export_mesa: bool,
    pub image_dmabuf_import_ext: bool,
    pub image_dmabuf_import_modifiers_ext: bool,
    pub swap_buffers_with_damage_ext: bool,
    pub swap_buffers_with_damage_khr: bool,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_backend_impl {
    pub start: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut wlr_backend) -> ()>,
    pub get_renderer: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                 -> *mut wlr_renderer>,
    pub get_session: Option<unsafe extern "C" fn(_: *mut wlr_backend)
                                -> *mut wlr_session>,
    pub get_presentation_clock: Option<unsafe extern "C" fn(_:
                                                                *mut wlr_backend)
                                           -> clockid_t>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_backend {
    pub impl_0: *const wlr_backend_impl,
    pub events: C2RustUnnamed_10,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
pub type wl_output_subpixel = libc::c_uint;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: wl_output_subpixel = 5;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: wl_output_subpixel = 4;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: wl_output_subpixel = 3;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: wl_output_subpixel = 2;
pub const WL_OUTPUT_SUBPIXEL_NONE: wl_output_subpixel = 1;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: wl_output_subpixel = 0;
pub type wl_output_transform = libc::c_uint;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
/* *
 * A client buffer.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_buffer {
    pub resource: *mut wl_resource,
    pub texture: *mut wlr_texture,
    pub released: bool,
    pub n_refs: size_t,
    pub resource_destroy: wl_listener,
}
/*
 * This an unstable interface of wlroots. No guarantees are made regarding the
 * future consistency of this API.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_cursor {
    pub output: *mut wlr_output,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub enabled: bool,
    pub visible: bool,
    pub width: uint32_t,
    pub height: uint32_t,
    pub hotspot_x: int32_t,
    pub hotspot_y: int32_t,
    pub link: wl_list,
    pub texture: *mut wlr_texture,
    pub surface: *mut wlr_surface,
    pub surface_commit: wl_listener,
    pub surface_destroy: wl_listener,
    pub events: C2RustUnnamed_11,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub destroy: wl_signal,
}
/* *
 * A compositor output region. This typically corresponds to a monitor that
 * displays part of the compositor space.
 *
 * The `frame` event will be emitted when it is a good time for the compositor
 * to submit a new frame.
 *
 * To render a new frame, compositors should call `wlr_output_attach_render`,
 * render and call `wlr_output_commit`. No rendering should happen outside a
 * `frame` event handler or before `wlr_output_attach_render`.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output {
    pub impl_0: *const wlr_output_impl,
    pub backend: *mut wlr_backend,
    pub display: *mut wl_display,
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub name: [libc::c_char; 24],
    pub make: [libc::c_char; 56],
    pub model: [libc::c_char; 16],
    pub serial: [libc::c_char; 16],
    pub phys_width: int32_t,
    pub phys_height: int32_t,
    pub modes: wl_list,
    pub current_mode: *mut wlr_output_mode,
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub enabled: bool,
    pub scale: libc::c_float,
    pub subpixel: wl_output_subpixel,
    pub transform: wl_output_transform,
    pub needs_frame: bool,
    pub damage: pixman_region32_t,
    pub frame_pending: bool,
    pub transform_matrix: [libc::c_float; 9],
    pub pending: wlr_output_state,
    pub commit_seq: uint32_t,
    pub events: C2RustUnnamed_12,
    pub idle_frame: *mut wl_event_source,
    pub idle_done: *mut wl_event_source,
    pub attach_render_locks: libc::c_int,
    pub cursors: wl_list,
    pub hardware_cursor: *mut wlr_output_cursor,
    pub software_cursor_locks: libc::c_int,
    pub display_destroy: wl_listener,
    pub data: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub frame: wl_signal,
    pub needs_frame: wl_signal,
    pub precommit: wl_signal,
    pub commit: wl_signal,
    pub present: wl_signal,
    pub enable: wl_signal,
    pub mode: wl_signal,
    pub scale: wl_signal,
    pub transform: wl_signal,
    pub destroy: wl_signal,
}
/* *
 * Holds the double-buffered output state.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub damage: pixman_region32_t,
    pub buffer_type: wlr_output_state_buffer_type,
    pub buffer: *mut wlr_buffer,
    // if WLR_OUTPUT_STATE_BUFFER_SCANOUT
}
pub type wlr_output_state_buffer_type = libc::c_uint;
pub const WLR_OUTPUT_STATE_BUFFER_SCANOUT: wlr_output_state_buffer_type = 1;
pub const WLR_OUTPUT_STATE_BUFFER_RENDER: wlr_output_state_buffer_type = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_rdp_peer_context {
    pub _p: rdpContext,
    pub backend: *mut wlr_rdp_backend,
    pub events: [*mut wl_event_source; 64],
    pub peer: *mut freerdp_peer,
    pub flags: uint32_t,
    pub rfx_context: *mut RFX_CONTEXT,
    pub encode_stream: *mut wStream,
    pub rfx_rects: *mut RFX_RECT,
    pub nsc_context: *mut NSC_CONTEXT,
    pub output: *mut wlr_rdp_output,
    pub pointer: *mut wlr_rdp_input_device,
    pub keyboard: *mut wlr_rdp_input_device,
    pub link: wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_rdp_input_device {
    pub wlr_input_device: wlr_input_device,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_rdp_output {
    pub wlr_output: wlr_output,
    pub backend: *mut wlr_rdp_backend,
    pub context: *mut wlr_rdp_peer_context,
    pub egl_surface: *mut libc::c_void,
    pub shadow_surface: *mut pixman_image_t,
    pub frame_timer: *mut wl_event_source,
    pub frame_delay: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wlr_rdp_backend {
    pub backend: wlr_backend,
    pub egl: wlr_egl,
    pub renderer: *mut wlr_renderer,
    pub display: *mut wl_display,
    pub display_destroy: wl_listener,
    pub tls_cert_path: *const libc::c_char,
    pub tls_key_path: *const libc::c_char,
    pub address: *mut libc::c_char,
    pub port: libc::c_int,
    pub listener: *mut freerdp_listener,
    pub listener_events: [*mut wl_event_source; 64],
    pub clients: wl_list,
}
static mut input_device_impl: wlr_input_device_impl =
    { let mut init = wlr_input_device_impl{destroy: None,}; init };
// ms
#[no_mangle]
pub unsafe extern "C" fn wlr_rdp_pointer_create(mut backend:
                                                    *mut wlr_rdp_backend,
                                                mut context:
                                                    *mut wlr_rdp_peer_context)
 -> *mut wlr_rdp_input_device {
    let mut device: *mut wlr_rdp_input_device =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_rdp_input_device>() as libc::c_ulong)
            as *mut wlr_rdp_input_device;
    if device.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate RDP input device\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/rdp/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 20i32);
        return 0 as *mut wlr_rdp_input_device
    }
    let mut vendor: libc::c_int = 0i32;
    let mut product: libc::c_int = 0i32;
    let mut name: *const libc::c_char =
        b"rdp\x00" as *const u8 as *const libc::c_char;
    let mut wlr_device: *mut wlr_input_device =
        &mut (*device).wlr_input_device;
    wlr_input_device_init(wlr_device, WLR_INPUT_DEVICE_POINTER,
                          &mut input_device_impl, name, vendor, product);
    (*wlr_device).c2rust_unnamed.pointer =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<wlr_pointer>() as libc::c_ulong) as
            *mut wlr_pointer;
    if (*wlr_device).c2rust_unnamed.pointer.is_null() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Failed to allocate RDP pointer device\x00" as
                     *const u8 as *const libc::c_char,
                 b"../backend/rdp/pointer.c\x00" as *const u8 as
                     *const libc::c_char, 32i32);
        return 0 as *mut wlr_rdp_input_device
    }
    (*wlr_device).output_name =
        strdup((*(*context).output).wlr_output.name.as_mut_ptr());
    wlr_pointer_init((*wlr_device).c2rust_unnamed.pointer,
                     0 as *const wlr_pointer_impl);
    wlr_signal_emit_safe(&mut (*backend).backend.events.new_input,
                         wlr_device as *mut libc::c_void);
    return device;
}
