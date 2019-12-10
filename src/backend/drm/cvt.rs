use libc;
extern "C" {
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _drmModeModeInfo {
    pub clock: uint32_t,
    pub hdisplay: uint16_t,
    pub hsync_start: uint16_t,
    pub hsync_end: uint16_t,
    pub htotal: uint16_t,
    pub hskew: uint16_t,
    pub vdisplay: uint16_t,
    pub vsync_start: uint16_t,
    pub vsync_end: uint16_t,
    pub vtotal: uint16_t,
    pub vscan: uint16_t,
    pub vrefresh: uint32_t,
    pub flags: uint32_t,
    pub type_0: uint32_t,
    pub name: [libc::c_char; 32],
}
pub type drmModeModeInfo = _drmModeModeInfo;
/*
 * Generate a CVT standard mode from hdisplay, vdisplay and vrefresh.
 *
 * These calculations are stolen from the CVT calculation spreadsheet written
 * by Graham Loveridge. He seems to be claiming no copyright and there seems to
 * be no license attached to this. He apparently just wants to see his name
 * mentioned.
 *
 * This file can be found at http://www.vesa.org/Public/CVT/CVTd6r1.xls
 *
 * Comments and structure corresponds to the comments and structure of the xls.
 * This should ease importing of future changes to the standard (not very
 * likely though).
 *
 * This function is borrowed from xorg-xserver's xf86CVTmode.
 */
#[no_mangle]
pub unsafe extern "C" fn generate_cvt_mode(mut mode: *mut drmModeModeInfo,
                                           mut hdisplay: libc::c_int,
                                           mut vdisplay: libc::c_int,
                                           mut vrefresh: libc::c_float,
                                           mut reduced: bool,
                                           mut interlaced: bool) {
    let mut margins: bool = 0i32 != 0; /* Please rename this */
    let mut vfield_rate: libc::c_float = 0.;
    let mut hperiod: libc::c_float = 0.;
    let mut hdisplay_rnd: libc::c_int = 0;
    let mut hmargin: libc::c_int = 0;
    let mut vdisplay_rnd: libc::c_int = 0;
    let mut vmargin: libc::c_int = 0;
    let mut vsync: libc::c_int = 0;
    let mut interlace: libc::c_float = 0.;
    /* CVT default is 60.0Hz */
    if vrefresh == 0. { vrefresh = 60.0f64 as libc::c_float }
    /* 1. Required field rate */
    if interlaced {
        vfield_rate = vrefresh * 2i32 as libc::c_float
    } else { vfield_rate = vrefresh }
    /* 2. Horizontal pixels */
    hdisplay_rnd = hdisplay - hdisplay % 8i32;
    /* 3. Determine left and right borders */
    if margins {
        /* right margin is actually exactly the same as left */
        hmargin =
            (hdisplay_rnd as libc::c_float as libc::c_double * 1.8f64 /
                 100.0f64) as libc::c_int;
        hmargin -= hmargin % 8i32
    } else { hmargin = 0i32 }
    /* 4. Find total active pixels */
    (*mode).hdisplay = (hdisplay_rnd + 2i32 * hmargin) as uint16_t;
    /* 5. Find number of lines per field */
    if interlaced {
        vdisplay_rnd = vdisplay / 2i32
    } else { vdisplay_rnd = vdisplay }
    /* 6. Find top and bottom margins */
	/* nope. */
    if margins {
        /* top and bottom margins are equal again. */
        vmargin =
            (vdisplay_rnd as libc::c_float as libc::c_double * 1.8f64 /
                 100.0f64) as libc::c_int
    } else { vmargin = 0i32 }
    (*mode).vdisplay = (vdisplay + 2i32 * vmargin) as uint16_t;
    /* 7. interlace */
    if interlaced {
        interlace = 0.5f64 as libc::c_float
    } else { interlace = 0.0f64 as libc::c_float }
    /* Determine vsync Width from aspect ratio */
    if vdisplay % 3i32 == 0 && vdisplay * 4i32 / 3i32 == hdisplay {
        vsync = 4i32
    } else if vdisplay % 9i32 == 0 && vdisplay * 16i32 / 9i32 == hdisplay {
        vsync = 5i32
    } else if vdisplay % 10i32 == 0 && vdisplay * 16i32 / 10i32 == hdisplay {
        vsync = 6i32
    } else if vdisplay % 4i32 == 0 && vdisplay * 5i32 / 4i32 == hdisplay {
        vsync = 7i32
    } else if vdisplay % 9i32 == 0 && vdisplay * 15i32 / 9i32 == hdisplay {
        vsync = 7i32
    } else { vsync = 10i32 } /* Custom */
    if !reduced {
        let mut hblank_percentage: libc::c_float = 0.; /* reduced blanking */
        let mut vsync_and_back_porch: libc::c_int = 0;
        let mut vblank_porch: libc::c_int = 0;
        let mut hblank: libc::c_int = 0;
        /* simplified GTF calculation */
        hperiod =
            (1000000.0f64 / vfield_rate as libc::c_double - 550.0f64) as
                libc::c_float /
                ((vdisplay_rnd + 2i32 * vmargin + 3i32) as libc::c_float +
                     interlace);
        if (550.0f64 / hperiod as libc::c_double) as libc::c_int + 1i32 <
               vsync + 3i32 {
            vsync_and_back_porch = vsync + 3i32
        } else {
            vsync_and_back_porch =
                (550.0f64 / hperiod as libc::c_double) as libc::c_int + 1i32
        }
        vblank_porch = vsync_and_back_porch - vsync;
        (*mode).vtotal =
            ((vdisplay_rnd + 2i32 * vmargin + vsync_and_back_porch) as
                 libc::c_float + interlace + 3i32 as libc::c_float) as
                uint16_t;
        hblank_percentage =
            (((40i32 - 20i32) * 128i32 / 256i32 + 20i32) as libc::c_double -
                 ((600i32 * 128i32 / 256i32) as libc::c_float * hperiod) as
                     libc::c_double / 1000.0f64) as libc::c_float;
        if hblank_percentage < 20i32 as libc::c_float {
            hblank_percentage = 20i32 as libc::c_float
        }
        hblank =
            (((*mode).hdisplay as libc::c_int as libc::c_float *
                  hblank_percentage) as libc::c_double /
                 (100.0f64 - hblank_percentage as libc::c_double)) as
                libc::c_int;
        hblank -= hblank % (2i32 * 8i32);
        (*mode).htotal =
            ((*mode).hdisplay as libc::c_int + hblank) as uint16_t;
        (*mode).hsync_end =
            ((*mode).hdisplay as libc::c_int + hblank / 2i32) as uint16_t;
        (*mode).hsync_start =
            ((*mode).hsync_end as libc::c_int -
                 (*mode).htotal as libc::c_int * 8i32 / 100i32) as uint16_t;
        (*mode).hsync_start =
            ((*mode).hsync_start as libc::c_int +
                 (8i32 - (*mode).hsync_start as libc::c_int % 8i32)) as
                uint16_t;
        (*mode).vsync_start =
            ((*mode).vdisplay as libc::c_int + 3i32) as uint16_t;
        (*mode).vsync_end =
            ((*mode).vsync_start as libc::c_int + vsync) as uint16_t
    } else {
        let mut vbi_lines: libc::c_int = 0;
        /* 8. Estimated Horizontal period */
        /* 9. Find number of lines in sync + backporch */
        /* 10. Find number of lines in back porch */
        /* 11. Find total number of lines in vertical field */
        /* 12. Find ideal blanking duty cycle from formula */
        /* 13. Blanking time */
        /* 14. Find total number of pixels in a line. */
        /* Fill in hsync values */
        /* Fill in vsync values */
        /* 8. Estimate Horizontal period. */
        hperiod =
            (1000000.0f64 / vfield_rate as libc::c_double - 460.0f64) as
                libc::c_float /
                (vdisplay_rnd + 2i32 * vmargin) as libc::c_float;
        /* 9. Find number of lines in vertical blanking */
        vbi_lines =
            (460.0f64 as libc::c_float / hperiod + 1i32 as libc::c_float) as
                libc::c_int;
        /* 10. Check if vertical blanking is sufficient */
        if vbi_lines < 3i32 + vsync + 6i32 { vbi_lines = 3i32 + vsync + 6i32 }
        /* 11. Find total number of lines in vertical field */
        (*mode).vtotal =
            ((vdisplay_rnd + 2i32 * vmargin) as libc::c_float + interlace +
                 vbi_lines as libc::c_float) as uint16_t;
        /* 12. Find total number of pixels in a line */
        (*mode).htotal =
            ((*mode).hdisplay as libc::c_int as libc::c_double + 160.0f64) as
                uint16_t;
        /* Fill in hsync values */
        (*mode).hsync_end =
            ((*mode).hdisplay as libc::c_int as libc::c_double +
                 160.0f64 / 2i32 as libc::c_double) as uint16_t;
        (*mode).hsync_start =
            ((*mode).hsync_end as libc::c_int as libc::c_double - 32.0f64) as
                uint16_t;
        /* Fill in vsync values */
        (*mode).vsync_start =
            ((*mode).vdisplay as libc::c_int + 3i32) as uint16_t;
        (*mode).vsync_end =
            ((*mode).vsync_start as libc::c_int + vsync) as uint16_t
    }
    /* 15/13. Find pixel clock frequency (kHz for xf86) */
    (*mode).clock =
        ((*mode).htotal as libc::c_int as libc::c_double * 1000.0f64 /
             hperiod as libc::c_double) as uint32_t;
    (*mode).clock =
        ((*mode).clock as
             libc::c_uint).wrapping_sub((*mode).clock.wrapping_rem(250i32 as
                                                                       libc::c_uint))
            as uint32_t as uint32_t;
    /* 17/15. Find actual Field rate */
    (*mode).vrefresh =
        (1000.0f64 * (*mode).clock as libc::c_float as libc::c_double /
             ((*mode).htotal as libc::c_int * (*mode).vtotal as libc::c_int)
                 as libc::c_float as libc::c_double) as uint32_t;
    /* 18/16. Find actual vertical frame frequency */
	/* ignore - just set the mode flag for interlaced */
    if interlaced {
        (*mode).vtotal = ((*mode).vtotal as libc::c_int * 2i32) as uint16_t;
        (*mode).flags |= (1i32 << 4i32) as libc::c_uint
    }
    snprintf((*mode).name.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
             b"%dx%d\x00" as *const u8 as *const libc::c_char, hdisplay,
             vdisplay);
    if reduced {
        (*mode).flags |= (1i32 << 0i32 | 1i32 << 3i32) as libc::c_uint
    } else { (*mode).flags |= (1i32 << 1i32 | 1i32 << 2i32) as libc::c_uint };
}
