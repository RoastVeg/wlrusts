use libc;
pub type GLchar = libc::c_char;
// Colored quads
#[no_mangle]
pub static mut quad_vertex_src: [GLchar; 240] =
    [117, 110, 105, 102, 111, 114, 109, 32, 109, 97, 116, 51, 32, 112, 114,
     111, 106, 59, 10, 117, 110, 105, 102, 111, 114, 109, 32, 118, 101, 99,
     52, 32, 99, 111, 108, 111, 114, 59, 10, 97, 116, 116, 114, 105, 98, 117,
     116, 101, 32, 118, 101, 99, 50, 32, 112, 111, 115, 59, 10, 97, 116, 116,
     114, 105, 98, 117, 116, 101, 32, 118, 101, 99, 50, 32, 116, 101, 120, 99,
     111, 111, 114, 100, 59, 10, 118, 97, 114, 121, 105, 110, 103, 32, 118,
     101, 99, 52, 32, 118, 95, 99, 111, 108, 111, 114, 59, 10, 118, 97, 114,
     121, 105, 110, 103, 32, 118, 101, 99, 50, 32, 118, 95, 116, 101, 120, 99,
     111, 111, 114, 100, 59, 10, 10, 118, 111, 105, 100, 32, 109, 97, 105,
     110, 40, 41, 32, 123, 10, 9, 103, 108, 95, 80, 111, 115, 105, 116, 105,
     111, 110, 32, 61, 32, 118, 101, 99, 52, 40, 112, 114, 111, 106, 32, 42,
     32, 118, 101, 99, 51, 40, 112, 111, 115, 44, 32, 49, 46, 48, 41, 44, 32,
     49, 46, 48, 41, 59, 10, 9, 118, 95, 99, 111, 108, 111, 114, 32, 61, 32,
     99, 111, 108, 111, 114, 59, 10, 9, 118, 95, 116, 101, 120, 99, 111, 111,
     114, 100, 32, 61, 32, 116, 101, 120, 99, 111, 111, 114, 100, 59, 10, 125,
     10, 0];
#[no_mangle]
pub static mut quad_fragment_src: [GLchar; 115] =
    [112, 114, 101, 99, 105, 115, 105, 111, 110, 32, 109, 101, 100, 105, 117,
     109, 112, 32, 102, 108, 111, 97, 116, 59, 10, 118, 97, 114, 121, 105,
     110, 103, 32, 118, 101, 99, 52, 32, 118, 95, 99, 111, 108, 111, 114, 59,
     10, 118, 97, 114, 121, 105, 110, 103, 32, 118, 101, 99, 50, 32, 118, 95,
     116, 101, 120, 99, 111, 111, 114, 100, 59, 10, 10, 118, 111, 105, 100,
     32, 109, 97, 105, 110, 40, 41, 32, 123, 10, 9, 103, 108, 95, 70, 114, 97,
     103, 67, 111, 108, 111, 114, 32, 61, 32, 118, 95, 99, 111, 108, 111, 114,
     59, 10, 125, 10, 0];
// Colored ellipses
#[no_mangle]
pub static mut ellipse_fragment_src: [GLchar; 193] =
    [112, 114, 101, 99, 105, 115, 105, 111, 110, 32, 109, 101, 100, 105, 117,
     109, 112, 32, 102, 108, 111, 97, 116, 59, 10, 118, 97, 114, 121, 105,
     110, 103, 32, 118, 101, 99, 52, 32, 118, 95, 99, 111, 108, 111, 114, 59,
     10, 118, 97, 114, 121, 105, 110, 103, 32, 118, 101, 99, 50, 32, 118, 95,
     116, 101, 120, 99, 111, 111, 114, 100, 59, 10, 10, 118, 111, 105, 100,
     32, 109, 97, 105, 110, 40, 41, 32, 123, 10, 9, 102, 108, 111, 97, 116,
     32, 108, 32, 61, 32, 108, 101, 110, 103, 116, 104, 40, 118, 95, 116, 101,
     120, 99, 111, 111, 114, 100, 32, 45, 32, 118, 101, 99, 50, 40, 48, 46,
     53, 44, 32, 48, 46, 53, 41, 41, 59, 10, 9, 105, 102, 32, 40, 108, 32, 62,
     32, 48, 46, 53, 41, 32, 123, 10, 9, 9, 100, 105, 115, 99, 97, 114, 100,
     59, 10, 9, 125, 10, 9, 103, 108, 95, 70, 114, 97, 103, 67, 111, 108, 111,
     114, 32, 61, 32, 118, 95, 99, 111, 108, 111, 114, 59, 10, 125, 10, 0];
// Textured quads
#[no_mangle]
pub static mut tex_vertex_src: [GLchar; 285] =
    [117, 110, 105, 102, 111, 114, 109, 32, 109, 97, 116, 51, 32, 112, 114,
     111, 106, 59, 10, 117, 110, 105, 102, 111, 114, 109, 32, 98, 111, 111,
     108, 32, 105, 110, 118, 101, 114, 116, 95, 121, 59, 10, 97, 116, 116,
     114, 105, 98, 117, 116, 101, 32, 118, 101, 99, 50, 32, 112, 111, 115, 59,
     10, 97, 116, 116, 114, 105, 98, 117, 116, 101, 32, 118, 101, 99, 50, 32,
     116, 101, 120, 99, 111, 111, 114, 100, 59, 10, 118, 97, 114, 121, 105,
     110, 103, 32, 118, 101, 99, 50, 32, 118, 95, 116, 101, 120, 99, 111, 111,
     114, 100, 59, 10, 10, 118, 111, 105, 100, 32, 109, 97, 105, 110, 40, 41,
     32, 123, 10, 9, 103, 108, 95, 80, 111, 115, 105, 116, 105, 111, 110, 32,
     61, 32, 118, 101, 99, 52, 40, 112, 114, 111, 106, 32, 42, 32, 118, 101,
     99, 51, 40, 112, 111, 115, 44, 32, 49, 46, 48, 41, 44, 32, 49, 46, 48,
     41, 59, 10, 9, 105, 102, 32, 40, 105, 110, 118, 101, 114, 116, 95, 121,
     41, 32, 123, 10, 9, 9, 118, 95, 116, 101, 120, 99, 111, 111, 114, 100,
     32, 61, 32, 118, 101, 99, 50, 40, 116, 101, 120, 99, 111, 111, 114, 100,
     46, 115, 44, 32, 49, 46, 48, 32, 45, 32, 116, 101, 120, 99, 111, 111,
     114, 100, 46, 116, 41, 59, 10, 9, 125, 32, 101, 108, 115, 101, 32, 123,
     10, 9, 9, 118, 95, 116, 101, 120, 99, 111, 111, 114, 100, 32, 61, 32,
     116, 101, 120, 99, 111, 111, 114, 100, 59, 10, 9, 125, 10, 125, 10, 0];
#[no_mangle]
pub static mut tex_fragment_src_rgba: [GLchar; 164] =
    [112, 114, 101, 99, 105, 115, 105, 111, 110, 32, 109, 101, 100, 105, 117,
     109, 112, 32, 102, 108, 111, 97, 116, 59, 10, 118, 97, 114, 121, 105,
     110, 103, 32, 118, 101, 99, 50, 32, 118, 95, 116, 101, 120, 99, 111, 111,
     114, 100, 59, 10, 117, 110, 105, 102, 111, 114, 109, 32, 115, 97, 109,
     112, 108, 101, 114, 50, 68, 32, 116, 101, 120, 59, 10, 117, 110, 105,
     102, 111, 114, 109, 32, 102, 108, 111, 97, 116, 32, 97, 108, 112, 104,
     97, 59, 10, 10, 118, 111, 105, 100, 32, 109, 97, 105, 110, 40, 41, 32,
     123, 10, 9, 103, 108, 95, 70, 114, 97, 103, 67, 111, 108, 111, 114, 32,
     61, 32, 116, 101, 120, 116, 117, 114, 101, 50, 68, 40, 116, 101, 120, 44,
     32, 118, 95, 116, 101, 120, 99, 111, 111, 114, 100, 41, 32, 42, 32, 97,
     108, 112, 104, 97, 59, 10, 125, 10, 0];
#[no_mangle]
pub static mut tex_fragment_src_rgbx: [GLchar; 179] =
    [112, 114, 101, 99, 105, 115, 105, 111, 110, 32, 109, 101, 100, 105, 117,
     109, 112, 32, 102, 108, 111, 97, 116, 59, 10, 118, 97, 114, 121, 105,
     110, 103, 32, 118, 101, 99, 50, 32, 118, 95, 116, 101, 120, 99, 111, 111,
     114, 100, 59, 10, 117, 110, 105, 102, 111, 114, 109, 32, 115, 97, 109,
     112, 108, 101, 114, 50, 68, 32, 116, 101, 120, 59, 10, 117, 110, 105,
     102, 111, 114, 109, 32, 102, 108, 111, 97, 116, 32, 97, 108, 112, 104,
     97, 59, 10, 10, 118, 111, 105, 100, 32, 109, 97, 105, 110, 40, 41, 32,
     123, 10, 9, 103, 108, 95, 70, 114, 97, 103, 67, 111, 108, 111, 114, 32,
     61, 32, 118, 101, 99, 52, 40, 116, 101, 120, 116, 117, 114, 101, 50, 68,
     40, 116, 101, 120, 44, 32, 118, 95, 116, 101, 120, 99, 111, 111, 114,
     100, 41, 46, 114, 103, 98, 44, 32, 49, 46, 48, 41, 32, 42, 32, 97, 108,
     112, 104, 97, 59, 10, 125, 10, 0];
#[no_mangle]
pub static mut tex_fragment_src_external: [GLchar; 231] =
    [35, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 71, 76, 95, 79, 69,
     83, 95, 69, 71, 76, 95, 105, 109, 97, 103, 101, 95, 101, 120, 116, 101,
     114, 110, 97, 108, 32, 58, 32, 114, 101, 113, 117, 105, 114, 101, 10, 10,
     112, 114, 101, 99, 105, 115, 105, 111, 110, 32, 109, 101, 100, 105, 117,
     109, 112, 32, 102, 108, 111, 97, 116, 59, 10, 118, 97, 114, 121, 105,
     110, 103, 32, 118, 101, 99, 50, 32, 118, 95, 116, 101, 120, 99, 111, 111,
     114, 100, 59, 10, 117, 110, 105, 102, 111, 114, 109, 32, 115, 97, 109,
     112, 108, 101, 114, 69, 120, 116, 101, 114, 110, 97, 108, 79, 69, 83, 32,
     116, 101, 120, 116, 117, 114, 101, 48, 59, 10, 117, 110, 105, 102, 111,
     114, 109, 32, 102, 108, 111, 97, 116, 32, 97, 108, 112, 104, 97, 59, 10,
     10, 118, 111, 105, 100, 32, 109, 97, 105, 110, 40, 41, 32, 123, 10, 9,
     103, 108, 95, 70, 114, 97, 103, 67, 111, 108, 111, 114, 32, 61, 32, 116,
     101, 120, 116, 117, 114, 101, 50, 68, 40, 116, 101, 120, 116, 117, 114,
     101, 48, 44, 32, 118, 95, 116, 101, 120, 99, 111, 111, 114, 100, 41, 32,
     42, 32, 97, 108, 112, 104, 97, 59, 10, 125, 10, 0];