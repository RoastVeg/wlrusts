use libc;
use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type AVOptionRanges;
    pub type AVOption;
    pub type AVBuffer;
    pub type AVBufferPool;
    pub type AVDictionary;
    pub type AVHWDeviceInternal;
    pub type AVHWFramesInternal;
    pub type AVCodecInternal;
    pub type MpegEncContext;
    pub type AVCodecHWConfigInternal;
    pub type AVCodecDefault;
    pub type AVFormatInternal;
    pub type AVStreamInternal;
    pub type AVDeviceCapabilitiesQuery;
    pub type AVDeviceInfoList;
    pub type AVCodecTag;
    pub type wl_proxy;
    pub type wl_display;
    pub type wl_output;
    pub type wl_registry;
    pub type zwlr_export_dmabuf_frame_v1;
    pub type zwlr_export_dmabuf_manager_v1;
    #[no_mangle]
    fn avcodec_alloc_context3(codec: *const AVCodec) -> *mut AVCodecContext;
    #[no_mangle]
    fn avcodec_parameters_from_context(par: *mut AVCodecParameters,
                                       codec: *const AVCodecContext)
     -> libc::c_int;
    #[no_mangle]
    fn avcodec_open2(avctx: *mut AVCodecContext, codec: *const AVCodec,
                     options: *mut *mut AVDictionary) -> libc::c_int;
    #[no_mangle]
    fn avcodec_close(avctx: *mut AVCodecContext) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn avcodec_send_frame(avctx: *mut AVCodecContext, frame: *const AVFrame)
     -> libc::c_int;
    #[no_mangle]
    fn av_dump_format(ic: *mut AVFormatContext, index: libc::c_int,
                      url: *const libc::c_char, is_output: libc::c_int);
    #[no_mangle]
    fn av_write_trailer(s: *mut AVFormatContext) -> libc::c_int;
    #[no_mangle]
    fn av_interleaved_write_frame(s: *mut AVFormatContext, pkt: *mut AVPacket)
     -> libc::c_int;
    #[no_mangle]
    fn avformat_write_header(s: *mut AVFormatContext,
                             options: *mut *mut AVDictionary) -> libc::c_int;
    #[no_mangle]
    fn strtof(_: *const libc::c_char, _: *mut *mut libc::c_char)
     -> libc::c_float;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn av_strerror(errnum: libc::c_int, errbuf: *mut libc::c_char,
                   errbuf_size: size_t) -> libc::c_int;
    #[no_mangle]
    fn av_mallocz(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn av_free(ptr: *mut libc::c_void);
    #[no_mangle]
    fn av_freep(ptr: *mut libc::c_void);
    #[no_mangle]
    fn av_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn av_rescale_q(a: int64_t, bq: AVRational, cq: AVRational) -> int64_t;
    #[no_mangle]
    fn av_log(avcl: *mut libc::c_void, level: libc::c_int,
              fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn av_default_item_name(ctx: *mut libc::c_void) -> *const libc::c_char;
    #[no_mangle]
    fn av_buffer_create(data: *mut uint8_t, size: libc::c_int,
                        free:
                            Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                        _: *mut uint8_t)
                                       -> ()>, opaque: *mut libc::c_void,
                        flags: libc::c_int) -> *mut AVBufferRef;
    #[no_mangle]
    fn av_buffer_ref(buf: *mut AVBufferRef) -> *mut AVBufferRef;
    #[no_mangle]
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    #[no_mangle]
    fn av_dict_set(pm: *mut *mut AVDictionary, key: *const libc::c_char,
                   value: *const libc::c_char, flags: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn av_dict_free(m: *mut *mut AVDictionary);
    #[no_mangle]
    fn av_frame_alloc() -> *mut AVFrame;
    #[no_mangle]
    fn av_frame_free(frame: *mut *mut AVFrame);
    #[no_mangle]
    fn av_hwdevice_find_type_by_name(name: *const libc::c_char)
     -> AVHWDeviceType;
    #[no_mangle]
    fn av_hwdevice_ctx_alloc(type_0: AVHWDeviceType) -> *mut AVBufferRef;
    #[no_mangle]
    fn av_hwdevice_ctx_init(ref_0: *mut AVBufferRef) -> libc::c_int;
    #[no_mangle]
    fn av_hwdevice_ctx_create(device_ctx: *mut *mut AVBufferRef,
                              type_0: AVHWDeviceType,
                              device: *const libc::c_char,
                              opts: *mut AVDictionary, flags: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn av_hwframe_ctx_alloc(device_ctx: *mut AVBufferRef) -> *mut AVBufferRef;
    #[no_mangle]
    fn av_hwframe_ctx_init(ref_0: *mut AVBufferRef) -> libc::c_int;
    #[no_mangle]
    fn av_hwframe_transfer_data(dst: *mut AVFrame, src: *const AVFrame,
                                flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn av_hwdevice_get_hwframe_constraints(ref_0: *mut AVBufferRef,
                                           hwconfig: *const libc::c_void)
     -> *mut AVHWFramesConstraints;
    #[no_mangle]
    fn av_hwframe_constraints_free(constraints:
                                       *mut *mut AVHWFramesConstraints);
    #[no_mangle]
    fn av_hwframe_map(dst: *mut AVFrame, src: *const AVFrame,
                      flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn av_init_packet(pkt: *mut AVPacket);
    #[no_mangle]
    fn av_packet_unref(pkt: *mut AVPacket);
    #[no_mangle]
    fn avcodec_receive_packet(avctx: *mut AVCodecContext,
                              avpkt: *mut AVPacket) -> libc::c_int;
    #[no_mangle]
    fn avcodec_find_encoder_by_name(name: *const libc::c_char)
     -> *mut AVCodec;
    #[no_mangle]
    fn avio_open(s: *mut *mut AVIOContext, url: *const libc::c_char,
                 flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn avio_closep(s: *mut *mut AVIOContext) -> libc::c_int;
    #[no_mangle]
    fn avformat_free_context(s: *mut AVFormatContext);
    #[no_mangle]
    fn avformat_new_stream(s: *mut AVFormatContext, c: *const AVCodec)
     -> *mut AVStream;
    #[no_mangle]
    fn avformat_alloc_output_context2(ctx: *mut *mut AVFormatContext,
                                      oformat: *mut AVOutputFormat,
                                      format_name: *const libc::c_char,
                                      filename: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn av_pix_fmt_count_planes(pix_fmt: AVPixelFormat) -> libc::c_int;
    #[no_mangle]
    fn av_get_pix_fmt(name: *const libc::c_char) -> AVPixelFormat;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pthread_create(__newthread: *mut pthread_t,
                      __attr: *const pthread_attr_t,
                      __start_routine:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      __arg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_init(__cond: *mut pthread_cond_t,
                         __cond_attr: *const pthread_condattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_wait(__cond: *mut pthread_cond_t,
                         __mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    static wl_registry_interface: wl_interface;
    #[no_mangle]
    static wl_output_interface: wl_interface;
    #[no_mangle]
    fn wl_display_roundtrip(display: *mut wl_display) -> libc::c_int;
    #[no_mangle]
    fn wl_display_dispatch(display: *mut wl_display) -> libc::c_int;
    #[no_mangle]
    fn wl_display_connect(name: *const libc::c_char) -> *mut wl_display;
    #[no_mangle]
    fn wl_proxy_add_listener(proxy: *mut wl_proxy,
                             implementation:
                                 *mut Option<unsafe extern "C" fn() -> ()>,
                             data: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn wl_proxy_destroy(proxy: *mut wl_proxy);
    #[no_mangle]
    fn wl_proxy_marshal_constructor_versioned(proxy: *mut wl_proxy,
                                              opcode: uint32_t,
                                              interface: *const wl_interface,
                                              version: uint32_t, _: ...)
     -> *mut wl_proxy;
    #[no_mangle]
    fn wl_proxy_marshal_constructor(proxy: *mut wl_proxy, opcode: uint32_t,
                                    interface: *const wl_interface, _: ...)
     -> *mut wl_proxy;
    #[no_mangle]
    fn wl_proxy_marshal(p: *mut wl_proxy, opcode: uint32_t, _: ...);
    #[no_mangle]
    static zwlr_export_dmabuf_manager_v1_interface: wl_interface;
    #[no_mangle]
    static zwlr_export_dmabuf_frame_v1_interface: wl_interface;
    #[no_mangle]
    fn wl_list_init(list: *mut wl_list);
    #[no_mangle]
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    #[no_mangle]
    fn wl_list_remove(elm: *mut wl_list);
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type AVMediaType = libc::c_int;
pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
pub type AVPictureType = libc::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
pub type ptrdiff_t = libc::c_long;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVRational {
    pub num: libc::c_int,
    pub den: libc::c_int,
}
pub type AVClassCategory = libc::c_uint;
pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const libc::c_char,
    pub item_name: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                              -> *const libc::c_char>,
    pub option: *const AVOption,
    pub version: libc::c_int,
    pub log_level_offset_offset: libc::c_int,
    pub parent_log_context_offset: libc::c_int,
    pub child_next: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: *mut libc::c_void)
                               -> *mut libc::c_void>,
    pub child_class_next: Option<unsafe extern "C" fn(_: *const AVClass)
                                     -> *const AVClass>,
    pub category: AVClassCategory,
    pub get_category: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> AVClassCategory>,
    pub query_ranges: Option<unsafe extern "C" fn(_: *mut *mut AVOptionRanges,
                                                  _: *mut libc::c_void,
                                                  _: *const libc::c_char,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
}
pub type AVPixelFormat = libc::c_int;
pub const AV_PIX_FMT_NB: AVPixelFormat = 193;
pub const AV_PIX_FMT_NV42: AVPixelFormat = 192;
pub const AV_PIX_FMT_NV24: AVPixelFormat = 191;
pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 190;
pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 189;
pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 188;
pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 187;
pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 186;
pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 185;
pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 184;
pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 183;
pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 182;
pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 181;
pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 180;
pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 179;
pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 178;
pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 177;
pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 176;
pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 175;
pub const AV_PIX_FMT_D3D11: AVPixelFormat = 174;
pub const AV_PIX_FMT_P016BE: AVPixelFormat = 173;
pub const AV_PIX_FMT_P016LE: AVPixelFormat = 172;
pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 171;
pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 170;
pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 169;
pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 168;
pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 167;
pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 166;
pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 165;
pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 164;
pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 163;
pub const AV_PIX_FMT_P010BE: AVPixelFormat = 162;
pub const AV_PIX_FMT_P010LE: AVPixelFormat = 161;
pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 160;
pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 159;
pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 158;
pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 157;
pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 156;
pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 155;
pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 154;
pub const AV_PIX_FMT_XVMC: AVPixelFormat = 153;
pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 152;
pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 151;
pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 150;
pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 149;
pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 148;
pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 147;
pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 146;
pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 145;
pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 144;
pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 143;
pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 142;
pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 141;
pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 140;
pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 139;
pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 138;
pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 137;
pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 136;
pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 135;
pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 134;
pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 133;
pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 132;
pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 131;
pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 130;
pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 129;
pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 128;
pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 127;
pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 126;
pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 125;
pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 124;
pub const AV_PIX_FMT_BGR0: AVPixelFormat = 123;
pub const AV_PIX_FMT_0BGR: AVPixelFormat = 122;
pub const AV_PIX_FMT_RGB0: AVPixelFormat = 121;
pub const AV_PIX_FMT_0RGB: AVPixelFormat = 120;
pub const AV_PIX_FMT_CUDA: AVPixelFormat = 119;
pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 118;
pub const AV_PIX_FMT_MMAL: AVPixelFormat = 117;
pub const AV_PIX_FMT_QSV: AVPixelFormat = 116;
pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 115;
pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 114;
pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 113;
pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 112;
pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 111;
pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 110;
pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 109;
pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 108;
pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 107;
pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 106;
pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 105;
pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 104;
pub const AV_PIX_FMT_NV16: AVPixelFormat = 103;
pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 102;
pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 101;
pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 100;
pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 99;
pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 98;
pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 97;
pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 96;
pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 95;
pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 94;
pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 93;
pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 92;
pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 91;
pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 90;
pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 89;
pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 88;
pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 87;
pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 86;
pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 85;
pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 84;
pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 83;
pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 82;
pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 81;
pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 80;
pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 79;
pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 78;
pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 77;
pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 76;
pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 75;
pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 74;
pub const AV_PIX_FMT_GBR24P: AVPixelFormat = 73;
pub const AV_PIX_FMT_GBRP: AVPixelFormat = 73;
pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 72;
pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 71;
pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 70;
pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 69;
pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 68;
pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 67;
pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 66;
pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 65;
pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 64;
pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 63;
pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 62;
pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 61;
pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 60;
pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 59;
pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = 58;
pub const AV_PIX_FMT_Y400A: AVPixelFormat = 58;
pub const AV_PIX_FMT_YA8: AVPixelFormat = 58;
pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 57;
pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 56;
pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 55;
pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 54;
pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 53;
pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 52;
pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 51;
pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 50;
pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 49;
pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 48;
pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 47;
pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 46;
pub const AV_PIX_FMT_VAAPI_VLD: AVPixelFormat = 46;
pub const AV_PIX_FMT_VAAPI_IDCT: AVPixelFormat = 45;
pub const AV_PIX_FMT_VAAPI_MOCO: AVPixelFormat = 44;
pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21;
pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20;
pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18;
pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17;
pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11;
pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
pub type AVColorPrimaries = libc::c_uint;
pub const AVCOL_PRI_NB: AVColorPrimaries = 23;
pub const AVCOL_PRI_JEDEC_P22: AVColorPrimaries = 22;
pub const AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
pub const AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
pub const AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = 10;
pub const AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
pub const AVCOL_PRI_BT2020: AVColorPrimaries = 9;
pub const AVCOL_PRI_FILM: AVColorPrimaries = 8;
pub const AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
pub const AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
pub const AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
pub const AVCOL_PRI_BT470M: AVColorPrimaries = 4;
pub const AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
pub const AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
pub const AVCOL_PRI_BT709: AVColorPrimaries = 1;
pub const AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
pub type AVColorTransferCharacteristic = libc::c_uint;
pub const AVCOL_TRC_NB: AVColorTransferCharacteristic = 19;
pub const AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;
pub const AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
pub const AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
pub const AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
pub const AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
pub const AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
pub const AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
pub const AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
pub const AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
pub const AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
pub const AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
pub const AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
pub const AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
pub const AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
pub const AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
pub const AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
pub const AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
pub type AVColorSpace = libc::c_uint;
pub const AVCOL_SPC_NB: AVColorSpace = 15;
pub const AVCOL_SPC_ICTCP: AVColorSpace = 14;
pub const AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
pub const AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
pub const AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
pub const AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
pub const AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
pub const AVCOL_SPC_YCOCG: AVColorSpace = 8;
pub const AVCOL_SPC_YCGCO: AVColorSpace = 8;
pub const AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
pub const AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
pub const AVCOL_SPC_BT470BG: AVColorSpace = 5;
pub const AVCOL_SPC_FCC: AVColorSpace = 4;
pub const AVCOL_SPC_RESERVED: AVColorSpace = 3;
pub const AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
pub const AVCOL_SPC_BT709: AVColorSpace = 1;
pub const AVCOL_SPC_RGB: AVColorSpace = 0;
pub type AVColorRange = libc::c_uint;
pub const AVCOL_RANGE_NB: AVColorRange = 3;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
pub type AVChromaLocation = libc::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
pub type AVSampleFormat = libc::c_int;
pub const AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
}
pub type AVFrameSideDataType = libc::c_uint;
pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 20;
pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 19;
pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 18;
pub const AV_FRAME_DATA_QP_TABLE_DATA: AVFrameSideDataType = 17;
pub const AV_FRAME_DATA_QP_TABLE_PROPERTIES: AVFrameSideDataType = 16;
pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVFrame {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
    pub extended_data: *mut *mut uint8_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub nb_samples: libc::c_int,
    pub format: libc::c_int,
    pub key_frame: libc::c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: int64_t,
    pub pkt_pts: int64_t,
    pub pkt_dts: int64_t,
    pub coded_picture_number: libc::c_int,
    pub display_picture_number: libc::c_int,
    pub quality: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub repeat_pict: libc::c_int,
    pub interlaced_frame: libc::c_int,
    pub top_field_first: libc::c_int,
    pub palette_has_changed: libc::c_int,
    pub reordered_opaque: int64_t,
    pub sample_rate: libc::c_int,
    pub channel_layout: uint64_t,
    pub buf: [*mut AVBufferRef; 8],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: libc::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: libc::c_int,
    pub flags: libc::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: int64_t,
    pub pkt_pos: int64_t,
    pub pkt_duration: int64_t,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: libc::c_int,
    pub channels: libc::c_int,
    pub pkt_size: libc::c_int,
    pub qscale_table: *mut int8_t,
    pub qstride: libc::c_int,
    pub qscale_type: libc::c_int,
    pub qp_table_buf: *mut AVBufferRef,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: size_t,
    pub crop_bottom: size_t,
    pub crop_left: size_t,
    pub crop_right: size_t,
    pub private_ref: *mut AVBufferRef,
}
pub type AVHWDeviceType = libc::c_uint;
pub const AV_HWDEVICE_TYPE_MEDIACODEC: AVHWDeviceType = 10;
pub const AV_HWDEVICE_TYPE_OPENCL: AVHWDeviceType = 9;
pub const AV_HWDEVICE_TYPE_DRM: AVHWDeviceType = 8;
pub const AV_HWDEVICE_TYPE_D3D11VA: AVHWDeviceType = 7;
pub const AV_HWDEVICE_TYPE_VIDEOTOOLBOX: AVHWDeviceType = 6;
pub const AV_HWDEVICE_TYPE_QSV: AVHWDeviceType = 5;
pub const AV_HWDEVICE_TYPE_DXVA2: AVHWDeviceType = 4;
pub const AV_HWDEVICE_TYPE_VAAPI: AVHWDeviceType = 3;
pub const AV_HWDEVICE_TYPE_CUDA: AVHWDeviceType = 2;
pub const AV_HWDEVICE_TYPE_VDPAU: AVHWDeviceType = 1;
pub const AV_HWDEVICE_TYPE_NONE: AVHWDeviceType = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVHWDeviceContext {
    pub av_class: *const AVClass,
    pub internal: *mut AVHWDeviceInternal,
    pub type_0: AVHWDeviceType,
    pub hwctx: *mut libc::c_void,
    pub free: Option<unsafe extern "C" fn(_: *mut AVHWDeviceContext) -> ()>,
    pub user_opaque: *mut libc::c_void,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVHWFramesContext {
    pub av_class: *const AVClass,
    pub internal: *mut AVHWFramesInternal,
    pub device_ref: *mut AVBufferRef,
    pub device_ctx: *mut AVHWDeviceContext,
    pub hwctx: *mut libc::c_void,
    pub free: Option<unsafe extern "C" fn(_: *mut AVHWFramesContext) -> ()>,
    pub user_opaque: *mut libc::c_void,
    pub pool: *mut AVBufferPool,
    pub initial_pool_size: libc::c_int,
    pub format: AVPixelFormat,
    pub sw_format: AVPixelFormat,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVHWFramesConstraints {
    pub valid_hw_formats: *mut AVPixelFormat,
    pub valid_sw_formats: *mut AVPixelFormat,
    pub min_width: libc::c_int,
    pub min_height: libc::c_int,
    pub max_width: libc::c_int,
    pub max_height: libc::c_int,
}
pub type AVCodecID = libc::c_uint;
pub const AV_CODEC_ID_WRAPPED_AVFRAME: AVCodecID = 135169;
pub const AV_CODEC_ID_FFMETADATA: AVCodecID = 135168;
pub const AV_CODEC_ID_MPEG4SYSTEMS: AVCodecID = 131073;
pub const AV_CODEC_ID_MPEG2TS: AVCodecID = 131072;
pub const AV_CODEC_ID_PROBE: AVCodecID = 102400;
pub const AV_CODEC_ID_BIN_DATA: AVCodecID = 100359;
pub const AV_CODEC_ID_TIMED_ID3: AVCodecID = 100358;
pub const AV_CODEC_ID_DVD_NAV: AVCodecID = 100357;
pub const AV_CODEC_ID_SMPTE_KLV: AVCodecID = 100356;
pub const AV_CODEC_ID_OTF: AVCodecID = 100355;
pub const AV_CODEC_ID_IDF: AVCodecID = 100354;
pub const AV_CODEC_ID_XBIN: AVCodecID = 100353;
pub const AV_CODEC_ID_BINTEXT: AVCodecID = 100352;
pub const AV_CODEC_ID_SCTE_35: AVCodecID = 98305;
pub const AV_CODEC_ID_TTF: AVCodecID = 98304;
pub const AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID = 98304;
pub const AV_CODEC_ID_ARIB_CAPTION: AVCodecID = 96272;
pub const AV_CODEC_ID_TTML: AVCodecID = 96271;
pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: AVCodecID = 96270;
pub const AV_CODEC_ID_ASS: AVCodecID = 96269;
pub const AV_CODEC_ID_PJS: AVCodecID = 96268;
pub const AV_CODEC_ID_VPLAYER: AVCodecID = 96267;
pub const AV_CODEC_ID_MPL2: AVCodecID = 96266;
pub const AV_CODEC_ID_WEBVTT: AVCodecID = 96265;
pub const AV_CODEC_ID_SUBRIP: AVCodecID = 96264;
pub const AV_CODEC_ID_SUBVIEWER: AVCodecID = 96263;
pub const AV_CODEC_ID_SUBVIEWER1: AVCodecID = 96262;
pub const AV_CODEC_ID_STL: AVCodecID = 96261;
pub const AV_CODEC_ID_REALTEXT: AVCodecID = 96260;
pub const AV_CODEC_ID_SAMI: AVCodecID = 96259;
pub const AV_CODEC_ID_JACOSUB: AVCodecID = 96258;
pub const AV_CODEC_ID_EIA_608: AVCodecID = 96257;
pub const AV_CODEC_ID_MICRODVD: AVCodecID = 96256;
pub const AV_CODEC_ID_SRT: AVCodecID = 94216;
pub const AV_CODEC_ID_DVB_TELETEXT: AVCodecID = 94215;
pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: AVCodecID = 94214;
pub const AV_CODEC_ID_MOV_TEXT: AVCodecID = 94213;
pub const AV_CODEC_ID_SSA: AVCodecID = 94212;
pub const AV_CODEC_ID_XSUB: AVCodecID = 94211;
pub const AV_CODEC_ID_TEXT: AVCodecID = 94210;
pub const AV_CODEC_ID_DVB_SUBTITLE: AVCodecID = 94209;
pub const AV_CODEC_ID_DVD_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_HCOM: AVCodecID = 88085;
pub const AV_CODEC_ID_ATRAC9: AVCodecID = 88084;
pub const AV_CODEC_ID_SBC: AVCodecID = 88083;
pub const AV_CODEC_ID_APTX_HD: AVCodecID = 88082;
pub const AV_CODEC_ID_APTX: AVCodecID = 88081;
pub const AV_CODEC_ID_DOLBY_E: AVCodecID = 88080;
pub const AV_CODEC_ID_ATRAC3PAL: AVCodecID = 88079;
pub const AV_CODEC_ID_ATRAC3AL: AVCodecID = 88078;
pub const AV_CODEC_ID_DST: AVCodecID = 88077;
pub const AV_CODEC_ID_XMA2: AVCodecID = 88076;
pub const AV_CODEC_ID_XMA1: AVCodecID = 88075;
pub const AV_CODEC_ID_INTERPLAY_ACM: AVCodecID = 88074;
pub const AV_CODEC_ID_4GV: AVCodecID = 88073;
pub const AV_CODEC_ID_DSD_MSBF_PLANAR: AVCodecID = 88072;
pub const AV_CODEC_ID_DSD_LSBF_PLANAR: AVCodecID = 88071;
pub const AV_CODEC_ID_DSD_MSBF: AVCodecID = 88070;
pub const AV_CODEC_ID_DSD_LSBF: AVCodecID = 88069;
pub const AV_CODEC_ID_SMV: AVCodecID = 88068;
pub const AV_CODEC_ID_EVRC: AVCodecID = 88067;
pub const AV_CODEC_ID_SONIC_LS: AVCodecID = 88066;
pub const AV_CODEC_ID_SONIC: AVCodecID = 88065;
pub const AV_CODEC_ID_FFWAVESYNTH: AVCodecID = 88064;
pub const AV_CODEC_ID_CODEC2: AVCodecID = 86083;
pub const AV_CODEC_ID_DSS_SP: AVCodecID = 86082;
pub const AV_CODEC_ID_ON2AVC: AVCodecID = 86081;
pub const AV_CODEC_ID_PAF_AUDIO: AVCodecID = 86080;
pub const AV_CODEC_ID_METASOUND: AVCodecID = 86079;
pub const AV_CODEC_ID_TAK: AVCodecID = 86078;
pub const AV_CODEC_ID_COMFORT_NOISE: AVCodecID = 86077;
pub const AV_CODEC_ID_OPUS: AVCodecID = 86076;
pub const AV_CODEC_ID_ILBC: AVCodecID = 86075;
pub const AV_CODEC_ID_IAC: AVCodecID = 86074;
pub const AV_CODEC_ID_RALF: AVCodecID = 86073;
pub const AV_CODEC_ID_BMV_AUDIO: AVCodecID = 86072;
pub const AV_CODEC_ID_8SVX_FIB: AVCodecID = 86071;
pub const AV_CODEC_ID_8SVX_EXP: AVCodecID = 86070;
pub const AV_CODEC_ID_G729: AVCodecID = 86069;
pub const AV_CODEC_ID_G723_1: AVCodecID = 86068;
pub const AV_CODEC_ID_CELT: AVCodecID = 86067;
pub const AV_CODEC_ID_QDMC: AVCodecID = 86066;
pub const AV_CODEC_ID_AAC_LATM: AVCodecID = 86065;
pub const AV_CODEC_ID_BINKAUDIO_DCT: AVCodecID = 86064;
pub const AV_CODEC_ID_BINKAUDIO_RDFT: AVCodecID = 86063;
pub const AV_CODEC_ID_ATRAC1: AVCodecID = 86062;
pub const AV_CODEC_ID_MP4ALS: AVCodecID = 86061;
pub const AV_CODEC_ID_TRUEHD: AVCodecID = 86060;
pub const AV_CODEC_ID_TWINVQ: AVCodecID = 86059;
pub const AV_CODEC_ID_MP1: AVCodecID = 86058;
pub const AV_CODEC_ID_SIPR: AVCodecID = 86057;
pub const AV_CODEC_ID_EAC3: AVCodecID = 86056;
pub const AV_CODEC_ID_ATRAC3P: AVCodecID = 86055;
pub const AV_CODEC_ID_WMALOSSLESS: AVCodecID = 86054;
pub const AV_CODEC_ID_WMAPRO: AVCodecID = 86053;
pub const AV_CODEC_ID_WMAVOICE: AVCodecID = 86052;
pub const AV_CODEC_ID_SPEEX: AVCodecID = 86051;
pub const AV_CODEC_ID_MUSEPACK8: AVCodecID = 86050;
pub const AV_CODEC_ID_NELLYMOSER: AVCodecID = 86049;
pub const AV_CODEC_ID_APE: AVCodecID = 86048;
pub const AV_CODEC_ID_ATRAC3: AVCodecID = 86047;
pub const AV_CODEC_ID_GSM_MS: AVCodecID = 86046;
pub const AV_CODEC_ID_MLP: AVCodecID = 86045;
pub const AV_CODEC_ID_MUSEPACK7: AVCodecID = 86044;
pub const AV_CODEC_ID_IMC: AVCodecID = 86043;
pub const AV_CODEC_ID_DSICINAUDIO: AVCodecID = 86042;
pub const AV_CODEC_ID_WAVPACK: AVCodecID = 86041;
pub const AV_CODEC_ID_QCELP: AVCodecID = 86040;
pub const AV_CODEC_ID_SMACKAUDIO: AVCodecID = 86039;
pub const AV_CODEC_ID_TTA: AVCodecID = 86038;
pub const AV_CODEC_ID_TRUESPEECH: AVCodecID = 86037;
pub const AV_CODEC_ID_COOK: AVCodecID = 86036;
pub const AV_CODEC_ID_QDM2: AVCodecID = 86035;
pub const AV_CODEC_ID_GSM: AVCodecID = 86034;
pub const AV_CODEC_ID_WESTWOOD_SND1: AVCodecID = 86033;
pub const AV_CODEC_ID_ALAC: AVCodecID = 86032;
pub const AV_CODEC_ID_SHORTEN: AVCodecID = 86031;
pub const AV_CODEC_ID_MP3ON4: AVCodecID = 86030;
pub const AV_CODEC_ID_MP3ADU: AVCodecID = 86029;
pub const AV_CODEC_ID_FLAC: AVCodecID = 86028;
pub const AV_CODEC_ID_VMDAUDIO: AVCodecID = 86027;
pub const AV_CODEC_ID_MACE6: AVCodecID = 86026;
pub const AV_CODEC_ID_MACE3: AVCodecID = 86025;
pub const AV_CODEC_ID_WMAV2: AVCodecID = 86024;
pub const AV_CODEC_ID_WMAV1: AVCodecID = 86023;
pub const AV_CODEC_ID_DVAUDIO: AVCodecID = 86022;
pub const AV_CODEC_ID_VORBIS: AVCodecID = 86021;
pub const AV_CODEC_ID_DTS: AVCodecID = 86020;
pub const AV_CODEC_ID_AC3: AVCodecID = 86019;
pub const AV_CODEC_ID_AAC: AVCodecID = 86018;
pub const AV_CODEC_ID_MP3: AVCodecID = 86017;
pub const AV_CODEC_ID_MP2: AVCodecID = 86016;
pub const AV_CODEC_ID_GREMLIN_DPCM: AVCodecID = 83969;
pub const AV_CODEC_ID_SDX2_DPCM: AVCodecID = 83968;
pub const AV_CODEC_ID_SOL_DPCM: AVCodecID = 81923;
pub const AV_CODEC_ID_XAN_DPCM: AVCodecID = 81922;
pub const AV_CODEC_ID_INTERPLAY_DPCM: AVCodecID = 81921;
pub const AV_CODEC_ID_ROQ_DPCM: AVCodecID = 81920;
pub const AV_CODEC_ID_RA_288: AVCodecID = 77825;
pub const AV_CODEC_ID_RA_144: AVCodecID = 77824;
pub const AV_CODEC_ID_AMR_WB: AVCodecID = 73729;
pub const AV_CODEC_ID_AMR_NB: AVCodecID = 73728;
pub const AV_CODEC_ID_ADPCM_AGM: AVCodecID = 71690;
pub const AV_CODEC_ID_ADPCM_MTAF: AVCodecID = 71689;
pub const AV_CODEC_ID_ADPCM_IMA_DAT4: AVCodecID = 71688;
pub const AV_CODEC_ID_ADPCM_AICA: AVCodecID = 71687;
pub const AV_CODEC_ID_ADPCM_PSX: AVCodecID = 71686;
pub const AV_CODEC_ID_ADPCM_THP_LE: AVCodecID = 71685;
pub const AV_CODEC_ID_ADPCM_G726LE: AVCodecID = 71684;
pub const AV_CODEC_ID_ADPCM_IMA_RAD: AVCodecID = 71683;
pub const AV_CODEC_ID_ADPCM_DTK: AVCodecID = 71682;
pub const AV_CODEC_ID_ADPCM_IMA_OKI: AVCodecID = 71681;
pub const AV_CODEC_ID_ADPCM_AFC: AVCodecID = 71680;
pub const AV_CODEC_ID_ADPCM_VIMA: AVCodecID = 69662;
pub const AV_CODEC_ID_ADPCM_IMA_APC: AVCodecID = 69661;
pub const AV_CODEC_ID_ADPCM_G722: AVCodecID = 69660;
pub const AV_CODEC_ID_ADPCM_IMA_ISS: AVCodecID = 69659;
pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: AVCodecID = 69658;
pub const AV_CODEC_ID_ADPCM_EA_XAS: AVCodecID = 69657;
pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: AVCodecID = 69656;
pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: AVCodecID = 69655;
pub const AV_CODEC_ID_ADPCM_EA_R2: AVCodecID = 69654;
pub const AV_CODEC_ID_ADPCM_EA_R3: AVCodecID = 69653;
pub const AV_CODEC_ID_ADPCM_EA_R1: AVCodecID = 69652;
pub const AV_CODEC_ID_ADPCM_IMA_AMV: AVCodecID = 69651;
pub const AV_CODEC_ID_ADPCM_THP: AVCodecID = 69650;
pub const AV_CODEC_ID_ADPCM_SBPRO_2: AVCodecID = 69649;
pub const AV_CODEC_ID_ADPCM_SBPRO_3: AVCodecID = 69648;
pub const AV_CODEC_ID_ADPCM_SBPRO_4: AVCodecID = 69647;
pub const AV_CODEC_ID_ADPCM_YAMAHA: AVCodecID = 69646;
pub const AV_CODEC_ID_ADPCM_SWF: AVCodecID = 69645;
pub const AV_CODEC_ID_ADPCM_CT: AVCodecID = 69644;
pub const AV_CODEC_ID_ADPCM_G726: AVCodecID = 69643;
pub const AV_CODEC_ID_ADPCM_EA: AVCodecID = 69642;
pub const AV_CODEC_ID_ADPCM_ADX: AVCodecID = 69641;
pub const AV_CODEC_ID_ADPCM_XA: AVCodecID = 69640;
pub const AV_CODEC_ID_ADPCM_4XM: AVCodecID = 69639;
pub const AV_CODEC_ID_ADPCM_MS: AVCodecID = 69638;
pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: AVCodecID = 69637;
pub const AV_CODEC_ID_ADPCM_IMA_WS: AVCodecID = 69636;
pub const AV_CODEC_ID_ADPCM_IMA_DK4: AVCodecID = 69635;
pub const AV_CODEC_ID_ADPCM_IMA_DK3: AVCodecID = 69634;
pub const AV_CODEC_ID_ADPCM_IMA_WAV: AVCodecID = 69633;
pub const AV_CODEC_ID_ADPCM_IMA_QT: AVCodecID = 69632;
pub const AV_CODEC_ID_PCM_VIDC: AVCodecID = 67588;
pub const AV_CODEC_ID_PCM_F24LE: AVCodecID = 67587;
pub const AV_CODEC_ID_PCM_F16LE: AVCodecID = 67586;
pub const AV_CODEC_ID_PCM_S64BE: AVCodecID = 67585;
pub const AV_CODEC_ID_PCM_S64LE: AVCodecID = 67584;
pub const AV_CODEC_ID_PCM_S16BE_PLANAR: AVCodecID = 65566;
pub const AV_CODEC_ID_PCM_S32LE_PLANAR: AVCodecID = 65565;
pub const AV_CODEC_ID_PCM_S24LE_PLANAR: AVCodecID = 65564;
pub const AV_CODEC_ID_PCM_S8_PLANAR: AVCodecID = 65563;
pub const AV_CODEC_ID_S302M: AVCodecID = 65562;
pub const AV_CODEC_ID_PCM_LXF: AVCodecID = 65561;
pub const AV_CODEC_ID_PCM_BLURAY: AVCodecID = 65560;
pub const AV_CODEC_ID_PCM_F64LE: AVCodecID = 65559;
pub const AV_CODEC_ID_PCM_F64BE: AVCodecID = 65558;
pub const AV_CODEC_ID_PCM_F32LE: AVCodecID = 65557;
pub const AV_CODEC_ID_PCM_F32BE: AVCodecID = 65556;
pub const AV_CODEC_ID_PCM_DVD: AVCodecID = 65555;
pub const AV_CODEC_ID_PCM_S16LE_PLANAR: AVCodecID = 65554;
pub const AV_CODEC_ID_PCM_ZORK: AVCodecID = 65553;
pub const AV_CODEC_ID_PCM_S24DAUD: AVCodecID = 65552;
pub const AV_CODEC_ID_PCM_U24BE: AVCodecID = 65551;
pub const AV_CODEC_ID_PCM_U24LE: AVCodecID = 65550;
pub const AV_CODEC_ID_PCM_S24BE: AVCodecID = 65549;
pub const AV_CODEC_ID_PCM_S24LE: AVCodecID = 65548;
pub const AV_CODEC_ID_PCM_U32BE: AVCodecID = 65547;
pub const AV_CODEC_ID_PCM_U32LE: AVCodecID = 65546;
pub const AV_CODEC_ID_PCM_S32BE: AVCodecID = 65545;
pub const AV_CODEC_ID_PCM_S32LE: AVCodecID = 65544;
pub const AV_CODEC_ID_PCM_ALAW: AVCodecID = 65543;
pub const AV_CODEC_ID_PCM_MULAW: AVCodecID = 65542;
pub const AV_CODEC_ID_PCM_U8: AVCodecID = 65541;
pub const AV_CODEC_ID_PCM_S8: AVCodecID = 65540;
pub const AV_CODEC_ID_PCM_U16BE: AVCodecID = 65539;
pub const AV_CODEC_ID_PCM_U16LE: AVCodecID = 65538;
pub const AV_CODEC_ID_PCM_S16BE: AVCodecID = 65537;
pub const AV_CODEC_ID_PCM_S16LE: AVCodecID = 65536;
pub const AV_CODEC_ID_FIRST_AUDIO: AVCodecID = 65536;
pub const AV_CODEC_ID_VP4: AVCodecID = 32813;
pub const AV_CODEC_ID_LSCR: AVCodecID = 32812;
pub const AV_CODEC_ID_AGM: AVCodecID = 32811;
pub const AV_CODEC_ID_ARBC: AVCodecID = 32810;
pub const AV_CODEC_ID_HYMT: AVCodecID = 32809;
pub const AV_CODEC_ID_RASC: AVCodecID = 32808;
pub const AV_CODEC_ID_WCMV: AVCodecID = 32807;
pub const AV_CODEC_ID_MWSC: AVCodecID = 32806;
pub const AV_CODEC_ID_PROSUMER: AVCodecID = 32805;
pub const AV_CODEC_ID_IMM4: AVCodecID = 32804;
pub const AV_CODEC_ID_FITS: AVCodecID = 32803;
pub const AV_CODEC_ID_GDV: AVCodecID = 32802;
pub const AV_CODEC_ID_SVG: AVCodecID = 32801;
pub const AV_CODEC_ID_SRGC: AVCodecID = 32800;
pub const AV_CODEC_ID_MSCC: AVCodecID = 32799;
pub const AV_CODEC_ID_BITPACKED: AVCodecID = 32798;
pub const AV_CODEC_ID_AV1: AVCodecID = 32797;
pub const AV_CODEC_ID_XPM: AVCodecID = 32796;
pub const AV_CODEC_ID_CLEARVIDEO: AVCodecID = 32795;
pub const AV_CODEC_ID_SCPR: AVCodecID = 32794;
pub const AV_CODEC_ID_FMVC: AVCodecID = 32793;
pub const AV_CODEC_ID_SPEEDHQ: AVCodecID = 32792;
pub const AV_CODEC_ID_PIXLET: AVCodecID = 32791;
pub const AV_CODEC_ID_PSD: AVCodecID = 32790;
pub const AV_CODEC_ID_YLC: AVCodecID = 32789;
pub const AV_CODEC_ID_SHEERVIDEO: AVCodecID = 32788;
pub const AV_CODEC_ID_MAGICYUV: AVCodecID = 32787;
pub const AV_CODEC_ID_M101: AVCodecID = 32786;
pub const AV_CODEC_ID_TRUEMOTION2RT: AVCodecID = 32785;
pub const AV_CODEC_ID_CFHD: AVCodecID = 32784;
pub const AV_CODEC_ID_DAALA: AVCodecID = 32783;
pub const AV_CODEC_ID_APNG: AVCodecID = 32782;
pub const AV_CODEC_ID_SMVJPEG: AVCodecID = 32781;
pub const AV_CODEC_ID_SNOW: AVCodecID = 32780;
pub const AV_CODEC_ID_XFACE: AVCodecID = 32779;
pub const AV_CODEC_ID_CPIA: AVCodecID = 32778;
pub const AV_CODEC_ID_AVRN: AVCodecID = 32777;
pub const AV_CODEC_ID_YUV4: AVCodecID = 32776;
pub const AV_CODEC_ID_V408: AVCodecID = 32775;
pub const AV_CODEC_ID_V308: AVCodecID = 32774;
pub const AV_CODEC_ID_TARGA_Y216: AVCodecID = 32773;
pub const AV_CODEC_ID_AYUV: AVCodecID = 32772;
pub const AV_CODEC_ID_AVUI: AVCodecID = 32771;
pub const AV_CODEC_ID_012V: AVCodecID = 32770;
pub const AV_CODEC_ID_AVRP: AVCodecID = 32769;
pub const AV_CODEC_ID_Y41P: AVCodecID = 32768;
pub const AV_CODEC_ID_AVS2: AVCodecID = 192;
pub const AV_CODEC_ID_RSCC: AVCodecID = 191;
pub const AV_CODEC_ID_SCREENPRESSO: AVCodecID = 190;
pub const AV_CODEC_ID_DXV: AVCodecID = 189;
pub const AV_CODEC_ID_DDS: AVCodecID = 188;
pub const AV_CODEC_ID_HAP: AVCodecID = 187;
pub const AV_CODEC_ID_HQ_HQA: AVCodecID = 186;
pub const AV_CODEC_ID_TDSC: AVCodecID = 185;
pub const AV_CODEC_ID_HQX: AVCodecID = 184;
pub const AV_CODEC_ID_MVC2: AVCodecID = 183;
pub const AV_CODEC_ID_MVC1: AVCodecID = 182;
pub const AV_CODEC_ID_SGIRLE: AVCodecID = 181;
pub const AV_CODEC_ID_SANM: AVCodecID = 180;
pub const AV_CODEC_ID_VP7: AVCodecID = 179;
pub const AV_CODEC_ID_EXR: AVCodecID = 178;
pub const AV_CODEC_ID_PAF_VIDEO: AVCodecID = 177;
pub const AV_CODEC_ID_BRENDER_PIX: AVCodecID = 176;
pub const AV_CODEC_ID_ALIAS_PIX: AVCodecID = 175;
pub const AV_CODEC_ID_FIC: AVCodecID = 174;
pub const AV_CODEC_ID_HEVC: AVCodecID = 173;
pub const AV_CODEC_ID_HNM4_VIDEO: AVCodecID = 172;
pub const AV_CODEC_ID_WEBP: AVCodecID = 171;
pub const AV_CODEC_ID_G2M: AVCodecID = 170;
pub const AV_CODEC_ID_ESCAPE130: AVCodecID = 169;
pub const AV_CODEC_ID_AIC: AVCodecID = 168;
pub const AV_CODEC_ID_VP9: AVCodecID = 167;
pub const AV_CODEC_ID_MSS2: AVCodecID = 166;
pub const AV_CODEC_ID_CLLC: AVCodecID = 165;
pub const AV_CODEC_ID_MTS2: AVCodecID = 164;
pub const AV_CODEC_ID_TSCC2: AVCodecID = 163;
pub const AV_CODEC_ID_MSA1: AVCodecID = 162;
pub const AV_CODEC_ID_MSS1: AVCodecID = 161;
pub const AV_CODEC_ID_ZEROCODEC: AVCodecID = 160;
pub const AV_CODEC_ID_XBM: AVCodecID = 159;
pub const AV_CODEC_ID_CDXL: AVCodecID = 158;
pub const AV_CODEC_ID_XWD: AVCodecID = 157;
pub const AV_CODEC_ID_V410: AVCodecID = 156;
pub const AV_CODEC_ID_DXTORY: AVCodecID = 155;
pub const AV_CODEC_ID_VBLE: AVCodecID = 154;
pub const AV_CODEC_ID_BMV_VIDEO: AVCodecID = 153;
pub const AV_CODEC_ID_UTVIDEO: AVCodecID = 152;
pub const AV_CODEC_ID_VC1IMAGE: AVCodecID = 151;
pub const AV_CODEC_ID_WMV3IMAGE: AVCodecID = 150;
pub const AV_CODEC_ID_DFA: AVCodecID = 149;
pub const AV_CODEC_ID_JV: AVCodecID = 148;
pub const AV_CODEC_ID_PRORES: AVCodecID = 147;
pub const AV_CODEC_ID_LAGARITH: AVCodecID = 146;
pub const AV_CODEC_ID_MXPEG: AVCodecID = 145;
pub const AV_CODEC_ID_R10K: AVCodecID = 144;
pub const AV_CODEC_ID_A64_MULTI5: AVCodecID = 143;
pub const AV_CODEC_ID_A64_MULTI: AVCodecID = 142;
pub const AV_CODEC_ID_ANSI: AVCodecID = 141;
pub const AV_CODEC_ID_PICTOR: AVCodecID = 140;
pub const AV_CODEC_ID_VP8: AVCodecID = 139;
pub const AV_CODEC_ID_YOP: AVCodecID = 138;
pub const AV_CODEC_ID_KGV1: AVCodecID = 137;
pub const AV_CODEC_ID_IFF_ILBM: AVCodecID = 136;
pub const AV_CODEC_ID_BINKVIDEO: AVCodecID = 135;
pub const AV_CODEC_ID_ANM: AVCodecID = 134;
pub const AV_CODEC_ID_R210: AVCodecID = 133;
pub const AV_CODEC_ID_CDGRAPHICS: AVCodecID = 132;
pub const AV_CODEC_ID_FLASHSV2: AVCodecID = 131;
pub const AV_CODEC_ID_FRWU: AVCodecID = 130;
pub const AV_CODEC_ID_MAD: AVCodecID = 129;
pub const AV_CODEC_ID_DPX: AVCodecID = 128;
pub const AV_CODEC_ID_V210: AVCodecID = 127;
pub const AV_CODEC_ID_TMV: AVCodecID = 126;
pub const AV_CODEC_ID_V210X: AVCodecID = 125;
pub const AV_CODEC_ID_AURA2: AVCodecID = 124;
pub const AV_CODEC_ID_AURA: AVCodecID = 123;
pub const AV_CODEC_ID_TQI: AVCodecID = 122;
pub const AV_CODEC_ID_TGQ: AVCodecID = 121;
pub const AV_CODEC_ID_TGV: AVCodecID = 120;
pub const AV_CODEC_ID_MOTIONPIXELS: AVCodecID = 119;
pub const AV_CODEC_ID_CMV: AVCodecID = 118;
pub const AV_CODEC_ID_BFI: AVCodecID = 117;
pub const AV_CODEC_ID_DIRAC: AVCodecID = 116;
pub const AV_CODEC_ID_ESCAPE124: AVCodecID = 115;
pub const AV_CODEC_ID_RL2: AVCodecID = 114;
pub const AV_CODEC_ID_MIMIC: AVCodecID = 113;
pub const AV_CODEC_ID_INDEO5: AVCodecID = 112;
pub const AV_CODEC_ID_INDEO4: AVCodecID = 111;
pub const AV_CODEC_ID_SUNRAST: AVCodecID = 110;
pub const AV_CODEC_ID_PCX: AVCodecID = 109;
pub const AV_CODEC_ID_VB: AVCodecID = 108;
pub const AV_CODEC_ID_AMV: AVCodecID = 107;
pub const AV_CODEC_ID_VP6A: AVCodecID = 106;
pub const AV_CODEC_ID_TXD: AVCodecID = 105;
pub const AV_CODEC_ID_PTX: AVCodecID = 104;
pub const AV_CODEC_ID_BETHSOFTVID: AVCodecID = 103;
pub const AV_CODEC_ID_C93: AVCodecID = 102;
pub const AV_CODEC_ID_SGI: AVCodecID = 101;
pub const AV_CODEC_ID_THP: AVCodecID = 100;
pub const AV_CODEC_ID_DNXHD: AVCodecID = 99;
pub const AV_CODEC_ID_DXA: AVCodecID = 98;
pub const AV_CODEC_ID_GIF: AVCodecID = 97;
pub const AV_CODEC_ID_TIFF: AVCodecID = 96;
pub const AV_CODEC_ID_TIERTEXSEQVIDEO: AVCodecID = 95;
pub const AV_CODEC_ID_DSICINVIDEO: AVCodecID = 94;
pub const AV_CODEC_ID_TARGA: AVCodecID = 93;
pub const AV_CODEC_ID_VP6F: AVCodecID = 92;
pub const AV_CODEC_ID_VP6: AVCodecID = 91;
pub const AV_CODEC_ID_VP5: AVCodecID = 90;
pub const AV_CODEC_ID_VMNC: AVCodecID = 89;
pub const AV_CODEC_ID_JPEG2000: AVCodecID = 88;
pub const AV_CODEC_ID_CAVS: AVCodecID = 87;
pub const AV_CODEC_ID_FLASHSV: AVCodecID = 86;
pub const AV_CODEC_ID_KMVC: AVCodecID = 85;
pub const AV_CODEC_ID_NUV: AVCodecID = 84;
pub const AV_CODEC_ID_SMACKVIDEO: AVCodecID = 83;
pub const AV_CODEC_ID_AVS: AVCodecID = 82;
pub const AV_CODEC_ID_ZMBV: AVCodecID = 81;
pub const AV_CODEC_ID_MMVIDEO: AVCodecID = 80;
pub const AV_CODEC_ID_CSCD: AVCodecID = 79;
pub const AV_CODEC_ID_BMP: AVCodecID = 78;
pub const AV_CODEC_ID_TRUEMOTION2: AVCodecID = 77;
pub const AV_CODEC_ID_FRAPS: AVCodecID = 76;
pub const AV_CODEC_ID_INDEO2: AVCodecID = 75;
pub const AV_CODEC_ID_AASC: AVCodecID = 74;
pub const AV_CODEC_ID_WNV1: AVCodecID = 73;
pub const AV_CODEC_ID_LOCO: AVCodecID = 72;
pub const AV_CODEC_ID_WMV3: AVCodecID = 71;
pub const AV_CODEC_ID_VC1: AVCodecID = 70;
pub const AV_CODEC_ID_RV40: AVCodecID = 69;
pub const AV_CODEC_ID_RV30: AVCodecID = 68;
pub const AV_CODEC_ID_FFVHUFF: AVCodecID = 67;
pub const AV_CODEC_ID_PAM: AVCodecID = 66;
pub const AV_CODEC_ID_PGMYUV: AVCodecID = 65;
pub const AV_CODEC_ID_PGM: AVCodecID = 64;
pub const AV_CODEC_ID_PBM: AVCodecID = 63;
pub const AV_CODEC_ID_PPM: AVCodecID = 62;
pub const AV_CODEC_ID_PNG: AVCodecID = 61;
pub const AV_CODEC_ID_QPEG: AVCodecID = 60;
pub const AV_CODEC_ID_VIXL: AVCodecID = 59;
pub const AV_CODEC_ID_QDRAW: AVCodecID = 58;
pub const AV_CODEC_ID_ULTI: AVCodecID = 57;
pub const AV_CODEC_ID_TSCC: AVCodecID = 56;
pub const AV_CODEC_ID_QTRLE: AVCodecID = 55;
pub const AV_CODEC_ID_ZLIB: AVCodecID = 54;
pub const AV_CODEC_ID_MSZH: AVCodecID = 53;
pub const AV_CODEC_ID_VMDVIDEO: AVCodecID = 52;
pub const AV_CODEC_ID_TRUEMOTION1: AVCodecID = 51;
pub const AV_CODEC_ID_FLIC: AVCodecID = 50;
pub const AV_CODEC_ID_SMC: AVCodecID = 49;
pub const AV_CODEC_ID_8BPS: AVCodecID = 48;
pub const AV_CODEC_ID_IDCIN: AVCodecID = 47;
pub const AV_CODEC_ID_MSVIDEO1: AVCodecID = 46;
pub const AV_CODEC_ID_MSRLE: AVCodecID = 45;
pub const AV_CODEC_ID_WS_VQA: AVCodecID = 44;
pub const AV_CODEC_ID_CINEPAK: AVCodecID = 43;
pub const AV_CODEC_ID_RPZA: AVCodecID = 42;
pub const AV_CODEC_ID_XAN_WC4: AVCodecID = 41;
pub const AV_CODEC_ID_XAN_WC3: AVCodecID = 40;
pub const AV_CODEC_ID_INTERPLAY_VIDEO: AVCodecID = 39;
pub const AV_CODEC_ID_ROQ: AVCodecID = 38;
pub const AV_CODEC_ID_MDEC: AVCodecID = 37;
pub const AV_CODEC_ID_CLJR: AVCodecID = 36;
pub const AV_CODEC_ID_VCR1: AVCodecID = 35;
pub const AV_CODEC_ID_4XM: AVCodecID = 34;
pub const AV_CODEC_ID_FFV1: AVCodecID = 33;
pub const AV_CODEC_ID_ASV2: AVCodecID = 32;
pub const AV_CODEC_ID_ASV1: AVCodecID = 31;
pub const AV_CODEC_ID_THEORA: AVCodecID = 30;
pub const AV_CODEC_ID_VP3: AVCodecID = 29;
pub const AV_CODEC_ID_INDEO3: AVCodecID = 28;
pub const AV_CODEC_ID_H264: AVCodecID = 27;
pub const AV_CODEC_ID_CYUV: AVCodecID = 26;
pub const AV_CODEC_ID_HUFFYUV: AVCodecID = 25;
pub const AV_CODEC_ID_DVVIDEO: AVCodecID = 24;
pub const AV_CODEC_ID_SVQ3: AVCodecID = 23;
pub const AV_CODEC_ID_SVQ1: AVCodecID = 22;
pub const AV_CODEC_ID_FLV1: AVCodecID = 21;
pub const AV_CODEC_ID_H263I: AVCodecID = 20;
pub const AV_CODEC_ID_H263P: AVCodecID = 19;
pub const AV_CODEC_ID_WMV2: AVCodecID = 18;
pub const AV_CODEC_ID_WMV1: AVCodecID = 17;
pub const AV_CODEC_ID_MSMPEG4V3: AVCodecID = 16;
pub const AV_CODEC_ID_MSMPEG4V2: AVCodecID = 15;
pub const AV_CODEC_ID_MSMPEG4V1: AVCodecID = 14;
pub const AV_CODEC_ID_RAWVIDEO: AVCodecID = 13;
pub const AV_CODEC_ID_MPEG4: AVCodecID = 12;
pub const AV_CODEC_ID_JPEGLS: AVCodecID = 11;
pub const AV_CODEC_ID_SP5X: AVCodecID = 10;
pub const AV_CODEC_ID_LJPEG: AVCodecID = 9;
pub const AV_CODEC_ID_MJPEGB: AVCodecID = 8;
pub const AV_CODEC_ID_MJPEG: AVCodecID = 7;
pub const AV_CODEC_ID_RV20: AVCodecID = 6;
pub const AV_CODEC_ID_RV10: AVCodecID = 5;
pub const AV_CODEC_ID_H263: AVCodecID = 4;
pub const AV_CODEC_ID_H261: AVCodecID = 3;
pub const AV_CODEC_ID_MPEG2VIDEO: AVCodecID = 2;
pub const AV_CODEC_ID_MPEG1VIDEO: AVCodecID = 1;
pub const AV_CODEC_ID_NONE: AVCodecID = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub type_0: AVMediaType,
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub props: libc::c_int,
    pub mime_types: *const *const libc::c_char,
    pub profiles: *const AVProfile,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVProfile {
    pub profile: libc::c_int,
    pub name: *const libc::c_char,
}
pub type AVDiscard = libc::c_int;
pub const AVDISCARD_ALL: AVDiscard = 48;
pub const AVDISCARD_NONKEY: AVDiscard = 32;
pub const AVDISCARD_NONINTRA: AVDiscard = 24;
pub const AVDISCARD_BIDIR: AVDiscard = 16;
pub const AVDISCARD_NONREF: AVDiscard = 8;
pub const AVDISCARD_DEFAULT: AVDiscard = 0;
pub const AVDISCARD_NONE: AVDiscard = -16;
pub type AVAudioServiceType = libc::c_uint;
pub const AV_AUDIO_SERVICE_TYPE_NB: AVAudioServiceType = 9;
pub const AV_AUDIO_SERVICE_TYPE_KARAOKE: AVAudioServiceType = 8;
pub const AV_AUDIO_SERVICE_TYPE_VOICE_OVER: AVAudioServiceType = 7;
pub const AV_AUDIO_SERVICE_TYPE_EMERGENCY: AVAudioServiceType = 6;
pub const AV_AUDIO_SERVICE_TYPE_COMMENTARY: AVAudioServiceType = 5;
pub const AV_AUDIO_SERVICE_TYPE_DIALOGUE: AVAudioServiceType = 4;
pub const AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: AVAudioServiceType = 3;
pub const AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: AVAudioServiceType = 2;
pub const AV_AUDIO_SERVICE_TYPE_EFFECTS: AVAudioServiceType = 1;
pub const AV_AUDIO_SERVICE_TYPE_MAIN: AVAudioServiceType = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct RcOverride {
    pub start_frame: libc::c_int,
    pub end_frame: libc::c_int,
    pub qscale: libc::c_int,
    pub quality_factor: libc::c_float,
}
pub type AVPacketSideDataType = libc::c_uint;
pub const AV_PKT_DATA_NB: AVPacketSideDataType = 27;
pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub type_0: AVPacketSideDataType,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub stream_index: libc::c_int,
    pub flags: libc::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: libc::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub convergence_duration: int64_t,
}
pub type AVFieldOrder = libc::c_uint;
pub const AV_FIELD_BT: AVFieldOrder = 5;
pub const AV_FIELD_TB: AVFieldOrder = 4;
pub const AV_FIELD_BB: AVFieldOrder = 3;
pub const AV_FIELD_TT: AVFieldOrder = 2;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: libc::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: libc::c_uint,
    pub priv_data: *mut libc::c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut libc::c_void,
    pub bit_rate: int64_t,
    pub bit_rate_tolerance: libc::c_int,
    pub global_quality: libc::c_int,
    pub compression_level: libc::c_int,
    pub flags: libc::c_int,
    pub flags2: libc::c_int,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub time_base: AVRational,
    pub ticks_per_frame: libc::c_int,
    pub delay: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub gop_size: libc::c_int,
    pub pix_fmt: AVPixelFormat,
    pub draw_horiz_band: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                     _: *const AVFrame,
                                                     _: *mut libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub get_format: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                _: *const AVPixelFormat)
                               -> AVPixelFormat>,
    pub max_b_frames: libc::c_int,
    pub b_quant_factor: libc::c_float,
    pub b_frame_strategy: libc::c_int,
    pub b_quant_offset: libc::c_float,
    pub has_b_frames: libc::c_int,
    pub mpeg_quant: libc::c_int,
    pub i_quant_factor: libc::c_float,
    pub i_quant_offset: libc::c_float,
    pub lumi_masking: libc::c_float,
    pub temporal_cplx_masking: libc::c_float,
    pub spatial_cplx_masking: libc::c_float,
    pub p_masking: libc::c_float,
    pub dark_masking: libc::c_float,
    pub slice_count: libc::c_int,
    pub prediction_method: libc::c_int,
    pub slice_offset: *mut libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: libc::c_int,
    pub me_sub_cmp: libc::c_int,
    pub mb_cmp: libc::c_int,
    pub ildct_cmp: libc::c_int,
    pub dia_size: libc::c_int,
    pub last_predictor_count: libc::c_int,
    pub pre_me: libc::c_int,
    pub me_pre_cmp: libc::c_int,
    pub pre_dia_size: libc::c_int,
    pub me_subpel_quality: libc::c_int,
    pub me_range: libc::c_int,
    pub slice_flags: libc::c_int,
    pub mb_decision: libc::c_int,
    pub intra_matrix: *mut uint16_t,
    pub inter_matrix: *mut uint16_t,
    pub scenechange_threshold: libc::c_int,
    pub noise_reduction: libc::c_int,
    pub intra_dc_precision: libc::c_int,
    pub skip_top: libc::c_int,
    pub skip_bottom: libc::c_int,
    pub mb_lmin: libc::c_int,
    pub mb_lmax: libc::c_int,
    pub me_penalty_compensation: libc::c_int,
    pub bidir_refine: libc::c_int,
    pub brd_scale: libc::c_int,
    pub keyint_min: libc::c_int,
    pub refs: libc::c_int,
    pub chromaoffset: libc::c_int,
    pub mv0_threshold: libc::c_int,
    pub b_sensitivity: libc::c_int,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub slices: libc::c_int,
    pub field_order: AVFieldOrder,
    pub sample_rate: libc::c_int,
    pub channels: libc::c_int,
    pub sample_fmt: AVSampleFormat,
    pub frame_size: libc::c_int,
    pub frame_number: libc::c_int,
    pub block_align: libc::c_int,
    pub cutoff: libc::c_int,
    pub channel_layout: uint64_t,
    pub request_channel_layout: uint64_t,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub get_buffer2: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                 _: *mut AVFrame,
                                                 _: libc::c_int)
                                -> libc::c_int>,
    pub refcounted_frames: libc::c_int,
    pub qcompress: libc::c_float,
    pub qblur: libc::c_float,
    pub qmin: libc::c_int,
    pub qmax: libc::c_int,
    pub max_qdiff: libc::c_int,
    pub rc_buffer_size: libc::c_int,
    pub rc_override_count: libc::c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: int64_t,
    pub rc_min_rate: int64_t,
    pub rc_max_available_vbv_use: libc::c_float,
    pub rc_min_vbv_overflow_use: libc::c_float,
    pub rc_initial_buffer_occupancy: libc::c_int,
    pub coder_type: libc::c_int,
    pub context_model: libc::c_int,
    pub frame_skip_threshold: libc::c_int,
    pub frame_skip_factor: libc::c_int,
    pub frame_skip_exp: libc::c_int,
    pub frame_skip_cmp: libc::c_int,
    pub trellis: libc::c_int,
    pub min_prediction_order: libc::c_int,
    pub max_prediction_order: libc::c_int,
    pub timecode_frame_start: int64_t,
    pub rtp_callback: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                  _: *mut libc::c_void,
                                                  _: libc::c_int,
                                                  _: libc::c_int) -> ()>,
    pub rtp_payload_size: libc::c_int,
    pub mv_bits: libc::c_int,
    pub header_bits: libc::c_int,
    pub i_tex_bits: libc::c_int,
    pub p_tex_bits: libc::c_int,
    pub i_count: libc::c_int,
    pub p_count: libc::c_int,
    pub skip_count: libc::c_int,
    pub misc_bits: libc::c_int,
    pub frame_bits: libc::c_int,
    pub stats_out: *mut libc::c_char,
    pub stats_in: *mut libc::c_char,
    pub workaround_bugs: libc::c_int,
    pub strict_std_compliance: libc::c_int,
    pub error_concealment: libc::c_int,
    pub debug: libc::c_int,
    pub err_recognition: libc::c_int,
    pub reordered_opaque: int64_t,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub dct_algo: libc::c_int,
    pub idct_algo: libc::c_int,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub lowres: libc::c_int,
    pub coded_frame: *mut AVFrame,
    pub thread_count: libc::c_int,
    pub thread_type: libc::c_int,
    pub active_thread_type: libc::c_int,
    pub thread_safe_callbacks: libc::c_int,
    pub execute: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                             _:
                                                 Option<unsafe extern "C" fn(_:
                                                                                 *mut AVCodecContext,
                                                                             _:
                                                                                 *mut libc::c_void)
                                                            -> libc::c_int>,
                                             _: *mut libc::c_void,
                                             _: *mut libc::c_int,
                                             _: libc::c_int, _: libc::c_int)
                            -> libc::c_int>,
    pub execute2: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                              _:
                                                  Option<unsafe extern "C" fn(_:
                                                                                  *mut AVCodecContext,
                                                                              _:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  libc::c_int,
                                                                              _:
                                                                                  libc::c_int)
                                                             -> libc::c_int>,
                                              _: *mut libc::c_void,
                                              _: *mut libc::c_int,
                                              _: libc::c_int) -> libc::c_int>,
    pub nsse_weight: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub subtitle_header: *mut uint8_t,
    pub subtitle_header_size: libc::c_int,
    pub vbv_delay: uint64_t,
    pub side_data_only_packets: libc::c_int,
    pub initial_padding: libc::c_int,
    pub framerate: AVRational,
    pub sw_pix_fmt: AVPixelFormat,
    pub pkt_timebase: AVRational,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub pts_correction_num_faulty_pts: int64_t,
    pub pts_correction_num_faulty_dts: int64_t,
    pub pts_correction_last_pts: int64_t,
    pub pts_correction_last_dts: int64_t,
    pub sub_charenc: *mut libc::c_char,
    pub sub_charenc_mode: libc::c_int,
    pub skip_alpha: libc::c_int,
    pub seek_preroll: libc::c_int,
    pub debug_mv: libc::c_int,
    pub chroma_intra_matrix: *mut uint16_t,
    pub dump_separator: *mut uint8_t,
    pub codec_whitelist: *mut libc::c_char,
    pub properties: libc::c_uint,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: libc::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub sub_text_format: libc::c_int,
    pub trailing_padding: libc::c_int,
    pub max_pixels: int64_t,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: libc::c_int,
    pub apply_cropping: libc::c_int,
    pub extra_hw_frames: libc::c_int,
    pub discard_damaged_percentage: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVHWAccel {
    pub name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: libc::c_int,
    pub alloc_frame: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                 _: *mut AVFrame)
                                -> libc::c_int>,
    pub start_frame: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                 _: *const uint8_t,
                                                 _: uint32_t) -> libc::c_int>,
    pub decode_params: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                   _: libc::c_int,
                                                   _: *const uint8_t,
                                                   _: uint32_t)
                                  -> libc::c_int>,
    pub decode_slice: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                  _: *const uint8_t,
                                                  _: uint32_t)
                                 -> libc::c_int>,
    pub end_frame: Option<unsafe extern "C" fn(_: *mut AVCodecContext)
                              -> libc::c_int>,
    pub frame_priv_data_size: libc::c_int,
    pub decode_mb: Option<unsafe extern "C" fn(_: *mut MpegEncContext) -> ()>,
    pub init: Option<unsafe extern "C" fn(_: *mut AVCodecContext)
                         -> libc::c_int>,
    pub uninit: Option<unsafe extern "C" fn(_: *mut AVCodecContext)
                           -> libc::c_int>,
    pub priv_data_size: libc::c_int,
    pub caps_internal: libc::c_int,
    pub frame_params: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                  _: *mut AVBufferRef)
                                 -> libc::c_int>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVCodec {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: libc::c_int,
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
    pub supported_samplerates: *const libc::c_int,
    pub sample_fmts: *const AVSampleFormat,
    pub channel_layouts: *const uint64_t,
    pub max_lowres: uint8_t,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const libc::c_char,
    pub priv_data_size: libc::c_int,
    pub next: *mut AVCodec,
    pub init_thread_copy: Option<unsafe extern "C" fn(_: *mut AVCodecContext)
                                     -> libc::c_int>,
    pub update_thread_context: Option<unsafe extern "C" fn(_:
                                                               *mut AVCodecContext,
                                                           _:
                                                               *const AVCodecContext)
                                          -> libc::c_int>,
    pub defaults: *const AVCodecDefault,
    pub init_static_data: Option<unsafe extern "C" fn(_: *mut AVCodec) -> ()>,
    pub init: Option<unsafe extern "C" fn(_: *mut AVCodecContext)
                         -> libc::c_int>,
    pub encode_sub: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                _: *mut uint8_t,
                                                _: libc::c_int,
                                                _: *const AVSubtitle)
                               -> libc::c_int>,
    pub encode2: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                             _: *mut AVPacket,
                                             _: *const AVFrame,
                                             _: *mut libc::c_int)
                            -> libc::c_int>,
    pub decode: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                            _: *mut libc::c_void,
                                            _: *mut libc::c_int,
                                            _: *mut AVPacket) -> libc::c_int>,
    pub close: Option<unsafe extern "C" fn(_: *mut AVCodecContext)
                          -> libc::c_int>,
    pub send_frame: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                _: *const AVFrame)
                               -> libc::c_int>,
    pub receive_packet: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                    _: *mut AVPacket)
                                   -> libc::c_int>,
    pub receive_frame: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                                   _: *mut AVFrame)
                                  -> libc::c_int>,
    pub flush: Option<unsafe extern "C" fn(_: *mut AVCodecContext) -> ()>,
    pub caps_internal: libc::c_int,
    pub bsfs: *const libc::c_char,
    pub hw_configs: *mut *const AVCodecHWConfigInternal,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVSubtitle {
    pub format: uint16_t,
    pub start_display_time: uint32_t,
    pub end_display_time: uint32_t,
    pub num_rects: libc::c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: int64_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVSubtitleRect {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub nb_colors: libc::c_int,
    pub pict: AVPicture,
    pub data: [*mut uint8_t; 4],
    pub linesize: [libc::c_int; 4],
    pub type_0: AVSubtitleType,
    pub text: *mut libc::c_char,
    pub ass: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type AVSubtitleType = libc::c_uint;
pub const SUBTITLE_ASS: AVSubtitleType = 3;
pub const SUBTITLE_TEXT: AVSubtitleType = 2;
pub const SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const SUBTITLE_NONE: AVSubtitleType = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVPicture {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVCodecParameters {
    pub codec_type: AVMediaType,
    pub codec_id: AVCodecID,
    pub codec_tag: uint32_t,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub format: libc::c_int,
    pub bit_rate: int64_t,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub field_order: AVFieldOrder,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub video_delay: libc::c_int,
    pub channel_layout: uint64_t,
    pub channels: libc::c_int,
    pub sample_rate: libc::c_int,
    pub block_align: libc::c_int,
    pub frame_size: libc::c_int,
    pub initial_padding: libc::c_int,
    pub trailing_padding: libc::c_int,
    pub seek_preroll: libc::c_int,
}
pub type AVPictureStructure = libc::c_uint;
pub const AV_PICTURE_STRUCTURE_FRAME: AVPictureStructure = 3;
pub const AV_PICTURE_STRUCTURE_BOTTOM_FIELD: AVPictureStructure = 2;
pub const AV_PICTURE_STRUCTURE_TOP_FIELD: AVPictureStructure = 1;
pub const AV_PICTURE_STRUCTURE_UNKNOWN: AVPictureStructure = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVCodecParserContext {
    pub priv_data: *mut libc::c_void,
    pub parser: *mut AVCodecParser,
    pub frame_offset: int64_t,
    pub cur_offset: int64_t,
    pub next_frame_offset: int64_t,
    pub pict_type: libc::c_int,
    pub repeat_pict: libc::c_int,
    pub pts: int64_t,
    pub dts: int64_t,
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub fetch_timestamp: libc::c_int,
    pub cur_frame_start_index: libc::c_int,
    pub cur_frame_offset: [int64_t; 4],
    pub cur_frame_pts: [int64_t; 4],
    pub cur_frame_dts: [int64_t; 4],
    pub flags: libc::c_int,
    pub offset: int64_t,
    pub cur_frame_end: [int64_t; 4],
    pub key_frame: libc::c_int,
    pub convergence_duration: int64_t,
    pub dts_sync_point: libc::c_int,
    pub dts_ref_dts_delta: libc::c_int,
    pub pts_dts_delta: libc::c_int,
    pub cur_frame_pos: [int64_t; 4],
    pub pos: int64_t,
    pub last_pos: int64_t,
    pub duration: libc::c_int,
    pub field_order: AVFieldOrder,
    pub picture_structure: AVPictureStructure,
    pub output_picture_number: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub format: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVCodecParser {
    pub codec_ids: [libc::c_int; 5],
    pub priv_data_size: libc::c_int,
    pub parser_init: Option<unsafe extern "C" fn(_: *mut AVCodecParserContext)
                                -> libc::c_int>,
    pub parser_parse: Option<unsafe extern "C" fn(_:
                                                      *mut AVCodecParserContext,
                                                  _: *mut AVCodecContext,
                                                  _: *mut *const uint8_t,
                                                  _: *mut libc::c_int,
                                                  _: *const uint8_t,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
    pub parser_close: Option<unsafe extern "C" fn(_:
                                                      *mut AVCodecParserContext)
                                 -> ()>,
    pub split: Option<unsafe extern "C" fn(_: *mut AVCodecContext,
                                           _: *const uint8_t, _: libc::c_int)
                          -> libc::c_int>,
    pub next: *mut AVCodecParser,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVIOInterruptCB {
    pub callback: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                             -> libc::c_int>,
    pub opaque: *mut libc::c_void,
}
pub type AVIODataMarkerType = libc::c_uint;
pub const AVIO_DATA_MARKER_FLUSH_POINT: AVIODataMarkerType = 5;
pub const AVIO_DATA_MARKER_TRAILER: AVIODataMarkerType = 4;
pub const AVIO_DATA_MARKER_UNKNOWN: AVIODataMarkerType = 3;
pub const AVIO_DATA_MARKER_BOUNDARY_POINT: AVIODataMarkerType = 2;
pub const AVIO_DATA_MARKER_SYNC_POINT: AVIODataMarkerType = 1;
pub const AVIO_DATA_MARKER_HEADER: AVIODataMarkerType = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVIOContext {
    pub av_class: *const AVClass,
    pub buffer: *mut libc::c_uchar,
    pub buffer_size: libc::c_int,
    pub buf_ptr: *mut libc::c_uchar,
    pub buf_end: *mut libc::c_uchar,
    pub opaque: *mut libc::c_void,
    pub read_packet: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: *mut uint8_t,
                                                 _: libc::c_int)
                                -> libc::c_int>,
    pub write_packet: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *mut uint8_t,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
    pub seek: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: int64_t,
                                          _: libc::c_int) -> int64_t>,
    pub pos: int64_t,
    pub eof_reached: libc::c_int,
    pub write_flag: libc::c_int,
    pub max_packet_size: libc::c_int,
    pub checksum: libc::c_ulong,
    pub checksum_ptr: *mut libc::c_uchar,
    pub update_checksum: Option<unsafe extern "C" fn(_: libc::c_ulong,
                                                     _: *const uint8_t,
                                                     _: libc::c_uint)
                                    -> libc::c_ulong>,
    pub error: libc::c_int,
    pub read_pause: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: libc::c_int)
                               -> libc::c_int>,
    pub read_seek: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: libc::c_int, _: int64_t,
                                               _: libc::c_int) -> int64_t>,
    pub seekable: libc::c_int,
    pub maxsize: int64_t,
    pub direct: libc::c_int,
    pub bytes_read: int64_t,
    pub seek_count: libc::c_int,
    pub writeout_count: libc::c_int,
    pub orig_buffer_size: libc::c_int,
    pub short_seek_threshold: libc::c_int,
    pub protocol_whitelist: *const libc::c_char,
    pub protocol_blacklist: *const libc::c_char,
    pub write_data_type: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: *mut uint8_t,
                                                     _: libc::c_int,
                                                     _: AVIODataMarkerType,
                                                     _: int64_t)
                                    -> libc::c_int>,
    pub ignore_boundary_point: libc::c_int,
    pub current_type: AVIODataMarkerType,
    pub last_time: int64_t,
    pub short_seek_get: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> libc::c_int>,
    pub written: int64_t,
    pub buf_ptr_max: *mut libc::c_uchar,
    pub min_packet_size: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVFormatContext {
    pub av_class: *const AVClass,
    pub iformat: *mut AVInputFormat,
    pub oformat: *mut AVOutputFormat,
    pub priv_data: *mut libc::c_void,
    pub pb: *mut AVIOContext,
    pub ctx_flags: libc::c_int,
    pub nb_streams: libc::c_uint,
    pub streams: *mut *mut AVStream,
    pub filename: [libc::c_char; 1024],
    pub url: *mut libc::c_char,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub bit_rate: int64_t,
    pub packet_size: libc::c_uint,
    pub max_delay: libc::c_int,
    pub flags: libc::c_int,
    pub probesize: int64_t,
    pub max_analyze_duration: int64_t,
    pub key: *const uint8_t,
    pub keylen: libc::c_int,
    pub nb_programs: libc::c_uint,
    pub programs: *mut *mut AVProgram,
    pub video_codec_id: AVCodecID,
    pub audio_codec_id: AVCodecID,
    pub subtitle_codec_id: AVCodecID,
    pub max_index_size: libc::c_uint,
    pub max_picture_buffer: libc::c_uint,
    pub nb_chapters: libc::c_uint,
    pub chapters: *mut *mut AVChapter,
    pub metadata: *mut AVDictionary,
    pub start_time_realtime: int64_t,
    pub fps_probe_size: libc::c_int,
    pub error_recognition: libc::c_int,
    pub interrupt_callback: AVIOInterruptCB,
    pub debug: libc::c_int,
    pub max_interleave_delta: int64_t,
    pub strict_std_compliance: libc::c_int,
    pub event_flags: libc::c_int,
    pub max_ts_probe: libc::c_int,
    pub avoid_negative_ts: libc::c_int,
    pub ts_id: libc::c_int,
    pub audio_preload: libc::c_int,
    pub max_chunk_duration: libc::c_int,
    pub max_chunk_size: libc::c_int,
    pub use_wallclock_as_timestamps: libc::c_int,
    pub avio_flags: libc::c_int,
    pub duration_estimation_method: AVDurationEstimationMethod,
    pub skip_initial_bytes: int64_t,
    pub correct_ts_overflow: libc::c_uint,
    pub seek2any: libc::c_int,
    pub flush_packets: libc::c_int,
    pub probe_score: libc::c_int,
    pub format_probesize: libc::c_int,
    pub codec_whitelist: *mut libc::c_char,
    pub format_whitelist: *mut libc::c_char,
    pub internal: *mut AVFormatInternal,
    pub io_repositioned: libc::c_int,
    pub video_codec: *mut AVCodec,
    pub audio_codec: *mut AVCodec,
    pub subtitle_codec: *mut AVCodec,
    pub data_codec: *mut AVCodec,
    pub metadata_header_padding: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub control_message_cb: av_format_control_message,
    pub output_ts_offset: int64_t,
    pub dump_separator: *mut uint8_t,
    pub data_codec_id: AVCodecID,
    pub open_cb: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                             _: *mut *mut AVIOContext,
                                             _: *const libc::c_char,
                                             _: libc::c_int,
                                             _: *const AVIOInterruptCB,
                                             _: *mut *mut AVDictionary)
                            -> libc::c_int>,
    pub protocol_whitelist: *mut libc::c_char,
    pub io_open: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                             _: *mut *mut AVIOContext,
                                             _: *const libc::c_char,
                                             _: libc::c_int,
                                             _: *mut *mut AVDictionary)
                            -> libc::c_int>,
    pub io_close: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                              _: *mut AVIOContext) -> ()>,
    pub protocol_blacklist: *mut libc::c_char,
    pub max_streams: libc::c_int,
    pub skip_estimate_duration_from_pts: libc::c_int,
}
pub type av_format_control_message
    =
    Option<unsafe extern "C" fn(_: *mut AVFormatContext, _: libc::c_int,
                                _: *mut libc::c_void, _: size_t)
               -> libc::c_int>;
pub type AVDurationEstimationMethod = libc::c_uint;
pub const AVFMT_DURATION_FROM_BITRATE: AVDurationEstimationMethod = 2;
pub const AVFMT_DURATION_FROM_STREAM: AVDurationEstimationMethod = 1;
pub const AVFMT_DURATION_FROM_PTS: AVDurationEstimationMethod = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVChapter {
    pub id: libc::c_int,
    pub time_base: AVRational,
    pub start: int64_t,
    pub end: int64_t,
    pub metadata: *mut AVDictionary,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVProgram {
    pub id: libc::c_int,
    pub flags: libc::c_int,
    pub discard: AVDiscard,
    pub stream_index: *mut libc::c_uint,
    pub nb_stream_indexes: libc::c_uint,
    pub metadata: *mut AVDictionary,
    pub program_num: libc::c_int,
    pub pmt_pid: libc::c_int,
    pub pcr_pid: libc::c_int,
    pub pmt_version: libc::c_int,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVStream {
    pub index: libc::c_int,
    pub id: libc::c_int,
    pub codec: *mut AVCodecContext,
    pub priv_data: *mut libc::c_void,
    pub time_base: AVRational,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub nb_frames: int64_t,
    pub disposition: libc::c_int,
    pub discard: AVDiscard,
    pub sample_aspect_ratio: AVRational,
    pub metadata: *mut AVDictionary,
    pub avg_frame_rate: AVRational,
    pub attached_pic: AVPacket,
    pub side_data: *mut AVPacketSideData,
    pub nb_side_data: libc::c_int,
    pub event_flags: libc::c_int,
    pub r_frame_rate: AVRational,
    pub recommended_encoder_configuration: *mut libc::c_char,
    pub codecpar: *mut AVCodecParameters,
    pub info: *mut C2RustUnnamed,
    pub pts_wrap_bits: libc::c_int,
    pub first_dts: int64_t,
    pub cur_dts: int64_t,
    pub last_IP_pts: int64_t,
    pub last_IP_duration: libc::c_int,
    pub probe_packets: libc::c_int,
    pub codec_info_nb_frames: libc::c_int,
    pub need_parsing: AVStreamParseType,
    pub parser: *mut AVCodecParserContext,
    pub last_in_packet_buffer: *mut AVPacketList,
    pub probe_data: AVProbeData,
    pub pts_buffer: [int64_t; 17],
    pub index_entries: *mut AVIndexEntry,
    pub nb_index_entries: libc::c_int,
    pub index_entries_allocated_size: libc::c_uint,
    pub stream_identifier: libc::c_int,
    pub program_num: libc::c_int,
    pub pmt_version: libc::c_int,
    pub pmt_stream_idx: libc::c_int,
    pub interleaver_chunk_size: int64_t,
    pub interleaver_chunk_duration: int64_t,
    pub request_probe: libc::c_int,
    pub skip_to_keyframe: libc::c_int,
    pub skip_samples: libc::c_int,
    pub start_skip_samples: int64_t,
    pub first_discard_sample: int64_t,
    pub last_discard_sample: int64_t,
    pub nb_decoded_frames: libc::c_int,
    pub mux_ts_offset: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: libc::c_int,
    pub update_initial_durations_done: libc::c_int,
    pub pts_reorder_error: [int64_t; 17],
    pub pts_reorder_error_count: [uint8_t; 17],
    pub last_dts_for_order_check: int64_t,
    pub dts_ordered: uint8_t,
    pub dts_misordered: uint8_t,
    pub inject_global_side_data: libc::c_int,
    pub display_aspect_ratio: AVRational,
    pub internal: *mut AVStreamInternal,
}
#[derive ( Copy, Clone, BitfieldStruct )]
#[repr(C)]
pub struct AVIndexEntry {
    pub pos: int64_t,
    pub timestamp: int64_t,
    #[bitfield(name = "flags", ty = "libc::c_int", bits = "0..=1")]
    #[bitfield(name = "size", ty = "libc::c_int", bits = "2..=31")]
    pub flags_size: [u8; 4],
    pub min_distance: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVProbeData {
    pub filename: *const libc::c_char,
    pub buf: *mut libc::c_uchar,
    pub buf_size: libc::c_int,
    pub mime_type: *const libc::c_char,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVPacketList {
    pub pkt: AVPacket,
    pub next: *mut AVPacketList,
}
pub type AVStreamParseType = libc::c_uint;
pub const AVSTREAM_PARSE_FULL_RAW: AVStreamParseType = 5;
pub const AVSTREAM_PARSE_FULL_ONCE: AVStreamParseType = 4;
pub const AVSTREAM_PARSE_TIMESTAMPS: AVStreamParseType = 3;
pub const AVSTREAM_PARSE_HEADERS: AVStreamParseType = 2;
pub const AVSTREAM_PARSE_FULL: AVStreamParseType = 1;
pub const AVSTREAM_PARSE_NONE: AVStreamParseType = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub last_dts: int64_t,
    pub duration_gcd: int64_t,
    pub duration_count: libc::c_int,
    pub rfps_duration_sum: int64_t,
    pub duration_error: *mut [[libc::c_double; 399]; 2],
    pub codec_info_duration: int64_t,
    pub codec_info_duration_fields: int64_t,
    pub frame_delay_evidence: libc::c_int,
    pub found_decoder: libc::c_int,
    pub last_duration: int64_t,
    pub fps_first_dts: int64_t,
    pub fps_first_dts_idx: libc::c_int,
    pub fps_last_dts: int64_t,
    pub fps_last_dts_idx: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVOutputFormat {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub mime_type: *const libc::c_char,
    pub extensions: *const libc::c_char,
    pub audio_codec: AVCodecID,
    pub video_codec: AVCodecID,
    pub subtitle_codec: AVCodecID,
    pub flags: libc::c_int,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub next: *mut AVOutputFormat,
    pub priv_data_size: libc::c_int,
    pub write_header: Option<unsafe extern "C" fn(_: *mut AVFormatContext)
                                 -> libc::c_int>,
    pub write_packet: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                                  _: *mut AVPacket)
                                 -> libc::c_int>,
    pub write_trailer: Option<unsafe extern "C" fn(_: *mut AVFormatContext)
                                  -> libc::c_int>,
    pub interleave_packet: Option<unsafe extern "C" fn(_:
                                                           *mut AVFormatContext,
                                                       _: *mut AVPacket,
                                                       _: *mut AVPacket,
                                                       _: libc::c_int)
                                      -> libc::c_int>,
    pub query_codec: Option<unsafe extern "C" fn(_: AVCodecID, _: libc::c_int)
                                -> libc::c_int>,
    pub get_output_timestamp: Option<unsafe extern "C" fn(_:
                                                              *mut AVFormatContext,
                                                          _: libc::c_int,
                                                          _: *mut int64_t,
                                                          _: *mut int64_t)
                                         -> ()>,
    pub control_message: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                                     _: libc::c_int,
                                                     _: *mut libc::c_void,
                                                     _: size_t)
                                    -> libc::c_int>,
    pub write_uncoded_frame: Option<unsafe extern "C" fn(_:
                                                             *mut AVFormatContext,
                                                         _: libc::c_int,
                                                         _: *mut *mut AVFrame,
                                                         _: libc::c_uint)
                                        -> libc::c_int>,
    pub get_device_list: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                                     _: *mut AVDeviceInfoList)
                                    -> libc::c_int>,
    pub create_device_capabilities: Option<unsafe extern "C" fn(_:
                                                                    *mut AVFormatContext,
                                                                _:
                                                                    *mut AVDeviceCapabilitiesQuery)
                                               -> libc::c_int>,
    pub free_device_capabilities: Option<unsafe extern "C" fn(_:
                                                                  *mut AVFormatContext,
                                                              _:
                                                                  *mut AVDeviceCapabilitiesQuery)
                                             -> libc::c_int>,
    pub data_codec: AVCodecID,
    pub init: Option<unsafe extern "C" fn(_: *mut AVFormatContext)
                         -> libc::c_int>,
    pub deinit: Option<unsafe extern "C" fn(_: *mut AVFormatContext) -> ()>,
    pub check_bitstream: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                                     _: *const AVPacket)
                                    -> libc::c_int>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVInputFormat {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub flags: libc::c_int,
    pub extensions: *const libc::c_char,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub mime_type: *const libc::c_char,
    pub next: *mut AVInputFormat,
    pub raw_codec_id: libc::c_int,
    pub priv_data_size: libc::c_int,
    pub read_probe: Option<unsafe extern "C" fn(_: *const AVProbeData)
                               -> libc::c_int>,
    pub read_header: Option<unsafe extern "C" fn(_: *mut AVFormatContext)
                                -> libc::c_int>,
    pub read_packet: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                                 _: *mut AVPacket)
                                -> libc::c_int>,
    pub read_close: Option<unsafe extern "C" fn(_: *mut AVFormatContext)
                               -> libc::c_int>,
    pub read_seek: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                               _: libc::c_int, _: int64_t,
                                               _: libc::c_int)
                              -> libc::c_int>,
    pub read_timestamp: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                                    _: libc::c_int,
                                                    _: *mut int64_t,
                                                    _: int64_t) -> int64_t>,
    pub read_play: Option<unsafe extern "C" fn(_: *mut AVFormatContext)
                              -> libc::c_int>,
    pub read_pause: Option<unsafe extern "C" fn(_: *mut AVFormatContext)
                               -> libc::c_int>,
    pub read_seek2: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                                _: libc::c_int, _: int64_t,
                                                _: int64_t, _: int64_t,
                                                _: libc::c_int)
                               -> libc::c_int>,
    pub get_device_list: Option<unsafe extern "C" fn(_: *mut AVFormatContext,
                                                     _: *mut AVDeviceInfoList)
                                    -> libc::c_int>,
    pub create_device_capabilities: Option<unsafe extern "C" fn(_:
                                                                    *mut AVFormatContext,
                                                                _:
                                                                    *mut AVDeviceCapabilitiesQuery)
                                               -> libc::c_int>,
    pub free_device_capabilities: Option<unsafe extern "C" fn(_:
                                                                  *mut AVFormatContext,
                                                              _:
                                                                  *mut AVDeviceCapabilitiesQuery)
                                             -> libc::c_int>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVDRMObjectDescriptor {
    pub fd: libc::c_int,
    pub size: size_t,
    pub format_modifier: uint64_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVDRMPlaneDescriptor {
    pub object_index: libc::c_int,
    pub offset: ptrdiff_t,
    pub pitch: ptrdiff_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVDRMLayerDescriptor {
    pub format: uint32_t,
    pub nb_planes: libc::c_int,
    pub planes: [AVDRMPlaneDescriptor; 4],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVDRMFrameDescriptor {
    pub nb_objects: libc::c_int,
    pub objects: [AVDRMObjectDescriptor; 4],
    pub nb_layers: libc::c_int,
    pub layers: [AVDRMLayerDescriptor; 4],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct AVDRMDeviceContext {
    pub fd: libc::c_int,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_0 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_1,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_2 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_3,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type __u32 = libc::c_uint;
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
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wayland_output {
    pub link: wl_list,
    pub id: uint32_t,
    pub output: *mut wl_output,
    pub make: *mut libc::c_char,
    pub model: *mut libc::c_char,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub framerate: AVRational,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct capture_context {
    pub class: *mut AVClass,
    pub display: *mut wl_display,
    pub registry: *mut wl_registry,
    pub export_manager: *mut zwlr_export_dmabuf_manager_v1,
    pub output_list: wl_list,
    pub target_output: *mut wl_output,
    pub with_cursor: bool,
    pub frame_callback: *mut zwlr_export_dmabuf_frame_v1,
    pub err: libc::c_int,
    pub quit: bool,
    pub vid_thread: pthread_t,
    pub current_frame: *mut AVFrame,
    pub avf: *mut AVFormatContext,
    pub avctx: *mut AVCodecContext,
    pub drm_device_ref: *mut AVBufferRef,
    pub drm_frames_ref: *mut AVBufferRef,
    pub mapped_device_ref: *mut AVBufferRef,
    pub mapped_frames_ref: *mut AVBufferRef,
    pub vid_frames: fifo_buffer,
    pub start_pts: int64_t,
    pub software_format: AVPixelFormat,
    pub hw_device_type: AVHWDeviceType,
    pub encoder_opts: *mut AVDictionary,
    pub is_software_encoder: libc::c_int,
    pub hardware_device: *mut libc::c_char,
    pub out_filename: *mut libc::c_char,
    pub encoder_name: *mut libc::c_char,
    pub out_bitrate: libc::c_float,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct fifo_buffer {
    pub queued_frames: *mut *mut AVFrame,
    pub num_queued_frames: libc::c_int,
    pub max_queued_frames: libc::c_int,
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub cond_lock: pthread_mutex_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_registry_listener {
    pub global: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *mut wl_registry, _: uint32_t,
                                            _: *const libc::c_char,
                                            _: uint32_t) -> ()>,
    pub global_remove: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                   _: *mut wl_registry,
                                                   _: uint32_t) -> ()>,
}
pub type wl_output_mode = libc::c_uint;
pub const WL_OUTPUT_MODE_PREFERRED: wl_output_mode = 2;
pub const WL_OUTPUT_MODE_CURRENT: wl_output_mode = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct wl_output_listener {
    pub geometry: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: *mut wl_output, _: int32_t,
                                              _: int32_t, _: int32_t,
                                              _: int32_t, _: int32_t,
                                              _: *const libc::c_char,
                                              _: *const libc::c_char,
                                              _: int32_t) -> ()>,
    pub mode: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut wl_output, _: uint32_t,
                                          _: int32_t, _: int32_t, _: int32_t)
                         -> ()>,
    pub done: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut wl_output) -> ()>,
    pub scale: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut wl_output, _: int32_t)
                          -> ()>,
}
/* ZWLR_EXPORT_DMABUF_FRAME_V1_FLAGS_ENUM */
/* *
 * @ingroup iface_zwlr_export_dmabuf_frame_v1
 * cancel reason
 *
 * Indicates reason for cancelling the frame.
 */
pub type zwlr_export_dmabuf_frame_v1_cancel_reason = libc::c_uint;
/* *
	 * temporary error, source will produce more frames
	 */
pub const ZWLR_EXPORT_DMABUF_FRAME_V1_CANCEL_REASON_RESIZING:
          zwlr_export_dmabuf_frame_v1_cancel_reason =
    2;
/* *
	 * fatal error, source will not produce frames
	 */
pub const ZWLR_EXPORT_DMABUF_FRAME_V1_CANCEL_REASON_PERMANENT:
          zwlr_export_dmabuf_frame_v1_cancel_reason =
    1;
/* *
	 * temporary error, source will produce more frames
	 */
pub const ZWLR_EXPORT_DMABUF_FRAME_V1_CANCEL_REASON_TEMPORARY:
          zwlr_export_dmabuf_frame_v1_cancel_reason =
    0;
/* ZWLR_EXPORT_DMABUF_FRAME_V1_CANCEL_REASON_ENUM */
/* *
 * @ingroup iface_zwlr_export_dmabuf_frame_v1
 * @struct zwlr_export_dmabuf_frame_v1_listener
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct zwlr_export_dmabuf_frame_v1_listener {
    pub frame: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _:
                                               *mut zwlr_export_dmabuf_frame_v1,
                                           _: uint32_t, _: uint32_t,
                                           _: uint32_t, _: uint32_t,
                                           _: uint32_t, _: uint32_t,
                                           _: uint32_t, _: uint32_t,
                                           _: uint32_t, _: uint32_t) -> ()>,
    pub object: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _:
                                                *mut zwlr_export_dmabuf_frame_v1,
                                            _: uint32_t, _: int32_t,
                                            _: uint32_t, _: uint32_t,
                                            _: uint32_t, _: uint32_t) -> ()>,
    pub ready: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _:
                                               *mut zwlr_export_dmabuf_frame_v1,
                                           _: uint32_t, _: uint32_t,
                                           _: uint32_t) -> ()>,
    pub cancel: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _:
                                                *mut zwlr_export_dmabuf_frame_v1,
                                            _: uint32_t) -> ()>,
}
#[inline]
unsafe extern "C" fn av_make_error_string(mut errbuf: *mut libc::c_char,
                                          mut errbuf_size: size_t,
                                          mut errnum: libc::c_int)
 -> *mut libc::c_char {
    av_strerror(errnum, errbuf, errbuf_size);
    return errbuf;
}
#[inline]
unsafe extern "C" fn wl_display_get_registry(mut wl_display: *mut wl_display)
 -> *mut wl_registry {
    let mut registry: *mut wl_proxy = 0 as *mut wl_proxy;
    registry =
        wl_proxy_marshal_constructor(wl_display as *mut wl_proxy,
                                     1i32 as uint32_t,
                                     &wl_registry_interface as
                                         *const wl_interface,
                                     0 as *mut libc::c_void);
    return registry as *mut wl_registry;
}
#[inline]
unsafe extern "C" fn wl_registry_add_listener(mut wl_registry:
                                                  *mut wl_registry,
                                              mut listener:
                                                  *const wl_registry_listener,
                                              mut data: *mut libc::c_void)
 -> libc::c_int {
    return wl_proxy_add_listener(wl_registry as *mut wl_proxy,
                                 listener as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, data);
}
#[inline]
unsafe extern "C" fn wl_registry_bind(mut wl_registry: *mut wl_registry,
                                      mut name: uint32_t,
                                      mut interface: *const wl_interface,
                                      mut version: uint32_t)
 -> *mut libc::c_void {
    let mut id: *mut wl_proxy = 0 as *mut wl_proxy;
    id =
        wl_proxy_marshal_constructor_versioned(wl_registry as *mut wl_proxy,
                                               0i32 as uint32_t, interface,
                                               version, name,
                                               (*interface).name, version,
                                               0 as *mut libc::c_void);
    return id as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn wl_output_add_listener(mut wl_output: *mut wl_output,
                                            mut listener:
                                                *const wl_output_listener,
                                            mut data: *mut libc::c_void)
 -> libc::c_int {
    return wl_proxy_add_listener(wl_output as *mut wl_proxy,
                                 listener as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, data);
}
/* Generated by wayland-scanner 1.17.0 */
/* *
 * @page page_wlr_export_dmabuf_unstable_v1 The wlr_export_dmabuf_unstable_v1 protocol
 * a protocol for low overhead screen content capturing
 *
 * @section page_desc_wlr_export_dmabuf_unstable_v1 Description
 *
 * An interface to capture surfaces in an efficient way by exporting DMA-BUFs.
 *
 * Warning! The protocol described in this file is experimental and
 * backward incompatible changes may be made. Backward compatible changes
 * may be added together with the corresponding interface version bump.
 * Backward incompatible changes are done by bumping the version number in
 * the protocol and interface names and resetting the interface version.
 * Once the protocol is to be declared stable, the 'z' prefix and the
 * version number in the protocol and interface names are removed and the
 * interface version number is reset.
 *
 * @section page_ifaces_wlr_export_dmabuf_unstable_v1 Interfaces
 * - @subpage page_iface_zwlr_export_dmabuf_manager_v1 - manager to inform clients and begin capturing
 * - @subpage page_iface_zwlr_export_dmabuf_frame_v1 - a DMA-BUF frame
 * @section page_copyright_wlr_export_dmabuf_unstable_v1 Copyright
 * <pre>
 *
 * Copyright © 2018 Rostislav Pehlivanov
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
 * </pre>
 */
/* *
 * @page page_iface_zwlr_export_dmabuf_manager_v1 zwlr_export_dmabuf_manager_v1
 * @section page_iface_zwlr_export_dmabuf_manager_v1_desc Description
 *
 * This object is a manager with which to start capturing from sources.
 * @section page_iface_zwlr_export_dmabuf_manager_v1_api API
 * See @ref iface_zwlr_export_dmabuf_manager_v1.
 */
/* *
 * @defgroup iface_zwlr_export_dmabuf_manager_v1 The zwlr_export_dmabuf_manager_v1 interface
 *
 * This object is a manager with which to start capturing from sources.
 */
/* *
 * @page page_iface_zwlr_export_dmabuf_frame_v1 zwlr_export_dmabuf_frame_v1
 * @section page_iface_zwlr_export_dmabuf_frame_v1_desc Description
 *
 * This object represents a single DMA-BUF frame.
 *
 * If the capture is successful, the compositor will first send a "frame"
 * event, followed by one or several "object". When the frame is available
 * for readout, the "ready" event is sent.
 *
 * If the capture failed, the "cancel" event is sent. This can happen anytime
 * before the "ready" event.
 *
 * Once either a "ready" or a "cancel" event is received, the client should
 * destroy the frame. Once an "object" event is received, the client is
 * responsible for closing the associated file descriptor.
 *
 * All frames are read-only and may not be written into or altered.
 * @section page_iface_zwlr_export_dmabuf_frame_v1_api API
 * See @ref iface_zwlr_export_dmabuf_frame_v1.
 */
/* *
 * @defgroup iface_zwlr_export_dmabuf_frame_v1 The zwlr_export_dmabuf_frame_v1 interface
 *
 * This object represents a single DMA-BUF frame.
 *
 * If the capture is successful, the compositor will first send a "frame"
 * event, followed by one or several "object". When the frame is available
 * for readout, the "ready" event is sent.
 *
 * If the capture failed, the "cancel" event is sent. This can happen anytime
 * before the "ready" event.
 *
 * Once either a "ready" or a "cancel" event is received, the client should
 * destroy the frame. Once an "object" event is received, the client is
 * responsible for closing the associated file descriptor.
 *
 * All frames are read-only and may not be written into or altered.
 */
/* *
 * @ingroup iface_zwlr_export_dmabuf_manager_v1
 */
/* *
 * @ingroup iface_zwlr_export_dmabuf_manager_v1
 */
/* * @ingroup iface_zwlr_export_dmabuf_manager_v1 */
/* * @ingroup iface_zwlr_export_dmabuf_manager_v1 */
/* *
 * @ingroup iface_zwlr_export_dmabuf_manager_v1
 *
 * Capture the next frame of a an entire output.
 */
#[inline]
unsafe extern "C" fn zwlr_export_dmabuf_manager_v1_capture_output(mut zwlr_export_dmabuf_manager_v1:
                                                                      *mut zwlr_export_dmabuf_manager_v1,
                                                                  mut overlay_cursor:
                                                                      int32_t,
                                                                  mut output:
                                                                      *mut wl_output)
 -> *mut zwlr_export_dmabuf_frame_v1 {
    let mut frame: *mut wl_proxy = 0 as *mut wl_proxy;
    frame =
        wl_proxy_marshal_constructor(zwlr_export_dmabuf_manager_v1 as
                                         *mut wl_proxy, 0i32 as uint32_t,
                                     &zwlr_export_dmabuf_frame_v1_interface as
                                         *const wl_interface,
                                     0 as *mut libc::c_void, overlay_cursor,
                                     output);
    return frame as *mut zwlr_export_dmabuf_frame_v1;
}
/* *
 * @ingroup iface_zwlr_export_dmabuf_manager_v1
 *
 * All objects created by the manager will still remain valid, until their
 * appropriate destroy request has been called.
 */
#[inline]
unsafe extern "C" fn zwlr_export_dmabuf_manager_v1_destroy(mut zwlr_export_dmabuf_manager_v1:
                                                               *mut zwlr_export_dmabuf_manager_v1) {
    wl_proxy_marshal(zwlr_export_dmabuf_manager_v1 as *mut wl_proxy,
                     1i32 as uint32_t);
    wl_proxy_destroy(zwlr_export_dmabuf_manager_v1 as *mut wl_proxy);
}
/* *
 * @ingroup iface_zwlr_export_dmabuf_frame_v1
 */
#[inline]
unsafe extern "C" fn zwlr_export_dmabuf_frame_v1_add_listener(mut zwlr_export_dmabuf_frame_v1:
                                                                  *mut zwlr_export_dmabuf_frame_v1,
                                                              mut listener:
                                                                  *const zwlr_export_dmabuf_frame_v1_listener,
                                                              mut data:
                                                                  *mut libc::c_void)
 -> libc::c_int {
    return wl_proxy_add_listener(zwlr_export_dmabuf_frame_v1 as *mut wl_proxy,
                                 listener as
                                     *mut Option<unsafe extern "C" fn()
                                                     -> ()>, data);
}
/* *
 * @ingroup iface_zwlr_export_dmabuf_frame_v1
 *
 * Unreferences the frame. This request must be called as soon as its no
 * longer used.
 *
 * It can be called at any time by the client. The client will still have
 * to close any FDs it has been given.
 */
#[inline]
unsafe extern "C" fn zwlr_export_dmabuf_frame_v1_destroy(mut zwlr_export_dmabuf_frame_v1:
                                                             *mut zwlr_export_dmabuf_frame_v1) {
    wl_proxy_marshal(zwlr_export_dmabuf_frame_v1 as *mut wl_proxy,
                     0i32 as uint32_t);
    wl_proxy_destroy(zwlr_export_dmabuf_frame_v1 as *mut wl_proxy);
}
unsafe extern "C" fn init_fifo(mut buf: *mut fifo_buffer,
                               mut max_queued_frames: libc::c_int)
 -> libc::c_int {
    pthread_mutex_init(&mut (*buf).lock, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*buf).cond, 0 as *const pthread_condattr_t);
    pthread_mutex_init(&mut (*buf).cond_lock,
                       0 as *const pthread_mutexattr_t);
    (*buf).num_queued_frames = 0i32;
    (*buf).max_queued_frames = max_queued_frames;
    (*buf).queued_frames =
        av_mallocz(((*buf).max_queued_frames as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<AVFrame>()
                                                        as libc::c_ulong)) as
            *mut *mut AVFrame;
    return if (*buf).queued_frames.is_null() { -12i32 } else { 0i32 };
}
unsafe extern "C" fn get_fifo_size(mut buf: *mut fifo_buffer) -> libc::c_int {
    pthread_mutex_lock(&mut (*buf).lock);
    let mut ret: libc::c_int = (*buf).num_queued_frames;
    pthread_mutex_unlock(&mut (*buf).lock);
    return ret;
}
unsafe extern "C" fn push_to_fifo(mut buf: *mut fifo_buffer,
                                  mut f: *mut AVFrame) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    pthread_mutex_lock(&mut (*buf).lock);
    if (*buf).num_queued_frames + 1i32 > (*buf).max_queued_frames {
        av_frame_free(&mut f);
        ret = 1i32
    } else {
        let fresh0 = (*buf).num_queued_frames;
        (*buf).num_queued_frames = (*buf).num_queued_frames + 1;
        let ref mut fresh1 = *(*buf).queued_frames.offset(fresh0 as isize);
        *fresh1 = f;
        ret = 0i32
    }
    pthread_mutex_unlock(&mut (*buf).lock);
    pthread_cond_signal(&mut (*buf).cond);
    return ret;
}
unsafe extern "C" fn pop_from_fifo(mut buf: *mut fifo_buffer)
 -> *mut AVFrame {
    pthread_mutex_lock(&mut (*buf).lock);
    if (*buf).num_queued_frames == 0 {
        pthread_mutex_unlock(&mut (*buf).lock);
        pthread_cond_wait(&mut (*buf).cond, &mut (*buf).cond_lock);
        pthread_mutex_lock(&mut (*buf).lock);
    }
    let mut rf: *mut AVFrame = *(*buf).queued_frames.offset(0);
    let mut i: libc::c_int = 1i32;
    while i < (*buf).num_queued_frames {
        let ref mut fresh2 =
            *(*buf).queued_frames.offset((i - 1i32) as isize);
        *fresh2 = *(*buf).queued_frames.offset(i as isize);
        i += 1
    }
    (*buf).num_queued_frames -= 1;
    let ref mut fresh3 =
        *(*buf).queued_frames.offset((*buf).num_queued_frames as isize);
    *fresh3 = 0 as *mut AVFrame;
    pthread_mutex_unlock(&mut (*buf).lock);
    return rf;
}
unsafe extern "C" fn free_fifo(mut buf: *mut fifo_buffer) {
    pthread_mutex_lock(&mut (*buf).lock);
    if (*buf).num_queued_frames != 0 {
        let mut i: libc::c_int = 0i32;
        while i < (*buf).num_queued_frames {
            av_frame_free(&mut *(*buf).queued_frames.offset(i as isize));
            i += 1
        }
    }
    av_freep(&mut (*buf).queued_frames as *mut *mut *mut AVFrame as
                 *mut libc::c_void);
    pthread_mutex_unlock(&mut (*buf).lock);
}
unsafe extern "C" fn output_handle_geometry(mut data: *mut libc::c_void,
                                            mut wl_output: *mut wl_output,
                                            mut x: int32_t, mut y: int32_t,
                                            mut phys_width: int32_t,
                                            mut phys_height: int32_t,
                                            mut subpixel: int32_t,
                                            mut make: *const libc::c_char,
                                            mut model: *const libc::c_char,
                                            mut transform: int32_t) {
    let mut output: *mut wayland_output = data as *mut wayland_output;
    (*output).make = av_strdup(make);
    (*output).model = av_strdup(model);
}
unsafe extern "C" fn output_handle_mode(mut data: *mut libc::c_void,
                                        mut wl_output: *mut wl_output,
                                        mut flags: uint32_t,
                                        mut width: int32_t,
                                        mut height: int32_t,
                                        mut refresh: int32_t) {
    if flags & WL_OUTPUT_MODE_CURRENT as libc::c_int as libc::c_uint != 0 {
        let mut output: *mut wayland_output = data as *mut wayland_output;
        (*output).width = width;
        (*output).height = height;
        (*output).framerate =
            { let mut init = AVRational{num: refresh, den: 1000i32,}; init }
    };
}
unsafe extern "C" fn output_handle_done(mut data: *mut libc::c_void,
                                        mut wl_output: *mut wl_output) {
    /* Nothing to do */
}
unsafe extern "C" fn output_handle_scale(mut data: *mut libc::c_void,
                                         mut wl_output: *mut wl_output,
                                         mut factor: int32_t) {
    /* Nothing to do */
}
static mut output_listener: wl_output_listener =
    unsafe {
        {
            let mut init =
                wl_output_listener{geometry:
                                       Some(output_handle_geometry as
                                                unsafe extern "C" fn(_:
                                                                         *mut libc::c_void,
                                                                     _:
                                                                         *mut wl_output,
                                                                     _:
                                                                         int32_t,
                                                                     _:
                                                                         int32_t,
                                                                     _:
                                                                         int32_t,
                                                                     _:
                                                                         int32_t,
                                                                     _:
                                                                         int32_t,
                                                                     _:
                                                                         *const libc::c_char,
                                                                     _:
                                                                         *const libc::c_char,
                                                                     _:
                                                                         int32_t)
                                                    -> ()),
                                   mode:
                                       Some(output_handle_mode as
                                                unsafe extern "C" fn(_:
                                                                         *mut libc::c_void,
                                                                     _:
                                                                         *mut wl_output,
                                                                     _:
                                                                         uint32_t,
                                                                     _:
                                                                         int32_t,
                                                                     _:
                                                                         int32_t,
                                                                     _:
                                                                         int32_t)
                                                    -> ()),
                                   done:
                                       Some(output_handle_done as
                                                unsafe extern "C" fn(_:
                                                                         *mut libc::c_void,
                                                                     _:
                                                                         *mut wl_output)
                                                    -> ()),
                                   scale:
                                       Some(output_handle_scale as
                                                unsafe extern "C" fn(_:
                                                                         *mut libc::c_void,
                                                                     _:
                                                                         *mut wl_output,
                                                                     _:
                                                                         int32_t)
                                                    -> ()),};
            init
        }
    };
unsafe extern "C" fn registry_handle_add(mut data: *mut libc::c_void,
                                         mut reg: *mut wl_registry,
                                         mut id: uint32_t,
                                         mut interface: *const libc::c_char,
                                         mut ver: uint32_t) {
    let mut ctx: *mut capture_context = data as *mut capture_context;
    if strcmp(interface, wl_output_interface.name) == 0 {
        let mut output: *mut wayland_output =
            av_mallocz(::std::mem::size_of::<wayland_output>() as
                           libc::c_ulong) as *mut wayland_output;
        (*output).id = id;
        (*output).output =
            wl_registry_bind(reg, id, &wl_output_interface, 1i32 as uint32_t)
                as *mut wl_output;
        wl_output_add_listener((*output).output, &output_listener,
                               output as *mut libc::c_void);
        wl_list_insert(&mut (*ctx).output_list, &mut (*output).link);
    }
    if strcmp(interface, zwlr_export_dmabuf_manager_v1_interface.name) == 0 {
        (*ctx).export_manager =
            wl_registry_bind(reg, id,
                             &zwlr_export_dmabuf_manager_v1_interface,
                             1i32 as uint32_t) as
                *mut zwlr_export_dmabuf_manager_v1
    };
}
unsafe extern "C" fn remove_output(mut out: *mut wayland_output) {
    wl_list_remove(&mut (*out).link);
    av_free((*out).make as *mut libc::c_void);
    av_free((*out).model as *mut libc::c_void);
    av_free(out as *mut libc::c_void);
}
unsafe extern "C" fn find_output(mut ctx: *mut capture_context,
                                 mut out: *mut wl_output, mut id: uint32_t)
 -> *mut wayland_output {
    let mut output: *mut wayland_output = 0 as *mut wayland_output;
    let mut tmp: *mut wayland_output = 0 as *mut wayland_output;
    output =
        ((*ctx).output_list.next as *mut libc::c_char).offset(-0) as
            *mut wayland_output;
    tmp =
        ((*output).link.next as *mut libc::c_char).offset(-0) as
            *mut wayland_output;
    while &mut (*output).link as *mut wl_list !=
              &mut (*ctx).output_list as *mut wl_list {
        if (*output).output == out || (*output).id == id { return output }
        output = tmp;
        tmp =
            ((*output).link.next as *mut libc::c_char).offset(-0) as
                *mut wayland_output
    }
    return 0 as *mut wayland_output;
}
unsafe extern "C" fn registry_handle_remove(mut data: *mut libc::c_void,
                                            mut reg: *mut wl_registry,
                                            mut id: uint32_t) {
    remove_output(find_output(data as *mut capture_context,
                              0 as *mut wl_output, id));
}
static mut registry_listener: wl_registry_listener =
    unsafe {
        {
            let mut init =
                wl_registry_listener{global:
                                         Some(registry_handle_add as
                                                  unsafe extern "C" fn(_:
                                                                           *mut libc::c_void,
                                                                       _:
                                                                           *mut wl_registry,
                                                                       _:
                                                                           uint32_t,
                                                                       _:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           uint32_t)
                                                      -> ()),
                                     global_remove:
                                         Some(registry_handle_remove as
                                                  unsafe extern "C" fn(_:
                                                                           *mut libc::c_void,
                                                                       _:
                                                                           *mut wl_registry,
                                                                       _:
                                                                           uint32_t)
                                                      -> ()),};
            init
        }
    };
unsafe extern "C" fn frame_free(mut opaque: *mut libc::c_void,
                                mut data: *mut uint8_t) {
    let mut desc: *mut AVDRMFrameDescriptor =
        data as *mut AVDRMFrameDescriptor;
    if !desc.is_null() {
        let mut i: libc::c_int = 0i32;
        while i < (*desc).nb_objects {
            close((*desc).objects[i as usize].fd);
            i += 1
        }
        av_free(data as *mut libc::c_void);
    }
    zwlr_export_dmabuf_frame_v1_destroy(opaque as
                                            *mut zwlr_export_dmabuf_frame_v1);
}
unsafe extern "C" fn frame_start(mut data: *mut libc::c_void,
                                 mut frame: *mut zwlr_export_dmabuf_frame_v1,
                                 mut width: uint32_t, mut height: uint32_t,
                                 mut offset_x: uint32_t,
                                 mut offset_y: uint32_t,
                                 mut buffer_flags: uint32_t,
                                 mut flags: uint32_t, mut format: uint32_t,
                                 mut mod_high: uint32_t,
                                 mut mod_low: uint32_t,
                                 mut num_objects: uint32_t) {
    let mut f: *mut AVFrame = 0 as *mut AVFrame;
    let mut ctx: *mut capture_context = data as *mut capture_context;
    let mut err: libc::c_int = 0i32;
    /* Allocate DRM specific struct */
    let mut desc: *mut AVDRMFrameDescriptor =
        av_mallocz(::std::mem::size_of::<AVDRMFrameDescriptor>() as
                       libc::c_ulong) as *mut AVDRMFrameDescriptor;
    if desc.is_null() {
        err = -12i32
    } else {
        (*desc).nb_objects = num_objects as libc::c_int;
        (*desc).objects[0].format_modifier =
            (mod_high as uint64_t) << 32i32 | mod_low as libc::c_ulong;
        (*desc).nb_layers = 1i32;
        (*desc).layers[0].format = format;
        /* Allocate a frame */
        f = av_frame_alloc();
        if f.is_null() {
            err = -12i32
        } else {
            /* Set base frame properties */
            (*ctx).current_frame = f;
            (*f).width = width as libc::c_int;
            (*f).height = height as libc::c_int;
            (*f).format = AV_PIX_FMT_DRM_PRIME as libc::c_int;
            /* Set the frame data to the DRM specific struct */
            (*f).buf[0] =
                av_buffer_create(desc as *mut uint8_t,
                                 ::std::mem::size_of::<AVDRMFrameDescriptor>()
                                     as libc::c_ulong as libc::c_int,
                                 Some(frame_free as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void,
                                                               _:
                                                                   *mut uint8_t)
                                              -> ()),
                                 frame as *mut libc::c_void, 0i32);
            if (*f).buf[0].is_null() {
                err = -12i32
            } else { (*f).data[0] = desc as *mut uint8_t; return }
        }
    }
    (*ctx).err = err;
    frame_free(frame as *mut libc::c_void, desc as *mut uint8_t);
}
unsafe extern "C" fn frame_object(mut data: *mut libc::c_void,
                                  mut frame: *mut zwlr_export_dmabuf_frame_v1,
                                  mut index: uint32_t, mut fd: int32_t,
                                  mut size: uint32_t, mut offset: uint32_t,
                                  mut stride: uint32_t,
                                  mut plane_index: uint32_t) {
    let mut ctx: *mut capture_context = data as *mut capture_context;
    let mut f: *mut AVFrame = (*ctx).current_frame;
    let mut desc: *mut AVDRMFrameDescriptor =
        (*f).data[0] as *mut AVDRMFrameDescriptor;
    (*desc).objects[index as usize].fd = fd;
    (*desc).objects[index as usize].size = size as size_t;
    (*desc).layers[0].planes[plane_index as usize].object_index =
        index as libc::c_int;
    (*desc).layers[0].planes[plane_index as usize].offset =
        offset as ptrdiff_t;
    (*desc).layers[0].planes[plane_index as usize].pitch =
        stride as ptrdiff_t;
}
unsafe extern "C" fn drm_fmt_to_pixfmt(mut fmt: uint32_t) -> AVPixelFormat {
    match fmt {
        842094158 => { return AV_PIX_FMT_NV12 }
        875713089 => { return AV_PIX_FMT_BGRA }
        875713112 => { return AV_PIX_FMT_BGR0 }
        875708993 => { return AV_PIX_FMT_RGBA }
        875709016 => { return AV_PIX_FMT_RGB0 }
        875708754 => { return AV_PIX_FMT_ABGR }
        875714642 => { return AV_PIX_FMT_0BGR }
        875708738 => { return AV_PIX_FMT_ARGB }
        875714626 => { return AV_PIX_FMT_0RGB }
        _ => { return AV_PIX_FMT_NONE }
    };
}
unsafe extern "C" fn attach_drm_frames_ref(mut ctx: *mut capture_context,
                                           mut f: *mut AVFrame,
                                           mut sw_format: AVPixelFormat)
 -> libc::c_int {
    let mut current_block: u64;
    let mut err: libc::c_int = 0i32;
    let mut hwfc: *mut AVHWFramesContext = 0 as *mut AVHWFramesContext;
    if !(*ctx).drm_frames_ref.is_null() {
        hwfc = (*(*ctx).drm_frames_ref).data as *mut AVHWFramesContext;
        if (*hwfc).width == (*f).width && (*hwfc).height == (*f).height &&
               (*hwfc).sw_format as libc::c_int == sw_format as libc::c_int {
            current_block = 15788532170911084015;
        } else {
            av_buffer_unref(&mut (*ctx).drm_frames_ref);
            current_block = 11875828834189669668;
        }
    } else { current_block = 11875828834189669668; }
    match current_block {
        11875828834189669668 => {
            (*ctx).drm_frames_ref =
                av_hwframe_ctx_alloc((*ctx).drm_device_ref);
            if (*ctx).drm_frames_ref.is_null() {
                err = -12i32;
                current_block = 3896031513207648507;
            } else {
                hwfc =
                    (*(*ctx).drm_frames_ref).data as *mut AVHWFramesContext;
                (*hwfc).format = (*f).format as AVPixelFormat;
                (*hwfc).sw_format = sw_format;
                (*hwfc).width = (*f).width;
                (*hwfc).height = (*f).height;
                err = av_hwframe_ctx_init((*ctx).drm_frames_ref);
                if err != 0 {
                    av_log(ctx as *mut libc::c_void, 16i32,
                           b"AVHWFramesContext init failed: %s!\n\x00" as
                               *const u8 as *const libc::c_char,
                           av_make_error_string([0i32 as libc::c_char, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0].as_mut_ptr(),
                                                64i32 as size_t, err));
                    current_block = 3896031513207648507;
                } else { current_block = 15788532170911084015; }
            }
        }
        _ => { }
    }
    match current_block {
        15788532170911084015 => {
            /* Set frame hardware context referencce */
            (*f).hw_frames_ctx = av_buffer_ref((*ctx).drm_frames_ref);
            if (*f).hw_frames_ctx.is_null() {
                err = -12i32
            } else { return 0i32 }
        }
        _ => { }
    }
    av_buffer_unref(&mut (*ctx).drm_frames_ref);
    return err;
}
unsafe extern "C" fn frame_ready(mut data: *mut libc::c_void,
                                 mut frame: *mut zwlr_export_dmabuf_frame_v1,
                                 mut tv_sec_hi: uint32_t,
                                 mut tv_sec_lo: uint32_t,
                                 mut tv_nsec: uint32_t) {
    let mut mapped_frame: *mut AVFrame = 0 as *mut AVFrame;
    let mut mapped_hwfc: *mut AVHWFramesContext = 0 as *mut AVHWFramesContext;
    let mut ctx: *mut capture_context = data as *mut capture_context;
    let mut f: *mut AVFrame = (*ctx).current_frame;
    let mut desc: *mut AVDRMFrameDescriptor =
        (*f).data[0] as *mut AVDRMFrameDescriptor;
    let mut pix_fmt: AVPixelFormat =
        drm_fmt_to_pixfmt((*desc).layers[0].format);
    let mut err: libc::c_int = 0i32;
    /* Timestamp, nanoseconds timebase */
    (*f).pts =
        ((tv_sec_hi as uint64_t) << 32i32 |
             tv_sec_lo as
                 libc::c_ulong).wrapping_mul(1000000000i32 as
                                                 libc::c_ulong).wrapping_add(tv_nsec
                                                                                 as
                                                                                 libc::c_ulong)
            as int64_t;
    if (*ctx).start_pts == 0 { (*ctx).start_pts = (*f).pts }
    (*f).pts =
        av_rescale_q((*f).pts - (*ctx).start_pts,
                     {
                         let mut init =
                             AVRational{num: 1i32, den: 1000000000i32,};
                         init
                     }, (*(*ctx).avctx).time_base);
    /* Attach the hardware frame context to the frame */
    err = attach_drm_frames_ref(ctx, f, pix_fmt);
    if !(err != 0) {
        /* TODO: support multiplane stuff */
        (*desc).layers[0].nb_planes = av_pix_fmt_count_planes(pix_fmt);
        mapped_frame = av_frame_alloc();
        if mapped_frame.is_null() {
            err = -12i32
        } else {
            mapped_hwfc = 0 as *mut AVHWFramesContext;
            mapped_hwfc =
                (*(*ctx).mapped_frames_ref).data as *mut AVHWFramesContext;
            (*mapped_frame).format = (*mapped_hwfc).format as libc::c_int;
            (*mapped_frame).pts = (*f).pts;
            /* Set frame hardware context referencce */
            (*mapped_frame).hw_frames_ctx =
                av_buffer_ref((*ctx).mapped_frames_ref);
            if (*mapped_frame).hw_frames_ctx.is_null() {
                err = -12i32
            } else {
                err = av_hwframe_map(mapped_frame, f, 0i32);
                if err != 0 {
                    av_log(ctx as *mut libc::c_void, 16i32,
                           b"Error mapping: %s!\n\x00" as *const u8 as
                               *const libc::c_char,
                           av_make_error_string([0i32 as libc::c_char, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0].as_mut_ptr(),
                                                64i32 as size_t, err));
                } else {
                    if push_to_fifo(&mut (*ctx).vid_frames, mapped_frame) != 0
                       {
                        av_log(ctx as *mut libc::c_void, 24i32,
                               b"Dropped frame!\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    if !(*ctx).quit && (*ctx).err == 0 { register_cb(ctx); }
                }
            }
        }
    }
    (*ctx).err = err;
    av_frame_free(&mut (*ctx).current_frame);
}
unsafe extern "C" fn frame_cancel(mut data: *mut libc::c_void,
                                  mut frame: *mut zwlr_export_dmabuf_frame_v1,
                                  mut reason: uint32_t) {
    let mut ctx: *mut capture_context = data as *mut capture_context;
    av_log(ctx as *mut libc::c_void, 24i32,
           b"Frame cancelled!\n\x00" as *const u8 as *const libc::c_char);
    av_frame_free(&mut (*ctx).current_frame);
    if reason ==
           ZWLR_EXPORT_DMABUF_FRAME_V1_CANCEL_REASON_PERMANENT as libc::c_int
               as libc::c_uint {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Permanent failure, exiting\n\x00" as *const u8 as
                   *const libc::c_char);
        (*ctx).err = 1i32
    } else { register_cb(ctx); };
}
static mut frame_listener: zwlr_export_dmabuf_frame_v1_listener =
    unsafe {
        {
            let mut init =
                zwlr_export_dmabuf_frame_v1_listener{frame:
                                                         Some(frame_start as
                                                                  unsafe extern "C" fn(_:
                                                                                           *mut libc::c_void,
                                                                                       _:
                                                                                           *mut zwlr_export_dmabuf_frame_v1,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t)
                                                                      -> ()),
                                                     object:
                                                         Some(frame_object as
                                                                  unsafe extern "C" fn(_:
                                                                                           *mut libc::c_void,
                                                                                       _:
                                                                                           *mut zwlr_export_dmabuf_frame_v1,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           int32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t)
                                                                      -> ()),
                                                     ready:
                                                         Some(frame_ready as
                                                                  unsafe extern "C" fn(_:
                                                                                           *mut libc::c_void,
                                                                                       _:
                                                                                           *mut zwlr_export_dmabuf_frame_v1,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t,
                                                                                       _:
                                                                                           uint32_t)
                                                                      -> ()),
                                                     cancel:
                                                         Some(frame_cancel as
                                                                  unsafe extern "C" fn(_:
                                                                                           *mut libc::c_void,
                                                                                       _:
                                                                                           *mut zwlr_export_dmabuf_frame_v1,
                                                                                       _:
                                                                                           uint32_t)
                                                                      ->
                                                                          ()),};
            init
        }
    };
unsafe extern "C" fn register_cb(mut ctx: *mut capture_context) {
    (*ctx).frame_callback =
        zwlr_export_dmabuf_manager_v1_capture_output((*ctx).export_manager,
                                                     (*ctx).with_cursor as
                                                         int32_t,
                                                     (*ctx).target_output);
    zwlr_export_dmabuf_frame_v1_add_listener((*ctx).frame_callback,
                                             &frame_listener,
                                             ctx as *mut libc::c_void);
}
unsafe extern "C" fn vid_encode_thread(mut arg: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut err: libc::c_int = 0i32;
    let mut ctx: *mut capture_context = arg as *mut capture_context;
    's_11:
        loop  {
            let mut f: *mut AVFrame = 0 as *mut AVFrame;
            if get_fifo_size(&mut (*ctx).vid_frames) != 0 || !(*ctx).quit {
                f = pop_from_fifo(&mut (*ctx).vid_frames)
            }
            if (*ctx).is_software_encoder != 0 && !f.is_null() {
                let mut soft_frame: *mut AVFrame = av_frame_alloc();
                av_hwframe_transfer_data(soft_frame, f, 0i32);
                (*soft_frame).pts = (*f).pts;
                av_frame_free(&mut f);
                f = soft_frame
            }
            err = avcodec_send_frame((*ctx).avctx, f);
            av_frame_free(&mut f);
            if err != 0 {
                av_log(ctx as *mut libc::c_void, 16i32,
                       b"Error encoding: %s!\n\x00" as *const u8 as
                           *const libc::c_char,
                       av_make_error_string([0i32 as libc::c_char, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0].as_mut_ptr(),
                                            64i32 as size_t, err));
                break ;
            } else {
                loop  {
                    let mut pkt: AVPacket =
                        AVPacket{buf: 0 as *mut AVBufferRef,
                                 pts: 0,
                                 dts: 0,
                                 data: 0 as *mut uint8_t,
                                 size: 0,
                                 stream_index: 0,
                                 flags: 0,
                                 side_data: 0 as *mut AVPacketSideData,
                                 side_data_elems: 0,
                                 duration: 0,
                                 pos: 0,
                                 convergence_duration: 0,};
                    av_init_packet(&mut pkt);
                    let mut ret: libc::c_int =
                        avcodec_receive_packet((*ctx).avctx, &mut pkt);
                    if ret == -11i32 { break ; }
                    if ret ==
                           -((('E' as i32 | ('O' as i32) << 8i32 |
                                   ('F' as i32) << 16i32) as libc::c_uint |
                                  (' ' as i32 as libc::c_uint) << 24i32) as
                                 libc::c_int) {
                        av_log(ctx as *mut libc::c_void, 32i32,
                               b"Encoder flushed!\n\x00" as *const u8 as
                                   *const libc::c_char);
                        break 's_11 ;
                    } else if ret != 0 {
                        av_log(ctx as *mut libc::c_void, 16i32,
                               b"Error encoding: %s!\n\x00" as *const u8 as
                                   *const libc::c_char,
                               av_make_error_string([0i32 as libc::c_char, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0,
                                                     0].as_mut_ptr(),
                                                    64i32 as size_t, ret));
                        err = ret;
                        break 's_11 ;
                    } else {
                        pkt.stream_index = 0i32;
                        err =
                            av_interleaved_write_frame((*ctx).avf, &mut pkt);
                        av_packet_unref(&mut pkt);
                        if !(err != 0) { continue ; }
                        av_log(ctx as *mut libc::c_void, 16i32,
                               b"Writing packet fail: %s!\n\x00" as *const u8
                                   as *const libc::c_char,
                               av_make_error_string([0i32 as libc::c_char, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0, 0, 0, 0,
                                                     0, 0, 0, 0, 0,
                                                     0].as_mut_ptr(),
                                                    64i32 as size_t, err));
                        break 's_11 ;
                    }
                }
                av_log(ctx as *mut libc::c_void, 32i32,
                       b"Encoded frame %i (%i in queue)\n\x00" as *const u8 as
                           *const libc::c_char, (*(*ctx).avctx).frame_number,
                       get_fifo_size(&mut (*ctx).vid_frames));
                if !((*ctx).err == 0) { break ; }
            }
        }
    if (*ctx).err == 0 { (*ctx).err = err }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn init_lavu_hwcontext(mut ctx: *mut capture_context)
 -> libc::c_int {
    /* DRM hwcontext */
    (*ctx).drm_device_ref = av_hwdevice_ctx_alloc(AV_HWDEVICE_TYPE_DRM);
    if (*ctx).drm_device_ref.is_null() { return -12i32 }
    let mut ref_data: *mut AVHWDeviceContext =
        (*(*ctx).drm_device_ref).data as *mut AVHWDeviceContext;
    let mut hwctx: *mut AVDRMDeviceContext =
        (*ref_data).hwctx as *mut AVDRMDeviceContext;
    /* We don't need a device (we don't even know it and can't open it) */
    (*hwctx).fd = -1i32;
    av_hwdevice_ctx_init((*ctx).drm_device_ref);
    /* Mapped hwcontext */
    let mut err: libc::c_int =
        av_hwdevice_ctx_create(&mut (*ctx).mapped_device_ref,
                               (*ctx).hw_device_type, (*ctx).hardware_device,
                               0 as *mut AVDictionary, 0i32);
    if err < 0i32 {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Failed to create a hardware device: %s\n\x00" as *const u8 as
                   *const libc::c_char,
               av_make_error_string([0i32 as libc::c_char, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0].as_mut_ptr(), 64i32 as size_t, err));
        return err
    }
    return 0i32;
}
unsafe extern "C" fn set_hwframe_ctx(mut ctx: *mut capture_context,
                                     mut hw_device_ctx: *mut AVBufferRef)
 -> libc::c_int {
    let mut frames_ctx: *mut AVHWFramesContext = 0 as *mut AVHWFramesContext;
    let mut err: libc::c_int = 0i32;
    (*ctx).mapped_frames_ref = av_hwframe_ctx_alloc(hw_device_ctx);
    if (*ctx).mapped_frames_ref.is_null() { return -12i32 }
    let mut cst: *mut AVHWFramesConstraints =
        av_hwdevice_get_hwframe_constraints((*ctx).mapped_device_ref,
                                            0 as *const libc::c_void);
    if cst.is_null() {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Failed to get hw device constraints!\n\x00" as *const u8 as
                   *const libc::c_char);
        av_buffer_unref(&mut (*ctx).mapped_frames_ref);
        return -12i32
    }
    frames_ctx = (*(*ctx).mapped_frames_ref).data as *mut AVHWFramesContext;
    (*frames_ctx).format = *(*cst).valid_hw_formats.offset(0);
    (*frames_ctx).sw_format = (*(*ctx).avctx).pix_fmt;
    (*frames_ctx).width = (*(*ctx).avctx).width;
    (*frames_ctx).height = (*(*ctx).avctx).height;
    av_hwframe_constraints_free(&mut cst);
    err = av_hwframe_ctx_init((*ctx).mapped_frames_ref);
    if err != 0 {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Failed to initialize hw frame context: %s!\n\x00" as
                   *const u8 as *const libc::c_char,
               av_make_error_string([0i32 as libc::c_char, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0].as_mut_ptr(), 64i32 as size_t, err));
        av_buffer_unref(&mut (*ctx).mapped_frames_ref);
        return err
    }
    if (*ctx).is_software_encoder == 0 {
        (*(*ctx).avctx).pix_fmt = (*frames_ctx).format;
        (*(*ctx).avctx).hw_frames_ctx =
            av_buffer_ref((*ctx).mapped_frames_ref);
        if (*(*ctx).avctx).hw_frames_ctx.is_null() {
            av_buffer_unref(&mut (*ctx).mapped_frames_ref);
            err = -12i32
        }
    }
    return err;
}
unsafe extern "C" fn init_encoding(mut ctx: *mut capture_context)
 -> libc::c_int {
    let mut err: libc::c_int = 0;
    /* lavf init */
    err =
        avformat_alloc_output_context2(&mut (*ctx).avf,
                                       0 as *mut AVOutputFormat,
                                       0 as *const libc::c_char,
                                       (*ctx).out_filename);
    if err != 0 {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Unable to init lavf context!\n\x00" as *const u8 as
                   *const libc::c_char);
        return err
    }
    let mut st: *mut AVStream =
        avformat_new_stream((*ctx).avf, 0 as *const AVCodec);
    if st.is_null() {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Unable to alloc stream!\n\x00" as *const u8 as
                   *const libc::c_char);
        return 1i32
    }
    /* Find encoder */
    let mut out_codec: *mut AVCodec =
        avcodec_find_encoder_by_name((*ctx).encoder_name);
    if out_codec.is_null() {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Codec not found (not compiled in lavc?)!\n\x00" as *const u8
                   as *const libc::c_char);
        return -22i32
    }
    (*(*(*ctx).avf).oformat).video_codec = (*out_codec).id;
    (*ctx).is_software_encoder =
        ((*out_codec).capabilities & 1i32 << 18i32 == 0) as libc::c_int;
    (*ctx).avctx = avcodec_alloc_context3(out_codec);
    if (*ctx).avctx.is_null() { return 1i32 }
    (*(*ctx).avctx).opaque = ctx as *mut libc::c_void;
    (*(*ctx).avctx).bit_rate =
        ((*ctx).out_bitrate as libc::c_int as libc::c_float * 1000000.0f32) as
            int64_t;
    (*(*ctx).avctx).pix_fmt = (*ctx).software_format;
    (*(*ctx).avctx).time_base =
        { let mut init = AVRational{num: 1i32, den: 1000i32,}; init };
    (*(*ctx).avctx).compression_level = 7i32;
    (*(*ctx).avctx).width =
        (*find_output(ctx, (*ctx).target_output, 0i32 as uint32_t)).width;
    (*(*ctx).avctx).height =
        (*find_output(ctx, (*ctx).target_output, 0i32 as uint32_t)).height;
    if (*(*(*ctx).avf).oformat).flags & 0x40i32 != 0 {
        (*(*ctx).avctx).flags |= 1i32 << 22i32
    }
    (*st).id = 0i32;
    (*st).time_base = (*(*ctx).avctx).time_base;
    (*st).avg_frame_rate =
        (*find_output(ctx, (*ctx).target_output, 0i32 as uint32_t)).framerate;
    /* Init hw frames context */
    err = set_hwframe_ctx(ctx, (*ctx).mapped_device_ref);
    if err != 0 { return err }
    err = avcodec_open2((*ctx).avctx, out_codec, &mut (*ctx).encoder_opts);
    if err != 0 {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Cannot open encoder: %s!\n\x00" as *const u8 as
                   *const libc::c_char,
               av_make_error_string([0i32 as libc::c_char, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0].as_mut_ptr(), 64i32 as size_t, err));
        return err
    }
    if avcodec_parameters_from_context((*st).codecpar, (*ctx).avctx) < 0i32 {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Couldn\'t copy codec params: %s!\n\x00" as *const u8 as
                   *const libc::c_char,
               av_make_error_string([0i32 as libc::c_char, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0].as_mut_ptr(), 64i32 as size_t, err));
        return err
    }
    /* Debug print */
    av_dump_format((*ctx).avf, 0i32, (*ctx).out_filename, 1i32);
    /* Open for writing */
    err = avio_open(&mut (*(*ctx).avf).pb, (*ctx).out_filename, 2i32);
    if err != 0 {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Couldn\'t open %s: %s!\n\x00" as *const u8 as
                   *const libc::c_char, (*ctx).out_filename,
               av_make_error_string([0i32 as libc::c_char, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0].as_mut_ptr(), 64i32 as size_t, err));
        return err
    }
    err = avformat_write_header((*ctx).avf, 0 as *mut *mut AVDictionary);
    if err != 0 {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Couldn\'t write header: %s!\n\x00" as *const u8 as
                   *const libc::c_char,
               av_make_error_string([0i32 as libc::c_char, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0].as_mut_ptr(), 64i32 as size_t, err));
        return err
    }
    return err;
}
#[no_mangle]
pub static mut q_ctx: *mut capture_context =
    0 as *const capture_context as *mut capture_context;
unsafe extern "C" fn on_quit_signal(mut signo: libc::c_int) {
    printf(b"\r\x00" as *const u8 as *const libc::c_char);
    av_log(q_ctx as *mut libc::c_void, 24i32,
           b"Quitting!\n\x00" as *const u8 as *const libc::c_char);
    (*q_ctx).quit = 1i32 != 0;
}
unsafe extern "C" fn main_loop(mut ctx: *mut capture_context) -> libc::c_int {
    let mut err: libc::c_int = 0;
    q_ctx = ctx;
    if signal(2i32,
              Some(on_quit_signal as
                       unsafe extern "C" fn(_: libc::c_int) -> ())) ==
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(-1i32 as libc::intptr_t) {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Unable to install signal handler!\n\x00" as *const u8 as
                   *const libc::c_char);
        return -22i32
    }
    err = init_lavu_hwcontext(ctx);
    if err != 0 { return err }
    err = init_encoding(ctx);
    if err != 0 { return err }
    /* Start video encoding thread */
    err = init_fifo(&mut (*ctx).vid_frames, 16i32);
    if err != 0 { return err }
    pthread_create(&mut (*ctx).vid_thread, 0 as *const pthread_attr_t,
                   Some(vid_encode_thread as
                            unsafe extern "C" fn(_: *mut libc::c_void)
                                -> *mut libc::c_void),
                   ctx as *mut libc::c_void);
    /* Start the frame callback */
    register_cb(ctx);
    /* Run capture */
    while wl_display_dispatch((*ctx).display) != -1i32 && (*ctx).err == 0 &&
              !(*ctx).quit {
    }
    /* Join with encoder thread */
    pthread_join((*ctx).vid_thread, 0 as *mut *mut libc::c_void);
    err = av_write_trailer((*ctx).avf);
    if err != 0 {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Error writing trailer: %s!\n\x00" as *const u8 as
                   *const libc::c_char,
               av_make_error_string([0i32 as libc::c_char, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                     0].as_mut_ptr(), 64i32 as size_t, err));
        return err
    }
    av_log(ctx as *mut libc::c_void, 32i32,
           b"Wrote trailer!\n\x00" as *const u8 as *const libc::c_char);
    return (*ctx).err;
}
unsafe extern "C" fn init(mut ctx: *mut capture_context) -> libc::c_int {
    (*ctx).display = wl_display_connect(0 as *const libc::c_char);
    if (*ctx).display.is_null() {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Failed to connect to display!\n\x00" as *const u8 as
                   *const libc::c_char);
        return -22i32
    }
    wl_list_init(&mut (*ctx).output_list);
    (*ctx).registry = wl_display_get_registry((*ctx).display);
    wl_registry_add_listener((*ctx).registry, &registry_listener,
                             ctx as *mut libc::c_void);
    wl_display_roundtrip((*ctx).display);
    wl_display_dispatch((*ctx).display);
    if (*ctx).export_manager.is_null() {
        av_log(ctx as *mut libc::c_void, 16i32,
               b"Compositor doesn\'t support %s!\n\x00" as *const u8 as
                   *const libc::c_char,
               zwlr_export_dmabuf_manager_v1_interface.name);
        return -1i32
    }
    return 0i32;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut o: *mut wayland_output = 0 as *mut wayland_output;
    let mut tmp_o: *mut wayland_output = 0 as *mut wayland_output;
    let mut o_id: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut ctx: capture_context =
        {
            let mut init =
                capture_context{class: 0 as *mut AVClass,
                                display: 0 as *mut wl_display,
                                registry: 0 as *mut wl_registry,
                                export_manager:
                                    0 as *mut zwlr_export_dmabuf_manager_v1,
                                output_list:
                                    wl_list{prev: 0 as *mut wl_list,
                                            next: 0 as *mut wl_list,},
                                target_output: 0 as *mut wl_output,
                                with_cursor: false,
                                frame_callback:
                                    0 as *mut zwlr_export_dmabuf_frame_v1,
                                err: 0,
                                quit: false,
                                vid_thread: 0,
                                current_frame: 0 as *mut AVFrame,
                                avf: 0 as *mut AVFormatContext,
                                avctx: 0 as *mut AVCodecContext,
                                drm_device_ref: 0 as *mut AVBufferRef,
                                drm_frames_ref: 0 as *mut AVBufferRef,
                                mapped_device_ref: 0 as *mut AVBufferRef,
                                mapped_frames_ref: 0 as *mut AVBufferRef,
                                vid_frames:
                                    fifo_buffer{queued_frames:
                                                    0 as *mut *mut AVFrame,
                                                num_queued_frames: 0,
                                                max_queued_frames: 0,
                                                lock:
                                                    pthread_mutex_t{__data:
                                                                        __pthread_mutex_s{__lock:
                                                                                              0,
                                                                                          __count:
                                                                                              0,
                                                                                          __owner:
                                                                                              0,
                                                                                          __nusers:
                                                                                              0,
                                                                                          __kind:
                                                                                              0,
                                                                                          __spins:
                                                                                              0,
                                                                                          __elision:
                                                                                              0,
                                                                                          __list:
                                                                                              __pthread_list_t{__prev:
                                                                                                                   0
                                                                                                                       as
                                                                                                                       *mut __pthread_internal_list,
                                                                                                               __next:
                                                                                                                   0
                                                                                                                       as
                                                                                                                       *mut __pthread_internal_list,},},},
                                                cond:
                                                    pthread_cond_t{__data:
                                                                       __pthread_cond_s{c2rust_unnamed:
                                                                                            C2RustUnnamed_2{__wseq:
                                                                                                                0,},
                                                                                        c2rust_unnamed_0:
                                                                                            C2RustUnnamed_0{__g1_start:
                                                                                                                0,},
                                                                                        __g_refs:
                                                                                            [0;
                                                                                                2],
                                                                                        __g_size:
                                                                                            [0;
                                                                                                2],
                                                                                        __g1_orig_size:
                                                                                            0,
                                                                                        __wrefs:
                                                                                            0,
                                                                                        __g_signals:
                                                                                            [0;
                                                                                                2],},},
                                                cond_lock:
                                                    pthread_mutex_t{__data:
                                                                        __pthread_mutex_s{__lock:
                                                                                              0,
                                                                                          __count:
                                                                                              0,
                                                                                          __owner:
                                                                                              0,
                                                                                          __nusers:
                                                                                              0,
                                                                                          __kind:
                                                                                              0,
                                                                                          __spins:
                                                                                              0,
                                                                                          __elision:
                                                                                              0,
                                                                                          __list:
                                                                                              __pthread_list_t{__prev:
                                                                                                                   0
                                                                                                                       as
                                                                                                                       *mut __pthread_internal_list,
                                                                                                               __next:
                                                                                                                   0
                                                                                                                       as
                                                                                                                       *mut __pthread_internal_list,},},},},
                                start_pts: 0,
                                software_format: AV_PIX_FMT_YUV420P,
                                hw_device_type: AV_HWDEVICE_TYPE_NONE,
                                encoder_opts: 0 as *mut AVDictionary,
                                is_software_encoder: 0,
                                hardware_device: 0 as *mut libc::c_char,
                                out_filename: 0 as *mut libc::c_char,
                                encoder_name: 0 as *mut libc::c_char,
                                out_bitrate: 0.,};
            init
        };
    ctx.class =
        &mut {
                 let mut init =
                     AVClass{class_name:
                                 b"dmabuf-capture\x00" as *const u8 as
                                     *const libc::c_char,
                             item_name:
                                 Some(av_default_item_name as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void)
                                              -> *const libc::c_char),
                             option: 0 as *const AVOption,
                             version: 56i32 << 16i32 | 31i32 << 8i32 | 100i32,
                             log_level_offset_offset: 0,
                             parent_log_context_offset: 0,
                             child_next: None,
                             child_class_next: None,
                             category: AV_CLASS_CATEGORY_NA,
                             get_category: None,
                             query_ranges: None,};
                 init
             } as *mut AVClass;
    err = init(&mut ctx);
    if !(err != 0) {
        o = 0 as *mut wayland_output;
        tmp_o = 0 as *mut wayland_output;
        o =
            (ctx.output_list.prev as *mut libc::c_char).offset(-0) as
                *mut wayland_output;
        tmp_o =
            ((*o).link.prev as *mut libc::c_char).offset(-0) as
                *mut wayland_output;
        while &mut (*o).link as *mut wl_list !=
                  &mut ctx.output_list as *mut wl_list {
            printf(b"Capturable output: %s Model: %s: ID: %i\n\x00" as
                       *const u8 as *const libc::c_char, (*o).make,
                   (*o).model, (*o).id);
            o = tmp_o;
            tmp_o =
                ((*o).link.prev as *mut libc::c_char).offset(-0) as
                    *mut wayland_output
        }
        if argc != 8i32 {
            printf(b"Invalid number of arguments! Usage and example:\n./dmabuf-capture <source id> <hardware device type> <device> <encoder name> <pixel format> <bitrate in Mbps> <file path>\n./dmabuf-capture 0 vaapi /dev/dri/renderD129 libx264 nv12 12 dmabuf_recording_01.mkv\n\x00"
                       as *const u8 as *const libc::c_char);
            return 1i32
        }
        o_id =
            strtol(*argv.offset(1), 0 as *mut *mut libc::c_char, 10i32) as
                libc::c_int;
        o = find_output(&mut ctx, 0 as *mut wl_output, o_id as uint32_t);
        if o.is_null() {
            printf(b"Unable to find output with ID %i!\n\x00" as *const u8 as
                       *const libc::c_char, o_id);
            return 1i32
        }
        ctx.target_output = (*o).output;
        ctx.with_cursor = 1i32 != 0;
        ctx.hw_device_type = av_hwdevice_find_type_by_name(*argv.offset(2));
        ctx.hardware_device = *argv.offset(3);
        ctx.encoder_name = *argv.offset(4);
        ctx.software_format = av_get_pix_fmt(*argv.offset(5));
        ctx.out_bitrate =
            strtof(*argv.offset(6), 0 as *mut *mut libc::c_char);
        ctx.out_filename = *argv.offset(7);
        av_dict_set(&mut ctx.encoder_opts,
                    b"preset\x00" as *const u8 as *const libc::c_char,
                    b"veryfast\x00" as *const u8 as *const libc::c_char,
                    0i32);
        err = main_loop(&mut ctx);
        (err) != 0;
    }
    uninit(&mut ctx);
    return err;
}
unsafe extern "C" fn uninit(mut ctx: *mut capture_context) {
    let mut output: *mut wayland_output = 0 as *mut wayland_output;
    let mut tmp_o: *mut wayland_output = 0 as *mut wayland_output;
    output =
        ((*ctx).output_list.next as *mut libc::c_char).offset(-0) as
            *mut wayland_output;
    tmp_o =
        ((*output).link.next as *mut libc::c_char).offset(-0) as
            *mut wayland_output;
    while &mut (*output).link as *mut wl_list !=
              &mut (*ctx).output_list as *mut wl_list {
        remove_output(output);
        output = tmp_o;
        tmp_o =
            ((*output).link.next as *mut libc::c_char).offset(-0) as
                *mut wayland_output
    }
    if !(*ctx).export_manager.is_null() {
        zwlr_export_dmabuf_manager_v1_destroy((*ctx).export_manager);
    }
    free_fifo(&mut (*ctx).vid_frames);
    av_buffer_unref(&mut (*ctx).drm_frames_ref);
    av_buffer_unref(&mut (*ctx).drm_device_ref);
    av_buffer_unref(&mut (*ctx).mapped_frames_ref);
    av_buffer_unref(&mut (*ctx).mapped_device_ref);
    av_dict_free(&mut (*ctx).encoder_opts);
    avcodec_close((*ctx).avctx);
    if !(*ctx).avf.is_null() { avio_closep(&mut (*(*ctx).avf).pb); }
    avformat_free_context((*ctx).avf);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
