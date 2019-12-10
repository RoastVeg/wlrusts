use libc;
pub type GLint = libc::c_int;
pub type size_t = libc::c_ulong;
pub type wl_shm_format = libc::c_uint;
pub const WL_SHM_FORMAT_YVU444: wl_shm_format = 875714137;
pub const WL_SHM_FORMAT_YUV444: wl_shm_format = 875713881;
pub const WL_SHM_FORMAT_YVU422: wl_shm_format = 909203033;
pub const WL_SHM_FORMAT_YUV422: wl_shm_format = 909202777;
pub const WL_SHM_FORMAT_YVU420: wl_shm_format = 842094169;
pub const WL_SHM_FORMAT_YUV420: wl_shm_format = 842093913;
pub const WL_SHM_FORMAT_YVU411: wl_shm_format = 825316953;
pub const WL_SHM_FORMAT_YUV411: wl_shm_format = 825316697;
pub const WL_SHM_FORMAT_YVU410: wl_shm_format = 961893977;
pub const WL_SHM_FORMAT_YUV410: wl_shm_format = 961959257;
pub const WL_SHM_FORMAT_NV61: wl_shm_format = 825644622;
pub const WL_SHM_FORMAT_NV16: wl_shm_format = 909203022;
pub const WL_SHM_FORMAT_NV21: wl_shm_format = 825382478;
pub const WL_SHM_FORMAT_NV12: wl_shm_format = 842094158;
pub const WL_SHM_FORMAT_AYUV: wl_shm_format = 1448433985;
pub const WL_SHM_FORMAT_VYUY: wl_shm_format = 1498765654;
pub const WL_SHM_FORMAT_UYVY: wl_shm_format = 1498831189;
pub const WL_SHM_FORMAT_YVYU: wl_shm_format = 1431918169;
pub const WL_SHM_FORMAT_YUYV: wl_shm_format = 1448695129;
pub const WL_SHM_FORMAT_BGRA1010102: wl_shm_format = 808665410;
pub const WL_SHM_FORMAT_RGBA1010102: wl_shm_format = 808665426;
pub const WL_SHM_FORMAT_ABGR2101010: wl_shm_format = 808665665;
pub const WL_SHM_FORMAT_ARGB2101010: wl_shm_format = 808669761;
pub const WL_SHM_FORMAT_BGRX1010102: wl_shm_format = 808671298;
pub const WL_SHM_FORMAT_RGBX1010102: wl_shm_format = 808671314;
pub const WL_SHM_FORMAT_XBGR2101010: wl_shm_format = 808665688;
pub const WL_SHM_FORMAT_XRGB2101010: wl_shm_format = 808669784;
pub const WL_SHM_FORMAT_BGRA8888: wl_shm_format = 875708738;
pub const WL_SHM_FORMAT_RGBA8888: wl_shm_format = 875708754;
pub const WL_SHM_FORMAT_ABGR8888: wl_shm_format = 875708993;
pub const WL_SHM_FORMAT_BGRX8888: wl_shm_format = 875714626;
pub const WL_SHM_FORMAT_RGBX8888: wl_shm_format = 875714642;
pub const WL_SHM_FORMAT_XBGR8888: wl_shm_format = 875709016;
pub const WL_SHM_FORMAT_BGR888: wl_shm_format = 875710274;
pub const WL_SHM_FORMAT_RGB888: wl_shm_format = 875710290;
pub const WL_SHM_FORMAT_BGR565: wl_shm_format = 909199170;
pub const WL_SHM_FORMAT_RGB565: wl_shm_format = 909199186;
pub const WL_SHM_FORMAT_BGRA5551: wl_shm_format = 892420418;
pub const WL_SHM_FORMAT_RGBA5551: wl_shm_format = 892420434;
pub const WL_SHM_FORMAT_ABGR1555: wl_shm_format = 892420673;
pub const WL_SHM_FORMAT_ARGB1555: wl_shm_format = 892424769;
pub const WL_SHM_FORMAT_BGRX5551: wl_shm_format = 892426306;
pub const WL_SHM_FORMAT_RGBX5551: wl_shm_format = 892426322;
pub const WL_SHM_FORMAT_XBGR1555: wl_shm_format = 892420696;
pub const WL_SHM_FORMAT_XRGB1555: wl_shm_format = 892424792;
pub const WL_SHM_FORMAT_BGRA4444: wl_shm_format = 842088770;
pub const WL_SHM_FORMAT_RGBA4444: wl_shm_format = 842088786;
pub const WL_SHM_FORMAT_ABGR4444: wl_shm_format = 842089025;
pub const WL_SHM_FORMAT_ARGB4444: wl_shm_format = 842093121;
pub const WL_SHM_FORMAT_BGRX4444: wl_shm_format = 842094658;
pub const WL_SHM_FORMAT_RGBX4444: wl_shm_format = 842094674;
pub const WL_SHM_FORMAT_XBGR4444: wl_shm_format = 842089048;
pub const WL_SHM_FORMAT_XRGB4444: wl_shm_format = 842093144;
pub const WL_SHM_FORMAT_BGR233: wl_shm_format = 944916290;
pub const WL_SHM_FORMAT_RGB332: wl_shm_format = 943867730;
pub const WL_SHM_FORMAT_C8: wl_shm_format = 538982467;
pub const WL_SHM_FORMAT_XRGB8888: wl_shm_format = 1;
pub const WL_SHM_FORMAT_ARGB8888: wl_shm_format = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct wlr_gles2_pixel_format {
    pub wl_format: wl_shm_format,
    pub gl_format: GLint,
    pub gl_type: GLint,
    pub depth: libc::c_int,
    pub bpp: libc::c_int,
    pub has_alpha: bool,
}
/*
 * The wayland formats are little endian while the GL formats are big endian,
 * so WL_SHM_FORMAT_ARGB8888 is actually compatible with GL_BGRA_EXT.
 */
static mut formats: [wlr_gles2_pixel_format; 4] =
    [{
         let mut init =
             wlr_gles2_pixel_format{wl_format: WL_SHM_FORMAT_ARGB8888,
                                    gl_format: 0x80e1i32,
                                    gl_type: 0x1401i32,
                                    depth: 32i32,
                                    bpp: 32i32,
                                    has_alpha: 1i32 != 0,};
         init
     },
     {
         let mut init =
             wlr_gles2_pixel_format{wl_format: WL_SHM_FORMAT_XRGB8888,
                                    gl_format: 0x80e1i32,
                                    gl_type: 0x1401i32,
                                    depth: 24i32,
                                    bpp: 32i32,
                                    has_alpha: 0i32 != 0,};
         init
     },
     {
         let mut init =
             wlr_gles2_pixel_format{wl_format: WL_SHM_FORMAT_XBGR8888,
                                    gl_format: 0x1908i32,
                                    gl_type: 0x1401i32,
                                    depth: 24i32,
                                    bpp: 32i32,
                                    has_alpha: 0i32 != 0,};
         init
     },
     {
         let mut init =
             wlr_gles2_pixel_format{wl_format: WL_SHM_FORMAT_ABGR8888,
                                    gl_format: 0x1908i32,
                                    gl_type: 0x1401i32,
                                    depth: 32i32,
                                    bpp: 32i32,
                                    has_alpha: 1i32 != 0,};
         init
     }];
static mut wl_formats: [wl_shm_format; 4] =
    [WL_SHM_FORMAT_ARGB8888, WL_SHM_FORMAT_XRGB8888, WL_SHM_FORMAT_ABGR8888,
     WL_SHM_FORMAT_XBGR8888];
// TODO: more pixel formats
#[no_mangle]
pub unsafe extern "C" fn get_gles2_format_from_wl(mut fmt: wl_shm_format)
 -> *const wlr_gles2_pixel_format {
    let mut i: size_t = 0i32 as size_t;
    while i <
              (::std::mem::size_of::<[wlr_gles2_pixel_format; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<wlr_gles2_pixel_format>()
                                                   as libc::c_ulong) {
        if formats[i as usize].wl_format as libc::c_uint ==
               fmt as libc::c_uint {
            return &*formats.as_ptr().offset(i as isize) as
                       *const wlr_gles2_pixel_format
        }
        i = i.wrapping_add(1)
    }
    return 0 as *const wlr_gles2_pixel_format;
}
#[no_mangle]
pub unsafe extern "C" fn get_gles2_format_from_gl(mut gl_format: GLint,
                                                  mut gl_type: GLint,
                                                  mut alpha: bool)
 -> *const wlr_gles2_pixel_format {
    let mut i: size_t = 0i32 as size_t;
    while i <
              (::std::mem::size_of::<[wlr_gles2_pixel_format; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<wlr_gles2_pixel_format>()
                                                   as libc::c_ulong) {
        if formats[i as usize].gl_format == gl_format &&
               formats[i as usize].gl_type == gl_type &&
               formats[i as usize].has_alpha as libc::c_int ==
                   alpha as libc::c_int {
            return &*formats.as_ptr().offset(i as isize) as
                       *const wlr_gles2_pixel_format
        }
        i = i.wrapping_add(1)
    }
    return 0 as *const wlr_gles2_pixel_format;
}
// Basically:
	//   GL_TEXTURE_2D == mutable
	//   GL_TEXTURE_EXTERNAL_OES == immutable
// Only affects target == GL_TEXTURE_2D
// used to interpret upload data
#[no_mangle]
pub unsafe extern "C" fn get_gles2_wl_formats(mut len: *mut size_t)
 -> *const wl_shm_format {
    *len =
        (::std::mem::size_of::<[wl_shm_format; 4]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<wl_shm_format>()
                                             as libc::c_ulong);
    return wl_formats.as_ptr();
}
