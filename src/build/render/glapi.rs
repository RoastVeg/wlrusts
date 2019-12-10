use libc;
extern "C" {
    pub type wl_display;
    pub type wl_resource;
    #[no_mangle]
    fn _wlr_log(verbosity: wlr_log_importance, format: *const libc::c_char,
                _: ...);
    #[no_mangle]
    fn eglGetProcAddress(procname: *const libc::c_char)
     -> __eglMustCastToProperFunctionPointerType;
}
pub type wlr_log_importance = libc::c_uint;
pub const WLR_LOG_IMPORTANCE_LAST: wlr_log_importance = 4;
pub const WLR_DEBUG: wlr_log_importance = 3;
pub const WLR_INFO: wlr_log_importance = 2;
pub const WLR_ERROR: wlr_log_importance = 1;
pub const WLR_SILENT: wlr_log_importance = 0;
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type khronos_int32_t = int32_t;
pub type khronos_uint64_t = uint64_t;
pub type EGLint = khronos_int32_t;
pub type EGLBoolean = libc::c_uint;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type __eglMustCastToProperFunctionPointerType
    =
    Option<unsafe extern "C" fn() -> ()>;
pub type EGLenum = libc::c_uint;
pub type EGLClientBuffer = *mut libc::c_void;
pub type EGLAttrib = intptr_t;
pub type EGLLabelKHR = *mut libc::c_void;
pub type EGLDEBUGPROCKHR
    =
    Option<unsafe extern "C" fn(_: EGLenum, _: *const libc::c_char, _: EGLint,
                                _: EGLLabelKHR, _: EGLLabelKHR,
                                _: *const libc::c_char) -> ()>;
pub type PFNEGLDEBUGMESSAGECONTROLKHRPROC
    =
    Option<unsafe extern "C" fn(_: EGLDEBUGPROCKHR, _: *const EGLAttrib)
               -> EGLint>;
pub type EGLImageKHR = *mut libc::c_void;
pub type PFNEGLCREATEIMAGEKHRPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLContext, _: EGLenum,
                                _: EGLClientBuffer, _: *const EGLint)
               -> EGLImageKHR>;
pub type PFNEGLDESTROYIMAGEKHRPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLImageKHR) -> EGLBoolean>;
pub type EGLuint64KHR = khronos_uint64_t;
pub type PFNEGLSWAPBUFFERSWITHDAMAGEKHRPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLSurface, _: *mut EGLint,
                                _: EGLint) -> EGLBoolean>;
pub type PFNEGLQUERYDMABUFFORMATSEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLint, _: *mut EGLint,
                                _: *mut EGLint) -> EGLBoolean>;
pub type PFNEGLQUERYDMABUFMODIFIERSEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLint, _: EGLint,
                                _: *mut EGLuint64KHR, _: *mut EGLBoolean,
                                _: *mut EGLint) -> EGLBoolean>;
pub type PFNEGLGETPLATFORMDISPLAYEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLenum, _: *mut libc::c_void,
                                _: *const EGLint) -> EGLDisplay>;
pub type PFNEGLCREATEPLATFORMWINDOWSURFACEEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLConfig,
                                _: *mut libc::c_void, _: *const EGLint)
               -> EGLSurface>;
pub type PFNEGLSWAPBUFFERSWITHDAMAGEEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLSurface, _: *mut EGLint,
                                _: EGLint) -> EGLBoolean>;
pub type PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLImageKHR,
                                _: *mut libc::c_int, _: *mut libc::c_int,
                                _: *mut EGLuint64KHR) -> EGLBoolean>;
pub type PFNEGLEXPORTDMABUFIMAGEMESAPROC
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: EGLImageKHR,
                                _: *mut libc::c_int, _: *mut EGLint,
                                _: *mut EGLint) -> EGLBoolean>;
pub type PFNEGLBINDWAYLANDDISPLAYWL
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: *mut wl_display)
               -> EGLBoolean>;
pub type PFNEGLUNBINDWAYLANDDISPLAYWL
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: *mut wl_display)
               -> EGLBoolean>;
pub type PFNEGLQUERYWAYLANDBUFFERWL
    =
    Option<unsafe extern "C" fn(_: EGLDisplay, _: *mut wl_resource, _: EGLint,
                                _: *mut EGLint) -> EGLBoolean>;
pub type GLenum = libc::c_uint;
pub type GLuint = libc::c_uint;
pub type GLchar = libc::c_char;
pub type GLboolean = libc::c_uchar;
pub type GLsizei = libc::c_int;
pub type GLDEBUGPROCKHR
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLuint, _: GLenum,
                                _: GLsizei, _: *const GLchar,
                                _: *const libc::c_void) -> ()>;
pub type PFNGLDEBUGMESSAGECONTROLKHRPROC
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLenum, _: GLsizei,
                                _: *const GLuint, _: GLboolean) -> ()>;
pub type PFNGLDEBUGMESSAGECALLBACKKHRPROC
    =
    Option<unsafe extern "C" fn(_: GLDEBUGPROCKHR, _: *const libc::c_void)
               -> ()>;
pub type PFNGLPUSHDEBUGGROUPKHRPROC
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLuint, _: GLsizei,
                                _: *const GLchar) -> ()>;
pub type PFNGLPOPDEBUGGROUPKHRPROC = Option<unsafe extern "C" fn() -> ()>;
pub type GLeglImageOES = *mut libc::c_void;
pub type PFNGLEGLIMAGETARGETTEXTURE2DOESPROC
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLeglImageOES) -> ()>;
#[no_mangle]
pub static mut eglGetPlatformDisplayEXT: PFNEGLGETPLATFORMDISPLAYEXTPROC =
    None;
#[no_mangle]
pub static mut eglCreatePlatformWindowSurfaceEXT:
           PFNEGLCREATEPLATFORMWINDOWSURFACEEXTPROC =
    None;
#[no_mangle]
pub static mut eglCreateImageKHR: PFNEGLCREATEIMAGEKHRPROC = None;
#[no_mangle]
pub static mut eglDestroyImageKHR: PFNEGLDESTROYIMAGEKHRPROC = None;
#[no_mangle]
pub static mut eglQueryWaylandBufferWL: PFNEGLQUERYWAYLANDBUFFERWL = None;
#[no_mangle]
pub static mut eglBindWaylandDisplayWL: PFNEGLBINDWAYLANDDISPLAYWL = None;
#[no_mangle]
pub static mut eglUnbindWaylandDisplayWL: PFNEGLUNBINDWAYLANDDISPLAYWL = None;
#[no_mangle]
pub static mut glEGLImageTargetTexture2DOES:
           PFNGLEGLIMAGETARGETTEXTURE2DOESPROC =
    None;
#[no_mangle]
pub static mut eglSwapBuffersWithDamageEXT: PFNEGLSWAPBUFFERSWITHDAMAGEEXTPROC
           =
    None;
#[no_mangle]
pub static mut eglSwapBuffersWithDamageKHR: PFNEGLSWAPBUFFERSWITHDAMAGEKHRPROC
           =
    None;
#[no_mangle]
pub static mut eglQueryDmaBufFormatsEXT: PFNEGLQUERYDMABUFFORMATSEXTPROC =
    None;
#[no_mangle]
pub static mut eglQueryDmaBufModifiersEXT: PFNEGLQUERYDMABUFMODIFIERSEXTPROC =
    None;
#[no_mangle]
pub static mut eglExportDMABUFImageQueryMESA:
           PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC =
    None;
#[no_mangle]
pub static mut eglExportDMABUFImageMESA: PFNEGLEXPORTDMABUFIMAGEMESAPROC =
    None;
#[no_mangle]
pub static mut eglDebugMessageControlKHR: PFNEGLDEBUGMESSAGECONTROLKHRPROC =
    None;
#[no_mangle]
pub static mut glDebugMessageCallbackKHR: PFNGLDEBUGMESSAGECALLBACKKHRPROC =
    None;
#[no_mangle]
pub static mut glDebugMessageControlKHR: PFNGLDEBUGMESSAGECONTROLKHRPROC =
    None;
#[no_mangle]
pub static mut glPopDebugGroupKHR: PFNGLPOPDEBUGGROUPKHRPROC = None;
#[no_mangle]
pub static mut glPushDebugGroupKHR: PFNGLPUSHDEBUGGROUPKHRPROC = None;
#[no_mangle]
pub unsafe extern "C" fn load_glapi() -> bool {
    static mut done: bool = 0i32 != 0;
    if done { return 1i32 != 0 }
    eglGetPlatformDisplayEXT =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLGETPLATFORMDISPLAYEXTPROC>(eglGetProcAddress(b"eglGetPlatformDisplayEXT\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const libc::c_char));
    if eglGetPlatformDisplayEXT.is_none() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unable to load eglGetPlatformDisplayEXT\x00" as
                     *const u8 as *const libc::c_char,
                 b"render/glapi.c\x00" as *const u8 as *const libc::c_char,
                 32i32);
        return 0i32 != 0
    }
    eglCreatePlatformWindowSurfaceEXT =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLCREATEPLATFORMWINDOWSURFACEEXTPROC>(eglGetProcAddress(b"eglCreatePlatformWindowSurfaceEXT\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char));
    if eglCreatePlatformWindowSurfaceEXT.is_none() {
        _wlr_log(WLR_ERROR,
                 b"[%s:%d] Unable to load eglCreatePlatformWindowSurfaceEXT\x00"
                     as *const u8 as *const libc::c_char,
                 b"render/glapi.c\x00" as *const u8 as *const libc::c_char,
                 37i32);
        return 0i32 != 0
    }
    eglCreateImageKHR =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLCREATEIMAGEKHRPROC>(eglGetProcAddress(b"eglCreateImageKHR\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char));
    eglDestroyImageKHR =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLDESTROYIMAGEKHRPROC>(eglGetProcAddress(b"eglDestroyImageKHR\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char));
    eglQueryWaylandBufferWL =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLQUERYWAYLANDBUFFERWL>(eglGetProcAddress(b"eglQueryWaylandBufferWL\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char));
    eglBindWaylandDisplayWL =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLBINDWAYLANDDISPLAYWL>(eglGetProcAddress(b"eglBindWaylandDisplayWL\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char));
    eglUnbindWaylandDisplayWL =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLUNBINDWAYLANDDISPLAYWL>(eglGetProcAddress(b"eglUnbindWaylandDisplayWL\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char));
    glEGLImageTargetTexture2DOES =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNGLEGLIMAGETARGETTEXTURE2DOESPROC>(eglGetProcAddress(b"glEGLImageTargetTexture2DOES\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const libc::c_char));
    eglSwapBuffersWithDamageEXT =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLSWAPBUFFERSWITHDAMAGEEXTPROC>(eglGetProcAddress(b"eglSwapBuffersWithDamageEXT\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char));
    eglSwapBuffersWithDamageKHR =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLSWAPBUFFERSWITHDAMAGEKHRPROC>(eglGetProcAddress(b"eglSwapBuffersWithDamageKHR\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char));
    eglQueryDmaBufFormatsEXT =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLQUERYDMABUFFORMATSEXTPROC>(eglGetProcAddress(b"eglQueryDmaBufFormatsEXT\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const libc::c_char));
    eglQueryDmaBufModifiersEXT =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLQUERYDMABUFMODIFIERSEXTPROC>(eglGetProcAddress(b"eglQueryDmaBufModifiersEXT\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
    eglExportDMABUFImageQueryMESA =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC>(eglGetProcAddress(b"eglExportDMABUFImageQueryMESA\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char));
    eglExportDMABUFImageMESA =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLEXPORTDMABUFIMAGEMESAPROC>(eglGetProcAddress(b"eglExportDMABUFImageMESA\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const libc::c_char));
    eglDebugMessageControlKHR =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNEGLDEBUGMESSAGECONTROLKHRPROC>(eglGetProcAddress(b"eglDebugMessageControlKHR\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char));
    glDebugMessageCallbackKHR =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNGLDEBUGMESSAGECALLBACKKHRPROC>(eglGetProcAddress(b"glDebugMessageCallbackKHR\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char));
    glDebugMessageControlKHR =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNGLDEBUGMESSAGECONTROLKHRPROC>(eglGetProcAddress(b"glDebugMessageControlKHR\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const libc::c_char));
    glPopDebugGroupKHR =
        eglGetProcAddress(b"glPopDebugGroupKHR\x00" as *const u8 as
                              *const libc::c_char);
    glPushDebugGroupKHR =
        ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                PFNGLPUSHDEBUGGROUPKHRPROC>(eglGetProcAddress(b"glPushDebugGroupKHR\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char));
    done = 1i32 != 0;
    return 1i32 != 0;
}
