use libc;
extern "C" {
    pub type __dirstream;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
             _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fwrite(_: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
              _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __ino64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type C2RustUnnamed = libc::c_uint;
pub const DT_WHT: C2RustUnnamed = 14;
pub const DT_SOCK: C2RustUnnamed = 12;
pub const DT_LNK: C2RustUnnamed = 10;
pub const DT_REG: C2RustUnnamed = 8;
pub const DT_BLK: C2RustUnnamed = 6;
pub const DT_DIR: C2RustUnnamed = 4;
pub const DT_CHR: C2RustUnnamed = 2;
pub const DT_FIFO: C2RustUnnamed = 1;
pub const DT_UNKNOWN: C2RustUnnamed = 0;
pub type DIR = __dirstream;
pub type size_t = libc::c_ulong;
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
/*
 * Copyright © 2002 Keith Packard
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
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
pub type XcursorBool = libc::c_int;
pub type XcursorUInt = libc::c_uint;
pub type XcursorDim = XcursorUInt;
pub type XcursorPixel = XcursorUInt;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _XcursorImage {
    pub version: XcursorUInt,
    pub size: XcursorDim,
    pub width: XcursorDim,
    pub height: XcursorDim,
    pub xhot: XcursorDim,
    pub yhot: XcursorDim,
    pub delay: XcursorUInt,
    pub pixels: *mut XcursorPixel,
}
pub type XcursorImage = _XcursorImage;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _XcursorImages {
    pub nimage: libc::c_int,
    pub images: *mut *mut XcursorImage,
    pub name: *mut libc::c_char,
}
/* version of the image data */
/* nominal size for matching */
/* actual width */
/* actual height */
/* hot spot x (must be inside image) */
/* hot spot y (must be inside image) */
/* animation delay to next frame (ms) */
/* pointer to pixels */
/*
 * Other data structures exposed by the library API
 */
pub type XcursorImages = _XcursorImages;
pub type XcursorFileHeader = _XcursorFileHeader;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _XcursorFileHeader {
    pub magic: XcursorUInt,
    pub header: XcursorUInt,
    pub version: XcursorUInt,
    pub ntoc: XcursorUInt,
    pub tocs: *mut XcursorFileToc,
}
pub type XcursorFileToc = _XcursorFileToc;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _XcursorFileToc {
    pub type_0: XcursorUInt,
    pub subtype: XcursorUInt,
    pub position: XcursorUInt,
}
/* number of images */
/* array of XcursorImage pointers */
/* name used to load images */
/* magic number */
/* byte length of header */
/* file version number */
/* number of toc entries */
/* table of contents */
/* chunk type */
/* subtype (size for images) */
/* absolute position in file */
/* 32767x32767 max cursor size */
pub type XcursorFile = _XcursorFile;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _XcursorFile {
    pub closure: *mut libc::c_void,
    pub read: Option<unsafe extern "C" fn(_: *mut XcursorFile,
                                          _: *mut libc::c_uchar,
                                          _: libc::c_int) -> libc::c_int>,
    pub write: Option<unsafe extern "C" fn(_: *mut XcursorFile,
                                           _: *mut libc::c_uchar,
                                           _: libc::c_int) -> libc::c_int>,
    pub seek: Option<unsafe extern "C" fn(_: *mut XcursorFile,
                                          _: libc::c_long, _: libc::c_int)
                         -> libc::c_int>,
}
/*
 * The rest of the file is a list of chunks, each tagged by type
 * and version.
 *
 *  Chunk:
 *	ChunkHeader
 *	<extra type-specific header fields>
 *	<type-specific data>
 *
 *  ChunkHeader:
 *	CARD32	    header	bytes in chunk header + type header
 *	CARD32	    type	chunk type
 *	CARD32	    subtype	chunk subtype
 *	CARD32	    version	chunk type version
 */
pub type XcursorChunkHeader = _XcursorChunkHeader;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _XcursorChunkHeader {
    pub header: XcursorUInt,
    pub type_0: XcursorUInt,
    pub subtype: XcursorUInt,
    pub version: XcursorUInt,
}
/* bytes in chunk header */
/* chunk type */
/* chunk subtype (size for images) */
/* version of this type */
/*
 * From libXcursor/src/file.c
 */
unsafe extern "C" fn XcursorImageCreate(mut width: libc::c_int,
                                        mut height: libc::c_int)
 -> *mut XcursorImage {
    let mut image: *mut XcursorImage = 0 as *mut XcursorImage;
    if width < 0i32 || height < 0i32 { return 0 as *mut XcursorImage }
    if width > 0x7fffi32 || height > 0x7fffi32 {
        return 0 as *mut XcursorImage
    }
    image =
        malloc((::std::mem::size_of::<XcursorImage>() as
                    libc::c_ulong).wrapping_add(((width * height) as
                                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<XcursorPixel>()
                                                                                     as
                                                                                     libc::c_ulong)))
            as *mut XcursorImage;
    if image.is_null() { return 0 as *mut XcursorImage }
    (*image).version = 1i32 as XcursorUInt;
    (*image).pixels = image.offset(1) as *mut XcursorPixel;
    (*image).size = if width > height { width } else { height } as XcursorDim;
    (*image).width = width as XcursorDim;
    (*image).height = height as XcursorDim;
    (*image).delay = 0i32 as XcursorUInt;
    return image;
}
unsafe extern "C" fn XcursorImageDestroy(mut image: *mut XcursorImage) {
    free(image as *mut libc::c_void);
}
unsafe extern "C" fn XcursorImagesCreate(mut size: libc::c_int)
 -> *mut XcursorImages {
    let mut images: *mut XcursorImages = 0 as *mut XcursorImages;
    images =
        malloc((::std::mem::size_of::<XcursorImages>() as
                    libc::c_ulong).wrapping_add((size as
                                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut XcursorImage>()
                                                                                     as
                                                                                     libc::c_ulong)))
            as *mut XcursorImages;
    if images.is_null() { return 0 as *mut XcursorImages }
    (*images).nimage = 0i32;
    (*images).images = images.offset(1) as *mut *mut XcursorImage;
    (*images).name = 0 as *mut libc::c_char;
    return images;
}
#[no_mangle]
pub unsafe extern "C" fn XcursorImagesDestroy(mut images:
                                                  *mut XcursorImages) {
    let mut n: libc::c_int = 0;
    if images.is_null() { return }
    n = 0i32;
    while n < (*images).nimage {
        XcursorImageDestroy(*(*images).images.offset(n as isize));
        n += 1
    }
    if !(*images).name.is_null() {
        free((*images).name as *mut libc::c_void);
    }
    free(images as *mut libc::c_void);
}
unsafe extern "C" fn XcursorImagesSetName(mut images: *mut XcursorImages,
                                          mut name: *const libc::c_char) {
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    if images.is_null() || name.is_null() { return }
    new =
        malloc(strlen(name).wrapping_add(1i32 as libc::c_ulong)) as
            *mut libc::c_char;
    if new.is_null() { return }
    strcpy(new, name);
    if !(*images).name.is_null() {
        free((*images).name as *mut libc::c_void);
    }
    (*images).name = new;
}
unsafe extern "C" fn _XcursorReadUInt(mut file: *mut XcursorFile,
                                      mut u: *mut XcursorUInt)
 -> XcursorBool {
    let mut bytes: [libc::c_uchar; 4] = [0; 4];
    if file.is_null() || u.is_null() { return 0i32 }
    if Some((*file).read.expect("non-null function pointer")).expect("non-null function pointer")(file,
                                                                                                  bytes.as_mut_ptr(),
                                                                                                  4i32)
           != 4i32 {
        return 0i32
    }
    *u =
        ((bytes[0] as libc::c_int) << 0i32 | (bytes[1] as libc::c_int) << 8i32
             | (bytes[2] as libc::c_int) << 16i32 |
             (bytes[3] as libc::c_int) << 24i32) as XcursorUInt;
    return 1i32;
}
unsafe extern "C" fn _XcursorFileHeaderDestroy(mut fileHeader:
                                                   *mut XcursorFileHeader) {
    free(fileHeader as *mut libc::c_void);
}
unsafe extern "C" fn _XcursorFileHeaderCreate(mut ntoc: libc::c_int)
 -> *mut XcursorFileHeader {
    let mut fileHeader: *mut XcursorFileHeader = 0 as *mut XcursorFileHeader;
    if ntoc > 0x10000i32 { return 0 as *mut XcursorFileHeader }
    fileHeader =
        malloc((::std::mem::size_of::<XcursorFileHeader>() as
                    libc::c_ulong).wrapping_add((ntoc as
                                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<XcursorFileToc>()
                                                                                     as
                                                                                     libc::c_ulong)))
            as *mut XcursorFileHeader;
    if fileHeader.is_null() { return 0 as *mut XcursorFileHeader }
    (*fileHeader).magic = 0x72756358i32 as XcursorUInt;
    (*fileHeader).header = (4i32 * 4i32) as XcursorUInt;
    (*fileHeader).version = (1i32 << 16i32 | 0i32) as XcursorUInt;
    (*fileHeader).ntoc = ntoc as XcursorUInt;
    (*fileHeader).tocs = fileHeader.offset(1) as *mut XcursorFileToc;
    return fileHeader;
}
unsafe extern "C" fn _XcursorReadFileHeader(mut file: *mut XcursorFile)
 -> *mut XcursorFileHeader {
    let mut head: XcursorFileHeader =
        XcursorFileHeader{magic: 0,
                          header: 0,
                          version: 0,
                          ntoc: 0,
                          tocs: 0 as *mut XcursorFileToc,};
    let mut fileHeader: *mut XcursorFileHeader = 0 as *mut XcursorFileHeader;
    let mut skip: XcursorUInt = 0;
    let mut n: libc::c_uint = 0;
    if file.is_null() { return 0 as *mut XcursorFileHeader }
    if _XcursorReadUInt(file, &mut head.magic) == 0 {
        return 0 as *mut XcursorFileHeader
    }
    if head.magic != 0x72756358i32 as libc::c_uint {
        return 0 as *mut XcursorFileHeader
    }
    if _XcursorReadUInt(file, &mut head.header) == 0 {
        return 0 as *mut XcursorFileHeader
    }
    if _XcursorReadUInt(file, &mut head.version) == 0 {
        return 0 as *mut XcursorFileHeader
    }
    if _XcursorReadUInt(file, &mut head.ntoc) == 0 {
        return 0 as *mut XcursorFileHeader
    }
    skip = head.header.wrapping_sub((4i32 * 4i32) as libc::c_uint);
    if skip != 0 {
        if Some((*file).seek.expect("non-null function pointer")).expect("non-null function pointer")(file,
                                                                                                      skip
                                                                                                          as
                                                                                                          libc::c_long,
                                                                                                      1i32)
               == -1i32 {
            return 0 as *mut XcursorFileHeader
        }
    }
    fileHeader = _XcursorFileHeaderCreate(head.ntoc as libc::c_int);
    if fileHeader.is_null() { return 0 as *mut XcursorFileHeader }
    (*fileHeader).magic = head.magic;
    (*fileHeader).header = head.header;
    (*fileHeader).version = head.version;
    (*fileHeader).ntoc = head.ntoc;
    n = 0i32 as libc::c_uint;
    while n < (*fileHeader).ntoc {
        if _XcursorReadUInt(file,
                            &mut (*(*fileHeader).tocs.offset(n as
                                                                 isize)).type_0)
               == 0 {
            break ;
        }
        if _XcursorReadUInt(file,
                            &mut (*(*fileHeader).tocs.offset(n as
                                                                 isize)).subtype)
               == 0 {
            break ;
        }
        if _XcursorReadUInt(file,
                            &mut (*(*fileHeader).tocs.offset(n as
                                                                 isize)).position)
               == 0 {
            break ;
        }
        n = n.wrapping_add(1)
    }
    if n != (*fileHeader).ntoc {
        _XcursorFileHeaderDestroy(fileHeader);
        return 0 as *mut XcursorFileHeader
    }
    return fileHeader;
}
unsafe extern "C" fn _XcursorSeekToToc(mut file: *mut XcursorFile,
                                       mut fileHeader: *mut XcursorFileHeader,
                                       mut toc: libc::c_int) -> XcursorBool {
    if file.is_null() || fileHeader.is_null() ||
           Some((*file).seek.expect("non-null function pointer")).expect("non-null function pointer")(file,
                                                                                                      (*(*fileHeader).tocs.offset(toc
                                                                                                                                      as
                                                                                                                                      isize)).position
                                                                                                          as
                                                                                                          libc::c_long,
                                                                                                      0i32)
               == -1i32 {
        return 0i32
    }
    return 1i32;
}
unsafe extern "C" fn _XcursorFileReadChunkHeader(mut file: *mut XcursorFile,
                                                 mut fileHeader:
                                                     *mut XcursorFileHeader,
                                                 mut toc: libc::c_int,
                                                 mut chunkHeader:
                                                     *mut XcursorChunkHeader)
 -> XcursorBool {
    if file.is_null() || fileHeader.is_null() || chunkHeader.is_null() {
        return 0i32
    }
    if _XcursorSeekToToc(file, fileHeader, toc) == 0 { return 0i32 }
    if _XcursorReadUInt(file, &mut (*chunkHeader).header) == 0 { return 0i32 }
    if _XcursorReadUInt(file, &mut (*chunkHeader).type_0) == 0 { return 0i32 }
    if _XcursorReadUInt(file, &mut (*chunkHeader).subtype) == 0 {
        return 0i32
    }
    if _XcursorReadUInt(file, &mut (*chunkHeader).version) == 0 {
        return 0i32
    }
    /* sanity check */
    if (*chunkHeader).type_0 !=
           (*(*fileHeader).tocs.offset(toc as isize)).type_0 ||
           (*chunkHeader).subtype !=
               (*(*fileHeader).tocs.offset(toc as isize)).subtype {
        return 0i32
    }
    return 1i32;
}
unsafe extern "C" fn _XcursorFindBestSize(mut fileHeader:
                                              *mut XcursorFileHeader,
                                          mut size: XcursorDim,
                                          mut nsizesp: *mut libc::c_int)
 -> XcursorDim {
    let mut n: libc::c_uint = 0;
    let mut nsizes: libc::c_int = 0i32;
    let mut bestSize: XcursorDim = 0i32 as XcursorDim;
    let mut thisSize: XcursorDim = 0;
    if fileHeader.is_null() || nsizesp.is_null() { return 0i32 as XcursorDim }
    n = 0i32 as libc::c_uint;
    while n < (*fileHeader).ntoc {
        if !((*(*fileHeader).tocs.offset(n as isize)).type_0 != 0xfffd0002u32)
           {
            thisSize = (*(*fileHeader).tocs.offset(n as isize)).subtype;
            if bestSize == 0 ||
                   (if thisSize > size {
                        thisSize.wrapping_sub(size)
                    } else { size.wrapping_sub(thisSize) }) <
                       (if bestSize > size {
                            bestSize.wrapping_sub(size)
                        } else { size.wrapping_sub(bestSize) }) {
                bestSize = thisSize;
                nsizes = 1i32
            } else if thisSize == bestSize { nsizes += 1 }
        }
        n = n.wrapping_add(1)
    }
    *nsizesp = nsizes;
    return bestSize;
}
unsafe extern "C" fn _XcursorFindImageToc(mut fileHeader:
                                              *mut XcursorFileHeader,
                                          mut size: XcursorDim,
                                          mut count: libc::c_int)
 -> libc::c_int {
    let mut toc: libc::c_uint = 0;
    let mut thisSize: XcursorDim = 0;
    if fileHeader.is_null() { return 0i32 }
    toc = 0i32 as libc::c_uint;
    while toc < (*fileHeader).ntoc {
        if !((*(*fileHeader).tocs.offset(toc as isize)).type_0 !=
                 0xfffd0002u32) {
            thisSize = (*(*fileHeader).tocs.offset(toc as isize)).subtype;
            if !(thisSize != size) { if count == 0 { break ; } count -= 1 }
        }
        toc = toc.wrapping_add(1)
    }
    if toc == (*fileHeader).ntoc { return -1i32 }
    return toc as libc::c_int;
}
unsafe extern "C" fn _XcursorReadImage(mut file: *mut XcursorFile,
                                       mut fileHeader: *mut XcursorFileHeader,
                                       mut toc: libc::c_int)
 -> *mut XcursorImage {
    let mut chunkHeader: XcursorChunkHeader =
        XcursorChunkHeader{header: 0, type_0: 0, subtype: 0, version: 0,};
    let mut head: XcursorImage =
        XcursorImage{version: 0,
                     size: 0,
                     width: 0,
                     height: 0,
                     xhot: 0,
                     yhot: 0,
                     delay: 0,
                     pixels: 0 as *mut XcursorPixel,};
    let mut image: *mut XcursorImage = 0 as *mut XcursorImage;
    let mut n: libc::c_int = 0;
    let mut p: *mut XcursorPixel = 0 as *mut XcursorPixel;
    if file.is_null() || fileHeader.is_null() {
        return 0 as *mut XcursorImage
    }
    if _XcursorFileReadChunkHeader(file, fileHeader, toc, &mut chunkHeader) ==
           0 {
        return 0 as *mut XcursorImage
    }
    if _XcursorReadUInt(file, &mut head.width) == 0 {
        return 0 as *mut XcursorImage
    }
    if _XcursorReadUInt(file, &mut head.height) == 0 {
        return 0 as *mut XcursorImage
    }
    if _XcursorReadUInt(file, &mut head.xhot) == 0 {
        return 0 as *mut XcursorImage
    }
    if _XcursorReadUInt(file, &mut head.yhot) == 0 {
        return 0 as *mut XcursorImage
    }
    if _XcursorReadUInt(file, &mut head.delay) == 0 {
        return 0 as *mut XcursorImage
    }
    /* sanity check data */
    if head.width > 0x7fffi32 as libc::c_uint ||
           head.height > 0x7fffi32 as libc::c_uint {
        return 0 as *mut XcursorImage
    }
    if head.width == 0i32 as libc::c_uint ||
           head.height == 0i32 as libc::c_uint {
        return 0 as *mut XcursorImage
    }
    if head.xhot > head.width || head.yhot > head.height {
        return 0 as *mut XcursorImage
    }
    /* Create the image and initialize it */
    image =
        XcursorImageCreate(head.width as libc::c_int,
                           head.height as libc::c_int);
    if image.is_null() { return 0 as *mut XcursorImage }
    if chunkHeader.version < (*image).version {
        (*image).version = chunkHeader.version
    }
    (*image).size = chunkHeader.subtype;
    (*image).xhot = head.xhot;
    (*image).yhot = head.yhot;
    (*image).delay = head.delay;
    n = (*image).width.wrapping_mul((*image).height) as libc::c_int;
    p = (*image).pixels;
    loop  {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0) { break ; }
        if _XcursorReadUInt(file, p) == 0 {
            XcursorImageDestroy(image);
            return 0 as *mut XcursorImage
        }
        p = p.offset(1)
    }
    return image;
}
unsafe extern "C" fn XcursorXcFileLoadImages(mut file: *mut XcursorFile,
                                             mut size: libc::c_int)
 -> *mut XcursorImages {
    let mut fileHeader: *mut XcursorFileHeader = 0 as *mut XcursorFileHeader;
    let mut bestSize: XcursorDim = 0;
    let mut nsize: libc::c_int = 0;
    let mut images: *mut XcursorImages = 0 as *mut XcursorImages;
    let mut n: libc::c_int = 0;
    let mut toc: libc::c_int = 0;
    if file.is_null() || size < 0i32 { return 0 as *mut XcursorImages }
    fileHeader = _XcursorReadFileHeader(file);
    if fileHeader.is_null() { return 0 as *mut XcursorImages }
    bestSize =
        _XcursorFindBestSize(fileHeader, size as XcursorDim, &mut nsize);
    if bestSize == 0 {
        _XcursorFileHeaderDestroy(fileHeader);
        return 0 as *mut XcursorImages
    }
    images = XcursorImagesCreate(nsize);
    if images.is_null() {
        _XcursorFileHeaderDestroy(fileHeader);
        return 0 as *mut XcursorImages
    }
    n = 0i32;
    while n < nsize {
        toc = _XcursorFindImageToc(fileHeader, bestSize, n);
        if toc < 0i32 { break ; }
        let ref mut fresh1 =
            *(*images).images.offset((*images).nimage as isize);
        *fresh1 = _XcursorReadImage(file, fileHeader, toc);
        if (*(*images).images.offset((*images).nimage as isize)).is_null() {
            break ;
        }
        (*images).nimage += 1;
        n += 1
    }
    _XcursorFileHeaderDestroy(fileHeader);
    if (*images).nimage != nsize {
        XcursorImagesDestroy(images);
        images = 0 as *mut XcursorImages
    }
    return images;
}
unsafe extern "C" fn _XcursorStdioFileRead(mut file: *mut XcursorFile,
                                           mut buf: *mut libc::c_uchar,
                                           mut len: libc::c_int)
 -> libc::c_int {
    let mut f: *mut FILE = (*file).closure as *mut FILE;
    return fread(buf as *mut libc::c_void, 1i32 as libc::c_ulong,
                 len as libc::c_ulong, f) as libc::c_int;
}
unsafe extern "C" fn _XcursorStdioFileWrite(mut file: *mut XcursorFile,
                                            mut buf: *mut libc::c_uchar,
                                            mut len: libc::c_int)
 -> libc::c_int {
    let mut f: *mut FILE = (*file).closure as *mut FILE;
    return fwrite(buf as *const libc::c_void, 1i32 as libc::c_ulong,
                  len as libc::c_ulong, f) as libc::c_int;
}
unsafe extern "C" fn _XcursorStdioFileSeek(mut file: *mut XcursorFile,
                                           mut offset: libc::c_long,
                                           mut whence: libc::c_int)
 -> libc::c_int {
    let mut f: *mut FILE = (*file).closure as *mut FILE;
    return fseek(f, offset, whence);
}
unsafe extern "C" fn _XcursorStdioFileInitialize(mut stdfile: *mut FILE,
                                                 mut file: *mut XcursorFile) {
    (*file).closure = stdfile as *mut libc::c_void;
    (*file).read =
        Some(_XcursorStdioFileRead as
                 unsafe extern "C" fn(_: *mut XcursorFile,
                                      _: *mut libc::c_uchar, _: libc::c_int)
                     -> libc::c_int);
    (*file).write =
        Some(_XcursorStdioFileWrite as
                 unsafe extern "C" fn(_: *mut XcursorFile,
                                      _: *mut libc::c_uchar, _: libc::c_int)
                     -> libc::c_int);
    (*file).seek =
        Some(_XcursorStdioFileSeek as
                 unsafe extern "C" fn(_: *mut XcursorFile, _: libc::c_long,
                                      _: libc::c_int) -> libc::c_int);
}
unsafe extern "C" fn XcursorFileLoadImages(mut file: *mut FILE,
                                           mut size: libc::c_int)
 -> *mut XcursorImages {
    let mut f: XcursorFile =
        XcursorFile{closure: 0 as *mut libc::c_void,
                    read: None,
                    write: None,
                    seek: None,};
    if file.is_null() { return 0 as *mut XcursorImages }
    _XcursorStdioFileInitialize(file, &mut f);
    return XcursorXcFileLoadImages(&mut f, size);
}
/*
 * From libXcursor/src/library.c
 */
unsafe extern "C" fn XcursorLibraryPath() -> *const libc::c_char {
    static mut path: *const libc::c_char = 0 as *const libc::c_char;
    if path.is_null() {
        path =
            getenv(b"XCURSOR_PATH\x00" as *const u8 as *const libc::c_char);
        if path.is_null() {
            path =
                b"~/.local/share/icons:~/.icons:/usr/share/icons:/usr/share/pixmaps:/usr/X11R6/lib/X11/icons\x00"
                    as *const u8 as *const libc::c_char
        }
    }
    return path;
}
unsafe extern "C" fn _XcursorAddPathElt(mut path: *mut libc::c_char,
                                        mut elt: *const libc::c_char,
                                        mut len: libc::c_int) {
    let mut pathlen: libc::c_int = strlen(path) as libc::c_int;
    /* append / if the path doesn't currently have one */
    if *path.offset(0) as libc::c_int == '\u{0}' as i32 ||
           *path.offset((pathlen - 1i32) as isize) as libc::c_int !=
               '/' as i32 {
        strcat(path, b"/\x00" as *const u8 as *const libc::c_char);
        pathlen += 1
    }
    if len == -1i32 { len = strlen(elt) as libc::c_int }
    /* strip leading slashes */
    while len != 0 && *elt.offset(0) as libc::c_int == '/' as i32 {
        elt = elt.offset(1);
        len -= 1
    }
    strncpy(path.offset(pathlen as isize), elt, len as libc::c_ulong);
    *path.offset((pathlen + len) as isize) = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn _XcursorBuildThemeDir(mut dir: *const libc::c_char,
                                           mut theme: *const libc::c_char)
 -> *mut libc::c_char {
    let mut colon: *const libc::c_char = 0 as *const libc::c_char;
    let mut tcolon: *const libc::c_char = 0 as *const libc::c_char;
    let mut full: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut home: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirlen: libc::c_int = 0;
    let mut homelen: libc::c_int = 0;
    let mut themelen: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if dir.is_null() || theme.is_null() { return 0 as *mut libc::c_char }
    colon = strchr(dir, ':' as i32);
    if colon.is_null() { colon = dir.offset(strlen(dir) as isize) }
    dirlen = colon.wrapping_offset_from(dir) as libc::c_long as libc::c_int;
    tcolon = strchr(theme, ':' as i32);
    if tcolon.is_null() { tcolon = theme.offset(strlen(theme) as isize) }
    themelen =
        tcolon.wrapping_offset_from(theme) as libc::c_long as libc::c_int;
    home = 0 as *mut libc::c_char;
    homelen = 0i32;
    if *dir as libc::c_int == '~' as i32 {
        home = getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
        if home.is_null() { return 0 as *mut libc::c_char }
        homelen = strlen(home) as libc::c_int;
        dir = dir.offset(1);
        dirlen -= 1
    }
    /*
     * add space for any needed directory separators, one per component,
     * and one for the trailing null
     */
    len = 1i32 + homelen + 1i32 + dirlen + 1i32 + themelen + 1i32;
    full = malloc(len as libc::c_ulong) as *mut libc::c_char;
    if full.is_null() { return 0 as *mut libc::c_char }
    *full.offset(0) = '\u{0}' as i32 as libc::c_char;
    if !home.is_null() { _XcursorAddPathElt(full, home, -1i32); }
    _XcursorAddPathElt(full, dir, dirlen);
    _XcursorAddPathElt(full, theme, themelen);
    return full;
}
unsafe extern "C" fn _XcursorBuildFullname(mut dir: *const libc::c_char,
                                           mut subdir: *const libc::c_char,
                                           mut file: *const libc::c_char)
 -> *mut libc::c_char {
    let mut full: *mut libc::c_char = 0 as *mut libc::c_char;
    if dir.is_null() || subdir.is_null() || file.is_null() {
        return 0 as *mut libc::c_char
    }
    full =
        malloc(strlen(dir).wrapping_add(1i32 as
                                            libc::c_ulong).wrapping_add(strlen(subdir)).wrapping_add(1i32
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_add(strlen(file)).wrapping_add(1i32
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_ulong))
            as *mut libc::c_char;
    if full.is_null() { return 0 as *mut libc::c_char }
    *full.offset(0) = '\u{0}' as i32 as libc::c_char;
    _XcursorAddPathElt(full, dir, -1i32);
    _XcursorAddPathElt(full, subdir, -1i32);
    _XcursorAddPathElt(full, file, -1i32);
    return full;
}
unsafe extern "C" fn _XcursorNextPath(mut path: *const libc::c_char)
 -> *const libc::c_char {
    let mut colon: *mut libc::c_char = strchr(path, ':' as i32);
    if colon.is_null() { return 0 as *const libc::c_char }
    return colon.offset(1);
}
unsafe extern "C" fn _XcursorThemeInherits(mut full: *const libc::c_char)
 -> *mut libc::c_char {
    let mut line: [libc::c_char; 8192] = [0; 8192];
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    if full.is_null() { return 0 as *mut libc::c_char }
    f = fopen(full, b"r\x00" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        while !fgets(line.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 8192]>() as
                         libc::c_ulong as libc::c_int, f).is_null() {
            if !(strncmp(line.as_mut_ptr(),
                         b"Inherits\x00" as *const u8 as *const libc::c_char,
                         8i32 as libc::c_ulong) == 0) {
                continue ;
            }
            let mut l: *mut libc::c_char = line.as_mut_ptr().offset(8);
            let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
            while *l as libc::c_int == ' ' as i32 { l = l.offset(1) }
            if *l as libc::c_int != '=' as i32 { continue ; }
            l = l.offset(1);
            while *l as libc::c_int == ' ' as i32 { l = l.offset(1) }
            result =
                malloc(strlen(l).wrapping_add(1i32 as libc::c_ulong)) as
                    *mut libc::c_char;
            if !result.is_null() {
                r = result;
                while *l != 0 {
                    while *l as libc::c_int == ';' as i32 ||
                              *l as libc::c_int == ',' as i32 ||
                              (*l as libc::c_int == ' ' as i32 ||
                                   *l as libc::c_int == '\t' as i32 ||
                                   *l as libc::c_int == '\n' as i32) {
                        l = l.offset(1)
                    }
                    if *l == 0 { break ; }
                    if r != result {
                        let fresh2 = r;
                        r = r.offset(1);
                        *fresh2 = ':' as i32 as libc::c_char
                    }
                    while *l as libc::c_int != 0 &&
                              !(*l as libc::c_int == ' ' as i32 ||
                                    *l as libc::c_int == '\t' as i32 ||
                                    *l as libc::c_int == '\n' as i32) &&
                              !(*l as libc::c_int == ';' as i32 ||
                                    *l as libc::c_int == ',' as i32) {
                        let fresh3 = l;
                        l = l.offset(1);
                        let fresh4 = r;
                        r = r.offset(1);
                        *fresh4 = *fresh3
                    }
                }
                let fresh5 = r;
                r = r.offset(1);
                *fresh5 = '\u{0}' as i32 as libc::c_char
            }
            break ;
        }
        fclose(f);
    }
    return result;
}
unsafe extern "C" fn XcursorScanTheme(mut theme: *const libc::c_char,
                                      mut name: *const libc::c_char)
 -> *mut FILE {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut full: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut inherits: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: *const libc::c_char = 0 as *const libc::c_char;
    if theme.is_null() || name.is_null() { return 0 as *mut FILE }
    /*
     * Scan this theme
     */
    path = XcursorLibraryPath();
    while !path.is_null() && f.is_null() {
        dir = _XcursorBuildThemeDir(path, theme);
        if !dir.is_null() {
            full =
                _XcursorBuildFullname(dir,
                                      b"cursors\x00" as *const u8 as
                                          *const libc::c_char, name);
            if !full.is_null() {
                f = fopen(full, b"r\x00" as *const u8 as *const libc::c_char);
                free(full as *mut libc::c_void);
            }
            if f.is_null() && inherits.is_null() {
                full =
                    _XcursorBuildFullname(dir,
                                          b"\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"index.theme\x00" as *const u8 as
                                              *const libc::c_char);
                if !full.is_null() {
                    inherits = _XcursorThemeInherits(full);
                    free(full as *mut libc::c_void);
                }
            }
            free(dir as *mut libc::c_void);
        }
        path = _XcursorNextPath(path)
    }
    /*
     * Recurse to scan inherited themes
     */
    i = inherits;
    while !i.is_null() && f.is_null() {
        if strcmp(i, theme) != 0i32 {
            f = XcursorScanTheme(i, name)
        } else {
            printf(b"Not calling XcursorScanTheme because of circular dependency: %s. %s\x00"
                       as *const u8 as *const libc::c_char, i, name);
        }
        i = _XcursorNextPath(i)
    }
    if !inherits.is_null() { free(inherits as *mut libc::c_void); }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn XcursorLibraryLoadImages(mut file:
                                                      *const libc::c_char,
                                                  mut theme:
                                                      *const libc::c_char,
                                                  mut size: libc::c_int)
 -> *mut XcursorImages {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut images: *mut XcursorImages = 0 as *mut XcursorImages;
    if file.is_null() { return 0 as *mut XcursorImages }
    if !theme.is_null() { f = XcursorScanTheme(theme, file) }
    if f.is_null() {
        f =
            XcursorScanTheme(b"default\x00" as *const u8 as
                                 *const libc::c_char, file)
    }
    if !f.is_null() {
        images = XcursorFileLoadImages(f, size);
        if !images.is_null() { XcursorImagesSetName(images, file); }
        fclose(f);
    }
    return images;
}
unsafe extern "C" fn load_all_cursors_from_dir(mut path: *const libc::c_char,
                                               mut size: libc::c_int,
                                               mut load_callback:
                                                   Option<unsafe extern "C" fn(_:
                                                                                   *mut XcursorImages,
                                                                               _:
                                                                                   *mut libc::c_void)
                                                              -> ()>,
                                               mut user_data:
                                                   *mut libc::c_void) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut dir: *mut DIR = opendir(path);
    let mut ent: *mut dirent = 0 as *mut dirent;
    let mut full: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut images: *mut XcursorImages = 0 as *mut XcursorImages;
    if dir.is_null() { return }
    ent = readdir(dir);
    while !ent.is_null() {
        if !((*ent).d_type as libc::c_int != DT_UNKNOWN as libc::c_int &&
                 ((*ent).d_type as libc::c_int != DT_REG as libc::c_int &&
                      (*ent).d_type as libc::c_int != DT_LNK as libc::c_int))
           {
            full =
                _XcursorBuildFullname(path,
                                      b"\x00" as *const u8 as
                                          *const libc::c_char,
                                      (*ent).d_name.as_mut_ptr());
            if !full.is_null() {
                f = fopen(full, b"r\x00" as *const u8 as *const libc::c_char);
                if f.is_null() {
                    free(full as *mut libc::c_void);
                } else {
                    images = XcursorFileLoadImages(f, size);
                    if !images.is_null() {
                        XcursorImagesSetName(images,
                                             (*ent).d_name.as_mut_ptr());
                        load_callback.expect("non-null function pointer")(images,
                                                                          user_data);
                    }
                    fclose(f);
                    free(full as *mut libc::c_void);
                }
            }
        }
        ent = readdir(dir)
    }
    closedir(dir);
}
/* * Load all the cursor of a theme
 *
 * This function loads all the cursor images of a given theme and its
 * inherited themes. Each cursor is loaded into an XcursorImages object
 * which is passed to the caller's load callback. If a cursor appears
 * more than once across all the inherited themes, the load callback
 * will be called multiple times, with possibly different XcursorImages
 * object which have the same name. The user is expected to destroy the
 * XcursorImages objects passed to the callback with
 * XcursorImagesDestroy().
 *
 * \param theme The name of theme that should be loaded
 * \param size The desired size of the cursor images
 * \param load_callback A callback function that will be called
 * for each cursor loaded. The first parameter is the XcursorImages
 * object representing the loaded cursor and the second is a pointer
 * to data provided by the user.
 * \param user_data The data that should be passed to the load callback
 */
#[no_mangle]
pub unsafe extern "C" fn xcursor_load_theme(mut theme: *const libc::c_char,
                                            mut size: libc::c_int,
                                            mut load_callback:
                                                Option<unsafe extern "C" fn(_:
                                                                                *mut XcursorImages,
                                                                            _:
                                                                                *mut libc::c_void)
                                                           -> ()>,
                                            mut user_data:
                                                *mut libc::c_void) {
    let mut full: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inherits: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: *const libc::c_char = 0 as *const libc::c_char;
    if theme.is_null() {
        theme = b"default\x00" as *const u8 as *const libc::c_char
    }
    path = XcursorLibraryPath();
    while !path.is_null() {
        dir = _XcursorBuildThemeDir(path, theme);
        if !dir.is_null() {
            full =
                _XcursorBuildFullname(dir,
                                      b"cursors\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"\x00" as *const u8 as
                                          *const libc::c_char);
            if !full.is_null() {
                load_all_cursors_from_dir(full, size, load_callback,
                                          user_data);
                free(full as *mut libc::c_void);
            }
            if inherits.is_null() {
                full =
                    _XcursorBuildFullname(dir,
                                          b"\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"index.theme\x00" as *const u8 as
                                              *const libc::c_char);
                if !full.is_null() {
                    inherits = _XcursorThemeInherits(full);
                    free(full as *mut libc::c_void);
                }
            }
            free(dir as *mut libc::c_void);
        }
        path = _XcursorNextPath(path)
    }
    i = inherits;
    while !i.is_null() {
        xcursor_load_theme(i, size, load_callback, user_data);
        i = _XcursorNextPath(i)
    }
    if !inherits.is_null() { free(inherits as *mut libc::c_void); };
}
